

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

#ifndef __bits10_1_h__
#define __bits10_1_h__

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

#ifndef __IBackgroundCopyCallback3_FWD_DEFINED__
#define __IBackgroundCopyCallback3_FWD_DEFINED__
typedef interface IBackgroundCopyCallback3 IBackgroundCopyCallback3;

#endif 	/* __IBackgroundCopyCallback3_FWD_DEFINED__ */


#ifndef __IBackgroundCopyFile6_FWD_DEFINED__
#define __IBackgroundCopyFile6_FWD_DEFINED__
typedef interface IBackgroundCopyFile6 IBackgroundCopyFile6;

#endif 	/* __IBackgroundCopyFile6_FWD_DEFINED__ */


#ifndef __BackgroundCopyManager10_1_FWD_DEFINED__
#define __BackgroundCopyManager10_1_FWD_DEFINED__

#ifdef __cplusplus
typedef class BackgroundCopyManager10_1 BackgroundCopyManager10_1;
#else
typedef struct BackgroundCopyManager10_1 BackgroundCopyManager10_1;
#endif /* __cplusplus */

#endif 	/* __BackgroundCopyManager10_1_FWD_DEFINED__ */


#ifndef __IBackgroundCopyCallback3_FWD_DEFINED__
#define __IBackgroundCopyCallback3_FWD_DEFINED__
typedef interface IBackgroundCopyCallback3 IBackgroundCopyCallback3;

#endif 	/* __IBackgroundCopyCallback3_FWD_DEFINED__ */


#ifndef __IBackgroundCopyFile6_FWD_DEFINED__
#define __IBackgroundCopyFile6_FWD_DEFINED__
typedef interface IBackgroundCopyFile6 IBackgroundCopyFile6;

#endif 	/* __IBackgroundCopyFile6_FWD_DEFINED__ */


/* header files for imported files */
#include "bits.h"
#include "bits1_5.h"
#include "bits2_0.h"
#include "bits2_5.h"
#include "bits3_0.h"
#include "bits4_0.h"
#include "bits5_0.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_bits10_1_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_bits10_1_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_bits10_1_0000_0000_v0_0_s_ifspec;

#ifndef __IBackgroundCopyCallback3_INTERFACE_DEFINED__
#define __IBackgroundCopyCallback3_INTERFACE_DEFINED__

/* interface IBackgroundCopyCallback3 */
/* [object][uuid] */ 


EXTERN_C const IID IID_IBackgroundCopyCallback3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("98c97bd2-e32b-4ad8-a528-95fd8b16bd42")
    IBackgroundCopyCallback3 : public IBackgroundCopyCallback2
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE FileRangesTransferred( 
            /* [in] */ __RPC__in_opt IBackgroundCopyJob *job,
            /* [in] */ __RPC__in_opt IBackgroundCopyFile *file,
            /* [in] */ DWORD rangeCount,
            /* [size_is][in] */ __RPC__in_ecount_full(rangeCount) const BG_FILE_RANGE ranges[  ]) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBackgroundCopyCallback3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBackgroundCopyCallback3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBackgroundCopyCallback3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBackgroundCopyCallback3 * This);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyCallback, JobTransferred)
        HRESULT ( STDMETHODCALLTYPE *JobTransferred )( 
            __RPC__in IBackgroundCopyCallback3 * This,
            /* [in] */ __RPC__in_opt IBackgroundCopyJob *pJob);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyCallback, JobError)
        HRESULT ( STDMETHODCALLTYPE *JobError )( 
            __RPC__in IBackgroundCopyCallback3 * This,
            /* [in] */ __RPC__in_opt IBackgroundCopyJob *pJob,
            /* [in] */ __RPC__in_opt IBackgroundCopyError *pError);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyCallback, JobModification)
        HRESULT ( STDMETHODCALLTYPE *JobModification )( 
            __RPC__in IBackgroundCopyCallback3 * This,
            /* [in] */ __RPC__in_opt IBackgroundCopyJob *pJob,
            /* [in] */ DWORD dwReserved);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyCallback2, FileTransferred)
        HRESULT ( STDMETHODCALLTYPE *FileTransferred )( 
            __RPC__in IBackgroundCopyCallback3 * This,
            /* [in] */ __RPC__in_opt IBackgroundCopyJob *pJob,
            /* [in] */ __RPC__in_opt IBackgroundCopyFile *pFile);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyCallback3, FileRangesTransferred)
        HRESULT ( STDMETHODCALLTYPE *FileRangesTransferred )( 
            __RPC__in IBackgroundCopyCallback3 * This,
            /* [in] */ __RPC__in_opt IBackgroundCopyJob *job,
            /* [in] */ __RPC__in_opt IBackgroundCopyFile *file,
            /* [in] */ DWORD rangeCount,
            /* [size_is][in] */ __RPC__in_ecount_full(rangeCount) const BG_FILE_RANGE ranges[  ]);
        
        END_INTERFACE
    } IBackgroundCopyCallback3Vtbl;

    interface IBackgroundCopyCallback3
    {
        CONST_VTBL struct IBackgroundCopyCallback3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBackgroundCopyCallback3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBackgroundCopyCallback3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBackgroundCopyCallback3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBackgroundCopyCallback3_JobTransferred(This,pJob)	\
    ( (This)->lpVtbl -> JobTransferred(This,pJob) ) 

#define IBackgroundCopyCallback3_JobError(This,pJob,pError)	\
    ( (This)->lpVtbl -> JobError(This,pJob,pError) ) 

#define IBackgroundCopyCallback3_JobModification(This,pJob,dwReserved)	\
    ( (This)->lpVtbl -> JobModification(This,pJob,dwReserved) ) 


#define IBackgroundCopyCallback3_FileTransferred(This,pJob,pFile)	\
    ( (This)->lpVtbl -> FileTransferred(This,pJob,pFile) ) 


#define IBackgroundCopyCallback3_FileRangesTransferred(This,job,file,rangeCount,ranges)	\
    ( (This)->lpVtbl -> FileRangesTransferred(This,job,file,rangeCount,ranges) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBackgroundCopyCallback3_INTERFACE_DEFINED__ */


#ifndef __IBackgroundCopyFile6_INTERFACE_DEFINED__
#define __IBackgroundCopyFile6_INTERFACE_DEFINED__

/* interface IBackgroundCopyFile6 */
/* [object][uuid] */ 


EXTERN_C const IID IID_IBackgroundCopyFile6;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("CF6784F7-D677-49FD-9368-CB47AEE9D1AD")
    IBackgroundCopyFile6 : public IBackgroundCopyFile5
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE UpdateDownloadPosition( 
            /* [in] */ UINT64 offset) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RequestFileRanges( 
            /* [in] */ DWORD rangeCount,
            /* [size_is][in] */ __RPC__in_ecount_full(rangeCount) const BG_FILE_RANGE ranges[  ]) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFilledFileRanges( 
            /* [ref][out] */ __RPC__out DWORD *rangeCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*rangeCount) BG_FILE_RANGE **ranges) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBackgroundCopyFile6Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBackgroundCopyFile6 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBackgroundCopyFile6 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBackgroundCopyFile6 * This);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyFile, GetRemoteName)
        HRESULT ( STDMETHODCALLTYPE *GetRemoteName )( 
            __RPC__in IBackgroundCopyFile6 * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *pVal);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyFile, GetLocalName)
        HRESULT ( STDMETHODCALLTYPE *GetLocalName )( 
            __RPC__in IBackgroundCopyFile6 * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *pVal);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyFile, GetProgress)
        HRESULT ( STDMETHODCALLTYPE *GetProgress )( 
            __RPC__in IBackgroundCopyFile6 * This,
            /* [out] */ __RPC__out BG_FILE_PROGRESS *pVal);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyFile2, GetFileRanges)
        HRESULT ( STDMETHODCALLTYPE *GetFileRanges )( 
            __RPC__in IBackgroundCopyFile6 * This,
            /* [unique][out][in] */ __RPC__inout_opt DWORD *RangeCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*RangeCount) BG_FILE_RANGE **Ranges);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyFile2, SetRemoteName)
        HRESULT ( STDMETHODCALLTYPE *SetRemoteName )( 
            __RPC__in IBackgroundCopyFile6 * This,
            __RPC__in LPCWSTR Val);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyFile3, GetTemporaryName)
        HRESULT ( STDMETHODCALLTYPE *GetTemporaryName )( 
            __RPC__in IBackgroundCopyFile6 * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *pFilename);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyFile3, SetValidationState)
        HRESULT ( STDMETHODCALLTYPE *SetValidationState )( 
            __RPC__in IBackgroundCopyFile6 * This,
            /* [in] */ BOOL state);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyFile3, GetValidationState)
        HRESULT ( STDMETHODCALLTYPE *GetValidationState )( 
            __RPC__in IBackgroundCopyFile6 * This,
            /* [out] */ __RPC__out BOOL *pState);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyFile3, IsDownloadedFromPeer)
        HRESULT ( STDMETHODCALLTYPE *IsDownloadedFromPeer )( 
            __RPC__in IBackgroundCopyFile6 * This,
            /* [out] */ __RPC__out BOOL *pVal);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyFile4, GetPeerDownloadStats)
        HRESULT ( STDMETHODCALLTYPE *GetPeerDownloadStats )( 
            __RPC__in IBackgroundCopyFile6 * This,
            /* [out] */ __RPC__out PUINT64 pFromOrigin,
            /* [out] */ __RPC__out PUINT64 pFromPeers);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyFile5, SetProperty)
        HRESULT ( STDMETHODCALLTYPE *SetProperty )( 
            __RPC__in IBackgroundCopyFile6 * This,
            /* [in] */ BITS_FILE_PROPERTY_ID PropertyId,
            /* [switch_is][in] */ BITS_FILE_PROPERTY_VALUE PropertyValue);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyFile5, GetProperty)
        HRESULT ( STDMETHODCALLTYPE *GetProperty )( 
            __RPC__in IBackgroundCopyFile6 * This,
            /* [in] */ BITS_FILE_PROPERTY_ID PropertyId,
            /* [switch_is][out] */ __RPC__out BITS_FILE_PROPERTY_VALUE *PropertyValue);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyFile6, UpdateDownloadPosition)
        HRESULT ( STDMETHODCALLTYPE *UpdateDownloadPosition )( 
            __RPC__in IBackgroundCopyFile6 * This,
            /* [in] */ UINT64 offset);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyFile6, RequestFileRanges)
        HRESULT ( STDMETHODCALLTYPE *RequestFileRanges )( 
            __RPC__in IBackgroundCopyFile6 * This,
            /* [in] */ DWORD rangeCount,
            /* [size_is][in] */ __RPC__in_ecount_full(rangeCount) const BG_FILE_RANGE ranges[  ]);
        
        DECLSPEC_XFGVIRT(IBackgroundCopyFile6, GetFilledFileRanges)
        HRESULT ( STDMETHODCALLTYPE *GetFilledFileRanges )( 
            __RPC__in IBackgroundCopyFile6 * This,
            /* [ref][out] */ __RPC__out DWORD *rangeCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*rangeCount) BG_FILE_RANGE **ranges);
        
        END_INTERFACE
    } IBackgroundCopyFile6Vtbl;

    interface IBackgroundCopyFile6
    {
        CONST_VTBL struct IBackgroundCopyFile6Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBackgroundCopyFile6_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBackgroundCopyFile6_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBackgroundCopyFile6_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBackgroundCopyFile6_GetRemoteName(This,pVal)	\
    ( (This)->lpVtbl -> GetRemoteName(This,pVal) ) 

#define IBackgroundCopyFile6_GetLocalName(This,pVal)	\
    ( (This)->lpVtbl -> GetLocalName(This,pVal) ) 

#define IBackgroundCopyFile6_GetProgress(This,pVal)	\
    ( (This)->lpVtbl -> GetProgress(This,pVal) ) 


#define IBackgroundCopyFile6_GetFileRanges(This,RangeCount,Ranges)	\
    ( (This)->lpVtbl -> GetFileRanges(This,RangeCount,Ranges) ) 

#define IBackgroundCopyFile6_SetRemoteName(This,Val)	\
    ( (This)->lpVtbl -> SetRemoteName(This,Val) ) 


#define IBackgroundCopyFile6_GetTemporaryName(This,pFilename)	\
    ( (This)->lpVtbl -> GetTemporaryName(This,pFilename) ) 

#define IBackgroundCopyFile6_SetValidationState(This,state)	\
    ( (This)->lpVtbl -> SetValidationState(This,state) ) 

#define IBackgroundCopyFile6_GetValidationState(This,pState)	\
    ( (This)->lpVtbl -> GetValidationState(This,pState) ) 

#define IBackgroundCopyFile6_IsDownloadedFromPeer(This,pVal)	\
    ( (This)->lpVtbl -> IsDownloadedFromPeer(This,pVal) ) 


#define IBackgroundCopyFile6_GetPeerDownloadStats(This,pFromOrigin,pFromPeers)	\
    ( (This)->lpVtbl -> GetPeerDownloadStats(This,pFromOrigin,pFromPeers) ) 


#define IBackgroundCopyFile6_SetProperty(This,PropertyId,PropertyValue)	\
    ( (This)->lpVtbl -> SetProperty(This,PropertyId,PropertyValue) ) 

#define IBackgroundCopyFile6_GetProperty(This,PropertyId,PropertyValue)	\
    ( (This)->lpVtbl -> GetProperty(This,PropertyId,PropertyValue) ) 


#define IBackgroundCopyFile6_UpdateDownloadPosition(This,offset)	\
    ( (This)->lpVtbl -> UpdateDownloadPosition(This,offset) ) 

#define IBackgroundCopyFile6_RequestFileRanges(This,rangeCount,ranges)	\
    ( (This)->lpVtbl -> RequestFileRanges(This,rangeCount,ranges) ) 

#define IBackgroundCopyFile6_GetFilledFileRanges(This,rangeCount,ranges)	\
    ( (This)->lpVtbl -> GetFilledFileRanges(This,rangeCount,ranges) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBackgroundCopyFile6_INTERFACE_DEFINED__ */



#ifndef __BackgroundCopyManager10_1_LIBRARY_DEFINED__
#define __BackgroundCopyManager10_1_LIBRARY_DEFINED__

/* library BackgroundCopyManager10_1 */
/* [version][lcid][uuid] */ 












EXTERN_C const IID LIBID_BackgroundCopyManager10_1;

EXTERN_C const CLSID CLSID_BackgroundCopyManager10_1;

#ifdef __cplusplus

class DECLSPEC_UUID("4BD3E4E1-7BD4-4A2B-9964-496400DE5193")
BackgroundCopyManager10_1;
#endif
#endif /* __BackgroundCopyManager10_1_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_bits10_1_0000_0003 */
/* [local] */ 

#include "bits10_2.h"
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_bits10_1_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_bits10_1_0000_0003_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif



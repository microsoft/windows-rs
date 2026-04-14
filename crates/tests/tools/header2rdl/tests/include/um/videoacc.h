

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

#ifndef __videoacc_h__
#define __videoacc_h__

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

#ifndef __IAMVideoAcceleratorNotify_FWD_DEFINED__
#define __IAMVideoAcceleratorNotify_FWD_DEFINED__
typedef interface IAMVideoAcceleratorNotify IAMVideoAcceleratorNotify;

#endif 	/* __IAMVideoAcceleratorNotify_FWD_DEFINED__ */


#ifndef __IAMVideoAccelerator_FWD_DEFINED__
#define __IAMVideoAccelerator_FWD_DEFINED__
typedef interface IAMVideoAccelerator IAMVideoAccelerator;

#endif 	/* __IAMVideoAccelerator_FWD_DEFINED__ */


/* header files for imported files */
#include "unknwn.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_videoacc_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
//
//   The following declarations within the 'if 0' block are dummy typedefs used to make
//   the motncomp.idl file build.  The actual definitions are contained in ddraw.h and amva.h
//
#if 0
typedef void *LPVOID;

typedef void *LPGUID;

typedef void *LPDIRECTDRAWSURFACE;

typedef void *LPDDPIXELFORMAT;

typedef void *LPAMVAInternalMemInfo;

typedef void AMVAUncompDataInfo;

typedef void *LPAMVACompBufferInfo;

typedef void AMVABUFFERINFO;

typedef void AMVAEndFrameInfo;

typedef void *LPAMVAUncompBufferInfo;

typedef void AMVABeginFrameInfo;

typedef IUnknown *IMediaSample;

#endif
#include <ddraw.h>
#include <amva.h>


extern RPC_IF_HANDLE __MIDL_itf_videoacc_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_videoacc_0000_0000_v0_0_s_ifspec;

#ifndef __IAMVideoAcceleratorNotify_INTERFACE_DEFINED__
#define __IAMVideoAcceleratorNotify_INTERFACE_DEFINED__

/* interface IAMVideoAcceleratorNotify */
/* [unique][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_IAMVideoAcceleratorNotify;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("256A6A21-FBAD-11d1-82BF-00A0C9696C8F")
    IAMVideoAcceleratorNotify : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetUncompSurfacesInfo( 
            /* [in] */ const GUID *pGuid,
            /* [annotation][out][in] */ 
            _Inout_  LPAMVAUncompBufferInfo pUncompBufferInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetUncompSurfacesInfo( 
            /* [in] */ DWORD dwActualUncompSurfacesAllocated) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCreateVideoAcceleratorData( 
            /* [in] */ const GUID *pGuid,
            /* [annotation][out] */ 
            _Out_  LPDWORD pdwSizeMiscData,
            /* [annotation][out] */ 
            _Outptr_result_bytebuffer_(*pdwSizeMiscData)  LPVOID *ppMiscData) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAMVideoAcceleratorNotifyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAMVideoAcceleratorNotify * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAMVideoAcceleratorNotify * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAMVideoAcceleratorNotify * This);
        
        DECLSPEC_XFGVIRT(IAMVideoAcceleratorNotify, GetUncompSurfacesInfo)
        HRESULT ( STDMETHODCALLTYPE *GetUncompSurfacesInfo )( 
            IAMVideoAcceleratorNotify * This,
            /* [in] */ const GUID *pGuid,
            /* [annotation][out][in] */ 
            _Inout_  LPAMVAUncompBufferInfo pUncompBufferInfo);
        
        DECLSPEC_XFGVIRT(IAMVideoAcceleratorNotify, SetUncompSurfacesInfo)
        HRESULT ( STDMETHODCALLTYPE *SetUncompSurfacesInfo )( 
            IAMVideoAcceleratorNotify * This,
            /* [in] */ DWORD dwActualUncompSurfacesAllocated);
        
        DECLSPEC_XFGVIRT(IAMVideoAcceleratorNotify, GetCreateVideoAcceleratorData)
        HRESULT ( STDMETHODCALLTYPE *GetCreateVideoAcceleratorData )( 
            IAMVideoAcceleratorNotify * This,
            /* [in] */ const GUID *pGuid,
            /* [annotation][out] */ 
            _Out_  LPDWORD pdwSizeMiscData,
            /* [annotation][out] */ 
            _Outptr_result_bytebuffer_(*pdwSizeMiscData)  LPVOID *ppMiscData);
        
        END_INTERFACE
    } IAMVideoAcceleratorNotifyVtbl;

    interface IAMVideoAcceleratorNotify
    {
        CONST_VTBL struct IAMVideoAcceleratorNotifyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAMVideoAcceleratorNotify_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAMVideoAcceleratorNotify_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAMVideoAcceleratorNotify_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAMVideoAcceleratorNotify_GetUncompSurfacesInfo(This,pGuid,pUncompBufferInfo)	\
    ( (This)->lpVtbl -> GetUncompSurfacesInfo(This,pGuid,pUncompBufferInfo) ) 

#define IAMVideoAcceleratorNotify_SetUncompSurfacesInfo(This,dwActualUncompSurfacesAllocated)	\
    ( (This)->lpVtbl -> SetUncompSurfacesInfo(This,dwActualUncompSurfacesAllocated) ) 

#define IAMVideoAcceleratorNotify_GetCreateVideoAcceleratorData(This,pGuid,pdwSizeMiscData,ppMiscData)	\
    ( (This)->lpVtbl -> GetCreateVideoAcceleratorData(This,pGuid,pdwSizeMiscData,ppMiscData) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAMVideoAcceleratorNotify_INTERFACE_DEFINED__ */


#ifndef __IAMVideoAccelerator_INTERFACE_DEFINED__
#define __IAMVideoAccelerator_INTERFACE_DEFINED__

/* interface IAMVideoAccelerator */
/* [unique][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_IAMVideoAccelerator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("256A6A22-FBAD-11d1-82BF-00A0C9696C8F")
    IAMVideoAccelerator : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetVideoAcceleratorGUIDs( 
            /* [annotation][out][in] */ 
            _Inout_  LPDWORD pdwNumGuidsSupported,
            /* [annotation][out][in] */ 
            _Out_writes_to_opt_(*pdwNumGuidsSupported, *pdwNumGuidsSupported)  LPGUID pGuidsSupported) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetUncompFormatsSupported( 
            /* [in] */ const GUID *pGuid,
            /* [annotation][out][in] */ 
            _Inout_  LPDWORD pdwNumFormatsSupported,
            /* [annotation][out][in] */ 
            _Out_writes_to_opt_(*pdwNumFormatsSupported, *pdwNumFormatsSupported)  LPDDPIXELFORMAT pFormatsSupported) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetInternalMemInfo( 
            /* [in] */ const GUID *pGuid,
            /* [in] */ const AMVAUncompDataInfo *pamvaUncompDataInfo,
            /* [annotation][out][in] */ 
            _Inout_  LPAMVAInternalMemInfo pamvaInternalMemInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCompBufferInfo( 
            /* [in] */ const GUID *pGuid,
            /* [in] */ const AMVAUncompDataInfo *pamvaUncompDataInfo,
            /* [annotation][out][in] */ 
            _Inout_  LPDWORD pdwNumTypesCompBuffers,
            /* [annotation][out] */ 
            _Out_writes_to_opt_(*pdwNumTypesCompBuffers, *pdwNumTypesCompBuffers)  LPAMVACompBufferInfo pamvaCompBufferInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetInternalCompBufferInfo( 
            /* [annotation][out][in] */ 
            _Inout_  LPDWORD pdwNumTypesCompBuffers,
            /* [annotation][out] */ 
            _Out_writes_to_opt_(*pdwNumTypesCompBuffers, *pdwNumTypesCompBuffers)  LPAMVACompBufferInfo pamvaCompBufferInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BeginFrame( 
            /* [in] */ const AMVABeginFrameInfo *amvaBeginFrameInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EndFrame( 
            /* [in] */ const AMVAEndFrameInfo *pEndFrameInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetBuffer( 
            /* [in] */ DWORD dwTypeIndex,
            /* [in] */ DWORD dwBufferIndex,
            /* [in] */ BOOL bReadOnly,
            /* [annotation][out] */ 
            _Out_  LPVOID *ppBuffer,
            /* [annotation][out] */ 
            _Out_  LONG *lpStride) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ReleaseBuffer( 
            /* [in] */ DWORD dwTypeIndex,
            /* [in] */ DWORD dwBufferIndex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Execute( 
            /* [in] */ DWORD dwFunction,
            /* [in] */ LPVOID lpPrivateInputData,
            /* [in] */ DWORD cbPrivateInputData,
            /* [in] */ LPVOID lpPrivateOutputDat,
            /* [in] */ DWORD cbPrivateOutputData,
            /* [in] */ DWORD dwNumBuffers,
            /* [annotation][in] */ 
            _In_reads_(dwNumBuffers)  const AMVABUFFERINFO *pamvaBufferInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE QueryRenderStatus( 
            /* [in] */ DWORD dwTypeIndex,
            /* [in] */ DWORD dwBufferIndex,
            /* [in] */ DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DisplayFrame( 
            /* [in] */ DWORD dwFlipToIndex,
            /* [in] */ IMediaSample *pMediaSample) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAMVideoAcceleratorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAMVideoAccelerator * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAMVideoAccelerator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAMVideoAccelerator * This);
        
        DECLSPEC_XFGVIRT(IAMVideoAccelerator, GetVideoAcceleratorGUIDs)
        HRESULT ( STDMETHODCALLTYPE *GetVideoAcceleratorGUIDs )( 
            IAMVideoAccelerator * This,
            /* [annotation][out][in] */ 
            _Inout_  LPDWORD pdwNumGuidsSupported,
            /* [annotation][out][in] */ 
            _Out_writes_to_opt_(*pdwNumGuidsSupported, *pdwNumGuidsSupported)  LPGUID pGuidsSupported);
        
        DECLSPEC_XFGVIRT(IAMVideoAccelerator, GetUncompFormatsSupported)
        HRESULT ( STDMETHODCALLTYPE *GetUncompFormatsSupported )( 
            IAMVideoAccelerator * This,
            /* [in] */ const GUID *pGuid,
            /* [annotation][out][in] */ 
            _Inout_  LPDWORD pdwNumFormatsSupported,
            /* [annotation][out][in] */ 
            _Out_writes_to_opt_(*pdwNumFormatsSupported, *pdwNumFormatsSupported)  LPDDPIXELFORMAT pFormatsSupported);
        
        DECLSPEC_XFGVIRT(IAMVideoAccelerator, GetInternalMemInfo)
        HRESULT ( STDMETHODCALLTYPE *GetInternalMemInfo )( 
            IAMVideoAccelerator * This,
            /* [in] */ const GUID *pGuid,
            /* [in] */ const AMVAUncompDataInfo *pamvaUncompDataInfo,
            /* [annotation][out][in] */ 
            _Inout_  LPAMVAInternalMemInfo pamvaInternalMemInfo);
        
        DECLSPEC_XFGVIRT(IAMVideoAccelerator, GetCompBufferInfo)
        HRESULT ( STDMETHODCALLTYPE *GetCompBufferInfo )( 
            IAMVideoAccelerator * This,
            /* [in] */ const GUID *pGuid,
            /* [in] */ const AMVAUncompDataInfo *pamvaUncompDataInfo,
            /* [annotation][out][in] */ 
            _Inout_  LPDWORD pdwNumTypesCompBuffers,
            /* [annotation][out] */ 
            _Out_writes_to_opt_(*pdwNumTypesCompBuffers, *pdwNumTypesCompBuffers)  LPAMVACompBufferInfo pamvaCompBufferInfo);
        
        DECLSPEC_XFGVIRT(IAMVideoAccelerator, GetInternalCompBufferInfo)
        HRESULT ( STDMETHODCALLTYPE *GetInternalCompBufferInfo )( 
            IAMVideoAccelerator * This,
            /* [annotation][out][in] */ 
            _Inout_  LPDWORD pdwNumTypesCompBuffers,
            /* [annotation][out] */ 
            _Out_writes_to_opt_(*pdwNumTypesCompBuffers, *pdwNumTypesCompBuffers)  LPAMVACompBufferInfo pamvaCompBufferInfo);
        
        DECLSPEC_XFGVIRT(IAMVideoAccelerator, BeginFrame)
        HRESULT ( STDMETHODCALLTYPE *BeginFrame )( 
            IAMVideoAccelerator * This,
            /* [in] */ const AMVABeginFrameInfo *amvaBeginFrameInfo);
        
        DECLSPEC_XFGVIRT(IAMVideoAccelerator, EndFrame)
        HRESULT ( STDMETHODCALLTYPE *EndFrame )( 
            IAMVideoAccelerator * This,
            /* [in] */ const AMVAEndFrameInfo *pEndFrameInfo);
        
        DECLSPEC_XFGVIRT(IAMVideoAccelerator, GetBuffer)
        HRESULT ( STDMETHODCALLTYPE *GetBuffer )( 
            IAMVideoAccelerator * This,
            /* [in] */ DWORD dwTypeIndex,
            /* [in] */ DWORD dwBufferIndex,
            /* [in] */ BOOL bReadOnly,
            /* [annotation][out] */ 
            _Out_  LPVOID *ppBuffer,
            /* [annotation][out] */ 
            _Out_  LONG *lpStride);
        
        DECLSPEC_XFGVIRT(IAMVideoAccelerator, ReleaseBuffer)
        HRESULT ( STDMETHODCALLTYPE *ReleaseBuffer )( 
            IAMVideoAccelerator * This,
            /* [in] */ DWORD dwTypeIndex,
            /* [in] */ DWORD dwBufferIndex);
        
        DECLSPEC_XFGVIRT(IAMVideoAccelerator, Execute)
        HRESULT ( STDMETHODCALLTYPE *Execute )( 
            IAMVideoAccelerator * This,
            /* [in] */ DWORD dwFunction,
            /* [in] */ LPVOID lpPrivateInputData,
            /* [in] */ DWORD cbPrivateInputData,
            /* [in] */ LPVOID lpPrivateOutputDat,
            /* [in] */ DWORD cbPrivateOutputData,
            /* [in] */ DWORD dwNumBuffers,
            /* [annotation][in] */ 
            _In_reads_(dwNumBuffers)  const AMVABUFFERINFO *pamvaBufferInfo);
        
        DECLSPEC_XFGVIRT(IAMVideoAccelerator, QueryRenderStatus)
        HRESULT ( STDMETHODCALLTYPE *QueryRenderStatus )( 
            IAMVideoAccelerator * This,
            /* [in] */ DWORD dwTypeIndex,
            /* [in] */ DWORD dwBufferIndex,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IAMVideoAccelerator, DisplayFrame)
        HRESULT ( STDMETHODCALLTYPE *DisplayFrame )( 
            IAMVideoAccelerator * This,
            /* [in] */ DWORD dwFlipToIndex,
            /* [in] */ IMediaSample *pMediaSample);
        
        END_INTERFACE
    } IAMVideoAcceleratorVtbl;

    interface IAMVideoAccelerator
    {
        CONST_VTBL struct IAMVideoAcceleratorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAMVideoAccelerator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAMVideoAccelerator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAMVideoAccelerator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAMVideoAccelerator_GetVideoAcceleratorGUIDs(This,pdwNumGuidsSupported,pGuidsSupported)	\
    ( (This)->lpVtbl -> GetVideoAcceleratorGUIDs(This,pdwNumGuidsSupported,pGuidsSupported) ) 

#define IAMVideoAccelerator_GetUncompFormatsSupported(This,pGuid,pdwNumFormatsSupported,pFormatsSupported)	\
    ( (This)->lpVtbl -> GetUncompFormatsSupported(This,pGuid,pdwNumFormatsSupported,pFormatsSupported) ) 

#define IAMVideoAccelerator_GetInternalMemInfo(This,pGuid,pamvaUncompDataInfo,pamvaInternalMemInfo)	\
    ( (This)->lpVtbl -> GetInternalMemInfo(This,pGuid,pamvaUncompDataInfo,pamvaInternalMemInfo) ) 

#define IAMVideoAccelerator_GetCompBufferInfo(This,pGuid,pamvaUncompDataInfo,pdwNumTypesCompBuffers,pamvaCompBufferInfo)	\
    ( (This)->lpVtbl -> GetCompBufferInfo(This,pGuid,pamvaUncompDataInfo,pdwNumTypesCompBuffers,pamvaCompBufferInfo) ) 

#define IAMVideoAccelerator_GetInternalCompBufferInfo(This,pdwNumTypesCompBuffers,pamvaCompBufferInfo)	\
    ( (This)->lpVtbl -> GetInternalCompBufferInfo(This,pdwNumTypesCompBuffers,pamvaCompBufferInfo) ) 

#define IAMVideoAccelerator_BeginFrame(This,amvaBeginFrameInfo)	\
    ( (This)->lpVtbl -> BeginFrame(This,amvaBeginFrameInfo) ) 

#define IAMVideoAccelerator_EndFrame(This,pEndFrameInfo)	\
    ( (This)->lpVtbl -> EndFrame(This,pEndFrameInfo) ) 

#define IAMVideoAccelerator_GetBuffer(This,dwTypeIndex,dwBufferIndex,bReadOnly,ppBuffer,lpStride)	\
    ( (This)->lpVtbl -> GetBuffer(This,dwTypeIndex,dwBufferIndex,bReadOnly,ppBuffer,lpStride) ) 

#define IAMVideoAccelerator_ReleaseBuffer(This,dwTypeIndex,dwBufferIndex)	\
    ( (This)->lpVtbl -> ReleaseBuffer(This,dwTypeIndex,dwBufferIndex) ) 

#define IAMVideoAccelerator_Execute(This,dwFunction,lpPrivateInputData,cbPrivateInputData,lpPrivateOutputDat,cbPrivateOutputData,dwNumBuffers,pamvaBufferInfo)	\
    ( (This)->lpVtbl -> Execute(This,dwFunction,lpPrivateInputData,cbPrivateInputData,lpPrivateOutputDat,cbPrivateOutputData,dwNumBuffers,pamvaBufferInfo) ) 

#define IAMVideoAccelerator_QueryRenderStatus(This,dwTypeIndex,dwBufferIndex,dwFlags)	\
    ( (This)->lpVtbl -> QueryRenderStatus(This,dwTypeIndex,dwBufferIndex,dwFlags) ) 

#define IAMVideoAccelerator_DisplayFrame(This,dwFlipToIndex,pMediaSample)	\
    ( (This)->lpVtbl -> DisplayFrame(This,dwFlipToIndex,pMediaSample) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAMVideoAccelerator_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_videoacc_0000_0002 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_videoacc_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_videoacc_0000_0002_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif



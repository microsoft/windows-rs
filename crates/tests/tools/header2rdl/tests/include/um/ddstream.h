

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

#ifndef __ddstream_h__
#define __ddstream_h__

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

#ifndef __IDirectDrawMediaStream_FWD_DEFINED__
#define __IDirectDrawMediaStream_FWD_DEFINED__
typedef interface IDirectDrawMediaStream IDirectDrawMediaStream;

#endif 	/* __IDirectDrawMediaStream_FWD_DEFINED__ */


#ifndef __IDirectDrawStreamSample_FWD_DEFINED__
#define __IDirectDrawStreamSample_FWD_DEFINED__
typedef interface IDirectDrawStreamSample IDirectDrawStreamSample;

#endif 	/* __IDirectDrawStreamSample_FWD_DEFINED__ */


/* header files for imported files */
#include "unknwn.h"
#include "mmstream.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_ddstream_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
//
//   The following declarations within the 'if 0' block are dummy typedefs used to make
//   the ddstream.idl file build.  The actual definitions are contained in DDRAW.H
//
#if 0
typedef void *LPDDSURFACEDESC;

typedef struct tDDSURFACEDESC DDSURFACEDESC;

#endif
#include <ddraw.h>

enum __MIDL___MIDL_itf_ddstream_0000_0000_0001
    {
        DDSFF_PROGRESSIVERENDER	= 0x1
    } ;







extern RPC_IF_HANDLE __MIDL_itf_ddstream_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_ddstream_0000_0000_v0_0_s_ifspec;

#ifndef __IDirectDrawMediaStream_INTERFACE_DEFINED__
#define __IDirectDrawMediaStream_INTERFACE_DEFINED__

/* interface IDirectDrawMediaStream */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_IDirectDrawMediaStream;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F4104FCE-9A70-11d0-8FDE-00C04FD9189D")
    IDirectDrawMediaStream : public IMediaStream
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetFormat( 
            /* [out] */ DDSURFACEDESC *pDDSDCurrent,
            /* [out] */ IDirectDrawPalette **ppDirectDrawPalette,
            /* [out] */ DDSURFACEDESC *pDDSDDesired,
            /* [out] */ DWORD *pdwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetFormat( 
            /* [in] */ const DDSURFACEDESC *pDDSurfaceDesc,
            /* [in] */ IDirectDrawPalette *pDirectDrawPalette) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDirectDraw( 
            /* [out] */ IDirectDraw **ppDirectDraw) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDirectDraw( 
            /* [in] */ IDirectDraw *pDirectDraw) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateSample( 
            /* [in] */ IDirectDrawSurface *pSurface,
            /* [in] */ const RECT *pRect,
            /* [in] */ DWORD dwFlags,
            /* [out] */ IDirectDrawStreamSample **ppSample) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTimePerFrame( 
            /* [out] */ STREAM_TIME *pFrameTime) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDirectDrawMediaStreamVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDirectDrawMediaStream * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDirectDrawMediaStream * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDirectDrawMediaStream * This);
        
        DECLSPEC_XFGVIRT(IMediaStream, GetMultiMediaStream)
        HRESULT ( STDMETHODCALLTYPE *GetMultiMediaStream )( 
            IDirectDrawMediaStream * This,
            /* [out] */ IMultiMediaStream **ppMultiMediaStream);
        
        DECLSPEC_XFGVIRT(IMediaStream, GetInformation)
        HRESULT ( STDMETHODCALLTYPE *GetInformation )( 
            IDirectDrawMediaStream * This,
            /* [out] */ MSPID *pPurposeId,
            /* [out] */ STREAM_TYPE *pType);
        
        DECLSPEC_XFGVIRT(IMediaStream, SetSameFormat)
        HRESULT ( STDMETHODCALLTYPE *SetSameFormat )( 
            IDirectDrawMediaStream * This,
            /* [in] */ IMediaStream *pStreamThatHasDesiredFormat,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IMediaStream, AllocateSample)
        HRESULT ( STDMETHODCALLTYPE *AllocateSample )( 
            IDirectDrawMediaStream * This,
            /* [in] */ DWORD dwFlags,
            /* [out] */ IStreamSample **ppSample);
        
        DECLSPEC_XFGVIRT(IMediaStream, CreateSharedSample)
        HRESULT ( STDMETHODCALLTYPE *CreateSharedSample )( 
            IDirectDrawMediaStream * This,
            /* [in] */ IStreamSample *pExistingSample,
            /* [in] */ DWORD dwFlags,
            /* [out] */ IStreamSample **ppNewSample);
        
        DECLSPEC_XFGVIRT(IMediaStream, SendEndOfStream)
        HRESULT ( STDMETHODCALLTYPE *SendEndOfStream )( 
            IDirectDrawMediaStream * This,
            DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IDirectDrawMediaStream, GetFormat)
        HRESULT ( STDMETHODCALLTYPE *GetFormat )( 
            IDirectDrawMediaStream * This,
            /* [out] */ DDSURFACEDESC *pDDSDCurrent,
            /* [out] */ IDirectDrawPalette **ppDirectDrawPalette,
            /* [out] */ DDSURFACEDESC *pDDSDDesired,
            /* [out] */ DWORD *pdwFlags);
        
        DECLSPEC_XFGVIRT(IDirectDrawMediaStream, SetFormat)
        HRESULT ( STDMETHODCALLTYPE *SetFormat )( 
            IDirectDrawMediaStream * This,
            /* [in] */ const DDSURFACEDESC *pDDSurfaceDesc,
            /* [in] */ IDirectDrawPalette *pDirectDrawPalette);
        
        DECLSPEC_XFGVIRT(IDirectDrawMediaStream, GetDirectDraw)
        HRESULT ( STDMETHODCALLTYPE *GetDirectDraw )( 
            IDirectDrawMediaStream * This,
            /* [out] */ IDirectDraw **ppDirectDraw);
        
        DECLSPEC_XFGVIRT(IDirectDrawMediaStream, SetDirectDraw)
        HRESULT ( STDMETHODCALLTYPE *SetDirectDraw )( 
            IDirectDrawMediaStream * This,
            /* [in] */ IDirectDraw *pDirectDraw);
        
        DECLSPEC_XFGVIRT(IDirectDrawMediaStream, CreateSample)
        HRESULT ( STDMETHODCALLTYPE *CreateSample )( 
            IDirectDrawMediaStream * This,
            /* [in] */ IDirectDrawSurface *pSurface,
            /* [in] */ const RECT *pRect,
            /* [in] */ DWORD dwFlags,
            /* [out] */ IDirectDrawStreamSample **ppSample);
        
        DECLSPEC_XFGVIRT(IDirectDrawMediaStream, GetTimePerFrame)
        HRESULT ( STDMETHODCALLTYPE *GetTimePerFrame )( 
            IDirectDrawMediaStream * This,
            /* [out] */ STREAM_TIME *pFrameTime);
        
        END_INTERFACE
    } IDirectDrawMediaStreamVtbl;

    interface IDirectDrawMediaStream
    {
        CONST_VTBL struct IDirectDrawMediaStreamVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDirectDrawMediaStream_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDirectDrawMediaStream_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDirectDrawMediaStream_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDirectDrawMediaStream_GetMultiMediaStream(This,ppMultiMediaStream)	\
    ( (This)->lpVtbl -> GetMultiMediaStream(This,ppMultiMediaStream) ) 

#define IDirectDrawMediaStream_GetInformation(This,pPurposeId,pType)	\
    ( (This)->lpVtbl -> GetInformation(This,pPurposeId,pType) ) 

#define IDirectDrawMediaStream_SetSameFormat(This,pStreamThatHasDesiredFormat,dwFlags)	\
    ( (This)->lpVtbl -> SetSameFormat(This,pStreamThatHasDesiredFormat,dwFlags) ) 

#define IDirectDrawMediaStream_AllocateSample(This,dwFlags,ppSample)	\
    ( (This)->lpVtbl -> AllocateSample(This,dwFlags,ppSample) ) 

#define IDirectDrawMediaStream_CreateSharedSample(This,pExistingSample,dwFlags,ppNewSample)	\
    ( (This)->lpVtbl -> CreateSharedSample(This,pExistingSample,dwFlags,ppNewSample) ) 

#define IDirectDrawMediaStream_SendEndOfStream(This,dwFlags)	\
    ( (This)->lpVtbl -> SendEndOfStream(This,dwFlags) ) 


#define IDirectDrawMediaStream_GetFormat(This,pDDSDCurrent,ppDirectDrawPalette,pDDSDDesired,pdwFlags)	\
    ( (This)->lpVtbl -> GetFormat(This,pDDSDCurrent,ppDirectDrawPalette,pDDSDDesired,pdwFlags) ) 

#define IDirectDrawMediaStream_SetFormat(This,pDDSurfaceDesc,pDirectDrawPalette)	\
    ( (This)->lpVtbl -> SetFormat(This,pDDSurfaceDesc,pDirectDrawPalette) ) 

#define IDirectDrawMediaStream_GetDirectDraw(This,ppDirectDraw)	\
    ( (This)->lpVtbl -> GetDirectDraw(This,ppDirectDraw) ) 

#define IDirectDrawMediaStream_SetDirectDraw(This,pDirectDraw)	\
    ( (This)->lpVtbl -> SetDirectDraw(This,pDirectDraw) ) 

#define IDirectDrawMediaStream_CreateSample(This,pSurface,pRect,dwFlags,ppSample)	\
    ( (This)->lpVtbl -> CreateSample(This,pSurface,pRect,dwFlags,ppSample) ) 

#define IDirectDrawMediaStream_GetTimePerFrame(This,pFrameTime)	\
    ( (This)->lpVtbl -> GetTimePerFrame(This,pFrameTime) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDirectDrawMediaStream_INTERFACE_DEFINED__ */


#ifndef __IDirectDrawStreamSample_INTERFACE_DEFINED__
#define __IDirectDrawStreamSample_INTERFACE_DEFINED__

/* interface IDirectDrawStreamSample */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_IDirectDrawStreamSample;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F4104FCF-9A70-11d0-8FDE-00C04FD9189D")
    IDirectDrawStreamSample : public IStreamSample
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetSurface( 
            /* [out] */ IDirectDrawSurface **ppDirectDrawSurface,
            /* [out] */ RECT *pRect) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetRect( 
            /* [in] */ const RECT *pRect) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDirectDrawStreamSampleVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDirectDrawStreamSample * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDirectDrawStreamSample * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDirectDrawStreamSample * This);
        
        DECLSPEC_XFGVIRT(IStreamSample, GetMediaStream)
        HRESULT ( STDMETHODCALLTYPE *GetMediaStream )( 
            IDirectDrawStreamSample * This,
            /* [in] */ IMediaStream **ppMediaStream);
        
        DECLSPEC_XFGVIRT(IStreamSample, GetSampleTimes)
        HRESULT ( STDMETHODCALLTYPE *GetSampleTimes )( 
            IDirectDrawStreamSample * This,
            /* [out] */ STREAM_TIME *pStartTime,
            /* [out] */ STREAM_TIME *pEndTime,
            /* [out] */ STREAM_TIME *pCurrentTime);
        
        DECLSPEC_XFGVIRT(IStreamSample, SetSampleTimes)
        HRESULT ( STDMETHODCALLTYPE *SetSampleTimes )( 
            IDirectDrawStreamSample * This,
            /* [in] */ const STREAM_TIME *pStartTime,
            /* [in] */ const STREAM_TIME *pEndTime);
        
        DECLSPEC_XFGVIRT(IStreamSample, Update)
        HRESULT ( STDMETHODCALLTYPE *Update )( 
            IDirectDrawStreamSample * This,
            /* [in] */ DWORD dwFlags,
            /* [in] */ HANDLE hEvent,
            /* [in] */ PAPCFUNC pfnAPC,
            /* [in] */ DWORD_PTR dwAPCData);
        
        DECLSPEC_XFGVIRT(IStreamSample, CompletionStatus)
        HRESULT ( STDMETHODCALLTYPE *CompletionStatus )( 
            IDirectDrawStreamSample * This,
            /* [in] */ DWORD dwFlags,
            /* [in] */ DWORD dwMilliseconds);
        
        DECLSPEC_XFGVIRT(IDirectDrawStreamSample, GetSurface)
        HRESULT ( STDMETHODCALLTYPE *GetSurface )( 
            IDirectDrawStreamSample * This,
            /* [out] */ IDirectDrawSurface **ppDirectDrawSurface,
            /* [out] */ RECT *pRect);
        
        DECLSPEC_XFGVIRT(IDirectDrawStreamSample, SetRect)
        HRESULT ( STDMETHODCALLTYPE *SetRect )( 
            IDirectDrawStreamSample * This,
            /* [in] */ const RECT *pRect);
        
        END_INTERFACE
    } IDirectDrawStreamSampleVtbl;

    interface IDirectDrawStreamSample
    {
        CONST_VTBL struct IDirectDrawStreamSampleVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDirectDrawStreamSample_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDirectDrawStreamSample_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDirectDrawStreamSample_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDirectDrawStreamSample_GetMediaStream(This,ppMediaStream)	\
    ( (This)->lpVtbl -> GetMediaStream(This,ppMediaStream) ) 

#define IDirectDrawStreamSample_GetSampleTimes(This,pStartTime,pEndTime,pCurrentTime)	\
    ( (This)->lpVtbl -> GetSampleTimes(This,pStartTime,pEndTime,pCurrentTime) ) 

#define IDirectDrawStreamSample_SetSampleTimes(This,pStartTime,pEndTime)	\
    ( (This)->lpVtbl -> SetSampleTimes(This,pStartTime,pEndTime) ) 

#define IDirectDrawStreamSample_Update(This,dwFlags,hEvent,pfnAPC,dwAPCData)	\
    ( (This)->lpVtbl -> Update(This,dwFlags,hEvent,pfnAPC,dwAPCData) ) 

#define IDirectDrawStreamSample_CompletionStatus(This,dwFlags,dwMilliseconds)	\
    ( (This)->lpVtbl -> CompletionStatus(This,dwFlags,dwMilliseconds) ) 


#define IDirectDrawStreamSample_GetSurface(This,ppDirectDrawSurface,pRect)	\
    ( (This)->lpVtbl -> GetSurface(This,ppDirectDrawSurface,pRect) ) 

#define IDirectDrawStreamSample_SetRect(This,pRect)	\
    ( (This)->lpVtbl -> SetRect(This,pRect) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDirectDrawStreamSample_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_ddstream_0000_0002 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_ddstream_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_ddstream_0000_0002_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif



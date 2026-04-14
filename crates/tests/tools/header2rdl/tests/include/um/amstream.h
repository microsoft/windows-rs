

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

#ifndef __amstream_h__
#define __amstream_h__

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

#ifndef __IAMMultiMediaStream_FWD_DEFINED__
#define __IAMMultiMediaStream_FWD_DEFINED__
typedef interface IAMMultiMediaStream IAMMultiMediaStream;

#endif 	/* __IAMMultiMediaStream_FWD_DEFINED__ */


#ifndef __IAMMediaStream_FWD_DEFINED__
#define __IAMMediaStream_FWD_DEFINED__
typedef interface IAMMediaStream IAMMediaStream;

#endif 	/* __IAMMediaStream_FWD_DEFINED__ */


#ifndef __IMediaStreamFilter_FWD_DEFINED__
#define __IMediaStreamFilter_FWD_DEFINED__
typedef interface IMediaStreamFilter IMediaStreamFilter;

#endif 	/* __IMediaStreamFilter_FWD_DEFINED__ */


#ifndef __IDirectDrawMediaSampleAllocator_FWD_DEFINED__
#define __IDirectDrawMediaSampleAllocator_FWD_DEFINED__
typedef interface IDirectDrawMediaSampleAllocator IDirectDrawMediaSampleAllocator;

#endif 	/* __IDirectDrawMediaSampleAllocator_FWD_DEFINED__ */


#ifndef __IDirectDrawMediaSample_FWD_DEFINED__
#define __IDirectDrawMediaSample_FWD_DEFINED__
typedef interface IDirectDrawMediaSample IDirectDrawMediaSample;

#endif 	/* __IDirectDrawMediaSample_FWD_DEFINED__ */


#ifndef __IAMMediaTypeStream_FWD_DEFINED__
#define __IAMMediaTypeStream_FWD_DEFINED__
typedef interface IAMMediaTypeStream IAMMediaTypeStream;

#endif 	/* __IAMMediaTypeStream_FWD_DEFINED__ */


#ifndef __IAMMediaTypeSample_FWD_DEFINED__
#define __IAMMediaTypeSample_FWD_DEFINED__
typedef interface IAMMediaTypeSample IAMMediaTypeSample;

#endif 	/* __IAMMediaTypeSample_FWD_DEFINED__ */


/* header files for imported files */
#include "unknwn.h"
#include "mmstream.h"
#include "strmif.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_amstream_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#include <ddraw.h>
#include <mmsystem.h>
#include <mmstream.h>
#include <ddstream.h>
#include <austream.h>








enum __MIDL___MIDL_itf_amstream_0000_0000_0001
    {
        AMMSF_NOGRAPHTHREAD	= 0x1
    } ;

enum __MIDL___MIDL_itf_amstream_0000_0000_0002
    {
        AMMSF_ADDDEFAULTRENDERER	= 0x1,
        AMMSF_CREATEPEER	= 0x2,
        AMMSF_STOPIFNOSAMPLES	= 0x4,
        AMMSF_NOSTALL	= 0x8
    } ;

enum __MIDL___MIDL_itf_amstream_0000_0000_0003
    {
        AMMSF_RENDERTYPEMASK	= 0x3,
        AMMSF_RENDERTOEXISTING	= 0,
        AMMSF_RENDERALLSTREAMS	= 0x1,
        AMMSF_NORENDER	= 0x2,
        AMMSF_NOCLOCK	= 0x4,
        AMMSF_RUN	= 0x8
    } ;
typedef /* [public][v1_enum] */ 
enum __MIDL___MIDL_itf_amstream_0000_0000_0004
    {
        Disabled	= 0,
        ReadData	= 1,
        RenderData	= 2
    } 	OUTPUT_STATE;



extern RPC_IF_HANDLE __MIDL_itf_amstream_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_amstream_0000_0000_v0_0_s_ifspec;

#ifndef __IAMMultiMediaStream_INTERFACE_DEFINED__
#define __IAMMultiMediaStream_INTERFACE_DEFINED__

/* interface IAMMultiMediaStream */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IAMMultiMediaStream;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BEBE595C-9A6F-11d0-8FDE-00C04FD9189D")
    IAMMultiMediaStream : public IMultiMediaStream
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ STREAM_TYPE StreamType,
            /* [in] */ DWORD dwFlags,
            /* [in] */ __RPC__in_opt IGraphBuilder *pFilterGraph) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFilterGraph( 
            /* [out] */ __RPC__deref_out_opt IGraphBuilder **ppGraphBuilder) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFilter( 
            /* [out] */ __RPC__deref_out_opt IMediaStreamFilter **ppFilter) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddMediaStream( 
            /* [in] */ __RPC__in_opt IUnknown *pStreamObject,
            /* [in] */ __RPC__in const MSPID *PurposeId,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__deref_out_opt IMediaStream **ppNewStream) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OpenFile( 
            /* [in] */ __RPC__in LPCWSTR pszFileName,
            /* [in] */ DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OpenMoniker( 
            /* [in] */ __RPC__in_opt IBindCtx *pCtx,
            /* [in] */ __RPC__in_opt IMoniker *pMoniker,
            /* [in] */ DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Render( 
            /* [in] */ DWORD dwFlags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAMMultiMediaStreamVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAMMultiMediaStream * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAMMultiMediaStream * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAMMultiMediaStream * This);
        
        DECLSPEC_XFGVIRT(IMultiMediaStream, GetInformation)
        HRESULT ( STDMETHODCALLTYPE *GetInformation )( 
            __RPC__in IAMMultiMediaStream * This,
            /* [out] */ __RPC__out DWORD *pdwFlags,
            /* [out] */ __RPC__out STREAM_TYPE *pStreamType);
        
        DECLSPEC_XFGVIRT(IMultiMediaStream, GetMediaStream)
        HRESULT ( STDMETHODCALLTYPE *GetMediaStream )( 
            __RPC__in IAMMultiMediaStream * This,
            /* [in] */ __RPC__in REFMSPID idPurpose,
            /* [out] */ __RPC__deref_out_opt IMediaStream **ppMediaStream);
        
        DECLSPEC_XFGVIRT(IMultiMediaStream, EnumMediaStreams)
        HRESULT ( STDMETHODCALLTYPE *EnumMediaStreams )( 
            __RPC__in IAMMultiMediaStream * This,
            /* [in] */ long Index,
            /* [out] */ __RPC__deref_out_opt IMediaStream **ppMediaStream);
        
        DECLSPEC_XFGVIRT(IMultiMediaStream, GetState)
        HRESULT ( STDMETHODCALLTYPE *GetState )( 
            __RPC__in IAMMultiMediaStream * This,
            /* [out] */ __RPC__out STREAM_STATE *pCurrentState);
        
        DECLSPEC_XFGVIRT(IMultiMediaStream, SetState)
        HRESULT ( STDMETHODCALLTYPE *SetState )( 
            __RPC__in IAMMultiMediaStream * This,
            /* [in] */ STREAM_STATE NewState);
        
        DECLSPEC_XFGVIRT(IMultiMediaStream, GetTime)
        HRESULT ( STDMETHODCALLTYPE *GetTime )( 
            __RPC__in IAMMultiMediaStream * This,
            /* [out] */ __RPC__out STREAM_TIME *pCurrentTime);
        
        DECLSPEC_XFGVIRT(IMultiMediaStream, GetDuration)
        HRESULT ( STDMETHODCALLTYPE *GetDuration )( 
            __RPC__in IAMMultiMediaStream * This,
            /* [out] */ __RPC__out STREAM_TIME *pDuration);
        
        DECLSPEC_XFGVIRT(IMultiMediaStream, Seek)
        HRESULT ( STDMETHODCALLTYPE *Seek )( 
            __RPC__in IAMMultiMediaStream * This,
            /* [in] */ STREAM_TIME SeekTime);
        
        DECLSPEC_XFGVIRT(IMultiMediaStream, GetEndOfStreamEventHandle)
        HRESULT ( STDMETHODCALLTYPE *GetEndOfStreamEventHandle )( 
            __RPC__in IAMMultiMediaStream * This,
            /* [out] */ __RPC__deref_out_opt HANDLE *phEOS);
        
        DECLSPEC_XFGVIRT(IAMMultiMediaStream, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            __RPC__in IAMMultiMediaStream * This,
            /* [in] */ STREAM_TYPE StreamType,
            /* [in] */ DWORD dwFlags,
            /* [in] */ __RPC__in_opt IGraphBuilder *pFilterGraph);
        
        DECLSPEC_XFGVIRT(IAMMultiMediaStream, GetFilterGraph)
        HRESULT ( STDMETHODCALLTYPE *GetFilterGraph )( 
            __RPC__in IAMMultiMediaStream * This,
            /* [out] */ __RPC__deref_out_opt IGraphBuilder **ppGraphBuilder);
        
        DECLSPEC_XFGVIRT(IAMMultiMediaStream, GetFilter)
        HRESULT ( STDMETHODCALLTYPE *GetFilter )( 
            __RPC__in IAMMultiMediaStream * This,
            /* [out] */ __RPC__deref_out_opt IMediaStreamFilter **ppFilter);
        
        DECLSPEC_XFGVIRT(IAMMultiMediaStream, AddMediaStream)
        HRESULT ( STDMETHODCALLTYPE *AddMediaStream )( 
            __RPC__in IAMMultiMediaStream * This,
            /* [in] */ __RPC__in_opt IUnknown *pStreamObject,
            /* [in] */ __RPC__in const MSPID *PurposeId,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__deref_out_opt IMediaStream **ppNewStream);
        
        DECLSPEC_XFGVIRT(IAMMultiMediaStream, OpenFile)
        HRESULT ( STDMETHODCALLTYPE *OpenFile )( 
            __RPC__in IAMMultiMediaStream * This,
            /* [in] */ __RPC__in LPCWSTR pszFileName,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IAMMultiMediaStream, OpenMoniker)
        HRESULT ( STDMETHODCALLTYPE *OpenMoniker )( 
            __RPC__in IAMMultiMediaStream * This,
            /* [in] */ __RPC__in_opt IBindCtx *pCtx,
            /* [in] */ __RPC__in_opt IMoniker *pMoniker,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IAMMultiMediaStream, Render)
        HRESULT ( STDMETHODCALLTYPE *Render )( 
            __RPC__in IAMMultiMediaStream * This,
            /* [in] */ DWORD dwFlags);
        
        END_INTERFACE
    } IAMMultiMediaStreamVtbl;

    interface IAMMultiMediaStream
    {
        CONST_VTBL struct IAMMultiMediaStreamVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAMMultiMediaStream_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAMMultiMediaStream_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAMMultiMediaStream_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAMMultiMediaStream_GetInformation(This,pdwFlags,pStreamType)	\
    ( (This)->lpVtbl -> GetInformation(This,pdwFlags,pStreamType) ) 

#define IAMMultiMediaStream_GetMediaStream(This,idPurpose,ppMediaStream)	\
    ( (This)->lpVtbl -> GetMediaStream(This,idPurpose,ppMediaStream) ) 

#define IAMMultiMediaStream_EnumMediaStreams(This,Index,ppMediaStream)	\
    ( (This)->lpVtbl -> EnumMediaStreams(This,Index,ppMediaStream) ) 

#define IAMMultiMediaStream_GetState(This,pCurrentState)	\
    ( (This)->lpVtbl -> GetState(This,pCurrentState) ) 

#define IAMMultiMediaStream_SetState(This,NewState)	\
    ( (This)->lpVtbl -> SetState(This,NewState) ) 

#define IAMMultiMediaStream_GetTime(This,pCurrentTime)	\
    ( (This)->lpVtbl -> GetTime(This,pCurrentTime) ) 

#define IAMMultiMediaStream_GetDuration(This,pDuration)	\
    ( (This)->lpVtbl -> GetDuration(This,pDuration) ) 

#define IAMMultiMediaStream_Seek(This,SeekTime)	\
    ( (This)->lpVtbl -> Seek(This,SeekTime) ) 

#define IAMMultiMediaStream_GetEndOfStreamEventHandle(This,phEOS)	\
    ( (This)->lpVtbl -> GetEndOfStreamEventHandle(This,phEOS) ) 


#define IAMMultiMediaStream_Initialize(This,StreamType,dwFlags,pFilterGraph)	\
    ( (This)->lpVtbl -> Initialize(This,StreamType,dwFlags,pFilterGraph) ) 

#define IAMMultiMediaStream_GetFilterGraph(This,ppGraphBuilder)	\
    ( (This)->lpVtbl -> GetFilterGraph(This,ppGraphBuilder) ) 

#define IAMMultiMediaStream_GetFilter(This,ppFilter)	\
    ( (This)->lpVtbl -> GetFilter(This,ppFilter) ) 

#define IAMMultiMediaStream_AddMediaStream(This,pStreamObject,PurposeId,dwFlags,ppNewStream)	\
    ( (This)->lpVtbl -> AddMediaStream(This,pStreamObject,PurposeId,dwFlags,ppNewStream) ) 

#define IAMMultiMediaStream_OpenFile(This,pszFileName,dwFlags)	\
    ( (This)->lpVtbl -> OpenFile(This,pszFileName,dwFlags) ) 

#define IAMMultiMediaStream_OpenMoniker(This,pCtx,pMoniker,dwFlags)	\
    ( (This)->lpVtbl -> OpenMoniker(This,pCtx,pMoniker,dwFlags) ) 

#define IAMMultiMediaStream_Render(This,dwFlags)	\
    ( (This)->lpVtbl -> Render(This,dwFlags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAMMultiMediaStream_INTERFACE_DEFINED__ */


#ifndef __IAMMediaStream_INTERFACE_DEFINED__
#define __IAMMediaStream_INTERFACE_DEFINED__

/* interface IAMMediaStream */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IAMMediaStream;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BEBE595D-9A6F-11d0-8FDE-00C04FD9189D")
    IAMMediaStream : public IMediaStream
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ __RPC__in_opt IUnknown *pSourceObject,
            /* [in] */ DWORD dwFlags,
            /* [in] */ __RPC__in REFMSPID PurposeId,
            /* [in] */ const STREAM_TYPE StreamType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetState( 
            /* [in] */ FILTER_STATE State) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE JoinAMMultiMediaStream( 
            /* [in] */ __RPC__in_opt IAMMultiMediaStream *pAMMultiMediaStream) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE JoinFilter( 
            /* [in] */ __RPC__in_opt IMediaStreamFilter *pMediaStreamFilter) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE JoinFilterGraph( 
            /* [in] */ __RPC__in_opt IFilterGraph *pFilterGraph) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAMMediaStreamVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAMMediaStream * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAMMediaStream * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAMMediaStream * This);
        
        DECLSPEC_XFGVIRT(IMediaStream, GetMultiMediaStream)
        HRESULT ( STDMETHODCALLTYPE *GetMultiMediaStream )( 
            __RPC__in IAMMediaStream * This,
            /* [out] */ __RPC__deref_out_opt IMultiMediaStream **ppMultiMediaStream);
        
        DECLSPEC_XFGVIRT(IMediaStream, GetInformation)
        HRESULT ( STDMETHODCALLTYPE *GetInformation )( 
            __RPC__in IAMMediaStream * This,
            /* [out] */ __RPC__out MSPID *pPurposeId,
            /* [out] */ __RPC__out STREAM_TYPE *pType);
        
        DECLSPEC_XFGVIRT(IMediaStream, SetSameFormat)
        HRESULT ( STDMETHODCALLTYPE *SetSameFormat )( 
            __RPC__in IAMMediaStream * This,
            /* [in] */ __RPC__in_opt IMediaStream *pStreamThatHasDesiredFormat,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IMediaStream, AllocateSample)
        HRESULT ( STDMETHODCALLTYPE *AllocateSample )( 
            __RPC__in IAMMediaStream * This,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__deref_out_opt IStreamSample **ppSample);
        
        DECLSPEC_XFGVIRT(IMediaStream, CreateSharedSample)
        HRESULT ( STDMETHODCALLTYPE *CreateSharedSample )( 
            __RPC__in IAMMediaStream * This,
            /* [in] */ __RPC__in_opt IStreamSample *pExistingSample,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__deref_out_opt IStreamSample **ppNewSample);
        
        DECLSPEC_XFGVIRT(IMediaStream, SendEndOfStream)
        HRESULT ( STDMETHODCALLTYPE *SendEndOfStream )( 
            __RPC__in IAMMediaStream * This,
            DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IAMMediaStream, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            __RPC__in IAMMediaStream * This,
            /* [in] */ __RPC__in_opt IUnknown *pSourceObject,
            /* [in] */ DWORD dwFlags,
            /* [in] */ __RPC__in REFMSPID PurposeId,
            /* [in] */ const STREAM_TYPE StreamType);
        
        DECLSPEC_XFGVIRT(IAMMediaStream, SetState)
        HRESULT ( STDMETHODCALLTYPE *SetState )( 
            __RPC__in IAMMediaStream * This,
            /* [in] */ FILTER_STATE State);
        
        DECLSPEC_XFGVIRT(IAMMediaStream, JoinAMMultiMediaStream)
        HRESULT ( STDMETHODCALLTYPE *JoinAMMultiMediaStream )( 
            __RPC__in IAMMediaStream * This,
            /* [in] */ __RPC__in_opt IAMMultiMediaStream *pAMMultiMediaStream);
        
        DECLSPEC_XFGVIRT(IAMMediaStream, JoinFilter)
        HRESULT ( STDMETHODCALLTYPE *JoinFilter )( 
            __RPC__in IAMMediaStream * This,
            /* [in] */ __RPC__in_opt IMediaStreamFilter *pMediaStreamFilter);
        
        DECLSPEC_XFGVIRT(IAMMediaStream, JoinFilterGraph)
        HRESULT ( STDMETHODCALLTYPE *JoinFilterGraph )( 
            __RPC__in IAMMediaStream * This,
            /* [in] */ __RPC__in_opt IFilterGraph *pFilterGraph);
        
        END_INTERFACE
    } IAMMediaStreamVtbl;

    interface IAMMediaStream
    {
        CONST_VTBL struct IAMMediaStreamVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAMMediaStream_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAMMediaStream_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAMMediaStream_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAMMediaStream_GetMultiMediaStream(This,ppMultiMediaStream)	\
    ( (This)->lpVtbl -> GetMultiMediaStream(This,ppMultiMediaStream) ) 

#define IAMMediaStream_GetInformation(This,pPurposeId,pType)	\
    ( (This)->lpVtbl -> GetInformation(This,pPurposeId,pType) ) 

#define IAMMediaStream_SetSameFormat(This,pStreamThatHasDesiredFormat,dwFlags)	\
    ( (This)->lpVtbl -> SetSameFormat(This,pStreamThatHasDesiredFormat,dwFlags) ) 

#define IAMMediaStream_AllocateSample(This,dwFlags,ppSample)	\
    ( (This)->lpVtbl -> AllocateSample(This,dwFlags,ppSample) ) 

#define IAMMediaStream_CreateSharedSample(This,pExistingSample,dwFlags,ppNewSample)	\
    ( (This)->lpVtbl -> CreateSharedSample(This,pExistingSample,dwFlags,ppNewSample) ) 

#define IAMMediaStream_SendEndOfStream(This,dwFlags)	\
    ( (This)->lpVtbl -> SendEndOfStream(This,dwFlags) ) 


#define IAMMediaStream_Initialize(This,pSourceObject,dwFlags,PurposeId,StreamType)	\
    ( (This)->lpVtbl -> Initialize(This,pSourceObject,dwFlags,PurposeId,StreamType) ) 

#define IAMMediaStream_SetState(This,State)	\
    ( (This)->lpVtbl -> SetState(This,State) ) 

#define IAMMediaStream_JoinAMMultiMediaStream(This,pAMMultiMediaStream)	\
    ( (This)->lpVtbl -> JoinAMMultiMediaStream(This,pAMMultiMediaStream) ) 

#define IAMMediaStream_JoinFilter(This,pMediaStreamFilter)	\
    ( (This)->lpVtbl -> JoinFilter(This,pMediaStreamFilter) ) 

#define IAMMediaStream_JoinFilterGraph(This,pFilterGraph)	\
    ( (This)->lpVtbl -> JoinFilterGraph(This,pFilterGraph) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAMMediaStream_INTERFACE_DEFINED__ */


#ifndef __IMediaStreamFilter_INTERFACE_DEFINED__
#define __IMediaStreamFilter_INTERFACE_DEFINED__

/* interface IMediaStreamFilter */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_IMediaStreamFilter;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BEBE595E-9A6F-11d0-8FDE-00C04FD9189D")
    IMediaStreamFilter : public IBaseFilter
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AddMediaStream( 
            /* [in] */ IAMMediaStream *pAMMediaStream) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMediaStream( 
            /* [in] */ REFMSPID idPurpose,
            /* [out] */ IMediaStream **ppMediaStream) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumMediaStreams( 
            /* [in] */ long Index,
            /* [out] */ IMediaStream **ppMediaStream) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SupportSeeking( 
            /* [in] */ BOOL bRenderer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ReferenceTimeToStreamTime( 
            /* [out][in] */ REFERENCE_TIME *pTime) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCurrentStreamTime( 
            /* [out] */ REFERENCE_TIME *pCurrentStreamTime) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WaitUntil( 
            /* [in] */ REFERENCE_TIME WaitStreamTime) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Flush( 
            /* [in] */ BOOL bCancelEOS) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EndOfStream( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMediaStreamFilterVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMediaStreamFilter * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMediaStreamFilter * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMediaStreamFilter * This);
        
        DECLSPEC_XFGVIRT(IPersist, GetClassID)
        HRESULT ( STDMETHODCALLTYPE *GetClassID )( 
            IMediaStreamFilter * This,
            /* [out] */ CLSID *pClassID);
        
        DECLSPEC_XFGVIRT(IMediaFilter, Stop)
        HRESULT ( STDMETHODCALLTYPE *Stop )( 
            IMediaStreamFilter * This);
        
        DECLSPEC_XFGVIRT(IMediaFilter, Pause)
        HRESULT ( STDMETHODCALLTYPE *Pause )( 
            IMediaStreamFilter * This);
        
        DECLSPEC_XFGVIRT(IMediaFilter, Run)
        HRESULT ( STDMETHODCALLTYPE *Run )( 
            IMediaStreamFilter * This,
            REFERENCE_TIME tStart);
        
        DECLSPEC_XFGVIRT(IMediaFilter, GetState)
        HRESULT ( STDMETHODCALLTYPE *GetState )( 
            IMediaStreamFilter * This,
            /* [in] */ DWORD dwMilliSecsTimeout,
            /* [annotation][out] */ 
            _Out_  FILTER_STATE *State);
        
        DECLSPEC_XFGVIRT(IMediaFilter, SetSyncSource)
        HRESULT ( STDMETHODCALLTYPE *SetSyncSource )( 
            IMediaStreamFilter * This,
            /* [annotation][in] */ 
            _In_opt_  IReferenceClock *pClock);
        
        DECLSPEC_XFGVIRT(IMediaFilter, GetSyncSource)
        HRESULT ( STDMETHODCALLTYPE *GetSyncSource )( 
            IMediaStreamFilter * This,
            /* [annotation][out] */ 
            _Outptr_result_maybenull_  IReferenceClock **pClock);
        
        DECLSPEC_XFGVIRT(IBaseFilter, EnumPins)
        HRESULT ( STDMETHODCALLTYPE *EnumPins )( 
            IMediaStreamFilter * This,
            /* [annotation][out] */ 
            _Out_  IEnumPins **ppEnum);
        
        DECLSPEC_XFGVIRT(IBaseFilter, FindPin)
        HRESULT ( STDMETHODCALLTYPE *FindPin )( 
            IMediaStreamFilter * This,
            /* [string][in] */ LPCWSTR Id,
            /* [annotation][out] */ 
            _Out_  IPin **ppPin);
        
        DECLSPEC_XFGVIRT(IBaseFilter, QueryFilterInfo)
        HRESULT ( STDMETHODCALLTYPE *QueryFilterInfo )( 
            IMediaStreamFilter * This,
            /* [annotation][out] */ 
            _Out_  FILTER_INFO *pInfo);
        
        DECLSPEC_XFGVIRT(IBaseFilter, JoinFilterGraph)
        HRESULT ( STDMETHODCALLTYPE *JoinFilterGraph )( 
            IMediaStreamFilter * This,
            /* [annotation][in] */ 
            _In_opt_  IFilterGraph *pGraph,
            /* [annotation][string][in] */ 
            _In_opt_  LPCWSTR pName);
        
        DECLSPEC_XFGVIRT(IBaseFilter, QueryVendorInfo)
        HRESULT ( STDMETHODCALLTYPE *QueryVendorInfo )( 
            IMediaStreamFilter * This,
            /* [annotation][string][out] */ 
            _Out_  LPWSTR *pVendorInfo);
        
        DECLSPEC_XFGVIRT(IMediaStreamFilter, AddMediaStream)
        HRESULT ( STDMETHODCALLTYPE *AddMediaStream )( 
            IMediaStreamFilter * This,
            /* [in] */ IAMMediaStream *pAMMediaStream);
        
        DECLSPEC_XFGVIRT(IMediaStreamFilter, GetMediaStream)
        HRESULT ( STDMETHODCALLTYPE *GetMediaStream )( 
            IMediaStreamFilter * This,
            /* [in] */ REFMSPID idPurpose,
            /* [out] */ IMediaStream **ppMediaStream);
        
        DECLSPEC_XFGVIRT(IMediaStreamFilter, EnumMediaStreams)
        HRESULT ( STDMETHODCALLTYPE *EnumMediaStreams )( 
            IMediaStreamFilter * This,
            /* [in] */ long Index,
            /* [out] */ IMediaStream **ppMediaStream);
        
        DECLSPEC_XFGVIRT(IMediaStreamFilter, SupportSeeking)
        HRESULT ( STDMETHODCALLTYPE *SupportSeeking )( 
            IMediaStreamFilter * This,
            /* [in] */ BOOL bRenderer);
        
        DECLSPEC_XFGVIRT(IMediaStreamFilter, ReferenceTimeToStreamTime)
        HRESULT ( STDMETHODCALLTYPE *ReferenceTimeToStreamTime )( 
            IMediaStreamFilter * This,
            /* [out][in] */ REFERENCE_TIME *pTime);
        
        DECLSPEC_XFGVIRT(IMediaStreamFilter, GetCurrentStreamTime)
        HRESULT ( STDMETHODCALLTYPE *GetCurrentStreamTime )( 
            IMediaStreamFilter * This,
            /* [out] */ REFERENCE_TIME *pCurrentStreamTime);
        
        DECLSPEC_XFGVIRT(IMediaStreamFilter, WaitUntil)
        HRESULT ( STDMETHODCALLTYPE *WaitUntil )( 
            IMediaStreamFilter * This,
            /* [in] */ REFERENCE_TIME WaitStreamTime);
        
        DECLSPEC_XFGVIRT(IMediaStreamFilter, Flush)
        HRESULT ( STDMETHODCALLTYPE *Flush )( 
            IMediaStreamFilter * This,
            /* [in] */ BOOL bCancelEOS);
        
        DECLSPEC_XFGVIRT(IMediaStreamFilter, EndOfStream)
        HRESULT ( STDMETHODCALLTYPE *EndOfStream )( 
            IMediaStreamFilter * This);
        
        END_INTERFACE
    } IMediaStreamFilterVtbl;

    interface IMediaStreamFilter
    {
        CONST_VTBL struct IMediaStreamFilterVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMediaStreamFilter_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMediaStreamFilter_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMediaStreamFilter_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMediaStreamFilter_GetClassID(This,pClassID)	\
    ( (This)->lpVtbl -> GetClassID(This,pClassID) ) 


#define IMediaStreamFilter_Stop(This)	\
    ( (This)->lpVtbl -> Stop(This) ) 

#define IMediaStreamFilter_Pause(This)	\
    ( (This)->lpVtbl -> Pause(This) ) 

#define IMediaStreamFilter_Run(This,tStart)	\
    ( (This)->lpVtbl -> Run(This,tStart) ) 

#define IMediaStreamFilter_GetState(This,dwMilliSecsTimeout,State)	\
    ( (This)->lpVtbl -> GetState(This,dwMilliSecsTimeout,State) ) 

#define IMediaStreamFilter_SetSyncSource(This,pClock)	\
    ( (This)->lpVtbl -> SetSyncSource(This,pClock) ) 

#define IMediaStreamFilter_GetSyncSource(This,pClock)	\
    ( (This)->lpVtbl -> GetSyncSource(This,pClock) ) 


#define IMediaStreamFilter_EnumPins(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumPins(This,ppEnum) ) 

#define IMediaStreamFilter_FindPin(This,Id,ppPin)	\
    ( (This)->lpVtbl -> FindPin(This,Id,ppPin) ) 

#define IMediaStreamFilter_QueryFilterInfo(This,pInfo)	\
    ( (This)->lpVtbl -> QueryFilterInfo(This,pInfo) ) 

#define IMediaStreamFilter_JoinFilterGraph(This,pGraph,pName)	\
    ( (This)->lpVtbl -> JoinFilterGraph(This,pGraph,pName) ) 

#define IMediaStreamFilter_QueryVendorInfo(This,pVendorInfo)	\
    ( (This)->lpVtbl -> QueryVendorInfo(This,pVendorInfo) ) 


#define IMediaStreamFilter_AddMediaStream(This,pAMMediaStream)	\
    ( (This)->lpVtbl -> AddMediaStream(This,pAMMediaStream) ) 

#define IMediaStreamFilter_GetMediaStream(This,idPurpose,ppMediaStream)	\
    ( (This)->lpVtbl -> GetMediaStream(This,idPurpose,ppMediaStream) ) 

#define IMediaStreamFilter_EnumMediaStreams(This,Index,ppMediaStream)	\
    ( (This)->lpVtbl -> EnumMediaStreams(This,Index,ppMediaStream) ) 

#define IMediaStreamFilter_SupportSeeking(This,bRenderer)	\
    ( (This)->lpVtbl -> SupportSeeking(This,bRenderer) ) 

#define IMediaStreamFilter_ReferenceTimeToStreamTime(This,pTime)	\
    ( (This)->lpVtbl -> ReferenceTimeToStreamTime(This,pTime) ) 

#define IMediaStreamFilter_GetCurrentStreamTime(This,pCurrentStreamTime)	\
    ( (This)->lpVtbl -> GetCurrentStreamTime(This,pCurrentStreamTime) ) 

#define IMediaStreamFilter_WaitUntil(This,WaitStreamTime)	\
    ( (This)->lpVtbl -> WaitUntil(This,WaitStreamTime) ) 

#define IMediaStreamFilter_Flush(This,bCancelEOS)	\
    ( (This)->lpVtbl -> Flush(This,bCancelEOS) ) 

#define IMediaStreamFilter_EndOfStream(This)	\
    ( (This)->lpVtbl -> EndOfStream(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMediaStreamFilter_INTERFACE_DEFINED__ */


#ifndef __IDirectDrawMediaSampleAllocator_INTERFACE_DEFINED__
#define __IDirectDrawMediaSampleAllocator_INTERFACE_DEFINED__

/* interface IDirectDrawMediaSampleAllocator */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_IDirectDrawMediaSampleAllocator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("AB6B4AFC-F6E4-11d0-900D-00C04FD9189D")
    IDirectDrawMediaSampleAllocator : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetDirectDraw( 
            IDirectDraw **ppDirectDraw) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDirectDrawMediaSampleAllocatorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDirectDrawMediaSampleAllocator * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDirectDrawMediaSampleAllocator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDirectDrawMediaSampleAllocator * This);
        
        DECLSPEC_XFGVIRT(IDirectDrawMediaSampleAllocator, GetDirectDraw)
        HRESULT ( STDMETHODCALLTYPE *GetDirectDraw )( 
            IDirectDrawMediaSampleAllocator * This,
            IDirectDraw **ppDirectDraw);
        
        END_INTERFACE
    } IDirectDrawMediaSampleAllocatorVtbl;

    interface IDirectDrawMediaSampleAllocator
    {
        CONST_VTBL struct IDirectDrawMediaSampleAllocatorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDirectDrawMediaSampleAllocator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDirectDrawMediaSampleAllocator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDirectDrawMediaSampleAllocator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDirectDrawMediaSampleAllocator_GetDirectDraw(This,ppDirectDraw)	\
    ( (This)->lpVtbl -> GetDirectDraw(This,ppDirectDraw) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDirectDrawMediaSampleAllocator_INTERFACE_DEFINED__ */


#ifndef __IDirectDrawMediaSample_INTERFACE_DEFINED__
#define __IDirectDrawMediaSample_INTERFACE_DEFINED__

/* interface IDirectDrawMediaSample */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_IDirectDrawMediaSample;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("AB6B4AFE-F6E4-11d0-900D-00C04FD9189D")
    IDirectDrawMediaSample : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetSurfaceAndReleaseLock( 
            /* [out] */ IDirectDrawSurface **ppDirectDrawSurface,
            /* [out] */ RECT *pRect) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE LockMediaSamplePointer( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDirectDrawMediaSampleVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDirectDrawMediaSample * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDirectDrawMediaSample * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDirectDrawMediaSample * This);
        
        DECLSPEC_XFGVIRT(IDirectDrawMediaSample, GetSurfaceAndReleaseLock)
        HRESULT ( STDMETHODCALLTYPE *GetSurfaceAndReleaseLock )( 
            IDirectDrawMediaSample * This,
            /* [out] */ IDirectDrawSurface **ppDirectDrawSurface,
            /* [out] */ RECT *pRect);
        
        DECLSPEC_XFGVIRT(IDirectDrawMediaSample, LockMediaSamplePointer)
        HRESULT ( STDMETHODCALLTYPE *LockMediaSamplePointer )( 
            IDirectDrawMediaSample * This);
        
        END_INTERFACE
    } IDirectDrawMediaSampleVtbl;

    interface IDirectDrawMediaSample
    {
        CONST_VTBL struct IDirectDrawMediaSampleVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDirectDrawMediaSample_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDirectDrawMediaSample_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDirectDrawMediaSample_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDirectDrawMediaSample_GetSurfaceAndReleaseLock(This,ppDirectDrawSurface,pRect)	\
    ( (This)->lpVtbl -> GetSurfaceAndReleaseLock(This,ppDirectDrawSurface,pRect) ) 

#define IDirectDrawMediaSample_LockMediaSamplePointer(This)	\
    ( (This)->lpVtbl -> LockMediaSamplePointer(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDirectDrawMediaSample_INTERFACE_DEFINED__ */


#ifndef __IAMMediaTypeStream_INTERFACE_DEFINED__
#define __IAMMediaTypeStream_INTERFACE_DEFINED__

/* interface IAMMediaTypeStream */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_IAMMediaTypeStream;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("AB6B4AFA-F6E4-11d0-900D-00C04FD9189D")
    IAMMediaTypeStream : public IMediaStream
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetFormat( 
            /* [out] */ AM_MEDIA_TYPE *pMediaType,
            /* [in] */ DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetFormat( 
            /* [in] */ AM_MEDIA_TYPE *pMediaType,
            /* [in] */ DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateSample( 
            /* [in] */ long lSampleSize,
            /* [in] */ BYTE *pbBuffer,
            /* [in] */ DWORD dwFlags,
            /* [in] */ IUnknown *pUnkOuter,
            /* [out] */ IAMMediaTypeSample **ppAMMediaTypeSample) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStreamAllocatorRequirements( 
            /* [out] */ ALLOCATOR_PROPERTIES *pProps) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetStreamAllocatorRequirements( 
            /* [in] */ ALLOCATOR_PROPERTIES *pProps) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAMMediaTypeStreamVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAMMediaTypeStream * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAMMediaTypeStream * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAMMediaTypeStream * This);
        
        DECLSPEC_XFGVIRT(IMediaStream, GetMultiMediaStream)
        HRESULT ( STDMETHODCALLTYPE *GetMultiMediaStream )( 
            IAMMediaTypeStream * This,
            /* [out] */ IMultiMediaStream **ppMultiMediaStream);
        
        DECLSPEC_XFGVIRT(IMediaStream, GetInformation)
        HRESULT ( STDMETHODCALLTYPE *GetInformation )( 
            IAMMediaTypeStream * This,
            /* [out] */ MSPID *pPurposeId,
            /* [out] */ STREAM_TYPE *pType);
        
        DECLSPEC_XFGVIRT(IMediaStream, SetSameFormat)
        HRESULT ( STDMETHODCALLTYPE *SetSameFormat )( 
            IAMMediaTypeStream * This,
            /* [in] */ IMediaStream *pStreamThatHasDesiredFormat,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IMediaStream, AllocateSample)
        HRESULT ( STDMETHODCALLTYPE *AllocateSample )( 
            IAMMediaTypeStream * This,
            /* [in] */ DWORD dwFlags,
            /* [out] */ IStreamSample **ppSample);
        
        DECLSPEC_XFGVIRT(IMediaStream, CreateSharedSample)
        HRESULT ( STDMETHODCALLTYPE *CreateSharedSample )( 
            IAMMediaTypeStream * This,
            /* [in] */ IStreamSample *pExistingSample,
            /* [in] */ DWORD dwFlags,
            /* [out] */ IStreamSample **ppNewSample);
        
        DECLSPEC_XFGVIRT(IMediaStream, SendEndOfStream)
        HRESULT ( STDMETHODCALLTYPE *SendEndOfStream )( 
            IAMMediaTypeStream * This,
            DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IAMMediaTypeStream, GetFormat)
        HRESULT ( STDMETHODCALLTYPE *GetFormat )( 
            IAMMediaTypeStream * This,
            /* [out] */ AM_MEDIA_TYPE *pMediaType,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IAMMediaTypeStream, SetFormat)
        HRESULT ( STDMETHODCALLTYPE *SetFormat )( 
            IAMMediaTypeStream * This,
            /* [in] */ AM_MEDIA_TYPE *pMediaType,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IAMMediaTypeStream, CreateSample)
        HRESULT ( STDMETHODCALLTYPE *CreateSample )( 
            IAMMediaTypeStream * This,
            /* [in] */ long lSampleSize,
            /* [in] */ BYTE *pbBuffer,
            /* [in] */ DWORD dwFlags,
            /* [in] */ IUnknown *pUnkOuter,
            /* [out] */ IAMMediaTypeSample **ppAMMediaTypeSample);
        
        DECLSPEC_XFGVIRT(IAMMediaTypeStream, GetStreamAllocatorRequirements)
        HRESULT ( STDMETHODCALLTYPE *GetStreamAllocatorRequirements )( 
            IAMMediaTypeStream * This,
            /* [out] */ ALLOCATOR_PROPERTIES *pProps);
        
        DECLSPEC_XFGVIRT(IAMMediaTypeStream, SetStreamAllocatorRequirements)
        HRESULT ( STDMETHODCALLTYPE *SetStreamAllocatorRequirements )( 
            IAMMediaTypeStream * This,
            /* [in] */ ALLOCATOR_PROPERTIES *pProps);
        
        END_INTERFACE
    } IAMMediaTypeStreamVtbl;

    interface IAMMediaTypeStream
    {
        CONST_VTBL struct IAMMediaTypeStreamVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAMMediaTypeStream_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAMMediaTypeStream_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAMMediaTypeStream_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAMMediaTypeStream_GetMultiMediaStream(This,ppMultiMediaStream)	\
    ( (This)->lpVtbl -> GetMultiMediaStream(This,ppMultiMediaStream) ) 

#define IAMMediaTypeStream_GetInformation(This,pPurposeId,pType)	\
    ( (This)->lpVtbl -> GetInformation(This,pPurposeId,pType) ) 

#define IAMMediaTypeStream_SetSameFormat(This,pStreamThatHasDesiredFormat,dwFlags)	\
    ( (This)->lpVtbl -> SetSameFormat(This,pStreamThatHasDesiredFormat,dwFlags) ) 

#define IAMMediaTypeStream_AllocateSample(This,dwFlags,ppSample)	\
    ( (This)->lpVtbl -> AllocateSample(This,dwFlags,ppSample) ) 

#define IAMMediaTypeStream_CreateSharedSample(This,pExistingSample,dwFlags,ppNewSample)	\
    ( (This)->lpVtbl -> CreateSharedSample(This,pExistingSample,dwFlags,ppNewSample) ) 

#define IAMMediaTypeStream_SendEndOfStream(This,dwFlags)	\
    ( (This)->lpVtbl -> SendEndOfStream(This,dwFlags) ) 


#define IAMMediaTypeStream_GetFormat(This,pMediaType,dwFlags)	\
    ( (This)->lpVtbl -> GetFormat(This,pMediaType,dwFlags) ) 

#define IAMMediaTypeStream_SetFormat(This,pMediaType,dwFlags)	\
    ( (This)->lpVtbl -> SetFormat(This,pMediaType,dwFlags) ) 

#define IAMMediaTypeStream_CreateSample(This,lSampleSize,pbBuffer,dwFlags,pUnkOuter,ppAMMediaTypeSample)	\
    ( (This)->lpVtbl -> CreateSample(This,lSampleSize,pbBuffer,dwFlags,pUnkOuter,ppAMMediaTypeSample) ) 

#define IAMMediaTypeStream_GetStreamAllocatorRequirements(This,pProps)	\
    ( (This)->lpVtbl -> GetStreamAllocatorRequirements(This,pProps) ) 

#define IAMMediaTypeStream_SetStreamAllocatorRequirements(This,pProps)	\
    ( (This)->lpVtbl -> SetStreamAllocatorRequirements(This,pProps) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAMMediaTypeStream_INTERFACE_DEFINED__ */


#ifndef __IAMMediaTypeSample_INTERFACE_DEFINED__
#define __IAMMediaTypeSample_INTERFACE_DEFINED__

/* interface IAMMediaTypeSample */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_IAMMediaTypeSample;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("AB6B4AFB-F6E4-11d0-900D-00C04FD9189D")
    IAMMediaTypeSample : public IStreamSample
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetPointer( 
            /* [size_is][in] */ BYTE *pBuffer,
            /* [in] */ long lSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPointer( 
            /* [out] */ BYTE **ppBuffer) = 0;
        
        virtual long STDMETHODCALLTYPE GetSize( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTime( 
            /* [out] */ REFERENCE_TIME *pTimeStart,
            /* [out] */ REFERENCE_TIME *pTimeEnd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetTime( 
            /* [in] */ REFERENCE_TIME *pTimeStart,
            /* [in] */ REFERENCE_TIME *pTimeEnd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsSyncPoint( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSyncPoint( 
            BOOL bIsSyncPoint) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsPreroll( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetPreroll( 
            BOOL bIsPreroll) = 0;
        
        virtual long STDMETHODCALLTYPE GetActualDataLength( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetActualDataLength( 
            long __MIDL__IAMMediaTypeSample0000) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMediaType( 
            /* [out] */ AM_MEDIA_TYPE **ppMediaType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetMediaType( 
            /* [in] */ AM_MEDIA_TYPE *pMediaType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsDiscontinuity( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDiscontinuity( 
            BOOL bDiscontinuity) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMediaTime( 
            /* [out] */ LONGLONG *pTimeStart,
            /* [out] */ LONGLONG *pTimeEnd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetMediaTime( 
            /* [in] */ LONGLONG *pTimeStart,
            /* [in] */ LONGLONG *pTimeEnd) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAMMediaTypeSampleVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAMMediaTypeSample * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAMMediaTypeSample * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAMMediaTypeSample * This);
        
        DECLSPEC_XFGVIRT(IStreamSample, GetMediaStream)
        HRESULT ( STDMETHODCALLTYPE *GetMediaStream )( 
            IAMMediaTypeSample * This,
            /* [in] */ IMediaStream **ppMediaStream);
        
        DECLSPEC_XFGVIRT(IStreamSample, GetSampleTimes)
        HRESULT ( STDMETHODCALLTYPE *GetSampleTimes )( 
            IAMMediaTypeSample * This,
            /* [out] */ STREAM_TIME *pStartTime,
            /* [out] */ STREAM_TIME *pEndTime,
            /* [out] */ STREAM_TIME *pCurrentTime);
        
        DECLSPEC_XFGVIRT(IStreamSample, SetSampleTimes)
        HRESULT ( STDMETHODCALLTYPE *SetSampleTimes )( 
            IAMMediaTypeSample * This,
            /* [in] */ const STREAM_TIME *pStartTime,
            /* [in] */ const STREAM_TIME *pEndTime);
        
        DECLSPEC_XFGVIRT(IStreamSample, Update)
        HRESULT ( STDMETHODCALLTYPE *Update )( 
            IAMMediaTypeSample * This,
            /* [in] */ DWORD dwFlags,
            /* [in] */ HANDLE hEvent,
            /* [in] */ PAPCFUNC pfnAPC,
            /* [in] */ DWORD_PTR dwAPCData);
        
        DECLSPEC_XFGVIRT(IStreamSample, CompletionStatus)
        HRESULT ( STDMETHODCALLTYPE *CompletionStatus )( 
            IAMMediaTypeSample * This,
            /* [in] */ DWORD dwFlags,
            /* [in] */ DWORD dwMilliseconds);
        
        DECLSPEC_XFGVIRT(IAMMediaTypeSample, SetPointer)
        HRESULT ( STDMETHODCALLTYPE *SetPointer )( 
            IAMMediaTypeSample * This,
            /* [size_is][in] */ BYTE *pBuffer,
            /* [in] */ long lSize);
        
        DECLSPEC_XFGVIRT(IAMMediaTypeSample, GetPointer)
        HRESULT ( STDMETHODCALLTYPE *GetPointer )( 
            IAMMediaTypeSample * This,
            /* [out] */ BYTE **ppBuffer);
        
        DECLSPEC_XFGVIRT(IAMMediaTypeSample, GetSize)
        long ( STDMETHODCALLTYPE *GetSize )( 
            IAMMediaTypeSample * This);
        
        DECLSPEC_XFGVIRT(IAMMediaTypeSample, GetTime)
        HRESULT ( STDMETHODCALLTYPE *GetTime )( 
            IAMMediaTypeSample * This,
            /* [out] */ REFERENCE_TIME *pTimeStart,
            /* [out] */ REFERENCE_TIME *pTimeEnd);
        
        DECLSPEC_XFGVIRT(IAMMediaTypeSample, SetTime)
        HRESULT ( STDMETHODCALLTYPE *SetTime )( 
            IAMMediaTypeSample * This,
            /* [in] */ REFERENCE_TIME *pTimeStart,
            /* [in] */ REFERENCE_TIME *pTimeEnd);
        
        DECLSPEC_XFGVIRT(IAMMediaTypeSample, IsSyncPoint)
        HRESULT ( STDMETHODCALLTYPE *IsSyncPoint )( 
            IAMMediaTypeSample * This);
        
        DECLSPEC_XFGVIRT(IAMMediaTypeSample, SetSyncPoint)
        HRESULT ( STDMETHODCALLTYPE *SetSyncPoint )( 
            IAMMediaTypeSample * This,
            BOOL bIsSyncPoint);
        
        DECLSPEC_XFGVIRT(IAMMediaTypeSample, IsPreroll)
        HRESULT ( STDMETHODCALLTYPE *IsPreroll )( 
            IAMMediaTypeSample * This);
        
        DECLSPEC_XFGVIRT(IAMMediaTypeSample, SetPreroll)
        HRESULT ( STDMETHODCALLTYPE *SetPreroll )( 
            IAMMediaTypeSample * This,
            BOOL bIsPreroll);
        
        DECLSPEC_XFGVIRT(IAMMediaTypeSample, GetActualDataLength)
        long ( STDMETHODCALLTYPE *GetActualDataLength )( 
            IAMMediaTypeSample * This);
        
        DECLSPEC_XFGVIRT(IAMMediaTypeSample, SetActualDataLength)
        HRESULT ( STDMETHODCALLTYPE *SetActualDataLength )( 
            IAMMediaTypeSample * This,
            long __MIDL__IAMMediaTypeSample0000);
        
        DECLSPEC_XFGVIRT(IAMMediaTypeSample, GetMediaType)
        HRESULT ( STDMETHODCALLTYPE *GetMediaType )( 
            IAMMediaTypeSample * This,
            /* [out] */ AM_MEDIA_TYPE **ppMediaType);
        
        DECLSPEC_XFGVIRT(IAMMediaTypeSample, SetMediaType)
        HRESULT ( STDMETHODCALLTYPE *SetMediaType )( 
            IAMMediaTypeSample * This,
            /* [in] */ AM_MEDIA_TYPE *pMediaType);
        
        DECLSPEC_XFGVIRT(IAMMediaTypeSample, IsDiscontinuity)
        HRESULT ( STDMETHODCALLTYPE *IsDiscontinuity )( 
            IAMMediaTypeSample * This);
        
        DECLSPEC_XFGVIRT(IAMMediaTypeSample, SetDiscontinuity)
        HRESULT ( STDMETHODCALLTYPE *SetDiscontinuity )( 
            IAMMediaTypeSample * This,
            BOOL bDiscontinuity);
        
        DECLSPEC_XFGVIRT(IAMMediaTypeSample, GetMediaTime)
        HRESULT ( STDMETHODCALLTYPE *GetMediaTime )( 
            IAMMediaTypeSample * This,
            /* [out] */ LONGLONG *pTimeStart,
            /* [out] */ LONGLONG *pTimeEnd);
        
        DECLSPEC_XFGVIRT(IAMMediaTypeSample, SetMediaTime)
        HRESULT ( STDMETHODCALLTYPE *SetMediaTime )( 
            IAMMediaTypeSample * This,
            /* [in] */ LONGLONG *pTimeStart,
            /* [in] */ LONGLONG *pTimeEnd);
        
        END_INTERFACE
    } IAMMediaTypeSampleVtbl;

    interface IAMMediaTypeSample
    {
        CONST_VTBL struct IAMMediaTypeSampleVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAMMediaTypeSample_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAMMediaTypeSample_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAMMediaTypeSample_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAMMediaTypeSample_GetMediaStream(This,ppMediaStream)	\
    ( (This)->lpVtbl -> GetMediaStream(This,ppMediaStream) ) 

#define IAMMediaTypeSample_GetSampleTimes(This,pStartTime,pEndTime,pCurrentTime)	\
    ( (This)->lpVtbl -> GetSampleTimes(This,pStartTime,pEndTime,pCurrentTime) ) 

#define IAMMediaTypeSample_SetSampleTimes(This,pStartTime,pEndTime)	\
    ( (This)->lpVtbl -> SetSampleTimes(This,pStartTime,pEndTime) ) 

#define IAMMediaTypeSample_Update(This,dwFlags,hEvent,pfnAPC,dwAPCData)	\
    ( (This)->lpVtbl -> Update(This,dwFlags,hEvent,pfnAPC,dwAPCData) ) 

#define IAMMediaTypeSample_CompletionStatus(This,dwFlags,dwMilliseconds)	\
    ( (This)->lpVtbl -> CompletionStatus(This,dwFlags,dwMilliseconds) ) 


#define IAMMediaTypeSample_SetPointer(This,pBuffer,lSize)	\
    ( (This)->lpVtbl -> SetPointer(This,pBuffer,lSize) ) 

#define IAMMediaTypeSample_GetPointer(This,ppBuffer)	\
    ( (This)->lpVtbl -> GetPointer(This,ppBuffer) ) 

#define IAMMediaTypeSample_GetSize(This)	\
    ( (This)->lpVtbl -> GetSize(This) ) 

#define IAMMediaTypeSample_GetTime(This,pTimeStart,pTimeEnd)	\
    ( (This)->lpVtbl -> GetTime(This,pTimeStart,pTimeEnd) ) 

#define IAMMediaTypeSample_SetTime(This,pTimeStart,pTimeEnd)	\
    ( (This)->lpVtbl -> SetTime(This,pTimeStart,pTimeEnd) ) 

#define IAMMediaTypeSample_IsSyncPoint(This)	\
    ( (This)->lpVtbl -> IsSyncPoint(This) ) 

#define IAMMediaTypeSample_SetSyncPoint(This,bIsSyncPoint)	\
    ( (This)->lpVtbl -> SetSyncPoint(This,bIsSyncPoint) ) 

#define IAMMediaTypeSample_IsPreroll(This)	\
    ( (This)->lpVtbl -> IsPreroll(This) ) 

#define IAMMediaTypeSample_SetPreroll(This,bIsPreroll)	\
    ( (This)->lpVtbl -> SetPreroll(This,bIsPreroll) ) 

#define IAMMediaTypeSample_GetActualDataLength(This)	\
    ( (This)->lpVtbl -> GetActualDataLength(This) ) 

#define IAMMediaTypeSample_SetActualDataLength(This,__MIDL__IAMMediaTypeSample0000)	\
    ( (This)->lpVtbl -> SetActualDataLength(This,__MIDL__IAMMediaTypeSample0000) ) 

#define IAMMediaTypeSample_GetMediaType(This,ppMediaType)	\
    ( (This)->lpVtbl -> GetMediaType(This,ppMediaType) ) 

#define IAMMediaTypeSample_SetMediaType(This,pMediaType)	\
    ( (This)->lpVtbl -> SetMediaType(This,pMediaType) ) 

#define IAMMediaTypeSample_IsDiscontinuity(This)	\
    ( (This)->lpVtbl -> IsDiscontinuity(This) ) 

#define IAMMediaTypeSample_SetDiscontinuity(This,bDiscontinuity)	\
    ( (This)->lpVtbl -> SetDiscontinuity(This,bDiscontinuity) ) 

#define IAMMediaTypeSample_GetMediaTime(This,pTimeStart,pTimeEnd)	\
    ( (This)->lpVtbl -> GetMediaTime(This,pTimeStart,pTimeEnd) ) 

#define IAMMediaTypeSample_SetMediaTime(This,pTimeStart,pTimeEnd)	\
    ( (This)->lpVtbl -> SetMediaTime(This,pTimeStart,pTimeEnd) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAMMediaTypeSample_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_amstream_0000_0007 */
/* [local] */ 

DEFINE_GUID(CLSID_AMMultiMediaStream, /* 49c47ce5-9ba4-11d0-8212-00c04fc32c45 */
0x49c47ce5, 0x9ba4, 0x11d0, 0x82, 0x12, 0x00, 0xc0, 0x4f, 0xc3, 0x2c, 0x45);
DEFINE_GUID(CLSID_AMDirectDrawStream, /* 49c47ce4-9ba4-11d0-8212-00c04fc32c45 */
0x49c47ce4, 0x9ba4, 0x11d0, 0x82, 0x12, 0x00, 0xc0, 0x4f, 0xc3, 0x2c, 0x45);
DEFINE_GUID(CLSID_AMAudioStream, /* 8496e040-af4c-11d0-8212-00c04fc32c45 */
0x8496e040, 0xaf4c, 0x11d0, 0x82, 0x12, 0x00, 0xc0, 0x4f, 0xc3, 0x2c, 0x45);
DEFINE_GUID(CLSID_AMAudioData, /* f2468580-af8a-11d0-8212-00c04fc32c45 */
0xf2468580, 0xaf8a, 0x11d0, 0x82, 0x12, 0x00, 0xc0, 0x4f, 0xc3, 0x2c, 0x45);
DEFINE_GUID(CLSID_AMMediaTypeStream, /* CF0F2F7C-F7BF-11d0-900D-00C04FD9189D */
0xcf0f2f7c, 0xf7bf, 0x11d0, 0x90, 0xd, 0x0, 0xc0, 0x4f, 0xd9, 0x18, 0x9d);
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_amstream_0000_0007_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_amstream_0000_0007_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif



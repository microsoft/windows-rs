

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

#ifndef __mediaobj_h__
#define __mediaobj_h__

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

#ifndef __IMediaBuffer_FWD_DEFINED__
#define __IMediaBuffer_FWD_DEFINED__
typedef interface IMediaBuffer IMediaBuffer;

#endif 	/* __IMediaBuffer_FWD_DEFINED__ */


#ifndef __IMediaObject_FWD_DEFINED__
#define __IMediaObject_FWD_DEFINED__
typedef interface IMediaObject IMediaObject;

#endif 	/* __IMediaObject_FWD_DEFINED__ */


#ifndef __IEnumDMO_FWD_DEFINED__
#define __IEnumDMO_FWD_DEFINED__
typedef interface IEnumDMO IEnumDMO;

#endif 	/* __IEnumDMO_FWD_DEFINED__ */


#ifndef __IMediaObjectInPlace_FWD_DEFINED__
#define __IMediaObjectInPlace_FWD_DEFINED__
typedef interface IMediaObjectInPlace IMediaObjectInPlace;

#endif 	/* __IMediaObjectInPlace_FWD_DEFINED__ */


#ifndef __IDMOQualityControl_FWD_DEFINED__
#define __IDMOQualityControl_FWD_DEFINED__
typedef interface IDMOQualityControl IDMOQualityControl;

#endif 	/* __IDMOQualityControl_FWD_DEFINED__ */


#ifndef __IDMOVideoOutputOptimizations_FWD_DEFINED__
#define __IDMOVideoOutputOptimizations_FWD_DEFINED__
typedef interface IDMOVideoOutputOptimizations IDMOVideoOutputOptimizations;

#endif 	/* __IDMOVideoOutputOptimizations_FWD_DEFINED__ */


/* header files for imported files */
#include "unknwn.h"
#include "objidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_mediaobj_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#ifdef __strmif_h__
typedef AM_MEDIA_TYPE DMO_MEDIA_TYPE;
#else
typedef struct _DMOMediaType
    {
    GUID majortype;
    GUID subtype;
    BOOL bFixedSizeSamples;
    BOOL bTemporalCompression;
    ULONG lSampleSize;
    GUID formattype;
    IUnknown *pUnk;
    ULONG cbFormat;
    /* [size_is] */ BYTE *pbFormat;
    } 	DMO_MEDIA_TYPE;

typedef LONGLONG REFERENCE_TIME;

#endif

enum _DMO_INPUT_DATA_BUFFER_FLAGS
    {
        DMO_INPUT_DATA_BUFFERF_SYNCPOINT	= 0x1,
        DMO_INPUT_DATA_BUFFERF_TIME	= 0x2,
        DMO_INPUT_DATA_BUFFERF_TIMELENGTH	= 0x4,
        DMO_INPUT_DATA_BUFFERF_DISCONTINUITY	= 0x8
    } ;

enum _DMO_OUTPUT_DATA_BUFFER_FLAGS
    {
        DMO_OUTPUT_DATA_BUFFERF_SYNCPOINT	= 0x1,
        DMO_OUTPUT_DATA_BUFFERF_TIME	= 0x2,
        DMO_OUTPUT_DATA_BUFFERF_TIMELENGTH	= 0x4,
        DMO_OUTPUT_DATA_BUFFERF_DISCONTINUITY	= 0x8,
        DMO_OUTPUT_DATA_BUFFERF_INCOMPLETE	= 0x1000000
    } ;

enum _DMO_INPUT_STATUS_FLAGS
    {
        DMO_INPUT_STATUSF_ACCEPT_DATA	= 0x1
    } ;

enum _DMO_INPUT_STREAM_INFO_FLAGS
    {
        DMO_INPUT_STREAMF_WHOLE_SAMPLES	= 0x1,
        DMO_INPUT_STREAMF_SINGLE_SAMPLE_PER_BUFFER	= 0x2,
        DMO_INPUT_STREAMF_FIXED_SAMPLE_SIZE	= 0x4,
        DMO_INPUT_STREAMF_HOLDS_BUFFERS	= 0x8
    } ;

enum _DMO_OUTPUT_STREAM_INFO_FLAGS
    {
        DMO_OUTPUT_STREAMF_WHOLE_SAMPLES	= 0x1,
        DMO_OUTPUT_STREAMF_SINGLE_SAMPLE_PER_BUFFER	= 0x2,
        DMO_OUTPUT_STREAMF_FIXED_SAMPLE_SIZE	= 0x4,
        DMO_OUTPUT_STREAMF_DISCARDABLE	= 0x8,
        DMO_OUTPUT_STREAMF_OPTIONAL	= 0x10
    } ;

enum _DMO_SET_TYPE_FLAGS
    {
        DMO_SET_TYPEF_TEST_ONLY	= 0x1,
        DMO_SET_TYPEF_CLEAR	= 0x2
    } ;

enum _DMO_PROCESS_OUTPUT_FLAGS
    {
        DMO_PROCESS_OUTPUT_DISCARD_WHEN_NO_BUFFER	= 0x1
    } ;
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_mediaobj_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mediaobj_0000_0000_v0_0_s_ifspec;

#ifndef __IMediaBuffer_INTERFACE_DEFINED__
#define __IMediaBuffer_INTERFACE_DEFINED__

/* interface IMediaBuffer */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMediaBuffer;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("59eff8b9-938c-4a26-82f2-95cb84cdc837")
    IMediaBuffer : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetLength( 
            DWORD cbLength) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMaxLength( 
            /* [annotation][out] */ 
            _Out_  DWORD *pcbMaxLength) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetBufferAndLength( 
            /* [annotation][out] */ 
            _Outptr_opt_result_bytebuffer_(*pcbLength)  BYTE **ppBuffer,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *pcbLength) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMediaBufferVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMediaBuffer * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMediaBuffer * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMediaBuffer * This);
        
        DECLSPEC_XFGVIRT(IMediaBuffer, SetLength)
        HRESULT ( STDMETHODCALLTYPE *SetLength )( 
            IMediaBuffer * This,
            DWORD cbLength);
        
        DECLSPEC_XFGVIRT(IMediaBuffer, GetMaxLength)
        HRESULT ( STDMETHODCALLTYPE *GetMaxLength )( 
            IMediaBuffer * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pcbMaxLength);
        
        DECLSPEC_XFGVIRT(IMediaBuffer, GetBufferAndLength)
        HRESULT ( STDMETHODCALLTYPE *GetBufferAndLength )( 
            IMediaBuffer * This,
            /* [annotation][out] */ 
            _Outptr_opt_result_bytebuffer_(*pcbLength)  BYTE **ppBuffer,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *pcbLength);
        
        END_INTERFACE
    } IMediaBufferVtbl;

    interface IMediaBuffer
    {
        CONST_VTBL struct IMediaBufferVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMediaBuffer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMediaBuffer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMediaBuffer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMediaBuffer_SetLength(This,cbLength)	\
    ( (This)->lpVtbl -> SetLength(This,cbLength) ) 

#define IMediaBuffer_GetMaxLength(This,pcbMaxLength)	\
    ( (This)->lpVtbl -> GetMaxLength(This,pcbMaxLength) ) 

#define IMediaBuffer_GetBufferAndLength(This,ppBuffer,pcbLength)	\
    ( (This)->lpVtbl -> GetBufferAndLength(This,ppBuffer,pcbLength) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMediaBuffer_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mediaobj_0000_0001 */
/* [local] */ 

typedef struct _DMO_OUTPUT_DATA_BUFFER
    {
    IMediaBuffer *pBuffer;
    DWORD dwStatus;
    REFERENCE_TIME rtTimestamp;
    REFERENCE_TIME rtTimelength;
    } 	DMO_OUTPUT_DATA_BUFFER;

typedef struct _DMO_OUTPUT_DATA_BUFFER *PDMO_OUTPUT_DATA_BUFFER;



extern RPC_IF_HANDLE __MIDL_itf_mediaobj_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mediaobj_0000_0001_v0_0_s_ifspec;

#ifndef __IMediaObject_INTERFACE_DEFINED__
#define __IMediaObject_INTERFACE_DEFINED__

/* interface IMediaObject */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMediaObject;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("d8ad0f58-5494-4102-97c5-ec798e59bcf4")
    IMediaObject : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetStreamCount( 
            /* [annotation][out] */ 
            _Out_  DWORD *pcInputStreams,
            /* [annotation][out] */ 
            _Out_  DWORD *pcOutputStreams) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetInputStreamInfo( 
            DWORD dwInputStreamIndex,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOutputStreamInfo( 
            DWORD dwOutputStreamIndex,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetInputType( 
            DWORD dwInputStreamIndex,
            DWORD dwTypeIndex,
            /* [annotation][out] */ 
            _Out_opt_  DMO_MEDIA_TYPE *pmt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOutputType( 
            DWORD dwOutputStreamIndex,
            DWORD dwTypeIndex,
            /* [annotation][out] */ 
            _Out_opt_  DMO_MEDIA_TYPE *pmt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetInputType( 
            DWORD dwInputStreamIndex,
            /* [annotation][in] */ 
            _In_opt_  const DMO_MEDIA_TYPE *pmt,
            DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetOutputType( 
            DWORD dwOutputStreamIndex,
            /* [annotation][in] */ 
            _In_opt_  const DMO_MEDIA_TYPE *pmt,
            DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetInputCurrentType( 
            DWORD dwInputStreamIndex,
            /* [annotation][out] */ 
            _Out_  DMO_MEDIA_TYPE *pmt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOutputCurrentType( 
            DWORD dwOutputStreamIndex,
            /* [annotation][out] */ 
            _Out_  DMO_MEDIA_TYPE *pmt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetInputSizeInfo( 
            DWORD dwInputStreamIndex,
            /* [annotation][out] */ 
            _Out_  DWORD *pcbSize,
            /* [annotation][out] */ 
            _Out_  DWORD *pcbMaxLookahead,
            /* [annotation][out] */ 
            _Out_  DWORD *pcbAlignment) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOutputSizeInfo( 
            DWORD dwOutputStreamIndex,
            /* [annotation][out] */ 
            _Out_  DWORD *pcbSize,
            /* [annotation][out] */ 
            _Out_  DWORD *pcbAlignment) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetInputMaxLatency( 
            DWORD dwInputStreamIndex,
            /* [annotation][out] */ 
            _Out_  REFERENCE_TIME *prtMaxLatency) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetInputMaxLatency( 
            DWORD dwInputStreamIndex,
            REFERENCE_TIME rtMaxLatency) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Flush( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Discontinuity( 
            DWORD dwInputStreamIndex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AllocateStreamingResources( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FreeStreamingResources( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetInputStatus( 
            DWORD dwInputStreamIndex,
            /* [annotation][out] */ 
            _Out_  DWORD *dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ProcessInput( 
            DWORD dwInputStreamIndex,
            IMediaBuffer *pBuffer,
            DWORD dwFlags,
            REFERENCE_TIME rtTimestamp,
            REFERENCE_TIME rtTimelength) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ProcessOutput( 
            DWORD dwFlags,
            DWORD cOutputBufferCount,
            /* [annotation][size_is][out][in] */ 
            _Out_writes_(cOutputBufferCount)  DMO_OUTPUT_DATA_BUFFER *pOutputBuffers,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwStatus) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Lock( 
            LONG bLock) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMediaObjectVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMediaObject * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMediaObject * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMediaObject * This);
        
        DECLSPEC_XFGVIRT(IMediaObject, GetStreamCount)
        HRESULT ( STDMETHODCALLTYPE *GetStreamCount )( 
            IMediaObject * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pcInputStreams,
            /* [annotation][out] */ 
            _Out_  DWORD *pcOutputStreams);
        
        DECLSPEC_XFGVIRT(IMediaObject, GetInputStreamInfo)
        HRESULT ( STDMETHODCALLTYPE *GetInputStreamInfo )( 
            IMediaObject * This,
            DWORD dwInputStreamIndex,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwFlags);
        
        DECLSPEC_XFGVIRT(IMediaObject, GetOutputStreamInfo)
        HRESULT ( STDMETHODCALLTYPE *GetOutputStreamInfo )( 
            IMediaObject * This,
            DWORD dwOutputStreamIndex,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwFlags);
        
        DECLSPEC_XFGVIRT(IMediaObject, GetInputType)
        HRESULT ( STDMETHODCALLTYPE *GetInputType )( 
            IMediaObject * This,
            DWORD dwInputStreamIndex,
            DWORD dwTypeIndex,
            /* [annotation][out] */ 
            _Out_opt_  DMO_MEDIA_TYPE *pmt);
        
        DECLSPEC_XFGVIRT(IMediaObject, GetOutputType)
        HRESULT ( STDMETHODCALLTYPE *GetOutputType )( 
            IMediaObject * This,
            DWORD dwOutputStreamIndex,
            DWORD dwTypeIndex,
            /* [annotation][out] */ 
            _Out_opt_  DMO_MEDIA_TYPE *pmt);
        
        DECLSPEC_XFGVIRT(IMediaObject, SetInputType)
        HRESULT ( STDMETHODCALLTYPE *SetInputType )( 
            IMediaObject * This,
            DWORD dwInputStreamIndex,
            /* [annotation][in] */ 
            _In_opt_  const DMO_MEDIA_TYPE *pmt,
            DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IMediaObject, SetOutputType)
        HRESULT ( STDMETHODCALLTYPE *SetOutputType )( 
            IMediaObject * This,
            DWORD dwOutputStreamIndex,
            /* [annotation][in] */ 
            _In_opt_  const DMO_MEDIA_TYPE *pmt,
            DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IMediaObject, GetInputCurrentType)
        HRESULT ( STDMETHODCALLTYPE *GetInputCurrentType )( 
            IMediaObject * This,
            DWORD dwInputStreamIndex,
            /* [annotation][out] */ 
            _Out_  DMO_MEDIA_TYPE *pmt);
        
        DECLSPEC_XFGVIRT(IMediaObject, GetOutputCurrentType)
        HRESULT ( STDMETHODCALLTYPE *GetOutputCurrentType )( 
            IMediaObject * This,
            DWORD dwOutputStreamIndex,
            /* [annotation][out] */ 
            _Out_  DMO_MEDIA_TYPE *pmt);
        
        DECLSPEC_XFGVIRT(IMediaObject, GetInputSizeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetInputSizeInfo )( 
            IMediaObject * This,
            DWORD dwInputStreamIndex,
            /* [annotation][out] */ 
            _Out_  DWORD *pcbSize,
            /* [annotation][out] */ 
            _Out_  DWORD *pcbMaxLookahead,
            /* [annotation][out] */ 
            _Out_  DWORD *pcbAlignment);
        
        DECLSPEC_XFGVIRT(IMediaObject, GetOutputSizeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetOutputSizeInfo )( 
            IMediaObject * This,
            DWORD dwOutputStreamIndex,
            /* [annotation][out] */ 
            _Out_  DWORD *pcbSize,
            /* [annotation][out] */ 
            _Out_  DWORD *pcbAlignment);
        
        DECLSPEC_XFGVIRT(IMediaObject, GetInputMaxLatency)
        HRESULT ( STDMETHODCALLTYPE *GetInputMaxLatency )( 
            IMediaObject * This,
            DWORD dwInputStreamIndex,
            /* [annotation][out] */ 
            _Out_  REFERENCE_TIME *prtMaxLatency);
        
        DECLSPEC_XFGVIRT(IMediaObject, SetInputMaxLatency)
        HRESULT ( STDMETHODCALLTYPE *SetInputMaxLatency )( 
            IMediaObject * This,
            DWORD dwInputStreamIndex,
            REFERENCE_TIME rtMaxLatency);
        
        DECLSPEC_XFGVIRT(IMediaObject, Flush)
        HRESULT ( STDMETHODCALLTYPE *Flush )( 
            IMediaObject * This);
        
        DECLSPEC_XFGVIRT(IMediaObject, Discontinuity)
        HRESULT ( STDMETHODCALLTYPE *Discontinuity )( 
            IMediaObject * This,
            DWORD dwInputStreamIndex);
        
        DECLSPEC_XFGVIRT(IMediaObject, AllocateStreamingResources)
        HRESULT ( STDMETHODCALLTYPE *AllocateStreamingResources )( 
            IMediaObject * This);
        
        DECLSPEC_XFGVIRT(IMediaObject, FreeStreamingResources)
        HRESULT ( STDMETHODCALLTYPE *FreeStreamingResources )( 
            IMediaObject * This);
        
        DECLSPEC_XFGVIRT(IMediaObject, GetInputStatus)
        HRESULT ( STDMETHODCALLTYPE *GetInputStatus )( 
            IMediaObject * This,
            DWORD dwInputStreamIndex,
            /* [annotation][out] */ 
            _Out_  DWORD *dwFlags);
        
        DECLSPEC_XFGVIRT(IMediaObject, ProcessInput)
        HRESULT ( STDMETHODCALLTYPE *ProcessInput )( 
            IMediaObject * This,
            DWORD dwInputStreamIndex,
            IMediaBuffer *pBuffer,
            DWORD dwFlags,
            REFERENCE_TIME rtTimestamp,
            REFERENCE_TIME rtTimelength);
        
        DECLSPEC_XFGVIRT(IMediaObject, ProcessOutput)
        HRESULT ( STDMETHODCALLTYPE *ProcessOutput )( 
            IMediaObject * This,
            DWORD dwFlags,
            DWORD cOutputBufferCount,
            /* [annotation][size_is][out][in] */ 
            _Out_writes_(cOutputBufferCount)  DMO_OUTPUT_DATA_BUFFER *pOutputBuffers,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwStatus);
        
        DECLSPEC_XFGVIRT(IMediaObject, Lock)
        HRESULT ( STDMETHODCALLTYPE *Lock )( 
            IMediaObject * This,
            LONG bLock);
        
        END_INTERFACE
    } IMediaObjectVtbl;

    interface IMediaObject
    {
        CONST_VTBL struct IMediaObjectVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMediaObject_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMediaObject_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMediaObject_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMediaObject_GetStreamCount(This,pcInputStreams,pcOutputStreams)	\
    ( (This)->lpVtbl -> GetStreamCount(This,pcInputStreams,pcOutputStreams) ) 

#define IMediaObject_GetInputStreamInfo(This,dwInputStreamIndex,pdwFlags)	\
    ( (This)->lpVtbl -> GetInputStreamInfo(This,dwInputStreamIndex,pdwFlags) ) 

#define IMediaObject_GetOutputStreamInfo(This,dwOutputStreamIndex,pdwFlags)	\
    ( (This)->lpVtbl -> GetOutputStreamInfo(This,dwOutputStreamIndex,pdwFlags) ) 

#define IMediaObject_GetInputType(This,dwInputStreamIndex,dwTypeIndex,pmt)	\
    ( (This)->lpVtbl -> GetInputType(This,dwInputStreamIndex,dwTypeIndex,pmt) ) 

#define IMediaObject_GetOutputType(This,dwOutputStreamIndex,dwTypeIndex,pmt)	\
    ( (This)->lpVtbl -> GetOutputType(This,dwOutputStreamIndex,dwTypeIndex,pmt) ) 

#define IMediaObject_SetInputType(This,dwInputStreamIndex,pmt,dwFlags)	\
    ( (This)->lpVtbl -> SetInputType(This,dwInputStreamIndex,pmt,dwFlags) ) 

#define IMediaObject_SetOutputType(This,dwOutputStreamIndex,pmt,dwFlags)	\
    ( (This)->lpVtbl -> SetOutputType(This,dwOutputStreamIndex,pmt,dwFlags) ) 

#define IMediaObject_GetInputCurrentType(This,dwInputStreamIndex,pmt)	\
    ( (This)->lpVtbl -> GetInputCurrentType(This,dwInputStreamIndex,pmt) ) 

#define IMediaObject_GetOutputCurrentType(This,dwOutputStreamIndex,pmt)	\
    ( (This)->lpVtbl -> GetOutputCurrentType(This,dwOutputStreamIndex,pmt) ) 

#define IMediaObject_GetInputSizeInfo(This,dwInputStreamIndex,pcbSize,pcbMaxLookahead,pcbAlignment)	\
    ( (This)->lpVtbl -> GetInputSizeInfo(This,dwInputStreamIndex,pcbSize,pcbMaxLookahead,pcbAlignment) ) 

#define IMediaObject_GetOutputSizeInfo(This,dwOutputStreamIndex,pcbSize,pcbAlignment)	\
    ( (This)->lpVtbl -> GetOutputSizeInfo(This,dwOutputStreamIndex,pcbSize,pcbAlignment) ) 

#define IMediaObject_GetInputMaxLatency(This,dwInputStreamIndex,prtMaxLatency)	\
    ( (This)->lpVtbl -> GetInputMaxLatency(This,dwInputStreamIndex,prtMaxLatency) ) 

#define IMediaObject_SetInputMaxLatency(This,dwInputStreamIndex,rtMaxLatency)	\
    ( (This)->lpVtbl -> SetInputMaxLatency(This,dwInputStreamIndex,rtMaxLatency) ) 

#define IMediaObject_Flush(This)	\
    ( (This)->lpVtbl -> Flush(This) ) 

#define IMediaObject_Discontinuity(This,dwInputStreamIndex)	\
    ( (This)->lpVtbl -> Discontinuity(This,dwInputStreamIndex) ) 

#define IMediaObject_AllocateStreamingResources(This)	\
    ( (This)->lpVtbl -> AllocateStreamingResources(This) ) 

#define IMediaObject_FreeStreamingResources(This)	\
    ( (This)->lpVtbl -> FreeStreamingResources(This) ) 

#define IMediaObject_GetInputStatus(This,dwInputStreamIndex,dwFlags)	\
    ( (This)->lpVtbl -> GetInputStatus(This,dwInputStreamIndex,dwFlags) ) 

#define IMediaObject_ProcessInput(This,dwInputStreamIndex,pBuffer,dwFlags,rtTimestamp,rtTimelength)	\
    ( (This)->lpVtbl -> ProcessInput(This,dwInputStreamIndex,pBuffer,dwFlags,rtTimestamp,rtTimelength) ) 

#define IMediaObject_ProcessOutput(This,dwFlags,cOutputBufferCount,pOutputBuffers,pdwStatus)	\
    ( (This)->lpVtbl -> ProcessOutput(This,dwFlags,cOutputBufferCount,pOutputBuffers,pdwStatus) ) 

#define IMediaObject_Lock(This,bLock)	\
    ( (This)->lpVtbl -> Lock(This,bLock) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMediaObject_INTERFACE_DEFINED__ */


#ifndef __IEnumDMO_INTERFACE_DEFINED__
#define __IEnumDMO_INTERFACE_DEFINED__

/* interface IEnumDMO */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IEnumDMO;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2c3cd98a-2bfa-4a53-9c27-5249ba64ba0f")
    IEnumDMO : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            DWORD cItemsToFetch,
            /* [annotation][length_is][size_is][out] */ 
            _Out_writes_to_(cItemsToFetch, *pcItemsFetched)  CLSID *pCLSID,
            /* [annotation][string][length_is][size_is][out] */ 
            _Out_writes_to_(cItemsToFetch, *pcItemsFetched)  LPWSTR *Names,
            /* [annotation][out] */ 
            _Out_  DWORD *pcItemsFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            DWORD cItemsToSkip) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [annotation][out] */ 
            _Out_  IEnumDMO **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumDMOVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IEnumDMO * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IEnumDMO * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IEnumDMO * This);
        
        DECLSPEC_XFGVIRT(IEnumDMO, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            IEnumDMO * This,
            DWORD cItemsToFetch,
            /* [annotation][length_is][size_is][out] */ 
            _Out_writes_to_(cItemsToFetch, *pcItemsFetched)  CLSID *pCLSID,
            /* [annotation][string][length_is][size_is][out] */ 
            _Out_writes_to_(cItemsToFetch, *pcItemsFetched)  LPWSTR *Names,
            /* [annotation][out] */ 
            _Out_  DWORD *pcItemsFetched);
        
        DECLSPEC_XFGVIRT(IEnumDMO, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            IEnumDMO * This,
            DWORD cItemsToSkip);
        
        DECLSPEC_XFGVIRT(IEnumDMO, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            IEnumDMO * This);
        
        DECLSPEC_XFGVIRT(IEnumDMO, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            IEnumDMO * This,
            /* [annotation][out] */ 
            _Out_  IEnumDMO **ppEnum);
        
        END_INTERFACE
    } IEnumDMOVtbl;

    interface IEnumDMO
    {
        CONST_VTBL struct IEnumDMOVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumDMO_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumDMO_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumDMO_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumDMO_Next(This,cItemsToFetch,pCLSID,Names,pcItemsFetched)	\
    ( (This)->lpVtbl -> Next(This,cItemsToFetch,pCLSID,Names,pcItemsFetched) ) 

#define IEnumDMO_Skip(This,cItemsToSkip)	\
    ( (This)->lpVtbl -> Skip(This,cItemsToSkip) ) 

#define IEnumDMO_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumDMO_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumDMO_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mediaobj_0000_0003 */
/* [local] */ 


enum _DMO_INPLACE_PROCESS_FLAGS
    {
        DMO_INPLACE_NORMAL	= 0,
        DMO_INPLACE_ZERO	= 0x1
    } ;


extern RPC_IF_HANDLE __MIDL_itf_mediaobj_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mediaobj_0000_0003_v0_0_s_ifspec;

#ifndef __IMediaObjectInPlace_INTERFACE_DEFINED__
#define __IMediaObjectInPlace_INTERFACE_DEFINED__

/* interface IMediaObjectInPlace */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMediaObjectInPlace;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("651b9ad0-0fc7-4aa9-9538-d89931010741")
    IMediaObjectInPlace : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Process( 
            /* [in] */ ULONG ulSize,
            /* [annotation][size_is][out][in] */ 
            _Out_writes_bytes_(ulSize)  BYTE *pData,
            /* [in] */ REFERENCE_TIME refTimeStart,
            /* [in] */ DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [annotation][out] */ 
            _Out_  IMediaObjectInPlace **ppMediaObject) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLatency( 
            /* [annotation][out] */ 
            _Out_  REFERENCE_TIME *pLatencyTime) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMediaObjectInPlaceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMediaObjectInPlace * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMediaObjectInPlace * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMediaObjectInPlace * This);
        
        DECLSPEC_XFGVIRT(IMediaObjectInPlace, Process)
        HRESULT ( STDMETHODCALLTYPE *Process )( 
            IMediaObjectInPlace * This,
            /* [in] */ ULONG ulSize,
            /* [annotation][size_is][out][in] */ 
            _Out_writes_bytes_(ulSize)  BYTE *pData,
            /* [in] */ REFERENCE_TIME refTimeStart,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IMediaObjectInPlace, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            IMediaObjectInPlace * This,
            /* [annotation][out] */ 
            _Out_  IMediaObjectInPlace **ppMediaObject);
        
        DECLSPEC_XFGVIRT(IMediaObjectInPlace, GetLatency)
        HRESULT ( STDMETHODCALLTYPE *GetLatency )( 
            IMediaObjectInPlace * This,
            /* [annotation][out] */ 
            _Out_  REFERENCE_TIME *pLatencyTime);
        
        END_INTERFACE
    } IMediaObjectInPlaceVtbl;

    interface IMediaObjectInPlace
    {
        CONST_VTBL struct IMediaObjectInPlaceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMediaObjectInPlace_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMediaObjectInPlace_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMediaObjectInPlace_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMediaObjectInPlace_Process(This,ulSize,pData,refTimeStart,dwFlags)	\
    ( (This)->lpVtbl -> Process(This,ulSize,pData,refTimeStart,dwFlags) ) 

#define IMediaObjectInPlace_Clone(This,ppMediaObject)	\
    ( (This)->lpVtbl -> Clone(This,ppMediaObject) ) 

#define IMediaObjectInPlace_GetLatency(This,pLatencyTime)	\
    ( (This)->lpVtbl -> GetLatency(This,pLatencyTime) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMediaObjectInPlace_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mediaobj_0000_0004 */
/* [local] */ 


enum _DMO_QUALITY_STATUS_FLAGS
    {
        DMO_QUALITY_STATUS_ENABLED	= 0x1
    } ;


extern RPC_IF_HANDLE __MIDL_itf_mediaobj_0000_0004_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mediaobj_0000_0004_v0_0_s_ifspec;

#ifndef __IDMOQualityControl_INTERFACE_DEFINED__
#define __IDMOQualityControl_INTERFACE_DEFINED__

/* interface IDMOQualityControl */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IDMOQualityControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("65abea96-cf36-453f-af8a-705e98f16260")
    IDMOQualityControl : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetNow( 
            /* [in] */ REFERENCE_TIME rtNow) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetStatus( 
            /* [in] */ DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStatus( 
            /* [annotation][out] */ 
            _Out_  DWORD *pdwFlags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDMOQualityControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDMOQualityControl * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDMOQualityControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDMOQualityControl * This);
        
        DECLSPEC_XFGVIRT(IDMOQualityControl, SetNow)
        HRESULT ( STDMETHODCALLTYPE *SetNow )( 
            IDMOQualityControl * This,
            /* [in] */ REFERENCE_TIME rtNow);
        
        DECLSPEC_XFGVIRT(IDMOQualityControl, SetStatus)
        HRESULT ( STDMETHODCALLTYPE *SetStatus )( 
            IDMOQualityControl * This,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IDMOQualityControl, GetStatus)
        HRESULT ( STDMETHODCALLTYPE *GetStatus )( 
            IDMOQualityControl * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwFlags);
        
        END_INTERFACE
    } IDMOQualityControlVtbl;

    interface IDMOQualityControl
    {
        CONST_VTBL struct IDMOQualityControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDMOQualityControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDMOQualityControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDMOQualityControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDMOQualityControl_SetNow(This,rtNow)	\
    ( (This)->lpVtbl -> SetNow(This,rtNow) ) 

#define IDMOQualityControl_SetStatus(This,dwFlags)	\
    ( (This)->lpVtbl -> SetStatus(This,dwFlags) ) 

#define IDMOQualityControl_GetStatus(This,pdwFlags)	\
    ( (This)->lpVtbl -> GetStatus(This,pdwFlags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDMOQualityControl_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mediaobj_0000_0005 */
/* [local] */ 


enum _DMO_VIDEO_OUTPUT_STREAM_FLAGS
    {
        DMO_VOSF_NEEDS_PREVIOUS_SAMPLE	= 0x1
    } ;


extern RPC_IF_HANDLE __MIDL_itf_mediaobj_0000_0005_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mediaobj_0000_0005_v0_0_s_ifspec;

#ifndef __IDMOVideoOutputOptimizations_INTERFACE_DEFINED__
#define __IDMOVideoOutputOptimizations_INTERFACE_DEFINED__

/* interface IDMOVideoOutputOptimizations */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IDMOVideoOutputOptimizations;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("be8f4f4e-5b16-4d29-b350-7f6b5d9298ac")
    IDMOVideoOutputOptimizations : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE QueryOperationModePreferences( 
            ULONG ulOutputStreamIndex,
            /* [annotation] */ 
            _Out_  DWORD *pdwRequestedCapabilities) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetOperationMode( 
            ULONG ulOutputStreamIndex,
            DWORD dwEnabledFeatures) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCurrentOperationMode( 
            ULONG ulOutputStreamIndex,
            /* [annotation] */ 
            _Out_  DWORD *pdwEnabledFeatures) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCurrentSampleRequirements( 
            ULONG ulOutputStreamIndex,
            /* [annotation] */ 
            _Out_  DWORD *pdwRequestedFeatures) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDMOVideoOutputOptimizationsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDMOVideoOutputOptimizations * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDMOVideoOutputOptimizations * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDMOVideoOutputOptimizations * This);
        
        DECLSPEC_XFGVIRT(IDMOVideoOutputOptimizations, QueryOperationModePreferences)
        HRESULT ( STDMETHODCALLTYPE *QueryOperationModePreferences )( 
            IDMOVideoOutputOptimizations * This,
            ULONG ulOutputStreamIndex,
            /* [annotation] */ 
            _Out_  DWORD *pdwRequestedCapabilities);
        
        DECLSPEC_XFGVIRT(IDMOVideoOutputOptimizations, SetOperationMode)
        HRESULT ( STDMETHODCALLTYPE *SetOperationMode )( 
            IDMOVideoOutputOptimizations * This,
            ULONG ulOutputStreamIndex,
            DWORD dwEnabledFeatures);
        
        DECLSPEC_XFGVIRT(IDMOVideoOutputOptimizations, GetCurrentOperationMode)
        HRESULT ( STDMETHODCALLTYPE *GetCurrentOperationMode )( 
            IDMOVideoOutputOptimizations * This,
            ULONG ulOutputStreamIndex,
            /* [annotation] */ 
            _Out_  DWORD *pdwEnabledFeatures);
        
        DECLSPEC_XFGVIRT(IDMOVideoOutputOptimizations, GetCurrentSampleRequirements)
        HRESULT ( STDMETHODCALLTYPE *GetCurrentSampleRequirements )( 
            IDMOVideoOutputOptimizations * This,
            ULONG ulOutputStreamIndex,
            /* [annotation] */ 
            _Out_  DWORD *pdwRequestedFeatures);
        
        END_INTERFACE
    } IDMOVideoOutputOptimizationsVtbl;

    interface IDMOVideoOutputOptimizations
    {
        CONST_VTBL struct IDMOVideoOutputOptimizationsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDMOVideoOutputOptimizations_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDMOVideoOutputOptimizations_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDMOVideoOutputOptimizations_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDMOVideoOutputOptimizations_QueryOperationModePreferences(This,ulOutputStreamIndex,pdwRequestedCapabilities)	\
    ( (This)->lpVtbl -> QueryOperationModePreferences(This,ulOutputStreamIndex,pdwRequestedCapabilities) ) 

#define IDMOVideoOutputOptimizations_SetOperationMode(This,ulOutputStreamIndex,dwEnabledFeatures)	\
    ( (This)->lpVtbl -> SetOperationMode(This,ulOutputStreamIndex,dwEnabledFeatures) ) 

#define IDMOVideoOutputOptimizations_GetCurrentOperationMode(This,ulOutputStreamIndex,pdwEnabledFeatures)	\
    ( (This)->lpVtbl -> GetCurrentOperationMode(This,ulOutputStreamIndex,pdwEnabledFeatures) ) 

#define IDMOVideoOutputOptimizations_GetCurrentSampleRequirements(This,ulOutputStreamIndex,pdwRequestedFeatures)	\
    ( (This)->lpVtbl -> GetCurrentSampleRequirements(This,ulOutputStreamIndex,pdwRequestedFeatures) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDMOVideoOutputOptimizations_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mediaobj_0000_0006 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_mediaobj_0000_0006_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mediaobj_0000_0006_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif





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

#ifndef __mftransform_h__
#define __mftransform_h__

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

#ifndef __IMFTransform_FWD_DEFINED__
#define __IMFTransform_FWD_DEFINED__
typedef interface IMFTransform IMFTransform;

#endif 	/* __IMFTransform_FWD_DEFINED__ */


#ifndef __IMFDeviceTransform_FWD_DEFINED__
#define __IMFDeviceTransform_FWD_DEFINED__
typedef interface IMFDeviceTransform IMFDeviceTransform;

#endif 	/* __IMFDeviceTransform_FWD_DEFINED__ */


#ifndef __IMFDeviceTransform2_FWD_DEFINED__
#define __IMFDeviceTransform2_FWD_DEFINED__
typedef interface IMFDeviceTransform2 IMFDeviceTransform2;

#endif 	/* __IMFDeviceTransform2_FWD_DEFINED__ */


#ifndef __IMFDeviceTransformCallback_FWD_DEFINED__
#define __IMFDeviceTransformCallback_FWD_DEFINED__
typedef interface IMFDeviceTransformCallback IMFDeviceTransformCallback;

#endif 	/* __IMFDeviceTransformCallback_FWD_DEFINED__ */


/* header files for imported files */
#include "mfobjects.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_mftransform_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES)
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES) */
#pragma endregion
#pragma region Application or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)

enum _MFT_INPUT_DATA_BUFFER_FLAGS
    {
        MFT_INPUT_DATA_BUFFER_PLACEHOLDER	= 0xffffffff
    } ;

enum _MFT_OUTPUT_DATA_BUFFER_FLAGS
    {
        MFT_OUTPUT_DATA_BUFFER_INCOMPLETE	= 0x1000000,
        MFT_OUTPUT_DATA_BUFFER_FORMAT_CHANGE	= 0x100,
        MFT_OUTPUT_DATA_BUFFER_STREAM_END	= 0x200,
        MFT_OUTPUT_DATA_BUFFER_NO_SAMPLE	= 0x300
    } ;

enum _MFT_INPUT_STATUS_FLAGS
    {
        MFT_INPUT_STATUS_ACCEPT_DATA	= 0x1
    } ;

enum _MFT_OUTPUT_STATUS_FLAGS
    {
        MFT_OUTPUT_STATUS_SAMPLE_READY	= 0x1
    } ;

enum _MFT_INPUT_STREAM_INFO_FLAGS
    {
        MFT_INPUT_STREAM_WHOLE_SAMPLES	= 0x1,
        MFT_INPUT_STREAM_SINGLE_SAMPLE_PER_BUFFER	= 0x2,
        MFT_INPUT_STREAM_FIXED_SAMPLE_SIZE	= 0x4,
        MFT_INPUT_STREAM_HOLDS_BUFFERS	= 0x8,
        MFT_INPUT_STREAM_DOES_NOT_ADDREF	= 0x100,
        MFT_INPUT_STREAM_REMOVABLE	= 0x200,
        MFT_INPUT_STREAM_OPTIONAL	= 0x400,
        MFT_INPUT_STREAM_PROCESSES_IN_PLACE	= 0x800
    } ;

enum _MFT_OUTPUT_STREAM_INFO_FLAGS
    {
        MFT_OUTPUT_STREAM_WHOLE_SAMPLES	= 0x1,
        MFT_OUTPUT_STREAM_SINGLE_SAMPLE_PER_BUFFER	= 0x2,
        MFT_OUTPUT_STREAM_FIXED_SAMPLE_SIZE	= 0x4,
        MFT_OUTPUT_STREAM_DISCARDABLE	= 0x8,
        MFT_OUTPUT_STREAM_OPTIONAL	= 0x10,
        MFT_OUTPUT_STREAM_PROVIDES_SAMPLES	= 0x100,
        MFT_OUTPUT_STREAM_CAN_PROVIDE_SAMPLES	= 0x200,
        MFT_OUTPUT_STREAM_LAZY_READ	= 0x400,
        MFT_OUTPUT_STREAM_REMOVABLE	= 0x800
    } ;

enum _MFT_SET_TYPE_FLAGS
    {
        MFT_SET_TYPE_TEST_ONLY	= 0x1
    } ;

enum _MFT_PROCESS_OUTPUT_FLAGS
    {
        MFT_PROCESS_OUTPUT_DISCARD_WHEN_NO_BUFFER	= 0x1,
        MFT_PROCESS_OUTPUT_REGENERATE_LAST_OUTPUT	= 0x2
    } ;

enum _MFT_PROCESS_OUTPUT_STATUS
    {
        MFT_PROCESS_OUTPUT_STATUS_NEW_STREAMS	= 0x100
    } ;

enum _MFT_DRAIN_TYPE
    {
        MFT_DRAIN_PRODUCE_TAILS	= 0,
        MFT_DRAIN_NO_TAILS	= 0x1
    } ;
#define MFT_STREAMS_UNLIMITED       0xFFFFFFFF
#define MFT_OUTPUT_BOUND_LOWER_UNBOUNDED MINLONGLONG
#define MFT_OUTPUT_BOUND_UPPER_UNBOUNDED MAXLONGLONG
typedef /* [v1_enum] */ 
enum _MFT_MESSAGE_TYPE
    {
        MFT_MESSAGE_COMMAND_FLUSH	= 0,
        MFT_MESSAGE_COMMAND_DRAIN	= 0x1,
        MFT_MESSAGE_SET_D3D_MANAGER	= 0x2,
        MFT_MESSAGE_DROP_SAMPLES	= 0x3,
        MFT_MESSAGE_COMMAND_TICK	= 0x4,
        MFT_MESSAGE_NOTIFY_BEGIN_STREAMING	= 0x10000000,
        MFT_MESSAGE_NOTIFY_END_STREAMING	= 0x10000001,
        MFT_MESSAGE_NOTIFY_END_OF_STREAM	= 0x10000002,
        MFT_MESSAGE_NOTIFY_START_OF_STREAM	= 0x10000003,
        MFT_MESSAGE_NOTIFY_RELEASE_RESOURCES	= 0x10000004,
        MFT_MESSAGE_NOTIFY_REACQUIRE_RESOURCES	= 0x10000005,
        MFT_MESSAGE_NOTIFY_EVENT	= 0x10000006,
        MFT_MESSAGE_COMMAND_SET_OUTPUT_STREAM_STATE	= 0x10000007,
        MFT_MESSAGE_COMMAND_FLUSH_OUTPUT_STREAM	= 0x10000008,
        MFT_MESSAGE_COMMAND_MARKER	= 0x20000000
    } 	MFT_MESSAGE_TYPE;

typedef struct _MFT_INPUT_STREAM_INFO
    {
    LONGLONG hnsMaxLatency;
    DWORD dwFlags;
    DWORD cbSize;
    DWORD cbMaxLookahead;
    DWORD cbAlignment;
    } 	MFT_INPUT_STREAM_INFO;

typedef struct _MFT_OUTPUT_STREAM_INFO
    {
    DWORD dwFlags;
    DWORD cbSize;
    DWORD cbAlignment;
    } 	MFT_OUTPUT_STREAM_INFO;

typedef struct _MFT_OUTPUT_DATA_BUFFER
    {
    DWORD dwStreamID;
    IMFSample *pSample;
    DWORD dwStatus;
    IMFCollection *pEvents;
    } 	MFT_OUTPUT_DATA_BUFFER;

typedef struct _MFT_OUTPUT_DATA_BUFFER *PMFT_OUTPUT_DATA_BUFFER;

//
// redefine all the method names to have MFT at the beginning so they don't class with DMO methods.
//
#ifdef MFT_UNIQUE_METHOD_NAMES
#define GetStreamLimits         MFTGetStreamLimits
#define GetStreamCount          MFTGetStreamCount
#define GetStreamIDs            MFTGetStreamIDs
#define GetInputStreamInfo      MFTGetInputStreamInfo
#define GetOutputStreamInfo     MFTGetOutputStreamInfo
#define DeleteInputStream       MFTDeleteInputStream
#define AddInputStreams         MFTAddInputStreams
#define GetInputAvailableType   MFTGetInputAvailableType
#define GetOutputAvailableType  MFTGetOutputAvailableType
#define SetInputType            MFTSetInputType
#define SetOutputType           MFTSetOutputType
#define GetInputCurrentType     MFTGetInputCurrentType
#define GetOutputCurrentType    MFTGetOutputCurrentType
#define GetInputStatus          MFTGetInputStatus
#define GetOutputStatus         MFTGetOutputStatus
#define SetOutputBounds         MFTSetOutputBounds
#define ProcessEvent            MFTProcessEvent
#define ProcessMessage          MFTProcessMessage
#define ProcessInput            MFTProcessInput
#define ProcessOutput           MFTProcessOutput
#endif


extern RPC_IF_HANDLE __MIDL_itf_mftransform_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mftransform_0000_0000_v0_0_s_ifspec;

#ifndef __IMFTransform_INTERFACE_DEFINED__
#define __IMFTransform_INTERFACE_DEFINED__

/* interface IMFTransform */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMFTransform;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("bf94c121-5b05-4e6f-8000-ba598961414d")
    IMFTransform : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetStreamLimits( 
            /* [out] */ __RPC__out DWORD *pdwInputMinimum,
            /* [out] */ __RPC__out DWORD *pdwInputMaximum,
            /* [out] */ __RPC__out DWORD *pdwOutputMinimum,
            /* [out] */ __RPC__out DWORD *pdwOutputMaximum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStreamCount( 
            /* [out] */ __RPC__out DWORD *pcInputStreams,
            /* [out] */ __RPC__out DWORD *pcOutputStreams) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStreamIDs( 
            DWORD dwInputIDArraySize,
            /* [size_is][out] */ __RPC__out_ecount_full(dwInputIDArraySize) DWORD *pdwInputIDs,
            DWORD dwOutputIDArraySize,
            /* [size_is][out] */ __RPC__out_ecount_full(dwOutputIDArraySize) DWORD *pdwOutputIDs) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetInputStreamInfo( 
            DWORD dwInputStreamID,
            /* [out] */ __RPC__out MFT_INPUT_STREAM_INFO *pStreamInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOutputStreamInfo( 
            DWORD dwOutputStreamID,
            /* [out] */ __RPC__out MFT_OUTPUT_STREAM_INFO *pStreamInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAttributes( 
            /* [out] */ __RPC__deref_out_opt IMFAttributes **pAttributes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetInputStreamAttributes( 
            DWORD dwInputStreamID,
            /* [out] */ __RPC__deref_out_opt IMFAttributes **pAttributes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOutputStreamAttributes( 
            DWORD dwOutputStreamID,
            /* [out] */ __RPC__deref_out_opt IMFAttributes **pAttributes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteInputStream( 
            DWORD dwStreamID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddInputStreams( 
            DWORD cStreams,
            /* [in] */ __RPC__in DWORD *adwStreamIDs) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetInputAvailableType( 
            DWORD dwInputStreamID,
            DWORD dwTypeIndex,
            /* [out] */ __RPC__deref_out_opt IMFMediaType **ppType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOutputAvailableType( 
            DWORD dwOutputStreamID,
            DWORD dwTypeIndex,
            /* [out] */ __RPC__deref_out_opt IMFMediaType **ppType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetInputType( 
            DWORD dwInputStreamID,
            /* [in] */ __RPC__in_opt IMFMediaType *pType,
            DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetOutputType( 
            DWORD dwOutputStreamID,
            /* [in] */ __RPC__in_opt IMFMediaType *pType,
            DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetInputCurrentType( 
            DWORD dwInputStreamID,
            /* [out] */ __RPC__deref_out_opt IMFMediaType **ppType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOutputCurrentType( 
            DWORD dwOutputStreamID,
            /* [out] */ __RPC__deref_out_opt IMFMediaType **ppType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetInputStatus( 
            DWORD dwInputStreamID,
            /* [out] */ __RPC__out DWORD *pdwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOutputStatus( 
            /* [out] */ __RPC__out DWORD *pdwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetOutputBounds( 
            LONGLONG hnsLowerBound,
            LONGLONG hnsUpperBound) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ProcessEvent( 
            DWORD dwInputStreamID,
            /* [in] */ __RPC__in_opt IMFMediaEvent *pEvent) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ProcessMessage( 
            MFT_MESSAGE_TYPE eMessage,
            ULONG_PTR ulParam) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE ProcessInput( 
            DWORD dwInputStreamID,
            IMFSample *pSample,
            DWORD dwFlags) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE ProcessOutput( 
            DWORD dwFlags,
            DWORD cOutputBufferCount,
            /* [size_is][out][in] */ MFT_OUTPUT_DATA_BUFFER *pOutputSamples,
            /* [out] */ DWORD *pdwStatus) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFTransformVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFTransform * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFTransform * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFTransform * This);
        
        DECLSPEC_XFGVIRT(IMFTransform, GetStreamLimits)
        HRESULT ( STDMETHODCALLTYPE *GetStreamLimits )( 
            __RPC__in IMFTransform * This,
            /* [out] */ __RPC__out DWORD *pdwInputMinimum,
            /* [out] */ __RPC__out DWORD *pdwInputMaximum,
            /* [out] */ __RPC__out DWORD *pdwOutputMinimum,
            /* [out] */ __RPC__out DWORD *pdwOutputMaximum);
        
        DECLSPEC_XFGVIRT(IMFTransform, GetStreamCount)
        HRESULT ( STDMETHODCALLTYPE *GetStreamCount )( 
            __RPC__in IMFTransform * This,
            /* [out] */ __RPC__out DWORD *pcInputStreams,
            /* [out] */ __RPC__out DWORD *pcOutputStreams);
        
        DECLSPEC_XFGVIRT(IMFTransform, GetStreamIDs)
        HRESULT ( STDMETHODCALLTYPE *GetStreamIDs )( 
            __RPC__in IMFTransform * This,
            DWORD dwInputIDArraySize,
            /* [size_is][out] */ __RPC__out_ecount_full(dwInputIDArraySize) DWORD *pdwInputIDs,
            DWORD dwOutputIDArraySize,
            /* [size_is][out] */ __RPC__out_ecount_full(dwOutputIDArraySize) DWORD *pdwOutputIDs);
        
        DECLSPEC_XFGVIRT(IMFTransform, GetInputStreamInfo)
        HRESULT ( STDMETHODCALLTYPE *GetInputStreamInfo )( 
            __RPC__in IMFTransform * This,
            DWORD dwInputStreamID,
            /* [out] */ __RPC__out MFT_INPUT_STREAM_INFO *pStreamInfo);
        
        DECLSPEC_XFGVIRT(IMFTransform, GetOutputStreamInfo)
        HRESULT ( STDMETHODCALLTYPE *GetOutputStreamInfo )( 
            __RPC__in IMFTransform * This,
            DWORD dwOutputStreamID,
            /* [out] */ __RPC__out MFT_OUTPUT_STREAM_INFO *pStreamInfo);
        
        DECLSPEC_XFGVIRT(IMFTransform, GetAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetAttributes )( 
            __RPC__in IMFTransform * This,
            /* [out] */ __RPC__deref_out_opt IMFAttributes **pAttributes);
        
        DECLSPEC_XFGVIRT(IMFTransform, GetInputStreamAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetInputStreamAttributes )( 
            __RPC__in IMFTransform * This,
            DWORD dwInputStreamID,
            /* [out] */ __RPC__deref_out_opt IMFAttributes **pAttributes);
        
        DECLSPEC_XFGVIRT(IMFTransform, GetOutputStreamAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetOutputStreamAttributes )( 
            __RPC__in IMFTransform * This,
            DWORD dwOutputStreamID,
            /* [out] */ __RPC__deref_out_opt IMFAttributes **pAttributes);
        
        DECLSPEC_XFGVIRT(IMFTransform, DeleteInputStream)
        HRESULT ( STDMETHODCALLTYPE *DeleteInputStream )( 
            __RPC__in IMFTransform * This,
            DWORD dwStreamID);
        
        DECLSPEC_XFGVIRT(IMFTransform, AddInputStreams)
        HRESULT ( STDMETHODCALLTYPE *AddInputStreams )( 
            __RPC__in IMFTransform * This,
            DWORD cStreams,
            /* [in] */ __RPC__in DWORD *adwStreamIDs);
        
        DECLSPEC_XFGVIRT(IMFTransform, GetInputAvailableType)
        HRESULT ( STDMETHODCALLTYPE *GetInputAvailableType )( 
            __RPC__in IMFTransform * This,
            DWORD dwInputStreamID,
            DWORD dwTypeIndex,
            /* [out] */ __RPC__deref_out_opt IMFMediaType **ppType);
        
        DECLSPEC_XFGVIRT(IMFTransform, GetOutputAvailableType)
        HRESULT ( STDMETHODCALLTYPE *GetOutputAvailableType )( 
            __RPC__in IMFTransform * This,
            DWORD dwOutputStreamID,
            DWORD dwTypeIndex,
            /* [out] */ __RPC__deref_out_opt IMFMediaType **ppType);
        
        DECLSPEC_XFGVIRT(IMFTransform, SetInputType)
        HRESULT ( STDMETHODCALLTYPE *SetInputType )( 
            __RPC__in IMFTransform * This,
            DWORD dwInputStreamID,
            /* [in] */ __RPC__in_opt IMFMediaType *pType,
            DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IMFTransform, SetOutputType)
        HRESULT ( STDMETHODCALLTYPE *SetOutputType )( 
            __RPC__in IMFTransform * This,
            DWORD dwOutputStreamID,
            /* [in] */ __RPC__in_opt IMFMediaType *pType,
            DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IMFTransform, GetInputCurrentType)
        HRESULT ( STDMETHODCALLTYPE *GetInputCurrentType )( 
            __RPC__in IMFTransform * This,
            DWORD dwInputStreamID,
            /* [out] */ __RPC__deref_out_opt IMFMediaType **ppType);
        
        DECLSPEC_XFGVIRT(IMFTransform, GetOutputCurrentType)
        HRESULT ( STDMETHODCALLTYPE *GetOutputCurrentType )( 
            __RPC__in IMFTransform * This,
            DWORD dwOutputStreamID,
            /* [out] */ __RPC__deref_out_opt IMFMediaType **ppType);
        
        DECLSPEC_XFGVIRT(IMFTransform, GetInputStatus)
        HRESULT ( STDMETHODCALLTYPE *GetInputStatus )( 
            __RPC__in IMFTransform * This,
            DWORD dwInputStreamID,
            /* [out] */ __RPC__out DWORD *pdwFlags);
        
        DECLSPEC_XFGVIRT(IMFTransform, GetOutputStatus)
        HRESULT ( STDMETHODCALLTYPE *GetOutputStatus )( 
            __RPC__in IMFTransform * This,
            /* [out] */ __RPC__out DWORD *pdwFlags);
        
        DECLSPEC_XFGVIRT(IMFTransform, SetOutputBounds)
        HRESULT ( STDMETHODCALLTYPE *SetOutputBounds )( 
            __RPC__in IMFTransform * This,
            LONGLONG hnsLowerBound,
            LONGLONG hnsUpperBound);
        
        DECLSPEC_XFGVIRT(IMFTransform, ProcessEvent)
        HRESULT ( STDMETHODCALLTYPE *ProcessEvent )( 
            __RPC__in IMFTransform * This,
            DWORD dwInputStreamID,
            /* [in] */ __RPC__in_opt IMFMediaEvent *pEvent);
        
        DECLSPEC_XFGVIRT(IMFTransform, ProcessMessage)
        HRESULT ( STDMETHODCALLTYPE *ProcessMessage )( 
            __RPC__in IMFTransform * This,
            MFT_MESSAGE_TYPE eMessage,
            ULONG_PTR ulParam);
        
        DECLSPEC_XFGVIRT(IMFTransform, ProcessInput)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *ProcessInput )( 
            IMFTransform * This,
            DWORD dwInputStreamID,
            IMFSample *pSample,
            DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IMFTransform, ProcessOutput)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *ProcessOutput )( 
            IMFTransform * This,
            DWORD dwFlags,
            DWORD cOutputBufferCount,
            /* [size_is][out][in] */ MFT_OUTPUT_DATA_BUFFER *pOutputSamples,
            /* [out] */ DWORD *pdwStatus);
        
        END_INTERFACE
    } IMFTransformVtbl;

    interface IMFTransform
    {
        CONST_VTBL struct IMFTransformVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFTransform_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFTransform_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFTransform_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFTransform_GetStreamLimits(This,pdwInputMinimum,pdwInputMaximum,pdwOutputMinimum,pdwOutputMaximum)	\
    ( (This)->lpVtbl -> GetStreamLimits(This,pdwInputMinimum,pdwInputMaximum,pdwOutputMinimum,pdwOutputMaximum) ) 

#define IMFTransform_GetStreamCount(This,pcInputStreams,pcOutputStreams)	\
    ( (This)->lpVtbl -> GetStreamCount(This,pcInputStreams,pcOutputStreams) ) 

#define IMFTransform_GetStreamIDs(This,dwInputIDArraySize,pdwInputIDs,dwOutputIDArraySize,pdwOutputIDs)	\
    ( (This)->lpVtbl -> GetStreamIDs(This,dwInputIDArraySize,pdwInputIDs,dwOutputIDArraySize,pdwOutputIDs) ) 

#define IMFTransform_GetInputStreamInfo(This,dwInputStreamID,pStreamInfo)	\
    ( (This)->lpVtbl -> GetInputStreamInfo(This,dwInputStreamID,pStreamInfo) ) 

#define IMFTransform_GetOutputStreamInfo(This,dwOutputStreamID,pStreamInfo)	\
    ( (This)->lpVtbl -> GetOutputStreamInfo(This,dwOutputStreamID,pStreamInfo) ) 

#define IMFTransform_GetAttributes(This,pAttributes)	\
    ( (This)->lpVtbl -> GetAttributes(This,pAttributes) ) 

#define IMFTransform_GetInputStreamAttributes(This,dwInputStreamID,pAttributes)	\
    ( (This)->lpVtbl -> GetInputStreamAttributes(This,dwInputStreamID,pAttributes) ) 

#define IMFTransform_GetOutputStreamAttributes(This,dwOutputStreamID,pAttributes)	\
    ( (This)->lpVtbl -> GetOutputStreamAttributes(This,dwOutputStreamID,pAttributes) ) 

#define IMFTransform_DeleteInputStream(This,dwStreamID)	\
    ( (This)->lpVtbl -> DeleteInputStream(This,dwStreamID) ) 

#define IMFTransform_AddInputStreams(This,cStreams,adwStreamIDs)	\
    ( (This)->lpVtbl -> AddInputStreams(This,cStreams,adwStreamIDs) ) 

#define IMFTransform_GetInputAvailableType(This,dwInputStreamID,dwTypeIndex,ppType)	\
    ( (This)->lpVtbl -> GetInputAvailableType(This,dwInputStreamID,dwTypeIndex,ppType) ) 

#define IMFTransform_GetOutputAvailableType(This,dwOutputStreamID,dwTypeIndex,ppType)	\
    ( (This)->lpVtbl -> GetOutputAvailableType(This,dwOutputStreamID,dwTypeIndex,ppType) ) 

#define IMFTransform_SetInputType(This,dwInputStreamID,pType,dwFlags)	\
    ( (This)->lpVtbl -> SetInputType(This,dwInputStreamID,pType,dwFlags) ) 

#define IMFTransform_SetOutputType(This,dwOutputStreamID,pType,dwFlags)	\
    ( (This)->lpVtbl -> SetOutputType(This,dwOutputStreamID,pType,dwFlags) ) 

#define IMFTransform_GetInputCurrentType(This,dwInputStreamID,ppType)	\
    ( (This)->lpVtbl -> GetInputCurrentType(This,dwInputStreamID,ppType) ) 

#define IMFTransform_GetOutputCurrentType(This,dwOutputStreamID,ppType)	\
    ( (This)->lpVtbl -> GetOutputCurrentType(This,dwOutputStreamID,ppType) ) 

#define IMFTransform_GetInputStatus(This,dwInputStreamID,pdwFlags)	\
    ( (This)->lpVtbl -> GetInputStatus(This,dwInputStreamID,pdwFlags) ) 

#define IMFTransform_GetOutputStatus(This,pdwFlags)	\
    ( (This)->lpVtbl -> GetOutputStatus(This,pdwFlags) ) 

#define IMFTransform_SetOutputBounds(This,hnsLowerBound,hnsUpperBound)	\
    ( (This)->lpVtbl -> SetOutputBounds(This,hnsLowerBound,hnsUpperBound) ) 

#define IMFTransform_ProcessEvent(This,dwInputStreamID,pEvent)	\
    ( (This)->lpVtbl -> ProcessEvent(This,dwInputStreamID,pEvent) ) 

#define IMFTransform_ProcessMessage(This,eMessage,ulParam)	\
    ( (This)->lpVtbl -> ProcessMessage(This,eMessage,ulParam) ) 

#define IMFTransform_ProcessInput(This,dwInputStreamID,pSample,dwFlags)	\
    ( (This)->lpVtbl -> ProcessInput(This,dwInputStreamID,pSample,dwFlags) ) 

#define IMFTransform_ProcessOutput(This,dwFlags,cOutputBufferCount,pOutputSamples,pdwStatus)	\
    ( (This)->lpVtbl -> ProcessOutput(This,dwFlags,cOutputBufferCount,pOutputSamples,pdwStatus) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFTransform_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mftransform_0000_0001 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion
typedef 
enum _DeviceStreamState
    {
        DeviceStreamState_Stop	= 0,
        DeviceStreamState_Pause	= ( DeviceStreamState_Stop + 1 ) ,
        DeviceStreamState_Run	= ( DeviceStreamState_Pause + 1 ) ,
        DeviceStreamState_Disabled	= ( DeviceStreamState_Run + 1 ) 
    } 	DeviceStreamState;

typedef enum _DeviceStreamState *PDeviceStreamState;

EXTERN_GUID(MEDeviceStreamCreated, 0x0252a1cf, 0x3540, 0x43b4, 0x91, 0x64, 0xd7, 0x2e, 0xb4, 0x05, 0xfa, 0x40);
#if (WINVER >= _WIN32_WINNT_WIN7) 
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
typedef struct _STREAM_MEDIUM
    {
    GUID gidMedium;
    UINT32 unMediumInstance;
    } 	STREAM_MEDIUM;

typedef struct _STREAM_MEDIUM *PSTREAM_MEDIUM;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#endif // (WINVER >= _WIN32_WINNT_WIN7) 
#pragma region Application or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)
//
// Define the MFT methods back so we don't accidentally hose the IMediaObject interface.
//
#ifdef MFT_UNIQUE_METHOD_NAMES
#undef GetStreamLimits        
#undef GetStreamCount         
#undef GetStreamIDs           
#undef GetInputStreamInfo     
#undef GetOutputStreamInfo    
#undef DeleteInputStream      
#undef AddInputStreams        
#undef GetInputAvailableType  
#undef GetOutputAvailableType 
#undef SetInputType           
#undef SetOutputType          
#undef GetInputCurrentType    
#undef GetOutputCurrentType   
#undef GetInputStatus         
#undef GetOutputStatus        
#undef SetOutputBounds        
#undef ProcessMessage         
#undef ProcessInput           
#undef ProcessOutput          
#endif
EXTERN_C const DECLSPEC_SELECTANY PROPERTYKEY MFPKEY_CLSID = { { 0xc57a84c0, 0x1a80, 0x40a3, {0x97, 0xb5, 0x92, 0x72, 0xa4, 0x3, 0xc8, 0xae} }, 0x01 }; 
EXTERN_C const DECLSPEC_SELECTANY PROPERTYKEY MFPKEY_CATEGORY = { { 0xc57a84c0, 0x1a80, 0x40a3, {0x97, 0xb5, 0x92, 0x72, 0xa4, 0x3, 0xc8, 0xae} }, 0x02 }; 
EXTERN_C const DECLSPEC_SELECTANY PROPERTYKEY MFPKEY_EXATTRIBUTE_SUPPORTED = { { 0x456fe843, 0x3c87, 0x40c0, {0x94, 0x9d, 0x14, 0x9, 0xc9, 0x7d, 0xab, 0x2c} }, 0x01 }; 
EXTERN_C const DECLSPEC_SELECTANY PROPERTYKEY MFPKEY_MULTICHANNEL_CHANNEL_MASK  = { { 0x58bdaf8c, 0x3224, 0x4692, { 0x86, 0xd0, 0x44, 0xd6, 0x5c, 0x5b, 0xf8, 0x2b } }, 0x01 }; 
EXTERN_C const DECLSPEC_SELECTANY GUID MF_SA_D3D_AWARE = { 0xeaa35c29,  0x775e, 0x488e, { 0x9b, 0x61, 0xb3, 0x28, 0x3e, 0x49, 0x58, 0x3b } }; 
EXTERN_C const DECLSPEC_SELECTANY GUID MF_SA_REQUIRED_SAMPLE_COUNT = { 0x18802c61, 0x324b, 0x4952, { 0xab, 0xd0, 0x17, 0x6f, 0xf5, 0xc6, 0x96, 0xff } };
 EXTERN_C const DECLSPEC_SELECTANY GUID MFT_END_STREAMING_AWARE = { 0x70fbc845,  0xb07e, 0x4089, { 0xb0, 0x64, 0x39, 0x9d, 0xc6, 0x11, 0xf, 0x29 } }; 
 EXTERN_C const DECLSPEC_SELECTANY GUID MF_SA_AUDIO_ENDPOINT_AWARE = { 0xc0381701, 0x805c, 0x42b2,{ 0xac, 0x8d, 0xe2, 0xb4, 0xbf, 0x21, 0xf4, 0xf8 } }; 
 EXTERN_C const DECLSPEC_SELECTANY GUID MFT_AUDIO_DECODER_AUDIO_ENDPOINT_ID = { 0xc7ccdd6e, 0x5398, 0x4695,{ 0x8b, 0xe7, 0x51, 0xb3, 0xe9, 0x51, 0x11, 0xbd } }; 
 EXTERN_C const DECLSPEC_SELECTANY GUID MFT_AUDIO_DECODER_SPATIAL_METADATA_CLIENT = { 0x5987df4, 0x1270, 0x4999,{ 0x92, 0x5f, 0x8e, 0x93, 0x9a, 0x7c, 0xa, 0xf7 } }; 
EXTERN_C const DECLSPEC_SELECTANY GUID MFT_AUDIO_DECODER_AUDIO_ENDPOINT_FORMFACTOR = { 0x8d574310, 0x909a, 0x433a, { 0xac, 0xe7, 0xee, 0xe7, 0x47, 0x19, 0xf9, 0x01 } };
EXTERN_C const DECLSPEC_SELECTANY GUID MFT_AUDIO_DECODER_AUDIO_ENDPOINT_IS_DIGITAL_STEREO_ONLY = { 0x26e5a90d, 0x4ad1, 0x4f8c, { 0xb8, 0xaf, 0xad, 0xf1, 0x4d, 0x21, 0x78, 0xf1 } };
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion
#if (WINVER >= _WIN32_WINNT_WINTHRESHOLD) 


extern RPC_IF_HANDLE __MIDL_itf_mftransform_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mftransform_0000_0001_v0_0_s_ifspec;

#ifndef __IMFDeviceTransform_INTERFACE_DEFINED__
#define __IMFDeviceTransform_INTERFACE_DEFINED__

/* interface IMFDeviceTransform */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFDeviceTransform;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D818FBD8-FC46-42F2-87AC-1EA2D1F9BF32")
    IMFDeviceTransform : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE InitializeTransform( 
            /* [annotation][in] */ 
            _In_  IMFAttributes *pAttributes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetInputAvailableType( 
            /* [annotation][in] */ 
            _In_  DWORD dwInputStreamID,
            /* [annotation][in] */ 
            _In_  DWORD dwTypeIndex,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFMediaType **pMediaType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetInputCurrentType( 
            /* [annotation][in] */ 
            _In_  DWORD dwInputStreamID,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFMediaType **pMediaType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetInputStreamAttributes( 
            /* [annotation][in] */ 
            _In_  DWORD dwInputStreamID,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFAttributes **ppAttributes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOutputAvailableType( 
            /* [annotation][in] */ 
            _In_  DWORD dwOutputStreamID,
            /* [annotation][in] */ 
            _In_  DWORD dwTypeIndex,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFMediaType **pMediaType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOutputCurrentType( 
            /* [annotation][in] */ 
            _In_  DWORD dwOutputStreamID,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFMediaType **pMediaType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOutputStreamAttributes( 
            /* [annotation][in] */ 
            _In_  DWORD dwOutputStreamID,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFAttributes **ppAttributes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStreamCount( 
            /* [annotation][out] */ 
            _Out_  DWORD *pcInputStreams,
            /* [annotation][out] */ 
            _Out_  DWORD *pcOutputStreams) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStreamIDs( 
            /* [annotation][in] */ 
            _In_  DWORD dwInputIDArraySize,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwInputStreamIds,
            /* [annotation][in] */ 
            _In_  DWORD dwOutputIDArraySize,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwOutputStreamIds) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ProcessEvent( 
            /* [annotation][in] */ 
            _In_  DWORD dwInputStreamID,
            /* [annotation][in] */ 
            _In_  IMFMediaEvent *pEvent) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ProcessInput( 
            /* [annotation][in] */ 
            _In_  DWORD dwInputStreamID,
            /* [annotation][in] */ 
            _In_  IMFSample *pSample,
            /* [annotation][in] */ 
            _In_  DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ProcessMessage( 
            /* [annotation][in] */ 
            _In_  MFT_MESSAGE_TYPE eMessage,
            /* [annotation][in] */ 
            _In_  ULONG_PTR ulParam) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ProcessOutput( 
            /* [annotation][in] */ 
            _In_  DWORD dwFlags,
            /* [annotation][in] */ 
            _In_  DWORD cOutputBufferCount,
            /* [size_is][annotation][out][in] */ 
            _Inout_  MFT_OUTPUT_DATA_BUFFER *pOutputSample,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwStatus) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetInputStreamState( 
            /* [annotation][in] */ 
            _In_  DWORD dwStreamID,
            /* [annotation][in] */ 
            _In_  IMFMediaType *pMediaType,
            /* [annotation][in] */ 
            _In_  DeviceStreamState value,
            /* [annotation][in] */ 
            _In_  DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetInputStreamState( 
            /* [annotation][in] */ 
            _In_  DWORD dwStreamID,
            /* [annotation][out] */ 
            _Out_  DeviceStreamState *value) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetOutputStreamState( 
            /* [annotation][in] */ 
            _In_  DWORD dwStreamID,
            /* [annotation][in] */ 
            _In_  IMFMediaType *pMediaType,
            /* [annotation][in] */ 
            _In_  DeviceStreamState value,
            /* [annotation][in] */ 
            _In_  DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOutputStreamState( 
            /* [annotation][in] */ 
            _In_  DWORD dwStreamID,
            /* [annotation][out] */ 
            _Out_  DeviceStreamState *value) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetInputStreamPreferredState( 
            /* [annotation][in] */ 
            _In_  DWORD dwStreamID,
            /* [annotation][out] */ 
            _Out_  DeviceStreamState *value,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFMediaType **ppMediaType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FlushInputStream( 
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FlushOutputStream( 
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  DWORD dwFlags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFDeviceTransformVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFDeviceTransform * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFDeviceTransform * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFDeviceTransform * This);
        
        DECLSPEC_XFGVIRT(IMFDeviceTransform, InitializeTransform)
        HRESULT ( STDMETHODCALLTYPE *InitializeTransform )( 
            IMFDeviceTransform * This,
            /* [annotation][in] */ 
            _In_  IMFAttributes *pAttributes);
        
        DECLSPEC_XFGVIRT(IMFDeviceTransform, GetInputAvailableType)
        HRESULT ( STDMETHODCALLTYPE *GetInputAvailableType )( 
            IMFDeviceTransform * This,
            /* [annotation][in] */ 
            _In_  DWORD dwInputStreamID,
            /* [annotation][in] */ 
            _In_  DWORD dwTypeIndex,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFMediaType **pMediaType);
        
        DECLSPEC_XFGVIRT(IMFDeviceTransform, GetInputCurrentType)
        HRESULT ( STDMETHODCALLTYPE *GetInputCurrentType )( 
            IMFDeviceTransform * This,
            /* [annotation][in] */ 
            _In_  DWORD dwInputStreamID,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFMediaType **pMediaType);
        
        DECLSPEC_XFGVIRT(IMFDeviceTransform, GetInputStreamAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetInputStreamAttributes )( 
            IMFDeviceTransform * This,
            /* [annotation][in] */ 
            _In_  DWORD dwInputStreamID,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFAttributes **ppAttributes);
        
        DECLSPEC_XFGVIRT(IMFDeviceTransform, GetOutputAvailableType)
        HRESULT ( STDMETHODCALLTYPE *GetOutputAvailableType )( 
            IMFDeviceTransform * This,
            /* [annotation][in] */ 
            _In_  DWORD dwOutputStreamID,
            /* [annotation][in] */ 
            _In_  DWORD dwTypeIndex,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFMediaType **pMediaType);
        
        DECLSPEC_XFGVIRT(IMFDeviceTransform, GetOutputCurrentType)
        HRESULT ( STDMETHODCALLTYPE *GetOutputCurrentType )( 
            IMFDeviceTransform * This,
            /* [annotation][in] */ 
            _In_  DWORD dwOutputStreamID,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFMediaType **pMediaType);
        
        DECLSPEC_XFGVIRT(IMFDeviceTransform, GetOutputStreamAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetOutputStreamAttributes )( 
            IMFDeviceTransform * This,
            /* [annotation][in] */ 
            _In_  DWORD dwOutputStreamID,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFAttributes **ppAttributes);
        
        DECLSPEC_XFGVIRT(IMFDeviceTransform, GetStreamCount)
        HRESULT ( STDMETHODCALLTYPE *GetStreamCount )( 
            IMFDeviceTransform * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pcInputStreams,
            /* [annotation][out] */ 
            _Out_  DWORD *pcOutputStreams);
        
        DECLSPEC_XFGVIRT(IMFDeviceTransform, GetStreamIDs)
        HRESULT ( STDMETHODCALLTYPE *GetStreamIDs )( 
            IMFDeviceTransform * This,
            /* [annotation][in] */ 
            _In_  DWORD dwInputIDArraySize,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwInputStreamIds,
            /* [annotation][in] */ 
            _In_  DWORD dwOutputIDArraySize,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwOutputStreamIds);
        
        DECLSPEC_XFGVIRT(IMFDeviceTransform, ProcessEvent)
        HRESULT ( STDMETHODCALLTYPE *ProcessEvent )( 
            IMFDeviceTransform * This,
            /* [annotation][in] */ 
            _In_  DWORD dwInputStreamID,
            /* [annotation][in] */ 
            _In_  IMFMediaEvent *pEvent);
        
        DECLSPEC_XFGVIRT(IMFDeviceTransform, ProcessInput)
        HRESULT ( STDMETHODCALLTYPE *ProcessInput )( 
            IMFDeviceTransform * This,
            /* [annotation][in] */ 
            _In_  DWORD dwInputStreamID,
            /* [annotation][in] */ 
            _In_  IMFSample *pSample,
            /* [annotation][in] */ 
            _In_  DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IMFDeviceTransform, ProcessMessage)
        HRESULT ( STDMETHODCALLTYPE *ProcessMessage )( 
            IMFDeviceTransform * This,
            /* [annotation][in] */ 
            _In_  MFT_MESSAGE_TYPE eMessage,
            /* [annotation][in] */ 
            _In_  ULONG_PTR ulParam);
        
        DECLSPEC_XFGVIRT(IMFDeviceTransform, ProcessOutput)
        HRESULT ( STDMETHODCALLTYPE *ProcessOutput )( 
            IMFDeviceTransform * This,
            /* [annotation][in] */ 
            _In_  DWORD dwFlags,
            /* [annotation][in] */ 
            _In_  DWORD cOutputBufferCount,
            /* [size_is][annotation][out][in] */ 
            _Inout_  MFT_OUTPUT_DATA_BUFFER *pOutputSample,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwStatus);
        
        DECLSPEC_XFGVIRT(IMFDeviceTransform, SetInputStreamState)
        HRESULT ( STDMETHODCALLTYPE *SetInputStreamState )( 
            IMFDeviceTransform * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamID,
            /* [annotation][in] */ 
            _In_  IMFMediaType *pMediaType,
            /* [annotation][in] */ 
            _In_  DeviceStreamState value,
            /* [annotation][in] */ 
            _In_  DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IMFDeviceTransform, GetInputStreamState)
        HRESULT ( STDMETHODCALLTYPE *GetInputStreamState )( 
            IMFDeviceTransform * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamID,
            /* [annotation][out] */ 
            _Out_  DeviceStreamState *value);
        
        DECLSPEC_XFGVIRT(IMFDeviceTransform, SetOutputStreamState)
        HRESULT ( STDMETHODCALLTYPE *SetOutputStreamState )( 
            IMFDeviceTransform * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamID,
            /* [annotation][in] */ 
            _In_  IMFMediaType *pMediaType,
            /* [annotation][in] */ 
            _In_  DeviceStreamState value,
            /* [annotation][in] */ 
            _In_  DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IMFDeviceTransform, GetOutputStreamState)
        HRESULT ( STDMETHODCALLTYPE *GetOutputStreamState )( 
            IMFDeviceTransform * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamID,
            /* [annotation][out] */ 
            _Out_  DeviceStreamState *value);
        
        DECLSPEC_XFGVIRT(IMFDeviceTransform, GetInputStreamPreferredState)
        HRESULT ( STDMETHODCALLTYPE *GetInputStreamPreferredState )( 
            IMFDeviceTransform * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamID,
            /* [annotation][out] */ 
            _Out_  DeviceStreamState *value,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFMediaType **ppMediaType);
        
        DECLSPEC_XFGVIRT(IMFDeviceTransform, FlushInputStream)
        HRESULT ( STDMETHODCALLTYPE *FlushInputStream )( 
            IMFDeviceTransform * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IMFDeviceTransform, FlushOutputStream)
        HRESULT ( STDMETHODCALLTYPE *FlushOutputStream )( 
            IMFDeviceTransform * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  DWORD dwFlags);
        
        END_INTERFACE
    } IMFDeviceTransformVtbl;

    interface IMFDeviceTransform
    {
        CONST_VTBL struct IMFDeviceTransformVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFDeviceTransform_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFDeviceTransform_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFDeviceTransform_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFDeviceTransform_InitializeTransform(This,pAttributes)	\
    ( (This)->lpVtbl -> InitializeTransform(This,pAttributes) ) 

#define IMFDeviceTransform_GetInputAvailableType(This,dwInputStreamID,dwTypeIndex,pMediaType)	\
    ( (This)->lpVtbl -> GetInputAvailableType(This,dwInputStreamID,dwTypeIndex,pMediaType) ) 

#define IMFDeviceTransform_GetInputCurrentType(This,dwInputStreamID,pMediaType)	\
    ( (This)->lpVtbl -> GetInputCurrentType(This,dwInputStreamID,pMediaType) ) 

#define IMFDeviceTransform_GetInputStreamAttributes(This,dwInputStreamID,ppAttributes)	\
    ( (This)->lpVtbl -> GetInputStreamAttributes(This,dwInputStreamID,ppAttributes) ) 

#define IMFDeviceTransform_GetOutputAvailableType(This,dwOutputStreamID,dwTypeIndex,pMediaType)	\
    ( (This)->lpVtbl -> GetOutputAvailableType(This,dwOutputStreamID,dwTypeIndex,pMediaType) ) 

#define IMFDeviceTransform_GetOutputCurrentType(This,dwOutputStreamID,pMediaType)	\
    ( (This)->lpVtbl -> GetOutputCurrentType(This,dwOutputStreamID,pMediaType) ) 

#define IMFDeviceTransform_GetOutputStreamAttributes(This,dwOutputStreamID,ppAttributes)	\
    ( (This)->lpVtbl -> GetOutputStreamAttributes(This,dwOutputStreamID,ppAttributes) ) 

#define IMFDeviceTransform_GetStreamCount(This,pcInputStreams,pcOutputStreams)	\
    ( (This)->lpVtbl -> GetStreamCount(This,pcInputStreams,pcOutputStreams) ) 

#define IMFDeviceTransform_GetStreamIDs(This,dwInputIDArraySize,pdwInputStreamIds,dwOutputIDArraySize,pdwOutputStreamIds)	\
    ( (This)->lpVtbl -> GetStreamIDs(This,dwInputIDArraySize,pdwInputStreamIds,dwOutputIDArraySize,pdwOutputStreamIds) ) 

#define IMFDeviceTransform_ProcessEvent(This,dwInputStreamID,pEvent)	\
    ( (This)->lpVtbl -> ProcessEvent(This,dwInputStreamID,pEvent) ) 

#define IMFDeviceTransform_ProcessInput(This,dwInputStreamID,pSample,dwFlags)	\
    ( (This)->lpVtbl -> ProcessInput(This,dwInputStreamID,pSample,dwFlags) ) 

#define IMFDeviceTransform_ProcessMessage(This,eMessage,ulParam)	\
    ( (This)->lpVtbl -> ProcessMessage(This,eMessage,ulParam) ) 

#define IMFDeviceTransform_ProcessOutput(This,dwFlags,cOutputBufferCount,pOutputSample,pdwStatus)	\
    ( (This)->lpVtbl -> ProcessOutput(This,dwFlags,cOutputBufferCount,pOutputSample,pdwStatus) ) 

#define IMFDeviceTransform_SetInputStreamState(This,dwStreamID,pMediaType,value,dwFlags)	\
    ( (This)->lpVtbl -> SetInputStreamState(This,dwStreamID,pMediaType,value,dwFlags) ) 

#define IMFDeviceTransform_GetInputStreamState(This,dwStreamID,value)	\
    ( (This)->lpVtbl -> GetInputStreamState(This,dwStreamID,value) ) 

#define IMFDeviceTransform_SetOutputStreamState(This,dwStreamID,pMediaType,value,dwFlags)	\
    ( (This)->lpVtbl -> SetOutputStreamState(This,dwStreamID,pMediaType,value,dwFlags) ) 

#define IMFDeviceTransform_GetOutputStreamState(This,dwStreamID,value)	\
    ( (This)->lpVtbl -> GetOutputStreamState(This,dwStreamID,value) ) 

#define IMFDeviceTransform_GetInputStreamPreferredState(This,dwStreamID,value,ppMediaType)	\
    ( (This)->lpVtbl -> GetInputStreamPreferredState(This,dwStreamID,value,ppMediaType) ) 

#define IMFDeviceTransform_FlushInputStream(This,dwStreamIndex,dwFlags)	\
    ( (This)->lpVtbl -> FlushInputStream(This,dwStreamIndex,dwFlags) ) 

#define IMFDeviceTransform_FlushOutputStream(This,dwStreamIndex,dwFlags)	\
    ( (This)->lpVtbl -> FlushOutputStream(This,dwStreamIndex,dwFlags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFDeviceTransform_INTERFACE_DEFINED__ */


#ifndef __IMFDeviceTransform2_INTERFACE_DEFINED__
#define __IMFDeviceTransform2_INTERFACE_DEFINED__

/* interface IMFDeviceTransform2 */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFDeviceTransform2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F5980FED-B521-488F-909F-1A5FCECEDB14")
    IMFDeviceTransform2 : public IMFDeviceTransform
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTransformAttributes( 
            /* [annotation][out] */ 
            _COM_Outptr_  IMFAttributes **ppAttributes) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFDeviceTransform2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFDeviceTransform2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFDeviceTransform2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFDeviceTransform2 * This);
        
        DECLSPEC_XFGVIRT(IMFDeviceTransform, InitializeTransform)
        HRESULT ( STDMETHODCALLTYPE *InitializeTransform )( 
            IMFDeviceTransform2 * This,
            /* [annotation][in] */ 
            _In_  IMFAttributes *pAttributes);
        
        DECLSPEC_XFGVIRT(IMFDeviceTransform, GetInputAvailableType)
        HRESULT ( STDMETHODCALLTYPE *GetInputAvailableType )( 
            IMFDeviceTransform2 * This,
            /* [annotation][in] */ 
            _In_  DWORD dwInputStreamID,
            /* [annotation][in] */ 
            _In_  DWORD dwTypeIndex,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFMediaType **pMediaType);
        
        DECLSPEC_XFGVIRT(IMFDeviceTransform, GetInputCurrentType)
        HRESULT ( STDMETHODCALLTYPE *GetInputCurrentType )( 
            IMFDeviceTransform2 * This,
            /* [annotation][in] */ 
            _In_  DWORD dwInputStreamID,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFMediaType **pMediaType);
        
        DECLSPEC_XFGVIRT(IMFDeviceTransform, GetInputStreamAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetInputStreamAttributes )( 
            IMFDeviceTransform2 * This,
            /* [annotation][in] */ 
            _In_  DWORD dwInputStreamID,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFAttributes **ppAttributes);
        
        DECLSPEC_XFGVIRT(IMFDeviceTransform, GetOutputAvailableType)
        HRESULT ( STDMETHODCALLTYPE *GetOutputAvailableType )( 
            IMFDeviceTransform2 * This,
            /* [annotation][in] */ 
            _In_  DWORD dwOutputStreamID,
            /* [annotation][in] */ 
            _In_  DWORD dwTypeIndex,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFMediaType **pMediaType);
        
        DECLSPEC_XFGVIRT(IMFDeviceTransform, GetOutputCurrentType)
        HRESULT ( STDMETHODCALLTYPE *GetOutputCurrentType )( 
            IMFDeviceTransform2 * This,
            /* [annotation][in] */ 
            _In_  DWORD dwOutputStreamID,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFMediaType **pMediaType);
        
        DECLSPEC_XFGVIRT(IMFDeviceTransform, GetOutputStreamAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetOutputStreamAttributes )( 
            IMFDeviceTransform2 * This,
            /* [annotation][in] */ 
            _In_  DWORD dwOutputStreamID,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFAttributes **ppAttributes);
        
        DECLSPEC_XFGVIRT(IMFDeviceTransform, GetStreamCount)
        HRESULT ( STDMETHODCALLTYPE *GetStreamCount )( 
            IMFDeviceTransform2 * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pcInputStreams,
            /* [annotation][out] */ 
            _Out_  DWORD *pcOutputStreams);
        
        DECLSPEC_XFGVIRT(IMFDeviceTransform, GetStreamIDs)
        HRESULT ( STDMETHODCALLTYPE *GetStreamIDs )( 
            IMFDeviceTransform2 * This,
            /* [annotation][in] */ 
            _In_  DWORD dwInputIDArraySize,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwInputStreamIds,
            /* [annotation][in] */ 
            _In_  DWORD dwOutputIDArraySize,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwOutputStreamIds);
        
        DECLSPEC_XFGVIRT(IMFDeviceTransform, ProcessEvent)
        HRESULT ( STDMETHODCALLTYPE *ProcessEvent )( 
            IMFDeviceTransform2 * This,
            /* [annotation][in] */ 
            _In_  DWORD dwInputStreamID,
            /* [annotation][in] */ 
            _In_  IMFMediaEvent *pEvent);
        
        DECLSPEC_XFGVIRT(IMFDeviceTransform, ProcessInput)
        HRESULT ( STDMETHODCALLTYPE *ProcessInput )( 
            IMFDeviceTransform2 * This,
            /* [annotation][in] */ 
            _In_  DWORD dwInputStreamID,
            /* [annotation][in] */ 
            _In_  IMFSample *pSample,
            /* [annotation][in] */ 
            _In_  DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IMFDeviceTransform, ProcessMessage)
        HRESULT ( STDMETHODCALLTYPE *ProcessMessage )( 
            IMFDeviceTransform2 * This,
            /* [annotation][in] */ 
            _In_  MFT_MESSAGE_TYPE eMessage,
            /* [annotation][in] */ 
            _In_  ULONG_PTR ulParam);
        
        DECLSPEC_XFGVIRT(IMFDeviceTransform, ProcessOutput)
        HRESULT ( STDMETHODCALLTYPE *ProcessOutput )( 
            IMFDeviceTransform2 * This,
            /* [annotation][in] */ 
            _In_  DWORD dwFlags,
            /* [annotation][in] */ 
            _In_  DWORD cOutputBufferCount,
            /* [size_is][annotation][out][in] */ 
            _Inout_  MFT_OUTPUT_DATA_BUFFER *pOutputSample,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwStatus);
        
        DECLSPEC_XFGVIRT(IMFDeviceTransform, SetInputStreamState)
        HRESULT ( STDMETHODCALLTYPE *SetInputStreamState )( 
            IMFDeviceTransform2 * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamID,
            /* [annotation][in] */ 
            _In_  IMFMediaType *pMediaType,
            /* [annotation][in] */ 
            _In_  DeviceStreamState value,
            /* [annotation][in] */ 
            _In_  DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IMFDeviceTransform, GetInputStreamState)
        HRESULT ( STDMETHODCALLTYPE *GetInputStreamState )( 
            IMFDeviceTransform2 * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamID,
            /* [annotation][out] */ 
            _Out_  DeviceStreamState *value);
        
        DECLSPEC_XFGVIRT(IMFDeviceTransform, SetOutputStreamState)
        HRESULT ( STDMETHODCALLTYPE *SetOutputStreamState )( 
            IMFDeviceTransform2 * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamID,
            /* [annotation][in] */ 
            _In_  IMFMediaType *pMediaType,
            /* [annotation][in] */ 
            _In_  DeviceStreamState value,
            /* [annotation][in] */ 
            _In_  DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IMFDeviceTransform, GetOutputStreamState)
        HRESULT ( STDMETHODCALLTYPE *GetOutputStreamState )( 
            IMFDeviceTransform2 * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamID,
            /* [annotation][out] */ 
            _Out_  DeviceStreamState *value);
        
        DECLSPEC_XFGVIRT(IMFDeviceTransform, GetInputStreamPreferredState)
        HRESULT ( STDMETHODCALLTYPE *GetInputStreamPreferredState )( 
            IMFDeviceTransform2 * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamID,
            /* [annotation][out] */ 
            _Out_  DeviceStreamState *value,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFMediaType **ppMediaType);
        
        DECLSPEC_XFGVIRT(IMFDeviceTransform, FlushInputStream)
        HRESULT ( STDMETHODCALLTYPE *FlushInputStream )( 
            IMFDeviceTransform2 * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IMFDeviceTransform, FlushOutputStream)
        HRESULT ( STDMETHODCALLTYPE *FlushOutputStream )( 
            IMFDeviceTransform2 * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IMFDeviceTransform2, GetTransformAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetTransformAttributes )( 
            IMFDeviceTransform2 * This,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFAttributes **ppAttributes);
        
        END_INTERFACE
    } IMFDeviceTransform2Vtbl;

    interface IMFDeviceTransform2
    {
        CONST_VTBL struct IMFDeviceTransform2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFDeviceTransform2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFDeviceTransform2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFDeviceTransform2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFDeviceTransform2_InitializeTransform(This,pAttributes)	\
    ( (This)->lpVtbl -> InitializeTransform(This,pAttributes) ) 

#define IMFDeviceTransform2_GetInputAvailableType(This,dwInputStreamID,dwTypeIndex,pMediaType)	\
    ( (This)->lpVtbl -> GetInputAvailableType(This,dwInputStreamID,dwTypeIndex,pMediaType) ) 

#define IMFDeviceTransform2_GetInputCurrentType(This,dwInputStreamID,pMediaType)	\
    ( (This)->lpVtbl -> GetInputCurrentType(This,dwInputStreamID,pMediaType) ) 

#define IMFDeviceTransform2_GetInputStreamAttributes(This,dwInputStreamID,ppAttributes)	\
    ( (This)->lpVtbl -> GetInputStreamAttributes(This,dwInputStreamID,ppAttributes) ) 

#define IMFDeviceTransform2_GetOutputAvailableType(This,dwOutputStreamID,dwTypeIndex,pMediaType)	\
    ( (This)->lpVtbl -> GetOutputAvailableType(This,dwOutputStreamID,dwTypeIndex,pMediaType) ) 

#define IMFDeviceTransform2_GetOutputCurrentType(This,dwOutputStreamID,pMediaType)	\
    ( (This)->lpVtbl -> GetOutputCurrentType(This,dwOutputStreamID,pMediaType) ) 

#define IMFDeviceTransform2_GetOutputStreamAttributes(This,dwOutputStreamID,ppAttributes)	\
    ( (This)->lpVtbl -> GetOutputStreamAttributes(This,dwOutputStreamID,ppAttributes) ) 

#define IMFDeviceTransform2_GetStreamCount(This,pcInputStreams,pcOutputStreams)	\
    ( (This)->lpVtbl -> GetStreamCount(This,pcInputStreams,pcOutputStreams) ) 

#define IMFDeviceTransform2_GetStreamIDs(This,dwInputIDArraySize,pdwInputStreamIds,dwOutputIDArraySize,pdwOutputStreamIds)	\
    ( (This)->lpVtbl -> GetStreamIDs(This,dwInputIDArraySize,pdwInputStreamIds,dwOutputIDArraySize,pdwOutputStreamIds) ) 

#define IMFDeviceTransform2_ProcessEvent(This,dwInputStreamID,pEvent)	\
    ( (This)->lpVtbl -> ProcessEvent(This,dwInputStreamID,pEvent) ) 

#define IMFDeviceTransform2_ProcessInput(This,dwInputStreamID,pSample,dwFlags)	\
    ( (This)->lpVtbl -> ProcessInput(This,dwInputStreamID,pSample,dwFlags) ) 

#define IMFDeviceTransform2_ProcessMessage(This,eMessage,ulParam)	\
    ( (This)->lpVtbl -> ProcessMessage(This,eMessage,ulParam) ) 

#define IMFDeviceTransform2_ProcessOutput(This,dwFlags,cOutputBufferCount,pOutputSample,pdwStatus)	\
    ( (This)->lpVtbl -> ProcessOutput(This,dwFlags,cOutputBufferCount,pOutputSample,pdwStatus) ) 

#define IMFDeviceTransform2_SetInputStreamState(This,dwStreamID,pMediaType,value,dwFlags)	\
    ( (This)->lpVtbl -> SetInputStreamState(This,dwStreamID,pMediaType,value,dwFlags) ) 

#define IMFDeviceTransform2_GetInputStreamState(This,dwStreamID,value)	\
    ( (This)->lpVtbl -> GetInputStreamState(This,dwStreamID,value) ) 

#define IMFDeviceTransform2_SetOutputStreamState(This,dwStreamID,pMediaType,value,dwFlags)	\
    ( (This)->lpVtbl -> SetOutputStreamState(This,dwStreamID,pMediaType,value,dwFlags) ) 

#define IMFDeviceTransform2_GetOutputStreamState(This,dwStreamID,value)	\
    ( (This)->lpVtbl -> GetOutputStreamState(This,dwStreamID,value) ) 

#define IMFDeviceTransform2_GetInputStreamPreferredState(This,dwStreamID,value,ppMediaType)	\
    ( (This)->lpVtbl -> GetInputStreamPreferredState(This,dwStreamID,value,ppMediaType) ) 

#define IMFDeviceTransform2_FlushInputStream(This,dwStreamIndex,dwFlags)	\
    ( (This)->lpVtbl -> FlushInputStream(This,dwStreamIndex,dwFlags) ) 

#define IMFDeviceTransform2_FlushOutputStream(This,dwStreamIndex,dwFlags)	\
    ( (This)->lpVtbl -> FlushOutputStream(This,dwStreamIndex,dwFlags) ) 


#define IMFDeviceTransform2_GetTransformAttributes(This,ppAttributes)	\
    ( (This)->lpVtbl -> GetTransformAttributes(This,ppAttributes) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFDeviceTransform2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mftransform_0000_0003 */
/* [local] */ 

#endif // (WINVER >= _WIN32_WINNT_WINTHRESHOLD ) 
#if (NTDDI_VERSION >= _WIN32_WINNT_WIN10) 
EXTERN_GUID( MF_DMFT_FRAME_BUFFER_INFO,  0x396CE1C9, 0x67A9, 0x454C, 0x87, 0x97, 0x95, 0xA4, 0x57, 0x99, 0xD8, 0x04);


extern RPC_IF_HANDLE __MIDL_itf_mftransform_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mftransform_0000_0003_v0_0_s_ifspec;

#ifndef __IMFDeviceTransformCallback_INTERFACE_DEFINED__
#define __IMFDeviceTransformCallback_INTERFACE_DEFINED__

/* interface IMFDeviceTransformCallback */
/* [local][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMFDeviceTransformCallback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6D5CB646-29EC-41FB-8179-8C4C6D750811")
    IMFDeviceTransformCallback : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnBufferSent( 
            /* [annotation][in] */ 
            _In_  IMFAttributes *pCallbackAttributes,
            /* [annotation][in] */ 
            _In_  DWORD pinId) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFDeviceTransformCallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFDeviceTransformCallback * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFDeviceTransformCallback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFDeviceTransformCallback * This);
        
        DECLSPEC_XFGVIRT(IMFDeviceTransformCallback, OnBufferSent)
        HRESULT ( STDMETHODCALLTYPE *OnBufferSent )( 
            IMFDeviceTransformCallback * This,
            /* [annotation][in] */ 
            _In_  IMFAttributes *pCallbackAttributes,
            /* [annotation][in] */ 
            _In_  DWORD pinId);
        
        END_INTERFACE
    } IMFDeviceTransformCallbackVtbl;

    interface IMFDeviceTransformCallback
    {
        CONST_VTBL struct IMFDeviceTransformCallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFDeviceTransformCallback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFDeviceTransformCallback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFDeviceTransformCallback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFDeviceTransformCallback_OnBufferSent(This,pCallbackAttributes,pinId)	\
    ( (This)->lpVtbl -> OnBufferSent(This,pCallbackAttributes,pinId) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFDeviceTransformCallback_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mftransform_0000_0004 */
/* [local] */ 

#endif // (WINVER >= _WIN32_WINNT_WIN10 ) 
#if (WINVER >= _WIN32_WINNT_WIN8) 
#pragma region Application or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)
EXTERN_C const DECLSPEC_SELECTANY GUID MF_SA_REQUIRED_SAMPLE_COUNT_PROGRESSIVE = { 0xb172d58e, 0xfa77, 0x4e48, { 0x8d, 0x2a, 0x1d, 0xf2, 0xd8, 0x50, 0xea, 0xc2 } };
EXTERN_C const DECLSPEC_SELECTANY GUID MF_SA_MINIMUM_OUTPUT_SAMPLE_COUNT = { 0x851745d5, 0xc3d6, 0x476d, { 0x95, 0x27, 0x49, 0x8e, 0xf2, 0xd1, 0xd, 0x18 } };
EXTERN_C const DECLSPEC_SELECTANY GUID MF_SA_MINIMUM_OUTPUT_SAMPLE_COUNT_PROGRESSIVE = { 0xf5523a5, 0x1cb2, 0x47c5, { 0xa5, 0x50, 0x2e, 0xeb, 0x84, 0xb4, 0xd1, 0x4a } };
EXTERN_C const DECLSPEC_SELECTANY GUID MFT_SUPPORT_3DVIDEO = { 0x93f81b1, 0x4f2e, 0x4631, { 0x81, 0x68, 0x79, 0x34, 0x3, 0x2a, 0x1, 0xd3 } };
typedef 
enum _MF3DVideoOutputType
    {
        MF3DVideoOutputType_BaseView	= 0,
        MF3DVideoOutputType_Stereo	= 1
    } 	MF3DVideoOutputType;

EXTERN_C const DECLSPEC_SELECTANY GUID MF_ENABLE_3DVIDEO_OUTPUT = { 0xbdad7bca, 0xe5f, 0x4b10, { 0xab, 0x16, 0x26, 0xde, 0x38, 0x1b, 0x62, 0x93 } };
EXTERN_C const DECLSPEC_SELECTANY GUID MF_SA_D3D11_BINDFLAGS =       { 0xeacf97ad, 0x065c, 0x4408, { 0xbe, 0xe3, 0xfd, 0xcb, 0xfd, 0x12, 0x8b, 0xe2 } };
EXTERN_C const DECLSPEC_SELECTANY GUID MF_SA_D3D11_USAGE =           { 0xe85fe442, 0x2ca3, 0x486e, { 0xa9, 0xc7, 0x10, 0x9d, 0xda, 0x60, 0x98, 0x80 } };
EXTERN_C const DECLSPEC_SELECTANY GUID MF_SA_D3D11_AWARE =           { 0x206b4fc8, 0xfcf9, 0x4c51, { 0xaf, 0xe3, 0x97, 0x64, 0x36, 0x9e, 0x33, 0xa0 } };
EXTERN_C const DECLSPEC_SELECTANY GUID MF_SA_D3D11_SHARED =          { 0x7b8f32c3, 0x6d96, 0x4b89, { 0x92, 0x3, 0xdd, 0x38, 0xb6, 0x14, 0x14, 0xf3  } };
EXTERN_C const DECLSPEC_SELECTANY GUID MF_SA_D3D11_SHARED_WITHOUT_MUTEX = { 0x39dbd44d, 0x2e44, 0x4931, { 0xa4, 0xc8, 0x35, 0x2d, 0x3d, 0xc4, 0x21, 0x15 } };
EXTERN_C const DECLSPEC_SELECTANY GUID MF_SA_D3D11_ALLOW_DYNAMIC_YUV_TEXTURE = { 0xce06d49f, 0x613, 0x4b9d, { 0x86, 0xa6, 0xd8, 0xc4, 0xf9, 0xc1, 0x0, 0x75 } };
EXTERN_C const DECLSPEC_SELECTANY GUID MF_SA_D3D11_HW_PROTECTED =    { 0x3a8ba9d9, 0x92ca, 0x4307, { 0xa3, 0x91, 0x69, 0x99, 0xdb, 0xf3, 0xb6, 0xce } };
EXTERN_C const DECLSPEC_SELECTANY GUID MF_SA_BUFFERS_PER_SAMPLE =    { 0x873c5171, 0x1e3d, 0x4e25, { 0x98, 0x8d, 0xb4, 0x33, 0xce, 0x04, 0x19, 0x83 } };
EXTERN_C const DECLSPEC_SELECTANY GUID MF_SA_D3D11_ALLOCATE_DISPLAYABLE_RESOURCES = { 0xeeface6d, 0x2ea9, 0x4adf, { 0xbb, 0xdf, 0x7b, 0xbc, 0x48, 0x2a, 0x1b, 0x6d } };
EXTERN_C const DECLSPEC_SELECTANY GUID MFT_DECODER_EXPOSE_OUTPUT_TYPES_IN_NATIVE_ORDER =  { 0xef80833f, 0xf8fa, 0x44d9, { 0x80, 0xd8, 0x41, 0xed, 0x62, 0x32, 0x67, 0xc } };
EXTERN_C const DECLSPEC_SELECTANY GUID MFT_DECODER_QUALITY_MANAGEMENT_CUSTOM_CONTROL =  { 0xa24e30d7, 0xde25, 0x4558, { 0xbb, 0xfb, 0x71, 0x7, 0xa, 0x2d, 0x33, 0x2e } };
EXTERN_C const DECLSPEC_SELECTANY GUID MFT_DECODER_QUALITY_MANAGEMENT_RECOVERY_WITHOUT_ARTIFACTS =  { 0xd8980deb, 0xa48, 0x425f, { 0x86, 0x23, 0x61, 0x1d, 0xb4, 0x1d, 0x38, 0x10 } };
EXTERN_C const DECLSPEC_SELECTANY GUID MFT_DECODER_OPERATING_POINT = { 0xa1230334, 0x55d4, 0x4d97, { 0x82, 0xa7, 0x26, 0xd3, 0xe6, 0x45, 0x67, 0x25 } };
EXTERN_C const DECLSPEC_SELECTANY GUID MFT_DECODER_AUTOMATIC_SOFTWARE_FALLBACK = { 0x41f34f53, 0x1bf6, 0x49ed, { 0xb9, 0x5d, 0x2, 0xd2, 0xa1, 0xd7, 0x11, 0x5a } };
EXTERN_C const DECLSPEC_SELECTANY GUID MFT_REMUX_MARK_I_PICTURE_AS_CLEAN_POINT =  { 0x364e8f85, 0x3f2e, 0x436c, { 0xb2, 0xa2, 0x44, 0x40, 0xa0, 0x12, 0xa9, 0xe8} };
EXTERN_C const DECLSPEC_SELECTANY GUID MFT_DECODER_FINAL_VIDEO_RESOLUTION_HINT =    { 0xdc2f8496, 0x15c4, 0x407a, { 0xb6, 0xf0, 0x1b, 0x66, 0xab, 0x5f, 0xbf, 0x53 } };
EXTERN_C const DECLSPEC_SELECTANY GUID MFT_ENCODER_SUPPORTS_CONFIG_EVENT =  { 0x86a355ae, 0x3a77, 0x4ec4, { 0x9f, 0x31, 0x1, 0x14, 0x9a, 0x4e, 0x92, 0xde } };
EXTERN_C const DECLSPEC_SELECTANY GUID MFT_ENUM_HARDWARE_VENDOR_ID_Attribute =  { 0x3aecb0cc, 0x35b, 0x4bcc, { 0x81, 0x85, 0x2b, 0x8d, 0x55, 0x1e, 0xf3, 0xaf } };
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion
#endif // (WINVER >= _WIN32_WINNT_WIN8) 
#if (WINVER >= _WIN32_WINNT_WIN7) 
#pragma region Application or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)
EXTERN_C const DECLSPEC_SELECTANY GUID MF_TRANSFORM_ASYNC = { 0xf81a699a, 0x649a, 0x497d, { 0x8c, 0x73, 0x29, 0xf8, 0xfe, 0xd6, 0xad, 0x7a } };
EXTERN_C const DECLSPEC_SELECTANY GUID MF_TRANSFORM_ASYNC_UNLOCK = { 0xe5666d6b, 0x3422, 0x4eb6, { 0xa4, 0x21, 0xda, 0x7d, 0xb1, 0xf8, 0xe2, 0x7 } };
EXTERN_C const DECLSPEC_SELECTANY GUID MF_TRANSFORM_FLAGS_Attribute = { 0x9359bb7e, 0x6275, 0x46c4, { 0xa0, 0x25, 0x1c, 0x1, 0xe4, 0x5f, 0x1a, 0x86 } };
EXTERN_C const DECLSPEC_SELECTANY GUID MF_TRANSFORM_CATEGORY_Attribute = { 0xceabba49, 0x506d, 0x4757, { 0xa6, 0xff, 0x66, 0xc1, 0x84, 0x98, 0x7e, 0x4e } };
EXTERN_C const DECLSPEC_SELECTANY GUID MFT_TRANSFORM_CLSID_Attribute = { 0x6821c42b, 0x65a4, 0x4e82, { 0x99, 0xbc, 0x9a, 0x88, 0x20, 0x5e, 0xcd, 0xc } };
EXTERN_C const DECLSPEC_SELECTANY GUID MFT_INPUT_TYPES_Attributes = { 0x4276c9b1, 0x759d, 0x4bf3, { 0x9c, 0xd0, 0xd, 0x72, 0x3d, 0x13, 0x8f, 0x96 } };
EXTERN_C const DECLSPEC_SELECTANY GUID MFT_OUTPUT_TYPES_Attributes = { 0x8eae8cf3, 0xa44f, 0x4306, { 0xba, 0x5c, 0xbf, 0x5d, 0xda, 0x24, 0x28, 0x18 } };
EXTERN_C const DECLSPEC_SELECTANY GUID MFT_ENUM_HARDWARE_URL_Attribute = { 0x2fb866ac, 0xb078, 0x4942, { 0xab, 0x6c, 0x0, 0x3d, 0x5, 0xcd, 0xa6, 0x74 } };
EXTERN_C const DECLSPEC_SELECTANY GUID MFT_FRIENDLY_NAME_Attribute = { 0x314ffbae, 0x5b41, 0x4c95, { 0x9c, 0x19, 0x4e, 0x7d, 0x58, 0x6f, 0xac, 0xe3 } };
EXTERN_C const DECLSPEC_SELECTANY GUID MFT_CONNECTED_STREAM_ATTRIBUTE  = { 0x71eeb820, 0xa59f, 0x4de2, {0xbc, 0xec, 0x38, 0xdb, 0x1d, 0xd6, 0x11, 0xa4} };
EXTERN_C const DECLSPEC_SELECTANY GUID MFT_CONNECTED_TO_HW_STREAM = { 0x34e6e728, 0x6d6, 0x4491, { 0xa5, 0x53, 0x47, 0x95, 0x65, 0xd, 0xb9, 0x12 } };
EXTERN_C const DECLSPEC_SELECTANY GUID MFT_PREFERRED_OUTPUTTYPE_Attribute = { 0x7e700499, 0x396a, 0x49ee, { 0xb1, 0xb4, 0xf6, 0x28, 0x2, 0x1e, 0x8c, 0x9d } };
EXTERN_C const DECLSPEC_SELECTANY GUID MFT_PROCESS_LOCAL_Attribute = { 0x543186e4, 0x4649, 0x4e65, { 0xb5, 0x88, 0x4a, 0xa3, 0x52, 0xaf, 0xf3, 0x79 } };
EXTERN_C const DECLSPEC_SELECTANY GUID MFT_PREFERRED_ENCODER_PROFILE = { 0x53004909, 0x1ef5, 0x46d7, { 0xa1, 0x8e, 0x5a, 0x75, 0xf8, 0xb5, 0x90, 0x5f } };
EXTERN_C const DECLSPEC_SELECTANY GUID MFT_HW_TIMESTAMP_WITH_QPC_Attribute = { 0x8d030fb8, 0xcc43, 0x4258, { 0xa2, 0x2e, 0x92, 0x10, 0xbe, 0xf8, 0x9b, 0xe4 } };
EXTERN_C const DECLSPEC_SELECTANY GUID MFT_FIELDOFUSE_UNLOCK_Attribute = { 0x8ec2e9fd, 0x9148, 0x410d, { 0x83, 0x1e, 0x70, 0x24, 0x39, 0x46, 0x1a, 0x8e } };
EXTERN_C const DECLSPEC_SELECTANY GUID MFT_CODEC_MERIT_Attribute = { 0x88a7cb15, 0x7b07, 0x4a34, { 0x91, 0x28, 0xe6, 0x4c, 0x67, 0x3, 0xc4, 0xd3 } };
EXTERN_C const DECLSPEC_SELECTANY GUID MFT_ENUM_TRANSCODE_ONLY_ATTRIBUTE = { 0x111ea8cd, 0xb62a, 0x4bdb, { 0x89, 0xf6, 0x67, 0xff, 0xcd, 0xc2, 0x45, 0x8b } };
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
STDAPI
MFCreateTransformActivate(
    _Out_ IMFActivate** ppActivate
    );
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#endif // (WINVER >= _WIN32_WINNT_WIN7) 
#pragma region Track Error Improvements
#if (WINVER >= _WIN32_WINNT_WINTHRESHOLD) 
EXTERN_GUID( MFT_AUDIO_DECODER_DEGRADATION_INFO_ATTRIBUTE, 0x6c3386ad, 0xec20, 0x430d, 0xb2, 0xa5, 0x50, 0x5c, 0x71, 0x78, 0xd9, 0xc4);
typedef 
enum MFT_AUDIO_DECODER_DEGRADATION_REASON
    {
        MFT_AUDIO_DECODER_DEGRADATION_REASON_NONE	= 0,
        MFT_AUDIO_DECODER_DEGRADATION_REASON_LICENSING_REQUIREMENT	= 1
    } 	MFT_AUDIO_DECODER_DEGRADATION_REASON;

typedef 
enum MFT_AUDIO_DECODER_DEGRADATION_TYPE
    {
        MFT_AUDIO_DECODER_DEGRADATION_TYPE_NONE	= 0,
        MFT_AUDIO_DECODER_DEGRADATION_TYPE_DOWNMIX2CHANNEL	= 1,
        MFT_AUDIO_DECODER_DEGRADATION_TYPE_DOWNMIX6CHANNEL	= 2,
        MFT_AUDIO_DECODER_DEGRADATION_TYPE_DOWNMIX8CHANNEL	= 3
    } 	MFT_AUDIO_DECODER_DEGRADATION_TYPE;

typedef struct MFAudioDecoderDegradationInfo
    {
    MFT_AUDIO_DECODER_DEGRADATION_REASON eDegradationReason;
    MFT_AUDIO_DECODER_DEGRADATION_TYPE eType;
    } 	MFAudioDecoderDegradationInfo;

typedef struct _MFT_STREAM_STATE_PARAM
    {
    DWORD StreamId;
    MF_STREAM_STATE State;
    } 	MFT_STREAM_STATE_PARAM;

typedef struct _MFT_STREAM_STATE_PARAM *PMFT_STREAM_STATE_PARAM;

#endif // (WINVER >= _WIN32_WINNT_WINTHRESHOLD) 
#pragma endregion
#if (NTDDI_VERSION >= NTDDI_WIN10_VB) 
#pragma region Application or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)
EXTERN_C const DECLSPEC_SELECTANY GUID MFT_POLICY_SET_AWARE = { 0x5a633b19, 0xcc39, 0x4fa8, { 0x8c, 0xa5, 0x59, 0x98, 0x1b, 0x7a, 0x0, 0x18 } };
EXTERN_C const DECLSPEC_SELECTANY GUID MFT_USING_HARDWARE_DRM = { 0x34faa77d, 0xd79e, 0x4957, { 0xb8, 0xce, 0x36, 0x2b, 0x26, 0x84, 0x99, 0x6c } }; 
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion
#endif // (NTDDI_VERSION >= NTDDI_WIN10_VB) 


extern RPC_IF_HANDLE __MIDL_itf_mftransform_0000_0004_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mftransform_0000_0004_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif



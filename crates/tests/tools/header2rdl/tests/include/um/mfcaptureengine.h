

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

#ifndef __mfcaptureengine_h__
#define __mfcaptureengine_h__

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

#ifndef __IMFCaptureEngineOnEventCallback_FWD_DEFINED__
#define __IMFCaptureEngineOnEventCallback_FWD_DEFINED__
typedef interface IMFCaptureEngineOnEventCallback IMFCaptureEngineOnEventCallback;

#endif 	/* __IMFCaptureEngineOnEventCallback_FWD_DEFINED__ */


#ifndef __IMFCaptureEngineOnSampleCallback_FWD_DEFINED__
#define __IMFCaptureEngineOnSampleCallback_FWD_DEFINED__
typedef interface IMFCaptureEngineOnSampleCallback IMFCaptureEngineOnSampleCallback;

#endif 	/* __IMFCaptureEngineOnSampleCallback_FWD_DEFINED__ */


#ifndef __IMFCaptureSink_FWD_DEFINED__
#define __IMFCaptureSink_FWD_DEFINED__
typedef interface IMFCaptureSink IMFCaptureSink;

#endif 	/* __IMFCaptureSink_FWD_DEFINED__ */


#ifndef __IMFCaptureRecordSink_FWD_DEFINED__
#define __IMFCaptureRecordSink_FWD_DEFINED__
typedef interface IMFCaptureRecordSink IMFCaptureRecordSink;

#endif 	/* __IMFCaptureRecordSink_FWD_DEFINED__ */


#ifndef __IMFCapturePreviewSink_FWD_DEFINED__
#define __IMFCapturePreviewSink_FWD_DEFINED__
typedef interface IMFCapturePreviewSink IMFCapturePreviewSink;

#endif 	/* __IMFCapturePreviewSink_FWD_DEFINED__ */


#ifndef __IMFCapturePhotoSink_FWD_DEFINED__
#define __IMFCapturePhotoSink_FWD_DEFINED__
typedef interface IMFCapturePhotoSink IMFCapturePhotoSink;

#endif 	/* __IMFCapturePhotoSink_FWD_DEFINED__ */


#ifndef __IMFCaptureSource_FWD_DEFINED__
#define __IMFCaptureSource_FWD_DEFINED__
typedef interface IMFCaptureSource IMFCaptureSource;

#endif 	/* __IMFCaptureSource_FWD_DEFINED__ */


#ifndef __IMFCaptureEngine_FWD_DEFINED__
#define __IMFCaptureEngine_FWD_DEFINED__
typedef interface IMFCaptureEngine IMFCaptureEngine;

#endif 	/* __IMFCaptureEngine_FWD_DEFINED__ */


#ifndef __IMFCaptureEngineClassFactory_FWD_DEFINED__
#define __IMFCaptureEngineClassFactory_FWD_DEFINED__
typedef interface IMFCaptureEngineClassFactory IMFCaptureEngineClassFactory;

#endif 	/* __IMFCaptureEngineClassFactory_FWD_DEFINED__ */


#ifndef __IMFCaptureEngineOnSampleCallback2_FWD_DEFINED__
#define __IMFCaptureEngineOnSampleCallback2_FWD_DEFINED__
typedef interface IMFCaptureEngineOnSampleCallback2 IMFCaptureEngineOnSampleCallback2;

#endif 	/* __IMFCaptureEngineOnSampleCallback2_FWD_DEFINED__ */


#ifndef __IMFCaptureSink2_FWD_DEFINED__
#define __IMFCaptureSink2_FWD_DEFINED__
typedef interface IMFCaptureSink2 IMFCaptureSink2;

#endif 	/* __IMFCaptureSink2_FWD_DEFINED__ */


/* header files for imported files */
#include "mfobjects.h"
#include "mfidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_mfcaptureengine_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#if (WINVER >= _WIN32_WINNT_WIN7) 
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#ifndef _MFVideoNormalizedRect_
#define _MFVideoNormalizedRect_
typedef struct MFVideoNormalizedRect
    {
    float left;
    float top;
    float right;
    float bottom;
    } 	MFVideoNormalizedRect;

#endif
typedef 
enum MF_CAPTURE_ENGINE_DEVICE_TYPE
    {
        MF_CAPTURE_ENGINE_DEVICE_TYPE_AUDIO	= 0,
        MF_CAPTURE_ENGINE_DEVICE_TYPE_VIDEO	= 0x1
    } 	MF_CAPTURE_ENGINE_DEVICE_TYPE;

typedef 
enum MF_CAPTURE_ENGINE_SINK_TYPE
    {
        MF_CAPTURE_ENGINE_SINK_TYPE_RECORD	= 0,
        MF_CAPTURE_ENGINE_SINK_TYPE_PREVIEW	= 0x1,
        MF_CAPTURE_ENGINE_SINK_TYPE_PHOTO	= 0x2
    } 	MF_CAPTURE_ENGINE_SINK_TYPE;


enum __MIDL___MIDL_itf_mfcaptureengine_0000_0000_0001
    {
        MF_CAPTURE_ENGINE_PREFERRED_SOURCE_STREAM_FOR_VIDEO_PREVIEW	= 0xfffffffa,
        MF_CAPTURE_ENGINE_PREFERRED_SOURCE_STREAM_FOR_VIDEO_RECORD	= 0xfffffff9,
        MF_CAPTURE_ENGINE_PREFERRED_SOURCE_STREAM_FOR_PHOTO	= 0xfffffff8,
        MF_CAPTURE_ENGINE_PREFERRED_SOURCE_STREAM_FOR_AUDIO	= 0xfffffff7,
        MF_CAPTURE_ENGINE_PREFERRED_SOURCE_STREAM_FOR_METADATA	= 0xfffffff6,
        MF_CAPTURE_ENGINE_MEDIASOURCE	= 0xffffffff
    } ;
typedef 
enum MF_CAPTURE_ENGINE_STREAM_CATEGORY
    {
        MF_CAPTURE_ENGINE_STREAM_CATEGORY_VIDEO_PREVIEW	= 0,
        MF_CAPTURE_ENGINE_STREAM_CATEGORY_VIDEO_CAPTURE	= 0x1,
        MF_CAPTURE_ENGINE_STREAM_CATEGORY_PHOTO_INDEPENDENT	= 0x2,
        MF_CAPTURE_ENGINE_STREAM_CATEGORY_PHOTO_DEPENDENT	= 0x3,
        MF_CAPTURE_ENGINE_STREAM_CATEGORY_AUDIO	= 0x4,
        MF_CAPTURE_ENGINE_STREAM_CATEGORY_UNSUPPORTED	= 0x5,
        MF_CAPTURE_ENGINE_STREAM_CATEGORY_METADATA	= 0x6
    } 	MF_CAPTURE_ENGINE_STREAM_CATEGORY;

typedef 
enum MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE
    {
        MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE_OTHER	= 0,
        MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE_COMMUNICATIONS	= 1,
        MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE_MEDIA	= 2,
        MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE_GAMECHAT	= 3,
        MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE_SPEECH	= 4,
        MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE_FARFIELDSPEECH	= 5,
        MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE_UNIFORMSPEECH	= 6,
        MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE_VOICETYPING	= 7
    } 	MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE;

typedef 
enum MF_CAPTURE_ENGINE_AUDIO_PROCESSING_MODE
    {
        MF_CAPTURE_ENGINE_AUDIO_PROCESSING_DEFAULT	= 0,
        MF_CAPTURE_ENGINE_AUDIO_PROCESSING_RAW	= 1
    } 	MF_CAPTURE_ENGINE_AUDIO_PROCESSING_MODE;

EXTERN_GUID( MF_CAPTURE_ENGINE_INITIALIZED, 0x219992bc, 0xcf92, 0x4531, 0xa1, 0xae, 0x96, 0xe1, 0xe8, 0x86, 0xc8, 0xf1 );
EXTERN_GUID( MF_CAPTURE_ENGINE_PREVIEW_STARTED, 0xa416df21, 0xf9d3, 0x4a74, 0x99, 0x1b, 0xb8, 0x17, 0x29, 0x89, 0x52, 0xc4 );
EXTERN_GUID( MF_CAPTURE_ENGINE_PREVIEW_STOPPED, 0x13d5143c, 0x1edd, 0x4e50,0xa2, 0xef, 0x35, 0x0a, 0x47, 0x67, 0x80, 0x60 );
EXTERN_GUID( MF_CAPTURE_ENGINE_RECORD_STARTED, 0xac2b027b, 0xddf9, 0x48a0,0x89, 0xbe, 0x38, 0xab, 0x35, 0xef, 0x45, 0xc0 );
EXTERN_GUID( MF_CAPTURE_ENGINE_RECORD_STOPPED, 0x55e5200a, 0xf98f, 0x4c0d, 0xa9, 0xec, 0x9e, 0xb2, 0x5e, 0xd3, 0xd7, 0x73 );
EXTERN_GUID( MF_CAPTURE_ENGINE_PHOTO_TAKEN, 0x3c50c445, 0x7304, 0x48eb,0x86, 0x5d, 0xbb, 0xa1, 0x9b, 0xa3, 0xaf, 0x5c );
EXTERN_GUID( MF_CAPTURE_SOURCE_CURRENT_DEVICE_MEDIA_TYPE_SET, 0xe7e75e4c, 0x039c, 0x4410, 0x81, 0x5b, 0x87, 0x41, 0x30, 0x7b, 0x63, 0xaa );
EXTERN_GUID( MF_CAPTURE_ENGINE_ERROR, 0x46b89fc6, 0x33cc, 0x4399,0x9d, 0xad, 0x78, 0x4d, 0xe7, 0x7d, 0x58, 0x7c );
EXTERN_GUID( MF_CAPTURE_ENGINE_EFFECT_ADDED, 0xaa8dc7b5, 0xa048, 0x4e13, 0x8e, 0xbe, 0xf2, 0x3c, 0x46, 0xc8, 0x30, 0xc1 );
EXTERN_GUID( MF_CAPTURE_ENGINE_EFFECT_REMOVED, 0xc6e8db07, 0xfb09, 0x4a48, 0x89, 0xc6, 0xbf, 0x92, 0xa0, 0x42, 0x22, 0xc9);
EXTERN_GUID( MF_CAPTURE_ENGINE_ALL_EFFECTS_REMOVED, 0xfded7521, 0x8ed8, 0x431a, 0xa9, 0x6b, 0xf3, 0xe2, 0x56, 0x5e, 0x98, 0x1c);
EXTERN_GUID( MF_CAPTURE_SINK_PREPARED, 0x7BFCE257, 0x12B1, 0x4409, 0x8C, 0x34, 0xD4, 0x45, 0xDA, 0xAB, 0x75, 0x78);
EXTERN_GUID( MF_CAPTURE_ENGINE_OUTPUT_MEDIA_TYPE_SET, 0xcaaad994, 0x83ec, 0x45e9,0xa3, 0x0a, 0x1f, 0x20, 0xaa, 0xdb, 0x98, 0x31);
EXTERN_GUID(MF_CAPTURE_ENGINE_CAMERA_STREAM_BLOCKED, 0xA4209417, 0x8D39, 0x46F3, 0xB7, 0x59, 0x59, 0x12, 0x52, 0x8F, 0x42, 0x07);
EXTERN_GUID(MF_CAPTURE_ENGINE_CAMERA_STREAM_UNBLOCKED, 0x9BE9EEF0, 0xCDAF, 0x4717, 0x85, 0x64, 0x83, 0x4A, 0xAE, 0x66, 0x41, 0x5C);
EXTERN_GUID( MF_CAPTURE_ENGINE_D3D_MANAGER, 0x76e25e7b, 0xd595, 0x4283, 0x96, 0x2c, 0xc5, 0x94, 0xaf, 0xd7, 0x8d, 0xdf);
EXTERN_GUID( MF_CAPTURE_ENGINE_RECORD_SINK_VIDEO_MAX_UNPROCESSED_SAMPLES, 0xb467f705, 0x7913, 0x4894, 0x9d, 0x42, 0xa2, 0x15, 0xfe, 0xa2, 0x3d, 0xa9);
EXTERN_GUID( MF_CAPTURE_ENGINE_RECORD_SINK_AUDIO_MAX_UNPROCESSED_SAMPLES, 0x1cddb141, 0xa7f4, 0x4d58, 0x98, 0x96, 0x4d, 0x15, 0xa5, 0x3c, 0x4e, 0xfe);
EXTERN_GUID(MF_CAPTURE_ENGINE_RECORD_SINK_VIDEO_MAX_PROCESSED_SAMPLES, 0xe7b4a49e, 0x382c, 0x4aef, 0xa9, 0x46, 0xae, 0xd5, 0x49, 0xb, 0x71, 0x11);
EXTERN_GUID(MF_CAPTURE_ENGINE_RECORD_SINK_AUDIO_MAX_PROCESSED_SAMPLES, 0x9896e12a, 0xf707, 0x4500, 0xb6, 0xbd, 0xdb, 0x8e, 0xb8, 0x10, 0xb5, 0xf);
EXTERN_GUID( MF_CAPTURE_ENGINE_USE_AUDIO_DEVICE_ONLY, 0x1c8077da, 0x8466, 0x4dc4, 0x8b, 0x8e, 0x27, 0x6b, 0x3f, 0x85, 0x92, 0x3b);
EXTERN_GUID( MF_CAPTURE_ENGINE_USE_VIDEO_DEVICE_ONLY, 0x7e025171, 0xcf32, 0x4f2e, 0x8f, 0x19, 0x41, 0x5, 0x77, 0xb7, 0x3a, 0x66);
EXTERN_GUID(MF_CAPTURE_ENGINE_DISABLE_HARDWARE_TRANSFORMS, 0xb7c42a6b, 0x3207,  0x4495, 0xb4, 0xe7, 0x81, 0xf9, 0xc3, 0x5d, 0x59, 0x91);
EXTERN_GUID(MF_CAPTURE_ENGINE_DISABLE_DXVA, 0xf9818862,  0x179d, 0x433f, 0xa3, 0x2f, 0x74, 0xcb, 0xcf, 0x74, 0x46, 0x6d);
EXTERN_GUID(MF_CAPTURE_ENGINE_MEDIASOURCE_CONFIG, 0xbc6989d2, 0x0fc1, 0x46e1, 0xa7, 0x4f, 0xef, 0xd3, 0x6b, 0xc7, 0x88, 0xde);
EXTERN_GUID(MF_CAPTURE_ENGINE_DECODER_MFT_FIELDOFUSE_UNLOCK_Attribute, 0x2b8ad2e8, 0x7acb, 0x4321, 0xa6, 0x06, 0x32, 0x5c, 0x42, 0x49, 0xf4, 0xfc);
EXTERN_GUID(MF_CAPTURE_ENGINE_ENCODER_MFT_FIELDOFUSE_UNLOCK_Attribute, 0x54c63a00, 0x78d5, 0x422f, 0xaa, 0x3e, 0x5e, 0x99, 0xac, 0x64, 0x92, 0x69);
EXTERN_GUID(MF_CAPTURE_ENGINE_ENABLE_CAMERA_STREAMSTATE_NOTIFICATION, 0x4C808E9D, 0xAAED, 0x4713, 0x90, 0xFB, 0xCB, 0x24, 0x06, 0x4A, 0xB8, 0xDA);
EXTERN_GUID(MF_CAPTURE_ENGINE_MEDIA_CATEGORY, 0x8e3f5bd5, 0xdbbf, 0x42f0, 0x85, 0x42, 0xd0, 0x7a, 0x39, 0x71, 0x76, 0x2a);
EXTERN_GUID(MF_CAPTURE_ENGINE_AUDIO_PROCESSING, 0x10f1be5e, 0x7e11, 0x410b, 0x97, 0x3d, 0xf4, 0xb6, 0x10, 0x90, 0x0, 0xfe);
EXTERN_GUID( MF_CAPTURE_ENGINE_EVENT_GENERATOR_GUID, 0xabfa8ad5, 0xfc6d, 0x4911, 0x87, 0xe0, 0x96, 0x19, 0x45, 0xf8, 0xf7, 0xce);
EXTERN_GUID( MF_CAPTURE_ENGINE_EVENT_STREAM_INDEX, 0x82697f44, 0xb1cf, 0x42eb, 0x97, 0x53, 0xf8, 0x6d, 0x64, 0x9c, 0x88, 0x65);
EXTERN_GUID(MF_CAPTURE_ENGINE_SELECTEDCAMERAPROFILE, 0x03160B7E, 0x1C6F, 0x4DB2, 0xAD, 0x56, 0xA7, 0xC4, 0x30, 0xF8, 0x23, 0x92);
EXTERN_GUID(MF_CAPTURE_ENGINE_SELECTEDCAMERAPROFILE_INDEX, 0x3CE88613, 0x2214, 0x46C3, 0xB4, 0x17, 0x82, 0xF8, 0xA3, 0x13, 0xC9, 0xC3);


extern RPC_IF_HANDLE __MIDL_itf_mfcaptureengine_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfcaptureengine_0000_0000_v0_0_s_ifspec;

#ifndef __IMFCaptureEngineOnEventCallback_INTERFACE_DEFINED__
#define __IMFCaptureEngineOnEventCallback_INTERFACE_DEFINED__

/* interface IMFCaptureEngineOnEventCallback */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFCaptureEngineOnEventCallback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("aeda51c0-9025-4983-9012-de597b88b089")
    IMFCaptureEngineOnEventCallback : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnEvent( 
            /* [annotation][in] */ 
            _In_  IMFMediaEvent *pEvent) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFCaptureEngineOnEventCallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFCaptureEngineOnEventCallback * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFCaptureEngineOnEventCallback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFCaptureEngineOnEventCallback * This);
        
        DECLSPEC_XFGVIRT(IMFCaptureEngineOnEventCallback, OnEvent)
        HRESULT ( STDMETHODCALLTYPE *OnEvent )( 
            IMFCaptureEngineOnEventCallback * This,
            /* [annotation][in] */ 
            _In_  IMFMediaEvent *pEvent);
        
        END_INTERFACE
    } IMFCaptureEngineOnEventCallbackVtbl;

    interface IMFCaptureEngineOnEventCallback
    {
        CONST_VTBL struct IMFCaptureEngineOnEventCallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFCaptureEngineOnEventCallback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFCaptureEngineOnEventCallback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFCaptureEngineOnEventCallback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFCaptureEngineOnEventCallback_OnEvent(This,pEvent)	\
    ( (This)->lpVtbl -> OnEvent(This,pEvent) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFCaptureEngineOnEventCallback_INTERFACE_DEFINED__ */


#ifndef __IMFCaptureEngineOnSampleCallback_INTERFACE_DEFINED__
#define __IMFCaptureEngineOnSampleCallback_INTERFACE_DEFINED__

/* interface IMFCaptureEngineOnSampleCallback */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFCaptureEngineOnSampleCallback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("52150b82-ab39-4467-980f-e48bf0822ecd")
    IMFCaptureEngineOnSampleCallback : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnSample( 
            /* [annotation][in] */ 
            _In_opt_  IMFSample *pSample) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFCaptureEngineOnSampleCallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFCaptureEngineOnSampleCallback * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFCaptureEngineOnSampleCallback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFCaptureEngineOnSampleCallback * This);
        
        DECLSPEC_XFGVIRT(IMFCaptureEngineOnSampleCallback, OnSample)
        HRESULT ( STDMETHODCALLTYPE *OnSample )( 
            IMFCaptureEngineOnSampleCallback * This,
            /* [annotation][in] */ 
            _In_opt_  IMFSample *pSample);
        
        END_INTERFACE
    } IMFCaptureEngineOnSampleCallbackVtbl;

    interface IMFCaptureEngineOnSampleCallback
    {
        CONST_VTBL struct IMFCaptureEngineOnSampleCallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFCaptureEngineOnSampleCallback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFCaptureEngineOnSampleCallback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFCaptureEngineOnSampleCallback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFCaptureEngineOnSampleCallback_OnSample(This,pSample)	\
    ( (This)->lpVtbl -> OnSample(This,pSample) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFCaptureEngineOnSampleCallback_INTERFACE_DEFINED__ */


#ifndef __IMFCaptureSink_INTERFACE_DEFINED__
#define __IMFCaptureSink_INTERFACE_DEFINED__

/* interface IMFCaptureSink */
/* [local][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMFCaptureSink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("72d6135b-35e9-412c-b926-fd5265f2a885")
    IMFCaptureSink : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetOutputMediaType( 
            /* [annotation][in] */ 
            _In_  DWORD dwSinkStreamIndex,
            /* [annotation][out] */ 
            _Out_opt_  IMFMediaType **ppMediaType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetService( 
            /* [annotation][in] */ 
            _In_  DWORD dwSinkStreamIndex,
            /* [annotation][in] */ 
            _In_  REFGUID rguidService,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][out] */ 
            _Out_opt_  IUnknown **ppUnknown) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddStream( 
            /* [annotation][in] */ 
            _In_  DWORD dwSourceStreamIndex,
            /* [annotation][in] */ 
            _In_  IMFMediaType *pMediaType,
            /* [annotation][in] */ 
            _In_opt_  IMFAttributes *pAttributes,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *pdwSinkStreamIndex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Prepare( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveAllStreams( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFCaptureSinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFCaptureSink * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFCaptureSink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFCaptureSink * This);
        
        DECLSPEC_XFGVIRT(IMFCaptureSink, GetOutputMediaType)
        HRESULT ( STDMETHODCALLTYPE *GetOutputMediaType )( 
            IMFCaptureSink * This,
            /* [annotation][in] */ 
            _In_  DWORD dwSinkStreamIndex,
            /* [annotation][out] */ 
            _Out_opt_  IMFMediaType **ppMediaType);
        
        DECLSPEC_XFGVIRT(IMFCaptureSink, GetService)
        HRESULT ( STDMETHODCALLTYPE *GetService )( 
            IMFCaptureSink * This,
            /* [annotation][in] */ 
            _In_  DWORD dwSinkStreamIndex,
            /* [annotation][in] */ 
            _In_  REFGUID rguidService,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][out] */ 
            _Out_opt_  IUnknown **ppUnknown);
        
        DECLSPEC_XFGVIRT(IMFCaptureSink, AddStream)
        HRESULT ( STDMETHODCALLTYPE *AddStream )( 
            IMFCaptureSink * This,
            /* [annotation][in] */ 
            _In_  DWORD dwSourceStreamIndex,
            /* [annotation][in] */ 
            _In_  IMFMediaType *pMediaType,
            /* [annotation][in] */ 
            _In_opt_  IMFAttributes *pAttributes,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *pdwSinkStreamIndex);
        
        DECLSPEC_XFGVIRT(IMFCaptureSink, Prepare)
        HRESULT ( STDMETHODCALLTYPE *Prepare )( 
            IMFCaptureSink * This);
        
        DECLSPEC_XFGVIRT(IMFCaptureSink, RemoveAllStreams)
        HRESULT ( STDMETHODCALLTYPE *RemoveAllStreams )( 
            IMFCaptureSink * This);
        
        END_INTERFACE
    } IMFCaptureSinkVtbl;

    interface IMFCaptureSink
    {
        CONST_VTBL struct IMFCaptureSinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFCaptureSink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFCaptureSink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFCaptureSink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFCaptureSink_GetOutputMediaType(This,dwSinkStreamIndex,ppMediaType)	\
    ( (This)->lpVtbl -> GetOutputMediaType(This,dwSinkStreamIndex,ppMediaType) ) 

#define IMFCaptureSink_GetService(This,dwSinkStreamIndex,rguidService,riid,ppUnknown)	\
    ( (This)->lpVtbl -> GetService(This,dwSinkStreamIndex,rguidService,riid,ppUnknown) ) 

#define IMFCaptureSink_AddStream(This,dwSourceStreamIndex,pMediaType,pAttributes,pdwSinkStreamIndex)	\
    ( (This)->lpVtbl -> AddStream(This,dwSourceStreamIndex,pMediaType,pAttributes,pdwSinkStreamIndex) ) 

#define IMFCaptureSink_Prepare(This)	\
    ( (This)->lpVtbl -> Prepare(This) ) 

#define IMFCaptureSink_RemoveAllStreams(This)	\
    ( (This)->lpVtbl -> RemoveAllStreams(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFCaptureSink_INTERFACE_DEFINED__ */


#ifndef __IMFCaptureRecordSink_INTERFACE_DEFINED__
#define __IMFCaptureRecordSink_INTERFACE_DEFINED__

/* interface IMFCaptureRecordSink */
/* [local][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMFCaptureRecordSink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3323b55a-f92a-4fe2-8edc-e9bfc0634d77")
    IMFCaptureRecordSink : public IMFCaptureSink
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetOutputByteStream( 
            /* [annotation][in] */ 
            _In_  IMFByteStream *pByteStream,
            /* [annotation][in] */ 
            _In_  REFGUID guidContainerType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetOutputFileName( 
            /* [annotation][in] */ 
            _In_  LPCWSTR fileName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSampleCallback( 
            /* [annotation][in] */ 
            _In_  DWORD dwStreamSinkIndex,
            /* [annotation][in] */ 
            _In_  IMFCaptureEngineOnSampleCallback *pCallback) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetCustomSink( 
            /* [annotation][in] */ 
            _In_  IMFMediaSink *pMediaSink) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRotation( 
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwRotationValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetRotation( 
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  DWORD dwRotationValue) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFCaptureRecordSinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFCaptureRecordSink * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFCaptureRecordSink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFCaptureRecordSink * This);
        
        DECLSPEC_XFGVIRT(IMFCaptureSink, GetOutputMediaType)
        HRESULT ( STDMETHODCALLTYPE *GetOutputMediaType )( 
            IMFCaptureRecordSink * This,
            /* [annotation][in] */ 
            _In_  DWORD dwSinkStreamIndex,
            /* [annotation][out] */ 
            _Out_opt_  IMFMediaType **ppMediaType);
        
        DECLSPEC_XFGVIRT(IMFCaptureSink, GetService)
        HRESULT ( STDMETHODCALLTYPE *GetService )( 
            IMFCaptureRecordSink * This,
            /* [annotation][in] */ 
            _In_  DWORD dwSinkStreamIndex,
            /* [annotation][in] */ 
            _In_  REFGUID rguidService,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][out] */ 
            _Out_opt_  IUnknown **ppUnknown);
        
        DECLSPEC_XFGVIRT(IMFCaptureSink, AddStream)
        HRESULT ( STDMETHODCALLTYPE *AddStream )( 
            IMFCaptureRecordSink * This,
            /* [annotation][in] */ 
            _In_  DWORD dwSourceStreamIndex,
            /* [annotation][in] */ 
            _In_  IMFMediaType *pMediaType,
            /* [annotation][in] */ 
            _In_opt_  IMFAttributes *pAttributes,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *pdwSinkStreamIndex);
        
        DECLSPEC_XFGVIRT(IMFCaptureSink, Prepare)
        HRESULT ( STDMETHODCALLTYPE *Prepare )( 
            IMFCaptureRecordSink * This);
        
        DECLSPEC_XFGVIRT(IMFCaptureSink, RemoveAllStreams)
        HRESULT ( STDMETHODCALLTYPE *RemoveAllStreams )( 
            IMFCaptureRecordSink * This);
        
        DECLSPEC_XFGVIRT(IMFCaptureRecordSink, SetOutputByteStream)
        HRESULT ( STDMETHODCALLTYPE *SetOutputByteStream )( 
            IMFCaptureRecordSink * This,
            /* [annotation][in] */ 
            _In_  IMFByteStream *pByteStream,
            /* [annotation][in] */ 
            _In_  REFGUID guidContainerType);
        
        DECLSPEC_XFGVIRT(IMFCaptureRecordSink, SetOutputFileName)
        HRESULT ( STDMETHODCALLTYPE *SetOutputFileName )( 
            IMFCaptureRecordSink * This,
            /* [annotation][in] */ 
            _In_  LPCWSTR fileName);
        
        DECLSPEC_XFGVIRT(IMFCaptureRecordSink, SetSampleCallback)
        HRESULT ( STDMETHODCALLTYPE *SetSampleCallback )( 
            IMFCaptureRecordSink * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamSinkIndex,
            /* [annotation][in] */ 
            _In_  IMFCaptureEngineOnSampleCallback *pCallback);
        
        DECLSPEC_XFGVIRT(IMFCaptureRecordSink, SetCustomSink)
        HRESULT ( STDMETHODCALLTYPE *SetCustomSink )( 
            IMFCaptureRecordSink * This,
            /* [annotation][in] */ 
            _In_  IMFMediaSink *pMediaSink);
        
        DECLSPEC_XFGVIRT(IMFCaptureRecordSink, GetRotation)
        HRESULT ( STDMETHODCALLTYPE *GetRotation )( 
            IMFCaptureRecordSink * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwRotationValue);
        
        DECLSPEC_XFGVIRT(IMFCaptureRecordSink, SetRotation)
        HRESULT ( STDMETHODCALLTYPE *SetRotation )( 
            IMFCaptureRecordSink * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  DWORD dwRotationValue);
        
        END_INTERFACE
    } IMFCaptureRecordSinkVtbl;

    interface IMFCaptureRecordSink
    {
        CONST_VTBL struct IMFCaptureRecordSinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFCaptureRecordSink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFCaptureRecordSink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFCaptureRecordSink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFCaptureRecordSink_GetOutputMediaType(This,dwSinkStreamIndex,ppMediaType)	\
    ( (This)->lpVtbl -> GetOutputMediaType(This,dwSinkStreamIndex,ppMediaType) ) 

#define IMFCaptureRecordSink_GetService(This,dwSinkStreamIndex,rguidService,riid,ppUnknown)	\
    ( (This)->lpVtbl -> GetService(This,dwSinkStreamIndex,rguidService,riid,ppUnknown) ) 

#define IMFCaptureRecordSink_AddStream(This,dwSourceStreamIndex,pMediaType,pAttributes,pdwSinkStreamIndex)	\
    ( (This)->lpVtbl -> AddStream(This,dwSourceStreamIndex,pMediaType,pAttributes,pdwSinkStreamIndex) ) 

#define IMFCaptureRecordSink_Prepare(This)	\
    ( (This)->lpVtbl -> Prepare(This) ) 

#define IMFCaptureRecordSink_RemoveAllStreams(This)	\
    ( (This)->lpVtbl -> RemoveAllStreams(This) ) 


#define IMFCaptureRecordSink_SetOutputByteStream(This,pByteStream,guidContainerType)	\
    ( (This)->lpVtbl -> SetOutputByteStream(This,pByteStream,guidContainerType) ) 

#define IMFCaptureRecordSink_SetOutputFileName(This,fileName)	\
    ( (This)->lpVtbl -> SetOutputFileName(This,fileName) ) 

#define IMFCaptureRecordSink_SetSampleCallback(This,dwStreamSinkIndex,pCallback)	\
    ( (This)->lpVtbl -> SetSampleCallback(This,dwStreamSinkIndex,pCallback) ) 

#define IMFCaptureRecordSink_SetCustomSink(This,pMediaSink)	\
    ( (This)->lpVtbl -> SetCustomSink(This,pMediaSink) ) 

#define IMFCaptureRecordSink_GetRotation(This,dwStreamIndex,pdwRotationValue)	\
    ( (This)->lpVtbl -> GetRotation(This,dwStreamIndex,pdwRotationValue) ) 

#define IMFCaptureRecordSink_SetRotation(This,dwStreamIndex,dwRotationValue)	\
    ( (This)->lpVtbl -> SetRotation(This,dwStreamIndex,dwRotationValue) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFCaptureRecordSink_INTERFACE_DEFINED__ */


#ifndef __IMFCapturePreviewSink_INTERFACE_DEFINED__
#define __IMFCapturePreviewSink_INTERFACE_DEFINED__

/* interface IMFCapturePreviewSink */
/* [local][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMFCapturePreviewSink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("77346cfd-5b49-4d73-ace0-5b52a859f2e0")
    IMFCapturePreviewSink : public IMFCaptureSink
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetRenderHandle( 
            /* [annotation][in] */ 
            _In_  HANDLE handle) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetRenderSurface( 
            /* [annotation][in] */ 
            _In_  IUnknown *pSurface) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UpdateVideo( 
            /* [annotation][in] */ 
            _In_opt_  const MFVideoNormalizedRect *pSrc,
            /* [annotation][in] */ 
            _In_opt_  const RECT *pDst,
            /* [annotation][in] */ 
            _In_opt_  const COLORREF *pBorderClr) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSampleCallback( 
            /* [annotation][in] */ 
            _In_  DWORD dwStreamSinkIndex,
            /* [annotation][in] */ 
            _In_  IMFCaptureEngineOnSampleCallback *pCallback) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMirrorState( 
            /* [annotation][out] */ 
            _Out_  BOOL *pfMirrorState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetMirrorState( 
            /* [annotation][in] */ 
            _In_  BOOL fMirrorState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRotation( 
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwRotationValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetRotation( 
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  DWORD dwRotationValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetCustomSink( 
            /* [annotation][in] */ 
            _In_  IMFMediaSink *pMediaSink) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFCapturePreviewSinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFCapturePreviewSink * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFCapturePreviewSink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFCapturePreviewSink * This);
        
        DECLSPEC_XFGVIRT(IMFCaptureSink, GetOutputMediaType)
        HRESULT ( STDMETHODCALLTYPE *GetOutputMediaType )( 
            IMFCapturePreviewSink * This,
            /* [annotation][in] */ 
            _In_  DWORD dwSinkStreamIndex,
            /* [annotation][out] */ 
            _Out_opt_  IMFMediaType **ppMediaType);
        
        DECLSPEC_XFGVIRT(IMFCaptureSink, GetService)
        HRESULT ( STDMETHODCALLTYPE *GetService )( 
            IMFCapturePreviewSink * This,
            /* [annotation][in] */ 
            _In_  DWORD dwSinkStreamIndex,
            /* [annotation][in] */ 
            _In_  REFGUID rguidService,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][out] */ 
            _Out_opt_  IUnknown **ppUnknown);
        
        DECLSPEC_XFGVIRT(IMFCaptureSink, AddStream)
        HRESULT ( STDMETHODCALLTYPE *AddStream )( 
            IMFCapturePreviewSink * This,
            /* [annotation][in] */ 
            _In_  DWORD dwSourceStreamIndex,
            /* [annotation][in] */ 
            _In_  IMFMediaType *pMediaType,
            /* [annotation][in] */ 
            _In_opt_  IMFAttributes *pAttributes,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *pdwSinkStreamIndex);
        
        DECLSPEC_XFGVIRT(IMFCaptureSink, Prepare)
        HRESULT ( STDMETHODCALLTYPE *Prepare )( 
            IMFCapturePreviewSink * This);
        
        DECLSPEC_XFGVIRT(IMFCaptureSink, RemoveAllStreams)
        HRESULT ( STDMETHODCALLTYPE *RemoveAllStreams )( 
            IMFCapturePreviewSink * This);
        
        DECLSPEC_XFGVIRT(IMFCapturePreviewSink, SetRenderHandle)
        HRESULT ( STDMETHODCALLTYPE *SetRenderHandle )( 
            IMFCapturePreviewSink * This,
            /* [annotation][in] */ 
            _In_  HANDLE handle);
        
        DECLSPEC_XFGVIRT(IMFCapturePreviewSink, SetRenderSurface)
        HRESULT ( STDMETHODCALLTYPE *SetRenderSurface )( 
            IMFCapturePreviewSink * This,
            /* [annotation][in] */ 
            _In_  IUnknown *pSurface);
        
        DECLSPEC_XFGVIRT(IMFCapturePreviewSink, UpdateVideo)
        HRESULT ( STDMETHODCALLTYPE *UpdateVideo )( 
            IMFCapturePreviewSink * This,
            /* [annotation][in] */ 
            _In_opt_  const MFVideoNormalizedRect *pSrc,
            /* [annotation][in] */ 
            _In_opt_  const RECT *pDst,
            /* [annotation][in] */ 
            _In_opt_  const COLORREF *pBorderClr);
        
        DECLSPEC_XFGVIRT(IMFCapturePreviewSink, SetSampleCallback)
        HRESULT ( STDMETHODCALLTYPE *SetSampleCallback )( 
            IMFCapturePreviewSink * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamSinkIndex,
            /* [annotation][in] */ 
            _In_  IMFCaptureEngineOnSampleCallback *pCallback);
        
        DECLSPEC_XFGVIRT(IMFCapturePreviewSink, GetMirrorState)
        HRESULT ( STDMETHODCALLTYPE *GetMirrorState )( 
            IMFCapturePreviewSink * This,
            /* [annotation][out] */ 
            _Out_  BOOL *pfMirrorState);
        
        DECLSPEC_XFGVIRT(IMFCapturePreviewSink, SetMirrorState)
        HRESULT ( STDMETHODCALLTYPE *SetMirrorState )( 
            IMFCapturePreviewSink * This,
            /* [annotation][in] */ 
            _In_  BOOL fMirrorState);
        
        DECLSPEC_XFGVIRT(IMFCapturePreviewSink, GetRotation)
        HRESULT ( STDMETHODCALLTYPE *GetRotation )( 
            IMFCapturePreviewSink * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwRotationValue);
        
        DECLSPEC_XFGVIRT(IMFCapturePreviewSink, SetRotation)
        HRESULT ( STDMETHODCALLTYPE *SetRotation )( 
            IMFCapturePreviewSink * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  DWORD dwRotationValue);
        
        DECLSPEC_XFGVIRT(IMFCapturePreviewSink, SetCustomSink)
        HRESULT ( STDMETHODCALLTYPE *SetCustomSink )( 
            IMFCapturePreviewSink * This,
            /* [annotation][in] */ 
            _In_  IMFMediaSink *pMediaSink);
        
        END_INTERFACE
    } IMFCapturePreviewSinkVtbl;

    interface IMFCapturePreviewSink
    {
        CONST_VTBL struct IMFCapturePreviewSinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFCapturePreviewSink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFCapturePreviewSink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFCapturePreviewSink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFCapturePreviewSink_GetOutputMediaType(This,dwSinkStreamIndex,ppMediaType)	\
    ( (This)->lpVtbl -> GetOutputMediaType(This,dwSinkStreamIndex,ppMediaType) ) 

#define IMFCapturePreviewSink_GetService(This,dwSinkStreamIndex,rguidService,riid,ppUnknown)	\
    ( (This)->lpVtbl -> GetService(This,dwSinkStreamIndex,rguidService,riid,ppUnknown) ) 

#define IMFCapturePreviewSink_AddStream(This,dwSourceStreamIndex,pMediaType,pAttributes,pdwSinkStreamIndex)	\
    ( (This)->lpVtbl -> AddStream(This,dwSourceStreamIndex,pMediaType,pAttributes,pdwSinkStreamIndex) ) 

#define IMFCapturePreviewSink_Prepare(This)	\
    ( (This)->lpVtbl -> Prepare(This) ) 

#define IMFCapturePreviewSink_RemoveAllStreams(This)	\
    ( (This)->lpVtbl -> RemoveAllStreams(This) ) 


#define IMFCapturePreviewSink_SetRenderHandle(This,handle)	\
    ( (This)->lpVtbl -> SetRenderHandle(This,handle) ) 

#define IMFCapturePreviewSink_SetRenderSurface(This,pSurface)	\
    ( (This)->lpVtbl -> SetRenderSurface(This,pSurface) ) 

#define IMFCapturePreviewSink_UpdateVideo(This,pSrc,pDst,pBorderClr)	\
    ( (This)->lpVtbl -> UpdateVideo(This,pSrc,pDst,pBorderClr) ) 

#define IMFCapturePreviewSink_SetSampleCallback(This,dwStreamSinkIndex,pCallback)	\
    ( (This)->lpVtbl -> SetSampleCallback(This,dwStreamSinkIndex,pCallback) ) 

#define IMFCapturePreviewSink_GetMirrorState(This,pfMirrorState)	\
    ( (This)->lpVtbl -> GetMirrorState(This,pfMirrorState) ) 

#define IMFCapturePreviewSink_SetMirrorState(This,fMirrorState)	\
    ( (This)->lpVtbl -> SetMirrorState(This,fMirrorState) ) 

#define IMFCapturePreviewSink_GetRotation(This,dwStreamIndex,pdwRotationValue)	\
    ( (This)->lpVtbl -> GetRotation(This,dwStreamIndex,pdwRotationValue) ) 

#define IMFCapturePreviewSink_SetRotation(This,dwStreamIndex,dwRotationValue)	\
    ( (This)->lpVtbl -> SetRotation(This,dwStreamIndex,dwRotationValue) ) 

#define IMFCapturePreviewSink_SetCustomSink(This,pMediaSink)	\
    ( (This)->lpVtbl -> SetCustomSink(This,pMediaSink) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFCapturePreviewSink_INTERFACE_DEFINED__ */


#ifndef __IMFCapturePhotoSink_INTERFACE_DEFINED__
#define __IMFCapturePhotoSink_INTERFACE_DEFINED__

/* interface IMFCapturePhotoSink */
/* [local][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMFCapturePhotoSink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("d2d43cc8-48bb-4aa7-95db-10c06977e777")
    IMFCapturePhotoSink : public IMFCaptureSink
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetOutputFileName( 
            /* [annotation][in] */ 
            _In_  LPCWSTR fileName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSampleCallback( 
            /* [annotation][in] */ 
            _In_  IMFCaptureEngineOnSampleCallback *pCallback) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetOutputByteStream( 
            /* [annotation][in] */ 
            _In_  IMFByteStream *pByteStream) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFCapturePhotoSinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFCapturePhotoSink * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFCapturePhotoSink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFCapturePhotoSink * This);
        
        DECLSPEC_XFGVIRT(IMFCaptureSink, GetOutputMediaType)
        HRESULT ( STDMETHODCALLTYPE *GetOutputMediaType )( 
            IMFCapturePhotoSink * This,
            /* [annotation][in] */ 
            _In_  DWORD dwSinkStreamIndex,
            /* [annotation][out] */ 
            _Out_opt_  IMFMediaType **ppMediaType);
        
        DECLSPEC_XFGVIRT(IMFCaptureSink, GetService)
        HRESULT ( STDMETHODCALLTYPE *GetService )( 
            IMFCapturePhotoSink * This,
            /* [annotation][in] */ 
            _In_  DWORD dwSinkStreamIndex,
            /* [annotation][in] */ 
            _In_  REFGUID rguidService,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][out] */ 
            _Out_opt_  IUnknown **ppUnknown);
        
        DECLSPEC_XFGVIRT(IMFCaptureSink, AddStream)
        HRESULT ( STDMETHODCALLTYPE *AddStream )( 
            IMFCapturePhotoSink * This,
            /* [annotation][in] */ 
            _In_  DWORD dwSourceStreamIndex,
            /* [annotation][in] */ 
            _In_  IMFMediaType *pMediaType,
            /* [annotation][in] */ 
            _In_opt_  IMFAttributes *pAttributes,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *pdwSinkStreamIndex);
        
        DECLSPEC_XFGVIRT(IMFCaptureSink, Prepare)
        HRESULT ( STDMETHODCALLTYPE *Prepare )( 
            IMFCapturePhotoSink * This);
        
        DECLSPEC_XFGVIRT(IMFCaptureSink, RemoveAllStreams)
        HRESULT ( STDMETHODCALLTYPE *RemoveAllStreams )( 
            IMFCapturePhotoSink * This);
        
        DECLSPEC_XFGVIRT(IMFCapturePhotoSink, SetOutputFileName)
        HRESULT ( STDMETHODCALLTYPE *SetOutputFileName )( 
            IMFCapturePhotoSink * This,
            /* [annotation][in] */ 
            _In_  LPCWSTR fileName);
        
        DECLSPEC_XFGVIRT(IMFCapturePhotoSink, SetSampleCallback)
        HRESULT ( STDMETHODCALLTYPE *SetSampleCallback )( 
            IMFCapturePhotoSink * This,
            /* [annotation][in] */ 
            _In_  IMFCaptureEngineOnSampleCallback *pCallback);
        
        DECLSPEC_XFGVIRT(IMFCapturePhotoSink, SetOutputByteStream)
        HRESULT ( STDMETHODCALLTYPE *SetOutputByteStream )( 
            IMFCapturePhotoSink * This,
            /* [annotation][in] */ 
            _In_  IMFByteStream *pByteStream);
        
        END_INTERFACE
    } IMFCapturePhotoSinkVtbl;

    interface IMFCapturePhotoSink
    {
        CONST_VTBL struct IMFCapturePhotoSinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFCapturePhotoSink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFCapturePhotoSink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFCapturePhotoSink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFCapturePhotoSink_GetOutputMediaType(This,dwSinkStreamIndex,ppMediaType)	\
    ( (This)->lpVtbl -> GetOutputMediaType(This,dwSinkStreamIndex,ppMediaType) ) 

#define IMFCapturePhotoSink_GetService(This,dwSinkStreamIndex,rguidService,riid,ppUnknown)	\
    ( (This)->lpVtbl -> GetService(This,dwSinkStreamIndex,rguidService,riid,ppUnknown) ) 

#define IMFCapturePhotoSink_AddStream(This,dwSourceStreamIndex,pMediaType,pAttributes,pdwSinkStreamIndex)	\
    ( (This)->lpVtbl -> AddStream(This,dwSourceStreamIndex,pMediaType,pAttributes,pdwSinkStreamIndex) ) 

#define IMFCapturePhotoSink_Prepare(This)	\
    ( (This)->lpVtbl -> Prepare(This) ) 

#define IMFCapturePhotoSink_RemoveAllStreams(This)	\
    ( (This)->lpVtbl -> RemoveAllStreams(This) ) 


#define IMFCapturePhotoSink_SetOutputFileName(This,fileName)	\
    ( (This)->lpVtbl -> SetOutputFileName(This,fileName) ) 

#define IMFCapturePhotoSink_SetSampleCallback(This,pCallback)	\
    ( (This)->lpVtbl -> SetSampleCallback(This,pCallback) ) 

#define IMFCapturePhotoSink_SetOutputByteStream(This,pByteStream)	\
    ( (This)->lpVtbl -> SetOutputByteStream(This,pByteStream) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFCapturePhotoSink_INTERFACE_DEFINED__ */


#ifndef __IMFCaptureSource_INTERFACE_DEFINED__
#define __IMFCaptureSource_INTERFACE_DEFINED__

/* interface IMFCaptureSource */
/* [local][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMFCaptureSource;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("439a42a8-0d2c-4505-be83-f79b2a05d5c4")
    IMFCaptureSource : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCaptureDeviceSource( 
            /* [annotation][in] */ 
            _In_  MF_CAPTURE_ENGINE_DEVICE_TYPE mfCaptureEngineDeviceType,
            /* [annotation][out] */ 
            _Out_opt_  IMFMediaSource **ppMediaSource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCaptureDeviceActivate( 
            /* [annotation][in] */ 
            _In_  MF_CAPTURE_ENGINE_DEVICE_TYPE mfCaptureEngineDeviceType,
            /* [annotation][out] */ 
            _Out_opt_  IMFActivate **ppActivate) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetService( 
            /* [annotation][in] */ 
            _In_  REFIID rguidService,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][out] */ 
            _Out_opt_  IUnknown **ppUnknown) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddEffect( 
            /* [annotation][in] */ 
            _In_  DWORD dwSourceStreamIndex,
            /* [annotation][in] */ 
            _In_  IUnknown *pUnknown) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveEffect( 
            /* [annotation][in] */ 
            _In_  DWORD dwSourceStreamIndex,
            /* [annotation][in] */ 
            _In_  IUnknown *pUnknown) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveAllEffects( 
            /* [annotation][in] */ 
            _In_  DWORD dwSourceStreamIndex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAvailableDeviceMediaType( 
            /* [annotation][in] */ 
            _In_  DWORD dwSourceStreamIndex,
            /* [annotation][in] */ 
            _In_  DWORD dwMediaTypeIndex,
            /* [annotation][out] */ 
            _Out_opt_  IMFMediaType **ppMediaType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetCurrentDeviceMediaType( 
            /* [annotation][in] */ 
            _In_  DWORD dwSourceStreamIndex,
            /* [annotation][in] */ 
            _In_  IMFMediaType *pMediaType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCurrentDeviceMediaType( 
            /* [annotation][in] */ 
            _In_  DWORD dwSourceStreamIndex,
            /* [annotation][out] */ 
            _Out_  IMFMediaType **ppMediaType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDeviceStreamCount( 
            /* [annotation][out] */ 
            _Out_  DWORD *pdwStreamCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDeviceStreamCategory( 
            /* [annotation][in] */ 
            _In_  DWORD dwSourceStreamIndex,
            /* [annotation][out] */ 
            _Out_  MF_CAPTURE_ENGINE_STREAM_CATEGORY *pStreamCategory) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMirrorState( 
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][out] */ 
            _Out_  BOOL *pfMirrorState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetMirrorState( 
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  BOOL fMirrorState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStreamIndexFromFriendlyName( 
            /* [annotation][in] */ 
            _In_  UINT32 uifriendlyName,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwActualStreamIndex) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFCaptureSourceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFCaptureSource * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFCaptureSource * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFCaptureSource * This);
        
        DECLSPEC_XFGVIRT(IMFCaptureSource, GetCaptureDeviceSource)
        HRESULT ( STDMETHODCALLTYPE *GetCaptureDeviceSource )( 
            IMFCaptureSource * This,
            /* [annotation][in] */ 
            _In_  MF_CAPTURE_ENGINE_DEVICE_TYPE mfCaptureEngineDeviceType,
            /* [annotation][out] */ 
            _Out_opt_  IMFMediaSource **ppMediaSource);
        
        DECLSPEC_XFGVIRT(IMFCaptureSource, GetCaptureDeviceActivate)
        HRESULT ( STDMETHODCALLTYPE *GetCaptureDeviceActivate )( 
            IMFCaptureSource * This,
            /* [annotation][in] */ 
            _In_  MF_CAPTURE_ENGINE_DEVICE_TYPE mfCaptureEngineDeviceType,
            /* [annotation][out] */ 
            _Out_opt_  IMFActivate **ppActivate);
        
        DECLSPEC_XFGVIRT(IMFCaptureSource, GetService)
        HRESULT ( STDMETHODCALLTYPE *GetService )( 
            IMFCaptureSource * This,
            /* [annotation][in] */ 
            _In_  REFIID rguidService,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][out] */ 
            _Out_opt_  IUnknown **ppUnknown);
        
        DECLSPEC_XFGVIRT(IMFCaptureSource, AddEffect)
        HRESULT ( STDMETHODCALLTYPE *AddEffect )( 
            IMFCaptureSource * This,
            /* [annotation][in] */ 
            _In_  DWORD dwSourceStreamIndex,
            /* [annotation][in] */ 
            _In_  IUnknown *pUnknown);
        
        DECLSPEC_XFGVIRT(IMFCaptureSource, RemoveEffect)
        HRESULT ( STDMETHODCALLTYPE *RemoveEffect )( 
            IMFCaptureSource * This,
            /* [annotation][in] */ 
            _In_  DWORD dwSourceStreamIndex,
            /* [annotation][in] */ 
            _In_  IUnknown *pUnknown);
        
        DECLSPEC_XFGVIRT(IMFCaptureSource, RemoveAllEffects)
        HRESULT ( STDMETHODCALLTYPE *RemoveAllEffects )( 
            IMFCaptureSource * This,
            /* [annotation][in] */ 
            _In_  DWORD dwSourceStreamIndex);
        
        DECLSPEC_XFGVIRT(IMFCaptureSource, GetAvailableDeviceMediaType)
        HRESULT ( STDMETHODCALLTYPE *GetAvailableDeviceMediaType )( 
            IMFCaptureSource * This,
            /* [annotation][in] */ 
            _In_  DWORD dwSourceStreamIndex,
            /* [annotation][in] */ 
            _In_  DWORD dwMediaTypeIndex,
            /* [annotation][out] */ 
            _Out_opt_  IMFMediaType **ppMediaType);
        
        DECLSPEC_XFGVIRT(IMFCaptureSource, SetCurrentDeviceMediaType)
        HRESULT ( STDMETHODCALLTYPE *SetCurrentDeviceMediaType )( 
            IMFCaptureSource * This,
            /* [annotation][in] */ 
            _In_  DWORD dwSourceStreamIndex,
            /* [annotation][in] */ 
            _In_  IMFMediaType *pMediaType);
        
        DECLSPEC_XFGVIRT(IMFCaptureSource, GetCurrentDeviceMediaType)
        HRESULT ( STDMETHODCALLTYPE *GetCurrentDeviceMediaType )( 
            IMFCaptureSource * This,
            /* [annotation][in] */ 
            _In_  DWORD dwSourceStreamIndex,
            /* [annotation][out] */ 
            _Out_  IMFMediaType **ppMediaType);
        
        DECLSPEC_XFGVIRT(IMFCaptureSource, GetDeviceStreamCount)
        HRESULT ( STDMETHODCALLTYPE *GetDeviceStreamCount )( 
            IMFCaptureSource * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwStreamCount);
        
        DECLSPEC_XFGVIRT(IMFCaptureSource, GetDeviceStreamCategory)
        HRESULT ( STDMETHODCALLTYPE *GetDeviceStreamCategory )( 
            IMFCaptureSource * This,
            /* [annotation][in] */ 
            _In_  DWORD dwSourceStreamIndex,
            /* [annotation][out] */ 
            _Out_  MF_CAPTURE_ENGINE_STREAM_CATEGORY *pStreamCategory);
        
        DECLSPEC_XFGVIRT(IMFCaptureSource, GetMirrorState)
        HRESULT ( STDMETHODCALLTYPE *GetMirrorState )( 
            IMFCaptureSource * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][out] */ 
            _Out_  BOOL *pfMirrorState);
        
        DECLSPEC_XFGVIRT(IMFCaptureSource, SetMirrorState)
        HRESULT ( STDMETHODCALLTYPE *SetMirrorState )( 
            IMFCaptureSource * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  BOOL fMirrorState);
        
        DECLSPEC_XFGVIRT(IMFCaptureSource, GetStreamIndexFromFriendlyName)
        HRESULT ( STDMETHODCALLTYPE *GetStreamIndexFromFriendlyName )( 
            IMFCaptureSource * This,
            /* [annotation][in] */ 
            _In_  UINT32 uifriendlyName,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwActualStreamIndex);
        
        END_INTERFACE
    } IMFCaptureSourceVtbl;

    interface IMFCaptureSource
    {
        CONST_VTBL struct IMFCaptureSourceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFCaptureSource_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFCaptureSource_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFCaptureSource_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFCaptureSource_GetCaptureDeviceSource(This,mfCaptureEngineDeviceType,ppMediaSource)	\
    ( (This)->lpVtbl -> GetCaptureDeviceSource(This,mfCaptureEngineDeviceType,ppMediaSource) ) 

#define IMFCaptureSource_GetCaptureDeviceActivate(This,mfCaptureEngineDeviceType,ppActivate)	\
    ( (This)->lpVtbl -> GetCaptureDeviceActivate(This,mfCaptureEngineDeviceType,ppActivate) ) 

#define IMFCaptureSource_GetService(This,rguidService,riid,ppUnknown)	\
    ( (This)->lpVtbl -> GetService(This,rguidService,riid,ppUnknown) ) 

#define IMFCaptureSource_AddEffect(This,dwSourceStreamIndex,pUnknown)	\
    ( (This)->lpVtbl -> AddEffect(This,dwSourceStreamIndex,pUnknown) ) 

#define IMFCaptureSource_RemoveEffect(This,dwSourceStreamIndex,pUnknown)	\
    ( (This)->lpVtbl -> RemoveEffect(This,dwSourceStreamIndex,pUnknown) ) 

#define IMFCaptureSource_RemoveAllEffects(This,dwSourceStreamIndex)	\
    ( (This)->lpVtbl -> RemoveAllEffects(This,dwSourceStreamIndex) ) 

#define IMFCaptureSource_GetAvailableDeviceMediaType(This,dwSourceStreamIndex,dwMediaTypeIndex,ppMediaType)	\
    ( (This)->lpVtbl -> GetAvailableDeviceMediaType(This,dwSourceStreamIndex,dwMediaTypeIndex,ppMediaType) ) 

#define IMFCaptureSource_SetCurrentDeviceMediaType(This,dwSourceStreamIndex,pMediaType)	\
    ( (This)->lpVtbl -> SetCurrentDeviceMediaType(This,dwSourceStreamIndex,pMediaType) ) 

#define IMFCaptureSource_GetCurrentDeviceMediaType(This,dwSourceStreamIndex,ppMediaType)	\
    ( (This)->lpVtbl -> GetCurrentDeviceMediaType(This,dwSourceStreamIndex,ppMediaType) ) 

#define IMFCaptureSource_GetDeviceStreamCount(This,pdwStreamCount)	\
    ( (This)->lpVtbl -> GetDeviceStreamCount(This,pdwStreamCount) ) 

#define IMFCaptureSource_GetDeviceStreamCategory(This,dwSourceStreamIndex,pStreamCategory)	\
    ( (This)->lpVtbl -> GetDeviceStreamCategory(This,dwSourceStreamIndex,pStreamCategory) ) 

#define IMFCaptureSource_GetMirrorState(This,dwStreamIndex,pfMirrorState)	\
    ( (This)->lpVtbl -> GetMirrorState(This,dwStreamIndex,pfMirrorState) ) 

#define IMFCaptureSource_SetMirrorState(This,dwStreamIndex,fMirrorState)	\
    ( (This)->lpVtbl -> SetMirrorState(This,dwStreamIndex,fMirrorState) ) 

#define IMFCaptureSource_GetStreamIndexFromFriendlyName(This,uifriendlyName,pdwActualStreamIndex)	\
    ( (This)->lpVtbl -> GetStreamIndexFromFriendlyName(This,uifriendlyName,pdwActualStreamIndex) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFCaptureSource_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfcaptureengine_0000_0007 */
/* [local] */ 

EXTERN_GUID(CLSID_MFCaptureEngine, 0xefce38d3, 0x8914, 0x4674,0xa7, 0xdf, 0xae, 0x1b, 0x3d, 0x65, 0x4b, 0x8a);


extern RPC_IF_HANDLE __MIDL_itf_mfcaptureengine_0000_0007_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfcaptureengine_0000_0007_v0_0_s_ifspec;

#ifndef __IMFCaptureEngine_INTERFACE_DEFINED__
#define __IMFCaptureEngine_INTERFACE_DEFINED__

/* interface IMFCaptureEngine */
/* [local][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMFCaptureEngine;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a6bba433-176b-48b2-b375-53aa03473207")
    IMFCaptureEngine : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [annotation][in] */ 
            _In_  IMFCaptureEngineOnEventCallback *pEventCallback,
            /* [annotation][in] */ 
            _In_opt_  IMFAttributes *pAttributes,
            /* [annotation][in] */ 
            _In_opt_  IUnknown *pAudioSource,
            /* [annotation][in] */ 
            _In_opt_  IUnknown *pVideoSource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE StartPreview( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE StopPreview( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE StartRecord( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE StopRecord( 
            /* [annotation][in] */ 
            _In_  BOOL bFinalize,
            /* [annotation][in] */ 
            _In_  BOOL bFlushUnprocessedSamples) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE TakePhoto( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSink( 
            /* [annotation][in] */ 
            _In_  MF_CAPTURE_ENGINE_SINK_TYPE mfCaptureEngineSinkType,
            /* [annotation][out] */ 
            _Out_  IMFCaptureSink **ppSink) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSource( 
            /* [annotation][out] */ 
            _Out_  IMFCaptureSource **ppSource) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFCaptureEngineVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFCaptureEngine * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFCaptureEngine * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFCaptureEngine * This);
        
        DECLSPEC_XFGVIRT(IMFCaptureEngine, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            IMFCaptureEngine * This,
            /* [annotation][in] */ 
            _In_  IMFCaptureEngineOnEventCallback *pEventCallback,
            /* [annotation][in] */ 
            _In_opt_  IMFAttributes *pAttributes,
            /* [annotation][in] */ 
            _In_opt_  IUnknown *pAudioSource,
            /* [annotation][in] */ 
            _In_opt_  IUnknown *pVideoSource);
        
        DECLSPEC_XFGVIRT(IMFCaptureEngine, StartPreview)
        HRESULT ( STDMETHODCALLTYPE *StartPreview )( 
            IMFCaptureEngine * This);
        
        DECLSPEC_XFGVIRT(IMFCaptureEngine, StopPreview)
        HRESULT ( STDMETHODCALLTYPE *StopPreview )( 
            IMFCaptureEngine * This);
        
        DECLSPEC_XFGVIRT(IMFCaptureEngine, StartRecord)
        HRESULT ( STDMETHODCALLTYPE *StartRecord )( 
            IMFCaptureEngine * This);
        
        DECLSPEC_XFGVIRT(IMFCaptureEngine, StopRecord)
        HRESULT ( STDMETHODCALLTYPE *StopRecord )( 
            IMFCaptureEngine * This,
            /* [annotation][in] */ 
            _In_  BOOL bFinalize,
            /* [annotation][in] */ 
            _In_  BOOL bFlushUnprocessedSamples);
        
        DECLSPEC_XFGVIRT(IMFCaptureEngine, TakePhoto)
        HRESULT ( STDMETHODCALLTYPE *TakePhoto )( 
            IMFCaptureEngine * This);
        
        DECLSPEC_XFGVIRT(IMFCaptureEngine, GetSink)
        HRESULT ( STDMETHODCALLTYPE *GetSink )( 
            IMFCaptureEngine * This,
            /* [annotation][in] */ 
            _In_  MF_CAPTURE_ENGINE_SINK_TYPE mfCaptureEngineSinkType,
            /* [annotation][out] */ 
            _Out_  IMFCaptureSink **ppSink);
        
        DECLSPEC_XFGVIRT(IMFCaptureEngine, GetSource)
        HRESULT ( STDMETHODCALLTYPE *GetSource )( 
            IMFCaptureEngine * This,
            /* [annotation][out] */ 
            _Out_  IMFCaptureSource **ppSource);
        
        END_INTERFACE
    } IMFCaptureEngineVtbl;

    interface IMFCaptureEngine
    {
        CONST_VTBL struct IMFCaptureEngineVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFCaptureEngine_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFCaptureEngine_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFCaptureEngine_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFCaptureEngine_Initialize(This,pEventCallback,pAttributes,pAudioSource,pVideoSource)	\
    ( (This)->lpVtbl -> Initialize(This,pEventCallback,pAttributes,pAudioSource,pVideoSource) ) 

#define IMFCaptureEngine_StartPreview(This)	\
    ( (This)->lpVtbl -> StartPreview(This) ) 

#define IMFCaptureEngine_StopPreview(This)	\
    ( (This)->lpVtbl -> StopPreview(This) ) 

#define IMFCaptureEngine_StartRecord(This)	\
    ( (This)->lpVtbl -> StartRecord(This) ) 

#define IMFCaptureEngine_StopRecord(This,bFinalize,bFlushUnprocessedSamples)	\
    ( (This)->lpVtbl -> StopRecord(This,bFinalize,bFlushUnprocessedSamples) ) 

#define IMFCaptureEngine_TakePhoto(This)	\
    ( (This)->lpVtbl -> TakePhoto(This) ) 

#define IMFCaptureEngine_GetSink(This,mfCaptureEngineSinkType,ppSink)	\
    ( (This)->lpVtbl -> GetSink(This,mfCaptureEngineSinkType,ppSink) ) 

#define IMFCaptureEngine_GetSource(This,ppSource)	\
    ( (This)->lpVtbl -> GetSource(This,ppSource) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFCaptureEngine_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfcaptureengine_0000_0008 */
/* [local] */ 

EXTERN_GUID(CLSID_MFCaptureEngineClassFactory, 0xefce38d3, 0x8914, 0x4674,0xa7, 0xdf, 0xae, 0x1b, 0x3d, 0x65, 0x4b, 0x8a);


extern RPC_IF_HANDLE __MIDL_itf_mfcaptureengine_0000_0008_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfcaptureengine_0000_0008_v0_0_s_ifspec;

#ifndef __IMFCaptureEngineClassFactory_INTERFACE_DEFINED__
#define __IMFCaptureEngineClassFactory_INTERFACE_DEFINED__

/* interface IMFCaptureEngineClassFactory */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFCaptureEngineClassFactory;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8f02d140-56fc-4302-a705-3a97c78be779")
    IMFCaptureEngineClassFactory : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateInstance( 
            /* [annotation][in] */ 
            _In_  REFCLSID clsid,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _Out_  LPVOID *ppvObject) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFCaptureEngineClassFactoryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFCaptureEngineClassFactory * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFCaptureEngineClassFactory * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFCaptureEngineClassFactory * This);
        
        DECLSPEC_XFGVIRT(IMFCaptureEngineClassFactory, CreateInstance)
        HRESULT ( STDMETHODCALLTYPE *CreateInstance )( 
            IMFCaptureEngineClassFactory * This,
            /* [annotation][in] */ 
            _In_  REFCLSID clsid,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _Out_  LPVOID *ppvObject);
        
        END_INTERFACE
    } IMFCaptureEngineClassFactoryVtbl;

    interface IMFCaptureEngineClassFactory
    {
        CONST_VTBL struct IMFCaptureEngineClassFactoryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFCaptureEngineClassFactory_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFCaptureEngineClassFactory_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFCaptureEngineClassFactory_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFCaptureEngineClassFactory_CreateInstance(This,clsid,riid,ppvObject)	\
    ( (This)->lpVtbl -> CreateInstance(This,clsid,riid,ppvObject) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFCaptureEngineClassFactory_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfcaptureengine_0000_0009 */
/* [local] */ 

EXTERN_GUID(MFSampleExtension_DeviceReferenceSystemTime, 0x6523775a, 0xba2d, 0x405f,0xb2, 0xc5, 0x01, 0xff, 0x88, 0xe2, 0xe8, 0xf6);


extern RPC_IF_HANDLE __MIDL_itf_mfcaptureengine_0000_0009_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfcaptureengine_0000_0009_v0_0_s_ifspec;

#ifndef __IMFCaptureEngineOnSampleCallback2_INTERFACE_DEFINED__
#define __IMFCaptureEngineOnSampleCallback2_INTERFACE_DEFINED__

/* interface IMFCaptureEngineOnSampleCallback2 */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFCaptureEngineOnSampleCallback2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("e37ceed7-340f-4514-9f4d-9c2ae026100b")
    IMFCaptureEngineOnSampleCallback2 : public IMFCaptureEngineOnSampleCallback
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnSynchronizedEvent( 
            /* [annotation][in] */ 
            _In_  IMFMediaEvent *pEvent) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFCaptureEngineOnSampleCallback2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFCaptureEngineOnSampleCallback2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFCaptureEngineOnSampleCallback2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFCaptureEngineOnSampleCallback2 * This);
        
        DECLSPEC_XFGVIRT(IMFCaptureEngineOnSampleCallback, OnSample)
        HRESULT ( STDMETHODCALLTYPE *OnSample )( 
            IMFCaptureEngineOnSampleCallback2 * This,
            /* [annotation][in] */ 
            _In_opt_  IMFSample *pSample);
        
        DECLSPEC_XFGVIRT(IMFCaptureEngineOnSampleCallback2, OnSynchronizedEvent)
        HRESULT ( STDMETHODCALLTYPE *OnSynchronizedEvent )( 
            IMFCaptureEngineOnSampleCallback2 * This,
            /* [annotation][in] */ 
            _In_  IMFMediaEvent *pEvent);
        
        END_INTERFACE
    } IMFCaptureEngineOnSampleCallback2Vtbl;

    interface IMFCaptureEngineOnSampleCallback2
    {
        CONST_VTBL struct IMFCaptureEngineOnSampleCallback2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFCaptureEngineOnSampleCallback2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFCaptureEngineOnSampleCallback2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFCaptureEngineOnSampleCallback2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFCaptureEngineOnSampleCallback2_OnSample(This,pSample)	\
    ( (This)->lpVtbl -> OnSample(This,pSample) ) 


#define IMFCaptureEngineOnSampleCallback2_OnSynchronizedEvent(This,pEvent)	\
    ( (This)->lpVtbl -> OnSynchronizedEvent(This,pEvent) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFCaptureEngineOnSampleCallback2_INTERFACE_DEFINED__ */


#ifndef __IMFCaptureSink2_INTERFACE_DEFINED__
#define __IMFCaptureSink2_INTERFACE_DEFINED__

/* interface IMFCaptureSink2 */
/* [local][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMFCaptureSink2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f9e4219e-6197-4b5e-b888-bee310ab2c59")
    IMFCaptureSink2 : public IMFCaptureSink
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetOutputMediaType( 
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  IMFMediaType *pMediaType,
            /* [annotation][in] */ 
            _In_opt_  IMFAttributes *pEncodingAttributes) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFCaptureSink2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFCaptureSink2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFCaptureSink2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFCaptureSink2 * This);
        
        DECLSPEC_XFGVIRT(IMFCaptureSink, GetOutputMediaType)
        HRESULT ( STDMETHODCALLTYPE *GetOutputMediaType )( 
            IMFCaptureSink2 * This,
            /* [annotation][in] */ 
            _In_  DWORD dwSinkStreamIndex,
            /* [annotation][out] */ 
            _Out_opt_  IMFMediaType **ppMediaType);
        
        DECLSPEC_XFGVIRT(IMFCaptureSink, GetService)
        HRESULT ( STDMETHODCALLTYPE *GetService )( 
            IMFCaptureSink2 * This,
            /* [annotation][in] */ 
            _In_  DWORD dwSinkStreamIndex,
            /* [annotation][in] */ 
            _In_  REFGUID rguidService,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][out] */ 
            _Out_opt_  IUnknown **ppUnknown);
        
        DECLSPEC_XFGVIRT(IMFCaptureSink, AddStream)
        HRESULT ( STDMETHODCALLTYPE *AddStream )( 
            IMFCaptureSink2 * This,
            /* [annotation][in] */ 
            _In_  DWORD dwSourceStreamIndex,
            /* [annotation][in] */ 
            _In_  IMFMediaType *pMediaType,
            /* [annotation][in] */ 
            _In_opt_  IMFAttributes *pAttributes,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *pdwSinkStreamIndex);
        
        DECLSPEC_XFGVIRT(IMFCaptureSink, Prepare)
        HRESULT ( STDMETHODCALLTYPE *Prepare )( 
            IMFCaptureSink2 * This);
        
        DECLSPEC_XFGVIRT(IMFCaptureSink, RemoveAllStreams)
        HRESULT ( STDMETHODCALLTYPE *RemoveAllStreams )( 
            IMFCaptureSink2 * This);
        
        DECLSPEC_XFGVIRT(IMFCaptureSink2, SetOutputMediaType)
        HRESULT ( STDMETHODCALLTYPE *SetOutputMediaType )( 
            IMFCaptureSink2 * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  IMFMediaType *pMediaType,
            /* [annotation][in] */ 
            _In_opt_  IMFAttributes *pEncodingAttributes);
        
        END_INTERFACE
    } IMFCaptureSink2Vtbl;

    interface IMFCaptureSink2
    {
        CONST_VTBL struct IMFCaptureSink2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFCaptureSink2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFCaptureSink2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFCaptureSink2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFCaptureSink2_GetOutputMediaType(This,dwSinkStreamIndex,ppMediaType)	\
    ( (This)->lpVtbl -> GetOutputMediaType(This,dwSinkStreamIndex,ppMediaType) ) 

#define IMFCaptureSink2_GetService(This,dwSinkStreamIndex,rguidService,riid,ppUnknown)	\
    ( (This)->lpVtbl -> GetService(This,dwSinkStreamIndex,rguidService,riid,ppUnknown) ) 

#define IMFCaptureSink2_AddStream(This,dwSourceStreamIndex,pMediaType,pAttributes,pdwSinkStreamIndex)	\
    ( (This)->lpVtbl -> AddStream(This,dwSourceStreamIndex,pMediaType,pAttributes,pdwSinkStreamIndex) ) 

#define IMFCaptureSink2_Prepare(This)	\
    ( (This)->lpVtbl -> Prepare(This) ) 

#define IMFCaptureSink2_RemoveAllStreams(This)	\
    ( (This)->lpVtbl -> RemoveAllStreams(This) ) 


#define IMFCaptureSink2_SetOutputMediaType(This,dwStreamIndex,pMediaType,pEncodingAttributes)	\
    ( (This)->lpVtbl -> SetOutputMediaType(This,dwStreamIndex,pMediaType,pEncodingAttributes) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFCaptureSink2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfcaptureengine_0000_0011 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#endif // (WINVER >= _WIN32_WINNT_WIN7) 


extern RPC_IF_HANDLE __MIDL_itf_mfcaptureengine_0000_0011_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfcaptureengine_0000_0011_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif



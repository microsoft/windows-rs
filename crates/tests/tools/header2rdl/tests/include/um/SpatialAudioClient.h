

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

#ifndef __spatialaudioclient_h__
#define __spatialaudioclient_h__

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

#ifndef __IAudioFormatEnumerator_FWD_DEFINED__
#define __IAudioFormatEnumerator_FWD_DEFINED__
typedef interface IAudioFormatEnumerator IAudioFormatEnumerator;

#endif 	/* __IAudioFormatEnumerator_FWD_DEFINED__ */


#ifndef __ISpatialAudioObjectBase_FWD_DEFINED__
#define __ISpatialAudioObjectBase_FWD_DEFINED__
typedef interface ISpatialAudioObjectBase ISpatialAudioObjectBase;

#endif 	/* __ISpatialAudioObjectBase_FWD_DEFINED__ */


#ifndef __ISpatialAudioObject_FWD_DEFINED__
#define __ISpatialAudioObject_FWD_DEFINED__
typedef interface ISpatialAudioObject ISpatialAudioObject;

#endif 	/* __ISpatialAudioObject_FWD_DEFINED__ */


#ifndef __ISpatialAudioObjectRenderStreamBase_FWD_DEFINED__
#define __ISpatialAudioObjectRenderStreamBase_FWD_DEFINED__
typedef interface ISpatialAudioObjectRenderStreamBase ISpatialAudioObjectRenderStreamBase;

#endif 	/* __ISpatialAudioObjectRenderStreamBase_FWD_DEFINED__ */


#ifndef __ISpatialAudioObjectRenderStream_FWD_DEFINED__
#define __ISpatialAudioObjectRenderStream_FWD_DEFINED__
typedef interface ISpatialAudioObjectRenderStream ISpatialAudioObjectRenderStream;

#endif 	/* __ISpatialAudioObjectRenderStream_FWD_DEFINED__ */


#ifndef __ISpatialAudioObjectRenderStreamNotify_FWD_DEFINED__
#define __ISpatialAudioObjectRenderStreamNotify_FWD_DEFINED__
typedef interface ISpatialAudioObjectRenderStreamNotify ISpatialAudioObjectRenderStreamNotify;

#endif 	/* __ISpatialAudioObjectRenderStreamNotify_FWD_DEFINED__ */


#ifndef __ISpatialAudioClient_FWD_DEFINED__
#define __ISpatialAudioClient_FWD_DEFINED__
typedef interface ISpatialAudioClient ISpatialAudioClient;

#endif 	/* __ISpatialAudioClient_FWD_DEFINED__ */


#ifndef __ISpatialAudioClient2_FWD_DEFINED__
#define __ISpatialAudioClient2_FWD_DEFINED__
typedef interface ISpatialAudioClient2 ISpatialAudioClient2;

#endif 	/* __ISpatialAudioClient2_FWD_DEFINED__ */


/* header files for imported files */
#include "wtypes.h"
#include "unknwn.h"
#include "hstring.h"
#include "AudioClient.h"
#include "propsys.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_spatialaudioclient_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Application and Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)
typedef /* [v1_enum] */ 
enum AudioObjectType
    {
        AudioObjectType_None	= 0,
        AudioObjectType_Dynamic	= ( 1 << 0 ) ,
        AudioObjectType_FrontLeft	= ( 1 << 1 ) ,
        AudioObjectType_FrontRight	= ( 1 << 2 ) ,
        AudioObjectType_FrontCenter	= ( 1 << 3 ) ,
        AudioObjectType_LowFrequency	= ( 1 << 4 ) ,
        AudioObjectType_SideLeft	= ( 1 << 5 ) ,
        AudioObjectType_SideRight	= ( 1 << 6 ) ,
        AudioObjectType_BackLeft	= ( 1 << 7 ) ,
        AudioObjectType_BackRight	= ( 1 << 8 ) ,
        AudioObjectType_TopFrontLeft	= ( 1 << 9 ) ,
        AudioObjectType_TopFrontRight	= ( 1 << 10 ) ,
        AudioObjectType_TopBackLeft	= ( 1 << 11 ) ,
        AudioObjectType_TopBackRight	= ( 1 << 12 ) ,
        AudioObjectType_BottomFrontLeft	= ( 1 << 13 ) ,
        AudioObjectType_BottomFrontRight	= ( 1 << 14 ) ,
        AudioObjectType_BottomBackLeft	= ( 1 << 15 ) ,
        AudioObjectType_BottomBackRight	= ( 1 << 16 ) ,
        AudioObjectType_BackCenter	= ( 1 << 17 ) ,
        AudioObjectType_StereoLeft	= ( 1 << 18 ) ,
        AudioObjectType_StereoRight	= ( 1 << 19 ) 
    } 	AudioObjectType;

DEFINE_ENUM_FLAG_OPERATORS(AudioObjectType);
typedef /* [v1_enum] */ 
enum SPATIAL_AUDIO_STREAM_OPTIONS
    {
        SPATIAL_AUDIO_STREAM_OPTIONS_NONE	= 0,
        SPATIAL_AUDIO_STREAM_OPTIONS_OFFLOAD	= 0x1
    } 	SPATIAL_AUDIO_STREAM_OPTIONS;

DEFINE_ENUM_FLAG_OPERATORS(SPATIAL_AUDIO_STREAM_OPTIONS);


#pragma pack(push, 1)
typedef struct SpatialAudioObjectRenderStreamActivationParams
    {
    const WAVEFORMATEX *ObjectFormat;
    AudioObjectType StaticObjectTypeMask;
    UINT32 MinDynamicObjectCount;
    UINT32 MaxDynamicObjectCount;
    AUDIO_STREAM_CATEGORY Category;
    HANDLE EventHandle;
    ISpatialAudioObjectRenderStreamNotify *NotifyObject;
    } 	SpatialAudioObjectRenderStreamActivationParams;

typedef struct SpatialAudioObjectRenderStreamActivationParams2
    {
    const WAVEFORMATEX *ObjectFormat;
    AudioObjectType StaticObjectTypeMask;
    UINT32 MinDynamicObjectCount;
    UINT32 MaxDynamicObjectCount;
    AUDIO_STREAM_CATEGORY Category;
    HANDLE EventHandle;
    ISpatialAudioObjectRenderStreamNotify *NotifyObject;
    SPATIAL_AUDIO_STREAM_OPTIONS Options;
    } 	SpatialAudioObjectRenderStreamActivationParams2;


#pragma pack(pop)


extern RPC_IF_HANDLE __MIDL_itf_spatialaudioclient_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_spatialaudioclient_0000_0000_v0_0_s_ifspec;

#ifndef __IAudioFormatEnumerator_INTERFACE_DEFINED__
#define __IAudioFormatEnumerator_INTERFACE_DEFINED__

/* interface IAudioFormatEnumerator */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IAudioFormatEnumerator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DCDAA858-895A-4A22-A5EB-67BDA506096D")
    IAudioFormatEnumerator : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCount( 
            /* [annotation][out] */ 
            _Out_  UINT32 *count) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFormat( 
            /* [annotation][in] */ 
            _In_  UINT32 index,
            /* [annotation][out] */ 
            _Outptr_  WAVEFORMATEX **format) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAudioFormatEnumeratorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAudioFormatEnumerator * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAudioFormatEnumerator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAudioFormatEnumerator * This);
        
        DECLSPEC_XFGVIRT(IAudioFormatEnumerator, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            IAudioFormatEnumerator * This,
            /* [annotation][out] */ 
            _Out_  UINT32 *count);
        
        DECLSPEC_XFGVIRT(IAudioFormatEnumerator, GetFormat)
        HRESULT ( STDMETHODCALLTYPE *GetFormat )( 
            IAudioFormatEnumerator * This,
            /* [annotation][in] */ 
            _In_  UINT32 index,
            /* [annotation][out] */ 
            _Outptr_  WAVEFORMATEX **format);
        
        END_INTERFACE
    } IAudioFormatEnumeratorVtbl;

    interface IAudioFormatEnumerator
    {
        CONST_VTBL struct IAudioFormatEnumeratorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAudioFormatEnumerator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAudioFormatEnumerator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAudioFormatEnumerator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAudioFormatEnumerator_GetCount(This,count)	\
    ( (This)->lpVtbl -> GetCount(This,count) ) 

#define IAudioFormatEnumerator_GetFormat(This,index,format)	\
    ( (This)->lpVtbl -> GetFormat(This,index,format) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAudioFormatEnumerator_INTERFACE_DEFINED__ */


#ifndef __ISpatialAudioObjectBase_INTERFACE_DEFINED__
#define __ISpatialAudioObjectBase_INTERFACE_DEFINED__

/* interface ISpatialAudioObjectBase */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_ISpatialAudioObjectBase;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("CCE0B8F2-8D4D-4EFB-A8CF-3D6ECF1C30E0")
    ISpatialAudioObjectBase : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetBuffer( 
            /* [annotation][size_is][size_is][out] */ 
            _Outptr_result_bytebuffer_(*bufferLength)  BYTE **buffer,
            /* [annotation][out] */ 
            _Out_  UINT32 *bufferLength) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetEndOfStream( 
            /* [annotation][in] */ 
            _In_  UINT32 frameCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsActive( 
            /* [annotation][out] */ 
            _Out_  BOOL *isActive) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAudioObjectType( 
            /* [annotation][out] */ 
            _Out_  AudioObjectType *audioObjectType) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISpatialAudioObjectBaseVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISpatialAudioObjectBase * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISpatialAudioObjectBase * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISpatialAudioObjectBase * This);
        
        DECLSPEC_XFGVIRT(ISpatialAudioObjectBase, GetBuffer)
        HRESULT ( STDMETHODCALLTYPE *GetBuffer )( 
            ISpatialAudioObjectBase * This,
            /* [annotation][size_is][size_is][out] */ 
            _Outptr_result_bytebuffer_(*bufferLength)  BYTE **buffer,
            /* [annotation][out] */ 
            _Out_  UINT32 *bufferLength);
        
        DECLSPEC_XFGVIRT(ISpatialAudioObjectBase, SetEndOfStream)
        HRESULT ( STDMETHODCALLTYPE *SetEndOfStream )( 
            ISpatialAudioObjectBase * This,
            /* [annotation][in] */ 
            _In_  UINT32 frameCount);
        
        DECLSPEC_XFGVIRT(ISpatialAudioObjectBase, IsActive)
        HRESULT ( STDMETHODCALLTYPE *IsActive )( 
            ISpatialAudioObjectBase * This,
            /* [annotation][out] */ 
            _Out_  BOOL *isActive);
        
        DECLSPEC_XFGVIRT(ISpatialAudioObjectBase, GetAudioObjectType)
        HRESULT ( STDMETHODCALLTYPE *GetAudioObjectType )( 
            ISpatialAudioObjectBase * This,
            /* [annotation][out] */ 
            _Out_  AudioObjectType *audioObjectType);
        
        END_INTERFACE
    } ISpatialAudioObjectBaseVtbl;

    interface ISpatialAudioObjectBase
    {
        CONST_VTBL struct ISpatialAudioObjectBaseVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISpatialAudioObjectBase_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISpatialAudioObjectBase_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISpatialAudioObjectBase_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISpatialAudioObjectBase_GetBuffer(This,buffer,bufferLength)	\
    ( (This)->lpVtbl -> GetBuffer(This,buffer,bufferLength) ) 

#define ISpatialAudioObjectBase_SetEndOfStream(This,frameCount)	\
    ( (This)->lpVtbl -> SetEndOfStream(This,frameCount) ) 

#define ISpatialAudioObjectBase_IsActive(This,isActive)	\
    ( (This)->lpVtbl -> IsActive(This,isActive) ) 

#define ISpatialAudioObjectBase_GetAudioObjectType(This,audioObjectType)	\
    ( (This)->lpVtbl -> GetAudioObjectType(This,audioObjectType) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISpatialAudioObjectBase_INTERFACE_DEFINED__ */


#ifndef __ISpatialAudioObject_INTERFACE_DEFINED__
#define __ISpatialAudioObject_INTERFACE_DEFINED__

/* interface ISpatialAudioObject */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_ISpatialAudioObject;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DDE28967-521B-46E5-8F00-BD6F2BC8AB1D")
    ISpatialAudioObject : public ISpatialAudioObjectBase
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetPosition( 
            /* [annotation][in] */ 
            _In_  float x,
            /* [annotation][in] */ 
            _In_  float y,
            /* [annotation][in] */ 
            _In_  float z) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetVolume( 
            /* [annotation][in] */ 
            _In_  float volume) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISpatialAudioObjectVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISpatialAudioObject * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISpatialAudioObject * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISpatialAudioObject * This);
        
        DECLSPEC_XFGVIRT(ISpatialAudioObjectBase, GetBuffer)
        HRESULT ( STDMETHODCALLTYPE *GetBuffer )( 
            ISpatialAudioObject * This,
            /* [annotation][size_is][size_is][out] */ 
            _Outptr_result_bytebuffer_(*bufferLength)  BYTE **buffer,
            /* [annotation][out] */ 
            _Out_  UINT32 *bufferLength);
        
        DECLSPEC_XFGVIRT(ISpatialAudioObjectBase, SetEndOfStream)
        HRESULT ( STDMETHODCALLTYPE *SetEndOfStream )( 
            ISpatialAudioObject * This,
            /* [annotation][in] */ 
            _In_  UINT32 frameCount);
        
        DECLSPEC_XFGVIRT(ISpatialAudioObjectBase, IsActive)
        HRESULT ( STDMETHODCALLTYPE *IsActive )( 
            ISpatialAudioObject * This,
            /* [annotation][out] */ 
            _Out_  BOOL *isActive);
        
        DECLSPEC_XFGVIRT(ISpatialAudioObjectBase, GetAudioObjectType)
        HRESULT ( STDMETHODCALLTYPE *GetAudioObjectType )( 
            ISpatialAudioObject * This,
            /* [annotation][out] */ 
            _Out_  AudioObjectType *audioObjectType);
        
        DECLSPEC_XFGVIRT(ISpatialAudioObject, SetPosition)
        HRESULT ( STDMETHODCALLTYPE *SetPosition )( 
            ISpatialAudioObject * This,
            /* [annotation][in] */ 
            _In_  float x,
            /* [annotation][in] */ 
            _In_  float y,
            /* [annotation][in] */ 
            _In_  float z);
        
        DECLSPEC_XFGVIRT(ISpatialAudioObject, SetVolume)
        HRESULT ( STDMETHODCALLTYPE *SetVolume )( 
            ISpatialAudioObject * This,
            /* [annotation][in] */ 
            _In_  float volume);
        
        END_INTERFACE
    } ISpatialAudioObjectVtbl;

    interface ISpatialAudioObject
    {
        CONST_VTBL struct ISpatialAudioObjectVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISpatialAudioObject_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISpatialAudioObject_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISpatialAudioObject_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISpatialAudioObject_GetBuffer(This,buffer,bufferLength)	\
    ( (This)->lpVtbl -> GetBuffer(This,buffer,bufferLength) ) 

#define ISpatialAudioObject_SetEndOfStream(This,frameCount)	\
    ( (This)->lpVtbl -> SetEndOfStream(This,frameCount) ) 

#define ISpatialAudioObject_IsActive(This,isActive)	\
    ( (This)->lpVtbl -> IsActive(This,isActive) ) 

#define ISpatialAudioObject_GetAudioObjectType(This,audioObjectType)	\
    ( (This)->lpVtbl -> GetAudioObjectType(This,audioObjectType) ) 


#define ISpatialAudioObject_SetPosition(This,x,y,z)	\
    ( (This)->lpVtbl -> SetPosition(This,x,y,z) ) 

#define ISpatialAudioObject_SetVolume(This,volume)	\
    ( (This)->lpVtbl -> SetVolume(This,volume) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISpatialAudioObject_INTERFACE_DEFINED__ */


#ifndef __ISpatialAudioObjectRenderStreamBase_INTERFACE_DEFINED__
#define __ISpatialAudioObjectRenderStreamBase_INTERFACE_DEFINED__

/* interface ISpatialAudioObjectRenderStreamBase */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_ISpatialAudioObjectRenderStreamBase;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("FEAAF403-C1D8-450D-AA05-E0CCEE7502A8")
    ISpatialAudioObjectRenderStreamBase : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetAvailableDynamicObjectCount( 
            /* [annotation][out] */ 
            _Out_  UINT32 *value) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetService( 
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **service) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Start( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Stop( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BeginUpdatingAudioObjects( 
            /* [annotation][out] */ 
            _Out_  UINT32 *availableDynamicObjectCount,
            /* [annotation][out] */ 
            _Out_  UINT32 *frameCountPerBuffer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EndUpdatingAudioObjects( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISpatialAudioObjectRenderStreamBaseVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISpatialAudioObjectRenderStreamBase * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISpatialAudioObjectRenderStreamBase * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISpatialAudioObjectRenderStreamBase * This);
        
        DECLSPEC_XFGVIRT(ISpatialAudioObjectRenderStreamBase, GetAvailableDynamicObjectCount)
        HRESULT ( STDMETHODCALLTYPE *GetAvailableDynamicObjectCount )( 
            ISpatialAudioObjectRenderStreamBase * This,
            /* [annotation][out] */ 
            _Out_  UINT32 *value);
        
        DECLSPEC_XFGVIRT(ISpatialAudioObjectRenderStreamBase, GetService)
        HRESULT ( STDMETHODCALLTYPE *GetService )( 
            ISpatialAudioObjectRenderStreamBase * This,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **service);
        
        DECLSPEC_XFGVIRT(ISpatialAudioObjectRenderStreamBase, Start)
        HRESULT ( STDMETHODCALLTYPE *Start )( 
            ISpatialAudioObjectRenderStreamBase * This);
        
        DECLSPEC_XFGVIRT(ISpatialAudioObjectRenderStreamBase, Stop)
        HRESULT ( STDMETHODCALLTYPE *Stop )( 
            ISpatialAudioObjectRenderStreamBase * This);
        
        DECLSPEC_XFGVIRT(ISpatialAudioObjectRenderStreamBase, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            ISpatialAudioObjectRenderStreamBase * This);
        
        DECLSPEC_XFGVIRT(ISpatialAudioObjectRenderStreamBase, BeginUpdatingAudioObjects)
        HRESULT ( STDMETHODCALLTYPE *BeginUpdatingAudioObjects )( 
            ISpatialAudioObjectRenderStreamBase * This,
            /* [annotation][out] */ 
            _Out_  UINT32 *availableDynamicObjectCount,
            /* [annotation][out] */ 
            _Out_  UINT32 *frameCountPerBuffer);
        
        DECLSPEC_XFGVIRT(ISpatialAudioObjectRenderStreamBase, EndUpdatingAudioObjects)
        HRESULT ( STDMETHODCALLTYPE *EndUpdatingAudioObjects )( 
            ISpatialAudioObjectRenderStreamBase * This);
        
        END_INTERFACE
    } ISpatialAudioObjectRenderStreamBaseVtbl;

    interface ISpatialAudioObjectRenderStreamBase
    {
        CONST_VTBL struct ISpatialAudioObjectRenderStreamBaseVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISpatialAudioObjectRenderStreamBase_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISpatialAudioObjectRenderStreamBase_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISpatialAudioObjectRenderStreamBase_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISpatialAudioObjectRenderStreamBase_GetAvailableDynamicObjectCount(This,value)	\
    ( (This)->lpVtbl -> GetAvailableDynamicObjectCount(This,value) ) 

#define ISpatialAudioObjectRenderStreamBase_GetService(This,riid,service)	\
    ( (This)->lpVtbl -> GetService(This,riid,service) ) 

#define ISpatialAudioObjectRenderStreamBase_Start(This)	\
    ( (This)->lpVtbl -> Start(This) ) 

#define ISpatialAudioObjectRenderStreamBase_Stop(This)	\
    ( (This)->lpVtbl -> Stop(This) ) 

#define ISpatialAudioObjectRenderStreamBase_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define ISpatialAudioObjectRenderStreamBase_BeginUpdatingAudioObjects(This,availableDynamicObjectCount,frameCountPerBuffer)	\
    ( (This)->lpVtbl -> BeginUpdatingAudioObjects(This,availableDynamicObjectCount,frameCountPerBuffer) ) 

#define ISpatialAudioObjectRenderStreamBase_EndUpdatingAudioObjects(This)	\
    ( (This)->lpVtbl -> EndUpdatingAudioObjects(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISpatialAudioObjectRenderStreamBase_INTERFACE_DEFINED__ */


#ifndef __ISpatialAudioObjectRenderStream_INTERFACE_DEFINED__
#define __ISpatialAudioObjectRenderStream_INTERFACE_DEFINED__

/* interface ISpatialAudioObjectRenderStream */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_ISpatialAudioObjectRenderStream;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BAB5F473-B423-477B-85F5-B5A332A04153")
    ISpatialAudioObjectRenderStream : public ISpatialAudioObjectRenderStreamBase
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ActivateSpatialAudioObject( 
            /* [annotation][in] */ 
            _In_  AudioObjectType type,
            /* [annotation][out] */ 
            _COM_Outptr_  ISpatialAudioObject **audioObject) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISpatialAudioObjectRenderStreamVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISpatialAudioObjectRenderStream * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISpatialAudioObjectRenderStream * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISpatialAudioObjectRenderStream * This);
        
        DECLSPEC_XFGVIRT(ISpatialAudioObjectRenderStreamBase, GetAvailableDynamicObjectCount)
        HRESULT ( STDMETHODCALLTYPE *GetAvailableDynamicObjectCount )( 
            ISpatialAudioObjectRenderStream * This,
            /* [annotation][out] */ 
            _Out_  UINT32 *value);
        
        DECLSPEC_XFGVIRT(ISpatialAudioObjectRenderStreamBase, GetService)
        HRESULT ( STDMETHODCALLTYPE *GetService )( 
            ISpatialAudioObjectRenderStream * This,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **service);
        
        DECLSPEC_XFGVIRT(ISpatialAudioObjectRenderStreamBase, Start)
        HRESULT ( STDMETHODCALLTYPE *Start )( 
            ISpatialAudioObjectRenderStream * This);
        
        DECLSPEC_XFGVIRT(ISpatialAudioObjectRenderStreamBase, Stop)
        HRESULT ( STDMETHODCALLTYPE *Stop )( 
            ISpatialAudioObjectRenderStream * This);
        
        DECLSPEC_XFGVIRT(ISpatialAudioObjectRenderStreamBase, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            ISpatialAudioObjectRenderStream * This);
        
        DECLSPEC_XFGVIRT(ISpatialAudioObjectRenderStreamBase, BeginUpdatingAudioObjects)
        HRESULT ( STDMETHODCALLTYPE *BeginUpdatingAudioObjects )( 
            ISpatialAudioObjectRenderStream * This,
            /* [annotation][out] */ 
            _Out_  UINT32 *availableDynamicObjectCount,
            /* [annotation][out] */ 
            _Out_  UINT32 *frameCountPerBuffer);
        
        DECLSPEC_XFGVIRT(ISpatialAudioObjectRenderStreamBase, EndUpdatingAudioObjects)
        HRESULT ( STDMETHODCALLTYPE *EndUpdatingAudioObjects )( 
            ISpatialAudioObjectRenderStream * This);
        
        DECLSPEC_XFGVIRT(ISpatialAudioObjectRenderStream, ActivateSpatialAudioObject)
        HRESULT ( STDMETHODCALLTYPE *ActivateSpatialAudioObject )( 
            ISpatialAudioObjectRenderStream * This,
            /* [annotation][in] */ 
            _In_  AudioObjectType type,
            /* [annotation][out] */ 
            _COM_Outptr_  ISpatialAudioObject **audioObject);
        
        END_INTERFACE
    } ISpatialAudioObjectRenderStreamVtbl;

    interface ISpatialAudioObjectRenderStream
    {
        CONST_VTBL struct ISpatialAudioObjectRenderStreamVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISpatialAudioObjectRenderStream_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISpatialAudioObjectRenderStream_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISpatialAudioObjectRenderStream_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISpatialAudioObjectRenderStream_GetAvailableDynamicObjectCount(This,value)	\
    ( (This)->lpVtbl -> GetAvailableDynamicObjectCount(This,value) ) 

#define ISpatialAudioObjectRenderStream_GetService(This,riid,service)	\
    ( (This)->lpVtbl -> GetService(This,riid,service) ) 

#define ISpatialAudioObjectRenderStream_Start(This)	\
    ( (This)->lpVtbl -> Start(This) ) 

#define ISpatialAudioObjectRenderStream_Stop(This)	\
    ( (This)->lpVtbl -> Stop(This) ) 

#define ISpatialAudioObjectRenderStream_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define ISpatialAudioObjectRenderStream_BeginUpdatingAudioObjects(This,availableDynamicObjectCount,frameCountPerBuffer)	\
    ( (This)->lpVtbl -> BeginUpdatingAudioObjects(This,availableDynamicObjectCount,frameCountPerBuffer) ) 

#define ISpatialAudioObjectRenderStream_EndUpdatingAudioObjects(This)	\
    ( (This)->lpVtbl -> EndUpdatingAudioObjects(This) ) 


#define ISpatialAudioObjectRenderStream_ActivateSpatialAudioObject(This,type,audioObject)	\
    ( (This)->lpVtbl -> ActivateSpatialAudioObject(This,type,audioObject) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISpatialAudioObjectRenderStream_INTERFACE_DEFINED__ */


#ifndef __ISpatialAudioObjectRenderStreamNotify_INTERFACE_DEFINED__
#define __ISpatialAudioObjectRenderStreamNotify_INTERFACE_DEFINED__

/* interface ISpatialAudioObjectRenderStreamNotify */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_ISpatialAudioObjectRenderStreamNotify;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DDDF83E6-68D7-4C70-883F-A1836AFB4A50")
    ISpatialAudioObjectRenderStreamNotify : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnAvailableDynamicObjectCountChange( 
            /* [annotation][in] */ 
            _In_  ISpatialAudioObjectRenderStreamBase *sender,
            /* [annotation][in] */ 
            _In_  LONGLONG hnsComplianceDeadlineTime,
            /* [annotation][in] */ 
            _In_  UINT32 availableDynamicObjectCountChange) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISpatialAudioObjectRenderStreamNotifyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISpatialAudioObjectRenderStreamNotify * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISpatialAudioObjectRenderStreamNotify * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISpatialAudioObjectRenderStreamNotify * This);
        
        DECLSPEC_XFGVIRT(ISpatialAudioObjectRenderStreamNotify, OnAvailableDynamicObjectCountChange)
        HRESULT ( STDMETHODCALLTYPE *OnAvailableDynamicObjectCountChange )( 
            ISpatialAudioObjectRenderStreamNotify * This,
            /* [annotation][in] */ 
            _In_  ISpatialAudioObjectRenderStreamBase *sender,
            /* [annotation][in] */ 
            _In_  LONGLONG hnsComplianceDeadlineTime,
            /* [annotation][in] */ 
            _In_  UINT32 availableDynamicObjectCountChange);
        
        END_INTERFACE
    } ISpatialAudioObjectRenderStreamNotifyVtbl;

    interface ISpatialAudioObjectRenderStreamNotify
    {
        CONST_VTBL struct ISpatialAudioObjectRenderStreamNotifyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISpatialAudioObjectRenderStreamNotify_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISpatialAudioObjectRenderStreamNotify_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISpatialAudioObjectRenderStreamNotify_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISpatialAudioObjectRenderStreamNotify_OnAvailableDynamicObjectCountChange(This,sender,hnsComplianceDeadlineTime,availableDynamicObjectCountChange)	\
    ( (This)->lpVtbl -> OnAvailableDynamicObjectCountChange(This,sender,hnsComplianceDeadlineTime,availableDynamicObjectCountChange) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISpatialAudioObjectRenderStreamNotify_INTERFACE_DEFINED__ */


#ifndef __ISpatialAudioClient_INTERFACE_DEFINED__
#define __ISpatialAudioClient_INTERFACE_DEFINED__

/* interface ISpatialAudioClient */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_ISpatialAudioClient;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BBF8E066-AAAA-49BE-9A4D-FD2A858EA27F")
    ISpatialAudioClient : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetStaticObjectPosition( 
            /* [annotation][in] */ 
            _In_  AudioObjectType type,
            /* [annotation][out] */ 
            _Out_  float *x,
            /* [annotation][out] */ 
            _Out_  float *y,
            /* [annotation][out] */ 
            _Out_  float *z) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNativeStaticObjectTypeMask( 
            /* [annotation][out] */ 
            _Out_  AudioObjectType *mask) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMaxDynamicObjectCount( 
            /* [annotation][out] */ 
            _Out_  UINT32 *value) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSupportedAudioObjectFormatEnumerator( 
            /* [annotation][out] */ 
            _COM_Outptr_  IAudioFormatEnumerator **enumerator) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMaxFrameCount( 
            /* [annotation][in] */ 
            _In_  const WAVEFORMATEX *objectFormat,
            /* [annotation][out] */ 
            _Out_  UINT32 *frameCountPerBuffer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsAudioObjectFormatSupported( 
            /* [annotation][in] */ 
            _In_  const WAVEFORMATEX *objectFormat) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsSpatialAudioStreamAvailable( 
            /* [annotation][in] */ 
            _In_  REFIID streamUuid,
            /* [annotation][in] */ 
            _In_opt_  const PROPVARIANT *auxiliaryInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ActivateSpatialAudioStream( 
            /* [annotation][in] */ 
            _In_  const PROPVARIANT *activationParams,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **stream) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISpatialAudioClientVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISpatialAudioClient * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISpatialAudioClient * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISpatialAudioClient * This);
        
        DECLSPEC_XFGVIRT(ISpatialAudioClient, GetStaticObjectPosition)
        HRESULT ( STDMETHODCALLTYPE *GetStaticObjectPosition )( 
            ISpatialAudioClient * This,
            /* [annotation][in] */ 
            _In_  AudioObjectType type,
            /* [annotation][out] */ 
            _Out_  float *x,
            /* [annotation][out] */ 
            _Out_  float *y,
            /* [annotation][out] */ 
            _Out_  float *z);
        
        DECLSPEC_XFGVIRT(ISpatialAudioClient, GetNativeStaticObjectTypeMask)
        HRESULT ( STDMETHODCALLTYPE *GetNativeStaticObjectTypeMask )( 
            ISpatialAudioClient * This,
            /* [annotation][out] */ 
            _Out_  AudioObjectType *mask);
        
        DECLSPEC_XFGVIRT(ISpatialAudioClient, GetMaxDynamicObjectCount)
        HRESULT ( STDMETHODCALLTYPE *GetMaxDynamicObjectCount )( 
            ISpatialAudioClient * This,
            /* [annotation][out] */ 
            _Out_  UINT32 *value);
        
        DECLSPEC_XFGVIRT(ISpatialAudioClient, GetSupportedAudioObjectFormatEnumerator)
        HRESULT ( STDMETHODCALLTYPE *GetSupportedAudioObjectFormatEnumerator )( 
            ISpatialAudioClient * This,
            /* [annotation][out] */ 
            _COM_Outptr_  IAudioFormatEnumerator **enumerator);
        
        DECLSPEC_XFGVIRT(ISpatialAudioClient, GetMaxFrameCount)
        HRESULT ( STDMETHODCALLTYPE *GetMaxFrameCount )( 
            ISpatialAudioClient * This,
            /* [annotation][in] */ 
            _In_  const WAVEFORMATEX *objectFormat,
            /* [annotation][out] */ 
            _Out_  UINT32 *frameCountPerBuffer);
        
        DECLSPEC_XFGVIRT(ISpatialAudioClient, IsAudioObjectFormatSupported)
        HRESULT ( STDMETHODCALLTYPE *IsAudioObjectFormatSupported )( 
            ISpatialAudioClient * This,
            /* [annotation][in] */ 
            _In_  const WAVEFORMATEX *objectFormat);
        
        DECLSPEC_XFGVIRT(ISpatialAudioClient, IsSpatialAudioStreamAvailable)
        HRESULT ( STDMETHODCALLTYPE *IsSpatialAudioStreamAvailable )( 
            ISpatialAudioClient * This,
            /* [annotation][in] */ 
            _In_  REFIID streamUuid,
            /* [annotation][in] */ 
            _In_opt_  const PROPVARIANT *auxiliaryInfo);
        
        DECLSPEC_XFGVIRT(ISpatialAudioClient, ActivateSpatialAudioStream)
        HRESULT ( STDMETHODCALLTYPE *ActivateSpatialAudioStream )( 
            ISpatialAudioClient * This,
            /* [annotation][in] */ 
            _In_  const PROPVARIANT *activationParams,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **stream);
        
        END_INTERFACE
    } ISpatialAudioClientVtbl;

    interface ISpatialAudioClient
    {
        CONST_VTBL struct ISpatialAudioClientVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISpatialAudioClient_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISpatialAudioClient_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISpatialAudioClient_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISpatialAudioClient_GetStaticObjectPosition(This,type,x,y,z)	\
    ( (This)->lpVtbl -> GetStaticObjectPosition(This,type,x,y,z) ) 

#define ISpatialAudioClient_GetNativeStaticObjectTypeMask(This,mask)	\
    ( (This)->lpVtbl -> GetNativeStaticObjectTypeMask(This,mask) ) 

#define ISpatialAudioClient_GetMaxDynamicObjectCount(This,value)	\
    ( (This)->lpVtbl -> GetMaxDynamicObjectCount(This,value) ) 

#define ISpatialAudioClient_GetSupportedAudioObjectFormatEnumerator(This,enumerator)	\
    ( (This)->lpVtbl -> GetSupportedAudioObjectFormatEnumerator(This,enumerator) ) 

#define ISpatialAudioClient_GetMaxFrameCount(This,objectFormat,frameCountPerBuffer)	\
    ( (This)->lpVtbl -> GetMaxFrameCount(This,objectFormat,frameCountPerBuffer) ) 

#define ISpatialAudioClient_IsAudioObjectFormatSupported(This,objectFormat)	\
    ( (This)->lpVtbl -> IsAudioObjectFormatSupported(This,objectFormat) ) 

#define ISpatialAudioClient_IsSpatialAudioStreamAvailable(This,streamUuid,auxiliaryInfo)	\
    ( (This)->lpVtbl -> IsSpatialAudioStreamAvailable(This,streamUuid,auxiliaryInfo) ) 

#define ISpatialAudioClient_ActivateSpatialAudioStream(This,activationParams,riid,stream)	\
    ( (This)->lpVtbl -> ActivateSpatialAudioStream(This,activationParams,riid,stream) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISpatialAudioClient_INTERFACE_DEFINED__ */


#ifndef __ISpatialAudioClient2_INTERFACE_DEFINED__
#define __ISpatialAudioClient2_INTERFACE_DEFINED__

/* interface ISpatialAudioClient2 */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_ISpatialAudioClient2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("caabe452-a66a-4bee-a93e-e320463f6a53")
    ISpatialAudioClient2 : public ISpatialAudioClient
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE IsOffloadCapable( 
            /* [in] */ AUDIO_STREAM_CATEGORY category,
            /* [annotation][out] */ 
            _Out_  BOOL *isOffloadCapable) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMaxFrameCountForCategory( 
            /* [in] */ AUDIO_STREAM_CATEGORY category,
            /* [in] */ BOOL offloadEnabled,
            /* [annotation][in] */ 
            _In_  const WAVEFORMATEX *objectFormat,
            /* [annotation][out] */ 
            _Out_  UINT32 *frameCountPerBuffer) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISpatialAudioClient2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISpatialAudioClient2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISpatialAudioClient2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISpatialAudioClient2 * This);
        
        DECLSPEC_XFGVIRT(ISpatialAudioClient, GetStaticObjectPosition)
        HRESULT ( STDMETHODCALLTYPE *GetStaticObjectPosition )( 
            ISpatialAudioClient2 * This,
            /* [annotation][in] */ 
            _In_  AudioObjectType type,
            /* [annotation][out] */ 
            _Out_  float *x,
            /* [annotation][out] */ 
            _Out_  float *y,
            /* [annotation][out] */ 
            _Out_  float *z);
        
        DECLSPEC_XFGVIRT(ISpatialAudioClient, GetNativeStaticObjectTypeMask)
        HRESULT ( STDMETHODCALLTYPE *GetNativeStaticObjectTypeMask )( 
            ISpatialAudioClient2 * This,
            /* [annotation][out] */ 
            _Out_  AudioObjectType *mask);
        
        DECLSPEC_XFGVIRT(ISpatialAudioClient, GetMaxDynamicObjectCount)
        HRESULT ( STDMETHODCALLTYPE *GetMaxDynamicObjectCount )( 
            ISpatialAudioClient2 * This,
            /* [annotation][out] */ 
            _Out_  UINT32 *value);
        
        DECLSPEC_XFGVIRT(ISpatialAudioClient, GetSupportedAudioObjectFormatEnumerator)
        HRESULT ( STDMETHODCALLTYPE *GetSupportedAudioObjectFormatEnumerator )( 
            ISpatialAudioClient2 * This,
            /* [annotation][out] */ 
            _COM_Outptr_  IAudioFormatEnumerator **enumerator);
        
        DECLSPEC_XFGVIRT(ISpatialAudioClient, GetMaxFrameCount)
        HRESULT ( STDMETHODCALLTYPE *GetMaxFrameCount )( 
            ISpatialAudioClient2 * This,
            /* [annotation][in] */ 
            _In_  const WAVEFORMATEX *objectFormat,
            /* [annotation][out] */ 
            _Out_  UINT32 *frameCountPerBuffer);
        
        DECLSPEC_XFGVIRT(ISpatialAudioClient, IsAudioObjectFormatSupported)
        HRESULT ( STDMETHODCALLTYPE *IsAudioObjectFormatSupported )( 
            ISpatialAudioClient2 * This,
            /* [annotation][in] */ 
            _In_  const WAVEFORMATEX *objectFormat);
        
        DECLSPEC_XFGVIRT(ISpatialAudioClient, IsSpatialAudioStreamAvailable)
        HRESULT ( STDMETHODCALLTYPE *IsSpatialAudioStreamAvailable )( 
            ISpatialAudioClient2 * This,
            /* [annotation][in] */ 
            _In_  REFIID streamUuid,
            /* [annotation][in] */ 
            _In_opt_  const PROPVARIANT *auxiliaryInfo);
        
        DECLSPEC_XFGVIRT(ISpatialAudioClient, ActivateSpatialAudioStream)
        HRESULT ( STDMETHODCALLTYPE *ActivateSpatialAudioStream )( 
            ISpatialAudioClient2 * This,
            /* [annotation][in] */ 
            _In_  const PROPVARIANT *activationParams,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **stream);
        
        DECLSPEC_XFGVIRT(ISpatialAudioClient2, IsOffloadCapable)
        HRESULT ( STDMETHODCALLTYPE *IsOffloadCapable )( 
            ISpatialAudioClient2 * This,
            /* [in] */ AUDIO_STREAM_CATEGORY category,
            /* [annotation][out] */ 
            _Out_  BOOL *isOffloadCapable);
        
        DECLSPEC_XFGVIRT(ISpatialAudioClient2, GetMaxFrameCountForCategory)
        HRESULT ( STDMETHODCALLTYPE *GetMaxFrameCountForCategory )( 
            ISpatialAudioClient2 * This,
            /* [in] */ AUDIO_STREAM_CATEGORY category,
            /* [in] */ BOOL offloadEnabled,
            /* [annotation][in] */ 
            _In_  const WAVEFORMATEX *objectFormat,
            /* [annotation][out] */ 
            _Out_  UINT32 *frameCountPerBuffer);
        
        END_INTERFACE
    } ISpatialAudioClient2Vtbl;

    interface ISpatialAudioClient2
    {
        CONST_VTBL struct ISpatialAudioClient2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISpatialAudioClient2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISpatialAudioClient2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISpatialAudioClient2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISpatialAudioClient2_GetStaticObjectPosition(This,type,x,y,z)	\
    ( (This)->lpVtbl -> GetStaticObjectPosition(This,type,x,y,z) ) 

#define ISpatialAudioClient2_GetNativeStaticObjectTypeMask(This,mask)	\
    ( (This)->lpVtbl -> GetNativeStaticObjectTypeMask(This,mask) ) 

#define ISpatialAudioClient2_GetMaxDynamicObjectCount(This,value)	\
    ( (This)->lpVtbl -> GetMaxDynamicObjectCount(This,value) ) 

#define ISpatialAudioClient2_GetSupportedAudioObjectFormatEnumerator(This,enumerator)	\
    ( (This)->lpVtbl -> GetSupportedAudioObjectFormatEnumerator(This,enumerator) ) 

#define ISpatialAudioClient2_GetMaxFrameCount(This,objectFormat,frameCountPerBuffer)	\
    ( (This)->lpVtbl -> GetMaxFrameCount(This,objectFormat,frameCountPerBuffer) ) 

#define ISpatialAudioClient2_IsAudioObjectFormatSupported(This,objectFormat)	\
    ( (This)->lpVtbl -> IsAudioObjectFormatSupported(This,objectFormat) ) 

#define ISpatialAudioClient2_IsSpatialAudioStreamAvailable(This,streamUuid,auxiliaryInfo)	\
    ( (This)->lpVtbl -> IsSpatialAudioStreamAvailable(This,streamUuid,auxiliaryInfo) ) 

#define ISpatialAudioClient2_ActivateSpatialAudioStream(This,activationParams,riid,stream)	\
    ( (This)->lpVtbl -> ActivateSpatialAudioStream(This,activationParams,riid,stream) ) 


#define ISpatialAudioClient2_IsOffloadCapable(This,category,isOffloadCapable)	\
    ( (This)->lpVtbl -> IsOffloadCapable(This,category,isOffloadCapable) ) 

#define ISpatialAudioClient2_GetMaxFrameCountForCategory(This,category,offloadEnabled,objectFormat,frameCountPerBuffer)	\
    ( (This)->lpVtbl -> GetMaxFrameCountForCategory(This,category,offloadEnabled,objectFormat,frameCountPerBuffer) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISpatialAudioClient2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_spatialaudioclient_0000_0008 */
/* [local] */ 

// SpatialAudioClientActivationParams is an optional activation parameter for ISpatialAudioClient
//
// ISpatialAudioClient implementations log various things via ETW tracing
// including a "context" identifier and version information.
//
// The "tracing context" identifier helps with correlation of which audio client instance belongs to which application context
//
// Sample app code:
// PROPVARIANT var;
// PropVariantInit(&var);
// auto p = reinterpret_cast<SpatialAudioClientActivationParams *>(CoTaskMemAlloc(sizeof(SpatialAudioClientActivationParams)));
// if (nullptr == p) { ... }
// p->tracingContextId = /* context identifier */;
// p->appId = /* app identifier */;
// p->majorVersion = /* app version info */;
// p->majorVersionN = /* app version info */;
// var.vt = VT_BLOB;
// var.blob.cbSize = sizeof(*p);
// var.blob.pBlobData = reinterpret_cast<BYTE *>(p);
// hr = ActivateAudioInterfaceAsync(device, __uuidof(ISpatialAudioClient), &var, ...);
// ...
// PropVariantClear(&var);
typedef struct SpatialAudioClientActivationParams
    {
    GUID tracingContextId;
    GUID appId;
    int majorVersion;
    int minorVersion1;
    int minorVersion2;
    int minorVersion3;
    } 	SpatialAudioClientActivationParams;

#define SPTLAUDCLNT_E_DESTROYED                           AUDCLNT_ERR(0x0100)
#define SPTLAUDCLNT_E_OUT_OF_ORDER                        AUDCLNT_ERR(0x0101)
#define SPTLAUDCLNT_E_RESOURCES_INVALIDATED               AUDCLNT_ERR(0x0102)
#define SPTLAUDCLNT_E_NO_MORE_OBJECTS                     AUDCLNT_ERR(0x0103)
#define SPTLAUDCLNT_E_PROPERTY_NOT_SUPPORTED              AUDCLNT_ERR(0x0104)
#define SPTLAUDCLNT_E_ERRORS_IN_OBJECT_CALLS              AUDCLNT_ERR(0x0105)
#define SPTLAUDCLNT_E_METADATA_FORMAT_NOT_SUPPORTED       AUDCLNT_ERR(0x0106)
#define SPTLAUDCLNT_E_STREAM_NOT_AVAILABLE                AUDCLNT_ERR(0x0107)
#define SPTLAUDCLNT_E_INVALID_LICENSE                     AUDCLNT_ERR(0x0108)
#define SPTLAUDCLNT_E_STREAM_NOT_STOPPED                  AUDCLNT_ERR(0x010a)
#define SPTLAUDCLNT_E_STATIC_OBJECT_NOT_AVAILABLE         AUDCLNT_ERR(0x010b)
#define SPTLAUDCLNT_E_OBJECT_ALREADY_ACTIVE               AUDCLNT_ERR(0x010c)
#define SPTLAUDCLNT_E_INTERNAL                            AUDCLNT_ERR(0x010d)
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */


extern RPC_IF_HANDLE __MIDL_itf_spatialaudioclient_0000_0008_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_spatialaudioclient_0000_0008_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif



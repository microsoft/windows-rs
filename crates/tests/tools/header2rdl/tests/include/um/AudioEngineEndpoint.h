

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

#ifndef __audioengineendpoint_h__
#define __audioengineendpoint_h__

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

#ifndef __IAudioEndpoint_FWD_DEFINED__
#define __IAudioEndpoint_FWD_DEFINED__
typedef interface IAudioEndpoint IAudioEndpoint;

#endif 	/* __IAudioEndpoint_FWD_DEFINED__ */


#ifndef __IAudioEndpointRT_FWD_DEFINED__
#define __IAudioEndpointRT_FWD_DEFINED__
typedef interface IAudioEndpointRT IAudioEndpointRT;

#endif 	/* __IAudioEndpointRT_FWD_DEFINED__ */


#ifndef __IAudioInputEndpointRT_FWD_DEFINED__
#define __IAudioInputEndpointRT_FWD_DEFINED__
typedef interface IAudioInputEndpointRT IAudioInputEndpointRT;

#endif 	/* __IAudioInputEndpointRT_FWD_DEFINED__ */


#ifndef __IAudioOutputEndpointRT_FWD_DEFINED__
#define __IAudioOutputEndpointRT_FWD_DEFINED__
typedef interface IAudioOutputEndpointRT IAudioOutputEndpointRT;

#endif 	/* __IAudioOutputEndpointRT_FWD_DEFINED__ */


#ifndef __IAudioDeviceEndpoint_FWD_DEFINED__
#define __IAudioDeviceEndpoint_FWD_DEFINED__
typedef interface IAudioDeviceEndpoint IAudioDeviceEndpoint;

#endif 	/* __IAudioDeviceEndpoint_FWD_DEFINED__ */


#ifndef __IAudioEndpointOffloadStreamVolume_FWD_DEFINED__
#define __IAudioEndpointOffloadStreamVolume_FWD_DEFINED__
typedef interface IAudioEndpointOffloadStreamVolume IAudioEndpointOffloadStreamVolume;

#endif 	/* __IAudioEndpointOffloadStreamVolume_FWD_DEFINED__ */


#ifndef __IAudioEndpointOffloadStreamMute_FWD_DEFINED__
#define __IAudioEndpointOffloadStreamMute_FWD_DEFINED__
typedef interface IAudioEndpointOffloadStreamMute IAudioEndpointOffloadStreamMute;

#endif 	/* __IAudioEndpointOffloadStreamMute_FWD_DEFINED__ */


#ifndef __IAudioEndpointOffloadStreamMeter_FWD_DEFINED__
#define __IAudioEndpointOffloadStreamMeter_FWD_DEFINED__
typedef interface IAudioEndpointOffloadStreamMeter IAudioEndpointOffloadStreamMeter;

#endif 	/* __IAudioEndpointOffloadStreamMeter_FWD_DEFINED__ */


#ifndef __IAudioEndpointLastBufferControl_FWD_DEFINED__
#define __IAudioEndpointLastBufferControl_FWD_DEFINED__
typedef interface IAudioEndpointLastBufferControl IAudioEndpointLastBufferControl;

#endif 	/* __IAudioEndpointLastBufferControl_FWD_DEFINED__ */


#ifndef __IAudioLfxControl_FWD_DEFINED__
#define __IAudioLfxControl_FWD_DEFINED__
typedef interface IAudioLfxControl IAudioLfxControl;

#endif 	/* __IAudioLfxControl_FWD_DEFINED__ */


#ifndef __IHardwareAudioEngineBase_FWD_DEFINED__
#define __IHardwareAudioEngineBase_FWD_DEFINED__
typedef interface IHardwareAudioEngineBase IHardwareAudioEngineBase;

#endif 	/* __IHardwareAudioEngineBase_FWD_DEFINED__ */


#ifndef __IAudioEndpointControl_FWD_DEFINED__
#define __IAudioEndpointControl_FWD_DEFINED__
typedef interface IAudioEndpointControl IAudioEndpointControl;

#endif 	/* __IAudioEndpointControl_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#include "propidl.h"
#include "audioapotypes.h"
#include "mmreg.h"
#include "mmdeviceapi.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_audioengineendpoint_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
typedef /* [public][public][public] */ 
enum __MIDL___MIDL_itf_audioengineendpoint_0000_0000_0001
    {
        eHostProcessConnector	= 0,
        eOffloadConnector	= ( eHostProcessConnector + 1 ) ,
        eLoopbackConnector	= ( eOffloadConnector + 1 ) ,
        eKeywordDetectorConnector	= ( eLoopbackConnector + 1 ) ,
        eLoopbackConnectorPostVolume	= ( eKeywordDetectorConnector + 1 ) ,
        eConnectorCount	= ( eLoopbackConnectorPostVolume + 1 ) 
    } 	EndpointConnectorType;

typedef struct AUDIO_ENDPOINT_SHARED_CREATE_PARAMS
    {
    UINT32 u32Size;
    UINT32 u32TSSessionId;
    EndpointConnectorType targetEndpointConnectorType;
    WAVEFORMATEX wfxDeviceFormat;
    } 	AUDIO_ENDPOINT_SHARED_CREATE_PARAMS;

typedef struct AUDIO_ENDPOINT_SHARED_CREATE_PARAMS *PAUDIO_ENDPOINT_SHARED_CREATE_PARAMS;

typedef 
enum AE_POSITION_FLAGS
    {
        POSITION_INVALID	= 0,
        POSITION_DISCONTINUOUS	= 1,
        POSITION_CONTINUOUS	= 2,
        POSITION_QPC_ERROR	= 4
    } 	AE_POSITION_FLAGS;

typedef struct AE_CURRENT_POSITION
    {
    UINT64 u64DevicePosition;
    UINT64 u64StreamPosition;
    UINT64 u64PaddingFrames;
    HNSTIME hnsQPCPosition;
    FLOAT32 f32FramesPerSecond;
    AE_POSITION_FLAGS Flag;
    } 	AE_CURRENT_POSITION;

typedef struct AE_CURRENT_POSITION *PAE_CURRENT_POSITION;



extern RPC_IF_HANDLE __MIDL_itf_audioengineendpoint_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_audioengineendpoint_0000_0000_v0_0_s_ifspec;

#ifndef __IAudioEndpoint_INTERFACE_DEFINED__
#define __IAudioEndpoint_INTERFACE_DEFINED__

/* interface IAudioEndpoint */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IAudioEndpoint;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("30A99515-1527-4451-AF9F-00C5F0234DAF")
    IAudioEndpoint : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetFrameFormat( 
            /* [out] */ WAVEFORMATEX **ppFormat) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFramesPerPacket( 
            /* [out] */ UINT32 *pFramesPerPacket) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLatency( 
            /* [out] */ HNSTIME *pLatency) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetStreamFlags( 
            /* [in] */ DWORD streamFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetEventHandle( 
            /* [in] */ HANDLE eventHandle) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAudioEndpointVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAudioEndpoint * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAudioEndpoint * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAudioEndpoint * This);
        
        DECLSPEC_XFGVIRT(IAudioEndpoint, GetFrameFormat)
        HRESULT ( STDMETHODCALLTYPE *GetFrameFormat )( 
            IAudioEndpoint * This,
            /* [out] */ WAVEFORMATEX **ppFormat);
        
        DECLSPEC_XFGVIRT(IAudioEndpoint, GetFramesPerPacket)
        HRESULT ( STDMETHODCALLTYPE *GetFramesPerPacket )( 
            IAudioEndpoint * This,
            /* [out] */ UINT32 *pFramesPerPacket);
        
        DECLSPEC_XFGVIRT(IAudioEndpoint, GetLatency)
        HRESULT ( STDMETHODCALLTYPE *GetLatency )( 
            IAudioEndpoint * This,
            /* [out] */ HNSTIME *pLatency);
        
        DECLSPEC_XFGVIRT(IAudioEndpoint, SetStreamFlags)
        HRESULT ( STDMETHODCALLTYPE *SetStreamFlags )( 
            IAudioEndpoint * This,
            /* [in] */ DWORD streamFlags);
        
        DECLSPEC_XFGVIRT(IAudioEndpoint, SetEventHandle)
        HRESULT ( STDMETHODCALLTYPE *SetEventHandle )( 
            IAudioEndpoint * This,
            /* [in] */ HANDLE eventHandle);
        
        END_INTERFACE
    } IAudioEndpointVtbl;

    interface IAudioEndpoint
    {
        CONST_VTBL struct IAudioEndpointVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAudioEndpoint_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAudioEndpoint_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAudioEndpoint_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAudioEndpoint_GetFrameFormat(This,ppFormat)	\
    ( (This)->lpVtbl -> GetFrameFormat(This,ppFormat) ) 

#define IAudioEndpoint_GetFramesPerPacket(This,pFramesPerPacket)	\
    ( (This)->lpVtbl -> GetFramesPerPacket(This,pFramesPerPacket) ) 

#define IAudioEndpoint_GetLatency(This,pLatency)	\
    ( (This)->lpVtbl -> GetLatency(This,pLatency) ) 

#define IAudioEndpoint_SetStreamFlags(This,streamFlags)	\
    ( (This)->lpVtbl -> SetStreamFlags(This,streamFlags) ) 

#define IAudioEndpoint_SetEventHandle(This,eventHandle)	\
    ( (This)->lpVtbl -> SetEventHandle(This,eventHandle) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAudioEndpoint_INTERFACE_DEFINED__ */


#ifndef __IAudioEndpointRT_INTERFACE_DEFINED__
#define __IAudioEndpointRT_INTERFACE_DEFINED__

/* interface IAudioEndpointRT */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IAudioEndpointRT;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DFD2005F-A6E5-4d39-A265-939ADA9FBB4D")
    IAudioEndpointRT : public IUnknown
    {
    public:
        virtual void STDMETHODCALLTYPE GetCurrentPadding( 
            /* [out] */ HNSTIME *pPadding,
            /* [out] */ AE_CURRENT_POSITION *pAeCurrentPosition) = 0;
        
        virtual void STDMETHODCALLTYPE ProcessingComplete( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetPinInactive( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetPinActive( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAudioEndpointRTVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAudioEndpointRT * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAudioEndpointRT * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAudioEndpointRT * This);
        
        DECLSPEC_XFGVIRT(IAudioEndpointRT, GetCurrentPadding)
        void ( STDMETHODCALLTYPE *GetCurrentPadding )( 
            IAudioEndpointRT * This,
            /* [out] */ HNSTIME *pPadding,
            /* [out] */ AE_CURRENT_POSITION *pAeCurrentPosition);
        
        DECLSPEC_XFGVIRT(IAudioEndpointRT, ProcessingComplete)
        void ( STDMETHODCALLTYPE *ProcessingComplete )( 
            IAudioEndpointRT * This);
        
        DECLSPEC_XFGVIRT(IAudioEndpointRT, SetPinInactive)
        HRESULT ( STDMETHODCALLTYPE *SetPinInactive )( 
            IAudioEndpointRT * This);
        
        DECLSPEC_XFGVIRT(IAudioEndpointRT, SetPinActive)
        HRESULT ( STDMETHODCALLTYPE *SetPinActive )( 
            IAudioEndpointRT * This);
        
        END_INTERFACE
    } IAudioEndpointRTVtbl;

    interface IAudioEndpointRT
    {
        CONST_VTBL struct IAudioEndpointRTVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAudioEndpointRT_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAudioEndpointRT_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAudioEndpointRT_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAudioEndpointRT_GetCurrentPadding(This,pPadding,pAeCurrentPosition)	\
    ( (This)->lpVtbl -> GetCurrentPadding(This,pPadding,pAeCurrentPosition) ) 

#define IAudioEndpointRT_ProcessingComplete(This)	\
    ( (This)->lpVtbl -> ProcessingComplete(This) ) 

#define IAudioEndpointRT_SetPinInactive(This)	\
    ( (This)->lpVtbl -> SetPinInactive(This) ) 

#define IAudioEndpointRT_SetPinActive(This)	\
    ( (This)->lpVtbl -> SetPinActive(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAudioEndpointRT_INTERFACE_DEFINED__ */


#ifndef __IAudioInputEndpointRT_INTERFACE_DEFINED__
#define __IAudioInputEndpointRT_INTERFACE_DEFINED__

/* interface IAudioInputEndpointRT */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IAudioInputEndpointRT;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8026AB61-92B2-43c1-A1DF-5C37EBD08D82")
    IAudioInputEndpointRT : public IUnknown
    {
    public:
        virtual void STDMETHODCALLTYPE GetInputDataPointer( 
            /* [out][in] */ APO_CONNECTION_PROPERTY *pConnectionProperty,
            /* [out][in] */ AE_CURRENT_POSITION *pAeTimeStamp) = 0;
        
        virtual void STDMETHODCALLTYPE ReleaseInputDataPointer( 
            /* [in] */ UINT32 u32FrameCount,
            /* [in] */ UINT_PTR pDataPointer) = 0;
        
        virtual void STDMETHODCALLTYPE PulseEndpoint( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAudioInputEndpointRTVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAudioInputEndpointRT * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAudioInputEndpointRT * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAudioInputEndpointRT * This);
        
        DECLSPEC_XFGVIRT(IAudioInputEndpointRT, GetInputDataPointer)
        void ( STDMETHODCALLTYPE *GetInputDataPointer )( 
            IAudioInputEndpointRT * This,
            /* [out][in] */ APO_CONNECTION_PROPERTY *pConnectionProperty,
            /* [out][in] */ AE_CURRENT_POSITION *pAeTimeStamp);
        
        DECLSPEC_XFGVIRT(IAudioInputEndpointRT, ReleaseInputDataPointer)
        void ( STDMETHODCALLTYPE *ReleaseInputDataPointer )( 
            IAudioInputEndpointRT * This,
            /* [in] */ UINT32 u32FrameCount,
            /* [in] */ UINT_PTR pDataPointer);
        
        DECLSPEC_XFGVIRT(IAudioInputEndpointRT, PulseEndpoint)
        void ( STDMETHODCALLTYPE *PulseEndpoint )( 
            IAudioInputEndpointRT * This);
        
        END_INTERFACE
    } IAudioInputEndpointRTVtbl;

    interface IAudioInputEndpointRT
    {
        CONST_VTBL struct IAudioInputEndpointRTVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAudioInputEndpointRT_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAudioInputEndpointRT_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAudioInputEndpointRT_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAudioInputEndpointRT_GetInputDataPointer(This,pConnectionProperty,pAeTimeStamp)	\
    ( (This)->lpVtbl -> GetInputDataPointer(This,pConnectionProperty,pAeTimeStamp) ) 

#define IAudioInputEndpointRT_ReleaseInputDataPointer(This,u32FrameCount,pDataPointer)	\
    ( (This)->lpVtbl -> ReleaseInputDataPointer(This,u32FrameCount,pDataPointer) ) 

#define IAudioInputEndpointRT_PulseEndpoint(This)	\
    ( (This)->lpVtbl -> PulseEndpoint(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAudioInputEndpointRT_INTERFACE_DEFINED__ */


#ifndef __IAudioOutputEndpointRT_INTERFACE_DEFINED__
#define __IAudioOutputEndpointRT_INTERFACE_DEFINED__

/* interface IAudioOutputEndpointRT */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IAudioOutputEndpointRT;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8FA906E4-C31C-4e31-932E-19A66385E9AA")
    IAudioOutputEndpointRT : public IUnknown
    {
    public:
        virtual UINT_PTR STDMETHODCALLTYPE GetOutputDataPointer( 
            /* [in] */ UINT32 u32FrameCount,
            /* [in] */ AE_CURRENT_POSITION *pAeTimeStamp) = 0;
        
        virtual void STDMETHODCALLTYPE ReleaseOutputDataPointer( 
            /* [in] */ const APO_CONNECTION_PROPERTY *pConnectionProperty) = 0;
        
        virtual void STDMETHODCALLTYPE PulseEndpoint( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAudioOutputEndpointRTVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAudioOutputEndpointRT * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAudioOutputEndpointRT * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAudioOutputEndpointRT * This);
        
        DECLSPEC_XFGVIRT(IAudioOutputEndpointRT, GetOutputDataPointer)
        UINT_PTR ( STDMETHODCALLTYPE *GetOutputDataPointer )( 
            IAudioOutputEndpointRT * This,
            /* [in] */ UINT32 u32FrameCount,
            /* [in] */ AE_CURRENT_POSITION *pAeTimeStamp);
        
        DECLSPEC_XFGVIRT(IAudioOutputEndpointRT, ReleaseOutputDataPointer)
        void ( STDMETHODCALLTYPE *ReleaseOutputDataPointer )( 
            IAudioOutputEndpointRT * This,
            /* [in] */ const APO_CONNECTION_PROPERTY *pConnectionProperty);
        
        DECLSPEC_XFGVIRT(IAudioOutputEndpointRT, PulseEndpoint)
        void ( STDMETHODCALLTYPE *PulseEndpoint )( 
            IAudioOutputEndpointRT * This);
        
        END_INTERFACE
    } IAudioOutputEndpointRTVtbl;

    interface IAudioOutputEndpointRT
    {
        CONST_VTBL struct IAudioOutputEndpointRTVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAudioOutputEndpointRT_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAudioOutputEndpointRT_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAudioOutputEndpointRT_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAudioOutputEndpointRT_GetOutputDataPointer(This,u32FrameCount,pAeTimeStamp)	\
    ( (This)->lpVtbl -> GetOutputDataPointer(This,u32FrameCount,pAeTimeStamp) ) 

#define IAudioOutputEndpointRT_ReleaseOutputDataPointer(This,pConnectionProperty)	\
    ( (This)->lpVtbl -> ReleaseOutputDataPointer(This,pConnectionProperty) ) 

#define IAudioOutputEndpointRT_PulseEndpoint(This)	\
    ( (This)->lpVtbl -> PulseEndpoint(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAudioOutputEndpointRT_INTERFACE_DEFINED__ */


#ifndef __IAudioDeviceEndpoint_INTERFACE_DEFINED__
#define __IAudioDeviceEndpoint_INTERFACE_DEFINED__

/* interface IAudioDeviceEndpoint */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IAudioDeviceEndpoint;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D4952F5A-A0B2-4cc4-8B82-9358488DD8AC")
    IAudioDeviceEndpoint : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetBuffer( 
            /* [in] */ HNSTIME MaxPeriod,
            /* [in] */ UINT32 u32LatencyCoefficient) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRTCaps( 
            /* [out] */ BOOL *pbIsRTCapable) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEventDrivenCapable( 
            /* [out] */ BOOL *pbisEventCapable) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteExclusiveModeParametersToSharedMemory( 
            /* [in] */ UINT_PTR hTargetProcess,
            /* [in] */ HNSTIME hnsPeriod,
            /* [in] */ HNSTIME hnsBufferDuration,
            /* [in] */ UINT32 u32LatencyCoefficient,
            /* [out] */ UINT32 *pu32SharedMemorySize,
            /* [out] */ UINT_PTR *phSharedMemory) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAudioDeviceEndpointVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAudioDeviceEndpoint * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAudioDeviceEndpoint * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAudioDeviceEndpoint * This);
        
        DECLSPEC_XFGVIRT(IAudioDeviceEndpoint, SetBuffer)
        HRESULT ( STDMETHODCALLTYPE *SetBuffer )( 
            IAudioDeviceEndpoint * This,
            /* [in] */ HNSTIME MaxPeriod,
            /* [in] */ UINT32 u32LatencyCoefficient);
        
        DECLSPEC_XFGVIRT(IAudioDeviceEndpoint, GetRTCaps)
        HRESULT ( STDMETHODCALLTYPE *GetRTCaps )( 
            IAudioDeviceEndpoint * This,
            /* [out] */ BOOL *pbIsRTCapable);
        
        DECLSPEC_XFGVIRT(IAudioDeviceEndpoint, GetEventDrivenCapable)
        HRESULT ( STDMETHODCALLTYPE *GetEventDrivenCapable )( 
            IAudioDeviceEndpoint * This,
            /* [out] */ BOOL *pbisEventCapable);
        
        DECLSPEC_XFGVIRT(IAudioDeviceEndpoint, WriteExclusiveModeParametersToSharedMemory)
        HRESULT ( STDMETHODCALLTYPE *WriteExclusiveModeParametersToSharedMemory )( 
            IAudioDeviceEndpoint * This,
            /* [in] */ UINT_PTR hTargetProcess,
            /* [in] */ HNSTIME hnsPeriod,
            /* [in] */ HNSTIME hnsBufferDuration,
            /* [in] */ UINT32 u32LatencyCoefficient,
            /* [out] */ UINT32 *pu32SharedMemorySize,
            /* [out] */ UINT_PTR *phSharedMemory);
        
        END_INTERFACE
    } IAudioDeviceEndpointVtbl;

    interface IAudioDeviceEndpoint
    {
        CONST_VTBL struct IAudioDeviceEndpointVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAudioDeviceEndpoint_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAudioDeviceEndpoint_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAudioDeviceEndpoint_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAudioDeviceEndpoint_SetBuffer(This,MaxPeriod,u32LatencyCoefficient)	\
    ( (This)->lpVtbl -> SetBuffer(This,MaxPeriod,u32LatencyCoefficient) ) 

#define IAudioDeviceEndpoint_GetRTCaps(This,pbIsRTCapable)	\
    ( (This)->lpVtbl -> GetRTCaps(This,pbIsRTCapable) ) 

#define IAudioDeviceEndpoint_GetEventDrivenCapable(This,pbisEventCapable)	\
    ( (This)->lpVtbl -> GetEventDrivenCapable(This,pbisEventCapable) ) 

#define IAudioDeviceEndpoint_WriteExclusiveModeParametersToSharedMemory(This,hTargetProcess,hnsPeriod,hnsBufferDuration,u32LatencyCoefficient,pu32SharedMemorySize,phSharedMemory)	\
    ( (This)->lpVtbl -> WriteExclusiveModeParametersToSharedMemory(This,hTargetProcess,hnsPeriod,hnsBufferDuration,u32LatencyCoefficient,pu32SharedMemorySize,phSharedMemory) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAudioDeviceEndpoint_INTERFACE_DEFINED__ */


#ifndef __IAudioEndpointOffloadStreamVolume_INTERFACE_DEFINED__
#define __IAudioEndpointOffloadStreamVolume_INTERFACE_DEFINED__

/* interface IAudioEndpointOffloadStreamVolume */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IAudioEndpointOffloadStreamVolume;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("64F1DD49-71CA-4281-8672-3A9EDDD1D0B6")
    IAudioEndpointOffloadStreamVolume : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetVolumeChannelCount( 
            /* [out] */ UINT32 *pu32ChannelCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetChannelVolumes( 
            /* [in] */ UINT32 u32ChannelCount,
            /* [in] */ FLOAT32 *pf32Volumes,
            /* [in] */ AUDIO_CURVE_TYPE u32CurveType,
            /* [in] */ HNSTIME *pCurveDuration) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetChannelVolumes( 
            /* [in] */ UINT32 u32ChannelCount,
            /* [out] */ FLOAT32 *pf32Volumes) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAudioEndpointOffloadStreamVolumeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAudioEndpointOffloadStreamVolume * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAudioEndpointOffloadStreamVolume * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAudioEndpointOffloadStreamVolume * This);
        
        DECLSPEC_XFGVIRT(IAudioEndpointOffloadStreamVolume, GetVolumeChannelCount)
        HRESULT ( STDMETHODCALLTYPE *GetVolumeChannelCount )( 
            IAudioEndpointOffloadStreamVolume * This,
            /* [out] */ UINT32 *pu32ChannelCount);
        
        DECLSPEC_XFGVIRT(IAudioEndpointOffloadStreamVolume, SetChannelVolumes)
        HRESULT ( STDMETHODCALLTYPE *SetChannelVolumes )( 
            IAudioEndpointOffloadStreamVolume * This,
            /* [in] */ UINT32 u32ChannelCount,
            /* [in] */ FLOAT32 *pf32Volumes,
            /* [in] */ AUDIO_CURVE_TYPE u32CurveType,
            /* [in] */ HNSTIME *pCurveDuration);
        
        DECLSPEC_XFGVIRT(IAudioEndpointOffloadStreamVolume, GetChannelVolumes)
        HRESULT ( STDMETHODCALLTYPE *GetChannelVolumes )( 
            IAudioEndpointOffloadStreamVolume * This,
            /* [in] */ UINT32 u32ChannelCount,
            /* [out] */ FLOAT32 *pf32Volumes);
        
        END_INTERFACE
    } IAudioEndpointOffloadStreamVolumeVtbl;

    interface IAudioEndpointOffloadStreamVolume
    {
        CONST_VTBL struct IAudioEndpointOffloadStreamVolumeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAudioEndpointOffloadStreamVolume_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAudioEndpointOffloadStreamVolume_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAudioEndpointOffloadStreamVolume_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAudioEndpointOffloadStreamVolume_GetVolumeChannelCount(This,pu32ChannelCount)	\
    ( (This)->lpVtbl -> GetVolumeChannelCount(This,pu32ChannelCount) ) 

#define IAudioEndpointOffloadStreamVolume_SetChannelVolumes(This,u32ChannelCount,pf32Volumes,u32CurveType,pCurveDuration)	\
    ( (This)->lpVtbl -> SetChannelVolumes(This,u32ChannelCount,pf32Volumes,u32CurveType,pCurveDuration) ) 

#define IAudioEndpointOffloadStreamVolume_GetChannelVolumes(This,u32ChannelCount,pf32Volumes)	\
    ( (This)->lpVtbl -> GetChannelVolumes(This,u32ChannelCount,pf32Volumes) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAudioEndpointOffloadStreamVolume_INTERFACE_DEFINED__ */


#ifndef __IAudioEndpointOffloadStreamMute_INTERFACE_DEFINED__
#define __IAudioEndpointOffloadStreamMute_INTERFACE_DEFINED__

/* interface IAudioEndpointOffloadStreamMute */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IAudioEndpointOffloadStreamMute;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DFE21355-5EC2-40E0-8D6B-710AC3C00249")
    IAudioEndpointOffloadStreamMute : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetMute( 
            /* [in] */ boolean bMuted) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMute( 
            /* [out] */ boolean *pbMuted) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAudioEndpointOffloadStreamMuteVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAudioEndpointOffloadStreamMute * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAudioEndpointOffloadStreamMute * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAudioEndpointOffloadStreamMute * This);
        
        DECLSPEC_XFGVIRT(IAudioEndpointOffloadStreamMute, SetMute)
        HRESULT ( STDMETHODCALLTYPE *SetMute )( 
            IAudioEndpointOffloadStreamMute * This,
            /* [in] */ boolean bMuted);
        
        DECLSPEC_XFGVIRT(IAudioEndpointOffloadStreamMute, GetMute)
        HRESULT ( STDMETHODCALLTYPE *GetMute )( 
            IAudioEndpointOffloadStreamMute * This,
            /* [out] */ boolean *pbMuted);
        
        END_INTERFACE
    } IAudioEndpointOffloadStreamMuteVtbl;

    interface IAudioEndpointOffloadStreamMute
    {
        CONST_VTBL struct IAudioEndpointOffloadStreamMuteVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAudioEndpointOffloadStreamMute_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAudioEndpointOffloadStreamMute_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAudioEndpointOffloadStreamMute_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAudioEndpointOffloadStreamMute_SetMute(This,bMuted)	\
    ( (This)->lpVtbl -> SetMute(This,bMuted) ) 

#define IAudioEndpointOffloadStreamMute_GetMute(This,pbMuted)	\
    ( (This)->lpVtbl -> GetMute(This,pbMuted) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAudioEndpointOffloadStreamMute_INTERFACE_DEFINED__ */


#ifndef __IAudioEndpointOffloadStreamMeter_INTERFACE_DEFINED__
#define __IAudioEndpointOffloadStreamMeter_INTERFACE_DEFINED__

/* interface IAudioEndpointOffloadStreamMeter */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IAudioEndpointOffloadStreamMeter;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("E1546DCE-9DD1-418B-9AB2-348CED161C86")
    IAudioEndpointOffloadStreamMeter : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetMeterChannelCount( 
            /* [out] */ UINT32 *pu32ChannelCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMeteringData( 
            /* [in] */ UINT32 u32ChannelCount,
            /* [out] */ FLOAT32 *pf32PeakValues) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAudioEndpointOffloadStreamMeterVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAudioEndpointOffloadStreamMeter * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAudioEndpointOffloadStreamMeter * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAudioEndpointOffloadStreamMeter * This);
        
        DECLSPEC_XFGVIRT(IAudioEndpointOffloadStreamMeter, GetMeterChannelCount)
        HRESULT ( STDMETHODCALLTYPE *GetMeterChannelCount )( 
            IAudioEndpointOffloadStreamMeter * This,
            /* [out] */ UINT32 *pu32ChannelCount);
        
        DECLSPEC_XFGVIRT(IAudioEndpointOffloadStreamMeter, GetMeteringData)
        HRESULT ( STDMETHODCALLTYPE *GetMeteringData )( 
            IAudioEndpointOffloadStreamMeter * This,
            /* [in] */ UINT32 u32ChannelCount,
            /* [out] */ FLOAT32 *pf32PeakValues);
        
        END_INTERFACE
    } IAudioEndpointOffloadStreamMeterVtbl;

    interface IAudioEndpointOffloadStreamMeter
    {
        CONST_VTBL struct IAudioEndpointOffloadStreamMeterVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAudioEndpointOffloadStreamMeter_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAudioEndpointOffloadStreamMeter_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAudioEndpointOffloadStreamMeter_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAudioEndpointOffloadStreamMeter_GetMeterChannelCount(This,pu32ChannelCount)	\
    ( (This)->lpVtbl -> GetMeterChannelCount(This,pu32ChannelCount) ) 

#define IAudioEndpointOffloadStreamMeter_GetMeteringData(This,u32ChannelCount,pf32PeakValues)	\
    ( (This)->lpVtbl -> GetMeteringData(This,u32ChannelCount,pf32PeakValues) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAudioEndpointOffloadStreamMeter_INTERFACE_DEFINED__ */


#ifndef __IAudioEndpointLastBufferControl_INTERFACE_DEFINED__
#define __IAudioEndpointLastBufferControl_INTERFACE_DEFINED__

/* interface IAudioEndpointLastBufferControl */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IAudioEndpointLastBufferControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F8520DD3-8F9D-4437-9861-62F584C33DD6")
    IAudioEndpointLastBufferControl : public IUnknown
    {
    public:
        virtual BOOL STDMETHODCALLTYPE IsLastBufferControlSupported( void) = 0;
        
        virtual void STDMETHODCALLTYPE ReleaseOutputDataPointerForLastBuffer( 
            /* [in] */ const APO_CONNECTION_PROPERTY *pConnectionProperty) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAudioEndpointLastBufferControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAudioEndpointLastBufferControl * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAudioEndpointLastBufferControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAudioEndpointLastBufferControl * This);
        
        DECLSPEC_XFGVIRT(IAudioEndpointLastBufferControl, IsLastBufferControlSupported)
        BOOL ( STDMETHODCALLTYPE *IsLastBufferControlSupported )( 
            IAudioEndpointLastBufferControl * This);
        
        DECLSPEC_XFGVIRT(IAudioEndpointLastBufferControl, ReleaseOutputDataPointerForLastBuffer)
        void ( STDMETHODCALLTYPE *ReleaseOutputDataPointerForLastBuffer )( 
            IAudioEndpointLastBufferControl * This,
            /* [in] */ const APO_CONNECTION_PROPERTY *pConnectionProperty);
        
        END_INTERFACE
    } IAudioEndpointLastBufferControlVtbl;

    interface IAudioEndpointLastBufferControl
    {
        CONST_VTBL struct IAudioEndpointLastBufferControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAudioEndpointLastBufferControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAudioEndpointLastBufferControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAudioEndpointLastBufferControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAudioEndpointLastBufferControl_IsLastBufferControlSupported(This)	\
    ( (This)->lpVtbl -> IsLastBufferControlSupported(This) ) 

#define IAudioEndpointLastBufferControl_ReleaseOutputDataPointerForLastBuffer(This,pConnectionProperty)	\
    ( (This)->lpVtbl -> ReleaseOutputDataPointerForLastBuffer(This,pConnectionProperty) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAudioEndpointLastBufferControl_INTERFACE_DEFINED__ */


#ifndef __IAudioLfxControl_INTERFACE_DEFINED__
#define __IAudioLfxControl_INTERFACE_DEFINED__

/* interface IAudioLfxControl */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IAudioLfxControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("076A6922-D802-4F83-BAF6-409D9CA11BFE")
    IAudioLfxControl : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetLocalEffectsState( 
            /* [in] */ BOOL bEnabled) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLocalEffectsState( 
            /* [out] */ BOOL *pbEnabled) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAudioLfxControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAudioLfxControl * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAudioLfxControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAudioLfxControl * This);
        
        DECLSPEC_XFGVIRT(IAudioLfxControl, SetLocalEffectsState)
        HRESULT ( STDMETHODCALLTYPE *SetLocalEffectsState )( 
            IAudioLfxControl * This,
            /* [in] */ BOOL bEnabled);
        
        DECLSPEC_XFGVIRT(IAudioLfxControl, GetLocalEffectsState)
        HRESULT ( STDMETHODCALLTYPE *GetLocalEffectsState )( 
            IAudioLfxControl * This,
            /* [out] */ BOOL *pbEnabled);
        
        END_INTERFACE
    } IAudioLfxControlVtbl;

    interface IAudioLfxControl
    {
        CONST_VTBL struct IAudioLfxControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAudioLfxControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAudioLfxControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAudioLfxControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAudioLfxControl_SetLocalEffectsState(This,bEnabled)	\
    ( (This)->lpVtbl -> SetLocalEffectsState(This,bEnabled) ) 

#define IAudioLfxControl_GetLocalEffectsState(This,pbEnabled)	\
    ( (This)->lpVtbl -> GetLocalEffectsState(This,pbEnabled) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAudioLfxControl_INTERFACE_DEFINED__ */


#ifndef __IHardwareAudioEngineBase_INTERFACE_DEFINED__
#define __IHardwareAudioEngineBase_INTERFACE_DEFINED__

/* interface IHardwareAudioEngineBase */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IHardwareAudioEngineBase;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("EDDCE3E4-F3C1-453a-B461-223563CBD886")
    IHardwareAudioEngineBase : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetAvailableOffloadConnectorCount( 
            /* [annotation][in] */ 
            _In_  LPWSTR _pwstrDeviceId,
            /* [annotation][in] */ 
            _In_  UINT32 _uConnectorId,
            /* [annotation][out] */ 
            _Out_  UINT32 *_pAvailableConnectorInstanceCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEngineFormat( 
            /* [annotation][in] */ 
            _In_  IMMDevice *pDevice,
            /* [annotation][in] */ 
            _In_  BOOL _bRequestDeviceFormat,
            /* [out] */ WAVEFORMATEX **_ppwfxFormat) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetEngineDeviceFormat( 
            /* [annotation][in] */ 
            _In_  IMMDevice *pDevice,
            /* [in] */ WAVEFORMATEX *_pwfxFormat) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetGfxState( 
            /* [annotation][in] */ 
            _In_  IMMDevice *pDevice,
            /* [annotation][in] */ 
            _In_  BOOL _bEnable) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetGfxState( 
            /* [annotation][in] */ 
            _In_  IMMDevice *pDevice,
            /* [annotation][out] */ 
            _Out_  BOOL *_pbEnable) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IHardwareAudioEngineBaseVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IHardwareAudioEngineBase * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IHardwareAudioEngineBase * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IHardwareAudioEngineBase * This);
        
        DECLSPEC_XFGVIRT(IHardwareAudioEngineBase, GetAvailableOffloadConnectorCount)
        HRESULT ( STDMETHODCALLTYPE *GetAvailableOffloadConnectorCount )( 
            IHardwareAudioEngineBase * This,
            /* [annotation][in] */ 
            _In_  LPWSTR _pwstrDeviceId,
            /* [annotation][in] */ 
            _In_  UINT32 _uConnectorId,
            /* [annotation][out] */ 
            _Out_  UINT32 *_pAvailableConnectorInstanceCount);
        
        DECLSPEC_XFGVIRT(IHardwareAudioEngineBase, GetEngineFormat)
        HRESULT ( STDMETHODCALLTYPE *GetEngineFormat )( 
            IHardwareAudioEngineBase * This,
            /* [annotation][in] */ 
            _In_  IMMDevice *pDevice,
            /* [annotation][in] */ 
            _In_  BOOL _bRequestDeviceFormat,
            /* [out] */ WAVEFORMATEX **_ppwfxFormat);
        
        DECLSPEC_XFGVIRT(IHardwareAudioEngineBase, SetEngineDeviceFormat)
        HRESULT ( STDMETHODCALLTYPE *SetEngineDeviceFormat )( 
            IHardwareAudioEngineBase * This,
            /* [annotation][in] */ 
            _In_  IMMDevice *pDevice,
            /* [in] */ WAVEFORMATEX *_pwfxFormat);
        
        DECLSPEC_XFGVIRT(IHardwareAudioEngineBase, SetGfxState)
        HRESULT ( STDMETHODCALLTYPE *SetGfxState )( 
            IHardwareAudioEngineBase * This,
            /* [annotation][in] */ 
            _In_  IMMDevice *pDevice,
            /* [annotation][in] */ 
            _In_  BOOL _bEnable);
        
        DECLSPEC_XFGVIRT(IHardwareAudioEngineBase, GetGfxState)
        HRESULT ( STDMETHODCALLTYPE *GetGfxState )( 
            IHardwareAudioEngineBase * This,
            /* [annotation][in] */ 
            _In_  IMMDevice *pDevice,
            /* [annotation][out] */ 
            _Out_  BOOL *_pbEnable);
        
        END_INTERFACE
    } IHardwareAudioEngineBaseVtbl;

    interface IHardwareAudioEngineBase
    {
        CONST_VTBL struct IHardwareAudioEngineBaseVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IHardwareAudioEngineBase_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IHardwareAudioEngineBase_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IHardwareAudioEngineBase_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IHardwareAudioEngineBase_GetAvailableOffloadConnectorCount(This,_pwstrDeviceId,_uConnectorId,_pAvailableConnectorInstanceCount)	\
    ( (This)->lpVtbl -> GetAvailableOffloadConnectorCount(This,_pwstrDeviceId,_uConnectorId,_pAvailableConnectorInstanceCount) ) 

#define IHardwareAudioEngineBase_GetEngineFormat(This,pDevice,_bRequestDeviceFormat,_ppwfxFormat)	\
    ( (This)->lpVtbl -> GetEngineFormat(This,pDevice,_bRequestDeviceFormat,_ppwfxFormat) ) 

#define IHardwareAudioEngineBase_SetEngineDeviceFormat(This,pDevice,_pwfxFormat)	\
    ( (This)->lpVtbl -> SetEngineDeviceFormat(This,pDevice,_pwfxFormat) ) 

#define IHardwareAudioEngineBase_SetGfxState(This,pDevice,_bEnable)	\
    ( (This)->lpVtbl -> SetGfxState(This,pDevice,_bEnable) ) 

#define IHardwareAudioEngineBase_GetGfxState(This,pDevice,_pbEnable)	\
    ( (This)->lpVtbl -> GetGfxState(This,pDevice,_pbEnable) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IHardwareAudioEngineBase_INTERFACE_DEFINED__ */


#ifndef __IAudioEndpointControl_INTERFACE_DEFINED__
#define __IAudioEndpointControl_INTERFACE_DEFINED__

/* interface IAudioEndpointControl */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IAudioEndpointControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C684B72A-6DF4-4774-BDF9-76B77509B653")
    IAudioEndpointControl : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Start( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Stop( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAudioEndpointControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAudioEndpointControl * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAudioEndpointControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAudioEndpointControl * This);
        
        DECLSPEC_XFGVIRT(IAudioEndpointControl, Start)
        HRESULT ( STDMETHODCALLTYPE *Start )( 
            __RPC__in IAudioEndpointControl * This);
        
        DECLSPEC_XFGVIRT(IAudioEndpointControl, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IAudioEndpointControl * This);
        
        DECLSPEC_XFGVIRT(IAudioEndpointControl, Stop)
        HRESULT ( STDMETHODCALLTYPE *Stop )( 
            __RPC__in IAudioEndpointControl * This);
        
        END_INTERFACE
    } IAudioEndpointControlVtbl;

    interface IAudioEndpointControl
    {
        CONST_VTBL struct IAudioEndpointControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAudioEndpointControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAudioEndpointControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAudioEndpointControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAudioEndpointControl_Start(This)	\
    ( (This)->lpVtbl -> Start(This) ) 

#define IAudioEndpointControl_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IAudioEndpointControl_Stop(This)	\
    ( (This)->lpVtbl -> Stop(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAudioEndpointControl_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_audioengineendpoint_0000_0012 */
/* [local] */ 

#define STATIC_DEVINTERFACE_AUDIOENDPOINTPLUGIN                                                       \
    0X9F2F7B66l, 0X65AC, 0X4FA6, 0X8A, 0XE4, 0X12, 0X3C, 0X78, 0XB8, 0X93, 0X13                         
DEFINE_GUIDSTRUCT("9F2F7B66-65AC-4FA6-8AE4-123C78B89313", DEVINTERFACE_AUDIOENDPOINTPLUGIN);          
#define DEVINTERFACE_AUDIOENDPOINTPLUGIN DEFINE_GUIDNAMED(DEVINTERFACE_AUDIOENDPOINTPLUGIN)             
DEFINE_PROPERTYKEY(DEVPKEY_AudioEndpointPlugin_FactoryCLSID,   0x12d83bd7, 0xcf12, 0x46be, 0x85, 0x40, 0x81, 0x27, 0x10, 0xd3, 0x2, 0x1c, 1); 
DEFINE_PROPERTYKEY(DEVPKEY_AudioEndpointPlugin_DataFlow,       0x12d83bd7, 0xcf12, 0x46be, 0x85, 0x40, 0x81, 0x27, 0x10, 0xd3, 0x2, 0x1c, 2); 
DEFINE_PROPERTYKEY(DEVPKEY_AudioEndpointPlugin_PnPInterface,   0x12d83bd7, 0xcf12, 0x46be, 0x85, 0x40, 0x81, 0x27, 0x10, 0xd3, 0x2, 0x1c, 3); 
DEFINE_PROPERTYKEY(DEVPKEY_AudioEndpointPlugin2_FactoryCLSID,   0x12d83bd7, 0xcf12, 0x46be, 0x85, 0x40, 0x81, 0x27, 0x10, 0xd3, 0x2, 0x1c, 4); 
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_audioengineendpoint_0000_0012_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_audioengineendpoint_0000_0012_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif



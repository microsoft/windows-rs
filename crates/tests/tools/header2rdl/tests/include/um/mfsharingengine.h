

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

#ifndef __mfsharingengine_h__
#define __mfsharingengine_h__

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

#ifndef __IMFSharingEngineClassFactory_FWD_DEFINED__
#define __IMFSharingEngineClassFactory_FWD_DEFINED__
typedef interface IMFSharingEngineClassFactory IMFSharingEngineClassFactory;

#endif 	/* __IMFSharingEngineClassFactory_FWD_DEFINED__ */


#ifndef __IMFMediaSharingEngine_FWD_DEFINED__
#define __IMFMediaSharingEngine_FWD_DEFINED__
typedef interface IMFMediaSharingEngine IMFMediaSharingEngine;

#endif 	/* __IMFMediaSharingEngine_FWD_DEFINED__ */


#ifndef __IMFMediaSharingEngineClassFactory_FWD_DEFINED__
#define __IMFMediaSharingEngineClassFactory_FWD_DEFINED__
typedef interface IMFMediaSharingEngineClassFactory IMFMediaSharingEngineClassFactory;

#endif 	/* __IMFMediaSharingEngineClassFactory_FWD_DEFINED__ */


#ifndef __IMFImageSharingEngine_FWD_DEFINED__
#define __IMFImageSharingEngine_FWD_DEFINED__
typedef interface IMFImageSharingEngine IMFImageSharingEngine;

#endif 	/* __IMFImageSharingEngine_FWD_DEFINED__ */


#ifndef __IMFImageSharingEngineClassFactory_FWD_DEFINED__
#define __IMFImageSharingEngineClassFactory_FWD_DEFINED__
typedef interface IMFImageSharingEngineClassFactory IMFImageSharingEngineClassFactory;

#endif 	/* __IMFImageSharingEngineClassFactory_FWD_DEFINED__ */


#ifndef __IPlayToControl_FWD_DEFINED__
#define __IPlayToControl_FWD_DEFINED__
typedef interface IPlayToControl IPlayToControl;

#endif 	/* __IPlayToControl_FWD_DEFINED__ */


#ifndef __IPlayToControlWithCapabilities_FWD_DEFINED__
#define __IPlayToControlWithCapabilities_FWD_DEFINED__
typedef interface IPlayToControlWithCapabilities IPlayToControlWithCapabilities;

#endif 	/* __IPlayToControlWithCapabilities_FWD_DEFINED__ */


#ifndef __IPlayToSourceClassFactory_FWD_DEFINED__
#define __IPlayToSourceClassFactory_FWD_DEFINED__
typedef interface IPlayToSourceClassFactory IPlayToSourceClassFactory;

#endif 	/* __IPlayToSourceClassFactory_FWD_DEFINED__ */


/* header files for imported files */
#include "MFMediaEngine.h"
#include "inspectable.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_mfsharingengine_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#if (WINVER >= _WIN32_WINNT_WIN8) 
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
typedef struct DEVICE_INFO
    {
    BSTR pFriendlyDeviceName;
    BSTR pUniqueDeviceName;
    BSTR pManufacturerName;
    BSTR pModelName;
    BSTR pIconURL;
    } 	DEVICE_INFO;

typedef 
enum MF_SHARING_ENGINE_EVENT
    {
        MF_SHARING_ENGINE_EVENT_DISCONNECT	= 2000,
        MF_SHARING_ENGINE_EVENT_LOCALRENDERINGSTARTED	= 2001,
        MF_SHARING_ENGINE_EVENT_LOCALRENDERINGENDED	= 2002,
        MF_SHARING_ENGINE_EVENT_STOPPED	= 2003,
        MF_SHARING_ENGINE_EVENT_ERROR	= 2501
    } 	MF_SHARING_ENGINE_EVENT;

typedef 
enum MF_MEDIA_SHARING_ENGINE_EVENT
    {
        MF_MEDIA_SHARING_ENGINE_EVENT_DISCONNECT	= 2000
    } 	MF_MEDIA_SHARING_ENGINE_EVENT;



extern RPC_IF_HANDLE __MIDL_itf_mfsharingengine_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfsharingengine_0000_0000_v0_0_s_ifspec;

#ifndef __IMFSharingEngineClassFactory_INTERFACE_DEFINED__
#define __IMFSharingEngineClassFactory_INTERFACE_DEFINED__

/* interface IMFSharingEngineClassFactory */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IMFSharingEngineClassFactory;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2BA61F92-8305-413B-9733-FAF15F259384")
    IMFSharingEngineClassFactory : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateInstance( 
            /* [annotation][in] */ 
            _In_  DWORD dwFlags,
            /* [annotation][in] */ 
            _In_opt_  IMFAttributes *pAttr,
            /* [annotation][out] */ 
            _COM_Outptr_  IUnknown **ppEngine) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFSharingEngineClassFactoryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFSharingEngineClassFactory * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFSharingEngineClassFactory * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFSharingEngineClassFactory * This);
        
        DECLSPEC_XFGVIRT(IMFSharingEngineClassFactory, CreateInstance)
        HRESULT ( STDMETHODCALLTYPE *CreateInstance )( 
            IMFSharingEngineClassFactory * This,
            /* [annotation][in] */ 
            _In_  DWORD dwFlags,
            /* [annotation][in] */ 
            _In_opt_  IMFAttributes *pAttr,
            /* [annotation][out] */ 
            _COM_Outptr_  IUnknown **ppEngine);
        
        END_INTERFACE
    } IMFSharingEngineClassFactoryVtbl;

    interface IMFSharingEngineClassFactory
    {
        CONST_VTBL struct IMFSharingEngineClassFactoryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFSharingEngineClassFactory_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFSharingEngineClassFactory_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFSharingEngineClassFactory_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFSharingEngineClassFactory_CreateInstance(This,dwFlags,pAttr,ppEngine)	\
    ( (This)->lpVtbl -> CreateInstance(This,dwFlags,pAttr,ppEngine) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFSharingEngineClassFactory_INTERFACE_DEFINED__ */


#ifndef __IMFMediaSharingEngine_INTERFACE_DEFINED__
#define __IMFMediaSharingEngine_INTERFACE_DEFINED__

/* interface IMFMediaSharingEngine */
/* [local][unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMFMediaSharingEngine;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8D3CE1BF-2367-40E0-9EEE-40D377CC1B46")
    IMFMediaSharingEngine : public IMFMediaEngine
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetDevice( 
            /* [annotation][out] */ 
            _Out_  DEVICE_INFO *pDevice) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFMediaSharingEngineVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFMediaSharingEngine * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFMediaSharingEngine * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFMediaSharingEngine * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, GetError)
        HRESULT ( STDMETHODCALLTYPE *GetError )( 
            IMFMediaSharingEngine * This,
            /* [annotation][out] */ 
            _Outptr_  IMFMediaError **ppError);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, SetErrorCode)
        HRESULT ( STDMETHODCALLTYPE *SetErrorCode )( 
            IMFMediaSharingEngine * This,
            /* [annotation][in] */ 
            _In_  MF_MEDIA_ENGINE_ERR error);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, SetSourceElements)
        HRESULT ( STDMETHODCALLTYPE *SetSourceElements )( 
            IMFMediaSharingEngine * This,
            /* [annotation][in] */ 
            _In_  IMFMediaEngineSrcElements *pSrcElements);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, SetSource)
        HRESULT ( STDMETHODCALLTYPE *SetSource )( 
            IMFMediaSharingEngine * This,
            /* [annotation][in] */ 
            _In_  BSTR pUrl);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, GetCurrentSource)
        HRESULT ( STDMETHODCALLTYPE *GetCurrentSource )( 
            IMFMediaSharingEngine * This,
            /* [annotation][out] */ 
            _Out_  BSTR *ppUrl);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, GetNetworkState)
        USHORT ( STDMETHODCALLTYPE *GetNetworkState )( 
            IMFMediaSharingEngine * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, GetPreload)
        MF_MEDIA_ENGINE_PRELOAD ( STDMETHODCALLTYPE *GetPreload )( 
            IMFMediaSharingEngine * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, SetPreload)
        HRESULT ( STDMETHODCALLTYPE *SetPreload )( 
            IMFMediaSharingEngine * This,
            /* [annotation][in] */ 
            _In_  MF_MEDIA_ENGINE_PRELOAD Preload);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, GetBuffered)
        HRESULT ( STDMETHODCALLTYPE *GetBuffered )( 
            IMFMediaSharingEngine * This,
            /* [annotation][out] */ 
            _Outptr_  IMFMediaTimeRange **ppBuffered);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, Load)
        HRESULT ( STDMETHODCALLTYPE *Load )( 
            IMFMediaSharingEngine * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, CanPlayType)
        HRESULT ( STDMETHODCALLTYPE *CanPlayType )( 
            IMFMediaSharingEngine * This,
            /* [annotation][in] */ 
            _In_  BSTR type,
            /* [annotation][out] */ 
            _Out_  MF_MEDIA_ENGINE_CANPLAY *pAnswer);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, GetReadyState)
        USHORT ( STDMETHODCALLTYPE *GetReadyState )( 
            IMFMediaSharingEngine * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, IsSeeking)
        BOOL ( STDMETHODCALLTYPE *IsSeeking )( 
            IMFMediaSharingEngine * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, GetCurrentTime)
        double ( STDMETHODCALLTYPE *GetCurrentTime )( 
            IMFMediaSharingEngine * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, SetCurrentTime)
        HRESULT ( STDMETHODCALLTYPE *SetCurrentTime )( 
            IMFMediaSharingEngine * This,
            /* [annotation][in] */ 
            _In_  double seekTime);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, GetStartTime)
        double ( STDMETHODCALLTYPE *GetStartTime )( 
            IMFMediaSharingEngine * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, GetDuration)
        double ( STDMETHODCALLTYPE *GetDuration )( 
            IMFMediaSharingEngine * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, IsPaused)
        BOOL ( STDMETHODCALLTYPE *IsPaused )( 
            IMFMediaSharingEngine * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, GetDefaultPlaybackRate)
        double ( STDMETHODCALLTYPE *GetDefaultPlaybackRate )( 
            IMFMediaSharingEngine * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, SetDefaultPlaybackRate)
        HRESULT ( STDMETHODCALLTYPE *SetDefaultPlaybackRate )( 
            IMFMediaSharingEngine * This,
            /* [annotation][in] */ 
            _In_  double Rate);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, GetPlaybackRate)
        double ( STDMETHODCALLTYPE *GetPlaybackRate )( 
            IMFMediaSharingEngine * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, SetPlaybackRate)
        HRESULT ( STDMETHODCALLTYPE *SetPlaybackRate )( 
            IMFMediaSharingEngine * This,
            /* [annotation][in] */ 
            _In_  double Rate);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, GetPlayed)
        HRESULT ( STDMETHODCALLTYPE *GetPlayed )( 
            IMFMediaSharingEngine * This,
            /* [annotation][out] */ 
            _Outptr_  IMFMediaTimeRange **ppPlayed);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, GetSeekable)
        HRESULT ( STDMETHODCALLTYPE *GetSeekable )( 
            IMFMediaSharingEngine * This,
            /* [annotation][out] */ 
            _Outptr_  IMFMediaTimeRange **ppSeekable);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, IsEnded)
        BOOL ( STDMETHODCALLTYPE *IsEnded )( 
            IMFMediaSharingEngine * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, GetAutoPlay)
        BOOL ( STDMETHODCALLTYPE *GetAutoPlay )( 
            IMFMediaSharingEngine * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, SetAutoPlay)
        HRESULT ( STDMETHODCALLTYPE *SetAutoPlay )( 
            IMFMediaSharingEngine * This,
            /* [annotation][in] */ 
            _In_  BOOL AutoPlay);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, GetLoop)
        BOOL ( STDMETHODCALLTYPE *GetLoop )( 
            IMFMediaSharingEngine * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, SetLoop)
        HRESULT ( STDMETHODCALLTYPE *SetLoop )( 
            IMFMediaSharingEngine * This,
            /* [annotation][in] */ 
            _In_  BOOL Loop);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, Play)
        HRESULT ( STDMETHODCALLTYPE *Play )( 
            IMFMediaSharingEngine * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, Pause)
        HRESULT ( STDMETHODCALLTYPE *Pause )( 
            IMFMediaSharingEngine * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, GetMuted)
        BOOL ( STDMETHODCALLTYPE *GetMuted )( 
            IMFMediaSharingEngine * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, SetMuted)
        HRESULT ( STDMETHODCALLTYPE *SetMuted )( 
            IMFMediaSharingEngine * This,
            /* [annotation][in] */ 
            _In_  BOOL Muted);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, GetVolume)
        double ( STDMETHODCALLTYPE *GetVolume )( 
            IMFMediaSharingEngine * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, SetVolume)
        HRESULT ( STDMETHODCALLTYPE *SetVolume )( 
            IMFMediaSharingEngine * This,
            /* [annotation][in] */ 
            _In_  double Volume);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, HasVideo)
        BOOL ( STDMETHODCALLTYPE *HasVideo )( 
            IMFMediaSharingEngine * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, HasAudio)
        BOOL ( STDMETHODCALLTYPE *HasAudio )( 
            IMFMediaSharingEngine * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, GetNativeVideoSize)
        HRESULT ( STDMETHODCALLTYPE *GetNativeVideoSize )( 
            IMFMediaSharingEngine * This,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *cx,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *cy);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, GetVideoAspectRatio)
        HRESULT ( STDMETHODCALLTYPE *GetVideoAspectRatio )( 
            IMFMediaSharingEngine * This,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *cx,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *cy);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, Shutdown)
        HRESULT ( STDMETHODCALLTYPE *Shutdown )( 
            IMFMediaSharingEngine * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, TransferVideoFrame)
        HRESULT ( STDMETHODCALLTYPE *TransferVideoFrame )( 
            IMFMediaSharingEngine * This,
            /* [annotation][in] */ 
            _In_  IUnknown *pDstSurf,
            /* [annotation][in] */ 
            _In_opt_  const MFVideoNormalizedRect *pSrc,
            /* [annotation][in] */ 
            _In_  const RECT *pDst,
            /* [annotation][in] */ 
            _In_opt_  const MFARGB *pBorderClr);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, OnVideoStreamTick)
        HRESULT ( STDMETHODCALLTYPE *OnVideoStreamTick )( 
            IMFMediaSharingEngine * This,
            /* [annotation][out] */ 
            _Out_  LONGLONG *pPts);
        
        DECLSPEC_XFGVIRT(IMFMediaSharingEngine, GetDevice)
        HRESULT ( STDMETHODCALLTYPE *GetDevice )( 
            IMFMediaSharingEngine * This,
            /* [annotation][out] */ 
            _Out_  DEVICE_INFO *pDevice);
        
        END_INTERFACE
    } IMFMediaSharingEngineVtbl;

    interface IMFMediaSharingEngine
    {
        CONST_VTBL struct IMFMediaSharingEngineVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFMediaSharingEngine_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFMediaSharingEngine_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFMediaSharingEngine_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFMediaSharingEngine_GetError(This,ppError)	\
    ( (This)->lpVtbl -> GetError(This,ppError) ) 

#define IMFMediaSharingEngine_SetErrorCode(This,error)	\
    ( (This)->lpVtbl -> SetErrorCode(This,error) ) 

#define IMFMediaSharingEngine_SetSourceElements(This,pSrcElements)	\
    ( (This)->lpVtbl -> SetSourceElements(This,pSrcElements) ) 

#define IMFMediaSharingEngine_SetSource(This,pUrl)	\
    ( (This)->lpVtbl -> SetSource(This,pUrl) ) 

#define IMFMediaSharingEngine_GetCurrentSource(This,ppUrl)	\
    ( (This)->lpVtbl -> GetCurrentSource(This,ppUrl) ) 

#define IMFMediaSharingEngine_GetNetworkState(This)	\
    ( (This)->lpVtbl -> GetNetworkState(This) ) 

#define IMFMediaSharingEngine_GetPreload(This)	\
    ( (This)->lpVtbl -> GetPreload(This) ) 

#define IMFMediaSharingEngine_SetPreload(This,Preload)	\
    ( (This)->lpVtbl -> SetPreload(This,Preload) ) 

#define IMFMediaSharingEngine_GetBuffered(This,ppBuffered)	\
    ( (This)->lpVtbl -> GetBuffered(This,ppBuffered) ) 

#define IMFMediaSharingEngine_Load(This)	\
    ( (This)->lpVtbl -> Load(This) ) 

#define IMFMediaSharingEngine_CanPlayType(This,type,pAnswer)	\
    ( (This)->lpVtbl -> CanPlayType(This,type,pAnswer) ) 

#define IMFMediaSharingEngine_GetReadyState(This)	\
    ( (This)->lpVtbl -> GetReadyState(This) ) 

#define IMFMediaSharingEngine_IsSeeking(This)	\
    ( (This)->lpVtbl -> IsSeeking(This) ) 

#define IMFMediaSharingEngine_GetCurrentTime(This)	\
    ( (This)->lpVtbl -> GetCurrentTime(This) ) 

#define IMFMediaSharingEngine_SetCurrentTime(This,seekTime)	\
    ( (This)->lpVtbl -> SetCurrentTime(This,seekTime) ) 

#define IMFMediaSharingEngine_GetStartTime(This)	\
    ( (This)->lpVtbl -> GetStartTime(This) ) 

#define IMFMediaSharingEngine_GetDuration(This)	\
    ( (This)->lpVtbl -> GetDuration(This) ) 

#define IMFMediaSharingEngine_IsPaused(This)	\
    ( (This)->lpVtbl -> IsPaused(This) ) 

#define IMFMediaSharingEngine_GetDefaultPlaybackRate(This)	\
    ( (This)->lpVtbl -> GetDefaultPlaybackRate(This) ) 

#define IMFMediaSharingEngine_SetDefaultPlaybackRate(This,Rate)	\
    ( (This)->lpVtbl -> SetDefaultPlaybackRate(This,Rate) ) 

#define IMFMediaSharingEngine_GetPlaybackRate(This)	\
    ( (This)->lpVtbl -> GetPlaybackRate(This) ) 

#define IMFMediaSharingEngine_SetPlaybackRate(This,Rate)	\
    ( (This)->lpVtbl -> SetPlaybackRate(This,Rate) ) 

#define IMFMediaSharingEngine_GetPlayed(This,ppPlayed)	\
    ( (This)->lpVtbl -> GetPlayed(This,ppPlayed) ) 

#define IMFMediaSharingEngine_GetSeekable(This,ppSeekable)	\
    ( (This)->lpVtbl -> GetSeekable(This,ppSeekable) ) 

#define IMFMediaSharingEngine_IsEnded(This)	\
    ( (This)->lpVtbl -> IsEnded(This) ) 

#define IMFMediaSharingEngine_GetAutoPlay(This)	\
    ( (This)->lpVtbl -> GetAutoPlay(This) ) 

#define IMFMediaSharingEngine_SetAutoPlay(This,AutoPlay)	\
    ( (This)->lpVtbl -> SetAutoPlay(This,AutoPlay) ) 

#define IMFMediaSharingEngine_GetLoop(This)	\
    ( (This)->lpVtbl -> GetLoop(This) ) 

#define IMFMediaSharingEngine_SetLoop(This,Loop)	\
    ( (This)->lpVtbl -> SetLoop(This,Loop) ) 

#define IMFMediaSharingEngine_Play(This)	\
    ( (This)->lpVtbl -> Play(This) ) 

#define IMFMediaSharingEngine_Pause(This)	\
    ( (This)->lpVtbl -> Pause(This) ) 

#define IMFMediaSharingEngine_GetMuted(This)	\
    ( (This)->lpVtbl -> GetMuted(This) ) 

#define IMFMediaSharingEngine_SetMuted(This,Muted)	\
    ( (This)->lpVtbl -> SetMuted(This,Muted) ) 

#define IMFMediaSharingEngine_GetVolume(This)	\
    ( (This)->lpVtbl -> GetVolume(This) ) 

#define IMFMediaSharingEngine_SetVolume(This,Volume)	\
    ( (This)->lpVtbl -> SetVolume(This,Volume) ) 

#define IMFMediaSharingEngine_HasVideo(This)	\
    ( (This)->lpVtbl -> HasVideo(This) ) 

#define IMFMediaSharingEngine_HasAudio(This)	\
    ( (This)->lpVtbl -> HasAudio(This) ) 

#define IMFMediaSharingEngine_GetNativeVideoSize(This,cx,cy)	\
    ( (This)->lpVtbl -> GetNativeVideoSize(This,cx,cy) ) 

#define IMFMediaSharingEngine_GetVideoAspectRatio(This,cx,cy)	\
    ( (This)->lpVtbl -> GetVideoAspectRatio(This,cx,cy) ) 

#define IMFMediaSharingEngine_Shutdown(This)	\
    ( (This)->lpVtbl -> Shutdown(This) ) 

#define IMFMediaSharingEngine_TransferVideoFrame(This,pDstSurf,pSrc,pDst,pBorderClr)	\
    ( (This)->lpVtbl -> TransferVideoFrame(This,pDstSurf,pSrc,pDst,pBorderClr) ) 

#define IMFMediaSharingEngine_OnVideoStreamTick(This,pPts)	\
    ( (This)->lpVtbl -> OnVideoStreamTick(This,pPts) ) 


#define IMFMediaSharingEngine_GetDevice(This,pDevice)	\
    ( (This)->lpVtbl -> GetDevice(This,pDevice) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFMediaSharingEngine_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfsharingengine_0000_0002 */
/* [local] */ 

EXTERN_GUID(MF_MEDIA_SHARING_ENGINE_DEVICE_NAME,
0x771e05d1, 0x862f, 0x4299, 0x95, 0xac, 0xae, 0x81, 0xfd, 0x14, 0xf3, 0xe7);
EXTERN_GUID(MF_MEDIA_SHARING_ENGINE_DEVICE,
0xb461c58a, 0x7a08, 0x4b98, 0x99, 0xa8, 0x70, 0xfd, 0x5f, 0x3b, 0xad, 0xfd);
DEFINE_GUID(MF_MEDIA_SHARING_ENGINE_INITIAL_SEEK_TIME, 
0x6f3497f5, 0xd528, 0x4a4f, 0x8d, 0xd7, 0xdb, 0x36, 0x65, 0x7e, 0xc4, 0xc9);
DEFINE_GUID(MF_SHUTDOWN_RENDERER_ON_ENGINE_SHUTDOWN, 
0xc112d94d, 0x6b9c, 0x48f8, 0xb6, 0xf9, 0x79, 0x50, 0xff, 0x9a, 0xb7, 0x1e);
DEFINE_GUID(MF_PREFERRED_SOURCE_URI, 
0x5fc85488, 0x436a, 0x4db8, 0x90, 0xaf, 0x4d, 0xb4, 0x2, 0xae, 0x5c, 0x57);
DEFINE_GUID(MF_SHARING_ENGINE_SHAREDRENDERER, 
0xefa446a0, 0x73e7, 0x404e, 0x8a, 0xe2, 0xfe, 0xf6, 0x0a, 0xf5, 0xa3, 0x2b);
DEFINE_GUID(MF_SHARING_ENGINE_CALLBACK, 
0x57dc1e95, 0xd252, 0x43fa, 0x9b, 0xbc, 0x18, 0x0, 0x70, 0xee, 0xfe, 0x6d);


extern RPC_IF_HANDLE __MIDL_itf_mfsharingengine_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfsharingengine_0000_0002_v0_0_s_ifspec;

#ifndef __IMFMediaSharingEngineClassFactory_INTERFACE_DEFINED__
#define __IMFMediaSharingEngineClassFactory_INTERFACE_DEFINED__

/* interface IMFMediaSharingEngineClassFactory */
/* [local][unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMFMediaSharingEngineClassFactory;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("524D2BC4-B2B1-4FE5-8FAC-FA4E4512B4E0")
    IMFMediaSharingEngineClassFactory : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateInstance( 
            /* [annotation][in] */ 
            _In_  DWORD dwFlags,
            /* [annotation][in] */ 
            _In_  IMFAttributes *pAttr,
            /* [annotation][out] */ 
            _Outptr_  IMFMediaSharingEngine **ppEngine) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFMediaSharingEngineClassFactoryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFMediaSharingEngineClassFactory * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFMediaSharingEngineClassFactory * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFMediaSharingEngineClassFactory * This);
        
        DECLSPEC_XFGVIRT(IMFMediaSharingEngineClassFactory, CreateInstance)
        HRESULT ( STDMETHODCALLTYPE *CreateInstance )( 
            IMFMediaSharingEngineClassFactory * This,
            /* [annotation][in] */ 
            _In_  DWORD dwFlags,
            /* [annotation][in] */ 
            _In_  IMFAttributes *pAttr,
            /* [annotation][out] */ 
            _Outptr_  IMFMediaSharingEngine **ppEngine);
        
        END_INTERFACE
    } IMFMediaSharingEngineClassFactoryVtbl;

    interface IMFMediaSharingEngineClassFactory
    {
        CONST_VTBL struct IMFMediaSharingEngineClassFactoryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFMediaSharingEngineClassFactory_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFMediaSharingEngineClassFactory_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFMediaSharingEngineClassFactory_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFMediaSharingEngineClassFactory_CreateInstance(This,dwFlags,pAttr,ppEngine)	\
    ( (This)->lpVtbl -> CreateInstance(This,dwFlags,pAttr,ppEngine) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFMediaSharingEngineClassFactory_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfsharingengine_0000_0003 */
/* [local] */ 

EXTERN_GUID(CLSID_MFMediaSharingEngineClassFactory, 0xf8e307fb, 0x6d45, 0x4ad3, 0x99, 0x93, 0x66, 0xcd, 0x5a, 0x52, 0x96, 0x59);


extern RPC_IF_HANDLE __MIDL_itf_mfsharingengine_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfsharingengine_0000_0003_v0_0_s_ifspec;

#ifndef __IMFImageSharingEngine_INTERFACE_DEFINED__
#define __IMFImageSharingEngine_INTERFACE_DEFINED__

/* interface IMFImageSharingEngine */
/* [local][unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMFImageSharingEngine;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("CFA0AE8E-7E1C-44D2-AE68-FC4C148A6354")
    IMFImageSharingEngine : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetSource( 
            /* [annotation][in] */ 
            _In_  IUnknown *pStream) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDevice( 
            /* [annotation][out] */ 
            _Out_  DEVICE_INFO *pDevice) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Shutdown( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFImageSharingEngineVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFImageSharingEngine * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFImageSharingEngine * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFImageSharingEngine * This);
        
        DECLSPEC_XFGVIRT(IMFImageSharingEngine, SetSource)
        HRESULT ( STDMETHODCALLTYPE *SetSource )( 
            IMFImageSharingEngine * This,
            /* [annotation][in] */ 
            _In_  IUnknown *pStream);
        
        DECLSPEC_XFGVIRT(IMFImageSharingEngine, GetDevice)
        HRESULT ( STDMETHODCALLTYPE *GetDevice )( 
            IMFImageSharingEngine * This,
            /* [annotation][out] */ 
            _Out_  DEVICE_INFO *pDevice);
        
        DECLSPEC_XFGVIRT(IMFImageSharingEngine, Shutdown)
        HRESULT ( STDMETHODCALLTYPE *Shutdown )( 
            IMFImageSharingEngine * This);
        
        END_INTERFACE
    } IMFImageSharingEngineVtbl;

    interface IMFImageSharingEngine
    {
        CONST_VTBL struct IMFImageSharingEngineVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFImageSharingEngine_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFImageSharingEngine_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFImageSharingEngine_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFImageSharingEngine_SetSource(This,pStream)	\
    ( (This)->lpVtbl -> SetSource(This,pStream) ) 

#define IMFImageSharingEngine_GetDevice(This,pDevice)	\
    ( (This)->lpVtbl -> GetDevice(This,pDevice) ) 

#define IMFImageSharingEngine_Shutdown(This)	\
    ( (This)->lpVtbl -> Shutdown(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFImageSharingEngine_INTERFACE_DEFINED__ */


#ifndef __IMFImageSharingEngineClassFactory_INTERFACE_DEFINED__
#define __IMFImageSharingEngineClassFactory_INTERFACE_DEFINED__

/* interface IMFImageSharingEngineClassFactory */
/* [local][unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMFImageSharingEngineClassFactory;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1FC55727-A7FB-4FC8-83AE-8AF024990AF1")
    IMFImageSharingEngineClassFactory : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateInstanceFromUDN( 
            /* [annotation][in] */ 
            _In_  BSTR pUniqueDeviceName,
            /* [annotation][out] */ 
            _Outptr_  IMFImageSharingEngine **ppEngine) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFImageSharingEngineClassFactoryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFImageSharingEngineClassFactory * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFImageSharingEngineClassFactory * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFImageSharingEngineClassFactory * This);
        
        DECLSPEC_XFGVIRT(IMFImageSharingEngineClassFactory, CreateInstanceFromUDN)
        HRESULT ( STDMETHODCALLTYPE *CreateInstanceFromUDN )( 
            IMFImageSharingEngineClassFactory * This,
            /* [annotation][in] */ 
            _In_  BSTR pUniqueDeviceName,
            /* [annotation][out] */ 
            _Outptr_  IMFImageSharingEngine **ppEngine);
        
        END_INTERFACE
    } IMFImageSharingEngineClassFactoryVtbl;

    interface IMFImageSharingEngineClassFactory
    {
        CONST_VTBL struct IMFImageSharingEngineClassFactoryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFImageSharingEngineClassFactory_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFImageSharingEngineClassFactory_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFImageSharingEngineClassFactory_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFImageSharingEngineClassFactory_CreateInstanceFromUDN(This,pUniqueDeviceName,ppEngine)	\
    ( (This)->lpVtbl -> CreateInstanceFromUDN(This,pUniqueDeviceName,ppEngine) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFImageSharingEngineClassFactory_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfsharingengine_0000_0005 */
/* [local] */ 

EXTERN_GUID(CLSID_MFImageSharingEngineClassFactory, 0xb22c3339, 0x87f3, 0x4059, 0xa0, 0xc5, 0x3, 0x7a, 0xa9, 0x70, 0x7e, 0xaf);
typedef /* [v1_enum] */ 
enum PLAYTO_SOURCE_CREATEFLAGS
    {
        PLAYTO_SOURCE_NONE	= 0,
        PLAYTO_SOURCE_IMAGE	= 0x1,
        PLAYTO_SOURCE_AUDIO	= 0x2,
        PLAYTO_SOURCE_VIDEO	= 0x4,
        PLAYTO_SOURCE_PROTECTED	= 0x8
    } 	PLAYTO_SOURCE_CREATEFLAGS;

DEFINE_ENUM_FLAG_OPERATORS(PLAYTO_SOURCE_CREATEFLAGS)


extern RPC_IF_HANDLE __MIDL_itf_mfsharingengine_0000_0005_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfsharingengine_0000_0005_v0_0_s_ifspec;

#ifndef __IPlayToControl_INTERFACE_DEFINED__
#define __IPlayToControl_INTERFACE_DEFINED__

/* interface IPlayToControl */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IPlayToControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("607574EB-F4B6-45C1-B08C-CB715122901D")
    IPlayToControl : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Connect( 
            /* [in] */ __RPC__in_opt IMFSharingEngineClassFactory *pFactory) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Disconnect( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPlayToControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPlayToControl * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPlayToControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPlayToControl * This);
        
        DECLSPEC_XFGVIRT(IPlayToControl, Connect)
        HRESULT ( STDMETHODCALLTYPE *Connect )( 
            __RPC__in IPlayToControl * This,
            /* [in] */ __RPC__in_opt IMFSharingEngineClassFactory *pFactory);
        
        DECLSPEC_XFGVIRT(IPlayToControl, Disconnect)
        HRESULT ( STDMETHODCALLTYPE *Disconnect )( 
            __RPC__in IPlayToControl * This);
        
        END_INTERFACE
    } IPlayToControlVtbl;

    interface IPlayToControl
    {
        CONST_VTBL struct IPlayToControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPlayToControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPlayToControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPlayToControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPlayToControl_Connect(This,pFactory)	\
    ( (This)->lpVtbl -> Connect(This,pFactory) ) 

#define IPlayToControl_Disconnect(This)	\
    ( (This)->lpVtbl -> Disconnect(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPlayToControl_INTERFACE_DEFINED__ */


#ifndef __IPlayToControlWithCapabilities_INTERFACE_DEFINED__
#define __IPlayToControlWithCapabilities_INTERFACE_DEFINED__

/* interface IPlayToControlWithCapabilities */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IPlayToControlWithCapabilities;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("AA9DD80F-C50A-4220-91C1-332287F82A34")
    IPlayToControlWithCapabilities : public IPlayToControl
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCapabilities( 
            /* [out] */ __RPC__out PLAYTO_SOURCE_CREATEFLAGS *pCapabilities) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPlayToControlWithCapabilitiesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPlayToControlWithCapabilities * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPlayToControlWithCapabilities * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPlayToControlWithCapabilities * This);
        
        DECLSPEC_XFGVIRT(IPlayToControl, Connect)
        HRESULT ( STDMETHODCALLTYPE *Connect )( 
            __RPC__in IPlayToControlWithCapabilities * This,
            /* [in] */ __RPC__in_opt IMFSharingEngineClassFactory *pFactory);
        
        DECLSPEC_XFGVIRT(IPlayToControl, Disconnect)
        HRESULT ( STDMETHODCALLTYPE *Disconnect )( 
            __RPC__in IPlayToControlWithCapabilities * This);
        
        DECLSPEC_XFGVIRT(IPlayToControlWithCapabilities, GetCapabilities)
        HRESULT ( STDMETHODCALLTYPE *GetCapabilities )( 
            __RPC__in IPlayToControlWithCapabilities * This,
            /* [out] */ __RPC__out PLAYTO_SOURCE_CREATEFLAGS *pCapabilities);
        
        END_INTERFACE
    } IPlayToControlWithCapabilitiesVtbl;

    interface IPlayToControlWithCapabilities
    {
        CONST_VTBL struct IPlayToControlWithCapabilitiesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPlayToControlWithCapabilities_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPlayToControlWithCapabilities_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPlayToControlWithCapabilities_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPlayToControlWithCapabilities_Connect(This,pFactory)	\
    ( (This)->lpVtbl -> Connect(This,pFactory) ) 

#define IPlayToControlWithCapabilities_Disconnect(This)	\
    ( (This)->lpVtbl -> Disconnect(This) ) 


#define IPlayToControlWithCapabilities_GetCapabilities(This,pCapabilities)	\
    ( (This)->lpVtbl -> GetCapabilities(This,pCapabilities) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPlayToControlWithCapabilities_INTERFACE_DEFINED__ */


#ifndef __IPlayToSourceClassFactory_INTERFACE_DEFINED__
#define __IPlayToSourceClassFactory_INTERFACE_DEFINED__

/* interface IPlayToSourceClassFactory */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IPlayToSourceClassFactory;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("842B32A3-9B9B-4D1C-B3F3-49193248A554")
    IPlayToSourceClassFactory : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateInstance( 
            /* [annotation][in] */ 
            _In_  DWORD dwFlags,
            /* [annotation][in] */ 
            _In_  IPlayToControl *pControl,
            /* [annotation][out] */ 
            _Outptr_  IInspectable **ppSource) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPlayToSourceClassFactoryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPlayToSourceClassFactory * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPlayToSourceClassFactory * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPlayToSourceClassFactory * This);
        
        DECLSPEC_XFGVIRT(IPlayToSourceClassFactory, CreateInstance)
        HRESULT ( STDMETHODCALLTYPE *CreateInstance )( 
            IPlayToSourceClassFactory * This,
            /* [annotation][in] */ 
            _In_  DWORD dwFlags,
            /* [annotation][in] */ 
            _In_  IPlayToControl *pControl,
            /* [annotation][out] */ 
            _Outptr_  IInspectable **ppSource);
        
        END_INTERFACE
    } IPlayToSourceClassFactoryVtbl;

    interface IPlayToSourceClassFactory
    {
        CONST_VTBL struct IPlayToSourceClassFactoryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPlayToSourceClassFactory_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPlayToSourceClassFactory_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPlayToSourceClassFactory_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPlayToSourceClassFactory_CreateInstance(This,dwFlags,pControl,ppSource)	\
    ( (This)->lpVtbl -> CreateInstance(This,dwFlags,pControl,ppSource) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPlayToSourceClassFactory_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfsharingengine_0000_0008 */
/* [local] */ 

EXTERN_GUID(CLSID_PlayToSourceClassFactory, 0xDA17539A, 0x3DC3, 0x42C1, 0xA7, 0x49, 0xA1, 0x83, 0xB5, 0x1F, 0x08, 0x5E);
EXTERN_GUID(GUID_PlayToService, 0xf6a8ff9d, 0x9e14, 0x41c9, 0xbf, 0x0f, 0x12, 0x0a, 0x2b, 0x3c, 0xe1, 0x20);
EXTERN_GUID(GUID_NativeDeviceService, 0xef71e53c, 0x52f4, 0x43c5, 0xb8, 0x6a, 0xad, 0x6c, 0xb2, 0x16, 0xa6, 0x1e);
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#endif // (WINVER >= _WIN32_WINNT_WIN8) 


extern RPC_IF_HANDLE __MIDL_itf_mfsharingengine_0000_0008_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfsharingengine_0000_0008_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif



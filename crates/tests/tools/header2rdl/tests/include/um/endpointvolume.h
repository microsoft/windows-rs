

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

#ifndef __endpointvolume_h__
#define __endpointvolume_h__

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

#ifndef __IAudioEndpointVolumeCallback_FWD_DEFINED__
#define __IAudioEndpointVolumeCallback_FWD_DEFINED__
typedef interface IAudioEndpointVolumeCallback IAudioEndpointVolumeCallback;

#endif 	/* __IAudioEndpointVolumeCallback_FWD_DEFINED__ */


#ifndef __IAudioEndpointVolume_FWD_DEFINED__
#define __IAudioEndpointVolume_FWD_DEFINED__
typedef interface IAudioEndpointVolume IAudioEndpointVolume;

#endif 	/* __IAudioEndpointVolume_FWD_DEFINED__ */


#ifndef __IAudioEndpointVolumeEx_FWD_DEFINED__
#define __IAudioEndpointVolumeEx_FWD_DEFINED__
typedef interface IAudioEndpointVolumeEx IAudioEndpointVolumeEx;

#endif 	/* __IAudioEndpointVolumeEx_FWD_DEFINED__ */


#ifndef __IAudioMeterInformation_FWD_DEFINED__
#define __IAudioMeterInformation_FWD_DEFINED__
typedef interface IAudioMeterInformation IAudioMeterInformation;

#endif 	/* __IAudioMeterInformation_FWD_DEFINED__ */


/* header files for imported files */
#include "unknwn.h"
#include "devicetopology.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_endpointvolume_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
typedef struct AUDIO_VOLUME_NOTIFICATION_DATA
    {
    GUID guidEventContext;
    BOOL bMuted;
    float fMasterVolume;
    UINT nChannels;
    float afChannelVolumes[ 1 ];
    } 	AUDIO_VOLUME_NOTIFICATION_DATA;

typedef struct AUDIO_VOLUME_NOTIFICATION_DATA *PAUDIO_VOLUME_NOTIFICATION_DATA;

#define   ENDPOINT_HARDWARE_SUPPORT_VOLUME    0x00000001
#define   ENDPOINT_HARDWARE_SUPPORT_MUTE      0x00000002
#define   ENDPOINT_HARDWARE_SUPPORT_METER     0x00000004


extern RPC_IF_HANDLE __MIDL_itf_endpointvolume_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_endpointvolume_0000_0000_v0_0_s_ifspec;

#ifndef __IAudioEndpointVolumeCallback_INTERFACE_DEFINED__
#define __IAudioEndpointVolumeCallback_INTERFACE_DEFINED__

/* interface IAudioEndpointVolumeCallback */
/* [unique][helpstring][nonextensible][uuid][local][object] */ 


EXTERN_C const IID IID_IAudioEndpointVolumeCallback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("657804FA-D6AD-4496-8A60-352752AF4F89")
    IAudioEndpointVolumeCallback : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnNotify( 
            PAUDIO_VOLUME_NOTIFICATION_DATA pNotify) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAudioEndpointVolumeCallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAudioEndpointVolumeCallback * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAudioEndpointVolumeCallback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAudioEndpointVolumeCallback * This);
        
        DECLSPEC_XFGVIRT(IAudioEndpointVolumeCallback, OnNotify)
        HRESULT ( STDMETHODCALLTYPE *OnNotify )( 
            IAudioEndpointVolumeCallback * This,
            PAUDIO_VOLUME_NOTIFICATION_DATA pNotify);
        
        END_INTERFACE
    } IAudioEndpointVolumeCallbackVtbl;

    interface IAudioEndpointVolumeCallback
    {
        CONST_VTBL struct IAudioEndpointVolumeCallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAudioEndpointVolumeCallback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAudioEndpointVolumeCallback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAudioEndpointVolumeCallback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAudioEndpointVolumeCallback_OnNotify(This,pNotify)	\
    ( (This)->lpVtbl -> OnNotify(This,pNotify) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAudioEndpointVolumeCallback_INTERFACE_DEFINED__ */


#ifndef __IAudioEndpointVolume_INTERFACE_DEFINED__
#define __IAudioEndpointVolume_INTERFACE_DEFINED__

/* interface IAudioEndpointVolume */
/* [unique][helpstring][nonextensible][uuid][local][object] */ 


EXTERN_C const IID IID_IAudioEndpointVolume;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5CDF2C82-841E-4546-9722-0CF74078229A")
    IAudioEndpointVolume : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE RegisterControlChangeNotify( 
            /* [annotation][in] */ 
            _In_  IAudioEndpointVolumeCallback *pNotify) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE UnregisterControlChangeNotify( 
            /* [annotation][in] */ 
            _In_  IAudioEndpointVolumeCallback *pNotify) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetChannelCount( 
            /* [annotation][out] */ 
            _Out_  UINT *pnChannelCount) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetMasterVolumeLevel( 
            /* [annotation][in] */ 
            _In_  float fLevelDB,
            /* [unique][in] */ LPCGUID pguidEventContext) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetMasterVolumeLevelScalar( 
            /* [annotation][in] */ 
            _In_  float fLevel,
            /* [unique][in] */ LPCGUID pguidEventContext) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetMasterVolumeLevel( 
            /* [annotation][out] */ 
            _Out_  float *pfLevelDB) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetMasterVolumeLevelScalar( 
            /* [annotation][out] */ 
            _Out_  float *pfLevel) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetChannelVolumeLevel( 
            /* [annotation][in] */ 
            _In_  UINT nChannel,
            float fLevelDB,
            /* [unique][in] */ LPCGUID pguidEventContext) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetChannelVolumeLevelScalar( 
            /* [annotation][in] */ 
            _In_  UINT nChannel,
            float fLevel,
            /* [unique][in] */ LPCGUID pguidEventContext) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetChannelVolumeLevel( 
            /* [annotation][in] */ 
            _In_  UINT nChannel,
            /* [annotation][out] */ 
            _Out_  float *pfLevelDB) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetChannelVolumeLevelScalar( 
            /* [annotation][in] */ 
            _In_  UINT nChannel,
            /* [annotation][out] */ 
            _Out_  float *pfLevel) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetMute( 
            /* [annotation][in] */ 
            _In_  BOOL bMute,
            /* [unique][in] */ LPCGUID pguidEventContext) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetMute( 
            /* [annotation][out] */ 
            _Out_  BOOL *pbMute) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetVolumeStepInfo( 
            /* [annotation][out] */ 
            _Out_  UINT *pnStep,
            /* [annotation][out] */ 
            _Out_  UINT *pnStepCount) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE VolumeStepUp( 
            /* [unique][in] */ LPCGUID pguidEventContext) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE VolumeStepDown( 
            /* [unique][in] */ LPCGUID pguidEventContext) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE QueryHardwareSupport( 
            /* [annotation][out] */ 
            _Out_  DWORD *pdwHardwareSupportMask) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetVolumeRange( 
            /* [annotation][out] */ 
            _Out_  float *pflVolumeMindB,
            /* [annotation][out] */ 
            _Out_  float *pflVolumeMaxdB,
            /* [annotation][out] */ 
            _Out_  float *pflVolumeIncrementdB) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAudioEndpointVolumeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAudioEndpointVolume * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAudioEndpointVolume * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAudioEndpointVolume * This);
        
        DECLSPEC_XFGVIRT(IAudioEndpointVolume, RegisterControlChangeNotify)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *RegisterControlChangeNotify )( 
            IAudioEndpointVolume * This,
            /* [annotation][in] */ 
            _In_  IAudioEndpointVolumeCallback *pNotify);
        
        DECLSPEC_XFGVIRT(IAudioEndpointVolume, UnregisterControlChangeNotify)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *UnregisterControlChangeNotify )( 
            IAudioEndpointVolume * This,
            /* [annotation][in] */ 
            _In_  IAudioEndpointVolumeCallback *pNotify);
        
        DECLSPEC_XFGVIRT(IAudioEndpointVolume, GetChannelCount)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetChannelCount )( 
            IAudioEndpointVolume * This,
            /* [annotation][out] */ 
            _Out_  UINT *pnChannelCount);
        
        DECLSPEC_XFGVIRT(IAudioEndpointVolume, SetMasterVolumeLevel)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetMasterVolumeLevel )( 
            IAudioEndpointVolume * This,
            /* [annotation][in] */ 
            _In_  float fLevelDB,
            /* [unique][in] */ LPCGUID pguidEventContext);
        
        DECLSPEC_XFGVIRT(IAudioEndpointVolume, SetMasterVolumeLevelScalar)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetMasterVolumeLevelScalar )( 
            IAudioEndpointVolume * This,
            /* [annotation][in] */ 
            _In_  float fLevel,
            /* [unique][in] */ LPCGUID pguidEventContext);
        
        DECLSPEC_XFGVIRT(IAudioEndpointVolume, GetMasterVolumeLevel)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetMasterVolumeLevel )( 
            IAudioEndpointVolume * This,
            /* [annotation][out] */ 
            _Out_  float *pfLevelDB);
        
        DECLSPEC_XFGVIRT(IAudioEndpointVolume, GetMasterVolumeLevelScalar)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetMasterVolumeLevelScalar )( 
            IAudioEndpointVolume * This,
            /* [annotation][out] */ 
            _Out_  float *pfLevel);
        
        DECLSPEC_XFGVIRT(IAudioEndpointVolume, SetChannelVolumeLevel)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetChannelVolumeLevel )( 
            IAudioEndpointVolume * This,
            /* [annotation][in] */ 
            _In_  UINT nChannel,
            float fLevelDB,
            /* [unique][in] */ LPCGUID pguidEventContext);
        
        DECLSPEC_XFGVIRT(IAudioEndpointVolume, SetChannelVolumeLevelScalar)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetChannelVolumeLevelScalar )( 
            IAudioEndpointVolume * This,
            /* [annotation][in] */ 
            _In_  UINT nChannel,
            float fLevel,
            /* [unique][in] */ LPCGUID pguidEventContext);
        
        DECLSPEC_XFGVIRT(IAudioEndpointVolume, GetChannelVolumeLevel)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetChannelVolumeLevel )( 
            IAudioEndpointVolume * This,
            /* [annotation][in] */ 
            _In_  UINT nChannel,
            /* [annotation][out] */ 
            _Out_  float *pfLevelDB);
        
        DECLSPEC_XFGVIRT(IAudioEndpointVolume, GetChannelVolumeLevelScalar)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetChannelVolumeLevelScalar )( 
            IAudioEndpointVolume * This,
            /* [annotation][in] */ 
            _In_  UINT nChannel,
            /* [annotation][out] */ 
            _Out_  float *pfLevel);
        
        DECLSPEC_XFGVIRT(IAudioEndpointVolume, SetMute)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetMute )( 
            IAudioEndpointVolume * This,
            /* [annotation][in] */ 
            _In_  BOOL bMute,
            /* [unique][in] */ LPCGUID pguidEventContext);
        
        DECLSPEC_XFGVIRT(IAudioEndpointVolume, GetMute)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetMute )( 
            IAudioEndpointVolume * This,
            /* [annotation][out] */ 
            _Out_  BOOL *pbMute);
        
        DECLSPEC_XFGVIRT(IAudioEndpointVolume, GetVolumeStepInfo)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetVolumeStepInfo )( 
            IAudioEndpointVolume * This,
            /* [annotation][out] */ 
            _Out_  UINT *pnStep,
            /* [annotation][out] */ 
            _Out_  UINT *pnStepCount);
        
        DECLSPEC_XFGVIRT(IAudioEndpointVolume, VolumeStepUp)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *VolumeStepUp )( 
            IAudioEndpointVolume * This,
            /* [unique][in] */ LPCGUID pguidEventContext);
        
        DECLSPEC_XFGVIRT(IAudioEndpointVolume, VolumeStepDown)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *VolumeStepDown )( 
            IAudioEndpointVolume * This,
            /* [unique][in] */ LPCGUID pguidEventContext);
        
        DECLSPEC_XFGVIRT(IAudioEndpointVolume, QueryHardwareSupport)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *QueryHardwareSupport )( 
            IAudioEndpointVolume * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwHardwareSupportMask);
        
        DECLSPEC_XFGVIRT(IAudioEndpointVolume, GetVolumeRange)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetVolumeRange )( 
            IAudioEndpointVolume * This,
            /* [annotation][out] */ 
            _Out_  float *pflVolumeMindB,
            /* [annotation][out] */ 
            _Out_  float *pflVolumeMaxdB,
            /* [annotation][out] */ 
            _Out_  float *pflVolumeIncrementdB);
        
        END_INTERFACE
    } IAudioEndpointVolumeVtbl;

    interface IAudioEndpointVolume
    {
        CONST_VTBL struct IAudioEndpointVolumeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAudioEndpointVolume_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAudioEndpointVolume_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAudioEndpointVolume_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAudioEndpointVolume_RegisterControlChangeNotify(This,pNotify)	\
    ( (This)->lpVtbl -> RegisterControlChangeNotify(This,pNotify) ) 

#define IAudioEndpointVolume_UnregisterControlChangeNotify(This,pNotify)	\
    ( (This)->lpVtbl -> UnregisterControlChangeNotify(This,pNotify) ) 

#define IAudioEndpointVolume_GetChannelCount(This,pnChannelCount)	\
    ( (This)->lpVtbl -> GetChannelCount(This,pnChannelCount) ) 

#define IAudioEndpointVolume_SetMasterVolumeLevel(This,fLevelDB,pguidEventContext)	\
    ( (This)->lpVtbl -> SetMasterVolumeLevel(This,fLevelDB,pguidEventContext) ) 

#define IAudioEndpointVolume_SetMasterVolumeLevelScalar(This,fLevel,pguidEventContext)	\
    ( (This)->lpVtbl -> SetMasterVolumeLevelScalar(This,fLevel,pguidEventContext) ) 

#define IAudioEndpointVolume_GetMasterVolumeLevel(This,pfLevelDB)	\
    ( (This)->lpVtbl -> GetMasterVolumeLevel(This,pfLevelDB) ) 

#define IAudioEndpointVolume_GetMasterVolumeLevelScalar(This,pfLevel)	\
    ( (This)->lpVtbl -> GetMasterVolumeLevelScalar(This,pfLevel) ) 

#define IAudioEndpointVolume_SetChannelVolumeLevel(This,nChannel,fLevelDB,pguidEventContext)	\
    ( (This)->lpVtbl -> SetChannelVolumeLevel(This,nChannel,fLevelDB,pguidEventContext) ) 

#define IAudioEndpointVolume_SetChannelVolumeLevelScalar(This,nChannel,fLevel,pguidEventContext)	\
    ( (This)->lpVtbl -> SetChannelVolumeLevelScalar(This,nChannel,fLevel,pguidEventContext) ) 

#define IAudioEndpointVolume_GetChannelVolumeLevel(This,nChannel,pfLevelDB)	\
    ( (This)->lpVtbl -> GetChannelVolumeLevel(This,nChannel,pfLevelDB) ) 

#define IAudioEndpointVolume_GetChannelVolumeLevelScalar(This,nChannel,pfLevel)	\
    ( (This)->lpVtbl -> GetChannelVolumeLevelScalar(This,nChannel,pfLevel) ) 

#define IAudioEndpointVolume_SetMute(This,bMute,pguidEventContext)	\
    ( (This)->lpVtbl -> SetMute(This,bMute,pguidEventContext) ) 

#define IAudioEndpointVolume_GetMute(This,pbMute)	\
    ( (This)->lpVtbl -> GetMute(This,pbMute) ) 

#define IAudioEndpointVolume_GetVolumeStepInfo(This,pnStep,pnStepCount)	\
    ( (This)->lpVtbl -> GetVolumeStepInfo(This,pnStep,pnStepCount) ) 

#define IAudioEndpointVolume_VolumeStepUp(This,pguidEventContext)	\
    ( (This)->lpVtbl -> VolumeStepUp(This,pguidEventContext) ) 

#define IAudioEndpointVolume_VolumeStepDown(This,pguidEventContext)	\
    ( (This)->lpVtbl -> VolumeStepDown(This,pguidEventContext) ) 

#define IAudioEndpointVolume_QueryHardwareSupport(This,pdwHardwareSupportMask)	\
    ( (This)->lpVtbl -> QueryHardwareSupport(This,pdwHardwareSupportMask) ) 

#define IAudioEndpointVolume_GetVolumeRange(This,pflVolumeMindB,pflVolumeMaxdB,pflVolumeIncrementdB)	\
    ( (This)->lpVtbl -> GetVolumeRange(This,pflVolumeMindB,pflVolumeMaxdB,pflVolumeIncrementdB) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAudioEndpointVolume_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_endpointvolume_0000_0002 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_endpointvolume_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_endpointvolume_0000_0002_v0_0_s_ifspec;

#ifndef __IAudioEndpointVolumeEx_INTERFACE_DEFINED__
#define __IAudioEndpointVolumeEx_INTERFACE_DEFINED__

/* interface IAudioEndpointVolumeEx */
/* [unique][helpstring][nonextensible][uuid][local][object] */ 


EXTERN_C const IID IID_IAudioEndpointVolumeEx;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("66E11784-F695-4F28-A505-A7080081A78F")
    IAudioEndpointVolumeEx : public IAudioEndpointVolume
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetVolumeRangeChannel( 
            /* [in] */ UINT iChannel,
            /* [annotation][out] */ 
            _Out_  float *pflVolumeMindB,
            /* [annotation][out] */ 
            _Out_  float *pflVolumeMaxdB,
            /* [annotation][out] */ 
            _Out_  float *pflVolumeIncrementdB) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAudioEndpointVolumeExVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAudioEndpointVolumeEx * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAudioEndpointVolumeEx * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAudioEndpointVolumeEx * This);
        
        DECLSPEC_XFGVIRT(IAudioEndpointVolume, RegisterControlChangeNotify)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *RegisterControlChangeNotify )( 
            IAudioEndpointVolumeEx * This,
            /* [annotation][in] */ 
            _In_  IAudioEndpointVolumeCallback *pNotify);
        
        DECLSPEC_XFGVIRT(IAudioEndpointVolume, UnregisterControlChangeNotify)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *UnregisterControlChangeNotify )( 
            IAudioEndpointVolumeEx * This,
            /* [annotation][in] */ 
            _In_  IAudioEndpointVolumeCallback *pNotify);
        
        DECLSPEC_XFGVIRT(IAudioEndpointVolume, GetChannelCount)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetChannelCount )( 
            IAudioEndpointVolumeEx * This,
            /* [annotation][out] */ 
            _Out_  UINT *pnChannelCount);
        
        DECLSPEC_XFGVIRT(IAudioEndpointVolume, SetMasterVolumeLevel)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetMasterVolumeLevel )( 
            IAudioEndpointVolumeEx * This,
            /* [annotation][in] */ 
            _In_  float fLevelDB,
            /* [unique][in] */ LPCGUID pguidEventContext);
        
        DECLSPEC_XFGVIRT(IAudioEndpointVolume, SetMasterVolumeLevelScalar)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetMasterVolumeLevelScalar )( 
            IAudioEndpointVolumeEx * This,
            /* [annotation][in] */ 
            _In_  float fLevel,
            /* [unique][in] */ LPCGUID pguidEventContext);
        
        DECLSPEC_XFGVIRT(IAudioEndpointVolume, GetMasterVolumeLevel)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetMasterVolumeLevel )( 
            IAudioEndpointVolumeEx * This,
            /* [annotation][out] */ 
            _Out_  float *pfLevelDB);
        
        DECLSPEC_XFGVIRT(IAudioEndpointVolume, GetMasterVolumeLevelScalar)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetMasterVolumeLevelScalar )( 
            IAudioEndpointVolumeEx * This,
            /* [annotation][out] */ 
            _Out_  float *pfLevel);
        
        DECLSPEC_XFGVIRT(IAudioEndpointVolume, SetChannelVolumeLevel)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetChannelVolumeLevel )( 
            IAudioEndpointVolumeEx * This,
            /* [annotation][in] */ 
            _In_  UINT nChannel,
            float fLevelDB,
            /* [unique][in] */ LPCGUID pguidEventContext);
        
        DECLSPEC_XFGVIRT(IAudioEndpointVolume, SetChannelVolumeLevelScalar)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetChannelVolumeLevelScalar )( 
            IAudioEndpointVolumeEx * This,
            /* [annotation][in] */ 
            _In_  UINT nChannel,
            float fLevel,
            /* [unique][in] */ LPCGUID pguidEventContext);
        
        DECLSPEC_XFGVIRT(IAudioEndpointVolume, GetChannelVolumeLevel)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetChannelVolumeLevel )( 
            IAudioEndpointVolumeEx * This,
            /* [annotation][in] */ 
            _In_  UINT nChannel,
            /* [annotation][out] */ 
            _Out_  float *pfLevelDB);
        
        DECLSPEC_XFGVIRT(IAudioEndpointVolume, GetChannelVolumeLevelScalar)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetChannelVolumeLevelScalar )( 
            IAudioEndpointVolumeEx * This,
            /* [annotation][in] */ 
            _In_  UINT nChannel,
            /* [annotation][out] */ 
            _Out_  float *pfLevel);
        
        DECLSPEC_XFGVIRT(IAudioEndpointVolume, SetMute)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetMute )( 
            IAudioEndpointVolumeEx * This,
            /* [annotation][in] */ 
            _In_  BOOL bMute,
            /* [unique][in] */ LPCGUID pguidEventContext);
        
        DECLSPEC_XFGVIRT(IAudioEndpointVolume, GetMute)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetMute )( 
            IAudioEndpointVolumeEx * This,
            /* [annotation][out] */ 
            _Out_  BOOL *pbMute);
        
        DECLSPEC_XFGVIRT(IAudioEndpointVolume, GetVolumeStepInfo)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetVolumeStepInfo )( 
            IAudioEndpointVolumeEx * This,
            /* [annotation][out] */ 
            _Out_  UINT *pnStep,
            /* [annotation][out] */ 
            _Out_  UINT *pnStepCount);
        
        DECLSPEC_XFGVIRT(IAudioEndpointVolume, VolumeStepUp)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *VolumeStepUp )( 
            IAudioEndpointVolumeEx * This,
            /* [unique][in] */ LPCGUID pguidEventContext);
        
        DECLSPEC_XFGVIRT(IAudioEndpointVolume, VolumeStepDown)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *VolumeStepDown )( 
            IAudioEndpointVolumeEx * This,
            /* [unique][in] */ LPCGUID pguidEventContext);
        
        DECLSPEC_XFGVIRT(IAudioEndpointVolume, QueryHardwareSupport)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *QueryHardwareSupport )( 
            IAudioEndpointVolumeEx * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwHardwareSupportMask);
        
        DECLSPEC_XFGVIRT(IAudioEndpointVolume, GetVolumeRange)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetVolumeRange )( 
            IAudioEndpointVolumeEx * This,
            /* [annotation][out] */ 
            _Out_  float *pflVolumeMindB,
            /* [annotation][out] */ 
            _Out_  float *pflVolumeMaxdB,
            /* [annotation][out] */ 
            _Out_  float *pflVolumeIncrementdB);
        
        DECLSPEC_XFGVIRT(IAudioEndpointVolumeEx, GetVolumeRangeChannel)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetVolumeRangeChannel )( 
            IAudioEndpointVolumeEx * This,
            /* [in] */ UINT iChannel,
            /* [annotation][out] */ 
            _Out_  float *pflVolumeMindB,
            /* [annotation][out] */ 
            _Out_  float *pflVolumeMaxdB,
            /* [annotation][out] */ 
            _Out_  float *pflVolumeIncrementdB);
        
        END_INTERFACE
    } IAudioEndpointVolumeExVtbl;

    interface IAudioEndpointVolumeEx
    {
        CONST_VTBL struct IAudioEndpointVolumeExVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAudioEndpointVolumeEx_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAudioEndpointVolumeEx_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAudioEndpointVolumeEx_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAudioEndpointVolumeEx_RegisterControlChangeNotify(This,pNotify)	\
    ( (This)->lpVtbl -> RegisterControlChangeNotify(This,pNotify) ) 

#define IAudioEndpointVolumeEx_UnregisterControlChangeNotify(This,pNotify)	\
    ( (This)->lpVtbl -> UnregisterControlChangeNotify(This,pNotify) ) 

#define IAudioEndpointVolumeEx_GetChannelCount(This,pnChannelCount)	\
    ( (This)->lpVtbl -> GetChannelCount(This,pnChannelCount) ) 

#define IAudioEndpointVolumeEx_SetMasterVolumeLevel(This,fLevelDB,pguidEventContext)	\
    ( (This)->lpVtbl -> SetMasterVolumeLevel(This,fLevelDB,pguidEventContext) ) 

#define IAudioEndpointVolumeEx_SetMasterVolumeLevelScalar(This,fLevel,pguidEventContext)	\
    ( (This)->lpVtbl -> SetMasterVolumeLevelScalar(This,fLevel,pguidEventContext) ) 

#define IAudioEndpointVolumeEx_GetMasterVolumeLevel(This,pfLevelDB)	\
    ( (This)->lpVtbl -> GetMasterVolumeLevel(This,pfLevelDB) ) 

#define IAudioEndpointVolumeEx_GetMasterVolumeLevelScalar(This,pfLevel)	\
    ( (This)->lpVtbl -> GetMasterVolumeLevelScalar(This,pfLevel) ) 

#define IAudioEndpointVolumeEx_SetChannelVolumeLevel(This,nChannel,fLevelDB,pguidEventContext)	\
    ( (This)->lpVtbl -> SetChannelVolumeLevel(This,nChannel,fLevelDB,pguidEventContext) ) 

#define IAudioEndpointVolumeEx_SetChannelVolumeLevelScalar(This,nChannel,fLevel,pguidEventContext)	\
    ( (This)->lpVtbl -> SetChannelVolumeLevelScalar(This,nChannel,fLevel,pguidEventContext) ) 

#define IAudioEndpointVolumeEx_GetChannelVolumeLevel(This,nChannel,pfLevelDB)	\
    ( (This)->lpVtbl -> GetChannelVolumeLevel(This,nChannel,pfLevelDB) ) 

#define IAudioEndpointVolumeEx_GetChannelVolumeLevelScalar(This,nChannel,pfLevel)	\
    ( (This)->lpVtbl -> GetChannelVolumeLevelScalar(This,nChannel,pfLevel) ) 

#define IAudioEndpointVolumeEx_SetMute(This,bMute,pguidEventContext)	\
    ( (This)->lpVtbl -> SetMute(This,bMute,pguidEventContext) ) 

#define IAudioEndpointVolumeEx_GetMute(This,pbMute)	\
    ( (This)->lpVtbl -> GetMute(This,pbMute) ) 

#define IAudioEndpointVolumeEx_GetVolumeStepInfo(This,pnStep,pnStepCount)	\
    ( (This)->lpVtbl -> GetVolumeStepInfo(This,pnStep,pnStepCount) ) 

#define IAudioEndpointVolumeEx_VolumeStepUp(This,pguidEventContext)	\
    ( (This)->lpVtbl -> VolumeStepUp(This,pguidEventContext) ) 

#define IAudioEndpointVolumeEx_VolumeStepDown(This,pguidEventContext)	\
    ( (This)->lpVtbl -> VolumeStepDown(This,pguidEventContext) ) 

#define IAudioEndpointVolumeEx_QueryHardwareSupport(This,pdwHardwareSupportMask)	\
    ( (This)->lpVtbl -> QueryHardwareSupport(This,pdwHardwareSupportMask) ) 

#define IAudioEndpointVolumeEx_GetVolumeRange(This,pflVolumeMindB,pflVolumeMaxdB,pflVolumeIncrementdB)	\
    ( (This)->lpVtbl -> GetVolumeRange(This,pflVolumeMindB,pflVolumeMaxdB,pflVolumeIncrementdB) ) 


#define IAudioEndpointVolumeEx_GetVolumeRangeChannel(This,iChannel,pflVolumeMindB,pflVolumeMaxdB,pflVolumeIncrementdB)	\
    ( (This)->lpVtbl -> GetVolumeRangeChannel(This,iChannel,pflVolumeMindB,pflVolumeMaxdB,pflVolumeIncrementdB) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAudioEndpointVolumeEx_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_endpointvolume_0000_0003 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)


extern RPC_IF_HANDLE __MIDL_itf_endpointvolume_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_endpointvolume_0000_0003_v0_0_s_ifspec;

#ifndef __IAudioMeterInformation_INTERFACE_DEFINED__
#define __IAudioMeterInformation_INTERFACE_DEFINED__

/* interface IAudioMeterInformation */
/* [unique][helpstring][nonextensible][uuid][local][object] */ 


EXTERN_C const IID IID_IAudioMeterInformation;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C02216F6-8C67-4B5B-9D00-D008E73E0064")
    IAudioMeterInformation : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetPeakValue( 
            /* [out] */ float *pfPeak) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetMeteringChannelCount( 
            /* [annotation][out] */ 
            _Out_  UINT *pnChannelCount) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetChannelsPeakValues( 
            /* [in] */ UINT32 u32ChannelCount,
            /* [size_is][out] */ float *afPeakValues) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE QueryHardwareSupport( 
            /* [annotation][out] */ 
            _Out_  DWORD *pdwHardwareSupportMask) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAudioMeterInformationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAudioMeterInformation * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAudioMeterInformation * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAudioMeterInformation * This);
        
        DECLSPEC_XFGVIRT(IAudioMeterInformation, GetPeakValue)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetPeakValue )( 
            IAudioMeterInformation * This,
            /* [out] */ float *pfPeak);
        
        DECLSPEC_XFGVIRT(IAudioMeterInformation, GetMeteringChannelCount)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetMeteringChannelCount )( 
            IAudioMeterInformation * This,
            /* [annotation][out] */ 
            _Out_  UINT *pnChannelCount);
        
        DECLSPEC_XFGVIRT(IAudioMeterInformation, GetChannelsPeakValues)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetChannelsPeakValues )( 
            IAudioMeterInformation * This,
            /* [in] */ UINT32 u32ChannelCount,
            /* [size_is][out] */ float *afPeakValues);
        
        DECLSPEC_XFGVIRT(IAudioMeterInformation, QueryHardwareSupport)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *QueryHardwareSupport )( 
            IAudioMeterInformation * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwHardwareSupportMask);
        
        END_INTERFACE
    } IAudioMeterInformationVtbl;

    interface IAudioMeterInformation
    {
        CONST_VTBL struct IAudioMeterInformationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAudioMeterInformation_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAudioMeterInformation_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAudioMeterInformation_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAudioMeterInformation_GetPeakValue(This,pfPeak)	\
    ( (This)->lpVtbl -> GetPeakValue(This,pfPeak) ) 

#define IAudioMeterInformation_GetMeteringChannelCount(This,pnChannelCount)	\
    ( (This)->lpVtbl -> GetMeteringChannelCount(This,pnChannelCount) ) 

#define IAudioMeterInformation_GetChannelsPeakValues(This,u32ChannelCount,afPeakValues)	\
    ( (This)->lpVtbl -> GetChannelsPeakValues(This,u32ChannelCount,afPeakValues) ) 

#define IAudioMeterInformation_QueryHardwareSupport(This,pdwHardwareSupportMask)	\
    ( (This)->lpVtbl -> QueryHardwareSupport(This,pdwHardwareSupportMask) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAudioMeterInformation_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_endpointvolume_0000_0004 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_endpointvolume_0000_0004_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_endpointvolume_0000_0004_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif



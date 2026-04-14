

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

#ifndef __audiopolicy_h__
#define __audiopolicy_h__

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

#ifndef __IAudioSessionEvents_FWD_DEFINED__
#define __IAudioSessionEvents_FWD_DEFINED__
typedef interface IAudioSessionEvents IAudioSessionEvents;

#endif 	/* __IAudioSessionEvents_FWD_DEFINED__ */


#ifndef __IAudioSessionControl_FWD_DEFINED__
#define __IAudioSessionControl_FWD_DEFINED__
typedef interface IAudioSessionControl IAudioSessionControl;

#endif 	/* __IAudioSessionControl_FWD_DEFINED__ */


#ifndef __IAudioSessionControl2_FWD_DEFINED__
#define __IAudioSessionControl2_FWD_DEFINED__
typedef interface IAudioSessionControl2 IAudioSessionControl2;

#endif 	/* __IAudioSessionControl2_FWD_DEFINED__ */


#ifndef __IAudioSessionManager_FWD_DEFINED__
#define __IAudioSessionManager_FWD_DEFINED__
typedef interface IAudioSessionManager IAudioSessionManager;

#endif 	/* __IAudioSessionManager_FWD_DEFINED__ */


#ifndef __IAudioVolumeDuckNotification_FWD_DEFINED__
#define __IAudioVolumeDuckNotification_FWD_DEFINED__
typedef interface IAudioVolumeDuckNotification IAudioVolumeDuckNotification;

#endif 	/* __IAudioVolumeDuckNotification_FWD_DEFINED__ */


#ifndef __IAudioSessionNotification_FWD_DEFINED__
#define __IAudioSessionNotification_FWD_DEFINED__
typedef interface IAudioSessionNotification IAudioSessionNotification;

#endif 	/* __IAudioSessionNotification_FWD_DEFINED__ */


#ifndef __IAudioSessionEnumerator_FWD_DEFINED__
#define __IAudioSessionEnumerator_FWD_DEFINED__
typedef interface IAudioSessionEnumerator IAudioSessionEnumerator;

#endif 	/* __IAudioSessionEnumerator_FWD_DEFINED__ */


#ifndef __IAudioSessionManager2_FWD_DEFINED__
#define __IAudioSessionManager2_FWD_DEFINED__
typedef interface IAudioSessionManager2 IAudioSessionManager2;

#endif 	/* __IAudioSessionManager2_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#include "propidl.h"
#include "AudioSessionTypes.h"
#include "AudioClient.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_audiopolicy_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Application and Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)
typedef 
enum AudioSessionDisconnectReason
    {
        DisconnectReasonDeviceRemoval	= 0,
        DisconnectReasonServerShutdown	= ( DisconnectReasonDeviceRemoval + 1 ) ,
        DisconnectReasonFormatChanged	= ( DisconnectReasonServerShutdown + 1 ) ,
        DisconnectReasonSessionLogoff	= ( DisconnectReasonFormatChanged + 1 ) ,
        DisconnectReasonSessionDisconnected	= ( DisconnectReasonSessionLogoff + 1 ) ,
        DisconnectReasonExclusiveModeOverride	= ( DisconnectReasonSessionDisconnected + 1 ) 
    } 	AudioSessionDisconnectReason;



extern RPC_IF_HANDLE __MIDL_itf_audiopolicy_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_audiopolicy_0000_0000_v0_0_s_ifspec;

#ifndef __IAudioSessionEvents_INTERFACE_DEFINED__
#define __IAudioSessionEvents_INTERFACE_DEFINED__

/* interface IAudioSessionEvents */
/* [local][uuid][unique][object] */ 


EXTERN_C const IID IID_IAudioSessionEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("24918ACC-64B3-37C1-8CA9-74A66E9957A8")
    IAudioSessionEvents : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnDisplayNameChanged( 
            /* [annotation][string][in] */ 
            _In_  LPCWSTR NewDisplayName,
            /* [in] */ LPCGUID EventContext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnIconPathChanged( 
            /* [annotation][string][in] */ 
            _In_  LPCWSTR NewIconPath,
            /* [in] */ LPCGUID EventContext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnSimpleVolumeChanged( 
            /* [annotation][in] */ 
            _In_  float NewVolume,
            /* [annotation][in] */ 
            _In_  BOOL NewMute,
            /* [in] */ LPCGUID EventContext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnChannelVolumeChanged( 
            /* [annotation][in] */ 
            _In_  DWORD ChannelCount,
            /* [annotation][size_is][in] */ 
            _In_reads_(ChannelCount)  float NewChannelVolumeArray[  ],
            /* [annotation][in] */ 
            _In_  DWORD ChangedChannel,
            /* [in] */ LPCGUID EventContext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnGroupingParamChanged( 
            /* [annotation][in] */ 
            _In_  LPCGUID NewGroupingParam,
            /* [in] */ LPCGUID EventContext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnStateChanged( 
            /* [annotation][in] */ 
            _In_  AudioSessionState NewState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnSessionDisconnected( 
            /* [annotation][in] */ 
            _In_  AudioSessionDisconnectReason DisconnectReason) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAudioSessionEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAudioSessionEvents * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAudioSessionEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAudioSessionEvents * This);
        
        DECLSPEC_XFGVIRT(IAudioSessionEvents, OnDisplayNameChanged)
        HRESULT ( STDMETHODCALLTYPE *OnDisplayNameChanged )( 
            IAudioSessionEvents * This,
            /* [annotation][string][in] */ 
            _In_  LPCWSTR NewDisplayName,
            /* [in] */ LPCGUID EventContext);
        
        DECLSPEC_XFGVIRT(IAudioSessionEvents, OnIconPathChanged)
        HRESULT ( STDMETHODCALLTYPE *OnIconPathChanged )( 
            IAudioSessionEvents * This,
            /* [annotation][string][in] */ 
            _In_  LPCWSTR NewIconPath,
            /* [in] */ LPCGUID EventContext);
        
        DECLSPEC_XFGVIRT(IAudioSessionEvents, OnSimpleVolumeChanged)
        HRESULT ( STDMETHODCALLTYPE *OnSimpleVolumeChanged )( 
            IAudioSessionEvents * This,
            /* [annotation][in] */ 
            _In_  float NewVolume,
            /* [annotation][in] */ 
            _In_  BOOL NewMute,
            /* [in] */ LPCGUID EventContext);
        
        DECLSPEC_XFGVIRT(IAudioSessionEvents, OnChannelVolumeChanged)
        HRESULT ( STDMETHODCALLTYPE *OnChannelVolumeChanged )( 
            IAudioSessionEvents * This,
            /* [annotation][in] */ 
            _In_  DWORD ChannelCount,
            /* [annotation][size_is][in] */ 
            _In_reads_(ChannelCount)  float NewChannelVolumeArray[  ],
            /* [annotation][in] */ 
            _In_  DWORD ChangedChannel,
            /* [in] */ LPCGUID EventContext);
        
        DECLSPEC_XFGVIRT(IAudioSessionEvents, OnGroupingParamChanged)
        HRESULT ( STDMETHODCALLTYPE *OnGroupingParamChanged )( 
            IAudioSessionEvents * This,
            /* [annotation][in] */ 
            _In_  LPCGUID NewGroupingParam,
            /* [in] */ LPCGUID EventContext);
        
        DECLSPEC_XFGVIRT(IAudioSessionEvents, OnStateChanged)
        HRESULT ( STDMETHODCALLTYPE *OnStateChanged )( 
            IAudioSessionEvents * This,
            /* [annotation][in] */ 
            _In_  AudioSessionState NewState);
        
        DECLSPEC_XFGVIRT(IAudioSessionEvents, OnSessionDisconnected)
        HRESULT ( STDMETHODCALLTYPE *OnSessionDisconnected )( 
            IAudioSessionEvents * This,
            /* [annotation][in] */ 
            _In_  AudioSessionDisconnectReason DisconnectReason);
        
        END_INTERFACE
    } IAudioSessionEventsVtbl;

    interface IAudioSessionEvents
    {
        CONST_VTBL struct IAudioSessionEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAudioSessionEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAudioSessionEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAudioSessionEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAudioSessionEvents_OnDisplayNameChanged(This,NewDisplayName,EventContext)	\
    ( (This)->lpVtbl -> OnDisplayNameChanged(This,NewDisplayName,EventContext) ) 

#define IAudioSessionEvents_OnIconPathChanged(This,NewIconPath,EventContext)	\
    ( (This)->lpVtbl -> OnIconPathChanged(This,NewIconPath,EventContext) ) 

#define IAudioSessionEvents_OnSimpleVolumeChanged(This,NewVolume,NewMute,EventContext)	\
    ( (This)->lpVtbl -> OnSimpleVolumeChanged(This,NewVolume,NewMute,EventContext) ) 

#define IAudioSessionEvents_OnChannelVolumeChanged(This,ChannelCount,NewChannelVolumeArray,ChangedChannel,EventContext)	\
    ( (This)->lpVtbl -> OnChannelVolumeChanged(This,ChannelCount,NewChannelVolumeArray,ChangedChannel,EventContext) ) 

#define IAudioSessionEvents_OnGroupingParamChanged(This,NewGroupingParam,EventContext)	\
    ( (This)->lpVtbl -> OnGroupingParamChanged(This,NewGroupingParam,EventContext) ) 

#define IAudioSessionEvents_OnStateChanged(This,NewState)	\
    ( (This)->lpVtbl -> OnStateChanged(This,NewState) ) 

#define IAudioSessionEvents_OnSessionDisconnected(This,DisconnectReason)	\
    ( (This)->lpVtbl -> OnSessionDisconnected(This,DisconnectReason) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAudioSessionEvents_INTERFACE_DEFINED__ */


#ifndef __IAudioSessionControl_INTERFACE_DEFINED__
#define __IAudioSessionControl_INTERFACE_DEFINED__

/* interface IAudioSessionControl */
/* [local][unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IAudioSessionControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F4B1A599-7266-4319-A8CA-E70ACB11E8CD")
    IAudioSessionControl : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetState( 
            /* [annotation][out] */ 
            _Out_  AudioSessionState *pRetVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDisplayName( 
            /* [annotation][string][out] */ 
            _Out_  LPWSTR *pRetVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDisplayName( 
            /* [annotation][string][in] */ 
            _In_  LPCWSTR Value,
            /* [unique][in] */ LPCGUID EventContext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetIconPath( 
            /* [annotation][string][out] */ 
            _Out_  LPWSTR *pRetVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetIconPath( 
            /* [annotation][string][in] */ 
            _In_  LPCWSTR Value,
            /* [unique][in] */ LPCGUID EventContext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetGroupingParam( 
            /* [annotation][out] */ 
            _Out_  GUID *pRetVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetGroupingParam( 
            /* [annotation][in] */ 
            _In_  LPCGUID Override,
            /* [unique][in] */ LPCGUID EventContext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegisterAudioSessionNotification( 
            /* [annotation][in] */ 
            _In_  IAudioSessionEvents *NewNotifications) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnregisterAudioSessionNotification( 
            /* [annotation][in] */ 
            _In_  IAudioSessionEvents *NewNotifications) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAudioSessionControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAudioSessionControl * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAudioSessionControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAudioSessionControl * This);
        
        DECLSPEC_XFGVIRT(IAudioSessionControl, GetState)
        HRESULT ( STDMETHODCALLTYPE *GetState )( 
            IAudioSessionControl * This,
            /* [annotation][out] */ 
            _Out_  AudioSessionState *pRetVal);
        
        DECLSPEC_XFGVIRT(IAudioSessionControl, GetDisplayName)
        HRESULT ( STDMETHODCALLTYPE *GetDisplayName )( 
            IAudioSessionControl * This,
            /* [annotation][string][out] */ 
            _Out_  LPWSTR *pRetVal);
        
        DECLSPEC_XFGVIRT(IAudioSessionControl, SetDisplayName)
        HRESULT ( STDMETHODCALLTYPE *SetDisplayName )( 
            IAudioSessionControl * This,
            /* [annotation][string][in] */ 
            _In_  LPCWSTR Value,
            /* [unique][in] */ LPCGUID EventContext);
        
        DECLSPEC_XFGVIRT(IAudioSessionControl, GetIconPath)
        HRESULT ( STDMETHODCALLTYPE *GetIconPath )( 
            IAudioSessionControl * This,
            /* [annotation][string][out] */ 
            _Out_  LPWSTR *pRetVal);
        
        DECLSPEC_XFGVIRT(IAudioSessionControl, SetIconPath)
        HRESULT ( STDMETHODCALLTYPE *SetIconPath )( 
            IAudioSessionControl * This,
            /* [annotation][string][in] */ 
            _In_  LPCWSTR Value,
            /* [unique][in] */ LPCGUID EventContext);
        
        DECLSPEC_XFGVIRT(IAudioSessionControl, GetGroupingParam)
        HRESULT ( STDMETHODCALLTYPE *GetGroupingParam )( 
            IAudioSessionControl * This,
            /* [annotation][out] */ 
            _Out_  GUID *pRetVal);
        
        DECLSPEC_XFGVIRT(IAudioSessionControl, SetGroupingParam)
        HRESULT ( STDMETHODCALLTYPE *SetGroupingParam )( 
            IAudioSessionControl * This,
            /* [annotation][in] */ 
            _In_  LPCGUID Override,
            /* [unique][in] */ LPCGUID EventContext);
        
        DECLSPEC_XFGVIRT(IAudioSessionControl, RegisterAudioSessionNotification)
        HRESULT ( STDMETHODCALLTYPE *RegisterAudioSessionNotification )( 
            IAudioSessionControl * This,
            /* [annotation][in] */ 
            _In_  IAudioSessionEvents *NewNotifications);
        
        DECLSPEC_XFGVIRT(IAudioSessionControl, UnregisterAudioSessionNotification)
        HRESULT ( STDMETHODCALLTYPE *UnregisterAudioSessionNotification )( 
            IAudioSessionControl * This,
            /* [annotation][in] */ 
            _In_  IAudioSessionEvents *NewNotifications);
        
        END_INTERFACE
    } IAudioSessionControlVtbl;

    interface IAudioSessionControl
    {
        CONST_VTBL struct IAudioSessionControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAudioSessionControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAudioSessionControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAudioSessionControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAudioSessionControl_GetState(This,pRetVal)	\
    ( (This)->lpVtbl -> GetState(This,pRetVal) ) 

#define IAudioSessionControl_GetDisplayName(This,pRetVal)	\
    ( (This)->lpVtbl -> GetDisplayName(This,pRetVal) ) 

#define IAudioSessionControl_SetDisplayName(This,Value,EventContext)	\
    ( (This)->lpVtbl -> SetDisplayName(This,Value,EventContext) ) 

#define IAudioSessionControl_GetIconPath(This,pRetVal)	\
    ( (This)->lpVtbl -> GetIconPath(This,pRetVal) ) 

#define IAudioSessionControl_SetIconPath(This,Value,EventContext)	\
    ( (This)->lpVtbl -> SetIconPath(This,Value,EventContext) ) 

#define IAudioSessionControl_GetGroupingParam(This,pRetVal)	\
    ( (This)->lpVtbl -> GetGroupingParam(This,pRetVal) ) 

#define IAudioSessionControl_SetGroupingParam(This,Override,EventContext)	\
    ( (This)->lpVtbl -> SetGroupingParam(This,Override,EventContext) ) 

#define IAudioSessionControl_RegisterAudioSessionNotification(This,NewNotifications)	\
    ( (This)->lpVtbl -> RegisterAudioSessionNotification(This,NewNotifications) ) 

#define IAudioSessionControl_UnregisterAudioSessionNotification(This,NewNotifications)	\
    ( (This)->lpVtbl -> UnregisterAudioSessionNotification(This,NewNotifications) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAudioSessionControl_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_audiopolicy_0000_0002 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_audiopolicy_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_audiopolicy_0000_0002_v0_0_s_ifspec;

#ifndef __IAudioSessionControl2_INTERFACE_DEFINED__
#define __IAudioSessionControl2_INTERFACE_DEFINED__

/* interface IAudioSessionControl2 */
/* [local][unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IAudioSessionControl2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("bfb7ff88-7239-4fc9-8fa2-07c950be9c6d")
    IAudioSessionControl2 : public IAudioSessionControl
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetSessionIdentifier( 
            /* [annotation][string][out] */ 
            _Out_  LPWSTR *pRetVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSessionInstanceIdentifier( 
            /* [annotation][string][out] */ 
            _Out_  LPWSTR *pRetVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProcessId( 
            /* [annotation][out] */ 
            _Out_  DWORD *pRetVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsSystemSoundsSession( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDuckingPreference( 
            /* [in] */ BOOL optOut) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAudioSessionControl2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAudioSessionControl2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAudioSessionControl2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAudioSessionControl2 * This);
        
        DECLSPEC_XFGVIRT(IAudioSessionControl, GetState)
        HRESULT ( STDMETHODCALLTYPE *GetState )( 
            IAudioSessionControl2 * This,
            /* [annotation][out] */ 
            _Out_  AudioSessionState *pRetVal);
        
        DECLSPEC_XFGVIRT(IAudioSessionControl, GetDisplayName)
        HRESULT ( STDMETHODCALLTYPE *GetDisplayName )( 
            IAudioSessionControl2 * This,
            /* [annotation][string][out] */ 
            _Out_  LPWSTR *pRetVal);
        
        DECLSPEC_XFGVIRT(IAudioSessionControl, SetDisplayName)
        HRESULT ( STDMETHODCALLTYPE *SetDisplayName )( 
            IAudioSessionControl2 * This,
            /* [annotation][string][in] */ 
            _In_  LPCWSTR Value,
            /* [unique][in] */ LPCGUID EventContext);
        
        DECLSPEC_XFGVIRT(IAudioSessionControl, GetIconPath)
        HRESULT ( STDMETHODCALLTYPE *GetIconPath )( 
            IAudioSessionControl2 * This,
            /* [annotation][string][out] */ 
            _Out_  LPWSTR *pRetVal);
        
        DECLSPEC_XFGVIRT(IAudioSessionControl, SetIconPath)
        HRESULT ( STDMETHODCALLTYPE *SetIconPath )( 
            IAudioSessionControl2 * This,
            /* [annotation][string][in] */ 
            _In_  LPCWSTR Value,
            /* [unique][in] */ LPCGUID EventContext);
        
        DECLSPEC_XFGVIRT(IAudioSessionControl, GetGroupingParam)
        HRESULT ( STDMETHODCALLTYPE *GetGroupingParam )( 
            IAudioSessionControl2 * This,
            /* [annotation][out] */ 
            _Out_  GUID *pRetVal);
        
        DECLSPEC_XFGVIRT(IAudioSessionControl, SetGroupingParam)
        HRESULT ( STDMETHODCALLTYPE *SetGroupingParam )( 
            IAudioSessionControl2 * This,
            /* [annotation][in] */ 
            _In_  LPCGUID Override,
            /* [unique][in] */ LPCGUID EventContext);
        
        DECLSPEC_XFGVIRT(IAudioSessionControl, RegisterAudioSessionNotification)
        HRESULT ( STDMETHODCALLTYPE *RegisterAudioSessionNotification )( 
            IAudioSessionControl2 * This,
            /* [annotation][in] */ 
            _In_  IAudioSessionEvents *NewNotifications);
        
        DECLSPEC_XFGVIRT(IAudioSessionControl, UnregisterAudioSessionNotification)
        HRESULT ( STDMETHODCALLTYPE *UnregisterAudioSessionNotification )( 
            IAudioSessionControl2 * This,
            /* [annotation][in] */ 
            _In_  IAudioSessionEvents *NewNotifications);
        
        DECLSPEC_XFGVIRT(IAudioSessionControl2, GetSessionIdentifier)
        HRESULT ( STDMETHODCALLTYPE *GetSessionIdentifier )( 
            IAudioSessionControl2 * This,
            /* [annotation][string][out] */ 
            _Out_  LPWSTR *pRetVal);
        
        DECLSPEC_XFGVIRT(IAudioSessionControl2, GetSessionInstanceIdentifier)
        HRESULT ( STDMETHODCALLTYPE *GetSessionInstanceIdentifier )( 
            IAudioSessionControl2 * This,
            /* [annotation][string][out] */ 
            _Out_  LPWSTR *pRetVal);
        
        DECLSPEC_XFGVIRT(IAudioSessionControl2, GetProcessId)
        HRESULT ( STDMETHODCALLTYPE *GetProcessId )( 
            IAudioSessionControl2 * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pRetVal);
        
        DECLSPEC_XFGVIRT(IAudioSessionControl2, IsSystemSoundsSession)
        HRESULT ( STDMETHODCALLTYPE *IsSystemSoundsSession )( 
            IAudioSessionControl2 * This);
        
        DECLSPEC_XFGVIRT(IAudioSessionControl2, SetDuckingPreference)
        HRESULT ( STDMETHODCALLTYPE *SetDuckingPreference )( 
            IAudioSessionControl2 * This,
            /* [in] */ BOOL optOut);
        
        END_INTERFACE
    } IAudioSessionControl2Vtbl;

    interface IAudioSessionControl2
    {
        CONST_VTBL struct IAudioSessionControl2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAudioSessionControl2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAudioSessionControl2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAudioSessionControl2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAudioSessionControl2_GetState(This,pRetVal)	\
    ( (This)->lpVtbl -> GetState(This,pRetVal) ) 

#define IAudioSessionControl2_GetDisplayName(This,pRetVal)	\
    ( (This)->lpVtbl -> GetDisplayName(This,pRetVal) ) 

#define IAudioSessionControl2_SetDisplayName(This,Value,EventContext)	\
    ( (This)->lpVtbl -> SetDisplayName(This,Value,EventContext) ) 

#define IAudioSessionControl2_GetIconPath(This,pRetVal)	\
    ( (This)->lpVtbl -> GetIconPath(This,pRetVal) ) 

#define IAudioSessionControl2_SetIconPath(This,Value,EventContext)	\
    ( (This)->lpVtbl -> SetIconPath(This,Value,EventContext) ) 

#define IAudioSessionControl2_GetGroupingParam(This,pRetVal)	\
    ( (This)->lpVtbl -> GetGroupingParam(This,pRetVal) ) 

#define IAudioSessionControl2_SetGroupingParam(This,Override,EventContext)	\
    ( (This)->lpVtbl -> SetGroupingParam(This,Override,EventContext) ) 

#define IAudioSessionControl2_RegisterAudioSessionNotification(This,NewNotifications)	\
    ( (This)->lpVtbl -> RegisterAudioSessionNotification(This,NewNotifications) ) 

#define IAudioSessionControl2_UnregisterAudioSessionNotification(This,NewNotifications)	\
    ( (This)->lpVtbl -> UnregisterAudioSessionNotification(This,NewNotifications) ) 


#define IAudioSessionControl2_GetSessionIdentifier(This,pRetVal)	\
    ( (This)->lpVtbl -> GetSessionIdentifier(This,pRetVal) ) 

#define IAudioSessionControl2_GetSessionInstanceIdentifier(This,pRetVal)	\
    ( (This)->lpVtbl -> GetSessionInstanceIdentifier(This,pRetVal) ) 

#define IAudioSessionControl2_GetProcessId(This,pRetVal)	\
    ( (This)->lpVtbl -> GetProcessId(This,pRetVal) ) 

#define IAudioSessionControl2_IsSystemSoundsSession(This)	\
    ( (This)->lpVtbl -> IsSystemSoundsSession(This) ) 

#define IAudioSessionControl2_SetDuckingPreference(This,optOut)	\
    ( (This)->lpVtbl -> SetDuckingPreference(This,optOut) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAudioSessionControl2_INTERFACE_DEFINED__ */


#ifndef __IAudioSessionManager_INTERFACE_DEFINED__
#define __IAudioSessionManager_INTERFACE_DEFINED__

/* interface IAudioSessionManager */
/* [local][unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IAudioSessionManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BFA971F1-4D5E-40BB-935E-967039BFBEE4")
    IAudioSessionManager : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetAudioSessionControl( 
            /* [annotation][in] */ 
            _In_opt_  LPCGUID AudioSessionGuid,
            /* [annotation][in] */ 
            _In_  DWORD StreamFlags,
            /* [annotation][out] */ 
            _Outptr_  IAudioSessionControl **SessionControl) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSimpleAudioVolume( 
            /* [annotation][in] */ 
            _In_opt_  LPCGUID AudioSessionGuid,
            /* [annotation][in] */ 
            _In_  DWORD StreamFlags,
            /* [annotation][out] */ 
            _Outptr_  ISimpleAudioVolume **AudioVolume) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAudioSessionManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAudioSessionManager * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAudioSessionManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAudioSessionManager * This);
        
        DECLSPEC_XFGVIRT(IAudioSessionManager, GetAudioSessionControl)
        HRESULT ( STDMETHODCALLTYPE *GetAudioSessionControl )( 
            IAudioSessionManager * This,
            /* [annotation][in] */ 
            _In_opt_  LPCGUID AudioSessionGuid,
            /* [annotation][in] */ 
            _In_  DWORD StreamFlags,
            /* [annotation][out] */ 
            _Outptr_  IAudioSessionControl **SessionControl);
        
        DECLSPEC_XFGVIRT(IAudioSessionManager, GetSimpleAudioVolume)
        HRESULT ( STDMETHODCALLTYPE *GetSimpleAudioVolume )( 
            IAudioSessionManager * This,
            /* [annotation][in] */ 
            _In_opt_  LPCGUID AudioSessionGuid,
            /* [annotation][in] */ 
            _In_  DWORD StreamFlags,
            /* [annotation][out] */ 
            _Outptr_  ISimpleAudioVolume **AudioVolume);
        
        END_INTERFACE
    } IAudioSessionManagerVtbl;

    interface IAudioSessionManager
    {
        CONST_VTBL struct IAudioSessionManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAudioSessionManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAudioSessionManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAudioSessionManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAudioSessionManager_GetAudioSessionControl(This,AudioSessionGuid,StreamFlags,SessionControl)	\
    ( (This)->lpVtbl -> GetAudioSessionControl(This,AudioSessionGuid,StreamFlags,SessionControl) ) 

#define IAudioSessionManager_GetSimpleAudioVolume(This,AudioSessionGuid,StreamFlags,AudioVolume)	\
    ( (This)->lpVtbl -> GetSimpleAudioVolume(This,AudioSessionGuid,StreamFlags,AudioVolume) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAudioSessionManager_INTERFACE_DEFINED__ */


#ifndef __IAudioVolumeDuckNotification_INTERFACE_DEFINED__
#define __IAudioVolumeDuckNotification_INTERFACE_DEFINED__

/* interface IAudioVolumeDuckNotification */
/* [local][unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IAudioVolumeDuckNotification;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C3B284D4-6D39-4359-B3CF-B56DDB3BB39C")
    IAudioVolumeDuckNotification : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnVolumeDuckNotification( 
            /* [in] */ LPCWSTR sessionID,
            /* [in] */ UINT32 countCommunicationSessions) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnVolumeUnduckNotification( 
            /* [in] */ LPCWSTR sessionID) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAudioVolumeDuckNotificationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAudioVolumeDuckNotification * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAudioVolumeDuckNotification * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAudioVolumeDuckNotification * This);
        
        DECLSPEC_XFGVIRT(IAudioVolumeDuckNotification, OnVolumeDuckNotification)
        HRESULT ( STDMETHODCALLTYPE *OnVolumeDuckNotification )( 
            IAudioVolumeDuckNotification * This,
            /* [in] */ LPCWSTR sessionID,
            /* [in] */ UINT32 countCommunicationSessions);
        
        DECLSPEC_XFGVIRT(IAudioVolumeDuckNotification, OnVolumeUnduckNotification)
        HRESULT ( STDMETHODCALLTYPE *OnVolumeUnduckNotification )( 
            IAudioVolumeDuckNotification * This,
            /* [in] */ LPCWSTR sessionID);
        
        END_INTERFACE
    } IAudioVolumeDuckNotificationVtbl;

    interface IAudioVolumeDuckNotification
    {
        CONST_VTBL struct IAudioVolumeDuckNotificationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAudioVolumeDuckNotification_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAudioVolumeDuckNotification_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAudioVolumeDuckNotification_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAudioVolumeDuckNotification_OnVolumeDuckNotification(This,sessionID,countCommunicationSessions)	\
    ( (This)->lpVtbl -> OnVolumeDuckNotification(This,sessionID,countCommunicationSessions) ) 

#define IAudioVolumeDuckNotification_OnVolumeUnduckNotification(This,sessionID)	\
    ( (This)->lpVtbl -> OnVolumeUnduckNotification(This,sessionID) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAudioVolumeDuckNotification_INTERFACE_DEFINED__ */


#ifndef __IAudioSessionNotification_INTERFACE_DEFINED__
#define __IAudioSessionNotification_INTERFACE_DEFINED__

/* interface IAudioSessionNotification */
/* [local][unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IAudioSessionNotification;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("641DD20B-4D41-49CC-ABA3-174B9477BB08")
    IAudioSessionNotification : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnSessionCreated( 
            /* [in] */ IAudioSessionControl *NewSession) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAudioSessionNotificationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAudioSessionNotification * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAudioSessionNotification * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAudioSessionNotification * This);
        
        DECLSPEC_XFGVIRT(IAudioSessionNotification, OnSessionCreated)
        HRESULT ( STDMETHODCALLTYPE *OnSessionCreated )( 
            IAudioSessionNotification * This,
            /* [in] */ IAudioSessionControl *NewSession);
        
        END_INTERFACE
    } IAudioSessionNotificationVtbl;

    interface IAudioSessionNotification
    {
        CONST_VTBL struct IAudioSessionNotificationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAudioSessionNotification_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAudioSessionNotification_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAudioSessionNotification_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAudioSessionNotification_OnSessionCreated(This,NewSession)	\
    ( (This)->lpVtbl -> OnSessionCreated(This,NewSession) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAudioSessionNotification_INTERFACE_DEFINED__ */


#ifndef __IAudioSessionEnumerator_INTERFACE_DEFINED__
#define __IAudioSessionEnumerator_INTERFACE_DEFINED__

/* interface IAudioSessionEnumerator */
/* [local][unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IAudioSessionEnumerator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("E2F5BB11-0570-40CA-ACDD-3AA01277DEE8")
    IAudioSessionEnumerator : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCount( 
            /* [out] */ int *SessionCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSession( 
            /* [in] */ int SessionCount,
            /* [out] */ IAudioSessionControl **Session) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAudioSessionEnumeratorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAudioSessionEnumerator * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAudioSessionEnumerator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAudioSessionEnumerator * This);
        
        DECLSPEC_XFGVIRT(IAudioSessionEnumerator, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            IAudioSessionEnumerator * This,
            /* [out] */ int *SessionCount);
        
        DECLSPEC_XFGVIRT(IAudioSessionEnumerator, GetSession)
        HRESULT ( STDMETHODCALLTYPE *GetSession )( 
            IAudioSessionEnumerator * This,
            /* [in] */ int SessionCount,
            /* [out] */ IAudioSessionControl **Session);
        
        END_INTERFACE
    } IAudioSessionEnumeratorVtbl;

    interface IAudioSessionEnumerator
    {
        CONST_VTBL struct IAudioSessionEnumeratorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAudioSessionEnumerator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAudioSessionEnumerator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAudioSessionEnumerator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAudioSessionEnumerator_GetCount(This,SessionCount)	\
    ( (This)->lpVtbl -> GetCount(This,SessionCount) ) 

#define IAudioSessionEnumerator_GetSession(This,SessionCount,Session)	\
    ( (This)->lpVtbl -> GetSession(This,SessionCount,Session) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAudioSessionEnumerator_INTERFACE_DEFINED__ */


#ifndef __IAudioSessionManager2_INTERFACE_DEFINED__
#define __IAudioSessionManager2_INTERFACE_DEFINED__

/* interface IAudioSessionManager2 */
/* [local][unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IAudioSessionManager2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("77AA99A0-1BD6-484F-8BC7-2C654C9A9B6F")
    IAudioSessionManager2 : public IAudioSessionManager
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetSessionEnumerator( 
            /* [retval][out] */ IAudioSessionEnumerator **SessionEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegisterSessionNotification( 
            IAudioSessionNotification *SessionNotification) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnregisterSessionNotification( 
            IAudioSessionNotification *SessionNotification) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegisterDuckNotification( 
            /* [string][in] */ LPCWSTR sessionID,
            /* [annotation][in] */ 
            _In_  IAudioVolumeDuckNotification *duckNotification) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnregisterDuckNotification( 
            /* [annotation][in] */ 
            _In_  IAudioVolumeDuckNotification *duckNotification) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAudioSessionManager2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAudioSessionManager2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAudioSessionManager2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAudioSessionManager2 * This);
        
        DECLSPEC_XFGVIRT(IAudioSessionManager, GetAudioSessionControl)
        HRESULT ( STDMETHODCALLTYPE *GetAudioSessionControl )( 
            IAudioSessionManager2 * This,
            /* [annotation][in] */ 
            _In_opt_  LPCGUID AudioSessionGuid,
            /* [annotation][in] */ 
            _In_  DWORD StreamFlags,
            /* [annotation][out] */ 
            _Outptr_  IAudioSessionControl **SessionControl);
        
        DECLSPEC_XFGVIRT(IAudioSessionManager, GetSimpleAudioVolume)
        HRESULT ( STDMETHODCALLTYPE *GetSimpleAudioVolume )( 
            IAudioSessionManager2 * This,
            /* [annotation][in] */ 
            _In_opt_  LPCGUID AudioSessionGuid,
            /* [annotation][in] */ 
            _In_  DWORD StreamFlags,
            /* [annotation][out] */ 
            _Outptr_  ISimpleAudioVolume **AudioVolume);
        
        DECLSPEC_XFGVIRT(IAudioSessionManager2, GetSessionEnumerator)
        HRESULT ( STDMETHODCALLTYPE *GetSessionEnumerator )( 
            IAudioSessionManager2 * This,
            /* [retval][out] */ IAudioSessionEnumerator **SessionEnum);
        
        DECLSPEC_XFGVIRT(IAudioSessionManager2, RegisterSessionNotification)
        HRESULT ( STDMETHODCALLTYPE *RegisterSessionNotification )( 
            IAudioSessionManager2 * This,
            IAudioSessionNotification *SessionNotification);
        
        DECLSPEC_XFGVIRT(IAudioSessionManager2, UnregisterSessionNotification)
        HRESULT ( STDMETHODCALLTYPE *UnregisterSessionNotification )( 
            IAudioSessionManager2 * This,
            IAudioSessionNotification *SessionNotification);
        
        DECLSPEC_XFGVIRT(IAudioSessionManager2, RegisterDuckNotification)
        HRESULT ( STDMETHODCALLTYPE *RegisterDuckNotification )( 
            IAudioSessionManager2 * This,
            /* [string][in] */ LPCWSTR sessionID,
            /* [annotation][in] */ 
            _In_  IAudioVolumeDuckNotification *duckNotification);
        
        DECLSPEC_XFGVIRT(IAudioSessionManager2, UnregisterDuckNotification)
        HRESULT ( STDMETHODCALLTYPE *UnregisterDuckNotification )( 
            IAudioSessionManager2 * This,
            /* [annotation][in] */ 
            _In_  IAudioVolumeDuckNotification *duckNotification);
        
        END_INTERFACE
    } IAudioSessionManager2Vtbl;

    interface IAudioSessionManager2
    {
        CONST_VTBL struct IAudioSessionManager2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAudioSessionManager2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAudioSessionManager2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAudioSessionManager2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAudioSessionManager2_GetAudioSessionControl(This,AudioSessionGuid,StreamFlags,SessionControl)	\
    ( (This)->lpVtbl -> GetAudioSessionControl(This,AudioSessionGuid,StreamFlags,SessionControl) ) 

#define IAudioSessionManager2_GetSimpleAudioVolume(This,AudioSessionGuid,StreamFlags,AudioVolume)	\
    ( (This)->lpVtbl -> GetSimpleAudioVolume(This,AudioSessionGuid,StreamFlags,AudioVolume) ) 


#define IAudioSessionManager2_GetSessionEnumerator(This,SessionEnum)	\
    ( (This)->lpVtbl -> GetSessionEnumerator(This,SessionEnum) ) 

#define IAudioSessionManager2_RegisterSessionNotification(This,SessionNotification)	\
    ( (This)->lpVtbl -> RegisterSessionNotification(This,SessionNotification) ) 

#define IAudioSessionManager2_UnregisterSessionNotification(This,SessionNotification)	\
    ( (This)->lpVtbl -> UnregisterSessionNotification(This,SessionNotification) ) 

#define IAudioSessionManager2_RegisterDuckNotification(This,sessionID,duckNotification)	\
    ( (This)->lpVtbl -> RegisterDuckNotification(This,sessionID,duckNotification) ) 

#define IAudioSessionManager2_UnregisterDuckNotification(This,duckNotification)	\
    ( (This)->lpVtbl -> UnregisterDuckNotification(This,duckNotification) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAudioSessionManager2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_audiopolicy_0000_0008 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_audiopolicy_0000_0008_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_audiopolicy_0000_0008_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif



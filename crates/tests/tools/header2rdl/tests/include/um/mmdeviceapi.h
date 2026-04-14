

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

#ifndef __mmdeviceapi_h__
#define __mmdeviceapi_h__

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

#ifndef __IMMNotificationClient_FWD_DEFINED__
#define __IMMNotificationClient_FWD_DEFINED__
typedef interface IMMNotificationClient IMMNotificationClient;

#endif 	/* __IMMNotificationClient_FWD_DEFINED__ */


#ifndef __IMMDevice_FWD_DEFINED__
#define __IMMDevice_FWD_DEFINED__
typedef interface IMMDevice IMMDevice;

#endif 	/* __IMMDevice_FWD_DEFINED__ */


#ifndef __IMMDeviceCollection_FWD_DEFINED__
#define __IMMDeviceCollection_FWD_DEFINED__
typedef interface IMMDeviceCollection IMMDeviceCollection;

#endif 	/* __IMMDeviceCollection_FWD_DEFINED__ */


#ifndef __IMMEndpoint_FWD_DEFINED__
#define __IMMEndpoint_FWD_DEFINED__
typedef interface IMMEndpoint IMMEndpoint;

#endif 	/* __IMMEndpoint_FWD_DEFINED__ */


#ifndef __IMMDeviceEnumerator_FWD_DEFINED__
#define __IMMDeviceEnumerator_FWD_DEFINED__
typedef interface IMMDeviceEnumerator IMMDeviceEnumerator;

#endif 	/* __IMMDeviceEnumerator_FWD_DEFINED__ */


#ifndef __IMMDeviceActivator_FWD_DEFINED__
#define __IMMDeviceActivator_FWD_DEFINED__
typedef interface IMMDeviceActivator IMMDeviceActivator;

#endif 	/* __IMMDeviceActivator_FWD_DEFINED__ */


#ifndef __IActivateAudioInterfaceCompletionHandler_FWD_DEFINED__
#define __IActivateAudioInterfaceCompletionHandler_FWD_DEFINED__
typedef interface IActivateAudioInterfaceCompletionHandler IActivateAudioInterfaceCompletionHandler;

#endif 	/* __IActivateAudioInterfaceCompletionHandler_FWD_DEFINED__ */


#ifndef __IActivateAudioInterfaceAsyncOperation_FWD_DEFINED__
#define __IActivateAudioInterfaceAsyncOperation_FWD_DEFINED__
typedef interface IActivateAudioInterfaceAsyncOperation IActivateAudioInterfaceAsyncOperation;

#endif 	/* __IActivateAudioInterfaceAsyncOperation_FWD_DEFINED__ */


#ifndef __IAudioSystemEffectsPropertyChangeNotificationClient_FWD_DEFINED__
#define __IAudioSystemEffectsPropertyChangeNotificationClient_FWD_DEFINED__
typedef interface IAudioSystemEffectsPropertyChangeNotificationClient IAudioSystemEffectsPropertyChangeNotificationClient;

#endif 	/* __IAudioSystemEffectsPropertyChangeNotificationClient_FWD_DEFINED__ */


#ifndef __IAudioSystemEffectsPropertyStore_FWD_DEFINED__
#define __IAudioSystemEffectsPropertyStore_FWD_DEFINED__
typedef interface IAudioSystemEffectsPropertyStore IAudioSystemEffectsPropertyStore;

#endif 	/* __IAudioSystemEffectsPropertyStore_FWD_DEFINED__ */


#ifndef __MMDeviceEnumerator_FWD_DEFINED__
#define __MMDeviceEnumerator_FWD_DEFINED__

#ifdef __cplusplus
typedef class MMDeviceEnumerator MMDeviceEnumerator;
#else
typedef struct MMDeviceEnumerator MMDeviceEnumerator;
#endif /* __cplusplus */

#endif 	/* __MMDeviceEnumerator_FWD_DEFINED__ */


/* header files for imported files */
#include "unknwn.h"
#include "propsys.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_mmdeviceapi_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES)
#define E_NOTFOUND HRESULT_FROM_WIN32(ERROR_NOT_FOUND)
#define E_UNSUPPORTED_TYPE HRESULT_FROM_WIN32(ERROR_UNSUPPORTED_TYPE)
#define DEVICE_STATE_ACTIVE      0x00000001
#define DEVICE_STATE_DISABLED    0x00000002
#define DEVICE_STATE_NOTPRESENT  0x00000004
#define DEVICE_STATE_UNPLUGGED   0x00000008
#define DEVICE_STATEMASK_ALL     0x0000000f
#ifdef DEFINE_PROPERTYKEY
#undef DEFINE_PROPERTYKEY
#endif
#ifdef INITGUID
#define DEFINE_PROPERTYKEY(name, l, w1, w2, b1, b2, b3, b4, b5, b6, b7, b8, pid) EXTERN_C const PROPERTYKEY DECLSPEC_SELECTANY name = { { l, w1, w2, { b1, b2,  b3,  b4,  b5,  b6,  b7,  b8 } }, pid }
#else
#define DEFINE_PROPERTYKEY(name, l, w1, w2, b1, b2, b3, b4, b5, b6, b7, b8, pid) EXTERN_C const PROPERTYKEY name
#endif // INITGUID
DEFINE_PROPERTYKEY(PKEY_AudioEndpoint_FormFactor, 0x1da5d803, 0xd492, 0x4edd, 0x8c, 0x23, 0xe0, 0xc0, 0xff, 0xee, 0x7f, 0x0e, 0); 
DEFINE_PROPERTYKEY(PKEY_AudioEndpoint_ControlPanelPageProvider, 0x1da5d803, 0xd492, 0x4edd, 0x8c, 0x23, 0xe0, 0xc0, 0xff, 0xee, 0x7f, 0x0e, 1); 
DEFINE_PROPERTYKEY(PKEY_AudioEndpoint_Association, 0x1da5d803, 0xd492, 0x4edd, 0x8c, 0x23, 0xe0, 0xc0, 0xff, 0xee, 0x7f, 0x0e, 2);
DEFINE_PROPERTYKEY(PKEY_AudioEndpoint_PhysicalSpeakers, 0x1da5d803, 0xd492, 0x4edd, 0x8c, 0x23, 0xe0, 0xc0, 0xff, 0xee, 0x7f, 0x0e, 3);
DEFINE_PROPERTYKEY(PKEY_AudioEndpoint_GUID, 0x1da5d803, 0xd492, 0x4edd, 0x8c, 0x23, 0xe0, 0xc0, 0xff, 0xee, 0x7f, 0x0e, 4);
DEFINE_PROPERTYKEY(PKEY_AudioEndpoint_Disable_SysFx, 0x1da5d803, 0xd492, 0x4edd, 0x8c, 0x23, 0xe0, 0xc0, 0xff, 0xee, 0x7f, 0x0e, 5);
#define ENDPOINT_SYSFX_ENABLED          0x00000000  // System Effects are enabled.
#define ENDPOINT_SYSFX_DISABLED         0x00000001  // System Effects are disabled.
DEFINE_PROPERTYKEY(PKEY_AudioEndpoint_FullRangeSpeakers, 0x1da5d803, 0xd492, 0x4edd, 0x8c, 0x23, 0xe0, 0xc0, 0xff, 0xee, 0x7f, 0x0e, 6);
DEFINE_PROPERTYKEY(PKEY_AudioEndpoint_Supports_EventDriven_Mode, 0x1da5d803, 0xd492, 0x4edd, 0x8c, 0x23, 0xe0, 0xc0, 0xff, 0xee, 0x7f, 0x0e, 7);
DEFINE_PROPERTYKEY(PKEY_AudioEndpoint_JackSubType, 0x1da5d803, 0xd492, 0x4edd, 0x8c, 0x23, 0xe0, 0xc0, 0xff, 0xee, 0x7f, 0x0e, 8);
DEFINE_PROPERTYKEY(PKEY_AudioEndpoint_Default_VolumeInDb, 0x1da5d803, 0xd492, 0x4edd, 0x8c, 0x23, 0xe0, 0xc0, 0xff, 0xee, 0x7f, 0x0e, 9);
DEFINE_PROPERTYKEY(PKEY_AudioEndpoint_Max_VolumeInDb, 0x1da5d803, 0xd492, 0x4edd, 0x8c, 0x23, 0xe0, 0xc0, 0xff, 0xee, 0x7f, 0x0e, 10);
DEFINE_PROPERTYKEY(PKEY_AudioEndpoint_Min_VolumeInDb, 0x1da5d803, 0xd492, 0x4edd, 0x8c, 0x23, 0xe0, 0xc0, 0xff, 0xee, 0x7f, 0x0e, 11);
DEFINE_PROPERTYKEY(PKEY_AudioEngine_DeviceFormat, 0xf19f064d, 0x82c, 0x4e27, 0xbc, 0x73, 0x68, 0x82, 0xa1, 0xbb, 0x8e, 0x4c, 0); 
DEFINE_PROPERTYKEY(PKEY_AudioEngine_OEMFormat, 0xe4870e26, 0x3cc5, 0x4cd2, 0xba, 0x46, 0xca, 0xa, 0x9a, 0x70, 0xed, 0x4, 3); 
DEFINE_PROPERTYKEY(PKEY_AudioEndpointLogo_IconEffects, 0xf1ab780d, 0x2010, 0x4ed3, 0xa3, 0xa6, 0x8b, 0x87, 0xf0, 0xf0, 0xc4, 0x76, 0);
DEFINE_PROPERTYKEY(PKEY_AudioEndpointLogo_IconPath, 0xf1ab780d, 0x2010, 0x4ed3, 0xa3, 0xa6, 0x8b, 0x87, 0xf0, 0xf0, 0xc4, 0x76, 1);
DEFINE_PROPERTYKEY(PKEY_AudioEndpointSettings_MenuText, 0x14242002, 0x0320, 0x4de4, 0x95, 0x55, 0xa7, 0xd8, 0x2b, 0x73, 0xc2, 0x86, 0);
DEFINE_PROPERTYKEY(PKEY_AudioEndpointSettings_LaunchContract, 0x14242002, 0x0320, 0x4de4, 0x95, 0x55, 0xa7, 0xd8, 0x2b, 0x73, 0xc2, 0x86, 1);
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
typedef struct tagDIRECTX_AUDIO_ACTIVATION_PARAMS
    {
    DWORD cbDirectXAudioActivationParams;
    GUID guidAudioSession;
    DWORD dwAudioStreamFlags;
    } 	DIRECTX_AUDIO_ACTIVATION_PARAMS;

typedef struct tagDIRECTX_AUDIO_ACTIVATION_PARAMS *PDIRECTX_AUDIO_ACTIVATION_PARAMS;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
typedef /* [public][public][public][public][public] */ 
enum __MIDL___MIDL_itf_mmdeviceapi_0000_0000_0001
    {
        eRender	= 0,
        eCapture	= ( eRender + 1 ) ,
        eAll	= ( eCapture + 1 ) ,
        EDataFlow_enum_count	= ( eAll + 1 ) 
    } 	EDataFlow;

typedef /* [public][public][public] */ 
enum __MIDL___MIDL_itf_mmdeviceapi_0000_0000_0002
    {
        eConsole	= 0,
        eMultimedia	= ( eConsole + 1 ) ,
        eCommunications	= ( eMultimedia + 1 ) ,
        ERole_enum_count	= ( eCommunications + 1 ) 
    } 	ERole;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES) */
#pragma endregion
#pragma region Application or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)
typedef /* [public] */ 
enum __MIDL___MIDL_itf_mmdeviceapi_0000_0000_0003
    {
        RemoteNetworkDevice	= 0,
        Speakers	= ( RemoteNetworkDevice + 1 ) ,
        LineLevel	= ( Speakers + 1 ) ,
        Headphones	= ( LineLevel + 1 ) ,
        Microphone	= ( Headphones + 1 ) ,
        Headset	= ( Microphone + 1 ) ,
        Handset	= ( Headset + 1 ) ,
        UnknownDigitalPassthrough	= ( Handset + 1 ) ,
        SPDIF	= ( UnknownDigitalPassthrough + 1 ) ,
        DigitalAudioDisplayDevice	= ( SPDIF + 1 ) ,
        UnknownFormFactor	= ( DigitalAudioDisplayDevice + 1 ) ,
        EndpointFormFactor_enum_count	= ( UnknownFormFactor + 1 ) 
    } 	EndpointFormFactor;

#define HDMI     DigitalAudioDisplayDevice

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
// ----------------------------------------------------------------------
// Device Interface Classes
// ----------------------------------------------------------------------
// {E6327CAD-DCEC-4949-AE8A-991E976A79D2}
DEFINE_GUID(DEVINTERFACE_AUDIO_RENDER , 0xe6327cad, 0xdcec, 0x4949, 0xae, 0x8a, 0x99, 0x1e, 0x97, 0x6a, 0x79, 0xd2);
// {2EEF81BE-33FA-4800-9670-1CD474972C3F}
DEFINE_GUID(DEVINTERFACE_AUDIO_CAPTURE, 0x2eef81be, 0x33fa, 0x4800, 0x96, 0x70, 0x1c, 0xd4, 0x74, 0x97, 0x2c, 0x3f);
#if (NTDDI_VERSION > NTDDI_WINBLUE || \
    (NTDDI_VERSION == NTDDI_WINBLUE && defined(WINBLUE_KBSPRING14)))
// {6DC23320-AB33-4CE4-80D4-BBB3EBBF2814}
DEFINE_GUID(DEVINTERFACE_MIDI_OUTPUT, 0x6dc23320, 0xab33, 0x4ce4, 0x80, 0xd4, 0xbb, 0xb3, 0xeb, 0xbf, 0x28, 0x14);
// {504BE32C-CCF6-4D2C-B73F-6F8B3747E22B}
DEFINE_GUID(DEVINTERFACE_MIDI_INPUT, 0x504be32c, 0xccf6, 0x4d2c, 0xb7, 0x3f, 0x6f, 0x8b, 0x37, 0x47, 0xe2, 0x2b);
#endif
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop and Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES)


extern RPC_IF_HANDLE __MIDL_itf_mmdeviceapi_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mmdeviceapi_0000_0000_v0_0_s_ifspec;

#ifndef __IMMNotificationClient_INTERFACE_DEFINED__
#define __IMMNotificationClient_INTERFACE_DEFINED__

/* interface IMMNotificationClient */
/* [unique][helpstring][nonextensible][uuid][local][object] */ 


EXTERN_C const IID IID_IMMNotificationClient;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7991EEC9-7E89-4D85-8390-6C703CEC60C0")
    IMMNotificationClient : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnDeviceStateChanged( 
            /* [annotation][in] */ 
            _In_  LPCWSTR pwstrDeviceId,
            /* [annotation][in] */ 
            _In_  DWORD dwNewState) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnDeviceAdded( 
            /* [annotation][in] */ 
            _In_  LPCWSTR pwstrDeviceId) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnDeviceRemoved( 
            /* [annotation][in] */ 
            _In_  LPCWSTR pwstrDeviceId) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnDefaultDeviceChanged( 
            /* [annotation][in] */ 
            _In_  EDataFlow flow,
            /* [annotation][in] */ 
            _In_  ERole role,
            /* [annotation][in] */ 
            _In_opt_  LPCWSTR pwstrDefaultDeviceId) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnPropertyValueChanged( 
            /* [annotation][in] */ 
            _In_  LPCWSTR pwstrDeviceId,
            /* [annotation][in] */ 
            _In_  const PROPERTYKEY key) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMMNotificationClientVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMMNotificationClient * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMMNotificationClient * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMMNotificationClient * This);
        
        DECLSPEC_XFGVIRT(IMMNotificationClient, OnDeviceStateChanged)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnDeviceStateChanged )( 
            IMMNotificationClient * This,
            /* [annotation][in] */ 
            _In_  LPCWSTR pwstrDeviceId,
            /* [annotation][in] */ 
            _In_  DWORD dwNewState);
        
        DECLSPEC_XFGVIRT(IMMNotificationClient, OnDeviceAdded)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnDeviceAdded )( 
            IMMNotificationClient * This,
            /* [annotation][in] */ 
            _In_  LPCWSTR pwstrDeviceId);
        
        DECLSPEC_XFGVIRT(IMMNotificationClient, OnDeviceRemoved)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnDeviceRemoved )( 
            IMMNotificationClient * This,
            /* [annotation][in] */ 
            _In_  LPCWSTR pwstrDeviceId);
        
        DECLSPEC_XFGVIRT(IMMNotificationClient, OnDefaultDeviceChanged)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnDefaultDeviceChanged )( 
            IMMNotificationClient * This,
            /* [annotation][in] */ 
            _In_  EDataFlow flow,
            /* [annotation][in] */ 
            _In_  ERole role,
            /* [annotation][in] */ 
            _In_opt_  LPCWSTR pwstrDefaultDeviceId);
        
        DECLSPEC_XFGVIRT(IMMNotificationClient, OnPropertyValueChanged)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnPropertyValueChanged )( 
            IMMNotificationClient * This,
            /* [annotation][in] */ 
            _In_  LPCWSTR pwstrDeviceId,
            /* [annotation][in] */ 
            _In_  const PROPERTYKEY key);
        
        END_INTERFACE
    } IMMNotificationClientVtbl;

    interface IMMNotificationClient
    {
        CONST_VTBL struct IMMNotificationClientVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMMNotificationClient_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMMNotificationClient_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMMNotificationClient_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMMNotificationClient_OnDeviceStateChanged(This,pwstrDeviceId,dwNewState)	\
    ( (This)->lpVtbl -> OnDeviceStateChanged(This,pwstrDeviceId,dwNewState) ) 

#define IMMNotificationClient_OnDeviceAdded(This,pwstrDeviceId)	\
    ( (This)->lpVtbl -> OnDeviceAdded(This,pwstrDeviceId) ) 

#define IMMNotificationClient_OnDeviceRemoved(This,pwstrDeviceId)	\
    ( (This)->lpVtbl -> OnDeviceRemoved(This,pwstrDeviceId) ) 

#define IMMNotificationClient_OnDefaultDeviceChanged(This,flow,role,pwstrDefaultDeviceId)	\
    ( (This)->lpVtbl -> OnDefaultDeviceChanged(This,flow,role,pwstrDefaultDeviceId) ) 

#define IMMNotificationClient_OnPropertyValueChanged(This,pwstrDeviceId,key)	\
    ( (This)->lpVtbl -> OnPropertyValueChanged(This,pwstrDeviceId,key) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMMNotificationClient_INTERFACE_DEFINED__ */


#ifndef __IMMDevice_INTERFACE_DEFINED__
#define __IMMDevice_INTERFACE_DEFINED__

/* interface IMMDevice */
/* [unique][helpstring][nonextensible][uuid][local][object] */ 


EXTERN_C const IID IID_IMMDevice;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D666063F-1587-4E43-81F1-B948E807363F")
    IMMDevice : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Activate( 
            /* [annotation][in] */ 
            _In_  REFIID iid,
            /* [annotation][in] */ 
            _In_  DWORD dwClsCtx,
            /* [annotation][unique][in] */ 
            _In_opt_  PROPVARIANT *pActivationParams,
            /* [annotation][iid_is][out] */ 
            _Out_  void **ppInterface) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OpenPropertyStore( 
            /* [annotation][in] */ 
            _In_  DWORD stgmAccess,
            /* [annotation][out] */ 
            _Out_  IPropertyStore **ppProperties) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetId( 
            /* [annotation][out] */ 
            _Outptr_  LPWSTR *ppstrId) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetState( 
            /* [annotation][out] */ 
            _Out_  DWORD *pdwState) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMMDeviceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMMDevice * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMMDevice * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMMDevice * This);
        
        DECLSPEC_XFGVIRT(IMMDevice, Activate)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Activate )( 
            IMMDevice * This,
            /* [annotation][in] */ 
            _In_  REFIID iid,
            /* [annotation][in] */ 
            _In_  DWORD dwClsCtx,
            /* [annotation][unique][in] */ 
            _In_opt_  PROPVARIANT *pActivationParams,
            /* [annotation][iid_is][out] */ 
            _Out_  void **ppInterface);
        
        DECLSPEC_XFGVIRT(IMMDevice, OpenPropertyStore)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OpenPropertyStore )( 
            IMMDevice * This,
            /* [annotation][in] */ 
            _In_  DWORD stgmAccess,
            /* [annotation][out] */ 
            _Out_  IPropertyStore **ppProperties);
        
        DECLSPEC_XFGVIRT(IMMDevice, GetId)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetId )( 
            IMMDevice * This,
            /* [annotation][out] */ 
            _Outptr_  LPWSTR *ppstrId);
        
        DECLSPEC_XFGVIRT(IMMDevice, GetState)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetState )( 
            IMMDevice * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwState);
        
        END_INTERFACE
    } IMMDeviceVtbl;

    interface IMMDevice
    {
        CONST_VTBL struct IMMDeviceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMMDevice_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMMDevice_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMMDevice_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMMDevice_Activate(This,iid,dwClsCtx,pActivationParams,ppInterface)	\
    ( (This)->lpVtbl -> Activate(This,iid,dwClsCtx,pActivationParams,ppInterface) ) 

#define IMMDevice_OpenPropertyStore(This,stgmAccess,ppProperties)	\
    ( (This)->lpVtbl -> OpenPropertyStore(This,stgmAccess,ppProperties) ) 

#define IMMDevice_GetId(This,ppstrId)	\
    ( (This)->lpVtbl -> GetId(This,ppstrId) ) 

#define IMMDevice_GetState(This,pdwState)	\
    ( (This)->lpVtbl -> GetState(This,pdwState) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMMDevice_INTERFACE_DEFINED__ */


#ifndef __IMMDeviceCollection_INTERFACE_DEFINED__
#define __IMMDeviceCollection_INTERFACE_DEFINED__

/* interface IMMDeviceCollection */
/* [unique][helpstring][nonextensible][uuid][local][object] */ 


EXTERN_C const IID IID_IMMDeviceCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0BD7A1BE-7A1A-44DB-8397-CC5392387B5E")
    IMMDeviceCollection : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetCount( 
            /* [annotation][out] */ 
            _Out_  UINT *pcDevices) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Item( 
            /* [annotation][in] */ 
            _In_  UINT nDevice,
            /* [annotation][out] */ 
            _Out_  IMMDevice **ppDevice) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMMDeviceCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMMDeviceCollection * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMMDeviceCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMMDeviceCollection * This);
        
        DECLSPEC_XFGVIRT(IMMDeviceCollection, GetCount)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            IMMDeviceCollection * This,
            /* [annotation][out] */ 
            _Out_  UINT *pcDevices);
        
        DECLSPEC_XFGVIRT(IMMDeviceCollection, Item)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Item )( 
            IMMDeviceCollection * This,
            /* [annotation][in] */ 
            _In_  UINT nDevice,
            /* [annotation][out] */ 
            _Out_  IMMDevice **ppDevice);
        
        END_INTERFACE
    } IMMDeviceCollectionVtbl;

    interface IMMDeviceCollection
    {
        CONST_VTBL struct IMMDeviceCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMMDeviceCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMMDeviceCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMMDeviceCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMMDeviceCollection_GetCount(This,pcDevices)	\
    ( (This)->lpVtbl -> GetCount(This,pcDevices) ) 

#define IMMDeviceCollection_Item(This,nDevice,ppDevice)	\
    ( (This)->lpVtbl -> Item(This,nDevice,ppDevice) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMMDeviceCollection_INTERFACE_DEFINED__ */


#ifndef __IMMEndpoint_INTERFACE_DEFINED__
#define __IMMEndpoint_INTERFACE_DEFINED__

/* interface IMMEndpoint */
/* [unique][helpstring][nonextensible][uuid][local][object] */ 


EXTERN_C const IID IID_IMMEndpoint;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1BE09788-6894-4089-8586-9A2A6C265AC5")
    IMMEndpoint : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetDataFlow( 
            /* [annotation][out] */ 
            _Out_  EDataFlow *pDataFlow) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMMEndpointVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMMEndpoint * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMMEndpoint * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMMEndpoint * This);
        
        DECLSPEC_XFGVIRT(IMMEndpoint, GetDataFlow)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetDataFlow )( 
            IMMEndpoint * This,
            /* [annotation][out] */ 
            _Out_  EDataFlow *pDataFlow);
        
        END_INTERFACE
    } IMMEndpointVtbl;

    interface IMMEndpoint
    {
        CONST_VTBL struct IMMEndpointVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMMEndpoint_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMMEndpoint_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMMEndpoint_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMMEndpoint_GetDataFlow(This,pDataFlow)	\
    ( (This)->lpVtbl -> GetDataFlow(This,pDataFlow) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMMEndpoint_INTERFACE_DEFINED__ */


#ifndef __IMMDeviceEnumerator_INTERFACE_DEFINED__
#define __IMMDeviceEnumerator_INTERFACE_DEFINED__

/* interface IMMDeviceEnumerator */
/* [unique][helpstring][nonextensible][uuid][local][object] */ 


EXTERN_C const IID IID_IMMDeviceEnumerator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A95664D2-9614-4F35-A746-DE8DB63617E6")
    IMMDeviceEnumerator : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE EnumAudioEndpoints( 
            /* [annotation][in] */ 
            _In_  EDataFlow dataFlow,
            /* [annotation][in] */ 
            _In_  DWORD dwStateMask,
            /* [annotation][out] */ 
            _Out_  IMMDeviceCollection **ppDevices) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetDefaultAudioEndpoint( 
            /* [annotation][in] */ 
            _In_  EDataFlow dataFlow,
            /* [annotation][in] */ 
            _In_  ERole role,
            /* [annotation][out] */ 
            _Out_  IMMDevice **ppEndpoint) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetDevice( 
            /* [annotation][in] */ 
            _In_  LPCWSTR pwstrId,
            /* [annotation][out] */ 
            _Out_  IMMDevice **ppDevice) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE RegisterEndpointNotificationCallback( 
            /* [annotation][in] */ 
            _In_  IMMNotificationClient *pClient) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE UnregisterEndpointNotificationCallback( 
            /* [annotation][in] */ 
            _In_  IMMNotificationClient *pClient) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMMDeviceEnumeratorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMMDeviceEnumerator * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMMDeviceEnumerator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMMDeviceEnumerator * This);
        
        DECLSPEC_XFGVIRT(IMMDeviceEnumerator, EnumAudioEndpoints)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *EnumAudioEndpoints )( 
            IMMDeviceEnumerator * This,
            /* [annotation][in] */ 
            _In_  EDataFlow dataFlow,
            /* [annotation][in] */ 
            _In_  DWORD dwStateMask,
            /* [annotation][out] */ 
            _Out_  IMMDeviceCollection **ppDevices);
        
        DECLSPEC_XFGVIRT(IMMDeviceEnumerator, GetDefaultAudioEndpoint)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetDefaultAudioEndpoint )( 
            IMMDeviceEnumerator * This,
            /* [annotation][in] */ 
            _In_  EDataFlow dataFlow,
            /* [annotation][in] */ 
            _In_  ERole role,
            /* [annotation][out] */ 
            _Out_  IMMDevice **ppEndpoint);
        
        DECLSPEC_XFGVIRT(IMMDeviceEnumerator, GetDevice)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetDevice )( 
            IMMDeviceEnumerator * This,
            /* [annotation][in] */ 
            _In_  LPCWSTR pwstrId,
            /* [annotation][out] */ 
            _Out_  IMMDevice **ppDevice);
        
        DECLSPEC_XFGVIRT(IMMDeviceEnumerator, RegisterEndpointNotificationCallback)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RegisterEndpointNotificationCallback )( 
            IMMDeviceEnumerator * This,
            /* [annotation][in] */ 
            _In_  IMMNotificationClient *pClient);
        
        DECLSPEC_XFGVIRT(IMMDeviceEnumerator, UnregisterEndpointNotificationCallback)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *UnregisterEndpointNotificationCallback )( 
            IMMDeviceEnumerator * This,
            /* [annotation][in] */ 
            _In_  IMMNotificationClient *pClient);
        
        END_INTERFACE
    } IMMDeviceEnumeratorVtbl;

    interface IMMDeviceEnumerator
    {
        CONST_VTBL struct IMMDeviceEnumeratorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMMDeviceEnumerator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMMDeviceEnumerator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMMDeviceEnumerator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMMDeviceEnumerator_EnumAudioEndpoints(This,dataFlow,dwStateMask,ppDevices)	\
    ( (This)->lpVtbl -> EnumAudioEndpoints(This,dataFlow,dwStateMask,ppDevices) ) 

#define IMMDeviceEnumerator_GetDefaultAudioEndpoint(This,dataFlow,role,ppEndpoint)	\
    ( (This)->lpVtbl -> GetDefaultAudioEndpoint(This,dataFlow,role,ppEndpoint) ) 

#define IMMDeviceEnumerator_GetDevice(This,pwstrId,ppDevice)	\
    ( (This)->lpVtbl -> GetDevice(This,pwstrId,ppDevice) ) 

#define IMMDeviceEnumerator_RegisterEndpointNotificationCallback(This,pClient)	\
    ( (This)->lpVtbl -> RegisterEndpointNotificationCallback(This,pClient) ) 

#define IMMDeviceEnumerator_UnregisterEndpointNotificationCallback(This,pClient)	\
    ( (This)->lpVtbl -> UnregisterEndpointNotificationCallback(This,pClient) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMMDeviceEnumerator_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mmdeviceapi_0000_0005 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
/* IMMDeviceActivator is reserved for system use */


extern RPC_IF_HANDLE __MIDL_itf_mmdeviceapi_0000_0005_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mmdeviceapi_0000_0005_v0_0_s_ifspec;

#ifndef __IMMDeviceActivator_INTERFACE_DEFINED__
#define __IMMDeviceActivator_INTERFACE_DEFINED__

/* interface IMMDeviceActivator */
/* [unique][helpstring][nonextensible][uuid][local][object] */ 


EXTERN_C const IID IID_IMMDeviceActivator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3B0D0EA4-D0A9-4B0E-935B-09516746FAC0")
    IMMDeviceActivator : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Activate( 
            /* [annotation][in] */ 
            _In_  REFIID iid,
            /* [annotation][in] */ 
            _In_  IMMDevice *pDevice,
            /* [annotation][in] */ 
            _In_opt_  PROPVARIANT *pActivationParams,
            /* [annotation][iid_is][out] */ 
            _Out_  void **ppInterface) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMMDeviceActivatorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMMDeviceActivator * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMMDeviceActivator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMMDeviceActivator * This);
        
        DECLSPEC_XFGVIRT(IMMDeviceActivator, Activate)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Activate )( 
            IMMDeviceActivator * This,
            /* [annotation][in] */ 
            _In_  REFIID iid,
            /* [annotation][in] */ 
            _In_  IMMDevice *pDevice,
            /* [annotation][in] */ 
            _In_opt_  PROPVARIANT *pActivationParams,
            /* [annotation][iid_is][out] */ 
            _Out_  void **ppInterface);
        
        END_INTERFACE
    } IMMDeviceActivatorVtbl;

    interface IMMDeviceActivator
    {
        CONST_VTBL struct IMMDeviceActivatorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMMDeviceActivator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMMDeviceActivator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMMDeviceActivator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMMDeviceActivator_Activate(This,iid,pDevice,pActivationParams,ppInterface)	\
    ( (This)->lpVtbl -> Activate(This,iid,pDevice,pActivationParams,ppInterface) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMMDeviceActivator_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mmdeviceapi_0000_0006 */
/* [local] */ 

/* IMMDeviceActivator is reserved for system use */
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)



extern RPC_IF_HANDLE __MIDL_itf_mmdeviceapi_0000_0006_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mmdeviceapi_0000_0006_v0_0_s_ifspec;

#ifndef __IActivateAudioInterfaceCompletionHandler_INTERFACE_DEFINED__
#define __IActivateAudioInterfaceCompletionHandler_INTERFACE_DEFINED__

/* interface IActivateAudioInterfaceCompletionHandler */
/* [unique][helpstring][nonextensible][uuid][local][object] */ 


EXTERN_C const IID IID_IActivateAudioInterfaceCompletionHandler;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("41D949AB-9862-444A-80F6-C261334DA5EB")
    IActivateAudioInterfaceCompletionHandler : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ActivateCompleted( 
            /* [annotation][in] */ 
            _In_  IActivateAudioInterfaceAsyncOperation *activateOperation) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IActivateAudioInterfaceCompletionHandlerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IActivateAudioInterfaceCompletionHandler * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IActivateAudioInterfaceCompletionHandler * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IActivateAudioInterfaceCompletionHandler * This);
        
        DECLSPEC_XFGVIRT(IActivateAudioInterfaceCompletionHandler, ActivateCompleted)
        HRESULT ( STDMETHODCALLTYPE *ActivateCompleted )( 
            IActivateAudioInterfaceCompletionHandler * This,
            /* [annotation][in] */ 
            _In_  IActivateAudioInterfaceAsyncOperation *activateOperation);
        
        END_INTERFACE
    } IActivateAudioInterfaceCompletionHandlerVtbl;

    interface IActivateAudioInterfaceCompletionHandler
    {
        CONST_VTBL struct IActivateAudioInterfaceCompletionHandlerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IActivateAudioInterfaceCompletionHandler_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IActivateAudioInterfaceCompletionHandler_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IActivateAudioInterfaceCompletionHandler_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IActivateAudioInterfaceCompletionHandler_ActivateCompleted(This,activateOperation)	\
    ( (This)->lpVtbl -> ActivateCompleted(This,activateOperation) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IActivateAudioInterfaceCompletionHandler_INTERFACE_DEFINED__ */


#ifndef __IActivateAudioInterfaceAsyncOperation_INTERFACE_DEFINED__
#define __IActivateAudioInterfaceAsyncOperation_INTERFACE_DEFINED__

/* interface IActivateAudioInterfaceAsyncOperation */
/* [unique][helpstring][nonextensible][uuid][local][object] */ 


EXTERN_C const IID IID_IActivateAudioInterfaceAsyncOperation;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("72A22D78-CDE4-431D-B8CC-843A71199B6D")
    IActivateAudioInterfaceAsyncOperation : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetActivateResult( 
            /* [annotation][out] */ 
            _Out_  HRESULT *activateResult,
            /* [annotation][out] */ 
            _Outptr_result_maybenull_  IUnknown **activatedInterface) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IActivateAudioInterfaceAsyncOperationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IActivateAudioInterfaceAsyncOperation * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IActivateAudioInterfaceAsyncOperation * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IActivateAudioInterfaceAsyncOperation * This);
        
        DECLSPEC_XFGVIRT(IActivateAudioInterfaceAsyncOperation, GetActivateResult)
        HRESULT ( STDMETHODCALLTYPE *GetActivateResult )( 
            IActivateAudioInterfaceAsyncOperation * This,
            /* [annotation][out] */ 
            _Out_  HRESULT *activateResult,
            /* [annotation][out] */ 
            _Outptr_result_maybenull_  IUnknown **activatedInterface);
        
        END_INTERFACE
    } IActivateAudioInterfaceAsyncOperationVtbl;

    interface IActivateAudioInterfaceAsyncOperation
    {
        CONST_VTBL struct IActivateAudioInterfaceAsyncOperationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IActivateAudioInterfaceAsyncOperation_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IActivateAudioInterfaceAsyncOperation_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IActivateAudioInterfaceAsyncOperation_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IActivateAudioInterfaceAsyncOperation_GetActivateResult(This,activateResult,activatedInterface)	\
    ( (This)->lpVtbl -> GetActivateResult(This,activateResult,activatedInterface) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IActivateAudioInterfaceAsyncOperation_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mmdeviceapi_0000_0008 */
/* [local] */ 

// ----------------------------------------------------------------------
// Function: ActivateAudioInterfaceAsync
// This function takes 
// * a device interface instance identifier representing either
//     - an audio device interface instance (e.g., built-in speakers), or
//     - an device interface class (e.g., audio render devices)
// * a COM interface identifier
// * activation parameters specific to the interface being activated
// and asynchronously returns a pointer to the specified interface
// ----------------------------------------------------------------------
STDAPI ActivateAudioInterfaceAsync(
    _In_ LPCWSTR deviceInterfacePath,
    _In_ REFIID riid,
    _In_opt_ PROPVARIANT *activationParams,
    _In_ IActivateAudioInterfaceCompletionHandler *completionHandler,
    _COM_Outptr_ IActivateAudioInterfaceAsyncOperation **activationOperation
    );
// The AUDIOCLIENT_ACTIVATION_PARAMS structure can be used when creating an IAudioClient.
// It is defined in AudioClientActivationParams.h
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
typedef /* [public] */ struct __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0001
    {
    LPARAM AddPageParam;
    IMMDevice *pEndpoint;
    IMMDevice *pPnpInterface;
    IMMDevice *pPnpDevnode;
    } 	AudioExtensionParams;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Desktop and Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_APP)
typedef /* [public][public] */ 
enum __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002
    {
        AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE_DEFAULT	= 0,
        AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE_USER	= 1,
        AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE_VOLATILE	= 2,
        AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE_ENUM_COUNT	= ( AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE_VOLATILE + 1 ) 
    } 	AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE;



extern RPC_IF_HANDLE __MIDL_itf_mmdeviceapi_0000_0008_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mmdeviceapi_0000_0008_v0_0_s_ifspec;

#ifndef __IAudioSystemEffectsPropertyChangeNotificationClient_INTERFACE_DEFINED__
#define __IAudioSystemEffectsPropertyChangeNotificationClient_INTERFACE_DEFINED__

/* interface IAudioSystemEffectsPropertyChangeNotificationClient */
/* [unique][helpstring][nonextensible][uuid][local][object] */ 


EXTERN_C const IID IID_IAudioSystemEffectsPropertyChangeNotificationClient;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("20049D40-56D5-400E-A2EF-385599FEED49")
    IAudioSystemEffectsPropertyChangeNotificationClient : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnPropertyChanged( 
            /* [annotation][in] */ 
            _In_  AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE type,
            /* [annotation][in] */ 
            _In_  const PROPERTYKEY key) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAudioSystemEffectsPropertyChangeNotificationClientVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAudioSystemEffectsPropertyChangeNotificationClient * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAudioSystemEffectsPropertyChangeNotificationClient * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAudioSystemEffectsPropertyChangeNotificationClient * This);
        
        DECLSPEC_XFGVIRT(IAudioSystemEffectsPropertyChangeNotificationClient, OnPropertyChanged)
        HRESULT ( STDMETHODCALLTYPE *OnPropertyChanged )( 
            IAudioSystemEffectsPropertyChangeNotificationClient * This,
            /* [annotation][in] */ 
            _In_  AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE type,
            /* [annotation][in] */ 
            _In_  const PROPERTYKEY key);
        
        END_INTERFACE
    } IAudioSystemEffectsPropertyChangeNotificationClientVtbl;

    interface IAudioSystemEffectsPropertyChangeNotificationClient
    {
        CONST_VTBL struct IAudioSystemEffectsPropertyChangeNotificationClientVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAudioSystemEffectsPropertyChangeNotificationClient_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAudioSystemEffectsPropertyChangeNotificationClient_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAudioSystemEffectsPropertyChangeNotificationClient_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAudioSystemEffectsPropertyChangeNotificationClient_OnPropertyChanged(This,type,key)	\
    ( (This)->lpVtbl -> OnPropertyChanged(This,type,key) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAudioSystemEffectsPropertyChangeNotificationClient_INTERFACE_DEFINED__ */


#ifndef __IAudioSystemEffectsPropertyStore_INTERFACE_DEFINED__
#define __IAudioSystemEffectsPropertyStore_INTERFACE_DEFINED__

/* interface IAudioSystemEffectsPropertyStore */
/* [unique][helpstring][nonextensible][uuid][local][object] */ 


EXTERN_C const IID IID_IAudioSystemEffectsPropertyStore;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("302AE7F9-D7E0-43E4-971B-1F8293613D2A")
    IAudioSystemEffectsPropertyStore : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OpenDefaultPropertyStore( 
            /* [annotation][in] */ 
            _In_  DWORD stgmAccess,
            /* [annotation][out] */ 
            _COM_Outptr_  IPropertyStore **propStore) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OpenUserPropertyStore( 
            /* [annotation][in] */ 
            _In_  DWORD stgmAccess,
            /* [annotation][out] */ 
            _COM_Outptr_  IPropertyStore **propStore) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OpenVolatilePropertyStore( 
            /* [annotation][in] */ 
            _In_  DWORD stgmAccess,
            /* [annotation][out] */ 
            _COM_Outptr_  IPropertyStore **propStore) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ResetUserPropertyStore( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ResetVolatilePropertyStore( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegisterPropertyChangeNotification( 
            /* [annotation][in] */ 
            _In_  IAudioSystemEffectsPropertyChangeNotificationClient *callback) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnregisterPropertyChangeNotification( 
            /* [annotation][in] */ 
            _In_  IAudioSystemEffectsPropertyChangeNotificationClient *callback) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAudioSystemEffectsPropertyStoreVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAudioSystemEffectsPropertyStore * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAudioSystemEffectsPropertyStore * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAudioSystemEffectsPropertyStore * This);
        
        DECLSPEC_XFGVIRT(IAudioSystemEffectsPropertyStore, OpenDefaultPropertyStore)
        HRESULT ( STDMETHODCALLTYPE *OpenDefaultPropertyStore )( 
            IAudioSystemEffectsPropertyStore * This,
            /* [annotation][in] */ 
            _In_  DWORD stgmAccess,
            /* [annotation][out] */ 
            _COM_Outptr_  IPropertyStore **propStore);
        
        DECLSPEC_XFGVIRT(IAudioSystemEffectsPropertyStore, OpenUserPropertyStore)
        HRESULT ( STDMETHODCALLTYPE *OpenUserPropertyStore )( 
            IAudioSystemEffectsPropertyStore * This,
            /* [annotation][in] */ 
            _In_  DWORD stgmAccess,
            /* [annotation][out] */ 
            _COM_Outptr_  IPropertyStore **propStore);
        
        DECLSPEC_XFGVIRT(IAudioSystemEffectsPropertyStore, OpenVolatilePropertyStore)
        HRESULT ( STDMETHODCALLTYPE *OpenVolatilePropertyStore )( 
            IAudioSystemEffectsPropertyStore * This,
            /* [annotation][in] */ 
            _In_  DWORD stgmAccess,
            /* [annotation][out] */ 
            _COM_Outptr_  IPropertyStore **propStore);
        
        DECLSPEC_XFGVIRT(IAudioSystemEffectsPropertyStore, ResetUserPropertyStore)
        HRESULT ( STDMETHODCALLTYPE *ResetUserPropertyStore )( 
            IAudioSystemEffectsPropertyStore * This);
        
        DECLSPEC_XFGVIRT(IAudioSystemEffectsPropertyStore, ResetVolatilePropertyStore)
        HRESULT ( STDMETHODCALLTYPE *ResetVolatilePropertyStore )( 
            IAudioSystemEffectsPropertyStore * This);
        
        DECLSPEC_XFGVIRT(IAudioSystemEffectsPropertyStore, RegisterPropertyChangeNotification)
        HRESULT ( STDMETHODCALLTYPE *RegisterPropertyChangeNotification )( 
            IAudioSystemEffectsPropertyStore * This,
            /* [annotation][in] */ 
            _In_  IAudioSystemEffectsPropertyChangeNotificationClient *callback);
        
        DECLSPEC_XFGVIRT(IAudioSystemEffectsPropertyStore, UnregisterPropertyChangeNotification)
        HRESULT ( STDMETHODCALLTYPE *UnregisterPropertyChangeNotification )( 
            IAudioSystemEffectsPropertyStore * This,
            /* [annotation][in] */ 
            _In_  IAudioSystemEffectsPropertyChangeNotificationClient *callback);
        
        END_INTERFACE
    } IAudioSystemEffectsPropertyStoreVtbl;

    interface IAudioSystemEffectsPropertyStore
    {
        CONST_VTBL struct IAudioSystemEffectsPropertyStoreVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAudioSystemEffectsPropertyStore_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAudioSystemEffectsPropertyStore_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAudioSystemEffectsPropertyStore_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAudioSystemEffectsPropertyStore_OpenDefaultPropertyStore(This,stgmAccess,propStore)	\
    ( (This)->lpVtbl -> OpenDefaultPropertyStore(This,stgmAccess,propStore) ) 

#define IAudioSystemEffectsPropertyStore_OpenUserPropertyStore(This,stgmAccess,propStore)	\
    ( (This)->lpVtbl -> OpenUserPropertyStore(This,stgmAccess,propStore) ) 

#define IAudioSystemEffectsPropertyStore_OpenVolatilePropertyStore(This,stgmAccess,propStore)	\
    ( (This)->lpVtbl -> OpenVolatilePropertyStore(This,stgmAccess,propStore) ) 

#define IAudioSystemEffectsPropertyStore_ResetUserPropertyStore(This)	\
    ( (This)->lpVtbl -> ResetUserPropertyStore(This) ) 

#define IAudioSystemEffectsPropertyStore_ResetVolatilePropertyStore(This)	\
    ( (This)->lpVtbl -> ResetVolatilePropertyStore(This) ) 

#define IAudioSystemEffectsPropertyStore_RegisterPropertyChangeNotification(This,callback)	\
    ( (This)->lpVtbl -> RegisterPropertyChangeNotification(This,callback) ) 

#define IAudioSystemEffectsPropertyStore_UnregisterPropertyChangeNotification(This,callback)	\
    ( (This)->lpVtbl -> UnregisterPropertyChangeNotification(This,callback) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAudioSystemEffectsPropertyStore_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mmdeviceapi_0000_0010 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop and Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES)


extern RPC_IF_HANDLE __MIDL_itf_mmdeviceapi_0000_0010_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mmdeviceapi_0000_0010_v0_0_s_ifspec;


#ifndef __MMDeviceAPILib_LIBRARY_DEFINED__
#define __MMDeviceAPILib_LIBRARY_DEFINED__

/* library MMDeviceAPILib */
/* [helpstring][version][uuid] */ 


EXTERN_C const IID LIBID_MMDeviceAPILib;

EXTERN_C const CLSID CLSID_MMDeviceEnumerator;

#ifdef __cplusplus

class DECLSPEC_UUID("BCDE0395-E52F-467C-8E3D-C4579291692E")
MMDeviceEnumerator;
#endif
#endif /* __MMDeviceAPILib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_mmdeviceapi_0000_0011 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_mmdeviceapi_0000_0011_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mmdeviceapi_0000_0011_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif



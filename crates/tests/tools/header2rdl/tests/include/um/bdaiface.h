

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

#ifndef __bdaiface_h__
#define __bdaiface_h__

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

#ifndef __IBDA_NetworkProvider_FWD_DEFINED__
#define __IBDA_NetworkProvider_FWD_DEFINED__
typedef interface IBDA_NetworkProvider IBDA_NetworkProvider;

#endif 	/* __IBDA_NetworkProvider_FWD_DEFINED__ */


#ifndef __IBDA_EthernetFilter_FWD_DEFINED__
#define __IBDA_EthernetFilter_FWD_DEFINED__
typedef interface IBDA_EthernetFilter IBDA_EthernetFilter;

#endif 	/* __IBDA_EthernetFilter_FWD_DEFINED__ */


#ifndef __IBDA_IPV4Filter_FWD_DEFINED__
#define __IBDA_IPV4Filter_FWD_DEFINED__
typedef interface IBDA_IPV4Filter IBDA_IPV4Filter;

#endif 	/* __IBDA_IPV4Filter_FWD_DEFINED__ */


#ifndef __IBDA_IPV6Filter_FWD_DEFINED__
#define __IBDA_IPV6Filter_FWD_DEFINED__
typedef interface IBDA_IPV6Filter IBDA_IPV6Filter;

#endif 	/* __IBDA_IPV6Filter_FWD_DEFINED__ */


#ifndef __IBDA_DeviceControl_FWD_DEFINED__
#define __IBDA_DeviceControl_FWD_DEFINED__
typedef interface IBDA_DeviceControl IBDA_DeviceControl;

#endif 	/* __IBDA_DeviceControl_FWD_DEFINED__ */


#ifndef __IBDA_PinControl_FWD_DEFINED__
#define __IBDA_PinControl_FWD_DEFINED__
typedef interface IBDA_PinControl IBDA_PinControl;

#endif 	/* __IBDA_PinControl_FWD_DEFINED__ */


#ifndef __IBDA_SignalProperties_FWD_DEFINED__
#define __IBDA_SignalProperties_FWD_DEFINED__
typedef interface IBDA_SignalProperties IBDA_SignalProperties;

#endif 	/* __IBDA_SignalProperties_FWD_DEFINED__ */


#ifndef __IBDA_SignalStatistics_FWD_DEFINED__
#define __IBDA_SignalStatistics_FWD_DEFINED__
typedef interface IBDA_SignalStatistics IBDA_SignalStatistics;

#endif 	/* __IBDA_SignalStatistics_FWD_DEFINED__ */


#ifndef __IBDA_Topology_FWD_DEFINED__
#define __IBDA_Topology_FWD_DEFINED__
typedef interface IBDA_Topology IBDA_Topology;

#endif 	/* __IBDA_Topology_FWD_DEFINED__ */


#ifndef __IBDA_VoidTransform_FWD_DEFINED__
#define __IBDA_VoidTransform_FWD_DEFINED__
typedef interface IBDA_VoidTransform IBDA_VoidTransform;

#endif 	/* __IBDA_VoidTransform_FWD_DEFINED__ */


#ifndef __IBDA_NullTransform_FWD_DEFINED__
#define __IBDA_NullTransform_FWD_DEFINED__
typedef interface IBDA_NullTransform IBDA_NullTransform;

#endif 	/* __IBDA_NullTransform_FWD_DEFINED__ */


#ifndef __IBDA_FrequencyFilter_FWD_DEFINED__
#define __IBDA_FrequencyFilter_FWD_DEFINED__
typedef interface IBDA_FrequencyFilter IBDA_FrequencyFilter;

#endif 	/* __IBDA_FrequencyFilter_FWD_DEFINED__ */


#ifndef __IBDA_LNBInfo_FWD_DEFINED__
#define __IBDA_LNBInfo_FWD_DEFINED__
typedef interface IBDA_LNBInfo IBDA_LNBInfo;

#endif 	/* __IBDA_LNBInfo_FWD_DEFINED__ */


#ifndef __IBDA_DiseqCommand_FWD_DEFINED__
#define __IBDA_DiseqCommand_FWD_DEFINED__
typedef interface IBDA_DiseqCommand IBDA_DiseqCommand;

#endif 	/* __IBDA_DiseqCommand_FWD_DEFINED__ */


#ifndef __IBDA_AutoDemodulate_FWD_DEFINED__
#define __IBDA_AutoDemodulate_FWD_DEFINED__
typedef interface IBDA_AutoDemodulate IBDA_AutoDemodulate;

#endif 	/* __IBDA_AutoDemodulate_FWD_DEFINED__ */


#ifndef __IBDA_AutoDemodulateEx_FWD_DEFINED__
#define __IBDA_AutoDemodulateEx_FWD_DEFINED__
typedef interface IBDA_AutoDemodulateEx IBDA_AutoDemodulateEx;

#endif 	/* __IBDA_AutoDemodulateEx_FWD_DEFINED__ */


#ifndef __IBDA_DigitalDemodulator_FWD_DEFINED__
#define __IBDA_DigitalDemodulator_FWD_DEFINED__
typedef interface IBDA_DigitalDemodulator IBDA_DigitalDemodulator;

#endif 	/* __IBDA_DigitalDemodulator_FWD_DEFINED__ */


#ifndef __IBDA_DigitalDemodulator2_FWD_DEFINED__
#define __IBDA_DigitalDemodulator2_FWD_DEFINED__
typedef interface IBDA_DigitalDemodulator2 IBDA_DigitalDemodulator2;

#endif 	/* __IBDA_DigitalDemodulator2_FWD_DEFINED__ */


#ifndef __IBDA_DigitalDemodulator3_FWD_DEFINED__
#define __IBDA_DigitalDemodulator3_FWD_DEFINED__
typedef interface IBDA_DigitalDemodulator3 IBDA_DigitalDemodulator3;

#endif 	/* __IBDA_DigitalDemodulator3_FWD_DEFINED__ */


#ifndef __ICCSubStreamFiltering_FWD_DEFINED__
#define __ICCSubStreamFiltering_FWD_DEFINED__
typedef interface ICCSubStreamFiltering ICCSubStreamFiltering;

#endif 	/* __ICCSubStreamFiltering_FWD_DEFINED__ */


#ifndef __IBDA_IPSinkControl_FWD_DEFINED__
#define __IBDA_IPSinkControl_FWD_DEFINED__
typedef interface IBDA_IPSinkControl IBDA_IPSinkControl;

#endif 	/* __IBDA_IPSinkControl_FWD_DEFINED__ */


#ifndef __IBDA_IPSinkInfo_FWD_DEFINED__
#define __IBDA_IPSinkInfo_FWD_DEFINED__
typedef interface IBDA_IPSinkInfo IBDA_IPSinkInfo;

#endif 	/* __IBDA_IPSinkInfo_FWD_DEFINED__ */


#ifndef __IEnumPIDMap_FWD_DEFINED__
#define __IEnumPIDMap_FWD_DEFINED__
typedef interface IEnumPIDMap IEnumPIDMap;

#endif 	/* __IEnumPIDMap_FWD_DEFINED__ */


#ifndef __IMPEG2PIDMap_FWD_DEFINED__
#define __IMPEG2PIDMap_FWD_DEFINED__
typedef interface IMPEG2PIDMap IMPEG2PIDMap;

#endif 	/* __IMPEG2PIDMap_FWD_DEFINED__ */


#ifndef __IFrequencyMap_FWD_DEFINED__
#define __IFrequencyMap_FWD_DEFINED__
typedef interface IFrequencyMap IFrequencyMap;

#endif 	/* __IFrequencyMap_FWD_DEFINED__ */


#ifndef __IBDA_EasMessage_FWD_DEFINED__
#define __IBDA_EasMessage_FWD_DEFINED__
typedef interface IBDA_EasMessage IBDA_EasMessage;

#endif 	/* __IBDA_EasMessage_FWD_DEFINED__ */


#ifndef __IBDA_TransportStreamInfo_FWD_DEFINED__
#define __IBDA_TransportStreamInfo_FWD_DEFINED__
typedef interface IBDA_TransportStreamInfo IBDA_TransportStreamInfo;

#endif 	/* __IBDA_TransportStreamInfo_FWD_DEFINED__ */


#ifndef __IBDA_ConditionalAccess_FWD_DEFINED__
#define __IBDA_ConditionalAccess_FWD_DEFINED__
typedef interface IBDA_ConditionalAccess IBDA_ConditionalAccess;

#endif 	/* __IBDA_ConditionalAccess_FWD_DEFINED__ */


#ifndef __IBDA_DiagnosticProperties_FWD_DEFINED__
#define __IBDA_DiagnosticProperties_FWD_DEFINED__
typedef interface IBDA_DiagnosticProperties IBDA_DiagnosticProperties;

#endif 	/* __IBDA_DiagnosticProperties_FWD_DEFINED__ */


#ifndef __IBDA_DRM_FWD_DEFINED__
#define __IBDA_DRM_FWD_DEFINED__
typedef interface IBDA_DRM IBDA_DRM;

#endif 	/* __IBDA_DRM_FWD_DEFINED__ */


#ifndef __IBDA_NameValueService_FWD_DEFINED__
#define __IBDA_NameValueService_FWD_DEFINED__
typedef interface IBDA_NameValueService IBDA_NameValueService;

#endif 	/* __IBDA_NameValueService_FWD_DEFINED__ */


#ifndef __IBDA_ConditionalAccessEx_FWD_DEFINED__
#define __IBDA_ConditionalAccessEx_FWD_DEFINED__
typedef interface IBDA_ConditionalAccessEx IBDA_ConditionalAccessEx;

#endif 	/* __IBDA_ConditionalAccessEx_FWD_DEFINED__ */


#ifndef __IBDA_ISDBConditionalAccess_FWD_DEFINED__
#define __IBDA_ISDBConditionalAccess_FWD_DEFINED__
typedef interface IBDA_ISDBConditionalAccess IBDA_ISDBConditionalAccess;

#endif 	/* __IBDA_ISDBConditionalAccess_FWD_DEFINED__ */


#ifndef __IBDA_EventingService_FWD_DEFINED__
#define __IBDA_EventingService_FWD_DEFINED__
typedef interface IBDA_EventingService IBDA_EventingService;

#endif 	/* __IBDA_EventingService_FWD_DEFINED__ */


#ifndef __IBDA_AUX_FWD_DEFINED__
#define __IBDA_AUX_FWD_DEFINED__
typedef interface IBDA_AUX IBDA_AUX;

#endif 	/* __IBDA_AUX_FWD_DEFINED__ */


#ifndef __IBDA_Encoder_FWD_DEFINED__
#define __IBDA_Encoder_FWD_DEFINED__
typedef interface IBDA_Encoder IBDA_Encoder;

#endif 	/* __IBDA_Encoder_FWD_DEFINED__ */


#ifndef __IBDA_FDC_FWD_DEFINED__
#define __IBDA_FDC_FWD_DEFINED__
typedef interface IBDA_FDC IBDA_FDC;

#endif 	/* __IBDA_FDC_FWD_DEFINED__ */


#ifndef __IBDA_GuideDataDeliveryService_FWD_DEFINED__
#define __IBDA_GuideDataDeliveryService_FWD_DEFINED__
typedef interface IBDA_GuideDataDeliveryService IBDA_GuideDataDeliveryService;

#endif 	/* __IBDA_GuideDataDeliveryService_FWD_DEFINED__ */


#ifndef __IBDA_DRMService_FWD_DEFINED__
#define __IBDA_DRMService_FWD_DEFINED__
typedef interface IBDA_DRMService IBDA_DRMService;

#endif 	/* __IBDA_DRMService_FWD_DEFINED__ */


#ifndef __IBDA_WMDRMSession_FWD_DEFINED__
#define __IBDA_WMDRMSession_FWD_DEFINED__
typedef interface IBDA_WMDRMSession IBDA_WMDRMSession;

#endif 	/* __IBDA_WMDRMSession_FWD_DEFINED__ */


#ifndef __IBDA_WMDRMTuner_FWD_DEFINED__
#define __IBDA_WMDRMTuner_FWD_DEFINED__
typedef interface IBDA_WMDRMTuner IBDA_WMDRMTuner;

#endif 	/* __IBDA_WMDRMTuner_FWD_DEFINED__ */


#ifndef __IBDA_DRIDRMService_FWD_DEFINED__
#define __IBDA_DRIDRMService_FWD_DEFINED__
typedef interface IBDA_DRIDRMService IBDA_DRIDRMService;

#endif 	/* __IBDA_DRIDRMService_FWD_DEFINED__ */


#ifndef __IBDA_DRIWMDRMSession_FWD_DEFINED__
#define __IBDA_DRIWMDRMSession_FWD_DEFINED__
typedef interface IBDA_DRIWMDRMSession IBDA_DRIWMDRMSession;

#endif 	/* __IBDA_DRIWMDRMSession_FWD_DEFINED__ */


#ifndef __IBDA_MUX_FWD_DEFINED__
#define __IBDA_MUX_FWD_DEFINED__
typedef interface IBDA_MUX IBDA_MUX;

#endif 	/* __IBDA_MUX_FWD_DEFINED__ */


#ifndef __IBDA_TransportStreamSelector_FWD_DEFINED__
#define __IBDA_TransportStreamSelector_FWD_DEFINED__
typedef interface IBDA_TransportStreamSelector IBDA_TransportStreamSelector;

#endif 	/* __IBDA_TransportStreamSelector_FWD_DEFINED__ */


#ifndef __IBDA_UserActivityService_FWD_DEFINED__
#define __IBDA_UserActivityService_FWD_DEFINED__
typedef interface IBDA_UserActivityService IBDA_UserActivityService;

#endif 	/* __IBDA_UserActivityService_FWD_DEFINED__ */


#ifndef __IESEvent_FWD_DEFINED__
#define __IESEvent_FWD_DEFINED__
typedef interface IESEvent IESEvent;

#endif 	/* __IESEvent_FWD_DEFINED__ */


#ifndef __IESEvents_FWD_DEFINED__
#define __IESEvents_FWD_DEFINED__
typedef interface IESEvents IESEvents;

#endif 	/* __IESEvents_FWD_DEFINED__ */


#ifndef __IBroadcastEvent_FWD_DEFINED__
#define __IBroadcastEvent_FWD_DEFINED__
typedef interface IBroadcastEvent IBroadcastEvent;

#endif 	/* __IBroadcastEvent_FWD_DEFINED__ */


#ifndef __IBroadcastEventEx_FWD_DEFINED__
#define __IBroadcastEventEx_FWD_DEFINED__
typedef interface IBroadcastEventEx IBroadcastEventEx;

#endif 	/* __IBroadcastEventEx_FWD_DEFINED__ */


/* header files for imported files */
#include "unknwn.h"
#include "strmif.h"
#include "BdaTypes.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_bdaiface_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#pragma warning(push)
#pragma warning(disable:4001) 
#pragma once
#pragma warning(push)
#pragma warning(disable:4001) 
#pragma once
#pragma warning(pop)
#pragma warning(pop)
#pragma region Desktop Family
#pragma region Desktop Family
#pragma endregion
typedef /* [v1_enum] */ 
enum SmartCardStatusType
    {
        CardInserted	= 0,
        CardRemoved	= ( CardInserted + 1 ) ,
        CardError	= ( CardRemoved + 1 ) ,
        CardDataChanged	= ( CardError + 1 ) ,
        CardFirmwareUpgrade	= ( CardDataChanged + 1 ) 
    } 	SmartCardStatusType;

typedef /* [v1_enum] */ 
enum SmartCardAssociationType
    {
        NotAssociated	= 0,
        Associated	= ( NotAssociated + 1 ) ,
        AssociationUnknown	= ( Associated + 1 ) 
    } 	SmartCardAssociationType;

typedef /* [v1_enum] */ 
enum LocationCodeSchemeType
    {
        SCTE_18	= 0
    } 	LocationCodeSchemeType;

typedef struct EALocationCodeType
    {
    LocationCodeSchemeType LocationCodeScheme;
    BYTE state_code;
    BYTE county_subdivision;
    WORD county_code;
    } 	EALocationCodeType;

typedef /* [v1_enum] */ 
enum EntitlementType
    {
        Entitled	= 0,
        NotEntitled	= ( Entitled + 1 ) ,
        TechnicalFailure	= ( NotEntitled + 1 ) 
    } 	EntitlementType;

typedef /* [v1_enum] */ 
enum UICloseReasonType
    {
        NotReady	= 0,
        UserClosed	= ( NotReady + 1 ) ,
        SystemClosed	= ( UserClosed + 1 ) ,
        DeviceClosed	= ( SystemClosed + 1 ) ,
        ErrorClosed	= ( DeviceClosed + 1 ) 
    } 	UICloseReasonType;

typedef struct SmartCardApplication
    {
    ApplicationTypeType ApplicationType;
    USHORT ApplicationVersion;
    BSTR pbstrApplicationName;
    BSTR pbstrApplicationURL;
    } 	SmartCardApplication;

typedef /* [v1_enum] */ 
enum BDA_DrmPairingError
    {
        BDA_DrmPairing_Succeeded	= 0,
        BDA_DrmPairing_HardwareFailure	= ( BDA_DrmPairing_Succeeded + 1 ) ,
        BDA_DrmPairing_NeedRevocationData	= ( BDA_DrmPairing_HardwareFailure + 1 ) ,
        BDA_DrmPairing_NeedIndiv	= ( BDA_DrmPairing_NeedRevocationData + 1 ) ,
        BDA_DrmPairing_Other	= ( BDA_DrmPairing_NeedIndiv + 1 ) ,
        BDA_DrmPairing_DrmInitFailed	= ( BDA_DrmPairing_Other + 1 ) ,
        BDA_DrmPairing_DrmNotPaired	= ( BDA_DrmPairing_DrmInitFailed + 1 ) ,
        BDA_DrmPairing_DrmRePairSoon	= ( BDA_DrmPairing_DrmNotPaired + 1 ) ,
        BDA_DrmPairing_Aborted	= ( BDA_DrmPairing_DrmRePairSoon + 1 ) ,
        BDA_DrmPairing_NeedSDKUpdate	= ( BDA_DrmPairing_Aborted + 1 ) 
    } 	BDA_DrmPairingError;

#pragma region Desktop Family
#pragma endregion
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_bdaiface_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_bdaiface_0000_0000_v0_0_s_ifspec;

#ifndef __IBDA_NetworkProvider_INTERFACE_DEFINED__
#define __IBDA_NetworkProvider_INTERFACE_DEFINED__

/* interface IBDA_NetworkProvider */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IBDA_NetworkProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("fd501041-8ebe-11ce-8183-00aa00577da2")
    IBDA_NetworkProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE PutSignalSource( 
            /* [in] */ ULONG ulSignalSource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSignalSource( 
            /* [out][in] */ __RPC__inout ULONG *pulSignalSource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNetworkType( 
            /* [out][in] */ __RPC__inout GUID *pguidNetworkType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PutTuningSpace( 
            /* [in] */ __RPC__in REFGUID guidTuningSpace) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTuningSpace( 
            /* [out][in] */ __RPC__inout GUID *pguidTuingSpace) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegisterDeviceFilter( 
            /* [in] */ __RPC__in_opt IUnknown *pUnkFilterControl,
            /* [out][in] */ __RPC__inout ULONG *ppvRegisitrationContext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnRegisterDeviceFilter( 
            /* [in] */ ULONG pvRegistrationContext) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBDA_NetworkProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBDA_NetworkProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBDA_NetworkProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBDA_NetworkProvider * This);
        
        DECLSPEC_XFGVIRT(IBDA_NetworkProvider, PutSignalSource)
        HRESULT ( STDMETHODCALLTYPE *PutSignalSource )( 
            __RPC__in IBDA_NetworkProvider * This,
            /* [in] */ ULONG ulSignalSource);
        
        DECLSPEC_XFGVIRT(IBDA_NetworkProvider, GetSignalSource)
        HRESULT ( STDMETHODCALLTYPE *GetSignalSource )( 
            __RPC__in IBDA_NetworkProvider * This,
            /* [out][in] */ __RPC__inout ULONG *pulSignalSource);
        
        DECLSPEC_XFGVIRT(IBDA_NetworkProvider, GetNetworkType)
        HRESULT ( STDMETHODCALLTYPE *GetNetworkType )( 
            __RPC__in IBDA_NetworkProvider * This,
            /* [out][in] */ __RPC__inout GUID *pguidNetworkType);
        
        DECLSPEC_XFGVIRT(IBDA_NetworkProvider, PutTuningSpace)
        HRESULT ( STDMETHODCALLTYPE *PutTuningSpace )( 
            __RPC__in IBDA_NetworkProvider * This,
            /* [in] */ __RPC__in REFGUID guidTuningSpace);
        
        DECLSPEC_XFGVIRT(IBDA_NetworkProvider, GetTuningSpace)
        HRESULT ( STDMETHODCALLTYPE *GetTuningSpace )( 
            __RPC__in IBDA_NetworkProvider * This,
            /* [out][in] */ __RPC__inout GUID *pguidTuingSpace);
        
        DECLSPEC_XFGVIRT(IBDA_NetworkProvider, RegisterDeviceFilter)
        HRESULT ( STDMETHODCALLTYPE *RegisterDeviceFilter )( 
            __RPC__in IBDA_NetworkProvider * This,
            /* [in] */ __RPC__in_opt IUnknown *pUnkFilterControl,
            /* [out][in] */ __RPC__inout ULONG *ppvRegisitrationContext);
        
        DECLSPEC_XFGVIRT(IBDA_NetworkProvider, UnRegisterDeviceFilter)
        HRESULT ( STDMETHODCALLTYPE *UnRegisterDeviceFilter )( 
            __RPC__in IBDA_NetworkProvider * This,
            /* [in] */ ULONG pvRegistrationContext);
        
        END_INTERFACE
    } IBDA_NetworkProviderVtbl;

    interface IBDA_NetworkProvider
    {
        CONST_VTBL struct IBDA_NetworkProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBDA_NetworkProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBDA_NetworkProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBDA_NetworkProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBDA_NetworkProvider_PutSignalSource(This,ulSignalSource)	\
    ( (This)->lpVtbl -> PutSignalSource(This,ulSignalSource) ) 

#define IBDA_NetworkProvider_GetSignalSource(This,pulSignalSource)	\
    ( (This)->lpVtbl -> GetSignalSource(This,pulSignalSource) ) 

#define IBDA_NetworkProvider_GetNetworkType(This,pguidNetworkType)	\
    ( (This)->lpVtbl -> GetNetworkType(This,pguidNetworkType) ) 

#define IBDA_NetworkProvider_PutTuningSpace(This,guidTuningSpace)	\
    ( (This)->lpVtbl -> PutTuningSpace(This,guidTuningSpace) ) 

#define IBDA_NetworkProvider_GetTuningSpace(This,pguidTuingSpace)	\
    ( (This)->lpVtbl -> GetTuningSpace(This,pguidTuingSpace) ) 

#define IBDA_NetworkProvider_RegisterDeviceFilter(This,pUnkFilterControl,ppvRegisitrationContext)	\
    ( (This)->lpVtbl -> RegisterDeviceFilter(This,pUnkFilterControl,ppvRegisitrationContext) ) 

#define IBDA_NetworkProvider_UnRegisterDeviceFilter(This,pvRegistrationContext)	\
    ( (This)->lpVtbl -> UnRegisterDeviceFilter(This,pvRegistrationContext) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBDA_NetworkProvider_INTERFACE_DEFINED__ */


#ifndef __IBDA_EthernetFilter_INTERFACE_DEFINED__
#define __IBDA_EthernetFilter_INTERFACE_DEFINED__

/* interface IBDA_EthernetFilter */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IBDA_EthernetFilter;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("71985F43-1CA1-11d3-9CC8-00C04F7971E0")
    IBDA_EthernetFilter : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetMulticastListSize( 
            /* [out][in] */ __RPC__inout ULONG *pulcbAddresses) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PutMulticastList( 
            /* [in] */ ULONG ulcbAddresses,
            /* [size_is][in] */ __RPC__in_ecount_full(ulcbAddresses) BYTE pAddressList[  ]) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMulticastList( 
            /* [out][in] */ __RPC__inout ULONG *pulcbAddresses,
            /* [size_is][out][in] */ __RPC__inout_ecount_full(*pulcbAddresses) BYTE pAddressList[  ]) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PutMulticastMode( 
            /* [in] */ ULONG ulModeMask) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMulticastMode( 
            /* [out] */ __RPC__out ULONG *pulModeMask) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBDA_EthernetFilterVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBDA_EthernetFilter * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBDA_EthernetFilter * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBDA_EthernetFilter * This);
        
        DECLSPEC_XFGVIRT(IBDA_EthernetFilter, GetMulticastListSize)
        HRESULT ( STDMETHODCALLTYPE *GetMulticastListSize )( 
            __RPC__in IBDA_EthernetFilter * This,
            /* [out][in] */ __RPC__inout ULONG *pulcbAddresses);
        
        DECLSPEC_XFGVIRT(IBDA_EthernetFilter, PutMulticastList)
        HRESULT ( STDMETHODCALLTYPE *PutMulticastList )( 
            __RPC__in IBDA_EthernetFilter * This,
            /* [in] */ ULONG ulcbAddresses,
            /* [size_is][in] */ __RPC__in_ecount_full(ulcbAddresses) BYTE pAddressList[  ]);
        
        DECLSPEC_XFGVIRT(IBDA_EthernetFilter, GetMulticastList)
        HRESULT ( STDMETHODCALLTYPE *GetMulticastList )( 
            __RPC__in IBDA_EthernetFilter * This,
            /* [out][in] */ __RPC__inout ULONG *pulcbAddresses,
            /* [size_is][out][in] */ __RPC__inout_ecount_full(*pulcbAddresses) BYTE pAddressList[  ]);
        
        DECLSPEC_XFGVIRT(IBDA_EthernetFilter, PutMulticastMode)
        HRESULT ( STDMETHODCALLTYPE *PutMulticastMode )( 
            __RPC__in IBDA_EthernetFilter * This,
            /* [in] */ ULONG ulModeMask);
        
        DECLSPEC_XFGVIRT(IBDA_EthernetFilter, GetMulticastMode)
        HRESULT ( STDMETHODCALLTYPE *GetMulticastMode )( 
            __RPC__in IBDA_EthernetFilter * This,
            /* [out] */ __RPC__out ULONG *pulModeMask);
        
        END_INTERFACE
    } IBDA_EthernetFilterVtbl;

    interface IBDA_EthernetFilter
    {
        CONST_VTBL struct IBDA_EthernetFilterVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBDA_EthernetFilter_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBDA_EthernetFilter_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBDA_EthernetFilter_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBDA_EthernetFilter_GetMulticastListSize(This,pulcbAddresses)	\
    ( (This)->lpVtbl -> GetMulticastListSize(This,pulcbAddresses) ) 

#define IBDA_EthernetFilter_PutMulticastList(This,ulcbAddresses,pAddressList)	\
    ( (This)->lpVtbl -> PutMulticastList(This,ulcbAddresses,pAddressList) ) 

#define IBDA_EthernetFilter_GetMulticastList(This,pulcbAddresses,pAddressList)	\
    ( (This)->lpVtbl -> GetMulticastList(This,pulcbAddresses,pAddressList) ) 

#define IBDA_EthernetFilter_PutMulticastMode(This,ulModeMask)	\
    ( (This)->lpVtbl -> PutMulticastMode(This,ulModeMask) ) 

#define IBDA_EthernetFilter_GetMulticastMode(This,pulModeMask)	\
    ( (This)->lpVtbl -> GetMulticastMode(This,pulModeMask) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBDA_EthernetFilter_INTERFACE_DEFINED__ */


#ifndef __IBDA_IPV4Filter_INTERFACE_DEFINED__
#define __IBDA_IPV4Filter_INTERFACE_DEFINED__

/* interface IBDA_IPV4Filter */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IBDA_IPV4Filter;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("71985F44-1CA1-11d3-9CC8-00C04F7971E0")
    IBDA_IPV4Filter : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetMulticastListSize( 
            /* [out][in] */ __RPC__inout ULONG *pulcbAddresses) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PutMulticastList( 
            /* [in] */ ULONG ulcbAddresses,
            /* [size_is][in] */ __RPC__in_ecount_full(ulcbAddresses) BYTE pAddressList[  ]) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMulticastList( 
            /* [out][in] */ __RPC__inout ULONG *pulcbAddresses,
            /* [size_is][out] */ __RPC__out_ecount_full(*pulcbAddresses) BYTE pAddressList[  ]) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PutMulticastMode( 
            /* [in] */ ULONG ulModeMask) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMulticastMode( 
            /* [out] */ __RPC__out ULONG *pulModeMask) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBDA_IPV4FilterVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBDA_IPV4Filter * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBDA_IPV4Filter * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBDA_IPV4Filter * This);
        
        DECLSPEC_XFGVIRT(IBDA_IPV4Filter, GetMulticastListSize)
        HRESULT ( STDMETHODCALLTYPE *GetMulticastListSize )( 
            __RPC__in IBDA_IPV4Filter * This,
            /* [out][in] */ __RPC__inout ULONG *pulcbAddresses);
        
        DECLSPEC_XFGVIRT(IBDA_IPV4Filter, PutMulticastList)
        HRESULT ( STDMETHODCALLTYPE *PutMulticastList )( 
            __RPC__in IBDA_IPV4Filter * This,
            /* [in] */ ULONG ulcbAddresses,
            /* [size_is][in] */ __RPC__in_ecount_full(ulcbAddresses) BYTE pAddressList[  ]);
        
        DECLSPEC_XFGVIRT(IBDA_IPV4Filter, GetMulticastList)
        HRESULT ( STDMETHODCALLTYPE *GetMulticastList )( 
            __RPC__in IBDA_IPV4Filter * This,
            /* [out][in] */ __RPC__inout ULONG *pulcbAddresses,
            /* [size_is][out] */ __RPC__out_ecount_full(*pulcbAddresses) BYTE pAddressList[  ]);
        
        DECLSPEC_XFGVIRT(IBDA_IPV4Filter, PutMulticastMode)
        HRESULT ( STDMETHODCALLTYPE *PutMulticastMode )( 
            __RPC__in IBDA_IPV4Filter * This,
            /* [in] */ ULONG ulModeMask);
        
        DECLSPEC_XFGVIRT(IBDA_IPV4Filter, GetMulticastMode)
        HRESULT ( STDMETHODCALLTYPE *GetMulticastMode )( 
            __RPC__in IBDA_IPV4Filter * This,
            /* [out] */ __RPC__out ULONG *pulModeMask);
        
        END_INTERFACE
    } IBDA_IPV4FilterVtbl;

    interface IBDA_IPV4Filter
    {
        CONST_VTBL struct IBDA_IPV4FilterVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBDA_IPV4Filter_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBDA_IPV4Filter_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBDA_IPV4Filter_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBDA_IPV4Filter_GetMulticastListSize(This,pulcbAddresses)	\
    ( (This)->lpVtbl -> GetMulticastListSize(This,pulcbAddresses) ) 

#define IBDA_IPV4Filter_PutMulticastList(This,ulcbAddresses,pAddressList)	\
    ( (This)->lpVtbl -> PutMulticastList(This,ulcbAddresses,pAddressList) ) 

#define IBDA_IPV4Filter_GetMulticastList(This,pulcbAddresses,pAddressList)	\
    ( (This)->lpVtbl -> GetMulticastList(This,pulcbAddresses,pAddressList) ) 

#define IBDA_IPV4Filter_PutMulticastMode(This,ulModeMask)	\
    ( (This)->lpVtbl -> PutMulticastMode(This,ulModeMask) ) 

#define IBDA_IPV4Filter_GetMulticastMode(This,pulModeMask)	\
    ( (This)->lpVtbl -> GetMulticastMode(This,pulModeMask) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBDA_IPV4Filter_INTERFACE_DEFINED__ */


#ifndef __IBDA_IPV6Filter_INTERFACE_DEFINED__
#define __IBDA_IPV6Filter_INTERFACE_DEFINED__

/* interface IBDA_IPV6Filter */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IBDA_IPV6Filter;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("E1785A74-2A23-4fb3-9245-A8F88017EF33")
    IBDA_IPV6Filter : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetMulticastListSize( 
            /* [out][in] */ __RPC__inout ULONG *pulcbAddresses) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PutMulticastList( 
            /* [in] */ ULONG ulcbAddresses,
            /* [size_is][in] */ __RPC__in_ecount_full(ulcbAddresses) BYTE pAddressList[  ]) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMulticastList( 
            /* [out][in] */ __RPC__inout ULONG *pulcbAddresses,
            /* [size_is][out] */ __RPC__out_ecount_full(*pulcbAddresses) BYTE pAddressList[  ]) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PutMulticastMode( 
            /* [in] */ ULONG ulModeMask) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMulticastMode( 
            /* [out] */ __RPC__out ULONG *pulModeMask) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBDA_IPV6FilterVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBDA_IPV6Filter * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBDA_IPV6Filter * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBDA_IPV6Filter * This);
        
        DECLSPEC_XFGVIRT(IBDA_IPV6Filter, GetMulticastListSize)
        HRESULT ( STDMETHODCALLTYPE *GetMulticastListSize )( 
            __RPC__in IBDA_IPV6Filter * This,
            /* [out][in] */ __RPC__inout ULONG *pulcbAddresses);
        
        DECLSPEC_XFGVIRT(IBDA_IPV6Filter, PutMulticastList)
        HRESULT ( STDMETHODCALLTYPE *PutMulticastList )( 
            __RPC__in IBDA_IPV6Filter * This,
            /* [in] */ ULONG ulcbAddresses,
            /* [size_is][in] */ __RPC__in_ecount_full(ulcbAddresses) BYTE pAddressList[  ]);
        
        DECLSPEC_XFGVIRT(IBDA_IPV6Filter, GetMulticastList)
        HRESULT ( STDMETHODCALLTYPE *GetMulticastList )( 
            __RPC__in IBDA_IPV6Filter * This,
            /* [out][in] */ __RPC__inout ULONG *pulcbAddresses,
            /* [size_is][out] */ __RPC__out_ecount_full(*pulcbAddresses) BYTE pAddressList[  ]);
        
        DECLSPEC_XFGVIRT(IBDA_IPV6Filter, PutMulticastMode)
        HRESULT ( STDMETHODCALLTYPE *PutMulticastMode )( 
            __RPC__in IBDA_IPV6Filter * This,
            /* [in] */ ULONG ulModeMask);
        
        DECLSPEC_XFGVIRT(IBDA_IPV6Filter, GetMulticastMode)
        HRESULT ( STDMETHODCALLTYPE *GetMulticastMode )( 
            __RPC__in IBDA_IPV6Filter * This,
            /* [out] */ __RPC__out ULONG *pulModeMask);
        
        END_INTERFACE
    } IBDA_IPV6FilterVtbl;

    interface IBDA_IPV6Filter
    {
        CONST_VTBL struct IBDA_IPV6FilterVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBDA_IPV6Filter_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBDA_IPV6Filter_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBDA_IPV6Filter_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBDA_IPV6Filter_GetMulticastListSize(This,pulcbAddresses)	\
    ( (This)->lpVtbl -> GetMulticastListSize(This,pulcbAddresses) ) 

#define IBDA_IPV6Filter_PutMulticastList(This,ulcbAddresses,pAddressList)	\
    ( (This)->lpVtbl -> PutMulticastList(This,ulcbAddresses,pAddressList) ) 

#define IBDA_IPV6Filter_GetMulticastList(This,pulcbAddresses,pAddressList)	\
    ( (This)->lpVtbl -> GetMulticastList(This,pulcbAddresses,pAddressList) ) 

#define IBDA_IPV6Filter_PutMulticastMode(This,ulModeMask)	\
    ( (This)->lpVtbl -> PutMulticastMode(This,ulModeMask) ) 

#define IBDA_IPV6Filter_GetMulticastMode(This,pulModeMask)	\
    ( (This)->lpVtbl -> GetMulticastMode(This,pulModeMask) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBDA_IPV6Filter_INTERFACE_DEFINED__ */


#ifndef __IBDA_DeviceControl_INTERFACE_DEFINED__
#define __IBDA_DeviceControl_INTERFACE_DEFINED__

/* interface IBDA_DeviceControl */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IBDA_DeviceControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("FD0A5AF3-B41D-11d2-9C95-00C04F7971E0")
    IBDA_DeviceControl : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE StartChanges( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CheckChanges( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CommitChanges( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetChangeState( 
            /* [out][in] */ __RPC__inout ULONG *pState) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBDA_DeviceControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBDA_DeviceControl * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBDA_DeviceControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBDA_DeviceControl * This);
        
        DECLSPEC_XFGVIRT(IBDA_DeviceControl, StartChanges)
        HRESULT ( STDMETHODCALLTYPE *StartChanges )( 
            __RPC__in IBDA_DeviceControl * This);
        
        DECLSPEC_XFGVIRT(IBDA_DeviceControl, CheckChanges)
        HRESULT ( STDMETHODCALLTYPE *CheckChanges )( 
            __RPC__in IBDA_DeviceControl * This);
        
        DECLSPEC_XFGVIRT(IBDA_DeviceControl, CommitChanges)
        HRESULT ( STDMETHODCALLTYPE *CommitChanges )( 
            __RPC__in IBDA_DeviceControl * This);
        
        DECLSPEC_XFGVIRT(IBDA_DeviceControl, GetChangeState)
        HRESULT ( STDMETHODCALLTYPE *GetChangeState )( 
            __RPC__in IBDA_DeviceControl * This,
            /* [out][in] */ __RPC__inout ULONG *pState);
        
        END_INTERFACE
    } IBDA_DeviceControlVtbl;

    interface IBDA_DeviceControl
    {
        CONST_VTBL struct IBDA_DeviceControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBDA_DeviceControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBDA_DeviceControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBDA_DeviceControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBDA_DeviceControl_StartChanges(This)	\
    ( (This)->lpVtbl -> StartChanges(This) ) 

#define IBDA_DeviceControl_CheckChanges(This)	\
    ( (This)->lpVtbl -> CheckChanges(This) ) 

#define IBDA_DeviceControl_CommitChanges(This)	\
    ( (This)->lpVtbl -> CommitChanges(This) ) 

#define IBDA_DeviceControl_GetChangeState(This,pState)	\
    ( (This)->lpVtbl -> GetChangeState(This,pState) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBDA_DeviceControl_INTERFACE_DEFINED__ */


#ifndef __IBDA_PinControl_INTERFACE_DEFINED__
#define __IBDA_PinControl_INTERFACE_DEFINED__

/* interface IBDA_PinControl */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IBDA_PinControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0DED49D5-A8B7-4d5d-97A1-12B0C195874D")
    IBDA_PinControl : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetPinID( 
            /* [out][in] */ __RPC__inout ULONG *pulPinID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPinType( 
            /* [out][in] */ __RPC__inout ULONG *pulPinType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegistrationContext( 
            /* [out][in] */ __RPC__inout ULONG *pulRegistrationCtx) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBDA_PinControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBDA_PinControl * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBDA_PinControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBDA_PinControl * This);
        
        DECLSPEC_XFGVIRT(IBDA_PinControl, GetPinID)
        HRESULT ( STDMETHODCALLTYPE *GetPinID )( 
            __RPC__in IBDA_PinControl * This,
            /* [out][in] */ __RPC__inout ULONG *pulPinID);
        
        DECLSPEC_XFGVIRT(IBDA_PinControl, GetPinType)
        HRESULT ( STDMETHODCALLTYPE *GetPinType )( 
            __RPC__in IBDA_PinControl * This,
            /* [out][in] */ __RPC__inout ULONG *pulPinType);
        
        DECLSPEC_XFGVIRT(IBDA_PinControl, RegistrationContext)
        HRESULT ( STDMETHODCALLTYPE *RegistrationContext )( 
            __RPC__in IBDA_PinControl * This,
            /* [out][in] */ __RPC__inout ULONG *pulRegistrationCtx);
        
        END_INTERFACE
    } IBDA_PinControlVtbl;

    interface IBDA_PinControl
    {
        CONST_VTBL struct IBDA_PinControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBDA_PinControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBDA_PinControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBDA_PinControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBDA_PinControl_GetPinID(This,pulPinID)	\
    ( (This)->lpVtbl -> GetPinID(This,pulPinID) ) 

#define IBDA_PinControl_GetPinType(This,pulPinType)	\
    ( (This)->lpVtbl -> GetPinType(This,pulPinType) ) 

#define IBDA_PinControl_RegistrationContext(This,pulRegistrationCtx)	\
    ( (This)->lpVtbl -> RegistrationContext(This,pulRegistrationCtx) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBDA_PinControl_INTERFACE_DEFINED__ */


#ifndef __IBDA_SignalProperties_INTERFACE_DEFINED__
#define __IBDA_SignalProperties_INTERFACE_DEFINED__

/* interface IBDA_SignalProperties */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IBDA_SignalProperties;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D2F1644B-B409-11d2-BC69-00A0C9EE9E16")
    IBDA_SignalProperties : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE PutNetworkType( 
            /* [in] */ __RPC__in REFGUID guidNetworkType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNetworkType( 
            /* [out][in] */ __RPC__inout GUID *pguidNetworkType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PutSignalSource( 
            /* [in] */ ULONG ulSignalSource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSignalSource( 
            /* [out][in] */ __RPC__inout ULONG *pulSignalSource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PutTuningSpace( 
            /* [in] */ __RPC__in REFGUID guidTuningSpace) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTuningSpace( 
            /* [out][in] */ __RPC__inout GUID *pguidTuingSpace) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBDA_SignalPropertiesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBDA_SignalProperties * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBDA_SignalProperties * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBDA_SignalProperties * This);
        
        DECLSPEC_XFGVIRT(IBDA_SignalProperties, PutNetworkType)
        HRESULT ( STDMETHODCALLTYPE *PutNetworkType )( 
            __RPC__in IBDA_SignalProperties * This,
            /* [in] */ __RPC__in REFGUID guidNetworkType);
        
        DECLSPEC_XFGVIRT(IBDA_SignalProperties, GetNetworkType)
        HRESULT ( STDMETHODCALLTYPE *GetNetworkType )( 
            __RPC__in IBDA_SignalProperties * This,
            /* [out][in] */ __RPC__inout GUID *pguidNetworkType);
        
        DECLSPEC_XFGVIRT(IBDA_SignalProperties, PutSignalSource)
        HRESULT ( STDMETHODCALLTYPE *PutSignalSource )( 
            __RPC__in IBDA_SignalProperties * This,
            /* [in] */ ULONG ulSignalSource);
        
        DECLSPEC_XFGVIRT(IBDA_SignalProperties, GetSignalSource)
        HRESULT ( STDMETHODCALLTYPE *GetSignalSource )( 
            __RPC__in IBDA_SignalProperties * This,
            /* [out][in] */ __RPC__inout ULONG *pulSignalSource);
        
        DECLSPEC_XFGVIRT(IBDA_SignalProperties, PutTuningSpace)
        HRESULT ( STDMETHODCALLTYPE *PutTuningSpace )( 
            __RPC__in IBDA_SignalProperties * This,
            /* [in] */ __RPC__in REFGUID guidTuningSpace);
        
        DECLSPEC_XFGVIRT(IBDA_SignalProperties, GetTuningSpace)
        HRESULT ( STDMETHODCALLTYPE *GetTuningSpace )( 
            __RPC__in IBDA_SignalProperties * This,
            /* [out][in] */ __RPC__inout GUID *pguidTuingSpace);
        
        END_INTERFACE
    } IBDA_SignalPropertiesVtbl;

    interface IBDA_SignalProperties
    {
        CONST_VTBL struct IBDA_SignalPropertiesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBDA_SignalProperties_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBDA_SignalProperties_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBDA_SignalProperties_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBDA_SignalProperties_PutNetworkType(This,guidNetworkType)	\
    ( (This)->lpVtbl -> PutNetworkType(This,guidNetworkType) ) 

#define IBDA_SignalProperties_GetNetworkType(This,pguidNetworkType)	\
    ( (This)->lpVtbl -> GetNetworkType(This,pguidNetworkType) ) 

#define IBDA_SignalProperties_PutSignalSource(This,ulSignalSource)	\
    ( (This)->lpVtbl -> PutSignalSource(This,ulSignalSource) ) 

#define IBDA_SignalProperties_GetSignalSource(This,pulSignalSource)	\
    ( (This)->lpVtbl -> GetSignalSource(This,pulSignalSource) ) 

#define IBDA_SignalProperties_PutTuningSpace(This,guidTuningSpace)	\
    ( (This)->lpVtbl -> PutTuningSpace(This,guidTuningSpace) ) 

#define IBDA_SignalProperties_GetTuningSpace(This,pguidTuingSpace)	\
    ( (This)->lpVtbl -> GetTuningSpace(This,pguidTuingSpace) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBDA_SignalProperties_INTERFACE_DEFINED__ */


#ifndef __IBDA_SignalStatistics_INTERFACE_DEFINED__
#define __IBDA_SignalStatistics_INTERFACE_DEFINED__

/* interface IBDA_SignalStatistics */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IBDA_SignalStatistics;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1347D106-CF3A-428a-A5CB-AC0D9A2A4338")
    IBDA_SignalStatistics : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE put_SignalStrength( 
            /* [in] */ LONG lDbStrength) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_SignalStrength( 
            /* [out][in] */ __RPC__inout LONG *plDbStrength) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_SignalQuality( 
            /* [in] */ LONG lPercentQuality) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_SignalQuality( 
            /* [out][in] */ __RPC__inout LONG *plPercentQuality) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_SignalPresent( 
            /* [in] */ BOOLEAN fPresent) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_SignalPresent( 
            /* [out][in] */ __RPC__inout BOOLEAN *pfPresent) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_SignalLocked( 
            /* [in] */ BOOLEAN fLocked) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_SignalLocked( 
            /* [out][in] */ __RPC__inout BOOLEAN *pfLocked) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_SampleTime( 
            /* [in] */ LONG lmsSampleTime) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_SampleTime( 
            /* [out][in] */ __RPC__inout LONG *plmsSampleTime) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBDA_SignalStatisticsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBDA_SignalStatistics * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBDA_SignalStatistics * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBDA_SignalStatistics * This);
        
        DECLSPEC_XFGVIRT(IBDA_SignalStatistics, put_SignalStrength)
        HRESULT ( STDMETHODCALLTYPE *put_SignalStrength )( 
            __RPC__in IBDA_SignalStatistics * This,
            /* [in] */ LONG lDbStrength);
        
        DECLSPEC_XFGVIRT(IBDA_SignalStatistics, get_SignalStrength)
        HRESULT ( STDMETHODCALLTYPE *get_SignalStrength )( 
            __RPC__in IBDA_SignalStatistics * This,
            /* [out][in] */ __RPC__inout LONG *plDbStrength);
        
        DECLSPEC_XFGVIRT(IBDA_SignalStatistics, put_SignalQuality)
        HRESULT ( STDMETHODCALLTYPE *put_SignalQuality )( 
            __RPC__in IBDA_SignalStatistics * This,
            /* [in] */ LONG lPercentQuality);
        
        DECLSPEC_XFGVIRT(IBDA_SignalStatistics, get_SignalQuality)
        HRESULT ( STDMETHODCALLTYPE *get_SignalQuality )( 
            __RPC__in IBDA_SignalStatistics * This,
            /* [out][in] */ __RPC__inout LONG *plPercentQuality);
        
        DECLSPEC_XFGVIRT(IBDA_SignalStatistics, put_SignalPresent)
        HRESULT ( STDMETHODCALLTYPE *put_SignalPresent )( 
            __RPC__in IBDA_SignalStatistics * This,
            /* [in] */ BOOLEAN fPresent);
        
        DECLSPEC_XFGVIRT(IBDA_SignalStatistics, get_SignalPresent)
        HRESULT ( STDMETHODCALLTYPE *get_SignalPresent )( 
            __RPC__in IBDA_SignalStatistics * This,
            /* [out][in] */ __RPC__inout BOOLEAN *pfPresent);
        
        DECLSPEC_XFGVIRT(IBDA_SignalStatistics, put_SignalLocked)
        HRESULT ( STDMETHODCALLTYPE *put_SignalLocked )( 
            __RPC__in IBDA_SignalStatistics * This,
            /* [in] */ BOOLEAN fLocked);
        
        DECLSPEC_XFGVIRT(IBDA_SignalStatistics, get_SignalLocked)
        HRESULT ( STDMETHODCALLTYPE *get_SignalLocked )( 
            __RPC__in IBDA_SignalStatistics * This,
            /* [out][in] */ __RPC__inout BOOLEAN *pfLocked);
        
        DECLSPEC_XFGVIRT(IBDA_SignalStatistics, put_SampleTime)
        HRESULT ( STDMETHODCALLTYPE *put_SampleTime )( 
            __RPC__in IBDA_SignalStatistics * This,
            /* [in] */ LONG lmsSampleTime);
        
        DECLSPEC_XFGVIRT(IBDA_SignalStatistics, get_SampleTime)
        HRESULT ( STDMETHODCALLTYPE *get_SampleTime )( 
            __RPC__in IBDA_SignalStatistics * This,
            /* [out][in] */ __RPC__inout LONG *plmsSampleTime);
        
        END_INTERFACE
    } IBDA_SignalStatisticsVtbl;

    interface IBDA_SignalStatistics
    {
        CONST_VTBL struct IBDA_SignalStatisticsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBDA_SignalStatistics_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBDA_SignalStatistics_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBDA_SignalStatistics_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBDA_SignalStatistics_put_SignalStrength(This,lDbStrength)	\
    ( (This)->lpVtbl -> put_SignalStrength(This,lDbStrength) ) 

#define IBDA_SignalStatistics_get_SignalStrength(This,plDbStrength)	\
    ( (This)->lpVtbl -> get_SignalStrength(This,plDbStrength) ) 

#define IBDA_SignalStatistics_put_SignalQuality(This,lPercentQuality)	\
    ( (This)->lpVtbl -> put_SignalQuality(This,lPercentQuality) ) 

#define IBDA_SignalStatistics_get_SignalQuality(This,plPercentQuality)	\
    ( (This)->lpVtbl -> get_SignalQuality(This,plPercentQuality) ) 

#define IBDA_SignalStatistics_put_SignalPresent(This,fPresent)	\
    ( (This)->lpVtbl -> put_SignalPresent(This,fPresent) ) 

#define IBDA_SignalStatistics_get_SignalPresent(This,pfPresent)	\
    ( (This)->lpVtbl -> get_SignalPresent(This,pfPresent) ) 

#define IBDA_SignalStatistics_put_SignalLocked(This,fLocked)	\
    ( (This)->lpVtbl -> put_SignalLocked(This,fLocked) ) 

#define IBDA_SignalStatistics_get_SignalLocked(This,pfLocked)	\
    ( (This)->lpVtbl -> get_SignalLocked(This,pfLocked) ) 

#define IBDA_SignalStatistics_put_SampleTime(This,lmsSampleTime)	\
    ( (This)->lpVtbl -> put_SampleTime(This,lmsSampleTime) ) 

#define IBDA_SignalStatistics_get_SampleTime(This,plmsSampleTime)	\
    ( (This)->lpVtbl -> get_SampleTime(This,plmsSampleTime) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBDA_SignalStatistics_INTERFACE_DEFINED__ */


#ifndef __IBDA_Topology_INTERFACE_DEFINED__
#define __IBDA_Topology_INTERFACE_DEFINED__

/* interface IBDA_Topology */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IBDA_Topology;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("79B56888-7FEA-4690-B45D-38FD3C7849BE")
    IBDA_Topology : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetNodeTypes( 
            /* [out][in] */ __RPC__inout ULONG *pulcNodeTypes,
            /* [in] */ ULONG ulcNodeTypesMax,
            /* [size_is][out][in] */ __RPC__inout_ecount_full(ulcNodeTypesMax) ULONG rgulNodeTypes[  ]) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNodeDescriptors( 
            /* [out][in] */ __RPC__inout ULONG *ulcNodeDescriptors,
            /* [in] */ ULONG ulcNodeDescriptorsMax,
            /* [size_is][out][in] */ __RPC__inout_ecount_full(ulcNodeDescriptorsMax) BDANODE_DESCRIPTOR rgNodeDescriptors[  ]) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNodeInterfaces( 
            /* [in] */ ULONG ulNodeType,
            /* [out][in] */ __RPC__inout ULONG *pulcInterfaces,
            /* [in] */ ULONG ulcInterfacesMax,
            /* [size_is][out][in] */ __RPC__inout_ecount_full(ulcInterfacesMax) GUID rgguidInterfaces[  ]) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPinTypes( 
            /* [out][in] */ __RPC__inout ULONG *pulcPinTypes,
            /* [in] */ ULONG ulcPinTypesMax,
            /* [size_is][out][in] */ __RPC__inout_ecount_full(ulcPinTypesMax) ULONG rgulPinTypes[  ]) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTemplateConnections( 
            /* [out][in] */ __RPC__inout ULONG *pulcConnections,
            /* [in] */ ULONG ulcConnectionsMax,
            /* [size_is][out][in] */ __RPC__inout_ecount_full(ulcConnectionsMax) BDA_TEMPLATE_CONNECTION rgConnections[  ]) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreatePin( 
            /* [in] */ ULONG ulPinType,
            /* [out][in] */ __RPC__inout ULONG *pulPinId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeletePin( 
            /* [in] */ ULONG ulPinId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetMediaType( 
            /* [in] */ ULONG ulPinId,
            /* [in] */ __RPC__in AM_MEDIA_TYPE *pMediaType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetMedium( 
            /* [in] */ ULONG ulPinId,
            /* [in] */ __RPC__in REGPINMEDIUM *pMedium) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateTopology( 
            /* [in] */ ULONG ulInputPinId,
            /* [in] */ ULONG ulOutputPinId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetControlNode( 
            /* [in] */ ULONG ulInputPinId,
            /* [in] */ ULONG ulOutputPinId,
            /* [in] */ ULONG ulNodeType,
            /* [out][in] */ __RPC__deref_inout_opt IUnknown **ppControlNode) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBDA_TopologyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBDA_Topology * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBDA_Topology * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBDA_Topology * This);
        
        DECLSPEC_XFGVIRT(IBDA_Topology, GetNodeTypes)
        HRESULT ( STDMETHODCALLTYPE *GetNodeTypes )( 
            __RPC__in IBDA_Topology * This,
            /* [out][in] */ __RPC__inout ULONG *pulcNodeTypes,
            /* [in] */ ULONG ulcNodeTypesMax,
            /* [size_is][out][in] */ __RPC__inout_ecount_full(ulcNodeTypesMax) ULONG rgulNodeTypes[  ]);
        
        DECLSPEC_XFGVIRT(IBDA_Topology, GetNodeDescriptors)
        HRESULT ( STDMETHODCALLTYPE *GetNodeDescriptors )( 
            __RPC__in IBDA_Topology * This,
            /* [out][in] */ __RPC__inout ULONG *ulcNodeDescriptors,
            /* [in] */ ULONG ulcNodeDescriptorsMax,
            /* [size_is][out][in] */ __RPC__inout_ecount_full(ulcNodeDescriptorsMax) BDANODE_DESCRIPTOR rgNodeDescriptors[  ]);
        
        DECLSPEC_XFGVIRT(IBDA_Topology, GetNodeInterfaces)
        HRESULT ( STDMETHODCALLTYPE *GetNodeInterfaces )( 
            __RPC__in IBDA_Topology * This,
            /* [in] */ ULONG ulNodeType,
            /* [out][in] */ __RPC__inout ULONG *pulcInterfaces,
            /* [in] */ ULONG ulcInterfacesMax,
            /* [size_is][out][in] */ __RPC__inout_ecount_full(ulcInterfacesMax) GUID rgguidInterfaces[  ]);
        
        DECLSPEC_XFGVIRT(IBDA_Topology, GetPinTypes)
        HRESULT ( STDMETHODCALLTYPE *GetPinTypes )( 
            __RPC__in IBDA_Topology * This,
            /* [out][in] */ __RPC__inout ULONG *pulcPinTypes,
            /* [in] */ ULONG ulcPinTypesMax,
            /* [size_is][out][in] */ __RPC__inout_ecount_full(ulcPinTypesMax) ULONG rgulPinTypes[  ]);
        
        DECLSPEC_XFGVIRT(IBDA_Topology, GetTemplateConnections)
        HRESULT ( STDMETHODCALLTYPE *GetTemplateConnections )( 
            __RPC__in IBDA_Topology * This,
            /* [out][in] */ __RPC__inout ULONG *pulcConnections,
            /* [in] */ ULONG ulcConnectionsMax,
            /* [size_is][out][in] */ __RPC__inout_ecount_full(ulcConnectionsMax) BDA_TEMPLATE_CONNECTION rgConnections[  ]);
        
        DECLSPEC_XFGVIRT(IBDA_Topology, CreatePin)
        HRESULT ( STDMETHODCALLTYPE *CreatePin )( 
            __RPC__in IBDA_Topology * This,
            /* [in] */ ULONG ulPinType,
            /* [out][in] */ __RPC__inout ULONG *pulPinId);
        
        DECLSPEC_XFGVIRT(IBDA_Topology, DeletePin)
        HRESULT ( STDMETHODCALLTYPE *DeletePin )( 
            __RPC__in IBDA_Topology * This,
            /* [in] */ ULONG ulPinId);
        
        DECLSPEC_XFGVIRT(IBDA_Topology, SetMediaType)
        HRESULT ( STDMETHODCALLTYPE *SetMediaType )( 
            __RPC__in IBDA_Topology * This,
            /* [in] */ ULONG ulPinId,
            /* [in] */ __RPC__in AM_MEDIA_TYPE *pMediaType);
        
        DECLSPEC_XFGVIRT(IBDA_Topology, SetMedium)
        HRESULT ( STDMETHODCALLTYPE *SetMedium )( 
            __RPC__in IBDA_Topology * This,
            /* [in] */ ULONG ulPinId,
            /* [in] */ __RPC__in REGPINMEDIUM *pMedium);
        
        DECLSPEC_XFGVIRT(IBDA_Topology, CreateTopology)
        HRESULT ( STDMETHODCALLTYPE *CreateTopology )( 
            __RPC__in IBDA_Topology * This,
            /* [in] */ ULONG ulInputPinId,
            /* [in] */ ULONG ulOutputPinId);
        
        DECLSPEC_XFGVIRT(IBDA_Topology, GetControlNode)
        HRESULT ( STDMETHODCALLTYPE *GetControlNode )( 
            __RPC__in IBDA_Topology * This,
            /* [in] */ ULONG ulInputPinId,
            /* [in] */ ULONG ulOutputPinId,
            /* [in] */ ULONG ulNodeType,
            /* [out][in] */ __RPC__deref_inout_opt IUnknown **ppControlNode);
        
        END_INTERFACE
    } IBDA_TopologyVtbl;

    interface IBDA_Topology
    {
        CONST_VTBL struct IBDA_TopologyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBDA_Topology_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBDA_Topology_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBDA_Topology_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBDA_Topology_GetNodeTypes(This,pulcNodeTypes,ulcNodeTypesMax,rgulNodeTypes)	\
    ( (This)->lpVtbl -> GetNodeTypes(This,pulcNodeTypes,ulcNodeTypesMax,rgulNodeTypes) ) 

#define IBDA_Topology_GetNodeDescriptors(This,ulcNodeDescriptors,ulcNodeDescriptorsMax,rgNodeDescriptors)	\
    ( (This)->lpVtbl -> GetNodeDescriptors(This,ulcNodeDescriptors,ulcNodeDescriptorsMax,rgNodeDescriptors) ) 

#define IBDA_Topology_GetNodeInterfaces(This,ulNodeType,pulcInterfaces,ulcInterfacesMax,rgguidInterfaces)	\
    ( (This)->lpVtbl -> GetNodeInterfaces(This,ulNodeType,pulcInterfaces,ulcInterfacesMax,rgguidInterfaces) ) 

#define IBDA_Topology_GetPinTypes(This,pulcPinTypes,ulcPinTypesMax,rgulPinTypes)	\
    ( (This)->lpVtbl -> GetPinTypes(This,pulcPinTypes,ulcPinTypesMax,rgulPinTypes) ) 

#define IBDA_Topology_GetTemplateConnections(This,pulcConnections,ulcConnectionsMax,rgConnections)	\
    ( (This)->lpVtbl -> GetTemplateConnections(This,pulcConnections,ulcConnectionsMax,rgConnections) ) 

#define IBDA_Topology_CreatePin(This,ulPinType,pulPinId)	\
    ( (This)->lpVtbl -> CreatePin(This,ulPinType,pulPinId) ) 

#define IBDA_Topology_DeletePin(This,ulPinId)	\
    ( (This)->lpVtbl -> DeletePin(This,ulPinId) ) 

#define IBDA_Topology_SetMediaType(This,ulPinId,pMediaType)	\
    ( (This)->lpVtbl -> SetMediaType(This,ulPinId,pMediaType) ) 

#define IBDA_Topology_SetMedium(This,ulPinId,pMedium)	\
    ( (This)->lpVtbl -> SetMedium(This,ulPinId,pMedium) ) 

#define IBDA_Topology_CreateTopology(This,ulInputPinId,ulOutputPinId)	\
    ( (This)->lpVtbl -> CreateTopology(This,ulInputPinId,ulOutputPinId) ) 

#define IBDA_Topology_GetControlNode(This,ulInputPinId,ulOutputPinId,ulNodeType,ppControlNode)	\
    ( (This)->lpVtbl -> GetControlNode(This,ulInputPinId,ulOutputPinId,ulNodeType,ppControlNode) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBDA_Topology_INTERFACE_DEFINED__ */


#ifndef __IBDA_VoidTransform_INTERFACE_DEFINED__
#define __IBDA_VoidTransform_INTERFACE_DEFINED__

/* interface IBDA_VoidTransform */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IBDA_VoidTransform;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("71985F46-1CA1-11d3-9CC8-00C04F7971E0")
    IBDA_VoidTransform : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Start( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Stop( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBDA_VoidTransformVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBDA_VoidTransform * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBDA_VoidTransform * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBDA_VoidTransform * This);
        
        DECLSPEC_XFGVIRT(IBDA_VoidTransform, Start)
        HRESULT ( STDMETHODCALLTYPE *Start )( 
            __RPC__in IBDA_VoidTransform * This);
        
        DECLSPEC_XFGVIRT(IBDA_VoidTransform, Stop)
        HRESULT ( STDMETHODCALLTYPE *Stop )( 
            __RPC__in IBDA_VoidTransform * This);
        
        END_INTERFACE
    } IBDA_VoidTransformVtbl;

    interface IBDA_VoidTransform
    {
        CONST_VTBL struct IBDA_VoidTransformVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBDA_VoidTransform_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBDA_VoidTransform_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBDA_VoidTransform_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBDA_VoidTransform_Start(This)	\
    ( (This)->lpVtbl -> Start(This) ) 

#define IBDA_VoidTransform_Stop(This)	\
    ( (This)->lpVtbl -> Stop(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBDA_VoidTransform_INTERFACE_DEFINED__ */


#ifndef __IBDA_NullTransform_INTERFACE_DEFINED__
#define __IBDA_NullTransform_INTERFACE_DEFINED__

/* interface IBDA_NullTransform */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IBDA_NullTransform;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DDF15B0D-BD25-11d2-9CA0-00C04F7971E0")
    IBDA_NullTransform : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Start( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Stop( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBDA_NullTransformVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBDA_NullTransform * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBDA_NullTransform * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBDA_NullTransform * This);
        
        DECLSPEC_XFGVIRT(IBDA_NullTransform, Start)
        HRESULT ( STDMETHODCALLTYPE *Start )( 
            __RPC__in IBDA_NullTransform * This);
        
        DECLSPEC_XFGVIRT(IBDA_NullTransform, Stop)
        HRESULT ( STDMETHODCALLTYPE *Stop )( 
            __RPC__in IBDA_NullTransform * This);
        
        END_INTERFACE
    } IBDA_NullTransformVtbl;

    interface IBDA_NullTransform
    {
        CONST_VTBL struct IBDA_NullTransformVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBDA_NullTransform_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBDA_NullTransform_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBDA_NullTransform_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBDA_NullTransform_Start(This)	\
    ( (This)->lpVtbl -> Start(This) ) 

#define IBDA_NullTransform_Stop(This)	\
    ( (This)->lpVtbl -> Stop(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBDA_NullTransform_INTERFACE_DEFINED__ */


#ifndef __IBDA_FrequencyFilter_INTERFACE_DEFINED__
#define __IBDA_FrequencyFilter_INTERFACE_DEFINED__

/* interface IBDA_FrequencyFilter */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IBDA_FrequencyFilter;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("71985F47-1CA1-11d3-9CC8-00C04F7971E0")
    IBDA_FrequencyFilter : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE put_Autotune( 
            /* [in] */ ULONG ulTransponder) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_Autotune( 
            /* [out][in] */ __RPC__inout ULONG *pulTransponder) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_Frequency( 
            /* [in] */ ULONG ulFrequency) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_Frequency( 
            /* [out][in] */ __RPC__inout ULONG *pulFrequency) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_Polarity( 
            /* [in] */ Polarisation Polarity) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_Polarity( 
            /* [out][in] */ __RPC__inout Polarisation *pPolarity) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_Range( 
            /* [in] */ ULONG ulRange) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_Range( 
            /* [out][in] */ __RPC__inout ULONG *pulRange) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_Bandwidth( 
            /* [in] */ ULONG ulBandwidth) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_Bandwidth( 
            /* [out][in] */ __RPC__inout ULONG *pulBandwidth) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_FrequencyMultiplier( 
            /* [in] */ ULONG ulMultiplier) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_FrequencyMultiplier( 
            /* [out][in] */ __RPC__inout ULONG *pulMultiplier) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBDA_FrequencyFilterVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBDA_FrequencyFilter * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBDA_FrequencyFilter * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBDA_FrequencyFilter * This);
        
        DECLSPEC_XFGVIRT(IBDA_FrequencyFilter, put_Autotune)
        HRESULT ( STDMETHODCALLTYPE *put_Autotune )( 
            __RPC__in IBDA_FrequencyFilter * This,
            /* [in] */ ULONG ulTransponder);
        
        DECLSPEC_XFGVIRT(IBDA_FrequencyFilter, get_Autotune)
        HRESULT ( STDMETHODCALLTYPE *get_Autotune )( 
            __RPC__in IBDA_FrequencyFilter * This,
            /* [out][in] */ __RPC__inout ULONG *pulTransponder);
        
        DECLSPEC_XFGVIRT(IBDA_FrequencyFilter, put_Frequency)
        HRESULT ( STDMETHODCALLTYPE *put_Frequency )( 
            __RPC__in IBDA_FrequencyFilter * This,
            /* [in] */ ULONG ulFrequency);
        
        DECLSPEC_XFGVIRT(IBDA_FrequencyFilter, get_Frequency)
        HRESULT ( STDMETHODCALLTYPE *get_Frequency )( 
            __RPC__in IBDA_FrequencyFilter * This,
            /* [out][in] */ __RPC__inout ULONG *pulFrequency);
        
        DECLSPEC_XFGVIRT(IBDA_FrequencyFilter, put_Polarity)
        HRESULT ( STDMETHODCALLTYPE *put_Polarity )( 
            __RPC__in IBDA_FrequencyFilter * This,
            /* [in] */ Polarisation Polarity);
        
        DECLSPEC_XFGVIRT(IBDA_FrequencyFilter, get_Polarity)
        HRESULT ( STDMETHODCALLTYPE *get_Polarity )( 
            __RPC__in IBDA_FrequencyFilter * This,
            /* [out][in] */ __RPC__inout Polarisation *pPolarity);
        
        DECLSPEC_XFGVIRT(IBDA_FrequencyFilter, put_Range)
        HRESULT ( STDMETHODCALLTYPE *put_Range )( 
            __RPC__in IBDA_FrequencyFilter * This,
            /* [in] */ ULONG ulRange);
        
        DECLSPEC_XFGVIRT(IBDA_FrequencyFilter, get_Range)
        HRESULT ( STDMETHODCALLTYPE *get_Range )( 
            __RPC__in IBDA_FrequencyFilter * This,
            /* [out][in] */ __RPC__inout ULONG *pulRange);
        
        DECLSPEC_XFGVIRT(IBDA_FrequencyFilter, put_Bandwidth)
        HRESULT ( STDMETHODCALLTYPE *put_Bandwidth )( 
            __RPC__in IBDA_FrequencyFilter * This,
            /* [in] */ ULONG ulBandwidth);
        
        DECLSPEC_XFGVIRT(IBDA_FrequencyFilter, get_Bandwidth)
        HRESULT ( STDMETHODCALLTYPE *get_Bandwidth )( 
            __RPC__in IBDA_FrequencyFilter * This,
            /* [out][in] */ __RPC__inout ULONG *pulBandwidth);
        
        DECLSPEC_XFGVIRT(IBDA_FrequencyFilter, put_FrequencyMultiplier)
        HRESULT ( STDMETHODCALLTYPE *put_FrequencyMultiplier )( 
            __RPC__in IBDA_FrequencyFilter * This,
            /* [in] */ ULONG ulMultiplier);
        
        DECLSPEC_XFGVIRT(IBDA_FrequencyFilter, get_FrequencyMultiplier)
        HRESULT ( STDMETHODCALLTYPE *get_FrequencyMultiplier )( 
            __RPC__in IBDA_FrequencyFilter * This,
            /* [out][in] */ __RPC__inout ULONG *pulMultiplier);
        
        END_INTERFACE
    } IBDA_FrequencyFilterVtbl;

    interface IBDA_FrequencyFilter
    {
        CONST_VTBL struct IBDA_FrequencyFilterVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBDA_FrequencyFilter_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBDA_FrequencyFilter_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBDA_FrequencyFilter_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBDA_FrequencyFilter_put_Autotune(This,ulTransponder)	\
    ( (This)->lpVtbl -> put_Autotune(This,ulTransponder) ) 

#define IBDA_FrequencyFilter_get_Autotune(This,pulTransponder)	\
    ( (This)->lpVtbl -> get_Autotune(This,pulTransponder) ) 

#define IBDA_FrequencyFilter_put_Frequency(This,ulFrequency)	\
    ( (This)->lpVtbl -> put_Frequency(This,ulFrequency) ) 

#define IBDA_FrequencyFilter_get_Frequency(This,pulFrequency)	\
    ( (This)->lpVtbl -> get_Frequency(This,pulFrequency) ) 

#define IBDA_FrequencyFilter_put_Polarity(This,Polarity)	\
    ( (This)->lpVtbl -> put_Polarity(This,Polarity) ) 

#define IBDA_FrequencyFilter_get_Polarity(This,pPolarity)	\
    ( (This)->lpVtbl -> get_Polarity(This,pPolarity) ) 

#define IBDA_FrequencyFilter_put_Range(This,ulRange)	\
    ( (This)->lpVtbl -> put_Range(This,ulRange) ) 

#define IBDA_FrequencyFilter_get_Range(This,pulRange)	\
    ( (This)->lpVtbl -> get_Range(This,pulRange) ) 

#define IBDA_FrequencyFilter_put_Bandwidth(This,ulBandwidth)	\
    ( (This)->lpVtbl -> put_Bandwidth(This,ulBandwidth) ) 

#define IBDA_FrequencyFilter_get_Bandwidth(This,pulBandwidth)	\
    ( (This)->lpVtbl -> get_Bandwidth(This,pulBandwidth) ) 

#define IBDA_FrequencyFilter_put_FrequencyMultiplier(This,ulMultiplier)	\
    ( (This)->lpVtbl -> put_FrequencyMultiplier(This,ulMultiplier) ) 

#define IBDA_FrequencyFilter_get_FrequencyMultiplier(This,pulMultiplier)	\
    ( (This)->lpVtbl -> get_FrequencyMultiplier(This,pulMultiplier) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBDA_FrequencyFilter_INTERFACE_DEFINED__ */


#ifndef __IBDA_LNBInfo_INTERFACE_DEFINED__
#define __IBDA_LNBInfo_INTERFACE_DEFINED__

/* interface IBDA_LNBInfo */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IBDA_LNBInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("992CF102-49F9-4719-A664-C4F23E2408F4")
    IBDA_LNBInfo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE put_LocalOscilatorFrequencyLowBand( 
            /* [in] */ ULONG ulLOFLow) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_LocalOscilatorFrequencyLowBand( 
            /* [out][in] */ __RPC__inout ULONG *pulLOFLow) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_LocalOscilatorFrequencyHighBand( 
            /* [in] */ ULONG ulLOFHigh) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_LocalOscilatorFrequencyHighBand( 
            /* [out][in] */ __RPC__inout ULONG *pulLOFHigh) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_HighLowSwitchFrequency( 
            /* [in] */ ULONG ulSwitchFrequency) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_HighLowSwitchFrequency( 
            /* [out][in] */ __RPC__inout ULONG *pulSwitchFrequency) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBDA_LNBInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBDA_LNBInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBDA_LNBInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBDA_LNBInfo * This);
        
        DECLSPEC_XFGVIRT(IBDA_LNBInfo, put_LocalOscilatorFrequencyLowBand)
        HRESULT ( STDMETHODCALLTYPE *put_LocalOscilatorFrequencyLowBand )( 
            __RPC__in IBDA_LNBInfo * This,
            /* [in] */ ULONG ulLOFLow);
        
        DECLSPEC_XFGVIRT(IBDA_LNBInfo, get_LocalOscilatorFrequencyLowBand)
        HRESULT ( STDMETHODCALLTYPE *get_LocalOscilatorFrequencyLowBand )( 
            __RPC__in IBDA_LNBInfo * This,
            /* [out][in] */ __RPC__inout ULONG *pulLOFLow);
        
        DECLSPEC_XFGVIRT(IBDA_LNBInfo, put_LocalOscilatorFrequencyHighBand)
        HRESULT ( STDMETHODCALLTYPE *put_LocalOscilatorFrequencyHighBand )( 
            __RPC__in IBDA_LNBInfo * This,
            /* [in] */ ULONG ulLOFHigh);
        
        DECLSPEC_XFGVIRT(IBDA_LNBInfo, get_LocalOscilatorFrequencyHighBand)
        HRESULT ( STDMETHODCALLTYPE *get_LocalOscilatorFrequencyHighBand )( 
            __RPC__in IBDA_LNBInfo * This,
            /* [out][in] */ __RPC__inout ULONG *pulLOFHigh);
        
        DECLSPEC_XFGVIRT(IBDA_LNBInfo, put_HighLowSwitchFrequency)
        HRESULT ( STDMETHODCALLTYPE *put_HighLowSwitchFrequency )( 
            __RPC__in IBDA_LNBInfo * This,
            /* [in] */ ULONG ulSwitchFrequency);
        
        DECLSPEC_XFGVIRT(IBDA_LNBInfo, get_HighLowSwitchFrequency)
        HRESULT ( STDMETHODCALLTYPE *get_HighLowSwitchFrequency )( 
            __RPC__in IBDA_LNBInfo * This,
            /* [out][in] */ __RPC__inout ULONG *pulSwitchFrequency);
        
        END_INTERFACE
    } IBDA_LNBInfoVtbl;

    interface IBDA_LNBInfo
    {
        CONST_VTBL struct IBDA_LNBInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBDA_LNBInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBDA_LNBInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBDA_LNBInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBDA_LNBInfo_put_LocalOscilatorFrequencyLowBand(This,ulLOFLow)	\
    ( (This)->lpVtbl -> put_LocalOscilatorFrequencyLowBand(This,ulLOFLow) ) 

#define IBDA_LNBInfo_get_LocalOscilatorFrequencyLowBand(This,pulLOFLow)	\
    ( (This)->lpVtbl -> get_LocalOscilatorFrequencyLowBand(This,pulLOFLow) ) 

#define IBDA_LNBInfo_put_LocalOscilatorFrequencyHighBand(This,ulLOFHigh)	\
    ( (This)->lpVtbl -> put_LocalOscilatorFrequencyHighBand(This,ulLOFHigh) ) 

#define IBDA_LNBInfo_get_LocalOscilatorFrequencyHighBand(This,pulLOFHigh)	\
    ( (This)->lpVtbl -> get_LocalOscilatorFrequencyHighBand(This,pulLOFHigh) ) 

#define IBDA_LNBInfo_put_HighLowSwitchFrequency(This,ulSwitchFrequency)	\
    ( (This)->lpVtbl -> put_HighLowSwitchFrequency(This,ulSwitchFrequency) ) 

#define IBDA_LNBInfo_get_HighLowSwitchFrequency(This,pulSwitchFrequency)	\
    ( (This)->lpVtbl -> get_HighLowSwitchFrequency(This,pulSwitchFrequency) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBDA_LNBInfo_INTERFACE_DEFINED__ */


#ifndef __IBDA_DiseqCommand_INTERFACE_DEFINED__
#define __IBDA_DiseqCommand_INTERFACE_DEFINED__

/* interface IBDA_DiseqCommand */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IBDA_DiseqCommand;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F84E2AB0-3C6B-45e3-A0FC-8669D4B81F11")
    IBDA_DiseqCommand : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE put_EnableDiseqCommands( 
            /* [in] */ BOOLEAN bEnable) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_DiseqLNBSource( 
            /* [in] */ ULONG ulLNBSource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_DiseqUseToneBurst( 
            /* [in] */ BOOLEAN bUseToneBurst) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_DiseqRepeats( 
            /* [in] */ ULONG ulRepeats) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_DiseqSendCommand( 
            /* [in] */ ULONG ulRequestId,
            /* [in] */ ULONG ulcbCommandLen,
            /* [size_is][in] */ __RPC__in_ecount_full(ulcbCommandLen) BYTE *pbCommand) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_DiseqResponse( 
            /* [in] */ ULONG ulRequestId,
            /* [out][in] */ __RPC__inout ULONG *pulcbResponseLen,
            /* [size_is][out][in] */ __RPC__inout_ecount_full(*pulcbResponseLen) BYTE pbResponse[  ]) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBDA_DiseqCommandVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBDA_DiseqCommand * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBDA_DiseqCommand * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBDA_DiseqCommand * This);
        
        DECLSPEC_XFGVIRT(IBDA_DiseqCommand, put_EnableDiseqCommands)
        HRESULT ( STDMETHODCALLTYPE *put_EnableDiseqCommands )( 
            __RPC__in IBDA_DiseqCommand * This,
            /* [in] */ BOOLEAN bEnable);
        
        DECLSPEC_XFGVIRT(IBDA_DiseqCommand, put_DiseqLNBSource)
        HRESULT ( STDMETHODCALLTYPE *put_DiseqLNBSource )( 
            __RPC__in IBDA_DiseqCommand * This,
            /* [in] */ ULONG ulLNBSource);
        
        DECLSPEC_XFGVIRT(IBDA_DiseqCommand, put_DiseqUseToneBurst)
        HRESULT ( STDMETHODCALLTYPE *put_DiseqUseToneBurst )( 
            __RPC__in IBDA_DiseqCommand * This,
            /* [in] */ BOOLEAN bUseToneBurst);
        
        DECLSPEC_XFGVIRT(IBDA_DiseqCommand, put_DiseqRepeats)
        HRESULT ( STDMETHODCALLTYPE *put_DiseqRepeats )( 
            __RPC__in IBDA_DiseqCommand * This,
            /* [in] */ ULONG ulRepeats);
        
        DECLSPEC_XFGVIRT(IBDA_DiseqCommand, put_DiseqSendCommand)
        HRESULT ( STDMETHODCALLTYPE *put_DiseqSendCommand )( 
            __RPC__in IBDA_DiseqCommand * This,
            /* [in] */ ULONG ulRequestId,
            /* [in] */ ULONG ulcbCommandLen,
            /* [size_is][in] */ __RPC__in_ecount_full(ulcbCommandLen) BYTE *pbCommand);
        
        DECLSPEC_XFGVIRT(IBDA_DiseqCommand, get_DiseqResponse)
        HRESULT ( STDMETHODCALLTYPE *get_DiseqResponse )( 
            __RPC__in IBDA_DiseqCommand * This,
            /* [in] */ ULONG ulRequestId,
            /* [out][in] */ __RPC__inout ULONG *pulcbResponseLen,
            /* [size_is][out][in] */ __RPC__inout_ecount_full(*pulcbResponseLen) BYTE pbResponse[  ]);
        
        END_INTERFACE
    } IBDA_DiseqCommandVtbl;

    interface IBDA_DiseqCommand
    {
        CONST_VTBL struct IBDA_DiseqCommandVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBDA_DiseqCommand_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBDA_DiseqCommand_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBDA_DiseqCommand_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBDA_DiseqCommand_put_EnableDiseqCommands(This,bEnable)	\
    ( (This)->lpVtbl -> put_EnableDiseqCommands(This,bEnable) ) 

#define IBDA_DiseqCommand_put_DiseqLNBSource(This,ulLNBSource)	\
    ( (This)->lpVtbl -> put_DiseqLNBSource(This,ulLNBSource) ) 

#define IBDA_DiseqCommand_put_DiseqUseToneBurst(This,bUseToneBurst)	\
    ( (This)->lpVtbl -> put_DiseqUseToneBurst(This,bUseToneBurst) ) 

#define IBDA_DiseqCommand_put_DiseqRepeats(This,ulRepeats)	\
    ( (This)->lpVtbl -> put_DiseqRepeats(This,ulRepeats) ) 

#define IBDA_DiseqCommand_put_DiseqSendCommand(This,ulRequestId,ulcbCommandLen,pbCommand)	\
    ( (This)->lpVtbl -> put_DiseqSendCommand(This,ulRequestId,ulcbCommandLen,pbCommand) ) 

#define IBDA_DiseqCommand_get_DiseqResponse(This,ulRequestId,pulcbResponseLen,pbResponse)	\
    ( (This)->lpVtbl -> get_DiseqResponse(This,ulRequestId,pulcbResponseLen,pbResponse) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBDA_DiseqCommand_INTERFACE_DEFINED__ */


#ifndef __IBDA_AutoDemodulate_INTERFACE_DEFINED__
#define __IBDA_AutoDemodulate_INTERFACE_DEFINED__

/* interface IBDA_AutoDemodulate */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IBDA_AutoDemodulate;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DDF15B12-BD25-11d2-9CA0-00C04F7971E0")
    IBDA_AutoDemodulate : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE put_AutoDemodulate( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBDA_AutoDemodulateVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBDA_AutoDemodulate * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBDA_AutoDemodulate * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBDA_AutoDemodulate * This);
        
        DECLSPEC_XFGVIRT(IBDA_AutoDemodulate, put_AutoDemodulate)
        HRESULT ( STDMETHODCALLTYPE *put_AutoDemodulate )( 
            __RPC__in IBDA_AutoDemodulate * This);
        
        END_INTERFACE
    } IBDA_AutoDemodulateVtbl;

    interface IBDA_AutoDemodulate
    {
        CONST_VTBL struct IBDA_AutoDemodulateVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBDA_AutoDemodulate_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBDA_AutoDemodulate_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBDA_AutoDemodulate_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBDA_AutoDemodulate_put_AutoDemodulate(This)	\
    ( (This)->lpVtbl -> put_AutoDemodulate(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBDA_AutoDemodulate_INTERFACE_DEFINED__ */


#ifndef __IBDA_AutoDemodulateEx_INTERFACE_DEFINED__
#define __IBDA_AutoDemodulateEx_INTERFACE_DEFINED__

/* interface IBDA_AutoDemodulateEx */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IBDA_AutoDemodulateEx;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("34518D13-1182-48e6-B28F-B24987787326")
    IBDA_AutoDemodulateEx : public IBDA_AutoDemodulate
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE get_SupportedDeviceNodeTypes( 
            /* [in] */ ULONG ulcDeviceNodeTypesMax,
            /* [out] */ __RPC__out ULONG *pulcDeviceNodeTypes,
            /* [out][in] */ __RPC__inout GUID *pguidDeviceNodeTypes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_SupportedVideoFormats( 
            /* [out] */ __RPC__out ULONG *pulAMTunerModeType,
            /* [out] */ __RPC__out ULONG *pulAnalogVideoStandard) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_AuxInputCount( 
            /* [out][in] */ __RPC__inout ULONG *pulCompositeCount,
            /* [out][in] */ __RPC__inout ULONG *pulSvideoCount) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBDA_AutoDemodulateExVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBDA_AutoDemodulateEx * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBDA_AutoDemodulateEx * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBDA_AutoDemodulateEx * This);
        
        DECLSPEC_XFGVIRT(IBDA_AutoDemodulate, put_AutoDemodulate)
        HRESULT ( STDMETHODCALLTYPE *put_AutoDemodulate )( 
            __RPC__in IBDA_AutoDemodulateEx * This);
        
        DECLSPEC_XFGVIRT(IBDA_AutoDemodulateEx, get_SupportedDeviceNodeTypes)
        HRESULT ( STDMETHODCALLTYPE *get_SupportedDeviceNodeTypes )( 
            __RPC__in IBDA_AutoDemodulateEx * This,
            /* [in] */ ULONG ulcDeviceNodeTypesMax,
            /* [out] */ __RPC__out ULONG *pulcDeviceNodeTypes,
            /* [out][in] */ __RPC__inout GUID *pguidDeviceNodeTypes);
        
        DECLSPEC_XFGVIRT(IBDA_AutoDemodulateEx, get_SupportedVideoFormats)
        HRESULT ( STDMETHODCALLTYPE *get_SupportedVideoFormats )( 
            __RPC__in IBDA_AutoDemodulateEx * This,
            /* [out] */ __RPC__out ULONG *pulAMTunerModeType,
            /* [out] */ __RPC__out ULONG *pulAnalogVideoStandard);
        
        DECLSPEC_XFGVIRT(IBDA_AutoDemodulateEx, get_AuxInputCount)
        HRESULT ( STDMETHODCALLTYPE *get_AuxInputCount )( 
            __RPC__in IBDA_AutoDemodulateEx * This,
            /* [out][in] */ __RPC__inout ULONG *pulCompositeCount,
            /* [out][in] */ __RPC__inout ULONG *pulSvideoCount);
        
        END_INTERFACE
    } IBDA_AutoDemodulateExVtbl;

    interface IBDA_AutoDemodulateEx
    {
        CONST_VTBL struct IBDA_AutoDemodulateExVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBDA_AutoDemodulateEx_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBDA_AutoDemodulateEx_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBDA_AutoDemodulateEx_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBDA_AutoDemodulateEx_put_AutoDemodulate(This)	\
    ( (This)->lpVtbl -> put_AutoDemodulate(This) ) 


#define IBDA_AutoDemodulateEx_get_SupportedDeviceNodeTypes(This,ulcDeviceNodeTypesMax,pulcDeviceNodeTypes,pguidDeviceNodeTypes)	\
    ( (This)->lpVtbl -> get_SupportedDeviceNodeTypes(This,ulcDeviceNodeTypesMax,pulcDeviceNodeTypes,pguidDeviceNodeTypes) ) 

#define IBDA_AutoDemodulateEx_get_SupportedVideoFormats(This,pulAMTunerModeType,pulAnalogVideoStandard)	\
    ( (This)->lpVtbl -> get_SupportedVideoFormats(This,pulAMTunerModeType,pulAnalogVideoStandard) ) 

#define IBDA_AutoDemodulateEx_get_AuxInputCount(This,pulCompositeCount,pulSvideoCount)	\
    ( (This)->lpVtbl -> get_AuxInputCount(This,pulCompositeCount,pulSvideoCount) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBDA_AutoDemodulateEx_INTERFACE_DEFINED__ */


#ifndef __IBDA_DigitalDemodulator_INTERFACE_DEFINED__
#define __IBDA_DigitalDemodulator_INTERFACE_DEFINED__

/* interface IBDA_DigitalDemodulator */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IBDA_DigitalDemodulator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("EF30F379-985B-4d10-B640-A79D5E04E1E0")
    IBDA_DigitalDemodulator : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE put_ModulationType( 
            /* [in] */ __RPC__in ModulationType *pModulationType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_ModulationType( 
            /* [out][in] */ __RPC__inout ModulationType *pModulationType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_InnerFECMethod( 
            /* [in] */ __RPC__in FECMethod *pFECMethod) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_InnerFECMethod( 
            /* [out][in] */ __RPC__inout FECMethod *pFECMethod) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_InnerFECRate( 
            /* [in] */ __RPC__in BinaryConvolutionCodeRate *pFECRate) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_InnerFECRate( 
            /* [out][in] */ __RPC__inout BinaryConvolutionCodeRate *pFECRate) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_OuterFECMethod( 
            /* [in] */ __RPC__in FECMethod *pFECMethod) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_OuterFECMethod( 
            /* [out][in] */ __RPC__inout FECMethod *pFECMethod) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_OuterFECRate( 
            /* [in] */ __RPC__in BinaryConvolutionCodeRate *pFECRate) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_OuterFECRate( 
            /* [out][in] */ __RPC__inout BinaryConvolutionCodeRate *pFECRate) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_SymbolRate( 
            /* [in] */ __RPC__in ULONG *pSymbolRate) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_SymbolRate( 
            /* [out][in] */ __RPC__inout ULONG *pSymbolRate) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_SpectralInversion( 
            /* [in] */ __RPC__in SpectralInversion *pSpectralInversion) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_SpectralInversion( 
            /* [out][in] */ __RPC__inout SpectralInversion *pSpectralInversion) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBDA_DigitalDemodulatorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBDA_DigitalDemodulator * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBDA_DigitalDemodulator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBDA_DigitalDemodulator * This);
        
        DECLSPEC_XFGVIRT(IBDA_DigitalDemodulator, put_ModulationType)
        HRESULT ( STDMETHODCALLTYPE *put_ModulationType )( 
            __RPC__in IBDA_DigitalDemodulator * This,
            /* [in] */ __RPC__in ModulationType *pModulationType);
        
        DECLSPEC_XFGVIRT(IBDA_DigitalDemodulator, get_ModulationType)
        HRESULT ( STDMETHODCALLTYPE *get_ModulationType )( 
            __RPC__in IBDA_DigitalDemodulator * This,
            /* [out][in] */ __RPC__inout ModulationType *pModulationType);
        
        DECLSPEC_XFGVIRT(IBDA_DigitalDemodulator, put_InnerFECMethod)
        HRESULT ( STDMETHODCALLTYPE *put_InnerFECMethod )( 
            __RPC__in IBDA_DigitalDemodulator * This,
            /* [in] */ __RPC__in FECMethod *pFECMethod);
        
        DECLSPEC_XFGVIRT(IBDA_DigitalDemodulator, get_InnerFECMethod)
        HRESULT ( STDMETHODCALLTYPE *get_InnerFECMethod )( 
            __RPC__in IBDA_DigitalDemodulator * This,
            /* [out][in] */ __RPC__inout FECMethod *pFECMethod);
        
        DECLSPEC_XFGVIRT(IBDA_DigitalDemodulator, put_InnerFECRate)
        HRESULT ( STDMETHODCALLTYPE *put_InnerFECRate )( 
            __RPC__in IBDA_DigitalDemodulator * This,
            /* [in] */ __RPC__in BinaryConvolutionCodeRate *pFECRate);
        
        DECLSPEC_XFGVIRT(IBDA_DigitalDemodulator, get_InnerFECRate)
        HRESULT ( STDMETHODCALLTYPE *get_InnerFECRate )( 
            __RPC__in IBDA_DigitalDemodulator * This,
            /* [out][in] */ __RPC__inout BinaryConvolutionCodeRate *pFECRate);
        
        DECLSPEC_XFGVIRT(IBDA_DigitalDemodulator, put_OuterFECMethod)
        HRESULT ( STDMETHODCALLTYPE *put_OuterFECMethod )( 
            __RPC__in IBDA_DigitalDemodulator * This,
            /* [in] */ __RPC__in FECMethod *pFECMethod);
        
        DECLSPEC_XFGVIRT(IBDA_DigitalDemodulator, get_OuterFECMethod)
        HRESULT ( STDMETHODCALLTYPE *get_OuterFECMethod )( 
            __RPC__in IBDA_DigitalDemodulator * This,
            /* [out][in] */ __RPC__inout FECMethod *pFECMethod);
        
        DECLSPEC_XFGVIRT(IBDA_DigitalDemodulator, put_OuterFECRate)
        HRESULT ( STDMETHODCALLTYPE *put_OuterFECRate )( 
            __RPC__in IBDA_DigitalDemodulator * This,
            /* [in] */ __RPC__in BinaryConvolutionCodeRate *pFECRate);
        
        DECLSPEC_XFGVIRT(IBDA_DigitalDemodulator, get_OuterFECRate)
        HRESULT ( STDMETHODCALLTYPE *get_OuterFECRate )( 
            __RPC__in IBDA_DigitalDemodulator * This,
            /* [out][in] */ __RPC__inout BinaryConvolutionCodeRate *pFECRate);
        
        DECLSPEC_XFGVIRT(IBDA_DigitalDemodulator, put_SymbolRate)
        HRESULT ( STDMETHODCALLTYPE *put_SymbolRate )( 
            __RPC__in IBDA_DigitalDemodulator * This,
            /* [in] */ __RPC__in ULONG *pSymbolRate);
        
        DECLSPEC_XFGVIRT(IBDA_DigitalDemodulator, get_SymbolRate)
        HRESULT ( STDMETHODCALLTYPE *get_SymbolRate )( 
            __RPC__in IBDA_DigitalDemodulator * This,
            /* [out][in] */ __RPC__inout ULONG *pSymbolRate);
        
        DECLSPEC_XFGVIRT(IBDA_DigitalDemodulator, put_SpectralInversion)
        HRESULT ( STDMETHODCALLTYPE *put_SpectralInversion )( 
            __RPC__in IBDA_DigitalDemodulator * This,
            /* [in] */ __RPC__in SpectralInversion *pSpectralInversion);
        
        DECLSPEC_XFGVIRT(IBDA_DigitalDemodulator, get_SpectralInversion)
        HRESULT ( STDMETHODCALLTYPE *get_SpectralInversion )( 
            __RPC__in IBDA_DigitalDemodulator * This,
            /* [out][in] */ __RPC__inout SpectralInversion *pSpectralInversion);
        
        END_INTERFACE
    } IBDA_DigitalDemodulatorVtbl;

    interface IBDA_DigitalDemodulator
    {
        CONST_VTBL struct IBDA_DigitalDemodulatorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBDA_DigitalDemodulator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBDA_DigitalDemodulator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBDA_DigitalDemodulator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBDA_DigitalDemodulator_put_ModulationType(This,pModulationType)	\
    ( (This)->lpVtbl -> put_ModulationType(This,pModulationType) ) 

#define IBDA_DigitalDemodulator_get_ModulationType(This,pModulationType)	\
    ( (This)->lpVtbl -> get_ModulationType(This,pModulationType) ) 

#define IBDA_DigitalDemodulator_put_InnerFECMethod(This,pFECMethod)	\
    ( (This)->lpVtbl -> put_InnerFECMethod(This,pFECMethod) ) 

#define IBDA_DigitalDemodulator_get_InnerFECMethod(This,pFECMethod)	\
    ( (This)->lpVtbl -> get_InnerFECMethod(This,pFECMethod) ) 

#define IBDA_DigitalDemodulator_put_InnerFECRate(This,pFECRate)	\
    ( (This)->lpVtbl -> put_InnerFECRate(This,pFECRate) ) 

#define IBDA_DigitalDemodulator_get_InnerFECRate(This,pFECRate)	\
    ( (This)->lpVtbl -> get_InnerFECRate(This,pFECRate) ) 

#define IBDA_DigitalDemodulator_put_OuterFECMethod(This,pFECMethod)	\
    ( (This)->lpVtbl -> put_OuterFECMethod(This,pFECMethod) ) 

#define IBDA_DigitalDemodulator_get_OuterFECMethod(This,pFECMethod)	\
    ( (This)->lpVtbl -> get_OuterFECMethod(This,pFECMethod) ) 

#define IBDA_DigitalDemodulator_put_OuterFECRate(This,pFECRate)	\
    ( (This)->lpVtbl -> put_OuterFECRate(This,pFECRate) ) 

#define IBDA_DigitalDemodulator_get_OuterFECRate(This,pFECRate)	\
    ( (This)->lpVtbl -> get_OuterFECRate(This,pFECRate) ) 

#define IBDA_DigitalDemodulator_put_SymbolRate(This,pSymbolRate)	\
    ( (This)->lpVtbl -> put_SymbolRate(This,pSymbolRate) ) 

#define IBDA_DigitalDemodulator_get_SymbolRate(This,pSymbolRate)	\
    ( (This)->lpVtbl -> get_SymbolRate(This,pSymbolRate) ) 

#define IBDA_DigitalDemodulator_put_SpectralInversion(This,pSpectralInversion)	\
    ( (This)->lpVtbl -> put_SpectralInversion(This,pSpectralInversion) ) 

#define IBDA_DigitalDemodulator_get_SpectralInversion(This,pSpectralInversion)	\
    ( (This)->lpVtbl -> get_SpectralInversion(This,pSpectralInversion) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBDA_DigitalDemodulator_INTERFACE_DEFINED__ */


#ifndef __IBDA_DigitalDemodulator2_INTERFACE_DEFINED__
#define __IBDA_DigitalDemodulator2_INTERFACE_DEFINED__

/* interface IBDA_DigitalDemodulator2 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IBDA_DigitalDemodulator2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("525ED3EE-5CF3-4e1e-9A06-5368A84F9A6E")
    IBDA_DigitalDemodulator2 : public IBDA_DigitalDemodulator
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE put_GuardInterval( 
            /* [in] */ __RPC__in GuardInterval *pGuardInterval) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_GuardInterval( 
            /* [out][in] */ __RPC__inout GuardInterval *pGuardInterval) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_TransmissionMode( 
            /* [in] */ __RPC__in TransmissionMode *pTransmissionMode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_TransmissionMode( 
            /* [out][in] */ __RPC__inout TransmissionMode *pTransmissionMode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_RollOff( 
            /* [in] */ __RPC__in RollOff *pRollOff) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_RollOff( 
            /* [out][in] */ __RPC__inout RollOff *pRollOff) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_Pilot( 
            /* [in] */ __RPC__in Pilot *pPilot) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_Pilot( 
            /* [out][in] */ __RPC__inout Pilot *pPilot) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBDA_DigitalDemodulator2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBDA_DigitalDemodulator2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBDA_DigitalDemodulator2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBDA_DigitalDemodulator2 * This);
        
        DECLSPEC_XFGVIRT(IBDA_DigitalDemodulator, put_ModulationType)
        HRESULT ( STDMETHODCALLTYPE *put_ModulationType )( 
            __RPC__in IBDA_DigitalDemodulator2 * This,
            /* [in] */ __RPC__in ModulationType *pModulationType);
        
        DECLSPEC_XFGVIRT(IBDA_DigitalDemodulator, get_ModulationType)
        HRESULT ( STDMETHODCALLTYPE *get_ModulationType )( 
            __RPC__in IBDA_DigitalDemodulator2 * This,
            /* [out][in] */ __RPC__inout ModulationType *pModulationType);
        
        DECLSPEC_XFGVIRT(IBDA_DigitalDemodulator, put_InnerFECMethod)
        HRESULT ( STDMETHODCALLTYPE *put_InnerFECMethod )( 
            __RPC__in IBDA_DigitalDemodulator2 * This,
            /* [in] */ __RPC__in FECMethod *pFECMethod);
        
        DECLSPEC_XFGVIRT(IBDA_DigitalDemodulator, get_InnerFECMethod)
        HRESULT ( STDMETHODCALLTYPE *get_InnerFECMethod )( 
            __RPC__in IBDA_DigitalDemodulator2 * This,
            /* [out][in] */ __RPC__inout FECMethod *pFECMethod);
        
        DECLSPEC_XFGVIRT(IBDA_DigitalDemodulator, put_InnerFECRate)
        HRESULT ( STDMETHODCALLTYPE *put_InnerFECRate )( 
            __RPC__in IBDA_DigitalDemodulator2 * This,
            /* [in] */ __RPC__in BinaryConvolutionCodeRate *pFECRate);
        
        DECLSPEC_XFGVIRT(IBDA_DigitalDemodulator, get_InnerFECRate)
        HRESULT ( STDMETHODCALLTYPE *get_InnerFECRate )( 
            __RPC__in IBDA_DigitalDemodulator2 * This,
            /* [out][in] */ __RPC__inout BinaryConvolutionCodeRate *pFECRate);
        
        DECLSPEC_XFGVIRT(IBDA_DigitalDemodulator, put_OuterFECMethod)
        HRESULT ( STDMETHODCALLTYPE *put_OuterFECMethod )( 
            __RPC__in IBDA_DigitalDemodulator2 * This,
            /* [in] */ __RPC__in FECMethod *pFECMethod);
        
        DECLSPEC_XFGVIRT(IBDA_DigitalDemodulator, get_OuterFECMethod)
        HRESULT ( STDMETHODCALLTYPE *get_OuterFECMethod )( 
            __RPC__in IBDA_DigitalDemodulator2 * This,
            /* [out][in] */ __RPC__inout FECMethod *pFECMethod);
        
        DECLSPEC_XFGVIRT(IBDA_DigitalDemodulator, put_OuterFECRate)
        HRESULT ( STDMETHODCALLTYPE *put_OuterFECRate )( 
            __RPC__in IBDA_DigitalDemodulator2 * This,
            /* [in] */ __RPC__in BinaryConvolutionCodeRate *pFECRate);
        
        DECLSPEC_XFGVIRT(IBDA_DigitalDemodulator, get_OuterFECRate)
        HRESULT ( STDMETHODCALLTYPE *get_OuterFECRate )( 
            __RPC__in IBDA_DigitalDemodulator2 * This,
            /* [out][in] */ __RPC__inout BinaryConvolutionCodeRate *pFECRate);
        
        DECLSPEC_XFGVIRT(IBDA_DigitalDemodulator, put_SymbolRate)
        HRESULT ( STDMETHODCALLTYPE *put_SymbolRate )( 
            __RPC__in IBDA_DigitalDemodulator2 * This,
            /* [in] */ __RPC__in ULONG *pSymbolRate);
        
        DECLSPEC_XFGVIRT(IBDA_DigitalDemodulator, get_SymbolRate)
        HRESULT ( STDMETHODCALLTYPE *get_SymbolRate )( 
            __RPC__in IBDA_DigitalDemodulator2 * This,
            /* [out][in] */ __RPC__inout ULONG *pSymbolRate);
        
        DECLSPEC_XFGVIRT(IBDA_DigitalDemodulator, put_SpectralInversion)
        HRESULT ( STDMETHODCALLTYPE *put_SpectralInversion )( 
            __RPC__in IBDA_DigitalDemodulator2 * This,
            /* [in] */ __RPC__in SpectralInversion *pSpectralInversion);
        
        DECLSPEC_XFGVIRT(IBDA_DigitalDemodulator, get_SpectralInversion)
        HRESULT ( STDMETHODCALLTYPE *get_SpectralInversion )( 
            __RPC__in IBDA_DigitalDemodulator2 * This,
            /* [out][in] */ __RPC__inout SpectralInversion *pSpectralInversion);
        
        DECLSPEC_XFGVIRT(IBDA_DigitalDemodulator2, put_GuardInterval)
        HRESULT ( STDMETHODCALLTYPE *put_GuardInterval )( 
            __RPC__in IBDA_DigitalDemodulator2 * This,
            /* [in] */ __RPC__in GuardInterval *pGuardInterval);
        
        DECLSPEC_XFGVIRT(IBDA_DigitalDemodulator2, get_GuardInterval)
        HRESULT ( STDMETHODCALLTYPE *get_GuardInterval )( 
            __RPC__in IBDA_DigitalDemodulator2 * This,
            /* [out][in] */ __RPC__inout GuardInterval *pGuardInterval);
        
        DECLSPEC_XFGVIRT(IBDA_DigitalDemodulator2, put_TransmissionMode)
        HRESULT ( STDMETHODCALLTYPE *put_TransmissionMode )( 
            __RPC__in IBDA_DigitalDemodulator2 * This,
            /* [in] */ __RPC__in TransmissionMode *pTransmissionMode);
        
        DECLSPEC_XFGVIRT(IBDA_DigitalDemodulator2, get_TransmissionMode)
        HRESULT ( STDMETHODCALLTYPE *get_TransmissionMode )( 
            __RPC__in IBDA_DigitalDemodulator2 * This,
            /* [out][in] */ __RPC__inout TransmissionMode *pTransmissionMode);
        
        DECLSPEC_XFGVIRT(IBDA_DigitalDemodulator2, put_RollOff)
        HRESULT ( STDMETHODCALLTYPE *put_RollOff )( 
            __RPC__in IBDA_DigitalDemodulator2 * This,
            /* [in] */ __RPC__in RollOff *pRollOff);
        
        DECLSPEC_XFGVIRT(IBDA_DigitalDemodulator2, get_RollOff)
        HRESULT ( STDMETHODCALLTYPE *get_RollOff )( 
            __RPC__in IBDA_DigitalDemodulator2 * This,
            /* [out][in] */ __RPC__inout RollOff *pRollOff);
        
        DECLSPEC_XFGVIRT(IBDA_DigitalDemodulator2, put_Pilot)
        HRESULT ( STDMETHODCALLTYPE *put_Pilot )( 
            __RPC__in IBDA_DigitalDemodulator2 * This,
            /* [in] */ __RPC__in Pilot *pPilot);
        
        DECLSPEC_XFGVIRT(IBDA_DigitalDemodulator2, get_Pilot)
        HRESULT ( STDMETHODCALLTYPE *get_Pilot )( 
            __RPC__in IBDA_DigitalDemodulator2 * This,
            /* [out][in] */ __RPC__inout Pilot *pPilot);
        
        END_INTERFACE
    } IBDA_DigitalDemodulator2Vtbl;

    interface IBDA_DigitalDemodulator2
    {
        CONST_VTBL struct IBDA_DigitalDemodulator2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBDA_DigitalDemodulator2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBDA_DigitalDemodulator2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBDA_DigitalDemodulator2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBDA_DigitalDemodulator2_put_ModulationType(This,pModulationType)	\
    ( (This)->lpVtbl -> put_ModulationType(This,pModulationType) ) 

#define IBDA_DigitalDemodulator2_get_ModulationType(This,pModulationType)	\
    ( (This)->lpVtbl -> get_ModulationType(This,pModulationType) ) 

#define IBDA_DigitalDemodulator2_put_InnerFECMethod(This,pFECMethod)	\
    ( (This)->lpVtbl -> put_InnerFECMethod(This,pFECMethod) ) 

#define IBDA_DigitalDemodulator2_get_InnerFECMethod(This,pFECMethod)	\
    ( (This)->lpVtbl -> get_InnerFECMethod(This,pFECMethod) ) 

#define IBDA_DigitalDemodulator2_put_InnerFECRate(This,pFECRate)	\
    ( (This)->lpVtbl -> put_InnerFECRate(This,pFECRate) ) 

#define IBDA_DigitalDemodulator2_get_InnerFECRate(This,pFECRate)	\
    ( (This)->lpVtbl -> get_InnerFECRate(This,pFECRate) ) 

#define IBDA_DigitalDemodulator2_put_OuterFECMethod(This,pFECMethod)	\
    ( (This)->lpVtbl -> put_OuterFECMethod(This,pFECMethod) ) 

#define IBDA_DigitalDemodulator2_get_OuterFECMethod(This,pFECMethod)	\
    ( (This)->lpVtbl -> get_OuterFECMethod(This,pFECMethod) ) 

#define IBDA_DigitalDemodulator2_put_OuterFECRate(This,pFECRate)	\
    ( (This)->lpVtbl -> put_OuterFECRate(This,pFECRate) ) 

#define IBDA_DigitalDemodulator2_get_OuterFECRate(This,pFECRate)	\
    ( (This)->lpVtbl -> get_OuterFECRate(This,pFECRate) ) 

#define IBDA_DigitalDemodulator2_put_SymbolRate(This,pSymbolRate)	\
    ( (This)->lpVtbl -> put_SymbolRate(This,pSymbolRate) ) 

#define IBDA_DigitalDemodulator2_get_SymbolRate(This,pSymbolRate)	\
    ( (This)->lpVtbl -> get_SymbolRate(This,pSymbolRate) ) 

#define IBDA_DigitalDemodulator2_put_SpectralInversion(This,pSpectralInversion)	\
    ( (This)->lpVtbl -> put_SpectralInversion(This,pSpectralInversion) ) 

#define IBDA_DigitalDemodulator2_get_SpectralInversion(This,pSpectralInversion)	\
    ( (This)->lpVtbl -> get_SpectralInversion(This,pSpectralInversion) ) 


#define IBDA_DigitalDemodulator2_put_GuardInterval(This,pGuardInterval)	\
    ( (This)->lpVtbl -> put_GuardInterval(This,pGuardInterval) ) 

#define IBDA_DigitalDemodulator2_get_GuardInterval(This,pGuardInterval)	\
    ( (This)->lpVtbl -> get_GuardInterval(This,pGuardInterval) ) 

#define IBDA_DigitalDemodulator2_put_TransmissionMode(This,pTransmissionMode)	\
    ( (This)->lpVtbl -> put_TransmissionMode(This,pTransmissionMode) ) 

#define IBDA_DigitalDemodulator2_get_TransmissionMode(This,pTransmissionMode)	\
    ( (This)->lpVtbl -> get_TransmissionMode(This,pTransmissionMode) ) 

#define IBDA_DigitalDemodulator2_put_RollOff(This,pRollOff)	\
    ( (This)->lpVtbl -> put_RollOff(This,pRollOff) ) 

#define IBDA_DigitalDemodulator2_get_RollOff(This,pRollOff)	\
    ( (This)->lpVtbl -> get_RollOff(This,pRollOff) ) 

#define IBDA_DigitalDemodulator2_put_Pilot(This,pPilot)	\
    ( (This)->lpVtbl -> put_Pilot(This,pPilot) ) 

#define IBDA_DigitalDemodulator2_get_Pilot(This,pPilot)	\
    ( (This)->lpVtbl -> get_Pilot(This,pPilot) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBDA_DigitalDemodulator2_INTERFACE_DEFINED__ */


#ifndef __IBDA_DigitalDemodulator3_INTERFACE_DEFINED__
#define __IBDA_DigitalDemodulator3_INTERFACE_DEFINED__

/* interface IBDA_DigitalDemodulator3 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IBDA_DigitalDemodulator3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("13F19604-7D32-4359-93A2-A05205D90AC9")
    IBDA_DigitalDemodulator3 : public IBDA_DigitalDemodulator2
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE put_SignalTimeouts( 
            /* [in] */ __RPC__in BDA_SIGNAL_TIMEOUTS *pSignalTimeouts) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_SignalTimeouts( 
            /* [out][in] */ __RPC__inout BDA_SIGNAL_TIMEOUTS *pSignalTimeouts) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_PLPNumber( 
            /* [in] */ __RPC__in ULONG *pPLPNumber) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_PLPNumber( 
            /* [out][in] */ __RPC__inout ULONG *pPLPNumber) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBDA_DigitalDemodulator3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBDA_DigitalDemodulator3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBDA_DigitalDemodulator3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBDA_DigitalDemodulator3 * This);
        
        DECLSPEC_XFGVIRT(IBDA_DigitalDemodulator, put_ModulationType)
        HRESULT ( STDMETHODCALLTYPE *put_ModulationType )( 
            __RPC__in IBDA_DigitalDemodulator3 * This,
            /* [in] */ __RPC__in ModulationType *pModulationType);
        
        DECLSPEC_XFGVIRT(IBDA_DigitalDemodulator, get_ModulationType)
        HRESULT ( STDMETHODCALLTYPE *get_ModulationType )( 
            __RPC__in IBDA_DigitalDemodulator3 * This,
            /* [out][in] */ __RPC__inout ModulationType *pModulationType);
        
        DECLSPEC_XFGVIRT(IBDA_DigitalDemodulator, put_InnerFECMethod)
        HRESULT ( STDMETHODCALLTYPE *put_InnerFECMethod )( 
            __RPC__in IBDA_DigitalDemodulator3 * This,
            /* [in] */ __RPC__in FECMethod *pFECMethod);
        
        DECLSPEC_XFGVIRT(IBDA_DigitalDemodulator, get_InnerFECMethod)
        HRESULT ( STDMETHODCALLTYPE *get_InnerFECMethod )( 
            __RPC__in IBDA_DigitalDemodulator3 * This,
            /* [out][in] */ __RPC__inout FECMethod *pFECMethod);
        
        DECLSPEC_XFGVIRT(IBDA_DigitalDemodulator, put_InnerFECRate)
        HRESULT ( STDMETHODCALLTYPE *put_InnerFECRate )( 
            __RPC__in IBDA_DigitalDemodulator3 * This,
            /* [in] */ __RPC__in BinaryConvolutionCodeRate *pFECRate);
        
        DECLSPEC_XFGVIRT(IBDA_DigitalDemodulator, get_InnerFECRate)
        HRESULT ( STDMETHODCALLTYPE *get_InnerFECRate )( 
            __RPC__in IBDA_DigitalDemodulator3 * This,
            /* [out][in] */ __RPC__inout BinaryConvolutionCodeRate *pFECRate);
        
        DECLSPEC_XFGVIRT(IBDA_DigitalDemodulator, put_OuterFECMethod)
        HRESULT ( STDMETHODCALLTYPE *put_OuterFECMethod )( 
            __RPC__in IBDA_DigitalDemodulator3 * This,
            /* [in] */ __RPC__in FECMethod *pFECMethod);
        
        DECLSPEC_XFGVIRT(IBDA_DigitalDemodulator, get_OuterFECMethod)
        HRESULT ( STDMETHODCALLTYPE *get_OuterFECMethod )( 
            __RPC__in IBDA_DigitalDemodulator3 * This,
            /* [out][in] */ __RPC__inout FECMethod *pFECMethod);
        
        DECLSPEC_XFGVIRT(IBDA_DigitalDemodulator, put_OuterFECRate)
        HRESULT ( STDMETHODCALLTYPE *put_OuterFECRate )( 
            __RPC__in IBDA_DigitalDemodulator3 * This,
            /* [in] */ __RPC__in BinaryConvolutionCodeRate *pFECRate);
        
        DECLSPEC_XFGVIRT(IBDA_DigitalDemodulator, get_OuterFECRate)
        HRESULT ( STDMETHODCALLTYPE *get_OuterFECRate )( 
            __RPC__in IBDA_DigitalDemodulator3 * This,
            /* [out][in] */ __RPC__inout BinaryConvolutionCodeRate *pFECRate);
        
        DECLSPEC_XFGVIRT(IBDA_DigitalDemodulator, put_SymbolRate)
        HRESULT ( STDMETHODCALLTYPE *put_SymbolRate )( 
            __RPC__in IBDA_DigitalDemodulator3 * This,
            /* [in] */ __RPC__in ULONG *pSymbolRate);
        
        DECLSPEC_XFGVIRT(IBDA_DigitalDemodulator, get_SymbolRate)
        HRESULT ( STDMETHODCALLTYPE *get_SymbolRate )( 
            __RPC__in IBDA_DigitalDemodulator3 * This,
            /* [out][in] */ __RPC__inout ULONG *pSymbolRate);
        
        DECLSPEC_XFGVIRT(IBDA_DigitalDemodulator, put_SpectralInversion)
        HRESULT ( STDMETHODCALLTYPE *put_SpectralInversion )( 
            __RPC__in IBDA_DigitalDemodulator3 * This,
            /* [in] */ __RPC__in SpectralInversion *pSpectralInversion);
        
        DECLSPEC_XFGVIRT(IBDA_DigitalDemodulator, get_SpectralInversion)
        HRESULT ( STDMETHODCALLTYPE *get_SpectralInversion )( 
            __RPC__in IBDA_DigitalDemodulator3 * This,
            /* [out][in] */ __RPC__inout SpectralInversion *pSpectralInversion);
        
        DECLSPEC_XFGVIRT(IBDA_DigitalDemodulator2, put_GuardInterval)
        HRESULT ( STDMETHODCALLTYPE *put_GuardInterval )( 
            __RPC__in IBDA_DigitalDemodulator3 * This,
            /* [in] */ __RPC__in GuardInterval *pGuardInterval);
        
        DECLSPEC_XFGVIRT(IBDA_DigitalDemodulator2, get_GuardInterval)
        HRESULT ( STDMETHODCALLTYPE *get_GuardInterval )( 
            __RPC__in IBDA_DigitalDemodulator3 * This,
            /* [out][in] */ __RPC__inout GuardInterval *pGuardInterval);
        
        DECLSPEC_XFGVIRT(IBDA_DigitalDemodulator2, put_TransmissionMode)
        HRESULT ( STDMETHODCALLTYPE *put_TransmissionMode )( 
            __RPC__in IBDA_DigitalDemodulator3 * This,
            /* [in] */ __RPC__in TransmissionMode *pTransmissionMode);
        
        DECLSPEC_XFGVIRT(IBDA_DigitalDemodulator2, get_TransmissionMode)
        HRESULT ( STDMETHODCALLTYPE *get_TransmissionMode )( 
            __RPC__in IBDA_DigitalDemodulator3 * This,
            /* [out][in] */ __RPC__inout TransmissionMode *pTransmissionMode);
        
        DECLSPEC_XFGVIRT(IBDA_DigitalDemodulator2, put_RollOff)
        HRESULT ( STDMETHODCALLTYPE *put_RollOff )( 
            __RPC__in IBDA_DigitalDemodulator3 * This,
            /* [in] */ __RPC__in RollOff *pRollOff);
        
        DECLSPEC_XFGVIRT(IBDA_DigitalDemodulator2, get_RollOff)
        HRESULT ( STDMETHODCALLTYPE *get_RollOff )( 
            __RPC__in IBDA_DigitalDemodulator3 * This,
            /* [out][in] */ __RPC__inout RollOff *pRollOff);
        
        DECLSPEC_XFGVIRT(IBDA_DigitalDemodulator2, put_Pilot)
        HRESULT ( STDMETHODCALLTYPE *put_Pilot )( 
            __RPC__in IBDA_DigitalDemodulator3 * This,
            /* [in] */ __RPC__in Pilot *pPilot);
        
        DECLSPEC_XFGVIRT(IBDA_DigitalDemodulator2, get_Pilot)
        HRESULT ( STDMETHODCALLTYPE *get_Pilot )( 
            __RPC__in IBDA_DigitalDemodulator3 * This,
            /* [out][in] */ __RPC__inout Pilot *pPilot);
        
        DECLSPEC_XFGVIRT(IBDA_DigitalDemodulator3, put_SignalTimeouts)
        HRESULT ( STDMETHODCALLTYPE *put_SignalTimeouts )( 
            __RPC__in IBDA_DigitalDemodulator3 * This,
            /* [in] */ __RPC__in BDA_SIGNAL_TIMEOUTS *pSignalTimeouts);
        
        DECLSPEC_XFGVIRT(IBDA_DigitalDemodulator3, get_SignalTimeouts)
        HRESULT ( STDMETHODCALLTYPE *get_SignalTimeouts )( 
            __RPC__in IBDA_DigitalDemodulator3 * This,
            /* [out][in] */ __RPC__inout BDA_SIGNAL_TIMEOUTS *pSignalTimeouts);
        
        DECLSPEC_XFGVIRT(IBDA_DigitalDemodulator3, put_PLPNumber)
        HRESULT ( STDMETHODCALLTYPE *put_PLPNumber )( 
            __RPC__in IBDA_DigitalDemodulator3 * This,
            /* [in] */ __RPC__in ULONG *pPLPNumber);
        
        DECLSPEC_XFGVIRT(IBDA_DigitalDemodulator3, get_PLPNumber)
        HRESULT ( STDMETHODCALLTYPE *get_PLPNumber )( 
            __RPC__in IBDA_DigitalDemodulator3 * This,
            /* [out][in] */ __RPC__inout ULONG *pPLPNumber);
        
        END_INTERFACE
    } IBDA_DigitalDemodulator3Vtbl;

    interface IBDA_DigitalDemodulator3
    {
        CONST_VTBL struct IBDA_DigitalDemodulator3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBDA_DigitalDemodulator3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBDA_DigitalDemodulator3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBDA_DigitalDemodulator3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBDA_DigitalDemodulator3_put_ModulationType(This,pModulationType)	\
    ( (This)->lpVtbl -> put_ModulationType(This,pModulationType) ) 

#define IBDA_DigitalDemodulator3_get_ModulationType(This,pModulationType)	\
    ( (This)->lpVtbl -> get_ModulationType(This,pModulationType) ) 

#define IBDA_DigitalDemodulator3_put_InnerFECMethod(This,pFECMethod)	\
    ( (This)->lpVtbl -> put_InnerFECMethod(This,pFECMethod) ) 

#define IBDA_DigitalDemodulator3_get_InnerFECMethod(This,pFECMethod)	\
    ( (This)->lpVtbl -> get_InnerFECMethod(This,pFECMethod) ) 

#define IBDA_DigitalDemodulator3_put_InnerFECRate(This,pFECRate)	\
    ( (This)->lpVtbl -> put_InnerFECRate(This,pFECRate) ) 

#define IBDA_DigitalDemodulator3_get_InnerFECRate(This,pFECRate)	\
    ( (This)->lpVtbl -> get_InnerFECRate(This,pFECRate) ) 

#define IBDA_DigitalDemodulator3_put_OuterFECMethod(This,pFECMethod)	\
    ( (This)->lpVtbl -> put_OuterFECMethod(This,pFECMethod) ) 

#define IBDA_DigitalDemodulator3_get_OuterFECMethod(This,pFECMethod)	\
    ( (This)->lpVtbl -> get_OuterFECMethod(This,pFECMethod) ) 

#define IBDA_DigitalDemodulator3_put_OuterFECRate(This,pFECRate)	\
    ( (This)->lpVtbl -> put_OuterFECRate(This,pFECRate) ) 

#define IBDA_DigitalDemodulator3_get_OuterFECRate(This,pFECRate)	\
    ( (This)->lpVtbl -> get_OuterFECRate(This,pFECRate) ) 

#define IBDA_DigitalDemodulator3_put_SymbolRate(This,pSymbolRate)	\
    ( (This)->lpVtbl -> put_SymbolRate(This,pSymbolRate) ) 

#define IBDA_DigitalDemodulator3_get_SymbolRate(This,pSymbolRate)	\
    ( (This)->lpVtbl -> get_SymbolRate(This,pSymbolRate) ) 

#define IBDA_DigitalDemodulator3_put_SpectralInversion(This,pSpectralInversion)	\
    ( (This)->lpVtbl -> put_SpectralInversion(This,pSpectralInversion) ) 

#define IBDA_DigitalDemodulator3_get_SpectralInversion(This,pSpectralInversion)	\
    ( (This)->lpVtbl -> get_SpectralInversion(This,pSpectralInversion) ) 


#define IBDA_DigitalDemodulator3_put_GuardInterval(This,pGuardInterval)	\
    ( (This)->lpVtbl -> put_GuardInterval(This,pGuardInterval) ) 

#define IBDA_DigitalDemodulator3_get_GuardInterval(This,pGuardInterval)	\
    ( (This)->lpVtbl -> get_GuardInterval(This,pGuardInterval) ) 

#define IBDA_DigitalDemodulator3_put_TransmissionMode(This,pTransmissionMode)	\
    ( (This)->lpVtbl -> put_TransmissionMode(This,pTransmissionMode) ) 

#define IBDA_DigitalDemodulator3_get_TransmissionMode(This,pTransmissionMode)	\
    ( (This)->lpVtbl -> get_TransmissionMode(This,pTransmissionMode) ) 

#define IBDA_DigitalDemodulator3_put_RollOff(This,pRollOff)	\
    ( (This)->lpVtbl -> put_RollOff(This,pRollOff) ) 

#define IBDA_DigitalDemodulator3_get_RollOff(This,pRollOff)	\
    ( (This)->lpVtbl -> get_RollOff(This,pRollOff) ) 

#define IBDA_DigitalDemodulator3_put_Pilot(This,pPilot)	\
    ( (This)->lpVtbl -> put_Pilot(This,pPilot) ) 

#define IBDA_DigitalDemodulator3_get_Pilot(This,pPilot)	\
    ( (This)->lpVtbl -> get_Pilot(This,pPilot) ) 


#define IBDA_DigitalDemodulator3_put_SignalTimeouts(This,pSignalTimeouts)	\
    ( (This)->lpVtbl -> put_SignalTimeouts(This,pSignalTimeouts) ) 

#define IBDA_DigitalDemodulator3_get_SignalTimeouts(This,pSignalTimeouts)	\
    ( (This)->lpVtbl -> get_SignalTimeouts(This,pSignalTimeouts) ) 

#define IBDA_DigitalDemodulator3_put_PLPNumber(This,pPLPNumber)	\
    ( (This)->lpVtbl -> put_PLPNumber(This,pPLPNumber) ) 

#define IBDA_DigitalDemodulator3_get_PLPNumber(This,pPLPNumber)	\
    ( (This)->lpVtbl -> get_PLPNumber(This,pPLPNumber) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBDA_DigitalDemodulator3_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_bdaiface_0000_0019 */
/* [local] */ 

typedef /* [public] */ 
enum __MIDL___MIDL_itf_bdaiface_0000_0019_0001
    {
        KSPROPERTY_IPSINK_MULTICASTLIST	= 0,
        KSPROPERTY_IPSINK_ADAPTER_DESCRIPTION	= ( KSPROPERTY_IPSINK_MULTICASTLIST + 1 ) ,
        KSPROPERTY_IPSINK_ADAPTER_ADDRESS	= ( KSPROPERTY_IPSINK_ADAPTER_DESCRIPTION + 1 ) 
    } 	KSPROPERTY_IPSINK;



extern RPC_IF_HANDLE __MIDL_itf_bdaiface_0000_0019_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_bdaiface_0000_0019_v0_0_s_ifspec;

#ifndef __ICCSubStreamFiltering_INTERFACE_DEFINED__
#define __ICCSubStreamFiltering_INTERFACE_DEFINED__

/* interface ICCSubStreamFiltering */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ICCSubStreamFiltering;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4B2BD7EA-8347-467b-8DBF-62F784929CC3")
    ICCSubStreamFiltering : public IUnknown
    {
    public:
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_SubstreamTypes( 
            /* [retval][out] */ __RPC__out long *pTypes) = 0;
        
        virtual /* [helpstring][propput] */ HRESULT STDMETHODCALLTYPE put_SubstreamTypes( 
            /* [in] */ long Types) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICCSubStreamFilteringVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICCSubStreamFiltering * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICCSubStreamFiltering * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICCSubStreamFiltering * This);
        
        DECLSPEC_XFGVIRT(ICCSubStreamFiltering, get_SubstreamTypes)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SubstreamTypes )( 
            __RPC__in ICCSubStreamFiltering * This,
            /* [retval][out] */ __RPC__out long *pTypes);
        
        DECLSPEC_XFGVIRT(ICCSubStreamFiltering, put_SubstreamTypes)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SubstreamTypes )( 
            __RPC__in ICCSubStreamFiltering * This,
            /* [in] */ long Types);
        
        END_INTERFACE
    } ICCSubStreamFilteringVtbl;

    interface ICCSubStreamFiltering
    {
        CONST_VTBL struct ICCSubStreamFilteringVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICCSubStreamFiltering_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICCSubStreamFiltering_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICCSubStreamFiltering_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICCSubStreamFiltering_get_SubstreamTypes(This,pTypes)	\
    ( (This)->lpVtbl -> get_SubstreamTypes(This,pTypes) ) 

#define ICCSubStreamFiltering_put_SubstreamTypes(This,Types)	\
    ( (This)->lpVtbl -> put_SubstreamTypes(This,Types) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICCSubStreamFiltering_INTERFACE_DEFINED__ */


#ifndef __IBDA_IPSinkControl_INTERFACE_DEFINED__
#define __IBDA_IPSinkControl_INTERFACE_DEFINED__

/* interface IBDA_IPSinkControl */
/* [helpstring][unique][uuid][object] */ 


EXTERN_C const IID IID_IBDA_IPSinkControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3F4DC8E2-4050-11d3-8F4B-00C04F7971E2")
    IBDA_IPSinkControl : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetMulticastList( 
            /* [out][in] */ __RPC__inout unsigned long *pulcbSize,
            /* [out][in] */ __RPC__deref_inout_opt BYTE **pbBuffer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAdapterIPAddress( 
            /* [out][in] */ __RPC__inout unsigned long *pulcbSize,
            /* [out][in] */ __RPC__deref_inout_opt BYTE **pbBuffer) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBDA_IPSinkControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBDA_IPSinkControl * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBDA_IPSinkControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBDA_IPSinkControl * This);
        
        DECLSPEC_XFGVIRT(IBDA_IPSinkControl, GetMulticastList)
        HRESULT ( STDMETHODCALLTYPE *GetMulticastList )( 
            __RPC__in IBDA_IPSinkControl * This,
            /* [out][in] */ __RPC__inout unsigned long *pulcbSize,
            /* [out][in] */ __RPC__deref_inout_opt BYTE **pbBuffer);
        
        DECLSPEC_XFGVIRT(IBDA_IPSinkControl, GetAdapterIPAddress)
        HRESULT ( STDMETHODCALLTYPE *GetAdapterIPAddress )( 
            __RPC__in IBDA_IPSinkControl * This,
            /* [out][in] */ __RPC__inout unsigned long *pulcbSize,
            /* [out][in] */ __RPC__deref_inout_opt BYTE **pbBuffer);
        
        END_INTERFACE
    } IBDA_IPSinkControlVtbl;

    interface IBDA_IPSinkControl
    {
        CONST_VTBL struct IBDA_IPSinkControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBDA_IPSinkControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBDA_IPSinkControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBDA_IPSinkControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBDA_IPSinkControl_GetMulticastList(This,pulcbSize,pbBuffer)	\
    ( (This)->lpVtbl -> GetMulticastList(This,pulcbSize,pbBuffer) ) 

#define IBDA_IPSinkControl_GetAdapterIPAddress(This,pulcbSize,pbBuffer)	\
    ( (This)->lpVtbl -> GetAdapterIPAddress(This,pulcbSize,pbBuffer) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBDA_IPSinkControl_INTERFACE_DEFINED__ */


#ifndef __IBDA_IPSinkInfo_INTERFACE_DEFINED__
#define __IBDA_IPSinkInfo_INTERFACE_DEFINED__

/* interface IBDA_IPSinkInfo */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IBDA_IPSinkInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A750108F-492E-4d51-95F7-649B23FF7AD7")
    IBDA_IPSinkInfo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE get_MulticastList( 
            /* [out][in] */ __RPC__inout ULONG *pulcbAddresses,
            /* [size_is][out] */ __RPC__out_ecount_full(*pulcbAddresses) BYTE **ppbAddressList) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_AdapterIPAddress( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrBuffer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_AdapterDescription( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrBuffer) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBDA_IPSinkInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBDA_IPSinkInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBDA_IPSinkInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBDA_IPSinkInfo * This);
        
        DECLSPEC_XFGVIRT(IBDA_IPSinkInfo, get_MulticastList)
        HRESULT ( STDMETHODCALLTYPE *get_MulticastList )( 
            __RPC__in IBDA_IPSinkInfo * This,
            /* [out][in] */ __RPC__inout ULONG *pulcbAddresses,
            /* [size_is][out] */ __RPC__out_ecount_full(*pulcbAddresses) BYTE **ppbAddressList);
        
        DECLSPEC_XFGVIRT(IBDA_IPSinkInfo, get_AdapterIPAddress)
        HRESULT ( STDMETHODCALLTYPE *get_AdapterIPAddress )( 
            __RPC__in IBDA_IPSinkInfo * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrBuffer);
        
        DECLSPEC_XFGVIRT(IBDA_IPSinkInfo, get_AdapterDescription)
        HRESULT ( STDMETHODCALLTYPE *get_AdapterDescription )( 
            __RPC__in IBDA_IPSinkInfo * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrBuffer);
        
        END_INTERFACE
    } IBDA_IPSinkInfoVtbl;

    interface IBDA_IPSinkInfo
    {
        CONST_VTBL struct IBDA_IPSinkInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBDA_IPSinkInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBDA_IPSinkInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBDA_IPSinkInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBDA_IPSinkInfo_get_MulticastList(This,pulcbAddresses,ppbAddressList)	\
    ( (This)->lpVtbl -> get_MulticastList(This,pulcbAddresses,ppbAddressList) ) 

#define IBDA_IPSinkInfo_get_AdapterIPAddress(This,pbstrBuffer)	\
    ( (This)->lpVtbl -> get_AdapterIPAddress(This,pbstrBuffer) ) 

#define IBDA_IPSinkInfo_get_AdapterDescription(This,pbstrBuffer)	\
    ( (This)->lpVtbl -> get_AdapterDescription(This,pbstrBuffer) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBDA_IPSinkInfo_INTERFACE_DEFINED__ */


#ifndef __IEnumPIDMap_INTERFACE_DEFINED__
#define __IEnumPIDMap_INTERFACE_DEFINED__

/* interface IEnumPIDMap */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IEnumPIDMap;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("afb6c2a2-2c41-11d3-8a60-0000f81e0e4a")
    IEnumPIDMap : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG cRequest,
            /* [size_is][out][in] */ __RPC__inout_ecount_full(cRequest) PID_MAP *pPIDMap,
            /* [out] */ __RPC__out ULONG *pcReceived) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG cRecords) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ __RPC__deref_out_opt IEnumPIDMap **ppIEnumPIDMap) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumPIDMapVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumPIDMap * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumPIDMap * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumPIDMap * This);
        
        DECLSPEC_XFGVIRT(IEnumPIDMap, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IEnumPIDMap * This,
            /* [in] */ ULONG cRequest,
            /* [size_is][out][in] */ __RPC__inout_ecount_full(cRequest) PID_MAP *pPIDMap,
            /* [out] */ __RPC__out ULONG *pcReceived);
        
        DECLSPEC_XFGVIRT(IEnumPIDMap, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumPIDMap * This,
            /* [in] */ ULONG cRecords);
        
        DECLSPEC_XFGVIRT(IEnumPIDMap, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumPIDMap * This);
        
        DECLSPEC_XFGVIRT(IEnumPIDMap, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumPIDMap * This,
            /* [out] */ __RPC__deref_out_opt IEnumPIDMap **ppIEnumPIDMap);
        
        END_INTERFACE
    } IEnumPIDMapVtbl;

    interface IEnumPIDMap
    {
        CONST_VTBL struct IEnumPIDMapVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumPIDMap_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumPIDMap_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumPIDMap_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumPIDMap_Next(This,cRequest,pPIDMap,pcReceived)	\
    ( (This)->lpVtbl -> Next(This,cRequest,pPIDMap,pcReceived) ) 

#define IEnumPIDMap_Skip(This,cRecords)	\
    ( (This)->lpVtbl -> Skip(This,cRecords) ) 

#define IEnumPIDMap_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumPIDMap_Clone(This,ppIEnumPIDMap)	\
    ( (This)->lpVtbl -> Clone(This,ppIEnumPIDMap) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumPIDMap_INTERFACE_DEFINED__ */


#ifndef __IMPEG2PIDMap_INTERFACE_DEFINED__
#define __IMPEG2PIDMap_INTERFACE_DEFINED__

/* interface IMPEG2PIDMap */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IMPEG2PIDMap;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("afb6c2a1-2c41-11d3-8a60-0000f81e0e4a")
    IMPEG2PIDMap : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE MapPID( 
            /* [in] */ ULONG culPID,
            /* [in] */ __RPC__in ULONG *pulPID,
            /* [in] */ MEDIA_SAMPLE_CONTENT MediaSampleContent) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnmapPID( 
            /* [in] */ ULONG culPID,
            /* [in] */ __RPC__in ULONG *pulPID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumPIDMap( 
            /* [out] */ __RPC__deref_out_opt IEnumPIDMap **pIEnumPIDMap) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMPEG2PIDMapVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMPEG2PIDMap * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMPEG2PIDMap * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMPEG2PIDMap * This);
        
        DECLSPEC_XFGVIRT(IMPEG2PIDMap, MapPID)
        HRESULT ( STDMETHODCALLTYPE *MapPID )( 
            __RPC__in IMPEG2PIDMap * This,
            /* [in] */ ULONG culPID,
            /* [in] */ __RPC__in ULONG *pulPID,
            /* [in] */ MEDIA_SAMPLE_CONTENT MediaSampleContent);
        
        DECLSPEC_XFGVIRT(IMPEG2PIDMap, UnmapPID)
        HRESULT ( STDMETHODCALLTYPE *UnmapPID )( 
            __RPC__in IMPEG2PIDMap * This,
            /* [in] */ ULONG culPID,
            /* [in] */ __RPC__in ULONG *pulPID);
        
        DECLSPEC_XFGVIRT(IMPEG2PIDMap, EnumPIDMap)
        HRESULT ( STDMETHODCALLTYPE *EnumPIDMap )( 
            __RPC__in IMPEG2PIDMap * This,
            /* [out] */ __RPC__deref_out_opt IEnumPIDMap **pIEnumPIDMap);
        
        END_INTERFACE
    } IMPEG2PIDMapVtbl;

    interface IMPEG2PIDMap
    {
        CONST_VTBL struct IMPEG2PIDMapVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMPEG2PIDMap_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMPEG2PIDMap_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMPEG2PIDMap_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMPEG2PIDMap_MapPID(This,culPID,pulPID,MediaSampleContent)	\
    ( (This)->lpVtbl -> MapPID(This,culPID,pulPID,MediaSampleContent) ) 

#define IMPEG2PIDMap_UnmapPID(This,culPID,pulPID)	\
    ( (This)->lpVtbl -> UnmapPID(This,culPID,pulPID) ) 

#define IMPEG2PIDMap_EnumPIDMap(This,pIEnumPIDMap)	\
    ( (This)->lpVtbl -> EnumPIDMap(This,pIEnumPIDMap) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMPEG2PIDMap_INTERFACE_DEFINED__ */


#ifndef __IFrequencyMap_INTERFACE_DEFINED__
#define __IFrequencyMap_INTERFACE_DEFINED__

/* interface IFrequencyMap */
/* [restricted][hidden][unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IFrequencyMap;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("06FB45C1-693C-4ea7-B79F-7A6A54D8DEF2")
    IFrequencyMap : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE get_FrequencyMapping( 
            /* [out] */ __RPC__out ULONG *ulCount,
            /* [size_is][size_is][out] */ __RPC__out_ecount_full(1) ULONG **ppulList) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE put_FrequencyMapping( 
            /* [in] */ ULONG ulCount,
            /* [size_is][in] */ __RPC__in_ecount_full(ulCount) ULONG pList[  ]) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE get_CountryCode( 
            /* [out] */ __RPC__out ULONG *pulCountryCode) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE put_CountryCode( 
            /* [in] */ ULONG ulCountryCode) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE get_DefaultFrequencyMapping( 
            /* [in] */ ULONG ulCountryCode,
            /* [out] */ __RPC__out ULONG *pulCount,
            /* [size_is][size_is][out] */ __RPC__out_ecount_full(1) ULONG **ppulList) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE get_CountryCodeList( 
            /* [out] */ __RPC__out ULONG *pulCount,
            /* [size_is][size_is][out] */ __RPC__out_ecount_full(1) ULONG **ppulList) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFrequencyMapVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFrequencyMap * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFrequencyMap * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFrequencyMap * This);
        
        DECLSPEC_XFGVIRT(IFrequencyMap, get_FrequencyMapping)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *get_FrequencyMapping )( 
            __RPC__in IFrequencyMap * This,
            /* [out] */ __RPC__out ULONG *ulCount,
            /* [size_is][size_is][out] */ __RPC__out_ecount_full(1) ULONG **ppulList);
        
        DECLSPEC_XFGVIRT(IFrequencyMap, put_FrequencyMapping)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *put_FrequencyMapping )( 
            __RPC__in IFrequencyMap * This,
            /* [in] */ ULONG ulCount,
            /* [size_is][in] */ __RPC__in_ecount_full(ulCount) ULONG pList[  ]);
        
        DECLSPEC_XFGVIRT(IFrequencyMap, get_CountryCode)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *get_CountryCode )( 
            __RPC__in IFrequencyMap * This,
            /* [out] */ __RPC__out ULONG *pulCountryCode);
        
        DECLSPEC_XFGVIRT(IFrequencyMap, put_CountryCode)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *put_CountryCode )( 
            __RPC__in IFrequencyMap * This,
            /* [in] */ ULONG ulCountryCode);
        
        DECLSPEC_XFGVIRT(IFrequencyMap, get_DefaultFrequencyMapping)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *get_DefaultFrequencyMapping )( 
            __RPC__in IFrequencyMap * This,
            /* [in] */ ULONG ulCountryCode,
            /* [out] */ __RPC__out ULONG *pulCount,
            /* [size_is][size_is][out] */ __RPC__out_ecount_full(1) ULONG **ppulList);
        
        DECLSPEC_XFGVIRT(IFrequencyMap, get_CountryCodeList)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *get_CountryCodeList )( 
            __RPC__in IFrequencyMap * This,
            /* [out] */ __RPC__out ULONG *pulCount,
            /* [size_is][size_is][out] */ __RPC__out_ecount_full(1) ULONG **ppulList);
        
        END_INTERFACE
    } IFrequencyMapVtbl;

    interface IFrequencyMap
    {
        CONST_VTBL struct IFrequencyMapVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFrequencyMap_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFrequencyMap_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFrequencyMap_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFrequencyMap_get_FrequencyMapping(This,ulCount,ppulList)	\
    ( (This)->lpVtbl -> get_FrequencyMapping(This,ulCount,ppulList) ) 

#define IFrequencyMap_put_FrequencyMapping(This,ulCount,pList)	\
    ( (This)->lpVtbl -> put_FrequencyMapping(This,ulCount,pList) ) 

#define IFrequencyMap_get_CountryCode(This,pulCountryCode)	\
    ( (This)->lpVtbl -> get_CountryCode(This,pulCountryCode) ) 

#define IFrequencyMap_put_CountryCode(This,ulCountryCode)	\
    ( (This)->lpVtbl -> put_CountryCode(This,ulCountryCode) ) 

#define IFrequencyMap_get_DefaultFrequencyMapping(This,ulCountryCode,pulCount,ppulList)	\
    ( (This)->lpVtbl -> get_DefaultFrequencyMapping(This,ulCountryCode,pulCount,ppulList) ) 

#define IFrequencyMap_get_CountryCodeList(This,pulCount,ppulList)	\
    ( (This)->lpVtbl -> get_CountryCodeList(This,pulCount,ppulList) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFrequencyMap_INTERFACE_DEFINED__ */


#ifndef __IBDA_EasMessage_INTERFACE_DEFINED__
#define __IBDA_EasMessage_INTERFACE_DEFINED__

/* interface IBDA_EasMessage */
/* [restricted][hidden][unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IBDA_EasMessage;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D806973D-3EBE-46de-8FBB-6358FE784208")
    IBDA_EasMessage : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE get_EasMessage( 
            /* [in] */ ULONG ulEventID,
            /* [out][in] */ __RPC__deref_inout_opt IUnknown **ppEASObject) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBDA_EasMessageVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBDA_EasMessage * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBDA_EasMessage * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBDA_EasMessage * This);
        
        DECLSPEC_XFGVIRT(IBDA_EasMessage, get_EasMessage)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *get_EasMessage )( 
            __RPC__in IBDA_EasMessage * This,
            /* [in] */ ULONG ulEventID,
            /* [out][in] */ __RPC__deref_inout_opt IUnknown **ppEASObject);
        
        END_INTERFACE
    } IBDA_EasMessageVtbl;

    interface IBDA_EasMessage
    {
        CONST_VTBL struct IBDA_EasMessageVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBDA_EasMessage_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBDA_EasMessage_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBDA_EasMessage_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBDA_EasMessage_get_EasMessage(This,ulEventID,ppEASObject)	\
    ( (This)->lpVtbl -> get_EasMessage(This,ulEventID,ppEASObject) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBDA_EasMessage_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_bdaiface_0000_0026 */
/* [local] */ 

#define SID_BDA_EasMessage __uuidof(IBDA_EasMessage)


extern RPC_IF_HANDLE __MIDL_itf_bdaiface_0000_0026_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_bdaiface_0000_0026_v0_0_s_ifspec;

#ifndef __IBDA_TransportStreamInfo_INTERFACE_DEFINED__
#define __IBDA_TransportStreamInfo_INTERFACE_DEFINED__

/* interface IBDA_TransportStreamInfo */
/* [restricted][hidden][unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IBDA_TransportStreamInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8E882535-5F86-47AB-86CF-C281A72A0549")
    IBDA_TransportStreamInfo : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE get_PatTableTickCount( 
            /* [out] */ __RPC__out ULONG *pPatTickCount) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBDA_TransportStreamInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBDA_TransportStreamInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBDA_TransportStreamInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBDA_TransportStreamInfo * This);
        
        DECLSPEC_XFGVIRT(IBDA_TransportStreamInfo, get_PatTableTickCount)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *get_PatTableTickCount )( 
            __RPC__in IBDA_TransportStreamInfo * This,
            /* [out] */ __RPC__out ULONG *pPatTickCount);
        
        END_INTERFACE
    } IBDA_TransportStreamInfoVtbl;

    interface IBDA_TransportStreamInfo
    {
        CONST_VTBL struct IBDA_TransportStreamInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBDA_TransportStreamInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBDA_TransportStreamInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBDA_TransportStreamInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBDA_TransportStreamInfo_get_PatTableTickCount(This,pPatTickCount)	\
    ( (This)->lpVtbl -> get_PatTableTickCount(This,pPatTickCount) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBDA_TransportStreamInfo_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_bdaiface_0000_0027 */
/* [local] */ 

#define SID_BDA_TransportStreamInfo __uuidof(IBDA_TransportStreamInfo)


extern RPC_IF_HANDLE __MIDL_itf_bdaiface_0000_0027_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_bdaiface_0000_0027_v0_0_s_ifspec;

#ifndef __IBDA_ConditionalAccess_INTERFACE_DEFINED__
#define __IBDA_ConditionalAccess_INTERFACE_DEFINED__

/* interface IBDA_ConditionalAccess */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IBDA_ConditionalAccess;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("CD51F1E0-7BE9-4123-8482-A2A796C0A6B0")
    IBDA_ConditionalAccess : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE get_SmartCardStatus( 
            /* [out] */ __RPC__out SmartCardStatusType *pCardStatus,
            /* [out] */ __RPC__out SmartCardAssociationType *pCardAssociation,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrCardError,
            /* [out] */ __RPC__out VARIANT_BOOL *pfOOBLocked) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE get_SmartCardInfo( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrCardName,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrCardManufacturer,
            /* [out] */ __RPC__out VARIANT_BOOL *pfDaylightSavings,
            /* [out] */ __RPC__out BYTE *pbyRatingRegion,
            /* [out] */ __RPC__out LONG *plTimeZoneOffsetMinutes,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrLanguage,
            /* [out] */ __RPC__out EALocationCodeType *pEALocationCode) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE get_SmartCardApplications( 
            /* [out][in] */ __RPC__inout ULONG *pulcApplications,
            /* [in] */ ULONG ulcApplicationsMax,
            /* [size_is][out][in] */ __RPC__inout_ecount_full(ulcApplicationsMax) SmartCardApplication rgApplications[  ]) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE get_Entitlement( 
            /* [in] */ USHORT usVirtualChannel,
            /* [out] */ __RPC__out EntitlementType *pEntitlement) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE TuneByChannel( 
            /* [in] */ USHORT usVirtualChannel) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetProgram( 
            /* [in] */ USHORT usProgramNumber) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE AddProgram( 
            /* [in] */ USHORT usProgramNumber) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE RemoveProgram( 
            /* [in] */ USHORT usProgramNumber) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetModuleUI( 
            /* [in] */ BYTE byDialogNumber,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrURL) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE InformUIClosed( 
            /* [in] */ BYTE byDialogNumber,
            /* [in] */ UICloseReasonType CloseReason) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBDA_ConditionalAccessVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBDA_ConditionalAccess * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBDA_ConditionalAccess * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBDA_ConditionalAccess * This);
        
        DECLSPEC_XFGVIRT(IBDA_ConditionalAccess, get_SmartCardStatus)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *get_SmartCardStatus )( 
            __RPC__in IBDA_ConditionalAccess * This,
            /* [out] */ __RPC__out SmartCardStatusType *pCardStatus,
            /* [out] */ __RPC__out SmartCardAssociationType *pCardAssociation,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrCardError,
            /* [out] */ __RPC__out VARIANT_BOOL *pfOOBLocked);
        
        DECLSPEC_XFGVIRT(IBDA_ConditionalAccess, get_SmartCardInfo)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *get_SmartCardInfo )( 
            __RPC__in IBDA_ConditionalAccess * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrCardName,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrCardManufacturer,
            /* [out] */ __RPC__out VARIANT_BOOL *pfDaylightSavings,
            /* [out] */ __RPC__out BYTE *pbyRatingRegion,
            /* [out] */ __RPC__out LONG *plTimeZoneOffsetMinutes,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrLanguage,
            /* [out] */ __RPC__out EALocationCodeType *pEALocationCode);
        
        DECLSPEC_XFGVIRT(IBDA_ConditionalAccess, get_SmartCardApplications)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *get_SmartCardApplications )( 
            __RPC__in IBDA_ConditionalAccess * This,
            /* [out][in] */ __RPC__inout ULONG *pulcApplications,
            /* [in] */ ULONG ulcApplicationsMax,
            /* [size_is][out][in] */ __RPC__inout_ecount_full(ulcApplicationsMax) SmartCardApplication rgApplications[  ]);
        
        DECLSPEC_XFGVIRT(IBDA_ConditionalAccess, get_Entitlement)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *get_Entitlement )( 
            __RPC__in IBDA_ConditionalAccess * This,
            /* [in] */ USHORT usVirtualChannel,
            /* [out] */ __RPC__out EntitlementType *pEntitlement);
        
        DECLSPEC_XFGVIRT(IBDA_ConditionalAccess, TuneByChannel)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *TuneByChannel )( 
            __RPC__in IBDA_ConditionalAccess * This,
            /* [in] */ USHORT usVirtualChannel);
        
        DECLSPEC_XFGVIRT(IBDA_ConditionalAccess, SetProgram)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetProgram )( 
            __RPC__in IBDA_ConditionalAccess * This,
            /* [in] */ USHORT usProgramNumber);
        
        DECLSPEC_XFGVIRT(IBDA_ConditionalAccess, AddProgram)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AddProgram )( 
            __RPC__in IBDA_ConditionalAccess * This,
            /* [in] */ USHORT usProgramNumber);
        
        DECLSPEC_XFGVIRT(IBDA_ConditionalAccess, RemoveProgram)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *RemoveProgram )( 
            __RPC__in IBDA_ConditionalAccess * This,
            /* [in] */ USHORT usProgramNumber);
        
        DECLSPEC_XFGVIRT(IBDA_ConditionalAccess, GetModuleUI)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetModuleUI )( 
            __RPC__in IBDA_ConditionalAccess * This,
            /* [in] */ BYTE byDialogNumber,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrURL);
        
        DECLSPEC_XFGVIRT(IBDA_ConditionalAccess, InformUIClosed)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *InformUIClosed )( 
            __RPC__in IBDA_ConditionalAccess * This,
            /* [in] */ BYTE byDialogNumber,
            /* [in] */ UICloseReasonType CloseReason);
        
        END_INTERFACE
    } IBDA_ConditionalAccessVtbl;

    interface IBDA_ConditionalAccess
    {
        CONST_VTBL struct IBDA_ConditionalAccessVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBDA_ConditionalAccess_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBDA_ConditionalAccess_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBDA_ConditionalAccess_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBDA_ConditionalAccess_get_SmartCardStatus(This,pCardStatus,pCardAssociation,pbstrCardError,pfOOBLocked)	\
    ( (This)->lpVtbl -> get_SmartCardStatus(This,pCardStatus,pCardAssociation,pbstrCardError,pfOOBLocked) ) 

#define IBDA_ConditionalAccess_get_SmartCardInfo(This,pbstrCardName,pbstrCardManufacturer,pfDaylightSavings,pbyRatingRegion,plTimeZoneOffsetMinutes,pbstrLanguage,pEALocationCode)	\
    ( (This)->lpVtbl -> get_SmartCardInfo(This,pbstrCardName,pbstrCardManufacturer,pfDaylightSavings,pbyRatingRegion,plTimeZoneOffsetMinutes,pbstrLanguage,pEALocationCode) ) 

#define IBDA_ConditionalAccess_get_SmartCardApplications(This,pulcApplications,ulcApplicationsMax,rgApplications)	\
    ( (This)->lpVtbl -> get_SmartCardApplications(This,pulcApplications,ulcApplicationsMax,rgApplications) ) 

#define IBDA_ConditionalAccess_get_Entitlement(This,usVirtualChannel,pEntitlement)	\
    ( (This)->lpVtbl -> get_Entitlement(This,usVirtualChannel,pEntitlement) ) 

#define IBDA_ConditionalAccess_TuneByChannel(This,usVirtualChannel)	\
    ( (This)->lpVtbl -> TuneByChannel(This,usVirtualChannel) ) 

#define IBDA_ConditionalAccess_SetProgram(This,usProgramNumber)	\
    ( (This)->lpVtbl -> SetProgram(This,usProgramNumber) ) 

#define IBDA_ConditionalAccess_AddProgram(This,usProgramNumber)	\
    ( (This)->lpVtbl -> AddProgram(This,usProgramNumber) ) 

#define IBDA_ConditionalAccess_RemoveProgram(This,usProgramNumber)	\
    ( (This)->lpVtbl -> RemoveProgram(This,usProgramNumber) ) 

#define IBDA_ConditionalAccess_GetModuleUI(This,byDialogNumber,pbstrURL)	\
    ( (This)->lpVtbl -> GetModuleUI(This,byDialogNumber,pbstrURL) ) 

#define IBDA_ConditionalAccess_InformUIClosed(This,byDialogNumber,CloseReason)	\
    ( (This)->lpVtbl -> InformUIClosed(This,byDialogNumber,CloseReason) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBDA_ConditionalAccess_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_bdaiface_0000_0028 */
/* [local] */ 

#define SID_BDA_ConditionalAccess __uuidof(IBDA_ConditionalAccess)


extern RPC_IF_HANDLE __MIDL_itf_bdaiface_0000_0028_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_bdaiface_0000_0028_v0_0_s_ifspec;

#ifndef __IBDA_DiagnosticProperties_INTERFACE_DEFINED__
#define __IBDA_DiagnosticProperties_INTERFACE_DEFINED__

/* interface IBDA_DiagnosticProperties */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IBDA_DiagnosticProperties;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("20e80cb5-c543-4c1b-8eb3-49e719eee7d4")
    IBDA_DiagnosticProperties : public IPropertyBag
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct IBDA_DiagnosticPropertiesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBDA_DiagnosticProperties * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBDA_DiagnosticProperties * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBDA_DiagnosticProperties * This);
        
        DECLSPEC_XFGVIRT(IPropertyBag, Read)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Read )( 
            IBDA_DiagnosticProperties * This,
            /* [in] */ LPCOLESTR pszPropName,
            /* [out][in] */ VARIANT *pVar,
            /* [unique][in] */ IErrorLog *pErrorLog);
        
        DECLSPEC_XFGVIRT(IPropertyBag, Write)
        HRESULT ( STDMETHODCALLTYPE *Write )( 
            __RPC__in IBDA_DiagnosticProperties * This,
            /* [in] */ __RPC__in LPCOLESTR pszPropName,
            /* [in] */ __RPC__in VARIANT *pVar);
        
        END_INTERFACE
    } IBDA_DiagnosticPropertiesVtbl;

    interface IBDA_DiagnosticProperties
    {
        CONST_VTBL struct IBDA_DiagnosticPropertiesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBDA_DiagnosticProperties_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBDA_DiagnosticProperties_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBDA_DiagnosticProperties_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBDA_DiagnosticProperties_Read(This,pszPropName,pVar,pErrorLog)	\
    ( (This)->lpVtbl -> Read(This,pszPropName,pVar,pErrorLog) ) 

#define IBDA_DiagnosticProperties_Write(This,pszPropName,pVar)	\
    ( (This)->lpVtbl -> Write(This,pszPropName,pVar) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBDA_DiagnosticProperties_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_bdaiface_0000_0029 */
/* [local] */ 

#define SID_BDA_DiagnosticProperties __uuidof(IBDA_DiagnosticProperties)


extern RPC_IF_HANDLE __MIDL_itf_bdaiface_0000_0029_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_bdaiface_0000_0029_v0_0_s_ifspec;

#ifndef __IBDA_DRM_INTERFACE_DEFINED__
#define __IBDA_DRM_INTERFACE_DEFINED__

/* interface IBDA_DRM */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IBDA_DRM;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F98D88B0-1992-4cd6-A6D9-B9AFAB99330D")
    IBDA_DRM : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetDRMPairingStatus( 
            /* [out] */ __RPC__out DWORD *pdwStatus,
            /* [out] */ __RPC__out HRESULT *phError) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PerformDRMPairing( 
            BOOL fSync) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBDA_DRMVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBDA_DRM * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBDA_DRM * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBDA_DRM * This);
        
        DECLSPEC_XFGVIRT(IBDA_DRM, GetDRMPairingStatus)
        HRESULT ( STDMETHODCALLTYPE *GetDRMPairingStatus )( 
            __RPC__in IBDA_DRM * This,
            /* [out] */ __RPC__out DWORD *pdwStatus,
            /* [out] */ __RPC__out HRESULT *phError);
        
        DECLSPEC_XFGVIRT(IBDA_DRM, PerformDRMPairing)
        HRESULT ( STDMETHODCALLTYPE *PerformDRMPairing )( 
            __RPC__in IBDA_DRM * This,
            BOOL fSync);
        
        END_INTERFACE
    } IBDA_DRMVtbl;

    interface IBDA_DRM
    {
        CONST_VTBL struct IBDA_DRMVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBDA_DRM_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBDA_DRM_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBDA_DRM_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBDA_DRM_GetDRMPairingStatus(This,pdwStatus,phError)	\
    ( (This)->lpVtbl -> GetDRMPairingStatus(This,pdwStatus,phError) ) 

#define IBDA_DRM_PerformDRMPairing(This,fSync)	\
    ( (This)->lpVtbl -> PerformDRMPairing(This,fSync) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBDA_DRM_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_bdaiface_0000_0030 */
/* [local] */ 

#define SID_BDA_DRM __uuidof(IBDA_DRM)


extern RPC_IF_HANDLE __MIDL_itf_bdaiface_0000_0030_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_bdaiface_0000_0030_v0_0_s_ifspec;

#ifndef __IBDA_NameValueService_INTERFACE_DEFINED__
#define __IBDA_NameValueService_INTERFACE_DEFINED__

/* interface IBDA_NameValueService */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IBDA_NameValueService;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7F0B3150-7B81-4ad4-98E3-7E9097094301")
    IBDA_NameValueService : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetValueNameByIndex( 
            /* [in] */ ULONG ulIndex,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrName) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetValue( 
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ __RPC__in BSTR bstrLanguage,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrValue) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetValue( 
            /* [in] */ ULONG ulDialogRequest,
            /* [in] */ __RPC__in BSTR bstrLanguage,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ __RPC__in BSTR bstrValue,
            /* [in] */ ULONG ulReserved) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBDA_NameValueServiceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBDA_NameValueService * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBDA_NameValueService * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBDA_NameValueService * This);
        
        DECLSPEC_XFGVIRT(IBDA_NameValueService, GetValueNameByIndex)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetValueNameByIndex )( 
            __RPC__in IBDA_NameValueService * This,
            /* [in] */ ULONG ulIndex,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IBDA_NameValueService, GetValue)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetValue )( 
            __RPC__in IBDA_NameValueService * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ __RPC__in BSTR bstrLanguage,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrValue);
        
        DECLSPEC_XFGVIRT(IBDA_NameValueService, SetValue)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetValue )( 
            __RPC__in IBDA_NameValueService * This,
            /* [in] */ ULONG ulDialogRequest,
            /* [in] */ __RPC__in BSTR bstrLanguage,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ __RPC__in BSTR bstrValue,
            /* [in] */ ULONG ulReserved);
        
        END_INTERFACE
    } IBDA_NameValueServiceVtbl;

    interface IBDA_NameValueService
    {
        CONST_VTBL struct IBDA_NameValueServiceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBDA_NameValueService_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBDA_NameValueService_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBDA_NameValueService_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBDA_NameValueService_GetValueNameByIndex(This,ulIndex,pbstrName)	\
    ( (This)->lpVtbl -> GetValueNameByIndex(This,ulIndex,pbstrName) ) 

#define IBDA_NameValueService_GetValue(This,bstrName,bstrLanguage,pbstrValue)	\
    ( (This)->lpVtbl -> GetValue(This,bstrName,bstrLanguage,pbstrValue) ) 

#define IBDA_NameValueService_SetValue(This,ulDialogRequest,bstrLanguage,bstrName,bstrValue,ulReserved)	\
    ( (This)->lpVtbl -> SetValue(This,ulDialogRequest,bstrLanguage,bstrName,bstrValue,ulReserved) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBDA_NameValueService_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_bdaiface_0000_0031 */
/* [local] */ 

#define SID_BDA_NameValueService __uuidof(IBDA_NameValueService)


extern RPC_IF_HANDLE __MIDL_itf_bdaiface_0000_0031_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_bdaiface_0000_0031_v0_0_s_ifspec;

#ifndef __IBDA_ConditionalAccessEx_INTERFACE_DEFINED__
#define __IBDA_ConditionalAccessEx_INTERFACE_DEFINED__

/* interface IBDA_ConditionalAccessEx */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IBDA_ConditionalAccessEx;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("497C3418-23CB-44ba-BB62-769F506FCEA7")
    IBDA_ConditionalAccessEx : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE CheckEntitlementToken( 
            /* [in] */ ULONG ulDialogRequest,
            /* [in] */ __RPC__in BSTR bstrLanguage,
            /* [in] */ BDA_CONDITIONALACCESS_REQUESTTYPE RequestType,
            /* [in] */ ULONG ulcbEntitlementTokenLen,
            /* [size_is][in] */ __RPC__in_ecount_full(ulcbEntitlementTokenLen) BYTE *pbEntitlementToken,
            /* [out] */ __RPC__out ULONG *pulDescrambleStatus) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetCaptureToken( 
            /* [in] */ ULONG ulcbCaptureTokenLen,
            /* [size_is][in] */ __RPC__in_ecount_full(ulcbCaptureTokenLen) BYTE *pbCaptureToken) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE OpenBroadcastMmi( 
            /* [in] */ ULONG ulDialogRequest,
            /* [in] */ __RPC__in BSTR bstrLanguage,
            /* [in] */ ULONG EventId) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE CloseMmiDialog( 
            /* [in] */ ULONG ulDialogRequest,
            /* [in] */ __RPC__in BSTR bstrLanguage,
            /* [in] */ ULONG ulDialogNumber,
            /* [in] */ BDA_CONDITIONALACCESS_MMICLOSEREASON ReasonCode,
            /* [out] */ __RPC__out ULONG *pulSessionResult) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE CreateDialogRequestNumber( 
            /* [out] */ __RPC__out ULONG *pulDialogRequestNumber) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBDA_ConditionalAccessExVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBDA_ConditionalAccessEx * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBDA_ConditionalAccessEx * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBDA_ConditionalAccessEx * This);
        
        DECLSPEC_XFGVIRT(IBDA_ConditionalAccessEx, CheckEntitlementToken)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CheckEntitlementToken )( 
            __RPC__in IBDA_ConditionalAccessEx * This,
            /* [in] */ ULONG ulDialogRequest,
            /* [in] */ __RPC__in BSTR bstrLanguage,
            /* [in] */ BDA_CONDITIONALACCESS_REQUESTTYPE RequestType,
            /* [in] */ ULONG ulcbEntitlementTokenLen,
            /* [size_is][in] */ __RPC__in_ecount_full(ulcbEntitlementTokenLen) BYTE *pbEntitlementToken,
            /* [out] */ __RPC__out ULONG *pulDescrambleStatus);
        
        DECLSPEC_XFGVIRT(IBDA_ConditionalAccessEx, SetCaptureToken)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetCaptureToken )( 
            __RPC__in IBDA_ConditionalAccessEx * This,
            /* [in] */ ULONG ulcbCaptureTokenLen,
            /* [size_is][in] */ __RPC__in_ecount_full(ulcbCaptureTokenLen) BYTE *pbCaptureToken);
        
        DECLSPEC_XFGVIRT(IBDA_ConditionalAccessEx, OpenBroadcastMmi)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *OpenBroadcastMmi )( 
            __RPC__in IBDA_ConditionalAccessEx * This,
            /* [in] */ ULONG ulDialogRequest,
            /* [in] */ __RPC__in BSTR bstrLanguage,
            /* [in] */ ULONG EventId);
        
        DECLSPEC_XFGVIRT(IBDA_ConditionalAccessEx, CloseMmiDialog)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CloseMmiDialog )( 
            __RPC__in IBDA_ConditionalAccessEx * This,
            /* [in] */ ULONG ulDialogRequest,
            /* [in] */ __RPC__in BSTR bstrLanguage,
            /* [in] */ ULONG ulDialogNumber,
            /* [in] */ BDA_CONDITIONALACCESS_MMICLOSEREASON ReasonCode,
            /* [out] */ __RPC__out ULONG *pulSessionResult);
        
        DECLSPEC_XFGVIRT(IBDA_ConditionalAccessEx, CreateDialogRequestNumber)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CreateDialogRequestNumber )( 
            __RPC__in IBDA_ConditionalAccessEx * This,
            /* [out] */ __RPC__out ULONG *pulDialogRequestNumber);
        
        END_INTERFACE
    } IBDA_ConditionalAccessExVtbl;

    interface IBDA_ConditionalAccessEx
    {
        CONST_VTBL struct IBDA_ConditionalAccessExVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBDA_ConditionalAccessEx_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBDA_ConditionalAccessEx_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBDA_ConditionalAccessEx_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBDA_ConditionalAccessEx_CheckEntitlementToken(This,ulDialogRequest,bstrLanguage,RequestType,ulcbEntitlementTokenLen,pbEntitlementToken,pulDescrambleStatus)	\
    ( (This)->lpVtbl -> CheckEntitlementToken(This,ulDialogRequest,bstrLanguage,RequestType,ulcbEntitlementTokenLen,pbEntitlementToken,pulDescrambleStatus) ) 

#define IBDA_ConditionalAccessEx_SetCaptureToken(This,ulcbCaptureTokenLen,pbCaptureToken)	\
    ( (This)->lpVtbl -> SetCaptureToken(This,ulcbCaptureTokenLen,pbCaptureToken) ) 

#define IBDA_ConditionalAccessEx_OpenBroadcastMmi(This,ulDialogRequest,bstrLanguage,EventId)	\
    ( (This)->lpVtbl -> OpenBroadcastMmi(This,ulDialogRequest,bstrLanguage,EventId) ) 

#define IBDA_ConditionalAccessEx_CloseMmiDialog(This,ulDialogRequest,bstrLanguage,ulDialogNumber,ReasonCode,pulSessionResult)	\
    ( (This)->lpVtbl -> CloseMmiDialog(This,ulDialogRequest,bstrLanguage,ulDialogNumber,ReasonCode,pulSessionResult) ) 

#define IBDA_ConditionalAccessEx_CreateDialogRequestNumber(This,pulDialogRequestNumber)	\
    ( (This)->lpVtbl -> CreateDialogRequestNumber(This,pulDialogRequestNumber) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBDA_ConditionalAccessEx_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_bdaiface_0000_0032 */
/* [local] */ 

#define SID_BDA_ConditionalAccessEx __uuidof(IBDA_ConditionalAccessEx)


extern RPC_IF_HANDLE __MIDL_itf_bdaiface_0000_0032_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_bdaiface_0000_0032_v0_0_s_ifspec;

#ifndef __IBDA_ISDBConditionalAccess_INTERFACE_DEFINED__
#define __IBDA_ISDBConditionalAccess_INTERFACE_DEFINED__

/* interface IBDA_ISDBConditionalAccess */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IBDA_ISDBConditionalAccess;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5E68C627-16C2-4e6c-B1E2-D00170CDAA0F")
    IBDA_ISDBConditionalAccess : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetIsdbCasRequest( 
            /* [in] */ ULONG ulRequestId,
            /* [in] */ ULONG ulcbRequestBufferLen,
            /* [size_is][in] */ __RPC__in_ecount_full(ulcbRequestBufferLen) BYTE *pbRequestBuffer) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBDA_ISDBConditionalAccessVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBDA_ISDBConditionalAccess * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBDA_ISDBConditionalAccess * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBDA_ISDBConditionalAccess * This);
        
        DECLSPEC_XFGVIRT(IBDA_ISDBConditionalAccess, SetIsdbCasRequest)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetIsdbCasRequest )( 
            __RPC__in IBDA_ISDBConditionalAccess * This,
            /* [in] */ ULONG ulRequestId,
            /* [in] */ ULONG ulcbRequestBufferLen,
            /* [size_is][in] */ __RPC__in_ecount_full(ulcbRequestBufferLen) BYTE *pbRequestBuffer);
        
        END_INTERFACE
    } IBDA_ISDBConditionalAccessVtbl;

    interface IBDA_ISDBConditionalAccess
    {
        CONST_VTBL struct IBDA_ISDBConditionalAccessVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBDA_ISDBConditionalAccess_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBDA_ISDBConditionalAccess_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBDA_ISDBConditionalAccess_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBDA_ISDBConditionalAccess_SetIsdbCasRequest(This,ulRequestId,ulcbRequestBufferLen,pbRequestBuffer)	\
    ( (This)->lpVtbl -> SetIsdbCasRequest(This,ulRequestId,ulcbRequestBufferLen,pbRequestBuffer) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBDA_ISDBConditionalAccess_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_bdaiface_0000_0033 */
/* [local] */ 

#define SID_BDA_ISDBConditionalAccess __uuidof(IBDA_ISDBConditionalAccess)


extern RPC_IF_HANDLE __MIDL_itf_bdaiface_0000_0033_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_bdaiface_0000_0033_v0_0_s_ifspec;

#ifndef __IBDA_EventingService_INTERFACE_DEFINED__
#define __IBDA_EventingService_INTERFACE_DEFINED__

/* interface IBDA_EventingService */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IBDA_EventingService;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("207C413F-00DC-4c61-BAD6-6FEE1FF07064")
    IBDA_EventingService : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE CompleteEvent( 
            /* [in] */ ULONG ulEventID,
            /* [in] */ ULONG ulEventResult) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBDA_EventingServiceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBDA_EventingService * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBDA_EventingService * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBDA_EventingService * This);
        
        DECLSPEC_XFGVIRT(IBDA_EventingService, CompleteEvent)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CompleteEvent )( 
            __RPC__in IBDA_EventingService * This,
            /* [in] */ ULONG ulEventID,
            /* [in] */ ULONG ulEventResult);
        
        END_INTERFACE
    } IBDA_EventingServiceVtbl;

    interface IBDA_EventingService
    {
        CONST_VTBL struct IBDA_EventingServiceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBDA_EventingService_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBDA_EventingService_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBDA_EventingService_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBDA_EventingService_CompleteEvent(This,ulEventID,ulEventResult)	\
    ( (This)->lpVtbl -> CompleteEvent(This,ulEventID,ulEventResult) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBDA_EventingService_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_bdaiface_0000_0034 */
/* [local] */ 

#define SID_BDA_EventingService __uuidof(IBDA_EventingService)


extern RPC_IF_HANDLE __MIDL_itf_bdaiface_0000_0034_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_bdaiface_0000_0034_v0_0_s_ifspec;

#ifndef __IBDA_AUX_INTERFACE_DEFINED__
#define __IBDA_AUX_INTERFACE_DEFINED__

/* interface IBDA_AUX */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IBDA_AUX;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7DEf4C09-6E66-4567-A819-F0E17F4A81AB")
    IBDA_AUX : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE QueryCapabilities( 
            /* [out] */ __RPC__out DWORD *pdwNumAuxInputsBSTR) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE EnumCapability( 
            /* [in] */ DWORD dwIndex,
            /* [out] */ __RPC__out DWORD *dwInputID,
            /* [out] */ __RPC__out GUID *pConnectorType,
            /* [out] */ __RPC__out DWORD *ConnTypeNum,
            /* [out] */ __RPC__out DWORD *NumVideoStds,
            /* [out] */ __RPC__out ULONGLONG *AnalogStds) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBDA_AUXVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBDA_AUX * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBDA_AUX * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBDA_AUX * This);
        
        DECLSPEC_XFGVIRT(IBDA_AUX, QueryCapabilities)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *QueryCapabilities )( 
            __RPC__in IBDA_AUX * This,
            /* [out] */ __RPC__out DWORD *pdwNumAuxInputsBSTR);
        
        DECLSPEC_XFGVIRT(IBDA_AUX, EnumCapability)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *EnumCapability )( 
            __RPC__in IBDA_AUX * This,
            /* [in] */ DWORD dwIndex,
            /* [out] */ __RPC__out DWORD *dwInputID,
            /* [out] */ __RPC__out GUID *pConnectorType,
            /* [out] */ __RPC__out DWORD *ConnTypeNum,
            /* [out] */ __RPC__out DWORD *NumVideoStds,
            /* [out] */ __RPC__out ULONGLONG *AnalogStds);
        
        END_INTERFACE
    } IBDA_AUXVtbl;

    interface IBDA_AUX
    {
        CONST_VTBL struct IBDA_AUXVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBDA_AUX_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBDA_AUX_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBDA_AUX_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBDA_AUX_QueryCapabilities(This,pdwNumAuxInputsBSTR)	\
    ( (This)->lpVtbl -> QueryCapabilities(This,pdwNumAuxInputsBSTR) ) 

#define IBDA_AUX_EnumCapability(This,dwIndex,dwInputID,pConnectorType,ConnTypeNum,NumVideoStds,AnalogStds)	\
    ( (This)->lpVtbl -> EnumCapability(This,dwIndex,dwInputID,pConnectorType,ConnTypeNum,NumVideoStds,AnalogStds) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBDA_AUX_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_bdaiface_0000_0035 */
/* [local] */ 

DEFINE_GUID(PBDA_AUX_CONNECTOR_TYPE_SVideo, 0xa0e905f4,0x24c9,0x4a54, 0xb7, 0x61, 0x21, 0x33, 0x55, 0xef, 0xc1, 0x3A);
DEFINE_GUID(PBDA_AUX_CONNECTOR_TYPE_Composite, 0xf6298b4c,0xc725,0x4d42, 0x84, 0x9b, 0x41, 0x0b, 0xbb, 0x14, 0xea, 0x62);
#define SID_BDA_AUX __uuidof(IBDA_AUX)
DEFINE_GUID(CLSID_PBDA_AUX_DATA_TYPE, /* fd456373-3323-4090-adca-8ed45f55cf10 */
0xfd456373, 0x3323, 0x4090, 0xad, 0xca, 0x8e, 0xd4, 0x5f, 0x55, 0xcf, 0x10);


extern RPC_IF_HANDLE __MIDL_itf_bdaiface_0000_0035_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_bdaiface_0000_0035_v0_0_s_ifspec;

#ifndef __IBDA_Encoder_INTERFACE_DEFINED__
#define __IBDA_Encoder_INTERFACE_DEFINED__

/* interface IBDA_Encoder */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IBDA_Encoder;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3a8bad59-59fe-4559-a0ba-396cfaa98ae3")
    IBDA_Encoder : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE QueryCapabilities( 
            /* [out] */ __RPC__out DWORD *NumAudioFmts,
            /* [out] */ __RPC__out DWORD *NumVideoFmts) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE EnumAudioCapability( 
            /* [in] */ DWORD FmtIndex,
            /* [out] */ __RPC__out DWORD *MethodID,
            /* [out] */ __RPC__out DWORD *AlgorithmType,
            /* [out] */ __RPC__out DWORD *SamplingRate,
            /* [out] */ __RPC__out DWORD *BitDepth,
            /* [out] */ __RPC__out DWORD *NumChannels) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE EnumVideoCapability( 
            /* [in] */ DWORD FmtIndex,
            /* [out] */ __RPC__out DWORD *MethodID,
            /* [out] */ __RPC__out DWORD *AlgorithmType,
            /* [out] */ __RPC__out DWORD *VerticalSize,
            /* [out] */ __RPC__out DWORD *HorizontalSize,
            /* [out] */ __RPC__out DWORD *AspectRatio,
            /* [out] */ __RPC__out DWORD *FrameRateCode,
            /* [out] */ __RPC__out DWORD *ProgressiveSequence) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetParameters( 
            /* [in] */ DWORD AudioBitrateMode,
            /* [in] */ DWORD AudioBitrate,
            /* [in] */ DWORD AudioMethodID,
            /* [in] */ DWORD AudioProgram,
            /* [in] */ DWORD VideoBitrateMode,
            /* [in] */ DWORD VideoBitrate,
            /* [in] */ DWORD VideoMethodID) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetState( 
            /* [out] */ __RPC__out DWORD *AudioBitrateMax,
            /* [out] */ __RPC__out DWORD *AudioBitrateMin,
            /* [out] */ __RPC__out DWORD *AudioBitrateMode,
            /* [out] */ __RPC__out DWORD *AudioBitrateStepping,
            /* [out] */ __RPC__out DWORD *AudioBitrate,
            /* [out] */ __RPC__out DWORD *AudioMethodID,
            /* [out] */ __RPC__out DWORD *AvailableAudioPrograms,
            /* [out] */ __RPC__out DWORD *AudioProgram,
            /* [out] */ __RPC__out DWORD *VideoBitrateMax,
            /* [out] */ __RPC__out DWORD *VideoBitrateMin,
            /* [out] */ __RPC__out DWORD *VideoBitrateMode,
            /* [out] */ __RPC__out DWORD *VideoBitrate,
            /* [out] */ __RPC__out DWORD *VideoBitrateStepping,
            /* [out] */ __RPC__out DWORD *VideoMethodID,
            /* [out] */ __RPC__out DWORD *SignalSourceID,
            /* [out] */ __RPC__out ULONGLONG *SignalFormat,
            /* [out] */ __RPC__out BOOL *SignalLock,
            /* [out] */ __RPC__out LONG *SignalLevel,
            /* [out] */ __RPC__out DWORD *SignalToNoiseRatio) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBDA_EncoderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBDA_Encoder * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBDA_Encoder * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBDA_Encoder * This);
        
        DECLSPEC_XFGVIRT(IBDA_Encoder, QueryCapabilities)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *QueryCapabilities )( 
            __RPC__in IBDA_Encoder * This,
            /* [out] */ __RPC__out DWORD *NumAudioFmts,
            /* [out] */ __RPC__out DWORD *NumVideoFmts);
        
        DECLSPEC_XFGVIRT(IBDA_Encoder, EnumAudioCapability)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *EnumAudioCapability )( 
            __RPC__in IBDA_Encoder * This,
            /* [in] */ DWORD FmtIndex,
            /* [out] */ __RPC__out DWORD *MethodID,
            /* [out] */ __RPC__out DWORD *AlgorithmType,
            /* [out] */ __RPC__out DWORD *SamplingRate,
            /* [out] */ __RPC__out DWORD *BitDepth,
            /* [out] */ __RPC__out DWORD *NumChannels);
        
        DECLSPEC_XFGVIRT(IBDA_Encoder, EnumVideoCapability)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *EnumVideoCapability )( 
            __RPC__in IBDA_Encoder * This,
            /* [in] */ DWORD FmtIndex,
            /* [out] */ __RPC__out DWORD *MethodID,
            /* [out] */ __RPC__out DWORD *AlgorithmType,
            /* [out] */ __RPC__out DWORD *VerticalSize,
            /* [out] */ __RPC__out DWORD *HorizontalSize,
            /* [out] */ __RPC__out DWORD *AspectRatio,
            /* [out] */ __RPC__out DWORD *FrameRateCode,
            /* [out] */ __RPC__out DWORD *ProgressiveSequence);
        
        DECLSPEC_XFGVIRT(IBDA_Encoder, SetParameters)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetParameters )( 
            __RPC__in IBDA_Encoder * This,
            /* [in] */ DWORD AudioBitrateMode,
            /* [in] */ DWORD AudioBitrate,
            /* [in] */ DWORD AudioMethodID,
            /* [in] */ DWORD AudioProgram,
            /* [in] */ DWORD VideoBitrateMode,
            /* [in] */ DWORD VideoBitrate,
            /* [in] */ DWORD VideoMethodID);
        
        DECLSPEC_XFGVIRT(IBDA_Encoder, GetState)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetState )( 
            __RPC__in IBDA_Encoder * This,
            /* [out] */ __RPC__out DWORD *AudioBitrateMax,
            /* [out] */ __RPC__out DWORD *AudioBitrateMin,
            /* [out] */ __RPC__out DWORD *AudioBitrateMode,
            /* [out] */ __RPC__out DWORD *AudioBitrateStepping,
            /* [out] */ __RPC__out DWORD *AudioBitrate,
            /* [out] */ __RPC__out DWORD *AudioMethodID,
            /* [out] */ __RPC__out DWORD *AvailableAudioPrograms,
            /* [out] */ __RPC__out DWORD *AudioProgram,
            /* [out] */ __RPC__out DWORD *VideoBitrateMax,
            /* [out] */ __RPC__out DWORD *VideoBitrateMin,
            /* [out] */ __RPC__out DWORD *VideoBitrateMode,
            /* [out] */ __RPC__out DWORD *VideoBitrate,
            /* [out] */ __RPC__out DWORD *VideoBitrateStepping,
            /* [out] */ __RPC__out DWORD *VideoMethodID,
            /* [out] */ __RPC__out DWORD *SignalSourceID,
            /* [out] */ __RPC__out ULONGLONG *SignalFormat,
            /* [out] */ __RPC__out BOOL *SignalLock,
            /* [out] */ __RPC__out LONG *SignalLevel,
            /* [out] */ __RPC__out DWORD *SignalToNoiseRatio);
        
        END_INTERFACE
    } IBDA_EncoderVtbl;

    interface IBDA_Encoder
    {
        CONST_VTBL struct IBDA_EncoderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBDA_Encoder_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBDA_Encoder_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBDA_Encoder_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBDA_Encoder_QueryCapabilities(This,NumAudioFmts,NumVideoFmts)	\
    ( (This)->lpVtbl -> QueryCapabilities(This,NumAudioFmts,NumVideoFmts) ) 

#define IBDA_Encoder_EnumAudioCapability(This,FmtIndex,MethodID,AlgorithmType,SamplingRate,BitDepth,NumChannels)	\
    ( (This)->lpVtbl -> EnumAudioCapability(This,FmtIndex,MethodID,AlgorithmType,SamplingRate,BitDepth,NumChannels) ) 

#define IBDA_Encoder_EnumVideoCapability(This,FmtIndex,MethodID,AlgorithmType,VerticalSize,HorizontalSize,AspectRatio,FrameRateCode,ProgressiveSequence)	\
    ( (This)->lpVtbl -> EnumVideoCapability(This,FmtIndex,MethodID,AlgorithmType,VerticalSize,HorizontalSize,AspectRatio,FrameRateCode,ProgressiveSequence) ) 

#define IBDA_Encoder_SetParameters(This,AudioBitrateMode,AudioBitrate,AudioMethodID,AudioProgram,VideoBitrateMode,VideoBitrate,VideoMethodID)	\
    ( (This)->lpVtbl -> SetParameters(This,AudioBitrateMode,AudioBitrate,AudioMethodID,AudioProgram,VideoBitrateMode,VideoBitrate,VideoMethodID) ) 

#define IBDA_Encoder_GetState(This,AudioBitrateMax,AudioBitrateMin,AudioBitrateMode,AudioBitrateStepping,AudioBitrate,AudioMethodID,AvailableAudioPrograms,AudioProgram,VideoBitrateMax,VideoBitrateMin,VideoBitrateMode,VideoBitrate,VideoBitrateStepping,VideoMethodID,SignalSourceID,SignalFormat,SignalLock,SignalLevel,SignalToNoiseRatio)	\
    ( (This)->lpVtbl -> GetState(This,AudioBitrateMax,AudioBitrateMin,AudioBitrateMode,AudioBitrateStepping,AudioBitrate,AudioMethodID,AvailableAudioPrograms,AudioProgram,VideoBitrateMax,VideoBitrateMin,VideoBitrateMode,VideoBitrate,VideoBitrateStepping,VideoMethodID,SignalSourceID,SignalFormat,SignalLock,SignalLevel,SignalToNoiseRatio) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBDA_Encoder_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_bdaiface_0000_0036 */
/* [local] */ 

#define SID_BDA_Encoder __uuidof(IBDA_Encoder)
DEFINE_GUID(CLSID_PBDA_Encoder_DATA_TYPE, /* 728fd6bc-5546-4716-b103-f899f5a1fa68 */
0x728fd6bc, 0x5546, 0x4716, 0xb1, 0x03, 0xf8, 0x99, 0xf5, 0xa1, 0xfa, 0x68);
#define PBDA_Encoder_Audio_AlgorithmType_MPEG1LayerII    0x00000000
#define PBDA_Encoder_Audio_AlgorithmType_AC3             0x00000001
#define PBDA_Encoder_Video_MPEG2PartII                   0x00000000
#define PBDA_Encoder_Video_MPEG4Part10                   0x00000001
#define PBDA_Encoder_Video_AVC                           0x00000001
#define PBDA_Encoder_Video_H264                          0x00000001
#define PBDA_Encoder_BitrateMode_Constant                0x00000001
#define PBDA_Encoder_BitrateMode_Variable                0x00000002
#define PBDA_Encoder_BitrateMode_Average                 0x00000003


extern RPC_IF_HANDLE __MIDL_itf_bdaiface_0000_0036_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_bdaiface_0000_0036_v0_0_s_ifspec;

#ifndef __IBDA_FDC_INTERFACE_DEFINED__
#define __IBDA_FDC_INTERFACE_DEFINED__

/* interface IBDA_FDC */
/* [unique][helpstring][uuid][object] */ 

#define SID_BDA_FDC __uuidof(IBDA_FDC
DEFINE_GUID(CLSID_PBDA_FDC_DATA_TYPE, /* E7DBF9A0-22AB-4047-8E67-EF9AD504E729 */
0xe7dbf9a0, 0x22ab, 0x4047, 0x8e, 0x67, 0xef, 0x9a, 0xd5, 0x4, 0xe7, 0x29);

EXTERN_C const IID IID_IBDA_FDC;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("138adc7e-58ae-437f-b0b4-c9fe19d5b4ac")
    IBDA_FDC : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetStatus( 
            /* [out] */ __RPC__out DWORD *CurrentBitrate,
            /* [out] */ __RPC__out BOOL *CarrierLock,
            /* [out] */ __RPC__out DWORD *CurrentFrequency,
            /* [out] */ __RPC__out BOOL *CurrentSpectrumInversion,
            /* [out] */ __RPC__deref_out_opt BSTR *CurrentPIDList,
            /* [out] */ __RPC__deref_out_opt BSTR *CurrentTIDList,
            /* [out] */ __RPC__out BOOL *Overflow) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE RequestTables( 
            /* [in] */ __RPC__in BSTR TableIDs) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE AddPid( 
            /* [in] */ __RPC__in BSTR PidsToAdd,
            /* [out] */ __RPC__out DWORD *RemainingFilterEntries) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE RemovePid( 
            /* [in] */ __RPC__in BSTR PidsToRemove) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE AddTid( 
            /* [in] */ __RPC__in BSTR TidsToAdd,
            /* [out] */ __RPC__deref_out_opt BSTR *CurrentTidList) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE RemoveTid( 
            /* [in] */ __RPC__in BSTR TidsToRemove) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetTableSection( 
            /* [out] */ __RPC__out DWORD *Pid,
            /* [in] */ DWORD MaxBufferSize,
            /* [out] */ __RPC__out DWORD *ActualSize,
            /* [out] */ __RPC__out BYTE *SecBuffer) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBDA_FDCVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBDA_FDC * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBDA_FDC * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBDA_FDC * This);
        
        DECLSPEC_XFGVIRT(IBDA_FDC, GetStatus)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetStatus )( 
            __RPC__in IBDA_FDC * This,
            /* [out] */ __RPC__out DWORD *CurrentBitrate,
            /* [out] */ __RPC__out BOOL *CarrierLock,
            /* [out] */ __RPC__out DWORD *CurrentFrequency,
            /* [out] */ __RPC__out BOOL *CurrentSpectrumInversion,
            /* [out] */ __RPC__deref_out_opt BSTR *CurrentPIDList,
            /* [out] */ __RPC__deref_out_opt BSTR *CurrentTIDList,
            /* [out] */ __RPC__out BOOL *Overflow);
        
        DECLSPEC_XFGVIRT(IBDA_FDC, RequestTables)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *RequestTables )( 
            __RPC__in IBDA_FDC * This,
            /* [in] */ __RPC__in BSTR TableIDs);
        
        DECLSPEC_XFGVIRT(IBDA_FDC, AddPid)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AddPid )( 
            __RPC__in IBDA_FDC * This,
            /* [in] */ __RPC__in BSTR PidsToAdd,
            /* [out] */ __RPC__out DWORD *RemainingFilterEntries);
        
        DECLSPEC_XFGVIRT(IBDA_FDC, RemovePid)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *RemovePid )( 
            __RPC__in IBDA_FDC * This,
            /* [in] */ __RPC__in BSTR PidsToRemove);
        
        DECLSPEC_XFGVIRT(IBDA_FDC, AddTid)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AddTid )( 
            __RPC__in IBDA_FDC * This,
            /* [in] */ __RPC__in BSTR TidsToAdd,
            /* [out] */ __RPC__deref_out_opt BSTR *CurrentTidList);
        
        DECLSPEC_XFGVIRT(IBDA_FDC, RemoveTid)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *RemoveTid )( 
            __RPC__in IBDA_FDC * This,
            /* [in] */ __RPC__in BSTR TidsToRemove);
        
        DECLSPEC_XFGVIRT(IBDA_FDC, GetTableSection)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetTableSection )( 
            __RPC__in IBDA_FDC * This,
            /* [out] */ __RPC__out DWORD *Pid,
            /* [in] */ DWORD MaxBufferSize,
            /* [out] */ __RPC__out DWORD *ActualSize,
            /* [out] */ __RPC__out BYTE *SecBuffer);
        
        END_INTERFACE
    } IBDA_FDCVtbl;

    interface IBDA_FDC
    {
        CONST_VTBL struct IBDA_FDCVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBDA_FDC_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBDA_FDC_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBDA_FDC_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBDA_FDC_GetStatus(This,CurrentBitrate,CarrierLock,CurrentFrequency,CurrentSpectrumInversion,CurrentPIDList,CurrentTIDList,Overflow)	\
    ( (This)->lpVtbl -> GetStatus(This,CurrentBitrate,CarrierLock,CurrentFrequency,CurrentSpectrumInversion,CurrentPIDList,CurrentTIDList,Overflow) ) 

#define IBDA_FDC_RequestTables(This,TableIDs)	\
    ( (This)->lpVtbl -> RequestTables(This,TableIDs) ) 

#define IBDA_FDC_AddPid(This,PidsToAdd,RemainingFilterEntries)	\
    ( (This)->lpVtbl -> AddPid(This,PidsToAdd,RemainingFilterEntries) ) 

#define IBDA_FDC_RemovePid(This,PidsToRemove)	\
    ( (This)->lpVtbl -> RemovePid(This,PidsToRemove) ) 

#define IBDA_FDC_AddTid(This,TidsToAdd,CurrentTidList)	\
    ( (This)->lpVtbl -> AddTid(This,TidsToAdd,CurrentTidList) ) 

#define IBDA_FDC_RemoveTid(This,TidsToRemove)	\
    ( (This)->lpVtbl -> RemoveTid(This,TidsToRemove) ) 

#define IBDA_FDC_GetTableSection(This,Pid,MaxBufferSize,ActualSize,SecBuffer)	\
    ( (This)->lpVtbl -> GetTableSection(This,Pid,MaxBufferSize,ActualSize,SecBuffer) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBDA_FDC_INTERFACE_DEFINED__ */


#ifndef __IBDA_GuideDataDeliveryService_INTERFACE_DEFINED__
#define __IBDA_GuideDataDeliveryService_INTERFACE_DEFINED__

/* interface IBDA_GuideDataDeliveryService */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IBDA_GuideDataDeliveryService;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C0AFCB73-23E7-4bc6-BAFA-FDC167B4719F")
    IBDA_GuideDataDeliveryService : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetGuideDataType( 
            /* [out] */ __RPC__out GUID *pguidDataType) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetGuideData( 
            /* [out][in] */ __RPC__inout ULONG *pulcbBufferLen,
            /* [out] */ __RPC__out BYTE *pbBuffer,
            /* [out] */ __RPC__out ULONG *pulGuideDataPercentageProgress) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE RequestGuideDataUpdate( void) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetTuneXmlFromServiceIdx( 
            /* [in] */ ULONG64 ul64ServiceIdx,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrTuneXml) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetServices( 
            /* [out][in] */ __RPC__inout ULONG *pulcbBufferLen,
            /* [out] */ __RPC__out BYTE *pbBuffer) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetServiceInfoFromTuneXml( 
            /* [in] */ __RPC__in BSTR bstrTuneXml,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrServiceDescription) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBDA_GuideDataDeliveryServiceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBDA_GuideDataDeliveryService * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBDA_GuideDataDeliveryService * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBDA_GuideDataDeliveryService * This);
        
        DECLSPEC_XFGVIRT(IBDA_GuideDataDeliveryService, GetGuideDataType)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetGuideDataType )( 
            __RPC__in IBDA_GuideDataDeliveryService * This,
            /* [out] */ __RPC__out GUID *pguidDataType);
        
        DECLSPEC_XFGVIRT(IBDA_GuideDataDeliveryService, GetGuideData)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetGuideData )( 
            __RPC__in IBDA_GuideDataDeliveryService * This,
            /* [out][in] */ __RPC__inout ULONG *pulcbBufferLen,
            /* [out] */ __RPC__out BYTE *pbBuffer,
            /* [out] */ __RPC__out ULONG *pulGuideDataPercentageProgress);
        
        DECLSPEC_XFGVIRT(IBDA_GuideDataDeliveryService, RequestGuideDataUpdate)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *RequestGuideDataUpdate )( 
            __RPC__in IBDA_GuideDataDeliveryService * This);
        
        DECLSPEC_XFGVIRT(IBDA_GuideDataDeliveryService, GetTuneXmlFromServiceIdx)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetTuneXmlFromServiceIdx )( 
            __RPC__in IBDA_GuideDataDeliveryService * This,
            /* [in] */ ULONG64 ul64ServiceIdx,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrTuneXml);
        
        DECLSPEC_XFGVIRT(IBDA_GuideDataDeliveryService, GetServices)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetServices )( 
            __RPC__in IBDA_GuideDataDeliveryService * This,
            /* [out][in] */ __RPC__inout ULONG *pulcbBufferLen,
            /* [out] */ __RPC__out BYTE *pbBuffer);
        
        DECLSPEC_XFGVIRT(IBDA_GuideDataDeliveryService, GetServiceInfoFromTuneXml)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetServiceInfoFromTuneXml )( 
            __RPC__in IBDA_GuideDataDeliveryService * This,
            /* [in] */ __RPC__in BSTR bstrTuneXml,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrServiceDescription);
        
        END_INTERFACE
    } IBDA_GuideDataDeliveryServiceVtbl;

    interface IBDA_GuideDataDeliveryService
    {
        CONST_VTBL struct IBDA_GuideDataDeliveryServiceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBDA_GuideDataDeliveryService_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBDA_GuideDataDeliveryService_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBDA_GuideDataDeliveryService_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBDA_GuideDataDeliveryService_GetGuideDataType(This,pguidDataType)	\
    ( (This)->lpVtbl -> GetGuideDataType(This,pguidDataType) ) 

#define IBDA_GuideDataDeliveryService_GetGuideData(This,pulcbBufferLen,pbBuffer,pulGuideDataPercentageProgress)	\
    ( (This)->lpVtbl -> GetGuideData(This,pulcbBufferLen,pbBuffer,pulGuideDataPercentageProgress) ) 

#define IBDA_GuideDataDeliveryService_RequestGuideDataUpdate(This)	\
    ( (This)->lpVtbl -> RequestGuideDataUpdate(This) ) 

#define IBDA_GuideDataDeliveryService_GetTuneXmlFromServiceIdx(This,ul64ServiceIdx,pbstrTuneXml)	\
    ( (This)->lpVtbl -> GetTuneXmlFromServiceIdx(This,ul64ServiceIdx,pbstrTuneXml) ) 

#define IBDA_GuideDataDeliveryService_GetServices(This,pulcbBufferLen,pbBuffer)	\
    ( (This)->lpVtbl -> GetServices(This,pulcbBufferLen,pbBuffer) ) 

#define IBDA_GuideDataDeliveryService_GetServiceInfoFromTuneXml(This,bstrTuneXml,pbstrServiceDescription)	\
    ( (This)->lpVtbl -> GetServiceInfoFromTuneXml(This,bstrTuneXml,pbstrServiceDescription) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBDA_GuideDataDeliveryService_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_bdaiface_0000_0038 */
/* [local] */ 

#define SID_BDA_GuideDataDeliveryService  __uuidof(IBDA_GuideDataDeliveryService)
DEFINE_GUID(CLSID_PBDA_GDDS_DATA_TYPE, /* C80C0DF3-6052-4c16-9F56-C44C21F73C45 */
0xC80C0DF3, 0x6052, 0x4c16, 0x9F, 0x56, 0xC4, 0x4C, 0x21, 0xF7, 0x3C, 0x45);


extern RPC_IF_HANDLE __MIDL_itf_bdaiface_0000_0038_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_bdaiface_0000_0038_v0_0_s_ifspec;

#ifndef __IBDA_DRMService_INTERFACE_DEFINED__
#define __IBDA_DRMService_INTERFACE_DEFINED__

/* interface IBDA_DRMService */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IBDA_DRMService;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BFF6B5BB-B0AE-484c-9DCA-73528FB0B46E")
    IBDA_DRMService : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetDRM( 
            /* [in] */ __RPC__in GUID *puuidNewDrm) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetDRMStatus( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrDrmUuidList,
            /* [out] */ __RPC__out GUID *DrmUuid) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBDA_DRMServiceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBDA_DRMService * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBDA_DRMService * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBDA_DRMService * This);
        
        DECLSPEC_XFGVIRT(IBDA_DRMService, SetDRM)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetDRM )( 
            __RPC__in IBDA_DRMService * This,
            /* [in] */ __RPC__in GUID *puuidNewDrm);
        
        DECLSPEC_XFGVIRT(IBDA_DRMService, GetDRMStatus)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetDRMStatus )( 
            __RPC__in IBDA_DRMService * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrDrmUuidList,
            /* [out] */ __RPC__out GUID *DrmUuid);
        
        END_INTERFACE
    } IBDA_DRMServiceVtbl;

    interface IBDA_DRMService
    {
        CONST_VTBL struct IBDA_DRMServiceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBDA_DRMService_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBDA_DRMService_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBDA_DRMService_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBDA_DRMService_SetDRM(This,puuidNewDrm)	\
    ( (This)->lpVtbl -> SetDRM(This,puuidNewDrm) ) 

#define IBDA_DRMService_GetDRMStatus(This,pbstrDrmUuidList,DrmUuid)	\
    ( (This)->lpVtbl -> GetDRMStatus(This,pbstrDrmUuidList,DrmUuid) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBDA_DRMService_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_bdaiface_0000_0039 */
/* [local] */ 

#define SID_BDA_DRMService __uuidof(IBDA_DRMService)


extern RPC_IF_HANDLE __MIDL_itf_bdaiface_0000_0039_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_bdaiface_0000_0039_v0_0_s_ifspec;

#ifndef __IBDA_WMDRMSession_INTERFACE_DEFINED__
#define __IBDA_WMDRMSession_INTERFACE_DEFINED__

/* interface IBDA_WMDRMSession */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IBDA_WMDRMSession;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4BE6FA3D-07CD-4139-8B80-8C18BA3AEC88")
    IBDA_WMDRMSession : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetStatus( 
            /* [out] */ __RPC__out ULONG *MaxCaptureToken,
            /* [out] */ __RPC__out ULONG *MaxStreamingPid,
            /* [out] */ __RPC__out ULONG *MaxLicense,
            /* [out] */ __RPC__out ULONG *MinSecurityLevel,
            /* [out] */ __RPC__out ULONG *RevInfoSequenceNumber,
            /* [out] */ __RPC__out ULONGLONG *RevInfoIssuedTime,
            /* [out] */ __RPC__out ULONG *RevInfoTTL,
            /* [out] */ __RPC__out ULONG *RevListVersion,
            /* [out] */ __RPC__out ULONG *ulState) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetRevInfo( 
            /* [in] */ ULONG ulRevInfoLen,
            /* [size_is][in] */ __RPC__in_ecount_full(ulRevInfoLen) BYTE *pbRevInfo) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetCrl( 
            /* [in] */ ULONG ulCrlLen,
            /* [size_is][in] */ __RPC__in_ecount_full(ulCrlLen) BYTE *pbCrlLen) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE TransactMessage( 
            /* [in] */ ULONG ulcbRequest,
            /* [size_is][in] */ __RPC__in_ecount_full(ulcbRequest) BYTE *pbRequest,
            /* [out][in] */ __RPC__inout ULONG *pulcbResponse,
            /* [out][in] */ __RPC__inout BYTE *pbResponse) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetLicense( 
            /* [in] */ __RPC__in GUID *uuidKey,
            /* [out][in] */ __RPC__inout ULONG *pulPackageLen,
            /* [out][in] */ __RPC__inout BYTE *pbPackage) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE ReissueLicense( 
            /* [in] */ __RPC__in GUID *uuidKey) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE RenewLicense( 
            /* [in] */ ULONG ulInXmrLicenseLen,
            /* [size_is][in] */ __RPC__in_ecount_full(ulInXmrLicenseLen) BYTE *pbInXmrLicense,
            /* [in] */ ULONG ulEntitlementTokenLen,
            /* [size_is][in] */ __RPC__in_ecount_full(ulEntitlementTokenLen) BYTE *pbEntitlementToken,
            /* [out] */ __RPC__out ULONG *pulDescrambleStatus,
            /* [out][in] */ __RPC__inout ULONG *pulOutXmrLicenseLen,
            /* [out][in] */ __RPC__inout BYTE *pbOutXmrLicense) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetKeyInfo( 
            /* [out][in] */ __RPC__inout ULONG *pulKeyInfoLen,
            /* [out][in] */ __RPC__inout BYTE *pbKeyInfo) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBDA_WMDRMSessionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBDA_WMDRMSession * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBDA_WMDRMSession * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBDA_WMDRMSession * This);
        
        DECLSPEC_XFGVIRT(IBDA_WMDRMSession, GetStatus)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetStatus )( 
            __RPC__in IBDA_WMDRMSession * This,
            /* [out] */ __RPC__out ULONG *MaxCaptureToken,
            /* [out] */ __RPC__out ULONG *MaxStreamingPid,
            /* [out] */ __RPC__out ULONG *MaxLicense,
            /* [out] */ __RPC__out ULONG *MinSecurityLevel,
            /* [out] */ __RPC__out ULONG *RevInfoSequenceNumber,
            /* [out] */ __RPC__out ULONGLONG *RevInfoIssuedTime,
            /* [out] */ __RPC__out ULONG *RevInfoTTL,
            /* [out] */ __RPC__out ULONG *RevListVersion,
            /* [out] */ __RPC__out ULONG *ulState);
        
        DECLSPEC_XFGVIRT(IBDA_WMDRMSession, SetRevInfo)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetRevInfo )( 
            __RPC__in IBDA_WMDRMSession * This,
            /* [in] */ ULONG ulRevInfoLen,
            /* [size_is][in] */ __RPC__in_ecount_full(ulRevInfoLen) BYTE *pbRevInfo);
        
        DECLSPEC_XFGVIRT(IBDA_WMDRMSession, SetCrl)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetCrl )( 
            __RPC__in IBDA_WMDRMSession * This,
            /* [in] */ ULONG ulCrlLen,
            /* [size_is][in] */ __RPC__in_ecount_full(ulCrlLen) BYTE *pbCrlLen);
        
        DECLSPEC_XFGVIRT(IBDA_WMDRMSession, TransactMessage)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *TransactMessage )( 
            __RPC__in IBDA_WMDRMSession * This,
            /* [in] */ ULONG ulcbRequest,
            /* [size_is][in] */ __RPC__in_ecount_full(ulcbRequest) BYTE *pbRequest,
            /* [out][in] */ __RPC__inout ULONG *pulcbResponse,
            /* [out][in] */ __RPC__inout BYTE *pbResponse);
        
        DECLSPEC_XFGVIRT(IBDA_WMDRMSession, GetLicense)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetLicense )( 
            __RPC__in IBDA_WMDRMSession * This,
            /* [in] */ __RPC__in GUID *uuidKey,
            /* [out][in] */ __RPC__inout ULONG *pulPackageLen,
            /* [out][in] */ __RPC__inout BYTE *pbPackage);
        
        DECLSPEC_XFGVIRT(IBDA_WMDRMSession, ReissueLicense)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ReissueLicense )( 
            __RPC__in IBDA_WMDRMSession * This,
            /* [in] */ __RPC__in GUID *uuidKey);
        
        DECLSPEC_XFGVIRT(IBDA_WMDRMSession, RenewLicense)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *RenewLicense )( 
            __RPC__in IBDA_WMDRMSession * This,
            /* [in] */ ULONG ulInXmrLicenseLen,
            /* [size_is][in] */ __RPC__in_ecount_full(ulInXmrLicenseLen) BYTE *pbInXmrLicense,
            /* [in] */ ULONG ulEntitlementTokenLen,
            /* [size_is][in] */ __RPC__in_ecount_full(ulEntitlementTokenLen) BYTE *pbEntitlementToken,
            /* [out] */ __RPC__out ULONG *pulDescrambleStatus,
            /* [out][in] */ __RPC__inout ULONG *pulOutXmrLicenseLen,
            /* [out][in] */ __RPC__inout BYTE *pbOutXmrLicense);
        
        DECLSPEC_XFGVIRT(IBDA_WMDRMSession, GetKeyInfo)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetKeyInfo )( 
            __RPC__in IBDA_WMDRMSession * This,
            /* [out][in] */ __RPC__inout ULONG *pulKeyInfoLen,
            /* [out][in] */ __RPC__inout BYTE *pbKeyInfo);
        
        END_INTERFACE
    } IBDA_WMDRMSessionVtbl;

    interface IBDA_WMDRMSession
    {
        CONST_VTBL struct IBDA_WMDRMSessionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBDA_WMDRMSession_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBDA_WMDRMSession_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBDA_WMDRMSession_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBDA_WMDRMSession_GetStatus(This,MaxCaptureToken,MaxStreamingPid,MaxLicense,MinSecurityLevel,RevInfoSequenceNumber,RevInfoIssuedTime,RevInfoTTL,RevListVersion,ulState)	\
    ( (This)->lpVtbl -> GetStatus(This,MaxCaptureToken,MaxStreamingPid,MaxLicense,MinSecurityLevel,RevInfoSequenceNumber,RevInfoIssuedTime,RevInfoTTL,RevListVersion,ulState) ) 

#define IBDA_WMDRMSession_SetRevInfo(This,ulRevInfoLen,pbRevInfo)	\
    ( (This)->lpVtbl -> SetRevInfo(This,ulRevInfoLen,pbRevInfo) ) 

#define IBDA_WMDRMSession_SetCrl(This,ulCrlLen,pbCrlLen)	\
    ( (This)->lpVtbl -> SetCrl(This,ulCrlLen,pbCrlLen) ) 

#define IBDA_WMDRMSession_TransactMessage(This,ulcbRequest,pbRequest,pulcbResponse,pbResponse)	\
    ( (This)->lpVtbl -> TransactMessage(This,ulcbRequest,pbRequest,pulcbResponse,pbResponse) ) 

#define IBDA_WMDRMSession_GetLicense(This,uuidKey,pulPackageLen,pbPackage)	\
    ( (This)->lpVtbl -> GetLicense(This,uuidKey,pulPackageLen,pbPackage) ) 

#define IBDA_WMDRMSession_ReissueLicense(This,uuidKey)	\
    ( (This)->lpVtbl -> ReissueLicense(This,uuidKey) ) 

#define IBDA_WMDRMSession_RenewLicense(This,ulInXmrLicenseLen,pbInXmrLicense,ulEntitlementTokenLen,pbEntitlementToken,pulDescrambleStatus,pulOutXmrLicenseLen,pbOutXmrLicense)	\
    ( (This)->lpVtbl -> RenewLicense(This,ulInXmrLicenseLen,pbInXmrLicense,ulEntitlementTokenLen,pbEntitlementToken,pulDescrambleStatus,pulOutXmrLicenseLen,pbOutXmrLicense) ) 

#define IBDA_WMDRMSession_GetKeyInfo(This,pulKeyInfoLen,pbKeyInfo)	\
    ( (This)->lpVtbl -> GetKeyInfo(This,pulKeyInfoLen,pbKeyInfo) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBDA_WMDRMSession_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_bdaiface_0000_0040 */
/* [local] */ 

#define SID_BDA_WMDRMSession __uuidof(IBDA_WMDRMSession)


extern RPC_IF_HANDLE __MIDL_itf_bdaiface_0000_0040_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_bdaiface_0000_0040_v0_0_s_ifspec;

#ifndef __IBDA_WMDRMTuner_INTERFACE_DEFINED__
#define __IBDA_WMDRMTuner_INTERFACE_DEFINED__

/* interface IBDA_WMDRMTuner */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IBDA_WMDRMTuner;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("86D979CF-A8A7-4f94-B5FB-14C0ACA68FE6")
    IBDA_WMDRMTuner : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE PurchaseEntitlement( 
            /* [in] */ ULONG ulDialogRequest,
            /* [in] */ __RPC__in BSTR bstrLanguage,
            /* [in] */ ULONG ulPurchaseTokenLen,
            /* [size_is][in] */ __RPC__in_ecount_full(ulPurchaseTokenLen) BYTE *pbPurchaseToken,
            /* [out] */ __RPC__out ULONG *pulDescrambleStatus,
            /* [out][in] */ __RPC__inout ULONG *pulCaptureTokenLen,
            /* [out][in] */ __RPC__inout BYTE *pbCaptureToken) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE CancelCaptureToken( 
            /* [in] */ ULONG ulCaptureTokenLen,
            /* [size_is][in] */ __RPC__in_ecount_full(ulCaptureTokenLen) BYTE *pbCaptureToken) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetPidProtection( 
            /* [in] */ ULONG ulPid,
            /* [in] */ __RPC__in GUID *uuidKey) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetPidProtection( 
            /* [in] */ ULONG pulPid,
            /* [out] */ __RPC__out GUID *uuidKey) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetSyncValue( 
            /* [in] */ ULONG ulSyncValue) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetStartCodeProfile( 
            /* [out][in] */ __RPC__inout ULONG *pulStartCodeProfileLen,
            /* [out][in] */ __RPC__inout BYTE *pbStartCodeProfile) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBDA_WMDRMTunerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBDA_WMDRMTuner * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBDA_WMDRMTuner * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBDA_WMDRMTuner * This);
        
        DECLSPEC_XFGVIRT(IBDA_WMDRMTuner, PurchaseEntitlement)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *PurchaseEntitlement )( 
            __RPC__in IBDA_WMDRMTuner * This,
            /* [in] */ ULONG ulDialogRequest,
            /* [in] */ __RPC__in BSTR bstrLanguage,
            /* [in] */ ULONG ulPurchaseTokenLen,
            /* [size_is][in] */ __RPC__in_ecount_full(ulPurchaseTokenLen) BYTE *pbPurchaseToken,
            /* [out] */ __RPC__out ULONG *pulDescrambleStatus,
            /* [out][in] */ __RPC__inout ULONG *pulCaptureTokenLen,
            /* [out][in] */ __RPC__inout BYTE *pbCaptureToken);
        
        DECLSPEC_XFGVIRT(IBDA_WMDRMTuner, CancelCaptureToken)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CancelCaptureToken )( 
            __RPC__in IBDA_WMDRMTuner * This,
            /* [in] */ ULONG ulCaptureTokenLen,
            /* [size_is][in] */ __RPC__in_ecount_full(ulCaptureTokenLen) BYTE *pbCaptureToken);
        
        DECLSPEC_XFGVIRT(IBDA_WMDRMTuner, SetPidProtection)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetPidProtection )( 
            __RPC__in IBDA_WMDRMTuner * This,
            /* [in] */ ULONG ulPid,
            /* [in] */ __RPC__in GUID *uuidKey);
        
        DECLSPEC_XFGVIRT(IBDA_WMDRMTuner, GetPidProtection)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetPidProtection )( 
            __RPC__in IBDA_WMDRMTuner * This,
            /* [in] */ ULONG pulPid,
            /* [out] */ __RPC__out GUID *uuidKey);
        
        DECLSPEC_XFGVIRT(IBDA_WMDRMTuner, SetSyncValue)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetSyncValue )( 
            __RPC__in IBDA_WMDRMTuner * This,
            /* [in] */ ULONG ulSyncValue);
        
        DECLSPEC_XFGVIRT(IBDA_WMDRMTuner, GetStartCodeProfile)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetStartCodeProfile )( 
            __RPC__in IBDA_WMDRMTuner * This,
            /* [out][in] */ __RPC__inout ULONG *pulStartCodeProfileLen,
            /* [out][in] */ __RPC__inout BYTE *pbStartCodeProfile);
        
        END_INTERFACE
    } IBDA_WMDRMTunerVtbl;

    interface IBDA_WMDRMTuner
    {
        CONST_VTBL struct IBDA_WMDRMTunerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBDA_WMDRMTuner_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBDA_WMDRMTuner_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBDA_WMDRMTuner_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBDA_WMDRMTuner_PurchaseEntitlement(This,ulDialogRequest,bstrLanguage,ulPurchaseTokenLen,pbPurchaseToken,pulDescrambleStatus,pulCaptureTokenLen,pbCaptureToken)	\
    ( (This)->lpVtbl -> PurchaseEntitlement(This,ulDialogRequest,bstrLanguage,ulPurchaseTokenLen,pbPurchaseToken,pulDescrambleStatus,pulCaptureTokenLen,pbCaptureToken) ) 

#define IBDA_WMDRMTuner_CancelCaptureToken(This,ulCaptureTokenLen,pbCaptureToken)	\
    ( (This)->lpVtbl -> CancelCaptureToken(This,ulCaptureTokenLen,pbCaptureToken) ) 

#define IBDA_WMDRMTuner_SetPidProtection(This,ulPid,uuidKey)	\
    ( (This)->lpVtbl -> SetPidProtection(This,ulPid,uuidKey) ) 

#define IBDA_WMDRMTuner_GetPidProtection(This,pulPid,uuidKey)	\
    ( (This)->lpVtbl -> GetPidProtection(This,pulPid,uuidKey) ) 

#define IBDA_WMDRMTuner_SetSyncValue(This,ulSyncValue)	\
    ( (This)->lpVtbl -> SetSyncValue(This,ulSyncValue) ) 

#define IBDA_WMDRMTuner_GetStartCodeProfile(This,pulStartCodeProfileLen,pbStartCodeProfile)	\
    ( (This)->lpVtbl -> GetStartCodeProfile(This,pulStartCodeProfileLen,pbStartCodeProfile) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBDA_WMDRMTuner_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_bdaiface_0000_0041 */
/* [local] */ 

#define SID_BDA_WMDRMTuner __uuidof(IBDA_WMDRMTuner)


extern RPC_IF_HANDLE __MIDL_itf_bdaiface_0000_0041_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_bdaiface_0000_0041_v0_0_s_ifspec;

#ifndef __IBDA_DRIDRMService_INTERFACE_DEFINED__
#define __IBDA_DRIDRMService_INTERFACE_DEFINED__

/* interface IBDA_DRIDRMService */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IBDA_DRIDRMService;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1F9BC2A5-44A3-4c52-AAB1-0BBCE5A1381D")
    IBDA_DRIDRMService : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetDRM( 
            /* [in] */ __RPC__in BSTR bstrNewDrm) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetDRMStatus( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrDrmUuidList,
            /* [out] */ __RPC__out GUID *DrmUuid) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetPairingStatus( 
            /* [out][in] */ __RPC__inout BDA_DrmPairingError *penumPairingStatus) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBDA_DRIDRMServiceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBDA_DRIDRMService * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBDA_DRIDRMService * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBDA_DRIDRMService * This);
        
        DECLSPEC_XFGVIRT(IBDA_DRIDRMService, SetDRM)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetDRM )( 
            __RPC__in IBDA_DRIDRMService * This,
            /* [in] */ __RPC__in BSTR bstrNewDrm);
        
        DECLSPEC_XFGVIRT(IBDA_DRIDRMService, GetDRMStatus)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetDRMStatus )( 
            __RPC__in IBDA_DRIDRMService * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrDrmUuidList,
            /* [out] */ __RPC__out GUID *DrmUuid);
        
        DECLSPEC_XFGVIRT(IBDA_DRIDRMService, GetPairingStatus)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetPairingStatus )( 
            __RPC__in IBDA_DRIDRMService * This,
            /* [out][in] */ __RPC__inout BDA_DrmPairingError *penumPairingStatus);
        
        END_INTERFACE
    } IBDA_DRIDRMServiceVtbl;

    interface IBDA_DRIDRMService
    {
        CONST_VTBL struct IBDA_DRIDRMServiceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBDA_DRIDRMService_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBDA_DRIDRMService_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBDA_DRIDRMService_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBDA_DRIDRMService_SetDRM(This,bstrNewDrm)	\
    ( (This)->lpVtbl -> SetDRM(This,bstrNewDrm) ) 

#define IBDA_DRIDRMService_GetDRMStatus(This,pbstrDrmUuidList,DrmUuid)	\
    ( (This)->lpVtbl -> GetDRMStatus(This,pbstrDrmUuidList,DrmUuid) ) 

#define IBDA_DRIDRMService_GetPairingStatus(This,penumPairingStatus)	\
    ( (This)->lpVtbl -> GetPairingStatus(This,penumPairingStatus) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBDA_DRIDRMService_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_bdaiface_0000_0042 */
/* [local] */ 

#define SID_BDA_DRIDRMService __uuidof(IBDA_DRIDRMService)


extern RPC_IF_HANDLE __MIDL_itf_bdaiface_0000_0042_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_bdaiface_0000_0042_v0_0_s_ifspec;

#ifndef __IBDA_DRIWMDRMSession_INTERFACE_DEFINED__
#define __IBDA_DRIWMDRMSession_INTERFACE_DEFINED__

/* interface IBDA_DRIWMDRMSession */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IBDA_DRIWMDRMSession;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("05C690F8-56DB-4bb2-B053-79C12098BB26")
    IBDA_DRIWMDRMSession : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE AcknowledgeLicense( 
            /* [in] */ HRESULT hrLicenseAck) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE ProcessLicenseChallenge( 
            /* [in] */ DWORD dwcbLicenseMessage,
            /* [size_is][in] */ __RPC__in_ecount_full(dwcbLicenseMessage) BYTE *pbLicenseMessage,
            /* [out][in] */ __RPC__inout DWORD *pdwcbLicenseResponse,
            /* [size_is][out][in] */ __RPC__inout_ecount_full(*pdwcbLicenseResponse) BYTE **ppbLicenseResponse) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE ProcessRegistrationChallenge( 
            /* [in] */ DWORD dwcbRegistrationMessage,
            /* [size_is][in] */ __RPC__in_ecount_full(dwcbRegistrationMessage) BYTE *pbRegistrationMessage,
            /* [out][in] */ __RPC__inout DWORD *pdwcbRegistrationResponse,
            /* [out][in] */ __RPC__deref_inout_opt BYTE **ppbRegistrationResponse) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetRevInfo( 
            /* [in] */ DWORD dwRevInfoLen,
            /* [size_is][in] */ __RPC__in_ecount_full(dwRevInfoLen) BYTE *pbRevInfo,
            /* [out][in] */ __RPC__inout DWORD *pdwResponse) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetCrl( 
            /* [in] */ DWORD dwCrlLen,
            /* [size_is][in] */ __RPC__in_ecount_full(dwCrlLen) BYTE *pbCrlLen,
            /* [out][in] */ __RPC__inout DWORD *pdwResponse) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetHMSAssociationData( void) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetLastCardeaError( 
            /* [out][in] */ __RPC__inout DWORD *pdwError) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBDA_DRIWMDRMSessionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBDA_DRIWMDRMSession * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBDA_DRIWMDRMSession * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBDA_DRIWMDRMSession * This);
        
        DECLSPEC_XFGVIRT(IBDA_DRIWMDRMSession, AcknowledgeLicense)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AcknowledgeLicense )( 
            __RPC__in IBDA_DRIWMDRMSession * This,
            /* [in] */ HRESULT hrLicenseAck);
        
        DECLSPEC_XFGVIRT(IBDA_DRIWMDRMSession, ProcessLicenseChallenge)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ProcessLicenseChallenge )( 
            __RPC__in IBDA_DRIWMDRMSession * This,
            /* [in] */ DWORD dwcbLicenseMessage,
            /* [size_is][in] */ __RPC__in_ecount_full(dwcbLicenseMessage) BYTE *pbLicenseMessage,
            /* [out][in] */ __RPC__inout DWORD *pdwcbLicenseResponse,
            /* [size_is][out][in] */ __RPC__inout_ecount_full(*pdwcbLicenseResponse) BYTE **ppbLicenseResponse);
        
        DECLSPEC_XFGVIRT(IBDA_DRIWMDRMSession, ProcessRegistrationChallenge)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ProcessRegistrationChallenge )( 
            __RPC__in IBDA_DRIWMDRMSession * This,
            /* [in] */ DWORD dwcbRegistrationMessage,
            /* [size_is][in] */ __RPC__in_ecount_full(dwcbRegistrationMessage) BYTE *pbRegistrationMessage,
            /* [out][in] */ __RPC__inout DWORD *pdwcbRegistrationResponse,
            /* [out][in] */ __RPC__deref_inout_opt BYTE **ppbRegistrationResponse);
        
        DECLSPEC_XFGVIRT(IBDA_DRIWMDRMSession, SetRevInfo)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetRevInfo )( 
            __RPC__in IBDA_DRIWMDRMSession * This,
            /* [in] */ DWORD dwRevInfoLen,
            /* [size_is][in] */ __RPC__in_ecount_full(dwRevInfoLen) BYTE *pbRevInfo,
            /* [out][in] */ __RPC__inout DWORD *pdwResponse);
        
        DECLSPEC_XFGVIRT(IBDA_DRIWMDRMSession, SetCrl)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetCrl )( 
            __RPC__in IBDA_DRIWMDRMSession * This,
            /* [in] */ DWORD dwCrlLen,
            /* [size_is][in] */ __RPC__in_ecount_full(dwCrlLen) BYTE *pbCrlLen,
            /* [out][in] */ __RPC__inout DWORD *pdwResponse);
        
        DECLSPEC_XFGVIRT(IBDA_DRIWMDRMSession, GetHMSAssociationData)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetHMSAssociationData )( 
            __RPC__in IBDA_DRIWMDRMSession * This);
        
        DECLSPEC_XFGVIRT(IBDA_DRIWMDRMSession, GetLastCardeaError)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetLastCardeaError )( 
            __RPC__in IBDA_DRIWMDRMSession * This,
            /* [out][in] */ __RPC__inout DWORD *pdwError);
        
        END_INTERFACE
    } IBDA_DRIWMDRMSessionVtbl;

    interface IBDA_DRIWMDRMSession
    {
        CONST_VTBL struct IBDA_DRIWMDRMSessionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBDA_DRIWMDRMSession_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBDA_DRIWMDRMSession_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBDA_DRIWMDRMSession_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBDA_DRIWMDRMSession_AcknowledgeLicense(This,hrLicenseAck)	\
    ( (This)->lpVtbl -> AcknowledgeLicense(This,hrLicenseAck) ) 

#define IBDA_DRIWMDRMSession_ProcessLicenseChallenge(This,dwcbLicenseMessage,pbLicenseMessage,pdwcbLicenseResponse,ppbLicenseResponse)	\
    ( (This)->lpVtbl -> ProcessLicenseChallenge(This,dwcbLicenseMessage,pbLicenseMessage,pdwcbLicenseResponse,ppbLicenseResponse) ) 

#define IBDA_DRIWMDRMSession_ProcessRegistrationChallenge(This,dwcbRegistrationMessage,pbRegistrationMessage,pdwcbRegistrationResponse,ppbRegistrationResponse)	\
    ( (This)->lpVtbl -> ProcessRegistrationChallenge(This,dwcbRegistrationMessage,pbRegistrationMessage,pdwcbRegistrationResponse,ppbRegistrationResponse) ) 

#define IBDA_DRIWMDRMSession_SetRevInfo(This,dwRevInfoLen,pbRevInfo,pdwResponse)	\
    ( (This)->lpVtbl -> SetRevInfo(This,dwRevInfoLen,pbRevInfo,pdwResponse) ) 

#define IBDA_DRIWMDRMSession_SetCrl(This,dwCrlLen,pbCrlLen,pdwResponse)	\
    ( (This)->lpVtbl -> SetCrl(This,dwCrlLen,pbCrlLen,pdwResponse) ) 

#define IBDA_DRIWMDRMSession_GetHMSAssociationData(This)	\
    ( (This)->lpVtbl -> GetHMSAssociationData(This) ) 

#define IBDA_DRIWMDRMSession_GetLastCardeaError(This,pdwError)	\
    ( (This)->lpVtbl -> GetLastCardeaError(This,pdwError) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBDA_DRIWMDRMSession_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_bdaiface_0000_0043 */
/* [local] */ 

#define SID_BDA_DRIWMDRMSession __uuidof(IBDA_DRIWMDRMSession)


extern RPC_IF_HANDLE __MIDL_itf_bdaiface_0000_0043_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_bdaiface_0000_0043_v0_0_s_ifspec;

#ifndef __IBDA_MUX_INTERFACE_DEFINED__
#define __IBDA_MUX_INTERFACE_DEFINED__

/* interface IBDA_MUX */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IBDA_MUX;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("942AAFEC-4C05-4c74-B8EB-8706C2A4943F")
    IBDA_MUX : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetPidList( 
            /* [in] */ ULONG ulPidListCount,
            /* [size_is][in] */ __RPC__in_ecount_full(ulPidListCount) BDA_MUX_PIDLISTITEM *pbPidListBuffer) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetPidList( 
            /* [out][in] */ __RPC__inout ULONG *pulPidListCount,
            /* [out][in] */ __RPC__inout BDA_MUX_PIDLISTITEM *pbPidListBuffer) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBDA_MUXVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBDA_MUX * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBDA_MUX * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBDA_MUX * This);
        
        DECLSPEC_XFGVIRT(IBDA_MUX, SetPidList)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetPidList )( 
            __RPC__in IBDA_MUX * This,
            /* [in] */ ULONG ulPidListCount,
            /* [size_is][in] */ __RPC__in_ecount_full(ulPidListCount) BDA_MUX_PIDLISTITEM *pbPidListBuffer);
        
        DECLSPEC_XFGVIRT(IBDA_MUX, GetPidList)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetPidList )( 
            __RPC__in IBDA_MUX * This,
            /* [out][in] */ __RPC__inout ULONG *pulPidListCount,
            /* [out][in] */ __RPC__inout BDA_MUX_PIDLISTITEM *pbPidListBuffer);
        
        END_INTERFACE
    } IBDA_MUXVtbl;

    interface IBDA_MUX
    {
        CONST_VTBL struct IBDA_MUXVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBDA_MUX_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBDA_MUX_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBDA_MUX_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBDA_MUX_SetPidList(This,ulPidListCount,pbPidListBuffer)	\
    ( (This)->lpVtbl -> SetPidList(This,ulPidListCount,pbPidListBuffer) ) 

#define IBDA_MUX_GetPidList(This,pulPidListCount,pbPidListBuffer)	\
    ( (This)->lpVtbl -> GetPidList(This,pulPidListCount,pbPidListBuffer) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBDA_MUX_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_bdaiface_0000_0044 */
/* [local] */ 

#define SID_BDA_MUX __uuidof(IBDA_MUX)


extern RPC_IF_HANDLE __MIDL_itf_bdaiface_0000_0044_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_bdaiface_0000_0044_v0_0_s_ifspec;

#ifndef __IBDA_TransportStreamSelector_INTERFACE_DEFINED__
#define __IBDA_TransportStreamSelector_INTERFACE_DEFINED__

/* interface IBDA_TransportStreamSelector */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IBDA_TransportStreamSelector;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1DCFAFE9-B45E-41b3-BB2A-561EB129AE98")
    IBDA_TransportStreamSelector : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetTSID( 
            /* [in] */ USHORT usTSID) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetTSInformation( 
            /* [out][in] */ __RPC__inout ULONG *pulTSInformationBufferLen,
            /* [size_is][out] */ __RPC__out_ecount_full(*pulTSInformationBufferLen) BYTE *pbTSInformationBuffer) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBDA_TransportStreamSelectorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBDA_TransportStreamSelector * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBDA_TransportStreamSelector * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBDA_TransportStreamSelector * This);
        
        DECLSPEC_XFGVIRT(IBDA_TransportStreamSelector, SetTSID)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetTSID )( 
            __RPC__in IBDA_TransportStreamSelector * This,
            /* [in] */ USHORT usTSID);
        
        DECLSPEC_XFGVIRT(IBDA_TransportStreamSelector, GetTSInformation)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetTSInformation )( 
            __RPC__in IBDA_TransportStreamSelector * This,
            /* [out][in] */ __RPC__inout ULONG *pulTSInformationBufferLen,
            /* [size_is][out] */ __RPC__out_ecount_full(*pulTSInformationBufferLen) BYTE *pbTSInformationBuffer);
        
        END_INTERFACE
    } IBDA_TransportStreamSelectorVtbl;

    interface IBDA_TransportStreamSelector
    {
        CONST_VTBL struct IBDA_TransportStreamSelectorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBDA_TransportStreamSelector_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBDA_TransportStreamSelector_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBDA_TransportStreamSelector_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBDA_TransportStreamSelector_SetTSID(This,usTSID)	\
    ( (This)->lpVtbl -> SetTSID(This,usTSID) ) 

#define IBDA_TransportStreamSelector_GetTSInformation(This,pulTSInformationBufferLen,pbTSInformationBuffer)	\
    ( (This)->lpVtbl -> GetTSInformation(This,pulTSInformationBufferLen,pbTSInformationBuffer) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBDA_TransportStreamSelector_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_bdaiface_0000_0045 */
/* [local] */ 

#define SID_BDA_TransportStreamSelector __uuidof(IBDA_TransportStreamSelector)


extern RPC_IF_HANDLE __MIDL_itf_bdaiface_0000_0045_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_bdaiface_0000_0045_v0_0_s_ifspec;

#ifndef __IBDA_UserActivityService_INTERFACE_DEFINED__
#define __IBDA_UserActivityService_INTERFACE_DEFINED__

/* interface IBDA_UserActivityService */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IBDA_UserActivityService;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("53B14189-E478-4b7a-A1FF-506DB4B99DFE")
    IBDA_UserActivityService : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetCurrentTunerUseReason( 
            /* [in] */ DWORD dwUseReason) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetUserActivityInterval( 
            /* [out] */ __RPC__out DWORD *pdwActivityInterval) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE UserActivityDetected( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBDA_UserActivityServiceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBDA_UserActivityService * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBDA_UserActivityService * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBDA_UserActivityService * This);
        
        DECLSPEC_XFGVIRT(IBDA_UserActivityService, SetCurrentTunerUseReason)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetCurrentTunerUseReason )( 
            __RPC__in IBDA_UserActivityService * This,
            /* [in] */ DWORD dwUseReason);
        
        DECLSPEC_XFGVIRT(IBDA_UserActivityService, GetUserActivityInterval)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetUserActivityInterval )( 
            __RPC__in IBDA_UserActivityService * This,
            /* [out] */ __RPC__out DWORD *pdwActivityInterval);
        
        DECLSPEC_XFGVIRT(IBDA_UserActivityService, UserActivityDetected)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *UserActivityDetected )( 
            __RPC__in IBDA_UserActivityService * This);
        
        END_INTERFACE
    } IBDA_UserActivityServiceVtbl;

    interface IBDA_UserActivityService
    {
        CONST_VTBL struct IBDA_UserActivityServiceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBDA_UserActivityService_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBDA_UserActivityService_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBDA_UserActivityService_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBDA_UserActivityService_SetCurrentTunerUseReason(This,dwUseReason)	\
    ( (This)->lpVtbl -> SetCurrentTunerUseReason(This,dwUseReason) ) 

#define IBDA_UserActivityService_GetUserActivityInterval(This,pdwActivityInterval)	\
    ( (This)->lpVtbl -> GetUserActivityInterval(This,pdwActivityInterval) ) 

#define IBDA_UserActivityService_UserActivityDetected(This)	\
    ( (This)->lpVtbl -> UserActivityDetected(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBDA_UserActivityService_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_bdaiface_0000_0046 */
/* [local] */ 

#define SID_BDA_UserActivityService __uuidof(IBDA_UserActivityService)


extern RPC_IF_HANDLE __MIDL_itf_bdaiface_0000_0046_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_bdaiface_0000_0046_v0_0_s_ifspec;

#ifndef __IESEvent_INTERFACE_DEFINED__
#define __IESEvent_INTERFACE_DEFINED__

/* interface IESEvent */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IESEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1F0E5357-AF43-44e6-8547-654C645145D2")
    IESEvent : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetEventId( 
            /* [retval][out] */ __RPC__out DWORD *pdwEventId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEventType( 
            /* [retval][out] */ __RPC__out GUID *pguidEventType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetCompletionStatus( 
            /* [in] */ DWORD dwResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetData( 
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pbData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStringData( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrData) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IESEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IESEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IESEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IESEvent * This);
        
        DECLSPEC_XFGVIRT(IESEvent, GetEventId)
        HRESULT ( STDMETHODCALLTYPE *GetEventId )( 
            __RPC__in IESEvent * This,
            /* [retval][out] */ __RPC__out DWORD *pdwEventId);
        
        DECLSPEC_XFGVIRT(IESEvent, GetEventType)
        HRESULT ( STDMETHODCALLTYPE *GetEventType )( 
            __RPC__in IESEvent * This,
            /* [retval][out] */ __RPC__out GUID *pguidEventType);
        
        DECLSPEC_XFGVIRT(IESEvent, SetCompletionStatus)
        HRESULT ( STDMETHODCALLTYPE *SetCompletionStatus )( 
            __RPC__in IESEvent * This,
            /* [in] */ DWORD dwResult);
        
        DECLSPEC_XFGVIRT(IESEvent, GetData)
        HRESULT ( STDMETHODCALLTYPE *GetData )( 
            __RPC__in IESEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pbData);
        
        DECLSPEC_XFGVIRT(IESEvent, GetStringData)
        HRESULT ( STDMETHODCALLTYPE *GetStringData )( 
            __RPC__in IESEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrData);
        
        END_INTERFACE
    } IESEventVtbl;

    interface IESEvent
    {
        CONST_VTBL struct IESEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IESEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IESEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IESEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IESEvent_GetEventId(This,pdwEventId)	\
    ( (This)->lpVtbl -> GetEventId(This,pdwEventId) ) 

#define IESEvent_GetEventType(This,pguidEventType)	\
    ( (This)->lpVtbl -> GetEventType(This,pguidEventType) ) 

#define IESEvent_SetCompletionStatus(This,dwResult)	\
    ( (This)->lpVtbl -> SetCompletionStatus(This,dwResult) ) 

#define IESEvent_GetData(This,pbData)	\
    ( (This)->lpVtbl -> GetData(This,pbData) ) 

#define IESEvent_GetStringData(This,pbstrData)	\
    ( (This)->lpVtbl -> GetStringData(This,pbstrData) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IESEvent_INTERFACE_DEFINED__ */


#ifndef __IESEvents_INTERFACE_DEFINED__
#define __IESEvents_INTERFACE_DEFINED__

/* interface IESEvents */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IESEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("ABD414BF-CFE5-4e5e-AF5B-4B4E49C5BFEB")
    IESEvents : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnESEventReceived( 
            /* [in] */ GUID guidEventType,
            /* [in] */ __RPC__in_opt IESEvent *pESEvent) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IESEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IESEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IESEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IESEvents * This);
        
        DECLSPEC_XFGVIRT(IESEvents, OnESEventReceived)
        HRESULT ( STDMETHODCALLTYPE *OnESEventReceived )( 
            __RPC__in IESEvents * This,
            /* [in] */ GUID guidEventType,
            /* [in] */ __RPC__in_opt IESEvent *pESEvent);
        
        END_INTERFACE
    } IESEventsVtbl;

    interface IESEvents
    {
        CONST_VTBL struct IESEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IESEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IESEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IESEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IESEvents_OnESEventReceived(This,guidEventType,pESEvent)	\
    ( (This)->lpVtbl -> OnESEventReceived(This,guidEventType,pESEvent) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IESEvents_INTERFACE_DEFINED__ */


#ifndef __IBroadcastEvent_INTERFACE_DEFINED__
#define __IBroadcastEvent_INTERFACE_DEFINED__

/* interface IBroadcastEvent */
/* [unique][helpstring][uuid][nonextensible][hidden][object] */ 


EXTERN_C const IID IID_IBroadcastEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3B21263F-26E8-489d-AAC4-924F7EFD9511")
    IBroadcastEvent : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Fire( 
            /* [in] */ GUID EventID) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBroadcastEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBroadcastEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBroadcastEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBroadcastEvent * This);
        
        DECLSPEC_XFGVIRT(IBroadcastEvent, Fire)
        HRESULT ( STDMETHODCALLTYPE *Fire )( 
            __RPC__in IBroadcastEvent * This,
            /* [in] */ GUID EventID);
        
        END_INTERFACE
    } IBroadcastEventVtbl;

    interface IBroadcastEvent
    {
        CONST_VTBL struct IBroadcastEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBroadcastEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBroadcastEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBroadcastEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBroadcastEvent_Fire(This,EventID)	\
    ( (This)->lpVtbl -> Fire(This,EventID) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBroadcastEvent_INTERFACE_DEFINED__ */


#ifndef __IBroadcastEventEx_INTERFACE_DEFINED__
#define __IBroadcastEventEx_INTERFACE_DEFINED__

/* interface IBroadcastEventEx */
/* [unique][helpstring][uuid][nonextensible][hidden][object] */ 


EXTERN_C const IID IID_IBroadcastEventEx;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3d9e3887-1929-423f-8021-43682de95448")
    IBroadcastEventEx : public IBroadcastEvent
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE FireEx( 
            /* [in] */ GUID EventID,
            /* [in] */ ULONG Param1,
            /* [in] */ ULONG Param2,
            /* [in] */ ULONG Param3,
            /* [in] */ ULONG Param4) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBroadcastEventExVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBroadcastEventEx * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBroadcastEventEx * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBroadcastEventEx * This);
        
        DECLSPEC_XFGVIRT(IBroadcastEvent, Fire)
        HRESULT ( STDMETHODCALLTYPE *Fire )( 
            __RPC__in IBroadcastEventEx * This,
            /* [in] */ GUID EventID);
        
        DECLSPEC_XFGVIRT(IBroadcastEventEx, FireEx)
        HRESULT ( STDMETHODCALLTYPE *FireEx )( 
            __RPC__in IBroadcastEventEx * This,
            /* [in] */ GUID EventID,
            /* [in] */ ULONG Param1,
            /* [in] */ ULONG Param2,
            /* [in] */ ULONG Param3,
            /* [in] */ ULONG Param4);
        
        END_INTERFACE
    } IBroadcastEventExVtbl;

    interface IBroadcastEventEx
    {
        CONST_VTBL struct IBroadcastEventExVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBroadcastEventEx_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBroadcastEventEx_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBroadcastEventEx_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBroadcastEventEx_Fire(This,EventID)	\
    ( (This)->lpVtbl -> Fire(This,EventID) ) 


#define IBroadcastEventEx_FireEx(This,EventID,Param1,Param2,Param3,Param4)	\
    ( (This)->lpVtbl -> FireEx(This,EventID,Param1,Param2,Param3,Param4) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBroadcastEventEx_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_bdaiface_0000_0050 */
/* [local] */ 

DEFINE_GUID(CLSID_BroadcastEventService, 0x0B3FFB92, 0x0919, 0x4934, 0x9D, 0x5B, 0x61, 0x9C, 0x71, 0x9D, 0x02, 0x02);
#define SID_SBroadcastEventService CLSID_BroadcastEventService
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_bdaiface_0000_0050_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_bdaiface_0000_0050_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  LPSAFEARRAY_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out LPSAFEARRAY * ); 
void                      __RPC_USER  LPSAFEARRAY_UserFree(     __RPC__in unsigned long *, __RPC__in LPSAFEARRAY * ); 

unsigned long             __RPC_USER  BSTR_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree64(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  LPSAFEARRAY_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out LPSAFEARRAY * ); 
void                      __RPC_USER  LPSAFEARRAY_UserFree64(     __RPC__in unsigned long *, __RPC__in LPSAFEARRAY * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif





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

#ifndef __mbnapi_h__
#define __mbnapi_h__

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

#ifndef __IDummyMBNUCMExt_FWD_DEFINED__
#define __IDummyMBNUCMExt_FWD_DEFINED__
typedef interface IDummyMBNUCMExt IDummyMBNUCMExt;

#endif 	/* __IDummyMBNUCMExt_FWD_DEFINED__ */


#ifndef __IMbnConnection_FWD_DEFINED__
#define __IMbnConnection_FWD_DEFINED__
typedef interface IMbnConnection IMbnConnection;

#endif 	/* __IMbnConnection_FWD_DEFINED__ */


#ifndef __IMbnConnectionEvents_FWD_DEFINED__
#define __IMbnConnectionEvents_FWD_DEFINED__
typedef interface IMbnConnectionEvents IMbnConnectionEvents;

#endif 	/* __IMbnConnectionEvents_FWD_DEFINED__ */


#ifndef __IMbnInterface_FWD_DEFINED__
#define __IMbnInterface_FWD_DEFINED__
typedef interface IMbnInterface IMbnInterface;

#endif 	/* __IMbnInterface_FWD_DEFINED__ */


#ifndef __IMbnInterfaceEvents_FWD_DEFINED__
#define __IMbnInterfaceEvents_FWD_DEFINED__
typedef interface IMbnInterfaceEvents IMbnInterfaceEvents;

#endif 	/* __IMbnInterfaceEvents_FWD_DEFINED__ */


#ifndef __IMbnInterfaceManager_FWD_DEFINED__
#define __IMbnInterfaceManager_FWD_DEFINED__
typedef interface IMbnInterfaceManager IMbnInterfaceManager;

#endif 	/* __IMbnInterfaceManager_FWD_DEFINED__ */


#ifndef __IMbnInterfaceManagerEvents_FWD_DEFINED__
#define __IMbnInterfaceManagerEvents_FWD_DEFINED__
typedef interface IMbnInterfaceManagerEvents IMbnInterfaceManagerEvents;

#endif 	/* __IMbnInterfaceManagerEvents_FWD_DEFINED__ */


#ifndef __IMbnRegistration_FWD_DEFINED__
#define __IMbnRegistration_FWD_DEFINED__
typedef interface IMbnRegistration IMbnRegistration;

#endif 	/* __IMbnRegistration_FWD_DEFINED__ */


#ifndef __IMbnRegistrationEvents_FWD_DEFINED__
#define __IMbnRegistrationEvents_FWD_DEFINED__
typedef interface IMbnRegistrationEvents IMbnRegistrationEvents;

#endif 	/* __IMbnRegistrationEvents_FWD_DEFINED__ */


#ifndef __IMbnConnectionManager_FWD_DEFINED__
#define __IMbnConnectionManager_FWD_DEFINED__
typedef interface IMbnConnectionManager IMbnConnectionManager;

#endif 	/* __IMbnConnectionManager_FWD_DEFINED__ */


#ifndef __IMbnConnectionManagerEvents_FWD_DEFINED__
#define __IMbnConnectionManagerEvents_FWD_DEFINED__
typedef interface IMbnConnectionManagerEvents IMbnConnectionManagerEvents;

#endif 	/* __IMbnConnectionManagerEvents_FWD_DEFINED__ */


#ifndef __IMbnPinManager_FWD_DEFINED__
#define __IMbnPinManager_FWD_DEFINED__
typedef interface IMbnPinManager IMbnPinManager;

#endif 	/* __IMbnPinManager_FWD_DEFINED__ */


#ifndef __IMbnPinManagerEvents_FWD_DEFINED__
#define __IMbnPinManagerEvents_FWD_DEFINED__
typedef interface IMbnPinManagerEvents IMbnPinManagerEvents;

#endif 	/* __IMbnPinManagerEvents_FWD_DEFINED__ */


#ifndef __IMbnPinEvents_FWD_DEFINED__
#define __IMbnPinEvents_FWD_DEFINED__
typedef interface IMbnPinEvents IMbnPinEvents;

#endif 	/* __IMbnPinEvents_FWD_DEFINED__ */


#ifndef __IMbnSubscriberInformation_FWD_DEFINED__
#define __IMbnSubscriberInformation_FWD_DEFINED__
typedef interface IMbnSubscriberInformation IMbnSubscriberInformation;

#endif 	/* __IMbnSubscriberInformation_FWD_DEFINED__ */


#ifndef __IMbnSignal_FWD_DEFINED__
#define __IMbnSignal_FWD_DEFINED__
typedef interface IMbnSignal IMbnSignal;

#endif 	/* __IMbnSignal_FWD_DEFINED__ */


#ifndef __IMbnSignalEvents_FWD_DEFINED__
#define __IMbnSignalEvents_FWD_DEFINED__
typedef interface IMbnSignalEvents IMbnSignalEvents;

#endif 	/* __IMbnSignalEvents_FWD_DEFINED__ */


#ifndef __IMbnConnectionContext_FWD_DEFINED__
#define __IMbnConnectionContext_FWD_DEFINED__
typedef interface IMbnConnectionContext IMbnConnectionContext;

#endif 	/* __IMbnConnectionContext_FWD_DEFINED__ */


#ifndef __IMbnConnectionContextEvents_FWD_DEFINED__
#define __IMbnConnectionContextEvents_FWD_DEFINED__
typedef interface IMbnConnectionContextEvents IMbnConnectionContextEvents;

#endif 	/* __IMbnConnectionContextEvents_FWD_DEFINED__ */


#ifndef __IMbnConnectionProfileManager_FWD_DEFINED__
#define __IMbnConnectionProfileManager_FWD_DEFINED__
typedef interface IMbnConnectionProfileManager IMbnConnectionProfileManager;

#endif 	/* __IMbnConnectionProfileManager_FWD_DEFINED__ */


#ifndef __IMbnConnectionProfile_FWD_DEFINED__
#define __IMbnConnectionProfile_FWD_DEFINED__
typedef interface IMbnConnectionProfile IMbnConnectionProfile;

#endif 	/* __IMbnConnectionProfile_FWD_DEFINED__ */


#ifndef __IMbnConnectionProfileEvents_FWD_DEFINED__
#define __IMbnConnectionProfileEvents_FWD_DEFINED__
typedef interface IMbnConnectionProfileEvents IMbnConnectionProfileEvents;

#endif 	/* __IMbnConnectionProfileEvents_FWD_DEFINED__ */


#ifndef __IMbnSmsConfiguration_FWD_DEFINED__
#define __IMbnSmsConfiguration_FWD_DEFINED__
typedef interface IMbnSmsConfiguration IMbnSmsConfiguration;

#endif 	/* __IMbnSmsConfiguration_FWD_DEFINED__ */


#ifndef __IMbnSmsReadMsgPdu_FWD_DEFINED__
#define __IMbnSmsReadMsgPdu_FWD_DEFINED__
typedef interface IMbnSmsReadMsgPdu IMbnSmsReadMsgPdu;

#endif 	/* __IMbnSmsReadMsgPdu_FWD_DEFINED__ */


#ifndef __IMbnSmsReadMsgTextCdma_FWD_DEFINED__
#define __IMbnSmsReadMsgTextCdma_FWD_DEFINED__
typedef interface IMbnSmsReadMsgTextCdma IMbnSmsReadMsgTextCdma;

#endif 	/* __IMbnSmsReadMsgTextCdma_FWD_DEFINED__ */


#ifndef __IMbnSms_FWD_DEFINED__
#define __IMbnSms_FWD_DEFINED__
typedef interface IMbnSms IMbnSms;

#endif 	/* __IMbnSms_FWD_DEFINED__ */


#ifndef __IMbnSmsEvents_FWD_DEFINED__
#define __IMbnSmsEvents_FWD_DEFINED__
typedef interface IMbnSmsEvents IMbnSmsEvents;

#endif 	/* __IMbnSmsEvents_FWD_DEFINED__ */


#ifndef __IMbnServiceActivation_FWD_DEFINED__
#define __IMbnServiceActivation_FWD_DEFINED__
typedef interface IMbnServiceActivation IMbnServiceActivation;

#endif 	/* __IMbnServiceActivation_FWD_DEFINED__ */


#ifndef __IMbnServiceActivationEvents_FWD_DEFINED__
#define __IMbnServiceActivationEvents_FWD_DEFINED__
typedef interface IMbnServiceActivationEvents IMbnServiceActivationEvents;

#endif 	/* __IMbnServiceActivationEvents_FWD_DEFINED__ */


#ifndef __IMbnVendorSpecificOperation_FWD_DEFINED__
#define __IMbnVendorSpecificOperation_FWD_DEFINED__
typedef interface IMbnVendorSpecificOperation IMbnVendorSpecificOperation;

#endif 	/* __IMbnVendorSpecificOperation_FWD_DEFINED__ */


#ifndef __IMbnVendorSpecificEvents_FWD_DEFINED__
#define __IMbnVendorSpecificEvents_FWD_DEFINED__
typedef interface IMbnVendorSpecificEvents IMbnVendorSpecificEvents;

#endif 	/* __IMbnVendorSpecificEvents_FWD_DEFINED__ */


#ifndef __IMbnConnectionProfileManagerEvents_FWD_DEFINED__
#define __IMbnConnectionProfileManagerEvents_FWD_DEFINED__
typedef interface IMbnConnectionProfileManagerEvents IMbnConnectionProfileManagerEvents;

#endif 	/* __IMbnConnectionProfileManagerEvents_FWD_DEFINED__ */


#ifndef __IMbnRadio_FWD_DEFINED__
#define __IMbnRadio_FWD_DEFINED__
typedef interface IMbnRadio IMbnRadio;

#endif 	/* __IMbnRadio_FWD_DEFINED__ */


#ifndef __IMbnRadioEvents_FWD_DEFINED__
#define __IMbnRadioEvents_FWD_DEFINED__
typedef interface IMbnRadioEvents IMbnRadioEvents;

#endif 	/* __IMbnRadioEvents_FWD_DEFINED__ */


#ifndef __IMbnMultiCarrier_FWD_DEFINED__
#define __IMbnMultiCarrier_FWD_DEFINED__
typedef interface IMbnMultiCarrier IMbnMultiCarrier;

#endif 	/* __IMbnMultiCarrier_FWD_DEFINED__ */


#ifndef __IMbnMultiCarrierEvents_FWD_DEFINED__
#define __IMbnMultiCarrierEvents_FWD_DEFINED__
typedef interface IMbnMultiCarrierEvents IMbnMultiCarrierEvents;

#endif 	/* __IMbnMultiCarrierEvents_FWD_DEFINED__ */


#ifndef __IMbnDeviceServiceStateEvents_FWD_DEFINED__
#define __IMbnDeviceServiceStateEvents_FWD_DEFINED__
typedef interface IMbnDeviceServiceStateEvents IMbnDeviceServiceStateEvents;

#endif 	/* __IMbnDeviceServiceStateEvents_FWD_DEFINED__ */


#ifndef __IMbnDeviceServicesManager_FWD_DEFINED__
#define __IMbnDeviceServicesManager_FWD_DEFINED__
typedef interface IMbnDeviceServicesManager IMbnDeviceServicesManager;

#endif 	/* __IMbnDeviceServicesManager_FWD_DEFINED__ */


#ifndef __IMbnDeviceServicesContext_FWD_DEFINED__
#define __IMbnDeviceServicesContext_FWD_DEFINED__
typedef interface IMbnDeviceServicesContext IMbnDeviceServicesContext;

#endif 	/* __IMbnDeviceServicesContext_FWD_DEFINED__ */


#ifndef __IMbnDeviceServicesEvents_FWD_DEFINED__
#define __IMbnDeviceServicesEvents_FWD_DEFINED__
typedef interface IMbnDeviceServicesEvents IMbnDeviceServicesEvents;

#endif 	/* __IMbnDeviceServicesEvents_FWD_DEFINED__ */


#ifndef __IMbnDeviceService_FWD_DEFINED__
#define __IMbnDeviceService_FWD_DEFINED__
typedef interface IMbnDeviceService IMbnDeviceService;

#endif 	/* __IMbnDeviceService_FWD_DEFINED__ */


#ifndef __IMbnInterface_FWD_DEFINED__
#define __IMbnInterface_FWD_DEFINED__
typedef interface IMbnInterface IMbnInterface;

#endif 	/* __IMbnInterface_FWD_DEFINED__ */


#ifndef __IMbnSubscriberInformation_FWD_DEFINED__
#define __IMbnSubscriberInformation_FWD_DEFINED__
typedef interface IMbnSubscriberInformation IMbnSubscriberInformation;

#endif 	/* __IMbnSubscriberInformation_FWD_DEFINED__ */


#ifndef __IMbnConnection_FWD_DEFINED__
#define __IMbnConnection_FWD_DEFINED__
typedef interface IMbnConnection IMbnConnection;

#endif 	/* __IMbnConnection_FWD_DEFINED__ */


#ifndef __IMbnInterfaceEvents_FWD_DEFINED__
#define __IMbnInterfaceEvents_FWD_DEFINED__
typedef interface IMbnInterfaceEvents IMbnInterfaceEvents;

#endif 	/* __IMbnInterfaceEvents_FWD_DEFINED__ */


#ifndef __IMbnSignal_FWD_DEFINED__
#define __IMbnSignal_FWD_DEFINED__
typedef interface IMbnSignal IMbnSignal;

#endif 	/* __IMbnSignal_FWD_DEFINED__ */


#ifndef __IMbnSignalEvents_FWD_DEFINED__
#define __IMbnSignalEvents_FWD_DEFINED__
typedef interface IMbnSignalEvents IMbnSignalEvents;

#endif 	/* __IMbnSignalEvents_FWD_DEFINED__ */


#ifndef __IMbnPinManager_FWD_DEFINED__
#define __IMbnPinManager_FWD_DEFINED__
typedef interface IMbnPinManager IMbnPinManager;

#endif 	/* __IMbnPinManager_FWD_DEFINED__ */


#ifndef __IMbnPinManagerEvents_FWD_DEFINED__
#define __IMbnPinManagerEvents_FWD_DEFINED__
typedef interface IMbnPinManagerEvents IMbnPinManagerEvents;

#endif 	/* __IMbnPinManagerEvents_FWD_DEFINED__ */


#ifndef __IMbnPinEvents_FWD_DEFINED__
#define __IMbnPinEvents_FWD_DEFINED__
typedef interface IMbnPinEvents IMbnPinEvents;

#endif 	/* __IMbnPinEvents_FWD_DEFINED__ */


#ifndef __IMbnRegistration_FWD_DEFINED__
#define __IMbnRegistration_FWD_DEFINED__
typedef interface IMbnRegistration IMbnRegistration;

#endif 	/* __IMbnRegistration_FWD_DEFINED__ */


#ifndef __IMbnRegistrationEvents_FWD_DEFINED__
#define __IMbnRegistrationEvents_FWD_DEFINED__
typedef interface IMbnRegistrationEvents IMbnRegistrationEvents;

#endif 	/* __IMbnRegistrationEvents_FWD_DEFINED__ */


#ifndef __IMbnConnectionContext_FWD_DEFINED__
#define __IMbnConnectionContext_FWD_DEFINED__
typedef interface IMbnConnectionContext IMbnConnectionContext;

#endif 	/* __IMbnConnectionContext_FWD_DEFINED__ */


#ifndef __IMbnConnectionContextEvents_FWD_DEFINED__
#define __IMbnConnectionContextEvents_FWD_DEFINED__
typedef interface IMbnConnectionContextEvents IMbnConnectionContextEvents;

#endif 	/* __IMbnConnectionContextEvents_FWD_DEFINED__ */


#ifndef __IMbnConnectionEvents_FWD_DEFINED__
#define __IMbnConnectionEvents_FWD_DEFINED__
typedef interface IMbnConnectionEvents IMbnConnectionEvents;

#endif 	/* __IMbnConnectionEvents_FWD_DEFINED__ */


#ifndef __IMbnConnectionProfileManager_FWD_DEFINED__
#define __IMbnConnectionProfileManager_FWD_DEFINED__
typedef interface IMbnConnectionProfileManager IMbnConnectionProfileManager;

#endif 	/* __IMbnConnectionProfileManager_FWD_DEFINED__ */


#ifndef __IMbnConnectionProfile_FWD_DEFINED__
#define __IMbnConnectionProfile_FWD_DEFINED__
typedef interface IMbnConnectionProfile IMbnConnectionProfile;

#endif 	/* __IMbnConnectionProfile_FWD_DEFINED__ */


#ifndef __IMbnConnectionProfileEvents_FWD_DEFINED__
#define __IMbnConnectionProfileEvents_FWD_DEFINED__
typedef interface IMbnConnectionProfileEvents IMbnConnectionProfileEvents;

#endif 	/* __IMbnConnectionProfileEvents_FWD_DEFINED__ */


#ifndef __IMbnSmsConfiguration_FWD_DEFINED__
#define __IMbnSmsConfiguration_FWD_DEFINED__
typedef interface IMbnSmsConfiguration IMbnSmsConfiguration;

#endif 	/* __IMbnSmsConfiguration_FWD_DEFINED__ */


#ifndef __IMbnSmsReadMsgPdu_FWD_DEFINED__
#define __IMbnSmsReadMsgPdu_FWD_DEFINED__
typedef interface IMbnSmsReadMsgPdu IMbnSmsReadMsgPdu;

#endif 	/* __IMbnSmsReadMsgPdu_FWD_DEFINED__ */


#ifndef __IMbnSmsReadMsgTextCdma_FWD_DEFINED__
#define __IMbnSmsReadMsgTextCdma_FWD_DEFINED__
typedef interface IMbnSmsReadMsgTextCdma IMbnSmsReadMsgTextCdma;

#endif 	/* __IMbnSmsReadMsgTextCdma_FWD_DEFINED__ */


#ifndef __IMbnSms_FWD_DEFINED__
#define __IMbnSms_FWD_DEFINED__
typedef interface IMbnSms IMbnSms;

#endif 	/* __IMbnSms_FWD_DEFINED__ */


#ifndef __IMbnSmsEvents_FWD_DEFINED__
#define __IMbnSmsEvents_FWD_DEFINED__
typedef interface IMbnSmsEvents IMbnSmsEvents;

#endif 	/* __IMbnSmsEvents_FWD_DEFINED__ */


#ifndef __IMbnServiceActivation_FWD_DEFINED__
#define __IMbnServiceActivation_FWD_DEFINED__
typedef interface IMbnServiceActivation IMbnServiceActivation;

#endif 	/* __IMbnServiceActivation_FWD_DEFINED__ */


#ifndef __IMbnServiceActivationEvents_FWD_DEFINED__
#define __IMbnServiceActivationEvents_FWD_DEFINED__
typedef interface IMbnServiceActivationEvents IMbnServiceActivationEvents;

#endif 	/* __IMbnServiceActivationEvents_FWD_DEFINED__ */


#ifndef __IMbnVendorSpecificOperation_FWD_DEFINED__
#define __IMbnVendorSpecificOperation_FWD_DEFINED__
typedef interface IMbnVendorSpecificOperation IMbnVendorSpecificOperation;

#endif 	/* __IMbnVendorSpecificOperation_FWD_DEFINED__ */


#ifndef __IMbnVendorSpecificEvents_FWD_DEFINED__
#define __IMbnVendorSpecificEvents_FWD_DEFINED__
typedef interface IMbnVendorSpecificEvents IMbnVendorSpecificEvents;

#endif 	/* __IMbnVendorSpecificEvents_FWD_DEFINED__ */


#ifndef __IMbnInterfaceManager_FWD_DEFINED__
#define __IMbnInterfaceManager_FWD_DEFINED__
typedef interface IMbnInterfaceManager IMbnInterfaceManager;

#endif 	/* __IMbnInterfaceManager_FWD_DEFINED__ */


#ifndef __IMbnInterfaceManagerEvents_FWD_DEFINED__
#define __IMbnInterfaceManagerEvents_FWD_DEFINED__
typedef interface IMbnInterfaceManagerEvents IMbnInterfaceManagerEvents;

#endif 	/* __IMbnInterfaceManagerEvents_FWD_DEFINED__ */


#ifndef __IMbnConnectionManager_FWD_DEFINED__
#define __IMbnConnectionManager_FWD_DEFINED__
typedef interface IMbnConnectionManager IMbnConnectionManager;

#endif 	/* __IMbnConnectionManager_FWD_DEFINED__ */


#ifndef __IMbnConnectionManagerEvents_FWD_DEFINED__
#define __IMbnConnectionManagerEvents_FWD_DEFINED__
typedef interface IMbnConnectionManagerEvents IMbnConnectionManagerEvents;

#endif 	/* __IMbnConnectionManagerEvents_FWD_DEFINED__ */


#ifndef __IMbnConnectionProfileManagerEvents_FWD_DEFINED__
#define __IMbnConnectionProfileManagerEvents_FWD_DEFINED__
typedef interface IMbnConnectionProfileManagerEvents IMbnConnectionProfileManagerEvents;

#endif 	/* __IMbnConnectionProfileManagerEvents_FWD_DEFINED__ */


#ifndef __IMbnRadio_FWD_DEFINED__
#define __IMbnRadio_FWD_DEFINED__
typedef interface IMbnRadio IMbnRadio;

#endif 	/* __IMbnRadio_FWD_DEFINED__ */


#ifndef __IMbnRadioEvents_FWD_DEFINED__
#define __IMbnRadioEvents_FWD_DEFINED__
typedef interface IMbnRadioEvents IMbnRadioEvents;

#endif 	/* __IMbnRadioEvents_FWD_DEFINED__ */


#ifndef __IMbnMultiCarrier_FWD_DEFINED__
#define __IMbnMultiCarrier_FWD_DEFINED__
typedef interface IMbnMultiCarrier IMbnMultiCarrier;

#endif 	/* __IMbnMultiCarrier_FWD_DEFINED__ */


#ifndef __IMbnMultiCarrierEvents_FWD_DEFINED__
#define __IMbnMultiCarrierEvents_FWD_DEFINED__
typedef interface IMbnMultiCarrierEvents IMbnMultiCarrierEvents;

#endif 	/* __IMbnMultiCarrierEvents_FWD_DEFINED__ */


#ifndef __IMbnDeviceServiceStateEvents_FWD_DEFINED__
#define __IMbnDeviceServiceStateEvents_FWD_DEFINED__
typedef interface IMbnDeviceServiceStateEvents IMbnDeviceServiceStateEvents;

#endif 	/* __IMbnDeviceServiceStateEvents_FWD_DEFINED__ */


#ifndef __IMbnPin_FWD_DEFINED__
#define __IMbnPin_FWD_DEFINED__
typedef interface IMbnPin IMbnPin;

#endif 	/* __IMbnPin_FWD_DEFINED__ */


#ifndef __IMbnDeviceService_FWD_DEFINED__
#define __IMbnDeviceService_FWD_DEFINED__
typedef interface IMbnDeviceService IMbnDeviceService;

#endif 	/* __IMbnDeviceService_FWD_DEFINED__ */


#ifndef __IMbnDeviceServicesContext_FWD_DEFINED__
#define __IMbnDeviceServicesContext_FWD_DEFINED__
typedef interface IMbnDeviceServicesContext IMbnDeviceServicesContext;

#endif 	/* __IMbnDeviceServicesContext_FWD_DEFINED__ */


#ifndef __IMbnDeviceServicesManager_FWD_DEFINED__
#define __IMbnDeviceServicesManager_FWD_DEFINED__
typedef interface IMbnDeviceServicesManager IMbnDeviceServicesManager;

#endif 	/* __IMbnDeviceServicesManager_FWD_DEFINED__ */


#ifndef __IMbnDeviceServicesEvents_FWD_DEFINED__
#define __IMbnDeviceServicesEvents_FWD_DEFINED__
typedef interface IMbnDeviceServicesEvents IMbnDeviceServicesEvents;

#endif 	/* __IMbnDeviceServicesEvents_FWD_DEFINED__ */


#ifndef __MbnConnectionProfileManager_FWD_DEFINED__
#define __MbnConnectionProfileManager_FWD_DEFINED__

#ifdef __cplusplus
typedef class MbnConnectionProfileManager MbnConnectionProfileManager;
#else
typedef struct MbnConnectionProfileManager MbnConnectionProfileManager;
#endif /* __cplusplus */

#endif 	/* __MbnConnectionProfileManager_FWD_DEFINED__ */


#ifndef __MbnInterfaceManager_FWD_DEFINED__
#define __MbnInterfaceManager_FWD_DEFINED__

#ifdef __cplusplus
typedef class MbnInterfaceManager MbnInterfaceManager;
#else
typedef struct MbnInterfaceManager MbnInterfaceManager;
#endif /* __cplusplus */

#endif 	/* __MbnInterfaceManager_FWD_DEFINED__ */


#ifndef __MbnConnectionManager_FWD_DEFINED__
#define __MbnConnectionManager_FWD_DEFINED__

#ifdef __cplusplus
typedef class MbnConnectionManager MbnConnectionManager;
#else
typedef struct MbnConnectionManager MbnConnectionManager;
#endif /* __cplusplus */

#endif 	/* __MbnConnectionManager_FWD_DEFINED__ */


#ifndef __MbnDeviceServicesManager_FWD_DEFINED__
#define __MbnDeviceServicesManager_FWD_DEFINED__

#ifdef __cplusplus
typedef class MbnDeviceServicesManager MbnDeviceServicesManager;
#else
typedef struct MbnDeviceServicesManager MbnDeviceServicesManager;
#endif /* __cplusplus */

#endif 	/* __MbnDeviceServicesManager_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_mbnapi_0000_0000 */
/* [local] */ 


#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)






































extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0000_v0_0_s_ifspec;

#ifndef __IDummyMBNUCMExt_INTERFACE_DEFINED__
#define __IDummyMBNUCMExt_INTERFACE_DEFINED__

/* interface IDummyMBNUCMExt */
/* [object][uuid] */ 


EXTERN_C const IID IID_IDummyMBNUCMExt;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DCBBBAB6-FFFF-4BBB-AAEE-338E368AF6FA")
    IDummyMBNUCMExt : public IDispatch
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct IDummyMBNUCMExtVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDummyMBNUCMExt * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDummyMBNUCMExt * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDummyMBNUCMExt * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IDummyMBNUCMExt * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IDummyMBNUCMExt * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IDummyMBNUCMExt * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IDummyMBNUCMExt * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        END_INTERFACE
    } IDummyMBNUCMExtVtbl;

    interface IDummyMBNUCMExt
    {
        CONST_VTBL struct IDummyMBNUCMExtVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDummyMBNUCMExt_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDummyMBNUCMExt_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDummyMBNUCMExt_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDummyMBNUCMExt_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IDummyMBNUCMExt_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IDummyMBNUCMExt_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IDummyMBNUCMExt_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDummyMBNUCMExt_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mbnapi_0000_0001 */
/* [local] */ 

typedef /* [helpstring][v1_enum][version][uuid] */  DECLSPEC_UUID("42FAAC0B-BDCC-11DC-A8A8-001321F1405F") 
enum MBN_SIGNAL_CONSTANTS
    {
        MBN_RSSI_DEFAULT	= 0xffffffff,
        MBN_RSSI_DISABLE	= 0,
        MBN_RSSI_UNKNOWN	= 99,
        MBN_ERROR_RATE_UNKNOWN	= 99
    } 	MBN_SIGNAL_CONSTANTS;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
typedef /* [helpstring][v1_enum][version][uuid] */  DECLSPEC_UUID("DCBBBAB6-6002-4BBB-AAEE-338E368AF6FA") 
enum MBN_CELLULAR_CLASS
    {
        MBN_CELLULAR_CLASS_NONE	= 0,
        MBN_CELLULAR_CLASS_GSM	= 0x1,
        MBN_CELLULAR_CLASS_CDMA	= 0x2
    } 	MBN_CELLULAR_CLASS;

typedef /* [helpstring][v1_enum][version][uuid] */  DECLSPEC_UUID("DCBBBAB6-6012-4BBB-AAEE-338E368AF6FA") 
enum MBN_VOICE_CLASS
    {
        MBN_VOICE_CLASS_NONE	= 0,
        MBN_VOICE_CLASS_NO_VOICE	= 0x1,
        MBN_VOICE_CLASS_SEPARATE_VOICE_DATA	= 0x2,
        MBN_VOICE_CLASS_SIMULTANEOUS_VOICE_DATA	= 0x3
    } 	MBN_VOICE_CLASS;

typedef /* [helpstring][v1_enum][version][uuid] */  DECLSPEC_UUID("6E1348BB-BDCB-11DC-A8A8-001321F1405F") 
enum MBN_PROVIDER_STATE
    {
        MBN_PROVIDER_STATE_NONE	= 0,
        MBN_PROVIDER_STATE_HOME	= 0x1,
        MBN_PROVIDER_STATE_FORBIDDEN	= 0x2,
        MBN_PROVIDER_STATE_PREFERRED	= 0x4,
        MBN_PROVIDER_STATE_VISIBLE	= 0x8,
        MBN_PROVIDER_STATE_REGISTERED	= 0x10,
        MBN_PROVIDER_STATE_PREFERRED_MULTICARRIER	= 0x20
    } 	MBN_PROVIDER_STATE;

typedef /* [helpstring][v1_enum][version][uuid] */  DECLSPEC_UUID("934092FD-BDCB-11DC-A8A8-001321F1405F") 
enum MBN_PROVIDER_CONSTANTS
    {
        MBN_PROVIDERNAME_LEN	= 20,
        MBN_PROVIDERID_LEN	= 6
    } 	MBN_PROVIDER_CONSTANTS;

typedef /* [helpstring][v1_enum][version][uuid] */  DECLSPEC_UUID("59C235E7-BDC9-11DC-A8A8-001321F1405F") 
enum MBN_INTERFACE_CAPS_CONSTANTS
    {
        MBN_DEVICEID_LEN	= 18,
        MBN_MANUFACTURER_LEN	= 32,
        MBN_MODEL_LEN	= 32,
        MBN_FIRMWARE_LEN	= 32
    } 	MBN_INTERFACE_CAPS_CONSTANTS;

typedef /* [helpstring][v1_enum][version][uuid] */  DECLSPEC_UUID("83A34F4C-BDC4-11DC-A8A8-001321F1405F") 
enum MBN_DATA_CLASS
    {
        MBN_DATA_CLASS_NONE	= 0,
        MBN_DATA_CLASS_GPRS	= 0x1,
        MBN_DATA_CLASS_EDGE	= 0x2,
        MBN_DATA_CLASS_UMTS	= 0x4,
        MBN_DATA_CLASS_HSDPA	= 0x8,
        MBN_DATA_CLASS_HSUPA	= 0x10,
        MBN_DATA_CLASS_LTE	= 0x20,
        MBN_DATA_CLASS_5G_NSA	= 0x40,
        MBN_DATA_CLASS_5G_SA	= 0x80,
        MBN_DATA_CLASS_1XRTT	= 0x10000,
        MBN_DATA_CLASS_1XEVDO	= 0x20000,
        MBN_DATA_CLASS_1XEVDO_REVA	= 0x40000,
        MBN_DATA_CLASS_1XEVDV	= 0x80000,
        MBN_DATA_CLASS_3XRTT	= 0x100000,
        MBN_DATA_CLASS_1XEVDO_REVB	= 0x200000,
        MBN_DATA_CLASS_UMB	= 0x400000,
        MBN_DATA_CLASS_CUSTOM	= 0x80000000
    } 	MBN_DATA_CLASS;

typedef /* [helpstring][v1_enum][version][uuid] */  DECLSPEC_UUID("2308C1F7-BDC8-11DC-A8A8-001321F1405F") 
enum MBN_CTRL_CAPS
    {
        MBN_CTRL_CAPS_NONE	= 0,
        MBN_CTRL_CAPS_REG_MANUAL	= 0x1,
        MBN_CTRL_CAPS_HW_RADIO_SWITCH	= 0x2,
        MBN_CTRL_CAPS_CDMA_MOBILE_IP	= 0x4,
        MBN_CTRL_CAPS_CDMA_SIMPLE_IP	= 0x8,
        MBN_CTRL_CAPS_PROTECT_UNIQUEID	= 0x10,
        MBN_CTRL_CAPS_MODEL_MULTI_CARRIER	= 0x20,
        MBN_CTRL_CAPS_USSD	= 0x40,
        MBN_CTRL_CAPS_MULTI_MODE	= 0x80
    } 	MBN_CTRL_CAPS;

typedef /* [helpstring][v1_enum][version][uuid] */  DECLSPEC_UUID("E4096459-BDC8-11DC-A8A8-001321F1405F") 
enum MBN_SMS_CAPS
    {
        MBN_SMS_CAPS_NONE	= 0,
        MBN_SMS_CAPS_PDU_RECEIVE	= 0x1,
        MBN_SMS_CAPS_PDU_SEND	= 0x2,
        MBN_SMS_CAPS_TEXT_RECEIVE	= 0x4,
        MBN_SMS_CAPS_TEXT_SEND	= 0x8
    } 	MBN_SMS_CAPS;

typedef /* [helpstring][v1_enum][version][uuid] */  DECLSPEC_UUID("5DB6A98B-BDC5-11DC-A8A8-001321F1405F") 
enum MBN_BAND_CLASS
    {
        MBN_BAND_CLASS_NONE	= 0,
        MBN_BAND_CLASS_0	= 0x1,
        MBN_BAND_CLASS_I	= 0x2,
        MBN_BAND_CLASS_II	= 0x4,
        MBN_BAND_CLASS_III	= 0x8,
        MBN_BAND_CLASS_IV	= 0x10,
        MBN_BAND_CLASS_V	= 0x20,
        MBN_BAND_CLASS_VI	= 0x40,
        MBN_BAND_CLASS_VII	= 0x80,
        MBN_BAND_CLASS_VIII	= 0x100,
        MBN_BAND_CLASS_IX	= 0x200,
        MBN_BAND_CLASS_X	= 0x400,
        MBN_BAND_CLASS_XI	= 0x800,
        MBN_BAND_CLASS_XII	= 0x1000,
        MBN_BAND_CLASS_XIII	= 0x2000,
        MBN_BAND_CLASS_XIV	= 0x4000,
        MBN_BAND_CLASS_XV	= 0x8000,
        MBN_BAND_CLASS_XVI	= 0x10000,
        MBN_BAND_CLASS_XVII	= 0x20000,
        MBN_BAND_CLASS_CUSTOM	= 0x80000000
    } 	MBN_BAND_CLASS;

typedef /* [uuid] */  DECLSPEC_UUID("CD1A4B17-BDC9-11DC-A8A8-001321F1405F") struct MBN_INTERFACE_CAPS
    {
    MBN_CELLULAR_CLASS cellularClass;
    MBN_VOICE_CLASS voiceClass;
    ULONG dataClass;
    BSTR customDataClass;
    ULONG gsmBandClass;
    ULONG cdmaBandClass;
    BSTR customBandClass;
    ULONG smsCaps;
    ULONG controlCaps;
    BSTR deviceID;
    BSTR manufacturer;
    BSTR model;
    BSTR firmwareInfo;
    } 	MBN_INTERFACE_CAPS;

typedef /* [uuid] */  DECLSPEC_UUID("DCBBBAB6-9005-4BBB-AAEE-338E368AF6FA") struct MBN_PROVIDER
    {
    BSTR providerID;
    ULONG providerState;
    BSTR providerName;
    ULONG dataClass;
    } 	MBN_PROVIDER;

typedef /* [uuid] */  DECLSPEC_UUID("9AA005AE-540B-46F1-9244-8826D188F821") struct MBN_PROVIDER2
    {
    MBN_PROVIDER provider;
    MBN_CELLULAR_CLASS cellularClass;
    ULONG signalStrength;
    ULONG signalError;
    } 	MBN_PROVIDER2;

typedef /* [helpstring][v1_enum][version][uuid] */  DECLSPEC_UUID("DCBBBAB6-6003-4BBB-AAEE-338E368AF6FA") 
enum MBN_READY_STATE
    {
        MBN_READY_STATE_OFF	= 0,
        MBN_READY_STATE_INITIALIZED	= ( MBN_READY_STATE_OFF + 1 ) ,
        MBN_READY_STATE_SIM_NOT_INSERTED	= ( MBN_READY_STATE_INITIALIZED + 1 ) ,
        MBN_READY_STATE_BAD_SIM	= ( MBN_READY_STATE_SIM_NOT_INSERTED + 1 ) ,
        MBN_READY_STATE_FAILURE	= ( MBN_READY_STATE_BAD_SIM + 1 ) ,
        MBN_READY_STATE_NOT_ACTIVATED	= ( MBN_READY_STATE_FAILURE + 1 ) ,
        MBN_READY_STATE_DEVICE_LOCKED	= ( MBN_READY_STATE_NOT_ACTIVATED + 1 ) ,
        MBN_READY_STATE_DEVICE_BLOCKED	= ( MBN_READY_STATE_DEVICE_LOCKED + 1 ) ,
        MBN_READY_STATE_NO_ESIM_PROFILE	= ( MBN_READY_STATE_DEVICE_BLOCKED + 1 ) 
    } 	MBN_READY_STATE;

typedef /* [helpstring][v1_enum][version][uuid] */  DECLSPEC_UUID("EFB7C00D-BDCD-11DC-A8A8-001321F1405F") 
enum MBN_ACTIVATION_STATE
    {
        MBN_ACTIVATION_STATE_NONE	= 0,
        MBN_ACTIVATION_STATE_ACTIVATED	= ( MBN_ACTIVATION_STATE_NONE + 1 ) ,
        MBN_ACTIVATION_STATE_ACTIVATING	= ( MBN_ACTIVATION_STATE_ACTIVATED + 1 ) ,
        MBN_ACTIVATION_STATE_DEACTIVATED	= ( MBN_ACTIVATION_STATE_ACTIVATING + 1 ) ,
        MBN_ACTIVATION_STATE_DEACTIVATING	= ( MBN_ACTIVATION_STATE_DEACTIVATED + 1 ) 
    } 	MBN_ACTIVATION_STATE;

typedef /* [helpstring][v1_enum][version][uuid] */  DECLSPEC_UUID("F601E1EB-BDCD-11DC-A8A8-001321F1405F") 
enum MBN_CONNECTION_MODE
    {
        MBN_CONNECTION_MODE_PROFILE	= 0,
        MBN_CONNECTION_MODE_TMP_PROFILE	= ( MBN_CONNECTION_MODE_PROFILE + 1 ) 
    } 	MBN_CONNECTION_MODE;

typedef /* [helpstring][v1_enum][version][uuid] */  DECLSPEC_UUID("6D8846E5-BDD1-11DC-A8A8-001321F1405F") 
enum MBN_VOICE_CALL_STATE
    {
        MBN_VOICE_CALL_STATE_NONE	= 0,
        MBN_VOICE_CALL_STATE_IN_PROGRESS	= ( MBN_VOICE_CALL_STATE_NONE + 1 ) ,
        MBN_VOICE_CALL_STATE_HANGUP	= ( MBN_VOICE_CALL_STATE_IN_PROGRESS + 1 ) 
    } 	MBN_VOICE_CALL_STATE;

typedef /* [helpstring][v1_enum][version][uuid] */  DECLSPEC_UUID("D5F1726B-BDCE-11DC-A8A8-001321F1405F") 
enum MBN_REGISTRATION_CONSTANTS
    {
        MBN_ROAMTEXT_LEN	= 64,
        MBN_CDMA_DEFAULT_PROVIDER_ID	= 0
    } 	MBN_REGISTRATION_CONSTANTS;

typedef /* [helpstring][v1_enum][version][uuid] */  DECLSPEC_UUID("DCBBBAB6-6009-4BBB-AAEE-338E368AF6FA") 
enum MBN_REGISTER_STATE
    {
        MBN_REGISTER_STATE_NONE	= 0,
        MBN_REGISTER_STATE_DEREGISTERED	= ( MBN_REGISTER_STATE_NONE + 1 ) ,
        MBN_REGISTER_STATE_SEARCHING	= ( MBN_REGISTER_STATE_DEREGISTERED + 1 ) ,
        MBN_REGISTER_STATE_HOME	= ( MBN_REGISTER_STATE_SEARCHING + 1 ) ,
        MBN_REGISTER_STATE_ROAMING	= ( MBN_REGISTER_STATE_HOME + 1 ) ,
        MBN_REGISTER_STATE_PARTNER	= ( MBN_REGISTER_STATE_ROAMING + 1 ) ,
        MBN_REGISTER_STATE_DENIED	= ( MBN_REGISTER_STATE_PARTNER + 1 ) 
    } 	MBN_REGISTER_STATE;

typedef /* [helpstring][v1_enum][version][uuid] */  DECLSPEC_UUID("D7F73F35-BDCD-11DC-A8A8-001321F1405F") 
enum MBN_REGISTER_MODE
    {
        MBN_REGISTER_MODE_NONE	= 0,
        MBN_REGISTER_MODE_AUTOMATIC	= ( MBN_REGISTER_MODE_NONE + 1 ) ,
        MBN_REGISTER_MODE_MANUAL	= ( MBN_REGISTER_MODE_AUTOMATIC + 1 ) 
    } 	MBN_REGISTER_MODE;

typedef /* [helpstring][v1_enum][version][uuid] */  DECLSPEC_UUID("C75E76F5-BDCD-11DC-A8A8-001321F1405F") 
enum MBN_PIN_CONSTANTS
    {
        MBN_ATTEMPTS_REMAINING_UNKNOWN	= 0xffffffff,
        MBN_PIN_LENGTH_UNKNOWN	= 0xffffffff
    } 	MBN_PIN_CONSTANTS;

typedef /* [helpstring][v1_enum][version][uuid] */  DECLSPEC_UUID("D11BD29D-BDCD-11DC-A8A8-001321F1405F") 
enum MBN_PIN_STATE
    {
        MBN_PIN_STATE_NONE	= 0,
        MBN_PIN_STATE_ENTER	= ( MBN_PIN_STATE_NONE + 1 ) ,
        MBN_PIN_STATE_UNBLOCK	= ( MBN_PIN_STATE_ENTER + 1 ) 
    } 	MBN_PIN_STATE;

typedef /* [helpstring][v1_enum][version][uuid] */  DECLSPEC_UUID("DCBBBAB6-6005-4BBB-AAEE-338E368AF6FA") 
enum MBN_PIN_TYPE
    {
        MBN_PIN_TYPE_NONE	= 0,
        MBN_PIN_TYPE_CUSTOM	= ( MBN_PIN_TYPE_NONE + 1 ) ,
        MBN_PIN_TYPE_PIN1	= ( MBN_PIN_TYPE_CUSTOM + 1 ) ,
        MBN_PIN_TYPE_PIN2	= ( MBN_PIN_TYPE_PIN1 + 1 ) ,
        MBN_PIN_TYPE_DEVICE_SIM_PIN	= ( MBN_PIN_TYPE_PIN2 + 1 ) ,
        MBN_PIN_TYPE_DEVICE_FIRST_SIM_PIN	= ( MBN_PIN_TYPE_DEVICE_SIM_PIN + 1 ) ,
        MBN_PIN_TYPE_NETWORK_PIN	= ( MBN_PIN_TYPE_DEVICE_FIRST_SIM_PIN + 1 ) ,
        MBN_PIN_TYPE_NETWORK_SUBSET_PIN	= ( MBN_PIN_TYPE_NETWORK_PIN + 1 ) ,
        MBN_PIN_TYPE_SVC_PROVIDER_PIN	= ( MBN_PIN_TYPE_NETWORK_SUBSET_PIN + 1 ) ,
        MBN_PIN_TYPE_CORPORATE_PIN	= ( MBN_PIN_TYPE_SVC_PROVIDER_PIN + 1 ) ,
        MBN_PIN_TYPE_SUBSIDY_LOCK	= ( MBN_PIN_TYPE_CORPORATE_PIN + 1 ) 
    } 	MBN_PIN_TYPE;

typedef /* [uuid] */  DECLSPEC_UUID("DCBBBAB6-9006-4BBB-AAEE-338E368AF6FA") struct MBN_PIN_INFO
    {
    MBN_PIN_STATE pinState;
    MBN_PIN_TYPE pinType;
    ULONG attemptsRemaining;
    } 	MBN_PIN_INFO;

typedef /* [helpstring][v1_enum][version][uuid] */  DECLSPEC_UUID("BD8A2871-BDCD-11DC-A8A8-001321F1405F") 
enum MBN_PIN_MODE
    {
        MBN_PIN_MODE_ENABLED	= 1,
        MBN_PIN_MODE_DISABLED	= ( MBN_PIN_MODE_ENABLED + 1 ) 
    } 	MBN_PIN_MODE;

typedef /* [helpstring][v1_enum][version][uuid] */  DECLSPEC_UUID("C2A93665-BDCD-11DC-A8A8-001321F1405F") 
enum MBN_PIN_FORMAT
    {
        MBN_PIN_FORMAT_NONE	= 0,
        MBN_PIN_FORMAT_NUMERIC	= ( MBN_PIN_FORMAT_NONE + 1 ) ,
        MBN_PIN_FORMAT_ALPHANUMERIC	= ( MBN_PIN_FORMAT_NUMERIC + 1 ) 
    } 	MBN_PIN_FORMAT;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
typedef /* [helpstring][v1_enum][version][uuid] */  DECLSPEC_UUID("0E62803F-BDD0-11DC-A8A8-001321F1405F") 
enum MBN_CONTEXT_CONSTANTS
    {
        MBN_ACCESSSTRING_LEN	= 100,
        MBN_USERNAME_LEN	= 255,
        MBN_PASSWORD_LEN	= 255,
        MBN_CONTEXT_ID_APPEND	= 0xffffffff
    } 	MBN_CONTEXT_CONSTANTS;

typedef /* [helpstring][v1_enum][version][uuid] */  DECLSPEC_UUID("E24B42EF-BDCD-11DC-A8A8-001321F1405F") 
enum MBN_AUTH_PROTOCOL
    {
        MBN_AUTH_PROTOCOL_NONE	= 0,
        MBN_AUTH_PROTOCOL_PAP	= ( MBN_AUTH_PROTOCOL_NONE + 1 ) ,
        MBN_AUTH_PROTOCOL_CHAP	= ( MBN_AUTH_PROTOCOL_PAP + 1 ) ,
        MBN_AUTH_PROTOCOL_MSCHAPV2	= ( MBN_AUTH_PROTOCOL_CHAP + 1 ) 
    } 	MBN_AUTH_PROTOCOL;

typedef /* [helpstring][v1_enum][version][uuid] */  DECLSPEC_UUID("E6A360B9-BDCD-11DC-A8A8-001321F1405F") 
enum MBN_COMPRESSION
    {
        MBN_COMPRESSION_NONE	= 0,
        MBN_COMPRESSION_ENABLE	= ( MBN_COMPRESSION_NONE + 1 ) 
    } 	MBN_COMPRESSION;

typedef /* [helpstring][v1_enum][version][uuid] */  DECLSPEC_UUID("EB4731EB-BDCD-11DC-A8A8-001321F1405F") 
enum MBN_CONTEXT_TYPE
    {
        MBN_CONTEXT_TYPE_NONE	= 0,
        MBN_CONTEXT_TYPE_INTERNET	= ( MBN_CONTEXT_TYPE_NONE + 1 ) ,
        MBN_CONTEXT_TYPE_VPN	= ( MBN_CONTEXT_TYPE_INTERNET + 1 ) ,
        MBN_CONTEXT_TYPE_VOICE	= ( MBN_CONTEXT_TYPE_VPN + 1 ) ,
        MBN_CONTEXT_TYPE_VIDEO_SHARE	= ( MBN_CONTEXT_TYPE_VOICE + 1 ) ,
        MBN_CONTEXT_TYPE_CUSTOM	= ( MBN_CONTEXT_TYPE_VIDEO_SHARE + 1 ) ,
        MBN_CONTEXT_TYPE_PURCHASE	= ( MBN_CONTEXT_TYPE_CUSTOM + 1 ) 
    } 	MBN_CONTEXT_TYPE;

typedef /* [uuid] */  DECLSPEC_UUID("FE1F7B6F-BDCD-11DC-A8A8-001321F1405F") struct MBN_CONTEXT
    {
    ULONG contextID;
    MBN_CONTEXT_TYPE contextType;
    BSTR accessString;
    BSTR userName;
    BSTR password;
    MBN_COMPRESSION compression;
    MBN_AUTH_PROTOCOL authType;
    } 	MBN_CONTEXT;

typedef /* [helpstring][v1_enum][version][uuid] */  DECLSPEC_UUID("130C65D3-BDD3-11DC-A8A8-001321F1405F") 
enum MBN_SMS_CONSTANTS
    {
        MBN_MESSAGE_INDEX_NONE	= 0,
        MBN_CDMA_SHORT_MSG_SIZE_UNKNOWN	= 0,
        MBN_CDMA_SHORT_MSG_SIZE_MAX	= 160
    } 	WWAEXT_SMS_CONSTANTS;

typedef /* [helpstring][v1_enum][version][uuid] */  DECLSPEC_UUID("FC208FC1-BDE5-11DC-A8A8-001321F1405F") 
enum MBN_MSG_STATUS
    {
        MBN_MSG_STATUS_NEW	= 0,
        MBN_MSG_STATUS_OLD	= ( MBN_MSG_STATUS_NEW + 1 ) ,
        MBN_MSG_STATUS_DRAFT	= ( MBN_MSG_STATUS_OLD + 1 ) ,
        MBN_MSG_STATUS_SENT	= ( MBN_MSG_STATUS_DRAFT + 1 ) 
    } 	MBN_MSG_STATUS;

typedef /* [helpstring][v1_enum][version][uuid] */  DECLSPEC_UUID("29912B29-BDD4-11DC-A8A8-001321F1405F") 
enum MBN_SMS_CDMA_LANG
    {
        MBN_SMS_CDMA_LANG_NONE	= 0,
        MBN_SMS_CDMA_LANG_ENGLISH	= ( MBN_SMS_CDMA_LANG_NONE + 1 ) ,
        MBN_SMS_CDMA_LANG_FRENCH	= ( MBN_SMS_CDMA_LANG_ENGLISH + 1 ) ,
        MBN_SMS_CDMA_LANG_SPANISH	= ( MBN_SMS_CDMA_LANG_FRENCH + 1 ) ,
        MBN_SMS_CDMA_LANG_JAPANESE	= ( MBN_SMS_CDMA_LANG_SPANISH + 1 ) ,
        MBN_SMS_CDMA_LANG_KOREAN	= ( MBN_SMS_CDMA_LANG_JAPANESE + 1 ) ,
        MBN_SMS_CDMA_LANG_CHINESE	= ( MBN_SMS_CDMA_LANG_KOREAN + 1 ) ,
        MBN_SMS_CDMA_LANG_HEBREW	= ( MBN_SMS_CDMA_LANG_CHINESE + 1 ) 
    } 	MBN_SMS_CDMA_LANG;

typedef /* [helpstring][v1_enum][version][uuid] */  DECLSPEC_UUID("7F8E74CB-BDD4-11DC-A8A8-001321F1405F") 
enum MBN_SMS_CDMA_ENCODING
    {
        MBN_SMS_CDMA_ENCODING_OCTET	= 0,
        MBN_SMS_CDMA_ENCODING_EPM	= ( MBN_SMS_CDMA_ENCODING_OCTET + 1 ) ,
        MBN_SMS_CDMA_ENCODING_7BIT_ASCII	= ( MBN_SMS_CDMA_ENCODING_EPM + 1 ) ,
        MBN_SMS_CDMA_ENCODING_IA5	= ( MBN_SMS_CDMA_ENCODING_7BIT_ASCII + 1 ) ,
        MBN_SMS_CDMA_ENCODING_UNICODE	= ( MBN_SMS_CDMA_ENCODING_IA5 + 1 ) ,
        MBN_SMS_CDMA_ENCODING_SHIFT_JIS	= ( MBN_SMS_CDMA_ENCODING_UNICODE + 1 ) ,
        MBN_SMS_CDMA_ENCODING_KOREAN	= ( MBN_SMS_CDMA_ENCODING_SHIFT_JIS + 1 ) ,
        MBN_SMS_CDMA_ENCODING_LATIN_HEBREW	= ( MBN_SMS_CDMA_ENCODING_KOREAN + 1 ) ,
        MBN_SMS_CDMA_ENCODING_LATIN	= ( MBN_SMS_CDMA_ENCODING_LATIN_HEBREW + 1 ) ,
        MBN_SMS_CDMA_ENCODING_GSM_7BIT	= ( MBN_SMS_CDMA_ENCODING_LATIN + 1 ) 
    } 	MBN_SMS_CDMA_ENCODING;

typedef /* [helpstring][v1_enum][version][uuid] */  DECLSPEC_UUID("0D42B514-BDDC-11DC-A8A8-001321F1405F") 
enum MBN_SMS_FLAG
    {
        MBN_SMS_FLAG_ALL	= 0,
        MBN_SMS_FLAG_INDEX	= ( MBN_SMS_FLAG_ALL + 1 ) ,
        MBN_SMS_FLAG_NEW	= ( MBN_SMS_FLAG_INDEX + 1 ) ,
        MBN_SMS_FLAG_OLD	= ( MBN_SMS_FLAG_NEW + 1 ) ,
        MBN_SMS_FLAG_SENT	= ( MBN_SMS_FLAG_OLD + 1 ) ,
        MBN_SMS_FLAG_DRAFT	= ( MBN_SMS_FLAG_SENT + 1 ) 
    } 	MBN_SMS_FLAG;

typedef /* [uuid] */  DECLSPEC_UUID("406BFD60-BDDC-11DC-A8A8-001321F1405F") struct MBN_SMS_FILTER
    {
    MBN_SMS_FLAG flag;
    ULONG messageIndex;
    } 	MBN_SMS_FILTER;

typedef /* [helpstring][v1_enum][version][uuid] */  DECLSPEC_UUID("8098009D-BDDC-11DC-A8A8-001321F1405F") 
enum MBN_SMS_STATUS_FLAG
    {
        MBN_SMS_FLAG_NONE	= 0,
        MBN_SMS_FLAG_MESSAGE_STORE_FULL	= 0x1,
        MBN_SMS_FLAG_NEW_MESSAGE	= 0x2
    } 	MBN_SMS_STATUS_FLAG;

typedef /* [uuid] */  DECLSPEC_UUID("1F6E9CA3-BDE6-11DC-A8A8-001321F1405F") struct MBN_SMS_STATUS_INFO
    {
    ULONG flag;
    ULONG messageIndex;
    } 	MBN_SMS_STATUS_INFO;

typedef /* [helpstring][v1_enum][version][uuid] */  DECLSPEC_UUID("4B0FE227-3709-41e2-8A78-E98C0CD0CB09") 
enum MBN_SMS_FORMAT
    {
        MBN_SMS_FORMAT_NONE	= 0,
        MBN_SMS_FORMAT_PDU	= 0x1,
        MBN_SMS_FORMAT_TEXT	= 0x2
    } 	MBN_SMS_FORMAT;

typedef /* [helpstring][v1_enum][version][uuid] */  DECLSPEC_UUID("532A3FE4-BDE6-11DC-A8A8-001321F1405F") 
enum MBN_RADIO
    {
        MBN_RADIO_OFF	= 0,
        MBN_RADIO_ON	= ( MBN_RADIO_OFF + 1 ) 
    } 	MBN_RADIO;

typedef /* [helpstring][v1_enum][version][uuid] */  DECLSPEC_UUID("0D17D0A2-B2AA-4B34-8655-C2F7F9217473") 
enum MBN_DEVICE_SERVICE_SESSIONS_STATE
    {
        MBN_DEVICE_SERVICE_SESSIONS_RESTORED	= 0
    } 	MBN_DEVICE_SERVICE_SESSIONS_STATE;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
typedef /* [uuid] */  DECLSPEC_UUID("60436154-3466-48A4-82E2-9A461527A8C5") struct MBN_DEVICE_SERVICE
    {
    BSTR deviceServiceID;
    VARIANT_BOOL dataWriteSupported;
    VARIANT_BOOL dataReadSupported;
    } 	MBN_DEVICE_SERVICE;

typedef /* [helpstring][v1_enum][version][uuid] */  DECLSPEC_UUID("386077A0-275B-45B6-9B32-C343A6749E86") 
enum MBN_DEVICE_SERVICES_INTERFACE_STATE
    {
        MBN_DEVICE_SERVICES_CAPABLE_INTERFACE_ARRIVAL	= 0,
        MBN_DEVICE_SERVICES_CAPABLE_INTERFACE_REMOVAL	= ( MBN_DEVICE_SERVICES_CAPABLE_INTERFACE_ARRIVAL + 1 ) 
    } 	MBN_DEVICE_SERVICES_INTERFACE_STATE;



extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0001_v0_0_s_ifspec;

#ifndef __IMbnConnection_INTERFACE_DEFINED__
#define __IMbnConnection_INTERFACE_DEFINED__

/* interface IMbnConnection */
/* [helpstring][uuid][oleautomation][nonextensible][object] */ 


EXTERN_C const IID IID_IMbnConnection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DCBBBAB6-200D-4BBB-AAEE-338E368AF6FA")
    IMbnConnection : public IUnknown
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ConnectionID( 
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *ConnectionID) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_InterfaceID( 
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *InterfaceID) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Connect( 
            /* [in] */ MBN_CONNECTION_MODE connectionMode,
            /* [string][in] */ __RPC__in_string LPCWSTR strProfile,
            /* [out] */ __RPC__out ULONG *requestID) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Disconnect( 
            /* [out] */ __RPC__out ULONG *requestID) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetConnectionState( 
            /* [ref][out] */ __RPC__out MBN_ACTIVATION_STATE *ConnectionState,
            /* [ref][out] */ __RPC__deref_out_opt BSTR *ProfileName) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetVoiceCallState( 
            /* [retval][ref][out] */ __RPC__out MBN_VOICE_CALL_STATE *voiceCallState) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetActivationNetworkError( 
            /* [retval][ref][out] */ __RPC__out ULONG *networkError) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMbnConnectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMbnConnection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMbnConnection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMbnConnection * This);
        
        DECLSPEC_XFGVIRT(IMbnConnection, get_ConnectionID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ConnectionID )( 
            __RPC__in IMbnConnection * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *ConnectionID);
        
        DECLSPEC_XFGVIRT(IMbnConnection, get_InterfaceID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_InterfaceID )( 
            __RPC__in IMbnConnection * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *InterfaceID);
        
        DECLSPEC_XFGVIRT(IMbnConnection, Connect)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Connect )( 
            __RPC__in IMbnConnection * This,
            /* [in] */ MBN_CONNECTION_MODE connectionMode,
            /* [string][in] */ __RPC__in_string LPCWSTR strProfile,
            /* [out] */ __RPC__out ULONG *requestID);
        
        DECLSPEC_XFGVIRT(IMbnConnection, Disconnect)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Disconnect )( 
            __RPC__in IMbnConnection * This,
            /* [out] */ __RPC__out ULONG *requestID);
        
        DECLSPEC_XFGVIRT(IMbnConnection, GetConnectionState)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetConnectionState )( 
            __RPC__in IMbnConnection * This,
            /* [ref][out] */ __RPC__out MBN_ACTIVATION_STATE *ConnectionState,
            /* [ref][out] */ __RPC__deref_out_opt BSTR *ProfileName);
        
        DECLSPEC_XFGVIRT(IMbnConnection, GetVoiceCallState)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetVoiceCallState )( 
            __RPC__in IMbnConnection * This,
            /* [retval][ref][out] */ __RPC__out MBN_VOICE_CALL_STATE *voiceCallState);
        
        DECLSPEC_XFGVIRT(IMbnConnection, GetActivationNetworkError)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetActivationNetworkError )( 
            __RPC__in IMbnConnection * This,
            /* [retval][ref][out] */ __RPC__out ULONG *networkError);
        
        END_INTERFACE
    } IMbnConnectionVtbl;

    interface IMbnConnection
    {
        CONST_VTBL struct IMbnConnectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMbnConnection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMbnConnection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMbnConnection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMbnConnection_get_ConnectionID(This,ConnectionID)	\
    ( (This)->lpVtbl -> get_ConnectionID(This,ConnectionID) ) 

#define IMbnConnection_get_InterfaceID(This,InterfaceID)	\
    ( (This)->lpVtbl -> get_InterfaceID(This,InterfaceID) ) 

#define IMbnConnection_Connect(This,connectionMode,strProfile,requestID)	\
    ( (This)->lpVtbl -> Connect(This,connectionMode,strProfile,requestID) ) 

#define IMbnConnection_Disconnect(This,requestID)	\
    ( (This)->lpVtbl -> Disconnect(This,requestID) ) 

#define IMbnConnection_GetConnectionState(This,ConnectionState,ProfileName)	\
    ( (This)->lpVtbl -> GetConnectionState(This,ConnectionState,ProfileName) ) 

#define IMbnConnection_GetVoiceCallState(This,voiceCallState)	\
    ( (This)->lpVtbl -> GetVoiceCallState(This,voiceCallState) ) 

#define IMbnConnection_GetActivationNetworkError(This,networkError)	\
    ( (This)->lpVtbl -> GetActivationNetworkError(This,networkError) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMbnConnection_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mbnapi_0000_0002 */
/* [local] */ 

#pragma deprecated(IMbnConnection) // Replaced by WinRT API in Windows.Networking.Connectivity and Windows.Networking.NetworkOperators namespaces


extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0002_v0_0_s_ifspec;

#ifndef __IMbnConnectionEvents_INTERFACE_DEFINED__
#define __IMbnConnectionEvents_INTERFACE_DEFINED__

/* interface IMbnConnectionEvents */
/* [helpstring][uuid][oleautomation][nonextensible][object] */ 


EXTERN_C const IID IID_IMbnConnectionEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DCBBBAB6-200E-4BBB-AAEE-338E368AF6FA")
    IMbnConnectionEvents : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnConnectComplete( 
            /* [in] */ __RPC__in_opt IMbnConnection *newConnection,
            /* [in] */ ULONG requestID,
            /* [in] */ HRESULT status) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnDisconnectComplete( 
            /* [in] */ __RPC__in_opt IMbnConnection *newConnection,
            /* [in] */ ULONG requestID,
            /* [in] */ HRESULT status) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnConnectStateChange( 
            /* [in] */ __RPC__in_opt IMbnConnection *newConnection) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnVoiceCallStateChange( 
            /* [in] */ __RPC__in_opt IMbnConnection *newConnection) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMbnConnectionEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMbnConnectionEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMbnConnectionEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMbnConnectionEvents * This);
        
        DECLSPEC_XFGVIRT(IMbnConnectionEvents, OnConnectComplete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnConnectComplete )( 
            __RPC__in IMbnConnectionEvents * This,
            /* [in] */ __RPC__in_opt IMbnConnection *newConnection,
            /* [in] */ ULONG requestID,
            /* [in] */ HRESULT status);
        
        DECLSPEC_XFGVIRT(IMbnConnectionEvents, OnDisconnectComplete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnDisconnectComplete )( 
            __RPC__in IMbnConnectionEvents * This,
            /* [in] */ __RPC__in_opt IMbnConnection *newConnection,
            /* [in] */ ULONG requestID,
            /* [in] */ HRESULT status);
        
        DECLSPEC_XFGVIRT(IMbnConnectionEvents, OnConnectStateChange)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnConnectStateChange )( 
            __RPC__in IMbnConnectionEvents * This,
            /* [in] */ __RPC__in_opt IMbnConnection *newConnection);
        
        DECLSPEC_XFGVIRT(IMbnConnectionEvents, OnVoiceCallStateChange)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnVoiceCallStateChange )( 
            __RPC__in IMbnConnectionEvents * This,
            /* [in] */ __RPC__in_opt IMbnConnection *newConnection);
        
        END_INTERFACE
    } IMbnConnectionEventsVtbl;

    interface IMbnConnectionEvents
    {
        CONST_VTBL struct IMbnConnectionEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMbnConnectionEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMbnConnectionEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMbnConnectionEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMbnConnectionEvents_OnConnectComplete(This,newConnection,requestID,status)	\
    ( (This)->lpVtbl -> OnConnectComplete(This,newConnection,requestID,status) ) 

#define IMbnConnectionEvents_OnDisconnectComplete(This,newConnection,requestID,status)	\
    ( (This)->lpVtbl -> OnDisconnectComplete(This,newConnection,requestID,status) ) 

#define IMbnConnectionEvents_OnConnectStateChange(This,newConnection)	\
    ( (This)->lpVtbl -> OnConnectStateChange(This,newConnection) ) 

#define IMbnConnectionEvents_OnVoiceCallStateChange(This,newConnection)	\
    ( (This)->lpVtbl -> OnVoiceCallStateChange(This,newConnection) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMbnConnectionEvents_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mbnapi_0000_0003 */
/* [local] */ 

#pragma deprecated(IMbnConnectionEvents) // Replaced by WinRT API in Windows.Networking.Connectivity and Windows.Networking.NetworkOperators namespaces


extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0003_v0_0_s_ifspec;

#ifndef __IMbnInterface_INTERFACE_DEFINED__
#define __IMbnInterface_INTERFACE_DEFINED__

/* interface IMbnInterface */
/* [helpstring][uuid][oleautomation][nonextensible][object] */ 


EXTERN_C const IID IID_IMbnInterface;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DCBBBAB6-2001-4BBB-AAEE-338E368AF6FA")
    IMbnInterface : public IUnknown
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_InterfaceID( 
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *InterfaceID) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetInterfaceCapability( 
            /* [retval][ref][out] */ __RPC__out MBN_INTERFACE_CAPS *interfaceCaps) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetSubscriberInformation( 
            /* [retval][out] */ __RPC__deref_out_opt IMbnSubscriberInformation **subscriberInformation) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetReadyState( 
            /* [retval][ref][out] */ __RPC__out MBN_READY_STATE *readyState) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE InEmergencyMode( 
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *emergencyMode) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetHomeProvider( 
            /* [retval][ref][out] */ __RPC__out MBN_PROVIDER *homeProvider) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetPreferredProviders( 
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *preferredProviders) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetPreferredProviders( 
            /* [in] */ __RPC__in SAFEARRAY * preferredProviders,
            /* [out] */ __RPC__out ULONG *requestID) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetVisibleProviders( 
            /* [out] */ __RPC__out ULONG *age,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *visibleProviders) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ScanNetwork( 
            /* [out] */ __RPC__out ULONG *requestID) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetConnection( 
            /* [retval][out] */ __RPC__deref_out_opt IMbnConnection **mbnConnection) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMbnInterfaceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMbnInterface * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMbnInterface * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMbnInterface * This);
        
        DECLSPEC_XFGVIRT(IMbnInterface, get_InterfaceID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_InterfaceID )( 
            __RPC__in IMbnInterface * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *InterfaceID);
        
        DECLSPEC_XFGVIRT(IMbnInterface, GetInterfaceCapability)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetInterfaceCapability )( 
            __RPC__in IMbnInterface * This,
            /* [retval][ref][out] */ __RPC__out MBN_INTERFACE_CAPS *interfaceCaps);
        
        DECLSPEC_XFGVIRT(IMbnInterface, GetSubscriberInformation)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetSubscriberInformation )( 
            __RPC__in IMbnInterface * This,
            /* [retval][out] */ __RPC__deref_out_opt IMbnSubscriberInformation **subscriberInformation);
        
        DECLSPEC_XFGVIRT(IMbnInterface, GetReadyState)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetReadyState )( 
            __RPC__in IMbnInterface * This,
            /* [retval][ref][out] */ __RPC__out MBN_READY_STATE *readyState);
        
        DECLSPEC_XFGVIRT(IMbnInterface, InEmergencyMode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *InEmergencyMode )( 
            __RPC__in IMbnInterface * This,
            /* [retval][ref][out] */ __RPC__out VARIANT_BOOL *emergencyMode);
        
        DECLSPEC_XFGVIRT(IMbnInterface, GetHomeProvider)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetHomeProvider )( 
            __RPC__in IMbnInterface * This,
            /* [retval][ref][out] */ __RPC__out MBN_PROVIDER *homeProvider);
        
        DECLSPEC_XFGVIRT(IMbnInterface, GetPreferredProviders)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetPreferredProviders )( 
            __RPC__in IMbnInterface * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *preferredProviders);
        
        DECLSPEC_XFGVIRT(IMbnInterface, SetPreferredProviders)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetPreferredProviders )( 
            __RPC__in IMbnInterface * This,
            /* [in] */ __RPC__in SAFEARRAY * preferredProviders,
            /* [out] */ __RPC__out ULONG *requestID);
        
        DECLSPEC_XFGVIRT(IMbnInterface, GetVisibleProviders)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetVisibleProviders )( 
            __RPC__in IMbnInterface * This,
            /* [out] */ __RPC__out ULONG *age,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *visibleProviders);
        
        DECLSPEC_XFGVIRT(IMbnInterface, ScanNetwork)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ScanNetwork )( 
            __RPC__in IMbnInterface * This,
            /* [out] */ __RPC__out ULONG *requestID);
        
        DECLSPEC_XFGVIRT(IMbnInterface, GetConnection)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetConnection )( 
            __RPC__in IMbnInterface * This,
            /* [retval][out] */ __RPC__deref_out_opt IMbnConnection **mbnConnection);
        
        END_INTERFACE
    } IMbnInterfaceVtbl;

    interface IMbnInterface
    {
        CONST_VTBL struct IMbnInterfaceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMbnInterface_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMbnInterface_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMbnInterface_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMbnInterface_get_InterfaceID(This,InterfaceID)	\
    ( (This)->lpVtbl -> get_InterfaceID(This,InterfaceID) ) 

#define IMbnInterface_GetInterfaceCapability(This,interfaceCaps)	\
    ( (This)->lpVtbl -> GetInterfaceCapability(This,interfaceCaps) ) 

#define IMbnInterface_GetSubscriberInformation(This,subscriberInformation)	\
    ( (This)->lpVtbl -> GetSubscriberInformation(This,subscriberInformation) ) 

#define IMbnInterface_GetReadyState(This,readyState)	\
    ( (This)->lpVtbl -> GetReadyState(This,readyState) ) 

#define IMbnInterface_InEmergencyMode(This,emergencyMode)	\
    ( (This)->lpVtbl -> InEmergencyMode(This,emergencyMode) ) 

#define IMbnInterface_GetHomeProvider(This,homeProvider)	\
    ( (This)->lpVtbl -> GetHomeProvider(This,homeProvider) ) 

#define IMbnInterface_GetPreferredProviders(This,preferredProviders)	\
    ( (This)->lpVtbl -> GetPreferredProviders(This,preferredProviders) ) 

#define IMbnInterface_SetPreferredProviders(This,preferredProviders,requestID)	\
    ( (This)->lpVtbl -> SetPreferredProviders(This,preferredProviders,requestID) ) 

#define IMbnInterface_GetVisibleProviders(This,age,visibleProviders)	\
    ( (This)->lpVtbl -> GetVisibleProviders(This,age,visibleProviders) ) 

#define IMbnInterface_ScanNetwork(This,requestID)	\
    ( (This)->lpVtbl -> ScanNetwork(This,requestID) ) 

#define IMbnInterface_GetConnection(This,mbnConnection)	\
    ( (This)->lpVtbl -> GetConnection(This,mbnConnection) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMbnInterface_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mbnapi_0000_0004 */
/* [local] */ 

#pragma deprecated(IMbnInterface) // Replaced by WinRT API in Windows.Networking.Connectivity and Windows.Networking.NetworkOperators namespaces


extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0004_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0004_v0_0_s_ifspec;

#ifndef __IMbnInterfaceEvents_INTERFACE_DEFINED__
#define __IMbnInterfaceEvents_INTERFACE_DEFINED__

/* interface IMbnInterfaceEvents */
/* [helpstring][uuid][oleautomation][nonextensible][object] */ 


EXTERN_C const IID IID_IMbnInterfaceEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DCBBBAB6-2002-4BBB-AAEE-338E368AF6FA")
    IMbnInterfaceEvents : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnInterfaceCapabilityAvailable( 
            /* [in] */ __RPC__in_opt IMbnInterface *newInterface) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnSubscriberInformationChange( 
            /* [in] */ __RPC__in_opt IMbnInterface *newInterface) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnReadyStateChange( 
            /* [in] */ __RPC__in_opt IMbnInterface *newInterface) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnEmergencyModeChange( 
            /* [in] */ __RPC__in_opt IMbnInterface *newInterface) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnHomeProviderAvailable( 
            /* [in] */ __RPC__in_opt IMbnInterface *newInterface) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnPreferredProvidersChange( 
            /* [in] */ __RPC__in_opt IMbnInterface *newInterface) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnSetPreferredProvidersComplete( 
            /* [in] */ __RPC__in_opt IMbnInterface *newInterface,
            /* [in] */ ULONG requestID,
            /* [in] */ HRESULT status) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnScanNetworkComplete( 
            /* [in] */ __RPC__in_opt IMbnInterface *newInterface,
            /* [in] */ ULONG requestID,
            /* [in] */ HRESULT status) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMbnInterfaceEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMbnInterfaceEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMbnInterfaceEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMbnInterfaceEvents * This);
        
        DECLSPEC_XFGVIRT(IMbnInterfaceEvents, OnInterfaceCapabilityAvailable)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnInterfaceCapabilityAvailable )( 
            __RPC__in IMbnInterfaceEvents * This,
            /* [in] */ __RPC__in_opt IMbnInterface *newInterface);
        
        DECLSPEC_XFGVIRT(IMbnInterfaceEvents, OnSubscriberInformationChange)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnSubscriberInformationChange )( 
            __RPC__in IMbnInterfaceEvents * This,
            /* [in] */ __RPC__in_opt IMbnInterface *newInterface);
        
        DECLSPEC_XFGVIRT(IMbnInterfaceEvents, OnReadyStateChange)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnReadyStateChange )( 
            __RPC__in IMbnInterfaceEvents * This,
            /* [in] */ __RPC__in_opt IMbnInterface *newInterface);
        
        DECLSPEC_XFGVIRT(IMbnInterfaceEvents, OnEmergencyModeChange)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnEmergencyModeChange )( 
            __RPC__in IMbnInterfaceEvents * This,
            /* [in] */ __RPC__in_opt IMbnInterface *newInterface);
        
        DECLSPEC_XFGVIRT(IMbnInterfaceEvents, OnHomeProviderAvailable)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnHomeProviderAvailable )( 
            __RPC__in IMbnInterfaceEvents * This,
            /* [in] */ __RPC__in_opt IMbnInterface *newInterface);
        
        DECLSPEC_XFGVIRT(IMbnInterfaceEvents, OnPreferredProvidersChange)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnPreferredProvidersChange )( 
            __RPC__in IMbnInterfaceEvents * This,
            /* [in] */ __RPC__in_opt IMbnInterface *newInterface);
        
        DECLSPEC_XFGVIRT(IMbnInterfaceEvents, OnSetPreferredProvidersComplete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnSetPreferredProvidersComplete )( 
            __RPC__in IMbnInterfaceEvents * This,
            /* [in] */ __RPC__in_opt IMbnInterface *newInterface,
            /* [in] */ ULONG requestID,
            /* [in] */ HRESULT status);
        
        DECLSPEC_XFGVIRT(IMbnInterfaceEvents, OnScanNetworkComplete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnScanNetworkComplete )( 
            __RPC__in IMbnInterfaceEvents * This,
            /* [in] */ __RPC__in_opt IMbnInterface *newInterface,
            /* [in] */ ULONG requestID,
            /* [in] */ HRESULT status);
        
        END_INTERFACE
    } IMbnInterfaceEventsVtbl;

    interface IMbnInterfaceEvents
    {
        CONST_VTBL struct IMbnInterfaceEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMbnInterfaceEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMbnInterfaceEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMbnInterfaceEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMbnInterfaceEvents_OnInterfaceCapabilityAvailable(This,newInterface)	\
    ( (This)->lpVtbl -> OnInterfaceCapabilityAvailable(This,newInterface) ) 

#define IMbnInterfaceEvents_OnSubscriberInformationChange(This,newInterface)	\
    ( (This)->lpVtbl -> OnSubscriberInformationChange(This,newInterface) ) 

#define IMbnInterfaceEvents_OnReadyStateChange(This,newInterface)	\
    ( (This)->lpVtbl -> OnReadyStateChange(This,newInterface) ) 

#define IMbnInterfaceEvents_OnEmergencyModeChange(This,newInterface)	\
    ( (This)->lpVtbl -> OnEmergencyModeChange(This,newInterface) ) 

#define IMbnInterfaceEvents_OnHomeProviderAvailable(This,newInterface)	\
    ( (This)->lpVtbl -> OnHomeProviderAvailable(This,newInterface) ) 

#define IMbnInterfaceEvents_OnPreferredProvidersChange(This,newInterface)	\
    ( (This)->lpVtbl -> OnPreferredProvidersChange(This,newInterface) ) 

#define IMbnInterfaceEvents_OnSetPreferredProvidersComplete(This,newInterface,requestID,status)	\
    ( (This)->lpVtbl -> OnSetPreferredProvidersComplete(This,newInterface,requestID,status) ) 

#define IMbnInterfaceEvents_OnScanNetworkComplete(This,newInterface,requestID,status)	\
    ( (This)->lpVtbl -> OnScanNetworkComplete(This,newInterface,requestID,status) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMbnInterfaceEvents_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mbnapi_0000_0005 */
/* [local] */ 

#pragma deprecated(IMbnInterfaceEvents) // Replaced by WinRT API in Windows.Networking.Connectivity and Windows.Networking.NetworkOperators namespaces


extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0005_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0005_v0_0_s_ifspec;

#ifndef __IMbnInterfaceManager_INTERFACE_DEFINED__
#define __IMbnInterfaceManager_INTERFACE_DEFINED__

/* interface IMbnInterfaceManager */
/* [helpstring][uuid][oleautomation][nonextensible][object] */ 


EXTERN_C const IID IID_IMbnInterfaceManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DCBBBAB6-201B-4BBB-AAEE-338E368AF6FA")
    IMbnInterfaceManager : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetInterface( 
            /* [in] */ __RPC__in LPCWSTR interfaceID,
            /* [retval][ref][out] */ __RPC__deref_out_opt IMbnInterface **mbnInterface) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetInterfaces( 
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *mbnInterfaces) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMbnInterfaceManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMbnInterfaceManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMbnInterfaceManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMbnInterfaceManager * This);
        
        DECLSPEC_XFGVIRT(IMbnInterfaceManager, GetInterface)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetInterface )( 
            __RPC__in IMbnInterfaceManager * This,
            /* [in] */ __RPC__in LPCWSTR interfaceID,
            /* [retval][ref][out] */ __RPC__deref_out_opt IMbnInterface **mbnInterface);
        
        DECLSPEC_XFGVIRT(IMbnInterfaceManager, GetInterfaces)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetInterfaces )( 
            __RPC__in IMbnInterfaceManager * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *mbnInterfaces);
        
        END_INTERFACE
    } IMbnInterfaceManagerVtbl;

    interface IMbnInterfaceManager
    {
        CONST_VTBL struct IMbnInterfaceManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMbnInterfaceManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMbnInterfaceManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMbnInterfaceManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMbnInterfaceManager_GetInterface(This,interfaceID,mbnInterface)	\
    ( (This)->lpVtbl -> GetInterface(This,interfaceID,mbnInterface) ) 

#define IMbnInterfaceManager_GetInterfaces(This,mbnInterfaces)	\
    ( (This)->lpVtbl -> GetInterfaces(This,mbnInterfaces) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMbnInterfaceManager_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mbnapi_0000_0006 */
/* [local] */ 

#pragma deprecated(IMbnInterfaceManager) // Replaced by WinRT API in Windows.Networking.Connectivity and Windows.Networking.NetworkOperators namespaces


extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0006_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0006_v0_0_s_ifspec;

#ifndef __IMbnInterfaceManagerEvents_INTERFACE_DEFINED__
#define __IMbnInterfaceManagerEvents_INTERFACE_DEFINED__

/* interface IMbnInterfaceManagerEvents */
/* [helpstring][uuid][oleautomation][nonextensible][object] */ 


EXTERN_C const IID IID_IMbnInterfaceManagerEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DCBBBAB6-201C-4BBB-AAEE-338E368AF6FA")
    IMbnInterfaceManagerEvents : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnInterfaceArrival( 
            /* [in] */ __RPC__in_opt IMbnInterface *newInterface) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnInterfaceRemoval( 
            /* [in] */ __RPC__in_opt IMbnInterface *oldInterface) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMbnInterfaceManagerEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMbnInterfaceManagerEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMbnInterfaceManagerEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMbnInterfaceManagerEvents * This);
        
        DECLSPEC_XFGVIRT(IMbnInterfaceManagerEvents, OnInterfaceArrival)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnInterfaceArrival )( 
            __RPC__in IMbnInterfaceManagerEvents * This,
            /* [in] */ __RPC__in_opt IMbnInterface *newInterface);
        
        DECLSPEC_XFGVIRT(IMbnInterfaceManagerEvents, OnInterfaceRemoval)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnInterfaceRemoval )( 
            __RPC__in IMbnInterfaceManagerEvents * This,
            /* [in] */ __RPC__in_opt IMbnInterface *oldInterface);
        
        END_INTERFACE
    } IMbnInterfaceManagerEventsVtbl;

    interface IMbnInterfaceManagerEvents
    {
        CONST_VTBL struct IMbnInterfaceManagerEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMbnInterfaceManagerEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMbnInterfaceManagerEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMbnInterfaceManagerEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMbnInterfaceManagerEvents_OnInterfaceArrival(This,newInterface)	\
    ( (This)->lpVtbl -> OnInterfaceArrival(This,newInterface) ) 

#define IMbnInterfaceManagerEvents_OnInterfaceRemoval(This,oldInterface)	\
    ( (This)->lpVtbl -> OnInterfaceRemoval(This,oldInterface) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMbnInterfaceManagerEvents_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mbnapi_0000_0007 */
/* [local] */ 

#pragma deprecated(IMbnInterfaceManagerEvents) // Replaced by WinRT API in Windows.Networking.Connectivity and Windows.Networking.NetworkOperators namespaces


extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0007_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0007_v0_0_s_ifspec;

#ifndef __IMbnRegistration_INTERFACE_DEFINED__
#define __IMbnRegistration_INTERFACE_DEFINED__

/* interface IMbnRegistration */
/* [helpstring][uuid][oleautomation][nonextensible][object] */ 


EXTERN_C const IID IID_IMbnRegistration;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DCBBBAB6-2009-4BBB-AAEE-338E368AF6FA")
    IMbnRegistration : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetRegisterState( 
            /* [retval][ref][out] */ __RPC__out MBN_REGISTER_STATE *registerState) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetRegisterMode( 
            /* [retval][ref][out] */ __RPC__out MBN_REGISTER_MODE *registerMode) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetProviderID( 
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *providerID) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetProviderName( 
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *providerName) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetRoamingText( 
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *roamingText) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetAvailableDataClasses( 
            /* [retval][ref][out] */ __RPC__out ULONG *availableDataClasses) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetCurrentDataClass( 
            /* [retval][ref][out] */ __RPC__out ULONG *currentDataClass) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetRegistrationNetworkError( 
            /* [retval][ref][out] */ __RPC__out ULONG *registrationNetworkError) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetPacketAttachNetworkError( 
            /* [retval][ref][out] */ __RPC__out ULONG *packetAttachNetworkError) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetRegisterMode( 
            /* [in] */ MBN_REGISTER_MODE registerMode,
            /* [string][in] */ __RPC__in_string LPCWSTR providerID,
            /* [in] */ ULONG dataClass,
            /* [out] */ __RPC__out ULONG *requestID) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMbnRegistrationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMbnRegistration * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMbnRegistration * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMbnRegistration * This);
        
        DECLSPEC_XFGVIRT(IMbnRegistration, GetRegisterState)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetRegisterState )( 
            __RPC__in IMbnRegistration * This,
            /* [retval][ref][out] */ __RPC__out MBN_REGISTER_STATE *registerState);
        
        DECLSPEC_XFGVIRT(IMbnRegistration, GetRegisterMode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetRegisterMode )( 
            __RPC__in IMbnRegistration * This,
            /* [retval][ref][out] */ __RPC__out MBN_REGISTER_MODE *registerMode);
        
        DECLSPEC_XFGVIRT(IMbnRegistration, GetProviderID)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetProviderID )( 
            __RPC__in IMbnRegistration * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *providerID);
        
        DECLSPEC_XFGVIRT(IMbnRegistration, GetProviderName)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetProviderName )( 
            __RPC__in IMbnRegistration * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *providerName);
        
        DECLSPEC_XFGVIRT(IMbnRegistration, GetRoamingText)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetRoamingText )( 
            __RPC__in IMbnRegistration * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *roamingText);
        
        DECLSPEC_XFGVIRT(IMbnRegistration, GetAvailableDataClasses)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetAvailableDataClasses )( 
            __RPC__in IMbnRegistration * This,
            /* [retval][ref][out] */ __RPC__out ULONG *availableDataClasses);
        
        DECLSPEC_XFGVIRT(IMbnRegistration, GetCurrentDataClass)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetCurrentDataClass )( 
            __RPC__in IMbnRegistration * This,
            /* [retval][ref][out] */ __RPC__out ULONG *currentDataClass);
        
        DECLSPEC_XFGVIRT(IMbnRegistration, GetRegistrationNetworkError)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetRegistrationNetworkError )( 
            __RPC__in IMbnRegistration * This,
            /* [retval][ref][out] */ __RPC__out ULONG *registrationNetworkError);
        
        DECLSPEC_XFGVIRT(IMbnRegistration, GetPacketAttachNetworkError)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetPacketAttachNetworkError )( 
            __RPC__in IMbnRegistration * This,
            /* [retval][ref][out] */ __RPC__out ULONG *packetAttachNetworkError);
        
        DECLSPEC_XFGVIRT(IMbnRegistration, SetRegisterMode)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetRegisterMode )( 
            __RPC__in IMbnRegistration * This,
            /* [in] */ MBN_REGISTER_MODE registerMode,
            /* [string][in] */ __RPC__in_string LPCWSTR providerID,
            /* [in] */ ULONG dataClass,
            /* [out] */ __RPC__out ULONG *requestID);
        
        END_INTERFACE
    } IMbnRegistrationVtbl;

    interface IMbnRegistration
    {
        CONST_VTBL struct IMbnRegistrationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMbnRegistration_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMbnRegistration_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMbnRegistration_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMbnRegistration_GetRegisterState(This,registerState)	\
    ( (This)->lpVtbl -> GetRegisterState(This,registerState) ) 

#define IMbnRegistration_GetRegisterMode(This,registerMode)	\
    ( (This)->lpVtbl -> GetRegisterMode(This,registerMode) ) 

#define IMbnRegistration_GetProviderID(This,providerID)	\
    ( (This)->lpVtbl -> GetProviderID(This,providerID) ) 

#define IMbnRegistration_GetProviderName(This,providerName)	\
    ( (This)->lpVtbl -> GetProviderName(This,providerName) ) 

#define IMbnRegistration_GetRoamingText(This,roamingText)	\
    ( (This)->lpVtbl -> GetRoamingText(This,roamingText) ) 

#define IMbnRegistration_GetAvailableDataClasses(This,availableDataClasses)	\
    ( (This)->lpVtbl -> GetAvailableDataClasses(This,availableDataClasses) ) 

#define IMbnRegistration_GetCurrentDataClass(This,currentDataClass)	\
    ( (This)->lpVtbl -> GetCurrentDataClass(This,currentDataClass) ) 

#define IMbnRegistration_GetRegistrationNetworkError(This,registrationNetworkError)	\
    ( (This)->lpVtbl -> GetRegistrationNetworkError(This,registrationNetworkError) ) 

#define IMbnRegistration_GetPacketAttachNetworkError(This,packetAttachNetworkError)	\
    ( (This)->lpVtbl -> GetPacketAttachNetworkError(This,packetAttachNetworkError) ) 

#define IMbnRegistration_SetRegisterMode(This,registerMode,providerID,dataClass,requestID)	\
    ( (This)->lpVtbl -> SetRegisterMode(This,registerMode,providerID,dataClass,requestID) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMbnRegistration_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mbnapi_0000_0008 */
/* [local] */ 

#pragma deprecated(IMbnRegistration) // Replaced by WinRT API in Windows.Networking.Connectivity and Windows.Networking.NetworkOperators namespaces


extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0008_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0008_v0_0_s_ifspec;

#ifndef __IMbnRegistrationEvents_INTERFACE_DEFINED__
#define __IMbnRegistrationEvents_INTERFACE_DEFINED__

/* interface IMbnRegistrationEvents */
/* [helpstring][uuid][oleautomation][nonextensible][object] */ 


EXTERN_C const IID IID_IMbnRegistrationEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DCBBBAB6-200A-4BBB-AAEE-338E368AF6FA")
    IMbnRegistrationEvents : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnRegisterModeAvailable( 
            /* [in] */ __RPC__in_opt IMbnRegistration *newInterface) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnRegisterStateChange( 
            /* [in] */ __RPC__in_opt IMbnRegistration *newInterface) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnPacketServiceStateChange( 
            /* [in] */ __RPC__in_opt IMbnRegistration *newInterface) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnSetRegisterModeComplete( 
            /* [in] */ __RPC__in_opt IMbnRegistration *newInterface,
            /* [in] */ ULONG requestID,
            /* [in] */ HRESULT status) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMbnRegistrationEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMbnRegistrationEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMbnRegistrationEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMbnRegistrationEvents * This);
        
        DECLSPEC_XFGVIRT(IMbnRegistrationEvents, OnRegisterModeAvailable)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnRegisterModeAvailable )( 
            __RPC__in IMbnRegistrationEvents * This,
            /* [in] */ __RPC__in_opt IMbnRegistration *newInterface);
        
        DECLSPEC_XFGVIRT(IMbnRegistrationEvents, OnRegisterStateChange)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnRegisterStateChange )( 
            __RPC__in IMbnRegistrationEvents * This,
            /* [in] */ __RPC__in_opt IMbnRegistration *newInterface);
        
        DECLSPEC_XFGVIRT(IMbnRegistrationEvents, OnPacketServiceStateChange)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnPacketServiceStateChange )( 
            __RPC__in IMbnRegistrationEvents * This,
            /* [in] */ __RPC__in_opt IMbnRegistration *newInterface);
        
        DECLSPEC_XFGVIRT(IMbnRegistrationEvents, OnSetRegisterModeComplete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnSetRegisterModeComplete )( 
            __RPC__in IMbnRegistrationEvents * This,
            /* [in] */ __RPC__in_opt IMbnRegistration *newInterface,
            /* [in] */ ULONG requestID,
            /* [in] */ HRESULT status);
        
        END_INTERFACE
    } IMbnRegistrationEventsVtbl;

    interface IMbnRegistrationEvents
    {
        CONST_VTBL struct IMbnRegistrationEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMbnRegistrationEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMbnRegistrationEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMbnRegistrationEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMbnRegistrationEvents_OnRegisterModeAvailable(This,newInterface)	\
    ( (This)->lpVtbl -> OnRegisterModeAvailable(This,newInterface) ) 

#define IMbnRegistrationEvents_OnRegisterStateChange(This,newInterface)	\
    ( (This)->lpVtbl -> OnRegisterStateChange(This,newInterface) ) 

#define IMbnRegistrationEvents_OnPacketServiceStateChange(This,newInterface)	\
    ( (This)->lpVtbl -> OnPacketServiceStateChange(This,newInterface) ) 

#define IMbnRegistrationEvents_OnSetRegisterModeComplete(This,newInterface,requestID,status)	\
    ( (This)->lpVtbl -> OnSetRegisterModeComplete(This,newInterface,requestID,status) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMbnRegistrationEvents_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mbnapi_0000_0009 */
/* [local] */ 

#pragma deprecated(IMbnRegistrationEvents) // Replaced by WinRT API in Windows.Networking.Connectivity and Windows.Networking.NetworkOperators namespaces


extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0009_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0009_v0_0_s_ifspec;

#ifndef __IMbnConnectionManager_INTERFACE_DEFINED__
#define __IMbnConnectionManager_INTERFACE_DEFINED__

/* interface IMbnConnectionManager */
/* [helpstring][uuid][oleautomation][nonextensible][object] */ 


EXTERN_C const IID IID_IMbnConnectionManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DCBBBAB6-201D-4BBB-AAEE-338E368AF6FA")
    IMbnConnectionManager : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetConnection( 
            /* [in] */ __RPC__in LPCWSTR connectionID,
            /* [retval][ref][out] */ __RPC__deref_out_opt IMbnConnection **mbnConnection) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetConnections( 
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *mbnConnections) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMbnConnectionManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMbnConnectionManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMbnConnectionManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMbnConnectionManager * This);
        
        DECLSPEC_XFGVIRT(IMbnConnectionManager, GetConnection)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetConnection )( 
            __RPC__in IMbnConnectionManager * This,
            /* [in] */ __RPC__in LPCWSTR connectionID,
            /* [retval][ref][out] */ __RPC__deref_out_opt IMbnConnection **mbnConnection);
        
        DECLSPEC_XFGVIRT(IMbnConnectionManager, GetConnections)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetConnections )( 
            __RPC__in IMbnConnectionManager * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *mbnConnections);
        
        END_INTERFACE
    } IMbnConnectionManagerVtbl;

    interface IMbnConnectionManager
    {
        CONST_VTBL struct IMbnConnectionManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMbnConnectionManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMbnConnectionManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMbnConnectionManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMbnConnectionManager_GetConnection(This,connectionID,mbnConnection)	\
    ( (This)->lpVtbl -> GetConnection(This,connectionID,mbnConnection) ) 

#define IMbnConnectionManager_GetConnections(This,mbnConnections)	\
    ( (This)->lpVtbl -> GetConnections(This,mbnConnections) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMbnConnectionManager_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mbnapi_0000_0010 */
/* [local] */ 

#pragma deprecated(IMbnConnectionManager) // Replaced by WinRT API in Windows.Networking.Connectivity and Windows.Networking.NetworkOperators namespaces


extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0010_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0010_v0_0_s_ifspec;

#ifndef __IMbnConnectionManagerEvents_INTERFACE_DEFINED__
#define __IMbnConnectionManagerEvents_INTERFACE_DEFINED__

/* interface IMbnConnectionManagerEvents */
/* [helpstring][uuid][oleautomation][nonextensible][object] */ 


EXTERN_C const IID IID_IMbnConnectionManagerEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DCBBBAB6-201E-4BBB-AAEE-338E368AF6FA")
    IMbnConnectionManagerEvents : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnConnectionArrival( 
            /* [in] */ __RPC__in_opt IMbnConnection *newConnection) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnConnectionRemoval( 
            /* [in] */ __RPC__in_opt IMbnConnection *oldConnection) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMbnConnectionManagerEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMbnConnectionManagerEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMbnConnectionManagerEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMbnConnectionManagerEvents * This);
        
        DECLSPEC_XFGVIRT(IMbnConnectionManagerEvents, OnConnectionArrival)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnConnectionArrival )( 
            __RPC__in IMbnConnectionManagerEvents * This,
            /* [in] */ __RPC__in_opt IMbnConnection *newConnection);
        
        DECLSPEC_XFGVIRT(IMbnConnectionManagerEvents, OnConnectionRemoval)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnConnectionRemoval )( 
            __RPC__in IMbnConnectionManagerEvents * This,
            /* [in] */ __RPC__in_opt IMbnConnection *oldConnection);
        
        END_INTERFACE
    } IMbnConnectionManagerEventsVtbl;

    interface IMbnConnectionManagerEvents
    {
        CONST_VTBL struct IMbnConnectionManagerEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMbnConnectionManagerEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMbnConnectionManagerEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMbnConnectionManagerEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMbnConnectionManagerEvents_OnConnectionArrival(This,newConnection)	\
    ( (This)->lpVtbl -> OnConnectionArrival(This,newConnection) ) 

#define IMbnConnectionManagerEvents_OnConnectionRemoval(This,oldConnection)	\
    ( (This)->lpVtbl -> OnConnectionRemoval(This,oldConnection) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMbnConnectionManagerEvents_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mbnapi_0000_0011 */
/* [local] */ 

#pragma deprecated(IMbnConnectionManagerEvents) // Replaced by WinRT API in Windows.Networking.Connectivity and Windows.Networking.NetworkOperators namespaces


extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0011_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0011_v0_0_s_ifspec;

#ifndef __IMbnPinManager_INTERFACE_DEFINED__
#define __IMbnPinManager_INTERFACE_DEFINED__

/* interface IMbnPinManager */
/* [helpstring][uuid][oleautomation][nonextensible][object] */ 


EXTERN_C const IID IID_IMbnPinManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DCBBBAB6-2005-4BBB-AAEE-338E368AF6FA")
    IMbnPinManager : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetPinList( 
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *pinList) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetPin( 
            /* [in] */ MBN_PIN_TYPE pinType,
            /* [retval][ref][out] */ __RPC__deref_out_opt IMbnPin **pin) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetPinState( 
            /* [out] */ __RPC__out ULONG *requestID) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMbnPinManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMbnPinManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMbnPinManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMbnPinManager * This);
        
        DECLSPEC_XFGVIRT(IMbnPinManager, GetPinList)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetPinList )( 
            __RPC__in IMbnPinManager * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *pinList);
        
        DECLSPEC_XFGVIRT(IMbnPinManager, GetPin)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetPin )( 
            __RPC__in IMbnPinManager * This,
            /* [in] */ MBN_PIN_TYPE pinType,
            /* [retval][ref][out] */ __RPC__deref_out_opt IMbnPin **pin);
        
        DECLSPEC_XFGVIRT(IMbnPinManager, GetPinState)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetPinState )( 
            __RPC__in IMbnPinManager * This,
            /* [out] */ __RPC__out ULONG *requestID);
        
        END_INTERFACE
    } IMbnPinManagerVtbl;

    interface IMbnPinManager
    {
        CONST_VTBL struct IMbnPinManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMbnPinManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMbnPinManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMbnPinManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMbnPinManager_GetPinList(This,pinList)	\
    ( (This)->lpVtbl -> GetPinList(This,pinList) ) 

#define IMbnPinManager_GetPin(This,pinType,pin)	\
    ( (This)->lpVtbl -> GetPin(This,pinType,pin) ) 

#define IMbnPinManager_GetPinState(This,requestID)	\
    ( (This)->lpVtbl -> GetPinState(This,requestID) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMbnPinManager_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mbnapi_0000_0012 */
/* [local] */ 

#pragma deprecated(IMbnPinManager) // Replaced by WinRT API in Windows.Networking.Connectivity and Windows.Networking.NetworkOperators namespaces


extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0012_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0012_v0_0_s_ifspec;

#ifndef __IMbnPinManagerEvents_INTERFACE_DEFINED__
#define __IMbnPinManagerEvents_INTERFACE_DEFINED__

/* interface IMbnPinManagerEvents */
/* [helpstring][uuid][oleautomation][nonextensible][object] */ 


EXTERN_C const IID IID_IMbnPinManagerEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DCBBBAB6-2006-4BBB-AAEE-338E368AF6FA")
    IMbnPinManagerEvents : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnPinListAvailable( 
            /* [in] */ __RPC__in_opt IMbnPinManager *pinManager) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnGetPinStateComplete( 
            /* [in] */ __RPC__in_opt IMbnPinManager *pinManager,
            /* [in] */ MBN_PIN_INFO pinInfo,
            /* [in] */ ULONG requestID,
            /* [in] */ HRESULT status) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMbnPinManagerEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMbnPinManagerEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMbnPinManagerEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMbnPinManagerEvents * This);
        
        DECLSPEC_XFGVIRT(IMbnPinManagerEvents, OnPinListAvailable)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnPinListAvailable )( 
            __RPC__in IMbnPinManagerEvents * This,
            /* [in] */ __RPC__in_opt IMbnPinManager *pinManager);
        
        DECLSPEC_XFGVIRT(IMbnPinManagerEvents, OnGetPinStateComplete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnGetPinStateComplete )( 
            __RPC__in IMbnPinManagerEvents * This,
            /* [in] */ __RPC__in_opt IMbnPinManager *pinManager,
            /* [in] */ MBN_PIN_INFO pinInfo,
            /* [in] */ ULONG requestID,
            /* [in] */ HRESULT status);
        
        END_INTERFACE
    } IMbnPinManagerEventsVtbl;

    interface IMbnPinManagerEvents
    {
        CONST_VTBL struct IMbnPinManagerEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMbnPinManagerEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMbnPinManagerEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMbnPinManagerEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMbnPinManagerEvents_OnPinListAvailable(This,pinManager)	\
    ( (This)->lpVtbl -> OnPinListAvailable(This,pinManager) ) 

#define IMbnPinManagerEvents_OnGetPinStateComplete(This,pinManager,pinInfo,requestID,status)	\
    ( (This)->lpVtbl -> OnGetPinStateComplete(This,pinManager,pinInfo,requestID,status) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMbnPinManagerEvents_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mbnapi_0000_0013 */
/* [local] */ 

#pragma deprecated(IMbnPinManagerEvents) // Replaced by WinRT API in Windows.Networking.Connectivity and Windows.Networking.NetworkOperators namespaces


extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0013_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0013_v0_0_s_ifspec;

#ifndef __IMbnPinEvents_INTERFACE_DEFINED__
#define __IMbnPinEvents_INTERFACE_DEFINED__

/* interface IMbnPinEvents */
/* [helpstring][uuid][oleautomation][nonextensible][object] */ 


EXTERN_C const IID IID_IMbnPinEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DCBBBAB6-2008-4BBB-AAEE-338E368AF6FA")
    IMbnPinEvents : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnEnableComplete( 
            /* [in] */ __RPC__in_opt IMbnPin *pin,
            /* [ref][in] */ __RPC__in MBN_PIN_INFO *pinInfo,
            /* [in] */ ULONG requestID,
            /* [in] */ HRESULT status) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnDisableComplete( 
            /* [in] */ __RPC__in_opt IMbnPin *pin,
            /* [ref][in] */ __RPC__in MBN_PIN_INFO *pinInfo,
            /* [in] */ ULONG requestID,
            /* [in] */ HRESULT status) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnEnterComplete( 
            /* [in] */ __RPC__in_opt IMbnPin *Pin,
            /* [ref][in] */ __RPC__in MBN_PIN_INFO *pinInfo,
            /* [in] */ ULONG requestID,
            /* [in] */ HRESULT status) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnChangeComplete( 
            /* [in] */ __RPC__in_opt IMbnPin *Pin,
            /* [ref][in] */ __RPC__in MBN_PIN_INFO *pinInfo,
            /* [in] */ ULONG requestID,
            /* [in] */ HRESULT status) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnUnblockComplete( 
            /* [in] */ __RPC__in_opt IMbnPin *Pin,
            /* [ref][in] */ __RPC__in MBN_PIN_INFO *pinInfo,
            /* [in] */ ULONG requestID,
            /* [in] */ HRESULT status) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMbnPinEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMbnPinEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMbnPinEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMbnPinEvents * This);
        
        DECLSPEC_XFGVIRT(IMbnPinEvents, OnEnableComplete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnEnableComplete )( 
            __RPC__in IMbnPinEvents * This,
            /* [in] */ __RPC__in_opt IMbnPin *pin,
            /* [ref][in] */ __RPC__in MBN_PIN_INFO *pinInfo,
            /* [in] */ ULONG requestID,
            /* [in] */ HRESULT status);
        
        DECLSPEC_XFGVIRT(IMbnPinEvents, OnDisableComplete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnDisableComplete )( 
            __RPC__in IMbnPinEvents * This,
            /* [in] */ __RPC__in_opt IMbnPin *pin,
            /* [ref][in] */ __RPC__in MBN_PIN_INFO *pinInfo,
            /* [in] */ ULONG requestID,
            /* [in] */ HRESULT status);
        
        DECLSPEC_XFGVIRT(IMbnPinEvents, OnEnterComplete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnEnterComplete )( 
            __RPC__in IMbnPinEvents * This,
            /* [in] */ __RPC__in_opt IMbnPin *Pin,
            /* [ref][in] */ __RPC__in MBN_PIN_INFO *pinInfo,
            /* [in] */ ULONG requestID,
            /* [in] */ HRESULT status);
        
        DECLSPEC_XFGVIRT(IMbnPinEvents, OnChangeComplete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnChangeComplete )( 
            __RPC__in IMbnPinEvents * This,
            /* [in] */ __RPC__in_opt IMbnPin *Pin,
            /* [ref][in] */ __RPC__in MBN_PIN_INFO *pinInfo,
            /* [in] */ ULONG requestID,
            /* [in] */ HRESULT status);
        
        DECLSPEC_XFGVIRT(IMbnPinEvents, OnUnblockComplete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnUnblockComplete )( 
            __RPC__in IMbnPinEvents * This,
            /* [in] */ __RPC__in_opt IMbnPin *Pin,
            /* [ref][in] */ __RPC__in MBN_PIN_INFO *pinInfo,
            /* [in] */ ULONG requestID,
            /* [in] */ HRESULT status);
        
        END_INTERFACE
    } IMbnPinEventsVtbl;

    interface IMbnPinEvents
    {
        CONST_VTBL struct IMbnPinEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMbnPinEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMbnPinEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMbnPinEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMbnPinEvents_OnEnableComplete(This,pin,pinInfo,requestID,status)	\
    ( (This)->lpVtbl -> OnEnableComplete(This,pin,pinInfo,requestID,status) ) 

#define IMbnPinEvents_OnDisableComplete(This,pin,pinInfo,requestID,status)	\
    ( (This)->lpVtbl -> OnDisableComplete(This,pin,pinInfo,requestID,status) ) 

#define IMbnPinEvents_OnEnterComplete(This,Pin,pinInfo,requestID,status)	\
    ( (This)->lpVtbl -> OnEnterComplete(This,Pin,pinInfo,requestID,status) ) 

#define IMbnPinEvents_OnChangeComplete(This,Pin,pinInfo,requestID,status)	\
    ( (This)->lpVtbl -> OnChangeComplete(This,Pin,pinInfo,requestID,status) ) 

#define IMbnPinEvents_OnUnblockComplete(This,Pin,pinInfo,requestID,status)	\
    ( (This)->lpVtbl -> OnUnblockComplete(This,Pin,pinInfo,requestID,status) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMbnPinEvents_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mbnapi_0000_0014 */
/* [local] */ 

#pragma deprecated(IMbnPinEvents) // Replaced by WinRT API in Windows.Networking.Connectivity and Windows.Networking.NetworkOperators namespaces
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0014_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0014_v0_0_s_ifspec;

#ifndef __IMbnSubscriberInformation_INTERFACE_DEFINED__
#define __IMbnSubscriberInformation_INTERFACE_DEFINED__

/* interface IMbnSubscriberInformation */
/* [helpstring][uuid][oleautomation][nonextensible][object] */ 


EXTERN_C const IID IID_IMbnSubscriberInformation;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("459ECC43-BCF5-11DC-A8A8-001321F1405F")
    IMbnSubscriberInformation : public IUnknown
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SubscriberID( 
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *SubscriberID) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SimIccID( 
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *SimIccID) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TelephoneNumbers( 
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *TelephoneNumbers) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMbnSubscriberInformationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMbnSubscriberInformation * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMbnSubscriberInformation * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMbnSubscriberInformation * This);
        
        DECLSPEC_XFGVIRT(IMbnSubscriberInformation, get_SubscriberID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SubscriberID )( 
            __RPC__in IMbnSubscriberInformation * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *SubscriberID);
        
        DECLSPEC_XFGVIRT(IMbnSubscriberInformation, get_SimIccID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SimIccID )( 
            __RPC__in IMbnSubscriberInformation * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *SimIccID);
        
        DECLSPEC_XFGVIRT(IMbnSubscriberInformation, get_TelephoneNumbers)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TelephoneNumbers )( 
            __RPC__in IMbnSubscriberInformation * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *TelephoneNumbers);
        
        END_INTERFACE
    } IMbnSubscriberInformationVtbl;

    interface IMbnSubscriberInformation
    {
        CONST_VTBL struct IMbnSubscriberInformationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMbnSubscriberInformation_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMbnSubscriberInformation_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMbnSubscriberInformation_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMbnSubscriberInformation_get_SubscriberID(This,SubscriberID)	\
    ( (This)->lpVtbl -> get_SubscriberID(This,SubscriberID) ) 

#define IMbnSubscriberInformation_get_SimIccID(This,SimIccID)	\
    ( (This)->lpVtbl -> get_SimIccID(This,SimIccID) ) 

#define IMbnSubscriberInformation_get_TelephoneNumbers(This,TelephoneNumbers)	\
    ( (This)->lpVtbl -> get_TelephoneNumbers(This,TelephoneNumbers) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMbnSubscriberInformation_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mbnapi_0000_0015 */
/* [local] */ 

#pragma deprecated(IMbnSubscriberInformation) // Replaced by WinRT API in Windows.Networking.Connectivity and Windows.Networking.NetworkOperators namespaces


extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0015_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0015_v0_0_s_ifspec;

#ifndef __IMbnSignal_INTERFACE_DEFINED__
#define __IMbnSignal_INTERFACE_DEFINED__

/* interface IMbnSignal */
/* [helpstring][uuid][oleautomation][nonextensible][object] */ 


EXTERN_C const IID IID_IMbnSignal;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DCBBBAB6-2003-4BBB-AAEE-338E368AF6FA")
    IMbnSignal : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetSignalStrength( 
            /* [retval][ref][out] */ __RPC__out ULONG *signalStrength) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetSignalError( 
            /* [retval][ref][out] */ __RPC__out ULONG *signalError) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMbnSignalVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMbnSignal * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMbnSignal * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMbnSignal * This);
        
        DECLSPEC_XFGVIRT(IMbnSignal, GetSignalStrength)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetSignalStrength )( 
            __RPC__in IMbnSignal * This,
            /* [retval][ref][out] */ __RPC__out ULONG *signalStrength);
        
        DECLSPEC_XFGVIRT(IMbnSignal, GetSignalError)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetSignalError )( 
            __RPC__in IMbnSignal * This,
            /* [retval][ref][out] */ __RPC__out ULONG *signalError);
        
        END_INTERFACE
    } IMbnSignalVtbl;

    interface IMbnSignal
    {
        CONST_VTBL struct IMbnSignalVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMbnSignal_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMbnSignal_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMbnSignal_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMbnSignal_GetSignalStrength(This,signalStrength)	\
    ( (This)->lpVtbl -> GetSignalStrength(This,signalStrength) ) 

#define IMbnSignal_GetSignalError(This,signalError)	\
    ( (This)->lpVtbl -> GetSignalError(This,signalError) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMbnSignal_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mbnapi_0000_0016 */
/* [local] */ 

#pragma deprecated(IMbnSignal) // Replaced by WinRT API in Windows.Networking.Connectivity and Windows.Networking.NetworkOperators namespaces


extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0016_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0016_v0_0_s_ifspec;

#ifndef __IMbnSignalEvents_INTERFACE_DEFINED__
#define __IMbnSignalEvents_INTERFACE_DEFINED__

/* interface IMbnSignalEvents */
/* [helpstring][uuid][oleautomation][nonextensible][object] */ 


EXTERN_C const IID IID_IMbnSignalEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DCBBBAB6-2004-4BBB-AAEE-338E368AF6FA")
    IMbnSignalEvents : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnSignalStateChange( 
            /* [in] */ __RPC__in_opt IMbnSignal *newInterface) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMbnSignalEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMbnSignalEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMbnSignalEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMbnSignalEvents * This);
        
        DECLSPEC_XFGVIRT(IMbnSignalEvents, OnSignalStateChange)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnSignalStateChange )( 
            __RPC__in IMbnSignalEvents * This,
            /* [in] */ __RPC__in_opt IMbnSignal *newInterface);
        
        END_INTERFACE
    } IMbnSignalEventsVtbl;

    interface IMbnSignalEvents
    {
        CONST_VTBL struct IMbnSignalEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMbnSignalEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMbnSignalEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMbnSignalEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMbnSignalEvents_OnSignalStateChange(This,newInterface)	\
    ( (This)->lpVtbl -> OnSignalStateChange(This,newInterface) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMbnSignalEvents_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mbnapi_0000_0017 */
/* [local] */ 

#pragma deprecated(IMbnSignalEvents) // Replaced by WinRT API in Windows.Networking.Connectivity and Windows.Networking.NetworkOperators namespaces


extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0017_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0017_v0_0_s_ifspec;

#ifndef __IMbnConnectionContext_INTERFACE_DEFINED__
#define __IMbnConnectionContext_INTERFACE_DEFINED__

/* interface IMbnConnectionContext */
/* [helpstring][uuid][oleautomation][nonextensible][object] */ 


EXTERN_C const IID IID_IMbnConnectionContext;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DCBBBAB6-200B-4BBB-AAEE-338E368AF6FA")
    IMbnConnectionContext : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetProvisionedContexts( 
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *provisionedContexts) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetProvisionedContext( 
            /* [in] */ MBN_CONTEXT provisionedContexts,
            /* [in] */ __RPC__in LPCWSTR providerID,
            /* [out] */ __RPC__out ULONG *requestID) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMbnConnectionContextVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMbnConnectionContext * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMbnConnectionContext * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMbnConnectionContext * This);
        
        DECLSPEC_XFGVIRT(IMbnConnectionContext, GetProvisionedContexts)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetProvisionedContexts )( 
            __RPC__in IMbnConnectionContext * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *provisionedContexts);
        
        DECLSPEC_XFGVIRT(IMbnConnectionContext, SetProvisionedContext)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetProvisionedContext )( 
            __RPC__in IMbnConnectionContext * This,
            /* [in] */ MBN_CONTEXT provisionedContexts,
            /* [in] */ __RPC__in LPCWSTR providerID,
            /* [out] */ __RPC__out ULONG *requestID);
        
        END_INTERFACE
    } IMbnConnectionContextVtbl;

    interface IMbnConnectionContext
    {
        CONST_VTBL struct IMbnConnectionContextVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMbnConnectionContext_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMbnConnectionContext_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMbnConnectionContext_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMbnConnectionContext_GetProvisionedContexts(This,provisionedContexts)	\
    ( (This)->lpVtbl -> GetProvisionedContexts(This,provisionedContexts) ) 

#define IMbnConnectionContext_SetProvisionedContext(This,provisionedContexts,providerID,requestID)	\
    ( (This)->lpVtbl -> SetProvisionedContext(This,provisionedContexts,providerID,requestID) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMbnConnectionContext_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mbnapi_0000_0018 */
/* [local] */ 

#pragma deprecated(IMbnConnectionContext) // Replaced by WinRT API in Windows.Networking.Connectivity and Windows.Networking.NetworkOperators namespaces


extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0018_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0018_v0_0_s_ifspec;

#ifndef __IMbnConnectionContextEvents_INTERFACE_DEFINED__
#define __IMbnConnectionContextEvents_INTERFACE_DEFINED__

/* interface IMbnConnectionContextEvents */
/* [helpstring][uuid][oleautomation][nonextensible][object] */ 


EXTERN_C const IID IID_IMbnConnectionContextEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DCBBBAB6-200C-4BBB-AAEE-338E368AF6FA")
    IMbnConnectionContextEvents : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnProvisionedContextListChange( 
            /* [in] */ __RPC__in_opt IMbnConnectionContext *newInterface) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnSetProvisionedContextComplete( 
            /* [in] */ __RPC__in_opt IMbnConnectionContext *newInterface,
            /* [in] */ ULONG requestID,
            /* [in] */ HRESULT status) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMbnConnectionContextEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMbnConnectionContextEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMbnConnectionContextEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMbnConnectionContextEvents * This);
        
        DECLSPEC_XFGVIRT(IMbnConnectionContextEvents, OnProvisionedContextListChange)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnProvisionedContextListChange )( 
            __RPC__in IMbnConnectionContextEvents * This,
            /* [in] */ __RPC__in_opt IMbnConnectionContext *newInterface);
        
        DECLSPEC_XFGVIRT(IMbnConnectionContextEvents, OnSetProvisionedContextComplete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnSetProvisionedContextComplete )( 
            __RPC__in IMbnConnectionContextEvents * This,
            /* [in] */ __RPC__in_opt IMbnConnectionContext *newInterface,
            /* [in] */ ULONG requestID,
            /* [in] */ HRESULT status);
        
        END_INTERFACE
    } IMbnConnectionContextEventsVtbl;

    interface IMbnConnectionContextEvents
    {
        CONST_VTBL struct IMbnConnectionContextEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMbnConnectionContextEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMbnConnectionContextEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMbnConnectionContextEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMbnConnectionContextEvents_OnProvisionedContextListChange(This,newInterface)	\
    ( (This)->lpVtbl -> OnProvisionedContextListChange(This,newInterface) ) 

#define IMbnConnectionContextEvents_OnSetProvisionedContextComplete(This,newInterface,requestID,status)	\
    ( (This)->lpVtbl -> OnSetProvisionedContextComplete(This,newInterface,requestID,status) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMbnConnectionContextEvents_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mbnapi_0000_0019 */
/* [local] */ 

#pragma deprecated(IMbnConnectionContextEvents) // Replaced by WinRT API in Windows.Networking.Connectivity and Windows.Networking.NetworkOperators namespaces


extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0019_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0019_v0_0_s_ifspec;

#ifndef __IMbnConnectionProfileManager_INTERFACE_DEFINED__
#define __IMbnConnectionProfileManager_INTERFACE_DEFINED__

/* interface IMbnConnectionProfileManager */
/* [helpstring][uuid][oleautomation][nonextensible][object] */ 


EXTERN_C const IID IID_IMbnConnectionProfileManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DCBBBAB6-200F-4BBB-AAEE-338E368AF6FA")
    IMbnConnectionProfileManager : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetConnectionProfiles( 
            /* [in] */ __RPC__in_opt IMbnInterface *mbnInterface,
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *connectionProfiles) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetConnectionProfile( 
            /* [in] */ __RPC__in_opt IMbnInterface *mbnInterface,
            /* [string][in] */ __RPC__in_string LPCWSTR profileName,
            /* [retval][ref][out] */ __RPC__deref_out_opt IMbnConnectionProfile **connectionProfile) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateConnectionProfile( 
            /* [string][in] */ __RPC__in_string LPCWSTR xmlProfile) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMbnConnectionProfileManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMbnConnectionProfileManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMbnConnectionProfileManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMbnConnectionProfileManager * This);
        
        DECLSPEC_XFGVIRT(IMbnConnectionProfileManager, GetConnectionProfiles)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetConnectionProfiles )( 
            __RPC__in IMbnConnectionProfileManager * This,
            /* [in] */ __RPC__in_opt IMbnInterface *mbnInterface,
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *connectionProfiles);
        
        DECLSPEC_XFGVIRT(IMbnConnectionProfileManager, GetConnectionProfile)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetConnectionProfile )( 
            __RPC__in IMbnConnectionProfileManager * This,
            /* [in] */ __RPC__in_opt IMbnInterface *mbnInterface,
            /* [string][in] */ __RPC__in_string LPCWSTR profileName,
            /* [retval][ref][out] */ __RPC__deref_out_opt IMbnConnectionProfile **connectionProfile);
        
        DECLSPEC_XFGVIRT(IMbnConnectionProfileManager, CreateConnectionProfile)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateConnectionProfile )( 
            __RPC__in IMbnConnectionProfileManager * This,
            /* [string][in] */ __RPC__in_string LPCWSTR xmlProfile);
        
        END_INTERFACE
    } IMbnConnectionProfileManagerVtbl;

    interface IMbnConnectionProfileManager
    {
        CONST_VTBL struct IMbnConnectionProfileManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMbnConnectionProfileManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMbnConnectionProfileManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMbnConnectionProfileManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMbnConnectionProfileManager_GetConnectionProfiles(This,mbnInterface,connectionProfiles)	\
    ( (This)->lpVtbl -> GetConnectionProfiles(This,mbnInterface,connectionProfiles) ) 

#define IMbnConnectionProfileManager_GetConnectionProfile(This,mbnInterface,profileName,connectionProfile)	\
    ( (This)->lpVtbl -> GetConnectionProfile(This,mbnInterface,profileName,connectionProfile) ) 

#define IMbnConnectionProfileManager_CreateConnectionProfile(This,xmlProfile)	\
    ( (This)->lpVtbl -> CreateConnectionProfile(This,xmlProfile) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMbnConnectionProfileManager_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mbnapi_0000_0020 */
/* [local] */ 

#pragma deprecated(IMbnConnectionProfileManager) // Replaced by WinRT API in Windows.Networking.Connectivity and Windows.Networking.NetworkOperators namespaces


extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0020_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0020_v0_0_s_ifspec;

#ifndef __IMbnConnectionProfile_INTERFACE_DEFINED__
#define __IMbnConnectionProfile_INTERFACE_DEFINED__

/* interface IMbnConnectionProfile */
/* [helpstring][uuid][oleautomation][nonextensible][object] */ 


EXTERN_C const IID IID_IMbnConnectionProfile;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DCBBBAB6-2010-4BBB-AAEE-338E368AF6FA")
    IMbnConnectionProfile : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetProfileXmlData( 
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *profileData) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE UpdateProfile( 
            /* [string][in] */ __RPC__in_string LPCWSTR strProfile) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Delete( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMbnConnectionProfileVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMbnConnectionProfile * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMbnConnectionProfile * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMbnConnectionProfile * This);
        
        DECLSPEC_XFGVIRT(IMbnConnectionProfile, GetProfileXmlData)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetProfileXmlData )( 
            __RPC__in IMbnConnectionProfile * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *profileData);
        
        DECLSPEC_XFGVIRT(IMbnConnectionProfile, UpdateProfile)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *UpdateProfile )( 
            __RPC__in IMbnConnectionProfile * This,
            /* [string][in] */ __RPC__in_string LPCWSTR strProfile);
        
        DECLSPEC_XFGVIRT(IMbnConnectionProfile, Delete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IMbnConnectionProfile * This);
        
        END_INTERFACE
    } IMbnConnectionProfileVtbl;

    interface IMbnConnectionProfile
    {
        CONST_VTBL struct IMbnConnectionProfileVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMbnConnectionProfile_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMbnConnectionProfile_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMbnConnectionProfile_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMbnConnectionProfile_GetProfileXmlData(This,profileData)	\
    ( (This)->lpVtbl -> GetProfileXmlData(This,profileData) ) 

#define IMbnConnectionProfile_UpdateProfile(This,strProfile)	\
    ( (This)->lpVtbl -> UpdateProfile(This,strProfile) ) 

#define IMbnConnectionProfile_Delete(This)	\
    ( (This)->lpVtbl -> Delete(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMbnConnectionProfile_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mbnapi_0000_0021 */
/* [local] */ 

#pragma deprecated(IMbnConnectionProfile) // Replaced by WinRT API in Windows.Networking.Connectivity and Windows.Networking.NetworkOperators namespaces


extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0021_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0021_v0_0_s_ifspec;

#ifndef __IMbnConnectionProfileEvents_INTERFACE_DEFINED__
#define __IMbnConnectionProfileEvents_INTERFACE_DEFINED__

/* interface IMbnConnectionProfileEvents */
/* [helpstring][uuid][oleautomation][nonextensible][object] */ 


EXTERN_C const IID IID_IMbnConnectionProfileEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DCBBBAB6-2011-4BBB-AAEE-338E368AF6FA")
    IMbnConnectionProfileEvents : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnProfileUpdate( 
            /* [in] */ __RPC__in_opt IMbnConnectionProfile *newProfile) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMbnConnectionProfileEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMbnConnectionProfileEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMbnConnectionProfileEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMbnConnectionProfileEvents * This);
        
        DECLSPEC_XFGVIRT(IMbnConnectionProfileEvents, OnProfileUpdate)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnProfileUpdate )( 
            __RPC__in IMbnConnectionProfileEvents * This,
            /* [in] */ __RPC__in_opt IMbnConnectionProfile *newProfile);
        
        END_INTERFACE
    } IMbnConnectionProfileEventsVtbl;

    interface IMbnConnectionProfileEvents
    {
        CONST_VTBL struct IMbnConnectionProfileEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMbnConnectionProfileEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMbnConnectionProfileEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMbnConnectionProfileEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMbnConnectionProfileEvents_OnProfileUpdate(This,newProfile)	\
    ( (This)->lpVtbl -> OnProfileUpdate(This,newProfile) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMbnConnectionProfileEvents_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mbnapi_0000_0022 */
/* [local] */ 

#pragma deprecated(IMbnConnectionProfileEvents) // Replaced by WinRT API in Windows.Networking.Connectivity and Windows.Networking.NetworkOperators namespaces


extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0022_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0022_v0_0_s_ifspec;

#ifndef __IMbnSmsConfiguration_INTERFACE_DEFINED__
#define __IMbnSmsConfiguration_INTERFACE_DEFINED__

/* interface IMbnSmsConfiguration */
/* [helpstring][uuid][oleautomation][nonextensible][object] */ 


EXTERN_C const IID IID_IMbnSmsConfiguration;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DCBBBAB6-2012-4BBB-AAEE-338E368AF6FA")
    IMbnSmsConfiguration : public IUnknown
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ServiceCenterAddress( 
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *scAddress) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_ServiceCenterAddress( 
            /* [in] */ __RPC__in LPCWSTR scAddress) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MaxMessageIndex( 
            /* [retval][ref][out] */ __RPC__out ULONG *index) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CdmaShortMsgSize( 
            /* [retval][ref][out] */ __RPC__out ULONG *shortMsgSize) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SmsFormat( 
            /* [retval][ref][out] */ __RPC__out MBN_SMS_FORMAT *smsFormat) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_SmsFormat( 
            /* [in] */ MBN_SMS_FORMAT smsFormat) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMbnSmsConfigurationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMbnSmsConfiguration * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMbnSmsConfiguration * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMbnSmsConfiguration * This);
        
        DECLSPEC_XFGVIRT(IMbnSmsConfiguration, get_ServiceCenterAddress)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ServiceCenterAddress )( 
            __RPC__in IMbnSmsConfiguration * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *scAddress);
        
        DECLSPEC_XFGVIRT(IMbnSmsConfiguration, put_ServiceCenterAddress)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ServiceCenterAddress )( 
            __RPC__in IMbnSmsConfiguration * This,
            /* [in] */ __RPC__in LPCWSTR scAddress);
        
        DECLSPEC_XFGVIRT(IMbnSmsConfiguration, get_MaxMessageIndex)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MaxMessageIndex )( 
            __RPC__in IMbnSmsConfiguration * This,
            /* [retval][ref][out] */ __RPC__out ULONG *index);
        
        DECLSPEC_XFGVIRT(IMbnSmsConfiguration, get_CdmaShortMsgSize)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CdmaShortMsgSize )( 
            __RPC__in IMbnSmsConfiguration * This,
            /* [retval][ref][out] */ __RPC__out ULONG *shortMsgSize);
        
        DECLSPEC_XFGVIRT(IMbnSmsConfiguration, get_SmsFormat)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SmsFormat )( 
            __RPC__in IMbnSmsConfiguration * This,
            /* [retval][ref][out] */ __RPC__out MBN_SMS_FORMAT *smsFormat);
        
        DECLSPEC_XFGVIRT(IMbnSmsConfiguration, put_SmsFormat)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SmsFormat )( 
            __RPC__in IMbnSmsConfiguration * This,
            /* [in] */ MBN_SMS_FORMAT smsFormat);
        
        END_INTERFACE
    } IMbnSmsConfigurationVtbl;

    interface IMbnSmsConfiguration
    {
        CONST_VTBL struct IMbnSmsConfigurationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMbnSmsConfiguration_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMbnSmsConfiguration_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMbnSmsConfiguration_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMbnSmsConfiguration_get_ServiceCenterAddress(This,scAddress)	\
    ( (This)->lpVtbl -> get_ServiceCenterAddress(This,scAddress) ) 

#define IMbnSmsConfiguration_put_ServiceCenterAddress(This,scAddress)	\
    ( (This)->lpVtbl -> put_ServiceCenterAddress(This,scAddress) ) 

#define IMbnSmsConfiguration_get_MaxMessageIndex(This,index)	\
    ( (This)->lpVtbl -> get_MaxMessageIndex(This,index) ) 

#define IMbnSmsConfiguration_get_CdmaShortMsgSize(This,shortMsgSize)	\
    ( (This)->lpVtbl -> get_CdmaShortMsgSize(This,shortMsgSize) ) 

#define IMbnSmsConfiguration_get_SmsFormat(This,smsFormat)	\
    ( (This)->lpVtbl -> get_SmsFormat(This,smsFormat) ) 

#define IMbnSmsConfiguration_put_SmsFormat(This,smsFormat)	\
    ( (This)->lpVtbl -> put_SmsFormat(This,smsFormat) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMbnSmsConfiguration_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mbnapi_0000_0023 */
/* [local] */ 

#pragma deprecated(IMbnSmsConfiguration) // Replaced by WinRT API in Windows.Devices.Sms namespace


extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0023_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0023_v0_0_s_ifspec;

#ifndef __IMbnSmsReadMsgPdu_INTERFACE_DEFINED__
#define __IMbnSmsReadMsgPdu_INTERFACE_DEFINED__

/* interface IMbnSmsReadMsgPdu */
/* [helpstring][uuid][oleautomation][nonextensible][object] */ 


EXTERN_C const IID IID_IMbnSmsReadMsgPdu;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DCBBBAB6-2013-4BBB-AAEE-338E368AF6FA")
    IMbnSmsReadMsgPdu : public IUnknown
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Index( 
            /* [retval][ref][out] */ __RPC__out ULONG *Index) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Status( 
            /* [retval][ref][out] */ __RPC__out MBN_MSG_STATUS *Status) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PduData( 
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *PduData) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Message( 
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *Message) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMbnSmsReadMsgPduVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMbnSmsReadMsgPdu * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMbnSmsReadMsgPdu * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMbnSmsReadMsgPdu * This);
        
        DECLSPEC_XFGVIRT(IMbnSmsReadMsgPdu, get_Index)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Index )( 
            __RPC__in IMbnSmsReadMsgPdu * This,
            /* [retval][ref][out] */ __RPC__out ULONG *Index);
        
        DECLSPEC_XFGVIRT(IMbnSmsReadMsgPdu, get_Status)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Status )( 
            __RPC__in IMbnSmsReadMsgPdu * This,
            /* [retval][ref][out] */ __RPC__out MBN_MSG_STATUS *Status);
        
        DECLSPEC_XFGVIRT(IMbnSmsReadMsgPdu, get_PduData)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PduData )( 
            __RPC__in IMbnSmsReadMsgPdu * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *PduData);
        
        DECLSPEC_XFGVIRT(IMbnSmsReadMsgPdu, get_Message)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Message )( 
            __RPC__in IMbnSmsReadMsgPdu * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *Message);
        
        END_INTERFACE
    } IMbnSmsReadMsgPduVtbl;

    interface IMbnSmsReadMsgPdu
    {
        CONST_VTBL struct IMbnSmsReadMsgPduVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMbnSmsReadMsgPdu_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMbnSmsReadMsgPdu_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMbnSmsReadMsgPdu_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMbnSmsReadMsgPdu_get_Index(This,Index)	\
    ( (This)->lpVtbl -> get_Index(This,Index) ) 

#define IMbnSmsReadMsgPdu_get_Status(This,Status)	\
    ( (This)->lpVtbl -> get_Status(This,Status) ) 

#define IMbnSmsReadMsgPdu_get_PduData(This,PduData)	\
    ( (This)->lpVtbl -> get_PduData(This,PduData) ) 

#define IMbnSmsReadMsgPdu_get_Message(This,Message)	\
    ( (This)->lpVtbl -> get_Message(This,Message) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMbnSmsReadMsgPdu_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mbnapi_0000_0024 */
/* [local] */ 

#pragma deprecated(IMbnSmsReadMsgPdu) // Replaced by WinRT API in Windows.Devices.Sms namespace


extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0024_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0024_v0_0_s_ifspec;

#ifndef __IMbnSmsReadMsgTextCdma_INTERFACE_DEFINED__
#define __IMbnSmsReadMsgTextCdma_INTERFACE_DEFINED__

/* interface IMbnSmsReadMsgTextCdma */
/* [helpstring][uuid][oleautomation][nonextensible][object] */ 


EXTERN_C const IID IID_IMbnSmsReadMsgTextCdma;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DCBBBAB6-2014-4BBB-AAEE-338E368AF6FA")
    IMbnSmsReadMsgTextCdma : public IUnknown
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Index( 
            /* [retval][ref][out] */ __RPC__out ULONG *Index) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Status( 
            /* [retval][ref][out] */ __RPC__out MBN_MSG_STATUS *Status) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Address( 
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *Address) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Timestamp( 
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *Timestamp) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_EncodingID( 
            /* [retval][ref][out] */ __RPC__out MBN_SMS_CDMA_ENCODING *EncodingID) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_LanguageID( 
            /* [retval][ref][out] */ __RPC__out MBN_SMS_CDMA_LANG *LanguageID) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SizeInCharacters( 
            /* [retval][ref][out] */ __RPC__out ULONG *SizeInCharacters) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Message( 
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *Message) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMbnSmsReadMsgTextCdmaVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMbnSmsReadMsgTextCdma * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMbnSmsReadMsgTextCdma * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMbnSmsReadMsgTextCdma * This);
        
        DECLSPEC_XFGVIRT(IMbnSmsReadMsgTextCdma, get_Index)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Index )( 
            __RPC__in IMbnSmsReadMsgTextCdma * This,
            /* [retval][ref][out] */ __RPC__out ULONG *Index);
        
        DECLSPEC_XFGVIRT(IMbnSmsReadMsgTextCdma, get_Status)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Status )( 
            __RPC__in IMbnSmsReadMsgTextCdma * This,
            /* [retval][ref][out] */ __RPC__out MBN_MSG_STATUS *Status);
        
        DECLSPEC_XFGVIRT(IMbnSmsReadMsgTextCdma, get_Address)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Address )( 
            __RPC__in IMbnSmsReadMsgTextCdma * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *Address);
        
        DECLSPEC_XFGVIRT(IMbnSmsReadMsgTextCdma, get_Timestamp)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Timestamp )( 
            __RPC__in IMbnSmsReadMsgTextCdma * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *Timestamp);
        
        DECLSPEC_XFGVIRT(IMbnSmsReadMsgTextCdma, get_EncodingID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EncodingID )( 
            __RPC__in IMbnSmsReadMsgTextCdma * This,
            /* [retval][ref][out] */ __RPC__out MBN_SMS_CDMA_ENCODING *EncodingID);
        
        DECLSPEC_XFGVIRT(IMbnSmsReadMsgTextCdma, get_LanguageID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LanguageID )( 
            __RPC__in IMbnSmsReadMsgTextCdma * This,
            /* [retval][ref][out] */ __RPC__out MBN_SMS_CDMA_LANG *LanguageID);
        
        DECLSPEC_XFGVIRT(IMbnSmsReadMsgTextCdma, get_SizeInCharacters)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SizeInCharacters )( 
            __RPC__in IMbnSmsReadMsgTextCdma * This,
            /* [retval][ref][out] */ __RPC__out ULONG *SizeInCharacters);
        
        DECLSPEC_XFGVIRT(IMbnSmsReadMsgTextCdma, get_Message)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Message )( 
            __RPC__in IMbnSmsReadMsgTextCdma * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *Message);
        
        END_INTERFACE
    } IMbnSmsReadMsgTextCdmaVtbl;

    interface IMbnSmsReadMsgTextCdma
    {
        CONST_VTBL struct IMbnSmsReadMsgTextCdmaVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMbnSmsReadMsgTextCdma_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMbnSmsReadMsgTextCdma_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMbnSmsReadMsgTextCdma_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMbnSmsReadMsgTextCdma_get_Index(This,Index)	\
    ( (This)->lpVtbl -> get_Index(This,Index) ) 

#define IMbnSmsReadMsgTextCdma_get_Status(This,Status)	\
    ( (This)->lpVtbl -> get_Status(This,Status) ) 

#define IMbnSmsReadMsgTextCdma_get_Address(This,Address)	\
    ( (This)->lpVtbl -> get_Address(This,Address) ) 

#define IMbnSmsReadMsgTextCdma_get_Timestamp(This,Timestamp)	\
    ( (This)->lpVtbl -> get_Timestamp(This,Timestamp) ) 

#define IMbnSmsReadMsgTextCdma_get_EncodingID(This,EncodingID)	\
    ( (This)->lpVtbl -> get_EncodingID(This,EncodingID) ) 

#define IMbnSmsReadMsgTextCdma_get_LanguageID(This,LanguageID)	\
    ( (This)->lpVtbl -> get_LanguageID(This,LanguageID) ) 

#define IMbnSmsReadMsgTextCdma_get_SizeInCharacters(This,SizeInCharacters)	\
    ( (This)->lpVtbl -> get_SizeInCharacters(This,SizeInCharacters) ) 

#define IMbnSmsReadMsgTextCdma_get_Message(This,Message)	\
    ( (This)->lpVtbl -> get_Message(This,Message) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMbnSmsReadMsgTextCdma_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mbnapi_0000_0025 */
/* [local] */ 

#pragma deprecated(IMbnSmsReadMsgTextCdma) // Replaced by WinRT API in Windows.Devices.Sms namespace


extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0025_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0025_v0_0_s_ifspec;

#ifndef __IMbnSms_INTERFACE_DEFINED__
#define __IMbnSms_INTERFACE_DEFINED__

/* interface IMbnSms */
/* [helpstring][uuid][oleautomation][nonextensible][object] */ 


EXTERN_C const IID IID_IMbnSms;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DCBBBAB6-2015-4BBB-AAEE-338E368AF6FA")
    IMbnSms : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetSmsConfiguration( 
            /* [retval][ref][out] */ __RPC__deref_out_opt IMbnSmsConfiguration **smsConfiguration) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetSmsConfiguration( 
            /* [in] */ __RPC__in_opt IMbnSmsConfiguration *smsConfiguration,
            /* [out] */ __RPC__out ULONG *requestID) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SmsSendPdu( 
            /* [in] */ __RPC__in LPCWSTR pduData,
            /* [in] */ BYTE size,
            /* [out] */ __RPC__out ULONG *requestID) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SmsSendCdma( 
            /* [in] */ __RPC__in LPCWSTR address,
            /* [in] */ MBN_SMS_CDMA_ENCODING encoding,
            /* [in] */ MBN_SMS_CDMA_LANG language,
            /* [in] */ ULONG sizeInCharacters,
            /* [in] */ __RPC__in SAFEARRAY * message,
            /* [out] */ __RPC__out ULONG *requestID) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SmsSendCdmaPdu( 
            /* [in] */ __RPC__in SAFEARRAY * message,
            /* [out] */ __RPC__out ULONG *requestID) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SmsRead( 
            /* [ref][in] */ __RPC__in MBN_SMS_FILTER *smsFilter,
            /* [in] */ MBN_SMS_FORMAT smsFormat,
            /* [out] */ __RPC__out ULONG *requestID) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SmsDelete( 
            /* [ref][in] */ __RPC__in MBN_SMS_FILTER *smsFilter,
            /* [out] */ __RPC__out ULONG *requestID) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetSmsStatus( 
            /* [ref][out] */ __RPC__out MBN_SMS_STATUS_INFO *smsStatusInfo) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMbnSmsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMbnSms * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMbnSms * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMbnSms * This);
        
        DECLSPEC_XFGVIRT(IMbnSms, GetSmsConfiguration)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetSmsConfiguration )( 
            __RPC__in IMbnSms * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt IMbnSmsConfiguration **smsConfiguration);
        
        DECLSPEC_XFGVIRT(IMbnSms, SetSmsConfiguration)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetSmsConfiguration )( 
            __RPC__in IMbnSms * This,
            /* [in] */ __RPC__in_opt IMbnSmsConfiguration *smsConfiguration,
            /* [out] */ __RPC__out ULONG *requestID);
        
        DECLSPEC_XFGVIRT(IMbnSms, SmsSendPdu)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SmsSendPdu )( 
            __RPC__in IMbnSms * This,
            /* [in] */ __RPC__in LPCWSTR pduData,
            /* [in] */ BYTE size,
            /* [out] */ __RPC__out ULONG *requestID);
        
        DECLSPEC_XFGVIRT(IMbnSms, SmsSendCdma)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SmsSendCdma )( 
            __RPC__in IMbnSms * This,
            /* [in] */ __RPC__in LPCWSTR address,
            /* [in] */ MBN_SMS_CDMA_ENCODING encoding,
            /* [in] */ MBN_SMS_CDMA_LANG language,
            /* [in] */ ULONG sizeInCharacters,
            /* [in] */ __RPC__in SAFEARRAY * message,
            /* [out] */ __RPC__out ULONG *requestID);
        
        DECLSPEC_XFGVIRT(IMbnSms, SmsSendCdmaPdu)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SmsSendCdmaPdu )( 
            __RPC__in IMbnSms * This,
            /* [in] */ __RPC__in SAFEARRAY * message,
            /* [out] */ __RPC__out ULONG *requestID);
        
        DECLSPEC_XFGVIRT(IMbnSms, SmsRead)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SmsRead )( 
            __RPC__in IMbnSms * This,
            /* [ref][in] */ __RPC__in MBN_SMS_FILTER *smsFilter,
            /* [in] */ MBN_SMS_FORMAT smsFormat,
            /* [out] */ __RPC__out ULONG *requestID);
        
        DECLSPEC_XFGVIRT(IMbnSms, SmsDelete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SmsDelete )( 
            __RPC__in IMbnSms * This,
            /* [ref][in] */ __RPC__in MBN_SMS_FILTER *smsFilter,
            /* [out] */ __RPC__out ULONG *requestID);
        
        DECLSPEC_XFGVIRT(IMbnSms, GetSmsStatus)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetSmsStatus )( 
            __RPC__in IMbnSms * This,
            /* [ref][out] */ __RPC__out MBN_SMS_STATUS_INFO *smsStatusInfo);
        
        END_INTERFACE
    } IMbnSmsVtbl;

    interface IMbnSms
    {
        CONST_VTBL struct IMbnSmsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMbnSms_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMbnSms_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMbnSms_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMbnSms_GetSmsConfiguration(This,smsConfiguration)	\
    ( (This)->lpVtbl -> GetSmsConfiguration(This,smsConfiguration) ) 

#define IMbnSms_SetSmsConfiguration(This,smsConfiguration,requestID)	\
    ( (This)->lpVtbl -> SetSmsConfiguration(This,smsConfiguration,requestID) ) 

#define IMbnSms_SmsSendPdu(This,pduData,size,requestID)	\
    ( (This)->lpVtbl -> SmsSendPdu(This,pduData,size,requestID) ) 

#define IMbnSms_SmsSendCdma(This,address,encoding,language,sizeInCharacters,message,requestID)	\
    ( (This)->lpVtbl -> SmsSendCdma(This,address,encoding,language,sizeInCharacters,message,requestID) ) 

#define IMbnSms_SmsSendCdmaPdu(This,message,requestID)	\
    ( (This)->lpVtbl -> SmsSendCdmaPdu(This,message,requestID) ) 

#define IMbnSms_SmsRead(This,smsFilter,smsFormat,requestID)	\
    ( (This)->lpVtbl -> SmsRead(This,smsFilter,smsFormat,requestID) ) 

#define IMbnSms_SmsDelete(This,smsFilter,requestID)	\
    ( (This)->lpVtbl -> SmsDelete(This,smsFilter,requestID) ) 

#define IMbnSms_GetSmsStatus(This,smsStatusInfo)	\
    ( (This)->lpVtbl -> GetSmsStatus(This,smsStatusInfo) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMbnSms_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mbnapi_0000_0026 */
/* [local] */ 

#pragma deprecated(IMbnSms) // Replaced by WinRT API in Windows.Devices.Sms namespace


extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0026_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0026_v0_0_s_ifspec;

#ifndef __IMbnSmsEvents_INTERFACE_DEFINED__
#define __IMbnSmsEvents_INTERFACE_DEFINED__

/* interface IMbnSmsEvents */
/* [helpstring][uuid][oleautomation][nonextensible][object] */ 


EXTERN_C const IID IID_IMbnSmsEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DCBBBAB6-2016-4BBB-AAEE-338E368AF6FA")
    IMbnSmsEvents : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnSmsConfigurationChange( 
            /* [in] */ __RPC__in_opt IMbnSms *sms) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnSetSmsConfigurationComplete( 
            /* [in] */ __RPC__in_opt IMbnSms *sms,
            /* [in] */ ULONG requestID,
            /* [in] */ HRESULT status) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnSmsSendComplete( 
            /* [in] */ __RPC__in_opt IMbnSms *sms,
            /* [in] */ ULONG requestID,
            /* [in] */ HRESULT status) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnSmsReadComplete( 
            /* [in] */ __RPC__in_opt IMbnSms *sms,
            /* [in] */ MBN_SMS_FORMAT smsFormat,
            /* [in] */ __RPC__in SAFEARRAY * readMsgs,
            /* [in] */ VARIANT_BOOL moreMsgs,
            /* [in] */ ULONG requestID,
            /* [in] */ HRESULT status) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnSmsNewClass0Message( 
            /* [in] */ __RPC__in_opt IMbnSms *sms,
            /* [in] */ MBN_SMS_FORMAT smsFormat,
            /* [in] */ __RPC__in SAFEARRAY * readMsgs) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnSmsDeleteComplete( 
            /* [in] */ __RPC__in_opt IMbnSms *sms,
            /* [in] */ ULONG requestID,
            /* [in] */ HRESULT status) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnSmsStatusChange( 
            /* [in] */ __RPC__in_opt IMbnSms *sms) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMbnSmsEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMbnSmsEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMbnSmsEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMbnSmsEvents * This);
        
        DECLSPEC_XFGVIRT(IMbnSmsEvents, OnSmsConfigurationChange)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnSmsConfigurationChange )( 
            __RPC__in IMbnSmsEvents * This,
            /* [in] */ __RPC__in_opt IMbnSms *sms);
        
        DECLSPEC_XFGVIRT(IMbnSmsEvents, OnSetSmsConfigurationComplete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnSetSmsConfigurationComplete )( 
            __RPC__in IMbnSmsEvents * This,
            /* [in] */ __RPC__in_opt IMbnSms *sms,
            /* [in] */ ULONG requestID,
            /* [in] */ HRESULT status);
        
        DECLSPEC_XFGVIRT(IMbnSmsEvents, OnSmsSendComplete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnSmsSendComplete )( 
            __RPC__in IMbnSmsEvents * This,
            /* [in] */ __RPC__in_opt IMbnSms *sms,
            /* [in] */ ULONG requestID,
            /* [in] */ HRESULT status);
        
        DECLSPEC_XFGVIRT(IMbnSmsEvents, OnSmsReadComplete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnSmsReadComplete )( 
            __RPC__in IMbnSmsEvents * This,
            /* [in] */ __RPC__in_opt IMbnSms *sms,
            /* [in] */ MBN_SMS_FORMAT smsFormat,
            /* [in] */ __RPC__in SAFEARRAY * readMsgs,
            /* [in] */ VARIANT_BOOL moreMsgs,
            /* [in] */ ULONG requestID,
            /* [in] */ HRESULT status);
        
        DECLSPEC_XFGVIRT(IMbnSmsEvents, OnSmsNewClass0Message)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnSmsNewClass0Message )( 
            __RPC__in IMbnSmsEvents * This,
            /* [in] */ __RPC__in_opt IMbnSms *sms,
            /* [in] */ MBN_SMS_FORMAT smsFormat,
            /* [in] */ __RPC__in SAFEARRAY * readMsgs);
        
        DECLSPEC_XFGVIRT(IMbnSmsEvents, OnSmsDeleteComplete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnSmsDeleteComplete )( 
            __RPC__in IMbnSmsEvents * This,
            /* [in] */ __RPC__in_opt IMbnSms *sms,
            /* [in] */ ULONG requestID,
            /* [in] */ HRESULT status);
        
        DECLSPEC_XFGVIRT(IMbnSmsEvents, OnSmsStatusChange)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnSmsStatusChange )( 
            __RPC__in IMbnSmsEvents * This,
            /* [in] */ __RPC__in_opt IMbnSms *sms);
        
        END_INTERFACE
    } IMbnSmsEventsVtbl;

    interface IMbnSmsEvents
    {
        CONST_VTBL struct IMbnSmsEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMbnSmsEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMbnSmsEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMbnSmsEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMbnSmsEvents_OnSmsConfigurationChange(This,sms)	\
    ( (This)->lpVtbl -> OnSmsConfigurationChange(This,sms) ) 

#define IMbnSmsEvents_OnSetSmsConfigurationComplete(This,sms,requestID,status)	\
    ( (This)->lpVtbl -> OnSetSmsConfigurationComplete(This,sms,requestID,status) ) 

#define IMbnSmsEvents_OnSmsSendComplete(This,sms,requestID,status)	\
    ( (This)->lpVtbl -> OnSmsSendComplete(This,sms,requestID,status) ) 

#define IMbnSmsEvents_OnSmsReadComplete(This,sms,smsFormat,readMsgs,moreMsgs,requestID,status)	\
    ( (This)->lpVtbl -> OnSmsReadComplete(This,sms,smsFormat,readMsgs,moreMsgs,requestID,status) ) 

#define IMbnSmsEvents_OnSmsNewClass0Message(This,sms,smsFormat,readMsgs)	\
    ( (This)->lpVtbl -> OnSmsNewClass0Message(This,sms,smsFormat,readMsgs) ) 

#define IMbnSmsEvents_OnSmsDeleteComplete(This,sms,requestID,status)	\
    ( (This)->lpVtbl -> OnSmsDeleteComplete(This,sms,requestID,status) ) 

#define IMbnSmsEvents_OnSmsStatusChange(This,sms)	\
    ( (This)->lpVtbl -> OnSmsStatusChange(This,sms) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMbnSmsEvents_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mbnapi_0000_0027 */
/* [local] */ 

#pragma deprecated(IMbnSmsEvents) // Replaced by WinRT API in Windows.Devices.Sms namespace


extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0027_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0027_v0_0_s_ifspec;

#ifndef __IMbnServiceActivation_INTERFACE_DEFINED__
#define __IMbnServiceActivation_INTERFACE_DEFINED__

/* interface IMbnServiceActivation */
/* [helpstring][uuid][oleautomation][nonextensible][object] */ 


EXTERN_C const IID IID_IMbnServiceActivation;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DCBBBAB6-2017-4BBB-AAEE-338E368AF6FA")
    IMbnServiceActivation : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Activate( 
            /* [in] */ __RPC__in SAFEARRAY * vendorSpecificData,
            /* [out] */ __RPC__out ULONG *requestID) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMbnServiceActivationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMbnServiceActivation * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMbnServiceActivation * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMbnServiceActivation * This);
        
        DECLSPEC_XFGVIRT(IMbnServiceActivation, Activate)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Activate )( 
            __RPC__in IMbnServiceActivation * This,
            /* [in] */ __RPC__in SAFEARRAY * vendorSpecificData,
            /* [out] */ __RPC__out ULONG *requestID);
        
        END_INTERFACE
    } IMbnServiceActivationVtbl;

    interface IMbnServiceActivation
    {
        CONST_VTBL struct IMbnServiceActivationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMbnServiceActivation_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMbnServiceActivation_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMbnServiceActivation_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMbnServiceActivation_Activate(This,vendorSpecificData,requestID)	\
    ( (This)->lpVtbl -> Activate(This,vendorSpecificData,requestID) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMbnServiceActivation_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mbnapi_0000_0028 */
/* [local] */ 

#pragma deprecated(IMbnServiceActivation) // Replaced by WinRT Windows.Networking.NetworkOperators.MobileBroadbandDeviceService


extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0028_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0028_v0_0_s_ifspec;

#ifndef __IMbnServiceActivationEvents_INTERFACE_DEFINED__
#define __IMbnServiceActivationEvents_INTERFACE_DEFINED__

/* interface IMbnServiceActivationEvents */
/* [helpstring][uuid][oleautomation][nonextensible][object] */ 


EXTERN_C const IID IID_IMbnServiceActivationEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DCBBBAB6-2018-4BBB-AAEE-338E368AF6FA")
    IMbnServiceActivationEvents : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnActivationComplete( 
            /* [in] */ __RPC__in_opt IMbnServiceActivation *serviceActivation,
            /* [in] */ __RPC__in SAFEARRAY * vendorSpecificData,
            /* [in] */ ULONG requestID,
            /* [in] */ HRESULT status,
            /* [in] */ ULONG networkError) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMbnServiceActivationEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMbnServiceActivationEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMbnServiceActivationEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMbnServiceActivationEvents * This);
        
        DECLSPEC_XFGVIRT(IMbnServiceActivationEvents, OnActivationComplete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnActivationComplete )( 
            __RPC__in IMbnServiceActivationEvents * This,
            /* [in] */ __RPC__in_opt IMbnServiceActivation *serviceActivation,
            /* [in] */ __RPC__in SAFEARRAY * vendorSpecificData,
            /* [in] */ ULONG requestID,
            /* [in] */ HRESULT status,
            /* [in] */ ULONG networkError);
        
        END_INTERFACE
    } IMbnServiceActivationEventsVtbl;

    interface IMbnServiceActivationEvents
    {
        CONST_VTBL struct IMbnServiceActivationEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMbnServiceActivationEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMbnServiceActivationEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMbnServiceActivationEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMbnServiceActivationEvents_OnActivationComplete(This,serviceActivation,vendorSpecificData,requestID,status,networkError)	\
    ( (This)->lpVtbl -> OnActivationComplete(This,serviceActivation,vendorSpecificData,requestID,status,networkError) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMbnServiceActivationEvents_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mbnapi_0000_0029 */
/* [local] */ 

#pragma deprecated(IMbnServiceActivationEvents) // Replaced by WinRT Windows.Networking.NetworkOperators.MobileBroadbandDeviceService


extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0029_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0029_v0_0_s_ifspec;

#ifndef __IMbnVendorSpecificOperation_INTERFACE_DEFINED__
#define __IMbnVendorSpecificOperation_INTERFACE_DEFINED__

/* interface IMbnVendorSpecificOperation */
/* [helpstring][uuid][oleautomation][nonextensible][object] */ 


EXTERN_C const IID IID_IMbnVendorSpecificOperation;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DCBBBAB6-2019-4BBB-AAEE-338E368AF6FA")
    IMbnVendorSpecificOperation : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetVendorSpecific( 
            /* [ref][in] */ __RPC__in SAFEARRAY * vendorSpecificData,
            /* [out] */ __RPC__out ULONG *requestID) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMbnVendorSpecificOperationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMbnVendorSpecificOperation * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMbnVendorSpecificOperation * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMbnVendorSpecificOperation * This);
        
        DECLSPEC_XFGVIRT(IMbnVendorSpecificOperation, SetVendorSpecific)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetVendorSpecific )( 
            __RPC__in IMbnVendorSpecificOperation * This,
            /* [ref][in] */ __RPC__in SAFEARRAY * vendorSpecificData,
            /* [out] */ __RPC__out ULONG *requestID);
        
        END_INTERFACE
    } IMbnVendorSpecificOperationVtbl;

    interface IMbnVendorSpecificOperation
    {
        CONST_VTBL struct IMbnVendorSpecificOperationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMbnVendorSpecificOperation_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMbnVendorSpecificOperation_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMbnVendorSpecificOperation_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMbnVendorSpecificOperation_SetVendorSpecific(This,vendorSpecificData,requestID)	\
    ( (This)->lpVtbl -> SetVendorSpecific(This,vendorSpecificData,requestID) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMbnVendorSpecificOperation_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mbnapi_0000_0030 */
/* [local] */ 

#pragma deprecated(IMbnVendorSpecificOperation) // Replaced by WinRT Windows.Networking.NetworkOperators.MobileBroadbandDeviceService


extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0030_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0030_v0_0_s_ifspec;

#ifndef __IMbnVendorSpecificEvents_INTERFACE_DEFINED__
#define __IMbnVendorSpecificEvents_INTERFACE_DEFINED__

/* interface IMbnVendorSpecificEvents */
/* [helpstring][uuid][oleautomation][nonextensible][object] */ 


EXTERN_C const IID IID_IMbnVendorSpecificEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DCBBBAB6-201A-4BBB-AAEE-338E368AF6FA")
    IMbnVendorSpecificEvents : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnEventNotification( 
            /* [in] */ __RPC__in_opt IMbnVendorSpecificOperation *vendorOperation,
            /* [in] */ __RPC__in SAFEARRAY * vendorSpecificData) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnSetVendorSpecificComplete( 
            /* [in] */ __RPC__in_opt IMbnVendorSpecificOperation *vendorOperation,
            /* [in] */ __RPC__in SAFEARRAY * vendorSpecificData,
            /* [in] */ ULONG requestID) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMbnVendorSpecificEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMbnVendorSpecificEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMbnVendorSpecificEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMbnVendorSpecificEvents * This);
        
        DECLSPEC_XFGVIRT(IMbnVendorSpecificEvents, OnEventNotification)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnEventNotification )( 
            __RPC__in IMbnVendorSpecificEvents * This,
            /* [in] */ __RPC__in_opt IMbnVendorSpecificOperation *vendorOperation,
            /* [in] */ __RPC__in SAFEARRAY * vendorSpecificData);
        
        DECLSPEC_XFGVIRT(IMbnVendorSpecificEvents, OnSetVendorSpecificComplete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnSetVendorSpecificComplete )( 
            __RPC__in IMbnVendorSpecificEvents * This,
            /* [in] */ __RPC__in_opt IMbnVendorSpecificOperation *vendorOperation,
            /* [in] */ __RPC__in SAFEARRAY * vendorSpecificData,
            /* [in] */ ULONG requestID);
        
        END_INTERFACE
    } IMbnVendorSpecificEventsVtbl;

    interface IMbnVendorSpecificEvents
    {
        CONST_VTBL struct IMbnVendorSpecificEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMbnVendorSpecificEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMbnVendorSpecificEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMbnVendorSpecificEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMbnVendorSpecificEvents_OnEventNotification(This,vendorOperation,vendorSpecificData)	\
    ( (This)->lpVtbl -> OnEventNotification(This,vendorOperation,vendorSpecificData) ) 

#define IMbnVendorSpecificEvents_OnSetVendorSpecificComplete(This,vendorOperation,vendorSpecificData,requestID)	\
    ( (This)->lpVtbl -> OnSetVendorSpecificComplete(This,vendorOperation,vendorSpecificData,requestID) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMbnVendorSpecificEvents_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mbnapi_0000_0031 */
/* [local] */ 

#pragma deprecated(IMbnVendorSpecificEvents) // Replaced by WinRT Windows.Networking.NetworkOperators.MobileBroadbandDeviceService


extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0031_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0031_v0_0_s_ifspec;

#ifndef __IMbnConnectionProfileManagerEvents_INTERFACE_DEFINED__
#define __IMbnConnectionProfileManagerEvents_INTERFACE_DEFINED__

/* interface IMbnConnectionProfileManagerEvents */
/* [helpstring][uuid][oleautomation][nonextensible][object] */ 


EXTERN_C const IID IID_IMbnConnectionProfileManagerEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DCBBBAB6-201F-4BBB-AAEE-338E368AF6FA")
    IMbnConnectionProfileManagerEvents : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnConnectionProfileArrival( 
            /* [in] */ __RPC__in_opt IMbnConnectionProfile *newConnectionProfile) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnConnectionProfileRemoval( 
            /* [in] */ __RPC__in_opt IMbnConnectionProfile *oldConnectionProfile) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMbnConnectionProfileManagerEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMbnConnectionProfileManagerEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMbnConnectionProfileManagerEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMbnConnectionProfileManagerEvents * This);
        
        DECLSPEC_XFGVIRT(IMbnConnectionProfileManagerEvents, OnConnectionProfileArrival)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnConnectionProfileArrival )( 
            __RPC__in IMbnConnectionProfileManagerEvents * This,
            /* [in] */ __RPC__in_opt IMbnConnectionProfile *newConnectionProfile);
        
        DECLSPEC_XFGVIRT(IMbnConnectionProfileManagerEvents, OnConnectionProfileRemoval)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnConnectionProfileRemoval )( 
            __RPC__in IMbnConnectionProfileManagerEvents * This,
            /* [in] */ __RPC__in_opt IMbnConnectionProfile *oldConnectionProfile);
        
        END_INTERFACE
    } IMbnConnectionProfileManagerEventsVtbl;

    interface IMbnConnectionProfileManagerEvents
    {
        CONST_VTBL struct IMbnConnectionProfileManagerEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMbnConnectionProfileManagerEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMbnConnectionProfileManagerEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMbnConnectionProfileManagerEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMbnConnectionProfileManagerEvents_OnConnectionProfileArrival(This,newConnectionProfile)	\
    ( (This)->lpVtbl -> OnConnectionProfileArrival(This,newConnectionProfile) ) 

#define IMbnConnectionProfileManagerEvents_OnConnectionProfileRemoval(This,oldConnectionProfile)	\
    ( (This)->lpVtbl -> OnConnectionProfileRemoval(This,oldConnectionProfile) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMbnConnectionProfileManagerEvents_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mbnapi_0000_0032 */
/* [local] */ 

#pragma deprecated(IMbnConnectionProfileManagerEvents) // Replaced by WinRT API in Windows.Networking.Connectivity and Windows.Networking.NetworkOperators namespaces


extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0032_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0032_v0_0_s_ifspec;

#ifndef __IMbnRadio_INTERFACE_DEFINED__
#define __IMbnRadio_INTERFACE_DEFINED__

/* interface IMbnRadio */
/* [helpstring][uuid][oleautomation][nonextensible][object] */ 


EXTERN_C const IID IID_IMbnRadio;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DCCCCAB6-201F-4BBB-AAEE-338E368AF6FA")
    IMbnRadio : public IUnknown
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SoftwareRadioState( 
            /* [retval][ref][out] */ __RPC__out MBN_RADIO *SoftwareRadioState) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_HardwareRadioState( 
            /* [retval][ref][out] */ __RPC__out MBN_RADIO *HardwareRadioState) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetSoftwareRadioState( 
            /* [in] */ MBN_RADIO radioState,
            /* [out] */ __RPC__out ULONG *requestID) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMbnRadioVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMbnRadio * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMbnRadio * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMbnRadio * This);
        
        DECLSPEC_XFGVIRT(IMbnRadio, get_SoftwareRadioState)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SoftwareRadioState )( 
            __RPC__in IMbnRadio * This,
            /* [retval][ref][out] */ __RPC__out MBN_RADIO *SoftwareRadioState);
        
        DECLSPEC_XFGVIRT(IMbnRadio, get_HardwareRadioState)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_HardwareRadioState )( 
            __RPC__in IMbnRadio * This,
            /* [retval][ref][out] */ __RPC__out MBN_RADIO *HardwareRadioState);
        
        DECLSPEC_XFGVIRT(IMbnRadio, SetSoftwareRadioState)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetSoftwareRadioState )( 
            __RPC__in IMbnRadio * This,
            /* [in] */ MBN_RADIO radioState,
            /* [out] */ __RPC__out ULONG *requestID);
        
        END_INTERFACE
    } IMbnRadioVtbl;

    interface IMbnRadio
    {
        CONST_VTBL struct IMbnRadioVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMbnRadio_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMbnRadio_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMbnRadio_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMbnRadio_get_SoftwareRadioState(This,SoftwareRadioState)	\
    ( (This)->lpVtbl -> get_SoftwareRadioState(This,SoftwareRadioState) ) 

#define IMbnRadio_get_HardwareRadioState(This,HardwareRadioState)	\
    ( (This)->lpVtbl -> get_HardwareRadioState(This,HardwareRadioState) ) 

#define IMbnRadio_SetSoftwareRadioState(This,radioState,requestID)	\
    ( (This)->lpVtbl -> SetSoftwareRadioState(This,radioState,requestID) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMbnRadio_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mbnapi_0000_0033 */
/* [local] */ 

#pragma deprecated(IMbnRadio) // Replaced by WinRT API in Windows.Networking.Connectivity and Windows.Networking.NetworkOperators namespaces


extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0033_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0033_v0_0_s_ifspec;

#ifndef __IMbnRadioEvents_INTERFACE_DEFINED__
#define __IMbnRadioEvents_INTERFACE_DEFINED__

/* interface IMbnRadioEvents */
/* [helpstring][uuid][oleautomation][nonextensible][object] */ 


EXTERN_C const IID IID_IMbnRadioEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DCDDDAB6-201F-4BBB-AAEE-338E368AF6FA")
    IMbnRadioEvents : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnRadioStateChange( 
            /* [in] */ __RPC__in_opt IMbnRadio *newInterface) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnSetSoftwareRadioStateComplete( 
            /* [in] */ __RPC__in_opt IMbnRadio *newInterface,
            /* [in] */ ULONG requestID,
            /* [in] */ HRESULT status) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMbnRadioEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMbnRadioEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMbnRadioEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMbnRadioEvents * This);
        
        DECLSPEC_XFGVIRT(IMbnRadioEvents, OnRadioStateChange)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnRadioStateChange )( 
            __RPC__in IMbnRadioEvents * This,
            /* [in] */ __RPC__in_opt IMbnRadio *newInterface);
        
        DECLSPEC_XFGVIRT(IMbnRadioEvents, OnSetSoftwareRadioStateComplete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnSetSoftwareRadioStateComplete )( 
            __RPC__in IMbnRadioEvents * This,
            /* [in] */ __RPC__in_opt IMbnRadio *newInterface,
            /* [in] */ ULONG requestID,
            /* [in] */ HRESULT status);
        
        END_INTERFACE
    } IMbnRadioEventsVtbl;

    interface IMbnRadioEvents
    {
        CONST_VTBL struct IMbnRadioEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMbnRadioEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMbnRadioEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMbnRadioEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMbnRadioEvents_OnRadioStateChange(This,newInterface)	\
    ( (This)->lpVtbl -> OnRadioStateChange(This,newInterface) ) 

#define IMbnRadioEvents_OnSetSoftwareRadioStateComplete(This,newInterface,requestID,status)	\
    ( (This)->lpVtbl -> OnSetSoftwareRadioStateComplete(This,newInterface,requestID,status) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMbnRadioEvents_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mbnapi_0000_0034 */
/* [local] */ 

#pragma deprecated(IMbnRadioEvents) // Replaced by WinRT API in Windows.Networking.Connectivity and Windows.Networking.NetworkOperators namespaces


extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0034_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0034_v0_0_s_ifspec;

#ifndef __IMbnMultiCarrier_INTERFACE_DEFINED__
#define __IMbnMultiCarrier_INTERFACE_DEFINED__

/* interface IMbnMultiCarrier */
/* [helpstring][uuid][oleautomation][nonextensible][object] */ 


EXTERN_C const IID IID_IMbnMultiCarrier;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DCBBBAB6-2020-4BBB-AAEE-338E368AF6FA")
    IMbnMultiCarrier : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetHomeProvider( 
            /* [in] */ __RPC__in MBN_PROVIDER2 *homeProvider,
            /* [out] */ __RPC__out ULONG *requestID) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetPreferredProviders( 
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *preferredMulticarrierProviders) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetVisibleProviders( 
            /* [out] */ __RPC__out ULONG *age,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *visibleProviders) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetSupportedCellularClasses( 
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *cellularClasses) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetCurrentCellularClass( 
            /* [retval][ref][out] */ __RPC__out MBN_CELLULAR_CLASS *currentCellularClass) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ScanNetwork( 
            /* [out] */ __RPC__out ULONG *requestID) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMbnMultiCarrierVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMbnMultiCarrier * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMbnMultiCarrier * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMbnMultiCarrier * This);
        
        DECLSPEC_XFGVIRT(IMbnMultiCarrier, SetHomeProvider)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetHomeProvider )( 
            __RPC__in IMbnMultiCarrier * This,
            /* [in] */ __RPC__in MBN_PROVIDER2 *homeProvider,
            /* [out] */ __RPC__out ULONG *requestID);
        
        DECLSPEC_XFGVIRT(IMbnMultiCarrier, GetPreferredProviders)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetPreferredProviders )( 
            __RPC__in IMbnMultiCarrier * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *preferredMulticarrierProviders);
        
        DECLSPEC_XFGVIRT(IMbnMultiCarrier, GetVisibleProviders)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetVisibleProviders )( 
            __RPC__in IMbnMultiCarrier * This,
            /* [out] */ __RPC__out ULONG *age,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *visibleProviders);
        
        DECLSPEC_XFGVIRT(IMbnMultiCarrier, GetSupportedCellularClasses)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetSupportedCellularClasses )( 
            __RPC__in IMbnMultiCarrier * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt SAFEARRAY * *cellularClasses);
        
        DECLSPEC_XFGVIRT(IMbnMultiCarrier, GetCurrentCellularClass)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetCurrentCellularClass )( 
            __RPC__in IMbnMultiCarrier * This,
            /* [retval][ref][out] */ __RPC__out MBN_CELLULAR_CLASS *currentCellularClass);
        
        DECLSPEC_XFGVIRT(IMbnMultiCarrier, ScanNetwork)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ScanNetwork )( 
            __RPC__in IMbnMultiCarrier * This,
            /* [out] */ __RPC__out ULONG *requestID);
        
        END_INTERFACE
    } IMbnMultiCarrierVtbl;

    interface IMbnMultiCarrier
    {
        CONST_VTBL struct IMbnMultiCarrierVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMbnMultiCarrier_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMbnMultiCarrier_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMbnMultiCarrier_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMbnMultiCarrier_SetHomeProvider(This,homeProvider,requestID)	\
    ( (This)->lpVtbl -> SetHomeProvider(This,homeProvider,requestID) ) 

#define IMbnMultiCarrier_GetPreferredProviders(This,preferredMulticarrierProviders)	\
    ( (This)->lpVtbl -> GetPreferredProviders(This,preferredMulticarrierProviders) ) 

#define IMbnMultiCarrier_GetVisibleProviders(This,age,visibleProviders)	\
    ( (This)->lpVtbl -> GetVisibleProviders(This,age,visibleProviders) ) 

#define IMbnMultiCarrier_GetSupportedCellularClasses(This,cellularClasses)	\
    ( (This)->lpVtbl -> GetSupportedCellularClasses(This,cellularClasses) ) 

#define IMbnMultiCarrier_GetCurrentCellularClass(This,currentCellularClass)	\
    ( (This)->lpVtbl -> GetCurrentCellularClass(This,currentCellularClass) ) 

#define IMbnMultiCarrier_ScanNetwork(This,requestID)	\
    ( (This)->lpVtbl -> ScanNetwork(This,requestID) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMbnMultiCarrier_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mbnapi_0000_0035 */
/* [local] */ 

#pragma deprecated(IMbnMultiCarrier) // Replaced by WinRT API in Windows.Networking.Connectivity and Windows.Networking.NetworkOperators namespaces


extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0035_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0035_v0_0_s_ifspec;

#ifndef __IMbnMultiCarrierEvents_INTERFACE_DEFINED__
#define __IMbnMultiCarrierEvents_INTERFACE_DEFINED__

/* interface IMbnMultiCarrierEvents */
/* [helpstring][uuid][oleautomation][nonextensible][object] */ 


EXTERN_C const IID IID_IMbnMultiCarrierEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DCDDDAB6-2021-4BBB-AAEE-338E368AF6FA")
    IMbnMultiCarrierEvents : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnSetHomeProviderComplete( 
            /* [in] */ __RPC__in_opt IMbnMultiCarrier *mbnInterface,
            /* [in] */ ULONG requestID,
            /* [in] */ HRESULT status) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnCurrentCellularClassChange( 
            /* [in] */ __RPC__in_opt IMbnMultiCarrier *mbnInterface) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnPreferredProvidersChange( 
            /* [in] */ __RPC__in_opt IMbnMultiCarrier *mbnInterface) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnScanNetworkComplete( 
            /* [in] */ __RPC__in_opt IMbnMultiCarrier *mbnInterface,
            /* [in] */ ULONG requestID,
            /* [in] */ HRESULT status) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnInterfaceCapabilityChange( 
            /* [in] */ __RPC__in_opt IMbnMultiCarrier *mbnInterface) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMbnMultiCarrierEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMbnMultiCarrierEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMbnMultiCarrierEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMbnMultiCarrierEvents * This);
        
        DECLSPEC_XFGVIRT(IMbnMultiCarrierEvents, OnSetHomeProviderComplete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnSetHomeProviderComplete )( 
            __RPC__in IMbnMultiCarrierEvents * This,
            /* [in] */ __RPC__in_opt IMbnMultiCarrier *mbnInterface,
            /* [in] */ ULONG requestID,
            /* [in] */ HRESULT status);
        
        DECLSPEC_XFGVIRT(IMbnMultiCarrierEvents, OnCurrentCellularClassChange)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnCurrentCellularClassChange )( 
            __RPC__in IMbnMultiCarrierEvents * This,
            /* [in] */ __RPC__in_opt IMbnMultiCarrier *mbnInterface);
        
        DECLSPEC_XFGVIRT(IMbnMultiCarrierEvents, OnPreferredProvidersChange)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnPreferredProvidersChange )( 
            __RPC__in IMbnMultiCarrierEvents * This,
            /* [in] */ __RPC__in_opt IMbnMultiCarrier *mbnInterface);
        
        DECLSPEC_XFGVIRT(IMbnMultiCarrierEvents, OnScanNetworkComplete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnScanNetworkComplete )( 
            __RPC__in IMbnMultiCarrierEvents * This,
            /* [in] */ __RPC__in_opt IMbnMultiCarrier *mbnInterface,
            /* [in] */ ULONG requestID,
            /* [in] */ HRESULT status);
        
        DECLSPEC_XFGVIRT(IMbnMultiCarrierEvents, OnInterfaceCapabilityChange)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnInterfaceCapabilityChange )( 
            __RPC__in IMbnMultiCarrierEvents * This,
            /* [in] */ __RPC__in_opt IMbnMultiCarrier *mbnInterface);
        
        END_INTERFACE
    } IMbnMultiCarrierEventsVtbl;

    interface IMbnMultiCarrierEvents
    {
        CONST_VTBL struct IMbnMultiCarrierEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMbnMultiCarrierEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMbnMultiCarrierEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMbnMultiCarrierEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMbnMultiCarrierEvents_OnSetHomeProviderComplete(This,mbnInterface,requestID,status)	\
    ( (This)->lpVtbl -> OnSetHomeProviderComplete(This,mbnInterface,requestID,status) ) 

#define IMbnMultiCarrierEvents_OnCurrentCellularClassChange(This,mbnInterface)	\
    ( (This)->lpVtbl -> OnCurrentCellularClassChange(This,mbnInterface) ) 

#define IMbnMultiCarrierEvents_OnPreferredProvidersChange(This,mbnInterface)	\
    ( (This)->lpVtbl -> OnPreferredProvidersChange(This,mbnInterface) ) 

#define IMbnMultiCarrierEvents_OnScanNetworkComplete(This,mbnInterface,requestID,status)	\
    ( (This)->lpVtbl -> OnScanNetworkComplete(This,mbnInterface,requestID,status) ) 

#define IMbnMultiCarrierEvents_OnInterfaceCapabilityChange(This,mbnInterface)	\
    ( (This)->lpVtbl -> OnInterfaceCapabilityChange(This,mbnInterface) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMbnMultiCarrierEvents_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mbnapi_0000_0036 */
/* [local] */ 

#pragma deprecated(IMbnMultiCarrierEvents) // Replaced by WinRT API in Windows.Networking.Connectivity and Windows.Networking.NetworkOperators namespaces


extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0036_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0036_v0_0_s_ifspec;

#ifndef __IMbnDeviceServiceStateEvents_INTERFACE_DEFINED__
#define __IMbnDeviceServiceStateEvents_INTERFACE_DEFINED__

/* interface IMbnDeviceServiceStateEvents */
/* [helpstring][uuid][oleautomation][nonextensible][object] */ 


EXTERN_C const IID IID_IMbnDeviceServiceStateEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5D3FF196-89EE-49D8-8B60-33FFDDFFC58D")
    IMbnDeviceServiceStateEvents : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnSessionsStateChange( 
            /* [in] */ __RPC__in BSTR interfaceID,
            /* [in] */ MBN_DEVICE_SERVICE_SESSIONS_STATE stateChange) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMbnDeviceServiceStateEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMbnDeviceServiceStateEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMbnDeviceServiceStateEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMbnDeviceServiceStateEvents * This);
        
        DECLSPEC_XFGVIRT(IMbnDeviceServiceStateEvents, OnSessionsStateChange)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnSessionsStateChange )( 
            __RPC__in IMbnDeviceServiceStateEvents * This,
            /* [in] */ __RPC__in BSTR interfaceID,
            /* [in] */ MBN_DEVICE_SERVICE_SESSIONS_STATE stateChange);
        
        END_INTERFACE
    } IMbnDeviceServiceStateEventsVtbl;

    interface IMbnDeviceServiceStateEvents
    {
        CONST_VTBL struct IMbnDeviceServiceStateEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMbnDeviceServiceStateEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMbnDeviceServiceStateEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMbnDeviceServiceStateEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMbnDeviceServiceStateEvents_OnSessionsStateChange(This,interfaceID,stateChange)	\
    ( (This)->lpVtbl -> OnSessionsStateChange(This,interfaceID,stateChange) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMbnDeviceServiceStateEvents_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mbnapi_0000_0037 */
/* [local] */ 

#pragma deprecated(IMbnDeviceServiceStateEvents) // Replaced by WinRT Windows.Networking.NetworkOperators.MobileBroadbandDeviceService
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)


extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0037_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0037_v0_0_s_ifspec;

#ifndef __IMbnDeviceServicesManager_INTERFACE_DEFINED__
#define __IMbnDeviceServicesManager_INTERFACE_DEFINED__

/* interface IMbnDeviceServicesManager */
/* [helpstring][uuid][oleautomation][nonextensible][object] */ 


EXTERN_C const IID IID_IMbnDeviceServicesManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("20A26258-6811-4478-AC1D-13324E45E41C")
    IMbnDeviceServicesManager : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetDeviceServicesContext( 
            /* [in] */ __RPC__in BSTR networkInterfaceID,
            /* [retval][ref][out] */ __RPC__deref_out_opt IMbnDeviceServicesContext **mbnDevicesContext) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMbnDeviceServicesManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMbnDeviceServicesManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMbnDeviceServicesManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMbnDeviceServicesManager * This);
        
        DECLSPEC_XFGVIRT(IMbnDeviceServicesManager, GetDeviceServicesContext)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetDeviceServicesContext )( 
            __RPC__in IMbnDeviceServicesManager * This,
            /* [in] */ __RPC__in BSTR networkInterfaceID,
            /* [retval][ref][out] */ __RPC__deref_out_opt IMbnDeviceServicesContext **mbnDevicesContext);
        
        END_INTERFACE
    } IMbnDeviceServicesManagerVtbl;

    interface IMbnDeviceServicesManager
    {
        CONST_VTBL struct IMbnDeviceServicesManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMbnDeviceServicesManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMbnDeviceServicesManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMbnDeviceServicesManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMbnDeviceServicesManager_GetDeviceServicesContext(This,networkInterfaceID,mbnDevicesContext)	\
    ( (This)->lpVtbl -> GetDeviceServicesContext(This,networkInterfaceID,mbnDevicesContext) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMbnDeviceServicesManager_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mbnapi_0000_0038 */
/* [local] */ 

#pragma deprecated(IMbnDeviceServicesManager) // Replaced by WinRT Windows.Networking.NetworkOperators.MobileBroadbandDeviceService


extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0038_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0038_v0_0_s_ifspec;

#ifndef __IMbnDeviceServicesContext_INTERFACE_DEFINED__
#define __IMbnDeviceServicesContext_INTERFACE_DEFINED__

/* interface IMbnDeviceServicesContext */
/* [helpstring][uuid][oleautomation][nonextensible][object] */ 


EXTERN_C const IID IID_IMbnDeviceServicesContext;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("FC5AC347-1592-4068-80BB-6A57580150D8")
    IMbnDeviceServicesContext : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE EnumerateDeviceServices( 
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *deviceServices) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetDeviceService( 
            /* [in] */ __RPC__in BSTR deviceServiceID,
            /* [retval][ref][out] */ __RPC__deref_out_opt IMbnDeviceService **mbnDeviceService) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MaxCommandSize( 
            /* [retval][ref][out] */ __RPC__out ULONG *maxCommandSize) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MaxDataSize( 
            /* [retval][ref][out] */ __RPC__out ULONG *maxDataSize) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMbnDeviceServicesContextVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMbnDeviceServicesContext * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMbnDeviceServicesContext * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMbnDeviceServicesContext * This);
        
        DECLSPEC_XFGVIRT(IMbnDeviceServicesContext, EnumerateDeviceServices)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *EnumerateDeviceServices )( 
            __RPC__in IMbnDeviceServicesContext * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *deviceServices);
        
        DECLSPEC_XFGVIRT(IMbnDeviceServicesContext, GetDeviceService)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetDeviceService )( 
            __RPC__in IMbnDeviceServicesContext * This,
            /* [in] */ __RPC__in BSTR deviceServiceID,
            /* [retval][ref][out] */ __RPC__deref_out_opt IMbnDeviceService **mbnDeviceService);
        
        DECLSPEC_XFGVIRT(IMbnDeviceServicesContext, get_MaxCommandSize)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MaxCommandSize )( 
            __RPC__in IMbnDeviceServicesContext * This,
            /* [retval][ref][out] */ __RPC__out ULONG *maxCommandSize);
        
        DECLSPEC_XFGVIRT(IMbnDeviceServicesContext, get_MaxDataSize)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MaxDataSize )( 
            __RPC__in IMbnDeviceServicesContext * This,
            /* [retval][ref][out] */ __RPC__out ULONG *maxDataSize);
        
        END_INTERFACE
    } IMbnDeviceServicesContextVtbl;

    interface IMbnDeviceServicesContext
    {
        CONST_VTBL struct IMbnDeviceServicesContextVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMbnDeviceServicesContext_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMbnDeviceServicesContext_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMbnDeviceServicesContext_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMbnDeviceServicesContext_EnumerateDeviceServices(This,deviceServices)	\
    ( (This)->lpVtbl -> EnumerateDeviceServices(This,deviceServices) ) 

#define IMbnDeviceServicesContext_GetDeviceService(This,deviceServiceID,mbnDeviceService)	\
    ( (This)->lpVtbl -> GetDeviceService(This,deviceServiceID,mbnDeviceService) ) 

#define IMbnDeviceServicesContext_get_MaxCommandSize(This,maxCommandSize)	\
    ( (This)->lpVtbl -> get_MaxCommandSize(This,maxCommandSize) ) 

#define IMbnDeviceServicesContext_get_MaxDataSize(This,maxDataSize)	\
    ( (This)->lpVtbl -> get_MaxDataSize(This,maxDataSize) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMbnDeviceServicesContext_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mbnapi_0000_0039 */
/* [local] */ 

#pragma deprecated(IMbnDeviceServicesContext) // Replaced by WinRT Windows.Networking.NetworkOperators.MobileBroadbandDeviceService


extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0039_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0039_v0_0_s_ifspec;

#ifndef __IMbnDeviceServicesEvents_INTERFACE_DEFINED__
#define __IMbnDeviceServicesEvents_INTERFACE_DEFINED__

/* interface IMbnDeviceServicesEvents */
/* [helpstring][uuid][oleautomation][nonextensible][object] */ 


EXTERN_C const IID IID_IMbnDeviceServicesEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0A900C19-6824-4E97-B76E-CF239D0CA642")
    IMbnDeviceServicesEvents : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnQuerySupportedCommandsComplete( 
            /* [in] */ __RPC__in_opt IMbnDeviceService *deviceService,
            /* [in] */ __RPC__in SAFEARRAY * commandIDList,
            /* [in] */ HRESULT status,
            /* [in] */ ULONG requestID) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnOpenCommandSessionComplete( 
            /* [in] */ __RPC__in_opt IMbnDeviceService *deviceService,
            /* [in] */ HRESULT status,
            /* [in] */ ULONG requestID) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnCloseCommandSessionComplete( 
            /* [in] */ __RPC__in_opt IMbnDeviceService *deviceService,
            /* [in] */ HRESULT status,
            /* [in] */ ULONG requestID) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnSetCommandComplete( 
            /* [in] */ __RPC__in_opt IMbnDeviceService *deviceService,
            /* [in] */ ULONG responseID,
            /* [in] */ __RPC__in SAFEARRAY * deviceServiceData,
            /* [in] */ HRESULT status,
            /* [in] */ ULONG requestID) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnQueryCommandComplete( 
            /* [in] */ __RPC__in_opt IMbnDeviceService *deviceService,
            /* [in] */ ULONG responseID,
            /* [in] */ __RPC__in SAFEARRAY * deviceServiceData,
            /* [in] */ HRESULT status,
            /* [in] */ ULONG requestID) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnEventNotification( 
            /* [in] */ __RPC__in_opt IMbnDeviceService *deviceService,
            /* [in] */ ULONG eventID,
            /* [in] */ __RPC__in SAFEARRAY * deviceServiceData) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnOpenDataSessionComplete( 
            /* [in] */ __RPC__in_opt IMbnDeviceService *deviceService,
            /* [in] */ HRESULT status,
            /* [in] */ ULONG requestID) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnCloseDataSessionComplete( 
            /* [in] */ __RPC__in_opt IMbnDeviceService *deviceService,
            /* [in] */ HRESULT status,
            /* [in] */ ULONG requestID) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnWriteDataComplete( 
            /* [in] */ __RPC__in_opt IMbnDeviceService *deviceService,
            /* [in] */ HRESULT status,
            /* [in] */ ULONG requestID) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnReadData( 
            /* [in] */ __RPC__in_opt IMbnDeviceService *deviceService,
            /* [in] */ __RPC__in SAFEARRAY * deviceServiceData) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OnInterfaceStateChange( 
            /* [in] */ __RPC__in BSTR interfaceID,
            /* [in] */ MBN_DEVICE_SERVICES_INTERFACE_STATE stateChange) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMbnDeviceServicesEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMbnDeviceServicesEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMbnDeviceServicesEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMbnDeviceServicesEvents * This);
        
        DECLSPEC_XFGVIRT(IMbnDeviceServicesEvents, OnQuerySupportedCommandsComplete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnQuerySupportedCommandsComplete )( 
            __RPC__in IMbnDeviceServicesEvents * This,
            /* [in] */ __RPC__in_opt IMbnDeviceService *deviceService,
            /* [in] */ __RPC__in SAFEARRAY * commandIDList,
            /* [in] */ HRESULT status,
            /* [in] */ ULONG requestID);
        
        DECLSPEC_XFGVIRT(IMbnDeviceServicesEvents, OnOpenCommandSessionComplete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnOpenCommandSessionComplete )( 
            __RPC__in IMbnDeviceServicesEvents * This,
            /* [in] */ __RPC__in_opt IMbnDeviceService *deviceService,
            /* [in] */ HRESULT status,
            /* [in] */ ULONG requestID);
        
        DECLSPEC_XFGVIRT(IMbnDeviceServicesEvents, OnCloseCommandSessionComplete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnCloseCommandSessionComplete )( 
            __RPC__in IMbnDeviceServicesEvents * This,
            /* [in] */ __RPC__in_opt IMbnDeviceService *deviceService,
            /* [in] */ HRESULT status,
            /* [in] */ ULONG requestID);
        
        DECLSPEC_XFGVIRT(IMbnDeviceServicesEvents, OnSetCommandComplete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnSetCommandComplete )( 
            __RPC__in IMbnDeviceServicesEvents * This,
            /* [in] */ __RPC__in_opt IMbnDeviceService *deviceService,
            /* [in] */ ULONG responseID,
            /* [in] */ __RPC__in SAFEARRAY * deviceServiceData,
            /* [in] */ HRESULT status,
            /* [in] */ ULONG requestID);
        
        DECLSPEC_XFGVIRT(IMbnDeviceServicesEvents, OnQueryCommandComplete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnQueryCommandComplete )( 
            __RPC__in IMbnDeviceServicesEvents * This,
            /* [in] */ __RPC__in_opt IMbnDeviceService *deviceService,
            /* [in] */ ULONG responseID,
            /* [in] */ __RPC__in SAFEARRAY * deviceServiceData,
            /* [in] */ HRESULT status,
            /* [in] */ ULONG requestID);
        
        DECLSPEC_XFGVIRT(IMbnDeviceServicesEvents, OnEventNotification)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnEventNotification )( 
            __RPC__in IMbnDeviceServicesEvents * This,
            /* [in] */ __RPC__in_opt IMbnDeviceService *deviceService,
            /* [in] */ ULONG eventID,
            /* [in] */ __RPC__in SAFEARRAY * deviceServiceData);
        
        DECLSPEC_XFGVIRT(IMbnDeviceServicesEvents, OnOpenDataSessionComplete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnOpenDataSessionComplete )( 
            __RPC__in IMbnDeviceServicesEvents * This,
            /* [in] */ __RPC__in_opt IMbnDeviceService *deviceService,
            /* [in] */ HRESULT status,
            /* [in] */ ULONG requestID);
        
        DECLSPEC_XFGVIRT(IMbnDeviceServicesEvents, OnCloseDataSessionComplete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnCloseDataSessionComplete )( 
            __RPC__in IMbnDeviceServicesEvents * This,
            /* [in] */ __RPC__in_opt IMbnDeviceService *deviceService,
            /* [in] */ HRESULT status,
            /* [in] */ ULONG requestID);
        
        DECLSPEC_XFGVIRT(IMbnDeviceServicesEvents, OnWriteDataComplete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnWriteDataComplete )( 
            __RPC__in IMbnDeviceServicesEvents * This,
            /* [in] */ __RPC__in_opt IMbnDeviceService *deviceService,
            /* [in] */ HRESULT status,
            /* [in] */ ULONG requestID);
        
        DECLSPEC_XFGVIRT(IMbnDeviceServicesEvents, OnReadData)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnReadData )( 
            __RPC__in IMbnDeviceServicesEvents * This,
            /* [in] */ __RPC__in_opt IMbnDeviceService *deviceService,
            /* [in] */ __RPC__in SAFEARRAY * deviceServiceData);
        
        DECLSPEC_XFGVIRT(IMbnDeviceServicesEvents, OnInterfaceStateChange)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OnInterfaceStateChange )( 
            __RPC__in IMbnDeviceServicesEvents * This,
            /* [in] */ __RPC__in BSTR interfaceID,
            /* [in] */ MBN_DEVICE_SERVICES_INTERFACE_STATE stateChange);
        
        END_INTERFACE
    } IMbnDeviceServicesEventsVtbl;

    interface IMbnDeviceServicesEvents
    {
        CONST_VTBL struct IMbnDeviceServicesEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMbnDeviceServicesEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMbnDeviceServicesEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMbnDeviceServicesEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMbnDeviceServicesEvents_OnQuerySupportedCommandsComplete(This,deviceService,commandIDList,status,requestID)	\
    ( (This)->lpVtbl -> OnQuerySupportedCommandsComplete(This,deviceService,commandIDList,status,requestID) ) 

#define IMbnDeviceServicesEvents_OnOpenCommandSessionComplete(This,deviceService,status,requestID)	\
    ( (This)->lpVtbl -> OnOpenCommandSessionComplete(This,deviceService,status,requestID) ) 

#define IMbnDeviceServicesEvents_OnCloseCommandSessionComplete(This,deviceService,status,requestID)	\
    ( (This)->lpVtbl -> OnCloseCommandSessionComplete(This,deviceService,status,requestID) ) 

#define IMbnDeviceServicesEvents_OnSetCommandComplete(This,deviceService,responseID,deviceServiceData,status,requestID)	\
    ( (This)->lpVtbl -> OnSetCommandComplete(This,deviceService,responseID,deviceServiceData,status,requestID) ) 

#define IMbnDeviceServicesEvents_OnQueryCommandComplete(This,deviceService,responseID,deviceServiceData,status,requestID)	\
    ( (This)->lpVtbl -> OnQueryCommandComplete(This,deviceService,responseID,deviceServiceData,status,requestID) ) 

#define IMbnDeviceServicesEvents_OnEventNotification(This,deviceService,eventID,deviceServiceData)	\
    ( (This)->lpVtbl -> OnEventNotification(This,deviceService,eventID,deviceServiceData) ) 

#define IMbnDeviceServicesEvents_OnOpenDataSessionComplete(This,deviceService,status,requestID)	\
    ( (This)->lpVtbl -> OnOpenDataSessionComplete(This,deviceService,status,requestID) ) 

#define IMbnDeviceServicesEvents_OnCloseDataSessionComplete(This,deviceService,status,requestID)	\
    ( (This)->lpVtbl -> OnCloseDataSessionComplete(This,deviceService,status,requestID) ) 

#define IMbnDeviceServicesEvents_OnWriteDataComplete(This,deviceService,status,requestID)	\
    ( (This)->lpVtbl -> OnWriteDataComplete(This,deviceService,status,requestID) ) 

#define IMbnDeviceServicesEvents_OnReadData(This,deviceService,deviceServiceData)	\
    ( (This)->lpVtbl -> OnReadData(This,deviceService,deviceServiceData) ) 

#define IMbnDeviceServicesEvents_OnInterfaceStateChange(This,interfaceID,stateChange)	\
    ( (This)->lpVtbl -> OnInterfaceStateChange(This,interfaceID,stateChange) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMbnDeviceServicesEvents_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mbnapi_0000_0040 */
/* [local] */ 

#pragma deprecated(IMbnDeviceServicesEvents) // Replaced by WinRT Windows.Networking.NetworkOperators.MobileBroadbandDeviceService


extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0040_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0040_v0_0_s_ifspec;

#ifndef __IMbnDeviceService_INTERFACE_DEFINED__
#define __IMbnDeviceService_INTERFACE_DEFINED__

/* interface IMbnDeviceService */
/* [helpstring][uuid][oleautomation][nonextensible][object] */ 


EXTERN_C const IID IID_IMbnDeviceService;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B3BB9A71-DC70-4BE9-A4DA-7886AE8B191B")
    IMbnDeviceService : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE QuerySupportedCommands( 
            /* [out] */ __RPC__out ULONG *requestID) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OpenCommandSession( 
            /* [out] */ __RPC__out ULONG *requestID) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CloseCommandSession( 
            /* [out] */ __RPC__out ULONG *requestID) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetCommand( 
            /* [in] */ ULONG commandID,
            /* [ref][in] */ __RPC__in SAFEARRAY * deviceServiceData,
            /* [out] */ __RPC__out ULONG *requestID) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE QueryCommand( 
            /* [in] */ ULONG commandID,
            /* [ref][in] */ __RPC__in SAFEARRAY * deviceServiceData,
            /* [out] */ __RPC__out ULONG *requestID) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE OpenDataSession( 
            /* [out] */ __RPC__out ULONG *requestID) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CloseDataSession( 
            /* [out] */ __RPC__out ULONG *requestID) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE WriteData( 
            /* [in] */ __RPC__in SAFEARRAY * deviceServiceData,
            /* [out] */ __RPC__out ULONG *requestID) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_InterfaceID( 
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *InterfaceID) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DeviceServiceID( 
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *DeviceServiceID) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_IsCommandSessionOpen( 
            /* [retval][ref][out] */ __RPC__out BOOL *value) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_IsDataSessionOpen( 
            /* [retval][ref][out] */ __RPC__out BOOL *value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMbnDeviceServiceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMbnDeviceService * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMbnDeviceService * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMbnDeviceService * This);
        
        DECLSPEC_XFGVIRT(IMbnDeviceService, QuerySupportedCommands)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *QuerySupportedCommands )( 
            __RPC__in IMbnDeviceService * This,
            /* [out] */ __RPC__out ULONG *requestID);
        
        DECLSPEC_XFGVIRT(IMbnDeviceService, OpenCommandSession)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OpenCommandSession )( 
            __RPC__in IMbnDeviceService * This,
            /* [out] */ __RPC__out ULONG *requestID);
        
        DECLSPEC_XFGVIRT(IMbnDeviceService, CloseCommandSession)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CloseCommandSession )( 
            __RPC__in IMbnDeviceService * This,
            /* [out] */ __RPC__out ULONG *requestID);
        
        DECLSPEC_XFGVIRT(IMbnDeviceService, SetCommand)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetCommand )( 
            __RPC__in IMbnDeviceService * This,
            /* [in] */ ULONG commandID,
            /* [ref][in] */ __RPC__in SAFEARRAY * deviceServiceData,
            /* [out] */ __RPC__out ULONG *requestID);
        
        DECLSPEC_XFGVIRT(IMbnDeviceService, QueryCommand)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *QueryCommand )( 
            __RPC__in IMbnDeviceService * This,
            /* [in] */ ULONG commandID,
            /* [ref][in] */ __RPC__in SAFEARRAY * deviceServiceData,
            /* [out] */ __RPC__out ULONG *requestID);
        
        DECLSPEC_XFGVIRT(IMbnDeviceService, OpenDataSession)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *OpenDataSession )( 
            __RPC__in IMbnDeviceService * This,
            /* [out] */ __RPC__out ULONG *requestID);
        
        DECLSPEC_XFGVIRT(IMbnDeviceService, CloseDataSession)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CloseDataSession )( 
            __RPC__in IMbnDeviceService * This,
            /* [out] */ __RPC__out ULONG *requestID);
        
        DECLSPEC_XFGVIRT(IMbnDeviceService, WriteData)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *WriteData )( 
            __RPC__in IMbnDeviceService * This,
            /* [in] */ __RPC__in SAFEARRAY * deviceServiceData,
            /* [out] */ __RPC__out ULONG *requestID);
        
        DECLSPEC_XFGVIRT(IMbnDeviceService, get_InterfaceID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_InterfaceID )( 
            __RPC__in IMbnDeviceService * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *InterfaceID);
        
        DECLSPEC_XFGVIRT(IMbnDeviceService, get_DeviceServiceID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DeviceServiceID )( 
            __RPC__in IMbnDeviceService * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt BSTR *DeviceServiceID);
        
        DECLSPEC_XFGVIRT(IMbnDeviceService, get_IsCommandSessionOpen)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_IsCommandSessionOpen )( 
            __RPC__in IMbnDeviceService * This,
            /* [retval][ref][out] */ __RPC__out BOOL *value);
        
        DECLSPEC_XFGVIRT(IMbnDeviceService, get_IsDataSessionOpen)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_IsDataSessionOpen )( 
            __RPC__in IMbnDeviceService * This,
            /* [retval][ref][out] */ __RPC__out BOOL *value);
        
        END_INTERFACE
    } IMbnDeviceServiceVtbl;

    interface IMbnDeviceService
    {
        CONST_VTBL struct IMbnDeviceServiceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMbnDeviceService_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMbnDeviceService_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMbnDeviceService_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMbnDeviceService_QuerySupportedCommands(This,requestID)	\
    ( (This)->lpVtbl -> QuerySupportedCommands(This,requestID) ) 

#define IMbnDeviceService_OpenCommandSession(This,requestID)	\
    ( (This)->lpVtbl -> OpenCommandSession(This,requestID) ) 

#define IMbnDeviceService_CloseCommandSession(This,requestID)	\
    ( (This)->lpVtbl -> CloseCommandSession(This,requestID) ) 

#define IMbnDeviceService_SetCommand(This,commandID,deviceServiceData,requestID)	\
    ( (This)->lpVtbl -> SetCommand(This,commandID,deviceServiceData,requestID) ) 

#define IMbnDeviceService_QueryCommand(This,commandID,deviceServiceData,requestID)	\
    ( (This)->lpVtbl -> QueryCommand(This,commandID,deviceServiceData,requestID) ) 

#define IMbnDeviceService_OpenDataSession(This,requestID)	\
    ( (This)->lpVtbl -> OpenDataSession(This,requestID) ) 

#define IMbnDeviceService_CloseDataSession(This,requestID)	\
    ( (This)->lpVtbl -> CloseDataSession(This,requestID) ) 

#define IMbnDeviceService_WriteData(This,deviceServiceData,requestID)	\
    ( (This)->lpVtbl -> WriteData(This,deviceServiceData,requestID) ) 

#define IMbnDeviceService_get_InterfaceID(This,InterfaceID)	\
    ( (This)->lpVtbl -> get_InterfaceID(This,InterfaceID) ) 

#define IMbnDeviceService_get_DeviceServiceID(This,DeviceServiceID)	\
    ( (This)->lpVtbl -> get_DeviceServiceID(This,DeviceServiceID) ) 

#define IMbnDeviceService_get_IsCommandSessionOpen(This,value)	\
    ( (This)->lpVtbl -> get_IsCommandSessionOpen(This,value) ) 

#define IMbnDeviceService_get_IsDataSessionOpen(This,value)	\
    ( (This)->lpVtbl -> get_IsDataSessionOpen(This,value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMbnDeviceService_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mbnapi_0000_0041 */
/* [local] */ 

#pragma deprecated(IMbnDeviceService) // Replaced by WinRT Windows.Networking.NetworkOperators.MobileBroadbandDeviceService
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0041_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mbnapi_0000_0041_v0_0_s_ifspec;


#ifndef __MbnApi_LIBRARY_DEFINED__
#define __MbnApi_LIBRARY_DEFINED__

/* library MbnApi */
/* [helpstring][version][uuid] */ 

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
typedef /* [hidden] */ struct __mbnapi_ReferenceRemainingTypes__
    {
    MBN_BAND_CLASS bandClass;
    MBN_CONTEXT_CONSTANTS contextConstants;
    MBN_CTRL_CAPS ctrlCaps;
    MBN_DATA_CLASS dataClass;
    MBN_INTERFACE_CAPS_CONSTANTS interfaceCapsConstants;
    MBN_PIN_CONSTANTS pinConstants;
    MBN_PROVIDER_CONSTANTS providerConstants;
    MBN_PROVIDER_STATE providerState;
    MBN_REGISTRATION_CONSTANTS registrationConstants;
    MBN_SIGNAL_CONSTANTS signalConstants;
    MBN_SMS_CAPS smsCaps;
    enum MBN_SMS_CONSTANTS smsConstants;
    WWAEXT_SMS_CONSTANTS wwaextSmsConstants;
    MBN_SMS_STATUS_FLAG smsStatusFlag;
    } 	__mbnapi_ReferenceRemainingTypes__;

typedef /* [hidden] */ struct __DummyPinType__
    {
    ULONG pinType;
    } 	__DummyPinType__;





































#pragma deprecated(IMbnPin) // Replaced by WinRT API in Windows.Networking.Connectivity and Windows.Networking.NetworkOperators namespaces
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)




#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion

EXTERN_C const IID LIBID_MbnApi;

#ifndef __IMbnPin_INTERFACE_DEFINED__
#define __IMbnPin_INTERFACE_DEFINED__

/* interface IMbnPin */
/* [helpstring][uuid][oleautomation][nonextensible][object] */ 


EXTERN_C const IID IID_IMbnPin;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DCBBBAB6-2007-4BBB-AAEE-338E368AF6FA")
    IMbnPin : public IUnknown
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PinType( 
            /* [retval][ref][out] */ __RPC__out MBN_PIN_TYPE *PinType) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PinFormat( 
            /* [retval][ref][out] */ __RPC__out MBN_PIN_FORMAT *PinFormat) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PinLengthMin( 
            /* [retval][ref][out] */ __RPC__out ULONG *PinLengthMin) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PinLengthMax( 
            /* [retval][ref][out] */ __RPC__out ULONG *PinLengthMax) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PinMode( 
            /* [retval][ref][out] */ __RPC__out MBN_PIN_MODE *PinMode) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Enable( 
            /* [string][ref][in] */ __RPC__in_string LPCWSTR pin,
            /* [out] */ __RPC__out ULONG *requestID) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Disable( 
            /* [string][ref][in] */ __RPC__in_string LPCWSTR pin,
            /* [out] */ __RPC__out ULONG *requestID) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Enter( 
            /* [string][ref][in] */ __RPC__in_string LPCWSTR pin,
            /* [out] */ __RPC__out ULONG *requestID) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Change( 
            /* [string][ref][in] */ __RPC__in_string LPCWSTR pin,
            /* [string][ref][in] */ __RPC__in_string LPCWSTR newPin,
            /* [out] */ __RPC__out ULONG *requestID) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Unblock( 
            /* [string][ref][in] */ __RPC__in_string LPCWSTR puk,
            /* [string][ref][in] */ __RPC__in_string LPCWSTR newPin,
            /* [out] */ __RPC__out ULONG *requestID) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetPinManager( 
            /* [retval][ref][out] */ __RPC__deref_out_opt IMbnPinManager **pinManager) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMbnPinVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMbnPin * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMbnPin * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMbnPin * This);
        
        DECLSPEC_XFGVIRT(IMbnPin, get_PinType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PinType )( 
            __RPC__in IMbnPin * This,
            /* [retval][ref][out] */ __RPC__out MBN_PIN_TYPE *PinType);
        
        DECLSPEC_XFGVIRT(IMbnPin, get_PinFormat)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PinFormat )( 
            __RPC__in IMbnPin * This,
            /* [retval][ref][out] */ __RPC__out MBN_PIN_FORMAT *PinFormat);
        
        DECLSPEC_XFGVIRT(IMbnPin, get_PinLengthMin)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PinLengthMin )( 
            __RPC__in IMbnPin * This,
            /* [retval][ref][out] */ __RPC__out ULONG *PinLengthMin);
        
        DECLSPEC_XFGVIRT(IMbnPin, get_PinLengthMax)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PinLengthMax )( 
            __RPC__in IMbnPin * This,
            /* [retval][ref][out] */ __RPC__out ULONG *PinLengthMax);
        
        DECLSPEC_XFGVIRT(IMbnPin, get_PinMode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PinMode )( 
            __RPC__in IMbnPin * This,
            /* [retval][ref][out] */ __RPC__out MBN_PIN_MODE *PinMode);
        
        DECLSPEC_XFGVIRT(IMbnPin, Enable)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Enable )( 
            __RPC__in IMbnPin * This,
            /* [string][ref][in] */ __RPC__in_string LPCWSTR pin,
            /* [out] */ __RPC__out ULONG *requestID);
        
        DECLSPEC_XFGVIRT(IMbnPin, Disable)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Disable )( 
            __RPC__in IMbnPin * This,
            /* [string][ref][in] */ __RPC__in_string LPCWSTR pin,
            /* [out] */ __RPC__out ULONG *requestID);
        
        DECLSPEC_XFGVIRT(IMbnPin, Enter)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Enter )( 
            __RPC__in IMbnPin * This,
            /* [string][ref][in] */ __RPC__in_string LPCWSTR pin,
            /* [out] */ __RPC__out ULONG *requestID);
        
        DECLSPEC_XFGVIRT(IMbnPin, Change)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Change )( 
            __RPC__in IMbnPin * This,
            /* [string][ref][in] */ __RPC__in_string LPCWSTR pin,
            /* [string][ref][in] */ __RPC__in_string LPCWSTR newPin,
            /* [out] */ __RPC__out ULONG *requestID);
        
        DECLSPEC_XFGVIRT(IMbnPin, Unblock)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Unblock )( 
            __RPC__in IMbnPin * This,
            /* [string][ref][in] */ __RPC__in_string LPCWSTR puk,
            /* [string][ref][in] */ __RPC__in_string LPCWSTR newPin,
            /* [out] */ __RPC__out ULONG *requestID);
        
        DECLSPEC_XFGVIRT(IMbnPin, GetPinManager)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetPinManager )( 
            __RPC__in IMbnPin * This,
            /* [retval][ref][out] */ __RPC__deref_out_opt IMbnPinManager **pinManager);
        
        END_INTERFACE
    } IMbnPinVtbl;

    interface IMbnPin
    {
        CONST_VTBL struct IMbnPinVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMbnPin_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMbnPin_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMbnPin_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMbnPin_get_PinType(This,PinType)	\
    ( (This)->lpVtbl -> get_PinType(This,PinType) ) 

#define IMbnPin_get_PinFormat(This,PinFormat)	\
    ( (This)->lpVtbl -> get_PinFormat(This,PinFormat) ) 

#define IMbnPin_get_PinLengthMin(This,PinLengthMin)	\
    ( (This)->lpVtbl -> get_PinLengthMin(This,PinLengthMin) ) 

#define IMbnPin_get_PinLengthMax(This,PinLengthMax)	\
    ( (This)->lpVtbl -> get_PinLengthMax(This,PinLengthMax) ) 

#define IMbnPin_get_PinMode(This,PinMode)	\
    ( (This)->lpVtbl -> get_PinMode(This,PinMode) ) 

#define IMbnPin_Enable(This,pin,requestID)	\
    ( (This)->lpVtbl -> Enable(This,pin,requestID) ) 

#define IMbnPin_Disable(This,pin,requestID)	\
    ( (This)->lpVtbl -> Disable(This,pin,requestID) ) 

#define IMbnPin_Enter(This,pin,requestID)	\
    ( (This)->lpVtbl -> Enter(This,pin,requestID) ) 

#define IMbnPin_Change(This,pin,newPin,requestID)	\
    ( (This)->lpVtbl -> Change(This,pin,newPin,requestID) ) 

#define IMbnPin_Unblock(This,puk,newPin,requestID)	\
    ( (This)->lpVtbl -> Unblock(This,puk,newPin,requestID) ) 

#define IMbnPin_GetPinManager(This,pinManager)	\
    ( (This)->lpVtbl -> GetPinManager(This,pinManager) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMbnPin_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_MbnConnectionProfileManager;

#ifdef __cplusplus

class DECLSPEC_UUID("BDFEE05A-4418-11DD-90ED-001C257CCFF1")
MbnConnectionProfileManager;
#endif

EXTERN_C const CLSID CLSID_MbnInterfaceManager;

#ifdef __cplusplus

class DECLSPEC_UUID("BDFEE05B-4418-11DD-90ED-001C257CCFF1")
MbnInterfaceManager;
#endif

EXTERN_C const CLSID CLSID_MbnConnectionManager;

#ifdef __cplusplus

class DECLSPEC_UUID("BDFEE05C-4418-11DD-90ED-001C257CCFF1")
MbnConnectionManager;
#endif

EXTERN_C const CLSID CLSID_MbnDeviceServicesManager;

#ifdef __cplusplus

class DECLSPEC_UUID("2269DAA3-2A9F-4165-A501-CE00A6F7A75B")
MbnDeviceServicesManager;
#endif
#endif /* __MbnApi_LIBRARY_DEFINED__ */

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





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

#ifndef __cellularapi_oem_h__
#define __cellularapi_oem_h__

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

#ifndef __IOemCellular_FWD_DEFINED__
#define __IOemCellular_FWD_DEFINED__
typedef interface IOemCellular IOemCellular;

#endif 	/* __IOemCellular_FWD_DEFINED__ */


#ifndef __IOemCellularDataStore_FWD_DEFINED__
#define __IOemCellularDataStore_FWD_DEFINED__
typedef interface IOemCellularDataStore IOemCellularDataStore;

#endif 	/* __IOemCellularDataStore_FWD_DEFINED__ */


#ifndef __IOemCellularModem_FWD_DEFINED__
#define __IOemCellularModem_FWD_DEFINED__
typedef interface IOemCellularModem IOemCellularModem;

#endif 	/* __IOemCellularModem_FWD_DEFINED__ */


#ifndef __IOemCellularModemEx_FWD_DEFINED__
#define __IOemCellularModemEx_FWD_DEFINED__
typedef interface IOemCellularModemEx IOemCellularModemEx;

#endif 	/* __IOemCellularModemEx_FWD_DEFINED__ */


#ifndef __IOemCan_FWD_DEFINED__
#define __IOemCan_FWD_DEFINED__
typedef interface IOemCan IOemCan;

#endif 	/* __IOemCan_FWD_DEFINED__ */


#ifndef __IOemCanExtForIMS_FWD_DEFINED__
#define __IOemCanExtForIMS_FWD_DEFINED__
typedef interface IOemCanExtForIMS IOemCanExtForIMS;

#endif 	/* __IOemCanExtForIMS_FWD_DEFINED__ */


#ifndef __IOemSlot_FWD_DEFINED__
#define __IOemSlot_FWD_DEFINED__
typedef interface IOemSlot IOemSlot;

#endif 	/* __IOemSlot_FWD_DEFINED__ */


#ifndef __IOemUicc_FWD_DEFINED__
#define __IOemUicc_FWD_DEFINED__
typedef interface IOemUicc IOemUicc;

#endif 	/* __IOemUicc_FWD_DEFINED__ */


#ifndef __IOemUiccApp_FWD_DEFINED__
#define __IOemUiccApp_FWD_DEFINED__
typedef interface IOemUiccApp IOemUiccApp;

#endif 	/* __IOemUiccApp_FWD_DEFINED__ */


#ifndef __IOemUiccAppEx_FWD_DEFINED__
#define __IOemUiccAppEx_FWD_DEFINED__
typedef interface IOemUiccAppEx IOemUiccAppEx;

#endif 	/* __IOemUiccAppEx_FWD_DEFINED__ */


#ifndef __IOemUiccAppEx2_FWD_DEFINED__
#define __IOemUiccAppEx2_FWD_DEFINED__
typedef interface IOemUiccAppEx2 IOemUiccAppEx2;

#endif 	/* __IOemUiccAppEx2_FWD_DEFINED__ */


#ifndef __IOem3GPPSupServices_FWD_DEFINED__
#define __IOem3GPPSupServices_FWD_DEFINED__
typedef interface IOem3GPPSupServices IOem3GPPSupServices;

#endif 	/* __IOem3GPPSupServices_FWD_DEFINED__ */


#ifndef __IOemCellularModemExistenceChange_FWD_DEFINED__
#define __IOemCellularModemExistenceChange_FWD_DEFINED__
typedef interface IOemCellularModemExistenceChange IOemCellularModemExistenceChange;

#endif 	/* __IOemCellularModemExistenceChange_FWD_DEFINED__ */


#ifndef __IOemCellularCanAvailabilityChange_FWD_DEFINED__
#define __IOemCellularCanAvailabilityChange_FWD_DEFINED__
typedef interface IOemCellularCanAvailabilityChange IOemCellularCanAvailabilityChange;

#endif 	/* __IOemCellularCanAvailabilityChange_FWD_DEFINED__ */


#ifndef __IOemSlotChange_FWD_DEFINED__
#define __IOemSlotChange_FWD_DEFINED__
typedef interface IOemSlotChange IOemSlotChange;

#endif 	/* __IOemSlotChange_FWD_DEFINED__ */


#ifndef __IOemCanRegistrationStateChange_FWD_DEFINED__
#define __IOemCanRegistrationStateChange_FWD_DEFINED__
typedef interface IOemCanRegistrationStateChange IOemCanRegistrationStateChange;

#endif 	/* __IOemCanRegistrationStateChange_FWD_DEFINED__ */


#ifndef __IOemSlotStateChange_FWD_DEFINED__
#define __IOemSlotStateChange_FWD_DEFINED__
typedef interface IOemSlotStateChange IOemSlotStateChange;

#endif 	/* __IOemSlotStateChange_FWD_DEFINED__ */


#ifndef __IOemUiccChange_FWD_DEFINED__
#define __IOemUiccChange_FWD_DEFINED__
typedef interface IOemUiccChange IOemUiccChange;

#endif 	/* __IOemUiccChange_FWD_DEFINED__ */


#ifndef __IOemRegistrationStatus_FWD_DEFINED__
#define __IOemRegistrationStatus_FWD_DEFINED__
typedef interface IOemRegistrationStatus IOemRegistrationStatus;

#endif 	/* __IOemRegistrationStatus_FWD_DEFINED__ */


#ifndef __IPowerStateChange_FWD_DEFINED__
#define __IPowerStateChange_FWD_DEFINED__
typedef interface IPowerStateChange IPowerStateChange;

#endif 	/* __IPowerStateChange_FWD_DEFINED__ */


#ifndef __IModemOpaqueCommandCompletion_FWD_DEFINED__
#define __IModemOpaqueCommandCompletion_FWD_DEFINED__
typedef interface IModemOpaqueCommandCompletion IModemOpaqueCommandCompletion;

#endif 	/* __IModemOpaqueCommandCompletion_FWD_DEFINED__ */


#ifndef __IOpaqueModemNotifications_FWD_DEFINED__
#define __IOpaqueModemNotifications_FWD_DEFINED__
typedef interface IOpaqueModemNotifications IOpaqueModemNotifications;

#endif 	/* __IOpaqueModemNotifications_FWD_DEFINED__ */


#ifndef __ISetRFStateCompletion_FWD_DEFINED__
#define __ISetRFStateCompletion_FWD_DEFINED__
typedef interface ISetRFStateCompletion ISetRFStateCompletion;

#endif 	/* __ISetRFStateCompletion_FWD_DEFINED__ */


#ifndef __IGetRFStateCompletion_FWD_DEFINED__
#define __IGetRFStateCompletion_FWD_DEFINED__
typedef interface IGetRFStateCompletion IGetRFStateCompletion;

#endif 	/* __IGetRFStateCompletion_FWD_DEFINED__ */


#ifndef __IGetRFStateExCompletion_FWD_DEFINED__
#define __IGetRFStateExCompletion_FWD_DEFINED__
typedef interface IGetRFStateExCompletion IGetRFStateExCompletion;

#endif 	/* __IGetRFStateExCompletion_FWD_DEFINED__ */


#ifndef __ICanInfoCompletion_FWD_DEFINED__
#define __ICanInfoCompletion_FWD_DEFINED__
typedef interface ICanInfoCompletion ICanInfoCompletion;

#endif 	/* __ICanInfoCompletion_FWD_DEFINED__ */


#ifndef __IPositionInfoCompletion_FWD_DEFINED__
#define __IPositionInfoCompletion_FWD_DEFINED__
typedef interface IPositionInfoCompletion IPositionInfoCompletion;

#endif 	/* __IPositionInfoCompletion_FWD_DEFINED__ */


#ifndef __IPinLockStateCompletion_FWD_DEFINED__
#define __IPinLockStateCompletion_FWD_DEFINED__
typedef interface IPinLockStateCompletion IPinLockStateCompletion;

#endif 	/* __IPinLockStateCompletion_FWD_DEFINED__ */


#ifndef __IGetRecordStatusCompletion_FWD_DEFINED__
#define __IGetRecordStatusCompletion_FWD_DEFINED__
typedef interface IGetRecordStatusCompletion IGetRecordStatusCompletion;

#endif 	/* __IGetRecordStatusCompletion_FWD_DEFINED__ */


#ifndef __IReadRecordCompletion_FWD_DEFINED__
#define __IReadRecordCompletion_FWD_DEFINED__
typedef interface IReadRecordCompletion IReadRecordCompletion;

#endif 	/* __IReadRecordCompletion_FWD_DEFINED__ */


#ifndef __IWriteRecordCompletion_FWD_DEFINED__
#define __IWriteRecordCompletion_FWD_DEFINED__
typedef interface IWriteRecordCompletion IWriteRecordCompletion;

#endif 	/* __IWriteRecordCompletion_FWD_DEFINED__ */


#ifndef __IIMSICompletion_FWD_DEFINED__
#define __IIMSICompletion_FWD_DEFINED__
typedef interface IIMSICompletion IIMSICompletion;

#endif 	/* __IIMSICompletion_FWD_DEFINED__ */


#ifndef __IGetSIDNIDCompletion_FWD_DEFINED__
#define __IGetSIDNIDCompletion_FWD_DEFINED__
typedef interface IGetSIDNIDCompletion IGetSIDNIDCompletion;

#endif 	/* __IGetSIDNIDCompletion_FWD_DEFINED__ */


#ifndef __IGetNAICompletion_FWD_DEFINED__
#define __IGetNAICompletion_FWD_DEFINED__
typedef interface IGetNAICompletion IGetNAICompletion;

#endif 	/* __IGetNAICompletion_FWD_DEFINED__ */


#ifndef __ISubscriberNumbersCompletion_FWD_DEFINED__
#define __ISubscriberNumbersCompletion_FWD_DEFINED__
typedef interface ISubscriberNumbersCompletion ISubscriberNumbersCompletion;

#endif 	/* __ISubscriberNumbersCompletion_FWD_DEFINED__ */


#ifndef __IGetPLMNwAcT_FWD_DEFINED__
#define __IGetPLMNwAcT_FWD_DEFINED__
typedef interface IGetPLMNwAcT IGetPLMNwAcT;

#endif 	/* __IGetPLMNwAcT_FWD_DEFINED__ */


#ifndef __ISetPLMNwAcT_FWD_DEFINED__
#define __ISetPLMNwAcT_FWD_DEFINED__
typedef interface ISetPLMNwAcT ISetPLMNwAcT;

#endif 	/* __ISetPLMNwAcT_FWD_DEFINED__ */


#ifndef __IGetCallForwardingCompletion_FWD_DEFINED__
#define __IGetCallForwardingCompletion_FWD_DEFINED__
typedef interface IGetCallForwardingCompletion IGetCallForwardingCompletion;

#endif 	/* __IGetCallForwardingCompletion_FWD_DEFINED__ */


#ifndef __ISimpleModemCompletion_FWD_DEFINED__
#define __ISimpleModemCompletion_FWD_DEFINED__
typedef interface ISimpleModemCompletion ISimpleModemCompletion;

#endif 	/* __ISimpleModemCompletion_FWD_DEFINED__ */


#ifndef __ICallWaitingSettingsCompletion_FWD_DEFINED__
#define __ICallWaitingSettingsCompletion_FWD_DEFINED__
typedef interface ICallWaitingSettingsCompletion ICallWaitingSettingsCompletion;

#endif 	/* __ICallWaitingSettingsCompletion_FWD_DEFINED__ */


#ifndef __IOemIMSStatusChange_FWD_DEFINED__
#define __IOemIMSStatusChange_FWD_DEFINED__
typedef interface IOemIMSStatusChange IOemIMSStatusChange;

#endif 	/* __IOemIMSStatusChange_FWD_DEFINED__ */


#ifndef __ICanInfoCompletion_FWD_DEFINED__
#define __ICanInfoCompletion_FWD_DEFINED__
typedef interface ICanInfoCompletion ICanInfoCompletion;

#endif 	/* __ICanInfoCompletion_FWD_DEFINED__ */


#ifndef __IGetRecordStatusCompletion_FWD_DEFINED__
#define __IGetRecordStatusCompletion_FWD_DEFINED__
typedef interface IGetRecordStatusCompletion IGetRecordStatusCompletion;

#endif 	/* __IGetRecordStatusCompletion_FWD_DEFINED__ */


#ifndef __IGetRFStateCompletion_FWD_DEFINED__
#define __IGetRFStateCompletion_FWD_DEFINED__
typedef interface IGetRFStateCompletion IGetRFStateCompletion;

#endif 	/* __IGetRFStateCompletion_FWD_DEFINED__ */


#ifndef __IGetSIDNIDCompletion_FWD_DEFINED__
#define __IGetSIDNIDCompletion_FWD_DEFINED__
typedef interface IGetSIDNIDCompletion IGetSIDNIDCompletion;

#endif 	/* __IGetSIDNIDCompletion_FWD_DEFINED__ */


#ifndef __IGetNAICompletion_FWD_DEFINED__
#define __IGetNAICompletion_FWD_DEFINED__
typedef interface IGetNAICompletion IGetNAICompletion;

#endif 	/* __IGetNAICompletion_FWD_DEFINED__ */


#ifndef __IIMSICompletion_FWD_DEFINED__
#define __IIMSICompletion_FWD_DEFINED__
typedef interface IIMSICompletion IIMSICompletion;

#endif 	/* __IIMSICompletion_FWD_DEFINED__ */


#ifndef __IModemOpaqueCommandCompletion_FWD_DEFINED__
#define __IModemOpaqueCommandCompletion_FWD_DEFINED__
typedef interface IModemOpaqueCommandCompletion IModemOpaqueCommandCompletion;

#endif 	/* __IModemOpaqueCommandCompletion_FWD_DEFINED__ */


#ifndef __IOemCan_FWD_DEFINED__
#define __IOemCan_FWD_DEFINED__
typedef interface IOemCan IOemCan;

#endif 	/* __IOemCan_FWD_DEFINED__ */


#ifndef __IOemCanExtForIMS_FWD_DEFINED__
#define __IOemCanExtForIMS_FWD_DEFINED__
typedef interface IOemCanExtForIMS IOemCanExtForIMS;

#endif 	/* __IOemCanExtForIMS_FWD_DEFINED__ */


#ifndef __IOemCanRegistrationStateChange_FWD_DEFINED__
#define __IOemCanRegistrationStateChange_FWD_DEFINED__
typedef interface IOemCanRegistrationStateChange IOemCanRegistrationStateChange;

#endif 	/* __IOemCanRegistrationStateChange_FWD_DEFINED__ */


#ifndef __IOemCellular_FWD_DEFINED__
#define __IOemCellular_FWD_DEFINED__
typedef interface IOemCellular IOemCellular;

#endif 	/* __IOemCellular_FWD_DEFINED__ */


#ifndef __IOemCellularCanAvailabilityChange_FWD_DEFINED__
#define __IOemCellularCanAvailabilityChange_FWD_DEFINED__
typedef interface IOemCellularCanAvailabilityChange IOemCellularCanAvailabilityChange;

#endif 	/* __IOemCellularCanAvailabilityChange_FWD_DEFINED__ */


#ifndef __IOemCellularModem_FWD_DEFINED__
#define __IOemCellularModem_FWD_DEFINED__
typedef interface IOemCellularModem IOemCellularModem;

#endif 	/* __IOemCellularModem_FWD_DEFINED__ */


#ifndef __IOemCellularModemExistenceChange_FWD_DEFINED__
#define __IOemCellularModemExistenceChange_FWD_DEFINED__
typedef interface IOemCellularModemExistenceChange IOemCellularModemExistenceChange;

#endif 	/* __IOemCellularModemExistenceChange_FWD_DEFINED__ */


#ifndef __IOemRegistrationStatus_FWD_DEFINED__
#define __IOemRegistrationStatus_FWD_DEFINED__
typedef interface IOemRegistrationStatus IOemRegistrationStatus;

#endif 	/* __IOemRegistrationStatus_FWD_DEFINED__ */


#ifndef __IOemSlot_FWD_DEFINED__
#define __IOemSlot_FWD_DEFINED__
typedef interface IOemSlot IOemSlot;

#endif 	/* __IOemSlot_FWD_DEFINED__ */


#ifndef __IOemSlotChange_FWD_DEFINED__
#define __IOemSlotChange_FWD_DEFINED__
typedef interface IOemSlotChange IOemSlotChange;

#endif 	/* __IOemSlotChange_FWD_DEFINED__ */


#ifndef __IOemSlotStateChange_FWD_DEFINED__
#define __IOemSlotStateChange_FWD_DEFINED__
typedef interface IOemSlotStateChange IOemSlotStateChange;

#endif 	/* __IOemSlotStateChange_FWD_DEFINED__ */


#ifndef __IOemUiccChange_FWD_DEFINED__
#define __IOemUiccChange_FWD_DEFINED__
typedef interface IOemUiccChange IOemUiccChange;

#endif 	/* __IOemUiccChange_FWD_DEFINED__ */


#ifndef __IOpaqueModemNotifications_FWD_DEFINED__
#define __IOpaqueModemNotifications_FWD_DEFINED__
typedef interface IOpaqueModemNotifications IOpaqueModemNotifications;

#endif 	/* __IOpaqueModemNotifications_FWD_DEFINED__ */


#ifndef __IPinLockStateCompletion_FWD_DEFINED__
#define __IPinLockStateCompletion_FWD_DEFINED__
typedef interface IPinLockStateCompletion IPinLockStateCompletion;

#endif 	/* __IPinLockStateCompletion_FWD_DEFINED__ */


#ifndef __IPositionInfoCompletion_FWD_DEFINED__
#define __IPositionInfoCompletion_FWD_DEFINED__
typedef interface IPositionInfoCompletion IPositionInfoCompletion;

#endif 	/* __IPositionInfoCompletion_FWD_DEFINED__ */


#ifndef __IPowerStateChange_FWD_DEFINED__
#define __IPowerStateChange_FWD_DEFINED__
typedef interface IPowerStateChange IPowerStateChange;

#endif 	/* __IPowerStateChange_FWD_DEFINED__ */


#ifndef __IReadRecordCompletion_FWD_DEFINED__
#define __IReadRecordCompletion_FWD_DEFINED__
typedef interface IReadRecordCompletion IReadRecordCompletion;

#endif 	/* __IReadRecordCompletion_FWD_DEFINED__ */


#ifndef __ISetRFStateCompletion_FWD_DEFINED__
#define __ISetRFStateCompletion_FWD_DEFINED__
typedef interface ISetRFStateCompletion ISetRFStateCompletion;

#endif 	/* __ISetRFStateCompletion_FWD_DEFINED__ */


#ifndef __ISubscriberNumbersCompletion_FWD_DEFINED__
#define __ISubscriberNumbersCompletion_FWD_DEFINED__
typedef interface ISubscriberNumbersCompletion ISubscriberNumbersCompletion;

#endif 	/* __ISubscriberNumbersCompletion_FWD_DEFINED__ */


#ifndef __IWriteRecordCompletion_FWD_DEFINED__
#define __IWriteRecordCompletion_FWD_DEFINED__
typedef interface IWriteRecordCompletion IWriteRecordCompletion;

#endif 	/* __IWriteRecordCompletion_FWD_DEFINED__ */


#ifndef __IGetPLMNwAcT_FWD_DEFINED__
#define __IGetPLMNwAcT_FWD_DEFINED__
typedef interface IGetPLMNwAcT IGetPLMNwAcT;

#endif 	/* __IGetPLMNwAcT_FWD_DEFINED__ */


#ifndef __ISetPLMNwAcT_FWD_DEFINED__
#define __ISetPLMNwAcT_FWD_DEFINED__
typedef interface ISetPLMNwAcT ISetPLMNwAcT;

#endif 	/* __ISetPLMNwAcT_FWD_DEFINED__ */


#ifndef __IOem3GPPSupServices_FWD_DEFINED__
#define __IOem3GPPSupServices_FWD_DEFINED__
typedef interface IOem3GPPSupServices IOem3GPPSupServices;

#endif 	/* __IOem3GPPSupServices_FWD_DEFINED__ */


#ifndef __IGetCallForwardingCompletion_FWD_DEFINED__
#define __IGetCallForwardingCompletion_FWD_DEFINED__
typedef interface IGetCallForwardingCompletion IGetCallForwardingCompletion;

#endif 	/* __IGetCallForwardingCompletion_FWD_DEFINED__ */


#ifndef __ISimpleModemCompletion_FWD_DEFINED__
#define __ISimpleModemCompletion_FWD_DEFINED__
typedef interface ISimpleModemCompletion ISimpleModemCompletion;

#endif 	/* __ISimpleModemCompletion_FWD_DEFINED__ */


#ifndef __ICallWaitingSettingsCompletion_FWD_DEFINED__
#define __ICallWaitingSettingsCompletion_FWD_DEFINED__
typedef interface ICallWaitingSettingsCompletion ICallWaitingSettingsCompletion;

#endif 	/* __ICallWaitingSettingsCompletion_FWD_DEFINED__ */


#ifndef __IOemIMSStatusChange_FWD_DEFINED__
#define __IOemIMSStatusChange_FWD_DEFINED__
typedef interface IOemIMSStatusChange IOemIMSStatusChange;

#endif 	/* __IOemIMSStatusChange_FWD_DEFINED__ */


#ifndef __OemCellular_FWD_DEFINED__
#define __OemCellular_FWD_DEFINED__

#ifdef __cplusplus
typedef class OemCellular OemCellular;
#else
typedef struct OemCellular OemCellular;
#endif /* __cplusplus */

#endif 	/* __OemCellular_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#include "RilAPITypes.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_cellularapi_oem_0000_0000 */
/* [local] */ 

//+-------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (c) Microsoft Corporation. All rights reserved.
//
//--------------------------------------------------------------------------
#if ( _MSC_VER >= 1020 )
#pragma once
#endif
DEFINE_GUID(SID_3GPP_SUPSVCMODEL, 0xd7d08e07, 0xd767, 0x4478, 0xb1, 0x4a, 0xee, 0xcc, 0x87, 0xea, 0x12, 0xf7);









































#define	MAXLENGTH_NAI	( 72 )


enum MODEMPOWERSTATE
    {
        MODEM_POWER_OFF	= 0x1,
        MODEM_POWER_GOING_ON	= 0x2,
        MODEM_POWER_ON	= 0x3,
        MODEM_POWER_GOING_OFF	= 0x4,
        MODEM_POWER_SHUTING_DOWN	= 0x5
    } ;
typedef struct RILSIDNID RILSIDNID;

typedef struct RILSIDNID *LPRILSIDNID;

typedef struct RILNAI NAI;

typedef struct RILNAI *LPRILNAI;


enum RILNAIPARAMMASK
    {
        RIL_PARAM_NAI_NAI	= 0x1,
        RIL_PARAM_NAI_ALL	= 0x1
    } ;
struct RILNAI
    {
    DWORD cbSize;
    DWORD dwParams;
    WCHAR wszNAI[ 73 ];
    } ;

enum RILSIDNIDPARAMMASK
    {
        RIL_PARAM_SIDNID_SID	= 0x1,
        RIL_PARAM_SIDNID_NID	= 0x2,
        RIL_PARAM_SIDNID_ALL	= 0x3
    } ;
typedef enum RILSIDNIDPARAMMASK RILSIDNIDPARAMMASK;

struct RILSIDNID
    {
    DWORD cbSize;
    DWORD dwParams;
    DWORD dwSid;
    DWORD dwNid;
    } ;

enum UICCDATASTOREACCESSMODE
    {
        UICCDATASTORE_PUBLIC	= 0x1,
        UICCDATASTORE_PRIVATE	= 0x2
    } ;
typedef enum UICCDATASTOREACCESSMODE UICCDATASTOREACCESSMODE;

#define	MAXLENGTH_UICCDATASTORE	( 10 )

struct UICCDATASTOREENTRY
    {
    DWORD position;
    DWORD lengthIccId;
    BYTE iccId[ 10 ];
    } ;
struct UICCDATASTOREINFO
    {
    DWORD dwCount;
    struct UICCDATASTOREENTRY entries[ 10 ];
    } ;
typedef struct UICCDATASTOREINFO UICCDATASTOREINFO;



extern RPC_IF_HANDLE __MIDL_itf_cellularapi_oem_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_cellularapi_oem_0000_0000_v0_0_s_ifspec;

#ifndef __IOemCellular_INTERFACE_DEFINED__
#define __IOemCellular_INTERFACE_DEFINED__

/* interface IOemCellular */
/* [uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IOemCellular;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("17AE6DC1-3723-4085-99BA-7160CDB8BFC1")
    IOemCellular : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE RegisterForOemModemExistenceChanges( 
            /* [in] */ __RPC__in_opt IOemCellularModemExistenceChange *pCallback) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE UnregisterForOemModemExistenceChanges( 
            /* [in] */ __RPC__in_opt IOemCellularModemExistenceChange *pCallback) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Destroy( void) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE WaitForDestroyCompletion( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOemCellularVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOemCellular * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOemCellular * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOemCellular * This);
        
        DECLSPEC_XFGVIRT(IOemCellular, RegisterForOemModemExistenceChanges)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *RegisterForOemModemExistenceChanges )( 
            __RPC__in IOemCellular * This,
            /* [in] */ __RPC__in_opt IOemCellularModemExistenceChange *pCallback);
        
        DECLSPEC_XFGVIRT(IOemCellular, UnregisterForOemModemExistenceChanges)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *UnregisterForOemModemExistenceChanges )( 
            __RPC__in IOemCellular * This,
            /* [in] */ __RPC__in_opt IOemCellularModemExistenceChange *pCallback);
        
        DECLSPEC_XFGVIRT(IOemCellular, Destroy)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Destroy )( 
            __RPC__in IOemCellular * This);
        
        DECLSPEC_XFGVIRT(IOemCellular, WaitForDestroyCompletion)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *WaitForDestroyCompletion )( 
            __RPC__in IOemCellular * This);
        
        END_INTERFACE
    } IOemCellularVtbl;

    interface IOemCellular
    {
        CONST_VTBL struct IOemCellularVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOemCellular_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOemCellular_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOemCellular_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOemCellular_RegisterForOemModemExistenceChanges(This,pCallback)	\
    ( (This)->lpVtbl -> RegisterForOemModemExistenceChanges(This,pCallback) ) 

#define IOemCellular_UnregisterForOemModemExistenceChanges(This,pCallback)	\
    ( (This)->lpVtbl -> UnregisterForOemModemExistenceChanges(This,pCallback) ) 

#define IOemCellular_Destroy(This)	\
    ( (This)->lpVtbl -> Destroy(This) ) 

#define IOemCellular_WaitForDestroyCompletion(This)	\
    ( (This)->lpVtbl -> WaitForDestroyCompletion(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOemCellular_INTERFACE_DEFINED__ */


#ifndef __IOemCellularDataStore_INTERFACE_DEFINED__
#define __IOemCellularDataStore_INTERFACE_DEFINED__

/* interface IOemCellularDataStore */
/* [uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IOemCellularDataStore;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b470078a-2784-4d09-ad35-42b2290abc54")
    IOemCellularDataStore : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetNamedValue( 
            /* [in] */ __RPC__in const BYTE *pIccId,
            /* [in] */ DWORD lengthIccId,
            /* [in] */ __RPC__in LPCWSTR pClientId,
            /* [in] */ __RPC__in LPCWSTR pPropertyName,
            /* [in] */ __RPC__in const BYTE *pPropertyValue,
            /* [in] */ DWORD lengthValue,
            /* [in] */ UICCDATASTOREACCESSMODE accessMode) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetNamedValue( 
            /* [in] */ __RPC__in const BYTE *pIccId,
            /* [in] */ DWORD lengthIccId,
            /* [in] */ __RPC__in LPCWSTR pClientId,
            /* [in] */ __RPC__in LPCWSTR pPropertyName,
            /* [out] */ __RPC__out BYTE *pPropertyValue,
            /* [out][in] */ __RPC__inout DWORD *pLengthValue,
            /* [in] */ UICCDATASTOREACCESSMODE accessMode) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetUiccDataStoreInfo( 
            /* [out][in] */ __RPC__inout struct UICCDATASTOREINFO *pDatastoreInfo) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOemCellularDataStoreVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOemCellularDataStore * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOemCellularDataStore * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOemCellularDataStore * This);
        
        DECLSPEC_XFGVIRT(IOemCellularDataStore, SetNamedValue)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetNamedValue )( 
            __RPC__in IOemCellularDataStore * This,
            /* [in] */ __RPC__in const BYTE *pIccId,
            /* [in] */ DWORD lengthIccId,
            /* [in] */ __RPC__in LPCWSTR pClientId,
            /* [in] */ __RPC__in LPCWSTR pPropertyName,
            /* [in] */ __RPC__in const BYTE *pPropertyValue,
            /* [in] */ DWORD lengthValue,
            /* [in] */ UICCDATASTOREACCESSMODE accessMode);
        
        DECLSPEC_XFGVIRT(IOemCellularDataStore, GetNamedValue)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetNamedValue )( 
            __RPC__in IOemCellularDataStore * This,
            /* [in] */ __RPC__in const BYTE *pIccId,
            /* [in] */ DWORD lengthIccId,
            /* [in] */ __RPC__in LPCWSTR pClientId,
            /* [in] */ __RPC__in LPCWSTR pPropertyName,
            /* [out] */ __RPC__out BYTE *pPropertyValue,
            /* [out][in] */ __RPC__inout DWORD *pLengthValue,
            /* [in] */ UICCDATASTOREACCESSMODE accessMode);
        
        DECLSPEC_XFGVIRT(IOemCellularDataStore, GetUiccDataStoreInfo)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetUiccDataStoreInfo )( 
            __RPC__in IOemCellularDataStore * This,
            /* [out][in] */ __RPC__inout struct UICCDATASTOREINFO *pDatastoreInfo);
        
        END_INTERFACE
    } IOemCellularDataStoreVtbl;

    interface IOemCellularDataStore
    {
        CONST_VTBL struct IOemCellularDataStoreVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOemCellularDataStore_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOemCellularDataStore_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOemCellularDataStore_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOemCellularDataStore_SetNamedValue(This,pIccId,lengthIccId,pClientId,pPropertyName,pPropertyValue,lengthValue,accessMode)	\
    ( (This)->lpVtbl -> SetNamedValue(This,pIccId,lengthIccId,pClientId,pPropertyName,pPropertyValue,lengthValue,accessMode) ) 

#define IOemCellularDataStore_GetNamedValue(This,pIccId,lengthIccId,pClientId,pPropertyName,pPropertyValue,pLengthValue,accessMode)	\
    ( (This)->lpVtbl -> GetNamedValue(This,pIccId,lengthIccId,pClientId,pPropertyName,pPropertyValue,pLengthValue,accessMode) ) 

#define IOemCellularDataStore_GetUiccDataStoreInfo(This,pDatastoreInfo)	\
    ( (This)->lpVtbl -> GetUiccDataStoreInfo(This,pDatastoreInfo) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOemCellularDataStore_INTERFACE_DEFINED__ */


#ifndef __IOemCellularModem_INTERFACE_DEFINED__
#define __IOemCellularModem_INTERFACE_DEFINED__

/* interface IOemCellularModem */
/* [uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IOemCellularModem;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("82F4E49E-1E6B-43C9-8769-FE3D437AE8A9")
    IOemCellularModem : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE RegisterForOemCanAvailabilityChanges( 
            /* [in] */ __RPC__in_opt IOemCellularCanAvailabilityChange *pCallBack) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE UnregisterForOemCanAvailabilityChanges( 
            /* [in] */ __RPC__in_opt IOemCellularCanAvailabilityChange *pCallback) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE RegisterForOemSlotChanges( 
            /* [in] */ __RPC__in_opt IOemSlotChange *pCallback) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE UnregisterForOemSlotChanges( 
            /* [in] */ __RPC__in_opt IOemSlotChange *pCallback) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE RegisterForPowerStateChanges( 
            /* [in] */ __RPC__in_opt IPowerStateChange *pCallback,
            /* [in] */ INT_PTR context) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE UnregisterForPowerStateChanges( 
            /* [in] */ __RPC__in_opt IPowerStateChange *pCallback) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SendModemOpaqueCommand( 
            /* [in] */ __RPC__in_opt IModemOpaqueCommandCompletion *pCallback,
            /* [size_is][in] */ __RPC__in_ecount_full(cbSize) BYTE *pOpaquePayload,
            /* [in] */ DWORD cbSize,
            /* [in] */ INT_PTR context) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE RegisterForOpaqueModemNotifications( 
            /* [in] */ __RPC__in_opt IOpaqueModemNotifications *pCallback) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE UnRegisterForOpaqueModemNotifications( 
            /* [in] */ __RPC__in_opt IOpaqueModemNotifications *pCallback) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetRFState( 
            /* [in] */ __RPC__in_opt ISetRFStateCompletion *pCallback,
            /* [in] */ DWORD dwRFPowerState,
            /* [in] */ INT_PTR context) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetRFState( 
            /* [in] */ __RPC__in_opt IGetRFStateCompletion *pCallback,
            /* [in] */ INT_PTR context) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOemCellularModemVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOemCellularModem * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOemCellularModem * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOemCellularModem * This);
        
        DECLSPEC_XFGVIRT(IOemCellularModem, RegisterForOemCanAvailabilityChanges)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *RegisterForOemCanAvailabilityChanges )( 
            __RPC__in IOemCellularModem * This,
            /* [in] */ __RPC__in_opt IOemCellularCanAvailabilityChange *pCallBack);
        
        DECLSPEC_XFGVIRT(IOemCellularModem, UnregisterForOemCanAvailabilityChanges)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *UnregisterForOemCanAvailabilityChanges )( 
            __RPC__in IOemCellularModem * This,
            /* [in] */ __RPC__in_opt IOemCellularCanAvailabilityChange *pCallback);
        
        DECLSPEC_XFGVIRT(IOemCellularModem, RegisterForOemSlotChanges)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *RegisterForOemSlotChanges )( 
            __RPC__in IOemCellularModem * This,
            /* [in] */ __RPC__in_opt IOemSlotChange *pCallback);
        
        DECLSPEC_XFGVIRT(IOemCellularModem, UnregisterForOemSlotChanges)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *UnregisterForOemSlotChanges )( 
            __RPC__in IOemCellularModem * This,
            /* [in] */ __RPC__in_opt IOemSlotChange *pCallback);
        
        DECLSPEC_XFGVIRT(IOemCellularModem, RegisterForPowerStateChanges)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *RegisterForPowerStateChanges )( 
            __RPC__in IOemCellularModem * This,
            /* [in] */ __RPC__in_opt IPowerStateChange *pCallback,
            /* [in] */ INT_PTR context);
        
        DECLSPEC_XFGVIRT(IOemCellularModem, UnregisterForPowerStateChanges)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *UnregisterForPowerStateChanges )( 
            __RPC__in IOemCellularModem * This,
            /* [in] */ __RPC__in_opt IPowerStateChange *pCallback);
        
        DECLSPEC_XFGVIRT(IOemCellularModem, SendModemOpaqueCommand)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SendModemOpaqueCommand )( 
            __RPC__in IOemCellularModem * This,
            /* [in] */ __RPC__in_opt IModemOpaqueCommandCompletion *pCallback,
            /* [size_is][in] */ __RPC__in_ecount_full(cbSize) BYTE *pOpaquePayload,
            /* [in] */ DWORD cbSize,
            /* [in] */ INT_PTR context);
        
        DECLSPEC_XFGVIRT(IOemCellularModem, RegisterForOpaqueModemNotifications)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *RegisterForOpaqueModemNotifications )( 
            __RPC__in IOemCellularModem * This,
            /* [in] */ __RPC__in_opt IOpaqueModemNotifications *pCallback);
        
        DECLSPEC_XFGVIRT(IOemCellularModem, UnRegisterForOpaqueModemNotifications)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *UnRegisterForOpaqueModemNotifications )( 
            __RPC__in IOemCellularModem * This,
            /* [in] */ __RPC__in_opt IOpaqueModemNotifications *pCallback);
        
        DECLSPEC_XFGVIRT(IOemCellularModem, SetRFState)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetRFState )( 
            __RPC__in IOemCellularModem * This,
            /* [in] */ __RPC__in_opt ISetRFStateCompletion *pCallback,
            /* [in] */ DWORD dwRFPowerState,
            /* [in] */ INT_PTR context);
        
        DECLSPEC_XFGVIRT(IOemCellularModem, GetRFState)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetRFState )( 
            __RPC__in IOemCellularModem * This,
            /* [in] */ __RPC__in_opt IGetRFStateCompletion *pCallback,
            /* [in] */ INT_PTR context);
        
        END_INTERFACE
    } IOemCellularModemVtbl;

    interface IOemCellularModem
    {
        CONST_VTBL struct IOemCellularModemVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOemCellularModem_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOemCellularModem_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOemCellularModem_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOemCellularModem_RegisterForOemCanAvailabilityChanges(This,pCallBack)	\
    ( (This)->lpVtbl -> RegisterForOemCanAvailabilityChanges(This,pCallBack) ) 

#define IOemCellularModem_UnregisterForOemCanAvailabilityChanges(This,pCallback)	\
    ( (This)->lpVtbl -> UnregisterForOemCanAvailabilityChanges(This,pCallback) ) 

#define IOemCellularModem_RegisterForOemSlotChanges(This,pCallback)	\
    ( (This)->lpVtbl -> RegisterForOemSlotChanges(This,pCallback) ) 

#define IOemCellularModem_UnregisterForOemSlotChanges(This,pCallback)	\
    ( (This)->lpVtbl -> UnregisterForOemSlotChanges(This,pCallback) ) 

#define IOemCellularModem_RegisterForPowerStateChanges(This,pCallback,context)	\
    ( (This)->lpVtbl -> RegisterForPowerStateChanges(This,pCallback,context) ) 

#define IOemCellularModem_UnregisterForPowerStateChanges(This,pCallback)	\
    ( (This)->lpVtbl -> UnregisterForPowerStateChanges(This,pCallback) ) 

#define IOemCellularModem_SendModemOpaqueCommand(This,pCallback,pOpaquePayload,cbSize,context)	\
    ( (This)->lpVtbl -> SendModemOpaqueCommand(This,pCallback,pOpaquePayload,cbSize,context) ) 

#define IOemCellularModem_RegisterForOpaqueModemNotifications(This,pCallback)	\
    ( (This)->lpVtbl -> RegisterForOpaqueModemNotifications(This,pCallback) ) 

#define IOemCellularModem_UnRegisterForOpaqueModemNotifications(This,pCallback)	\
    ( (This)->lpVtbl -> UnRegisterForOpaqueModemNotifications(This,pCallback) ) 

#define IOemCellularModem_SetRFState(This,pCallback,dwRFPowerState,context)	\
    ( (This)->lpVtbl -> SetRFState(This,pCallback,dwRFPowerState,context) ) 

#define IOemCellularModem_GetRFState(This,pCallback,context)	\
    ( (This)->lpVtbl -> GetRFState(This,pCallback,context) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOemCellularModem_INTERFACE_DEFINED__ */


#ifndef __IOemCellularModemEx_INTERFACE_DEFINED__
#define __IOemCellularModemEx_INTERFACE_DEFINED__

/* interface IOemCellularModemEx */
/* [uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IOemCellularModemEx;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1902f624-534e-4624-a815-3546021f0854")
    IOemCellularModemEx : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetRFState( 
            /* [in] */ __RPC__in_opt ISetRFStateCompletion *pCallback,
            /* [in] */ __RPC__in LPRILRFSTATE lpRFState,
            /* [in] */ INT_PTR context) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetRFState( 
            /* [in] */ __RPC__in_opt IGetRFStateExCompletion *pCallback,
            /* [in] */ INT_PTR context) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOemCellularModemExVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOemCellularModemEx * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOemCellularModemEx * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOemCellularModemEx * This);
        
        DECLSPEC_XFGVIRT(IOemCellularModemEx, SetRFState)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetRFState )( 
            __RPC__in IOemCellularModemEx * This,
            /* [in] */ __RPC__in_opt ISetRFStateCompletion *pCallback,
            /* [in] */ __RPC__in LPRILRFSTATE lpRFState,
            /* [in] */ INT_PTR context);
        
        DECLSPEC_XFGVIRT(IOemCellularModemEx, GetRFState)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetRFState )( 
            __RPC__in IOemCellularModemEx * This,
            /* [in] */ __RPC__in_opt IGetRFStateExCompletion *pCallback,
            /* [in] */ INT_PTR context);
        
        END_INTERFACE
    } IOemCellularModemExVtbl;

    interface IOemCellularModemEx
    {
        CONST_VTBL struct IOemCellularModemExVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOemCellularModemEx_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOemCellularModemEx_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOemCellularModemEx_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOemCellularModemEx_SetRFState(This,pCallback,lpRFState,context)	\
    ( (This)->lpVtbl -> SetRFState(This,pCallback,lpRFState,context) ) 

#define IOemCellularModemEx_GetRFState(This,pCallback,context)	\
    ( (This)->lpVtbl -> GetRFState(This,pCallback,context) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOemCellularModemEx_INTERFACE_DEFINED__ */


#ifndef __IOemCan_INTERFACE_DEFINED__
#define __IOemCan_INTERFACE_DEFINED__

/* interface IOemCan */
/* [uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IOemCan;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A3822E4E-FFB5-4E46-8980-0182B8454E2E")
    IOemCan : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE RegisterForOemCanRegistrationChanges( 
            /* [in] */ __RPC__in_opt IOemCanRegistrationStateChange *pCallback) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE UnregisterForOemCanRegistrationChanges( 
            /* [in] */ __RPC__in_opt IOemCanRegistrationStateChange *pCallback) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetInfo( 
            /* [in] */ enum RILDEVICEINFORMATION deviceInfo,
            /* [in] */ __RPC__in_opt ICanInfoCompletion *pCallback,
            /* [in] */ INT_PTR context) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetPositionInfo( 
            /* [in] */ __RPC__in_opt IPositionInfoCompletion *pCallback,
            /* [in] */ INT_PTR context) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE QueryService( 
            /* [in] */ __RPC__in REFGUID guidService,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOemCanVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOemCan * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOemCan * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOemCan * This);
        
        DECLSPEC_XFGVIRT(IOemCan, RegisterForOemCanRegistrationChanges)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *RegisterForOemCanRegistrationChanges )( 
            __RPC__in IOemCan * This,
            /* [in] */ __RPC__in_opt IOemCanRegistrationStateChange *pCallback);
        
        DECLSPEC_XFGVIRT(IOemCan, UnregisterForOemCanRegistrationChanges)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *UnregisterForOemCanRegistrationChanges )( 
            __RPC__in IOemCan * This,
            /* [in] */ __RPC__in_opt IOemCanRegistrationStateChange *pCallback);
        
        DECLSPEC_XFGVIRT(IOemCan, GetInfo)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetInfo )( 
            __RPC__in IOemCan * This,
            /* [in] */ enum RILDEVICEINFORMATION deviceInfo,
            /* [in] */ __RPC__in_opt ICanInfoCompletion *pCallback,
            /* [in] */ INT_PTR context);
        
        DECLSPEC_XFGVIRT(IOemCan, GetPositionInfo)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetPositionInfo )( 
            __RPC__in IOemCan * This,
            /* [in] */ __RPC__in_opt IPositionInfoCompletion *pCallback,
            /* [in] */ INT_PTR context);
        
        DECLSPEC_XFGVIRT(IOemCan, QueryService)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *QueryService )( 
            __RPC__in IOemCan * This,
            /* [in] */ __RPC__in REFGUID guidService,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv);
        
        END_INTERFACE
    } IOemCanVtbl;

    interface IOemCan
    {
        CONST_VTBL struct IOemCanVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOemCan_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOemCan_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOemCan_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOemCan_RegisterForOemCanRegistrationChanges(This,pCallback)	\
    ( (This)->lpVtbl -> RegisterForOemCanRegistrationChanges(This,pCallback) ) 

#define IOemCan_UnregisterForOemCanRegistrationChanges(This,pCallback)	\
    ( (This)->lpVtbl -> UnregisterForOemCanRegistrationChanges(This,pCallback) ) 

#define IOemCan_GetInfo(This,deviceInfo,pCallback,context)	\
    ( (This)->lpVtbl -> GetInfo(This,deviceInfo,pCallback,context) ) 

#define IOemCan_GetPositionInfo(This,pCallback,context)	\
    ( (This)->lpVtbl -> GetPositionInfo(This,pCallback,context) ) 

#define IOemCan_QueryService(This,guidService,riid,ppv)	\
    ( (This)->lpVtbl -> QueryService(This,guidService,riid,ppv) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOemCan_INTERFACE_DEFINED__ */


#ifndef __IOemCanExtForIMS_INTERFACE_DEFINED__
#define __IOemCanExtForIMS_INTERFACE_DEFINED__

/* interface IOemCanExtForIMS */
/* [uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IOemCanExtForIMS;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C5310A1A-E885-4450-B823-84AD7FD15CBF")
    IOemCanExtForIMS : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE RegisterForOemIMSStatusChanges( 
            /* [in] */ __RPC__in_opt IOemIMSStatusChange *pCallback) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE UnregisterForOemIMSStatusChanges( 
            /* [in] */ __RPC__in_opt IOemIMSStatusChange *pCallback) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOemCanExtForIMSVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOemCanExtForIMS * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOemCanExtForIMS * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOemCanExtForIMS * This);
        
        DECLSPEC_XFGVIRT(IOemCanExtForIMS, RegisterForOemIMSStatusChanges)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *RegisterForOemIMSStatusChanges )( 
            __RPC__in IOemCanExtForIMS * This,
            /* [in] */ __RPC__in_opt IOemIMSStatusChange *pCallback);
        
        DECLSPEC_XFGVIRT(IOemCanExtForIMS, UnregisterForOemIMSStatusChanges)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *UnregisterForOemIMSStatusChanges )( 
            __RPC__in IOemCanExtForIMS * This,
            /* [in] */ __RPC__in_opt IOemIMSStatusChange *pCallback);
        
        END_INTERFACE
    } IOemCanExtForIMSVtbl;

    interface IOemCanExtForIMS
    {
        CONST_VTBL struct IOemCanExtForIMSVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOemCanExtForIMS_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOemCanExtForIMS_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOemCanExtForIMS_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOemCanExtForIMS_RegisterForOemIMSStatusChanges(This,pCallback)	\
    ( (This)->lpVtbl -> RegisterForOemIMSStatusChanges(This,pCallback) ) 

#define IOemCanExtForIMS_UnregisterForOemIMSStatusChanges(This,pCallback)	\
    ( (This)->lpVtbl -> UnregisterForOemIMSStatusChanges(This,pCallback) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOemCanExtForIMS_INTERFACE_DEFINED__ */


#ifndef __IOemSlot_INTERFACE_DEFINED__
#define __IOemSlot_INTERFACE_DEFINED__

/* interface IOemSlot */
/* [uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IOemSlot;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DAE67DE6-0C18-4158-9323-CF7FB14EC216")
    IOemSlot : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE RegisterForOemSlotChanges( 
            /* [in] */ __RPC__in_opt IOemSlotStateChange *pCallback) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE UnregisterForOemSlotChanges( 
            /* [in] */ __RPC__in_opt IOemSlotStateChange *pCallback) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOemSlotVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOemSlot * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOemSlot * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOemSlot * This);
        
        DECLSPEC_XFGVIRT(IOemSlot, RegisterForOemSlotChanges)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *RegisterForOemSlotChanges )( 
            __RPC__in IOemSlot * This,
            /* [in] */ __RPC__in_opt IOemSlotStateChange *pCallback);
        
        DECLSPEC_XFGVIRT(IOemSlot, UnregisterForOemSlotChanges)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *UnregisterForOemSlotChanges )( 
            __RPC__in IOemSlot * This,
            /* [in] */ __RPC__in_opt IOemSlotStateChange *pCallback);
        
        END_INTERFACE
    } IOemSlotVtbl;

    interface IOemSlot
    {
        CONST_VTBL struct IOemSlotVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOemSlot_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOemSlot_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOemSlot_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOemSlot_RegisterForOemSlotChanges(This,pCallback)	\
    ( (This)->lpVtbl -> RegisterForOemSlotChanges(This,pCallback) ) 

#define IOemSlot_UnregisterForOemSlotChanges(This,pCallback)	\
    ( (This)->lpVtbl -> UnregisterForOemSlotChanges(This,pCallback) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOemSlot_INTERFACE_DEFINED__ */


#ifndef __IOemUicc_INTERFACE_DEFINED__
#define __IOemUicc_INTERFACE_DEFINED__

/* interface IOemUicc */
/* [uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IOemUicc;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("FD5DBCF6-810D-4CE8-AC06-1B8628808950")
    IOemUicc : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE RegisterForOemUiccChanges( 
            /* [in] */ __RPC__in_opt IOemUiccChange *pCallback) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE UnregisterForOemUiccChanges( 
            /* [in] */ __RPC__in_opt IOemUiccChange *pCallback) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOemUiccVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOemUicc * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOemUicc * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOemUicc * This);
        
        DECLSPEC_XFGVIRT(IOemUicc, RegisterForOemUiccChanges)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *RegisterForOemUiccChanges )( 
            __RPC__in IOemUicc * This,
            /* [in] */ __RPC__in_opt IOemUiccChange *pCallback);
        
        DECLSPEC_XFGVIRT(IOemUicc, UnregisterForOemUiccChanges)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *UnregisterForOemUiccChanges )( 
            __RPC__in IOemUicc * This,
            /* [in] */ __RPC__in_opt IOemUiccChange *pCallback);
        
        END_INTERFACE
    } IOemUiccVtbl;

    interface IOemUicc
    {
        CONST_VTBL struct IOemUiccVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOemUicc_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOemUicc_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOemUicc_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOemUicc_RegisterForOemUiccChanges(This,pCallback)	\
    ( (This)->lpVtbl -> RegisterForOemUiccChanges(This,pCallback) ) 

#define IOemUicc_UnregisterForOemUiccChanges(This,pCallback)	\
    ( (This)->lpVtbl -> UnregisterForOemUiccChanges(This,pCallback) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOemUicc_INTERFACE_DEFINED__ */


#ifndef __IOemUiccApp_INTERFACE_DEFINED__
#define __IOemUiccApp_INTERFACE_DEFINED__

/* interface IOemUiccApp */
/* [uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IOemUiccApp;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2D00BFEE-AA9F-48FD-9D2B-E8D3D3EDDC60")
    IOemUiccApp : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetAppId( 
            /* [out] */ __RPC__out BYTE *appId,
            /* [out][in] */ __RPC__inout DWORD *length) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetType( 
            /* [out] */ __RPC__out RILUICCAPPTYPE *pType) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetPinLockState( 
            /* [in] */ __RPC__in_opt IPinLockStateCompletion *pCallback,
            /* [in] */ INT_PTR context) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE ReadRecord( 
            /* [in] */ __RPC__in_opt IReadRecordCompletion *pCallback,
            /* [in] */ DWORD fileID,
            /* [in] */ DWORD recordIndex,
            /* [in] */ INT_PTR context) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE WriteRecord( 
            /* [in] */ __RPC__in_opt IWriteRecordCompletion *pCallback,
            /* [in] */ DWORD fileID,
            /* [in] */ DWORD recordIndex,
            /* [in] */ __RPC__in const BYTE *bData,
            /* [in] */ DWORD cbSize,
            /* [in] */ INT_PTR context) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetRecordStatusOnFilePath( 
            /* [in] */ __RPC__in_opt IGetRecordStatusCompletion *pCallback,
            /* [size_is][in] */ __RPC__in_ecount_full(filePathLen) const WORD *uiccFilePath,
            /* [in] */ DWORD filePathLen,
            /* [in] */ INT_PTR context) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE ReadRecordOnFilePath( 
            /* [in] */ __RPC__in_opt IReadRecordCompletion *pCallback,
            /* [size_is][in] */ __RPC__in_ecount_full(filePathLen) const WORD *uiccFilePath,
            /* [in] */ DWORD filePathLen,
            /* [in] */ DWORD recordIndex,
            /* [in] */ INT_PTR context) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE WriteRecordOnFilePath( 
            /* [in] */ __RPC__in_opt IWriteRecordCompletion *pCallback,
            /* [size_is][in] */ __RPC__in_ecount_full(filePathLen) const WORD *uiccFilePath,
            /* [in] */ DWORD filePathLen,
            /* [in] */ DWORD recordIndex,
            /* [in] */ __RPC__in const BYTE *bData,
            /* [in] */ DWORD cbSize,
            /* [in] */ INT_PTR context) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetIMSI( 
            /* [in] */ __RPC__in_opt IIMSICompletion *pCallback,
            /* [in] */ INT_PTR context) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetSIDNID( 
            /* [in] */ __RPC__in_opt IGetSIDNIDCompletion *pCallback,
            /* [in] */ INT_PTR context) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetSubscriberNumbers( 
            /* [in] */ __RPC__in_opt ISubscriberNumbersCompletion *pCallback,
            /* [in] */ INT_PTR context) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOemUiccAppVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOemUiccApp * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOemUiccApp * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOemUiccApp * This);
        
        DECLSPEC_XFGVIRT(IOemUiccApp, GetAppId)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetAppId )( 
            __RPC__in IOemUiccApp * This,
            /* [out] */ __RPC__out BYTE *appId,
            /* [out][in] */ __RPC__inout DWORD *length);
        
        DECLSPEC_XFGVIRT(IOemUiccApp, GetType)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetType )( 
            __RPC__in IOemUiccApp * This,
            /* [out] */ __RPC__out RILUICCAPPTYPE *pType);
        
        DECLSPEC_XFGVIRT(IOemUiccApp, GetPinLockState)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetPinLockState )( 
            __RPC__in IOemUiccApp * This,
            /* [in] */ __RPC__in_opt IPinLockStateCompletion *pCallback,
            /* [in] */ INT_PTR context);
        
        DECLSPEC_XFGVIRT(IOemUiccApp, ReadRecord)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ReadRecord )( 
            __RPC__in IOemUiccApp * This,
            /* [in] */ __RPC__in_opt IReadRecordCompletion *pCallback,
            /* [in] */ DWORD fileID,
            /* [in] */ DWORD recordIndex,
            /* [in] */ INT_PTR context);
        
        DECLSPEC_XFGVIRT(IOemUiccApp, WriteRecord)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *WriteRecord )( 
            __RPC__in IOemUiccApp * This,
            /* [in] */ __RPC__in_opt IWriteRecordCompletion *pCallback,
            /* [in] */ DWORD fileID,
            /* [in] */ DWORD recordIndex,
            /* [in] */ __RPC__in const BYTE *bData,
            /* [in] */ DWORD cbSize,
            /* [in] */ INT_PTR context);
        
        DECLSPEC_XFGVIRT(IOemUiccApp, GetRecordStatusOnFilePath)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetRecordStatusOnFilePath )( 
            __RPC__in IOemUiccApp * This,
            /* [in] */ __RPC__in_opt IGetRecordStatusCompletion *pCallback,
            /* [size_is][in] */ __RPC__in_ecount_full(filePathLen) const WORD *uiccFilePath,
            /* [in] */ DWORD filePathLen,
            /* [in] */ INT_PTR context);
        
        DECLSPEC_XFGVIRT(IOemUiccApp, ReadRecordOnFilePath)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ReadRecordOnFilePath )( 
            __RPC__in IOemUiccApp * This,
            /* [in] */ __RPC__in_opt IReadRecordCompletion *pCallback,
            /* [size_is][in] */ __RPC__in_ecount_full(filePathLen) const WORD *uiccFilePath,
            /* [in] */ DWORD filePathLen,
            /* [in] */ DWORD recordIndex,
            /* [in] */ INT_PTR context);
        
        DECLSPEC_XFGVIRT(IOemUiccApp, WriteRecordOnFilePath)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *WriteRecordOnFilePath )( 
            __RPC__in IOemUiccApp * This,
            /* [in] */ __RPC__in_opt IWriteRecordCompletion *pCallback,
            /* [size_is][in] */ __RPC__in_ecount_full(filePathLen) const WORD *uiccFilePath,
            /* [in] */ DWORD filePathLen,
            /* [in] */ DWORD recordIndex,
            /* [in] */ __RPC__in const BYTE *bData,
            /* [in] */ DWORD cbSize,
            /* [in] */ INT_PTR context);
        
        DECLSPEC_XFGVIRT(IOemUiccApp, GetIMSI)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetIMSI )( 
            __RPC__in IOemUiccApp * This,
            /* [in] */ __RPC__in_opt IIMSICompletion *pCallback,
            /* [in] */ INT_PTR context);
        
        DECLSPEC_XFGVIRT(IOemUiccApp, GetSIDNID)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetSIDNID )( 
            __RPC__in IOemUiccApp * This,
            /* [in] */ __RPC__in_opt IGetSIDNIDCompletion *pCallback,
            /* [in] */ INT_PTR context);
        
        DECLSPEC_XFGVIRT(IOemUiccApp, GetSubscriberNumbers)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetSubscriberNumbers )( 
            __RPC__in IOemUiccApp * This,
            /* [in] */ __RPC__in_opt ISubscriberNumbersCompletion *pCallback,
            /* [in] */ INT_PTR context);
        
        END_INTERFACE
    } IOemUiccAppVtbl;

    interface IOemUiccApp
    {
        CONST_VTBL struct IOemUiccAppVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOemUiccApp_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOemUiccApp_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOemUiccApp_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOemUiccApp_GetAppId(This,appId,length)	\
    ( (This)->lpVtbl -> GetAppId(This,appId,length) ) 

#define IOemUiccApp_GetType(This,pType)	\
    ( (This)->lpVtbl -> GetType(This,pType) ) 

#define IOemUiccApp_GetPinLockState(This,pCallback,context)	\
    ( (This)->lpVtbl -> GetPinLockState(This,pCallback,context) ) 

#define IOemUiccApp_ReadRecord(This,pCallback,fileID,recordIndex,context)	\
    ( (This)->lpVtbl -> ReadRecord(This,pCallback,fileID,recordIndex,context) ) 

#define IOemUiccApp_WriteRecord(This,pCallback,fileID,recordIndex,bData,cbSize,context)	\
    ( (This)->lpVtbl -> WriteRecord(This,pCallback,fileID,recordIndex,bData,cbSize,context) ) 

#define IOemUiccApp_GetRecordStatusOnFilePath(This,pCallback,uiccFilePath,filePathLen,context)	\
    ( (This)->lpVtbl -> GetRecordStatusOnFilePath(This,pCallback,uiccFilePath,filePathLen,context) ) 

#define IOemUiccApp_ReadRecordOnFilePath(This,pCallback,uiccFilePath,filePathLen,recordIndex,context)	\
    ( (This)->lpVtbl -> ReadRecordOnFilePath(This,pCallback,uiccFilePath,filePathLen,recordIndex,context) ) 

#define IOemUiccApp_WriteRecordOnFilePath(This,pCallback,uiccFilePath,filePathLen,recordIndex,bData,cbSize,context)	\
    ( (This)->lpVtbl -> WriteRecordOnFilePath(This,pCallback,uiccFilePath,filePathLen,recordIndex,bData,cbSize,context) ) 

#define IOemUiccApp_GetIMSI(This,pCallback,context)	\
    ( (This)->lpVtbl -> GetIMSI(This,pCallback,context) ) 

#define IOemUiccApp_GetSIDNID(This,pCallback,context)	\
    ( (This)->lpVtbl -> GetSIDNID(This,pCallback,context) ) 

#define IOemUiccApp_GetSubscriberNumbers(This,pCallback,context)	\
    ( (This)->lpVtbl -> GetSubscriberNumbers(This,pCallback,context) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOemUiccApp_INTERFACE_DEFINED__ */


#ifndef __IOemUiccAppEx_INTERFACE_DEFINED__
#define __IOemUiccAppEx_INTERFACE_DEFINED__

/* interface IOemUiccAppEx */
/* [uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IOemUiccAppEx;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0C50366C-D5ED-4F00-AA54-1537A63D8A01")
    IOemUiccAppEx : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetPreferredOperatorList( 
            /* [in] */ __RPC__in_opt IGetPLMNwAcT *pResponseHandler) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetPreferredOperatorList( 
            /* [in] */ __RPC__in_opt ISetPLMNwAcT *pResponseHandler,
            /* [size_is][in] */ __RPC__in_ecount_full(dwLength) RILOPERATORNAMES *pOperatorList,
            /* [in] */ DWORD dwLength) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOemUiccAppExVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOemUiccAppEx * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOemUiccAppEx * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOemUiccAppEx * This);
        
        DECLSPEC_XFGVIRT(IOemUiccAppEx, GetPreferredOperatorList)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetPreferredOperatorList )( 
            __RPC__in IOemUiccAppEx * This,
            /* [in] */ __RPC__in_opt IGetPLMNwAcT *pResponseHandler);
        
        DECLSPEC_XFGVIRT(IOemUiccAppEx, SetPreferredOperatorList)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetPreferredOperatorList )( 
            __RPC__in IOemUiccAppEx * This,
            /* [in] */ __RPC__in_opt ISetPLMNwAcT *pResponseHandler,
            /* [size_is][in] */ __RPC__in_ecount_full(dwLength) RILOPERATORNAMES *pOperatorList,
            /* [in] */ DWORD dwLength);
        
        END_INTERFACE
    } IOemUiccAppExVtbl;

    interface IOemUiccAppEx
    {
        CONST_VTBL struct IOemUiccAppExVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOemUiccAppEx_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOemUiccAppEx_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOemUiccAppEx_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOemUiccAppEx_GetPreferredOperatorList(This,pResponseHandler)	\
    ( (This)->lpVtbl -> GetPreferredOperatorList(This,pResponseHandler) ) 

#define IOemUiccAppEx_SetPreferredOperatorList(This,pResponseHandler,pOperatorList,dwLength)	\
    ( (This)->lpVtbl -> SetPreferredOperatorList(This,pResponseHandler,pOperatorList,dwLength) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOemUiccAppEx_INTERFACE_DEFINED__ */


#ifndef __IOemUiccAppEx2_INTERFACE_DEFINED__
#define __IOemUiccAppEx2_INTERFACE_DEFINED__

/* interface IOemUiccAppEx2 */
/* [uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IOemUiccAppEx2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C9772450-A940-4E27-A3EA-AFCC5ECC7556")
    IOemUiccAppEx2 : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetNAI( 
            /* [in] */ __RPC__in_opt IGetNAICompletion *pCallback,
            /* [in] */ INT_PTR context) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOemUiccAppEx2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOemUiccAppEx2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOemUiccAppEx2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOemUiccAppEx2 * This);
        
        DECLSPEC_XFGVIRT(IOemUiccAppEx2, GetNAI)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetNAI )( 
            __RPC__in IOemUiccAppEx2 * This,
            /* [in] */ __RPC__in_opt IGetNAICompletion *pCallback,
            /* [in] */ INT_PTR context);
        
        END_INTERFACE
    } IOemUiccAppEx2Vtbl;

    interface IOemUiccAppEx2
    {
        CONST_VTBL struct IOemUiccAppEx2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOemUiccAppEx2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOemUiccAppEx2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOemUiccAppEx2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOemUiccAppEx2_GetNAI(This,pCallback,context)	\
    ( (This)->lpVtbl -> GetNAI(This,pCallback,context) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOemUiccAppEx2_INTERFACE_DEFINED__ */


#ifndef __IOem3GPPSupServices_INTERFACE_DEFINED__
#define __IOem3GPPSupServices_INTERFACE_DEFINED__

/* interface IOem3GPPSupServices */
/* [uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IOem3GPPSupServices;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("138820C5-299D-4d1d-80B9-1664A17B1B41")
    IOem3GPPSupServices : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetCallForwardingSettings( 
            /* [in] */ __RPC__in_opt IGetCallForwardingCompletion *pCallback,
            /* [in] */ INT_PTR context,
            /* [in] */ RILCALLFORWARDINGSETTINGSREASON reason,
            /* [in] */ BOOL allClasses,
            /* [in] */ DWORD infoClasses) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetCallForwardingStatus( 
            /* [in] */ __RPC__in_opt ISimpleModemCompletion *pCallback,
            /* [in] */ INT_PTR context,
            /* [in] */ RILCALLFORWARDINGSETTINGSREASON dwReason,
            /* [in] */ BOOL fAllClasses,
            /* [in] */ DWORD dwInfoClasses,
            /* [in] */ RILSERVICESETTINGSSTATUS dwStatus) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE AddCallForwarding( 
            /* [in] */ __RPC__in_opt ISimpleModemCompletion *pCallback,
            /* [in] */ INT_PTR context,
            /* [in] */ RILCALLFORWARDINGSETTINGSREASON dwReason,
            /* [in] */ __RPC__in const RILCALLFORWARDINGSETTINGS *lpSettings) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE RemoveCallForwarding( 
            /* [in] */ __RPC__in_opt ISimpleModemCompletion *pCallback,
            /* [in] */ INT_PTR context,
            /* [in] */ RILCALLFORWARDINGSETTINGSREASON dwReason,
            /* [in] */ DWORD dwInfoClasses) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetCallWaitingSettings( 
            /* [in] */ __RPC__in_opt ICallWaitingSettingsCompletion *pCallback,
            /* [in] */ INT_PTR context,
            /* [in] */ BOOL fAllClasses,
            /* [in] */ DWORD dwInfoClasses) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetCallWaitingSettings( 
            /* [in] */ __RPC__in_opt ISimpleModemCompletion *pCallback,
            /* [in] */ INT_PTR context,
            /* [in] */ BOOL fAllClasses,
            /* [in] */ DWORD dwInfoClasses,
            /* [in] */ RILSERVICESETTINGSSTATUS dwStatus) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOem3GPPSupServicesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOem3GPPSupServices * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOem3GPPSupServices * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOem3GPPSupServices * This);
        
        DECLSPEC_XFGVIRT(IOem3GPPSupServices, GetCallForwardingSettings)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetCallForwardingSettings )( 
            __RPC__in IOem3GPPSupServices * This,
            /* [in] */ __RPC__in_opt IGetCallForwardingCompletion *pCallback,
            /* [in] */ INT_PTR context,
            /* [in] */ RILCALLFORWARDINGSETTINGSREASON reason,
            /* [in] */ BOOL allClasses,
            /* [in] */ DWORD infoClasses);
        
        DECLSPEC_XFGVIRT(IOem3GPPSupServices, SetCallForwardingStatus)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetCallForwardingStatus )( 
            __RPC__in IOem3GPPSupServices * This,
            /* [in] */ __RPC__in_opt ISimpleModemCompletion *pCallback,
            /* [in] */ INT_PTR context,
            /* [in] */ RILCALLFORWARDINGSETTINGSREASON dwReason,
            /* [in] */ BOOL fAllClasses,
            /* [in] */ DWORD dwInfoClasses,
            /* [in] */ RILSERVICESETTINGSSTATUS dwStatus);
        
        DECLSPEC_XFGVIRT(IOem3GPPSupServices, AddCallForwarding)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AddCallForwarding )( 
            __RPC__in IOem3GPPSupServices * This,
            /* [in] */ __RPC__in_opt ISimpleModemCompletion *pCallback,
            /* [in] */ INT_PTR context,
            /* [in] */ RILCALLFORWARDINGSETTINGSREASON dwReason,
            /* [in] */ __RPC__in const RILCALLFORWARDINGSETTINGS *lpSettings);
        
        DECLSPEC_XFGVIRT(IOem3GPPSupServices, RemoveCallForwarding)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *RemoveCallForwarding )( 
            __RPC__in IOem3GPPSupServices * This,
            /* [in] */ __RPC__in_opt ISimpleModemCompletion *pCallback,
            /* [in] */ INT_PTR context,
            /* [in] */ RILCALLFORWARDINGSETTINGSREASON dwReason,
            /* [in] */ DWORD dwInfoClasses);
        
        DECLSPEC_XFGVIRT(IOem3GPPSupServices, GetCallWaitingSettings)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetCallWaitingSettings )( 
            __RPC__in IOem3GPPSupServices * This,
            /* [in] */ __RPC__in_opt ICallWaitingSettingsCompletion *pCallback,
            /* [in] */ INT_PTR context,
            /* [in] */ BOOL fAllClasses,
            /* [in] */ DWORD dwInfoClasses);
        
        DECLSPEC_XFGVIRT(IOem3GPPSupServices, SetCallWaitingSettings)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetCallWaitingSettings )( 
            __RPC__in IOem3GPPSupServices * This,
            /* [in] */ __RPC__in_opt ISimpleModemCompletion *pCallback,
            /* [in] */ INT_PTR context,
            /* [in] */ BOOL fAllClasses,
            /* [in] */ DWORD dwInfoClasses,
            /* [in] */ RILSERVICESETTINGSSTATUS dwStatus);
        
        END_INTERFACE
    } IOem3GPPSupServicesVtbl;

    interface IOem3GPPSupServices
    {
        CONST_VTBL struct IOem3GPPSupServicesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOem3GPPSupServices_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOem3GPPSupServices_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOem3GPPSupServices_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOem3GPPSupServices_GetCallForwardingSettings(This,pCallback,context,reason,allClasses,infoClasses)	\
    ( (This)->lpVtbl -> GetCallForwardingSettings(This,pCallback,context,reason,allClasses,infoClasses) ) 

#define IOem3GPPSupServices_SetCallForwardingStatus(This,pCallback,context,dwReason,fAllClasses,dwInfoClasses,dwStatus)	\
    ( (This)->lpVtbl -> SetCallForwardingStatus(This,pCallback,context,dwReason,fAllClasses,dwInfoClasses,dwStatus) ) 

#define IOem3GPPSupServices_AddCallForwarding(This,pCallback,context,dwReason,lpSettings)	\
    ( (This)->lpVtbl -> AddCallForwarding(This,pCallback,context,dwReason,lpSettings) ) 

#define IOem3GPPSupServices_RemoveCallForwarding(This,pCallback,context,dwReason,dwInfoClasses)	\
    ( (This)->lpVtbl -> RemoveCallForwarding(This,pCallback,context,dwReason,dwInfoClasses) ) 

#define IOem3GPPSupServices_GetCallWaitingSettings(This,pCallback,context,fAllClasses,dwInfoClasses)	\
    ( (This)->lpVtbl -> GetCallWaitingSettings(This,pCallback,context,fAllClasses,dwInfoClasses) ) 

#define IOem3GPPSupServices_SetCallWaitingSettings(This,pCallback,context,fAllClasses,dwInfoClasses,dwStatus)	\
    ( (This)->lpVtbl -> SetCallWaitingSettings(This,pCallback,context,fAllClasses,dwInfoClasses,dwStatus) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOem3GPPSupServices_INTERFACE_DEFINED__ */


#ifndef __IOemCellularModemExistenceChange_INTERFACE_DEFINED__
#define __IOemCellularModemExistenceChange_INTERFACE_DEFINED__

/* interface IOemCellularModemExistenceChange */
/* [uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IOemCellularModemExistenceChange;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A2CBC3C1-79D5-4F9D-A6F9-FDE497848EC1")
    IOemCellularModemExistenceChange : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE OnOemModemAdded( 
            /* [in] */ __RPC__in_opt IOemCellularModem *pModem) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE OnOemModemRemoved( 
            /* [in] */ __RPC__in_opt IOemCellularModem *pModem) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE OnOemModemExistenceDone( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOemCellularModemExistenceChangeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOemCellularModemExistenceChange * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOemCellularModemExistenceChange * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOemCellularModemExistenceChange * This);
        
        DECLSPEC_XFGVIRT(IOemCellularModemExistenceChange, OnOemModemAdded)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *OnOemModemAdded )( 
            __RPC__in IOemCellularModemExistenceChange * This,
            /* [in] */ __RPC__in_opt IOemCellularModem *pModem);
        
        DECLSPEC_XFGVIRT(IOemCellularModemExistenceChange, OnOemModemRemoved)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *OnOemModemRemoved )( 
            __RPC__in IOemCellularModemExistenceChange * This,
            /* [in] */ __RPC__in_opt IOemCellularModem *pModem);
        
        DECLSPEC_XFGVIRT(IOemCellularModemExistenceChange, OnOemModemExistenceDone)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *OnOemModemExistenceDone )( 
            __RPC__in IOemCellularModemExistenceChange * This);
        
        END_INTERFACE
    } IOemCellularModemExistenceChangeVtbl;

    interface IOemCellularModemExistenceChange
    {
        CONST_VTBL struct IOemCellularModemExistenceChangeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOemCellularModemExistenceChange_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOemCellularModemExistenceChange_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOemCellularModemExistenceChange_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOemCellularModemExistenceChange_OnOemModemAdded(This,pModem)	\
    ( (This)->lpVtbl -> OnOemModemAdded(This,pModem) ) 

#define IOemCellularModemExistenceChange_OnOemModemRemoved(This,pModem)	\
    ( (This)->lpVtbl -> OnOemModemRemoved(This,pModem) ) 

#define IOemCellularModemExistenceChange_OnOemModemExistenceDone(This)	\
    ( (This)->lpVtbl -> OnOemModemExistenceDone(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOemCellularModemExistenceChange_INTERFACE_DEFINED__ */


#ifndef __IOemCellularCanAvailabilityChange_INTERFACE_DEFINED__
#define __IOemCellularCanAvailabilityChange_INTERFACE_DEFINED__

/* interface IOemCellularCanAvailabilityChange */
/* [uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IOemCellularCanAvailabilityChange;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("CF34841C-A795-4EC6-B1BC-C3744A2D6DCA")
    IOemCellularCanAvailabilityChange : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE OnOemCanAdded( 
            /* [in] */ __RPC__in_opt IOemCan *pCan) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE OnOemCanRemoved( 
            /* [in] */ __RPC__in_opt IOemCan *pCan) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE OnOemCanDone( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOemCellularCanAvailabilityChangeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOemCellularCanAvailabilityChange * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOemCellularCanAvailabilityChange * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOemCellularCanAvailabilityChange * This);
        
        DECLSPEC_XFGVIRT(IOemCellularCanAvailabilityChange, OnOemCanAdded)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *OnOemCanAdded )( 
            __RPC__in IOemCellularCanAvailabilityChange * This,
            /* [in] */ __RPC__in_opt IOemCan *pCan);
        
        DECLSPEC_XFGVIRT(IOemCellularCanAvailabilityChange, OnOemCanRemoved)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *OnOemCanRemoved )( 
            __RPC__in IOemCellularCanAvailabilityChange * This,
            /* [in] */ __RPC__in_opt IOemCan *pCan);
        
        DECLSPEC_XFGVIRT(IOemCellularCanAvailabilityChange, OnOemCanDone)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *OnOemCanDone )( 
            __RPC__in IOemCellularCanAvailabilityChange * This);
        
        END_INTERFACE
    } IOemCellularCanAvailabilityChangeVtbl;

    interface IOemCellularCanAvailabilityChange
    {
        CONST_VTBL struct IOemCellularCanAvailabilityChangeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOemCellularCanAvailabilityChange_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOemCellularCanAvailabilityChange_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOemCellularCanAvailabilityChange_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOemCellularCanAvailabilityChange_OnOemCanAdded(This,pCan)	\
    ( (This)->lpVtbl -> OnOemCanAdded(This,pCan) ) 

#define IOemCellularCanAvailabilityChange_OnOemCanRemoved(This,pCan)	\
    ( (This)->lpVtbl -> OnOemCanRemoved(This,pCan) ) 

#define IOemCellularCanAvailabilityChange_OnOemCanDone(This)	\
    ( (This)->lpVtbl -> OnOemCanDone(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOemCellularCanAvailabilityChange_INTERFACE_DEFINED__ */


#ifndef __IOemSlotChange_INTERFACE_DEFINED__
#define __IOemSlotChange_INTERFACE_DEFINED__

/* interface IOemSlotChange */
/* [uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IOemSlotChange;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("20C4AF87-BC47-4150-BA97-4F53FFD95E48")
    IOemSlotChange : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE OnOemSlotAdded( 
            /* [in] */ __RPC__in_opt IOemSlot *pSlot,
            /* [in] */ __RPC__in_opt IOemCellularModem *pCellularModem) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE OnOemSlotRemoved( 
            /* [in] */ __RPC__in_opt IOemSlot *pSlot,
            /* [in] */ __RPC__in_opt IOemCellularModem *pCellularModem) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE OnOemSlotExistenceDone( 
            /* [in] */ __RPC__in_opt IOemCellularModem *pCellularModem) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOemSlotChangeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOemSlotChange * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOemSlotChange * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOemSlotChange * This);
        
        DECLSPEC_XFGVIRT(IOemSlotChange, OnOemSlotAdded)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *OnOemSlotAdded )( 
            __RPC__in IOemSlotChange * This,
            /* [in] */ __RPC__in_opt IOemSlot *pSlot,
            /* [in] */ __RPC__in_opt IOemCellularModem *pCellularModem);
        
        DECLSPEC_XFGVIRT(IOemSlotChange, OnOemSlotRemoved)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *OnOemSlotRemoved )( 
            __RPC__in IOemSlotChange * This,
            /* [in] */ __RPC__in_opt IOemSlot *pSlot,
            /* [in] */ __RPC__in_opt IOemCellularModem *pCellularModem);
        
        DECLSPEC_XFGVIRT(IOemSlotChange, OnOemSlotExistenceDone)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *OnOemSlotExistenceDone )( 
            __RPC__in IOemSlotChange * This,
            /* [in] */ __RPC__in_opt IOemCellularModem *pCellularModem);
        
        END_INTERFACE
    } IOemSlotChangeVtbl;

    interface IOemSlotChange
    {
        CONST_VTBL struct IOemSlotChangeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOemSlotChange_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOemSlotChange_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOemSlotChange_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOemSlotChange_OnOemSlotAdded(This,pSlot,pCellularModem)	\
    ( (This)->lpVtbl -> OnOemSlotAdded(This,pSlot,pCellularModem) ) 

#define IOemSlotChange_OnOemSlotRemoved(This,pSlot,pCellularModem)	\
    ( (This)->lpVtbl -> OnOemSlotRemoved(This,pSlot,pCellularModem) ) 

#define IOemSlotChange_OnOemSlotExistenceDone(This,pCellularModem)	\
    ( (This)->lpVtbl -> OnOemSlotExistenceDone(This,pCellularModem) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOemSlotChange_INTERFACE_DEFINED__ */


#ifndef __IOemCanRegistrationStateChange_INTERFACE_DEFINED__
#define __IOemCanRegistrationStateChange_INTERFACE_DEFINED__

/* interface IOemCanRegistrationStateChange */
/* [uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IOemCanRegistrationStateChange;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B8E5DF58-2329-400C-93C5-FFB951BF3415")
    IOemCanRegistrationStateChange : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE OnOemRegistrationStatusChanged( 
            /* [size_is][in] */ __RPC__in_ecount_full(cStatus) IOemRegistrationStatus **status,
            /* [in] */ DWORD cStatus) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOemCanRegistrationStateChangeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOemCanRegistrationStateChange * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOemCanRegistrationStateChange * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOemCanRegistrationStateChange * This);
        
        DECLSPEC_XFGVIRT(IOemCanRegistrationStateChange, OnOemRegistrationStatusChanged)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *OnOemRegistrationStatusChanged )( 
            __RPC__in IOemCanRegistrationStateChange * This,
            /* [size_is][in] */ __RPC__in_ecount_full(cStatus) IOemRegistrationStatus **status,
            /* [in] */ DWORD cStatus);
        
        END_INTERFACE
    } IOemCanRegistrationStateChangeVtbl;

    interface IOemCanRegistrationStateChange
    {
        CONST_VTBL struct IOemCanRegistrationStateChangeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOemCanRegistrationStateChange_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOemCanRegistrationStateChange_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOemCanRegistrationStateChange_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOemCanRegistrationStateChange_OnOemRegistrationStatusChanged(This,status,cStatus)	\
    ( (This)->lpVtbl -> OnOemRegistrationStatusChanged(This,status,cStatus) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOemCanRegistrationStateChange_INTERFACE_DEFINED__ */


#ifndef __IOemSlotStateChange_INTERFACE_DEFINED__
#define __IOemSlotStateChange_INTERFACE_DEFINED__

/* interface IOemSlotStateChange */
/* [uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IOemSlotStateChange;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("59BB79ED-5435-456C-A8D1-6680D56AAD1E")
    IOemSlotStateChange : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE OnOemSlotStateChanged( 
            /* [in] */ RILUICCSLOTSTATE currentSlotState,
            /* [in] */ __RPC__in_opt IOemSlot *pSlot) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE OnOemUiccAdded( 
            /* [in] */ __RPC__in_opt IOemUicc *pUicc,
            /* [in] */ __RPC__in_opt IOemSlot *pSlot) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE OnOemUiccRemoved( 
            /* [in] */ __RPC__in_opt IOemUicc *pUicc,
            /* [in] */ __RPC__in_opt IOemSlot *pSlot) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE OnOemFirstSnapshotDone( 
            /* [in] */ __RPC__in_opt IOemSlot *pSlot) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOemSlotStateChangeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOemSlotStateChange * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOemSlotStateChange * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOemSlotStateChange * This);
        
        DECLSPEC_XFGVIRT(IOemSlotStateChange, OnOemSlotStateChanged)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *OnOemSlotStateChanged )( 
            __RPC__in IOemSlotStateChange * This,
            /* [in] */ RILUICCSLOTSTATE currentSlotState,
            /* [in] */ __RPC__in_opt IOemSlot *pSlot);
        
        DECLSPEC_XFGVIRT(IOemSlotStateChange, OnOemUiccAdded)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *OnOemUiccAdded )( 
            __RPC__in IOemSlotStateChange * This,
            /* [in] */ __RPC__in_opt IOemUicc *pUicc,
            /* [in] */ __RPC__in_opt IOemSlot *pSlot);
        
        DECLSPEC_XFGVIRT(IOemSlotStateChange, OnOemUiccRemoved)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *OnOemUiccRemoved )( 
            __RPC__in IOemSlotStateChange * This,
            /* [in] */ __RPC__in_opt IOemUicc *pUicc,
            /* [in] */ __RPC__in_opt IOemSlot *pSlot);
        
        DECLSPEC_XFGVIRT(IOemSlotStateChange, OnOemFirstSnapshotDone)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *OnOemFirstSnapshotDone )( 
            __RPC__in IOemSlotStateChange * This,
            /* [in] */ __RPC__in_opt IOemSlot *pSlot);
        
        END_INTERFACE
    } IOemSlotStateChangeVtbl;

    interface IOemSlotStateChange
    {
        CONST_VTBL struct IOemSlotStateChangeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOemSlotStateChange_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOemSlotStateChange_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOemSlotStateChange_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOemSlotStateChange_OnOemSlotStateChanged(This,currentSlotState,pSlot)	\
    ( (This)->lpVtbl -> OnOemSlotStateChanged(This,currentSlotState,pSlot) ) 

#define IOemSlotStateChange_OnOemUiccAdded(This,pUicc,pSlot)	\
    ( (This)->lpVtbl -> OnOemUiccAdded(This,pUicc,pSlot) ) 

#define IOemSlotStateChange_OnOemUiccRemoved(This,pUicc,pSlot)	\
    ( (This)->lpVtbl -> OnOemUiccRemoved(This,pUicc,pSlot) ) 

#define IOemSlotStateChange_OnOemFirstSnapshotDone(This,pSlot)	\
    ( (This)->lpVtbl -> OnOemFirstSnapshotDone(This,pSlot) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOemSlotStateChange_INTERFACE_DEFINED__ */


#ifndef __IOemUiccChange_INTERFACE_DEFINED__
#define __IOemUiccChange_INTERFACE_DEFINED__

/* interface IOemUiccChange */
/* [uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IOemUiccChange;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("302739D6-299D-4B82-A12E-EF930FD8BD16")
    IOemUiccChange : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE UiccIccId( 
            /* [in] */ __RPC__in BYTE *iccID,
            /* [in] */ DWORD length,
            /* [in] */ __RPC__in_opt IOemUicc *pUicc) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE OnOemUiccAppAdded( 
            /* [in] */ __RPC__in_opt IOemUiccApp *uiccApp,
            /* [in] */ __RPC__in_opt IOemUicc *pUicc) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE OnOemUiccAppRemoved( 
            /* [in] */ __RPC__in_opt IOemUiccApp *uiccApp,
            /* [in] */ __RPC__in_opt IOemUicc *pUicc) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE OnOemUiccAppFetchDone( 
            /* [in] */ __RPC__in_opt IOemUicc *pUicc) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOemUiccChangeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOemUiccChange * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOemUiccChange * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOemUiccChange * This);
        
        DECLSPEC_XFGVIRT(IOemUiccChange, UiccIccId)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *UiccIccId )( 
            __RPC__in IOemUiccChange * This,
            /* [in] */ __RPC__in BYTE *iccID,
            /* [in] */ DWORD length,
            /* [in] */ __RPC__in_opt IOemUicc *pUicc);
        
        DECLSPEC_XFGVIRT(IOemUiccChange, OnOemUiccAppAdded)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *OnOemUiccAppAdded )( 
            __RPC__in IOemUiccChange * This,
            /* [in] */ __RPC__in_opt IOemUiccApp *uiccApp,
            /* [in] */ __RPC__in_opt IOemUicc *pUicc);
        
        DECLSPEC_XFGVIRT(IOemUiccChange, OnOemUiccAppRemoved)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *OnOemUiccAppRemoved )( 
            __RPC__in IOemUiccChange * This,
            /* [in] */ __RPC__in_opt IOemUiccApp *uiccApp,
            /* [in] */ __RPC__in_opt IOemUicc *pUicc);
        
        DECLSPEC_XFGVIRT(IOemUiccChange, OnOemUiccAppFetchDone)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *OnOemUiccAppFetchDone )( 
            __RPC__in IOemUiccChange * This,
            /* [in] */ __RPC__in_opt IOemUicc *pUicc);
        
        END_INTERFACE
    } IOemUiccChangeVtbl;

    interface IOemUiccChange
    {
        CONST_VTBL struct IOemUiccChangeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOemUiccChange_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOemUiccChange_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOemUiccChange_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOemUiccChange_UiccIccId(This,iccID,length,pUicc)	\
    ( (This)->lpVtbl -> UiccIccId(This,iccID,length,pUicc) ) 

#define IOemUiccChange_OnOemUiccAppAdded(This,uiccApp,pUicc)	\
    ( (This)->lpVtbl -> OnOemUiccAppAdded(This,uiccApp,pUicc) ) 

#define IOemUiccChange_OnOemUiccAppRemoved(This,uiccApp,pUicc)	\
    ( (This)->lpVtbl -> OnOemUiccAppRemoved(This,uiccApp,pUicc) ) 

#define IOemUiccChange_OnOemUiccAppFetchDone(This,pUicc)	\
    ( (This)->lpVtbl -> OnOemUiccAppFetchDone(This,pUicc) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOemUiccChange_INTERFACE_DEFINED__ */


#ifndef __IOemRegistrationStatus_INTERFACE_DEFINED__
#define __IOemRegistrationStatus_INTERFACE_DEFINED__

/* interface IOemRegistrationStatus */
/* [uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IOemRegistrationStatus;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F8C3B6D1-4629-414B-8E38-DD396FEBE730")
    IOemRegistrationStatus : public IUnknown
    {
    public:
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Subscription( 
            /* [retval][out] */ __RPC__deref_out_opt IOemUiccApp **ret) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_SystemType( 
            /* [retval][out] */ __RPC__out enum RILSYSTEMTYPE *systemType) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_RejectReason( 
            /* [retval][out] */ __RPC__out DWORD *reason) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_OperatorName( 
            /* [retval][out] */ __RPC__out struct RILOPERATORNAMES *name) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_RegistrationStatus( 
            /* [retval][out] */ __RPC__out enum RILREGSTAT *status) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_NetworkCode( 
            /* [retval][out] */ __RPC__out struct RILNETWORKCODE *pNetworkCode) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOemRegistrationStatusVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOemRegistrationStatus * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOemRegistrationStatus * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOemRegistrationStatus * This);
        
        DECLSPEC_XFGVIRT(IOemRegistrationStatus, get_Subscription)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Subscription )( 
            __RPC__in IOemRegistrationStatus * This,
            /* [retval][out] */ __RPC__deref_out_opt IOemUiccApp **ret);
        
        DECLSPEC_XFGVIRT(IOemRegistrationStatus, get_SystemType)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SystemType )( 
            __RPC__in IOemRegistrationStatus * This,
            /* [retval][out] */ __RPC__out enum RILSYSTEMTYPE *systemType);
        
        DECLSPEC_XFGVIRT(IOemRegistrationStatus, get_RejectReason)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RejectReason )( 
            __RPC__in IOemRegistrationStatus * This,
            /* [retval][out] */ __RPC__out DWORD *reason);
        
        DECLSPEC_XFGVIRT(IOemRegistrationStatus, get_OperatorName)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OperatorName )( 
            __RPC__in IOemRegistrationStatus * This,
            /* [retval][out] */ __RPC__out struct RILOPERATORNAMES *name);
        
        DECLSPEC_XFGVIRT(IOemRegistrationStatus, get_RegistrationStatus)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RegistrationStatus )( 
            __RPC__in IOemRegistrationStatus * This,
            /* [retval][out] */ __RPC__out enum RILREGSTAT *status);
        
        DECLSPEC_XFGVIRT(IOemRegistrationStatus, get_NetworkCode)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NetworkCode )( 
            __RPC__in IOemRegistrationStatus * This,
            /* [retval][out] */ __RPC__out struct RILNETWORKCODE *pNetworkCode);
        
        END_INTERFACE
    } IOemRegistrationStatusVtbl;

    interface IOemRegistrationStatus
    {
        CONST_VTBL struct IOemRegistrationStatusVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOemRegistrationStatus_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOemRegistrationStatus_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOemRegistrationStatus_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOemRegistrationStatus_get_Subscription(This,ret)	\
    ( (This)->lpVtbl -> get_Subscription(This,ret) ) 

#define IOemRegistrationStatus_get_SystemType(This,systemType)	\
    ( (This)->lpVtbl -> get_SystemType(This,systemType) ) 

#define IOemRegistrationStatus_get_RejectReason(This,reason)	\
    ( (This)->lpVtbl -> get_RejectReason(This,reason) ) 

#define IOemRegistrationStatus_get_OperatorName(This,name)	\
    ( (This)->lpVtbl -> get_OperatorName(This,name) ) 

#define IOemRegistrationStatus_get_RegistrationStatus(This,status)	\
    ( (This)->lpVtbl -> get_RegistrationStatus(This,status) ) 

#define IOemRegistrationStatus_get_NetworkCode(This,pNetworkCode)	\
    ( (This)->lpVtbl -> get_NetworkCode(This,pNetworkCode) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOemRegistrationStatus_INTERFACE_DEFINED__ */


#ifndef __IPowerStateChange_INTERFACE_DEFINED__
#define __IPowerStateChange_INTERFACE_DEFINED__

/* interface IPowerStateChange */
/* [uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IPowerStateChange;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0e7d938e-d7fb-49a4-8cac-4db70d952c03")
    IPowerStateChange : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnPowerStateChange( 
            /* [in] */ enum MODEMPOWERSTATE state,
            /* [in] */ INT_PTR context) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPowerStateChangeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPowerStateChange * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPowerStateChange * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPowerStateChange * This);
        
        DECLSPEC_XFGVIRT(IPowerStateChange, OnPowerStateChange)
        HRESULT ( STDMETHODCALLTYPE *OnPowerStateChange )( 
            __RPC__in IPowerStateChange * This,
            /* [in] */ enum MODEMPOWERSTATE state,
            /* [in] */ INT_PTR context);
        
        END_INTERFACE
    } IPowerStateChangeVtbl;

    interface IPowerStateChange
    {
        CONST_VTBL struct IPowerStateChangeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPowerStateChange_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPowerStateChange_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPowerStateChange_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPowerStateChange_OnPowerStateChange(This,state,context)	\
    ( (This)->lpVtbl -> OnPowerStateChange(This,state,context) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPowerStateChange_INTERFACE_DEFINED__ */


#ifndef __IModemOpaqueCommandCompletion_INTERFACE_DEFINED__
#define __IModemOpaqueCommandCompletion_INTERFACE_DEFINED__

/* interface IModemOpaqueCommandCompletion */
/* [uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IModemOpaqueCommandCompletion;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C9B4415B-0643-4855-BB04-A5354F038E62")
    IModemOpaqueCommandCompletion : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE OnModemOpaqueCommandCompletion( 
            /* [in] */ HRESULT result,
            /* [size_is][in] */ __RPC__in_ecount_full(cbSize) BYTE *pOpaqueResponse,
            /* [in] */ DWORD cbSize,
            /* [in] */ INT_PTR context) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IModemOpaqueCommandCompletionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IModemOpaqueCommandCompletion * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IModemOpaqueCommandCompletion * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IModemOpaqueCommandCompletion * This);
        
        DECLSPEC_XFGVIRT(IModemOpaqueCommandCompletion, OnModemOpaqueCommandCompletion)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *OnModemOpaqueCommandCompletion )( 
            __RPC__in IModemOpaqueCommandCompletion * This,
            /* [in] */ HRESULT result,
            /* [size_is][in] */ __RPC__in_ecount_full(cbSize) BYTE *pOpaqueResponse,
            /* [in] */ DWORD cbSize,
            /* [in] */ INT_PTR context);
        
        END_INTERFACE
    } IModemOpaqueCommandCompletionVtbl;

    interface IModemOpaqueCommandCompletion
    {
        CONST_VTBL struct IModemOpaqueCommandCompletionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IModemOpaqueCommandCompletion_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IModemOpaqueCommandCompletion_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IModemOpaqueCommandCompletion_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IModemOpaqueCommandCompletion_OnModemOpaqueCommandCompletion(This,result,pOpaqueResponse,cbSize,context)	\
    ( (This)->lpVtbl -> OnModemOpaqueCommandCompletion(This,result,pOpaqueResponse,cbSize,context) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IModemOpaqueCommandCompletion_INTERFACE_DEFINED__ */


#ifndef __IOpaqueModemNotifications_INTERFACE_DEFINED__
#define __IOpaqueModemNotifications_INTERFACE_DEFINED__

/* interface IOpaqueModemNotifications */
/* [uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IOpaqueModemNotifications;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("85A26F4E-CD62-4B6E-90A1-0D5E2D6EE3FF")
    IOpaqueModemNotifications : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE OnOpaqueModemNotifications( 
            /* [in] */ DWORD dwCode,
            /* [in] */ __RPC__in BYTE *pOpaqueNotification,
            /* [in] */ DWORD cbSize) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOpaqueModemNotificationsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOpaqueModemNotifications * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOpaqueModemNotifications * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOpaqueModemNotifications * This);
        
        DECLSPEC_XFGVIRT(IOpaqueModemNotifications, OnOpaqueModemNotifications)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *OnOpaqueModemNotifications )( 
            __RPC__in IOpaqueModemNotifications * This,
            /* [in] */ DWORD dwCode,
            /* [in] */ __RPC__in BYTE *pOpaqueNotification,
            /* [in] */ DWORD cbSize);
        
        END_INTERFACE
    } IOpaqueModemNotificationsVtbl;

    interface IOpaqueModemNotifications
    {
        CONST_VTBL struct IOpaqueModemNotificationsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOpaqueModemNotifications_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOpaqueModemNotifications_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOpaqueModemNotifications_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOpaqueModemNotifications_OnOpaqueModemNotifications(This,dwCode,pOpaqueNotification,cbSize)	\
    ( (This)->lpVtbl -> OnOpaqueModemNotifications(This,dwCode,pOpaqueNotification,cbSize) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOpaqueModemNotifications_INTERFACE_DEFINED__ */


#ifndef __ISetRFStateCompletion_INTERFACE_DEFINED__
#define __ISetRFStateCompletion_INTERFACE_DEFINED__

/* interface ISetRFStateCompletion */
/* [uuid][oleautomation][object] */ 


EXTERN_C const IID IID_ISetRFStateCompletion;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("340B4A01-07CE-4334-B367-05C87085CBB0")
    ISetRFStateCompletion : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnSetRFStateCompletion( 
            /* [in] */ HRESULT result,
            /* [in] */ INT_PTR context) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISetRFStateCompletionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISetRFStateCompletion * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISetRFStateCompletion * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISetRFStateCompletion * This);
        
        DECLSPEC_XFGVIRT(ISetRFStateCompletion, OnSetRFStateCompletion)
        HRESULT ( STDMETHODCALLTYPE *OnSetRFStateCompletion )( 
            __RPC__in ISetRFStateCompletion * This,
            /* [in] */ HRESULT result,
            /* [in] */ INT_PTR context);
        
        END_INTERFACE
    } ISetRFStateCompletionVtbl;

    interface ISetRFStateCompletion
    {
        CONST_VTBL struct ISetRFStateCompletionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISetRFStateCompletion_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISetRFStateCompletion_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISetRFStateCompletion_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISetRFStateCompletion_OnSetRFStateCompletion(This,result,context)	\
    ( (This)->lpVtbl -> OnSetRFStateCompletion(This,result,context) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISetRFStateCompletion_INTERFACE_DEFINED__ */


#ifndef __IGetRFStateCompletion_INTERFACE_DEFINED__
#define __IGetRFStateCompletion_INTERFACE_DEFINED__

/* interface IGetRFStateCompletion */
/* [uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IGetRFStateCompletion;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4255DD18-D829-4C04-849A-624727B990A2")
    IGetRFStateCompletion : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnGetRFStateCompletion( 
            /* [in] */ HRESULT result,
            /* [in] */ __RPC__in DWORD *pRFState,
            /* [in] */ INT_PTR context) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGetRFStateCompletionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGetRFStateCompletion * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGetRFStateCompletion * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGetRFStateCompletion * This);
        
        DECLSPEC_XFGVIRT(IGetRFStateCompletion, OnGetRFStateCompletion)
        HRESULT ( STDMETHODCALLTYPE *OnGetRFStateCompletion )( 
            __RPC__in IGetRFStateCompletion * This,
            /* [in] */ HRESULT result,
            /* [in] */ __RPC__in DWORD *pRFState,
            /* [in] */ INT_PTR context);
        
        END_INTERFACE
    } IGetRFStateCompletionVtbl;

    interface IGetRFStateCompletion
    {
        CONST_VTBL struct IGetRFStateCompletionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGetRFStateCompletion_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGetRFStateCompletion_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGetRFStateCompletion_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGetRFStateCompletion_OnGetRFStateCompletion(This,result,pRFState,context)	\
    ( (This)->lpVtbl -> OnGetRFStateCompletion(This,result,pRFState,context) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGetRFStateCompletion_INTERFACE_DEFINED__ */


#ifndef __IGetRFStateExCompletion_INTERFACE_DEFINED__
#define __IGetRFStateExCompletion_INTERFACE_DEFINED__

/* interface IGetRFStateExCompletion */
/* [uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IGetRFStateExCompletion;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("74e511fd-f17b-48fc-bd56-93af42e71031")
    IGetRFStateExCompletion : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnGetRFStateCompletion( 
            /* [in] */ HRESULT result,
            /* [in] */ __RPC__in LPRILRFSTATE lpRFState,
            /* [in] */ INT_PTR context) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGetRFStateExCompletionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGetRFStateExCompletion * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGetRFStateExCompletion * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGetRFStateExCompletion * This);
        
        DECLSPEC_XFGVIRT(IGetRFStateExCompletion, OnGetRFStateCompletion)
        HRESULT ( STDMETHODCALLTYPE *OnGetRFStateCompletion )( 
            __RPC__in IGetRFStateExCompletion * This,
            /* [in] */ HRESULT result,
            /* [in] */ __RPC__in LPRILRFSTATE lpRFState,
            /* [in] */ INT_PTR context);
        
        END_INTERFACE
    } IGetRFStateExCompletionVtbl;

    interface IGetRFStateExCompletion
    {
        CONST_VTBL struct IGetRFStateExCompletionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGetRFStateExCompletion_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGetRFStateExCompletion_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGetRFStateExCompletion_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGetRFStateExCompletion_OnGetRFStateCompletion(This,result,lpRFState,context)	\
    ( (This)->lpVtbl -> OnGetRFStateCompletion(This,result,lpRFState,context) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGetRFStateExCompletion_INTERFACE_DEFINED__ */


#ifndef __ICanInfoCompletion_INTERFACE_DEFINED__
#define __ICanInfoCompletion_INTERFACE_DEFINED__

/* interface ICanInfoCompletion */
/* [uuid][oleautomation][object] */ 


EXTERN_C const IID IID_ICanInfoCompletion;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b5bb6a5e-8d3f-4203-b58b-26c3d4e8eedc")
    ICanInfoCompletion : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE OnGetInfoCompletion( 
            /* [in] */ HRESULT result,
            /* [in] */ __RPC__in const WCHAR *value,
            /* [in] */ INT_PTR context) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICanInfoCompletionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICanInfoCompletion * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICanInfoCompletion * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICanInfoCompletion * This);
        
        DECLSPEC_XFGVIRT(ICanInfoCompletion, OnGetInfoCompletion)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *OnGetInfoCompletion )( 
            __RPC__in ICanInfoCompletion * This,
            /* [in] */ HRESULT result,
            /* [in] */ __RPC__in const WCHAR *value,
            /* [in] */ INT_PTR context);
        
        END_INTERFACE
    } ICanInfoCompletionVtbl;

    interface ICanInfoCompletion
    {
        CONST_VTBL struct ICanInfoCompletionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICanInfoCompletion_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICanInfoCompletion_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICanInfoCompletion_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICanInfoCompletion_OnGetInfoCompletion(This,result,value,context)	\
    ( (This)->lpVtbl -> OnGetInfoCompletion(This,result,value,context) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICanInfoCompletion_INTERFACE_DEFINED__ */


#ifndef __IPositionInfoCompletion_INTERFACE_DEFINED__
#define __IPositionInfoCompletion_INTERFACE_DEFINED__

/* interface IPositionInfoCompletion */
/* [uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IPositionInfoCompletion;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("03569923-a28f-47bd-9315-38d8fb11717d")
    IPositionInfoCompletion : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE OnGetPositionInfoCompletion( 
            /* [in] */ HRESULT result,
            /* [in] */ __RPC__in LPRILPOSITIONINFO lpPositionInfo,
            /* [in] */ INT_PTR context) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPositionInfoCompletionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPositionInfoCompletion * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPositionInfoCompletion * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPositionInfoCompletion * This);
        
        DECLSPEC_XFGVIRT(IPositionInfoCompletion, OnGetPositionInfoCompletion)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *OnGetPositionInfoCompletion )( 
            __RPC__in IPositionInfoCompletion * This,
            /* [in] */ HRESULT result,
            /* [in] */ __RPC__in LPRILPOSITIONINFO lpPositionInfo,
            /* [in] */ INT_PTR context);
        
        END_INTERFACE
    } IPositionInfoCompletionVtbl;

    interface IPositionInfoCompletion
    {
        CONST_VTBL struct IPositionInfoCompletionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPositionInfoCompletion_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPositionInfoCompletion_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPositionInfoCompletion_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPositionInfoCompletion_OnGetPositionInfoCompletion(This,result,lpPositionInfo,context)	\
    ( (This)->lpVtbl -> OnGetPositionInfoCompletion(This,result,lpPositionInfo,context) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPositionInfoCompletion_INTERFACE_DEFINED__ */


#ifndef __IPinLockStateCompletion_INTERFACE_DEFINED__
#define __IPinLockStateCompletion_INTERFACE_DEFINED__

/* interface IPinLockStateCompletion */
/* [uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IPinLockStateCompletion;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3b2481a7-aee9-462f-bd38-31f15c15ff88")
    IPinLockStateCompletion : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE OnGetPinLockStateCompletion( 
            /* [in] */ HRESULT result,
            /* [in] */ __RPC__in LPRILUICCLOCKSTATE pUICCLockState,
            /* [in] */ DWORD length,
            /* [in] */ INT_PTR context) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPinLockStateCompletionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPinLockStateCompletion * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPinLockStateCompletion * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPinLockStateCompletion * This);
        
        DECLSPEC_XFGVIRT(IPinLockStateCompletion, OnGetPinLockStateCompletion)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *OnGetPinLockStateCompletion )( 
            __RPC__in IPinLockStateCompletion * This,
            /* [in] */ HRESULT result,
            /* [in] */ __RPC__in LPRILUICCLOCKSTATE pUICCLockState,
            /* [in] */ DWORD length,
            /* [in] */ INT_PTR context);
        
        END_INTERFACE
    } IPinLockStateCompletionVtbl;

    interface IPinLockStateCompletion
    {
        CONST_VTBL struct IPinLockStateCompletionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPinLockStateCompletion_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPinLockStateCompletion_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPinLockStateCompletion_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPinLockStateCompletion_OnGetPinLockStateCompletion(This,result,pUICCLockState,length,context)	\
    ( (This)->lpVtbl -> OnGetPinLockStateCompletion(This,result,pUICCLockState,length,context) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPinLockStateCompletion_INTERFACE_DEFINED__ */


#ifndef __IGetRecordStatusCompletion_INTERFACE_DEFINED__
#define __IGetRecordStatusCompletion_INTERFACE_DEFINED__

/* interface IGetRecordStatusCompletion */
/* [uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IGetRecordStatusCompletion;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2d033fa3-ae5c-495c-bcf5-459ffa07036a")
    IGetRecordStatusCompletion : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE OnGetRecordStatusCompletion( 
            /* [in] */ HRESULT result,
            /* [in] */ __RPC__in LPRILUICCRECORDSTATUS recordStatus,
            /* [in] */ DWORD length,
            /* [in] */ INT_PTR context) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGetRecordStatusCompletionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGetRecordStatusCompletion * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGetRecordStatusCompletion * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGetRecordStatusCompletion * This);
        
        DECLSPEC_XFGVIRT(IGetRecordStatusCompletion, OnGetRecordStatusCompletion)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *OnGetRecordStatusCompletion )( 
            __RPC__in IGetRecordStatusCompletion * This,
            /* [in] */ HRESULT result,
            /* [in] */ __RPC__in LPRILUICCRECORDSTATUS recordStatus,
            /* [in] */ DWORD length,
            /* [in] */ INT_PTR context);
        
        END_INTERFACE
    } IGetRecordStatusCompletionVtbl;

    interface IGetRecordStatusCompletion
    {
        CONST_VTBL struct IGetRecordStatusCompletionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGetRecordStatusCompletion_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGetRecordStatusCompletion_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGetRecordStatusCompletion_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGetRecordStatusCompletion_OnGetRecordStatusCompletion(This,result,recordStatus,length,context)	\
    ( (This)->lpVtbl -> OnGetRecordStatusCompletion(This,result,recordStatus,length,context) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGetRecordStatusCompletion_INTERFACE_DEFINED__ */


#ifndef __IReadRecordCompletion_INTERFACE_DEFINED__
#define __IReadRecordCompletion_INTERFACE_DEFINED__

/* interface IReadRecordCompletion */
/* [uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IReadRecordCompletion;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5a8c7c5f-aace-47b8-b38b-41785a9fc090")
    IReadRecordCompletion : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE OnReadRecordCompletion( 
            /* [in] */ HRESULT result,
            /* [in] */ __RPC__in BYTE *record,
            /* [in] */ DWORD cbSize,
            /* [in] */ INT_PTR context) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IReadRecordCompletionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IReadRecordCompletion * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IReadRecordCompletion * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IReadRecordCompletion * This);
        
        DECLSPEC_XFGVIRT(IReadRecordCompletion, OnReadRecordCompletion)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *OnReadRecordCompletion )( 
            __RPC__in IReadRecordCompletion * This,
            /* [in] */ HRESULT result,
            /* [in] */ __RPC__in BYTE *record,
            /* [in] */ DWORD cbSize,
            /* [in] */ INT_PTR context);
        
        END_INTERFACE
    } IReadRecordCompletionVtbl;

    interface IReadRecordCompletion
    {
        CONST_VTBL struct IReadRecordCompletionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IReadRecordCompletion_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IReadRecordCompletion_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IReadRecordCompletion_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IReadRecordCompletion_OnReadRecordCompletion(This,result,record,cbSize,context)	\
    ( (This)->lpVtbl -> OnReadRecordCompletion(This,result,record,cbSize,context) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IReadRecordCompletion_INTERFACE_DEFINED__ */


#ifndef __IWriteRecordCompletion_INTERFACE_DEFINED__
#define __IWriteRecordCompletion_INTERFACE_DEFINED__

/* interface IWriteRecordCompletion */
/* [uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IWriteRecordCompletion;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f9b2b7de-3ab4-4af8-bac0-fd9a83113242")
    IWriteRecordCompletion : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE OnWriteRecordCompletion( 
            /* [in] */ HRESULT result,
            /* [in] */ INT_PTR context) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWriteRecordCompletionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWriteRecordCompletion * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWriteRecordCompletion * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWriteRecordCompletion * This);
        
        DECLSPEC_XFGVIRT(IWriteRecordCompletion, OnWriteRecordCompletion)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *OnWriteRecordCompletion )( 
            __RPC__in IWriteRecordCompletion * This,
            /* [in] */ HRESULT result,
            /* [in] */ INT_PTR context);
        
        END_INTERFACE
    } IWriteRecordCompletionVtbl;

    interface IWriteRecordCompletion
    {
        CONST_VTBL struct IWriteRecordCompletionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWriteRecordCompletion_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWriteRecordCompletion_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWriteRecordCompletion_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWriteRecordCompletion_OnWriteRecordCompletion(This,result,context)	\
    ( (This)->lpVtbl -> OnWriteRecordCompletion(This,result,context) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWriteRecordCompletion_INTERFACE_DEFINED__ */


#ifndef __IIMSICompletion_INTERFACE_DEFINED__
#define __IIMSICompletion_INTERFACE_DEFINED__

/* interface IIMSICompletion */
/* [uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IIMSICompletion;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A9342324-DA50-46B2-BA1C-2B1CB60C3817")
    IIMSICompletion : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE OnGetIMSICompletion( 
            /* [in] */ HRESULT result,
            /* [in] */ __RPC__in const LPRILIMSI rilImsi,
            /* [in] */ INT_PTR context) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IIMSICompletionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IIMSICompletion * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IIMSICompletion * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IIMSICompletion * This);
        
        DECLSPEC_XFGVIRT(IIMSICompletion, OnGetIMSICompletion)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *OnGetIMSICompletion )( 
            __RPC__in IIMSICompletion * This,
            /* [in] */ HRESULT result,
            /* [in] */ __RPC__in const LPRILIMSI rilImsi,
            /* [in] */ INT_PTR context);
        
        END_INTERFACE
    } IIMSICompletionVtbl;

    interface IIMSICompletion
    {
        CONST_VTBL struct IIMSICompletionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IIMSICompletion_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IIMSICompletion_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IIMSICompletion_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IIMSICompletion_OnGetIMSICompletion(This,result,rilImsi,context)	\
    ( (This)->lpVtbl -> OnGetIMSICompletion(This,result,rilImsi,context) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IIMSICompletion_INTERFACE_DEFINED__ */


#ifndef __IGetSIDNIDCompletion_INTERFACE_DEFINED__
#define __IGetSIDNIDCompletion_INTERFACE_DEFINED__

/* interface IGetSIDNIDCompletion */
/* [uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IGetSIDNIDCompletion;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("e7d5d35e-a770-4adc-8bb1-a677ea6e89ac")
    IGetSIDNIDCompletion : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE OnGetSIDNIDCompletion( 
            /* [in] */ HRESULT result,
            /* [in] */ __RPC__in const LPRILSIDNID rilSidNid,
            /* [in] */ INT_PTR context) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGetSIDNIDCompletionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGetSIDNIDCompletion * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGetSIDNIDCompletion * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGetSIDNIDCompletion * This);
        
        DECLSPEC_XFGVIRT(IGetSIDNIDCompletion, OnGetSIDNIDCompletion)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *OnGetSIDNIDCompletion )( 
            __RPC__in IGetSIDNIDCompletion * This,
            /* [in] */ HRESULT result,
            /* [in] */ __RPC__in const LPRILSIDNID rilSidNid,
            /* [in] */ INT_PTR context);
        
        END_INTERFACE
    } IGetSIDNIDCompletionVtbl;

    interface IGetSIDNIDCompletion
    {
        CONST_VTBL struct IGetSIDNIDCompletionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGetSIDNIDCompletion_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGetSIDNIDCompletion_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGetSIDNIDCompletion_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGetSIDNIDCompletion_OnGetSIDNIDCompletion(This,result,rilSidNid,context)	\
    ( (This)->lpVtbl -> OnGetSIDNIDCompletion(This,result,rilSidNid,context) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGetSIDNIDCompletion_INTERFACE_DEFINED__ */


#ifndef __IGetNAICompletion_INTERFACE_DEFINED__
#define __IGetNAICompletion_INTERFACE_DEFINED__

/* interface IGetNAICompletion */
/* [uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IGetNAICompletion;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4AFE7F70-7D1F-45D1-BC93-908161FEBC58")
    IGetNAICompletion : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE OnGetNAICompletion( 
            /* [in] */ HRESULT result,
            /* [in] */ __RPC__in const LPRILNAI rilNai,
            /* [in] */ INT_PTR context) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGetNAICompletionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGetNAICompletion * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGetNAICompletion * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGetNAICompletion * This);
        
        DECLSPEC_XFGVIRT(IGetNAICompletion, OnGetNAICompletion)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *OnGetNAICompletion )( 
            __RPC__in IGetNAICompletion * This,
            /* [in] */ HRESULT result,
            /* [in] */ __RPC__in const LPRILNAI rilNai,
            /* [in] */ INT_PTR context);
        
        END_INTERFACE
    } IGetNAICompletionVtbl;

    interface IGetNAICompletion
    {
        CONST_VTBL struct IGetNAICompletionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGetNAICompletion_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGetNAICompletion_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGetNAICompletion_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGetNAICompletion_OnGetNAICompletion(This,result,rilNai,context)	\
    ( (This)->lpVtbl -> OnGetNAICompletion(This,result,rilNai,context) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGetNAICompletion_INTERFACE_DEFINED__ */


#ifndef __ISubscriberNumbersCompletion_INTERFACE_DEFINED__
#define __ISubscriberNumbersCompletion_INTERFACE_DEFINED__

/* interface ISubscriberNumbersCompletion */
/* [uuid][oleautomation][object] */ 


EXTERN_C const IID IID_ISubscriberNumbersCompletion;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("be6d7c2a-ba04-4fcc-a611-d33000643d4f")
    ISubscriberNumbersCompletion : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE OnGetSubscriberNumbersCompletion( 
            /* [in] */ HRESULT result,
            /* [in] */ __RPC__in LPRILUICCSUBSCRIBERNUMBERS pSubscriberNumbers,
            /* [in] */ DWORD length,
            /* [in] */ INT_PTR context) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISubscriberNumbersCompletionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISubscriberNumbersCompletion * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISubscriberNumbersCompletion * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISubscriberNumbersCompletion * This);
        
        DECLSPEC_XFGVIRT(ISubscriberNumbersCompletion, OnGetSubscriberNumbersCompletion)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *OnGetSubscriberNumbersCompletion )( 
            __RPC__in ISubscriberNumbersCompletion * This,
            /* [in] */ HRESULT result,
            /* [in] */ __RPC__in LPRILUICCSUBSCRIBERNUMBERS pSubscriberNumbers,
            /* [in] */ DWORD length,
            /* [in] */ INT_PTR context);
        
        END_INTERFACE
    } ISubscriberNumbersCompletionVtbl;

    interface ISubscriberNumbersCompletion
    {
        CONST_VTBL struct ISubscriberNumbersCompletionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISubscriberNumbersCompletion_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISubscriberNumbersCompletion_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISubscriberNumbersCompletion_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISubscriberNumbersCompletion_OnGetSubscriberNumbersCompletion(This,result,pSubscriberNumbers,length,context)	\
    ( (This)->lpVtbl -> OnGetSubscriberNumbersCompletion(This,result,pSubscriberNumbers,length,context) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISubscriberNumbersCompletion_INTERFACE_DEFINED__ */


#ifndef __IGetPLMNwAcT_INTERFACE_DEFINED__
#define __IGetPLMNwAcT_INTERFACE_DEFINED__

/* interface IGetPLMNwAcT */
/* [uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IGetPLMNwAcT;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("E9BEB716-6ADD-492B-BBEA-B2FE1068A86E")
    IGetPLMNwAcT : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE OnGetPreferredOperatorListCompletion( 
            /* [in] */ HRESULT result,
            /* [size_is][in] */ __RPC__in_ecount_full(dwLength) RILOPERATORNAMES *pOperatorList,
            /* [in] */ DWORD dwLength) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGetPLMNwAcTVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGetPLMNwAcT * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGetPLMNwAcT * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGetPLMNwAcT * This);
        
        DECLSPEC_XFGVIRT(IGetPLMNwAcT, OnGetPreferredOperatorListCompletion)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *OnGetPreferredOperatorListCompletion )( 
            __RPC__in IGetPLMNwAcT * This,
            /* [in] */ HRESULT result,
            /* [size_is][in] */ __RPC__in_ecount_full(dwLength) RILOPERATORNAMES *pOperatorList,
            /* [in] */ DWORD dwLength);
        
        END_INTERFACE
    } IGetPLMNwAcTVtbl;

    interface IGetPLMNwAcT
    {
        CONST_VTBL struct IGetPLMNwAcTVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGetPLMNwAcT_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGetPLMNwAcT_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGetPLMNwAcT_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGetPLMNwAcT_OnGetPreferredOperatorListCompletion(This,result,pOperatorList,dwLength)	\
    ( (This)->lpVtbl -> OnGetPreferredOperatorListCompletion(This,result,pOperatorList,dwLength) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGetPLMNwAcT_INTERFACE_DEFINED__ */


#ifndef __ISetPLMNwAcT_INTERFACE_DEFINED__
#define __ISetPLMNwAcT_INTERFACE_DEFINED__

/* interface ISetPLMNwAcT */
/* [uuid][oleautomation][object] */ 


EXTERN_C const IID IID_ISetPLMNwAcT;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4E331477-F454-4DE7-8B18-5130B891B012")
    ISetPLMNwAcT : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE OnSetPreferredOperatorListCompletion( 
            /* [in] */ HRESULT result) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISetPLMNwAcTVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISetPLMNwAcT * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISetPLMNwAcT * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISetPLMNwAcT * This);
        
        DECLSPEC_XFGVIRT(ISetPLMNwAcT, OnSetPreferredOperatorListCompletion)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *OnSetPreferredOperatorListCompletion )( 
            __RPC__in ISetPLMNwAcT * This,
            /* [in] */ HRESULT result);
        
        END_INTERFACE
    } ISetPLMNwAcTVtbl;

    interface ISetPLMNwAcT
    {
        CONST_VTBL struct ISetPLMNwAcTVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISetPLMNwAcT_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISetPLMNwAcT_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISetPLMNwAcT_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISetPLMNwAcT_OnSetPreferredOperatorListCompletion(This,result)	\
    ( (This)->lpVtbl -> OnSetPreferredOperatorListCompletion(This,result) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISetPLMNwAcT_INTERFACE_DEFINED__ */


#ifndef __IGetCallForwardingCompletion_INTERFACE_DEFINED__
#define __IGetCallForwardingCompletion_INTERFACE_DEFINED__

/* interface IGetCallForwardingCompletion */
/* [uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IGetCallForwardingCompletion;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c329148f-3374-438b-8ed4-e1b9ab28eee3")
    IGetCallForwardingCompletion : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnGetCallForwardingSettingsCompletion( 
            /* [in] */ HRESULT result,
            /* [in] */ INT_PTR context,
            /* [in] */ __RPC__in RILCALLFORWARDINGSETTINGS *settings,
            /* [in] */ DWORD cSettings) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGetCallForwardingCompletionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGetCallForwardingCompletion * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGetCallForwardingCompletion * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGetCallForwardingCompletion * This);
        
        DECLSPEC_XFGVIRT(IGetCallForwardingCompletion, OnGetCallForwardingSettingsCompletion)
        HRESULT ( STDMETHODCALLTYPE *OnGetCallForwardingSettingsCompletion )( 
            __RPC__in IGetCallForwardingCompletion * This,
            /* [in] */ HRESULT result,
            /* [in] */ INT_PTR context,
            /* [in] */ __RPC__in RILCALLFORWARDINGSETTINGS *settings,
            /* [in] */ DWORD cSettings);
        
        END_INTERFACE
    } IGetCallForwardingCompletionVtbl;

    interface IGetCallForwardingCompletion
    {
        CONST_VTBL struct IGetCallForwardingCompletionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGetCallForwardingCompletion_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGetCallForwardingCompletion_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGetCallForwardingCompletion_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGetCallForwardingCompletion_OnGetCallForwardingSettingsCompletion(This,result,context,settings,cSettings)	\
    ( (This)->lpVtbl -> OnGetCallForwardingSettingsCompletion(This,result,context,settings,cSettings) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGetCallForwardingCompletion_INTERFACE_DEFINED__ */


#ifndef __ISimpleModemCompletion_INTERFACE_DEFINED__
#define __ISimpleModemCompletion_INTERFACE_DEFINED__

/* interface ISimpleModemCompletion */
/* [uuid][oleautomation][object] */ 


EXTERN_C const IID IID_ISimpleModemCompletion;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9A189741-DED1-4535-B116-B6D287BF70D2")
    ISimpleModemCompletion : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE OnFinished( 
            HRESULT result,
            /* [in] */ INT_PTR context) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISimpleModemCompletionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISimpleModemCompletion * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISimpleModemCompletion * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISimpleModemCompletion * This);
        
        DECLSPEC_XFGVIRT(ISimpleModemCompletion, OnFinished)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *OnFinished )( 
            __RPC__in ISimpleModemCompletion * This,
            HRESULT result,
            /* [in] */ INT_PTR context);
        
        END_INTERFACE
    } ISimpleModemCompletionVtbl;

    interface ISimpleModemCompletion
    {
        CONST_VTBL struct ISimpleModemCompletionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISimpleModemCompletion_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISimpleModemCompletion_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISimpleModemCompletion_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISimpleModemCompletion_OnFinished(This,result,context)	\
    ( (This)->lpVtbl -> OnFinished(This,result,context) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISimpleModemCompletion_INTERFACE_DEFINED__ */


#ifndef __ICallWaitingSettingsCompletion_INTERFACE_DEFINED__
#define __ICallWaitingSettingsCompletion_INTERFACE_DEFINED__

/* interface ICallWaitingSettingsCompletion */
/* [uuid][oleautomation][object] */ 


EXTERN_C const IID IID_ICallWaitingSettingsCompletion;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f9581718-2283-4336-9397-1c6c067299d2")
    ICallWaitingSettingsCompletion : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE OnCallWaitingSettingsCompletion( 
            /* [in] */ HRESULT hr,
            /* [in] */ DWORD dwInfoClasses,
            /* [in] */ INT_PTR context) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICallWaitingSettingsCompletionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICallWaitingSettingsCompletion * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICallWaitingSettingsCompletion * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICallWaitingSettingsCompletion * This);
        
        DECLSPEC_XFGVIRT(ICallWaitingSettingsCompletion, OnCallWaitingSettingsCompletion)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *OnCallWaitingSettingsCompletion )( 
            __RPC__in ICallWaitingSettingsCompletion * This,
            /* [in] */ HRESULT hr,
            /* [in] */ DWORD dwInfoClasses,
            /* [in] */ INT_PTR context);
        
        END_INTERFACE
    } ICallWaitingSettingsCompletionVtbl;

    interface ICallWaitingSettingsCompletion
    {
        CONST_VTBL struct ICallWaitingSettingsCompletionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICallWaitingSettingsCompletion_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICallWaitingSettingsCompletion_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICallWaitingSettingsCompletion_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICallWaitingSettingsCompletion_OnCallWaitingSettingsCompletion(This,hr,dwInfoClasses,context)	\
    ( (This)->lpVtbl -> OnCallWaitingSettingsCompletion(This,hr,dwInfoClasses,context) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICallWaitingSettingsCompletion_INTERFACE_DEFINED__ */


#ifndef __IOemIMSStatusChange_INTERFACE_DEFINED__
#define __IOemIMSStatusChange_INTERFACE_DEFINED__

/* interface IOemIMSStatusChange */
/* [uuid][oleautomation][object] */ 


EXTERN_C const IID IID_IOemIMSStatusChange;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7d034a18-baef-4f86-ad84-bb0bbfb9f834")
    IOemIMSStatusChange : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE OnOemIMSStatusChanged( 
            /* [in] */ __RPC__in LPRILIMSSTATUS rilIMSStatus,
            /* [in] */ __RPC__in_opt IOemCanExtForIMS *pCan) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOemIMSStatusChangeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IOemIMSStatusChange * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IOemIMSStatusChange * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IOemIMSStatusChange * This);
        
        DECLSPEC_XFGVIRT(IOemIMSStatusChange, OnOemIMSStatusChanged)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *OnOemIMSStatusChanged )( 
            __RPC__in IOemIMSStatusChange * This,
            /* [in] */ __RPC__in LPRILIMSSTATUS rilIMSStatus,
            /* [in] */ __RPC__in_opt IOemCanExtForIMS *pCan);
        
        END_INTERFACE
    } IOemIMSStatusChangeVtbl;

    interface IOemIMSStatusChange
    {
        CONST_VTBL struct IOemIMSStatusChangeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOemIMSStatusChange_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOemIMSStatusChange_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOemIMSStatusChange_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOemIMSStatusChange_OnOemIMSStatusChanged(This,rilIMSStatus,pCan)	\
    ( (This)->lpVtbl -> OnOemIMSStatusChanged(This,rilIMSStatus,pCan) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOemIMSStatusChange_INTERFACE_DEFINED__ */



#ifndef __OemCellularIF_LIBRARY_DEFINED__
#define __OemCellularIF_LIBRARY_DEFINED__

/* library OemCellularIF */
/* [version][uuid] */ 









































EXTERN_C const IID LIBID_OemCellularIF;

EXTERN_C const CLSID CLSID_OemCellular;

#ifdef __cplusplus

class DECLSPEC_UUID("9D27B916-4F17-4EE8-A71C-D84222993D64")
OemCellular;
#endif
#endif /* __OemCellularIF_LIBRARY_DEFINED__ */

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif



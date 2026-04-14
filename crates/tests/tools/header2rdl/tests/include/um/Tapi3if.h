

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

#ifndef __tapi3if_h__
#define __tapi3if_h__

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

#ifndef __ITTAPI_FWD_DEFINED__
#define __ITTAPI_FWD_DEFINED__
typedef interface ITTAPI ITTAPI;

#endif 	/* __ITTAPI_FWD_DEFINED__ */


#ifndef __ITTAPI2_FWD_DEFINED__
#define __ITTAPI2_FWD_DEFINED__
typedef interface ITTAPI2 ITTAPI2;

#endif 	/* __ITTAPI2_FWD_DEFINED__ */


#ifndef __ITMediaSupport_FWD_DEFINED__
#define __ITMediaSupport_FWD_DEFINED__
typedef interface ITMediaSupport ITMediaSupport;

#endif 	/* __ITMediaSupport_FWD_DEFINED__ */


#ifndef __ITPluggableTerminalClassInfo_FWD_DEFINED__
#define __ITPluggableTerminalClassInfo_FWD_DEFINED__
typedef interface ITPluggableTerminalClassInfo ITPluggableTerminalClassInfo;

#endif 	/* __ITPluggableTerminalClassInfo_FWD_DEFINED__ */


#ifndef __ITPluggableTerminalSuperclassInfo_FWD_DEFINED__
#define __ITPluggableTerminalSuperclassInfo_FWD_DEFINED__
typedef interface ITPluggableTerminalSuperclassInfo ITPluggableTerminalSuperclassInfo;

#endif 	/* __ITPluggableTerminalSuperclassInfo_FWD_DEFINED__ */


#ifndef __ITTerminalSupport_FWD_DEFINED__
#define __ITTerminalSupport_FWD_DEFINED__
typedef interface ITTerminalSupport ITTerminalSupport;

#endif 	/* __ITTerminalSupport_FWD_DEFINED__ */


#ifndef __ITTerminalSupport2_FWD_DEFINED__
#define __ITTerminalSupport2_FWD_DEFINED__
typedef interface ITTerminalSupport2 ITTerminalSupport2;

#endif 	/* __ITTerminalSupport2_FWD_DEFINED__ */


#ifndef __ITAddress_FWD_DEFINED__
#define __ITAddress_FWD_DEFINED__
typedef interface ITAddress ITAddress;

#endif 	/* __ITAddress_FWD_DEFINED__ */


#ifndef __ITAddress2_FWD_DEFINED__
#define __ITAddress2_FWD_DEFINED__
typedef interface ITAddress2 ITAddress2;

#endif 	/* __ITAddress2_FWD_DEFINED__ */


#ifndef __ITAddressCapabilities_FWD_DEFINED__
#define __ITAddressCapabilities_FWD_DEFINED__
typedef interface ITAddressCapabilities ITAddressCapabilities;

#endif 	/* __ITAddressCapabilities_FWD_DEFINED__ */


#ifndef __ITPhone_FWD_DEFINED__
#define __ITPhone_FWD_DEFINED__
typedef interface ITPhone ITPhone;

#endif 	/* __ITPhone_FWD_DEFINED__ */


#ifndef __ITAutomatedPhoneControl_FWD_DEFINED__
#define __ITAutomatedPhoneControl_FWD_DEFINED__
typedef interface ITAutomatedPhoneControl ITAutomatedPhoneControl;

#endif 	/* __ITAutomatedPhoneControl_FWD_DEFINED__ */


#ifndef __ITBasicCallControl_FWD_DEFINED__
#define __ITBasicCallControl_FWD_DEFINED__
typedef interface ITBasicCallControl ITBasicCallControl;

#endif 	/* __ITBasicCallControl_FWD_DEFINED__ */


#ifndef __ITCallInfo_FWD_DEFINED__
#define __ITCallInfo_FWD_DEFINED__
typedef interface ITCallInfo ITCallInfo;

#endif 	/* __ITCallInfo_FWD_DEFINED__ */


#ifndef __ITCallInfo2_FWD_DEFINED__
#define __ITCallInfo2_FWD_DEFINED__
typedef interface ITCallInfo2 ITCallInfo2;

#endif 	/* __ITCallInfo2_FWD_DEFINED__ */


#ifndef __ITTerminal_FWD_DEFINED__
#define __ITTerminal_FWD_DEFINED__
typedef interface ITTerminal ITTerminal;

#endif 	/* __ITTerminal_FWD_DEFINED__ */


#ifndef __ITMultiTrackTerminal_FWD_DEFINED__
#define __ITMultiTrackTerminal_FWD_DEFINED__
typedef interface ITMultiTrackTerminal ITMultiTrackTerminal;

#endif 	/* __ITMultiTrackTerminal_FWD_DEFINED__ */


#ifndef __ITFileTrack_FWD_DEFINED__
#define __ITFileTrack_FWD_DEFINED__
typedef interface ITFileTrack ITFileTrack;

#endif 	/* __ITFileTrack_FWD_DEFINED__ */


#ifndef __ITMediaPlayback_FWD_DEFINED__
#define __ITMediaPlayback_FWD_DEFINED__
typedef interface ITMediaPlayback ITMediaPlayback;

#endif 	/* __ITMediaPlayback_FWD_DEFINED__ */


#ifndef __ITMediaRecord_FWD_DEFINED__
#define __ITMediaRecord_FWD_DEFINED__
typedef interface ITMediaRecord ITMediaRecord;

#endif 	/* __ITMediaRecord_FWD_DEFINED__ */


#ifndef __ITMediaControl_FWD_DEFINED__
#define __ITMediaControl_FWD_DEFINED__
typedef interface ITMediaControl ITMediaControl;

#endif 	/* __ITMediaControl_FWD_DEFINED__ */


#ifndef __ITBasicAudioTerminal_FWD_DEFINED__
#define __ITBasicAudioTerminal_FWD_DEFINED__
typedef interface ITBasicAudioTerminal ITBasicAudioTerminal;

#endif 	/* __ITBasicAudioTerminal_FWD_DEFINED__ */


#ifndef __ITStaticAudioTerminal_FWD_DEFINED__
#define __ITStaticAudioTerminal_FWD_DEFINED__
typedef interface ITStaticAudioTerminal ITStaticAudioTerminal;

#endif 	/* __ITStaticAudioTerminal_FWD_DEFINED__ */


#ifndef __ITCallHub_FWD_DEFINED__
#define __ITCallHub_FWD_DEFINED__
typedef interface ITCallHub ITCallHub;

#endif 	/* __ITCallHub_FWD_DEFINED__ */


#ifndef __ITLegacyAddressMediaControl_FWD_DEFINED__
#define __ITLegacyAddressMediaControl_FWD_DEFINED__
typedef interface ITLegacyAddressMediaControl ITLegacyAddressMediaControl;

#endif 	/* __ITLegacyAddressMediaControl_FWD_DEFINED__ */


#ifndef __ITPrivateEvent_FWD_DEFINED__
#define __ITPrivateEvent_FWD_DEFINED__
typedef interface ITPrivateEvent ITPrivateEvent;

#endif 	/* __ITPrivateEvent_FWD_DEFINED__ */


#ifndef __ITLegacyAddressMediaControl2_FWD_DEFINED__
#define __ITLegacyAddressMediaControl2_FWD_DEFINED__
typedef interface ITLegacyAddressMediaControl2 ITLegacyAddressMediaControl2;

#endif 	/* __ITLegacyAddressMediaControl2_FWD_DEFINED__ */


#ifndef __ITLegacyCallMediaControl_FWD_DEFINED__
#define __ITLegacyCallMediaControl_FWD_DEFINED__
typedef interface ITLegacyCallMediaControl ITLegacyCallMediaControl;

#endif 	/* __ITLegacyCallMediaControl_FWD_DEFINED__ */


#ifndef __ITLegacyCallMediaControl2_FWD_DEFINED__
#define __ITLegacyCallMediaControl2_FWD_DEFINED__
typedef interface ITLegacyCallMediaControl2 ITLegacyCallMediaControl2;

#endif 	/* __ITLegacyCallMediaControl2_FWD_DEFINED__ */


#ifndef __ITDetectTone_FWD_DEFINED__
#define __ITDetectTone_FWD_DEFINED__
typedef interface ITDetectTone ITDetectTone;

#endif 	/* __ITDetectTone_FWD_DEFINED__ */


#ifndef __ITCustomTone_FWD_DEFINED__
#define __ITCustomTone_FWD_DEFINED__
typedef interface ITCustomTone ITCustomTone;

#endif 	/* __ITCustomTone_FWD_DEFINED__ */


#ifndef __IEnumPhone_FWD_DEFINED__
#define __IEnumPhone_FWD_DEFINED__
typedef interface IEnumPhone IEnumPhone;

#endif 	/* __IEnumPhone_FWD_DEFINED__ */


#ifndef __IEnumTerminal_FWD_DEFINED__
#define __IEnumTerminal_FWD_DEFINED__
typedef interface IEnumTerminal IEnumTerminal;

#endif 	/* __IEnumTerminal_FWD_DEFINED__ */


#ifndef __IEnumTerminalClass_FWD_DEFINED__
#define __IEnumTerminalClass_FWD_DEFINED__
typedef interface IEnumTerminalClass IEnumTerminalClass;

#endif 	/* __IEnumTerminalClass_FWD_DEFINED__ */


#ifndef __IEnumCall_FWD_DEFINED__
#define __IEnumCall_FWD_DEFINED__
typedef interface IEnumCall IEnumCall;

#endif 	/* __IEnumCall_FWD_DEFINED__ */


#ifndef __IEnumAddress_FWD_DEFINED__
#define __IEnumAddress_FWD_DEFINED__
typedef interface IEnumAddress IEnumAddress;

#endif 	/* __IEnumAddress_FWD_DEFINED__ */


#ifndef __IEnumCallHub_FWD_DEFINED__
#define __IEnumCallHub_FWD_DEFINED__
typedef interface IEnumCallHub IEnumCallHub;

#endif 	/* __IEnumCallHub_FWD_DEFINED__ */


#ifndef __IEnumBstr_FWD_DEFINED__
#define __IEnumBstr_FWD_DEFINED__
typedef interface IEnumBstr IEnumBstr;

#endif 	/* __IEnumBstr_FWD_DEFINED__ */


#ifndef __IEnumPluggableTerminalClassInfo_FWD_DEFINED__
#define __IEnumPluggableTerminalClassInfo_FWD_DEFINED__
typedef interface IEnumPluggableTerminalClassInfo IEnumPluggableTerminalClassInfo;

#endif 	/* __IEnumPluggableTerminalClassInfo_FWD_DEFINED__ */


#ifndef __IEnumPluggableSuperclassInfo_FWD_DEFINED__
#define __IEnumPluggableSuperclassInfo_FWD_DEFINED__
typedef interface IEnumPluggableSuperclassInfo IEnumPluggableSuperclassInfo;

#endif 	/* __IEnumPluggableSuperclassInfo_FWD_DEFINED__ */


#ifndef __ITPhoneEvent_FWD_DEFINED__
#define __ITPhoneEvent_FWD_DEFINED__
typedef interface ITPhoneEvent ITPhoneEvent;

#endif 	/* __ITPhoneEvent_FWD_DEFINED__ */


#ifndef __ITCallStateEvent_FWD_DEFINED__
#define __ITCallStateEvent_FWD_DEFINED__
typedef interface ITCallStateEvent ITCallStateEvent;

#endif 	/* __ITCallStateEvent_FWD_DEFINED__ */


#ifndef __ITPhoneDeviceSpecificEvent_FWD_DEFINED__
#define __ITPhoneDeviceSpecificEvent_FWD_DEFINED__
typedef interface ITPhoneDeviceSpecificEvent ITPhoneDeviceSpecificEvent;

#endif 	/* __ITPhoneDeviceSpecificEvent_FWD_DEFINED__ */


#ifndef __ITCallMediaEvent_FWD_DEFINED__
#define __ITCallMediaEvent_FWD_DEFINED__
typedef interface ITCallMediaEvent ITCallMediaEvent;

#endif 	/* __ITCallMediaEvent_FWD_DEFINED__ */


#ifndef __ITDigitDetectionEvent_FWD_DEFINED__
#define __ITDigitDetectionEvent_FWD_DEFINED__
typedef interface ITDigitDetectionEvent ITDigitDetectionEvent;

#endif 	/* __ITDigitDetectionEvent_FWD_DEFINED__ */


#ifndef __ITDigitGenerationEvent_FWD_DEFINED__
#define __ITDigitGenerationEvent_FWD_DEFINED__
typedef interface ITDigitGenerationEvent ITDigitGenerationEvent;

#endif 	/* __ITDigitGenerationEvent_FWD_DEFINED__ */


#ifndef __ITDigitsGatheredEvent_FWD_DEFINED__
#define __ITDigitsGatheredEvent_FWD_DEFINED__
typedef interface ITDigitsGatheredEvent ITDigitsGatheredEvent;

#endif 	/* __ITDigitsGatheredEvent_FWD_DEFINED__ */


#ifndef __ITToneDetectionEvent_FWD_DEFINED__
#define __ITToneDetectionEvent_FWD_DEFINED__
typedef interface ITToneDetectionEvent ITToneDetectionEvent;

#endif 	/* __ITToneDetectionEvent_FWD_DEFINED__ */


#ifndef __ITTAPIObjectEvent_FWD_DEFINED__
#define __ITTAPIObjectEvent_FWD_DEFINED__
typedef interface ITTAPIObjectEvent ITTAPIObjectEvent;

#endif 	/* __ITTAPIObjectEvent_FWD_DEFINED__ */


#ifndef __ITTAPIObjectEvent2_FWD_DEFINED__
#define __ITTAPIObjectEvent2_FWD_DEFINED__
typedef interface ITTAPIObjectEvent2 ITTAPIObjectEvent2;

#endif 	/* __ITTAPIObjectEvent2_FWD_DEFINED__ */


#ifndef __ITTAPIEventNotification_FWD_DEFINED__
#define __ITTAPIEventNotification_FWD_DEFINED__
typedef interface ITTAPIEventNotification ITTAPIEventNotification;

#endif 	/* __ITTAPIEventNotification_FWD_DEFINED__ */


#ifndef __ITCallHubEvent_FWD_DEFINED__
#define __ITCallHubEvent_FWD_DEFINED__
typedef interface ITCallHubEvent ITCallHubEvent;

#endif 	/* __ITCallHubEvent_FWD_DEFINED__ */


#ifndef __ITAddressEvent_FWD_DEFINED__
#define __ITAddressEvent_FWD_DEFINED__
typedef interface ITAddressEvent ITAddressEvent;

#endif 	/* __ITAddressEvent_FWD_DEFINED__ */


#ifndef __ITAddressDeviceSpecificEvent_FWD_DEFINED__
#define __ITAddressDeviceSpecificEvent_FWD_DEFINED__
typedef interface ITAddressDeviceSpecificEvent ITAddressDeviceSpecificEvent;

#endif 	/* __ITAddressDeviceSpecificEvent_FWD_DEFINED__ */


#ifndef __ITFileTerminalEvent_FWD_DEFINED__
#define __ITFileTerminalEvent_FWD_DEFINED__
typedef interface ITFileTerminalEvent ITFileTerminalEvent;

#endif 	/* __ITFileTerminalEvent_FWD_DEFINED__ */


#ifndef __ITTTSTerminalEvent_FWD_DEFINED__
#define __ITTTSTerminalEvent_FWD_DEFINED__
typedef interface ITTTSTerminalEvent ITTTSTerminalEvent;

#endif 	/* __ITTTSTerminalEvent_FWD_DEFINED__ */


#ifndef __ITASRTerminalEvent_FWD_DEFINED__
#define __ITASRTerminalEvent_FWD_DEFINED__
typedef interface ITASRTerminalEvent ITASRTerminalEvent;

#endif 	/* __ITASRTerminalEvent_FWD_DEFINED__ */


#ifndef __ITToneTerminalEvent_FWD_DEFINED__
#define __ITToneTerminalEvent_FWD_DEFINED__
typedef interface ITToneTerminalEvent ITToneTerminalEvent;

#endif 	/* __ITToneTerminalEvent_FWD_DEFINED__ */


#ifndef __ITQOSEvent_FWD_DEFINED__
#define __ITQOSEvent_FWD_DEFINED__
typedef interface ITQOSEvent ITQOSEvent;

#endif 	/* __ITQOSEvent_FWD_DEFINED__ */


#ifndef __ITCallInfoChangeEvent_FWD_DEFINED__
#define __ITCallInfoChangeEvent_FWD_DEFINED__
typedef interface ITCallInfoChangeEvent ITCallInfoChangeEvent;

#endif 	/* __ITCallInfoChangeEvent_FWD_DEFINED__ */


#ifndef __ITRequest_FWD_DEFINED__
#define __ITRequest_FWD_DEFINED__
typedef interface ITRequest ITRequest;

#endif 	/* __ITRequest_FWD_DEFINED__ */


#ifndef __ITRequestEvent_FWD_DEFINED__
#define __ITRequestEvent_FWD_DEFINED__
typedef interface ITRequestEvent ITRequestEvent;

#endif 	/* __ITRequestEvent_FWD_DEFINED__ */


#ifndef __ITCollection_FWD_DEFINED__
#define __ITCollection_FWD_DEFINED__
typedef interface ITCollection ITCollection;

#endif 	/* __ITCollection_FWD_DEFINED__ */


#ifndef __ITCollection2_FWD_DEFINED__
#define __ITCollection2_FWD_DEFINED__
typedef interface ITCollection2 ITCollection2;

#endif 	/* __ITCollection2_FWD_DEFINED__ */


#ifndef __ITForwardInformation_FWD_DEFINED__
#define __ITForwardInformation_FWD_DEFINED__
typedef interface ITForwardInformation ITForwardInformation;

#endif 	/* __ITForwardInformation_FWD_DEFINED__ */


#ifndef __ITForwardInformation2_FWD_DEFINED__
#define __ITForwardInformation2_FWD_DEFINED__
typedef interface ITForwardInformation2 ITForwardInformation2;

#endif 	/* __ITForwardInformation2_FWD_DEFINED__ */


#ifndef __ITAddressTranslation_FWD_DEFINED__
#define __ITAddressTranslation_FWD_DEFINED__
typedef interface ITAddressTranslation ITAddressTranslation;

#endif 	/* __ITAddressTranslation_FWD_DEFINED__ */


#ifndef __ITAddressTranslationInfo_FWD_DEFINED__
#define __ITAddressTranslationInfo_FWD_DEFINED__
typedef interface ITAddressTranslationInfo ITAddressTranslationInfo;

#endif 	/* __ITAddressTranslationInfo_FWD_DEFINED__ */


#ifndef __ITLocationInfo_FWD_DEFINED__
#define __ITLocationInfo_FWD_DEFINED__
typedef interface ITLocationInfo ITLocationInfo;

#endif 	/* __ITLocationInfo_FWD_DEFINED__ */


#ifndef __IEnumLocation_FWD_DEFINED__
#define __IEnumLocation_FWD_DEFINED__
typedef interface IEnumLocation IEnumLocation;

#endif 	/* __IEnumLocation_FWD_DEFINED__ */


#ifndef __ITCallingCard_FWD_DEFINED__
#define __ITCallingCard_FWD_DEFINED__
typedef interface ITCallingCard ITCallingCard;

#endif 	/* __ITCallingCard_FWD_DEFINED__ */


#ifndef __IEnumCallingCard_FWD_DEFINED__
#define __IEnumCallingCard_FWD_DEFINED__
typedef interface IEnumCallingCard IEnumCallingCard;

#endif 	/* __IEnumCallingCard_FWD_DEFINED__ */


#ifndef __ITCallNotificationEvent_FWD_DEFINED__
#define __ITCallNotificationEvent_FWD_DEFINED__
typedef interface ITCallNotificationEvent ITCallNotificationEvent;

#endif 	/* __ITCallNotificationEvent_FWD_DEFINED__ */


#ifndef __ITDispatchMapper_FWD_DEFINED__
#define __ITDispatchMapper_FWD_DEFINED__
typedef interface ITDispatchMapper ITDispatchMapper;

#endif 	/* __ITDispatchMapper_FWD_DEFINED__ */


#ifndef __ITStreamControl_FWD_DEFINED__
#define __ITStreamControl_FWD_DEFINED__
typedef interface ITStreamControl ITStreamControl;

#endif 	/* __ITStreamControl_FWD_DEFINED__ */


#ifndef __ITStream_FWD_DEFINED__
#define __ITStream_FWD_DEFINED__
typedef interface ITStream ITStream;

#endif 	/* __ITStream_FWD_DEFINED__ */


#ifndef __IEnumStream_FWD_DEFINED__
#define __IEnumStream_FWD_DEFINED__
typedef interface IEnumStream IEnumStream;

#endif 	/* __IEnumStream_FWD_DEFINED__ */


#ifndef __ITSubStreamControl_FWD_DEFINED__
#define __ITSubStreamControl_FWD_DEFINED__
typedef interface ITSubStreamControl ITSubStreamControl;

#endif 	/* __ITSubStreamControl_FWD_DEFINED__ */


#ifndef __ITSubStream_FWD_DEFINED__
#define __ITSubStream_FWD_DEFINED__
typedef interface ITSubStream ITSubStream;

#endif 	/* __ITSubStream_FWD_DEFINED__ */


#ifndef __IEnumSubStream_FWD_DEFINED__
#define __IEnumSubStream_FWD_DEFINED__
typedef interface IEnumSubStream IEnumSubStream;

#endif 	/* __IEnumSubStream_FWD_DEFINED__ */


#ifndef __ITLegacyWaveSupport_FWD_DEFINED__
#define __ITLegacyWaveSupport_FWD_DEFINED__
typedef interface ITLegacyWaveSupport ITLegacyWaveSupport;

#endif 	/* __ITLegacyWaveSupport_FWD_DEFINED__ */


#ifndef __ITBasicCallControl2_FWD_DEFINED__
#define __ITBasicCallControl2_FWD_DEFINED__
typedef interface ITBasicCallControl2 ITBasicCallControl2;

#endif 	/* __ITBasicCallControl2_FWD_DEFINED__ */


#ifndef __ITScriptableAudioFormat_FWD_DEFINED__
#define __ITScriptableAudioFormat_FWD_DEFINED__
typedef interface ITScriptableAudioFormat ITScriptableAudioFormat;

#endif 	/* __ITScriptableAudioFormat_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "strmif.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_tapi3if_0000_0000 */
/* [local] */ 

/* Copyright (c) Microsoft Corporation. All rights reserved. */
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#if 0
typedef long TAPIHWND;

#endif
#ifdef _X86_
typedef long TAPIHWND;
#else
typedef LONGLONG TAPIHWND;
#endif
typedef long TAPI_DIGITMODE;

typedef 
enum TAPI_TONEMODE
    {
        TTM_RINGBACK	= 0x2,
        TTM_BUSY	= 0x4,
        TTM_BEEP	= 0x8,
        TTM_BILLING	= 0x10
    } 	TAPI_TONEMODE;

typedef 
enum TAPI_GATHERTERM
    {
        TGT_BUFFERFULL	= 0x1,
        TGT_TERMDIGIT	= 0x2,
        TGT_FIRSTTIMEOUT	= 0x4,
        TGT_INTERTIMEOUT	= 0x8,
        TGT_CANCEL	= 0x10
    } 	TAPI_GATHERTERM;

typedef struct TAPI_CUSTOMTONE
    {
    DWORD dwFrequency;
    DWORD dwCadenceOn;
    DWORD dwCadenceOff;
    DWORD dwVolume;
    } 	TAPI_CUSTOMTONE;

typedef struct TAPI_CUSTOMTONE *LPTAPI_CUSTOMTONE;

typedef struct TAPI_DETECTTONE
    {
    DWORD dwAppSpecific;
    DWORD dwDuration;
    DWORD dwFrequency1;
    DWORD dwFrequency2;
    DWORD dwFrequency3;
    } 	TAPI_DETECTTONE;

typedef struct TAPI_DETECTTONE *LPTAPI_DETECTTONE;

typedef 
enum ADDRESS_EVENT
    {
        AE_STATE	= 0,
        AE_CAPSCHANGE	= ( AE_STATE + 1 ) ,
        AE_RINGING	= ( AE_CAPSCHANGE + 1 ) ,
        AE_CONFIGCHANGE	= ( AE_RINGING + 1 ) ,
        AE_FORWARD	= ( AE_CONFIGCHANGE + 1 ) ,
        AE_NEWTERMINAL	= ( AE_FORWARD + 1 ) ,
        AE_REMOVETERMINAL	= ( AE_NEWTERMINAL + 1 ) ,
        AE_MSGWAITON	= ( AE_REMOVETERMINAL + 1 ) ,
        AE_MSGWAITOFF	= ( AE_MSGWAITON + 1 ) ,
        AE_LASTITEM	= AE_MSGWAITOFF
    } 	ADDRESS_EVENT;

typedef 
enum ADDRESS_STATE
    {
        AS_INSERVICE	= 0,
        AS_OUTOFSERVICE	= ( AS_INSERVICE + 1 ) 
    } 	ADDRESS_STATE;

typedef 
enum CALL_STATE
    {
        CS_IDLE	= 0,
        CS_INPROGRESS	= ( CS_IDLE + 1 ) ,
        CS_CONNECTED	= ( CS_INPROGRESS + 1 ) ,
        CS_DISCONNECTED	= ( CS_CONNECTED + 1 ) ,
        CS_OFFERING	= ( CS_DISCONNECTED + 1 ) ,
        CS_HOLD	= ( CS_OFFERING + 1 ) ,
        CS_QUEUED	= ( CS_HOLD + 1 ) ,
        CS_LASTITEM	= CS_QUEUED
    } 	CALL_STATE;

typedef 
enum CALL_STATE_EVENT_CAUSE
    {
        CEC_NONE	= 0,
        CEC_DISCONNECT_NORMAL	= ( CEC_NONE + 1 ) ,
        CEC_DISCONNECT_BUSY	= ( CEC_DISCONNECT_NORMAL + 1 ) ,
        CEC_DISCONNECT_BADADDRESS	= ( CEC_DISCONNECT_BUSY + 1 ) ,
        CEC_DISCONNECT_NOANSWER	= ( CEC_DISCONNECT_BADADDRESS + 1 ) ,
        CEC_DISCONNECT_CANCELLED	= ( CEC_DISCONNECT_NOANSWER + 1 ) ,
        CEC_DISCONNECT_REJECTED	= ( CEC_DISCONNECT_CANCELLED + 1 ) ,
        CEC_DISCONNECT_FAILED	= ( CEC_DISCONNECT_REJECTED + 1 ) ,
        CEC_DISCONNECT_BLOCKED	= ( CEC_DISCONNECT_FAILED + 1 ) 
    } 	CALL_STATE_EVENT_CAUSE;

typedef 
enum CALL_MEDIA_EVENT
    {
        CME_NEW_STREAM	= 0,
        CME_STREAM_FAIL	= ( CME_NEW_STREAM + 1 ) ,
        CME_TERMINAL_FAIL	= ( CME_STREAM_FAIL + 1 ) ,
        CME_STREAM_NOT_USED	= ( CME_TERMINAL_FAIL + 1 ) ,
        CME_STREAM_ACTIVE	= ( CME_STREAM_NOT_USED + 1 ) ,
        CME_STREAM_INACTIVE	= ( CME_STREAM_ACTIVE + 1 ) ,
        CME_LASTITEM	= CME_STREAM_INACTIVE
    } 	CALL_MEDIA_EVENT;

typedef 
enum CALL_MEDIA_EVENT_CAUSE
    {
        CMC_UNKNOWN	= 0,
        CMC_BAD_DEVICE	= ( CMC_UNKNOWN + 1 ) ,
        CMC_CONNECT_FAIL	= ( CMC_BAD_DEVICE + 1 ) ,
        CMC_LOCAL_REQUEST	= ( CMC_CONNECT_FAIL + 1 ) ,
        CMC_REMOTE_REQUEST	= ( CMC_LOCAL_REQUEST + 1 ) ,
        CMC_MEDIA_TIMEOUT	= ( CMC_REMOTE_REQUEST + 1 ) ,
        CMC_MEDIA_RECOVERED	= ( CMC_MEDIA_TIMEOUT + 1 ) ,
        CMC_QUALITY_OF_SERVICE	= ( CMC_MEDIA_RECOVERED + 1 ) 
    } 	CALL_MEDIA_EVENT_CAUSE;

typedef 
enum DISCONNECT_CODE
    {
        DC_NORMAL	= 0,
        DC_NOANSWER	= ( DC_NORMAL + 1 ) ,
        DC_REJECTED	= ( DC_NOANSWER + 1 ) 
    } 	DISCONNECT_CODE;

typedef 
enum TERMINAL_STATE
    {
        TS_INUSE	= 0,
        TS_NOTINUSE	= ( TS_INUSE + 1 ) 
    } 	TERMINAL_STATE;

typedef 
enum TERMINAL_DIRECTION
    {
        TD_CAPTURE	= 0,
        TD_RENDER	= ( TD_CAPTURE + 1 ) ,
        TD_BIDIRECTIONAL	= ( TD_RENDER + 1 ) ,
        TD_MULTITRACK_MIXED	= ( TD_BIDIRECTIONAL + 1 ) ,
        TD_NONE	= ( TD_MULTITRACK_MIXED + 1 ) 
    } 	TERMINAL_DIRECTION;

typedef 
enum TERMINAL_TYPE
    {
        TT_STATIC	= 0,
        TT_DYNAMIC	= ( TT_STATIC + 1 ) 
    } 	TERMINAL_TYPE;

typedef 
enum CALL_PRIVILEGE
    {
        CP_OWNER	= 0,
        CP_MONITOR	= ( CP_OWNER + 1 ) 
    } 	CALL_PRIVILEGE;

typedef 
enum TAPI_EVENT
    {
        TE_TAPIOBJECT	= 0x1,
        TE_ADDRESS	= 0x2,
        TE_CALLNOTIFICATION	= 0x4,
        TE_CALLSTATE	= 0x8,
        TE_CALLMEDIA	= 0x10,
        TE_CALLHUB	= 0x20,
        TE_CALLINFOCHANGE	= 0x40,
        TE_PRIVATE	= 0x80,
        TE_REQUEST	= 0x100,
        TE_AGENT	= 0x200,
        TE_AGENTSESSION	= 0x400,
        TE_QOSEVENT	= 0x800,
        TE_AGENTHANDLER	= 0x1000,
        TE_ACDGROUP	= 0x2000,
        TE_QUEUE	= 0x4000,
        TE_DIGITEVENT	= 0x8000,
        TE_GENERATEEVENT	= 0x10000,
        TE_ASRTERMINAL	= 0x20000,
        TE_TTSTERMINAL	= 0x40000,
        TE_FILETERMINAL	= 0x80000,
        TE_TONETERMINAL	= 0x100000,
        TE_PHONEEVENT	= 0x200000,
        TE_TONEEVENT	= 0x400000,
        TE_GATHERDIGITS	= 0x800000,
        TE_ADDRESSDEVSPECIFIC	= 0x1000000,
        TE_PHONEDEVSPECIFIC	= 0x2000000
    } 	TAPI_EVENT;

typedef 
enum CALL_NOTIFICATION_EVENT
    {
        CNE_OWNER	= 0,
        CNE_MONITOR	= ( CNE_OWNER + 1 ) ,
        CNE_LASTITEM	= CNE_MONITOR
    } 	CALL_NOTIFICATION_EVENT;

typedef 
enum CALLHUB_EVENT
    {
        CHE_CALLJOIN	= 0,
        CHE_CALLLEAVE	= ( CHE_CALLJOIN + 1 ) ,
        CHE_CALLHUBNEW	= ( CHE_CALLLEAVE + 1 ) ,
        CHE_CALLHUBIDLE	= ( CHE_CALLHUBNEW + 1 ) ,
        CHE_LASTITEM	= CHE_CALLHUBIDLE
    } 	CALLHUB_EVENT;

typedef 
enum CALLHUB_STATE
    {
        CHS_ACTIVE	= 0,
        CHS_IDLE	= ( CHS_ACTIVE + 1 ) 
    } 	CALLHUB_STATE;

typedef 
enum TAPIOBJECT_EVENT
    {
        TE_ADDRESSCREATE	= 0,
        TE_ADDRESSREMOVE	= ( TE_ADDRESSCREATE + 1 ) ,
        TE_REINIT	= ( TE_ADDRESSREMOVE + 1 ) ,
        TE_TRANSLATECHANGE	= ( TE_REINIT + 1 ) ,
        TE_ADDRESSCLOSE	= ( TE_TRANSLATECHANGE + 1 ) ,
        TE_PHONECREATE	= ( TE_ADDRESSCLOSE + 1 ) ,
        TE_PHONEREMOVE	= ( TE_PHONECREATE + 1 ) 
    } 	TAPIOBJECT_EVENT;

typedef 
enum TAPI_OBJECT_TYPE
    {
        TOT_NONE	= 0,
        TOT_TAPI	= ( TOT_NONE + 1 ) ,
        TOT_ADDRESS	= ( TOT_TAPI + 1 ) ,
        TOT_TERMINAL	= ( TOT_ADDRESS + 1 ) ,
        TOT_CALL	= ( TOT_TERMINAL + 1 ) ,
        TOT_CALLHUB	= ( TOT_CALL + 1 ) ,
        TOT_PHONE	= ( TOT_CALLHUB + 1 ) 
    } 	TAPI_OBJECT_TYPE;

typedef 
enum QOS_SERVICE_LEVEL
    {
        QSL_NEEDED	= 1,
        QSL_IF_AVAILABLE	= 2,
        QSL_BEST_EFFORT	= 3
    } 	QOS_SERVICE_LEVEL;

typedef 
enum QOS_EVENT
    {
        QE_NOQOS	= 1,
        QE_ADMISSIONFAILURE	= 2,
        QE_POLICYFAILURE	= 3,
        QE_GENERICERROR	= 4,
        QE_LASTITEM	= QE_GENERICERROR
    } 	QOS_EVENT;

typedef 
enum CALLINFOCHANGE_CAUSE
    {
        CIC_OTHER	= 0,
        CIC_DEVSPECIFIC	= ( CIC_OTHER + 1 ) ,
        CIC_BEARERMODE	= ( CIC_DEVSPECIFIC + 1 ) ,
        CIC_RATE	= ( CIC_BEARERMODE + 1 ) ,
        CIC_APPSPECIFIC	= ( CIC_RATE + 1 ) ,
        CIC_CALLID	= ( CIC_APPSPECIFIC + 1 ) ,
        CIC_RELATEDCALLID	= ( CIC_CALLID + 1 ) ,
        CIC_ORIGIN	= ( CIC_RELATEDCALLID + 1 ) ,
        CIC_REASON	= ( CIC_ORIGIN + 1 ) ,
        CIC_COMPLETIONID	= ( CIC_REASON + 1 ) ,
        CIC_NUMOWNERINCR	= ( CIC_COMPLETIONID + 1 ) ,
        CIC_NUMOWNERDECR	= ( CIC_NUMOWNERINCR + 1 ) ,
        CIC_NUMMONITORS	= ( CIC_NUMOWNERDECR + 1 ) ,
        CIC_TRUNK	= ( CIC_NUMMONITORS + 1 ) ,
        CIC_CALLERID	= ( CIC_TRUNK + 1 ) ,
        CIC_CALLEDID	= ( CIC_CALLERID + 1 ) ,
        CIC_CONNECTEDID	= ( CIC_CALLEDID + 1 ) ,
        CIC_REDIRECTIONID	= ( CIC_CONNECTEDID + 1 ) ,
        CIC_REDIRECTINGID	= ( CIC_REDIRECTIONID + 1 ) ,
        CIC_USERUSERINFO	= ( CIC_REDIRECTINGID + 1 ) ,
        CIC_HIGHLEVELCOMP	= ( CIC_USERUSERINFO + 1 ) ,
        CIC_LOWLEVELCOMP	= ( CIC_HIGHLEVELCOMP + 1 ) ,
        CIC_CHARGINGINFO	= ( CIC_LOWLEVELCOMP + 1 ) ,
        CIC_TREATMENT	= ( CIC_CHARGINGINFO + 1 ) ,
        CIC_CALLDATA	= ( CIC_TREATMENT + 1 ) ,
        CIC_PRIVILEGE	= ( CIC_CALLDATA + 1 ) ,
        CIC_MEDIATYPE	= ( CIC_PRIVILEGE + 1 ) ,
        CIC_LASTITEM	= CIC_MEDIATYPE
    } 	CALLINFOCHANGE_CAUSE;

typedef 
enum CALLINFO_LONG
    {
        CIL_MEDIATYPESAVAILABLE	= 0,
        CIL_BEARERMODE	= ( CIL_MEDIATYPESAVAILABLE + 1 ) ,
        CIL_CALLERIDADDRESSTYPE	= ( CIL_BEARERMODE + 1 ) ,
        CIL_CALLEDIDADDRESSTYPE	= ( CIL_CALLERIDADDRESSTYPE + 1 ) ,
        CIL_CONNECTEDIDADDRESSTYPE	= ( CIL_CALLEDIDADDRESSTYPE + 1 ) ,
        CIL_REDIRECTIONIDADDRESSTYPE	= ( CIL_CONNECTEDIDADDRESSTYPE + 1 ) ,
        CIL_REDIRECTINGIDADDRESSTYPE	= ( CIL_REDIRECTIONIDADDRESSTYPE + 1 ) ,
        CIL_ORIGIN	= ( CIL_REDIRECTINGIDADDRESSTYPE + 1 ) ,
        CIL_REASON	= ( CIL_ORIGIN + 1 ) ,
        CIL_APPSPECIFIC	= ( CIL_REASON + 1 ) ,
        CIL_CALLPARAMSFLAGS	= ( CIL_APPSPECIFIC + 1 ) ,
        CIL_CALLTREATMENT	= ( CIL_CALLPARAMSFLAGS + 1 ) ,
        CIL_MINRATE	= ( CIL_CALLTREATMENT + 1 ) ,
        CIL_MAXRATE	= ( CIL_MINRATE + 1 ) ,
        CIL_COUNTRYCODE	= ( CIL_MAXRATE + 1 ) ,
        CIL_CALLID	= ( CIL_COUNTRYCODE + 1 ) ,
        CIL_RELATEDCALLID	= ( CIL_CALLID + 1 ) ,
        CIL_COMPLETIONID	= ( CIL_RELATEDCALLID + 1 ) ,
        CIL_NUMBEROFOWNERS	= ( CIL_COMPLETIONID + 1 ) ,
        CIL_NUMBEROFMONITORS	= ( CIL_NUMBEROFOWNERS + 1 ) ,
        CIL_TRUNK	= ( CIL_NUMBEROFMONITORS + 1 ) ,
        CIL_RATE	= ( CIL_TRUNK + 1 ) ,
        CIL_GENERATEDIGITDURATION	= ( CIL_RATE + 1 ) ,
        CIL_MONITORDIGITMODES	= ( CIL_GENERATEDIGITDURATION + 1 ) ,
        CIL_MONITORMEDIAMODES	= ( CIL_MONITORDIGITMODES + 1 ) 
    } 	CALLINFO_LONG;

typedef 
enum CALLINFO_STRING
    {
        CIS_CALLERIDNAME	= 0,
        CIS_CALLERIDNUMBER	= ( CIS_CALLERIDNAME + 1 ) ,
        CIS_CALLEDIDNAME	= ( CIS_CALLERIDNUMBER + 1 ) ,
        CIS_CALLEDIDNUMBER	= ( CIS_CALLEDIDNAME + 1 ) ,
        CIS_CONNECTEDIDNAME	= ( CIS_CALLEDIDNUMBER + 1 ) ,
        CIS_CONNECTEDIDNUMBER	= ( CIS_CONNECTEDIDNAME + 1 ) ,
        CIS_REDIRECTIONIDNAME	= ( CIS_CONNECTEDIDNUMBER + 1 ) ,
        CIS_REDIRECTIONIDNUMBER	= ( CIS_REDIRECTIONIDNAME + 1 ) ,
        CIS_REDIRECTINGIDNAME	= ( CIS_REDIRECTIONIDNUMBER + 1 ) ,
        CIS_REDIRECTINGIDNUMBER	= ( CIS_REDIRECTINGIDNAME + 1 ) ,
        CIS_CALLEDPARTYFRIENDLYNAME	= ( CIS_REDIRECTINGIDNUMBER + 1 ) ,
        CIS_COMMENT	= ( CIS_CALLEDPARTYFRIENDLYNAME + 1 ) ,
        CIS_DISPLAYABLEADDRESS	= ( CIS_COMMENT + 1 ) ,
        CIS_CALLINGPARTYID	= ( CIS_DISPLAYABLEADDRESS + 1 ) 
    } 	CALLINFO_STRING;

typedef 
enum CALLINFO_BUFFER
    {
        CIB_USERUSERINFO	= 0,
        CIB_DEVSPECIFICBUFFER	= ( CIB_USERUSERINFO + 1 ) ,
        CIB_CALLDATABUFFER	= ( CIB_DEVSPECIFICBUFFER + 1 ) ,
        CIB_CHARGINGINFOBUFFER	= ( CIB_CALLDATABUFFER + 1 ) ,
        CIB_HIGHLEVELCOMPATIBILITYBUFFER	= ( CIB_CHARGINGINFOBUFFER + 1 ) ,
        CIB_LOWLEVELCOMPATIBILITYBUFFER	= ( CIB_HIGHLEVELCOMPATIBILITYBUFFER + 1 ) 
    } 	CALLINFO_BUFFER;

typedef 
enum ADDRESS_CAPABILITY
    {
        AC_ADDRESSTYPES	= 0,
        AC_BEARERMODES	= ( AC_ADDRESSTYPES + 1 ) ,
        AC_MAXACTIVECALLS	= ( AC_BEARERMODES + 1 ) ,
        AC_MAXONHOLDCALLS	= ( AC_MAXACTIVECALLS + 1 ) ,
        AC_MAXONHOLDPENDINGCALLS	= ( AC_MAXONHOLDCALLS + 1 ) ,
        AC_MAXNUMCONFERENCE	= ( AC_MAXONHOLDPENDINGCALLS + 1 ) ,
        AC_MAXNUMTRANSCONF	= ( AC_MAXNUMCONFERENCE + 1 ) ,
        AC_MONITORDIGITSUPPORT	= ( AC_MAXNUMTRANSCONF + 1 ) ,
        AC_GENERATEDIGITSUPPORT	= ( AC_MONITORDIGITSUPPORT + 1 ) ,
        AC_GENERATETONEMODES	= ( AC_GENERATEDIGITSUPPORT + 1 ) ,
        AC_GENERATETONEMAXNUMFREQ	= ( AC_GENERATETONEMODES + 1 ) ,
        AC_MONITORTONEMAXNUMFREQ	= ( AC_GENERATETONEMAXNUMFREQ + 1 ) ,
        AC_MONITORTONEMAXNUMENTRIES	= ( AC_MONITORTONEMAXNUMFREQ + 1 ) ,
        AC_DEVCAPFLAGS	= ( AC_MONITORTONEMAXNUMENTRIES + 1 ) ,
        AC_ANSWERMODES	= ( AC_DEVCAPFLAGS + 1 ) ,
        AC_LINEFEATURES	= ( AC_ANSWERMODES + 1 ) ,
        AC_SETTABLEDEVSTATUS	= ( AC_LINEFEATURES + 1 ) ,
        AC_PARKSUPPORT	= ( AC_SETTABLEDEVSTATUS + 1 ) ,
        AC_CALLERIDSUPPORT	= ( AC_PARKSUPPORT + 1 ) ,
        AC_CALLEDIDSUPPORT	= ( AC_CALLERIDSUPPORT + 1 ) ,
        AC_CONNECTEDIDSUPPORT	= ( AC_CALLEDIDSUPPORT + 1 ) ,
        AC_REDIRECTIONIDSUPPORT	= ( AC_CONNECTEDIDSUPPORT + 1 ) ,
        AC_REDIRECTINGIDSUPPORT	= ( AC_REDIRECTIONIDSUPPORT + 1 ) ,
        AC_ADDRESSCAPFLAGS	= ( AC_REDIRECTINGIDSUPPORT + 1 ) ,
        AC_CALLFEATURES1	= ( AC_ADDRESSCAPFLAGS + 1 ) ,
        AC_CALLFEATURES2	= ( AC_CALLFEATURES1 + 1 ) ,
        AC_REMOVEFROMCONFCAPS	= ( AC_CALLFEATURES2 + 1 ) ,
        AC_REMOVEFROMCONFSTATE	= ( AC_REMOVEFROMCONFCAPS + 1 ) ,
        AC_TRANSFERMODES	= ( AC_REMOVEFROMCONFSTATE + 1 ) ,
        AC_ADDRESSFEATURES	= ( AC_TRANSFERMODES + 1 ) ,
        AC_PREDICTIVEAUTOTRANSFERSTATES	= ( AC_ADDRESSFEATURES + 1 ) ,
        AC_MAXCALLDATASIZE	= ( AC_PREDICTIVEAUTOTRANSFERSTATES + 1 ) ,
        AC_LINEID	= ( AC_MAXCALLDATASIZE + 1 ) ,
        AC_ADDRESSID	= ( AC_LINEID + 1 ) ,
        AC_FORWARDMODES	= ( AC_ADDRESSID + 1 ) ,
        AC_MAXFORWARDENTRIES	= ( AC_FORWARDMODES + 1 ) ,
        AC_MAXSPECIFICENTRIES	= ( AC_MAXFORWARDENTRIES + 1 ) ,
        AC_MINFWDNUMRINGS	= ( AC_MAXSPECIFICENTRIES + 1 ) ,
        AC_MAXFWDNUMRINGS	= ( AC_MINFWDNUMRINGS + 1 ) ,
        AC_MAXCALLCOMPLETIONS	= ( AC_MAXFWDNUMRINGS + 1 ) ,
        AC_CALLCOMPLETIONCONDITIONS	= ( AC_MAXCALLCOMPLETIONS + 1 ) ,
        AC_CALLCOMPLETIONMODES	= ( AC_CALLCOMPLETIONCONDITIONS + 1 ) ,
        AC_PERMANENTDEVICEID	= ( AC_CALLCOMPLETIONMODES + 1 ) ,
        AC_GATHERDIGITSMINTIMEOUT	= ( AC_PERMANENTDEVICEID + 1 ) ,
        AC_GATHERDIGITSMAXTIMEOUT	= ( AC_GATHERDIGITSMINTIMEOUT + 1 ) ,
        AC_GENERATEDIGITMINDURATION	= ( AC_GATHERDIGITSMAXTIMEOUT + 1 ) ,
        AC_GENERATEDIGITMAXDURATION	= ( AC_GENERATEDIGITMINDURATION + 1 ) ,
        AC_GENERATEDIGITDEFAULTDURATION	= ( AC_GENERATEDIGITMAXDURATION + 1 ) 
    } 	ADDRESS_CAPABILITY;

typedef 
enum ADDRESS_CAPABILITY_STRING
    {
        ACS_PROTOCOL	= 0,
        ACS_ADDRESSDEVICESPECIFIC	= ( ACS_PROTOCOL + 1 ) ,
        ACS_LINEDEVICESPECIFIC	= ( ACS_ADDRESSDEVICESPECIFIC + 1 ) ,
        ACS_PROVIDERSPECIFIC	= ( ACS_LINEDEVICESPECIFIC + 1 ) ,
        ACS_SWITCHSPECIFIC	= ( ACS_PROVIDERSPECIFIC + 1 ) ,
        ACS_PERMANENTDEVICEGUID	= ( ACS_SWITCHSPECIFIC + 1 ) 
    } 	ADDRESS_CAPABILITY_STRING;

typedef 
enum FULLDUPLEX_SUPPORT
    {
        FDS_SUPPORTED	= 0,
        FDS_NOTSUPPORTED	= ( FDS_SUPPORTED + 1 ) ,
        FDS_UNKNOWN	= ( FDS_NOTSUPPORTED + 1 ) 
    } 	FULLDUPLEX_SUPPORT;

typedef 
enum FINISH_MODE
    {
        FM_ASTRANSFER	= 0,
        FM_ASCONFERENCE	= ( FM_ASTRANSFER + 1 ) 
    } 	FINISH_MODE;

typedef 
enum PHONE_PRIVILEGE
    {
        PP_OWNER	= 0,
        PP_MONITOR	= ( PP_OWNER + 1 ) 
    } 	PHONE_PRIVILEGE;

typedef 
enum PHONE_HOOK_SWITCH_DEVICE
    {
        PHSD_HANDSET	= 0x1,
        PHSD_SPEAKERPHONE	= 0x2,
        PHSD_HEADSET	= 0x4
    } 	PHONE_HOOK_SWITCH_DEVICE;

typedef 
enum PHONE_HOOK_SWITCH_STATE
    {
        PHSS_ONHOOK	= 0x1,
        PHSS_OFFHOOK_MIC_ONLY	= 0x2,
        PHSS_OFFHOOK_SPEAKER_ONLY	= 0x4,
        PHSS_OFFHOOK	= 0x8
    } 	PHONE_HOOK_SWITCH_STATE;

typedef 
enum PHONE_LAMP_MODE
    {
        LM_DUMMY	= 0x1,
        LM_OFF	= 0x2,
        LM_STEADY	= 0x4,
        LM_WINK	= 0x8,
        LM_FLASH	= 0x10,
        LM_FLUTTER	= 0x20,
        LM_BROKENFLUTTER	= 0x40,
        LM_UNKNOWN	= 0x80
    } 	PHONE_LAMP_MODE;

typedef 
enum PHONECAPS_LONG
    {
        PCL_HOOKSWITCHES	= 0,
        PCL_HANDSETHOOKSWITCHMODES	= ( PCL_HOOKSWITCHES + 1 ) ,
        PCL_HEADSETHOOKSWITCHMODES	= ( PCL_HANDSETHOOKSWITCHMODES + 1 ) ,
        PCL_SPEAKERPHONEHOOKSWITCHMODES	= ( PCL_HEADSETHOOKSWITCHMODES + 1 ) ,
        PCL_DISPLAYNUMROWS	= ( PCL_SPEAKERPHONEHOOKSWITCHMODES + 1 ) ,
        PCL_DISPLAYNUMCOLUMNS	= ( PCL_DISPLAYNUMROWS + 1 ) ,
        PCL_NUMRINGMODES	= ( PCL_DISPLAYNUMCOLUMNS + 1 ) ,
        PCL_NUMBUTTONLAMPS	= ( PCL_NUMRINGMODES + 1 ) ,
        PCL_GENERICPHONE	= ( PCL_NUMBUTTONLAMPS + 1 ) 
    } 	PHONECAPS_LONG;

typedef 
enum PHONECAPS_STRING
    {
        PCS_PHONENAME	= 0,
        PCS_PHONEINFO	= ( PCS_PHONENAME + 1 ) ,
        PCS_PROVIDERINFO	= ( PCS_PHONEINFO + 1 ) 
    } 	PHONECAPS_STRING;

typedef 
enum PHONECAPS_BUFFER
    {
        PCB_DEVSPECIFICBUFFER	= 0
    } 	PHONECAPS_BUFFER;

typedef 
enum PHONE_BUTTON_STATE
    {
        PBS_UP	= 0x1,
        PBS_DOWN	= 0x2,
        PBS_UNKNOWN	= 0x4,
        PBS_UNAVAIL	= 0x8
    } 	PHONE_BUTTON_STATE;

typedef 
enum PHONE_BUTTON_MODE
    {
        PBM_DUMMY	= 0,
        PBM_CALL	= ( PBM_DUMMY + 1 ) ,
        PBM_FEATURE	= ( PBM_CALL + 1 ) ,
        PBM_KEYPAD	= ( PBM_FEATURE + 1 ) ,
        PBM_LOCAL	= ( PBM_KEYPAD + 1 ) ,
        PBM_DISPLAY	= ( PBM_LOCAL + 1 ) 
    } 	PHONE_BUTTON_MODE;

typedef 
enum PHONE_BUTTON_FUNCTION
    {
        PBF_UNKNOWN	= 0,
        PBF_CONFERENCE	= ( PBF_UNKNOWN + 1 ) ,
        PBF_TRANSFER	= ( PBF_CONFERENCE + 1 ) ,
        PBF_DROP	= ( PBF_TRANSFER + 1 ) ,
        PBF_HOLD	= ( PBF_DROP + 1 ) ,
        PBF_RECALL	= ( PBF_HOLD + 1 ) ,
        PBF_DISCONNECT	= ( PBF_RECALL + 1 ) ,
        PBF_CONNECT	= ( PBF_DISCONNECT + 1 ) ,
        PBF_MSGWAITON	= ( PBF_CONNECT + 1 ) ,
        PBF_MSGWAITOFF	= ( PBF_MSGWAITON + 1 ) ,
        PBF_SELECTRING	= ( PBF_MSGWAITOFF + 1 ) ,
        PBF_ABBREVDIAL	= ( PBF_SELECTRING + 1 ) ,
        PBF_FORWARD	= ( PBF_ABBREVDIAL + 1 ) ,
        PBF_PICKUP	= ( PBF_FORWARD + 1 ) ,
        PBF_RINGAGAIN	= ( PBF_PICKUP + 1 ) ,
        PBF_PARK	= ( PBF_RINGAGAIN + 1 ) ,
        PBF_REJECT	= ( PBF_PARK + 1 ) ,
        PBF_REDIRECT	= ( PBF_REJECT + 1 ) ,
        PBF_MUTE	= ( PBF_REDIRECT + 1 ) ,
        PBF_VOLUMEUP	= ( PBF_MUTE + 1 ) ,
        PBF_VOLUMEDOWN	= ( PBF_VOLUMEUP + 1 ) ,
        PBF_SPEAKERON	= ( PBF_VOLUMEDOWN + 1 ) ,
        PBF_SPEAKEROFF	= ( PBF_SPEAKERON + 1 ) ,
        PBF_FLASH	= ( PBF_SPEAKEROFF + 1 ) ,
        PBF_DATAON	= ( PBF_FLASH + 1 ) ,
        PBF_DATAOFF	= ( PBF_DATAON + 1 ) ,
        PBF_DONOTDISTURB	= ( PBF_DATAOFF + 1 ) ,
        PBF_INTERCOM	= ( PBF_DONOTDISTURB + 1 ) ,
        PBF_BRIDGEDAPP	= ( PBF_INTERCOM + 1 ) ,
        PBF_BUSY	= ( PBF_BRIDGEDAPP + 1 ) ,
        PBF_CALLAPP	= ( PBF_BUSY + 1 ) ,
        PBF_DATETIME	= ( PBF_CALLAPP + 1 ) ,
        PBF_DIRECTORY	= ( PBF_DATETIME + 1 ) ,
        PBF_COVER	= ( PBF_DIRECTORY + 1 ) ,
        PBF_CALLID	= ( PBF_COVER + 1 ) ,
        PBF_LASTNUM	= ( PBF_CALLID + 1 ) ,
        PBF_NIGHTSRV	= ( PBF_LASTNUM + 1 ) ,
        PBF_SENDCALLS	= ( PBF_NIGHTSRV + 1 ) ,
        PBF_MSGINDICATOR	= ( PBF_SENDCALLS + 1 ) ,
        PBF_REPDIAL	= ( PBF_MSGINDICATOR + 1 ) ,
        PBF_SETREPDIAL	= ( PBF_REPDIAL + 1 ) ,
        PBF_SYSTEMSPEED	= ( PBF_SETREPDIAL + 1 ) ,
        PBF_STATIONSPEED	= ( PBF_SYSTEMSPEED + 1 ) ,
        PBF_CAMPON	= ( PBF_STATIONSPEED + 1 ) ,
        PBF_SAVEREPEAT	= ( PBF_CAMPON + 1 ) ,
        PBF_QUEUECALL	= ( PBF_SAVEREPEAT + 1 ) ,
        PBF_NONE	= ( PBF_QUEUECALL + 1 ) ,
        PBF_SEND	= ( PBF_NONE + 1 ) 
    } 	PHONE_BUTTON_FUNCTION;

typedef 
enum PHONE_TONE
    {
        PT_KEYPADZERO	= 0,
        PT_KEYPADONE	= ( PT_KEYPADZERO + 1 ) ,
        PT_KEYPADTWO	= ( PT_KEYPADONE + 1 ) ,
        PT_KEYPADTHREE	= ( PT_KEYPADTWO + 1 ) ,
        PT_KEYPADFOUR	= ( PT_KEYPADTHREE + 1 ) ,
        PT_KEYPADFIVE	= ( PT_KEYPADFOUR + 1 ) ,
        PT_KEYPADSIX	= ( PT_KEYPADFIVE + 1 ) ,
        PT_KEYPADSEVEN	= ( PT_KEYPADSIX + 1 ) ,
        PT_KEYPADEIGHT	= ( PT_KEYPADSEVEN + 1 ) ,
        PT_KEYPADNINE	= ( PT_KEYPADEIGHT + 1 ) ,
        PT_KEYPADSTAR	= ( PT_KEYPADNINE + 1 ) ,
        PT_KEYPADPOUND	= ( PT_KEYPADSTAR + 1 ) ,
        PT_KEYPADA	= ( PT_KEYPADPOUND + 1 ) ,
        PT_KEYPADB	= ( PT_KEYPADA + 1 ) ,
        PT_KEYPADC	= ( PT_KEYPADB + 1 ) ,
        PT_KEYPADD	= ( PT_KEYPADC + 1 ) ,
        PT_NORMALDIALTONE	= ( PT_KEYPADD + 1 ) ,
        PT_EXTERNALDIALTONE	= ( PT_NORMALDIALTONE + 1 ) ,
        PT_BUSY	= ( PT_EXTERNALDIALTONE + 1 ) ,
        PT_RINGBACK	= ( PT_BUSY + 1 ) ,
        PT_ERRORTONE	= ( PT_RINGBACK + 1 ) ,
        PT_SILENCE	= ( PT_ERRORTONE + 1 ) 
    } 	PHONE_TONE;

typedef 
enum PHONE_EVENT
    {
        PE_DISPLAY	= 0,
        PE_LAMPMODE	= ( PE_DISPLAY + 1 ) ,
        PE_RINGMODE	= ( PE_LAMPMODE + 1 ) ,
        PE_RINGVOLUME	= ( PE_RINGMODE + 1 ) ,
        PE_HOOKSWITCH	= ( PE_RINGVOLUME + 1 ) ,
        PE_CAPSCHANGE	= ( PE_HOOKSWITCH + 1 ) ,
        PE_BUTTON	= ( PE_CAPSCHANGE + 1 ) ,
        PE_CLOSE	= ( PE_BUTTON + 1 ) ,
        PE_NUMBERGATHERED	= ( PE_CLOSE + 1 ) ,
        PE_DIALING	= ( PE_NUMBERGATHERED + 1 ) ,
        PE_ANSWER	= ( PE_DIALING + 1 ) ,
        PE_DISCONNECT	= ( PE_ANSWER + 1 ) ,
        PE_LASTITEM	= PE_DISCONNECT
    } 	PHONE_EVENT;

#define	INTERFACEMASK	( 0xff0000 )

#define	DISPIDMASK	( 0xffff )

#define	IDISPTAPI	( 0x10000 )

#define	IDISPTAPICALLCENTER	( 0x20000 )

#define	IDISPCALLINFO	( 0x10000 )

#define	IDISPBASICCALLCONTROL	( 0x20000 )

#define	IDISPLEGACYCALLMEDIACONTROL	( 0x30000 )

#define	IDISPAGGREGATEDMSPCALLOBJ	( 0x40000 )

#define	IDISPADDRESS	( 0x10000 )

#define	IDISPADDRESSCAPABILITIES	( 0x20000 )

#define	IDISPMEDIASUPPORT	( 0x30000 )

#define	IDISPADDRESSTRANSLATION	( 0x40000 )

#define	IDISPLEGACYADDRESSMEDIACONTROL	( 0x50000 )

#define	IDISPAGGREGATEDMSPADDRESSOBJ	( 0x60000 )

#define	IDISPPHONE	( 0x10000 )

#define	IDISPAPC	( 0x20000 )

#define	IDISPMULTITRACK	( 0x10000 )

#define	IDISPMEDIACONTROL	( 0x20000 )

#define	IDISPMEDIARECORD	( 0x30000 )

#define	IDISPMEDIAPLAYBACK	( 0x40000 )

#define	IDISPFILETRACK	( 0x10000 )



















































extern RPC_IF_HANDLE __MIDL_itf_tapi3if_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_tapi3if_0000_0000_v0_0_s_ifspec;

#ifndef __ITTAPI_INTERFACE_DEFINED__
#define __ITTAPI_INTERFACE_DEFINED__

/* interface ITTAPI */
/* [object][dual][unique][helpstring][uuid] */ 


EXTERN_C const IID IID_ITTAPI;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B1EFC382-9355-11d0-835C-00AA003CCABD")
    ITTAPI : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Initialize( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Shutdown( void) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Addresses( 
            /* [retval][out] */ __RPC__out VARIANT *pVariant) = 0;
        
        virtual /* [helpstring][hidden][id] */ HRESULT STDMETHODCALLTYPE EnumerateAddresses( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumAddress **ppEnumAddress) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE RegisterCallNotifications( 
            /* [in] */ __RPC__in_opt ITAddress *pAddress,
            /* [in] */ VARIANT_BOOL fMonitor,
            /* [in] */ VARIANT_BOOL fOwner,
            /* [in] */ long lMediaTypes,
            /* [in] */ long lCallbackInstance,
            /* [retval][out] */ __RPC__out long *plRegister) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE UnregisterNotifications( 
            /* [in] */ long lRegister) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CallHubs( 
            /* [retval][out] */ __RPC__out VARIANT *pVariant) = 0;
        
        virtual /* [helpstring][hidden][id] */ HRESULT STDMETHODCALLTYPE EnumerateCallHubs( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumCallHub **ppEnumCallHub) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetCallHubTracking( 
            /* [in] */ VARIANT pAddresses,
            /* [in] */ VARIANT_BOOL bTracking) = 0;
        
        virtual /* [hidden][id] */ HRESULT STDMETHODCALLTYPE EnumeratePrivateTAPIObjects( 
            /* [out] */ __RPC__deref_out_opt IEnumUnknown **ppEnumUnknown) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PrivateTAPIObjects( 
            /* [retval][out] */ __RPC__out VARIANT *pVariant) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE RegisterRequestRecipient( 
            /* [in] */ long lRegistrationInstance,
            /* [in] */ long lRequestMode,
            /* [in] */ VARIANT_BOOL fEnable) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetAssistedTelephonyPriority( 
            /* [in] */ __RPC__in BSTR pAppFilename,
            /* [in] */ VARIANT_BOOL fPriority) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetApplicationPriority( 
            /* [in] */ __RPC__in BSTR pAppFilename,
            /* [in] */ long lMediaType,
            /* [in] */ VARIANT_BOOL fPriority) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_EventFilter( 
            /* [in] */ long lFilterMask) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_EventFilter( 
            /* [retval][out] */ __RPC__out long *plFilterMask) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITTAPIVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITTAPI * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITTAPI * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITTAPI * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITTAPI * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITTAPI * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITTAPI * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITTAPI * This,
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
        
        DECLSPEC_XFGVIRT(ITTAPI, Initialize)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            __RPC__in ITTAPI * This);
        
        DECLSPEC_XFGVIRT(ITTAPI, Shutdown)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Shutdown )( 
            __RPC__in ITTAPI * This);
        
        DECLSPEC_XFGVIRT(ITTAPI, get_Addresses)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Addresses )( 
            __RPC__in ITTAPI * This,
            /* [retval][out] */ __RPC__out VARIANT *pVariant);
        
        DECLSPEC_XFGVIRT(ITTAPI, EnumerateAddresses)
        /* [helpstring][hidden][id] */ HRESULT ( STDMETHODCALLTYPE *EnumerateAddresses )( 
            __RPC__in ITTAPI * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumAddress **ppEnumAddress);
        
        DECLSPEC_XFGVIRT(ITTAPI, RegisterCallNotifications)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RegisterCallNotifications )( 
            __RPC__in ITTAPI * This,
            /* [in] */ __RPC__in_opt ITAddress *pAddress,
            /* [in] */ VARIANT_BOOL fMonitor,
            /* [in] */ VARIANT_BOOL fOwner,
            /* [in] */ long lMediaTypes,
            /* [in] */ long lCallbackInstance,
            /* [retval][out] */ __RPC__out long *plRegister);
        
        DECLSPEC_XFGVIRT(ITTAPI, UnregisterNotifications)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *UnregisterNotifications )( 
            __RPC__in ITTAPI * This,
            /* [in] */ long lRegister);
        
        DECLSPEC_XFGVIRT(ITTAPI, get_CallHubs)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CallHubs )( 
            __RPC__in ITTAPI * This,
            /* [retval][out] */ __RPC__out VARIANT *pVariant);
        
        DECLSPEC_XFGVIRT(ITTAPI, EnumerateCallHubs)
        /* [helpstring][hidden][id] */ HRESULT ( STDMETHODCALLTYPE *EnumerateCallHubs )( 
            __RPC__in ITTAPI * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumCallHub **ppEnumCallHub);
        
        DECLSPEC_XFGVIRT(ITTAPI, SetCallHubTracking)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetCallHubTracking )( 
            __RPC__in ITTAPI * This,
            /* [in] */ VARIANT pAddresses,
            /* [in] */ VARIANT_BOOL bTracking);
        
        DECLSPEC_XFGVIRT(ITTAPI, EnumeratePrivateTAPIObjects)
        /* [hidden][id] */ HRESULT ( STDMETHODCALLTYPE *EnumeratePrivateTAPIObjects )( 
            __RPC__in ITTAPI * This,
            /* [out] */ __RPC__deref_out_opt IEnumUnknown **ppEnumUnknown);
        
        DECLSPEC_XFGVIRT(ITTAPI, get_PrivateTAPIObjects)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PrivateTAPIObjects )( 
            __RPC__in ITTAPI * This,
            /* [retval][out] */ __RPC__out VARIANT *pVariant);
        
        DECLSPEC_XFGVIRT(ITTAPI, RegisterRequestRecipient)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RegisterRequestRecipient )( 
            __RPC__in ITTAPI * This,
            /* [in] */ long lRegistrationInstance,
            /* [in] */ long lRequestMode,
            /* [in] */ VARIANT_BOOL fEnable);
        
        DECLSPEC_XFGVIRT(ITTAPI, SetAssistedTelephonyPriority)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetAssistedTelephonyPriority )( 
            __RPC__in ITTAPI * This,
            /* [in] */ __RPC__in BSTR pAppFilename,
            /* [in] */ VARIANT_BOOL fPriority);
        
        DECLSPEC_XFGVIRT(ITTAPI, SetApplicationPriority)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetApplicationPriority )( 
            __RPC__in ITTAPI * This,
            /* [in] */ __RPC__in BSTR pAppFilename,
            /* [in] */ long lMediaType,
            /* [in] */ VARIANT_BOOL fPriority);
        
        DECLSPEC_XFGVIRT(ITTAPI, put_EventFilter)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_EventFilter )( 
            __RPC__in ITTAPI * This,
            /* [in] */ long lFilterMask);
        
        DECLSPEC_XFGVIRT(ITTAPI, get_EventFilter)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EventFilter )( 
            __RPC__in ITTAPI * This,
            /* [retval][out] */ __RPC__out long *plFilterMask);
        
        END_INTERFACE
    } ITTAPIVtbl;

    interface ITTAPI
    {
        CONST_VTBL struct ITTAPIVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITTAPI_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITTAPI_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITTAPI_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITTAPI_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITTAPI_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITTAPI_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITTAPI_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITTAPI_Initialize(This)	\
    ( (This)->lpVtbl -> Initialize(This) ) 

#define ITTAPI_Shutdown(This)	\
    ( (This)->lpVtbl -> Shutdown(This) ) 

#define ITTAPI_get_Addresses(This,pVariant)	\
    ( (This)->lpVtbl -> get_Addresses(This,pVariant) ) 

#define ITTAPI_EnumerateAddresses(This,ppEnumAddress)	\
    ( (This)->lpVtbl -> EnumerateAddresses(This,ppEnumAddress) ) 

#define ITTAPI_RegisterCallNotifications(This,pAddress,fMonitor,fOwner,lMediaTypes,lCallbackInstance,plRegister)	\
    ( (This)->lpVtbl -> RegisterCallNotifications(This,pAddress,fMonitor,fOwner,lMediaTypes,lCallbackInstance,plRegister) ) 

#define ITTAPI_UnregisterNotifications(This,lRegister)	\
    ( (This)->lpVtbl -> UnregisterNotifications(This,lRegister) ) 

#define ITTAPI_get_CallHubs(This,pVariant)	\
    ( (This)->lpVtbl -> get_CallHubs(This,pVariant) ) 

#define ITTAPI_EnumerateCallHubs(This,ppEnumCallHub)	\
    ( (This)->lpVtbl -> EnumerateCallHubs(This,ppEnumCallHub) ) 

#define ITTAPI_SetCallHubTracking(This,pAddresses,bTracking)	\
    ( (This)->lpVtbl -> SetCallHubTracking(This,pAddresses,bTracking) ) 

#define ITTAPI_EnumeratePrivateTAPIObjects(This,ppEnumUnknown)	\
    ( (This)->lpVtbl -> EnumeratePrivateTAPIObjects(This,ppEnumUnknown) ) 

#define ITTAPI_get_PrivateTAPIObjects(This,pVariant)	\
    ( (This)->lpVtbl -> get_PrivateTAPIObjects(This,pVariant) ) 

#define ITTAPI_RegisterRequestRecipient(This,lRegistrationInstance,lRequestMode,fEnable)	\
    ( (This)->lpVtbl -> RegisterRequestRecipient(This,lRegistrationInstance,lRequestMode,fEnable) ) 

#define ITTAPI_SetAssistedTelephonyPriority(This,pAppFilename,fPriority)	\
    ( (This)->lpVtbl -> SetAssistedTelephonyPriority(This,pAppFilename,fPriority) ) 

#define ITTAPI_SetApplicationPriority(This,pAppFilename,lMediaType,fPriority)	\
    ( (This)->lpVtbl -> SetApplicationPriority(This,pAppFilename,lMediaType,fPriority) ) 

#define ITTAPI_put_EventFilter(This,lFilterMask)	\
    ( (This)->lpVtbl -> put_EventFilter(This,lFilterMask) ) 

#define ITTAPI_get_EventFilter(This,plFilterMask)	\
    ( (This)->lpVtbl -> get_EventFilter(This,plFilterMask) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITTAPI_INTERFACE_DEFINED__ */


#ifndef __ITTAPI2_INTERFACE_DEFINED__
#define __ITTAPI2_INTERFACE_DEFINED__

/* interface ITTAPI2 */
/* [object][dual][helpstring][uuid] */ 


EXTERN_C const IID IID_ITTAPI2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("54FBDC8C-D90F-4dad-9695-B373097F094B")
    ITTAPI2 : public ITTAPI
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Phones( 
            /* [retval][out] */ __RPC__out VARIANT *pPhones) = 0;
        
        virtual /* [helpstring][hidden][id] */ HRESULT STDMETHODCALLTYPE EnumeratePhones( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumPhone **ppEnumPhone) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateEmptyCollectionObject( 
            /* [retval][out] */ __RPC__deref_out_opt ITCollection2 **ppCollection) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITTAPI2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITTAPI2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITTAPI2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITTAPI2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITTAPI2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITTAPI2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITTAPI2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITTAPI2 * This,
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
        
        DECLSPEC_XFGVIRT(ITTAPI, Initialize)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            __RPC__in ITTAPI2 * This);
        
        DECLSPEC_XFGVIRT(ITTAPI, Shutdown)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Shutdown )( 
            __RPC__in ITTAPI2 * This);
        
        DECLSPEC_XFGVIRT(ITTAPI, get_Addresses)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Addresses )( 
            __RPC__in ITTAPI2 * This,
            /* [retval][out] */ __RPC__out VARIANT *pVariant);
        
        DECLSPEC_XFGVIRT(ITTAPI, EnumerateAddresses)
        /* [helpstring][hidden][id] */ HRESULT ( STDMETHODCALLTYPE *EnumerateAddresses )( 
            __RPC__in ITTAPI2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumAddress **ppEnumAddress);
        
        DECLSPEC_XFGVIRT(ITTAPI, RegisterCallNotifications)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RegisterCallNotifications )( 
            __RPC__in ITTAPI2 * This,
            /* [in] */ __RPC__in_opt ITAddress *pAddress,
            /* [in] */ VARIANT_BOOL fMonitor,
            /* [in] */ VARIANT_BOOL fOwner,
            /* [in] */ long lMediaTypes,
            /* [in] */ long lCallbackInstance,
            /* [retval][out] */ __RPC__out long *plRegister);
        
        DECLSPEC_XFGVIRT(ITTAPI, UnregisterNotifications)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *UnregisterNotifications )( 
            __RPC__in ITTAPI2 * This,
            /* [in] */ long lRegister);
        
        DECLSPEC_XFGVIRT(ITTAPI, get_CallHubs)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CallHubs )( 
            __RPC__in ITTAPI2 * This,
            /* [retval][out] */ __RPC__out VARIANT *pVariant);
        
        DECLSPEC_XFGVIRT(ITTAPI, EnumerateCallHubs)
        /* [helpstring][hidden][id] */ HRESULT ( STDMETHODCALLTYPE *EnumerateCallHubs )( 
            __RPC__in ITTAPI2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumCallHub **ppEnumCallHub);
        
        DECLSPEC_XFGVIRT(ITTAPI, SetCallHubTracking)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetCallHubTracking )( 
            __RPC__in ITTAPI2 * This,
            /* [in] */ VARIANT pAddresses,
            /* [in] */ VARIANT_BOOL bTracking);
        
        DECLSPEC_XFGVIRT(ITTAPI, EnumeratePrivateTAPIObjects)
        /* [hidden][id] */ HRESULT ( STDMETHODCALLTYPE *EnumeratePrivateTAPIObjects )( 
            __RPC__in ITTAPI2 * This,
            /* [out] */ __RPC__deref_out_opt IEnumUnknown **ppEnumUnknown);
        
        DECLSPEC_XFGVIRT(ITTAPI, get_PrivateTAPIObjects)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PrivateTAPIObjects )( 
            __RPC__in ITTAPI2 * This,
            /* [retval][out] */ __RPC__out VARIANT *pVariant);
        
        DECLSPEC_XFGVIRT(ITTAPI, RegisterRequestRecipient)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RegisterRequestRecipient )( 
            __RPC__in ITTAPI2 * This,
            /* [in] */ long lRegistrationInstance,
            /* [in] */ long lRequestMode,
            /* [in] */ VARIANT_BOOL fEnable);
        
        DECLSPEC_XFGVIRT(ITTAPI, SetAssistedTelephonyPriority)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetAssistedTelephonyPriority )( 
            __RPC__in ITTAPI2 * This,
            /* [in] */ __RPC__in BSTR pAppFilename,
            /* [in] */ VARIANT_BOOL fPriority);
        
        DECLSPEC_XFGVIRT(ITTAPI, SetApplicationPriority)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetApplicationPriority )( 
            __RPC__in ITTAPI2 * This,
            /* [in] */ __RPC__in BSTR pAppFilename,
            /* [in] */ long lMediaType,
            /* [in] */ VARIANT_BOOL fPriority);
        
        DECLSPEC_XFGVIRT(ITTAPI, put_EventFilter)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_EventFilter )( 
            __RPC__in ITTAPI2 * This,
            /* [in] */ long lFilterMask);
        
        DECLSPEC_XFGVIRT(ITTAPI, get_EventFilter)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EventFilter )( 
            __RPC__in ITTAPI2 * This,
            /* [retval][out] */ __RPC__out long *plFilterMask);
        
        DECLSPEC_XFGVIRT(ITTAPI2, get_Phones)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Phones )( 
            __RPC__in ITTAPI2 * This,
            /* [retval][out] */ __RPC__out VARIANT *pPhones);
        
        DECLSPEC_XFGVIRT(ITTAPI2, EnumeratePhones)
        /* [helpstring][hidden][id] */ HRESULT ( STDMETHODCALLTYPE *EnumeratePhones )( 
            __RPC__in ITTAPI2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumPhone **ppEnumPhone);
        
        DECLSPEC_XFGVIRT(ITTAPI2, CreateEmptyCollectionObject)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateEmptyCollectionObject )( 
            __RPC__in ITTAPI2 * This,
            /* [retval][out] */ __RPC__deref_out_opt ITCollection2 **ppCollection);
        
        END_INTERFACE
    } ITTAPI2Vtbl;

    interface ITTAPI2
    {
        CONST_VTBL struct ITTAPI2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITTAPI2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITTAPI2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITTAPI2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITTAPI2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITTAPI2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITTAPI2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITTAPI2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITTAPI2_Initialize(This)	\
    ( (This)->lpVtbl -> Initialize(This) ) 

#define ITTAPI2_Shutdown(This)	\
    ( (This)->lpVtbl -> Shutdown(This) ) 

#define ITTAPI2_get_Addresses(This,pVariant)	\
    ( (This)->lpVtbl -> get_Addresses(This,pVariant) ) 

#define ITTAPI2_EnumerateAddresses(This,ppEnumAddress)	\
    ( (This)->lpVtbl -> EnumerateAddresses(This,ppEnumAddress) ) 

#define ITTAPI2_RegisterCallNotifications(This,pAddress,fMonitor,fOwner,lMediaTypes,lCallbackInstance,plRegister)	\
    ( (This)->lpVtbl -> RegisterCallNotifications(This,pAddress,fMonitor,fOwner,lMediaTypes,lCallbackInstance,plRegister) ) 

#define ITTAPI2_UnregisterNotifications(This,lRegister)	\
    ( (This)->lpVtbl -> UnregisterNotifications(This,lRegister) ) 

#define ITTAPI2_get_CallHubs(This,pVariant)	\
    ( (This)->lpVtbl -> get_CallHubs(This,pVariant) ) 

#define ITTAPI2_EnumerateCallHubs(This,ppEnumCallHub)	\
    ( (This)->lpVtbl -> EnumerateCallHubs(This,ppEnumCallHub) ) 

#define ITTAPI2_SetCallHubTracking(This,pAddresses,bTracking)	\
    ( (This)->lpVtbl -> SetCallHubTracking(This,pAddresses,bTracking) ) 

#define ITTAPI2_EnumeratePrivateTAPIObjects(This,ppEnumUnknown)	\
    ( (This)->lpVtbl -> EnumeratePrivateTAPIObjects(This,ppEnumUnknown) ) 

#define ITTAPI2_get_PrivateTAPIObjects(This,pVariant)	\
    ( (This)->lpVtbl -> get_PrivateTAPIObjects(This,pVariant) ) 

#define ITTAPI2_RegisterRequestRecipient(This,lRegistrationInstance,lRequestMode,fEnable)	\
    ( (This)->lpVtbl -> RegisterRequestRecipient(This,lRegistrationInstance,lRequestMode,fEnable) ) 

#define ITTAPI2_SetAssistedTelephonyPriority(This,pAppFilename,fPriority)	\
    ( (This)->lpVtbl -> SetAssistedTelephonyPriority(This,pAppFilename,fPriority) ) 

#define ITTAPI2_SetApplicationPriority(This,pAppFilename,lMediaType,fPriority)	\
    ( (This)->lpVtbl -> SetApplicationPriority(This,pAppFilename,lMediaType,fPriority) ) 

#define ITTAPI2_put_EventFilter(This,lFilterMask)	\
    ( (This)->lpVtbl -> put_EventFilter(This,lFilterMask) ) 

#define ITTAPI2_get_EventFilter(This,plFilterMask)	\
    ( (This)->lpVtbl -> get_EventFilter(This,plFilterMask) ) 


#define ITTAPI2_get_Phones(This,pPhones)	\
    ( (This)->lpVtbl -> get_Phones(This,pPhones) ) 

#define ITTAPI2_EnumeratePhones(This,ppEnumPhone)	\
    ( (This)->lpVtbl -> EnumeratePhones(This,ppEnumPhone) ) 

#define ITTAPI2_CreateEmptyCollectionObject(This,ppCollection)	\
    ( (This)->lpVtbl -> CreateEmptyCollectionObject(This,ppCollection) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITTAPI2_INTERFACE_DEFINED__ */


#ifndef __ITMediaSupport_INTERFACE_DEFINED__
#define __ITMediaSupport_INTERFACE_DEFINED__

/* interface ITMediaSupport */
/* [object][dual][helpstring][uuid] */ 


EXTERN_C const IID IID_ITMediaSupport;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B1EFC384-9355-11d0-835C-00AA003CCABD")
    ITMediaSupport : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MediaTypes( 
            /* [retval][out] */ __RPC__out long *plMediaTypes) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE QueryMediaType( 
            /* [in] */ long lMediaType,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfSupport) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITMediaSupportVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITMediaSupport * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITMediaSupport * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITMediaSupport * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITMediaSupport * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITMediaSupport * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITMediaSupport * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITMediaSupport * This,
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
        
        DECLSPEC_XFGVIRT(ITMediaSupport, get_MediaTypes)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MediaTypes )( 
            __RPC__in ITMediaSupport * This,
            /* [retval][out] */ __RPC__out long *plMediaTypes);
        
        DECLSPEC_XFGVIRT(ITMediaSupport, QueryMediaType)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *QueryMediaType )( 
            __RPC__in ITMediaSupport * This,
            /* [in] */ long lMediaType,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfSupport);
        
        END_INTERFACE
    } ITMediaSupportVtbl;

    interface ITMediaSupport
    {
        CONST_VTBL struct ITMediaSupportVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITMediaSupport_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITMediaSupport_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITMediaSupport_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITMediaSupport_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITMediaSupport_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITMediaSupport_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITMediaSupport_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITMediaSupport_get_MediaTypes(This,plMediaTypes)	\
    ( (This)->lpVtbl -> get_MediaTypes(This,plMediaTypes) ) 

#define ITMediaSupport_QueryMediaType(This,lMediaType,pfSupport)	\
    ( (This)->lpVtbl -> QueryMediaType(This,lMediaType,pfSupport) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITMediaSupport_INTERFACE_DEFINED__ */


#ifndef __ITPluggableTerminalClassInfo_INTERFACE_DEFINED__
#define __ITPluggableTerminalClassInfo_INTERFACE_DEFINED__

/* interface ITPluggableTerminalClassInfo */
/* [object][dual][helpstring][uuid] */ 


EXTERN_C const IID IID_ITPluggableTerminalClassInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("41757F4A-CF09-4b34-BC96-0A79D2390076")
    ITPluggableTerminalClassInfo : public IDispatch
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pName) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Company( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pCompany) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Version( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVersion) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_TerminalClass( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pTerminalClass) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_CLSID( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pCLSID) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Direction( 
            /* [retval][out] */ __RPC__out TERMINAL_DIRECTION *pDirection) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_MediaTypes( 
            /* [retval][out] */ __RPC__out long *pMediaTypes) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITPluggableTerminalClassInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITPluggableTerminalClassInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITPluggableTerminalClassInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITPluggableTerminalClassInfo * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITPluggableTerminalClassInfo * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITPluggableTerminalClassInfo * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITPluggableTerminalClassInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITPluggableTerminalClassInfo * This,
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
        
        DECLSPEC_XFGVIRT(ITPluggableTerminalClassInfo, get_Name)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in ITPluggableTerminalClassInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pName);
        
        DECLSPEC_XFGVIRT(ITPluggableTerminalClassInfo, get_Company)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Company )( 
            __RPC__in ITPluggableTerminalClassInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pCompany);
        
        DECLSPEC_XFGVIRT(ITPluggableTerminalClassInfo, get_Version)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Version )( 
            __RPC__in ITPluggableTerminalClassInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVersion);
        
        DECLSPEC_XFGVIRT(ITPluggableTerminalClassInfo, get_TerminalClass)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_TerminalClass )( 
            __RPC__in ITPluggableTerminalClassInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pTerminalClass);
        
        DECLSPEC_XFGVIRT(ITPluggableTerminalClassInfo, get_CLSID)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_CLSID )( 
            __RPC__in ITPluggableTerminalClassInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pCLSID);
        
        DECLSPEC_XFGVIRT(ITPluggableTerminalClassInfo, get_Direction)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Direction )( 
            __RPC__in ITPluggableTerminalClassInfo * This,
            /* [retval][out] */ __RPC__out TERMINAL_DIRECTION *pDirection);
        
        DECLSPEC_XFGVIRT(ITPluggableTerminalClassInfo, get_MediaTypes)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_MediaTypes )( 
            __RPC__in ITPluggableTerminalClassInfo * This,
            /* [retval][out] */ __RPC__out long *pMediaTypes);
        
        END_INTERFACE
    } ITPluggableTerminalClassInfoVtbl;

    interface ITPluggableTerminalClassInfo
    {
        CONST_VTBL struct ITPluggableTerminalClassInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITPluggableTerminalClassInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITPluggableTerminalClassInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITPluggableTerminalClassInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITPluggableTerminalClassInfo_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITPluggableTerminalClassInfo_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITPluggableTerminalClassInfo_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITPluggableTerminalClassInfo_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITPluggableTerminalClassInfo_get_Name(This,pName)	\
    ( (This)->lpVtbl -> get_Name(This,pName) ) 

#define ITPluggableTerminalClassInfo_get_Company(This,pCompany)	\
    ( (This)->lpVtbl -> get_Company(This,pCompany) ) 

#define ITPluggableTerminalClassInfo_get_Version(This,pVersion)	\
    ( (This)->lpVtbl -> get_Version(This,pVersion) ) 

#define ITPluggableTerminalClassInfo_get_TerminalClass(This,pTerminalClass)	\
    ( (This)->lpVtbl -> get_TerminalClass(This,pTerminalClass) ) 

#define ITPluggableTerminalClassInfo_get_CLSID(This,pCLSID)	\
    ( (This)->lpVtbl -> get_CLSID(This,pCLSID) ) 

#define ITPluggableTerminalClassInfo_get_Direction(This,pDirection)	\
    ( (This)->lpVtbl -> get_Direction(This,pDirection) ) 

#define ITPluggableTerminalClassInfo_get_MediaTypes(This,pMediaTypes)	\
    ( (This)->lpVtbl -> get_MediaTypes(This,pMediaTypes) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITPluggableTerminalClassInfo_INTERFACE_DEFINED__ */


#ifndef __ITPluggableTerminalSuperclassInfo_INTERFACE_DEFINED__
#define __ITPluggableTerminalSuperclassInfo_INTERFACE_DEFINED__

/* interface ITPluggableTerminalSuperclassInfo */
/* [object][dual][helpstring][uuid] */ 


EXTERN_C const IID IID_ITPluggableTerminalSuperclassInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6D54E42C-4625-4359-A6F7-631999107E05")
    ITPluggableTerminalSuperclassInfo : public IDispatch
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pName) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_CLSID( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pCLSID) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITPluggableTerminalSuperclassInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITPluggableTerminalSuperclassInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITPluggableTerminalSuperclassInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITPluggableTerminalSuperclassInfo * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITPluggableTerminalSuperclassInfo * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITPluggableTerminalSuperclassInfo * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITPluggableTerminalSuperclassInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITPluggableTerminalSuperclassInfo * This,
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
        
        DECLSPEC_XFGVIRT(ITPluggableTerminalSuperclassInfo, get_Name)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in ITPluggableTerminalSuperclassInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pName);
        
        DECLSPEC_XFGVIRT(ITPluggableTerminalSuperclassInfo, get_CLSID)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_CLSID )( 
            __RPC__in ITPluggableTerminalSuperclassInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pCLSID);
        
        END_INTERFACE
    } ITPluggableTerminalSuperclassInfoVtbl;

    interface ITPluggableTerminalSuperclassInfo
    {
        CONST_VTBL struct ITPluggableTerminalSuperclassInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITPluggableTerminalSuperclassInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITPluggableTerminalSuperclassInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITPluggableTerminalSuperclassInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITPluggableTerminalSuperclassInfo_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITPluggableTerminalSuperclassInfo_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITPluggableTerminalSuperclassInfo_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITPluggableTerminalSuperclassInfo_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITPluggableTerminalSuperclassInfo_get_Name(This,pName)	\
    ( (This)->lpVtbl -> get_Name(This,pName) ) 

#define ITPluggableTerminalSuperclassInfo_get_CLSID(This,pCLSID)	\
    ( (This)->lpVtbl -> get_CLSID(This,pCLSID) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITPluggableTerminalSuperclassInfo_INTERFACE_DEFINED__ */


#ifndef __ITTerminalSupport_INTERFACE_DEFINED__
#define __ITTerminalSupport_INTERFACE_DEFINED__

/* interface ITTerminalSupport */
/* [object][dual][helpstring][uuid] */ 


EXTERN_C const IID IID_ITTerminalSupport;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B1EFC385-9355-11d0-835C-00AA003CCABD")
    ITTerminalSupport : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_StaticTerminals( 
            /* [retval][out] */ __RPC__out VARIANT *pVariant) = 0;
        
        virtual /* [helpstring][hidden][id] */ HRESULT STDMETHODCALLTYPE EnumerateStaticTerminals( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumTerminal **ppTerminalEnumerator) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DynamicTerminalClasses( 
            /* [retval][out] */ __RPC__out VARIANT *pVariant) = 0;
        
        virtual /* [helpstring][hidden][id] */ HRESULT STDMETHODCALLTYPE EnumerateDynamicTerminalClasses( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumTerminalClass **ppTerminalClassEnumerator) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateTerminal( 
            /* [in] */ __RPC__in BSTR pTerminalClass,
            /* [in] */ long lMediaType,
            /* [in] */ TERMINAL_DIRECTION Direction,
            /* [retval][out] */ __RPC__deref_out_opt ITTerminal **ppTerminal) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetDefaultStaticTerminal( 
            /* [in] */ long lMediaType,
            /* [in] */ TERMINAL_DIRECTION Direction,
            /* [retval][out] */ __RPC__deref_out_opt ITTerminal **ppTerminal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITTerminalSupportVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITTerminalSupport * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITTerminalSupport * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITTerminalSupport * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITTerminalSupport * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITTerminalSupport * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITTerminalSupport * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITTerminalSupport * This,
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
        
        DECLSPEC_XFGVIRT(ITTerminalSupport, get_StaticTerminals)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StaticTerminals )( 
            __RPC__in ITTerminalSupport * This,
            /* [retval][out] */ __RPC__out VARIANT *pVariant);
        
        DECLSPEC_XFGVIRT(ITTerminalSupport, EnumerateStaticTerminals)
        /* [helpstring][hidden][id] */ HRESULT ( STDMETHODCALLTYPE *EnumerateStaticTerminals )( 
            __RPC__in ITTerminalSupport * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumTerminal **ppTerminalEnumerator);
        
        DECLSPEC_XFGVIRT(ITTerminalSupport, get_DynamicTerminalClasses)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DynamicTerminalClasses )( 
            __RPC__in ITTerminalSupport * This,
            /* [retval][out] */ __RPC__out VARIANT *pVariant);
        
        DECLSPEC_XFGVIRT(ITTerminalSupport, EnumerateDynamicTerminalClasses)
        /* [helpstring][hidden][id] */ HRESULT ( STDMETHODCALLTYPE *EnumerateDynamicTerminalClasses )( 
            __RPC__in ITTerminalSupport * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumTerminalClass **ppTerminalClassEnumerator);
        
        DECLSPEC_XFGVIRT(ITTerminalSupport, CreateTerminal)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateTerminal )( 
            __RPC__in ITTerminalSupport * This,
            /* [in] */ __RPC__in BSTR pTerminalClass,
            /* [in] */ long lMediaType,
            /* [in] */ TERMINAL_DIRECTION Direction,
            /* [retval][out] */ __RPC__deref_out_opt ITTerminal **ppTerminal);
        
        DECLSPEC_XFGVIRT(ITTerminalSupport, GetDefaultStaticTerminal)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetDefaultStaticTerminal )( 
            __RPC__in ITTerminalSupport * This,
            /* [in] */ long lMediaType,
            /* [in] */ TERMINAL_DIRECTION Direction,
            /* [retval][out] */ __RPC__deref_out_opt ITTerminal **ppTerminal);
        
        END_INTERFACE
    } ITTerminalSupportVtbl;

    interface ITTerminalSupport
    {
        CONST_VTBL struct ITTerminalSupportVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITTerminalSupport_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITTerminalSupport_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITTerminalSupport_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITTerminalSupport_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITTerminalSupport_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITTerminalSupport_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITTerminalSupport_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITTerminalSupport_get_StaticTerminals(This,pVariant)	\
    ( (This)->lpVtbl -> get_StaticTerminals(This,pVariant) ) 

#define ITTerminalSupport_EnumerateStaticTerminals(This,ppTerminalEnumerator)	\
    ( (This)->lpVtbl -> EnumerateStaticTerminals(This,ppTerminalEnumerator) ) 

#define ITTerminalSupport_get_DynamicTerminalClasses(This,pVariant)	\
    ( (This)->lpVtbl -> get_DynamicTerminalClasses(This,pVariant) ) 

#define ITTerminalSupport_EnumerateDynamicTerminalClasses(This,ppTerminalClassEnumerator)	\
    ( (This)->lpVtbl -> EnumerateDynamicTerminalClasses(This,ppTerminalClassEnumerator) ) 

#define ITTerminalSupport_CreateTerminal(This,pTerminalClass,lMediaType,Direction,ppTerminal)	\
    ( (This)->lpVtbl -> CreateTerminal(This,pTerminalClass,lMediaType,Direction,ppTerminal) ) 

#define ITTerminalSupport_GetDefaultStaticTerminal(This,lMediaType,Direction,ppTerminal)	\
    ( (This)->lpVtbl -> GetDefaultStaticTerminal(This,lMediaType,Direction,ppTerminal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITTerminalSupport_INTERFACE_DEFINED__ */


#ifndef __ITTerminalSupport2_INTERFACE_DEFINED__
#define __ITTerminalSupport2_INTERFACE_DEFINED__

/* interface ITTerminalSupport2 */
/* [object][dual][helpstring][uuid] */ 


EXTERN_C const IID IID_ITTerminalSupport2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F3EB39BC-1B1F-4e99-A0C0-56305C4DD591")
    ITTerminalSupport2 : public ITTerminalSupport
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PluggableSuperclasses( 
            /* [retval][out] */ __RPC__out VARIANT *pVariant) = 0;
        
        virtual /* [helpstring][hidden][id] */ HRESULT STDMETHODCALLTYPE EnumeratePluggableSuperclasses( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumPluggableSuperclassInfo **ppSuperclassEnumerator) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PluggableTerminalClasses( 
            /* [in] */ __RPC__in BSTR bstrTerminalSuperclass,
            /* [in] */ long lMediaType,
            /* [retval][out] */ __RPC__out VARIANT *pVariant) = 0;
        
        virtual /* [helpstring][hidden][id] */ HRESULT STDMETHODCALLTYPE EnumeratePluggableTerminalClasses( 
            /* [in] */ CLSID iidTerminalSuperclass,
            /* [in] */ long lMediaType,
            /* [retval][out] */ __RPC__deref_out_opt IEnumPluggableTerminalClassInfo **ppClassEnumerator) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITTerminalSupport2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITTerminalSupport2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITTerminalSupport2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITTerminalSupport2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITTerminalSupport2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITTerminalSupport2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITTerminalSupport2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITTerminalSupport2 * This,
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
        
        DECLSPEC_XFGVIRT(ITTerminalSupport, get_StaticTerminals)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StaticTerminals )( 
            __RPC__in ITTerminalSupport2 * This,
            /* [retval][out] */ __RPC__out VARIANT *pVariant);
        
        DECLSPEC_XFGVIRT(ITTerminalSupport, EnumerateStaticTerminals)
        /* [helpstring][hidden][id] */ HRESULT ( STDMETHODCALLTYPE *EnumerateStaticTerminals )( 
            __RPC__in ITTerminalSupport2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumTerminal **ppTerminalEnumerator);
        
        DECLSPEC_XFGVIRT(ITTerminalSupport, get_DynamicTerminalClasses)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DynamicTerminalClasses )( 
            __RPC__in ITTerminalSupport2 * This,
            /* [retval][out] */ __RPC__out VARIANT *pVariant);
        
        DECLSPEC_XFGVIRT(ITTerminalSupport, EnumerateDynamicTerminalClasses)
        /* [helpstring][hidden][id] */ HRESULT ( STDMETHODCALLTYPE *EnumerateDynamicTerminalClasses )( 
            __RPC__in ITTerminalSupport2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumTerminalClass **ppTerminalClassEnumerator);
        
        DECLSPEC_XFGVIRT(ITTerminalSupport, CreateTerminal)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateTerminal )( 
            __RPC__in ITTerminalSupport2 * This,
            /* [in] */ __RPC__in BSTR pTerminalClass,
            /* [in] */ long lMediaType,
            /* [in] */ TERMINAL_DIRECTION Direction,
            /* [retval][out] */ __RPC__deref_out_opt ITTerminal **ppTerminal);
        
        DECLSPEC_XFGVIRT(ITTerminalSupport, GetDefaultStaticTerminal)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetDefaultStaticTerminal )( 
            __RPC__in ITTerminalSupport2 * This,
            /* [in] */ long lMediaType,
            /* [in] */ TERMINAL_DIRECTION Direction,
            /* [retval][out] */ __RPC__deref_out_opt ITTerminal **ppTerminal);
        
        DECLSPEC_XFGVIRT(ITTerminalSupport2, get_PluggableSuperclasses)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PluggableSuperclasses )( 
            __RPC__in ITTerminalSupport2 * This,
            /* [retval][out] */ __RPC__out VARIANT *pVariant);
        
        DECLSPEC_XFGVIRT(ITTerminalSupport2, EnumeratePluggableSuperclasses)
        /* [helpstring][hidden][id] */ HRESULT ( STDMETHODCALLTYPE *EnumeratePluggableSuperclasses )( 
            __RPC__in ITTerminalSupport2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumPluggableSuperclassInfo **ppSuperclassEnumerator);
        
        DECLSPEC_XFGVIRT(ITTerminalSupport2, get_PluggableTerminalClasses)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PluggableTerminalClasses )( 
            __RPC__in ITTerminalSupport2 * This,
            /* [in] */ __RPC__in BSTR bstrTerminalSuperclass,
            /* [in] */ long lMediaType,
            /* [retval][out] */ __RPC__out VARIANT *pVariant);
        
        DECLSPEC_XFGVIRT(ITTerminalSupport2, EnumeratePluggableTerminalClasses)
        /* [helpstring][hidden][id] */ HRESULT ( STDMETHODCALLTYPE *EnumeratePluggableTerminalClasses )( 
            __RPC__in ITTerminalSupport2 * This,
            /* [in] */ CLSID iidTerminalSuperclass,
            /* [in] */ long lMediaType,
            /* [retval][out] */ __RPC__deref_out_opt IEnumPluggableTerminalClassInfo **ppClassEnumerator);
        
        END_INTERFACE
    } ITTerminalSupport2Vtbl;

    interface ITTerminalSupport2
    {
        CONST_VTBL struct ITTerminalSupport2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITTerminalSupport2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITTerminalSupport2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITTerminalSupport2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITTerminalSupport2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITTerminalSupport2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITTerminalSupport2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITTerminalSupport2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITTerminalSupport2_get_StaticTerminals(This,pVariant)	\
    ( (This)->lpVtbl -> get_StaticTerminals(This,pVariant) ) 

#define ITTerminalSupport2_EnumerateStaticTerminals(This,ppTerminalEnumerator)	\
    ( (This)->lpVtbl -> EnumerateStaticTerminals(This,ppTerminalEnumerator) ) 

#define ITTerminalSupport2_get_DynamicTerminalClasses(This,pVariant)	\
    ( (This)->lpVtbl -> get_DynamicTerminalClasses(This,pVariant) ) 

#define ITTerminalSupport2_EnumerateDynamicTerminalClasses(This,ppTerminalClassEnumerator)	\
    ( (This)->lpVtbl -> EnumerateDynamicTerminalClasses(This,ppTerminalClassEnumerator) ) 

#define ITTerminalSupport2_CreateTerminal(This,pTerminalClass,lMediaType,Direction,ppTerminal)	\
    ( (This)->lpVtbl -> CreateTerminal(This,pTerminalClass,lMediaType,Direction,ppTerminal) ) 

#define ITTerminalSupport2_GetDefaultStaticTerminal(This,lMediaType,Direction,ppTerminal)	\
    ( (This)->lpVtbl -> GetDefaultStaticTerminal(This,lMediaType,Direction,ppTerminal) ) 


#define ITTerminalSupport2_get_PluggableSuperclasses(This,pVariant)	\
    ( (This)->lpVtbl -> get_PluggableSuperclasses(This,pVariant) ) 

#define ITTerminalSupport2_EnumeratePluggableSuperclasses(This,ppSuperclassEnumerator)	\
    ( (This)->lpVtbl -> EnumeratePluggableSuperclasses(This,ppSuperclassEnumerator) ) 

#define ITTerminalSupport2_get_PluggableTerminalClasses(This,bstrTerminalSuperclass,lMediaType,pVariant)	\
    ( (This)->lpVtbl -> get_PluggableTerminalClasses(This,bstrTerminalSuperclass,lMediaType,pVariant) ) 

#define ITTerminalSupport2_EnumeratePluggableTerminalClasses(This,iidTerminalSuperclass,lMediaType,ppClassEnumerator)	\
    ( (This)->lpVtbl -> EnumeratePluggableTerminalClasses(This,iidTerminalSuperclass,lMediaType,ppClassEnumerator) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITTerminalSupport2_INTERFACE_DEFINED__ */


#ifndef __ITAddress_INTERFACE_DEFINED__
#define __ITAddress_INTERFACE_DEFINED__

/* interface ITAddress */
/* [object][dual][helpstring][uuid] */ 


EXTERN_C const IID IID_ITAddress;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B1EFC386-9355-11d0-835C-00AA003CCABD")
    ITAddress : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_State( 
            /* [retval][out] */ __RPC__out ADDRESS_STATE *pAddressState) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_AddressName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ServiceProviderName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TAPIObject( 
            /* [retval][out] */ __RPC__deref_out_opt ITTAPI **ppTapiObject) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateCall( 
            /* [in] */ __RPC__in BSTR pDestAddress,
            /* [in] */ long lAddressType,
            /* [in] */ long lMediaTypes,
            /* [retval][out] */ __RPC__deref_out_opt ITBasicCallControl **ppCall) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Calls( 
            /* [retval][out] */ __RPC__out VARIANT *pVariant) = 0;
        
        virtual /* [helpstring][hidden][id] */ HRESULT STDMETHODCALLTYPE EnumerateCalls( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumCall **ppCallEnum) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DialableAddress( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pDialableAddress) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateForwardInfoObject( 
            /* [retval][out] */ __RPC__deref_out_opt ITForwardInformation **ppForwardInfo) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Forward( 
            /* [in] */ __RPC__in_opt ITForwardInformation *pForwardInfo,
            /* [in] */ __RPC__in_opt ITBasicCallControl *pCall) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CurrentForwardInfo( 
            /* [retval][out] */ __RPC__deref_out_opt ITForwardInformation **ppForwardInfo) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_MessageWaiting( 
            /* [in] */ VARIANT_BOOL fMessageWaiting) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MessageWaiting( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfMessageWaiting) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_DoNotDisturb( 
            /* [in] */ VARIANT_BOOL fDoNotDisturb) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DoNotDisturb( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfDoNotDisturb) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITAddressVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITAddress * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITAddress * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITAddress * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITAddress * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITAddress * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITAddress * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITAddress * This,
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
        
        DECLSPEC_XFGVIRT(ITAddress, get_State)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_State )( 
            __RPC__in ITAddress * This,
            /* [retval][out] */ __RPC__out ADDRESS_STATE *pAddressState);
        
        DECLSPEC_XFGVIRT(ITAddress, get_AddressName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AddressName )( 
            __RPC__in ITAddress * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppName);
        
        DECLSPEC_XFGVIRT(ITAddress, get_ServiceProviderName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ServiceProviderName )( 
            __RPC__in ITAddress * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppName);
        
        DECLSPEC_XFGVIRT(ITAddress, get_TAPIObject)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TAPIObject )( 
            __RPC__in ITAddress * This,
            /* [retval][out] */ __RPC__deref_out_opt ITTAPI **ppTapiObject);
        
        DECLSPEC_XFGVIRT(ITAddress, CreateCall)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateCall )( 
            __RPC__in ITAddress * This,
            /* [in] */ __RPC__in BSTR pDestAddress,
            /* [in] */ long lAddressType,
            /* [in] */ long lMediaTypes,
            /* [retval][out] */ __RPC__deref_out_opt ITBasicCallControl **ppCall);
        
        DECLSPEC_XFGVIRT(ITAddress, get_Calls)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Calls )( 
            __RPC__in ITAddress * This,
            /* [retval][out] */ __RPC__out VARIANT *pVariant);
        
        DECLSPEC_XFGVIRT(ITAddress, EnumerateCalls)
        /* [helpstring][hidden][id] */ HRESULT ( STDMETHODCALLTYPE *EnumerateCalls )( 
            __RPC__in ITAddress * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumCall **ppCallEnum);
        
        DECLSPEC_XFGVIRT(ITAddress, get_DialableAddress)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DialableAddress )( 
            __RPC__in ITAddress * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pDialableAddress);
        
        DECLSPEC_XFGVIRT(ITAddress, CreateForwardInfoObject)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateForwardInfoObject )( 
            __RPC__in ITAddress * This,
            /* [retval][out] */ __RPC__deref_out_opt ITForwardInformation **ppForwardInfo);
        
        DECLSPEC_XFGVIRT(ITAddress, Forward)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Forward )( 
            __RPC__in ITAddress * This,
            /* [in] */ __RPC__in_opt ITForwardInformation *pForwardInfo,
            /* [in] */ __RPC__in_opt ITBasicCallControl *pCall);
        
        DECLSPEC_XFGVIRT(ITAddress, get_CurrentForwardInfo)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CurrentForwardInfo )( 
            __RPC__in ITAddress * This,
            /* [retval][out] */ __RPC__deref_out_opt ITForwardInformation **ppForwardInfo);
        
        DECLSPEC_XFGVIRT(ITAddress, put_MessageWaiting)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MessageWaiting )( 
            __RPC__in ITAddress * This,
            /* [in] */ VARIANT_BOOL fMessageWaiting);
        
        DECLSPEC_XFGVIRT(ITAddress, get_MessageWaiting)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MessageWaiting )( 
            __RPC__in ITAddress * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfMessageWaiting);
        
        DECLSPEC_XFGVIRT(ITAddress, put_DoNotDisturb)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DoNotDisturb )( 
            __RPC__in ITAddress * This,
            /* [in] */ VARIANT_BOOL fDoNotDisturb);
        
        DECLSPEC_XFGVIRT(ITAddress, get_DoNotDisturb)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DoNotDisturb )( 
            __RPC__in ITAddress * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfDoNotDisturb);
        
        END_INTERFACE
    } ITAddressVtbl;

    interface ITAddress
    {
        CONST_VTBL struct ITAddressVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITAddress_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITAddress_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITAddress_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITAddress_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITAddress_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITAddress_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITAddress_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITAddress_get_State(This,pAddressState)	\
    ( (This)->lpVtbl -> get_State(This,pAddressState) ) 

#define ITAddress_get_AddressName(This,ppName)	\
    ( (This)->lpVtbl -> get_AddressName(This,ppName) ) 

#define ITAddress_get_ServiceProviderName(This,ppName)	\
    ( (This)->lpVtbl -> get_ServiceProviderName(This,ppName) ) 

#define ITAddress_get_TAPIObject(This,ppTapiObject)	\
    ( (This)->lpVtbl -> get_TAPIObject(This,ppTapiObject) ) 

#define ITAddress_CreateCall(This,pDestAddress,lAddressType,lMediaTypes,ppCall)	\
    ( (This)->lpVtbl -> CreateCall(This,pDestAddress,lAddressType,lMediaTypes,ppCall) ) 

#define ITAddress_get_Calls(This,pVariant)	\
    ( (This)->lpVtbl -> get_Calls(This,pVariant) ) 

#define ITAddress_EnumerateCalls(This,ppCallEnum)	\
    ( (This)->lpVtbl -> EnumerateCalls(This,ppCallEnum) ) 

#define ITAddress_get_DialableAddress(This,pDialableAddress)	\
    ( (This)->lpVtbl -> get_DialableAddress(This,pDialableAddress) ) 

#define ITAddress_CreateForwardInfoObject(This,ppForwardInfo)	\
    ( (This)->lpVtbl -> CreateForwardInfoObject(This,ppForwardInfo) ) 

#define ITAddress_Forward(This,pForwardInfo,pCall)	\
    ( (This)->lpVtbl -> Forward(This,pForwardInfo,pCall) ) 

#define ITAddress_get_CurrentForwardInfo(This,ppForwardInfo)	\
    ( (This)->lpVtbl -> get_CurrentForwardInfo(This,ppForwardInfo) ) 

#define ITAddress_put_MessageWaiting(This,fMessageWaiting)	\
    ( (This)->lpVtbl -> put_MessageWaiting(This,fMessageWaiting) ) 

#define ITAddress_get_MessageWaiting(This,pfMessageWaiting)	\
    ( (This)->lpVtbl -> get_MessageWaiting(This,pfMessageWaiting) ) 

#define ITAddress_put_DoNotDisturb(This,fDoNotDisturb)	\
    ( (This)->lpVtbl -> put_DoNotDisturb(This,fDoNotDisturb) ) 

#define ITAddress_get_DoNotDisturb(This,pfDoNotDisturb)	\
    ( (This)->lpVtbl -> get_DoNotDisturb(This,pfDoNotDisturb) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITAddress_INTERFACE_DEFINED__ */


#ifndef __ITAddress2_INTERFACE_DEFINED__
#define __ITAddress2_INTERFACE_DEFINED__

/* interface ITAddress2 */
/* [object][dual][helpstring][uuid] */ 


EXTERN_C const IID IID_ITAddress2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B0AE5D9B-BE51-46c9-B0F7-DFA8A22A8BC4")
    ITAddress2 : public ITAddress
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Phones( 
            /* [retval][out] */ __RPC__out VARIANT *pPhones) = 0;
        
        virtual /* [helpstring][hidden][id] */ HRESULT STDMETHODCALLTYPE EnumeratePhones( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumPhone **ppEnumPhone) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetPhoneFromTerminal( 
            /* [in] */ __RPC__in_opt ITTerminal *pTerminal,
            /* [retval][out] */ __RPC__deref_out_opt ITPhone **ppPhone) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PreferredPhones( 
            /* [retval][out] */ __RPC__out VARIANT *pPhones) = 0;
        
        virtual /* [helpstring][hidden][id] */ HRESULT STDMETHODCALLTYPE EnumeratePreferredPhones( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumPhone **ppEnumPhone) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_EventFilter( 
            /* [in] */ TAPI_EVENT TapiEvent,
            /* [in] */ long lSubEvent,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pEnable) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_EventFilter( 
            /* [in] */ TAPI_EVENT TapiEvent,
            /* [in] */ long lSubEvent,
            /* [in] */ VARIANT_BOOL bEnable) = 0;
        
        virtual /* [helpstring][hidden][id] */ HRESULT STDMETHODCALLTYPE DeviceSpecific( 
            /* [in] */ __RPC__in_opt ITCallInfo *pCall,
            /* [in] */ __RPC__in BYTE *pParams,
            /* [in] */ DWORD dwSize) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE DeviceSpecificVariant( 
            /* [in] */ __RPC__in_opt ITCallInfo *pCall,
            /* [in] */ VARIANT varDevSpecificByteArray) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE NegotiateExtVersion( 
            /* [in] */ long lLowVersion,
            /* [in] */ long lHighVersion,
            /* [retval][out] */ __RPC__out long *plExtVersion) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITAddress2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITAddress2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITAddress2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITAddress2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITAddress2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITAddress2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITAddress2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITAddress2 * This,
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
        
        DECLSPEC_XFGVIRT(ITAddress, get_State)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_State )( 
            __RPC__in ITAddress2 * This,
            /* [retval][out] */ __RPC__out ADDRESS_STATE *pAddressState);
        
        DECLSPEC_XFGVIRT(ITAddress, get_AddressName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AddressName )( 
            __RPC__in ITAddress2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppName);
        
        DECLSPEC_XFGVIRT(ITAddress, get_ServiceProviderName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ServiceProviderName )( 
            __RPC__in ITAddress2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppName);
        
        DECLSPEC_XFGVIRT(ITAddress, get_TAPIObject)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TAPIObject )( 
            __RPC__in ITAddress2 * This,
            /* [retval][out] */ __RPC__deref_out_opt ITTAPI **ppTapiObject);
        
        DECLSPEC_XFGVIRT(ITAddress, CreateCall)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateCall )( 
            __RPC__in ITAddress2 * This,
            /* [in] */ __RPC__in BSTR pDestAddress,
            /* [in] */ long lAddressType,
            /* [in] */ long lMediaTypes,
            /* [retval][out] */ __RPC__deref_out_opt ITBasicCallControl **ppCall);
        
        DECLSPEC_XFGVIRT(ITAddress, get_Calls)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Calls )( 
            __RPC__in ITAddress2 * This,
            /* [retval][out] */ __RPC__out VARIANT *pVariant);
        
        DECLSPEC_XFGVIRT(ITAddress, EnumerateCalls)
        /* [helpstring][hidden][id] */ HRESULT ( STDMETHODCALLTYPE *EnumerateCalls )( 
            __RPC__in ITAddress2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumCall **ppCallEnum);
        
        DECLSPEC_XFGVIRT(ITAddress, get_DialableAddress)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DialableAddress )( 
            __RPC__in ITAddress2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pDialableAddress);
        
        DECLSPEC_XFGVIRT(ITAddress, CreateForwardInfoObject)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateForwardInfoObject )( 
            __RPC__in ITAddress2 * This,
            /* [retval][out] */ __RPC__deref_out_opt ITForwardInformation **ppForwardInfo);
        
        DECLSPEC_XFGVIRT(ITAddress, Forward)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Forward )( 
            __RPC__in ITAddress2 * This,
            /* [in] */ __RPC__in_opt ITForwardInformation *pForwardInfo,
            /* [in] */ __RPC__in_opt ITBasicCallControl *pCall);
        
        DECLSPEC_XFGVIRT(ITAddress, get_CurrentForwardInfo)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CurrentForwardInfo )( 
            __RPC__in ITAddress2 * This,
            /* [retval][out] */ __RPC__deref_out_opt ITForwardInformation **ppForwardInfo);
        
        DECLSPEC_XFGVIRT(ITAddress, put_MessageWaiting)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MessageWaiting )( 
            __RPC__in ITAddress2 * This,
            /* [in] */ VARIANT_BOOL fMessageWaiting);
        
        DECLSPEC_XFGVIRT(ITAddress, get_MessageWaiting)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MessageWaiting )( 
            __RPC__in ITAddress2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfMessageWaiting);
        
        DECLSPEC_XFGVIRT(ITAddress, put_DoNotDisturb)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DoNotDisturb )( 
            __RPC__in ITAddress2 * This,
            /* [in] */ VARIANT_BOOL fDoNotDisturb);
        
        DECLSPEC_XFGVIRT(ITAddress, get_DoNotDisturb)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DoNotDisturb )( 
            __RPC__in ITAddress2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfDoNotDisturb);
        
        DECLSPEC_XFGVIRT(ITAddress2, get_Phones)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Phones )( 
            __RPC__in ITAddress2 * This,
            /* [retval][out] */ __RPC__out VARIANT *pPhones);
        
        DECLSPEC_XFGVIRT(ITAddress2, EnumeratePhones)
        /* [helpstring][hidden][id] */ HRESULT ( STDMETHODCALLTYPE *EnumeratePhones )( 
            __RPC__in ITAddress2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumPhone **ppEnumPhone);
        
        DECLSPEC_XFGVIRT(ITAddress2, GetPhoneFromTerminal)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetPhoneFromTerminal )( 
            __RPC__in ITAddress2 * This,
            /* [in] */ __RPC__in_opt ITTerminal *pTerminal,
            /* [retval][out] */ __RPC__deref_out_opt ITPhone **ppPhone);
        
        DECLSPEC_XFGVIRT(ITAddress2, get_PreferredPhones)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PreferredPhones )( 
            __RPC__in ITAddress2 * This,
            /* [retval][out] */ __RPC__out VARIANT *pPhones);
        
        DECLSPEC_XFGVIRT(ITAddress2, EnumeratePreferredPhones)
        /* [helpstring][hidden][id] */ HRESULT ( STDMETHODCALLTYPE *EnumeratePreferredPhones )( 
            __RPC__in ITAddress2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumPhone **ppEnumPhone);
        
        DECLSPEC_XFGVIRT(ITAddress2, get_EventFilter)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EventFilter )( 
            __RPC__in ITAddress2 * This,
            /* [in] */ TAPI_EVENT TapiEvent,
            /* [in] */ long lSubEvent,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pEnable);
        
        DECLSPEC_XFGVIRT(ITAddress2, put_EventFilter)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_EventFilter )( 
            __RPC__in ITAddress2 * This,
            /* [in] */ TAPI_EVENT TapiEvent,
            /* [in] */ long lSubEvent,
            /* [in] */ VARIANT_BOOL bEnable);
        
        DECLSPEC_XFGVIRT(ITAddress2, DeviceSpecific)
        /* [helpstring][hidden][id] */ HRESULT ( STDMETHODCALLTYPE *DeviceSpecific )( 
            __RPC__in ITAddress2 * This,
            /* [in] */ __RPC__in_opt ITCallInfo *pCall,
            /* [in] */ __RPC__in BYTE *pParams,
            /* [in] */ DWORD dwSize);
        
        DECLSPEC_XFGVIRT(ITAddress2, DeviceSpecificVariant)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DeviceSpecificVariant )( 
            __RPC__in ITAddress2 * This,
            /* [in] */ __RPC__in_opt ITCallInfo *pCall,
            /* [in] */ VARIANT varDevSpecificByteArray);
        
        DECLSPEC_XFGVIRT(ITAddress2, NegotiateExtVersion)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *NegotiateExtVersion )( 
            __RPC__in ITAddress2 * This,
            /* [in] */ long lLowVersion,
            /* [in] */ long lHighVersion,
            /* [retval][out] */ __RPC__out long *plExtVersion);
        
        END_INTERFACE
    } ITAddress2Vtbl;

    interface ITAddress2
    {
        CONST_VTBL struct ITAddress2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITAddress2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITAddress2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITAddress2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITAddress2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITAddress2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITAddress2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITAddress2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITAddress2_get_State(This,pAddressState)	\
    ( (This)->lpVtbl -> get_State(This,pAddressState) ) 

#define ITAddress2_get_AddressName(This,ppName)	\
    ( (This)->lpVtbl -> get_AddressName(This,ppName) ) 

#define ITAddress2_get_ServiceProviderName(This,ppName)	\
    ( (This)->lpVtbl -> get_ServiceProviderName(This,ppName) ) 

#define ITAddress2_get_TAPIObject(This,ppTapiObject)	\
    ( (This)->lpVtbl -> get_TAPIObject(This,ppTapiObject) ) 

#define ITAddress2_CreateCall(This,pDestAddress,lAddressType,lMediaTypes,ppCall)	\
    ( (This)->lpVtbl -> CreateCall(This,pDestAddress,lAddressType,lMediaTypes,ppCall) ) 

#define ITAddress2_get_Calls(This,pVariant)	\
    ( (This)->lpVtbl -> get_Calls(This,pVariant) ) 

#define ITAddress2_EnumerateCalls(This,ppCallEnum)	\
    ( (This)->lpVtbl -> EnumerateCalls(This,ppCallEnum) ) 

#define ITAddress2_get_DialableAddress(This,pDialableAddress)	\
    ( (This)->lpVtbl -> get_DialableAddress(This,pDialableAddress) ) 

#define ITAddress2_CreateForwardInfoObject(This,ppForwardInfo)	\
    ( (This)->lpVtbl -> CreateForwardInfoObject(This,ppForwardInfo) ) 

#define ITAddress2_Forward(This,pForwardInfo,pCall)	\
    ( (This)->lpVtbl -> Forward(This,pForwardInfo,pCall) ) 

#define ITAddress2_get_CurrentForwardInfo(This,ppForwardInfo)	\
    ( (This)->lpVtbl -> get_CurrentForwardInfo(This,ppForwardInfo) ) 

#define ITAddress2_put_MessageWaiting(This,fMessageWaiting)	\
    ( (This)->lpVtbl -> put_MessageWaiting(This,fMessageWaiting) ) 

#define ITAddress2_get_MessageWaiting(This,pfMessageWaiting)	\
    ( (This)->lpVtbl -> get_MessageWaiting(This,pfMessageWaiting) ) 

#define ITAddress2_put_DoNotDisturb(This,fDoNotDisturb)	\
    ( (This)->lpVtbl -> put_DoNotDisturb(This,fDoNotDisturb) ) 

#define ITAddress2_get_DoNotDisturb(This,pfDoNotDisturb)	\
    ( (This)->lpVtbl -> get_DoNotDisturb(This,pfDoNotDisturb) ) 


#define ITAddress2_get_Phones(This,pPhones)	\
    ( (This)->lpVtbl -> get_Phones(This,pPhones) ) 

#define ITAddress2_EnumeratePhones(This,ppEnumPhone)	\
    ( (This)->lpVtbl -> EnumeratePhones(This,ppEnumPhone) ) 

#define ITAddress2_GetPhoneFromTerminal(This,pTerminal,ppPhone)	\
    ( (This)->lpVtbl -> GetPhoneFromTerminal(This,pTerminal,ppPhone) ) 

#define ITAddress2_get_PreferredPhones(This,pPhones)	\
    ( (This)->lpVtbl -> get_PreferredPhones(This,pPhones) ) 

#define ITAddress2_EnumeratePreferredPhones(This,ppEnumPhone)	\
    ( (This)->lpVtbl -> EnumeratePreferredPhones(This,ppEnumPhone) ) 

#define ITAddress2_get_EventFilter(This,TapiEvent,lSubEvent,pEnable)	\
    ( (This)->lpVtbl -> get_EventFilter(This,TapiEvent,lSubEvent,pEnable) ) 

#define ITAddress2_put_EventFilter(This,TapiEvent,lSubEvent,bEnable)	\
    ( (This)->lpVtbl -> put_EventFilter(This,TapiEvent,lSubEvent,bEnable) ) 

#define ITAddress2_DeviceSpecific(This,pCall,pParams,dwSize)	\
    ( (This)->lpVtbl -> DeviceSpecific(This,pCall,pParams,dwSize) ) 

#define ITAddress2_DeviceSpecificVariant(This,pCall,varDevSpecificByteArray)	\
    ( (This)->lpVtbl -> DeviceSpecificVariant(This,pCall,varDevSpecificByteArray) ) 

#define ITAddress2_NegotiateExtVersion(This,lLowVersion,lHighVersion,plExtVersion)	\
    ( (This)->lpVtbl -> NegotiateExtVersion(This,lLowVersion,lHighVersion,plExtVersion) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITAddress2_INTERFACE_DEFINED__ */


#ifndef __ITAddressCapabilities_INTERFACE_DEFINED__
#define __ITAddressCapabilities_INTERFACE_DEFINED__

/* interface ITAddressCapabilities */
/* [object][dual][helpstring][uuid] */ 


EXTERN_C const IID IID_ITAddressCapabilities;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8DF232F5-821B-11d1-BB5C-00C04FB6809F")
    ITAddressCapabilities : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_AddressCapability( 
            /* [in] */ ADDRESS_CAPABILITY AddressCap,
            /* [retval][out] */ __RPC__out long *plCapability) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_AddressCapabilityString( 
            /* [in] */ ADDRESS_CAPABILITY_STRING AddressCapString,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppCapabilityString) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CallTreatments( 
            /* [retval][out] */ __RPC__out VARIANT *pVariant) = 0;
        
        virtual /* [hidden][id] */ HRESULT STDMETHODCALLTYPE EnumerateCallTreatments( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumBstr **ppEnumCallTreatment) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CompletionMessages( 
            /* [retval][out] */ __RPC__out VARIANT *pVariant) = 0;
        
        virtual /* [hidden][id] */ HRESULT STDMETHODCALLTYPE EnumerateCompletionMessages( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumBstr **ppEnumCompletionMessage) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DeviceClasses( 
            /* [retval][out] */ __RPC__out VARIANT *pVariant) = 0;
        
        virtual /* [hidden][id] */ HRESULT STDMETHODCALLTYPE EnumerateDeviceClasses( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumBstr **ppEnumDeviceClass) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITAddressCapabilitiesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITAddressCapabilities * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITAddressCapabilities * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITAddressCapabilities * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITAddressCapabilities * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITAddressCapabilities * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITAddressCapabilities * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITAddressCapabilities * This,
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
        
        DECLSPEC_XFGVIRT(ITAddressCapabilities, get_AddressCapability)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AddressCapability )( 
            __RPC__in ITAddressCapabilities * This,
            /* [in] */ ADDRESS_CAPABILITY AddressCap,
            /* [retval][out] */ __RPC__out long *plCapability);
        
        DECLSPEC_XFGVIRT(ITAddressCapabilities, get_AddressCapabilityString)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AddressCapabilityString )( 
            __RPC__in ITAddressCapabilities * This,
            /* [in] */ ADDRESS_CAPABILITY_STRING AddressCapString,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppCapabilityString);
        
        DECLSPEC_XFGVIRT(ITAddressCapabilities, get_CallTreatments)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CallTreatments )( 
            __RPC__in ITAddressCapabilities * This,
            /* [retval][out] */ __RPC__out VARIANT *pVariant);
        
        DECLSPEC_XFGVIRT(ITAddressCapabilities, EnumerateCallTreatments)
        /* [hidden][id] */ HRESULT ( STDMETHODCALLTYPE *EnumerateCallTreatments )( 
            __RPC__in ITAddressCapabilities * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumBstr **ppEnumCallTreatment);
        
        DECLSPEC_XFGVIRT(ITAddressCapabilities, get_CompletionMessages)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CompletionMessages )( 
            __RPC__in ITAddressCapabilities * This,
            /* [retval][out] */ __RPC__out VARIANT *pVariant);
        
        DECLSPEC_XFGVIRT(ITAddressCapabilities, EnumerateCompletionMessages)
        /* [hidden][id] */ HRESULT ( STDMETHODCALLTYPE *EnumerateCompletionMessages )( 
            __RPC__in ITAddressCapabilities * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumBstr **ppEnumCompletionMessage);
        
        DECLSPEC_XFGVIRT(ITAddressCapabilities, get_DeviceClasses)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DeviceClasses )( 
            __RPC__in ITAddressCapabilities * This,
            /* [retval][out] */ __RPC__out VARIANT *pVariant);
        
        DECLSPEC_XFGVIRT(ITAddressCapabilities, EnumerateDeviceClasses)
        /* [hidden][id] */ HRESULT ( STDMETHODCALLTYPE *EnumerateDeviceClasses )( 
            __RPC__in ITAddressCapabilities * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumBstr **ppEnumDeviceClass);
        
        END_INTERFACE
    } ITAddressCapabilitiesVtbl;

    interface ITAddressCapabilities
    {
        CONST_VTBL struct ITAddressCapabilitiesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITAddressCapabilities_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITAddressCapabilities_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITAddressCapabilities_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITAddressCapabilities_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITAddressCapabilities_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITAddressCapabilities_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITAddressCapabilities_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITAddressCapabilities_get_AddressCapability(This,AddressCap,plCapability)	\
    ( (This)->lpVtbl -> get_AddressCapability(This,AddressCap,plCapability) ) 

#define ITAddressCapabilities_get_AddressCapabilityString(This,AddressCapString,ppCapabilityString)	\
    ( (This)->lpVtbl -> get_AddressCapabilityString(This,AddressCapString,ppCapabilityString) ) 

#define ITAddressCapabilities_get_CallTreatments(This,pVariant)	\
    ( (This)->lpVtbl -> get_CallTreatments(This,pVariant) ) 

#define ITAddressCapabilities_EnumerateCallTreatments(This,ppEnumCallTreatment)	\
    ( (This)->lpVtbl -> EnumerateCallTreatments(This,ppEnumCallTreatment) ) 

#define ITAddressCapabilities_get_CompletionMessages(This,pVariant)	\
    ( (This)->lpVtbl -> get_CompletionMessages(This,pVariant) ) 

#define ITAddressCapabilities_EnumerateCompletionMessages(This,ppEnumCompletionMessage)	\
    ( (This)->lpVtbl -> EnumerateCompletionMessages(This,ppEnumCompletionMessage) ) 

#define ITAddressCapabilities_get_DeviceClasses(This,pVariant)	\
    ( (This)->lpVtbl -> get_DeviceClasses(This,pVariant) ) 

#define ITAddressCapabilities_EnumerateDeviceClasses(This,ppEnumDeviceClass)	\
    ( (This)->lpVtbl -> EnumerateDeviceClasses(This,ppEnumDeviceClass) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITAddressCapabilities_INTERFACE_DEFINED__ */


#ifndef __ITPhone_INTERFACE_DEFINED__
#define __ITPhone_INTERFACE_DEFINED__

/* interface ITPhone */
/* [object][dual][helpstring][uuid] */ 


EXTERN_C const IID IID_ITPhone;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("09D48DB4-10CC-4388-9DE7-A8465618975A")
    ITPhone : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Open( 
            /* [in] */ PHONE_PRIVILEGE Privilege) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Close( void) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Addresses( 
            /* [retval][out] */ __RPC__out VARIANT *pAddresses) = 0;
        
        virtual /* [helpstring][hidden][id] */ HRESULT STDMETHODCALLTYPE EnumerateAddresses( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumAddress **ppEnumAddress) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PhoneCapsLong( 
            /* [in] */ PHONECAPS_LONG pclCap,
            /* [retval][out] */ __RPC__out long *plCapability) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PhoneCapsString( 
            /* [in] */ PHONECAPS_STRING pcsCap,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppCapability) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Terminals( 
            /* [in] */ __RPC__in_opt ITAddress *pAddress,
            /* [retval][out] */ __RPC__out VARIANT *pTerminals) = 0;
        
        virtual /* [helpstring][hidden][id] */ HRESULT STDMETHODCALLTYPE EnumerateTerminals( 
            /* [in] */ __RPC__in_opt ITAddress *pAddress,
            /* [retval][out] */ __RPC__deref_out_opt IEnumTerminal **ppEnumTerminal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ButtonMode( 
            /* [in] */ long lButtonID,
            /* [retval][out] */ __RPC__out PHONE_BUTTON_MODE *pButtonMode) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_ButtonMode( 
            /* [in] */ long lButtonID,
            /* [in] */ PHONE_BUTTON_MODE ButtonMode) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ButtonFunction( 
            /* [in] */ long lButtonID,
            /* [retval][out] */ __RPC__out PHONE_BUTTON_FUNCTION *pButtonFunction) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_ButtonFunction( 
            /* [in] */ long lButtonID,
            /* [in] */ PHONE_BUTTON_FUNCTION ButtonFunction) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ButtonText( 
            /* [in] */ long lButtonID,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppButtonText) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_ButtonText( 
            /* [in] */ long lButtonID,
            /* [in] */ __RPC__in BSTR bstrButtonText) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ButtonState( 
            /* [in] */ long lButtonID,
            /* [retval][out] */ __RPC__out PHONE_BUTTON_STATE *pButtonState) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_HookSwitchState( 
            /* [in] */ PHONE_HOOK_SWITCH_DEVICE HookSwitchDevice,
            /* [retval][out] */ __RPC__out PHONE_HOOK_SWITCH_STATE *pHookSwitchState) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_HookSwitchState( 
            /* [in] */ PHONE_HOOK_SWITCH_DEVICE HookSwitchDevice,
            /* [in] */ PHONE_HOOK_SWITCH_STATE HookSwitchState) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_RingMode( 
            /* [in] */ long lRingMode) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RingMode( 
            /* [retval][out] */ __RPC__out long *plRingMode) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_RingVolume( 
            /* [in] */ long lRingVolume) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RingVolume( 
            /* [retval][out] */ __RPC__out long *plRingVolume) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Privilege( 
            /* [retval][out] */ __RPC__out PHONE_PRIVILEGE *pPrivilege) = 0;
        
        virtual /* [helpstring][hidden][id] */ HRESULT STDMETHODCALLTYPE GetPhoneCapsBuffer( 
            /* [in] */ PHONECAPS_BUFFER pcbCaps,
            /* [out] */ __RPC__out DWORD *pdwSize,
            /* [out] */ __RPC__deref_out_opt BYTE **ppPhoneCapsBuffer) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PhoneCapsBuffer( 
            /* [in] */ PHONECAPS_BUFFER pcbCaps,
            /* [retval][out] */ __RPC__out VARIANT *pVarBuffer) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_LampMode( 
            /* [in] */ long lLampID,
            /* [retval][out] */ __RPC__out PHONE_LAMP_MODE *pLampMode) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_LampMode( 
            /* [in] */ long lLampID,
            /* [in] */ PHONE_LAMP_MODE LampMode) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Display( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDisplay) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetDisplay( 
            /* [in] */ long lRow,
            /* [in] */ long lColumn,
            /* [in] */ __RPC__in BSTR bstrDisplay) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PreferredAddresses( 
            /* [retval][out] */ __RPC__out VARIANT *pAddresses) = 0;
        
        virtual /* [helpstring][hidden][id] */ HRESULT STDMETHODCALLTYPE EnumeratePreferredAddresses( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumAddress **ppEnumAddress) = 0;
        
        virtual /* [helpstring][hidden][id] */ HRESULT STDMETHODCALLTYPE DeviceSpecific( 
            /* [in] */ __RPC__in BYTE *pParams,
            /* [in] */ DWORD dwSize) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE DeviceSpecificVariant( 
            /* [in] */ VARIANT varDevSpecificByteArray) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE NegotiateExtVersion( 
            /* [in] */ long lLowVersion,
            /* [in] */ long lHighVersion,
            /* [retval][out] */ __RPC__out long *plExtVersion) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITPhoneVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITPhone * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITPhone * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITPhone * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITPhone * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITPhone * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITPhone * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITPhone * This,
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
        
        DECLSPEC_XFGVIRT(ITPhone, Open)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Open )( 
            __RPC__in ITPhone * This,
            /* [in] */ PHONE_PRIVILEGE Privilege);
        
        DECLSPEC_XFGVIRT(ITPhone, Close)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Close )( 
            __RPC__in ITPhone * This);
        
        DECLSPEC_XFGVIRT(ITPhone, get_Addresses)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Addresses )( 
            __RPC__in ITPhone * This,
            /* [retval][out] */ __RPC__out VARIANT *pAddresses);
        
        DECLSPEC_XFGVIRT(ITPhone, EnumerateAddresses)
        /* [helpstring][hidden][id] */ HRESULT ( STDMETHODCALLTYPE *EnumerateAddresses )( 
            __RPC__in ITPhone * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumAddress **ppEnumAddress);
        
        DECLSPEC_XFGVIRT(ITPhone, get_PhoneCapsLong)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PhoneCapsLong )( 
            __RPC__in ITPhone * This,
            /* [in] */ PHONECAPS_LONG pclCap,
            /* [retval][out] */ __RPC__out long *plCapability);
        
        DECLSPEC_XFGVIRT(ITPhone, get_PhoneCapsString)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PhoneCapsString )( 
            __RPC__in ITPhone * This,
            /* [in] */ PHONECAPS_STRING pcsCap,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppCapability);
        
        DECLSPEC_XFGVIRT(ITPhone, get_Terminals)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Terminals )( 
            __RPC__in ITPhone * This,
            /* [in] */ __RPC__in_opt ITAddress *pAddress,
            /* [retval][out] */ __RPC__out VARIANT *pTerminals);
        
        DECLSPEC_XFGVIRT(ITPhone, EnumerateTerminals)
        /* [helpstring][hidden][id] */ HRESULT ( STDMETHODCALLTYPE *EnumerateTerminals )( 
            __RPC__in ITPhone * This,
            /* [in] */ __RPC__in_opt ITAddress *pAddress,
            /* [retval][out] */ __RPC__deref_out_opt IEnumTerminal **ppEnumTerminal);
        
        DECLSPEC_XFGVIRT(ITPhone, get_ButtonMode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ButtonMode )( 
            __RPC__in ITPhone * This,
            /* [in] */ long lButtonID,
            /* [retval][out] */ __RPC__out PHONE_BUTTON_MODE *pButtonMode);
        
        DECLSPEC_XFGVIRT(ITPhone, put_ButtonMode)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ButtonMode )( 
            __RPC__in ITPhone * This,
            /* [in] */ long lButtonID,
            /* [in] */ PHONE_BUTTON_MODE ButtonMode);
        
        DECLSPEC_XFGVIRT(ITPhone, get_ButtonFunction)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ButtonFunction )( 
            __RPC__in ITPhone * This,
            /* [in] */ long lButtonID,
            /* [retval][out] */ __RPC__out PHONE_BUTTON_FUNCTION *pButtonFunction);
        
        DECLSPEC_XFGVIRT(ITPhone, put_ButtonFunction)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ButtonFunction )( 
            __RPC__in ITPhone * This,
            /* [in] */ long lButtonID,
            /* [in] */ PHONE_BUTTON_FUNCTION ButtonFunction);
        
        DECLSPEC_XFGVIRT(ITPhone, get_ButtonText)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ButtonText )( 
            __RPC__in ITPhone * This,
            /* [in] */ long lButtonID,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppButtonText);
        
        DECLSPEC_XFGVIRT(ITPhone, put_ButtonText)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ButtonText )( 
            __RPC__in ITPhone * This,
            /* [in] */ long lButtonID,
            /* [in] */ __RPC__in BSTR bstrButtonText);
        
        DECLSPEC_XFGVIRT(ITPhone, get_ButtonState)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ButtonState )( 
            __RPC__in ITPhone * This,
            /* [in] */ long lButtonID,
            /* [retval][out] */ __RPC__out PHONE_BUTTON_STATE *pButtonState);
        
        DECLSPEC_XFGVIRT(ITPhone, get_HookSwitchState)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_HookSwitchState )( 
            __RPC__in ITPhone * This,
            /* [in] */ PHONE_HOOK_SWITCH_DEVICE HookSwitchDevice,
            /* [retval][out] */ __RPC__out PHONE_HOOK_SWITCH_STATE *pHookSwitchState);
        
        DECLSPEC_XFGVIRT(ITPhone, put_HookSwitchState)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_HookSwitchState )( 
            __RPC__in ITPhone * This,
            /* [in] */ PHONE_HOOK_SWITCH_DEVICE HookSwitchDevice,
            /* [in] */ PHONE_HOOK_SWITCH_STATE HookSwitchState);
        
        DECLSPEC_XFGVIRT(ITPhone, put_RingMode)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_RingMode )( 
            __RPC__in ITPhone * This,
            /* [in] */ long lRingMode);
        
        DECLSPEC_XFGVIRT(ITPhone, get_RingMode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RingMode )( 
            __RPC__in ITPhone * This,
            /* [retval][out] */ __RPC__out long *plRingMode);
        
        DECLSPEC_XFGVIRT(ITPhone, put_RingVolume)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_RingVolume )( 
            __RPC__in ITPhone * This,
            /* [in] */ long lRingVolume);
        
        DECLSPEC_XFGVIRT(ITPhone, get_RingVolume)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RingVolume )( 
            __RPC__in ITPhone * This,
            /* [retval][out] */ __RPC__out long *plRingVolume);
        
        DECLSPEC_XFGVIRT(ITPhone, get_Privilege)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Privilege )( 
            __RPC__in ITPhone * This,
            /* [retval][out] */ __RPC__out PHONE_PRIVILEGE *pPrivilege);
        
        DECLSPEC_XFGVIRT(ITPhone, GetPhoneCapsBuffer)
        /* [helpstring][hidden][id] */ HRESULT ( STDMETHODCALLTYPE *GetPhoneCapsBuffer )( 
            __RPC__in ITPhone * This,
            /* [in] */ PHONECAPS_BUFFER pcbCaps,
            /* [out] */ __RPC__out DWORD *pdwSize,
            /* [out] */ __RPC__deref_out_opt BYTE **ppPhoneCapsBuffer);
        
        DECLSPEC_XFGVIRT(ITPhone, get_PhoneCapsBuffer)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PhoneCapsBuffer )( 
            __RPC__in ITPhone * This,
            /* [in] */ PHONECAPS_BUFFER pcbCaps,
            /* [retval][out] */ __RPC__out VARIANT *pVarBuffer);
        
        DECLSPEC_XFGVIRT(ITPhone, get_LampMode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LampMode )( 
            __RPC__in ITPhone * This,
            /* [in] */ long lLampID,
            /* [retval][out] */ __RPC__out PHONE_LAMP_MODE *pLampMode);
        
        DECLSPEC_XFGVIRT(ITPhone, put_LampMode)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_LampMode )( 
            __RPC__in ITPhone * This,
            /* [in] */ long lLampID,
            /* [in] */ PHONE_LAMP_MODE LampMode);
        
        DECLSPEC_XFGVIRT(ITPhone, get_Display)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Display )( 
            __RPC__in ITPhone * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDisplay);
        
        DECLSPEC_XFGVIRT(ITPhone, SetDisplay)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetDisplay )( 
            __RPC__in ITPhone * This,
            /* [in] */ long lRow,
            /* [in] */ long lColumn,
            /* [in] */ __RPC__in BSTR bstrDisplay);
        
        DECLSPEC_XFGVIRT(ITPhone, get_PreferredAddresses)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PreferredAddresses )( 
            __RPC__in ITPhone * This,
            /* [retval][out] */ __RPC__out VARIANT *pAddresses);
        
        DECLSPEC_XFGVIRT(ITPhone, EnumeratePreferredAddresses)
        /* [helpstring][hidden][id] */ HRESULT ( STDMETHODCALLTYPE *EnumeratePreferredAddresses )( 
            __RPC__in ITPhone * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumAddress **ppEnumAddress);
        
        DECLSPEC_XFGVIRT(ITPhone, DeviceSpecific)
        /* [helpstring][hidden][id] */ HRESULT ( STDMETHODCALLTYPE *DeviceSpecific )( 
            __RPC__in ITPhone * This,
            /* [in] */ __RPC__in BYTE *pParams,
            /* [in] */ DWORD dwSize);
        
        DECLSPEC_XFGVIRT(ITPhone, DeviceSpecificVariant)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DeviceSpecificVariant )( 
            __RPC__in ITPhone * This,
            /* [in] */ VARIANT varDevSpecificByteArray);
        
        DECLSPEC_XFGVIRT(ITPhone, NegotiateExtVersion)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *NegotiateExtVersion )( 
            __RPC__in ITPhone * This,
            /* [in] */ long lLowVersion,
            /* [in] */ long lHighVersion,
            /* [retval][out] */ __RPC__out long *plExtVersion);
        
        END_INTERFACE
    } ITPhoneVtbl;

    interface ITPhone
    {
        CONST_VTBL struct ITPhoneVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITPhone_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITPhone_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITPhone_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITPhone_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITPhone_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITPhone_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITPhone_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITPhone_Open(This,Privilege)	\
    ( (This)->lpVtbl -> Open(This,Privilege) ) 

#define ITPhone_Close(This)	\
    ( (This)->lpVtbl -> Close(This) ) 

#define ITPhone_get_Addresses(This,pAddresses)	\
    ( (This)->lpVtbl -> get_Addresses(This,pAddresses) ) 

#define ITPhone_EnumerateAddresses(This,ppEnumAddress)	\
    ( (This)->lpVtbl -> EnumerateAddresses(This,ppEnumAddress) ) 

#define ITPhone_get_PhoneCapsLong(This,pclCap,plCapability)	\
    ( (This)->lpVtbl -> get_PhoneCapsLong(This,pclCap,plCapability) ) 

#define ITPhone_get_PhoneCapsString(This,pcsCap,ppCapability)	\
    ( (This)->lpVtbl -> get_PhoneCapsString(This,pcsCap,ppCapability) ) 

#define ITPhone_get_Terminals(This,pAddress,pTerminals)	\
    ( (This)->lpVtbl -> get_Terminals(This,pAddress,pTerminals) ) 

#define ITPhone_EnumerateTerminals(This,pAddress,ppEnumTerminal)	\
    ( (This)->lpVtbl -> EnumerateTerminals(This,pAddress,ppEnumTerminal) ) 

#define ITPhone_get_ButtonMode(This,lButtonID,pButtonMode)	\
    ( (This)->lpVtbl -> get_ButtonMode(This,lButtonID,pButtonMode) ) 

#define ITPhone_put_ButtonMode(This,lButtonID,ButtonMode)	\
    ( (This)->lpVtbl -> put_ButtonMode(This,lButtonID,ButtonMode) ) 

#define ITPhone_get_ButtonFunction(This,lButtonID,pButtonFunction)	\
    ( (This)->lpVtbl -> get_ButtonFunction(This,lButtonID,pButtonFunction) ) 

#define ITPhone_put_ButtonFunction(This,lButtonID,ButtonFunction)	\
    ( (This)->lpVtbl -> put_ButtonFunction(This,lButtonID,ButtonFunction) ) 

#define ITPhone_get_ButtonText(This,lButtonID,ppButtonText)	\
    ( (This)->lpVtbl -> get_ButtonText(This,lButtonID,ppButtonText) ) 

#define ITPhone_put_ButtonText(This,lButtonID,bstrButtonText)	\
    ( (This)->lpVtbl -> put_ButtonText(This,lButtonID,bstrButtonText) ) 

#define ITPhone_get_ButtonState(This,lButtonID,pButtonState)	\
    ( (This)->lpVtbl -> get_ButtonState(This,lButtonID,pButtonState) ) 

#define ITPhone_get_HookSwitchState(This,HookSwitchDevice,pHookSwitchState)	\
    ( (This)->lpVtbl -> get_HookSwitchState(This,HookSwitchDevice,pHookSwitchState) ) 

#define ITPhone_put_HookSwitchState(This,HookSwitchDevice,HookSwitchState)	\
    ( (This)->lpVtbl -> put_HookSwitchState(This,HookSwitchDevice,HookSwitchState) ) 

#define ITPhone_put_RingMode(This,lRingMode)	\
    ( (This)->lpVtbl -> put_RingMode(This,lRingMode) ) 

#define ITPhone_get_RingMode(This,plRingMode)	\
    ( (This)->lpVtbl -> get_RingMode(This,plRingMode) ) 

#define ITPhone_put_RingVolume(This,lRingVolume)	\
    ( (This)->lpVtbl -> put_RingVolume(This,lRingVolume) ) 

#define ITPhone_get_RingVolume(This,plRingVolume)	\
    ( (This)->lpVtbl -> get_RingVolume(This,plRingVolume) ) 

#define ITPhone_get_Privilege(This,pPrivilege)	\
    ( (This)->lpVtbl -> get_Privilege(This,pPrivilege) ) 

#define ITPhone_GetPhoneCapsBuffer(This,pcbCaps,pdwSize,ppPhoneCapsBuffer)	\
    ( (This)->lpVtbl -> GetPhoneCapsBuffer(This,pcbCaps,pdwSize,ppPhoneCapsBuffer) ) 

#define ITPhone_get_PhoneCapsBuffer(This,pcbCaps,pVarBuffer)	\
    ( (This)->lpVtbl -> get_PhoneCapsBuffer(This,pcbCaps,pVarBuffer) ) 

#define ITPhone_get_LampMode(This,lLampID,pLampMode)	\
    ( (This)->lpVtbl -> get_LampMode(This,lLampID,pLampMode) ) 

#define ITPhone_put_LampMode(This,lLampID,LampMode)	\
    ( (This)->lpVtbl -> put_LampMode(This,lLampID,LampMode) ) 

#define ITPhone_get_Display(This,pbstrDisplay)	\
    ( (This)->lpVtbl -> get_Display(This,pbstrDisplay) ) 

#define ITPhone_SetDisplay(This,lRow,lColumn,bstrDisplay)	\
    ( (This)->lpVtbl -> SetDisplay(This,lRow,lColumn,bstrDisplay) ) 

#define ITPhone_get_PreferredAddresses(This,pAddresses)	\
    ( (This)->lpVtbl -> get_PreferredAddresses(This,pAddresses) ) 

#define ITPhone_EnumeratePreferredAddresses(This,ppEnumAddress)	\
    ( (This)->lpVtbl -> EnumeratePreferredAddresses(This,ppEnumAddress) ) 

#define ITPhone_DeviceSpecific(This,pParams,dwSize)	\
    ( (This)->lpVtbl -> DeviceSpecific(This,pParams,dwSize) ) 

#define ITPhone_DeviceSpecificVariant(This,varDevSpecificByteArray)	\
    ( (This)->lpVtbl -> DeviceSpecificVariant(This,varDevSpecificByteArray) ) 

#define ITPhone_NegotiateExtVersion(This,lLowVersion,lHighVersion,plExtVersion)	\
    ( (This)->lpVtbl -> NegotiateExtVersion(This,lLowVersion,lHighVersion,plExtVersion) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITPhone_INTERFACE_DEFINED__ */


#ifndef __ITAutomatedPhoneControl_INTERFACE_DEFINED__
#define __ITAutomatedPhoneControl_INTERFACE_DEFINED__

/* interface ITAutomatedPhoneControl */
/* [object][dual][helpstring][uuid] */ 


EXTERN_C const IID IID_ITAutomatedPhoneControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1EE1AF0E-6159-4a61-B79B-6A4BA3FC9DFC")
    ITAutomatedPhoneControl : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE StartTone( 
            /* [in] */ PHONE_TONE Tone,
            /* [in] */ long lDuration) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE StopTone( void) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Tone( 
            /* [retval][out] */ __RPC__out PHONE_TONE *pTone) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE StartRinger( 
            /* [in] */ long lRingMode,
            /* [in] */ long lDuration) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE StopRinger( void) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Ringer( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfRinging) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_PhoneHandlingEnabled( 
            /* [in] */ VARIANT_BOOL fEnabled) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PhoneHandlingEnabled( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfEnabled) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_AutoEndOfNumberTimeout( 
            /* [in] */ long lTimeout) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_AutoEndOfNumberTimeout( 
            /* [retval][out] */ __RPC__out long *plTimeout) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_AutoDialtone( 
            /* [in] */ VARIANT_BOOL fEnabled) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_AutoDialtone( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfEnabled) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_AutoStopTonesOnOnHook( 
            /* [in] */ VARIANT_BOOL fEnabled) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_AutoStopTonesOnOnHook( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfEnabled) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_AutoStopRingOnOffHook( 
            /* [in] */ VARIANT_BOOL fEnabled) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_AutoStopRingOnOffHook( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfEnabled) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_AutoKeypadTones( 
            /* [in] */ VARIANT_BOOL fEnabled) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_AutoKeypadTones( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfEnabled) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_AutoKeypadTonesMinimumDuration( 
            /* [in] */ long lDuration) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_AutoKeypadTonesMinimumDuration( 
            /* [retval][out] */ __RPC__out long *plDuration) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_AutoVolumeControl( 
            /* [in] */ VARIANT_BOOL fEnabled) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_AutoVolumeControl( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *fEnabled) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_AutoVolumeControlStep( 
            /* [in] */ long lStepSize) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_AutoVolumeControlStep( 
            /* [retval][out] */ __RPC__out long *plStepSize) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_AutoVolumeControlRepeatDelay( 
            /* [in] */ long lDelay) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_AutoVolumeControlRepeatDelay( 
            /* [retval][out] */ __RPC__out long *plDelay) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_AutoVolumeControlRepeatPeriod( 
            /* [in] */ long lPeriod) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_AutoVolumeControlRepeatPeriod( 
            /* [retval][out] */ __RPC__out long *plPeriod) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SelectCall( 
            /* [in] */ __RPC__in_opt ITCallInfo *pCall,
            /* [in] */ VARIANT_BOOL fSelectDefaultTerminals) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE UnselectCall( 
            /* [in] */ __RPC__in_opt ITCallInfo *pCall) = 0;
        
        virtual /* [helpstring][hidden][id] */ HRESULT STDMETHODCALLTYPE EnumerateSelectedCalls( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumCall **ppCallEnum) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SelectedCalls( 
            /* [retval][out] */ __RPC__out VARIANT *pVariant) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITAutomatedPhoneControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITAutomatedPhoneControl * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITAutomatedPhoneControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITAutomatedPhoneControl * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITAutomatedPhoneControl * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITAutomatedPhoneControl * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITAutomatedPhoneControl * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITAutomatedPhoneControl * This,
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
        
        DECLSPEC_XFGVIRT(ITAutomatedPhoneControl, StartTone)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *StartTone )( 
            __RPC__in ITAutomatedPhoneControl * This,
            /* [in] */ PHONE_TONE Tone,
            /* [in] */ long lDuration);
        
        DECLSPEC_XFGVIRT(ITAutomatedPhoneControl, StopTone)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *StopTone )( 
            __RPC__in ITAutomatedPhoneControl * This);
        
        DECLSPEC_XFGVIRT(ITAutomatedPhoneControl, get_Tone)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Tone )( 
            __RPC__in ITAutomatedPhoneControl * This,
            /* [retval][out] */ __RPC__out PHONE_TONE *pTone);
        
        DECLSPEC_XFGVIRT(ITAutomatedPhoneControl, StartRinger)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *StartRinger )( 
            __RPC__in ITAutomatedPhoneControl * This,
            /* [in] */ long lRingMode,
            /* [in] */ long lDuration);
        
        DECLSPEC_XFGVIRT(ITAutomatedPhoneControl, StopRinger)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *StopRinger )( 
            __RPC__in ITAutomatedPhoneControl * This);
        
        DECLSPEC_XFGVIRT(ITAutomatedPhoneControl, get_Ringer)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Ringer )( 
            __RPC__in ITAutomatedPhoneControl * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfRinging);
        
        DECLSPEC_XFGVIRT(ITAutomatedPhoneControl, put_PhoneHandlingEnabled)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PhoneHandlingEnabled )( 
            __RPC__in ITAutomatedPhoneControl * This,
            /* [in] */ VARIANT_BOOL fEnabled);
        
        DECLSPEC_XFGVIRT(ITAutomatedPhoneControl, get_PhoneHandlingEnabled)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PhoneHandlingEnabled )( 
            __RPC__in ITAutomatedPhoneControl * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfEnabled);
        
        DECLSPEC_XFGVIRT(ITAutomatedPhoneControl, put_AutoEndOfNumberTimeout)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_AutoEndOfNumberTimeout )( 
            __RPC__in ITAutomatedPhoneControl * This,
            /* [in] */ long lTimeout);
        
        DECLSPEC_XFGVIRT(ITAutomatedPhoneControl, get_AutoEndOfNumberTimeout)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AutoEndOfNumberTimeout )( 
            __RPC__in ITAutomatedPhoneControl * This,
            /* [retval][out] */ __RPC__out long *plTimeout);
        
        DECLSPEC_XFGVIRT(ITAutomatedPhoneControl, put_AutoDialtone)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_AutoDialtone )( 
            __RPC__in ITAutomatedPhoneControl * This,
            /* [in] */ VARIANT_BOOL fEnabled);
        
        DECLSPEC_XFGVIRT(ITAutomatedPhoneControl, get_AutoDialtone)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AutoDialtone )( 
            __RPC__in ITAutomatedPhoneControl * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfEnabled);
        
        DECLSPEC_XFGVIRT(ITAutomatedPhoneControl, put_AutoStopTonesOnOnHook)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_AutoStopTonesOnOnHook )( 
            __RPC__in ITAutomatedPhoneControl * This,
            /* [in] */ VARIANT_BOOL fEnabled);
        
        DECLSPEC_XFGVIRT(ITAutomatedPhoneControl, get_AutoStopTonesOnOnHook)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AutoStopTonesOnOnHook )( 
            __RPC__in ITAutomatedPhoneControl * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfEnabled);
        
        DECLSPEC_XFGVIRT(ITAutomatedPhoneControl, put_AutoStopRingOnOffHook)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_AutoStopRingOnOffHook )( 
            __RPC__in ITAutomatedPhoneControl * This,
            /* [in] */ VARIANT_BOOL fEnabled);
        
        DECLSPEC_XFGVIRT(ITAutomatedPhoneControl, get_AutoStopRingOnOffHook)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AutoStopRingOnOffHook )( 
            __RPC__in ITAutomatedPhoneControl * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfEnabled);
        
        DECLSPEC_XFGVIRT(ITAutomatedPhoneControl, put_AutoKeypadTones)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_AutoKeypadTones )( 
            __RPC__in ITAutomatedPhoneControl * This,
            /* [in] */ VARIANT_BOOL fEnabled);
        
        DECLSPEC_XFGVIRT(ITAutomatedPhoneControl, get_AutoKeypadTones)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AutoKeypadTones )( 
            __RPC__in ITAutomatedPhoneControl * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfEnabled);
        
        DECLSPEC_XFGVIRT(ITAutomatedPhoneControl, put_AutoKeypadTonesMinimumDuration)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_AutoKeypadTonesMinimumDuration )( 
            __RPC__in ITAutomatedPhoneControl * This,
            /* [in] */ long lDuration);
        
        DECLSPEC_XFGVIRT(ITAutomatedPhoneControl, get_AutoKeypadTonesMinimumDuration)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AutoKeypadTonesMinimumDuration )( 
            __RPC__in ITAutomatedPhoneControl * This,
            /* [retval][out] */ __RPC__out long *plDuration);
        
        DECLSPEC_XFGVIRT(ITAutomatedPhoneControl, put_AutoVolumeControl)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_AutoVolumeControl )( 
            __RPC__in ITAutomatedPhoneControl * This,
            /* [in] */ VARIANT_BOOL fEnabled);
        
        DECLSPEC_XFGVIRT(ITAutomatedPhoneControl, get_AutoVolumeControl)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AutoVolumeControl )( 
            __RPC__in ITAutomatedPhoneControl * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *fEnabled);
        
        DECLSPEC_XFGVIRT(ITAutomatedPhoneControl, put_AutoVolumeControlStep)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_AutoVolumeControlStep )( 
            __RPC__in ITAutomatedPhoneControl * This,
            /* [in] */ long lStepSize);
        
        DECLSPEC_XFGVIRT(ITAutomatedPhoneControl, get_AutoVolumeControlStep)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AutoVolumeControlStep )( 
            __RPC__in ITAutomatedPhoneControl * This,
            /* [retval][out] */ __RPC__out long *plStepSize);
        
        DECLSPEC_XFGVIRT(ITAutomatedPhoneControl, put_AutoVolumeControlRepeatDelay)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_AutoVolumeControlRepeatDelay )( 
            __RPC__in ITAutomatedPhoneControl * This,
            /* [in] */ long lDelay);
        
        DECLSPEC_XFGVIRT(ITAutomatedPhoneControl, get_AutoVolumeControlRepeatDelay)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AutoVolumeControlRepeatDelay )( 
            __RPC__in ITAutomatedPhoneControl * This,
            /* [retval][out] */ __RPC__out long *plDelay);
        
        DECLSPEC_XFGVIRT(ITAutomatedPhoneControl, put_AutoVolumeControlRepeatPeriod)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_AutoVolumeControlRepeatPeriod )( 
            __RPC__in ITAutomatedPhoneControl * This,
            /* [in] */ long lPeriod);
        
        DECLSPEC_XFGVIRT(ITAutomatedPhoneControl, get_AutoVolumeControlRepeatPeriod)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AutoVolumeControlRepeatPeriod )( 
            __RPC__in ITAutomatedPhoneControl * This,
            /* [retval][out] */ __RPC__out long *plPeriod);
        
        DECLSPEC_XFGVIRT(ITAutomatedPhoneControl, SelectCall)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SelectCall )( 
            __RPC__in ITAutomatedPhoneControl * This,
            /* [in] */ __RPC__in_opt ITCallInfo *pCall,
            /* [in] */ VARIANT_BOOL fSelectDefaultTerminals);
        
        DECLSPEC_XFGVIRT(ITAutomatedPhoneControl, UnselectCall)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *UnselectCall )( 
            __RPC__in ITAutomatedPhoneControl * This,
            /* [in] */ __RPC__in_opt ITCallInfo *pCall);
        
        DECLSPEC_XFGVIRT(ITAutomatedPhoneControl, EnumerateSelectedCalls)
        /* [helpstring][hidden][id] */ HRESULT ( STDMETHODCALLTYPE *EnumerateSelectedCalls )( 
            __RPC__in ITAutomatedPhoneControl * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumCall **ppCallEnum);
        
        DECLSPEC_XFGVIRT(ITAutomatedPhoneControl, get_SelectedCalls)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SelectedCalls )( 
            __RPC__in ITAutomatedPhoneControl * This,
            /* [retval][out] */ __RPC__out VARIANT *pVariant);
        
        END_INTERFACE
    } ITAutomatedPhoneControlVtbl;

    interface ITAutomatedPhoneControl
    {
        CONST_VTBL struct ITAutomatedPhoneControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITAutomatedPhoneControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITAutomatedPhoneControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITAutomatedPhoneControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITAutomatedPhoneControl_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITAutomatedPhoneControl_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITAutomatedPhoneControl_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITAutomatedPhoneControl_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITAutomatedPhoneControl_StartTone(This,Tone,lDuration)	\
    ( (This)->lpVtbl -> StartTone(This,Tone,lDuration) ) 

#define ITAutomatedPhoneControl_StopTone(This)	\
    ( (This)->lpVtbl -> StopTone(This) ) 

#define ITAutomatedPhoneControl_get_Tone(This,pTone)	\
    ( (This)->lpVtbl -> get_Tone(This,pTone) ) 

#define ITAutomatedPhoneControl_StartRinger(This,lRingMode,lDuration)	\
    ( (This)->lpVtbl -> StartRinger(This,lRingMode,lDuration) ) 

#define ITAutomatedPhoneControl_StopRinger(This)	\
    ( (This)->lpVtbl -> StopRinger(This) ) 

#define ITAutomatedPhoneControl_get_Ringer(This,pfRinging)	\
    ( (This)->lpVtbl -> get_Ringer(This,pfRinging) ) 

#define ITAutomatedPhoneControl_put_PhoneHandlingEnabled(This,fEnabled)	\
    ( (This)->lpVtbl -> put_PhoneHandlingEnabled(This,fEnabled) ) 

#define ITAutomatedPhoneControl_get_PhoneHandlingEnabled(This,pfEnabled)	\
    ( (This)->lpVtbl -> get_PhoneHandlingEnabled(This,pfEnabled) ) 

#define ITAutomatedPhoneControl_put_AutoEndOfNumberTimeout(This,lTimeout)	\
    ( (This)->lpVtbl -> put_AutoEndOfNumberTimeout(This,lTimeout) ) 

#define ITAutomatedPhoneControl_get_AutoEndOfNumberTimeout(This,plTimeout)	\
    ( (This)->lpVtbl -> get_AutoEndOfNumberTimeout(This,plTimeout) ) 

#define ITAutomatedPhoneControl_put_AutoDialtone(This,fEnabled)	\
    ( (This)->lpVtbl -> put_AutoDialtone(This,fEnabled) ) 

#define ITAutomatedPhoneControl_get_AutoDialtone(This,pfEnabled)	\
    ( (This)->lpVtbl -> get_AutoDialtone(This,pfEnabled) ) 

#define ITAutomatedPhoneControl_put_AutoStopTonesOnOnHook(This,fEnabled)	\
    ( (This)->lpVtbl -> put_AutoStopTonesOnOnHook(This,fEnabled) ) 

#define ITAutomatedPhoneControl_get_AutoStopTonesOnOnHook(This,pfEnabled)	\
    ( (This)->lpVtbl -> get_AutoStopTonesOnOnHook(This,pfEnabled) ) 

#define ITAutomatedPhoneControl_put_AutoStopRingOnOffHook(This,fEnabled)	\
    ( (This)->lpVtbl -> put_AutoStopRingOnOffHook(This,fEnabled) ) 

#define ITAutomatedPhoneControl_get_AutoStopRingOnOffHook(This,pfEnabled)	\
    ( (This)->lpVtbl -> get_AutoStopRingOnOffHook(This,pfEnabled) ) 

#define ITAutomatedPhoneControl_put_AutoKeypadTones(This,fEnabled)	\
    ( (This)->lpVtbl -> put_AutoKeypadTones(This,fEnabled) ) 

#define ITAutomatedPhoneControl_get_AutoKeypadTones(This,pfEnabled)	\
    ( (This)->lpVtbl -> get_AutoKeypadTones(This,pfEnabled) ) 

#define ITAutomatedPhoneControl_put_AutoKeypadTonesMinimumDuration(This,lDuration)	\
    ( (This)->lpVtbl -> put_AutoKeypadTonesMinimumDuration(This,lDuration) ) 

#define ITAutomatedPhoneControl_get_AutoKeypadTonesMinimumDuration(This,plDuration)	\
    ( (This)->lpVtbl -> get_AutoKeypadTonesMinimumDuration(This,plDuration) ) 

#define ITAutomatedPhoneControl_put_AutoVolumeControl(This,fEnabled)	\
    ( (This)->lpVtbl -> put_AutoVolumeControl(This,fEnabled) ) 

#define ITAutomatedPhoneControl_get_AutoVolumeControl(This,fEnabled)	\
    ( (This)->lpVtbl -> get_AutoVolumeControl(This,fEnabled) ) 

#define ITAutomatedPhoneControl_put_AutoVolumeControlStep(This,lStepSize)	\
    ( (This)->lpVtbl -> put_AutoVolumeControlStep(This,lStepSize) ) 

#define ITAutomatedPhoneControl_get_AutoVolumeControlStep(This,plStepSize)	\
    ( (This)->lpVtbl -> get_AutoVolumeControlStep(This,plStepSize) ) 

#define ITAutomatedPhoneControl_put_AutoVolumeControlRepeatDelay(This,lDelay)	\
    ( (This)->lpVtbl -> put_AutoVolumeControlRepeatDelay(This,lDelay) ) 

#define ITAutomatedPhoneControl_get_AutoVolumeControlRepeatDelay(This,plDelay)	\
    ( (This)->lpVtbl -> get_AutoVolumeControlRepeatDelay(This,plDelay) ) 

#define ITAutomatedPhoneControl_put_AutoVolumeControlRepeatPeriod(This,lPeriod)	\
    ( (This)->lpVtbl -> put_AutoVolumeControlRepeatPeriod(This,lPeriod) ) 

#define ITAutomatedPhoneControl_get_AutoVolumeControlRepeatPeriod(This,plPeriod)	\
    ( (This)->lpVtbl -> get_AutoVolumeControlRepeatPeriod(This,plPeriod) ) 

#define ITAutomatedPhoneControl_SelectCall(This,pCall,fSelectDefaultTerminals)	\
    ( (This)->lpVtbl -> SelectCall(This,pCall,fSelectDefaultTerminals) ) 

#define ITAutomatedPhoneControl_UnselectCall(This,pCall)	\
    ( (This)->lpVtbl -> UnselectCall(This,pCall) ) 

#define ITAutomatedPhoneControl_EnumerateSelectedCalls(This,ppCallEnum)	\
    ( (This)->lpVtbl -> EnumerateSelectedCalls(This,ppCallEnum) ) 

#define ITAutomatedPhoneControl_get_SelectedCalls(This,pVariant)	\
    ( (This)->lpVtbl -> get_SelectedCalls(This,pVariant) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITAutomatedPhoneControl_INTERFACE_DEFINED__ */


#ifndef __ITBasicCallControl_INTERFACE_DEFINED__
#define __ITBasicCallControl_INTERFACE_DEFINED__

/* interface ITBasicCallControl */
/* [object][dual][helpstring][uuid] */ 


EXTERN_C const IID IID_ITBasicCallControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B1EFC389-9355-11d0-835C-00AA003CCABD")
    ITBasicCallControl : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Connect( 
            /* [in] */ VARIANT_BOOL fSync) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Answer( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Disconnect( 
            /* [in] */ DISCONNECT_CODE code) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Hold( 
            /* [in] */ VARIANT_BOOL fHold) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE HandoffDirect( 
            /* [in] */ __RPC__in BSTR pApplicationName) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE HandoffIndirect( 
            /* [in] */ long lMediaType) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Conference( 
            /* [in] */ __RPC__in_opt ITBasicCallControl *pCall,
            /* [in] */ VARIANT_BOOL fSync) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Transfer( 
            /* [in] */ __RPC__in_opt ITBasicCallControl *pCall,
            /* [in] */ VARIANT_BOOL fSync) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE BlindTransfer( 
            /* [in] */ __RPC__in BSTR pDestAddress) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SwapHold( 
            /* [in] */ __RPC__in_opt ITBasicCallControl *pCall) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ParkDirect( 
            /* [in] */ __RPC__in BSTR pParkAddress) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ParkIndirect( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppNonDirAddress) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Unpark( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetQOS( 
            /* [in] */ long lMediaType,
            /* [in] */ QOS_SERVICE_LEVEL ServiceLevel) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Pickup( 
            /* [in] */ __RPC__in BSTR pGroupID) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Dial( 
            /* [in] */ __RPC__in BSTR pDestAddress) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Finish( 
            /* [in] */ FINISH_MODE finishMode) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE RemoveFromConference( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITBasicCallControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITBasicCallControl * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITBasicCallControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITBasicCallControl * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITBasicCallControl * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITBasicCallControl * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITBasicCallControl * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITBasicCallControl * This,
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
        
        DECLSPEC_XFGVIRT(ITBasicCallControl, Connect)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Connect )( 
            __RPC__in ITBasicCallControl * This,
            /* [in] */ VARIANT_BOOL fSync);
        
        DECLSPEC_XFGVIRT(ITBasicCallControl, Answer)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Answer )( 
            __RPC__in ITBasicCallControl * This);
        
        DECLSPEC_XFGVIRT(ITBasicCallControl, Disconnect)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Disconnect )( 
            __RPC__in ITBasicCallControl * This,
            /* [in] */ DISCONNECT_CODE code);
        
        DECLSPEC_XFGVIRT(ITBasicCallControl, Hold)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Hold )( 
            __RPC__in ITBasicCallControl * This,
            /* [in] */ VARIANT_BOOL fHold);
        
        DECLSPEC_XFGVIRT(ITBasicCallControl, HandoffDirect)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *HandoffDirect )( 
            __RPC__in ITBasicCallControl * This,
            /* [in] */ __RPC__in BSTR pApplicationName);
        
        DECLSPEC_XFGVIRT(ITBasicCallControl, HandoffIndirect)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *HandoffIndirect )( 
            __RPC__in ITBasicCallControl * This,
            /* [in] */ long lMediaType);
        
        DECLSPEC_XFGVIRT(ITBasicCallControl, Conference)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Conference )( 
            __RPC__in ITBasicCallControl * This,
            /* [in] */ __RPC__in_opt ITBasicCallControl *pCall,
            /* [in] */ VARIANT_BOOL fSync);
        
        DECLSPEC_XFGVIRT(ITBasicCallControl, Transfer)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Transfer )( 
            __RPC__in ITBasicCallControl * This,
            /* [in] */ __RPC__in_opt ITBasicCallControl *pCall,
            /* [in] */ VARIANT_BOOL fSync);
        
        DECLSPEC_XFGVIRT(ITBasicCallControl, BlindTransfer)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *BlindTransfer )( 
            __RPC__in ITBasicCallControl * This,
            /* [in] */ __RPC__in BSTR pDestAddress);
        
        DECLSPEC_XFGVIRT(ITBasicCallControl, SwapHold)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SwapHold )( 
            __RPC__in ITBasicCallControl * This,
            /* [in] */ __RPC__in_opt ITBasicCallControl *pCall);
        
        DECLSPEC_XFGVIRT(ITBasicCallControl, ParkDirect)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ParkDirect )( 
            __RPC__in ITBasicCallControl * This,
            /* [in] */ __RPC__in BSTR pParkAddress);
        
        DECLSPEC_XFGVIRT(ITBasicCallControl, ParkIndirect)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ParkIndirect )( 
            __RPC__in ITBasicCallControl * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppNonDirAddress);
        
        DECLSPEC_XFGVIRT(ITBasicCallControl, Unpark)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Unpark )( 
            __RPC__in ITBasicCallControl * This);
        
        DECLSPEC_XFGVIRT(ITBasicCallControl, SetQOS)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetQOS )( 
            __RPC__in ITBasicCallControl * This,
            /* [in] */ long lMediaType,
            /* [in] */ QOS_SERVICE_LEVEL ServiceLevel);
        
        DECLSPEC_XFGVIRT(ITBasicCallControl, Pickup)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Pickup )( 
            __RPC__in ITBasicCallControl * This,
            /* [in] */ __RPC__in BSTR pGroupID);
        
        DECLSPEC_XFGVIRT(ITBasicCallControl, Dial)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Dial )( 
            __RPC__in ITBasicCallControl * This,
            /* [in] */ __RPC__in BSTR pDestAddress);
        
        DECLSPEC_XFGVIRT(ITBasicCallControl, Finish)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Finish )( 
            __RPC__in ITBasicCallControl * This,
            /* [in] */ FINISH_MODE finishMode);
        
        DECLSPEC_XFGVIRT(ITBasicCallControl, RemoveFromConference)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RemoveFromConference )( 
            __RPC__in ITBasicCallControl * This);
        
        END_INTERFACE
    } ITBasicCallControlVtbl;

    interface ITBasicCallControl
    {
        CONST_VTBL struct ITBasicCallControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITBasicCallControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITBasicCallControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITBasicCallControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITBasicCallControl_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITBasicCallControl_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITBasicCallControl_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITBasicCallControl_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITBasicCallControl_Connect(This,fSync)	\
    ( (This)->lpVtbl -> Connect(This,fSync) ) 

#define ITBasicCallControl_Answer(This)	\
    ( (This)->lpVtbl -> Answer(This) ) 

#define ITBasicCallControl_Disconnect(This,code)	\
    ( (This)->lpVtbl -> Disconnect(This,code) ) 

#define ITBasicCallControl_Hold(This,fHold)	\
    ( (This)->lpVtbl -> Hold(This,fHold) ) 

#define ITBasicCallControl_HandoffDirect(This,pApplicationName)	\
    ( (This)->lpVtbl -> HandoffDirect(This,pApplicationName) ) 

#define ITBasicCallControl_HandoffIndirect(This,lMediaType)	\
    ( (This)->lpVtbl -> HandoffIndirect(This,lMediaType) ) 

#define ITBasicCallControl_Conference(This,pCall,fSync)	\
    ( (This)->lpVtbl -> Conference(This,pCall,fSync) ) 

#define ITBasicCallControl_Transfer(This,pCall,fSync)	\
    ( (This)->lpVtbl -> Transfer(This,pCall,fSync) ) 

#define ITBasicCallControl_BlindTransfer(This,pDestAddress)	\
    ( (This)->lpVtbl -> BlindTransfer(This,pDestAddress) ) 

#define ITBasicCallControl_SwapHold(This,pCall)	\
    ( (This)->lpVtbl -> SwapHold(This,pCall) ) 

#define ITBasicCallControl_ParkDirect(This,pParkAddress)	\
    ( (This)->lpVtbl -> ParkDirect(This,pParkAddress) ) 

#define ITBasicCallControl_ParkIndirect(This,ppNonDirAddress)	\
    ( (This)->lpVtbl -> ParkIndirect(This,ppNonDirAddress) ) 

#define ITBasicCallControl_Unpark(This)	\
    ( (This)->lpVtbl -> Unpark(This) ) 

#define ITBasicCallControl_SetQOS(This,lMediaType,ServiceLevel)	\
    ( (This)->lpVtbl -> SetQOS(This,lMediaType,ServiceLevel) ) 

#define ITBasicCallControl_Pickup(This,pGroupID)	\
    ( (This)->lpVtbl -> Pickup(This,pGroupID) ) 

#define ITBasicCallControl_Dial(This,pDestAddress)	\
    ( (This)->lpVtbl -> Dial(This,pDestAddress) ) 

#define ITBasicCallControl_Finish(This,finishMode)	\
    ( (This)->lpVtbl -> Finish(This,finishMode) ) 

#define ITBasicCallControl_RemoveFromConference(This)	\
    ( (This)->lpVtbl -> RemoveFromConference(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITBasicCallControl_INTERFACE_DEFINED__ */


#ifndef __ITCallInfo_INTERFACE_DEFINED__
#define __ITCallInfo_INTERFACE_DEFINED__

/* interface ITCallInfo */
/* [object][dual][helpstring][uuid] */ 


EXTERN_C const IID IID_ITCallInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("350F85D1-1227-11D3-83D4-00C04FB6809F")
    ITCallInfo : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Address( 
            /* [retval][out] */ __RPC__deref_out_opt ITAddress **ppAddress) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CallState( 
            /* [retval][out] */ __RPC__out CALL_STATE *pCallState) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Privilege( 
            /* [retval][out] */ __RPC__out CALL_PRIVILEGE *pPrivilege) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CallHub( 
            /* [retval][out] */ __RPC__deref_out_opt ITCallHub **ppCallHub) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CallInfoLong( 
            /* [in] */ CALLINFO_LONG CallInfoLong,
            /* [retval][out] */ __RPC__out long *plCallInfoLongVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_CallInfoLong( 
            /* [in] */ CALLINFO_LONG CallInfoLong,
            /* [in] */ long lCallInfoLongVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CallInfoString( 
            /* [in] */ CALLINFO_STRING CallInfoString,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppCallInfoString) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_CallInfoString( 
            /* [in] */ CALLINFO_STRING CallInfoString,
            /* [in] */ __RPC__in BSTR pCallInfoString) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CallInfoBuffer( 
            /* [in] */ CALLINFO_BUFFER CallInfoBuffer,
            /* [retval][out] */ __RPC__out VARIANT *ppCallInfoBuffer) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_CallInfoBuffer( 
            /* [in] */ CALLINFO_BUFFER CallInfoBuffer,
            /* [in] */ VARIANT pCallInfoBuffer) = 0;
        
        virtual /* [hidden][id] */ HRESULT STDMETHODCALLTYPE GetCallInfoBuffer( 
            /* [in] */ CALLINFO_BUFFER CallInfoBuffer,
            /* [out] */ __RPC__out DWORD *pdwSize,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pdwSize) BYTE **ppCallInfoBuffer) = 0;
        
        virtual /* [hidden][id] */ HRESULT STDMETHODCALLTYPE SetCallInfoBuffer( 
            /* [in] */ CALLINFO_BUFFER CallInfoBuffer,
            /* [in] */ DWORD dwSize,
            /* [size_is][in] */ __RPC__in_ecount_full(dwSize) BYTE *pCallInfoBuffer) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ReleaseUserUserInfo( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITCallInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITCallInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITCallInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITCallInfo * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITCallInfo * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITCallInfo * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITCallInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITCallInfo * This,
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
        
        DECLSPEC_XFGVIRT(ITCallInfo, get_Address)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Address )( 
            __RPC__in ITCallInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt ITAddress **ppAddress);
        
        DECLSPEC_XFGVIRT(ITCallInfo, get_CallState)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CallState )( 
            __RPC__in ITCallInfo * This,
            /* [retval][out] */ __RPC__out CALL_STATE *pCallState);
        
        DECLSPEC_XFGVIRT(ITCallInfo, get_Privilege)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Privilege )( 
            __RPC__in ITCallInfo * This,
            /* [retval][out] */ __RPC__out CALL_PRIVILEGE *pPrivilege);
        
        DECLSPEC_XFGVIRT(ITCallInfo, get_CallHub)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CallHub )( 
            __RPC__in ITCallInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt ITCallHub **ppCallHub);
        
        DECLSPEC_XFGVIRT(ITCallInfo, get_CallInfoLong)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CallInfoLong )( 
            __RPC__in ITCallInfo * This,
            /* [in] */ CALLINFO_LONG CallInfoLong,
            /* [retval][out] */ __RPC__out long *plCallInfoLongVal);
        
        DECLSPEC_XFGVIRT(ITCallInfo, put_CallInfoLong)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_CallInfoLong )( 
            __RPC__in ITCallInfo * This,
            /* [in] */ CALLINFO_LONG CallInfoLong,
            /* [in] */ long lCallInfoLongVal);
        
        DECLSPEC_XFGVIRT(ITCallInfo, get_CallInfoString)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CallInfoString )( 
            __RPC__in ITCallInfo * This,
            /* [in] */ CALLINFO_STRING CallInfoString,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppCallInfoString);
        
        DECLSPEC_XFGVIRT(ITCallInfo, put_CallInfoString)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_CallInfoString )( 
            __RPC__in ITCallInfo * This,
            /* [in] */ CALLINFO_STRING CallInfoString,
            /* [in] */ __RPC__in BSTR pCallInfoString);
        
        DECLSPEC_XFGVIRT(ITCallInfo, get_CallInfoBuffer)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CallInfoBuffer )( 
            __RPC__in ITCallInfo * This,
            /* [in] */ CALLINFO_BUFFER CallInfoBuffer,
            /* [retval][out] */ __RPC__out VARIANT *ppCallInfoBuffer);
        
        DECLSPEC_XFGVIRT(ITCallInfo, put_CallInfoBuffer)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_CallInfoBuffer )( 
            __RPC__in ITCallInfo * This,
            /* [in] */ CALLINFO_BUFFER CallInfoBuffer,
            /* [in] */ VARIANT pCallInfoBuffer);
        
        DECLSPEC_XFGVIRT(ITCallInfo, GetCallInfoBuffer)
        /* [hidden][id] */ HRESULT ( STDMETHODCALLTYPE *GetCallInfoBuffer )( 
            __RPC__in ITCallInfo * This,
            /* [in] */ CALLINFO_BUFFER CallInfoBuffer,
            /* [out] */ __RPC__out DWORD *pdwSize,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pdwSize) BYTE **ppCallInfoBuffer);
        
        DECLSPEC_XFGVIRT(ITCallInfo, SetCallInfoBuffer)
        /* [hidden][id] */ HRESULT ( STDMETHODCALLTYPE *SetCallInfoBuffer )( 
            __RPC__in ITCallInfo * This,
            /* [in] */ CALLINFO_BUFFER CallInfoBuffer,
            /* [in] */ DWORD dwSize,
            /* [size_is][in] */ __RPC__in_ecount_full(dwSize) BYTE *pCallInfoBuffer);
        
        DECLSPEC_XFGVIRT(ITCallInfo, ReleaseUserUserInfo)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ReleaseUserUserInfo )( 
            __RPC__in ITCallInfo * This);
        
        END_INTERFACE
    } ITCallInfoVtbl;

    interface ITCallInfo
    {
        CONST_VTBL struct ITCallInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITCallInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITCallInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITCallInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITCallInfo_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITCallInfo_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITCallInfo_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITCallInfo_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITCallInfo_get_Address(This,ppAddress)	\
    ( (This)->lpVtbl -> get_Address(This,ppAddress) ) 

#define ITCallInfo_get_CallState(This,pCallState)	\
    ( (This)->lpVtbl -> get_CallState(This,pCallState) ) 

#define ITCallInfo_get_Privilege(This,pPrivilege)	\
    ( (This)->lpVtbl -> get_Privilege(This,pPrivilege) ) 

#define ITCallInfo_get_CallHub(This,ppCallHub)	\
    ( (This)->lpVtbl -> get_CallHub(This,ppCallHub) ) 

#define ITCallInfo_get_CallInfoLong(This,CallInfoLong,plCallInfoLongVal)	\
    ( (This)->lpVtbl -> get_CallInfoLong(This,CallInfoLong,plCallInfoLongVal) ) 

#define ITCallInfo_put_CallInfoLong(This,CallInfoLong,lCallInfoLongVal)	\
    ( (This)->lpVtbl -> put_CallInfoLong(This,CallInfoLong,lCallInfoLongVal) ) 

#define ITCallInfo_get_CallInfoString(This,CallInfoString,ppCallInfoString)	\
    ( (This)->lpVtbl -> get_CallInfoString(This,CallInfoString,ppCallInfoString) ) 

#define ITCallInfo_put_CallInfoString(This,CallInfoString,pCallInfoString)	\
    ( (This)->lpVtbl -> put_CallInfoString(This,CallInfoString,pCallInfoString) ) 

#define ITCallInfo_get_CallInfoBuffer(This,CallInfoBuffer,ppCallInfoBuffer)	\
    ( (This)->lpVtbl -> get_CallInfoBuffer(This,CallInfoBuffer,ppCallInfoBuffer) ) 

#define ITCallInfo_put_CallInfoBuffer(This,CallInfoBuffer,pCallInfoBuffer)	\
    ( (This)->lpVtbl -> put_CallInfoBuffer(This,CallInfoBuffer,pCallInfoBuffer) ) 

#define ITCallInfo_GetCallInfoBuffer(This,CallInfoBuffer,pdwSize,ppCallInfoBuffer)	\
    ( (This)->lpVtbl -> GetCallInfoBuffer(This,CallInfoBuffer,pdwSize,ppCallInfoBuffer) ) 

#define ITCallInfo_SetCallInfoBuffer(This,CallInfoBuffer,dwSize,pCallInfoBuffer)	\
    ( (This)->lpVtbl -> SetCallInfoBuffer(This,CallInfoBuffer,dwSize,pCallInfoBuffer) ) 

#define ITCallInfo_ReleaseUserUserInfo(This)	\
    ( (This)->lpVtbl -> ReleaseUserUserInfo(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITCallInfo_INTERFACE_DEFINED__ */


#ifndef __ITCallInfo2_INTERFACE_DEFINED__
#define __ITCallInfo2_INTERFACE_DEFINED__

/* interface ITCallInfo2 */
/* [object][dual][helpstring][uuid] */ 


EXTERN_C const IID IID_ITCallInfo2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("94D70CA6-7AB0-4daa-81CA-B8F8643FAEC1")
    ITCallInfo2 : public ITCallInfo
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_EventFilter( 
            /* [in] */ TAPI_EVENT TapiEvent,
            /* [in] */ long lSubEvent,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pEnable) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_EventFilter( 
            /* [in] */ TAPI_EVENT TapiEvent,
            /* [in] */ long lSubEvent,
            /* [in] */ VARIANT_BOOL bEnable) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITCallInfo2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITCallInfo2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITCallInfo2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITCallInfo2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITCallInfo2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITCallInfo2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITCallInfo2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITCallInfo2 * This,
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
        
        DECLSPEC_XFGVIRT(ITCallInfo, get_Address)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Address )( 
            __RPC__in ITCallInfo2 * This,
            /* [retval][out] */ __RPC__deref_out_opt ITAddress **ppAddress);
        
        DECLSPEC_XFGVIRT(ITCallInfo, get_CallState)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CallState )( 
            __RPC__in ITCallInfo2 * This,
            /* [retval][out] */ __RPC__out CALL_STATE *pCallState);
        
        DECLSPEC_XFGVIRT(ITCallInfo, get_Privilege)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Privilege )( 
            __RPC__in ITCallInfo2 * This,
            /* [retval][out] */ __RPC__out CALL_PRIVILEGE *pPrivilege);
        
        DECLSPEC_XFGVIRT(ITCallInfo, get_CallHub)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CallHub )( 
            __RPC__in ITCallInfo2 * This,
            /* [retval][out] */ __RPC__deref_out_opt ITCallHub **ppCallHub);
        
        DECLSPEC_XFGVIRT(ITCallInfo, get_CallInfoLong)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CallInfoLong )( 
            __RPC__in ITCallInfo2 * This,
            /* [in] */ CALLINFO_LONG CallInfoLong,
            /* [retval][out] */ __RPC__out long *plCallInfoLongVal);
        
        DECLSPEC_XFGVIRT(ITCallInfo, put_CallInfoLong)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_CallInfoLong )( 
            __RPC__in ITCallInfo2 * This,
            /* [in] */ CALLINFO_LONG CallInfoLong,
            /* [in] */ long lCallInfoLongVal);
        
        DECLSPEC_XFGVIRT(ITCallInfo, get_CallInfoString)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CallInfoString )( 
            __RPC__in ITCallInfo2 * This,
            /* [in] */ CALLINFO_STRING CallInfoString,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppCallInfoString);
        
        DECLSPEC_XFGVIRT(ITCallInfo, put_CallInfoString)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_CallInfoString )( 
            __RPC__in ITCallInfo2 * This,
            /* [in] */ CALLINFO_STRING CallInfoString,
            /* [in] */ __RPC__in BSTR pCallInfoString);
        
        DECLSPEC_XFGVIRT(ITCallInfo, get_CallInfoBuffer)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CallInfoBuffer )( 
            __RPC__in ITCallInfo2 * This,
            /* [in] */ CALLINFO_BUFFER CallInfoBuffer,
            /* [retval][out] */ __RPC__out VARIANT *ppCallInfoBuffer);
        
        DECLSPEC_XFGVIRT(ITCallInfo, put_CallInfoBuffer)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_CallInfoBuffer )( 
            __RPC__in ITCallInfo2 * This,
            /* [in] */ CALLINFO_BUFFER CallInfoBuffer,
            /* [in] */ VARIANT pCallInfoBuffer);
        
        DECLSPEC_XFGVIRT(ITCallInfo, GetCallInfoBuffer)
        /* [hidden][id] */ HRESULT ( STDMETHODCALLTYPE *GetCallInfoBuffer )( 
            __RPC__in ITCallInfo2 * This,
            /* [in] */ CALLINFO_BUFFER CallInfoBuffer,
            /* [out] */ __RPC__out DWORD *pdwSize,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pdwSize) BYTE **ppCallInfoBuffer);
        
        DECLSPEC_XFGVIRT(ITCallInfo, SetCallInfoBuffer)
        /* [hidden][id] */ HRESULT ( STDMETHODCALLTYPE *SetCallInfoBuffer )( 
            __RPC__in ITCallInfo2 * This,
            /* [in] */ CALLINFO_BUFFER CallInfoBuffer,
            /* [in] */ DWORD dwSize,
            /* [size_is][in] */ __RPC__in_ecount_full(dwSize) BYTE *pCallInfoBuffer);
        
        DECLSPEC_XFGVIRT(ITCallInfo, ReleaseUserUserInfo)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ReleaseUserUserInfo )( 
            __RPC__in ITCallInfo2 * This);
        
        DECLSPEC_XFGVIRT(ITCallInfo2, get_EventFilter)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EventFilter )( 
            __RPC__in ITCallInfo2 * This,
            /* [in] */ TAPI_EVENT TapiEvent,
            /* [in] */ long lSubEvent,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pEnable);
        
        DECLSPEC_XFGVIRT(ITCallInfo2, put_EventFilter)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_EventFilter )( 
            __RPC__in ITCallInfo2 * This,
            /* [in] */ TAPI_EVENT TapiEvent,
            /* [in] */ long lSubEvent,
            /* [in] */ VARIANT_BOOL bEnable);
        
        END_INTERFACE
    } ITCallInfo2Vtbl;

    interface ITCallInfo2
    {
        CONST_VTBL struct ITCallInfo2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITCallInfo2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITCallInfo2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITCallInfo2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITCallInfo2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITCallInfo2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITCallInfo2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITCallInfo2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITCallInfo2_get_Address(This,ppAddress)	\
    ( (This)->lpVtbl -> get_Address(This,ppAddress) ) 

#define ITCallInfo2_get_CallState(This,pCallState)	\
    ( (This)->lpVtbl -> get_CallState(This,pCallState) ) 

#define ITCallInfo2_get_Privilege(This,pPrivilege)	\
    ( (This)->lpVtbl -> get_Privilege(This,pPrivilege) ) 

#define ITCallInfo2_get_CallHub(This,ppCallHub)	\
    ( (This)->lpVtbl -> get_CallHub(This,ppCallHub) ) 

#define ITCallInfo2_get_CallInfoLong(This,CallInfoLong,plCallInfoLongVal)	\
    ( (This)->lpVtbl -> get_CallInfoLong(This,CallInfoLong,plCallInfoLongVal) ) 

#define ITCallInfo2_put_CallInfoLong(This,CallInfoLong,lCallInfoLongVal)	\
    ( (This)->lpVtbl -> put_CallInfoLong(This,CallInfoLong,lCallInfoLongVal) ) 

#define ITCallInfo2_get_CallInfoString(This,CallInfoString,ppCallInfoString)	\
    ( (This)->lpVtbl -> get_CallInfoString(This,CallInfoString,ppCallInfoString) ) 

#define ITCallInfo2_put_CallInfoString(This,CallInfoString,pCallInfoString)	\
    ( (This)->lpVtbl -> put_CallInfoString(This,CallInfoString,pCallInfoString) ) 

#define ITCallInfo2_get_CallInfoBuffer(This,CallInfoBuffer,ppCallInfoBuffer)	\
    ( (This)->lpVtbl -> get_CallInfoBuffer(This,CallInfoBuffer,ppCallInfoBuffer) ) 

#define ITCallInfo2_put_CallInfoBuffer(This,CallInfoBuffer,pCallInfoBuffer)	\
    ( (This)->lpVtbl -> put_CallInfoBuffer(This,CallInfoBuffer,pCallInfoBuffer) ) 

#define ITCallInfo2_GetCallInfoBuffer(This,CallInfoBuffer,pdwSize,ppCallInfoBuffer)	\
    ( (This)->lpVtbl -> GetCallInfoBuffer(This,CallInfoBuffer,pdwSize,ppCallInfoBuffer) ) 

#define ITCallInfo2_SetCallInfoBuffer(This,CallInfoBuffer,dwSize,pCallInfoBuffer)	\
    ( (This)->lpVtbl -> SetCallInfoBuffer(This,CallInfoBuffer,dwSize,pCallInfoBuffer) ) 

#define ITCallInfo2_ReleaseUserUserInfo(This)	\
    ( (This)->lpVtbl -> ReleaseUserUserInfo(This) ) 


#define ITCallInfo2_get_EventFilter(This,TapiEvent,lSubEvent,pEnable)	\
    ( (This)->lpVtbl -> get_EventFilter(This,TapiEvent,lSubEvent,pEnable) ) 

#define ITCallInfo2_put_EventFilter(This,TapiEvent,lSubEvent,bEnable)	\
    ( (This)->lpVtbl -> put_EventFilter(This,TapiEvent,lSubEvent,bEnable) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITCallInfo2_INTERFACE_DEFINED__ */


#ifndef __ITTerminal_INTERFACE_DEFINED__
#define __ITTerminal_INTERFACE_DEFINED__

/* interface ITTerminal */
/* [object][dual][helpstring][uuid] */ 


EXTERN_C const IID IID_ITTerminal;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B1EFC38A-9355-11d0-835C-00AA003CCABD")
    ITTerminal : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_State( 
            /* [retval][out] */ __RPC__out TERMINAL_STATE *pTerminalState) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TerminalType( 
            /* [retval][out] */ __RPC__out TERMINAL_TYPE *pType) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TerminalClass( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppTerminalClass) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MediaType( 
            /* [retval][out] */ __RPC__out long *plMediaType) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Direction( 
            /* [retval][out] */ __RPC__out TERMINAL_DIRECTION *pDirection) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITTerminalVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITTerminal * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITTerminal * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITTerminal * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITTerminal * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITTerminal * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITTerminal * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITTerminal * This,
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
        
        DECLSPEC_XFGVIRT(ITTerminal, get_Name)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in ITTerminal * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppName);
        
        DECLSPEC_XFGVIRT(ITTerminal, get_State)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_State )( 
            __RPC__in ITTerminal * This,
            /* [retval][out] */ __RPC__out TERMINAL_STATE *pTerminalState);
        
        DECLSPEC_XFGVIRT(ITTerminal, get_TerminalType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TerminalType )( 
            __RPC__in ITTerminal * This,
            /* [retval][out] */ __RPC__out TERMINAL_TYPE *pType);
        
        DECLSPEC_XFGVIRT(ITTerminal, get_TerminalClass)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TerminalClass )( 
            __RPC__in ITTerminal * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppTerminalClass);
        
        DECLSPEC_XFGVIRT(ITTerminal, get_MediaType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MediaType )( 
            __RPC__in ITTerminal * This,
            /* [retval][out] */ __RPC__out long *plMediaType);
        
        DECLSPEC_XFGVIRT(ITTerminal, get_Direction)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Direction )( 
            __RPC__in ITTerminal * This,
            /* [retval][out] */ __RPC__out TERMINAL_DIRECTION *pDirection);
        
        END_INTERFACE
    } ITTerminalVtbl;

    interface ITTerminal
    {
        CONST_VTBL struct ITTerminalVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITTerminal_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITTerminal_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITTerminal_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITTerminal_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITTerminal_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITTerminal_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITTerminal_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITTerminal_get_Name(This,ppName)	\
    ( (This)->lpVtbl -> get_Name(This,ppName) ) 

#define ITTerminal_get_State(This,pTerminalState)	\
    ( (This)->lpVtbl -> get_State(This,pTerminalState) ) 

#define ITTerminal_get_TerminalType(This,pType)	\
    ( (This)->lpVtbl -> get_TerminalType(This,pType) ) 

#define ITTerminal_get_TerminalClass(This,ppTerminalClass)	\
    ( (This)->lpVtbl -> get_TerminalClass(This,ppTerminalClass) ) 

#define ITTerminal_get_MediaType(This,plMediaType)	\
    ( (This)->lpVtbl -> get_MediaType(This,plMediaType) ) 

#define ITTerminal_get_Direction(This,pDirection)	\
    ( (This)->lpVtbl -> get_Direction(This,pDirection) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITTerminal_INTERFACE_DEFINED__ */


#ifndef __ITMultiTrackTerminal_INTERFACE_DEFINED__
#define __ITMultiTrackTerminal_INTERFACE_DEFINED__

/* interface ITMultiTrackTerminal */
/* [helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_ITMultiTrackTerminal;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("FE040091-ADE8-4072-95C9-BF7DE8C54B44")
    ITMultiTrackTerminal : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TrackTerminals( 
            /* [retval][out] */ __RPC__out VARIANT *pVariant) = 0;
        
        virtual /* [helpstring][hidden][id] */ HRESULT STDMETHODCALLTYPE EnumerateTrackTerminals( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumTerminal **ppEnumTerminal) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateTrackTerminal( 
            /* [in] */ long MediaType,
            /* [in] */ TERMINAL_DIRECTION TerminalDirection,
            /* [retval][out] */ __RPC__deref_out_opt ITTerminal **ppTerminal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MediaTypesInUse( 
            /* [retval][out] */ __RPC__out long *plMediaTypesInUse) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DirectionsInUse( 
            /* [retval][out] */ __RPC__out TERMINAL_DIRECTION *plDirectionsInUsed) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE RemoveTrackTerminal( 
            /* [in] */ __RPC__in_opt ITTerminal *pTrackTerminalToRemove) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITMultiTrackTerminalVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITMultiTrackTerminal * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITMultiTrackTerminal * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITMultiTrackTerminal * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITMultiTrackTerminal * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITMultiTrackTerminal * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITMultiTrackTerminal * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITMultiTrackTerminal * This,
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
        
        DECLSPEC_XFGVIRT(ITMultiTrackTerminal, get_TrackTerminals)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TrackTerminals )( 
            __RPC__in ITMultiTrackTerminal * This,
            /* [retval][out] */ __RPC__out VARIANT *pVariant);
        
        DECLSPEC_XFGVIRT(ITMultiTrackTerminal, EnumerateTrackTerminals)
        /* [helpstring][hidden][id] */ HRESULT ( STDMETHODCALLTYPE *EnumerateTrackTerminals )( 
            __RPC__in ITMultiTrackTerminal * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumTerminal **ppEnumTerminal);
        
        DECLSPEC_XFGVIRT(ITMultiTrackTerminal, CreateTrackTerminal)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateTrackTerminal )( 
            __RPC__in ITMultiTrackTerminal * This,
            /* [in] */ long MediaType,
            /* [in] */ TERMINAL_DIRECTION TerminalDirection,
            /* [retval][out] */ __RPC__deref_out_opt ITTerminal **ppTerminal);
        
        DECLSPEC_XFGVIRT(ITMultiTrackTerminal, get_MediaTypesInUse)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MediaTypesInUse )( 
            __RPC__in ITMultiTrackTerminal * This,
            /* [retval][out] */ __RPC__out long *plMediaTypesInUse);
        
        DECLSPEC_XFGVIRT(ITMultiTrackTerminal, get_DirectionsInUse)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DirectionsInUse )( 
            __RPC__in ITMultiTrackTerminal * This,
            /* [retval][out] */ __RPC__out TERMINAL_DIRECTION *plDirectionsInUsed);
        
        DECLSPEC_XFGVIRT(ITMultiTrackTerminal, RemoveTrackTerminal)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RemoveTrackTerminal )( 
            __RPC__in ITMultiTrackTerminal * This,
            /* [in] */ __RPC__in_opt ITTerminal *pTrackTerminalToRemove);
        
        END_INTERFACE
    } ITMultiTrackTerminalVtbl;

    interface ITMultiTrackTerminal
    {
        CONST_VTBL struct ITMultiTrackTerminalVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITMultiTrackTerminal_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITMultiTrackTerminal_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITMultiTrackTerminal_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITMultiTrackTerminal_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITMultiTrackTerminal_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITMultiTrackTerminal_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITMultiTrackTerminal_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITMultiTrackTerminal_get_TrackTerminals(This,pVariant)	\
    ( (This)->lpVtbl -> get_TrackTerminals(This,pVariant) ) 

#define ITMultiTrackTerminal_EnumerateTrackTerminals(This,ppEnumTerminal)	\
    ( (This)->lpVtbl -> EnumerateTrackTerminals(This,ppEnumTerminal) ) 

#define ITMultiTrackTerminal_CreateTrackTerminal(This,MediaType,TerminalDirection,ppTerminal)	\
    ( (This)->lpVtbl -> CreateTrackTerminal(This,MediaType,TerminalDirection,ppTerminal) ) 

#define ITMultiTrackTerminal_get_MediaTypesInUse(This,plMediaTypesInUse)	\
    ( (This)->lpVtbl -> get_MediaTypesInUse(This,plMediaTypesInUse) ) 

#define ITMultiTrackTerminal_get_DirectionsInUse(This,plDirectionsInUsed)	\
    ( (This)->lpVtbl -> get_DirectionsInUse(This,plDirectionsInUsed) ) 

#define ITMultiTrackTerminal_RemoveTrackTerminal(This,pTrackTerminalToRemove)	\
    ( (This)->lpVtbl -> RemoveTrackTerminal(This,pTrackTerminalToRemove) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITMultiTrackTerminal_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_tapi3if_0000_0017 */
/* [local] */ 

typedef 
enum TERMINAL_MEDIA_STATE
    {
        TMS_IDLE	= 0,
        TMS_ACTIVE	= ( TMS_IDLE + 1 ) ,
        TMS_PAUSED	= ( TMS_ACTIVE + 1 ) ,
        TMS_LASTITEM	= TMS_PAUSED
    } 	TERMINAL_MEDIA_STATE;

typedef 
enum FT_STATE_EVENT_CAUSE
    {
        FTEC_NORMAL	= 0,
        FTEC_END_OF_FILE	= ( FTEC_NORMAL + 1 ) ,
        FTEC_READ_ERROR	= ( FTEC_END_OF_FILE + 1 ) ,
        FTEC_WRITE_ERROR	= ( FTEC_READ_ERROR + 1 ) 
    } 	FT_STATE_EVENT_CAUSE;



extern RPC_IF_HANDLE __MIDL_itf_tapi3if_0000_0017_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_tapi3if_0000_0017_v0_0_s_ifspec;

#ifndef __ITFileTrack_INTERFACE_DEFINED__
#define __ITFileTrack_INTERFACE_DEFINED__

/* interface ITFileTrack */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_ITFileTrack;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("31CA6EA9-C08A-4bea-8811-8E9C1BA3EA3A")
    ITFileTrack : public IDispatch
    {
    public:
        virtual /* [helpstring][hidden][id][propget] */ HRESULT STDMETHODCALLTYPE get_Format( 
            /* [retval][out] */ __RPC__deref_out_opt AM_MEDIA_TYPE **ppmt) = 0;
        
        virtual /* [helpstring][hidden][id][propput] */ HRESULT STDMETHODCALLTYPE put_Format( 
            /* [in] */ __RPC__in const AM_MEDIA_TYPE *pmt) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ControllingTerminal( 
            /* [retval][out] */ __RPC__deref_out_opt ITTerminal **ppControllingTerminal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_AudioFormatForScripting( 
            /* [retval][out] */ __RPC__deref_out_opt ITScriptableAudioFormat **ppAudioFormat) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_AudioFormatForScripting( 
            /* [in] */ __RPC__in_opt ITScriptableAudioFormat *pAudioFormat) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_EmptyAudioFormatForScripting( 
            /* [retval][out] */ __RPC__deref_out_opt ITScriptableAudioFormat **ppAudioFormat) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITFileTrackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITFileTrack * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITFileTrack * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITFileTrack * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITFileTrack * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITFileTrack * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITFileTrack * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITFileTrack * This,
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
        
        DECLSPEC_XFGVIRT(ITFileTrack, get_Format)
        /* [helpstring][hidden][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Format )( 
            __RPC__in ITFileTrack * This,
            /* [retval][out] */ __RPC__deref_out_opt AM_MEDIA_TYPE **ppmt);
        
        DECLSPEC_XFGVIRT(ITFileTrack, put_Format)
        /* [helpstring][hidden][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Format )( 
            __RPC__in ITFileTrack * This,
            /* [in] */ __RPC__in const AM_MEDIA_TYPE *pmt);
        
        DECLSPEC_XFGVIRT(ITFileTrack, get_ControllingTerminal)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ControllingTerminal )( 
            __RPC__in ITFileTrack * This,
            /* [retval][out] */ __RPC__deref_out_opt ITTerminal **ppControllingTerminal);
        
        DECLSPEC_XFGVIRT(ITFileTrack, get_AudioFormatForScripting)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AudioFormatForScripting )( 
            __RPC__in ITFileTrack * This,
            /* [retval][out] */ __RPC__deref_out_opt ITScriptableAudioFormat **ppAudioFormat);
        
        DECLSPEC_XFGVIRT(ITFileTrack, put_AudioFormatForScripting)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_AudioFormatForScripting )( 
            __RPC__in ITFileTrack * This,
            /* [in] */ __RPC__in_opt ITScriptableAudioFormat *pAudioFormat);
        
        DECLSPEC_XFGVIRT(ITFileTrack, get_EmptyAudioFormatForScripting)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EmptyAudioFormatForScripting )( 
            __RPC__in ITFileTrack * This,
            /* [retval][out] */ __RPC__deref_out_opt ITScriptableAudioFormat **ppAudioFormat);
        
        END_INTERFACE
    } ITFileTrackVtbl;

    interface ITFileTrack
    {
        CONST_VTBL struct ITFileTrackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITFileTrack_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITFileTrack_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITFileTrack_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITFileTrack_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITFileTrack_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITFileTrack_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITFileTrack_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITFileTrack_get_Format(This,ppmt)	\
    ( (This)->lpVtbl -> get_Format(This,ppmt) ) 

#define ITFileTrack_put_Format(This,pmt)	\
    ( (This)->lpVtbl -> put_Format(This,pmt) ) 

#define ITFileTrack_get_ControllingTerminal(This,ppControllingTerminal)	\
    ( (This)->lpVtbl -> get_ControllingTerminal(This,ppControllingTerminal) ) 

#define ITFileTrack_get_AudioFormatForScripting(This,ppAudioFormat)	\
    ( (This)->lpVtbl -> get_AudioFormatForScripting(This,ppAudioFormat) ) 

#define ITFileTrack_put_AudioFormatForScripting(This,pAudioFormat)	\
    ( (This)->lpVtbl -> put_AudioFormatForScripting(This,pAudioFormat) ) 

#define ITFileTrack_get_EmptyAudioFormatForScripting(This,ppAudioFormat)	\
    ( (This)->lpVtbl -> get_EmptyAudioFormatForScripting(This,ppAudioFormat) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITFileTrack_INTERFACE_DEFINED__ */


#ifndef __ITMediaPlayback_INTERFACE_DEFINED__
#define __ITMediaPlayback_INTERFACE_DEFINED__

/* interface ITMediaPlayback */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_ITMediaPlayback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("627E8AE6-AE4C-4a69-BB63-2AD625404B77")
    ITMediaPlayback : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_PlayList( 
            /* [in] */ VARIANTARG PlayListVariant) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PlayList( 
            /* [retval][out] */ __RPC__out VARIANTARG *pPlayListVariant) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITMediaPlaybackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITMediaPlayback * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITMediaPlayback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITMediaPlayback * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITMediaPlayback * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITMediaPlayback * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITMediaPlayback * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITMediaPlayback * This,
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
        
        DECLSPEC_XFGVIRT(ITMediaPlayback, put_PlayList)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PlayList )( 
            __RPC__in ITMediaPlayback * This,
            /* [in] */ VARIANTARG PlayListVariant);
        
        DECLSPEC_XFGVIRT(ITMediaPlayback, get_PlayList)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PlayList )( 
            __RPC__in ITMediaPlayback * This,
            /* [retval][out] */ __RPC__out VARIANTARG *pPlayListVariant);
        
        END_INTERFACE
    } ITMediaPlaybackVtbl;

    interface ITMediaPlayback
    {
        CONST_VTBL struct ITMediaPlaybackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITMediaPlayback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITMediaPlayback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITMediaPlayback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITMediaPlayback_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITMediaPlayback_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITMediaPlayback_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITMediaPlayback_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITMediaPlayback_put_PlayList(This,PlayListVariant)	\
    ( (This)->lpVtbl -> put_PlayList(This,PlayListVariant) ) 

#define ITMediaPlayback_get_PlayList(This,pPlayListVariant)	\
    ( (This)->lpVtbl -> get_PlayList(This,pPlayListVariant) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITMediaPlayback_INTERFACE_DEFINED__ */


#ifndef __ITMediaRecord_INTERFACE_DEFINED__
#define __ITMediaRecord_INTERFACE_DEFINED__

/* interface ITMediaRecord */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_ITMediaRecord;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F5DD4592-5476-4cc1-9D4D-FAD3EEFE7DB2")
    ITMediaRecord : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_FileName( 
            /* [in] */ __RPC__in BSTR bstrFileName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_FileName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrFileName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITMediaRecordVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITMediaRecord * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITMediaRecord * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITMediaRecord * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITMediaRecord * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITMediaRecord * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITMediaRecord * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITMediaRecord * This,
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
        
        DECLSPEC_XFGVIRT(ITMediaRecord, put_FileName)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_FileName )( 
            __RPC__in ITMediaRecord * This,
            /* [in] */ __RPC__in BSTR bstrFileName);
        
        DECLSPEC_XFGVIRT(ITMediaRecord, get_FileName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FileName )( 
            __RPC__in ITMediaRecord * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrFileName);
        
        END_INTERFACE
    } ITMediaRecordVtbl;

    interface ITMediaRecord
    {
        CONST_VTBL struct ITMediaRecordVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITMediaRecord_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITMediaRecord_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITMediaRecord_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITMediaRecord_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITMediaRecord_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITMediaRecord_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITMediaRecord_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITMediaRecord_put_FileName(This,bstrFileName)	\
    ( (This)->lpVtbl -> put_FileName(This,bstrFileName) ) 

#define ITMediaRecord_get_FileName(This,pbstrFileName)	\
    ( (This)->lpVtbl -> get_FileName(This,pbstrFileName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITMediaRecord_INTERFACE_DEFINED__ */


#ifndef __ITMediaControl_INTERFACE_DEFINED__
#define __ITMediaControl_INTERFACE_DEFINED__

/* interface ITMediaControl */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_ITMediaControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C445DDE8-5199-4bc7-9807-5FFB92E42E09")
    ITMediaControl : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Start( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Stop( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Pause( void) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MediaState( 
            /* [retval][out] */ __RPC__out TERMINAL_MEDIA_STATE *pTerminalMediaState) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITMediaControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITMediaControl * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITMediaControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITMediaControl * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITMediaControl * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITMediaControl * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITMediaControl * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITMediaControl * This,
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
        
        DECLSPEC_XFGVIRT(ITMediaControl, Start)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Start )( 
            __RPC__in ITMediaControl * This);
        
        DECLSPEC_XFGVIRT(ITMediaControl, Stop)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Stop )( 
            __RPC__in ITMediaControl * This);
        
        DECLSPEC_XFGVIRT(ITMediaControl, Pause)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Pause )( 
            __RPC__in ITMediaControl * This);
        
        DECLSPEC_XFGVIRT(ITMediaControl, get_MediaState)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MediaState )( 
            __RPC__in ITMediaControl * This,
            /* [retval][out] */ __RPC__out TERMINAL_MEDIA_STATE *pTerminalMediaState);
        
        END_INTERFACE
    } ITMediaControlVtbl;

    interface ITMediaControl
    {
        CONST_VTBL struct ITMediaControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITMediaControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITMediaControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITMediaControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITMediaControl_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITMediaControl_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITMediaControl_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITMediaControl_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITMediaControl_Start(This)	\
    ( (This)->lpVtbl -> Start(This) ) 

#define ITMediaControl_Stop(This)	\
    ( (This)->lpVtbl -> Stop(This) ) 

#define ITMediaControl_Pause(This)	\
    ( (This)->lpVtbl -> Pause(This) ) 

#define ITMediaControl_get_MediaState(This,pTerminalMediaState)	\
    ( (This)->lpVtbl -> get_MediaState(This,pTerminalMediaState) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITMediaControl_INTERFACE_DEFINED__ */


#ifndef __ITBasicAudioTerminal_INTERFACE_DEFINED__
#define __ITBasicAudioTerminal_INTERFACE_DEFINED__

/* interface ITBasicAudioTerminal */
/* [object][dual][helpstring][uuid] */ 


EXTERN_C const IID IID_ITBasicAudioTerminal;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B1EFC38D-9355-11d0-835C-00AA003CCABD")
    ITBasicAudioTerminal : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Volume( 
            /* [in] */ long lVolume) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Volume( 
            /* [retval][out] */ __RPC__out long *plVolume) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Balance( 
            /* [in] */ long lBalance) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Balance( 
            /* [retval][out] */ __RPC__out long *plBalance) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITBasicAudioTerminalVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITBasicAudioTerminal * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITBasicAudioTerminal * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITBasicAudioTerminal * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITBasicAudioTerminal * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITBasicAudioTerminal * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITBasicAudioTerminal * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITBasicAudioTerminal * This,
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
        
        DECLSPEC_XFGVIRT(ITBasicAudioTerminal, put_Volume)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Volume )( 
            __RPC__in ITBasicAudioTerminal * This,
            /* [in] */ long lVolume);
        
        DECLSPEC_XFGVIRT(ITBasicAudioTerminal, get_Volume)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Volume )( 
            __RPC__in ITBasicAudioTerminal * This,
            /* [retval][out] */ __RPC__out long *plVolume);
        
        DECLSPEC_XFGVIRT(ITBasicAudioTerminal, put_Balance)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Balance )( 
            __RPC__in ITBasicAudioTerminal * This,
            /* [in] */ long lBalance);
        
        DECLSPEC_XFGVIRT(ITBasicAudioTerminal, get_Balance)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Balance )( 
            __RPC__in ITBasicAudioTerminal * This,
            /* [retval][out] */ __RPC__out long *plBalance);
        
        END_INTERFACE
    } ITBasicAudioTerminalVtbl;

    interface ITBasicAudioTerminal
    {
        CONST_VTBL struct ITBasicAudioTerminalVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITBasicAudioTerminal_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITBasicAudioTerminal_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITBasicAudioTerminal_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITBasicAudioTerminal_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITBasicAudioTerminal_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITBasicAudioTerminal_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITBasicAudioTerminal_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITBasicAudioTerminal_put_Volume(This,lVolume)	\
    ( (This)->lpVtbl -> put_Volume(This,lVolume) ) 

#define ITBasicAudioTerminal_get_Volume(This,plVolume)	\
    ( (This)->lpVtbl -> get_Volume(This,plVolume) ) 

#define ITBasicAudioTerminal_put_Balance(This,lBalance)	\
    ( (This)->lpVtbl -> put_Balance(This,lBalance) ) 

#define ITBasicAudioTerminal_get_Balance(This,plBalance)	\
    ( (This)->lpVtbl -> get_Balance(This,plBalance) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITBasicAudioTerminal_INTERFACE_DEFINED__ */


#ifndef __ITStaticAudioTerminal_INTERFACE_DEFINED__
#define __ITStaticAudioTerminal_INTERFACE_DEFINED__

/* interface ITStaticAudioTerminal */
/* [object][dual][helpstring][uuid] */ 


EXTERN_C const IID IID_ITStaticAudioTerminal;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A86B7871-D14C-48e6-922E-A8D15F984800")
    ITStaticAudioTerminal : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_WaveId( 
            /* [retval][out] */ __RPC__out long *plWaveId) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITStaticAudioTerminalVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITStaticAudioTerminal * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITStaticAudioTerminal * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITStaticAudioTerminal * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITStaticAudioTerminal * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITStaticAudioTerminal * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITStaticAudioTerminal * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITStaticAudioTerminal * This,
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
        
        DECLSPEC_XFGVIRT(ITStaticAudioTerminal, get_WaveId)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_WaveId )( 
            __RPC__in ITStaticAudioTerminal * This,
            /* [retval][out] */ __RPC__out long *plWaveId);
        
        END_INTERFACE
    } ITStaticAudioTerminalVtbl;

    interface ITStaticAudioTerminal
    {
        CONST_VTBL struct ITStaticAudioTerminalVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITStaticAudioTerminal_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITStaticAudioTerminal_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITStaticAudioTerminal_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITStaticAudioTerminal_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITStaticAudioTerminal_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITStaticAudioTerminal_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITStaticAudioTerminal_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITStaticAudioTerminal_get_WaveId(This,plWaveId)	\
    ( (This)->lpVtbl -> get_WaveId(This,plWaveId) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITStaticAudioTerminal_INTERFACE_DEFINED__ */


#ifndef __ITCallHub_INTERFACE_DEFINED__
#define __ITCallHub_INTERFACE_DEFINED__

/* interface ITCallHub */
/* [object][dual][helpstring][uuid] */ 


EXTERN_C const IID IID_ITCallHub;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A3C1544E-5B92-11d1-8F4E-00C04FB6809F")
    ITCallHub : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Clear( void) = 0;
        
        virtual /* [helpstring][hidden][id] */ HRESULT STDMETHODCALLTYPE EnumerateCalls( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumCall **ppEnumCall) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Calls( 
            /* [retval][out] */ __RPC__out VARIANT *pCalls) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_NumCalls( 
            /* [retval][out] */ __RPC__out long *plCalls) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_State( 
            /* [retval][out] */ __RPC__out CALLHUB_STATE *pState) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITCallHubVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITCallHub * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITCallHub * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITCallHub * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITCallHub * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITCallHub * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITCallHub * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITCallHub * This,
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
        
        DECLSPEC_XFGVIRT(ITCallHub, Clear)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Clear )( 
            __RPC__in ITCallHub * This);
        
        DECLSPEC_XFGVIRT(ITCallHub, EnumerateCalls)
        /* [helpstring][hidden][id] */ HRESULT ( STDMETHODCALLTYPE *EnumerateCalls )( 
            __RPC__in ITCallHub * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumCall **ppEnumCall);
        
        DECLSPEC_XFGVIRT(ITCallHub, get_Calls)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Calls )( 
            __RPC__in ITCallHub * This,
            /* [retval][out] */ __RPC__out VARIANT *pCalls);
        
        DECLSPEC_XFGVIRT(ITCallHub, get_NumCalls)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NumCalls )( 
            __RPC__in ITCallHub * This,
            /* [retval][out] */ __RPC__out long *plCalls);
        
        DECLSPEC_XFGVIRT(ITCallHub, get_State)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_State )( 
            __RPC__in ITCallHub * This,
            /* [retval][out] */ __RPC__out CALLHUB_STATE *pState);
        
        END_INTERFACE
    } ITCallHubVtbl;

    interface ITCallHub
    {
        CONST_VTBL struct ITCallHubVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITCallHub_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITCallHub_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITCallHub_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITCallHub_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITCallHub_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITCallHub_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITCallHub_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITCallHub_Clear(This)	\
    ( (This)->lpVtbl -> Clear(This) ) 

#define ITCallHub_EnumerateCalls(This,ppEnumCall)	\
    ( (This)->lpVtbl -> EnumerateCalls(This,ppEnumCall) ) 

#define ITCallHub_get_Calls(This,pCalls)	\
    ( (This)->lpVtbl -> get_Calls(This,pCalls) ) 

#define ITCallHub_get_NumCalls(This,plCalls)	\
    ( (This)->lpVtbl -> get_NumCalls(This,plCalls) ) 

#define ITCallHub_get_State(This,pState)	\
    ( (This)->lpVtbl -> get_State(This,pState) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITCallHub_INTERFACE_DEFINED__ */


#ifndef __ITLegacyAddressMediaControl_INTERFACE_DEFINED__
#define __ITLegacyAddressMediaControl_INTERFACE_DEFINED__

/* interface ITLegacyAddressMediaControl */
/* [object][unique][hidden][helpstring][uuid] */ 


EXTERN_C const IID IID_ITLegacyAddressMediaControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("AB493640-4C0B-11D2-A046-00C04FB6809F")
    ITLegacyAddressMediaControl : public IUnknown
    {
    public:
        virtual /* [hidden][id] */ HRESULT STDMETHODCALLTYPE GetID( 
            /* [in] */ __RPC__in BSTR pDeviceClass,
            /* [out] */ __RPC__out DWORD *pdwSize,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pdwSize) BYTE **ppDeviceID) = 0;
        
        virtual /* [hidden][id] */ HRESULT STDMETHODCALLTYPE GetDevConfig( 
            /* [in] */ __RPC__in BSTR pDeviceClass,
            /* [out] */ __RPC__out DWORD *pdwSize,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pdwSize) BYTE **ppDeviceConfig) = 0;
        
        virtual /* [hidden][id] */ HRESULT STDMETHODCALLTYPE SetDevConfig( 
            /* [in] */ __RPC__in BSTR pDeviceClass,
            /* [in] */ DWORD dwSize,
            /* [size_is][in] */ __RPC__in_ecount_full(dwSize) BYTE *pDeviceConfig) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITLegacyAddressMediaControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITLegacyAddressMediaControl * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITLegacyAddressMediaControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITLegacyAddressMediaControl * This);
        
        DECLSPEC_XFGVIRT(ITLegacyAddressMediaControl, GetID)
        /* [hidden][id] */ HRESULT ( STDMETHODCALLTYPE *GetID )( 
            __RPC__in ITLegacyAddressMediaControl * This,
            /* [in] */ __RPC__in BSTR pDeviceClass,
            /* [out] */ __RPC__out DWORD *pdwSize,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pdwSize) BYTE **ppDeviceID);
        
        DECLSPEC_XFGVIRT(ITLegacyAddressMediaControl, GetDevConfig)
        /* [hidden][id] */ HRESULT ( STDMETHODCALLTYPE *GetDevConfig )( 
            __RPC__in ITLegacyAddressMediaControl * This,
            /* [in] */ __RPC__in BSTR pDeviceClass,
            /* [out] */ __RPC__out DWORD *pdwSize,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pdwSize) BYTE **ppDeviceConfig);
        
        DECLSPEC_XFGVIRT(ITLegacyAddressMediaControl, SetDevConfig)
        /* [hidden][id] */ HRESULT ( STDMETHODCALLTYPE *SetDevConfig )( 
            __RPC__in ITLegacyAddressMediaControl * This,
            /* [in] */ __RPC__in BSTR pDeviceClass,
            /* [in] */ DWORD dwSize,
            /* [size_is][in] */ __RPC__in_ecount_full(dwSize) BYTE *pDeviceConfig);
        
        END_INTERFACE
    } ITLegacyAddressMediaControlVtbl;

    interface ITLegacyAddressMediaControl
    {
        CONST_VTBL struct ITLegacyAddressMediaControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITLegacyAddressMediaControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITLegacyAddressMediaControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITLegacyAddressMediaControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITLegacyAddressMediaControl_GetID(This,pDeviceClass,pdwSize,ppDeviceID)	\
    ( (This)->lpVtbl -> GetID(This,pDeviceClass,pdwSize,ppDeviceID) ) 

#define ITLegacyAddressMediaControl_GetDevConfig(This,pDeviceClass,pdwSize,ppDeviceConfig)	\
    ( (This)->lpVtbl -> GetDevConfig(This,pDeviceClass,pdwSize,ppDeviceConfig) ) 

#define ITLegacyAddressMediaControl_SetDevConfig(This,pDeviceClass,dwSize,pDeviceConfig)	\
    ( (This)->lpVtbl -> SetDevConfig(This,pDeviceClass,dwSize,pDeviceConfig) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITLegacyAddressMediaControl_INTERFACE_DEFINED__ */


#ifndef __ITPrivateEvent_INTERFACE_DEFINED__
#define __ITPrivateEvent_INTERFACE_DEFINED__

/* interface ITPrivateEvent */
/* [object][dual][helpstring][uuid] */ 


EXTERN_C const IID IID_ITPrivateEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0e269cd0-10d4-4121-9c22-9c85d625650d")
    ITPrivateEvent : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Address( 
            /* [retval][out] */ __RPC__deref_out_opt ITAddress **ppAddress) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Call( 
            /* [retval][out] */ __RPC__deref_out_opt ITCallInfo **ppCallInfo) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CallHub( 
            /* [retval][out] */ __RPC__deref_out_opt ITCallHub **ppCallHub) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_EventCode( 
            /* [retval][out] */ __RPC__out long *plEventCode) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_EventInterface( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **pEventInterface) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITPrivateEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITPrivateEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITPrivateEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITPrivateEvent * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITPrivateEvent * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITPrivateEvent * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITPrivateEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITPrivateEvent * This,
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
        
        DECLSPEC_XFGVIRT(ITPrivateEvent, get_Address)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Address )( 
            __RPC__in ITPrivateEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt ITAddress **ppAddress);
        
        DECLSPEC_XFGVIRT(ITPrivateEvent, get_Call)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Call )( 
            __RPC__in ITPrivateEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt ITCallInfo **ppCallInfo);
        
        DECLSPEC_XFGVIRT(ITPrivateEvent, get_CallHub)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CallHub )( 
            __RPC__in ITPrivateEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt ITCallHub **ppCallHub);
        
        DECLSPEC_XFGVIRT(ITPrivateEvent, get_EventCode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EventCode )( 
            __RPC__in ITPrivateEvent * This,
            /* [retval][out] */ __RPC__out long *plEventCode);
        
        DECLSPEC_XFGVIRT(ITPrivateEvent, get_EventInterface)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EventInterface )( 
            __RPC__in ITPrivateEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **pEventInterface);
        
        END_INTERFACE
    } ITPrivateEventVtbl;

    interface ITPrivateEvent
    {
        CONST_VTBL struct ITPrivateEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITPrivateEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITPrivateEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITPrivateEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITPrivateEvent_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITPrivateEvent_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITPrivateEvent_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITPrivateEvent_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITPrivateEvent_get_Address(This,ppAddress)	\
    ( (This)->lpVtbl -> get_Address(This,ppAddress) ) 

#define ITPrivateEvent_get_Call(This,ppCallInfo)	\
    ( (This)->lpVtbl -> get_Call(This,ppCallInfo) ) 

#define ITPrivateEvent_get_CallHub(This,ppCallHub)	\
    ( (This)->lpVtbl -> get_CallHub(This,ppCallHub) ) 

#define ITPrivateEvent_get_EventCode(This,plEventCode)	\
    ( (This)->lpVtbl -> get_EventCode(This,plEventCode) ) 

#define ITPrivateEvent_get_EventInterface(This,pEventInterface)	\
    ( (This)->lpVtbl -> get_EventInterface(This,pEventInterface) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITPrivateEvent_INTERFACE_DEFINED__ */


#ifndef __ITLegacyAddressMediaControl2_INTERFACE_DEFINED__
#define __ITLegacyAddressMediaControl2_INTERFACE_DEFINED__

/* interface ITLegacyAddressMediaControl2 */
/* [object][unique][hidden][helpstring][uuid] */ 


EXTERN_C const IID IID_ITLegacyAddressMediaControl2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B0EE512B-A531-409e-9DD9-4099FE86C738")
    ITLegacyAddressMediaControl2 : public ITLegacyAddressMediaControl
    {
    public:
        virtual /* [hidden][id] */ HRESULT STDMETHODCALLTYPE ConfigDialog( 
            /* [in] */ __RPC__in HWND hwndOwner,
            /* [in] */ __RPC__in BSTR pDeviceClass) = 0;
        
        virtual /* [hidden][id] */ HRESULT STDMETHODCALLTYPE ConfigDialogEdit( 
            /* [in] */ __RPC__in HWND hwndOwner,
            /* [in] */ __RPC__in BSTR pDeviceClass,
            /* [in] */ DWORD dwSizeIn,
            /* [size_is][in] */ __RPC__in_ecount_full(dwSizeIn) BYTE *pDeviceConfigIn,
            /* [out] */ __RPC__out DWORD *pdwSizeOut,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pdwSizeOut) BYTE **ppDeviceConfigOut) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITLegacyAddressMediaControl2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITLegacyAddressMediaControl2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITLegacyAddressMediaControl2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITLegacyAddressMediaControl2 * This);
        
        DECLSPEC_XFGVIRT(ITLegacyAddressMediaControl, GetID)
        /* [hidden][id] */ HRESULT ( STDMETHODCALLTYPE *GetID )( 
            __RPC__in ITLegacyAddressMediaControl2 * This,
            /* [in] */ __RPC__in BSTR pDeviceClass,
            /* [out] */ __RPC__out DWORD *pdwSize,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pdwSize) BYTE **ppDeviceID);
        
        DECLSPEC_XFGVIRT(ITLegacyAddressMediaControl, GetDevConfig)
        /* [hidden][id] */ HRESULT ( STDMETHODCALLTYPE *GetDevConfig )( 
            __RPC__in ITLegacyAddressMediaControl2 * This,
            /* [in] */ __RPC__in BSTR pDeviceClass,
            /* [out] */ __RPC__out DWORD *pdwSize,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pdwSize) BYTE **ppDeviceConfig);
        
        DECLSPEC_XFGVIRT(ITLegacyAddressMediaControl, SetDevConfig)
        /* [hidden][id] */ HRESULT ( STDMETHODCALLTYPE *SetDevConfig )( 
            __RPC__in ITLegacyAddressMediaControl2 * This,
            /* [in] */ __RPC__in BSTR pDeviceClass,
            /* [in] */ DWORD dwSize,
            /* [size_is][in] */ __RPC__in_ecount_full(dwSize) BYTE *pDeviceConfig);
        
        DECLSPEC_XFGVIRT(ITLegacyAddressMediaControl2, ConfigDialog)
        /* [hidden][id] */ HRESULT ( STDMETHODCALLTYPE *ConfigDialog )( 
            __RPC__in ITLegacyAddressMediaControl2 * This,
            /* [in] */ __RPC__in HWND hwndOwner,
            /* [in] */ __RPC__in BSTR pDeviceClass);
        
        DECLSPEC_XFGVIRT(ITLegacyAddressMediaControl2, ConfigDialogEdit)
        /* [hidden][id] */ HRESULT ( STDMETHODCALLTYPE *ConfigDialogEdit )( 
            __RPC__in ITLegacyAddressMediaControl2 * This,
            /* [in] */ __RPC__in HWND hwndOwner,
            /* [in] */ __RPC__in BSTR pDeviceClass,
            /* [in] */ DWORD dwSizeIn,
            /* [size_is][in] */ __RPC__in_ecount_full(dwSizeIn) BYTE *pDeviceConfigIn,
            /* [out] */ __RPC__out DWORD *pdwSizeOut,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pdwSizeOut) BYTE **ppDeviceConfigOut);
        
        END_INTERFACE
    } ITLegacyAddressMediaControl2Vtbl;

    interface ITLegacyAddressMediaControl2
    {
        CONST_VTBL struct ITLegacyAddressMediaControl2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITLegacyAddressMediaControl2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITLegacyAddressMediaControl2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITLegacyAddressMediaControl2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITLegacyAddressMediaControl2_GetID(This,pDeviceClass,pdwSize,ppDeviceID)	\
    ( (This)->lpVtbl -> GetID(This,pDeviceClass,pdwSize,ppDeviceID) ) 

#define ITLegacyAddressMediaControl2_GetDevConfig(This,pDeviceClass,pdwSize,ppDeviceConfig)	\
    ( (This)->lpVtbl -> GetDevConfig(This,pDeviceClass,pdwSize,ppDeviceConfig) ) 

#define ITLegacyAddressMediaControl2_SetDevConfig(This,pDeviceClass,dwSize,pDeviceConfig)	\
    ( (This)->lpVtbl -> SetDevConfig(This,pDeviceClass,dwSize,pDeviceConfig) ) 


#define ITLegacyAddressMediaControl2_ConfigDialog(This,hwndOwner,pDeviceClass)	\
    ( (This)->lpVtbl -> ConfigDialog(This,hwndOwner,pDeviceClass) ) 

#define ITLegacyAddressMediaControl2_ConfigDialogEdit(This,hwndOwner,pDeviceClass,dwSizeIn,pDeviceConfigIn,pdwSizeOut,ppDeviceConfigOut)	\
    ( (This)->lpVtbl -> ConfigDialogEdit(This,hwndOwner,pDeviceClass,dwSizeIn,pDeviceConfigIn,pdwSizeOut,ppDeviceConfigOut) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITLegacyAddressMediaControl2_INTERFACE_DEFINED__ */


#ifndef __ITLegacyCallMediaControl_INTERFACE_DEFINED__
#define __ITLegacyCallMediaControl_INTERFACE_DEFINED__

/* interface ITLegacyCallMediaControl */
/* [object][unique][dual][helpstring][uuid] */ 


EXTERN_C const IID IID_ITLegacyCallMediaControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("d624582f-cc23-4436-b8a5-47c625c8045d")
    ITLegacyCallMediaControl : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE DetectDigits( 
            /* [in] */ TAPI_DIGITMODE DigitMode) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GenerateDigits( 
            /* [in] */ __RPC__in BSTR pDigits,
            /* [in] */ TAPI_DIGITMODE DigitMode) = 0;
        
        virtual /* [hidden][id] */ HRESULT STDMETHODCALLTYPE GetID( 
            /* [in] */ __RPC__in BSTR pDeviceClass,
            /* [out] */ __RPC__out DWORD *pdwSize,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pdwSize) BYTE **ppDeviceID) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetMediaType( 
            /* [in] */ long lMediaType) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE MonitorMedia( 
            /* [in] */ long lMediaType) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITLegacyCallMediaControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITLegacyCallMediaControl * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITLegacyCallMediaControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITLegacyCallMediaControl * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITLegacyCallMediaControl * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITLegacyCallMediaControl * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITLegacyCallMediaControl * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITLegacyCallMediaControl * This,
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
        
        DECLSPEC_XFGVIRT(ITLegacyCallMediaControl, DetectDigits)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DetectDigits )( 
            __RPC__in ITLegacyCallMediaControl * This,
            /* [in] */ TAPI_DIGITMODE DigitMode);
        
        DECLSPEC_XFGVIRT(ITLegacyCallMediaControl, GenerateDigits)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GenerateDigits )( 
            __RPC__in ITLegacyCallMediaControl * This,
            /* [in] */ __RPC__in BSTR pDigits,
            /* [in] */ TAPI_DIGITMODE DigitMode);
        
        DECLSPEC_XFGVIRT(ITLegacyCallMediaControl, GetID)
        /* [hidden][id] */ HRESULT ( STDMETHODCALLTYPE *GetID )( 
            __RPC__in ITLegacyCallMediaControl * This,
            /* [in] */ __RPC__in BSTR pDeviceClass,
            /* [out] */ __RPC__out DWORD *pdwSize,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pdwSize) BYTE **ppDeviceID);
        
        DECLSPEC_XFGVIRT(ITLegacyCallMediaControl, SetMediaType)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetMediaType )( 
            __RPC__in ITLegacyCallMediaControl * This,
            /* [in] */ long lMediaType);
        
        DECLSPEC_XFGVIRT(ITLegacyCallMediaControl, MonitorMedia)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *MonitorMedia )( 
            __RPC__in ITLegacyCallMediaControl * This,
            /* [in] */ long lMediaType);
        
        END_INTERFACE
    } ITLegacyCallMediaControlVtbl;

    interface ITLegacyCallMediaControl
    {
        CONST_VTBL struct ITLegacyCallMediaControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITLegacyCallMediaControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITLegacyCallMediaControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITLegacyCallMediaControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITLegacyCallMediaControl_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITLegacyCallMediaControl_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITLegacyCallMediaControl_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITLegacyCallMediaControl_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITLegacyCallMediaControl_DetectDigits(This,DigitMode)	\
    ( (This)->lpVtbl -> DetectDigits(This,DigitMode) ) 

#define ITLegacyCallMediaControl_GenerateDigits(This,pDigits,DigitMode)	\
    ( (This)->lpVtbl -> GenerateDigits(This,pDigits,DigitMode) ) 

#define ITLegacyCallMediaControl_GetID(This,pDeviceClass,pdwSize,ppDeviceID)	\
    ( (This)->lpVtbl -> GetID(This,pDeviceClass,pdwSize,ppDeviceID) ) 

#define ITLegacyCallMediaControl_SetMediaType(This,lMediaType)	\
    ( (This)->lpVtbl -> SetMediaType(This,lMediaType) ) 

#define ITLegacyCallMediaControl_MonitorMedia(This,lMediaType)	\
    ( (This)->lpVtbl -> MonitorMedia(This,lMediaType) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITLegacyCallMediaControl_INTERFACE_DEFINED__ */


#ifndef __ITLegacyCallMediaControl2_INTERFACE_DEFINED__
#define __ITLegacyCallMediaControl2_INTERFACE_DEFINED__

/* interface ITLegacyCallMediaControl2 */
/* [object][dual][helpstring][uuid] */ 


EXTERN_C const IID IID_ITLegacyCallMediaControl2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("57CA332D-7BC2-44f1-A60C-936FE8D7CE73")
    ITLegacyCallMediaControl2 : public ITLegacyCallMediaControl
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GenerateDigits2( 
            /* [in] */ __RPC__in BSTR pDigits,
            /* [in] */ TAPI_DIGITMODE DigitMode,
            /* [in] */ long lDuration) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GatherDigits( 
            /* [in] */ TAPI_DIGITMODE DigitMode,
            /* [in] */ long lNumDigits,
            /* [in] */ __RPC__in BSTR pTerminationDigits,
            /* [in] */ long lFirstDigitTimeout,
            /* [in] */ long lInterDigitTimeout) = 0;
        
        virtual /* [helpstring][hidden][id] */ HRESULT STDMETHODCALLTYPE DetectTones( 
            /* [in] */ __RPC__in TAPI_DETECTTONE *pToneList,
            /* [in] */ long lNumTones) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE DetectTonesByCollection( 
            /* [in] */ __RPC__in_opt ITCollection2 *pDetectToneCollection) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GenerateTone( 
            /* [in] */ TAPI_TONEMODE ToneMode,
            /* [in] */ long lDuration) = 0;
        
        virtual /* [helpstring][hidden][id] */ HRESULT STDMETHODCALLTYPE GenerateCustomTones( 
            /* [in] */ __RPC__in TAPI_CUSTOMTONE *pToneList,
            /* [in] */ long lNumTones,
            /* [in] */ long lDuration) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GenerateCustomTonesByCollection( 
            /* [in] */ __RPC__in_opt ITCollection2 *pCustomToneCollection,
            /* [in] */ long lDuration) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateDetectToneObject( 
            /* [retval][out] */ __RPC__deref_out_opt ITDetectTone **ppDetectTone) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateCustomToneObject( 
            /* [retval][out] */ __RPC__deref_out_opt ITCustomTone **ppCustomTone) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetIDAsVariant( 
            /* [in] */ __RPC__in BSTR bstrDeviceClass,
            /* [retval][out] */ __RPC__out VARIANT *pVarDeviceID) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITLegacyCallMediaControl2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITLegacyCallMediaControl2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITLegacyCallMediaControl2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITLegacyCallMediaControl2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITLegacyCallMediaControl2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITLegacyCallMediaControl2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITLegacyCallMediaControl2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITLegacyCallMediaControl2 * This,
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
        
        DECLSPEC_XFGVIRT(ITLegacyCallMediaControl, DetectDigits)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DetectDigits )( 
            __RPC__in ITLegacyCallMediaControl2 * This,
            /* [in] */ TAPI_DIGITMODE DigitMode);
        
        DECLSPEC_XFGVIRT(ITLegacyCallMediaControl, GenerateDigits)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GenerateDigits )( 
            __RPC__in ITLegacyCallMediaControl2 * This,
            /* [in] */ __RPC__in BSTR pDigits,
            /* [in] */ TAPI_DIGITMODE DigitMode);
        
        DECLSPEC_XFGVIRT(ITLegacyCallMediaControl, GetID)
        /* [hidden][id] */ HRESULT ( STDMETHODCALLTYPE *GetID )( 
            __RPC__in ITLegacyCallMediaControl2 * This,
            /* [in] */ __RPC__in BSTR pDeviceClass,
            /* [out] */ __RPC__out DWORD *pdwSize,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pdwSize) BYTE **ppDeviceID);
        
        DECLSPEC_XFGVIRT(ITLegacyCallMediaControl, SetMediaType)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetMediaType )( 
            __RPC__in ITLegacyCallMediaControl2 * This,
            /* [in] */ long lMediaType);
        
        DECLSPEC_XFGVIRT(ITLegacyCallMediaControl, MonitorMedia)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *MonitorMedia )( 
            __RPC__in ITLegacyCallMediaControl2 * This,
            /* [in] */ long lMediaType);
        
        DECLSPEC_XFGVIRT(ITLegacyCallMediaControl2, GenerateDigits2)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GenerateDigits2 )( 
            __RPC__in ITLegacyCallMediaControl2 * This,
            /* [in] */ __RPC__in BSTR pDigits,
            /* [in] */ TAPI_DIGITMODE DigitMode,
            /* [in] */ long lDuration);
        
        DECLSPEC_XFGVIRT(ITLegacyCallMediaControl2, GatherDigits)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GatherDigits )( 
            __RPC__in ITLegacyCallMediaControl2 * This,
            /* [in] */ TAPI_DIGITMODE DigitMode,
            /* [in] */ long lNumDigits,
            /* [in] */ __RPC__in BSTR pTerminationDigits,
            /* [in] */ long lFirstDigitTimeout,
            /* [in] */ long lInterDigitTimeout);
        
        DECLSPEC_XFGVIRT(ITLegacyCallMediaControl2, DetectTones)
        /* [helpstring][hidden][id] */ HRESULT ( STDMETHODCALLTYPE *DetectTones )( 
            __RPC__in ITLegacyCallMediaControl2 * This,
            /* [in] */ __RPC__in TAPI_DETECTTONE *pToneList,
            /* [in] */ long lNumTones);
        
        DECLSPEC_XFGVIRT(ITLegacyCallMediaControl2, DetectTonesByCollection)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DetectTonesByCollection )( 
            __RPC__in ITLegacyCallMediaControl2 * This,
            /* [in] */ __RPC__in_opt ITCollection2 *pDetectToneCollection);
        
        DECLSPEC_XFGVIRT(ITLegacyCallMediaControl2, GenerateTone)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GenerateTone )( 
            __RPC__in ITLegacyCallMediaControl2 * This,
            /* [in] */ TAPI_TONEMODE ToneMode,
            /* [in] */ long lDuration);
        
        DECLSPEC_XFGVIRT(ITLegacyCallMediaControl2, GenerateCustomTones)
        /* [helpstring][hidden][id] */ HRESULT ( STDMETHODCALLTYPE *GenerateCustomTones )( 
            __RPC__in ITLegacyCallMediaControl2 * This,
            /* [in] */ __RPC__in TAPI_CUSTOMTONE *pToneList,
            /* [in] */ long lNumTones,
            /* [in] */ long lDuration);
        
        DECLSPEC_XFGVIRT(ITLegacyCallMediaControl2, GenerateCustomTonesByCollection)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GenerateCustomTonesByCollection )( 
            __RPC__in ITLegacyCallMediaControl2 * This,
            /* [in] */ __RPC__in_opt ITCollection2 *pCustomToneCollection,
            /* [in] */ long lDuration);
        
        DECLSPEC_XFGVIRT(ITLegacyCallMediaControl2, CreateDetectToneObject)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateDetectToneObject )( 
            __RPC__in ITLegacyCallMediaControl2 * This,
            /* [retval][out] */ __RPC__deref_out_opt ITDetectTone **ppDetectTone);
        
        DECLSPEC_XFGVIRT(ITLegacyCallMediaControl2, CreateCustomToneObject)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateCustomToneObject )( 
            __RPC__in ITLegacyCallMediaControl2 * This,
            /* [retval][out] */ __RPC__deref_out_opt ITCustomTone **ppCustomTone);
        
        DECLSPEC_XFGVIRT(ITLegacyCallMediaControl2, GetIDAsVariant)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetIDAsVariant )( 
            __RPC__in ITLegacyCallMediaControl2 * This,
            /* [in] */ __RPC__in BSTR bstrDeviceClass,
            /* [retval][out] */ __RPC__out VARIANT *pVarDeviceID);
        
        END_INTERFACE
    } ITLegacyCallMediaControl2Vtbl;

    interface ITLegacyCallMediaControl2
    {
        CONST_VTBL struct ITLegacyCallMediaControl2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITLegacyCallMediaControl2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITLegacyCallMediaControl2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITLegacyCallMediaControl2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITLegacyCallMediaControl2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITLegacyCallMediaControl2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITLegacyCallMediaControl2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITLegacyCallMediaControl2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITLegacyCallMediaControl2_DetectDigits(This,DigitMode)	\
    ( (This)->lpVtbl -> DetectDigits(This,DigitMode) ) 

#define ITLegacyCallMediaControl2_GenerateDigits(This,pDigits,DigitMode)	\
    ( (This)->lpVtbl -> GenerateDigits(This,pDigits,DigitMode) ) 

#define ITLegacyCallMediaControl2_GetID(This,pDeviceClass,pdwSize,ppDeviceID)	\
    ( (This)->lpVtbl -> GetID(This,pDeviceClass,pdwSize,ppDeviceID) ) 

#define ITLegacyCallMediaControl2_SetMediaType(This,lMediaType)	\
    ( (This)->lpVtbl -> SetMediaType(This,lMediaType) ) 

#define ITLegacyCallMediaControl2_MonitorMedia(This,lMediaType)	\
    ( (This)->lpVtbl -> MonitorMedia(This,lMediaType) ) 


#define ITLegacyCallMediaControl2_GenerateDigits2(This,pDigits,DigitMode,lDuration)	\
    ( (This)->lpVtbl -> GenerateDigits2(This,pDigits,DigitMode,lDuration) ) 

#define ITLegacyCallMediaControl2_GatherDigits(This,DigitMode,lNumDigits,pTerminationDigits,lFirstDigitTimeout,lInterDigitTimeout)	\
    ( (This)->lpVtbl -> GatherDigits(This,DigitMode,lNumDigits,pTerminationDigits,lFirstDigitTimeout,lInterDigitTimeout) ) 

#define ITLegacyCallMediaControl2_DetectTones(This,pToneList,lNumTones)	\
    ( (This)->lpVtbl -> DetectTones(This,pToneList,lNumTones) ) 

#define ITLegacyCallMediaControl2_DetectTonesByCollection(This,pDetectToneCollection)	\
    ( (This)->lpVtbl -> DetectTonesByCollection(This,pDetectToneCollection) ) 

#define ITLegacyCallMediaControl2_GenerateTone(This,ToneMode,lDuration)	\
    ( (This)->lpVtbl -> GenerateTone(This,ToneMode,lDuration) ) 

#define ITLegacyCallMediaControl2_GenerateCustomTones(This,pToneList,lNumTones,lDuration)	\
    ( (This)->lpVtbl -> GenerateCustomTones(This,pToneList,lNumTones,lDuration) ) 

#define ITLegacyCallMediaControl2_GenerateCustomTonesByCollection(This,pCustomToneCollection,lDuration)	\
    ( (This)->lpVtbl -> GenerateCustomTonesByCollection(This,pCustomToneCollection,lDuration) ) 

#define ITLegacyCallMediaControl2_CreateDetectToneObject(This,ppDetectTone)	\
    ( (This)->lpVtbl -> CreateDetectToneObject(This,ppDetectTone) ) 

#define ITLegacyCallMediaControl2_CreateCustomToneObject(This,ppCustomTone)	\
    ( (This)->lpVtbl -> CreateCustomToneObject(This,ppCustomTone) ) 

#define ITLegacyCallMediaControl2_GetIDAsVariant(This,bstrDeviceClass,pVarDeviceID)	\
    ( (This)->lpVtbl -> GetIDAsVariant(This,bstrDeviceClass,pVarDeviceID) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITLegacyCallMediaControl2_INTERFACE_DEFINED__ */


#ifndef __ITDetectTone_INTERFACE_DEFINED__
#define __ITDetectTone_INTERFACE_DEFINED__

/* interface ITDetectTone */
/* [object][dual][helpstring][uuid] */ 


EXTERN_C const IID IID_ITDetectTone;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("961F79BD-3097-49df-A1D6-909B77E89CA0")
    ITDetectTone : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_AppSpecific( 
            /* [retval][out] */ __RPC__out long *plAppSpecific) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_AppSpecific( 
            /* [in] */ long lAppSpecific) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Duration( 
            /* [retval][out] */ __RPC__out long *plDuration) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Duration( 
            /* [in] */ long lDuration) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Frequency( 
            /* [in] */ long Index,
            /* [retval][out] */ __RPC__out long *plFrequency) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Frequency( 
            /* [in] */ long Index,
            /* [in] */ long lFrequency) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITDetectToneVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITDetectTone * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITDetectTone * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITDetectTone * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITDetectTone * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITDetectTone * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITDetectTone * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITDetectTone * This,
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
        
        DECLSPEC_XFGVIRT(ITDetectTone, get_AppSpecific)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AppSpecific )( 
            __RPC__in ITDetectTone * This,
            /* [retval][out] */ __RPC__out long *plAppSpecific);
        
        DECLSPEC_XFGVIRT(ITDetectTone, put_AppSpecific)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_AppSpecific )( 
            __RPC__in ITDetectTone * This,
            /* [in] */ long lAppSpecific);
        
        DECLSPEC_XFGVIRT(ITDetectTone, get_Duration)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Duration )( 
            __RPC__in ITDetectTone * This,
            /* [retval][out] */ __RPC__out long *plDuration);
        
        DECLSPEC_XFGVIRT(ITDetectTone, put_Duration)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Duration )( 
            __RPC__in ITDetectTone * This,
            /* [in] */ long lDuration);
        
        DECLSPEC_XFGVIRT(ITDetectTone, get_Frequency)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Frequency )( 
            __RPC__in ITDetectTone * This,
            /* [in] */ long Index,
            /* [retval][out] */ __RPC__out long *plFrequency);
        
        DECLSPEC_XFGVIRT(ITDetectTone, put_Frequency)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Frequency )( 
            __RPC__in ITDetectTone * This,
            /* [in] */ long Index,
            /* [in] */ long lFrequency);
        
        END_INTERFACE
    } ITDetectToneVtbl;

    interface ITDetectTone
    {
        CONST_VTBL struct ITDetectToneVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITDetectTone_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITDetectTone_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITDetectTone_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITDetectTone_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITDetectTone_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITDetectTone_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITDetectTone_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITDetectTone_get_AppSpecific(This,plAppSpecific)	\
    ( (This)->lpVtbl -> get_AppSpecific(This,plAppSpecific) ) 

#define ITDetectTone_put_AppSpecific(This,lAppSpecific)	\
    ( (This)->lpVtbl -> put_AppSpecific(This,lAppSpecific) ) 

#define ITDetectTone_get_Duration(This,plDuration)	\
    ( (This)->lpVtbl -> get_Duration(This,plDuration) ) 

#define ITDetectTone_put_Duration(This,lDuration)	\
    ( (This)->lpVtbl -> put_Duration(This,lDuration) ) 

#define ITDetectTone_get_Frequency(This,Index,plFrequency)	\
    ( (This)->lpVtbl -> get_Frequency(This,Index,plFrequency) ) 

#define ITDetectTone_put_Frequency(This,Index,lFrequency)	\
    ( (This)->lpVtbl -> put_Frequency(This,Index,lFrequency) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITDetectTone_INTERFACE_DEFINED__ */


#ifndef __ITCustomTone_INTERFACE_DEFINED__
#define __ITCustomTone_INTERFACE_DEFINED__

/* interface ITCustomTone */
/* [object][unique][dual][helpstring][uuid] */ 


EXTERN_C const IID IID_ITCustomTone;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("357AD764-B3C6-4b2a-8FA5-0722827A9254")
    ITCustomTone : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Frequency( 
            /* [retval][out] */ __RPC__out long *plFrequency) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Frequency( 
            /* [in] */ long lFrequency) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CadenceOn( 
            /* [retval][out] */ __RPC__out long *plCadenceOn) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_CadenceOn( 
            /* [in] */ long CadenceOn) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CadenceOff( 
            /* [retval][out] */ __RPC__out long *plCadenceOff) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_CadenceOff( 
            /* [in] */ long lCadenceOff) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Volume( 
            /* [retval][out] */ __RPC__out long *plVolume) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Volume( 
            /* [in] */ long lVolume) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITCustomToneVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITCustomTone * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITCustomTone * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITCustomTone * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITCustomTone * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITCustomTone * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITCustomTone * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITCustomTone * This,
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
        
        DECLSPEC_XFGVIRT(ITCustomTone, get_Frequency)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Frequency )( 
            __RPC__in ITCustomTone * This,
            /* [retval][out] */ __RPC__out long *plFrequency);
        
        DECLSPEC_XFGVIRT(ITCustomTone, put_Frequency)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Frequency )( 
            __RPC__in ITCustomTone * This,
            /* [in] */ long lFrequency);
        
        DECLSPEC_XFGVIRT(ITCustomTone, get_CadenceOn)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CadenceOn )( 
            __RPC__in ITCustomTone * This,
            /* [retval][out] */ __RPC__out long *plCadenceOn);
        
        DECLSPEC_XFGVIRT(ITCustomTone, put_CadenceOn)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_CadenceOn )( 
            __RPC__in ITCustomTone * This,
            /* [in] */ long CadenceOn);
        
        DECLSPEC_XFGVIRT(ITCustomTone, get_CadenceOff)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CadenceOff )( 
            __RPC__in ITCustomTone * This,
            /* [retval][out] */ __RPC__out long *plCadenceOff);
        
        DECLSPEC_XFGVIRT(ITCustomTone, put_CadenceOff)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_CadenceOff )( 
            __RPC__in ITCustomTone * This,
            /* [in] */ long lCadenceOff);
        
        DECLSPEC_XFGVIRT(ITCustomTone, get_Volume)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Volume )( 
            __RPC__in ITCustomTone * This,
            /* [retval][out] */ __RPC__out long *plVolume);
        
        DECLSPEC_XFGVIRT(ITCustomTone, put_Volume)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Volume )( 
            __RPC__in ITCustomTone * This,
            /* [in] */ long lVolume);
        
        END_INTERFACE
    } ITCustomToneVtbl;

    interface ITCustomTone
    {
        CONST_VTBL struct ITCustomToneVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITCustomTone_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITCustomTone_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITCustomTone_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITCustomTone_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITCustomTone_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITCustomTone_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITCustomTone_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITCustomTone_get_Frequency(This,plFrequency)	\
    ( (This)->lpVtbl -> get_Frequency(This,plFrequency) ) 

#define ITCustomTone_put_Frequency(This,lFrequency)	\
    ( (This)->lpVtbl -> put_Frequency(This,lFrequency) ) 

#define ITCustomTone_get_CadenceOn(This,plCadenceOn)	\
    ( (This)->lpVtbl -> get_CadenceOn(This,plCadenceOn) ) 

#define ITCustomTone_put_CadenceOn(This,CadenceOn)	\
    ( (This)->lpVtbl -> put_CadenceOn(This,CadenceOn) ) 

#define ITCustomTone_get_CadenceOff(This,plCadenceOff)	\
    ( (This)->lpVtbl -> get_CadenceOff(This,plCadenceOff) ) 

#define ITCustomTone_put_CadenceOff(This,lCadenceOff)	\
    ( (This)->lpVtbl -> put_CadenceOff(This,lCadenceOff) ) 

#define ITCustomTone_get_Volume(This,plVolume)	\
    ( (This)->lpVtbl -> get_Volume(This,plVolume) ) 

#define ITCustomTone_put_Volume(This,lVolume)	\
    ( (This)->lpVtbl -> put_Volume(This,lVolume) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITCustomTone_INTERFACE_DEFINED__ */


#ifndef __IEnumPhone_INTERFACE_DEFINED__
#define __IEnumPhone_INTERFACE_DEFINED__

/* interface IEnumPhone */
/* [object][unique][hidden][helpstring][uuid] */ 


EXTERN_C const IID IID_IEnumPhone;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F15B7669-4780-4595-8C89-FB369C8CF7AA")
    IEnumPhone : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG celt,
            /* [size_is][out] */ __RPC__out_ecount_full(celt) ITPhone **ppElements,
            /* [full][out][in] */ __RPC__inout_opt ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumPhone **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumPhoneVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumPhone * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumPhone * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumPhone * This);
        
        DECLSPEC_XFGVIRT(IEnumPhone, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IEnumPhone * This,
            /* [in] */ ULONG celt,
            /* [size_is][out] */ __RPC__out_ecount_full(celt) ITPhone **ppElements,
            /* [full][out][in] */ __RPC__inout_opt ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IEnumPhone, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumPhone * This);
        
        DECLSPEC_XFGVIRT(IEnumPhone, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumPhone * This,
            /* [in] */ ULONG celt);
        
        DECLSPEC_XFGVIRT(IEnumPhone, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumPhone * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumPhone **ppEnum);
        
        END_INTERFACE
    } IEnumPhoneVtbl;

    interface IEnumPhone
    {
        CONST_VTBL struct IEnumPhoneVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumPhone_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumPhone_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumPhone_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumPhone_Next(This,celt,ppElements,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,ppElements,pceltFetched) ) 

#define IEnumPhone_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumPhone_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IEnumPhone_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumPhone_INTERFACE_DEFINED__ */


#ifndef __IEnumTerminal_INTERFACE_DEFINED__
#define __IEnumTerminal_INTERFACE_DEFINED__

/* interface IEnumTerminal */
/* [object][unique][hidden][helpstring][uuid] */ 


EXTERN_C const IID IID_IEnumTerminal;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("AE269CF4-935E-11d0-835C-00AA003CCABD")
    IEnumTerminal : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG celt,
            /* [out] */ __RPC__deref_out_opt ITTerminal **ppElements,
            /* [full][out][in] */ __RPC__inout_opt ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumTerminal **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumTerminalVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumTerminal * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumTerminal * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumTerminal * This);
        
        DECLSPEC_XFGVIRT(IEnumTerminal, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IEnumTerminal * This,
            /* [in] */ ULONG celt,
            /* [out] */ __RPC__deref_out_opt ITTerminal **ppElements,
            /* [full][out][in] */ __RPC__inout_opt ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IEnumTerminal, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumTerminal * This);
        
        DECLSPEC_XFGVIRT(IEnumTerminal, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumTerminal * This,
            /* [in] */ ULONG celt);
        
        DECLSPEC_XFGVIRT(IEnumTerminal, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumTerminal * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumTerminal **ppEnum);
        
        END_INTERFACE
    } IEnumTerminalVtbl;

    interface IEnumTerminal
    {
        CONST_VTBL struct IEnumTerminalVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumTerminal_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumTerminal_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumTerminal_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumTerminal_Next(This,celt,ppElements,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,ppElements,pceltFetched) ) 

#define IEnumTerminal_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumTerminal_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IEnumTerminal_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumTerminal_INTERFACE_DEFINED__ */


#ifndef __IEnumTerminalClass_INTERFACE_DEFINED__
#define __IEnumTerminalClass_INTERFACE_DEFINED__

/* interface IEnumTerminalClass */
/* [object][unique][hidden][helpstring][uuid] */ 


EXTERN_C const IID IID_IEnumTerminalClass;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("AE269CF5-935E-11d0-835C-00AA003CCABD")
    IEnumTerminalClass : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG celt,
            /* [size_is][out] */ __RPC__out_ecount_full(celt) GUID *pElements,
            /* [full][out][in] */ __RPC__inout_opt ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumTerminalClass **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumTerminalClassVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumTerminalClass * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumTerminalClass * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumTerminalClass * This);
        
        DECLSPEC_XFGVIRT(IEnumTerminalClass, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IEnumTerminalClass * This,
            /* [in] */ ULONG celt,
            /* [size_is][out] */ __RPC__out_ecount_full(celt) GUID *pElements,
            /* [full][out][in] */ __RPC__inout_opt ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IEnumTerminalClass, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumTerminalClass * This);
        
        DECLSPEC_XFGVIRT(IEnumTerminalClass, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumTerminalClass * This,
            /* [in] */ ULONG celt);
        
        DECLSPEC_XFGVIRT(IEnumTerminalClass, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumTerminalClass * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumTerminalClass **ppEnum);
        
        END_INTERFACE
    } IEnumTerminalClassVtbl;

    interface IEnumTerminalClass
    {
        CONST_VTBL struct IEnumTerminalClassVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumTerminalClass_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumTerminalClass_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumTerminalClass_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumTerminalClass_Next(This,celt,pElements,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,pElements,pceltFetched) ) 

#define IEnumTerminalClass_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumTerminalClass_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IEnumTerminalClass_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumTerminalClass_INTERFACE_DEFINED__ */


#ifndef __IEnumCall_INTERFACE_DEFINED__
#define __IEnumCall_INTERFACE_DEFINED__

/* interface IEnumCall */
/* [object][unique][hidden][helpstring][uuid] */ 


EXTERN_C const IID IID_IEnumCall;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("AE269CF6-935E-11d0-835C-00AA003CCABD")
    IEnumCall : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG celt,
            /* [out] */ __RPC__deref_out_opt ITCallInfo **ppElements,
            /* [full][out][in] */ __RPC__inout_opt ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumCall **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumCallVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumCall * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumCall * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumCall * This);
        
        DECLSPEC_XFGVIRT(IEnumCall, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IEnumCall * This,
            /* [in] */ ULONG celt,
            /* [out] */ __RPC__deref_out_opt ITCallInfo **ppElements,
            /* [full][out][in] */ __RPC__inout_opt ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IEnumCall, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumCall * This);
        
        DECLSPEC_XFGVIRT(IEnumCall, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumCall * This,
            /* [in] */ ULONG celt);
        
        DECLSPEC_XFGVIRT(IEnumCall, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumCall * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumCall **ppEnum);
        
        END_INTERFACE
    } IEnumCallVtbl;

    interface IEnumCall
    {
        CONST_VTBL struct IEnumCallVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumCall_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumCall_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumCall_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumCall_Next(This,celt,ppElements,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,ppElements,pceltFetched) ) 

#define IEnumCall_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumCall_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IEnumCall_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumCall_INTERFACE_DEFINED__ */


#ifndef __IEnumAddress_INTERFACE_DEFINED__
#define __IEnumAddress_INTERFACE_DEFINED__

/* interface IEnumAddress */
/* [object][unique][hidden][helpstring][uuid] */ 


EXTERN_C const IID IID_IEnumAddress;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1666FCA1-9363-11d0-835C-00AA003CCABD")
    IEnumAddress : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG celt,
            /* [size_is][out] */ __RPC__out_ecount_full(celt) ITAddress **ppElements,
            /* [full][out][in] */ __RPC__inout_opt ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumAddress **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumAddressVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumAddress * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumAddress * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumAddress * This);
        
        DECLSPEC_XFGVIRT(IEnumAddress, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IEnumAddress * This,
            /* [in] */ ULONG celt,
            /* [size_is][out] */ __RPC__out_ecount_full(celt) ITAddress **ppElements,
            /* [full][out][in] */ __RPC__inout_opt ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IEnumAddress, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumAddress * This);
        
        DECLSPEC_XFGVIRT(IEnumAddress, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumAddress * This,
            /* [in] */ ULONG celt);
        
        DECLSPEC_XFGVIRT(IEnumAddress, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumAddress * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumAddress **ppEnum);
        
        END_INTERFACE
    } IEnumAddressVtbl;

    interface IEnumAddress
    {
        CONST_VTBL struct IEnumAddressVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumAddress_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumAddress_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumAddress_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumAddress_Next(This,celt,ppElements,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,ppElements,pceltFetched) ) 

#define IEnumAddress_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumAddress_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IEnumAddress_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumAddress_INTERFACE_DEFINED__ */


#ifndef __IEnumCallHub_INTERFACE_DEFINED__
#define __IEnumCallHub_INTERFACE_DEFINED__

/* interface IEnumCallHub */
/* [object][unique][hidden][helpstring][uuid] */ 


EXTERN_C const IID IID_IEnumCallHub;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A3C15450-5B92-11d1-8F4E-00C04FB6809F")
    IEnumCallHub : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG celt,
            /* [size_is][out] */ __RPC__out_ecount_full(celt) ITCallHub **ppElements,
            /* [full][out][in] */ __RPC__inout_opt ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumCallHub **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumCallHubVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumCallHub * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumCallHub * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumCallHub * This);
        
        DECLSPEC_XFGVIRT(IEnumCallHub, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IEnumCallHub * This,
            /* [in] */ ULONG celt,
            /* [size_is][out] */ __RPC__out_ecount_full(celt) ITCallHub **ppElements,
            /* [full][out][in] */ __RPC__inout_opt ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IEnumCallHub, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumCallHub * This);
        
        DECLSPEC_XFGVIRT(IEnumCallHub, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumCallHub * This,
            /* [in] */ ULONG celt);
        
        DECLSPEC_XFGVIRT(IEnumCallHub, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumCallHub * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumCallHub **ppEnum);
        
        END_INTERFACE
    } IEnumCallHubVtbl;

    interface IEnumCallHub
    {
        CONST_VTBL struct IEnumCallHubVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumCallHub_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumCallHub_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumCallHub_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumCallHub_Next(This,celt,ppElements,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,ppElements,pceltFetched) ) 

#define IEnumCallHub_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumCallHub_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IEnumCallHub_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumCallHub_INTERFACE_DEFINED__ */


#ifndef __IEnumBstr_INTERFACE_DEFINED__
#define __IEnumBstr_INTERFACE_DEFINED__

/* interface IEnumBstr */
/* [object][unique][hidden][helpstring][uuid] */ 


EXTERN_C const IID IID_IEnumBstr;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("35372049-0BC6-11d2-A033-00C04FB6809F")
    IEnumBstr : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG celt,
            /* [size_is][out] */ __RPC__out_ecount_full(celt) BSTR *ppStrings,
            /* [full][out][in] */ __RPC__inout_opt ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumBstr **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumBstrVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumBstr * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumBstr * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumBstr * This);
        
        DECLSPEC_XFGVIRT(IEnumBstr, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IEnumBstr * This,
            /* [in] */ ULONG celt,
            /* [size_is][out] */ __RPC__out_ecount_full(celt) BSTR *ppStrings,
            /* [full][out][in] */ __RPC__inout_opt ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IEnumBstr, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumBstr * This);
        
        DECLSPEC_XFGVIRT(IEnumBstr, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumBstr * This,
            /* [in] */ ULONG celt);
        
        DECLSPEC_XFGVIRT(IEnumBstr, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumBstr * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumBstr **ppEnum);
        
        END_INTERFACE
    } IEnumBstrVtbl;

    interface IEnumBstr
    {
        CONST_VTBL struct IEnumBstrVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumBstr_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumBstr_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumBstr_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumBstr_Next(This,celt,ppStrings,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,ppStrings,pceltFetched) ) 

#define IEnumBstr_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumBstr_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IEnumBstr_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumBstr_INTERFACE_DEFINED__ */


#ifndef __IEnumPluggableTerminalClassInfo_INTERFACE_DEFINED__
#define __IEnumPluggableTerminalClassInfo_INTERFACE_DEFINED__

/* interface IEnumPluggableTerminalClassInfo */
/* [object][unique][hidden][helpstring][uuid] */ 


EXTERN_C const IID IID_IEnumPluggableTerminalClassInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4567450C-DBEE-4e3f-AAF5-37BF9EBF5E29")
    IEnumPluggableTerminalClassInfo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG celt,
            /* [size_is][out] */ __RPC__out_ecount_full(celt) ITPluggableTerminalClassInfo **ppElements,
            /* [full][out][in] */ __RPC__inout_opt ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumPluggableTerminalClassInfo **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumPluggableTerminalClassInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumPluggableTerminalClassInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumPluggableTerminalClassInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumPluggableTerminalClassInfo * This);
        
        DECLSPEC_XFGVIRT(IEnumPluggableTerminalClassInfo, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IEnumPluggableTerminalClassInfo * This,
            /* [in] */ ULONG celt,
            /* [size_is][out] */ __RPC__out_ecount_full(celt) ITPluggableTerminalClassInfo **ppElements,
            /* [full][out][in] */ __RPC__inout_opt ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IEnumPluggableTerminalClassInfo, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumPluggableTerminalClassInfo * This);
        
        DECLSPEC_XFGVIRT(IEnumPluggableTerminalClassInfo, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumPluggableTerminalClassInfo * This,
            /* [in] */ ULONG celt);
        
        DECLSPEC_XFGVIRT(IEnumPluggableTerminalClassInfo, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumPluggableTerminalClassInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumPluggableTerminalClassInfo **ppEnum);
        
        END_INTERFACE
    } IEnumPluggableTerminalClassInfoVtbl;

    interface IEnumPluggableTerminalClassInfo
    {
        CONST_VTBL struct IEnumPluggableTerminalClassInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumPluggableTerminalClassInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumPluggableTerminalClassInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumPluggableTerminalClassInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumPluggableTerminalClassInfo_Next(This,celt,ppElements,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,ppElements,pceltFetched) ) 

#define IEnumPluggableTerminalClassInfo_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumPluggableTerminalClassInfo_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IEnumPluggableTerminalClassInfo_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumPluggableTerminalClassInfo_INTERFACE_DEFINED__ */


#ifndef __IEnumPluggableSuperclassInfo_INTERFACE_DEFINED__
#define __IEnumPluggableSuperclassInfo_INTERFACE_DEFINED__

/* interface IEnumPluggableSuperclassInfo */
/* [object][unique][hidden][helpstring][uuid] */ 


EXTERN_C const IID IID_IEnumPluggableSuperclassInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("E9586A80-89E6-4cff-931D-478D5751F4C0")
    IEnumPluggableSuperclassInfo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG celt,
            /* [size_is][out] */ __RPC__out_ecount_full(celt) ITPluggableTerminalSuperclassInfo **ppElements,
            /* [full][out][in] */ __RPC__inout_opt ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumPluggableSuperclassInfo **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumPluggableSuperclassInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumPluggableSuperclassInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumPluggableSuperclassInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumPluggableSuperclassInfo * This);
        
        DECLSPEC_XFGVIRT(IEnumPluggableSuperclassInfo, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IEnumPluggableSuperclassInfo * This,
            /* [in] */ ULONG celt,
            /* [size_is][out] */ __RPC__out_ecount_full(celt) ITPluggableTerminalSuperclassInfo **ppElements,
            /* [full][out][in] */ __RPC__inout_opt ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IEnumPluggableSuperclassInfo, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumPluggableSuperclassInfo * This);
        
        DECLSPEC_XFGVIRT(IEnumPluggableSuperclassInfo, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumPluggableSuperclassInfo * This,
            /* [in] */ ULONG celt);
        
        DECLSPEC_XFGVIRT(IEnumPluggableSuperclassInfo, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumPluggableSuperclassInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumPluggableSuperclassInfo **ppEnum);
        
        END_INTERFACE
    } IEnumPluggableSuperclassInfoVtbl;

    interface IEnumPluggableSuperclassInfo
    {
        CONST_VTBL struct IEnumPluggableSuperclassInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumPluggableSuperclassInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumPluggableSuperclassInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumPluggableSuperclassInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumPluggableSuperclassInfo_Next(This,celt,ppElements,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,ppElements,pceltFetched) ) 

#define IEnumPluggableSuperclassInfo_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumPluggableSuperclassInfo_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IEnumPluggableSuperclassInfo_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumPluggableSuperclassInfo_INTERFACE_DEFINED__ */


#ifndef __ITPhoneEvent_INTERFACE_DEFINED__
#define __ITPhoneEvent_INTERFACE_DEFINED__

/* interface ITPhoneEvent */
/* [object][oleautomation][helpstring][uuid] */ 


EXTERN_C const IID IID_ITPhoneEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8F942DD8-64ED-4aaf-A77D-B23DB0837EAD")
    ITPhoneEvent : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Phone( 
            /* [retval][out] */ __RPC__deref_out_opt ITPhone **ppPhone) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Event( 
            /* [retval][out] */ __RPC__out PHONE_EVENT *pEvent) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ButtonState( 
            /* [retval][out] */ __RPC__out PHONE_BUTTON_STATE *pState) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_HookSwitchState( 
            /* [retval][out] */ __RPC__out PHONE_HOOK_SWITCH_STATE *pState) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_HookSwitchDevice( 
            /* [retval][out] */ __RPC__out PHONE_HOOK_SWITCH_DEVICE *pDevice) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RingMode( 
            /* [retval][out] */ __RPC__out long *plRingMode) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ButtonLampId( 
            /* [retval][out] */ __RPC__out long *plButtonLampId) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_NumberGathered( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppNumber) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Call( 
            /* [retval][out] */ __RPC__deref_out_opt ITCallInfo **ppCallInfo) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITPhoneEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITPhoneEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITPhoneEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITPhoneEvent * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITPhoneEvent * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITPhoneEvent * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITPhoneEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITPhoneEvent * This,
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
        
        DECLSPEC_XFGVIRT(ITPhoneEvent, get_Phone)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Phone )( 
            __RPC__in ITPhoneEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt ITPhone **ppPhone);
        
        DECLSPEC_XFGVIRT(ITPhoneEvent, get_Event)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Event )( 
            __RPC__in ITPhoneEvent * This,
            /* [retval][out] */ __RPC__out PHONE_EVENT *pEvent);
        
        DECLSPEC_XFGVIRT(ITPhoneEvent, get_ButtonState)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ButtonState )( 
            __RPC__in ITPhoneEvent * This,
            /* [retval][out] */ __RPC__out PHONE_BUTTON_STATE *pState);
        
        DECLSPEC_XFGVIRT(ITPhoneEvent, get_HookSwitchState)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_HookSwitchState )( 
            __RPC__in ITPhoneEvent * This,
            /* [retval][out] */ __RPC__out PHONE_HOOK_SWITCH_STATE *pState);
        
        DECLSPEC_XFGVIRT(ITPhoneEvent, get_HookSwitchDevice)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_HookSwitchDevice )( 
            __RPC__in ITPhoneEvent * This,
            /* [retval][out] */ __RPC__out PHONE_HOOK_SWITCH_DEVICE *pDevice);
        
        DECLSPEC_XFGVIRT(ITPhoneEvent, get_RingMode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RingMode )( 
            __RPC__in ITPhoneEvent * This,
            /* [retval][out] */ __RPC__out long *plRingMode);
        
        DECLSPEC_XFGVIRT(ITPhoneEvent, get_ButtonLampId)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ButtonLampId )( 
            __RPC__in ITPhoneEvent * This,
            /* [retval][out] */ __RPC__out long *plButtonLampId);
        
        DECLSPEC_XFGVIRT(ITPhoneEvent, get_NumberGathered)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NumberGathered )( 
            __RPC__in ITPhoneEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppNumber);
        
        DECLSPEC_XFGVIRT(ITPhoneEvent, get_Call)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Call )( 
            __RPC__in ITPhoneEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt ITCallInfo **ppCallInfo);
        
        END_INTERFACE
    } ITPhoneEventVtbl;

    interface ITPhoneEvent
    {
        CONST_VTBL struct ITPhoneEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITPhoneEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITPhoneEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITPhoneEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITPhoneEvent_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITPhoneEvent_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITPhoneEvent_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITPhoneEvent_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITPhoneEvent_get_Phone(This,ppPhone)	\
    ( (This)->lpVtbl -> get_Phone(This,ppPhone) ) 

#define ITPhoneEvent_get_Event(This,pEvent)	\
    ( (This)->lpVtbl -> get_Event(This,pEvent) ) 

#define ITPhoneEvent_get_ButtonState(This,pState)	\
    ( (This)->lpVtbl -> get_ButtonState(This,pState) ) 

#define ITPhoneEvent_get_HookSwitchState(This,pState)	\
    ( (This)->lpVtbl -> get_HookSwitchState(This,pState) ) 

#define ITPhoneEvent_get_HookSwitchDevice(This,pDevice)	\
    ( (This)->lpVtbl -> get_HookSwitchDevice(This,pDevice) ) 

#define ITPhoneEvent_get_RingMode(This,plRingMode)	\
    ( (This)->lpVtbl -> get_RingMode(This,plRingMode) ) 

#define ITPhoneEvent_get_ButtonLampId(This,plButtonLampId)	\
    ( (This)->lpVtbl -> get_ButtonLampId(This,plButtonLampId) ) 

#define ITPhoneEvent_get_NumberGathered(This,ppNumber)	\
    ( (This)->lpVtbl -> get_NumberGathered(This,ppNumber) ) 

#define ITPhoneEvent_get_Call(This,ppCallInfo)	\
    ( (This)->lpVtbl -> get_Call(This,ppCallInfo) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITPhoneEvent_INTERFACE_DEFINED__ */


#ifndef __ITCallStateEvent_INTERFACE_DEFINED__
#define __ITCallStateEvent_INTERFACE_DEFINED__

/* interface ITCallStateEvent */
/* [object][dual][helpstring][uuid] */ 


EXTERN_C const IID IID_ITCallStateEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("62F47097-95C9-11d0-835D-00AA003CCABD")
    ITCallStateEvent : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Call( 
            /* [retval][out] */ __RPC__deref_out_opt ITCallInfo **ppCallInfo) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_State( 
            /* [retval][out] */ __RPC__out CALL_STATE *pCallState) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Cause( 
            /* [retval][out] */ __RPC__out CALL_STATE_EVENT_CAUSE *pCEC) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CallbackInstance( 
            /* [retval][out] */ __RPC__out long *plCallbackInstance) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITCallStateEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITCallStateEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITCallStateEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITCallStateEvent * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITCallStateEvent * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITCallStateEvent * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITCallStateEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITCallStateEvent * This,
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
        
        DECLSPEC_XFGVIRT(ITCallStateEvent, get_Call)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Call )( 
            __RPC__in ITCallStateEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt ITCallInfo **ppCallInfo);
        
        DECLSPEC_XFGVIRT(ITCallStateEvent, get_State)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_State )( 
            __RPC__in ITCallStateEvent * This,
            /* [retval][out] */ __RPC__out CALL_STATE *pCallState);
        
        DECLSPEC_XFGVIRT(ITCallStateEvent, get_Cause)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Cause )( 
            __RPC__in ITCallStateEvent * This,
            /* [retval][out] */ __RPC__out CALL_STATE_EVENT_CAUSE *pCEC);
        
        DECLSPEC_XFGVIRT(ITCallStateEvent, get_CallbackInstance)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CallbackInstance )( 
            __RPC__in ITCallStateEvent * This,
            /* [retval][out] */ __RPC__out long *plCallbackInstance);
        
        END_INTERFACE
    } ITCallStateEventVtbl;

    interface ITCallStateEvent
    {
        CONST_VTBL struct ITCallStateEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITCallStateEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITCallStateEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITCallStateEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITCallStateEvent_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITCallStateEvent_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITCallStateEvent_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITCallStateEvent_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITCallStateEvent_get_Call(This,ppCallInfo)	\
    ( (This)->lpVtbl -> get_Call(This,ppCallInfo) ) 

#define ITCallStateEvent_get_State(This,pCallState)	\
    ( (This)->lpVtbl -> get_State(This,pCallState) ) 

#define ITCallStateEvent_get_Cause(This,pCEC)	\
    ( (This)->lpVtbl -> get_Cause(This,pCEC) ) 

#define ITCallStateEvent_get_CallbackInstance(This,plCallbackInstance)	\
    ( (This)->lpVtbl -> get_CallbackInstance(This,plCallbackInstance) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITCallStateEvent_INTERFACE_DEFINED__ */


#ifndef __ITPhoneDeviceSpecificEvent_INTERFACE_DEFINED__
#define __ITPhoneDeviceSpecificEvent_INTERFACE_DEFINED__

/* interface ITPhoneDeviceSpecificEvent */
/* [object][oleautomation][helpstring][uuid] */ 


EXTERN_C const IID IID_ITPhoneDeviceSpecificEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("63FFB2A6-872B-4cd3-A501-326E8FB40AF7")
    ITPhoneDeviceSpecificEvent : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Phone( 
            /* [retval][out] */ __RPC__deref_out_opt ITPhone **ppPhone) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_lParam1( 
            /* [retval][out] */ __RPC__out long *pParam1) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_lParam2( 
            /* [retval][out] */ __RPC__out long *pParam2) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_lParam3( 
            /* [retval][out] */ __RPC__out long *pParam3) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITPhoneDeviceSpecificEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITPhoneDeviceSpecificEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITPhoneDeviceSpecificEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITPhoneDeviceSpecificEvent * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITPhoneDeviceSpecificEvent * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITPhoneDeviceSpecificEvent * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITPhoneDeviceSpecificEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITPhoneDeviceSpecificEvent * This,
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
        
        DECLSPEC_XFGVIRT(ITPhoneDeviceSpecificEvent, get_Phone)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Phone )( 
            __RPC__in ITPhoneDeviceSpecificEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt ITPhone **ppPhone);
        
        DECLSPEC_XFGVIRT(ITPhoneDeviceSpecificEvent, get_lParam1)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_lParam1 )( 
            __RPC__in ITPhoneDeviceSpecificEvent * This,
            /* [retval][out] */ __RPC__out long *pParam1);
        
        DECLSPEC_XFGVIRT(ITPhoneDeviceSpecificEvent, get_lParam2)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_lParam2 )( 
            __RPC__in ITPhoneDeviceSpecificEvent * This,
            /* [retval][out] */ __RPC__out long *pParam2);
        
        DECLSPEC_XFGVIRT(ITPhoneDeviceSpecificEvent, get_lParam3)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_lParam3 )( 
            __RPC__in ITPhoneDeviceSpecificEvent * This,
            /* [retval][out] */ __RPC__out long *pParam3);
        
        END_INTERFACE
    } ITPhoneDeviceSpecificEventVtbl;

    interface ITPhoneDeviceSpecificEvent
    {
        CONST_VTBL struct ITPhoneDeviceSpecificEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITPhoneDeviceSpecificEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITPhoneDeviceSpecificEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITPhoneDeviceSpecificEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITPhoneDeviceSpecificEvent_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITPhoneDeviceSpecificEvent_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITPhoneDeviceSpecificEvent_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITPhoneDeviceSpecificEvent_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITPhoneDeviceSpecificEvent_get_Phone(This,ppPhone)	\
    ( (This)->lpVtbl -> get_Phone(This,ppPhone) ) 

#define ITPhoneDeviceSpecificEvent_get_lParam1(This,pParam1)	\
    ( (This)->lpVtbl -> get_lParam1(This,pParam1) ) 

#define ITPhoneDeviceSpecificEvent_get_lParam2(This,pParam2)	\
    ( (This)->lpVtbl -> get_lParam2(This,pParam2) ) 

#define ITPhoneDeviceSpecificEvent_get_lParam3(This,pParam3)	\
    ( (This)->lpVtbl -> get_lParam3(This,pParam3) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITPhoneDeviceSpecificEvent_INTERFACE_DEFINED__ */


#ifndef __ITCallMediaEvent_INTERFACE_DEFINED__
#define __ITCallMediaEvent_INTERFACE_DEFINED__

/* interface ITCallMediaEvent */
/* [object][dual][helpstring][uuid] */ 


EXTERN_C const IID IID_ITCallMediaEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("FF36B87F-EC3A-11d0-8EE4-00C04FB6809F")
    ITCallMediaEvent : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Call( 
            /* [retval][out] */ __RPC__deref_out_opt ITCallInfo **ppCallInfo) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Event( 
            /* [retval][out] */ __RPC__out CALL_MEDIA_EVENT *pCallMediaEvent) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Error( 
            /* [retval][out] */ __RPC__out HRESULT *phrError) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Terminal( 
            /* [retval][out] */ __RPC__deref_out_opt ITTerminal **ppTerminal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Stream( 
            /* [retval][out] */ __RPC__deref_out_opt ITStream **ppStream) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Cause( 
            /* [retval][out] */ __RPC__out CALL_MEDIA_EVENT_CAUSE *pCause) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITCallMediaEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITCallMediaEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITCallMediaEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITCallMediaEvent * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITCallMediaEvent * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITCallMediaEvent * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITCallMediaEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITCallMediaEvent * This,
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
        
        DECLSPEC_XFGVIRT(ITCallMediaEvent, get_Call)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Call )( 
            __RPC__in ITCallMediaEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt ITCallInfo **ppCallInfo);
        
        DECLSPEC_XFGVIRT(ITCallMediaEvent, get_Event)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Event )( 
            __RPC__in ITCallMediaEvent * This,
            /* [retval][out] */ __RPC__out CALL_MEDIA_EVENT *pCallMediaEvent);
        
        DECLSPEC_XFGVIRT(ITCallMediaEvent, get_Error)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Error )( 
            __RPC__in ITCallMediaEvent * This,
            /* [retval][out] */ __RPC__out HRESULT *phrError);
        
        DECLSPEC_XFGVIRT(ITCallMediaEvent, get_Terminal)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Terminal )( 
            __RPC__in ITCallMediaEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt ITTerminal **ppTerminal);
        
        DECLSPEC_XFGVIRT(ITCallMediaEvent, get_Stream)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Stream )( 
            __RPC__in ITCallMediaEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt ITStream **ppStream);
        
        DECLSPEC_XFGVIRT(ITCallMediaEvent, get_Cause)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Cause )( 
            __RPC__in ITCallMediaEvent * This,
            /* [retval][out] */ __RPC__out CALL_MEDIA_EVENT_CAUSE *pCause);
        
        END_INTERFACE
    } ITCallMediaEventVtbl;

    interface ITCallMediaEvent
    {
        CONST_VTBL struct ITCallMediaEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITCallMediaEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITCallMediaEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITCallMediaEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITCallMediaEvent_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITCallMediaEvent_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITCallMediaEvent_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITCallMediaEvent_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITCallMediaEvent_get_Call(This,ppCallInfo)	\
    ( (This)->lpVtbl -> get_Call(This,ppCallInfo) ) 

#define ITCallMediaEvent_get_Event(This,pCallMediaEvent)	\
    ( (This)->lpVtbl -> get_Event(This,pCallMediaEvent) ) 

#define ITCallMediaEvent_get_Error(This,phrError)	\
    ( (This)->lpVtbl -> get_Error(This,phrError) ) 

#define ITCallMediaEvent_get_Terminal(This,ppTerminal)	\
    ( (This)->lpVtbl -> get_Terminal(This,ppTerminal) ) 

#define ITCallMediaEvent_get_Stream(This,ppStream)	\
    ( (This)->lpVtbl -> get_Stream(This,ppStream) ) 

#define ITCallMediaEvent_get_Cause(This,pCause)	\
    ( (This)->lpVtbl -> get_Cause(This,pCause) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITCallMediaEvent_INTERFACE_DEFINED__ */


#ifndef __ITDigitDetectionEvent_INTERFACE_DEFINED__
#define __ITDigitDetectionEvent_INTERFACE_DEFINED__

/* interface ITDigitDetectionEvent */
/* [object][oleautomation][helpstring][uuid] */ 


EXTERN_C const IID IID_ITDigitDetectionEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("80D3BFAC-57D9-11d2-A04A-00C04FB6809F")
    ITDigitDetectionEvent : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Call( 
            /* [retval][out] */ __RPC__deref_out_opt ITCallInfo **ppCallInfo) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Digit( 
            /* [retval][out] */ __RPC__out unsigned char *pucDigit) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DigitMode( 
            /* [retval][out] */ __RPC__out TAPI_DIGITMODE *pDigitMode) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TickCount( 
            /* [retval][out] */ __RPC__out long *plTickCount) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CallbackInstance( 
            /* [retval][out] */ __RPC__out long *plCallbackInstance) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITDigitDetectionEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITDigitDetectionEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITDigitDetectionEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITDigitDetectionEvent * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITDigitDetectionEvent * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITDigitDetectionEvent * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITDigitDetectionEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITDigitDetectionEvent * This,
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
        
        DECLSPEC_XFGVIRT(ITDigitDetectionEvent, get_Call)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Call )( 
            __RPC__in ITDigitDetectionEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt ITCallInfo **ppCallInfo);
        
        DECLSPEC_XFGVIRT(ITDigitDetectionEvent, get_Digit)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Digit )( 
            __RPC__in ITDigitDetectionEvent * This,
            /* [retval][out] */ __RPC__out unsigned char *pucDigit);
        
        DECLSPEC_XFGVIRT(ITDigitDetectionEvent, get_DigitMode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DigitMode )( 
            __RPC__in ITDigitDetectionEvent * This,
            /* [retval][out] */ __RPC__out TAPI_DIGITMODE *pDigitMode);
        
        DECLSPEC_XFGVIRT(ITDigitDetectionEvent, get_TickCount)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TickCount )( 
            __RPC__in ITDigitDetectionEvent * This,
            /* [retval][out] */ __RPC__out long *plTickCount);
        
        DECLSPEC_XFGVIRT(ITDigitDetectionEvent, get_CallbackInstance)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CallbackInstance )( 
            __RPC__in ITDigitDetectionEvent * This,
            /* [retval][out] */ __RPC__out long *plCallbackInstance);
        
        END_INTERFACE
    } ITDigitDetectionEventVtbl;

    interface ITDigitDetectionEvent
    {
        CONST_VTBL struct ITDigitDetectionEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITDigitDetectionEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITDigitDetectionEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITDigitDetectionEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITDigitDetectionEvent_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITDigitDetectionEvent_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITDigitDetectionEvent_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITDigitDetectionEvent_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITDigitDetectionEvent_get_Call(This,ppCallInfo)	\
    ( (This)->lpVtbl -> get_Call(This,ppCallInfo) ) 

#define ITDigitDetectionEvent_get_Digit(This,pucDigit)	\
    ( (This)->lpVtbl -> get_Digit(This,pucDigit) ) 

#define ITDigitDetectionEvent_get_DigitMode(This,pDigitMode)	\
    ( (This)->lpVtbl -> get_DigitMode(This,pDigitMode) ) 

#define ITDigitDetectionEvent_get_TickCount(This,plTickCount)	\
    ( (This)->lpVtbl -> get_TickCount(This,plTickCount) ) 

#define ITDigitDetectionEvent_get_CallbackInstance(This,plCallbackInstance)	\
    ( (This)->lpVtbl -> get_CallbackInstance(This,plCallbackInstance) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITDigitDetectionEvent_INTERFACE_DEFINED__ */


#ifndef __ITDigitGenerationEvent_INTERFACE_DEFINED__
#define __ITDigitGenerationEvent_INTERFACE_DEFINED__

/* interface ITDigitGenerationEvent */
/* [object][dual][helpstring][uuid] */ 


EXTERN_C const IID IID_ITDigitGenerationEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("80D3BFAD-57D9-11d2-A04A-00C04FB6809F")
    ITDigitGenerationEvent : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Call( 
            /* [retval][out] */ __RPC__deref_out_opt ITCallInfo **ppCallInfo) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_GenerationTermination( 
            /* [retval][out] */ __RPC__out long *plGenerationTermination) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TickCount( 
            /* [retval][out] */ __RPC__out long *plTickCount) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CallbackInstance( 
            /* [retval][out] */ __RPC__out long *plCallbackInstance) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITDigitGenerationEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITDigitGenerationEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITDigitGenerationEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITDigitGenerationEvent * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITDigitGenerationEvent * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITDigitGenerationEvent * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITDigitGenerationEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITDigitGenerationEvent * This,
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
        
        DECLSPEC_XFGVIRT(ITDigitGenerationEvent, get_Call)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Call )( 
            __RPC__in ITDigitGenerationEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt ITCallInfo **ppCallInfo);
        
        DECLSPEC_XFGVIRT(ITDigitGenerationEvent, get_GenerationTermination)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_GenerationTermination )( 
            __RPC__in ITDigitGenerationEvent * This,
            /* [retval][out] */ __RPC__out long *plGenerationTermination);
        
        DECLSPEC_XFGVIRT(ITDigitGenerationEvent, get_TickCount)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TickCount )( 
            __RPC__in ITDigitGenerationEvent * This,
            /* [retval][out] */ __RPC__out long *plTickCount);
        
        DECLSPEC_XFGVIRT(ITDigitGenerationEvent, get_CallbackInstance)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CallbackInstance )( 
            __RPC__in ITDigitGenerationEvent * This,
            /* [retval][out] */ __RPC__out long *plCallbackInstance);
        
        END_INTERFACE
    } ITDigitGenerationEventVtbl;

    interface ITDigitGenerationEvent
    {
        CONST_VTBL struct ITDigitGenerationEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITDigitGenerationEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITDigitGenerationEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITDigitGenerationEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITDigitGenerationEvent_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITDigitGenerationEvent_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITDigitGenerationEvent_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITDigitGenerationEvent_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITDigitGenerationEvent_get_Call(This,ppCallInfo)	\
    ( (This)->lpVtbl -> get_Call(This,ppCallInfo) ) 

#define ITDigitGenerationEvent_get_GenerationTermination(This,plGenerationTermination)	\
    ( (This)->lpVtbl -> get_GenerationTermination(This,plGenerationTermination) ) 

#define ITDigitGenerationEvent_get_TickCount(This,plTickCount)	\
    ( (This)->lpVtbl -> get_TickCount(This,plTickCount) ) 

#define ITDigitGenerationEvent_get_CallbackInstance(This,plCallbackInstance)	\
    ( (This)->lpVtbl -> get_CallbackInstance(This,plCallbackInstance) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITDigitGenerationEvent_INTERFACE_DEFINED__ */


#ifndef __ITDigitsGatheredEvent_INTERFACE_DEFINED__
#define __ITDigitsGatheredEvent_INTERFACE_DEFINED__

/* interface ITDigitsGatheredEvent */
/* [object][dual][helpstring][uuid] */ 


EXTERN_C const IID IID_ITDigitsGatheredEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("E52EC4C1-CBA3-441a-9E6A-93CB909E9724")
    ITDigitsGatheredEvent : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Call( 
            /* [retval][out] */ __RPC__deref_out_opt ITCallInfo **ppCallInfo) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Digits( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppDigits) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_GatherTermination( 
            /* [retval][out] */ __RPC__out TAPI_GATHERTERM *pGatherTermination) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TickCount( 
            /* [retval][out] */ __RPC__out long *plTickCount) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CallbackInstance( 
            /* [retval][out] */ __RPC__out long *plCallbackInstance) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITDigitsGatheredEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITDigitsGatheredEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITDigitsGatheredEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITDigitsGatheredEvent * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITDigitsGatheredEvent * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITDigitsGatheredEvent * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITDigitsGatheredEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITDigitsGatheredEvent * This,
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
        
        DECLSPEC_XFGVIRT(ITDigitsGatheredEvent, get_Call)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Call )( 
            __RPC__in ITDigitsGatheredEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt ITCallInfo **ppCallInfo);
        
        DECLSPEC_XFGVIRT(ITDigitsGatheredEvent, get_Digits)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Digits )( 
            __RPC__in ITDigitsGatheredEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppDigits);
        
        DECLSPEC_XFGVIRT(ITDigitsGatheredEvent, get_GatherTermination)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_GatherTermination )( 
            __RPC__in ITDigitsGatheredEvent * This,
            /* [retval][out] */ __RPC__out TAPI_GATHERTERM *pGatherTermination);
        
        DECLSPEC_XFGVIRT(ITDigitsGatheredEvent, get_TickCount)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TickCount )( 
            __RPC__in ITDigitsGatheredEvent * This,
            /* [retval][out] */ __RPC__out long *plTickCount);
        
        DECLSPEC_XFGVIRT(ITDigitsGatheredEvent, get_CallbackInstance)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CallbackInstance )( 
            __RPC__in ITDigitsGatheredEvent * This,
            /* [retval][out] */ __RPC__out long *plCallbackInstance);
        
        END_INTERFACE
    } ITDigitsGatheredEventVtbl;

    interface ITDigitsGatheredEvent
    {
        CONST_VTBL struct ITDigitsGatheredEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITDigitsGatheredEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITDigitsGatheredEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITDigitsGatheredEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITDigitsGatheredEvent_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITDigitsGatheredEvent_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITDigitsGatheredEvent_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITDigitsGatheredEvent_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITDigitsGatheredEvent_get_Call(This,ppCallInfo)	\
    ( (This)->lpVtbl -> get_Call(This,ppCallInfo) ) 

#define ITDigitsGatheredEvent_get_Digits(This,ppDigits)	\
    ( (This)->lpVtbl -> get_Digits(This,ppDigits) ) 

#define ITDigitsGatheredEvent_get_GatherTermination(This,pGatherTermination)	\
    ( (This)->lpVtbl -> get_GatherTermination(This,pGatherTermination) ) 

#define ITDigitsGatheredEvent_get_TickCount(This,plTickCount)	\
    ( (This)->lpVtbl -> get_TickCount(This,plTickCount) ) 

#define ITDigitsGatheredEvent_get_CallbackInstance(This,plCallbackInstance)	\
    ( (This)->lpVtbl -> get_CallbackInstance(This,plCallbackInstance) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITDigitsGatheredEvent_INTERFACE_DEFINED__ */


#ifndef __ITToneDetectionEvent_INTERFACE_DEFINED__
#define __ITToneDetectionEvent_INTERFACE_DEFINED__

/* interface ITToneDetectionEvent */
/* [object][dual][helpstring][uuid] */ 


EXTERN_C const IID IID_ITToneDetectionEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("407E0FAF-D047-4753-B0C6-8E060373FECD")
    ITToneDetectionEvent : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Call( 
            /* [retval][out] */ __RPC__deref_out_opt ITCallInfo **ppCallInfo) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_AppSpecific( 
            /* [retval][out] */ __RPC__out long *plAppSpecific) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TickCount( 
            /* [retval][out] */ __RPC__out long *plTickCount) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CallbackInstance( 
            /* [retval][out] */ __RPC__out long *plCallbackInstance) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITToneDetectionEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITToneDetectionEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITToneDetectionEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITToneDetectionEvent * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITToneDetectionEvent * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITToneDetectionEvent * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITToneDetectionEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITToneDetectionEvent * This,
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
        
        DECLSPEC_XFGVIRT(ITToneDetectionEvent, get_Call)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Call )( 
            __RPC__in ITToneDetectionEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt ITCallInfo **ppCallInfo);
        
        DECLSPEC_XFGVIRT(ITToneDetectionEvent, get_AppSpecific)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AppSpecific )( 
            __RPC__in ITToneDetectionEvent * This,
            /* [retval][out] */ __RPC__out long *plAppSpecific);
        
        DECLSPEC_XFGVIRT(ITToneDetectionEvent, get_TickCount)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TickCount )( 
            __RPC__in ITToneDetectionEvent * This,
            /* [retval][out] */ __RPC__out long *plTickCount);
        
        DECLSPEC_XFGVIRT(ITToneDetectionEvent, get_CallbackInstance)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CallbackInstance )( 
            __RPC__in ITToneDetectionEvent * This,
            /* [retval][out] */ __RPC__out long *plCallbackInstance);
        
        END_INTERFACE
    } ITToneDetectionEventVtbl;

    interface ITToneDetectionEvent
    {
        CONST_VTBL struct ITToneDetectionEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITToneDetectionEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITToneDetectionEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITToneDetectionEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITToneDetectionEvent_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITToneDetectionEvent_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITToneDetectionEvent_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITToneDetectionEvent_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITToneDetectionEvent_get_Call(This,ppCallInfo)	\
    ( (This)->lpVtbl -> get_Call(This,ppCallInfo) ) 

#define ITToneDetectionEvent_get_AppSpecific(This,plAppSpecific)	\
    ( (This)->lpVtbl -> get_AppSpecific(This,plAppSpecific) ) 

#define ITToneDetectionEvent_get_TickCount(This,plTickCount)	\
    ( (This)->lpVtbl -> get_TickCount(This,plTickCount) ) 

#define ITToneDetectionEvent_get_CallbackInstance(This,plCallbackInstance)	\
    ( (This)->lpVtbl -> get_CallbackInstance(This,plCallbackInstance) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITToneDetectionEvent_INTERFACE_DEFINED__ */


#ifndef __ITTAPIObjectEvent_INTERFACE_DEFINED__
#define __ITTAPIObjectEvent_INTERFACE_DEFINED__

/* interface ITTAPIObjectEvent */
/* [object][dual][helpstring][uuid] */ 


EXTERN_C const IID IID_ITTAPIObjectEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F4854D48-937A-11d1-BB58-00C04FB6809F")
    ITTAPIObjectEvent : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TAPIObject( 
            /* [retval][out] */ __RPC__deref_out_opt ITTAPI **ppTAPIObject) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Event( 
            /* [retval][out] */ __RPC__out TAPIOBJECT_EVENT *pEvent) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Address( 
            /* [retval][out] */ __RPC__deref_out_opt ITAddress **ppAddress) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CallbackInstance( 
            /* [retval][out] */ __RPC__out long *plCallbackInstance) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITTAPIObjectEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITTAPIObjectEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITTAPIObjectEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITTAPIObjectEvent * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITTAPIObjectEvent * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITTAPIObjectEvent * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITTAPIObjectEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITTAPIObjectEvent * This,
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
        
        DECLSPEC_XFGVIRT(ITTAPIObjectEvent, get_TAPIObject)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TAPIObject )( 
            __RPC__in ITTAPIObjectEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt ITTAPI **ppTAPIObject);
        
        DECLSPEC_XFGVIRT(ITTAPIObjectEvent, get_Event)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Event )( 
            __RPC__in ITTAPIObjectEvent * This,
            /* [retval][out] */ __RPC__out TAPIOBJECT_EVENT *pEvent);
        
        DECLSPEC_XFGVIRT(ITTAPIObjectEvent, get_Address)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Address )( 
            __RPC__in ITTAPIObjectEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt ITAddress **ppAddress);
        
        DECLSPEC_XFGVIRT(ITTAPIObjectEvent, get_CallbackInstance)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CallbackInstance )( 
            __RPC__in ITTAPIObjectEvent * This,
            /* [retval][out] */ __RPC__out long *plCallbackInstance);
        
        END_INTERFACE
    } ITTAPIObjectEventVtbl;

    interface ITTAPIObjectEvent
    {
        CONST_VTBL struct ITTAPIObjectEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITTAPIObjectEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITTAPIObjectEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITTAPIObjectEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITTAPIObjectEvent_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITTAPIObjectEvent_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITTAPIObjectEvent_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITTAPIObjectEvent_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITTAPIObjectEvent_get_TAPIObject(This,ppTAPIObject)	\
    ( (This)->lpVtbl -> get_TAPIObject(This,ppTAPIObject) ) 

#define ITTAPIObjectEvent_get_Event(This,pEvent)	\
    ( (This)->lpVtbl -> get_Event(This,pEvent) ) 

#define ITTAPIObjectEvent_get_Address(This,ppAddress)	\
    ( (This)->lpVtbl -> get_Address(This,ppAddress) ) 

#define ITTAPIObjectEvent_get_CallbackInstance(This,plCallbackInstance)	\
    ( (This)->lpVtbl -> get_CallbackInstance(This,plCallbackInstance) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITTAPIObjectEvent_INTERFACE_DEFINED__ */


#ifndef __ITTAPIObjectEvent2_INTERFACE_DEFINED__
#define __ITTAPIObjectEvent2_INTERFACE_DEFINED__

/* interface ITTAPIObjectEvent2 */
/* [object][oleautomation][helpstring][uuid] */ 


EXTERN_C const IID IID_ITTAPIObjectEvent2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("359DDA6E-68CE-4383-BF0B-169133C41B46")
    ITTAPIObjectEvent2 : public ITTAPIObjectEvent
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Phone( 
            /* [retval][out] */ __RPC__deref_out_opt ITPhone **ppPhone) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITTAPIObjectEvent2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITTAPIObjectEvent2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITTAPIObjectEvent2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITTAPIObjectEvent2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITTAPIObjectEvent2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITTAPIObjectEvent2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITTAPIObjectEvent2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITTAPIObjectEvent2 * This,
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
        
        DECLSPEC_XFGVIRT(ITTAPIObjectEvent, get_TAPIObject)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TAPIObject )( 
            __RPC__in ITTAPIObjectEvent2 * This,
            /* [retval][out] */ __RPC__deref_out_opt ITTAPI **ppTAPIObject);
        
        DECLSPEC_XFGVIRT(ITTAPIObjectEvent, get_Event)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Event )( 
            __RPC__in ITTAPIObjectEvent2 * This,
            /* [retval][out] */ __RPC__out TAPIOBJECT_EVENT *pEvent);
        
        DECLSPEC_XFGVIRT(ITTAPIObjectEvent, get_Address)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Address )( 
            __RPC__in ITTAPIObjectEvent2 * This,
            /* [retval][out] */ __RPC__deref_out_opt ITAddress **ppAddress);
        
        DECLSPEC_XFGVIRT(ITTAPIObjectEvent, get_CallbackInstance)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CallbackInstance )( 
            __RPC__in ITTAPIObjectEvent2 * This,
            /* [retval][out] */ __RPC__out long *plCallbackInstance);
        
        DECLSPEC_XFGVIRT(ITTAPIObjectEvent2, get_Phone)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Phone )( 
            __RPC__in ITTAPIObjectEvent2 * This,
            /* [retval][out] */ __RPC__deref_out_opt ITPhone **ppPhone);
        
        END_INTERFACE
    } ITTAPIObjectEvent2Vtbl;

    interface ITTAPIObjectEvent2
    {
        CONST_VTBL struct ITTAPIObjectEvent2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITTAPIObjectEvent2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITTAPIObjectEvent2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITTAPIObjectEvent2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITTAPIObjectEvent2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITTAPIObjectEvent2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITTAPIObjectEvent2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITTAPIObjectEvent2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITTAPIObjectEvent2_get_TAPIObject(This,ppTAPIObject)	\
    ( (This)->lpVtbl -> get_TAPIObject(This,ppTAPIObject) ) 

#define ITTAPIObjectEvent2_get_Event(This,pEvent)	\
    ( (This)->lpVtbl -> get_Event(This,pEvent) ) 

#define ITTAPIObjectEvent2_get_Address(This,ppAddress)	\
    ( (This)->lpVtbl -> get_Address(This,ppAddress) ) 

#define ITTAPIObjectEvent2_get_CallbackInstance(This,plCallbackInstance)	\
    ( (This)->lpVtbl -> get_CallbackInstance(This,plCallbackInstance) ) 


#define ITTAPIObjectEvent2_get_Phone(This,ppPhone)	\
    ( (This)->lpVtbl -> get_Phone(This,ppPhone) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITTAPIObjectEvent2_INTERFACE_DEFINED__ */


#ifndef __ITTAPIEventNotification_INTERFACE_DEFINED__
#define __ITTAPIEventNotification_INTERFACE_DEFINED__

/* interface ITTAPIEventNotification */
/* [object][oleautomation][helpstring][uuid] */ 


EXTERN_C const IID IID_ITTAPIEventNotification;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("EDDB9426-3B91-11d1-8F30-00C04FB6809F")
    ITTAPIEventNotification : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Event( 
            /* [in] */ TAPI_EVENT TapiEvent,
            /* [in] */ __RPC__in_opt IDispatch *pEvent) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITTAPIEventNotificationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITTAPIEventNotification * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITTAPIEventNotification * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITTAPIEventNotification * This);
        
        DECLSPEC_XFGVIRT(ITTAPIEventNotification, Event)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Event )( 
            __RPC__in ITTAPIEventNotification * This,
            /* [in] */ TAPI_EVENT TapiEvent,
            /* [in] */ __RPC__in_opt IDispatch *pEvent);
        
        END_INTERFACE
    } ITTAPIEventNotificationVtbl;

    interface ITTAPIEventNotification
    {
        CONST_VTBL struct ITTAPIEventNotificationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITTAPIEventNotification_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITTAPIEventNotification_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITTAPIEventNotification_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITTAPIEventNotification_Event(This,TapiEvent,pEvent)	\
    ( (This)->lpVtbl -> Event(This,TapiEvent,pEvent) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITTAPIEventNotification_INTERFACE_DEFINED__ */


#ifndef __ITCallHubEvent_INTERFACE_DEFINED__
#define __ITCallHubEvent_INTERFACE_DEFINED__

/* interface ITCallHubEvent */
/* [object][oleautomation][helpstring][uuid] */ 


EXTERN_C const IID IID_ITCallHubEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A3C15451-5B92-11d1-8F4E-00C04FB6809F")
    ITCallHubEvent : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Event( 
            /* [retval][out] */ __RPC__out CALLHUB_EVENT *pEvent) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CallHub( 
            /* [retval][out] */ __RPC__deref_out_opt ITCallHub **ppCallHub) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Call( 
            /* [retval][out] */ __RPC__deref_out_opt ITCallInfo **ppCall) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITCallHubEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITCallHubEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITCallHubEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITCallHubEvent * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITCallHubEvent * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITCallHubEvent * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITCallHubEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITCallHubEvent * This,
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
        
        DECLSPEC_XFGVIRT(ITCallHubEvent, get_Event)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Event )( 
            __RPC__in ITCallHubEvent * This,
            /* [retval][out] */ __RPC__out CALLHUB_EVENT *pEvent);
        
        DECLSPEC_XFGVIRT(ITCallHubEvent, get_CallHub)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CallHub )( 
            __RPC__in ITCallHubEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt ITCallHub **ppCallHub);
        
        DECLSPEC_XFGVIRT(ITCallHubEvent, get_Call)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Call )( 
            __RPC__in ITCallHubEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt ITCallInfo **ppCall);
        
        END_INTERFACE
    } ITCallHubEventVtbl;

    interface ITCallHubEvent
    {
        CONST_VTBL struct ITCallHubEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITCallHubEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITCallHubEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITCallHubEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITCallHubEvent_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITCallHubEvent_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITCallHubEvent_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITCallHubEvent_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITCallHubEvent_get_Event(This,pEvent)	\
    ( (This)->lpVtbl -> get_Event(This,pEvent) ) 

#define ITCallHubEvent_get_CallHub(This,ppCallHub)	\
    ( (This)->lpVtbl -> get_CallHub(This,ppCallHub) ) 

#define ITCallHubEvent_get_Call(This,ppCall)	\
    ( (This)->lpVtbl -> get_Call(This,ppCall) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITCallHubEvent_INTERFACE_DEFINED__ */


#ifndef __ITAddressEvent_INTERFACE_DEFINED__
#define __ITAddressEvent_INTERFACE_DEFINED__

/* interface ITAddressEvent */
/* [object][oleautomation][helpstring][uuid] */ 


EXTERN_C const IID IID_ITAddressEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("831CE2D1-83B5-11d1-BB5C-00C04FB6809F")
    ITAddressEvent : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Address( 
            /* [retval][out] */ __RPC__deref_out_opt ITAddress **ppAddress) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Event( 
            /* [retval][out] */ __RPC__out ADDRESS_EVENT *pEvent) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Terminal( 
            /* [retval][out] */ __RPC__deref_out_opt ITTerminal **ppTerminal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITAddressEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITAddressEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITAddressEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITAddressEvent * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITAddressEvent * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITAddressEvent * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITAddressEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITAddressEvent * This,
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
        
        DECLSPEC_XFGVIRT(ITAddressEvent, get_Address)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Address )( 
            __RPC__in ITAddressEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt ITAddress **ppAddress);
        
        DECLSPEC_XFGVIRT(ITAddressEvent, get_Event)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Event )( 
            __RPC__in ITAddressEvent * This,
            /* [retval][out] */ __RPC__out ADDRESS_EVENT *pEvent);
        
        DECLSPEC_XFGVIRT(ITAddressEvent, get_Terminal)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Terminal )( 
            __RPC__in ITAddressEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt ITTerminal **ppTerminal);
        
        END_INTERFACE
    } ITAddressEventVtbl;

    interface ITAddressEvent
    {
        CONST_VTBL struct ITAddressEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITAddressEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITAddressEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITAddressEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITAddressEvent_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITAddressEvent_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITAddressEvent_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITAddressEvent_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITAddressEvent_get_Address(This,ppAddress)	\
    ( (This)->lpVtbl -> get_Address(This,ppAddress) ) 

#define ITAddressEvent_get_Event(This,pEvent)	\
    ( (This)->lpVtbl -> get_Event(This,pEvent) ) 

#define ITAddressEvent_get_Terminal(This,ppTerminal)	\
    ( (This)->lpVtbl -> get_Terminal(This,ppTerminal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITAddressEvent_INTERFACE_DEFINED__ */


#ifndef __ITAddressDeviceSpecificEvent_INTERFACE_DEFINED__
#define __ITAddressDeviceSpecificEvent_INTERFACE_DEFINED__

/* interface ITAddressDeviceSpecificEvent */
/* [object][oleautomation][helpstring][uuid] */ 


EXTERN_C const IID IID_ITAddressDeviceSpecificEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3ACB216B-40BD-487a-8672-5CE77BD7E3A3")
    ITAddressDeviceSpecificEvent : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Address( 
            /* [retval][out] */ __RPC__deref_out_opt ITAddress **ppAddress) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Call( 
            /* [retval][out] */ __RPC__deref_out_opt ITCallInfo **ppCall) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_lParam1( 
            /* [retval][out] */ __RPC__out long *pParam1) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_lParam2( 
            /* [retval][out] */ __RPC__out long *pParam2) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_lParam3( 
            /* [retval][out] */ __RPC__out long *pParam3) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITAddressDeviceSpecificEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITAddressDeviceSpecificEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITAddressDeviceSpecificEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITAddressDeviceSpecificEvent * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITAddressDeviceSpecificEvent * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITAddressDeviceSpecificEvent * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITAddressDeviceSpecificEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITAddressDeviceSpecificEvent * This,
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
        
        DECLSPEC_XFGVIRT(ITAddressDeviceSpecificEvent, get_Address)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Address )( 
            __RPC__in ITAddressDeviceSpecificEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt ITAddress **ppAddress);
        
        DECLSPEC_XFGVIRT(ITAddressDeviceSpecificEvent, get_Call)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Call )( 
            __RPC__in ITAddressDeviceSpecificEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt ITCallInfo **ppCall);
        
        DECLSPEC_XFGVIRT(ITAddressDeviceSpecificEvent, get_lParam1)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_lParam1 )( 
            __RPC__in ITAddressDeviceSpecificEvent * This,
            /* [retval][out] */ __RPC__out long *pParam1);
        
        DECLSPEC_XFGVIRT(ITAddressDeviceSpecificEvent, get_lParam2)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_lParam2 )( 
            __RPC__in ITAddressDeviceSpecificEvent * This,
            /* [retval][out] */ __RPC__out long *pParam2);
        
        DECLSPEC_XFGVIRT(ITAddressDeviceSpecificEvent, get_lParam3)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_lParam3 )( 
            __RPC__in ITAddressDeviceSpecificEvent * This,
            /* [retval][out] */ __RPC__out long *pParam3);
        
        END_INTERFACE
    } ITAddressDeviceSpecificEventVtbl;

    interface ITAddressDeviceSpecificEvent
    {
        CONST_VTBL struct ITAddressDeviceSpecificEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITAddressDeviceSpecificEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITAddressDeviceSpecificEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITAddressDeviceSpecificEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITAddressDeviceSpecificEvent_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITAddressDeviceSpecificEvent_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITAddressDeviceSpecificEvent_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITAddressDeviceSpecificEvent_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITAddressDeviceSpecificEvent_get_Address(This,ppAddress)	\
    ( (This)->lpVtbl -> get_Address(This,ppAddress) ) 

#define ITAddressDeviceSpecificEvent_get_Call(This,ppCall)	\
    ( (This)->lpVtbl -> get_Call(This,ppCall) ) 

#define ITAddressDeviceSpecificEvent_get_lParam1(This,pParam1)	\
    ( (This)->lpVtbl -> get_lParam1(This,pParam1) ) 

#define ITAddressDeviceSpecificEvent_get_lParam2(This,pParam2)	\
    ( (This)->lpVtbl -> get_lParam2(This,pParam2) ) 

#define ITAddressDeviceSpecificEvent_get_lParam3(This,pParam3)	\
    ( (This)->lpVtbl -> get_lParam3(This,pParam3) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITAddressDeviceSpecificEvent_INTERFACE_DEFINED__ */


#ifndef __ITFileTerminalEvent_INTERFACE_DEFINED__
#define __ITFileTerminalEvent_INTERFACE_DEFINED__

/* interface ITFileTerminalEvent */
/* [object][oleautomation][helpstring][uuid] */ 


EXTERN_C const IID IID_ITFileTerminalEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("E4A7FBAC-8C17-4427-9F55-9F589AC8AF00")
    ITFileTerminalEvent : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Terminal( 
            /* [retval][out] */ __RPC__deref_out_opt ITTerminal **ppTerminal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Track( 
            /* [retval][out] */ __RPC__deref_out_opt ITFileTrack **ppTrackTerminal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Call( 
            /* [retval][out] */ __RPC__deref_out_opt ITCallInfo **ppCall) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_State( 
            /* [retval][out] */ __RPC__out TERMINAL_MEDIA_STATE *pState) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Cause( 
            /* [retval][out] */ __RPC__out FT_STATE_EVENT_CAUSE *pCause) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Error( 
            /* [retval][out] */ __RPC__out HRESULT *phrErrorCode) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITFileTerminalEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITFileTerminalEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITFileTerminalEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITFileTerminalEvent * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITFileTerminalEvent * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITFileTerminalEvent * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITFileTerminalEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITFileTerminalEvent * This,
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
        
        DECLSPEC_XFGVIRT(ITFileTerminalEvent, get_Terminal)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Terminal )( 
            __RPC__in ITFileTerminalEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt ITTerminal **ppTerminal);
        
        DECLSPEC_XFGVIRT(ITFileTerminalEvent, get_Track)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Track )( 
            __RPC__in ITFileTerminalEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt ITFileTrack **ppTrackTerminal);
        
        DECLSPEC_XFGVIRT(ITFileTerminalEvent, get_Call)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Call )( 
            __RPC__in ITFileTerminalEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt ITCallInfo **ppCall);
        
        DECLSPEC_XFGVIRT(ITFileTerminalEvent, get_State)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_State )( 
            __RPC__in ITFileTerminalEvent * This,
            /* [retval][out] */ __RPC__out TERMINAL_MEDIA_STATE *pState);
        
        DECLSPEC_XFGVIRT(ITFileTerminalEvent, get_Cause)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Cause )( 
            __RPC__in ITFileTerminalEvent * This,
            /* [retval][out] */ __RPC__out FT_STATE_EVENT_CAUSE *pCause);
        
        DECLSPEC_XFGVIRT(ITFileTerminalEvent, get_Error)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Error )( 
            __RPC__in ITFileTerminalEvent * This,
            /* [retval][out] */ __RPC__out HRESULT *phrErrorCode);
        
        END_INTERFACE
    } ITFileTerminalEventVtbl;

    interface ITFileTerminalEvent
    {
        CONST_VTBL struct ITFileTerminalEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITFileTerminalEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITFileTerminalEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITFileTerminalEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITFileTerminalEvent_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITFileTerminalEvent_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITFileTerminalEvent_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITFileTerminalEvent_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITFileTerminalEvent_get_Terminal(This,ppTerminal)	\
    ( (This)->lpVtbl -> get_Terminal(This,ppTerminal) ) 

#define ITFileTerminalEvent_get_Track(This,ppTrackTerminal)	\
    ( (This)->lpVtbl -> get_Track(This,ppTrackTerminal) ) 

#define ITFileTerminalEvent_get_Call(This,ppCall)	\
    ( (This)->lpVtbl -> get_Call(This,ppCall) ) 

#define ITFileTerminalEvent_get_State(This,pState)	\
    ( (This)->lpVtbl -> get_State(This,pState) ) 

#define ITFileTerminalEvent_get_Cause(This,pCause)	\
    ( (This)->lpVtbl -> get_Cause(This,pCause) ) 

#define ITFileTerminalEvent_get_Error(This,phrErrorCode)	\
    ( (This)->lpVtbl -> get_Error(This,phrErrorCode) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITFileTerminalEvent_INTERFACE_DEFINED__ */


#ifndef __ITTTSTerminalEvent_INTERFACE_DEFINED__
#define __ITTTSTerminalEvent_INTERFACE_DEFINED__

/* interface ITTTSTerminalEvent */
/* [object][oleautomation][helpstring][uuid] */ 


EXTERN_C const IID IID_ITTTSTerminalEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D964788F-95A5-461d-AB0C-B9900A6C2713")
    ITTTSTerminalEvent : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Terminal( 
            /* [retval][out] */ __RPC__deref_out_opt ITTerminal **ppTerminal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Call( 
            /* [retval][out] */ __RPC__deref_out_opt ITCallInfo **ppCall) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Error( 
            /* [retval][out] */ __RPC__out HRESULT *phrErrorCode) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITTTSTerminalEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITTTSTerminalEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITTTSTerminalEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITTTSTerminalEvent * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITTTSTerminalEvent * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITTTSTerminalEvent * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITTTSTerminalEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITTTSTerminalEvent * This,
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
        
        DECLSPEC_XFGVIRT(ITTTSTerminalEvent, get_Terminal)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Terminal )( 
            __RPC__in ITTTSTerminalEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt ITTerminal **ppTerminal);
        
        DECLSPEC_XFGVIRT(ITTTSTerminalEvent, get_Call)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Call )( 
            __RPC__in ITTTSTerminalEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt ITCallInfo **ppCall);
        
        DECLSPEC_XFGVIRT(ITTTSTerminalEvent, get_Error)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Error )( 
            __RPC__in ITTTSTerminalEvent * This,
            /* [retval][out] */ __RPC__out HRESULT *phrErrorCode);
        
        END_INTERFACE
    } ITTTSTerminalEventVtbl;

    interface ITTTSTerminalEvent
    {
        CONST_VTBL struct ITTTSTerminalEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITTTSTerminalEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITTTSTerminalEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITTTSTerminalEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITTTSTerminalEvent_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITTTSTerminalEvent_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITTTSTerminalEvent_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITTTSTerminalEvent_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITTTSTerminalEvent_get_Terminal(This,ppTerminal)	\
    ( (This)->lpVtbl -> get_Terminal(This,ppTerminal) ) 

#define ITTTSTerminalEvent_get_Call(This,ppCall)	\
    ( (This)->lpVtbl -> get_Call(This,ppCall) ) 

#define ITTTSTerminalEvent_get_Error(This,phrErrorCode)	\
    ( (This)->lpVtbl -> get_Error(This,phrErrorCode) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITTTSTerminalEvent_INTERFACE_DEFINED__ */


#ifndef __ITASRTerminalEvent_INTERFACE_DEFINED__
#define __ITASRTerminalEvent_INTERFACE_DEFINED__

/* interface ITASRTerminalEvent */
/* [object][oleautomation][helpstring][uuid] */ 


EXTERN_C const IID IID_ITASRTerminalEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("EE016A02-4FA9-467c-933F-5A15B12377D7")
    ITASRTerminalEvent : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Terminal( 
            /* [retval][out] */ __RPC__deref_out_opt ITTerminal **ppTerminal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Call( 
            /* [retval][out] */ __RPC__deref_out_opt ITCallInfo **ppCall) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Error( 
            /* [retval][out] */ __RPC__out HRESULT *phrErrorCode) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITASRTerminalEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITASRTerminalEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITASRTerminalEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITASRTerminalEvent * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITASRTerminalEvent * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITASRTerminalEvent * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITASRTerminalEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITASRTerminalEvent * This,
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
        
        DECLSPEC_XFGVIRT(ITASRTerminalEvent, get_Terminal)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Terminal )( 
            __RPC__in ITASRTerminalEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt ITTerminal **ppTerminal);
        
        DECLSPEC_XFGVIRT(ITASRTerminalEvent, get_Call)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Call )( 
            __RPC__in ITASRTerminalEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt ITCallInfo **ppCall);
        
        DECLSPEC_XFGVIRT(ITASRTerminalEvent, get_Error)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Error )( 
            __RPC__in ITASRTerminalEvent * This,
            /* [retval][out] */ __RPC__out HRESULT *phrErrorCode);
        
        END_INTERFACE
    } ITASRTerminalEventVtbl;

    interface ITASRTerminalEvent
    {
        CONST_VTBL struct ITASRTerminalEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITASRTerminalEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITASRTerminalEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITASRTerminalEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITASRTerminalEvent_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITASRTerminalEvent_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITASRTerminalEvent_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITASRTerminalEvent_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITASRTerminalEvent_get_Terminal(This,ppTerminal)	\
    ( (This)->lpVtbl -> get_Terminal(This,ppTerminal) ) 

#define ITASRTerminalEvent_get_Call(This,ppCall)	\
    ( (This)->lpVtbl -> get_Call(This,ppCall) ) 

#define ITASRTerminalEvent_get_Error(This,phrErrorCode)	\
    ( (This)->lpVtbl -> get_Error(This,phrErrorCode) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITASRTerminalEvent_INTERFACE_DEFINED__ */


#ifndef __ITToneTerminalEvent_INTERFACE_DEFINED__
#define __ITToneTerminalEvent_INTERFACE_DEFINED__

/* interface ITToneTerminalEvent */
/* [object][oleautomation][helpstring][uuid] */ 


EXTERN_C const IID IID_ITToneTerminalEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("E6F56009-611F-4945-BBD2-2D0CE5612056")
    ITToneTerminalEvent : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Terminal( 
            /* [retval][out] */ __RPC__deref_out_opt ITTerminal **ppTerminal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Call( 
            /* [retval][out] */ __RPC__deref_out_opt ITCallInfo **ppCall) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Error( 
            /* [retval][out] */ __RPC__out HRESULT *phrErrorCode) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITToneTerminalEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITToneTerminalEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITToneTerminalEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITToneTerminalEvent * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITToneTerminalEvent * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITToneTerminalEvent * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITToneTerminalEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITToneTerminalEvent * This,
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
        
        DECLSPEC_XFGVIRT(ITToneTerminalEvent, get_Terminal)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Terminal )( 
            __RPC__in ITToneTerminalEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt ITTerminal **ppTerminal);
        
        DECLSPEC_XFGVIRT(ITToneTerminalEvent, get_Call)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Call )( 
            __RPC__in ITToneTerminalEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt ITCallInfo **ppCall);
        
        DECLSPEC_XFGVIRT(ITToneTerminalEvent, get_Error)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Error )( 
            __RPC__in ITToneTerminalEvent * This,
            /* [retval][out] */ __RPC__out HRESULT *phrErrorCode);
        
        END_INTERFACE
    } ITToneTerminalEventVtbl;

    interface ITToneTerminalEvent
    {
        CONST_VTBL struct ITToneTerminalEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITToneTerminalEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITToneTerminalEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITToneTerminalEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITToneTerminalEvent_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITToneTerminalEvent_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITToneTerminalEvent_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITToneTerminalEvent_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITToneTerminalEvent_get_Terminal(This,ppTerminal)	\
    ( (This)->lpVtbl -> get_Terminal(This,ppTerminal) ) 

#define ITToneTerminalEvent_get_Call(This,ppCall)	\
    ( (This)->lpVtbl -> get_Call(This,ppCall) ) 

#define ITToneTerminalEvent_get_Error(This,phrErrorCode)	\
    ( (This)->lpVtbl -> get_Error(This,phrErrorCode) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITToneTerminalEvent_INTERFACE_DEFINED__ */


#ifndef __ITQOSEvent_INTERFACE_DEFINED__
#define __ITQOSEvent_INTERFACE_DEFINED__

/* interface ITQOSEvent */
/* [object][oleautomation][helpstring][uuid] */ 


EXTERN_C const IID IID_ITQOSEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("CFA3357C-AD77-11d1-BB68-00C04FB6809F")
    ITQOSEvent : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Call( 
            /* [retval][out] */ __RPC__deref_out_opt ITCallInfo **ppCall) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Event( 
            /* [retval][out] */ __RPC__out QOS_EVENT *pQosEvent) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MediaType( 
            /* [retval][out] */ __RPC__out long *plMediaType) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITQOSEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITQOSEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITQOSEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITQOSEvent * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITQOSEvent * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITQOSEvent * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITQOSEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITQOSEvent * This,
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
        
        DECLSPEC_XFGVIRT(ITQOSEvent, get_Call)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Call )( 
            __RPC__in ITQOSEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt ITCallInfo **ppCall);
        
        DECLSPEC_XFGVIRT(ITQOSEvent, get_Event)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Event )( 
            __RPC__in ITQOSEvent * This,
            /* [retval][out] */ __RPC__out QOS_EVENT *pQosEvent);
        
        DECLSPEC_XFGVIRT(ITQOSEvent, get_MediaType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MediaType )( 
            __RPC__in ITQOSEvent * This,
            /* [retval][out] */ __RPC__out long *plMediaType);
        
        END_INTERFACE
    } ITQOSEventVtbl;

    interface ITQOSEvent
    {
        CONST_VTBL struct ITQOSEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITQOSEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITQOSEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITQOSEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITQOSEvent_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITQOSEvent_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITQOSEvent_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITQOSEvent_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITQOSEvent_get_Call(This,ppCall)	\
    ( (This)->lpVtbl -> get_Call(This,ppCall) ) 

#define ITQOSEvent_get_Event(This,pQosEvent)	\
    ( (This)->lpVtbl -> get_Event(This,pQosEvent) ) 

#define ITQOSEvent_get_MediaType(This,plMediaType)	\
    ( (This)->lpVtbl -> get_MediaType(This,plMediaType) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITQOSEvent_INTERFACE_DEFINED__ */


#ifndef __ITCallInfoChangeEvent_INTERFACE_DEFINED__
#define __ITCallInfoChangeEvent_INTERFACE_DEFINED__

/* interface ITCallInfoChangeEvent */
/* [object][oleautomation][helpstring][uuid] */ 


EXTERN_C const IID IID_ITCallInfoChangeEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5D4B65F9-E51C-11d1-A02F-00C04FB6809F")
    ITCallInfoChangeEvent : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Call( 
            /* [retval][out] */ __RPC__deref_out_opt ITCallInfo **ppCall) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Cause( 
            /* [retval][out] */ __RPC__out CALLINFOCHANGE_CAUSE *pCIC) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CallbackInstance( 
            /* [retval][out] */ __RPC__out long *plCallbackInstance) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITCallInfoChangeEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITCallInfoChangeEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITCallInfoChangeEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITCallInfoChangeEvent * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITCallInfoChangeEvent * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITCallInfoChangeEvent * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITCallInfoChangeEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITCallInfoChangeEvent * This,
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
        
        DECLSPEC_XFGVIRT(ITCallInfoChangeEvent, get_Call)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Call )( 
            __RPC__in ITCallInfoChangeEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt ITCallInfo **ppCall);
        
        DECLSPEC_XFGVIRT(ITCallInfoChangeEvent, get_Cause)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Cause )( 
            __RPC__in ITCallInfoChangeEvent * This,
            /* [retval][out] */ __RPC__out CALLINFOCHANGE_CAUSE *pCIC);
        
        DECLSPEC_XFGVIRT(ITCallInfoChangeEvent, get_CallbackInstance)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CallbackInstance )( 
            __RPC__in ITCallInfoChangeEvent * This,
            /* [retval][out] */ __RPC__out long *plCallbackInstance);
        
        END_INTERFACE
    } ITCallInfoChangeEventVtbl;

    interface ITCallInfoChangeEvent
    {
        CONST_VTBL struct ITCallInfoChangeEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITCallInfoChangeEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITCallInfoChangeEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITCallInfoChangeEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITCallInfoChangeEvent_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITCallInfoChangeEvent_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITCallInfoChangeEvent_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITCallInfoChangeEvent_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITCallInfoChangeEvent_get_Call(This,ppCall)	\
    ( (This)->lpVtbl -> get_Call(This,ppCall) ) 

#define ITCallInfoChangeEvent_get_Cause(This,pCIC)	\
    ( (This)->lpVtbl -> get_Cause(This,pCIC) ) 

#define ITCallInfoChangeEvent_get_CallbackInstance(This,plCallbackInstance)	\
    ( (This)->lpVtbl -> get_CallbackInstance(This,plCallbackInstance) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITCallInfoChangeEvent_INTERFACE_DEFINED__ */


#ifndef __ITRequest_INTERFACE_DEFINED__
#define __ITRequest_INTERFACE_DEFINED__

/* interface ITRequest */
/* [object][oleautomation][helpstring][uuid] */ 


EXTERN_C const IID IID_ITRequest;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("AC48FFDF-F8C4-11d1-A030-00C04FB6809F")
    ITRequest : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE MakeCall( 
            /* [in] */ __RPC__in BSTR pDestAddress,
            /* [in] */ __RPC__in BSTR pAppName,
            /* [in] */ __RPC__in BSTR pCalledParty,
            /* [in] */ __RPC__in BSTR pComment) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITRequestVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITRequest * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITRequest * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITRequest * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITRequest * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITRequest * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITRequest * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITRequest * This,
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
        
        DECLSPEC_XFGVIRT(ITRequest, MakeCall)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *MakeCall )( 
            __RPC__in ITRequest * This,
            /* [in] */ __RPC__in BSTR pDestAddress,
            /* [in] */ __RPC__in BSTR pAppName,
            /* [in] */ __RPC__in BSTR pCalledParty,
            /* [in] */ __RPC__in BSTR pComment);
        
        END_INTERFACE
    } ITRequestVtbl;

    interface ITRequest
    {
        CONST_VTBL struct ITRequestVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITRequest_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITRequest_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITRequest_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITRequest_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITRequest_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITRequest_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITRequest_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITRequest_MakeCall(This,pDestAddress,pAppName,pCalledParty,pComment)	\
    ( (This)->lpVtbl -> MakeCall(This,pDestAddress,pAppName,pCalledParty,pComment) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITRequest_INTERFACE_DEFINED__ */


#ifndef __ITRequestEvent_INTERFACE_DEFINED__
#define __ITRequestEvent_INTERFACE_DEFINED__

/* interface ITRequestEvent */
/* [object][oleautomation][helpstring][uuid] */ 


EXTERN_C const IID IID_ITRequestEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("AC48FFDE-F8C4-11d1-A030-00C04FB6809F")
    ITRequestEvent : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RegistrationInstance( 
            /* [retval][out] */ __RPC__out long *plRegistrationInstance) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RequestMode( 
            /* [retval][out] */ __RPC__out long *plRequestMode) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DestAddress( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppDestAddress) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_AppName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppAppName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CalledParty( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppCalledParty) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Comment( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppComment) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITRequestEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITRequestEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITRequestEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITRequestEvent * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITRequestEvent * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITRequestEvent * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITRequestEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITRequestEvent * This,
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
        
        DECLSPEC_XFGVIRT(ITRequestEvent, get_RegistrationInstance)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RegistrationInstance )( 
            __RPC__in ITRequestEvent * This,
            /* [retval][out] */ __RPC__out long *plRegistrationInstance);
        
        DECLSPEC_XFGVIRT(ITRequestEvent, get_RequestMode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RequestMode )( 
            __RPC__in ITRequestEvent * This,
            /* [retval][out] */ __RPC__out long *plRequestMode);
        
        DECLSPEC_XFGVIRT(ITRequestEvent, get_DestAddress)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DestAddress )( 
            __RPC__in ITRequestEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppDestAddress);
        
        DECLSPEC_XFGVIRT(ITRequestEvent, get_AppName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AppName )( 
            __RPC__in ITRequestEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppAppName);
        
        DECLSPEC_XFGVIRT(ITRequestEvent, get_CalledParty)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CalledParty )( 
            __RPC__in ITRequestEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppCalledParty);
        
        DECLSPEC_XFGVIRT(ITRequestEvent, get_Comment)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Comment )( 
            __RPC__in ITRequestEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppComment);
        
        END_INTERFACE
    } ITRequestEventVtbl;

    interface ITRequestEvent
    {
        CONST_VTBL struct ITRequestEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITRequestEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITRequestEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITRequestEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITRequestEvent_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITRequestEvent_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITRequestEvent_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITRequestEvent_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITRequestEvent_get_RegistrationInstance(This,plRegistrationInstance)	\
    ( (This)->lpVtbl -> get_RegistrationInstance(This,plRegistrationInstance) ) 

#define ITRequestEvent_get_RequestMode(This,plRequestMode)	\
    ( (This)->lpVtbl -> get_RequestMode(This,plRequestMode) ) 

#define ITRequestEvent_get_DestAddress(This,ppDestAddress)	\
    ( (This)->lpVtbl -> get_DestAddress(This,ppDestAddress) ) 

#define ITRequestEvent_get_AppName(This,ppAppName)	\
    ( (This)->lpVtbl -> get_AppName(This,ppAppName) ) 

#define ITRequestEvent_get_CalledParty(This,ppCalledParty)	\
    ( (This)->lpVtbl -> get_CalledParty(This,ppCalledParty) ) 

#define ITRequestEvent_get_Comment(This,ppComment)	\
    ( (This)->lpVtbl -> get_Comment(This,ppComment) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITRequestEvent_INTERFACE_DEFINED__ */


#ifndef __ITCollection_INTERFACE_DEFINED__
#define __ITCollection_INTERFACE_DEFINED__

/* interface ITCollection */
/* [dual][helpstring][uuid][public][object] */ 


EXTERN_C const IID IID_ITCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5EC5ACF2-9C02-11d0-8362-00AA003CCABD")
    ITCollection : public IDispatch
    {
    public:
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *lCount) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ long Index,
            /* [retval][out] */ __RPC__out VARIANT *pVariant) = 0;
        
        virtual /* [helpstring][id][restricted][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppNewEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITCollection * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITCollection * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITCollection * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITCollection * This,
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
        
        DECLSPEC_XFGVIRT(ITCollection, get_Count)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in ITCollection * This,
            /* [retval][out] */ __RPC__out long *lCount);
        
        DECLSPEC_XFGVIRT(ITCollection, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in ITCollection * This,
            /* [in] */ long Index,
            /* [retval][out] */ __RPC__out VARIANT *pVariant);
        
        DECLSPEC_XFGVIRT(ITCollection, get__NewEnum)
        /* [helpstring][id][restricted][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in ITCollection * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppNewEnum);
        
        END_INTERFACE
    } ITCollectionVtbl;

    interface ITCollection
    {
        CONST_VTBL struct ITCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITCollection_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITCollection_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITCollection_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITCollection_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITCollection_get_Count(This,lCount)	\
    ( (This)->lpVtbl -> get_Count(This,lCount) ) 

#define ITCollection_get_Item(This,Index,pVariant)	\
    ( (This)->lpVtbl -> get_Item(This,Index,pVariant) ) 

#define ITCollection_get__NewEnum(This,ppNewEnum)	\
    ( (This)->lpVtbl -> get__NewEnum(This,ppNewEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITCollection_INTERFACE_DEFINED__ */


#ifndef __ITCollection2_INTERFACE_DEFINED__
#define __ITCollection2_INTERFACE_DEFINED__

/* interface ITCollection2 */
/* [dual][helpstring][uuid][public][object] */ 


EXTERN_C const IID IID_ITCollection2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("E6DDDDA5-A6D3-48ff-8737-D32FC4D95477")
    ITCollection2 : public ITCollection
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Add( 
            /* [in] */ long Index,
            /* [in] */ __RPC__in VARIANT *pVariant) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Remove( 
            /* [in] */ long Index) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITCollection2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITCollection2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITCollection2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITCollection2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITCollection2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITCollection2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITCollection2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITCollection2 * This,
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
        
        DECLSPEC_XFGVIRT(ITCollection, get_Count)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in ITCollection2 * This,
            /* [retval][out] */ __RPC__out long *lCount);
        
        DECLSPEC_XFGVIRT(ITCollection, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in ITCollection2 * This,
            /* [in] */ long Index,
            /* [retval][out] */ __RPC__out VARIANT *pVariant);
        
        DECLSPEC_XFGVIRT(ITCollection, get__NewEnum)
        /* [helpstring][id][restricted][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in ITCollection2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppNewEnum);
        
        DECLSPEC_XFGVIRT(ITCollection2, Add)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Add )( 
            __RPC__in ITCollection2 * This,
            /* [in] */ long Index,
            /* [in] */ __RPC__in VARIANT *pVariant);
        
        DECLSPEC_XFGVIRT(ITCollection2, Remove)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Remove )( 
            __RPC__in ITCollection2 * This,
            /* [in] */ long Index);
        
        END_INTERFACE
    } ITCollection2Vtbl;

    interface ITCollection2
    {
        CONST_VTBL struct ITCollection2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITCollection2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITCollection2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITCollection2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITCollection2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITCollection2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITCollection2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITCollection2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITCollection2_get_Count(This,lCount)	\
    ( (This)->lpVtbl -> get_Count(This,lCount) ) 

#define ITCollection2_get_Item(This,Index,pVariant)	\
    ( (This)->lpVtbl -> get_Item(This,Index,pVariant) ) 

#define ITCollection2_get__NewEnum(This,ppNewEnum)	\
    ( (This)->lpVtbl -> get__NewEnum(This,ppNewEnum) ) 


#define ITCollection2_Add(This,Index,pVariant)	\
    ( (This)->lpVtbl -> Add(This,Index,pVariant) ) 

#define ITCollection2_Remove(This,Index)	\
    ( (This)->lpVtbl -> Remove(This,Index) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITCollection2_INTERFACE_DEFINED__ */


#ifndef __ITForwardInformation_INTERFACE_DEFINED__
#define __ITForwardInformation_INTERFACE_DEFINED__

/* interface ITForwardInformation */
/* [object][dual][helpstring][uuid] */ 


EXTERN_C const IID IID_ITForwardInformation;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("449F659E-88A3-11d1-BB5D-00C04FB6809F")
    ITForwardInformation : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_NumRingsNoAnswer( 
            /* [in] */ long lNumRings) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_NumRingsNoAnswer( 
            /* [retval][out] */ __RPC__out long *plNumRings) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetForwardType( 
            /* [in] */ long ForwardType,
            /* [in] */ __RPC__in BSTR pDestAddress,
            /* [in] */ __RPC__in BSTR pCallerAddress) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ForwardTypeDestination( 
            /* [in] */ long ForwardType,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppDestAddress) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ForwardTypeCaller( 
            /* [in] */ long Forwardtype,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppCallerAddress) = 0;
        
        virtual /* [hidden][id] */ HRESULT STDMETHODCALLTYPE GetForwardType( 
            /* [in] */ long ForwardType,
            /* [out] */ __RPC__deref_out_opt BSTR *ppDestinationAddress,
            /* [out] */ __RPC__deref_out_opt BSTR *ppCallerAddress) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Clear( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITForwardInformationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITForwardInformation * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITForwardInformation * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITForwardInformation * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITForwardInformation * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITForwardInformation * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITForwardInformation * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITForwardInformation * This,
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
        
        DECLSPEC_XFGVIRT(ITForwardInformation, put_NumRingsNoAnswer)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_NumRingsNoAnswer )( 
            __RPC__in ITForwardInformation * This,
            /* [in] */ long lNumRings);
        
        DECLSPEC_XFGVIRT(ITForwardInformation, get_NumRingsNoAnswer)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NumRingsNoAnswer )( 
            __RPC__in ITForwardInformation * This,
            /* [retval][out] */ __RPC__out long *plNumRings);
        
        DECLSPEC_XFGVIRT(ITForwardInformation, SetForwardType)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetForwardType )( 
            __RPC__in ITForwardInformation * This,
            /* [in] */ long ForwardType,
            /* [in] */ __RPC__in BSTR pDestAddress,
            /* [in] */ __RPC__in BSTR pCallerAddress);
        
        DECLSPEC_XFGVIRT(ITForwardInformation, get_ForwardTypeDestination)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ForwardTypeDestination )( 
            __RPC__in ITForwardInformation * This,
            /* [in] */ long ForwardType,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppDestAddress);
        
        DECLSPEC_XFGVIRT(ITForwardInformation, get_ForwardTypeCaller)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ForwardTypeCaller )( 
            __RPC__in ITForwardInformation * This,
            /* [in] */ long Forwardtype,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppCallerAddress);
        
        DECLSPEC_XFGVIRT(ITForwardInformation, GetForwardType)
        /* [hidden][id] */ HRESULT ( STDMETHODCALLTYPE *GetForwardType )( 
            __RPC__in ITForwardInformation * This,
            /* [in] */ long ForwardType,
            /* [out] */ __RPC__deref_out_opt BSTR *ppDestinationAddress,
            /* [out] */ __RPC__deref_out_opt BSTR *ppCallerAddress);
        
        DECLSPEC_XFGVIRT(ITForwardInformation, Clear)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Clear )( 
            __RPC__in ITForwardInformation * This);
        
        END_INTERFACE
    } ITForwardInformationVtbl;

    interface ITForwardInformation
    {
        CONST_VTBL struct ITForwardInformationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITForwardInformation_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITForwardInformation_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITForwardInformation_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITForwardInformation_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITForwardInformation_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITForwardInformation_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITForwardInformation_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITForwardInformation_put_NumRingsNoAnswer(This,lNumRings)	\
    ( (This)->lpVtbl -> put_NumRingsNoAnswer(This,lNumRings) ) 

#define ITForwardInformation_get_NumRingsNoAnswer(This,plNumRings)	\
    ( (This)->lpVtbl -> get_NumRingsNoAnswer(This,plNumRings) ) 

#define ITForwardInformation_SetForwardType(This,ForwardType,pDestAddress,pCallerAddress)	\
    ( (This)->lpVtbl -> SetForwardType(This,ForwardType,pDestAddress,pCallerAddress) ) 

#define ITForwardInformation_get_ForwardTypeDestination(This,ForwardType,ppDestAddress)	\
    ( (This)->lpVtbl -> get_ForwardTypeDestination(This,ForwardType,ppDestAddress) ) 

#define ITForwardInformation_get_ForwardTypeCaller(This,Forwardtype,ppCallerAddress)	\
    ( (This)->lpVtbl -> get_ForwardTypeCaller(This,Forwardtype,ppCallerAddress) ) 

#define ITForwardInformation_GetForwardType(This,ForwardType,ppDestinationAddress,ppCallerAddress)	\
    ( (This)->lpVtbl -> GetForwardType(This,ForwardType,ppDestinationAddress,ppCallerAddress) ) 

#define ITForwardInformation_Clear(This)	\
    ( (This)->lpVtbl -> Clear(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITForwardInformation_INTERFACE_DEFINED__ */


#ifndef __ITForwardInformation2_INTERFACE_DEFINED__
#define __ITForwardInformation2_INTERFACE_DEFINED__

/* interface ITForwardInformation2 */
/* [object][dual][helpstring][uuid] */ 


EXTERN_C const IID IID_ITForwardInformation2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5229B4ED-B260-4382-8E1A-5DF3A8A4CCC0")
    ITForwardInformation2 : public ITForwardInformation
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetForwardType2( 
            /* [in] */ long ForwardType,
            /* [in] */ __RPC__in BSTR pDestAddress,
            /* [in] */ long DestAddressType,
            /* [in] */ __RPC__in BSTR pCallerAddress,
            /* [in] */ long CallerAddressType) = 0;
        
        virtual /* [hidden][id] */ HRESULT STDMETHODCALLTYPE GetForwardType2( 
            /* [in] */ long ForwardType,
            /* [out] */ __RPC__deref_out_opt BSTR *ppDestinationAddress,
            /* [out] */ __RPC__out long *pDestAddressType,
            /* [out] */ __RPC__deref_out_opt BSTR *ppCallerAddress,
            /* [out] */ __RPC__out long *pCallerAddressType) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ForwardTypeDestinationAddressType( 
            /* [in] */ long ForwardType,
            /* [retval][out] */ __RPC__out long *pDestAddressType) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ForwardTypeCallerAddressType( 
            /* [in] */ long Forwardtype,
            /* [retval][out] */ __RPC__out long *pCallerAddressType) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITForwardInformation2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITForwardInformation2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITForwardInformation2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITForwardInformation2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITForwardInformation2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITForwardInformation2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITForwardInformation2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITForwardInformation2 * This,
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
        
        DECLSPEC_XFGVIRT(ITForwardInformation, put_NumRingsNoAnswer)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_NumRingsNoAnswer )( 
            __RPC__in ITForwardInformation2 * This,
            /* [in] */ long lNumRings);
        
        DECLSPEC_XFGVIRT(ITForwardInformation, get_NumRingsNoAnswer)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NumRingsNoAnswer )( 
            __RPC__in ITForwardInformation2 * This,
            /* [retval][out] */ __RPC__out long *plNumRings);
        
        DECLSPEC_XFGVIRT(ITForwardInformation, SetForwardType)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetForwardType )( 
            __RPC__in ITForwardInformation2 * This,
            /* [in] */ long ForwardType,
            /* [in] */ __RPC__in BSTR pDestAddress,
            /* [in] */ __RPC__in BSTR pCallerAddress);
        
        DECLSPEC_XFGVIRT(ITForwardInformation, get_ForwardTypeDestination)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ForwardTypeDestination )( 
            __RPC__in ITForwardInformation2 * This,
            /* [in] */ long ForwardType,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppDestAddress);
        
        DECLSPEC_XFGVIRT(ITForwardInformation, get_ForwardTypeCaller)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ForwardTypeCaller )( 
            __RPC__in ITForwardInformation2 * This,
            /* [in] */ long Forwardtype,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppCallerAddress);
        
        DECLSPEC_XFGVIRT(ITForwardInformation, GetForwardType)
        /* [hidden][id] */ HRESULT ( STDMETHODCALLTYPE *GetForwardType )( 
            __RPC__in ITForwardInformation2 * This,
            /* [in] */ long ForwardType,
            /* [out] */ __RPC__deref_out_opt BSTR *ppDestinationAddress,
            /* [out] */ __RPC__deref_out_opt BSTR *ppCallerAddress);
        
        DECLSPEC_XFGVIRT(ITForwardInformation, Clear)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Clear )( 
            __RPC__in ITForwardInformation2 * This);
        
        DECLSPEC_XFGVIRT(ITForwardInformation2, SetForwardType2)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetForwardType2 )( 
            __RPC__in ITForwardInformation2 * This,
            /* [in] */ long ForwardType,
            /* [in] */ __RPC__in BSTR pDestAddress,
            /* [in] */ long DestAddressType,
            /* [in] */ __RPC__in BSTR pCallerAddress,
            /* [in] */ long CallerAddressType);
        
        DECLSPEC_XFGVIRT(ITForwardInformation2, GetForwardType2)
        /* [hidden][id] */ HRESULT ( STDMETHODCALLTYPE *GetForwardType2 )( 
            __RPC__in ITForwardInformation2 * This,
            /* [in] */ long ForwardType,
            /* [out] */ __RPC__deref_out_opt BSTR *ppDestinationAddress,
            /* [out] */ __RPC__out long *pDestAddressType,
            /* [out] */ __RPC__deref_out_opt BSTR *ppCallerAddress,
            /* [out] */ __RPC__out long *pCallerAddressType);
        
        DECLSPEC_XFGVIRT(ITForwardInformation2, get_ForwardTypeDestinationAddressType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ForwardTypeDestinationAddressType )( 
            __RPC__in ITForwardInformation2 * This,
            /* [in] */ long ForwardType,
            /* [retval][out] */ __RPC__out long *pDestAddressType);
        
        DECLSPEC_XFGVIRT(ITForwardInformation2, get_ForwardTypeCallerAddressType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ForwardTypeCallerAddressType )( 
            __RPC__in ITForwardInformation2 * This,
            /* [in] */ long Forwardtype,
            /* [retval][out] */ __RPC__out long *pCallerAddressType);
        
        END_INTERFACE
    } ITForwardInformation2Vtbl;

    interface ITForwardInformation2
    {
        CONST_VTBL struct ITForwardInformation2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITForwardInformation2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITForwardInformation2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITForwardInformation2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITForwardInformation2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITForwardInformation2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITForwardInformation2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITForwardInformation2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITForwardInformation2_put_NumRingsNoAnswer(This,lNumRings)	\
    ( (This)->lpVtbl -> put_NumRingsNoAnswer(This,lNumRings) ) 

#define ITForwardInformation2_get_NumRingsNoAnswer(This,plNumRings)	\
    ( (This)->lpVtbl -> get_NumRingsNoAnswer(This,plNumRings) ) 

#define ITForwardInformation2_SetForwardType(This,ForwardType,pDestAddress,pCallerAddress)	\
    ( (This)->lpVtbl -> SetForwardType(This,ForwardType,pDestAddress,pCallerAddress) ) 

#define ITForwardInformation2_get_ForwardTypeDestination(This,ForwardType,ppDestAddress)	\
    ( (This)->lpVtbl -> get_ForwardTypeDestination(This,ForwardType,ppDestAddress) ) 

#define ITForwardInformation2_get_ForwardTypeCaller(This,Forwardtype,ppCallerAddress)	\
    ( (This)->lpVtbl -> get_ForwardTypeCaller(This,Forwardtype,ppCallerAddress) ) 

#define ITForwardInformation2_GetForwardType(This,ForwardType,ppDestinationAddress,ppCallerAddress)	\
    ( (This)->lpVtbl -> GetForwardType(This,ForwardType,ppDestinationAddress,ppCallerAddress) ) 

#define ITForwardInformation2_Clear(This)	\
    ( (This)->lpVtbl -> Clear(This) ) 


#define ITForwardInformation2_SetForwardType2(This,ForwardType,pDestAddress,DestAddressType,pCallerAddress,CallerAddressType)	\
    ( (This)->lpVtbl -> SetForwardType2(This,ForwardType,pDestAddress,DestAddressType,pCallerAddress,CallerAddressType) ) 

#define ITForwardInformation2_GetForwardType2(This,ForwardType,ppDestinationAddress,pDestAddressType,ppCallerAddress,pCallerAddressType)	\
    ( (This)->lpVtbl -> GetForwardType2(This,ForwardType,ppDestinationAddress,pDestAddressType,ppCallerAddress,pCallerAddressType) ) 

#define ITForwardInformation2_get_ForwardTypeDestinationAddressType(This,ForwardType,pDestAddressType)	\
    ( (This)->lpVtbl -> get_ForwardTypeDestinationAddressType(This,ForwardType,pDestAddressType) ) 

#define ITForwardInformation2_get_ForwardTypeCallerAddressType(This,Forwardtype,pCallerAddressType)	\
    ( (This)->lpVtbl -> get_ForwardTypeCallerAddressType(This,Forwardtype,pCallerAddressType) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITForwardInformation2_INTERFACE_DEFINED__ */


#ifndef __ITAddressTranslation_INTERFACE_DEFINED__
#define __ITAddressTranslation_INTERFACE_DEFINED__

/* interface ITAddressTranslation */
/* [object][dual][helpstring][uuid] */ 


EXTERN_C const IID IID_ITAddressTranslation;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0C4D8F03-8DDB-11d1-A09E-00805FC147D3")
    ITAddressTranslation : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE TranslateAddress( 
            /* [in] */ __RPC__in BSTR pAddressToTranslate,
            /* [in] */ long lCard,
            /* [in] */ long lTranslateOptions,
            /* [retval][out] */ __RPC__deref_out_opt ITAddressTranslationInfo **ppTranslated) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE TranslateDialog( 
            /* [in] */ TAPIHWND hwndOwner,
            /* [in] */ __RPC__in BSTR pAddressIn) = 0;
        
        virtual /* [helpstring][hidden][id] */ HRESULT STDMETHODCALLTYPE EnumerateLocations( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumLocation **ppEnumLocation) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Locations( 
            /* [retval][out] */ __RPC__out VARIANT *pVariant) = 0;
        
        virtual /* [helpstring][hidden][id] */ HRESULT STDMETHODCALLTYPE EnumerateCallingCards( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumCallingCard **ppEnumCallingCard) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CallingCards( 
            /* [retval][out] */ __RPC__out VARIANT *pVariant) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITAddressTranslationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITAddressTranslation * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITAddressTranslation * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITAddressTranslation * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITAddressTranslation * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITAddressTranslation * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITAddressTranslation * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITAddressTranslation * This,
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
        
        DECLSPEC_XFGVIRT(ITAddressTranslation, TranslateAddress)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *TranslateAddress )( 
            __RPC__in ITAddressTranslation * This,
            /* [in] */ __RPC__in BSTR pAddressToTranslate,
            /* [in] */ long lCard,
            /* [in] */ long lTranslateOptions,
            /* [retval][out] */ __RPC__deref_out_opt ITAddressTranslationInfo **ppTranslated);
        
        DECLSPEC_XFGVIRT(ITAddressTranslation, TranslateDialog)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *TranslateDialog )( 
            __RPC__in ITAddressTranslation * This,
            /* [in] */ TAPIHWND hwndOwner,
            /* [in] */ __RPC__in BSTR pAddressIn);
        
        DECLSPEC_XFGVIRT(ITAddressTranslation, EnumerateLocations)
        /* [helpstring][hidden][id] */ HRESULT ( STDMETHODCALLTYPE *EnumerateLocations )( 
            __RPC__in ITAddressTranslation * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumLocation **ppEnumLocation);
        
        DECLSPEC_XFGVIRT(ITAddressTranslation, get_Locations)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Locations )( 
            __RPC__in ITAddressTranslation * This,
            /* [retval][out] */ __RPC__out VARIANT *pVariant);
        
        DECLSPEC_XFGVIRT(ITAddressTranslation, EnumerateCallingCards)
        /* [helpstring][hidden][id] */ HRESULT ( STDMETHODCALLTYPE *EnumerateCallingCards )( 
            __RPC__in ITAddressTranslation * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumCallingCard **ppEnumCallingCard);
        
        DECLSPEC_XFGVIRT(ITAddressTranslation, get_CallingCards)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CallingCards )( 
            __RPC__in ITAddressTranslation * This,
            /* [retval][out] */ __RPC__out VARIANT *pVariant);
        
        END_INTERFACE
    } ITAddressTranslationVtbl;

    interface ITAddressTranslation
    {
        CONST_VTBL struct ITAddressTranslationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITAddressTranslation_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITAddressTranslation_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITAddressTranslation_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITAddressTranslation_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITAddressTranslation_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITAddressTranslation_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITAddressTranslation_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITAddressTranslation_TranslateAddress(This,pAddressToTranslate,lCard,lTranslateOptions,ppTranslated)	\
    ( (This)->lpVtbl -> TranslateAddress(This,pAddressToTranslate,lCard,lTranslateOptions,ppTranslated) ) 

#define ITAddressTranslation_TranslateDialog(This,hwndOwner,pAddressIn)	\
    ( (This)->lpVtbl -> TranslateDialog(This,hwndOwner,pAddressIn) ) 

#define ITAddressTranslation_EnumerateLocations(This,ppEnumLocation)	\
    ( (This)->lpVtbl -> EnumerateLocations(This,ppEnumLocation) ) 

#define ITAddressTranslation_get_Locations(This,pVariant)	\
    ( (This)->lpVtbl -> get_Locations(This,pVariant) ) 

#define ITAddressTranslation_EnumerateCallingCards(This,ppEnumCallingCard)	\
    ( (This)->lpVtbl -> EnumerateCallingCards(This,ppEnumCallingCard) ) 

#define ITAddressTranslation_get_CallingCards(This,pVariant)	\
    ( (This)->lpVtbl -> get_CallingCards(This,pVariant) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITAddressTranslation_INTERFACE_DEFINED__ */


#ifndef __ITAddressTranslationInfo_INTERFACE_DEFINED__
#define __ITAddressTranslationInfo_INTERFACE_DEFINED__

/* interface ITAddressTranslationInfo */
/* [object][dual][helpstring][uuid] */ 


EXTERN_C const IID IID_ITAddressTranslationInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("AFC15945-8D40-11d1-A09E-00805FC147D3")
    ITAddressTranslationInfo : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DialableString( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppDialableString) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DisplayableString( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppDisplayableString) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CurrentCountryCode( 
            /* [retval][out] */ __RPC__out long *CountryCode) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DestinationCountryCode( 
            /* [retval][out] */ __RPC__out long *CountryCode) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TranslationResults( 
            /* [retval][out] */ __RPC__out long *plResults) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITAddressTranslationInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITAddressTranslationInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITAddressTranslationInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITAddressTranslationInfo * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITAddressTranslationInfo * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITAddressTranslationInfo * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITAddressTranslationInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITAddressTranslationInfo * This,
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
        
        DECLSPEC_XFGVIRT(ITAddressTranslationInfo, get_DialableString)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DialableString )( 
            __RPC__in ITAddressTranslationInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppDialableString);
        
        DECLSPEC_XFGVIRT(ITAddressTranslationInfo, get_DisplayableString)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DisplayableString )( 
            __RPC__in ITAddressTranslationInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppDisplayableString);
        
        DECLSPEC_XFGVIRT(ITAddressTranslationInfo, get_CurrentCountryCode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CurrentCountryCode )( 
            __RPC__in ITAddressTranslationInfo * This,
            /* [retval][out] */ __RPC__out long *CountryCode);
        
        DECLSPEC_XFGVIRT(ITAddressTranslationInfo, get_DestinationCountryCode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DestinationCountryCode )( 
            __RPC__in ITAddressTranslationInfo * This,
            /* [retval][out] */ __RPC__out long *CountryCode);
        
        DECLSPEC_XFGVIRT(ITAddressTranslationInfo, get_TranslationResults)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TranslationResults )( 
            __RPC__in ITAddressTranslationInfo * This,
            /* [retval][out] */ __RPC__out long *plResults);
        
        END_INTERFACE
    } ITAddressTranslationInfoVtbl;

    interface ITAddressTranslationInfo
    {
        CONST_VTBL struct ITAddressTranslationInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITAddressTranslationInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITAddressTranslationInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITAddressTranslationInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITAddressTranslationInfo_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITAddressTranslationInfo_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITAddressTranslationInfo_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITAddressTranslationInfo_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITAddressTranslationInfo_get_DialableString(This,ppDialableString)	\
    ( (This)->lpVtbl -> get_DialableString(This,ppDialableString) ) 

#define ITAddressTranslationInfo_get_DisplayableString(This,ppDisplayableString)	\
    ( (This)->lpVtbl -> get_DisplayableString(This,ppDisplayableString) ) 

#define ITAddressTranslationInfo_get_CurrentCountryCode(This,CountryCode)	\
    ( (This)->lpVtbl -> get_CurrentCountryCode(This,CountryCode) ) 

#define ITAddressTranslationInfo_get_DestinationCountryCode(This,CountryCode)	\
    ( (This)->lpVtbl -> get_DestinationCountryCode(This,CountryCode) ) 

#define ITAddressTranslationInfo_get_TranslationResults(This,plResults)	\
    ( (This)->lpVtbl -> get_TranslationResults(This,plResults) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITAddressTranslationInfo_INTERFACE_DEFINED__ */


#ifndef __ITLocationInfo_INTERFACE_DEFINED__
#define __ITLocationInfo_INTERFACE_DEFINED__

/* interface ITLocationInfo */
/* [object][dual][helpstring][uuid] */ 


EXTERN_C const IID IID_ITLocationInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0C4D8EFF-8DDB-11d1-A09E-00805FC147D3")
    ITLocationInfo : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PermanentLocationID( 
            /* [retval][out] */ __RPC__out long *plLocationID) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CountryCode( 
            /* [retval][out] */ __RPC__out long *plCountryCode) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CountryID( 
            /* [retval][out] */ __RPC__out long *plCountryID) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Options( 
            /* [retval][out] */ __RPC__out long *plOptions) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PreferredCardID( 
            /* [retval][out] */ __RPC__out long *plCardID) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_LocationName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppLocationName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CityCode( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppCode) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_LocalAccessCode( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppCode) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_LongDistanceAccessCode( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppCode) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TollPrefixList( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppTollList) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CancelCallWaitingCode( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppCode) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITLocationInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITLocationInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITLocationInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITLocationInfo * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITLocationInfo * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITLocationInfo * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITLocationInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITLocationInfo * This,
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
        
        DECLSPEC_XFGVIRT(ITLocationInfo, get_PermanentLocationID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PermanentLocationID )( 
            __RPC__in ITLocationInfo * This,
            /* [retval][out] */ __RPC__out long *plLocationID);
        
        DECLSPEC_XFGVIRT(ITLocationInfo, get_CountryCode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CountryCode )( 
            __RPC__in ITLocationInfo * This,
            /* [retval][out] */ __RPC__out long *plCountryCode);
        
        DECLSPEC_XFGVIRT(ITLocationInfo, get_CountryID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CountryID )( 
            __RPC__in ITLocationInfo * This,
            /* [retval][out] */ __RPC__out long *plCountryID);
        
        DECLSPEC_XFGVIRT(ITLocationInfo, get_Options)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Options )( 
            __RPC__in ITLocationInfo * This,
            /* [retval][out] */ __RPC__out long *plOptions);
        
        DECLSPEC_XFGVIRT(ITLocationInfo, get_PreferredCardID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PreferredCardID )( 
            __RPC__in ITLocationInfo * This,
            /* [retval][out] */ __RPC__out long *plCardID);
        
        DECLSPEC_XFGVIRT(ITLocationInfo, get_LocationName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LocationName )( 
            __RPC__in ITLocationInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppLocationName);
        
        DECLSPEC_XFGVIRT(ITLocationInfo, get_CityCode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CityCode )( 
            __RPC__in ITLocationInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppCode);
        
        DECLSPEC_XFGVIRT(ITLocationInfo, get_LocalAccessCode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LocalAccessCode )( 
            __RPC__in ITLocationInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppCode);
        
        DECLSPEC_XFGVIRT(ITLocationInfo, get_LongDistanceAccessCode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LongDistanceAccessCode )( 
            __RPC__in ITLocationInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppCode);
        
        DECLSPEC_XFGVIRT(ITLocationInfo, get_TollPrefixList)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TollPrefixList )( 
            __RPC__in ITLocationInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppTollList);
        
        DECLSPEC_XFGVIRT(ITLocationInfo, get_CancelCallWaitingCode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CancelCallWaitingCode )( 
            __RPC__in ITLocationInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppCode);
        
        END_INTERFACE
    } ITLocationInfoVtbl;

    interface ITLocationInfo
    {
        CONST_VTBL struct ITLocationInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITLocationInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITLocationInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITLocationInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITLocationInfo_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITLocationInfo_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITLocationInfo_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITLocationInfo_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITLocationInfo_get_PermanentLocationID(This,plLocationID)	\
    ( (This)->lpVtbl -> get_PermanentLocationID(This,plLocationID) ) 

#define ITLocationInfo_get_CountryCode(This,plCountryCode)	\
    ( (This)->lpVtbl -> get_CountryCode(This,plCountryCode) ) 

#define ITLocationInfo_get_CountryID(This,plCountryID)	\
    ( (This)->lpVtbl -> get_CountryID(This,plCountryID) ) 

#define ITLocationInfo_get_Options(This,plOptions)	\
    ( (This)->lpVtbl -> get_Options(This,plOptions) ) 

#define ITLocationInfo_get_PreferredCardID(This,plCardID)	\
    ( (This)->lpVtbl -> get_PreferredCardID(This,plCardID) ) 

#define ITLocationInfo_get_LocationName(This,ppLocationName)	\
    ( (This)->lpVtbl -> get_LocationName(This,ppLocationName) ) 

#define ITLocationInfo_get_CityCode(This,ppCode)	\
    ( (This)->lpVtbl -> get_CityCode(This,ppCode) ) 

#define ITLocationInfo_get_LocalAccessCode(This,ppCode)	\
    ( (This)->lpVtbl -> get_LocalAccessCode(This,ppCode) ) 

#define ITLocationInfo_get_LongDistanceAccessCode(This,ppCode)	\
    ( (This)->lpVtbl -> get_LongDistanceAccessCode(This,ppCode) ) 

#define ITLocationInfo_get_TollPrefixList(This,ppTollList)	\
    ( (This)->lpVtbl -> get_TollPrefixList(This,ppTollList) ) 

#define ITLocationInfo_get_CancelCallWaitingCode(This,ppCode)	\
    ( (This)->lpVtbl -> get_CancelCallWaitingCode(This,ppCode) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITLocationInfo_INTERFACE_DEFINED__ */


#ifndef __IEnumLocation_INTERFACE_DEFINED__
#define __IEnumLocation_INTERFACE_DEFINED__

/* interface IEnumLocation */
/* [object][unique][hidden][helpstring][uuid] */ 


EXTERN_C const IID IID_IEnumLocation;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0C4D8F01-8DDB-11d1-A09E-00805FC147D3")
    IEnumLocation : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG celt,
            /* [out] */ __RPC__deref_out_opt ITLocationInfo **ppElements,
            /* [full][out][in] */ __RPC__inout_opt ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumLocation **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumLocationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumLocation * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumLocation * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumLocation * This);
        
        DECLSPEC_XFGVIRT(IEnumLocation, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IEnumLocation * This,
            /* [in] */ ULONG celt,
            /* [out] */ __RPC__deref_out_opt ITLocationInfo **ppElements,
            /* [full][out][in] */ __RPC__inout_opt ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IEnumLocation, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumLocation * This);
        
        DECLSPEC_XFGVIRT(IEnumLocation, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumLocation * This,
            /* [in] */ ULONG celt);
        
        DECLSPEC_XFGVIRT(IEnumLocation, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumLocation * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumLocation **ppEnum);
        
        END_INTERFACE
    } IEnumLocationVtbl;

    interface IEnumLocation
    {
        CONST_VTBL struct IEnumLocationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumLocation_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumLocation_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumLocation_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumLocation_Next(This,celt,ppElements,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,ppElements,pceltFetched) ) 

#define IEnumLocation_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumLocation_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IEnumLocation_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumLocation_INTERFACE_DEFINED__ */


#ifndef __ITCallingCard_INTERFACE_DEFINED__
#define __ITCallingCard_INTERFACE_DEFINED__

/* interface ITCallingCard */
/* [object][dual][helpstring][uuid] */ 


EXTERN_C const IID IID_ITCallingCard;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0C4D8F00-8DDB-11d1-A09E-00805FC147D3")
    ITCallingCard : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PermanentCardID( 
            /* [retval][out] */ __RPC__out long *plCardID) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_NumberOfDigits( 
            /* [retval][out] */ __RPC__out long *plDigits) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Options( 
            /* [retval][out] */ __RPC__out long *plOptions) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CardName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppCardName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SameAreaDialingRule( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppRule) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_LongDistanceDialingRule( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppRule) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_InternationalDialingRule( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppRule) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITCallingCardVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITCallingCard * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITCallingCard * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITCallingCard * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITCallingCard * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITCallingCard * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITCallingCard * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITCallingCard * This,
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
        
        DECLSPEC_XFGVIRT(ITCallingCard, get_PermanentCardID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PermanentCardID )( 
            __RPC__in ITCallingCard * This,
            /* [retval][out] */ __RPC__out long *plCardID);
        
        DECLSPEC_XFGVIRT(ITCallingCard, get_NumberOfDigits)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NumberOfDigits )( 
            __RPC__in ITCallingCard * This,
            /* [retval][out] */ __RPC__out long *plDigits);
        
        DECLSPEC_XFGVIRT(ITCallingCard, get_Options)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Options )( 
            __RPC__in ITCallingCard * This,
            /* [retval][out] */ __RPC__out long *plOptions);
        
        DECLSPEC_XFGVIRT(ITCallingCard, get_CardName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CardName )( 
            __RPC__in ITCallingCard * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppCardName);
        
        DECLSPEC_XFGVIRT(ITCallingCard, get_SameAreaDialingRule)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SameAreaDialingRule )( 
            __RPC__in ITCallingCard * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppRule);
        
        DECLSPEC_XFGVIRT(ITCallingCard, get_LongDistanceDialingRule)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LongDistanceDialingRule )( 
            __RPC__in ITCallingCard * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppRule);
        
        DECLSPEC_XFGVIRT(ITCallingCard, get_InternationalDialingRule)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_InternationalDialingRule )( 
            __RPC__in ITCallingCard * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppRule);
        
        END_INTERFACE
    } ITCallingCardVtbl;

    interface ITCallingCard
    {
        CONST_VTBL struct ITCallingCardVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITCallingCard_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITCallingCard_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITCallingCard_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITCallingCard_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITCallingCard_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITCallingCard_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITCallingCard_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITCallingCard_get_PermanentCardID(This,plCardID)	\
    ( (This)->lpVtbl -> get_PermanentCardID(This,plCardID) ) 

#define ITCallingCard_get_NumberOfDigits(This,plDigits)	\
    ( (This)->lpVtbl -> get_NumberOfDigits(This,plDigits) ) 

#define ITCallingCard_get_Options(This,plOptions)	\
    ( (This)->lpVtbl -> get_Options(This,plOptions) ) 

#define ITCallingCard_get_CardName(This,ppCardName)	\
    ( (This)->lpVtbl -> get_CardName(This,ppCardName) ) 

#define ITCallingCard_get_SameAreaDialingRule(This,ppRule)	\
    ( (This)->lpVtbl -> get_SameAreaDialingRule(This,ppRule) ) 

#define ITCallingCard_get_LongDistanceDialingRule(This,ppRule)	\
    ( (This)->lpVtbl -> get_LongDistanceDialingRule(This,ppRule) ) 

#define ITCallingCard_get_InternationalDialingRule(This,ppRule)	\
    ( (This)->lpVtbl -> get_InternationalDialingRule(This,ppRule) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITCallingCard_INTERFACE_DEFINED__ */


#ifndef __IEnumCallingCard_INTERFACE_DEFINED__
#define __IEnumCallingCard_INTERFACE_DEFINED__

/* interface IEnumCallingCard */
/* [object][unique][hidden][helpstring][uuid] */ 


EXTERN_C const IID IID_IEnumCallingCard;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0C4D8F02-8DDB-11d1-A09E-00805FC147D3")
    IEnumCallingCard : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG celt,
            /* [out] */ __RPC__deref_out_opt ITCallingCard **ppElements,
            /* [full][out][in] */ __RPC__inout_opt ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumCallingCard **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumCallingCardVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumCallingCard * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumCallingCard * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumCallingCard * This);
        
        DECLSPEC_XFGVIRT(IEnumCallingCard, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IEnumCallingCard * This,
            /* [in] */ ULONG celt,
            /* [out] */ __RPC__deref_out_opt ITCallingCard **ppElements,
            /* [full][out][in] */ __RPC__inout_opt ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IEnumCallingCard, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumCallingCard * This);
        
        DECLSPEC_XFGVIRT(IEnumCallingCard, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumCallingCard * This,
            /* [in] */ ULONG celt);
        
        DECLSPEC_XFGVIRT(IEnumCallingCard, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumCallingCard * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumCallingCard **ppEnum);
        
        END_INTERFACE
    } IEnumCallingCardVtbl;

    interface IEnumCallingCard
    {
        CONST_VTBL struct IEnumCallingCardVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumCallingCard_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumCallingCard_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumCallingCard_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumCallingCard_Next(This,celt,ppElements,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,ppElements,pceltFetched) ) 

#define IEnumCallingCard_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumCallingCard_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IEnumCallingCard_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumCallingCard_INTERFACE_DEFINED__ */


#ifndef __ITCallNotificationEvent_INTERFACE_DEFINED__
#define __ITCallNotificationEvent_INTERFACE_DEFINED__

/* interface ITCallNotificationEvent */
/* [object][dual][helpstring][uuid] */ 


EXTERN_C const IID IID_ITCallNotificationEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("895801DF-3DD6-11d1-8F30-00C04FB6809F")
    ITCallNotificationEvent : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Call( 
            /* [retval][out] */ __RPC__deref_out_opt ITCallInfo **ppCall) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Event( 
            /* [retval][out] */ __RPC__out CALL_NOTIFICATION_EVENT *pCallNotificationEvent) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CallbackInstance( 
            /* [retval][out] */ __RPC__out long *plCallbackInstance) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITCallNotificationEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITCallNotificationEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITCallNotificationEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITCallNotificationEvent * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITCallNotificationEvent * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITCallNotificationEvent * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITCallNotificationEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITCallNotificationEvent * This,
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
        
        DECLSPEC_XFGVIRT(ITCallNotificationEvent, get_Call)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Call )( 
            __RPC__in ITCallNotificationEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt ITCallInfo **ppCall);
        
        DECLSPEC_XFGVIRT(ITCallNotificationEvent, get_Event)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Event )( 
            __RPC__in ITCallNotificationEvent * This,
            /* [retval][out] */ __RPC__out CALL_NOTIFICATION_EVENT *pCallNotificationEvent);
        
        DECLSPEC_XFGVIRT(ITCallNotificationEvent, get_CallbackInstance)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CallbackInstance )( 
            __RPC__in ITCallNotificationEvent * This,
            /* [retval][out] */ __RPC__out long *plCallbackInstance);
        
        END_INTERFACE
    } ITCallNotificationEventVtbl;

    interface ITCallNotificationEvent
    {
        CONST_VTBL struct ITCallNotificationEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITCallNotificationEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITCallNotificationEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITCallNotificationEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITCallNotificationEvent_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITCallNotificationEvent_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITCallNotificationEvent_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITCallNotificationEvent_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITCallNotificationEvent_get_Call(This,ppCall)	\
    ( (This)->lpVtbl -> get_Call(This,ppCall) ) 

#define ITCallNotificationEvent_get_Event(This,pCallNotificationEvent)	\
    ( (This)->lpVtbl -> get_Event(This,pCallNotificationEvent) ) 

#define ITCallNotificationEvent_get_CallbackInstance(This,plCallbackInstance)	\
    ( (This)->lpVtbl -> get_CallbackInstance(This,plCallbackInstance) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITCallNotificationEvent_INTERFACE_DEFINED__ */


#ifndef __ITDispatchMapper_INTERFACE_DEFINED__
#define __ITDispatchMapper_INTERFACE_DEFINED__

/* interface ITDispatchMapper */
/* [object][helpstring][uuid] */ 


EXTERN_C const IID IID_ITDispatchMapper;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("E9225295-C759-11d1-A02B-00C04FB6809F")
    ITDispatchMapper : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE QueryDispatchInterface( 
            /* [in] */ __RPC__in BSTR pIID,
            /* [in] */ __RPC__in_opt IDispatch *pInterfaceToMap,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppReturnedInterface) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITDispatchMapperVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITDispatchMapper * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITDispatchMapper * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITDispatchMapper * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITDispatchMapper * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITDispatchMapper * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITDispatchMapper * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITDispatchMapper * This,
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
        
        DECLSPEC_XFGVIRT(ITDispatchMapper, QueryDispatchInterface)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *QueryDispatchInterface )( 
            __RPC__in ITDispatchMapper * This,
            /* [in] */ __RPC__in BSTR pIID,
            /* [in] */ __RPC__in_opt IDispatch *pInterfaceToMap,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppReturnedInterface);
        
        END_INTERFACE
    } ITDispatchMapperVtbl;

    interface ITDispatchMapper
    {
        CONST_VTBL struct ITDispatchMapperVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITDispatchMapper_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITDispatchMapper_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITDispatchMapper_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITDispatchMapper_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITDispatchMapper_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITDispatchMapper_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITDispatchMapper_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITDispatchMapper_QueryDispatchInterface(This,pIID,pInterfaceToMap,ppReturnedInterface)	\
    ( (This)->lpVtbl -> QueryDispatchInterface(This,pIID,pInterfaceToMap,ppReturnedInterface) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITDispatchMapper_INTERFACE_DEFINED__ */


#ifndef __ITStreamControl_INTERFACE_DEFINED__
#define __ITStreamControl_INTERFACE_DEFINED__

/* interface ITStreamControl */
/* [object][dual][unique][helpstring][uuid] */ 


EXTERN_C const IID IID_ITStreamControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("EE3BD604-3868-11D2-A045-00C04FB6809F")
    ITStreamControl : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateStream( 
            /* [in] */ long lMediaType,
            /* [in] */ TERMINAL_DIRECTION td,
            /* [retval][out] */ __RPC__deref_out_opt ITStream **ppStream) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE RemoveStream( 
            /* [in] */ __RPC__in_opt ITStream *pStream) = 0;
        
        virtual /* [hidden][id] */ HRESULT STDMETHODCALLTYPE EnumerateStreams( 
            /* [out] */ __RPC__deref_out_opt IEnumStream **ppEnumStream) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Streams( 
            /* [retval][out] */ __RPC__out VARIANT *pVariant) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITStreamControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITStreamControl * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITStreamControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITStreamControl * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITStreamControl * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITStreamControl * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITStreamControl * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITStreamControl * This,
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
        
        DECLSPEC_XFGVIRT(ITStreamControl, CreateStream)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateStream )( 
            __RPC__in ITStreamControl * This,
            /* [in] */ long lMediaType,
            /* [in] */ TERMINAL_DIRECTION td,
            /* [retval][out] */ __RPC__deref_out_opt ITStream **ppStream);
        
        DECLSPEC_XFGVIRT(ITStreamControl, RemoveStream)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RemoveStream )( 
            __RPC__in ITStreamControl * This,
            /* [in] */ __RPC__in_opt ITStream *pStream);
        
        DECLSPEC_XFGVIRT(ITStreamControl, EnumerateStreams)
        /* [hidden][id] */ HRESULT ( STDMETHODCALLTYPE *EnumerateStreams )( 
            __RPC__in ITStreamControl * This,
            /* [out] */ __RPC__deref_out_opt IEnumStream **ppEnumStream);
        
        DECLSPEC_XFGVIRT(ITStreamControl, get_Streams)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Streams )( 
            __RPC__in ITStreamControl * This,
            /* [retval][out] */ __RPC__out VARIANT *pVariant);
        
        END_INTERFACE
    } ITStreamControlVtbl;

    interface ITStreamControl
    {
        CONST_VTBL struct ITStreamControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITStreamControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITStreamControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITStreamControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITStreamControl_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITStreamControl_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITStreamControl_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITStreamControl_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITStreamControl_CreateStream(This,lMediaType,td,ppStream)	\
    ( (This)->lpVtbl -> CreateStream(This,lMediaType,td,ppStream) ) 

#define ITStreamControl_RemoveStream(This,pStream)	\
    ( (This)->lpVtbl -> RemoveStream(This,pStream) ) 

#define ITStreamControl_EnumerateStreams(This,ppEnumStream)	\
    ( (This)->lpVtbl -> EnumerateStreams(This,ppEnumStream) ) 

#define ITStreamControl_get_Streams(This,pVariant)	\
    ( (This)->lpVtbl -> get_Streams(This,pVariant) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITStreamControl_INTERFACE_DEFINED__ */


#ifndef __ITStream_INTERFACE_DEFINED__
#define __ITStream_INTERFACE_DEFINED__

/* interface ITStream */
/* [object][dual][unique][helpstring][uuid] */ 


EXTERN_C const IID IID_ITStream;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("EE3BD605-3868-11D2-A045-00C04FB6809F")
    ITStream : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MediaType( 
            /* [retval][out] */ __RPC__out long *plMediaType) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Direction( 
            /* [retval][out] */ __RPC__out TERMINAL_DIRECTION *pTD) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppName) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE StartStream( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE PauseStream( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE StopStream( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SelectTerminal( 
            /* [in] */ __RPC__in_opt ITTerminal *pTerminal) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE UnselectTerminal( 
            /* [in] */ __RPC__in_opt ITTerminal *pTerminal) = 0;
        
        virtual /* [hidden][id] */ HRESULT STDMETHODCALLTYPE EnumerateTerminals( 
            /* [out] */ __RPC__deref_out_opt IEnumTerminal **ppEnumTerminal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Terminals( 
            /* [retval][out] */ __RPC__out VARIANT *pTerminals) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITStreamVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITStream * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITStream * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITStream * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITStream * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITStream * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITStream * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITStream * This,
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
        
        DECLSPEC_XFGVIRT(ITStream, get_MediaType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MediaType )( 
            __RPC__in ITStream * This,
            /* [retval][out] */ __RPC__out long *plMediaType);
        
        DECLSPEC_XFGVIRT(ITStream, get_Direction)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Direction )( 
            __RPC__in ITStream * This,
            /* [retval][out] */ __RPC__out TERMINAL_DIRECTION *pTD);
        
        DECLSPEC_XFGVIRT(ITStream, get_Name)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in ITStream * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppName);
        
        DECLSPEC_XFGVIRT(ITStream, StartStream)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *StartStream )( 
            __RPC__in ITStream * This);
        
        DECLSPEC_XFGVIRT(ITStream, PauseStream)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *PauseStream )( 
            __RPC__in ITStream * This);
        
        DECLSPEC_XFGVIRT(ITStream, StopStream)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *StopStream )( 
            __RPC__in ITStream * This);
        
        DECLSPEC_XFGVIRT(ITStream, SelectTerminal)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SelectTerminal )( 
            __RPC__in ITStream * This,
            /* [in] */ __RPC__in_opt ITTerminal *pTerminal);
        
        DECLSPEC_XFGVIRT(ITStream, UnselectTerminal)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *UnselectTerminal )( 
            __RPC__in ITStream * This,
            /* [in] */ __RPC__in_opt ITTerminal *pTerminal);
        
        DECLSPEC_XFGVIRT(ITStream, EnumerateTerminals)
        /* [hidden][id] */ HRESULT ( STDMETHODCALLTYPE *EnumerateTerminals )( 
            __RPC__in ITStream * This,
            /* [out] */ __RPC__deref_out_opt IEnumTerminal **ppEnumTerminal);
        
        DECLSPEC_XFGVIRT(ITStream, get_Terminals)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Terminals )( 
            __RPC__in ITStream * This,
            /* [retval][out] */ __RPC__out VARIANT *pTerminals);
        
        END_INTERFACE
    } ITStreamVtbl;

    interface ITStream
    {
        CONST_VTBL struct ITStreamVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITStream_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITStream_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITStream_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITStream_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITStream_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITStream_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITStream_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITStream_get_MediaType(This,plMediaType)	\
    ( (This)->lpVtbl -> get_MediaType(This,plMediaType) ) 

#define ITStream_get_Direction(This,pTD)	\
    ( (This)->lpVtbl -> get_Direction(This,pTD) ) 

#define ITStream_get_Name(This,ppName)	\
    ( (This)->lpVtbl -> get_Name(This,ppName) ) 

#define ITStream_StartStream(This)	\
    ( (This)->lpVtbl -> StartStream(This) ) 

#define ITStream_PauseStream(This)	\
    ( (This)->lpVtbl -> PauseStream(This) ) 

#define ITStream_StopStream(This)	\
    ( (This)->lpVtbl -> StopStream(This) ) 

#define ITStream_SelectTerminal(This,pTerminal)	\
    ( (This)->lpVtbl -> SelectTerminal(This,pTerminal) ) 

#define ITStream_UnselectTerminal(This,pTerminal)	\
    ( (This)->lpVtbl -> UnselectTerminal(This,pTerminal) ) 

#define ITStream_EnumerateTerminals(This,ppEnumTerminal)	\
    ( (This)->lpVtbl -> EnumerateTerminals(This,ppEnumTerminal) ) 

#define ITStream_get_Terminals(This,pTerminals)	\
    ( (This)->lpVtbl -> get_Terminals(This,pTerminals) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITStream_INTERFACE_DEFINED__ */


#ifndef __IEnumStream_INTERFACE_DEFINED__
#define __IEnumStream_INTERFACE_DEFINED__

/* interface IEnumStream */
/* [object][unique][hidden][helpstring][uuid] */ 


EXTERN_C const IID IID_IEnumStream;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("EE3BD606-3868-11d2-A045-00C04FB6809F")
    IEnumStream : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG celt,
            /* [out] */ __RPC__deref_out_opt ITStream **ppElements,
            /* [full][out][in] */ __RPC__inout_opt ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumStream **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumStreamVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumStream * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumStream * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumStream * This);
        
        DECLSPEC_XFGVIRT(IEnumStream, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IEnumStream * This,
            /* [in] */ ULONG celt,
            /* [out] */ __RPC__deref_out_opt ITStream **ppElements,
            /* [full][out][in] */ __RPC__inout_opt ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IEnumStream, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumStream * This);
        
        DECLSPEC_XFGVIRT(IEnumStream, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumStream * This,
            /* [in] */ ULONG celt);
        
        DECLSPEC_XFGVIRT(IEnumStream, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumStream * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumStream **ppEnum);
        
        END_INTERFACE
    } IEnumStreamVtbl;

    interface IEnumStream
    {
        CONST_VTBL struct IEnumStreamVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumStream_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumStream_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumStream_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumStream_Next(This,celt,ppElements,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,ppElements,pceltFetched) ) 

#define IEnumStream_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumStream_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IEnumStream_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumStream_INTERFACE_DEFINED__ */


#ifndef __ITSubStreamControl_INTERFACE_DEFINED__
#define __ITSubStreamControl_INTERFACE_DEFINED__

/* interface ITSubStreamControl */
/* [object][dual][unique][helpstring][uuid] */ 


EXTERN_C const IID IID_ITSubStreamControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("EE3BD607-3868-11D2-A045-00C04FB6809F")
    ITSubStreamControl : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateSubStream( 
            /* [retval][out] */ __RPC__deref_out_opt ITSubStream **ppSubStream) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE RemoveSubStream( 
            /* [in] */ __RPC__in_opt ITSubStream *pSubStream) = 0;
        
        virtual /* [hidden][id] */ HRESULT STDMETHODCALLTYPE EnumerateSubStreams( 
            /* [out] */ __RPC__deref_out_opt IEnumSubStream **ppEnumSubStream) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SubStreams( 
            /* [retval][out] */ __RPC__out VARIANT *pVariant) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITSubStreamControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITSubStreamControl * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITSubStreamControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITSubStreamControl * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITSubStreamControl * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITSubStreamControl * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITSubStreamControl * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITSubStreamControl * This,
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
        
        DECLSPEC_XFGVIRT(ITSubStreamControl, CreateSubStream)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateSubStream )( 
            __RPC__in ITSubStreamControl * This,
            /* [retval][out] */ __RPC__deref_out_opt ITSubStream **ppSubStream);
        
        DECLSPEC_XFGVIRT(ITSubStreamControl, RemoveSubStream)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RemoveSubStream )( 
            __RPC__in ITSubStreamControl * This,
            /* [in] */ __RPC__in_opt ITSubStream *pSubStream);
        
        DECLSPEC_XFGVIRT(ITSubStreamControl, EnumerateSubStreams)
        /* [hidden][id] */ HRESULT ( STDMETHODCALLTYPE *EnumerateSubStreams )( 
            __RPC__in ITSubStreamControl * This,
            /* [out] */ __RPC__deref_out_opt IEnumSubStream **ppEnumSubStream);
        
        DECLSPEC_XFGVIRT(ITSubStreamControl, get_SubStreams)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SubStreams )( 
            __RPC__in ITSubStreamControl * This,
            /* [retval][out] */ __RPC__out VARIANT *pVariant);
        
        END_INTERFACE
    } ITSubStreamControlVtbl;

    interface ITSubStreamControl
    {
        CONST_VTBL struct ITSubStreamControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITSubStreamControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITSubStreamControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITSubStreamControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITSubStreamControl_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITSubStreamControl_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITSubStreamControl_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITSubStreamControl_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITSubStreamControl_CreateSubStream(This,ppSubStream)	\
    ( (This)->lpVtbl -> CreateSubStream(This,ppSubStream) ) 

#define ITSubStreamControl_RemoveSubStream(This,pSubStream)	\
    ( (This)->lpVtbl -> RemoveSubStream(This,pSubStream) ) 

#define ITSubStreamControl_EnumerateSubStreams(This,ppEnumSubStream)	\
    ( (This)->lpVtbl -> EnumerateSubStreams(This,ppEnumSubStream) ) 

#define ITSubStreamControl_get_SubStreams(This,pVariant)	\
    ( (This)->lpVtbl -> get_SubStreams(This,pVariant) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITSubStreamControl_INTERFACE_DEFINED__ */


#ifndef __ITSubStream_INTERFACE_DEFINED__
#define __ITSubStream_INTERFACE_DEFINED__

/* interface ITSubStream */
/* [object][dual][unique][helpstring][uuid] */ 


EXTERN_C const IID IID_ITSubStream;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("EE3BD608-3868-11D2-A045-00C04FB6809F")
    ITSubStream : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE StartSubStream( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE PauseSubStream( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE StopSubStream( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SelectTerminal( 
            /* [in] */ __RPC__in_opt ITTerminal *pTerminal) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE UnselectTerminal( 
            /* [in] */ __RPC__in_opt ITTerminal *pTerminal) = 0;
        
        virtual /* [hidden][id] */ HRESULT STDMETHODCALLTYPE EnumerateTerminals( 
            /* [out] */ __RPC__deref_out_opt IEnumTerminal **ppEnumTerminal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Terminals( 
            /* [retval][out] */ __RPC__out VARIANT *pTerminals) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Stream( 
            /* [retval][out] */ __RPC__deref_out_opt ITStream **ppITStream) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITSubStreamVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITSubStream * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITSubStream * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITSubStream * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITSubStream * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITSubStream * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITSubStream * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITSubStream * This,
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
        
        DECLSPEC_XFGVIRT(ITSubStream, StartSubStream)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *StartSubStream )( 
            __RPC__in ITSubStream * This);
        
        DECLSPEC_XFGVIRT(ITSubStream, PauseSubStream)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *PauseSubStream )( 
            __RPC__in ITSubStream * This);
        
        DECLSPEC_XFGVIRT(ITSubStream, StopSubStream)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *StopSubStream )( 
            __RPC__in ITSubStream * This);
        
        DECLSPEC_XFGVIRT(ITSubStream, SelectTerminal)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SelectTerminal )( 
            __RPC__in ITSubStream * This,
            /* [in] */ __RPC__in_opt ITTerminal *pTerminal);
        
        DECLSPEC_XFGVIRT(ITSubStream, UnselectTerminal)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *UnselectTerminal )( 
            __RPC__in ITSubStream * This,
            /* [in] */ __RPC__in_opt ITTerminal *pTerminal);
        
        DECLSPEC_XFGVIRT(ITSubStream, EnumerateTerminals)
        /* [hidden][id] */ HRESULT ( STDMETHODCALLTYPE *EnumerateTerminals )( 
            __RPC__in ITSubStream * This,
            /* [out] */ __RPC__deref_out_opt IEnumTerminal **ppEnumTerminal);
        
        DECLSPEC_XFGVIRT(ITSubStream, get_Terminals)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Terminals )( 
            __RPC__in ITSubStream * This,
            /* [retval][out] */ __RPC__out VARIANT *pTerminals);
        
        DECLSPEC_XFGVIRT(ITSubStream, get_Stream)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Stream )( 
            __RPC__in ITSubStream * This,
            /* [retval][out] */ __RPC__deref_out_opt ITStream **ppITStream);
        
        END_INTERFACE
    } ITSubStreamVtbl;

    interface ITSubStream
    {
        CONST_VTBL struct ITSubStreamVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITSubStream_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITSubStream_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITSubStream_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITSubStream_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITSubStream_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITSubStream_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITSubStream_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITSubStream_StartSubStream(This)	\
    ( (This)->lpVtbl -> StartSubStream(This) ) 

#define ITSubStream_PauseSubStream(This)	\
    ( (This)->lpVtbl -> PauseSubStream(This) ) 

#define ITSubStream_StopSubStream(This)	\
    ( (This)->lpVtbl -> StopSubStream(This) ) 

#define ITSubStream_SelectTerminal(This,pTerminal)	\
    ( (This)->lpVtbl -> SelectTerminal(This,pTerminal) ) 

#define ITSubStream_UnselectTerminal(This,pTerminal)	\
    ( (This)->lpVtbl -> UnselectTerminal(This,pTerminal) ) 

#define ITSubStream_EnumerateTerminals(This,ppEnumTerminal)	\
    ( (This)->lpVtbl -> EnumerateTerminals(This,ppEnumTerminal) ) 

#define ITSubStream_get_Terminals(This,pTerminals)	\
    ( (This)->lpVtbl -> get_Terminals(This,pTerminals) ) 

#define ITSubStream_get_Stream(This,ppITStream)	\
    ( (This)->lpVtbl -> get_Stream(This,ppITStream) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITSubStream_INTERFACE_DEFINED__ */


#ifndef __IEnumSubStream_INTERFACE_DEFINED__
#define __IEnumSubStream_INTERFACE_DEFINED__

/* interface IEnumSubStream */
/* [object][unique][hidden][helpstring][uuid] */ 


EXTERN_C const IID IID_IEnumSubStream;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("EE3BD609-3868-11d2-A045-00C04FB6809F")
    IEnumSubStream : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG celt,
            /* [out] */ __RPC__deref_out_opt ITSubStream **ppElements,
            /* [full][out][in] */ __RPC__inout_opt ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumSubStream **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumSubStreamVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumSubStream * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumSubStream * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumSubStream * This);
        
        DECLSPEC_XFGVIRT(IEnumSubStream, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IEnumSubStream * This,
            /* [in] */ ULONG celt,
            /* [out] */ __RPC__deref_out_opt ITSubStream **ppElements,
            /* [full][out][in] */ __RPC__inout_opt ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IEnumSubStream, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumSubStream * This);
        
        DECLSPEC_XFGVIRT(IEnumSubStream, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumSubStream * This,
            /* [in] */ ULONG celt);
        
        DECLSPEC_XFGVIRT(IEnumSubStream, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumSubStream * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumSubStream **ppEnum);
        
        END_INTERFACE
    } IEnumSubStreamVtbl;

    interface IEnumSubStream
    {
        CONST_VTBL struct IEnumSubStreamVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumSubStream_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumSubStream_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumSubStream_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumSubStream_Next(This,celt,ppElements,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,ppElements,pceltFetched) ) 

#define IEnumSubStream_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumSubStream_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IEnumSubStream_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumSubStream_INTERFACE_DEFINED__ */


#ifndef __ITLegacyWaveSupport_INTERFACE_DEFINED__
#define __ITLegacyWaveSupport_INTERFACE_DEFINED__

/* interface ITLegacyWaveSupport */
/* [object][dual][helpstring][uuid] */ 


EXTERN_C const IID IID_ITLegacyWaveSupport;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("207823EA-E252-11d2-B77E-0080C7135381")
    ITLegacyWaveSupport : public IDispatch
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE IsFullDuplex( 
            /* [out] */ __RPC__out FULLDUPLEX_SUPPORT *pSupport) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITLegacyWaveSupportVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITLegacyWaveSupport * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITLegacyWaveSupport * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITLegacyWaveSupport * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITLegacyWaveSupport * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITLegacyWaveSupport * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITLegacyWaveSupport * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITLegacyWaveSupport * This,
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
        
        DECLSPEC_XFGVIRT(ITLegacyWaveSupport, IsFullDuplex)
        HRESULT ( STDMETHODCALLTYPE *IsFullDuplex )( 
            __RPC__in ITLegacyWaveSupport * This,
            /* [out] */ __RPC__out FULLDUPLEX_SUPPORT *pSupport);
        
        END_INTERFACE
    } ITLegacyWaveSupportVtbl;

    interface ITLegacyWaveSupport
    {
        CONST_VTBL struct ITLegacyWaveSupportVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITLegacyWaveSupport_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITLegacyWaveSupport_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITLegacyWaveSupport_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITLegacyWaveSupport_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITLegacyWaveSupport_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITLegacyWaveSupport_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITLegacyWaveSupport_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITLegacyWaveSupport_IsFullDuplex(This,pSupport)	\
    ( (This)->lpVtbl -> IsFullDuplex(This,pSupport) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITLegacyWaveSupport_INTERFACE_DEFINED__ */


#ifndef __ITBasicCallControl2_INTERFACE_DEFINED__
#define __ITBasicCallControl2_INTERFACE_DEFINED__

/* interface ITBasicCallControl2 */
/* [object][dual][helpstring][uuid] */ 


EXTERN_C const IID IID_ITBasicCallControl2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("161A4A56-1E99-4b3f-A46A-168F38A5EE4C")
    ITBasicCallControl2 : public ITBasicCallControl
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE RequestTerminal( 
            /* [in] */ __RPC__in BSTR bstrTerminalClassGUID,
            /* [in] */ long lMediaType,
            /* [in] */ TERMINAL_DIRECTION Direction,
            /* [retval][out] */ __RPC__deref_out_opt ITTerminal **ppTerminal) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SelectTerminalOnCall( 
            /* [in] */ __RPC__in_opt ITTerminal *pTerminal) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE UnselectTerminalOnCall( 
            /* [in] */ __RPC__in_opt ITTerminal *pTerminal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITBasicCallControl2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITBasicCallControl2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITBasicCallControl2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITBasicCallControl2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITBasicCallControl2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITBasicCallControl2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITBasicCallControl2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITBasicCallControl2 * This,
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
        
        DECLSPEC_XFGVIRT(ITBasicCallControl, Connect)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Connect )( 
            __RPC__in ITBasicCallControl2 * This,
            /* [in] */ VARIANT_BOOL fSync);
        
        DECLSPEC_XFGVIRT(ITBasicCallControl, Answer)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Answer )( 
            __RPC__in ITBasicCallControl2 * This);
        
        DECLSPEC_XFGVIRT(ITBasicCallControl, Disconnect)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Disconnect )( 
            __RPC__in ITBasicCallControl2 * This,
            /* [in] */ DISCONNECT_CODE code);
        
        DECLSPEC_XFGVIRT(ITBasicCallControl, Hold)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Hold )( 
            __RPC__in ITBasicCallControl2 * This,
            /* [in] */ VARIANT_BOOL fHold);
        
        DECLSPEC_XFGVIRT(ITBasicCallControl, HandoffDirect)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *HandoffDirect )( 
            __RPC__in ITBasicCallControl2 * This,
            /* [in] */ __RPC__in BSTR pApplicationName);
        
        DECLSPEC_XFGVIRT(ITBasicCallControl, HandoffIndirect)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *HandoffIndirect )( 
            __RPC__in ITBasicCallControl2 * This,
            /* [in] */ long lMediaType);
        
        DECLSPEC_XFGVIRT(ITBasicCallControl, Conference)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Conference )( 
            __RPC__in ITBasicCallControl2 * This,
            /* [in] */ __RPC__in_opt ITBasicCallControl *pCall,
            /* [in] */ VARIANT_BOOL fSync);
        
        DECLSPEC_XFGVIRT(ITBasicCallControl, Transfer)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Transfer )( 
            __RPC__in ITBasicCallControl2 * This,
            /* [in] */ __RPC__in_opt ITBasicCallControl *pCall,
            /* [in] */ VARIANT_BOOL fSync);
        
        DECLSPEC_XFGVIRT(ITBasicCallControl, BlindTransfer)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *BlindTransfer )( 
            __RPC__in ITBasicCallControl2 * This,
            /* [in] */ __RPC__in BSTR pDestAddress);
        
        DECLSPEC_XFGVIRT(ITBasicCallControl, SwapHold)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SwapHold )( 
            __RPC__in ITBasicCallControl2 * This,
            /* [in] */ __RPC__in_opt ITBasicCallControl *pCall);
        
        DECLSPEC_XFGVIRT(ITBasicCallControl, ParkDirect)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ParkDirect )( 
            __RPC__in ITBasicCallControl2 * This,
            /* [in] */ __RPC__in BSTR pParkAddress);
        
        DECLSPEC_XFGVIRT(ITBasicCallControl, ParkIndirect)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ParkIndirect )( 
            __RPC__in ITBasicCallControl2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ppNonDirAddress);
        
        DECLSPEC_XFGVIRT(ITBasicCallControl, Unpark)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Unpark )( 
            __RPC__in ITBasicCallControl2 * This);
        
        DECLSPEC_XFGVIRT(ITBasicCallControl, SetQOS)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetQOS )( 
            __RPC__in ITBasicCallControl2 * This,
            /* [in] */ long lMediaType,
            /* [in] */ QOS_SERVICE_LEVEL ServiceLevel);
        
        DECLSPEC_XFGVIRT(ITBasicCallControl, Pickup)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Pickup )( 
            __RPC__in ITBasicCallControl2 * This,
            /* [in] */ __RPC__in BSTR pGroupID);
        
        DECLSPEC_XFGVIRT(ITBasicCallControl, Dial)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Dial )( 
            __RPC__in ITBasicCallControl2 * This,
            /* [in] */ __RPC__in BSTR pDestAddress);
        
        DECLSPEC_XFGVIRT(ITBasicCallControl, Finish)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Finish )( 
            __RPC__in ITBasicCallControl2 * This,
            /* [in] */ FINISH_MODE finishMode);
        
        DECLSPEC_XFGVIRT(ITBasicCallControl, RemoveFromConference)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RemoveFromConference )( 
            __RPC__in ITBasicCallControl2 * This);
        
        DECLSPEC_XFGVIRT(ITBasicCallControl2, RequestTerminal)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RequestTerminal )( 
            __RPC__in ITBasicCallControl2 * This,
            /* [in] */ __RPC__in BSTR bstrTerminalClassGUID,
            /* [in] */ long lMediaType,
            /* [in] */ TERMINAL_DIRECTION Direction,
            /* [retval][out] */ __RPC__deref_out_opt ITTerminal **ppTerminal);
        
        DECLSPEC_XFGVIRT(ITBasicCallControl2, SelectTerminalOnCall)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SelectTerminalOnCall )( 
            __RPC__in ITBasicCallControl2 * This,
            /* [in] */ __RPC__in_opt ITTerminal *pTerminal);
        
        DECLSPEC_XFGVIRT(ITBasicCallControl2, UnselectTerminalOnCall)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *UnselectTerminalOnCall )( 
            __RPC__in ITBasicCallControl2 * This,
            /* [in] */ __RPC__in_opt ITTerminal *pTerminal);
        
        END_INTERFACE
    } ITBasicCallControl2Vtbl;

    interface ITBasicCallControl2
    {
        CONST_VTBL struct ITBasicCallControl2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITBasicCallControl2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITBasicCallControl2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITBasicCallControl2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITBasicCallControl2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITBasicCallControl2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITBasicCallControl2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITBasicCallControl2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITBasicCallControl2_Connect(This,fSync)	\
    ( (This)->lpVtbl -> Connect(This,fSync) ) 

#define ITBasicCallControl2_Answer(This)	\
    ( (This)->lpVtbl -> Answer(This) ) 

#define ITBasicCallControl2_Disconnect(This,code)	\
    ( (This)->lpVtbl -> Disconnect(This,code) ) 

#define ITBasicCallControl2_Hold(This,fHold)	\
    ( (This)->lpVtbl -> Hold(This,fHold) ) 

#define ITBasicCallControl2_HandoffDirect(This,pApplicationName)	\
    ( (This)->lpVtbl -> HandoffDirect(This,pApplicationName) ) 

#define ITBasicCallControl2_HandoffIndirect(This,lMediaType)	\
    ( (This)->lpVtbl -> HandoffIndirect(This,lMediaType) ) 

#define ITBasicCallControl2_Conference(This,pCall,fSync)	\
    ( (This)->lpVtbl -> Conference(This,pCall,fSync) ) 

#define ITBasicCallControl2_Transfer(This,pCall,fSync)	\
    ( (This)->lpVtbl -> Transfer(This,pCall,fSync) ) 

#define ITBasicCallControl2_BlindTransfer(This,pDestAddress)	\
    ( (This)->lpVtbl -> BlindTransfer(This,pDestAddress) ) 

#define ITBasicCallControl2_SwapHold(This,pCall)	\
    ( (This)->lpVtbl -> SwapHold(This,pCall) ) 

#define ITBasicCallControl2_ParkDirect(This,pParkAddress)	\
    ( (This)->lpVtbl -> ParkDirect(This,pParkAddress) ) 

#define ITBasicCallControl2_ParkIndirect(This,ppNonDirAddress)	\
    ( (This)->lpVtbl -> ParkIndirect(This,ppNonDirAddress) ) 

#define ITBasicCallControl2_Unpark(This)	\
    ( (This)->lpVtbl -> Unpark(This) ) 

#define ITBasicCallControl2_SetQOS(This,lMediaType,ServiceLevel)	\
    ( (This)->lpVtbl -> SetQOS(This,lMediaType,ServiceLevel) ) 

#define ITBasicCallControl2_Pickup(This,pGroupID)	\
    ( (This)->lpVtbl -> Pickup(This,pGroupID) ) 

#define ITBasicCallControl2_Dial(This,pDestAddress)	\
    ( (This)->lpVtbl -> Dial(This,pDestAddress) ) 

#define ITBasicCallControl2_Finish(This,finishMode)	\
    ( (This)->lpVtbl -> Finish(This,finishMode) ) 

#define ITBasicCallControl2_RemoveFromConference(This)	\
    ( (This)->lpVtbl -> RemoveFromConference(This) ) 


#define ITBasicCallControl2_RequestTerminal(This,bstrTerminalClassGUID,lMediaType,Direction,ppTerminal)	\
    ( (This)->lpVtbl -> RequestTerminal(This,bstrTerminalClassGUID,lMediaType,Direction,ppTerminal) ) 

#define ITBasicCallControl2_SelectTerminalOnCall(This,pTerminal)	\
    ( (This)->lpVtbl -> SelectTerminalOnCall(This,pTerminal) ) 

#define ITBasicCallControl2_UnselectTerminalOnCall(This,pTerminal)	\
    ( (This)->lpVtbl -> UnselectTerminalOnCall(This,pTerminal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITBasicCallControl2_INTERFACE_DEFINED__ */


#ifndef __ITScriptableAudioFormat_INTERFACE_DEFINED__
#define __ITScriptableAudioFormat_INTERFACE_DEFINED__

/* interface ITScriptableAudioFormat */
/* [object][dual][helpstring][uuid] */ 


EXTERN_C const IID IID_ITScriptableAudioFormat;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B87658BD-3C59-4f64-BE74-AEDE3E86A81E")
    ITScriptableAudioFormat : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Channels( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Channels( 
            /* [in] */ const long nNewVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SamplesPerSec( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_SamplesPerSec( 
            /* [in] */ const long nNewVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_AvgBytesPerSec( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_AvgBytesPerSec( 
            /* [in] */ const long nNewVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_BlockAlign( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_BlockAlign( 
            /* [in] */ const long nNewVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_BitsPerSample( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_BitsPerSample( 
            /* [in] */ const long nNewVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_FormatTag( 
            /* [retval][out] */ __RPC__out long *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_FormatTag( 
            /* [in] */ const long nNewVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITScriptableAudioFormatVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITScriptableAudioFormat * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITScriptableAudioFormat * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITScriptableAudioFormat * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITScriptableAudioFormat * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITScriptableAudioFormat * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITScriptableAudioFormat * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITScriptableAudioFormat * This,
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
        
        DECLSPEC_XFGVIRT(ITScriptableAudioFormat, get_Channels)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Channels )( 
            __RPC__in ITScriptableAudioFormat * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(ITScriptableAudioFormat, put_Channels)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Channels )( 
            __RPC__in ITScriptableAudioFormat * This,
            /* [in] */ const long nNewVal);
        
        DECLSPEC_XFGVIRT(ITScriptableAudioFormat, get_SamplesPerSec)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SamplesPerSec )( 
            __RPC__in ITScriptableAudioFormat * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(ITScriptableAudioFormat, put_SamplesPerSec)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SamplesPerSec )( 
            __RPC__in ITScriptableAudioFormat * This,
            /* [in] */ const long nNewVal);
        
        DECLSPEC_XFGVIRT(ITScriptableAudioFormat, get_AvgBytesPerSec)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AvgBytesPerSec )( 
            __RPC__in ITScriptableAudioFormat * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(ITScriptableAudioFormat, put_AvgBytesPerSec)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_AvgBytesPerSec )( 
            __RPC__in ITScriptableAudioFormat * This,
            /* [in] */ const long nNewVal);
        
        DECLSPEC_XFGVIRT(ITScriptableAudioFormat, get_BlockAlign)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_BlockAlign )( 
            __RPC__in ITScriptableAudioFormat * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(ITScriptableAudioFormat, put_BlockAlign)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_BlockAlign )( 
            __RPC__in ITScriptableAudioFormat * This,
            /* [in] */ const long nNewVal);
        
        DECLSPEC_XFGVIRT(ITScriptableAudioFormat, get_BitsPerSample)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_BitsPerSample )( 
            __RPC__in ITScriptableAudioFormat * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(ITScriptableAudioFormat, put_BitsPerSample)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_BitsPerSample )( 
            __RPC__in ITScriptableAudioFormat * This,
            /* [in] */ const long nNewVal);
        
        DECLSPEC_XFGVIRT(ITScriptableAudioFormat, get_FormatTag)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FormatTag )( 
            __RPC__in ITScriptableAudioFormat * This,
            /* [retval][out] */ __RPC__out long *pVal);
        
        DECLSPEC_XFGVIRT(ITScriptableAudioFormat, put_FormatTag)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_FormatTag )( 
            __RPC__in ITScriptableAudioFormat * This,
            /* [in] */ const long nNewVal);
        
        END_INTERFACE
    } ITScriptableAudioFormatVtbl;

    interface ITScriptableAudioFormat
    {
        CONST_VTBL struct ITScriptableAudioFormatVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITScriptableAudioFormat_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITScriptableAudioFormat_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITScriptableAudioFormat_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITScriptableAudioFormat_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITScriptableAudioFormat_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITScriptableAudioFormat_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITScriptableAudioFormat_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITScriptableAudioFormat_get_Channels(This,pVal)	\
    ( (This)->lpVtbl -> get_Channels(This,pVal) ) 

#define ITScriptableAudioFormat_put_Channels(This,nNewVal)	\
    ( (This)->lpVtbl -> put_Channels(This,nNewVal) ) 

#define ITScriptableAudioFormat_get_SamplesPerSec(This,pVal)	\
    ( (This)->lpVtbl -> get_SamplesPerSec(This,pVal) ) 

#define ITScriptableAudioFormat_put_SamplesPerSec(This,nNewVal)	\
    ( (This)->lpVtbl -> put_SamplesPerSec(This,nNewVal) ) 

#define ITScriptableAudioFormat_get_AvgBytesPerSec(This,pVal)	\
    ( (This)->lpVtbl -> get_AvgBytesPerSec(This,pVal) ) 

#define ITScriptableAudioFormat_put_AvgBytesPerSec(This,nNewVal)	\
    ( (This)->lpVtbl -> put_AvgBytesPerSec(This,nNewVal) ) 

#define ITScriptableAudioFormat_get_BlockAlign(This,pVal)	\
    ( (This)->lpVtbl -> get_BlockAlign(This,pVal) ) 

#define ITScriptableAudioFormat_put_BlockAlign(This,nNewVal)	\
    ( (This)->lpVtbl -> put_BlockAlign(This,nNewVal) ) 

#define ITScriptableAudioFormat_get_BitsPerSample(This,pVal)	\
    ( (This)->lpVtbl -> get_BitsPerSample(This,pVal) ) 

#define ITScriptableAudioFormat_put_BitsPerSample(This,nNewVal)	\
    ( (This)->lpVtbl -> put_BitsPerSample(This,nNewVal) ) 

#define ITScriptableAudioFormat_get_FormatTag(This,pVal)	\
    ( (This)->lpVtbl -> get_FormatTag(This,pVal) ) 

#define ITScriptableAudioFormat_put_FormatTag(This,nNewVal)	\
    ( (This)->lpVtbl -> put_FormatTag(This,nNewVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITScriptableAudioFormat_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_tapi3if_0000_0083 */
/* [local] */ 

/****************************************
 * Terminal Classes
 ****************************************/

// Video Windows - {F7438990-D6EB-11d0-82A6-00AA00B5CA1B}
EXTERN_C const CLSID CLSID_VideoWindowTerm;

// Video input (camera) {AAF578EC-DC70-11d0-8ED3-00C04FB6809F}
EXTERN_C const CLSID CLSID_VideoInputTerminal;

// Handset device {AAF578EB-DC70-11d0-8ED3-00C04FB6809F}
EXTERN_C const CLSID CLSID_HandsetTerminal;

// Headset device {AAF578ED-DC70-11d0-8ED3-00C04FB6809F}
EXTERN_C const CLSID CLSID_HeadsetTerminal;

// Speakerphone device {AAF578EE-DC70-11d0-8ED3-00C04FB6809F}
EXTERN_C const CLSID CLSID_SpeakerphoneTerminal;

// Microphone (sound card) {AAF578EF-DC70-11d0-8ED3-00C04FB6809F}
EXTERN_C const CLSID CLSID_MicrophoneTerminal;

// Speakers (sound card) {AAF578F0-DC70-11d0-8ED3-00C04FB6809F}
EXTERN_C const CLSID CLSID_SpeakersTerminal;

// Media stream terminal {E2F7AEF7-4971-11D1-A671-006097C9A2E8}
EXTERN_C const CLSID CLSID_MediaStreamTerminal;

// File Recording terminal {521F3D06-C3D0-4511-8617-86B9A783DA77}
EXTERN_C const CLSID CLSID_FileRecordingTerminal;

// File Recording Track {BF14A2E4-E88B-4ef5-9740-5AC5D022F8C9}
EXTERN_C const CLSID CLSID_FileRecordingTrack;

// File Playback Terminal {0CB9914C-79CD-47dc-ADB0-327F47CEFB20}
EXTERN_C const CLSID CLSID_FilePlaybackTerminal;

// define the media modes
#define TAPIMEDIATYPE_AUDIO                     0x8
#define TAPIMEDIATYPE_VIDEO                     0x8000
#define TAPIMEDIATYPE_DATAMODEM                 0x10
#define TAPIMEDIATYPE_G3FAX                     0x20
#define TAPIMEDIATYPE_MULTITRACK                0x10000

// {831CE2D6-83B5-11d1-BB5C-00C04FB6809F}
EXTERN_C const CLSID TAPIPROTOCOL_PSTN;

// {831CE2D7-83B5-11d1-BB5C-00C04FB6809F}
EXTERN_C const CLSID TAPIPROTOCOL_H323;

// {831CE2D8-83B5-11d1-BB5C-00C04FB6809F}
EXTERN_C const CLSID TAPIPROTOCOL_Multicast;

#define __TapiConstants_MODULE_DEFINED__
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_tapi3if_0000_0083_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_tapi3if_0000_0083_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  HWND_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HWND * ); 
void                      __RPC_USER  HWND_UserFree(     __RPC__in unsigned long *, __RPC__in HWND * ); 

unsigned long             __RPC_USER  VARIANT_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

unsigned long             __RPC_USER  BSTR_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree64(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  HWND_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HWND * ); 
void                      __RPC_USER  HWND_UserFree64(     __RPC__in unsigned long *, __RPC__in HWND * ); 

unsigned long             __RPC_USER  VARIANT_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree64(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif



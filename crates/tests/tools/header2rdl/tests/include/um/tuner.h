

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

#ifndef __tuner_h__
#define __tuner_h__

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

#ifndef __ITuningSpaces_FWD_DEFINED__
#define __ITuningSpaces_FWD_DEFINED__
typedef interface ITuningSpaces ITuningSpaces;

#endif 	/* __ITuningSpaces_FWD_DEFINED__ */


#ifndef __ITuningSpaceContainer_FWD_DEFINED__
#define __ITuningSpaceContainer_FWD_DEFINED__
typedef interface ITuningSpaceContainer ITuningSpaceContainer;

#endif 	/* __ITuningSpaceContainer_FWD_DEFINED__ */


#ifndef __ITuningSpace_FWD_DEFINED__
#define __ITuningSpace_FWD_DEFINED__
typedef interface ITuningSpace ITuningSpace;

#endif 	/* __ITuningSpace_FWD_DEFINED__ */


#ifndef __IEnumTuningSpaces_FWD_DEFINED__
#define __IEnumTuningSpaces_FWD_DEFINED__
typedef interface IEnumTuningSpaces IEnumTuningSpaces;

#endif 	/* __IEnumTuningSpaces_FWD_DEFINED__ */


#ifndef __IDVBTuningSpace_FWD_DEFINED__
#define __IDVBTuningSpace_FWD_DEFINED__
typedef interface IDVBTuningSpace IDVBTuningSpace;

#endif 	/* __IDVBTuningSpace_FWD_DEFINED__ */


#ifndef __IDVBTuningSpace2_FWD_DEFINED__
#define __IDVBTuningSpace2_FWD_DEFINED__
typedef interface IDVBTuningSpace2 IDVBTuningSpace2;

#endif 	/* __IDVBTuningSpace2_FWD_DEFINED__ */


#ifndef __IDVBSTuningSpace_FWD_DEFINED__
#define __IDVBSTuningSpace_FWD_DEFINED__
typedef interface IDVBSTuningSpace IDVBSTuningSpace;

#endif 	/* __IDVBSTuningSpace_FWD_DEFINED__ */


#ifndef __IAuxInTuningSpace_FWD_DEFINED__
#define __IAuxInTuningSpace_FWD_DEFINED__
typedef interface IAuxInTuningSpace IAuxInTuningSpace;

#endif 	/* __IAuxInTuningSpace_FWD_DEFINED__ */


#ifndef __IAuxInTuningSpace2_FWD_DEFINED__
#define __IAuxInTuningSpace2_FWD_DEFINED__
typedef interface IAuxInTuningSpace2 IAuxInTuningSpace2;

#endif 	/* __IAuxInTuningSpace2_FWD_DEFINED__ */


#ifndef __IAnalogTVTuningSpace_FWD_DEFINED__
#define __IAnalogTVTuningSpace_FWD_DEFINED__
typedef interface IAnalogTVTuningSpace IAnalogTVTuningSpace;

#endif 	/* __IAnalogTVTuningSpace_FWD_DEFINED__ */


#ifndef __IATSCTuningSpace_FWD_DEFINED__
#define __IATSCTuningSpace_FWD_DEFINED__
typedef interface IATSCTuningSpace IATSCTuningSpace;

#endif 	/* __IATSCTuningSpace_FWD_DEFINED__ */


#ifndef __IDigitalCableTuningSpace_FWD_DEFINED__
#define __IDigitalCableTuningSpace_FWD_DEFINED__
typedef interface IDigitalCableTuningSpace IDigitalCableTuningSpace;

#endif 	/* __IDigitalCableTuningSpace_FWD_DEFINED__ */


#ifndef __IAnalogRadioTuningSpace_FWD_DEFINED__
#define __IAnalogRadioTuningSpace_FWD_DEFINED__
typedef interface IAnalogRadioTuningSpace IAnalogRadioTuningSpace;

#endif 	/* __IAnalogRadioTuningSpace_FWD_DEFINED__ */


#ifndef __IAnalogRadioTuningSpace2_FWD_DEFINED__
#define __IAnalogRadioTuningSpace2_FWD_DEFINED__
typedef interface IAnalogRadioTuningSpace2 IAnalogRadioTuningSpace2;

#endif 	/* __IAnalogRadioTuningSpace2_FWD_DEFINED__ */


#ifndef __ITuneRequest_FWD_DEFINED__
#define __ITuneRequest_FWD_DEFINED__
typedef interface ITuneRequest ITuneRequest;

#endif 	/* __ITuneRequest_FWD_DEFINED__ */


#ifndef __IChannelIDTuneRequest_FWD_DEFINED__
#define __IChannelIDTuneRequest_FWD_DEFINED__
typedef interface IChannelIDTuneRequest IChannelIDTuneRequest;

#endif 	/* __IChannelIDTuneRequest_FWD_DEFINED__ */


#ifndef __IChannelTuneRequest_FWD_DEFINED__
#define __IChannelTuneRequest_FWD_DEFINED__
typedef interface IChannelTuneRequest IChannelTuneRequest;

#endif 	/* __IChannelTuneRequest_FWD_DEFINED__ */


#ifndef __IATSCChannelTuneRequest_FWD_DEFINED__
#define __IATSCChannelTuneRequest_FWD_DEFINED__
typedef interface IATSCChannelTuneRequest IATSCChannelTuneRequest;

#endif 	/* __IATSCChannelTuneRequest_FWD_DEFINED__ */


#ifndef __IDigitalCableTuneRequest_FWD_DEFINED__
#define __IDigitalCableTuneRequest_FWD_DEFINED__
typedef interface IDigitalCableTuneRequest IDigitalCableTuneRequest;

#endif 	/* __IDigitalCableTuneRequest_FWD_DEFINED__ */


#ifndef __IDVBTuneRequest_FWD_DEFINED__
#define __IDVBTuneRequest_FWD_DEFINED__
typedef interface IDVBTuneRequest IDVBTuneRequest;

#endif 	/* __IDVBTuneRequest_FWD_DEFINED__ */


#ifndef __IMPEG2TuneRequest_FWD_DEFINED__
#define __IMPEG2TuneRequest_FWD_DEFINED__
typedef interface IMPEG2TuneRequest IMPEG2TuneRequest;

#endif 	/* __IMPEG2TuneRequest_FWD_DEFINED__ */


#ifndef __IMPEG2TuneRequestFactory_FWD_DEFINED__
#define __IMPEG2TuneRequestFactory_FWD_DEFINED__
typedef interface IMPEG2TuneRequestFactory IMPEG2TuneRequestFactory;

#endif 	/* __IMPEG2TuneRequestFactory_FWD_DEFINED__ */


#ifndef __IMPEG2TuneRequestSupport_FWD_DEFINED__
#define __IMPEG2TuneRequestSupport_FWD_DEFINED__
typedef interface IMPEG2TuneRequestSupport IMPEG2TuneRequestSupport;

#endif 	/* __IMPEG2TuneRequestSupport_FWD_DEFINED__ */


#ifndef __ITunerCap_FWD_DEFINED__
#define __ITunerCap_FWD_DEFINED__
typedef interface ITunerCap ITunerCap;

#endif 	/* __ITunerCap_FWD_DEFINED__ */


#ifndef __ITunerCapEx_FWD_DEFINED__
#define __ITunerCapEx_FWD_DEFINED__
typedef interface ITunerCapEx ITunerCapEx;

#endif 	/* __ITunerCapEx_FWD_DEFINED__ */


#ifndef __ITuner_FWD_DEFINED__
#define __ITuner_FWD_DEFINED__
typedef interface ITuner ITuner;

#endif 	/* __ITuner_FWD_DEFINED__ */


#ifndef __IScanningTuner_FWD_DEFINED__
#define __IScanningTuner_FWD_DEFINED__
typedef interface IScanningTuner IScanningTuner;

#endif 	/* __IScanningTuner_FWD_DEFINED__ */


#ifndef __IScanningTunerEx_FWD_DEFINED__
#define __IScanningTunerEx_FWD_DEFINED__
typedef interface IScanningTunerEx IScanningTunerEx;

#endif 	/* __IScanningTunerEx_FWD_DEFINED__ */


#ifndef __IComponentType_FWD_DEFINED__
#define __IComponentType_FWD_DEFINED__
typedef interface IComponentType IComponentType;

#endif 	/* __IComponentType_FWD_DEFINED__ */


#ifndef __ILanguageComponentType_FWD_DEFINED__
#define __ILanguageComponentType_FWD_DEFINED__
typedef interface ILanguageComponentType ILanguageComponentType;

#endif 	/* __ILanguageComponentType_FWD_DEFINED__ */


#ifndef __IMPEG2ComponentType_FWD_DEFINED__
#define __IMPEG2ComponentType_FWD_DEFINED__
typedef interface IMPEG2ComponentType IMPEG2ComponentType;

#endif 	/* __IMPEG2ComponentType_FWD_DEFINED__ */


#ifndef __IATSCComponentType_FWD_DEFINED__
#define __IATSCComponentType_FWD_DEFINED__
typedef interface IATSCComponentType IATSCComponentType;

#endif 	/* __IATSCComponentType_FWD_DEFINED__ */


#ifndef __IEnumComponentTypes_FWD_DEFINED__
#define __IEnumComponentTypes_FWD_DEFINED__
typedef interface IEnumComponentTypes IEnumComponentTypes;

#endif 	/* __IEnumComponentTypes_FWD_DEFINED__ */


#ifndef __IComponentTypes_FWD_DEFINED__
#define __IComponentTypes_FWD_DEFINED__
typedef interface IComponentTypes IComponentTypes;

#endif 	/* __IComponentTypes_FWD_DEFINED__ */


#ifndef __IComponent_FWD_DEFINED__
#define __IComponent_FWD_DEFINED__
typedef interface IComponent IComponent;

#endif 	/* __IComponent_FWD_DEFINED__ */


#ifndef __IAnalogAudioComponentType_FWD_DEFINED__
#define __IAnalogAudioComponentType_FWD_DEFINED__
typedef interface IAnalogAudioComponentType IAnalogAudioComponentType;

#endif 	/* __IAnalogAudioComponentType_FWD_DEFINED__ */


#ifndef __IMPEG2Component_FWD_DEFINED__
#define __IMPEG2Component_FWD_DEFINED__
typedef interface IMPEG2Component IMPEG2Component;

#endif 	/* __IMPEG2Component_FWD_DEFINED__ */


#ifndef __IEnumComponents_FWD_DEFINED__
#define __IEnumComponents_FWD_DEFINED__
typedef interface IEnumComponents IEnumComponents;

#endif 	/* __IEnumComponents_FWD_DEFINED__ */


#ifndef __IComponents_FWD_DEFINED__
#define __IComponents_FWD_DEFINED__
typedef interface IComponents IComponents;

#endif 	/* __IComponents_FWD_DEFINED__ */


#ifndef __IComponentsOld_FWD_DEFINED__
#define __IComponentsOld_FWD_DEFINED__
typedef interface IComponentsOld IComponentsOld;

#endif 	/* __IComponentsOld_FWD_DEFINED__ */


#ifndef __ILocator_FWD_DEFINED__
#define __ILocator_FWD_DEFINED__
typedef interface ILocator ILocator;

#endif 	/* __ILocator_FWD_DEFINED__ */


#ifndef __IAnalogLocator_FWD_DEFINED__
#define __IAnalogLocator_FWD_DEFINED__
typedef interface IAnalogLocator IAnalogLocator;

#endif 	/* __IAnalogLocator_FWD_DEFINED__ */


#ifndef __IDigitalLocator_FWD_DEFINED__
#define __IDigitalLocator_FWD_DEFINED__
typedef interface IDigitalLocator IDigitalLocator;

#endif 	/* __IDigitalLocator_FWD_DEFINED__ */


#ifndef __IATSCLocator_FWD_DEFINED__
#define __IATSCLocator_FWD_DEFINED__
typedef interface IATSCLocator IATSCLocator;

#endif 	/* __IATSCLocator_FWD_DEFINED__ */


#ifndef __IATSCLocator2_FWD_DEFINED__
#define __IATSCLocator2_FWD_DEFINED__
typedef interface IATSCLocator2 IATSCLocator2;

#endif 	/* __IATSCLocator2_FWD_DEFINED__ */


#ifndef __IDigitalCableLocator_FWD_DEFINED__
#define __IDigitalCableLocator_FWD_DEFINED__
typedef interface IDigitalCableLocator IDigitalCableLocator;

#endif 	/* __IDigitalCableLocator_FWD_DEFINED__ */


#ifndef __IDVBTLocator_FWD_DEFINED__
#define __IDVBTLocator_FWD_DEFINED__
typedef interface IDVBTLocator IDVBTLocator;

#endif 	/* __IDVBTLocator_FWD_DEFINED__ */


#ifndef __IDVBTLocator2_FWD_DEFINED__
#define __IDVBTLocator2_FWD_DEFINED__
typedef interface IDVBTLocator2 IDVBTLocator2;

#endif 	/* __IDVBTLocator2_FWD_DEFINED__ */


#ifndef __IDVBSLocator_FWD_DEFINED__
#define __IDVBSLocator_FWD_DEFINED__
typedef interface IDVBSLocator IDVBSLocator;

#endif 	/* __IDVBSLocator_FWD_DEFINED__ */


#ifndef __IDVBSLocator2_FWD_DEFINED__
#define __IDVBSLocator2_FWD_DEFINED__
typedef interface IDVBSLocator2 IDVBSLocator2;

#endif 	/* __IDVBSLocator2_FWD_DEFINED__ */


#ifndef __IDVBCLocator_FWD_DEFINED__
#define __IDVBCLocator_FWD_DEFINED__
typedef interface IDVBCLocator IDVBCLocator;

#endif 	/* __IDVBCLocator_FWD_DEFINED__ */


#ifndef __IISDBSLocator_FWD_DEFINED__
#define __IISDBSLocator_FWD_DEFINED__
typedef interface IISDBSLocator IISDBSLocator;

#endif 	/* __IISDBSLocator_FWD_DEFINED__ */


#ifndef __IESOpenMmiEvent_FWD_DEFINED__
#define __IESOpenMmiEvent_FWD_DEFINED__
typedef interface IESOpenMmiEvent IESOpenMmiEvent;

#endif 	/* __IESOpenMmiEvent_FWD_DEFINED__ */


#ifndef __IESCloseMmiEvent_FWD_DEFINED__
#define __IESCloseMmiEvent_FWD_DEFINED__
typedef interface IESCloseMmiEvent IESCloseMmiEvent;

#endif 	/* __IESCloseMmiEvent_FWD_DEFINED__ */


#ifndef __IESValueUpdatedEvent_FWD_DEFINED__
#define __IESValueUpdatedEvent_FWD_DEFINED__
typedef interface IESValueUpdatedEvent IESValueUpdatedEvent;

#endif 	/* __IESValueUpdatedEvent_FWD_DEFINED__ */


#ifndef __IESRequestTunerEvent_FWD_DEFINED__
#define __IESRequestTunerEvent_FWD_DEFINED__
typedef interface IESRequestTunerEvent IESRequestTunerEvent;

#endif 	/* __IESRequestTunerEvent_FWD_DEFINED__ */


#ifndef __IESIsdbCasResponseEvent_FWD_DEFINED__
#define __IESIsdbCasResponseEvent_FWD_DEFINED__
typedef interface IESIsdbCasResponseEvent IESIsdbCasResponseEvent;

#endif 	/* __IESIsdbCasResponseEvent_FWD_DEFINED__ */


#ifndef __IGpnvsCommonBase_FWD_DEFINED__
#define __IGpnvsCommonBase_FWD_DEFINED__
typedef interface IGpnvsCommonBase IGpnvsCommonBase;

#endif 	/* __IGpnvsCommonBase_FWD_DEFINED__ */


#ifndef __IESEventFactory_FWD_DEFINED__
#define __IESEventFactory_FWD_DEFINED__
typedef interface IESEventFactory IESEventFactory;

#endif 	/* __IESEventFactory_FWD_DEFINED__ */


#ifndef __IESLicenseRenewalResultEvent_FWD_DEFINED__
#define __IESLicenseRenewalResultEvent_FWD_DEFINED__
typedef interface IESLicenseRenewalResultEvent IESLicenseRenewalResultEvent;

#endif 	/* __IESLicenseRenewalResultEvent_FWD_DEFINED__ */


#ifndef __IESFileExpiryDateEvent_FWD_DEFINED__
#define __IESFileExpiryDateEvent_FWD_DEFINED__
typedef interface IESFileExpiryDateEvent IESFileExpiryDateEvent;

#endif 	/* __IESFileExpiryDateEvent_FWD_DEFINED__ */


#ifndef __IESEventService_FWD_DEFINED__
#define __IESEventService_FWD_DEFINED__
typedef interface IESEventService IESEventService;

#endif 	/* __IESEventService_FWD_DEFINED__ */


#ifndef __IESEventServiceConfiguration_FWD_DEFINED__
#define __IESEventServiceConfiguration_FWD_DEFINED__
typedef interface IESEventServiceConfiguration IESEventServiceConfiguration;

#endif 	/* __IESEventServiceConfiguration_FWD_DEFINED__ */


#ifndef __IRegisterTuner_FWD_DEFINED__
#define __IRegisterTuner_FWD_DEFINED__
typedef interface IRegisterTuner IRegisterTuner;

#endif 	/* __IRegisterTuner_FWD_DEFINED__ */


#ifndef __IBDAComparable_FWD_DEFINED__
#define __IBDAComparable_FWD_DEFINED__
typedef interface IBDAComparable IBDAComparable;

#endif 	/* __IBDAComparable_FWD_DEFINED__ */


#ifndef __IPersistTuneXml_FWD_DEFINED__
#define __IPersistTuneXml_FWD_DEFINED__
typedef interface IPersistTuneXml IPersistTuneXml;

#endif 	/* __IPersistTuneXml_FWD_DEFINED__ */


#ifndef __IPersistTuneXmlUtility_FWD_DEFINED__
#define __IPersistTuneXmlUtility_FWD_DEFINED__
typedef interface IPersistTuneXmlUtility IPersistTuneXmlUtility;

#endif 	/* __IPersistTuneXmlUtility_FWD_DEFINED__ */


#ifndef __IPersistTuneXmlUtility2_FWD_DEFINED__
#define __IPersistTuneXmlUtility2_FWD_DEFINED__
typedef interface IPersistTuneXmlUtility2 IPersistTuneXmlUtility2;

#endif 	/* __IPersistTuneXmlUtility2_FWD_DEFINED__ */


#ifndef __IBDACreateTuneRequestEx_FWD_DEFINED__
#define __IBDACreateTuneRequestEx_FWD_DEFINED__
typedef interface IBDACreateTuneRequestEx IBDACreateTuneRequestEx;

#endif 	/* __IBDACreateTuneRequestEx_FWD_DEFINED__ */


#ifndef __SystemTuningSpaces_FWD_DEFINED__
#define __SystemTuningSpaces_FWD_DEFINED__

#ifdef __cplusplus
typedef class SystemTuningSpaces SystemTuningSpaces;
#else
typedef struct SystemTuningSpaces SystemTuningSpaces;
#endif /* __cplusplus */

#endif 	/* __SystemTuningSpaces_FWD_DEFINED__ */


#ifndef __TuningSpace_FWD_DEFINED__
#define __TuningSpace_FWD_DEFINED__

#ifdef __cplusplus
typedef class TuningSpace TuningSpace;
#else
typedef struct TuningSpace TuningSpace;
#endif /* __cplusplus */

#endif 	/* __TuningSpace_FWD_DEFINED__ */


#ifndef __ChannelIDTuningSpace_FWD_DEFINED__
#define __ChannelIDTuningSpace_FWD_DEFINED__

#ifdef __cplusplus
typedef class ChannelIDTuningSpace ChannelIDTuningSpace;
#else
typedef struct ChannelIDTuningSpace ChannelIDTuningSpace;
#endif /* __cplusplus */

#endif 	/* __ChannelIDTuningSpace_FWD_DEFINED__ */


#ifndef __ATSCTuningSpace_FWD_DEFINED__
#define __ATSCTuningSpace_FWD_DEFINED__

#ifdef __cplusplus
typedef class ATSCTuningSpace ATSCTuningSpace;
#else
typedef struct ATSCTuningSpace ATSCTuningSpace;
#endif /* __cplusplus */

#endif 	/* __ATSCTuningSpace_FWD_DEFINED__ */


#ifndef __DigitalCableTuningSpace_FWD_DEFINED__
#define __DigitalCableTuningSpace_FWD_DEFINED__

#ifdef __cplusplus
typedef class DigitalCableTuningSpace DigitalCableTuningSpace;
#else
typedef struct DigitalCableTuningSpace DigitalCableTuningSpace;
#endif /* __cplusplus */

#endif 	/* __DigitalCableTuningSpace_FWD_DEFINED__ */


#ifndef __AnalogRadioTuningSpace_FWD_DEFINED__
#define __AnalogRadioTuningSpace_FWD_DEFINED__

#ifdef __cplusplus
typedef class AnalogRadioTuningSpace AnalogRadioTuningSpace;
#else
typedef struct AnalogRadioTuningSpace AnalogRadioTuningSpace;
#endif /* __cplusplus */

#endif 	/* __AnalogRadioTuningSpace_FWD_DEFINED__ */


#ifndef __AuxInTuningSpace_FWD_DEFINED__
#define __AuxInTuningSpace_FWD_DEFINED__

#ifdef __cplusplus
typedef class AuxInTuningSpace AuxInTuningSpace;
#else
typedef struct AuxInTuningSpace AuxInTuningSpace;
#endif /* __cplusplus */

#endif 	/* __AuxInTuningSpace_FWD_DEFINED__ */


#ifndef __AnalogTVTuningSpace_FWD_DEFINED__
#define __AnalogTVTuningSpace_FWD_DEFINED__

#ifdef __cplusplus
typedef class AnalogTVTuningSpace AnalogTVTuningSpace;
#else
typedef struct AnalogTVTuningSpace AnalogTVTuningSpace;
#endif /* __cplusplus */

#endif 	/* __AnalogTVTuningSpace_FWD_DEFINED__ */


#ifndef __DVBTuningSpace_FWD_DEFINED__
#define __DVBTuningSpace_FWD_DEFINED__

#ifdef __cplusplus
typedef class DVBTuningSpace DVBTuningSpace;
#else
typedef struct DVBTuningSpace DVBTuningSpace;
#endif /* __cplusplus */

#endif 	/* __DVBTuningSpace_FWD_DEFINED__ */


#ifndef __DVBSTuningSpace_FWD_DEFINED__
#define __DVBSTuningSpace_FWD_DEFINED__

#ifdef __cplusplus
typedef class DVBSTuningSpace DVBSTuningSpace;
#else
typedef struct DVBSTuningSpace DVBSTuningSpace;
#endif /* __cplusplus */

#endif 	/* __DVBSTuningSpace_FWD_DEFINED__ */


#ifndef __ComponentTypes_FWD_DEFINED__
#define __ComponentTypes_FWD_DEFINED__

#ifdef __cplusplus
typedef class ComponentTypes ComponentTypes;
#else
typedef struct ComponentTypes ComponentTypes;
#endif /* __cplusplus */

#endif 	/* __ComponentTypes_FWD_DEFINED__ */


#ifndef __ComponentType_FWD_DEFINED__
#define __ComponentType_FWD_DEFINED__

#ifdef __cplusplus
typedef class ComponentType ComponentType;
#else
typedef struct ComponentType ComponentType;
#endif /* __cplusplus */

#endif 	/* __ComponentType_FWD_DEFINED__ */


#ifndef __LanguageComponentType_FWD_DEFINED__
#define __LanguageComponentType_FWD_DEFINED__

#ifdef __cplusplus
typedef class LanguageComponentType LanguageComponentType;
#else
typedef struct LanguageComponentType LanguageComponentType;
#endif /* __cplusplus */

#endif 	/* __LanguageComponentType_FWD_DEFINED__ */


#ifndef __MPEG2ComponentType_FWD_DEFINED__
#define __MPEG2ComponentType_FWD_DEFINED__

#ifdef __cplusplus
typedef class MPEG2ComponentType MPEG2ComponentType;
#else
typedef struct MPEG2ComponentType MPEG2ComponentType;
#endif /* __cplusplus */

#endif 	/* __MPEG2ComponentType_FWD_DEFINED__ */


#ifndef __ATSCComponentType_FWD_DEFINED__
#define __ATSCComponentType_FWD_DEFINED__

#ifdef __cplusplus
typedef class ATSCComponentType ATSCComponentType;
#else
typedef struct ATSCComponentType ATSCComponentType;
#endif /* __cplusplus */

#endif 	/* __ATSCComponentType_FWD_DEFINED__ */


#ifndef __Components_FWD_DEFINED__
#define __Components_FWD_DEFINED__

#ifdef __cplusplus
typedef class Components Components;
#else
typedef struct Components Components;
#endif /* __cplusplus */

#endif 	/* __Components_FWD_DEFINED__ */


#ifndef __Component_FWD_DEFINED__
#define __Component_FWD_DEFINED__

#ifdef __cplusplus
typedef class Component Component;
#else
typedef struct Component Component;
#endif /* __cplusplus */

#endif 	/* __Component_FWD_DEFINED__ */


#ifndef __MPEG2Component_FWD_DEFINED__
#define __MPEG2Component_FWD_DEFINED__

#ifdef __cplusplus
typedef class MPEG2Component MPEG2Component;
#else
typedef struct MPEG2Component MPEG2Component;
#endif /* __cplusplus */

#endif 	/* __MPEG2Component_FWD_DEFINED__ */


#ifndef __AnalogAudioComponentType_FWD_DEFINED__
#define __AnalogAudioComponentType_FWD_DEFINED__

#ifdef __cplusplus
typedef class AnalogAudioComponentType AnalogAudioComponentType;
#else
typedef struct AnalogAudioComponentType AnalogAudioComponentType;
#endif /* __cplusplus */

#endif 	/* __AnalogAudioComponentType_FWD_DEFINED__ */


#ifndef __TuneRequest_FWD_DEFINED__
#define __TuneRequest_FWD_DEFINED__

#ifdef __cplusplus
typedef class TuneRequest TuneRequest;
#else
typedef struct TuneRequest TuneRequest;
#endif /* __cplusplus */

#endif 	/* __TuneRequest_FWD_DEFINED__ */


#ifndef __ChannelIDTuneRequest_FWD_DEFINED__
#define __ChannelIDTuneRequest_FWD_DEFINED__

#ifdef __cplusplus
typedef class ChannelIDTuneRequest ChannelIDTuneRequest;
#else
typedef struct ChannelIDTuneRequest ChannelIDTuneRequest;
#endif /* __cplusplus */

#endif 	/* __ChannelIDTuneRequest_FWD_DEFINED__ */


#ifndef __ChannelTuneRequest_FWD_DEFINED__
#define __ChannelTuneRequest_FWD_DEFINED__

#ifdef __cplusplus
typedef class ChannelTuneRequest ChannelTuneRequest;
#else
typedef struct ChannelTuneRequest ChannelTuneRequest;
#endif /* __cplusplus */

#endif 	/* __ChannelTuneRequest_FWD_DEFINED__ */


#ifndef __ATSCChannelTuneRequest_FWD_DEFINED__
#define __ATSCChannelTuneRequest_FWD_DEFINED__

#ifdef __cplusplus
typedef class ATSCChannelTuneRequest ATSCChannelTuneRequest;
#else
typedef struct ATSCChannelTuneRequest ATSCChannelTuneRequest;
#endif /* __cplusplus */

#endif 	/* __ATSCChannelTuneRequest_FWD_DEFINED__ */


#ifndef __DigitalCableTuneRequest_FWD_DEFINED__
#define __DigitalCableTuneRequest_FWD_DEFINED__

#ifdef __cplusplus
typedef class DigitalCableTuneRequest DigitalCableTuneRequest;
#else
typedef struct DigitalCableTuneRequest DigitalCableTuneRequest;
#endif /* __cplusplus */

#endif 	/* __DigitalCableTuneRequest_FWD_DEFINED__ */


#ifndef __MPEG2TuneRequest_FWD_DEFINED__
#define __MPEG2TuneRequest_FWD_DEFINED__

#ifdef __cplusplus
typedef class MPEG2TuneRequest MPEG2TuneRequest;
#else
typedef struct MPEG2TuneRequest MPEG2TuneRequest;
#endif /* __cplusplus */

#endif 	/* __MPEG2TuneRequest_FWD_DEFINED__ */


#ifndef __MPEG2TuneRequestFactory_FWD_DEFINED__
#define __MPEG2TuneRequestFactory_FWD_DEFINED__

#ifdef __cplusplus
typedef class MPEG2TuneRequestFactory MPEG2TuneRequestFactory;
#else
typedef struct MPEG2TuneRequestFactory MPEG2TuneRequestFactory;
#endif /* __cplusplus */

#endif 	/* __MPEG2TuneRequestFactory_FWD_DEFINED__ */


#ifndef __Locator_FWD_DEFINED__
#define __Locator_FWD_DEFINED__

#ifdef __cplusplus
typedef class Locator Locator;
#else
typedef struct Locator Locator;
#endif /* __cplusplus */

#endif 	/* __Locator_FWD_DEFINED__ */


#ifndef __DigitalLocator_FWD_DEFINED__
#define __DigitalLocator_FWD_DEFINED__

#ifdef __cplusplus
typedef class DigitalLocator DigitalLocator;
#else
typedef struct DigitalLocator DigitalLocator;
#endif /* __cplusplus */

#endif 	/* __DigitalLocator_FWD_DEFINED__ */


#ifndef __AnalogLocator_FWD_DEFINED__
#define __AnalogLocator_FWD_DEFINED__

#ifdef __cplusplus
typedef class AnalogLocator AnalogLocator;
#else
typedef struct AnalogLocator AnalogLocator;
#endif /* __cplusplus */

#endif 	/* __AnalogLocator_FWD_DEFINED__ */


#ifndef __ATSCLocator_FWD_DEFINED__
#define __ATSCLocator_FWD_DEFINED__

#ifdef __cplusplus
typedef class ATSCLocator ATSCLocator;
#else
typedef struct ATSCLocator ATSCLocator;
#endif /* __cplusplus */

#endif 	/* __ATSCLocator_FWD_DEFINED__ */


#ifndef __DigitalCableLocator_FWD_DEFINED__
#define __DigitalCableLocator_FWD_DEFINED__

#ifdef __cplusplus
typedef class DigitalCableLocator DigitalCableLocator;
#else
typedef struct DigitalCableLocator DigitalCableLocator;
#endif /* __cplusplus */

#endif 	/* __DigitalCableLocator_FWD_DEFINED__ */


#ifndef __DVBTLocator_FWD_DEFINED__
#define __DVBTLocator_FWD_DEFINED__

#ifdef __cplusplus
typedef class DVBTLocator DVBTLocator;
#else
typedef struct DVBTLocator DVBTLocator;
#endif /* __cplusplus */

#endif 	/* __DVBTLocator_FWD_DEFINED__ */


#ifndef __DVBTLocator2_FWD_DEFINED__
#define __DVBTLocator2_FWD_DEFINED__

#ifdef __cplusplus
typedef class DVBTLocator2 DVBTLocator2;
#else
typedef struct DVBTLocator2 DVBTLocator2;
#endif /* __cplusplus */

#endif 	/* __DVBTLocator2_FWD_DEFINED__ */


#ifndef __DVBSLocator_FWD_DEFINED__
#define __DVBSLocator_FWD_DEFINED__

#ifdef __cplusplus
typedef class DVBSLocator DVBSLocator;
#else
typedef struct DVBSLocator DVBSLocator;
#endif /* __cplusplus */

#endif 	/* __DVBSLocator_FWD_DEFINED__ */


#ifndef __DVBCLocator_FWD_DEFINED__
#define __DVBCLocator_FWD_DEFINED__

#ifdef __cplusplus
typedef class DVBCLocator DVBCLocator;
#else
typedef struct DVBCLocator DVBCLocator;
#endif /* __cplusplus */

#endif 	/* __DVBCLocator_FWD_DEFINED__ */


#ifndef __ISDBSLocator_FWD_DEFINED__
#define __ISDBSLocator_FWD_DEFINED__

#ifdef __cplusplus
typedef class ISDBSLocator ISDBSLocator;
#else
typedef struct ISDBSLocator ISDBSLocator;
#endif /* __cplusplus */

#endif 	/* __ISDBSLocator_FWD_DEFINED__ */


#ifndef __DVBTuneRequest_FWD_DEFINED__
#define __DVBTuneRequest_FWD_DEFINED__

#ifdef __cplusplus
typedef class DVBTuneRequest DVBTuneRequest;
#else
typedef struct DVBTuneRequest DVBTuneRequest;
#endif /* __cplusplus */

#endif 	/* __DVBTuneRequest_FWD_DEFINED__ */


#ifndef __CreatePropBagOnRegKey_FWD_DEFINED__
#define __CreatePropBagOnRegKey_FWD_DEFINED__

#ifdef __cplusplus
typedef class CreatePropBagOnRegKey CreatePropBagOnRegKey;
#else
typedef struct CreatePropBagOnRegKey CreatePropBagOnRegKey;
#endif /* __cplusplus */

#endif 	/* __CreatePropBagOnRegKey_FWD_DEFINED__ */


#ifndef __BroadcastEventService_FWD_DEFINED__
#define __BroadcastEventService_FWD_DEFINED__

#ifdef __cplusplus
typedef class BroadcastEventService BroadcastEventService;
#else
typedef struct BroadcastEventService BroadcastEventService;
#endif /* __cplusplus */

#endif 	/* __BroadcastEventService_FWD_DEFINED__ */


#ifndef __TunerMarshaler_FWD_DEFINED__
#define __TunerMarshaler_FWD_DEFINED__

#ifdef __cplusplus
typedef class TunerMarshaler TunerMarshaler;
#else
typedef struct TunerMarshaler TunerMarshaler;
#endif /* __cplusplus */

#endif 	/* __TunerMarshaler_FWD_DEFINED__ */


#ifndef __PersistTuneXmlUtility_FWD_DEFINED__
#define __PersistTuneXmlUtility_FWD_DEFINED__

#ifdef __cplusplus
typedef class PersistTuneXmlUtility PersistTuneXmlUtility;
#else
typedef struct PersistTuneXmlUtility PersistTuneXmlUtility;
#endif /* __cplusplus */

#endif 	/* __PersistTuneXmlUtility_FWD_DEFINED__ */


#ifndef __ESEventService_FWD_DEFINED__
#define __ESEventService_FWD_DEFINED__

#ifdef __cplusplus
typedef class ESEventService ESEventService;
#else
typedef struct ESEventService ESEventService;
#endif /* __cplusplus */

#endif 	/* __ESEventService_FWD_DEFINED__ */


#ifndef __ESEventFactory_FWD_DEFINED__
#define __ESEventFactory_FWD_DEFINED__

#ifdef __cplusplus
typedef class ESEventFactory ESEventFactory;
#else
typedef struct ESEventFactory ESEventFactory;
#endif /* __cplusplus */

#endif 	/* __ESEventFactory_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "comcat.h"
#include "strmif.h"
#include "bdaiface.h"
#include "regbag.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_tuner_0000_0000 */
/* [local] */ 

//+-------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (C) Microsoft Corporation, 1999-2002.
//
//--------------------------------------------------------------------------
#pragma once
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)





























enum __MIDL___MIDL_itf_tuner_0000_0000_0001
    {
        DISPID_TUNER_TS_UNIQUENAME	= 1,
        DISPID_TUNER_TS_FRIENDLYNAME	= 2,
        DISPID_TUNER_TS_CLSID	= 3,
        DISPID_TUNER_TS_NETWORKTYPE	= 4,
        DISPID_TUNER_TS__NETWORKTYPE	= 5,
        DISPID_TUNER_TS_CREATETUNEREQUEST	= 6,
        DISPID_TUNER_TS_ENUMCATEGORYGUIDS	= 7,
        DISPID_TUNER_TS_ENUMDEVICEMONIKERS	= 8,
        DISPID_TUNER_TS_DEFAULTPREFERREDCOMPONENTTYPES	= 9,
        DISPID_TUNER_TS_FREQMAP	= 10,
        DISPID_TUNER_TS_DEFLOCATOR	= 11,
        DISPID_TUNER_TS_CLONE	= 12,
        DISPID_TUNER_TR_TUNINGSPACE	= 1,
        DISPID_TUNER_TR_COMPONENTS	= 2,
        DISPID_TUNER_TR_CLONE	= 3,
        DISPID_TUNER_TR_LOCATOR	= 4,
        DISPID_TUNER_CT_CATEGORY	= 1,
        DISPID_TUNER_CT_MEDIAMAJORTYPE	= 2,
        DISPID_TUNER_CT__MEDIAMAJORTYPE	= 3,
        DISPID_TUNER_CT_MEDIASUBTYPE	= 4,
        DISPID_TUNER_CT__MEDIASUBTYPE	= 5,
        DISPID_TUNER_CT_MEDIAFORMATTYPE	= 6,
        DISPID_TUNER_CT__MEDIAFORMATTYPE	= 7,
        DISPID_TUNER_CT_MEDIATYPE	= 8,
        DISPID_TUNER_CT_CLONE	= 9,
        DISPID_TUNER_LCT_LANGID	= 100,
        DISPID_TUNER_MP2CT_TYPE	= 200,
        DISPID_TUNER_ATSCCT_FLAGS	= 300,
        DISPID_TUNER_L_CARRFREQ	= 1,
        DISPID_TUNER_L_INNERFECMETHOD	= 2,
        DISPID_TUNER_L_INNERFECRATE	= 3,
        DISPID_TUNER_L_OUTERFECMETHOD	= 4,
        DISPID_TUNER_L_OUTERFECRATE	= 5,
        DISPID_TUNER_L_MOD	= 6,
        DISPID_TUNER_L_SYMRATE	= 7,
        DISPID_TUNER_L_CLONE	= 8,
        DISPID_TUNER_L_ATSC_PHYS_CHANNEL	= 201,
        DISPID_TUNER_L_ATSC_TSID	= 202,
        DISPID_TUNER_L_ATSC_MP2_PROGNO	= 203,
        DISPID_TUNER_L_DVBT_BANDWIDTH	= 301,
        DISPID_TUNER_L_DVBT_LPINNERFECMETHOD	= 302,
        DISPID_TUNER_L_DVBT_LPINNERFECRATE	= 303,
        DISPID_TUNER_L_DVBT_GUARDINTERVAL	= 304,
        DISPID_TUNER_L_DVBT_HALPHA	= 305,
        DISPID_TUNER_L_DVBT_TRANSMISSIONMODE	= 306,
        DISPID_TUNER_L_DVBT_INUSE	= 307,
        DISPID_TUNER_L_DVBT2_PHYSICALLAYERPIPEID	= 351,
        DISPID_TUNER_L_DVBS_POLARISATION	= 401,
        DISPID_TUNER_L_DVBS_WEST	= 402,
        DISPID_TUNER_L_DVBS_ORBITAL	= 403,
        DISPID_TUNER_L_DVBS_AZIMUTH	= 404,
        DISPID_TUNER_L_DVBS_ELEVATION	= 405,
        DISPID_TUNER_L_DVBS2_DISEQ_LNB_SOURCE	= 406,
        DISPID_TUNER_TS_DVBS2_LOW_OSC_FREQ_OVERRIDE	= 407,
        DISPID_TUNER_TS_DVBS2_HI_OSC_FREQ_OVERRIDE	= 408,
        DISPID_TUNER_TS_DVBS2_LNB_SWITCH_FREQ_OVERRIDE	= 409,
        DISPID_TUNER_TS_DVBS2_SPECTRAL_INVERSION_OVERRIDE	= 410,
        DISPID_TUNER_L_DVBS2_ROLLOFF	= 411,
        DISPID_TUNER_L_DVBS2_PILOT	= 412,
        DISPID_TUNER_L_ANALOG_STANDARD	= 601,
        DISPID_TUNER_L_DTV_O_MAJOR_CHANNEL	= 701,
        DISPID_TUNER_C_TYPE	= 1,
        DISPID_TUNER_C_STATUS	= 2,
        DISPID_TUNER_C_LANGID	= 3,
        DISPID_TUNER_C_DESCRIPTION	= 4,
        DISPID_TUNER_C_CLONE	= 5,
        DISPID_TUNER_C_MP2_PID	= 101,
        DISPID_TUNER_C_MP2_PCRPID	= 102,
        DISPID_TUNER_C_MP2_PROGNO	= 103,
        DISPID_TUNER_C_ANALOG_AUDIO	= 201,
        DISPID_TUNER_TS_DVB_SYSTEMTYPE	= 101,
        DISPID_TUNER_TS_DVB2_NETWORK_ID	= 102,
        DISPID_TUNER_TS_DVBS_LOW_OSC_FREQ	= 1001,
        DISPID_TUNER_TS_DVBS_HI_OSC_FREQ	= 1002,
        DISPID_TUNER_TS_DVBS_LNB_SWITCH_FREQ	= 1003,
        DISPID_TUNER_TS_DVBS_INPUT_RANGE	= 1004,
        DISPID_TUNER_TS_DVBS_SPECTRAL_INVERSION	= 1005,
        DISPID_TUNER_TS_AR_MINFREQUENCY	= 101,
        DISPID_TUNER_TS_AR_MAXFREQUENCY	= 102,
        DISPID_TUNER_TS_AR_STEP	= 103,
        DISPID_TUNER_TS_AR_COUNTRYCODE	= 104,
        DISPID_TUNER_TS_AUX_COUNTRYCODE	= 101,
        DISPID_TUNER_TS_ATV_MINCHANNEL	= 101,
        DISPID_TUNER_TS_ATV_MAXCHANNEL	= 102,
        DISPID_TUNER_TS_ATV_INPUTTYPE	= 103,
        DISPID_TUNER_TS_ATV_COUNTRYCODE	= 104,
        DISPID_TUNER_TS_ATSC_MINMINORCHANNEL	= 201,
        DISPID_TUNER_TS_ATSC_MAXMINORCHANNEL	= 202,
        DISPID_TUNER_TS_ATSC_MINPHYSCHANNEL	= 203,
        DISPID_TUNER_TS_ATSC_MAXPHYSCHANNEL	= 204,
        DISPID_TUNER_TS_DC_MINMAJORCHANNEL	= 301,
        DISPID_TUNER_TS_DC_MAXMAJORCHANNEL	= 302,
        DISPID_TUNER_TS_DC_MINSOURCEID	= 303,
        DISPID_TUNER_TS_DC_MAXSOURCEID	= 304,
        DISPID_CHTUNER_ATVAC_CHANNEL	= 101,
        DISPID_CHTUNER_ATVDC_SYSTEM	= 101,
        DISPID_CHTUNER_ATVDC_CONTENT	= 102,
        DISPID_CHTUNER_CIDTR_CHANNELID	= 101,
        DISPID_CHTUNER_CTR_CHANNEL	= 101,
        DISPID_CHTUNER_ACTR_MINOR_CHANNEL	= 201,
        DISPID_CHTUNER_DCTR_MAJOR_CHANNEL	= 301,
        DISPID_CHTUNER_DCTR_SRCID	= 302,
        DISPID_DVBTUNER_DVBC_ATTRIBUTESVALID	= 101,
        DISPID_DVBTUNER_DVBC_PID	= 102,
        DISPID_DVBTUNER_DVBC_TAG	= 103,
        DISPID_DVBTUNER_DVBC_COMPONENTTYPE	= 104,
        DISPID_DVBTUNER_ONID	= 101,
        DISPID_DVBTUNER_TSID	= 102,
        DISPID_DVBTUNER_SID	= 103,
        DISPID_MP2TUNER_TSID	= 101,
        DISPID_MP2TUNER_PROGNO	= 102,
        DISPID_MP2TUNERFACTORY_CREATETUNEREQUEST	= 1
    } ;


extern RPC_IF_HANDLE __MIDL_itf_tuner_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_tuner_0000_0000_v0_0_s_ifspec;

#ifndef __ITuningSpaces_INTERFACE_DEFINED__
#define __ITuningSpaces_INTERFACE_DEFINED__

/* interface ITuningSpaces */
/* [unique][helpstring][nonextensible][proxy][oleautomation][dual][uuid][object] */ 


EXTERN_C const IID IID_ITuningSpaces;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("901284E4-33FE-4b69-8D63-634A596F3756")
    ITuningSpaces : public IDispatch
    {
    public:
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *Count) = 0;
        
        virtual /* [restricted][hidden][id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumVARIANT **NewEnum) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT varIndex,
            /* [retval][out] */ __RPC__deref_out_opt ITuningSpace **TuningSpace) = 0;
        
        virtual /* [helpstring][restricted][hidden][propget] */ HRESULT STDMETHODCALLTYPE get_EnumTuningSpaces( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumTuningSpaces **NewEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITuningSpacesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITuningSpaces * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITuningSpaces * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITuningSpaces * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITuningSpaces * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITuningSpaces * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITuningSpaces * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITuningSpaces * This,
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
        
        DECLSPEC_XFGVIRT(ITuningSpaces, get_Count)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in ITuningSpaces * This,
            /* [retval][out] */ __RPC__out long *Count);
        
        DECLSPEC_XFGVIRT(ITuningSpaces, get__NewEnum)
        /* [restricted][hidden][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in ITuningSpaces * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumVARIANT **NewEnum);
        
        DECLSPEC_XFGVIRT(ITuningSpaces, get_Item)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in ITuningSpaces * This,
            /* [in] */ VARIANT varIndex,
            /* [retval][out] */ __RPC__deref_out_opt ITuningSpace **TuningSpace);
        
        DECLSPEC_XFGVIRT(ITuningSpaces, get_EnumTuningSpaces)
        /* [helpstring][restricted][hidden][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EnumTuningSpaces )( 
            __RPC__in ITuningSpaces * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumTuningSpaces **NewEnum);
        
        END_INTERFACE
    } ITuningSpacesVtbl;

    interface ITuningSpaces
    {
        CONST_VTBL struct ITuningSpacesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITuningSpaces_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITuningSpaces_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITuningSpaces_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITuningSpaces_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITuningSpaces_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITuningSpaces_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITuningSpaces_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITuningSpaces_get_Count(This,Count)	\
    ( (This)->lpVtbl -> get_Count(This,Count) ) 

#define ITuningSpaces_get__NewEnum(This,NewEnum)	\
    ( (This)->lpVtbl -> get__NewEnum(This,NewEnum) ) 

#define ITuningSpaces_get_Item(This,varIndex,TuningSpace)	\
    ( (This)->lpVtbl -> get_Item(This,varIndex,TuningSpace) ) 

#define ITuningSpaces_get_EnumTuningSpaces(This,NewEnum)	\
    ( (This)->lpVtbl -> get_EnumTuningSpaces(This,NewEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITuningSpaces_INTERFACE_DEFINED__ */


#ifndef __ITuningSpaceContainer_INTERFACE_DEFINED__
#define __ITuningSpaceContainer_INTERFACE_DEFINED__

/* interface ITuningSpaceContainer */
/* [unique][helpstring][nonextensible][hidden][proxy][oleautomation][dual][uuid][object] */ 


EXTERN_C const IID IID_ITuningSpaceContainer;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5B692E84-E2F1-11d2-9493-00C04F72D980")
    ITuningSpaceContainer : public IDispatch
    {
    public:
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *Count) = 0;
        
        virtual /* [restricted][hidden][id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumVARIANT **NewEnum) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT varIndex,
            /* [retval][out] */ __RPC__deref_out_opt ITuningSpace **TuningSpace) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Item( 
            /* [in] */ VARIANT varIndex,
            /* [in] */ __RPC__in_opt ITuningSpace *TuningSpace) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE TuningSpacesForCLSID( 
            /* [in] */ __RPC__in BSTR SpaceCLSID,
            /* [retval][out] */ __RPC__deref_out_opt ITuningSpaces **NewColl) = 0;
        
        virtual /* [helpstring][restricted][hidden] */ HRESULT STDMETHODCALLTYPE _TuningSpacesForCLSID( 
            /* [in] */ __RPC__in REFCLSID SpaceCLSID,
            /* [retval][out] */ __RPC__deref_out_opt ITuningSpaces **NewColl) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE TuningSpacesForName( 
            /* [in] */ __RPC__in BSTR Name,
            /* [retval][out] */ __RPC__deref_out_opt ITuningSpaces **NewColl) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE FindID( 
            /* [in] */ __RPC__in_opt ITuningSpace *TuningSpace,
            /* [retval][out] */ __RPC__out long *ID) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Add( 
            /* [in] */ __RPC__in_opt ITuningSpace *TuningSpace,
            /* [retval][out] */ __RPC__out VARIANT *NewIndex) = 0;
        
        virtual /* [helpstring][restricted][hidden][propget] */ HRESULT STDMETHODCALLTYPE get_EnumTuningSpaces( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumTuningSpaces **ppEnum) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Remove( 
            /* [in] */ VARIANT Index) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_MaxCount( 
            /* [retval][out] */ __RPC__out long *MaxCount) = 0;
        
        virtual /* [helpstring][restricted][hidden][propput] */ HRESULT STDMETHODCALLTYPE put_MaxCount( 
            /* [in] */ long MaxCount) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITuningSpaceContainerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITuningSpaceContainer * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITuningSpaceContainer * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITuningSpaceContainer * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITuningSpaceContainer * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITuningSpaceContainer * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITuningSpaceContainer * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITuningSpaceContainer * This,
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
        
        DECLSPEC_XFGVIRT(ITuningSpaceContainer, get_Count)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in ITuningSpaceContainer * This,
            /* [retval][out] */ __RPC__out long *Count);
        
        DECLSPEC_XFGVIRT(ITuningSpaceContainer, get__NewEnum)
        /* [restricted][hidden][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in ITuningSpaceContainer * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumVARIANT **NewEnum);
        
        DECLSPEC_XFGVIRT(ITuningSpaceContainer, get_Item)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in ITuningSpaceContainer * This,
            /* [in] */ VARIANT varIndex,
            /* [retval][out] */ __RPC__deref_out_opt ITuningSpace **TuningSpace);
        
        DECLSPEC_XFGVIRT(ITuningSpaceContainer, put_Item)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Item )( 
            __RPC__in ITuningSpaceContainer * This,
            /* [in] */ VARIANT varIndex,
            /* [in] */ __RPC__in_opt ITuningSpace *TuningSpace);
        
        DECLSPEC_XFGVIRT(ITuningSpaceContainer, TuningSpacesForCLSID)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *TuningSpacesForCLSID )( 
            __RPC__in ITuningSpaceContainer * This,
            /* [in] */ __RPC__in BSTR SpaceCLSID,
            /* [retval][out] */ __RPC__deref_out_opt ITuningSpaces **NewColl);
        
        DECLSPEC_XFGVIRT(ITuningSpaceContainer, _TuningSpacesForCLSID)
        /* [helpstring][restricted][hidden] */ HRESULT ( STDMETHODCALLTYPE *_TuningSpacesForCLSID )( 
            __RPC__in ITuningSpaceContainer * This,
            /* [in] */ __RPC__in REFCLSID SpaceCLSID,
            /* [retval][out] */ __RPC__deref_out_opt ITuningSpaces **NewColl);
        
        DECLSPEC_XFGVIRT(ITuningSpaceContainer, TuningSpacesForName)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *TuningSpacesForName )( 
            __RPC__in ITuningSpaceContainer * This,
            /* [in] */ __RPC__in BSTR Name,
            /* [retval][out] */ __RPC__deref_out_opt ITuningSpaces **NewColl);
        
        DECLSPEC_XFGVIRT(ITuningSpaceContainer, FindID)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *FindID )( 
            __RPC__in ITuningSpaceContainer * This,
            /* [in] */ __RPC__in_opt ITuningSpace *TuningSpace,
            /* [retval][out] */ __RPC__out long *ID);
        
        DECLSPEC_XFGVIRT(ITuningSpaceContainer, Add)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Add )( 
            __RPC__in ITuningSpaceContainer * This,
            /* [in] */ __RPC__in_opt ITuningSpace *TuningSpace,
            /* [retval][out] */ __RPC__out VARIANT *NewIndex);
        
        DECLSPEC_XFGVIRT(ITuningSpaceContainer, get_EnumTuningSpaces)
        /* [helpstring][restricted][hidden][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EnumTuningSpaces )( 
            __RPC__in ITuningSpaceContainer * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumTuningSpaces **ppEnum);
        
        DECLSPEC_XFGVIRT(ITuningSpaceContainer, Remove)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Remove )( 
            __RPC__in ITuningSpaceContainer * This,
            /* [in] */ VARIANT Index);
        
        DECLSPEC_XFGVIRT(ITuningSpaceContainer, get_MaxCount)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MaxCount )( 
            __RPC__in ITuningSpaceContainer * This,
            /* [retval][out] */ __RPC__out long *MaxCount);
        
        DECLSPEC_XFGVIRT(ITuningSpaceContainer, put_MaxCount)
        /* [helpstring][restricted][hidden][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MaxCount )( 
            __RPC__in ITuningSpaceContainer * This,
            /* [in] */ long MaxCount);
        
        END_INTERFACE
    } ITuningSpaceContainerVtbl;

    interface ITuningSpaceContainer
    {
        CONST_VTBL struct ITuningSpaceContainerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITuningSpaceContainer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITuningSpaceContainer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITuningSpaceContainer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITuningSpaceContainer_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITuningSpaceContainer_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITuningSpaceContainer_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITuningSpaceContainer_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITuningSpaceContainer_get_Count(This,Count)	\
    ( (This)->lpVtbl -> get_Count(This,Count) ) 

#define ITuningSpaceContainer_get__NewEnum(This,NewEnum)	\
    ( (This)->lpVtbl -> get__NewEnum(This,NewEnum) ) 

#define ITuningSpaceContainer_get_Item(This,varIndex,TuningSpace)	\
    ( (This)->lpVtbl -> get_Item(This,varIndex,TuningSpace) ) 

#define ITuningSpaceContainer_put_Item(This,varIndex,TuningSpace)	\
    ( (This)->lpVtbl -> put_Item(This,varIndex,TuningSpace) ) 

#define ITuningSpaceContainer_TuningSpacesForCLSID(This,SpaceCLSID,NewColl)	\
    ( (This)->lpVtbl -> TuningSpacesForCLSID(This,SpaceCLSID,NewColl) ) 

#define ITuningSpaceContainer__TuningSpacesForCLSID(This,SpaceCLSID,NewColl)	\
    ( (This)->lpVtbl -> _TuningSpacesForCLSID(This,SpaceCLSID,NewColl) ) 

#define ITuningSpaceContainer_TuningSpacesForName(This,Name,NewColl)	\
    ( (This)->lpVtbl -> TuningSpacesForName(This,Name,NewColl) ) 

#define ITuningSpaceContainer_FindID(This,TuningSpace,ID)	\
    ( (This)->lpVtbl -> FindID(This,TuningSpace,ID) ) 

#define ITuningSpaceContainer_Add(This,TuningSpace,NewIndex)	\
    ( (This)->lpVtbl -> Add(This,TuningSpace,NewIndex) ) 

#define ITuningSpaceContainer_get_EnumTuningSpaces(This,ppEnum)	\
    ( (This)->lpVtbl -> get_EnumTuningSpaces(This,ppEnum) ) 

#define ITuningSpaceContainer_Remove(This,Index)	\
    ( (This)->lpVtbl -> Remove(This,Index) ) 

#define ITuningSpaceContainer_get_MaxCount(This,MaxCount)	\
    ( (This)->lpVtbl -> get_MaxCount(This,MaxCount) ) 

#define ITuningSpaceContainer_put_MaxCount(This,MaxCount)	\
    ( (This)->lpVtbl -> put_MaxCount(This,MaxCount) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITuningSpaceContainer_INTERFACE_DEFINED__ */


#ifndef __ITuningSpace_INTERFACE_DEFINED__
#define __ITuningSpace_INTERFACE_DEFINED__

/* interface ITuningSpace */
/* [unique][helpstring][nonextensible][proxy][oleautomation][dual][uuid][object] */ 


EXTERN_C const IID IID_ITuningSpace;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("061C6E30-E622-11d2-9493-00C04F72D980")
    ITuningSpace : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_UniqueName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Name) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_UniqueName( 
            /* [in] */ __RPC__in BSTR Name) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_FriendlyName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Name) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_FriendlyName( 
            /* [in] */ __RPC__in BSTR Name) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CLSID( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *SpaceCLSID) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_NetworkType( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *NetworkTypeGuid) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_NetworkType( 
            /* [in] */ __RPC__in BSTR NetworkTypeGuid) = 0;
        
        virtual /* [restricted][hidden][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get__NetworkType( 
            /* [retval][out] */ __RPC__out GUID *NetworkTypeGuid) = 0;
        
        virtual /* [restricted][hidden][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put__NetworkType( 
            /* [in] */ __RPC__in REFCLSID NetworkTypeGuid) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateTuneRequest( 
            /* [retval][out] */ __RPC__deref_out_opt ITuneRequest **TuneRequest) = 0;
        
        virtual /* [restricted][hidden][id] */ HRESULT STDMETHODCALLTYPE EnumCategoryGUIDs( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumGUID **ppEnum) = 0;
        
        virtual /* [restricted][hidden][id] */ HRESULT STDMETHODCALLTYPE EnumDeviceMonikers( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumMoniker **ppEnum) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_DefaultPreferredComponentTypes( 
            /* [retval][out] */ __RPC__deref_out_opt IComponentTypes **ComponentTypes) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_DefaultPreferredComponentTypes( 
            /* [in] */ __RPC__in_opt IComponentTypes *NewComponentTypes) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_FrequencyMapping( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pMapping) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_FrequencyMapping( 
            __RPC__in BSTR Mapping) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_DefaultLocator( 
            /* [retval][out] */ __RPC__deref_out_opt ILocator **LocatorVal) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_DefaultLocator( 
            /* [in] */ __RPC__in_opt ILocator *LocatorVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt ITuningSpace **NewTS) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITuningSpaceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITuningSpace * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITuningSpace * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITuningSpace * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITuningSpace * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITuningSpace * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITuningSpace * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITuningSpace * This,
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
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_UniqueName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UniqueName )( 
            __RPC__in ITuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Name);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_UniqueName)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_UniqueName )( 
            __RPC__in ITuningSpace * This,
            /* [in] */ __RPC__in BSTR Name);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_FriendlyName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FriendlyName )( 
            __RPC__in ITuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Name);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_FriendlyName)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_FriendlyName )( 
            __RPC__in ITuningSpace * This,
            /* [in] */ __RPC__in BSTR Name);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_CLSID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CLSID )( 
            __RPC__in ITuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *SpaceCLSID);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_NetworkType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NetworkType )( 
            __RPC__in ITuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *NetworkTypeGuid);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_NetworkType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_NetworkType )( 
            __RPC__in ITuningSpace * This,
            /* [in] */ __RPC__in BSTR NetworkTypeGuid);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get__NetworkType)
        /* [restricted][hidden][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NetworkType )( 
            __RPC__in ITuningSpace * This,
            /* [retval][out] */ __RPC__out GUID *NetworkTypeGuid);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put__NetworkType)
        /* [restricted][hidden][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put__NetworkType )( 
            __RPC__in ITuningSpace * This,
            /* [in] */ __RPC__in REFCLSID NetworkTypeGuid);
        
        DECLSPEC_XFGVIRT(ITuningSpace, CreateTuneRequest)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateTuneRequest )( 
            __RPC__in ITuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt ITuneRequest **TuneRequest);
        
        DECLSPEC_XFGVIRT(ITuningSpace, EnumCategoryGUIDs)
        /* [restricted][hidden][id] */ HRESULT ( STDMETHODCALLTYPE *EnumCategoryGUIDs )( 
            __RPC__in ITuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumGUID **ppEnum);
        
        DECLSPEC_XFGVIRT(ITuningSpace, EnumDeviceMonikers)
        /* [restricted][hidden][id] */ HRESULT ( STDMETHODCALLTYPE *EnumDeviceMonikers )( 
            __RPC__in ITuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumMoniker **ppEnum);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_DefaultPreferredComponentTypes)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DefaultPreferredComponentTypes )( 
            __RPC__in ITuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt IComponentTypes **ComponentTypes);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_DefaultPreferredComponentTypes)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DefaultPreferredComponentTypes )( 
            __RPC__in ITuningSpace * This,
            /* [in] */ __RPC__in_opt IComponentTypes *NewComponentTypes);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_FrequencyMapping)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FrequencyMapping )( 
            __RPC__in ITuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pMapping);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_FrequencyMapping)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_FrequencyMapping )( 
            __RPC__in ITuningSpace * This,
            __RPC__in BSTR Mapping);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_DefaultLocator)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DefaultLocator )( 
            __RPC__in ITuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt ILocator **LocatorVal);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_DefaultLocator)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DefaultLocator )( 
            __RPC__in ITuningSpace * This,
            /* [in] */ __RPC__in_opt ILocator *LocatorVal);
        
        DECLSPEC_XFGVIRT(ITuningSpace, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in ITuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt ITuningSpace **NewTS);
        
        END_INTERFACE
    } ITuningSpaceVtbl;

    interface ITuningSpace
    {
        CONST_VTBL struct ITuningSpaceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITuningSpace_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITuningSpace_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITuningSpace_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITuningSpace_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITuningSpace_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITuningSpace_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITuningSpace_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITuningSpace_get_UniqueName(This,Name)	\
    ( (This)->lpVtbl -> get_UniqueName(This,Name) ) 

#define ITuningSpace_put_UniqueName(This,Name)	\
    ( (This)->lpVtbl -> put_UniqueName(This,Name) ) 

#define ITuningSpace_get_FriendlyName(This,Name)	\
    ( (This)->lpVtbl -> get_FriendlyName(This,Name) ) 

#define ITuningSpace_put_FriendlyName(This,Name)	\
    ( (This)->lpVtbl -> put_FriendlyName(This,Name) ) 

#define ITuningSpace_get_CLSID(This,SpaceCLSID)	\
    ( (This)->lpVtbl -> get_CLSID(This,SpaceCLSID) ) 

#define ITuningSpace_get_NetworkType(This,NetworkTypeGuid)	\
    ( (This)->lpVtbl -> get_NetworkType(This,NetworkTypeGuid) ) 

#define ITuningSpace_put_NetworkType(This,NetworkTypeGuid)	\
    ( (This)->lpVtbl -> put_NetworkType(This,NetworkTypeGuid) ) 

#define ITuningSpace_get__NetworkType(This,NetworkTypeGuid)	\
    ( (This)->lpVtbl -> get__NetworkType(This,NetworkTypeGuid) ) 

#define ITuningSpace_put__NetworkType(This,NetworkTypeGuid)	\
    ( (This)->lpVtbl -> put__NetworkType(This,NetworkTypeGuid) ) 

#define ITuningSpace_CreateTuneRequest(This,TuneRequest)	\
    ( (This)->lpVtbl -> CreateTuneRequest(This,TuneRequest) ) 

#define ITuningSpace_EnumCategoryGUIDs(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumCategoryGUIDs(This,ppEnum) ) 

#define ITuningSpace_EnumDeviceMonikers(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumDeviceMonikers(This,ppEnum) ) 

#define ITuningSpace_get_DefaultPreferredComponentTypes(This,ComponentTypes)	\
    ( (This)->lpVtbl -> get_DefaultPreferredComponentTypes(This,ComponentTypes) ) 

#define ITuningSpace_put_DefaultPreferredComponentTypes(This,NewComponentTypes)	\
    ( (This)->lpVtbl -> put_DefaultPreferredComponentTypes(This,NewComponentTypes) ) 

#define ITuningSpace_get_FrequencyMapping(This,pMapping)	\
    ( (This)->lpVtbl -> get_FrequencyMapping(This,pMapping) ) 

#define ITuningSpace_put_FrequencyMapping(This,Mapping)	\
    ( (This)->lpVtbl -> put_FrequencyMapping(This,Mapping) ) 

#define ITuningSpace_get_DefaultLocator(This,LocatorVal)	\
    ( (This)->lpVtbl -> get_DefaultLocator(This,LocatorVal) ) 

#define ITuningSpace_put_DefaultLocator(This,LocatorVal)	\
    ( (This)->lpVtbl -> put_DefaultLocator(This,LocatorVal) ) 

#define ITuningSpace_Clone(This,NewTS)	\
    ( (This)->lpVtbl -> Clone(This,NewTS) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITuningSpace_INTERFACE_DEFINED__ */


#ifndef __IEnumTuningSpaces_INTERFACE_DEFINED__
#define __IEnumTuningSpaces_INTERFACE_DEFINED__

/* interface IEnumTuningSpaces */
/* [unique][uuid][object][restricted][hidden] */ 


EXTERN_C const IID IID_IEnumTuningSpaces;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8B8EB248-FC2B-11d2-9D8C-00C04F72D980")
    IEnumTuningSpaces : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG celt,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) ITuningSpace **rgelt,
            /* [out] */ __RPC__out ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ __RPC__deref_out_opt IEnumTuningSpaces **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumTuningSpacesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumTuningSpaces * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumTuningSpaces * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumTuningSpaces * This);
        
        DECLSPEC_XFGVIRT(IEnumTuningSpaces, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IEnumTuningSpaces * This,
            /* [in] */ ULONG celt,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) ITuningSpace **rgelt,
            /* [out] */ __RPC__out ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IEnumTuningSpaces, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumTuningSpaces * This,
            /* [in] */ ULONG celt);
        
        DECLSPEC_XFGVIRT(IEnumTuningSpaces, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumTuningSpaces * This);
        
        DECLSPEC_XFGVIRT(IEnumTuningSpaces, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumTuningSpaces * This,
            /* [out] */ __RPC__deref_out_opt IEnumTuningSpaces **ppEnum);
        
        END_INTERFACE
    } IEnumTuningSpacesVtbl;

    interface IEnumTuningSpaces
    {
        CONST_VTBL struct IEnumTuningSpacesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumTuningSpaces_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumTuningSpaces_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumTuningSpaces_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumTuningSpaces_Next(This,celt,rgelt,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,rgelt,pceltFetched) ) 

#define IEnumTuningSpaces_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IEnumTuningSpaces_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumTuningSpaces_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumTuningSpaces_INTERFACE_DEFINED__ */


#ifndef __IDVBTuningSpace_INTERFACE_DEFINED__
#define __IDVBTuningSpace_INTERFACE_DEFINED__

/* interface IDVBTuningSpace */
/* [unique][uuid][nonextensible][proxy][oleautomation][dual][hidden][object] */ 


EXTERN_C const IID IID_IDVBTuningSpace;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("ADA0B268-3B19-4e5b-ACC4-49F852BE13BA")
    IDVBTuningSpace : public ITuningSpace
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SystemType( 
            /* [retval][out] */ __RPC__out DVBSystemType *SysType) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_SystemType( 
            /* [in] */ DVBSystemType SysType) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDVBTuningSpaceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDVBTuningSpace * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDVBTuningSpace * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDVBTuningSpace * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IDVBTuningSpace * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IDVBTuningSpace * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IDVBTuningSpace * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IDVBTuningSpace * This,
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
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_UniqueName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UniqueName )( 
            __RPC__in IDVBTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Name);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_UniqueName)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_UniqueName )( 
            __RPC__in IDVBTuningSpace * This,
            /* [in] */ __RPC__in BSTR Name);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_FriendlyName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FriendlyName )( 
            __RPC__in IDVBTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Name);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_FriendlyName)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_FriendlyName )( 
            __RPC__in IDVBTuningSpace * This,
            /* [in] */ __RPC__in BSTR Name);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_CLSID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CLSID )( 
            __RPC__in IDVBTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *SpaceCLSID);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_NetworkType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NetworkType )( 
            __RPC__in IDVBTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *NetworkTypeGuid);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_NetworkType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_NetworkType )( 
            __RPC__in IDVBTuningSpace * This,
            /* [in] */ __RPC__in BSTR NetworkTypeGuid);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get__NetworkType)
        /* [restricted][hidden][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NetworkType )( 
            __RPC__in IDVBTuningSpace * This,
            /* [retval][out] */ __RPC__out GUID *NetworkTypeGuid);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put__NetworkType)
        /* [restricted][hidden][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put__NetworkType )( 
            __RPC__in IDVBTuningSpace * This,
            /* [in] */ __RPC__in REFCLSID NetworkTypeGuid);
        
        DECLSPEC_XFGVIRT(ITuningSpace, CreateTuneRequest)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateTuneRequest )( 
            __RPC__in IDVBTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt ITuneRequest **TuneRequest);
        
        DECLSPEC_XFGVIRT(ITuningSpace, EnumCategoryGUIDs)
        /* [restricted][hidden][id] */ HRESULT ( STDMETHODCALLTYPE *EnumCategoryGUIDs )( 
            __RPC__in IDVBTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumGUID **ppEnum);
        
        DECLSPEC_XFGVIRT(ITuningSpace, EnumDeviceMonikers)
        /* [restricted][hidden][id] */ HRESULT ( STDMETHODCALLTYPE *EnumDeviceMonikers )( 
            __RPC__in IDVBTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumMoniker **ppEnum);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_DefaultPreferredComponentTypes)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DefaultPreferredComponentTypes )( 
            __RPC__in IDVBTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt IComponentTypes **ComponentTypes);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_DefaultPreferredComponentTypes)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DefaultPreferredComponentTypes )( 
            __RPC__in IDVBTuningSpace * This,
            /* [in] */ __RPC__in_opt IComponentTypes *NewComponentTypes);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_FrequencyMapping)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FrequencyMapping )( 
            __RPC__in IDVBTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pMapping);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_FrequencyMapping)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_FrequencyMapping )( 
            __RPC__in IDVBTuningSpace * This,
            __RPC__in BSTR Mapping);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_DefaultLocator)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DefaultLocator )( 
            __RPC__in IDVBTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt ILocator **LocatorVal);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_DefaultLocator)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DefaultLocator )( 
            __RPC__in IDVBTuningSpace * This,
            /* [in] */ __RPC__in_opt ILocator *LocatorVal);
        
        DECLSPEC_XFGVIRT(ITuningSpace, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IDVBTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt ITuningSpace **NewTS);
        
        DECLSPEC_XFGVIRT(IDVBTuningSpace, get_SystemType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SystemType )( 
            __RPC__in IDVBTuningSpace * This,
            /* [retval][out] */ __RPC__out DVBSystemType *SysType);
        
        DECLSPEC_XFGVIRT(IDVBTuningSpace, put_SystemType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SystemType )( 
            __RPC__in IDVBTuningSpace * This,
            /* [in] */ DVBSystemType SysType);
        
        END_INTERFACE
    } IDVBTuningSpaceVtbl;

    interface IDVBTuningSpace
    {
        CONST_VTBL struct IDVBTuningSpaceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDVBTuningSpace_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDVBTuningSpace_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDVBTuningSpace_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDVBTuningSpace_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IDVBTuningSpace_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IDVBTuningSpace_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IDVBTuningSpace_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IDVBTuningSpace_get_UniqueName(This,Name)	\
    ( (This)->lpVtbl -> get_UniqueName(This,Name) ) 

#define IDVBTuningSpace_put_UniqueName(This,Name)	\
    ( (This)->lpVtbl -> put_UniqueName(This,Name) ) 

#define IDVBTuningSpace_get_FriendlyName(This,Name)	\
    ( (This)->lpVtbl -> get_FriendlyName(This,Name) ) 

#define IDVBTuningSpace_put_FriendlyName(This,Name)	\
    ( (This)->lpVtbl -> put_FriendlyName(This,Name) ) 

#define IDVBTuningSpace_get_CLSID(This,SpaceCLSID)	\
    ( (This)->lpVtbl -> get_CLSID(This,SpaceCLSID) ) 

#define IDVBTuningSpace_get_NetworkType(This,NetworkTypeGuid)	\
    ( (This)->lpVtbl -> get_NetworkType(This,NetworkTypeGuid) ) 

#define IDVBTuningSpace_put_NetworkType(This,NetworkTypeGuid)	\
    ( (This)->lpVtbl -> put_NetworkType(This,NetworkTypeGuid) ) 

#define IDVBTuningSpace_get__NetworkType(This,NetworkTypeGuid)	\
    ( (This)->lpVtbl -> get__NetworkType(This,NetworkTypeGuid) ) 

#define IDVBTuningSpace_put__NetworkType(This,NetworkTypeGuid)	\
    ( (This)->lpVtbl -> put__NetworkType(This,NetworkTypeGuid) ) 

#define IDVBTuningSpace_CreateTuneRequest(This,TuneRequest)	\
    ( (This)->lpVtbl -> CreateTuneRequest(This,TuneRequest) ) 

#define IDVBTuningSpace_EnumCategoryGUIDs(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumCategoryGUIDs(This,ppEnum) ) 

#define IDVBTuningSpace_EnumDeviceMonikers(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumDeviceMonikers(This,ppEnum) ) 

#define IDVBTuningSpace_get_DefaultPreferredComponentTypes(This,ComponentTypes)	\
    ( (This)->lpVtbl -> get_DefaultPreferredComponentTypes(This,ComponentTypes) ) 

#define IDVBTuningSpace_put_DefaultPreferredComponentTypes(This,NewComponentTypes)	\
    ( (This)->lpVtbl -> put_DefaultPreferredComponentTypes(This,NewComponentTypes) ) 

#define IDVBTuningSpace_get_FrequencyMapping(This,pMapping)	\
    ( (This)->lpVtbl -> get_FrequencyMapping(This,pMapping) ) 

#define IDVBTuningSpace_put_FrequencyMapping(This,Mapping)	\
    ( (This)->lpVtbl -> put_FrequencyMapping(This,Mapping) ) 

#define IDVBTuningSpace_get_DefaultLocator(This,LocatorVal)	\
    ( (This)->lpVtbl -> get_DefaultLocator(This,LocatorVal) ) 

#define IDVBTuningSpace_put_DefaultLocator(This,LocatorVal)	\
    ( (This)->lpVtbl -> put_DefaultLocator(This,LocatorVal) ) 

#define IDVBTuningSpace_Clone(This,NewTS)	\
    ( (This)->lpVtbl -> Clone(This,NewTS) ) 


#define IDVBTuningSpace_get_SystemType(This,SysType)	\
    ( (This)->lpVtbl -> get_SystemType(This,SysType) ) 

#define IDVBTuningSpace_put_SystemType(This,SysType)	\
    ( (This)->lpVtbl -> put_SystemType(This,SysType) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDVBTuningSpace_INTERFACE_DEFINED__ */


#ifndef __IDVBTuningSpace2_INTERFACE_DEFINED__
#define __IDVBTuningSpace2_INTERFACE_DEFINED__

/* interface IDVBTuningSpace2 */
/* [unique][uuid][nonextensible][proxy][oleautomation][dual][hidden][object] */ 


EXTERN_C const IID IID_IDVBTuningSpace2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("843188B4-CE62-43db-966B-8145A094E040")
    IDVBTuningSpace2 : public IDVBTuningSpace
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_NetworkID( 
            /* [retval][out] */ __RPC__out long *NetworkID) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_NetworkID( 
            /* [in] */ long NetworkID) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDVBTuningSpace2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDVBTuningSpace2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDVBTuningSpace2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDVBTuningSpace2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IDVBTuningSpace2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IDVBTuningSpace2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IDVBTuningSpace2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IDVBTuningSpace2 * This,
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
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_UniqueName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UniqueName )( 
            __RPC__in IDVBTuningSpace2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Name);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_UniqueName)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_UniqueName )( 
            __RPC__in IDVBTuningSpace2 * This,
            /* [in] */ __RPC__in BSTR Name);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_FriendlyName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FriendlyName )( 
            __RPC__in IDVBTuningSpace2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Name);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_FriendlyName)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_FriendlyName )( 
            __RPC__in IDVBTuningSpace2 * This,
            /* [in] */ __RPC__in BSTR Name);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_CLSID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CLSID )( 
            __RPC__in IDVBTuningSpace2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *SpaceCLSID);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_NetworkType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NetworkType )( 
            __RPC__in IDVBTuningSpace2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *NetworkTypeGuid);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_NetworkType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_NetworkType )( 
            __RPC__in IDVBTuningSpace2 * This,
            /* [in] */ __RPC__in BSTR NetworkTypeGuid);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get__NetworkType)
        /* [restricted][hidden][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NetworkType )( 
            __RPC__in IDVBTuningSpace2 * This,
            /* [retval][out] */ __RPC__out GUID *NetworkTypeGuid);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put__NetworkType)
        /* [restricted][hidden][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put__NetworkType )( 
            __RPC__in IDVBTuningSpace2 * This,
            /* [in] */ __RPC__in REFCLSID NetworkTypeGuid);
        
        DECLSPEC_XFGVIRT(ITuningSpace, CreateTuneRequest)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateTuneRequest )( 
            __RPC__in IDVBTuningSpace2 * This,
            /* [retval][out] */ __RPC__deref_out_opt ITuneRequest **TuneRequest);
        
        DECLSPEC_XFGVIRT(ITuningSpace, EnumCategoryGUIDs)
        /* [restricted][hidden][id] */ HRESULT ( STDMETHODCALLTYPE *EnumCategoryGUIDs )( 
            __RPC__in IDVBTuningSpace2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumGUID **ppEnum);
        
        DECLSPEC_XFGVIRT(ITuningSpace, EnumDeviceMonikers)
        /* [restricted][hidden][id] */ HRESULT ( STDMETHODCALLTYPE *EnumDeviceMonikers )( 
            __RPC__in IDVBTuningSpace2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumMoniker **ppEnum);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_DefaultPreferredComponentTypes)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DefaultPreferredComponentTypes )( 
            __RPC__in IDVBTuningSpace2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IComponentTypes **ComponentTypes);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_DefaultPreferredComponentTypes)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DefaultPreferredComponentTypes )( 
            __RPC__in IDVBTuningSpace2 * This,
            /* [in] */ __RPC__in_opt IComponentTypes *NewComponentTypes);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_FrequencyMapping)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FrequencyMapping )( 
            __RPC__in IDVBTuningSpace2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pMapping);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_FrequencyMapping)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_FrequencyMapping )( 
            __RPC__in IDVBTuningSpace2 * This,
            __RPC__in BSTR Mapping);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_DefaultLocator)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DefaultLocator )( 
            __RPC__in IDVBTuningSpace2 * This,
            /* [retval][out] */ __RPC__deref_out_opt ILocator **LocatorVal);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_DefaultLocator)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DefaultLocator )( 
            __RPC__in IDVBTuningSpace2 * This,
            /* [in] */ __RPC__in_opt ILocator *LocatorVal);
        
        DECLSPEC_XFGVIRT(ITuningSpace, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IDVBTuningSpace2 * This,
            /* [retval][out] */ __RPC__deref_out_opt ITuningSpace **NewTS);
        
        DECLSPEC_XFGVIRT(IDVBTuningSpace, get_SystemType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SystemType )( 
            __RPC__in IDVBTuningSpace2 * This,
            /* [retval][out] */ __RPC__out DVBSystemType *SysType);
        
        DECLSPEC_XFGVIRT(IDVBTuningSpace, put_SystemType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SystemType )( 
            __RPC__in IDVBTuningSpace2 * This,
            /* [in] */ DVBSystemType SysType);
        
        DECLSPEC_XFGVIRT(IDVBTuningSpace2, get_NetworkID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NetworkID )( 
            __RPC__in IDVBTuningSpace2 * This,
            /* [retval][out] */ __RPC__out long *NetworkID);
        
        DECLSPEC_XFGVIRT(IDVBTuningSpace2, put_NetworkID)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_NetworkID )( 
            __RPC__in IDVBTuningSpace2 * This,
            /* [in] */ long NetworkID);
        
        END_INTERFACE
    } IDVBTuningSpace2Vtbl;

    interface IDVBTuningSpace2
    {
        CONST_VTBL struct IDVBTuningSpace2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDVBTuningSpace2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDVBTuningSpace2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDVBTuningSpace2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDVBTuningSpace2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IDVBTuningSpace2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IDVBTuningSpace2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IDVBTuningSpace2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IDVBTuningSpace2_get_UniqueName(This,Name)	\
    ( (This)->lpVtbl -> get_UniqueName(This,Name) ) 

#define IDVBTuningSpace2_put_UniqueName(This,Name)	\
    ( (This)->lpVtbl -> put_UniqueName(This,Name) ) 

#define IDVBTuningSpace2_get_FriendlyName(This,Name)	\
    ( (This)->lpVtbl -> get_FriendlyName(This,Name) ) 

#define IDVBTuningSpace2_put_FriendlyName(This,Name)	\
    ( (This)->lpVtbl -> put_FriendlyName(This,Name) ) 

#define IDVBTuningSpace2_get_CLSID(This,SpaceCLSID)	\
    ( (This)->lpVtbl -> get_CLSID(This,SpaceCLSID) ) 

#define IDVBTuningSpace2_get_NetworkType(This,NetworkTypeGuid)	\
    ( (This)->lpVtbl -> get_NetworkType(This,NetworkTypeGuid) ) 

#define IDVBTuningSpace2_put_NetworkType(This,NetworkTypeGuid)	\
    ( (This)->lpVtbl -> put_NetworkType(This,NetworkTypeGuid) ) 

#define IDVBTuningSpace2_get__NetworkType(This,NetworkTypeGuid)	\
    ( (This)->lpVtbl -> get__NetworkType(This,NetworkTypeGuid) ) 

#define IDVBTuningSpace2_put__NetworkType(This,NetworkTypeGuid)	\
    ( (This)->lpVtbl -> put__NetworkType(This,NetworkTypeGuid) ) 

#define IDVBTuningSpace2_CreateTuneRequest(This,TuneRequest)	\
    ( (This)->lpVtbl -> CreateTuneRequest(This,TuneRequest) ) 

#define IDVBTuningSpace2_EnumCategoryGUIDs(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumCategoryGUIDs(This,ppEnum) ) 

#define IDVBTuningSpace2_EnumDeviceMonikers(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumDeviceMonikers(This,ppEnum) ) 

#define IDVBTuningSpace2_get_DefaultPreferredComponentTypes(This,ComponentTypes)	\
    ( (This)->lpVtbl -> get_DefaultPreferredComponentTypes(This,ComponentTypes) ) 

#define IDVBTuningSpace2_put_DefaultPreferredComponentTypes(This,NewComponentTypes)	\
    ( (This)->lpVtbl -> put_DefaultPreferredComponentTypes(This,NewComponentTypes) ) 

#define IDVBTuningSpace2_get_FrequencyMapping(This,pMapping)	\
    ( (This)->lpVtbl -> get_FrequencyMapping(This,pMapping) ) 

#define IDVBTuningSpace2_put_FrequencyMapping(This,Mapping)	\
    ( (This)->lpVtbl -> put_FrequencyMapping(This,Mapping) ) 

#define IDVBTuningSpace2_get_DefaultLocator(This,LocatorVal)	\
    ( (This)->lpVtbl -> get_DefaultLocator(This,LocatorVal) ) 

#define IDVBTuningSpace2_put_DefaultLocator(This,LocatorVal)	\
    ( (This)->lpVtbl -> put_DefaultLocator(This,LocatorVal) ) 

#define IDVBTuningSpace2_Clone(This,NewTS)	\
    ( (This)->lpVtbl -> Clone(This,NewTS) ) 


#define IDVBTuningSpace2_get_SystemType(This,SysType)	\
    ( (This)->lpVtbl -> get_SystemType(This,SysType) ) 

#define IDVBTuningSpace2_put_SystemType(This,SysType)	\
    ( (This)->lpVtbl -> put_SystemType(This,SysType) ) 


#define IDVBTuningSpace2_get_NetworkID(This,NetworkID)	\
    ( (This)->lpVtbl -> get_NetworkID(This,NetworkID) ) 

#define IDVBTuningSpace2_put_NetworkID(This,NetworkID)	\
    ( (This)->lpVtbl -> put_NetworkID(This,NetworkID) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDVBTuningSpace2_INTERFACE_DEFINED__ */


#ifndef __IDVBSTuningSpace_INTERFACE_DEFINED__
#define __IDVBSTuningSpace_INTERFACE_DEFINED__

/* interface IDVBSTuningSpace */
/* [unique][uuid][nonextensible][proxy][oleautomation][dual][hidden][object] */ 


EXTERN_C const IID IID_IDVBSTuningSpace;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("CDF7BE60-D954-42fd-A972-78971958E470")
    IDVBSTuningSpace : public IDVBTuningSpace2
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_LowOscillator( 
            /* [retval][out] */ __RPC__out long *LowOscillator) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_LowOscillator( 
            /* [in] */ long LowOscillator) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_HighOscillator( 
            /* [retval][out] */ __RPC__out long *HighOscillator) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_HighOscillator( 
            /* [in] */ long HighOscillator) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_LNBSwitch( 
            /* [retval][out] */ __RPC__out long *LNBSwitch) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_LNBSwitch( 
            /* [in] */ long LNBSwitch) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_InputRange( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *InputRange) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_InputRange( 
            /* [in] */ __RPC__in BSTR InputRange) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SpectralInversion( 
            /* [retval][out] */ __RPC__out SpectralInversion *SpectralInversionVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_SpectralInversion( 
            /* [in] */ SpectralInversion SpectralInversionVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDVBSTuningSpaceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDVBSTuningSpace * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDVBSTuningSpace * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDVBSTuningSpace * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IDVBSTuningSpace * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IDVBSTuningSpace * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IDVBSTuningSpace * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IDVBSTuningSpace * This,
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
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_UniqueName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UniqueName )( 
            __RPC__in IDVBSTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Name);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_UniqueName)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_UniqueName )( 
            __RPC__in IDVBSTuningSpace * This,
            /* [in] */ __RPC__in BSTR Name);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_FriendlyName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FriendlyName )( 
            __RPC__in IDVBSTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Name);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_FriendlyName)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_FriendlyName )( 
            __RPC__in IDVBSTuningSpace * This,
            /* [in] */ __RPC__in BSTR Name);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_CLSID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CLSID )( 
            __RPC__in IDVBSTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *SpaceCLSID);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_NetworkType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NetworkType )( 
            __RPC__in IDVBSTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *NetworkTypeGuid);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_NetworkType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_NetworkType )( 
            __RPC__in IDVBSTuningSpace * This,
            /* [in] */ __RPC__in BSTR NetworkTypeGuid);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get__NetworkType)
        /* [restricted][hidden][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NetworkType )( 
            __RPC__in IDVBSTuningSpace * This,
            /* [retval][out] */ __RPC__out GUID *NetworkTypeGuid);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put__NetworkType)
        /* [restricted][hidden][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put__NetworkType )( 
            __RPC__in IDVBSTuningSpace * This,
            /* [in] */ __RPC__in REFCLSID NetworkTypeGuid);
        
        DECLSPEC_XFGVIRT(ITuningSpace, CreateTuneRequest)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateTuneRequest )( 
            __RPC__in IDVBSTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt ITuneRequest **TuneRequest);
        
        DECLSPEC_XFGVIRT(ITuningSpace, EnumCategoryGUIDs)
        /* [restricted][hidden][id] */ HRESULT ( STDMETHODCALLTYPE *EnumCategoryGUIDs )( 
            __RPC__in IDVBSTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumGUID **ppEnum);
        
        DECLSPEC_XFGVIRT(ITuningSpace, EnumDeviceMonikers)
        /* [restricted][hidden][id] */ HRESULT ( STDMETHODCALLTYPE *EnumDeviceMonikers )( 
            __RPC__in IDVBSTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumMoniker **ppEnum);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_DefaultPreferredComponentTypes)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DefaultPreferredComponentTypes )( 
            __RPC__in IDVBSTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt IComponentTypes **ComponentTypes);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_DefaultPreferredComponentTypes)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DefaultPreferredComponentTypes )( 
            __RPC__in IDVBSTuningSpace * This,
            /* [in] */ __RPC__in_opt IComponentTypes *NewComponentTypes);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_FrequencyMapping)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FrequencyMapping )( 
            __RPC__in IDVBSTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pMapping);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_FrequencyMapping)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_FrequencyMapping )( 
            __RPC__in IDVBSTuningSpace * This,
            __RPC__in BSTR Mapping);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_DefaultLocator)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DefaultLocator )( 
            __RPC__in IDVBSTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt ILocator **LocatorVal);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_DefaultLocator)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DefaultLocator )( 
            __RPC__in IDVBSTuningSpace * This,
            /* [in] */ __RPC__in_opt ILocator *LocatorVal);
        
        DECLSPEC_XFGVIRT(ITuningSpace, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IDVBSTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt ITuningSpace **NewTS);
        
        DECLSPEC_XFGVIRT(IDVBTuningSpace, get_SystemType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SystemType )( 
            __RPC__in IDVBSTuningSpace * This,
            /* [retval][out] */ __RPC__out DVBSystemType *SysType);
        
        DECLSPEC_XFGVIRT(IDVBTuningSpace, put_SystemType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SystemType )( 
            __RPC__in IDVBSTuningSpace * This,
            /* [in] */ DVBSystemType SysType);
        
        DECLSPEC_XFGVIRT(IDVBTuningSpace2, get_NetworkID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NetworkID )( 
            __RPC__in IDVBSTuningSpace * This,
            /* [retval][out] */ __RPC__out long *NetworkID);
        
        DECLSPEC_XFGVIRT(IDVBTuningSpace2, put_NetworkID)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_NetworkID )( 
            __RPC__in IDVBSTuningSpace * This,
            /* [in] */ long NetworkID);
        
        DECLSPEC_XFGVIRT(IDVBSTuningSpace, get_LowOscillator)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LowOscillator )( 
            __RPC__in IDVBSTuningSpace * This,
            /* [retval][out] */ __RPC__out long *LowOscillator);
        
        DECLSPEC_XFGVIRT(IDVBSTuningSpace, put_LowOscillator)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_LowOscillator )( 
            __RPC__in IDVBSTuningSpace * This,
            /* [in] */ long LowOscillator);
        
        DECLSPEC_XFGVIRT(IDVBSTuningSpace, get_HighOscillator)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_HighOscillator )( 
            __RPC__in IDVBSTuningSpace * This,
            /* [retval][out] */ __RPC__out long *HighOscillator);
        
        DECLSPEC_XFGVIRT(IDVBSTuningSpace, put_HighOscillator)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_HighOscillator )( 
            __RPC__in IDVBSTuningSpace * This,
            /* [in] */ long HighOscillator);
        
        DECLSPEC_XFGVIRT(IDVBSTuningSpace, get_LNBSwitch)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LNBSwitch )( 
            __RPC__in IDVBSTuningSpace * This,
            /* [retval][out] */ __RPC__out long *LNBSwitch);
        
        DECLSPEC_XFGVIRT(IDVBSTuningSpace, put_LNBSwitch)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_LNBSwitch )( 
            __RPC__in IDVBSTuningSpace * This,
            /* [in] */ long LNBSwitch);
        
        DECLSPEC_XFGVIRT(IDVBSTuningSpace, get_InputRange)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_InputRange )( 
            __RPC__in IDVBSTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *InputRange);
        
        DECLSPEC_XFGVIRT(IDVBSTuningSpace, put_InputRange)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_InputRange )( 
            __RPC__in IDVBSTuningSpace * This,
            /* [in] */ __RPC__in BSTR InputRange);
        
        DECLSPEC_XFGVIRT(IDVBSTuningSpace, get_SpectralInversion)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SpectralInversion )( 
            __RPC__in IDVBSTuningSpace * This,
            /* [retval][out] */ __RPC__out SpectralInversion *SpectralInversionVal);
        
        DECLSPEC_XFGVIRT(IDVBSTuningSpace, put_SpectralInversion)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SpectralInversion )( 
            __RPC__in IDVBSTuningSpace * This,
            /* [in] */ SpectralInversion SpectralInversionVal);
        
        END_INTERFACE
    } IDVBSTuningSpaceVtbl;

    interface IDVBSTuningSpace
    {
        CONST_VTBL struct IDVBSTuningSpaceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDVBSTuningSpace_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDVBSTuningSpace_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDVBSTuningSpace_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDVBSTuningSpace_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IDVBSTuningSpace_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IDVBSTuningSpace_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IDVBSTuningSpace_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IDVBSTuningSpace_get_UniqueName(This,Name)	\
    ( (This)->lpVtbl -> get_UniqueName(This,Name) ) 

#define IDVBSTuningSpace_put_UniqueName(This,Name)	\
    ( (This)->lpVtbl -> put_UniqueName(This,Name) ) 

#define IDVBSTuningSpace_get_FriendlyName(This,Name)	\
    ( (This)->lpVtbl -> get_FriendlyName(This,Name) ) 

#define IDVBSTuningSpace_put_FriendlyName(This,Name)	\
    ( (This)->lpVtbl -> put_FriendlyName(This,Name) ) 

#define IDVBSTuningSpace_get_CLSID(This,SpaceCLSID)	\
    ( (This)->lpVtbl -> get_CLSID(This,SpaceCLSID) ) 

#define IDVBSTuningSpace_get_NetworkType(This,NetworkTypeGuid)	\
    ( (This)->lpVtbl -> get_NetworkType(This,NetworkTypeGuid) ) 

#define IDVBSTuningSpace_put_NetworkType(This,NetworkTypeGuid)	\
    ( (This)->lpVtbl -> put_NetworkType(This,NetworkTypeGuid) ) 

#define IDVBSTuningSpace_get__NetworkType(This,NetworkTypeGuid)	\
    ( (This)->lpVtbl -> get__NetworkType(This,NetworkTypeGuid) ) 

#define IDVBSTuningSpace_put__NetworkType(This,NetworkTypeGuid)	\
    ( (This)->lpVtbl -> put__NetworkType(This,NetworkTypeGuid) ) 

#define IDVBSTuningSpace_CreateTuneRequest(This,TuneRequest)	\
    ( (This)->lpVtbl -> CreateTuneRequest(This,TuneRequest) ) 

#define IDVBSTuningSpace_EnumCategoryGUIDs(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumCategoryGUIDs(This,ppEnum) ) 

#define IDVBSTuningSpace_EnumDeviceMonikers(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumDeviceMonikers(This,ppEnum) ) 

#define IDVBSTuningSpace_get_DefaultPreferredComponentTypes(This,ComponentTypes)	\
    ( (This)->lpVtbl -> get_DefaultPreferredComponentTypes(This,ComponentTypes) ) 

#define IDVBSTuningSpace_put_DefaultPreferredComponentTypes(This,NewComponentTypes)	\
    ( (This)->lpVtbl -> put_DefaultPreferredComponentTypes(This,NewComponentTypes) ) 

#define IDVBSTuningSpace_get_FrequencyMapping(This,pMapping)	\
    ( (This)->lpVtbl -> get_FrequencyMapping(This,pMapping) ) 

#define IDVBSTuningSpace_put_FrequencyMapping(This,Mapping)	\
    ( (This)->lpVtbl -> put_FrequencyMapping(This,Mapping) ) 

#define IDVBSTuningSpace_get_DefaultLocator(This,LocatorVal)	\
    ( (This)->lpVtbl -> get_DefaultLocator(This,LocatorVal) ) 

#define IDVBSTuningSpace_put_DefaultLocator(This,LocatorVal)	\
    ( (This)->lpVtbl -> put_DefaultLocator(This,LocatorVal) ) 

#define IDVBSTuningSpace_Clone(This,NewTS)	\
    ( (This)->lpVtbl -> Clone(This,NewTS) ) 


#define IDVBSTuningSpace_get_SystemType(This,SysType)	\
    ( (This)->lpVtbl -> get_SystemType(This,SysType) ) 

#define IDVBSTuningSpace_put_SystemType(This,SysType)	\
    ( (This)->lpVtbl -> put_SystemType(This,SysType) ) 


#define IDVBSTuningSpace_get_NetworkID(This,NetworkID)	\
    ( (This)->lpVtbl -> get_NetworkID(This,NetworkID) ) 

#define IDVBSTuningSpace_put_NetworkID(This,NetworkID)	\
    ( (This)->lpVtbl -> put_NetworkID(This,NetworkID) ) 


#define IDVBSTuningSpace_get_LowOscillator(This,LowOscillator)	\
    ( (This)->lpVtbl -> get_LowOscillator(This,LowOscillator) ) 

#define IDVBSTuningSpace_put_LowOscillator(This,LowOscillator)	\
    ( (This)->lpVtbl -> put_LowOscillator(This,LowOscillator) ) 

#define IDVBSTuningSpace_get_HighOscillator(This,HighOscillator)	\
    ( (This)->lpVtbl -> get_HighOscillator(This,HighOscillator) ) 

#define IDVBSTuningSpace_put_HighOscillator(This,HighOscillator)	\
    ( (This)->lpVtbl -> put_HighOscillator(This,HighOscillator) ) 

#define IDVBSTuningSpace_get_LNBSwitch(This,LNBSwitch)	\
    ( (This)->lpVtbl -> get_LNBSwitch(This,LNBSwitch) ) 

#define IDVBSTuningSpace_put_LNBSwitch(This,LNBSwitch)	\
    ( (This)->lpVtbl -> put_LNBSwitch(This,LNBSwitch) ) 

#define IDVBSTuningSpace_get_InputRange(This,InputRange)	\
    ( (This)->lpVtbl -> get_InputRange(This,InputRange) ) 

#define IDVBSTuningSpace_put_InputRange(This,InputRange)	\
    ( (This)->lpVtbl -> put_InputRange(This,InputRange) ) 

#define IDVBSTuningSpace_get_SpectralInversion(This,SpectralInversionVal)	\
    ( (This)->lpVtbl -> get_SpectralInversion(This,SpectralInversionVal) ) 

#define IDVBSTuningSpace_put_SpectralInversion(This,SpectralInversionVal)	\
    ( (This)->lpVtbl -> put_SpectralInversion(This,SpectralInversionVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDVBSTuningSpace_INTERFACE_DEFINED__ */


#ifndef __IAuxInTuningSpace_INTERFACE_DEFINED__
#define __IAuxInTuningSpace_INTERFACE_DEFINED__

/* interface IAuxInTuningSpace */
/* [unique][uuid][nonextensible][proxy][oleautomation][dual][hidden][object] */ 


EXTERN_C const IID IID_IAuxInTuningSpace;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("E48244B8-7E17-4f76-A763-5090FF1E2F30")
    IAuxInTuningSpace : public ITuningSpace
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct IAuxInTuningSpaceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAuxInTuningSpace * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAuxInTuningSpace * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAuxInTuningSpace * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IAuxInTuningSpace * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IAuxInTuningSpace * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IAuxInTuningSpace * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IAuxInTuningSpace * This,
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
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_UniqueName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UniqueName )( 
            __RPC__in IAuxInTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Name);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_UniqueName)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_UniqueName )( 
            __RPC__in IAuxInTuningSpace * This,
            /* [in] */ __RPC__in BSTR Name);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_FriendlyName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FriendlyName )( 
            __RPC__in IAuxInTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Name);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_FriendlyName)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_FriendlyName )( 
            __RPC__in IAuxInTuningSpace * This,
            /* [in] */ __RPC__in BSTR Name);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_CLSID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CLSID )( 
            __RPC__in IAuxInTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *SpaceCLSID);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_NetworkType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NetworkType )( 
            __RPC__in IAuxInTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *NetworkTypeGuid);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_NetworkType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_NetworkType )( 
            __RPC__in IAuxInTuningSpace * This,
            /* [in] */ __RPC__in BSTR NetworkTypeGuid);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get__NetworkType)
        /* [restricted][hidden][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NetworkType )( 
            __RPC__in IAuxInTuningSpace * This,
            /* [retval][out] */ __RPC__out GUID *NetworkTypeGuid);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put__NetworkType)
        /* [restricted][hidden][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put__NetworkType )( 
            __RPC__in IAuxInTuningSpace * This,
            /* [in] */ __RPC__in REFCLSID NetworkTypeGuid);
        
        DECLSPEC_XFGVIRT(ITuningSpace, CreateTuneRequest)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateTuneRequest )( 
            __RPC__in IAuxInTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt ITuneRequest **TuneRequest);
        
        DECLSPEC_XFGVIRT(ITuningSpace, EnumCategoryGUIDs)
        /* [restricted][hidden][id] */ HRESULT ( STDMETHODCALLTYPE *EnumCategoryGUIDs )( 
            __RPC__in IAuxInTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumGUID **ppEnum);
        
        DECLSPEC_XFGVIRT(ITuningSpace, EnumDeviceMonikers)
        /* [restricted][hidden][id] */ HRESULT ( STDMETHODCALLTYPE *EnumDeviceMonikers )( 
            __RPC__in IAuxInTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumMoniker **ppEnum);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_DefaultPreferredComponentTypes)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DefaultPreferredComponentTypes )( 
            __RPC__in IAuxInTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt IComponentTypes **ComponentTypes);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_DefaultPreferredComponentTypes)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DefaultPreferredComponentTypes )( 
            __RPC__in IAuxInTuningSpace * This,
            /* [in] */ __RPC__in_opt IComponentTypes *NewComponentTypes);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_FrequencyMapping)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FrequencyMapping )( 
            __RPC__in IAuxInTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pMapping);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_FrequencyMapping)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_FrequencyMapping )( 
            __RPC__in IAuxInTuningSpace * This,
            __RPC__in BSTR Mapping);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_DefaultLocator)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DefaultLocator )( 
            __RPC__in IAuxInTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt ILocator **LocatorVal);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_DefaultLocator)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DefaultLocator )( 
            __RPC__in IAuxInTuningSpace * This,
            /* [in] */ __RPC__in_opt ILocator *LocatorVal);
        
        DECLSPEC_XFGVIRT(ITuningSpace, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IAuxInTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt ITuningSpace **NewTS);
        
        END_INTERFACE
    } IAuxInTuningSpaceVtbl;

    interface IAuxInTuningSpace
    {
        CONST_VTBL struct IAuxInTuningSpaceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAuxInTuningSpace_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAuxInTuningSpace_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAuxInTuningSpace_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAuxInTuningSpace_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IAuxInTuningSpace_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IAuxInTuningSpace_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IAuxInTuningSpace_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IAuxInTuningSpace_get_UniqueName(This,Name)	\
    ( (This)->lpVtbl -> get_UniqueName(This,Name) ) 

#define IAuxInTuningSpace_put_UniqueName(This,Name)	\
    ( (This)->lpVtbl -> put_UniqueName(This,Name) ) 

#define IAuxInTuningSpace_get_FriendlyName(This,Name)	\
    ( (This)->lpVtbl -> get_FriendlyName(This,Name) ) 

#define IAuxInTuningSpace_put_FriendlyName(This,Name)	\
    ( (This)->lpVtbl -> put_FriendlyName(This,Name) ) 

#define IAuxInTuningSpace_get_CLSID(This,SpaceCLSID)	\
    ( (This)->lpVtbl -> get_CLSID(This,SpaceCLSID) ) 

#define IAuxInTuningSpace_get_NetworkType(This,NetworkTypeGuid)	\
    ( (This)->lpVtbl -> get_NetworkType(This,NetworkTypeGuid) ) 

#define IAuxInTuningSpace_put_NetworkType(This,NetworkTypeGuid)	\
    ( (This)->lpVtbl -> put_NetworkType(This,NetworkTypeGuid) ) 

#define IAuxInTuningSpace_get__NetworkType(This,NetworkTypeGuid)	\
    ( (This)->lpVtbl -> get__NetworkType(This,NetworkTypeGuid) ) 

#define IAuxInTuningSpace_put__NetworkType(This,NetworkTypeGuid)	\
    ( (This)->lpVtbl -> put__NetworkType(This,NetworkTypeGuid) ) 

#define IAuxInTuningSpace_CreateTuneRequest(This,TuneRequest)	\
    ( (This)->lpVtbl -> CreateTuneRequest(This,TuneRequest) ) 

#define IAuxInTuningSpace_EnumCategoryGUIDs(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumCategoryGUIDs(This,ppEnum) ) 

#define IAuxInTuningSpace_EnumDeviceMonikers(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumDeviceMonikers(This,ppEnum) ) 

#define IAuxInTuningSpace_get_DefaultPreferredComponentTypes(This,ComponentTypes)	\
    ( (This)->lpVtbl -> get_DefaultPreferredComponentTypes(This,ComponentTypes) ) 

#define IAuxInTuningSpace_put_DefaultPreferredComponentTypes(This,NewComponentTypes)	\
    ( (This)->lpVtbl -> put_DefaultPreferredComponentTypes(This,NewComponentTypes) ) 

#define IAuxInTuningSpace_get_FrequencyMapping(This,pMapping)	\
    ( (This)->lpVtbl -> get_FrequencyMapping(This,pMapping) ) 

#define IAuxInTuningSpace_put_FrequencyMapping(This,Mapping)	\
    ( (This)->lpVtbl -> put_FrequencyMapping(This,Mapping) ) 

#define IAuxInTuningSpace_get_DefaultLocator(This,LocatorVal)	\
    ( (This)->lpVtbl -> get_DefaultLocator(This,LocatorVal) ) 

#define IAuxInTuningSpace_put_DefaultLocator(This,LocatorVal)	\
    ( (This)->lpVtbl -> put_DefaultLocator(This,LocatorVal) ) 

#define IAuxInTuningSpace_Clone(This,NewTS)	\
    ( (This)->lpVtbl -> Clone(This,NewTS) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAuxInTuningSpace_INTERFACE_DEFINED__ */


#ifndef __IAuxInTuningSpace2_INTERFACE_DEFINED__
#define __IAuxInTuningSpace2_INTERFACE_DEFINED__

/* interface IAuxInTuningSpace2 */
/* [unique][uuid][nonextensible][proxy][oleautomation][dual][hidden][object] */ 


EXTERN_C const IID IID_IAuxInTuningSpace2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B10931ED-8BFE-4AB0-9DCE-E469C29A9729")
    IAuxInTuningSpace2 : public IAuxInTuningSpace
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CountryCode( 
            /* [retval][out] */ __RPC__out long *CountryCodeVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_CountryCode( 
            /* [in] */ long NewCountryCodeVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAuxInTuningSpace2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAuxInTuningSpace2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAuxInTuningSpace2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAuxInTuningSpace2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IAuxInTuningSpace2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IAuxInTuningSpace2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IAuxInTuningSpace2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IAuxInTuningSpace2 * This,
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
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_UniqueName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UniqueName )( 
            __RPC__in IAuxInTuningSpace2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Name);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_UniqueName)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_UniqueName )( 
            __RPC__in IAuxInTuningSpace2 * This,
            /* [in] */ __RPC__in BSTR Name);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_FriendlyName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FriendlyName )( 
            __RPC__in IAuxInTuningSpace2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Name);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_FriendlyName)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_FriendlyName )( 
            __RPC__in IAuxInTuningSpace2 * This,
            /* [in] */ __RPC__in BSTR Name);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_CLSID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CLSID )( 
            __RPC__in IAuxInTuningSpace2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *SpaceCLSID);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_NetworkType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NetworkType )( 
            __RPC__in IAuxInTuningSpace2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *NetworkTypeGuid);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_NetworkType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_NetworkType )( 
            __RPC__in IAuxInTuningSpace2 * This,
            /* [in] */ __RPC__in BSTR NetworkTypeGuid);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get__NetworkType)
        /* [restricted][hidden][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NetworkType )( 
            __RPC__in IAuxInTuningSpace2 * This,
            /* [retval][out] */ __RPC__out GUID *NetworkTypeGuid);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put__NetworkType)
        /* [restricted][hidden][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put__NetworkType )( 
            __RPC__in IAuxInTuningSpace2 * This,
            /* [in] */ __RPC__in REFCLSID NetworkTypeGuid);
        
        DECLSPEC_XFGVIRT(ITuningSpace, CreateTuneRequest)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateTuneRequest )( 
            __RPC__in IAuxInTuningSpace2 * This,
            /* [retval][out] */ __RPC__deref_out_opt ITuneRequest **TuneRequest);
        
        DECLSPEC_XFGVIRT(ITuningSpace, EnumCategoryGUIDs)
        /* [restricted][hidden][id] */ HRESULT ( STDMETHODCALLTYPE *EnumCategoryGUIDs )( 
            __RPC__in IAuxInTuningSpace2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumGUID **ppEnum);
        
        DECLSPEC_XFGVIRT(ITuningSpace, EnumDeviceMonikers)
        /* [restricted][hidden][id] */ HRESULT ( STDMETHODCALLTYPE *EnumDeviceMonikers )( 
            __RPC__in IAuxInTuningSpace2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumMoniker **ppEnum);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_DefaultPreferredComponentTypes)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DefaultPreferredComponentTypes )( 
            __RPC__in IAuxInTuningSpace2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IComponentTypes **ComponentTypes);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_DefaultPreferredComponentTypes)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DefaultPreferredComponentTypes )( 
            __RPC__in IAuxInTuningSpace2 * This,
            /* [in] */ __RPC__in_opt IComponentTypes *NewComponentTypes);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_FrequencyMapping)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FrequencyMapping )( 
            __RPC__in IAuxInTuningSpace2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pMapping);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_FrequencyMapping)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_FrequencyMapping )( 
            __RPC__in IAuxInTuningSpace2 * This,
            __RPC__in BSTR Mapping);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_DefaultLocator)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DefaultLocator )( 
            __RPC__in IAuxInTuningSpace2 * This,
            /* [retval][out] */ __RPC__deref_out_opt ILocator **LocatorVal);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_DefaultLocator)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DefaultLocator )( 
            __RPC__in IAuxInTuningSpace2 * This,
            /* [in] */ __RPC__in_opt ILocator *LocatorVal);
        
        DECLSPEC_XFGVIRT(ITuningSpace, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IAuxInTuningSpace2 * This,
            /* [retval][out] */ __RPC__deref_out_opt ITuningSpace **NewTS);
        
        DECLSPEC_XFGVIRT(IAuxInTuningSpace2, get_CountryCode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CountryCode )( 
            __RPC__in IAuxInTuningSpace2 * This,
            /* [retval][out] */ __RPC__out long *CountryCodeVal);
        
        DECLSPEC_XFGVIRT(IAuxInTuningSpace2, put_CountryCode)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_CountryCode )( 
            __RPC__in IAuxInTuningSpace2 * This,
            /* [in] */ long NewCountryCodeVal);
        
        END_INTERFACE
    } IAuxInTuningSpace2Vtbl;

    interface IAuxInTuningSpace2
    {
        CONST_VTBL struct IAuxInTuningSpace2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAuxInTuningSpace2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAuxInTuningSpace2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAuxInTuningSpace2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAuxInTuningSpace2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IAuxInTuningSpace2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IAuxInTuningSpace2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IAuxInTuningSpace2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IAuxInTuningSpace2_get_UniqueName(This,Name)	\
    ( (This)->lpVtbl -> get_UniqueName(This,Name) ) 

#define IAuxInTuningSpace2_put_UniqueName(This,Name)	\
    ( (This)->lpVtbl -> put_UniqueName(This,Name) ) 

#define IAuxInTuningSpace2_get_FriendlyName(This,Name)	\
    ( (This)->lpVtbl -> get_FriendlyName(This,Name) ) 

#define IAuxInTuningSpace2_put_FriendlyName(This,Name)	\
    ( (This)->lpVtbl -> put_FriendlyName(This,Name) ) 

#define IAuxInTuningSpace2_get_CLSID(This,SpaceCLSID)	\
    ( (This)->lpVtbl -> get_CLSID(This,SpaceCLSID) ) 

#define IAuxInTuningSpace2_get_NetworkType(This,NetworkTypeGuid)	\
    ( (This)->lpVtbl -> get_NetworkType(This,NetworkTypeGuid) ) 

#define IAuxInTuningSpace2_put_NetworkType(This,NetworkTypeGuid)	\
    ( (This)->lpVtbl -> put_NetworkType(This,NetworkTypeGuid) ) 

#define IAuxInTuningSpace2_get__NetworkType(This,NetworkTypeGuid)	\
    ( (This)->lpVtbl -> get__NetworkType(This,NetworkTypeGuid) ) 

#define IAuxInTuningSpace2_put__NetworkType(This,NetworkTypeGuid)	\
    ( (This)->lpVtbl -> put__NetworkType(This,NetworkTypeGuid) ) 

#define IAuxInTuningSpace2_CreateTuneRequest(This,TuneRequest)	\
    ( (This)->lpVtbl -> CreateTuneRequest(This,TuneRequest) ) 

#define IAuxInTuningSpace2_EnumCategoryGUIDs(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumCategoryGUIDs(This,ppEnum) ) 

#define IAuxInTuningSpace2_EnumDeviceMonikers(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumDeviceMonikers(This,ppEnum) ) 

#define IAuxInTuningSpace2_get_DefaultPreferredComponentTypes(This,ComponentTypes)	\
    ( (This)->lpVtbl -> get_DefaultPreferredComponentTypes(This,ComponentTypes) ) 

#define IAuxInTuningSpace2_put_DefaultPreferredComponentTypes(This,NewComponentTypes)	\
    ( (This)->lpVtbl -> put_DefaultPreferredComponentTypes(This,NewComponentTypes) ) 

#define IAuxInTuningSpace2_get_FrequencyMapping(This,pMapping)	\
    ( (This)->lpVtbl -> get_FrequencyMapping(This,pMapping) ) 

#define IAuxInTuningSpace2_put_FrequencyMapping(This,Mapping)	\
    ( (This)->lpVtbl -> put_FrequencyMapping(This,Mapping) ) 

#define IAuxInTuningSpace2_get_DefaultLocator(This,LocatorVal)	\
    ( (This)->lpVtbl -> get_DefaultLocator(This,LocatorVal) ) 

#define IAuxInTuningSpace2_put_DefaultLocator(This,LocatorVal)	\
    ( (This)->lpVtbl -> put_DefaultLocator(This,LocatorVal) ) 

#define IAuxInTuningSpace2_Clone(This,NewTS)	\
    ( (This)->lpVtbl -> Clone(This,NewTS) ) 



#define IAuxInTuningSpace2_get_CountryCode(This,CountryCodeVal)	\
    ( (This)->lpVtbl -> get_CountryCode(This,CountryCodeVal) ) 

#define IAuxInTuningSpace2_put_CountryCode(This,NewCountryCodeVal)	\
    ( (This)->lpVtbl -> put_CountryCode(This,NewCountryCodeVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAuxInTuningSpace2_INTERFACE_DEFINED__ */


#ifndef __IAnalogTVTuningSpace_INTERFACE_DEFINED__
#define __IAnalogTVTuningSpace_INTERFACE_DEFINED__

/* interface IAnalogTVTuningSpace */
/* [unique][uuid][nonextensible][proxy][oleautomation][dual][hidden][object] */ 


EXTERN_C const IID IID_IAnalogTVTuningSpace;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2A6E293C-2595-11d3-B64C-00C04F79498E")
    IAnalogTVTuningSpace : public ITuningSpace
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MinChannel( 
            /* [retval][out] */ __RPC__out long *MinChannelVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_MinChannel( 
            /* [in] */ long NewMinChannelVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MaxChannel( 
            /* [retval][out] */ __RPC__out long *MaxChannelVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_MaxChannel( 
            /* [in] */ long NewMaxChannelVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_InputType( 
            /* [retval][out] */ __RPC__out TunerInputType *InputTypeVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_InputType( 
            /* [in] */ TunerInputType NewInputTypeVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CountryCode( 
            /* [retval][out] */ __RPC__out long *CountryCodeVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_CountryCode( 
            /* [in] */ long NewCountryCodeVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAnalogTVTuningSpaceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAnalogTVTuningSpace * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAnalogTVTuningSpace * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAnalogTVTuningSpace * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IAnalogTVTuningSpace * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IAnalogTVTuningSpace * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IAnalogTVTuningSpace * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IAnalogTVTuningSpace * This,
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
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_UniqueName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UniqueName )( 
            __RPC__in IAnalogTVTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Name);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_UniqueName)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_UniqueName )( 
            __RPC__in IAnalogTVTuningSpace * This,
            /* [in] */ __RPC__in BSTR Name);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_FriendlyName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FriendlyName )( 
            __RPC__in IAnalogTVTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Name);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_FriendlyName)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_FriendlyName )( 
            __RPC__in IAnalogTVTuningSpace * This,
            /* [in] */ __RPC__in BSTR Name);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_CLSID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CLSID )( 
            __RPC__in IAnalogTVTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *SpaceCLSID);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_NetworkType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NetworkType )( 
            __RPC__in IAnalogTVTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *NetworkTypeGuid);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_NetworkType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_NetworkType )( 
            __RPC__in IAnalogTVTuningSpace * This,
            /* [in] */ __RPC__in BSTR NetworkTypeGuid);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get__NetworkType)
        /* [restricted][hidden][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NetworkType )( 
            __RPC__in IAnalogTVTuningSpace * This,
            /* [retval][out] */ __RPC__out GUID *NetworkTypeGuid);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put__NetworkType)
        /* [restricted][hidden][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put__NetworkType )( 
            __RPC__in IAnalogTVTuningSpace * This,
            /* [in] */ __RPC__in REFCLSID NetworkTypeGuid);
        
        DECLSPEC_XFGVIRT(ITuningSpace, CreateTuneRequest)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateTuneRequest )( 
            __RPC__in IAnalogTVTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt ITuneRequest **TuneRequest);
        
        DECLSPEC_XFGVIRT(ITuningSpace, EnumCategoryGUIDs)
        /* [restricted][hidden][id] */ HRESULT ( STDMETHODCALLTYPE *EnumCategoryGUIDs )( 
            __RPC__in IAnalogTVTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumGUID **ppEnum);
        
        DECLSPEC_XFGVIRT(ITuningSpace, EnumDeviceMonikers)
        /* [restricted][hidden][id] */ HRESULT ( STDMETHODCALLTYPE *EnumDeviceMonikers )( 
            __RPC__in IAnalogTVTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumMoniker **ppEnum);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_DefaultPreferredComponentTypes)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DefaultPreferredComponentTypes )( 
            __RPC__in IAnalogTVTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt IComponentTypes **ComponentTypes);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_DefaultPreferredComponentTypes)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DefaultPreferredComponentTypes )( 
            __RPC__in IAnalogTVTuningSpace * This,
            /* [in] */ __RPC__in_opt IComponentTypes *NewComponentTypes);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_FrequencyMapping)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FrequencyMapping )( 
            __RPC__in IAnalogTVTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pMapping);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_FrequencyMapping)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_FrequencyMapping )( 
            __RPC__in IAnalogTVTuningSpace * This,
            __RPC__in BSTR Mapping);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_DefaultLocator)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DefaultLocator )( 
            __RPC__in IAnalogTVTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt ILocator **LocatorVal);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_DefaultLocator)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DefaultLocator )( 
            __RPC__in IAnalogTVTuningSpace * This,
            /* [in] */ __RPC__in_opt ILocator *LocatorVal);
        
        DECLSPEC_XFGVIRT(ITuningSpace, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IAnalogTVTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt ITuningSpace **NewTS);
        
        DECLSPEC_XFGVIRT(IAnalogTVTuningSpace, get_MinChannel)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MinChannel )( 
            __RPC__in IAnalogTVTuningSpace * This,
            /* [retval][out] */ __RPC__out long *MinChannelVal);
        
        DECLSPEC_XFGVIRT(IAnalogTVTuningSpace, put_MinChannel)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MinChannel )( 
            __RPC__in IAnalogTVTuningSpace * This,
            /* [in] */ long NewMinChannelVal);
        
        DECLSPEC_XFGVIRT(IAnalogTVTuningSpace, get_MaxChannel)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MaxChannel )( 
            __RPC__in IAnalogTVTuningSpace * This,
            /* [retval][out] */ __RPC__out long *MaxChannelVal);
        
        DECLSPEC_XFGVIRT(IAnalogTVTuningSpace, put_MaxChannel)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MaxChannel )( 
            __RPC__in IAnalogTVTuningSpace * This,
            /* [in] */ long NewMaxChannelVal);
        
        DECLSPEC_XFGVIRT(IAnalogTVTuningSpace, get_InputType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_InputType )( 
            __RPC__in IAnalogTVTuningSpace * This,
            /* [retval][out] */ __RPC__out TunerInputType *InputTypeVal);
        
        DECLSPEC_XFGVIRT(IAnalogTVTuningSpace, put_InputType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_InputType )( 
            __RPC__in IAnalogTVTuningSpace * This,
            /* [in] */ TunerInputType NewInputTypeVal);
        
        DECLSPEC_XFGVIRT(IAnalogTVTuningSpace, get_CountryCode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CountryCode )( 
            __RPC__in IAnalogTVTuningSpace * This,
            /* [retval][out] */ __RPC__out long *CountryCodeVal);
        
        DECLSPEC_XFGVIRT(IAnalogTVTuningSpace, put_CountryCode)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_CountryCode )( 
            __RPC__in IAnalogTVTuningSpace * This,
            /* [in] */ long NewCountryCodeVal);
        
        END_INTERFACE
    } IAnalogTVTuningSpaceVtbl;

    interface IAnalogTVTuningSpace
    {
        CONST_VTBL struct IAnalogTVTuningSpaceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAnalogTVTuningSpace_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAnalogTVTuningSpace_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAnalogTVTuningSpace_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAnalogTVTuningSpace_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IAnalogTVTuningSpace_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IAnalogTVTuningSpace_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IAnalogTVTuningSpace_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IAnalogTVTuningSpace_get_UniqueName(This,Name)	\
    ( (This)->lpVtbl -> get_UniqueName(This,Name) ) 

#define IAnalogTVTuningSpace_put_UniqueName(This,Name)	\
    ( (This)->lpVtbl -> put_UniqueName(This,Name) ) 

#define IAnalogTVTuningSpace_get_FriendlyName(This,Name)	\
    ( (This)->lpVtbl -> get_FriendlyName(This,Name) ) 

#define IAnalogTVTuningSpace_put_FriendlyName(This,Name)	\
    ( (This)->lpVtbl -> put_FriendlyName(This,Name) ) 

#define IAnalogTVTuningSpace_get_CLSID(This,SpaceCLSID)	\
    ( (This)->lpVtbl -> get_CLSID(This,SpaceCLSID) ) 

#define IAnalogTVTuningSpace_get_NetworkType(This,NetworkTypeGuid)	\
    ( (This)->lpVtbl -> get_NetworkType(This,NetworkTypeGuid) ) 

#define IAnalogTVTuningSpace_put_NetworkType(This,NetworkTypeGuid)	\
    ( (This)->lpVtbl -> put_NetworkType(This,NetworkTypeGuid) ) 

#define IAnalogTVTuningSpace_get__NetworkType(This,NetworkTypeGuid)	\
    ( (This)->lpVtbl -> get__NetworkType(This,NetworkTypeGuid) ) 

#define IAnalogTVTuningSpace_put__NetworkType(This,NetworkTypeGuid)	\
    ( (This)->lpVtbl -> put__NetworkType(This,NetworkTypeGuid) ) 

#define IAnalogTVTuningSpace_CreateTuneRequest(This,TuneRequest)	\
    ( (This)->lpVtbl -> CreateTuneRequest(This,TuneRequest) ) 

#define IAnalogTVTuningSpace_EnumCategoryGUIDs(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumCategoryGUIDs(This,ppEnum) ) 

#define IAnalogTVTuningSpace_EnumDeviceMonikers(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumDeviceMonikers(This,ppEnum) ) 

#define IAnalogTVTuningSpace_get_DefaultPreferredComponentTypes(This,ComponentTypes)	\
    ( (This)->lpVtbl -> get_DefaultPreferredComponentTypes(This,ComponentTypes) ) 

#define IAnalogTVTuningSpace_put_DefaultPreferredComponentTypes(This,NewComponentTypes)	\
    ( (This)->lpVtbl -> put_DefaultPreferredComponentTypes(This,NewComponentTypes) ) 

#define IAnalogTVTuningSpace_get_FrequencyMapping(This,pMapping)	\
    ( (This)->lpVtbl -> get_FrequencyMapping(This,pMapping) ) 

#define IAnalogTVTuningSpace_put_FrequencyMapping(This,Mapping)	\
    ( (This)->lpVtbl -> put_FrequencyMapping(This,Mapping) ) 

#define IAnalogTVTuningSpace_get_DefaultLocator(This,LocatorVal)	\
    ( (This)->lpVtbl -> get_DefaultLocator(This,LocatorVal) ) 

#define IAnalogTVTuningSpace_put_DefaultLocator(This,LocatorVal)	\
    ( (This)->lpVtbl -> put_DefaultLocator(This,LocatorVal) ) 

#define IAnalogTVTuningSpace_Clone(This,NewTS)	\
    ( (This)->lpVtbl -> Clone(This,NewTS) ) 


#define IAnalogTVTuningSpace_get_MinChannel(This,MinChannelVal)	\
    ( (This)->lpVtbl -> get_MinChannel(This,MinChannelVal) ) 

#define IAnalogTVTuningSpace_put_MinChannel(This,NewMinChannelVal)	\
    ( (This)->lpVtbl -> put_MinChannel(This,NewMinChannelVal) ) 

#define IAnalogTVTuningSpace_get_MaxChannel(This,MaxChannelVal)	\
    ( (This)->lpVtbl -> get_MaxChannel(This,MaxChannelVal) ) 

#define IAnalogTVTuningSpace_put_MaxChannel(This,NewMaxChannelVal)	\
    ( (This)->lpVtbl -> put_MaxChannel(This,NewMaxChannelVal) ) 

#define IAnalogTVTuningSpace_get_InputType(This,InputTypeVal)	\
    ( (This)->lpVtbl -> get_InputType(This,InputTypeVal) ) 

#define IAnalogTVTuningSpace_put_InputType(This,NewInputTypeVal)	\
    ( (This)->lpVtbl -> put_InputType(This,NewInputTypeVal) ) 

#define IAnalogTVTuningSpace_get_CountryCode(This,CountryCodeVal)	\
    ( (This)->lpVtbl -> get_CountryCode(This,CountryCodeVal) ) 

#define IAnalogTVTuningSpace_put_CountryCode(This,NewCountryCodeVal)	\
    ( (This)->lpVtbl -> put_CountryCode(This,NewCountryCodeVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAnalogTVTuningSpace_INTERFACE_DEFINED__ */


#ifndef __IATSCTuningSpace_INTERFACE_DEFINED__
#define __IATSCTuningSpace_INTERFACE_DEFINED__

/* interface IATSCTuningSpace */
/* [unique][uuid][nonextensible][proxy][oleautomation][dual][hidden][object] */ 


EXTERN_C const IID IID_IATSCTuningSpace;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0369B4E2-45B6-11d3-B650-00C04F79498E")
    IATSCTuningSpace : public IAnalogTVTuningSpace
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MinMinorChannel( 
            /* [retval][out] */ __RPC__out long *MinMinorChannelVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_MinMinorChannel( 
            /* [in] */ long NewMinMinorChannelVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MaxMinorChannel( 
            /* [retval][out] */ __RPC__out long *MaxMinorChannelVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_MaxMinorChannel( 
            /* [in] */ long NewMaxMinorChannelVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MinPhysicalChannel( 
            /* [retval][out] */ __RPC__out long *MinPhysicalChannelVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_MinPhysicalChannel( 
            /* [in] */ long NewMinPhysicalChannelVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MaxPhysicalChannel( 
            /* [retval][out] */ __RPC__out long *MaxPhysicalChannelVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_MaxPhysicalChannel( 
            /* [in] */ long NewMaxPhysicalChannelVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IATSCTuningSpaceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IATSCTuningSpace * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IATSCTuningSpace * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IATSCTuningSpace * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IATSCTuningSpace * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IATSCTuningSpace * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IATSCTuningSpace * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IATSCTuningSpace * This,
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
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_UniqueName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UniqueName )( 
            __RPC__in IATSCTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Name);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_UniqueName)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_UniqueName )( 
            __RPC__in IATSCTuningSpace * This,
            /* [in] */ __RPC__in BSTR Name);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_FriendlyName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FriendlyName )( 
            __RPC__in IATSCTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Name);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_FriendlyName)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_FriendlyName )( 
            __RPC__in IATSCTuningSpace * This,
            /* [in] */ __RPC__in BSTR Name);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_CLSID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CLSID )( 
            __RPC__in IATSCTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *SpaceCLSID);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_NetworkType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NetworkType )( 
            __RPC__in IATSCTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *NetworkTypeGuid);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_NetworkType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_NetworkType )( 
            __RPC__in IATSCTuningSpace * This,
            /* [in] */ __RPC__in BSTR NetworkTypeGuid);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get__NetworkType)
        /* [restricted][hidden][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NetworkType )( 
            __RPC__in IATSCTuningSpace * This,
            /* [retval][out] */ __RPC__out GUID *NetworkTypeGuid);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put__NetworkType)
        /* [restricted][hidden][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put__NetworkType )( 
            __RPC__in IATSCTuningSpace * This,
            /* [in] */ __RPC__in REFCLSID NetworkTypeGuid);
        
        DECLSPEC_XFGVIRT(ITuningSpace, CreateTuneRequest)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateTuneRequest )( 
            __RPC__in IATSCTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt ITuneRequest **TuneRequest);
        
        DECLSPEC_XFGVIRT(ITuningSpace, EnumCategoryGUIDs)
        /* [restricted][hidden][id] */ HRESULT ( STDMETHODCALLTYPE *EnumCategoryGUIDs )( 
            __RPC__in IATSCTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumGUID **ppEnum);
        
        DECLSPEC_XFGVIRT(ITuningSpace, EnumDeviceMonikers)
        /* [restricted][hidden][id] */ HRESULT ( STDMETHODCALLTYPE *EnumDeviceMonikers )( 
            __RPC__in IATSCTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumMoniker **ppEnum);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_DefaultPreferredComponentTypes)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DefaultPreferredComponentTypes )( 
            __RPC__in IATSCTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt IComponentTypes **ComponentTypes);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_DefaultPreferredComponentTypes)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DefaultPreferredComponentTypes )( 
            __RPC__in IATSCTuningSpace * This,
            /* [in] */ __RPC__in_opt IComponentTypes *NewComponentTypes);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_FrequencyMapping)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FrequencyMapping )( 
            __RPC__in IATSCTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pMapping);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_FrequencyMapping)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_FrequencyMapping )( 
            __RPC__in IATSCTuningSpace * This,
            __RPC__in BSTR Mapping);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_DefaultLocator)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DefaultLocator )( 
            __RPC__in IATSCTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt ILocator **LocatorVal);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_DefaultLocator)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DefaultLocator )( 
            __RPC__in IATSCTuningSpace * This,
            /* [in] */ __RPC__in_opt ILocator *LocatorVal);
        
        DECLSPEC_XFGVIRT(ITuningSpace, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IATSCTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt ITuningSpace **NewTS);
        
        DECLSPEC_XFGVIRT(IAnalogTVTuningSpace, get_MinChannel)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MinChannel )( 
            __RPC__in IATSCTuningSpace * This,
            /* [retval][out] */ __RPC__out long *MinChannelVal);
        
        DECLSPEC_XFGVIRT(IAnalogTVTuningSpace, put_MinChannel)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MinChannel )( 
            __RPC__in IATSCTuningSpace * This,
            /* [in] */ long NewMinChannelVal);
        
        DECLSPEC_XFGVIRT(IAnalogTVTuningSpace, get_MaxChannel)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MaxChannel )( 
            __RPC__in IATSCTuningSpace * This,
            /* [retval][out] */ __RPC__out long *MaxChannelVal);
        
        DECLSPEC_XFGVIRT(IAnalogTVTuningSpace, put_MaxChannel)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MaxChannel )( 
            __RPC__in IATSCTuningSpace * This,
            /* [in] */ long NewMaxChannelVal);
        
        DECLSPEC_XFGVIRT(IAnalogTVTuningSpace, get_InputType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_InputType )( 
            __RPC__in IATSCTuningSpace * This,
            /* [retval][out] */ __RPC__out TunerInputType *InputTypeVal);
        
        DECLSPEC_XFGVIRT(IAnalogTVTuningSpace, put_InputType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_InputType )( 
            __RPC__in IATSCTuningSpace * This,
            /* [in] */ TunerInputType NewInputTypeVal);
        
        DECLSPEC_XFGVIRT(IAnalogTVTuningSpace, get_CountryCode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CountryCode )( 
            __RPC__in IATSCTuningSpace * This,
            /* [retval][out] */ __RPC__out long *CountryCodeVal);
        
        DECLSPEC_XFGVIRT(IAnalogTVTuningSpace, put_CountryCode)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_CountryCode )( 
            __RPC__in IATSCTuningSpace * This,
            /* [in] */ long NewCountryCodeVal);
        
        DECLSPEC_XFGVIRT(IATSCTuningSpace, get_MinMinorChannel)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MinMinorChannel )( 
            __RPC__in IATSCTuningSpace * This,
            /* [retval][out] */ __RPC__out long *MinMinorChannelVal);
        
        DECLSPEC_XFGVIRT(IATSCTuningSpace, put_MinMinorChannel)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MinMinorChannel )( 
            __RPC__in IATSCTuningSpace * This,
            /* [in] */ long NewMinMinorChannelVal);
        
        DECLSPEC_XFGVIRT(IATSCTuningSpace, get_MaxMinorChannel)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MaxMinorChannel )( 
            __RPC__in IATSCTuningSpace * This,
            /* [retval][out] */ __RPC__out long *MaxMinorChannelVal);
        
        DECLSPEC_XFGVIRT(IATSCTuningSpace, put_MaxMinorChannel)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MaxMinorChannel )( 
            __RPC__in IATSCTuningSpace * This,
            /* [in] */ long NewMaxMinorChannelVal);
        
        DECLSPEC_XFGVIRT(IATSCTuningSpace, get_MinPhysicalChannel)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MinPhysicalChannel )( 
            __RPC__in IATSCTuningSpace * This,
            /* [retval][out] */ __RPC__out long *MinPhysicalChannelVal);
        
        DECLSPEC_XFGVIRT(IATSCTuningSpace, put_MinPhysicalChannel)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MinPhysicalChannel )( 
            __RPC__in IATSCTuningSpace * This,
            /* [in] */ long NewMinPhysicalChannelVal);
        
        DECLSPEC_XFGVIRT(IATSCTuningSpace, get_MaxPhysicalChannel)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MaxPhysicalChannel )( 
            __RPC__in IATSCTuningSpace * This,
            /* [retval][out] */ __RPC__out long *MaxPhysicalChannelVal);
        
        DECLSPEC_XFGVIRT(IATSCTuningSpace, put_MaxPhysicalChannel)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MaxPhysicalChannel )( 
            __RPC__in IATSCTuningSpace * This,
            /* [in] */ long NewMaxPhysicalChannelVal);
        
        END_INTERFACE
    } IATSCTuningSpaceVtbl;

    interface IATSCTuningSpace
    {
        CONST_VTBL struct IATSCTuningSpaceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IATSCTuningSpace_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IATSCTuningSpace_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IATSCTuningSpace_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IATSCTuningSpace_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IATSCTuningSpace_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IATSCTuningSpace_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IATSCTuningSpace_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IATSCTuningSpace_get_UniqueName(This,Name)	\
    ( (This)->lpVtbl -> get_UniqueName(This,Name) ) 

#define IATSCTuningSpace_put_UniqueName(This,Name)	\
    ( (This)->lpVtbl -> put_UniqueName(This,Name) ) 

#define IATSCTuningSpace_get_FriendlyName(This,Name)	\
    ( (This)->lpVtbl -> get_FriendlyName(This,Name) ) 

#define IATSCTuningSpace_put_FriendlyName(This,Name)	\
    ( (This)->lpVtbl -> put_FriendlyName(This,Name) ) 

#define IATSCTuningSpace_get_CLSID(This,SpaceCLSID)	\
    ( (This)->lpVtbl -> get_CLSID(This,SpaceCLSID) ) 

#define IATSCTuningSpace_get_NetworkType(This,NetworkTypeGuid)	\
    ( (This)->lpVtbl -> get_NetworkType(This,NetworkTypeGuid) ) 

#define IATSCTuningSpace_put_NetworkType(This,NetworkTypeGuid)	\
    ( (This)->lpVtbl -> put_NetworkType(This,NetworkTypeGuid) ) 

#define IATSCTuningSpace_get__NetworkType(This,NetworkTypeGuid)	\
    ( (This)->lpVtbl -> get__NetworkType(This,NetworkTypeGuid) ) 

#define IATSCTuningSpace_put__NetworkType(This,NetworkTypeGuid)	\
    ( (This)->lpVtbl -> put__NetworkType(This,NetworkTypeGuid) ) 

#define IATSCTuningSpace_CreateTuneRequest(This,TuneRequest)	\
    ( (This)->lpVtbl -> CreateTuneRequest(This,TuneRequest) ) 

#define IATSCTuningSpace_EnumCategoryGUIDs(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumCategoryGUIDs(This,ppEnum) ) 

#define IATSCTuningSpace_EnumDeviceMonikers(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumDeviceMonikers(This,ppEnum) ) 

#define IATSCTuningSpace_get_DefaultPreferredComponentTypes(This,ComponentTypes)	\
    ( (This)->lpVtbl -> get_DefaultPreferredComponentTypes(This,ComponentTypes) ) 

#define IATSCTuningSpace_put_DefaultPreferredComponentTypes(This,NewComponentTypes)	\
    ( (This)->lpVtbl -> put_DefaultPreferredComponentTypes(This,NewComponentTypes) ) 

#define IATSCTuningSpace_get_FrequencyMapping(This,pMapping)	\
    ( (This)->lpVtbl -> get_FrequencyMapping(This,pMapping) ) 

#define IATSCTuningSpace_put_FrequencyMapping(This,Mapping)	\
    ( (This)->lpVtbl -> put_FrequencyMapping(This,Mapping) ) 

#define IATSCTuningSpace_get_DefaultLocator(This,LocatorVal)	\
    ( (This)->lpVtbl -> get_DefaultLocator(This,LocatorVal) ) 

#define IATSCTuningSpace_put_DefaultLocator(This,LocatorVal)	\
    ( (This)->lpVtbl -> put_DefaultLocator(This,LocatorVal) ) 

#define IATSCTuningSpace_Clone(This,NewTS)	\
    ( (This)->lpVtbl -> Clone(This,NewTS) ) 


#define IATSCTuningSpace_get_MinChannel(This,MinChannelVal)	\
    ( (This)->lpVtbl -> get_MinChannel(This,MinChannelVal) ) 

#define IATSCTuningSpace_put_MinChannel(This,NewMinChannelVal)	\
    ( (This)->lpVtbl -> put_MinChannel(This,NewMinChannelVal) ) 

#define IATSCTuningSpace_get_MaxChannel(This,MaxChannelVal)	\
    ( (This)->lpVtbl -> get_MaxChannel(This,MaxChannelVal) ) 

#define IATSCTuningSpace_put_MaxChannel(This,NewMaxChannelVal)	\
    ( (This)->lpVtbl -> put_MaxChannel(This,NewMaxChannelVal) ) 

#define IATSCTuningSpace_get_InputType(This,InputTypeVal)	\
    ( (This)->lpVtbl -> get_InputType(This,InputTypeVal) ) 

#define IATSCTuningSpace_put_InputType(This,NewInputTypeVal)	\
    ( (This)->lpVtbl -> put_InputType(This,NewInputTypeVal) ) 

#define IATSCTuningSpace_get_CountryCode(This,CountryCodeVal)	\
    ( (This)->lpVtbl -> get_CountryCode(This,CountryCodeVal) ) 

#define IATSCTuningSpace_put_CountryCode(This,NewCountryCodeVal)	\
    ( (This)->lpVtbl -> put_CountryCode(This,NewCountryCodeVal) ) 


#define IATSCTuningSpace_get_MinMinorChannel(This,MinMinorChannelVal)	\
    ( (This)->lpVtbl -> get_MinMinorChannel(This,MinMinorChannelVal) ) 

#define IATSCTuningSpace_put_MinMinorChannel(This,NewMinMinorChannelVal)	\
    ( (This)->lpVtbl -> put_MinMinorChannel(This,NewMinMinorChannelVal) ) 

#define IATSCTuningSpace_get_MaxMinorChannel(This,MaxMinorChannelVal)	\
    ( (This)->lpVtbl -> get_MaxMinorChannel(This,MaxMinorChannelVal) ) 

#define IATSCTuningSpace_put_MaxMinorChannel(This,NewMaxMinorChannelVal)	\
    ( (This)->lpVtbl -> put_MaxMinorChannel(This,NewMaxMinorChannelVal) ) 

#define IATSCTuningSpace_get_MinPhysicalChannel(This,MinPhysicalChannelVal)	\
    ( (This)->lpVtbl -> get_MinPhysicalChannel(This,MinPhysicalChannelVal) ) 

#define IATSCTuningSpace_put_MinPhysicalChannel(This,NewMinPhysicalChannelVal)	\
    ( (This)->lpVtbl -> put_MinPhysicalChannel(This,NewMinPhysicalChannelVal) ) 

#define IATSCTuningSpace_get_MaxPhysicalChannel(This,MaxPhysicalChannelVal)	\
    ( (This)->lpVtbl -> get_MaxPhysicalChannel(This,MaxPhysicalChannelVal) ) 

#define IATSCTuningSpace_put_MaxPhysicalChannel(This,NewMaxPhysicalChannelVal)	\
    ( (This)->lpVtbl -> put_MaxPhysicalChannel(This,NewMaxPhysicalChannelVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IATSCTuningSpace_INTERFACE_DEFINED__ */


#ifndef __IDigitalCableTuningSpace_INTERFACE_DEFINED__
#define __IDigitalCableTuningSpace_INTERFACE_DEFINED__

/* interface IDigitalCableTuningSpace */
/* [unique][uuid][nonextensible][proxy][oleautomation][dual][hidden][object] */ 


EXTERN_C const IID IID_IDigitalCableTuningSpace;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("013F9F9C-B449-4ec7-A6D2-9D4F2FC70AE5")
    IDigitalCableTuningSpace : public IATSCTuningSpace
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MinMajorChannel( 
            /* [retval][out] */ __RPC__out long *MinMajorChannelVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_MinMajorChannel( 
            /* [in] */ long NewMinMajorChannelVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MaxMajorChannel( 
            /* [retval][out] */ __RPC__out long *MaxMajorChannelVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_MaxMajorChannel( 
            /* [in] */ long NewMaxMajorChannelVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MinSourceID( 
            /* [retval][out] */ __RPC__out long *MinSourceIDVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_MinSourceID( 
            /* [in] */ long NewMinSourceIDVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MaxSourceID( 
            /* [retval][out] */ __RPC__out long *MaxSourceIDVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_MaxSourceID( 
            /* [in] */ long NewMaxSourceIDVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDigitalCableTuningSpaceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDigitalCableTuningSpace * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDigitalCableTuningSpace * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDigitalCableTuningSpace * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IDigitalCableTuningSpace * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IDigitalCableTuningSpace * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IDigitalCableTuningSpace * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IDigitalCableTuningSpace * This,
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
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_UniqueName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UniqueName )( 
            __RPC__in IDigitalCableTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Name);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_UniqueName)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_UniqueName )( 
            __RPC__in IDigitalCableTuningSpace * This,
            /* [in] */ __RPC__in BSTR Name);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_FriendlyName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FriendlyName )( 
            __RPC__in IDigitalCableTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Name);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_FriendlyName)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_FriendlyName )( 
            __RPC__in IDigitalCableTuningSpace * This,
            /* [in] */ __RPC__in BSTR Name);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_CLSID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CLSID )( 
            __RPC__in IDigitalCableTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *SpaceCLSID);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_NetworkType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NetworkType )( 
            __RPC__in IDigitalCableTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *NetworkTypeGuid);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_NetworkType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_NetworkType )( 
            __RPC__in IDigitalCableTuningSpace * This,
            /* [in] */ __RPC__in BSTR NetworkTypeGuid);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get__NetworkType)
        /* [restricted][hidden][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NetworkType )( 
            __RPC__in IDigitalCableTuningSpace * This,
            /* [retval][out] */ __RPC__out GUID *NetworkTypeGuid);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put__NetworkType)
        /* [restricted][hidden][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put__NetworkType )( 
            __RPC__in IDigitalCableTuningSpace * This,
            /* [in] */ __RPC__in REFCLSID NetworkTypeGuid);
        
        DECLSPEC_XFGVIRT(ITuningSpace, CreateTuneRequest)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateTuneRequest )( 
            __RPC__in IDigitalCableTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt ITuneRequest **TuneRequest);
        
        DECLSPEC_XFGVIRT(ITuningSpace, EnumCategoryGUIDs)
        /* [restricted][hidden][id] */ HRESULT ( STDMETHODCALLTYPE *EnumCategoryGUIDs )( 
            __RPC__in IDigitalCableTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumGUID **ppEnum);
        
        DECLSPEC_XFGVIRT(ITuningSpace, EnumDeviceMonikers)
        /* [restricted][hidden][id] */ HRESULT ( STDMETHODCALLTYPE *EnumDeviceMonikers )( 
            __RPC__in IDigitalCableTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumMoniker **ppEnum);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_DefaultPreferredComponentTypes)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DefaultPreferredComponentTypes )( 
            __RPC__in IDigitalCableTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt IComponentTypes **ComponentTypes);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_DefaultPreferredComponentTypes)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DefaultPreferredComponentTypes )( 
            __RPC__in IDigitalCableTuningSpace * This,
            /* [in] */ __RPC__in_opt IComponentTypes *NewComponentTypes);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_FrequencyMapping)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FrequencyMapping )( 
            __RPC__in IDigitalCableTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pMapping);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_FrequencyMapping)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_FrequencyMapping )( 
            __RPC__in IDigitalCableTuningSpace * This,
            __RPC__in BSTR Mapping);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_DefaultLocator)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DefaultLocator )( 
            __RPC__in IDigitalCableTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt ILocator **LocatorVal);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_DefaultLocator)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DefaultLocator )( 
            __RPC__in IDigitalCableTuningSpace * This,
            /* [in] */ __RPC__in_opt ILocator *LocatorVal);
        
        DECLSPEC_XFGVIRT(ITuningSpace, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IDigitalCableTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt ITuningSpace **NewTS);
        
        DECLSPEC_XFGVIRT(IAnalogTVTuningSpace, get_MinChannel)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MinChannel )( 
            __RPC__in IDigitalCableTuningSpace * This,
            /* [retval][out] */ __RPC__out long *MinChannelVal);
        
        DECLSPEC_XFGVIRT(IAnalogTVTuningSpace, put_MinChannel)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MinChannel )( 
            __RPC__in IDigitalCableTuningSpace * This,
            /* [in] */ long NewMinChannelVal);
        
        DECLSPEC_XFGVIRT(IAnalogTVTuningSpace, get_MaxChannel)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MaxChannel )( 
            __RPC__in IDigitalCableTuningSpace * This,
            /* [retval][out] */ __RPC__out long *MaxChannelVal);
        
        DECLSPEC_XFGVIRT(IAnalogTVTuningSpace, put_MaxChannel)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MaxChannel )( 
            __RPC__in IDigitalCableTuningSpace * This,
            /* [in] */ long NewMaxChannelVal);
        
        DECLSPEC_XFGVIRT(IAnalogTVTuningSpace, get_InputType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_InputType )( 
            __RPC__in IDigitalCableTuningSpace * This,
            /* [retval][out] */ __RPC__out TunerInputType *InputTypeVal);
        
        DECLSPEC_XFGVIRT(IAnalogTVTuningSpace, put_InputType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_InputType )( 
            __RPC__in IDigitalCableTuningSpace * This,
            /* [in] */ TunerInputType NewInputTypeVal);
        
        DECLSPEC_XFGVIRT(IAnalogTVTuningSpace, get_CountryCode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CountryCode )( 
            __RPC__in IDigitalCableTuningSpace * This,
            /* [retval][out] */ __RPC__out long *CountryCodeVal);
        
        DECLSPEC_XFGVIRT(IAnalogTVTuningSpace, put_CountryCode)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_CountryCode )( 
            __RPC__in IDigitalCableTuningSpace * This,
            /* [in] */ long NewCountryCodeVal);
        
        DECLSPEC_XFGVIRT(IATSCTuningSpace, get_MinMinorChannel)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MinMinorChannel )( 
            __RPC__in IDigitalCableTuningSpace * This,
            /* [retval][out] */ __RPC__out long *MinMinorChannelVal);
        
        DECLSPEC_XFGVIRT(IATSCTuningSpace, put_MinMinorChannel)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MinMinorChannel )( 
            __RPC__in IDigitalCableTuningSpace * This,
            /* [in] */ long NewMinMinorChannelVal);
        
        DECLSPEC_XFGVIRT(IATSCTuningSpace, get_MaxMinorChannel)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MaxMinorChannel )( 
            __RPC__in IDigitalCableTuningSpace * This,
            /* [retval][out] */ __RPC__out long *MaxMinorChannelVal);
        
        DECLSPEC_XFGVIRT(IATSCTuningSpace, put_MaxMinorChannel)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MaxMinorChannel )( 
            __RPC__in IDigitalCableTuningSpace * This,
            /* [in] */ long NewMaxMinorChannelVal);
        
        DECLSPEC_XFGVIRT(IATSCTuningSpace, get_MinPhysicalChannel)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MinPhysicalChannel )( 
            __RPC__in IDigitalCableTuningSpace * This,
            /* [retval][out] */ __RPC__out long *MinPhysicalChannelVal);
        
        DECLSPEC_XFGVIRT(IATSCTuningSpace, put_MinPhysicalChannel)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MinPhysicalChannel )( 
            __RPC__in IDigitalCableTuningSpace * This,
            /* [in] */ long NewMinPhysicalChannelVal);
        
        DECLSPEC_XFGVIRT(IATSCTuningSpace, get_MaxPhysicalChannel)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MaxPhysicalChannel )( 
            __RPC__in IDigitalCableTuningSpace * This,
            /* [retval][out] */ __RPC__out long *MaxPhysicalChannelVal);
        
        DECLSPEC_XFGVIRT(IATSCTuningSpace, put_MaxPhysicalChannel)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MaxPhysicalChannel )( 
            __RPC__in IDigitalCableTuningSpace * This,
            /* [in] */ long NewMaxPhysicalChannelVal);
        
        DECLSPEC_XFGVIRT(IDigitalCableTuningSpace, get_MinMajorChannel)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MinMajorChannel )( 
            __RPC__in IDigitalCableTuningSpace * This,
            /* [retval][out] */ __RPC__out long *MinMajorChannelVal);
        
        DECLSPEC_XFGVIRT(IDigitalCableTuningSpace, put_MinMajorChannel)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MinMajorChannel )( 
            __RPC__in IDigitalCableTuningSpace * This,
            /* [in] */ long NewMinMajorChannelVal);
        
        DECLSPEC_XFGVIRT(IDigitalCableTuningSpace, get_MaxMajorChannel)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MaxMajorChannel )( 
            __RPC__in IDigitalCableTuningSpace * This,
            /* [retval][out] */ __RPC__out long *MaxMajorChannelVal);
        
        DECLSPEC_XFGVIRT(IDigitalCableTuningSpace, put_MaxMajorChannel)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MaxMajorChannel )( 
            __RPC__in IDigitalCableTuningSpace * This,
            /* [in] */ long NewMaxMajorChannelVal);
        
        DECLSPEC_XFGVIRT(IDigitalCableTuningSpace, get_MinSourceID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MinSourceID )( 
            __RPC__in IDigitalCableTuningSpace * This,
            /* [retval][out] */ __RPC__out long *MinSourceIDVal);
        
        DECLSPEC_XFGVIRT(IDigitalCableTuningSpace, put_MinSourceID)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MinSourceID )( 
            __RPC__in IDigitalCableTuningSpace * This,
            /* [in] */ long NewMinSourceIDVal);
        
        DECLSPEC_XFGVIRT(IDigitalCableTuningSpace, get_MaxSourceID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MaxSourceID )( 
            __RPC__in IDigitalCableTuningSpace * This,
            /* [retval][out] */ __RPC__out long *MaxSourceIDVal);
        
        DECLSPEC_XFGVIRT(IDigitalCableTuningSpace, put_MaxSourceID)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MaxSourceID )( 
            __RPC__in IDigitalCableTuningSpace * This,
            /* [in] */ long NewMaxSourceIDVal);
        
        END_INTERFACE
    } IDigitalCableTuningSpaceVtbl;

    interface IDigitalCableTuningSpace
    {
        CONST_VTBL struct IDigitalCableTuningSpaceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDigitalCableTuningSpace_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDigitalCableTuningSpace_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDigitalCableTuningSpace_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDigitalCableTuningSpace_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IDigitalCableTuningSpace_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IDigitalCableTuningSpace_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IDigitalCableTuningSpace_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IDigitalCableTuningSpace_get_UniqueName(This,Name)	\
    ( (This)->lpVtbl -> get_UniqueName(This,Name) ) 

#define IDigitalCableTuningSpace_put_UniqueName(This,Name)	\
    ( (This)->lpVtbl -> put_UniqueName(This,Name) ) 

#define IDigitalCableTuningSpace_get_FriendlyName(This,Name)	\
    ( (This)->lpVtbl -> get_FriendlyName(This,Name) ) 

#define IDigitalCableTuningSpace_put_FriendlyName(This,Name)	\
    ( (This)->lpVtbl -> put_FriendlyName(This,Name) ) 

#define IDigitalCableTuningSpace_get_CLSID(This,SpaceCLSID)	\
    ( (This)->lpVtbl -> get_CLSID(This,SpaceCLSID) ) 

#define IDigitalCableTuningSpace_get_NetworkType(This,NetworkTypeGuid)	\
    ( (This)->lpVtbl -> get_NetworkType(This,NetworkTypeGuid) ) 

#define IDigitalCableTuningSpace_put_NetworkType(This,NetworkTypeGuid)	\
    ( (This)->lpVtbl -> put_NetworkType(This,NetworkTypeGuid) ) 

#define IDigitalCableTuningSpace_get__NetworkType(This,NetworkTypeGuid)	\
    ( (This)->lpVtbl -> get__NetworkType(This,NetworkTypeGuid) ) 

#define IDigitalCableTuningSpace_put__NetworkType(This,NetworkTypeGuid)	\
    ( (This)->lpVtbl -> put__NetworkType(This,NetworkTypeGuid) ) 

#define IDigitalCableTuningSpace_CreateTuneRequest(This,TuneRequest)	\
    ( (This)->lpVtbl -> CreateTuneRequest(This,TuneRequest) ) 

#define IDigitalCableTuningSpace_EnumCategoryGUIDs(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumCategoryGUIDs(This,ppEnum) ) 

#define IDigitalCableTuningSpace_EnumDeviceMonikers(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumDeviceMonikers(This,ppEnum) ) 

#define IDigitalCableTuningSpace_get_DefaultPreferredComponentTypes(This,ComponentTypes)	\
    ( (This)->lpVtbl -> get_DefaultPreferredComponentTypes(This,ComponentTypes) ) 

#define IDigitalCableTuningSpace_put_DefaultPreferredComponentTypes(This,NewComponentTypes)	\
    ( (This)->lpVtbl -> put_DefaultPreferredComponentTypes(This,NewComponentTypes) ) 

#define IDigitalCableTuningSpace_get_FrequencyMapping(This,pMapping)	\
    ( (This)->lpVtbl -> get_FrequencyMapping(This,pMapping) ) 

#define IDigitalCableTuningSpace_put_FrequencyMapping(This,Mapping)	\
    ( (This)->lpVtbl -> put_FrequencyMapping(This,Mapping) ) 

#define IDigitalCableTuningSpace_get_DefaultLocator(This,LocatorVal)	\
    ( (This)->lpVtbl -> get_DefaultLocator(This,LocatorVal) ) 

#define IDigitalCableTuningSpace_put_DefaultLocator(This,LocatorVal)	\
    ( (This)->lpVtbl -> put_DefaultLocator(This,LocatorVal) ) 

#define IDigitalCableTuningSpace_Clone(This,NewTS)	\
    ( (This)->lpVtbl -> Clone(This,NewTS) ) 


#define IDigitalCableTuningSpace_get_MinChannel(This,MinChannelVal)	\
    ( (This)->lpVtbl -> get_MinChannel(This,MinChannelVal) ) 

#define IDigitalCableTuningSpace_put_MinChannel(This,NewMinChannelVal)	\
    ( (This)->lpVtbl -> put_MinChannel(This,NewMinChannelVal) ) 

#define IDigitalCableTuningSpace_get_MaxChannel(This,MaxChannelVal)	\
    ( (This)->lpVtbl -> get_MaxChannel(This,MaxChannelVal) ) 

#define IDigitalCableTuningSpace_put_MaxChannel(This,NewMaxChannelVal)	\
    ( (This)->lpVtbl -> put_MaxChannel(This,NewMaxChannelVal) ) 

#define IDigitalCableTuningSpace_get_InputType(This,InputTypeVal)	\
    ( (This)->lpVtbl -> get_InputType(This,InputTypeVal) ) 

#define IDigitalCableTuningSpace_put_InputType(This,NewInputTypeVal)	\
    ( (This)->lpVtbl -> put_InputType(This,NewInputTypeVal) ) 

#define IDigitalCableTuningSpace_get_CountryCode(This,CountryCodeVal)	\
    ( (This)->lpVtbl -> get_CountryCode(This,CountryCodeVal) ) 

#define IDigitalCableTuningSpace_put_CountryCode(This,NewCountryCodeVal)	\
    ( (This)->lpVtbl -> put_CountryCode(This,NewCountryCodeVal) ) 


#define IDigitalCableTuningSpace_get_MinMinorChannel(This,MinMinorChannelVal)	\
    ( (This)->lpVtbl -> get_MinMinorChannel(This,MinMinorChannelVal) ) 

#define IDigitalCableTuningSpace_put_MinMinorChannel(This,NewMinMinorChannelVal)	\
    ( (This)->lpVtbl -> put_MinMinorChannel(This,NewMinMinorChannelVal) ) 

#define IDigitalCableTuningSpace_get_MaxMinorChannel(This,MaxMinorChannelVal)	\
    ( (This)->lpVtbl -> get_MaxMinorChannel(This,MaxMinorChannelVal) ) 

#define IDigitalCableTuningSpace_put_MaxMinorChannel(This,NewMaxMinorChannelVal)	\
    ( (This)->lpVtbl -> put_MaxMinorChannel(This,NewMaxMinorChannelVal) ) 

#define IDigitalCableTuningSpace_get_MinPhysicalChannel(This,MinPhysicalChannelVal)	\
    ( (This)->lpVtbl -> get_MinPhysicalChannel(This,MinPhysicalChannelVal) ) 

#define IDigitalCableTuningSpace_put_MinPhysicalChannel(This,NewMinPhysicalChannelVal)	\
    ( (This)->lpVtbl -> put_MinPhysicalChannel(This,NewMinPhysicalChannelVal) ) 

#define IDigitalCableTuningSpace_get_MaxPhysicalChannel(This,MaxPhysicalChannelVal)	\
    ( (This)->lpVtbl -> get_MaxPhysicalChannel(This,MaxPhysicalChannelVal) ) 

#define IDigitalCableTuningSpace_put_MaxPhysicalChannel(This,NewMaxPhysicalChannelVal)	\
    ( (This)->lpVtbl -> put_MaxPhysicalChannel(This,NewMaxPhysicalChannelVal) ) 


#define IDigitalCableTuningSpace_get_MinMajorChannel(This,MinMajorChannelVal)	\
    ( (This)->lpVtbl -> get_MinMajorChannel(This,MinMajorChannelVal) ) 

#define IDigitalCableTuningSpace_put_MinMajorChannel(This,NewMinMajorChannelVal)	\
    ( (This)->lpVtbl -> put_MinMajorChannel(This,NewMinMajorChannelVal) ) 

#define IDigitalCableTuningSpace_get_MaxMajorChannel(This,MaxMajorChannelVal)	\
    ( (This)->lpVtbl -> get_MaxMajorChannel(This,MaxMajorChannelVal) ) 

#define IDigitalCableTuningSpace_put_MaxMajorChannel(This,NewMaxMajorChannelVal)	\
    ( (This)->lpVtbl -> put_MaxMajorChannel(This,NewMaxMajorChannelVal) ) 

#define IDigitalCableTuningSpace_get_MinSourceID(This,MinSourceIDVal)	\
    ( (This)->lpVtbl -> get_MinSourceID(This,MinSourceIDVal) ) 

#define IDigitalCableTuningSpace_put_MinSourceID(This,NewMinSourceIDVal)	\
    ( (This)->lpVtbl -> put_MinSourceID(This,NewMinSourceIDVal) ) 

#define IDigitalCableTuningSpace_get_MaxSourceID(This,MaxSourceIDVal)	\
    ( (This)->lpVtbl -> get_MaxSourceID(This,MaxSourceIDVal) ) 

#define IDigitalCableTuningSpace_put_MaxSourceID(This,NewMaxSourceIDVal)	\
    ( (This)->lpVtbl -> put_MaxSourceID(This,NewMaxSourceIDVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDigitalCableTuningSpace_INTERFACE_DEFINED__ */


#ifndef __IAnalogRadioTuningSpace_INTERFACE_DEFINED__
#define __IAnalogRadioTuningSpace_INTERFACE_DEFINED__

/* interface IAnalogRadioTuningSpace */
/* [unique][uuid][nonextensible][proxy][oleautomation][dual][hidden][object] */ 


EXTERN_C const IID IID_IAnalogRadioTuningSpace;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2A6E293B-2595-11d3-B64C-00C04F79498E")
    IAnalogRadioTuningSpace : public ITuningSpace
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MinFrequency( 
            /* [retval][out] */ __RPC__out long *MinFrequencyVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_MinFrequency( 
            /* [in] */ long NewMinFrequencyVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MaxFrequency( 
            /* [retval][out] */ __RPC__out long *MaxFrequencyVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_MaxFrequency( 
            /* [in] */ long NewMaxFrequencyVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Step( 
            /* [retval][out] */ __RPC__out long *StepVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Step( 
            /* [in] */ long NewStepVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAnalogRadioTuningSpaceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAnalogRadioTuningSpace * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAnalogRadioTuningSpace * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAnalogRadioTuningSpace * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IAnalogRadioTuningSpace * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IAnalogRadioTuningSpace * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IAnalogRadioTuningSpace * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IAnalogRadioTuningSpace * This,
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
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_UniqueName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UniqueName )( 
            __RPC__in IAnalogRadioTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Name);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_UniqueName)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_UniqueName )( 
            __RPC__in IAnalogRadioTuningSpace * This,
            /* [in] */ __RPC__in BSTR Name);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_FriendlyName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FriendlyName )( 
            __RPC__in IAnalogRadioTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Name);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_FriendlyName)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_FriendlyName )( 
            __RPC__in IAnalogRadioTuningSpace * This,
            /* [in] */ __RPC__in BSTR Name);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_CLSID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CLSID )( 
            __RPC__in IAnalogRadioTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *SpaceCLSID);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_NetworkType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NetworkType )( 
            __RPC__in IAnalogRadioTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *NetworkTypeGuid);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_NetworkType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_NetworkType )( 
            __RPC__in IAnalogRadioTuningSpace * This,
            /* [in] */ __RPC__in BSTR NetworkTypeGuid);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get__NetworkType)
        /* [restricted][hidden][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NetworkType )( 
            __RPC__in IAnalogRadioTuningSpace * This,
            /* [retval][out] */ __RPC__out GUID *NetworkTypeGuid);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put__NetworkType)
        /* [restricted][hidden][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put__NetworkType )( 
            __RPC__in IAnalogRadioTuningSpace * This,
            /* [in] */ __RPC__in REFCLSID NetworkTypeGuid);
        
        DECLSPEC_XFGVIRT(ITuningSpace, CreateTuneRequest)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateTuneRequest )( 
            __RPC__in IAnalogRadioTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt ITuneRequest **TuneRequest);
        
        DECLSPEC_XFGVIRT(ITuningSpace, EnumCategoryGUIDs)
        /* [restricted][hidden][id] */ HRESULT ( STDMETHODCALLTYPE *EnumCategoryGUIDs )( 
            __RPC__in IAnalogRadioTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumGUID **ppEnum);
        
        DECLSPEC_XFGVIRT(ITuningSpace, EnumDeviceMonikers)
        /* [restricted][hidden][id] */ HRESULT ( STDMETHODCALLTYPE *EnumDeviceMonikers )( 
            __RPC__in IAnalogRadioTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumMoniker **ppEnum);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_DefaultPreferredComponentTypes)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DefaultPreferredComponentTypes )( 
            __RPC__in IAnalogRadioTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt IComponentTypes **ComponentTypes);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_DefaultPreferredComponentTypes)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DefaultPreferredComponentTypes )( 
            __RPC__in IAnalogRadioTuningSpace * This,
            /* [in] */ __RPC__in_opt IComponentTypes *NewComponentTypes);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_FrequencyMapping)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FrequencyMapping )( 
            __RPC__in IAnalogRadioTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pMapping);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_FrequencyMapping)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_FrequencyMapping )( 
            __RPC__in IAnalogRadioTuningSpace * This,
            __RPC__in BSTR Mapping);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_DefaultLocator)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DefaultLocator )( 
            __RPC__in IAnalogRadioTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt ILocator **LocatorVal);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_DefaultLocator)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DefaultLocator )( 
            __RPC__in IAnalogRadioTuningSpace * This,
            /* [in] */ __RPC__in_opt ILocator *LocatorVal);
        
        DECLSPEC_XFGVIRT(ITuningSpace, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IAnalogRadioTuningSpace * This,
            /* [retval][out] */ __RPC__deref_out_opt ITuningSpace **NewTS);
        
        DECLSPEC_XFGVIRT(IAnalogRadioTuningSpace, get_MinFrequency)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MinFrequency )( 
            __RPC__in IAnalogRadioTuningSpace * This,
            /* [retval][out] */ __RPC__out long *MinFrequencyVal);
        
        DECLSPEC_XFGVIRT(IAnalogRadioTuningSpace, put_MinFrequency)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MinFrequency )( 
            __RPC__in IAnalogRadioTuningSpace * This,
            /* [in] */ long NewMinFrequencyVal);
        
        DECLSPEC_XFGVIRT(IAnalogRadioTuningSpace, get_MaxFrequency)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MaxFrequency )( 
            __RPC__in IAnalogRadioTuningSpace * This,
            /* [retval][out] */ __RPC__out long *MaxFrequencyVal);
        
        DECLSPEC_XFGVIRT(IAnalogRadioTuningSpace, put_MaxFrequency)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MaxFrequency )( 
            __RPC__in IAnalogRadioTuningSpace * This,
            /* [in] */ long NewMaxFrequencyVal);
        
        DECLSPEC_XFGVIRT(IAnalogRadioTuningSpace, get_Step)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Step )( 
            __RPC__in IAnalogRadioTuningSpace * This,
            /* [retval][out] */ __RPC__out long *StepVal);
        
        DECLSPEC_XFGVIRT(IAnalogRadioTuningSpace, put_Step)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Step )( 
            __RPC__in IAnalogRadioTuningSpace * This,
            /* [in] */ long NewStepVal);
        
        END_INTERFACE
    } IAnalogRadioTuningSpaceVtbl;

    interface IAnalogRadioTuningSpace
    {
        CONST_VTBL struct IAnalogRadioTuningSpaceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAnalogRadioTuningSpace_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAnalogRadioTuningSpace_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAnalogRadioTuningSpace_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAnalogRadioTuningSpace_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IAnalogRadioTuningSpace_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IAnalogRadioTuningSpace_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IAnalogRadioTuningSpace_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IAnalogRadioTuningSpace_get_UniqueName(This,Name)	\
    ( (This)->lpVtbl -> get_UniqueName(This,Name) ) 

#define IAnalogRadioTuningSpace_put_UniqueName(This,Name)	\
    ( (This)->lpVtbl -> put_UniqueName(This,Name) ) 

#define IAnalogRadioTuningSpace_get_FriendlyName(This,Name)	\
    ( (This)->lpVtbl -> get_FriendlyName(This,Name) ) 

#define IAnalogRadioTuningSpace_put_FriendlyName(This,Name)	\
    ( (This)->lpVtbl -> put_FriendlyName(This,Name) ) 

#define IAnalogRadioTuningSpace_get_CLSID(This,SpaceCLSID)	\
    ( (This)->lpVtbl -> get_CLSID(This,SpaceCLSID) ) 

#define IAnalogRadioTuningSpace_get_NetworkType(This,NetworkTypeGuid)	\
    ( (This)->lpVtbl -> get_NetworkType(This,NetworkTypeGuid) ) 

#define IAnalogRadioTuningSpace_put_NetworkType(This,NetworkTypeGuid)	\
    ( (This)->lpVtbl -> put_NetworkType(This,NetworkTypeGuid) ) 

#define IAnalogRadioTuningSpace_get__NetworkType(This,NetworkTypeGuid)	\
    ( (This)->lpVtbl -> get__NetworkType(This,NetworkTypeGuid) ) 

#define IAnalogRadioTuningSpace_put__NetworkType(This,NetworkTypeGuid)	\
    ( (This)->lpVtbl -> put__NetworkType(This,NetworkTypeGuid) ) 

#define IAnalogRadioTuningSpace_CreateTuneRequest(This,TuneRequest)	\
    ( (This)->lpVtbl -> CreateTuneRequest(This,TuneRequest) ) 

#define IAnalogRadioTuningSpace_EnumCategoryGUIDs(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumCategoryGUIDs(This,ppEnum) ) 

#define IAnalogRadioTuningSpace_EnumDeviceMonikers(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumDeviceMonikers(This,ppEnum) ) 

#define IAnalogRadioTuningSpace_get_DefaultPreferredComponentTypes(This,ComponentTypes)	\
    ( (This)->lpVtbl -> get_DefaultPreferredComponentTypes(This,ComponentTypes) ) 

#define IAnalogRadioTuningSpace_put_DefaultPreferredComponentTypes(This,NewComponentTypes)	\
    ( (This)->lpVtbl -> put_DefaultPreferredComponentTypes(This,NewComponentTypes) ) 

#define IAnalogRadioTuningSpace_get_FrequencyMapping(This,pMapping)	\
    ( (This)->lpVtbl -> get_FrequencyMapping(This,pMapping) ) 

#define IAnalogRadioTuningSpace_put_FrequencyMapping(This,Mapping)	\
    ( (This)->lpVtbl -> put_FrequencyMapping(This,Mapping) ) 

#define IAnalogRadioTuningSpace_get_DefaultLocator(This,LocatorVal)	\
    ( (This)->lpVtbl -> get_DefaultLocator(This,LocatorVal) ) 

#define IAnalogRadioTuningSpace_put_DefaultLocator(This,LocatorVal)	\
    ( (This)->lpVtbl -> put_DefaultLocator(This,LocatorVal) ) 

#define IAnalogRadioTuningSpace_Clone(This,NewTS)	\
    ( (This)->lpVtbl -> Clone(This,NewTS) ) 


#define IAnalogRadioTuningSpace_get_MinFrequency(This,MinFrequencyVal)	\
    ( (This)->lpVtbl -> get_MinFrequency(This,MinFrequencyVal) ) 

#define IAnalogRadioTuningSpace_put_MinFrequency(This,NewMinFrequencyVal)	\
    ( (This)->lpVtbl -> put_MinFrequency(This,NewMinFrequencyVal) ) 

#define IAnalogRadioTuningSpace_get_MaxFrequency(This,MaxFrequencyVal)	\
    ( (This)->lpVtbl -> get_MaxFrequency(This,MaxFrequencyVal) ) 

#define IAnalogRadioTuningSpace_put_MaxFrequency(This,NewMaxFrequencyVal)	\
    ( (This)->lpVtbl -> put_MaxFrequency(This,NewMaxFrequencyVal) ) 

#define IAnalogRadioTuningSpace_get_Step(This,StepVal)	\
    ( (This)->lpVtbl -> get_Step(This,StepVal) ) 

#define IAnalogRadioTuningSpace_put_Step(This,NewStepVal)	\
    ( (This)->lpVtbl -> put_Step(This,NewStepVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAnalogRadioTuningSpace_INTERFACE_DEFINED__ */


#ifndef __IAnalogRadioTuningSpace2_INTERFACE_DEFINED__
#define __IAnalogRadioTuningSpace2_INTERFACE_DEFINED__

/* interface IAnalogRadioTuningSpace2 */
/* [unique][uuid][nonextensible][proxy][oleautomation][dual][hidden][object] */ 


EXTERN_C const IID IID_IAnalogRadioTuningSpace2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("39DD45DA-2DA8-46BA-8A8A-87E2B73D983A")
    IAnalogRadioTuningSpace2 : public IAnalogRadioTuningSpace
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CountryCode( 
            /* [retval][out] */ __RPC__out long *CountryCodeVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_CountryCode( 
            /* [in] */ long NewCountryCodeVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAnalogRadioTuningSpace2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAnalogRadioTuningSpace2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAnalogRadioTuningSpace2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAnalogRadioTuningSpace2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IAnalogRadioTuningSpace2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IAnalogRadioTuningSpace2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IAnalogRadioTuningSpace2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IAnalogRadioTuningSpace2 * This,
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
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_UniqueName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UniqueName )( 
            __RPC__in IAnalogRadioTuningSpace2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Name);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_UniqueName)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_UniqueName )( 
            __RPC__in IAnalogRadioTuningSpace2 * This,
            /* [in] */ __RPC__in BSTR Name);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_FriendlyName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FriendlyName )( 
            __RPC__in IAnalogRadioTuningSpace2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Name);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_FriendlyName)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_FriendlyName )( 
            __RPC__in IAnalogRadioTuningSpace2 * This,
            /* [in] */ __RPC__in BSTR Name);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_CLSID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CLSID )( 
            __RPC__in IAnalogRadioTuningSpace2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *SpaceCLSID);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_NetworkType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NetworkType )( 
            __RPC__in IAnalogRadioTuningSpace2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *NetworkTypeGuid);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_NetworkType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_NetworkType )( 
            __RPC__in IAnalogRadioTuningSpace2 * This,
            /* [in] */ __RPC__in BSTR NetworkTypeGuid);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get__NetworkType)
        /* [restricted][hidden][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NetworkType )( 
            __RPC__in IAnalogRadioTuningSpace2 * This,
            /* [retval][out] */ __RPC__out GUID *NetworkTypeGuid);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put__NetworkType)
        /* [restricted][hidden][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put__NetworkType )( 
            __RPC__in IAnalogRadioTuningSpace2 * This,
            /* [in] */ __RPC__in REFCLSID NetworkTypeGuid);
        
        DECLSPEC_XFGVIRT(ITuningSpace, CreateTuneRequest)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateTuneRequest )( 
            __RPC__in IAnalogRadioTuningSpace2 * This,
            /* [retval][out] */ __RPC__deref_out_opt ITuneRequest **TuneRequest);
        
        DECLSPEC_XFGVIRT(ITuningSpace, EnumCategoryGUIDs)
        /* [restricted][hidden][id] */ HRESULT ( STDMETHODCALLTYPE *EnumCategoryGUIDs )( 
            __RPC__in IAnalogRadioTuningSpace2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumGUID **ppEnum);
        
        DECLSPEC_XFGVIRT(ITuningSpace, EnumDeviceMonikers)
        /* [restricted][hidden][id] */ HRESULT ( STDMETHODCALLTYPE *EnumDeviceMonikers )( 
            __RPC__in IAnalogRadioTuningSpace2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumMoniker **ppEnum);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_DefaultPreferredComponentTypes)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DefaultPreferredComponentTypes )( 
            __RPC__in IAnalogRadioTuningSpace2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IComponentTypes **ComponentTypes);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_DefaultPreferredComponentTypes)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DefaultPreferredComponentTypes )( 
            __RPC__in IAnalogRadioTuningSpace2 * This,
            /* [in] */ __RPC__in_opt IComponentTypes *NewComponentTypes);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_FrequencyMapping)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FrequencyMapping )( 
            __RPC__in IAnalogRadioTuningSpace2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pMapping);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_FrequencyMapping)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_FrequencyMapping )( 
            __RPC__in IAnalogRadioTuningSpace2 * This,
            __RPC__in BSTR Mapping);
        
        DECLSPEC_XFGVIRT(ITuningSpace, get_DefaultLocator)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DefaultLocator )( 
            __RPC__in IAnalogRadioTuningSpace2 * This,
            /* [retval][out] */ __RPC__deref_out_opt ILocator **LocatorVal);
        
        DECLSPEC_XFGVIRT(ITuningSpace, put_DefaultLocator)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DefaultLocator )( 
            __RPC__in IAnalogRadioTuningSpace2 * This,
            /* [in] */ __RPC__in_opt ILocator *LocatorVal);
        
        DECLSPEC_XFGVIRT(ITuningSpace, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IAnalogRadioTuningSpace2 * This,
            /* [retval][out] */ __RPC__deref_out_opt ITuningSpace **NewTS);
        
        DECLSPEC_XFGVIRT(IAnalogRadioTuningSpace, get_MinFrequency)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MinFrequency )( 
            __RPC__in IAnalogRadioTuningSpace2 * This,
            /* [retval][out] */ __RPC__out long *MinFrequencyVal);
        
        DECLSPEC_XFGVIRT(IAnalogRadioTuningSpace, put_MinFrequency)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MinFrequency )( 
            __RPC__in IAnalogRadioTuningSpace2 * This,
            /* [in] */ long NewMinFrequencyVal);
        
        DECLSPEC_XFGVIRT(IAnalogRadioTuningSpace, get_MaxFrequency)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MaxFrequency )( 
            __RPC__in IAnalogRadioTuningSpace2 * This,
            /* [retval][out] */ __RPC__out long *MaxFrequencyVal);
        
        DECLSPEC_XFGVIRT(IAnalogRadioTuningSpace, put_MaxFrequency)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MaxFrequency )( 
            __RPC__in IAnalogRadioTuningSpace2 * This,
            /* [in] */ long NewMaxFrequencyVal);
        
        DECLSPEC_XFGVIRT(IAnalogRadioTuningSpace, get_Step)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Step )( 
            __RPC__in IAnalogRadioTuningSpace2 * This,
            /* [retval][out] */ __RPC__out long *StepVal);
        
        DECLSPEC_XFGVIRT(IAnalogRadioTuningSpace, put_Step)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Step )( 
            __RPC__in IAnalogRadioTuningSpace2 * This,
            /* [in] */ long NewStepVal);
        
        DECLSPEC_XFGVIRT(IAnalogRadioTuningSpace2, get_CountryCode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CountryCode )( 
            __RPC__in IAnalogRadioTuningSpace2 * This,
            /* [retval][out] */ __RPC__out long *CountryCodeVal);
        
        DECLSPEC_XFGVIRT(IAnalogRadioTuningSpace2, put_CountryCode)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_CountryCode )( 
            __RPC__in IAnalogRadioTuningSpace2 * This,
            /* [in] */ long NewCountryCodeVal);
        
        END_INTERFACE
    } IAnalogRadioTuningSpace2Vtbl;

    interface IAnalogRadioTuningSpace2
    {
        CONST_VTBL struct IAnalogRadioTuningSpace2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAnalogRadioTuningSpace2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAnalogRadioTuningSpace2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAnalogRadioTuningSpace2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAnalogRadioTuningSpace2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IAnalogRadioTuningSpace2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IAnalogRadioTuningSpace2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IAnalogRadioTuningSpace2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IAnalogRadioTuningSpace2_get_UniqueName(This,Name)	\
    ( (This)->lpVtbl -> get_UniqueName(This,Name) ) 

#define IAnalogRadioTuningSpace2_put_UniqueName(This,Name)	\
    ( (This)->lpVtbl -> put_UniqueName(This,Name) ) 

#define IAnalogRadioTuningSpace2_get_FriendlyName(This,Name)	\
    ( (This)->lpVtbl -> get_FriendlyName(This,Name) ) 

#define IAnalogRadioTuningSpace2_put_FriendlyName(This,Name)	\
    ( (This)->lpVtbl -> put_FriendlyName(This,Name) ) 

#define IAnalogRadioTuningSpace2_get_CLSID(This,SpaceCLSID)	\
    ( (This)->lpVtbl -> get_CLSID(This,SpaceCLSID) ) 

#define IAnalogRadioTuningSpace2_get_NetworkType(This,NetworkTypeGuid)	\
    ( (This)->lpVtbl -> get_NetworkType(This,NetworkTypeGuid) ) 

#define IAnalogRadioTuningSpace2_put_NetworkType(This,NetworkTypeGuid)	\
    ( (This)->lpVtbl -> put_NetworkType(This,NetworkTypeGuid) ) 

#define IAnalogRadioTuningSpace2_get__NetworkType(This,NetworkTypeGuid)	\
    ( (This)->lpVtbl -> get__NetworkType(This,NetworkTypeGuid) ) 

#define IAnalogRadioTuningSpace2_put__NetworkType(This,NetworkTypeGuid)	\
    ( (This)->lpVtbl -> put__NetworkType(This,NetworkTypeGuid) ) 

#define IAnalogRadioTuningSpace2_CreateTuneRequest(This,TuneRequest)	\
    ( (This)->lpVtbl -> CreateTuneRequest(This,TuneRequest) ) 

#define IAnalogRadioTuningSpace2_EnumCategoryGUIDs(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumCategoryGUIDs(This,ppEnum) ) 

#define IAnalogRadioTuningSpace2_EnumDeviceMonikers(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumDeviceMonikers(This,ppEnum) ) 

#define IAnalogRadioTuningSpace2_get_DefaultPreferredComponentTypes(This,ComponentTypes)	\
    ( (This)->lpVtbl -> get_DefaultPreferredComponentTypes(This,ComponentTypes) ) 

#define IAnalogRadioTuningSpace2_put_DefaultPreferredComponentTypes(This,NewComponentTypes)	\
    ( (This)->lpVtbl -> put_DefaultPreferredComponentTypes(This,NewComponentTypes) ) 

#define IAnalogRadioTuningSpace2_get_FrequencyMapping(This,pMapping)	\
    ( (This)->lpVtbl -> get_FrequencyMapping(This,pMapping) ) 

#define IAnalogRadioTuningSpace2_put_FrequencyMapping(This,Mapping)	\
    ( (This)->lpVtbl -> put_FrequencyMapping(This,Mapping) ) 

#define IAnalogRadioTuningSpace2_get_DefaultLocator(This,LocatorVal)	\
    ( (This)->lpVtbl -> get_DefaultLocator(This,LocatorVal) ) 

#define IAnalogRadioTuningSpace2_put_DefaultLocator(This,LocatorVal)	\
    ( (This)->lpVtbl -> put_DefaultLocator(This,LocatorVal) ) 

#define IAnalogRadioTuningSpace2_Clone(This,NewTS)	\
    ( (This)->lpVtbl -> Clone(This,NewTS) ) 


#define IAnalogRadioTuningSpace2_get_MinFrequency(This,MinFrequencyVal)	\
    ( (This)->lpVtbl -> get_MinFrequency(This,MinFrequencyVal) ) 

#define IAnalogRadioTuningSpace2_put_MinFrequency(This,NewMinFrequencyVal)	\
    ( (This)->lpVtbl -> put_MinFrequency(This,NewMinFrequencyVal) ) 

#define IAnalogRadioTuningSpace2_get_MaxFrequency(This,MaxFrequencyVal)	\
    ( (This)->lpVtbl -> get_MaxFrequency(This,MaxFrequencyVal) ) 

#define IAnalogRadioTuningSpace2_put_MaxFrequency(This,NewMaxFrequencyVal)	\
    ( (This)->lpVtbl -> put_MaxFrequency(This,NewMaxFrequencyVal) ) 

#define IAnalogRadioTuningSpace2_get_Step(This,StepVal)	\
    ( (This)->lpVtbl -> get_Step(This,StepVal) ) 

#define IAnalogRadioTuningSpace2_put_Step(This,NewStepVal)	\
    ( (This)->lpVtbl -> put_Step(This,NewStepVal) ) 


#define IAnalogRadioTuningSpace2_get_CountryCode(This,CountryCodeVal)	\
    ( (This)->lpVtbl -> get_CountryCode(This,CountryCodeVal) ) 

#define IAnalogRadioTuningSpace2_put_CountryCode(This,NewCountryCodeVal)	\
    ( (This)->lpVtbl -> put_CountryCode(This,NewCountryCodeVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAnalogRadioTuningSpace2_INTERFACE_DEFINED__ */


#ifndef __ITuneRequest_INTERFACE_DEFINED__
#define __ITuneRequest_INTERFACE_DEFINED__

/* interface ITuneRequest */
/* [unique][helpstring][proxy][oleautomation][dual][uuid][nonextensible][object] */ 


EXTERN_C const IID IID_ITuneRequest;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("07DDC146-FC3D-11d2-9D8C-00C04F72D980")
    ITuneRequest : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TuningSpace( 
            /* [retval][out] */ __RPC__deref_out_opt ITuningSpace **TuningSpace) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Components( 
            /* [retval][out] */ __RPC__deref_out_opt IComponents **Components) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt ITuneRequest **NewTuneRequest) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Locator( 
            /* [retval][out] */ __RPC__deref_out_opt ILocator **Locator) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Locator( 
            /* [in] */ __RPC__in_opt ILocator *Locator) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITuneRequestVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITuneRequest * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITuneRequest * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITuneRequest * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITuneRequest * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITuneRequest * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITuneRequest * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITuneRequest * This,
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
        
        DECLSPEC_XFGVIRT(ITuneRequest, get_TuningSpace)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TuningSpace )( 
            __RPC__in ITuneRequest * This,
            /* [retval][out] */ __RPC__deref_out_opt ITuningSpace **TuningSpace);
        
        DECLSPEC_XFGVIRT(ITuneRequest, get_Components)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Components )( 
            __RPC__in ITuneRequest * This,
            /* [retval][out] */ __RPC__deref_out_opt IComponents **Components);
        
        DECLSPEC_XFGVIRT(ITuneRequest, Clone)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in ITuneRequest * This,
            /* [retval][out] */ __RPC__deref_out_opt ITuneRequest **NewTuneRequest);
        
        DECLSPEC_XFGVIRT(ITuneRequest, get_Locator)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Locator )( 
            __RPC__in ITuneRequest * This,
            /* [retval][out] */ __RPC__deref_out_opt ILocator **Locator);
        
        DECLSPEC_XFGVIRT(ITuneRequest, put_Locator)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Locator )( 
            __RPC__in ITuneRequest * This,
            /* [in] */ __RPC__in_opt ILocator *Locator);
        
        END_INTERFACE
    } ITuneRequestVtbl;

    interface ITuneRequest
    {
        CONST_VTBL struct ITuneRequestVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITuneRequest_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITuneRequest_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITuneRequest_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITuneRequest_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITuneRequest_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITuneRequest_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITuneRequest_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITuneRequest_get_TuningSpace(This,TuningSpace)	\
    ( (This)->lpVtbl -> get_TuningSpace(This,TuningSpace) ) 

#define ITuneRequest_get_Components(This,Components)	\
    ( (This)->lpVtbl -> get_Components(This,Components) ) 

#define ITuneRequest_Clone(This,NewTuneRequest)	\
    ( (This)->lpVtbl -> Clone(This,NewTuneRequest) ) 

#define ITuneRequest_get_Locator(This,Locator)	\
    ( (This)->lpVtbl -> get_Locator(This,Locator) ) 

#define ITuneRequest_put_Locator(This,Locator)	\
    ( (This)->lpVtbl -> put_Locator(This,Locator) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITuneRequest_INTERFACE_DEFINED__ */


#ifndef __IChannelIDTuneRequest_INTERFACE_DEFINED__
#define __IChannelIDTuneRequest_INTERFACE_DEFINED__

/* interface IChannelIDTuneRequest */
/* [unique][helpstring][oleautomation][dual][uuid][nonextensible][object] */ 


EXTERN_C const IID IID_IChannelIDTuneRequest;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("156EFF60-86F4-4e28-89FC-109799FD57EE")
    IChannelIDTuneRequest : public ITuneRequest
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ChannelID( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ChannelID) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_ChannelID( 
            /* [in] */ __RPC__in BSTR ChannelID) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IChannelIDTuneRequestVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IChannelIDTuneRequest * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IChannelIDTuneRequest * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IChannelIDTuneRequest * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IChannelIDTuneRequest * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IChannelIDTuneRequest * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IChannelIDTuneRequest * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IChannelIDTuneRequest * This,
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
        
        DECLSPEC_XFGVIRT(ITuneRequest, get_TuningSpace)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TuningSpace )( 
            __RPC__in IChannelIDTuneRequest * This,
            /* [retval][out] */ __RPC__deref_out_opt ITuningSpace **TuningSpace);
        
        DECLSPEC_XFGVIRT(ITuneRequest, get_Components)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Components )( 
            __RPC__in IChannelIDTuneRequest * This,
            /* [retval][out] */ __RPC__deref_out_opt IComponents **Components);
        
        DECLSPEC_XFGVIRT(ITuneRequest, Clone)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IChannelIDTuneRequest * This,
            /* [retval][out] */ __RPC__deref_out_opt ITuneRequest **NewTuneRequest);
        
        DECLSPEC_XFGVIRT(ITuneRequest, get_Locator)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Locator )( 
            __RPC__in IChannelIDTuneRequest * This,
            /* [retval][out] */ __RPC__deref_out_opt ILocator **Locator);
        
        DECLSPEC_XFGVIRT(ITuneRequest, put_Locator)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Locator )( 
            __RPC__in IChannelIDTuneRequest * This,
            /* [in] */ __RPC__in_opt ILocator *Locator);
        
        DECLSPEC_XFGVIRT(IChannelIDTuneRequest, get_ChannelID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ChannelID )( 
            __RPC__in IChannelIDTuneRequest * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ChannelID);
        
        DECLSPEC_XFGVIRT(IChannelIDTuneRequest, put_ChannelID)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ChannelID )( 
            __RPC__in IChannelIDTuneRequest * This,
            /* [in] */ __RPC__in BSTR ChannelID);
        
        END_INTERFACE
    } IChannelIDTuneRequestVtbl;

    interface IChannelIDTuneRequest
    {
        CONST_VTBL struct IChannelIDTuneRequestVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IChannelIDTuneRequest_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IChannelIDTuneRequest_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IChannelIDTuneRequest_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IChannelIDTuneRequest_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IChannelIDTuneRequest_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IChannelIDTuneRequest_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IChannelIDTuneRequest_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IChannelIDTuneRequest_get_TuningSpace(This,TuningSpace)	\
    ( (This)->lpVtbl -> get_TuningSpace(This,TuningSpace) ) 

#define IChannelIDTuneRequest_get_Components(This,Components)	\
    ( (This)->lpVtbl -> get_Components(This,Components) ) 

#define IChannelIDTuneRequest_Clone(This,NewTuneRequest)	\
    ( (This)->lpVtbl -> Clone(This,NewTuneRequest) ) 

#define IChannelIDTuneRequest_get_Locator(This,Locator)	\
    ( (This)->lpVtbl -> get_Locator(This,Locator) ) 

#define IChannelIDTuneRequest_put_Locator(This,Locator)	\
    ( (This)->lpVtbl -> put_Locator(This,Locator) ) 


#define IChannelIDTuneRequest_get_ChannelID(This,ChannelID)	\
    ( (This)->lpVtbl -> get_ChannelID(This,ChannelID) ) 

#define IChannelIDTuneRequest_put_ChannelID(This,ChannelID)	\
    ( (This)->lpVtbl -> put_ChannelID(This,ChannelID) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IChannelIDTuneRequest_INTERFACE_DEFINED__ */


#ifndef __IChannelTuneRequest_INTERFACE_DEFINED__
#define __IChannelTuneRequest_INTERFACE_DEFINED__

/* interface IChannelTuneRequest */
/* [unique][helpstring][proxy][oleautomation][dual][uuid][nonextensible][object] */ 


EXTERN_C const IID IID_IChannelTuneRequest;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0369B4E0-45B6-11d3-B650-00C04F79498E")
    IChannelTuneRequest : public ITuneRequest
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Channel( 
            /* [retval][out] */ __RPC__out long *Channel) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Channel( 
            /* [in] */ long Channel) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IChannelTuneRequestVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IChannelTuneRequest * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IChannelTuneRequest * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IChannelTuneRequest * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IChannelTuneRequest * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IChannelTuneRequest * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IChannelTuneRequest * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IChannelTuneRequest * This,
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
        
        DECLSPEC_XFGVIRT(ITuneRequest, get_TuningSpace)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TuningSpace )( 
            __RPC__in IChannelTuneRequest * This,
            /* [retval][out] */ __RPC__deref_out_opt ITuningSpace **TuningSpace);
        
        DECLSPEC_XFGVIRT(ITuneRequest, get_Components)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Components )( 
            __RPC__in IChannelTuneRequest * This,
            /* [retval][out] */ __RPC__deref_out_opt IComponents **Components);
        
        DECLSPEC_XFGVIRT(ITuneRequest, Clone)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IChannelTuneRequest * This,
            /* [retval][out] */ __RPC__deref_out_opt ITuneRequest **NewTuneRequest);
        
        DECLSPEC_XFGVIRT(ITuneRequest, get_Locator)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Locator )( 
            __RPC__in IChannelTuneRequest * This,
            /* [retval][out] */ __RPC__deref_out_opt ILocator **Locator);
        
        DECLSPEC_XFGVIRT(ITuneRequest, put_Locator)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Locator )( 
            __RPC__in IChannelTuneRequest * This,
            /* [in] */ __RPC__in_opt ILocator *Locator);
        
        DECLSPEC_XFGVIRT(IChannelTuneRequest, get_Channel)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Channel )( 
            __RPC__in IChannelTuneRequest * This,
            /* [retval][out] */ __RPC__out long *Channel);
        
        DECLSPEC_XFGVIRT(IChannelTuneRequest, put_Channel)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Channel )( 
            __RPC__in IChannelTuneRequest * This,
            /* [in] */ long Channel);
        
        END_INTERFACE
    } IChannelTuneRequestVtbl;

    interface IChannelTuneRequest
    {
        CONST_VTBL struct IChannelTuneRequestVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IChannelTuneRequest_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IChannelTuneRequest_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IChannelTuneRequest_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IChannelTuneRequest_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IChannelTuneRequest_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IChannelTuneRequest_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IChannelTuneRequest_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IChannelTuneRequest_get_TuningSpace(This,TuningSpace)	\
    ( (This)->lpVtbl -> get_TuningSpace(This,TuningSpace) ) 

#define IChannelTuneRequest_get_Components(This,Components)	\
    ( (This)->lpVtbl -> get_Components(This,Components) ) 

#define IChannelTuneRequest_Clone(This,NewTuneRequest)	\
    ( (This)->lpVtbl -> Clone(This,NewTuneRequest) ) 

#define IChannelTuneRequest_get_Locator(This,Locator)	\
    ( (This)->lpVtbl -> get_Locator(This,Locator) ) 

#define IChannelTuneRequest_put_Locator(This,Locator)	\
    ( (This)->lpVtbl -> put_Locator(This,Locator) ) 


#define IChannelTuneRequest_get_Channel(This,Channel)	\
    ( (This)->lpVtbl -> get_Channel(This,Channel) ) 

#define IChannelTuneRequest_put_Channel(This,Channel)	\
    ( (This)->lpVtbl -> put_Channel(This,Channel) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IChannelTuneRequest_INTERFACE_DEFINED__ */


#ifndef __IATSCChannelTuneRequest_INTERFACE_DEFINED__
#define __IATSCChannelTuneRequest_INTERFACE_DEFINED__

/* interface IATSCChannelTuneRequest */
/* [unique][helpstring][proxy][oleautomation][dual][uuid][nonextensible][object] */ 


EXTERN_C const IID IID_IATSCChannelTuneRequest;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0369B4E1-45B6-11d3-B650-00C04F79498E")
    IATSCChannelTuneRequest : public IChannelTuneRequest
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MinorChannel( 
            /* [retval][out] */ __RPC__out long *MinorChannel) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_MinorChannel( 
            /* [in] */ long MinorChannel) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IATSCChannelTuneRequestVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IATSCChannelTuneRequest * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IATSCChannelTuneRequest * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IATSCChannelTuneRequest * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IATSCChannelTuneRequest * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IATSCChannelTuneRequest * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IATSCChannelTuneRequest * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IATSCChannelTuneRequest * This,
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
        
        DECLSPEC_XFGVIRT(ITuneRequest, get_TuningSpace)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TuningSpace )( 
            __RPC__in IATSCChannelTuneRequest * This,
            /* [retval][out] */ __RPC__deref_out_opt ITuningSpace **TuningSpace);
        
        DECLSPEC_XFGVIRT(ITuneRequest, get_Components)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Components )( 
            __RPC__in IATSCChannelTuneRequest * This,
            /* [retval][out] */ __RPC__deref_out_opt IComponents **Components);
        
        DECLSPEC_XFGVIRT(ITuneRequest, Clone)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IATSCChannelTuneRequest * This,
            /* [retval][out] */ __RPC__deref_out_opt ITuneRequest **NewTuneRequest);
        
        DECLSPEC_XFGVIRT(ITuneRequest, get_Locator)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Locator )( 
            __RPC__in IATSCChannelTuneRequest * This,
            /* [retval][out] */ __RPC__deref_out_opt ILocator **Locator);
        
        DECLSPEC_XFGVIRT(ITuneRequest, put_Locator)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Locator )( 
            __RPC__in IATSCChannelTuneRequest * This,
            /* [in] */ __RPC__in_opt ILocator *Locator);
        
        DECLSPEC_XFGVIRT(IChannelTuneRequest, get_Channel)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Channel )( 
            __RPC__in IATSCChannelTuneRequest * This,
            /* [retval][out] */ __RPC__out long *Channel);
        
        DECLSPEC_XFGVIRT(IChannelTuneRequest, put_Channel)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Channel )( 
            __RPC__in IATSCChannelTuneRequest * This,
            /* [in] */ long Channel);
        
        DECLSPEC_XFGVIRT(IATSCChannelTuneRequest, get_MinorChannel)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MinorChannel )( 
            __RPC__in IATSCChannelTuneRequest * This,
            /* [retval][out] */ __RPC__out long *MinorChannel);
        
        DECLSPEC_XFGVIRT(IATSCChannelTuneRequest, put_MinorChannel)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MinorChannel )( 
            __RPC__in IATSCChannelTuneRequest * This,
            /* [in] */ long MinorChannel);
        
        END_INTERFACE
    } IATSCChannelTuneRequestVtbl;

    interface IATSCChannelTuneRequest
    {
        CONST_VTBL struct IATSCChannelTuneRequestVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IATSCChannelTuneRequest_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IATSCChannelTuneRequest_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IATSCChannelTuneRequest_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IATSCChannelTuneRequest_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IATSCChannelTuneRequest_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IATSCChannelTuneRequest_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IATSCChannelTuneRequest_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IATSCChannelTuneRequest_get_TuningSpace(This,TuningSpace)	\
    ( (This)->lpVtbl -> get_TuningSpace(This,TuningSpace) ) 

#define IATSCChannelTuneRequest_get_Components(This,Components)	\
    ( (This)->lpVtbl -> get_Components(This,Components) ) 

#define IATSCChannelTuneRequest_Clone(This,NewTuneRequest)	\
    ( (This)->lpVtbl -> Clone(This,NewTuneRequest) ) 

#define IATSCChannelTuneRequest_get_Locator(This,Locator)	\
    ( (This)->lpVtbl -> get_Locator(This,Locator) ) 

#define IATSCChannelTuneRequest_put_Locator(This,Locator)	\
    ( (This)->lpVtbl -> put_Locator(This,Locator) ) 


#define IATSCChannelTuneRequest_get_Channel(This,Channel)	\
    ( (This)->lpVtbl -> get_Channel(This,Channel) ) 

#define IATSCChannelTuneRequest_put_Channel(This,Channel)	\
    ( (This)->lpVtbl -> put_Channel(This,Channel) ) 


#define IATSCChannelTuneRequest_get_MinorChannel(This,MinorChannel)	\
    ( (This)->lpVtbl -> get_MinorChannel(This,MinorChannel) ) 

#define IATSCChannelTuneRequest_put_MinorChannel(This,MinorChannel)	\
    ( (This)->lpVtbl -> put_MinorChannel(This,MinorChannel) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IATSCChannelTuneRequest_INTERFACE_DEFINED__ */


#ifndef __IDigitalCableTuneRequest_INTERFACE_DEFINED__
#define __IDigitalCableTuneRequest_INTERFACE_DEFINED__

/* interface IDigitalCableTuneRequest */
/* [unique][helpstring][proxy][oleautomation][dual][uuid][nonextensible][object] */ 


EXTERN_C const IID IID_IDigitalCableTuneRequest;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BAD7753B-6B37-4810-AE57-3CE0C4A9E6CB")
    IDigitalCableTuneRequest : public IATSCChannelTuneRequest
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MajorChannel( 
            /* [retval][out] */ __RPC__out long *pMajorChannel) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_MajorChannel( 
            /* [in] */ long MajorChannel) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SourceID( 
            /* [retval][out] */ __RPC__out long *pSourceID) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_SourceID( 
            /* [in] */ long SourceID) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDigitalCableTuneRequestVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDigitalCableTuneRequest * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDigitalCableTuneRequest * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDigitalCableTuneRequest * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IDigitalCableTuneRequest * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IDigitalCableTuneRequest * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IDigitalCableTuneRequest * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IDigitalCableTuneRequest * This,
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
        
        DECLSPEC_XFGVIRT(ITuneRequest, get_TuningSpace)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TuningSpace )( 
            __RPC__in IDigitalCableTuneRequest * This,
            /* [retval][out] */ __RPC__deref_out_opt ITuningSpace **TuningSpace);
        
        DECLSPEC_XFGVIRT(ITuneRequest, get_Components)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Components )( 
            __RPC__in IDigitalCableTuneRequest * This,
            /* [retval][out] */ __RPC__deref_out_opt IComponents **Components);
        
        DECLSPEC_XFGVIRT(ITuneRequest, Clone)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IDigitalCableTuneRequest * This,
            /* [retval][out] */ __RPC__deref_out_opt ITuneRequest **NewTuneRequest);
        
        DECLSPEC_XFGVIRT(ITuneRequest, get_Locator)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Locator )( 
            __RPC__in IDigitalCableTuneRequest * This,
            /* [retval][out] */ __RPC__deref_out_opt ILocator **Locator);
        
        DECLSPEC_XFGVIRT(ITuneRequest, put_Locator)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Locator )( 
            __RPC__in IDigitalCableTuneRequest * This,
            /* [in] */ __RPC__in_opt ILocator *Locator);
        
        DECLSPEC_XFGVIRT(IChannelTuneRequest, get_Channel)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Channel )( 
            __RPC__in IDigitalCableTuneRequest * This,
            /* [retval][out] */ __RPC__out long *Channel);
        
        DECLSPEC_XFGVIRT(IChannelTuneRequest, put_Channel)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Channel )( 
            __RPC__in IDigitalCableTuneRequest * This,
            /* [in] */ long Channel);
        
        DECLSPEC_XFGVIRT(IATSCChannelTuneRequest, get_MinorChannel)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MinorChannel )( 
            __RPC__in IDigitalCableTuneRequest * This,
            /* [retval][out] */ __RPC__out long *MinorChannel);
        
        DECLSPEC_XFGVIRT(IATSCChannelTuneRequest, put_MinorChannel)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MinorChannel )( 
            __RPC__in IDigitalCableTuneRequest * This,
            /* [in] */ long MinorChannel);
        
        DECLSPEC_XFGVIRT(IDigitalCableTuneRequest, get_MajorChannel)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MajorChannel )( 
            __RPC__in IDigitalCableTuneRequest * This,
            /* [retval][out] */ __RPC__out long *pMajorChannel);
        
        DECLSPEC_XFGVIRT(IDigitalCableTuneRequest, put_MajorChannel)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MajorChannel )( 
            __RPC__in IDigitalCableTuneRequest * This,
            /* [in] */ long MajorChannel);
        
        DECLSPEC_XFGVIRT(IDigitalCableTuneRequest, get_SourceID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SourceID )( 
            __RPC__in IDigitalCableTuneRequest * This,
            /* [retval][out] */ __RPC__out long *pSourceID);
        
        DECLSPEC_XFGVIRT(IDigitalCableTuneRequest, put_SourceID)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SourceID )( 
            __RPC__in IDigitalCableTuneRequest * This,
            /* [in] */ long SourceID);
        
        END_INTERFACE
    } IDigitalCableTuneRequestVtbl;

    interface IDigitalCableTuneRequest
    {
        CONST_VTBL struct IDigitalCableTuneRequestVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDigitalCableTuneRequest_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDigitalCableTuneRequest_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDigitalCableTuneRequest_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDigitalCableTuneRequest_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IDigitalCableTuneRequest_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IDigitalCableTuneRequest_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IDigitalCableTuneRequest_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IDigitalCableTuneRequest_get_TuningSpace(This,TuningSpace)	\
    ( (This)->lpVtbl -> get_TuningSpace(This,TuningSpace) ) 

#define IDigitalCableTuneRequest_get_Components(This,Components)	\
    ( (This)->lpVtbl -> get_Components(This,Components) ) 

#define IDigitalCableTuneRequest_Clone(This,NewTuneRequest)	\
    ( (This)->lpVtbl -> Clone(This,NewTuneRequest) ) 

#define IDigitalCableTuneRequest_get_Locator(This,Locator)	\
    ( (This)->lpVtbl -> get_Locator(This,Locator) ) 

#define IDigitalCableTuneRequest_put_Locator(This,Locator)	\
    ( (This)->lpVtbl -> put_Locator(This,Locator) ) 


#define IDigitalCableTuneRequest_get_Channel(This,Channel)	\
    ( (This)->lpVtbl -> get_Channel(This,Channel) ) 

#define IDigitalCableTuneRequest_put_Channel(This,Channel)	\
    ( (This)->lpVtbl -> put_Channel(This,Channel) ) 


#define IDigitalCableTuneRequest_get_MinorChannel(This,MinorChannel)	\
    ( (This)->lpVtbl -> get_MinorChannel(This,MinorChannel) ) 

#define IDigitalCableTuneRequest_put_MinorChannel(This,MinorChannel)	\
    ( (This)->lpVtbl -> put_MinorChannel(This,MinorChannel) ) 


#define IDigitalCableTuneRequest_get_MajorChannel(This,pMajorChannel)	\
    ( (This)->lpVtbl -> get_MajorChannel(This,pMajorChannel) ) 

#define IDigitalCableTuneRequest_put_MajorChannel(This,MajorChannel)	\
    ( (This)->lpVtbl -> put_MajorChannel(This,MajorChannel) ) 

#define IDigitalCableTuneRequest_get_SourceID(This,pSourceID)	\
    ( (This)->lpVtbl -> get_SourceID(This,pSourceID) ) 

#define IDigitalCableTuneRequest_put_SourceID(This,SourceID)	\
    ( (This)->lpVtbl -> put_SourceID(This,SourceID) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDigitalCableTuneRequest_INTERFACE_DEFINED__ */


#ifndef __IDVBTuneRequest_INTERFACE_DEFINED__
#define __IDVBTuneRequest_INTERFACE_DEFINED__

/* interface IDVBTuneRequest */
/* [unique][helpstring][proxy][oleautomation][dual][uuid][nonextensible][object] */ 


EXTERN_C const IID IID_IDVBTuneRequest;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0D6F567E-A636-42bb-83BA-CE4C1704AFA2")
    IDVBTuneRequest : public ITuneRequest
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ONID( 
            /* [retval][out] */ __RPC__out long *ONID) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_ONID( 
            /* [in] */ long ONID) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TSID( 
            /* [retval][out] */ __RPC__out long *TSID) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_TSID( 
            /* [in] */ long TSID) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SID( 
            /* [retval][out] */ __RPC__out long *SID) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_SID( 
            /* [in] */ long SID) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDVBTuneRequestVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDVBTuneRequest * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDVBTuneRequest * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDVBTuneRequest * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IDVBTuneRequest * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IDVBTuneRequest * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IDVBTuneRequest * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IDVBTuneRequest * This,
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
        
        DECLSPEC_XFGVIRT(ITuneRequest, get_TuningSpace)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TuningSpace )( 
            __RPC__in IDVBTuneRequest * This,
            /* [retval][out] */ __RPC__deref_out_opt ITuningSpace **TuningSpace);
        
        DECLSPEC_XFGVIRT(ITuneRequest, get_Components)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Components )( 
            __RPC__in IDVBTuneRequest * This,
            /* [retval][out] */ __RPC__deref_out_opt IComponents **Components);
        
        DECLSPEC_XFGVIRT(ITuneRequest, Clone)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IDVBTuneRequest * This,
            /* [retval][out] */ __RPC__deref_out_opt ITuneRequest **NewTuneRequest);
        
        DECLSPEC_XFGVIRT(ITuneRequest, get_Locator)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Locator )( 
            __RPC__in IDVBTuneRequest * This,
            /* [retval][out] */ __RPC__deref_out_opt ILocator **Locator);
        
        DECLSPEC_XFGVIRT(ITuneRequest, put_Locator)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Locator )( 
            __RPC__in IDVBTuneRequest * This,
            /* [in] */ __RPC__in_opt ILocator *Locator);
        
        DECLSPEC_XFGVIRT(IDVBTuneRequest, get_ONID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ONID )( 
            __RPC__in IDVBTuneRequest * This,
            /* [retval][out] */ __RPC__out long *ONID);
        
        DECLSPEC_XFGVIRT(IDVBTuneRequest, put_ONID)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ONID )( 
            __RPC__in IDVBTuneRequest * This,
            /* [in] */ long ONID);
        
        DECLSPEC_XFGVIRT(IDVBTuneRequest, get_TSID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TSID )( 
            __RPC__in IDVBTuneRequest * This,
            /* [retval][out] */ __RPC__out long *TSID);
        
        DECLSPEC_XFGVIRT(IDVBTuneRequest, put_TSID)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_TSID )( 
            __RPC__in IDVBTuneRequest * This,
            /* [in] */ long TSID);
        
        DECLSPEC_XFGVIRT(IDVBTuneRequest, get_SID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SID )( 
            __RPC__in IDVBTuneRequest * This,
            /* [retval][out] */ __RPC__out long *SID);
        
        DECLSPEC_XFGVIRT(IDVBTuneRequest, put_SID)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SID )( 
            __RPC__in IDVBTuneRequest * This,
            /* [in] */ long SID);
        
        END_INTERFACE
    } IDVBTuneRequestVtbl;

    interface IDVBTuneRequest
    {
        CONST_VTBL struct IDVBTuneRequestVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDVBTuneRequest_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDVBTuneRequest_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDVBTuneRequest_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDVBTuneRequest_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IDVBTuneRequest_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IDVBTuneRequest_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IDVBTuneRequest_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IDVBTuneRequest_get_TuningSpace(This,TuningSpace)	\
    ( (This)->lpVtbl -> get_TuningSpace(This,TuningSpace) ) 

#define IDVBTuneRequest_get_Components(This,Components)	\
    ( (This)->lpVtbl -> get_Components(This,Components) ) 

#define IDVBTuneRequest_Clone(This,NewTuneRequest)	\
    ( (This)->lpVtbl -> Clone(This,NewTuneRequest) ) 

#define IDVBTuneRequest_get_Locator(This,Locator)	\
    ( (This)->lpVtbl -> get_Locator(This,Locator) ) 

#define IDVBTuneRequest_put_Locator(This,Locator)	\
    ( (This)->lpVtbl -> put_Locator(This,Locator) ) 


#define IDVBTuneRequest_get_ONID(This,ONID)	\
    ( (This)->lpVtbl -> get_ONID(This,ONID) ) 

#define IDVBTuneRequest_put_ONID(This,ONID)	\
    ( (This)->lpVtbl -> put_ONID(This,ONID) ) 

#define IDVBTuneRequest_get_TSID(This,TSID)	\
    ( (This)->lpVtbl -> get_TSID(This,TSID) ) 

#define IDVBTuneRequest_put_TSID(This,TSID)	\
    ( (This)->lpVtbl -> put_TSID(This,TSID) ) 

#define IDVBTuneRequest_get_SID(This,SID)	\
    ( (This)->lpVtbl -> get_SID(This,SID) ) 

#define IDVBTuneRequest_put_SID(This,SID)	\
    ( (This)->lpVtbl -> put_SID(This,SID) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDVBTuneRequest_INTERFACE_DEFINED__ */


#ifndef __IMPEG2TuneRequest_INTERFACE_DEFINED__
#define __IMPEG2TuneRequest_INTERFACE_DEFINED__

/* interface IMPEG2TuneRequest */
/* [unique][helpstring][proxy][oleautomation][dual][uuid][nonextensible][object] */ 


EXTERN_C const IID IID_IMPEG2TuneRequest;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("EB7D987F-8A01-42ad-B8AE-574DEEE44D1A")
    IMPEG2TuneRequest : public ITuneRequest
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TSID( 
            /* [retval][out] */ __RPC__out long *TSID) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_TSID( 
            /* [in] */ long TSID) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ProgNo( 
            /* [retval][out] */ __RPC__out long *ProgNo) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_ProgNo( 
            /* [in] */ long ProgNo) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMPEG2TuneRequestVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMPEG2TuneRequest * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMPEG2TuneRequest * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMPEG2TuneRequest * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMPEG2TuneRequest * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMPEG2TuneRequest * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMPEG2TuneRequest * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMPEG2TuneRequest * This,
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
        
        DECLSPEC_XFGVIRT(ITuneRequest, get_TuningSpace)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TuningSpace )( 
            __RPC__in IMPEG2TuneRequest * This,
            /* [retval][out] */ __RPC__deref_out_opt ITuningSpace **TuningSpace);
        
        DECLSPEC_XFGVIRT(ITuneRequest, get_Components)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Components )( 
            __RPC__in IMPEG2TuneRequest * This,
            /* [retval][out] */ __RPC__deref_out_opt IComponents **Components);
        
        DECLSPEC_XFGVIRT(ITuneRequest, Clone)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IMPEG2TuneRequest * This,
            /* [retval][out] */ __RPC__deref_out_opt ITuneRequest **NewTuneRequest);
        
        DECLSPEC_XFGVIRT(ITuneRequest, get_Locator)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Locator )( 
            __RPC__in IMPEG2TuneRequest * This,
            /* [retval][out] */ __RPC__deref_out_opt ILocator **Locator);
        
        DECLSPEC_XFGVIRT(ITuneRequest, put_Locator)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Locator )( 
            __RPC__in IMPEG2TuneRequest * This,
            /* [in] */ __RPC__in_opt ILocator *Locator);
        
        DECLSPEC_XFGVIRT(IMPEG2TuneRequest, get_TSID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TSID )( 
            __RPC__in IMPEG2TuneRequest * This,
            /* [retval][out] */ __RPC__out long *TSID);
        
        DECLSPEC_XFGVIRT(IMPEG2TuneRequest, put_TSID)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_TSID )( 
            __RPC__in IMPEG2TuneRequest * This,
            /* [in] */ long TSID);
        
        DECLSPEC_XFGVIRT(IMPEG2TuneRequest, get_ProgNo)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProgNo )( 
            __RPC__in IMPEG2TuneRequest * This,
            /* [retval][out] */ __RPC__out long *ProgNo);
        
        DECLSPEC_XFGVIRT(IMPEG2TuneRequest, put_ProgNo)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ProgNo )( 
            __RPC__in IMPEG2TuneRequest * This,
            /* [in] */ long ProgNo);
        
        END_INTERFACE
    } IMPEG2TuneRequestVtbl;

    interface IMPEG2TuneRequest
    {
        CONST_VTBL struct IMPEG2TuneRequestVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMPEG2TuneRequest_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMPEG2TuneRequest_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMPEG2TuneRequest_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMPEG2TuneRequest_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMPEG2TuneRequest_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMPEG2TuneRequest_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMPEG2TuneRequest_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMPEG2TuneRequest_get_TuningSpace(This,TuningSpace)	\
    ( (This)->lpVtbl -> get_TuningSpace(This,TuningSpace) ) 

#define IMPEG2TuneRequest_get_Components(This,Components)	\
    ( (This)->lpVtbl -> get_Components(This,Components) ) 

#define IMPEG2TuneRequest_Clone(This,NewTuneRequest)	\
    ( (This)->lpVtbl -> Clone(This,NewTuneRequest) ) 

#define IMPEG2TuneRequest_get_Locator(This,Locator)	\
    ( (This)->lpVtbl -> get_Locator(This,Locator) ) 

#define IMPEG2TuneRequest_put_Locator(This,Locator)	\
    ( (This)->lpVtbl -> put_Locator(This,Locator) ) 


#define IMPEG2TuneRequest_get_TSID(This,TSID)	\
    ( (This)->lpVtbl -> get_TSID(This,TSID) ) 

#define IMPEG2TuneRequest_put_TSID(This,TSID)	\
    ( (This)->lpVtbl -> put_TSID(This,TSID) ) 

#define IMPEG2TuneRequest_get_ProgNo(This,ProgNo)	\
    ( (This)->lpVtbl -> get_ProgNo(This,ProgNo) ) 

#define IMPEG2TuneRequest_put_ProgNo(This,ProgNo)	\
    ( (This)->lpVtbl -> put_ProgNo(This,ProgNo) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMPEG2TuneRequest_INTERFACE_DEFINED__ */


#ifndef __IMPEG2TuneRequestFactory_INTERFACE_DEFINED__
#define __IMPEG2TuneRequestFactory_INTERFACE_DEFINED__

/* interface IMPEG2TuneRequestFactory */
/* [unique][helpstring][proxy][oleautomation][dual][uuid][hidden][nonextensible][object] */ 


EXTERN_C const IID IID_IMPEG2TuneRequestFactory;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("14E11ABD-EE37-4893-9EA1-6964DE933E39")
    IMPEG2TuneRequestFactory : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateTuneRequest( 
            /* [in] */ __RPC__in_opt ITuningSpace *TuningSpace,
            /* [retval][out] */ __RPC__deref_out_opt IMPEG2TuneRequest **TuneRequest) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMPEG2TuneRequestFactoryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMPEG2TuneRequestFactory * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMPEG2TuneRequestFactory * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMPEG2TuneRequestFactory * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMPEG2TuneRequestFactory * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMPEG2TuneRequestFactory * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMPEG2TuneRequestFactory * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMPEG2TuneRequestFactory * This,
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
        
        DECLSPEC_XFGVIRT(IMPEG2TuneRequestFactory, CreateTuneRequest)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateTuneRequest )( 
            __RPC__in IMPEG2TuneRequestFactory * This,
            /* [in] */ __RPC__in_opt ITuningSpace *TuningSpace,
            /* [retval][out] */ __RPC__deref_out_opt IMPEG2TuneRequest **TuneRequest);
        
        END_INTERFACE
    } IMPEG2TuneRequestFactoryVtbl;

    interface IMPEG2TuneRequestFactory
    {
        CONST_VTBL struct IMPEG2TuneRequestFactoryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMPEG2TuneRequestFactory_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMPEG2TuneRequestFactory_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMPEG2TuneRequestFactory_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMPEG2TuneRequestFactory_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMPEG2TuneRequestFactory_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMPEG2TuneRequestFactory_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMPEG2TuneRequestFactory_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMPEG2TuneRequestFactory_CreateTuneRequest(This,TuningSpace,TuneRequest)	\
    ( (This)->lpVtbl -> CreateTuneRequest(This,TuningSpace,TuneRequest) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMPEG2TuneRequestFactory_INTERFACE_DEFINED__ */


#ifndef __IMPEG2TuneRequestSupport_INTERFACE_DEFINED__
#define __IMPEG2TuneRequestSupport_INTERFACE_DEFINED__

/* interface IMPEG2TuneRequestSupport */
/* [unique][helpstring][uuid][nonextensible][restricted][hidden][object] */ 


EXTERN_C const IID IID_IMPEG2TuneRequestSupport;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1B9D5FC3-5BBC-4b6c-BB18-B9D10E3EEEBF")
    IMPEG2TuneRequestSupport : public IUnknown
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct IMPEG2TuneRequestSupportVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMPEG2TuneRequestSupport * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMPEG2TuneRequestSupport * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMPEG2TuneRequestSupport * This);
        
        END_INTERFACE
    } IMPEG2TuneRequestSupportVtbl;

    interface IMPEG2TuneRequestSupport
    {
        CONST_VTBL struct IMPEG2TuneRequestSupportVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMPEG2TuneRequestSupport_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMPEG2TuneRequestSupport_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMPEG2TuneRequestSupport_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMPEG2TuneRequestSupport_INTERFACE_DEFINED__ */


#ifndef __ITunerCap_INTERFACE_DEFINED__
#define __ITunerCap_INTERFACE_DEFINED__

/* interface ITunerCap */
/* [unique][helpstring][uuid][nonextensible][hidden][object] */ 


EXTERN_C const IID IID_ITunerCap;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("E60DFA45-8D56-4e65-A8AB-D6BE9412C249")
    ITunerCap : public IUnknown
    {
    public:
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_SupportedNetworkTypes( 
            /* [in] */ ULONG ulcNetworkTypesMax,
            /* [out] */ __RPC__out ULONG *pulcNetworkTypes,
            /* [out][in] */ __RPC__inout GUID *pguidNetworkTypes) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_SupportedVideoFormats( 
            /* [out] */ __RPC__out ULONG *pulAMTunerModeType,
            /* [out] */ __RPC__out ULONG *pulAnalogVideoStandard) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_AuxInputCount( 
            /* [out][in] */ __RPC__inout ULONG *pulCompositeCount,
            /* [out][in] */ __RPC__inout ULONG *pulSvideoCount) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITunerCapVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITunerCap * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITunerCap * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITunerCap * This);
        
        DECLSPEC_XFGVIRT(ITunerCap, get_SupportedNetworkTypes)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SupportedNetworkTypes )( 
            __RPC__in ITunerCap * This,
            /* [in] */ ULONG ulcNetworkTypesMax,
            /* [out] */ __RPC__out ULONG *pulcNetworkTypes,
            /* [out][in] */ __RPC__inout GUID *pguidNetworkTypes);
        
        DECLSPEC_XFGVIRT(ITunerCap, get_SupportedVideoFormats)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SupportedVideoFormats )( 
            __RPC__in ITunerCap * This,
            /* [out] */ __RPC__out ULONG *pulAMTunerModeType,
            /* [out] */ __RPC__out ULONG *pulAnalogVideoStandard);
        
        DECLSPEC_XFGVIRT(ITunerCap, get_AuxInputCount)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AuxInputCount )( 
            __RPC__in ITunerCap * This,
            /* [out][in] */ __RPC__inout ULONG *pulCompositeCount,
            /* [out][in] */ __RPC__inout ULONG *pulSvideoCount);
        
        END_INTERFACE
    } ITunerCapVtbl;

    interface ITunerCap
    {
        CONST_VTBL struct ITunerCapVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITunerCap_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITunerCap_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITunerCap_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITunerCap_get_SupportedNetworkTypes(This,ulcNetworkTypesMax,pulcNetworkTypes,pguidNetworkTypes)	\
    ( (This)->lpVtbl -> get_SupportedNetworkTypes(This,ulcNetworkTypesMax,pulcNetworkTypes,pguidNetworkTypes) ) 

#define ITunerCap_get_SupportedVideoFormats(This,pulAMTunerModeType,pulAnalogVideoStandard)	\
    ( (This)->lpVtbl -> get_SupportedVideoFormats(This,pulAMTunerModeType,pulAnalogVideoStandard) ) 

#define ITunerCap_get_AuxInputCount(This,pulCompositeCount,pulSvideoCount)	\
    ( (This)->lpVtbl -> get_AuxInputCount(This,pulCompositeCount,pulSvideoCount) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITunerCap_INTERFACE_DEFINED__ */


#ifndef __ITunerCapEx_INTERFACE_DEFINED__
#define __ITunerCapEx_INTERFACE_DEFINED__

/* interface ITunerCapEx */
/* [unique][helpstring][uuid][nonextensible][hidden][object] */ 


EXTERN_C const IID IID_ITunerCapEx;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("ed3e0c66-18c8-4ea6-9300-f6841fdd35dc")
    ITunerCapEx : public IUnknown
    {
    public:
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Has608_708Caption( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbHasCaption) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITunerCapExVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITunerCapEx * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITunerCapEx * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITunerCapEx * This);
        
        DECLSPEC_XFGVIRT(ITunerCapEx, get_Has608_708Caption)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Has608_708Caption )( 
            __RPC__in ITunerCapEx * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbHasCaption);
        
        END_INTERFACE
    } ITunerCapExVtbl;

    interface ITunerCapEx
    {
        CONST_VTBL struct ITunerCapExVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITunerCapEx_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITunerCapEx_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITunerCapEx_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITunerCapEx_get_Has608_708Caption(This,pbHasCaption)	\
    ( (This)->lpVtbl -> get_Has608_708Caption(This,pbHasCaption) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITunerCapEx_INTERFACE_DEFINED__ */


#ifndef __ITuner_INTERFACE_DEFINED__
#define __ITuner_INTERFACE_DEFINED__

/* interface ITuner */
/* [unique][helpstring][uuid][nonextensible][hidden][object] */ 


EXTERN_C const IID IID_ITuner;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("28C52640-018A-11d3-9D8E-00C04F72D980")
    ITuner : public IUnknown
    {
    public:
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_TuningSpace( 
            /* [retval][out] */ __RPC__deref_out_opt ITuningSpace **TuningSpace) = 0;
        
        virtual /* [helpstring][propput] */ HRESULT STDMETHODCALLTYPE put_TuningSpace( 
            /* [in] */ __RPC__in_opt ITuningSpace *TuningSpace) = 0;
        
        virtual /* [helpstring][restricted][hidden] */ HRESULT STDMETHODCALLTYPE EnumTuningSpaces( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumTuningSpaces **ppEnum) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_TuneRequest( 
            /* [retval][out] */ __RPC__deref_out_opt ITuneRequest **TuneRequest) = 0;
        
        virtual /* [helpstring][propput] */ HRESULT STDMETHODCALLTYPE put_TuneRequest( 
            /* [in] */ __RPC__in_opt ITuneRequest *TuneRequest) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Validate( 
            /* [in] */ __RPC__in_opt ITuneRequest *TuneRequest) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_PreferredComponentTypes( 
            /* [retval][out] */ __RPC__deref_out_opt IComponentTypes **ComponentTypes) = 0;
        
        virtual /* [helpstring][propput] */ HRESULT STDMETHODCALLTYPE put_PreferredComponentTypes( 
            /* [in] */ __RPC__in_opt IComponentTypes *ComponentTypes) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_SignalStrength( 
            /* [retval][out] */ __RPC__out long *Strength) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE TriggerSignalEvents( 
            /* [in] */ long Interval) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITunerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITuner * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITuner * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITuner * This);
        
        DECLSPEC_XFGVIRT(ITuner, get_TuningSpace)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TuningSpace )( 
            __RPC__in ITuner * This,
            /* [retval][out] */ __RPC__deref_out_opt ITuningSpace **TuningSpace);
        
        DECLSPEC_XFGVIRT(ITuner, put_TuningSpace)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_TuningSpace )( 
            __RPC__in ITuner * This,
            /* [in] */ __RPC__in_opt ITuningSpace *TuningSpace);
        
        DECLSPEC_XFGVIRT(ITuner, EnumTuningSpaces)
        /* [helpstring][restricted][hidden] */ HRESULT ( STDMETHODCALLTYPE *EnumTuningSpaces )( 
            __RPC__in ITuner * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumTuningSpaces **ppEnum);
        
        DECLSPEC_XFGVIRT(ITuner, get_TuneRequest)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TuneRequest )( 
            __RPC__in ITuner * This,
            /* [retval][out] */ __RPC__deref_out_opt ITuneRequest **TuneRequest);
        
        DECLSPEC_XFGVIRT(ITuner, put_TuneRequest)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_TuneRequest )( 
            __RPC__in ITuner * This,
            /* [in] */ __RPC__in_opt ITuneRequest *TuneRequest);
        
        DECLSPEC_XFGVIRT(ITuner, Validate)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Validate )( 
            __RPC__in ITuner * This,
            /* [in] */ __RPC__in_opt ITuneRequest *TuneRequest);
        
        DECLSPEC_XFGVIRT(ITuner, get_PreferredComponentTypes)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PreferredComponentTypes )( 
            __RPC__in ITuner * This,
            /* [retval][out] */ __RPC__deref_out_opt IComponentTypes **ComponentTypes);
        
        DECLSPEC_XFGVIRT(ITuner, put_PreferredComponentTypes)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PreferredComponentTypes )( 
            __RPC__in ITuner * This,
            /* [in] */ __RPC__in_opt IComponentTypes *ComponentTypes);
        
        DECLSPEC_XFGVIRT(ITuner, get_SignalStrength)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SignalStrength )( 
            __RPC__in ITuner * This,
            /* [retval][out] */ __RPC__out long *Strength);
        
        DECLSPEC_XFGVIRT(ITuner, TriggerSignalEvents)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *TriggerSignalEvents )( 
            __RPC__in ITuner * This,
            /* [in] */ long Interval);
        
        END_INTERFACE
    } ITunerVtbl;

    interface ITuner
    {
        CONST_VTBL struct ITunerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITuner_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITuner_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITuner_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITuner_get_TuningSpace(This,TuningSpace)	\
    ( (This)->lpVtbl -> get_TuningSpace(This,TuningSpace) ) 

#define ITuner_put_TuningSpace(This,TuningSpace)	\
    ( (This)->lpVtbl -> put_TuningSpace(This,TuningSpace) ) 

#define ITuner_EnumTuningSpaces(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumTuningSpaces(This,ppEnum) ) 

#define ITuner_get_TuneRequest(This,TuneRequest)	\
    ( (This)->lpVtbl -> get_TuneRequest(This,TuneRequest) ) 

#define ITuner_put_TuneRequest(This,TuneRequest)	\
    ( (This)->lpVtbl -> put_TuneRequest(This,TuneRequest) ) 

#define ITuner_Validate(This,TuneRequest)	\
    ( (This)->lpVtbl -> Validate(This,TuneRequest) ) 

#define ITuner_get_PreferredComponentTypes(This,ComponentTypes)	\
    ( (This)->lpVtbl -> get_PreferredComponentTypes(This,ComponentTypes) ) 

#define ITuner_put_PreferredComponentTypes(This,ComponentTypes)	\
    ( (This)->lpVtbl -> put_PreferredComponentTypes(This,ComponentTypes) ) 

#define ITuner_get_SignalStrength(This,Strength)	\
    ( (This)->lpVtbl -> get_SignalStrength(This,Strength) ) 

#define ITuner_TriggerSignalEvents(This,Interval)	\
    ( (This)->lpVtbl -> TriggerSignalEvents(This,Interval) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITuner_INTERFACE_DEFINED__ */


#ifndef __IScanningTuner_INTERFACE_DEFINED__
#define __IScanningTuner_INTERFACE_DEFINED__

/* interface IScanningTuner */
/* [unique][helpstring][uuid][nonextensible][hidden][object] */ 


EXTERN_C const IID IID_IScanningTuner;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1DFD0A5C-0284-11d3-9D8E-00C04F72D980")
    IScanningTuner : public ITuner
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SeekUp( void) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SeekDown( void) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE ScanUp( 
            /* [in] */ long MillisecondsPause) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE ScanDown( 
            /* [in] */ long MillisecondsPause) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE AutoProgram( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IScanningTunerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IScanningTuner * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IScanningTuner * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IScanningTuner * This);
        
        DECLSPEC_XFGVIRT(ITuner, get_TuningSpace)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TuningSpace )( 
            __RPC__in IScanningTuner * This,
            /* [retval][out] */ __RPC__deref_out_opt ITuningSpace **TuningSpace);
        
        DECLSPEC_XFGVIRT(ITuner, put_TuningSpace)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_TuningSpace )( 
            __RPC__in IScanningTuner * This,
            /* [in] */ __RPC__in_opt ITuningSpace *TuningSpace);
        
        DECLSPEC_XFGVIRT(ITuner, EnumTuningSpaces)
        /* [helpstring][restricted][hidden] */ HRESULT ( STDMETHODCALLTYPE *EnumTuningSpaces )( 
            __RPC__in IScanningTuner * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumTuningSpaces **ppEnum);
        
        DECLSPEC_XFGVIRT(ITuner, get_TuneRequest)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TuneRequest )( 
            __RPC__in IScanningTuner * This,
            /* [retval][out] */ __RPC__deref_out_opt ITuneRequest **TuneRequest);
        
        DECLSPEC_XFGVIRT(ITuner, put_TuneRequest)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_TuneRequest )( 
            __RPC__in IScanningTuner * This,
            /* [in] */ __RPC__in_opt ITuneRequest *TuneRequest);
        
        DECLSPEC_XFGVIRT(ITuner, Validate)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Validate )( 
            __RPC__in IScanningTuner * This,
            /* [in] */ __RPC__in_opt ITuneRequest *TuneRequest);
        
        DECLSPEC_XFGVIRT(ITuner, get_PreferredComponentTypes)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PreferredComponentTypes )( 
            __RPC__in IScanningTuner * This,
            /* [retval][out] */ __RPC__deref_out_opt IComponentTypes **ComponentTypes);
        
        DECLSPEC_XFGVIRT(ITuner, put_PreferredComponentTypes)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PreferredComponentTypes )( 
            __RPC__in IScanningTuner * This,
            /* [in] */ __RPC__in_opt IComponentTypes *ComponentTypes);
        
        DECLSPEC_XFGVIRT(ITuner, get_SignalStrength)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SignalStrength )( 
            __RPC__in IScanningTuner * This,
            /* [retval][out] */ __RPC__out long *Strength);
        
        DECLSPEC_XFGVIRT(ITuner, TriggerSignalEvents)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *TriggerSignalEvents )( 
            __RPC__in IScanningTuner * This,
            /* [in] */ long Interval);
        
        DECLSPEC_XFGVIRT(IScanningTuner, SeekUp)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SeekUp )( 
            __RPC__in IScanningTuner * This);
        
        DECLSPEC_XFGVIRT(IScanningTuner, SeekDown)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SeekDown )( 
            __RPC__in IScanningTuner * This);
        
        DECLSPEC_XFGVIRT(IScanningTuner, ScanUp)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ScanUp )( 
            __RPC__in IScanningTuner * This,
            /* [in] */ long MillisecondsPause);
        
        DECLSPEC_XFGVIRT(IScanningTuner, ScanDown)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ScanDown )( 
            __RPC__in IScanningTuner * This,
            /* [in] */ long MillisecondsPause);
        
        DECLSPEC_XFGVIRT(IScanningTuner, AutoProgram)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AutoProgram )( 
            __RPC__in IScanningTuner * This);
        
        END_INTERFACE
    } IScanningTunerVtbl;

    interface IScanningTuner
    {
        CONST_VTBL struct IScanningTunerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IScanningTuner_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IScanningTuner_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IScanningTuner_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IScanningTuner_get_TuningSpace(This,TuningSpace)	\
    ( (This)->lpVtbl -> get_TuningSpace(This,TuningSpace) ) 

#define IScanningTuner_put_TuningSpace(This,TuningSpace)	\
    ( (This)->lpVtbl -> put_TuningSpace(This,TuningSpace) ) 

#define IScanningTuner_EnumTuningSpaces(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumTuningSpaces(This,ppEnum) ) 

#define IScanningTuner_get_TuneRequest(This,TuneRequest)	\
    ( (This)->lpVtbl -> get_TuneRequest(This,TuneRequest) ) 

#define IScanningTuner_put_TuneRequest(This,TuneRequest)	\
    ( (This)->lpVtbl -> put_TuneRequest(This,TuneRequest) ) 

#define IScanningTuner_Validate(This,TuneRequest)	\
    ( (This)->lpVtbl -> Validate(This,TuneRequest) ) 

#define IScanningTuner_get_PreferredComponentTypes(This,ComponentTypes)	\
    ( (This)->lpVtbl -> get_PreferredComponentTypes(This,ComponentTypes) ) 

#define IScanningTuner_put_PreferredComponentTypes(This,ComponentTypes)	\
    ( (This)->lpVtbl -> put_PreferredComponentTypes(This,ComponentTypes) ) 

#define IScanningTuner_get_SignalStrength(This,Strength)	\
    ( (This)->lpVtbl -> get_SignalStrength(This,Strength) ) 

#define IScanningTuner_TriggerSignalEvents(This,Interval)	\
    ( (This)->lpVtbl -> TriggerSignalEvents(This,Interval) ) 


#define IScanningTuner_SeekUp(This)	\
    ( (This)->lpVtbl -> SeekUp(This) ) 

#define IScanningTuner_SeekDown(This)	\
    ( (This)->lpVtbl -> SeekDown(This) ) 

#define IScanningTuner_ScanUp(This,MillisecondsPause)	\
    ( (This)->lpVtbl -> ScanUp(This,MillisecondsPause) ) 

#define IScanningTuner_ScanDown(This,MillisecondsPause)	\
    ( (This)->lpVtbl -> ScanDown(This,MillisecondsPause) ) 

#define IScanningTuner_AutoProgram(This)	\
    ( (This)->lpVtbl -> AutoProgram(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IScanningTuner_INTERFACE_DEFINED__ */


#ifndef __IScanningTunerEx_INTERFACE_DEFINED__
#define __IScanningTunerEx_INTERFACE_DEFINED__

/* interface IScanningTunerEx */
/* [unique][helpstring][uuid][nonextensible][hidden][object] */ 


EXTERN_C const IID IID_IScanningTunerEx;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("04BBD195-0E2D-4593-9BD5-4F908BC33CF5")
    IScanningTunerEx : public IScanningTuner
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetCurrentLocator( 
            __RPC__deref_in_opt ILocator **pILocator) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE PerformExhaustiveScan( 
            /* [in] */ long dwLowerFreq,
            /* [in] */ long dwHigherFreq,
            /* [in] */ VARIANT_BOOL bFineTune,
            /* [in] */ HEVENT hEvent) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE TerminateCurrentScan( 
            /* [retval][out] */ __RPC__out long *pcurrentFreq) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE ResumeCurrentScan( 
            /* [in] */ HEVENT hEvent) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetTunerScanningCapability( 
            /* [out] */ __RPC__out long *HardwareAssistedScanning,
            /* [out] */ __RPC__out long *NumStandardsSupported,
            /* [out] */ __RPC__out GUID *BroadcastStandards) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetTunerStatus( 
            /* [out] */ __RPC__out long *SecondsLeft,
            /* [out] */ __RPC__out long *CurrentLockType,
            /* [out] */ __RPC__out long *AutoDetect,
            /* [out] */ __RPC__out long *CurrentFreq) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetCurrentTunerStandardCapability( 
            /* [in] */ GUID CurrentBroadcastStandard,
            /* [out] */ __RPC__out long *SettlingTime,
            /* [out] */ __RPC__out long *TvStandardsSupported) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetScanSignalTypeFilter( 
            /* [in] */ long ScanModulationTypes,
            /* [in] */ long AnalogVideoStandard) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IScanningTunerExVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IScanningTunerEx * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IScanningTunerEx * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IScanningTunerEx * This);
        
        DECLSPEC_XFGVIRT(ITuner, get_TuningSpace)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TuningSpace )( 
            __RPC__in IScanningTunerEx * This,
            /* [retval][out] */ __RPC__deref_out_opt ITuningSpace **TuningSpace);
        
        DECLSPEC_XFGVIRT(ITuner, put_TuningSpace)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_TuningSpace )( 
            __RPC__in IScanningTunerEx * This,
            /* [in] */ __RPC__in_opt ITuningSpace *TuningSpace);
        
        DECLSPEC_XFGVIRT(ITuner, EnumTuningSpaces)
        /* [helpstring][restricted][hidden] */ HRESULT ( STDMETHODCALLTYPE *EnumTuningSpaces )( 
            __RPC__in IScanningTunerEx * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumTuningSpaces **ppEnum);
        
        DECLSPEC_XFGVIRT(ITuner, get_TuneRequest)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TuneRequest )( 
            __RPC__in IScanningTunerEx * This,
            /* [retval][out] */ __RPC__deref_out_opt ITuneRequest **TuneRequest);
        
        DECLSPEC_XFGVIRT(ITuner, put_TuneRequest)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_TuneRequest )( 
            __RPC__in IScanningTunerEx * This,
            /* [in] */ __RPC__in_opt ITuneRequest *TuneRequest);
        
        DECLSPEC_XFGVIRT(ITuner, Validate)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Validate )( 
            __RPC__in IScanningTunerEx * This,
            /* [in] */ __RPC__in_opt ITuneRequest *TuneRequest);
        
        DECLSPEC_XFGVIRT(ITuner, get_PreferredComponentTypes)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PreferredComponentTypes )( 
            __RPC__in IScanningTunerEx * This,
            /* [retval][out] */ __RPC__deref_out_opt IComponentTypes **ComponentTypes);
        
        DECLSPEC_XFGVIRT(ITuner, put_PreferredComponentTypes)
        /* [helpstring][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PreferredComponentTypes )( 
            __RPC__in IScanningTunerEx * This,
            /* [in] */ __RPC__in_opt IComponentTypes *ComponentTypes);
        
        DECLSPEC_XFGVIRT(ITuner, get_SignalStrength)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SignalStrength )( 
            __RPC__in IScanningTunerEx * This,
            /* [retval][out] */ __RPC__out long *Strength);
        
        DECLSPEC_XFGVIRT(ITuner, TriggerSignalEvents)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *TriggerSignalEvents )( 
            __RPC__in IScanningTunerEx * This,
            /* [in] */ long Interval);
        
        DECLSPEC_XFGVIRT(IScanningTuner, SeekUp)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SeekUp )( 
            __RPC__in IScanningTunerEx * This);
        
        DECLSPEC_XFGVIRT(IScanningTuner, SeekDown)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SeekDown )( 
            __RPC__in IScanningTunerEx * This);
        
        DECLSPEC_XFGVIRT(IScanningTuner, ScanUp)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ScanUp )( 
            __RPC__in IScanningTunerEx * This,
            /* [in] */ long MillisecondsPause);
        
        DECLSPEC_XFGVIRT(IScanningTuner, ScanDown)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ScanDown )( 
            __RPC__in IScanningTunerEx * This,
            /* [in] */ long MillisecondsPause);
        
        DECLSPEC_XFGVIRT(IScanningTuner, AutoProgram)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AutoProgram )( 
            __RPC__in IScanningTunerEx * This);
        
        DECLSPEC_XFGVIRT(IScanningTunerEx, GetCurrentLocator)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetCurrentLocator )( 
            __RPC__in IScanningTunerEx * This,
            __RPC__deref_in_opt ILocator **pILocator);
        
        DECLSPEC_XFGVIRT(IScanningTunerEx, PerformExhaustiveScan)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *PerformExhaustiveScan )( 
            __RPC__in IScanningTunerEx * This,
            /* [in] */ long dwLowerFreq,
            /* [in] */ long dwHigherFreq,
            /* [in] */ VARIANT_BOOL bFineTune,
            /* [in] */ HEVENT hEvent);
        
        DECLSPEC_XFGVIRT(IScanningTunerEx, TerminateCurrentScan)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *TerminateCurrentScan )( 
            __RPC__in IScanningTunerEx * This,
            /* [retval][out] */ __RPC__out long *pcurrentFreq);
        
        DECLSPEC_XFGVIRT(IScanningTunerEx, ResumeCurrentScan)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *ResumeCurrentScan )( 
            __RPC__in IScanningTunerEx * This,
            /* [in] */ HEVENT hEvent);
        
        DECLSPEC_XFGVIRT(IScanningTunerEx, GetTunerScanningCapability)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetTunerScanningCapability )( 
            __RPC__in IScanningTunerEx * This,
            /* [out] */ __RPC__out long *HardwareAssistedScanning,
            /* [out] */ __RPC__out long *NumStandardsSupported,
            /* [out] */ __RPC__out GUID *BroadcastStandards);
        
        DECLSPEC_XFGVIRT(IScanningTunerEx, GetTunerStatus)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetTunerStatus )( 
            __RPC__in IScanningTunerEx * This,
            /* [out] */ __RPC__out long *SecondsLeft,
            /* [out] */ __RPC__out long *CurrentLockType,
            /* [out] */ __RPC__out long *AutoDetect,
            /* [out] */ __RPC__out long *CurrentFreq);
        
        DECLSPEC_XFGVIRT(IScanningTunerEx, GetCurrentTunerStandardCapability)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetCurrentTunerStandardCapability )( 
            __RPC__in IScanningTunerEx * This,
            /* [in] */ GUID CurrentBroadcastStandard,
            /* [out] */ __RPC__out long *SettlingTime,
            /* [out] */ __RPC__out long *TvStandardsSupported);
        
        DECLSPEC_XFGVIRT(IScanningTunerEx, SetScanSignalTypeFilter)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetScanSignalTypeFilter )( 
            __RPC__in IScanningTunerEx * This,
            /* [in] */ long ScanModulationTypes,
            /* [in] */ long AnalogVideoStandard);
        
        END_INTERFACE
    } IScanningTunerExVtbl;

    interface IScanningTunerEx
    {
        CONST_VTBL struct IScanningTunerExVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IScanningTunerEx_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IScanningTunerEx_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IScanningTunerEx_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IScanningTunerEx_get_TuningSpace(This,TuningSpace)	\
    ( (This)->lpVtbl -> get_TuningSpace(This,TuningSpace) ) 

#define IScanningTunerEx_put_TuningSpace(This,TuningSpace)	\
    ( (This)->lpVtbl -> put_TuningSpace(This,TuningSpace) ) 

#define IScanningTunerEx_EnumTuningSpaces(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumTuningSpaces(This,ppEnum) ) 

#define IScanningTunerEx_get_TuneRequest(This,TuneRequest)	\
    ( (This)->lpVtbl -> get_TuneRequest(This,TuneRequest) ) 

#define IScanningTunerEx_put_TuneRequest(This,TuneRequest)	\
    ( (This)->lpVtbl -> put_TuneRequest(This,TuneRequest) ) 

#define IScanningTunerEx_Validate(This,TuneRequest)	\
    ( (This)->lpVtbl -> Validate(This,TuneRequest) ) 

#define IScanningTunerEx_get_PreferredComponentTypes(This,ComponentTypes)	\
    ( (This)->lpVtbl -> get_PreferredComponentTypes(This,ComponentTypes) ) 

#define IScanningTunerEx_put_PreferredComponentTypes(This,ComponentTypes)	\
    ( (This)->lpVtbl -> put_PreferredComponentTypes(This,ComponentTypes) ) 

#define IScanningTunerEx_get_SignalStrength(This,Strength)	\
    ( (This)->lpVtbl -> get_SignalStrength(This,Strength) ) 

#define IScanningTunerEx_TriggerSignalEvents(This,Interval)	\
    ( (This)->lpVtbl -> TriggerSignalEvents(This,Interval) ) 


#define IScanningTunerEx_SeekUp(This)	\
    ( (This)->lpVtbl -> SeekUp(This) ) 

#define IScanningTunerEx_SeekDown(This)	\
    ( (This)->lpVtbl -> SeekDown(This) ) 

#define IScanningTunerEx_ScanUp(This,MillisecondsPause)	\
    ( (This)->lpVtbl -> ScanUp(This,MillisecondsPause) ) 

#define IScanningTunerEx_ScanDown(This,MillisecondsPause)	\
    ( (This)->lpVtbl -> ScanDown(This,MillisecondsPause) ) 

#define IScanningTunerEx_AutoProgram(This)	\
    ( (This)->lpVtbl -> AutoProgram(This) ) 


#define IScanningTunerEx_GetCurrentLocator(This,pILocator)	\
    ( (This)->lpVtbl -> GetCurrentLocator(This,pILocator) ) 

#define IScanningTunerEx_PerformExhaustiveScan(This,dwLowerFreq,dwHigherFreq,bFineTune,hEvent)	\
    ( (This)->lpVtbl -> PerformExhaustiveScan(This,dwLowerFreq,dwHigherFreq,bFineTune,hEvent) ) 

#define IScanningTunerEx_TerminateCurrentScan(This,pcurrentFreq)	\
    ( (This)->lpVtbl -> TerminateCurrentScan(This,pcurrentFreq) ) 

#define IScanningTunerEx_ResumeCurrentScan(This,hEvent)	\
    ( (This)->lpVtbl -> ResumeCurrentScan(This,hEvent) ) 

#define IScanningTunerEx_GetTunerScanningCapability(This,HardwareAssistedScanning,NumStandardsSupported,BroadcastStandards)	\
    ( (This)->lpVtbl -> GetTunerScanningCapability(This,HardwareAssistedScanning,NumStandardsSupported,BroadcastStandards) ) 

#define IScanningTunerEx_GetTunerStatus(This,SecondsLeft,CurrentLockType,AutoDetect,CurrentFreq)	\
    ( (This)->lpVtbl -> GetTunerStatus(This,SecondsLeft,CurrentLockType,AutoDetect,CurrentFreq) ) 

#define IScanningTunerEx_GetCurrentTunerStandardCapability(This,CurrentBroadcastStandard,SettlingTime,TvStandardsSupported)	\
    ( (This)->lpVtbl -> GetCurrentTunerStandardCapability(This,CurrentBroadcastStandard,SettlingTime,TvStandardsSupported) ) 

#define IScanningTunerEx_SetScanSignalTypeFilter(This,ScanModulationTypes,AnalogVideoStandard)	\
    ( (This)->lpVtbl -> SetScanSignalTypeFilter(This,ScanModulationTypes,AnalogVideoStandard) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IScanningTunerEx_INTERFACE_DEFINED__ */


#ifndef __IComponentType_INTERFACE_DEFINED__
#define __IComponentType_INTERFACE_DEFINED__

/* interface IComponentType */
/* [unique][helpstring][proxy][oleautomation][dual][uuid][nonextensible][hidden][object] */ 


EXTERN_C const IID IID_IComponentType;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6A340DC0-0311-11d3-9D8E-00C04F72D980")
    IComponentType : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Category( 
            /* [retval][out] */ __RPC__out ComponentCategory *Category) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Category( 
            /* [in] */ ComponentCategory Category) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MediaMajorType( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *MediaMajorType) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_MediaMajorType( 
            /* [in] */ __RPC__in BSTR MediaMajorType) = 0;
        
        virtual /* [restricted][hidden][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get__MediaMajorType( 
            /* [retval][out] */ __RPC__out GUID *MediaMajorTypeGuid) = 0;
        
        virtual /* [restricted][hidden][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put__MediaMajorType( 
            /* [in] */ __RPC__in REFCLSID MediaMajorTypeGuid) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MediaSubType( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *MediaSubType) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_MediaSubType( 
            /* [in] */ __RPC__in BSTR MediaSubType) = 0;
        
        virtual /* [restricted][hidden][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get__MediaSubType( 
            /* [retval][out] */ __RPC__out GUID *MediaSubTypeGuid) = 0;
        
        virtual /* [restricted][hidden][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put__MediaSubType( 
            /* [in] */ __RPC__in REFCLSID MediaSubTypeGuid) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MediaFormatType( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *MediaFormatType) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_MediaFormatType( 
            /* [in] */ __RPC__in BSTR MediaFormatType) = 0;
        
        virtual /* [restricted][hidden][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get__MediaFormatType( 
            /* [retval][out] */ __RPC__out GUID *MediaFormatTypeGuid) = 0;
        
        virtual /* [restricted][hidden][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put__MediaFormatType( 
            /* [in] */ __RPC__in REFCLSID MediaFormatTypeGuid) = 0;
        
        virtual /* [helpstring][restricted][hidden][id][propget] */ HRESULT STDMETHODCALLTYPE get_MediaType( 
            /* [retval][out] */ __RPC__out AM_MEDIA_TYPE *MediaType) = 0;
        
        virtual /* [helpstring][restricted][hidden][id][propput] */ HRESULT STDMETHODCALLTYPE put_MediaType( 
            /* [in] */ __RPC__in AM_MEDIA_TYPE *MediaType) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt IComponentType **NewCT) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IComponentTypeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IComponentType * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IComponentType * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IComponentType * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IComponentType * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IComponentType * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IComponentType * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IComponentType * This,
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
        
        DECLSPEC_XFGVIRT(IComponentType, get_Category)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Category )( 
            __RPC__in IComponentType * This,
            /* [retval][out] */ __RPC__out ComponentCategory *Category);
        
        DECLSPEC_XFGVIRT(IComponentType, put_Category)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Category )( 
            __RPC__in IComponentType * This,
            /* [in] */ ComponentCategory Category);
        
        DECLSPEC_XFGVIRT(IComponentType, get_MediaMajorType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MediaMajorType )( 
            __RPC__in IComponentType * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *MediaMajorType);
        
        DECLSPEC_XFGVIRT(IComponentType, put_MediaMajorType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MediaMajorType )( 
            __RPC__in IComponentType * This,
            /* [in] */ __RPC__in BSTR MediaMajorType);
        
        DECLSPEC_XFGVIRT(IComponentType, get__MediaMajorType)
        /* [restricted][hidden][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__MediaMajorType )( 
            __RPC__in IComponentType * This,
            /* [retval][out] */ __RPC__out GUID *MediaMajorTypeGuid);
        
        DECLSPEC_XFGVIRT(IComponentType, put__MediaMajorType)
        /* [restricted][hidden][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put__MediaMajorType )( 
            __RPC__in IComponentType * This,
            /* [in] */ __RPC__in REFCLSID MediaMajorTypeGuid);
        
        DECLSPEC_XFGVIRT(IComponentType, get_MediaSubType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MediaSubType )( 
            __RPC__in IComponentType * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *MediaSubType);
        
        DECLSPEC_XFGVIRT(IComponentType, put_MediaSubType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MediaSubType )( 
            __RPC__in IComponentType * This,
            /* [in] */ __RPC__in BSTR MediaSubType);
        
        DECLSPEC_XFGVIRT(IComponentType, get__MediaSubType)
        /* [restricted][hidden][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__MediaSubType )( 
            __RPC__in IComponentType * This,
            /* [retval][out] */ __RPC__out GUID *MediaSubTypeGuid);
        
        DECLSPEC_XFGVIRT(IComponentType, put__MediaSubType)
        /* [restricted][hidden][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put__MediaSubType )( 
            __RPC__in IComponentType * This,
            /* [in] */ __RPC__in REFCLSID MediaSubTypeGuid);
        
        DECLSPEC_XFGVIRT(IComponentType, get_MediaFormatType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MediaFormatType )( 
            __RPC__in IComponentType * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *MediaFormatType);
        
        DECLSPEC_XFGVIRT(IComponentType, put_MediaFormatType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MediaFormatType )( 
            __RPC__in IComponentType * This,
            /* [in] */ __RPC__in BSTR MediaFormatType);
        
        DECLSPEC_XFGVIRT(IComponentType, get__MediaFormatType)
        /* [restricted][hidden][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__MediaFormatType )( 
            __RPC__in IComponentType * This,
            /* [retval][out] */ __RPC__out GUID *MediaFormatTypeGuid);
        
        DECLSPEC_XFGVIRT(IComponentType, put__MediaFormatType)
        /* [restricted][hidden][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put__MediaFormatType )( 
            __RPC__in IComponentType * This,
            /* [in] */ __RPC__in REFCLSID MediaFormatTypeGuid);
        
        DECLSPEC_XFGVIRT(IComponentType, get_MediaType)
        /* [helpstring][restricted][hidden][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MediaType )( 
            __RPC__in IComponentType * This,
            /* [retval][out] */ __RPC__out AM_MEDIA_TYPE *MediaType);
        
        DECLSPEC_XFGVIRT(IComponentType, put_MediaType)
        /* [helpstring][restricted][hidden][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MediaType )( 
            __RPC__in IComponentType * This,
            /* [in] */ __RPC__in AM_MEDIA_TYPE *MediaType);
        
        DECLSPEC_XFGVIRT(IComponentType, Clone)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IComponentType * This,
            /* [retval][out] */ __RPC__deref_out_opt IComponentType **NewCT);
        
        END_INTERFACE
    } IComponentTypeVtbl;

    interface IComponentType
    {
        CONST_VTBL struct IComponentTypeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IComponentType_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IComponentType_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IComponentType_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IComponentType_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IComponentType_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IComponentType_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IComponentType_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IComponentType_get_Category(This,Category)	\
    ( (This)->lpVtbl -> get_Category(This,Category) ) 

#define IComponentType_put_Category(This,Category)	\
    ( (This)->lpVtbl -> put_Category(This,Category) ) 

#define IComponentType_get_MediaMajorType(This,MediaMajorType)	\
    ( (This)->lpVtbl -> get_MediaMajorType(This,MediaMajorType) ) 

#define IComponentType_put_MediaMajorType(This,MediaMajorType)	\
    ( (This)->lpVtbl -> put_MediaMajorType(This,MediaMajorType) ) 

#define IComponentType_get__MediaMajorType(This,MediaMajorTypeGuid)	\
    ( (This)->lpVtbl -> get__MediaMajorType(This,MediaMajorTypeGuid) ) 

#define IComponentType_put__MediaMajorType(This,MediaMajorTypeGuid)	\
    ( (This)->lpVtbl -> put__MediaMajorType(This,MediaMajorTypeGuid) ) 

#define IComponentType_get_MediaSubType(This,MediaSubType)	\
    ( (This)->lpVtbl -> get_MediaSubType(This,MediaSubType) ) 

#define IComponentType_put_MediaSubType(This,MediaSubType)	\
    ( (This)->lpVtbl -> put_MediaSubType(This,MediaSubType) ) 

#define IComponentType_get__MediaSubType(This,MediaSubTypeGuid)	\
    ( (This)->lpVtbl -> get__MediaSubType(This,MediaSubTypeGuid) ) 

#define IComponentType_put__MediaSubType(This,MediaSubTypeGuid)	\
    ( (This)->lpVtbl -> put__MediaSubType(This,MediaSubTypeGuid) ) 

#define IComponentType_get_MediaFormatType(This,MediaFormatType)	\
    ( (This)->lpVtbl -> get_MediaFormatType(This,MediaFormatType) ) 

#define IComponentType_put_MediaFormatType(This,MediaFormatType)	\
    ( (This)->lpVtbl -> put_MediaFormatType(This,MediaFormatType) ) 

#define IComponentType_get__MediaFormatType(This,MediaFormatTypeGuid)	\
    ( (This)->lpVtbl -> get__MediaFormatType(This,MediaFormatTypeGuid) ) 

#define IComponentType_put__MediaFormatType(This,MediaFormatTypeGuid)	\
    ( (This)->lpVtbl -> put__MediaFormatType(This,MediaFormatTypeGuid) ) 

#define IComponentType_get_MediaType(This,MediaType)	\
    ( (This)->lpVtbl -> get_MediaType(This,MediaType) ) 

#define IComponentType_put_MediaType(This,MediaType)	\
    ( (This)->lpVtbl -> put_MediaType(This,MediaType) ) 

#define IComponentType_Clone(This,NewCT)	\
    ( (This)->lpVtbl -> Clone(This,NewCT) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IComponentType_INTERFACE_DEFINED__ */


#ifndef __ILanguageComponentType_INTERFACE_DEFINED__
#define __ILanguageComponentType_INTERFACE_DEFINED__

/* interface ILanguageComponentType */
/* [unique][helpstring][proxy][oleautomation][dual][uuid][nonextensible][hidden][object] */ 


EXTERN_C const IID IID_ILanguageComponentType;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B874C8BA-0FA2-11d3-9D8E-00C04F72D980")
    ILanguageComponentType : public IComponentType
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_LangID( 
            /* [retval][out] */ __RPC__out long *LangID) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_LangID( 
            /* [in] */ long LangID) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ILanguageComponentTypeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ILanguageComponentType * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ILanguageComponentType * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ILanguageComponentType * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ILanguageComponentType * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ILanguageComponentType * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ILanguageComponentType * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ILanguageComponentType * This,
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
        
        DECLSPEC_XFGVIRT(IComponentType, get_Category)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Category )( 
            __RPC__in ILanguageComponentType * This,
            /* [retval][out] */ __RPC__out ComponentCategory *Category);
        
        DECLSPEC_XFGVIRT(IComponentType, put_Category)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Category )( 
            __RPC__in ILanguageComponentType * This,
            /* [in] */ ComponentCategory Category);
        
        DECLSPEC_XFGVIRT(IComponentType, get_MediaMajorType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MediaMajorType )( 
            __RPC__in ILanguageComponentType * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *MediaMajorType);
        
        DECLSPEC_XFGVIRT(IComponentType, put_MediaMajorType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MediaMajorType )( 
            __RPC__in ILanguageComponentType * This,
            /* [in] */ __RPC__in BSTR MediaMajorType);
        
        DECLSPEC_XFGVIRT(IComponentType, get__MediaMajorType)
        /* [restricted][hidden][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__MediaMajorType )( 
            __RPC__in ILanguageComponentType * This,
            /* [retval][out] */ __RPC__out GUID *MediaMajorTypeGuid);
        
        DECLSPEC_XFGVIRT(IComponentType, put__MediaMajorType)
        /* [restricted][hidden][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put__MediaMajorType )( 
            __RPC__in ILanguageComponentType * This,
            /* [in] */ __RPC__in REFCLSID MediaMajorTypeGuid);
        
        DECLSPEC_XFGVIRT(IComponentType, get_MediaSubType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MediaSubType )( 
            __RPC__in ILanguageComponentType * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *MediaSubType);
        
        DECLSPEC_XFGVIRT(IComponentType, put_MediaSubType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MediaSubType )( 
            __RPC__in ILanguageComponentType * This,
            /* [in] */ __RPC__in BSTR MediaSubType);
        
        DECLSPEC_XFGVIRT(IComponentType, get__MediaSubType)
        /* [restricted][hidden][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__MediaSubType )( 
            __RPC__in ILanguageComponentType * This,
            /* [retval][out] */ __RPC__out GUID *MediaSubTypeGuid);
        
        DECLSPEC_XFGVIRT(IComponentType, put__MediaSubType)
        /* [restricted][hidden][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put__MediaSubType )( 
            __RPC__in ILanguageComponentType * This,
            /* [in] */ __RPC__in REFCLSID MediaSubTypeGuid);
        
        DECLSPEC_XFGVIRT(IComponentType, get_MediaFormatType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MediaFormatType )( 
            __RPC__in ILanguageComponentType * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *MediaFormatType);
        
        DECLSPEC_XFGVIRT(IComponentType, put_MediaFormatType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MediaFormatType )( 
            __RPC__in ILanguageComponentType * This,
            /* [in] */ __RPC__in BSTR MediaFormatType);
        
        DECLSPEC_XFGVIRT(IComponentType, get__MediaFormatType)
        /* [restricted][hidden][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__MediaFormatType )( 
            __RPC__in ILanguageComponentType * This,
            /* [retval][out] */ __RPC__out GUID *MediaFormatTypeGuid);
        
        DECLSPEC_XFGVIRT(IComponentType, put__MediaFormatType)
        /* [restricted][hidden][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put__MediaFormatType )( 
            __RPC__in ILanguageComponentType * This,
            /* [in] */ __RPC__in REFCLSID MediaFormatTypeGuid);
        
        DECLSPEC_XFGVIRT(IComponentType, get_MediaType)
        /* [helpstring][restricted][hidden][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MediaType )( 
            __RPC__in ILanguageComponentType * This,
            /* [retval][out] */ __RPC__out AM_MEDIA_TYPE *MediaType);
        
        DECLSPEC_XFGVIRT(IComponentType, put_MediaType)
        /* [helpstring][restricted][hidden][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MediaType )( 
            __RPC__in ILanguageComponentType * This,
            /* [in] */ __RPC__in AM_MEDIA_TYPE *MediaType);
        
        DECLSPEC_XFGVIRT(IComponentType, Clone)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in ILanguageComponentType * This,
            /* [retval][out] */ __RPC__deref_out_opt IComponentType **NewCT);
        
        DECLSPEC_XFGVIRT(ILanguageComponentType, get_LangID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LangID )( 
            __RPC__in ILanguageComponentType * This,
            /* [retval][out] */ __RPC__out long *LangID);
        
        DECLSPEC_XFGVIRT(ILanguageComponentType, put_LangID)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_LangID )( 
            __RPC__in ILanguageComponentType * This,
            /* [in] */ long LangID);
        
        END_INTERFACE
    } ILanguageComponentTypeVtbl;

    interface ILanguageComponentType
    {
        CONST_VTBL struct ILanguageComponentTypeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ILanguageComponentType_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ILanguageComponentType_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ILanguageComponentType_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ILanguageComponentType_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ILanguageComponentType_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ILanguageComponentType_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ILanguageComponentType_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ILanguageComponentType_get_Category(This,Category)	\
    ( (This)->lpVtbl -> get_Category(This,Category) ) 

#define ILanguageComponentType_put_Category(This,Category)	\
    ( (This)->lpVtbl -> put_Category(This,Category) ) 

#define ILanguageComponentType_get_MediaMajorType(This,MediaMajorType)	\
    ( (This)->lpVtbl -> get_MediaMajorType(This,MediaMajorType) ) 

#define ILanguageComponentType_put_MediaMajorType(This,MediaMajorType)	\
    ( (This)->lpVtbl -> put_MediaMajorType(This,MediaMajorType) ) 

#define ILanguageComponentType_get__MediaMajorType(This,MediaMajorTypeGuid)	\
    ( (This)->lpVtbl -> get__MediaMajorType(This,MediaMajorTypeGuid) ) 

#define ILanguageComponentType_put__MediaMajorType(This,MediaMajorTypeGuid)	\
    ( (This)->lpVtbl -> put__MediaMajorType(This,MediaMajorTypeGuid) ) 

#define ILanguageComponentType_get_MediaSubType(This,MediaSubType)	\
    ( (This)->lpVtbl -> get_MediaSubType(This,MediaSubType) ) 

#define ILanguageComponentType_put_MediaSubType(This,MediaSubType)	\
    ( (This)->lpVtbl -> put_MediaSubType(This,MediaSubType) ) 

#define ILanguageComponentType_get__MediaSubType(This,MediaSubTypeGuid)	\
    ( (This)->lpVtbl -> get__MediaSubType(This,MediaSubTypeGuid) ) 

#define ILanguageComponentType_put__MediaSubType(This,MediaSubTypeGuid)	\
    ( (This)->lpVtbl -> put__MediaSubType(This,MediaSubTypeGuid) ) 

#define ILanguageComponentType_get_MediaFormatType(This,MediaFormatType)	\
    ( (This)->lpVtbl -> get_MediaFormatType(This,MediaFormatType) ) 

#define ILanguageComponentType_put_MediaFormatType(This,MediaFormatType)	\
    ( (This)->lpVtbl -> put_MediaFormatType(This,MediaFormatType) ) 

#define ILanguageComponentType_get__MediaFormatType(This,MediaFormatTypeGuid)	\
    ( (This)->lpVtbl -> get__MediaFormatType(This,MediaFormatTypeGuid) ) 

#define ILanguageComponentType_put__MediaFormatType(This,MediaFormatTypeGuid)	\
    ( (This)->lpVtbl -> put__MediaFormatType(This,MediaFormatTypeGuid) ) 

#define ILanguageComponentType_get_MediaType(This,MediaType)	\
    ( (This)->lpVtbl -> get_MediaType(This,MediaType) ) 

#define ILanguageComponentType_put_MediaType(This,MediaType)	\
    ( (This)->lpVtbl -> put_MediaType(This,MediaType) ) 

#define ILanguageComponentType_Clone(This,NewCT)	\
    ( (This)->lpVtbl -> Clone(This,NewCT) ) 


#define ILanguageComponentType_get_LangID(This,LangID)	\
    ( (This)->lpVtbl -> get_LangID(This,LangID) ) 

#define ILanguageComponentType_put_LangID(This,LangID)	\
    ( (This)->lpVtbl -> put_LangID(This,LangID) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ILanguageComponentType_INTERFACE_DEFINED__ */


#ifndef __IMPEG2ComponentType_INTERFACE_DEFINED__
#define __IMPEG2ComponentType_INTERFACE_DEFINED__

/* interface IMPEG2ComponentType */
/* [unique][helpstring][proxy][oleautomation][dual][uuid][nonextensible][hidden][object] */ 


EXTERN_C const IID IID_IMPEG2ComponentType;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2C073D84-B51C-48c9-AA9F-68971E1F6E38")
    IMPEG2ComponentType : public ILanguageComponentType
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_StreamType( 
            /* [retval][out] */ __RPC__out MPEG2StreamType *MP2StreamType) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_StreamType( 
            /* [in] */ MPEG2StreamType MP2StreamType) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMPEG2ComponentTypeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMPEG2ComponentType * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMPEG2ComponentType * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMPEG2ComponentType * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMPEG2ComponentType * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMPEG2ComponentType * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMPEG2ComponentType * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMPEG2ComponentType * This,
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
        
        DECLSPEC_XFGVIRT(IComponentType, get_Category)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Category )( 
            __RPC__in IMPEG2ComponentType * This,
            /* [retval][out] */ __RPC__out ComponentCategory *Category);
        
        DECLSPEC_XFGVIRT(IComponentType, put_Category)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Category )( 
            __RPC__in IMPEG2ComponentType * This,
            /* [in] */ ComponentCategory Category);
        
        DECLSPEC_XFGVIRT(IComponentType, get_MediaMajorType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MediaMajorType )( 
            __RPC__in IMPEG2ComponentType * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *MediaMajorType);
        
        DECLSPEC_XFGVIRT(IComponentType, put_MediaMajorType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MediaMajorType )( 
            __RPC__in IMPEG2ComponentType * This,
            /* [in] */ __RPC__in BSTR MediaMajorType);
        
        DECLSPEC_XFGVIRT(IComponentType, get__MediaMajorType)
        /* [restricted][hidden][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__MediaMajorType )( 
            __RPC__in IMPEG2ComponentType * This,
            /* [retval][out] */ __RPC__out GUID *MediaMajorTypeGuid);
        
        DECLSPEC_XFGVIRT(IComponentType, put__MediaMajorType)
        /* [restricted][hidden][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put__MediaMajorType )( 
            __RPC__in IMPEG2ComponentType * This,
            /* [in] */ __RPC__in REFCLSID MediaMajorTypeGuid);
        
        DECLSPEC_XFGVIRT(IComponentType, get_MediaSubType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MediaSubType )( 
            __RPC__in IMPEG2ComponentType * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *MediaSubType);
        
        DECLSPEC_XFGVIRT(IComponentType, put_MediaSubType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MediaSubType )( 
            __RPC__in IMPEG2ComponentType * This,
            /* [in] */ __RPC__in BSTR MediaSubType);
        
        DECLSPEC_XFGVIRT(IComponentType, get__MediaSubType)
        /* [restricted][hidden][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__MediaSubType )( 
            __RPC__in IMPEG2ComponentType * This,
            /* [retval][out] */ __RPC__out GUID *MediaSubTypeGuid);
        
        DECLSPEC_XFGVIRT(IComponentType, put__MediaSubType)
        /* [restricted][hidden][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put__MediaSubType )( 
            __RPC__in IMPEG2ComponentType * This,
            /* [in] */ __RPC__in REFCLSID MediaSubTypeGuid);
        
        DECLSPEC_XFGVIRT(IComponentType, get_MediaFormatType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MediaFormatType )( 
            __RPC__in IMPEG2ComponentType * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *MediaFormatType);
        
        DECLSPEC_XFGVIRT(IComponentType, put_MediaFormatType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MediaFormatType )( 
            __RPC__in IMPEG2ComponentType * This,
            /* [in] */ __RPC__in BSTR MediaFormatType);
        
        DECLSPEC_XFGVIRT(IComponentType, get__MediaFormatType)
        /* [restricted][hidden][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__MediaFormatType )( 
            __RPC__in IMPEG2ComponentType * This,
            /* [retval][out] */ __RPC__out GUID *MediaFormatTypeGuid);
        
        DECLSPEC_XFGVIRT(IComponentType, put__MediaFormatType)
        /* [restricted][hidden][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put__MediaFormatType )( 
            __RPC__in IMPEG2ComponentType * This,
            /* [in] */ __RPC__in REFCLSID MediaFormatTypeGuid);
        
        DECLSPEC_XFGVIRT(IComponentType, get_MediaType)
        /* [helpstring][restricted][hidden][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MediaType )( 
            __RPC__in IMPEG2ComponentType * This,
            /* [retval][out] */ __RPC__out AM_MEDIA_TYPE *MediaType);
        
        DECLSPEC_XFGVIRT(IComponentType, put_MediaType)
        /* [helpstring][restricted][hidden][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MediaType )( 
            __RPC__in IMPEG2ComponentType * This,
            /* [in] */ __RPC__in AM_MEDIA_TYPE *MediaType);
        
        DECLSPEC_XFGVIRT(IComponentType, Clone)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IMPEG2ComponentType * This,
            /* [retval][out] */ __RPC__deref_out_opt IComponentType **NewCT);
        
        DECLSPEC_XFGVIRT(ILanguageComponentType, get_LangID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LangID )( 
            __RPC__in IMPEG2ComponentType * This,
            /* [retval][out] */ __RPC__out long *LangID);
        
        DECLSPEC_XFGVIRT(ILanguageComponentType, put_LangID)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_LangID )( 
            __RPC__in IMPEG2ComponentType * This,
            /* [in] */ long LangID);
        
        DECLSPEC_XFGVIRT(IMPEG2ComponentType, get_StreamType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StreamType )( 
            __RPC__in IMPEG2ComponentType * This,
            /* [retval][out] */ __RPC__out MPEG2StreamType *MP2StreamType);
        
        DECLSPEC_XFGVIRT(IMPEG2ComponentType, put_StreamType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_StreamType )( 
            __RPC__in IMPEG2ComponentType * This,
            /* [in] */ MPEG2StreamType MP2StreamType);
        
        END_INTERFACE
    } IMPEG2ComponentTypeVtbl;

    interface IMPEG2ComponentType
    {
        CONST_VTBL struct IMPEG2ComponentTypeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMPEG2ComponentType_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMPEG2ComponentType_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMPEG2ComponentType_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMPEG2ComponentType_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMPEG2ComponentType_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMPEG2ComponentType_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMPEG2ComponentType_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMPEG2ComponentType_get_Category(This,Category)	\
    ( (This)->lpVtbl -> get_Category(This,Category) ) 

#define IMPEG2ComponentType_put_Category(This,Category)	\
    ( (This)->lpVtbl -> put_Category(This,Category) ) 

#define IMPEG2ComponentType_get_MediaMajorType(This,MediaMajorType)	\
    ( (This)->lpVtbl -> get_MediaMajorType(This,MediaMajorType) ) 

#define IMPEG2ComponentType_put_MediaMajorType(This,MediaMajorType)	\
    ( (This)->lpVtbl -> put_MediaMajorType(This,MediaMajorType) ) 

#define IMPEG2ComponentType_get__MediaMajorType(This,MediaMajorTypeGuid)	\
    ( (This)->lpVtbl -> get__MediaMajorType(This,MediaMajorTypeGuid) ) 

#define IMPEG2ComponentType_put__MediaMajorType(This,MediaMajorTypeGuid)	\
    ( (This)->lpVtbl -> put__MediaMajorType(This,MediaMajorTypeGuid) ) 

#define IMPEG2ComponentType_get_MediaSubType(This,MediaSubType)	\
    ( (This)->lpVtbl -> get_MediaSubType(This,MediaSubType) ) 

#define IMPEG2ComponentType_put_MediaSubType(This,MediaSubType)	\
    ( (This)->lpVtbl -> put_MediaSubType(This,MediaSubType) ) 

#define IMPEG2ComponentType_get__MediaSubType(This,MediaSubTypeGuid)	\
    ( (This)->lpVtbl -> get__MediaSubType(This,MediaSubTypeGuid) ) 

#define IMPEG2ComponentType_put__MediaSubType(This,MediaSubTypeGuid)	\
    ( (This)->lpVtbl -> put__MediaSubType(This,MediaSubTypeGuid) ) 

#define IMPEG2ComponentType_get_MediaFormatType(This,MediaFormatType)	\
    ( (This)->lpVtbl -> get_MediaFormatType(This,MediaFormatType) ) 

#define IMPEG2ComponentType_put_MediaFormatType(This,MediaFormatType)	\
    ( (This)->lpVtbl -> put_MediaFormatType(This,MediaFormatType) ) 

#define IMPEG2ComponentType_get__MediaFormatType(This,MediaFormatTypeGuid)	\
    ( (This)->lpVtbl -> get__MediaFormatType(This,MediaFormatTypeGuid) ) 

#define IMPEG2ComponentType_put__MediaFormatType(This,MediaFormatTypeGuid)	\
    ( (This)->lpVtbl -> put__MediaFormatType(This,MediaFormatTypeGuid) ) 

#define IMPEG2ComponentType_get_MediaType(This,MediaType)	\
    ( (This)->lpVtbl -> get_MediaType(This,MediaType) ) 

#define IMPEG2ComponentType_put_MediaType(This,MediaType)	\
    ( (This)->lpVtbl -> put_MediaType(This,MediaType) ) 

#define IMPEG2ComponentType_Clone(This,NewCT)	\
    ( (This)->lpVtbl -> Clone(This,NewCT) ) 


#define IMPEG2ComponentType_get_LangID(This,LangID)	\
    ( (This)->lpVtbl -> get_LangID(This,LangID) ) 

#define IMPEG2ComponentType_put_LangID(This,LangID)	\
    ( (This)->lpVtbl -> put_LangID(This,LangID) ) 


#define IMPEG2ComponentType_get_StreamType(This,MP2StreamType)	\
    ( (This)->lpVtbl -> get_StreamType(This,MP2StreamType) ) 

#define IMPEG2ComponentType_put_StreamType(This,MP2StreamType)	\
    ( (This)->lpVtbl -> put_StreamType(This,MP2StreamType) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMPEG2ComponentType_INTERFACE_DEFINED__ */


#ifndef __IATSCComponentType_INTERFACE_DEFINED__
#define __IATSCComponentType_INTERFACE_DEFINED__

/* interface IATSCComponentType */
/* [unique][helpstring][proxy][oleautomation][dual][uuid][nonextensible][hidden][object] */ 


EXTERN_C const IID IID_IATSCComponentType;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("FC189E4D-7BD4-4125-B3B3-3A76A332CC96")
    IATSCComponentType : public IMPEG2ComponentType
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Flags( 
            /* [retval][out] */ __RPC__out long *Flags) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Flags( 
            /* [in] */ long flags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IATSCComponentTypeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IATSCComponentType * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IATSCComponentType * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IATSCComponentType * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IATSCComponentType * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IATSCComponentType * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IATSCComponentType * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IATSCComponentType * This,
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
        
        DECLSPEC_XFGVIRT(IComponentType, get_Category)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Category )( 
            __RPC__in IATSCComponentType * This,
            /* [retval][out] */ __RPC__out ComponentCategory *Category);
        
        DECLSPEC_XFGVIRT(IComponentType, put_Category)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Category )( 
            __RPC__in IATSCComponentType * This,
            /* [in] */ ComponentCategory Category);
        
        DECLSPEC_XFGVIRT(IComponentType, get_MediaMajorType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MediaMajorType )( 
            __RPC__in IATSCComponentType * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *MediaMajorType);
        
        DECLSPEC_XFGVIRT(IComponentType, put_MediaMajorType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MediaMajorType )( 
            __RPC__in IATSCComponentType * This,
            /* [in] */ __RPC__in BSTR MediaMajorType);
        
        DECLSPEC_XFGVIRT(IComponentType, get__MediaMajorType)
        /* [restricted][hidden][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__MediaMajorType )( 
            __RPC__in IATSCComponentType * This,
            /* [retval][out] */ __RPC__out GUID *MediaMajorTypeGuid);
        
        DECLSPEC_XFGVIRT(IComponentType, put__MediaMajorType)
        /* [restricted][hidden][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put__MediaMajorType )( 
            __RPC__in IATSCComponentType * This,
            /* [in] */ __RPC__in REFCLSID MediaMajorTypeGuid);
        
        DECLSPEC_XFGVIRT(IComponentType, get_MediaSubType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MediaSubType )( 
            __RPC__in IATSCComponentType * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *MediaSubType);
        
        DECLSPEC_XFGVIRT(IComponentType, put_MediaSubType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MediaSubType )( 
            __RPC__in IATSCComponentType * This,
            /* [in] */ __RPC__in BSTR MediaSubType);
        
        DECLSPEC_XFGVIRT(IComponentType, get__MediaSubType)
        /* [restricted][hidden][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__MediaSubType )( 
            __RPC__in IATSCComponentType * This,
            /* [retval][out] */ __RPC__out GUID *MediaSubTypeGuid);
        
        DECLSPEC_XFGVIRT(IComponentType, put__MediaSubType)
        /* [restricted][hidden][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put__MediaSubType )( 
            __RPC__in IATSCComponentType * This,
            /* [in] */ __RPC__in REFCLSID MediaSubTypeGuid);
        
        DECLSPEC_XFGVIRT(IComponentType, get_MediaFormatType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MediaFormatType )( 
            __RPC__in IATSCComponentType * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *MediaFormatType);
        
        DECLSPEC_XFGVIRT(IComponentType, put_MediaFormatType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MediaFormatType )( 
            __RPC__in IATSCComponentType * This,
            /* [in] */ __RPC__in BSTR MediaFormatType);
        
        DECLSPEC_XFGVIRT(IComponentType, get__MediaFormatType)
        /* [restricted][hidden][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__MediaFormatType )( 
            __RPC__in IATSCComponentType * This,
            /* [retval][out] */ __RPC__out GUID *MediaFormatTypeGuid);
        
        DECLSPEC_XFGVIRT(IComponentType, put__MediaFormatType)
        /* [restricted][hidden][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put__MediaFormatType )( 
            __RPC__in IATSCComponentType * This,
            /* [in] */ __RPC__in REFCLSID MediaFormatTypeGuid);
        
        DECLSPEC_XFGVIRT(IComponentType, get_MediaType)
        /* [helpstring][restricted][hidden][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MediaType )( 
            __RPC__in IATSCComponentType * This,
            /* [retval][out] */ __RPC__out AM_MEDIA_TYPE *MediaType);
        
        DECLSPEC_XFGVIRT(IComponentType, put_MediaType)
        /* [helpstring][restricted][hidden][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MediaType )( 
            __RPC__in IATSCComponentType * This,
            /* [in] */ __RPC__in AM_MEDIA_TYPE *MediaType);
        
        DECLSPEC_XFGVIRT(IComponentType, Clone)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IATSCComponentType * This,
            /* [retval][out] */ __RPC__deref_out_opt IComponentType **NewCT);
        
        DECLSPEC_XFGVIRT(ILanguageComponentType, get_LangID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LangID )( 
            __RPC__in IATSCComponentType * This,
            /* [retval][out] */ __RPC__out long *LangID);
        
        DECLSPEC_XFGVIRT(ILanguageComponentType, put_LangID)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_LangID )( 
            __RPC__in IATSCComponentType * This,
            /* [in] */ long LangID);
        
        DECLSPEC_XFGVIRT(IMPEG2ComponentType, get_StreamType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_StreamType )( 
            __RPC__in IATSCComponentType * This,
            /* [retval][out] */ __RPC__out MPEG2StreamType *MP2StreamType);
        
        DECLSPEC_XFGVIRT(IMPEG2ComponentType, put_StreamType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_StreamType )( 
            __RPC__in IATSCComponentType * This,
            /* [in] */ MPEG2StreamType MP2StreamType);
        
        DECLSPEC_XFGVIRT(IATSCComponentType, get_Flags)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Flags )( 
            __RPC__in IATSCComponentType * This,
            /* [retval][out] */ __RPC__out long *Flags);
        
        DECLSPEC_XFGVIRT(IATSCComponentType, put_Flags)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Flags )( 
            __RPC__in IATSCComponentType * This,
            /* [in] */ long flags);
        
        END_INTERFACE
    } IATSCComponentTypeVtbl;

    interface IATSCComponentType
    {
        CONST_VTBL struct IATSCComponentTypeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IATSCComponentType_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IATSCComponentType_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IATSCComponentType_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IATSCComponentType_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IATSCComponentType_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IATSCComponentType_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IATSCComponentType_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IATSCComponentType_get_Category(This,Category)	\
    ( (This)->lpVtbl -> get_Category(This,Category) ) 

#define IATSCComponentType_put_Category(This,Category)	\
    ( (This)->lpVtbl -> put_Category(This,Category) ) 

#define IATSCComponentType_get_MediaMajorType(This,MediaMajorType)	\
    ( (This)->lpVtbl -> get_MediaMajorType(This,MediaMajorType) ) 

#define IATSCComponentType_put_MediaMajorType(This,MediaMajorType)	\
    ( (This)->lpVtbl -> put_MediaMajorType(This,MediaMajorType) ) 

#define IATSCComponentType_get__MediaMajorType(This,MediaMajorTypeGuid)	\
    ( (This)->lpVtbl -> get__MediaMajorType(This,MediaMajorTypeGuid) ) 

#define IATSCComponentType_put__MediaMajorType(This,MediaMajorTypeGuid)	\
    ( (This)->lpVtbl -> put__MediaMajorType(This,MediaMajorTypeGuid) ) 

#define IATSCComponentType_get_MediaSubType(This,MediaSubType)	\
    ( (This)->lpVtbl -> get_MediaSubType(This,MediaSubType) ) 

#define IATSCComponentType_put_MediaSubType(This,MediaSubType)	\
    ( (This)->lpVtbl -> put_MediaSubType(This,MediaSubType) ) 

#define IATSCComponentType_get__MediaSubType(This,MediaSubTypeGuid)	\
    ( (This)->lpVtbl -> get__MediaSubType(This,MediaSubTypeGuid) ) 

#define IATSCComponentType_put__MediaSubType(This,MediaSubTypeGuid)	\
    ( (This)->lpVtbl -> put__MediaSubType(This,MediaSubTypeGuid) ) 

#define IATSCComponentType_get_MediaFormatType(This,MediaFormatType)	\
    ( (This)->lpVtbl -> get_MediaFormatType(This,MediaFormatType) ) 

#define IATSCComponentType_put_MediaFormatType(This,MediaFormatType)	\
    ( (This)->lpVtbl -> put_MediaFormatType(This,MediaFormatType) ) 

#define IATSCComponentType_get__MediaFormatType(This,MediaFormatTypeGuid)	\
    ( (This)->lpVtbl -> get__MediaFormatType(This,MediaFormatTypeGuid) ) 

#define IATSCComponentType_put__MediaFormatType(This,MediaFormatTypeGuid)	\
    ( (This)->lpVtbl -> put__MediaFormatType(This,MediaFormatTypeGuid) ) 

#define IATSCComponentType_get_MediaType(This,MediaType)	\
    ( (This)->lpVtbl -> get_MediaType(This,MediaType) ) 

#define IATSCComponentType_put_MediaType(This,MediaType)	\
    ( (This)->lpVtbl -> put_MediaType(This,MediaType) ) 

#define IATSCComponentType_Clone(This,NewCT)	\
    ( (This)->lpVtbl -> Clone(This,NewCT) ) 


#define IATSCComponentType_get_LangID(This,LangID)	\
    ( (This)->lpVtbl -> get_LangID(This,LangID) ) 

#define IATSCComponentType_put_LangID(This,LangID)	\
    ( (This)->lpVtbl -> put_LangID(This,LangID) ) 


#define IATSCComponentType_get_StreamType(This,MP2StreamType)	\
    ( (This)->lpVtbl -> get_StreamType(This,MP2StreamType) ) 

#define IATSCComponentType_put_StreamType(This,MP2StreamType)	\
    ( (This)->lpVtbl -> put_StreamType(This,MP2StreamType) ) 


#define IATSCComponentType_get_Flags(This,Flags)	\
    ( (This)->lpVtbl -> get_Flags(This,Flags) ) 

#define IATSCComponentType_put_Flags(This,flags)	\
    ( (This)->lpVtbl -> put_Flags(This,flags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IATSCComponentType_INTERFACE_DEFINED__ */


#ifndef __IEnumComponentTypes_INTERFACE_DEFINED__
#define __IEnumComponentTypes_INTERFACE_DEFINED__

/* interface IEnumComponentTypes */
/* [unique][uuid][object][restricted][hidden] */ 


EXTERN_C const IID IID_IEnumComponentTypes;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8A674B4A-1F63-11d3-B64C-00C04F79498E")
    IEnumComponentTypes : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG celt,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) IComponentType **rgelt,
            /* [out] */ __RPC__out ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ __RPC__deref_out_opt IEnumComponentTypes **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumComponentTypesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumComponentTypes * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumComponentTypes * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumComponentTypes * This);
        
        DECLSPEC_XFGVIRT(IEnumComponentTypes, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IEnumComponentTypes * This,
            /* [in] */ ULONG celt,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) IComponentType **rgelt,
            /* [out] */ __RPC__out ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IEnumComponentTypes, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumComponentTypes * This,
            /* [in] */ ULONG celt);
        
        DECLSPEC_XFGVIRT(IEnumComponentTypes, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumComponentTypes * This);
        
        DECLSPEC_XFGVIRT(IEnumComponentTypes, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumComponentTypes * This,
            /* [out] */ __RPC__deref_out_opt IEnumComponentTypes **ppEnum);
        
        END_INTERFACE
    } IEnumComponentTypesVtbl;

    interface IEnumComponentTypes
    {
        CONST_VTBL struct IEnumComponentTypesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumComponentTypes_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumComponentTypes_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumComponentTypes_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumComponentTypes_Next(This,celt,rgelt,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,rgelt,pceltFetched) ) 

#define IEnumComponentTypes_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IEnumComponentTypes_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumComponentTypes_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumComponentTypes_INTERFACE_DEFINED__ */


#ifndef __IComponentTypes_INTERFACE_DEFINED__
#define __IComponentTypes_INTERFACE_DEFINED__

/* interface IComponentTypes */
/* [unique][helpstring][proxy][oleautomation][dual][uuid][nonextensible][hidden][object] */ 


EXTERN_C const IID IID_IComponentTypes;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0DC13D4A-0313-11d3-9D8E-00C04F72D980")
    IComponentTypes : public IDispatch
    {
    public:
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *Count) = 0;
        
        virtual /* [restricted][hidden][id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumVARIANT **ppNewEnum) = 0;
        
        virtual /* [helpstring][restricted][hidden] */ HRESULT STDMETHODCALLTYPE EnumComponentTypes( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumComponentTypes **ppNewEnum) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT Index,
            /* [retval][out] */ __RPC__deref_out_opt IComponentType **ComponentType) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Item( 
            /* [in] */ VARIANT Index,
            /* [in] */ __RPC__in_opt IComponentType *ComponentType) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Add( 
            /* [in] */ __RPC__in_opt IComponentType *ComponentType,
            /* [retval][out] */ __RPC__out VARIANT *NewIndex) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Remove( 
            /* [in] */ VARIANT Index) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt IComponentTypes **NewList) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IComponentTypesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IComponentTypes * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IComponentTypes * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IComponentTypes * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IComponentTypes * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IComponentTypes * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IComponentTypes * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IComponentTypes * This,
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
        
        DECLSPEC_XFGVIRT(IComponentTypes, get_Count)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IComponentTypes * This,
            /* [retval][out] */ __RPC__out long *Count);
        
        DECLSPEC_XFGVIRT(IComponentTypes, get__NewEnum)
        /* [restricted][hidden][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IComponentTypes * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumVARIANT **ppNewEnum);
        
        DECLSPEC_XFGVIRT(IComponentTypes, EnumComponentTypes)
        /* [helpstring][restricted][hidden] */ HRESULT ( STDMETHODCALLTYPE *EnumComponentTypes )( 
            __RPC__in IComponentTypes * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumComponentTypes **ppNewEnum);
        
        DECLSPEC_XFGVIRT(IComponentTypes, get_Item)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IComponentTypes * This,
            /* [in] */ VARIANT Index,
            /* [retval][out] */ __RPC__deref_out_opt IComponentType **ComponentType);
        
        DECLSPEC_XFGVIRT(IComponentTypes, put_Item)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Item )( 
            __RPC__in IComponentTypes * This,
            /* [in] */ VARIANT Index,
            /* [in] */ __RPC__in_opt IComponentType *ComponentType);
        
        DECLSPEC_XFGVIRT(IComponentTypes, Add)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Add )( 
            __RPC__in IComponentTypes * This,
            /* [in] */ __RPC__in_opt IComponentType *ComponentType,
            /* [retval][out] */ __RPC__out VARIANT *NewIndex);
        
        DECLSPEC_XFGVIRT(IComponentTypes, Remove)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Remove )( 
            __RPC__in IComponentTypes * This,
            /* [in] */ VARIANT Index);
        
        DECLSPEC_XFGVIRT(IComponentTypes, Clone)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IComponentTypes * This,
            /* [retval][out] */ __RPC__deref_out_opt IComponentTypes **NewList);
        
        END_INTERFACE
    } IComponentTypesVtbl;

    interface IComponentTypes
    {
        CONST_VTBL struct IComponentTypesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IComponentTypes_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IComponentTypes_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IComponentTypes_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IComponentTypes_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IComponentTypes_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IComponentTypes_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IComponentTypes_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IComponentTypes_get_Count(This,Count)	\
    ( (This)->lpVtbl -> get_Count(This,Count) ) 

#define IComponentTypes_get__NewEnum(This,ppNewEnum)	\
    ( (This)->lpVtbl -> get__NewEnum(This,ppNewEnum) ) 

#define IComponentTypes_EnumComponentTypes(This,ppNewEnum)	\
    ( (This)->lpVtbl -> EnumComponentTypes(This,ppNewEnum) ) 

#define IComponentTypes_get_Item(This,Index,ComponentType)	\
    ( (This)->lpVtbl -> get_Item(This,Index,ComponentType) ) 

#define IComponentTypes_put_Item(This,Index,ComponentType)	\
    ( (This)->lpVtbl -> put_Item(This,Index,ComponentType) ) 

#define IComponentTypes_Add(This,ComponentType,NewIndex)	\
    ( (This)->lpVtbl -> Add(This,ComponentType,NewIndex) ) 

#define IComponentTypes_Remove(This,Index)	\
    ( (This)->lpVtbl -> Remove(This,Index) ) 

#define IComponentTypes_Clone(This,NewList)	\
    ( (This)->lpVtbl -> Clone(This,NewList) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IComponentTypes_INTERFACE_DEFINED__ */


#ifndef __IComponent_INTERFACE_DEFINED__
#define __IComponent_INTERFACE_DEFINED__

/* interface IComponent */
/* [unique][helpstring][proxy][oleautomation][dual][uuid][nonextensible][object] */ 


EXTERN_C const IID IID_IComponent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1A5576FC-0E19-11d3-9D8E-00C04F72D980")
    IComponent : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Type( 
            /* [retval][out] */ __RPC__deref_out_opt IComponentType **CT) = 0;
        
        virtual /* [helpstring][id][propput][restricted][hidden] */ HRESULT STDMETHODCALLTYPE put_Type( 
            /* [in] */ __RPC__in_opt IComponentType *CT) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DescLangID( 
            /* [retval][out] */ __RPC__out long *LangID) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_DescLangID( 
            /* [in] */ long LangID) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Status( 
            /* [retval][out] */ __RPC__out ComponentStatus *Status) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Status( 
            /* [in] */ ComponentStatus Status) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Description( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Description) = 0;
        
        virtual /* [helpstring][id][propput][restricted][hidden] */ HRESULT STDMETHODCALLTYPE put_Description( 
            /* [in] */ __RPC__in BSTR Description) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt IComponent **NewComponent) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IComponentVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IComponent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IComponent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IComponent * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IComponent * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IComponent * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IComponent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IComponent * This,
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
        
        DECLSPEC_XFGVIRT(IComponent, get_Type)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Type )( 
            __RPC__in IComponent * This,
            /* [retval][out] */ __RPC__deref_out_opt IComponentType **CT);
        
        DECLSPEC_XFGVIRT(IComponent, put_Type)
        /* [helpstring][id][propput][restricted][hidden] */ HRESULT ( STDMETHODCALLTYPE *put_Type )( 
            __RPC__in IComponent * This,
            /* [in] */ __RPC__in_opt IComponentType *CT);
        
        DECLSPEC_XFGVIRT(IComponent, get_DescLangID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DescLangID )( 
            __RPC__in IComponent * This,
            /* [retval][out] */ __RPC__out long *LangID);
        
        DECLSPEC_XFGVIRT(IComponent, put_DescLangID)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DescLangID )( 
            __RPC__in IComponent * This,
            /* [in] */ long LangID);
        
        DECLSPEC_XFGVIRT(IComponent, get_Status)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Status )( 
            __RPC__in IComponent * This,
            /* [retval][out] */ __RPC__out ComponentStatus *Status);
        
        DECLSPEC_XFGVIRT(IComponent, put_Status)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Status )( 
            __RPC__in IComponent * This,
            /* [in] */ ComponentStatus Status);
        
        DECLSPEC_XFGVIRT(IComponent, get_Description)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IComponent * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Description);
        
        DECLSPEC_XFGVIRT(IComponent, put_Description)
        /* [helpstring][id][propput][restricted][hidden] */ HRESULT ( STDMETHODCALLTYPE *put_Description )( 
            __RPC__in IComponent * This,
            /* [in] */ __RPC__in BSTR Description);
        
        DECLSPEC_XFGVIRT(IComponent, Clone)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IComponent * This,
            /* [retval][out] */ __RPC__deref_out_opt IComponent **NewComponent);
        
        END_INTERFACE
    } IComponentVtbl;

    interface IComponent
    {
        CONST_VTBL struct IComponentVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IComponent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IComponent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IComponent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IComponent_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IComponent_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IComponent_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IComponent_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IComponent_get_Type(This,CT)	\
    ( (This)->lpVtbl -> get_Type(This,CT) ) 

#define IComponent_put_Type(This,CT)	\
    ( (This)->lpVtbl -> put_Type(This,CT) ) 

#define IComponent_get_DescLangID(This,LangID)	\
    ( (This)->lpVtbl -> get_DescLangID(This,LangID) ) 

#define IComponent_put_DescLangID(This,LangID)	\
    ( (This)->lpVtbl -> put_DescLangID(This,LangID) ) 

#define IComponent_get_Status(This,Status)	\
    ( (This)->lpVtbl -> get_Status(This,Status) ) 

#define IComponent_put_Status(This,Status)	\
    ( (This)->lpVtbl -> put_Status(This,Status) ) 

#define IComponent_get_Description(This,Description)	\
    ( (This)->lpVtbl -> get_Description(This,Description) ) 

#define IComponent_put_Description(This,Description)	\
    ( (This)->lpVtbl -> put_Description(This,Description) ) 

#define IComponent_Clone(This,NewComponent)	\
    ( (This)->lpVtbl -> Clone(This,NewComponent) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IComponent_INTERFACE_DEFINED__ */


#ifndef __IAnalogAudioComponentType_INTERFACE_DEFINED__
#define __IAnalogAudioComponentType_INTERFACE_DEFINED__

/* interface IAnalogAudioComponentType */
/* [unique][helpstring][proxy][oleautomation][dual][uuid][nonextensible][object] */ 


EXTERN_C const IID IID_IAnalogAudioComponentType;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2CFEB2A8-1787-4A24-A941-C6EAEC39C842")
    IAnalogAudioComponentType : public IComponentType
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_AnalogAudioMode( 
            /* [retval][out] */ __RPC__out TVAudioMode *Mode) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_AnalogAudioMode( 
            /* [in] */ TVAudioMode Mode) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAnalogAudioComponentTypeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAnalogAudioComponentType * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAnalogAudioComponentType * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAnalogAudioComponentType * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IAnalogAudioComponentType * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IAnalogAudioComponentType * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IAnalogAudioComponentType * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IAnalogAudioComponentType * This,
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
        
        DECLSPEC_XFGVIRT(IComponentType, get_Category)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Category )( 
            __RPC__in IAnalogAudioComponentType * This,
            /* [retval][out] */ __RPC__out ComponentCategory *Category);
        
        DECLSPEC_XFGVIRT(IComponentType, put_Category)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Category )( 
            __RPC__in IAnalogAudioComponentType * This,
            /* [in] */ ComponentCategory Category);
        
        DECLSPEC_XFGVIRT(IComponentType, get_MediaMajorType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MediaMajorType )( 
            __RPC__in IAnalogAudioComponentType * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *MediaMajorType);
        
        DECLSPEC_XFGVIRT(IComponentType, put_MediaMajorType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MediaMajorType )( 
            __RPC__in IAnalogAudioComponentType * This,
            /* [in] */ __RPC__in BSTR MediaMajorType);
        
        DECLSPEC_XFGVIRT(IComponentType, get__MediaMajorType)
        /* [restricted][hidden][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__MediaMajorType )( 
            __RPC__in IAnalogAudioComponentType * This,
            /* [retval][out] */ __RPC__out GUID *MediaMajorTypeGuid);
        
        DECLSPEC_XFGVIRT(IComponentType, put__MediaMajorType)
        /* [restricted][hidden][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put__MediaMajorType )( 
            __RPC__in IAnalogAudioComponentType * This,
            /* [in] */ __RPC__in REFCLSID MediaMajorTypeGuid);
        
        DECLSPEC_XFGVIRT(IComponentType, get_MediaSubType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MediaSubType )( 
            __RPC__in IAnalogAudioComponentType * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *MediaSubType);
        
        DECLSPEC_XFGVIRT(IComponentType, put_MediaSubType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MediaSubType )( 
            __RPC__in IAnalogAudioComponentType * This,
            /* [in] */ __RPC__in BSTR MediaSubType);
        
        DECLSPEC_XFGVIRT(IComponentType, get__MediaSubType)
        /* [restricted][hidden][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__MediaSubType )( 
            __RPC__in IAnalogAudioComponentType * This,
            /* [retval][out] */ __RPC__out GUID *MediaSubTypeGuid);
        
        DECLSPEC_XFGVIRT(IComponentType, put__MediaSubType)
        /* [restricted][hidden][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put__MediaSubType )( 
            __RPC__in IAnalogAudioComponentType * This,
            /* [in] */ __RPC__in REFCLSID MediaSubTypeGuid);
        
        DECLSPEC_XFGVIRT(IComponentType, get_MediaFormatType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MediaFormatType )( 
            __RPC__in IAnalogAudioComponentType * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *MediaFormatType);
        
        DECLSPEC_XFGVIRT(IComponentType, put_MediaFormatType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MediaFormatType )( 
            __RPC__in IAnalogAudioComponentType * This,
            /* [in] */ __RPC__in BSTR MediaFormatType);
        
        DECLSPEC_XFGVIRT(IComponentType, get__MediaFormatType)
        /* [restricted][hidden][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__MediaFormatType )( 
            __RPC__in IAnalogAudioComponentType * This,
            /* [retval][out] */ __RPC__out GUID *MediaFormatTypeGuid);
        
        DECLSPEC_XFGVIRT(IComponentType, put__MediaFormatType)
        /* [restricted][hidden][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put__MediaFormatType )( 
            __RPC__in IAnalogAudioComponentType * This,
            /* [in] */ __RPC__in REFCLSID MediaFormatTypeGuid);
        
        DECLSPEC_XFGVIRT(IComponentType, get_MediaType)
        /* [helpstring][restricted][hidden][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MediaType )( 
            __RPC__in IAnalogAudioComponentType * This,
            /* [retval][out] */ __RPC__out AM_MEDIA_TYPE *MediaType);
        
        DECLSPEC_XFGVIRT(IComponentType, put_MediaType)
        /* [helpstring][restricted][hidden][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MediaType )( 
            __RPC__in IAnalogAudioComponentType * This,
            /* [in] */ __RPC__in AM_MEDIA_TYPE *MediaType);
        
        DECLSPEC_XFGVIRT(IComponentType, Clone)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IAnalogAudioComponentType * This,
            /* [retval][out] */ __RPC__deref_out_opt IComponentType **NewCT);
        
        DECLSPEC_XFGVIRT(IAnalogAudioComponentType, get_AnalogAudioMode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AnalogAudioMode )( 
            __RPC__in IAnalogAudioComponentType * This,
            /* [retval][out] */ __RPC__out TVAudioMode *Mode);
        
        DECLSPEC_XFGVIRT(IAnalogAudioComponentType, put_AnalogAudioMode)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_AnalogAudioMode )( 
            __RPC__in IAnalogAudioComponentType * This,
            /* [in] */ TVAudioMode Mode);
        
        END_INTERFACE
    } IAnalogAudioComponentTypeVtbl;

    interface IAnalogAudioComponentType
    {
        CONST_VTBL struct IAnalogAudioComponentTypeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAnalogAudioComponentType_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAnalogAudioComponentType_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAnalogAudioComponentType_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAnalogAudioComponentType_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IAnalogAudioComponentType_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IAnalogAudioComponentType_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IAnalogAudioComponentType_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IAnalogAudioComponentType_get_Category(This,Category)	\
    ( (This)->lpVtbl -> get_Category(This,Category) ) 

#define IAnalogAudioComponentType_put_Category(This,Category)	\
    ( (This)->lpVtbl -> put_Category(This,Category) ) 

#define IAnalogAudioComponentType_get_MediaMajorType(This,MediaMajorType)	\
    ( (This)->lpVtbl -> get_MediaMajorType(This,MediaMajorType) ) 

#define IAnalogAudioComponentType_put_MediaMajorType(This,MediaMajorType)	\
    ( (This)->lpVtbl -> put_MediaMajorType(This,MediaMajorType) ) 

#define IAnalogAudioComponentType_get__MediaMajorType(This,MediaMajorTypeGuid)	\
    ( (This)->lpVtbl -> get__MediaMajorType(This,MediaMajorTypeGuid) ) 

#define IAnalogAudioComponentType_put__MediaMajorType(This,MediaMajorTypeGuid)	\
    ( (This)->lpVtbl -> put__MediaMajorType(This,MediaMajorTypeGuid) ) 

#define IAnalogAudioComponentType_get_MediaSubType(This,MediaSubType)	\
    ( (This)->lpVtbl -> get_MediaSubType(This,MediaSubType) ) 

#define IAnalogAudioComponentType_put_MediaSubType(This,MediaSubType)	\
    ( (This)->lpVtbl -> put_MediaSubType(This,MediaSubType) ) 

#define IAnalogAudioComponentType_get__MediaSubType(This,MediaSubTypeGuid)	\
    ( (This)->lpVtbl -> get__MediaSubType(This,MediaSubTypeGuid) ) 

#define IAnalogAudioComponentType_put__MediaSubType(This,MediaSubTypeGuid)	\
    ( (This)->lpVtbl -> put__MediaSubType(This,MediaSubTypeGuid) ) 

#define IAnalogAudioComponentType_get_MediaFormatType(This,MediaFormatType)	\
    ( (This)->lpVtbl -> get_MediaFormatType(This,MediaFormatType) ) 

#define IAnalogAudioComponentType_put_MediaFormatType(This,MediaFormatType)	\
    ( (This)->lpVtbl -> put_MediaFormatType(This,MediaFormatType) ) 

#define IAnalogAudioComponentType_get__MediaFormatType(This,MediaFormatTypeGuid)	\
    ( (This)->lpVtbl -> get__MediaFormatType(This,MediaFormatTypeGuid) ) 

#define IAnalogAudioComponentType_put__MediaFormatType(This,MediaFormatTypeGuid)	\
    ( (This)->lpVtbl -> put__MediaFormatType(This,MediaFormatTypeGuid) ) 

#define IAnalogAudioComponentType_get_MediaType(This,MediaType)	\
    ( (This)->lpVtbl -> get_MediaType(This,MediaType) ) 

#define IAnalogAudioComponentType_put_MediaType(This,MediaType)	\
    ( (This)->lpVtbl -> put_MediaType(This,MediaType) ) 

#define IAnalogAudioComponentType_Clone(This,NewCT)	\
    ( (This)->lpVtbl -> Clone(This,NewCT) ) 


#define IAnalogAudioComponentType_get_AnalogAudioMode(This,Mode)	\
    ( (This)->lpVtbl -> get_AnalogAudioMode(This,Mode) ) 

#define IAnalogAudioComponentType_put_AnalogAudioMode(This,Mode)	\
    ( (This)->lpVtbl -> put_AnalogAudioMode(This,Mode) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAnalogAudioComponentType_INTERFACE_DEFINED__ */


#ifndef __IMPEG2Component_INTERFACE_DEFINED__
#define __IMPEG2Component_INTERFACE_DEFINED__

/* interface IMPEG2Component */
/* [unique][helpstring][proxy][oleautomation][dual][uuid][nonextensible][object] */ 


EXTERN_C const IID IID_IMPEG2Component;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1493E353-1EB6-473c-802D-8E6B8EC9D2A9")
    IMPEG2Component : public IComponent
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PID( 
            /* [retval][out] */ __RPC__out long *PID) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_PID( 
            /* [in] */ long PID) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PCRPID( 
            /* [retval][out] */ __RPC__out long *PCRPID) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_PCRPID( 
            /* [in] */ long PCRPID) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ProgramNumber( 
            /* [retval][out] */ __RPC__out long *ProgramNumber) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_ProgramNumber( 
            /* [in] */ long ProgramNumber) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMPEG2ComponentVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMPEG2Component * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMPEG2Component * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMPEG2Component * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IMPEG2Component * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IMPEG2Component * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IMPEG2Component * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IMPEG2Component * This,
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
        
        DECLSPEC_XFGVIRT(IComponent, get_Type)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Type )( 
            __RPC__in IMPEG2Component * This,
            /* [retval][out] */ __RPC__deref_out_opt IComponentType **CT);
        
        DECLSPEC_XFGVIRT(IComponent, put_Type)
        /* [helpstring][id][propput][restricted][hidden] */ HRESULT ( STDMETHODCALLTYPE *put_Type )( 
            __RPC__in IMPEG2Component * This,
            /* [in] */ __RPC__in_opt IComponentType *CT);
        
        DECLSPEC_XFGVIRT(IComponent, get_DescLangID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DescLangID )( 
            __RPC__in IMPEG2Component * This,
            /* [retval][out] */ __RPC__out long *LangID);
        
        DECLSPEC_XFGVIRT(IComponent, put_DescLangID)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DescLangID )( 
            __RPC__in IMPEG2Component * This,
            /* [in] */ long LangID);
        
        DECLSPEC_XFGVIRT(IComponent, get_Status)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Status )( 
            __RPC__in IMPEG2Component * This,
            /* [retval][out] */ __RPC__out ComponentStatus *Status);
        
        DECLSPEC_XFGVIRT(IComponent, put_Status)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Status )( 
            __RPC__in IMPEG2Component * This,
            /* [in] */ ComponentStatus Status);
        
        DECLSPEC_XFGVIRT(IComponent, get_Description)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IMPEG2Component * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Description);
        
        DECLSPEC_XFGVIRT(IComponent, put_Description)
        /* [helpstring][id][propput][restricted][hidden] */ HRESULT ( STDMETHODCALLTYPE *put_Description )( 
            __RPC__in IMPEG2Component * This,
            /* [in] */ __RPC__in BSTR Description);
        
        DECLSPEC_XFGVIRT(IComponent, Clone)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IMPEG2Component * This,
            /* [retval][out] */ __RPC__deref_out_opt IComponent **NewComponent);
        
        DECLSPEC_XFGVIRT(IMPEG2Component, get_PID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PID )( 
            __RPC__in IMPEG2Component * This,
            /* [retval][out] */ __RPC__out long *PID);
        
        DECLSPEC_XFGVIRT(IMPEG2Component, put_PID)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PID )( 
            __RPC__in IMPEG2Component * This,
            /* [in] */ long PID);
        
        DECLSPEC_XFGVIRT(IMPEG2Component, get_PCRPID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PCRPID )( 
            __RPC__in IMPEG2Component * This,
            /* [retval][out] */ __RPC__out long *PCRPID);
        
        DECLSPEC_XFGVIRT(IMPEG2Component, put_PCRPID)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PCRPID )( 
            __RPC__in IMPEG2Component * This,
            /* [in] */ long PCRPID);
        
        DECLSPEC_XFGVIRT(IMPEG2Component, get_ProgramNumber)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProgramNumber )( 
            __RPC__in IMPEG2Component * This,
            /* [retval][out] */ __RPC__out long *ProgramNumber);
        
        DECLSPEC_XFGVIRT(IMPEG2Component, put_ProgramNumber)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ProgramNumber )( 
            __RPC__in IMPEG2Component * This,
            /* [in] */ long ProgramNumber);
        
        END_INTERFACE
    } IMPEG2ComponentVtbl;

    interface IMPEG2Component
    {
        CONST_VTBL struct IMPEG2ComponentVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMPEG2Component_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMPEG2Component_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMPEG2Component_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMPEG2Component_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IMPEG2Component_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IMPEG2Component_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IMPEG2Component_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IMPEG2Component_get_Type(This,CT)	\
    ( (This)->lpVtbl -> get_Type(This,CT) ) 

#define IMPEG2Component_put_Type(This,CT)	\
    ( (This)->lpVtbl -> put_Type(This,CT) ) 

#define IMPEG2Component_get_DescLangID(This,LangID)	\
    ( (This)->lpVtbl -> get_DescLangID(This,LangID) ) 

#define IMPEG2Component_put_DescLangID(This,LangID)	\
    ( (This)->lpVtbl -> put_DescLangID(This,LangID) ) 

#define IMPEG2Component_get_Status(This,Status)	\
    ( (This)->lpVtbl -> get_Status(This,Status) ) 

#define IMPEG2Component_put_Status(This,Status)	\
    ( (This)->lpVtbl -> put_Status(This,Status) ) 

#define IMPEG2Component_get_Description(This,Description)	\
    ( (This)->lpVtbl -> get_Description(This,Description) ) 

#define IMPEG2Component_put_Description(This,Description)	\
    ( (This)->lpVtbl -> put_Description(This,Description) ) 

#define IMPEG2Component_Clone(This,NewComponent)	\
    ( (This)->lpVtbl -> Clone(This,NewComponent) ) 


#define IMPEG2Component_get_PID(This,PID)	\
    ( (This)->lpVtbl -> get_PID(This,PID) ) 

#define IMPEG2Component_put_PID(This,PID)	\
    ( (This)->lpVtbl -> put_PID(This,PID) ) 

#define IMPEG2Component_get_PCRPID(This,PCRPID)	\
    ( (This)->lpVtbl -> get_PCRPID(This,PCRPID) ) 

#define IMPEG2Component_put_PCRPID(This,PCRPID)	\
    ( (This)->lpVtbl -> put_PCRPID(This,PCRPID) ) 

#define IMPEG2Component_get_ProgramNumber(This,ProgramNumber)	\
    ( (This)->lpVtbl -> get_ProgramNumber(This,ProgramNumber) ) 

#define IMPEG2Component_put_ProgramNumber(This,ProgramNumber)	\
    ( (This)->lpVtbl -> put_ProgramNumber(This,ProgramNumber) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMPEG2Component_INTERFACE_DEFINED__ */


#ifndef __IEnumComponents_INTERFACE_DEFINED__
#define __IEnumComponents_INTERFACE_DEFINED__

/* interface IEnumComponents */
/* [unique][uuid][object][restricted][hidden] */ 


EXTERN_C const IID IID_IEnumComponents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2A6E2939-2595-11d3-B64C-00C04F79498E")
    IEnumComponents : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG celt,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) IComponent **rgelt,
            /* [out] */ __RPC__out ULONG *pceltFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG celt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ __RPC__deref_out_opt IEnumComponents **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumComponentsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumComponents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumComponents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumComponents * This);
        
        DECLSPEC_XFGVIRT(IEnumComponents, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IEnumComponents * This,
            /* [in] */ ULONG celt,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(celt, *pceltFetched) IComponent **rgelt,
            /* [out] */ __RPC__out ULONG *pceltFetched);
        
        DECLSPEC_XFGVIRT(IEnumComponents, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumComponents * This,
            /* [in] */ ULONG celt);
        
        DECLSPEC_XFGVIRT(IEnumComponents, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumComponents * This);
        
        DECLSPEC_XFGVIRT(IEnumComponents, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumComponents * This,
            /* [out] */ __RPC__deref_out_opt IEnumComponents **ppEnum);
        
        END_INTERFACE
    } IEnumComponentsVtbl;

    interface IEnumComponents
    {
        CONST_VTBL struct IEnumComponentsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumComponents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumComponents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumComponents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumComponents_Next(This,celt,rgelt,pceltFetched)	\
    ( (This)->lpVtbl -> Next(This,celt,rgelt,pceltFetched) ) 

#define IEnumComponents_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IEnumComponents_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumComponents_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumComponents_INTERFACE_DEFINED__ */


#ifndef __IComponents_INTERFACE_DEFINED__
#define __IComponents_INTERFACE_DEFINED__

/* interface IComponents */
/* [unique][helpstring][proxy][oleautomation][dual][uuid][nonextensible][object] */ 


EXTERN_C const IID IID_IComponents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("39A48091-FFFE-4182-A161-3FF802640E26")
    IComponents : public IDispatch
    {
    public:
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *Count) = 0;
        
        virtual /* [restricted][hidden][id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumVARIANT **ppNewEnum) = 0;
        
        virtual /* [helpstring][restricted][hidden] */ HRESULT STDMETHODCALLTYPE EnumComponents( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumComponents **ppNewEnum) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT Index,
            /* [retval][out] */ __RPC__deref_out_opt IComponent **ppComponent) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Add( 
            /* [in] */ __RPC__in_opt IComponent *Component,
            /* [retval][out] */ __RPC__out VARIANT *NewIndex) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Remove( 
            /* [in] */ VARIANT Index) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt IComponents **NewList) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Item( 
            /* [in] */ VARIANT Index,
            /* [in] */ __RPC__in_opt IComponent *ppComponent) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IComponentsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IComponents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IComponents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IComponents * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IComponents * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IComponents * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IComponents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IComponents * This,
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
        
        DECLSPEC_XFGVIRT(IComponents, get_Count)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IComponents * This,
            /* [retval][out] */ __RPC__out long *Count);
        
        DECLSPEC_XFGVIRT(IComponents, get__NewEnum)
        /* [restricted][hidden][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IComponents * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumVARIANT **ppNewEnum);
        
        DECLSPEC_XFGVIRT(IComponents, EnumComponents)
        /* [helpstring][restricted][hidden] */ HRESULT ( STDMETHODCALLTYPE *EnumComponents )( 
            __RPC__in IComponents * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumComponents **ppNewEnum);
        
        DECLSPEC_XFGVIRT(IComponents, get_Item)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IComponents * This,
            /* [in] */ VARIANT Index,
            /* [retval][out] */ __RPC__deref_out_opt IComponent **ppComponent);
        
        DECLSPEC_XFGVIRT(IComponents, Add)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Add )( 
            __RPC__in IComponents * This,
            /* [in] */ __RPC__in_opt IComponent *Component,
            /* [retval][out] */ __RPC__out VARIANT *NewIndex);
        
        DECLSPEC_XFGVIRT(IComponents, Remove)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Remove )( 
            __RPC__in IComponents * This,
            /* [in] */ VARIANT Index);
        
        DECLSPEC_XFGVIRT(IComponents, Clone)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IComponents * This,
            /* [retval][out] */ __RPC__deref_out_opt IComponents **NewList);
        
        DECLSPEC_XFGVIRT(IComponents, put_Item)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Item )( 
            __RPC__in IComponents * This,
            /* [in] */ VARIANT Index,
            /* [in] */ __RPC__in_opt IComponent *ppComponent);
        
        END_INTERFACE
    } IComponentsVtbl;

    interface IComponents
    {
        CONST_VTBL struct IComponentsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IComponents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IComponents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IComponents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IComponents_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IComponents_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IComponents_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IComponents_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IComponents_get_Count(This,Count)	\
    ( (This)->lpVtbl -> get_Count(This,Count) ) 

#define IComponents_get__NewEnum(This,ppNewEnum)	\
    ( (This)->lpVtbl -> get__NewEnum(This,ppNewEnum) ) 

#define IComponents_EnumComponents(This,ppNewEnum)	\
    ( (This)->lpVtbl -> EnumComponents(This,ppNewEnum) ) 

#define IComponents_get_Item(This,Index,ppComponent)	\
    ( (This)->lpVtbl -> get_Item(This,Index,ppComponent) ) 

#define IComponents_Add(This,Component,NewIndex)	\
    ( (This)->lpVtbl -> Add(This,Component,NewIndex) ) 

#define IComponents_Remove(This,Index)	\
    ( (This)->lpVtbl -> Remove(This,Index) ) 

#define IComponents_Clone(This,NewList)	\
    ( (This)->lpVtbl -> Clone(This,NewList) ) 

#define IComponents_put_Item(This,Index,ppComponent)	\
    ( (This)->lpVtbl -> put_Item(This,Index,ppComponent) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IComponents_INTERFACE_DEFINED__ */


#ifndef __IComponentsOld_INTERFACE_DEFINED__
#define __IComponentsOld_INTERFACE_DEFINED__

/* interface IComponentsOld */
/* [unique][helpstring][proxy][oleautomation][dual][uuid][nonextensible][object] */ 


EXTERN_C const IID IID_IComponentsOld;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("FCD01846-0E19-11d3-9D8E-00C04F72D980")
    IComponentsOld : public IDispatch
    {
    public:
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *Count) = 0;
        
        virtual /* [restricted][hidden][id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumVARIANT **ppNewEnum) = 0;
        
        virtual /* [helpstring][restricted][hidden] */ HRESULT STDMETHODCALLTYPE EnumComponents( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumComponents **ppNewEnum) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ VARIANT Index,
            /* [retval][out] */ __RPC__deref_out_opt IComponent **ppComponent) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Add( 
            /* [in] */ __RPC__in_opt IComponent *Component,
            /* [retval][out] */ __RPC__out VARIANT *NewIndex) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Remove( 
            /* [in] */ VARIANT Index) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt IComponents **NewList) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IComponentsOldVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IComponentsOld * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IComponentsOld * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IComponentsOld * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IComponentsOld * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IComponentsOld * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IComponentsOld * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IComponentsOld * This,
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
        
        DECLSPEC_XFGVIRT(IComponentsOld, get_Count)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IComponentsOld * This,
            /* [retval][out] */ __RPC__out long *Count);
        
        DECLSPEC_XFGVIRT(IComponentsOld, get__NewEnum)
        /* [restricted][hidden][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IComponentsOld * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumVARIANT **ppNewEnum);
        
        DECLSPEC_XFGVIRT(IComponentsOld, EnumComponents)
        /* [helpstring][restricted][hidden] */ HRESULT ( STDMETHODCALLTYPE *EnumComponents )( 
            __RPC__in IComponentsOld * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumComponents **ppNewEnum);
        
        DECLSPEC_XFGVIRT(IComponentsOld, get_Item)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in IComponentsOld * This,
            /* [in] */ VARIANT Index,
            /* [retval][out] */ __RPC__deref_out_opt IComponent **ppComponent);
        
        DECLSPEC_XFGVIRT(IComponentsOld, Add)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Add )( 
            __RPC__in IComponentsOld * This,
            /* [in] */ __RPC__in_opt IComponent *Component,
            /* [retval][out] */ __RPC__out VARIANT *NewIndex);
        
        DECLSPEC_XFGVIRT(IComponentsOld, Remove)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Remove )( 
            __RPC__in IComponentsOld * This,
            /* [in] */ VARIANT Index);
        
        DECLSPEC_XFGVIRT(IComponentsOld, Clone)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IComponentsOld * This,
            /* [retval][out] */ __RPC__deref_out_opt IComponents **NewList);
        
        END_INTERFACE
    } IComponentsOldVtbl;

    interface IComponentsOld
    {
        CONST_VTBL struct IComponentsOldVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IComponentsOld_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IComponentsOld_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IComponentsOld_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IComponentsOld_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IComponentsOld_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IComponentsOld_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IComponentsOld_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IComponentsOld_get_Count(This,Count)	\
    ( (This)->lpVtbl -> get_Count(This,Count) ) 

#define IComponentsOld_get__NewEnum(This,ppNewEnum)	\
    ( (This)->lpVtbl -> get__NewEnum(This,ppNewEnum) ) 

#define IComponentsOld_EnumComponents(This,ppNewEnum)	\
    ( (This)->lpVtbl -> EnumComponents(This,ppNewEnum) ) 

#define IComponentsOld_get_Item(This,Index,ppComponent)	\
    ( (This)->lpVtbl -> get_Item(This,Index,ppComponent) ) 

#define IComponentsOld_Add(This,Component,NewIndex)	\
    ( (This)->lpVtbl -> Add(This,Component,NewIndex) ) 

#define IComponentsOld_Remove(This,Index)	\
    ( (This)->lpVtbl -> Remove(This,Index) ) 

#define IComponentsOld_Clone(This,NewList)	\
    ( (This)->lpVtbl -> Clone(This,NewList) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IComponentsOld_INTERFACE_DEFINED__ */


#ifndef __ILocator_INTERFACE_DEFINED__
#define __ILocator_INTERFACE_DEFINED__

/* interface ILocator */
/* [unique][helpstring][proxy][oleautomation][dual][uuid][nonextensible][object] */ 


EXTERN_C const IID IID_ILocator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("286D7F89-760C-4F89-80C4-66841D2507AA")
    ILocator : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CarrierFrequency( 
            /* [retval][out] */ __RPC__out long *Frequency) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_CarrierFrequency( 
            /* [in] */ long Frequency) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_InnerFEC( 
            /* [retval][out] */ __RPC__out FECMethod *FEC) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_InnerFEC( 
            /* [in] */ FECMethod FEC) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_InnerFECRate( 
            /* [retval][out] */ __RPC__out BinaryConvolutionCodeRate *FEC) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_InnerFECRate( 
            /* [in] */ BinaryConvolutionCodeRate FEC) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_OuterFEC( 
            /* [retval][out] */ __RPC__out FECMethod *FEC) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_OuterFEC( 
            /* [in] */ FECMethod FEC) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_OuterFECRate( 
            /* [retval][out] */ __RPC__out BinaryConvolutionCodeRate *FEC) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_OuterFECRate( 
            /* [in] */ BinaryConvolutionCodeRate FEC) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Modulation( 
            /* [retval][out] */ __RPC__out ModulationType *Modulation) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Modulation( 
            /* [in] */ ModulationType Modulation) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SymbolRate( 
            /* [retval][out] */ __RPC__out long *Rate) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_SymbolRate( 
            /* [in] */ long Rate) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt ILocator **NewLocator) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ILocatorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ILocator * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ILocator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ILocator * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ILocator * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ILocator * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ILocator * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ILocator * This,
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
        
        DECLSPEC_XFGVIRT(ILocator, get_CarrierFrequency)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CarrierFrequency )( 
            __RPC__in ILocator * This,
            /* [retval][out] */ __RPC__out long *Frequency);
        
        DECLSPEC_XFGVIRT(ILocator, put_CarrierFrequency)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_CarrierFrequency )( 
            __RPC__in ILocator * This,
            /* [in] */ long Frequency);
        
        DECLSPEC_XFGVIRT(ILocator, get_InnerFEC)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_InnerFEC )( 
            __RPC__in ILocator * This,
            /* [retval][out] */ __RPC__out FECMethod *FEC);
        
        DECLSPEC_XFGVIRT(ILocator, put_InnerFEC)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_InnerFEC )( 
            __RPC__in ILocator * This,
            /* [in] */ FECMethod FEC);
        
        DECLSPEC_XFGVIRT(ILocator, get_InnerFECRate)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_InnerFECRate )( 
            __RPC__in ILocator * This,
            /* [retval][out] */ __RPC__out BinaryConvolutionCodeRate *FEC);
        
        DECLSPEC_XFGVIRT(ILocator, put_InnerFECRate)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_InnerFECRate )( 
            __RPC__in ILocator * This,
            /* [in] */ BinaryConvolutionCodeRate FEC);
        
        DECLSPEC_XFGVIRT(ILocator, get_OuterFEC)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OuterFEC )( 
            __RPC__in ILocator * This,
            /* [retval][out] */ __RPC__out FECMethod *FEC);
        
        DECLSPEC_XFGVIRT(ILocator, put_OuterFEC)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_OuterFEC )( 
            __RPC__in ILocator * This,
            /* [in] */ FECMethod FEC);
        
        DECLSPEC_XFGVIRT(ILocator, get_OuterFECRate)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OuterFECRate )( 
            __RPC__in ILocator * This,
            /* [retval][out] */ __RPC__out BinaryConvolutionCodeRate *FEC);
        
        DECLSPEC_XFGVIRT(ILocator, put_OuterFECRate)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_OuterFECRate )( 
            __RPC__in ILocator * This,
            /* [in] */ BinaryConvolutionCodeRate FEC);
        
        DECLSPEC_XFGVIRT(ILocator, get_Modulation)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Modulation )( 
            __RPC__in ILocator * This,
            /* [retval][out] */ __RPC__out ModulationType *Modulation);
        
        DECLSPEC_XFGVIRT(ILocator, put_Modulation)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Modulation )( 
            __RPC__in ILocator * This,
            /* [in] */ ModulationType Modulation);
        
        DECLSPEC_XFGVIRT(ILocator, get_SymbolRate)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SymbolRate )( 
            __RPC__in ILocator * This,
            /* [retval][out] */ __RPC__out long *Rate);
        
        DECLSPEC_XFGVIRT(ILocator, put_SymbolRate)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SymbolRate )( 
            __RPC__in ILocator * This,
            /* [in] */ long Rate);
        
        DECLSPEC_XFGVIRT(ILocator, Clone)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in ILocator * This,
            /* [retval][out] */ __RPC__deref_out_opt ILocator **NewLocator);
        
        END_INTERFACE
    } ILocatorVtbl;

    interface ILocator
    {
        CONST_VTBL struct ILocatorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ILocator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ILocator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ILocator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ILocator_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ILocator_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ILocator_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ILocator_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ILocator_get_CarrierFrequency(This,Frequency)	\
    ( (This)->lpVtbl -> get_CarrierFrequency(This,Frequency) ) 

#define ILocator_put_CarrierFrequency(This,Frequency)	\
    ( (This)->lpVtbl -> put_CarrierFrequency(This,Frequency) ) 

#define ILocator_get_InnerFEC(This,FEC)	\
    ( (This)->lpVtbl -> get_InnerFEC(This,FEC) ) 

#define ILocator_put_InnerFEC(This,FEC)	\
    ( (This)->lpVtbl -> put_InnerFEC(This,FEC) ) 

#define ILocator_get_InnerFECRate(This,FEC)	\
    ( (This)->lpVtbl -> get_InnerFECRate(This,FEC) ) 

#define ILocator_put_InnerFECRate(This,FEC)	\
    ( (This)->lpVtbl -> put_InnerFECRate(This,FEC) ) 

#define ILocator_get_OuterFEC(This,FEC)	\
    ( (This)->lpVtbl -> get_OuterFEC(This,FEC) ) 

#define ILocator_put_OuterFEC(This,FEC)	\
    ( (This)->lpVtbl -> put_OuterFEC(This,FEC) ) 

#define ILocator_get_OuterFECRate(This,FEC)	\
    ( (This)->lpVtbl -> get_OuterFECRate(This,FEC) ) 

#define ILocator_put_OuterFECRate(This,FEC)	\
    ( (This)->lpVtbl -> put_OuterFECRate(This,FEC) ) 

#define ILocator_get_Modulation(This,Modulation)	\
    ( (This)->lpVtbl -> get_Modulation(This,Modulation) ) 

#define ILocator_put_Modulation(This,Modulation)	\
    ( (This)->lpVtbl -> put_Modulation(This,Modulation) ) 

#define ILocator_get_SymbolRate(This,Rate)	\
    ( (This)->lpVtbl -> get_SymbolRate(This,Rate) ) 

#define ILocator_put_SymbolRate(This,Rate)	\
    ( (This)->lpVtbl -> put_SymbolRate(This,Rate) ) 

#define ILocator_Clone(This,NewLocator)	\
    ( (This)->lpVtbl -> Clone(This,NewLocator) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ILocator_INTERFACE_DEFINED__ */


#ifndef __IAnalogLocator_INTERFACE_DEFINED__
#define __IAnalogLocator_INTERFACE_DEFINED__

/* interface IAnalogLocator */
/* [unique][helpstring][proxy][oleautomation][dual][uuid][nonextensible][object] */ 


EXTERN_C const IID IID_IAnalogLocator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("34D1F26B-E339-430D-ABCE-738CB48984DC")
    IAnalogLocator : public ILocator
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_VideoStandard( 
            /* [retval][out] */ __RPC__out AnalogVideoStandard *AVS) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_VideoStandard( 
            /* [in] */ AnalogVideoStandard AVS) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAnalogLocatorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAnalogLocator * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAnalogLocator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAnalogLocator * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IAnalogLocator * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IAnalogLocator * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IAnalogLocator * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IAnalogLocator * This,
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
        
        DECLSPEC_XFGVIRT(ILocator, get_CarrierFrequency)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CarrierFrequency )( 
            __RPC__in IAnalogLocator * This,
            /* [retval][out] */ __RPC__out long *Frequency);
        
        DECLSPEC_XFGVIRT(ILocator, put_CarrierFrequency)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_CarrierFrequency )( 
            __RPC__in IAnalogLocator * This,
            /* [in] */ long Frequency);
        
        DECLSPEC_XFGVIRT(ILocator, get_InnerFEC)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_InnerFEC )( 
            __RPC__in IAnalogLocator * This,
            /* [retval][out] */ __RPC__out FECMethod *FEC);
        
        DECLSPEC_XFGVIRT(ILocator, put_InnerFEC)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_InnerFEC )( 
            __RPC__in IAnalogLocator * This,
            /* [in] */ FECMethod FEC);
        
        DECLSPEC_XFGVIRT(ILocator, get_InnerFECRate)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_InnerFECRate )( 
            __RPC__in IAnalogLocator * This,
            /* [retval][out] */ __RPC__out BinaryConvolutionCodeRate *FEC);
        
        DECLSPEC_XFGVIRT(ILocator, put_InnerFECRate)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_InnerFECRate )( 
            __RPC__in IAnalogLocator * This,
            /* [in] */ BinaryConvolutionCodeRate FEC);
        
        DECLSPEC_XFGVIRT(ILocator, get_OuterFEC)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OuterFEC )( 
            __RPC__in IAnalogLocator * This,
            /* [retval][out] */ __RPC__out FECMethod *FEC);
        
        DECLSPEC_XFGVIRT(ILocator, put_OuterFEC)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_OuterFEC )( 
            __RPC__in IAnalogLocator * This,
            /* [in] */ FECMethod FEC);
        
        DECLSPEC_XFGVIRT(ILocator, get_OuterFECRate)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OuterFECRate )( 
            __RPC__in IAnalogLocator * This,
            /* [retval][out] */ __RPC__out BinaryConvolutionCodeRate *FEC);
        
        DECLSPEC_XFGVIRT(ILocator, put_OuterFECRate)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_OuterFECRate )( 
            __RPC__in IAnalogLocator * This,
            /* [in] */ BinaryConvolutionCodeRate FEC);
        
        DECLSPEC_XFGVIRT(ILocator, get_Modulation)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Modulation )( 
            __RPC__in IAnalogLocator * This,
            /* [retval][out] */ __RPC__out ModulationType *Modulation);
        
        DECLSPEC_XFGVIRT(ILocator, put_Modulation)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Modulation )( 
            __RPC__in IAnalogLocator * This,
            /* [in] */ ModulationType Modulation);
        
        DECLSPEC_XFGVIRT(ILocator, get_SymbolRate)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SymbolRate )( 
            __RPC__in IAnalogLocator * This,
            /* [retval][out] */ __RPC__out long *Rate);
        
        DECLSPEC_XFGVIRT(ILocator, put_SymbolRate)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SymbolRate )( 
            __RPC__in IAnalogLocator * This,
            /* [in] */ long Rate);
        
        DECLSPEC_XFGVIRT(ILocator, Clone)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IAnalogLocator * This,
            /* [retval][out] */ __RPC__deref_out_opt ILocator **NewLocator);
        
        DECLSPEC_XFGVIRT(IAnalogLocator, get_VideoStandard)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_VideoStandard )( 
            __RPC__in IAnalogLocator * This,
            /* [retval][out] */ __RPC__out AnalogVideoStandard *AVS);
        
        DECLSPEC_XFGVIRT(IAnalogLocator, put_VideoStandard)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_VideoStandard )( 
            __RPC__in IAnalogLocator * This,
            /* [in] */ AnalogVideoStandard AVS);
        
        END_INTERFACE
    } IAnalogLocatorVtbl;

    interface IAnalogLocator
    {
        CONST_VTBL struct IAnalogLocatorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAnalogLocator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAnalogLocator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAnalogLocator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAnalogLocator_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IAnalogLocator_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IAnalogLocator_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IAnalogLocator_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IAnalogLocator_get_CarrierFrequency(This,Frequency)	\
    ( (This)->lpVtbl -> get_CarrierFrequency(This,Frequency) ) 

#define IAnalogLocator_put_CarrierFrequency(This,Frequency)	\
    ( (This)->lpVtbl -> put_CarrierFrequency(This,Frequency) ) 

#define IAnalogLocator_get_InnerFEC(This,FEC)	\
    ( (This)->lpVtbl -> get_InnerFEC(This,FEC) ) 

#define IAnalogLocator_put_InnerFEC(This,FEC)	\
    ( (This)->lpVtbl -> put_InnerFEC(This,FEC) ) 

#define IAnalogLocator_get_InnerFECRate(This,FEC)	\
    ( (This)->lpVtbl -> get_InnerFECRate(This,FEC) ) 

#define IAnalogLocator_put_InnerFECRate(This,FEC)	\
    ( (This)->lpVtbl -> put_InnerFECRate(This,FEC) ) 

#define IAnalogLocator_get_OuterFEC(This,FEC)	\
    ( (This)->lpVtbl -> get_OuterFEC(This,FEC) ) 

#define IAnalogLocator_put_OuterFEC(This,FEC)	\
    ( (This)->lpVtbl -> put_OuterFEC(This,FEC) ) 

#define IAnalogLocator_get_OuterFECRate(This,FEC)	\
    ( (This)->lpVtbl -> get_OuterFECRate(This,FEC) ) 

#define IAnalogLocator_put_OuterFECRate(This,FEC)	\
    ( (This)->lpVtbl -> put_OuterFECRate(This,FEC) ) 

#define IAnalogLocator_get_Modulation(This,Modulation)	\
    ( (This)->lpVtbl -> get_Modulation(This,Modulation) ) 

#define IAnalogLocator_put_Modulation(This,Modulation)	\
    ( (This)->lpVtbl -> put_Modulation(This,Modulation) ) 

#define IAnalogLocator_get_SymbolRate(This,Rate)	\
    ( (This)->lpVtbl -> get_SymbolRate(This,Rate) ) 

#define IAnalogLocator_put_SymbolRate(This,Rate)	\
    ( (This)->lpVtbl -> put_SymbolRate(This,Rate) ) 

#define IAnalogLocator_Clone(This,NewLocator)	\
    ( (This)->lpVtbl -> Clone(This,NewLocator) ) 


#define IAnalogLocator_get_VideoStandard(This,AVS)	\
    ( (This)->lpVtbl -> get_VideoStandard(This,AVS) ) 

#define IAnalogLocator_put_VideoStandard(This,AVS)	\
    ( (This)->lpVtbl -> put_VideoStandard(This,AVS) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAnalogLocator_INTERFACE_DEFINED__ */


#ifndef __IDigitalLocator_INTERFACE_DEFINED__
#define __IDigitalLocator_INTERFACE_DEFINED__

/* interface IDigitalLocator */
/* [unique][helpstring][proxy][oleautomation][dual][uuid][nonextensible][object] */ 


EXTERN_C const IID IID_IDigitalLocator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("19B595D8-839A-47F0-96DF-4F194F3C768C")
    IDigitalLocator : public ILocator
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct IDigitalLocatorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDigitalLocator * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDigitalLocator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDigitalLocator * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IDigitalLocator * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IDigitalLocator * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IDigitalLocator * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IDigitalLocator * This,
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
        
        DECLSPEC_XFGVIRT(ILocator, get_CarrierFrequency)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CarrierFrequency )( 
            __RPC__in IDigitalLocator * This,
            /* [retval][out] */ __RPC__out long *Frequency);
        
        DECLSPEC_XFGVIRT(ILocator, put_CarrierFrequency)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_CarrierFrequency )( 
            __RPC__in IDigitalLocator * This,
            /* [in] */ long Frequency);
        
        DECLSPEC_XFGVIRT(ILocator, get_InnerFEC)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_InnerFEC )( 
            __RPC__in IDigitalLocator * This,
            /* [retval][out] */ __RPC__out FECMethod *FEC);
        
        DECLSPEC_XFGVIRT(ILocator, put_InnerFEC)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_InnerFEC )( 
            __RPC__in IDigitalLocator * This,
            /* [in] */ FECMethod FEC);
        
        DECLSPEC_XFGVIRT(ILocator, get_InnerFECRate)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_InnerFECRate )( 
            __RPC__in IDigitalLocator * This,
            /* [retval][out] */ __RPC__out BinaryConvolutionCodeRate *FEC);
        
        DECLSPEC_XFGVIRT(ILocator, put_InnerFECRate)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_InnerFECRate )( 
            __RPC__in IDigitalLocator * This,
            /* [in] */ BinaryConvolutionCodeRate FEC);
        
        DECLSPEC_XFGVIRT(ILocator, get_OuterFEC)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OuterFEC )( 
            __RPC__in IDigitalLocator * This,
            /* [retval][out] */ __RPC__out FECMethod *FEC);
        
        DECLSPEC_XFGVIRT(ILocator, put_OuterFEC)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_OuterFEC )( 
            __RPC__in IDigitalLocator * This,
            /* [in] */ FECMethod FEC);
        
        DECLSPEC_XFGVIRT(ILocator, get_OuterFECRate)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OuterFECRate )( 
            __RPC__in IDigitalLocator * This,
            /* [retval][out] */ __RPC__out BinaryConvolutionCodeRate *FEC);
        
        DECLSPEC_XFGVIRT(ILocator, put_OuterFECRate)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_OuterFECRate )( 
            __RPC__in IDigitalLocator * This,
            /* [in] */ BinaryConvolutionCodeRate FEC);
        
        DECLSPEC_XFGVIRT(ILocator, get_Modulation)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Modulation )( 
            __RPC__in IDigitalLocator * This,
            /* [retval][out] */ __RPC__out ModulationType *Modulation);
        
        DECLSPEC_XFGVIRT(ILocator, put_Modulation)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Modulation )( 
            __RPC__in IDigitalLocator * This,
            /* [in] */ ModulationType Modulation);
        
        DECLSPEC_XFGVIRT(ILocator, get_SymbolRate)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SymbolRate )( 
            __RPC__in IDigitalLocator * This,
            /* [retval][out] */ __RPC__out long *Rate);
        
        DECLSPEC_XFGVIRT(ILocator, put_SymbolRate)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SymbolRate )( 
            __RPC__in IDigitalLocator * This,
            /* [in] */ long Rate);
        
        DECLSPEC_XFGVIRT(ILocator, Clone)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IDigitalLocator * This,
            /* [retval][out] */ __RPC__deref_out_opt ILocator **NewLocator);
        
        END_INTERFACE
    } IDigitalLocatorVtbl;

    interface IDigitalLocator
    {
        CONST_VTBL struct IDigitalLocatorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDigitalLocator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDigitalLocator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDigitalLocator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDigitalLocator_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IDigitalLocator_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IDigitalLocator_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IDigitalLocator_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IDigitalLocator_get_CarrierFrequency(This,Frequency)	\
    ( (This)->lpVtbl -> get_CarrierFrequency(This,Frequency) ) 

#define IDigitalLocator_put_CarrierFrequency(This,Frequency)	\
    ( (This)->lpVtbl -> put_CarrierFrequency(This,Frequency) ) 

#define IDigitalLocator_get_InnerFEC(This,FEC)	\
    ( (This)->lpVtbl -> get_InnerFEC(This,FEC) ) 

#define IDigitalLocator_put_InnerFEC(This,FEC)	\
    ( (This)->lpVtbl -> put_InnerFEC(This,FEC) ) 

#define IDigitalLocator_get_InnerFECRate(This,FEC)	\
    ( (This)->lpVtbl -> get_InnerFECRate(This,FEC) ) 

#define IDigitalLocator_put_InnerFECRate(This,FEC)	\
    ( (This)->lpVtbl -> put_InnerFECRate(This,FEC) ) 

#define IDigitalLocator_get_OuterFEC(This,FEC)	\
    ( (This)->lpVtbl -> get_OuterFEC(This,FEC) ) 

#define IDigitalLocator_put_OuterFEC(This,FEC)	\
    ( (This)->lpVtbl -> put_OuterFEC(This,FEC) ) 

#define IDigitalLocator_get_OuterFECRate(This,FEC)	\
    ( (This)->lpVtbl -> get_OuterFECRate(This,FEC) ) 

#define IDigitalLocator_put_OuterFECRate(This,FEC)	\
    ( (This)->lpVtbl -> put_OuterFECRate(This,FEC) ) 

#define IDigitalLocator_get_Modulation(This,Modulation)	\
    ( (This)->lpVtbl -> get_Modulation(This,Modulation) ) 

#define IDigitalLocator_put_Modulation(This,Modulation)	\
    ( (This)->lpVtbl -> put_Modulation(This,Modulation) ) 

#define IDigitalLocator_get_SymbolRate(This,Rate)	\
    ( (This)->lpVtbl -> get_SymbolRate(This,Rate) ) 

#define IDigitalLocator_put_SymbolRate(This,Rate)	\
    ( (This)->lpVtbl -> put_SymbolRate(This,Rate) ) 

#define IDigitalLocator_Clone(This,NewLocator)	\
    ( (This)->lpVtbl -> Clone(This,NewLocator) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDigitalLocator_INTERFACE_DEFINED__ */


#ifndef __IATSCLocator_INTERFACE_DEFINED__
#define __IATSCLocator_INTERFACE_DEFINED__

/* interface IATSCLocator */
/* [unique][helpstring][proxy][oleautomation][dual][uuid][nonextensible][hidden][object] */ 


EXTERN_C const IID IID_IATSCLocator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BF8D986F-8C2B-4131-94D7-4D3D9FCC21EF")
    IATSCLocator : public IDigitalLocator
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PhysicalChannel( 
            /* [retval][out] */ __RPC__out long *PhysicalChannel) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_PhysicalChannel( 
            /* [in] */ long PhysicalChannel) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TSID( 
            /* [retval][out] */ __RPC__out long *TSID) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_TSID( 
            /* [in] */ long TSID) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IATSCLocatorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IATSCLocator * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IATSCLocator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IATSCLocator * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IATSCLocator * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IATSCLocator * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IATSCLocator * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IATSCLocator * This,
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
        
        DECLSPEC_XFGVIRT(ILocator, get_CarrierFrequency)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CarrierFrequency )( 
            __RPC__in IATSCLocator * This,
            /* [retval][out] */ __RPC__out long *Frequency);
        
        DECLSPEC_XFGVIRT(ILocator, put_CarrierFrequency)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_CarrierFrequency )( 
            __RPC__in IATSCLocator * This,
            /* [in] */ long Frequency);
        
        DECLSPEC_XFGVIRT(ILocator, get_InnerFEC)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_InnerFEC )( 
            __RPC__in IATSCLocator * This,
            /* [retval][out] */ __RPC__out FECMethod *FEC);
        
        DECLSPEC_XFGVIRT(ILocator, put_InnerFEC)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_InnerFEC )( 
            __RPC__in IATSCLocator * This,
            /* [in] */ FECMethod FEC);
        
        DECLSPEC_XFGVIRT(ILocator, get_InnerFECRate)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_InnerFECRate )( 
            __RPC__in IATSCLocator * This,
            /* [retval][out] */ __RPC__out BinaryConvolutionCodeRate *FEC);
        
        DECLSPEC_XFGVIRT(ILocator, put_InnerFECRate)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_InnerFECRate )( 
            __RPC__in IATSCLocator * This,
            /* [in] */ BinaryConvolutionCodeRate FEC);
        
        DECLSPEC_XFGVIRT(ILocator, get_OuterFEC)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OuterFEC )( 
            __RPC__in IATSCLocator * This,
            /* [retval][out] */ __RPC__out FECMethod *FEC);
        
        DECLSPEC_XFGVIRT(ILocator, put_OuterFEC)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_OuterFEC )( 
            __RPC__in IATSCLocator * This,
            /* [in] */ FECMethod FEC);
        
        DECLSPEC_XFGVIRT(ILocator, get_OuterFECRate)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OuterFECRate )( 
            __RPC__in IATSCLocator * This,
            /* [retval][out] */ __RPC__out BinaryConvolutionCodeRate *FEC);
        
        DECLSPEC_XFGVIRT(ILocator, put_OuterFECRate)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_OuterFECRate )( 
            __RPC__in IATSCLocator * This,
            /* [in] */ BinaryConvolutionCodeRate FEC);
        
        DECLSPEC_XFGVIRT(ILocator, get_Modulation)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Modulation )( 
            __RPC__in IATSCLocator * This,
            /* [retval][out] */ __RPC__out ModulationType *Modulation);
        
        DECLSPEC_XFGVIRT(ILocator, put_Modulation)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Modulation )( 
            __RPC__in IATSCLocator * This,
            /* [in] */ ModulationType Modulation);
        
        DECLSPEC_XFGVIRT(ILocator, get_SymbolRate)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SymbolRate )( 
            __RPC__in IATSCLocator * This,
            /* [retval][out] */ __RPC__out long *Rate);
        
        DECLSPEC_XFGVIRT(ILocator, put_SymbolRate)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SymbolRate )( 
            __RPC__in IATSCLocator * This,
            /* [in] */ long Rate);
        
        DECLSPEC_XFGVIRT(ILocator, Clone)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IATSCLocator * This,
            /* [retval][out] */ __RPC__deref_out_opt ILocator **NewLocator);
        
        DECLSPEC_XFGVIRT(IATSCLocator, get_PhysicalChannel)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PhysicalChannel )( 
            __RPC__in IATSCLocator * This,
            /* [retval][out] */ __RPC__out long *PhysicalChannel);
        
        DECLSPEC_XFGVIRT(IATSCLocator, put_PhysicalChannel)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PhysicalChannel )( 
            __RPC__in IATSCLocator * This,
            /* [in] */ long PhysicalChannel);
        
        DECLSPEC_XFGVIRT(IATSCLocator, get_TSID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TSID )( 
            __RPC__in IATSCLocator * This,
            /* [retval][out] */ __RPC__out long *TSID);
        
        DECLSPEC_XFGVIRT(IATSCLocator, put_TSID)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_TSID )( 
            __RPC__in IATSCLocator * This,
            /* [in] */ long TSID);
        
        END_INTERFACE
    } IATSCLocatorVtbl;

    interface IATSCLocator
    {
        CONST_VTBL struct IATSCLocatorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IATSCLocator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IATSCLocator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IATSCLocator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IATSCLocator_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IATSCLocator_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IATSCLocator_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IATSCLocator_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IATSCLocator_get_CarrierFrequency(This,Frequency)	\
    ( (This)->lpVtbl -> get_CarrierFrequency(This,Frequency) ) 

#define IATSCLocator_put_CarrierFrequency(This,Frequency)	\
    ( (This)->lpVtbl -> put_CarrierFrequency(This,Frequency) ) 

#define IATSCLocator_get_InnerFEC(This,FEC)	\
    ( (This)->lpVtbl -> get_InnerFEC(This,FEC) ) 

#define IATSCLocator_put_InnerFEC(This,FEC)	\
    ( (This)->lpVtbl -> put_InnerFEC(This,FEC) ) 

#define IATSCLocator_get_InnerFECRate(This,FEC)	\
    ( (This)->lpVtbl -> get_InnerFECRate(This,FEC) ) 

#define IATSCLocator_put_InnerFECRate(This,FEC)	\
    ( (This)->lpVtbl -> put_InnerFECRate(This,FEC) ) 

#define IATSCLocator_get_OuterFEC(This,FEC)	\
    ( (This)->lpVtbl -> get_OuterFEC(This,FEC) ) 

#define IATSCLocator_put_OuterFEC(This,FEC)	\
    ( (This)->lpVtbl -> put_OuterFEC(This,FEC) ) 

#define IATSCLocator_get_OuterFECRate(This,FEC)	\
    ( (This)->lpVtbl -> get_OuterFECRate(This,FEC) ) 

#define IATSCLocator_put_OuterFECRate(This,FEC)	\
    ( (This)->lpVtbl -> put_OuterFECRate(This,FEC) ) 

#define IATSCLocator_get_Modulation(This,Modulation)	\
    ( (This)->lpVtbl -> get_Modulation(This,Modulation) ) 

#define IATSCLocator_put_Modulation(This,Modulation)	\
    ( (This)->lpVtbl -> put_Modulation(This,Modulation) ) 

#define IATSCLocator_get_SymbolRate(This,Rate)	\
    ( (This)->lpVtbl -> get_SymbolRate(This,Rate) ) 

#define IATSCLocator_put_SymbolRate(This,Rate)	\
    ( (This)->lpVtbl -> put_SymbolRate(This,Rate) ) 

#define IATSCLocator_Clone(This,NewLocator)	\
    ( (This)->lpVtbl -> Clone(This,NewLocator) ) 



#define IATSCLocator_get_PhysicalChannel(This,PhysicalChannel)	\
    ( (This)->lpVtbl -> get_PhysicalChannel(This,PhysicalChannel) ) 

#define IATSCLocator_put_PhysicalChannel(This,PhysicalChannel)	\
    ( (This)->lpVtbl -> put_PhysicalChannel(This,PhysicalChannel) ) 

#define IATSCLocator_get_TSID(This,TSID)	\
    ( (This)->lpVtbl -> get_TSID(This,TSID) ) 

#define IATSCLocator_put_TSID(This,TSID)	\
    ( (This)->lpVtbl -> put_TSID(This,TSID) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IATSCLocator_INTERFACE_DEFINED__ */


#ifndef __IATSCLocator2_INTERFACE_DEFINED__
#define __IATSCLocator2_INTERFACE_DEFINED__

/* interface IATSCLocator2 */
/* [unique][helpstring][proxy][oleautomation][dual][uuid][nonextensible][hidden][object] */ 


EXTERN_C const IID IID_IATSCLocator2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("612AA885-66CF-4090-BA0A-566F5312E4CA")
    IATSCLocator2 : public IATSCLocator
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ProgramNumber( 
            /* [retval][out] */ __RPC__out long *ProgramNumber) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_ProgramNumber( 
            /* [in] */ long ProgramNumber) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IATSCLocator2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IATSCLocator2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IATSCLocator2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IATSCLocator2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IATSCLocator2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IATSCLocator2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IATSCLocator2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IATSCLocator2 * This,
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
        
        DECLSPEC_XFGVIRT(ILocator, get_CarrierFrequency)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CarrierFrequency )( 
            __RPC__in IATSCLocator2 * This,
            /* [retval][out] */ __RPC__out long *Frequency);
        
        DECLSPEC_XFGVIRT(ILocator, put_CarrierFrequency)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_CarrierFrequency )( 
            __RPC__in IATSCLocator2 * This,
            /* [in] */ long Frequency);
        
        DECLSPEC_XFGVIRT(ILocator, get_InnerFEC)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_InnerFEC )( 
            __RPC__in IATSCLocator2 * This,
            /* [retval][out] */ __RPC__out FECMethod *FEC);
        
        DECLSPEC_XFGVIRT(ILocator, put_InnerFEC)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_InnerFEC )( 
            __RPC__in IATSCLocator2 * This,
            /* [in] */ FECMethod FEC);
        
        DECLSPEC_XFGVIRT(ILocator, get_InnerFECRate)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_InnerFECRate )( 
            __RPC__in IATSCLocator2 * This,
            /* [retval][out] */ __RPC__out BinaryConvolutionCodeRate *FEC);
        
        DECLSPEC_XFGVIRT(ILocator, put_InnerFECRate)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_InnerFECRate )( 
            __RPC__in IATSCLocator2 * This,
            /* [in] */ BinaryConvolutionCodeRate FEC);
        
        DECLSPEC_XFGVIRT(ILocator, get_OuterFEC)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OuterFEC )( 
            __RPC__in IATSCLocator2 * This,
            /* [retval][out] */ __RPC__out FECMethod *FEC);
        
        DECLSPEC_XFGVIRT(ILocator, put_OuterFEC)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_OuterFEC )( 
            __RPC__in IATSCLocator2 * This,
            /* [in] */ FECMethod FEC);
        
        DECLSPEC_XFGVIRT(ILocator, get_OuterFECRate)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OuterFECRate )( 
            __RPC__in IATSCLocator2 * This,
            /* [retval][out] */ __RPC__out BinaryConvolutionCodeRate *FEC);
        
        DECLSPEC_XFGVIRT(ILocator, put_OuterFECRate)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_OuterFECRate )( 
            __RPC__in IATSCLocator2 * This,
            /* [in] */ BinaryConvolutionCodeRate FEC);
        
        DECLSPEC_XFGVIRT(ILocator, get_Modulation)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Modulation )( 
            __RPC__in IATSCLocator2 * This,
            /* [retval][out] */ __RPC__out ModulationType *Modulation);
        
        DECLSPEC_XFGVIRT(ILocator, put_Modulation)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Modulation )( 
            __RPC__in IATSCLocator2 * This,
            /* [in] */ ModulationType Modulation);
        
        DECLSPEC_XFGVIRT(ILocator, get_SymbolRate)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SymbolRate )( 
            __RPC__in IATSCLocator2 * This,
            /* [retval][out] */ __RPC__out long *Rate);
        
        DECLSPEC_XFGVIRT(ILocator, put_SymbolRate)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SymbolRate )( 
            __RPC__in IATSCLocator2 * This,
            /* [in] */ long Rate);
        
        DECLSPEC_XFGVIRT(ILocator, Clone)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IATSCLocator2 * This,
            /* [retval][out] */ __RPC__deref_out_opt ILocator **NewLocator);
        
        DECLSPEC_XFGVIRT(IATSCLocator, get_PhysicalChannel)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PhysicalChannel )( 
            __RPC__in IATSCLocator2 * This,
            /* [retval][out] */ __RPC__out long *PhysicalChannel);
        
        DECLSPEC_XFGVIRT(IATSCLocator, put_PhysicalChannel)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PhysicalChannel )( 
            __RPC__in IATSCLocator2 * This,
            /* [in] */ long PhysicalChannel);
        
        DECLSPEC_XFGVIRT(IATSCLocator, get_TSID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TSID )( 
            __RPC__in IATSCLocator2 * This,
            /* [retval][out] */ __RPC__out long *TSID);
        
        DECLSPEC_XFGVIRT(IATSCLocator, put_TSID)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_TSID )( 
            __RPC__in IATSCLocator2 * This,
            /* [in] */ long TSID);
        
        DECLSPEC_XFGVIRT(IATSCLocator2, get_ProgramNumber)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProgramNumber )( 
            __RPC__in IATSCLocator2 * This,
            /* [retval][out] */ __RPC__out long *ProgramNumber);
        
        DECLSPEC_XFGVIRT(IATSCLocator2, put_ProgramNumber)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ProgramNumber )( 
            __RPC__in IATSCLocator2 * This,
            /* [in] */ long ProgramNumber);
        
        END_INTERFACE
    } IATSCLocator2Vtbl;

    interface IATSCLocator2
    {
        CONST_VTBL struct IATSCLocator2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IATSCLocator2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IATSCLocator2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IATSCLocator2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IATSCLocator2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IATSCLocator2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IATSCLocator2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IATSCLocator2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IATSCLocator2_get_CarrierFrequency(This,Frequency)	\
    ( (This)->lpVtbl -> get_CarrierFrequency(This,Frequency) ) 

#define IATSCLocator2_put_CarrierFrequency(This,Frequency)	\
    ( (This)->lpVtbl -> put_CarrierFrequency(This,Frequency) ) 

#define IATSCLocator2_get_InnerFEC(This,FEC)	\
    ( (This)->lpVtbl -> get_InnerFEC(This,FEC) ) 

#define IATSCLocator2_put_InnerFEC(This,FEC)	\
    ( (This)->lpVtbl -> put_InnerFEC(This,FEC) ) 

#define IATSCLocator2_get_InnerFECRate(This,FEC)	\
    ( (This)->lpVtbl -> get_InnerFECRate(This,FEC) ) 

#define IATSCLocator2_put_InnerFECRate(This,FEC)	\
    ( (This)->lpVtbl -> put_InnerFECRate(This,FEC) ) 

#define IATSCLocator2_get_OuterFEC(This,FEC)	\
    ( (This)->lpVtbl -> get_OuterFEC(This,FEC) ) 

#define IATSCLocator2_put_OuterFEC(This,FEC)	\
    ( (This)->lpVtbl -> put_OuterFEC(This,FEC) ) 

#define IATSCLocator2_get_OuterFECRate(This,FEC)	\
    ( (This)->lpVtbl -> get_OuterFECRate(This,FEC) ) 

#define IATSCLocator2_put_OuterFECRate(This,FEC)	\
    ( (This)->lpVtbl -> put_OuterFECRate(This,FEC) ) 

#define IATSCLocator2_get_Modulation(This,Modulation)	\
    ( (This)->lpVtbl -> get_Modulation(This,Modulation) ) 

#define IATSCLocator2_put_Modulation(This,Modulation)	\
    ( (This)->lpVtbl -> put_Modulation(This,Modulation) ) 

#define IATSCLocator2_get_SymbolRate(This,Rate)	\
    ( (This)->lpVtbl -> get_SymbolRate(This,Rate) ) 

#define IATSCLocator2_put_SymbolRate(This,Rate)	\
    ( (This)->lpVtbl -> put_SymbolRate(This,Rate) ) 

#define IATSCLocator2_Clone(This,NewLocator)	\
    ( (This)->lpVtbl -> Clone(This,NewLocator) ) 



#define IATSCLocator2_get_PhysicalChannel(This,PhysicalChannel)	\
    ( (This)->lpVtbl -> get_PhysicalChannel(This,PhysicalChannel) ) 

#define IATSCLocator2_put_PhysicalChannel(This,PhysicalChannel)	\
    ( (This)->lpVtbl -> put_PhysicalChannel(This,PhysicalChannel) ) 

#define IATSCLocator2_get_TSID(This,TSID)	\
    ( (This)->lpVtbl -> get_TSID(This,TSID) ) 

#define IATSCLocator2_put_TSID(This,TSID)	\
    ( (This)->lpVtbl -> put_TSID(This,TSID) ) 


#define IATSCLocator2_get_ProgramNumber(This,ProgramNumber)	\
    ( (This)->lpVtbl -> get_ProgramNumber(This,ProgramNumber) ) 

#define IATSCLocator2_put_ProgramNumber(This,ProgramNumber)	\
    ( (This)->lpVtbl -> put_ProgramNumber(This,ProgramNumber) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IATSCLocator2_INTERFACE_DEFINED__ */


#ifndef __IDigitalCableLocator_INTERFACE_DEFINED__
#define __IDigitalCableLocator_INTERFACE_DEFINED__

/* interface IDigitalCableLocator */
/* [unique][helpstring][proxy][oleautomation][dual][uuid][nonextensible][hidden][object] */ 


EXTERN_C const IID IID_IDigitalCableLocator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("48F66A11-171A-419A-9525-BEEECD51584C")
    IDigitalCableLocator : public IATSCLocator2
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct IDigitalCableLocatorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDigitalCableLocator * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDigitalCableLocator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDigitalCableLocator * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IDigitalCableLocator * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IDigitalCableLocator * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IDigitalCableLocator * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IDigitalCableLocator * This,
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
        
        DECLSPEC_XFGVIRT(ILocator, get_CarrierFrequency)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CarrierFrequency )( 
            __RPC__in IDigitalCableLocator * This,
            /* [retval][out] */ __RPC__out long *Frequency);
        
        DECLSPEC_XFGVIRT(ILocator, put_CarrierFrequency)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_CarrierFrequency )( 
            __RPC__in IDigitalCableLocator * This,
            /* [in] */ long Frequency);
        
        DECLSPEC_XFGVIRT(ILocator, get_InnerFEC)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_InnerFEC )( 
            __RPC__in IDigitalCableLocator * This,
            /* [retval][out] */ __RPC__out FECMethod *FEC);
        
        DECLSPEC_XFGVIRT(ILocator, put_InnerFEC)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_InnerFEC )( 
            __RPC__in IDigitalCableLocator * This,
            /* [in] */ FECMethod FEC);
        
        DECLSPEC_XFGVIRT(ILocator, get_InnerFECRate)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_InnerFECRate )( 
            __RPC__in IDigitalCableLocator * This,
            /* [retval][out] */ __RPC__out BinaryConvolutionCodeRate *FEC);
        
        DECLSPEC_XFGVIRT(ILocator, put_InnerFECRate)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_InnerFECRate )( 
            __RPC__in IDigitalCableLocator * This,
            /* [in] */ BinaryConvolutionCodeRate FEC);
        
        DECLSPEC_XFGVIRT(ILocator, get_OuterFEC)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OuterFEC )( 
            __RPC__in IDigitalCableLocator * This,
            /* [retval][out] */ __RPC__out FECMethod *FEC);
        
        DECLSPEC_XFGVIRT(ILocator, put_OuterFEC)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_OuterFEC )( 
            __RPC__in IDigitalCableLocator * This,
            /* [in] */ FECMethod FEC);
        
        DECLSPEC_XFGVIRT(ILocator, get_OuterFECRate)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OuterFECRate )( 
            __RPC__in IDigitalCableLocator * This,
            /* [retval][out] */ __RPC__out BinaryConvolutionCodeRate *FEC);
        
        DECLSPEC_XFGVIRT(ILocator, put_OuterFECRate)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_OuterFECRate )( 
            __RPC__in IDigitalCableLocator * This,
            /* [in] */ BinaryConvolutionCodeRate FEC);
        
        DECLSPEC_XFGVIRT(ILocator, get_Modulation)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Modulation )( 
            __RPC__in IDigitalCableLocator * This,
            /* [retval][out] */ __RPC__out ModulationType *Modulation);
        
        DECLSPEC_XFGVIRT(ILocator, put_Modulation)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Modulation )( 
            __RPC__in IDigitalCableLocator * This,
            /* [in] */ ModulationType Modulation);
        
        DECLSPEC_XFGVIRT(ILocator, get_SymbolRate)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SymbolRate )( 
            __RPC__in IDigitalCableLocator * This,
            /* [retval][out] */ __RPC__out long *Rate);
        
        DECLSPEC_XFGVIRT(ILocator, put_SymbolRate)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SymbolRate )( 
            __RPC__in IDigitalCableLocator * This,
            /* [in] */ long Rate);
        
        DECLSPEC_XFGVIRT(ILocator, Clone)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IDigitalCableLocator * This,
            /* [retval][out] */ __RPC__deref_out_opt ILocator **NewLocator);
        
        DECLSPEC_XFGVIRT(IATSCLocator, get_PhysicalChannel)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PhysicalChannel )( 
            __RPC__in IDigitalCableLocator * This,
            /* [retval][out] */ __RPC__out long *PhysicalChannel);
        
        DECLSPEC_XFGVIRT(IATSCLocator, put_PhysicalChannel)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PhysicalChannel )( 
            __RPC__in IDigitalCableLocator * This,
            /* [in] */ long PhysicalChannel);
        
        DECLSPEC_XFGVIRT(IATSCLocator, get_TSID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TSID )( 
            __RPC__in IDigitalCableLocator * This,
            /* [retval][out] */ __RPC__out long *TSID);
        
        DECLSPEC_XFGVIRT(IATSCLocator, put_TSID)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_TSID )( 
            __RPC__in IDigitalCableLocator * This,
            /* [in] */ long TSID);
        
        DECLSPEC_XFGVIRT(IATSCLocator2, get_ProgramNumber)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProgramNumber )( 
            __RPC__in IDigitalCableLocator * This,
            /* [retval][out] */ __RPC__out long *ProgramNumber);
        
        DECLSPEC_XFGVIRT(IATSCLocator2, put_ProgramNumber)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ProgramNumber )( 
            __RPC__in IDigitalCableLocator * This,
            /* [in] */ long ProgramNumber);
        
        END_INTERFACE
    } IDigitalCableLocatorVtbl;

    interface IDigitalCableLocator
    {
        CONST_VTBL struct IDigitalCableLocatorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDigitalCableLocator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDigitalCableLocator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDigitalCableLocator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDigitalCableLocator_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IDigitalCableLocator_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IDigitalCableLocator_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IDigitalCableLocator_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IDigitalCableLocator_get_CarrierFrequency(This,Frequency)	\
    ( (This)->lpVtbl -> get_CarrierFrequency(This,Frequency) ) 

#define IDigitalCableLocator_put_CarrierFrequency(This,Frequency)	\
    ( (This)->lpVtbl -> put_CarrierFrequency(This,Frequency) ) 

#define IDigitalCableLocator_get_InnerFEC(This,FEC)	\
    ( (This)->lpVtbl -> get_InnerFEC(This,FEC) ) 

#define IDigitalCableLocator_put_InnerFEC(This,FEC)	\
    ( (This)->lpVtbl -> put_InnerFEC(This,FEC) ) 

#define IDigitalCableLocator_get_InnerFECRate(This,FEC)	\
    ( (This)->lpVtbl -> get_InnerFECRate(This,FEC) ) 

#define IDigitalCableLocator_put_InnerFECRate(This,FEC)	\
    ( (This)->lpVtbl -> put_InnerFECRate(This,FEC) ) 

#define IDigitalCableLocator_get_OuterFEC(This,FEC)	\
    ( (This)->lpVtbl -> get_OuterFEC(This,FEC) ) 

#define IDigitalCableLocator_put_OuterFEC(This,FEC)	\
    ( (This)->lpVtbl -> put_OuterFEC(This,FEC) ) 

#define IDigitalCableLocator_get_OuterFECRate(This,FEC)	\
    ( (This)->lpVtbl -> get_OuterFECRate(This,FEC) ) 

#define IDigitalCableLocator_put_OuterFECRate(This,FEC)	\
    ( (This)->lpVtbl -> put_OuterFECRate(This,FEC) ) 

#define IDigitalCableLocator_get_Modulation(This,Modulation)	\
    ( (This)->lpVtbl -> get_Modulation(This,Modulation) ) 

#define IDigitalCableLocator_put_Modulation(This,Modulation)	\
    ( (This)->lpVtbl -> put_Modulation(This,Modulation) ) 

#define IDigitalCableLocator_get_SymbolRate(This,Rate)	\
    ( (This)->lpVtbl -> get_SymbolRate(This,Rate) ) 

#define IDigitalCableLocator_put_SymbolRate(This,Rate)	\
    ( (This)->lpVtbl -> put_SymbolRate(This,Rate) ) 

#define IDigitalCableLocator_Clone(This,NewLocator)	\
    ( (This)->lpVtbl -> Clone(This,NewLocator) ) 



#define IDigitalCableLocator_get_PhysicalChannel(This,PhysicalChannel)	\
    ( (This)->lpVtbl -> get_PhysicalChannel(This,PhysicalChannel) ) 

#define IDigitalCableLocator_put_PhysicalChannel(This,PhysicalChannel)	\
    ( (This)->lpVtbl -> put_PhysicalChannel(This,PhysicalChannel) ) 

#define IDigitalCableLocator_get_TSID(This,TSID)	\
    ( (This)->lpVtbl -> get_TSID(This,TSID) ) 

#define IDigitalCableLocator_put_TSID(This,TSID)	\
    ( (This)->lpVtbl -> put_TSID(This,TSID) ) 


#define IDigitalCableLocator_get_ProgramNumber(This,ProgramNumber)	\
    ( (This)->lpVtbl -> get_ProgramNumber(This,ProgramNumber) ) 

#define IDigitalCableLocator_put_ProgramNumber(This,ProgramNumber)	\
    ( (This)->lpVtbl -> put_ProgramNumber(This,ProgramNumber) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDigitalCableLocator_INTERFACE_DEFINED__ */


#ifndef __IDVBTLocator_INTERFACE_DEFINED__
#define __IDVBTLocator_INTERFACE_DEFINED__

/* interface IDVBTLocator */
/* [unique][helpstring][proxy][oleautomation][dual][uuid][nonextensible][hidden][object] */ 


EXTERN_C const IID IID_IDVBTLocator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8664DA16-DDA2-42ac-926A-C18F9127C302")
    IDVBTLocator : public IDigitalLocator
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Bandwidth( 
            /* [retval][out] */ __RPC__out long *BandWidthVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Bandwidth( 
            /* [in] */ long BandwidthVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_LPInnerFEC( 
            /* [retval][out] */ __RPC__out FECMethod *FEC) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_LPInnerFEC( 
            /* [in] */ FECMethod FEC) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_LPInnerFECRate( 
            /* [retval][out] */ __RPC__out BinaryConvolutionCodeRate *FEC) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_LPInnerFECRate( 
            /* [in] */ BinaryConvolutionCodeRate FEC) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_HAlpha( 
            /* [retval][out] */ __RPC__out HierarchyAlpha *Alpha) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_HAlpha( 
            /* [in] */ HierarchyAlpha Alpha) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Guard( 
            /* [retval][out] */ __RPC__out GuardInterval *GI) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Guard( 
            /* [in] */ GuardInterval GI) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Mode( 
            /* [retval][out] */ __RPC__out TransmissionMode *mode) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Mode( 
            /* [in] */ TransmissionMode mode) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_OtherFrequencyInUse( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *OtherFrequencyInUseVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_OtherFrequencyInUse( 
            /* [in] */ VARIANT_BOOL OtherFrequencyInUseVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDVBTLocatorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDVBTLocator * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDVBTLocator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDVBTLocator * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IDVBTLocator * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IDVBTLocator * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IDVBTLocator * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IDVBTLocator * This,
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
        
        DECLSPEC_XFGVIRT(ILocator, get_CarrierFrequency)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CarrierFrequency )( 
            __RPC__in IDVBTLocator * This,
            /* [retval][out] */ __RPC__out long *Frequency);
        
        DECLSPEC_XFGVIRT(ILocator, put_CarrierFrequency)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_CarrierFrequency )( 
            __RPC__in IDVBTLocator * This,
            /* [in] */ long Frequency);
        
        DECLSPEC_XFGVIRT(ILocator, get_InnerFEC)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_InnerFEC )( 
            __RPC__in IDVBTLocator * This,
            /* [retval][out] */ __RPC__out FECMethod *FEC);
        
        DECLSPEC_XFGVIRT(ILocator, put_InnerFEC)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_InnerFEC )( 
            __RPC__in IDVBTLocator * This,
            /* [in] */ FECMethod FEC);
        
        DECLSPEC_XFGVIRT(ILocator, get_InnerFECRate)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_InnerFECRate )( 
            __RPC__in IDVBTLocator * This,
            /* [retval][out] */ __RPC__out BinaryConvolutionCodeRate *FEC);
        
        DECLSPEC_XFGVIRT(ILocator, put_InnerFECRate)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_InnerFECRate )( 
            __RPC__in IDVBTLocator * This,
            /* [in] */ BinaryConvolutionCodeRate FEC);
        
        DECLSPEC_XFGVIRT(ILocator, get_OuterFEC)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OuterFEC )( 
            __RPC__in IDVBTLocator * This,
            /* [retval][out] */ __RPC__out FECMethod *FEC);
        
        DECLSPEC_XFGVIRT(ILocator, put_OuterFEC)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_OuterFEC )( 
            __RPC__in IDVBTLocator * This,
            /* [in] */ FECMethod FEC);
        
        DECLSPEC_XFGVIRT(ILocator, get_OuterFECRate)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OuterFECRate )( 
            __RPC__in IDVBTLocator * This,
            /* [retval][out] */ __RPC__out BinaryConvolutionCodeRate *FEC);
        
        DECLSPEC_XFGVIRT(ILocator, put_OuterFECRate)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_OuterFECRate )( 
            __RPC__in IDVBTLocator * This,
            /* [in] */ BinaryConvolutionCodeRate FEC);
        
        DECLSPEC_XFGVIRT(ILocator, get_Modulation)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Modulation )( 
            __RPC__in IDVBTLocator * This,
            /* [retval][out] */ __RPC__out ModulationType *Modulation);
        
        DECLSPEC_XFGVIRT(ILocator, put_Modulation)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Modulation )( 
            __RPC__in IDVBTLocator * This,
            /* [in] */ ModulationType Modulation);
        
        DECLSPEC_XFGVIRT(ILocator, get_SymbolRate)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SymbolRate )( 
            __RPC__in IDVBTLocator * This,
            /* [retval][out] */ __RPC__out long *Rate);
        
        DECLSPEC_XFGVIRT(ILocator, put_SymbolRate)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SymbolRate )( 
            __RPC__in IDVBTLocator * This,
            /* [in] */ long Rate);
        
        DECLSPEC_XFGVIRT(ILocator, Clone)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IDVBTLocator * This,
            /* [retval][out] */ __RPC__deref_out_opt ILocator **NewLocator);
        
        DECLSPEC_XFGVIRT(IDVBTLocator, get_Bandwidth)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Bandwidth )( 
            __RPC__in IDVBTLocator * This,
            /* [retval][out] */ __RPC__out long *BandWidthVal);
        
        DECLSPEC_XFGVIRT(IDVBTLocator, put_Bandwidth)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Bandwidth )( 
            __RPC__in IDVBTLocator * This,
            /* [in] */ long BandwidthVal);
        
        DECLSPEC_XFGVIRT(IDVBTLocator, get_LPInnerFEC)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LPInnerFEC )( 
            __RPC__in IDVBTLocator * This,
            /* [retval][out] */ __RPC__out FECMethod *FEC);
        
        DECLSPEC_XFGVIRT(IDVBTLocator, put_LPInnerFEC)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_LPInnerFEC )( 
            __RPC__in IDVBTLocator * This,
            /* [in] */ FECMethod FEC);
        
        DECLSPEC_XFGVIRT(IDVBTLocator, get_LPInnerFECRate)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LPInnerFECRate )( 
            __RPC__in IDVBTLocator * This,
            /* [retval][out] */ __RPC__out BinaryConvolutionCodeRate *FEC);
        
        DECLSPEC_XFGVIRT(IDVBTLocator, put_LPInnerFECRate)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_LPInnerFECRate )( 
            __RPC__in IDVBTLocator * This,
            /* [in] */ BinaryConvolutionCodeRate FEC);
        
        DECLSPEC_XFGVIRT(IDVBTLocator, get_HAlpha)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_HAlpha )( 
            __RPC__in IDVBTLocator * This,
            /* [retval][out] */ __RPC__out HierarchyAlpha *Alpha);
        
        DECLSPEC_XFGVIRT(IDVBTLocator, put_HAlpha)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_HAlpha )( 
            __RPC__in IDVBTLocator * This,
            /* [in] */ HierarchyAlpha Alpha);
        
        DECLSPEC_XFGVIRT(IDVBTLocator, get_Guard)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Guard )( 
            __RPC__in IDVBTLocator * This,
            /* [retval][out] */ __RPC__out GuardInterval *GI);
        
        DECLSPEC_XFGVIRT(IDVBTLocator, put_Guard)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Guard )( 
            __RPC__in IDVBTLocator * This,
            /* [in] */ GuardInterval GI);
        
        DECLSPEC_XFGVIRT(IDVBTLocator, get_Mode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Mode )( 
            __RPC__in IDVBTLocator * This,
            /* [retval][out] */ __RPC__out TransmissionMode *mode);
        
        DECLSPEC_XFGVIRT(IDVBTLocator, put_Mode)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Mode )( 
            __RPC__in IDVBTLocator * This,
            /* [in] */ TransmissionMode mode);
        
        DECLSPEC_XFGVIRT(IDVBTLocator, get_OtherFrequencyInUse)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OtherFrequencyInUse )( 
            __RPC__in IDVBTLocator * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *OtherFrequencyInUseVal);
        
        DECLSPEC_XFGVIRT(IDVBTLocator, put_OtherFrequencyInUse)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_OtherFrequencyInUse )( 
            __RPC__in IDVBTLocator * This,
            /* [in] */ VARIANT_BOOL OtherFrequencyInUseVal);
        
        END_INTERFACE
    } IDVBTLocatorVtbl;

    interface IDVBTLocator
    {
        CONST_VTBL struct IDVBTLocatorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDVBTLocator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDVBTLocator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDVBTLocator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDVBTLocator_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IDVBTLocator_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IDVBTLocator_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IDVBTLocator_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IDVBTLocator_get_CarrierFrequency(This,Frequency)	\
    ( (This)->lpVtbl -> get_CarrierFrequency(This,Frequency) ) 

#define IDVBTLocator_put_CarrierFrequency(This,Frequency)	\
    ( (This)->lpVtbl -> put_CarrierFrequency(This,Frequency) ) 

#define IDVBTLocator_get_InnerFEC(This,FEC)	\
    ( (This)->lpVtbl -> get_InnerFEC(This,FEC) ) 

#define IDVBTLocator_put_InnerFEC(This,FEC)	\
    ( (This)->lpVtbl -> put_InnerFEC(This,FEC) ) 

#define IDVBTLocator_get_InnerFECRate(This,FEC)	\
    ( (This)->lpVtbl -> get_InnerFECRate(This,FEC) ) 

#define IDVBTLocator_put_InnerFECRate(This,FEC)	\
    ( (This)->lpVtbl -> put_InnerFECRate(This,FEC) ) 

#define IDVBTLocator_get_OuterFEC(This,FEC)	\
    ( (This)->lpVtbl -> get_OuterFEC(This,FEC) ) 

#define IDVBTLocator_put_OuterFEC(This,FEC)	\
    ( (This)->lpVtbl -> put_OuterFEC(This,FEC) ) 

#define IDVBTLocator_get_OuterFECRate(This,FEC)	\
    ( (This)->lpVtbl -> get_OuterFECRate(This,FEC) ) 

#define IDVBTLocator_put_OuterFECRate(This,FEC)	\
    ( (This)->lpVtbl -> put_OuterFECRate(This,FEC) ) 

#define IDVBTLocator_get_Modulation(This,Modulation)	\
    ( (This)->lpVtbl -> get_Modulation(This,Modulation) ) 

#define IDVBTLocator_put_Modulation(This,Modulation)	\
    ( (This)->lpVtbl -> put_Modulation(This,Modulation) ) 

#define IDVBTLocator_get_SymbolRate(This,Rate)	\
    ( (This)->lpVtbl -> get_SymbolRate(This,Rate) ) 

#define IDVBTLocator_put_SymbolRate(This,Rate)	\
    ( (This)->lpVtbl -> put_SymbolRate(This,Rate) ) 

#define IDVBTLocator_Clone(This,NewLocator)	\
    ( (This)->lpVtbl -> Clone(This,NewLocator) ) 



#define IDVBTLocator_get_Bandwidth(This,BandWidthVal)	\
    ( (This)->lpVtbl -> get_Bandwidth(This,BandWidthVal) ) 

#define IDVBTLocator_put_Bandwidth(This,BandwidthVal)	\
    ( (This)->lpVtbl -> put_Bandwidth(This,BandwidthVal) ) 

#define IDVBTLocator_get_LPInnerFEC(This,FEC)	\
    ( (This)->lpVtbl -> get_LPInnerFEC(This,FEC) ) 

#define IDVBTLocator_put_LPInnerFEC(This,FEC)	\
    ( (This)->lpVtbl -> put_LPInnerFEC(This,FEC) ) 

#define IDVBTLocator_get_LPInnerFECRate(This,FEC)	\
    ( (This)->lpVtbl -> get_LPInnerFECRate(This,FEC) ) 

#define IDVBTLocator_put_LPInnerFECRate(This,FEC)	\
    ( (This)->lpVtbl -> put_LPInnerFECRate(This,FEC) ) 

#define IDVBTLocator_get_HAlpha(This,Alpha)	\
    ( (This)->lpVtbl -> get_HAlpha(This,Alpha) ) 

#define IDVBTLocator_put_HAlpha(This,Alpha)	\
    ( (This)->lpVtbl -> put_HAlpha(This,Alpha) ) 

#define IDVBTLocator_get_Guard(This,GI)	\
    ( (This)->lpVtbl -> get_Guard(This,GI) ) 

#define IDVBTLocator_put_Guard(This,GI)	\
    ( (This)->lpVtbl -> put_Guard(This,GI) ) 

#define IDVBTLocator_get_Mode(This,mode)	\
    ( (This)->lpVtbl -> get_Mode(This,mode) ) 

#define IDVBTLocator_put_Mode(This,mode)	\
    ( (This)->lpVtbl -> put_Mode(This,mode) ) 

#define IDVBTLocator_get_OtherFrequencyInUse(This,OtherFrequencyInUseVal)	\
    ( (This)->lpVtbl -> get_OtherFrequencyInUse(This,OtherFrequencyInUseVal) ) 

#define IDVBTLocator_put_OtherFrequencyInUse(This,OtherFrequencyInUseVal)	\
    ( (This)->lpVtbl -> put_OtherFrequencyInUse(This,OtherFrequencyInUseVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDVBTLocator_INTERFACE_DEFINED__ */


#ifndef __IDVBTLocator2_INTERFACE_DEFINED__
#define __IDVBTLocator2_INTERFACE_DEFINED__

/* interface IDVBTLocator2 */
/* [unique][helpstring][proxy][oleautomation][dual][uuid][nonextensible][hidden][object] */ 


EXTERN_C const IID IID_IDVBTLocator2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("448A2EDF-AE95-4b43-A3CC-747843C453D4")
    IDVBTLocator2 : public IDVBTLocator
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PhysicalLayerPipeId( 
            /* [retval][out] */ __RPC__out long *PhysicalLayerPipeIdVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_PhysicalLayerPipeId( 
            /* [in] */ long PhysicalLayerPipeIdVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDVBTLocator2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDVBTLocator2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDVBTLocator2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDVBTLocator2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IDVBTLocator2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IDVBTLocator2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IDVBTLocator2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IDVBTLocator2 * This,
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
        
        DECLSPEC_XFGVIRT(ILocator, get_CarrierFrequency)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CarrierFrequency )( 
            __RPC__in IDVBTLocator2 * This,
            /* [retval][out] */ __RPC__out long *Frequency);
        
        DECLSPEC_XFGVIRT(ILocator, put_CarrierFrequency)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_CarrierFrequency )( 
            __RPC__in IDVBTLocator2 * This,
            /* [in] */ long Frequency);
        
        DECLSPEC_XFGVIRT(ILocator, get_InnerFEC)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_InnerFEC )( 
            __RPC__in IDVBTLocator2 * This,
            /* [retval][out] */ __RPC__out FECMethod *FEC);
        
        DECLSPEC_XFGVIRT(ILocator, put_InnerFEC)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_InnerFEC )( 
            __RPC__in IDVBTLocator2 * This,
            /* [in] */ FECMethod FEC);
        
        DECLSPEC_XFGVIRT(ILocator, get_InnerFECRate)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_InnerFECRate )( 
            __RPC__in IDVBTLocator2 * This,
            /* [retval][out] */ __RPC__out BinaryConvolutionCodeRate *FEC);
        
        DECLSPEC_XFGVIRT(ILocator, put_InnerFECRate)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_InnerFECRate )( 
            __RPC__in IDVBTLocator2 * This,
            /* [in] */ BinaryConvolutionCodeRate FEC);
        
        DECLSPEC_XFGVIRT(ILocator, get_OuterFEC)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OuterFEC )( 
            __RPC__in IDVBTLocator2 * This,
            /* [retval][out] */ __RPC__out FECMethod *FEC);
        
        DECLSPEC_XFGVIRT(ILocator, put_OuterFEC)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_OuterFEC )( 
            __RPC__in IDVBTLocator2 * This,
            /* [in] */ FECMethod FEC);
        
        DECLSPEC_XFGVIRT(ILocator, get_OuterFECRate)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OuterFECRate )( 
            __RPC__in IDVBTLocator2 * This,
            /* [retval][out] */ __RPC__out BinaryConvolutionCodeRate *FEC);
        
        DECLSPEC_XFGVIRT(ILocator, put_OuterFECRate)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_OuterFECRate )( 
            __RPC__in IDVBTLocator2 * This,
            /* [in] */ BinaryConvolutionCodeRate FEC);
        
        DECLSPEC_XFGVIRT(ILocator, get_Modulation)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Modulation )( 
            __RPC__in IDVBTLocator2 * This,
            /* [retval][out] */ __RPC__out ModulationType *Modulation);
        
        DECLSPEC_XFGVIRT(ILocator, put_Modulation)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Modulation )( 
            __RPC__in IDVBTLocator2 * This,
            /* [in] */ ModulationType Modulation);
        
        DECLSPEC_XFGVIRT(ILocator, get_SymbolRate)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SymbolRate )( 
            __RPC__in IDVBTLocator2 * This,
            /* [retval][out] */ __RPC__out long *Rate);
        
        DECLSPEC_XFGVIRT(ILocator, put_SymbolRate)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SymbolRate )( 
            __RPC__in IDVBTLocator2 * This,
            /* [in] */ long Rate);
        
        DECLSPEC_XFGVIRT(ILocator, Clone)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IDVBTLocator2 * This,
            /* [retval][out] */ __RPC__deref_out_opt ILocator **NewLocator);
        
        DECLSPEC_XFGVIRT(IDVBTLocator, get_Bandwidth)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Bandwidth )( 
            __RPC__in IDVBTLocator2 * This,
            /* [retval][out] */ __RPC__out long *BandWidthVal);
        
        DECLSPEC_XFGVIRT(IDVBTLocator, put_Bandwidth)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Bandwidth )( 
            __RPC__in IDVBTLocator2 * This,
            /* [in] */ long BandwidthVal);
        
        DECLSPEC_XFGVIRT(IDVBTLocator, get_LPInnerFEC)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LPInnerFEC )( 
            __RPC__in IDVBTLocator2 * This,
            /* [retval][out] */ __RPC__out FECMethod *FEC);
        
        DECLSPEC_XFGVIRT(IDVBTLocator, put_LPInnerFEC)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_LPInnerFEC )( 
            __RPC__in IDVBTLocator2 * This,
            /* [in] */ FECMethod FEC);
        
        DECLSPEC_XFGVIRT(IDVBTLocator, get_LPInnerFECRate)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LPInnerFECRate )( 
            __RPC__in IDVBTLocator2 * This,
            /* [retval][out] */ __RPC__out BinaryConvolutionCodeRate *FEC);
        
        DECLSPEC_XFGVIRT(IDVBTLocator, put_LPInnerFECRate)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_LPInnerFECRate )( 
            __RPC__in IDVBTLocator2 * This,
            /* [in] */ BinaryConvolutionCodeRate FEC);
        
        DECLSPEC_XFGVIRT(IDVBTLocator, get_HAlpha)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_HAlpha )( 
            __RPC__in IDVBTLocator2 * This,
            /* [retval][out] */ __RPC__out HierarchyAlpha *Alpha);
        
        DECLSPEC_XFGVIRT(IDVBTLocator, put_HAlpha)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_HAlpha )( 
            __RPC__in IDVBTLocator2 * This,
            /* [in] */ HierarchyAlpha Alpha);
        
        DECLSPEC_XFGVIRT(IDVBTLocator, get_Guard)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Guard )( 
            __RPC__in IDVBTLocator2 * This,
            /* [retval][out] */ __RPC__out GuardInterval *GI);
        
        DECLSPEC_XFGVIRT(IDVBTLocator, put_Guard)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Guard )( 
            __RPC__in IDVBTLocator2 * This,
            /* [in] */ GuardInterval GI);
        
        DECLSPEC_XFGVIRT(IDVBTLocator, get_Mode)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Mode )( 
            __RPC__in IDVBTLocator2 * This,
            /* [retval][out] */ __RPC__out TransmissionMode *mode);
        
        DECLSPEC_XFGVIRT(IDVBTLocator, put_Mode)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Mode )( 
            __RPC__in IDVBTLocator2 * This,
            /* [in] */ TransmissionMode mode);
        
        DECLSPEC_XFGVIRT(IDVBTLocator, get_OtherFrequencyInUse)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OtherFrequencyInUse )( 
            __RPC__in IDVBTLocator2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *OtherFrequencyInUseVal);
        
        DECLSPEC_XFGVIRT(IDVBTLocator, put_OtherFrequencyInUse)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_OtherFrequencyInUse )( 
            __RPC__in IDVBTLocator2 * This,
            /* [in] */ VARIANT_BOOL OtherFrequencyInUseVal);
        
        DECLSPEC_XFGVIRT(IDVBTLocator2, get_PhysicalLayerPipeId)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PhysicalLayerPipeId )( 
            __RPC__in IDVBTLocator2 * This,
            /* [retval][out] */ __RPC__out long *PhysicalLayerPipeIdVal);
        
        DECLSPEC_XFGVIRT(IDVBTLocator2, put_PhysicalLayerPipeId)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PhysicalLayerPipeId )( 
            __RPC__in IDVBTLocator2 * This,
            /* [in] */ long PhysicalLayerPipeIdVal);
        
        END_INTERFACE
    } IDVBTLocator2Vtbl;

    interface IDVBTLocator2
    {
        CONST_VTBL struct IDVBTLocator2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDVBTLocator2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDVBTLocator2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDVBTLocator2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDVBTLocator2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IDVBTLocator2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IDVBTLocator2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IDVBTLocator2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IDVBTLocator2_get_CarrierFrequency(This,Frequency)	\
    ( (This)->lpVtbl -> get_CarrierFrequency(This,Frequency) ) 

#define IDVBTLocator2_put_CarrierFrequency(This,Frequency)	\
    ( (This)->lpVtbl -> put_CarrierFrequency(This,Frequency) ) 

#define IDVBTLocator2_get_InnerFEC(This,FEC)	\
    ( (This)->lpVtbl -> get_InnerFEC(This,FEC) ) 

#define IDVBTLocator2_put_InnerFEC(This,FEC)	\
    ( (This)->lpVtbl -> put_InnerFEC(This,FEC) ) 

#define IDVBTLocator2_get_InnerFECRate(This,FEC)	\
    ( (This)->lpVtbl -> get_InnerFECRate(This,FEC) ) 

#define IDVBTLocator2_put_InnerFECRate(This,FEC)	\
    ( (This)->lpVtbl -> put_InnerFECRate(This,FEC) ) 

#define IDVBTLocator2_get_OuterFEC(This,FEC)	\
    ( (This)->lpVtbl -> get_OuterFEC(This,FEC) ) 

#define IDVBTLocator2_put_OuterFEC(This,FEC)	\
    ( (This)->lpVtbl -> put_OuterFEC(This,FEC) ) 

#define IDVBTLocator2_get_OuterFECRate(This,FEC)	\
    ( (This)->lpVtbl -> get_OuterFECRate(This,FEC) ) 

#define IDVBTLocator2_put_OuterFECRate(This,FEC)	\
    ( (This)->lpVtbl -> put_OuterFECRate(This,FEC) ) 

#define IDVBTLocator2_get_Modulation(This,Modulation)	\
    ( (This)->lpVtbl -> get_Modulation(This,Modulation) ) 

#define IDVBTLocator2_put_Modulation(This,Modulation)	\
    ( (This)->lpVtbl -> put_Modulation(This,Modulation) ) 

#define IDVBTLocator2_get_SymbolRate(This,Rate)	\
    ( (This)->lpVtbl -> get_SymbolRate(This,Rate) ) 

#define IDVBTLocator2_put_SymbolRate(This,Rate)	\
    ( (This)->lpVtbl -> put_SymbolRate(This,Rate) ) 

#define IDVBTLocator2_Clone(This,NewLocator)	\
    ( (This)->lpVtbl -> Clone(This,NewLocator) ) 



#define IDVBTLocator2_get_Bandwidth(This,BandWidthVal)	\
    ( (This)->lpVtbl -> get_Bandwidth(This,BandWidthVal) ) 

#define IDVBTLocator2_put_Bandwidth(This,BandwidthVal)	\
    ( (This)->lpVtbl -> put_Bandwidth(This,BandwidthVal) ) 

#define IDVBTLocator2_get_LPInnerFEC(This,FEC)	\
    ( (This)->lpVtbl -> get_LPInnerFEC(This,FEC) ) 

#define IDVBTLocator2_put_LPInnerFEC(This,FEC)	\
    ( (This)->lpVtbl -> put_LPInnerFEC(This,FEC) ) 

#define IDVBTLocator2_get_LPInnerFECRate(This,FEC)	\
    ( (This)->lpVtbl -> get_LPInnerFECRate(This,FEC) ) 

#define IDVBTLocator2_put_LPInnerFECRate(This,FEC)	\
    ( (This)->lpVtbl -> put_LPInnerFECRate(This,FEC) ) 

#define IDVBTLocator2_get_HAlpha(This,Alpha)	\
    ( (This)->lpVtbl -> get_HAlpha(This,Alpha) ) 

#define IDVBTLocator2_put_HAlpha(This,Alpha)	\
    ( (This)->lpVtbl -> put_HAlpha(This,Alpha) ) 

#define IDVBTLocator2_get_Guard(This,GI)	\
    ( (This)->lpVtbl -> get_Guard(This,GI) ) 

#define IDVBTLocator2_put_Guard(This,GI)	\
    ( (This)->lpVtbl -> put_Guard(This,GI) ) 

#define IDVBTLocator2_get_Mode(This,mode)	\
    ( (This)->lpVtbl -> get_Mode(This,mode) ) 

#define IDVBTLocator2_put_Mode(This,mode)	\
    ( (This)->lpVtbl -> put_Mode(This,mode) ) 

#define IDVBTLocator2_get_OtherFrequencyInUse(This,OtherFrequencyInUseVal)	\
    ( (This)->lpVtbl -> get_OtherFrequencyInUse(This,OtherFrequencyInUseVal) ) 

#define IDVBTLocator2_put_OtherFrequencyInUse(This,OtherFrequencyInUseVal)	\
    ( (This)->lpVtbl -> put_OtherFrequencyInUse(This,OtherFrequencyInUseVal) ) 


#define IDVBTLocator2_get_PhysicalLayerPipeId(This,PhysicalLayerPipeIdVal)	\
    ( (This)->lpVtbl -> get_PhysicalLayerPipeId(This,PhysicalLayerPipeIdVal) ) 

#define IDVBTLocator2_put_PhysicalLayerPipeId(This,PhysicalLayerPipeIdVal)	\
    ( (This)->lpVtbl -> put_PhysicalLayerPipeId(This,PhysicalLayerPipeIdVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDVBTLocator2_INTERFACE_DEFINED__ */


#ifndef __IDVBSLocator_INTERFACE_DEFINED__
#define __IDVBSLocator_INTERFACE_DEFINED__

/* interface IDVBSLocator */
/* [unique][helpstring][proxy][oleautomation][dual][uuid][nonextensible][hidden][object] */ 


EXTERN_C const IID IID_IDVBSLocator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3D7C353C-0D04-45f1-A742-F97CC1188DC8")
    IDVBSLocator : public IDigitalLocator
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SignalPolarisation( 
            /* [retval][out] */ __RPC__out Polarisation *PolarisationVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_SignalPolarisation( 
            /* [in] */ Polarisation PolarisationVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_WestPosition( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *WestLongitude) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_WestPosition( 
            /* [in] */ VARIANT_BOOL WestLongitude) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_OrbitalPosition( 
            /* [retval][out] */ __RPC__out long *longitude) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_OrbitalPosition( 
            /* [in] */ long longitude) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Azimuth( 
            /* [retval][out] */ __RPC__out long *Azimuth) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Azimuth( 
            /* [in] */ long Azimuth) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Elevation( 
            /* [retval][out] */ __RPC__out long *Elevation) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Elevation( 
            /* [in] */ long Elevation) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDVBSLocatorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDVBSLocator * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDVBSLocator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDVBSLocator * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IDVBSLocator * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IDVBSLocator * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IDVBSLocator * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IDVBSLocator * This,
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
        
        DECLSPEC_XFGVIRT(ILocator, get_CarrierFrequency)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CarrierFrequency )( 
            __RPC__in IDVBSLocator * This,
            /* [retval][out] */ __RPC__out long *Frequency);
        
        DECLSPEC_XFGVIRT(ILocator, put_CarrierFrequency)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_CarrierFrequency )( 
            __RPC__in IDVBSLocator * This,
            /* [in] */ long Frequency);
        
        DECLSPEC_XFGVIRT(ILocator, get_InnerFEC)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_InnerFEC )( 
            __RPC__in IDVBSLocator * This,
            /* [retval][out] */ __RPC__out FECMethod *FEC);
        
        DECLSPEC_XFGVIRT(ILocator, put_InnerFEC)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_InnerFEC )( 
            __RPC__in IDVBSLocator * This,
            /* [in] */ FECMethod FEC);
        
        DECLSPEC_XFGVIRT(ILocator, get_InnerFECRate)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_InnerFECRate )( 
            __RPC__in IDVBSLocator * This,
            /* [retval][out] */ __RPC__out BinaryConvolutionCodeRate *FEC);
        
        DECLSPEC_XFGVIRT(ILocator, put_InnerFECRate)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_InnerFECRate )( 
            __RPC__in IDVBSLocator * This,
            /* [in] */ BinaryConvolutionCodeRate FEC);
        
        DECLSPEC_XFGVIRT(ILocator, get_OuterFEC)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OuterFEC )( 
            __RPC__in IDVBSLocator * This,
            /* [retval][out] */ __RPC__out FECMethod *FEC);
        
        DECLSPEC_XFGVIRT(ILocator, put_OuterFEC)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_OuterFEC )( 
            __RPC__in IDVBSLocator * This,
            /* [in] */ FECMethod FEC);
        
        DECLSPEC_XFGVIRT(ILocator, get_OuterFECRate)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OuterFECRate )( 
            __RPC__in IDVBSLocator * This,
            /* [retval][out] */ __RPC__out BinaryConvolutionCodeRate *FEC);
        
        DECLSPEC_XFGVIRT(ILocator, put_OuterFECRate)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_OuterFECRate )( 
            __RPC__in IDVBSLocator * This,
            /* [in] */ BinaryConvolutionCodeRate FEC);
        
        DECLSPEC_XFGVIRT(ILocator, get_Modulation)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Modulation )( 
            __RPC__in IDVBSLocator * This,
            /* [retval][out] */ __RPC__out ModulationType *Modulation);
        
        DECLSPEC_XFGVIRT(ILocator, put_Modulation)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Modulation )( 
            __RPC__in IDVBSLocator * This,
            /* [in] */ ModulationType Modulation);
        
        DECLSPEC_XFGVIRT(ILocator, get_SymbolRate)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SymbolRate )( 
            __RPC__in IDVBSLocator * This,
            /* [retval][out] */ __RPC__out long *Rate);
        
        DECLSPEC_XFGVIRT(ILocator, put_SymbolRate)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SymbolRate )( 
            __RPC__in IDVBSLocator * This,
            /* [in] */ long Rate);
        
        DECLSPEC_XFGVIRT(ILocator, Clone)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IDVBSLocator * This,
            /* [retval][out] */ __RPC__deref_out_opt ILocator **NewLocator);
        
        DECLSPEC_XFGVIRT(IDVBSLocator, get_SignalPolarisation)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SignalPolarisation )( 
            __RPC__in IDVBSLocator * This,
            /* [retval][out] */ __RPC__out Polarisation *PolarisationVal);
        
        DECLSPEC_XFGVIRT(IDVBSLocator, put_SignalPolarisation)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SignalPolarisation )( 
            __RPC__in IDVBSLocator * This,
            /* [in] */ Polarisation PolarisationVal);
        
        DECLSPEC_XFGVIRT(IDVBSLocator, get_WestPosition)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_WestPosition )( 
            __RPC__in IDVBSLocator * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *WestLongitude);
        
        DECLSPEC_XFGVIRT(IDVBSLocator, put_WestPosition)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_WestPosition )( 
            __RPC__in IDVBSLocator * This,
            /* [in] */ VARIANT_BOOL WestLongitude);
        
        DECLSPEC_XFGVIRT(IDVBSLocator, get_OrbitalPosition)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OrbitalPosition )( 
            __RPC__in IDVBSLocator * This,
            /* [retval][out] */ __RPC__out long *longitude);
        
        DECLSPEC_XFGVIRT(IDVBSLocator, put_OrbitalPosition)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_OrbitalPosition )( 
            __RPC__in IDVBSLocator * This,
            /* [in] */ long longitude);
        
        DECLSPEC_XFGVIRT(IDVBSLocator, get_Azimuth)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Azimuth )( 
            __RPC__in IDVBSLocator * This,
            /* [retval][out] */ __RPC__out long *Azimuth);
        
        DECLSPEC_XFGVIRT(IDVBSLocator, put_Azimuth)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Azimuth )( 
            __RPC__in IDVBSLocator * This,
            /* [in] */ long Azimuth);
        
        DECLSPEC_XFGVIRT(IDVBSLocator, get_Elevation)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Elevation )( 
            __RPC__in IDVBSLocator * This,
            /* [retval][out] */ __RPC__out long *Elevation);
        
        DECLSPEC_XFGVIRT(IDVBSLocator, put_Elevation)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Elevation )( 
            __RPC__in IDVBSLocator * This,
            /* [in] */ long Elevation);
        
        END_INTERFACE
    } IDVBSLocatorVtbl;

    interface IDVBSLocator
    {
        CONST_VTBL struct IDVBSLocatorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDVBSLocator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDVBSLocator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDVBSLocator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDVBSLocator_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IDVBSLocator_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IDVBSLocator_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IDVBSLocator_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IDVBSLocator_get_CarrierFrequency(This,Frequency)	\
    ( (This)->lpVtbl -> get_CarrierFrequency(This,Frequency) ) 

#define IDVBSLocator_put_CarrierFrequency(This,Frequency)	\
    ( (This)->lpVtbl -> put_CarrierFrequency(This,Frequency) ) 

#define IDVBSLocator_get_InnerFEC(This,FEC)	\
    ( (This)->lpVtbl -> get_InnerFEC(This,FEC) ) 

#define IDVBSLocator_put_InnerFEC(This,FEC)	\
    ( (This)->lpVtbl -> put_InnerFEC(This,FEC) ) 

#define IDVBSLocator_get_InnerFECRate(This,FEC)	\
    ( (This)->lpVtbl -> get_InnerFECRate(This,FEC) ) 

#define IDVBSLocator_put_InnerFECRate(This,FEC)	\
    ( (This)->lpVtbl -> put_InnerFECRate(This,FEC) ) 

#define IDVBSLocator_get_OuterFEC(This,FEC)	\
    ( (This)->lpVtbl -> get_OuterFEC(This,FEC) ) 

#define IDVBSLocator_put_OuterFEC(This,FEC)	\
    ( (This)->lpVtbl -> put_OuterFEC(This,FEC) ) 

#define IDVBSLocator_get_OuterFECRate(This,FEC)	\
    ( (This)->lpVtbl -> get_OuterFECRate(This,FEC) ) 

#define IDVBSLocator_put_OuterFECRate(This,FEC)	\
    ( (This)->lpVtbl -> put_OuterFECRate(This,FEC) ) 

#define IDVBSLocator_get_Modulation(This,Modulation)	\
    ( (This)->lpVtbl -> get_Modulation(This,Modulation) ) 

#define IDVBSLocator_put_Modulation(This,Modulation)	\
    ( (This)->lpVtbl -> put_Modulation(This,Modulation) ) 

#define IDVBSLocator_get_SymbolRate(This,Rate)	\
    ( (This)->lpVtbl -> get_SymbolRate(This,Rate) ) 

#define IDVBSLocator_put_SymbolRate(This,Rate)	\
    ( (This)->lpVtbl -> put_SymbolRate(This,Rate) ) 

#define IDVBSLocator_Clone(This,NewLocator)	\
    ( (This)->lpVtbl -> Clone(This,NewLocator) ) 



#define IDVBSLocator_get_SignalPolarisation(This,PolarisationVal)	\
    ( (This)->lpVtbl -> get_SignalPolarisation(This,PolarisationVal) ) 

#define IDVBSLocator_put_SignalPolarisation(This,PolarisationVal)	\
    ( (This)->lpVtbl -> put_SignalPolarisation(This,PolarisationVal) ) 

#define IDVBSLocator_get_WestPosition(This,WestLongitude)	\
    ( (This)->lpVtbl -> get_WestPosition(This,WestLongitude) ) 

#define IDVBSLocator_put_WestPosition(This,WestLongitude)	\
    ( (This)->lpVtbl -> put_WestPosition(This,WestLongitude) ) 

#define IDVBSLocator_get_OrbitalPosition(This,longitude)	\
    ( (This)->lpVtbl -> get_OrbitalPosition(This,longitude) ) 

#define IDVBSLocator_put_OrbitalPosition(This,longitude)	\
    ( (This)->lpVtbl -> put_OrbitalPosition(This,longitude) ) 

#define IDVBSLocator_get_Azimuth(This,Azimuth)	\
    ( (This)->lpVtbl -> get_Azimuth(This,Azimuth) ) 

#define IDVBSLocator_put_Azimuth(This,Azimuth)	\
    ( (This)->lpVtbl -> put_Azimuth(This,Azimuth) ) 

#define IDVBSLocator_get_Elevation(This,Elevation)	\
    ( (This)->lpVtbl -> get_Elevation(This,Elevation) ) 

#define IDVBSLocator_put_Elevation(This,Elevation)	\
    ( (This)->lpVtbl -> put_Elevation(This,Elevation) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDVBSLocator_INTERFACE_DEFINED__ */


#ifndef __IDVBSLocator2_INTERFACE_DEFINED__
#define __IDVBSLocator2_INTERFACE_DEFINED__

/* interface IDVBSLocator2 */
/* [unique][helpstring][proxy][oleautomation][dual][uuid][nonextensible][hidden][object] */ 


EXTERN_C const IID IID_IDVBSLocator2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6044634A-1733-4f99-B982-5FB12AFCE4F0")
    IDVBSLocator2 : public IDVBSLocator
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DiseqLNBSource( 
            /* [retval][out] */ __RPC__out LNB_Source *DiseqLNBSourceVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_DiseqLNBSource( 
            /* [in] */ LNB_Source DiseqLNBSourceVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_LocalOscillatorOverrideLow( 
            /* [retval][out] */ __RPC__out long *LocalOscillatorOverrideLowVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_LocalOscillatorOverrideLow( 
            /* [in] */ long LocalOscillatorOverrideLowVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_LocalOscillatorOverrideHigh( 
            /* [retval][out] */ __RPC__out long *LocalOscillatorOverrideHighVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_LocalOscillatorOverrideHigh( 
            /* [in] */ long LocalOscillatorOverrideHighVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_LocalLNBSwitchOverride( 
            /* [retval][out] */ __RPC__out long *LocalLNBSwitchOverrideVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_LocalLNBSwitchOverride( 
            /* [in] */ long LocalLNBSwitchOverrideVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_LocalSpectralInversionOverride( 
            /* [retval][out] */ __RPC__out SpectralInversion *LocalSpectralInversionOverrideVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_LocalSpectralInversionOverride( 
            /* [in] */ SpectralInversion LocalSpectralInversionOverrideVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SignalRollOff( 
            /* [retval][out] */ __RPC__out RollOff *RollOffVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_SignalRollOff( 
            /* [in] */ RollOff RollOffVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SignalPilot( 
            /* [retval][out] */ __RPC__out Pilot *PilotVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_SignalPilot( 
            /* [in] */ Pilot PilotVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDVBSLocator2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDVBSLocator2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDVBSLocator2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDVBSLocator2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IDVBSLocator2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IDVBSLocator2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IDVBSLocator2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IDVBSLocator2 * This,
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
        
        DECLSPEC_XFGVIRT(ILocator, get_CarrierFrequency)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CarrierFrequency )( 
            __RPC__in IDVBSLocator2 * This,
            /* [retval][out] */ __RPC__out long *Frequency);
        
        DECLSPEC_XFGVIRT(ILocator, put_CarrierFrequency)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_CarrierFrequency )( 
            __RPC__in IDVBSLocator2 * This,
            /* [in] */ long Frequency);
        
        DECLSPEC_XFGVIRT(ILocator, get_InnerFEC)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_InnerFEC )( 
            __RPC__in IDVBSLocator2 * This,
            /* [retval][out] */ __RPC__out FECMethod *FEC);
        
        DECLSPEC_XFGVIRT(ILocator, put_InnerFEC)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_InnerFEC )( 
            __RPC__in IDVBSLocator2 * This,
            /* [in] */ FECMethod FEC);
        
        DECLSPEC_XFGVIRT(ILocator, get_InnerFECRate)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_InnerFECRate )( 
            __RPC__in IDVBSLocator2 * This,
            /* [retval][out] */ __RPC__out BinaryConvolutionCodeRate *FEC);
        
        DECLSPEC_XFGVIRT(ILocator, put_InnerFECRate)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_InnerFECRate )( 
            __RPC__in IDVBSLocator2 * This,
            /* [in] */ BinaryConvolutionCodeRate FEC);
        
        DECLSPEC_XFGVIRT(ILocator, get_OuterFEC)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OuterFEC )( 
            __RPC__in IDVBSLocator2 * This,
            /* [retval][out] */ __RPC__out FECMethod *FEC);
        
        DECLSPEC_XFGVIRT(ILocator, put_OuterFEC)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_OuterFEC )( 
            __RPC__in IDVBSLocator2 * This,
            /* [in] */ FECMethod FEC);
        
        DECLSPEC_XFGVIRT(ILocator, get_OuterFECRate)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OuterFECRate )( 
            __RPC__in IDVBSLocator2 * This,
            /* [retval][out] */ __RPC__out BinaryConvolutionCodeRate *FEC);
        
        DECLSPEC_XFGVIRT(ILocator, put_OuterFECRate)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_OuterFECRate )( 
            __RPC__in IDVBSLocator2 * This,
            /* [in] */ BinaryConvolutionCodeRate FEC);
        
        DECLSPEC_XFGVIRT(ILocator, get_Modulation)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Modulation )( 
            __RPC__in IDVBSLocator2 * This,
            /* [retval][out] */ __RPC__out ModulationType *Modulation);
        
        DECLSPEC_XFGVIRT(ILocator, put_Modulation)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Modulation )( 
            __RPC__in IDVBSLocator2 * This,
            /* [in] */ ModulationType Modulation);
        
        DECLSPEC_XFGVIRT(ILocator, get_SymbolRate)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SymbolRate )( 
            __RPC__in IDVBSLocator2 * This,
            /* [retval][out] */ __RPC__out long *Rate);
        
        DECLSPEC_XFGVIRT(ILocator, put_SymbolRate)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SymbolRate )( 
            __RPC__in IDVBSLocator2 * This,
            /* [in] */ long Rate);
        
        DECLSPEC_XFGVIRT(ILocator, Clone)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IDVBSLocator2 * This,
            /* [retval][out] */ __RPC__deref_out_opt ILocator **NewLocator);
        
        DECLSPEC_XFGVIRT(IDVBSLocator, get_SignalPolarisation)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SignalPolarisation )( 
            __RPC__in IDVBSLocator2 * This,
            /* [retval][out] */ __RPC__out Polarisation *PolarisationVal);
        
        DECLSPEC_XFGVIRT(IDVBSLocator, put_SignalPolarisation)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SignalPolarisation )( 
            __RPC__in IDVBSLocator2 * This,
            /* [in] */ Polarisation PolarisationVal);
        
        DECLSPEC_XFGVIRT(IDVBSLocator, get_WestPosition)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_WestPosition )( 
            __RPC__in IDVBSLocator2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *WestLongitude);
        
        DECLSPEC_XFGVIRT(IDVBSLocator, put_WestPosition)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_WestPosition )( 
            __RPC__in IDVBSLocator2 * This,
            /* [in] */ VARIANT_BOOL WestLongitude);
        
        DECLSPEC_XFGVIRT(IDVBSLocator, get_OrbitalPosition)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OrbitalPosition )( 
            __RPC__in IDVBSLocator2 * This,
            /* [retval][out] */ __RPC__out long *longitude);
        
        DECLSPEC_XFGVIRT(IDVBSLocator, put_OrbitalPosition)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_OrbitalPosition )( 
            __RPC__in IDVBSLocator2 * This,
            /* [in] */ long longitude);
        
        DECLSPEC_XFGVIRT(IDVBSLocator, get_Azimuth)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Azimuth )( 
            __RPC__in IDVBSLocator2 * This,
            /* [retval][out] */ __RPC__out long *Azimuth);
        
        DECLSPEC_XFGVIRT(IDVBSLocator, put_Azimuth)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Azimuth )( 
            __RPC__in IDVBSLocator2 * This,
            /* [in] */ long Azimuth);
        
        DECLSPEC_XFGVIRT(IDVBSLocator, get_Elevation)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Elevation )( 
            __RPC__in IDVBSLocator2 * This,
            /* [retval][out] */ __RPC__out long *Elevation);
        
        DECLSPEC_XFGVIRT(IDVBSLocator, put_Elevation)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Elevation )( 
            __RPC__in IDVBSLocator2 * This,
            /* [in] */ long Elevation);
        
        DECLSPEC_XFGVIRT(IDVBSLocator2, get_DiseqLNBSource)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DiseqLNBSource )( 
            __RPC__in IDVBSLocator2 * This,
            /* [retval][out] */ __RPC__out LNB_Source *DiseqLNBSourceVal);
        
        DECLSPEC_XFGVIRT(IDVBSLocator2, put_DiseqLNBSource)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DiseqLNBSource )( 
            __RPC__in IDVBSLocator2 * This,
            /* [in] */ LNB_Source DiseqLNBSourceVal);
        
        DECLSPEC_XFGVIRT(IDVBSLocator2, get_LocalOscillatorOverrideLow)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LocalOscillatorOverrideLow )( 
            __RPC__in IDVBSLocator2 * This,
            /* [retval][out] */ __RPC__out long *LocalOscillatorOverrideLowVal);
        
        DECLSPEC_XFGVIRT(IDVBSLocator2, put_LocalOscillatorOverrideLow)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_LocalOscillatorOverrideLow )( 
            __RPC__in IDVBSLocator2 * This,
            /* [in] */ long LocalOscillatorOverrideLowVal);
        
        DECLSPEC_XFGVIRT(IDVBSLocator2, get_LocalOscillatorOverrideHigh)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LocalOscillatorOverrideHigh )( 
            __RPC__in IDVBSLocator2 * This,
            /* [retval][out] */ __RPC__out long *LocalOscillatorOverrideHighVal);
        
        DECLSPEC_XFGVIRT(IDVBSLocator2, put_LocalOscillatorOverrideHigh)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_LocalOscillatorOverrideHigh )( 
            __RPC__in IDVBSLocator2 * This,
            /* [in] */ long LocalOscillatorOverrideHighVal);
        
        DECLSPEC_XFGVIRT(IDVBSLocator2, get_LocalLNBSwitchOverride)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LocalLNBSwitchOverride )( 
            __RPC__in IDVBSLocator2 * This,
            /* [retval][out] */ __RPC__out long *LocalLNBSwitchOverrideVal);
        
        DECLSPEC_XFGVIRT(IDVBSLocator2, put_LocalLNBSwitchOverride)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_LocalLNBSwitchOverride )( 
            __RPC__in IDVBSLocator2 * This,
            /* [in] */ long LocalLNBSwitchOverrideVal);
        
        DECLSPEC_XFGVIRT(IDVBSLocator2, get_LocalSpectralInversionOverride)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LocalSpectralInversionOverride )( 
            __RPC__in IDVBSLocator2 * This,
            /* [retval][out] */ __RPC__out SpectralInversion *LocalSpectralInversionOverrideVal);
        
        DECLSPEC_XFGVIRT(IDVBSLocator2, put_LocalSpectralInversionOverride)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_LocalSpectralInversionOverride )( 
            __RPC__in IDVBSLocator2 * This,
            /* [in] */ SpectralInversion LocalSpectralInversionOverrideVal);
        
        DECLSPEC_XFGVIRT(IDVBSLocator2, get_SignalRollOff)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SignalRollOff )( 
            __RPC__in IDVBSLocator2 * This,
            /* [retval][out] */ __RPC__out RollOff *RollOffVal);
        
        DECLSPEC_XFGVIRT(IDVBSLocator2, put_SignalRollOff)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SignalRollOff )( 
            __RPC__in IDVBSLocator2 * This,
            /* [in] */ RollOff RollOffVal);
        
        DECLSPEC_XFGVIRT(IDVBSLocator2, get_SignalPilot)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SignalPilot )( 
            __RPC__in IDVBSLocator2 * This,
            /* [retval][out] */ __RPC__out Pilot *PilotVal);
        
        DECLSPEC_XFGVIRT(IDVBSLocator2, put_SignalPilot)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SignalPilot )( 
            __RPC__in IDVBSLocator2 * This,
            /* [in] */ Pilot PilotVal);
        
        END_INTERFACE
    } IDVBSLocator2Vtbl;

    interface IDVBSLocator2
    {
        CONST_VTBL struct IDVBSLocator2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDVBSLocator2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDVBSLocator2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDVBSLocator2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDVBSLocator2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IDVBSLocator2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IDVBSLocator2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IDVBSLocator2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IDVBSLocator2_get_CarrierFrequency(This,Frequency)	\
    ( (This)->lpVtbl -> get_CarrierFrequency(This,Frequency) ) 

#define IDVBSLocator2_put_CarrierFrequency(This,Frequency)	\
    ( (This)->lpVtbl -> put_CarrierFrequency(This,Frequency) ) 

#define IDVBSLocator2_get_InnerFEC(This,FEC)	\
    ( (This)->lpVtbl -> get_InnerFEC(This,FEC) ) 

#define IDVBSLocator2_put_InnerFEC(This,FEC)	\
    ( (This)->lpVtbl -> put_InnerFEC(This,FEC) ) 

#define IDVBSLocator2_get_InnerFECRate(This,FEC)	\
    ( (This)->lpVtbl -> get_InnerFECRate(This,FEC) ) 

#define IDVBSLocator2_put_InnerFECRate(This,FEC)	\
    ( (This)->lpVtbl -> put_InnerFECRate(This,FEC) ) 

#define IDVBSLocator2_get_OuterFEC(This,FEC)	\
    ( (This)->lpVtbl -> get_OuterFEC(This,FEC) ) 

#define IDVBSLocator2_put_OuterFEC(This,FEC)	\
    ( (This)->lpVtbl -> put_OuterFEC(This,FEC) ) 

#define IDVBSLocator2_get_OuterFECRate(This,FEC)	\
    ( (This)->lpVtbl -> get_OuterFECRate(This,FEC) ) 

#define IDVBSLocator2_put_OuterFECRate(This,FEC)	\
    ( (This)->lpVtbl -> put_OuterFECRate(This,FEC) ) 

#define IDVBSLocator2_get_Modulation(This,Modulation)	\
    ( (This)->lpVtbl -> get_Modulation(This,Modulation) ) 

#define IDVBSLocator2_put_Modulation(This,Modulation)	\
    ( (This)->lpVtbl -> put_Modulation(This,Modulation) ) 

#define IDVBSLocator2_get_SymbolRate(This,Rate)	\
    ( (This)->lpVtbl -> get_SymbolRate(This,Rate) ) 

#define IDVBSLocator2_put_SymbolRate(This,Rate)	\
    ( (This)->lpVtbl -> put_SymbolRate(This,Rate) ) 

#define IDVBSLocator2_Clone(This,NewLocator)	\
    ( (This)->lpVtbl -> Clone(This,NewLocator) ) 



#define IDVBSLocator2_get_SignalPolarisation(This,PolarisationVal)	\
    ( (This)->lpVtbl -> get_SignalPolarisation(This,PolarisationVal) ) 

#define IDVBSLocator2_put_SignalPolarisation(This,PolarisationVal)	\
    ( (This)->lpVtbl -> put_SignalPolarisation(This,PolarisationVal) ) 

#define IDVBSLocator2_get_WestPosition(This,WestLongitude)	\
    ( (This)->lpVtbl -> get_WestPosition(This,WestLongitude) ) 

#define IDVBSLocator2_put_WestPosition(This,WestLongitude)	\
    ( (This)->lpVtbl -> put_WestPosition(This,WestLongitude) ) 

#define IDVBSLocator2_get_OrbitalPosition(This,longitude)	\
    ( (This)->lpVtbl -> get_OrbitalPosition(This,longitude) ) 

#define IDVBSLocator2_put_OrbitalPosition(This,longitude)	\
    ( (This)->lpVtbl -> put_OrbitalPosition(This,longitude) ) 

#define IDVBSLocator2_get_Azimuth(This,Azimuth)	\
    ( (This)->lpVtbl -> get_Azimuth(This,Azimuth) ) 

#define IDVBSLocator2_put_Azimuth(This,Azimuth)	\
    ( (This)->lpVtbl -> put_Azimuth(This,Azimuth) ) 

#define IDVBSLocator2_get_Elevation(This,Elevation)	\
    ( (This)->lpVtbl -> get_Elevation(This,Elevation) ) 

#define IDVBSLocator2_put_Elevation(This,Elevation)	\
    ( (This)->lpVtbl -> put_Elevation(This,Elevation) ) 


#define IDVBSLocator2_get_DiseqLNBSource(This,DiseqLNBSourceVal)	\
    ( (This)->lpVtbl -> get_DiseqLNBSource(This,DiseqLNBSourceVal) ) 

#define IDVBSLocator2_put_DiseqLNBSource(This,DiseqLNBSourceVal)	\
    ( (This)->lpVtbl -> put_DiseqLNBSource(This,DiseqLNBSourceVal) ) 

#define IDVBSLocator2_get_LocalOscillatorOverrideLow(This,LocalOscillatorOverrideLowVal)	\
    ( (This)->lpVtbl -> get_LocalOscillatorOverrideLow(This,LocalOscillatorOverrideLowVal) ) 

#define IDVBSLocator2_put_LocalOscillatorOverrideLow(This,LocalOscillatorOverrideLowVal)	\
    ( (This)->lpVtbl -> put_LocalOscillatorOverrideLow(This,LocalOscillatorOverrideLowVal) ) 

#define IDVBSLocator2_get_LocalOscillatorOverrideHigh(This,LocalOscillatorOverrideHighVal)	\
    ( (This)->lpVtbl -> get_LocalOscillatorOverrideHigh(This,LocalOscillatorOverrideHighVal) ) 

#define IDVBSLocator2_put_LocalOscillatorOverrideHigh(This,LocalOscillatorOverrideHighVal)	\
    ( (This)->lpVtbl -> put_LocalOscillatorOverrideHigh(This,LocalOscillatorOverrideHighVal) ) 

#define IDVBSLocator2_get_LocalLNBSwitchOverride(This,LocalLNBSwitchOverrideVal)	\
    ( (This)->lpVtbl -> get_LocalLNBSwitchOverride(This,LocalLNBSwitchOverrideVal) ) 

#define IDVBSLocator2_put_LocalLNBSwitchOverride(This,LocalLNBSwitchOverrideVal)	\
    ( (This)->lpVtbl -> put_LocalLNBSwitchOverride(This,LocalLNBSwitchOverrideVal) ) 

#define IDVBSLocator2_get_LocalSpectralInversionOverride(This,LocalSpectralInversionOverrideVal)	\
    ( (This)->lpVtbl -> get_LocalSpectralInversionOverride(This,LocalSpectralInversionOverrideVal) ) 

#define IDVBSLocator2_put_LocalSpectralInversionOverride(This,LocalSpectralInversionOverrideVal)	\
    ( (This)->lpVtbl -> put_LocalSpectralInversionOverride(This,LocalSpectralInversionOverrideVal) ) 

#define IDVBSLocator2_get_SignalRollOff(This,RollOffVal)	\
    ( (This)->lpVtbl -> get_SignalRollOff(This,RollOffVal) ) 

#define IDVBSLocator2_put_SignalRollOff(This,RollOffVal)	\
    ( (This)->lpVtbl -> put_SignalRollOff(This,RollOffVal) ) 

#define IDVBSLocator2_get_SignalPilot(This,PilotVal)	\
    ( (This)->lpVtbl -> get_SignalPilot(This,PilotVal) ) 

#define IDVBSLocator2_put_SignalPilot(This,PilotVal)	\
    ( (This)->lpVtbl -> put_SignalPilot(This,PilotVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDVBSLocator2_INTERFACE_DEFINED__ */


#ifndef __IDVBCLocator_INTERFACE_DEFINED__
#define __IDVBCLocator_INTERFACE_DEFINED__

/* interface IDVBCLocator */
/* [unique][helpstring][proxy][oleautomation][dual][uuid][nonextensible][hidden][object] */ 


EXTERN_C const IID IID_IDVBCLocator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6E42F36E-1DD2-43c4-9F78-69D25AE39034")
    IDVBCLocator : public IDigitalLocator
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct IDVBCLocatorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDVBCLocator * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDVBCLocator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDVBCLocator * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IDVBCLocator * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IDVBCLocator * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IDVBCLocator * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IDVBCLocator * This,
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
        
        DECLSPEC_XFGVIRT(ILocator, get_CarrierFrequency)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CarrierFrequency )( 
            __RPC__in IDVBCLocator * This,
            /* [retval][out] */ __RPC__out long *Frequency);
        
        DECLSPEC_XFGVIRT(ILocator, put_CarrierFrequency)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_CarrierFrequency )( 
            __RPC__in IDVBCLocator * This,
            /* [in] */ long Frequency);
        
        DECLSPEC_XFGVIRT(ILocator, get_InnerFEC)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_InnerFEC )( 
            __RPC__in IDVBCLocator * This,
            /* [retval][out] */ __RPC__out FECMethod *FEC);
        
        DECLSPEC_XFGVIRT(ILocator, put_InnerFEC)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_InnerFEC )( 
            __RPC__in IDVBCLocator * This,
            /* [in] */ FECMethod FEC);
        
        DECLSPEC_XFGVIRT(ILocator, get_InnerFECRate)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_InnerFECRate )( 
            __RPC__in IDVBCLocator * This,
            /* [retval][out] */ __RPC__out BinaryConvolutionCodeRate *FEC);
        
        DECLSPEC_XFGVIRT(ILocator, put_InnerFECRate)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_InnerFECRate )( 
            __RPC__in IDVBCLocator * This,
            /* [in] */ BinaryConvolutionCodeRate FEC);
        
        DECLSPEC_XFGVIRT(ILocator, get_OuterFEC)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OuterFEC )( 
            __RPC__in IDVBCLocator * This,
            /* [retval][out] */ __RPC__out FECMethod *FEC);
        
        DECLSPEC_XFGVIRT(ILocator, put_OuterFEC)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_OuterFEC )( 
            __RPC__in IDVBCLocator * This,
            /* [in] */ FECMethod FEC);
        
        DECLSPEC_XFGVIRT(ILocator, get_OuterFECRate)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OuterFECRate )( 
            __RPC__in IDVBCLocator * This,
            /* [retval][out] */ __RPC__out BinaryConvolutionCodeRate *FEC);
        
        DECLSPEC_XFGVIRT(ILocator, put_OuterFECRate)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_OuterFECRate )( 
            __RPC__in IDVBCLocator * This,
            /* [in] */ BinaryConvolutionCodeRate FEC);
        
        DECLSPEC_XFGVIRT(ILocator, get_Modulation)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Modulation )( 
            __RPC__in IDVBCLocator * This,
            /* [retval][out] */ __RPC__out ModulationType *Modulation);
        
        DECLSPEC_XFGVIRT(ILocator, put_Modulation)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Modulation )( 
            __RPC__in IDVBCLocator * This,
            /* [in] */ ModulationType Modulation);
        
        DECLSPEC_XFGVIRT(ILocator, get_SymbolRate)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SymbolRate )( 
            __RPC__in IDVBCLocator * This,
            /* [retval][out] */ __RPC__out long *Rate);
        
        DECLSPEC_XFGVIRT(ILocator, put_SymbolRate)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SymbolRate )( 
            __RPC__in IDVBCLocator * This,
            /* [in] */ long Rate);
        
        DECLSPEC_XFGVIRT(ILocator, Clone)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IDVBCLocator * This,
            /* [retval][out] */ __RPC__deref_out_opt ILocator **NewLocator);
        
        END_INTERFACE
    } IDVBCLocatorVtbl;

    interface IDVBCLocator
    {
        CONST_VTBL struct IDVBCLocatorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDVBCLocator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDVBCLocator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDVBCLocator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDVBCLocator_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IDVBCLocator_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IDVBCLocator_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IDVBCLocator_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IDVBCLocator_get_CarrierFrequency(This,Frequency)	\
    ( (This)->lpVtbl -> get_CarrierFrequency(This,Frequency) ) 

#define IDVBCLocator_put_CarrierFrequency(This,Frequency)	\
    ( (This)->lpVtbl -> put_CarrierFrequency(This,Frequency) ) 

#define IDVBCLocator_get_InnerFEC(This,FEC)	\
    ( (This)->lpVtbl -> get_InnerFEC(This,FEC) ) 

#define IDVBCLocator_put_InnerFEC(This,FEC)	\
    ( (This)->lpVtbl -> put_InnerFEC(This,FEC) ) 

#define IDVBCLocator_get_InnerFECRate(This,FEC)	\
    ( (This)->lpVtbl -> get_InnerFECRate(This,FEC) ) 

#define IDVBCLocator_put_InnerFECRate(This,FEC)	\
    ( (This)->lpVtbl -> put_InnerFECRate(This,FEC) ) 

#define IDVBCLocator_get_OuterFEC(This,FEC)	\
    ( (This)->lpVtbl -> get_OuterFEC(This,FEC) ) 

#define IDVBCLocator_put_OuterFEC(This,FEC)	\
    ( (This)->lpVtbl -> put_OuterFEC(This,FEC) ) 

#define IDVBCLocator_get_OuterFECRate(This,FEC)	\
    ( (This)->lpVtbl -> get_OuterFECRate(This,FEC) ) 

#define IDVBCLocator_put_OuterFECRate(This,FEC)	\
    ( (This)->lpVtbl -> put_OuterFECRate(This,FEC) ) 

#define IDVBCLocator_get_Modulation(This,Modulation)	\
    ( (This)->lpVtbl -> get_Modulation(This,Modulation) ) 

#define IDVBCLocator_put_Modulation(This,Modulation)	\
    ( (This)->lpVtbl -> put_Modulation(This,Modulation) ) 

#define IDVBCLocator_get_SymbolRate(This,Rate)	\
    ( (This)->lpVtbl -> get_SymbolRate(This,Rate) ) 

#define IDVBCLocator_put_SymbolRate(This,Rate)	\
    ( (This)->lpVtbl -> put_SymbolRate(This,Rate) ) 

#define IDVBCLocator_Clone(This,NewLocator)	\
    ( (This)->lpVtbl -> Clone(This,NewLocator) ) 



#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDVBCLocator_INTERFACE_DEFINED__ */


#ifndef __IISDBSLocator_INTERFACE_DEFINED__
#define __IISDBSLocator_INTERFACE_DEFINED__

/* interface IISDBSLocator */
/* [unique][helpstring][proxy][oleautomation][dual][uuid][nonextensible][hidden][object] */ 


EXTERN_C const IID IID_IISDBSLocator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C9897087-E29C-473f-9E4B-7072123DEA14")
    IISDBSLocator : public IDVBSLocator
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct IISDBSLocatorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IISDBSLocator * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IISDBSLocator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IISDBSLocator * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IISDBSLocator * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IISDBSLocator * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IISDBSLocator * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IISDBSLocator * This,
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
        
        DECLSPEC_XFGVIRT(ILocator, get_CarrierFrequency)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CarrierFrequency )( 
            __RPC__in IISDBSLocator * This,
            /* [retval][out] */ __RPC__out long *Frequency);
        
        DECLSPEC_XFGVIRT(ILocator, put_CarrierFrequency)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_CarrierFrequency )( 
            __RPC__in IISDBSLocator * This,
            /* [in] */ long Frequency);
        
        DECLSPEC_XFGVIRT(ILocator, get_InnerFEC)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_InnerFEC )( 
            __RPC__in IISDBSLocator * This,
            /* [retval][out] */ __RPC__out FECMethod *FEC);
        
        DECLSPEC_XFGVIRT(ILocator, put_InnerFEC)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_InnerFEC )( 
            __RPC__in IISDBSLocator * This,
            /* [in] */ FECMethod FEC);
        
        DECLSPEC_XFGVIRT(ILocator, get_InnerFECRate)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_InnerFECRate )( 
            __RPC__in IISDBSLocator * This,
            /* [retval][out] */ __RPC__out BinaryConvolutionCodeRate *FEC);
        
        DECLSPEC_XFGVIRT(ILocator, put_InnerFECRate)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_InnerFECRate )( 
            __RPC__in IISDBSLocator * This,
            /* [in] */ BinaryConvolutionCodeRate FEC);
        
        DECLSPEC_XFGVIRT(ILocator, get_OuterFEC)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OuterFEC )( 
            __RPC__in IISDBSLocator * This,
            /* [retval][out] */ __RPC__out FECMethod *FEC);
        
        DECLSPEC_XFGVIRT(ILocator, put_OuterFEC)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_OuterFEC )( 
            __RPC__in IISDBSLocator * This,
            /* [in] */ FECMethod FEC);
        
        DECLSPEC_XFGVIRT(ILocator, get_OuterFECRate)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OuterFECRate )( 
            __RPC__in IISDBSLocator * This,
            /* [retval][out] */ __RPC__out BinaryConvolutionCodeRate *FEC);
        
        DECLSPEC_XFGVIRT(ILocator, put_OuterFECRate)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_OuterFECRate )( 
            __RPC__in IISDBSLocator * This,
            /* [in] */ BinaryConvolutionCodeRate FEC);
        
        DECLSPEC_XFGVIRT(ILocator, get_Modulation)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Modulation )( 
            __RPC__in IISDBSLocator * This,
            /* [retval][out] */ __RPC__out ModulationType *Modulation);
        
        DECLSPEC_XFGVIRT(ILocator, put_Modulation)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Modulation )( 
            __RPC__in IISDBSLocator * This,
            /* [in] */ ModulationType Modulation);
        
        DECLSPEC_XFGVIRT(ILocator, get_SymbolRate)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SymbolRate )( 
            __RPC__in IISDBSLocator * This,
            /* [retval][out] */ __RPC__out long *Rate);
        
        DECLSPEC_XFGVIRT(ILocator, put_SymbolRate)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SymbolRate )( 
            __RPC__in IISDBSLocator * This,
            /* [in] */ long Rate);
        
        DECLSPEC_XFGVIRT(ILocator, Clone)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IISDBSLocator * This,
            /* [retval][out] */ __RPC__deref_out_opt ILocator **NewLocator);
        
        DECLSPEC_XFGVIRT(IDVBSLocator, get_SignalPolarisation)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SignalPolarisation )( 
            __RPC__in IISDBSLocator * This,
            /* [retval][out] */ __RPC__out Polarisation *PolarisationVal);
        
        DECLSPEC_XFGVIRT(IDVBSLocator, put_SignalPolarisation)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SignalPolarisation )( 
            __RPC__in IISDBSLocator * This,
            /* [in] */ Polarisation PolarisationVal);
        
        DECLSPEC_XFGVIRT(IDVBSLocator, get_WestPosition)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_WestPosition )( 
            __RPC__in IISDBSLocator * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *WestLongitude);
        
        DECLSPEC_XFGVIRT(IDVBSLocator, put_WestPosition)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_WestPosition )( 
            __RPC__in IISDBSLocator * This,
            /* [in] */ VARIANT_BOOL WestLongitude);
        
        DECLSPEC_XFGVIRT(IDVBSLocator, get_OrbitalPosition)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OrbitalPosition )( 
            __RPC__in IISDBSLocator * This,
            /* [retval][out] */ __RPC__out long *longitude);
        
        DECLSPEC_XFGVIRT(IDVBSLocator, put_OrbitalPosition)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_OrbitalPosition )( 
            __RPC__in IISDBSLocator * This,
            /* [in] */ long longitude);
        
        DECLSPEC_XFGVIRT(IDVBSLocator, get_Azimuth)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Azimuth )( 
            __RPC__in IISDBSLocator * This,
            /* [retval][out] */ __RPC__out long *Azimuth);
        
        DECLSPEC_XFGVIRT(IDVBSLocator, put_Azimuth)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Azimuth )( 
            __RPC__in IISDBSLocator * This,
            /* [in] */ long Azimuth);
        
        DECLSPEC_XFGVIRT(IDVBSLocator, get_Elevation)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Elevation )( 
            __RPC__in IISDBSLocator * This,
            /* [retval][out] */ __RPC__out long *Elevation);
        
        DECLSPEC_XFGVIRT(IDVBSLocator, put_Elevation)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Elevation )( 
            __RPC__in IISDBSLocator * This,
            /* [in] */ long Elevation);
        
        END_INTERFACE
    } IISDBSLocatorVtbl;

    interface IISDBSLocator
    {
        CONST_VTBL struct IISDBSLocatorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IISDBSLocator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IISDBSLocator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IISDBSLocator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IISDBSLocator_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IISDBSLocator_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IISDBSLocator_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IISDBSLocator_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IISDBSLocator_get_CarrierFrequency(This,Frequency)	\
    ( (This)->lpVtbl -> get_CarrierFrequency(This,Frequency) ) 

#define IISDBSLocator_put_CarrierFrequency(This,Frequency)	\
    ( (This)->lpVtbl -> put_CarrierFrequency(This,Frequency) ) 

#define IISDBSLocator_get_InnerFEC(This,FEC)	\
    ( (This)->lpVtbl -> get_InnerFEC(This,FEC) ) 

#define IISDBSLocator_put_InnerFEC(This,FEC)	\
    ( (This)->lpVtbl -> put_InnerFEC(This,FEC) ) 

#define IISDBSLocator_get_InnerFECRate(This,FEC)	\
    ( (This)->lpVtbl -> get_InnerFECRate(This,FEC) ) 

#define IISDBSLocator_put_InnerFECRate(This,FEC)	\
    ( (This)->lpVtbl -> put_InnerFECRate(This,FEC) ) 

#define IISDBSLocator_get_OuterFEC(This,FEC)	\
    ( (This)->lpVtbl -> get_OuterFEC(This,FEC) ) 

#define IISDBSLocator_put_OuterFEC(This,FEC)	\
    ( (This)->lpVtbl -> put_OuterFEC(This,FEC) ) 

#define IISDBSLocator_get_OuterFECRate(This,FEC)	\
    ( (This)->lpVtbl -> get_OuterFECRate(This,FEC) ) 

#define IISDBSLocator_put_OuterFECRate(This,FEC)	\
    ( (This)->lpVtbl -> put_OuterFECRate(This,FEC) ) 

#define IISDBSLocator_get_Modulation(This,Modulation)	\
    ( (This)->lpVtbl -> get_Modulation(This,Modulation) ) 

#define IISDBSLocator_put_Modulation(This,Modulation)	\
    ( (This)->lpVtbl -> put_Modulation(This,Modulation) ) 

#define IISDBSLocator_get_SymbolRate(This,Rate)	\
    ( (This)->lpVtbl -> get_SymbolRate(This,Rate) ) 

#define IISDBSLocator_put_SymbolRate(This,Rate)	\
    ( (This)->lpVtbl -> put_SymbolRate(This,Rate) ) 

#define IISDBSLocator_Clone(This,NewLocator)	\
    ( (This)->lpVtbl -> Clone(This,NewLocator) ) 



#define IISDBSLocator_get_SignalPolarisation(This,PolarisationVal)	\
    ( (This)->lpVtbl -> get_SignalPolarisation(This,PolarisationVal) ) 

#define IISDBSLocator_put_SignalPolarisation(This,PolarisationVal)	\
    ( (This)->lpVtbl -> put_SignalPolarisation(This,PolarisationVal) ) 

#define IISDBSLocator_get_WestPosition(This,WestLongitude)	\
    ( (This)->lpVtbl -> get_WestPosition(This,WestLongitude) ) 

#define IISDBSLocator_put_WestPosition(This,WestLongitude)	\
    ( (This)->lpVtbl -> put_WestPosition(This,WestLongitude) ) 

#define IISDBSLocator_get_OrbitalPosition(This,longitude)	\
    ( (This)->lpVtbl -> get_OrbitalPosition(This,longitude) ) 

#define IISDBSLocator_put_OrbitalPosition(This,longitude)	\
    ( (This)->lpVtbl -> put_OrbitalPosition(This,longitude) ) 

#define IISDBSLocator_get_Azimuth(This,Azimuth)	\
    ( (This)->lpVtbl -> get_Azimuth(This,Azimuth) ) 

#define IISDBSLocator_put_Azimuth(This,Azimuth)	\
    ( (This)->lpVtbl -> put_Azimuth(This,Azimuth) ) 

#define IISDBSLocator_get_Elevation(This,Elevation)	\
    ( (This)->lpVtbl -> get_Elevation(This,Elevation) ) 

#define IISDBSLocator_put_Elevation(This,Elevation)	\
    ( (This)->lpVtbl -> put_Elevation(This,Elevation) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IISDBSLocator_INTERFACE_DEFINED__ */


#ifndef __IESOpenMmiEvent_INTERFACE_DEFINED__
#define __IESOpenMmiEvent_INTERFACE_DEFINED__

/* interface IESOpenMmiEvent */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IESOpenMmiEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BA4B6526-1A35-4635-8B56-3EC612746A8C")
    IESOpenMmiEvent : public IESEvent
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetDialogNumber( 
            /* [out] */ __RPC__out DWORD *pDialogRequest,
            /* [retval][out] */ __RPC__out DWORD *pDialogNumber) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDialogType( 
            /* [retval][out] */ __RPC__out GUID *guidDialogType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDialogData( 
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pbData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDialogStringData( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrBaseUrl,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrData) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IESOpenMmiEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IESOpenMmiEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IESOpenMmiEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IESOpenMmiEvent * This);
        
        DECLSPEC_XFGVIRT(IESEvent, GetEventId)
        HRESULT ( STDMETHODCALLTYPE *GetEventId )( 
            __RPC__in IESOpenMmiEvent * This,
            /* [retval][out] */ __RPC__out DWORD *pdwEventId);
        
        DECLSPEC_XFGVIRT(IESEvent, GetEventType)
        HRESULT ( STDMETHODCALLTYPE *GetEventType )( 
            __RPC__in IESOpenMmiEvent * This,
            /* [retval][out] */ __RPC__out GUID *pguidEventType);
        
        DECLSPEC_XFGVIRT(IESEvent, SetCompletionStatus)
        HRESULT ( STDMETHODCALLTYPE *SetCompletionStatus )( 
            __RPC__in IESOpenMmiEvent * This,
            /* [in] */ DWORD dwResult);
        
        DECLSPEC_XFGVIRT(IESEvent, GetData)
        HRESULT ( STDMETHODCALLTYPE *GetData )( 
            __RPC__in IESOpenMmiEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pbData);
        
        DECLSPEC_XFGVIRT(IESEvent, GetStringData)
        HRESULT ( STDMETHODCALLTYPE *GetStringData )( 
            __RPC__in IESOpenMmiEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrData);
        
        DECLSPEC_XFGVIRT(IESOpenMmiEvent, GetDialogNumber)
        HRESULT ( STDMETHODCALLTYPE *GetDialogNumber )( 
            __RPC__in IESOpenMmiEvent * This,
            /* [out] */ __RPC__out DWORD *pDialogRequest,
            /* [retval][out] */ __RPC__out DWORD *pDialogNumber);
        
        DECLSPEC_XFGVIRT(IESOpenMmiEvent, GetDialogType)
        HRESULT ( STDMETHODCALLTYPE *GetDialogType )( 
            __RPC__in IESOpenMmiEvent * This,
            /* [retval][out] */ __RPC__out GUID *guidDialogType);
        
        DECLSPEC_XFGVIRT(IESOpenMmiEvent, GetDialogData)
        HRESULT ( STDMETHODCALLTYPE *GetDialogData )( 
            __RPC__in IESOpenMmiEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pbData);
        
        DECLSPEC_XFGVIRT(IESOpenMmiEvent, GetDialogStringData)
        HRESULT ( STDMETHODCALLTYPE *GetDialogStringData )( 
            __RPC__in IESOpenMmiEvent * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrBaseUrl,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrData);
        
        END_INTERFACE
    } IESOpenMmiEventVtbl;

    interface IESOpenMmiEvent
    {
        CONST_VTBL struct IESOpenMmiEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IESOpenMmiEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IESOpenMmiEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IESOpenMmiEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IESOpenMmiEvent_GetEventId(This,pdwEventId)	\
    ( (This)->lpVtbl -> GetEventId(This,pdwEventId) ) 

#define IESOpenMmiEvent_GetEventType(This,pguidEventType)	\
    ( (This)->lpVtbl -> GetEventType(This,pguidEventType) ) 

#define IESOpenMmiEvent_SetCompletionStatus(This,dwResult)	\
    ( (This)->lpVtbl -> SetCompletionStatus(This,dwResult) ) 

#define IESOpenMmiEvent_GetData(This,pbData)	\
    ( (This)->lpVtbl -> GetData(This,pbData) ) 

#define IESOpenMmiEvent_GetStringData(This,pbstrData)	\
    ( (This)->lpVtbl -> GetStringData(This,pbstrData) ) 


#define IESOpenMmiEvent_GetDialogNumber(This,pDialogRequest,pDialogNumber)	\
    ( (This)->lpVtbl -> GetDialogNumber(This,pDialogRequest,pDialogNumber) ) 

#define IESOpenMmiEvent_GetDialogType(This,guidDialogType)	\
    ( (This)->lpVtbl -> GetDialogType(This,guidDialogType) ) 

#define IESOpenMmiEvent_GetDialogData(This,pbData)	\
    ( (This)->lpVtbl -> GetDialogData(This,pbData) ) 

#define IESOpenMmiEvent_GetDialogStringData(This,pbstrBaseUrl,pbstrData)	\
    ( (This)->lpVtbl -> GetDialogStringData(This,pbstrBaseUrl,pbstrData) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IESOpenMmiEvent_INTERFACE_DEFINED__ */


#ifndef __IESCloseMmiEvent_INTERFACE_DEFINED__
#define __IESCloseMmiEvent_INTERFACE_DEFINED__

/* interface IESCloseMmiEvent */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IESCloseMmiEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6B80E96F-55E2-45aa-B754-0C23C8E7D5C1")
    IESCloseMmiEvent : public IESEvent
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetDialogNumber( 
            /* [retval][out] */ __RPC__out DWORD *pDialogNumber) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IESCloseMmiEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IESCloseMmiEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IESCloseMmiEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IESCloseMmiEvent * This);
        
        DECLSPEC_XFGVIRT(IESEvent, GetEventId)
        HRESULT ( STDMETHODCALLTYPE *GetEventId )( 
            __RPC__in IESCloseMmiEvent * This,
            /* [retval][out] */ __RPC__out DWORD *pdwEventId);
        
        DECLSPEC_XFGVIRT(IESEvent, GetEventType)
        HRESULT ( STDMETHODCALLTYPE *GetEventType )( 
            __RPC__in IESCloseMmiEvent * This,
            /* [retval][out] */ __RPC__out GUID *pguidEventType);
        
        DECLSPEC_XFGVIRT(IESEvent, SetCompletionStatus)
        HRESULT ( STDMETHODCALLTYPE *SetCompletionStatus )( 
            __RPC__in IESCloseMmiEvent * This,
            /* [in] */ DWORD dwResult);
        
        DECLSPEC_XFGVIRT(IESEvent, GetData)
        HRESULT ( STDMETHODCALLTYPE *GetData )( 
            __RPC__in IESCloseMmiEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pbData);
        
        DECLSPEC_XFGVIRT(IESEvent, GetStringData)
        HRESULT ( STDMETHODCALLTYPE *GetStringData )( 
            __RPC__in IESCloseMmiEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrData);
        
        DECLSPEC_XFGVIRT(IESCloseMmiEvent, GetDialogNumber)
        HRESULT ( STDMETHODCALLTYPE *GetDialogNumber )( 
            __RPC__in IESCloseMmiEvent * This,
            /* [retval][out] */ __RPC__out DWORD *pDialogNumber);
        
        END_INTERFACE
    } IESCloseMmiEventVtbl;

    interface IESCloseMmiEvent
    {
        CONST_VTBL struct IESCloseMmiEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IESCloseMmiEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IESCloseMmiEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IESCloseMmiEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IESCloseMmiEvent_GetEventId(This,pdwEventId)	\
    ( (This)->lpVtbl -> GetEventId(This,pdwEventId) ) 

#define IESCloseMmiEvent_GetEventType(This,pguidEventType)	\
    ( (This)->lpVtbl -> GetEventType(This,pguidEventType) ) 

#define IESCloseMmiEvent_SetCompletionStatus(This,dwResult)	\
    ( (This)->lpVtbl -> SetCompletionStatus(This,dwResult) ) 

#define IESCloseMmiEvent_GetData(This,pbData)	\
    ( (This)->lpVtbl -> GetData(This,pbData) ) 

#define IESCloseMmiEvent_GetStringData(This,pbstrData)	\
    ( (This)->lpVtbl -> GetStringData(This,pbstrData) ) 


#define IESCloseMmiEvent_GetDialogNumber(This,pDialogNumber)	\
    ( (This)->lpVtbl -> GetDialogNumber(This,pDialogNumber) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IESCloseMmiEvent_INTERFACE_DEFINED__ */


#ifndef __IESValueUpdatedEvent_INTERFACE_DEFINED__
#define __IESValueUpdatedEvent_INTERFACE_DEFINED__

/* interface IESValueUpdatedEvent */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IESValueUpdatedEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8A24C46E-BB63-4664-8602-5D9C718C146D")
    IESValueUpdatedEvent : public IESEvent
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetValueNames( 
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pbstrNames) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IESValueUpdatedEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IESValueUpdatedEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IESValueUpdatedEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IESValueUpdatedEvent * This);
        
        DECLSPEC_XFGVIRT(IESEvent, GetEventId)
        HRESULT ( STDMETHODCALLTYPE *GetEventId )( 
            __RPC__in IESValueUpdatedEvent * This,
            /* [retval][out] */ __RPC__out DWORD *pdwEventId);
        
        DECLSPEC_XFGVIRT(IESEvent, GetEventType)
        HRESULT ( STDMETHODCALLTYPE *GetEventType )( 
            __RPC__in IESValueUpdatedEvent * This,
            /* [retval][out] */ __RPC__out GUID *pguidEventType);
        
        DECLSPEC_XFGVIRT(IESEvent, SetCompletionStatus)
        HRESULT ( STDMETHODCALLTYPE *SetCompletionStatus )( 
            __RPC__in IESValueUpdatedEvent * This,
            /* [in] */ DWORD dwResult);
        
        DECLSPEC_XFGVIRT(IESEvent, GetData)
        HRESULT ( STDMETHODCALLTYPE *GetData )( 
            __RPC__in IESValueUpdatedEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pbData);
        
        DECLSPEC_XFGVIRT(IESEvent, GetStringData)
        HRESULT ( STDMETHODCALLTYPE *GetStringData )( 
            __RPC__in IESValueUpdatedEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrData);
        
        DECLSPEC_XFGVIRT(IESValueUpdatedEvent, GetValueNames)
        HRESULT ( STDMETHODCALLTYPE *GetValueNames )( 
            __RPC__in IESValueUpdatedEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pbstrNames);
        
        END_INTERFACE
    } IESValueUpdatedEventVtbl;

    interface IESValueUpdatedEvent
    {
        CONST_VTBL struct IESValueUpdatedEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IESValueUpdatedEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IESValueUpdatedEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IESValueUpdatedEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IESValueUpdatedEvent_GetEventId(This,pdwEventId)	\
    ( (This)->lpVtbl -> GetEventId(This,pdwEventId) ) 

#define IESValueUpdatedEvent_GetEventType(This,pguidEventType)	\
    ( (This)->lpVtbl -> GetEventType(This,pguidEventType) ) 

#define IESValueUpdatedEvent_SetCompletionStatus(This,dwResult)	\
    ( (This)->lpVtbl -> SetCompletionStatus(This,dwResult) ) 

#define IESValueUpdatedEvent_GetData(This,pbData)	\
    ( (This)->lpVtbl -> GetData(This,pbData) ) 

#define IESValueUpdatedEvent_GetStringData(This,pbstrData)	\
    ( (This)->lpVtbl -> GetStringData(This,pbstrData) ) 


#define IESValueUpdatedEvent_GetValueNames(This,pbstrNames)	\
    ( (This)->lpVtbl -> GetValueNames(This,pbstrNames) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IESValueUpdatedEvent_INTERFACE_DEFINED__ */


#ifndef __IESRequestTunerEvent_INTERFACE_DEFINED__
#define __IESRequestTunerEvent_INTERFACE_DEFINED__

/* interface IESRequestTunerEvent */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IESRequestTunerEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("54C7A5E8-C3BB-4f51-AF14-E0E2C0E34C6D")
    IESRequestTunerEvent : public IESEvent
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetPriority( 
            /* [retval][out] */ __RPC__out BYTE *pbyPriority) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetReason( 
            /* [retval][out] */ __RPC__out BYTE *pbyReason) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetConsequences( 
            /* [retval][out] */ __RPC__out BYTE *pbyConsequences) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEstimatedTime( 
            /* [retval][out] */ __RPC__out DWORD *pdwEstimatedTime) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IESRequestTunerEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IESRequestTunerEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IESRequestTunerEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IESRequestTunerEvent * This);
        
        DECLSPEC_XFGVIRT(IESEvent, GetEventId)
        HRESULT ( STDMETHODCALLTYPE *GetEventId )( 
            __RPC__in IESRequestTunerEvent * This,
            /* [retval][out] */ __RPC__out DWORD *pdwEventId);
        
        DECLSPEC_XFGVIRT(IESEvent, GetEventType)
        HRESULT ( STDMETHODCALLTYPE *GetEventType )( 
            __RPC__in IESRequestTunerEvent * This,
            /* [retval][out] */ __RPC__out GUID *pguidEventType);
        
        DECLSPEC_XFGVIRT(IESEvent, SetCompletionStatus)
        HRESULT ( STDMETHODCALLTYPE *SetCompletionStatus )( 
            __RPC__in IESRequestTunerEvent * This,
            /* [in] */ DWORD dwResult);
        
        DECLSPEC_XFGVIRT(IESEvent, GetData)
        HRESULT ( STDMETHODCALLTYPE *GetData )( 
            __RPC__in IESRequestTunerEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pbData);
        
        DECLSPEC_XFGVIRT(IESEvent, GetStringData)
        HRESULT ( STDMETHODCALLTYPE *GetStringData )( 
            __RPC__in IESRequestTunerEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrData);
        
        DECLSPEC_XFGVIRT(IESRequestTunerEvent, GetPriority)
        HRESULT ( STDMETHODCALLTYPE *GetPriority )( 
            __RPC__in IESRequestTunerEvent * This,
            /* [retval][out] */ __RPC__out BYTE *pbyPriority);
        
        DECLSPEC_XFGVIRT(IESRequestTunerEvent, GetReason)
        HRESULT ( STDMETHODCALLTYPE *GetReason )( 
            __RPC__in IESRequestTunerEvent * This,
            /* [retval][out] */ __RPC__out BYTE *pbyReason);
        
        DECLSPEC_XFGVIRT(IESRequestTunerEvent, GetConsequences)
        HRESULT ( STDMETHODCALLTYPE *GetConsequences )( 
            __RPC__in IESRequestTunerEvent * This,
            /* [retval][out] */ __RPC__out BYTE *pbyConsequences);
        
        DECLSPEC_XFGVIRT(IESRequestTunerEvent, GetEstimatedTime)
        HRESULT ( STDMETHODCALLTYPE *GetEstimatedTime )( 
            __RPC__in IESRequestTunerEvent * This,
            /* [retval][out] */ __RPC__out DWORD *pdwEstimatedTime);
        
        END_INTERFACE
    } IESRequestTunerEventVtbl;

    interface IESRequestTunerEvent
    {
        CONST_VTBL struct IESRequestTunerEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IESRequestTunerEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IESRequestTunerEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IESRequestTunerEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IESRequestTunerEvent_GetEventId(This,pdwEventId)	\
    ( (This)->lpVtbl -> GetEventId(This,pdwEventId) ) 

#define IESRequestTunerEvent_GetEventType(This,pguidEventType)	\
    ( (This)->lpVtbl -> GetEventType(This,pguidEventType) ) 

#define IESRequestTunerEvent_SetCompletionStatus(This,dwResult)	\
    ( (This)->lpVtbl -> SetCompletionStatus(This,dwResult) ) 

#define IESRequestTunerEvent_GetData(This,pbData)	\
    ( (This)->lpVtbl -> GetData(This,pbData) ) 

#define IESRequestTunerEvent_GetStringData(This,pbstrData)	\
    ( (This)->lpVtbl -> GetStringData(This,pbstrData) ) 


#define IESRequestTunerEvent_GetPriority(This,pbyPriority)	\
    ( (This)->lpVtbl -> GetPriority(This,pbyPriority) ) 

#define IESRequestTunerEvent_GetReason(This,pbyReason)	\
    ( (This)->lpVtbl -> GetReason(This,pbyReason) ) 

#define IESRequestTunerEvent_GetConsequences(This,pbyConsequences)	\
    ( (This)->lpVtbl -> GetConsequences(This,pbyConsequences) ) 

#define IESRequestTunerEvent_GetEstimatedTime(This,pdwEstimatedTime)	\
    ( (This)->lpVtbl -> GetEstimatedTime(This,pdwEstimatedTime) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IESRequestTunerEvent_INTERFACE_DEFINED__ */


#ifndef __IESIsdbCasResponseEvent_INTERFACE_DEFINED__
#define __IESIsdbCasResponseEvent_INTERFACE_DEFINED__

/* interface IESIsdbCasResponseEvent */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IESIsdbCasResponseEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2017cb03-dc0f-4c24-83ca-36307b2cd19f")
    IESIsdbCasResponseEvent : public IESEvent
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetRequestId( 
            /* [retval][out] */ __RPC__out DWORD *pRequestId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStatus( 
            /* [retval][out] */ __RPC__out DWORD *pStatus) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDataLength( 
            /* [retval][out] */ __RPC__out DWORD *pRequestLength) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetResponseData( 
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pbData) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IESIsdbCasResponseEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IESIsdbCasResponseEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IESIsdbCasResponseEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IESIsdbCasResponseEvent * This);
        
        DECLSPEC_XFGVIRT(IESEvent, GetEventId)
        HRESULT ( STDMETHODCALLTYPE *GetEventId )( 
            __RPC__in IESIsdbCasResponseEvent * This,
            /* [retval][out] */ __RPC__out DWORD *pdwEventId);
        
        DECLSPEC_XFGVIRT(IESEvent, GetEventType)
        HRESULT ( STDMETHODCALLTYPE *GetEventType )( 
            __RPC__in IESIsdbCasResponseEvent * This,
            /* [retval][out] */ __RPC__out GUID *pguidEventType);
        
        DECLSPEC_XFGVIRT(IESEvent, SetCompletionStatus)
        HRESULT ( STDMETHODCALLTYPE *SetCompletionStatus )( 
            __RPC__in IESIsdbCasResponseEvent * This,
            /* [in] */ DWORD dwResult);
        
        DECLSPEC_XFGVIRT(IESEvent, GetData)
        HRESULT ( STDMETHODCALLTYPE *GetData )( 
            __RPC__in IESIsdbCasResponseEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pbData);
        
        DECLSPEC_XFGVIRT(IESEvent, GetStringData)
        HRESULT ( STDMETHODCALLTYPE *GetStringData )( 
            __RPC__in IESIsdbCasResponseEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrData);
        
        DECLSPEC_XFGVIRT(IESIsdbCasResponseEvent, GetRequestId)
        HRESULT ( STDMETHODCALLTYPE *GetRequestId )( 
            __RPC__in IESIsdbCasResponseEvent * This,
            /* [retval][out] */ __RPC__out DWORD *pRequestId);
        
        DECLSPEC_XFGVIRT(IESIsdbCasResponseEvent, GetStatus)
        HRESULT ( STDMETHODCALLTYPE *GetStatus )( 
            __RPC__in IESIsdbCasResponseEvent * This,
            /* [retval][out] */ __RPC__out DWORD *pStatus);
        
        DECLSPEC_XFGVIRT(IESIsdbCasResponseEvent, GetDataLength)
        HRESULT ( STDMETHODCALLTYPE *GetDataLength )( 
            __RPC__in IESIsdbCasResponseEvent * This,
            /* [retval][out] */ __RPC__out DWORD *pRequestLength);
        
        DECLSPEC_XFGVIRT(IESIsdbCasResponseEvent, GetResponseData)
        HRESULT ( STDMETHODCALLTYPE *GetResponseData )( 
            __RPC__in IESIsdbCasResponseEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pbData);
        
        END_INTERFACE
    } IESIsdbCasResponseEventVtbl;

    interface IESIsdbCasResponseEvent
    {
        CONST_VTBL struct IESIsdbCasResponseEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IESIsdbCasResponseEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IESIsdbCasResponseEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IESIsdbCasResponseEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IESIsdbCasResponseEvent_GetEventId(This,pdwEventId)	\
    ( (This)->lpVtbl -> GetEventId(This,pdwEventId) ) 

#define IESIsdbCasResponseEvent_GetEventType(This,pguidEventType)	\
    ( (This)->lpVtbl -> GetEventType(This,pguidEventType) ) 

#define IESIsdbCasResponseEvent_SetCompletionStatus(This,dwResult)	\
    ( (This)->lpVtbl -> SetCompletionStatus(This,dwResult) ) 

#define IESIsdbCasResponseEvent_GetData(This,pbData)	\
    ( (This)->lpVtbl -> GetData(This,pbData) ) 

#define IESIsdbCasResponseEvent_GetStringData(This,pbstrData)	\
    ( (This)->lpVtbl -> GetStringData(This,pbstrData) ) 


#define IESIsdbCasResponseEvent_GetRequestId(This,pRequestId)	\
    ( (This)->lpVtbl -> GetRequestId(This,pRequestId) ) 

#define IESIsdbCasResponseEvent_GetStatus(This,pStatus)	\
    ( (This)->lpVtbl -> GetStatus(This,pStatus) ) 

#define IESIsdbCasResponseEvent_GetDataLength(This,pRequestLength)	\
    ( (This)->lpVtbl -> GetDataLength(This,pRequestLength) ) 

#define IESIsdbCasResponseEvent_GetResponseData(This,pbData)	\
    ( (This)->lpVtbl -> GetResponseData(This,pbData) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IESIsdbCasResponseEvent_INTERFACE_DEFINED__ */


#ifndef __IGpnvsCommonBase_INTERFACE_DEFINED__
#define __IGpnvsCommonBase_INTERFACE_DEFINED__

/* interface IGpnvsCommonBase */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IGpnvsCommonBase;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("907E0B5C-E42D-4f04-91F0-26F401F36907")
    IGpnvsCommonBase : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetValueUpdateName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGpnvsCommonBaseVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGpnvsCommonBase * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGpnvsCommonBase * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGpnvsCommonBase * This);
        
        DECLSPEC_XFGVIRT(IGpnvsCommonBase, GetValueUpdateName)
        HRESULT ( STDMETHODCALLTYPE *GetValueUpdateName )( 
            __RPC__in IGpnvsCommonBase * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        END_INTERFACE
    } IGpnvsCommonBaseVtbl;

    interface IGpnvsCommonBase
    {
        CONST_VTBL struct IGpnvsCommonBaseVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGpnvsCommonBase_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGpnvsCommonBase_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGpnvsCommonBase_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGpnvsCommonBase_GetValueUpdateName(This,pbstrName)	\
    ( (This)->lpVtbl -> GetValueUpdateName(This,pbstrName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGpnvsCommonBase_INTERFACE_DEFINED__ */


#ifndef __IESEventFactory_INTERFACE_DEFINED__
#define __IESEventFactory_INTERFACE_DEFINED__

/* interface IESEventFactory */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IESEventFactory;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("506A09B8-7F86-4e04-AC05-3303BFE8FC49")
    IESEventFactory : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateESEvent( 
            /* [in] */ __RPC__in_opt IUnknown *pServiceProvider,
            /* [in] */ DWORD dwEventId,
            /* [in] */ GUID guidEventType,
            /* [in] */ DWORD dwEventDataLength,
            /* [size_is][in] */ __RPC__in_ecount_full(dwEventDataLength) BYTE *pEventData,
            /* [in] */ __RPC__in BSTR bstrBaseUrl,
            /* [in] */ __RPC__in_opt IUnknown *pInitContext,
            /* [retval][out] */ __RPC__deref_out_opt IESEvent **ppESEvent) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IESEventFactoryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IESEventFactory * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IESEventFactory * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IESEventFactory * This);
        
        DECLSPEC_XFGVIRT(IESEventFactory, CreateESEvent)
        HRESULT ( STDMETHODCALLTYPE *CreateESEvent )( 
            __RPC__in IESEventFactory * This,
            /* [in] */ __RPC__in_opt IUnknown *pServiceProvider,
            /* [in] */ DWORD dwEventId,
            /* [in] */ GUID guidEventType,
            /* [in] */ DWORD dwEventDataLength,
            /* [size_is][in] */ __RPC__in_ecount_full(dwEventDataLength) BYTE *pEventData,
            /* [in] */ __RPC__in BSTR bstrBaseUrl,
            /* [in] */ __RPC__in_opt IUnknown *pInitContext,
            /* [retval][out] */ __RPC__deref_out_opt IESEvent **ppESEvent);
        
        END_INTERFACE
    } IESEventFactoryVtbl;

    interface IESEventFactory
    {
        CONST_VTBL struct IESEventFactoryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IESEventFactory_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IESEventFactory_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IESEventFactory_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IESEventFactory_CreateESEvent(This,pServiceProvider,dwEventId,guidEventType,dwEventDataLength,pEventData,bstrBaseUrl,pInitContext,ppESEvent)	\
    ( (This)->lpVtbl -> CreateESEvent(This,pServiceProvider,dwEventId,guidEventType,dwEventDataLength,pEventData,bstrBaseUrl,pInitContext,ppESEvent) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IESEventFactory_INTERFACE_DEFINED__ */


#ifndef __IESLicenseRenewalResultEvent_INTERFACE_DEFINED__
#define __IESLicenseRenewalResultEvent_INTERFACE_DEFINED__

/* interface IESLicenseRenewalResultEvent */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IESLicenseRenewalResultEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("d5a48ef5-a81b-4df0-acaa-5e35e7ea45d4")
    IESLicenseRenewalResultEvent : public IESEvent
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCallersId( 
            /* [retval][out] */ __RPC__out DWORD *pdwCallersId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFileName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrFilename) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsRenewalSuccessful( 
            /* [retval][out] */ __RPC__out BOOL *pfRenewalSuccessful) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsCheckEntitlementCallRequired( 
            /* [retval][out] */ __RPC__out BOOL *pfCheckEntTokenCallNeeded) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDescrambledStatus( 
            /* [retval][out] */ __RPC__out DWORD *pDescrambledStatus) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRenewalResultCode( 
            /* [retval][out] */ __RPC__out DWORD *pdwRenewalResultCode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCASFailureCode( 
            /* [retval][out] */ __RPC__out DWORD *pdwCASFailureCode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRenewalHResult( 
            /* [retval][out] */ __RPC__out HRESULT *phr) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEntitlementTokenLength( 
            /* [retval][out] */ __RPC__out DWORD *pdwLength) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEntitlementToken( 
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pbData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetExpiryDate( 
            /* [retval][out] */ __RPC__out DWORD64 *pqwExpiryDate) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IESLicenseRenewalResultEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IESLicenseRenewalResultEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IESLicenseRenewalResultEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IESLicenseRenewalResultEvent * This);
        
        DECLSPEC_XFGVIRT(IESEvent, GetEventId)
        HRESULT ( STDMETHODCALLTYPE *GetEventId )( 
            __RPC__in IESLicenseRenewalResultEvent * This,
            /* [retval][out] */ __RPC__out DWORD *pdwEventId);
        
        DECLSPEC_XFGVIRT(IESEvent, GetEventType)
        HRESULT ( STDMETHODCALLTYPE *GetEventType )( 
            __RPC__in IESLicenseRenewalResultEvent * This,
            /* [retval][out] */ __RPC__out GUID *pguidEventType);
        
        DECLSPEC_XFGVIRT(IESEvent, SetCompletionStatus)
        HRESULT ( STDMETHODCALLTYPE *SetCompletionStatus )( 
            __RPC__in IESLicenseRenewalResultEvent * This,
            /* [in] */ DWORD dwResult);
        
        DECLSPEC_XFGVIRT(IESEvent, GetData)
        HRESULT ( STDMETHODCALLTYPE *GetData )( 
            __RPC__in IESLicenseRenewalResultEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pbData);
        
        DECLSPEC_XFGVIRT(IESEvent, GetStringData)
        HRESULT ( STDMETHODCALLTYPE *GetStringData )( 
            __RPC__in IESLicenseRenewalResultEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrData);
        
        DECLSPEC_XFGVIRT(IESLicenseRenewalResultEvent, GetCallersId)
        HRESULT ( STDMETHODCALLTYPE *GetCallersId )( 
            __RPC__in IESLicenseRenewalResultEvent * This,
            /* [retval][out] */ __RPC__out DWORD *pdwCallersId);
        
        DECLSPEC_XFGVIRT(IESLicenseRenewalResultEvent, GetFileName)
        HRESULT ( STDMETHODCALLTYPE *GetFileName )( 
            __RPC__in IESLicenseRenewalResultEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrFilename);
        
        DECLSPEC_XFGVIRT(IESLicenseRenewalResultEvent, IsRenewalSuccessful)
        HRESULT ( STDMETHODCALLTYPE *IsRenewalSuccessful )( 
            __RPC__in IESLicenseRenewalResultEvent * This,
            /* [retval][out] */ __RPC__out BOOL *pfRenewalSuccessful);
        
        DECLSPEC_XFGVIRT(IESLicenseRenewalResultEvent, IsCheckEntitlementCallRequired)
        HRESULT ( STDMETHODCALLTYPE *IsCheckEntitlementCallRequired )( 
            __RPC__in IESLicenseRenewalResultEvent * This,
            /* [retval][out] */ __RPC__out BOOL *pfCheckEntTokenCallNeeded);
        
        DECLSPEC_XFGVIRT(IESLicenseRenewalResultEvent, GetDescrambledStatus)
        HRESULT ( STDMETHODCALLTYPE *GetDescrambledStatus )( 
            __RPC__in IESLicenseRenewalResultEvent * This,
            /* [retval][out] */ __RPC__out DWORD *pDescrambledStatus);
        
        DECLSPEC_XFGVIRT(IESLicenseRenewalResultEvent, GetRenewalResultCode)
        HRESULT ( STDMETHODCALLTYPE *GetRenewalResultCode )( 
            __RPC__in IESLicenseRenewalResultEvent * This,
            /* [retval][out] */ __RPC__out DWORD *pdwRenewalResultCode);
        
        DECLSPEC_XFGVIRT(IESLicenseRenewalResultEvent, GetCASFailureCode)
        HRESULT ( STDMETHODCALLTYPE *GetCASFailureCode )( 
            __RPC__in IESLicenseRenewalResultEvent * This,
            /* [retval][out] */ __RPC__out DWORD *pdwCASFailureCode);
        
        DECLSPEC_XFGVIRT(IESLicenseRenewalResultEvent, GetRenewalHResult)
        HRESULT ( STDMETHODCALLTYPE *GetRenewalHResult )( 
            __RPC__in IESLicenseRenewalResultEvent * This,
            /* [retval][out] */ __RPC__out HRESULT *phr);
        
        DECLSPEC_XFGVIRT(IESLicenseRenewalResultEvent, GetEntitlementTokenLength)
        HRESULT ( STDMETHODCALLTYPE *GetEntitlementTokenLength )( 
            __RPC__in IESLicenseRenewalResultEvent * This,
            /* [retval][out] */ __RPC__out DWORD *pdwLength);
        
        DECLSPEC_XFGVIRT(IESLicenseRenewalResultEvent, GetEntitlementToken)
        HRESULT ( STDMETHODCALLTYPE *GetEntitlementToken )( 
            __RPC__in IESLicenseRenewalResultEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pbData);
        
        DECLSPEC_XFGVIRT(IESLicenseRenewalResultEvent, GetExpiryDate)
        HRESULT ( STDMETHODCALLTYPE *GetExpiryDate )( 
            __RPC__in IESLicenseRenewalResultEvent * This,
            /* [retval][out] */ __RPC__out DWORD64 *pqwExpiryDate);
        
        END_INTERFACE
    } IESLicenseRenewalResultEventVtbl;

    interface IESLicenseRenewalResultEvent
    {
        CONST_VTBL struct IESLicenseRenewalResultEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IESLicenseRenewalResultEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IESLicenseRenewalResultEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IESLicenseRenewalResultEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IESLicenseRenewalResultEvent_GetEventId(This,pdwEventId)	\
    ( (This)->lpVtbl -> GetEventId(This,pdwEventId) ) 

#define IESLicenseRenewalResultEvent_GetEventType(This,pguidEventType)	\
    ( (This)->lpVtbl -> GetEventType(This,pguidEventType) ) 

#define IESLicenseRenewalResultEvent_SetCompletionStatus(This,dwResult)	\
    ( (This)->lpVtbl -> SetCompletionStatus(This,dwResult) ) 

#define IESLicenseRenewalResultEvent_GetData(This,pbData)	\
    ( (This)->lpVtbl -> GetData(This,pbData) ) 

#define IESLicenseRenewalResultEvent_GetStringData(This,pbstrData)	\
    ( (This)->lpVtbl -> GetStringData(This,pbstrData) ) 


#define IESLicenseRenewalResultEvent_GetCallersId(This,pdwCallersId)	\
    ( (This)->lpVtbl -> GetCallersId(This,pdwCallersId) ) 

#define IESLicenseRenewalResultEvent_GetFileName(This,pbstrFilename)	\
    ( (This)->lpVtbl -> GetFileName(This,pbstrFilename) ) 

#define IESLicenseRenewalResultEvent_IsRenewalSuccessful(This,pfRenewalSuccessful)	\
    ( (This)->lpVtbl -> IsRenewalSuccessful(This,pfRenewalSuccessful) ) 

#define IESLicenseRenewalResultEvent_IsCheckEntitlementCallRequired(This,pfCheckEntTokenCallNeeded)	\
    ( (This)->lpVtbl -> IsCheckEntitlementCallRequired(This,pfCheckEntTokenCallNeeded) ) 

#define IESLicenseRenewalResultEvent_GetDescrambledStatus(This,pDescrambledStatus)	\
    ( (This)->lpVtbl -> GetDescrambledStatus(This,pDescrambledStatus) ) 

#define IESLicenseRenewalResultEvent_GetRenewalResultCode(This,pdwRenewalResultCode)	\
    ( (This)->lpVtbl -> GetRenewalResultCode(This,pdwRenewalResultCode) ) 

#define IESLicenseRenewalResultEvent_GetCASFailureCode(This,pdwCASFailureCode)	\
    ( (This)->lpVtbl -> GetCASFailureCode(This,pdwCASFailureCode) ) 

#define IESLicenseRenewalResultEvent_GetRenewalHResult(This,phr)	\
    ( (This)->lpVtbl -> GetRenewalHResult(This,phr) ) 

#define IESLicenseRenewalResultEvent_GetEntitlementTokenLength(This,pdwLength)	\
    ( (This)->lpVtbl -> GetEntitlementTokenLength(This,pdwLength) ) 

#define IESLicenseRenewalResultEvent_GetEntitlementToken(This,pbData)	\
    ( (This)->lpVtbl -> GetEntitlementToken(This,pbData) ) 

#define IESLicenseRenewalResultEvent_GetExpiryDate(This,pqwExpiryDate)	\
    ( (This)->lpVtbl -> GetExpiryDate(This,pqwExpiryDate) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IESLicenseRenewalResultEvent_INTERFACE_DEFINED__ */


#ifndef __IESFileExpiryDateEvent_INTERFACE_DEFINED__
#define __IESFileExpiryDateEvent_INTERFACE_DEFINED__

/* interface IESFileExpiryDateEvent */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IESFileExpiryDateEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BA9EDCB6-4D36-4cfe-8C56-87A6B0CA48E1")
    IESFileExpiryDateEvent : public IESEvent
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTunerId( 
            /* [retval][out] */ __RPC__out GUID *pguidTunerId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetExpiryDate( 
            /* [retval][out] */ __RPC__out DWORD64 *pqwExpiryDate) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFinalExpiryDate( 
            /* [retval][out] */ __RPC__out DWORD64 *pqwExpiryDate) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMaxRenewalCount( 
            /* [retval][out] */ __RPC__out DWORD *dwMaxRenewalCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsEntitlementTokenPresent( 
            /* [retval][out] */ __RPC__out BOOL *pfEntTokenPresent) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DoesExpireAfterFirstUse( 
            /* [retval][out] */ __RPC__out BOOL *pfExpireAfterFirstUse) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IESFileExpiryDateEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IESFileExpiryDateEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IESFileExpiryDateEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IESFileExpiryDateEvent * This);
        
        DECLSPEC_XFGVIRT(IESEvent, GetEventId)
        HRESULT ( STDMETHODCALLTYPE *GetEventId )( 
            __RPC__in IESFileExpiryDateEvent * This,
            /* [retval][out] */ __RPC__out DWORD *pdwEventId);
        
        DECLSPEC_XFGVIRT(IESEvent, GetEventType)
        HRESULT ( STDMETHODCALLTYPE *GetEventType )( 
            __RPC__in IESFileExpiryDateEvent * This,
            /* [retval][out] */ __RPC__out GUID *pguidEventType);
        
        DECLSPEC_XFGVIRT(IESEvent, SetCompletionStatus)
        HRESULT ( STDMETHODCALLTYPE *SetCompletionStatus )( 
            __RPC__in IESFileExpiryDateEvent * This,
            /* [in] */ DWORD dwResult);
        
        DECLSPEC_XFGVIRT(IESEvent, GetData)
        HRESULT ( STDMETHODCALLTYPE *GetData )( 
            __RPC__in IESFileExpiryDateEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pbData);
        
        DECLSPEC_XFGVIRT(IESEvent, GetStringData)
        HRESULT ( STDMETHODCALLTYPE *GetStringData )( 
            __RPC__in IESFileExpiryDateEvent * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrData);
        
        DECLSPEC_XFGVIRT(IESFileExpiryDateEvent, GetTunerId)
        HRESULT ( STDMETHODCALLTYPE *GetTunerId )( 
            __RPC__in IESFileExpiryDateEvent * This,
            /* [retval][out] */ __RPC__out GUID *pguidTunerId);
        
        DECLSPEC_XFGVIRT(IESFileExpiryDateEvent, GetExpiryDate)
        HRESULT ( STDMETHODCALLTYPE *GetExpiryDate )( 
            __RPC__in IESFileExpiryDateEvent * This,
            /* [retval][out] */ __RPC__out DWORD64 *pqwExpiryDate);
        
        DECLSPEC_XFGVIRT(IESFileExpiryDateEvent, GetFinalExpiryDate)
        HRESULT ( STDMETHODCALLTYPE *GetFinalExpiryDate )( 
            __RPC__in IESFileExpiryDateEvent * This,
            /* [retval][out] */ __RPC__out DWORD64 *pqwExpiryDate);
        
        DECLSPEC_XFGVIRT(IESFileExpiryDateEvent, GetMaxRenewalCount)
        HRESULT ( STDMETHODCALLTYPE *GetMaxRenewalCount )( 
            __RPC__in IESFileExpiryDateEvent * This,
            /* [retval][out] */ __RPC__out DWORD *dwMaxRenewalCount);
        
        DECLSPEC_XFGVIRT(IESFileExpiryDateEvent, IsEntitlementTokenPresent)
        HRESULT ( STDMETHODCALLTYPE *IsEntitlementTokenPresent )( 
            __RPC__in IESFileExpiryDateEvent * This,
            /* [retval][out] */ __RPC__out BOOL *pfEntTokenPresent);
        
        DECLSPEC_XFGVIRT(IESFileExpiryDateEvent, DoesExpireAfterFirstUse)
        HRESULT ( STDMETHODCALLTYPE *DoesExpireAfterFirstUse )( 
            __RPC__in IESFileExpiryDateEvent * This,
            /* [retval][out] */ __RPC__out BOOL *pfExpireAfterFirstUse);
        
        END_INTERFACE
    } IESFileExpiryDateEventVtbl;

    interface IESFileExpiryDateEvent
    {
        CONST_VTBL struct IESFileExpiryDateEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IESFileExpiryDateEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IESFileExpiryDateEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IESFileExpiryDateEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IESFileExpiryDateEvent_GetEventId(This,pdwEventId)	\
    ( (This)->lpVtbl -> GetEventId(This,pdwEventId) ) 

#define IESFileExpiryDateEvent_GetEventType(This,pguidEventType)	\
    ( (This)->lpVtbl -> GetEventType(This,pguidEventType) ) 

#define IESFileExpiryDateEvent_SetCompletionStatus(This,dwResult)	\
    ( (This)->lpVtbl -> SetCompletionStatus(This,dwResult) ) 

#define IESFileExpiryDateEvent_GetData(This,pbData)	\
    ( (This)->lpVtbl -> GetData(This,pbData) ) 

#define IESFileExpiryDateEvent_GetStringData(This,pbstrData)	\
    ( (This)->lpVtbl -> GetStringData(This,pbstrData) ) 


#define IESFileExpiryDateEvent_GetTunerId(This,pguidTunerId)	\
    ( (This)->lpVtbl -> GetTunerId(This,pguidTunerId) ) 

#define IESFileExpiryDateEvent_GetExpiryDate(This,pqwExpiryDate)	\
    ( (This)->lpVtbl -> GetExpiryDate(This,pqwExpiryDate) ) 

#define IESFileExpiryDateEvent_GetFinalExpiryDate(This,pqwExpiryDate)	\
    ( (This)->lpVtbl -> GetFinalExpiryDate(This,pqwExpiryDate) ) 

#define IESFileExpiryDateEvent_GetMaxRenewalCount(This,dwMaxRenewalCount)	\
    ( (This)->lpVtbl -> GetMaxRenewalCount(This,dwMaxRenewalCount) ) 

#define IESFileExpiryDateEvent_IsEntitlementTokenPresent(This,pfEntTokenPresent)	\
    ( (This)->lpVtbl -> IsEntitlementTokenPresent(This,pfEntTokenPresent) ) 

#define IESFileExpiryDateEvent_DoesExpireAfterFirstUse(This,pfExpireAfterFirstUse)	\
    ( (This)->lpVtbl -> DoesExpireAfterFirstUse(This,pfExpireAfterFirstUse) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IESFileExpiryDateEvent_INTERFACE_DEFINED__ */


#ifndef __IESEventService_INTERFACE_DEFINED__
#define __IESEventService_INTERFACE_DEFINED__

/* interface IESEventService */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IESEventService;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("ED89A619-4C06-4b2f-99EB-C7669B13047C")
    IESEventService : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE FireESEvent( 
            __RPC__in_opt IESEvent *pESEvent) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IESEventServiceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IESEventService * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IESEventService * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IESEventService * This);
        
        DECLSPEC_XFGVIRT(IESEventService, FireESEvent)
        HRESULT ( STDMETHODCALLTYPE *FireESEvent )( 
            __RPC__in IESEventService * This,
            __RPC__in_opt IESEvent *pESEvent);
        
        END_INTERFACE
    } IESEventServiceVtbl;

    interface IESEventService
    {
        CONST_VTBL struct IESEventServiceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IESEventService_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IESEventService_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IESEventService_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IESEventService_FireESEvent(This,pESEvent)	\
    ( (This)->lpVtbl -> FireESEvent(This,pESEvent) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IESEventService_INTERFACE_DEFINED__ */


#ifndef __IESEventServiceConfiguration_INTERFACE_DEFINED__
#define __IESEventServiceConfiguration_INTERFACE_DEFINED__

/* interface IESEventServiceConfiguration */
/* [unique][helpstring][uuid][nonextensible][hidden][object] */ 


EXTERN_C const IID IID_IESEventServiceConfiguration;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("33B9DAAE-9309-491d-A051-BCAD2A70CD66")
    IESEventServiceConfiguration : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetParent( 
            __RPC__in_opt IESEventService *pEventService) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveParent( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetOwner( 
            __RPC__in_opt IESEvents *pESEvents) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveOwner( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetGraph( 
            __RPC__in_opt IFilterGraph *pGraph) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveGraph( 
            __RPC__in_opt IFilterGraph *pGraph) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IESEventServiceConfigurationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IESEventServiceConfiguration * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IESEventServiceConfiguration * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IESEventServiceConfiguration * This);
        
        DECLSPEC_XFGVIRT(IESEventServiceConfiguration, SetParent)
        HRESULT ( STDMETHODCALLTYPE *SetParent )( 
            __RPC__in IESEventServiceConfiguration * This,
            __RPC__in_opt IESEventService *pEventService);
        
        DECLSPEC_XFGVIRT(IESEventServiceConfiguration, RemoveParent)
        HRESULT ( STDMETHODCALLTYPE *RemoveParent )( 
            __RPC__in IESEventServiceConfiguration * This);
        
        DECLSPEC_XFGVIRT(IESEventServiceConfiguration, SetOwner)
        HRESULT ( STDMETHODCALLTYPE *SetOwner )( 
            __RPC__in IESEventServiceConfiguration * This,
            __RPC__in_opt IESEvents *pESEvents);
        
        DECLSPEC_XFGVIRT(IESEventServiceConfiguration, RemoveOwner)
        HRESULT ( STDMETHODCALLTYPE *RemoveOwner )( 
            __RPC__in IESEventServiceConfiguration * This);
        
        DECLSPEC_XFGVIRT(IESEventServiceConfiguration, SetGraph)
        HRESULT ( STDMETHODCALLTYPE *SetGraph )( 
            __RPC__in IESEventServiceConfiguration * This,
            __RPC__in_opt IFilterGraph *pGraph);
        
        DECLSPEC_XFGVIRT(IESEventServiceConfiguration, RemoveGraph)
        HRESULT ( STDMETHODCALLTYPE *RemoveGraph )( 
            __RPC__in IESEventServiceConfiguration * This,
            __RPC__in_opt IFilterGraph *pGraph);
        
        END_INTERFACE
    } IESEventServiceConfigurationVtbl;

    interface IESEventServiceConfiguration
    {
        CONST_VTBL struct IESEventServiceConfigurationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IESEventServiceConfiguration_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IESEventServiceConfiguration_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IESEventServiceConfiguration_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IESEventServiceConfiguration_SetParent(This,pEventService)	\
    ( (This)->lpVtbl -> SetParent(This,pEventService) ) 

#define IESEventServiceConfiguration_RemoveParent(This)	\
    ( (This)->lpVtbl -> RemoveParent(This) ) 

#define IESEventServiceConfiguration_SetOwner(This,pESEvents)	\
    ( (This)->lpVtbl -> SetOwner(This,pESEvents) ) 

#define IESEventServiceConfiguration_RemoveOwner(This)	\
    ( (This)->lpVtbl -> RemoveOwner(This) ) 

#define IESEventServiceConfiguration_SetGraph(This,pGraph)	\
    ( (This)->lpVtbl -> SetGraph(This,pGraph) ) 

#define IESEventServiceConfiguration_RemoveGraph(This,pGraph)	\
    ( (This)->lpVtbl -> RemoveGraph(This,pGraph) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IESEventServiceConfiguration_INTERFACE_DEFINED__ */


#ifndef __IRegisterTuner_INTERFACE_DEFINED__
#define __IRegisterTuner_INTERFACE_DEFINED__

/* interface IRegisterTuner */
/* [unique][helpstring][uuid][nonextensible][hidden][object] */ 


EXTERN_C const IID IID_IRegisterTuner;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("359B3901-572C-4854-BB49-CDEF66606A25")
    IRegisterTuner : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Register( 
            /* [in] */ __RPC__in_opt ITuner *pTuner,
            /* [in] */ __RPC__in_opt IGraphBuilder *pGraph) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Unregister( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRegisterTunerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRegisterTuner * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRegisterTuner * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRegisterTuner * This);
        
        DECLSPEC_XFGVIRT(IRegisterTuner, Register)
        HRESULT ( STDMETHODCALLTYPE *Register )( 
            __RPC__in IRegisterTuner * This,
            /* [in] */ __RPC__in_opt ITuner *pTuner,
            /* [in] */ __RPC__in_opt IGraphBuilder *pGraph);
        
        DECLSPEC_XFGVIRT(IRegisterTuner, Unregister)
        HRESULT ( STDMETHODCALLTYPE *Unregister )( 
            __RPC__in IRegisterTuner * This);
        
        END_INTERFACE
    } IRegisterTunerVtbl;

    interface IRegisterTuner
    {
        CONST_VTBL struct IRegisterTunerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRegisterTuner_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRegisterTuner_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRegisterTuner_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRegisterTuner_Register(This,pTuner,pGraph)	\
    ( (This)->lpVtbl -> Register(This,pTuner,pGraph) ) 

#define IRegisterTuner_Unregister(This)	\
    ( (This)->lpVtbl -> Unregister(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRegisterTuner_INTERFACE_DEFINED__ */


#ifndef __IBDAComparable_INTERFACE_DEFINED__
#define __IBDAComparable_INTERFACE_DEFINED__

/* interface IBDAComparable */
/* [unique][helpstring][uuid][nonextensible][hidden][object] */ 


EXTERN_C const IID IID_IBDAComparable;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B34505E0-2F0E-497b-80BC-D43F3B24ED7F")
    IBDAComparable : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE CompareExact( 
            /* [in] */ __RPC__in_opt IDispatch *CompareTo,
            /* [retval][out] */ __RPC__out long *Result) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE CompareEquivalent( 
            /* [in] */ __RPC__in_opt IDispatch *CompareTo,
            /* [in] */ DWORD dwFlags,
            /* [retval][out] */ __RPC__out long *Result) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE HashExact( 
            /* [retval][out] */ __RPC__out __int64 *Result) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE HashExactIncremental( 
            /* [in] */ __int64 PartialResult,
            /* [retval][out] */ __RPC__out __int64 *Result) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE HashEquivalent( 
            /* [in] */ DWORD dwFlags,
            /* [retval][out] */ __RPC__out __int64 *Result) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE HashEquivalentIncremental( 
            /* [in] */ __int64 PartialResult,
            /* [in] */ DWORD dwFlags,
            /* [retval][out] */ __RPC__out __int64 *Result) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBDAComparableVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBDAComparable * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBDAComparable * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBDAComparable * This);
        
        DECLSPEC_XFGVIRT(IBDAComparable, CompareExact)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CompareExact )( 
            __RPC__in IBDAComparable * This,
            /* [in] */ __RPC__in_opt IDispatch *CompareTo,
            /* [retval][out] */ __RPC__out long *Result);
        
        DECLSPEC_XFGVIRT(IBDAComparable, CompareEquivalent)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CompareEquivalent )( 
            __RPC__in IBDAComparable * This,
            /* [in] */ __RPC__in_opt IDispatch *CompareTo,
            /* [in] */ DWORD dwFlags,
            /* [retval][out] */ __RPC__out long *Result);
        
        DECLSPEC_XFGVIRT(IBDAComparable, HashExact)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *HashExact )( 
            __RPC__in IBDAComparable * This,
            /* [retval][out] */ __RPC__out __int64 *Result);
        
        DECLSPEC_XFGVIRT(IBDAComparable, HashExactIncremental)
        HRESULT ( STDMETHODCALLTYPE *HashExactIncremental )( 
            __RPC__in IBDAComparable * This,
            /* [in] */ __int64 PartialResult,
            /* [retval][out] */ __RPC__out __int64 *Result);
        
        DECLSPEC_XFGVIRT(IBDAComparable, HashEquivalent)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *HashEquivalent )( 
            __RPC__in IBDAComparable * This,
            /* [in] */ DWORD dwFlags,
            /* [retval][out] */ __RPC__out __int64 *Result);
        
        DECLSPEC_XFGVIRT(IBDAComparable, HashEquivalentIncremental)
        HRESULT ( STDMETHODCALLTYPE *HashEquivalentIncremental )( 
            __RPC__in IBDAComparable * This,
            /* [in] */ __int64 PartialResult,
            /* [in] */ DWORD dwFlags,
            /* [retval][out] */ __RPC__out __int64 *Result);
        
        END_INTERFACE
    } IBDAComparableVtbl;

    interface IBDAComparable
    {
        CONST_VTBL struct IBDAComparableVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBDAComparable_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBDAComparable_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBDAComparable_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBDAComparable_CompareExact(This,CompareTo,Result)	\
    ( (This)->lpVtbl -> CompareExact(This,CompareTo,Result) ) 

#define IBDAComparable_CompareEquivalent(This,CompareTo,dwFlags,Result)	\
    ( (This)->lpVtbl -> CompareEquivalent(This,CompareTo,dwFlags,Result) ) 

#define IBDAComparable_HashExact(This,Result)	\
    ( (This)->lpVtbl -> HashExact(This,Result) ) 

#define IBDAComparable_HashExactIncremental(This,PartialResult,Result)	\
    ( (This)->lpVtbl -> HashExactIncremental(This,PartialResult,Result) ) 

#define IBDAComparable_HashEquivalent(This,dwFlags,Result)	\
    ( (This)->lpVtbl -> HashEquivalent(This,dwFlags,Result) ) 

#define IBDAComparable_HashEquivalentIncremental(This,PartialResult,dwFlags,Result)	\
    ( (This)->lpVtbl -> HashEquivalentIncremental(This,PartialResult,dwFlags,Result) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBDAComparable_INTERFACE_DEFINED__ */


#ifndef __IPersistTuneXml_INTERFACE_DEFINED__
#define __IPersistTuneXml_INTERFACE_DEFINED__

/* interface IPersistTuneXml */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IPersistTuneXml;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0754CD31-8D15-47a9-8215-D20064157244")
    IPersistTuneXml : public IPersist
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE InitNew( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Load( 
            /* [in] */ VARIANT varValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Save( 
            /* [out] */ __RPC__out VARIANT *pvarFragment) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPersistTuneXmlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPersistTuneXml * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPersistTuneXml * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPersistTuneXml * This);
        
        DECLSPEC_XFGVIRT(IPersist, GetClassID)
        HRESULT ( STDMETHODCALLTYPE *GetClassID )( 
            __RPC__in IPersistTuneXml * This,
            /* [out] */ __RPC__out CLSID *pClassID);
        
        DECLSPEC_XFGVIRT(IPersistTuneXml, InitNew)
        HRESULT ( STDMETHODCALLTYPE *InitNew )( 
            __RPC__in IPersistTuneXml * This);
        
        DECLSPEC_XFGVIRT(IPersistTuneXml, Load)
        HRESULT ( STDMETHODCALLTYPE *Load )( 
            __RPC__in IPersistTuneXml * This,
            /* [in] */ VARIANT varValue);
        
        DECLSPEC_XFGVIRT(IPersistTuneXml, Save)
        HRESULT ( STDMETHODCALLTYPE *Save )( 
            __RPC__in IPersistTuneXml * This,
            /* [out] */ __RPC__out VARIANT *pvarFragment);
        
        END_INTERFACE
    } IPersistTuneXmlVtbl;

    interface IPersistTuneXml
    {
        CONST_VTBL struct IPersistTuneXmlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPersistTuneXml_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPersistTuneXml_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPersistTuneXml_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPersistTuneXml_GetClassID(This,pClassID)	\
    ( (This)->lpVtbl -> GetClassID(This,pClassID) ) 


#define IPersistTuneXml_InitNew(This)	\
    ( (This)->lpVtbl -> InitNew(This) ) 

#define IPersistTuneXml_Load(This,varValue)	\
    ( (This)->lpVtbl -> Load(This,varValue) ) 

#define IPersistTuneXml_Save(This,pvarFragment)	\
    ( (This)->lpVtbl -> Save(This,pvarFragment) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPersistTuneXml_INTERFACE_DEFINED__ */


#ifndef __IPersistTuneXmlUtility_INTERFACE_DEFINED__
#define __IPersistTuneXmlUtility_INTERFACE_DEFINED__

/* interface IPersistTuneXmlUtility */
/* [unique][helpstring][uuid][nonextensible][hidden][object] */ 


EXTERN_C const IID IID_IPersistTuneXmlUtility;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("990237AE-AC11-4614-BE8F-DD217A4CB4CB")
    IPersistTuneXmlUtility : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Deserialize( 
            /* [in] */ VARIANT varValue,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppObject) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPersistTuneXmlUtilityVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPersistTuneXmlUtility * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPersistTuneXmlUtility * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPersistTuneXmlUtility * This);
        
        DECLSPEC_XFGVIRT(IPersistTuneXmlUtility, Deserialize)
        HRESULT ( STDMETHODCALLTYPE *Deserialize )( 
            __RPC__in IPersistTuneXmlUtility * This,
            /* [in] */ VARIANT varValue,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppObject);
        
        END_INTERFACE
    } IPersistTuneXmlUtilityVtbl;

    interface IPersistTuneXmlUtility
    {
        CONST_VTBL struct IPersistTuneXmlUtilityVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPersistTuneXmlUtility_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPersistTuneXmlUtility_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPersistTuneXmlUtility_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPersistTuneXmlUtility_Deserialize(This,varValue,ppObject)	\
    ( (This)->lpVtbl -> Deserialize(This,varValue,ppObject) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPersistTuneXmlUtility_INTERFACE_DEFINED__ */


#ifndef __IPersistTuneXmlUtility2_INTERFACE_DEFINED__
#define __IPersistTuneXmlUtility2_INTERFACE_DEFINED__

/* interface IPersistTuneXmlUtility2 */
/* [unique][helpstring][uuid][nonextensible][hidden][object] */ 


EXTERN_C const IID IID_IPersistTuneXmlUtility2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("992E165F-EA24-4b2f-9A1D-009D92120451")
    IPersistTuneXmlUtility2 : public IPersistTuneXmlUtility
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Serialize( 
            /* [in] */ __RPC__in_opt ITuneRequest *piTuneRequest,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pString) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPersistTuneXmlUtility2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPersistTuneXmlUtility2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPersistTuneXmlUtility2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPersistTuneXmlUtility2 * This);
        
        DECLSPEC_XFGVIRT(IPersistTuneXmlUtility, Deserialize)
        HRESULT ( STDMETHODCALLTYPE *Deserialize )( 
            __RPC__in IPersistTuneXmlUtility2 * This,
            /* [in] */ VARIANT varValue,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppObject);
        
        DECLSPEC_XFGVIRT(IPersistTuneXmlUtility2, Serialize)
        HRESULT ( STDMETHODCALLTYPE *Serialize )( 
            __RPC__in IPersistTuneXmlUtility2 * This,
            /* [in] */ __RPC__in_opt ITuneRequest *piTuneRequest,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pString);
        
        END_INTERFACE
    } IPersistTuneXmlUtility2Vtbl;

    interface IPersistTuneXmlUtility2
    {
        CONST_VTBL struct IPersistTuneXmlUtility2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPersistTuneXmlUtility2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPersistTuneXmlUtility2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPersistTuneXmlUtility2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPersistTuneXmlUtility2_Deserialize(This,varValue,ppObject)	\
    ( (This)->lpVtbl -> Deserialize(This,varValue,ppObject) ) 


#define IPersistTuneXmlUtility2_Serialize(This,piTuneRequest,pString)	\
    ( (This)->lpVtbl -> Serialize(This,piTuneRequest,pString) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPersistTuneXmlUtility2_INTERFACE_DEFINED__ */


#ifndef __IBDACreateTuneRequestEx_INTERFACE_DEFINED__
#define __IBDACreateTuneRequestEx_INTERFACE_DEFINED__

/* interface IBDACreateTuneRequestEx */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IBDACreateTuneRequestEx;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C0A4A1D4-2B3C-491a-BA22-499FBADD4D12")
    IBDACreateTuneRequestEx : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateTuneRequestEx( 
            /* [in] */ __RPC__in REFCLSID TuneRequestIID,
            /* [retval][out] */ __RPC__deref_out_opt ITuneRequest **TuneRequest) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBDACreateTuneRequestExVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IBDACreateTuneRequestEx * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IBDACreateTuneRequestEx * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IBDACreateTuneRequestEx * This);
        
        DECLSPEC_XFGVIRT(IBDACreateTuneRequestEx, CreateTuneRequestEx)
        HRESULT ( STDMETHODCALLTYPE *CreateTuneRequestEx )( 
            __RPC__in IBDACreateTuneRequestEx * This,
            /* [in] */ __RPC__in REFCLSID TuneRequestIID,
            /* [retval][out] */ __RPC__deref_out_opt ITuneRequest **TuneRequest);
        
        END_INTERFACE
    } IBDACreateTuneRequestExVtbl;

    interface IBDACreateTuneRequestEx
    {
        CONST_VTBL struct IBDACreateTuneRequestExVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBDACreateTuneRequestEx_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBDACreateTuneRequestEx_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBDACreateTuneRequestEx_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBDACreateTuneRequestEx_CreateTuneRequestEx(This,TuneRequestIID,TuneRequest)	\
    ( (This)->lpVtbl -> CreateTuneRequestEx(This,TuneRequestIID,TuneRequest) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBDACreateTuneRequestEx_INTERFACE_DEFINED__ */



#ifndef __TunerLib_LIBRARY_DEFINED__
#define __TunerLib_LIBRARY_DEFINED__

/* library TunerLib */
/* [helpstring][version][uuid] */ 

#define SID_ESEventService CLSID_ESEventService
#define SID_ESEventFactory CLSID_ESEventFactory
#define SID_SContentTuneRequest IID_ITuner
#define SID_ScanningTuner IID_IScanningTuner
#define SID_ScanningTunerEx IID_IScanningTunerEx

EXTERN_C const IID LIBID_TunerLib;

EXTERN_C const CLSID CLSID_SystemTuningSpaces;

#ifdef __cplusplus

class DECLSPEC_UUID("D02AAC50-027E-11d3-9D8E-00C04F72D980")
SystemTuningSpaces;
#endif

EXTERN_C const CLSID CLSID_TuningSpace;

#ifdef __cplusplus

class DECLSPEC_UUID("5FFDC5E6-B83A-4b55-B6E8-C69E765FE9DB")
TuningSpace;
#endif

EXTERN_C const CLSID CLSID_ChannelIDTuningSpace;

#ifdef __cplusplus

class DECLSPEC_UUID("CC829A2F-3365-463f-AF13-81DBB6F3A555")
ChannelIDTuningSpace;
#endif

EXTERN_C const CLSID CLSID_ATSCTuningSpace;

#ifdef __cplusplus

class DECLSPEC_UUID("A2E30750-6C3D-11d3-B653-00C04F79498E")
ATSCTuningSpace;
#endif

EXTERN_C const CLSID CLSID_DigitalCableTuningSpace;

#ifdef __cplusplus

class DECLSPEC_UUID("D9BB4CEE-B87A-47F1-AC92-B08D9C7813FC")
DigitalCableTuningSpace;
#endif

EXTERN_C const CLSID CLSID_AnalogRadioTuningSpace;

#ifdef __cplusplus

class DECLSPEC_UUID("8A674B4C-1F63-11d3-B64C-00C04F79498E")
AnalogRadioTuningSpace;
#endif

EXTERN_C const CLSID CLSID_AuxInTuningSpace;

#ifdef __cplusplus

class DECLSPEC_UUID("F9769A06-7ACA-4e39-9CFB-97BB35F0E77E")
AuxInTuningSpace;
#endif

EXTERN_C const CLSID CLSID_AnalogTVTuningSpace;

#ifdef __cplusplus

class DECLSPEC_UUID("8A674B4D-1F63-11d3-B64C-00C04F79498E")
AnalogTVTuningSpace;
#endif

EXTERN_C const CLSID CLSID_DVBTuningSpace;

#ifdef __cplusplus

class DECLSPEC_UUID("C6B14B32-76AA-4a86-A7AC-5C79AAF58DA7")
DVBTuningSpace;
#endif

EXTERN_C const CLSID CLSID_DVBSTuningSpace;

#ifdef __cplusplus

class DECLSPEC_UUID("B64016F3-C9A2-4066-96F0-BD9563314726")
DVBSTuningSpace;
#endif

EXTERN_C const CLSID CLSID_ComponentTypes;

#ifdef __cplusplus

class DECLSPEC_UUID("A1A2B1C4-0E3A-11d3-9D8E-00C04F72D980")
ComponentTypes;
#endif

EXTERN_C const CLSID CLSID_ComponentType;

#ifdef __cplusplus

class DECLSPEC_UUID("823535A0-0318-11d3-9D8E-00C04F72D980")
ComponentType;
#endif

EXTERN_C const CLSID CLSID_LanguageComponentType;

#ifdef __cplusplus

class DECLSPEC_UUID("1BE49F30-0E1B-11d3-9D8E-00C04F72D980")
LanguageComponentType;
#endif

EXTERN_C const CLSID CLSID_MPEG2ComponentType;

#ifdef __cplusplus

class DECLSPEC_UUID("418008F3-CF67-4668-9628-10DC52BE1D08")
MPEG2ComponentType;
#endif

EXTERN_C const CLSID CLSID_ATSCComponentType;

#ifdef __cplusplus

class DECLSPEC_UUID("A8DCF3D5-0780-4ef4-8A83-2CFFAACB8ACE")
ATSCComponentType;
#endif

EXTERN_C const CLSID CLSID_Components;

#ifdef __cplusplus

class DECLSPEC_UUID("809B6661-94C4-49e6-B6EC-3F0F862215AA")
Components;
#endif

EXTERN_C const CLSID CLSID_Component;

#ifdef __cplusplus

class DECLSPEC_UUID("59DC47A8-116C-11d3-9D8E-00C04F72D980")
Component;
#endif

EXTERN_C const CLSID CLSID_MPEG2Component;

#ifdef __cplusplus

class DECLSPEC_UUID("055CB2D7-2969-45cd-914B-76890722F112")
MPEG2Component;
#endif

EXTERN_C const CLSID CLSID_AnalogAudioComponentType;

#ifdef __cplusplus

class DECLSPEC_UUID("28AB0005-E845-4FFA-AA9B-F4665236141C")
AnalogAudioComponentType;
#endif

EXTERN_C const CLSID CLSID_TuneRequest;

#ifdef __cplusplus

class DECLSPEC_UUID("B46E0D38-AB35-4a06-A137-70576B01B39F")
TuneRequest;
#endif

EXTERN_C const CLSID CLSID_ChannelIDTuneRequest;

#ifdef __cplusplus

class DECLSPEC_UUID("3A9428A7-31A4-45e9-9EFB-E055BF7BB3DB")
ChannelIDTuneRequest;
#endif

EXTERN_C const CLSID CLSID_ChannelTuneRequest;

#ifdef __cplusplus

class DECLSPEC_UUID("0369B4E5-45B6-11d3-B650-00C04F79498E")
ChannelTuneRequest;
#endif

EXTERN_C const CLSID CLSID_ATSCChannelTuneRequest;

#ifdef __cplusplus

class DECLSPEC_UUID("0369B4E6-45B6-11d3-B650-00C04F79498E")
ATSCChannelTuneRequest;
#endif

EXTERN_C const CLSID CLSID_DigitalCableTuneRequest;

#ifdef __cplusplus

class DECLSPEC_UUID("26EC0B63-AA90-458A-8DF4-5659F2C8A18A")
DigitalCableTuneRequest;
#endif

EXTERN_C const CLSID CLSID_MPEG2TuneRequest;

#ifdef __cplusplus

class DECLSPEC_UUID("0955AC62-BF2E-4cba-A2B9-A63F772D46CF")
MPEG2TuneRequest;
#endif

EXTERN_C const CLSID CLSID_MPEG2TuneRequestFactory;

#ifdef __cplusplus

class DECLSPEC_UUID("2C63E4EB-4CEA-41b8-919C-E947EA19A77C")
MPEG2TuneRequestFactory;
#endif

EXTERN_C const CLSID CLSID_Locator;

#ifdef __cplusplus

class DECLSPEC_UUID("0888C883-AC4F-4943-B516-2C38D9B34562")
Locator;
#endif

EXTERN_C const CLSID CLSID_DigitalLocator;

#ifdef __cplusplus

class DECLSPEC_UUID("6E50CC0D-C19B-4BF6-810B-5BD60761F5CC")
DigitalLocator;
#endif

EXTERN_C const CLSID CLSID_AnalogLocator;

#ifdef __cplusplus

class DECLSPEC_UUID("49638B91-48AB-48B7-A47A-7D0E75A08EDE")
AnalogLocator;
#endif

EXTERN_C const CLSID CLSID_ATSCLocator;

#ifdef __cplusplus

class DECLSPEC_UUID("8872FF1B-98FA-4d7a-8D93-C9F1055F85BB")
ATSCLocator;
#endif

EXTERN_C const CLSID CLSID_DigitalCableLocator;

#ifdef __cplusplus

class DECLSPEC_UUID("03C06416-D127-407A-AB4C-FDD279ABBE5D")
DigitalCableLocator;
#endif

EXTERN_C const CLSID CLSID_DVBTLocator;

#ifdef __cplusplus

class DECLSPEC_UUID("9CD64701-BDF3-4d14-8E03-F12983D86664")
DVBTLocator;
#endif

EXTERN_C const CLSID CLSID_DVBTLocator2;

#ifdef __cplusplus

class DECLSPEC_UUID("EFE3FA02-45D7-4920-BE96-53FA7F35B0E6")
DVBTLocator2;
#endif

EXTERN_C const CLSID CLSID_DVBSLocator;

#ifdef __cplusplus

class DECLSPEC_UUID("1DF7D126-4050-47f0-A7CF-4C4CA9241333")
DVBSLocator;
#endif

EXTERN_C const CLSID CLSID_DVBCLocator;

#ifdef __cplusplus

class DECLSPEC_UUID("C531D9FD-9685-4028-8B68-6E1232079F1E")
DVBCLocator;
#endif

EXTERN_C const CLSID CLSID_ISDBSLocator;

#ifdef __cplusplus

class DECLSPEC_UUID("6504AFED-A629-455c-A7F1-04964DEA5CC4")
ISDBSLocator;
#endif

EXTERN_C const CLSID CLSID_DVBTuneRequest;

#ifdef __cplusplus

class DECLSPEC_UUID("15D6504A-5494-499c-886C-973C9E53B9F1")
DVBTuneRequest;
#endif

EXTERN_C const CLSID CLSID_CreatePropBagOnRegKey;

#ifdef __cplusplus

class DECLSPEC_UUID("8A674B49-1F63-11d3-B64C-00C04F79498E")
CreatePropBagOnRegKey;
#endif

EXTERN_C const CLSID CLSID_BroadcastEventService;

#ifdef __cplusplus

class DECLSPEC_UUID("0B3FFB92-0919-4934-9D5B-619C719D0202")
BroadcastEventService;
#endif

EXTERN_C const CLSID CLSID_TunerMarshaler;

#ifdef __cplusplus

class DECLSPEC_UUID("6438570B-0C08-4a25-9504-8012BB4D50CF")
TunerMarshaler;
#endif

EXTERN_C const CLSID CLSID_PersistTuneXmlUtility;

#ifdef __cplusplus

class DECLSPEC_UUID("E77026B0-B97F-4cbb-B7FB-F4F03AD69F11")
PersistTuneXmlUtility;
#endif

EXTERN_C const CLSID CLSID_ESEventService;

#ifdef __cplusplus

class DECLSPEC_UUID("C20447FC-EC60-475e-813F-D2B0A6DECEFE")
ESEventService;
#endif

EXTERN_C const CLSID CLSID_ESEventFactory;

#ifdef __cplusplus

class DECLSPEC_UUID("8E8A07DA-71F8-40c1-A929-5E3A868AC2C6")
ESEventFactory;
#endif
#endif /* __TunerLib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_tuner_0000_0070 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_tuner_0000_0070_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_tuner_0000_0070_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  LPSAFEARRAY_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out LPSAFEARRAY * ); 
void                      __RPC_USER  LPSAFEARRAY_UserFree(     __RPC__in unsigned long *, __RPC__in LPSAFEARRAY * ); 

unsigned long             __RPC_USER  VARIANT_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

unsigned long             __RPC_USER  BSTR_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree64(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  LPSAFEARRAY_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out LPSAFEARRAY * ); 
void                      __RPC_USER  LPSAFEARRAY_UserFree64(     __RPC__in unsigned long *, __RPC__in LPSAFEARRAY * ); 

unsigned long             __RPC_USER  VARIANT_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree64(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif



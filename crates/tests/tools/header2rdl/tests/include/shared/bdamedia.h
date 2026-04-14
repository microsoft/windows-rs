//------------------------------------------------------------------------------
// File: BDAMedia.h
//
// Desc: Broadcast Driver Architecture Multimedia Definitions.
//
// Copyright (c) 1996 - 2001, Microsoft Corporation.  All rights reserved.
//------------------------------------------------------------------------------


#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#if !defined(_KSMEDIA_)
#error KSMEDIA.H must be included before BDAMEDIA.H
#endif // !defined(_KSMEDIA_)

#if !defined(_BDATYPES_)
#error BDATYPES.H must be included before BDAMEDIA.H
#endif // !defined(_BDATYPES_)

#if !defined(_BDAMEDIA_)
#define _BDAMEDIA_

#if defined(__cplusplus)
extern "C" {
#endif // defined(__cplusplus)



//===========================================================================
//
//  KSProperty Set Structure Definitions for BDA
//
//===========================================================================

typedef struct _KSP_BDA_NODE_PIN {
    KSPROPERTY      Property;
    ULONG           ulNodeType;
    ULONG           ulInputPinId;
    ULONG           ulOutputPinId;
} KSP_BDA_NODE_PIN, *PKSP_BDA_NODE_PIN;


typedef struct _KSM_BDA_PIN
{
    KSMETHOD    Method;
    union
    {
        ULONG       PinId;
        ULONG       PinType;
    };
    ULONG       Reserved;
} KSM_BDA_PIN, * PKSM_BDA_PIN;


typedef struct _KSM_BDA_PIN_PAIR
{
    KSMETHOD    Method;
    union
    {
        ULONG       InputPinId;
        ULONG       InputPinType;
    };
    union
    {
        ULONG       OutputPinId;
        ULONG       OutputPinType;
    };
} KSM_BDA_PIN_PAIR, * PKSM_BDA_PIN_PAIR;


typedef struct {
    KSP_NODE        Property;
    ULONG           EsPid;
} KSP_NODE_ESPID, *PKSP_NODE_ESPID;

typedef struct _KSM_BDA_DEBUG_LEVEL 
{
    KSMETHOD    Method;
    UCHAR       ucDebugLevel;
    ULONG       ulDebugStringSize;
    BYTE        argbDebugString [MIN_DIMENSION];
} KSM_BDA_DEBUG_LEVEL, * PKSM_BDA_DEBUG_LEVEL;

typedef struct _BDA_DEBUG_DATA 
{
    PBDARESULT  lResult;
    GUID        uuidDebugDataType;
    ULONG       ulDataSize;
    BYTE        argbDebugData [MIN_DIMENSION];
} BDA_DEBUG_DATA;

typedef struct _BDA_EVENT_DATA 
{
    PBDARESULT  lResult;
    ULONG       ulEventID;
    GUID        uuidEventType;
    ULONG       ulEventDataLength;    
    BYTE        argbEventData [MIN_DIMENSION];
} BDA_EVENT_DATA, * PBDA_EVENT_DATA;

typedef struct _KSM_BDA_EVENT_COMPLETE {
    KSMETHOD Method;
    ULONG   ulEventID;
    ULONG   ulEventResult;
} KSM_BDA_EVENT_COMPLETE, * PKSM_BDA_EVENT_COMPLETE;

//===========================================================================
//
//  KSMethod Set Structure Definitions for DRM, WMDRM, WMDRMTUNER 
//
//===========================================================================

typedef struct _KSM_BDA_DRM_SETDRM 
{
    KSM_NODE    NodeMethod;
    GUID        NewDRMuuid;
} KSM_BDA_DRM_SETDRM, * PKSM_BDA_DRM_SETDRM;

typedef struct _KSM_BDA_BUFFER 
{
    KSM_NODE    NodeMethod;
    ULONG       ulBufferSize;
    BYTE        argbBuffer[MIN_DIMENSION];
} KSM_BDA_BUFFER, *PKSM_BDA_BUFFER;

typedef struct KSM_BDA_WMDRM_LICENSE 
{
    KSM_NODE    NodeMethod;
    GUID        uuidKeyID;
} KSM_BDA_WMDRM_LICENSE, *PKSM_BDA_WMDRM_LICENSE;

typedef struct _KSM_BDA_WMDRM_RENEWLICENSE 
{
    KSM_NODE    NodeMethod;
    ULONG       ulXMRLicenseLength; 
    ULONG       ulEntitlementTokenLength;
    BYTE        argbDataBuffer[MIN_DIMENSION]; //License and Entitlement Token Buffer 
} KSM_BDA_WMDRM_RENEWLICENSE, *PKSM_BDA_WMDRM_RENEWLICENSE;

typedef struct _KSM_BDA_WMDRMTUNER_PURCHASEENTITLEMENT 
{
    KSM_NODE    NodeMethod;
    ULONG       ulDialogRequest;
    CHAR        cLanguage[12];
    ULONG       ulPurchaseTokenLength;
    BYTE        argbDataBuffer[MIN_DIMENSION]; //Language Buffer before PurchaseToken
} KSM_BDA_WMDRMTUNER_PURCHASEENTITLEMENT, *PKSM_BDA_WMDRMTUNER_PURCHASEENTITLEMENT;

typedef struct _KSM_BDA_WMDRMTUNER_SETPIDPROTECTION 
{
    KSM_NODE    NodeMethod;
    ULONG       ulPID; 
    GUID        uuidKeyID;  
} KSM_BDA_WMDRMTUNER_SETPIDPROTECTION, *PKSM_BDA_WMDRMTUNER_SETPIDPROTECTION;

typedef struct _KSM_BDA_WMDRMTUNER_GETPIDPROTECTION 
{
    KSM_NODE    NodeMethod;
    ULONG       ulPID; 
} KSM_BDA_WMDRMTUNER_GETPIDPROTECTION, *PKSM_BDA_WMDRMTUNER_GETPIDPROTECTION;

typedef struct _KSM_BDA_WMDRMTUNER_SYNCVALUE 
{
    KSM_NODE    NodeMethod;
    ULONG       ulSyncValue;
} KSM_BDA_WMDRMTUNER_SYNCVALUE, *PKSM_BDA_WMDRMTUNER_SYNCVALUE;


//===========================================================================
//
//  KSMethod Set Structure Definitions for PBDA TUNER  
//
//===========================================================================

typedef struct _KSM_BDA_TUNER_TUNEREQUEST 
{
    KSMETHOD    Method;
    ULONG       ulTuneLength; 
    BYTE        argbTuneData [MIN_DIMENSION];
} KSM_BDA_TUNER_TUNEREQUEST, *PKSM_BDA_TUNER_TUNEREQUEST;

//===========================================================================
//
//  KSMethod Set Structure Definitions for PBDA GENERAL PURPOSE NAME VALUES  
//
//===========================================================================

typedef struct _KSM_BDA_GPNV_GETVALUE 
{
    KSMETHOD    Method;
    ULONG       ulNameLength;
    CHAR        cLanguage[12];
    BYTE        argbData [MIN_DIMENSION]; 
} KSM_BDA_GPNV_GETVALUE, *PKSM_BDA_GPNV_GETVALUE;

typedef struct _KSM_BDA_GPNV_SETVALUE 
{
    KSMETHOD    Method;
    ULONG       ulDialogRequest; 
    CHAR        cLanguage[12];
    ULONG       ulNameLength;
    ULONG       ulValueLength;
    BYTE        argbName [MIN_DIMENSION]; 
} KSM_BDA_GPNV_SETVALUE, *PKSM_BDA_GPNV_SETVALUE;

typedef struct _KSM_BDA_GPNV_NAMEINDEX 
{
    KSMETHOD    Method;
    ULONG       ulValueNameIndex; 
} KSM_BDA_GPNV_NAMEINDEX, *PKSM_BDA_GPNV_NAMEINDEX;

//===========================================================================
//
//  KSMethod Set Structure Definitions for PBDA SCANNING  
//
//===========================================================================
typedef struct _KSM_BDA_SCAN_CAPABILTIES 
{
    KSMETHOD    Method;
    GUID        uuidBroadcastStandard;
} KSM_BDA_SCAN_CAPABILTIES, *PKSM_BDA_SCAN_CAPABILTIES;

typedef struct _KSM_BDA_SCAN_FILTER 
{
    KSMETHOD    Method;
    ULONG       ulScanModulationTypeSize;
    ULONG64     AnalogVideoStandards;	
    BYTE        argbScanModulationTypes[MIN_DIMENSION];
} KSM_BDA_SCAN_FILTER, *PKSM_BDA_SCAN_FILTER;

typedef struct _KSM_BDA_SCAN_START 
{
    KSMETHOD    Method;
    ULONG       LowerFrequency;
    ULONG       HigherFrequency;
} KSM_BDA_SCAN_START, *PKSM_BDA_SCAN_START;


//===========================================================================
//
//  KSMethod Set Structure Definitions for PBDA GUIDE DATA  
//
//===========================================================================

typedef  struct _KSM_BDA_GDDS_TUNEXMLFROMIDX {
    KSMETHOD    Method;
    ULONG64     ulIdx;
} KSM_BDA_GDDS_TUNEXMLFROMIDX, *PKSM_BDA_GDDS_TUNEXMLFROMIDX;

typedef  struct _KSM_BDA_GDDS_SERVICEFROMTUNEXML 
{
    KSMETHOD    Method;
    ULONG       ulTuneXmlLength;
    BYTE        argbTuneXml[MIN_DIMENSION];
} KSM_BDA_GDDS_SERVICEFROMTUNEXML , *PKSM_BDA_GDDS_SERVICEFROMTUNEXML;

//===========================================================================
//
//  KSMethod Set Structure Definitions for PBDA USER ACTIVITY   
//
//===========================================================================
typedef struct _KSM_BDA_USERACTIVITY_USEREASON 
{
    KSMETHOD    Method;
    ULONG       ulUseReason; 
} KSM_BDA_USERACTIVITY_USEREASON, *PKSM_BDA_USERACTIVITY_USEREASON;

//===========================================================================
//
//  KSMethod Set Structure Definitions for PBDA CAS  
//
//===========================================================================
typedef struct _KSM_BDA_CAS_ENTITLEMENTTOKEN 
{
    KSM_NODE    NodeMethod;
    ULONG       ulDialogRequest; 
    CHAR        cLanguage[12];
    ULONG       ulRequestType;
    ULONG       ulEntitlementTokenLen;
    BYTE        argbEntitlementToken[MIN_DIMENSION];
} KSM_BDA_CAS_ENTITLEMENTTOKEN, *PKSM_BDA_CAS_ENTITLEMENTTOKEN;

typedef struct _KSM_BDA_CAS_CAPTURETOKEN 
{ 
    KSM_NODE    NodeMethod;
    ULONG       ulTokenLength; 
    BYTE        argbToken [MIN_DIMENSION];
} KSM_BDA_CAS_CAPTURETOKEN, *PKSM_BDA_CAS_CAPTURETOKEN;

typedef struct _KSM_BDA_CAS_OPENBROADCASTMMI 
{
    KSM_NODE    NodeMethod;
    ULONG       ulDialogRequest; 
    CHAR        cLanguage[12];
    ULONG       ulEventId;
} KSM_BDA_CAS_OPENBROADCASTMMI, *PKSM_BDA_CAS_OPENBROADCASTMMI;

typedef struct _KSM_BDA_CAS_CLOSEMMIDIALOG 
{
    KSM_NODE    NodeMethod;
    ULONG       ulDialogRequest;
    CHAR        cLanguage[12];
    ULONG       ulDialogNumber;
    ULONG       ulReason;
} KSM_BDA_CAS_CLOSEMMIDIALOG, *PKSM_BDA_CAS_CLOSEMMIDIALOG;

typedef struct _KSM_BDA_ISDBCAS_REQUEST 
{
    KSM_NODE    NodeMethod;
    ULONG       ulRequestID;
    ULONG       ulIsdbCommandSize;
    BYTE        argbIsdbCommandData [MIN_DIMENSION];
} KSM_BDA_ISDBCAS_REQUEST, *PKSM_BDA_ISDBCAS_REQUEST;

//===========================================================================
//
//  KSMethod Set Structure Definitions for Transprt Stream Selector  
//
//===========================================================================
typedef struct _KSM_BDA_TS_SELECTOR_SETTSID 
{
    KSM_NODE    NodeMethod;
    USHORT      usTSID; 
} KSM_BDA_TS_SELECTOR_SETTSID, *PKSM_BDA_TS_SELECTOR_SETTSID;


//===========================================================================
//
//  BDA Data Range definitions.  Includes specifier definitions.
//
//===========================================================================

//  Antenna Signal Formats
//

typedef struct tagKS_DATARANGE_BDA_ANTENNA {
   KSDATARANGE                  DataRange;

   //   Antenna specifier can go here if required
   //
} KS_DATARANGE_BDA_ANTENNA, *PKS_DATARANGE_BDA_ANTENNA;



//  Transport Formats
//

typedef struct tagBDA_TRANSPORT_INFO {
    ULONG           ulcbPhyiscalPacket; // Size, in bytes, of a physical packet
                                        // (e.g. Satellite link payload size.
    ULONG           ulcbPhyiscalFrame;  // Size, in bytes, of each physical frame
                                        // 0 indicates no HW requirement
    ULONG           ulcbPhyiscalFrameAlignment; // Capture buffer alignment in bytes
                                                // 0 and 1 indicate no alignment requirements
    REFERENCE_TIME  AvgTimePerFrame; // Normal ActiveMovie units (100 nS)

} BDA_TRANSPORT_INFO, *PBDA_TRANSPORT_INFO;

typedef struct tagKS_DATARANGE_BDA_TRANSPORT {
   KSDATARANGE                  DataRange;
   BDA_TRANSPORT_INFO           BdaTransportInfo;

   //   Transport specifier can go here if required
   //
} KS_DATARANGE_BDA_TRANSPORT, *PKS_DATARANGE_BDA_TRANSPORT;


//===========================================================================
//  BDA Event Guids
//
//      These are sent by the IBroadcastEvent service on the graph.
//      To receive,
//          0) Implement IBroadcastEvent in your receiving object - this has one Method on it: Fire()
//          1) QI the graphs service provider for SID_SBroadcastEventService
//                 for the IID_IBroadcastEvent object
//          2) OR create the event service (CLSID_BroadcastEventService) if not already there
//                 and register it
//          3) QI that object for it's IConnectionPoint interface (*pCP)
//          4) Advise your object on *pCP  (e.g. pCP->Advise(static_cast<IBroadCastEvent*>(this), &dwCookie)
//          5) Unadvise when done..
//          6) Implement IBroadcastEvent::Fire(GUID gEventID)
//             Check for relevant event below and deal with it appropriatly...
//===========================================================================

// {83183C03-C09E-45c4-A719-807A94952BF9}
#define STATIC_EVENTID_TuningChanging \
    0x83183c03, 0xc09e, 0x45c4, 0xa7, 0x19, 0x80, 0x7a, 0x94, 0x95, 0x2b, 0xf9
DEFINE_GUIDSTRUCT("83183C03-C09E-45c4-A719-807A94952BF9", EVENTID_TuningChanging);
#define EVENTID_TuningChanging DEFINE_GUIDNAMED(EVENTID_TuningChanging)

// {9D7E6235-4B7D-425d-A6D1-D717C33B9C4C}
#define STATIC_EVENTID_TuningChanged \
    0x9d7e6235, 0x4b7d, 0x425d, 0xa6, 0xd1, 0xd7, 0x17, 0xc3, 0x3b, 0x9c, 0x4c
DEFINE_GUIDSTRUCT("9D7E6235-4B7D-425d-A6D1-D717C33B9C4C", EVENTID_TuningChanged);
#define EVENTID_TuningChanged DEFINE_GUIDNAMED(EVENTID_TuningChanged)

// {9F02D3D0-9F06-4369-9F1E-3AD6CA19807E}
#define STATIC_EVENTID_CandidatePostTuneData \
    0x9F02D3D0, 0x9F06, 0x4369, 0x9F, 0x1E, 0x3A, 0xD6, 0xCA, 0x19, 0x80, 0x7E
DEFINE_GUIDSTRUCT("9F02D3D0-9F06-4369-9F1E-3AD6CA19807E", EVENTID_CandidatePostTuneData);
#define EVENTID_CandidatePostTuneData DEFINE_GUIDNAMED(EVENTID_CandidatePostTuneData)

// {2A65C528-2249-4070-AC16-00390CDFB2DD}
#define STATIC_EVENTID_CADenialCountChanged \
    0x2a65c528, 0x2249, 0x4070, 0xac, 0x16, 0x0, 0x39, 0xc, 0xdf, 0xb2, 0xdd
DEFINE_GUIDSTRUCT("2A65C528-2249-4070-AC16-00390CDFB2DD", EVENTID_CADenialCountChanged);
#define EVENTID_CADenialCountChanged DEFINE_GUIDNAMED(EVENTID_CADenialCountChanged)

// {6D9CFAF2-702D-4b01-8DFF-6892AD20D191}
#define STATIC_EVENTID_SignalStatusChanged \
    0x6d9cfaf2, 0x702d, 0x4b01, 0x8d, 0xff, 0x68, 0x92, 0xad, 0x20, 0xd1, 0x91
DEFINE_GUIDSTRUCT("6D9CFAF2-702D-4b01-8DFF-6892AD20D191", EVENTID_SignalStatusChanged);
#define EVENTID_SignalStatusChanged DEFINE_GUIDNAMED(EVENTID_SignalStatusChanged)

// {C87EC52D-CD18-404a-A076-C02A273D3DE7}
#define STATIC_EVENTID_NewSignalAcquired \
    0xc87ec52d, 0xcd18, 0x404a, 0xa0, 0x76, 0xc0, 0x2a, 0x27, 0x3d, 0x3d, 0xe7
DEFINE_GUIDSTRUCT("C87EC52D-CD18-404a-A076-C02A273D3DE7", EVENTID_NewSignalAcquired);
#define EVENTID_NewSignalAcquired DEFINE_GUIDNAMED(EVENTID_NewSignalAcquired)

// {D10DF9D5-C261-4b85-9E8A-517B3299CAB2}
#define STATIC_EVENTID_EASMessageReceived \
    0xd10df9d5, 0xc261, 0x4b85, 0x9e, 0x8a, 0x51, 0x7b, 0x32, 0x99, 0xca, 0xb2
DEFINE_GUIDSTRUCT("D10DF9D5-C261-4b85-9E8A-517B3299CAB2", EVENTID_EASMessageReceived);
#define EVENTID_EASMessageReceived DEFINE_GUIDNAMED(EVENTID_EASMessageReceived)

// This event is broadcasted with FireEx when a table(currently, PAT, PMT, NIT 
// and SDT for DVB; PAT, PMT, MGT and VCT for ATSC). The four parameters are:
// dwPara1 - TSID, ONID|TSID for DVB EIT
// dwPara2 - TID|PID
// dwPara3 - dwHashedVersion
// dwPara4 - program number for PMT, Segment#|SID for EIT, but not used for others
// {1B9C3703-D447-4e16-97BB-01799FC031ED}
#define STATIC_EVENTID_PSITable \
    0x1b9c3703, 0xd447, 0x4e16, 0x97, 0xbb, 0x1, 0x79, 0x9f, 0xc0, 0x31, 0xed
DEFINE_GUIDSTRUCT("1B9C3703-D447-4e16-97BB-01799FC031ED", EVENTID_PSITable);
#define EVENTID_PSITable DEFINE_GUIDNAMED(EVENTID_PSITable)

// This event is broadcasted with FireEx when the capture graph recognized that a
// current tuning channel has been terminated by broadcaster.
// The four parameters are:
// dwPara1 - TSID
// dwPara2 - ONID|SID
// dwPara3 - channel frequency
// dwPara4 - satellite orbital position (0xFFFFFFFF for non-satellite)
// {0A1D591C-E0D2-4f8e-8960-2335BEF45CCB}
#define STATIC_EVENTID_ServiceTerminated \
    0xa1d591c, 0xe0d2, 0x4f8e, 0x89, 0x60, 0x23, 0x35, 0xbe, 0xf4, 0x5c, 0xcb
DEFINE_GUIDSTRUCT("0A1D591C-E0D2-4f8e-8960-2335BEF45CCB", EVENTID_ServiceTerminated);
#define EVENTID_ServiceTerminated DEFINE_GUIDNAMED(EVENTID_ServiceTerminated)

// {A265FAEA-F874-4b38-9FF7-C53D02969996}
#define STATIC_EVENTID_CardStatusChanged\
    0xa265faea, 0xf874, 0x4b38, 0x9f, 0xf7, 0xc5, 0x3d, 0x2, 0x96, 0x99, 0x96
DEFINE_GUIDSTRUCT("A265FAEA-F874-4b38-9FF7-C53D02969996", EVENTID_CardStatusChanged);
#define EVENTID_CardStatusChanged DEFINE_GUIDNAMED(EVENTID_CardStatusChanged)
#define DTV_CardStatus_Inserted      0
#define DTV_CardStatus_Removed       1
#define DTV_CardStatus_Error         2
#define DTV_CardStatus_FirmwareDownload         3

// {000906F5-F0D1-41d6-A7DF-4028697669F6}
#define STATIC_EVENTID_DRMParingStatusChanged \
    0x906f5, 0xf0d1, 0x41d6, 0xa7, 0xdf, 0x40, 0x28, 0x69, 0x76, 0x69, 0xf6
DEFINE_GUIDSTRUCT("000906F5-F0D1-41d6-A7DF-4028697669F6", EVENTID_DRMParingStatusChanged);
#define EVENTID_DRMParingStatusChanged DEFINE_GUIDNAMED(EVENTID_DRMParingStatusChanged)
// The 1st parameter to this event is a BDA_DRMPairingStatus and 2nd is the error code.

// {5B2EBF78-B752-4420-B41E-A472DC95828E}
#define STATIC_EVENTID_DRMParingStepComplete \
    0x5b2ebf78, 0xb752, 0x4420, 0xb4, 0x1e, 0xa4, 0x72, 0xdc, 0x95, 0x82, 0x8e
DEFINE_GUIDSTRUCT("5B2EBF78-B752-4420-B41E-A472DC95828E", EVENTID_DRMParingStepComplete);
#define EVENTID_DRMParingStepComplete DEFINE_GUIDNAMED(EVENTID_DRMParingStepComplete)
// The 1st parameter is which pairing manager is generting the event
// The 2nd parameter is the step in the pairing process which is now complete
// The 3rd parameter is the result of the step
#define OCUR_PAIRING_PROTOCOL_VERSION 2
#define PBDA_PAIRING_PROTOCOL_VERSION 3

// {052C29AF-09A4-4b93-890F-BD6A348968A4}
#define STATIC_EVENTID_MMIMessage \
    0x52c29af, 0x9a4, 0x4b93, 0x89, 0xf, 0xbd, 0x6a, 0x34, 0x89, 0x68, 0xa4
DEFINE_GUIDSTRUCT("052C29AF-09A4-4b93-890F-BD6A348968A4", EVENTID_MMIMessage);
#define EVENTID_MMIMessage DEFINE_GUIDNAMED(EVENTID_MMIMessage)
#define DTV_MMIMessage_Open             0
#define DTV_MMIMessage_Close            1

// {9071AD5D-2359-4c95-8694-AFA81D70BFD5}
#define STATIC_EVENTID_EntitlementChanged \
    0x9071ad5d, 0x2359, 0x4c95, 0x86, 0x94, 0xaf, 0xa8, 0x1d, 0x70, 0xbf, 0xd5
DEFINE_GUIDSTRUCT("9071AD5D-2359-4c95-8694-AFA81D70BFD5", EVENTID_EntitlementChanged);
#define EVENTID_EntitlementChanged DEFINE_GUIDNAMED(EVENTID_EntitlementChanged)
#define DTV_Entitlement_CanDecrypt          0
#define DTV_Entitlement_NotEntitled         1
#define DTV_Entitlement_TechnicalFailure    2


// This FireEx event is fired when tuning to a STB channel number
// the first parameter passed is the channel number the STB has been tuned to
// {17C4D730-D0F0-413a-8C99-500469DE35AD}
#define STATIC_EVENTID_STBChannelNumber\
    0x17c4d730, 0xd0f0, 0x413a, 0x8c, 0x99, 0x50, 0x04, 0x69, 0xde, 0x35, 0xad
DEFINE_GUIDSTRUCT("17C4D730-D0F0-413a-8C99-500469DE35AD", EVENTID_STBChannelNumber);
#define EVENTID_STBChannelNumber DEFINE_GUIDNAMED(EVENTID_STBChannelNumber)

// {5CA51711-5DDC-41a6-9430-E41B8B3BBC5B}
#define STATIC_EVENTID_BDAEventingServicePendingEvent \
    0x5ca51711, 0x5ddc, 0x41a6, 0x94, 0x30, 0xe4, 0x1b, 0x8b, 0x3b, 0xbc, 0x5b	
DEFINE_GUIDSTRUCT("5CA51711-5DDC-41a6-9430-E41B8B3BBC5B", EVENTID_BDAEventingServicePendingEvent);
#define EVENTID_BDAEventingServicePendingEvent DEFINE_GUIDNAMED(EVENTID_BDAEventingServicePendingEvent)

// {EFC3A459-AE8B-4b4a-8FE9-79A0D097F3EA}
#define STATIC_EVENTID_BDAConditionalAccessTAG \
	0xefc3a459, 0xae8b, 0x4b4a, 0x8f, 0xe9, 0x79, 0xa0, 0xd0, 0x97, 0xf3, 0xea
DEFINE_GUIDSTRUCT("EFC3A459-AE8B-4b4a-8FE9-79A0D097F3EA", EVENTID_BDAConditionalAccessTAG);
#define EVENTID_BDAConditionalAccessTAG DEFINE_GUIDNAMED(EVENTID_BDAConditionalAccessTAG)

// {B2127D42-7BE5-4f4b-9130-6679899F4F4B}
#define STATIC_EVENTTYPE_CASDescrambleFailureEvent \
    0xb2127d42, 0x7be5, 0x4f4b, 0x91, 0x30, 0x66, 0x79, 0x89, 0x9f, 0x4f, 0x4b
DEFINE_GUIDSTRUCT("B2127D42-7BE5-4f4b-9130-6679899F4F4B", EVENTTYPE_CASDescrambleFailureEvent);
#define EVENTTYPE_CASDescrambleFailureEvent DEFINE_GUIDNAMED(EVENTTYPE_CASDescrambleFailureEvent)

// {EAD831AE-5529-4d1f-AFCE-0D8CD1257D30}
#define STATIC_EVENTID_CASFailureSpanningEvent \
    0xead831ae, 0x5529, 0x4d1f, 0xaf, 0xce, 0xd, 0x8c, 0xd1, 0x25, 0x7d, 0x30
DEFINE_GUIDSTRUCT("EAD831AE-5529-4d1f-AFCE-0D8CD1257D30", EVENTID_CASFailureSpanningEvent);
#define EVENTID_CASFailureSpanningEvent DEFINE_GUIDNAMED(EVENTID_CASFailureSpanningEvent)

typedef enum {
    ChannelChangeSpanningEvent_Start = 0,   // Same as MSNP_EVENT_CHANGING defined in ehtraceguids.h
    ChannelChangeSpanningEvent_End = 2      // Same as MSNP_EVENT_COMPLETED defined in ehtraceguids.h
} ChannelChangeSpanningEvent_State;

// {9067C5E5-4C5C-4205-86C8-7AFE20FE1EFA} same as __uuidof(EH_MSNP_TUNING_EVENT) defined in ehtraceguids.h
#define STATIC_EVENTID_ChannelChangeSpanningEvent \
    0x9067C5E5, 0x4C5C, 0x4205, 0x86, 0xc8, 0x7a, 0xfe, 0x20, 0xfe, 0x1e, 0xfa
DEFINE_GUIDSTRUCT("9067C5E5-4C5C-4205-86C8-7AFE20FE1EFA", EVENTID_ChannelChangeSpanningEvent);
#define EVENTID_ChannelChangeSpanningEvent DEFINE_GUIDNAMED(EVENTID_ChannelChangeSpanningEvent)


typedef struct _ChannelChangeInfo
{
    ChannelChangeSpanningEvent_State state;
    ULONGLONG TimeStamp;
}ChannelChangeInfo;

#define STATIC_EVENTID_ChannelTypeSpanningEvent \
    0x72ab1d51, 0x87d2, 0x489b, 0xba, 0x11, 0xe, 0x8, 0xdc, 0x21, 0x2, 0x43
DEFINE_GUIDSTRUCT("72ab1d51-87d2-489b-ba11-0e08dc210243", EVENTID_ChannelTypeSpanningEvent);
#define EVENTID_ChannelTypeSpanningEvent DEFINE_GUIDNAMED(EVENTID_ChannelTypeSpanningEvent)

typedef enum
{
    ChannelTypeNone        = 0x0000,
    // bit flags, can be ORed
    // type == 2 ^ ComponentCategory in bdatypes.h
    ChannelTypeOther       = 0x0001,
    ChannelTypeVideo       = 0x0002,
    ChannelTypeAudio       = 0x0004,
    ChannelTypeText        = 0x0008,
    ChannelTypeSubtitles   = 0x0010,
    ChannelTypeCaptions    = 0x0020,
    ChannelTypeSuperimpose = 0x0040,
    ChannelTypeData        = 0x0080
} ChannelType;

typedef struct _ChannelTypeInfo
{
    ChannelType channelType;
    ULONGLONG timeStamp;
}ChannelTypeInfo;

typedef struct _ChannelInfo
{
    LONG lFrequency;
    union
    {
        struct
        {
            LONG lONID;
            LONG lTSID;
            LONG lSID;
        } DVB;
        struct
        {
            LONG lProgNumber;
        } DC;
        struct 
        {
            LONG lProgNumber;
        } ATSC;
    };

} ChannelInfo;

// {41F36D80-4132-4cc2-B121-01A43219D81B}
#define STATIC_EVENTID_ChannelInfoSpanningEvent \
    0x41f36d80, 0x4132, 0x4cc2, 0xb1, 0x21, 0x1, 0xa4, 0x32, 0x19, 0xd8, 0x1b
DEFINE_GUIDSTRUCT("41F36D80-4132-4cc2-B121-01A43219D81B", EVENTID_ChannelInfoSpanningEvent);
#define EVENTID_ChannelInfoSpanningEvent DEFINE_GUIDNAMED(EVENTID_ChannelInfoSpanningEvent)

// F6CFC8F4-DA93-4f2f-BFF8-BA1EE6FCA3A2 same as __uuidof(EH_RRT_EVENT) defined in ehtraceguids.h
#define STATIC_EVENTID_RRTSpanningEvent \
    0xf6cfc8f4, 0xda93, 0x4f2f, 0xbf, 0xf8, 0xba, 0x1e, 0xe6, 0xfc, 0xa3, 0xa2
DEFINE_GUIDSTRUCT("F6CFC8F4-DA93-4f2f-BFF8-BA1EE6FCA3A2", EVENTID_RRTSpanningEvent);
#define EVENTID_RRTSpanningEvent DEFINE_GUIDNAMED(EVENTID_RRTSpanningEvent)

// Data sturcture for both CaptionServiceDescriptor and Content Advisory descriptor
typedef struct _SpanningEventDescriptor{
    WORD wDataLen;          // Total length of the data(2*sizeof(WORD)+lengthof(bDescriptor))
    WORD wProgNumber;       // Program numberassociated with this descriptor
    WORD wSID;              // Source ID associated with this descriptor
    BYTE bDescriptor[1];    // Raw descriptor data
} SpanningEventDescriptor;

// Caption Service descriptior spanning event
// {EFE779D9-97F0-4786-800D-95CF505DDC66} same as __uuidof(EH_CaptionService_EVENT) defined in ehtraceguids.h
#define STATIC_EVENTID_CSDescriptorSpanningEvent \
	0xefe779d9, 0x97f0, 0x4786, 0x80, 0xd, 0x95, 0xcf, 0x50, 0x5d, 0xdc, 0x66
DEFINE_GUIDSTRUCT("EFE779D9-97F0-4786-800D-95CF505DDC66", EVENTID_CSDescriptorSpanningEvent);
#define EVENTID_CSDescriptorSpanningEvent DEFINE_GUIDNAMED(EVENTID_CSDescriptorSpanningEvent)

// Content Advisory descriptor spanning event
// {3AB4A2E6-4247-4b34-896C-30AFA5D21C24} same as __uuidof(EH_ContentAdvisory_EVENT) defined in ehtraceguids.h
#define STATIC_EVENTID_CtxADescriptorSpanningEvent \
	0x3ab4a2e6, 0x4247, 0x4b34, 0x89, 0x6c, 0x30, 0xaf, 0xa5, 0xd2, 0x1c, 0x24
DEFINE_GUIDSTRUCT("3AB4A2E6-4247-4b34-896C-30AFA5D21C24", EVENTID_CtxADescriptorSpanningEvent);
#define EVENTID_CtxADescriptorSpanningEvent DEFINE_GUIDNAMED(EVENTID_CtxADescriptorSpanningEvent)

typedef struct _DVBScramblingControlSpanningEvent
{
    ULONG ulPID;
    BOOL fScrambled;
} DVBScramblingControlSpanningEvent;

// transport_scarmbling_control flag global event
// {4BD4E1C4-90A1-4109-8236-27F00E7DCC5B}
#define STATIC_EVENTID_DVBScramblingControlSpanningEvent \
    0x4bd4e1c4, 0x90a1, 0x4109, 0x82, 0x36, 0x27, 0xf0, 0xe, 0x7d, 0xcc, 0x5b
DEFINE_GUIDSTRUCT("4BD4E1C4-90A1-4109-8236-27F00E7DCC5B", EVENTID_DVBScramblingControlSpanningEvent);
#define EVENTID_DVBScramblingControlSpanningEvent DEFINE_GUIDNAMED(EVENTID_DVBScramblingControlSpanningEvent)

typedef enum {
    SignalAndServiceStatusSpanningEvent_None            = -1,    
    SignalAndServiceStatusSpanningEvent_Clear           = 0,    // same as MSNP_EVENT_SIGNALANDSERVICE_TYPE in ehtraceguids.h
    SignalAndServiceStatusSpanningEvent_NoTVSignal      = 1,
    SignalAndServiceStatusSpanningEvent_ServiceOffAir   = 2,
    SignalAndServiceStatusSpanningEvent_WeakTVSignal    = 3,
    SignalAndServiceStatusSpanningEvent_NoSubscription  = 4,
    SignalAndServiceStatusSpanningEvent_AllAVScrambled  = 5,
} SignalAndServiceStatusSpanningEvent_State;

// Signal and Service Status event
// {8068C5CB-3C04-492b-B47D-0308820DCE51} same as __uuidof(EH_MSNP_SIGNALANDSERVICE_EVENT) defined in ehtraceguids.h
#define STATIC_EVENTID_SignalAndServiceStatusSpanningEvent \
    0x8068c5cb, 0x3c04, 0x492b, 0xb4, 0x7d, 0x3, 0x8, 0x82, 0xd, 0xce, 0x51
DEFINE_GUIDSTRUCT("8068C5CB-3C04-492b-B47D-0308820DCE51", EVENTID_SignalAndServiceStatusSpanningEvent);
#define EVENTID_SignalAndServiceStatusSpanningEvent DEFINE_GUIDNAMED(EVENTID_SignalAndServiceStatusSpanningEvent)

#define EVENTID_SignalAndServiceStatusEvent EVENTID_SignalAndServiceStatusSpanningEvent 

// Data structure for EmmMessageSpanningEvent
typedef struct _SpanningEventEmmMessage{
    BYTE bCAbroadcasterGroupId; // CA Broadcaster Group ID from CA_service_descriptor (ARIB STD-B25)
    BYTE bMessageControl;       // Message Control from CA_service_descriptor (ARIB STD-B25)
    WORD wServiceId;            // Service ID of ISDB bound with this
    WORD wTableIdExtension;     // Zero means the followings are inoperable
    BYTE bDeletionStatus;
    BYTE bDisplayingDuration1;
    BYTE bDisplayingDuration2;
    BYTE bDisplayingDuration3;
    BYTE bDisplayingCycle;
    BYTE bFormatVersion;
    BYTE bDisplayPosition;
    WORD wMessageLength;
    WCHAR szMessageArea[MIN_DIMENSION];
} SpanningEventEmmMessage;

// EMM Message spanning event
// {6BF00268-4F7E-4294-AA87-E9E953E43F14} same as __uuidof(EH_EmmMessage_EVENT) defined in ehtraceguids.h
#define STATIC_EVENTID_EmmMessageSpanningEvent \
    0x6bf00268, 0x4f7e, 0x4294, 0xaa, 0x87, 0xe9, 0xe9, 0x53, 0xe4, 0x3f, 0x14
DEFINE_GUIDSTRUCT("6BF00268-4F7E-4294-AA87-E9E953E43F14", EVENTID_EmmMessageSpanningEvent);
#define EVENTID_EmmMessageSpanningEvent DEFINE_GUIDNAMED(EVENTID_EmmMessageSpanningEvent)

// {501CBFBE-B849-42ce-9BE9-3DB869FB82B3}
#define STATIC_EVENTID_AudioTypeSpanningEvent \
	0x501cbfbe, 0xb849, 0x42ce, 0x9b, 0xe9, 0x3d, 0xb8, 0x69, 0xfb, 0x82, 0xb3
DEFINE_GUIDSTRUCT("501CBFBE-B849-42ce-9BE9-3DB869FB82B3", EVENTID_AudioTypeSpanningEvent);
#define EVENTID_AudioTypeSpanningEvent DEFINE_GUIDNAMED(EVENTID_AudioTypeSpanningEvent)
// AC 3 audio type and ISO639 language descriptor audio type are slight different. The 
// AudioType values defined here is for the convenience of the user of audio type info 
// and the conversion from the original spec to these values is done in capture.
//
// ISO639 language descriptor audio types:
// 0x00	undefined               (standard audio)
// 0x01	clean effects           
// 0x02	hearing impaired        
// 0x03	visual impaired commentary
// 0x04-0xFF	reserved
// 
// AC3 audio types
// 0 Complete Main (CM)
// 1 Music and Effects (ME)
// 2 Visually Impaired (VI)
// 3 Hearing Impaired (HI)
// 4 Dialogue (D)
// 5 Commentary (C)
// 6 Emergency (E)
// 7 Voiceover (VO)

#define AudioType_Standard              0
#define AudioType_Music_And_Effects     1
#define AudioType_Visually_Impaired     2
#define AudioType_Hearing_Impaired      3
#define AudioType_Dialogue              4
#define AudioType_Commentary            5
#define AudioType_Emergency             6
#define AudioType_Voiceover             7
#define AudioType_Reserved              -1

// {82af2ebc-30a6-4264-a80b-ad2e1372ac60}
#define STATIC_EVENTID_StreamTypeSpanningEvent \
	0x82af2ebc, 0x30a6, 0x4264, 0xa8, 0x0b, 0xad, 0x2e, 0x13, 0x72, 0xac, 0x60
DEFINE_GUIDSTRUCT("82af2ebc-30a6-4264-a80b-ad2e1372ac60", EVENTID_StreamTypeSpanningEvent);
#define EVENTID_StreamTypeSpanningEvent DEFINE_GUIDNAMED(EVENTID_StreamTypeSpanningEvent)

// {3A954083-93D0-463e-90B2-0742C496EDF0}
#define STATIC_EVENTID_ARIBcontentSpanningEvent \
	0x3a954083, 0x93d0, 0x463e, 0x90, 0xb2, 0x7, 0x42, 0xc4, 0x96, 0xed, 0xf0
DEFINE_GUIDSTRUCT("3A954083-93D0-463e-90B2-0742C496EDF0", EVENTID_ARIBcontentSpanningEvent);
#define EVENTID_ARIBcontentSpanningEvent DEFINE_GUIDNAMED(EVENTID_ARIBcontentSpanningEvent)

// {E292666D-9C02-448d-AA8D-781A93FDC395}
#define STATIC_EVENTID_LanguageSpanningEvent \
	0xe292666d, 0x9c02, 0x448d, 0xaa, 0x8d, 0x78, 0x1a, 0x93, 0xfd, 0xc3, 0x95
DEFINE_GUIDSTRUCT("E292666D-9C02-448d-AA8D-781A93FDC395", EVENTID_LanguageSpanningEvent);
#define EVENTID_LanguageSpanningEvent DEFINE_GUIDNAMED(EVENTID_LanguageSpanningEvent)
typedef struct _LanguageInfo
{
    LANGID LangID;
    LONG lISOLangCode;
} LanguageInfo;

// {A9A29B56-A84B-488c-89D5-0D4E7657C8CE}
#define STATIC_EVENTID_DualMonoSpanningEvent \
    0xa9a29b56, 0xa84b, 0x488c, 0x89, 0xd5, 0x0d, 0x4e, 0x76, 0x57, 0xc8, 0xce
DEFINE_GUIDSTRUCT("A9A29B56-A84B-488c-89D5-0D4E7657C8CE", EVENTID_DualMonoSpanningEvent);
#define EVENTID_DualMonoSpanningEvent DEFINE_GUIDNAMED(EVENTID_DualMonoSpanningEvent)
typedef struct _DualMonoInfo
{
    LANGID LangID1;
    LANGID LangID2;
    LONG lISOLangCode1;
    LONG lISOLangCode2;
} DualMonoInfo;

// {47FC8F65-E2BB-4634-9CEF-FDBFE6261D5C}
#define STATIC_EVENTID_PIDListSpanningEvent \
	0x47fc8f65, 0xe2bb, 0x4634, 0x9c, 0xef, 0xfd, 0xbf, 0xe6, 0x26, 0x1d, 0x5c
DEFINE_GUIDSTRUCT("47FC8F65-E2BB-4634-9CEF-FDBFE6261D5C", EVENTID_PIDListSpanningEvent);
#define EVENTID_PIDListSpanningEvent DEFINE_GUIDNAMED(EVENTID_PIDListSpanningEvent)
typedef struct _PIDListSpanningEvent
{
    WORD wPIDCount;
    ULONG pulPIDs[1];
} PIDListSpanningEvent;

// {107BD41C-A6DA-4691-8369-11B2CDAA288E}
#define STATIC_EVENTID_AudioDescriptorSpanningEvent \
    0x107bd41c, 0xa6da, 0x4691, 0x83, 0x69, 0x11, 0xb2, 0xcd, 0xaa, 0x28, 0x8e
DEFINE_GUIDSTRUCT("107BD41C-A6DA-4691-8369-11B2CDAA288E", EVENTID_AudioDescriptorSpanningEvent);
#define EVENTID_AudioDescriptorSpanningEvent DEFINE_GUIDNAMED(EVENTID_AudioDescriptorSpanningEvent)

// {5DCEC048-D0B9-4163-872C-4F32223BE88A}
#define STATIC_EVENTID_SubtitleSpanningEvent \
	0x5dcec048, 0xd0b9, 0x4163, 0x87, 0x2c, 0x4f, 0x32, 0x22, 0x3b, 0xe8, 0x8a
DEFINE_GUIDSTRUCT("5DCEC048-D0B9-4163-872C-4F32223BE88A", EVENTID_SubtitleSpanningEvent);
#define EVENTID_SubtitleSpanningEvent DEFINE_GUIDNAMED(EVENTID_SubtitleSpanningEvent)

// {9599D950-5F33-4617-AF7C-1E54B510DAA3}
#define STATIC_EVENTID_TeletextSpanningEvent \
	0x9599d950, 0x5f33, 0x4617, 0xaf, 0x7c, 0x1e, 0x54, 0xb5, 0x10, 0xda, 0xa3
DEFINE_GUIDSTRUCT("9599D950-5F33-4617-AF7C-1E54B510DAA3", EVENTID_TeletextSpanningEvent);
#define EVENTID_TeletextSpanningEvent DEFINE_GUIDNAMED(EVENTID_TeletextSpanningEvent)

// {CAF1AB68-E153-4d41-A6B3-A7C998DB75EE}
#define STATIC_EVENTID_StreamIDSpanningEvent \
	0xcaf1ab68, 0xe153, 0x4d41, 0xa6, 0xb3, 0xa7, 0xc9, 0x98, 0xdb, 0x75, 0xee
DEFINE_GUIDSTRUCT("CAF1AB68-E153-4d41-A6B3-A7C998DB75EE", EVENTID_StreamIDSpanningEvent);
#define EVENTID_StreamIDSpanningEvent DEFINE_GUIDNAMED(EVENTID_StreamIDSpanningEvent)

// {F947AA85-FB52-48e8-B9C5-E1E1F411A51A}
#define STATIC_EVENTID_PBDAParentalControlEvent \
	0xf947aa85, 0xfb52, 0x48e8, 0xb9, 0xc5, 0xe1, 0xe1, 0xf4, 0x11, 0xa5, 0x1a
DEFINE_GUIDSTRUCT("F947AA85-FB52-48e8-B9C5-E1E1F411A51A", EVENTID_PBDAParentalControlEvent);
#define EVENTID_PBDAParentalControlEvent DEFINE_GUIDNAMED(EVENTID_PBDAParentalControlEvent)

#pragma pack(push,1)

#define MAX_COUNTRY_CODE_STRING 3

typedef struct {
    DWORD rating_attribute_id;
    DWORD rating_attribute_value;
} RATING_ATTRIBUTE, *LPRATING_ATTRIBUTE;

typedef struct {
    GUID                rating_system_id;
    BYTE                rating_system_is_age_type:  1;
    BYTE                reserved:                   7;
    BYTE                country_code[MAX_COUNTRY_CODE_STRING];
    DWORD               rating_attribute_count;
    RATING_ATTRIBUTE    *lpratingattrib; 
} RATING_SYSTEM, *LPRATING_SYSTEM; 

typedef struct {
    DWORD           rating_system_count;
    RATING_SYSTEM   *lpratingsystem;

} RATING_INFO, *LPRATING_INFO;

// attribute_id
#define PARENTAL_CONTROL_TIME_RANGE                  0x00000001  // Parental Control Time Range
#define REQUIRED_PARENTAL_CONTROL_TIME_RANGE         0x00000002  // Required Parental Control Time Range
#define PARENTAL_CONTROL_CONTENT_RATING     0x00000100  // Rating   (overall/primary content rating)
#define PARENTAL_CONTROL_ATTRIB_VIOLENCE    0x00000200  // Violence
#define PARENTAL_CONTROL_ATTRIB_LANGUAGE    0x00000201  // Language
#define PARENTAL_CONTROL_ATTRIB_SEXUAL      0x00000202  // Sexual Content
#define PARENTAL_CONTROL_ATTRIB_DIALOGUE    0x00000203  // Dialogue 
#define PARENTAL_CONTROL_ATTRIB_FANTASY     0x00000204  // Fantasy Violence 

#define PARENTAL_CONTROL_VALUE_UNDEFINED    0           // UNDEFINED 

typedef struct _PBDAParentalControl
{
    ULONG               rating_system_count;    // number of rating systems in PBDA parenatl control table
    RATING_SYSTEM *     rating_systems;         // PBDA unified rating systems
} PBDAParentalControl;

#pragma pack(pop)

// {D97287B2-2DFD-436a-9485-99D7D4AB5A69}
#define STATIC_EVENTID_TuneFailureEvent \
    0xd97287b2, 0x2dfd, 0x436a, 0x94, 0x85, 0x99, 0xd7, 0xd4, 0xab, 0x5a, 0x69
DEFINE_GUIDSTRUCT("D97287B2-2DFD-436a-9485-99D7D4AB5A69", EVENTID_TuneFailureEvent);
#define EVENTID_TuneFailureEvent DEFINE_GUIDNAMED(EVENTID_TuneFailureEvent)

// {6F8AA455-5EE1-48ab-A27C-4C8D70B9AEBA}
#define STATIC_EVENTID_TuneFailureSpanningEvent \
    0x6f8aa455, 0x5ee1, 0x48ab, 0xa2, 0x7c, 0x4c, 0x8d, 0x70, 0xb9, 0xae, 0xba
DEFINE_GUIDSTRUCT("6F8AA455-5EE1-48ab-A27C-4C8D70B9AEBA", EVENTID_TuneFailureSpanningEvent);
#define EVENTID_TuneFailureSpanningEvent DEFINE_GUIDNAMED(EVENTID_TuneFailureSpanningEvent)

// {2A67A58D-ECA5-4eac-ABCB-E734D3776D0A}
#define STATIC_EVENTID_DvbParentalRatingDescriptor \
    0x2a67a58d, 0xeca5, 0x4eac, 0xab, 0xcb, 0xe7, 0x34, 0xd3, 0x77, 0x6d, 0x0a
DEFINE_GUIDSTRUCT("2A67A58D-ECA5-4eac-ABCB-E734D3776D0A", EVENTID_DvbParentalRatingDescriptor);
#define EVENTID_DvbParentalRatingDescriptor DEFINE_GUIDNAMED(EVENTID_DvbParentalRatingDescriptor)
typedef struct
{
    CHAR szCountryCode[4];  // 3-chars + null
    BYTE bRating;           // rating
} DvbParentalRatingParam;
typedef struct 
{
    ULONG                   ulNumParams; // if zero, no rating
    DvbParentalRatingParam pParams[1];
} DvbParentalRatingDescriptor;

// {F5689FFE-55F9-4bb3-96BE-AE971C63BAE0}
#define STATIC_EVENTID_DFNWithNoActualAVData \
    0xf5689ffe, 0x55f9, 0x4bb3, 0x96, 0xbe, 0xae, 0x97, 0x1c, 0x63, 0xba, 0xe0
DEFINE_GUIDSTRUCT("F5689FFE-55F9-4bb3-96BE-AE971C63BAE0", EVENTID_DFNWithNoActualAVData);
#define EVENTID_DFNWithNoActualAVData DEFINE_GUIDNAMED(EVENTID_DFNWithNoActualAVData)

//===========================================================================
//
//  BDA Stream Format GUIDs
//
//===========================================================================

#define STATIC_KSDATAFORMAT_TYPE_BDA_ANTENNA\
    0x71985f41, 0x1ca1, 0x11d3, 0x9c, 0xc8, 0x0, 0xc0, 0x4f, 0x79, 0x71, 0xe0
DEFINE_GUIDSTRUCT("71985F41-1CA1-11d3-9CC8-00C04F7971E0", KSDATAFORMAT_TYPE_BDA_ANTENNA);
#define KSDATAFORMAT_TYPE_BDA_ANTENNA DEFINE_GUIDNAMED(KSDATAFORMAT_TYPE_BDA_ANTENNA)


#define STATIC_KSDATAFORMAT_SUBTYPE_BDA_MPEG2_TRANSPORT\
    0xf4aeb342, 0x0329, 0x4fdd, 0xa8, 0xfd, 0x4a, 0xff, 0x49, 0x26, 0xc9, 0x78
DEFINE_GUIDSTRUCT("F4AEB342-0329-4fdd-A8FD-4AFF4926C978", KSDATAFORMAT_SUBTYPE_BDA_MPEG2_TRANSPORT);
#define KSDATAFORMAT_SUBTYPE_BDA_MPEG2_TRANSPORT DEFINE_GUIDNAMED(KSDATAFORMAT_SUBTYPE_BDA_MPEG2_TRANSPORT)


#define STATIC_KSDATAFORMAT_SPECIFIER_BDA_TRANSPORT\
    0x8deda6fd, 0xac5f, 0x4334, 0x8e, 0xcf, 0xa4, 0xba, 0x8f, 0xa7, 0xd0, 0xf0
DEFINE_GUIDSTRUCT("8DEDA6FD-AC5F-4334-8ECF-A4BA8FA7D0F0", KSDATAFORMAT_SPECIFIER_BDA_TRANSPORT);
#define KSDATAFORMAT_SPECIFIER_BDA_TRANSPORT DEFINE_GUIDNAMED(KSDATAFORMAT_SPECIFIER_BDA_TRANSPORT)


#define STATIC_KSDATAFORMAT_TYPE_BDA_IF_SIGNAL\
    0x61be0b47, 0xa5eb, 0x499b, 0x9a, 0x85, 0x5b, 0x16, 0xc0, 0x7f, 0x12, 0x58
DEFINE_GUIDSTRUCT("61BE0B47-A5EB-499b-9A85-5B16C07F1258", KSDATAFORMAT_TYPE_BDA_IF_SIGNAL);
#define KSDATAFORMAT_TYPE_BDA_IF_SIGNAL DEFINE_GUIDNAMED(KSDATAFORMAT_TYPE_BDA_IF_SIGNAL)


#define STATIC_KSDATAFORMAT_TYPE_MPEG2_SECTIONS\
    0x455f176c, 0x4b06, 0x47ce, 0x9a, 0xef, 0x8c, 0xae, 0xf7, 0x3d, 0xf7, 0xb5
DEFINE_GUIDSTRUCT("455F176C-4B06-47CE-9AEF-8CAEF73DF7B5", KSDATAFORMAT_TYPE_MPEG2_SECTIONS);
#define KSDATAFORMAT_TYPE_MPEG2_SECTIONS DEFINE_GUIDNAMED(KSDATAFORMAT_TYPE_MPEG2_SECTIONS)


#define STATIC_KSDATAFORMAT_SUBTYPE_ATSC_SI\
    0xb3c7397c, 0xd303, 0x414d, 0xb3, 0x3c, 0x4e, 0xd2, 0xc9, 0xd2, 0x97, 0x33
DEFINE_GUIDSTRUCT("B3C7397C-D303-414D-B33C-4ED2C9D29733", KSDATAFORMAT_SUBTYPE_ATSC_SI);
#define KSDATAFORMAT_SUBTYPE_ATSC_SI DEFINE_GUIDNAMED(KSDATAFORMAT_SUBTYPE_ATSC_SI)


#define STATIC_KSDATAFORMAT_SUBTYPE_DVB_SI\
    0xe9dd31a3, 0x221d, 0x4adb, 0x85, 0x32, 0x9a, 0xf3, 0x9, 0xc1, 0xa4, 0x8
DEFINE_GUIDSTRUCT("e9dd31a3-221d-4adb-8532-9af309c1a408", KSDATAFORMAT_SUBTYPE_DVB_SI);
#define KSDATAFORMAT_SUBTYPE_DVB_SI DEFINE_GUIDNAMED(KSDATAFORMAT_SUBTYPE_DVB_SI)


#define STATIC_KSDATAFORMAT_SUBTYPE_BDA_OPENCABLE_PSIP\
    0x762e3f66, 0x336f, 0x48d1, 0xbf, 0x83, 0x2b, 0x0, 0x35, 0x2c, 0x11, 0xf0
DEFINE_GUIDSTRUCT("762E3F66-336F-48d1-BF83-2B00352C11F0", KSDATAFORMAT_SUBTYPE_BDA_OPENCABLE_PSIP);
#define KSDATAFORMAT_SUBTYPE_BDA_OPENCABLE_PSIP DEFINE_GUIDNAMED(KSDATAFORMAT_SUBTYPE_BDA_OPENCABLE_PSIP)


#define STATIC_KSDATAFORMAT_SUBTYPE_BDA_OPENCABLE_OOB_PSIP\
    0x951727db, 0xd2ce, 0x4528, 0x96, 0xf6, 0x33, 0x1, 0xfa, 0xbb, 0x2d, 0xe0
DEFINE_GUIDSTRUCT("951727DB-D2CE-4528-96F6-3301FABB2DE0", KSDATAFORMAT_SUBTYPE_BDA_OPENCABLE_OOB_PSIP);
#define KSDATAFORMAT_SUBTYPE_BDA_OPENCABLE_OOB_PSIP DEFINE_GUIDNAMED(KSDATAFORMAT_SUBTYPE_BDA_OPENCABLE_OOB_PSIP)


#define STATIC_KSDATAFORMAT_SUBTYPE_ISDB_SI\
    0x4a2eeb99, 0x6458, 0x4538, 0xb1, 0x87, 0x04, 0x01, 0x7c, 0x41, 0x41, 0x3f
DEFINE_GUIDSTRUCT("4a2eeb99-6458-4538-b187-04017c41413f", KSDATAFORMAT_SUBTYPE_ISDB_SI);
#define KSDATAFORMAT_SUBTYPE_ISDB_SI DEFINE_GUIDNAMED(KSDATAFORMAT_SUBTYPE_ISDB_SI)


#define STATIC_KSDATAFORMAT_SUBTYPE_PBDA_TRANSPORT_RAW\
    0x0d7aed42, 0xcb9a, 0x11db, 0x97, 0x05, 0x00, 0x50, 0x56, 0xc0, 0x00, 0x08
DEFINE_GUIDSTRUCT("0d7AED42-CB9A-11DB-9705-005056C00008", KSDATAFORMAT_SUBTYPE_PBDA_TRANSPORT_RAW);
#define KSDATAFORMAT_SUBTYPE_PBDA_TRANSPORT_RAW DEFINE_GUIDNAMED(KSDATAFORMAT_SUBTYPE_PBDA_TRANSPORT_RAW)


//===========================================================================
//
//  KSPinName Definitions for BDA
//
//===========================================================================

//  Pin name for a BDA transport pin
//
// {78216A81-CFA8-493e-9711-36A61C08BD9D}
//
#define STATIC_PINNAME_BDA_TRANSPORT \
    0x78216a81, 0xcfa8, 0x493e, 0x97, 0x11, 0x36, 0xa6, 0x1c, 0x8, 0xbd, 0x9d
DEFINE_GUIDSTRUCT("78216A81-CFA8-493e-9711-36A61C08BD9D", PINNAME_BDA_TRANSPORT);
#define PINNAME_BDA_TRANSPORT DEFINE_GUIDNAMED(PINNAME_BDA_TRANSPORT)


//  Pin name for a BDA analog video pin
//
// {5C0C8281-5667-486c-8482-63E31F01A6E9}
//
#define STATIC_PINNAME_BDA_ANALOG_VIDEO \
    0x5c0c8281, 0x5667, 0x486c, 0x84, 0x82, 0x63, 0xe3, 0x1f, 0x1, 0xa6, 0xe9
DEFINE_GUIDSTRUCT("5C0C8281-5667-486c-8482-63E31F01A6E9", PINNAME_BDA_ANALOG_VIDEO);
#define PINNAME_BDA_ANALOG_VIDEO DEFINE_GUIDNAMED(PINNAME_BDA_ANALOG_VIDEO)


//  Pin name for a BDA analog audio pin
//
// {D28A580A-9B1F-4b0c-9C33-9BF0A8EA636B}
//
#define STATIC_PINNAME_BDA_ANALOG_AUDIO \
    0xd28a580a, 0x9b1f, 0x4b0c, 0x9c, 0x33, 0x9b, 0xf0, 0xa8, 0xea, 0x63, 0x6b
DEFINE_GUIDSTRUCT("D28A580A-9B1F-4b0c-9C33-9BF0A8EA636B", PINNAME_BDA_ANALOG_AUDIO);
#define PINNAME_BDA_ANALOG_AUDIO DEFINE_GUIDNAMED(PINNAME_BDA_ANALOG_AUDIO)


//  Pin name for a BDA FM Radio pin
//
// {D2855FED-B2D3-4eeb-9BD0-193436A2F890}
//
#define STATIC_PINNAME_BDA_FM_RADIO \
    0xd2855fed, 0xb2d3, 0x4eeb, 0x9b, 0xd0, 0x19, 0x34, 0x36, 0xa2, 0xf8, 0x90
DEFINE_GUIDSTRUCT("D2855FED-B2D3-4eeb-9BD0-193436A2F890", PINNAME_BDA_FM_RADIO);
#define PINNAME_BDA_FM_RADIO DEFINE_GUIDNAMED(PINNAME_BDA_FM_RADIO)


//  Pin name for a BDA Intermediate Frequency pin
//
// {1A9D4A42-F3CD-48a1-9AEA-71DE133CBE14}
//
#define STATIC_PINNAME_BDA_IF_PIN \
    0x1a9d4a42, 0xf3cd, 0x48a1, 0x9a, 0xea, 0x71, 0xde, 0x13, 0x3c, 0xbe, 0x14
DEFINE_GUIDSTRUCT("1A9D4A42-F3CD-48a1-9AEA-71DE133CBE14", PINNAME_BDA_IF_PIN);
#define PINNAME_BDA_IF_PIN DEFINE_GUIDNAMED(PINNAME_BDA_IF_PIN)


//  Pin name for a BDA Open Cable PSIP pin
//
// {297BB104-E5C9-4ACE-B123-95C3CBB24D4F}
//
#define STATIC_PINNAME_BDA_OPENCABLE_PSIP_PIN \
    0x297bb104, 0xe5c9, 0x4ace, 0xb1, 0x23, 0x95, 0xc3, 0xcb, 0xb2, 0x4d, 0x4f
DEFINE_GUIDSTRUCT("297BB104-E5C9-4ACE-B123-95C3CBB24D4F", PINNAME_BDA_OPENCABLE_PSIP_PIN);
#define PINNAME_BDA_OPENCABLE_PSIP_PIN DEFINE_GUIDNAMED(PINNAME_BDA_OPENCABLE_PSIP_PIN)


//===========================================================================
//
//  KSProperty Set Definitions for BDA
//
//===========================================================================


//------------------------------------------------------------
//
//  BDA Network Ethernet Filter Property Set
//
// {71985F43-1CA1-11d3-9CC8-00C04F7971E0}
//
#define STATIC_KSPROPSETID_BdaEthernetFilter \
    0x71985f43, 0x1ca1, 0x11d3, 0x9c, 0xc8, 0x0, 0xc0, 0x4f, 0x79, 0x71, 0xe0
DEFINE_GUIDSTRUCT("71985F43-1CA1-11d3-9CC8-00C04F7971E0", KSPROPSETID_BdaEthernetFilter);
#define KSPROPSETID_BdaEthernetFilter DEFINE_GUIDNAMED(KSPROPSETID_BdaEthernetFilter)

typedef enum {
    KSPROPERTY_BDA_ETHERNET_FILTER_MULTICAST_LIST_SIZE = 0,
    KSPROPERTY_BDA_ETHERNET_FILTER_MULTICAST_LIST,
    KSPROPERTY_BDA_ETHERNET_FILTER_MULTICAST_MODE
} KSPROPERTY_BDA_ETHERNET_FILTER;

#define DEFINE_KSPROPERTY_ITEM_BDA_ETHERNET_FILTER_MULTICAST_LIST_SIZE(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_ETHERNET_FILTER_MULTICAST_LIST_SIZE,\
        (GetHandler),\
        sizeof(KSPROPERTY),\
        sizeof(ULONG),\
        FALSE,\
        NULL, 0, NULL, NULL, 0)

#define DEFINE_KSPROPERTY_ITEM_BDA_ETHERNET_FILTER_MULTICAST_LIST(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_ETHERNET_FILTER_MULTICAST_LIST,\
        (GetHandler),\
        sizeof(KSPROPERTY),\
        sizeof(BDA_ETHERNET_ADDRESS_LIST),\
        (SetHandler),\
        NULL, 0, NULL, NULL, 0)

#define DEFINE_KSPROPERTY_ITEM_BDA_ETHERNET_FILTER_MULTICAST_MODE(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_ETHERNET_FILTER_MULTICAST_MODE,\
        (GetHandler),\
        sizeof(KSPROPERTY),\
        sizeof(BDA_MULTICAST_MODE),\
        (SetHandler),\
        NULL, 0, NULL, NULL, 0)



//------------------------------------------------------------
//
//  BDA Network IPv4 Filter Property Set
//
// {71985F44-1CA1-11d3-9CC8-00C04F7971E0}
//
#define STATIC_KSPROPSETID_BdaIPv4Filter \
    0x71985f44, 0x1ca1, 0x11d3, 0x9c, 0xc8, 0x0, 0xc0, 0x4f, 0x79, 0x71, 0xe0
DEFINE_GUIDSTRUCT("71985F44-1CA1-11d3-9CC8-00C04F7971E0", KSPROPSETID_BdaIPv4Filter);
#define KSPROPSETID_BdaIPv4Filter DEFINE_GUIDNAMED(KSPROPSETID_BdaIPv4Filter)

typedef enum {
    KSPROPERTY_BDA_IPv4_FILTER_MULTICAST_LIST_SIZE = 0,
    KSPROPERTY_BDA_IPv4_FILTER_MULTICAST_LIST,
    KSPROPERTY_BDA_IPv4_FILTER_MULTICAST_MODE
} KSPROPERTY_BDA_IPv4_FILTER;

#define DEFINE_KSPROPERTY_ITEM_BDA_IPv4_FILTER_MULTICAST_LIST_SIZE(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_IPv4_FILTER_MULTICAST_LIST_SIZE,\
        (GetHandler),\
        sizeof(KSPROPERTY),\
        sizeof(ULONG),\
        FALSE,\
        NULL, 0, NULL, NULL, 0)

#define DEFINE_KSPROPERTY_ITEM_BDA_IPv4_FILTER_MULTICAST_LIST(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_IPv4_FILTER_MULTICAST_LIST,\
        (GetHandler),\
        sizeof(KSPROPERTY),\
        sizeof(BDA_IPv4_ADDRESS_LIST),\
        (SetHandler),\
        NULL, 0, NULL, NULL, 0)

#define DEFINE_KSPROPERTY_ITEM_BDA_IPv4_FILTER_MULTICAST_MODE(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_IPv4_FILTER_MULTICAST_MODE,\
        (GetHandler),\
        sizeof(KSPROPERTY),\
        sizeof(BDA_MULTICAST_MODE),\
        (SetHandler),\
        NULL, 0, NULL, NULL, 0)



//------------------------------------------------------------
//
//  BDA Network IPv6 Filter Property Set
//
// {E1785A74-2A23-4fb3-9245-A8F88017EF33}
//
#define STATIC_KSPROPSETID_BdaIPv6Filter \
    0xe1785a74, 0x2a23, 0x4fb3, 0x92, 0x45, 0xa8, 0xf8, 0x80, 0x17, 0xef, 0x33
DEFINE_GUIDSTRUCT("E1785A74-2A23-4fb3-9245-A8F88017EF33", KSPROPSETID_BdaIPv6Filter);
#define KSPROPSETID_BdaIPv6Filter DEFINE_GUIDNAMED(KSPROPSETID_BdaIPv6Filter)

typedef enum {
    KSPROPERTY_BDA_IPv6_FILTER_MULTICAST_LIST_SIZE = 0,
    KSPROPERTY_BDA_IPv6_FILTER_MULTICAST_LIST,
    KSPROPERTY_BDA_IPv6_FILTER_MULTICAST_MODE
} KSPROPERTY_BDA_IPv6_FILTER;

#define DEFINE_KSPROPERTY_ITEM_BDA_IPv6_FILTER_MULTICAST_LIST_SIZE(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_IPv6_FILTER_MULTICAST_LIST_SIZE,\
        (GetHandler),\
        sizeof(KSPROPERTY),\
        sizeof(ULONG),\
        FALSE,\
        NULL, 0, NULL, NULL, 0)

#define DEFINE_KSPROPERTY_ITEM_BDA_IPv6_FILTER_MULTICAST_LIST(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_IPv6_FILTER_MULTICAST_LIST,\
        (GetHandler),\
        sizeof(KSPROPERTY),\
        sizeof(BDA_IPv6_ADDRESS_LIST),\
        (SetHandler),\
        NULL, 0, NULL, NULL, 0)

#define DEFINE_KSPROPERTY_ITEM_BDA_IPv6_FILTER_MULTICAST_MODE(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_IPv6_FILTER_MULTICAST_MODE,\
        (GetHandler),\
        sizeof(KSPROPERTY),\
        sizeof(BDA_MULTICAST_MODE),\
        (SetHandler),\
        NULL, 0, NULL, NULL, 0)


//------------------------------------------------------------
//
//
//  BDA Signal Statistics Property Set
//
//  Used to get signal statistics from a control node or a pin.
//  Set NodeId == -1 to get properties from the pin.
//
//  {1347D106-CF3A-428a-A5CB-AC0D9A2A4338}
//
#define STATIC_KSPROPSETID_BdaSignalStats \
    0x1347d106, 0xcf3a, 0x428a, 0xa5, 0xcb, 0xac, 0xd, 0x9a, 0x2a, 0x43, 0x38
DEFINE_GUIDSTRUCT("1347D106-CF3A-428a-A5CB-AC0D9A2A4338", KSPROPSETID_BdaSignalStats);
#define KSPROPSETID_BdaSignalStats DEFINE_GUIDNAMED(KSPROPSETID_BdaSignalStats)

typedef enum {
    KSPROPERTY_BDA_SIGNAL_STRENGTH = 0,
    KSPROPERTY_BDA_SIGNAL_QUALITY,
    KSPROPERTY_BDA_SIGNAL_PRESENT,
    KSPROPERTY_BDA_SIGNAL_LOCKED,
    KSPROPERTY_BDA_SAMPLE_TIME,
    KSPROPERTY_BDA_SIGNAL_LOCK_CAPS,
    KSPROPERTY_BDA_SIGNAL_LOCK_TYPE
} KSPROPERTY_BDA_SIGNAL_STATS;

typedef enum _BdaLockType {
    Bda_LockType_None          = 0x00,
    Bda_LockType_PLL           = 0x01,
    Bda_LockType_DecoderDemod  = 0x02,
    Bda_LockType_Complete      = 0x80
} BDA_LockType;

//  OPTIONAL
//  Carrier strength in mDb (1/1000 of a DB).
//
//  A strength of 0 is nominal strength as expected for the given
//  type of broadcast network.
//
//  Sub-nominal strengths are reported as positive mDb
//
//  Super-nominal strengths are reported as negative mDb
//
#define DEFINE_KSPROPERTY_ITEM_BDA_SIGNAL_STRENGTH(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_SIGNAL_STRENGTH,\
        (GetHandler),\
        sizeof(KSP_NODE),\
        sizeof(LONG),\
        (SetHandler),\
        NULL, 0, NULL, NULL, 0)

//  OPTIONAL
//  Amount of data successfully extracted from the signal as a percent.
//
//  Signal Quality is usually reported by the demodulation node and is
//  a representation of how much of the original data could be extracted
//  from the signal.
//
//  In the case of Analog Signals, this percentage can be
//  computed by examining the timing of HSync and VSync as will as by
//  looking at information contained in HBlanking and VBlanking intervals.
//
//  In the case of Digital Signals, this percentage can be
//  computed by examining packet CRCs and FEC confidence values.
//
//  100 percent is ideal.
//  95 percent shows very little (almost unnoticable) artifacts when rendered.
//  90 percent contains few enough artifacts as to be easily viewable.
//  80 percent is the minimum level to be viewable.
//  60 percent is the minimum level to expect data services
//  (including EPG) to work.
//  20 percent indicates that the demodulator knows that a properly modulated
//  signal exists but can't produce enough data to be useful.
//
#define DEFINE_KSPROPERTY_ITEM_BDA_SIGNAL_QUALITY(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_SIGNAL_QUALITY,\
        (GetHandler),\
        sizeof(KSP_NODE),\
        sizeof(LONG),\
        (SetHandler),\
        NULL, 0, NULL, NULL, 0)

//  REQUIRED
//  True if a signal carrier is present.
//
//  Should be returned by the RF tuner node.
//
#define DEFINE_KSPROPERTY_ITEM_BDA_SIGNAL_PRESENT(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_SIGNAL_PRESENT,\
        (GetHandler),\
        sizeof(KSP_NODE),\
        sizeof(BOOL),\
        (SetHandler),\
        NULL, 0, NULL, NULL, 0)

//  REQUIRED
//  True if the signal can be locked.
//
//  Ususally represents PLL lock when returned by the RF Tuner Node.
//
//  Represents Signal Quality of at least 20% when returned by the
//  demodulator node.
//
#define DEFINE_KSPROPERTY_ITEM_BDA_SIGNAL_LOCKED(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_SIGNAL_LOCKED,\
        (GetHandler),\
        sizeof(KSP_NODE),\
        sizeof(BOOL),\
        (SetHandler),\
        NULL, 0, NULL, NULL, 0)

//  OPTIONAL
//  Indicates the sample time overwhich signal level and quality are
//  averaged.
//
//  Each time a signal statistics property is requested, the node should
//  report the average value for the last n milliseconds where n is the
//  value set by this property.  If no value is set or if the driver does
//  not support this property, the driver should default to
//  100 millisecond sample times.
//
//  The driver may report values for the most recently completed sample
//  period.
//
#define DEFINE_KSPROPERTY_ITEM_BDA_SAMPLE_TIME(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_SAMPLE_TIME,\
        (GetHandler),\
        sizeof(KSP_NODE),\
        sizeof(LONG),\
        (SetHandler),\
        NULL, 0, NULL, NULL, 0)


//  REQUIRED
//  returns a bitmask of supported BDA_LockType_ values
//
//  Should be returned by the RF tuner node.
//
#define DEFINE_KSPROPERTY_ITEM_BDA_SIGNAL_LOCK_CAPS(GetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_SIGNAL_LOCK_CAPS,\
        (GetHandler),\
        sizeof(KSP_NODE),\
        sizeof(BOOL),\
        NULL,\
        NULL, 0, NULL, NULL, 0)

//  REQUIRED
//  returns current BDA_LockType_ value
//
//  Should be returned by the RF tuner node.
//
#define DEFINE_KSPROPERTY_ITEM_BDA_SIGNAL_LOCK_TYPE(GetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_SIGNAL_LOCK_TYPE,\
        (GetHandler),\
        sizeof(KSP_NODE),\
        sizeof(BDA_LockType),\
        NULL,\
        NULL, 0, NULL, NULL, 0)

//------------------------------------------------------------
//
//
//  BDA Change Sync Method Set
//
// {FD0A5AF3-B41D-11d2-9C95-00C04F7971E0}
//
#define STATIC_KSMETHODSETID_BdaChangeSync \
    0xfd0a5af3, 0xb41d, 0x11d2, 0x9c, 0x95, 0x0, 0xc0, 0x4f, 0x79, 0x71, 0xe0
DEFINE_GUIDSTRUCT("FD0A5AF3-B41D-11d2-9C95-00C04F7971E0", KSMETHODSETID_BdaChangeSync);
#define KSMETHODSETID_BdaChangeSync DEFINE_GUIDNAMED(KSMETHODSETID_BdaChangeSync)

typedef enum {
    KSMETHOD_BDA_START_CHANGES = 0,
    KSMETHOD_BDA_CHECK_CHANGES,
    KSMETHOD_BDA_COMMIT_CHANGES,
    KSMETHOD_BDA_GET_CHANGE_STATE
} KSMETHOD_BDA_CHANGE_SYNC;

#define DEFINE_KSMETHOD_ITEM_BDA_START_CHANGES(MethodHandler, SupportHandler)\
    DEFINE_KSMETHOD_ITEM(\
        KSMETHOD_BDA_START_CHANGES,\
        KSMETHOD_TYPE_NONE,\
        (MethodHandler),\
        sizeof(KSMETHOD),\
        0,\
        SupportHandler)

#define DEFINE_KSMETHOD_ITEM_BDA_CHECK_CHANGES(MethodHandler, SupportHandler)\
    DEFINE_KSMETHOD_ITEM(\
        KSMETHOD_BDA_CHECK_CHANGES,\
        KSMETHOD_TYPE_NONE,\
        (MethodHandler),\
        sizeof(KSMETHOD),\
        0,\
        SupportHandler)

#define DEFINE_KSMETHOD_ITEM_BDA_COMMIT_CHANGES(MethodHandler, SupportHandler)\
    DEFINE_KSMETHOD_ITEM(\
        KSMETHOD_BDA_COMMIT_CHANGES,\
        KSMETHOD_TYPE_NONE,\
        (MethodHandler),\
        sizeof(KSMETHOD),\
        0,\
        SupportHandler)

#define DEFINE_KSMETHOD_ITEM_BDA_GET_CHANGE_STATE(MethodHandler, SupportHandler)\
    DEFINE_KSMETHOD_ITEM(\
        KSMETHOD_BDA_GET_CHANGE_STATE,\
        KSMETHOD_TYPE_READ,\
        (MethodHandler),\
        sizeof(KSMETHOD),\
        0,\
        SupportHandler)



//------------------------------------------------------------
//
//
//  BDA Device Configuration Method Set
//
// {71985F45-1CA1-11d3-9CC8-00C04F7971E0}
//
#define STATIC_KSMETHODSETID_BdaDeviceConfiguration \
    0x71985f45, 0x1ca1, 0x11d3, 0x9c, 0xc8, 0x0, 0xc0, 0x4f, 0x79, 0x71, 0xe0
DEFINE_GUIDSTRUCT("71985F45-1CA1-11d3-9CC8-00C04F7971E0", KSMETHODSETID_BdaDeviceConfiguration);
#define KSMETHODSETID_BdaDeviceConfiguration DEFINE_GUIDNAMED(KSMETHODSETID_BdaDeviceConfiguration)

typedef enum {
    KSMETHOD_BDA_CREATE_PIN_FACTORY = 0,
    KSMETHOD_BDA_DELETE_PIN_FACTORY,
    KSMETHOD_BDA_CREATE_TOPOLOGY
} KSMETHOD_BDA_DEVICE_CONFIGURATION;

#define DEFINE_KSMETHOD_ITEM_BDA_CREATE_PIN_FACTORY(MethodHandler, SupportHandler)\
    DEFINE_KSMETHOD_ITEM(\
        KSMETHOD_BDA_CREATE_PIN_FACTORY,\
        KSMETHOD_TYPE_READ,\
        (MethodHandler),\
        sizeof(KSM_BDA_PIN),\
        sizeof(ULONG),\
        SupportHandler)

#define DEFINE_KSMETHOD_ITEM_BDA_DELETE_PIN_FACTORY(MethodHandler, SupportHandler)\
    DEFINE_KSMETHOD_ITEM(\
        KSMETHOD_BDA_DELETE_PIN_FACTORY,\
        KSMETHOD_TYPE_NONE,\
        (MethodHandler),\
        sizeof(KSM_BDA_PIN),\
        0,\
        SupportHandler)

#define DEFINE_KSMETHOD_ITEM_BDA_CREATE_TOPOLOGY(MethodHandler, SupportHandler)\
    DEFINE_KSMETHOD_ITEM(\
        KSMETHOD_BDA_CREATE_TOPOLOGY,\
        KSMETHOD_TYPE_WRITE,\
        (MethodHandler),\
        sizeof(KSM_BDA_PIN_PAIR),\
        0,\
        SupportHandler)



//------------------------------------------------------------
//
//
//  BDA Topology Property Set
//
// {A14EE835-0A23-11d3-9CC7-00C04F7971E0}
//
#define STATIC_KSPROPSETID_BdaTopology \
    0xa14ee835, 0x0a23, 0x11d3, 0x9c, 0xc7, 0x0, 0xc0, 0x4f, 0x79, 0x71, 0xe0
DEFINE_GUIDSTRUCT("A14EE835-0A23-11d3-9CC7-00C04F7971E0", KSPROPSETID_BdaTopology);
#define KSPROPSETID_BdaTopology DEFINE_GUIDNAMED(KSPROPSETID_BdaTopology)

typedef enum {
    KSPROPERTY_BDA_NODE_TYPES,
    KSPROPERTY_BDA_PIN_TYPES,
    KSPROPERTY_BDA_TEMPLATE_CONNECTIONS,
    KSPROPERTY_BDA_NODE_METHODS,
    KSPROPERTY_BDA_NODE_PROPERTIES,
    KSPROPERTY_BDA_NODE_EVENTS,
    KSPROPERTY_BDA_CONTROLLING_PIN_ID,
    KSPROPERTY_BDA_NODE_DESCRIPTORS
 }KSPROPERTY_BDA_TOPOLOGY;

#define DEFINE_KSPROPERTY_ITEM_BDA_NODE_TYPES(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_NODE_TYPES,\
        (GetHandler),\
        sizeof(KSPROPERTY),\
        0,\
        FALSE,\
        NULL, 0, NULL, NULL, 0)

#define DEFINE_KSPROPERTY_ITEM_BDA_PIN_TYPES(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_PIN_TYPES,\
        (GetHandler),\
        sizeof(KSPROPERTY),\
        0,\
        FALSE,\
        NULL, 0, NULL, NULL, 0)

#define DEFINE_KSPROPERTY_ITEM_BDA_TEMPLATE_CONNECTIONS(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_TEMPLATE_CONNECTIONS,\
        (GetHandler),\
        sizeof(KSPROPERTY),\
        sizeof( BDA_TEMPLATE_CONNECTION),\
        FALSE,\
        NULL, 0, NULL, NULL, 0)

#define DEFINE_KSPROPERTY_ITEM_BDA_NODE_METHODS(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_NODE_METHODS,\
        (GetHandler),\
        sizeof(KSP_NODE),\
        0,\
        FALSE,\
        NULL, 0, NULL, NULL, 0)

#define DEFINE_KSPROPERTY_ITEM_BDA_NODE_PROPERTIES(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_NODE_PROPERTIES,\
        (GetHandler),\
        sizeof(KSP_NODE),\
        0,\
        FALSE,\
        NULL, 0, NULL, NULL, 0)

#define DEFINE_KSPROPERTY_ITEM_BDA_NODE_EVENTS(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_NODE_EVENTS,\
        (GetHandler),\
        sizeof(KSP_NODE),\
        0,\
        FALSE,\
        NULL, 0, NULL, NULL, 0)

#define DEFINE_KSPROPERTY_ITEM_BDA_CONTROLLING_PIN_ID(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_CONTROLLING_PIN_ID,\
        (GetHandler),\
        sizeof(KSP_BDA_NODE_PIN),\
        sizeof( ULONG),\
        FALSE,\
        NULL, 0, NULL, NULL, 0)

#define DEFINE_KSPROPERTY_ITEM_BDA_NODE_DESCRIPTORS(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_NODE_DESCRIPTORS,\
        (GetHandler),\
        sizeof(KSPROPERTY),\
        0,\
        FALSE,\
        NULL, 0, NULL, NULL, 0)



//------------------------------------------------------------
//
//
//  BDA Pin Control Property Set
//
// {0DED49D5-A8B7-4d5d-97A1-12B0C195874D}
//
#define STATIC_KSPROPSETID_BdaPinControl \
    0xded49d5, 0xa8b7, 0x4d5d, 0x97, 0xa1, 0x12, 0xb0, 0xc1, 0x95, 0x87, 0x4d
DEFINE_GUIDSTRUCT("0DED49D5-A8B7-4d5d-97A1-12B0C195874D", KSPROPSETID_BdaPinControl);
#define KSPROPSETID_BdaPinControl DEFINE_GUIDNAMED(KSPROPSETID_BdaPinControl)

typedef enum {
    KSPROPERTY_BDA_PIN_ID = 0,
    KSPROPERTY_BDA_PIN_TYPE
} KSPROPERTY_BDA_PIN_CONTROL;

#define DEFINE_KSPROPERTY_ITEM_BDA_PIN_ID(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_PIN_ID,\
        (GetHandler),\
        sizeof( KSPROPERTY),\
        sizeof( ULONG),\
        FALSE,\
        NULL, 0, NULL, NULL, 0)

#define DEFINE_KSPROPERTY_ITEM_BDA_PIN_TYPE(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_PIN_TYPE,\
        (GetHandler),\
        sizeof( KSPROPERTY),\
        sizeof( ULONG),\
        FALSE,\
        NULL, 0, NULL, NULL, 0)



//------------------------------------------------------------
//
//
//  BDA Pin Event Set
//
// {104781CD-50BD-40d5-95FB-087E0E86A591}
//
#define STATIC_KSEVENTSETID_BdaPinEvent \
    0x104781cd, 0x50bd, 0x40d5, 0x95, 0xfb, 0x08, 0x7e, 0xe, 0x86, 0xa5, 0x91
DEFINE_GUIDSTRUCT("104781CD-50BD-40d5-95FB-087E0E86A591", KSEVENTSETID_BdaPinEvent);
#define KSEVENTSETID_BdaPinEvent DEFINE_GUIDNAMED(KSEVENTSETID_BdaPinEvent)

typedef enum {
    KSEVENT_BDA_PIN_CONNECTED = 0,
    KSEVENT_BDA_PIN_DISCONNECTED
} KSPROPERTY_BDA_PIN_EVENT;

#define DEFINE_KSEVENT_ITEM_BDA_PIN_CONNECTED(AddHandler, RemoveHandler, SupportHandler)\
    DEFINE_KSEVENT_ITEM(\
        KSEVENT_BDA_PIN_CONNECTED,\
        sizeof( KSEVENTDATA), \
        0, \
        (AddHandler),\
        (RemoveHandler),\
        (SupportHandler)\
        )

#define DEFINE_KSEVENT_ITEM_BDA_PIN_DISCONNECTED(AddHandler, RemoveHandler, SupportHandler)\
    DEFINE_KSEVENT_ITEM(\
        KSEVENT_BDA_PIN_DISCONNECTED,\
        sizeof( KSEVENTDATA), \
        0, \
        (AddHandler),\
        (RemoveHandler),\
        (SupportHandler)\
        )



//------------------------------------------------------------
//
//
//  BDA Void Transform Property Set
//
// {71985F46-1CA1-11d3-9CC8-00C04F7971E0}
//
#define STATIC_KSPROPSETID_BdaVoidTransform \
    0x71985f46, 0x1ca1, 0x11d3, 0x9c, 0xc8, 0x0, 0xc0, 0x4f, 0x79, 0x71, 0xe0
DEFINE_GUIDSTRUCT("71985F46-1CA1-11d3-9CC8-00C04F7971E0", KSPROPSETID_BdaVoidTransform);
#define KSPROPSETID_BdaVoidTransform DEFINE_GUIDNAMED(KSPROPSETID_BdaVoidTransform)

typedef enum {
    KSPROPERTY_BDA_VOID_TRANSFORM_START = 0,
    KSPROPERTY_BDA_VOID_TRANSFORM_STOP
} KSPROPERTY_BDA_VOID_TRANSFORM;

#define DEFINE_KSPROPERTY_ITEM_BDA_VOID_TRANSFORM_START(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_VOID_TRANSFORM_START,\
        FALSE,\
        sizeof(KSPROPERTY),\
        0,\
        (SetHandler),\
        NULL, 0, NULL, NULL, 0)

#define DEFINE_KSPROPERTY_ITEM_BDA_VOID_TRANSFORM_STOP(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_VOID_TRANSFORM_STOP,\
        FALSE,\
        sizeof(KSPROPERTY),\
        0,\
        (SetHandler),\
        NULL, 0, NULL, NULL, 0)



//------------------------------------------------------------
//
//
//  BDA Null Transform Property Set
//
// {DDF15B0D-BD25-11d2-9CA0-00C04F7971E0}
//
#define STATIC_KSPROPSETID_BdaNullTransform \
    0xddf15b0d, 0xbd25, 0x11d2, 0x9c, 0xa0, 0x0, 0xc0, 0x4f, 0x79, 0x71, 0xe0
DEFINE_GUIDSTRUCT("DDF15B0D-BD25-11d2-9CA0-00C04F7971E0", KSPROPSETID_BdaNullTransform);
#define KSPROPSETID_BdaNullTransform DEFINE_GUIDNAMED(KSPROPSETID_BdaNullTransform)

typedef enum {
    KSPROPERTY_BDA_NULL_TRANSFORM_START = 0,
    KSPROPERTY_BDA_NULL_TRANSFORM_STOP
} KSPROPERTY_BDA_NULL_TRANSFORM;

#define DEFINE_KSPROPERTY_ITEM_BDA_NULL_TRANSFORM_START(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_NULL_TRANSFORM_START,\
        FALSE,\
        sizeof(KSPROPERTY),\
        0,\
        (SetHandler),\
        NULL, 0, NULL, NULL, 0)

#define DEFINE_KSPROPERTY_ITEM_BDA_NULL_TRANSFORM_STOP(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_NULL_TRANSFORM_STOP,\
        FALSE,\
        sizeof(KSPROPERTY),\
        0,\
        (SetHandler),\
        NULL, 0, NULL, NULL, 0)



//------------------------------------------------------------
//
//
//  BDA Frequency Filter Property Set
//
// {71985F47-1CA1-11d3-9CC8-00C04F7971E0}
//
#define STATIC_KSPROPSETID_BdaFrequencyFilter \
    0x71985f47, 0x1ca1, 0x11d3, 0x9c, 0xc8, 0x0, 0xc0, 0x4f, 0x79, 0x71, 0xe0
DEFINE_GUIDSTRUCT("71985F47-1CA1-11d3-9CC8-00C04F7971E0", KSPROPSETID_BdaFrequencyFilter);
#define KSPROPSETID_BdaFrequencyFilter DEFINE_GUIDNAMED(KSPROPSETID_BdaFrequencyFilter)

typedef enum {
    KSPROPERTY_BDA_RF_TUNER_FREQUENCY = 0,
    KSPROPERTY_BDA_RF_TUNER_POLARITY,
    KSPROPERTY_BDA_RF_TUNER_RANGE,
    KSPROPERTY_BDA_RF_TUNER_TRANSPONDER,
    KSPROPERTY_BDA_RF_TUNER_BANDWIDTH,
    KSPROPERTY_BDA_RF_TUNER_FREQUENCY_MULTIPLIER,
    KSPROPERTY_BDA_RF_TUNER_CAPS,
    KSPROPERTY_BDA_RF_TUNER_SCAN_STATUS,
    KSPROPERTY_BDA_RF_TUNER_STANDARD,
    KSPROPERTY_BDA_RF_TUNER_STANDARD_MODE
} KSPROPERTY_BDA_FREQUENCY_FILTER;

typedef enum _BdaSignalType {
    Bda_SignalType_Unknown =  0,
    Bda_SignalType_Analog  =  1,
    Bda_SignalType_Digital =  2
} BDA_SignalType;

// 
// The BDA_DigitalSignalStandard enumeration is tentatively put out for Beta review
// Based on feedback, this may be updated or entirely removed in a later release
//
typedef enum
{
    Bda_DigitalStandard_None     = 0x00000000,  
    Bda_DigitalStandard_DVB_T    = 0x00000001,
    Bda_DigitalStandard_DVB_S    = 0x00000002,
    Bda_DigitalStandard_DVB_C    = 0x00000004,
    Bda_DigitalStandard_ATSC     = 0x00000008,
    Bda_DigitalStandard_ISDB_T   = 0x00000010,
    Bda_DigitalStandard_ISDB_S   = 0x00000020,
    Bda_DigitalStandard_ISDB_C   = 0x00000040
} BDA_DigitalSignalStandard;

typedef struct {
    KSP_NODE Property;
    ULONG  Mode;                        // IN: KSPROPERTY_TUNER_MODE
    ULONG  AnalogStandardsSupported;    // Bda_AnalogStandard_* (if TV or DSS)
    ULONG  DigitalStandardsSupported;   // Bda_DigitalStandard_*
    ULONG  MinFrequency;                // R - Hz
    ULONG  MaxFrequency;                // R - Hz
    ULONG  SettlingTime;                // R - milliSeconds to settle for any sort of update to the tuner
    ULONG  AnalogSensingRange;          // R - max range (kHz) in which tuner can detect an analog signal 
    ULONG  DigitalSensingRange;         // R - max range (kHz) in which tuner can detect an digital signal
    ULONG  MilliSecondsPerMHz;          // R - max time to lock in to a signal 1MHz away assuming the signal has been detected, but its offset is unknown
} KSPROPERTY_BDA_RF_TUNER_CAPS_S, *PKSPROPERTY_BDA_RF_TUNER_CAPS_S;

typedef struct {
    KSP_NODE Property;
    ULONG CurrentFrequency;          // R - current frequency
    ULONG FrequencyRangeMin;         // R - lower limit of range left to scan
    ULONG FrequencyRangeMax;         // R - upper limit of range left to scan
    ULONG MilliSecondsLeft;          // R - time left to complete scanning
} KSPROPERTY_BDA_RF_TUNER_SCAN_STATUS_S, *PKSPROPERTY_BDA_RF_TUNER_SCAN_STATUS_S;

typedef struct {
    KSP_NODE Property;
    BDA_SignalType SignalType;       // RW - specifies whether the signal is Analog or Digital. this is required to interpret the SignalStandard field
    ULONG  SignalStandard;           // RW - current signal standard (KS_AnalogVideo_* or Bda_DigitalStandard_*) set by the application or detected by the device
} KSPROPERTY_BDA_RF_TUNER_STANDARD_S, *PKSPROPERTY_BDA_RF_TUNER_STANDARD_S;

typedef struct {
    KSP_NODE Property;
    BOOL AutoDetect;                 // RW - specifies whether the driver is in auto-detect mode for the signal standard
} KSPROPERTY_BDA_RF_TUNER_STANDARD_MODE_S, *PKSPROPERTY_BDA_RF_TUNER_STANDARD_MODE_S;

#define DEFINE_KSPROPERTY_ITEM_BDA_RF_TUNER_FREQUENCY(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_RF_TUNER_FREQUENCY,\
        (GetHandler),\
        sizeof(KSP_NODE),\
        sizeof(ULONG),\
        (SetHandler),\
        NULL, 0, NULL, NULL, 0)

#define DEFINE_KSPROPERTY_ITEM_BDA_RF_TUNER_POLARITY(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_RF_TUNER_POLARITY,\
        (GetHandler),\
        sizeof(KSP_NODE),\
        sizeof(ULONG),\
        (SetHandler),\
        NULL, 0, NULL, NULL, 0)

#define DEFINE_KSPROPERTY_ITEM_BDA_RF_TUNER_RANGE(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_RF_TUNER_RANGE,\
        (GetHandler),\
        sizeof(KSP_NODE),\
        sizeof(ULONG),\
        (SetHandler),\
        NULL, 0, NULL, NULL, 0)

#define DEFINE_KSPROPERTY_ITEM_BDA_RF_TUNER_TRANSPONDER(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_RF_TUNER_TRANSPONDER,\
        (GetHandler),\
        sizeof(KSP_NODE),\
        sizeof(ULONG),\
        (SetHandler),\
        NULL, 0, NULL, NULL, 0)

#define DEFINE_KSPROPERTY_ITEM_BDA_RF_TUNER_BANDWIDTH(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_RF_TUNER_BANDWIDTH,\
        (GetHandler),\
        sizeof(KSP_NODE),\
        sizeof(ULONG),\
        (SetHandler),\
        NULL, 0, NULL, NULL, 0)

#define DEFINE_KSPROPERTY_ITEM_BDA_RF_TUNER_FREQUENCY_MULTIPLIER(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_RF_TUNER_FREQUENCY_MULTIPLIER,\
        (GetHandler),\
        sizeof(KSP_NODE),\
        sizeof(ULONG),\
        (SetHandler),\
        NULL, 0, NULL, NULL, 0)

//------------------------------------------------------------
//
//
//  BDA Tuner Event Set
//
// {AAB59E17-01C9-4ebf-93F2-FC3B79B46F91}
//
#define STATIC_KSEVENTSETID_BdaTunerEvent \
    0xaab59e17, 0x1c9, 0x4ebf, 0x93, 0xf2, 0xfc, 0x3b, 0x79, 0xb4, 0x6f, 0x91
DEFINE_GUIDSTRUCT("AAB59E17-01C9-4ebf-93F2-FC3B79B46F91", KSEVENTSETID_BdaTunerEvent);
#define KSEVENTSETID_BdaTunerEvent DEFINE_GUIDNAMED(KSEVENTSETID_BdaTunerEvent)

typedef enum {
    KSEVENT_BDA_TUNER_SCAN = 0
} KSEVENT_BDA_TUNER;

typedef struct {
    KSEVENTDATA EventData;
    ULONG StartFrequency;          // W - initial frequency for the scan
    ULONG EndFrequency;            // W - final frequency for the scan, can be less than the initial value indicating a "scan down" is requested
    BDA_LockType LockRequested;    // W - whether the device should look for just a PLL lock, decoder lock, etc. should be a supported lock type.
} KSEVENTDATA_BDA_RF_TUNER_SCAN_S, *PKSEVENTDATA_BDA_RF_TUNER_SCAN_S;

//------------------------------------------------------------
//
//
//  BDA LNB Info Property Set
//
// {992CF102-49F9-4719-A664-C4F23E2408F4}
//
#define STATIC_KSPROPSETID_BdaLNBInfo \
    0x992cf102, 0x49f9, 0x4719, 0xa6, 0x64, 0xc4, 0xf2, 0x3e, 0x24, 0x8, 0xf4
DEFINE_GUIDSTRUCT("992CF102-49F9-4719-A664-C4F23E2408F4", KSPROPSETID_BdaLNBInfo);
#define KSPROPSETID_BdaLNBInfo DEFINE_GUIDNAMED(KSPROPSETID_BdaLNBInfo)

typedef enum {
    KSPROPERTY_BDA_LNB_LOF_LOW_BAND = 0,
    KSPROPERTY_BDA_LNB_LOF_HIGH_BAND,
    KSPROPERTY_BDA_LNB_SWITCH_FREQUENCY
} KSPROPERTY_BDA_LNB_INFO;

#define DEFINE_KSPROPERTY_ITEM_BDA_LNB_LOF_LOW_BAND(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_LNB_LOF_LOW_BAND,\
        (GetHandler),\
        sizeof(KSP_NODE),\
        sizeof(ULONG),\
        (SetHandler),\
        NULL, 0, NULL, NULL, 0)

#define DEFINE_KSPROPERTY_ITEM_BDA_LNB_LOF_HIGH_BAND(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_LNB_LOF_HIGH_BAND,\
        (GetHandler),\
        sizeof(KSP_NODE),\
        sizeof(ULONG),\
        (SetHandler),\
        NULL, 0, NULL, NULL, 0)

#define DEFINE_KSPROPERTY_ITEM_BDA_LNB_SWITCH_FREQUENCY(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_LNB_SWITCH_FREQUENCY,\
        (GetHandler),\
        sizeof(KSP_NODE),\
        sizeof(ULONG),\
        (SetHandler),\
        NULL, 0, NULL, NULL, 0)

//------------------------------------------------------------
//
//DiseqC Specific commands and source selection
//DVB-S applications can use the commands to select there LNB Source
//or control there installation equipment (e.g. motor dish, switches) 
//
// {F84E2AB0-3C6B-45e3-A0FC-8669D4B81F11}
//
#define STATIC_KSPROPSETID_BdaDiseqCommand \
    0xf84e2ab0, 0x3c6b, 0x45e3, 0xa0, 0xfc, 0x86, 0x69, 0xd4, 0xb8, 0x1f, 0x11
DEFINE_GUIDSTRUCT("F84E2AB0-3C6B-45e3-A0FC-8669D4B81F11", KSPROPSETID_BdaDiseqCommand);
#define KSPROPSETID_BdaDiseqCommand DEFINE_GUIDNAMED(KSPROPSETID_BdaDiseqCommand)

typedef enum {
    KSPROPERTY_BDA_DISEQC_ENABLE = 0,
    KSPROPERTY_BDA_DISEQC_LNB_SOURCE,
    KSPROPERTY_BDA_DISEQC_USETONEBURST,
    KSPROPERTY_BDA_DISEQC_REPEATS,
    KSPROPERTY_BDA_DISEQC_SEND,
    KSPROPERTY_BDA_DISEQC_RESPONSE
} KSPROPERTY_BDA_DISEQC_COMMAND;

#define DEFINE_KSPROPERTY_ITEM_BDA_DISEQC_ENABLE(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
    KSPROPERTY_BDA_DISEQC_ENABLE,\
    (GetHandler),\
    sizeof(KSP_NODE),\
    sizeof(BOOL),\
    (SetHandler),\
    NULL, 0, NULL, NULL, 0)

#define DEFINE_KSPROPERTY_ITEM_BDA_DISEQC_LNB_SOURCE(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
    KSPROPERTY_BDA_DISEQC_LNB_SOURCE,\
    (GetHandler),\
    sizeof(KSP_NODE),\
    sizeof(ULONG),\
    (SetHandler),\
    NULL, 0, NULL, NULL, 0)

#define DEFINE_KSPROPERTY_ITEM_BDA_DISEQC_USETONEBURST(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
    KSPROPERTY_BDA_DISEQC_USETONEBURST,\
    (GetHandler),\
    sizeof(KSP_NODE),\
    sizeof(BOOL),\
    (SetHandler),\
    NULL, 0, NULL, NULL, 0)

#define DEFINE_KSPROPERTY_ITEM_BDA_DISEQC_REPEATS(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
    KSPROPERTY_BDA_DISEQC_REPEATS,\
    (GetHandler),\
    sizeof(KSP_NODE),\
    sizeof(ULONG),\
    (SetHandler),\
    NULL, 0, NULL, NULL, 0)

#define DEFINE_KSPROPERTY_ITEM_BDA_DISEQC_SEND(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
    KSPROPERTY_BDA_DISEQC_SEND,\
    (GetHandler),\
    sizeof(KSP_NODE),\
    sizeof(BDA_DISEQC_SEND),\
    (SetHandler),\
    NULL, 0, NULL, NULL, 0)

#define DEFINE_KSPROPERTY_ITEM_BDA_DISEQC_RESPONSE(GetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
    KSPROPERTY_BDA_DISEQC_RESPONSE,\
    (GetHandler),\
    sizeof(KSP_NODE),\
    sizeof(BDA_DISEQC_RESPONSE),\
    NULL,\
    NULL, 0, NULL, NULL, 0)

//------------------------------------------------------------
//
//
//  BDA DiseqC Event Set
//
// {8B19BBF0-4184-43ac-AD3C-0C889BE4C212}
//

#define STATIC_KSEVENTSETID_BdaDiseqCEvent \
    0x8b19bbf0, 0x4184, 0x43ac, 0xad, 0x3c, 0xc, 0x88, 0x9b, 0xe4, 0xc2, 0x12
DEFINE_GUIDSTRUCT("8B19BBF0-4184-43ac-AD3C-0C889BE4C212", KSEVENTSETID_BdaDiseqCEvent);
#define KSEVENTSETID_BdaDiseqCEvent DEFINE_GUIDNAMED(KSEVENTSETID_BdaDiseqCEvent)

typedef enum {
    KSEVENT_BDA_DISEQC_DATA_RECEIVED = 0
} KSPROPERTY_BDA_DISEQC_EVENT;

#define DEFINE_KSEVENT_BDA_DISEQC_DATA_RECEIVED(AddHandler, RemoveHandler, SupportHandler)\
    DEFINE_KSEVENT_ITEM(\
    KSEVENT_BDA_DISEQC_DATA_RECEIVED,\
    sizeof( KSEVENTDATA), \
    0, \
    (AddHandler),\
    (RemoveHandler),\
    (SupportHandler)\
    )

//------------------------------------------------------------
//
//
//  BDA Digital Demodulator Property Set
//
// {EF30F379-985B-4d10-B640-A79D5E04E1E0}
//
#define STATIC_KSPROPSETID_BdaDigitalDemodulator \
    0xef30f379, 0x985b, 0x4d10, 0xb6, 0x40, 0xa7, 0x9d, 0x5e, 0x4, 0xe1, 0xe0
DEFINE_GUIDSTRUCT("EF30F379-985B-4d10-B640-A79D5E04E1E0", KSPROPSETID_BdaDigitalDemodulator);
#define KSPROPSETID_BdaDigitalDemodulator DEFINE_GUIDNAMED(KSPROPSETID_BdaDigitalDemodulator)

typedef enum {
    KSPROPERTY_BDA_MODULATION_TYPE = 0,
    KSPROPERTY_BDA_INNER_FEC_TYPE,
    KSPROPERTY_BDA_INNER_FEC_RATE,
    KSPROPERTY_BDA_OUTER_FEC_TYPE,
    KSPROPERTY_BDA_OUTER_FEC_RATE,
    KSPROPERTY_BDA_SYMBOL_RATE,
    KSPROPERTY_BDA_SPECTRAL_INVERSION,
    KSPROPERTY_BDA_GUARD_INTERVAL,
    KSPROPERTY_BDA_TRANSMISSION_MODE,
    KSPROPERTY_BDA_ROLL_OFF,
    KSPROPERTY_BDA_PILOT,
    KSPROPERTY_BDA_SIGNALTIMEOUTS,
    KSPROPERTY_BDA_PLP_NUMBER
} KSPROPERTY_BDA_DIGITAL_DEMODULATOR;

#define DEFINE_KSPROPERTY_ITEM_BDA_MODULATION_TYPE(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_MODULATION_TYPE,\
        (GetHandler),\
        sizeof(KSP_NODE),\
        sizeof(ModulationType),\
        (SetHandler),\
        NULL, 0, NULL, NULL, 0)

#define DEFINE_KSPROPERTY_ITEM_BDA_INNER_FEC_TYPE(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_INNER_FEC_TYPE,\
        (GetHandler),\
        sizeof(KSP_NODE),\
        sizeof(FECMethod),\
        (SetHandler),\
        NULL, 0, NULL, NULL, 0)

#define DEFINE_KSPROPERTY_ITEM_BDA_INNER_FEC_RATE(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_INNER_FEC_RATE,\
        (GetHandler),\
        sizeof(KSP_NODE),\
        sizeof(BinaryConvolutionCodeRate),\
        (SetHandler),\
        NULL, 0, NULL, NULL, 0)

#define DEFINE_KSPROPERTY_ITEM_BDA_OUTER_FEC_TYPE(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_OUTER_FEC_TYPE,\
        (GetHandler),\
        sizeof(KSP_NODE),\
        sizeof(FECMethod),\
        (SetHandler),\
        NULL, 0, NULL, NULL, 0)

#define DEFINE_KSPROPERTY_ITEM_BDA_OUTER_FEC_RATE(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_OUTER_FEC_RATE,\
        (GetHandler),\
        sizeof(KSP_NODE),\
        sizeof(BinaryConvolutionCodeRate),\
        (SetHandler),\
        NULL, 0, NULL, NULL, 0)

#define DEFINE_KSPROPERTY_ITEM_BDA_SYMBOL_RATE(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_SYMBOL_RATE,\
        (GetHandler),\
        sizeof(KSP_NODE),\
        sizeof(ULONG),\
        (SetHandler),\
        NULL, 0, NULL, NULL, 0)

#define DEFINE_KSPROPERTY_ITEM_BDA_SPECTRAL_INVERSION(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_SPECTRAL_INVERSION,\
        (GetHandler),\
        sizeof(KSP_NODE),\
        sizeof(SpectralInversion),\
        (SetHandler),\
        NULL, 0, NULL, NULL, 0)

#define DEFINE_KSPROPERTY_ITEM_BDA_GUARD_INTERVAL(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_GUARD_INTERVAL,\
        (GetHandler),\
        sizeof(KSP_NODE),\
        sizeof(GuardInterval),\
        (SetHandler),\
        NULL, 0, NULL, NULL, 0)

#define DEFINE_KSPROPERTY_ITEM_BDA_TRANSMISSION_MODE(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_TRANSMISSION_MODE,\
        (GetHandler),\
        sizeof(KSP_NODE),\
        sizeof(TransmissionMode),\
        (SetHandler),\
        NULL, 0, NULL, NULL, 0)

#define DEFINE_KSPROPERTY_ITEM_BDA_ROLL_OFF(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_ROLL_OFF,\
        (GetHandler),\
        sizeof(KSP_NODE),\
        sizeof(RollOff),\
        (SetHandler),\
        NULL, 0, NULL, NULL, 0)

#define DEFINE_KSPROPERTY_ITEM_BDA_PILOT(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_PILOT,\
        (GetHandler),\
        sizeof(KSP_NODE),\
        sizeof(Pilot),\
        (SetHandler),\
        NULL, 0, NULL, NULL, 0)

#define DEFINE_KSPROPERTY_ITEM_BDA_SIGNALTIMEOUTS(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_SIGNALTIMEOUTS,\
        (GetHandler),\
        sizeof(KSP_NODE),\
        sizeof(BDA_SIGNAL_TIMEOUTS),\
        (SetHandler),\
        NULL, 0, NULL, NULL, 0)

#define DEFINE_KSPROPERTY_ITEM_BDA_PLP_NUMBER(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_PLP_NUMBER,\
        (GetHandler),\
        sizeof(KSP_NODE),\
        sizeof(ULONG),\
        (SetHandler),\
        NULL, 0, NULL, NULL, 0)

//------------------------------------------------------------
//
//
//  BDA Autodemodulate Property Set
//
// {DDF15B12-BD25-11d2-9CA0-00C04F7971E0}
//
#define STATIC_KSPROPSETID_BdaAutodemodulate \
    0xddf15b12, 0xbd25, 0x11d2, 0x9c, 0xa0, 0x0, 0xc0, 0x4f, 0x79, 0x71, 0xe0
DEFINE_GUIDSTRUCT("DDF15B12-BD25-11d2-9CA0-00C04F7971E0", KSPROPSETID_BdaAutodemodulate);
#define KSPROPSETID_BdaAutodemodulate DEFINE_GUIDNAMED(KSPROPSETID_BdaAutodemodulate)

typedef enum {
    KSPROPERTY_BDA_AUTODEMODULATE_START = 0,
    KSPROPERTY_BDA_AUTODEMODULATE_STOP
} KSPROPERTY_BDA_AUTODEMODULATE;

#define DEFINE_KSPROPERTY_ITEM_BDA_AUTODEMODULATE_START(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_AUTODEMODULATE_START,\
        FALSE,\
        sizeof(KSP_NODE),\
        0,\
        (SetHandler),\
        NULL, 0, NULL, NULL, 0)

#define DEFINE_KSPROPERTY_ITEM_BDA_AUTODEMODULATE_STOP(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_AUTODEMODULATE_STOP,\
        FALSE,\
        sizeof(KSP_NODE),\
        0,\
        (SetHandler),\
        NULL, 0, NULL, NULL, 0)

//------------------------------------------------------------
//
//
//  BDA Table Section Property Set
//
// {516B99C5-971C-4aaf-B3F3-D9FDA8A15E16}
//

#define STATIC_KSPROPSETID_BdaTableSection \
    0x516b99c5, 0x971c, 0x4aaf, 0xb3, 0xf3, 0xd9, 0xfd, 0xa8, 0xa1, 0x5e, 0x16
DEFINE_GUIDSTRUCT("516B99C5-971C-4aaf-B3F3-D9FDA8A15E16", KSPROPSETID_BdaTableSection);
#define KSPROPSETID_BdaTableSection DEFINE_GUIDNAMED(KSPROPSETID_BdaTableSection)

typedef enum {
    KSPROPERTY_BDA_TABLE_SECTION = 0,
} KSPROPERTY_IDS_BDA_TABLE;

#define DEFINE_KSPROPERTY_ITEM_BDA_TABLE_SECTION(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_TABLE_SECTION,\
        (GetHandler),\
        sizeof(KSP_NODE),\
        sizeof(BDA_TABLE_SECTION),\
        (SetHandler),\
        NULL, 0, NULL, NULL, 0)

//------------------------------------------------------------
//
//
//  BDA PID Filter Property Set
//
// {D0A67D65-08DF-4fec-8533-E5B550410B85}
//
#define STATIC_KSPROPSETID_BdaPIDFilter \
    0xd0a67d65, 0x8df, 0x4fec, 0x85, 0x33, 0xe5, 0xb5, 0x50, 0x41, 0xb, 0x85
DEFINE_GUIDSTRUCT("D0A67D65-08DF-4fec-8533-E5B550410B85", KSPROPSETID_BdaPIDFilter);
#define KSPROPSETID_BdaPIDFilter DEFINE_GUIDNAMED(KSPROPSETID_BdaPIDFilter)

typedef enum {
    KSPROPERTY_BDA_PIDFILTER_MAP_PIDS = 0,
    KSPROPERTY_BDA_PIDFILTER_UNMAP_PIDS,
    KSPROPERTY_BDA_PIDFILTER_LIST_PIDS
} KSPROPERTY_BDA_PIDFILTER;

#define DEFINE_KSPROPERTY_ITEM_BDA_PIDFILTER_MAP_PIDS(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_PIDFILTER_MAP_PIDS,\
        (GetHandler),\
        sizeof(KSP_NODE),\
        sizeof(BDA_PID_MAP),\
        (SetHandler),\
        NULL, 0, NULL, NULL, 0)

#define DEFINE_KSPROPERTY_ITEM_BDA_PIDFILTER_UNMAP_PIDS(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_PIDFILTER_UNMAP_PIDS,\
        (GetHandler),\
        sizeof(KSP_NODE),\
        sizeof(BDA_PID_UNMAP),\
        (SetHandler),\
        NULL, 0, NULL, NULL, 0)

#define DEFINE_KSPROPERTY_ITEM_BDA_PIDFILTER_LIST_PIDS(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_PIDFILTER_LIST_PIDS,\
        (GetHandler),\
        sizeof(KSP_NODE),\
        0,\
        (SetHandler),\
        NULL, 0, NULL, NULL, 0)



//------------------------------------------------------------
//
//
//  BDA CA Property Set
//
// {B0693766-5278-4ec6-B9E1-3CE40560EF5A}
//
#define STATIC_KSPROPSETID_BdaCA \
    0xb0693766, 0x5278, 0x4ec6, 0xb9, 0xe1, 0x3c, 0xe4, 0x5, 0x60, 0xef, 0x5a
DEFINE_GUIDSTRUCT("B0693766-5278-4ec6-B9E1-3CE40560EF5A", KSPROPSETID_BdaCA);
#define KSPROPSETID_BdaCA DEFINE_GUIDNAMED(KSPROPSETID_BdaCA)

typedef enum {
    KSPROPERTY_BDA_ECM_MAP_STATUS = 0,
    KSPROPERTY_BDA_CA_MODULE_STATUS,
    KSPROPERTY_BDA_CA_SMART_CARD_STATUS,
    KSPROPERTY_BDA_CA_MODULE_UI,
    KSPROPERTY_BDA_CA_SET_PROGRAM_PIDS,
    KSPROPERTY_BDA_CA_REMOVE_PROGRAM
} KSPROPERTY_BDA_CA;

#define DEFINE_KSPROPERTY_ITEM_BDA_ECM_MAP_STATUS(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_ECM_MAP_STATUS,\
        (GetHandler),\
        sizeof(KSP_NODE),\
        sizeof(ULONG),\
        (SetHandler),\
        NULL, 0, NULL, NULL, 0)

#define DEFINE_KSPROPERTY_ITEM_BDA_CA_MODULE_STATUS(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_CA_MODULE_STATUS,\
        (GetHandler),\
        sizeof(KSP_NODE),\
        sizeof(ULONG),\
        (SetHandler),\
        NULL, 0, NULL, NULL, 0)

#define DEFINE_KSPROPERTY_ITEM_BDA_CA_SMART_CARD_STATUS(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_CA_SMART_CARD_STATUS,\
        (GetHandler),\
        sizeof(KSP_NODE),\
        sizeof(ULONG),\
        (SetHandler),\
        NULL, 0, NULL, NULL, 0)

#define DEFINE_KSPROPERTY_ITEM_BDA_CA_MODULE_UI(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_CA_MODULE_UI,\
        (GetHandler),\
        sizeof(KSP_NODE),\
        sizeof(BDA_CA_MODULE_UI),\
        (SetHandler),\
        NULL, 0, NULL, NULL, 0)

#define DEFINE_KSPROPERTY_ITEM_BDA_CA_SET_PROGRAM_PIDS(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_CA_SET_PROGRAM_PIDS,\
        (GetHandler),\
        sizeof(KSP_NODE),\
        sizeof(BDA_PROGRAM_PID_LIST),\
        (SetHandler),\
        NULL, 0, NULL, NULL, 0)

#define DEFINE_KSPROPERTY_ITEM_BDA_CA_REMOVE_PROGRAM(GetHandler, SetHandler)\
    DEFINE_KSPROPERTY_ITEM(\
        KSPROPERTY_BDA_CA_REMOVE_PROGRAM,\
        (GetHandler),\
        sizeof(KSP_NODE),\
        sizeof(ULONG),\
        (SetHandler),\
        NULL, 0, NULL, NULL, 0)


//------------------------------------------------------------
//
//
//  BDA CA Event Set
//
// {488C4CCC-B768-4129-8EB1-B00A071F9068}
//
#define STATIC_KSEVENTSETID_BdaCAEvent \
    0x488c4ccc, 0xb768, 0x4129, 0x8e, 0xb1, 0xb0, 0xa, 0x7, 0x1f, 0x90, 0x68
DEFINE_GUIDSTRUCT("488C4CCC-B768-4129-8EB1-B00A071F9068", KSEVENTSETID_BdaCAEvent);
#define KSEVENTSETID_BdaCAEvent DEFINE_GUIDNAMED(KSEVENTSETID_BdaCAEvent)

typedef enum {
    KSEVENT_BDA_PROGRAM_FLOW_STATUS_CHANGED = 0,
    KSEVENT_BDA_CA_MODULE_STATUS_CHANGED,
    KSEVENT_BDA_CA_SMART_CARD_STATUS_CHANGED,
    KSEVENT_BDA_CA_MODULE_UI_REQUESTED
} KSPROPERTY_BDA_CA_EVENT;

#define DEFINE_KSEVENT_BDA_PROGRAM_FLOW_STATUS_CHANGED(AddHandler, RemoveHandler, SupportHandler)\
    DEFINE_KSEVENT_ITEM(\
        KSEVENT_BDA_PROGRAM_FLOW_STATUS_CHANGED,\
        sizeof( KSEVENTDATA), \
        0, \
        (AddHandler),\
        (RemoveHandler),\
        (SupportHandler)\
        )

#define DEFINE_KSEVENT_BDA_CA_MODULE_STATUS_CHANGED(AddHandler, RemoveHandler, SupportHandler)\
    DEFINE_KSEVENT_ITEM(\
        KSEVENT_BDA_CA_MODULE_STATUS_CHANGED,\
        sizeof( KSEVENTDATA), \
        0, \
        (AddHandler),\
        (RemoveHandler),\
        (SupportHandler)\
        )

#define DEFINE_KSEVENT_BDA_CA_SMART_CARD_STATUS_CHANGED(AddHandler, RemoveHandler, SupportHandler)\
    DEFINE_KSEVENT_ITEM(\
        KSEVENT_BDA_CA_SMART_CARD_STATUS_CHANGED,\
        sizeof( KSEVENTDATA), \
        0, \
        (AddHandler),\
        (RemoveHandler),\
        (SupportHandler)\
        )

#define DEFINE_KSEVENT_BDA_CA_MODULE_UI_REQUESTED(AddHandler, RemoveHandler, SupportHandler)\
    DEFINE_KSEVENT_ITEM(\
        KSEVENT_BDA_CA_MODULE_UI_REQUESTED,\
        sizeof( KSEVENTDATA), \
        0, \
        (AddHandler),\
        (RemoveHandler),\
        (SupportHandler)\
        )

//------------------------------------------------------------
//
//
//  BdaDrmService Method Set
//
// {BFF6B5BB-B0AE-484c-9DCA-73528FB0B46E}
//
#define STATIC_KSMETHODSETID_BdaDrmService \
    0xbff6b5bb, 0xb0ae, 0x484c, 0x9d, 0xca, 0x73, 0x52, 0x8f, 0xb0, 0xb4, 0x6e
DEFINE_GUIDSTRUCT("BFF6B5BB-B0AE-484c-9DCA-73528FB0B46E", KSMETHODSETID_BdaDrmService);
#define KSMETHODSETID_BdaDrmService DEFINE_GUIDNAMED(KSMETHODSETID_BdaDrmService)

typedef enum {
    KSMETHOD_BDA_DRM_CURRENT = 0,
    KSMETHOD_BDA_DRM_DRMSTATUS
    } KSMETHOD_BDA_DRM;

#define DEFINE_KSMETHOD_BDA_DRM_SETDRM(MethodHandler, SupportHandler)\
    DEFINE_KSMETHOD_ITEM(\
    KSMETHOD_BDA_DRM_CURRENT,\
    KSMETHOD_TYPE_MODIFY | KSMETHOD_TYPE_SOURCE,\
    (MethodHandler),\
    sizeof(KSM_BDA_DRM_SETDRM),\
    sizeof(PBDARESULT),\
    SupportHandler)

#define DEFINE_KSMETHOD_BDA_DRM_DRMSTATUS(MethodHandler, SupportHandler)\
    DEFINE_KSMETHOD_ITEM(\
    KSMETHOD_BDA_DRM_DRMSTATUS,\
    KSMETHOD_TYPE_MODIFY | KSMETHOD_TYPE_SOURCE,\
    (MethodHandler),\
    sizeof(KSM_NODE),\
    sizeof(BDA_DRM_DRMSTATUS),\
    SupportHandler)

//------------------------------------------------------------
//
//
//  PBDA WMDRM SESSION Method Set
//
// {4BE6FA3D-07CD-4139-8B80-8C18BA3AEC88}
//
#define STATIC_KSMETHODSETID_BdaWmdrmSession \
    0x4be6fa3d, 0x7cd, 0x4139, 0x8b, 0x80, 0x8c, 0x18, 0xba, 0x3a, 0xec, 0x88
DEFINE_GUIDSTRUCT("4BE6FA3D-07CD-4139-8B80-8C18BA3AEC88", KSMETHODSETID_BdaWmdrmSession);
#define KSMETHODSETID_BdaWmdrmSession DEFINE_GUIDNAMED(KSMETHODSETID_BdaWmdrmSession)

typedef enum {
    KSMETHOD_BDA_WMDRM_STATUS = 0,
    KSMETHOD_BDA_WMDRM_REVINFO,
    KSMETHOD_BDA_WMDRM_CRL,
    KSMETHOD_BDA_WMDRM_MESSAGE,
    KSMETHOD_BDA_WMDRM_REISSUELICENSE,
    KSMETHOD_BDA_WMDRM_RENEWLICENSE,
    KSMETHOD_BDA_WMDRM_LICENSE,
    KSMETHOD_BDA_WMDRM_KEYINFO
    } KSMETHOD_BDA_WMDRM;

#define DEFINE_KSMETHOD_BDA_WMDRM_STATUS(MethodHandler, SupportHandler)\
    DEFINE_KSMETHOD_ITEM(\
    KSMETHOD_BDA_WMDRM_STATUS,\
    KSMETHOD_TYPE_MODIFY | KSMETHOD_TYPE_SOURCE,\
    (MethodHandler),\
    sizeof(KSM_NODE),\
    sizeof(BDA_WMDRM_STATUS),\
    SupportHandler)

#define DEFINE_KSMETHOD_BDA_WMDRM_REVINFO(MethodHandler, SupportHandler)\
    DEFINE_KSMETHOD_ITEM(\
    KSMETHOD_BDA_WMDRM_REVINFO,\
    KSMETHOD_TYPE_MODIFY | KSMETHOD_TYPE_SOURCE,\
    (MethodHandler),\
    sizeof(KSM_BDA_BUFFER),\
    sizeof(PBDARESULT),\
    SupportHandler)

#define DEFINE_KSMETHOD_BDA_WMDRM_CRL(MethodHandler, SupportHandler)\
    DEFINE_KSMETHOD_ITEM(\
    KSMETHOD_BDA_WMDRM_CRL,\
    KSMETHOD_TYPE_MODIFY | KSMETHOD_TYPE_SOURCE,\
    (MethodHandler),\
    sizeof(KSM_BDA_BUFFER),\
    sizeof(PBDARESULT),\
    SupportHandler)

#define DEFINE_KSMETHOD_BDA_WMDRM_TRANSACTMESSAGE(MethodHandler, SupportHandler)\
    DEFINE_KSMETHOD_ITEM(\
    KSMETHOD_BDA_WMDRM_MESSAGE,\
    KSMETHOD_TYPE_MODIFY | KSMETHOD_TYPE_SOURCE,\
    (MethodHandler),\
    sizeof(KSM_BDA_BUFFER),\
    sizeof(BDA_BUFFER),\
    SupportHandler)

#define DEFINE_KSMETHOD_BDA_WMDRM_REISSUELICENSE(MethodHandler, SupportHandler)\
    DEFINE_KSMETHOD_ITEM(\
    KSMETHOD_BDA_WMDRM_REISSUELICENSE,\
    KSMETHOD_TYPE_MODIFY | KSMETHOD_TYPE_SOURCE,\
    (MethodHandler),\
    sizeof(KSM_BDA_WMDRM_LICENSE),\
    sizeof(PBDARESULT),\
    SupportHandler)

#define DEFINE_KSMETHOD_BDA_WMDRM_RENEWLICENSE(MethodHandler, SupportHandler)\
    DEFINE_KSMETHOD_ITEM(\
    KSMETHOD_BDA_WMDRM_RENEWLICENSE,\
    KSMETHOD_TYPE_MODIFY | KSMETHOD_TYPE_SOURCE,\
    (MethodHandler),\
    sizeof(KSM_BDA_WMDRM_RENEWLICENSE),\
    sizeof(BDA_WMDRM_RENEWLICENSE),\
    SupportHandler)

#define DEFINE_KSMETHOD_BDA_WMDRM_GETLICENSE(MethodHandler, SupportHandler)\
    DEFINE_KSMETHOD_ITEM(\
    KSMETHOD_BDA_WMDRM_LICENSE,\
    KSMETHOD_TYPE_MODIFY | KSMETHOD_TYPE_SOURCE,\
    (MethodHandler),\
    sizeof(KSM_BDA_WMDRM_LICENSE),\
    sizeof(BDA_BUFFER),\
    SupportHandler)

#define DEFINE_KSMETHOD_BDA_WMDRM_KEYINFO(MethodHandler, SupportHandler)\
    DEFINE_KSMETHOD_ITEM(\
    KSMETHOD_BDA_WMDRM_KEYINFO,\
    KSMETHOD_TYPE_MODIFY | KSMETHOD_TYPE_SOURCE,\
    (MethodHandler),\
    sizeof(KSM_NODE),\
    sizeof(BDA_WMDRM_KEYINFOLIST),\
    SupportHandler)

//------------------------------------------------------------
//
//
//  PBDA WMDRM Tuner Method Set
//
// {86D979CF-A8A7-4f94-B5FB-14C0ACA68FE6}
//
#define STATIC_KSMETHODSETID_BdaWmdrmTuner \
    0x86d979cf, 0xa8a7, 0x4f94, 0xb5, 0xfb, 0x14, 0xc0, 0xac, 0xa6, 0x8f, 0xe6
DEFINE_GUIDSTRUCT("86D979CF-A8A7-4f94-B5FB-14C0ACA68FE6", KSMETHODSETID_BdaWmdrmTuner);
#define KSMETHODSETID_BdaWmdrmTuner DEFINE_GUIDNAMED(KSMETHODSETID_BdaWmdrmTuner)

typedef enum {
    KSMETHOD_BDA_WMDRMTUNER_CANCELCAPTURETOKEN = 0,
    KSMETHOD_BDA_WMDRMTUNER_SETPIDPROTECTION,
    KSMETHOD_BDA_WMDRMTUNER_GETPIDPROTECTION,
    KSMETHOD_BDA_WMDRMTUNER_SETSYNCVALUE,
    KSMETHOD_BDA_WMDRMTUNER_STARTCODEPROFILE,
    KSMETHOD_BDA_WMDRMTUNER_PURCHASE_ENTITLEMENT
    } KSMETHOD_BDA_WMDRM_TUNER;

#define DEFINE_KSMETHOD_BDA_WMDRMTUNER_CANCELCAPTURETOKEN(MethodHandler, SupportHandler)\
    DEFINE_KSMETHOD_ITEM(\
    KSMETHOD_BDA_WMDRMTUNER_CANCELCAPTURETOKEN,\
    KSMETHOD_TYPE_MODIFY | KSMETHOD_TYPE_SOURCE,\
    (MethodHandler),\
    sizeof(KSM_BDA_BUFFER),\
    sizeof(PBDARESULT),\
    SupportHandler)

#define DEFINE_KSMETHOD_BDA_WMDRMTUNER_SETPIDPROTECTION(MethodHandler, SupportHandler)\
    DEFINE_KSMETHOD_ITEM(\
    KSMETHOD_BDA_WMDRMTUNER_SETPIDPROTECTION,\
    KSMETHOD_TYPE_MODIFY | KSMETHOD_TYPE_SOURCE,\
    (MethodHandler),\
    sizeof(KSM_BDA_WMDRMTUNER_SETPIDPROTECTION),\
    sizeof(PBDARESULT),\
    SupportHandler)

#define DEFINE_KSMETHOD_BDA_WMDRMTUNER_GETPIDPROTECTION(MethodHandler, SupportHandler)\
    DEFINE_KSMETHOD_ITEM(\
    KSMETHOD_BDA_WMDRMTUNER_GETPIDPROTECTION,\
    KSMETHOD_TYPE_MODIFY | KSMETHOD_TYPE_SOURCE,\
    (MethodHandler),\
    sizeof(KSM_BDA_WMDRMTUNER_GETPIDPROTECTION),\
    sizeof(BDA_WMDRMTUNER_PIDPROTECTION),\
    SupportHandler)

#define DEFINE_KSMETHOD_BDA_WMDRMTUNER_SETSYNCVALUE(MethodHandler, SupportHandler)\
    DEFINE_KSMETHOD_ITEM(\
    KSMETHOD_BDA_WMDRMTUNER_SETSYNCVALUE,\
    KSMETHOD_TYPE_MODIFY | KSMETHOD_TYPE_SOURCE,\
    (MethodHandler),\
    sizeof(KSM_BDA_WMDRMTUNER_SYNCVALUE),\
    sizeof(PBDARESULT),\
    SupportHandler)

#define DEFINE_KSMETHOD_BDA_WMDRMTUNER_STARTCODEPROFILE(MethodHandler, SupportHandler)\
    DEFINE_KSMETHOD_ITEM(\
    KSMETHOD_BDA_WMDRMTUNER_STARTCODEPROFILE,\
    KSMETHOD_TYPE_MODIFY | KSMETHOD_TYPE_SOURCE,\
    (MethodHandler),\
    sizeof(KSP_NODE),\
    sizeof(BDA_BUFFER),\
    SupportHandler)

#define DEFINE_KSMETHOD_BDA_WMDRMTUNER_PURCHASE_ENTITLEMENT(MethodHandler, SupportHandler)\
    DEFINE_KSMETHOD_ITEM(\
    KSMETHOD_BDA_WMDRMTUNER_PURCHASE_ENTITLEMENT,\
    KSMETHOD_TYPE_MODIFY | KSMETHOD_TYPE_SOURCE,\
    (MethodHandler),\
    sizeof(KSM_BDA_WMDRMTUNER_PURCHASEENTITLEMENT),\
    sizeof(BDA_WMDRMTUNER_PURCHASEENTITLEMENT),\
    SupportHandler)


//------------------------------------------------------------
//
//
//  BDA Eventing Service Property Set
//
// {B0693766-5278-4ec6-B9E1-3CE40560EF5A}
//


#define STATIC_KSMETHODSETID_BdaEventing \
    0xf99492da, 0x6193, 0x4eb0, 0x86, 0x90, 0x66, 0x86, 0xcb, 0xff, 0x71, 0x3e
DEFINE_GUIDSTRUCT("f99492da-6193-4eb0-8690-6686cbff713e", KSMETHODSETID_BdaEventing);
#define KSMETHODSETID_BdaEventing DEFINE_GUIDNAMED(KSMETHODSETID_BdaEventing)

typedef enum {
    KSMETHOD_BDA_EVENT_DATA = 0,
    KSMETHOD_BDA_EVENT_COMPLETE
} KSMETHOD_BDA_EVENTING_SERVICE;

#define DEFINE_KSMETHOD_ITEM_BDA_EVENT_DATA(MethodHandler, SupportHandler)\
    DEFINE_KSMETHOD_ITEM(\
        KSMETHOD_BDA_EVENT_DATA,\
        KSMETHOD_TYPE_MODIFY | KSMETHOD_TYPE_SOURCE, \
        (MethodHandler),\
        sizeof(KSMETHOD),\
        sizeof(BDA_EVENT_DATA),\
        SupportHandler)

#define DEFINE_KSMETHOD_ITEM_BDA_EVENT_COMPLETE(MethodHandler, SupportHandler)\
    DEFINE_KSMETHOD_ITEM(\
        KSMETHOD_BDA_EVENT_COMPLETE,\
        KSMETHOD_TYPE_MODIFY | KSMETHOD_TYPE_SOURCE, \
        (MethodHandler),\
        sizeof(KSM_BDA_EVENT_COMPLETE),\
        sizeof(LONG),\
        SupportHandler)


//------------------------------------------------------------
//
//
//  BDA Eventing Service Event 
//
// ae7e55b2-96d7-4e29-908f-62f95b2a1679
//
#define STATIC_KSEVENTSETID_BdaEvent \
    0xae7e55b2, 0x96d7, 0x4e29, 0x90, 0x8f, 0x62, 0xf9, 0x5b, 0x2a, 0x16, 0x79
DEFINE_GUIDSTRUCT("ae7e55b2-96d7-4e29-908f-62f95b2a1679", KSEVENTSETID_BdaEvent);
#define KSEVENTSETID_BdaEvent DEFINE_GUIDNAMED(KSEVENTSETID_BdaEvent)

typedef enum {
    KSEVENT_BDA_EVENT_PENDINGEVENT = 0
} KSEVENT_BDA_EVENT_TYPE;

#define DEFINE_KSEVENT_ITEM_BDA_PENDINGEVENT(AddHandler, RemoveHandler, SupportHandler)\
    DEFINE_KSEVENT_ITEM(\
        KSEVENT_BDA_EVENT_PENDINGEVENT,\
        sizeof( KSEVENTDATA), \
        0, \
        (AddHandler),\
        (RemoveHandler),\
        (SupportHandler)\
        )

//------------------------------------------------------------
//
//
//  BDA Debug Service Property Set
//
// {0d4a90ec-c69d-4ee2-8c5a-fb1f63a50da1}
//

#define STATIC_KSMETHODSETID_BdaDebug \
    0x0d4a90ec, 0xc69d, 0x4ee2, 0x8c, 0x5a, 0xfb, 0x1f, 0x63, 0xa5, 0x0d, 0xa1
DEFINE_GUIDSTRUCT("0d4a90ec-c69d-4ee2-8c5a-fb1f63a50da1", KSMETHODSETID_BdaDebug);
#define KSMETHODSETID_BdaDebug DEFINE_GUIDNAMED(KSMETHODSETID_BdaDebug)

typedef enum {
    KSMETHOD_BDA_DEBUG_LEVEL = 0,
    KSMETHOD_BDA_DEBUG_DATA
} KSMETHOD_BDA_DEBUG_SERVICE;

#define DEFINE_KSMETHOD_ITEM_BDA_DEBUG_LEVEL(MethodHandler, SupportHandler)\
    DEFINE_KSMETHOD_ITEM(\
        KSMETHOD_BDA_DEBUG_LEVEL,\
        KSMETHOD_TYPE_MODIFY | KSMETHOD_TYPE_SOURCE, \
        (MethodHandler),\
        sizeof(KSM_BDA_DEBUG_LEVEL),\
        sizeof(LONG),\
        SupportHandler)


#define DEFINE_KSMETHOD_ITEM_BDA_DEBUG_DATA(MethodHandler, SupportHandler)\
    DEFINE_KSMETHOD_ITEM(\
        KSMETHOD_BDA_DEBUG_DATA,\
        KSMETHOD_TYPE_MODIFY, \
        (MethodHandler),\
        sizeof(KSMETHOD),\
        0,\
        SupportHandler)


//------------------------------------------------------------
//
//
//  BDA Tuner Service Method Set
//
// {B774102F-AC07-478a-8228-2742D961FA7E}
//

#define STATIC_KSMETHODSETID_BdaTuner \
    0xb774102f, 0xac07, 0x478a, 0x82, 0x28, 0x27, 0x42, 0xd9, 0x61, 0xfa, 0x7e
DEFINE_GUIDSTRUCT("b774102f-ac07-478a-8228-2742d961fa7e", KSMETHODSETID_BdaTuner);
#define KSMETHODSETID_BdaTuner DEFINE_GUIDNAMED(KSMETHODSETID_BdaTuner)

typedef enum {
    KSMETHOD_BDA_TUNER_SETTUNER = 0,
    KSMETHOD_BDA_TUNER_GETTUNERSTATE,
    KSMETHOD_BDA_TUNER_SIGNALNOISERATIO
} KSMETHOD_BDA_TUNER_SERVICE;

#define DEFINE_KSMETHOD_ITEM_BDA_TUNER_SETTUNER(MethodHandler, SupportHandler)\
    DEFINE_KSMETHOD_ITEM(\
    KSMETHOD_BDA_TUNER_SETTUNER,\
    KSMETHOD_TYPE_MODIFY | KSMETHOD_TYPE_SOURCE, \
    (MethodHandler),\
    sizeof(KSM_BDA_TUNER_TUNEREQUEST),\
    sizeof(PBDARESULT),\
    SupportHandler)

#define DEFINE_KSMETHOD_ITEM_BDA_TUNER_GETTUNERSTATE(MethodHandler, SupportHandler)\
    DEFINE_KSMETHOD_ITEM(\
    KSMETHOD_BDA_TUNER_GETTUNERSTATE,\
    KSMETHOD_TYPE_MODIFY | KSMETHOD_TYPE_SOURCE, \
    (MethodHandler),\
    sizeof(KSMETHOD),\
    sizeof(BDA_TUNER_TUNERSTATE),\
    SupportHandler)

#define DEFINE_KSMETHOD_ITEM_BDA_TUNER_SIGNALNOISERATIO(MethodHandler, SupportHandler)\
    DEFINE_KSMETHOD_ITEM(\
    KSMETHOD_BDA_TUNER_SIGNALNOISERATIO,\
    KSMETHOD_TYPE_MODIFY | KSMETHOD_TYPE_SOURCE, \
    (MethodHandler),\
    sizeof(KSMETHOD),\
    sizeof(BDA_TUNER_DIAGNOSTICS),\
    SupportHandler)


//------------------------------------------------------------
//
//
//  BDA General Purpose Name Value Service Method Set
//
// {0c24096d-5ff5-47de-a856-062e587e3727}//8bit string
// {36e07304-9f0d-4e88-9118-ac0ba317b7f2}//unicode version
//

#define STATIC_KSMETHODSETID_BdaNameValueA \
    0xc24096d, 0x5ff5, 0x47de, 0xa8, 0x56, 0x6, 0x2e, 0x58, 0x7e, 0x37, 0x27
DEFINE_GUIDSTRUCT("0c24096d-5ff5-47de-a856-062e587e3727", KSMETHODSETID_BdaNameValueA);
#define KSMETHODSETID_BdaNameValueA DEFINE_GUIDNAMED(KSMETHODSETID_BdaNameValueA)

#define STATIC_KSMETHODSETID_BdaNameValue \
    0x36e07304, 0x9f0d, 0x4e88, 0x91, 0x18, 0xac, 0xb, 0xa3, 0x17, 0xb7, 0xf2
DEFINE_GUIDSTRUCT("36e07304-9f0d-4e88-9118-ac0ba317b7f2", KSMETHODSETID_BdaNameValue);
#define KSMETHODSETID_BdaNameValue DEFINE_GUIDNAMED(KSMETHODSETID_BdaNameValue)

typedef enum {
    KSMETHOD_BDA_GPNV_GETVALUE = 0,
    KSMETHOD_BDA_GPNV_SETVALUE,
    KSMETHOD_BDA_GPNV_NAMEFROMINDEX,
    KSMETHOD_BDA_GPNV_GETVALUEUPDATENAME
} KSMETHOD_BDA_GPNV_SERVICE;

#define DEFINE_KSMETHOD_ITEM_BDA_GPNV_GETVALUE(MethodHandler, SupportHandler)\
    DEFINE_KSMETHOD_ITEM(\
    KSMETHOD_BDA_GPNV_GETVALUE,\
    KSMETHOD_TYPE_MODIFY | KSMETHOD_TYPE_SOURCE, \
    (MethodHandler),\
    sizeof(KSM_BDA_GPNV_GETVALUE),\
    sizeof(BDA_STRING),\
    SupportHandler)

#define DEFINE_KSMETHOD_ITEM_BDA_GPNV_SETVALUE(MethodHandler, SupportHandler)\
    DEFINE_KSMETHOD_ITEM(\
    KSMETHOD_BDA_GPNV_SETVALUE,\
    KSMETHOD_TYPE_MODIFY | KSMETHOD_TYPE_SOURCE, \
    (MethodHandler),\
    sizeof(KSM_BDA_GPNV_SETVALUE),\
    sizeof(PBDARESULT),\
    SupportHandler)

#define DEFINE_KSMETHOD_ITEM_BDA_GPNV_NAMEFROMINDEX(MethodHandler, SupportHandler)\
    DEFINE_KSMETHOD_ITEM(\
    KSMETHOD_BDA_GPNV_NAMEFROMINDEX,\
    KSMETHOD_TYPE_MODIFY | KSMETHOD_TYPE_SOURCE, \
    (MethodHandler),\
    sizeof(KSM_BDA_GPNV_NAMEINDEX),\
    sizeof(BDA_STRING),\
    SupportHandler)

#define DEFINE_KSMETHOD_ITEM_BDA_GPNV_GETVALUEUPDATENAME(MethodHandler, SupportHandler)\
    DEFINE_KSMETHOD_ITEM(\
    KSMETHOD_BDA_GPNV_GETVALUEUPDATENAME,\
    KSMETHOD_TYPE_MODIFY | KSMETHOD_TYPE_SOURCE, \
    (MethodHandler),\
    sizeof(KSMETHOD),\
    sizeof(BDA_STRING),\
    SupportHandler)



//------------------------------------------------------------
//
//
//  BDA Mux Service Method Set
//
// {942AAFEC-4C05-4c74-B8EB-8706C2A4943F}
//

#define STATIC_KSMETHODSETID_BdaMux \
    0x942aafec, 0x4c05, 0x4c74, 0xb8, 0xeb, 0x87, 0x6, 0xc2, 0xa4, 0x94, 0x3f
DEFINE_GUIDSTRUCT("942AAFEC-4C05-4c74-B8EB-8706C2A4943F", KSMETHODSETID_BdaMux);
#define KSMETHODSETID_BdaMux DEFINE_GUIDNAMED(KSMETHODSETID_BdaMux)

typedef enum {
    KSMETHOD_BDA_MUX_GETPIDLIST = 0,
    KSMETHOD_BDA_MUX_SETPIDLIST
} KSMETHOD_BDA_MUX_SERVICE;

#define DEFINE_KSMETHOD_ITEM_BDA_MUX_GETPIDLIST(MethodHandler, SupportHandler)\
    DEFINE_KSMETHOD_ITEM(\
    KSMETHOD_BDA_MUX_GETPIDLIST,\
    KSMETHOD_TYPE_MODIFY | KSMETHOD_TYPE_SOURCE, \
    (MethodHandler),\
    sizeof(KSM_NODE),\
    sizeof(BDA_BUFFER),\
    SupportHandler)

#define DEFINE_KSMETHOD_ITEM_BDA_MUX_SETPIDLIST(MethodHandler, SupportHandler)\
    DEFINE_KSMETHOD_ITEM(\
    KSMETHOD_BDA_MUX_SETPIDLIST,\
    KSMETHOD_TYPE_MODIFY | KSMETHOD_TYPE_SOURCE, \
    (MethodHandler),\
    sizeof(KSM_BDA_BUFFER),\
    sizeof(PBDARESULT),\
    SupportHandler)


//------------------------------------------------------------
//
//
//  BDA Scanning Service Method Set
//
// {12EB49DF-6249-47f3-B190-E21E6E2F8A9C}
//

#define STATIC_KSMETHODSETID_BdaScanning \
    0x12eb49df, 0x6249, 0x47f3, 0xb1, 0x90, 0xe2, 0x1e, 0x6e, 0x2f, 0x8a, 0x9c
DEFINE_GUIDSTRUCT("12EB49DF-6249-47f3-B190-E21E6E2F8A9C", KSMETHODSETID_BdaScanning);
#define KSMETHODSETID_BdaScanning DEFINE_GUIDNAMED(KSMETHODSETID_BdaScanning)

typedef enum {
    KSMETHOD_BDA_SCAN_CAPABILTIES = 0,
    KSMETHOD_BDA_SCANNING_STATE,
    KSMETHOD_BDA_SCAN_FILTER,
    KSMETHOD_BDA_SCAN_START,
    KSMETHOD_BDA_SCAN_RESUME,
    KSMETHOD_BDA_SCAN_STOP
} KSMETHOD_BDA_SCAN_SERVICE;

#define DEFINE_KSMETHOD_ITEM_BDA_SCAN_CAPABILTIES(MethodHandler, SupportHandler)\
    DEFINE_KSMETHOD_ITEM(\
    KSMETHOD_BDA_SCAN_CAPABILTIES,\
    KSMETHOD_TYPE_MODIFY | KSMETHOD_TYPE_SOURCE, \
    (MethodHandler),\
    sizeof(KSM_BDA_SCAN_CAPABILTIES),\
    sizeof(BDA_SCAN_CAPABILTIES),\
    SupportHandler)

#define DEFINE_KSMETHOD_ITEM_BDA_SCANNING_STATE(MethodHandler, SupportHandler)\
    DEFINE_KSMETHOD_ITEM(\
    KSMETHOD_BDA_SCANNING_STATE,\
    KSMETHOD_TYPE_MODIFY | KSMETHOD_TYPE_SOURCE, \
    (MethodHandler),\
    sizeof(KSMETHOD),\
    sizeof(BDA_SCAN_STATE),\
    SupportHandler)

#define DEFINE_KSMETHOD_ITEM_BDA_SCAN_FILTER(MethodHandler, SupportHandler)\
    DEFINE_KSMETHOD_ITEM(\
    KSMETHOD_BDA_SCAN_FILTER,\
    KSMETHOD_TYPE_MODIFY | KSMETHOD_TYPE_SOURCE, \
    (MethodHandler),\
    sizeof(KSM_BDA_SCAN_FILTER),\
    sizeof(PBDARESULT),\
    SupportHandler)

#define DEFINE_KSMETHOD_ITEM_BDA_SCAN_START(MethodHandler, SupportHandler)\
    DEFINE_KSMETHOD_ITEM(\
    KSMETHOD_BDA_SCAN_START,\
    KSMETHOD_TYPE_MODIFY | KSMETHOD_TYPE_SOURCE, \
    (MethodHandler),\
    sizeof(KSM_BDA_SCAN_START),\
    sizeof(PBDARESULT),\
    SupportHandler)

#define DEFINE_KSMETHOD_ITEM_BDA_SCAN_RESUME(MethodHandler, SupportHandler)\
    DEFINE_KSMETHOD_ITEM(\
    KSMETHOD_BDA_SCAN_RESUME,\
    KSMETHOD_TYPE_MODIFY | KSMETHOD_TYPE_SOURCE, \
    (MethodHandler),\
    sizeof(KSMETHOD),\
    sizeof(PBDARESULT),\
    SupportHandler)

#define DEFINE_KSMETHOD_ITEM_BDA_SCAN_STOP(MethodHandler, SupportHandler)\
    DEFINE_KSMETHOD_ITEM(\
    KSMETHOD_BDA_SCAN_STOP,\
    KSMETHOD_TYPE_MODIFY | KSMETHOD_TYPE_SOURCE, \
    (MethodHandler),\
    sizeof(KSMETHOD),\
    sizeof(PBDARESULT),\
    SupportHandler)


//------------------------------------------------------------
//
//
//  BDA Guide Data Delivery Service Method Set
//
// {8D9D5562-1589-417d-99CE-AC531DDA19F9}
//

#define STATIC_KSMETHODSETID_BdaGuideDataDeliveryService \
    0x8d9d5562, 0x1589, 0x417d, 0x99, 0xce, 0xac, 0x53, 0x1d, 0xda, 0x19, 0xf9
DEFINE_GUIDSTRUCT("8D9D5562-1589-417d-99CE-AC531DDA19F9", KSMETHODSETID_BdaGuideDataDeliveryService);
#define KSMETHODSETID_BdaGuideDataDeliveryService DEFINE_GUIDNAMED(KSMETHODSETID_BdaGuideDataDeliveryService)

typedef enum {
    KSMETHOD_BDA_GDDS_DATATYPE = 0,
    KSMETHOD_BDA_GDDS_DATA,
    KSMETHOD_BDA_GDDS_TUNEXMLFROMIDX,
    KSMETHOD_BDA_GDDS_GETSERVICES,
    KSMETHOD_BDA_GDDS_SERVICEFROMTUNEXML,
    KSMETHOD_BDA_GDDS_DATAUPDATE
} KSMETHOD_BDA_GDDS_SERVICE;

#define DEFINE_KSMETHOD_ITEM_BDA_GDDS_DATATYPE(MethodHandler, SupportHandler)\
    DEFINE_KSMETHOD_ITEM(\
    KSMETHOD_BDA_GDDS_DATATYPE,\
    KSMETHOD_TYPE_MODIFY | KSMETHOD_TYPE_SOURCE, \
    (MethodHandler),\
    sizeof(KSMETHOD),\
    sizeof(BDA_GDDS_DATATYPE),\
    SupportHandler)

#define DEFINE_KSMETHOD_ITEM_BDA_GDDS_DATA(MethodHandler, SupportHandler)\
    DEFINE_KSMETHOD_ITEM(\
    KSMETHOD_BDA_GDDS_DATA,\
    KSMETHOD_TYPE_MODIFY | KSMETHOD_TYPE_SOURCE, \
    (MethodHandler),\
    sizeof(KSMETHOD),\
    sizeof(BDA_GDDS_DATA),\
    SupportHandler)

#define DEFINE_KSMETHOD_ITEM_BDA_GDDS_TUNEXMLFROMIDX(MethodHandler, SupportHandler)\
    DEFINE_KSMETHOD_ITEM(\
    KSMETHOD_BDA_GDDS_TUNEXMLFROMIDX,\
    KSMETHOD_TYPE_MODIFY | KSMETHOD_TYPE_SOURCE, \
    (MethodHandler),\
    sizeof(KSM_BDA_GDDS_TUNEXMLFROMIDX),\
    sizeof(BDA_STRING),\
    SupportHandler)

#define DEFINE_KSMETHOD_ITEM_BDA_GDDS_GETSERVICES(MethodHandler, SupportHandler)\
    DEFINE_KSMETHOD_ITEM(\
    KSMETHOD_BDA_GDDS_GETSERVICES,\
    KSMETHOD_TYPE_MODIFY | KSMETHOD_TYPE_SOURCE, \
    (MethodHandler),\
    sizeof(KSMETHOD),\
    sizeof(BDA_BUFFER),\
    SupportHandler)

#define DEFINE_KSMETHOD_ITEM_BDA_GDDS_SERVICEFROMTUNEXML(MethodHandler, SupportHandler)\
    DEFINE_KSMETHOD_ITEM(\
    KSMETHOD_BDA_GDDS_SERVICEFROMTUNEXML,\
    KSMETHOD_TYPE_MODIFY | KSMETHOD_TYPE_SOURCE, \
    (MethodHandler),\
    sizeof(KSM_BDA_GDDS_SERVICEFROMTUNEXML),\
    sizeof(BDA_STRING),\
    SupportHandler)

#define DEFINE_KSMETHOD_ITEM_BDA_GDDS_DATAUPDATE(MethodHandler, SupportHandler)\
    DEFINE_KSMETHOD_ITEM(\
    KSMETHOD_BDA_GDDS_DATAUPDATE,\
    KSMETHOD_TYPE_MODIFY | KSMETHOD_TYPE_SOURCE, \
    (MethodHandler),\
    sizeof(KSMETHOD),\
    sizeof(PBDARESULT),\
    SupportHandler)


//------------------------------------------------------------
//
//
//  BDA CAS Core Service Method Set
//
// {10CED3B4-320B-41bf-9824-1B2E68E71EB9}
//

#define STATIC_KSMETHODSETID_BdaConditionalAccessService \
    0x10ced3b4, 0x320b, 0x41bf, 0x98, 0x24, 0x1b, 0x2e, 0x68, 0xe7, 0x1e, 0xb9
DEFINE_GUIDSTRUCT("10CED3B4-320B-41bf-9824-1B2E68E71EB9", KSMETHODSETID_BdaConditionalAccessService);
#define KSMETHODSETID_BdaConditionalAccessService DEFINE_GUIDNAMED(KSMETHODSETID_BdaConditionalAccessService)

typedef enum {
    KSMETHOD_BDA_CAS_CHECKENTITLEMENTTOKEN = 0,
    KSMETHOD_BDA_CAS_SETCAPTURETOKEN,
    KSMETHOD_BDA_CAS_OPENBROADCASTMMI,
    KSMETHOD_BDA_CAS_CLOSEMMIDIALOG
} KSMETHOD_BDA_CAS_SERVICE;

#define DEFINE_KSMETHOD_ITEM_BDA_CAS_CHECKENTITLEMENTTOKEN(MethodHandler, SupportHandler)\
    DEFINE_KSMETHOD_ITEM(\
    KSMETHOD_BDA_CAS_CHECKENTITLEMENTTOKEN,\
    KSMETHOD_TYPE_MODIFY | KSMETHOD_TYPE_SOURCE, \
    (MethodHandler),\
    sizeof(KSM_BDA_CAS_ENTITLEMENTTOKEN),\
    sizeof(BDA_CAS_CHECK_ENTITLEMENTTOKEN),\
    SupportHandler)

#define DEFINE_KSMETHOD_ITEM_BDA_CAS_SETCAPTURETOKEN(MethodHandler, SupportHandler)\
    DEFINE_KSMETHOD_ITEM(\
    KSMETHOD_BDA_CAS_SETCAPTURETOKEN,\
    KSMETHOD_TYPE_MODIFY | KSMETHOD_TYPE_SOURCE, \
    (MethodHandler),\
    sizeof(KSM_BDA_CAS_CAPTURETOKEN),\
    sizeof(PBDARESULT),\
    SupportHandler)

#define DEFINE_KSMETHOD_ITEM_BDA_CAS_OPENBROADCASTMMI(MethodHandler, SupportHandler)\
    DEFINE_KSMETHOD_ITEM(\
    KSMETHOD_BDA_CAS_OPENBROADCASTMMI,\
    KSMETHOD_TYPE_MODIFY | KSMETHOD_TYPE_SOURCE, \
    (MethodHandler),\
    sizeof(KSM_BDA_CAS_OPENBROADCASTMMI),\
    sizeof(PBDARESULT),\
    SupportHandler)

#define DEFINE_KSMETHOD_ITEM_BDA_CAS_CLOSEMMIDIALOG(MethodHandler, SupportHandler)\
    DEFINE_KSMETHOD_ITEM(\
    KSMETHOD_BDA_CAS_CLOSEMMIDIALOG,\
    KSMETHOD_TYPE_MODIFY | KSMETHOD_TYPE_SOURCE, \
    (MethodHandler),\
    sizeof(KSM_BDA_CAS_CLOSEMMIDIALOG),\
    sizeof(BDA_CAS_CLOSE_MMIDIALOG),\
    SupportHandler)


//------------------------------------------------------------
//
//
//  BDA ISDB CAS Service Method Set
//
// {5E68C627-16C2-4e6c-B1E2-D00170CDAA0F}
//

#define STATIC_KSMETHODSETID_BdaIsdbConditionalAccess \
    0x5e68c627, 0x16c2, 0x4e6c, 0xb1, 0xe2, 0xd0, 0x1, 0x70, 0xcd, 0xaa, 0xf
DEFINE_GUIDSTRUCT("5E68C627-16C2-4e6c-B1E2-D00170CDAA0F", KSMETHODSETID_BdaIsdbConditionalAccess);
#define KSMETHODSETID_BdaIsdbConditionalAccess DEFINE_GUIDNAMED(KSMETHODSETID_BdaIsdbConditionalAccess)

typedef enum {
    KSMETHOD_BDA_ISDBCAS_SETREQUEST = 0,
    KSMETHOD_BDA_ISDBCAS_RESPONSEDATA
} KSMETHOD_BDA_ISDB_CAS;

#define DEFINE_KSMETHOD_ITEM_BDA_ISDBCAS_SETREQUEST(MethodHandler, SupportHandler)\
    DEFINE_KSMETHOD_ITEM(\
    KSMETHOD_BDA_ISDBCAS_SETREQUEST,\
    KSMETHOD_TYPE_MODIFY | KSMETHOD_TYPE_SOURCE, \
    (MethodHandler),\
    sizeof(KSM_BDA_ISDBCAS_REQUEST),\
    sizeof(PBDARESULT),\
    SupportHandler)

#define DEFINE_KSMETHOD_ITEM_BDA_ISDBCAS_RESPONSEDATA(MethodHandler, SupportHandler)\
    DEFINE_KSMETHOD_ITEM(\
    KSMETHOD_BDA_ISDBCAS_RESPONSEDATA,\
    KSMETHOD_TYPE_MODIFY | KSMETHOD_TYPE_SOURCE, \
    (MethodHandler),\
    sizeof(KSM_NODE),\
    sizeof(BDA_ISDBCAS_RESPONSEDATA),\
    SupportHandler)

//------------------------------------------------------------
//
//
//  BDA Transprt Stream Selector Service Method Set
//
// {1DCFAFE9-B45E-41b3-BB2A-561EB129AE98}
//

#define STATIC_KSMETHODSETID_BdaTSSelector \
    0x1dcfafe9, 0xb45e, 0x41b3, 0xbb, 0x2a, 0x56, 0x1e, 0xb1, 0x29, 0xae, 0x98
DEFINE_GUIDSTRUCT("1DCFAFE9-B45E-41b3-BB2A-561EB129AE98", KSMETHODSETID_BdaTSSelector);
#define KSMETHODSETID_BdaTSSelector DEFINE_GUIDNAMED(KSMETHODSETID_BdaTSSelector)

typedef enum {
    KSMETHOD_BDA_TS_SELECTOR_SETTSID = 0,
    KSMETHOD_BDA_TS_SELECTOR_GETTSINFORMATION,
} KSMETHOD_BDA_TS_SELECTOR;

#define DEFINE_KSMETHOD_ITEM_BDA_TS_SELECTOR_SETTSID(MethodHandler, SupportHandler)\
    DEFINE_KSMETHOD_ITEM(\
    KSMETHOD_BDA_TS_SELECTOR_SETTSID,\
    KSMETHOD_TYPE_MODIFY | KSMETHOD_TYPE_SOURCE, \
    (MethodHandler),\
    sizeof(KSM_BDA_TS_SELECTOR_SETTSID),\
    sizeof(ULONG),\
    SupportHandler)

#define DEFINE_KSMETHOD_ITEM_BDA_TS_SELECTOR_GETTSINFORMATION(MethodHandler, SupportHandler)\
    DEFINE_KSMETHOD_ITEM(\
    KSMETHOD_BDA_TS_SELECTOR_GETTSINFORMATION,\
    KSMETHOD_TYPE_MODIFY | KSMETHOD_TYPE_SOURCE, \
    (MethodHandler),\
    sizeof(KSM_NODE),\
    sizeof(BDA_TS_SELECTORINFO),\
    SupportHandler)

//------------------------------------------------------------
//
//
//  BDA User Activity Service Method Set
//
// {EDA5C834-4531-483c-BE0A-94E6C96FF396}
//

#define STATIC_KSMETHODSETID_BdaUserActivity \
    0xeda5c834, 0x4531, 0x483c, 0xbe, 0xa, 0x94, 0xe6, 0xc9, 0x6f, 0xf3, 0x96
DEFINE_GUIDSTRUCT("EDA5C834-4531-483c-BE0A-94E6C96FF396", KSMETHODSETID_BdaUserActivity);
#define KSMETHODSETID_BdaUserActivity DEFINE_GUIDNAMED(KSMETHODSETID_BdaUserActivity)

typedef enum {
    KSMETHOD_BDA_USERACTIVITY_USEREASON = 0,
    KSMETHOD_BDA_USERACTIVITY_INTERVAL,
    KSMETHOD_BDA_USERACTIVITY_DETECTED
} KSMETHOD_BDA_USERACTIVITY_SERVICE;

#define DEFINE_KSMETHOD_ITEM_BDA_USERACTIVITY_USEREASON(MethodHandler, SupportHandler)\
    DEFINE_KSMETHOD_ITEM(\
    KSMETHOD_BDA_USERACTIVITY_USEREASON,\
    KSMETHOD_TYPE_MODIFY | KSMETHOD_TYPE_SOURCE, \
    (MethodHandler),\
    sizeof(KSM_BDA_USERACTIVITY_USEREASON),\
    sizeof(PBDARESULT),\
    SupportHandler)

#define DEFINE_KSMETHOD_ITEM_BDA_USERACTIVITY_INTERVAL(MethodHandler, SupportHandler)\
    DEFINE_KSMETHOD_ITEM(\
    KSMETHOD_BDA_USERACTIVITY_INTERVAL,\
    KSMETHOD_TYPE_MODIFY | KSMETHOD_TYPE_SOURCE, \
    (MethodHandler),\
    sizeof(KSMETHOD),\
    sizeof(BDA_USERACTIVITY_INTERVAL),\
    SupportHandler)

#define DEFINE_KSMETHOD_ITEM_BDA_USERACTIVITY_DETECTED(MethodHandler, SupportHandler)\
    DEFINE_KSMETHOD_ITEM(\
    KSMETHOD_BDA_USERACTIVITY_DETECTED,\
    KSMETHOD_TYPE_MODIFY | KSMETHOD_TYPE_SOURCE, \
    (MethodHandler),\
    sizeof(KSMETHOD),\
    sizeof(PBDARESULT),\
    SupportHandler)

//===========================================================================
//
// BDA Filter Categories
//
//===========================================================================

#define STATIC_KSCATEGORY_BDA_RECEIVER_COMPONENT \
    0xFD0A5AF4, 0xB41D, 0x11d2, 0x9c, 0x95, 0x0, 0xc0, 0x4f, 0x79, 0x71, 0xe0
DEFINE_GUIDSTRUCT("FD0A5AF4-B41D-11d2-9C95-00C04F7971E0", KSCATEGORY_BDA_RECEIVER_COMPONENT);
#define KSCATEGORY_BDA_RECEIVER_COMPONENT DEFINE_GUIDNAMED(KSCATEGORY_BDA_RECEIVER_COMPONENT)


#define STATIC_KSCATEGORY_BDA_NETWORK_TUNER \
    0x71985f48, 0x1ca1, 0x11d3, 0x9c, 0xc8, 0x0, 0xc0, 0x4f, 0x79, 0x71, 0xe0
DEFINE_GUIDSTRUCT("71985F48-1CA1-11d3-9CC8-00C04F7971E0", KSCATEGORY_BDA_NETWORK_TUNER);
#define KSCATEGORY_BDA_NETWORK_TUNER DEFINE_GUIDNAMED(KSCATEGORY_BDA_NETWORK_TUNER)


#define STATIC_KSCATEGORY_BDA_NETWORK_EPG \
    0x71985f49, 0x1ca1, 0x11d3, 0x9c, 0xc8, 0x0, 0xc0, 0x4f, 0x79, 0x71, 0xe0
DEFINE_GUIDSTRUCT("71985F49-1CA1-11d3-9CC8-00C04F7971E0", KSCATEGORY_BDA_NETWORK_EPG);
#define KSCATEGORY_BDA_NETWORK_EPG DEFINE_GUIDNAMED(KSCATEGORY_BDA_NETWORK_EPG)


#define STATIC_KSCATEGORY_BDA_IP_SINK \
    0x71985f4aL, 0x1ca1, 0x11d3, 0x9c, 0xc8, 0x00, 0xc0, 0x4f, 0x79, 0x71, 0xe0
DEFINE_GUIDSTRUCT("71985F4A-1CA1-11d3-9CC8-00C04F7971E0", KSCATEGORY_BDA_IP_SINK);
#define KSCATEGORY_IP_SINK DEFINE_GUIDNAMED(KSCATEGORY_BDA_IP_SINK)


#define STATIC_KSCATEGORY_BDA_NETWORK_PROVIDER \
    0x71985f4b, 0x1ca1, 0x11d3, 0x9c, 0xc8, 0x0, 0xc0, 0x4f, 0x79, 0x71, 0xe0
DEFINE_GUIDSTRUCT("71985F4B-1CA1-11d3-9CC8-00C04F7971E0", KSCATEGORY_BDA_NETWORK_PROVIDER);
#define KSCATEGORY_BDA_NETWORK_PROVIDER DEFINE_GUIDNAMED(KSCATEGORY_BDA_NETWORK_PROVIDER)

// {A2E3074F-6C3D-11d3-B653-00C04F79498E}
#define STATIC_KSCATEGORY_BDA_TRANSPORT_INFORMATION \
        0xa2e3074f, 0x6c3d, 0x11d3, 0xb6, 0x53, 0x0, 0xc0, 0x4f, 0x79, 0x49, 0x8e
DEFINE_GUIDSTRUCT("A2E3074F-6C3D-11d3-B653-00C04F79498E", KSCATEGORY_BDA_TRANSPORT_INFORMATION);
#define KSCATEGORY_BDA_TRANSPORT_INFORMATION DEFINE_GUIDNAMED(KSCATEGORY_BDA_TRANSPORT_INFORMATION)

//===========================================================================
//
// BDA Node Categories
//
//===========================================================================

//
// Typical usage of the node categories to define supported tuner device standards.
//
// Analog (only)
//   KSNODE_BDA_RF_TUNER
//
// ATSC (only):
//   KSNODE_BDA_8VSB_DEMODULATOR node
//
// DVB-T (only):
//   KSNODE_BDA_COFDM_DEMODULATOR
//
// DVB-S (only)
//   KSNODE_BDA_QPSK_DEMODULATOR
//
// DVB-S2 (only)
//   KSNODE_BDA_8PSK_DEMODULATOR
//
// Digital Cable (both DVB-C and US):
//   KSNODE_BDA_QAM_DEMODULATOR node
//
// Hybrid Analog/ATSC:
//   KSNODE_BDA_RF_TUNER & KSNODE_BDA_8VSB_DEMODULATOR nodes
//
// Hybrid Analog/Digital Cable:
//   KSNODE_BDA_RF_TUNER & KSNODE_BDA_QAM_DEMODULATOR nodes
//
// Hybrid Analog and Digital Cable w/ CableCard:
//   KSNODE_BDA_RF_TUNER & KSNODE_BDA_QAM_DEMODULATOR & KSNODE_BDA_OPENCABLE_POD nodes
//
// Hybrid Analog and DVB-T:
//   KSNODE_BDA_RF_TUNER & KSNODE_BDA_COFDM_DEMODULATOR
//
// ISDB-S (only)
//   KSNODE_BDA_ISDB_S_DEMODULATOR & KSNODE_BDA_TS_SELECTOR
//

#define STATIC_KSNODE_BDA_RF_TUNER \
    0x71985f4c, 0x1ca1, 0x11d3, 0x9c, 0xc8, 0x0, 0xc0, 0x4f, 0x79, 0x71, 0xe0
DEFINE_GUIDSTRUCT("71985F4C-1CA1-11d3-9CC8-00C04F7971E0", KSNODE_BDA_RF_TUNER);
#define KSNODE_BDA_RF_TUNER DEFINE_GUIDNAMED(KSNODE_BDA_RF_TUNER)

#define STATIC_KSNODE_BDA_ANALOG_DEMODULATOR \
    0x634db199, 0x27dd, 0x46b8, 0xac, 0xfb, 0xec, 0xc9, 0x8e, 0x61, 0xa2, 0xad
DEFINE_GUIDSTRUCT("634DB199-27DD-46b8-ACFB-ECC98E61A2AD",  KSNODE_BDA_ANALOG_DEMODULATOR);
#define  KSNODE_BDA_ANALOG_DEMODULATOR DEFINE_GUIDNAMED( KSNODE_BDA_ANALOG_DEMODULATOR)


#define STATIC_KSNODE_BDA_QAM_DEMODULATOR \
    0x71985f4d, 0x1ca1, 0x11d3, 0x9c, 0xc8, 0x0, 0xc0, 0x4f, 0x79, 0x71, 0xe0
DEFINE_GUIDSTRUCT("71985F4D-1CA1-11d3-9CC8-00C04F7971E0", KSNODE_BDA_QAM_DEMODULATOR);
#define KSNODE_BDA_QAM_DEMODULATOR DEFINE_GUIDNAMED(KSNODE_BDA_QAM_DEMODULATOR)


#define STATIC_KSNODE_BDA_QPSK_DEMODULATOR \
    0x6390c905, 0x27c1, 0x4d67, 0xbd, 0xb7, 0x77, 0xc5, 0xd, 0x7, 0x93, 0x0
DEFINE_GUIDSTRUCT("6390C905-27C1-4d67-BDB7-77C50D079300", KSNODE_BDA_QPSK_DEMODULATOR);
#define KSNODE_BDA_QPSK_DEMODULATOR DEFINE_GUIDNAMED(KSNODE_BDA_QPSK_DEMODULATOR)


#define STATIC_KSNODE_BDA_8VSB_DEMODULATOR \
    0x71985f4f, 0x1ca1, 0x11d3, 0x9c, 0xc8, 0x0, 0xc0, 0x4f, 0x79, 0x71, 0xe0
DEFINE_GUIDSTRUCT("71985F4F-1CA1-11d3-9CC8-00C04F7971E0", KSNODE_BDA_8VSB_DEMODULATOR);
#define KSNODE_BDA_8VSB_DEMODULATOR DEFINE_GUIDNAMED(KSNODE_BDA_8VSB_DEMODULATOR)


#define STATIC_KSNODE_BDA_COFDM_DEMODULATOR \
    0x2dac6e05, 0xedbe, 0x4b9c, 0xb3, 0x87, 0x1b, 0x6f, 0xad, 0x7d, 0x64, 0x95
DEFINE_GUIDSTRUCT("2DAC6E05-EDBE-4b9c-B387-1B6FAD7D6495", KSNODE_BDA_COFDM_DEMODULATOR);
#define KSNODE_BDA_COFDM_DEMODULATOR DEFINE_GUIDNAMED(KSNODE_BDA_COFDM_DEMODULATOR)


#define STATIC_KSNODE_BDA_8PSK_DEMODULATOR \
    0xe957a0e7, 0xdd98, 0x4a3c, 0x81, 0x0b, 0x35, 0x25, 0x15, 0x7a, 0xb6, 0x2e
DEFINE_GUIDSTRUCT("E957A0E7-DD98-4A3C-810B-3525157AB62E", KSNODE_BDA_8PSK_DEMODULATOR);
#define KSNODE_BDA_8PSK_DEMODULATOR DEFINE_GUIDNAMED(KSNODE_BDA_8PSK_DEMODULATOR)


#define STATIC_KSNODE_BDA_ISDB_T_DEMODULATOR \
    0xfcea3ae3, 0x2cb2, 0x464d, 0x8f, 0x5d, 0x30, 0x5c, 0x0b, 0xb7, 0x78, 0xa2
    DEFINE_GUIDSTRUCT("fcea3ae3-2cb2-464d-8f5d-305c0bb778a2", KSNODE_BDA_ISDB_T_DEMODULATOR);
#define KSNODE_BDA_ISDB_T_DEMODULATOR DEFINE_GUIDNAMED(KSNODE_BDA_ISDB_T_DEMODULATOR)

#define STATIC_KSNODE_BDA_ISDB_S_DEMODULATOR \
    0xedde230a, 0x9086, 0x432d, 0xb8, 0xa5, 0x66, 0x70, 0x26, 0x38, 0x07, 0xe9
    DEFINE_GUIDSTRUCT("edde230a-9086-432d-b8a5-6670263807e9", KSNODE_BDA_ISDB_S_DEMODULATOR);
#define KSNODE_BDA_ISDB_S_DEMODULATOR DEFINE_GUIDNAMED(KSNODE_BDA_ISDB_S_DEMODULATOR)


#define STATIC_KSNODE_BDA_OPENCABLE_POD \
    0x345812a0, 0xfb7c, 0x4790, 0xaa, 0x7e, 0xb1, 0xdb, 0x88, 0xac, 0x19, 0xc9
DEFINE_GUIDSTRUCT("345812A0-FB7C-4790-AA7E-B1DB88AC19C9", KSNODE_BDA_OPENCABLE_POD);
#define KSNODE_BDA_OPENCABLE_POD DEFINE_GUIDNAMED(KSNODE_BDA_OPENCABLE_POD)

#define STATIC_KSNODE_BDA_COMMON_CA_POD \
    0xd83ef8fc, 0xf3b8, 0x45ab, 0x8b, 0x71, 0xec, 0xf7, 0xc3, 0x39, 0xde, 0xb4
DEFINE_GUIDSTRUCT("D83EF8FC-F3B8-45ab-8B71-ECF7C339DEB4", KSNODE_BDA_COMMON_CA_POD);
#define KSNODE_BDA_COMMON_CA_POD DEFINE_GUIDNAMED(KSNODE_BDA_COMMON_CA_POD)


#define STATIC_KSNODE_BDA_PID_FILTER \
    0xf5412789, 0xb0a0, 0x44e1, 0xae, 0x4f, 0xee, 0x99, 0x9b, 0x1b, 0x7f, 0xbe
DEFINE_GUIDSTRUCT("F5412789-B0A0-44e1-AE4F-EE999B1B7FBE", KSNODE_BDA_PID_FILTER);
#define KSNODE_BDA_PID_FILTER DEFINE_GUIDNAMED(KSNODE_BDA_PID_FILTER)


#define STATIC_KSNODE_BDA_IP_SINK \
    0x71985f4e, 0x1ca1, 0x11d3, 0x9c, 0xc8, 0x0, 0xc0, 0x4f, 0x79, 0x71, 0xe0
DEFINE_GUIDSTRUCT("71985F4E-1CA1-11d3-9CC8-00C04F7971E0", KSNODE_BDA_IP_SINK);
#define KSNODE_IP_SINK DEFINE_GUIDNAMED(KSNODE_BDA_IP_SINK)

#define STATIC_KSNODE_BDA_VIDEO_ENCODER \
    0xd98429e3, 0x65c9, 0x4ac4, 0x93, 0xaa, 0x76, 0x67, 0x82, 0x83, 0x3b, 0x7a 
DEFINE_GUIDSTRUCT("d98429e3-65c9-4ac4-93aa-766782833b7a", KSNODE_BDA_VIDEO_ENCODER);
#define KSNODE_BDA_VIDEO_ENCODER DEFINE_GUIDNAMED(KSNODE_BDA_VIDEO_ENCODER)

#define STATIC_KSNODE_BDA_PBDA_CAS \
    0xc026869f, 0x7129, 0x4e71, 0x86, 0x96, 0xec, 0x8f, 0x75, 0x29, 0x9b, 0x77
DEFINE_GUIDSTRUCT("C026869F-7129-4e71-8696-EC8F75299B77", KSNODE_BDA_PBDA_CAS);
#define KSNODE_BDA_PBDA_CAS DEFINE_GUIDNAMED(KSNODE_BDA_PBDA_CAS)

#define STATIC_KSNODE_BDA_PBDA_ISDBCAS \
    0xf2cf2ab3, 0x5b9d, 0x40ae, 0xab, 0x7c, 0x4e, 0x7a, 0xd0, 0xbd, 0x1c, 0x52
DEFINE_GUIDSTRUCT("F2CF2AB3-5B9D-40ae-AB7C-4E7AD0BD1C52", KSNODE_BDA_PBDA_ISDBCAS);
#define KSNODE_BDA_PBDA_ISDBCAS DEFINE_GUIDNAMED(KSNODE_BDA_PBDA_ISDBCAS)

#define STATIC_KSNODE_BDA_PBDA_TUNER \
    0xaa5e8286, 0x593c, 0x4979, 0x94, 0x94, 0x46, 0xa2, 0xa9, 0xdf, 0xe0, 0x76
DEFINE_GUIDSTRUCT("AA5E8286-593C-4979-9494-46A2A9DFE076", KSNODE_BDA_PBDA_TUNER);
#define KSNODE_BDA_PBDA_TUNER DEFINE_GUIDNAMED(KSNODE_BDA_PBDA_TUNER)

#define STATIC_KSNODE_BDA_PBDA_MUX \
    0xf88c7787, 0x6678, 0x4f4b, 0xa1, 0x3e, 0xda, 0x9, 0x86, 0x1d, 0x68, 0x2b
DEFINE_GUIDSTRUCT("F88C7787-6678-4f4b-A13E-DA09861D682B", KSNODE_BDA_PBDA_MUX);
#define KSNODE_BDA_PBDA_MUX DEFINE_GUIDNAMED(KSNODE_BDA_PBDA_MUX)

#define STATIC_KSNODE_BDA_PBDA_DRM \
    0x9eeebd03, 0xeea1, 0x450f, 0x96, 0xae, 0x63, 0x3e, 0x6d, 0xe6, 0x3c, 0xce
DEFINE_GUIDSTRUCT("9EEEBD03-EEA1-450f-96AE-633E6DE63CCE", KSNODE_BDA_PBDA_DRM);
#define KSNODE_BDA_PBDA_DRM DEFINE_GUIDNAMED(KSNODE_BDA_PBDA_DRM)

#define STATIC_KSNODE_BDA_DRI_DRM \
    0x4f95ad74, 0xcefb, 0x42d2, 0x94, 0xa9, 0x68, 0xc5, 0xb2, 0xc1, 0xaa, 0xbe
DEFINE_GUIDSTRUCT("4F95AD74-CEFB-42d2-94A9-68C5B2C1AABE", KSNODE_BDA_DRI_DRM);
#define KSNODE_BDA_DRI_DRM DEFINE_GUIDNAMED(KSNODE_BDA_DRI_DRM)

#define STATIC_KSNODE_BDA_TS_SELECTOR \
    0x5eddf185, 0xfed1, 0x4f45, 0x96, 0x85, 0xbb, 0xb7, 0x3c, 0x32, 0x3c, 0xfc
DEFINE_GUIDSTRUCT("5EDDF185-FED1-4f45-9685-BBB73C323CFC", KSNODE_BDA_TS_SELECTOR);
#define KSNODE_BDA_TS_SELECTOR DEFINE_GUIDNAMED(KSNODE_BDA_TS_SELECTOR)


//===========================================================================
//
// IPSink PINNAME GUID
//
//===========================================================================

#define STATIC_PINNAME_IPSINK_INPUT \
    0x3fdffa70L, 0xac9a, 0x11d2, 0x8f, 0x17, 0x00, 0xc0, 0x4f, 0x79, 0x71, 0xe2
DEFINE_GUIDSTRUCT("3fdffa70-ac9a-11d2-8f17-00c04f7971e2", PINNAME_IPSINK_INPUT);
#define PINNAME_IPSINK_INPUT   DEFINE_GUIDNAMED(PINNAME_IPSINK_INPUT)


//===========================================================================
//
// BDA IPSink Categories/Types
//
//===========================================================================


#define STATIC_KSDATAFORMAT_TYPE_BDA_IP\
    0xe25f7b8e, 0xcccc, 0x11d2, 0x8f, 0x25, 0x0, 0xc0, 0x4f, 0x79, 0x71, 0xe2
DEFINE_GUIDSTRUCT("e25f7b8e-cccc-11d2-8f25-00c04f7971e2", KSDATAFORMAT_TYPE_BDA_IP);
#define KSDATAFORMAT_TYPE_BDA_IP  DEFINE_GUIDNAMED(KSDATAFORMAT_TYPE_BDA_IP)

#define STATIC_KSDATAFORMAT_SUBTYPE_BDA_IP\
    0x5a9a213c, 0xdb08, 0x11d2, 0x8f, 0x32, 0x0, 0xc0, 0x4f, 0x79, 0x71, 0xe2
DEFINE_GUIDSTRUCT("5a9a213c-db08-11d2-8f32-00c04f7971e2", KSDATAFORMAT_SUBTYPE_BDA_IP);
#define KSDATAFORMAT_SUBTYPE_BDA_IP  DEFINE_GUIDNAMED(KSDATAFORMAT_SUBTYPE_BDA_IP)

#define STATIC_KSDATAFORMAT_SPECIFIER_BDA_IP\
    0x6b891420, 0xdb09, 0x11d2, 0x8f, 0x32, 0x0, 0xc0, 0x4f, 0x79, 0x71, 0xe2
DEFINE_GUIDSTRUCT("6B891420-DB09-11d2-8F32-00C04F7971E2", KSDATAFORMAT_SPECIFIER_BDA_IP);
#define KSDATAFORMAT_SPECIFIER_BDA_IP  DEFINE_GUIDNAMED(KSDATAFORMAT_SPECIFIER_BDA_IP)



#define STATIC_KSDATAFORMAT_TYPE_BDA_IP_CONTROL\
    0xdadd5799, 0x7d5b, 0x4b63, 0x80, 0xfb, 0xd1, 0x44, 0x2f, 0x26, 0xb6, 0x21
DEFINE_GUIDSTRUCT("DADD5799-7D5B-4b63-80FB-D1442F26B621", KSDATAFORMAT_TYPE_BDA_IP_CONTROL);
#define KSDATAFORMAT_TYPE_BDA_IP_CONTROL  DEFINE_GUIDNAMED(KSDATAFORMAT_TYPE_BDA_IP_CONTROL)

#define STATIC_KSDATAFORMAT_SUBTYPE_BDA_IP_CONTROL\
    0x499856e8, 0xe85b, 0x48ed, 0x9b, 0xea, 0x41, 0xd, 0xd, 0xd4, 0xef, 0x81
DEFINE_GUIDSTRUCT("499856E8-E85B-48ed-9BEA-410D0DD4EF81", KSDATAFORMAT_SUBTYPE_BDA_IP_CONTROL);
#define KSDATAFORMAT_SUBTYPE_BDA_IP_CONTROL  DEFINE_GUIDNAMED(KSDATAFORMAT_SUBTYPE_BDA_IP_CONTROL)


//===========================================================================
//
// MPE PINNAME GUID
//
//===========================================================================

#define STATIC_PINNAME_MPE \
    0xc1b06d73L, 0x1dbb, 0x11d3, 0x8f, 0x46, 0x00, 0xC0, 0x4f, 0x79, 0x71, 0xE2
DEFINE_GUIDSTRUCT("C1B06D73-1DBB-11d3-8F46-00C04F7971E2", PINNAME_MPE);
#define PINNAME_MPE   DEFINE_GUIDNAMED(PINNAME_MPE)


/////////////////////////////////////////////////////////////
//
// BDA MPE Categories/Types
//
#define STATIC_KSDATAFORMAT_TYPE_MPE \
    0x455f176c, 0x4b06, 0x47ce, 0x9a, 0xef, 0x8c, 0xae, 0xf7, 0x3d, 0xf7, 0xb5
DEFINE_GUIDSTRUCT("455F176C-4B06-47ce-9AEF-8CAEF73DF7B5", KSDATAFORMAT_TYPE_MPE);
#define KSDATAFORMAT_TYPE_MPE  DEFINE_GUIDNAMED(KSDATAFORMAT_TYPE_MPE)


//===========================================================================
//
// BDA NETWORK TYPE GUID
//
//===========================================================================

#define STATIC_DIGITAL_CABLE_NETWORK_TYPE \
    0x143827AB, 0xF77B, 0x498d, 0x81, 0xCA, 0x5A, 0x00, 0x7A, 0xEC, 0x28, 0xBF
DEFINE_GUIDSTRUCT("143827AB-F77B-498d-81CA-5A007AEC28BF", DIGITAL_CABLE_NETWORK_TYPE);
#define DIGITAL_CABLE_NETWORK_TYPE   DEFINE_GUIDNAMED(DIGITAL_CABLE_NETWORK_TYPE)

#define STATIC_ANALOG_TV_NETWORK_TYPE \
    0xb820d87e, 0xe0e3, 0x478f, 0x8a, 0x38, 0x4e, 0x13, 0xf7, 0xb3, 0xdf, 0x42
DEFINE_GUIDSTRUCT("B820D87E-E0E3-478f-8A38-4E13F7B3DF42", ANALOG_TV_NETWORK_TYPE);
#define ANALOG_TV_NETWORK_TYPE   DEFINE_GUIDNAMED(ANALOG_TV_NETWORK_TYPE)

#define STATIC_ANALOG_AUXIN_NETWORK_TYPE \
    0x742EF867, 0x9E1, 0x40A3, 0x82, 0xD3, 0x96, 0x69, 0xBA, 0x35, 0x32, 0x5F
DEFINE_GUIDSTRUCT("742EF867-09E1-40A3-82D3-9669BA35325F", ANALOG_AUXIN_NETWORK_TYPE);
#define ANALOG_AUXIN_NETWORK_TYPE   DEFINE_GUIDNAMED(ANALOG_AUXIN_NETWORK_TYPE)

#define STATIC_ANALOG_FM_NETWORK_TYPE \
    0x7728087b, 0x2bb9, 0x4e30, 0x80, 0x78, 0x44, 0x94, 0x76, 0xe5, 0x9d, 0xbb
DEFINE_GUIDSTRUCT("7728087B-2BB9-4E30-8078-449476E59DBB", ANALOG_FM_NETWORK_TYPE);
#define ANALOG_FM_NETWORK_TYPE   DEFINE_GUIDNAMED(ANALOG_FM_NETWORK_TYPE)

#define STATIC_ISDB_TERRESTRIAL_TV_NETWORK_TYPE \
    0x95037f6f, 0x3ac7, 0x4452, 0xb6, 0xc4, 0x45, 0xa9, 0xce, 0x92, 0x92, 0xa2
DEFINE_GUIDSTRUCT("95037F6F-3AC7-4452-B6C4-45A9CE9292A2", ISDB_TERRESTRIAL_TV_NETWORK_TYPE); 
#define ISDB_TERRESTRIAL_TV_NETWORK_TYPE DEFINE_GUIDNAMED(ISDB_TERRESTRIAL_TV_NETWORK_TYPE) 
#define STATIC_ISDB_T_NETWORK_TYPE \
    0xfc3855a6, 0xc901, 0x4f2e, 0xab, 0xa8, 0x90, 0x81, 0x5a, 0xfc, 0x6c, 0x83
DEFINE_GUIDSTRUCT("fc3855a6-c901-4f2e-aba8-90815afc6c83", ISDB_T_NETWORK_TYPE);
#define ISDB_T_NETWORK_TYPE   DEFINE_GUIDNAMED(ISDB_T_NETWORK_TYPE)

#define STATIC_ISDB_SATELLITE_TV_NETWORK_TYPE \
    0xb0a4e6a0, 0x6a1a, 0x4b83, 0xbb, 0x5b, 0x90, 0x3e, 0x1d, 0x90, 0xe6, 0xb6 
DEFINE_GUIDSTRUCT("B0A4E6A0-6A1A-4B83-BB5B-903E1D90E6B6", ISDB_SATELLITE_TV_NETWORK_TYPE); 
#define ISDB_SATELLITE_TV_NETWORK_TYPE DEFINE_GUIDNAMED(ISDB_SATELLITE_TV_NETWORK_TYPE)
#define STATIC_ISDB_S_NETWORK_TYPE \
    0xa1e78202, 0x1459, 0x41b1, 0x9c, 0xa9, 0x2a, 0x92, 0x58, 0x7a, 0x42, 0xcc
DEFINE_GUIDSTRUCT("a1e78202-1459-41b1-9ca9-2a92587a42cc", ISDB_S_NETWORK_TYPE);
#define ISDB_S_NETWORK_TYPE   DEFINE_GUIDNAMED(ISDB_S_NETWORK_TYPE)

#define STATIC_ISDB_CABLE_TV_NETWORK_TYPE \
    0xc974ddb5, 0x41fe, 0x4b25, 0x97, 0x41, 0x92, 0xf0, 0x49, 0xf1, 0xd5, 0xd1 
DEFINE_GUIDSTRUCT("C974DDB5-41FE-4B25-9741-92F049F1D5D1", ISDB_CABLE_TV_NETWORK_TYPE); 
#define ISDB_CABLE_TV_NETWORK_TYPE DEFINE_GUIDNAMED(ISDB_CABLE_TV_NETWORK_TYPE)

#define STATIC_DIRECT_TV_SATELLITE_TV_NETWORK_TYPE \
    0x93b66fb5, 0x93d4, 0x4323, 0x92, 0x1c, 0xc1, 0xf5, 0x2d, 0xf6, 0x1d, 0x3f 
DEFINE_GUIDSTRUCT("93B66FB5-93D4-4323-921C-C1F52DF61D3F", DIRECT_TV_SATELLITE_TV_NETWORK_TYPE); 
#define DIRECT_TV_SATELLITE_TV_NETWORK_TYPE DEFINE_GUIDNAMED(DIRECT_TV_SATELLITE_TV_NETWORK_TYPE)

#define STATIC_ECHOSTAR_SATELLITE_TV_NETWORK_TYPE \
    0xc4f6b31b, 0xc6bf, 0x4759, 0x88, 0x6f, 0xa7, 0x38, 0x6d, 0xca, 0x27, 0xa0 
DEFINE_GUIDSTRUCT("C4F6B31B-C6BF-4759-886F-A7386DCA27A0", ECHOSTAR_SATELLITE_TV_NETWORK_TYPE); 
#define ECHOSTAR_SATELLITE_TV_NETWORK_TYPE DEFINE_GUIDNAMED(ECHOSTAR_SATELLITE_TV_NETWORK_TYPE)

// Same as CLSID_ATSCNetworkProvider in uuids.h
#define STATIC_ATSC_TERRESTRIAL_TV_NETWORK_TYPE \
    0x0dad2fdd, 0x5fd7, 0x11d3, 0x8f, 0x50, 0x00, 0xc0, 0x4f, 0x79, 0x71, 0xe2
DEFINE_GUIDSTRUCT("0DAD2FDD-5FD7-11D3-8F50-00C04F7971E2", ATSC_TERRESTRIAL_TV_NETWORK_TYPE); 
#define ATSC_TERRESTRIAL_TV_NETWORK_TYPE DEFINE_GUIDNAMED(ATSC_TERRESTRIAL_TV_NETWORK_TYPE) 

// Same as CLSID_DVBTNetworkProvider in uuids.h
#define STATIC_DVB_TERRESTRIAL_TV_NETWORK_TYPE \
    0x216c62df, 0x6d7f, 0x4e9a, 0x85, 0x71, 0x05, 0xf1, 0x4e, 0xdb, 0x76, 0x6a
DEFINE_GUIDSTRUCT("216C62DF-6D7F-4E9A-8571-05F14EDB766A", DVB_TERRESTRIAL_TV_NETWORK_TYPE); 
#define DVB_TERRESTRIAL_TV_NETWORK_TYPE DEFINE_GUIDNAMED(DVB_TERRESTRIAL_TV_NETWORK_TYPE) 

// Same as CLSID_DVBSNetworkProvider in uuids.h
#define STATIC_BSKYB_TERRESTRIAL_TV_NETWORK_TYPE \
    0x9E9E46C6, 0x3ABA, 0x4f08, 0xAD, 0x0E, 0xCC, 0x5A, 0xC8, 0x14, 0x8C, 0x2B
DEFINE_GUIDSTRUCT("9E9E46C6-3ABA-4f08-AD0E-CC5AC8148C2B", BSKYB_TERRESTRIAL_TV_NETWORK_TYPE);
#define BSKYB_TERRESTRIAL_TV_NETWORK_TYPE DEFINE_GUIDNAMED(BSKYB_TERRESTRIAL_TV_NETWORK_TYPE)

// Same as CLSID_DVBSNetworkProvider in uuids.h
#define STATIC_DVB_SATELLITE_TV_NETWORK_TYPE \
    0xfa4b375a, 0x45b4, 0x4d45, 0x84, 0x40, 0x26, 0x39, 0x57, 0xb1, 0x16, 0x23 
DEFINE_GUIDSTRUCT("FA4B375A-45B4-4D45-8440-263957B11623", DVB_SATELLITE_TV_NETWORK_TYPE); 
#define DVB_SATELLITE_TV_NETWORK_TYPE DEFINE_GUIDNAMED(DVB_SATELLITE_TV_NETWORK_TYPE)

// Same as CLSID_DVBCNetworkProvider in uuids.h
#define STATIC_DVB_CABLE_TV_NETWORK_TYPE \
    0xdc0c0fe7, 0x485, 0x4266, 0xb9, 0x3f, 0x68, 0xfb, 0xf8, 0x0e, 0xd8, 0x34 
DEFINE_GUIDSTRUCT("DC0C0FE7-0485-4266-B93F-68FBF80ED834", DVB_CABLE_TV_NETWORK_TYPE); 
#define DVB_CABLE_TV_NETWORK_TYPE DEFINE_GUIDNAMED(DVB_CABLE_TV_NETWORK_TYPE) 


//===========================================================================
//
// PBDA EVENT GUIDS
//
//===========================================================================
#define STATIC_BDA_DEBUG_DATA_AVAILABLE \
    0x69C24F54, 0x9983, 0x497e, 0xb4, 0x15, 0x28, 0x2b, 0xe4, 0xc5, 0x55, 0xfb 
DEFINE_GUIDSTRUCT("69C24F54-9983-497e-B415-282BE4C555FB", BDA_DEBUG_DATA_AVAILABLE ); 
#define BDA_DEBUG_DATA_AVAILABLE  DEFINE_GUIDNAMED(BDA_DEBUG_DATA_AVAILABLE) 


#define STATIC_BDA_DEBUG_DATA_TYPE_STRING \
    0xa806e767, 0xde5c, 0x430c, 0x80, 0xbf, 0xa2, 0x1e, 0xbe, 0x06, 0xc7, 0x48 
DEFINE_GUIDSTRUCT("a806e767-de5c-430c-80bf-a21ebe06c748", BDA_DEBUG_DATA_TYPE_STRING ); 
#define BDA_DEBUG_DATA_TYPE_STRING  DEFINE_GUIDNAMED(BDA_DEBUG_DATA_TYPE_STRING) 


#define STATIC_EVENTID_BDA_IsdbCASResponse \
    0xd4cb1966, 0x41bc, 0x4ced, 0x9a, 0x20, 0xfd, 0xce, 0xac, 0x78, 0xf7, 0x0d	
DEFINE_GUIDSTRUCT("D4CB1966-41BC-4ced-9A20-FDCEAC78F70D", EVENTID_BDA_IsdbCASResponse);
#define EVENTID_BDA_IsdbCASResponse DEFINE_GUIDNAMED(EVENTID_BDA_IsdbCASResponse)


#define STATIC_EVENTID_BDA_CASRequestTuner \
    0xcf39a9d8, 0xf5d3, 0x4685, 0xbe, 0x57, 0xed, 0x81, 0xdb, 0xa4, 0x6b, 0x27	
DEFINE_GUIDSTRUCT("CF39A9D8-F5D3-4685-BE57-ED81DBA46B27", EVENTID_BDA_CASRequestTuner);
#define EVENTID_BDA_CASRequestTuner DEFINE_GUIDNAMED(EVENTID_BDA_CASRequestTuner)


#define STATIC_EVENTID_BDA_CASReleaseTuner \
    0x20c1a16b, 0x441f, 0x49a5, 0xbb, 0x5c, 0xe9, 0xa0, 0x44, 0x95, 0xc6, 0xc1	
DEFINE_GUIDSTRUCT("20C1A16B-441F-49a5-BB5C-E9A04495C6C1", EVENTID_BDA_CASReleaseTuner);
#define EVENTID_BDA_CASReleaseTuner DEFINE_GUIDNAMED(EVENTID_BDA_CASReleaseTuner)


#define STATIC_EVENTID_BDA_CASOpenMMI \
    0x85dac915, 0xe593, 0x410d, 0x84, 0x71, 0xd6, 0x81, 0x21, 0x05, 0xf2, 0x8e	
DEFINE_GUIDSTRUCT("85DAC915-E593-410d-8471-D6812105F28E", EVENTID_BDA_CASOpenMMI);
#define EVENTID_BDA_CASOpenMMI DEFINE_GUIDNAMED(EVENTID_BDA_CASOpenMMI)


#define STATIC_EVENTID_BDA_CASCloseMMI \
    0x5d0f550f, 0xde2e, 0x479d, 0x83, 0x45, 0xec, 0x0e, 0x95, 0x57, 0xe8, 0xa2	
DEFINE_GUIDSTRUCT("5D0F550F-DE2E-479d-8345-EC0E9557E8A2", EVENTID_BDA_CASCloseMMI);
#define EVENTID_BDA_CASCloseMMI DEFINE_GUIDNAMED(EVENTID_BDA_CASCloseMMI)


#define STATIC_EVENTID_BDA_CASBroadcastMMI \
    0x676876f0, 0x1132, 0x404c, 0xa7, 0xca, 0xe7, 0x20, 0x69, 0xa9, 0xd5, 0x4f	
DEFINE_GUIDSTRUCT("676876F0-1132-404c-A7CA-E72069A9D54F", EVENTID_BDA_CASBroadcastMMI);
#define EVENTID_BDA_CASBroadcastMMI DEFINE_GUIDNAMED(EVENTID_BDA_CASBroadcastMMI)


#define STATIC_EVENTID_BDA_TunerSignalLock \
    0x1872e740, 0xf573, 0x429b, 0xa0, 0x0e, 0xd9, 0xc1, 0xe4, 0x08, 0xaf, 0x09	
DEFINE_GUIDSTRUCT("1872E740-F573-429b-A00E-D9C1E408AF09", EVENTID_BDA_TunerSignalLock);
#define EVENTID_BDA_TunerSignalLock DEFINE_GUIDNAMED(EVENTID_BDA_TunerSignalLock)

#define STATIC_EVENTID_BDA_TunerNoSignal \
    0xe29b382b, 0x1edd, 0x4930, 0xbc, 0x46, 0x68, 0x2f, 0xd7, 0x2d, 0x2d, 0xfb	
DEFINE_GUIDSTRUCT("E29B382B-1EDD-4930-BC46-682FD72D2DFB", EVENTID_BDA_TunerNoSignal);
#define EVENTID_BDA_TunerNoSignal DEFINE_GUIDNAMED(EVENTID_BDA_TunerNoSignal)

#define STATIC_EVENTID_BDA_GPNVValueUpdate \
    0xff75c68c, 0xf416, 0x4e7e, 0xbf, 0x17, 0x6d, 0x55, 0xc5, 0xdf, 0x15, 0x75	
DEFINE_GUIDSTRUCT("FF75C68C-F416-4e7e-BF17-6D55C5DF1575", EVENTID_BDA_GPNVValueUpdate);
#define EVENTID_BDA_GPNVValueUpdate DEFINE_GUIDNAMED(EVENTID_BDA_GPNVValueUpdate)


#define STATIC_EVENTID_BDA_UpdateDrmStatus \
    0x65a6f681, 0x1462, 0x473b, 0x88, 0xce, 0xcb, 0x73, 0x14, 0x27, 0xbd, 0xb5	
DEFINE_GUIDSTRUCT("65A6F681-1462-473b-88CE-CB731427BDB5", EVENTID_BDA_UpdateDrmStatus);
#define EVENTID_BDA_UpdateDrmStatus DEFINE_GUIDNAMED(EVENTID_BDA_UpdateDrmStatus)


#define STATIC_EVENTID_BDA_UpdateScanState \
    0x55702B50, 0x7B49, 0x42B8, 0xA8, 0x2F, 0x4A, 0xFB, 0x69, 0x1B, 0x06, 0x28	
DEFINE_GUIDSTRUCT("55702B50-7B49-42B8-A82F-4AFB691B0628", EVENTID_BDA_UpdateScanState);
#define EVENTID_BDA_UpdateScanState DEFINE_GUIDNAMED(EVENTID_BDA_UpdateScanState)


#define STATIC_EVENTID_BDA_GuideDataAvailable \
    0x98db717a, 0x478a, 0x4cd4, 0x92, 0xd0, 0x95, 0xf6, 0x6b, 0x89, 0xe5, 0xb1	
DEFINE_GUIDSTRUCT("98DB717A-478A-4cd4-92D0-95F66B89E5B1", EVENTID_BDA_GuideDataAvailable);
#define EVENTID_BDA_GuideDataAvailable DEFINE_GUIDNAMED(EVENTID_BDA_GuideDataAvailable)


#define STATIC_EVENTID_BDA_GuideServiceInformationUpdated \
    0xa1c3ea2b, 0x175f, 0x4458, 0xb7, 0x35, 0x50, 0x7d, 0x22, 0xdb, 0x23, 0xa6	
DEFINE_GUIDSTRUCT("A1C3EA2B-175F-4458-B735-507D22DB23A6", EVENTID_BDA_GuideServiceInformationUpdated);
#define EVENTID_BDA_GuideServiceInformationUpdated DEFINE_GUIDNAMED(EVENTID_BDA_GuideServiceInformationUpdated)


#define STATIC_EVENTID_BDA_GuideDataError \
    0xAC33C448, 0x6F73, 0x4fd7, 0xB3, 0x41, 0x59, 0x4C, 0x36, 0x0D, 0x8D, 0x74	
DEFINE_GUIDSTRUCT("AC33C448-6F73-4fd7-B341-594C360D8D74", EVENTID_BDA_GuideDataError);
#define EVENTID_BDA_GuideDataError DEFINE_GUIDNAMED(EVENTID_BDA_GuideDataError)


#define STATIC_EVENTID_BDA_DiseqCResponseAvailable \
    0xefa628f8, 0x1f2c, 0x4b67, 0x9e, 0xa5, 0xac, 0xf6, 0xfa, 0x9a, 0x1f, 0x36	
DEFINE_GUIDSTRUCT("EFA628F8-1F2C-4b67-9EA5-ACF6FA9A1F36", EVENTID_BDA_DiseqCResponseAvailable);
#define EVENTID_BDA_DiseqCResponseAvailable DEFINE_GUIDNAMED(EVENTID_BDA_DiseqCResponseAvailable)

#define STATIC_EVENTID_BDA_LbigsOpenConnection \
    0x356207B2, 0x6F31, 0x4eb0, 0xa2, 0x71, 0xb3, 0xfa, 0x6b, 0xb7, 0x68, 0x0f	
DEFINE_GUIDSTRUCT("356207B2-6F31-4eb0-A271-B3FA6BB7680F", EVENTID_BDA_LbigsOpenConnection);
#define EVENTID_BDA_LbigsOpenConnection DEFINE_GUIDNAMED(EVENTID_BDA_LbigsOpenConnection)

#define STATIC_EVENTID_BDA_LbigsSendData \
    0x1123277B, 0xF1C6, 0x4154, 0x8b, 0x0d, 0x48, 0xe6, 0x15, 0x70, 0x59, 0xaa	
DEFINE_GUIDSTRUCT("1123277B-F1C6-4154-8B0D-48E6157059AA", EVENTID_BDA_LbigsSendData);
#define EVENTID_BDA_LbigsSendData DEFINE_GUIDNAMED(EVENTID_BDA_LbigsSendData)

#define STATIC_EVENTID_BDA_LbigsCloseConnectionHandle \
    0xC2F08B99, 0x65EF, 0x4314, 0x96, 0x71, 0xe9, 0x9d, 0x4c, 0xce, 0x0b, 0xae	
DEFINE_GUIDSTRUCT("C2F08B99-65EF-4314-9671-E99D4CCE0BAE", EVENTID_BDA_LbigsCloseConnectionHandle);
#define EVENTID_BDA_LbigsCloseConnectionHandle DEFINE_GUIDNAMED(EVENTID_BDA_LbigsCloseConnectionHandle)

#define STATIC_EVENTID_BDA_EncoderSignalLock \
    0x5ec90eb9, 0x39fa, 0x4cfc, 0xb9, 0x3f, 0x00, 0xbb, 0x11, 0x07, 0x7f, 0x5e
DEFINE_GUIDSTRUCT("5EC90EB9-39FA-4CFC-B93F-00BB11077F5E", EVENTID_BDA_EncoderSignalLock);
#define EVENTID_BDA_EncoderSignalLock DEFINE_GUIDNAMED(EVENTID_BDA_EncoderSignalLock)


#define STATIC_EVENTID_BDA_FdcStatus \
    0x05f25366, 0xd0eb, 0x43d2, 0xbc, 0x3c, 0x68, 0x2b, 0x86, 0x3d, 0xf1, 0x42
DEFINE_GUIDSTRUCT("05F25366-D0EB-43d2-BC3C-682B863DF142", EVENTID_BDA_FdcStatus);
#define EVENTID_BDA_FdcStatus DEFINE_GUIDNAMED(EVENTID_BDA_FdcStatus)


#define STATIC_EVENTID_BDA_FdcTableSection \
    0x6a0cD757, 0x4ce3,0x4e5b, 0x94, 0x44, 0x71, 0x87, 0xb8, 0x71, 0x52, 0xc5
DEFINE_GUIDSTRUCT("6A0CD757-4CE3-4e5b-9444-7187B87152C5", EVENTID_BDA_FdcTableSection);
#define EVENTID_BDA_FdcTableSection DEFINE_GUIDNAMED(EVENTID_BDA_FdcTableSection)


#define STATIC_EVENTID_BDA_TransprtStreamSelectorInfo \
    0xc40f9f85, 0x9d0, 0x489c, 0x9e, 0x9c, 0x0a, 0xbb, 0xb5, 0x69, 0x51, 0xb0
DEFINE_GUIDSTRUCT("C40F9F85-09D0-489c-9E9C-0ABBB56951B0", EVENTID_BDA_TransprtStreamSelectorInfo);
#define EVENTID_BDA_TransprtStreamSelectorInfo DEFINE_GUIDNAMED(EVENTID_BDA_TransprtStreamSelectorInfo)


#define STATIC_EVENTID_BDA_RatingPinReset \
    0xc6e048c0, 0xc574, 0x4c26, 0xbc, 0xda, 0x2f, 0x4d, 0x35, 0xeb, 0x5e, 0x85
DEFINE_GUIDSTRUCT("C6E048C0-C574-4C26-BCDA-2F4D35EB5E85", EVENTID_BDA_RatingPinReset);
#define EVENTID_BDA_RatingPinReset DEFINE_GUIDNAMED(EVENTID_BDA_RatingPinReset)


//===========================================================================
//
// PBDA COMMON USE GUIDs
//
//===========================================================================

#define STATIC_PBDA_ALWAYS_TUNE_IN_MUX \
    0x1E1D7141, 0x583F, 0x4ac2, 0xB0, 0x19, 0x1F, 0x43, 0x0E, 0xDA, 0x0F, 0x4C	
DEFINE_GUIDSTRUCT("1E1D7141-583F-4ac2-B019-1F430EDA0F4C", PBDA_ALWAYS_TUNE_IN_MUX); 
#define PBDA_ALWAYS_TUNE_IN_MUX DEFINE_GUIDNAMED(PBDA_ALWAYS_TUNE_IN_MUX) 



#if defined(__cplusplus)
}
#endif // defined(__cplusplus)

#endif // !defined(_BDAMEDIA_)


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


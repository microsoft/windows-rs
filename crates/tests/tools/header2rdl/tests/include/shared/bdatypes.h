//------------------------------------------------------------------------------
// File: BDATypes.h
//
// Desc: Typedefs and enums needed by both the WDM drivers and the user mode
//       COM interfaces.
//
// Copyright (c) 1999 - 2004, Microsoft Corporation.  All rights reserved.
//------------------------------------------------------------------------------


#ifndef _BDATYPES_

#define _BDATYPES_      1

// !!!! do not #pragma once, we use this file twice(once for native and once for mgd) in managed interop
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#include <exposeenums2managed.h>

/* Utility Macros */

#define MIN_DIMENSION   1


//===========================================================================
//
//  BDA Topology Structures
//
//===========================================================================

#ifndef MANAGED_ENUMS

typedef struct _BDA_TEMPLATE_CONNECTION
{
    ULONG   FromNodeType;
    ULONG   FromNodePinType;
    ULONG   ToNodeType;
    ULONG   ToNodePinType;
}BDA_TEMPLATE_CONNECTION, *PBDA_TEMPLATE_CONNECTION;


typedef struct _BDA_TEMPLATE_PIN_JOINT
{
    ULONG   uliTemplateConnection;
    ULONG   ulcInstancesMax;
}BDA_TEMPLATE_PIN_JOINT, *PBDA_TEMPLATE_PIN_JOINT;
#endif


//===========================================================================
//
//  BDA Events
//
//===========================================================================

//  In-band Event IDs
//
ENUM BDA_EVENT_ID {
    BDA_EVENT_SIGNAL_LOSS = 0,
    BDA_EVENT_SIGNAL_LOCK,
    BDA_EVENT_DATA_START,
    BDA_EVENT_DATA_STOP,
    BDA_EVENT_CHANNEL_ACQUIRED,
    BDA_EVENT_CHANNEL_LOST,
    BDA_EVENT_CHANNEL_SOURCE_CHANGED,
    BDA_EVENT_CHANNEL_ACTIVATED,
    BDA_EVENT_CHANNEL_DEACTIVATED,
    BDA_EVENT_SUBCHANNEL_ACQUIRED,
    BDA_EVENT_SUBCHANNEL_LOST,
    BDA_EVENT_SUBCHANNEL_SOURCE_CHANGED,
    BDA_EVENT_SUBCHANNEL_ACTIVATED,
    BDA_EVENT_SUBCHANNEL_DEACTIVATED,
    BDA_EVENT_ACCESS_GRANTED,
    BDA_EVENT_ACCESS_DENIED,
    BDA_EVENT_OFFER_EXTENDED,
    BDA_EVENT_PURCHASE_COMPLETED,
    BDA_EVENT_SMART_CARD_INSERTED,
    BDA_EVENT_SMART_CARD_REMOVED
} BDA_EVENT_ID, *PBDA_EVENT_ID;



//===========================================================================
//
//  KSSTREAM_HEADER extensions for BDA
//
//===========================================================================

#ifndef MANAGED_ENUMS
typedef struct tagKS_BDA_FRAME_INFO {
    ULONG                   ExtendedHeaderSize; // Size of this extended header
    DWORD                   dwFrameFlags;  //
    ULONG                   ulEvent; //
    ULONG                   ulChannelNumber; //
    ULONG                   ulSubchannelNumber; //
    ULONG                   ulReason; //
} KS_BDA_FRAME_INFO, *PKS_BDA_FRAME_INFO;


//------------------------------------------------------------
//
//  BDA Network Ethernet Filter Property Set
//
// {71985F43-1CA1-11d3-9CC8-00C04F7971E0}
//
typedef struct _BDA_ETHERNET_ADDRESS {
    BYTE    rgbAddress[6];
} BDA_ETHERNET_ADDRESS, *PBDA_ETHERNET_ADDRESS;

typedef struct _BDA_ETHERNET_ADDRESS_LIST {
    ULONG               ulcAddresses;
    BDA_ETHERNET_ADDRESS    rgAddressl[MIN_DIMENSION];
} BDA_ETHERNET_ADDRESS_LIST, * PBDA_ETHERNET_ADDRESS_LIST;

#endif

ENUM BDA_MULTICAST_MODE {
    BDA_PROMISCUOUS_MULTICAST = 0,
    BDA_FILTERED_MULTICAST,
    BDA_NO_MULTICAST
} BDA_MULTICAST_MODE, *PBDA_MULTICAST_MODE;


//------------------------------------------------------------
//
//  BDA Network IPv4 Filter Property Set
//
// {71985F44-1CA1-11d3-9CC8-00C04F7971E0}
//
#ifndef MANAGED_ENUMS
typedef struct _BDA_IPv4_ADDRESS {
    BYTE    rgbAddress[4];
} BDA_IPv4_ADDRESS, *PBDA_IPv4_ADDRESS;

typedef struct _BDA_IPv4_ADDRESS_LIST {
    ULONG               ulcAddresses;
    BDA_IPv4_ADDRESS    rgAddressl[MIN_DIMENSION];
} BDA_IPv4_ADDRESS_LIST, * PBDA_IPv4_ADDRESS_LIST;

//------------------------------------------------------------
//
//  BDA Network IPv4 Filter Property Set
//
// {E1785A74-2A23-4fb3-9245-A8F88017EF33}
//
typedef struct _BDA_IPv6_ADDRESS {
    BYTE    rgbAddress[6];
} BDA_IPv6_ADDRESS, *PBDA_IPv6_ADDRESS;

typedef struct _BDA_IPv6_ADDRESS_LIST {
    ULONG               ulcAddresses;
    BDA_IPv6_ADDRESS    rgAddressl[MIN_DIMENSION];
} BDA_IPv6_ADDRESS_LIST, * PBDA_IPv6_ADDRESS_LIST;

#endif


//------------------------------------------------------------
//
//
//  BDA Signal Property Set
//
//  {D2F1644B-B409-11d2-BC69-00A0C9EE9E16}
//
ENUM BDA_SIGNAL_STATE {
    BDA_SIGNAL_UNAVAILABLE = 0,
    BDA_SIGNAL_INACTIVE,
    BDA_SIGNAL_ACTIVE
} BDA_SIGNAL_STATE, * PBDA_SIGNAL_STATE;


//------------------------------------------------------------
//
//
//  BDA Change Sync Method Set
//
// {FD0A5AF3-B41D-11d2-9C95-00C04F7971E0}
//
ENUM BDA_CHANGE_STATE {
    BDA_CHANGES_COMPLETE = 0,
    BDA_CHANGES_PENDING
} BDA_CHANGE_STATE, * PBDA_CHANGE_STATE;


//------------------------------------------------------------
//
//
//  BDA Device Configuration Method Set
//
// {71985F45-1CA1-11d3-9CC8-00C04F7971E0}
//


//------------------------------------------------------------
//
//
//  BDA Topology Property Set
//
// {A14EE835-0A23-11d3-9CC7-00C04F7971E0}
//

#ifndef MANAGED_ENUMS
typedef struct _BDANODE_DESCRIPTOR
{
    ULONG               ulBdaNodeType;  // The node type as it is used
                                        // in the BDA template topology

    GUID                guidFunction;   // GUID from BdaMedia.h describing
                                        // the node's function (e.g.
                                        // KSNODE_BDA_RF_TUNER)

    GUID                guidName;       // GUID that can be use to look up
                                        // a displayable name for the node.
} BDANODE_DESCRIPTOR, *PBDANODE_DESCRIPTOR;


//------------------------------------------------------------
//
//
//  BDA Void Transform Property Set
//
// {71985F46-1CA1-11d3-9CC8-00C04F7971E0}
//


//------------------------------------------------------------
//
//
//  BDA Null Transform Property Set
//
// {DDF15B0D-BD25-11d2-9CA0-00C04F7971E0}
//


//------------------------------------------------------------
//
//
//  BDA Frequency Filter Property Set
//
// {71985F47-1CA1-11d3-9CC8-00C04F7971E0}
//


//------------------------------------------------------------
//
//
//  BDA Autodemodulate Property Set
//
// {DDF15B12-BD25-11d2-9CA0-00C04F7971E0}
//


//------------------------------------------------------------
//
//
//  BDA Table Section Property Set
//
// {516B99C5-971C-4aaf-B3F3-D9FDA8A15E16}
//

typedef struct _BDA_TABLE_SECTION
{
    ULONG               ulPrimarySectionId;
    ULONG               ulSecondarySectionId;
    ULONG               ulcbSectionLength;
    ULONG               argbSectionData[MIN_DIMENSION];
} BDA_TABLE_SECTION, *PBDA_TABLE_SECTION;

#endif

#ifndef MANAGED_ENUMS

//------------------------------------------------------------
//
//
//  BDA Diseq Command Property Set
//
// {F84E2AB0-3C6B-45e3-A0FC-8669D4B81F11}
//

typedef struct _BDA_DISEQC_SEND
{
    ULONG   ulRequestId;
    ULONG   ulPacketLength;
    BYTE    argbPacketData[8];
} BDA_DISEQC_SEND, *PBDA_DISEQC_SEND;

typedef struct _BDA_DISEQC_RESPONSE
{
    ULONG   ulRequestId;
    ULONG   ulPacketLength;
    BYTE    argbPacketData[8];
} BDA_DISEQC_RESPONSE, *PBDA_DISEQC_RESPONSE;


#endif
//------------------------------------------------------------
//
//
//  BDA PID Filter Property Set
//
// {D0A67D65-08DF-4fec-8533-E5B550410B85}
//

//---------------------------------------------------------------------
// From IEnumPIDMap interface
//---------------------------------------------------------------------

ENUM MEDIA_SAMPLE_CONTENT {
    MEDIA_TRANSPORT_PACKET,         //  complete TS packet e.g. pass-through mode
    MEDIA_ELEMENTARY_STREAM,        //  PES payloads; audio/video only
    MEDIA_MPEG2_PSI,                //  PAT, PMT, CAT, Private
    MEDIA_TRANSPORT_PAYLOAD         //  gathered TS packet payloads (PES packets, etc...)
} MEDIA_SAMPLE_CONTENT ;

#ifndef MANAGED_ENUMS
typedef struct {
    ULONG                   ulPID ;
    MEDIA_SAMPLE_CONTENT    MediaSampleContent ;
} PID_MAP ;

typedef struct _BDA_PID_MAP
{
    MEDIA_SAMPLE_CONTENT    MediaSampleContent;
    ULONG                   ulcPIDs;
    ULONG                   aulPIDs[MIN_DIMENSION];
} BDA_PID_MAP, *PBDA_PID_MAP;

typedef struct _BDA_PID_UNMAP
{
    ULONG               ulcPIDs;
    ULONG               aulPIDs[MIN_DIMENSION];
} BDA_PID_UNMAP, *PBDA_PID_UNMAP;


//------------------------------------------------------------
//
//
//  BDA CA Property Set
//
// {B0693766-5278-4ec6-B9E1-3CE40560EF5A}
//
typedef struct _BDA_CA_MODULE_UI
{
    ULONG   ulFormat;
    ULONG   ulbcDesc;
    ULONG   ulDesc[MIN_DIMENSION];
} BDA_CA_MODULE_UI, *PBDA_CA_MODULE_UI;

typedef struct _BDA_PROGRAM_PID_LIST
{
    ULONG   ulProgramNumber;
    ULONG   ulcPIDs;
    ULONG   ulPID[MIN_DIMENSION];
} BDA_PROGRAM_PID_LIST, *PBDA_PROGRAM_PID_LIST;

#endif

//------------------------------------------------------------
//
//
//  BDA CA Event Set
//
// {488C4CCC-B768-4129-8EB1-B00A071F9068}
//

#ifndef MANAGED_ENUMS

//=============================================================
// PBDA RESULT parameter definition 
//=============================================================
typedef LONG PBDARESULT;

//=============================================================
//  BDA_DRM_STATUS used by the DRMService
//=============================================================

typedef struct _BDA_DRM_DRMSTATUS 
{
    PBDARESULT lResult;
    GUID    DRMuuid;
    ULONG   ulDrmUuidListStringSize; 
    GUID    argbDrmUuidListString[MIN_DIMENSION];
} BDA_DRM_DRMSTATUS, *PBDA_DRM_DRMSTATUS;


//=============================================================
// PBDA_WMDRM and PBDA_WMDRMTuner structures 
//=============================================================

typedef struct _BDA_WMDRM_STATUS
{
    PBDARESULT lResult;
    ULONG      ulMaxCaptureTokenSize; 
    ULONG      uMaxStreamingPid; 
    ULONG      ulMaxLicense; 
    ULONG      ulMinSecurityLevel; 
    ULONG      ulRevInfoSequenceNumber; 
    ULONGLONG  ulRevInfoIssuedTime; 
    ULONG      ulRevListVersion; 
    ULONG      ulRevInfoTTL; 
    ULONG      ulState;
} BDA_WMDRM_STATUS, *PBDA_WMDRM_STATUS;

typedef struct _BDA_WMDRM_KEYINFOLIST 
{
    PBDARESULT  lResult;
    ULONG       ulKeyuuidBufferLen;
    GUID        argKeyuuidBuffer[MIN_DIMENSION];
} BDA_WMDRM_KEYINFOLIST, *PBDA_WMDRM_KEYINFOLIST;

typedef struct _BDA_BUFFER 
{
    PBDARESULT  lResult;
    ULONG       ulBufferSize;
    BYTE        argbBuffer[MIN_DIMENSION];
} BDA_BUFFER, *PBDA_BUFFER;

//=============================================================
// PBDA - DRM structures used in methods
//=============================================================

typedef struct _BDA_WMDRM_RENEWLICENSE 
{
    PBDARESULT  lResult;
    ULONG       ulDescrambleStatus; 
    ULONG       ulXmrLicenseOutputLength; 
    BYTE        argbXmrLicenceOutputBuffer[MIN_DIMENSION]; //License and Entitlement Token Buffer 
} BDA_WMDRM_RENEWLICENSE, *PBDA_WMDRM_RENEWLICENSE;

typedef struct _BDA_WMDRMTUNER_PIDPROTECTION 
{
    PBDARESULT  lResult;
    GUID        uuidKeyID;  
} BDA_WMDRMTUNER_PIDPROTECTION, *PBDA_WMDRMTUNER_PIDPROTECTION;

typedef struct _BDA_WMDRMTUNER_PURCHASEENTITLEMENT
{
    PBDARESULT  lResult;
    ULONG       ulDescrambleStatus;
    ULONG       ulCaptureTokenLength;
    BYTE        argbCaptureTokenBuffer[MIN_DIMENSION]; 
} BDA_WMDRMTUNER_PURCHASEENTITLEMENT, *PBDA_WMDRMTUNER_PURCHASEENTITLEMENT;

//=============================================================
// PBDA - TUNER structures used in methods
//=============================================================

typedef struct _BDA_TUNER_TUNERSTATE {
    PBDARESULT  lResult;
    ULONG       ulTuneLength; 
    BYTE        argbTuneData [MIN_DIMENSION];
} BDA_TUNER_TUNERSTATE, *PBDA_TUNER_TUNERSTATE;

typedef struct _BDA_TUNER_DIAGNOSTICS {
    PBDARESULT  lResult;
    ULONG       ulSignalLevel;
    ULONG       ulSignalLevelQuality;
    ULONG       ulSignalNoiseRatio; 
} BDA_TUNER_DIAGNOSTICS, *PBDA_TUNER_DIAGNOSTICS;

//=============================================================
// PBDA - STRING structure used in methods
//=============================================================

typedef struct _BDA_STRING 
{
    PBDARESULT  lResult;
    ULONG       ulStringSize;
    BYTE        argbString[MIN_DIMENSION];
} BDA_STRING, *PBDA_STRING;


//=============================================================
// PBDA - SCANNING structures used in methods
//=============================================================

typedef struct _BDA_SCAN_CAPABILTIES 
{
    PBDARESULT  lResult;
    UINT64      ul64AnalogStandardsSupported;    
} BDA_SCAN_CAPABILTIES, *PBDA_SCAN_CAPABILTIES;

typedef struct _BDA_SCAN_STATE 
{
    PBDARESULT  lResult;
    ULONG       ulSignalLock;
    ULONG       ulSecondsLeft;
    ULONG       ulCurrentFrequency;    
} BDA_SCAN_STATE, *PBDA_SCAN_STATE; 

typedef struct _BDA_SCAN_START 
{
    PBDARESULT  lResult;
    ULONG       LowerFrequency;
    ULONG       HigerFrequency;
} BDA_SCAN_START, *PBDA_SCAN_START;


//=============================================================
// PBDA - GUIDE DATA structures used in methods
//=============================================================

typedef struct _BDA_GDDS_DATATYPE 
{
    PBDARESULT  lResult;
    GUID        uuidDataType; 
} BDA_GDDS_DATATYPE, *P_BDA_GDDS_DATATYPE;

typedef struct _BDA_GDDS_DATA 
{
    PBDARESULT  lResult;
    ULONG       ulDataLength; 
    ULONG       ulPercentageProgress;
    BYTE        argbData [MIN_DIMENSION];
} BDA_GDDS_DATA, *P_BDA_GDDS_DATA;


//=============================================================
// PBDA - USER ACTIVITY structures used in methods
//=============================================================

typedef struct _BDA_USERACTIVITY_INTERVAL 
{
    PBDARESULT  lResult;
    ULONG       ulActivityInterval; 
} BDA_USERACTIVITY_INTERVAL, *P_BDA_USERACTIVITY_INTERVAL;


//=============================================================
// PBDA - CAS structures used in methods
//=============================================================

typedef struct _BDA_CAS_CHECK_ENTITLEMENTTOKEN 
{
    PBDARESULT  lResult;
    ULONG       ulDescrambleStatus; 
} BDA_CAS_CHECK_ENTITLEMENTTOKEN, *PBDA_CAS_CHECK_ENTITLEMENTTOKEN;

typedef struct _BDA_CAS_CLOSE_MMIDIALOG 
{
    PBDARESULT  lResult;
    ULONG       SessionResult;
} BDA_CAS_CLOSE_MMIDIALOG, *PBDA_CAS_CLOSE_MMIDIALOG;

typedef struct _BDA_CAS_REQUESTTUNERDATA
{
    UCHAR       ucRequestPriority;
    UCHAR       ucRequestReason;
    UCHAR       ucRequestConsequences;
    ULONG       ulEstimatedTime;
} BDA_CAS_REQUESTTUNERDATA, *PBDA_CAS_REQUESTTUNERDATA;

typedef struct _BDA_CAS_OPENMMIDATA 
{
    ULONG       ulDialogNumber;
    ULONG       ulDialogRequest;
    GUID        uuidDialogType;
    USHORT      usDialogDataLength;
    BYTE        argbDialogData[MIN_DIMENSION];
} BDA_CAS_OPENMMIDATA, *PBDA_CAS_OPENMMIDATA;

typedef struct _BDA_CAS_CLOSEMMIDATA 
{
    ULONG       ulDialogNumber;
} BDA_CAS_CLOSEMMIDATA, *PBDA_CAS_CLOSEMMIDATA;

//=============================================================
// PBDA - ISDB CAS structures used in methods
//=============================================================

#pragma pack(push, 1)

ENUM ISDBCAS_REQUEST_ID {
    ISDBCAS_REQUEST_ID_EMG = 0x38,
    ISDBCAS_REQUEST_ID_EMD = 0x3A,
} ISDBCAS_REQUEST_ID ;

typedef struct _BDA_ISDBCAS_REQUESTHEADER 
{
    BYTE        bInstruction;                   // EMD/EMG
    BYTE        bReserved[3];                   // future use
    ULONG       ulDataLength;                   // byte size of argbIsdbCommand
    BYTE        argbIsdbCommand[MIN_DIMENSION];
} BDA_ISDBCAS_REQUESTHEADER, *PBDA_ISDBCAS_REQUESTHEADER;

typedef struct _BDA_ISDBCAS_RESPONSEDATA 
{
    PBDARESULT  lResult;
    ULONG       ulRequestID;
    ULONG       ulIsdbStatus;
    ULONG       ulIsdbDataSize;
    BYTE        argbIsdbCommandData[MIN_DIMENSION];
} BDA_ISDBCAS_RESPONSEDATA, *PBDA_ISDBCAS_RESPONSEDATA;

typedef struct _BDA_ISDBCAS_EMG_REQ 
{
    BYTE        bCLA;                           // 0x90
    BYTE        bINS;                           // 0x38
    BYTE        bP1;                            // 0x00
    BYTE        bP2;                            // 0x00
    BYTE        bLC;                            // Following bytes - 1(LE)
    BYTE        bCardId[6];                     // from EMM message packet
    BYTE        bProtocol;                      // from EMM message packet
    BYTE        bCABroadcasterGroupId;          // from EMM message packet
    BYTE        bMessageControl;                // from EMM message packet
    BYTE        bMessageCode[MIN_DIMENSION];    // Last byte is reserved as 'LE'
} BDA_ISDBCAS_EMG_REQ, *PBDA_ISDBCAS_EMG_REQ;

#pragma pack(pop)

//=============================================================
// PBDA - MUX structures used in methods
//=============================================================
ENUM MUX_PID_TYPE {
    PID_OTHER = -1,
    PID_ELEMENTARY_STREAM,            //  PES payloads
    PID_MPEG2_SECTION_PSI_SI,       //  ISO 13818_1 Sections PAT, PMT, CAT, Private. Service Information Sections e.g SDT, NIT, EIT, BAT
} MUX_PID_TYPE ;

#pragma pack(push, 2)
typedef struct _BDA_MUX_PIDLISTITEM 
{
    USHORT          usPIDNumber;        //PID number of the stream
    USHORT          usProgramNumber;    //associated Service Id, if applicable
    MUX_PID_TYPE    ePIDType;           //PID Type of the stream if applicable
} BDA_MUX_PIDLISTITEM, *PBDA_MUX_PIDLISTITEM;
#pragma pack(pop)

#endif

//=============================================================
// BDA - TS Selector structures used in methods
//=============================================================

//  |<-------------- bTSInfolength ---------------------->|
//  |                                                     |
//  |                     |                               |
//  | BDA_TS_SELECTORINFO | BDA_TS_SELECTORINFO_ISDBS_EXT |
//  |                     | (for ISDB-S extension)        |
//  |                     |                               |

#pragma pack(push, 1)
typedef struct _BDA_TS_SELECTORINFO
{
    BYTE            bTSInfolength;          // buffer length including extension
    BYTE            bReserved[2];
    GUID            guidNetworkType;        // current type of tuning
    BYTE            bTSIDCount;             // number of usTSID
    USHORT          usTSID[MIN_DIMENSION];
} BDA_TS_SELECTORINFO, *PBDA_TS_SELECTORINFO;

typedef struct _BDA_TS_SELECTORINFO_ISDBS_EXT
{
    BYTE            bTMCC[48];
} BDA_TS_SELECTORINFO_ISDBS_EXT, *PBDA_TS_SELECTORINFO_ISDBS_EXT;
#pragma pack(pop)

//DVB-T2 related L1 signalling information returned in _GETTSINFORMATION

typedef struct _BDA_DVBT2_L1_SIGNALLING_DATA 
{
    BYTE  L1Pre_TYPE;
    BYTE  L1Pre_BWT_S1_S2;
    BYTE  L1Pre_REPETITION_GUARD_PAPR;
    BYTE  L1Pre_MOD_COD_FEC;
    BYTE  L1Pre_POSTSIZE_INFO_PILOT[5];
    BYTE  L1Pre_TX_ID_AVAIL;
    BYTE  L1Pre_CELL_ID[2];
    BYTE  L1Pre_NETWORK_ID[2];
    BYTE  L1Pre_T2SYSTEM_ID[2];
    BYTE  L1Pre_NUM_T2_FRAMES;
    BYTE  L1Pre_NUM_DATA_REGENFLAG_L1POSTEXT[2];
    BYTE  L1Pre_NUMRF_CURRENTRF_RESERVED[2];
    BYTE  L1Pre_CRC32[4];
    BYTE  L1PostData[MIN_DIMENSION];
} BDA_DVBT2_L1_SIGNALLING_DATA, *PBDA_DVBT2_L1_SIGNALLING_DATA;


//=============================================================
// PBDA - RATING structures used in methods
//=============================================================
typedef struct _BDA_RATING_PINRESET 
{
    BYTE    bPinLength;                 //Buffer size including a null termination
    BYTE    argbNewPin[MIN_DIMENSION];  //Null terminated UTF8. Use empty string if disable pin
} BDA_RATING_PINRESET, *PBDA_RATING_PINRESET;

//=============================================================
//
//
//  BDA Tuning Model enumerations
//
//
//=============================================================

// system type for particular DVB Tuning Space instance
ENUM DVBSystemType {
    DVB_Cable,
    DVB_Terrestrial,
    DVB_Satellite,
    ISDB_Terrestrial,
    ISDB_Satellite,
} DVBSystemType;

//------------------------------------------------------------
//
//  BDA Channel Tune Request

ENUM  BDA_Channel {
    BDA_UNDEFINED_CHANNEL = -1,
} BDA_Channel ;


//------------------------------------------------------------
//
//  BDA Component(substream)
//
//  Note: Persistent TS remember preferred component categories by their number.
//        Please update the rgs files at multimedia\dshow\vidctl\msvidctl\res
//        and multimedia\dshow\vidctl\manifests\Video-TVVideoControl.man accordingly
//        if the order/value changes.
//        Also make sure ehiProxy.asmmeta, ehiVidCtl.asmmeta and bdatunepia.asmmeta
//        are properly updated.
//
ENUM ComponentCategory {
    CategoryNotSet = -1,
    CategoryOther = 0,
    CategoryVideo,
    CategoryAudio,
    CategoryText,
    CategorySubtitles,
    CategoryCaptions,
    CategorySuperimpose,
    CategoryData,
    CATEGORY_COUNT,
} ComponentCategory;

// Component Status
ENUM ComponentStatus {
    StatusActive,
    StatusInactive,
    StatusUnavailable,
} ComponentStatus;


//------------------------------------------------------------
//
//  BDA MPEG2 Component Type
//
// from the MPEG2 specification
ENUM MPEG2StreamType {
    BDA_UNITIALIZED_MPEG2STREAMTYPE = -1,
    Reserved1                       = 0x00,
    ISO_IEC_11172_2_VIDEO           = 0x01,
    ISO_IEC_13818_2_VIDEO           = 0x02,
    ISO_IEC_11172_3_AUDIO           = 0x03,
    ISO_IEC_13818_3_AUDIO           = 0x04,
    ISO_IEC_13818_1_PRIVATE_SECTION = 0x05,
    ISO_IEC_13818_1_PES             = 0x06,
    ISO_IEC_13522_MHEG              = 0x07,
    ANNEX_A_DSM_CC                  = 0x08,
    ITU_T_REC_H_222_1               = 0x09,
    ISO_IEC_13818_6_TYPE_A          = 0x0A,
    ISO_IEC_13818_6_TYPE_B          = 0x0B,
    ISO_IEC_13818_6_TYPE_C          = 0x0C,
    ISO_IEC_13818_6_TYPE_D          = 0x0D,
    ISO_IEC_13818_1_AUXILIARY       = 0x0E,
    ISO_IEC_13818_7_AUDIO           = 0x0F,
    ISO_IEC_14496_2_VISUAL          = 0x10,
    ISO_IEC_14496_3_AUDIO           = 0x11,
    ISO_IEC_14496_1_IN_PES          = 0x12,
    ISO_IEC_14496_1_IN_SECTION      = 0x13,
    ISO_IEC_13818_6_DOWNLOAD        = 0x14,
    METADATA_IN_PES                 = 0x15,
    METADATA_IN_SECTION             = 0x16,
    METADATA_IN_DATA_CAROUSEL       = 0x17,
    METADATA_IN_OBJECT_CAROUSEL     = 0x18,
    METADATA_IN_DOWNLOAD_PROTOCOL   = 0x19,
    IRPM_STREAMM                    = 0x1A,
    ITU_T_H264                      = 0x1B,
    ISO_IEC_13818_1_RESERVED        = 0x1C, // continues until 0x7F
    USER_PRIVATE                    = 0x10, // standard says 0x80, retaining for backwards compatibility
    HEVC_VIDEO_OR_TEMPORAL_VIDEO    = 0x24,
    HEVC_TEMPORAL_VIDEO_SUBSET      = 0x25,
    MPEG_H_AUDIO                    = 0x2D, // used for single-stream delivery or for the main stream in case of multi-stream delivery.
    MPEG_H_AUDIO_MS                 = 0x2E, // used for additional (auxiliary) stream in case of MPEG-H multi-stream delivery.
    ISO_IEC_USER_PRIVATE            = 0x80,
    DOLBY_AC3_AUDIO                 = 0x81,
    DOLBY_DIGITAL_PLUS_AUDIO_ATSC   = 0X87
} MPEG2StreamType;



//------------------------------------------------------------
//
//  mpeg-2 transport stride format block; associated with media
//   types MEDIATYPE_Stream/MEDIASUBTYPE_MPEG2_TRANSPORT_STRIDE;
//   *all* format blocks associated with above media type *must*
//   start with the MPEG2_TRANSPORT_STRIDE structure
//

#ifndef MANAGED_ENUMS
typedef struct _MPEG2_TRANSPORT_STRIDE {
    DWORD   dwOffset ;
    DWORD   dwPacketLength ;
    DWORD   dwStride ;
} MPEG2_TRANSPORT_STRIDE, *PMPEG2_TRANSPORT_STRIDE ;
#endif

//------------------------------------------------------------
//
//  BDA ATSC Component Type
//
//
// ATSC made AC3 Audio a descriptor instead of
// defining a user private stream type.

FLAGS ATSCComponentTypeFlags {
    // bit flags for various component type properties
    ATSCCT_AC3 = 0x00000001,
} ATSCComponentTypeFlags;


//------------------------------------------------------------
//
//  BDA Locators
//


ENUM BinaryConvolutionCodeRate {
    BDA_BCC_RATE_NOT_SET = -1,
    BDA_BCC_RATE_NOT_DEFINED = 0,
    BDA_BCC_RATE_1_2 = 1,   // 1/2
    BDA_BCC_RATE_2_3,       // 2/3
    BDA_BCC_RATE_3_4,       // 3/4
    BDA_BCC_RATE_3_5,       // 3/5
    BDA_BCC_RATE_4_5,       // 4/5
    BDA_BCC_RATE_5_6,       // 5/6
    BDA_BCC_RATE_5_11,      // 5/11
    BDA_BCC_RATE_7_8,       // 7/8
    BDA_BCC_RATE_1_4,       // 1/4
    BDA_BCC_RATE_1_3,       // 1/3
    BDA_BCC_RATE_2_5,       // 2/5
    BDA_BCC_RATE_6_7,       // 6/7
    BDA_BCC_RATE_8_9,       // 8/9
    BDA_BCC_RATE_9_10,      // 9/10
    BDA_BCC_RATE_MAX,
} BinaryConvolutionCodeRate;

ENUM FECMethod {
    BDA_FEC_METHOD_NOT_SET = -1,
    BDA_FEC_METHOD_NOT_DEFINED = 0,
    BDA_FEC_VITERBI = 1,    // FEC is a Viterbi Binary Convolution.
    BDA_FEC_RS_204_188,     // The FEC is Reed-Solomon 204/188 (outer FEC)
    BDA_FEC_LDPC,           // Low Density Parity Check error correction code
    BDA_FEC_BCH,            // Bose-Chaudhuri-Hocquenghem multiple error correction binary block code
    BDA_FEC_RS_147_130,     // The FEC is Reed-Solomon 147/130 (outer FEC) DirecTV-DSS
    BDA_FEC_MAX,
} FECMethod;

ENUM ModulationType {
    BDA_MOD_NOT_SET = -1,
    BDA_MOD_NOT_DEFINED = 0,
    BDA_MOD_16QAM = 1,
    BDA_MOD_32QAM,
    BDA_MOD_64QAM,
    BDA_MOD_80QAM,
    BDA_MOD_96QAM,
    BDA_MOD_112QAM,
    BDA_MOD_128QAM,
    BDA_MOD_160QAM,
    BDA_MOD_192QAM,
    BDA_MOD_224QAM,
    BDA_MOD_256QAM,
    BDA_MOD_320QAM,
    BDA_MOD_384QAM,
    BDA_MOD_448QAM,
    BDA_MOD_512QAM,
    BDA_MOD_640QAM,
    BDA_MOD_768QAM,
    BDA_MOD_896QAM,
    BDA_MOD_1024QAM,
    BDA_MOD_QPSK,             // Quadrature Phase Shift Keying (including backwards compatible mode)
    BDA_MOD_BPSK,             // Binary Phase Shift Keying
    BDA_MOD_OQPSK,            // Offset QPSK
    BDA_MOD_8VSB,             // 8-Level Vestigial Sideband
    BDA_MOD_16VSB,            // 16-Level Vestigial Sideband
    BDA_MOD_ANALOG_AMPLITUDE, // std am
    BDA_MOD_ANALOG_FREQUENCY, // std fm
    BDA_MOD_8PSK,             // 8 Phase Shift Keying (including backwards compatible mode)
    BDA_MOD_RF, // analog TV (Video standards such as NTSC/PAL/SECAM specified in IAnalogLocator VideoStandard property)
    BDA_MOD_16APSK,           // DVB-S2 modulation 16-Level APSK
    BDA_MOD_32APSK,           // DVB-S2 modulation 32-Level APSK
    BDA_MOD_NBC_QPSK,         // Non-Backwards Compatible Quadrature Phase Shift Keying
    BDA_MOD_NBC_8PSK,         // Non-Backwards Compatible 8 Phase Shift Keying
    BDA_MOD_DIRECTV,          // DIRECTV DSS
    BDA_MOD_ISDB_T_TMCC,      // Automatic demodulation by Transmission and Multiplexing Configuration Control signal for ISDB-T
    BDA_MOD_ISDB_S_TMCC,      // Automatic demodulation by Transmission and Multiplexing Configuration Control signal for ISDB-S
    BDA_MOD_MAX,
} ModulationType;

#ifdef _MANAGED
FLAGS TAG(ScanModulationTypes) : unsigned int
#else
// this is to silence enum warning C4369
FLAGS TAG(ScanModulationTypes)
#endif
{
    BDA_SCAN_MOD_16QAM    = 0x00000001,
    BDA_SCAN_MOD_32QAM    = 0x00000002,
    BDA_SCAN_MOD_64QAM    = 0x00000004,
    BDA_SCAN_MOD_80QAM    = 0x00000008,
    BDA_SCAN_MOD_96QAM    = 0x00000010,
    BDA_SCAN_MOD_112QAM   = 0x00000020,
    BDA_SCAN_MOD_128QAM   = 0x00000040,
    BDA_SCAN_MOD_160QAM   = 0x00000080,
    BDA_SCAN_MOD_192QAM   = 0x00000100,
    BDA_SCAN_MOD_224QAM   = 0x00000200,
    BDA_SCAN_MOD_256QAM   = 0x00000400,
    BDA_SCAN_MOD_320QAM   = 0x00000800,
    BDA_SCAN_MOD_384QAM   = 0x00001000,
    BDA_SCAN_MOD_448QAM   = 0x00002000,
    BDA_SCAN_MOD_512QAM   = 0x00004000,
    BDA_SCAN_MOD_640QAM   = 0x00008000,
    BDA_SCAN_MOD_768QAM   = 0x00010000,
    BDA_SCAN_MOD_896QAM   = 0x00020000,
    BDA_SCAN_MOD_1024QAM  = 0x00040000,
    BDA_SCAN_MOD_QPSK     = 0x00080000,
    BDA_SCAN_MOD_BPSK     = 0x00100000,
    BDA_SCAN_MOD_OQPSK    = 0x00200000,
    BDA_SCAN_MOD_8VSB     = 0x00400000,
    BDA_SCAN_MOD_16VSB    = 0x00800000,
    BDA_SCAN_MOD_AM_RADIO = 0x01000000,
    BDA_SCAN_MOD_FM_RADIO = 0x02000000,
    BDA_SCAN_MOD_8PSK     = 0x04000000,
    BDA_SCAN_MOD_RF       = 0x08000000, // analog TV
    ScanModulationTypesMask_MCE_DigitalCable = BDA_MOD_64QAM |
                            BDA_MOD_256QAM,
    ScanModulationTypesMask_MCE_TerrestrialATSC = BDA_MOD_8VSB,
    ScanModulationTypesMask_MCE_AnalogTv = BDA_MOD_RF,
    ScanModulationTypesMask_MCE_All_TV = 0xffffffff,
    ScanModulationTypesMask_DVBC = BDA_MOD_64QAM | BDA_SCAN_MOD_128QAM |
                            BDA_MOD_256QAM,
    BDA_SCAN_MOD_16APSK     = 0x10000000,
    BDA_SCAN_MOD_32APSK     = 0x20000000,
} ScanModulationTypes;

ENUM SpectralInversion {
    BDA_SPECTRAL_INVERSION_NOT_SET = -1,
    BDA_SPECTRAL_INVERSION_NOT_DEFINED = 0,
    BDA_SPECTRAL_INVERSION_AUTOMATIC = 1,
    BDA_SPECTRAL_INVERSION_NORMAL,
    BDA_SPECTRAL_INVERSION_INVERTED,
    BDA_SPECTRAL_INVERSION_MAX
} SpectralInversion;

ENUM Polarisation {
    BDA_POLARISATION_NOT_SET = -1,
    BDA_POLARISATION_NOT_DEFINED = 0,
    BDA_POLARISATION_LINEAR_H = 1, // Linear horizontal polarisation
    BDA_POLARISATION_LINEAR_V, // Linear vertical polarisation
    BDA_POLARISATION_CIRCULAR_L, // Circular left polarisation
    BDA_POLARISATION_CIRCULAR_R, // Circular right polarisation
    BDA_POLARISATION_MAX,
} Polarisation;

ENUM LNB_Source {
    BDA_LNB_SOURCE_NOT_SET = -1,
    BDA_LNB_SOURCE_NOT_DEFINED = 0,
    BDA_LNB_SOURCE_A = 1, // 
    BDA_LNB_SOURCE_B = 2, // 
    BDA_LNB_SOURCE_C = 3, // 
    BDA_LNB_SOURCE_D = 4, // 
    BDA_LNB_SOURCE_MAX,
} LNB_Source;


ENUM GuardInterval {
    BDA_GUARD_NOT_SET = -1,
    BDA_GUARD_NOT_DEFINED = 0,
    BDA_GUARD_1_32 = 1, // Guard interval is 1/32
    BDA_GUARD_1_16, // Guard interval is 1/16
    BDA_GUARD_1_8, // Guard interval is 1/8
    BDA_GUARD_1_4, // Guard interval is 1/4
    BDA_GUARD_1_128, // Guard interval is 1/128 (DVB-T2)
    BDA_GUARD_19_128, // Guard interval is 19/128 (DVB-T2)
    BDA_GUARD_19_256, // Guard interval is 19/256 (DVB-T2)
    BDA_GUARD_MAX,
} GuardInterval;

ENUM HierarchyAlpha {
    BDA_HALPHA_NOT_SET = -1,
    BDA_HALPHA_NOT_DEFINED = 0,
    BDA_HALPHA_1 = 1, // Hierarchy alpha is 1.
    BDA_HALPHA_2, // Hierarchy alpha is 2.
    BDA_HALPHA_4, // Hierarchy alpha is 4.
    BDA_HALPHA_MAX,
} HierarchyAlpha;

ENUM TransmissionMode {
    BDA_XMIT_MODE_NOT_SET = -1,
    BDA_XMIT_MODE_NOT_DEFINED = 0,
    BDA_XMIT_MODE_2K = 1, // Transmission uses 1705 carriers (use a 2K FFT)
    BDA_XMIT_MODE_8K,     // Transmission uses 6817 carriers (use an 8K FFT)
    BDA_XMIT_MODE_4K,
    BDA_XMIT_MODE_2K_INTERLEAVED,
    BDA_XMIT_MODE_4K_INTERLEAVED,
    BDA_XMIT_MODE_1K,    //DVB-T2 (use 1K FFT)
    BDA_XMIT_MODE_16K,   //DVB-T2 (use 16K FFT)
    BDA_XMIT_MODE_32K,   //DVB-T2 (use 32K FFT)
    BDA_XMIT_MODE_MAX,
} TransmissionMode;

ENUM RollOff {
    BDA_ROLL_OFF_NOT_SET = -1,
    BDA_ROLL_OFF_NOT_DEFINED = 0,
    BDA_ROLL_OFF_20 = 1,         // .20 Roll Off (DVB-S2 Only)
    BDA_ROLL_OFF_25,             // .25 Roll Off (DVB-S2 Only)
    BDA_ROLL_OFF_35,             // .35 Roll Off (DVB-S2 Only)
    BDA_ROLL_OFF_MAX,
} RollOff;

ENUM Pilot {
    BDA_PILOT_NOT_SET = -1,
    BDA_PILOT_NOT_DEFINED = 0,
    BDA_PILOT_OFF = 1,           // Pilot Off (DVB-S2 Only)
    BDA_PILOT_ON,                // Pilot On  (DVB-S2 Only)
    BDA_PILOT_MAX,
} Pilot;

typedef struct _BDA_SIGNAL_TIMEOUTS
{
    ULONG      ulCarrierTimeoutMs;
    ULONG      ulScanningTimeoutMs;
    ULONG      ulTuningTimeoutMs;
} BDA_SIGNAL_TIMEOUTS, *PBDA_SIGNAL_TIMEOUTS;

//  Settings for Tuner Frequency
//
ENUM BDA_Frequency {
    BDA_FREQUENCY_NOT_SET = -1,
    BDA_FREQUENCY_NOT_DEFINED = 0
} BDA_Frequency;

//  Settings for Tuner Range
//
//  Tuner range refers to the setting of LNB High/Low as well as the
//  selection of a satellite on a multiple satellite switch.
//
ENUM BDA_Range {
    BDA_RANGE_NOT_SET = -1,
    BDA_RANGE_NOT_DEFINED = 0
} BDA_Range;

//  Settings for Tuner Channel Bandwidth
//
ENUM BDA_Channel_Bandwidth {
    BDA_CHAN_BANDWITH_NOT_SET      = -1,
    BDA_CHAN_BANDWITH_NOT_DEFINED  = 0
} BDA_Channel_Bandwidth;

//  Settings for Tuner Frequency Multiplier
//
ENUM BDA_Frequency_Multiplier {
    BDA_FREQUENCY_MULTIPLIER_NOT_SET       = -1,
    BDA_FREQUENCY_MULTIPLIER_NOT_DEFINED   = 0
} BDA_Frequency_Multiplier;

FLAGS BDA_Comp_Flags {
    // equiv comparison rule overrides, default behavior is type specific
    BDACOMP_NOT_DEFINED              = 0x00000000,
    BDACOMP_EXCLUDE_TS_FROM_TR       = 0x00000001,  // never put TS in TR equiv comparison
    BDACOMP_INCLUDE_LOCATOR_IN_TR    = 0x00000002,  // always include loc in TR equiv comparison
    BDACOMP_INCLUDE_COMPONENTS_IN_TR = 0x00000004, // always include components in TR equiv comparison
} BDA_Comp_Flags;

ENUM ApplicationTypeType
{
    SCTE28_ConditionalAccess = 0,
    SCTE28_POD_Host_Binding_Information,
    SCTE28_IPService,
    SCTE28_NetworkInterface_SCTE55_2,
    SCTE28_NetworkInterface_SCTE55_1,
    SCTE28_CopyProtection,
    SCTE28_Diagnostic,
    SCTE28_Undesignated,
    SCTE28_Reserved,
}ApplicationTypeType;


ENUM BDA_CONDITIONALACCESS_REQUESTTYPE {
    CONDITIONALACCESS_ACCESS_UNSPECIFIED = 0,
    CONDITIONALACCESS_ACCESS_NOT_POSSIBLE,
    CONDITIONALACCESS_ACCESS_POSSIBLE,
    CONDITIONALACCESS_ACCESS_POSSIBLE_NO_STREAMING_DISRUPTION
} BDA_CONDITIONALACCESS_REQUESTTYPE;

ENUM BDA_CONDITIONALACCESS_MMICLOSEREASON {
    CONDITIONALACCESS_UNSPECIFIED = 0,
    CONDITIONALACCESS_CLOSED_ITSELF,
    CONDITIONALACCESS_TUNER_REQUESTED_CLOSE,
    CONDITIONALACCESS_DIALOG_TIMEOUT,
    CONDITIONALACCESS_DIALOG_FOCUS_CHANGE,
    CONDITIONALACCESS_DIALOG_USER_DISMISSED,
    CONDITIONALACCESS_DIALOG_USER_NOT_AVAILABLE
} BDA_CONDITIONALACCESS_MMICLOSEREASON;

ENUM BDA_CONDITIONALACCESS_SESSION_RESULT {
    CONDITIONALACCESS_SUCCESSFULL = 0,
    CONDITIONALACCESS_ENDED_NOCHANGE,
    CONDITIONALACCESS_ABORTED
} BDA_CONDITIONALACCESS_SESSION_RESULT;

ENUM BDA_DISCOVERY_STATE {
    BDA_DISCOVERY_UNSPECIFIED = 0,
    BDA_DISCOVERY_REQUIRED,
    BDA_DISCOVERY_COMPLETE
} BDA_DISCOVERY_STATE;

// Digital Demodulator for DVBT2 Physical Layer Pipe
#define BDA_PLP_ID_NOT_SET -1       

#include <unexposeenums2managed.h>


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // not defined _BDATYPES_

// end of file -- bdatypes.h

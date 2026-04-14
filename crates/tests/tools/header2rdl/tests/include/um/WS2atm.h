/*++

Copyright (c) 1997-1999  Microsoft Corporation

Module Name:

	ws2atm.h

Abstract:

	Winsock 2 ATM Annex definitions.

Revision History:

Notes:

--*/

#ifndef _WS2ATM_H_
#define _WS2ATM_H_

#if _MSC_VER > 1000
#pragma once
#endif

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#include <pshpack4.h>

#define ATMPROTO_AALUSER           0x00   /* User-defined AAL */
#define ATMPROTO_AAL1              0x01   /* AAL 1 */
#define ATMPROTO_AAL2              0x02   /* AAL 2 */
#define ATMPROTO_AAL34             0x03   /* AAL 3/4 */
#define ATMPROTO_AAL5              0x05   /* AAL 5 */

#define SAP_FIELD_ABSENT           0xFFFFFFFE
#define SAP_FIELD_ANY              0xFFFFFFFF
#define SAP_FIELD_ANY_AESA_SEL     0xFFFFFFFA
#define SAP_FIELD_ANY_AESA_REST    0xFFFFFFFB

/*
 *  values used for AddressType in struct ATM_ADDRESS
 */
#define ATM_E164               0x01   /* E.164 addressing scheme */
#define ATM_NSAP               0x02   /* NSAP-style ATM Endsystem Address scheme */
#define ATM_AESA               0x02   /* NSAP-style ATM Endsystem Address scheme */

#define ATM_ADDR_SIZE          20
typedef struct {
    DWORD AddressType;                /* E.164 or NSAP-style ATM Endsystem Address */
    DWORD NumofDigits;                /* number of digits; */
    UCHAR Addr[ATM_ADDR_SIZE];        /* IA5 digits for E164, BCD encoding for NSAP */
                                      /* format as defined in the ATM Forum UNI 3.1 */
} ATM_ADDRESS;

/*
 *  values used for Layer2Protocol in B-LLI
 */
#define BLLI_L2_ISO_1745           0x01   /* Basic mode ISO 1745                      */
#define BLLI_L2_Q921               0x02   /* CCITT Rec. Q.921                         */
#define BLLI_L2_X25L               0x06   /* CCITT Rec. X.25, link layer              */
#define BLLI_L2_X25M               0x07   /* CCITT Rec. X.25, multilink               */
#define BLLI_L2_ELAPB              0x08   /* Extended LAPB; for half duplex operation */
#define BLLI_L2_HDLC_ARM           0x09   /* HDLC ARM (ISO 4335)                      */
#define BLLI_L2_HDLC_NRM           0x0A   /* HDLC NRM (ISO 4335)                      */
#define BLLI_L2_HDLC_ABM           0x0B   /* HDLC ABM (ISO 4335)                      */
#define BLLI_L2_LLC                0x0C   /* LAN logical link control (ISO 8802/2)    */
#define BLLI_L2_X75                0x0D   /* CCITT Rec. X.75, single link procedure   */
#define BLLI_L2_Q922               0x0E   /* CCITT Rec. Q.922                         */
#define BLLI_L2_USER_SPECIFIED     0x10   /* User Specified                           */
#define BLLI_L2_ISO_7776           0x11   /* ISO 7776 DTE-DTE operation               */

/*
 *  values used for Layer3Protocol in B-LLI
 */
#define BLLI_L3_X25                0x06   /* CCITT Rec. X.25, packet layer            */
#define BLLI_L3_ISO_8208           0x07   /* ISO/IEC 8208 (X.25 packet layer for DTE  */
#define BLLI_L3_X223               0x08   /* X.223/ISO 8878                           */
#define BLLI_L3_SIO_8473           0x09   /* ISO/IEC 8473 (OSI connectionless)        */
#define BLLI_L3_T70                0x0A   /* CCITT Rec. T.70 min. network layer       */
#define BLLI_L3_ISO_TR9577         0x0B   /* ISO/IEC TR 9577 Network Layer Protocol ID*/
#define BLLI_L3_USER_SPECIFIED     0x10   /* User Specified                           */

/*
 *  values used for Layer3IPI in B-LLI
 */
#define BLLI_L3_IPI_SNAP           0x80   /* IEEE 802.1 SNAP identifier               */
#define BLLI_L3_IPI_IP             0xCC   /* Internet Protocol (IP) identifier        */

typedef struct {
    DWORD Layer2Protocol;                 /* User information layer 2 protocol           */
    DWORD Layer2UserSpecifiedProtocol;    /* User specified layer 2 protocol information */
    DWORD Layer3Protocol;                 /* User information layer 3 protocol           */
    DWORD Layer3UserSpecifiedProtocol;    /* User specified layer 3 protocol information */
    DWORD Layer3IPI;                      /* ISO/IEC TR 9577 Initial Protocol Identifier */
    UCHAR SnapID[5];                      /* SNAP ID consisting of OUI and PID           */
} ATM_BLLI;

/*
 *  values used for the HighLayerInfoType field in ATM_BHLI
 */
#define BHLI_ISO                   0x00   /* ISO                                 */
#define BHLI_UserSpecific          0x01   /* User Specific                       */
#define BHLI_HighLayerProfile      0x02   /* High layer profile (only in UNI3.0) */
#define BHLI_VendorSpecificAppId   0x03   /* Vendor-Specific Application ID      */

typedef struct {
    DWORD HighLayerInfoType;          /* High Layer Information Type      */
    DWORD HighLayerInfoLength;        /* number of bytes in HighLayerInfo */
    UCHAR HighLayerInfo[8];           /* the value dependent on the       */
                                      /*   HighLayerInfoType field        */
} ATM_BHLI;

typedef struct sockaddr_atm {
    u_short satm_family;              /* address family should be AF_ATM  */
    ATM_ADDRESS satm_number;          /* ATM address                      */
    ATM_BLLI satm_blli;               /* B-LLI                            */
    ATM_BHLI satm_bhli;               /* B-HLI                            */
} sockaddr_atm, SOCKADDR_ATM, *PSOCKADDR_ATM, *LPSOCKADDR_ATM;

typedef enum {
    IE_AALParameters,
    IE_TrafficDescriptor,
    IE_BroadbandBearerCapability,
    IE_BHLI,
    IE_BLLI,
    IE_CalledPartyNumber,
    IE_CalledPartySubaddress,
    IE_CallingPartyNumber,
    IE_CallingPartySubaddress,
    IE_Cause,
    IE_QOSClass,
    IE_TransitNetworkSelection,
} Q2931_IE_TYPE;

typedef struct {
    Q2931_IE_TYPE IEType;
    ULONG         IELength;
    UCHAR         IE[1];
} Q2931_IE;

/*
 *  manifest constants for the AALType field in struct AAL_PARAMETERS_IE
 */
typedef enum {
    AALTYPE_5     = 5,   /* AAL 5 */
    AALTYPE_USER  = 16,  /* user-defined AAL */
} AAL_TYPE;

/*
 *  values used for the Mode field in struct AAL5_PARAMETERS
 */
#define AAL5_MODE_MESSAGE           0x01
#define AAL5_MODE_STREAMING         0x02

/*
 *  values used for the SSCSType field in struct AAL5_PARAMETERS
 */
#define AAL5_SSCS_NULL              0x00
#define AAL5_SSCS_SSCOP_ASSURED     0x01
#define AAL5_SSCS_SSCOP_NON_ASSURED 0x02
#define AAL5_SSCS_FRAME_RELAY       0x04

typedef struct {
    ULONG ForwardMaxCPCSSDUSize;
    ULONG BackwardMaxCPCSSDUSize;
    UCHAR Mode;                        /* only available in UNI 3.0 */
    UCHAR SSCSType;
} AAL5_PARAMETERS;

typedef struct {
    ULONG UserDefined;
} AALUSER_PARAMETERS;

typedef struct {
    AAL_TYPE AALType;
    union {
        AAL5_PARAMETERS     AAL5Parameters;
        AALUSER_PARAMETERS  AALUserParameters;
    } AALSpecificParameters;
} AAL_PARAMETERS_IE;

typedef struct {
    ULONG PeakCellRate_CLP0;
    ULONG PeakCellRate_CLP01;
    ULONG SustainableCellRate_CLP0;
    ULONG SustainableCellRate_CLP01;
    ULONG MaxBurstSize_CLP0;
    ULONG MaxBurstSize_CLP01;
    BOOL  Tagging;
} ATM_TD;

typedef struct {
    ATM_TD Forward;
    ATM_TD Backward;
    BOOL   BestEffort;
} ATM_TRAFFIC_DESCRIPTOR_IE;

/*
 *  values used for the BearerClass field in struct ATM_BROADBAND_BEARER_CAPABILITY_IE
 */
#define BCOB_A                   0x01   /* Bearer class A                      */
#define BCOB_C                   0x03   /* Bearer class C                      */
#define BCOB_X                   0x10   /* Bearer class X                      */

/*
 *  values used for the TrafficType field in struct ATM_BROADBAND_BEARER_CAPABILITY_IE
 */
#define TT_NOIND                 0x00   /* No indication of traffic type       */
#define TT_CBR                   0x04   /* Constant bit rate                   */
#define TT_VBR                   0x08   /* Variable bit rate                   */

/*
 *  values used for the TimingRequirements field in struct ATM_BROADBAND_BEARER_CAPABILITY_IE
 */
#define TR_NOIND                 0x00   /* No timing requirement indication    */
#define TR_END_TO_END            0x01   /* End-to-end timing required          */
#define TR_NO_END_TO_END         0x02   /* End-to-end timing not required      */

/*
 *  values used for the ClippingSusceptability field in struct ATM_BROADBAND_BEARER_CAPABILITY_IE
 */
#define CLIP_NOT                 0x00   /* Not susceptible to clipping         */
#define CLIP_SUS                 0x20   /* Susceptible to clipping             */

/*
 *  values used for the UserPlaneConnectionConfig field in
 *  struct ATM_BROADBAND_BEARER_CAPABILITY_IE
 */
#define UP_P2P                   0x00   /* Point-to-point connection           */
#define UP_P2MP                  0x01   /* Point-to-multipoint connection      */

typedef struct {
    UCHAR BearerClass;
    UCHAR TrafficType;
    UCHAR TimingRequirements;
    UCHAR ClippingSusceptability;
    UCHAR UserPlaneConnectionConfig;
} ATM_BROADBAND_BEARER_CAPABILITY_IE;

typedef ATM_BHLI ATM_BHLI_IE;

/*
 *  values used for the Layer2Mode field in struct ATM_BLLI_IE
 */
#define BLLI_L2_MODE_NORMAL         0x40
#define BLLI_L2_MODE_EXT            0x80

/*
 *  values used for the Layer3Mode field in struct ATM_BLLI_IE
 */
#define BLLI_L3_MODE_NORMAL         0x40
#define BLLI_L3_MODE_EXT            0x80

/*
 *  values used for the Layer3DefaultPacketSize field in struct ATM_BLLI_IE
 */
#define BLLI_L3_PACKET_16           0x04
#define BLLI_L3_PACKET_32           0x05
#define BLLI_L3_PACKET_64           0x06
#define BLLI_L3_PACKET_128          0x07
#define BLLI_L3_PACKET_256          0x08
#define BLLI_L3_PACKET_512          0x09
#define BLLI_L3_PACKET_1024         0x0A
#define BLLI_L3_PACKET_2048         0x0B
#define BLLI_L3_PACKET_4096         0x0C

typedef struct {
    DWORD Layer2Protocol;                 /* User information layer 2 protocol           */
    UCHAR Layer2Mode;
    UCHAR Layer2WindowSize;
    DWORD Layer2UserSpecifiedProtocol;    /* User specified layer 2 protocol information */
    DWORD Layer3Protocol;                 /* User information layer 3 protocol           */
    UCHAR Layer3Mode;
    UCHAR Layer3DefaultPacketSize;
    UCHAR Layer3PacketWindowSize;
    DWORD Layer3UserSpecifiedProtocol;    /* User specified layer 3 protocol information */
    DWORD Layer3IPI;                      /* ISO/IEC TR 9577 Initial Protocol Identifier */
    UCHAR SnapID[5];                      /* SNAP ID consisting of OUI and PID           */
} ATM_BLLI_IE;

typedef ATM_ADDRESS ATM_CALLED_PARTY_NUMBER_IE;

typedef ATM_ADDRESS ATM_CALLED_PARTY_SUBADDRESS_IE;

/*
 *  values used for the Presentation_Indication field in
 *  struct ATM_CALLING_PARTY_NUMBER_IE
 */
#define PI_ALLOWED                  0x00
#define PI_RESTRICTED               0x40
#define PI_NUMBER_NOT_AVAILABLE     0x80

/*
 *  values used for the Screening_Indicator field in
 *  struct ATM_CALLING_PARTY_NUMBER_IE
 */
#define SI_USER_NOT_SCREENED        0x00
#define SI_USER_PASSED              0x01
#define SI_USER_FAILED              0x02
#define SI_NETWORK                  0x03

typedef struct {
    ATM_ADDRESS ATM_Number;
    UCHAR       Presentation_Indication;
    UCHAR       Screening_Indicator;
} ATM_CALLING_PARTY_NUMBER_IE;

typedef ATM_ADDRESS ATM_CALLING_PARTY_SUBADDRESS_IE;

/*
 *  values used for the Location field in struct ATM_CAUSE_IE
 */
#define CAUSE_LOC_USER                      0x00
#define CAUSE_LOC_PRIVATE_LOCAL             0x01
#define CAUSE_LOC_PUBLIC_LOCAL              0x02
#define CAUSE_LOC_TRANSIT_NETWORK           0x03
#define CAUSE_LOC_PUBLIC_REMOTE             0x04
#define CAUSE_LOC_PRIVATE_REMOTE            0x05
#define CAUSE_LOC_INTERNATIONAL_NETWORK     0x07
#define CAUSE_LOC_BEYOND_INTERWORKING       0x0A

/*
 *  values used for the Cause field in struct ATM_CAUSE_IE
 */
#define CAUSE_UNALLOCATED_NUMBER                0x01
#define CAUSE_NO_ROUTE_TO_TRANSIT_NETWORK       0x02
#define CAUSE_NO_ROUTE_TO_DESTINATION           0x03
#define CAUSE_VPI_VCI_UNACCEPTABLE              0x0A
#define CAUSE_NORMAL_CALL_CLEARING              0x10
#define CAUSE_USER_BUSY                         0x11
#define CAUSE_NO_USER_RESPONDING                0x12
#define CAUSE_CALL_REJECTED                     0x15
#define CAUSE_NUMBER_CHANGED                    0x16
#define CAUSE_USER_REJECTS_CLIR                 0x17
#define CAUSE_DESTINATION_OUT_OF_ORDER          0x1B
#define CAUSE_INVALID_NUMBER_FORMAT             0x1C
#define CAUSE_STATUS_ENQUIRY_RESPONSE           0x1E
#define CAUSE_NORMAL_UNSPECIFIED                0x1F
#define CAUSE_VPI_VCI_UNAVAILABLE               0x23
#define CAUSE_NETWORK_OUT_OF_ORDER              0x26
#define CAUSE_TEMPORARY_FAILURE                 0x29
#define CAUSE_ACCESS_INFORMAION_DISCARDED       0x2B
#define CAUSE_NO_VPI_VCI_AVAILABLE              0x2D
#define CAUSE_RESOURCE_UNAVAILABLE              0x2F
#define CAUSE_QOS_UNAVAILABLE                   0x31
#define CAUSE_USER_CELL_RATE_UNAVAILABLE        0x33
#define CAUSE_BEARER_CAPABILITY_UNAUTHORIZED    0x39
#define CAUSE_BEARER_CAPABILITY_UNAVAILABLE     0x3A
#define CAUSE_OPTION_UNAVAILABLE                0x3F
#define CAUSE_BEARER_CAPABILITY_UNIMPLEMENTED   0x41
#define CAUSE_UNSUPPORTED_TRAFFIC_PARAMETERS    0x49
#define CAUSE_INVALID_CALL_REFERENCE            0x51
#define CAUSE_CHANNEL_NONEXISTENT               0x52
#define CAUSE_INCOMPATIBLE_DESTINATION          0x58
#define CAUSE_INVALID_ENDPOINT_REFERENCE        0x59
#define CAUSE_INVALID_TRANSIT_NETWORK_SELECTION 0x5B
#define CAUSE_TOO_MANY_PENDING_ADD_PARTY        0x5C
#define CAUSE_AAL_PARAMETERS_UNSUPPORTED        0x5D
#define CAUSE_MANDATORY_IE_MISSING              0x60
#define CAUSE_UNIMPLEMENTED_MESSAGE_TYPE        0x61
#define CAUSE_UNIMPLEMENTED_IE                  0x63
#define CAUSE_INVALID_IE_CONTENTS               0x64
#define CAUSE_INVALID_STATE_FOR_MESSAGE         0x65
#define CAUSE_RECOVERY_ON_TIMEOUT               0x66
#define CAUSE_INCORRECT_MESSAGE_LENGTH          0x68
#define CAUSE_PROTOCOL_ERROR                    0x6F

/*
 *  values used for the Condition portion of the Diagnostics field
 *  in struct ATM_CAUSE_IE, for certain Cause values
 */
#define CAUSE_COND_UNKNOWN                  0x00
#define CAUSE_COND_PERMANENT                0x01
#define CAUSE_COND_TRANSIENT                0x02

/*
 *  values used for the Rejection Reason portion of the Diagnostics field
 *  in struct ATM_CAUSE_IE, for certain Cause values
 */
#define CAUSE_REASON_USER                   0x00
#define CAUSE_REASON_IE_MISSING             0x04
#define CAUSE_REASON_IE_INSUFFICIENT        0x08

/*
 *  values used for the P-U flag of the Diagnostics field
 *  in struct ATM_CAUSE_IE, for certain Cause values
 */
#define CAUSE_PU_PROVIDER                   0x00
#define CAUSE_PU_USER                       0x08

/*
 *  values used for the N-A flag of the Diagnostics field
 *  in struct ATM_CAUSE_IE, for certain Cause values
 */
#define CAUSE_NA_NORMAL                     0x00
#define CAUSE_NA_ABNORMAL                   0x04

typedef struct {
    UCHAR Location;
    UCHAR Cause;
    UCHAR DiagnosticsLength;
    UCHAR Diagnostics[4];
} ATM_CAUSE_IE;

/*
 *  values used for the QOSClassForward and QOSClassBackward
 *  field in struct ATM_QOS_CLASS_IE
 */
#define QOS_CLASS0                  0x00
#define QOS_CLASS1                  0x01
#define QOS_CLASS2                  0x02
#define QOS_CLASS3                  0x03
#define QOS_CLASS4                  0x04

typedef struct {
    UCHAR QOSClassForward;
    UCHAR QOSClassBackward;
} ATM_QOS_CLASS_IE;

/*
 *  values used for the TypeOfNetworkId field in struct ATM_TRANSIT_NETWORK_SELECTION_IE
 */
#define TNS_TYPE_NATIONAL           0x40

/*
 *  values used for the NetworkIdPlan field in struct ATM_TRANSIT_NETWORK_SELECTION_IE
 */
#define TNS_PLAN_CARRIER_ID_CODE    0x01

typedef struct {
    UCHAR TypeOfNetworkId;
    UCHAR NetworkIdPlan;
    UCHAR NetworkIdLength;
    UCHAR NetworkId[1];
} ATM_TRANSIT_NETWORK_SELECTION_IE;

/*
 *  ATM specific Ioctl codes
 */
#define SIO_GET_NUMBER_OF_ATM_DEVICES   0x50160001
#define SIO_GET_ATM_ADDRESS             0xd0160002
#define SIO_ASSOCIATE_PVC               0x90160003
#define SIO_GET_ATM_CONNECTION_ID       0x50160004

/* ATM Connection Identifier */

typedef struct {
    DWORD  DeviceNumber;
    DWORD  VPI;
    DWORD  VCI;
} ATM_CONNECTION_ID;

/*
 * Input buffer format for SIO_ASSOCIATE_PVC
 */

typedef struct {
   ATM_CONNECTION_ID   PvcConnectionId;
   QOS                 PvcQos;
} ATM_PVC_PARAMS;

#include <poppack.h>

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif   /* _WS2ATM_H_ */

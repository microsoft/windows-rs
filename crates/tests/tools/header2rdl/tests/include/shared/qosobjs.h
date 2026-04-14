/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    qosobjs.h

Abstract:

    This module contains QoS object definitions.

--*/

#pragma once
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#ifdef __cplusplus
extern  "C" {
#endif


#define QOS_MAX_OBJECT_STRING_LENGTH    256


//
// QoS objects supported by traffic control
//
#define QOS_TRAFFIC_GENERAL_ID_BASE 4000

#define QOS_OBJECT_DS_CLASS                    (0x00000001 + QOS_TRAFFIC_GENERAL_ID_BASE)
        /* QOS_DS_CLASS structure passed */
#define QOS_OBJECT_TRAFFIC_CLASS               (0x00000002 + QOS_TRAFFIC_GENERAL_ID_BASE) 
          /* QOS_Traffic class structure passed */
#define   QOS_OBJECT_DIFFSERV                  (0x00000003 + QOS_TRAFFIC_GENERAL_ID_BASE)
          /* QOS_DIFFSERV Structure */
#define QOS_OBJECT_TCP_TRAFFIC                 (0x00000004 + QOS_TRAFFIC_GENERAL_ID_BASE)
        /* QOS_TCP_TRAFFIC structure */
#define QOS_OBJECT_FRIENDLY_NAME               (0x00000005 + QOS_TRAFFIC_GENERAL_ID_BASE)
        /* QOS_FRIENDLY_NAME structure */


//
// This structure is used to associate a friendly name with the flow
// 

typedef struct _QOS_FRIENDLY_NAME {
    QOS_OBJECT_HDR ObjectHdr;
    WCHAR          FriendlyName[QOS_MAX_OBJECT_STRING_LENGTH];
} QOS_FRIENDLY_NAME, *LPQOS_FRIENDLY_NAME;

//
// This structure may carry an 802.1 TrafficClass parameter which 
// has been provided to the host by a layer 2 network, for example, 
// in an 802.1 extended RSVP RESV message. If this object is obtained
// from the network, hosts will stamp the MAC headers of corresponding
// transmitted packets, with the value in the object. Otherwise, hosts
// may select a value based on the standard Intserv mapping of 
// ServiceType to 802.1 TrafficClass.
//
//

typedef struct _QOS_TRAFFIC_CLASS {

    QOS_OBJECT_HDR   ObjectHdr;
    ULONG            TrafficClass;

} QOS_TRAFFIC_CLASS, *LPQOS_TRAFFIC_CLASS;

//
// This structure may carry an DSField parameter which  has been provided to 
// the host by a layer 3 network, for example, in an extended RSVP RESV message. 
// If this object is obtained from the network, hosts will stamp the DS Field on the
// IP header of transmitted packets, with the value in the object. Otherwise, hosts
// may select a value based on the standard Intserv mapping of ServiceType to DS Field 
//

typedef struct _QOS_DS_CLASS {

    QOS_OBJECT_HDR ObjectHdr;
    ULONG          DSField;

} QOS_DS_CLASS, *LPQOS_DS_CLASS;


//
// This structure is used to create DiffServ Flows. This creates flows in the packet scheduler
// and allows it to classify to packets based on a particular DS field. This structure takes
// a variable length array of QOS_DIFFSERV_RULE, where each DS field is specified by a 
// QOS_DIFFSERV_RULE
//
typedef struct _QOS_DIFFSERV {

    QOS_OBJECT_HDR ObjectHdr;
    ULONG          DSFieldCount;
    UCHAR          DiffservRule[1];
} QOS_DIFFSERV, *LPQOS_DIFFSERV;

//
// The rule for a Diffserv DS codepoint. 
//
typedef struct _QOS_DIFFSERV_RULE {
    UCHAR InboundDSField;
    UCHAR ConformingOutboundDSField;
    UCHAR NonConformingOutboundDSField;
    UCHAR ConformingUserPriority;
    UCHAR NonConformingUserPriority;
} QOS_DIFFSERV_RULE, *LPQOS_DIFFSERV_RULE;

// 
// This structure is passed to indicate that the IP Precedence and UserPriority mappings for the flow
// have to be set to the system defaults for TCP traffic. If this object is passed, 
// the ServiceType ==> DSField mapping, ServiceType ==> UserPriorityMapping, QOS_OBJECT_DS_CLASS
// and QOS_OBJECT_TRAFFIC_CLASS will be ignored.
//

typedef struct _QOS_TCP_TRAFFIC {
    QOS_OBJECT_HDR ObjectHdr;
} QOS_TCP_TRAFFIC, *LPQOS_TCP_TRAFFIC;
    

#ifdef __cplusplus
}
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


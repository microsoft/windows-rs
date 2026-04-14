/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    qos.h - QoS definitions for NDIS components.

Abstract:

    This module defines the Quality of Service structures and types used
    by Winsock applications.

WARNING:

    This api is deprecated and will be removed in a future release of Windows.
    Please use the QOS2.h api.

Revision History:

--*/

#ifndef __QOS_H_
#define __QOS_H_

#pragma once
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


/*
 *  Definitions for valued-based Service Type for each direction of data flow.
 */

typedef ULONG   SERVICETYPE;

#define SERVICETYPE_NOTRAFFIC               0x00000000  /* No data in this 
                                                         * direction */
#define SERVICETYPE_BESTEFFORT              0x00000001  /* Best Effort */
#define SERVICETYPE_CONTROLLEDLOAD          0x00000002  /* Controlled Load */
#define SERVICETYPE_GUARANTEED              0x00000003  /* Guaranteed */

#define SERVICETYPE_NETWORK_UNAVAILABLE     0x00000004  /* Used to notify 
                                                         * change to user */
#define SERVICETYPE_GENERAL_INFORMATION     0x00000005  /* corresponds to 
                                                         * "General Parameters"
                                                         * defined by IntServ */
#define SERVICETYPE_NOCHANGE                0x00000006  /* used to indicate
                                                         * that the flow spec
                                                         * contains no change
                                                         * from any previous
                                                         * one */
#define SERVICETYPE_NONCONFORMING           0x00000009  /* Non-Conforming Traffic */
#define SERVICETYPE_NETWORK_CONTROL         0x0000000A  /* Network Control traffic */
#define SERVICETYPE_QUALITATIVE             0x0000000D  /* Qualitative applications */ 



/*********  The usage of these is currently not supported.  ***************/
#define SERVICE_BESTEFFORT                  0x80010000
#define SERVICE_CONTROLLEDLOAD              0x80020000
#define SERVICE_GUARANTEED                  0x80040000
#define SERVICE_QUALITATIVE                 0x80200000
/* **************************** ***** ************************************ */



/*
 * Flags to control the usage of RSVP on this flow.
 */

/*
 * to turn off traffic control, 'OR' ( | ) this flag with the 
 * ServiceType field in the FLOWSPEC
 */
#define SERVICE_NO_TRAFFIC_CONTROL   0x81000000


/*
 * this flag can be used to prevent any rsvp signaling messages from being 
 * sent. Local traffic control will be invoked, but no RSVP Path messages 
 * will be sent.This flag can also be used in conjunction with a receiving 
 * flowspec to suppress the automatic generation of a Reserve message.  
 * The application would receive notification that a Path  message had arrived 
 * and would then need to alter the QOS by issuing WSAIoctl( SIO_SET_QOS ), 
 * to unset this flag and thereby causing Reserve messages to go out.
 */

#define SERVICE_NO_QOS_SIGNALING   0x40000000




/*
 *  Flow Specifications for each direction of data flow.
 */
typedef struct _flowspec
{
    ULONG       TokenRate;              /* In Bytes/sec */
    ULONG       TokenBucketSize;        /* In Bytes */
    ULONG       PeakBandwidth;          /* In Bytes/sec */
    ULONG       Latency;                /* In microseconds */
    ULONG       DelayVariation;         /* In microseconds */
    SERVICETYPE ServiceType;
    ULONG       MaxSduSize;             /* In Bytes */
    ULONG       MinimumPolicedSize;     /* In Bytes */

} FLOWSPEC, *PFLOWSPEC, * LPFLOWSPEC;

/*
 * this value can be used in the FLOWSPEC structure to instruct the Rsvp Service 
 * provider to derive the appropriate default value for the parameter.  Note 
 * that not all values in the FLOWSPEC structure can be defaults. In the
 * ReceivingFlowspec, all parameters can be defaulted except the ServiceType.  
 * In the SendingFlowspec, the MaxSduSize and MinimumPolicedSize can be
 * defaulted. Other defaults may be possible. Refer to the appropriate
 * documentation.
 */
#define QOS_NOT_SPECIFIED     0xFFFFFFFF

/*
 * define a value that can be used for the PeakBandwidth, which will map into 
 * positive infinity when the FLOWSPEC is converted into IntServ floating point 
 * format.  We can't use (-1) because that value was previously defined to mean
 * "select the default".
 */
#define   POSITIVE_INFINITY_RATE     0xFFFFFFFE



/*
 * the provider specific structure can have a number of objects in it.
 * Each next structure in the
 * ProviderSpecific will be the QOS_OBJECT_HDR struct that prefaces the actual
 * data with a type and length for that object.  This QOS_OBJECT struct can 
 * repeat several times if there are several objects.  This list of objects
 * terminates either when the buffer length has been reached ( WSABUF ) or
 * an object of type QOS_END_OF_LIST is encountered.
 */
typedef struct  {

    ULONG   ObjectType;
    ULONG   ObjectLength;  /* the length of object buffer INCLUDING 
                            * this header */

} QOS_OBJECT_HDR, *LPQOS_OBJECT_HDR;


/*
 * general QOS objects start at this offset from the base and have a range 
 * of 1000
 */
#define   QOS_GENERAL_ID_BASE                      2000

#define   QOS_OBJECT_END_OF_LIST                   (0x00000001 + QOS_GENERAL_ID_BASE) 
          /* QOS_End_of_list structure passed */
#define   QOS_OBJECT_SD_MODE                       (0x00000002 + QOS_GENERAL_ID_BASE) 
          /* QOS_ShapeDiscard structure passed */
#define   QOS_OBJECT_SHAPING_RATE	           (0x00000003 + QOS_GENERAL_ID_BASE)
          /* QOS_ShapingRate structure */
#define   QOS_OBJECT_DESTADDR                      (0x00000004 + QOS_GENERAL_ID_BASE)
          /* QOS_DestAddr structure (defined in qossp.h) */


/*
 * This structure is used to define the behaviour that the traffic
 * control packet shaper will apply to the flow.
 *
 * TC_NONCONF_BORROW - the flow will receive resources remaining 
 *  after all higher priority flows have been serviced. If a 
 *  TokenRate is specified, packets may be non-conforming and
 *  will be demoted to less than best-effort priority.
 *  
 * TC_NONCONF_SHAPE - TokenRate must be specified. Non-conforming
 *  packets will be retianed in the packet shaper until they become
 *  conforming.
 *
 * TC_NONCONF_DISCARD - TokenRate must be specified. Non-conforming
 *  packets will be discarded.
 *
 */

typedef struct _QOS_SD_MODE {

    QOS_OBJECT_HDR   ObjectHdr;
    ULONG            ShapeDiscardMode;

} QOS_SD_MODE, *LPQOS_SD_MODE;

#define TC_NONCONF_BORROW      0
#define TC_NONCONF_SHAPE       1
#define TC_NONCONF_DISCARD     2
#define TC_NONCONF_BORROW_PLUS 3 // Not supported currently


/*
 * This structure allows an app to specify a prorated "average token rate" using by
 * the traffic shaper under SHAPE modehaper queue. It is expressed in bytes per sec.
 *
 * ShapingRate (bytes per sec.)
 *
 */

typedef struct _QOS_SHAPING_RATE {

    QOS_OBJECT_HDR   ObjectHdr;
    ULONG            ShapingRate;

} QOS_SHAPING_RATE, *LPQOS_SHAPING_RATE;



#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif  /* __QOS_H_ */

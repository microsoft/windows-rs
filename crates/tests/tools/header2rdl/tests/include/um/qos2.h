/*++

Copyright (c) 2004  Microsoft Corporation

Module Name:

    qos2.h

Abstract:

    This module contains QOS structures and function headers

--*/

#pragma once
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#include <ws2tcpip.h>
#include <mstcpip.h>

//
// Support calls from C++
//
#if defined(__cplusplus)
    #define ExternC extern "C"
#else
    #define ExternC
#endif

//
// Each admitted flow has a unique Flow ID. This ID is valid only
// in the process which called QOSAddSocketToFlow() for that flow
// with the handle returned from QOSCreateHandle
//
typedef ULONG QOS_FLOWID, *PQOS_FLOWID;

//
// Definitions of various traffic types. Application identifies
// each flow as being of a certain type. This enables the QOS
// subsystem to apply user-specified per-type policies to flows
//
// QOSTrafficTypeBestEffort:
//      
//      This service type requests the same network priority to the traffic 
//      as regular traffic not associated to the qWave api. For home scenarios, 
//      this is DLNA class DLNAQOS_1.
//
// QOSTrafficTypeBackground:
//
//      This service type requests a network priority to the traffic lower than 
//      traffic of type QOSTrafficTypeBestEffort. For example, this service 
//      could be used for traffic of applications doing data backups. For home 
//      scenarios, this is DLNA class DLNAQOS_0.
//
// QOSTrafficTypeExcellentEffort:
//
//      This service type requests a network priority to the traffic higher than 
//      QOSTrafficTypeBestEffort. This service type should be used for data 
//      traffic that, although not A/V, is more important than normal end-user 
//      scenarios. For example, email traffic. This service type is not part of 
//      the DLNA specification.
//
// QOSTrafficTypeAudioVideo:
//
//      This service type should be used for A/V streaming scenarios such as 
//      MPEG2 streaming. For home scenarios, this is DLNA class DLNAQOS_2.
//
// QOSTrafficTypeVoice:
//
//      This service type should be used for realtime voice streams such as 
//      VoIP. This service type is not part of the DLNA specification.
//
// QOSTrafficTypeControl:
//
//      This service type should only be used for the most critical data. For 
//      example, you might use it for data carrying user inputs to an A/V 
//      experience, e.g. play, pause, FF, RW, etc. The A/V traffic however 
//      should use QOSTrafficTypeAudioVideo. For home scenarios, this is DLNA 
//      class DLNAQOS_3.
//
typedef enum _QOS_TRAFFIC_TYPE 
{
    QOSTrafficTypeBestEffort        = 0,
    QOSTrafficTypeBackground        = 1,
    QOSTrafficTypeExcellentEffort   = 2,
    QOSTrafficTypeAudioVideo        = 3,
    QOSTrafficTypeVoice             = 4,
    QOSTrafficTypeControl           = 5
} QOS_TRAFFIC_TYPE, *PQOS_TRAFFIC_TYPE;

//
// This enum lists the operations that may be given to the QOSSetFlow API.
//
// QOSSetTrafficType: 
//
//      Allows an application to change the traffic type of its 
//      flow. For example, this may be useful if you are alternating between 
//      best effort and other types.
//
//      This must be accompanied by a QOS_TRAFFIC_TYPE value.
//
// QOSSetOutgoingRate:
//
//      This allows an application to define the characteristics of the 
//      outgoing traffic on its flow. This may be used to shape the outgoing 
//      traffic or to create a contract linking packet marking to packet 
//      shaping. Such a contract is required for environments were policy uses 
//      admission control. The traffic type will not be respected until a 
//      shaping rate is in place.
//
//      Must be accompagnied by a QOS_FLOWRATE_OUTGOING structure.
//
// QOSSetOutgoingDSCPValue:
//
//      This allows an application to specify the precise DSCP value used in
//      outgoing traffic.  This setting is overriden if an adaptive flow 
//      requests prioritization on the same machine.  This setting requires
//      the calling application be a member of the Administrators or the
//      Network Configuration Operators group.  This setting can only be
//      applied to a non-adaptive flow.
//
//      Must be accompanied by a DWORD value representing the DSCP value to
//      use.  The value must be from 0 to 63 inclusive.
//
//      To revert the effect of setting a DSCP value, the caller should call
//      the QOSSetFlow API again with the operation value QOSSetTrafficType.
//
typedef enum _QOS_SET_FLOW
{
    QOSSetTrafficType           = 0,
    QOSSetOutgoingRate          = 1,
    QOSSetOutgoingDSCPValue     = 2
} QOS_SET_FLOW, *PQOS_SET_FLOW;

//
// This structure is returned by QOSQueryFlow and allows the application to
// verify what priority markings (or tags) are applied to its traffic. These
// values may change as the flow changes or policies get updated.
//
typedef struct _QOS_PACKET_PRIORITY
{
    ULONG   ConformantDSCPValue;    // the DSCP marking used for the flow's 
                                    // traffic that respects the flow rate 
                                    // specified

    ULONG   NonConformantDSCPValue; // the DSCP marking used for the flow's 
                                    // traffic that exceeds the flow rate 
                                    // specified. Applicable only if shaping 
                                    // behavior is set to 
                                    // QOSUseNonConformantMarkings

    ULONG   ConformantL2Value;      // the L2 tag used for the flow's traffic 
                                    // that respects the flow rate specified

    ULONG   NonConformantL2Value;   // the L2 tag used for the flow's traffic 
                                    // that respects the flow rate specified.
                                    // Applicable only if shaping behavior is 
                                    // set to QOSUseNonConformantMarkings
} QOS_PACKET_PRIORITY, *PQOS_PACKET_PRIORITY;

typedef struct _QOS_FLOW_FUNDAMENTALS
{
    BOOL    BottleneckBandwidthSet; // TRUE if the BottleneckBandwidth field
                                    // contains a value.
    UINT64  BottleneckBandwidth;    // In units of bits/s ; does not include 
                                    // layer 3

    BOOL    AvailableBandwidthSet;  // TRUE if the AvailableBandwidth field
                                    // contains a value.
    UINT64  AvailableBandwidth;     // In units of bits/s ; does not include 
                                    // layer 3

    BOOL    RTTSet;                 // TRUE if the RTT field
                                    // contains a value.
    UINT32  RTT;                    // RTT is in microseconds. 
} QOS_FLOW_FUNDAMENTALS, *PQOS_FLOW_FUNDAMENTALS;

typedef enum _QOS_FLOWRATE_REASON
{
    QOSFlowRateNotApplicable            = 0,
    QOSFlowRateContentChange            = 1,
    QOSFlowRateCongestion               = 2,
    QOSFlowRateHigherContentEncoding    = 3,
    QOSFlowRateUserCaused               = 4
} QOS_FLOWRATE_REASON, *PQOS_FLOWRATE_REASON;

//
// This enum is used to define the shaping behavior. Remember that, if policy
// demands admission control, packet priority is only applied through a contract 
// with the application. This enum allows you to define how the contract is 
// enforced.
//
// QOSShapeOnly: if the flow is set to shape only, qWave will use Window's 
// scheduler to enforce the flow rate requested. Any data packet that would 
// exceed the rate will be delayed until appropriate. Packets will be marked
// as best effort packets.
//
// QOSShapeAndMark: if the flow is set to shape, qWave will use Window's 
// scheduler to enforce the flow rate requested. Any data packet that would 
// exceed the rate will be delayed until appropriate. Packets will always 
// receive conformant priority values.
//
// QOSUseNonConformantMarkings: if the flow is set to use non conformant values,
// qWave will not enforce the flow rate requested. If sending a data packet 
// would exceed the flow rate, this packet will receive a priority value 
// indicating it is non-conformant. Such packets will receive default treatment -
// which is best effort - from network equipment. This may lead to lost packets
// and/or re-ordered packets.
//
typedef enum _QOS_SHAPING{
    QOSShapeOnly                        = 0,
    QOSShapeAndMark                     = 1,
    QOSUseNonConformantMarkings         = 2
} QOS_SHAPING, *PQOS_SHAPING;

#define QOS_OUTGOING_DEFAULT_MINIMUM_BANDWIDTH  0xFFFFFFFF

//
// This struct describes the information required by qWave to accept a flowrate
// from the application. We recommend that you read the description for
// QOS_ADD_OVERHEAD and QOS_SUBTRACT_OVERHEAD before using this structure.
//
// Bandwidth: the rate (in bits/s) you expect to send your traffic at. As a 
// warning, you should note that traffic on the network is measured at the IP 
// level and not at the application level. Thus, the rate you specify should 
// account for the IP and protocol headers. Although there are various ways
// to estimate this final rate, you may wish to use the QOS_ADD_OVERHEAD and
// QOS_SUBTRACT_OVERHEAD functions for these calculations.
//
// If you are unsure what bandwidth value you need but expect to use very little
// use QOS_OUTGOING_DEFAULT_MINIMUM_BANDWIDTH. The system will allocate you
// a small amount of bandwidth for your operations.
//
// ShapingBehavior: how the contract for admission control will be enforced
//
// Reason: if a rate change has occured, this reason should indicate why.
typedef struct _QOS_FLOWRATE_OUTGOING
{
    UINT64                  Bandwidth;      // In units of bits/s
    QOS_SHAPING             ShapingBehavior;// Shaping behavior
    QOS_FLOWRATE_REASON     Reason;         // Optional field for the 
                                            // application to indicate why it's 
                                            // changing a flow's data rate
} QOS_FLOWRATE_OUTGOING, *PQOS_FLOWRATE_OUTGOING;

//
// When using the enum QOSQueryOutgoingRate you should expect the returned rate
// to measure bandwidth at layer 3. If you wish to adjust this rate, based on 
// your application's characteristics, for the IP header and protocol header
// overhead, you should review the QOS_SUBTRACT_OVERHEAD inlined function.
typedef enum _QOS_QUERY_FLOW
{
    QOSQueryFlowFundamentals    = 0,
    QOSQueryPacketPriority      = 1,
    QOSQueryOutgoingRate        = 2
} QOS_QUERY_FLOW, *PQOS_QUERY_FLOW;

typedef enum _QOS_NOTIFY_FLOW
{
    QOSNotifyCongested          = 0,
    QOSNotifyUncongested        = 1,       
    QOSNotifyAvailable          = 2       
} QOS_NOTIFY_FLOW, *PQOS_NOTIFY_FLOW;

//
// The type for QOS protocol version numbers.
// For Vista, the version should be
// MajorVersion: 1
// MinorVersion: 0

typedef struct _QOS_VERSION
{
    USHORT MajorVersion;
    USHORT MinorVersion;
} QOS_VERSION, *PQOS_VERSION;

#define QOS_QUERYFLOW_FRESH         0x00000001
#define QOS_NON_ADAPTIVE_FLOW       0x00000002

__inline
INT
QOS_HEADER_OVERHEAD(
    _In_    INT     af, 
    _In_    INT     protocol
){
    UINT32 overhead;

    if (af == AF_INET)
        overhead = 20;                          // IPv4 header overhead in bytes
    else
        overhead = 40;                          // IPv6 header overhead in bytes

    if (protocol == IPPROTO_TCP)
        overhead += 20;                         // TCP header overhead in bytes
    else
        overhead += 8;                          // UDP header overhead in bytes

    return overhead;    
}


//
// Description:
//
//  API to calculate the impact of IP and protocol header overhead on a data 
//  rate.
//
// Arguments:
//
//      .af             - Address family used to create the socket. Please 
//                        review the socket and WSASocket documentation. This 
//                        should be AF_INET or AF_INET6
//
//      .protocol       - Protocol type used to create the socket. Please review 
//                        the socket and WSASocket documentation. This should 
//                        be IPPROTO_TCP or IPPROTO_UDP
//
//      .targetDataPacketSize
//                      - This is the expected packet size of your data stream.
//                        ***See note below on targetDataPacketSize***
//
//      .dataRate       - Your dataRate in bits/s.
//
// Note on targetDataPacketSize: 
//
//  If you're using a TCP socket on IPv4 and will be making large sends then you 
//  would expect the data packet size to be 1460 bytes: Ethernet has an MTU of 
//  1500 bytes and the overhead of an IPv4 header is typically 20 bytes 
//  while a TCP header is also 20 bytes.
//
//  If you're using a UDP socket and your packet size varies, you may wish to 
//  pass in a reasonnable minimal value. This would adjust your rate for the
//  worst case.
//
//  The value 0 is an invalid parameter which will result in a division by 0.
//
// Return Values:
//
//  This call will return the data rate, in bits/s, augmented by the overhead
//  on each packet given the address family and the protocol you've created your
//  socket with.
//
__inline
UINT64
QOS_ADD_OVERHEAD(
    _In_    INT     af, 
    _In_    INT     protocol, 
    _In_    UINT32  targetDataPacketSize, 
    _In_    UINT64  dataRate
){
    UINT32 overhead;
    double d;
    UINT64 r;

    //
    // Calculate the header overhead
    overhead = QOS_HEADER_OVERHEAD(af, protocol);

    //
    // Convert overhead and dataRate to bits
    overhead *= 8;
    targetDataPacketSize *= 8;

    //
    // The adjustment is:
    //
    //                         (       dataRate                  )
    // returnRate = dataRate + ( -------------------- * overhead )
    //                         ( targetDataPacketSize            )
    //
    // For each packet we expect to see go out, we need to add the 
    // overhead
    d = (double) overhead;
    d /= (double) targetDataPacketSize;
    d *= (double) dataRate;

    r = dataRate;
    r += (UINT64) d;

    return r;
}

//
// Description:
//
//  API to calculate the impact of IP and protocol header overhead on a data 
//  rate.
//
// Arguments:
//
//      .af             - Address family used to create the socket. Please 
//                        review the socket and WSASocket documentation. This 
//                        should be AF_INET or AF_INET6
//
//      .protocol       - Protocol type used to create the socket. Please review 
//                        the socket and WSASocket documentation. This should 
//                        be IPPROTO_TCP or IPPROTO_UDP
//
//      .targetDataPacketSize
//                      - This is the expected packet size of your data stream.
//                        ***See note below on targetDataPacketSize***
//
//      .dataRate       - Your dataRate in bits/s.
//
// Note on targetDataPacketSize: 
//
//  If you're using a TCP socket on IPv4 and will be making large sends then you 
//  would expect the data packet size to be 1460 bytes: Ethernet has an MTU of 
//  1500 bytes and that the overhead of an IPv4 header is typically 20 bytes 
//  while a TCP header is also 20 bytes.
//
//  If you're using a UDP socket and your packet size varies, you may wish to 
//  pass in a reasonnable minimal value. This would adjust your rate for the
//  worst case.
//
// Return Values:
//
//  This call will return the data rate, in bits/s, reduced by the overhead
//  on each packet given the address family and the protocol you've created your
//  socket with.
//
__inline
UINT64
QOS_SUBTRACT_OVERHEAD(
    _In_    INT     af, 
    _In_    INT     protocol, 
    _In_    UINT32  targetDataPacketSize, 
    _In_    UINT64  dataRate
){
    UINT32 overhead;
    double d;
    UINT64 r;

    //
    // Calculate the header overhead
    overhead = QOS_HEADER_OVERHEAD(af, protocol);

    //
    // Convert overhead and dataRate to bits
    overhead *= 8;
    targetDataPacketSize *= 8;

    //
    // The adjustment is:
    //
    //                         (            dataRate                        )
    // returnRate = dataRate - ( ------------------------------- * overhead )
    //                         ( targetDataPacketSize + overhead            )
    //
    // For each packet we expect to see go out, we need to add the 
    // overhead
    d = (double) overhead;
    d /= (double) targetDataPacketSize + overhead;
    d *= (double) dataRate;

    r = dataRate;
    r -= (UINT64) d;

    return r;
}

//
// Description:
//
//  API to initialize QOS subsystem. Every process intending to use
//  QOS must first make the QOSCreateHandle call.
//
//  The handle returned by this call is useful for performing
//  overlapped IO. For example, it can be associated with a
//  IOCP to receive overlapped completion notifications.
//
//  Although a single QOSHandle would be sufficient for most applications,
//  applications have the option of calling QOSCreateHandle multiple times
//  to obtain multiple handles
//
//      .Version        - Specifies what version of this API you wish to use.
//                        This parameter must be of type QOS_VERSION. For Vista
//                        the only valid value is 1.0
//
//      .QOSHandle      - On output, if the call is successful, this will return
//                        a file handle to the QOS subsystem. This handle should
//                        be closed with QOSCloseHandle.
//
// Return Values:
//
//  If the function succeeds, the return value is nonzero.
//
//  If the function fails, the return value is zero. To get extended error 
//  information, call GetLastError. Here are some of the errors possible.
//  This list is not exhaustive.
//
//      ERROR_SERVICE_DEPENDENCY_FAIL
//          One of the dependencies of the service is unavailable. The qWave
//          service could not be started.
//
//      ERROR_RESOURCE_DISABLED
//          One of the resources required by the service is unavailable. This 
//          error may be returned if the user has not enabled the firewall
//          exception for the qWave service. Please see the developer guidelines 
//          and MSFT firewall documentation for more details.
//
ExternC
BOOL
WINAPI
QOSCreateHandle(
    _In_    PQOS_VERSION    Version,
    _Out_   PHANDLE         QOSHandle
);

//
// Description:
//
//  API to close a handle returned by QOSCreateHandle
//
//  When closing a handle, all flows added on this handle are immediately 
//  removed from the system. Any traffic going out a socket used to create these 
//  flows will no longer be marked. Moreover, any pending operations for these 
//  flows are completed with ERROR_ABORTED.
//
//  If any clients were tracked through this handle, the subsystem will continue
//  tracking these for some limited amount of time. This can be stopped by 
//  calling either QOSStopTrackingClient on this handle before closing it or 
//  through another handle once this one has been closed.
//
// Arguments:
//
//      .QOSHandle      - Handle to the QOS subsystem obtained through 
//                        QOSCreateHandle.
//
// Return Values:
//
//  If the function succeeds, the return value is nonzero.
//
//  If the function fails, the return value is zero. To get extended error 
//  information, call GetLastError.
//
ExternC
BOOL
WINAPI
QOSCloseHandle(
    _In_    HANDLE         QOSHandle
);

//
// Description:
//
//  API to inform the QOS subsystem of the existence of a new client. 
//  The QOS subsystem will start gathering statistics about this client device. 
//  This call is NOT required to add a flow, but it is recommended for adaptive
//  flows.
//
//  Ideally, an application would make this API call as soon as it is aware of a 
//  possible client device with which it may need to create a QOS flow. By using
//  this call, you increase the likelihood that qWave will have gathered 
//  sufficient information on the network path to assist you when you attempt to
//  set the flow rate.
//
// Arguments:
//
//      .QOSHandle      - Handle to the QOS subsystem obtained through 
//                        QOSCreateHandle.
//
//      .DestAddr       - IP address of the client device. Note that a client 
//                        is identified strictly by it's IP address and address 
//                        family. A port number is not required and will be 
//                        ignored.
//
//      .Flags          - Used to modify the behavior of the 
//                        QOSStartTrackingClient call. Reserved for future use.
//
// Return Values:
//
//  If the function succeeds, the return value is nonzero.
//
//  If the function fails, the return value is zero. To get extended error 
//  information, call GetLastError. Here are some of the errors possible.
//  This list is not exhaustive.
//
//      ERROR_BAD_NET_NAME
//          qWave was unable to reach the host specified. Specifically, address
//          resolution failed.
//
//      ERROR_HOST_UNREACHABLE
//          The qWave service was unable to communicate with the peer host to
//          run qWave experiments. This could be a sign that there is no 
//          qWave sink on the remote host or that a firewall is blocking 
//          communication.
//
//      ERROR_NOT_SUPPORTED
//          The qWave subsystem cannot track information about the destination
//          you've specified. It could be that the other host does does not 
//          have the required components or that it is not on the same link.
//
ExternC
BOOL
WINAPI
QOSStartTrackingClient(
    _In_        HANDLE          QOSHandle,
    _In_        PSOCKADDR       DestAddr,
    _Reserved_  DWORD           Flags
);

//
// Description:
//
//  API to remove a client. The QOS subsystem will stop gathering statistics
//  about this client device. This call will only be accepted if 
//  QOSStartTrackingClient was previously called on the host. If a flow is 
//  currently in progress, this will not affect the flow.
//
// Arguments:
//
//      .QOSHandle      - Handle to the QOS subsystem obtained through 
//                        QOSCreateHandle.
//
//      .DestAddr       - IP address of the client device
//
//      .Flags          - Used to modify the behavior of the 
//                        QOSStopTrackingClient call. Reserved for future use.
//
// Return Values:
//
//  If the function succeeds, the return value is nonzero.
//
//  If the function fails, the return value is zero. To get extended error 
//  information, call GetLastError.
//
ExternC
BOOL
WINAPI
QOSStopTrackingClient(
    _In_        HANDLE          QOSHandle,
    _In_        PSOCKADDR       DestAddr,
    _Reserved_  DWORD           Flags
);

//
// Description:
//
//  API to enumerate all the existing flows. This call requires administrative 
//  rights. Through it the caller can obtain the list of current flow IDs on the 
//  system. Using QOSQueryFlow, one can then query the flows.
//
// Arguments:
//
//      .QOSHandle      - Handle to the QOS subsystem obtained through 
//                        QOSCreateHandle.
//
//      .Size           - On input represents the size in bytes of the buffer. 
//                        On output, if successful, the amount of bytes copied 
//                        in the buffer. If the call fails with 
//                        ERROR_INSUFFICIENT_BUFFER, the parameter size will 
//                        indicate the minimum required buffer size.
//
//      .Buffer         - On output contains an array of QOS_FLOWID.
//
// Note: 
//
//  One cannot modify flows from another process.
//
// Return Values:
//
//  If the function succeeds, the return value is nonzero.
//
//  If the function fails, the return value is zero. To get extended error 
//  information, call GetLastError. Here are some of the errors possible.
//  This list is not exhaustive.
//
//      ERROR_ACCESS_DENIED
//          The caller does not have the administrator rights required 
//          to perform this call.
//
ExternC
BOOL
WINAPI
QOSEnumerateFlows(
    _In_                HANDLE      QOSHandle,
    _Inout_             PULONG      Size,
    _Out_writes_bytes_(*Size) PVOID       Buffer
);

//
// Description:
//
//  API to add a new flow. Note that the flow's traffic is not affected through
//  this call. There are two categories of applications that will use this api:
//  adaptive and non-adaptive. An adaptive application will make use of 
//  notifications and information in QOS_FLOW_FUNDAMENTALS to adapt to changing
//  network characteristics (such as congestion). qWAVE utilizes Link Layer
//  Topology Discovery (LLTD) QoS extensions for adaptive flows, which may be
//  present on the destination. A non-adaptive application either does not 
//  desire to adapt to changing network characteristics (priority marking and/or
//  throttling only), or is sending traffic to an endpoint that does not support
//  this capability as indicated by ERROR_NOT_SUPPORTED. For adaptive 
//  applications, the caller should immediately call QOSSetFlow with the 
//  QOSSetOutgoingRate parameter. Non-adaptive applications or applications 
//  creating non-adaptive flows should call this api with the 
//  QOS_NON_ADAPTIVE_FLOW flag set.
//
// Arguments:
//
//      .QOSHandle      - Handle to the QOS subsystem obtained through 
//                        QOSCreateHandle.
//
//      .Socket         - The socket the application will use to source traffic. 
//                        ***See note below on Socket***
//
//      .DestAddr       - The destination address the application will send 
//                        traffic to. A  destination port must be specified. 
//                        This parameter is optional for connected sockets. See 
//                        the note below.
//
//      .TrafficType    - Describes the type of traffic your flow will be used 
//                        for. This parameter will be ignored when adding a 
//                        socket to an existing flow. 
//
//      .Flags          - Used to modify the behavior of the QOSAddSocketToFlow
//                        call. 
//                        ***See note below on Flags***
//
//      .FlowId         - On input *FlowId MUST be 0 to create a new flow. On 
//                        output, if the call is successful, a flow ID is 
//                        generated and placed in *FlowId. 
//
//                        If you are adding a socket to an existing flow, 
//                        *FlowId will be the flow ID of the existing flow.
//                        ***See note below on Flags***
//
// Note on Socket: 
//
//  If the app wishes to use a different interface to source its data
//  than the one favored by the routing table, it should bind its socket 
//  before calling QOSAddSocketToFlow.
//
// Note on the DestAddr parameter:
//
//  The DestAddr parameter is optional. Since the Qwave api must always 
//  know the destination host (and IP port) to which your traffic will be 
//  sent here are the ramifications:
//
//      1) If your socket is not connected, you MUST specify this parameter. 
//      2) If your socket is connected, you do not need to specify this 
//         parameter however, if you do, the destination host and port must 
//         match that use in the socket's connect call.
//      3) Since, for TCP, the connect call incurs a delay (dependent on  
//         network conditions and the peer host) you may start qWave experiments
//         beforehand. You would call QOSAddSocketToFlow passing in the  
//         application's peer IP address and port.
//
// Note on TrafficType: 
// 
//  Its worth noting that traffic types, if policy allows, will map to a 
//  protocol level tagging of your traffic. For example, the traffic types 
//  equivalent to DLNA may result in the DSCP markings specified in DLNA 
//  being applied. Whether marking is applied, and if so which value, is 
//  controlled by the policy system. In effect this is a request which may,
//  or may not, as appropriate for the network infrastructure, be honored.
//
//  Policy may also require that you run through some form of admission 
//  control and bind yourself to a contract. This is useful to prevent
//  applications from swamping the network. Such admission control will
//  rely on both your traffic type and the flow rate of your traffic. When 
//  possible, you should specify the flow rate of your application using the
//  QOSSetFlow call with the QOSSetOutgoingRate operation.
//
// Note on Flags:
//
//  Only one flag is currently supported for QOSAddSocketToFlow.
//
//      QOS_NON_ADAPTIVE_FLOW - If specified, the QOS subsystem will not gather
//                              data about the network path for this flow. As a 
//                              result, APIs which rely on bandwidth estimation
//                              techniques will not be available. For example, 
//                              this would block QOSQueryFlow with 
//                              QOSQueryFlowFundamentals and QOSNotifyFlow with
//                              QOSNotifyCongested, QOSNotifyUncongested or
//                              QOSNotifyAvailable.
//
//                              An application should use this flag if it does
//                              not intent to adapt its flow to changes in the
//                              state of the network be it because of its
//                              scenario, type of network or capabilities of
//                              the receiving host.
//
//                              You can only add multiple sockets to the same 
//                              flow if the flow is not adaptive. You must also 
//                              specify this flag when you call to add a socket
//                              to an existing flow.
//
// Return Values:
//
//  If the function succeeds, the return value is nonzero.
//
//  If the function fails, the return value is zero. To get extended error 
//  information, call GetLastError. Here are some of the errors possible.
//  This list is not exhaustive.
//
//      ERROR_BAD_NET_NAME
//          qWave was unable to reach the host specified. Specifically, address
//          resolution failed.
//
//      ERROR_CONNECTION_REFUSED
//          QOS could not connect to destination device. The remote host
//          rejected the connection.
//
//      ERROR_HOST_UNREACHABLE
//          The host is not reachable given the current network configuration. 
//          If the host was previously reachable, you should confirm that there
//          hasn't been a PnP state change
//
//      ERROR_NOT_SUPPORTED
//          The qWave subsystem cannot track information about the destination
//          you've specified. It could be that the other host does does not 
//          have the required components or that it is not on the same link.
//
ExternC
BOOL
WINAPI
QOSAddSocketToFlow(
    _In_        HANDLE              QOSHandle,          
    _In_        SOCKET              Socket,
    _In_opt_    PSOCKADDR           DestAddr,
    _In_        QOS_TRAFFIC_TYPE    TrafficType,
    _In_opt_    DWORD               Flags,
    _Inout_     PQOS_FLOWID         FlowId
);

//
// Description:
//
//  API used by app to notify QOS subsystem that a previously admitted flow has 
//  been terminated by the app. QOS subsystem uses this call to update its 
//  internal information
//
// Arguments:
//
//      .QOSHandle      - Handle to the QOS subsystem obtained through 
//                        QOSCreateHandle.
//
//      .Socket         - Socket to be removed from the flow
//                        ***See note below on Socket***
//
//      .FlowId         - The flow which the app is interested in modifying. 
//                        This is obtained through QOSAddSocketToFlow.
//
//      .Flags          - Used to modify the behavior of the 
//                        QOSRemoveSocketFromFlow call. This is currently 
//                        reserved for future use.
//
// Note: 
//
//  Closing a QOSHandle will automatically abort all pending operations issued 
//  on that QOSHandle. If the handle is closed while a QOSRemoveSocketFromFlow 
//  call is still in progress, the call will complete with 
//  ERROR_OPERATION_ABORTED. 
//
// Note on Socket: 
// 
//  Only flows created with the QOS_NON_ADAPTIVE_FLOW flag may have multiple
//  sockets added to the same flow. By passing a socket parameter in this
//  api call you can remove each of these individually. 
//
//  If you do not pass a socket, the flow will be destroyed.
//
//  If only one socket was attached to the flow, passing this socket as a 
//  parameter to this api and passing NULL as a socket are equivalent calls.
//
// Return Values:
//
//  If the function succeeds, the return value is nonzero.
//
//  If the function fails, the return value is zero. To get extended error 
//  information, call GetLastError. Here are some of the errors possible.
//  This list is not exhaustive.
//
//      ERROR_NOT_FOUND
//          Invalid FlowId specified
//
ExternC
BOOL
WINAPI
QOSRemoveSocketFromFlow(
    _In_        HANDLE              QOSHandle,
    _In_opt_    SOCKET              Socket,
    _In_        QOS_FLOWID          FlowId,
    _Reserved_  DWORD               Flags
);

//
// Description:
//
//  This API is used by the app to inform the QOS subsystem of change in a flow.
//
// Arguments:
//
//      .QOSHandle      - Handle to the QOS subsystem obtained through 
//                        QOSCreateHandle.
//
//      .FlowId         - The flow which the app is interested in modifying 
//                        This is obtained through QOSAddSocketToFlow.
//
//      .Operation      - What modification on the flow you are trying to 
//                        apply. This must be of the type QOS_SET_FLOW. The 
//                        input parameter should match the requested flow 
//                        information.
//                        ***See note below on Operation***
//
//      .Size           - The length, in bytes, of Buffer. You should specify 
//                        the correct buffer length for the structure you
//                        specify in Buffer.
//                        ***See note below on Operation***
//
//      .Buffer         - Pointer to the buffer to describe the modication.
//                        ***See note below on Operation***
//
//      .Flags          - Used to modify the behavior of the QOSSetFlow call.
//                        This is currently reserved for future use.
//
//      .Overlapped     - Pointer to an OVERLAPPED structure. 
//
// Note on Operation:
//
//  Given the input parameter for operation, the content of the buffer passed in 
//  will differ. Here is the mapping:
//
//      QOSSetTrafficType           - QOS_TRAFFIC_TYPE
//      QOSSetOutgoingRate          - QOS_FLOWRATE_OUTGOING
//      QOSSetOutgoingDSCPValue     - DWORD
//
//  Specific to QOS_FLOWRATE_OUTGOING structure:
//
//      Bandwidth is defined as number of bits per second. It should include 
//      Layer 3 overhead (e.g. IPv4 header, UDP, etc.). You may want to use the 
//      macro QOS_ADD_OVERHEAD to help you estimate the overhead.
//
//      Using Bandwidth == 0 disables shaping and stops affecting the flow (e.g. 
//      the traffic is not marked anymore). MinPacketSize will be ignored.
//
// Return Values:
//
//  If the function succeeds, the return value is nonzero.
//
//  If the function fails, the return value is zero. To get extended error 
//  information, call GetLastError. Here are some of the errors possible.
//  This list is not exhaustive.
//
//      ERROR_ACCESS_DISABLED_BY_POLICY
//          The QOS subsystem is currently configured by policy to not allow 
//          this operation on the network path between this host and 
//          your destination host.
//
//      ERROR_BUSY
//          Indicates that the QOS subsystem has not had enough CPU cycles to
//          estimate the network characteristics of the path.
//
//      ERROR_IO_PENDING          
//          Indicates that update flow request was successfully initiated
//          (results will be returned during overlapped completion)
//      
//      ERROR_NETWORK_BUSY
//          Indicates that the requested flow properties were not available
//          on this path. The network currently cannot support the 
//          characteristics you requested
//
//      ERROR_NOT_FOUND
//          Invalid FlowId specified
//
//      ERROR_NOT_SUPPORTED
//          The operation you're trying to do requires information about the
//          network which the qWave subsystem does not have. Obtaining this
//          information on your network is currently not supported.
//
//      ERROR_RETRY
//          There is currently insufficient data about networking conditions
//          to answer your query. This is typically a transient state where
//          qWave has erred on the side of caution as it waits for more data
//          before ascertaining the state of the network.
//
//      ERROR_ACCESS_DENIED
//          The caller does not have the administrator rights required 
//          to perform this call.
//
ExternC
BOOL
WINAPI
QOSSetFlow(
    _In_                HANDLE          QOSHandle,          
    _In_                QOS_FLOWID      FlowId,             
    _In_                QOS_SET_FLOW    Operation,
    _In_                ULONG           Size,
    _In_reads_bytes_(Size)   PVOID           Buffer,  
    _Reserved_          DWORD           Flags,
    _Out_opt_           LPOVERLAPPED    Overlapped
);

//
// Description:
//
//  API to query information about a flow. 
//
// Arguments:
//
//      .QOSHandle      - Handle to the QOS subsystem obtained through 
//                        QOSCreateHandle.
//
//      .FlowId         - The flow which the app is interested in receiving 
//                        information about. This is obtained through 
//                        QOSAddSocketToFlow.
//
//      .Operation      - What information about the flow you are trying to 
//                        obtain. This must be of the type QOS_QUERY_FLOW. The 
//                        input parameter should match the requested flow 
//                        information.
//                        ***See note below on Operation***
//
//      .Size           - The length, in bytes, of Buffer. If the buffer 
//                        specified is too small, the call will fail with error
//                        ERROR_INSUFFICIENT_BUFFER and this parameter will 
//                        be updated the appropriate buffer size.
//                        ***See note below on Operation***
//
//      .Buffer         - Pointer to the buffer to receive the queried data.
//                        ***See note below on Operation***
//
//      .Flags          - Used to modify the behavior of the QOSQueryFlow call.
//                        ***See note below on Flags***
//
//      .Overlapped     - Pointer to an OVERLAPPED structure. 
//
// Note on Operation:
//
//  Given the input parameter for operation, the buffer passed in will differ.
//  Here is the mapping.
//
//      QOSQueryFlowFundamentals    - QOS_FLOW_FUNDAMENTALS
//
//      QOSQueryPacketPriority      - QOS_PACKET_PRIORITY
//
//                                    This structure will return the actual
//                                    priority markings applied to your traffic
//                                    by the QOS subsystem given your asks and
//                                    the policy subsystem.
//
//      QOSQueryOutgoingRate        - UINT64
//
// Note on Flags:
//
//  Only one flag is currently supported for QOSQueryFlow.
//
//      QOS_QUERYFLOW_FRESH -   If specified, the QOS subsystem will only return 
//                              once fresh data is available. If fresh data is 
//                              unavailable, it will try to obtain such data. 
//                              If this is not possible, the call will fail 
//                              with ERROR_RETRY.
//
//                              This flag is only valid for 
//                              QOSQueryFlowFundamentals.
//                              Note, this parameter is ignored if the caller is
//                              not the owner of the flow.
//
// Return Values:
//
//  If the function succeeds, the return value is nonzero.
//
//  If the function fails, the return value is zero. To get extended error 
//  information, call GetLastError. Here are some of the errors possible.
//  This list is not exhaustive.
//
//      ERROR_ACCESS_DISABLED_BY_POLICY
//          The QOS subsystem is currently configured by policy to not allow 
//          this operation on the network path between this host and 
//          your destination host.
//
//      ERROR_BUSY
//          Indicates that the QOS subsystem has not had enough CPU cycles to
//          estimate the network characteristics of the path.
//
//      ERROR_HOST_UNREACHABLE
//          The host is not reachable given the current network configuration. 
//          If the host was previously reachable, you should confirm that there
//          hasn't been a PnP state change
//
//      ERROR_INSUFFICIENT_BUFFER
//          The buffer length, as specified through Size, is not sufficient
//          for the queried data. Size now contains the required size.
//
//      ERROR_IO_PENDING          
//          Indicates that update flow request was successfully initiated
//          (results will be returned during overlapped completion)
//      
//      ERROR_NOT_FOUND
//          Invalid FlowId specified
//
//      ERROR_NOT_SUPPORTED
//          The operation you're trying to do requires information about the
//          network which the qWave subsystem does not have. Obtaining this
//          information on your network is currently not supported.
//
//      ERROR_RETRY
//          There is currently insufficient data about networking conditions
//          to answer your query. This is typically a transient state where
//          qWave has erred on the side of caution as it waits for more data
//          before ascertaining the state of the network.
//
ExternC
BOOL
WINAPI
QOSQueryFlow(
    _In_                HANDLE              QOSHandle,
    _In_                QOS_FLOWID          FlowId,
    _In_                QOS_QUERY_FLOW      Operation,
    _Inout_             PULONG              Size,
    _Out_writes_bytes_(*Size) PVOID               Buffer,  
    _In_opt_            DWORD               Flags,
    _Out_opt_           LPOVERLAPPED        Overlapped
);

//
// Description:
//
//  API to receive notification of change of network characteristics.
//
// Arguments:
//
//      .QOSHandle      - Handle to the QOS subsystem obtained through 
//                        QOSCreateHandle.
//
//      .FlowId         - The flow which the app is interested in receiving 
//                        notifications on changes. This is obtained through 
//                        QOSAddSocketToFlow.
//
//      .Operation      - What notification about the flow you are trying to 
//                        obtain. This must be of the type QOS_NOTIFY_FLOW. The 
//                        input parameter should match the requested flow 
//                        information.
//
//      .Size           - The length, in bytes, of Buffer. You should specify 
//                        the correct buffer length for the structure you
//                        specify in Buffer.
//                        ***See note below on Operation***
//
//      .Buffer         - Pointer to the buffer to receive the queried data.
//                        ***See note below on Operation***
//
//      .Flags          - Used to modify the behavior of the QOSNotifyFlow call.
//                        This is currently reserved for future use.
//
//      .Overlapped     - Pointer to an OVERLAPPED structure. 
//
//
// Note on Operation:
//
//  The following notification may be supported.
//
//  1)  QOSNotifyCongested
//
//      QOSNotifyCongested will complete the operation when the network path
//      is congested. If the path is presently congested, the operation may 
//      complete immediately.
//
//      NOTE: The optional parameters Size and Buffer must be NULL.
//
//  2)  QOSNotifyUncongested
//
//      QOSNotifyUncongested will complete the operation when the network path
//      is not congested. If the path is not presently congested, the operation 
//      may complete immediately.
//
//      NOTE: The optional parameters Size and Buffer must be NULL.
//
//  2)  QOSNotifyAvailable
//
//      QOSNotifyAvailable will complete when available capacity is sufficient 
//      to allow an existing flow to be upgraded from its currently admitted
//      bandwidth to the specified bandwidth. This should be used to upgrade a 
//      transrated flow back to full bandwidth.
//      Requested bandwidth should include Layer 3 overhead. Please use the 
//      QOS_ADD_OVERHEAD function inlined in this header file as a starting 
//      point for your calculations.
//
//      The Buffer parameter should point to a UINT64 value with the requested
//      bandwidth. This value should be the target bandwidth of the flow.
//      For example, if your flow is using 3 Mb but you want to increase that 
//      by 2 Mb, the parameter Buffer should point to a UINT64 with the value
//      5 Mb.
//
//      UINT64    Requested bandwidth
//
//      The call will fail if the requested bandwidth is less than or equal 
//      to the bandwidth the flow is already using.
//
// Return Values:
//
//  If the function succeeds, the return value is nonzero.
//
//  If the function fails, the return value is zero. To get extended error 
//  information, call GetLastError. Here are some of the errors possible.
//  This list is not exhaustive.
//
//      ERROR_ACCESS_DISABLED_BY_POLICY
//          The QOS subsystem is currently configured by policy to not allow 
//          this operation on the network path between this host and 
//          your destination host.
//
//      ERROR_ALREADY_EXISTS
//          There is already a request for notification of the same type pending
//          on this flow.
//
//      ERROR_HOST_UNREACHABLE
//          The host is not reachable given the current network configuration. 
//          If the host was previously reachable, you should confirm that there
//          hasn't been a PnP state change
//
//      ERROR_IO_PENDING          
//          Indicates that update flow request was successfully initiated
//          (results will be returned during overlapped completion)
//      
//      ERROR_NOT_FOUND
//          Invalid FlowId specified
//
//      ERROR_NOT_SUPPORTED
//          The operation you're trying to do requires information about the
//          network which the qWave subsystem does not have. Obtaining this
//          information on your network is currently not supported.
//
ExternC
BOOL
WINAPI
QOSNotifyFlow(
    _In_                        HANDLE          QOSHandle,
    _In_                        QOS_FLOWID      FlowId,
    _In_                        QOS_NOTIFY_FLOW Operation,
    _Inout_opt_                 PULONG          Size,
    _Inout_updates_bytes_opt_(*Size)   PVOID           Buffer,
    _Reserved_                  DWORD           Flags,
    _Out_opt_                   LPOVERLAPPED    Overlapped
);

//
// Description:
//
//  API to cancel a pending operation like QOSSetFlow.
// 
//  Closing a QOSHandle will automatically abort all pending
//  operations issued on that QOSHandle. If the handle is closed while
//  a QOSCancel call is still in progress, the call will complete 
//  with ERROR_OPERATION_ABORTED. 
//
// Arguments:
//
//      .QOSHandle      - Handle to the QOS subsystem obtained through 
//                        QOSCreateHandle.
//
//      .Overlapped     - Pointer to an OVERLAPPED structure. This is the 
//                        OVERLAPPED structure used in the original operation.
//
// Note: 
//
//  The QOSCancel call must be made in the same process from
//  which the original to-be-cancelled call was made.
// 
// Return Values:
//
//  If the function can successfully initiate cancellation of the specified 
//  operation, the return value is nonzero. The cancelled operation completes 
//  via its completion mechanism and indicates ERROR_OPERATION_ABORTED as the 
//  completion code.
//
//  If the function fails, the return value is zero. To get extended error 
//  information, call GetLastError.
//
ExternC
BOOL
WINAPI
QOSCancel(
    _In_    HANDLE          QOSHandle,          
    _In_    LPOVERLAPPED    Overlapped
);


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


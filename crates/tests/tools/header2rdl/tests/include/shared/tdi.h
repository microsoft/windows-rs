/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    tdi.h

Abstract:

    This header file contains interface definitions for NT transport
    providers.  This interface is documented in the NT Transport
    Driver Interface (TDI) Specification, Version 2.

Revision History:

--*/

//
// Include the types which are common to TDI and other network users
//

#ifndef _TDI_USER_
#define _TDI_USER_

#pragma once
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#include <nettypes.h>

#include <ntddtdi.h>

//
// Include Transport driver interface definitions
// All of the following have two definitions; ones that correspond exactly to
// the TDI spec, and those that correspond to the NT coding standards. They
// should be equivalent.
//

typedef LONG TDI_STATUS;
typedef PVOID CONNECTION_CONTEXT;       // connection context

//
// Basic type used to represent an address at the transport level. There may
// be many addresses represented in a single address structure. If there are
// multiple addresses, a given provider must understand all of them or it can
// use none of them. Note that it is acceptible for the provider to not know
// how to use the address, as long as it knows the address type. Thus, a
// TCP/IP NetBIOS provider may know both NetBIOS and TCP/IP addresses, but
// use only the NetBIOS address; the TCP/IP address would (likely) be passed on
// to the TCP/IP provider.
//

typedef UNALIGNED struct _TA_ADDRESS {
    USHORT AddressLength;       // length in bytes of Address[] in this
    USHORT AddressType;         // type of this address
    UCHAR Address[1];           // actually AddressLength bytes long
} TA_ADDRESS, *PTA_ADDRESS;

typedef struct _TRANSPORT_ADDRESS {
    LONG TAAddressCount;            // number of addresses following
    TA_ADDRESS Address[1];          // actually TAAddressCount elements long
} TRANSPORT_ADDRESS, *PTRANSPORT_ADDRESS;

//
// define some names for the EAs so people don't have to make them up.
//

#define TdiTransportAddress "TransportAddress"
#define TdiConnectionContext "ConnectionContext"
#define TDI_TRANSPORT_ADDRESS_LENGTH (sizeof (TdiTransportAddress) - 1)
#define TDI_CONNECTION_CONTEXT_LENGTH (sizeof (TdiConnectionContext) - 1)

//
// Known Address types
//

#define TDI_ADDRESS_TYPE_UNSPEC    ((USHORT)0)  // unspecified
#define TDI_ADDRESS_TYPE_UNIX      ((USHORT)1)  // local to host (pipes, portals)
#define TDI_ADDRESS_TYPE_IP        ((USHORT)2)  // internetwork: UDP, TCP, etc.
#define TDI_ADDRESS_TYPE_IMPLINK   ((USHORT)3)  // arpanet imp addresses
#define TDI_ADDRESS_TYPE_PUP       ((USHORT)4)  // pup protocols: e.g. BSP
#define TDI_ADDRESS_TYPE_CHAOS     ((USHORT)5)  // mit CHAOS protocols
#define TDI_ADDRESS_TYPE_NS        ((USHORT)6)  // XEROX NS protocols
#define TDI_ADDRESS_TYPE_IPX       ((USHORT)6)  // Netware IPX
#define TDI_ADDRESS_TYPE_NBS       ((USHORT)7)  // nbs protocols
#define TDI_ADDRESS_TYPE_ECMA      ((USHORT)8)  // european computer manufacturers
#define TDI_ADDRESS_TYPE_DATAKIT   ((USHORT)9)  // datakit protocols
#define TDI_ADDRESS_TYPE_CCITT     ((USHORT)10) // CCITT protocols, X.25 etc
#define TDI_ADDRESS_TYPE_SNA       ((USHORT)11) // IBM SNA
#define TDI_ADDRESS_TYPE_DECnet    ((USHORT)12) // DECnet
#define TDI_ADDRESS_TYPE_DLI       ((USHORT)13) // Direct data link interface
#define TDI_ADDRESS_TYPE_LAT       ((USHORT)14) // LAT
#define TDI_ADDRESS_TYPE_HYLINK    ((USHORT)15) // NSC Hyperchannel
#define TDI_ADDRESS_TYPE_APPLETALK ((USHORT)16) // AppleTalk
#define TDI_ADDRESS_TYPE_NETBIOS   ((USHORT)17) // Netbios Addresses
#define TDI_ADDRESS_TYPE_8022      ((USHORT)18) //
#define TDI_ADDRESS_TYPE_OSI_TSAP  ((USHORT)19) //
#define TDI_ADDRESS_TYPE_NETONE    ((USHORT)20) // for WzMail
#define TDI_ADDRESS_TYPE_VNS       ((USHORT)21) // Banyan VINES IP
#define TDI_ADDRESS_TYPE_NETBIOS_EX   ((USHORT)22) // NETBIOS address extensions
#define TDI_ADDRESS_TYPE_IP6       ((USHORT)23) // IP version 6
#define TDI_ADDRESS_TYPE_NETBIOS_UNICODE_EX       ((USHORT)24) // WCHAR Netbios address

//
// Definition of address structures. These need to be packed
// and misaligned where necessary.
//

#include <pshpack1.h>

//
// Unicode NetBIOS
//
enum eNameBufferType {
    NBT_READONLY = 0,           // default
    NBT_WRITEONLY,
    NBT_READWRITE,
    NBT_WRITTEN
};

typedef UNALIGNED struct _TDI_ADDRESS_NETBIOS_UNICODE_EX_XP {
    USHORT                  NetbiosNameType;
    enum eNameBufferType    NameBufferType;
    UNICODE_STRING          EndpointName;   // Buffer should point to EndpointBuffer
    UNICODE_STRING          RemoteName;     // Buffer should point to RemoteNameBuffer

    WCHAR                   EndpointBuffer[17];   // UNICODE
    WCHAR                   RemoteNameBuffer[1];     // UNICODE
} TDI_ADDRESS_NETBIOS_UNICODE_EX_XP, *PTDI_ADDRESS_NETBIOS_UNICODE_EX_XP;

typedef UNALIGNED struct _TA_ADDRESS_NETBIOS_UNICODE_EX_XP {
    LONG TAAddressCount;
    struct _AddrNetbiosWCharEx {
        USHORT AddressLength;       // length in bytes of this address == ??
        USHORT AddressType;         // this will == TDI_ADDRESS_TYPE_NETBIOS_WCHAR_EX
        TDI_ADDRESS_NETBIOS_UNICODE_EX_XP Address[1];
    } Address [1];
} TA_NETBIOS_UNICODE_EX_ADDRESS_XP, *PTA_NETBIOS_UNICODE_EX_ADDRESS_XP;

#if (NTDDI_VERSION >= NTDDI_WINXP)
typedef TDI_ADDRESS_NETBIOS_UNICODE_EX_XP TDI_ADDRESS_NETBIOS_UNICODE_EX;
typedef TDI_ADDRESS_NETBIOS_UNICODE_EX* PTDI_ADDRESS_NETBIOS_UNICODE_EX;

typedef TA_NETBIOS_UNICODE_EX_ADDRESS_XP TA_NETBIOS_UNICODE_EX_ADDRESS;
typedef TA_NETBIOS_UNICODE_EX_ADDRESS* PTA_NETBIOS_UNICODE_EX_ADDRESS;
#endif

//
// NetBIOS
//

typedef UNALIGNED struct _TDI_ADDRESS_NETBIOS {
    USHORT NetbiosNameType;
    UCHAR NetbiosName[16];
} TDI_ADDRESS_NETBIOS, *PTDI_ADDRESS_NETBIOS;

#define TDI_ADDRESS_NETBIOS_TYPE_UNIQUE         ((USHORT)0x0000)
#define TDI_ADDRESS_NETBIOS_TYPE_GROUP          ((USHORT)0x0001)
#define TDI_ADDRESS_NETBIOS_TYPE_QUICK_UNIQUE   ((USHORT)0x0002)
#define TDI_ADDRESS_NETBIOS_TYPE_QUICK_GROUP    ((USHORT)0x0003)

#define TDI_ADDRESS_LENGTH_NETBIOS sizeof (TDI_ADDRESS_NETBIOS)

//
// NETBIOS Extended address
//

typedef struct _TDI_ADDRESS_NETBIOS_EX {
   UCHAR  EndpointName[16];               // the called name to be used in NETBT session setup
   TDI_ADDRESS_NETBIOS NetbiosAddress;
} TDI_ADDRESS_NETBIOS_EX, *PTDI_ADDRESS_NETBIOS_EX;

#define TDI_ADDRESS_LENGTH_NETBIOS_EX sizeof(TDI_ADDRESS_NETBIOS_EX)

//
// Xns address for UB
//

typedef struct _TDI_ADDRESS_NETONE {
    USHORT NetoneNameType;
    UCHAR NetoneName[20];
} TDI_ADDRESS_NETONE, *PTDI_ADDRESS_NETONE;

#define TDI_ADDRESS_NETONE_TYPE_UNIQUE  ((USHORT)0x0000)
#define TDI_ADDRESS_NETONE_TYPE_ROTORED ((USHORT)0x0001)

#define TDI_ADDRESS_LENGTH_NETONE sizeof (TDI_ADDRESS_NETONE)


//
// AppleTalk
//

typedef struct _TDI_ADDRESS_APPLETALK {
    USHORT  Network;
    UCHAR   Node;
    UCHAR   Socket;
} TDI_ADDRESS_APPLETALK, *PTDI_ADDRESS_APPLETALK;

#define TDI_ADDRESS_LENGTH_APPLETALK sizeof (TDI_ADDRESS_APPLETALK)


//
// 802.2 MAC addresses
//

typedef struct _TDI_ADDRESS_8022 {
    UCHAR MACAddress[6];
} TDI_ADDRESS_8022, *PTDI_ADDRESS_8022;

#define TDI_ADDRESS_LENGTH_8022  sizeof (TDI_ADDRESS_8022);


//
// IP address
//

typedef struct _TDI_ADDRESS_IP {
    USHORT sin_port;
    ULONG  in_addr;
    UCHAR  sin_zero[8];
} TDI_ADDRESS_IP, *PTDI_ADDRESS_IP;

#define TDI_ADDRESS_LENGTH_IP sizeof (TDI_ADDRESS_IP)

//
// IPv6 address
//

typedef struct _TDI_ADDRESS_IP6_WIN2K {
    USHORT sin6_port;
    ULONG  sin6_flowinfo;
    UCHAR  sin6_addr[16];
} TDI_ADDRESS_IP6_WIN2K, *PTDI_ADDRESS_IP6_WIN2K;

typedef struct _TDI_ADDRESS_IP6_XP {
    USHORT sin6_port;
    ULONG  sin6_flowinfo;
    USHORT sin6_addr[8];
    ULONG  sin6_scope_id;
} TDI_ADDRESS_IP6_XP, *PTDI_ADDRESS_IP6_XP;

#if (NTDDI_VERSION >= NTDDI_WINXP)
typedef TDI_ADDRESS_IP6_XP TDI_ADDRESS_IP6;
#else
typedef TDI_ADDRESS_IP6_WIN2K TDI_ADDRESS_IP6;
#endif

typedef TDI_ADDRESS_IP6* PTDI_ADDRESS_IP6;
#define TDI_ADDRESS_LENGTH_IP6 sizeof (TDI_ADDRESS_IP6)

//
// IPX address
//

typedef struct _TDI_ADDRESS_IPX {
    ULONG NetworkAddress;
    UCHAR NodeAddress[6];
    USHORT Socket;
} TDI_ADDRESS_IPX, *PTDI_ADDRESS_IPX;


#define TDI_ADDRESS_LENGTH_IPX sizeof (TDI_ADDRESS_IPX)

//
// XNS address (same as IPX)
//

typedef struct _TDI_ADDRESS_NS {
    ULONG NetworkAddress;
    UCHAR NodeAddress[6];
    USHORT Socket;
} TDI_ADDRESS_NS, *PTDI_ADDRESS_NS;


#define TDI_ADDRESS_LENGTH_NS sizeof (TDI_ADDRESS_NS)

//
// Banyan VINES IP address
//

typedef struct _TDI_ADDRESS_VNS {
    UCHAR   net_address[4];     // network address (static)
    UCHAR   subnet_addr[2];     // subnet address (dynamic)
    UCHAR   port[2];
    UCHAR   hops;           // # hops for broadcasts
    UCHAR   filler[5];          // filler, zeros
} TDI_ADDRESS_VNS, *PTDI_ADDRESS_VNS;

#define TDI_ADDRESS_LENGTH_VNS sizeof (TDI_ADDRESS_VNS)


// OSI TSAP

/*
 *   The maximum size of the tranport address (tp_addr field of a
 *   sockaddr_tp structure) is 64.
 */

#define ISO_MAX_ADDR_LENGTH 64

/*
 *   There are two types of ISO addresses, hierarchical and
 *   non-hierarchical.  For hierarchical addresses, the tp_addr
 *   field contains both the transport selector and the network
 *   address.  For non-hierarchical addresses, tp_addr contains only
 *   the transport address, which must be translated by the ISO TP4
 *   transport provider into the transport selector and network address.
 */

#define ISO_HIERARCHICAL            0
#define ISO_NON_HIERARCHICAL        1

typedef struct _TDI_ADDRESS_OSI_TSAP {
   USHORT tp_addr_type;  /* ISO_HIERARCHICAL or ISO_NON_HIERARCHICAL
*/
   USHORT tp_taddr_len;  /* Length of transport address, <= 52 */
   USHORT tp_tsel_len;   /* Length of transport selector, <= 32 */
                         /* 0 if ISO_NON_HIERARCHICAL */
   UCHAR tp_addr[ISO_MAX_ADDR_LENGTH];
} TDI_ADDRESS_OSI_TSAP, *PTDI_ADDRESS_OSI_TSAP;

#define TDI_ADDRESS_LENGTH_OSI_TSAP sizeof (TDI_ADDRESS_OSI_TSAP)

//
// Some pre-defined structures to make life easier for
// the 99.99% of us who use but one address.
//

typedef struct _TA_ADDRESS_NETBIOS {
    LONG TAAddressCount;
    struct _Addr {
        USHORT AddressLength;       // length in bytes of this address == 18
        USHORT AddressType;         // this will == TDI_ADDRESS_TYPE_NETBIOS
        TDI_ADDRESS_NETBIOS Address[1];
    } Address [1];
} TA_NETBIOS_ADDRESS, *PTA_NETBIOS_ADDRESS;

typedef struct _TA_ADDRESS_NETBIOS_EX {
    LONG TAAddressCount;
    struct _AddrNetbiosEx {
        USHORT AddressLength;       // length in bytes of this address == 36
        USHORT AddressType;         // this will == TDI_ADDRESS_TYPE_NETBIOS_EX
        TDI_ADDRESS_NETBIOS_EX Address[1];
    } Address [1];
} TA_NETBIOS_EX_ADDRESS, *PTA_NETBIOS_EX_ADDRESS;

typedef struct _TA_APPLETALK_ADDR {
    LONG TAAddressCount;
    struct _AddrAtalk {
        USHORT AddressLength;       // length in bytes of this address == 4
        USHORT AddressType;         // this will == TDI_ADDRESS_TYPE_APPLETALK
        TDI_ADDRESS_APPLETALK   Address[1];
    } Address[1];
} TA_APPLETALK_ADDRESS, *PTA_APPLETALK_ADDRESS;

typedef struct _TA_ADDRESS_IP {
    LONG TAAddressCount;
    struct _AddrIp {
        USHORT AddressLength;       // length in bytes of this address == 14
        USHORT AddressType;         // this will == TDI_ADDRESS_TYPE_IP
        TDI_ADDRESS_IP Address[1];
    } Address [1];
} TA_IP_ADDRESS, *PTA_IP_ADDRESS;

typedef struct _TA_ADDRESS_IP6 {
    LONG TAAddressCount;
    struct _AddrIp6 {
        USHORT AddressLength;       // length in bytes of this address == 24
        USHORT AddressType;         // this will == TDI_ADDRESS_TYPE_IP6
        TDI_ADDRESS_IP6 Address[1];
    } Address [1];
} TA_IP6_ADDRESS, *PTA_IP6_ADDRESS;

typedef struct _TA_ADDRESS_IPX {
    LONG TAAddressCount;
    struct _AddrIpx {
        USHORT AddressLength;       // length in bytes of this address == 12
        USHORT AddressType;         // this will == TDI_ADDRESS_TYPE_IPX
        TDI_ADDRESS_IPX Address[1];
    } Address [1];
} TA_IPX_ADDRESS, *PTA_IPX_ADDRESS;

typedef struct _TA_ADDRESS_NS {
    LONG TAAddressCount;
    struct _AddrNs {
        USHORT AddressLength;       // length in bytes of this address == 12
        USHORT AddressType;         // this will == TDI_ADDRESS_TYPE_NS
        TDI_ADDRESS_NS Address[1];
    } Address [1];
} TA_NS_ADDRESS, *PTA_NS_ADDRESS;

typedef struct _TA_ADDRESS_VNS {
    LONG TAAddressCount;
    struct _AddrVns {
        USHORT AddressLength;       // length in bytes of this address == 14
        USHORT AddressType;         // this will == TDI_ADDRESS_TYPE_VNS
        TDI_ADDRESS_VNS Address[1];
    } Address [1];
} TA_VNS_ADDRESS, *PTA_VNS_ADDRESS;

#include <poppack.h>


//
// This structure is passed with every request to TDI. It describes that
// request and the parameters to it.
//

typedef struct _TDI_REQUEST {
    union {
        HANDLE AddressHandle;
        CONNECTION_CONTEXT ConnectionContext;
        HANDLE ControlChannel;
    } Handle;

    PVOID RequestNotifyObject;
    PVOID RequestContext;
    TDI_STATUS TdiStatus;
} TDI_REQUEST, *PTDI_REQUEST;

//
// Structure for information returned by the TDI provider. This structure is
// filled in upon request completion.
//

typedef struct _TDI_REQUEST_STATUS {
    TDI_STATUS Status;              // status of request completion
    PVOID RequestContext;           // the request Context
    ULONG BytesTransferred;          // number of bytes transferred in the request

} TDI_REQUEST_STATUS, *PTDI_REQUEST_STATUS;

//
// connection primitives information structure. This is passed to the TDI calls
// (Accept, Connect, xxx) that do connecting sorts of things.
//

typedef struct _TDI_CONNECTION_INFORMATION {
    LONG UserDataLength;        // length of user data buffer
    _Field_size_bytes_(UserDataLength) PVOID UserData;             // pointer to user data buffer
    LONG OptionsLength;         // length of follwoing buffer
    _Field_size_bytes_(OptionsLength) PVOID Options;              // pointer to buffer containing options
    LONG RemoteAddressLength;   // length of following buffer
    _Field_size_bytes_(RemoteAddressLength) PVOID RemoteAddress;        // buffer containing the remote address
} TDI_CONNECTION_INFORMATION, *PTDI_CONNECTION_INFORMATION;

//
// structure defining a counted string is defined in
// \nt\public\sdk\inc\ntdefs.h as
//  typedef struct _STRING {
//    USHORT Length;
//    USHORT MaximumLength;
//    PCHAR Buffer;
//  } STRING;
//  typedef STRING *PSTRING;
//  typedef STRING ANSI_STRING;
//  typedef PSTRING PANSI_STRING;
//

//
// Event types that are known
//

#define TDI_EVENT_CONNECT              ((USHORT)0) // TDI_IND_CONNECT event handler.
#define TDI_EVENT_DISCONNECT           ((USHORT)1) // TDI_IND_DISCONNECT event handler.
#define TDI_EVENT_ERROR                ((USHORT)2) // TDI_IND_ERROR event handler.
#define TDI_EVENT_RECEIVE              ((USHORT)3) // TDI_IND_RECEIVE event handler.
#define TDI_EVENT_RECEIVE_DATAGRAM     ((USHORT)4) // TDI_IND_RECEIVE_DATAGRAM event handler.
#define TDI_EVENT_RECEIVE_EXPEDITED    ((USHORT)5) // TDI_IND_RECEIVE_EXPEDITED event handler.
#define TDI_EVENT_SEND_POSSIBLE        ((USHORT)6) // TDI_IND_SEND_POSSIBLE event handler

//
// Associate Address is done through NtDeviceIoControlFile, which passes this
// structure as its input buffer. The Handle specified in the
// NtDeviceIoControlFile is the handle of the connection returned in the
// NtCreateFile call.
//

typedef struct _TDI_REQUEST_ASSOCIATE {
    TDI_REQUEST Request;
    HANDLE AddressHandle;
} TDI_REQUEST_ASSOCIATE_ADDRESS, *PTDI_REQUEST_ASSOCIATE_ADDRESS;


//
// Disassociate Address passes no structure, uses the request code
// IOCTL_TDI_DISASSOCIATE_ADDRESS. This call will never pend.
//

//
// Connect is done through NtDeviceIoControlFile, which passes this
// structure as its input buffer. The Handle specified in the
// NtDeviceIoControlFile is the handle of the connection returned in the
// NtCreateFile call.
//

typedef struct _TDI_CONNECT_REQUEST {
    TDI_REQUEST Request;
    PTDI_CONNECTION_INFORMATION RequestConnectionInformation;
    PTDI_CONNECTION_INFORMATION ReturnConnectionInformation;
    LARGE_INTEGER Timeout;
} TDI_REQUEST_CONNECT, *PTDI_REQUEST_CONNECT;

//
// Accept is done through NtDeviceIoControlFile, which passes this
// structure as its input buffer. The Handle specified in the
// NtDeviceIoControlFile is the handle of the connection returned in the
// NtCreateFile call. Accept is called by the user when a listen completes,
// before any activity can occur on a connection. AcceptConnectionId specifies
// the connection on which the connection is accepted; in most cases, this
// will be null, which that the connection is to be accepted on the
// connection on which the listen completed. If the transport provider supports
// "forwarding" of connections (the idea that a particular connection listens
// all the time, and creates new connections as needed for incoming connection
// requests and attaches them to the listen), this is the mechanism used to
// associate connections with the listen.
//

typedef struct _TDI_REQUEST_ACCEPT {
    TDI_REQUEST Request;
    PTDI_CONNECTION_INFORMATION RequestConnectionInformation;
    PTDI_CONNECTION_INFORMATION ReturnConnectionInformation;
} TDI_REQUEST_ACCEPT, *PTDI_REQUEST_ACCEPT;

//
// Listen is done through NtDeviceIoControlFile, which passes this
// structure as its input buffer. The Handle specified in the
// NtDeviceIoControlFile is the handle of the connection returned in the
// NtCreateFile call. RequestConnectionInformation contains information about
// the remote address to be listen for connections from; if NULL, any address
// is accepted. ReturnConnectionInformation returns information about the
// remote node that actually connected.
//

typedef struct _TDI_REQUEST_LISTEN {
    TDI_REQUEST Request;
    PTDI_CONNECTION_INFORMATION RequestConnectionInformation;
    PTDI_CONNECTION_INFORMATION ReturnConnectionInformation;
    USHORT ListenFlags;
} TDI_REQUEST_LISTEN, *PTDI_REQUEST_LISTEN;

//
// Disconnect is done through NtDeviceIoControlFile, which passes this
// structure as its input buffer. The Handle specified in the
// NtDeviceIoControlFile is the handle of the connection returned in the
// NtCreateFile call. Disconnect differs from Close in offering more options.
// For example, Close terminates all activity on a connection (immediately),
// failing all outstanding requests, and tearing down the connection. With
// some providers, Disconnect allows a "graceful" disconnect, causing new activity
// to be rejected but allowing old activity to run down to completion.
//

typedef struct _TDI_DISCONNECT_REQUEST {
    TDI_REQUEST Request;
    LARGE_INTEGER Timeout;
} TDI_REQUEST_DISCONNECT, *PTDI_REQUEST_DISCONNECT;

//
// Send is done through NtDeviceIoControlFile, which passes this
// structure as its input buffer. The Handle specified in the
// NtDeviceIoControlFile is the handle of the connection returned in the
// NtCreateFile call. Note that it is possible to Send using the file system's
// Write call. This will have the same effect as doing a Send with all flags
// set to null.
//

typedef struct _TDI_REQUEST_SEND {
    TDI_REQUEST Request;
    USHORT SendFlags;
} TDI_REQUEST_SEND, *PTDI_REQUEST_SEND;

//
// Receive is done through NtDeviceIoControlFile, which passes this
// structure as its input buffer. The Handle specified in the
// NtDeviceIoControlFile is the handle of the connection returned in the
// NtCreateFile call. Note that it is possible to Receive using the file
// system's Read call. Note further that receive returns a number of TDI_STATUS
// values, which indicate things such as partial receives.
//

typedef struct _TDI_REQUEST_RECEIVE {
    TDI_REQUEST Request;
    USHORT ReceiveFlags;
} TDI_REQUEST_RECEIVE, *PTDI_REQUEST_RECEIVE;

//
// SendDatagram is done through NtDeviceIoControlFile, which passes this
// structure as its input buffer. The Handle specified in the
// NtDeviceIoControlFile is the handle of the ADDRESS (note this is
// different than above!!) returned in the NtCreateFile call. Send Datagram
// specifies  the address of the receiver through the SendDatagramInformation
// structure, using RemoteAddress to point to the transport address of the
// destination of the datagram.
//

typedef struct _TDI_REQUEST_SEND_DATAGRAM {
    TDI_REQUEST Request;
    PTDI_CONNECTION_INFORMATION SendDatagramInformation;
} TDI_REQUEST_SEND_DATAGRAM, *PTDI_REQUEST_SEND_DATAGRAM;

//
// ReceiveDatagram is done through NtDeviceIoControlFile, which passes this
// structure as its input buffer. The Handle specified in the
// NtDeviceIoControlFile is the handle of the ADDRESS (note this is
// different than above!!) returned in the NtCreateFile call. Receive Datagram
// specifies the address from which to receive a datagram through the
// ReceiveDatagramInformation structure, using RemoteAddress to point to the
// transport address of the origin of the datagram. (Broadcast datagrams are
// received by making the pointer NULL.) The actual address of the sender of
// the datagram is returned in ReturnInformation.
//
// for the receive datagram call
//

typedef struct _TDI_REQUEST_RECEIVE_DATAGRAM {
    TDI_REQUEST Request;
    PTDI_CONNECTION_INFORMATION ReceiveDatagramInformation;
    PTDI_CONNECTION_INFORMATION ReturnInformation;
    USHORT ReceiveFlags;
} TDI_REQUEST_RECEIVE_DATAGRAM, *PTDI_REQUEST_RECEIVE_DATAGRAM;

//
// SetEventHandler is done through NtDeviceIoControlFile, which passes this
// structure as its input buffer. The Handle specified in the
// NtDeviceIoControlFile is the handle of the ADDRESS (note this is
// different than above!!) returned in the NtCreateFile call.

typedef struct _TDI_REQUEST_SET_EVENT {
    TDI_REQUEST Request;
    LONG EventType;
    PVOID EventHandler;
    PVOID EventContext;
} TDI_REQUEST_SET_EVENT_HANDLER, *PTDI_REQUEST_SET_EVENT_HANDLER;

//
// ReceiveIndicator values (from TdiReceive and TdiReceiveDatagram requests,
// and also presented at TDI_IND_RECEIVE and TDI_IND_RECEIVE_DATAGRAM time).
//
// The TDI_RECEIVE_PARTIAL bit is no longer used at the kernel level
// interface.  TDI_RECEIVE_ENTIRE_MESSAGE has replaced it.  Providers
// may continue to set TDI_RECEIVE_PARTIAL when appropriate if they so
// desire, but the TDI_RECEIVE_ENTIRE_MESSAGE bit must be set or
// cleared as appropriate on all receive indications.
//

#define TDI_RECEIVE_BROADCAST           0x00000004 // received TSDU was broadcast.
#define TDI_RECEIVE_MULTICAST           0x00000008 // received TSDU was multicast.
#define TDI_RECEIVE_PARTIAL             0x00000010 // received TSDU is not fully presented.
#define TDI_RECEIVE_NORMAL              0x00000020 // received TSDU is normal data
#define TDI_RECEIVE_EXPEDITED           0x00000040 // received TSDU is expedited data
#define TDI_RECEIVE_PEEK                0x00000080 // received TSDU is not released
#define TDI_RECEIVE_NO_RESPONSE_EXP     0x00000100 // HINT: no back-traffic expected
#define TDI_RECEIVE_COPY_LOOKAHEAD      0x00000200 // for kernel-mode indications
#define TDI_RECEIVE_ENTIRE_MESSAGE      0x00000400 // opposite of RECEIVE_PARTIAL
                                                   //  (for kernel-mode indications)
#define TDI_RECEIVE_AT_DISPATCH_LEVEL   0x00000800 // receive indication called
                                                   //  at dispatch level
#define TDI_RECEIVE_CONTROL_INFO        0x00001000 // Control info is being passed up.
#define TDI_RECEIVE_FORCE_INDICATION    0x00002000 // reindicate rejected data.
#define TDI_RECEIVE_NO_PUSH             0x00004000 // complete only when full.



//
// Listen Flags
//

#define TDI_QUERY_ACCEPT                0x00000001     // complete TdiListen
                                                       //   without accepting
                                                       //   connection

//
// Options which are used for both SendOptions and ReceiveIndicators.
//

#define TDI_SEND_EXPEDITED            ((USHORT)0x0020) // TSDU is/was urgent/expedited.
#define TDI_SEND_PARTIAL              ((USHORT)0x0040) // TSDU is/was terminated by an EOR.
#define TDI_SEND_NO_RESPONSE_EXPECTED ((USHORT)0x0080) // HINT: no back traffic expected.
#define TDI_SEND_NON_BLOCKING         ((USHORT)0x0100) // don't block if no buffer space in protocol
#define TDI_SEND_AND_DISCONNECT       ((USHORT)0x0200) // Piggy back disconnect to remote and do not
                                                       // indicate disconnect from remote


//
// Disconnect Flags
//

#define TDI_DISCONNECT_WAIT           ((USHORT)0x0001) // used for disconnect
                                                       //   notification
#define TDI_DISCONNECT_ABORT          ((USHORT)0x0002) // immediately terminate
                                                       //   connection
#define TDI_DISCONNECT_RELEASE        ((USHORT)0x0004) // initiate graceful
                                                       //   disconnect

//
// TdiRequest structure for TdiQueryInformation request.
//

typedef struct _TDI_REQUEST_QUERY_INFORMATION {
    TDI_REQUEST Request;
    ULONG QueryType;                          // class of information to be queried.
    PTDI_CONNECTION_INFORMATION RequestConnectionInformation;
} TDI_REQUEST_QUERY_INFORMATION, *PTDI_REQUEST_QUERY_INFORMATION;

//
// TdiRequest structure for TdiSetInformation request.
//

typedef struct _TDI_REQUEST_SET_INFORMATION {
    TDI_REQUEST Request;
    ULONG SetType;                          // class of information to be set.
    PTDI_CONNECTION_INFORMATION RequestConnectionInformation;
} TDI_REQUEST_SET_INFORMATION, *PTDI_REQUEST_SET_INFORMATION;

//
// This is the old name, do not use it.
//

typedef TDI_REQUEST_SET_INFORMATION  TDI_REQ_SET_INFORMATION, *PTDI_REQ_SET_INFORMATION;

//
// Convenient universal request type.
//

typedef union _TDI_REQUEST_TYPE {
    TDI_REQUEST_ACCEPT TdiAccept;
    TDI_REQUEST_CONNECT TdiConnect;
    TDI_REQUEST_DISCONNECT TdiDisconnect;
    TDI_REQUEST_LISTEN TdiListen;
    TDI_REQUEST_QUERY_INFORMATION TdiQueryInformation;
    TDI_REQUEST_RECEIVE TdiReceive;
    TDI_REQUEST_RECEIVE_DATAGRAM TdiReceiveDatagram;
    TDI_REQUEST_SEND TdiSend;
    TDI_REQUEST_SEND_DATAGRAM TdiSendDatagram;
    TDI_REQUEST_SET_EVENT_HANDLER TdiSetEventHandler;
    TDI_REQUEST_SET_INFORMATION TdiSetInformation;
} TDI_REQUEST_TYPE, *PTDI_REQUEST_TYPE;


//
// Query information types
//

//
// Generic query info types, must be supported by all transports.
//

#define TDI_QUERY_BROADCAST_ADDRESS      0x00000001
#define TDI_QUERY_PROVIDER_INFORMATION   0x00000002   // temp, renamed ...
#define TDI_QUERY_PROVIDER_INFO          0x00000002   // ... to this
#define TDI_QUERY_ADDRESS_INFO           0x00000003
#define TDI_QUERY_CONNECTION_INFO        0x00000004
#define TDI_QUERY_PROVIDER_STATISTICS    0x00000005
#define TDI_QUERY_DATAGRAM_INFO          0x00000006
#define TDI_QUERY_DATA_LINK_ADDRESS      0x00000007
#define TDI_QUERY_NETWORK_ADDRESS        0x00000008
#define TDI_QUERY_MAX_DATAGRAM_INFO      0x00000009
#define TDI_QUERY_ROUTING_INFO           0x0000000a


//
// netbios specific query information types, must be supported by netbios
// providers. Query adapter status returns the ADAPTER_STATUS struture defined
// in the file NB30.H. Query session status returns the SESSION_HEADER/
// SESSION_BUFFER structures defined in NB30.H. Query find name returns
// the FIND_NAME_HEADER/FIND_NAME_BUFFER structures defined in NB30.H.
//

#define TDI_QUERY_ADAPTER_STATUS         0x00000100
#define TDI_QUERY_SESSION_STATUS         0x00000200
#define TDI_QUERY_FIND_NAME              0x00000300

//
// The following structures are returned by TdiQueryInformation and are read
// by TdiSetInformation. Note that a provider with netbios support will also
// return the adapter status
//

typedef struct _TDI_ENDPOINT_INFO {
    ULONG State;                        // current state of the endpoint.
    ULONG Event;                        // last event at the endpoint.
    ULONG TransmittedTsdus;             // TSDUs sent from this endpoint.
    ULONG ReceivedTsdus;                // TSDUs received at this endpoint.
    ULONG TransmissionErrors;           // TSDUs transmitted in error.
    ULONG ReceiveErrors;                // TSDUs received in error.
    ULONG MinimumLookaheadData;         // guaranteed min size of lookahead data.
    ULONG MaximumLookaheadData;         // maximum size of lookahead data.
    ULONG PriorityLevel;                // priority class assigned to outgoing data.
    ULONG SecurityLevel;                // security level assigned to outgoing data.
    ULONG SecurityCompartment;          // security compartment assigned to outgoing data.
} TDI_ENDPOINT_INFO, *PTDI_ENDPOINT_INFO;

typedef struct _TDI_CONNECTION_INFO {
    ULONG State;                        // current state of the connection.
    ULONG Event;                        // last event on the connection.
    ULONG TransmittedTsdus;             // TSDUs sent on this connection.
    ULONG ReceivedTsdus;                // TSDUs received on this connection.
    ULONG TransmissionErrors;           // TSDUs transmitted in error/this connection.
    ULONG ReceiveErrors;                // TSDUs received in error/this connection.
    LARGE_INTEGER Throughput;           // estimated throughput on this connection.
    LARGE_INTEGER Delay;                // estimated delay on this connection.
    ULONG SendBufferSize;               // size of buffer for sends - only
                                        // meaningful for internal buffering
                                        // protocols like tcp
    ULONG ReceiveBufferSize;            // size of buffer for receives - only
                                        // meaningful for internal buffering
                                        // protocols like tcp
    BOOLEAN Unreliable;                 // is this connection "unreliable".
} TDI_CONNECTION_INFO, *PTDI_CONNECTION_INFO;

typedef struct _TDI_ADDRESS_INFO {
    ULONG ActivityCount;                // outstanding open file objects/this address.
    TRANSPORT_ADDRESS Address;          // the actual address & its components.
} TDI_ADDRESS_INFO, *PTDI_ADDRESS_INFO;

typedef struct _TDI_DATAGRAM_INFO {
    ULONG MaximumDatagramBytes;
    ULONG MaximumDatagramCount;
} TDI_DATAGRAM_INFO, *PTDI_DATAGRAM_INFO;

typedef struct _TDI_MAX_DATAGRAM_INFO {
    ULONG MaxDatagramSize;              // max datagram length in bytes.
} TDI_MAX_DATAGRAM_INFO, *PTDI_MAX_DATAGRAM_INFO;

typedef struct _TDI_PROVIDER_INFO {
    ULONG Version;                      // TDI version: 0xaabb, aa=major, bb=minor
    ULONG MaxSendSize;                  // max size of user send.
    ULONG MaxConnectionUserData;        // max size of user-specified connect data.
    ULONG MaxDatagramSize;              // max datagram length in bytes.
    ULONG ServiceFlags;                 // service options, defined below.
    ULONG MinimumLookaheadData;         // guaranteed min size of lookahead data.
    ULONG MaximumLookaheadData;         // maximum size of lookahead data.
    ULONG NumberOfResources;            // how many TDI_RESOURCE_STATS for provider.
    LARGE_INTEGER StartTime;            // when the provider became active.
} TDI_PROVIDER_INFO, *PTDI_PROVIDER_INFO;

typedef struct _TDI_ROUTING_INFO_WS03 {
    ULONG Protocol;                     // protocol of the end point.
    ULONG InterfaceId;                  // interface-id of the outgoing interface.
    ULONG LinkId;                       // link-id of the outgoing link (if any).
    TRANSPORT_ADDRESS Address;          // address information of the end point.
} TDI_ROUTING_INFO_WS03, *PTDI_ROUTING_INFO_WS03;

#if (NTDDI_VERSION >= NTDDI_WS03)
typedef TDI_ROUTING_INFO_WS03 TDI_ROUTING_INFO;
typedef TDI_ROUTING_INFO* PTDI_ROUTING_INFO;
#endif //NTDDI_WINS03

#define TDI_SERVICE_CONNECTION_MODE     0x00000001 // connection mode supported.
#define TDI_SERVICE_ORDERLY_RELEASE     0x00000002 // orderly release supported.
#define TDI_SERVICE_CONNECTIONLESS_MODE 0x00000004 // connectionless mode supported.
#define TDI_SERVICE_ERROR_FREE_DELIVERY 0x00000008 // error free delivery supported.
#define TDI_SERVICE_SECURITY_LEVEL      0x00000010 // security wrapper supported.
#define TDI_SERVICE_BROADCAST_SUPPORTED 0x00000020 // broadcast datagrams supported.
#define TDI_SERVICE_MULTICAST_SUPPORTED 0x00000040 // multicast datagrams supported.
#define TDI_SERVICE_DELAYED_ACCEPTANCE  0x00000080 // use of TDI_ACCEPT_OR_REJECT is supported.
#define TDI_SERVICE_EXPEDITED_DATA      0x00000100 // expedited data supported.
#define TDI_SERVICE_INTERNAL_BUFFERING  0x00000200 // protocol does internal buffering
#define TDI_SERVICE_ROUTE_DIRECTED      0x00000400 // directed packets may go further than MC.
#define TDI_SERVICE_NO_ZERO_LENGTH      0x00000800 // zero-length sends NOT supported
#define TDI_SERVICE_POINT_TO_POINT      0x00001000 // transport is functioning as a RAS gateway
#define TDI_SERVICE_MESSAGE_MODE        0x00002000 // message-mode send supported
#define TDI_SERVICE_HALF_DUPLEX         0x00004000 // data can be received after local disc
#define TDI_SERVICE_DGRAM_CONNECTION    0x00008000 // Pseudo connection for datagrams to handle
                                                   // GPC, QOS etc.,
#define TDI_SERVICE_FORCE_ACCESS_CHECK  0x00010000 // kernel mode caller should force access
                                                   // check when opening trasnport objects
                                                   // if it passes the handle to user mode
#define TDI_SERVICE_SEND_AND_DISCONNECT 0x00020000 // combines send and disconnect processing
#define TDI_SERVICE_DIRECT_ACCEPT       0x00040000 // (deprecated) completes accept-requests directly              
#define TDI_SERVICE_ACCEPT_LOCAL_ADDR   0x00080000 // supplies local address
                                                   // with accept-completion
#define TDI_SERVICE_ADDRESS_SECURITY    0x00100000 // supports security descriptor assignment to
                                                   // addresses, extracts default SD from subject
                                                   // security context for user mode callers
#define TDI_SERVICE_PREPOST_RECVS       0x00200000 // prefers pre-posting of receives.
#define TDI_SERVICE_NO_PUSH             0x00400000 // can complete receives after
                                                   // filling the buffers completely

typedef struct _TDI_PROVIDER_RESOURCE_STATS {
    ULONG ResourceId;                   // identifies resource in question.
    ULONG MaximumResourceUsed;          // maximum number in use at once.
    ULONG AverageResourceUsed;          // average number in use.
    ULONG ResourceExhausted;            // number of times resource not available.
} TDI_PROVIDER_RESOURCE_STATS, *PTDI_PROVIDER_RESOURCE_STATS;

typedef struct _TDI_PROVIDER_STATISTICS {
    ULONG Version;                      // TDI version: 0xaabb, aa=major, bb=minor
    ULONG OpenConnections;              // currently active connections.
    ULONG ConnectionsAfterNoRetry;      // successful connections, no retries.
    ULONG ConnectionsAfterRetry;        // successful connections after retry.
    ULONG LocalDisconnects;             // connection disconnected locally.
    ULONG RemoteDisconnects;            // connection disconnected by remote.
    ULONG LinkFailures;                 // connections dropped, link failure.
    ULONG AdapterFailures;              // connections dropped, adapter failure.
    ULONG SessionTimeouts;              // connections dropped, session timeout.
    ULONG CancelledConnections;         // connect attempts cancelled.
    ULONG RemoteResourceFailures;       // connections failed, remote resource problems.
    ULONG LocalResourceFailures;        // connections failed, local resource problems.
    ULONG NotFoundFailures;             // connections failed, remote not found.
    ULONG NoListenFailures;             // connections rejected, we had no listens.
    ULONG DatagramsSent;
    LARGE_INTEGER DatagramBytesSent;
    ULONG DatagramsReceived;
    LARGE_INTEGER DatagramBytesReceived;
    ULONG PacketsSent;                  // total packets given to NDIS.
    ULONG PacketsReceived;              // total packets received from NDIS.
    ULONG DataFramesSent;
    LARGE_INTEGER DataFrameBytesSent;
    ULONG DataFramesReceived;
    LARGE_INTEGER DataFrameBytesReceived;
    ULONG DataFramesResent;
    LARGE_INTEGER DataFrameBytesResent;
    ULONG DataFramesRejected;
    LARGE_INTEGER DataFrameBytesRejected;
    ULONG ResponseTimerExpirations;     // e.g. T1 for Netbios
    ULONG AckTimerExpirations;          // e.g. T2 for Netbios
    ULONG MaximumSendWindow;            // in bytes
    ULONG AverageSendWindow;            // in bytes
    ULONG PiggybackAckQueued;           // attempts to wait to piggyback ack.
    ULONG PiggybackAckTimeouts;         // times that wait timed out.
    LARGE_INTEGER WastedPacketSpace;    // total amount of "wasted" packet space.
    ULONG WastedSpacePackets;           // how many packets contributed to that.
    ULONG NumberOfResources;            // how many TDI_RESOURCE_STATS follow.
    TDI_PROVIDER_RESOURCE_STATS ResourceStats[1];    // variable array of them.
} TDI_PROVIDER_STATISTICS, *PTDI_PROVIDER_STATISTICS;


_IRQL_requires_max_(PASSIVE_LEVEL)
_Success_(return==STATUS_SUCCESS)
_At_(Buffer,
    _Out_writes_bytes_(
        sizeof(FILE_FULL_EA_INFORMATION) + TDI_TRANSPORT_ADDRESS_LENGTH +
            sizeof(TA_NETBIOS_ADDRESS)))
NTSTATUS
TdiOpenNetbiosAddress (
    _Out_ PHANDLE FileHandle,
    PUCHAR Buffer,
    _In_ PVOID DeviceName,
    _In_opt_ PVOID Name
    );



#define IOCTL_TDI_MAGIC_BULLET          _TDI_CONTROL_CODE( 0x7f, METHOD_NEITHER )

//
// Define these to match the kernel ones for compatibility; eventually
// these will be removed.
//

typedef TDI_REQUEST_ASSOCIATE_ADDRESS TDI_REQUEST_USER_ASSOCIATE, *PTDI_REQUEST_USER_ASSOCIATE;
typedef TDI_REQUEST_CONNECT TDI_REQUEST_USER_CONNECT, *PTDI_REQUEST_USER_CONNECT;
typedef TDI_REQUEST_QUERY_INFORMATION TDI_REQUEST_USER_QUERY_INFO, *PTDI_REQUEST_USER_QUERY_INFO;

//
// The header in the OutputBuffer passed to TdiAction
//

typedef struct _TDI_ACTION_HEADER {
    ULONG   TransportId;
    USHORT  ActionCode;
    USHORT  Reserved;
} TDI_ACTION_HEADER, *PTDI_ACTION_HEADER;

typedef struct _STREAMS_TDI_ACTION {
    TDI_ACTION_HEADER Header;
    BOOLEAN DatagramOption;
    ULONG BufferLength;
    CHAR Buffer[1];
} STREAMS_TDI_ACTION, *PSTREAMS_TDI_ACTION;

// These are tags that transports pass to ndis as a first param of NdisAllocatePacketPoolEx
// api. Ndis uses this as pooltag for allocating packet pools for that transport.
#define NDIS_PACKET_POOL_TAG_FOR_NWLNKIPX 'iPDN'
#define NDIS_PACKET_POOL_TAG_FOR_NWLNKSPX 'sPDN'
#define NDIS_PACKET_POOL_TAG_FOR_NWLNKNB  'nPDN'
#define NDIS_PACKET_POOL_TAG_FOR_TCPIP    'tPDN'
#define NDIS_PACKET_POOL_TAG_FOR_NBF      'bPDN'
#define NDIS_PACKET_POOL_TAG_FOR_APPLETALK      'aPDN'


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // ndef _TDI_USER_

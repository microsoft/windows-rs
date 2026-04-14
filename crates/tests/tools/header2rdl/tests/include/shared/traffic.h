/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    traffic.h

Abstract:

    This module contains API definitions for the traffic control interface.

--*/

#ifndef __TRAFFIC_H
#define __TRAFFIC_H

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family or Traffic Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_TRAFFIC)


#ifdef __cplusplus
extern  "C" {
#endif

#include <qosobjs.h>

//---------------------------------------------------------------------------
//
// Define's
//
#define CURRENT_TCI_VERSION         0x0002


//
// Definitions of notification events. These may be passed
// to the client's notification handler, to identify the
// notification type
//

//
// A TC interface has come up
//
#define TC_NOTIFY_IFC_UP        1
//
// A TC interface has come down
//
#define TC_NOTIFY_IFC_CLOSE     2
//
// A change on a TC interface, typically a change in the
// list of supported network addresses
//
#define TC_NOTIFY_IFC_CHANGE    3
//
// A TC parameter has changed
//
#define TC_NOTIFY_PARAM_CHANGED 4
//
// A flow has been closed by the TC interface
// for example: after a remote call close, or the whole interface
// is going down
//
#define TC_NOTIFY_FLOW_CLOSE        5

#define TC_INVALID_HANDLE   ((HANDLE)0)

#define MAX_STRING_LENGTH   256


//---------------------------------------------------------------------------
//
// Typedef's and structures
//

#ifndef CALLBACK
#define CALLBACK __stdcall
#endif

#ifndef APIENTRY
#define APIENTRY FAR __stdcall
#endif

//
// Handlers registered by the TCI client
//

typedef
VOID (CALLBACK * TCI_NOTIFY_HANDLER)(
    _In_                 HANDLE      ClRegCtx,
    _In_                 HANDLE      ClIfcCtx,
    _In_                 ULONG       Event,      // See list below
    _In_                 HANDLE      SubCode,
    _In_                 ULONG       BufSize,
    _In_reads_bytes_(BufSize) PVOID       Buffer
    );

typedef
VOID (CALLBACK * TCI_ADD_FLOW_COMPLETE_HANDLER)(
    _In_ HANDLE   ClFlowCtx,
    _In_ ULONG    Status
    );

typedef
VOID (CALLBACK * TCI_MOD_FLOW_COMPLETE_HANDLER)(
    _In_ HANDLE   ClFlowCtx,
    _In_ ULONG    Status
    );

typedef
VOID (CALLBACK * TCI_DEL_FLOW_COMPLETE_HANDLER)(
    _In_ HANDLE   ClFlowCtx,
    _In_ ULONG    Status
    );


typedef struct _TCI_CLIENT_FUNC_LIST {

    TCI_NOTIFY_HANDLER              ClNotifyHandler;
    TCI_ADD_FLOW_COMPLETE_HANDLER   ClAddFlowCompleteHandler;
    TCI_MOD_FLOW_COMPLETE_HANDLER   ClModifyFlowCompleteHandler;
    TCI_DEL_FLOW_COMPLETE_HANDLER   ClDeleteFlowCompleteHandler;

} TCI_CLIENT_FUNC_LIST, *PTCI_CLIENT_FUNC_LIST;


//
// Network address descriptor
//
typedef struct _ADDRESS_LIST_DESCRIPTOR {

    ULONG                   MediaType;
    NETWORK_ADDRESS_LIST    AddressList;

} ADDRESS_LIST_DESCRIPTOR, *PADDRESS_LIST_DESCRIPTOR;


//
// An interface ID that is returned by the enumerator
//
typedef struct _TC_IFC_DESCRIPTOR {

    ULONG                   Length;
    LPWSTR                  pInterfaceName;
    LPWSTR                  pInterfaceID;
    ADDRESS_LIST_DESCRIPTOR AddressListDesc;

} TC_IFC_DESCRIPTOR, *PTC_IFC_DESCRIPTOR;


//
// This structure is returned by a QoS data provider in reply to
// GUID_QOS_SUPPORTED query or with an interface UP notification
//
typedef struct _TC_SUPPORTED_INFO_BUFFER {

    USHORT                      InstanceIDLength;

    // device or interface ID
    WCHAR                       InstanceID[MAX_STRING_LENGTH];

    // the NET_LUID for the interface
    ULONG64                     InterfaceLuid; 

    // address list
    ADDRESS_LIST_DESCRIPTOR     AddrListDesc;

} TC_SUPPORTED_INFO_BUFFER, *PTC_SUPPORTED_INFO_BUFFER;


//
// Filters are used to match packets. The Pattern field
// indicates the values to which bits in corresponding
// positions in candidate packets should be compared. The
// Mask field indicates which bits are to be compared and
// which bits are don't cares.
//
// Different filters can be submitted on the TCI interface.
// The generic filter structure is defined to include an
// AddressType, which indicates the specific type of filter to
// follow.
//

typedef struct _TC_GEN_FILTER {

    USHORT  AddressType;    // IP, IPX, etc.
    ULONG   PatternSize;    // byte count of the pattern
    _Field_size_bytes_(PatternSize) PVOID   Pattern;        // specific format, e.g. IP_PATTERN
    _Field_size_bytes_(PatternSize) PVOID   Mask;           // same type as Pattern

} TC_GEN_FILTER, *PTC_GEN_FILTER;


//
// A generic flow includes two flowspecs and a freeform
// buffer which contains flow specific TC objects.
//
typedef struct _TC_GEN_FLOW {
    
    FLOWSPEC        SendingFlowspec;
    FLOWSPEC        ReceivingFlowspec;
    ULONG           TcObjectsLength;        // number of optional bytes
    QOS_OBJECT_HDR  TcObjects[1];

} TC_GEN_FLOW, *PTC_GEN_FLOW;


//
// Format of specific pattern or mask used by GPC for the IP protocol
//
typedef struct _IP_PATTERN {

    ULONG       Reserved1;
    ULONG       Reserved2;

    ULONG       SrcAddr;
    ULONG       DstAddr;

    union {
        struct { USHORT s_srcport,s_dstport; } S_un_ports;
        struct { UCHAR s_type,s_code; USHORT filler; } S_un_icmp;
        ULONG   S_Spi;
    } S_un;

    UCHAR       ProtocolId;
    UCHAR       Reserved3[3];

#define tcSrcPort S_un.S_un_ports.s_srcport
#define tcDstPort S_un.S_un_ports.s_dstport
#define tcIcmpType        S_un.S_un_icmp.s_type
#define tcIcmpCode        S_un.S_un_icmp.s_code
#define tcSpi             S_un.S_Spi

} IP_PATTERN, *PIP_PATTERN;


//
// Format of specific pattern or mask used by GPC for the IPX protocol
//
typedef struct _IPX_PATTERN {

    struct {
        ULONG   NetworkAddress;
        UCHAR   NodeAddress[6];
        USHORT  Socket;
    } Src, Dest;

} IPX_PATTERN, *PIPX_PATTERN;


//
// The enumeration buffer is the flow parameters + a list of filters
//
typedef struct _ENUMERATION_BUFFER {

    ULONG           Length;
    ULONG           OwnerProcessId;
    USHORT          FlowNameLength;
    WCHAR           FlowName[MAX_STRING_LENGTH];
    PTC_GEN_FLOW    pFlow;
    ULONG           NumberOfFilters;
    TC_GEN_FILTER   GenericFilter[1];   // one for each filter

} ENUMERATION_BUFFER, *PENUMERATION_BUFFER;


//---------------------------------------------------------------------------
//
// Interface Function Definitions
//

ULONG
APIENTRY
TcRegisterClient(
    _In_  ULONG                   TciVersion,
    _In_  HANDLE                  ClRegCtx,
    _In_  PTCI_CLIENT_FUNC_LIST   ClientHandlerList,
    _Out_ PHANDLE                 pClientHandle
    );

ULONG
APIENTRY
TcEnumerateInterfaces(
    _In_    HANDLE              ClientHandle,
    _Inout_ PULONG              pBufferSize,
    _Out_   PTC_IFC_DESCRIPTOR  InterfaceBuffer
    );

ULONG
APIENTRY
TcOpenInterfaceA(
    _In_  LPSTR           pInterfaceName,
    _In_  HANDLE          ClientHandle,
    _In_  HANDLE          ClIfcCtx,
    _Out_ PHANDLE         pIfcHandle
    );

ULONG
APIENTRY
TcOpenInterfaceW(
    _In_  LPWSTR          pInterfaceName,
    _In_  HANDLE          ClientHandle,
    _In_  HANDLE          ClIfcCtx,
    _Out_ PHANDLE         pIfcHandle
    );

ULONG
APIENTRY
TcCloseInterface(
    _In_ HANDLE       IfcHandle
    );

ULONG
APIENTRY
TcQueryInterface(
    _In_                       HANDLE      IfcHandle,
    _In_                       LPGUID      pGuidParam,
    _In_                       BOOLEAN     NotifyChange,
    _Inout_                    PULONG      pBufferSize,
    _Out_writes_bytes_(*pBufferSize) PVOID       Buffer
    );

ULONG
APIENTRY
TcSetInterface(
    _In_                    HANDLE      IfcHandle,
    _In_                    LPGUID      pGuidParam,
    _In_                    ULONG       BufferSize,
    _In_reads_bytes_(BufferSize) PVOID       Buffer
    );

ULONG
APIENTRY
TcQueryFlowA(
    _In_                       LPSTR       pFlowName,
    _In_                       LPGUID      pGuidParam,
    _Inout_                    PULONG      pBufferSize,
    _Out_writes_bytes_(*pBufferSize) PVOID       Buffer
    );

ULONG
APIENTRY
TcQueryFlowW(
    _In_                       LPWSTR      pFlowName,
    _In_                       LPGUID      pGuidParam,
    _Inout_                    PULONG      pBufferSize,
    _Out_writes_bytes_(*pBufferSize) PVOID       Buffer
    );

ULONG
APIENTRY
TcSetFlowA(
    _In_                    LPSTR       pFlowName,
    _In_                    LPGUID      pGuidParam,
    _In_                    ULONG       BufferSize,
    _In_reads_bytes_(BufferSize) PVOID       Buffer
    );

ULONG
APIENTRY
TcSetFlowW(
    _In_                    LPWSTR      pFlowName,
    _In_                    LPGUID      pGuidParam,
    _In_                    ULONG       BufferSize,
    _In_reads_bytes_(BufferSize) PVOID       Buffer
    );

ULONG
APIENTRY
TcAddFlow(
    _In_  HANDLE          IfcHandle,
    _In_  HANDLE          ClFlowCtx,
    _In_  ULONG           Flags,
    _In_  PTC_GEN_FLOW    pGenericFlow,
    _Out_ PHANDLE         pFlowHandle
    );

ULONG
APIENTRY
TcGetFlowNameA(
    _In_                  HANDLE  FlowHandle,
    _In_                  ULONG   StrSize,
    _Out_writes_(StrSize) LPSTR   pFlowName
    );

ULONG
APIENTRY
TcGetFlowNameW(
    _In_                  HANDLE  FlowHandle,
    _In_                  ULONG   StrSize,
    _Out_writes_(StrSize) LPWSTR  pFlowName
    );

ULONG
APIENTRY
TcModifyFlow(   
    _In_ HANDLE          FlowHandle,
    _In_ PTC_GEN_FLOW    pGenericFlow
    );

ULONG
APIENTRY
TcAddFilter(
    _In_  HANDLE          FlowHandle,
    _In_  PTC_GEN_FILTER  pGenericFilter,
    _Out_ PHANDLE         pFilterHandle
    );

ULONG
APIENTRY
TcDeregisterClient(
    _In_ HANDLE          ClientHandle
    );

ULONG
APIENTRY
TcDeleteFlow(
    _In_ HANDLE          FlowHandle
    );

ULONG
APIENTRY
TcDeleteFilter(
    _In_ HANDLE          FilterHandle
    );

ULONG
APIENTRY
TcEnumerateFlows(   
    _In_    HANDLE                  IfcHandle,
    _Inout_ PHANDLE                 pEnumHandle,
    _Inout_ PULONG                  pFlowCount,
    _Inout_ PULONG                  pBufSize,
    _Out_   PENUMERATION_BUFFER     Buffer
    );

#ifdef UNICODE

#define TcOpenInterface                 TcOpenInterfaceW
#define TcQueryFlow                     TcQueryFlowW
#define TcSetFlow                       TcSetFlowW
#define TcGetFlowName                   TcGetFlowNameW

#else   // UNICODE

#define TcOpenInterface     TcOpenInterfaceA
#define TcQueryFlow         TcQueryFlowA
#define TcSetFlow           TcSetFlowA
#define TcGetFlowName       TcGetFlowNameA

#endif  // UNICODE


#ifdef __cplusplus
}
#endif



#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_TRAFFIC) */
#pragma endregion

#endif


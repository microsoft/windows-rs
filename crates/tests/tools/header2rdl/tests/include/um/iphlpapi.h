/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    iphlpapi.h

Abstract:
    Header file for functions to interact with the IP Stack for MIB-II and
    related functionality

--*/

#ifndef __IPHLPAPI_H__
#define __IPHLPAPI_H__

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// Establish DLL function linkage if supported by the current build         //
// environment and not previously defined.                                  //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////

#ifndef IPHLPAPI_DLL_LINKAGE
#ifdef DECLSPEC_IMPORT
#define IPHLPAPI_DLL_LINKAGE DECLSPEC_IMPORT
#else
#define IPHLPAPI_DLL_LINKAGE
#endif
#endif

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

#ifdef __cplusplus
extern "C" {
#endif

//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// IPRTRMIB.H has the definitions of the structures used to set and get     //
// information                                                              //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////

#include <iprtrmib.h>
#include <ipexport.h>
#include <iptypes.h>
#include <tcpestats.h>

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// The GetXXXTable APIs take a buffer and a size of buffer.  If the buffer  //
// is not large enough, the APIs return ERROR_INSUFFICIENT_BUFFER  and      //
// *pdwSize is the required buffer size                                     //
// The bOrder is a BOOLEAN, which if TRUE sorts the table according to      //
// MIB-II (RFC XXXX)                                                        //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////


//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// Retrieves the number of interfaces in the system. These include LAN and  //
// WAN interfaces                                                           //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////

IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
GetNumberOfInterfaces(
    _Out_ PDWORD  pdwNumIf
    );

//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// Gets the MIB-II ifEntry                                                  //
// The dwIndex field of the MIB_IFROW should be set to the index of the     //
// interface being queried                                                  //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////

IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
GetIfEntry(
    _Inout_ PMIB_IFROW   pIfRow
    );

//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// Gets the MIB-II IfTable                                                  //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////

IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
GetIfTable(
    _Out_writes_bytes_opt_(*pdwSize) PMIB_IFTABLE pIfTable,
    _Inout_                    PULONG       pdwSize,
    _In_                       BOOL         bOrder
    );

//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// Gets the Interface to IP Address mapping                                 //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////

IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
GetIpAddrTable(
    _Out_writes_bytes_opt_(*pdwSize)    PMIB_IPADDRTABLE pIpAddrTable,
    _Inout_                       PULONG           pdwSize,
    _In_                          BOOL             bOrder
    );

//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// Gets the current IP Address to Physical Address (ARP) mapping            //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////

IPHLPAPI_DLL_LINKAGE
ULONG
WINAPI
GetIpNetTable(
    _Out_writes_bytes_opt_(*SizePointer) PMIB_IPNETTABLE IpNetTable,
    _Inout_                        PULONG          SizePointer,
    _In_                           BOOL            Order
    );

//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// Gets the IP Routing Table  (RFX XXXX)                                    //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////

IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
GetIpForwardTable(
    _Out_writes_bytes_opt_(*pdwSize)    PMIB_IPFORWARDTABLE pIpForwardTable,
    _Inout_                       PULONG              pdwSize,
    _In_                          BOOL                bOrder
    );

//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// Gets TCP Connection/UDP Listener Table                                   //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

IPHLPAPI_DLL_LINKAGE
ULONG
WINAPI
GetTcpTable(
    _Out_writes_bytes_opt_(*SizePointer)   PMIB_TCPTABLE TcpTable,
    _Inout_                          PULONG        SizePointer,
    _In_                             BOOL          Order
    );

IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
GetExtendedTcpTable(
    _Out_writes_bytes_opt_(*pdwSize)        PVOID           pTcpTable,
    _Inout_
    _When_(ulAf == AF_INET, _Deref_in_range_(>=, sizeof(MIB_TCPTABLE)))
    _When_(
        (TableClass == TCP_TABLE_OWNER_PID_LISTENER ||
         TableClass == TCP_TABLE_OWNER_PID_CONNECTIONS ||
         TableClass == TCP_TABLE_OWNER_PID_ALL) && ulAf == AF_INET6,
        _Deref_in_range_(>=, sizeof(MIB_TCP6TABLE_OWNER_PID)))
    _When_(
        (TableClass == TCP_TABLE_OWNER_MODULE_LISTENER ||
         TableClass == TCP_TABLE_OWNER_MODULE_CONNECTIONS ||
         TableClass == TCP_TABLE_OWNER_MODULE_ALL) && ulAf == AF_INET6,
        _Deref_in_range_(>=, sizeof(MIB_TCP6TABLE_OWNER_MODULE)))
    PDWORD          pdwSize,
    _In_                          BOOL            bOrder,
    _In_                          ULONG           ulAf,
    _In_                          TCP_TABLE_CLASS TableClass,
    _In_                          ULONG           Reserved
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
GetOwnerModuleFromTcpEntry(
    _In_                   PMIB_TCPROW_OWNER_MODULE      pTcpEntry,
    _In_                   TCPIP_OWNER_MODULE_INFO_CLASS Class,
    _Out_writes_bytes_(*pdwSize) PVOID                         pBuffer,
    _Inout_                PDWORD                        pdwSize
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

IPHLPAPI_DLL_LINKAGE
ULONG
WINAPI
GetUdpTable(
    _Out_writes_bytes_opt_(*SizePointer)   PMIB_UDPTABLE UdpTable,
    _Inout_                          PULONG        SizePointer,
    _In_                             BOOL          Order
    );

IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
GetExtendedUdpTable(
    _Out_writes_bytes_opt_(*pdwSize)    PVOID           pUdpTable,
    _Inout_                       PDWORD          pdwSize,
    _In_                          BOOL            bOrder,
    _In_                          ULONG           ulAf,
    _In_                          UDP_TABLE_CLASS TableClass,
    _In_                          ULONG           Reserved
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
GetOwnerModuleFromUdpEntry(
    _In_                      PMIB_UDPROW_OWNER_MODULE      pUdpEntry,
    _In_                      TCPIP_OWNER_MODULE_INFO_CLASS Class,
    _Out_writes_bytes_(*pdwSize)    PVOID                         pBuffer,
    _Inout_                   PDWORD                        pdwSize
    );

#if (NTDDI_VERSION >= NTDDI_VISTA)
IPHLPAPI_DLL_LINKAGE
ULONG
WINAPI
GetTcpTable2(
    _Out_writes_bytes_opt_(*SizePointer)   PMIB_TCPTABLE2 TcpTable,
    _Inout_                          PULONG         SizePointer,
    _In_                             BOOL           Order
    );
#endif // (NTDDI_VERSION >= NTDDI_VISTA)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#if (NTDDI_VERSION < NTDDI_VISTA)
//
// Deprecated APIs, Added for documentation.
//

DWORD
AllocateAndGetTcpExTableFromStack(
    _Outptr_ PVOID         *ppTcpTable,
    _In_        BOOL          bOrder,
    _In_        HANDLE        hHeap,
    _In_        DWORD         dwFlags,
    _In_        DWORD         dwFamily
    );

DWORD
AllocateAndGetUdpExTableFromStack(
    _Outptr_ PVOID         *ppUdpTable,
    _In_        BOOL          bOrder,
    _In_        HANDLE        hHeap,
    _In_        DWORD         dwFlags,
    _In_        DWORD         dwFamily
    );

#endif // (NTDDI_VERSION < NTDDI_VISTA)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#pragma region Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

#ifdef _WS2IPDEF_
//
// The following definitions require Winsock2.
//

#if (NTDDI_VERSION >= NTDDI_VISTA)
IPHLPAPI_DLL_LINKAGE
ULONG
WINAPI
GetTcp6Table(
    _Out_writes_bytes_(*SizePointer)   PMIB_TCP6TABLE TcpTable,
    _Inout_                      PULONG         SizePointer,
    _In_                         BOOL           Order
    );

IPHLPAPI_DLL_LINKAGE
ULONG
WINAPI
GetTcp6Table2(
    _Out_writes_bytes_(*SizePointer)   PMIB_TCP6TABLE2 TcpTable,
    _Inout_                      PULONG          SizePointer,
    _In_                         BOOL            Order
    );

#endif // (NTDDI_VERSION >= NTDDI_VISTA)

#ifdef WINAPI

IPHLPAPI_DLL_LINKAGE
ULONG
WINAPI
GetPerTcpConnectionEStats(
    _In_ PMIB_TCPROW Row,
    _In_ TCP_ESTATS_TYPE EstatsType,
    _Out_writes_bytes_opt_(RwSize) PUCHAR Rw,
    _In_ ULONG RwVersion,
    _In_ ULONG RwSize,
    _Out_writes_bytes_opt_(RosSize) PUCHAR Ros,
    _In_ ULONG RosVersion,
    _In_ ULONG RosSize,
    _Out_writes_bytes_opt_(RodSize) PUCHAR Rod,
    _In_ ULONG RodVersion,
    _In_ ULONG RodSize
    );

IPHLPAPI_DLL_LINKAGE
ULONG
WINAPI
SetPerTcpConnectionEStats(
    _In_ PMIB_TCPROW Row,
    _In_ TCP_ESTATS_TYPE EstatsType,
    _In_reads_bytes_(RwSize) PUCHAR Rw,
    _In_ ULONG RwVersion,
    _In_ ULONG RwSize,
    _In_ ULONG Offset
    );

#ifdef _WS2IPDEF_

IPHLPAPI_DLL_LINKAGE
ULONG
WINAPI
GetPerTcp6ConnectionEStats(
    _In_ PMIB_TCP6ROW Row,
    _In_ TCP_ESTATS_TYPE EstatsType,
    _Out_writes_bytes_opt_(RwSize) PUCHAR Rw,
    _In_  ULONG RwVersion,
    _In_  ULONG RwSize,
    _Out_writes_bytes_opt_(RosSize) PUCHAR Ros,
    _In_  ULONG RosVersion,
    _In_  ULONG RosSize,
    _Out_writes_bytes_opt_(RodSize) PUCHAR Rod,
    _In_  ULONG RodVersion,
    _In_  ULONG RodSize
    );

IPHLPAPI_DLL_LINKAGE
ULONG
WINAPI
SetPerTcp6ConnectionEStats(
    _In_ PMIB_TCP6ROW Row,
    _In_ TCP_ESTATS_TYPE EstatsType,
    _In_reads_bytes_(RwSize) PUCHAR Rw,
    _In_ ULONG RwVersion,
    _In_ ULONG RwSize,
    _In_ ULONG Offset
    );

#endif // _WS2IPDEF_

#endif // WINAPI

IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
GetOwnerModuleFromTcp6Entry(
    _In_                      PMIB_TCP6ROW_OWNER_MODULE     pTcpEntry,
    _In_                      TCPIP_OWNER_MODULE_INFO_CLASS Class,
    _Out_writes_bytes_(*pdwSize)    PVOID                         pBuffer,
    _Inout_                   PDWORD                        pdwSize
    );

IPHLPAPI_DLL_LINKAGE
ULONG
WINAPI
GetUdp6Table(
    _Out_writes_bytes_opt_(*SizePointer)   PMIB_UDP6TABLE Udp6Table,
    _Inout_                      PULONG         SizePointer,
    _In_                         BOOL           Order
    );

IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
GetOwnerModuleFromUdp6Entry(
    _In_                      PMIB_UDP6ROW_OWNER_MODULE     pUdpEntry,
    _In_                      TCPIP_OWNER_MODULE_INFO_CLASS Class,
    _Out_writes_bytes_(*pdwSize)    PVOID                         pBuffer,
    _Inout_                   PDWORD                        pdwSize
    );

#endif // _WS2IPDEF_

//
// Because this function isn't marked with WINAPI, it is not marked with
// IPHLPAPI_DLL_LINKAGE in order to prevent build breaks with managed projects.
//
DWORD
GetOwnerModuleFromPidAndInfo(
    _In_                     ULONG                         ulPid,
    _In_                     ULONGLONG                     *pInfo,
    _In_                     TCPIP_OWNER_MODULE_INFO_CLASS Class,
    _Out_writes_bytes_(*pdwSize)   PVOID                         pBuffer,
    _Inout_                  PDWORD                        pdwSize
    );

//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// Gets IP/ICMP/TCP/UDP Statistics                                          //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////

#if (NTDDI_VERSION >= NTDDI_WIN2K)
IPHLPAPI_DLL_LINKAGE
ULONG
WINAPI
GetIpStatistics(
    _Out_ PMIB_IPSTATS Statistics
    );
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

#if (NTDDI_VERSION >= NTDDI_WIN2K)
IPHLPAPI_DLL_LINKAGE
ULONG
WINAPI
GetIcmpStatistics(
    _Out_ PMIB_ICMP Statistics
    );
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

#if (NTDDI_VERSION >= NTDDI_WIN2K)
IPHLPAPI_DLL_LINKAGE
ULONG
WINAPI
GetTcpStatistics(
    _Out_ PMIB_TCPSTATS Statistics
    );

IPHLPAPI_DLL_LINKAGE
ULONG
WINAPI
GetUdpStatistics(
    _Out_ PMIB_UDPSTATS Stats
    );
#endif

#if (NTDDI_VERSION >= NTDDI_WINXP)
IPHLPAPI_DLL_LINKAGE
ULONG
WINAPI
SetIpStatisticsEx(
    _In_ PMIB_IPSTATS Statistics,
    _In_ ULONG Family
    );
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

#if (NTDDI_VERSION >= NTDDI_WINXP)
IPHLPAPI_DLL_LINKAGE
ULONG
WINAPI
GetIpStatisticsEx(
    _Out_ PMIB_IPSTATS Statistics,
    _In_  ULONG Family
    );

IPHLPAPI_DLL_LINKAGE
ULONG
WINAPI
GetIcmpStatisticsEx(
    _Out_ PMIB_ICMP_EX Statistics,
    _In_  ULONG Family
    );

IPHLPAPI_DLL_LINKAGE
ULONG
WINAPI
GetTcpStatisticsEx(
    _Out_ PMIB_TCPSTATS Statistics,
    _In_  ULONG Family
    );

IPHLPAPI_DLL_LINKAGE
ULONG
WINAPI
GetUdpStatisticsEx(
    _Out_ PMIB_UDPSTATS Statistics,
    _In_  ULONG Family
    );
#endif

#if (NTDDI_VERSION >= NTDDI_WIN10_RS3)
IPHLPAPI_DLL_LINKAGE
ULONG
WINAPI
GetTcpStatisticsEx2(
    _Out_ PMIB_TCPSTATS2 Statistics,
    _In_  ULONG Family
    );

IPHLPAPI_DLL_LINKAGE
ULONG
WINAPI
GetUdpStatisticsEx2(
    _Out_ PMIB_UDPSTATS2 Statistics,
    _In_  ULONG Family
    );
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// Used to set the ifAdminStatus on an interface.  The only fields of the   //
// MIB_IFROW that are relevant are the dwIndex (index of the interface      //
// whose status needs to be set) and the dwAdminStatus which can be either  //
// MIB_IF_ADMIN_STATUS_UP or MIB_IF_ADMIN_STATUS_DOWN                       //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////

IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
SetIfEntry(
    _In_ PMIB_IFROW pIfRow
    );

//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// Used to create, modify or delete a route.  In all cases the              //
// dwForwardIfIndex, dwForwardDest, dwForwardMask, dwForwardNextHop and     //
// dwForwardPolicy MUST BE SPECIFIED. Currently dwForwardPolicy is unused   //
// and MUST BE 0.                                                           //
// For a set, the complete MIB_IPFORWARDROW structure must be specified     //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////

IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
CreateIpForwardEntry(
    _In_ PMIB_IPFORWARDROW pRoute
    );

IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
SetIpForwardEntry(
    _In_ PMIB_IPFORWARDROW pRoute
    );

IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
DeleteIpForwardEntry(
    _In_ PMIB_IPFORWARDROW pRoute
    );

//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// Used to set the ipForwarding to ON or OFF (currently only ON->OFF is     //
// allowed) and to set the defaultTTL.  If only one of the fields needs to  //
// be modified and the other needs to be the same as before the other field //
// needs to be set to MIB_USE_CURRENT_TTL or MIB_USE_CURRENT_FORWARDING as  //
// the case may be                                                          //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////

#if (NTDDI_VERSION >= NTDDI_WIN2K)
IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
SetIpStatistics(
    _In_ PMIB_IPSTATS pIpStats
    );
#endif

//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// Used to set the defaultTTL.                                              //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////

IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
SetIpTTL(
    _In_ UINT nTTL
    );

//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// Used to create, modify or delete an ARP entry.  In all cases the dwIndex //
// dwAddr field MUST BE SPECIFIED.                                          //
// For a set, the complete MIB_IPNETROW structure must be specified         //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////

IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
CreateIpNetEntry(
    _In_ PMIB_IPNETROW    pArpEntry
    );

IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
SetIpNetEntry(
    _In_ PMIB_IPNETROW    pArpEntry
    );

IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
DeleteIpNetEntry(
    _In_ PMIB_IPNETROW    pArpEntry
    );

IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
FlushIpNetTable(
    _In_ DWORD   dwIfIndex
    );


//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// Used to create or delete a Proxy ARP entry. The dwIndex is the index of  //
// the interface on which to PARP for the dwAddress.  If the interface is   //
// of a type that doesnt support ARP, e.g. PPP, then the call will fail     //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////

IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
CreateProxyArpEntry(
    _In_  DWORD   dwAddress,
    _In_  DWORD   dwMask,
    _In_  DWORD   dwIfIndex
    );

IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
DeleteProxyArpEntry(
    _In_  DWORD   dwAddress,
    _In_  DWORD   dwMask,
    _In_  DWORD   dwIfIndex
    );

//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// Used to set the state of a TCP Connection. The only state that it can be //
// set to is MIB_TCP_STATE_DELETE_TCB.  The complete MIB_TCPROW structure   //
// MUST BE SPECIFIED                                                        //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////

IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
SetTcpEntry(
    _In_ PMIB_TCPROW pTcpRow
    );

IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
GetInterfaceInfo(
    _Out_writes_bytes_opt_(*dwOutBufLen) PIP_INTERFACE_INFO  pIfTable,
    _Inout_                        PULONG              dwOutBufLen
    );

IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
GetUniDirectionalAdapterInfo(
    _Out_writes_bytes_opt_(*dwOutBufLen) PIP_UNIDIRECTIONAL_ADAPTER_ADDRESS pIPIfInfo,
    _Inout_                        PULONG                             dwOutBufLen
    );

#if (NTDDI_VERSION >= NTDDI_WIN2KSP1)
#ifndef NHPALLOCATEANDGETINTERFACEINFOFROMSTACK_DEFINED
#define NHPALLOCATEANDGETINTERFACEINFOFROMSTACK_DEFINED

IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
NhpAllocateAndGetInterfaceInfoFromStack(
    _Outptr_ IP_INTERFACE_NAME_INFO  **ppTable,
    _Out_       PDWORD                  pdwCount,
    _In_        BOOL                    bOrder,
    _In_        HANDLE                  hHeap,
    _In_        DWORD                   dwFlags
    );

#endif
#endif // (NTDDI_VERSION >= NTDDI_WIN2KSP1)

//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// Gets the "best" outgoing interface for the specified destination address //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////

IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
GetBestInterface(
    _In_  IPAddr  dwDestAddr,
    _Out_ PDWORD  pdwBestIfIndex
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

#pragma warning(push)
#pragma warning(disable:4115)
IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
GetBestInterfaceEx(
    _In_  struct sockaddr *pDestAddr,
    _Out_ PDWORD           pdwBestIfIndex
    );
#pragma warning(pop)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

//////////////////////////////////////////////////////////////////////////////
//                                                                          //
// Gets the best (longest matching prefix) route for the given destination  //
// If the source address is also specified (i.e. is not 0x00000000), and    //
// there are multiple "best" routes to the given destination, the returned  //
// route will be one that goes out over the interface which has an address  //
// that matches the source address                                          //
//                                                                          //
//////////////////////////////////////////////////////////////////////////////

IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
GetBestRoute(
    _In_      DWORD               dwDestAddr,
    _In_opt_  DWORD               dwSourceAddr,
    _Out_     PMIB_IPFORWARDROW   pBestRoute
    );

IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
NotifyAddrChange(
    _Out_ PHANDLE      Handle,
    _In_  LPOVERLAPPED overlapped
    );

IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
NotifyRouteChange(
    _Out_ PHANDLE      Handle,
    _In_  LPOVERLAPPED overlapped
    );

IPHLPAPI_DLL_LINKAGE
BOOL
WINAPI
CancelIPChangeNotify(
    _In_  LPOVERLAPPED notifyOverlapped
    );

IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
GetAdapterIndex(
    _In_    LPWSTR  AdapterName,
    _Inout_ PULONG IfIndex
    );

IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
AddIPAddress(
    _In_ IPAddr  Address,
    _In_ IPMask  IpMask,
    _In_ DWORD   IfIndex,
    _Out_ PULONG  NTEContext,
    _Out_ PULONG  NTEInstance
    );

IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
DeleteIPAddress(
    _In_ ULONG NTEContext
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

#if (NTDDI_VERSION >= NTDDI_WIN2KSP1)
IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
GetNetworkParams(
    _Out_writes_bytes_opt_(*pOutBufLen)   PFIXED_INFO pFixedInfo,
    _Inout_                         PULONG      pOutBufLen
    );
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

IPHLPAPI_DLL_LINKAGE
ULONG
WINAPI
GetAdaptersInfo(
    _Out_writes_bytes_opt_(*SizePointer) PIP_ADAPTER_INFO AdapterInfo,
    _Inout_                         PULONG           SizePointer
    );

IPHLPAPI_DLL_LINKAGE
PIP_ADAPTER_ORDER_MAP
WINAPI
GetAdapterOrderMap(
    VOID
    );


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

#ifdef _WINSOCK2API_

//
// The following functions require Winsock2.
//

IPHLPAPI_DLL_LINKAGE
ULONG
WINAPI
GetAdaptersAddresses(
    _In_ ULONG Family,
    _In_ ULONG Flags,
    _Reserved_ PVOID Reserved,
    _Out_writes_bytes_opt_(*SizePointer) PIP_ADAPTER_ADDRESSES AdapterAddresses,
    _Inout_ PULONG SizePointer
    );

#endif

#if (NTDDI_VERSION >= NTDDI_WIN2KSP1)
IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
GetPerAdapterInfo(
    _In_                            ULONG                IfIndex,
    _Out_writes_bytes_opt_(*pOutBufLen)   PIP_PER_ADAPTER_INFO pPerAdapterInfo,
    _Inout_                         PULONG               pOutBufLen
    );
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion


#pragma region Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

#if (NTDDI_VERSION >= NTDDI_WIN10_FE)

typedef struct _INTERFACE_HARDWARE_TIMESTAMP_CAPABILITIES
{
    BOOLEAN PtpV2OverUdpIPv4EventMessageReceive;
    BOOLEAN PtpV2OverUdpIPv4AllMessageReceive;
    BOOLEAN PtpV2OverUdpIPv4EventMessageTransmit;
    BOOLEAN PtpV2OverUdpIPv4AllMessageTransmit;
    BOOLEAN PtpV2OverUdpIPv6EventMessageReceive;
    BOOLEAN PtpV2OverUdpIPv6AllMessageReceive;
    BOOLEAN PtpV2OverUdpIPv6EventMessageTransmit;
    BOOLEAN PtpV2OverUdpIPv6AllMessageTransmit;
    BOOLEAN AllReceive;
    BOOLEAN AllTransmit;
    BOOLEAN TaggedTransmit;
} INTERFACE_HARDWARE_TIMESTAMP_CAPABILITIES, *PINTERFACE_HARDWARE_TIMESTAMP_CAPABILITIES;

typedef struct _INTERFACE_SOFTWARE_TIMESTAMP_CAPABILITIES
{
    BOOLEAN AllReceive;
    BOOLEAN AllTransmit;
    BOOLEAN TaggedTransmit;
} INTERFACE_SOFTWARE_TIMESTAMP_CAPABILITIES, *PINTERFACE_SOFTWARE_TIMESTAMP_CAPABILITIES;

typedef struct _INTERFACE_TIMESTAMP_CAPABILITIES
{
    ULONG64 HardwareClockFrequencyHz;
    BOOLEAN SupportsCrossTimestamp;
    INTERFACE_HARDWARE_TIMESTAMP_CAPABILITIES HardwareCapabilities;
    INTERFACE_SOFTWARE_TIMESTAMP_CAPABILITIES SoftwareCapabilities;
} INTERFACE_TIMESTAMP_CAPABILITIES, *PINTERFACE_TIMESTAMP_CAPABILITIES;

typedef struct _INTERFACE_HARDWARE_CROSSTIMESTAMP
{
    ULONG64 SystemTimestamp1;
    ULONG64 HardwareClockTimestamp;
    ULONG64 SystemTimestamp2;
} INTERFACE_HARDWARE_CROSSTIMESTAMP, *PINTERFACE_HARDWARE_CROSSTIMESTAMP;

DECLARE_HANDLE(HIFTIMESTAMPCHANGE);

IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
GetInterfaceActiveTimestampCapabilities(
    _In_ CONST NET_LUID *InterfaceLuid,
    _Out_ PINTERFACE_TIMESTAMP_CAPABILITIES TimestampCapabilites
    );

IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
GetInterfaceSupportedTimestampCapabilities(
    _In_ CONST NET_LUID *InterfaceLuid,
    _Out_ PINTERFACE_TIMESTAMP_CAPABILITIES TimestampCapabilites
    );

IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
CaptureInterfaceHardwareCrossTimestamp(
    _In_ CONST NET_LUID *InterfaceLuid,
    _Inout_ PINTERFACE_HARDWARE_CROSSTIMESTAMP CrossTimestamp
    );

typedef
VOID
CALLBACK
INTERFACE_TIMESTAMP_CONFIG_CHANGE_CALLBACK(
    _In_ PVOID CallerContext
    );

typedef
INTERFACE_TIMESTAMP_CONFIG_CHANGE_CALLBACK *PINTERFACE_TIMESTAMP_CONFIG_CHANGE_CALLBACK;

IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
RegisterInterfaceTimestampConfigChange(
    _In_ PINTERFACE_TIMESTAMP_CONFIG_CHANGE_CALLBACK Callback,
    _In_opt_ PVOID CallerContext,
    _Out_ HIFTIMESTAMPCHANGE *NotificationHandle
    );

IPHLPAPI_DLL_LINKAGE
VOID
WINAPI
UnregisterInterfaceTimestampConfigChange(
    _In_ HIFTIMESTAMPCHANGE NotificationHandle
    );

IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
GetInterfaceCurrentTimestampCapabilities(
    _In_ CONST NET_LUID *InterfaceLuid,
    _Inout_ PINTERFACE_TIMESTAMP_CAPABILITIES TimestampCapabilites
    );

IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
GetInterfaceHardwareTimestampCapabilities(
    _In_ CONST NET_LUID *InterfaceLuid,
    _Inout_ PINTERFACE_TIMESTAMP_CAPABILITIES TimestampCapabilites
    );

IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
NotifyIfTimestampConfigChange(
    _In_opt_ PVOID CallerContext,
    _In_ PINTERFACE_TIMESTAMP_CONFIG_CHANGE_CALLBACK Callback,
    _Out_ HIFTIMESTAMPCHANGE *NotificationHandle
    );

IPHLPAPI_DLL_LINKAGE
VOID
WINAPI
CancelIfTimestampConfigChange(
    _In_ HIFTIMESTAMPCHANGE NotificationHandle
    );

#elif (NTDDI_VERSION >= NTDDI_WIN10_RS5)

#define INTERFACE_TIMESTAMP_CAPABILITIES_VERSION_1 0x01
#define INTERFACE_HARDWARE_CROSSTIMESTAMP_VERSION_1 0x01

typedef struct _INTERFACE_TIMESTAMP_CAPABILITY_FLAGS
{
    BOOLEAN PtpV2OverUdpIPv4EventMsgReceiveHw;
    BOOLEAN PtpV2OverUdpIPv4AllMsgReceiveHw;
    BOOLEAN PtpV2OverUdpIPv4EventMsgTransmitHw;
    BOOLEAN PtpV2OverUdpIPv4AllMsgTransmitHw;
    BOOLEAN PtpV2OverUdpIPv6EventMsgReceiveHw;
    BOOLEAN PtpV2OverUdpIPv6AllMsgReceiveHw;
    BOOLEAN PtpV2OverUdpIPv6EventMsgTransmitHw;
    BOOLEAN PtpV2OverUdpIPv6AllMsgTransmitHw;
    BOOLEAN AllReceiveHw;
    BOOLEAN AllTransmitHw;
    BOOLEAN TaggedTransmitHw;
    BOOLEAN AllReceiveSw;
    BOOLEAN AllTransmitSw;
    BOOLEAN TaggedTransmitSw;

} INTERFACE_TIMESTAMP_CAPABILITY_FLAGS, *PINTERFACE_TIMESTAMP_CAPABILITY_FLAGS;

typedef struct _INTERFACE_TIMESTAMP_CAPABILITIES
{
    ULONG Version;
    ULONG64 HardwareClockFrequencyHz;
    BOOLEAN CrossTimestamp;
    ULONG64 Reserved1;
    ULONG64 Reserved2;
    INTERFACE_TIMESTAMP_CAPABILITY_FLAGS TimestampFlags;
} INTERFACE_TIMESTAMP_CAPABILITIES, *PINTERFACE_TIMESTAMP_CAPABILITIES;

typedef struct _INTERFACE_HARDWARE_CROSSTIMESTAMP
{
    ULONG Version;
    ULONG Flags;
    ULONG64 SystemTimestamp1;
    ULONG64 HardwareClockTimestamp;
    ULONG64 SystemTimestamp2;

} INTERFACE_HARDWARE_CROSSTIMESTAMP, *PINTERFACE_HARDWARE_CROSSTIMESTAMP;

DECLARE_HANDLE(HIFTIMESTAMPCHANGE);

IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
GetInterfaceCurrentTimestampCapabilities(
    _In_ CONST NET_LUID *InterfaceLuid,
    _Inout_ PINTERFACE_TIMESTAMP_CAPABILITIES TimestampCapabilites
    );

IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
GetInterfaceHardwareTimestampCapabilities(
    _In_ CONST NET_LUID *InterfaceLuid,
    _Inout_ PINTERFACE_TIMESTAMP_CAPABILITIES TimestampCapabilites
    );

IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
CaptureInterfaceHardwareCrossTimestamp(
    _In_ CONST NET_LUID *InterfaceLuid,
    _Inout_ PINTERFACE_HARDWARE_CROSSTIMESTAMP CrossTimestamp
    );

typedef
VOID
CALLBACK
INTERFACE_TIMESTAMP_CONFIG_CHANGE_CALLBACK(
    _In_ PVOID CallerContext
    );

typedef
INTERFACE_TIMESTAMP_CONFIG_CHANGE_CALLBACK *PINTERFACE_TIMESTAMP_CONFIG_CHANGE_CALLBACK;

IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
NotifyIfTimestampConfigChange(
    _In_opt_ PVOID CallerContext,
    _In_ PINTERFACE_TIMESTAMP_CONFIG_CHANGE_CALLBACK Callback,
    _Out_ HIFTIMESTAMPCHANGE *NotificationHandle
    );

IPHLPAPI_DLL_LINKAGE
VOID
WINAPI
CancelIfTimestampConfigChange(
    _In_ HIFTIMESTAMPCHANGE NotificationHandle
    );

#endif // (NTDDI_VERSION >= NTDDI_WIN10_FE)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
IpReleaseAddress(
    _In_ PIP_ADAPTER_INDEX_MAP  AdapterInfo
    );

IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
IpRenewAddress(
    _In_ PIP_ADAPTER_INDEX_MAP  AdapterInfo
    );

IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
SendARP(
    _In_ IPAddr DestIP,
    _In_ IPAddr SrcIP,
    _Out_writes_bytes_(*PhyAddrLen) PVOID pMacAddr,
    _Inout_ PULONG  PhyAddrLen
    );

IPHLPAPI_DLL_LINKAGE
BOOL
WINAPI
GetRTTAndHopCount(
    _In_  IPAddr DestIpAddress,
    _Out_ PULONG HopCount,
    _In_  ULONG  MaxHops,
    _Out_ PULONG RTT
    );

IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
GetFriendlyIfIndex(
    _In_ DWORD IfIndex
    );

IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
EnableRouter(
    _Out_ HANDLE* pHandle,
    _Out_ OVERLAPPED* pOverlapped
    );

IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
UnenableRouter(
    _In_      OVERLAPPED* pOverlapped,
    _Out_opt_ LPDWORD lpdwEnableCount
    );

IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
DisableMediaSense(
    _Out_ HANDLE *pHandle,
    _In_  OVERLAPPED *pOverLapped
    );

IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
RestoreMediaSense(
    _In_      OVERLAPPED* pOverlapped,
    _Out_opt_ LPDWORD lpdwEnableCount
    );

#if (NTDDI_VERSION >= NTDDI_VISTA)

IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
GetIpErrorString(
    _In_    IP_STATUS ErrorCode,
    _Out_writes_opt_(*Size + 1) PWSTR Buffer,
    _Inout_ PDWORD Size
    );

#if (NTDDI_VERSION >= NTDDI_VISTA)
#ifdef _WS2DEF_
IPHLPAPI_DLL_LINKAGE
ULONG
WINAPI
ResolveNeighbor(
    _In_    SOCKADDR *NetworkAddress,
    _Out_writes_bytes_(*PhysicalAddressLength) PVOID PhysicalAddress,
    _Inout_ PULONG PhysicalAddressLength
    );
#endif
#endif

//
// Port reservation API routines.
//

IPHLPAPI_DLL_LINKAGE
ULONG
WINAPI
CreatePersistentTcpPortReservation(
    _In_  USHORT StartPort,
    _In_  USHORT NumberOfPorts,
    _Out_ PULONG64 Token
    );

IPHLPAPI_DLL_LINKAGE
ULONG
WINAPI
CreatePersistentUdpPortReservation(
    _In_  USHORT StartPort,
    _In_  USHORT NumberOfPorts,
    _Out_ PULONG64 Token
    );

IPHLPAPI_DLL_LINKAGE
ULONG
WINAPI
DeletePersistentTcpPortReservation(
    _In_ USHORT StartPort,
    _In_ USHORT NumberOfPorts
    );

IPHLPAPI_DLL_LINKAGE
ULONG
WINAPI
DeletePersistentUdpPortReservation(
    _In_ USHORT StartPort,
    _In_ USHORT NumberOfPorts
    );

IPHLPAPI_DLL_LINKAGE
ULONG
WINAPI
LookupPersistentTcpPortReservation(
    _In_  USHORT StartPort,
    _In_  USHORT NumberOfPorts,
    _Out_ PULONG64 Token
    );

IPHLPAPI_DLL_LINKAGE
ULONG
WINAPI
LookupPersistentUdpPortReservation(
    _In_  USHORT StartPort,
    _In_  USHORT NumberOfPorts,
    _Out_ PULONG64 Token
    );


//
// Network String parsing API
//

#define NET_STRING_IPV4_ADDRESS           0x00000001
   // The string identifies an IPv4 Host/router using literal address.
   // (port or prefix not allowed)
#define NET_STRING_IPV4_SERVICE           0x00000002
   // The string identifies an IPv4 service using literal address.
   // (port required; prefix not allowed)
#define NET_STRING_IPV4_NETWORK           0x00000004
   // The string identifies an IPv4 network.
   // (prefix required; port not allowed)
#define NET_STRING_IPV6_ADDRESS           0x00000008
   // The string identifies an IPv6 Host/router using literal address.
   // (port or prefix not allowed; scope-id allowed)
#define NET_STRING_IPV6_ADDRESS_NO_SCOPE  0x00000010
   // The string identifies an IPv6 Host/router using literal address
   // where the interface context is already known.
   // (port or prefix not allowed; scope-id not allowed)
#define NET_STRING_IPV6_SERVICE           0x00000020
   // The string identifies an IPv6 service using literal address.
   // (port required; prefix not allowed; scope-id allowed)
#define NET_STRING_IPV6_SERVICE_NO_SCOPE  0x00000040
   // The string identifies an IPv6 service using literal address
   // where the interface context is already known.
   // (port required; prefix not allowed; scope-id not allowed)
#define NET_STRING_IPV6_NETWORK           0x00000080
   // The string identifies an IPv6 network.
   // (prefix required; port or scope-id not allowed)
#define NET_STRING_NAMED_ADDRESS          0x00000100
   // The string identifies an Internet Host using DNS.
   // (port or prefix or scope-id not allowed)
#define NET_STRING_NAMED_SERVICE          0x00000200
   // The string identifies an Internet service using DNS.
   // (port required; prefix or scope-id not allowed)

#define NET_STRING_IP_ADDRESS             (NET_STRING_IPV4_ADDRESS   | \
                                           NET_STRING_IPV6_ADDRESS)

#define NET_STRING_IP_ADDRESS_NO_SCOPE    (NET_STRING_IPV4_ADDRESS   | \
                                           NET_STRING_IPV6_ADDRESS_NO_SCOPE)

#define NET_STRING_IP_SERVICE             (NET_STRING_IPV4_SERVICE   | \
                                           NET_STRING_IPV6_SERVICE)

#define NET_STRING_IP_SERVICE_NO_SCOPE    (NET_STRING_IPV4_SERVICE   | \
                                           NET_STRING_IPV6_SERVICE_NO_SCOPE)

#define NET_STRING_IP_NETWORK             (NET_STRING_IPV4_NETWORK   | \
                                           NET_STRING_IPV6_NETWORK)

#define NET_STRING_ANY_ADDRESS            (NET_STRING_NAMED_ADDRESS  | \
                                           NET_STRING_IP_ADDRESS)

#define NET_STRING_ANY_ADDRESS_NO_SCOPE   (NET_STRING_NAMED_ADDRESS  | \
                                           NET_STRING_IP_ADDRESS_NO_SCOPE)

#define NET_STRING_ANY_SERVICE            (NET_STRING_NAMED_SERVICE  | \
                                           NET_STRING_IP_SERVICE)

#define NET_STRING_ANY_SERVICE_NO_SCOPE   (NET_STRING_NAMED_SERVICE  | \
                                           NET_STRING_IP_SERVICE_NO_SCOPE)

typedef enum NET_ADDRESS_FORMAT_
{
   NET_ADDRESS_FORMAT_UNSPECIFIED = 0,

   NET_ADDRESS_DNS_NAME,
   NET_ADDRESS_IPV4,
   NET_ADDRESS_IPV6

} NET_ADDRESS_FORMAT;

#if defined (_WS2DEF_) && defined (_WS2IPDEF_) && defined(_WINDNS_INCLUDED_)
	// app must include winsock2.h, ws2ipdef.h, and windns.h to use this API

typedef struct NET_ADDRESS_INFO_
{
   NET_ADDRESS_FORMAT Format;

   union
   {
      struct {
         WCHAR Address[DNS_MAX_NAME_BUFFER_LENGTH];
         WCHAR Port[6];
      } NamedAddress;

      SOCKADDR_IN Ipv4Address;
      SOCKADDR_IN6 Ipv6Address;
      SOCKADDR IpAddress;
   };

} NET_ADDRESS_INFO, *PNET_ADDRESS_INFO;

IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
ParseNetworkString(
   _In_z_    CONST WCHAR* NetworkString,
   _In_      DWORD Types,
   _Out_opt_ PNET_ADDRESS_INFO AddressInfo,
   _Out_opt_ USHORT* PortNumber,
   _Out_opt_ BYTE* PrefixLength
   );

#endif

#endif // (NTDDI_VERSION >= NTDDI_VISTA)

#if (NTDDI_VERSION >= NTDDI_VISTA)

#include <netioapi.h>

#endif // (NTDDI_VERSION >= NTDDI_VISTA)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)
#ifdef __cplusplus
}
#endif
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#endif //__IPHLPAPI_H__


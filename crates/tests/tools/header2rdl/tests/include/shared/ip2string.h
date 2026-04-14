/*++ BUILD Version: 0006    // Increment this if a change has global effects

Copyright (c) Microsoft Corporation.  All rights reserved.

Module Name:

    ip2string.h

Abstract:

    Include file for NT runtime routines that are callable by both
    kernel mode code in the executive and user mode code in various
    NT subsystems.

    This module contains definitions that are exposed in kit headers
    for use by external developers. Splited from ntrtl_x.h

Author:

    Zhihui Chen (stevewo) 03-May-2006

Environment:

    These routines are dynamically linked in the caller's executable and
    are callable in either kernel mode or user mode.


--*/

#ifndef __IP2STRING__
#define __IP2STRING__
#pragma once
#include <winapifamily.h>

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)


#ifdef __cplusplus
extern "C" {
#endif

//
//  Some simple Rtl routines for IP address <-> string literal conversion
//  ... and Ethernet
//

struct in_addr;
struct in6_addr;
union _DL_EUI48;
typedef union _DL_EUI48 DL_EUI48, *PDL_EUI48;

NTSYSAPI
PSTR
NTAPI
RtlIpv4AddressToStringA (
    _In_ const struct in_addr *Addr,
    _Out_writes_(16) PSTR S
    );

NTSYSAPI
PSTR
NTAPI
RtlIpv6AddressToStringA (
    _In_ const struct in6_addr *Addr,
    _Out_writes_(46) PSTR S
    );

NTSYSAPI
PSTR
NTAPI
RtlEthernetAddressToStringA (
    _In_ const DL_EUI48 *Addr,
    _Out_writes_(18) PSTR S
    );

NTSYSAPI
NTSTATUS
NTAPI
RtlIpv4AddressToStringExA(
    _In_ const struct in_addr *Address,
    _In_ USHORT Port,
    _Out_writes_to_(*AddressStringLength, *AddressStringLength) PSTR AddressString,
    _Inout_ PULONG AddressStringLength
    );

NTSYSAPI
NTSTATUS
NTAPI
RtlIpv6AddressToStringExA(
    _In_ const struct in6_addr *Address,
    _In_ ULONG ScopeId,
    _In_ USHORT Port,
    _Out_writes_to_(*AddressStringLength, *AddressStringLength) PSTR AddressString,
    _Inout_ PULONG AddressStringLength
    );

NTSYSAPI
PWSTR
NTAPI
RtlIpv4AddressToStringW (
    _In_ const struct in_addr *Addr,
    _Out_writes_(16) PWSTR S
    );

NTSYSAPI
PWSTR
NTAPI
RtlIpv6AddressToStringW (
    _In_ const struct in6_addr *Addr,
    _Out_writes_(46) PWSTR S
    );

NTSYSAPI
PWSTR
NTAPI
RtlEthernetAddressToStringW (
    _In_ const DL_EUI48 *Addr,
    _Out_writes_(18) PWSTR S
    );

NTSYSAPI
NTSTATUS
NTAPI
RtlIpv4AddressToStringExW(
    _In_ const struct in_addr *Address,
    _In_ USHORT Port,
    _Out_writes_to_(*AddressStringLength, *AddressStringLength) PWSTR AddressString,
    _Inout_ PULONG AddressStringLength
    );

NTSYSAPI
NTSTATUS
NTAPI
RtlIpv6AddressToStringExW(
    _In_ const struct in6_addr *Address,
    _In_ ULONG ScopeId,
    _In_ USHORT Port,
    _Out_writes_to_(*AddressStringLength, *AddressStringLength) PWSTR AddressString,
    _Inout_ PULONG AddressStringLength
    );

_Must_inspect_result_
NTSYSAPI
NTSTATUS
NTAPI
RtlIpv4StringToAddressA (
    _In_ PCSTR S,
    _In_ BOOLEAN Strict,
    _Out_ PCSTR *Terminator,
    _Out_ struct in_addr *Addr
    );

_Must_inspect_result_
NTSYSAPI
NTSTATUS
NTAPI
RtlIpv6StringToAddressA (
    _In_ PCSTR S,
    _Out_ PCSTR *Terminator,
    _Out_ struct in6_addr *Addr
    );

_Must_inspect_result_
NTSYSAPI
NTSTATUS
NTAPI
RtlEthernetStringToAddressA (
    _In_ PCSTR S,
    _Out_ PCSTR *Terminator,
    _Out_ DL_EUI48 *Addr
    );

_Must_inspect_result_
NTSYSAPI
NTSTATUS
NTAPI
RtlIpv4StringToAddressExA (
    _In_ PCSTR AddressString,
    _In_ BOOLEAN Strict,
    _Out_ struct in_addr *Address,
    _Out_ PUSHORT Port
    );

_Must_inspect_result_
NTSYSAPI
NTSTATUS
NTAPI
RtlIpv6StringToAddressExA (
    _In_ PCSTR AddressString,
    _Out_ struct in6_addr *Address,
    _Out_ PULONG ScopeId,
    _Out_ PUSHORT Port
    );

_Must_inspect_result_
NTSYSAPI
NTSTATUS
NTAPI
RtlIpv4StringToAddressW (
    _In_ PCWSTR S,
    _In_ BOOLEAN Strict,
    _Out_ LPCWSTR *Terminator,
    _Out_ struct in_addr *Addr
    );

_Must_inspect_result_
NTSYSAPI
NTSTATUS
NTAPI
RtlIpv6StringToAddressW (
    _In_ PCWSTR S,
    _Out_ PCWSTR *Terminator,
    _Out_ struct in6_addr *Addr
    );

_Must_inspect_result_
NTSYSAPI
NTSTATUS
NTAPI
RtlEthernetStringToAddressW (
    _In_ PCWSTR S,
    _Out_ LPCWSTR *Terminator,
    _Out_ DL_EUI48 *Addr
    );

_Must_inspect_result_
NTSYSAPI
NTSTATUS
NTAPI
RtlIpv4StringToAddressExW (
    _In_ PCWSTR AddressString,
    _In_ BOOLEAN Strict,
    _Out_ struct in_addr *Address,
    _Out_ PUSHORT Port
    );

_Must_inspect_result_
NTSYSAPI
NTSTATUS
NTAPI
RtlIpv6StringToAddressExW (
    _In_ PCWSTR AddressString,
    _Out_ struct in6_addr *Address,
    _Out_ PULONG ScopeId,
    _Out_ PUSHORT Port
    );

#ifdef UNICODE
#define RtlIpv4AddressToString RtlIpv4AddressToStringW
#define RtlIpv6AddressToString RtlIpv6AddressToStringW
#define RtlEthernetAddressToString RtlEthernetAddressToStringW
#define RtlIpv4StringToAddress RtlIpv4StringToAddressW
#define RtlIpv6StringToAddress RtlIpv6StringToAddressW
#define RtlEthernetStringToAddress RtlEthernetStringToAddressW
#define RtlIpv6StringToAddressEx RtlIpv6StringToAddressExW
#define RtlIpv4AddressToStringEx RtlIpv4AddressToStringExW
#define RtlIpv6AddressToStringEx RtlIpv6AddressToStringExW
#define RtlIpv4StringToAddressEx RtlIpv4StringToAddressExW
#else
#define RtlIpv4AddressToString RtlIpv4AddressToStringA
#define RtlIpv6AddressToString RtlIpv6AddressToStringA
#define RtlEthernetAddressToString RtlEthernetAddressToStringA
#define RtlIpv4StringToAddress RtlIpv4StringToAddressA
#define RtlIpv6StringToAddress RtlIpv6StringToAddressA
#define RtlEthernetStringToAddress RtlEthernetStringToAddressA
#define RtlIpv6StringToAddressEx RtlIpv6StringToAddressExA
#define RtlIpv4AddressToStringEx RtlIpv4AddressToStringExA
#define RtlIpv6AddressToStringEx RtlIpv6AddressToStringExA
#define RtlIpv4StringToAddressEx RtlIpv4StringToAddressExA
#endif // UNICODE

#ifdef __cplusplus
}       // extern "C"
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif // __IP2STRING__

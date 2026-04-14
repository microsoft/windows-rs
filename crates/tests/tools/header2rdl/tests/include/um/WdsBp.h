/*++

Copyright (c) 2006 Microsoft Corporation

Module Name:

    WdsBp.h

Abstract:

    This module defines data structure and public APIs which are used to parse
    and construct packets for WDS Network Boot Program.

Environment:

    User Mode

--*/

#ifndef __WDSBP_H__
#define __WDSBP_H__
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#ifdef __cplusplus
extern "C"
{
#endif

//
// Calling Convention.
//

#define WDSBPAPI        __stdcall

//
// Options used by WDS Network Boot Program.
//

#define WDSBP_MAKE_OPTION(_Pk, _Type, _Id)      (static_cast<ULONG>((_Type) | ((_Pk) << 8) | ((_Id) << 16)))

//
// Packet Types.
//

#define WDSBP_PK_TYPE_DHCP                      1
#define WDSBP_PK_TYPE_WDSNBP                    2
#define WDSBP_PK_TYPE_BCD                       4
#define WDSBP_PK_TYPE_DHCPV6                    8

//
// Option Types.
//

#define WDSBP_OPT_TYPE_NONE                     0
#define WDSBP_OPT_TYPE_BYTE                     1
#define WDSBP_OPT_TYPE_USHORT                   2
#define WDSBP_OPT_TYPE_ULONG                    3
#define WDSBP_OPT_TYPE_WSTR                     4
#define WDSBP_OPT_TYPE_STR                      5
#define WDSBP_OPT_TYPE_IP4                      6
#define WDSBP_OPT_TYPE_IP6                      7

//
// Boot Program Options.
//

#define WDSBP_OPT_ARCHITECTURE                  WDSBP_MAKE_OPTION(WDSBP_PK_TYPE_WDSNBP,     WDSBP_OPT_TYPE_USHORT,        1)
#define WDSBP_OPT_NEXT_ACTION                   WDSBP_MAKE_OPTION(WDSBP_PK_TYPE_WDSNBP,     WDSBP_OPT_TYPE_BYTE,          2)
#define WDSBP_OPT_POLL_INTERVAL                 WDSBP_MAKE_OPTION(WDSBP_PK_TYPE_WDSNBP,     WDSBP_OPT_TYPE_USHORT,        3)
#define WDSBP_OPT_POLL_RETRY_COUNT              WDSBP_MAKE_OPTION(WDSBP_PK_TYPE_WDSNBP,     WDSBP_OPT_TYPE_USHORT,        4)
#define WDSBP_OPT_REQUEST_ID                    WDSBP_MAKE_OPTION(WDSBP_PK_TYPE_WDSNBP,     WDSBP_OPT_TYPE_ULONG,         5)
#define WDSBP_OPT_MESSAGE                       WDSBP_MAKE_OPTION(WDSBP_PK_TYPE_WDSNBP,     WDSBP_OPT_TYPE_STR,           6)
#define WDSBP_OPT_VERSION_QUERY                 WDSBP_MAKE_OPTION(WDSBP_PK_TYPE_WDSNBP,     WDSBP_OPT_TYPE_NONE,          7)
#define WDSBP_OPT_SERVER_VERSION                WDSBP_MAKE_OPTION(WDSBP_PK_TYPE_WDSNBP,     WDSBP_OPT_TYPE_ULONG,         8)
#define WDSBP_OPT_REFERRAL_SERVER               WDSBP_MAKE_OPTION(WDSBP_PK_TYPE_WDSNBP,     WDSBP_OPT_TYPE_IP4,           9)
#define WDSBP_OPT_PXE_CLIENT_PROMPT             WDSBP_MAKE_OPTION(WDSBP_PK_TYPE_WDSNBP,     WDSBP_OPT_TYPE_BYTE,          11)
#define WDSBP_OPT_PXE_PROMPT_DONE               WDSBP_MAKE_OPTION(WDSBP_PK_TYPE_WDSNBP,     WDSBP_OPT_TYPE_BYTE,          12)
#define WDSBP_OPT_NBP_VER                       WDSBP_MAKE_OPTION(WDSBP_PK_TYPE_WDSNBP,     WDSBP_OPT_TYPE_USHORT,        13)
#define WDSBP_OPT_ACTION_DONE                   WDSBP_MAKE_OPTION(WDSBP_PK_TYPE_WDSNBP,     WDSBP_OPT_TYPE_BYTE,          14)
#define WDSBP_OPT_ALLOW_SERVER_SELECTION        WDSBP_MAKE_OPTION(WDSBP_PK_TYPE_WDSNBP,     WDSBP_OPT_TYPE_BYTE,          15)
#define WDSBP_OPT_SERVER_FEATURES               WDSBP_MAKE_OPTION(WDSBP_PK_TYPE_WDSNBP,     WDSBP_OPT_TYPE_ULONG,         16)
#define WDSBP_OPT_BCD_FILE_PATH                 WDSBP_MAKE_OPTION(WDSBP_PK_TYPE_BCD,        WDSBP_OPT_TYPE_STR,           252)

//
// As the commercial devices with PXEv6 clients are (U)EFI machines, no architecture discovery is needed.
// Thus, in DHCPv6, there is no WDS BP architecture option.
//
#define WDSBP_OPTV6_NEXT_ACTION                 WDSBP_MAKE_OPTION(WDSBP_PK_TYPE_DHCPV6,     WDSBP_OPT_TYPE_BYTE,          50001)
#define WDSBP_OPTV6_POLL_INTERVAL               WDSBP_MAKE_OPTION(WDSBP_PK_TYPE_DHCPV6,     WDSBP_OPT_TYPE_USHORT,        50002)
#define WDSBP_OPTV6_POLL_RETRY_COUNT            WDSBP_MAKE_OPTION(WDSBP_PK_TYPE_DHCPV6,     WDSBP_OPT_TYPE_USHORT,        50003)
#define WDSBP_OPTV6_REQUEST_ID                  WDSBP_MAKE_OPTION(WDSBP_PK_TYPE_DHCPV6,     WDSBP_OPT_TYPE_ULONG,         50004)
#define WDSBP_OPTV6_MESSAGE                     WDSBP_MAKE_OPTION(WDSBP_PK_TYPE_DHCPV6,     WDSBP_OPT_TYPE_STR,           50005)
#define WDSBP_OPTV6_VERSION_QUERY               WDSBP_MAKE_OPTION(WDSBP_PK_TYPE_DHCPV6,     WDSBP_OPT_TYPE_NONE,          50006)
#define WDSBP_OPTV6_SERVER_VERSION              WDSBP_MAKE_OPTION(WDSBP_PK_TYPE_DHCPV6,     WDSBP_OPT_TYPE_ULONG,         50007)
#define WDSBP_OPTV6_REFERRAL_SERVER             WDSBP_MAKE_OPTION(WDSBP_PK_TYPE_DHCPV6,     WDSBP_OPT_TYPE_IP6,           50008)
#define WDSBP_OPTV6_PXE_CLIENT_PROMPT           WDSBP_MAKE_OPTION(WDSBP_PK_TYPE_DHCPV6,     WDSBP_OPT_TYPE_BYTE,          50009)
#define WDSBP_OPTV6_PXE_PROMPT_DONE             WDSBP_MAKE_OPTION(WDSBP_PK_TYPE_DHCPV6,     WDSBP_OPT_TYPE_BYTE,          50010)
#define WDSBP_OPTV6_NBP_VER                     WDSBP_MAKE_OPTION(WDSBP_PK_TYPE_DHCPV6,     WDSBP_OPT_TYPE_USHORT,        50011)
#define WDSBP_OPTV6_ACTION_DONE                 WDSBP_MAKE_OPTION(WDSBP_PK_TYPE_DHCPV6,     WDSBP_OPT_TYPE_BYTE,          50012)
#define WDSBP_OPTV6_ALLOW_SERVER_SELECTION      WDSBP_MAKE_OPTION(WDSBP_PK_TYPE_DHCPV6,     WDSBP_OPT_TYPE_BYTE,          50013)
#define WDSBP_OPTV6_BCD_FILE_PATH               WDSBP_MAKE_OPTION(WDSBP_PK_TYPE_DHCPV6,     WDSBP_OPT_TYPE_STR,           50014)
#define WDSBP_OPTV6_REPLYING_SERVER_NAME        WDSBP_MAKE_OPTION(WDSBP_PK_TYPE_DHCPV6,     WDSBP_OPT_TYPE_STR,           50015)
#define WDSBP_OPTV6_REPLYING_SERVER_ADDRESS     WDSBP_MAKE_OPTION(WDSBP_PK_TYPE_DHCPV6,     WDSBP_OPT_TYPE_IP6,           50016)
#define WDSBP_OPTV6_BOOT_PROGRAM_TFTP_PATH      WDSBP_MAKE_OPTION(WDSBP_PK_TYPE_DHCPV6,     WDSBP_OPT_TYPE_STR,           50017)
#define WDSBP_OPTV6_SERVER_FEATURES             WDSBP_MAKE_OPTION(WDSBP_PK_TYPE_DHCPV6,     WDSBP_OPT_TYPE_ULONG,         50018)

//
// Helper Macros.
//

#define WDSBP_IS_PK_TYPE(_Value, _Type)         (((_Value) & (_Type)) == (_Type))

//
// Values for WDSBP_OPT_NEXT_ACTION Option.
//

#define WDSBP_OPTVAL_ACTION_APPROVAL            1
#define WDSBP_OPTVAL_ACTION_REFERRAL            3
#define WDSBP_OPTVAL_ACTION_ABORT               5

//
// Values for WDSBP_OPT_PXE_CLIENT_PROMPT and WDSBP_OPT_PXE_PROMPT_DONE.
//

#define WDSBP_OPTVAL_PXE_PROMPT_OPTIN           1
#define WDSBP_OPTVAL_PXE_PROMPT_NOPROMPT        2
#define WDSBP_OPTVAL_PXE_PROMPT_OPTOUT          3

//
// Values for WDSBP_OPT_NBP_VER.
//

#define WDSBP_OPTVAL_NBP_VER_7                  0x0700
#define WDSBP_OPTVAL_NBP_VER_8                  0x0800

//
// APIs.
//

DWORD
WDSBPAPI
WdsBpParseInitialize (
    _In_reads_bytes_(uPacketLen) PVOID pPacket,
    _In_ ULONG uPacketLen,
    _Out_opt_ PBYTE pbPacketType,
    _Out_ HANDLE *phHandle
    );

DWORD
WDSBPAPI
WdsBpParseInitializev6 (
    _In_reads_bytes_(uPacketLen) PVOID pPacket,
    _In_ ULONG uPacketLen,
    _Out_opt_ PBYTE pbPacketType,
    _Out_ HANDLE *phHandle
    );

DWORD
WDSBPAPI
WdsBpInitialize (
    _In_ BYTE bPacketType,
    _Out_ HANDLE *phHandle
    );

DWORD
WDSBPAPI
WdsBpCloseHandle (
    _In_ HANDLE hHandle
    );

_Success_(return == ERROR_SUCCESS)
DWORD
WDSBPAPI
WdsBpQueryOption (
    _In_ HANDLE hHandle,
    _In_ ULONG uOption,
    _In_ ULONG uValueLen,
    _Out_writes_bytes_(uValueLen) PVOID pValue,
    _Out_opt_ PULONG puBytes
    );

DWORD
WDSBPAPI
WdsBpAddOption (
    _In_ HANDLE hHandle,
    _In_ ULONG uOption,
    _In_ ULONG uValueLen,
    _In_reads_bytes_(uValueLen) PVOID pValue
    );

_Success_(return == ERROR_SUCCESS)
DWORD
WDSBPAPI
WdsBpGetOptionBuffer (
    _In_ HANDLE hHandle,
    _In_ ULONG uBufferLen,
    _Out_writes_bytes_(uBufferLen) PVOID pBuffer,
    _Out_ PULONG puBytes
    );

#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif


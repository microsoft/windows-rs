/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    wdspxe.h

Abstract:

    Contains functions used to write Providers for WDSPXE Server

--*/

#ifndef __WDSPXE_H__
#define __WDSPXE_H__
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#ifdef __cplusplus
extern "C" {
#endif

//
// Calling convention for WDSPXE APIs.
//
#define PXEAPI __stdcall

//
// Ports for WDS PXE Server.
//
#define     PXE_DHCP_SERVER_PORT            67
#define     PXE_DHCP_CLIENT_PORT            68
#define     PXE_SERVER_PORT                 4011
#define     PXE_DHCPV6_SERVER_PORT          547
#define     PXE_DHCPV6_CLIENT_PORT          546

//
// PXE_DHCP_MESSAGE
//
#define     PXE_DHCP_FILE_SIZE              128
#define     PXE_DHCP_SERVER_SIZE            64
#define     PXE_DHCP_HWAADR_SIZE            16

#define     PXE_DHCP_MAGIC_COOKIE_SIZE      4
#define     PXEDHCP_MAGIC_COOKIE            (ULONG)0x63538263

//
// PXE on-wire data structures.
//

#include <pshpack1.h>

//
// PXE_DHCP_OPTION
//
typedef struct tagPXE_DHCP_OPTION
{
    BYTE OptionType;
    BYTE OptionLength;
    BYTE OptionValue[1];
} PXE_DHCP_OPTION, *PPXE_DHCP_OPTION;

typedef struct tagPXE_DHCP_MESSAGE
{
    BYTE Operation;
    BYTE HardwareAddressType;
    BYTE HardwareAddressLength;
    BYTE HopCount;
    DWORD TransactionID;
    WORD SecondsSinceBoot;
    WORD Reserved;
    ULONG ClientIpAddress;
    ULONG YourIpAddress;
    ULONG BootstrapServerAddress;
    ULONG RelayAgentIpAddress;
    BYTE HardwareAddress[PXE_DHCP_HWAADR_SIZE];
    BYTE HostName[PXE_DHCP_SERVER_SIZE];
    BYTE BootFileName[PXE_DHCP_FILE_SIZE];

    union
    {
        BYTE bMagicCookie[PXE_DHCP_MAGIC_COOKIE_SIZE];
        ULONG uMagicCookie;
    };

    PXE_DHCP_OPTION Option;
} PXE_DHCP_MESSAGE, *PPXE_DHCP_MESSAGE;

typedef struct tagPXE_DHCPV6_OPTION
{
    WORD OptionCode;
    WORD DataLength;
    BYTE Data[1];
} PXE_DHCPV6_OPTION, *PPXE_DHCPV6_OPTION;

typedef struct tagPXE_DHCPV6_MESSAGE_HEADER
{
    BYTE MessageType;
    BYTE Message[1];
} PXE_DHCPV6_MESSAGE_HEADER, *PPXE_DHCPV6_MESSAGE_HEADER;

typedef struct tagPXE_DHCPV6_MESSAGE
{
    BYTE MessageType;
    BYTE TransactionIDByte1;
    BYTE TransactionIDByte2;
    BYTE TransactionIDByte3;
    PXE_DHCPV6_OPTION Options[1];
} PXE_DHCPV6_MESSAGE, *PPXE_DHCPV6_MESSAGE;

typedef struct tagPXE_DHCPV6_RELAY_MESSAGE
{
    BYTE MessageType;
    BYTE HopCount;
    BYTE LinkAddress[16];
    BYTE PeerAddress[16];
    PXE_DHCPV6_OPTION Options[1];
} PXE_DHCPV6_RELAY_MESSAGE, *PPXE_DHCPV6_RELAY_MESSAGE;


#include <poppack.h>

//
// PXE API data structures.
//

//
// PXE_REG_INDEX
//
typedef ULONG PXE_REG_INDEX;

#define PXE_REG_INDEX_TOP           (0)
#define PXE_REG_INDEX_BOTTOM        0xFFFFFFFF

//
// PXE_PROVIDER
//
typedef struct tagPXE_PROVIDER
{
    ULONG uSizeOfStruct;
    LPWSTR pwszName;
    LPWSTR pwszFilePath;
    BOOL bIsCritical;
    ULONG uIndex;
} PXE_PROVIDER, *PPXE_PROVIDER;

//
// PXE_CALLBACK_TYPE
//
typedef ULONG PXE_CALLBACK_TYPE;

#define PXE_CALLBACK_RECV_REQUEST       0
#define PXE_CALLBACK_SHUTDOWN           1
#define PXE_CALLBACK_SERVICE_CONTROL    2

#define PXE_CALLBACK_MAX                3

//
// PXE_GSI_TYPE
//
typedef ULONG PXE_GSI_TYPE;

#define PXE_GSI_TRACE_ENABLED           1
#define PXE_GSI_SERVER_DUID             2

//
// PXE_ADDRESS
//
#define PXE_MAX_ADDRESS         16

#define PXE_ADDR_BROADCAST      0x0001
#define PXE_ADDR_USE_PORT       0x0002
#define PXE_ADDR_USE_ADDR       0x0004
#define PXE_ADDR_USE_DHCP_RULES 0x0008

typedef struct tagPXE_ADDRESS
{
    ULONG uFlags;

    union
    {
        BYTE bAddress[PXE_MAX_ADDRESS];
        ULONG uIpAddress;
    };

    ULONG uAddrLen;
    USHORT uPort;
} PXE_ADDRESS, *PPXE_ADDRESS;

//
// PXE_DHCPV6_RELAY_MESSAGE
//

#define PXE_DHCPV6_RELAY_HOP_COUNT_LIMIT 32

typedef struct tagPXE_DHCPV6_NESTED_RELAY_MESSAGE
{
    //
    // A pointer to and the size of a nested RELAY-FORW message.
    //

    _Field_size_bytes_(cbRelayMessage) PPXE_DHCPV6_RELAY_MESSAGE pRelayMessage;
    ULONG cbRelayMessage;

    //
    // If the nested RELAY-FORW message contains OPTION_INTERFACE_ID, the
    // following will track a pointer to and the size of the option payload.
    // Otherwise, these fields will be NULL and 0 respectively.
    //

    _Field_size_bytes_(cbInterfaceIdOption) PVOID pInterfaceIdOption;
    WORD cbInterfaceIdOption;
} PXE_DHCPV6_NESTED_RELAY_MESSAGE, *PPXE_DHCPV6_NESTED_RELAY_MESSAGE;

//
// PXE_BOOT_ACTION
//
typedef ULONG PXE_BOOT_ACTION;

#define PXE_BA_NBP              1
#define PXE_BA_CUSTOM           2
#define PXE_BA_IGNORE           3
#define PXE_BA_REJECTED         4

//
// PXE_SEVERITY
//
typedef ULONG PXE_SEVERITY;

#define PXE_TRACE_VERBOSE           0x00010000
#define PXE_TRACE_INFO              0x00020000
#define PXE_TRACE_WARNING           0x00040000
#define PXE_TRACE_ERROR             0x00080000
#define PXE_TRACE_FATAL             0x00100000

//
// PXE_PROVIDER_ATTRIBUTE
//
typedef ULONG PXE_PROVIDER_ATTRIBUTE;

#define PXE_PROV_ATTR_FILTER        0
#define PXE_PROV_ATTR_FILTER_IPV6   1
#define PXE_PROV_ATTR_IPV6_CAPABLE  2

#define PXE_PROV_FILTER_ALL         0x0000
#define PXE_PROV_FILTER_DHCP_ONLY   0x0001
#define PXE_PROV_FILTER_PXE_ONLY    0x0002
//
// Provider Registration and Enumeration APIs
//
DWORD
PXEAPI
PxeProviderRegister (
    _In_ LPCWSTR pszProviderName,
    _In_ LPCWSTR pszModulePath,
    _In_ PXE_REG_INDEX Index,
    _In_ BOOL bIsCritical,
    _Out_opt_ PHKEY phProviderKey
    );

DWORD
PXEAPI
PxeProviderUnRegister (
    _In_ LPCWSTR pszProviderName
    );

DWORD
PXEAPI
PxeProviderQueryIndex (
    _In_ LPCWSTR pszProviderName,
    _Out_ PULONG puIndex
    );

DWORD
PXEAPI
PxeProviderEnumFirst (
    _Out_ HANDLE *phEnum
    );

DWORD
PXEAPI
PxeProviderEnumNext (
    _In_ HANDLE hEnum,
    _Out_ PPXE_PROVIDER *ppProvider
    );

DWORD
PXEAPI
PxeProviderEnumClose (
    _In_ HANDLE hEnum
    );

DWORD
PxeProviderFreeInfo (
    _In_ PPXE_PROVIDER pProvider
    );

//
// Callback Registration
//
DWORD
PXEAPI
PxeRegisterCallback (
    _In_ HANDLE hProvider,
    _In_ PXE_CALLBACK_TYPE CallbackType,
    _In_ PVOID pCallbackFunction,
    _In_opt_ PVOID pContext
    );

//
// UDP Send/Boot Action
//
DWORD
PXEAPI
PxeSendReply (
    _In_ HANDLE hClientRequest,
    _In_reads_bytes_(uPacketLen) PVOID pPacket,
    _In_ ULONG uPacketLen,
    _In_ PXE_ADDRESS *pAddress
    );

//
// Notify of Async Processing
//
DWORD
PXEAPI
PxeAsyncRecvDone (
    _In_ HANDLE hClientRequest,
    _In_ PXE_BOOT_ACTION Action
    );

//
// Trace
//

DWORD
PXEAPI
PxeTrace (
    _In_ HANDLE hProvider,
    _In_ PXE_SEVERITY Severity,
    _In_ LPCWSTR pszFormat,
    ...
    );

DWORD
PXEAPI
PxeTraceV (
    _In_ HANDLE hProvider,
    _In_ PXE_SEVERITY Severity,
    _In_ LPCWSTR pszFormat,
    _In_ va_list Params
    );

//
// Packet Allocation
//
PVOID
PXEAPI
PxePacketAllocate (
    _In_ HANDLE hProvider,
    _In_ HANDLE hClientRequest,
    _In_ ULONG uSize
    );

DWORD
PXEAPI
PxePacketFree (
    _In_ HANDLE hProvider,
    _In_ HANDLE hClientRequest,
    _In_ PVOID pPacket
    );

//
// Provider Attributes
//
DWORD
PXEAPI
PxeProviderSetAttribute (
    _In_ HANDLE hProvider,
    _In_ PXE_PROVIDER_ATTRIBUTE Attribute,
    _In_reads_bytes_(uParamLen) PVOID pParameterBuffer,
    _In_ ULONG uParamLen
    );

//
// Dhcp Functions
//
DWORD
PXEAPI
PxeDhcpInitialize (
    _In_reads_bytes_(uRecvPacketLen) PVOID pRecvPacket,
    _In_ ULONG uRecvPacketLen,
    _Out_writes_bytes_to_(uMaxReplyPacketLen, *puReplyPacketLen) PVOID pReplyPacket,
    _In_ ULONG uMaxReplyPacketLen,
    _Out_ PULONG puReplyPacketLen
    );

DWORD
PXEAPI
PxeDhcpv6Initialize (
    _In_reads_bytes_(cbRequest) PVOID pRequest,
    _In_ ULONG cbRequest,
    _Out_writes_bytes_to_(cbReply, *pcbReplyUsed) PVOID pReply,
    _In_ ULONG cbReply,
    _Out_ PULONG pcbReplyUsed
    );

DWORD
PXEAPI
PxeDhcpAppendOption (
    _Inout_updates_bytes_(uMaxReplyPacketLen) PVOID pReplyPacket,
    _In_ ULONG uMaxReplyPacketLen,
    _Inout_ PULONG puReplyPacketLen,
    _In_ BYTE bOption,
    _In_ BYTE bOptionLen,
    _In_reads_bytes_opt_(bOptionLen) PVOID pValue
    );

_Success_(return == ERROR_SUCCESS)
DWORD
PXEAPI
PxeDhcpv6AppendOption (
    _Inout_updates_bytes_to_(cbReply, *pcbReplyUsed) PVOID pReply,
    _In_ ULONG cbReply,
    _Inout_ PULONG pcbReplyUsed,
    _In_ WORD wOptionType,
    _In_ WORD cbOption,
    _In_reads_bytes_(cbOption) PVOID pOption
    );

DWORD
PXEAPI
PxeDhcpAppendOptionRaw (
    _Inout_updates_bytes_(uMaxReplyPacketLen) PVOID pReplyPacket,
    _In_ ULONG uMaxReplyPacketLen,
    _Inout_ PULONG puReplyPacketLen,
    _In_ USHORT uBufferLen,
    _In_reads_bytes_(uBufferLen) PVOID pBuffer
    );

_Success_(return == ERROR_SUCCESS)
DWORD
PXEAPI
PxeDhcpv6AppendOptionRaw (
    _Inout_updates_bytes_to_(cbReply, *pcbReplyUsed) PVOID pReply,
    _In_ ULONG cbReply,
    _Inout_ PULONG pcbReplyUsed,
    _In_ USHORT cbBuffer,
    _In_reads_bytes_(cbBuffer) PVOID pBuffer
    );

DWORD
PXEAPI
PxeDhcpIsValid (
    _In_reads_bytes_(uPacketLen) PVOID pPacket,
    _In_ ULONG uPacketLen,
    _In_ BOOL bRequestPacket,
    _Out_opt_ PBOOL pbPxeOptionPresent
    );

DWORD
PXEAPI
PxeDhcpv6IsValid (
    _In_reads_bytes_(uPacketLen) PVOID pPacket,
    _In_ ULONG uPacketLen,
    _In_ BOOL bRequestPacket,
    _Out_opt_ PBOOL pbPxeOptionPresent
    );

DWORD
PXEAPI
PxeDhcpGetOptionValue (
    _In_reads_bytes_(uPacketLen) PVOID pPacket,
    _In_ ULONG uPacketLen,
    _In_ ULONG uInstance,
    _In_ BYTE bOption,
    _Out_opt_ PBYTE pbOptionLen,
    _Outptr_opt_result_bytebuffer_(*pbOptionLen) PVOID *ppOptionValue
    );

DWORD
PXEAPI
PxeDhcpv6GetOptionValue (
    _In_reads_bytes_(uPacketLen) PVOID pPacket,
    _In_ ULONG uPacketLen,
    _In_ ULONG uInstance,
    _In_ WORD wOption,
    _Out_opt_ PWORD pwOptionLen,
    _Outptr_opt_result_bytebuffer_(*pwOptionLen) PVOID *ppOptionValue
    );

DWORD
PXEAPI
PxeDhcpGetVendorOptionValue (
    _In_reads_bytes_(uPacketLen) PVOID pPacket,
    _In_ ULONG uPacketLen,
    _In_ BYTE bOption,
    _In_ ULONG uInstance,
    _Out_opt_ PBYTE pbOptionLen,
    _Outptr_opt_result_bytebuffer_(*pbOptionLen) PVOID *ppOptionValue
    );

DWORD
PXEAPI
PxeDhcpv6GetVendorOptionValue (
    _In_reads_bytes_(uPacketLen) PVOID pPacket,
    _In_ ULONG uPacketLen,
    _In_ DWORD dwEnterpriseNumber,
    _In_ WORD wOption,
    _In_ ULONG uInstance,
    _Out_opt_ PWORD pwOptionLen,
    _Outptr_opt_result_bytebuffer_(*pwOptionLen) PVOID *ppOptionValue
    );

DWORD
PXEAPI
PxeDhcpv6ParseRelayForw (
    _In_reads_bytes_(uRelayForwPacketLen) PVOID pRelayForwPacket,
    _In_ ULONG uRelayForwPacketLen,
    _Out_writes_to_(nRelayMessages,*pnRelayMessages) PPXE_DHCPV6_NESTED_RELAY_MESSAGE pRelayMessages,
    _In_ ULONG nRelayMessages,
    _Out_ PULONG pnRelayMessages,
    _Outptr_result_bytebuffer_(*pcbInnerPacket) PBYTE *ppInnerPacket,
    _Out_ PULONG pcbInnerPacket
    );

DWORD
PXEAPI
PxeDhcpv6CreateRelayRepl (
    _In_reads_(nRelayMessages) PPXE_DHCPV6_NESTED_RELAY_MESSAGE pRelayMessages,
    _In_ ULONG nRelayMessages,
    _In_reads_bytes_(cbInnerPacket) PBYTE pInnerPacket,
    _In_ ULONG cbInnerPacket,
    _Out_writes_bytes_to_(cbReplyBuffer, *pcbReplyBuffer) PVOID pReplyBuffer,
    _In_ ULONG cbReplyBuffer,
    _Out_ PULONG pcbReplyBuffer
    );

//
// Server Information
//
DWORD
PXEAPI
PxeGetServerInfo (
    _In_ PXE_GSI_TYPE uInfoType,
    _Out_writes_bytes_(uBufferLen) PVOID pBuffer,
    _In_ ULONG uBufferLen
    );

DWORD
PXEAPI
PxeGetServerInfoEx (
    _In_ PXE_GSI_TYPE uInfoType,
    _Out_writes_bytes_to_(uBufferLen, *puBufferUsed) PVOID pBuffer,
    _In_ ULONG uBufferLen,
    _Out_ PULONG puBufferUsed
    );

#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif


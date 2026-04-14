/*

Copyright (c) Microsoft Corporation.  All rights reserved.

Module Name:

    websocket.h

Abstract:

    Header files for WebSocket Protocol Component.

--*/

#ifndef __WEBSOCKET_H__
#define __WEBSOCKET_H__

#ifdef _MSC_VER
#pragma once
#endif

#ifdef __cplusplus
extern "C"
{
#endif

#include <winapifamily.h>

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#if NTDDI_VERSION >= NTDDI_WIN8

DECLARE_HANDLE(WEB_SOCKET_HANDLE);

#define WEB_SOCKET_MAX_CLOSE_REASON_LENGTH 123

typedef enum _WEB_SOCKET_CLOSE_STATUS
{
    WEB_SOCKET_SUCCESS_CLOSE_STATUS                 = 1000,
    WEB_SOCKET_ENDPOINT_UNAVAILABLE_CLOSE_STATUS    = 1001,
    WEB_SOCKET_PROTOCOL_ERROR_CLOSE_STATUS          = 1002,
    WEB_SOCKET_INVALID_DATA_TYPE_CLOSE_STATUS       = 1003,
    WEB_SOCKET_EMPTY_CLOSE_STATUS                   = 1005,
    WEB_SOCKET_ABORTED_CLOSE_STATUS                 = 1006,
    WEB_SOCKET_INVALID_PAYLOAD_CLOSE_STATUS         = 1007,
    WEB_SOCKET_POLICY_VIOLATION_CLOSE_STATUS        = 1008,
    WEB_SOCKET_MESSAGE_TOO_BIG_CLOSE_STATUS         = 1009,
    WEB_SOCKET_UNSUPPORTED_EXTENSIONS_CLOSE_STATUS  = 1010,
    WEB_SOCKET_SERVER_ERROR_CLOSE_STATUS            = 1011,
    WEB_SOCKET_SECURE_HANDSHAKE_ERROR_CLOSE_STATUS  = 1015,
} WEB_SOCKET_CLOSE_STATUS;

typedef enum _WEB_SOCKET_PROPERTY_TYPE
{
    WEB_SOCKET_RECEIVE_BUFFER_SIZE_PROPERTY_TYPE        = 0,
    WEB_SOCKET_SEND_BUFFER_SIZE_PROPERTY_TYPE           = 1,
    WEB_SOCKET_DISABLE_MASKING_PROPERTY_TYPE            = 2,
    WEB_SOCKET_ALLOCATED_BUFFER_PROPERTY_TYPE           = 3,
    WEB_SOCKET_DISABLE_UTF8_VERIFICATION_PROPERTY_TYPE  = 4,
    WEB_SOCKET_KEEPALIVE_INTERVAL_PROPERTY_TYPE         = 5,
    WEB_SOCKET_SUPPORTED_VERSIONS_PROPERTY_TYPE         = 6,
} WEB_SOCKET_PROPERTY_TYPE;

typedef enum _WEB_SOCKET_ACTION_QUEUE
{
    WEB_SOCKET_SEND_ACTION_QUEUE                    = 0x1,
    WEB_SOCKET_RECEIVE_ACTION_QUEUE                 = 0x2,
    WEB_SOCKET_ALL_ACTION_QUEUE                     = WEB_SOCKET_SEND_ACTION_QUEUE | WEB_SOCKET_RECEIVE_ACTION_QUEUE
} WEB_SOCKET_ACTION_QUEUE;

typedef enum _WEB_SOCKET_BUFFER_TYPE
{
    WEB_SOCKET_UTF8_MESSAGE_BUFFER_TYPE             = 0x80000000,
    WEB_SOCKET_UTF8_FRAGMENT_BUFFER_TYPE            = 0x80000001,
    WEB_SOCKET_BINARY_MESSAGE_BUFFER_TYPE           = 0x80000002,
    WEB_SOCKET_BINARY_FRAGMENT_BUFFER_TYPE          = 0x80000003,
    WEB_SOCKET_CLOSE_BUFFER_TYPE                    = 0x80000004,
    WEB_SOCKET_PING_PONG_BUFFER_TYPE                = 0x80000005,
    WEB_SOCKET_UNSOLICITED_PONG_BUFFER_TYPE         = 0x80000006,
} WEB_SOCKET_BUFFER_TYPE;

typedef enum _WEB_SOCKET_ACTION
{
    WEB_SOCKET_NO_ACTION                            = 0,
    WEB_SOCKET_SEND_TO_NETWORK_ACTION               = 1,
    WEB_SOCKET_INDICATE_SEND_COMPLETE_ACTION        = 2,
    WEB_SOCKET_RECEIVE_FROM_NETWORK_ACTION          = 3,
    WEB_SOCKET_INDICATE_RECEIVE_COMPLETE_ACTION     = 4,
} WEB_SOCKET_ACTION, *PWEB_SOCKET_ACTION;

typedef struct _WEB_SOCKET_PROPERTY
{
    WEB_SOCKET_PROPERTY_TYPE Type;
    PVOID pvValue;
    ULONG ulValueSize;
} WEB_SOCKET_PROPERTY, *PWEB_SOCKET_PROPERTY;

typedef struct _WEB_SOCKET_HTTP_HEADER
{
    PCHAR pcName;
    ULONG ulNameLength;
    PCHAR pcValue;
    ULONG ulValueLength;
} WEB_SOCKET_HTTP_HEADER, *PWEB_SOCKET_HTTP_HEADER;

typedef union _WEB_SOCKET_BUFFER
{
    struct
    {
        _Field_size_bytes_(ulBufferLength) PBYTE pbBuffer;
        ULONG ulBufferLength;
    } Data;

    struct
    {
        _Field_size_bytes_(ulReasonLength) PBYTE pbReason;
        _Field_range_(0, WEB_SOCKET_MAX_CLOSE_REASON_LENGTH) ULONG ulReasonLength;
        USHORT usStatus;
    } CloseStatus;
} WEB_SOCKET_BUFFER, *PWEB_SOCKET_BUFFER;

HRESULT WINAPI WebSocketCreateClientHandle(
    _In_reads_(ulPropertyCount) const PWEB_SOCKET_PROPERTY pProperties,
    _In_ ULONG ulPropertyCount,
    _Out_ WEB_SOCKET_HANDLE *phWebSocket);

HRESULT WINAPI WebSocketBeginClientHandshake(
    _In_ WEB_SOCKET_HANDLE hWebSocket,
    _In_reads_opt_(ulSubprotocolCount) PCSTR *pszSubprotocols,
    _In_ ULONG ulSubprotocolCount,
    _In_reads_opt_(ulExtensionCount) PCSTR *pszExtensions,
    _In_ ULONG ulExtensionCount,
    _In_reads_opt_(ulInitialHeaderCount) const PWEB_SOCKET_HTTP_HEADER pInitialHeaders,
    _In_ ULONG ulInitialHeaderCount,
    _Out_writes_(*pulAdditionalHeaderCount) PWEB_SOCKET_HTTP_HEADER *pAdditionalHeaders,
    _Out_ ULONG *pulAdditionalHeaderCount);

HRESULT WINAPI WebSocketEndClientHandshake(
    _In_ WEB_SOCKET_HANDLE hWebSocket,
    _In_reads_(ulReponseHeaderCount) const PWEB_SOCKET_HTTP_HEADER pResponseHeaders,
    _In_ ULONG ulReponseHeaderCount,
    _Inout_updates_to_opt_(*pulSelectedExtensionCount, *pulSelectedExtensionCount) ULONG *pulSelectedExtensions,
    _Inout_opt_ ULONG *pulSelectedExtensionCount,
    _Inout_opt_ ULONG *pulSelectedSubprotocol);

HRESULT WINAPI WebSocketCreateServerHandle(
    _In_reads_(ulPropertyCount) const PWEB_SOCKET_PROPERTY pProperties,
    _In_ ULONG ulPropertyCount,
    _Out_ WEB_SOCKET_HANDLE *phWebSocket);

HRESULT WINAPI WebSocketBeginServerHandshake(
    _In_ WEB_SOCKET_HANDLE hWebSocket,
    _In_opt_ PCSTR pszSubprotocolSelected,
    _In_reads_opt_(ulExtensionSelectedCount) PCSTR *pszExtensionSelected,
    _In_ ULONG ulExtensionSelectedCount,
    _In_reads_(ulRequestHeaderCount) const PWEB_SOCKET_HTTP_HEADER pRequestHeaders,
    _In_ ULONG ulRequestHeaderCount,
    _Out_writes_(*pulResponseHeaderCount) PWEB_SOCKET_HTTP_HEADER *pResponseHeaders,
    _Out_ ULONG* pulResponseHeaderCount);

HRESULT WINAPI WebSocketEndServerHandshake(
     _In_ WEB_SOCKET_HANDLE hWebSocket);

HRESULT WINAPI WebSocketSend(
    _In_ WEB_SOCKET_HANDLE hWebSocket,
    _In_ WEB_SOCKET_BUFFER_TYPE BufferType,
    _In_opt_ WEB_SOCKET_BUFFER *pBuffer,
    _In_opt_ PVOID Context);

HRESULT WINAPI WebSocketReceive(
    _In_ WEB_SOCKET_HANDLE hWebSocket,
    _In_opt_ WEB_SOCKET_BUFFER *pBuffer,
    _In_opt_ PVOID pvContext);

HRESULT WINAPI WebSocketGetAction(
    _In_ WEB_SOCKET_HANDLE hWebSocket,
    _In_ WEB_SOCKET_ACTION_QUEUE eActionQueue,
    _Inout_updates_to_(*pulDataBufferCount, *pulDataBufferCount) WEB_SOCKET_BUFFER *pDataBuffers,
    _Inout_ ULONG *pulDataBufferCount,
    _Out_ WEB_SOCKET_ACTION *pAction,
    _Out_ WEB_SOCKET_BUFFER_TYPE *pBufferType,
    _Out_opt_ PVOID *pvApplicationContext,
    _Out_ PVOID *pvActionContext);

VOID WINAPI WebSocketCompleteAction(
    _In_ WEB_SOCKET_HANDLE hWebSocket,
    _In_ PVOID pvActionContext,
    _In_ ULONG ulBytesTransferred);

VOID WINAPI WebSocketAbortHandle(
    _In_ WEB_SOCKET_HANDLE hWebSocket);

VOID WINAPI WebSocketDeleteHandle(
    _In_ WEB_SOCKET_HANDLE hWebSocket);

HRESULT WINAPI WebSocketGetGlobalProperty(
    _In_ WEB_SOCKET_PROPERTY_TYPE eType,
    _Inout_updates_to_(*ulSize, *ulSize) PVOID pvValue,
    _Inout_ ULONG* ulSize);

#endif //NTDDI_VERSION >= NTDDI_WIN8

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#ifdef __cplusplus
}
#endif

#endif //__WEBSOCKET_H__

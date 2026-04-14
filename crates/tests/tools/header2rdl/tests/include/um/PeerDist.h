/*++

Copyright (c) Microsoft Corporation.  All rights reserved.

Module Name:

    peerdist.h

Abstract:

    Peer Distribution APIs and definitions

--*/


#ifndef _PEERDIST_H_
#define _PEERDIST_H_

#ifndef MIDL_PASS
#include <windows.h>
#endif

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#ifdef __cplusplus

extern "C" {

#endif // __cplusplus

#if NTDDI_VERSION >= NTDDI_WIN7

#define PEERDIST_PUBLICATION_OPTIONS_VERSION_1  1L

#if NTDDI_VERSION >= NTDDI_WIN8

    #define PEERDIST_PUBLICATION_OPTIONS_VERSION    2L
    #define PEERDIST_PUBLICATION_OPTIONS_VERSION_2  2L

    #define PEERDIST_VERSION ((DWORD)MAKELONG(2,0))

#else // Indicates NTDDI_VERSION < NTDDI_WIN8

    #define PEERDIST_PUBLICATION_OPTIONS_VERSION    1L

    #define PEERDIST_VERSION MAKEWORD(1,0)

#endif // NTDDI_VERSION >= NTDDI_WIN8


typedef enum
{
    PEERDIST_STATUS_DISABLED,
    PEERDIST_STATUS_UNAVAILABLE,
    PEERDIST_STATUS_AVAILABLE,
} PEERDIST_STATUS;

typedef struct peerdist_publication_options_tag
{
    DWORD dwVersion;
    DWORD dwFlags;
}
PEERDIST_PUBLICATION_OPTIONS, * PPEERDIST_PUBLICATION_OPTIONS;
typedef const PEERDIST_PUBLICATION_OPTIONS * PCPEERDIST_PUBLICATION_OPTIONS;

typedef struct peerdist_content_tag_tag
{
    BYTE      Data[ 16 ];
} PEERDIST_CONTENT_TAG, *PPEERDIST_CONTENT_TAG;
typedef const PEERDIST_CONTENT_TAG * PCPEERDIST_CONTENT_TAG;

#define PEERDIST_READ_TIMEOUT_LOCAL_CACHE_ONLY 0UL

#define PEERDIST_READ_TIMEOUT_DEFAULT 0xfffffffeUL

#ifndef MIDL_PASS

typedef HANDLE      PEERDIST_INSTANCE_HANDLE;
typedef PEERDIST_INSTANCE_HANDLE*    PPEERDIST_INSTANCE_HANDLE;

typedef HANDLE      PEERDIST_CONTENT_HANDLE;
typedef PEERDIST_CONTENT_HANDLE *   PPEERDIST_CONTENT_HANDLE;

typedef HANDLE      PEERDIST_CONTENTINFO_HANDLE;
typedef PEERDIST_CONTENTINFO_HANDLE * PPEERDIST_CONTENTINFO_HANDLE;

typedef HANDLE      PEERDIST_STREAM_HANDLE;
typedef PEERDIST_STREAM_HANDLE *     PPEERDIST_STREAM_HANDLE;


DWORD WINAPI PeerDistStartup(
        _In_  DWORD  dwVersionRequested,
        _Out_ PPEERDIST_INSTANCE_HANDLE phPeerDist,
        _Out_opt_ PDWORD pdwSupportedVersion
        );

DWORD WINAPI PeerDistShutdown(
        _In_ PEERDIST_INSTANCE_HANDLE hPeerDist
        );

DWORD WINAPI PeerDistGetStatus(
        _In_  PEERDIST_INSTANCE_HANDLE  hPeerDist, 
        _Out_ PEERDIST_STATUS * pPeerDistStatus
        );

DWORD WINAPI PeerDistRegisterForStatusChangeNotification(
        _In_     PEERDIST_INSTANCE_HANDLE hPeerDist,
        _In_opt_ HANDLE hCompletionPort,
        _In_opt_ ULONG_PTR ulCompletionKey,    
        _In_     LPOVERLAPPED lpOverlapped,
        _Out_    PEERDIST_STATUS * pPeerDistStatus
        );

DWORD WINAPI PeerDistUnregisterForStatusChangeNotification(
    _In_ PEERDIST_INSTANCE_HANDLE hPeerDist
    );

DWORD WINAPI PeerDistServerPublishStream(
        _In_ PEERDIST_INSTANCE_HANDLE  hPeerDist,
             DWORD cbContentIdentifier,
        _In_reads_bytes_(cbContentIdentifier) PBYTE pContentIdentifier,
             ULONGLONG cbContentLength,
        _In_opt_ PCPEERDIST_PUBLICATION_OPTIONS pPublishOptions,
        _In_opt_ HANDLE hCompletionPort,
        _In_opt_ ULONG_PTR ulCompletionKey,
        _Out_ PPEERDIST_STREAM_HANDLE phStream
        );

DWORD WINAPI PeerDistServerPublishAddToStream(
        _In_ PEERDIST_INSTANCE_HANDLE  hPeerDist,
        _In_ PEERDIST_STREAM_HANDLE hStream,
             DWORD cbNumberOfBytes,
        _In_reads_bytes_(cbNumberOfBytes) PBYTE pBuffer,
        _In_ LPOVERLAPPED lpOverlapped
        );

DWORD WINAPI PeerDistServerPublishCompleteStream(
        _In_ PEERDIST_INSTANCE_HANDLE  hPeerDist,
        _In_ PEERDIST_STREAM_HANDLE hStream,
        _In_ LPOVERLAPPED lpOverlapped
        );

DWORD WINAPI PeerDistServerCloseStreamHandle(
        _In_ PEERDIST_INSTANCE_HANDLE hPeerDist,
        _In_ PEERDIST_STREAM_HANDLE hStream
        );

DWORD WINAPI PeerDistServerUnpublish(
        _In_ PEERDIST_INSTANCE_HANDLE  hPeerDist,
             DWORD cbContentIdentifier,
        _In_reads_bytes_(cbContentIdentifier) PBYTE pContentIdentifier
        );

DWORD WINAPI PeerDistServerOpenContentInformation(
        _In_ PEERDIST_INSTANCE_HANDLE  hPeerDist,
             DWORD cbContentIdentifier,
        _In_reads_bytes_(cbContentIdentifier) PBYTE pContentIdentifier,
             ULONGLONG ullContentOffset,
             ULONGLONG cbContentLength,
        _In_opt_ HANDLE hCompletionPort,
        _In_opt_ ULONG_PTR ulCompletionKey,
        _Out_ PPEERDIST_CONTENTINFO_HANDLE phContentInfo
        );

DWORD WINAPI PeerDistServerRetrieveContentInformation(
        _In_ PEERDIST_INSTANCE_HANDLE  hPeerDist,
        _In_ PEERDIST_CONTENTINFO_HANDLE hContentInfo,
             DWORD cbMaxNumberOfBytes,
        _Out_writes_bytes_(cbMaxNumberOfBytes) PBYTE pBuffer,
        _In_ LPOVERLAPPED lpOverlapped  // Offset and OffsetHigh are offset into content info
        );

DWORD WINAPI PeerDistServerCloseContentInformation(
        _In_ PEERDIST_INSTANCE_HANDLE  hPeerDist,
        _In_ PEERDIST_CONTENTINFO_HANDLE hContentInfo
        );

DWORD WINAPI PeerDistServerCancelAsyncOperation(
        _In_ PEERDIST_INSTANCE_HANDLE hPeerDist,
             DWORD cbContentIdentifier,
        _In_reads_bytes_(cbContentIdentifier) PBYTE pContentIdentifier,
        _In_ LPOVERLAPPED pOverlapped
        );

DWORD WINAPI PeerDistClientOpenContent(
        _In_ PEERDIST_INSTANCE_HANDLE hPeerDist,
        _In_ PCPEERDIST_CONTENT_TAG pContentTag,
        _In_opt_ HANDLE hCompletionPort,
        _In_opt_ ULONG_PTR ulCompletionKey,
        _Out_ PPEERDIST_CONTENT_HANDLE phContentHandle
        );

DWORD WINAPI PeerDistClientCloseContent(
        _In_ PEERDIST_INSTANCE_HANDLE hPeerDist,
        _In_ PEERDIST_CONTENT_HANDLE hContentHandle
        );

DWORD WINAPI PeerDistClientAddContentInformation(
        _In_ PEERDIST_INSTANCE_HANDLE hPeerDist,
        _In_ PEERDIST_CONTENT_HANDLE hContentHandle,
             DWORD cbNumberOfBytes,
        _In_reads_bytes_(cbNumberOfBytes) PBYTE pBuffer,
        _In_ LPOVERLAPPED lpOverlapped
        );

DWORD WINAPI PeerDistClientCompleteContentInformation(
        _In_ PEERDIST_INSTANCE_HANDLE hPeerDist,
        _In_ PEERDIST_CONTENT_HANDLE hContentHandle,
        _In_ LPOVERLAPPED lpOverlapped
        );

DWORD WINAPI PeerDistClientAddData(
        _In_ PEERDIST_INSTANCE_HANDLE hPeerDist,
        _In_ PEERDIST_CONTENT_HANDLE hContentHandle,
             DWORD cbNumberOfBytes,
        _In_reads_bytes_(cbNumberOfBytes) PBYTE pBuffer,
        _In_ LPOVERLAPPED lpOverlapped
        );

DWORD WINAPI PeerDistClientBlockRead(
        _In_ PEERDIST_INSTANCE_HANDLE hPeerDist,
        _In_ PEERDIST_CONTENT_HANDLE hContentHandle,
             DWORD cbMaxNumberOfBytes,
        _Out_writes_bytes_opt_(cbMaxNumberOfBytes) PBYTE pBuffer,
             DWORD dwTimeoutInMilliseconds,
        _In_ LPOVERLAPPED lpOverlapped
        );

DWORD WINAPI PeerDistClientStreamRead(
        _In_ PEERDIST_INSTANCE_HANDLE hPeerDist,
        _In_ PEERDIST_CONTENT_HANDLE hContentHandle,
             DWORD cbMaxNumberOfBytes,
        _Out_writes_bytes_opt_(cbMaxNumberOfBytes) PBYTE pBuffer,
             DWORD dwTimeoutInMilliseconds,
        _In_ LPOVERLAPPED lpOverlapped
        );

DWORD WINAPI PeerDistClientFlushContent(
        _In_ PEERDIST_INSTANCE_HANDLE hPeerDist,
        _In_ PCPEERDIST_CONTENT_TAG pContentTag,
        _In_opt_ HANDLE hCompletionPort,
        _In_opt_ ULONG_PTR ulCompletionKey,
        _In_ LPOVERLAPPED lpOverlapped
        );

DWORD WINAPI PeerDistClientCancelAsyncOperation(
        _In_ PEERDIST_INSTANCE_HANDLE hPeerDist,
        _In_ PEERDIST_CONTENT_HANDLE hContentHandle,
        _In_opt_ LPOVERLAPPED pOverlapped
        );

#endif // MIDL_PASS

#endif // NTDDI_VERSION >= NTDDI_WIN7

#if NTDDI_VERSION >= NTDDI_WIN8

#define PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION   2L
#define PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_1 1L
#define PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_2 2L

typedef struct peerdist_retrieval_options_tag 
{
    DWORD cbSize;
    DWORD dwContentInfoMinVersion;
    DWORD dwContentInfoMaxVersion;
    DWORD dwReserved;
} PEERDIST_RETRIEVAL_OPTIONS, *PPEERDIST_RETRIEVAL_OPTIONS;
typedef const PEERDIST_RETRIEVAL_OPTIONS * PCPEERDIST_RETRIEVAL_OPTIONS;

typedef struct peerdist_status_info_tag
{
    DWORD cbSize;
    PEERDIST_STATUS status;
    DWORD dwMinVer;
    DWORD dwMaxVer;
} PEERDIST_STATUS_INFO, *PPEERDIST_STATUS_INFO;
typedef const PEERDIST_STATUS_INFO * PCPEERDIST_STATUS_INFO;

typedef enum _PEERDIST_CLIENT_INFO_BY_HANDLE_CLASS {
    PeerDistClientBasicInfo                    = 0,
    MaximumPeerDistClientInfoByHandlesClass 
} PEERDIST_CLIENT_INFO_BY_HANDLE_CLASS, *PPEERDIST_CLIENT_INFO_BY_HANDLE_CLASS;

typedef struct _PEERDIST_CLIENT_BASIC_INFO {
    BOOL  fFlashCrowd;
} PEERDIST_CLIENT_BASIC_INFO, *PPEERDIST_CLIENT_BASIC_INFO;

#ifndef MIDL_PASS

DWORD WINAPI PeerDistGetStatusEx(
        _In_  PEERDIST_INSTANCE_HANDLE hPeerDist,
        _Inout_ PEERDIST_STATUS_INFO *pPeerDistStatus
        );

DWORD WINAPI PeerDistRegisterForStatusChangeNotificationEx(
        _In_      PEERDIST_INSTANCE_HANDLE hPeerDist,
        _In_opt_  HANDLE hCompletionPort,
        _In_opt_  ULONG_PTR ulCompletionKey,
        _In_      LPOVERLAPPED lpOverlapped,
        _Inout_ PEERDIST_STATUS_INFO *pPeerDistStatus
        );

BOOL WINAPI PeerDistGetOverlappedResult(
        _In_ LPOVERLAPPED lpOverlapped,
        _Out_ LPDWORD lpNumberOfBytesTransferred,
        _In_ BOOL bWait
        );

DWORD WINAPI PeerDistServerOpenContentInformationEx(
        _In_ PEERDIST_INSTANCE_HANDLE  hPeerDist,
             DWORD cbContentIdentifier,
        _In_reads_bytes_(cbContentIdentifier) PBYTE pContentIdentifier,
             ULONGLONG ullContentOffset,  
             ULONGLONG cbContentLength,
        _In_ PEERDIST_RETRIEVAL_OPTIONS *pRetrievalOptions,
        _In_opt_ HANDLE hCompletionPort,
        _In_opt_ ULONG_PTR ulCompletionKey,
        _Out_ PPEERDIST_CONTENTINFO_HANDLE phContentInfo
        );

DWORD WINAPI PeerDistClientGetInformationByHandle(
        _In_  PEERDIST_INSTANCE_HANDLE hPeerDist,
        _In_  PEERDIST_CONTENT_HANDLE hContentHandle,
        _In_  PEERDIST_CLIENT_INFO_BY_HANDLE_CLASS PeerDistClientInfoClass,
            DWORD dwBufferSize,
        _Out_writes_bytes_(dwBufferSize) LPVOID lpInformation
        ); 

#endif // MIDL_PASS

#endif // NTDDI_VERSION >= NTDDI_WIN8

#ifdef __cplusplus

} // extern "C"

#endif // __cplusplus

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _PEERDIST_H_

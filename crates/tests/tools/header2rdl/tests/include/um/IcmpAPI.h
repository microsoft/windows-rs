/*++

Copyright (c) 1991-1999  Microsoft Corporation

Module Name:

    icmpapi.h

Abstract:

    Declarations for the Win32 ICMP Echo request API.

Author:

    Portable Systems Group 30-December-1993

Revision History:


Notes:

--*/

#ifndef _ICMP_INCLUDED_
#define _ICMP_INCLUDED_

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

//
// Establish DLL function linkage if supported by the current build
// environment and not previously defined.
//

#ifndef IPHLPAPI_DLL_LINKAGE
#ifdef DECLSPEC_IMPORT
#define IPHLPAPI_DLL_LINKAGE DECLSPEC_IMPORT
#else
#define IPHLPAPI_DLL_LINKAGE
#endif
#endif

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)


#ifdef __cplusplus
extern "C" {
#endif

//
// Exported Routines.
//

//++
//
// Routine Name:
//
//     IcmpCreateFile
//
// Routine Description:
//
//     Opens a handle on which ICMP Echo Requests can be issued.
//
// Arguments:
//
//     None.
//
// Return Value:
//
//     An open file handle or INVALID_HANDLE_VALUE. Extended error information
//     is available by calling GetLastError().
//
//--

IPHLPAPI_DLL_LINKAGE
HANDLE
WINAPI
IcmpCreateFile(
    VOID
    );

#if (NTDDI_VERSION >= NTDDI_WINXP)
//++
//
// Routine Name:
//
//     Icmp6CreateFile
//
// Routine Description:
//
//     Opens a handle on which ICMPv6 Echo Requests can be issued.
//
// Arguments:
//
//     None.
//
// Return Value:
//
//     An open file handle or INVALID_HANDLE_VALUE. Extended error information
//     is available by calling GetLastError().
//
//--

IPHLPAPI_DLL_LINKAGE
HANDLE
WINAPI
Icmp6CreateFile(
    VOID
    );
#endif

//++
//
// Routine Name:
//
//     IcmpCloseHandle
//
// Routine Description:
//
//     Closes a handle opened by ICMPOpenFile.
//
// Arguments:
//
//     IcmpHandle  - The handle to close.
//
// Return Value:
//
//     TRUE if the handle was closed successfully, otherwise FALSE. Extended
//     error information is available by calling GetLastError().
//
//--

IPHLPAPI_DLL_LINKAGE
BOOL
WINAPI
IcmpCloseHandle(
    _In_ HANDLE  IcmpHandle
    );

//++
//
// Routine Name:
//
//     IcmpSendEcho
//
// Routine Description:
//
//     Sends an ICMP Echo request and returns any replies. The
//     call returns when the timeout has expired or the reply buffer
//     is filled.
//
// Arguments:
//
//     IcmpHandle           - An open handle returned by ICMPCreateFile.
//
//     DestinationAddress   - The destination of the echo request.
//
//     RequestData          - A buffer containing the data to send in the
//                            request.
//
//     RequestSize          - The number of bytes in the request data buffer.
//
//     RequestOptions       - Pointer to the IP header options for the request.
//                            May be NULL.
//
//     ReplyBuffer          - A buffer to hold any replies to the request.
//                            On return, the buffer will contain an array of
//                            ICMP_ECHO_REPLY structures followed by the
//                            options and data for the replies. The buffer
//                            should be large enough to hold at least one
//                            ICMP_ECHO_REPLY structure plus
//                            MAX(RequestSize, 8) bytes of data since an ICMP
//                            error message contains 8 bytes of data.
//
//     ReplySize            - The size in bytes of the reply buffer.
//
//     Timeout              - The time in milliseconds to wait for replies.
//
// Return Value:
//
//     Returns the number of ICMP_ECHO_REPLY structures stored in ReplyBuffer.
//     The status of each reply is contained in the structure. If the return
//     value is zero, extended error information is available via
//     GetLastError().
//
//--

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
IcmpSendEcho(
    _In_                       HANDLE                   IcmpHandle,
    _In_                       IPAddr                   DestinationAddress,
    _In_reads_bytes_(RequestSize)   LPVOID                   RequestData,
    _In_                       WORD                     RequestSize,
    _In_opt_                   PIP_OPTION_INFORMATION   RequestOptions,
    _Out_writes_bytes_(ReplySize)    LPVOID                   ReplyBuffer,
    _In_range_(>=, sizeof(ICMP_ECHO_REPLY) + RequestSize + 8)
    DWORD                    ReplySize,
    _In_                       DWORD                    Timeout
    );


//++
//
// Routine Description:
//
//    Sends an ICMP Echo request and the call returns either immediately
//    (if Event or ApcRoutine is NonNULL) or returns after the specified
//    timeout.   The ReplyBuffer contains the ICMP responses, if any.
//
// Arguments:
//
//    IcmpHandle           - An open handle returned by ICMPCreateFile.
//
//    Event                - This is the event to be signalled whenever an IcmpResponse
//                           comes in.
//
//    ApcRoutine           - This routine would be called when the calling thread
//                           is in an alertable thread and an ICMP reply comes in.
//
//    ApcContext           - This optional parameter is given to the ApcRoutine when
//                           this call succeeds.
//
//    DestinationAddress   - The destination of the echo request.
//
//    RequestData          - A buffer containing the data to send in the
//                           request.
//
//    RequestSize          - The number of bytes in the request data buffer.
//
//    RequestOptions       - Pointer to the IP header options for the request.
//                           May be NULL.
//
//    ReplyBuffer          - A buffer to hold any replies to the request.
//                           On return, the buffer will contain an array of
//                           ICMP_ECHO_REPLY structures followed by options
//                           and data. The buffer must be large enough to
//                           hold at least one ICMP_ECHO_REPLY structure.
//                           It should be large enough to also hold
//                           8 more bytes of data - this is the size of
//                           an ICMP error message.
//
//    ReplySize            - The size in bytes of the reply buffer.
//
//    Timeout              - The time in milliseconds to wait for replies.
//                           This is NOT used if ApcRoutine is not NULL or if Event
//                           is not NULL.
//
// Return Value:
//
//    Returns the number of replies received and stored in ReplyBuffer. If
//    the return value is zero, extended error information is available
//    via GetLastError().
//
// Remarks:
//
//    On NT platforms,
//    If used Asynchronously (either ApcRoutine or Event is specified), then
//    ReplyBuffer and ReplySize are still needed.  This is where the response
//    comes in.
//    ICMP Response data is copied to the ReplyBuffer provided, and the caller of
//    this function has to parse it asynchronously.  The function IcmpParseReply
//    is provided for this purpose.
//
//    On non-NT platforms,
//    Event, ApcRoutine and ApcContext are IGNORED.
//
//--

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
IcmpSendEcho2(
    _In_                        HANDLE                   IcmpHandle,
    _In_opt_                    HANDLE                   Event,
#ifdef PIO_APC_ROUTINE_DEFINED
    _In_opt_                    PIO_APC_ROUTINE          ApcRoutine,
#else
    _In_opt_                    FARPROC                  ApcRoutine,
#endif
    _In_opt_                    PVOID                    ApcContext,
    _In_                        IPAddr                   DestinationAddress,
    _In_reads_bytes_(RequestSize)    LPVOID                   RequestData,
    _In_                        WORD                     RequestSize,
    _In_opt_                    PIP_OPTION_INFORMATION   RequestOptions,
    _Out_writes_bytes_(ReplySize)     LPVOID                   ReplyBuffer,
    _In_range_(>=, sizeof(ICMP_ECHO_REPLY) + RequestSize + 8)
    DWORD                    ReplySize,
    _In_                        DWORD                    Timeout
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#if (NTDDI_VERSION >= NTDDI_VISTASP1)
IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
IcmpSendEcho2Ex(
    _In_                       HANDLE                   IcmpHandle,
    _In_opt_                   HANDLE                   Event,
#ifdef PIO_APC_ROUTINE_DEFINED
    _In_opt_                   PIO_APC_ROUTINE          ApcRoutine,
#else
    _In_opt_                   FARPROC                  ApcRoutine,
#endif
    _In_opt_                   PVOID                    ApcContext,
    _In_                       IPAddr                   SourceAddress,
    _In_                       IPAddr                   DestinationAddress,
    _In_reads_bytes_(RequestSize)   LPVOID                   RequestData,
    _In_                       WORD                     RequestSize,
    _In_opt_                   PIP_OPTION_INFORMATION   RequestOptions,
    _Out_writes_bytes_(ReplySize)    LPVOID                   ReplyBuffer,
    _In_range_(>=, sizeof(ICMP_ECHO_REPLY) + RequestSize + 8)
    DWORD                    ReplySize,
    _In_                       DWORD                    Timeout
    );
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

#if (NTDDI_VERSION >= NTDDI_WINXP)
IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
Icmp6SendEcho2(
    _In_                     HANDLE                   IcmpHandle,
    _In_opt_                 HANDLE                   Event,
#ifdef PIO_APC_ROUTINE_DEFINED
    _In_opt_                 PIO_APC_ROUTINE          ApcRoutine,
#else
    _In_opt_                 FARPROC                  ApcRoutine,
#endif
    _In_opt_                 PVOID                    ApcContext,
    _In_                     struct sockaddr_in6     *SourceAddress,
    _In_                     struct sockaddr_in6     *DestinationAddress,
    _In_reads_bytes_(RequestSize) LPVOID                   RequestData,
    _In_                     WORD                     RequestSize,
    _In_opt_                 PIP_OPTION_INFORMATION   RequestOptions,
    _Out_writes_bytes_(ReplySize)  LPVOID                   ReplyBuffer,
    _In_range_(>=, sizeof(ICMPV6_ECHO_REPLY) + RequestSize + 8)
    DWORD                    ReplySize,
    _In_                     DWORD                    Timeout
    );
#endif

//++
//
// Routine Description:
//
//    Parses the reply buffer provided and returns the number of ICMP responses found.
//
// Arguments:
//
//    ReplyBuffer            - This must be the same buffer that was passed to IcmpSendEcho2
//                             This is rewritten to hold an array of ICMP_ECHO_REPLY structures.
//                             (i.e. the type is PICMP_ECHO_REPLY).
//
//    ReplySize              - This must be the size of the above buffer.
//
// Return Value:
//    Returns the number of ICMP responses found.  If there is an errors, return value is
//    zero.  The error can be determined by a call to GetLastError.
//
// Remarks:
//    This function SHOULD NOT BE USED on a reply buffer that was passed to SendIcmpEcho.
//    SendIcmpEcho actually parses the buffer before returning back to the user.  This function
//    is meant to be used only with SendIcmpEcho2.
//--

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
IcmpParseReplies(
    _Out_writes_bytes_(ReplySize)   LPVOID                   ReplyBuffer,
    _In_range_(>=, sizeof(ICMP_ECHO_REPLY) + 8)
    DWORD                    ReplySize
    );

#if (NTDDI_VERSION >= NTDDI_WINXP)
IPHLPAPI_DLL_LINKAGE
DWORD
WINAPI
Icmp6ParseReplies(
    _Out_writes_bytes_(ReplySize)   LPVOID                   ReplyBuffer,
    _In_range_(>, sizeof(ICMPV6_ECHO_REPLY) + 8)
    DWORD                    ReplySize
    );
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif // _ICMP_INCLUDED_




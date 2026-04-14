/*++

Copyright (c) 1989-2002  Microsoft Corporation

Module Name:

    fltUser.h

Abstract:
    Header file which contains the structures, type definitions,
    constants, global variables and function prototypes that are
    visible to user mode applications that interact with filters.

Environment:

    User mode

--*/
#ifndef __FLTUSER_H__
#define __FLTUSER_H__
#include <winapifamily.h>

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)


//
// IMPORTANT!!!!!
//
// This is how FltMgr was released (from oldest to newest)
// xpsp2, (srv03, w2ksp4+security rollup package(srp)), Vista, Win7, Win8
//

//
//  The defines items that are part of the filter manager baseline
//

#define FLT_MGR_BASELINE (((OSVER(NTDDI_VERSION) == NTDDI_WIN2K) && (SPVER(NTDDI_VERSION) >= SPVER(NTDDI_WIN2KSP4))) || \
                          ((OSVER(NTDDI_VERSION) == NTDDI_WINXP) && (SPVER(NTDDI_VERSION) >= SPVER(NTDDI_WINXPSP2))) || \
                          ((OSVER(NTDDI_VERSION) == NTDDI_WS03)  && (SPVER(NTDDI_VERSION) >= SPVER(NTDDI_WS03SP1))) ||  \
                          (NTDDI_VERSION >= NTDDI_VISTA))

//
//  This defines items that were added after XPSP2 was released.  This means
//  they are in Srv03 SP1, W2K SP4+URP, and Longhorn and above.
//

#define FLT_MGR_AFTER_XPSP2 (((OSVER(NTDDI_VERSION) == NTDDI_WIN2K) && (SPVER(NTDDI_VERSION) >= SPVER(NTDDI_WIN2KSP4))) ||  \
                             ((OSVER(NTDDI_VERSION) == NTDDI_WINXP) && (SPVER(NTDDI_VERSION) >  SPVER(NTDDI_WINXPSP2))) ||  \
                             ((OSVER(NTDDI_VERSION) == NTDDI_WS03)  && (SPVER(NTDDI_VERSION) >= SPVER(NTDDI_WS03SP1))) ||   \
                             (NTDDI_VERSION >= NTDDI_VISTA))

//
//  This defines items that only exist in longhorn or later
//

#define FLT_MGR_LONGHORN (NTDDI_VERSION >= NTDDI_VISTA)

//
//  This defines items that only exist in Windows 7 or later
//

#define FLT_MGR_WIN7 (NTDDI_VERSION >= NTDDI_WIN7)

//
//  This defines items that only exist in Windows 8 or later
//

#define FLT_MGR_WIN8 (NTDDI_VERSION >= NTDDI_WIN8)

//
//  This defines items that only exist in Windows Blue or later
//

#define FLT_MGR_WINBLUE (NTDDI_VERSION >= NTDDI_WINBLUE)


///////////////////////////////////////////////////////////////////////////////
//
//  Standard includes
//
///////////////////////////////////////////////////////////////////////////////

#include <fltUserStructures.h>

#ifdef __cplusplus
extern "C" {
#endif

//
// These are all of the baseline set of user-mode functions in FltMgr.
//

#if FLT_MGR_BASELINE

//
//  Version-differential support for ASSERTs
//  (NT_ASSERT on Vista and later, ASSERT otherwise)
//

#if FLT_MGR_LONGHORN

#define FLT_ASSERT(_e) NT_ASSERT(_e)
#define FLT_ASSERTMSG(_m, _e) NT_ASSERTMSG(_m, _e)

#else

#define FLT_ASSERT(_e) ASSERT(_e)
#define FLT_ASSERTMSG(_m, _e) ASSERTMSG(_m, _e)

#endif

//
//  Functions for loading, unloading and monitoring Filters
//

_Must_inspect_result_
HRESULT
WINAPI
FilterLoad (
    _In_ LPCWSTR lpFilterName
    );

_Must_inspect_result_
HRESULT
WINAPI
FilterUnload (
    _In_ LPCWSTR lpFilterName
    );


//****************************************************************************
//
//  Functions for creating and closing handles
//
//****************************************************************************

//
//  Filter
//

_Must_inspect_result_
HRESULT
WINAPI
FilterCreate (
    _In_ LPCWSTR lpFilterName,
    _Outptr_ HFILTER *hFilter
    );

HRESULT
WINAPI
FilterClose(
    _In_ HFILTER hFilter
    );

//
//  FilterInstance
//

_Must_inspect_result_
HRESULT
WINAPI
FilterInstanceCreate (
    _In_ LPCWSTR lpFilterName,
    _In_ LPCWSTR lpVolumeName,
    _In_opt_ LPCWSTR lpInstanceName,
   _Outptr_ HFILTER_INSTANCE *hInstance
    );

HRESULT
WINAPI
FilterInstanceClose(
    _In_ HFILTER_INSTANCE hInstance
    );


//****************************************************************************
//
//  Functions for creating and deleting FilterInstances in the
//  device stack.
//
//****************************************************************************

_Must_inspect_result_
HRESULT
WINAPI
FilterAttach (
    _In_ LPCWSTR lpFilterName,
    _In_ LPCWSTR lpVolumeName,
    _In_opt_ LPCWSTR lpInstanceName ,
    _In_opt_ DWORD dwCreatedInstanceNameLength ,
    _Out_writes_bytes_opt_(dwCreatedInstanceNameLength) LPWSTR lpCreatedInstanceName
    );

_Must_inspect_result_
HRESULT
WINAPI
FilterAttachAtAltitude (
    _In_ LPCWSTR lpFilterName,
    _In_ LPCWSTR lpVolumeName,
    _In_ LPCWSTR lpAltitude,
    _In_opt_ LPCWSTR lpInstanceName ,
    _In_opt_ DWORD dwCreatedInstanceNameLength ,
    _Out_writes_bytes_opt_(dwCreatedInstanceNameLength) LPWSTR lpCreatedInstanceName
    );

_Must_inspect_result_
HRESULT
WINAPI
FilterDetach (
    _In_ LPCWSTR lpFilterName,
    _In_ LPCWSTR lpVolumeName,
    _In_opt_ LPCWSTR lpInstanceName
    );


//****************************************************************************
//
//  Functions for iterating through Filters and FilterInstances and
//  getting information on a Filter or FilterInstance.
//
//****************************************************************************

//
//  Functions for iterating through Filters
//

_Must_inspect_result_
HRESULT
WINAPI
FilterFindFirst (
    _In_ FILTER_INFORMATION_CLASS dwInformationClass,
    _Out_writes_bytes_to_(dwBufferSize,*lpBytesReturned) LPVOID lpBuffer,
    _In_ DWORD dwBufferSize,
    _Out_ LPDWORD lpBytesReturned,
    _Out_ LPHANDLE lpFilterFind
    );

_Must_inspect_result_
HRESULT
WINAPI
FilterFindNext (
    _In_ HANDLE hFilterFind,
    _In_ FILTER_INFORMATION_CLASS dwInformationClass,
    _Out_writes_bytes_to_(dwBufferSize,*lpBytesReturned) LPVOID lpBuffer,
    _In_ DWORD dwBufferSize,
    _Out_ LPDWORD lpBytesReturned
    );

_Must_inspect_result_
HRESULT
WINAPI
FilterFindClose(
    _In_ HANDLE hFilterFind
    );


_Must_inspect_result_
HRESULT
WINAPI
FilterVolumeFindFirst (
    _In_ FILTER_VOLUME_INFORMATION_CLASS dwInformationClass,
    _Out_writes_bytes_to_(dwBufferSize,*lpBytesReturned) LPVOID lpBuffer,
    _In_ DWORD dwBufferSize,
    _Out_ LPDWORD lpBytesReturned,
    _Out_ PHANDLE lpVolumeFind
    );

_Must_inspect_result_
HRESULT
WINAPI
FilterVolumeFindNext (
    _In_ HANDLE hVolumeFind,
    _In_ FILTER_VOLUME_INFORMATION_CLASS dwInformationClass,
    _Out_writes_bytes_to_(dwBufferSize,*lpBytesReturned) LPVOID lpBuffer,
    _In_ DWORD dwBufferSize,
    _Out_ LPDWORD lpBytesReturned
    );

HRESULT
WINAPI
FilterVolumeFindClose(
    _In_ HANDLE hVolumeFind
    );

//
//  Functions for iterating through FilterInstances
//

_Must_inspect_result_
HRESULT
WINAPI
FilterInstanceFindFirst (
    _In_ LPCWSTR lpFilterName,
    _In_ INSTANCE_INFORMATION_CLASS dwInformationClass,
    _Out_writes_bytes_to_(dwBufferSize,*lpBytesReturned) LPVOID lpBuffer,
    _In_ DWORD dwBufferSize,
    _Out_ LPDWORD lpBytesReturned,
    _Out_ LPHANDLE lpFilterInstanceFind
    );

_Must_inspect_result_
HRESULT
WINAPI
FilterInstanceFindNext (
    _In_ HANDLE hFilterInstanceFind,
    _In_ INSTANCE_INFORMATION_CLASS dwInformationClass,
    _Out_writes_bytes_to_(dwBufferSize,*lpBytesReturned) LPVOID lpBuffer,
    _In_ DWORD dwBufferSize,
    _Out_ LPDWORD lpBytesReturned
    );

_Must_inspect_result_
HRESULT
WINAPI
FilterInstanceFindClose(
    _In_ HANDLE hFilterInstanceFind
    );


//
//  Functions for iterating through VolumeInstances
//

_Must_inspect_result_
HRESULT
WINAPI
FilterVolumeInstanceFindFirst (
    _In_ LPCWSTR lpVolumeName,
    _In_ INSTANCE_INFORMATION_CLASS dwInformationClass,
    _Out_writes_bytes_to_(dwBufferSize,*lpBytesReturned) LPVOID lpBuffer,
    _In_ DWORD dwBufferSize,
    _Out_ LPDWORD lpBytesReturned,
    _Out_ LPHANDLE lpVolumeInstanceFind
    );

_Must_inspect_result_
HRESULT
WINAPI
FilterVolumeInstanceFindNext (
    _In_ HANDLE hVolumeInstanceFind,
    _In_ INSTANCE_INFORMATION_CLASS dwInformationClass,
    _Out_writes_bytes_to_(dwBufferSize,*lpBytesReturned) LPVOID lpBuffer,
    _In_ DWORD dwBufferSize,
    _Out_ LPDWORD lpBytesReturned
    );

HRESULT
WINAPI
FilterVolumeInstanceFindClose(
    _In_ HANDLE hVolumeInstanceFind
    );


//
//  Functions for getting information on Filters and FilterInstances
//

_Must_inspect_result_
HRESULT
WINAPI
FilterGetInformation (
    _In_ HFILTER hFilter,
    _In_ FILTER_INFORMATION_CLASS dwInformationClass,
    _Out_writes_bytes_to_(dwBufferSize,*lpBytesReturned) LPVOID lpBuffer,
    _In_ DWORD dwBufferSize,
    _Out_ LPDWORD lpBytesReturned
    );

_Must_inspect_result_
HRESULT
WINAPI
FilterInstanceGetInformation (
    _In_ HFILTER_INSTANCE hInstance,
    _In_ INSTANCE_INFORMATION_CLASS dwInformationClass,
    _Out_writes_bytes_to_(dwBufferSize,*lpBytesReturned) LPVOID lpBuffer,
    _In_ DWORD dwBufferSize,
    _Out_ LPDWORD lpBytesReturned
    );


//****************************************************************************
//
//  Functions for communicating with Filters and FilterInstances
//
//****************************************************************************

_Must_inspect_result_
HRESULT
WINAPI
FilterConnectCommunicationPort (
    _In_ LPCWSTR lpPortName,
    _In_ DWORD dwOptions,
    _In_reads_bytes_opt_(wSizeOfContext) LPCVOID lpContext,
    _In_ WORD wSizeOfContext,
    _In_opt_ LPSECURITY_ATTRIBUTES lpSecurityAttributes,
    _Outptr_ HANDLE *hPort
    );

#if FLT_MGR_WINBLUE

//
//  FilterConnectCommunicationPort() dwOptions flags supported in Windows Blue
//  and beyond.
//

#define FLT_PORT_FLAG_SYNC_HANDLE       0x00000001

#endif

_Must_inspect_result_
HRESULT
WINAPI
FilterSendMessage (
    _In_ HANDLE hPort,
    _In_reads_bytes_(dwInBufferSize) LPVOID lpInBuffer,
    _In_ DWORD dwInBufferSize,
    _Out_writes_bytes_to_opt_(dwOutBufferSize,*lpBytesReturned) LPVOID lpOutBuffer,
    _In_ DWORD dwOutBufferSize,
    _Out_ LPDWORD lpBytesReturned
    );

_Must_inspect_result_
HRESULT
WINAPI
FilterGetMessage (
    _In_ HANDLE hPort,
    _Out_writes_bytes_(dwMessageBufferSize) PFILTER_MESSAGE_HEADER lpMessageBuffer,
    _In_ DWORD dwMessageBufferSize,
    _Inout_opt_ LPOVERLAPPED lpOverlapped
    );

_Must_inspect_result_
HRESULT
WINAPI
FilterReplyMessage (
    _In_ HANDLE hPort,
    _In_reads_bytes_(dwReplyBufferSize) PFILTER_REPLY_HEADER lpReplyBuffer,
    _In_ DWORD dwReplyBufferSize
    );

//****************************************************************************
//
//  Other support functions
//
//****************************************************************************

_Must_inspect_result_
HRESULT
WINAPI
FilterGetDosName (
    _In_ LPCWSTR lpVolumeName,
    _Out_writes_(dwDosNameBufferSize) LPWSTR lpDosName,
    _In_ DWORD dwDosNameBufferSize
    );

#endif // end the FLT_MGR_BASELINE

#ifdef __cplusplus
}       // Balance extern "C" above
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif /* __FLTUSER_H__ */


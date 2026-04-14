/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    npapi.h

Abstract:

    Network Provider API prototypes and manifests.  A network provider
    is a client of the Win32 WNet APIs.  See the "NT/Win32 Network
    Provider API Specification" document for further details.

Environment:

    User Mode -Win32

--*/

#ifndef _NPAPI_INCLUDED
#define _NPAPI_INCLUDED

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#ifdef __cplusplus
extern "C" {
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or RemoteFS Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_REMOTEFS)

//
//  CONNECTIONS
//

DWORD APIENTRY
NPAddConnection (
    _In_     LPNETRESOURCEW lpNetResource,
    _In_opt_ LPWSTR         lpPassword,
    _In_opt_ LPWSTR         lpUserName
    );

typedef DWORD (APIENTRY *PF_NPAddConnection) (
    _In_     LPNETRESOURCEW lpNetResource,
    _In_opt_ LPWSTR         lpPassword,
    _In_opt_ LPWSTR         lpUserName
    );


DWORD APIENTRY
NPAddConnection3 (
    _In_opt_ HWND           hwndOwner,
    _In_     LPNETRESOURCEW lpNetResource,
    _In_opt_ LPWSTR         lpPassword,
    _In_opt_ LPWSTR         lpUserName,
    _In_     DWORD          dwFlags
    );

typedef DWORD (APIENTRY *PF_NPAddConnection3) (
    _In_opt_ HWND           hwndOwner,
    _In_     LPNETRESOURCEW lpNetResource,
    _In_opt_ LPWSTR         lpPassword,
    _In_opt_ LPWSTR         lpUserName,
    _In_     DWORD          dwFlags
    );

DWORD APIENTRY
NPAddConnection4 (
    _In_opt_ HWND           hwndOwner,
    _In_     LPNETRESOURCEW lpNetResource,
    _In_reads_bytes_opt_(cbAuthBuffer) PVOID  lpAuthBuffer,
    _In_     DWORD          cbAuthBuffer,
    _In_     DWORD          dwFlags,
    _In_reads_bytes_opt_(cbUseOptions) PBYTE lpUseOptions,
    _In_     DWORD          cbUseOptions
    );

typedef DWORD (APIENTRY *PF_NPAddConnection4) (
    _In_opt_ HWND           hwndOwner,
    _In_     LPNETRESOURCEW lpNetResource,
    _In_reads_bytes_opt_(cbAuthBuffer) PVOID  lpAuthBuffer,
    _In_     DWORD          cbAuthBuffer,
    _In_     DWORD          dwFlags,
    _In_reads_bytes_opt_(cbUseOptions) PBYTE lpUseOptions,
    _In_     DWORD          cbUseOptions
    );

DWORD APIENTRY
NPCancelConnection (
    _In_ LPWSTR  lpName,
    _In_ BOOL    fForce
    );

typedef DWORD (APIENTRY *PF_NPCancelConnection) (
    _In_ LPWSTR  lpName,
    _In_ BOOL    fForce
    );

DWORD APIENTRY
NPCancelConnection2 (
    _In_ LPWSTR  lpName,
    _In_ BOOL    fForce,
    _In_ DWORD   dwFlags
    );

typedef DWORD (APIENTRY *PF_NPCancelConnection2) (
    _In_ LPWSTR  lpName,
    _In_ BOOL    fForce,
    _In_ DWORD   dwFlags
    );

DWORD APIENTRY
NPGetConnection (
    _In_ LPWSTR     lpLocalName,
    _Out_writes_opt_(*lpnBufferLen) LPWSTR lpRemoteName,
    _Inout_ LPDWORD lpnBufferLen
    );

typedef DWORD (APIENTRY *PF_NPGetConnection) (
    _In_ LPWSTR   lpLocalName,
    _Out_writes_opt_(*lpnBufferLen) LPWSTR lpRemoteName,
    _Inout_ LPDWORD lpnBufferLen
    );


#define WNGETCON_CONNECTED      0x00000000
#define WNGETCON_DISCONNECTED   0x00000001

DWORD APIENTRY
NPGetConnection3 (
    _In_ LPCWSTR    lpLocalName,
    _In_ DWORD      dwLevel,
    _Out_writes_bytes_(*lpBufferSize) LPVOID lpBuffer,
    _Inout_ LPDWORD lpBufferSize
    );

typedef DWORD (APIENTRY *PF_NPGetConnection3) (
    _In_ LPCWSTR    lpLocalName,
    _In_ DWORD      dwLevel,
    _Out_writes_bytes_(*lpBufferSize) LPVOID lpBuffer,
    _Inout_ LPDWORD lpBufferSize
    );


DWORD APIENTRY
NPGetUniversalName (
    _In_ LPCWSTR    lpLocalPath,
    _In_ DWORD      dwInfoLevel,
    _Out_writes_bytes_(*lpBufferSize) LPVOID lpBuffer,
    _Inout_ LPDWORD lpBufferSize
    );

typedef DWORD (APIENTRY *PF_NPGetUniversalName) (
    _In_ LPCWSTR    lpLocalPath,
    _In_ DWORD      dwInfoLevel,
    _Out_writes_bytes_(*lpnBufferSize) LPVOID lpBuffer,
    _Inout_ LPDWORD lpnBufferSize
    );

DWORD APIENTRY
NPGetConnectionPerformance (
    _In_  LPCWSTR                lpRemoteName,
    _Out_ LPNETCONNECTINFOSTRUCT lpNetConnectInfo
    );

typedef DWORD (APIENTRY *PF_NPGetConnectionPerformance) (
    _In_  LPCWSTR                lpRemoteName,
    _Out_ LPNETCONNECTINFOSTRUCT lpNetConnectInfo
    );


DWORD APIENTRY
NPOpenEnum (
    _In_     DWORD          dwScope,
    _In_     DWORD          dwType,
    _In_     DWORD          dwUsage,
    _In_opt_ LPNETRESOURCEW lpNetResource,
    _Out_    LPHANDLE       lphEnum
    );

typedef DWORD (APIENTRY *PF_NPOpenEnum) (
    _In_     DWORD          dwScope,
    _In_     DWORD          dwType,
    _In_     DWORD          dwUsage,
    _In_opt_ LPNETRESOURCEW lpNetResource,
    _Out_    LPHANDLE       lphEnum
    );

DWORD APIENTRY
NPEnumResource (
    _In_    HANDLE  hEnum,
    _Inout_ LPDWORD lpcCount,
    _Out_writes_bytes_(*lpBufferSize) LPVOID lpBuffer,
    _Inout_ LPDWORD lpBufferSize
    );

typedef DWORD (APIENTRY *PF_NPEnumResource) (
    _In_    HANDLE  hEnum,
    _Inout_ LPDWORD lpcCount,
    _Out_writes_bytes_(*lpBufferSize) LPVOID  lpBuffer,
    _Inout_ LPDWORD lpBufferSize
    );

DWORD APIENTRY
NPCloseEnum (
    _In_ HANDLE   hEnum
    );

typedef DWORD (APIENTRY *PF_NPCloseEnum) (
    _In_ HANDLE   hEnum
    );


//
//  CAPABILITIES
//

#define WNNC_SPEC_VERSION                0x00000001
#define WNNC_SPEC_VERSION51              0x00050001

#define WNNC_NET_TYPE                    0x00000002
#define WNNC_NET_NONE                    0x00000000

#define WNNC_DRIVER_VERSION              0x00000003

#define WNNC_USER                        0x00000004
#define WNNC_USR_GETUSER                 0x00000001

#define WNNC_CONNECTION                  0x00000006
#define WNNC_CON_ADDCONNECTION           0x00000001
#define WNNC_CON_CANCELCONNECTION        0x00000002
#define WNNC_CON_GETCONNECTIONS          0x00000004
#define WNNC_CON_ADDCONNECTION3          0x00000008
#define WNNC_CON_ADDCONNECTION4          0x00000010
#define WNNC_CON_CANCELCONNECTION2       0x00000020
#define WNNC_CON_GETPERFORMANCE          0x00000040
#define WNNC_CON_DEFER                   0x00000080

#define WNNC_DIALOG                      0x00000008
#define WNNC_DLG_DEVICEMODE              0x00000001
#define WNNC_DLG_PROPERTYDIALOG          0x00000020
#define WNNC_DLG_SEARCHDIALOG            0x00000040
#define WNNC_DLG_FORMATNETWORKNAME       0x00000080
#define WNNC_DLG_PERMISSIONEDITOR        0x00000100
#define WNNC_DLG_GETRESOURCEPARENT       0x00000200
#define WNNC_DLG_GETRESOURCEINFORMATION  0x00000800

#define WNNC_ADMIN                       0x00000009
#define WNNC_ADM_GETDIRECTORYTYPE        0x00000001
#define WNNC_ADM_DIRECTORYNOTIFY         0x00000002

#define WNNC_ENUMERATION                 0x0000000B
#define WNNC_ENUM_GLOBAL                 0x00000001
#define WNNC_ENUM_LOCAL                  0x00000002
#define WNNC_ENUM_CONTEXT                0x00000004
#define WNNC_ENUM_SHAREABLE              0x00000008

#define WNNC_START                       0x0000000C
#define WNNC_WAIT_FOR_START              0x00000001

#define WNNC_CONNECTION_FLAGS            0x0000000D
#define WNNC_CF_DEFAULT ( CONNECT_TEMPORARY | CONNECT_INTERACTIVE | CONNECT_PROMPT )

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN10)  

#define WNNC_CF_MAXIMUM (WNNC_CF_DEFAULT | CONNECT_UPDATE_PROFILE | CONNECT_DEFERRED | CONNECT_COMMANDLINE | CONNECT_CMD_SAVECRED | CONNECT_CRED_RESET | CONNECT_REQUIRE_INTEGRITY | CONNECT_REQUIRE_PRIVACY | CONNECT_WRITE_THROUGH_SEMANTICS | CONNECT_GLOBAL_MAPPING)

#else

#define WNNC_CF_MAXIMUM (WNNC_CF_DEFAULT | CONNECT_DEFERRED | CONNECT_COMMANDLINE | CONNECT_CMD_SAVECRED | CONNECT_CRED_RESET )

#endif




DWORD APIENTRY
NPGetCaps (
    _In_ DWORD ndex
    );

typedef DWORD (APIENTRY *PF_NPGetCaps) (
    _In_ DWORD ndex
    );

//
//  OTHER
//

DWORD APIENTRY
NPGetUser (
    _In_    LPWSTR  lpName,
    _Out_writes_(*lpnBufferLen) LPWSTR lpUserName,
    _Inout_ LPDWORD lpnBufferLen
    );

typedef DWORD (APIENTRY *PF_NPGetUser) (
    _In_    LPWSTR  lpName,
    _Out_writes_(*lpnBufferLen) LPWSTR lpUserName,
    _Inout_ LPDWORD lpnBufferLen
    );

DWORD APIENTRY
NPGetPersistentUseOptionsForConnection (
    _In_    LPWSTR  lpRemotePath,
    _In_reads_bytes_opt_(cbReadUseOptions) LPBYTE lpReadUseOptions,
    _In_ DWORD cbReadUseOptions,
    _When_(return == 0, _Out_writes_bytes_to_( *lpSizeWriteUseOptions, *lpSizeWriteUseOptions ) )
    _When_(return != 0, _Out_writes_bytes_to_( *lpSizeWriteUseOptions,  0) )
        LPBYTE lpWriteUseOptions,
    _Inout_ LPDWORD lpSizeWriteUseOptions
    );

typedef DWORD (APIENTRY *PF_NPGetPersistentUseOptionsForConnection) (
    _In_    LPWSTR  lpRemotePath,
    _In_reads_bytes_(cbReadUseOptions) LPBYTE lpReadUseOptions,
    _In_ DWORD cbReadUseOptions,
    _When_(return == 0, _Out_writes_bytes_to_( *lpSizeWriteUseOptions, *lpSizeWriteUseOptions ) )
    _When_(return != 0, _Out_writes_bytes_to_( *lpSizeWriteUseOptions,  0) )
        LPBYTE lpWriteUseOptions,
    _Inout_ LPDWORD lpSizeWriteUseOptions
    );

#define WNTYPE_DRIVE    1
#define WNTYPE_FILE     2
#define WNTYPE_PRINTER  3
#define WNTYPE_COMM     4

#define WNPS_FILE       0
#define WNPS_DIR        1
#define WNPS_MULT       2

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_REMOTEFS) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

DWORD APIENTRY
NPDeviceMode(
    _In_ HWND hParent
    );

typedef DWORD (APIENTRY *PF_NPDeviceMode) (
    _In_ HWND hParent
    );

// flag for search dialog
#define WNSRCH_REFRESH_FIRST_LEVEL 0x00000001

DWORD APIENTRY
NPSearchDialog(
    _In_     HWND   hwndParent,
    _In_opt_ LPNETRESOURCEW lpNetResource,
    _Out_writes_(cbBuffer) LPVOID lpBuffer,
    _In_     DWORD   cbBuffer,
    _Out_    LPDWORD lpnFlags
    );

typedef DWORD (APIENTRY *PF_NPSearchDialog) (
    _In_     HWND   hwndParent,
    _In_opt_ LPNETRESOURCEW lpNetResource,
    _Out_writes_(cbBuffer) LPVOID lpBuffer,
    _In_     DWORD   cbBuffer,
    _Out_    LPDWORD lpnFlags
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#pragma region Desktop Family or RemoteFS Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_REMOTEFS)

DWORD APIENTRY
NPGetResourceParent(
    _In_ LPNETRESOURCEW lpNetResource,
    _Out_writes_bytes_(*lpBufferSize) LPVOID lpBuffer,
    _Inout_ LPDWORD lpBufferSize
    );

typedef DWORD (APIENTRY *PF_NPGetResourceParent) (
    _In_ LPNETRESOURCEW lpNetResource,
    _Out_writes_bytes_(*lpBufferSize) LPVOID lpBuffer,
    _Inout_ LPDWORD lpBufferSize
    );

DWORD APIENTRY NPGetResourceInformation(
    _In_ LPNETRESOURCEW lpNetResource,
    _Out_writes_bytes_(*lpBufferSize) LPVOID lpBuffer,
    _Inout_ LPDWORD lpBufferSize,
    _Outptr_ LPWSTR *lplpSystem
    );

typedef DWORD (APIENTRY *PF_NPGetResourceInformation) (
    _In_ LPNETRESOURCEW lpNetResource,
    _Out_writes_bytes_(*lpBufferSize) LPVOID lpBuffer,
    _Inout_ LPDWORD lpBufferSize,
    _Outptr_ LPWSTR *lplpSystem
    );

DWORD APIENTRY
NPFormatNetworkName(
    _In_    LPWSTR   lpRemoteName,
    _Out_writes_(*lpnLength) LPWSTR lpFormattedName,
    _Inout_ LPDWORD  lpnLength,
    _In_    DWORD    dwFlags,
    _In_    DWORD    dwAveCharPerLine
    );

typedef DWORD (APIENTRY *PF_NPFormatNetworkName) (
    _In_    LPWSTR   lpRemoteName,
    _Out_writes_(*lpnLength) LPWSTR lpFormattedName,
    _Inout_ LPDWORD  lpnLength,
    _In_    DWORD    dwFlags,
    _In_    DWORD    dwAveCharPerLine
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_REMOTEFS) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

DWORD APIENTRY
NPGetPropertyText(
    _In_ DWORD  iButton,
    _In_ DWORD  nPropSel,
    _In_ LPWSTR lpName,
    _Out_writes_(nButtonNameLen) LPWSTR lpButtonName,
    _In_ DWORD  nButtonNameLen,
    _In_ DWORD  nType
    );

typedef DWORD (APIENTRY *PF_NPGetPropertyText) (
    _In_ DWORD  iButton,
    _In_ DWORD  nPropSel,
    _In_ LPWSTR lpName,
    _Out_writes_(nButtonNameLen) LPWSTR lpButtonName,
    _In_ DWORD  nButtonNameLen,
    _In_ DWORD  nType
    );

DWORD APIENTRY
NPPropertyDialog(
    _In_ HWND   hwndParent,
    _In_ DWORD  iButtonDlg,
    _In_ DWORD  nPropSel,
    _In_ LPWSTR lpFileName,
    _In_ DWORD  nType
    );

typedef DWORD (APIENTRY *PF_NPPropertyDialog) (
    _In_ HWND   hwndParent,
    _In_ DWORD  iButtonDlg,
    _In_ DWORD  nPropSel,
    _In_ LPWSTR lpFileName,
    _In_ DWORD  nType
    );


//
//  ADMIN
//

#define WNDT_NORMAL   0
#define WNDT_NETWORK  1

#define WNDN_MKDIR    1
#define WNDN_RMDIR    2
#define WNDN_MVDIR    3

DWORD APIENTRY
NPGetDirectoryType (
    _In_ LPWSTR  lpName,
    _In_ LPINT   lpType,
    _In_ BOOL    bFlushCache
    );

typedef DWORD (APIENTRY *PF_NPGetDirectoryType) (
    _In_ LPWSTR  lpName,
    _In_ LPINT   lpType,
    _In_ BOOL    bFlushCache
    );

DWORD APIENTRY
NPDirectoryNotify (
    _In_ HWND    hwnd,
    _In_ LPWSTR  lpDir,
    _In_ DWORD   dwOper
    );

typedef DWORD (APIENTRY *PF_NPDirectoryNotify) (
    _In_ HWND    hwnd,
    _In_ LPWSTR  lpDir,
    _In_ DWORD   dwOper
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

VOID
WNetSetLastErrorA(
    _In_ DWORD   err,
    _In_ LPSTR   lpError,
    _In_ LPSTR   lpProviders
    );

VOID
WNetSetLastErrorW(
    _In_ DWORD   err,
    _In_ LPWSTR  lpError,
    _In_ LPWSTR  lpProviders
    );

#ifdef UNICODE
#define WNetSetLastError   WNetSetLastErrorW
#else
#define WNetSetLastError   WNetSetLastErrorA
#endif  // UNICODE

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

//
//  CREDENTIAL MANAGEMENT and other classes of providers
//


// Define the Net/Authentication and othr Provider Classes
#define WN_NETWORK_CLASS            0x00000001
#define WN_CREDENTIAL_CLASS         0x00000002
#define WN_PRIMARY_AUTHENT_CLASS    0x00000004
#define WN_SERVICE_CLASS            0x00000008

#define WN_VALID_LOGON_ACCOUNT      0x00000001
#define WN_NT_PASSWORD_CHANGED      0x00000002

DWORD APIENTRY
NPLogonNotify (
    _In_     PLUID               lpLogonId,
    _In_     LPCWSTR             lpAuthentInfoType,
    _In_     LPVOID              lpAuthentInfo,
    _In_opt_ LPCWSTR             lpPreviousAuthentInfoType,
    _In_opt_ LPVOID              lpPreviousAuthentInfo,
    _In_     LPWSTR              lpStationName,
    _In_opt_ LPVOID              StationHandle,
    _Out_    LPWSTR              *lpLogonScript
    );

typedef DWORD (APIENTRY *PF_NPLogonNotify) (
    _In_     PLUID               lpLogonId,
    _In_     LPCWSTR             lpAuthentInfoType,
    _In_     LPVOID              lpAuthentInfo,
    _In_opt_ LPCWSTR             lpPreviousAuthentInfoType,
    _In_opt_ LPVOID              lpPreviousAuthentInfo,
    _In_     LPWSTR              lpStationName,
    _In_opt_ LPVOID              StationHandle,
    _Out_    LPWSTR              *lpLogonScript
    );

DWORD APIENTRY
NPPasswordChangeNotify (
    _In_     LPCWSTR             lpAuthentInfoType,
    _In_     LPVOID              lpAuthentInfo,
    _In_     LPCWSTR             lpPreviousAuthentInfoType,
    _In_     LPVOID              lpPreviousAuthentInfo,
    _In_     LPWSTR              lpStationName,
    _In_opt_ LPVOID              StationHandle,
    _In_     DWORD               dwChangeInfo
    );

typedef DWORD (APIENTRY *PF_NPPasswordChangeNotify) (
    _In_     LPCWSTR             lpAuthentInfoType,
    _In_     LPVOID              lpAuthentInfo,
    _In_     LPCWSTR             lpPreviousAuthentInfoType,
    _In_     LPVOID              lpPreviousAuthentInfo,
    _In_     LPWSTR              lpStationName,
    _In_opt_ LPVOID              StationHandle,
    _In_     DWORD               dwChangeInfo
    );


//
//  CONNECTION NOTIFICATION
//

//
// NotifyStatus
//
#define NOTIFY_PRE      0x00000001
#define NOTIFY_POST     0x00000002

typedef struct _NOTIFYINFO {
    DWORD       dwNotifyStatus;
    DWORD       dwOperationStatus;
    LPVOID      lpContext;
} NOTIFYINFO, *LPNOTIFYINFO;

typedef struct _NOTIFYADD {
    HWND            hwndOwner;
    NETRESOURCE     NetResource;
    DWORD           dwAddFlags;
} NOTIFYADD, *LPNOTIFYADD;

typedef struct _NOTIFYCANCEL {
    LPWSTR      lpName;
    LPWSTR      lpProvider;
    DWORD       dwFlags;
    BOOL        fForce;
} NOTIFYCANCEL, *LPNOTIFYCANCEL;


DWORD APIENTRY
AddConnectNotify (
    _Inout_ LPNOTIFYINFO lpNotifyInfo,
    _In_    LPNOTIFYADD  lpAddInfo
    );

typedef DWORD (APIENTRY *PF_AddConnectNotify) (
    _Inout_ LPNOTIFYINFO lpNotifyInfo,
    _In_    LPNOTIFYADD  lpAddInfo
    );

DWORD APIENTRY
CancelConnectNotify (
    _Inout_ LPNOTIFYINFO   lpNotifyInfo,
    _In_    LPNOTIFYCANCEL lpCancelInfo
    );

typedef DWORD (APIENTRY *PF_CancelConnectNotify) (
    _Inout_ LPNOTIFYINFO   lpNotifyInfo,
    _In_    LPNOTIFYCANCEL lpCancelInfo
    );

//
// Permission editor dialogs
//

//
// Capabilities bits of permission editor dialogs
//
#define WNPERMC_PERM  0x00000001
#define WNPERMC_AUDIT 0x00000002
#define WNPERMC_OWNER 0x00000004

DWORD APIENTRY
NPFMXGetPermCaps (
    _In_ LPWSTR lpDriveName
    );

typedef DWORD (APIENTRY *PF_NPFMXGetPermCaps) (
    _In_ LPWSTR lpDriveName
    );

//
// Type of security dialog
//
#define WNPERM_DLG_PERM   0
#define WNPERM_DLG_AUDIT  1
#define WNPERM_DLG_OWNER  2

DWORD APIENTRY
NPFMXEditPerm (
    _In_ LPWSTR lpDriveName,
    _In_ HWND   hwndFMX,
    _In_ DWORD  nDialogType
    );

typedef DWORD (APIENTRY *PF_NPFMXEditPerm) (
    _In_ LPWSTR lpDriveName,
    _In_ HWND   hwndFMX,
    _In_ DWORD  nDialogType
    );

DWORD APIENTRY
NPFMXGetPermHelp (
    _In_    LPWSTR  lpDriveName,
    _In_    DWORD   nDialogType,
    _In_    BOOL    fDirectory,
    _Out_writes_(*lpBufferSize) LPVOID lpFileNameBuffer,
    _Inout_ LPDWORD lpBufferSize,
    _Out_   LPDWORD lpnHelpContext
    );

typedef DWORD (APIENTRY *PF_NPFMXGetPermHelp) (
    _In_    LPWSTR  lpDriveName,
    _In_    DWORD   nDialogType,
    _In_    BOOL    fDirectory,
    _Out_writes_(*lpBufferSize) LPVOID lpFileNameBuffer,
    _Inout_ LPDWORD lpBufferSize,
    _Out_   LPDWORD lpnHelpContext
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#ifdef __cplusplus
}
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif  // _NPAPI_INCLUDED

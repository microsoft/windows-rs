#include <winapifamily.h>

/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    winnetwk.h

Abstract:

    Standard WINNET Header File for WIN32

Environment:

    User Mode -Win32

Notes:

    optional-notes

--*/

#ifndef _WINNETWK_
#define _WINNETWK_


#pragma once


#ifdef __cplusplus
extern "C" {
#endif

#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4820) // padding added after data member
#endif


#pragma region Desktop Family or System Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)



//
// Network types
//

#include <wnnc.h>

//
//  Network Resources.
//

#define RESOURCE_CONNECTED      0x00000001
#define RESOURCE_GLOBALNET      0x00000002
#define RESOURCE_REMEMBERED     0x00000003
#if(WINVER >= 0x0400)
#define RESOURCE_RECENT         0x00000004
#define RESOURCE_CONTEXT        0x00000005
#endif /* WINVER >= 0x0400 */

#define RESOURCETYPE_ANY        0x00000000
#define RESOURCETYPE_DISK       0x00000001
#define RESOURCETYPE_PRINT      0x00000002
#if(WINVER >= 0x0400)
#define RESOURCETYPE_RESERVED   0x00000008
#endif /* WINVER >= 0x0400 */
#define RESOURCETYPE_UNKNOWN    0xFFFFFFFF

#define RESOURCEUSAGE_CONNECTABLE   0x00000001
#define RESOURCEUSAGE_CONTAINER     0x00000002
#if(WINVER >= 0x0400)
#define RESOURCEUSAGE_NOLOCALDEVICE 0x00000004
#define RESOURCEUSAGE_SIBLING       0x00000008
#define RESOURCEUSAGE_ATTACHED      0x00000010
#define RESOURCEUSAGE_ALL           (RESOURCEUSAGE_CONNECTABLE | RESOURCEUSAGE_CONTAINER | RESOURCEUSAGE_ATTACHED)
#endif /* WINVER >= 0x0400 */
#define RESOURCEUSAGE_RESERVED      0x80000000

#define RESOURCEDISPLAYTYPE_GENERIC        0x00000000
#define RESOURCEDISPLAYTYPE_DOMAIN         0x00000001
#define RESOURCEDISPLAYTYPE_SERVER         0x00000002
#define RESOURCEDISPLAYTYPE_SHARE          0x00000003
#define RESOURCEDISPLAYTYPE_FILE           0x00000004
#define RESOURCEDISPLAYTYPE_GROUP          0x00000005
#if(WINVER >= 0x0400)
#define RESOURCEDISPLAYTYPE_NETWORK        0x00000006
#define RESOURCEDISPLAYTYPE_ROOT           0x00000007
#define RESOURCEDISPLAYTYPE_SHAREADMIN     0x00000008
#define RESOURCEDISPLAYTYPE_DIRECTORY      0x00000009
#endif /* WINVER >= 0x0400 */
#define RESOURCEDISPLAYTYPE_TREE           0x0000000A
#if(WINVER >= 0x0400)
#define RESOURCEDISPLAYTYPE_NDSCONTAINER   0x0000000B
#endif /* WINVER >= 0x0400 */

typedef struct  _NETRESOURCEA {
    DWORD    dwScope;
    DWORD    dwType;
    DWORD    dwDisplayType;
    DWORD    dwUsage;
    LPSTR    lpLocalName;
    LPSTR    lpRemoteName;
    LPSTR    lpComment ;
    LPSTR    lpProvider;
}NETRESOURCEA, *LPNETRESOURCEA;
typedef struct  _NETRESOURCEW {
    DWORD    dwScope;
    DWORD    dwType;
    DWORD    dwDisplayType;
    DWORD    dwUsage;
    LPWSTR   lpLocalName;
    LPWSTR   lpRemoteName;
    LPWSTR   lpComment ;
    LPWSTR   lpProvider;
}NETRESOURCEW, *LPNETRESOURCEW;
#ifdef UNICODE
typedef NETRESOURCEW NETRESOURCE;
typedef LPNETRESOURCEW LPNETRESOURCE;
#else
typedef NETRESOURCEA NETRESOURCE;
typedef LPNETRESOURCEA LPNETRESOURCE;
#endif // UNICODE

//
//  Network Connections.
//

#define NETPROPERTY_PERSISTENT       1

#define CONNECT_UPDATE_PROFILE      0x00000001
#define CONNECT_UPDATE_RECENT       0x00000002
#define CONNECT_TEMPORARY           0x00000004
#define CONNECT_INTERACTIVE         0x00000008
#define CONNECT_PROMPT              0x00000010
#define CONNECT_NEED_DRIVE          0x00000020
#if(WINVER >= 0x0400)
#define CONNECT_REFCOUNT            0x00000040
#define CONNECT_REDIRECT            0x00000080
#define CONNECT_LOCALDRIVE          0x00000100
#define CONNECT_CURRENT_MEDIA       0x00000200
#define CONNECT_DEFERRED            0x00000400
#define CONNECT_RESERVED            0xFF000000
#endif /* WINVER >= 0x0400 */
#if(WINVER >= 0x0500)
#define CONNECT_COMMANDLINE         0x00000800
#define CONNECT_CMD_SAVECRED        0x00001000
#endif /* WINVER >= 0x0500 */
#if(WINVER >= 0x0600)
#define CONNECT_CRED_RESET          0x00002000
#endif /* WINVER >= 0x0600 */

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN10)  

#define CONNECT_REQUIRE_INTEGRITY                0x00004000
#define CONNECT_REQUIRE_PRIVACY                  0x00008000
#define CONNECT_WRITE_THROUGH_SEMANTICS          0x00010000
#define CONNECT_GLOBAL_MAPPING                   0x00040000 

#endif

_Check_return_
DWORD APIENTRY
WNetAddConnectionA(
    _In_     LPCSTR   lpRemoteName,
    _In_opt_ LPCSTR   lpPassword,
    _In_opt_ LPCSTR   lpLocalName
    );
_Check_return_
DWORD APIENTRY
WNetAddConnectionW(
    _In_     LPCWSTR   lpRemoteName,
    _In_opt_ LPCWSTR   lpPassword,
    _In_opt_ LPCWSTR   lpLocalName
    );
#ifdef UNICODE
#define WNetAddConnection  WNetAddConnectionW
#else
#define WNetAddConnection  WNetAddConnectionA
#endif // !UNICODE

_Check_return_
DWORD APIENTRY
WNetAddConnection2A(
    _In_     LPNETRESOURCEA lpNetResource,
    _In_opt_ LPCSTR       lpPassword,
    _In_opt_ LPCSTR       lpUserName,
    _In_     DWORD          dwFlags
    );
_Check_return_
DWORD APIENTRY
WNetAddConnection2W(
    _In_     LPNETRESOURCEW lpNetResource,
    _In_opt_ LPCWSTR       lpPassword,
    _In_opt_ LPCWSTR       lpUserName,
    _In_     DWORD          dwFlags
    );
#ifdef UNICODE
#define WNetAddConnection2  WNetAddConnection2W
#else
#define WNetAddConnection2  WNetAddConnection2A
#endif // !UNICODE

_Check_return_
DWORD APIENTRY
WNetAddConnection3A(
    _In_opt_ HWND           hwndOwner,
    _In_     LPNETRESOURCEA lpNetResource,
    _In_opt_ LPCSTR       lpPassword,
    _In_opt_ LPCSTR       lpUserName,
    _In_     DWORD          dwFlags
    );
_Check_return_
DWORD APIENTRY
WNetAddConnection3W(
    _In_opt_ HWND           hwndOwner,
    _In_     LPNETRESOURCEW lpNetResource,
    _In_opt_ LPCWSTR       lpPassword,
    _In_opt_ LPCWSTR       lpUserName,
    _In_     DWORD          dwFlags
    );
#ifdef UNICODE
#define WNetAddConnection3  WNetAddConnection3W
#else
#define WNetAddConnection3  WNetAddConnection3A
#endif // !UNICODE

#if(_WIN32_WINNT >= _WIN32_WINNT_WIN10)
_Check_return_
DWORD APIENTRY
WNetAddConnection4A(
    _In_opt_ HWND           hwndOwner,
    _In_     LPNETRESOURCEA lpNetResource,
    _In_reads_bytes_(cbAuthBuffer) PVOID  pAuthBuffer,
    _In_     DWORD          cbAuthBuffer,
    _In_     DWORD          dwFlags,
    _In_reads_bytes_(cbUseOptions)     PBYTE          lpUseOptions,
    _In_     DWORD          cbUseOptions
    );
_Check_return_
DWORD APIENTRY
WNetAddConnection4W(
    _In_opt_ HWND           hwndOwner,
    _In_     LPNETRESOURCEW lpNetResource,
    _In_reads_bytes_(cbAuthBuffer) PVOID  pAuthBuffer,
    _In_     DWORD          cbAuthBuffer,
    _In_     DWORD          dwFlags,
    _In_reads_bytes_(cbUseOptions)     PBYTE          lpUseOptions,
    _In_     DWORD          cbUseOptions
    );
#ifdef UNICODE
#define WNetAddConnection4  WNetAddConnection4W
#else
#define WNetAddConnection4  WNetAddConnection4A
#endif // !UNICODE
#endif //(_WIN32_WINNT >= _WIN32_WINNT_WIN10)

_Check_return_
DWORD APIENTRY
WNetCancelConnectionA(
    _In_ LPCSTR lpName,
    _In_ BOOL     fForce
    );
_Check_return_
DWORD APIENTRY
WNetCancelConnectionW(
    _In_ LPCWSTR lpName,
    _In_ BOOL     fForce
    );
#ifdef UNICODE
#define WNetCancelConnection  WNetCancelConnectionW
#else
#define WNetCancelConnection  WNetCancelConnectionA
#endif // !UNICODE

_Check_return_
DWORD APIENTRY
WNetCancelConnection2A(
    _In_ LPCSTR lpName,
    _In_ DWORD    dwFlags,
    _In_ BOOL     fForce
    );
_Check_return_
DWORD APIENTRY
WNetCancelConnection2W(
    _In_ LPCWSTR lpName,
    _In_ DWORD    dwFlags,
    _In_ BOOL     fForce
    );
#ifdef UNICODE
#define WNetCancelConnection2  WNetCancelConnection2W
#else
#define WNetCancelConnection2  WNetCancelConnection2A
#endif // !UNICODE

_Check_return_
DWORD APIENTRY
WNetGetConnectionA(
    _In_ LPCSTR lpLocalName,
    _Out_writes_opt_(*lpnLength) LPSTR  lpRemoteName,
    _Inout_ LPDWORD lpnLength
    );
_Check_return_
DWORD APIENTRY
WNetGetConnectionW(
    _In_ LPCWSTR lpLocalName,
    _Out_writes_opt_(*lpnLength) LPWSTR  lpRemoteName,
    _Inout_ LPDWORD lpnLength
    );
#ifdef UNICODE
#define WNetGetConnection  WNetGetConnectionW
#else
#define WNetGetConnection  WNetGetConnectionA
#endif // !UNICODE


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#if (_WIN32_WINNT >= _WIN32_WINNT_LONGHORN)
_Check_return_
DWORD APIENTRY
WNetRestoreSingleConnectionW(
    _In_opt_ HWND    hwndParent,
    _In_     LPCWSTR lpDevice,
    _In_     BOOL    fUseUI
    );

#else
_Check_return_
DWORD APIENTRY
WNetRestoreConnectionW(
    _In_opt_ HWND    hWnd,
    _In_     LPCWSTR lpDevice
    );

#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#pragma region Desktop Family or System Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#if(WINVER >= 0x0400)
_Check_return_
DWORD APIENTRY
WNetUseConnectionA(
    _In_opt_ HWND            hwndOwner,
    _In_     LPNETRESOURCEA  lpNetResource,
    _In_opt_ LPCSTR        lpPassword,
    _In_opt_ LPCSTR        lpUserId,
    _In_     DWORD           dwFlags,
    _Out_writes_opt_(*lpBufferSize) LPSTR lpAccessName,
    _Inout_opt_ LPDWORD lpBufferSize,
    _Out_opt_ LPDWORD   lpResult
    );
_Check_return_
DWORD APIENTRY
WNetUseConnectionW(
    _In_opt_ HWND            hwndOwner,
    _In_     LPNETRESOURCEW  lpNetResource,
    _In_opt_ LPCWSTR        lpPassword,
    _In_opt_ LPCWSTR        lpUserId,
    _In_     DWORD           dwFlags,
    _Out_writes_opt_(*lpBufferSize) LPWSTR lpAccessName,
    _Inout_opt_ LPDWORD lpBufferSize,
    _Out_opt_ LPDWORD   lpResult
    );
#ifdef UNICODE
#define WNetUseConnection  WNetUseConnectionW
#else
#define WNetUseConnection  WNetUseConnectionA
#endif // !UNICODE
#endif /* WINVER >= 0x0400 */

#if(_WIN32_WINNT >= _WIN32_WINNT_WIN10)
_Check_return_
DWORD APIENTRY
WNetUseConnection4A(
    _In_opt_ HWND            hwndOwner,
    _In_     LPNETRESOURCEA  lpNetResource,
    _In_reads_bytes_opt_(cbAuthBuffer) PVOID  pAuthBuffer,
    _In_     DWORD           cbAuthBuffer,
    _In_     DWORD           dwFlags,
    _In_reads_bytes_opt_(cbUseOptions)     PBYTE           lpUseOptions,
    _In_     DWORD           cbUseOptions,
    _Out_writes_opt_(*lpBufferSize) LPSTR lpAccessName,
    _Inout_opt_ LPDWORD lpBufferSize,
    _Out_opt_ LPDWORD   lpResult
    );
_Check_return_
DWORD APIENTRY
WNetUseConnection4W(
    _In_opt_ HWND            hwndOwner,
    _In_     LPNETRESOURCEW  lpNetResource,
    _In_reads_bytes_opt_(cbAuthBuffer) PVOID  pAuthBuffer,
    _In_     DWORD           cbAuthBuffer,
    _In_     DWORD           dwFlags,
    _In_reads_bytes_opt_(cbUseOptions)     PBYTE           lpUseOptions,
    _In_     DWORD           cbUseOptions,
    _Out_writes_opt_(*lpBufferSize) LPWSTR lpAccessName,
    _Inout_opt_ LPDWORD lpBufferSize,
    _Out_opt_ LPDWORD   lpResult
    );
#ifdef UNICODE
#define WNetUseConnection4  WNetUseConnection4W
#else
#define WNetUseConnection4  WNetUseConnection4A
#endif // !UNICODE
#endif //(_WIN32_WINNT >= _WIN32_WINNT_WIN10)


//
//  Network Connection Dialogs.
//
_Check_return_
DWORD APIENTRY
WNetConnectionDialog(
    _In_ HWND  hwnd,
    _In_ DWORD dwType
    );

_Check_return_
DWORD APIENTRY
WNetDisconnectDialog(
    _In_opt_ HWND hwnd,
    _In_ DWORD dwType
    );

#if(WINVER >= 0x0400)
typedef struct _CONNECTDLGSTRUCTA{
    DWORD cbStructure;       /* size of this structure in bytes */
    HWND hwndOwner;          /* owner window for the dialog */
    LPNETRESOURCEA lpConnRes;/* Requested Resource info    */
    DWORD dwFlags;           /* flags (see below) */
    DWORD dwDevNum;          /* number of devices connected to */
} CONNECTDLGSTRUCTA, FAR *LPCONNECTDLGSTRUCTA;
typedef struct _CONNECTDLGSTRUCTW{
    DWORD cbStructure;       /* size of this structure in bytes */
    HWND hwndOwner;          /* owner window for the dialog */
    LPNETRESOURCEW lpConnRes;/* Requested Resource info    */
    DWORD dwFlags;           /* flags (see below) */
    DWORD dwDevNum;          /* number of devices connected to */
} CONNECTDLGSTRUCTW, FAR *LPCONNECTDLGSTRUCTW;
#ifdef UNICODE
typedef CONNECTDLGSTRUCTW CONNECTDLGSTRUCT;
typedef LPCONNECTDLGSTRUCTW LPCONNECTDLGSTRUCT;
#else
typedef CONNECTDLGSTRUCTA CONNECTDLGSTRUCT;
typedef LPCONNECTDLGSTRUCTA LPCONNECTDLGSTRUCT;
#endif // UNICODE

#define CONNDLG_RO_PATH     0x00000001 /* Resource path should be read-only    */
#define CONNDLG_CONN_POINT  0x00000002 /* Netware -style movable connection point enabled */
#define CONNDLG_USE_MRU     0x00000004 /* Use MRU combobox  */
#define CONNDLG_HIDE_BOX    0x00000008 /* Hide persistent connect checkbox  */

/*
 * NOTE:  Set at most ONE of the below flags.  If neither flag is set,
 *        then the persistence is set to whatever the user chose during
 *        a previous connection
 */
#define CONNDLG_PERSIST     0x00000010 /* Force persistent connection */
#define CONNDLG_NOT_PERSIST 0x00000020 /* Force connection NOT persistent */

_Check_return_
DWORD APIENTRY
WNetConnectionDialog1A(
    _Inout_ LPCONNECTDLGSTRUCTA lpConnDlgStruct
    );
_Check_return_
DWORD APIENTRY
WNetConnectionDialog1W(
    _Inout_ LPCONNECTDLGSTRUCTW lpConnDlgStruct
    );
#ifdef UNICODE
#define WNetConnectionDialog1  WNetConnectionDialog1W
#else
#define WNetConnectionDialog1  WNetConnectionDialog1A
#endif // !UNICODE

typedef struct _DISCDLGSTRUCTA{
    DWORD           cbStructure;      /* size of this structure in bytes */
    HWND            hwndOwner;        /* owner window for the dialog */
    LPSTR           lpLocalName;      /* local device name */
    LPSTR           lpRemoteName;     /* network resource name */
    DWORD           dwFlags;          /* flags */
} DISCDLGSTRUCTA, FAR *LPDISCDLGSTRUCTA;
typedef struct _DISCDLGSTRUCTW{
    DWORD           cbStructure;      /* size of this structure in bytes */
    HWND            hwndOwner;        /* owner window for the dialog */
    LPWSTR          lpLocalName;      /* local device name */
    LPWSTR          lpRemoteName;     /* network resource name */
    DWORD           dwFlags;          /* flags */
} DISCDLGSTRUCTW, FAR *LPDISCDLGSTRUCTW;
#ifdef UNICODE
typedef DISCDLGSTRUCTW DISCDLGSTRUCT;
typedef LPDISCDLGSTRUCTW LPDISCDLGSTRUCT;
#else
typedef DISCDLGSTRUCTA DISCDLGSTRUCT;
typedef LPDISCDLGSTRUCTA LPDISCDLGSTRUCT;
#endif // UNICODE

#define DISC_UPDATE_PROFILE         0x00000001
#define DISC_NO_FORCE               0x00000040

_Check_return_
DWORD APIENTRY
WNetDisconnectDialog1A(
    _In_ LPDISCDLGSTRUCTA lpConnDlgStruct
    );
_Check_return_
DWORD APIENTRY
WNetDisconnectDialog1W(
    _In_ LPDISCDLGSTRUCTW lpConnDlgStruct
    );
#ifdef UNICODE
#define WNetDisconnectDialog1  WNetDisconnectDialog1W
#else
#define WNetDisconnectDialog1  WNetDisconnectDialog1A
#endif // !UNICODE
#endif /* WINVER >= 0x0400 */

//
//  Network Browsing.
//

_Check_return_
DWORD APIENTRY
WNetOpenEnumA(
    _In_  DWORD          dwScope,
    _In_  DWORD          dwType,
    _In_  DWORD          dwUsage,
    _In_opt_ LPNETRESOURCEA lpNetResource,
    _Out_ LPHANDLE       lphEnum
    );
_Check_return_
DWORD APIENTRY
WNetOpenEnumW(
    _In_  DWORD          dwScope,
    _In_  DWORD          dwType,
    _In_  DWORD          dwUsage,
    _In_opt_ LPNETRESOURCEW lpNetResource,
    _Out_ LPHANDLE       lphEnum
    );
#ifdef UNICODE
#define WNetOpenEnum  WNetOpenEnumW
#else
#define WNetOpenEnum  WNetOpenEnumA
#endif // !UNICODE

_Check_return_
DWORD APIENTRY
WNetEnumResourceA(
    _In_    HANDLE  hEnum,
    _Inout_ LPDWORD lpcCount,
    _Out_writes_bytes_(*lpBufferSize) LPVOID  lpBuffer,
    _Inout_ LPDWORD lpBufferSize
    );
_Check_return_
DWORD APIENTRY
WNetEnumResourceW(
    _In_    HANDLE  hEnum,
    _Inout_ LPDWORD lpcCount,
    _Out_writes_bytes_(*lpBufferSize) LPVOID  lpBuffer,
    _Inout_ LPDWORD lpBufferSize
    );
#ifdef UNICODE
#define WNetEnumResource  WNetEnumResourceW
#else
#define WNetEnumResource  WNetEnumResourceA
#endif // !UNICODE

_Check_return_
DWORD APIENTRY
WNetCloseEnum(
    _In_ HANDLE   hEnum
    );

#if(WINVER >= 0x0400)
_Check_return_
DWORD APIENTRY
WNetGetResourceParentA(
    _In_ LPNETRESOURCEA lpNetResource,
    _Out_writes_bytes_(*lpcbBuffer) LPVOID lpBuffer,
    _Inout_ LPDWORD lpcbBuffer
    );
_Check_return_
DWORD APIENTRY
WNetGetResourceParentW(
    _In_ LPNETRESOURCEW lpNetResource,
    _Out_writes_bytes_(*lpcbBuffer) LPVOID lpBuffer,
    _Inout_ LPDWORD lpcbBuffer
    );
#ifdef UNICODE
#define WNetGetResourceParent  WNetGetResourceParentW
#else
#define WNetGetResourceParent  WNetGetResourceParentA
#endif // !UNICODE

_Check_return_
DWORD APIENTRY
WNetGetResourceInformationA(
    _In_ LPNETRESOURCEA  lpNetResource,
    _Out_writes_bytes_(*lpcbBuffer) LPVOID lpBuffer,
    _Inout_ LPDWORD lpcbBuffer,
    _Outptr_ LPSTR *lplpSystem
    );
_Check_return_
DWORD APIENTRY
WNetGetResourceInformationW(
    _In_ LPNETRESOURCEW  lpNetResource,
    _Out_writes_bytes_(*lpcbBuffer) LPVOID lpBuffer,
    _Inout_ LPDWORD lpcbBuffer,
    _Outptr_ LPWSTR *lplpSystem
    );
#ifdef UNICODE
#define WNetGetResourceInformation  WNetGetResourceInformationW
#else
#define WNetGetResourceInformation  WNetGetResourceInformationA
#endif // !UNICODE
#endif /* WINVER >= 0x0400 */

//
//  Universal Naming.
//

#define UNIVERSAL_NAME_INFO_LEVEL   0x00000001
#define REMOTE_NAME_INFO_LEVEL      0x00000002

typedef struct  _UNIVERSAL_NAME_INFOA {
    LPSTR    lpUniversalName;
}UNIVERSAL_NAME_INFOA, *LPUNIVERSAL_NAME_INFOA;
typedef struct  _UNIVERSAL_NAME_INFOW {
    LPWSTR   lpUniversalName;
}UNIVERSAL_NAME_INFOW, *LPUNIVERSAL_NAME_INFOW;
#ifdef UNICODE
typedef UNIVERSAL_NAME_INFOW UNIVERSAL_NAME_INFO;
typedef LPUNIVERSAL_NAME_INFOW LPUNIVERSAL_NAME_INFO;
#else
typedef UNIVERSAL_NAME_INFOA UNIVERSAL_NAME_INFO;
typedef LPUNIVERSAL_NAME_INFOA LPUNIVERSAL_NAME_INFO;
#endif // UNICODE

typedef struct  _REMOTE_NAME_INFOA {
    LPSTR    lpUniversalName;
    LPSTR    lpConnectionName;
    LPSTR    lpRemainingPath;
}REMOTE_NAME_INFOA, *LPREMOTE_NAME_INFOA;
typedef struct  _REMOTE_NAME_INFOW {
    LPWSTR   lpUniversalName;
    LPWSTR   lpConnectionName;
    LPWSTR   lpRemainingPath;
}REMOTE_NAME_INFOW, *LPREMOTE_NAME_INFOW;
#ifdef UNICODE
typedef REMOTE_NAME_INFOW REMOTE_NAME_INFO;
typedef LPREMOTE_NAME_INFOW LPREMOTE_NAME_INFO;
#else
typedef REMOTE_NAME_INFOA REMOTE_NAME_INFO;
typedef LPREMOTE_NAME_INFOA LPREMOTE_NAME_INFO;
#endif // UNICODE

_Check_return_
DWORD APIENTRY
WNetGetUniversalNameA(
    _In_ LPCSTR lpLocalPath,
    _In_ DWORD    dwInfoLevel,
    _Out_writes_bytes_(*lpBufferSize) LPVOID lpBuffer,
    _Inout_ LPDWORD lpBufferSize
    );
_Check_return_
DWORD APIENTRY
WNetGetUniversalNameW(
    _In_ LPCWSTR lpLocalPath,
    _In_ DWORD    dwInfoLevel,
    _Out_writes_bytes_(*lpBufferSize) LPVOID lpBuffer,
    _Inout_ LPDWORD lpBufferSize
    );
#ifdef UNICODE
#define WNetGetUniversalName  WNetGetUniversalNameW
#else
#define WNetGetUniversalName  WNetGetUniversalNameA
#endif // !UNICODE

//
//  Authentication and Logon/Logoff.
//
_Check_return_
DWORD APIENTRY
WNetGetUserA(
    _In_opt_ LPCSTR  lpName,
    _Out_writes_(*lpnLength) LPSTR lpUserName,
    _Inout_  LPDWORD lpnLength
    );
//
//  Authentication and Logon/Logoff.
//
_Check_return_
DWORD APIENTRY
WNetGetUserW(
    _In_opt_ LPCWSTR  lpName,
    _Out_writes_(*lpnLength) LPWSTR lpUserName,
    _Inout_  LPDWORD lpnLength
    );
#ifdef UNICODE
#define WNetGetUser  WNetGetUserW
#else
#define WNetGetUser  WNetGetUserA
#endif // !UNICODE



//
// Other.
//

#if(WINVER >= 0x0400)
#define WNFMT_MULTILINE         0x01
#define WNFMT_ABBREVIATED       0x02
#define WNFMT_INENUM            0x10
#define WNFMT_CONNECTION        0x20
#endif /* WINVER >= 0x0400 */


#if(WINVER >= 0x0400)
_Check_return_
DWORD APIENTRY
WNetGetProviderNameA(
    _In_    DWORD   dwNetType,
    _Out_writes_(*lpBufferSize) LPSTR lpProviderName,
    _Inout_ LPDWORD lpBufferSize
    );
_Check_return_
DWORD APIENTRY
WNetGetProviderNameW(
    _In_    DWORD   dwNetType,
    _Out_writes_(*lpBufferSize) LPWSTR lpProviderName,
    _Inout_ LPDWORD lpBufferSize
    );
#ifdef UNICODE
#define WNetGetProviderName  WNetGetProviderNameW
#else
#define WNetGetProviderName  WNetGetProviderNameA
#endif // !UNICODE

typedef struct _NETINFOSTRUCT{
    DWORD cbStructure;
    DWORD dwProviderVersion;
    DWORD dwStatus;
    DWORD dwCharacteristics;
    ULONG_PTR dwHandle;
    WORD  wNetType;
    DWORD dwPrinters;
    DWORD dwDrives;
} NETINFOSTRUCT, FAR *LPNETINFOSTRUCT;

#define NETINFO_DLL16       0x00000001  /* Provider running as 16 bit Winnet Driver */
#define NETINFO_DISKRED     0x00000004  /* Provider requires disk redirections to connect */
#define NETINFO_PRINTERRED  0x00000008  /* Provider requires printer redirections to connect */

_Check_return_
DWORD APIENTRY
WNetGetNetworkInformationA(
    _In_  LPCSTR        lpProvider,
    _Out_ LPNETINFOSTRUCT lpNetInfoStruct
    );
_Check_return_
DWORD APIENTRY
WNetGetNetworkInformationW(
    _In_  LPCWSTR        lpProvider,
    _Out_ LPNETINFOSTRUCT lpNetInfoStruct
    );
#ifdef UNICODE
#define WNetGetNetworkInformation  WNetGetNetworkInformationW
#else
#define WNetGetNetworkInformation  WNetGetNetworkInformationA
#endif // !UNICODE

#endif /* WINVER >= 0x0400 */

//
//  Error handling.
//

_Check_return_
DWORD APIENTRY
WNetGetLastErrorA(
    _Out_ LPDWORD    lpError,
    _Out_writes_(nErrorBufSize) LPSTR lpErrorBuf,
    _In_ DWORD      nErrorBufSize,
    _Out_writes_(nNameBufSize) LPSTR  lpNameBuf,
    _In_ DWORD      nNameBufSize
    );
_Check_return_
DWORD APIENTRY
WNetGetLastErrorW(
    _Out_ LPDWORD    lpError,
    _Out_writes_(nErrorBufSize) LPWSTR lpErrorBuf,
    _In_ DWORD      nErrorBufSize,
    _Out_writes_(nNameBufSize) LPWSTR  lpNameBuf,
    _In_ DWORD      nNameBufSize
    );
#ifdef UNICODE
#define WNetGetLastError  WNetGetLastErrorW
#else
#define WNetGetLastError  WNetGetLastErrorA
#endif // !UNICODE

//
//  STATUS CODES
//

// General

#define WN_SUCCESS                      NO_ERROR
#define WN_NO_ERROR                     NO_ERROR
#define WN_NOT_SUPPORTED                ERROR_NOT_SUPPORTED
#define WN_CANCEL                       ERROR_CANCELLED
#define WN_RETRY                        ERROR_RETRY
#define WN_NET_ERROR                    ERROR_UNEXP_NET_ERR
#define WN_MORE_DATA                    ERROR_MORE_DATA
#define WN_BAD_POINTER                  ERROR_INVALID_ADDRESS
#define WN_BAD_VALUE                    ERROR_INVALID_PARAMETER
#define WN_BAD_USER                     ERROR_BAD_USERNAME
#define WN_BAD_PASSWORD                 ERROR_INVALID_PASSWORD
#define WN_ACCESS_DENIED                ERROR_ACCESS_DENIED
#define WN_FUNCTION_BUSY                ERROR_BUSY
#define WN_WINDOWS_ERROR                ERROR_UNEXP_NET_ERR
#define WN_OUT_OF_MEMORY                ERROR_NOT_ENOUGH_MEMORY
#define WN_NO_NETWORK                   ERROR_NO_NETWORK
#define WN_EXTENDED_ERROR               ERROR_EXTENDED_ERROR
#define WN_BAD_LEVEL                    ERROR_INVALID_LEVEL
#define WN_BAD_HANDLE                   ERROR_INVALID_HANDLE
#if(WINVER >= 0x0400)
#define WN_NOT_INITIALIZING             ERROR_ALREADY_INITIALIZED
#define WN_NO_MORE_DEVICES              ERROR_NO_MORE_DEVICES
#endif /* WINVER >= 0x0400 */

// Connection

#define WN_NOT_CONNECTED                       ERROR_NOT_CONNECTED
#define WN_OPEN_FILES                          ERROR_OPEN_FILES
#define WN_DEVICE_IN_USE                       ERROR_DEVICE_IN_USE
#define WN_BAD_NETNAME                         ERROR_BAD_NET_NAME
#define WN_BAD_LOCALNAME                       ERROR_BAD_DEVICE
#define WN_ALREADY_CONNECTED                   ERROR_ALREADY_ASSIGNED
#define WN_DEVICE_ERROR                        ERROR_GEN_FAILURE
#define WN_CONNECTION_CLOSED                   ERROR_CONNECTION_UNAVAIL
#define WN_NO_NET_OR_BAD_PATH                  ERROR_NO_NET_OR_BAD_PATH
#define WN_BAD_PROVIDER                        ERROR_BAD_PROVIDER
#define WN_CANNOT_OPEN_PROFILE                 ERROR_CANNOT_OPEN_PROFILE
#define WN_BAD_PROFILE                         ERROR_BAD_PROFILE
#define WN_BAD_DEV_TYPE                        ERROR_BAD_DEV_TYPE
#define WN_DEVICE_ALREADY_REMEMBERED           ERROR_DEVICE_ALREADY_REMEMBERED
#define WN_CONNECTED_OTHER_PASSWORD            ERROR_CONNECTED_OTHER_PASSWORD
#if(WINVER >= 0x0501)
#define WN_CONNECTED_OTHER_PASSWORD_DEFAULT    ERROR_CONNECTED_OTHER_PASSWORD_DEFAULT
#endif /* WINVER >= 0x0501 */

// Enumeration

#define WN_NO_MORE_ENTRIES              ERROR_NO_MORE_ITEMS
#define WN_NOT_CONTAINER                ERROR_NOT_CONTAINER

#if(WINVER >= 0x0400)
// Authentication

#define WN_NOT_AUTHENTICATED            ERROR_NOT_AUTHENTICATED
#define WN_NOT_LOGGED_ON                ERROR_NOT_LOGGED_ON
#define WN_NOT_VALIDATED                ERROR_NO_LOGON_SERVERS
#endif /* WINVER >= 0x0400 */

//
//  For Shell
//

#if(WINVER >= 0x0400)
typedef struct _NETCONNECTINFOSTRUCT{
    DWORD cbStructure;
    DWORD dwFlags;
    DWORD dwSpeed;
    DWORD dwDelay;
    DWORD dwOptDataSize;
} NETCONNECTINFOSTRUCT,  *LPNETCONNECTINFOSTRUCT;

#define WNCON_FORNETCARD        0x00000001
#define WNCON_NOTROUTED         0x00000002
#define WNCON_SLOWLINK          0x00000004
#define WNCON_DYNAMIC           0x00000008

_Check_return_
DWORD APIENTRY
MultinetGetConnectionPerformanceA(
    _In_  LPNETRESOURCEA lpNetResource,
    _Out_ LPNETCONNECTINFOSTRUCT lpNetConnectInfoStruct
    );
_Check_return_
DWORD APIENTRY
MultinetGetConnectionPerformanceW(
    _In_  LPNETRESOURCEW lpNetResource,
    _Out_ LPNETCONNECTINFOSTRUCT lpNetConnectInfoStruct
    );
#ifdef UNICODE
#define MultinetGetConnectionPerformance  MultinetGetConnectionPerformanceW
#else
#define MultinetGetConnectionPerformance  MultinetGetConnectionPerformanceA
#endif // !UNICODE
#endif /* WINVER >= 0x0400 */


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion


#if _MSC_VER >= 1200
#pragma warning(pop)
#endif

#ifdef __cplusplus
}
#endif


#endif  // _WINNETWK_


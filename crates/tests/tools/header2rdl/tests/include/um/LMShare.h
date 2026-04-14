/*++ BUILD Version: 0001    // Increment this if a change has global effects

Copyright (c) 1990-1999  Microsoft Corporation

Module Name:

    lmshare.h

Abstract:

    This module defines the API function prototypes and data structures
    for the following groups of NT API functions:
        NetShare
        NetSession
        NetFile
        NetConnection

Environment:

    User Mode - Win32

Notes:

    You must include <windef.h> and <lmcons.h> before this file.

--*/

#include <winapifamily.h>

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

//
// SHARE API
//

#ifndef _LMSHARE_
#define _LMSHARE_

#if _MSC_VER > 1000
#pragma once
#endif

#ifdef __cplusplus
extern "C" {
#endif

#include <lmcons.h>

//
// Function Prototypes - Share
//

_Check_return_
NET_API_STATUS NET_API_FUNCTION
NetShareAdd(
    _In_opt_ LMSTR servername,
    _In_ DWORD level,
    _When_( level ==  2, _In_reads_bytes_(sizeof(SHARE_INFO_2)) )
    _When_( level ==  502, _In_reads_bytes_(sizeof(SHARE_INFO_502)) )
    _When_( level ==  503, _In_reads_bytes_(sizeof(SHARE_INFO_503)) )
    LPBYTE buf,
    _Out_opt_ LPDWORD parm_err
    );

_Check_return_
_Success_( return == 0 || return == ERROR_MORE_DATA )
NET_API_STATUS NET_API_FUNCTION
NetShareEnum(
    _In_opt_ LMSTR servername,
    _In_ DWORD level,
    _When_( level == 0,_Outptr_result_bytebuffer_(*entriesread * sizeof(SHARE_INFO_0)) )
    _When_( level == 1, _Outptr_result_bytebuffer_(*entriesread * sizeof(SHARE_INFO_1)) )
    _When_( level == 2, _Outptr_result_bytebuffer_(*entriesread * sizeof(SHARE_INFO_2)) )
    _When_( level == 502, _Outptr_result_bytebuffer_(*entriesread * sizeof(SHARE_INFO_502)) )
    _When_( level == 503, _Outptr_result_bytebuffer_(*entriesread * sizeof(SHARE_INFO_503)) )
    LPBYTE *bufptr,
    _In_ DWORD prefmaxlen,
    _Out_ LPDWORD entriesread,
    _Out_ LPDWORD totalentries,
    _Inout_opt_ LPDWORD resume_handle
    );

_Check_return_
_Success_( return == 0 || return == ERROR_MORE_DATA )
NET_API_STATUS NET_API_FUNCTION
NetShareEnumSticky(
    _In_opt_ LMSTR servername,
    _In_ DWORD level,
    _When_( level == 0, _Outptr_result_bytebuffer_(*entriesread * sizeof(SHARE_INFO_0)) )
    _When_( level == 1, _Outptr_result_bytebuffer_(*entriesread * sizeof(SHARE_INFO_1)) )
    _When_( level == 2, _Outptr_result_bytebuffer_(*entriesread * sizeof(SHARE_INFO_2)) )
    _When_( level == 502, _Outptr_result_bytebuffer_(*entriesread * sizeof(SHARE_INFO_502)) )
    _When_( level == 503, _Outptr_result_bytebuffer_(*entriesread * sizeof(SHARE_INFO_503)) )
    LPBYTE *bufptr,
    _In_ DWORD prefmaxlen,
    _Out_ LPDWORD entriesread,
    _Out_ LPDWORD totalentries,
    _Inout_opt_ LPDWORD resume_handle
    );

_Check_return_
_Success_( return == 0 || return == ERROR_MORE_DATA )
NET_API_STATUS NET_API_FUNCTION
NetShareGetInfo(
    _In_opt_ LMSTR servername,
    _In_ LMSTR netname,
    _In_ DWORD level,
    _When_( level == 0, _Outptr_result_bytebuffer_(sizeof(SHARE_INFO_0)) )
    _When_( level == 1, _Outptr_result_bytebuffer_(sizeof(SHARE_INFO_1)) )
    _When_( level == 2, _Outptr_result_bytebuffer_(sizeof(SHARE_INFO_2)) )
    _When_( level == 501, _Outptr_result_bytebuffer_(sizeof(SHARE_INFO_501)) )
    _When_( level == 502, _Outptr_result_bytebuffer_(sizeof(SHARE_INFO_502)) )
    _When_( level == 503, _Outptr_result_bytebuffer_(sizeof(SHARE_INFO_503)) )
    _When_( level == 1005, _Outptr_result_bytebuffer_(sizeof(SHARE_INFO_1005)) )
    LPBYTE *bufptr
    );

_Check_return_
NET_API_STATUS NET_API_FUNCTION
NetShareSetInfo(
    _In_opt_ LMSTR servername,
    _In_ LMSTR netname,
    _In_ DWORD level,
    _When_( level == 1, _In_reads_bytes_(sizeof(SHARE_INFO_1)) )
    _When_( level == 2, _In_reads_bytes_(sizeof(SHARE_INFO_2)) )
    _When_( level == 502, _In_reads_bytes_(sizeof(SHARE_INFO_502)) )
    _When_( level == 503, _In_reads_bytes_(sizeof(SHARE_INFO_503)) )
    _When_( level == 1004, _In_reads_bytes_(sizeof(SHARE_INFO_1004)) )
    _When_( level == 1005, _In_reads_bytes_(sizeof(SHARE_INFO_1005)) )
    _When_( level == 1006, _In_reads_bytes_(sizeof(SHARE_INFO_1006)) )
    _When_( level == 1501, _In_reads_bytes_(sizeof(SHARE_INFO_1501)) )
    LPBYTE  buf,
    _Out_opt_ LPDWORD parm_err
    );

_Check_return_
NET_API_STATUS NET_API_FUNCTION
NetShareDel(
    _In_opt_ LMSTR servername,
    _In_ LMSTR netname,
    _Reserved_ DWORD reserved
    );

_Check_return_
NET_API_STATUS NET_API_FUNCTION
NetShareDelSticky(
    _In_opt_ LMSTR servername,
    _In_ LMSTR netname,
    _Reserved_ DWORD reserved
    );

_Check_return_
NET_API_STATUS NET_API_FUNCTION
NetShareCheck(
    _In_opt_ LMSTR servername,
    _In_ LMSTR device,
    _Out_ LPDWORD type
    );

_Check_return_
NET_API_STATUS NET_API_FUNCTION
NetShareDelEx(
    _In_opt_ LMSTR servername,
    _In_ DWORD level,
    _When_( level == 0, _In_reads_bytes_(sizeof(SHARE_INFO_0)) )
    _When_( level == 1, _In_reads_bytes_(sizeof(SHARE_INFO_1)) )
    _When_( level == 2, _In_reads_bytes_(sizeof(SHARE_INFO_2)) )
    _When_( level == 502, _In_reads_bytes_(sizeof(SHARE_INFO_502)) )
    _When_( level == 503, _In_reads_bytes_(sizeof(SHARE_INFO_503)) )
    LPBYTE buf
    );

//
// Data Structures - Share
//

typedef struct _SHARE_INFO_0 {
    LMSTR   shi0_netname;
} SHARE_INFO_0, *PSHARE_INFO_0, *LPSHARE_INFO_0;

typedef struct _SHARE_INFO_1 {
    LMSTR   shi1_netname;
    DWORD   shi1_type;
    LMSTR   shi1_remark;
} SHARE_INFO_1, *PSHARE_INFO_1, *LPSHARE_INFO_1;

typedef struct _SHARE_INFO_2 {
    LMSTR   shi2_netname;
    DWORD   shi2_type;
    LMSTR   shi2_remark;
    DWORD   shi2_permissions;
    DWORD   shi2_max_uses;
    DWORD   shi2_current_uses;
    LMSTR   shi2_path;
    LMSTR   shi2_passwd;
} SHARE_INFO_2, *PSHARE_INFO_2, *LPSHARE_INFO_2;

typedef struct _SHARE_INFO_501 {
    LMSTR   shi501_netname;
    DWORD   shi501_type;
    LMSTR   shi501_remark;
    DWORD   shi501_flags;
} SHARE_INFO_501, *PSHARE_INFO_501, *LPSHARE_INFO_501;

typedef struct _SHARE_INFO_502 {
    LMSTR     shi502_netname;
    DWORD     shi502_type;
    LMSTR     shi502_remark;
    DWORD     shi502_permissions;
    DWORD     shi502_max_uses;
    DWORD     shi502_current_uses;
    LMSTR     shi502_path;
    LMSTR     shi502_passwd;
    DWORD     shi502_reserved;
    PSECURITY_DESCRIPTOR  shi502_security_descriptor;
} SHARE_INFO_502, *PSHARE_INFO_502, *LPSHARE_INFO_502;

typedef struct _SHARE_INFO_503 {
    LMSTR     shi503_netname;
    DWORD     shi503_type;
    LMSTR     shi503_remark;
    DWORD     shi503_permissions;
    DWORD     shi503_max_uses;
    DWORD     shi503_current_uses;
    LMSTR     shi503_path;
    LMSTR     shi503_passwd;
    LMSTR     shi503_servername;
    DWORD     shi503_reserved;
    PSECURITY_DESCRIPTOR  shi503_security_descriptor;
} SHARE_INFO_503, *PSHARE_INFO_503, *LPSHARE_INFO_503;

typedef struct _SHARE_INFO_1004 {
    LMSTR   shi1004_remark;
} SHARE_INFO_1004, *PSHARE_INFO_1004, *LPSHARE_INFO_1004;

typedef struct _SHARE_INFO_1005 {
    DWORD  shi1005_flags;
} SHARE_INFO_1005, *PSHARE_INFO_1005, *LPSHARE_INFO_1005;

typedef struct _SHARE_INFO_1006 {
    DWORD   shi1006_max_uses;
} SHARE_INFO_1006, *PSHARE_INFO_1006, *LPSHARE_INFO_1006;

typedef struct _SHARE_INFO_1501 {
    DWORD   shi1501_reserved;
    PSECURITY_DESCRIPTOR  shi1501_security_descriptor;
} SHARE_INFO_1501, *PSHARE_INFO_1501, *LPSHARE_INFO_1501;

typedef struct _SHARE_INFO_1503 {
    GUID shi1503_sharefilter;
} SHARE_INFO_1503, *PSHARE_INFO_1503, *LPSHARE_INFO_1503;

//
// NetShareAlias functions
//
_Check_return_
NET_API_STATUS NET_API_FUNCTION
NetServerAliasAdd(
    _In_opt_ LMSTR servername,
    _In_ DWORD level,
    _In_reads_bytes_(sizeof(SERVER_ALIAS_INFO_0)) LPBYTE buf
    );

_Check_return_
NET_API_STATUS NET_API_FUNCTION
NetServerAliasDel(
    _In_opt_ LMSTR servername,
    _In_ DWORD level,
    _When_( level == 0, _In_reads_bytes_(sizeof(SERVER_ALIAS_INFO_0_CONTAINER)) )
    LPBYTE buf
    );

_Check_return_
_Success_( return == 0 || return == ERROR_MORE_DATA )
NET_API_STATUS NET_API_FUNCTION
NetServerAliasEnum(
    _In_opt_ LMSTR servername,
    _In_ DWORD level,
    _When_( level == 0, _Outptr_result_bytebuffer_(*entriesread * sizeof(SERVER_ALIAS_INFO_0_CONTAINER)) )
    LPBYTE *bufptr,
    _In_ DWORD prefmaxlen,
    _Out_ LPDWORD entriesread,
    _Out_ LPDWORD totalentries,
    _Inout_opt_ LPDWORD resumehandle
    );

typedef struct _SERVER_ALIAS_INFO_0 {
    LMSTR      srvai0_alias;
    LMSTR      srvai0_target;
    BOOLEAN    srvai0_default;
    ULONG      srvai0_reserved;
} SERVER_ALIAS_INFO_0, *PSERVER_ALIAS_INFO_0, *LPSERVER_ALIAS_INFO_0;

//
// Special Values and Constants - Share
//

//
// Values for parm_err parameter.
//

#define SHARE_NETNAME_PARMNUM         1
#define SHARE_TYPE_PARMNUM            3
#define SHARE_REMARK_PARMNUM          4
#define SHARE_PERMISSIONS_PARMNUM     5
#define SHARE_MAX_USES_PARMNUM        6
#define SHARE_CURRENT_USES_PARMNUM    7
#define SHARE_PATH_PARMNUM            8
#define SHARE_PASSWD_PARMNUM          9
#define SHARE_FILE_SD_PARMNUM       501
#define SHARE_SERVER_PARMNUM        503
#define SHARE_QOS_POLICY_PARMNUM    504

//
// Single-field infolevels for NetShareSetInfo.
//

#define SHARE_REMARK_INFOLEVEL          \
            (PARMNUM_BASE_INFOLEVEL + SHARE_REMARK_PARMNUM)
#define SHARE_MAX_USES_INFOLEVEL        \
            (PARMNUM_BASE_INFOLEVEL + SHARE_MAX_USES_PARMNUM)
#define SHARE_FILE_SD_INFOLEVEL         \
            (PARMNUM_BASE_INFOLEVEL + SHARE_FILE_SD_PARMNUM)

#define SHI1_NUM_ELEMENTS       4
#define SHI2_NUM_ELEMENTS       10

//
// Share types (shi1_type and shi2_type fields).
//

#define STYPE_DISKTREE          0
#define STYPE_PRINTQ            1
#define STYPE_DEVICE            2
#define STYPE_IPC               3

#define STYPE_MASK              0x000000FF              // AND with shi_type to

#define STYPE_RESERVED1         0x01000000              // Reserved for internal processing
#define STYPE_RESERVED2         0x02000000
#define STYPE_RESERVED3         0x04000000
#define STYPE_RESERVED4         0x08000000
#define STYPE_RESERVED5         0x00100000
#define STYPE_RESERVED_ALL      0x3FFFFF00

#define STYPE_TEMPORARY         0x40000000
#define STYPE_SPECIAL           0x80000000

#define SHI_USES_UNLIMITED      (DWORD)-1

//
// Flags values for the 501 and 1005 levels
//
#define SHI1005_FLAGS_DFS       0x0001    // Share is in the DFS
#define SHI1005_FLAGS_DFS_ROOT  0x0002    // Share is root of DFS

#define CSC_MASK_EXT            0x2030    // Used to mask off the following states (including SHI1005_FLAGS_ENABLE_HASH)
#define CSC_MASK                0x0030    // Used to mask off the following states

#define CSC_CACHE_MANUAL_REINT  0x0000    // No automatic file by file reintegration
#define CSC_CACHE_AUTO_REINT    0x0010    // File by file reintegration is OK
#define CSC_CACHE_VDO           0x0020    // no need to flow opens
#define CSC_CACHE_NONE          0x0030    // no CSC for this share

#define SHI1005_FLAGS_RESTRICT_EXCLUSIVE_OPENS                  0x000100          // Used to disallow read-deny read behavior
#define SHI1005_FLAGS_FORCE_SHARED_DELETE                       0x000200          // Used to allows force shared delete
#define SHI1005_FLAGS_ALLOW_NAMESPACE_CACHING                   0x000400          // The clients may cache the namespace
#define SHI1005_FLAGS_ACCESS_BASED_DIRECTORY_ENUM               0x000800          // Trim visible files in enumerations based on access
#define SHI1005_FLAGS_FORCE_LEVELII_OPLOCK                      0x001000          // Only issue level2 oplock
#define SHI1005_FLAGS_ENABLE_HASH                               0x002000          // Enable hash generation and retrieval requests from share
#define SHI1005_FLAGS_ENABLE_CA                                 0x004000          // Enable continuously available
#define SHI1005_FLAGS_ENCRYPT_DATA                              0x008000          // Require encryption
#define SHI1005_FLAGS_RESERVED                                  0x010000          // Reserved for internal use
#define SHI1005_FLAGS_DISABLE_CLIENT_BUFFERING                  0x020000          // Used to set the allowed client buffering
#define SHI1005_FLAGS_IDENTITY_REMOTING                         0x040000          // Allows auth tunneling
#define SHI1005_FLAGS_CLUSTER_MANAGED                           0x080000          // Used to prevent share from being modified by users
#define SHI1005_FLAGS_COMPRESS_DATA                             0x100000          // Request compression
#define SHI1005_FLAGS_ISOLATED_TRANSPORT                        0x200000          // Request asymmetric connections for this share
#define SHI1005_FLAGS_DISABLE_DIRECTORY_HANDLE_LEASING          0x400000          // Used to disable handle caching for directories

//
// The subset of 1005 infolevel flags that can be set via the API
//

#define SHI1005_VALID_FLAGS_SET    (CSC_MASK|                                           \
                                    SHI1005_FLAGS_RESTRICT_EXCLUSIVE_OPENS|             \
                                    SHI1005_FLAGS_FORCE_SHARED_DELETE|                  \
                                    SHI1005_FLAGS_ALLOW_NAMESPACE_CACHING|              \
                                    SHI1005_FLAGS_ACCESS_BASED_DIRECTORY_ENUM|          \
                                    SHI1005_FLAGS_FORCE_LEVELII_OPLOCK|                 \
                                    SHI1005_FLAGS_ENABLE_HASH|                          \
                                    SHI1005_FLAGS_ENABLE_CA |                           \
                                    SHI1005_FLAGS_ENCRYPT_DATA |                        \
                                    SHI1005_FLAGS_DISABLE_CLIENT_BUFFERING |            \
                                    SHI1005_FLAGS_IDENTITY_REMOTING |                   \
                                    SHI1005_FLAGS_CLUSTER_MANAGED |                     \
                                    SHI1005_FLAGS_RESERVED |                            \
                                    SHI1005_FLAGS_COMPRESS_DATA |                       \
                                    SHI1005_FLAGS_ISOLATED_TRANSPORT |                  \
                                    SHI1005_FLAGS_DISABLE_DIRECTORY_HANDLE_LEASING)

#endif // _LMSHARE_

//
// SESSION API
//

#ifndef _LMSESSION_
#define _LMSESSION_

//
// Function Prototypes Session
//
_Check_return_
_Success_( return == 0 || return == ERROR_MORE_DATA )
NET_API_STATUS NET_API_FUNCTION
NetSessionEnum(
    _In_opt_ LMSTR servername,
    _In_opt_ LMSTR UncClientName,
    _In_opt_ LMSTR username,
    _In_ DWORD level,
    _When_( level ==  0, _Outptr_result_bytebuffer_(*entriesread * sizeof(SESSION_INFO_0)) )
    _When_( level ==  1, _Outptr_result_bytebuffer_(*entriesread * sizeof(SESSION_INFO_1)) )
    _When_( level ==  2, _Outptr_result_bytebuffer_(*entriesread * sizeof(SESSION_INFO_2)) )
    _When_( level ==  10, _Outptr_result_bytebuffer_(*entriesread * sizeof(SESSION_INFO_10)) )
    _When_( level ==  502, _Outptr_result_bytebuffer_(*entriesread * sizeof(SESSION_INFO_502)) )
    LPBYTE *bufptr,
    _In_ DWORD prefmaxlen,
    _Out_ LPDWORD entriesread,
    _Out_ LPDWORD totalentries,
    _Inout_opt_ LPDWORD resume_handle
    );

_Check_return_
NET_API_STATUS NET_API_FUNCTION
NetSessionDel(
    _In_opt_ LMSTR servername,
    _In_opt_ LMSTR UncClientName,
    _In_opt_ LMSTR username
    );

_Check_return_
_Success_( return == 0 )
NET_API_STATUS NET_API_FUNCTION
NetSessionGetInfo(
    _In_opt_ LMSTR servername,
    _In_ LMSTR UncClientName,
    _In_ LMSTR username,
    _In_ DWORD level,
    _When_( level == 0, _Outptr_result_bytebuffer_(sizeof(SESSION_INFO_0)) )
    _When_( level == 1, _Outptr_result_bytebuffer_(sizeof(SESSION_INFO_1)) )
    _When_( level == 2, _Outptr_result_bytebuffer_(sizeof(SESSION_INFO_2)) )
    _When_( level == 10, _Outptr_result_bytebuffer_(sizeof(SESSION_INFO_10)) )
    LPBYTE *bufptr
    );

//
// Data Structures - Session
//

typedef struct _SESSION_INFO_0 {
    LMSTR     sesi0_cname;              // client name (no backslashes)
} SESSION_INFO_0, *PSESSION_INFO_0, *LPSESSION_INFO_0;

typedef struct _SESSION_INFO_1 {
    LMSTR     sesi1_cname;              // client name (no backslashes)
    LMSTR     sesi1_username;
    DWORD     sesi1_num_opens;
    DWORD     sesi1_time;
    DWORD     sesi1_idle_time;
    DWORD     sesi1_user_flags;
} SESSION_INFO_1, *PSESSION_INFO_1, *LPSESSION_INFO_1;

typedef struct _SESSION_INFO_2 {
    LMSTR     sesi2_cname;              // client name (no backslashes)
    LMSTR     sesi2_username;
    DWORD     sesi2_num_opens;
    DWORD     sesi2_time;
    DWORD     sesi2_idle_time;
    DWORD     sesi2_user_flags;
    LMSTR     sesi2_cltype_name;
} SESSION_INFO_2, *PSESSION_INFO_2, *LPSESSION_INFO_2;

typedef struct _SESSION_INFO_10 {
    LMSTR     sesi10_cname;             // client name (no backslashes)
    LMSTR     sesi10_username;
    DWORD     sesi10_time;
    DWORD     sesi10_idle_time;
} SESSION_INFO_10, *PSESSION_INFO_10, *LPSESSION_INFO_10;

typedef struct _SESSION_INFO_502 {
    LMSTR     sesi502_cname;             // client name (no backslashes)
    LMSTR     sesi502_username;
    DWORD     sesi502_num_opens;
    DWORD     sesi502_time;
    DWORD     sesi502_idle_time;
    DWORD     sesi502_user_flags;
    LMSTR     sesi502_cltype_name;
    LMSTR     sesi502_transport;
} SESSION_INFO_502, *PSESSION_INFO_502, *LPSESSION_INFO_502;

//
// Special Values and Constants - Session
//

//
// Bits defined in sesi1_user_flags.
//

#define SESS_GUEST          0x00000001  // session is logged on as a guest
#define SESS_NOENCRYPTION   0x00000002  // session is not using encryption

#define SESI1_NUM_ELEMENTS  8
#define SESI2_NUM_ELEMENTS  9

#endif // _LMSESSION_

//
// CONNECTION API
//

#ifndef _LMCONNECTION_

#define _LMCONNECTION_

//
// Function Prototypes - CONNECTION
//

_Check_return_
_Success_( return == 0  || return == ERROR_MORE_DATA )
NET_API_STATUS NET_API_FUNCTION
NetConnectionEnum(
    _In_opt_ LMSTR servername,
    _In_ LMSTR qualifier,
    _In_ DWORD level,
    _When_( level == 0, _Outptr_result_bytebuffer_(*entriesread * sizeof(CONNECTION_INFO_0)) )
    _When_( level == 1, _Outptr_result_bytebuffer_(*entriesread * sizeof(CONNECTION_INFO_1)) )
    LPBYTE *bufptr,
    _In_ DWORD prefmaxlen,
    _Out_ LPDWORD entriesread,
    _Out_ LPDWORD totalentries,
    _Inout_opt_ LPDWORD resume_handle
    );

//
// Data Structures - CONNECTION
//

typedef struct _CONNECTION_INFO_0 {
    DWORD   coni0_id;
} CONNECTION_INFO_0, *PCONNECTION_INFO_0, *LPCONNECTION_INFO_0;

typedef struct _CONNECTION_INFO_1 {
    DWORD   coni1_id;
    DWORD   coni1_type;
    DWORD   coni1_num_opens;
    DWORD   coni1_num_users;
    DWORD   coni1_time;
    LMSTR   coni1_username;
    LMSTR   coni1_netname;
} CONNECTION_INFO_1, *PCONNECTION_INFO_1, *LPCONNECTION_INFO_1;

#endif // _LMCONNECTION_

//
// FILE API
//

#ifndef _LMFILE_
#define _LMFILE_

//
// Function Prototypes - FILE
//

_Check_return_
NET_API_STATUS NET_API_FUNCTION
NetFileClose(
    _In_opt_ LMSTR servername,
    _In_ DWORD fileid
    );

_Check_return_
_Success_( return == 0 || return == ERROR_MORE_DATA )
NET_API_STATUS NET_API_FUNCTION
NetFileEnum(
    _In_opt_ LMSTR servername,
    _In_opt_ LMSTR basepath,
    _In_opt_ LMSTR username,
    _In_ DWORD level,
    _When_( level == 2, _Outptr_result_bytebuffer_(*entriesread * sizeof(FILE_INFO_2)) )
    _When_( level == 3, _Outptr_result_bytebuffer_(*entriesread * sizeof(FILE_INFO_3)) )
    LPBYTE *bufptr,
    _In_ DWORD prefmaxlen,
    _Out_ LPDWORD entriesread,
    _Out_ LPDWORD totalentries,
    _Inout_opt_ PDWORD_PTR resume_handle
    );

_Check_return_
_Success_( return == 0)
NET_API_STATUS NET_API_FUNCTION
NetFileGetInfo(
    _In_opt_ LMSTR servername,
    _In_ DWORD fileid,
    _In_ DWORD level,
    _When_( level == 2, _Outptr_result_bytebuffer_(sizeof(FILE_INFO_2)) )
    _When_( level == 3, _Outptr_result_bytebuffer_(sizeof(FILE_INFO_3)) )
    LPBYTE *bufptr
    );

//
// Data Structures - File
//

//  File APIs are available at information levels 2 & 3 only. Levels 0 &
//  1 are not supported.
//

typedef struct _FILE_INFO_2 {
    DWORD     fi2_id;
} FILE_INFO_2, *PFILE_INFO_2, *LPFILE_INFO_2;

typedef struct _FILE_INFO_3 {
    DWORD     fi3_id;
    DWORD     fi3_permissions;
    DWORD     fi3_num_locks;
    LMSTR     fi3_pathname;
    LMSTR     fi3_username;
} FILE_INFO_3, *PFILE_INFO_3, *LPFILE_INFO_3;

//
// Special Values and Constants - File
//

//
// bit values for permissions
//

#define PERM_FILE_READ      0x1 // user has read access
#define PERM_FILE_WRITE     0x2 // user has write access
#define PERM_FILE_CREATE    0x4 // user has create access

#ifdef __cplusplus
}
#endif

#endif // _LMFILE_

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion


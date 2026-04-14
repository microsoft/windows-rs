/*++ BUILD Version: 0004    // Increment this if a change has global effects

Copyright (c) 1991-1999  Microsoft Corporation

Module Name:

    LmRepl.h

Abstract:

    This file contains structures, function prototypes, and definitions
    for the replicator APIs.

Environment:

    User Mode - Win32
    Portable to any flat, 32-bit environment.  (Uses Win32 typedefs.)
    Requires ANSI C extensions: slash-slash comments, long external names.

Notes:

    You must include LmCons.h before this file.


--*/

#ifndef _LMREPL_
#define _LMREPL_

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#ifdef __cplusplus
extern "C" {
#endif

#pragma warning(push)
  // SAL Annotations not available for obsolete APIs 
#pragma warning(disable:28740)

//
// Replicator Configuration APIs
//

#define REPL_ROLE_EXPORT        1
#define REPL_ROLE_IMPORT        2
#define REPL_ROLE_BOTH          3


#define REPL_INTERVAL_INFOLEVEL         (PARMNUM_BASE_INFOLEVEL + 0)
#define REPL_PULSE_INFOLEVEL            (PARMNUM_BASE_INFOLEVEL + 1)
#define REPL_GUARDTIME_INFOLEVEL        (PARMNUM_BASE_INFOLEVEL + 2)
#define REPL_RANDOM_INFOLEVEL           (PARMNUM_BASE_INFOLEVEL + 3)


typedef struct _REPL_INFO_0 {
    DWORD          rp0_role;
    LPWSTR         rp0_exportpath;
    LPWSTR         rp0_exportlist;
    LPWSTR         rp0_importpath;
    LPWSTR         rp0_importlist;
    LPWSTR         rp0_logonusername;
    DWORD          rp0_interval;
    DWORD          rp0_pulse;
    DWORD          rp0_guardtime;
    DWORD          rp0_random;
} REPL_INFO_0, *PREPL_INFO_0, *LPREPL_INFO_0;

typedef struct _REPL_INFO_1000 {
    DWORD          rp1000_interval;
} REPL_INFO_1000, *PREPL_INFO_1000, *LPREPL_INFO_1000;

typedef struct _REPL_INFO_1001 {
    DWORD          rp1001_pulse;
} REPL_INFO_1001, *PREPL_INFO_1001, *LPREPL_INFO_1001;

typedef struct _REPL_INFO_1002 {
    DWORD          rp1002_guardtime;
} REPL_INFO_1002, *PREPL_INFO_1002, *LPREPL_INFO_1002;

typedef struct _REPL_INFO_1003 {
    DWORD          rp1003_random;
} REPL_INFO_1003, *PREPL_INFO_1003, *LPREPL_INFO_1003;


NET_API_STATUS NET_API_FUNCTION
NetReplGetInfo (
    IN LPCWSTR servername OPTIONAL,
    IN DWORD level,
    OUT LPBYTE * bufptr
    );

NET_API_STATUS NET_API_FUNCTION
NetReplSetInfo (
    IN LPCWSTR servername OPTIONAL,
    IN DWORD level,
    IN const LPBYTE buf,
    OUT LPDWORD parm_err OPTIONAL
    );


//
// Replicator Export Directory APIs
//

#define REPL_INTEGRITY_FILE     1
#define REPL_INTEGRITY_TREE     2


#define REPL_EXTENT_FILE        1
#define REPL_EXTENT_TREE        2


#define REPL_EXPORT_INTEGRITY_INFOLEVEL (PARMNUM_BASE_INFOLEVEL + 0)
#define REPL_EXPORT_EXTENT_INFOLEVEL    (PARMNUM_BASE_INFOLEVEL + 1)


typedef struct _REPL_EDIR_INFO_0 {
    LPWSTR         rped0_dirname;
} REPL_EDIR_INFO_0, *PREPL_EDIR_INFO_0, *LPREPL_EDIR_INFO_0;

typedef struct _REPL_EDIR_INFO_1 {
    LPWSTR         rped1_dirname;
    DWORD          rped1_integrity;
    DWORD          rped1_extent;
} REPL_EDIR_INFO_1, *PREPL_EDIR_INFO_1, *LPREPL_EDIR_INFO_1;

typedef struct _REPL_EDIR_INFO_2 {
    LPWSTR         rped2_dirname;
    DWORD          rped2_integrity;
    DWORD          rped2_extent;
    DWORD          rped2_lockcount;
    DWORD          rped2_locktime;
} REPL_EDIR_INFO_2, *PREPL_EDIR_INFO_2, *LPREPL_EDIR_INFO_2;

typedef struct _REPL_EDIR_INFO_1000 {
    DWORD          rped1000_integrity;
} REPL_EDIR_INFO_1000, *PREPL_EDIR_INFO_1000, *LPREPL_EDIR_INFO_1000;

typedef struct _REPL_EDIR_INFO_1001 {
    DWORD          rped1001_extent;
} REPL_EDIR_INFO_1001, *PREPL_EDIR_INFO_1001, *LPREPL_EDIR_INFO_1001;


NET_API_STATUS NET_API_FUNCTION
NetReplExportDirAdd (
    IN LPCWSTR servername OPTIONAL,
    IN DWORD level,
    IN const LPBYTE buf,
    OUT LPDWORD parm_err OPTIONAL
    );

NET_API_STATUS NET_API_FUNCTION
NetReplExportDirDel (
    IN LPCWSTR servername OPTIONAL,
    IN LPCWSTR dirname
    );

NET_API_STATUS NET_API_FUNCTION
NetReplExportDirEnum (
    IN LPCWSTR servername OPTIONAL,
    IN DWORD level,
    OUT LPBYTE * bufptr,
    IN DWORD prefmaxlen,
    OUT LPDWORD entriesread,
    OUT LPDWORD totalentries,
    IN OUT LPDWORD resumehandle OPTIONAL
    );

NET_API_STATUS NET_API_FUNCTION
NetReplExportDirGetInfo (
    IN LPCWSTR servername OPTIONAL,
    IN LPCWSTR dirname,
    IN DWORD level,
    OUT LPBYTE * bufptr
    );

NET_API_STATUS NET_API_FUNCTION
NetReplExportDirSetInfo (
    IN LPCWSTR servername OPTIONAL,
    IN LPCWSTR dirname,
    IN DWORD level,
    IN const LPBYTE buf,
    OUT LPDWORD parm_err OPTIONAL
    );

NET_API_STATUS NET_API_FUNCTION
NetReplExportDirLock (
    IN LPCWSTR servername OPTIONAL,
    IN LPCWSTR dirname
    );

NET_API_STATUS NET_API_FUNCTION
NetReplExportDirUnlock (
    IN LPCWSTR servername OPTIONAL,
    IN LPCWSTR dirname,
    IN DWORD unlockforce
    );


#define REPL_UNLOCK_NOFORCE     0
#define REPL_UNLOCK_FORCE       1


//
// Replicator Import Directory APIs
//


typedef struct _REPL_IDIR_INFO_0 {
    LPWSTR         rpid0_dirname;
} REPL_IDIR_INFO_0, *PREPL_IDIR_INFO_0, *LPREPL_IDIR_INFO_0;

typedef struct _REPL_IDIR_INFO_1 {
    LPWSTR         rpid1_dirname;
    DWORD          rpid1_state;
    LPWSTR         rpid1_mastername;
    DWORD          rpid1_last_update_time;
    DWORD          rpid1_lockcount;
    DWORD          rpid1_locktime;
} REPL_IDIR_INFO_1, *PREPL_IDIR_INFO_1, *LPREPL_IDIR_INFO_1;


NET_API_STATUS NET_API_FUNCTION
NetReplImportDirAdd (
    IN LPCWSTR servername OPTIONAL,
    IN DWORD level,
    IN const LPBYTE buf,
    OUT LPDWORD parm_err OPTIONAL
    );

NET_API_STATUS NET_API_FUNCTION
NetReplImportDirDel (
    IN LPCWSTR servername OPTIONAL,
    IN LPCWSTR dirname
    );

NET_API_STATUS NET_API_FUNCTION
NetReplImportDirEnum (
    IN LPCWSTR servername OPTIONAL,
    IN DWORD level,
    OUT LPBYTE * bufptr,
    IN DWORD prefmaxlen,
    OUT LPDWORD entriesread,
    OUT LPDWORD totalentries,
    IN OUT LPDWORD resumehandle OPTIONAL
    );

NET_API_STATUS NET_API_FUNCTION
NetReplImportDirGetInfo (
    IN LPCWSTR servername OPTIONAL,
    IN LPCWSTR dirname,
    IN DWORD level,
    OUT LPBYTE * bufptr
    );

NET_API_STATUS NET_API_FUNCTION
NetReplImportDirLock (
    IN LPCWSTR servername OPTIONAL,
    IN LPCWSTR dirname
    );


NET_API_STATUS NET_API_FUNCTION
NetReplImportDirUnlock (
    IN LPCWSTR servername OPTIONAL,
    IN LPCWSTR dirname,
    IN DWORD unlockforce
    );


#pragma warning(pop)

#define REPL_STATE_OK                   0
#define REPL_STATE_NO_MASTER            1
#define REPL_STATE_NO_SYNC              2
#define REPL_STATE_NEVER_REPLICATED     3


#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif //_LMREPL_

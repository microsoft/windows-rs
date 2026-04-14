/*++ BUILD Version: 0001    // Increment this if a change has global effects

Copyright (c) 1991-1999  Microsoft Corporation

Module Name:

    lmuse.c

Abstract:

    This file contains structures, function prototypes, and definitions
    for the NetUse API.


Environment:

    User Mode - Win32
    Portable to any flat, 32-bit environment.  (Uses Win32 typedefs.)
    Requires ANSI C extensions: slash-slash comments, long external names.

Notes:

    You must include NETCONS.H before this file, since this file depends
    on values defined in NETCONS.H.


--*/

#ifndef _LMUSE_
#define _LMUSE_

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)


#ifdef __cplusplus
extern "C" {
#endif

#include <lmcons.h>
#include <lmuseflg.h>                   // Deletion force level flags

//
// LevelFlags : The lower 16 bits describe the use level while the upper 16 bits are flags.
//


#define USE_FLAG_GLOBAL_MAPPING 0x10000

#define USE_LEVEL(LEVELFLAGS) ((LEVELFLAGS) & 0xffff)
#define USE_FLAGS(LEVELFLAGS) ((LEVELFLAGS) & 0xffff0000)


//
// Function Prototypes
//

NET_API_STATUS NET_API_FUNCTION
NetUseAdd (
    _In_opt_ LPTSTR servername,
    _In_ DWORD LevelFlags,
    _When_( USE_LEVEL(LevelFlags)== 0, _In_reads_bytes_(sizeof(USE_INFO_0)))
    _When_( USE_LEVEL(LevelFlags)== 1, _In_reads_bytes_(sizeof(USE_INFO_1)))
    _When_( USE_LEVEL(LevelFlags)== 2, _In_reads_bytes_(sizeof(USE_INFO_2)))
    _When_( USE_LEVEL(LevelFlags)== 3, _In_reads_bytes_(sizeof(USE_INFO_3)))
    _When_( USE_LEVEL(LevelFlags)== 4, _In_reads_bytes_(sizeof(USE_INFO_4)))
    _When_( USE_LEVEL(LevelFlags)== 5, _In_reads_bytes_(sizeof(USE_INFO_5)))
    LPBYTE buf,
    _Out_opt_ LPDWORD parm_err
    );

NET_API_STATUS NET_API_FUNCTION
NetUseDel (
    _In_opt_ LMSTR  UncServerName OPTIONAL,
    _In_ LMSTR  UseName,
    _In_ DWORD ForceLevelFlags
    );

NET_API_STATUS NET_API_FUNCTION
NetUseEnum (
    _In_opt_ LMSTR  UncServerName,
    _In_ DWORD LevelFlags,
    _Outptr_opt_result_buffer_(_Inexpressible_(EntriesRead)) LPBYTE *BufPtr,
    _In_ DWORD PreferedMaximumSize,
    _Out_opt_ LPDWORD EntriesRead,
    _Out_ LPDWORD TotalEntries,
    _Inout_opt_ LPDWORD ResumeHandle
    );

NET_API_STATUS NET_API_FUNCTION
NetUseGetInfo (
    _In_opt_ LMSTR  UncServerName,
    _In_ LMSTR  UseName,
    _In_ DWORD LevelFlags,
    _When_(USE_LEVEL(LevelFlags) == 0, _Outptr_opt_result_bytebuffer_(sizeof(USE_INFO_0)))
    _When_(USE_LEVEL(LevelFlags) == 1, _Outptr_opt_result_bytebuffer_(sizeof(USE_INFO_1)))
    _When_(USE_LEVEL(LevelFlags) == 2, _Outptr_opt_result_bytebuffer_(sizeof(USE_INFO_2)))
    _When_(USE_LEVEL(LevelFlags) == 3, _Outptr_opt_result_bytebuffer_(sizeof(USE_INFO_3)))
    _When_(USE_LEVEL(LevelFlags) == 4, _Outptr_opt_result_bytebuffer_(sizeof(USE_INFO_4)))
    _When_(USE_LEVEL(LevelFlags) == 5, _Outptr_opt_result_bytebuffer_(sizeof(USE_INFO_5)))
    LPBYTE *bufptr
    );

//
//  Data Structures
//

typedef struct _USE_INFO_0 {
    LMSTR   ui0_local;
    LMSTR   ui0_remote;
}USE_INFO_0, *PUSE_INFO_0, *LPUSE_INFO_0;

typedef struct _USE_INFO_1 {
    LMSTR   ui1_local;
    LMSTR   ui1_remote;
    LMSTR   ui1_password;
    DWORD   ui1_status;
    DWORD   ui1_asg_type;
    DWORD   ui1_refcount;
    DWORD   ui1_usecount;
}USE_INFO_1, *PUSE_INFO_1, *LPUSE_INFO_1;

typedef struct _USE_INFO_2 {
    LMSTR    ui2_local;
    LMSTR    ui2_remote;
    LMSTR    ui2_password;
    DWORD    ui2_status;
    DWORD    ui2_asg_type;
    DWORD    ui2_refcount;
    DWORD    ui2_usecount;
    LMSTR    ui2_username;
    LMSTR    ui2_domainname;
}USE_INFO_2, *PUSE_INFO_2, *LPUSE_INFO_2;

typedef struct _USE_INFO_3 {
    USE_INFO_2 ui3_ui2;
    ULONG      ui3_flags;
} USE_INFO_3, *PUSE_INFO_3, *LPUSE_INFO_3;

typedef struct _USE_INFO_4 {
    USE_INFO_3 ui4_ui3;
    DWORD      ui4_auth_identity_length;
    PBYTE      ui4_auth_identity;
} USE_INFO_4, *PUSE_INFO_4, *LPUSE_INFO_4;

typedef struct _USE_INFO_5 {

    USE_INFO_3 ui4_ui3;
    DWORD      ui4_auth_identity_length;
    PBYTE      ui4_auth_identity;
    DWORD      ui5_security_descriptor_length;
    PBYTE      ui5_security_descriptor;
    DWORD      ui5_use_options_length;
    PBYTE      ui5_use_options;

} USE_INFO_5, *PUSE_INFO_5, *LPUSE_INFO_5;

//
// Special Values and Constants
//

//
// One of these values indicates the parameter within an information
// structure that is invalid when ERROR_INVALID_PARAMETER is returned by
// NetUseAdd.
//

#define USE_LOCAL_PARMNUM       	1
#define USE_REMOTE_PARMNUM      	2
#define USE_PASSWORD_PARMNUM    	3
#define USE_ASGTYPE_PARMNUM     	4
#define USE_USERNAME_PARMNUM    	5
#define USE_DOMAINNAME_PARMNUM  	6
#define USE_FLAGS_PARMNUM  		7
#define USE_AUTHIDENTITY_PARMNUM  	8
#define USE_SD_PARMNUM  		9
#define USE_OPTIONS_PARMNUM  		10

//
// Values appearing in the ui1_status field of use_info_1 structure.
// Note that USE_SESSLOST and USE_DISCONN are synonyms.
//

#define USE_OK                  0
#define USE_PAUSED              1
#define USE_SESSLOST            2
#define USE_DISCONN             2
#define USE_NETERR              3
#define USE_CONN                4
#define USE_RECONN              5


//
// Values of the ui1_asg_type field of use_info_1 structure
//

#define USE_WILDCARD            ( (DWORD) (-1) )
#define USE_DISKDEV             0
#define USE_SPOOLDEV            1
#define USE_CHARDEV             2
#define USE_IPC                 3

//
// Flags defined in the use_info_3 structure
//

#define CREATE_NO_CONNECT 0x1        // creation flags
#define CREATE_BYPASS_CSC 0x2        // force connection to server, bypassing CSC
                                     //  all ops on this connection go to the server,
                                     //  never to the cache
#define CREATE_CRED_RESET 0x4	     // Create a connection with credentials passed in 
                     //  this netuse if none exist. If connection already 
                     //  exists then update credentials after issuing remote
                     //  tree connection. This is needed as CSC cannot verify 
                     //  credentials while offline.

#define USE_DEFAULT_CREDENTIALS 0x4  // No explicit credentials passed to NetUseAdd

//
// Enforce connection level integrity and privacy.
//

#define CREATE_REQUIRE_CONNECTION_INTEGRITY 0x8
#define CREATE_REQUIRE_CONNECTION_PRIVACY   0x10

//
// Persist the mapping in the registry. (Only valid for global mappings.)
//

#define CREATE_PERSIST_MAPPING   0x20

//
// Enables write-through semantics on all files opened via this mapping.
//

#define CREATE_WRITE_THROUGH_SEMANTICS 0x40

#define CREATE_GLOBAL_MAPPING 0x100

//
// Use options
//
#define USE_OPTION_DEFERRED_CONNECTION_PARAMS 'CfeD'
#define USE_OPTION_TRANSPORT_PARAMS 'ParT'
#define USE_OPTION_SMB_COMPRESSION_PARAMS 'PmoC'
#define USE_OPTION_BLOCK_NTLM_PARAMS 'NolB'

typedef struct _USE_OPTION_GENERIC {
    ULONG  Tag;
    USHORT Length;
    USHORT Reserved;
} USE_OPTION_GENERIC, *PUSE_OPTION_GENERIC;

typedef struct _USE_OPTION_DEFERRED_CONNECTION_PARAMETERS {
    ULONG  Tag;      // 'CfeD'
    USHORT Length;   // sizeof(USE_OPTION_DEFERRED_CONNECTION_PARAMETERS) + sizeof(FILE_REMOTE_PROTOCOL_INFO)
    USHORT Reserved; // 0
    // Followed by FILE_REMOTE_PROTOCOL_INFO
} USE_OPTION_DEFERRED_CONNECTION_PARAMETERS, *PUSE_OPTION_DEFERRED_CONNECTION_PARAMETERS ;

typedef enum _TRANSPORT_TYPE {
   UseTransportType_None,
   UseTransportType_Wsk,
   UseTransportType_Quic
} TRANSPORT_TYPE, *PTRANSPORT_TYPE;

typedef enum _TRANSPORT_INFO_FLAG {
    NoneFlag               = 0x00000000,
    TcpPortSetFlag         = 0x00000001,
    QuicPortSetFlag        = 0x00000002,
    RdmaPortSetFlag        = 0x00000004
}  TRANSPORT_INFO_FLAG, *PTRANSPORT_INFO_FLAG;

typedef struct _TRANSPORT_INFO {
    TRANSPORT_TYPE Type;
    BOOLEAN SkipCertificateCheck;
    USHORT TcpPort;
    USHORT QuicPort;
    USHORT RdmaPort;
    ULONG Flags; // TRANSPORT_INFO_FLAG
} TRANSPORT_INFO, *PTRANSPORT_INFO;

typedef struct _USE_OPTION_TRANSPORT_PARAMETERS {
    ULONG  Tag;      // 'ParT'
    USHORT Length;   // sizeof(USE_OPTION_TRANSPORT_PARAMETERS) + sizeof(TRANSPORT_INFO)
    USHORT Reserved; // 0
    // Followed by TRANSPORT_INFO
} USE_OPTION_TRANSPORT_PARAMETERS, *PUSE_OPTION_TRANSPORT_PARAMETERS ;

typedef struct _SMB_COMPRESSION_INFO {
    BOOLEAN Switch; // FALSE: OFF; TRUE: ON;
    BYTE Reserved1; // 0
    USHORT Reserved2; // 0
    ULONG Reserved3; // 0
} SMB_COMPRESSION_INFO, *PSMB_COMPRESSION_INFO;

typedef struct _SMB_USE_OPTION_COMPRESSION_PARAMETERS {
    ULONG Tag;    // 'ParC'
    USHORT Length;
    USHORT Reserved;
} SMB_USE_OPTION_COMPRESSION_PARAMETERS, *PSMB_USE_OPTION_COMPRESSION_PARAMETERS;

typedef struct _BLOCK_NTLM_INFO {
    BOOLEAN BlockNTLM;
    BYTE Reserved1; // 0
    USHORT Reserved2; // 0
    ULONG Reserved3; // 0 
} BLOCK_NTLM_INFO, *PBLOCK_NTLM_INFO;

typedef struct _USE_OPTION_BLOCK_NTLM_PARAMETERS {
    ULONG Tag;    // 'NolB'
    USHORT Length;
    USHORT Reserved;
} USE_OPTION_BLOCK_NTLM_PARAMETERS, *PUSE_OPTION_BLOCK_NTLM_PARAMETERS;

typedef struct _SMB_TREE_CONNECT_PARAMETERS {
    ULONG EABufferOffset;  // relative offset 
    DWORD EABufferLen;
    ULONG CreateOptions;
    ULONG TreeConnectAttributes;
} SMB_TREE_CONNECT_PARAMETERS, *PSMB_TREE_CONNECT_PARAMETERS;

typedef struct _USE_OPTION_PROPERTIES {
    ULONG Tag;
    PVOID pInfo;
    size_t Length;
} USE_OPTION_PROPERTIES, *PUSE_OPTION_PROPERTIES;

#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif // _LMUSE_

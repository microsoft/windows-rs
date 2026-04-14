/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    netsh.h

Abstract:
    This file contains definitions which are needed by all NetSh helper DLLs.

--*/

#ifndef _NETSH_H_
#define _NETSH_H_

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#pragma warning(push)
#pragma warning(disable:4201) // nameless struct/union

#ifdef __cplusplus
extern "C" {
#endif

//
// Error codes
//
#define NETSH_ERROR_BASE                        15000
#define ERROR_NO_ENTRIES                        (NETSH_ERROR_BASE + 0)
#define ERROR_INVALID_SYNTAX                    (NETSH_ERROR_BASE + 1)
#define ERROR_PROTOCOL_NOT_IN_TRANSPORT         (NETSH_ERROR_BASE + 2)
#define ERROR_NO_CHANGE                         (NETSH_ERROR_BASE + 3)
#define ERROR_CMD_NOT_FOUND                     (NETSH_ERROR_BASE + 4)
#define ERROR_ENTRY_PT_NOT_FOUND                (NETSH_ERROR_BASE + 5)
#define ERROR_DLL_LOAD_FAILED                   (NETSH_ERROR_BASE + 6)
#define ERROR_INIT_DISPLAY                      (NETSH_ERROR_BASE + 7)
#define ERROR_TAG_ALREADY_PRESENT               (NETSH_ERROR_BASE + 8)
#define ERROR_INVALID_OPTION_TAG                (NETSH_ERROR_BASE + 9)
#define ERROR_NO_TAG                            (NETSH_ERROR_BASE + 10)
#define ERROR_MISSING_OPTION                    (NETSH_ERROR_BASE + 11)
#define ERROR_TRANSPORT_NOT_PRESENT             (NETSH_ERROR_BASE + 12)
#define ERROR_SHOW_USAGE                        (NETSH_ERROR_BASE + 13)
#define ERROR_INVALID_OPTION_VALUE              (NETSH_ERROR_BASE + 14)
#define ERROR_OKAY                              (NETSH_ERROR_BASE + 15)
#define ERROR_CONTINUE_IN_PARENT_CONTEXT        (NETSH_ERROR_BASE + 16)
#define ERROR_SUPPRESS_OUTPUT                   (NETSH_ERROR_BASE + 17)
#define ERROR_HELPER_ALREADY_REGISTERED         (NETSH_ERROR_BASE + 18)
#define ERROR_CONTEXT_ALREADY_REGISTERED        (NETSH_ERROR_BASE + 19)
#define ERROR_PARSING_FAILURE                   (NETSH_ERROR_BASE + 20)
#define NETSH_ERROR_END                ERROR_CONTEXT_ALREADY_REGISTERED

// Flags
enum NS_CMD_FLAGS
{
    CMD_FLAG_PRIVATE     = 0x01, // not valid in sub-contexts
    CMD_FLAG_INTERACTIVE = 0x02, // not valid from outside netsh
    CMD_FLAG_LOCAL       = 0x08, // not valid from a remote machine
    CMD_FLAG_ONLINE      = 0x10, // not valid in offline/non-commit mode
    CMD_FLAG_HIDDEN      = 0x20, // hide from help but allow execution
    CMD_FLAG_LIMIT_MASK  = 0xffff,
    CMD_FLAG_PRIORITY    = 0x80000000 // ulPriority field is used*/
};

typedef enum _NS_REQS
{
    NS_REQ_ZERO           = 0,
    NS_REQ_PRESENT        = 1,
    NS_REQ_ALLOW_MULTIPLE = 2,
    NS_REQ_ONE_OR_MORE    = 3
} NS_REQS;

enum NS_EVENTS
{
    NS_EVENT_LOOP       = 0x00010000,
    NS_EVENT_LAST_N     = 0x00000001,
    NS_EVENT_LAST_SECS  = 0x00000002,
    NS_EVENT_FROM_N     = 0x00000004,
    NS_EVENT_FROM_START = 0x00000008
};

enum NS_MODE_CHANGE
{
    NETSH_COMMIT                   = 0,
    NETSH_UNCOMMIT                 = 1,
    NETSH_FLUSH                    = 2,
    NETSH_COMMIT_STATE             = 3,
    NETSH_SAVE                     = 4
};


#define NS_GET_EVENT_IDS_FN_NAME    "GetEventIds"

// Same as MAX_DLL_NAME
#define MAX_NAME_LEN                    48

#define NETSH_VERSION_50                0x0005000

#define NETSH_ARG_DELIMITER             L"="
#define NETSH_CMD_DELIMITER             L" "

#define NETSH_MAX_TOKEN_LENGTH          64
#define NETSH_MAX_CMD_TOKEN_LENGTH      128

#define NETSH_ROOT_GUID { 0, 0, 0, { 0, 0, 0, 0, 0, 0, 0, 0 } }

#define DEFAULT_CONTEXT_PRIORITY 100

typedef struct _TOKEN_VALUE
{
    LPCWSTR  pwszToken; // literal token string
    DWORD    dwValue;   // ID of info string
} TOKEN_VALUE, *PTOKEN_VALUE;

// Macros
#define CREATE_CMD_ENTRY(t,f)            {CMD_##t, f, HLP_##t, HLP_##t##_EX, CMD_FLAG_PRIVATE, NULL}
#define CREATE_CMD_ENTRY_EX(t,f,i)       {CMD_##t, f, HLP_##t, HLP_##t##_EX, i, NULL}
#define CREATE_CMD_ENTRY_EX_VER(t,f,i,v) {CMD_##t, f, HLP_##t, HLP_##t##_EX, i, v}

#define CREATE_CMD_GROUP_ENTRY(t,s)            {CMD_##t, HLP_##t, sizeof(s)/sizeof(CMD_ENTRY), 0, s, NULL }
#define CREATE_CMD_GROUP_ENTRY_EX(t,s,i)       {CMD_##t, HLP_##t, sizeof(s)/sizeof(CMD_ENTRY), i, s, NULL }
#define CREATE_CMD_GROUP_ENTRY_EX_VER(t,s,i,v) {CMD_##t, HLP_##t, sizeof(s)/sizeof(CMD_ENTRY), i, s, v }

#define NUM_TOKENS_IN_TABLE(TokenArray) sizeof(TokenArray)/sizeof(TOKEN_VALUE)
#define NUM_TAGS_IN_TABLE(TagsArray)    sizeof(TagsArray)/sizeof(TAG_TYPE)

#define GET_RESOURCE_STRING_FN_NAME    "GetResourceString"

// Callbacks

typedef
DWORD
(WINAPI GET_RESOURCE_STRING_FN)(
    IN  DWORD   dwMsgID,         // resource identifier
    OUT LPWSTR  lpBuffer,        // resource buffer
    IN  DWORD   nBufferMax       // size of buffer in characters
    );

typedef GET_RESOURCE_STRING_FN *PGET_RESOURCE_STRING_FN;

typedef
DWORD
(WINAPI NS_CONTEXT_COMMIT_FN)(
    IN  DWORD       dwAction
    );

typedef NS_CONTEXT_COMMIT_FN *PNS_CONTEXT_COMMIT_FN;

typedef
DWORD
(WINAPI NS_CONTEXT_CONNECT_FN)(
    IN  LPCWSTR      pwszMachine
    );

typedef NS_CONTEXT_CONNECT_FN *PNS_CONTEXT_CONNECT_FN;

typedef struct _NS_CONTEXT_ATTRIBUTES NS_CONTEXT_ATTRIBUTES;

typedef
DWORD
(WINAPI NS_CONTEXT_DUMP_FN)(
    IN      LPCWSTR     pwszRouter,
    _In_reads_(dwArgCount) LPWSTR     *ppwcArguments,
    _In_    DWORD       dwArgCount,
    IN      LPCVOID     pvData
    );

typedef NS_CONTEXT_DUMP_FN *PNS_CONTEXT_DUMP_FN;

typedef
DWORD
(WINAPI NS_DLL_STOP_FN)(
    IN  DWORD       dwReserved
    );

typedef NS_DLL_STOP_FN *PNS_DLL_STOP_FN;

typedef
DWORD
(WINAPI NS_HELPER_START_FN)(
    _In_  CONST GUID *pguidParent,
    _In_  DWORD       dwVersion
    );

typedef NS_HELPER_START_FN *PNS_HELPER_START_FN;

typedef
DWORD
(WINAPI NS_HELPER_STOP_FN)(
    _In_  DWORD       dwReserved
    );

typedef NS_HELPER_STOP_FN *PNS_HELPER_STOP_FN;

typedef DWORD (FN_HANDLE_CMD)(
    IN      LPCWSTR   pwszMachine,
    _Inout_updates_(dwArgCount) LPWSTR   *ppwcArguments,
    IN      DWORD     dwCurrentIndex,
    IN      DWORD     dwArgCount,
    IN      DWORD     dwFlags,
    IN      LPCVOID   pvData,
    OUT     BOOL     *pbDone
    );

typedef FN_HANDLE_CMD *PFN_HANDLE_CMD;

typedef
BOOL
(WINAPI NS_OSVERSIONCHECK)(
    _In_  UINT     CIMOSType,                   // WMI: Win32_OperatingSystem  OSType
    _In_  UINT     CIMOSProductSuite,           // WMI: Win32_OperatingSystem  OSProductSuite
    _In_  LPCWSTR  CIMOSVersion,                // WMI: Win32_OperatingSystem  Version
    _In_  LPCWSTR  CIMOSBuildNumber,            // WMI: Win32_OperatingSystem  BuildNumber
    _In_  LPCWSTR  CIMServicePackMajorVersion,  // WMI: Win32_OperatingSystem  ServicePackMajorVersion
    _In_  LPCWSTR  CIMServicePackMinorVersion,  // WMI: Win32_OperatingSystem  ServicePackMinorVersion
    _In_  UINT     uiReserved,
    _In_  DWORD    dwReserved
    );

typedef NS_OSVERSIONCHECK *PNS_OSVERSIONCHECK;

// Structures
typedef struct _NS_HELPER_ATTRIBUTES
{
    union
    {
        struct
        {
            DWORD       dwVersion;
            DWORD       dwReserved;
        };
        ULONGLONG       _ullAlign;
    };
    GUID                      guidHelper;           // GUID associated with the helper
    PNS_HELPER_START_FN       pfnStart;             // Function to start this helper
    PNS_HELPER_STOP_FN        pfnStop;              // Function to stop this helper
} NS_HELPER_ATTRIBUTES, *PNS_HELPER_ATTRIBUTES;

typedef struct _CMD_ENTRY
{
    LPCWSTR             pwszCmdToken;           // The token for the command
    PFN_HANDLE_CMD      pfnCmdHandler;          // The function which handles this command
    DWORD               dwShortCmdHelpToken;    // The short help message
    DWORD               dwCmdHlpToken;          // The message to display if the only thing after the command is a help token (HELP, /?, -?, ?)
    DWORD               dwFlags;                // Flags (see CMD_FLAGS_xxx above)
    PNS_OSVERSIONCHECK  pOsVersionCheck;        // Check for the version of the OS this command can run against
} CMD_ENTRY, *PCMD_ENTRY;

typedef struct _CMD_GROUP_ENTRY
{
    LPCWSTR         pwszCmdGroupToken;      // The token for the command verb
    DWORD           dwShortCmdHelpToken;    // The message to display in a command listing.
    ULONG           ulCmdGroupSize;         // The number of entries in the cmd table
    DWORD           dwFlags;                // Flags (see CMD_FLAG_xxx)
    PCMD_ENTRY      pCmdGroup;              // The command table
    PNS_OSVERSIONCHECK  pOsVersionCheck;    // Check for the version of the OS this command can run against
} CMD_GROUP_ENTRY, *PCMD_GROUP_ENTRY;

typedef struct _NS_CONTEXT_ATTRIBUTES
{
    union
    {
        struct
        {
            DWORD       dwVersion;
            DWORD       dwReserved;
        };
        ULONGLONG       _ullAlign;
    };

    LPWSTR                   pwszContext;          // Name of the context
    GUID                     guidHelper;           // GUID of the helper servicing this context
    DWORD                    dwFlags;              // Flags limiting when context is available. (See CMD_FLAG_xxx)
    ULONG                    ulPriority;           // Priority field is only relevant if CMD_FLAG_PRIORITY is set in dwFlags
    ULONG                    ulNumTopCmds;         // Number of top-level commands
    struct _CMD_ENTRY        (*pTopCmds)[];        // Array of top-level commands
    ULONG                    ulNumGroups;          // Number of command groups
    struct _CMD_GROUP_ENTRY  (*pCmdGroups)[];      // Array of command groups

    PNS_CONTEXT_COMMIT_FN    pfnCommitFn;
    PNS_CONTEXT_DUMP_FN      pfnDumpFn;
    PNS_CONTEXT_CONNECT_FN   pfnConnectFn;
    PVOID                    pReserved;
    PNS_OSVERSIONCHECK       pfnOsVersionCheck;

} NS_CONTEXT_ATTRIBUTES, *PNS_CONTEXT_ATTRIBUTES;

typedef CONST struct _NS_CONTEXT_ATTRIBUTES * PCNS_CONTEXT_ATTRIBUTES;

typedef struct _TAG_TYPE
{
    LPCWSTR  pwszTag;     // tag string
    DWORD    dwRequired;  // required or not
    BOOL     bPresent;    // present or not
} TAG_TYPE, *PTAG_TYPE;

typedef
DWORD
(NS_DLL_INIT_FN)(
    IN  DWORD      dwNetshVersion,
    OUT PVOID      pReserved
    );

typedef NS_DLL_INIT_FN *PNS_DLL_INIT_FN;

DWORD WINAPI MatchEnumTag(
    IN  HANDLE             hModule,
    IN  LPCWSTR            pwcArg,
    IN  DWORD              dwNumArg,
    IN  CONST TOKEN_VALUE *pEnumTable,
    OUT PDWORD             pdwValue
    );

BOOL WINAPI MatchToken(
    IN  LPCWSTR  pwszUserToken,
    IN  LPCWSTR  pwszCmdToken
    );

DWORD WINAPI PreprocessCommand(
    _In_opt_                   HANDLE    hModule,
    _Inout_updates_(dwArgCount) LPWSTR   *ppwcArguments,
    _In_                       DWORD     dwCurrentIndex,
    _In_                       DWORD     dwArgCount,
    _Inout_updates_opt_(dwTagCount)    TAG_TYPE *pttTags,
    _In_                       DWORD     dwTagCount,
    _In_                       DWORD     dwMinArgs,
    _In_                       DWORD     dwMaxArgs,
    _Out_writes_opt_(dwArgCount-dwCurrentIndex)    DWORD    *pdwTagType
    );

DWORD PrintError(
    IN  HANDLE  hModule, OPTIONAL
    IN  DWORD   dwErrId,
    ...
    );

DWORD PrintMessageFromModule(
    IN  HANDLE  hModule,
    IN  DWORD   dwMsgId,
    ...
    );

DWORD PrintMessage(
    IN  LPCWSTR  pwszFormat,
    ...
    );

DWORD WINAPI RegisterContext(
    IN    CONST NS_CONTEXT_ATTRIBUTES *pChildContext
    );

DWORD WINAPI RegisterHelper(
    IN    CONST GUID                 *pguidParentContext,
    IN    CONST NS_HELPER_ATTRIBUTES *pfnRegisterSubContext
    );

#ifdef __cplusplus
}
#endif

#pragma warning(pop)


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _NETSH_H_

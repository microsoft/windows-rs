/*+--------------------------------------------------------------------------

Microsoft Windows
Copyright (c) Microsoft Corporation.  All rights reserved.

File:       tbs.h

Contents:   Definitions and prototypes for the TBS (TPM Base Services) for
            user mode(tbs.dll) and kernel mode(tbs.sys)


---------------------------------------------------------------------------*/

#ifndef _TBS_H_
#define _TBS_H_

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family or SecureStartup Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_SECURESTARTUP)

#if defined(__cplusplus)
extern "C" {
#endif

#if (NTDDI_VERSION >= NTDDI_VISTA)

#ifndef WINAPI 
#define WINAPI __stdcall
#endif

#define CONST const

typedef signed int          INT32,      *PINT32;
typedef const INT32                     *PCINT32;
typedef unsigned int        UINT32,     *PUINT32;
typedef const UINT32                    *PCUINT32;


#define VOID void
typedef VOID                            *PVOID;
typedef const VOID                      *PCVOID;

typedef INT32               BOOL,       *PBOOL;
typedef const BOOL                      *PCBOOL;

typedef UINT8               BYTE,       *PBYTE;
typedef const BYTE                      *PCBYTE;

typedef BOOL   TBS_BOOL;
typedef UINT32 TBS_RESULT;
typedef PVOID  TBS_HCONTEXT, *PTBS_HCONTEXT;
typedef UINT32 TBS_COMMAND_PRIORITY;
typedef UINT32 TBS_COMMAND_LOCALITY;
typedef UINT32 TBS_OWNERAUTH_TYPE;
typedef UINT32 TBS_HANDLE;

#define TBS_CONTEXT_VERSION_ONE 1

#define TBS_COMMAND_PRIORITY_LOW        100
#define TBS_COMMAND_PRIORITY_NORMAL     200
#define TBS_COMMAND_PRIORITY_HIGH       300
#define TBS_COMMAND_PRIORITY_SYSTEM     400
#define TBS_COMMAND_PRIORITY_MAX        0x80000000

#define TBS_COMMAND_LOCALITY_ZERO       0
#define TBS_COMMAND_LOCALITY_ONE        1
#define TBS_COMMAND_LOCALITY_TWO        2
#define TBS_COMMAND_LOCALITY_THREE      3
#define TBS_COMMAND_LOCALITY_FOUR       4

#define TBS_SUCCESS                     0

#define TBS_IN_OUT_BUF_SIZE_MAX         (256*1024)

#define TBS_OWNERAUTH_TYPE_FULL         1
#define TBS_OWNERAUTH_TYPE_ADMIN        2
#define TBS_OWNERAUTH_TYPE_USER         3
#define TBS_OWNERAUTH_TYPE_ENDORSEMENT  4

//
// TPM 2.0 OwnerAuth types
//

#define TBS_OWNERAUTH_TYPE_ENDORSEMENT_20  12
#define TBS_OWNERAUTH_TYPE_STORAGE_20      13

typedef struct tdTBS_CONTEXT_PARAMS
{
    UINT32 version;
} TBS_CONTEXT_PARAMS, *PTBS_CONTEXT_PARAMS;
typedef const TBS_CONTEXT_PARAMS *PCTBS_CONTEXT_PARAMS;

TBS_RESULT WINAPI
Tbsi_Context_Create(
    _In_ PCTBS_CONTEXT_PARAMS pContextParams,
    _Out_ PTBS_HCONTEXT phContext);

TBS_RESULT WINAPI
Tbsi_Tpm_Vendor_Maintenance_Mode(
    _In_ PCTBS_CONTEXT_PARAMS pContextParams,
    _Out_ PTBS_HCONTEXT phContext);

TBS_RESULT WINAPI
Tbsip_Context_Close(
    _In_ TBS_HCONTEXT   hContext);

TBS_RESULT WINAPI
Tbsip_Submit_Command(
    _In_ TBS_HCONTEXT hContext,
    _In_ TBS_COMMAND_LOCALITY Locality,
    _In_ TBS_COMMAND_PRIORITY Priority,
    _In_reads_bytes_(cbCommand) PCBYTE pabCommand,
    _In_ UINT32 cbCommand,
    _Out_writes_bytes_(*pcbResult) PBYTE pabResult,
    _Inout_ PUINT32 pcbResult);

TBS_RESULT WINAPI
Tbsip_Cancel_Commands(
    _In_ TBS_HCONTEXT hContext);

TBS_RESULT WINAPI
Tbsi_Physical_Presence_Command(
    _In_ TBS_HCONTEXT hContext,
    _In_reads_bytes_(cbInput) PCBYTE pabInput,
    _In_ UINT32 cbInput,
    _Out_writes_bytes_(*pcbOutput) PBYTE pabOutput,
    _Out_ PUINT32 pcbOutput);

#endif // (NTDDI_VERSION >= NTDDI_VISTA)

#if (NTDDI_VERSION >= NTDDI_VISTASP1)

_Success_(return == TBS_SUCCESS)
TBS_RESULT WINAPI
Tbsi_Get_TCG_Log(
    _In_ TBS_HCONTEXT hContext,
    _Out_writes_bytes_to_opt_(*pOutputBufLen, *pOutputBufLen) PBYTE pOutputBuf,
    _Inout_ PUINT32 pOutputBufLen);

#endif // _WIN32_WINNT_VISTASP1

#if (NTDDI_VERSION >= NTDDI_WIN8)

#define TBS_CONTEXT_VERSION_TWO 2

//
// Alternate TBS_CONTEXT_PARAMS if version == TBS_CONTEXT_VERSION_TWO
//

#pragma warning(push)
#pragma warning(disable: 4201) // nonstandard extension used : nameless struct/union

typedef struct tdTBS_CONTEXT_PARAMS2
{
    UINT32  version;

    union
    {
        struct
        {
            UINT32 requestRaw : 1;     // if set to 1, request raw context
            UINT32 includeTpm12 : 1;   // if 1.2 device present, can use this
            UINT32 includeTpm20 : 1;   // if 2.0 device present, can use this
        };
        UINT32  asUINT32;
    };
} TBS_CONTEXT_PARAMS2, *PTBS_CONTEXT_PARAMS2;
typedef const TBS_CONTEXT_PARAMS2 *PCTBS_CONTEXT_PARAMS2;

#pragma warning(pop)

//
//  32 byte structure used for the TPM Provisioning WNF notification.
//
typedef struct tdTPM_WNF_PROVISIONING
{
    UINT32 status;           //    4 bytes
    BYTE   message[28];      // + 28 bytes
} TPM_WNF_PROVISIONING;      // = 32 bytes

#define TPM_WNF_INFO_CLEAR_SUCCESSFUL       0x00000001
#define TPM_WNF_INFO_OWNERSHIP_SUCCESSFUL   0x00000002

#define TPM_WNF_INFO_NO_REBOOT_REQUIRED     1

#ifndef TPM_VERSION_UNKNOWN
//
// TPM device info.
//
#define TPM_VERSION_UNKNOWN     0
#define TPM_VERSION_12          1
#define TPM_VERSION_20          2

#define TPM_IFTYPE_UNKNOWN        0
#define TPM_IFTYPE_1              1 // for 1.2 - use I/O-port or MMIO
#define TPM_IFTYPE_TRUSTZONE      2 // 2.0: Trustzone
#define TPM_IFTYPE_HW             3 // 2.0: HW TPM
#define TPM_IFTYPE_EMULATOR       4 // 2.0: SW-emulator
#define TPM_IFTYPE_SPB            5 // 2.0: SPB attached

typedef struct _TPM_DEVICE_INFO
{
    UINT32      structVersion;      // = 1 for now
    UINT32      tpmVersion;         // 1.2 / 2.0
    UINT32      tpmInterfaceType;   // HW, simulator, ...
    UINT32      tpmImpRevision;     // code-drop revision,
                                    // implenmentation-specific
} TPM_DEVICE_INFO, *PTPM_DEVICE_INFO;
typedef const TPM_DEVICE_INFO *PCTPM_DEVICE_INFO;
#endif //TPM_VERSION_UNKNOWN

TBS_RESULT WINAPI
Tbsi_GetDeviceInfo(
    _In_ UINT32 Size,
    _Out_writes_bytes_(Size) PVOID Info);

_Success_(return == TBS_SUCCESS)
TBS_RESULT WINAPI
Tbsi_Get_OwnerAuth(
    _In_ TBS_HCONTEXT hContext,
    _In_ TBS_OWNERAUTH_TYPE ownerauthType,
    _Out_writes_bytes_to_opt_(*pOutputBufLen, *pOutputBufLen) PBYTE pOutputBuf,
    _Inout_  PUINT32 pOutputBufLen);

TBS_RESULT WINAPI
Tbsi_Revoke_Attestation();

#endif // (NTDDI_VERSION >= NTDDI_WIN8)

#if (NTDDI_VERSION >= NTDDI_WINBLUE)

#ifndef _NTDDK_

// These functions are not available in kernel mode

HRESULT
GetDeviceID(
    _Out_writes_bytes_to_opt_(cbWindowsAIK, *pcbResult) PBYTE pbWindowsAIK,
    _In_ UINT32 cbWindowsAIK,
    _Out_ PUINT32 pcbResult,
    _Out_opt_ BOOL *pfProtectedByTPM);

HRESULT
GetDeviceIDString(
    _Out_writes_to_opt_(cchWindowsAIK, *pcchResult) PWSTR pszWindowsAIK,
    _In_ UINT32 cchWindowsAIK,
    _Out_ PUINT32 pcchResult,
    _Out_opt_ BOOL *pfProtectedByTPM);

#endif // ifndef _NTDDK_

#endif // (NTDDI_VERSION >= NTDDI_WINBLUE)

#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)

TBS_RESULT WINAPI
Tbsi_Create_Windows_Key(
    __in TBS_HANDLE keyHandle);

#endif // (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)

#if (NTDDI_VERSION >= NTDDI_WIN10_RS4)

//
// Log types for Tbsi_Get_TCG_Log_Ex
//

#define TBS_TCGLOG_SRTM_CURRENT     0
// Returns the SRTM log associated with the current session (boot or resume)
#define TBS_TCGLOG_DRTM_CURRENT     1
// Returns the DRTM log associated with the current session (boot or resume)
#define TBS_TCGLOG_SRTM_BOOT        2
// (User mode only) Returns the SRTM log associated with the most recent clean boot
#define TBS_TCGLOG_SRTM_RESUME      3
// (User mode only) Returns the SRTM log associated with the current resume from hibernation
#define TBS_TCGLOG_DRTM_BOOT        4
// (User mode only) Returns the DRTM log associated with the most recent clean boot
#define TBS_TCGLOG_DRTM_RESUME      5
// (User mode only) Returns the DRTM log associated with the current resume from hibernation

_Success_(return == TBS_SUCCESS)
TBS_RESULT WINAPI
Tbsi_Get_TCG_Log_Ex(
    _In_ UINT32 logType,
    _Out_writes_bytes_opt_(*pcbOutput) PBYTE pbOutput,
    _Inout_ PUINT32 pcbOutput);

#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS4)

#if (NTDDI_VERSION >= NTDDI_WIN10_NI)

inline BOOL WINAPI
Tbsi_Is_Tpm_Present();

#endif // (NTDDI_VERSION >= NTDDI_WIN10_NI)

#if defined(__cplusplus)
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_SECURESTARTUP) */
#pragma endregion

#endif // ifndef _TBS_H_


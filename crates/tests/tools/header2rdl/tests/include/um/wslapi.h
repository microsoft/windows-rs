/********************************************************************************
*                                                                               *
* wslapi.h -- ApiSet Contract for api-ms-win-wsl-api-l1-1-0                     *
*                                                                               *
* Copyright (c) Microsoft Corporation. All rights reserved.                     *
*                                                                               *
********************************************************************************/

#ifdef _MSC_VER
#pragma once
#endif // _MSC_VER

#ifndef _WSLAPI_H_
#define _WSLAPI_H_

#include <apiset.h>
#include <apisetcconv.h>

#ifdef _CONTRACT_GEN
#include <nt.h>
#include <ntrtl.h>
#include <nturtl.h>
#include <wtypes.h>
#endif // _CONTRACT_GEN

#ifdef __cplusplus
extern "C" {
#endif

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SERVER)

/* Determines if a distribution is already registered */
BOOL
WslIsDistributionRegistered(
    _In_ PCWSTR distributionName
    );

/* Registers a new distribution given the information provided. */
HRESULT
WslRegisterDistribution(
    _In_ PCWSTR distributionName,
    _In_ PCWSTR tarGzFilename
    );

/* Unregisters the specified distribution */
HRESULT
WslUnregisterDistribution(
    _In_ PCWSTR distributionName
    );

/* Flags specifying WSL behavior */
typedef enum
{
    WSL_DISTRIBUTION_FLAGS_NONE                  = 0x0,
    WSL_DISTRIBUTION_FLAGS_ENABLE_INTEROP        = 0x1,
    WSL_DISTRIBUTION_FLAGS_APPEND_NT_PATH        = 0x2,
    WSL_DISTRIBUTION_FLAGS_ENABLE_DRIVE_MOUNTING = 0x4
} WSL_DISTRIBUTION_FLAGS;
DEFINE_ENUM_FLAG_OPERATORS(WSL_DISTRIBUTION_FLAGS);

#define WSL_DISTRIBUTION_FLAGS_VALID (WSL_DISTRIBUTION_FLAGS_ENABLE_INTEROP | WSL_DISTRIBUTION_FLAGS_APPEND_NT_PATH | WSL_DISTRIBUTION_FLAGS_ENABLE_DRIVE_MOUNTING)
#define WSL_DISTRIBUTION_FLAGS_DEFAULT (WSL_DISTRIBUTION_FLAGS_ENABLE_INTEROP | WSL_DISTRIBUTION_FLAGS_APPEND_NT_PATH | WSL_DISTRIBUTION_FLAGS_ENABLE_DRIVE_MOUNTING)

/* Configure the given distribution */
HRESULT
WslConfigureDistribution(
    _In_ PCWSTR distributionName,
    _In_ ULONG defaultUID,
    _In_ WSL_DISTRIBUTION_FLAGS wslDistributionFlags
    );

/* Get the given distribution's configuration info */
HRESULT
WslGetDistributionConfiguration(
    _In_ PCWSTR distributionName,
    _Out_ ULONG* distributionVersion,
    _Out_ ULONG* defaultUID,
    _Out_ WSL_DISTRIBUTION_FLAGS* wslDistributionFlags,
    _Outptr_result_buffer_(*defaultEnvironmentVariableCount) PSTR** defaultEnvironmentVariables,
    _Out_ ULONG* defaultEnvironmentVariableCount
    );

HRESULT
WslLaunchInteractive(
    _In_ PCWSTR distributionName,
    _In_opt_ PCWSTR command,
    _In_ BOOL useCurrentWorkingDirectory,
    _Out_ DWORD* exitCode
    );

HRESULT
WslLaunch(
    _In_ PCWSTR distributionName,
    _In_opt_ PCWSTR command,
    _In_ BOOL useCurrentWorkingDirectory,
    _In_ HANDLE stdIn,
    _In_ HANDLE stdOut,
    _In_ HANDLE stdErr,
    _Out_ HANDLE* process
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SERVER) */
#pragma endregion

#ifdef __cplusplus
}
#endif

#endif // _WSLAPI_H_

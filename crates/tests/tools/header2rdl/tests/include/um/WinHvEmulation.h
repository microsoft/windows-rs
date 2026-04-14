/*++

Copyright (c) Microsoft Corporation.  All rights reserved.

Module Name:

    WinHvEmu.ext

Abstract:

    ApiSet contract for the Windows Hyper-V Instruction Emulator APIs.

--*/

#ifndef _WINHVEMUAPI_H_
#define _WINHVEMUAPI_H_

#if defined(_MSC_VER) && (_MSC_VER >= 1200)
#pragma once
#pragma warning(push)
#pragma warning(disable:4201) /* nonstandard extension used : nameless struct/union */
#endif

#include <apiset.h>
#include <apisetcconv.h>
#include <minwindef.h>
#include <winapifamily.h>

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#include <WinHvPlatformDefs.h>

#if defined(_AMD64_)

typedef union WHV_EMULATOR_STATUS
{
    struct
    {
        UINT32 EmulationSuccessful : 1;
        UINT32 InternalEmulationFailure : 1;
        UINT32 IoPortCallbackFailed : 1;
        UINT32 MemoryCallbackFailed : 1;
        UINT32 TranslateGvaPageCallbackFailed : 1;
        UINT32 TranslateGvaPageCallbackGpaIsNotAligned : 1;
        UINT32 GetVirtualProcessorRegistersCallbackFailed : 1;
        UINT32 SetVirtualProcessorRegistersCallbackFailed : 1;
        UINT32 InterruptCausedIntercept : 1;
        UINT32 GuestCannotBeFaulted : 1;
        UINT32 Reserved : 22;
    };

    UINT32 AsUINT32;
} WHV_EMULATOR_STATUS;

#if defined(_MSC_VER) && (_MSC_VER >= 1200)
#pragma warning(pop)
#endif

//
// Callbacks registered in emulator creation
//

typedef struct WHV_EMULATOR_MEMORY_ACCESS_INFO
{
    WHV_GUEST_PHYSICAL_ADDRESS GpaAddress;
    UINT8 Direction;
    UINT8 AccessSize;
    UINT8 Data[8];
} WHV_EMULATOR_MEMORY_ACCESS_INFO;

typedef struct WHV_EMULATOR_IO_ACCESS_INFO
{
    UINT8 Direction;
    UINT16 Port;
    UINT16 AccessSize;
    UINT32 Data;
} WHV_EMULATOR_IO_ACCESS_INFO;

typedef HRESULT (CALLBACK *WHV_EMULATOR_IO_PORT_CALLBACK)(
    _In_ VOID* Context,
    _Inout_ WHV_EMULATOR_IO_ACCESS_INFO* IoAccess
    );

typedef HRESULT (CALLBACK *WHV_EMULATOR_MEMORY_CALLBACK)(
    _In_ VOID* Context,
    _Inout_ WHV_EMULATOR_MEMORY_ACCESS_INFO* MemoryAccess
    );

typedef HRESULT (CALLBACK *WHV_EMULATOR_GET_VIRTUAL_PROCESSOR_REGISTERS_CALLBACK)(
    _In_ VOID* Context,
    _In_reads_(RegisterCount) const WHV_REGISTER_NAME* RegisterNames,
    _In_ UINT32 RegisterCount,
    _Out_writes_(RegisterCount) WHV_REGISTER_VALUE* RegisterValues
    );

typedef HRESULT (CALLBACK *WHV_EMULATOR_SET_VIRTUAL_PROCESSOR_REGISTERS_CALLBACK)(
    _In_ VOID* Context,
    _In_reads_(RegisterCount) const WHV_REGISTER_NAME* RegisterNames,
    _In_ UINT32 RegisterCount,
    _In_reads_(RegisterCount) const WHV_REGISTER_VALUE* RegisterValues
    );

typedef HRESULT (CALLBACK *WHV_EMULATOR_TRANSLATE_GVA_PAGE_CALLBACK)(
    _In_ VOID* Context,
    _In_ WHV_GUEST_VIRTUAL_ADDRESS Gva,
    _In_ WHV_TRANSLATE_GVA_FLAGS TranslateFlags,
    _Out_ WHV_TRANSLATE_GVA_RESULT_CODE* TranslationResult,
    _Out_ WHV_GUEST_PHYSICAL_ADDRESS* Gpa
    );

typedef struct WHV_EMULATOR_CALLBACKS
{
    UINT32 Size;
    UINT32 Reserved;
    WHV_EMULATOR_IO_PORT_CALLBACK WHvEmulatorIoPortCallback;
    WHV_EMULATOR_MEMORY_CALLBACK WHvEmulatorMemoryCallback;
    WHV_EMULATOR_GET_VIRTUAL_PROCESSOR_REGISTERS_CALLBACK WHvEmulatorGetVirtualProcessorRegisters;
    WHV_EMULATOR_SET_VIRTUAL_PROCESSOR_REGISTERS_CALLBACK WHvEmulatorSetVirtualProcessorRegisters;
    WHV_EMULATOR_TRANSLATE_GVA_PAGE_CALLBACK WHvEmulatorTranslateGvaPage;
} WHV_EMULATOR_CALLBACKS;

typedef VOID* WHV_EMULATOR_HANDLE;

#endif

//
// Public callable functions
//

#ifdef __cplusplus
extern "C" {
#endif

HRESULT
WINAPI
WHvEmulatorCreateEmulator(
    _In_ const WHV_EMULATOR_CALLBACKS* Callbacks,
    _Out_ WHV_EMULATOR_HANDLE* Emulator
    );

HRESULT
WINAPI
WHvEmulatorDestroyEmulator(
    _In_ WHV_EMULATOR_HANDLE Emulator
    );

HRESULT
WINAPI
WHvEmulatorTryIoEmulation(
    _In_ WHV_EMULATOR_HANDLE Emulator,
    _In_ VOID* Context,
    _In_ const WHV_VP_EXIT_CONTEXT* VpContext,
    _In_ const WHV_X64_IO_PORT_ACCESS_CONTEXT* IoInstructionContext,
    _Out_ WHV_EMULATOR_STATUS* EmulatorReturnStatus
    );

HRESULT
WINAPI
WHvEmulatorTryMmioEmulation(
    _In_ WHV_EMULATOR_HANDLE Emulator,
    _In_ VOID* Context,
    _In_ const WHV_VP_EXIT_CONTEXT* VpContext,
    _In_ const WHV_MEMORY_ACCESS_CONTEXT* MmioInstructionContext,
    _Out_ WHV_EMULATOR_STATUS* EmulatorReturnStatus
    );

#ifdef __cplusplus
}
#endif

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#endif // _WINHVEMUAPI_H_

#ifndef ext_ms_win_hyperv_hvemulation_l1_1_0_query_routines
#define ext_ms_win_hyperv_hvemulation_l1_1_0_query_routines

//
//Private Extension API Query Routines
//

#ifdef __cplusplus
extern "C" {
#endif

#if defined(_AMD64_)

#endif

BOOLEAN
__stdcall
IsWHvEmulatorCreateEmulatorPresent(
    VOID
    );

BOOLEAN
__stdcall
IsWHvEmulatorDestroyEmulatorPresent(
    VOID
    );

BOOLEAN
__stdcall
IsWHvEmulatorTryIoEmulationPresent(
    VOID
    );

BOOLEAN
__stdcall
IsWHvEmulatorTryMmioEmulationPresent(
    VOID
    );

#ifdef __cplusplus
}
#endif

#endif // endof guard


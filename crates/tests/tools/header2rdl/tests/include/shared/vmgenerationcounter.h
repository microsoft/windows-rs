/*++

Copyright (c) Microsoft Corporation. All Rights Reserved.

Module Name:

    vmgenerationcounter.h

Abstract:

    This file implements the published interface for the virtual machine
    generation counter.

--*/

#ifdef _MSC_VER
#pragma once
#endif

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#if (NTDDI_VERSION >= NTDDI_WINXP)

// {3ff2c92b-6598-4e60-8e1c-0ccf4927e319}
DEFINE_GUID(GUID_DEVINTERFACE_VM_GENCOUNTER,
0x3ff2c92b, 0x6598, 0x4e60, 0x8e, 0x1c, 0x0c, 0xcf, 0x49, 0x27, 0xe3, 0x19);

#define VM_GENCOUNTER_SYMBOLIC_LINK_NAME L"\\VmGenerationCounter"

#define IOCTL_VMGENCOUNTER_READ CTL_CODE(FILE_DEVICE_ACPI, 0x1, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)

typedef struct _VM_GENCOUNTER
{
    ULONGLONG GenerationCount;
    ULONGLONG GenerationCountHigh;
} VM_GENCOUNTER, *PVM_GENCOUNTER;

#endif // (NTDDI_VERSION >= NTDDI_WINXP)

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#pragma endregion



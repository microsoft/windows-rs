/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    VmSavedStateDumpDefs.h

Abstract:

    This module contains the VmSavedSate Dump Provider definitions used by the APIs.

--*/

#ifndef _VMSAVEDSTATEDUMPDEFS_H_
#define _VMSAVEDSTATEDUMPDEFS_H_

#if defined(_MSC_VER) && (_MSC_VER >= 1200)
#pragma once
#endif

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#include <winapifamily.h>

typedef VOID* VM_SAVED_STATE_DUMP_HANDLE;

typedef UINT64 GUEST_VIRTUAL_ADDRESS;
typedef UINT64 GUEST_PHYSICAL_ADDRESS;

//
// Define paging modes
//
typedef enum PAGING_MODE
{
    Paging_Invalid = 0,
    Paging_NonPaged,
    Paging_32Bit,
    Paging_Pae,
    Paging_Long,
    Paging_Armv8,
} PAGING_MODE;


//
// Define guest physical memory chunks
//
typedef struct GPA_MEMORY_CHUNK
{
    UINT64  GuestPhysicalStartPageIndex;
    UINT64  PageCount;
} GPA_MEMORY_CHUNK;


//
// Define Virtual Processors dump information
//

typedef enum VIRTUAL_PROCESSOR_ARCH
{
    Arch_Unknown = 0,
    Arch_x86,
    Arch_x64,
    Arch_Armv8,
} VIRTUAL_PROCESSOR_ARCH;


//
// Define Processor Vendor dump information.
//

typedef enum VIRTUAL_PROCESSOR_VENDOR
{
    ProcessorVendor_Unknown,
    ProcessorVendor_Amd,
    ProcessorVendor_Intel,
    ProcessorVendor_Hygon,
    ProcessorVendor_Arm,
} VIRTUAL_PROCESSOR_VENDOR;


//
// Define Guest OS information.
//

typedef enum GUEST_OS_VENDOR
{
    GuestOsVendorUndefined  = 0x0000,
    GuestOsVendorMicrosoft  = 0x0001,
    GuestOsVendorHPE        = 0x0002,
    GuestOsVendorLANCOM     = 0x0200,
} GUEST_OS_VENDOR;


typedef enum GUEST_OS_MICROSOFT_IDS
{
    GuestOsMicrosoftUndefined   = 0x00,
    GuestOsMicrosoftMSDOS       = 0x01,
    GuestOsMicrosoftWindows3x   = 0x02,
    GuestOsMicrosoftWindows9x   = 0x03,
    GuestOsMicrosoftWindowsNT   = 0x04,
    GuestOsMicrosoftWindowsCE   = 0x05,
} GUEST_OS_MICROSOFT_IDS;


typedef enum GUEST_OS_OPENSOURCE_IDS
{
    GuestOsOpenSourceUndefined  = 0x00,
    GuestOsOpenSourceLinux      = 0x01,
    GuestOsOpenSourceFreeBSD    = 0x02,
    GuestOsOpenSourceXen        = 0x03,
    GuestOsOpenSourceIllumos    = 0x04,
} GUEST_OS_OPENSOURCE_IDS;


typedef union GUEST_OS_INFO
{
    UINT64 AsUINT64;
    struct
    {
        UINT64 BuildNumber    : 16;
        UINT64 ServiceVersion : 8; // Service Pack, etc.
        UINT64 MinorVersion   : 8;
        UINT64 MajorVersion   : 8;
        UINT64 OsId           : 8;
        UINT64 VendorId       : 16;
    } ClosedSource;

    struct
    {
        UINT64 VendorSpecific1 : 16;
        UINT64 Version         : 32;
        UINT64 VendorSpecific2 : 8;
        UINT64 OsId            : 7;
        UINT64 IsOpenSource    : 1;
    } OpenSource;
} GUEST_OS_INFO;

typedef enum REGISTER_ID
{
    //
    // X86 / X64 registers:
    //

    //
    // General Purpose Registers
    //

    X64_RegisterRax = 0,
    X64_RegisterRcx,
    X64_RegisterRdx,
    X64_RegisterRbx,
    X64_RegisterRsp,
    X64_RegisterRbp,
    X64_RegisterRsi,
    X64_RegisterRdi,
    X64_RegisterR8,
    X64_RegisterR9,
    X64_RegisterR10,
    X64_RegisterR11,
    X64_RegisterR12,
    X64_RegisterR13,
    X64_RegisterR14,
    X64_RegisterR15,
    X64_RegisterRip,
    X64_RegisterRFlags,

    //
    // Floating Point Registers
    //

    X64_RegisterXmm0,
    X64_RegisterXmm1,
    X64_RegisterXmm2,
    X64_RegisterXmm3,
    X64_RegisterXmm4,
    X64_RegisterXmm5,
    X64_RegisterXmm6,
    X64_RegisterXmm7,
    X64_RegisterXmm8,
    X64_RegisterXmm9,
    X64_RegisterXmm10,
    X64_RegisterXmm11,
    X64_RegisterXmm12,
    X64_RegisterXmm13,
    X64_RegisterXmm14,
    X64_RegisterXmm15,
    X64_RegisterFpMmx0,
    X64_RegisterFpMmx1,
    X64_RegisterFpMmx2,
    X64_RegisterFpMmx3,
    X64_RegisterFpMmx4,
    X64_RegisterFpMmx5,
    X64_RegisterFpMmx6,
    X64_RegisterFpMmx7,
    X64_RegisterFpControlStatus,
    X64_RegisterXmmControlStatus,

    //
    // Control Registers
    //

    X64_RegisterCr0,
    X64_RegisterCr2,
    X64_RegisterCr3,
    X64_RegisterCr4,
    X64_RegisterCr8,
    X64_RegisterEfer,

    //
    // Debug Registers
    //

    X64_RegisterDr0,
    X64_RegisterDr1,
    X64_RegisterDr2,
    X64_RegisterDr3,
    X64_RegisterDr6,
    X64_RegisterDr7,

    //
    // Segment Registers
    //

    X64_RegisterEs,
    X64_RegisterCs,
    X64_RegisterSs,
    X64_RegisterDs,
    X64_RegisterFs,
    X64_RegisterGs,
    X64_RegisterLdtr,
    X64_RegisterTr,

    //
    // Table Registers
    //

    X64_RegisterIdtr,
    X64_RegisterGdtr,

    X64_RegisterMax, // Sentinel value.

    //
    // ARM64 registers:
    //

    ARM64_RegisterX0,
    ARM64_RegisterX1,
    ARM64_RegisterX2,
    ARM64_RegisterX3,
    ARM64_RegisterX4,
    ARM64_RegisterX5,
    ARM64_RegisterX6,
    ARM64_RegisterX7,
    ARM64_RegisterX8,
    ARM64_RegisterX9,
    ARM64_RegisterX10,
    ARM64_RegisterX11,
    ARM64_RegisterX12,
    ARM64_RegisterX13,
    ARM64_RegisterX14,
    ARM64_RegisterX15,
    ARM64_RegisterX16,
    ARM64_RegisterX17,
    ARM64_RegisterX18,
    ARM64_RegisterX19,
    ARM64_RegisterX20,
    ARM64_RegisterX21,
    ARM64_RegisterX22,
    ARM64_RegisterX23,
    ARM64_RegisterX24,
    ARM64_RegisterX25,
    ARM64_RegisterX26,
    ARM64_RegisterX27,
    ARM64_RegisterX28,
    ARM64_RegisterXFp,
    ARM64_RegisterXLr,
    ARM64_RegisterPc,
    ARM64_RegisterSpEl0,
    ARM64_RegisterSpEl1,
    ARM64_RegisterCpsr,

    //
    // Floating Point Registers
    //

    ARM64_RegisterQ0,
    ARM64_RegisterQ1,
    ARM64_RegisterQ2,
    ARM64_RegisterQ3,
    ARM64_RegisterQ4,
    ARM64_RegisterQ5,
    ARM64_RegisterQ6,
    ARM64_RegisterQ7,
    ARM64_RegisterQ8,
    ARM64_RegisterQ9,
    ARM64_RegisterQ10,
    ARM64_RegisterQ11,
    ARM64_RegisterQ12,
    ARM64_RegisterQ13,
    ARM64_RegisterQ14,
    ARM64_RegisterQ15,
    ARM64_RegisterQ16,
    ARM64_RegisterQ17,
    ARM64_RegisterQ18,
    ARM64_RegisterQ19,
    ARM64_RegisterQ20,
    ARM64_RegisterQ21,
    ARM64_RegisterQ22,
    ARM64_RegisterQ23,
    ARM64_RegisterQ24,
    ARM64_RegisterQ25,
    ARM64_RegisterQ26,
    ARM64_RegisterQ27,
    ARM64_RegisterQ28,
    ARM64_RegisterQ29,
    ARM64_RegisterQ30,
    ARM64_RegisterQ31,
    ARM64_RegisterFpStatus,
    ARM64_RegisterFpControl,

    //
    // Control registers.
    //

    ARM64_RegisterEsrEl1,
    ARM64_RegisterSpsrEl1,
    ARM64_RegisterFarEl1,
    ARM64_RegisterParEl1,
    ARM64_RegisterElrEl1,
    ARM64_RegisterTtbr0El1,
    ARM64_RegisterTtbr1El1,
    ARM64_RegisterVbarEl1,
    ARM64_RegisterSctlrEl1,
    ARM64_RegisterActlrEl1,
    ARM64_RegisterTcrEl1,
    ARM64_RegisterMairEl1,
    ARM64_RegisterAmairEl1,
    ARM64_RegisterTpidrEl0,
    ARM64_RegisterTpidrroEl0,
    ARM64_RegisterTpidrEl1,
    ARM64_RegisterContextIdrEl1,
    ARM64_RegisterCpacrEl1,
    ARM64_RegisterCsselrEl1,
    ARM64_RegisterCntkctlEl1,
    ARM64_RegisterCntvCvalEl0,
    ARM64_RegisterCntvCtlEl0,

    ARM64_RegisterMax, // Sentinel value

} REGISTER_ID;

#pragma warning (push)
#pragma warning(disable : 4201) // nameless struct/union

/// This struct represents a register value.
typedef union VIRTUAL_PROCESSOR_REGISTER
{
    UINT64 Reg64;
    UINT32 Reg32;
    UINT16 Reg16;
    UINT8 Reg8;

    // 128 bit value.
    struct
    {
        UINT64 Low64;
        UINT64 High64;
    } Reg128;

    union
    {
        // X64 segment register including hidden state.
        struct
        {
            UINT64 Base;
            UINT32 Limit;
            UINT16 Selector;
            union
            {
                UINT16 Attributes;
                struct
                {
                    UINT16 SegmentType:4;
                    UINT16 NonSystemSegment:1;
                    UINT16 DescriptorPrivilegeLevel:2;
                    UINT16 Present:1;
                    UINT16 Reserved:4;
                    UINT16 Available:1;
                    UINT16 Long:1;
                    UINT16 Default:1;
                    UINT16 Granularity:1;
                };
            };
        } Segment;

        // X64 table register.
        struct
        {
            UINT16 Limit;
            UINT64 Base;
        } Table;

        // X64 FP control status register.
        struct
        {
            UINT16 FpControl;
            UINT16 FpStatus;
            UINT8  FpTag;
            UINT8  Reserved;
            UINT16 LastFpOp;
            union
            {
                // Long Mode
                UINT64 LastFpRip;

                // 32 Bit Mode
                struct
                {
                    UINT32 LastFpEip;
                    UINT16 LastFpCs;
                };
            };
        } FpControlStatus;

        // X64 XMM control status register.
        struct
        {
            union
            {
                // Long Mode
                UINT64 LastFpRdp;

                // 32 Bit Mode
                struct
                {
                    UINT32 LastFpDp;
                    UINT16 LastFpDs;
                };
            };

            UINT32 XmmStatusControl;
            UINT32 XmmStatusControlMask;
        } XmmControlStatus;
    } X64;

#ifdef __cplusplus

    VIRTUAL_PROCESSOR_REGISTER()
    {
        Reg128.Low64 = 0;
        Reg128.High64 = 0;
    }

    VIRTUAL_PROCESSOR_REGISTER(UINT64 Value)
    {
        Reg128.Low64 = Value;
        Reg128.High64 = 0;
    }

#endif // __cplusplus

} VIRTUAL_PROCESSOR_REGISTER;

#pragma warning(pop)

/// Struct that defines a DOS Image.
typedef struct _DOS_IMAGE_INFO
{
    LPCSTR PdbName;
    GUEST_VIRTUAL_ADDRESS ImageBaseAddress;
    DWORD ImageSize;
    UINT32 Timestamp;
} DOS_IMAGE_INFO, *PDOS_IMAGE_INFO;


/// Function type for the guest symbol provider debug info callback.
///
/// \param InfoMessage  Supplies a debug info message.
///
typedef void (CALLBACK *GUEST_SYMBOLS_PROVIDER_DEBUG_INFO_CALLBACK)(_In_ LPCSTR InfoMessage);


/// Function type for the dos image scan callback.
///
/// \param ImageInfo  Supplies a found valid DOS image in the virtual address space.
///                   Do not assume the PdbName string's pointer will be valid after the callback has returned.
///
/// \return FALSE if the caller wants the scan to continue.
///         TRUE if the caller has found an image it was looking for and wishes to stop the scan.
///
typedef BOOL (CALLBACK *FOUND_IMAGE_CALLBACK)(_In_ PVOID Context, _In_ PDOS_IMAGE_INFO ImageInfo);


typedef struct _MODULE_INFO
{
    LPCSTR ProcessImageName;
    DOS_IMAGE_INFO Image;
} MODULE_INFO, *PMODULE_INFO;

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

#endif // _VMSAVEDSTATEDUMPDEFS_H_

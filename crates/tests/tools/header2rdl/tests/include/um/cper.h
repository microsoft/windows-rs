/*++

Copyright (c) 2007 Microsoft Corporation

Module Name:

    cper.h

Abstract:

    This header file defines the structures used to represent the Common
    Platform Error Record as defined in Appendix N of the Unified Extensible
    Firmware Interface (UEFI) specification (revision 2.6).

    This specification as well as any approved errata may be obtained from
    http://www.uefi.org.

    This header file also includes Microsoft specific extensions to the Common
    Platform Error Record as allowed by Appendix N, Section 2.3 of the Unified
    Extensible Firmware Interface specification (Non-standard Section Body).

--*/

#ifndef _CPER_H_
#define _CPER_H_

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#include <cperguid.h>

#pragma warning(push)
#pragma warning(disable:4201)
#pragma warning(disable:4214)

#if defined(MCI_STATUS)
#undef MCI_STATUS
#endif

//
// The general format of the common platform error record is illustrated below.
// A record consists of a header; followed by one or more section descriptors;
// and for each descriptor, an associated section which may contain either error
// or informational data.
//
// The record may include extra buffer space to allow for the dynamic addition
// of error sections descriptors and bodies, as well as for dynamically
// increasing the size of existing sections.
//
// +---------------------------------------------+
// | Record Header                               |
// |   SectionCount == N                         |
// +---------------------------------------------+
// | Section Descriptor 1                        |
// |   Offset, size                              | ---+
// +---------------------------------------------+    |
// | Section Descriptor 2                        |    |
// |   Offset, size                              | ---+---+
// +---------------------------------------------+    |   |
// |                                             |    |   |
// | ....                                        |    |   |
// |                                             |    |   |
// +---------------------------------------------+    |   |
// | Section Descriptor N                        | ---+---+---+
// |   Offset, size                              |    |   |   |
// +---------------------------------------------+    |   |   |
// |                     Buffer space for adding |    |   |   |
// |                   more section descriptors. |    |   |   |
// +---------------------------------------------|    |   |   |
// | Section 1                                   | <--+   |   |
// |                                             |        |   |
// +---------------------------------------------+        |   |
// | Section 2                                   | <------+   |
// |                                             |            |
// +---------------------------------------------+            |
// |                                             |            |
// |                                             |            |
// | ....                                        |            |
// |                                             |            |
// |                                             |            |
// +---------------------------------------------+            |
// | Section N                                   | <----------+
// |                                             |
// +---------------------------------------------+
// |                                             |
// |                                             |
// |                                             |
// |                     Buffer space for adding |
// |                        more section bodies. |
// |                                             |
// |                                             |
// |                                             |
// +---------------------------------------------+
//

// -------------------------------------------- Specification validation macros

//
// The following macro implements a compile-time check for the offset and length
// of the specified structure member. This can be used to validate the defined
// structures against the specification.
//

#define CPER_FIELD_CHECK(type, field, offset, length) \
    C_ASSERT(((FIELD_OFFSET(type, field) == (offset)) && \
              (RTL_FIELD_SIZE(type, field) == (length))))

#include <pshpack1.h>

//---------------------------------- Downlevel GUID variable name compatibility

#if WHEA_DOWNLEVEL_TYPE_NAMES

#define PROCESSOR_GENERIC_SECTION_GUID          PROCESSOR_GENERIC_ERROR_SECTION_GUID
#define X86_PROCESSOR_SPECIFIC_SECTION_GUID     XPF_PROCESSOR_ERROR_SECTION_GUID
#define IPF_PROCESSOR_SPECIFIC_SECTION_GUID     IPF_PROCESSOR_ERROR_SECTION_GUID
#define ARM_PROCESSOR_SPECIFIC_SECTION_GUID     ARM_PROCESSOR_ERROR_SECTION_GUID
#define PLATFORM_MEMORY_SECTION_GUID            MEMORY_ERROR_SECTION_GUID
#define PCIEXPRESS_SECTION_GUID                 PCIEXPRESS_ERROR_SECTION_GUID
#define PCIX_BUS_SECTION_GUID                   PCIXBUS_ERROR_SECTION_GUID
#define PCIX_COMPONENT_SECTION_GUID             PCIXDEVICE_ERROR_SECTION_GUID
#define IPF_SAL_RECORD_REFERENCE_SECTION_GUID   FIRMWARE_ERROR_RECORD_REFERENCE_GUID

#endif

//------------------------------------------ Common Platform Error Record types

//
// These types are used in several of the common platform error record
// structures.
//

typedef union _WHEA_REVISION {
    struct {
        UCHAR MinorRevision;
        UCHAR MajorRevision;
    } DUMMYSTRUCTNAME;
    USHORT AsUSHORT;
} WHEA_REVISION, *PWHEA_REVISION;

typedef enum _WHEA_ERROR_SEVERITY {
    WheaErrSevRecoverable   = 0,
    WheaErrSevFatal         = 1,
    WheaErrSevCorrected     = 2,
    WheaErrSevInformational = 3
} WHEA_ERROR_SEVERITY, *PWHEA_ERROR_SEVERITY;

typedef union _WHEA_TIMESTAMP {
    struct {
        ULONGLONG Seconds:8;
        ULONGLONG Minutes:8;
        ULONGLONG Hours:8;
        ULONGLONG Precise:1;
        ULONGLONG Reserved:7;
        ULONGLONG Day:8;
        ULONGLONG Month:8;
        ULONGLONG Year:8;
        ULONGLONG Century:8;
    } DUMMYSTRUCTNAME;
    LARGE_INTEGER AsLARGE_INTEGER;
} WHEA_TIMESTAMP, *PWHEA_TIMESTAMP;

typedef union _WHEA_PERSISTENCE_INFO {
    struct {
        ULONGLONG Signature:16;
        ULONGLONG Length:24;
        ULONGLONG Identifier:16;
        ULONGLONG Attributes:2;
        ULONGLONG DoNotLog:1;
        ULONGLONG Reserved:5;
    } DUMMYSTRUCTNAME;
    ULONGLONG AsULONGLONG;
} WHEA_PERSISTENCE_INFO, *PWHEA_PERSISTENCE_INFO;

#define ERRTYP_INTERNAL                 0x01 // 1
#define ERRTYP_BUS                      0x10 // 16
#define ERRTYP_MEM                      0x04 // 4
#define ERRTYP_TLB                      0x05 // 5
#define ERRTYP_CACHE                    0x06 // 6
#define ERRTYP_FUNCTION                 0x07 // 7
#define ERRTYP_SELFTEST                 0x08 // 8
#define ERRTYP_FLOW                     0x09 // 9
#define ERRTYP_MAP                      0x11 // 17
#define ERRTYP_IMPROPER                 0x12 // 18
#define ERRTYP_UNIMPL                   0x13 // 19
#define ERRTYP_LOSSOFLOCKSTEP           0x14 // 20
#define ERRTYP_RESPONSE                 0x15 // 21
#define ERRTYP_PARITY                   0x16 // 22
#define ERRTYP_PROTOCOL                 0x17 // 23
#define ERRTYP_PATHERROR                0x18 // 24
#define ERRTYP_TIMEOUT                  0x19 // 25
#define ERRTYP_POISONED                 0x1A // 26

typedef union _WHEA_ERROR_STATUS {
    ULONGLONG ErrorStatus;
    struct {
        ULONGLONG Reserved1:8;
        ULONGLONG ErrorType:8;
        ULONGLONG Address:1;
        ULONGLONG Control:1;
        ULONGLONG Data:1;
        ULONGLONG Responder:1;
        ULONGLONG Requester:1;
        ULONGLONG FirstError:1;
        ULONGLONG Overflow:1;
        ULONGLONG Reserved2:41;
    } DUMMYSTRUCTNAME;
} WHEA_ERROR_STATUS, *PWHEA_ERROR_STATUS;

//---------------------------------------------------- WHEA_ERROR_RECORD_HEADER

typedef union _WHEA_ERROR_RECORD_HEADER_VALIDBITS {
    struct {
        ULONG PlatformId:1;
        ULONG Timestamp:1;
        ULONG PartitionId:1;
        ULONG Reserved:29;
    } DUMMYSTRUCTNAME;
    ULONG AsULONG;
} WHEA_ERROR_RECORD_HEADER_VALIDBITS, *PWHEA_ERROR_RECORD_HEADER_VALIDBITS;

#define WHEA_ERROR_RECORD_VALID_PLATFORMID           0x00000001
#define WHEA_ERROR_RECORD_VALID_TIMESTAMP            0x00000002
#define WHEA_ERROR_RECORD_VALID_PARTITIONID          0x00000004

typedef union _WHEA_ERROR_RECORD_HEADER_FLAGS {
    struct {
        ULONG Recovered:1;
        ULONG PreviousError:1;
        ULONG Simulated:1;
        ULONG DeviceDriver:1;
        ULONG CriticalEvent:1;
        ULONG PersistPfn:1;
        ULONG SectionsTruncated:1;
        ULONG RecoveryInProgress:1;
        ULONG Throttle:1;
        ULONG Reserved:23;
    } DUMMYSTRUCTNAME;
    ULONG AsULONG;
} WHEA_ERROR_RECORD_HEADER_FLAGS, *PWHEA_ERROR_RECORD_HEADER_FLAGS;

#define WHEA_ERROR_RECORD_FLAGS_RECOVERED            0x00000001
#define WHEA_ERROR_RECORD_FLAGS_PREVIOUSERROR        0x00000002
#define WHEA_ERROR_RECORD_FLAGS_SIMULATED            0x00000004
#define WHEA_ERROR_RECORD_FLAGS_DEVICE_DRIVER        0x00000008

typedef struct _WHEA_ERROR_RECORD_HEADER {
    ULONG Signature;
    WHEA_REVISION Revision;
    ULONG SignatureEnd;
    USHORT SectionCount;
    WHEA_ERROR_SEVERITY Severity;
    WHEA_ERROR_RECORD_HEADER_VALIDBITS ValidBits;
    _Field_range_(>=, (sizeof(WHEA_ERROR_RECORD_HEADER)
                       + (SectionCount
                          * sizeof(WHEA_ERROR_RECORD_SECTION_DESCRIPTOR))))
        ULONG Length;
    WHEA_TIMESTAMP Timestamp;
    GUID PlatformId;
    GUID PartitionId;
    GUID CreatorId;
    GUID NotifyType;
    ULONGLONG RecordId;
    WHEA_ERROR_RECORD_HEADER_FLAGS Flags;
    WHEA_PERSISTENCE_INFO PersistenceInfo;
    union {
        struct {
            ULONG OsBuildNumber; // Pupulated by AzPshedPi, not in vanilla windows
            UCHAR Reserved2[8];
        };

        UCHAR Reserved[12];
    };
} WHEA_ERROR_RECORD_HEADER, *PWHEA_ERROR_RECORD_HEADER;

//
// Distinguished values used in the common platform error record header
// signature.
//

#define WHEA_ERROR_RECORD_SIGNATURE         'REPC'
#define WHEA_ERROR_RECORD_REVISION          0x0210
#define WHEA_ERROR_RECORD_SIGNATURE_END     0xFFFFFFFF

//
// Validate the error record header structure against the definitions in the
// UEFI specification.
//

CPER_FIELD_CHECK(WHEA_ERROR_RECORD_HEADER, Signature,         0,  4);
CPER_FIELD_CHECK(WHEA_ERROR_RECORD_HEADER, Revision,          4,  2);
CPER_FIELD_CHECK(WHEA_ERROR_RECORD_HEADER, SignatureEnd,      6,  4);
CPER_FIELD_CHECK(WHEA_ERROR_RECORD_HEADER, SectionCount,     10,  2);
CPER_FIELD_CHECK(WHEA_ERROR_RECORD_HEADER, Severity,         12,  4);
CPER_FIELD_CHECK(WHEA_ERROR_RECORD_HEADER, ValidBits,        16,  4);
CPER_FIELD_CHECK(WHEA_ERROR_RECORD_HEADER, Length,           20,  4);
CPER_FIELD_CHECK(WHEA_ERROR_RECORD_HEADER, Timestamp,        24,  8);
CPER_FIELD_CHECK(WHEA_ERROR_RECORD_HEADER, PlatformId,       32, 16);
CPER_FIELD_CHECK(WHEA_ERROR_RECORD_HEADER, PartitionId,      48, 16);
CPER_FIELD_CHECK(WHEA_ERROR_RECORD_HEADER, CreatorId,        64, 16);
CPER_FIELD_CHECK(WHEA_ERROR_RECORD_HEADER, NotifyType,       80, 16);
CPER_FIELD_CHECK(WHEA_ERROR_RECORD_HEADER, RecordId,         96,  8);
CPER_FIELD_CHECK(WHEA_ERROR_RECORD_HEADER, Flags,           104,  4);
CPER_FIELD_CHECK(WHEA_ERROR_RECORD_HEADER, PersistenceInfo, 108,  8);
CPER_FIELD_CHECK(WHEA_ERROR_RECORD_HEADER, Reserved,        116, 12);

//---------------------------------------- WHEA_ERROR_RECORD_SECTION_DESCRIPTOR

typedef union _WHEA_ERROR_RECORD_SECTION_DESCRIPTOR_FLAGS {
    struct {
        ULONG Primary:1;
        ULONG ContainmentWarning:1;
        ULONG Reset:1;
        ULONG ThresholdExceeded:1;
        ULONG ResourceNotAvailable:1;
        ULONG LatentError:1;
        ULONG Propagated:1;
        ULONG FruTextByPlugin:1;
        ULONG Reserved:24;
    } DUMMYSTRUCTNAME;
    ULONG AsULONG;
} WHEA_ERROR_RECORD_SECTION_DESCRIPTOR_FLAGS,
    *PWHEA_ERROR_RECORD_SECTION_DESCRIPTOR_FLAGS;

#define WHEA_SECTION_DESCRIPTOR_FLAGS_PRIMARY            0x00000001
#define WHEA_SECTION_DESCRIPTOR_FLAGS_CONTAINMENTWRN     0x00000002
#define WHEA_SECTION_DESCRIPTOR_FLAGS_RESET              0x00000004
#define WHEA_SECTION_DESCRIPTOR_FLAGS_THRESHOLDEXCEEDED  0x00000008
#define WHEA_SECTION_DESCRIPTOR_FLAGS_RESOURCENA         0x00000010
#define WHEA_SECTION_DESCRIPTOR_FLAGS_LATENTERROR        0x00000020
#define WHEA_SECTION_DESCRIPTOR_FLAGS_PROPAGATED         0x00000040
#define WHEA_SECTION_DESCRIPTOR_FLAGS_FRU_TEXT_BY_PLUGIN 0x00000080

typedef union _WHEA_ERROR_RECORD_SECTION_DESCRIPTOR_VALIDBITS {
    struct {
        UCHAR FRUId:1;
        UCHAR FRUText:1;
        UCHAR Reserved:6;
    } DUMMYSTRUCTNAME;
    UCHAR AsUCHAR;
} WHEA_ERROR_RECORD_SECTION_DESCRIPTOR_VALIDBITS,
    *PWHEA_ERROR_RECORD_SECTION_DESCRIPTOR_VALIDBITS;

typedef struct _WHEA_ERROR_RECORD_SECTION_DESCRIPTOR {
    ULONG SectionOffset;
    ULONG SectionLength;
    WHEA_REVISION Revision;
    WHEA_ERROR_RECORD_SECTION_DESCRIPTOR_VALIDBITS ValidBits;
    UCHAR Reserved;
    WHEA_ERROR_RECORD_SECTION_DESCRIPTOR_FLAGS Flags;
    GUID SectionType;
    GUID FRUId;
    WHEA_ERROR_SEVERITY SectionSeverity;
    CCHAR FRUText[20];
} WHEA_ERROR_RECORD_SECTION_DESCRIPTOR, *PWHEA_ERROR_RECORD_SECTION_DESCRIPTOR;

#define WHEA_ERROR_RECORD_SECTION_DESCRIPTOR_REVISION   0x0300

#if WHEA_DOWNLEVEL_TYPE_NAMES

#define WHEA_SECTION_DESCRIPTOR_REVISION \
    WHEA_ERROR_RECORD_SECTION_DESCRIPTOR_REVISION

#endif

//
// Validate the error record section descriptor structure against the
// definitions in the UEFI specification.
//

CPER_FIELD_CHECK(WHEA_ERROR_RECORD_SECTION_DESCRIPTOR, SectionOffset,    0,  4);
CPER_FIELD_CHECK(WHEA_ERROR_RECORD_SECTION_DESCRIPTOR, SectionLength,    4,  4);
CPER_FIELD_CHECK(WHEA_ERROR_RECORD_SECTION_DESCRIPTOR, Revision,         8,  2);
CPER_FIELD_CHECK(WHEA_ERROR_RECORD_SECTION_DESCRIPTOR, ValidBits,       10,  1);
CPER_FIELD_CHECK(WHEA_ERROR_RECORD_SECTION_DESCRIPTOR, Reserved,        11,  1);
CPER_FIELD_CHECK(WHEA_ERROR_RECORD_SECTION_DESCRIPTOR, Flags,           12,  4);
CPER_FIELD_CHECK(WHEA_ERROR_RECORD_SECTION_DESCRIPTOR, SectionType,     16, 16);
CPER_FIELD_CHECK(WHEA_ERROR_RECORD_SECTION_DESCRIPTOR, FRUId,           32, 16);
CPER_FIELD_CHECK(WHEA_ERROR_RECORD_SECTION_DESCRIPTOR, SectionSeverity, 48,  4);
CPER_FIELD_CHECK(WHEA_ERROR_RECORD_SECTION_DESCRIPTOR, FRUText,         52, 20);

//----------------------------------------------------------- WHEA_ERROR_RECORD

typedef _Struct_size_bytes_(Header.Length) struct _WHEA_ERROR_RECORD {
    WHEA_ERROR_RECORD_HEADER Header;
    _Field_size_(Header.SectionCount)
        WHEA_ERROR_RECORD_SECTION_DESCRIPTOR SectionDescriptor[ANYSIZE_ARRAY];
} WHEA_ERROR_RECORD, *PWHEA_ERROR_RECORD;

//
// Validate the error record structure against the definitions in the UEFI
// specification.
//

CPER_FIELD_CHECK(WHEA_ERROR_RECORD, Header,              0,  128);
CPER_FIELD_CHECK(WHEA_ERROR_RECORD, SectionDescriptor, 128,   72);

//---------------------------------------- WHEA_PROCESSOR_GENERIC_ERROR_SECTION

#define GENPROC_PROCTYPE_XPF                 0
#define GENPROC_PROCTYPE_IPF                 1
#define GENPROC_PROCTYPE_ARM                 2

#define GENPROC_PROCISA_X86                  0
#define GENPROC_PROCISA_IPF                  1
#define GENPROC_PROCISA_X64                  2
#define GENPROC_PROCISA_ARM32                4
#define GENPROC_PROCISA_ARM64                8

#define GENPROC_PROCERRTYPE_UNKNOWN          0
#define GENPROC_PROCERRTYPE_CACHE            1
#define GENPROC_PROCERRTYPE_TLB              2
#define GENPROC_PROCERRTYPE_BUS              4
#define GENPROC_PROCERRTYPE_MAE              8

#define GENPROC_OP_GENERIC                   0
#define GENPROC_OP_DATAREAD                  1
#define GENPROC_OP_DATAWRITE                 2
#define GENPROC_OP_INSTRUCTIONEXE            3

#define GENPROC_FLAGS_RESTARTABLE            0x01
#define GENPROC_FLAGS_PRECISEIP              0x02
#define GENPROC_FLAGS_OVERFLOW               0x04
#define GENPROC_FLAGS_CORRECTED              0x08

typedef union _WHEA_PROCESSOR_FAMILY_INFO {
    struct {
        ULONG Stepping:4;
        ULONG Model:4;
        ULONG Family:4;
        ULONG ProcessorType:2;
        ULONG Reserved1:2;
        ULONG ExtendedModel:4;
        ULONG ExtendedFamily:8;
        ULONG Reserved2:4;
        ULONG NativeModelId;
    } DUMMYSTRUCTNAME;
    ULONGLONG AsULONGLONG;
} WHEA_PROCESSOR_FAMILY_INFO, *PWHEA_PROCESSOR_FAMILY_INFO;

typedef union _WHEA_PROCESSOR_GENERIC_ERROR_SECTION_VALIDBITS {
    struct {
        ULONGLONG ProcessorType:1;
        ULONGLONG InstructionSet:1;
        ULONGLONG ErrorType:1;
        ULONGLONG Operation:1;
        ULONGLONG Flags:1;
        ULONGLONG Level:1;
        ULONGLONG CPUVersion:1;
        ULONGLONG CPUBrandString:1;
        ULONGLONG ProcessorId:1;
        ULONGLONG TargetAddress:1;
        ULONGLONG RequesterId:1;
        ULONGLONG ResponderId:1;
        ULONGLONG InstructionPointer:1;
        ULONGLONG NativeModelId:1;
        ULONGLONG Reserved:50;
    } DUMMYSTRUCTNAME;
    ULONGLONG ValidBits;
} WHEA_PROCESSOR_GENERIC_ERROR_SECTION_VALIDBITS,
  *PWHEA_PROCESSOR_GENERIC_ERROR_SECTION_VALIDBITS;

typedef struct _WHEA_PROCESSOR_GENERIC_ERROR_SECTION {
    WHEA_PROCESSOR_GENERIC_ERROR_SECTION_VALIDBITS ValidBits;
    UCHAR ProcessorType;
    UCHAR InstructionSet;
    UCHAR ErrorType;
    UCHAR Operation;
    UCHAR Flags;
    UCHAR Level;
    USHORT Reserved;
    ULONGLONG CPUVersion;
    UCHAR CPUBrandString[128];
    ULONGLONG ProcessorId;
    ULONGLONG TargetAddress;
    ULONGLONG RequesterId;
    ULONGLONG ResponderId;
    ULONGLONG InstructionPointer;
} WHEA_PROCESSOR_GENERIC_ERROR_SECTION, *PWHEA_PROCESSOR_GENERIC_ERROR_SECTION;

//
// Define alternate type name for downlevel source compatibility.
//

#if WHEA_DOWNLEVEL_TYPE_NAMES

typedef WHEA_PROCESSOR_GENERIC_ERROR_SECTION_VALIDBITS
    WHEA_GENERIC_PROCESSOR_ERROR_VALIDBITS,
    *PWHEA_GENERIC_PROCESSOR_ERROR_VALIDBITS;

typedef WHEA_PROCESSOR_GENERIC_ERROR_SECTION
    WHEA_GENERIC_PROCESSOR_ERROR, *PWHEA_GENERIC_PROCESSOR_ERROR;

#endif

//
// Validate the processor generic error section structure against the
// definitions in the UEFI  specification.
//

CPER_FIELD_CHECK(WHEA_PROCESSOR_GENERIC_ERROR_SECTION, ValidBits,            0,   8);
CPER_FIELD_CHECK(WHEA_PROCESSOR_GENERIC_ERROR_SECTION, ProcessorType,        8,   1);
CPER_FIELD_CHECK(WHEA_PROCESSOR_GENERIC_ERROR_SECTION, InstructionSet,       9,   1);
CPER_FIELD_CHECK(WHEA_PROCESSOR_GENERIC_ERROR_SECTION, ErrorType,           10,   1);
CPER_FIELD_CHECK(WHEA_PROCESSOR_GENERIC_ERROR_SECTION, Operation,           11,   1);
CPER_FIELD_CHECK(WHEA_PROCESSOR_GENERIC_ERROR_SECTION, Flags,               12,   1);
CPER_FIELD_CHECK(WHEA_PROCESSOR_GENERIC_ERROR_SECTION, Level,               13,   1);
CPER_FIELD_CHECK(WHEA_PROCESSOR_GENERIC_ERROR_SECTION, Reserved,            14,   2);
CPER_FIELD_CHECK(WHEA_PROCESSOR_GENERIC_ERROR_SECTION, CPUVersion,          16,   8);
CPER_FIELD_CHECK(WHEA_PROCESSOR_GENERIC_ERROR_SECTION, CPUBrandString,      24, 128);
CPER_FIELD_CHECK(WHEA_PROCESSOR_GENERIC_ERROR_SECTION, ProcessorId,        152,   8);
CPER_FIELD_CHECK(WHEA_PROCESSOR_GENERIC_ERROR_SECTION, TargetAddress,      160,   8);
CPER_FIELD_CHECK(WHEA_PROCESSOR_GENERIC_ERROR_SECTION, RequesterId,        168,   8);
CPER_FIELD_CHECK(WHEA_PROCESSOR_GENERIC_ERROR_SECTION, ResponderId,        176,   8);
CPER_FIELD_CHECK(WHEA_PROCESSOR_GENERIC_ERROR_SECTION, InstructionPointer, 184,   8);

//-------------------------------------------- WHEA_XPF_PROCESSOR_ERROR_SECTION

//
// x86/x64 cache check structure.
//

#define XPF_CACHE_CHECK_TRANSACTIONTYPE_INSTRUCTION     0
#define XPF_CACHE_CHECK_TRANSACTIONTYPE_DATAACCESS      1
#define XPF_CACHE_CHECK_TRANSACTIONTYPE_GENERIC         2

#define XPF_CACHE_CHECK_OPERATION_GENERIC               0
#define XPF_CACHE_CHECK_OPERATION_GENREAD               1
#define XPF_CACHE_CHECK_OPERATION_GENWRITE              2
#define XPF_CACHE_CHECK_OPERATION_DATAREAD              3
#define XPF_CACHE_CHECK_OPERATION_DATAWRITE             4
#define XPF_CACHE_CHECK_OPERATION_INSTRUCTIONFETCH      5
#define XPF_CACHE_CHECK_OPERATION_PREFETCH              6
#define XPF_CACHE_CHECK_OPERATION_EVICTION              7
#define XPF_CACHE_CHECK_OPERATION_SNOOP                 8

typedef union _WHEA_XPF_CACHE_CHECK {
    struct {
        ULONGLONG TransactionTypeValid:1;
        ULONGLONG OperationValid:1;
        ULONGLONG LevelValid:1;
        ULONGLONG ProcessorContextCorruptValid:1;
        ULONGLONG UncorrectedValid:1;
        ULONGLONG PreciseIPValid:1;
        ULONGLONG RestartableIPValid:1;
        ULONGLONG OverflowValid:1;
        ULONGLONG ReservedValid:8;

        ULONGLONG TransactionType:2;
        ULONGLONG Operation:4;
        ULONGLONG Level:3;
        ULONGLONG ProcessorContextCorrupt:1;
        ULONGLONG Uncorrected:1;
        ULONGLONG PreciseIP:1;
        ULONGLONG RestartableIP:1;
        ULONGLONG Overflow:1;

        ULONGLONG Reserved:34;
    } DUMMYSTRUCTNAME;
    ULONGLONG XpfCacheCheck;
} WHEA_XPF_CACHE_CHECK, *PWHEA_XPF_CACHE_CHECK;

//
// x86/x64 TLB check structure.
//

#define XPF_TLB_CHECK_TRANSACTIONTYPE_INSTRUCTION     0
#define XPF_TLB_CHECK_TRANSACTIONTYPE_DATAACCESS      1
#define XPF_TLB_CHECK_TRANSACTIONTYPE_GENERIC         2

#define XPF_TLB_CHECK_OPERATION_GENERIC               0
#define XPF_TLB_CHECK_OPERATION_GENREAD               1
#define XPF_TLB_CHECK_OPERATION_GENWRITE              2
#define XPF_TLB_CHECK_OPERATION_DATAREAD              3
#define XPF_TLB_CHECK_OPERATION_DATAWRITE             4
#define XPF_TLB_CHECK_OPERATION_INSTRUCTIONFETCH      5
#define XPF_TLB_CHECK_OPERATION_PREFETCH              6

typedef union _WHEA_XPF_TLB_CHECK {
    struct {
        ULONGLONG TransactionTypeValid:1;
        ULONGLONG OperationValid:1;
        ULONGLONG LevelValid:1;
        ULONGLONG ProcessorContextCorruptValid:1;
        ULONGLONG UncorrectedValid:1;
        ULONGLONG PreciseIPValid:1;
        ULONGLONG RestartableIPValid:1;
        ULONGLONG OverflowValid:1;
        ULONGLONG ReservedValid:8;

        ULONGLONG TransactionType:2;
        ULONGLONG Operation:4;
        ULONGLONG Level:3;
        ULONGLONG ProcessorContextCorrupt:1;
        ULONGLONG Uncorrected:1;
        ULONGLONG PreciseIP:1;
        ULONGLONG RestartableIP:1;
        ULONGLONG Overflow:1;
        ULONGLONG Reserved:34;
    } DUMMYSTRUCTNAME;
    ULONGLONG XpfTLBCheck;
} WHEA_XPF_TLB_CHECK, *PWHEA_XPF_TLB_CHECK;

//
// x86/x64 bus check structure.
//

#define XPF_BUS_CHECK_TRANSACTIONTYPE_INSTRUCTION     0
#define XPF_BUS_CHECK_TRANSACTIONTYPE_DATAACCESS      1
#define XPF_BUS_CHECK_TRANSACTIONTYPE_GENERIC         2

#define XPF_BUS_CHECK_OPERATION_GENERIC               0
#define XPF_BUS_CHECK_OPERATION_GENREAD               1
#define XPF_BUS_CHECK_OPERATION_GENWRITE              2
#define XPF_BUS_CHECK_OPERATION_DATAREAD              3
#define XPF_BUS_CHECK_OPERATION_DATAWRITE             4
#define XPF_BUS_CHECK_OPERATION_INSTRUCTIONFETCH      5
#define XPF_BUS_CHECK_OPERATION_PREFETCH              6

#define XPF_BUS_CHECK_PARTICIPATION_PROCORIGINATED    0
#define XPF_BUS_CHECK_PARTICIPATION_PROCRESPONDED     1
#define XPF_BUS_CHECK_PARTICIPATION_PROCOBSERVED      2
#define XPF_BUS_CHECK_PARTICIPATION_GENERIC           3

#define XPF_BUS_CHECK_ADDRESS_MEMORY                  0
#define XPF_BUS_CHECK_ADDRESS_RESERVED                1
#define XPF_BUS_CHECK_ADDRESS_IO                      2
#define XPF_BUS_CHECK_ADDRESS_OTHER                   3

typedef union _WHEA_XPF_BUS_CHECK {
    struct {
        ULONGLONG TransactionTypeValid:1;
        ULONGLONG OperationValid:1;
        ULONGLONG LevelValid:1;
        ULONGLONG ProcessorContextCorruptValid:1;
        ULONGLONG UncorrectedValid:1;
        ULONGLONG PreciseIPValid:1;
        ULONGLONG RestartableIPValid:1;
        ULONGLONG OverflowValid:1;
        ULONGLONG ParticipationValid:1;
        ULONGLONG TimeoutValid:1;
        ULONGLONG AddressSpaceValid:1;
        ULONGLONG ReservedValid:5;

        ULONGLONG TransactionType:2;
        ULONGLONG Operation:4;
        ULONGLONG Level:3;
        ULONGLONG ProcessorContextCorrupt:1;
        ULONGLONG Uncorrected:1;
        ULONGLONG PreciseIP:1;
        ULONGLONG RestartableIP:1;
        ULONGLONG Overflow:1;
        ULONGLONG Participation:2;
        ULONGLONG Timeout:1;
        ULONGLONG AddressSpace:2;
        ULONGLONG Reserved:29;
    } DUMMYSTRUCTNAME;
    ULONGLONG XpfBusCheck;
} WHEA_XPF_BUS_CHECK, *PWHEA_XPF_BUS_CHECK;

//
// x86/x64 micro-architecture specific check structure.
//

#define XPF_MS_CHECK_ERRORTYPE_NOERROR               0
#define XPF_MS_CHECK_ERRORTYPE_UNCLASSIFIED          1
#define XPF_MS_CHECK_ERRORTYPE_MCROMPARITY           2
#define XPF_MS_CHECK_ERRORTYPE_EXTERNAL              3
#define XPF_MS_CHECK_ERRORTYPE_FRC                   4
#define XPF_MS_CHECK_ERRORTYPE_INTERNALUNCLASSIFIED  5

typedef union _WHEA_XPF_MS_CHECK {
    struct {
        ULONGLONG ErrorTypeValid:1;
        ULONGLONG ProcessorContextCorruptValid:1;
        ULONGLONG UncorrectedValid:1;
        ULONGLONG PreciseIPValid:1;
        ULONGLONG RestartableIPValid:1;
        ULONGLONG OverflowValid:1;
        ULONGLONG ReservedValue:10;

        ULONGLONG ErrorType:3;
        ULONGLONG ProcessorContextCorrupt:1;
        ULONGLONG Uncorrected:1;
        ULONGLONG PreciseIP:1;
        ULONGLONG RestartableIP:1;
        ULONGLONG Overflow:1;
        ULONGLONG Reserved:40;
    } DUMMYSTRUCTNAME;
    ULONGLONG XpfMsCheck;
} WHEA_XPF_MS_CHECK, *PWHEA_XPF_MS_CHECK;

//
// x86/x64 Processor Error Information Structure.
//

typedef union _WHEA_XPF_PROCINFO_VALIDBITS {
    struct {
        ULONGLONG CheckInfo:1;
        ULONGLONG TargetId:1;
        ULONGLONG RequesterId:1;
        ULONGLONG ResponderId:1;
        ULONGLONG InstructionPointer:1;
        ULONGLONG Reserved:59;
    } DUMMYSTRUCTNAME;
    ULONGLONG ValidBits;
} WHEA_XPF_PROCINFO_VALIDBITS, *PWHEA_XPF_PROCINFO_VALIDBITS;

typedef struct _WHEA_XPF_PROCINFO {
    GUID CheckInfoId;
    WHEA_XPF_PROCINFO_VALIDBITS ValidBits;
    union {
        WHEA_XPF_CACHE_CHECK CacheCheck;
        WHEA_XPF_TLB_CHECK TlbCheck;
        WHEA_XPF_BUS_CHECK BusCheck;
        WHEA_XPF_MS_CHECK MsCheck;
        ULONGLONG AsULONGLONG;
    } CheckInfo;
    ULONGLONG TargetId;
    ULONGLONG RequesterId;
    ULONGLONG ResponderId;
    ULONGLONG InstructionPointer;
} WHEA_XPF_PROCINFO, *PWHEA_XPF_PROCINFO;

//
// x86/x64 Processor Context Information Structure.
//

typedef struct _WHEA_X86_REGISTER_STATE {
    ULONG Eax;
    ULONG Ebx;
    ULONG Ecx;
    ULONG Edx;
    ULONG Esi;
    ULONG Edi;
    ULONG Ebp;
    ULONG Esp;
    USHORT Cs;
    USHORT Ds;
    USHORT Ss;
    USHORT Es;
    USHORT Fs;
    USHORT Gs;
    ULONG Eflags;
    ULONG Eip;
    ULONG Cr0;
    ULONG Cr1;
    ULONG Cr2;
    ULONG Cr3;
    ULONG Cr4;
    ULONGLONG Gdtr;
    ULONGLONG Idtr;
    USHORT Ldtr;
    USHORT Tr;
} WHEA_X86_REGISTER_STATE, *PWHEA_X86_REGISTER_STATE;

typedef struct DECLSPEC_ALIGN(16) _WHEA128A {
    ULONGLONG Low;
    LONGLONG High;
} WHEA128A, *PWHEA128A;

#if defined(_MSC_VER)
#if (_MSC_VER >= 1200)
#pragma warning(push)
#pragma warning(disable:4324) // structure padded due to __declspec(align())
#endif
#endif

typedef struct _WHEA_X64_REGISTER_STATE {
    ULONGLONG Rax;
    ULONGLONG Rbx;
    ULONGLONG Rcx;
    ULONGLONG Rdx;
    ULONGLONG Rsi;
    ULONGLONG Rdi;
    ULONGLONG Rbp;
    ULONGLONG Rsp;
    ULONGLONG R8;
    ULONGLONG R9;
    ULONGLONG R10;
    ULONGLONG R11;
    ULONGLONG R12;
    ULONGLONG R13;
    ULONGLONG R14;
    ULONGLONG R15;
    USHORT Cs;
    USHORT Ds;
    USHORT Ss;
    USHORT Es;
    USHORT Fs;
    USHORT Gs;
    ULONG Reserved;
    ULONGLONG Rflags;
    ULONGLONG Eip;
    ULONGLONG Cr0;
    ULONGLONG Cr1;
    ULONGLONG Cr2;
    ULONGLONG Cr3;
    ULONGLONG Cr4;
    ULONGLONG Cr8;
    WHEA128A Gdtr;
    WHEA128A Idtr;
    USHORT Ldtr;
    USHORT Tr;
} WHEA_X64_REGISTER_STATE, *PWHEA_X64_REGISTER_STATE;

#if defined(_MSC_VER)
#if (_MSC_VER >= 1200)
#pragma warning(pop)
#endif
#endif

#define XPF_CONTEXT_INFO_UNCLASSIFIEDDATA       0
#define XPF_CONTEXT_INFO_MSRREGISTERS           1
#define XPF_CONTEXT_INFO_32BITCONTEXT           2
#define XPF_CONTEXT_INFO_64BITCONTEXT           3
#define XPF_CONTEXT_INFO_FXSAVE                 4
#define XPF_CONTEXT_INFO_32BITDEBUGREGS         5
#define XPF_CONTEXT_INFO_64BITDEBUGREGS         6
#define XPF_CONTEXT_INFO_MMREGISTERS            7

typedef struct _WHEA_XPF_CONTEXT_INFO {
    USHORT RegisterContextType;
    USHORT RegisterDataSize;
    ULONG MSRAddress;
    ULONGLONG MmRegisterAddress;

    //
    // UCHAR RegisterData[ANYSIZE_ARRAY];
    //

} WHEA_XPF_CONTEXT_INFO, *PWHEA_XPF_CONTEXT_INFO;

//
// x86/x64 Processor Error Section
//

typedef union _WHEA_XPF_PROCESSOR_ERROR_SECTION_VALIDBITS {
    struct {
        ULONGLONG LocalAPICId:1;
        ULONGLONG CpuId:1;
        ULONGLONG ProcInfoCount:6;
        ULONGLONG ContextInfoCount:6;
        ULONGLONG Reserved:50;
    } DUMMYSTRUCTNAME;
    ULONGLONG ValidBits;
} WHEA_XPF_PROCESSOR_ERROR_SECTION_VALIDBITS,
  *PWHEA_XPF_PROCESSOR_ERROR_SECTION_VALIDBITS;

typedef struct _WHEA_XPF_PROCESSOR_ERROR_SECTION {
    WHEA_XPF_PROCESSOR_ERROR_SECTION_VALIDBITS ValidBits;
    ULONGLONG LocalAPICId;
    UCHAR CpuId[48];

    //
    // WHEA_XPF_PROCINFO ProcInfo[ANYSIZE_ARRAY];
    // WHEA_XPF_CONTEXT_INFO ContextInfo[ANYSIZE_ARRAY];
    //

    UCHAR VariableInfo[ANYSIZE_ARRAY];
} WHEA_XPF_PROCESSOR_ERROR_SECTION, *PWHEA_XPF_PROCESSOR_ERROR_SECTION;

//
// Define alternate type names for downlevel source compatibility.
//

#if WHEA_DOWNLEVEL_TYPE_NAMES

typedef struct WHEA_XPF_PROCESSOR_ERROR_SECTION_VALIDBITS
    WHEA_XPF_PROCESSOR_ERROR_VALIDBITS, *PWHEA_XPF_PROCESSOR_ERROR_VALIDBITS;

typedef struct WHEA_XPF_PROCESSOR_ERROR_SECTION
    WHEA_XPF_PROCESSOR_ERROR, *PWHEA_XPF_PROCESSOR_ERROR;

#endif

//
// Validate the x86/x64 processor error section structures against the
// definitions in the UEFI  specification.
//

CPER_FIELD_CHECK(WHEA_XPF_PROCINFO, CheckInfoId,         0, 16);
CPER_FIELD_CHECK(WHEA_XPF_PROCINFO, ValidBits,          16,  8);
CPER_FIELD_CHECK(WHEA_XPF_PROCINFO, CheckInfo,          24,  8);
CPER_FIELD_CHECK(WHEA_XPF_PROCINFO, TargetId,           32,  8);
CPER_FIELD_CHECK(WHEA_XPF_PROCINFO, RequesterId,        40,  8);
CPER_FIELD_CHECK(WHEA_XPF_PROCINFO, ResponderId,        48,  8);
CPER_FIELD_CHECK(WHEA_XPF_PROCINFO, InstructionPointer, 56,  8);

CPER_FIELD_CHECK(WHEA_X86_REGISTER_STATE, Eax,       0,   4);
CPER_FIELD_CHECK(WHEA_X86_REGISTER_STATE, Ebx,       4,   4);
CPER_FIELD_CHECK(WHEA_X86_REGISTER_STATE, Ecx,       8,   4);
CPER_FIELD_CHECK(WHEA_X86_REGISTER_STATE, Edx,      12,   4);
CPER_FIELD_CHECK(WHEA_X86_REGISTER_STATE, Esi,      16,   4);
CPER_FIELD_CHECK(WHEA_X86_REGISTER_STATE, Edi,      20,   4);
CPER_FIELD_CHECK(WHEA_X86_REGISTER_STATE, Ebp,      24,   4);
CPER_FIELD_CHECK(WHEA_X86_REGISTER_STATE, Esp,      28,   4);
CPER_FIELD_CHECK(WHEA_X86_REGISTER_STATE, Cs,       32,   2);
CPER_FIELD_CHECK(WHEA_X86_REGISTER_STATE, Ds,       34,   2);
CPER_FIELD_CHECK(WHEA_X86_REGISTER_STATE, Ss,       36,   2);
CPER_FIELD_CHECK(WHEA_X86_REGISTER_STATE, Es,       38,   2);
CPER_FIELD_CHECK(WHEA_X86_REGISTER_STATE, Fs,       40,   2);
CPER_FIELD_CHECK(WHEA_X86_REGISTER_STATE, Gs,       42,   2);
CPER_FIELD_CHECK(WHEA_X86_REGISTER_STATE, Eflags,   44,   4);
CPER_FIELD_CHECK(WHEA_X86_REGISTER_STATE, Eip,      48,   4);
CPER_FIELD_CHECK(WHEA_X86_REGISTER_STATE, Cr0,      52,   4);
CPER_FIELD_CHECK(WHEA_X86_REGISTER_STATE, Cr1,      56,   4);
CPER_FIELD_CHECK(WHEA_X86_REGISTER_STATE, Cr2,      60,   4);
CPER_FIELD_CHECK(WHEA_X86_REGISTER_STATE, Cr3,      64,   4);
CPER_FIELD_CHECK(WHEA_X86_REGISTER_STATE, Cr4,      68,   4);
CPER_FIELD_CHECK(WHEA_X86_REGISTER_STATE, Gdtr,     72,   8);
CPER_FIELD_CHECK(WHEA_X86_REGISTER_STATE, Idtr,     80,   8);
CPER_FIELD_CHECK(WHEA_X86_REGISTER_STATE, Ldtr,     88,   2);
CPER_FIELD_CHECK(WHEA_X86_REGISTER_STATE, Tr,       90,   2);

CPER_FIELD_CHECK(WHEA_X64_REGISTER_STATE, Rax,       0,   8);
CPER_FIELD_CHECK(WHEA_X64_REGISTER_STATE, Rbx,       8,   8);
CPER_FIELD_CHECK(WHEA_X64_REGISTER_STATE, Rcx,      16,   8);
CPER_FIELD_CHECK(WHEA_X64_REGISTER_STATE, Rdx,      24,   8);
CPER_FIELD_CHECK(WHEA_X64_REGISTER_STATE, Rsi,      32,   8);
CPER_FIELD_CHECK(WHEA_X64_REGISTER_STATE, Rdi,      40,   8);
CPER_FIELD_CHECK(WHEA_X64_REGISTER_STATE, Rbp,      48,   8);
CPER_FIELD_CHECK(WHEA_X64_REGISTER_STATE, Rsp,      56,   8);
CPER_FIELD_CHECK(WHEA_X64_REGISTER_STATE, R8,       64,   8);
CPER_FIELD_CHECK(WHEA_X64_REGISTER_STATE, R9,       72,   8);
CPER_FIELD_CHECK(WHEA_X64_REGISTER_STATE, R10,      80,   8);
CPER_FIELD_CHECK(WHEA_X64_REGISTER_STATE, R11,      88,   8);
CPER_FIELD_CHECK(WHEA_X64_REGISTER_STATE, R12,      96,   8);
CPER_FIELD_CHECK(WHEA_X64_REGISTER_STATE, R13,      104,  8);
CPER_FIELD_CHECK(WHEA_X64_REGISTER_STATE, R14,      112,  8);
CPER_FIELD_CHECK(WHEA_X64_REGISTER_STATE, R15,      120,  8);
CPER_FIELD_CHECK(WHEA_X64_REGISTER_STATE, Cs,       128,  2);
CPER_FIELD_CHECK(WHEA_X64_REGISTER_STATE, Ds,       130,  2);
CPER_FIELD_CHECK(WHEA_X64_REGISTER_STATE, Ss,       132,  2);
CPER_FIELD_CHECK(WHEA_X64_REGISTER_STATE, Es,       134,  2);
CPER_FIELD_CHECK(WHEA_X64_REGISTER_STATE, Fs,       136,  2);
CPER_FIELD_CHECK(WHEA_X64_REGISTER_STATE, Gs,       138,  2);
CPER_FIELD_CHECK(WHEA_X64_REGISTER_STATE, Reserved, 140,  4);
CPER_FIELD_CHECK(WHEA_X64_REGISTER_STATE, Rflags,   144,  8);
CPER_FIELD_CHECK(WHEA_X64_REGISTER_STATE, Eip,      152,  8);
CPER_FIELD_CHECK(WHEA_X64_REGISTER_STATE, Cr0,      160,  8);
CPER_FIELD_CHECK(WHEA_X64_REGISTER_STATE, Cr1,      168,  8);
CPER_FIELD_CHECK(WHEA_X64_REGISTER_STATE, Cr2,      176,  8);
CPER_FIELD_CHECK(WHEA_X64_REGISTER_STATE, Cr3,      184,  8);
CPER_FIELD_CHECK(WHEA_X64_REGISTER_STATE, Cr4,      192,  8);
CPER_FIELD_CHECK(WHEA_X64_REGISTER_STATE, Cr8,      200,  8);
CPER_FIELD_CHECK(WHEA_X64_REGISTER_STATE, Gdtr,     208, 16);
CPER_FIELD_CHECK(WHEA_X64_REGISTER_STATE, Idtr,     224, 16);
CPER_FIELD_CHECK(WHEA_X64_REGISTER_STATE, Ldtr,     240,  2);
CPER_FIELD_CHECK(WHEA_X64_REGISTER_STATE, Tr,       242,  2);

CPER_FIELD_CHECK(WHEA_XPF_CONTEXT_INFO, RegisterContextType,  0, 2);
CPER_FIELD_CHECK(WHEA_XPF_CONTEXT_INFO, RegisterDataSize,     2, 2);
CPER_FIELD_CHECK(WHEA_XPF_CONTEXT_INFO, MSRAddress,           4, 4);
CPER_FIELD_CHECK(WHEA_XPF_CONTEXT_INFO, MmRegisterAddress,    8, 8);

CPER_FIELD_CHECK(WHEA_XPF_PROCESSOR_ERROR_SECTION, ValidBits,     0,  8);
CPER_FIELD_CHECK(WHEA_XPF_PROCESSOR_ERROR_SECTION, LocalAPICId,   8,  8);
CPER_FIELD_CHECK(WHEA_XPF_PROCESSOR_ERROR_SECTION, CpuId,        16, 48);
CPER_FIELD_CHECK(WHEA_XPF_PROCESSOR_ERROR_SECTION, VariableInfo, 64, ANYSIZE_ARRAY);

//--------------------------------------------------- WHEA_MEMORY_ERROR_SECTION

typedef union _WHEA_MEMORY_ERROR_SECTION_VALIDBITS {
    struct {
        ULONGLONG ErrorStatus:1;
        ULONGLONG PhysicalAddress:1;
        ULONGLONG PhysicalAddressMask:1;
        ULONGLONG Node:1;
        ULONGLONG Card:1;
        ULONGLONG Module:1;
        ULONGLONG Bank:1;
        ULONGLONG Device:1;
        ULONGLONG Row:1;
        ULONGLONG Column:1;
        ULONGLONG BitPosition:1;
        ULONGLONG RequesterId:1;
        ULONGLONG ResponderId:1;
        ULONGLONG TargetId:1;
        ULONGLONG ErrorType:1;
        ULONGLONG RankNumber:1;
        ULONGLONG CardHandle:1;
        ULONGLONG ModuleHandle:1;
        ULONGLONG ExtendedRow:1;
        ULONGLONG BankGroup:1;
        ULONGLONG BankAddress:1;
        ULONGLONG ChipIdentification:1;
        ULONGLONG Reserved:42;
    } DUMMYSTRUCTNAME;
    ULONGLONG ValidBits;
} WHEA_MEMORY_ERROR_SECTION_VALIDBITS,
  *PWHEA_MEMORY_ERROR_SECTION_VALIDBITS;

#define WHEA_MEMERRTYPE_UNKNOWN                 0x00
#define WHEA_MEMERRTYPE_NOERROR                 0x01
#define WHEA_MEMERRTYPE_SINGLEBITECC            0x02
#define WHEA_MEMERRTYPE_MULTIBITECC             0x03
#define WHEA_MEMERRTYPE_SINGLESYMCHIPKILL       0x04
#define WHEA_MEMERRTYPE_MULTISYMCHIPKILL        0x05
#define WHEA_MEMERRTYPE_MASTERABORT             0x06
#define WHEA_MEMERRTYPE_TARGETABORT             0x07
#define WHEA_MEMERRTYPE_PARITYERROR             0x08
#define WHEA_MEMERRTYPE_WATCHDOGTIMEOUT         0x09
#define WHEA_MEMERRTYPE_INVALIDADDRESS          0x0A
#define WHEA_MEMERRTYPE_MIRRORBROKEN            0x0B
#define WHEA_MEMERRTYPE_MEMORYSPARING           0x0C

typedef struct _WHEA_MEMORY_ERROR_SECTION {
    WHEA_MEMORY_ERROR_SECTION_VALIDBITS ValidBits;
    WHEA_ERROR_STATUS ErrorStatus;
    ULONGLONG PhysicalAddress;
    ULONGLONG PhysicalAddressMask;
    USHORT Node;
    USHORT Card;
    USHORT Module;
    USHORT Bank;
    USHORT Device;
    USHORT Row;
    USHORT Column;
    USHORT BitPosition;
    ULONGLONG RequesterId;
    ULONGLONG ResponderId;
    ULONGLONG TargetId;
    UCHAR ErrorType;
    UCHAR Extended;
    USHORT RankNumber;
    USHORT CardHandle;
    USHORT ModuleHandle;
} WHEA_MEMORY_ERROR_SECTION, *PWHEA_MEMORY_ERROR_SECTION;

//
// Define alternate names allowing for downlevel source compatibility.
//

#if WHEA_DOWNLEVEL_TYPE_NAMES

typedef WHEA_MEMORY_ERROR_SECTION_VALIDBITS
    WHEA_MEMORY_ERROR_VALIDBITS, *PWHEA_MEMORY_ERROR_VALIDBITS;

typedef WHEA_MEMORY_ERROR_SECTION
    WHEA_MEMORY_ERROR, *PWHEA_MEMORY_ERROR;

#endif

//
// Validate the memory error section structures against the definitions in the
// UEFI  specification.
//

CPER_FIELD_CHECK(WHEA_MEMORY_ERROR_SECTION, ValidBits,            0, 8);
CPER_FIELD_CHECK(WHEA_MEMORY_ERROR_SECTION, ErrorStatus,          8, 8);
CPER_FIELD_CHECK(WHEA_MEMORY_ERROR_SECTION, PhysicalAddress,     16, 8);
CPER_FIELD_CHECK(WHEA_MEMORY_ERROR_SECTION, PhysicalAddressMask, 24, 8);
CPER_FIELD_CHECK(WHEA_MEMORY_ERROR_SECTION, Node,                32, 2);
CPER_FIELD_CHECK(WHEA_MEMORY_ERROR_SECTION, Card,                34, 2);
CPER_FIELD_CHECK(WHEA_MEMORY_ERROR_SECTION, Module,              36, 2);
CPER_FIELD_CHECK(WHEA_MEMORY_ERROR_SECTION, Bank,                38, 2);
CPER_FIELD_CHECK(WHEA_MEMORY_ERROR_SECTION, Device,              40, 2);
CPER_FIELD_CHECK(WHEA_MEMORY_ERROR_SECTION, Row,                 42, 2);
CPER_FIELD_CHECK(WHEA_MEMORY_ERROR_SECTION, Column,              44, 2);
CPER_FIELD_CHECK(WHEA_MEMORY_ERROR_SECTION, BitPosition,         46, 2);
CPER_FIELD_CHECK(WHEA_MEMORY_ERROR_SECTION, RequesterId,         48, 8);
CPER_FIELD_CHECK(WHEA_MEMORY_ERROR_SECTION, ResponderId,         56, 8);
CPER_FIELD_CHECK(WHEA_MEMORY_ERROR_SECTION, TargetId,            64, 8);
CPER_FIELD_CHECK(WHEA_MEMORY_ERROR_SECTION, ErrorType,           72, 1);

//----------------------------------------------- WHEA_MEMORY_ERROR_EXT_SECTION

typedef enum _WHEA_MEMORY_DEFINITION {
    WheaMemoryUndefined = 0,
    WheaMemoryFm,
    WheaMemoryNm,
    WheaMemoryHbm,
    WheaMemoryMax
} WHEA_MEMORY_DEFINITION, *PWHEA_MEMORY_DEFINITION;

typedef union _WHEA_MEMORY_ERROR_EXT_SECTION_FLAGS {
    struct {
        UINT64 AddressTranslationByPrmSuccess : 1;
        UINT64 AddressTranslationByPrmFailed : 1;
        UINT64 AddressTranslationByPrmNotSupported : 1;
        UINT64 AddressTranslationByPluginSuccess : 1;
        UINT64 AddressTranslationByPluginFailed : 1;
        UINT64 AddressTranslationByPluginNotSupported : 1;
        UINT64 Reserved : 58;
    } DUMMYSTRUCTNAME;

    UINT64 AsUINT64;
} WHEA_MEMORY_ERROR_EXT_SECTION_FLAGS, *PWHEA_MEMORY_ERROR_EXT_SECTION_FLAGS;

typedef union _WHEA_MEMORY_ERROR_EXT_SECTION_INTEL_VALIDBITS {
    struct {
        UINT64 MemDef : 1;
        UINT64 SystemAddress : 1;
        UINT64 SpareSystemAddress : 1;
        UINT64 DevicePhysicalAddress : 1;
        UINT64 ChannelAddress : 1;
        UINT64 RankAddress : 1;
        UINT64 ProcessorSocketId : 1;
        UINT64 MemoryControllerId : 1;
        UINT64 TargetId : 1;
        UINT64 LogicalChannelId : 1;
        UINT64 ChannelId : 1;
        UINT64 SubChannelId : 1;
        UINT64 PhysicalRankId : 1;
        UINT64 DimmSlotId : 1;
        UINT64 DimmRankId : 1;
        UINT64 Bank : 1;
        UINT64 BankGroup : 1;
        UINT64 Row : 1;
        UINT64 Column : 1;
        UINT64 LockStepRank : 1;
        UINT64 LockStepPhysicalRank : 1;
        UINT64 LockStepBank : 1;
        UINT64 LockStepBankGroup : 1;
        UINT64 ChipSelect : 1;
        UINT64 Node : 1;
        UINT64 ChipId : 1;
        UINT64 Reserved : 38;
    } DUMMYSTRUCTNAME;

    UINT64 ValidBits;
} WHEA_MEMORY_ERROR_EXT_SECTION_INTEL_VALIDBITS,
  *PWHEA_MEMORY_ERROR_EXT_SECTION_INTEL_VALIDBITS;

typedef struct _WHEA_MEMORY_HARDWARE_ADDRESS_INTEL {
    WHEA_MEMORY_DEFINITION MemDef;
    UINT64 SystemAddress;
    UINT64 SpareSystemAddress;
    UINT64 DevicePhysicalAddress;
    UINT64 ChannelAddress;
    UINT64 RankAddress;
    UINT8 ProcessorSocketId;
    UINT8 MemoryControllerId;
    UINT8 TargetId;
    UINT8 LogicalChannelId;
    UINT8 ChannelId;
    UINT8 SubChannelId;
    UINT8 PhysicalRankId;
    UINT8 DimmSlotId;
    UINT8 DimmRankId;
    UINT8 Bank;
    UINT8 BankGroup;
    UINT32 Row;
    UINT32 Column;
    UINT8 LockStepRank;
    UINT8 LockStepPhysicalRank;
    UINT8 LockStepBank;
    UINT8 LockStepBankGroup;
    UINT8 ChipSelect;
    UINT8 Node;
    UINT8 ChipId;
    UINT8 Reserved[40];
} WHEA_MEMORY_HARDWARE_ADDRESS_INTEL, *PWHEA_MEMORY_HARDWARE_ADDRESS_INTEL;

typedef struct _WHEA_MEMORY_ERROR_EXT_SECTION_INTEL {
    WHEA_MEMORY_ERROR_EXT_SECTION_FLAGS Flags;
    WHEA_MEMORY_ERROR_EXT_SECTION_INTEL_VALIDBITS ValidBits;
    WHEA_MEMORY_HARDWARE_ADDRESS_INTEL HardwareAddress;
    UINT8 Reserved[40];
} WHEA_MEMORY_ERROR_EXT_SECTION_INTEL, *PWHEA_MEMORY_ERROR_EXT_SECTION_INTEL;

//----------------------------------------------------- WHEA_PMEM_ERROR_SECTION

#define WHEA_PMEM_ERROR_SECTION_LOCATION_INFO_SIZE 64
#define WHEA_PMEM_ERROR_SECTION_MAX_PAGES 50

typedef union _WHEA_PMEM_ERROR_SECTION_VALIDBITS {
    struct {
        ULONGLONG ErrorStatus:1;
        ULONGLONG NFITHandle:1;
        ULONGLONG LocationInfo:1;
        ULONGLONG Reserved:61;
    } DUMMYSTRUCTNAME;
    ULONGLONG ValidBits;
} WHEA_PMEM_ERROR_SECTION_VALIDBITS,
  *PWHEA_PMEM_ERROR_SECTION_VALIDBITS;

typedef struct _WHEA_PMEM_PAGE_RANGE {
    ULONG64 StartingPfn;
    ULONG64 PageCount;
    ULONG64 MarkedBadBitmap;
} WHEA_PMEM_PAGE_RANGE, *PWHEA_PMEM_PAGE_RANGE;

#define WHEA_PMEM_IS_PFN_ALREADY_MARKED_BAD(PageRange, TargetPfn) \
    (((TargetPfn) - ((PageRange)->StartingPfn) < sizeof(ULONG64) * 8) && \
     ((((PageRange)->MarkedBadBitmap) & (1ull << ((TargetPfn) - ((PageRange)->StartingPfn)))) != 0))

#define WHEA_PMEM_IS_PAGE_RANGE_ALREADY_MARKED_BAD(PageRange) \
    (((PageRange)->PageCount <= sizeof(ULONG64) * 8) && \
     (((PageRange)->MarkedBadBitmap) == (ULONG64_MAX >> (sizeof(ULONG64) * 8 - (PageRange)->PageCount))))

typedef struct _WHEA_PMEM_ERROR_SECTION {
    WHEA_PMEM_ERROR_SECTION_VALIDBITS ValidBits;
    UCHAR LocationInfo[WHEA_PMEM_ERROR_SECTION_LOCATION_INFO_SIZE];
    WHEA_ERROR_STATUS ErrorStatus;
    ULONG NFITHandle;
    ULONG PageRangeCount;
    WHEA_PMEM_PAGE_RANGE PageRange[ANYSIZE_ARRAY];
} WHEA_PMEM_ERROR_SECTION, *PWHEA_PMEM_ERROR_SECTION;

CPER_FIELD_CHECK(WHEA_PMEM_ERROR_SECTION, ValidBits,            0, 8);
CPER_FIELD_CHECK(WHEA_PMEM_ERROR_SECTION, LocationInfo,         8, 64);
CPER_FIELD_CHECK(WHEA_PMEM_ERROR_SECTION, ErrorStatus,         72, 8);
CPER_FIELD_CHECK(WHEA_PMEM_ERROR_SECTION, NFITHandle,          80, 4);
CPER_FIELD_CHECK(WHEA_PMEM_ERROR_SECTION, PageRangeCount,      84, 4);

//----------------------------------------- WHEA_PCIE_CORRECTABLE_ERROR_SECTION

#define WHEA_PCIE_CORRECTABLE_ERROR_SECTION_COUNT_SIZE 32

typedef struct _WHEA_PCIE_ADDRESS {
    UINT32 Segment;
    UINT32 Bus;
    UINT32 Device;
    UINT32 Function;
} WHEA_PCIE_ADDRESS, *PWHEA_PCIE_ADDRESS;

typedef union _WHEA_PCIE_CORRECTABLE_ERROR_DEVICES_VALIDBITS {
    struct {
        ULONGLONG Segment:1;
        ULONGLONG Bus:1;
        ULONGLONG Device:1;
        ULONGLONG Function:1;
        ULONGLONG Mask:1;
        ULONGLONG CorrectableErrorCount:1;
        ULONGLONG Reserved:58;
    } DUMMYSTRUCTNAME;
    ULONGLONG ValidBits;
} WHEA_PCIE_CORRECTABLE_ERROR_DEVICES_VALIDBITS,
  *PWHEA_PCIE_CORRECTABLE_ERROR_DEVICES_VALIDBITS;

typedef struct _WHEA_PCIE_CORRECTABLE_ERROR_DEVICES {
    WHEA_PCIE_CORRECTABLE_ERROR_DEVICES_VALIDBITS ValidBits;
    WHEA_PCIE_ADDRESS Address;
    UINT32 Mask;
    UINT32 CorrectableErrorCount
               [WHEA_PCIE_CORRECTABLE_ERROR_SECTION_COUNT_SIZE];
} WHEA_PCIE_CORRECTABLE_ERROR_DEVICES, *PWHEA_PCIE_CORRECTABLE_ERROR_DEVICES;

typedef struct _WHEA_PCIE_CORRECTABLE_ERROR_SECTION_HEADER {
    UINT16 Version;
    UINT16 Count;
} WHEA_PCIE_CORRECTABLE_ERROR_SECTION_HEADER, 
      *PWHEA_PCIE_CORRECTABLE_ERROR_SECTION_HEADER;

typedef struct _WHEA_PCIE_CORRECTABLE_ERROR_SECTION {
    WHEA_PCIE_CORRECTABLE_ERROR_SECTION_HEADER Header;
    _Field_size_(Header.Count)
        WHEA_PCIE_CORRECTABLE_ERROR_DEVICES Devices[ANYSIZE_ARRAY];
} WHEA_PCIE_CORRECTABLE_ERROR_SECTION, *PWHEA_PCIE_CORRECTABLE_ERROR_SECTION;

CPER_FIELD_CHECK(WHEA_PCIE_CORRECTABLE_ERROR_SECTION, Header,  0,   4);
CPER_FIELD_CHECK(WHEA_PCIE_CORRECTABLE_ERROR_SECTION, Devices, 4, 156);

//----------------------------------------- WHEA_MEMORY_CORRECTABLE_ERROR_SECTION

typedef union _WHEA_MEMORY_CORRECTABLE_ERROR_SECTION_VALIDBITS {
    struct {
        ULONGLONG SocketId:1;
        ULONGLONG ChannelId:1;
        ULONGLONG DimmSlot:1;
        ULONGLONG CorrectableErrorCount:1;
        ULONGLONG Reserved:60;
    } DUMMYSTRUCTNAME;
    ULONGLONG ValidBits;
} WHEA_MEMORY_CORRECTABLE_ERROR_SECTION_VALIDBITS,
  *PWHEA_MEMORY_CORRECTABLE_ERROR_SECTION_VALIDBITS;

typedef struct _WHEA_MEMORY_CORRECTABLE_ERROR_HEADER {
    UINT16 Version;
    UINT16 Count;
} WHEA_MEMORY_CORRECTABLE_ERROR_HEADER, *PWHEA_MEMORY_CORRECTABLE_ERROR_HEADER;

typedef struct _WHEA_MEMORY_CORRECTABLE_ERROR_DATA {
    WHEA_MEMORY_CORRECTABLE_ERROR_SECTION_VALIDBITS ValidBits;
    UINT32 SocketId;
    UINT32 ChannelId;
    UINT32 DimmSlot;
    UINT32 CorrectableErrorCount;
} WHEA_MEMORY_CORRECTABLE_ERROR_DATA, *PWHEA_MEMORY_CORRECTABLE_ERROR_DATA;

typedef struct _WHEA_MEMORY_CORRECTABLE_ERROR_SECTION {
    WHEA_MEMORY_CORRECTABLE_ERROR_HEADER Header;
    WHEA_MEMORY_CORRECTABLE_ERROR_DATA Data[ANYSIZE_ARRAY];
} WHEA_MEMORY_CORRECTABLE_ERROR_SECTION, 
    *PWHEA_MEMORY_CORRECTABLE_ERROR_SECTION;

CPER_FIELD_CHECK(WHEA_MEMORY_CORRECTABLE_ERROR_SECTION, Header, 0,  4);
CPER_FIELD_CHECK(WHEA_MEMORY_CORRECTABLE_ERROR_SECTION, Data,   4,  24);

//----------------------------------------------- WHEA_PCIEXPRESS_ERROR_SECTION

typedef union _WHEA_PCIEXPRESS_ERROR_SECTION_VALIDBITS {
    struct {
        ULONGLONG PortType:1;
        ULONGLONG Version:1;
        ULONGLONG CommandStatus:1;
        ULONGLONG DeviceId:1;
        ULONGLONG DeviceSerialNumber:1;
        ULONGLONG BridgeControlStatus:1;
        ULONGLONG ExpressCapability:1;
        ULONGLONG AerInfo:1;
        ULONGLONG Reserved:56;
    } DUMMYSTRUCTNAME;
    ULONGLONG ValidBits;
} WHEA_PCIEXPRESS_ERROR_SECTION_VALIDBITS,
  *PWHEA_PCIEXPRESS_ERROR_SECTION_VALIDBITS;

typedef struct _WHEA_PCIEXPRESS_DEVICE_ID {
    USHORT VendorID;
    USHORT DeviceID;
    ULONG ClassCode:24;
    ULONG FunctionNumber:8;
    ULONG DeviceNumber:8;
    ULONG Segment:16;
    ULONG PrimaryBusNumber:8;
    ULONG SecondaryBusNumber:8;
    ULONG Reserved1:3;
    ULONG SlotNumber:13;
    ULONG Reserved2:8;
} WHEA_PCIEXPRESS_DEVICE_ID, *PWHEA_PCIEXPRESS_DEVICE_ID;

typedef union _WHEA_PCIEXPRESS_VERSION {
    struct {
        UCHAR MinorVersion;
        UCHAR MajorVersion;
        USHORT Reserved;
    } DUMMYSTRUCTNAME;
    ULONG AsULONG;
} WHEA_PCIEXPRESS_VERSION, *PWHEA_PCIEXPRESS_VERSION;

typedef union _WHEA_PCIEXPRESS_COMMAND_STATUS {
    struct {
        USHORT Command;
        USHORT Status;
    } DUMMYSTRUCTNAME;
    ULONG AsULONG;
} WHEA_PCIEXPRESS_COMMAND_STATUS, *PWHEA_PCIEXPRESS_COMMAND_STATUS;

typedef union _WHEA_PCIEXPRESS_BRIDGE_CONTROL_STATUS {
    struct {
        USHORT BridgeSecondaryStatus;
        USHORT BridgeControl;
    } DUMMYSTRUCTNAME;
    ULONG AsULONG;
} WHEA_PCIEXPRESS_BRIDGE_CONTROL_STATUS,
    *PWHEA_PCIEXPRESS_BRIDGE_CONTROL_STATUS;

typedef enum _WHEA_PCIEXPRESS_DEVICE_TYPE {
    WheaPciExpressEndpoint = 0,
    WheaPciExpressLegacyEndpoint,
    WheaPciExpressRootPort = 4,
    WheaPciExpressUpstreamSwitchPort,
    WheaPciExpressDownstreamSwitchPort,
    WheaPciExpressToPciXBridge,
    WheaPciXToExpressBridge,
    WheaPciExpressRootComplexIntegratedEndpoint,
    WheaPciExpressRootComplexEventCollector
} WHEA_PCIEXPRESS_DEVICE_TYPE;

typedef struct _WHEA_PCIEXPRESS_ERROR_SECTION {
    WHEA_PCIEXPRESS_ERROR_SECTION_VALIDBITS ValidBits;
    WHEA_PCIEXPRESS_DEVICE_TYPE PortType;
    WHEA_PCIEXPRESS_VERSION Version;
    WHEA_PCIEXPRESS_COMMAND_STATUS CommandStatus;
    ULONG Reserved;
    WHEA_PCIEXPRESS_DEVICE_ID DeviceId;
    ULONGLONG DeviceSerialNumber;
    WHEA_PCIEXPRESS_BRIDGE_CONTROL_STATUS BridgeControlStatus;
    UCHAR ExpressCapability[60];
    UCHAR AerInfo[96];
} WHEA_PCIEXPRESS_ERROR_SECTION, *PWHEA_PCIEXPRESS_ERROR_SECTION;

#if WHEA_DOWNLEVEL_TYPE_NAMES

typedef WHEA_PCIEXPRESS_ERROR_SECTION_VALIDBITS
    WHEA_PCIEXPRESS_ERROR_VALIDBITS,
    *PWHEA_PCIEXPRESS_ERROR_VALIDBITS;

typedef WHEA_PCIEXPRESS_ERROR_SECTION
    WHEA_PCIEXPRESS_ERROR, *PWHEA_PCIEXPRESS_ERROR;

#endif

//
// Validate the PCI Express error section structures against the definitions
// in the UEFI  specification.
//

CPER_FIELD_CHECK(WHEA_PCIEXPRESS_ERROR_SECTION, ValidBits,             0,  8);
CPER_FIELD_CHECK(WHEA_PCIEXPRESS_ERROR_SECTION, PortType,              8,  4);
CPER_FIELD_CHECK(WHEA_PCIEXPRESS_ERROR_SECTION, Version,              12,  4);
CPER_FIELD_CHECK(WHEA_PCIEXPRESS_ERROR_SECTION, CommandStatus,        16,  4);
CPER_FIELD_CHECK(WHEA_PCIEXPRESS_ERROR_SECTION, Reserved,             20,  4);
CPER_FIELD_CHECK(WHEA_PCIEXPRESS_ERROR_SECTION, DeviceId,             24, 16);
CPER_FIELD_CHECK(WHEA_PCIEXPRESS_ERROR_SECTION, DeviceSerialNumber,   40,  8);
CPER_FIELD_CHECK(WHEA_PCIEXPRESS_ERROR_SECTION, BridgeControlStatus,  48,  4);
CPER_FIELD_CHECK(WHEA_PCIEXPRESS_ERROR_SECTION, ExpressCapability,    52, 60);
CPER_FIELD_CHECK(WHEA_PCIEXPRESS_ERROR_SECTION, AerInfo,             112, 96);

//-------------------------------------------------- WHEA_PCIXBUS_ERROR_SECTION

#define PCIXBUS_ERRTYPE_UNKNOWN             0x0000
#define PCIXBUS_ERRTYPE_DATAPARITY          0x0001
#define PCIXBUS_ERRTYPE_SYSTEM              0x0002
#define PCIXBUS_ERRTYPE_MASTERABORT         0x0003
#define PCIXBUS_ERRTYPE_BUSTIMEOUT          0x0004
#define PCIXBUS_ERRTYPE_MASTERDATAPARITY    0x0005
#define PCIXBUS_ERRTYPE_ADDRESSPARITY       0x0006
#define PCIXBUS_ERRTYPE_COMMANDPARITY       0x0007

typedef union _WHEA_PCIXBUS_ERROR_SECTION_VALIDBITS {
    struct {
        ULONGLONG ErrorStatus:1;
        ULONGLONG ErrorType:1;
        ULONGLONG BusId:1;
        ULONGLONG BusAddress:1;
        ULONGLONG BusData:1;
        ULONGLONG BusCommand:1;
        ULONGLONG RequesterId:1;
        ULONGLONG CompleterId:1;
        ULONGLONG TargetId:1;
        ULONGLONG Reserved:55;
    } DUMMYSTRUCTNAME;
    ULONGLONG ValidBits;
} WHEA_PCIXBUS_ERROR_SECTION_VALIDBITS, *PWHEA_PCIXBUS_ERROR_SECTION_VALIDBITS;

typedef union _WHEA_PCIXBUS_ID {
    struct {
        UCHAR BusNumber;
        UCHAR BusSegment;
    } DUMMYSTRUCTNAME;
    USHORT AsUSHORT;
} WHEA_PCIXBUS_ID, *PWHEA_PCIXBUS_ID;

typedef union _WHEA_PCIXBUS_COMMAND {
    struct {
        ULONGLONG Command:56;
        ULONGLONG PCIXCommand:1;
        ULONGLONG Reserved:7;
    } DUMMYSTRUCTNAME;
    ULONGLONG AsULONGLONG;
} WHEA_PCIXBUS_COMMAND, *PWHEA_PCIXBUS_COMMAND;

typedef struct _WHEA_PCIXBUS_ERROR_SECTION {
    WHEA_PCIXBUS_ERROR_SECTION_VALIDBITS ValidBits;
    WHEA_ERROR_STATUS ErrorStatus;
    USHORT ErrorType;
    WHEA_PCIXBUS_ID BusId;
    ULONG Reserved;
    ULONGLONG BusAddress;
    ULONGLONG BusData;
    WHEA_PCIXBUS_COMMAND BusCommand;
    ULONGLONG RequesterId;
    ULONGLONG CompleterId;
    ULONGLONG TargetId;
} WHEA_PCIXBUS_ERROR_SECTION, *PWHEA_PCIXBUS_ERROR_SECTION;

#if WHEA_DOWNLEVEL_TYPE_NAMES

typedef WHEA_PCIXBUS_ERROR_SECTION_VALIDBITS
    WHEA_PCIXBUS_ERROR_VALIDBITS,
    *PWHEA_PCIXBUS_ERROR_VALIDBITS;

typedef WHEA_PCIXBUS_ERROR_SECTION
    WHEA_PCIXBUS_ERROR, *PWHEA_PCIXBUS_ERROR;

#endif

CPER_FIELD_CHECK(WHEA_PCIXBUS_ERROR_SECTION, ValidBits,    0, 8);
CPER_FIELD_CHECK(WHEA_PCIXBUS_ERROR_SECTION, ErrorStatus,  8, 8);
CPER_FIELD_CHECK(WHEA_PCIXBUS_ERROR_SECTION, ErrorType,   16, 2);
CPER_FIELD_CHECK(WHEA_PCIXBUS_ERROR_SECTION, BusId,       18, 2);
CPER_FIELD_CHECK(WHEA_PCIXBUS_ERROR_SECTION, Reserved,    20, 4);
CPER_FIELD_CHECK(WHEA_PCIXBUS_ERROR_SECTION, BusAddress,  24, 8);
CPER_FIELD_CHECK(WHEA_PCIXBUS_ERROR_SECTION, BusData,     32, 8);
CPER_FIELD_CHECK(WHEA_PCIXBUS_ERROR_SECTION, BusCommand,  40, 8);
CPER_FIELD_CHECK(WHEA_PCIXBUS_ERROR_SECTION, RequesterId, 48, 8);
CPER_FIELD_CHECK(WHEA_PCIXBUS_ERROR_SECTION, CompleterId, 56, 8);
CPER_FIELD_CHECK(WHEA_PCIXBUS_ERROR_SECTION, TargetId,    64, 8);

//----------------------------------------------- WHEA_PCIXDEVICE_ERROR_SECTION

typedef union _WHEA_PCIXDEVICE_ERROR_SECTION_VALIDBITS {
    struct {
        ULONGLONG ErrorStatus:1;
        ULONGLONG IdInfo:1;
        ULONGLONG MemoryNumber:1;
        ULONGLONG IoNumber:1;
        ULONGLONG RegisterDataPairs:1;
        ULONGLONG Reserved:59;
    } DUMMYSTRUCTNAME;
    ULONGLONG ValidBits;
} WHEA_PCIXDEVICE_ERROR_SECTION_VALIDBITS,
  *PWHEA_PCIXDEVICE_ERROR_SECTION_VALIDBITS;

typedef struct _WHEA_PCIXDEVICE_ID {
    USHORT VendorId;
    USHORT DeviceId;
    ULONG ClassCode:24;
    ULONG FunctionNumber:8;
    ULONG DeviceNumber:8;
    ULONG BusNumber:8;
    ULONG SegmentNumber:8;
    ULONG Reserved1:8;
    ULONG Reserved2;
} WHEA_PCIXDEVICE_ID, *PWHEA_PCIXDEVICE_ID;

typedef struct WHEA_PCIXDEVICE_REGISTER_PAIR {
    ULONGLONG Register;
    ULONGLONG Data;
} WHEA_PCIXDEVICE_REGISTER_PAIR, *PWHEA_PCIXDEVICE_REGISTER_PAIR;

typedef struct _WHEA_PCIXDEVICE_ERROR_SECTION {
    WHEA_PCIXDEVICE_ERROR_SECTION_VALIDBITS ValidBits;
    WHEA_ERROR_STATUS ErrorStatus;
    WHEA_PCIXDEVICE_ID IdInfo;
    ULONG MemoryNumber;
    ULONG IoNumber;
    WHEA_PCIXDEVICE_REGISTER_PAIR RegisterDataPairs[ANYSIZE_ARRAY];
} WHEA_PCIXDEVICE_ERROR_SECTION, *PWHEA_PCIXDEVICE_ERROR_SECTION;

#if WHEA_DOWNLEVEL_TYPE_NAMES

typedef WHEA_PCIXDEVICE_ERROR_SECTION_VALIDBITS
    WHEA_PCIXDEVICE_ERROR_VALIDBITS, *PWHEA_PCIXDEVICE_ERROR_VALIDBITS;

typedef WHEA_PCIXDEVICE_ERROR_SECTION
    WHEA_PCIXDEVICE_ERROR, *PWHEA_PCIXDEVICE_ERROR;

#endif

CPER_FIELD_CHECK(WHEA_PCIXDEVICE_ERROR_SECTION, ValidBits,          0,  8);
CPER_FIELD_CHECK(WHEA_PCIXDEVICE_ERROR_SECTION, ErrorStatus,        8,  8);
CPER_FIELD_CHECK(WHEA_PCIXDEVICE_ERROR_SECTION, IdInfo,            16, 16);
CPER_FIELD_CHECK(WHEA_PCIXDEVICE_ERROR_SECTION, MemoryNumber,      32,  4);
CPER_FIELD_CHECK(WHEA_PCIXDEVICE_ERROR_SECTION, IoNumber,          36,  4);
CPER_FIELD_CHECK(WHEA_PCIXDEVICE_ERROR_SECTION, RegisterDataPairs, 40, 16);

//---------------------------------------- WHEA_FIRMWARE_ERROR_RECORD_REFERENCE

#define WHEA_FIRMWARE_RECORD_TYPE_IPFSAL 0

typedef struct _WHEA_FIRMWARE_ERROR_RECORD_REFERENCE {
    UCHAR Type;
    UCHAR Reserved[7];
    ULONGLONG FirmwareRecordId;
} WHEA_FIRMWARE_ERROR_RECORD_REFERENCE, *PWHEA_FIRMWARE_ERROR_RECORD_REFERENCE;

#if WHEA_DOWNLEVEL_TYPE_NAMES

typedef WHEA_FIRMWARE_ERROR_RECORD_REFERENCE
    WHEA_FIRMWARE_RECORD, *PWHEA_FIRMWARE_RECORD;

#endif

CPER_FIELD_CHECK(WHEA_FIRMWARE_ERROR_RECORD_REFERENCE, Type,             0,  1);
CPER_FIELD_CHECK(WHEA_FIRMWARE_ERROR_RECORD_REFERENCE, Reserved,         1,  7);
CPER_FIELD_CHECK(WHEA_FIRMWARE_ERROR_RECORD_REFERENCE, FirmwareRecordId, 8,  8);

//
// This is the start of the Microsoft specific extensions to the Common Platform
// Error Record specification. This is in accordance with Appendix N, section
// 2.3 of the Unified Extensible Firmware Interface specification, which allows
// the specification of non-standard section bodies.
//

//------------------------------------------------------------- XPF_MCA_SECTION

//
// The IA32_MCG_CAP register provides information about the machine-check
// architecture of the processor.
// From: Intel 64 and IA-32 Architectures SDM
//       Volume 3B; December 2017; Chapter 15.3.1.1
//

typedef union _MCG_CAP {
    struct {
        ULONG64 CountField: 8;
        ULONG64 ControlMsrPresent: 1;
        ULONG64 ExtendedMsrsPresent: 1;
        ULONG64 SignalingExtensionPresent: 1;
        ULONG64 ThresholdErrorStatusPresent: 1;
        ULONG64 Reserved: 4;
        ULONG64 ExtendedRegisterCount: 8;
        ULONG64 SoftwareErrorRecoverySupported: 1;
        ULONG64 EnhancedMachineCheckCapability: 1;
        ULONG64 ExtendedErrorLogging: 1;
        ULONG64 LocalMachineCheckException: 1;
    } DUMMYSTRUCTNAME;
    ULONG64 QuadPart;
} MCG_CAP, *PMCG_CAP;

typedef union _MCG_STATUS {
    struct {
        ULONG RestartIpValid:1;
        ULONG ErrorIpValid:1;
        ULONG MachineCheckInProgress:1;
        ULONG LocalMceValid:1;
        ULONG Reserved1:28;
        ULONG Reserved2;
    } DUMMYSTRUCTNAME;
    ULONGLONG QuadPart;
} MCG_STATUS, *PMCG_STATUS;

typedef struct _MCI_STATUS_BITS_COMMON {
        ULONG64 McaErrorCode : 16;
        ULONG64 ModelErrorCode : 16;
        ULONG64 Reserved : 25;
        ULONG64 ContextCorrupt : 1;
        ULONG64 AddressValid : 1;
        ULONG64 MiscValid : 1;
        ULONG64 ErrorEnabled : 1;
        ULONG64 UncorrectedError : 1;
        ULONG64 StatusOverFlow : 1;
        ULONG64 Valid : 1;
} MCI_STATUS_BITS_COMMON, *PMCI_STATUS_BITS_COMMON;

//
// WHEA specific implementations of MCI_STATUS register
// Allows for more machine specific granularity
// From: AMD64 Archtecture Programmer's Manual
//       Volume 2; Revision 3.29; Chapter 13
//

typedef struct _MCI_STATUS_AMD_BITS {
        ULONG64 McaErrorCode : 16;
        ULONG64 ModelErrorCode : 16;
        ULONG64 ImplementationSpecific2 : 11;
        ULONG64 Poison : 1;
        ULONG64 Deferred : 1;
        ULONG64 ImplementationSpecific1 : 12;
        ULONG64 ContextCorrupt : 1;
        ULONG64 AddressValid : 1;
        ULONG64 MiscValid : 1;
        ULONG64 ErrorEnabled : 1;
        ULONG64 UncorrectedError : 1;
        ULONG64 StatusOverFlow : 1;
        ULONG64 Valid : 1;
} MCI_STATUS_AMD_BITS, *PMCI_STATUS_AMD_BITS;

//
// From: Intel 64 and IA-32 Architectures SDM
//       Volume 3B; December 2017; Chapter 15
//

typedef struct _MCI_STATUS_INTEL_BITS {
        ULONG64 McaErrorCode : 16;
        ULONG64 ModelErrorCode : 16;
        ULONG64 OtherInfo : 5;
        ULONG64 FirmwareUpdateError : 1;
        ULONG64 CorrectedErrorCount : 15;
        ULONG64 ThresholdErrorStatus : 2;
        ULONG64 ActionRequired : 1;
        ULONG64 Signalling : 1;
        ULONG64 ContextCorrupt : 1;
        ULONG64 AddressValid : 1;
        ULONG64 MiscValid : 1;
        ULONG64 ErrorEnabled : 1;
        ULONG64 UncorrectedError : 1;
        ULONG64 StatusOverFlow : 1;
        ULONG64 Valid : 1;
} MCI_STATUS_INTEL_BITS, *PMCI_STATUS_INTEL_BITS;

typedef union _MCI_STATUS {
    MCI_STATUS_BITS_COMMON CommonBits;
    MCI_STATUS_AMD_BITS AmdBits;
    MCI_STATUS_INTEL_BITS IntelBits;
    ULONG64 QuadPart;
} MCI_STATUS, *PMCI_STATUS;

typedef enum _WHEA_CPU_VENDOR {
    WheaCpuVendorOther = 0,
    WheaCpuVendorIntel,
    WheaCpuVendorAmd
} WHEA_CPU_VENDOR, *PWHEA_CPU_VENDOR;

#define WHEA_XPF_MCA_EXTREG_MAX_COUNT            24
#define WHEA_XPF_MCA_SECTION_VERSION_2           2
#define WHEA_XPF_MCA_SECTION_VERSION_3           3
#define WHEA_XPF_MCA_SECTION_VERSION_4           4
#define WHEA_XPF_MCA_SECTION_VERSION             WHEA_XPF_MCA_SECTION_VERSION_4
#define WHEA_AMD_EXT_REG_NUM                     10
#define WHEA_XPF_MCA_EXBANK_COUNT                32

//
// NOTE: You must update WHEA_AMD_EXT_REG_NUM if you add additional registers
// to this struct to keep the size the same.
//

typedef struct _WHEA_AMD_EXTENDED_REGISTERS {
    ULONGLONG IPID;
    ULONGLONG SYND;
    ULONGLONG CONFIG;
    ULONGLONG DESTAT;
    ULONGLONG DEADDR;
    ULONGLONG MISC1;
    ULONGLONG MISC2;
    ULONGLONG MISC3;
    ULONGLONG MISC4;
    ULONGLONG RasCap;
    ULONGLONG Reserved[WHEA_XPF_MCA_EXTREG_MAX_COUNT - WHEA_AMD_EXT_REG_NUM];
} WHEA_AMD_EXTENDED_REGISTERS, *PWHEA_AMD_EXTENDED_REGISTERS;

typedef struct _XPF_RECOVERY_INFO {
    struct {
        UINT32 NotSupported : 1;
        UINT32 Overflow : 1;
        UINT32 ContextCorrupt : 1;
        UINT32 RestartIpErrorIpNotValid : 1;
        UINT32 NoRecoveryContext : 1;
        UINT32 MiscOrAddrNotValid : 1;
        UINT32 InvalidAddressMode : 1;
        UINT32 HighIrql : 1;
        UINT32 InterruptsDisabled : 1;
        UINT32 SwapBusy : 1;
        UINT32 StackOverflow : 1;
        UINT32 Reserved : 21;
    } FailureReason;

    struct {
        UINT32 RecoveryAttempted : 1;
        UINT32 HvHandled : 1;
        UINT32 Reserved : 30;
    } Action;

    BOOLEAN ActionRequired;
    BOOLEAN RecoverySucceeded;
    BOOLEAN RecoveryKernel;
    UINT8 Reserved;
    UINT16 Reserved2;
    UINT16 Reserved3;
    UINT32 Reserved4;
} XPF_RECOVERY_INFO, *PXPF_RECOVERY_INFO;

typedef struct _WHEA_XPF_MCA_SECTION {
    ULONG VersionNumber;
    WHEA_CPU_VENDOR CpuVendor;
    LARGE_INTEGER Timestamp;
    ULONG ProcessorNumber;
    MCG_STATUS GlobalStatus;
    ULONGLONG InstructionPointer;
    ULONG BankNumber;
    MCI_STATUS Status;
    ULONGLONG Address;
    ULONGLONG Misc;
    ULONG ExtendedRegisterCount;
    ULONG ApicId;
    union {
        ULONGLONG ExtendedRegisters[WHEA_XPF_MCA_EXTREG_MAX_COUNT];
        WHEA_AMD_EXTENDED_REGISTERS AMDExtendedRegisters;
    };
    MCG_CAP GlobalCapability;

    //
    // Version 3 Fields follow.
    //

    XPF_RECOVERY_INFO RecoveryInfo;

    //
    // Version 4 Fields follow.
    //

    ULONG ExBankCount;
    ULONG BankNumberEx[WHEA_XPF_MCA_EXBANK_COUNT];
    MCI_STATUS StatusEx[WHEA_XPF_MCA_EXBANK_COUNT];
    ULONGLONG AddressEx[WHEA_XPF_MCA_EXBANK_COUNT];
    ULONGLONG MiscEx[WHEA_XPF_MCA_EXBANK_COUNT];
} WHEA_XPF_MCA_SECTION, *PWHEA_XPF_MCA_SECTION;

//------------------------------------------------------ WHEA_NMI_ERROR_SECTION

typedef union _WHEA_NMI_ERROR_SECTION_FLAGS {
    struct {
        ULONG HypervisorError:1;
        ULONG Reserved:31;
    } DUMMYSTRUCTNAME;
    ULONG AsULONG;
} WHEA_NMI_ERROR_SECTION_FLAGS, *PWHEA_NMI_ERROR_SECTION_FLAGS;

typedef struct _WHEA_NMI_ERROR_SECTION {
    UCHAR Data[8];
    WHEA_NMI_ERROR_SECTION_FLAGS Flags;
} WHEA_NMI_ERROR_SECTION, *PWHEA_NMI_ERROR_SECTION;

//------------------------------------------------------ WHEA_MSR_DUMP_SECTION

typedef struct _WHEA_MSR_DUMP_SECTION {
    UCHAR MsrDumpBuffer;
    ULONG MsrDumpLength;
    UCHAR MsrDumpData[1];
} WHEA_MSR_DUMP_SECTION, *PWHEA_MSR_DUMP_SECTION;

//------------------------------------------------------ MU_TELEMETRY_SECTION

typedef struct _MU_TELEMETRY_SECTION {
  GUID ComponentID;
  GUID SubComponentID;
  UINT32 Reserved;
  UINT32 ErrorStatusValue;
  UINT64 AdditionalInfo1;
  UINT64 AdditionalInfo2;
} MU_TELEMETRY_SECTION, *PMU_TELEMETRY_SECTION;

//------------------------------------------------------ WHEA_ARM_PROCESSOR_ERROR_SECTION

typedef union _WHEA_ARM_PROCESSOR_ERROR_SECTION_VALID_BITS {
    struct {
        ULONG MPIDR:1;
        ULONG AffinityLevel:1;
        ULONG RunningState:1;
        ULONG VendorSpecificInfo:1;
        ULONG Reserved:28;
    } DUMMYSTRUCTNAME;
    ULONG AsULONG;
} WHEA_ARM_PROCESSOR_ERROR_SECTION_VALID_BITS,
  *PWHEA_ARM_PROCESSOR_ERROR_SECTION_VALID_BITS;

typedef struct _WHEA_ARM_PROCESSOR_ERROR_SECTION {
    WHEA_ARM_PROCESSOR_ERROR_SECTION_VALID_BITS ValidBits;
    USHORT ErrorInformationStructures;
    USHORT ContextInformationStructures;
    ULONG SectionLength;
    UCHAR ErrorAffinityLevel;
    UCHAR Reserved[3];
    ULONGLONG MPIDR_EL1;
    ULONGLONG MIDR_EL1;
    ULONG RunningState;
    ULONG PSCIState;
    UCHAR Data[1];
} WHEA_ARM_PROCESSOR_ERROR_SECTION, *PWHEA_ARM_PROCESSOR_ERROR_SECTION;

CPER_FIELD_CHECK(WHEA_ARM_PROCESSOR_ERROR_SECTION, ValidBits,                    0,    4);
CPER_FIELD_CHECK(WHEA_ARM_PROCESSOR_ERROR_SECTION, ErrorInformationStructures,   4,    2);
CPER_FIELD_CHECK(WHEA_ARM_PROCESSOR_ERROR_SECTION, ContextInformationStructures, 6,    2);
CPER_FIELD_CHECK(WHEA_ARM_PROCESSOR_ERROR_SECTION, SectionLength,                8,    4);
CPER_FIELD_CHECK(WHEA_ARM_PROCESSOR_ERROR_SECTION, ErrorAffinityLevel,           12,   1);
CPER_FIELD_CHECK(WHEA_ARM_PROCESSOR_ERROR_SECTION, Reserved,                     13,   3);
CPER_FIELD_CHECK(WHEA_ARM_PROCESSOR_ERROR_SECTION, MPIDR_EL1,                    16,   8);
CPER_FIELD_CHECK(WHEA_ARM_PROCESSOR_ERROR_SECTION, MIDR_EL1,                     24,   8);
CPER_FIELD_CHECK(WHEA_ARM_PROCESSOR_ERROR_SECTION, RunningState,                 32,   4);
CPER_FIELD_CHECK(WHEA_ARM_PROCESSOR_ERROR_SECTION, PSCIState,                    36,   4);
CPER_FIELD_CHECK(WHEA_ARM_PROCESSOR_ERROR_SECTION, Data,                         40,   1);

//--------------------------------------------------------------- ERROR RECOVERY_INFO_SECTION

typedef enum _WHEA_RECOVERY_TYPE {
    WheaRecoveryTypeActionRequired = 1,
    WheaRecoveryTypeActionOptional,
    WheaRecoveryTypeMax
} WHEA_RECOVERY_TYPE, *PWHEA_RECOVERY_TYPE;

typedef union _WHEA_RECOVERY_ACTION {
    struct {
        UINT64 NoneAttempted : 1;
        UINT64 TerminateProcess : 1;
        UINT64 ForwardedToVm : 1;
        UINT64 MarkPageBad : 1;
        UINT64 PoisonNotPresent :1;
        UINT64 Reserved : 59;
    } DUMMYSTRUCTNAME;

    UINT64 AsUINT64;
} WHEA_RECOVERY_ACTION, *PWHEA_RECOVERY_ACTION;

typedef enum _WHEA_RECOVERY_FAILURE_REASON {
    WheaRecoveryFailureReasonKernelCouldNotMarkMemoryBad = 1,
    WheaRecoveryFailureReasonKernelMarkMemoryBadTimedOut,
    WheaRecoveryFailureReasonNoRecoveryContext,
    WheaRecoveryFailureReasonNotContinuable,
    WheaRecoveryFailureReasonPcc,
    WheaRecoveryFailureReasonOverflow,
    WheaRecoveryFailureReasonNotSupported,
    WheaRecoveryFailureReasonMiscOrAddrNotValid,
    WheaRecoveryFailureReasonInvalidAddressMode,
    WheaRecoveryFailureReasonHighIrql,
    WheaRecoveryFailureReasonInsufficientAltContextWrappers,
    WheaRecoveryFailureReasonInterruptsDisabled,
    WheaRecoveryFailureReasonSwapBusy,
    WheaRecoveryFailureReasonStackOverflow,
    WheaRecoveryFailureReasonUnexpectedFailure,
    WheaRecoveryFailureReasonKernelWillPageFaultBCAtCurrentIrql,
    WheaRecoveryFailureReasonFarNotValid,
    WheaRecoveryFailureReasonMax
} WHEA_RECOVERY_FAILURE_REASON, *PWHEA_RECOVERY_FAILURE_REASON;

typedef struct _WHEA_ERROR_RECOVERY_INFO_SECTION {
    BOOLEAN RecoveryKernel;
    WHEA_RECOVERY_ACTION RecoveryAction;
    WHEA_RECOVERY_TYPE RecoveryType;
    KIRQL Irql;
    BOOLEAN RecoverySucceeded;
    WHEA_RECOVERY_FAILURE_REASON FailureReason;
    CCHAR ProcessName[20];
} WHEA_ERROR_RECOVERY_INFO_SECTION, *PWHEA_ERROR_RECOVERY_INFO_SECTION;

//------------------------------------------------------ WHEA_ARM_PROCESSOR_ERROR_INFORMATION

typedef union _WHEA_ARM_PROCESSOR_ERROR_INFORMATION_VALID_BITS {
    struct {
        USHORT MultipleError:1;
        USHORT Flags:1;
        USHORT ErrorInformation:1;
        USHORT VirtualFaultAddress:1;
        USHORT PhysicalFaultAddress:1;
        USHORT Reserved:11;
    } DUMMYSTRUCTNAME;
    USHORT AsUSHORT;
} WHEA_ARM_PROCESSOR_ERROR_INFORMATION_VALID_BITS,
  *PWHEA_ARM_PROCESSOR_ERROR_INFORMATION_VALID_BITS;

typedef union _WHEA_ARM_CACHE_ERROR_VALID_BITS {
    struct {
        USHORT TransactionType:1;
        USHORT Operation:1;
        USHORT Level:1;
        USHORT ProcessorContextCorrupt:1;
        USHORT Corrected:1;
        USHORT PrecisePC:1;
        USHORT RestartablePC:1;
        USHORT Reserved:9;
    } DUMMYSTRUCTNAME;
    USHORT AsUSHORT;
} WHEA_ARM_CACHE_ERROR_VALID_BITS, *PWHEA_ARM_CACHE_ERROR_VALID_BITS;

typedef struct _WHEA_ARM_CACHE_ERROR {
    WHEA_ARM_CACHE_ERROR_VALID_BITS ValidationBit;
    UCHAR TransactionType:2;
    UCHAR Operation:4;
    UCHAR Level:3;
    UCHAR ProcessorContextCorrupt:1;
    UCHAR Corrected:1;
    UCHAR PrecisePC:1;
    UCHAR RestartablePC:1;
    ULONGLONG Reserved:35;
} WHEA_ARM_CACHE_ERROR, *PWHEA_ARM_CACHE_ERROR;

typedef union _WHEA_ARM_TLB_ERROR_VALID_BITS {
    struct {
        USHORT TransactionType:1;
        USHORT Operation:1;
        USHORT Level:1;
        USHORT ProcessorContextCorrupt:1;
        USHORT Corrected:1;
        USHORT PrecisePC:1;
        USHORT RestartablePC:1;
        USHORT Reserved:9;
    } DUMMYSTRUCTNAME;
    USHORT AsUSHORT;
} WHEA_ARM_TLB_ERROR_VALID_BITS, *PWHEA_ARM_TLB_ERROR_VALID_BITS;

typedef struct _WHEA_ARM_TLB_ERROR {
    WHEA_ARM_TLB_ERROR_VALID_BITS ValidationBit;
    UCHAR TransactionType:2;
    UCHAR Operation:4;
    UCHAR Level:3;
    UCHAR ProcessorContextCorrupt:1;
    UCHAR Corrected:1;
    UCHAR PrecisePC:1;
    UCHAR RestartablePC:1;
    ULONGLONG Reserved:36;
} WHEA_ARM_TLB_ERROR, *PWHEA_ARM_TLB_ERROR;

typedef union _WHEA_ARM_BUS_ERROR_VALID_BITS {
    struct {
        USHORT TransactionType:1;
        USHORT Operation:1;
        USHORT Level:1;
        USHORT ProcessorContextCorrupt:1;
        USHORT Corrected:1;
        USHORT PrecisePC:1;
        USHORT RestartablePC:1;
        USHORT ParticipationType:1;
        USHORT Timeout:1;
        USHORT AddressSpace:1;
        USHORT MemoryAttributes:1;
        USHORT AccessMode:1;
        USHORT Reserved:4;
    } DUMMYSTRUCTNAME;
    USHORT AsUSHORT;
} WHEA_ARM_BUS_ERROR_VALID_BITS, *PWHEA_ARM_BUS_ERROR_VALID_BITS;

typedef struct _WHEA_ARM_BUS_ERROR {
    WHEA_ARM_BUS_ERROR_VALID_BITS ValidationBit;
    UCHAR TransactionType:2;
    UCHAR Operation:4;
    UCHAR Level:3;
    UCHAR ProcessorContextCorrupt:1;
    UCHAR Corrected:1;
    UCHAR PrecisePC:1;
    UCHAR RestartablePC:1;
    UCHAR ParticipationType:2;
    UCHAR TimeOut:1;
    UCHAR AddressSpace:2;
    USHORT MemoryAccessAttributes:9;
    UCHAR AccessMode:1;
    ULONG Reserved:20;
} WHEA_ARM_BUS_ERROR, *PWHEA_ARM_BUS_ERROR;

typedef union _WHEA_ARM_PROCESSOR_ERROR {
    WHEA_ARM_CACHE_ERROR CacheError;
    WHEA_ARM_TLB_ERROR TlbError;
    WHEA_ARM_BUS_ERROR BusError;
    ULONGLONG AsULONGLONG;
} WHEA_ARM_PROCESSOR_ERROR, *PWHEA_ARM_PROCESSOR_ERROR;

typedef struct _WHEA_ARM_PROCESSOR_ERROR_INFORMATION {
    UCHAR Version;
    UCHAR Length;
    WHEA_ARM_PROCESSOR_ERROR_INFORMATION_VALID_BITS ValidationBit;
    UCHAR Type;
    USHORT MultipleError;
    UCHAR Flags;
    ULONGLONG ErrorInformation;
    ULONGLONG VirtualFaultAddress;
    ULONGLONG PhysicalFaultAddress;
} WHEA_ARM_PROCESSOR_ERROR_INFORMATION, *PWHEA_ARM_PROCESSOR_ERROR_INFORMATION;

CPER_FIELD_CHECK(WHEA_ARM_PROCESSOR_ERROR_INFORMATION, Version,                 0,   1);
CPER_FIELD_CHECK(WHEA_ARM_PROCESSOR_ERROR_INFORMATION, Length,                  1,   1);
CPER_FIELD_CHECK(WHEA_ARM_PROCESSOR_ERROR_INFORMATION, ValidationBit,           2,   2);
CPER_FIELD_CHECK(WHEA_ARM_PROCESSOR_ERROR_INFORMATION, Type,                    4,   1);
CPER_FIELD_CHECK(WHEA_ARM_PROCESSOR_ERROR_INFORMATION, MultipleError,           5,   2);
CPER_FIELD_CHECK(WHEA_ARM_PROCESSOR_ERROR_INFORMATION, Flags,                   7,   1);
CPER_FIELD_CHECK(WHEA_ARM_PROCESSOR_ERROR_INFORMATION, ErrorInformation,        8,   8);
CPER_FIELD_CHECK(WHEA_ARM_PROCESSOR_ERROR_INFORMATION, VirtualFaultAddress,    16,   8);
CPER_FIELD_CHECK(WHEA_ARM_PROCESSOR_ERROR_INFORMATION, PhysicalFaultAddress,  24,   8);

//------------------------------------------------------ WHEA_ARM_PROCESSOR_ERROR_CONTEXT_INFORMATION_HEADER

typedef union _WHEA_ARM_PROCESSOR_ERROR_CONTEXT_INFORMATION_HEADER_FLAGS {
    struct {
        ULONG ExceptionLevel:1;
        ULONG NonSecure:1;
        ULONG AArch64:1;
        ULONG Reserved:29;
    } DUMMYSTRUCTNAME;
    ULONG AsULONG;
} WHEA_ARM_PROCESSOR_ERROR_CONTEXT_INFORMATION_HEADER_FLAGS,
  *PWHEA_ARM_PROCESSOR_ERROR_CONTEXT_INFORMATION_HEADER_FLAGS;

typedef struct _WHEA_ARMV8_AARCH32_GPRS {
    ULONG R0;
    ULONG R1;
    ULONG R2;
    ULONG R3;
    ULONG R4;
    ULONG R5;
    ULONG R6;
    ULONG R7;
    ULONG R8;
    ULONG R9;
    ULONG R10;
    ULONG R11;
    ULONG R12;
    ULONG R13; // SP
    ULONG R14; // LR
    ULONG R15; // PC
} WHEA_ARMV8_AARCH32_GPRS, *PWHEA_ARMV8_AARCH32_GPRS;

typedef struct _WHEA_ARM_AARCH32_EL1_CSR {
    ULONG DFAR;
    ULONG DFSR;
    ULONG IFAR;
    ULONG ISR;
    ULONG MAIR0;
    ULONG MAIR1;
    ULONG MIDR;
    ULONG MPIDR;
    ULONG NMRR;
    ULONG PRRR;
    ULONG SCTLR; // NS
    ULONG SPSR;
    ULONG SPSR_abt;
    ULONG SPSR_fiq;
    ULONG SPSR_irq;
    ULONG SPSR_svc;
    ULONG SPSR_und;
    ULONG TPIDRPRW;
    ULONG TPIDRURO;
    ULONG TPIDRURW;
    ULONG TTBCR;
    ULONG TTBR0;
    ULONG TTBR1;
    ULONG DACR;
} WHEA_ARM_AARCH32_EL1_CSR, *PWHEA_ARM_AARCH32_EL1;

typedef struct _WHEA_ARM_AARCH32_EL2_CSR {
    ULONG ELR_hyp;
    ULONG HAMAIR0;
    ULONG HAMAIR1;
    ULONG HCR;
    ULONG HCR2;
    ULONG HDFAR;
    ULONG HIFAR;
    ULONG HPFAR;
    ULONG HSR;
    ULONG HTCR;
    ULONG HTPIDR;
    ULONG HTTBR;
    ULONG SPSR_hyp;
    ULONG VTCR;
    ULONG VTTBR;
    ULONG DACR32_EL2;
} WHEA_ARM_AARCH32_EL2_CSR, *PWHEA_ARM_AARCH32_EL2_CSR;

typedef struct _WHEA_ARM_AARCH32_SECURE_CSR {
    ULONG SCTLR;
    ULONG SPSR_mon;
} WHEA_ARM_AARCH32_SECURE_CSR, *PWHEA_ARM_AARCH32_SECURE_CSR;

typedef struct _WHEA_ARMV8_AARCH64_GPRS {
    ULONGLONG X0;
    ULONGLONG X1;
    ULONGLONG X2;
    ULONGLONG X3;
    ULONGLONG X4;
    ULONGLONG X5;
    ULONGLONG X6;
    ULONGLONG X7;
    ULONGLONG X8;
    ULONGLONG X9;
    ULONGLONG X10;
    ULONGLONG X11;
    ULONGLONG X12;
    ULONGLONG X13;
    ULONGLONG X14;
    ULONGLONG X15;
    ULONGLONG X16;
    ULONGLONG X17;
    ULONGLONG X18;
    ULONGLONG X19;
    ULONGLONG X20;
    ULONGLONG X21;
    ULONGLONG X22;
    ULONGLONG X23;
    ULONGLONG X24;
    ULONGLONG X25;
    ULONGLONG X26;
    ULONGLONG X27;
    ULONGLONG X28;
    ULONGLONG X29;
    ULONGLONG X30;
    ULONGLONG SP;
} WHEA_ARMV8_AARCH64_GPRS, *PWHEA_ARMV8_AARCH64_GPRS;

typedef struct _WHEA_ARM_AARCH64_EL1_CSR {
    ULONGLONG ELR_EL1;
    ULONGLONG ESR_EL2;
    ULONGLONG FAR_EL1;
    ULONGLONG ISR_EL1;
    ULONGLONG MAIR_EL1;
    ULONGLONG MIDR_EL1;
    ULONGLONG MPIDR_EL1;
    ULONGLONG SCTLR_EL1;
    ULONGLONG SP_EL0;
    ULONGLONG SP_EL1;
    ULONGLONG SPSR_EL1;
    ULONGLONG TCR_EL1;
    ULONGLONG TPIDR_EL0;
    ULONGLONG TPIDR_EL1;
    ULONGLONG TPIDRRO_EL0;
    ULONGLONG TTBR0_EL1;
    ULONGLONG TTBR1_EL1;
} WHEA_ARM_AARCH64_EL1_CSR, *PWHEA_ARM_AARCH64_EL1_CSR;

typedef struct _WHEA_ARM_AARCH64_EL2_CSR {
    ULONGLONG ELR_EL2;
    ULONGLONG ESR_EL2;
    ULONGLONG FAR_EL2;
    ULONGLONG HACR_EL2;
    ULONGLONG HCR_EL2;
    ULONGLONG HPFAR_EL2;
    ULONGLONG MAIR_EL2;
    ULONGLONG SCTLR_EL2;
    ULONGLONG SP_EL2;
    ULONGLONG SPSR_EL2;
    ULONGLONG TCR_EL2;
    ULONGLONG TPIDR_EL2;
    ULONGLONG TTBR0_EL2;
    ULONGLONG VTCR_EL2;
    ULONGLONG VTTBR_EL2;
} WHEA_ARM_AARCH64_EL2_CSR, *PWHEA_ARM_AARCH64_EL2_CSR;

typedef struct _WHEA_ARMV8_AARCH64_EL3_CSR {
    ULONGLONG ELR_EL3;
    ULONGLONG ESR_EL3;
    ULONGLONG FAR_EL3;
    ULONGLONG MAIR_EL3;
    ULONGLONG SCTLR_EL3;
    ULONGLONG SP_EL3;
    ULONGLONG SPSR_EL3;
    ULONGLONG TCR_EL3;
    ULONGLONG TPIDR_EL3;
    ULONGLONG TTBR0_EL3;
} WHEA_ARMV8_AARCH64_EL3_CSR, *PWHEA_ARMV8_AARCH64_EL3_CSR;

typedef struct _WHEA_ARM_MISC_CSR {
    USHORT MRSEncoding;
    ULONGLONG Value;
} WHEA_ARM_MISC_CSR, *PWHEA_ARM_MISC_CSR;

typedef struct _WHEA_ARM_PROCESSOR_ERROR_CONTEXT_INFORMATION_HEADER {
    USHORT Version;
    USHORT RegisterContextType;
    ULONG RegisterArraySize;
    UCHAR RegisterArray[1];
} WHEA_ARM_PROCESSOR_ERROR_CONTEXT_INFORMATION_HEADER,
  *PWHEA_ARM_PROCESSOR_ERROR_CONTEXT_INFORMATION_HEADER;

CPER_FIELD_CHECK(WHEA_ARM_PROCESSOR_ERROR_CONTEXT_INFORMATION_HEADER, Version,               0,    2);
CPER_FIELD_CHECK(WHEA_ARM_PROCESSOR_ERROR_CONTEXT_INFORMATION_HEADER, RegisterContextType,   2,    2);
CPER_FIELD_CHECK(WHEA_ARM_PROCESSOR_ERROR_CONTEXT_INFORMATION_HEADER, RegisterArraySize,     4,    4);
CPER_FIELD_CHECK(WHEA_ARM_PROCESSOR_ERROR_CONTEXT_INFORMATION_HEADER, RegisterArray,         8,    1);

// ----------------------------------------------------------------- SEA Section

typedef struct _WHEA_SEA_SECTION {
    ULONG Esr;
    ULONG64 Far;
    ULONG64 Par;
    BOOLEAN WasKernel;
} WHEA_SEA_SECTION, *PWHEA_SEA_SECTION;

typedef struct _WHEA_SEI_SECTION {
    ULONG Esr;
    ULONG64 Far;
} WHEA_SEI_SECTION, *PWHEA_SEI_SECTION;

// -------------------------------------------------------- Arm RAS Node Section

typedef struct _WHEA_ARM_RAS_NODE_SECTION {
    UINT32 NodeFieldCount;
    UINT32 NodeIndex;
    UINT8 InterfaceType;
    UINT8 AestNodeType;
    UINT8 Reserved[6];
    // Fields as defined in the Arm RAS extensions version 8.6  (ARM DDI 0587)
    UINT64 ErrFr;
    UINT64 ErrCtlr;
    UINT64 ErrStatus;
    UINT64 ErrAddr;
    UINT64 ErrMisc0;
    UINT64 ErrMisc1;
    UINT64 ErrMisc2;
    UINT64 ErrMisc3;
} WHEA_ARM_RAS_NODE_SECTION, *PWHEA_ARM_RAS_NODE_SECTION;

//
// To make ensure backwards compatability in the future the number of node field's
// is recorded. This allows expanding the section on new machines without breaking
// parsing for older generations.
//

#define WHEA_ARM_RAS_NODE_FIELD_COUNT 8

//
// Interface types possible for Arm Ras Nodes, as defined in the AEST ACPI table
// definition in "ACPI for the Armv8 RAS Extenstions"  (Document Number DEN0085)
//

typedef enum _WHEA_ARM_RAS_NODE_INTERFACES {
    WheaArmRasNodeInterfaceSystemRegister = 0,
    WheaArmRasNodeInterfaceMmio = 1
} WHEA_ARM_RAS_NODE_INTERFACES, *PWHEA_ARM_RAS_NODE_INTERFACES;

// -------------------------------------------------------- PCI Recovery Section

typedef enum _WHEA_PCI_RECOVERY_SIGNAL {
    WheaPciRecoverySignalUnknown = 0,
    WheaPciRecoverySignalAer,
    WheaPciRecoverySignalDpc
}WHEA_PCI_RECOVERY_SIGNAL, *PWHEA_PCI_RECOVERY_SIGNAL;

typedef enum _WHEA_PCI_RECOVERY_STATUS {
    WheaPciREcoveryStatusUnknown = 0,
    WheaPciRecoveryStatusNoError,
    WheaPciRecoveryStatusLinkDisableTimeout,
    WheaPciRecoveryStatusLinkEnableTimeout,
    WheaPciRecoveryStatusRpBusyTimeout,
    WheaPciRecoveryStatusComplexTree,
    WheaPciRecoveryStatusBusNotFound,
    WheaPciRecoveryStatusDeviceNotFound,
    WheaPciRecoveryStatusDdaAerNotRecoverable,
    WheaPciRecoveryStatusFailedRecovery,
}WHEA_PCI_RECOVERY_STATUS,  *PWHEA_PCI_RECOVERY_STATUS;

typedef struct _WHEA_PCI_RECOVERY_SECTION {
    UINT8 SignalType;
    BOOLEAN RecoveryAttempted;
    UINT8 RecoveryStatus;
} WHEA_PCI_RECOVERY_SECTION, *PWHEA_PCI_RECOVERY_SECTION;

#include <poppack.h>

#pragma warning(pop)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // #ifndef _CPER_H_

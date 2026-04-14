/*****************************************************************
*                                                                *
* mindumpdef.h -- Basic Windows Crash Dump File Definitions      *
*                                                                *
* Copyright (c) Microsoft Corporation. All rights reserved.      *
*                                                                *
******************************************************************/

#ifndef _MINDUMPDEF_
#define _MINDUMPDEF_

#if _MSC_VER > 1000
#pragma once
#endif

#ifndef MIDL_PASS
#if _MSC_VER >= 1200
#pragma warning(push)
#endif
#pragma warning( disable : 4200 ) // nonstandard extension used : zero-sized array in struct/union
#pragma warning( disable : 4201 ) // nonstandard extension used : nameless struct/union
#endif // MIDL_PASS


#include <minwindef.h>


#define DMP_PHYSICAL_MEMORY_BLOCK_SIZE_32   (700)
#define DMP_CONTEXT_RECORD_SIZE_32          (1200)
#define DMP_RESERVED_0_SIZE_32              (1760)
#define DMP_RESERVED_2_SIZE_32              (16)
#define DMP_RESERVED_3_SIZE_32              (56)

#define DMP_PHYSICAL_MEMORY_BLOCK_SIZE_64   (700)
#define DMP_CONTEXT_RECORD_SIZE_64          (3000)
#define DMP_RESERVED_0_SIZE_64              (4008)

#define DMP_HEADER_COMMENT_SIZE             (128)


typedef enum _DUMP_TYPES {
    DUMP_TYPE_INVALID           = -1,
    DUMP_TYPE_UNKNOWN           = 0,
    DUMP_TYPE_FULL              = 1,
    DUMP_TYPE_SUMMARY           = 2,
    DUMP_TYPE_HEADER            = 3,
    DUMP_TYPE_TRIAGE            = 4,
    DUMP_TYPE_BITMAP_FULL       = 5,
    DUMP_TYPE_BITMAP_KERNEL     = 6,
    DUMP_TYPE_AUTOMATIC         = 7
} DUMP_TYPE;


#define DUMP_SIGNATURE32   ('EGAP')
#define DUMP_VALID_DUMP32  ('PMUD')

#define DUMP_SIGNATURE64   ('EGAP')
#define DUMP_VALID_DUMP64  ('46UD')

#define DUMP_SUMMARY_SIGNATURE  ('PMDS')
#define DUMP_SUMMARY_VALID      ('PMUD')

#define DUMP_FULL_SIGNATURE ('PMDF')
#define DUMP_FULL_VALID     ('PMUD')

#define DUMP_SUMMARY_VALID_KERNEL_VA                     (1)
#define DUMP_SUMMARY_VALID_CURRENT_USER_VA               (2)


typedef struct _PHYSICAL_MEMORY_RUN32 {
    ULONG BasePage;
    ULONG PageCount;
} PHYSICAL_MEMORY_RUN32, *PPHYSICAL_MEMORY_RUN32;


typedef struct _PHYSICAL_MEMORY_DESCRIPTOR32 {
    ULONG NumberOfRuns;
    ULONG NumberOfPages;
    PHYSICAL_MEMORY_RUN32 Run[1];
} PHYSICAL_MEMORY_DESCRIPTOR32, *PPHYSICAL_MEMORY_DESCRIPTOR32;


typedef struct _PHYSICAL_MEMORY_RUN64 {
    ULONG64 BasePage;
    ULONG64 PageCount;
} PHYSICAL_MEMORY_RUN64, *PPHYSICAL_MEMORY_RUN64;


typedef struct _PHYSICAL_MEMORY_DESCRIPTOR64 {
    ULONG NumberOfRuns;
    ULONG64 NumberOfPages;
    PHYSICAL_MEMORY_RUN64 Run[1];
} PHYSICAL_MEMORY_DESCRIPTOR64, *PPHYSICAL_MEMORY_DESCRIPTOR64;


typedef union _DUMP_FILE_ATTRIBUTES {
    struct {
        ULONG HiberCrash: 1;
        ULONG DumpDevicePowerOff: 1;
        ULONG InsufficientDumpfileSize: 1;
        ULONG KernelGeneratedTriageDump: 1;
        ULONG LiveDumpGeneratedDump: 1;
        ULONG DumpIsGeneratedOffline: 1;
        ULONG FilterDumpFile: 1;
        ULONG EarlyBootCrash: 1;

        //
        // If below flag is set, it means Dump data (i.e. non-secureheader data) is encrypted, and Secure header is in use
        //

        ULONG EncryptedDumpData: 1;

        //
        // Below flag would be set by dump decryption software to indicate that the dump was originally encrypted and
        // current dump is obtained after decryption of the original dump data.
        //

        ULONG DecryptedDump: 1;

        ULONG ReservedFlags: 22;
    };

    ULONG Attributes;
} DUMP_FILE_ATTRIBUTES, *PDUMP_FILE_ATTRIBUTES;


typedef struct _DUMP_HEADER32 {
    ULONG Signature;
    ULONG ValidDump;
    ULONG MajorVersion;
    ULONG MinorVersion;
    ULONG DirectoryTableBase;
    ULONG PfnDataBase;
    ULONG PsLoadedModuleList;
    ULONG PsActiveProcessHead;
    ULONG MachineImageType;
    ULONG NumberProcessors;
    ULONG BugCheckCode;
    ULONG BugCheckParameter1;
    ULONG BugCheckParameter2;
    ULONG BugCheckParameter3;
    ULONG BugCheckParameter4;
    CHAR VersionUser[32];
    UCHAR PaeEnabled;               // Present only for Win2k and better
    UCHAR KdSecondaryVersion;       // Present only for W2K3 SP1 and better
    UCHAR Spare3[2];
    ULONG KdDebuggerDataBlock;      // Present only for Win2k SP1 and better.

    union {
        PHYSICAL_MEMORY_DESCRIPTOR32 PhysicalMemoryBlock;
        UCHAR PhysicalMemoryBlockBuffer [ DMP_PHYSICAL_MEMORY_BLOCK_SIZE_32 ];
    };
    UCHAR ContextRecord [ DMP_CONTEXT_RECORD_SIZE_32 ];
    EXCEPTION_RECORD32 Exception;
    CHAR Comment [ DMP_HEADER_COMMENT_SIZE ];   // May not be present.
    DUMP_FILE_ATTRIBUTES Attributes;
    ULONG BootId;
    UCHAR _reserved0 [ DMP_RESERVED_0_SIZE_32 ];
    ULONG DumpType;                             // Present for Win2k and better.
    ULONG MiniDumpFields;
    ULONG SecondaryDataState;
    ULONG ProductType;
    ULONG SuiteMask;
    ULONG WriterStatus;
    LARGE_INTEGER RequiredDumpSpace;            // Present for Win2k and better.
    UCHAR _reserved2 [ DMP_RESERVED_2_SIZE_32 ];
    LARGE_INTEGER SystemUpTime;                 // Present only for Whistler and better.
    LARGE_INTEGER SystemTime;                   // Present only for Win2k and better.
    UCHAR _reserved3 [ DMP_RESERVED_3_SIZE_32 ];
} DUMP_HEADER32, *PDUMP_HEADER32;


typedef struct _DUMP_HEADER64 {
    ULONG Signature;
    ULONG ValidDump;
    ULONG MajorVersion;
    ULONG MinorVersion;
    ULONG64 DirectoryTableBase;
    ULONG64 PfnDataBase;
    ULONG64 PsLoadedModuleList;
    ULONG64 PsActiveProcessHead;
    ULONG MachineImageType;
    ULONG NumberProcessors;
    ULONG BugCheckCode;
    ULONG64 BugCheckParameter1;
    ULONG64 BugCheckParameter2;
    ULONG64 BugCheckParameter3;
    ULONG64 BugCheckParameter4;
    CHAR VersionUser[32];
    ULONG64 KdDebuggerDataBlock;

    union {
        PHYSICAL_MEMORY_DESCRIPTOR64 PhysicalMemoryBlock;
        UCHAR PhysicalMemoryBlockBuffer [ DMP_PHYSICAL_MEMORY_BLOCK_SIZE_64 ];
    };
    UCHAR ContextRecord [ DMP_CONTEXT_RECORD_SIZE_64 ];
    EXCEPTION_RECORD64 Exception;
    ULONG DumpType;
    LARGE_INTEGER RequiredDumpSpace;
    LARGE_INTEGER SystemTime;
    CHAR Comment [ DMP_HEADER_COMMENT_SIZE ];   // May not be present.
    LARGE_INTEGER SystemUpTime;
    ULONG MiniDumpFields;
    ULONG SecondaryDataState;
    ULONG ProductType;
    ULONG SuiteMask;
    ULONG WriterStatus;
    UCHAR Unused1;
    UCHAR KdSecondaryVersion;       // Present only for W2K3 SP1 and better
    UCHAR Unused[2];
    DUMP_FILE_ATTRIBUTES Attributes;
    ULONG BootId;
    UCHAR _reserved0[ DMP_RESERVED_0_SIZE_64 ];
} DUMP_HEADER64, *PDUMP_HEADER64;

#ifndef MIDL_PASS
#if _MSC_VER >= 1200
#pragma warning(pop)
#else
#pragma warning( default : 4200 ) // nonstandard extension used : zero-sized array in struct/union
#endif
#endif // MIDL_PASS

#endif // _MINDUMPDEF_

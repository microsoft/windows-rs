// Copyright (C) Microsoft Corporation. All rights reserved.

#if defined(_MSC_VER)
#pragma once
#endif

#ifndef _MINIDUMP_H
#define _MINIDUMP_H

#ifdef _CONTRACT_GEN
#include <nt.h>
#include <ntrtl.h>
#include <nturtl.h>
#include <windows.h>
#endif
#include <apiset.h>
#include <apisetcconv.h>

#pragma region Desktop Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES)

// HEXTRACT: hide_line begin_dbghelp begin_imagehlp

#include <pshpack4.h>

#if defined(_MSC_VER)
#if _MSC_VER >= 800
#if _MSC_VER >= 1200
#pragma warning(push)
#endif
#pragma warning(disable:4200)    /* Zero length array */
#pragma warning(disable:4201)    /* Nameless struct/union */
#endif
#endif

#ifdef __cplusplus
extern "C" {
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#define MINIDUMP_SIGNATURE ('PMDM')
#define MINIDUMP_VERSION   (42899)
typedef DWORD RVA;
typedef ULONG64 RVA64;

typedef struct _MINIDUMP_LOCATION_DESCRIPTOR {
    ULONG32 DataSize;
    RVA Rva;
} MINIDUMP_LOCATION_DESCRIPTOR;

typedef struct _MINIDUMP_LOCATION_DESCRIPTOR64 {
    ULONG64 DataSize;
    RVA64 Rva;
} MINIDUMP_LOCATION_DESCRIPTOR64;

typedef struct _MINIDUMP_MEMORY_DESCRIPTOR {
    ULONG64 StartOfMemoryRange;
    MINIDUMP_LOCATION_DESCRIPTOR Memory;
} MINIDUMP_MEMORY_DESCRIPTOR, *PMINIDUMP_MEMORY_DESCRIPTOR;

// DESCRIPTOR64 is used for full-memory minidumps where
// all of the raw memory is laid out sequentially at the
// end of the dump.  There is no need for individual RVAs
// as the RVA is the base RVA plus the sum of the preceeding
// data blocks.
typedef struct _MINIDUMP_MEMORY_DESCRIPTOR64 {
    ULONG64 StartOfMemoryRange;
    ULONG64 DataSize;
} MINIDUMP_MEMORY_DESCRIPTOR64, *PMINIDUMP_MEMORY_DESCRIPTOR64;

typedef struct _MINIDUMP_HEADER {
    ULONG32 Signature;
    ULONG32 Version;
    ULONG32 NumberOfStreams;
    RVA StreamDirectoryRva;
    ULONG32 CheckSum;
    union {
        ULONG32 Reserved;
        ULONG32 TimeDateStamp;
    };
    ULONG64 Flags;
} MINIDUMP_HEADER, *PMINIDUMP_HEADER;

//
// The MINIDUMP_HEADER field StreamDirectoryRva points to
// an array of MINIDUMP_DIRECTORY structures.
//

typedef struct _MINIDUMP_DIRECTORY {
    ULONG32 StreamType;
    MINIDUMP_LOCATION_DESCRIPTOR Location;
} MINIDUMP_DIRECTORY, *PMINIDUMP_DIRECTORY;

typedef struct _MINIDUMP_STRING {
    ULONG32 Length;         // Length in bytes of the string
    WCHAR   Buffer [0];     // Variable size buffer
} MINIDUMP_STRING, *PMINIDUMP_STRING;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

//
// The MINIDUMP_DIRECTORY field StreamType may be one of the following types.
// Types will be added in the future, so if a program reading the minidump
// header encounters a stream type it does not understand it should ignore
// the data altogether. Any tag above LastReservedStream will not be used by
// the system and is reserved for program-specific information.
//

typedef enum _MINIDUMP_STREAM_TYPE {

    UnusedStream                = 0,
    ReservedStream0             = 1,
    ReservedStream1             = 2,
    ThreadListStream            = 3,
    ModuleListStream            = 4,
    MemoryListStream            = 5,
    ExceptionStream             = 6,
    SystemInfoStream            = 7,
    ThreadExListStream          = 8,
    Memory64ListStream          = 9,
    CommentStreamA              = 10,
    CommentStreamW              = 11,
    HandleDataStream            = 12,
    FunctionTableStream         = 13,
    UnloadedModuleListStream    = 14,
    MiscInfoStream              = 15,
    MemoryInfoListStream        = 16,
    ThreadInfoListStream        = 17,
    HandleOperationListStream   = 18,
    TokenStream                 = 19,
    JavaScriptDataStream        = 20,
    SystemMemoryInfoStream      = 21,
    ProcessVmCountersStream     = 22,
    IptTraceStream              = 23,
    ThreadNamesStream           = 24,

    ceStreamNull                = 0x8000,
    ceStreamSystemInfo          = 0x8001,
    ceStreamException           = 0x8002,
    ceStreamModuleList          = 0x8003,
    ceStreamProcessList         = 0x8004,
    ceStreamThreadList          = 0x8005,
    ceStreamThreadContextList   = 0x8006,
    ceStreamThreadCallStackList = 0x8007,
    ceStreamMemoryVirtualList   = 0x8008,
    ceStreamMemoryPhysicalList  = 0x8009,
    ceStreamBucketParameters    = 0x800A,
    ceStreamProcessModuleMap    = 0x800B,
    ceStreamDiagnosisList       = 0x800C,

    LastReservedStream          = 0xffff

} MINIDUMP_STREAM_TYPE;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

//
// The minidump system information contains processor and
// Operating System specific information.
//

//
// CPU information is obtained from one of two places.
//
//  1) On x86 computers, CPU_INFORMATION is obtained from the CPUID
//     instruction. You must use the X86 portion of the union for X86
//     computers.
//
//  2) On non-x86 architectures, CPU_INFORMATION is obtained by calling
//     IsProcessorFeatureSupported().
//

typedef union _CPU_INFORMATION {

    //
    // X86 platforms use CPUID function to obtain processor information.
    //

    struct {

        //
        // CPUID Subfunction 0, register EAX (VendorId [0]),
        // EBX (VendorId [1]) and ECX (VendorId [2]).
        //

        ULONG32 VendorId [ 3 ];

        //
        // CPUID Subfunction 1, register EAX
        //

        ULONG32 VersionInformation;

        //
        // CPUID Subfunction 1, register EDX
        //

        ULONG32 FeatureInformation;

        //
        // CPUID, Subfunction 80000001, register EBX. This will only
        // be obtained if the vendor id is "AuthenticAMD".
        //

        ULONG32 AMDExtendedCpuFeatures;

    } X86CpuInfo;

    //
    // Non-x86 platforms use processor feature flags.
    //

    struct {

        ULONG64 ProcessorFeatures [ 2 ];

    } OtherCpuInfo;

} CPU_INFORMATION, *PCPU_INFORMATION;

typedef struct _MINIDUMP_SYSTEM_INFO {

    //
    // ProcessorArchitecture, ProcessorLevel and ProcessorRevision are all
    // taken from the SYSTEM_INFO structure obtained by GetSystemInfo( ).
    //

    USHORT ProcessorArchitecture;
    USHORT ProcessorLevel;
    USHORT ProcessorRevision;

    union {
        USHORT Reserved0;
        struct {
            UCHAR NumberOfProcessors;
            UCHAR ProductType;
        };
    };

    //
    // MajorVersion, MinorVersion, BuildNumber, PlatformId and
    // CSDVersion are all taken from the OSVERSIONINFO structure
    // returned by GetVersionEx( ).
    //

    ULONG32 MajorVersion;
    ULONG32 MinorVersion;
    ULONG32 BuildNumber;
    ULONG32 PlatformId;

    //
    // RVA to a CSDVersion string in the string table.
    //

    RVA CSDVersionRva;

    union {
        ULONG32 Reserved1;
        struct {
            USHORT SuiteMask;
            USHORT Reserved2;
        };
    };

    CPU_INFORMATION Cpu;

} MINIDUMP_SYSTEM_INFO, *PMINIDUMP_SYSTEM_INFO;

//
// The minidump thread contains standard thread
// information plus an RVA to the memory for this
// thread and an RVA to the CONTEXT structure for
// this thread.
//

//
// ThreadId must be 4 bytes on all architectures.
//

C_ASSERT (sizeof ( ((PPROCESS_INFORMATION)0)->dwThreadId ) == 4);

typedef struct _MINIDUMP_THREAD {
    ULONG32 ThreadId;
    ULONG32 SuspendCount;
    ULONG32 PriorityClass;
    ULONG32 Priority;
    ULONG64 Teb;
    MINIDUMP_MEMORY_DESCRIPTOR Stack;
    MINIDUMP_LOCATION_DESCRIPTOR ThreadContext;
} MINIDUMP_THREAD, *PMINIDUMP_THREAD;

//
// The thread list is a container of threads.
//

typedef struct _MINIDUMP_THREAD_LIST {
    ULONG32 NumberOfThreads;
    MINIDUMP_THREAD Threads [0];
} MINIDUMP_THREAD_LIST, *PMINIDUMP_THREAD_LIST;

typedef struct _MINIDUMP_THREAD_EX {
    ULONG32 ThreadId;
    ULONG32 SuspendCount;
    ULONG32 PriorityClass;
    ULONG32 Priority;
    ULONG64 Teb;
    MINIDUMP_MEMORY_DESCRIPTOR Stack;
    MINIDUMP_LOCATION_DESCRIPTOR ThreadContext;
    MINIDUMP_MEMORY_DESCRIPTOR BackingStore;
} MINIDUMP_THREAD_EX, *PMINIDUMP_THREAD_EX;

//
// The thread list is a container of threads.
//

typedef struct _MINIDUMP_THREAD_EX_LIST {
    ULONG32 NumberOfThreads;
    MINIDUMP_THREAD_EX Threads [0];
} MINIDUMP_THREAD_EX_LIST, *PMINIDUMP_THREAD_EX_LIST;

//
// The MINIDUMP_EXCEPTION is the same as EXCEPTION on Win64.
//

typedef struct _MINIDUMP_EXCEPTION  {
    ULONG32 ExceptionCode;
    ULONG32 ExceptionFlags;
    ULONG64 ExceptionRecord;
    ULONG64 ExceptionAddress;
    ULONG32 NumberParameters;
    ULONG32 __unusedAlignment;
    ULONG64 ExceptionInformation [ EXCEPTION_MAXIMUM_PARAMETERS ];
} MINIDUMP_EXCEPTION, *PMINIDUMP_EXCEPTION;

//
// The exception information stream contains the id of the thread that caused
// the exception (ThreadId), the exception record for the exception
// (ExceptionRecord) and an RVA to the thread context where the exception
// occured.
//

typedef struct MINIDUMP_EXCEPTION_STREAM {
    ULONG32 ThreadId;
    ULONG32  __alignment;
    MINIDUMP_EXCEPTION ExceptionRecord;
    MINIDUMP_LOCATION_DESCRIPTOR ThreadContext;
} MINIDUMP_EXCEPTION_STREAM, *PMINIDUMP_EXCEPTION_STREAM;

//
// The MINIDUMP_MODULE contains information about a
// a specific module. It includes the CheckSum and
// the TimeDateStamp for the module so the module
// can be reloaded during the analysis phase.
//

typedef struct _MINIDUMP_MODULE {
    ULONG64 BaseOfImage;
    ULONG32 SizeOfImage;
    ULONG32 CheckSum;
    ULONG32 TimeDateStamp;
    RVA ModuleNameRva;
    VS_FIXEDFILEINFO VersionInfo;
    MINIDUMP_LOCATION_DESCRIPTOR CvRecord;
    MINIDUMP_LOCATION_DESCRIPTOR MiscRecord;
    ULONG64 Reserved0;                          // Reserved for future use.
    ULONG64 Reserved1;                          // Reserved for future use.
} MINIDUMP_MODULE, *PMINIDUMP_MODULE;

//
// The minidump module list is a container for modules.
//

typedef struct _MINIDUMP_MODULE_LIST {
    ULONG32 NumberOfModules;
    MINIDUMP_MODULE Modules [ 0 ];
} MINIDUMP_MODULE_LIST, *PMINIDUMP_MODULE_LIST;

//
// Memory Ranges
//

typedef struct _MINIDUMP_MEMORY_LIST {
    ULONG32 NumberOfMemoryRanges;
    MINIDUMP_MEMORY_DESCRIPTOR MemoryRanges [0];
} MINIDUMP_MEMORY_LIST, *PMINIDUMP_MEMORY_LIST;

typedef struct _MINIDUMP_MEMORY64_LIST {
    ULONG64 NumberOfMemoryRanges;
    RVA64 BaseRva;
    MINIDUMP_MEMORY_DESCRIPTOR64 MemoryRanges [0];
} MINIDUMP_MEMORY64_LIST, *PMINIDUMP_MEMORY64_LIST;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#pragma region Desktop Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES)

//
// Support for user supplied exception information.
//

typedef struct _MINIDUMP_EXCEPTION_INFORMATION {
    DWORD ThreadId;
    PEXCEPTION_POINTERS ExceptionPointers;
    BOOL ClientPointers;
} MINIDUMP_EXCEPTION_INFORMATION, *PMINIDUMP_EXCEPTION_INFORMATION;

typedef struct _MINIDUMP_EXCEPTION_INFORMATION64 {
    DWORD ThreadId;
    ULONG64 ExceptionRecord;
    ULONG64 ContextRecord;
    BOOL ClientPointers;
} MINIDUMP_EXCEPTION_INFORMATION64, *PMINIDUMP_EXCEPTION_INFORMATION64;

//
// Support for capturing system handle state at the time of the dump.
//

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

// Per-handle object information varies according to
// the OS, the OS version, the processor type and
// so on.  The minidump gives a minidump identifier
// to each possible data format for identification
// purposes but does not control nor describe the actual data.
typedef enum _MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE {
    MiniHandleObjectInformationNone,
    MiniThreadInformation1,
    MiniMutantInformation1,
    MiniMutantInformation2,
    MiniProcessInformation1,
    MiniProcessInformation2,
    MiniEventInformation1,
    MiniSectionInformation1,
    MiniSemaphoreInformation1,
    MiniHandleObjectInformationTypeMax
} MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE;

typedef struct _MINIDUMP_HANDLE_OBJECT_INFORMATION {
    RVA NextInfoRva;
    ULONG32 InfoType;
    ULONG32 SizeOfInfo;
    // Raw information follows.
} MINIDUMP_HANDLE_OBJECT_INFORMATION;

typedef struct _MINIDUMP_HANDLE_DESCRIPTOR {
    ULONG64 Handle;
    RVA TypeNameRva;
    RVA ObjectNameRva;
    ULONG32 Attributes;
    ULONG32 GrantedAccess;
    ULONG32 HandleCount;
    ULONG32 PointerCount;
} MINIDUMP_HANDLE_DESCRIPTOR, *PMINIDUMP_HANDLE_DESCRIPTOR;

typedef struct _MINIDUMP_HANDLE_DESCRIPTOR_2 {
    ULONG64 Handle;
    RVA TypeNameRva;
    RVA ObjectNameRva;
    ULONG32 Attributes;
    ULONG32 GrantedAccess;
    ULONG32 HandleCount;
    ULONG32 PointerCount;
    RVA ObjectInfoRva;
    ULONG32 Reserved0;
} MINIDUMP_HANDLE_DESCRIPTOR_2, *PMINIDUMP_HANDLE_DESCRIPTOR_2;

// The latest MINIDUMP_HANDLE_DESCRIPTOR definition.
typedef MINIDUMP_HANDLE_DESCRIPTOR_2 MINIDUMP_HANDLE_DESCRIPTOR_N;
typedef MINIDUMP_HANDLE_DESCRIPTOR_N *PMINIDUMP_HANDLE_DESCRIPTOR_N;

typedef struct _MINIDUMP_HANDLE_DATA_STREAM {
    ULONG32 SizeOfHeader;
    ULONG32 SizeOfDescriptor;
    ULONG32 NumberOfDescriptors;
    ULONG32 Reserved;
} MINIDUMP_HANDLE_DATA_STREAM, *PMINIDUMP_HANDLE_DATA_STREAM;

// Some operating systems can track the last operations
// performed on a handle.  For example, Application Verifier
// can enable this for some versions of Windows.  The
// handle operation list collects handle operations
// known for the dump target.
// Each entry is an AVRF_HANDLE_OPERATION.
typedef struct _MINIDUMP_HANDLE_OPERATION_LIST {
    ULONG32 SizeOfHeader;
    ULONG32 SizeOfEntry;
    ULONG32 NumberOfEntries;
    ULONG32 Reserved;
} MINIDUMP_HANDLE_OPERATION_LIST, *PMINIDUMP_HANDLE_OPERATION_LIST;

//
// Support for capturing dynamic function table state at the time of the dump.
//

typedef struct _MINIDUMP_FUNCTION_TABLE_DESCRIPTOR {
    ULONG64 MinimumAddress;
    ULONG64 MaximumAddress;
    ULONG64 BaseAddress;
    ULONG32 EntryCount;
    ULONG32 SizeOfAlignPad;
} MINIDUMP_FUNCTION_TABLE_DESCRIPTOR, *PMINIDUMP_FUNCTION_TABLE_DESCRIPTOR;

typedef struct _MINIDUMP_FUNCTION_TABLE_STREAM {
    ULONG32 SizeOfHeader;
    ULONG32 SizeOfDescriptor;
    ULONG32 SizeOfNativeDescriptor;
    ULONG32 SizeOfFunctionEntry;
    ULONG32 NumberOfDescriptors;
    ULONG32 SizeOfAlignPad;
} MINIDUMP_FUNCTION_TABLE_STREAM, *PMINIDUMP_FUNCTION_TABLE_STREAM;

//
// The MINIDUMP_UNLOADED_MODULE contains information about a
// a specific module that was previously loaded but no
// longer is.  This can help with diagnosing problems where
// callers attempt to call code that is no longer loaded.
//

typedef struct _MINIDUMP_UNLOADED_MODULE {
    ULONG64 BaseOfImage;
    ULONG32 SizeOfImage;
    ULONG32 CheckSum;
    ULONG32 TimeDateStamp;
    RVA ModuleNameRva;
} MINIDUMP_UNLOADED_MODULE, *PMINIDUMP_UNLOADED_MODULE;

//
// The minidump unloaded module list is a container for unloaded modules.
//

typedef struct _MINIDUMP_UNLOADED_MODULE_LIST {
    ULONG32 SizeOfHeader;
    ULONG32 SizeOfEntry;
    ULONG32 NumberOfEntries;
} MINIDUMP_UNLOADED_MODULE_LIST, *PMINIDUMP_UNLOADED_MODULE_LIST;

//
// The XSTATE_CONFIG_FEATURE_MSC_INFO struct is used to determine if the x86/amd64 machine
// supports a specific XState feature.
// Also, it is used to locate the XState context feature (AVX) in the context extended buffer.
//

typedef struct _XSTATE_CONFIG_FEATURE_MSC_INFO
{
    ULONG32 SizeOfInfo;
    ULONG32 ContextSize;
    ULONG64 EnabledFeatures;
    XSTATE_FEATURE Features[MAXIMUM_XSTATE_FEATURES];
} XSTATE_CONFIG_FEATURE_MSC_INFO, *PXSTATE_CONFIG_FEATURE_MSC_INFO;

//
// The miscellaneous information stream contains a variety
// of small pieces of information.  A member is valid if
// it's within the available size and its corresponding
// bit is set.
//

#define MINIDUMP_MISC1_PROCESS_ID            0x00000001
#define MINIDUMP_MISC1_PROCESS_TIMES         0x00000002
#define MINIDUMP_MISC1_PROCESSOR_POWER_INFO  0x00000004
#define MINIDUMP_MISC3_PROCESS_INTEGRITY     0x00000010
#define MINIDUMP_MISC3_PROCESS_EXECUTE_FLAGS 0x00000020
#define MINIDUMP_MISC3_TIMEZONE              0x00000040
#define MINIDUMP_MISC3_PROTECTED_PROCESS     0x00000080
#define MINIDUMP_MISC4_BUILDSTRING           0x00000100
#define MINIDUMP_MISC5_PROCESS_COOKIE        0x00000200

typedef struct _MINIDUMP_MISC_INFO {
    ULONG32 SizeOfInfo;
    ULONG32 Flags1;
    ULONG32 ProcessId;
    ULONG32 ProcessCreateTime;
    ULONG32 ProcessUserTime;
    ULONG32 ProcessKernelTime;
} MINIDUMP_MISC_INFO, *PMINIDUMP_MISC_INFO;

typedef struct _MINIDUMP_MISC_INFO_2 {
    ULONG32 SizeOfInfo;
    ULONG32 Flags1;
    ULONG32 ProcessId;
    ULONG32 ProcessCreateTime;
    ULONG32 ProcessUserTime;
    ULONG32 ProcessKernelTime;
    ULONG32 ProcessorMaxMhz;
    ULONG32 ProcessorCurrentMhz;
    ULONG32 ProcessorMhzLimit;
    ULONG32 ProcessorMaxIdleState;
    ULONG32 ProcessorCurrentIdleState;
} MINIDUMP_MISC_INFO_2, *PMINIDUMP_MISC_INFO_2;

typedef struct _MINIDUMP_MISC_INFO_3 {
    ULONG32 SizeOfInfo;
    ULONG32 Flags1;
    ULONG32 ProcessId;
    ULONG32 ProcessCreateTime;
    ULONG32 ProcessUserTime;
    ULONG32 ProcessKernelTime;
    ULONG32 ProcessorMaxMhz;
    ULONG32 ProcessorCurrentMhz;
    ULONG32 ProcessorMhzLimit;
    ULONG32 ProcessorMaxIdleState;
    ULONG32 ProcessorCurrentIdleState;
    ULONG32 ProcessIntegrityLevel;
    ULONG32 ProcessExecuteFlags;
    ULONG32 ProtectedProcess;
    ULONG32 TimeZoneId;
    TIME_ZONE_INFORMATION TimeZone;
} MINIDUMP_MISC_INFO_3, *PMINIDUMP_MISC_INFO_3;

typedef struct _MINIDUMP_MISC_INFO_4 {
    ULONG32 SizeOfInfo;
    ULONG32 Flags1;
    ULONG32 ProcessId;
    ULONG32 ProcessCreateTime;
    ULONG32 ProcessUserTime;
    ULONG32 ProcessKernelTime;
    ULONG32 ProcessorMaxMhz;
    ULONG32 ProcessorCurrentMhz;
    ULONG32 ProcessorMhzLimit;
    ULONG32 ProcessorMaxIdleState;
    ULONG32 ProcessorCurrentIdleState;
    ULONG32 ProcessIntegrityLevel;
    ULONG32 ProcessExecuteFlags;
    ULONG32 ProtectedProcess;
    ULONG32 TimeZoneId;
    TIME_ZONE_INFORMATION TimeZone;
    WCHAR   BuildString[MAX_PATH];
    WCHAR   DbgBldStr[40];
} MINIDUMP_MISC_INFO_4, *PMINIDUMP_MISC_INFO_4;

typedef struct _MINIDUMP_MISC_INFO_5 {
    ULONG32 SizeOfInfo;
    ULONG32 Flags1;
    ULONG32 ProcessId;
    ULONG32 ProcessCreateTime;
    ULONG32 ProcessUserTime;
    ULONG32 ProcessKernelTime;
    ULONG32 ProcessorMaxMhz;
    ULONG32 ProcessorCurrentMhz;
    ULONG32 ProcessorMhzLimit;
    ULONG32 ProcessorMaxIdleState;
    ULONG32 ProcessorCurrentIdleState;
    ULONG32 ProcessIntegrityLevel;
    ULONG32 ProcessExecuteFlags;
    ULONG32 ProtectedProcess;
    ULONG32 TimeZoneId;
    TIME_ZONE_INFORMATION TimeZone;
    WCHAR   BuildString[MAX_PATH];
    WCHAR   DbgBldStr[40];
    XSTATE_CONFIG_FEATURE_MSC_INFO XStateData;
    ULONG32 ProcessCookie;
} MINIDUMP_MISC_INFO_5, *PMINIDUMP_MISC_INFO_5;

// The latest MINIDUMP_MISC_INFO definition.
typedef MINIDUMP_MISC_INFO_5 MINIDUMP_MISC_INFO_N;
typedef MINIDUMP_MISC_INFO_N* PMINIDUMP_MISC_INFO_N;

//
// The memory information stream contains memory region
// description information.  This stream corresponds to
// what VirtualQuery would return for the process the
// dump was created for.
//

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#pragma region Desktop Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES)

typedef struct _MINIDUMP_MEMORY_INFO {
    ULONG64 BaseAddress;
    ULONG64 AllocationBase;
    ULONG32 AllocationProtect;
    ULONG32 __alignment1;
    ULONG64 RegionSize;
    ULONG32 State;
    ULONG32 Protect;
    ULONG32 Type;
    ULONG32 __alignment2;
} MINIDUMP_MEMORY_INFO, *PMINIDUMP_MEMORY_INFO;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

typedef struct _MINIDUMP_MEMORY_INFO_LIST {
    ULONG SizeOfHeader;
    ULONG SizeOfEntry;
    ULONG64 NumberOfEntries;
} MINIDUMP_MEMORY_INFO_LIST, *PMINIDUMP_MEMORY_INFO_LIST;

//
// The thread names stream in a minidump, containing information
// about each thread's name/description (if available).
//

typedef struct _MINIDUMP_THREAD_NAME {
    ULONG ThreadId;
    RVA64 RvaOfThreadName;
} MINIDUMP_THREAD_NAME, *PMINIDUMP_THREAD_NAME;

typedef struct _MINIDUMP_THREAD_NAME_LIST {
    ULONG NumberOfThreadNames;
    MINIDUMP_THREAD_NAME ThreadNames[0]; // Variable size buffer
} MINIDUMP_THREAD_NAME_LIST, *PMINIDUMP_THREAD_NAME_LIST;

//
// The memory information stream contains memory region
// description information.  This stream corresponds to
// what VirtualQuery would return for the process the
// dump was created for.
//

// Thread dump writer status flags.
#define MINIDUMP_THREAD_INFO_ERROR_THREAD    0x00000001
#define MINIDUMP_THREAD_INFO_WRITING_THREAD  0x00000002
#define MINIDUMP_THREAD_INFO_EXITED_THREAD   0x00000004
#define MINIDUMP_THREAD_INFO_INVALID_INFO    0x00000008
#define MINIDUMP_THREAD_INFO_INVALID_CONTEXT 0x00000010
#define MINIDUMP_THREAD_INFO_INVALID_TEB     0x00000020

typedef struct _MINIDUMP_THREAD_INFO {
    ULONG32 ThreadId;
    ULONG32 DumpFlags;
    ULONG32 DumpError;
    ULONG32 ExitStatus;
    ULONG64 CreateTime;
    ULONG64 ExitTime;
    ULONG64 KernelTime;
    ULONG64 UserTime;
    ULONG64 StartAddress;
    ULONG64 Affinity;
} MINIDUMP_THREAD_INFO, *PMINIDUMP_THREAD_INFO;

typedef struct _MINIDUMP_THREAD_INFO_LIST {
    ULONG SizeOfHeader;
    ULONG SizeOfEntry;
    ULONG NumberOfEntries;
} MINIDUMP_THREAD_INFO_LIST, *PMINIDUMP_THREAD_INFO_LIST;

//
// Support for token information.
//
typedef struct _MINIDUMP_TOKEN_INFO_HEADER {
    ULONG   TokenSize;   // The size of the token structure.
    ULONG   TokenId;     // The PID in NtOpenProcessToken() call or TID in NtOpenThreadToken() call.
    ULONG64 TokenHandle; // The handle value returned.
} MINIDUMP_TOKEN_INFO_HEADER, *PMINIDUMP_TOKEN_INFO_HEADER;

typedef struct _MINIDUMP_TOKEN_INFO_LIST {
    ULONG TokenListSize;
    ULONG TokenListEntries;
    ULONG ListHeaderSize;
    ULONG ElementHeaderSize;
} MINIDUMP_TOKEN_INFO_LIST, *PMINIDUMP_TOKEN_INFO_LIST;

//
// Support for global system memory/performance information.
//
typedef struct _MINIDUMP_SYSTEM_BASIC_INFORMATION {
    ULONG TimerResolution;
    ULONG PageSize;
    ULONG NumberOfPhysicalPages;
    ULONG LowestPhysicalPageNumber;
    ULONG HighestPhysicalPageNumber;
    ULONG AllocationGranularity;
    ULONG64 MinimumUserModeAddress;
    ULONG64 MaximumUserModeAddress;
    ULONG64 ActiveProcessorsAffinityMask;
    ULONG NumberOfProcessors;
} MINIDUMP_SYSTEM_BASIC_INFORMATION, *PMINIDUMP_SYSTEM_BASIC_INFORMATION;

typedef struct _MINIDUMP_SYSTEM_FILECACHE_INFORMATION {
    ULONG64 CurrentSize;
    ULONG64 PeakSize;
    ULONG PageFaultCount;
    ULONG64 MinimumWorkingSet;
    ULONG64 MaximumWorkingSet;
    ULONG64 CurrentSizeIncludingTransitionInPages;
    ULONG64 PeakSizeIncludingTransitionInPages;
    ULONG TransitionRePurposeCount;
    ULONG Flags;
} MINIDUMP_SYSTEM_FILECACHE_INFORMATION, *PMINIDUMP_SYSTEM_FILECACHE_INFORMATION;

typedef struct _MINIDUMP_SYSTEM_BASIC_PERFORMANCE_INFORMATION {
    ULONG64 AvailablePages;
    ULONG64 CommittedPages;
    ULONG64 CommitLimit;
    ULONG64 PeakCommitment;
} MINIDUMP_SYSTEM_BASIC_PERFORMANCE_INFORMATION, *PMINIDUMP_SYSTEM_BASIC_PERFORMANCE_INFORMATION;

typedef struct _MINIDUMP_SYSTEM_PERFORMANCE_INFORMATION {
    ULONG64 IdleProcessTime;
    ULONG64 IoReadTransferCount;
    ULONG64 IoWriteTransferCount;
    ULONG64 IoOtherTransferCount;
    ULONG IoReadOperationCount;
    ULONG IoWriteOperationCount;
    ULONG IoOtherOperationCount;
    ULONG AvailablePages;
    ULONG CommittedPages;
    ULONG CommitLimit;
    ULONG PeakCommitment;
    ULONG PageFaultCount;
    ULONG CopyOnWriteCount;
    ULONG TransitionCount;
    ULONG CacheTransitionCount;
    ULONG DemandZeroCount;
    ULONG PageReadCount;
    ULONG PageReadIoCount;
    ULONG CacheReadCount;
    ULONG CacheIoCount;
    ULONG DirtyPagesWriteCount;
    ULONG DirtyWriteIoCount;
    ULONG MappedPagesWriteCount;
    ULONG MappedWriteIoCount;
    ULONG PagedPoolPages;
    ULONG NonPagedPoolPages;
    ULONG PagedPoolAllocs;
    ULONG PagedPoolFrees;
    ULONG NonPagedPoolAllocs;
    ULONG NonPagedPoolFrees;
    ULONG FreeSystemPtes;
    ULONG ResidentSystemCodePage;
    ULONG TotalSystemDriverPages;
    ULONG TotalSystemCodePages;
    ULONG NonPagedPoolLookasideHits;
    ULONG PagedPoolLookasideHits;
    ULONG AvailablePagedPoolPages;
    ULONG ResidentSystemCachePage;
    ULONG ResidentPagedPoolPage;
    ULONG ResidentSystemDriverPage;
    ULONG CcFastReadNoWait;
    ULONG CcFastReadWait;
    ULONG CcFastReadResourceMiss;
    ULONG CcFastReadNotPossible;
    ULONG CcFastMdlReadNoWait;
    ULONG CcFastMdlReadWait;
    ULONG CcFastMdlReadResourceMiss;
    ULONG CcFastMdlReadNotPossible;
    ULONG CcMapDataNoWait;
    ULONG CcMapDataWait;
    ULONG CcMapDataNoWaitMiss;
    ULONG CcMapDataWaitMiss;
    ULONG CcPinMappedDataCount;
    ULONG CcPinReadNoWait;
    ULONG CcPinReadWait;
    ULONG CcPinReadNoWaitMiss;
    ULONG CcPinReadWaitMiss;
    ULONG CcCopyReadNoWait;
    ULONG CcCopyReadWait;
    ULONG CcCopyReadNoWaitMiss;
    ULONG CcCopyReadWaitMiss;
    ULONG CcMdlReadNoWait;
    ULONG CcMdlReadWait;
    ULONG CcMdlReadNoWaitMiss;
    ULONG CcMdlReadWaitMiss;
    ULONG CcReadAheadIos;
    ULONG CcLazyWriteIos;
    ULONG CcLazyWritePages;
    ULONG CcDataFlushes;
    ULONG CcDataPages;
    ULONG ContextSwitches;
    ULONG FirstLevelTbFills;
    ULONG SecondLevelTbFills;
    ULONG SystemCalls;

    ULONG64 CcTotalDirtyPages;
    ULONG64 CcDirtyPageThreshold;

    LONG64 ResidentAvailablePages;
    ULONG64 SharedCommittedPages;
} MINIDUMP_SYSTEM_PERFORMANCE_INFORMATION, *PMINIDUMP_SYSTEM_PERFORMANCE_INFORMATION;

typedef struct _MINIDUMP_SYSTEM_PERFORMANCE_INFORMATION_2 {
    ULONG64 IdleProcessTime;
    ULONG64 IoReadTransferCount;
    ULONG64 IoWriteTransferCount;
    ULONG64 IoOtherTransferCount;
    ULONG IoReadOperationCount;
    ULONG IoWriteOperationCount;
    ULONG IoOtherOperationCount;
    ULONG AvailablePages;
    ULONG CommittedPages;
    ULONG CommitLimit;
    ULONG PeakCommitment;
    ULONG PageFaultCount;
    ULONG CopyOnWriteCount;
    ULONG TransitionCount;
    ULONG CacheTransitionCount;
    ULONG DemandZeroCount;
    ULONG PageReadCount;
    ULONG PageReadIoCount;
    ULONG CacheReadCount;
    ULONG CacheIoCount;
    ULONG DirtyPagesWriteCount;
    ULONG DirtyWriteIoCount;
    ULONG MappedPagesWriteCount;
    ULONG MappedWriteIoCount;
    ULONG PagedPoolPages;
    ULONG NonPagedPoolPages;
    ULONG PagedPoolAllocs;
    ULONG PagedPoolFrees;
    ULONG NonPagedPoolAllocs;
    ULONG NonPagedPoolFrees;
    ULONG FreeSystemPtes;
    ULONG ResidentSystemCodePage;
    ULONG TotalSystemDriverPages;
    ULONG TotalSystemCodePages;
    ULONG NonPagedPoolLookasideHits;
    ULONG PagedPoolLookasideHits;
    ULONG AvailablePagedPoolPages;
    ULONG ResidentSystemCachePage;
    ULONG ResidentPagedPoolPage;
    ULONG ResidentSystemDriverPage;
    ULONG CcFastReadNoWait;
    ULONG CcFastReadWait;
    ULONG CcFastReadResourceMiss;
    ULONG CcFastReadNotPossible;
    ULONG CcFastMdlReadNoWait;
    ULONG CcFastMdlReadWait;
    ULONG CcFastMdlReadResourceMiss;
    ULONG CcFastMdlReadNotPossible;
    ULONG CcMapDataNoWait;
    ULONG CcMapDataWait;
    ULONG CcMapDataNoWaitMiss;
    ULONG CcMapDataWaitMiss;
    ULONG CcPinMappedDataCount;
    ULONG CcPinReadNoWait;
    ULONG CcPinReadWait;
    ULONG CcPinReadNoWaitMiss;
    ULONG CcPinReadWaitMiss;
    ULONG CcCopyReadNoWait;
    ULONG CcCopyReadWait;
    ULONG CcCopyReadNoWaitMiss;
    ULONG CcCopyReadWaitMiss;
    ULONG CcMdlReadNoWait;
    ULONG CcMdlReadWait;
    ULONG CcMdlReadNoWaitMiss;
    ULONG CcMdlReadWaitMiss;
    ULONG CcReadAheadIos;
    ULONG CcLazyWriteIos;
    ULONG CcLazyWritePages;
    ULONG CcDataFlushes;
    ULONG CcDataPages;
    ULONG ContextSwitches;
    ULONG FirstLevelTbFills;
    ULONG SecondLevelTbFills;
    ULONG SystemCalls;

    ULONG64 CcTotalDirtyPages;
    ULONG64 CcDirtyPageThreshold;

    LONG64 ResidentAvailablePages;
    ULONG64 SharedCommittedPages;

    ULONGLONG MdlPagesAllocated;
    ULONGLONG PfnDatabaseCommittedPages;

    ULONGLONG SystemPageTableCommittedPages;
    ULONGLONG ContiguousPagesAllocated;
} MINIDUMP_SYSTEM_PERFORMANCE_INFORMATION_2, *PMINIDUMP_SYSTEM_PERFORMANCE_INFORMATION_2;

#define MINIDUMP_SYSMEMINFO1_FILECACHE_TRANSITIONREPURPOSECOUNT_FLAGS                       0x0001
#define MINIDUMP_SYSMEMINFO1_BASICPERF                                                      0x0002
#define MINIDUMP_SYSMEMINFO1_PERF_CCTOTALDIRTYPAGES_CCDIRTYPAGETHRESHOLD                    0x0004
#define MINIDUMP_SYSMEMINFO1_PERF_RESIDENTAVAILABLEPAGES_SHAREDCOMMITPAGES                  0x0008
#define MINIDUMP_SYSMEMINFO1_PERF_MDLPAGESALLOCATED_PFNDATABASECOMMITTEDPAGES               0x0010
#define MINIDUMP_SYSMEMINFO1_PERF_SYSTEMPAGETABLECOMMITTEDPAGES_CONTIGUOUSPAGESALLOCATED    0x0020

typedef struct _MINIDUMP_SYSTEM_MEMORY_INFO_1 {
    USHORT Revision;
    USHORT Flags;

    MINIDUMP_SYSTEM_BASIC_INFORMATION BasicInfo;
    MINIDUMP_SYSTEM_FILECACHE_INFORMATION FileCacheInfo;
    MINIDUMP_SYSTEM_BASIC_PERFORMANCE_INFORMATION BasicPerfInfo;
    MINIDUMP_SYSTEM_PERFORMANCE_INFORMATION PerfInfo;
} MINIDUMP_SYSTEM_MEMORY_INFO_1, *PMINIDUMP_SYSTEM_MEMORY_INFO_1;

typedef struct _MINIDUMP_SYSTEM_MEMORY_INFO_2 {
    USHORT Revision;
    USHORT Flags;

    MINIDUMP_SYSTEM_BASIC_INFORMATION BasicInfo;
    MINIDUMP_SYSTEM_FILECACHE_INFORMATION FileCacheInfo;
    MINIDUMP_SYSTEM_BASIC_PERFORMANCE_INFORMATION BasicPerfInfo;
    MINIDUMP_SYSTEM_PERFORMANCE_INFORMATION_2 PerfInfo;
} MINIDUMP_SYSTEM_MEMORY_INFO_2, *PMINIDUMP_SYSTEM_MEMORY_INFO_2;

typedef MINIDUMP_SYSTEM_MEMORY_INFO_2 MINIDUMP_SYSTEM_MEMORY_INFO_N;
typedef MINIDUMP_SYSTEM_MEMORY_INFO_N *PMINIDUMP_SYSTEM_MEMORY_INFO_N;

//
// Support for process VM counters.
//
typedef struct _MINIDUMP_PROCESS_VM_COUNTERS_1 {
    USHORT Revision;
    ULONG PageFaultCount;
    ULONG64 PeakWorkingSetSize;
    ULONG64 WorkingSetSize;
    ULONG64 QuotaPeakPagedPoolUsage;
    ULONG64 QuotaPagedPoolUsage;
    ULONG64 QuotaPeakNonPagedPoolUsage;
    ULONG64 QuotaNonPagedPoolUsage;
    ULONG64 PagefileUsage;
    ULONG64 PeakPagefileUsage;
    ULONG64 PrivateUsage;
} MINIDUMP_PROCESS_VM_COUNTERS_1, *PMINIDUMP_PROCESS_VM_COUNTERS_1;

#define MINIDUMP_PROCESS_VM_COUNTERS                0x0001
#define MINIDUMP_PROCESS_VM_COUNTERS_VIRTUALSIZE    0x0002
#define MINIDUMP_PROCESS_VM_COUNTERS_EX             0x0004
#define MINIDUMP_PROCESS_VM_COUNTERS_EX2            0x0008
#define MINIDUMP_PROCESS_VM_COUNTERS_JOB            0x0010

typedef struct _MINIDUMP_PROCESS_VM_COUNTERS_2 {
    USHORT Revision;
    USHORT Flags;
    ULONG PageFaultCount;
    ULONG64 PeakWorkingSetSize;
    ULONG64 WorkingSetSize;
    ULONG64 QuotaPeakPagedPoolUsage;
    ULONG64 QuotaPagedPoolUsage;
    ULONG64 QuotaPeakNonPagedPoolUsage;
    ULONG64 QuotaNonPagedPoolUsage;
    ULONG64 PagefileUsage;
    ULONG64 PeakPagefileUsage;
    ULONG64 PeakVirtualSize;            // VIRTUALSIZE
    ULONG64 VirtualSize;                // VIRTUALSIZE
    ULONG64 PrivateUsage;               // EX+
    ULONG64 PrivateWorkingSetSize;      // EX2+
    ULONG64 SharedCommitUsage;          // EX2+

    ULONG64 JobSharedCommitUsage;       // JOB+
    ULONG64 JobPrivateCommitUsage;      // JOB+
    ULONG64 JobPeakPrivateCommitUsage;  // JOB+
    ULONG64 JobPrivateCommitLimit;      // JOB+
    ULONG64 JobTotalCommitLimit;        // JOB+
} MINIDUMP_PROCESS_VM_COUNTERS_2, *PMINIDUMP_PROCESS_VM_COUNTERS_2;

typedef MINIDUMP_PROCESS_VM_COUNTERS_2 MINIDUMP_PROCESS_VM_COUNTERS_N;
typedef MINIDUMP_PROCESS_VM_COUNTERS_N *PMINIDUMP_PROCESS_VM_COUNTERS_N;

//
// Support for arbitrary user-defined information.
//

typedef struct _MINIDUMP_USER_RECORD {
    ULONG32 Type;
    MINIDUMP_LOCATION_DESCRIPTOR Memory;
} MINIDUMP_USER_RECORD, *PMINIDUMP_USER_RECORD;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#pragma region Desktop Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES)

typedef struct _MINIDUMP_USER_STREAM {
    ULONG32 Type;
    ULONG BufferSize;
    PVOID Buffer;

} MINIDUMP_USER_STREAM, *PMINIDUMP_USER_STREAM;

typedef struct _MINIDUMP_USER_STREAM_INFORMATION {
    ULONG UserStreamCount;
    PMINIDUMP_USER_STREAM UserStreamArray;
} MINIDUMP_USER_STREAM_INFORMATION, *PMINIDUMP_USER_STREAM_INFORMATION;

//
// Callback support.
//

typedef enum _MINIDUMP_CALLBACK_TYPE {
    ModuleCallback,
    ThreadCallback,
    ThreadExCallback,
    IncludeThreadCallback,
    IncludeModuleCallback,
    MemoryCallback,
    CancelCallback,
    WriteKernelMinidumpCallback,
    KernelMinidumpStatusCallback,
    RemoveMemoryCallback,
    IncludeVmRegionCallback,
    IoStartCallback,
    IoWriteAllCallback,
    IoFinishCallback,
    ReadMemoryFailureCallback,
    SecondaryFlagsCallback,
    IsProcessSnapshotCallback,
    VmStartCallback,
    VmQueryCallback,
    VmPreReadCallback,
    VmPostReadCallback
} MINIDUMP_CALLBACK_TYPE;

typedef struct _MINIDUMP_THREAD_CALLBACK {
    ULONG ThreadId;
    HANDLE ThreadHandle;
#if defined(_ARM64_)
    ULONG Pad;
#endif
    CONTEXT Context;
    ULONG SizeOfContext;
    ULONG64 StackBase;
    ULONG64 StackEnd;
} MINIDUMP_THREAD_CALLBACK, *PMINIDUMP_THREAD_CALLBACK;

typedef struct _MINIDUMP_THREAD_EX_CALLBACK {
    ULONG ThreadId;
    HANDLE ThreadHandle;
#if defined(_ARM64_)
    ULONG Pad;
#endif
    CONTEXT Context;
    ULONG SizeOfContext;
    ULONG64 StackBase;
    ULONG64 StackEnd;
    ULONG64 BackingStoreBase;
    ULONG64 BackingStoreEnd;
} MINIDUMP_THREAD_EX_CALLBACK, *PMINIDUMP_THREAD_EX_CALLBACK;

typedef struct _MINIDUMP_INCLUDE_THREAD_CALLBACK {
    ULONG ThreadId;
} MINIDUMP_INCLUDE_THREAD_CALLBACK, *PMINIDUMP_INCLUDE_THREAD_CALLBACK;

typedef enum _THREAD_WRITE_FLAGS {
    ThreadWriteThread            = 0x0001,
    ThreadWriteStack             = 0x0002,
    ThreadWriteContext           = 0x0004,
    ThreadWriteBackingStore      = 0x0008,
    ThreadWriteInstructionWindow = 0x0010,
    ThreadWriteThreadData        = 0x0020,
    ThreadWriteThreadInfo        = 0x0040,
} THREAD_WRITE_FLAGS;

typedef struct _MINIDUMP_MODULE_CALLBACK {
    PWCHAR FullPath;
    ULONG64 BaseOfImage;
    ULONG SizeOfImage;
    ULONG CheckSum;
    ULONG TimeDateStamp;
    VS_FIXEDFILEINFO VersionInfo;
    PVOID CvRecord;
    ULONG SizeOfCvRecord;
    PVOID MiscRecord;
    ULONG SizeOfMiscRecord;
} MINIDUMP_MODULE_CALLBACK, *PMINIDUMP_MODULE_CALLBACK;

typedef struct _MINIDUMP_INCLUDE_MODULE_CALLBACK {
    ULONG64 BaseOfImage;
} MINIDUMP_INCLUDE_MODULE_CALLBACK, *PMINIDUMP_INCLUDE_MODULE_CALLBACK;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

typedef enum _MODULE_WRITE_FLAGS {
    ModuleWriteModule        = 0x0001,
    ModuleWriteDataSeg       = 0x0002,
    ModuleWriteMiscRecord    = 0x0004,
    ModuleWriteCvRecord      = 0x0008,
    ModuleReferencedByMemory = 0x0010,
    ModuleWriteTlsData       = 0x0020,
    ModuleWriteCodeSegs      = 0x0040,
} MODULE_WRITE_FLAGS;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#pragma region Desktop Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES)

typedef struct _MINIDUMP_IO_CALLBACK {
    HANDLE Handle;
    ULONG64 Offset;
    PVOID Buffer;
    ULONG BufferBytes;
} MINIDUMP_IO_CALLBACK, *PMINIDUMP_IO_CALLBACK;

typedef struct _MINIDUMP_READ_MEMORY_FAILURE_CALLBACK
{
    ULONG64 Offset;
    ULONG Bytes;
    HRESULT FailureStatus;
} MINIDUMP_READ_MEMORY_FAILURE_CALLBACK,
  *PMINIDUMP_READ_MEMORY_FAILURE_CALLBACK;

typedef struct _MINIDUMP_VM_QUERY_CALLBACK
{
    ULONG64 Offset;
} MINIDUMP_VM_QUERY_CALLBACK, *PMINIDUMP_VM_QUERY_CALLBACK;

typedef struct _MINIDUMP_VM_PRE_READ_CALLBACK
{
    ULONG64 Offset;
    PVOID Buffer;
    ULONG Size;
} MINIDUMP_VM_PRE_READ_CALLBACK, *PMINIDUMP_VM_PRE_READ_CALLBACK;

typedef struct _MINIDUMP_VM_POST_READ_CALLBACK
{
    ULONG64 Offset;
    PVOID Buffer;
    ULONG Size;
    ULONG Completed;
    HRESULT Status;
} MINIDUMP_VM_POST_READ_CALLBACK, *PMINIDUMP_VM_POST_READ_CALLBACK;

typedef struct _MINIDUMP_CALLBACK_INPUT {
    ULONG ProcessId;
    HANDLE ProcessHandle;
    ULONG CallbackType;
    union {
        HRESULT Status;
        MINIDUMP_THREAD_CALLBACK Thread;
        MINIDUMP_THREAD_EX_CALLBACK ThreadEx;
        MINIDUMP_MODULE_CALLBACK Module;
        MINIDUMP_INCLUDE_THREAD_CALLBACK IncludeThread;
        MINIDUMP_INCLUDE_MODULE_CALLBACK IncludeModule;
        MINIDUMP_IO_CALLBACK Io;
        MINIDUMP_READ_MEMORY_FAILURE_CALLBACK ReadMemoryFailure;
        ULONG SecondaryFlags;
        MINIDUMP_VM_QUERY_CALLBACK VmQuery;
        MINIDUMP_VM_PRE_READ_CALLBACK VmPreRead;
        MINIDUMP_VM_POST_READ_CALLBACK VmPostRead;
    };
} MINIDUMP_CALLBACK_INPUT, *PMINIDUMP_CALLBACK_INPUT;

typedef struct _MINIDUMP_CALLBACK_OUTPUT {
    union {
        ULONG ModuleWriteFlags;
        ULONG ThreadWriteFlags;
        ULONG SecondaryFlags;
        struct {
            ULONG64 MemoryBase;
            ULONG MemorySize;
        };
        struct {
            BOOL CheckCancel;
            BOOL Cancel;
        };
        HANDLE Handle;
        struct {
            MINIDUMP_MEMORY_INFO VmRegion;
            BOOL Continue;
        };
        struct {
            HRESULT VmQueryStatus;
            MINIDUMP_MEMORY_INFO VmQueryResult;
        };
        struct {
            HRESULT VmReadStatus;
            ULONG VmReadBytesCompleted;
        };
        HRESULT Status;
    };
} MINIDUMP_CALLBACK_OUTPUT, *PMINIDUMP_CALLBACK_OUTPUT;

//
// A normal minidump contains just the information
// necessary to capture stack traces for all of the
// existing threads in a process.
//
// A minidump with data segments includes all of the data
// sections from loaded modules in order to capture
// global variable contents.  This can make the dump much
// larger if many modules have global data.
//
// A minidump with full memory includes all of the accessible
// memory in the process and can be very large.  A minidump
// with full memory always has the raw memory data at the end
// of the dump so that the initial structures in the dump can
// be mapped directly without having to include the raw
// memory information.
//
// Stack and backing store memory can be filtered to remove
// data unnecessary for stack walking.  This can improve
// compression of stacks and also deletes data that may
// be private and should not be stored in a dump.
// Memory can also be scanned to see what modules are
// referenced by stack and backing store memory to allow
// omission of other modules to reduce dump size.
// In either of these modes the ModuleReferencedByMemory flag
// is set for all modules referenced before the base
// module callbacks occur.
//
// On some operating systems a list of modules that were
// recently unloaded is kept in addition to the currently
// loaded module list.  This information can be saved in
// the dump if desired.
//
// Stack and backing store memory can be scanned for referenced
// pages in order to pick up data referenced by locals or other
// stack memory.  This can increase the size of a dump significantly.
//
// Module paths may contain undesired information such as user names
// or other important directory names so they can be stripped.  This
// option reduces the ability to locate the proper image later
// and should only be used in certain situations.
//
// Complete operating system per-process and per-thread information can
// be gathered and stored in the dump.
//
// The virtual address space can be scanned for various types
// of memory to be included in the dump.
//
// Code which is concerned with potentially private information
// getting into the minidump can set a flag that automatically
// modifies all existing and future flags to avoid placing
// unnecessary data in the dump.  Basic data, such as stack
// information, will still be included but optional data, such
// as indirect memory, will not.
//
// When doing a full memory dump it's possible to store all
// of the enumerated memory region descriptive information
// in a memory information stream.
//
// Additional thread information beyond the basic thread
// structure can be collected if desired.
//
// A minidump with code segments includes all of the code
// and code-related sections from loaded modules in order
// to capture executable content.
//
// MiniDumpWithoutAuxiliaryState turns off any secondary,
// auxiliary-supported memory gathering.
//
// MiniDumpWithFullAuxiliaryState asks any present auxiliary
// data providers to include all of their state in the dump.
// The exact set of what is provided depends on the auxiliary.
// This can be quite large.
//
// MiniDumpFilterWriteCombinedMemory asks to exclude all memory
// with the virtual protection attribute of PAGE_WRITECOMBINE.
//

typedef enum _MINIDUMP_TYPE {
    MiniDumpNormal                         = 0x00000000,
    MiniDumpWithDataSegs                   = 0x00000001,
    MiniDumpWithFullMemory                 = 0x00000002,
    MiniDumpWithHandleData                 = 0x00000004,
    MiniDumpFilterMemory                   = 0x00000008,
    MiniDumpScanMemory                     = 0x00000010,
    MiniDumpWithUnloadedModules            = 0x00000020,
    MiniDumpWithIndirectlyReferencedMemory = 0x00000040,
    MiniDumpFilterModulePaths              = 0x00000080,
    MiniDumpWithProcessThreadData          = 0x00000100,
    MiniDumpWithPrivateReadWriteMemory     = 0x00000200,
    MiniDumpWithoutOptionalData            = 0x00000400,
    MiniDumpWithFullMemoryInfo             = 0x00000800,
    MiniDumpWithThreadInfo                 = 0x00001000,
    MiniDumpWithCodeSegs                   = 0x00002000,
    MiniDumpWithoutAuxiliaryState          = 0x00004000,
    MiniDumpWithFullAuxiliaryState         = 0x00008000,
    MiniDumpWithPrivateWriteCopyMemory     = 0x00010000,
    MiniDumpIgnoreInaccessibleMemory       = 0x00020000,
    MiniDumpWithTokenInformation           = 0x00040000,
    MiniDumpWithModuleHeaders              = 0x00080000,
    MiniDumpFilterTriage                   = 0x00100000,
    MiniDumpWithAvxXStateContext           = 0x00200000,
    MiniDumpWithIptTrace                   = 0x00400000,
    MiniDumpScanInaccessiblePartialPages   = 0x00800000,
    MiniDumpFilterWriteCombinedMemory      = 0x01000000,
    MiniDumpValidTypeFlags                 = 0x01ffffff,
    MiniDumpNoIgnoreInaccessibleMemory     = 0x02000000,
    MiniDumpValidTypeFlagsEx               = 0x03ffffff,
} MINIDUMP_TYPE;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

//
// In addition to the primary flags provided to
// MiniDumpWriteDump there are additional, less
// frequently used options queried via the secondary
// flags callback.
//
// MiniSecondaryWithoutPowerInfo suppresses the minidump
// query that retrieves processor power information for
// MINIDUMP_MISC_INFO.
//

typedef enum _MINIDUMP_SECONDARY_FLAGS {
    MiniSecondaryWithoutPowerInfo = 0x00000001,

    MiniSecondaryValidFlags       = 0x00000001,
} MINIDUMP_SECONDARY_FLAGS;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#pragma region Desktop Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES)

//
// The minidump callback should modify the FieldsToWrite parameter to reflect
// what portions of the specified thread or module should be written to the
// file.
//

typedef
BOOL
(WINAPI * MINIDUMP_CALLBACK_ROUTINE) (
    _Inout_ PVOID CallbackParam,
    _In_    PMINIDUMP_CALLBACK_INPUT CallbackInput,
    _Inout_ PMINIDUMP_CALLBACK_OUTPUT CallbackOutput
    );

typedef struct _MINIDUMP_CALLBACK_INFORMATION {
    MINIDUMP_CALLBACK_ROUTINE CallbackRoutine;
    PVOID CallbackParam;
} MINIDUMP_CALLBACK_INFORMATION, *PMINIDUMP_CALLBACK_INFORMATION;

//++
//
// PVOID
// RVA_TO_ADDR(
//     PVOID Mapping,
//     ULONG Rva
//     )
//
// Routine Description:
//
//     Map an RVA that is contained within a mapped file to it's associated
//     flat address.
//
// Arguments:
//
//     Mapping - Base address of mapped file containing the RVA.
//
//     Rva - An Rva to fixup.
//
// Return Values:
//
//     A pointer to the desired data.
//
//--

#define RVA_TO_ADDR(Mapping,Rva) ((PVOID)(((ULONG_PTR) (Mapping)) + (Rva)))

BOOL
WINAPI
MiniDumpWriteDump(
    _In_ HANDLE hProcess,
    _In_ DWORD ProcessId,
    _In_ HANDLE hFile,
    _In_ MINIDUMP_TYPE DumpType,
    _In_opt_ PMINIDUMP_EXCEPTION_INFORMATION ExceptionParam,
    _In_opt_ PMINIDUMP_USER_STREAM_INFORMATION UserStreamParam,
    _In_opt_ PMINIDUMP_CALLBACK_INFORMATION CallbackParam
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

BOOL
WINAPI
MiniDumpReadDumpStream(
    _In_ PVOID BaseOfDump,
    _In_ ULONG StreamNumber,
    _Outptr_result_maybenull_ PMINIDUMP_DIRECTORY* Dir,
    _Outptr_result_maybenull_ PVOID* StreamPointer,
    _Out_opt_ ULONG* StreamSize
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#pragma region Desktop Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES)

#ifdef __cplusplus
}
#endif

#if defined(_MSC_VER)
#if _MSC_VER >= 800
#if _MSC_VER >= 1200
#pragma warning(pop)
#else
#pragma warning(default:4200)    /* Zero length array */
#pragma warning(default:4201)    /* Nameless struct/union */
#endif
#endif
#endif

#include <poppack.h>

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#endif /* _MINIDUMP_H */

// HEXTRACT: end_dbghelp hide-line end_imagehlp

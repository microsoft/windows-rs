/*++ BUILD Version: 0000     Increment this if a change has global effects

Copyright (c) Microsoft Corporation. All rights reserved.

Abstract:

    Defines the process snapshot API: a facility to capture the state of a
    process in part or whole.

--*/

#ifdef _MSC_VER
#pragma once
#endif


// Handle descriptor flags.
typedef enum
{
    PSS_HANDLE_NONE                           = 0x00,
    PSS_HANDLE_HAVE_TYPE                      = 0x01,
    PSS_HANDLE_HAVE_NAME                      = 0x02,
    PSS_HANDLE_HAVE_BASIC_INFORMATION         = 0x04,
    PSS_HANDLE_HAVE_TYPE_SPECIFIC_INFORMATION = 0x08
} PSS_HANDLE_FLAGS;
DEFINE_ENUM_FLAG_OPERATORS(PSS_HANDLE_FLAGS);

// Object types. (As recognized by this facility.)
typedef enum
{
    PSS_OBJECT_TYPE_UNKNOWN   = 0,
    PSS_OBJECT_TYPE_PROCESS   = 1,
    PSS_OBJECT_TYPE_THREAD    = 2,
    PSS_OBJECT_TYPE_MUTANT    = 3,
    PSS_OBJECT_TYPE_EVENT     = 4,
    PSS_OBJECT_TYPE_SECTION   = 5,
    // BUGBUG WINBLUE 571662 2013-12-31 GenghisK: #ifdef this for OS after
    // WINBLUE
    PSS_OBJECT_TYPE_SEMAPHORE = 6
} PSS_OBJECT_TYPE;

// Capture/creation flags.
typedef enum
{
    PSS_CAPTURE_NONE                                = 0x00000000,
    PSS_CAPTURE_VA_CLONE                            = 0x00000001,
    PSS_CAPTURE_RESERVED_00000002                   = 0x00000002,
    PSS_CAPTURE_HANDLES                             = 0x00000004,
    PSS_CAPTURE_HANDLE_NAME_INFORMATION             = 0x00000008,
    PSS_CAPTURE_HANDLE_BASIC_INFORMATION            = 0x00000010,
    PSS_CAPTURE_HANDLE_TYPE_SPECIFIC_INFORMATION    = 0x00000020,
    PSS_CAPTURE_HANDLE_TRACE                        = 0x00000040,
    PSS_CAPTURE_THREADS                             = 0x00000080,
    PSS_CAPTURE_THREAD_CONTEXT                      = 0x00000100,
    PSS_CAPTURE_THREAD_CONTEXT_EXTENDED             = 0x00000200,
    PSS_CAPTURE_RESERVED_00000400                   = 0x00000400,
    PSS_CAPTURE_VA_SPACE                            = 0x00000800,
    PSS_CAPTURE_VA_SPACE_SECTION_INFORMATION        = 0x00001000,
    PSS_CAPTURE_IPT_TRACE                           = 0x00002000,
    PSS_CAPTURE_RESERVED_00004000                   = 0x00004000,

    PSS_CREATE_BREAKAWAY_OPTIONAL                   = 0x04000000,
    PSS_CREATE_BREAKAWAY                            = 0x08000000,
    PSS_CREATE_FORCE_BREAKAWAY                      = 0x10000000,
    PSS_CREATE_USE_VM_ALLOCATIONS                   = 0x20000000,
    PSS_CREATE_MEASURE_PERFORMANCE                  = 0x40000000,
    PSS_CREATE_RELEASE_SECTION                      = 0x80000000
} PSS_CAPTURE_FLAGS;
DEFINE_ENUM_FLAG_OPERATORS(PSS_CAPTURE_FLAGS);

// Defines the resolution of the performance counters in
// PSS_PERFORMANCE_COUNTERS.
// The current resolution is microseconds.
#define PSS_PERF_RESOLUTION     1000000

// PSS_QUERY_SNAPSHOT information classes.
typedef enum
{
    PSS_QUERY_PROCESS_INFORMATION = 0,
    PSS_QUERY_VA_CLONE_INFORMATION = 1,
    PSS_QUERY_AUXILIARY_PAGES_INFORMATION = 2,
    PSS_QUERY_VA_SPACE_INFORMATION = 3,
    PSS_QUERY_HANDLE_INFORMATION = 4,
    PSS_QUERY_THREAD_INFORMATION = 5,
    PSS_QUERY_HANDLE_TRACE_INFORMATION = 6,
    PSS_QUERY_PERFORMANCE_COUNTERS = 7
} PSS_QUERY_INFORMATION_CLASS;

// PSS_WALK_SNAPSHOT information classes.
typedef enum
{
    PSS_WALK_AUXILIARY_PAGES = 0,
    PSS_WALK_VA_SPACE = 1,
    PSS_WALK_HANDLES = 2,
    PSS_WALK_THREADS = 3,
    PSS_WALK_THREAD_NAME = 4
} PSS_WALK_INFORMATION_CLASS;

// PssDuplicateSnapshot flags.
typedef enum
{
    PSS_DUPLICATE_NONE         = 0x00,
    PSS_DUPLICATE_CLOSE_SOURCE = 0x01
} PSS_DUPLICATE_FLAGS;
DEFINE_ENUM_FLAG_OPERATORS(PSS_DUPLICATE_FLAGS);


DECLARE_HANDLE(HPSS);
DECLARE_HANDLE(HPSSWALK);

// PssQuerySnapshot/PssWalkSnapshot information structures.

// Information class: PSS_QUERY_PROCESS_INFORMATION.
typedef enum
{
    PSS_PROCESS_FLAGS_NONE        = 0x00000000,
    PSS_PROCESS_FLAGS_PROTECTED   = 0x00000001,
    PSS_PROCESS_FLAGS_WOW64       = 0x00000002,
    PSS_PROCESS_FLAGS_RESERVED_03 = 0x00000004,
    PSS_PROCESS_FLAGS_RESERVED_04 = 0x00000008,
    PSS_PROCESS_FLAGS_FROZEN      = 0x00000010
} PSS_PROCESS_FLAGS;
DEFINE_ENUM_FLAG_OPERATORS(PSS_PROCESS_FLAGS);

typedef struct
{
    DWORD ExitStatus;
    void* PebBaseAddress;
    ULONG_PTR AffinityMask;
    LONG BasePriority;
    DWORD ProcessId;
    DWORD ParentProcessId;
    PSS_PROCESS_FLAGS Flags;
    FILETIME CreateTime;
    FILETIME ExitTime;
    FILETIME KernelTime;
    FILETIME UserTime;
    DWORD PriorityClass;
    ULONG_PTR PeakVirtualSize;
    ULONG_PTR VirtualSize;
    DWORD PageFaultCount;
    ULONG_PTR PeakWorkingSetSize;
    ULONG_PTR WorkingSetSize;
    ULONG_PTR QuotaPeakPagedPoolUsage;
    ULONG_PTR QuotaPagedPoolUsage;
    ULONG_PTR QuotaPeakNonPagedPoolUsage;
    ULONG_PTR QuotaNonPagedPoolUsage;
    ULONG_PTR PagefileUsage;
    ULONG_PTR PeakPagefileUsage;
    ULONG_PTR PrivateUsage;
    DWORD ExecuteFlags;
    wchar_t ImageFileName[MAX_PATH];
} PSS_PROCESS_INFORMATION;

// Information class: PSS_QUERY_VA_CLONE_INFORMATION.
typedef struct
{
    HANDLE VaCloneHandle;
} PSS_VA_CLONE_INFORMATION;

// Information class: PSS_QUERY_AUXILIARY_PAGES_INFORMATION.
typedef struct
{
    DWORD AuxPagesCaptured;
} PSS_AUXILIARY_PAGES_INFORMATION;

// Information class: PSS_QUERY_VA_SPACE_INFORMATION.
typedef struct
{
    DWORD RegionCount;
} PSS_VA_SPACE_INFORMATION;

// Information class: PSS_QUERY_HANDLE_INFORMATION.
typedef struct 
{
    DWORD HandlesCaptured;
} PSS_HANDLE_INFORMATION;

// Information class: PSS_QUERY_THREAD_INFORMATION.
typedef struct
{
    DWORD ThreadsCaptured;
    DWORD ContextLength;
} PSS_THREAD_INFORMATION;

// Information class: PSS_QUERY_HANDLE_TRACE_INFORMATION.
typedef struct
{
    HANDLE SectionHandle;
    DWORD Size;
} PSS_HANDLE_TRACE_INFORMATION;

// Information class: PSS_QUERY_PERFORMANCE_COUNTERS.
typedef struct
{
    UINT64 TotalCycleCount;
    UINT64 TotalWallClockPeriod;
    UINT64 VaCloneCycleCount;
    UINT64 VaCloneWallClockPeriod;
    UINT64 VaSpaceCycleCount;
    UINT64 VaSpaceWallClockPeriod;
    UINT64 AuxPagesCycleCount;
    UINT64 AuxPagesWallClockPeriod;
    UINT64 HandlesCycleCount;
    UINT64 HandlesWallClockPeriod;
    UINT64 ThreadsCycleCount;
    UINT64 ThreadsWallClockPeriod;
} PSS_PERFORMANCE_COUNTERS;

// Information class: PSS_WALK_AUXILIARY_PAGES.
typedef struct
{
    void* Address;
    MEMORY_BASIC_INFORMATION BasicInformation;
    FILETIME CaptureTime;

    void* PageContents;
    DWORD PageSize;
} PSS_AUXILIARY_PAGE_ENTRY;

// Information class: PSS_WALK_VA_SPACE.
typedef struct
{
    void* BaseAddress;
    void* AllocationBase;
    DWORD AllocationProtect;
    ULONG_PTR RegionSize;
    DWORD State;
    DWORD Protect;
    DWORD Type;
    DWORD TimeDateStamp;    // IMAGE_NT_HEADERS.FileHeader.TimeDateStamp
    DWORD SizeOfImage;      // IMAGE_NT_HEADERS.OptionalHeader.SizeOfImage
    void* ImageBase;        // IMAGE_NT_HEADERS.OptionalHeader.ImageBase
    DWORD CheckSum;         // IMAGE_NT_HEADERS.OptionalHeader.CheckSum
    WORD MappedFileNameLength;
    wchar_t const* MappedFileName;  // valid for life time of walk marker
} PSS_VA_SPACE_ENTRY;

// Information class: PSS_WALK_HANDLES.
typedef struct
{
    HANDLE Handle;
    PSS_HANDLE_FLAGS Flags;
    PSS_OBJECT_TYPE ObjectType;
    FILETIME CaptureTime;
    // Object basic information.
    DWORD Attributes;
    DWORD GrantedAccess;
    DWORD HandleCount;
    DWORD PointerCount;
    DWORD PagedPoolCharge;
    DWORD NonPagedPoolCharge;
    FILETIME CreationTime;
    WORD TypeNameLength;
    wchar_t const* TypeName;        // valid for life time of walk marker
    WORD ObjectNameLength;
    wchar_t const* ObjectName;      // valid for life time of walk marker
    // Type-specific information.
    union
    {
        // Valid for ObjectType = PSS_OBJECT_TYPE_PROCESS.
        struct
        {
            DWORD ExitStatus;
            void* PebBaseAddress;
            ULONG_PTR AffinityMask;
            LONG BasePriority;
            DWORD ProcessId;
            DWORD ParentProcessId;
            DWORD Flags;
        } Process;
        // Valid for ObjectType = PSS_OBJECT_TYPE_THREAD.
        struct
        {
            DWORD ExitStatus;
            void* TebBaseAddress;
            DWORD ProcessId;
            DWORD ThreadId;
            ULONG_PTR AffinityMask;
            int Priority;
            int BasePriority;
            void* Win32StartAddress;
        } Thread;
        // Valid for ObjectType = PSS_OBJECT_TYPE_MUTANT.
        struct
        {
            LONG CurrentCount;
            BOOL Abandoned;
            DWORD OwnerProcessId;
            DWORD OwnerThreadId;
        } Mutant;
        // Valid for ObjectType = PSS_OBJECT_TYPE_EVENT.
        struct
        {
            BOOL ManualReset;
            BOOL Signaled;
        } Event;
        // Valid for ObjectType = PSS_OBJECT_TYPE_SECTION.
        struct
        {
            void* BaseAddress;
            DWORD AllocationAttributes;
            LARGE_INTEGER MaximumSize;
        } Section;
        // Valid for ObjectType = PSS_OBJECT_TYPE_SEMAPHORE.

        struct
        {
            LONG CurrentCount;
            LONG MaximumCount;
        } Semaphore;
    } TypeSpecificInformation;
} PSS_HANDLE_ENTRY;

// Information class: PSS_WALK_THREADS.
typedef enum
{
    PSS_THREAD_FLAGS_NONE       = 0x0000,
    PSS_THREAD_FLAGS_TERMINATED = 0x0001
} PSS_THREAD_FLAGS;
DEFINE_ENUM_FLAG_OPERATORS(PSS_THREAD_FLAGS);

typedef struct
{
    DWORD ExitStatus;
    void* TebBaseAddress;
    DWORD ProcessId;
    DWORD ThreadId;
    ULONG_PTR AffinityMask;
    int Priority;
    int BasePriority;

    void* LastSyscallFirstArgument;
    WORD LastSyscallNumber;

    FILETIME CreateTime;
    FILETIME ExitTime;
    FILETIME KernelTime;
    FILETIME UserTime;

    void* Win32StartAddress;
    FILETIME CaptureTime;

    PSS_THREAD_FLAGS Flags;
    WORD SuspendCount;
    WORD SizeOfContextRecord;

    PCONTEXT ContextRecord;         // valid for life time of walk marker
} PSS_THREAD_ENTRY;

typedef struct
{
    WORD ThreadNameSize;
    wchar_t const* ThreadName;      // valid for life time of walk marker
} PSS_THREAD_NAME;

// Allocator API structure for walk marker allocations.
typedef struct
{
    void* Context;
    void* (WINAPI *AllocRoutine) (_In_ void* Context, _In_ DWORD Size);
    void (WINAPI *FreeRoutine) (_In_ void* Context, _In_opt_ void* Address);
} PSS_ALLOCATOR;

#include <winapifamily.h>

#pragma region App Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

#if (NTDDI_VERSION >= NTDDI_WINBLUE)

// Win32 APIs.
_Success_(return == ERROR_SUCCESS)
STDAPI_(DWORD)
PssCaptureSnapshot(
    _In_ HANDLE ProcessHandle,
    _In_ PSS_CAPTURE_FLAGS CaptureFlags,
    _In_opt_ DWORD ThreadContextFlags,
    _Out_ HPSS* SnapshotHandle
    );

_Success_(return == ERROR_SUCCESS)
STDAPI_(DWORD)
PssFreeSnapshot(
    _In_ HANDLE ProcessHandle,
    _In_ HPSS SnapshotHandle
    );

_Success_(return == ERROR_SUCCESS)
STDAPI_(DWORD)
PssQuerySnapshot(
    _In_ HPSS SnapshotHandle,
    _In_ PSS_QUERY_INFORMATION_CLASS InformationClass,
    _Out_writes_bytes_(BufferLength) void* Buffer,
    _In_ DWORD BufferLength
    );

_Success_(return == ERROR_SUCCESS)
STDAPI_(DWORD)
PssWalkSnapshot(
    _In_ HPSS SnapshotHandle,
    _In_ PSS_WALK_INFORMATION_CLASS InformationClass,
    _In_ HPSSWALK WalkMarkerHandle,
    _Out_writes_opt_(BufferLength) void* Buffer,
    _In_ DWORD BufferLength
    );

_Success_(return == ERROR_SUCCESS)
STDAPI_(DWORD)
PssDuplicateSnapshot(
    _In_ HANDLE SourceProcessHandle,
    _In_ HPSS SnapshotHandle,
    _In_ HANDLE TargetProcessHandle,
    _Out_ HPSS* TargetSnapshotHandle,
    _In_opt_ PSS_DUPLICATE_FLAGS Flags
    );

// Win32 walk marker management.
_Success_(return == ERROR_SUCCESS)
STDAPI_(DWORD)
PssWalkMarkerCreate(
    _In_opt_ PSS_ALLOCATOR const *Allocator,
    _Out_ HPSSWALK* WalkMarkerHandle
    );

_Success_(return == ERROR_SUCCESS)
STDAPI_(DWORD)
PssWalkMarkerFree(
    _In_ HPSSWALK WalkMarkerHandle
    );

_Success_(return == ERROR_SUCCESS)
STDAPI_(DWORD)
PssWalkMarkerGetPosition(
    _In_ HPSSWALK WalkMarkerHandle,
    _Out_ ULONG_PTR *Position
    );

_Success_(return == ERROR_SUCCESS)
STDAPI_(DWORD)
PssWalkMarkerSetPosition(
    _In_ HPSSWALK WalkMarkerHandle,
    _In_ ULONG_PTR Position
    );

_Success_(return == ERROR_SUCCESS)
STDAPI_(DWORD)
PssWalkMarkerSeekToBeginning(
    _In_ HPSSWALK WalkMarkerHandle
    );

#endif // (NTDDI_VERSION >= NTDDI_WINBLUE)
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion



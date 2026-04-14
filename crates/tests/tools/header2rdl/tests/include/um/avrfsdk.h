/*++ BUILD Version: 0011    // Increment this if a change has global effects

Copyright (c) 2004  Microsoft Corporation

Module Name:

    avrfsdk.h

Abstract:

    This header file defines Application Verifier Exposed Interfaces

Author:

    Ivan Brugiolo (ivanbrug)


Revision History:

    Creation: 15-Mar-2004

--*/

#ifndef _AVRFSDK_
#define _AVRFSDK_
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#ifdef __cplusplus
extern "C" {
#endif


//
// tool helper function
//

#define AVRF_MAX_TRACES 32

typedef struct _AVRF_BACKTRACE_INFORMATION {
    ULONG   Depth;
    ULONG   Index;
    ULONG64 ReturnAddresses[AVRF_MAX_TRACES];
} AVRF_BACKTRACE_INFORMATION, *PAVRF_BACKTRACE_INFORMATION;

enum eUserAllocationState {
    AllocationStateUnknown,
    AllocationStateBusy,
    AllocationStateFree
};

enum eHeapAllocationState {
    HeapFullPageHeap        = 0x40000000,        
    HeapMetadata            = 0x80000000,
    HeapStateMask           = 0xFFFF0000,
};

enum eHeapEnumerationLevel {
    HeapEnumerationEverything = 0x0,
    // reserved codes from 0x1 to oxFFFFFFFE
    HeapEnumerationStop = 0xFFFFFFFF,
};

typedef struct _AVRF_HEAP_ALLOCATION {
    ULONG64 HeapHandle;
    ULONG64 UserAllocation;    
    ULONG64 UserAllocationSize;
    ULONG64 Allocation;
    ULONG64 AllocationSize;    
    ULONG   UserAllocationState;
    ULONG   HeapState;
    ULONG64 HeapContext;
    PAVRF_BACKTRACE_INFORMATION BackTraceInformation;
} AVRF_HEAP_ALLOCATION, *PAVRF_HEAP_ALLOCATION;


enum eHANDLE_TRACE_OPERATIONS {
    OperationDbUnused,
    OperationDbOPEN,
    OperationDbCLOSE,
    OperationDbBADREF
};

typedef struct _AVRF_HANDLE_OPERATION {
    ULONG64 Handle;
    ULONG  ProcessId;
    ULONG  ThreadId;
    ULONG  OperationType;
    ULONG  Spare0;
    AVRF_BACKTRACE_INFORMATION BackTraceInformation;
} AVRF_HANDLE_OPERATION, * PAVRF_HANDLE_OPERATION;


enum eAvrfResourceTypes {
    AvrfResourceHeapAllocation,
    AvrfResourceHandleTrace,
    AvrfResourceMax
};

//
// generic prototype
//

typedef ULONG 
(NTAPI * AVRF_RESOURCE_ENUMERATE_CALLBACK)(
    PVOID ResourceDescription,
    PVOID EnumerationContext,
    PULONG EnumerationLevel
    );

//
// Heap Allocations Specialized Prototype
//

typedef ULONG 
(NTAPI * AVRF_HEAPALLOCATION_ENUMERATE_CALLBACK)(
    PAVRF_HEAP_ALLOCATION HeapAllocation,
    PVOID EnumerationContext,
    PULONG EnumerationLevel
    );

//
// Handle Tracing Specialized Prototype
//

typedef ULONG 
(NTAPI * AVRF_HANDLEOPERATION_ENUMERATE_CALLBACK)(
    PAVRF_HANDLE_OPERATION HandleOperation,
    PVOID EnumerationContext,
    PULONG EnumerationLevel
    );

//
// flags for AvrfResourceHeapAllocation
//

#define AVRF_ENUM_RESOURCES_FLAGS_SUSPEND             0x00000001
#define AVRF_ENUM_RESOURCES_FLAGS_DONT_RESOLVE_TRACES 0x00000002

ULONG NTAPI
VerifierEnumerateResource(
    HANDLE Process,
    ULONG  Flags,    
    ULONG  ResourceType,
    AVRF_RESOURCE_ENUMERATE_CALLBACK ResourceCallback,
    PVOID  EnumerationContext
    );
    
//
// API to find out if per user verifier settings are enabled
//

BOOL NTAPI
VerifierIsPerUserSettingsEnabled(
    VOID
    );
    
#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif /*AVRFSDK*/

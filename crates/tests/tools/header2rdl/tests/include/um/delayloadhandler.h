//
// delayloadhandler.h
//
//  Copyright (c) Microsoft Corporation.  All rights reserved.
//
//  Define structures and prototypes necessary for implementing a delayload
//  failure hook.
//

#ifdef _MSC_VER
#pragma once
#endif  // _MSC_VER

#ifdef __cplusplus
extern "C" {
#endif

#if NTDDI_VERSION >= NTDDI_WIN8

typedef struct _DELAYLOAD_PROC_DESCRIPTOR {
    ULONG ImportDescribedByName;
    union {
        LPCSTR Name;
        ULONG Ordinal;
    } Description;
} DELAYLOAD_PROC_DESCRIPTOR, *PDELAYLOAD_PROC_DESCRIPTOR;

typedef struct _DELAYLOAD_INFO {
    ULONG Size;
    PCIMAGE_DELAYLOAD_DESCRIPTOR DelayloadDescriptor;
    PIMAGE_THUNK_DATA ThunkAddress;
    LPCSTR TargetDllName;
    DELAYLOAD_PROC_DESCRIPTOR TargetApiDescriptor;
    PVOID TargetModuleBase;
    PVOID Unused;
    ULONG LastError;
} DELAYLOAD_INFO, *PDELAYLOAD_INFO;

typedef
PVOID
(WINAPI *PDELAYLOAD_FAILURE_DLL_CALLBACK) (
    _In_ ULONG NotificationReason,
    _In_ PDELAYLOAD_INFO DelayloadInfo
    );

#define DELAYLOAD_GPA_FAILURE 4

extern PDELAYLOAD_FAILURE_DLL_CALLBACK __pfnDliFailureHook2;

#endif // NTDDI_VERSION >= NTDDI_WIN8

#ifdef __cplusplus
}
#endif

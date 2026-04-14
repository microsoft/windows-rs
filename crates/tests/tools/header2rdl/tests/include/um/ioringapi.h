/*******************************************************************
*                                                                  *
* ioringapi.h -- ApiSet Contract for api-ms-win-core-ioring-l1     *
*                                                                  *
* Copyright (c) Microsoft Corporation. All rights reserved.        *
*                                                                  *
*******************************************************************/

#ifndef _APISET_IORING_
#define _APISET_IORING_

#include <apiset.h>
#include <apisetcconv.h>
#include <minwindef.h>
#include <minwinbase.h>
#include <ntioring_x.h>
#include <windef.h>
#include <winbase.h>
#include <sdkddkver.h>

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)
#ifdef NTDDI_WIN10_CO
#if (NTDDI_VERSION >= NTDDI_WIN10_CO)

// Opaque `HANDLE` for an IORING
// Release resources held by this using CloseIoRing(); [NOT CloseHandle()!]
DECLARE_HANDLE(HIORING);

// Flags to alter the behavior of the kernel for a submission queue entry
typedef enum IORING_SQE_FLAGS
{
    IOSQE_FLAGS_NONE = 0,
#ifdef NTDDI_WIN10_NI
#if (NTDDI_VERSION >= NTDDI_WIN10_NI)
    IOSQE_FLAGS_DRAIN_PRECEDING_OPS = 0x00000001,
#endif
#endif
} IORING_SQE_FLAGS;
DEFINE_ENUM_FLAG_OPERATORS( IORING_SQE_FLAGS )

// Flags to configure the kernel behavior of an IoRing. The implementation will
// fail the create call if it does not understand any of the required flags and
// ignores any advisory flags that it does not understand.
typedef enum IORING_CREATE_REQUIRED_FLAGS
{
    IORING_CREATE_REQUIRED_FLAGS_NONE = 0,
} IORING_CREATE_REQUIRED_FLAGS;
DEFINE_ENUM_FLAG_OPERATORS( IORING_CREATE_REQUIRED_FLAGS )

typedef enum IORING_CREATE_ADVISORY_FLAGS
{
    IORING_CREATE_ADVISORY_FLAGS_NONE = 0,

    // Requests the IORING implementation to skip parameter checks in the builder APIs.
    // Ordinarily the builder APIs perform checks to catch programming errors as early as possible.
    // This flag is used to disable that if the implementation understands it (as an advisory flag
    // it has no effect on an implementation that doesn't understand it so it is safe to use on all
    // versions). Normally, this is used in RELEASE builds to eliminate the redundant checks. Errors
    // from invalid parameters are still checked in the kernel and any errors appear in the completion
    // queue entries for the operation.
    IORING_CREATE_SKIP_BUILDER_PARAM_CHECKS = 0x00000001,
} IORING_CREATE_ADVISORY_FLAGS;
DEFINE_ENUM_FLAG_OPERATORS( IORING_CREATE_ADVISORY_FLAGS )

typedef struct IORING_CREATE_FLAGS
{
    IORING_CREATE_REQUIRED_FLAGS Required;
    IORING_CREATE_ADVISORY_FLAGS Advisory;
} IORING_CREATE_FLAGS;

typedef struct IORING_INFO
{
    IORING_VERSION IoRingVersion;
    IORING_CREATE_FLAGS Flags;
    UINT32 SubmissionQueueSize;
    UINT32 CompletionQueueSize;
} IORING_INFO;

typedef struct IORING_CAPABILITIES
{
    IORING_VERSION MaxVersion;
    UINT32 MaxSubmissionQueueSize;
    UINT32 MaxCompletionQueueSize;
    IORING_FEATURE_FLAGS FeatureFlags;
} IORING_CAPABILITIES;

// enum used as discriminator for references to resources that
// support preregistration in an IORING
typedef enum IORING_REF_KIND
{
    IORING_REF_RAW,
    IORING_REF_REGISTERED,
} IORING_REF_KIND;

typedef struct IORING_HANDLE_REF
{
#ifdef __cplusplus
    explicit IORING_HANDLE_REF(HANDLE h)
        : Kind(IORING_REF_KIND::IORING_REF_RAW)
        , Handle(h)
    {}

    explicit IORING_HANDLE_REF(UINT32 index)
        : Kind(IORING_REF_KIND::IORING_REF_REGISTERED)
        , Handle(index)
    {}
#endif

    IORING_REF_KIND Kind;
    union HandleUnion
    {
#ifdef __cplusplus
        HandleUnion(HANDLE h)
            : Handle(h)
        {}

        HandleUnion(UINT32 index)
            : Index(index)
        {}
#endif
        // Handle to the file object if Kind == IORING_REF_RAW
        HANDLE Handle;

        // Index of registered file handle if Kind == IORING_REF_REGISTERED
        UINT32 Index;
    } Handle;
} IORING_HANDLE_REF;

#ifdef __cplusplus
#define IoRingHandleRefFromHandle(h) IORING_HANDLE_REF(static_cast<HANDLE>(h))
#define IoRingHandleRefFromIndex(i) IORING_HANDLE_REF(static_cast<UINT32>(i))
#else
#define IoRingHandleRefFromHandle(h) {IORING_REF_RAW, {.Handle = h}}
#define IoRingHandleRefFromIndex(i) {IORING_REF_REGISTERED, {.Index = i}}
#endif

typedef struct IORING_BUFFER_REF
{
#ifdef __cplusplus
    explicit IORING_BUFFER_REF(void* address)
        : Kind(IORING_REF_KIND::IORING_REF_RAW)
        , Buffer(address)
    {}

    explicit IORING_BUFFER_REF(IORING_REGISTERED_BUFFER registeredBuffer)
        : Kind(IORING_REF_KIND::IORING_REF_REGISTERED)
        , Buffer(registeredBuffer)
    {}

    IORING_BUFFER_REF(UINT32 index, UINT32 offset)
        : IORING_BUFFER_REF(IORING_REGISTERED_BUFFER{index, offset})
    {}
#endif

    IORING_REF_KIND Kind;
    union BufferUnion
    {
#ifdef __cplusplus
        BufferUnion(void* address)
            : Address(address)
        {}

        BufferUnion(IORING_REGISTERED_BUFFER indexAndOffset)
            : IndexAndOffset(indexAndOffset)
        {}
#endif
        // Address of the buffer if Kind == IORING_REF_RAW
        void* Address;

        // Registered buffer details if Kind == IORING_REF_REGISTERED
        IORING_REGISTERED_BUFFER IndexAndOffset;
    }Buffer;
} IORING_BUFFER_REF;

#ifdef __cplusplus
#define IoRingBufferRefFromPointer(p) IORING_BUFFER_REF(static_cast<void*>(p))
#define IoRingBufferRefFromIndexAndOffset(i,o) IORING_BUFFER_REF((i),(o))
#else
#define IoRingBufferRefFromPointer(p) {IORING_REF_RAW, {.Address = p}}
#define IoRingBufferRefFromIndexAndOffset(i,o) {IORING_REF_REGISTERED, {.IndexAndOffset = {(i),(o)}}}
#endif

typedef struct IORING_CQE
{
    UINT_PTR UserData;
    HRESULT ResultCode;
    ULONG_PTR Information;
} IORING_CQE;

#endif // NTDDI_VERSION >= NTDDI_WIN10_CO
#endif // ifdef NTDDI_WIN10_CO

#ifdef __cplusplus
extern "C" {
#endif

STDAPI QueryIoRingCapabilities(_Out_ IORING_CAPABILITIES* capabilities);

STDAPI_(BOOL) IsIoRingOpSupported(_In_ HIORING ioRing, IORING_OP_CODE op);

STDAPI
CreateIoRing(
    IORING_VERSION ioringVersion,
    IORING_CREATE_FLAGS flags,
    UINT32 submissionQueueSize,
    UINT32 completionQueueSize,
    _Out_ HIORING* h
    );

STDAPI GetIoRingInfo(_In_ HIORING ioRing, _Out_ IORING_INFO* info);

STDAPI SubmitIoRing(_In_ HIORING ioRing, UINT32 waitOperations, UINT32 milliseconds, _Out_opt_ UINT32* submittedEntries);

STDAPI CloseIoRing(_In_ _Post_ptr_invalid_ HIORING ioRing);

STDAPI PopIoRingCompletion(_In_ HIORING ioRing, _Out_ IORING_CQE* cqe);

STDAPI SetIoRingCompletionEvent(_In_ HIORING ioRing, _In_ HANDLE hEvent);

// Submission Queue entry builders

// Builds a submission queue entry for IORING_OP_CANCEL
STDAPI BuildIoRingCancelRequest(_In_ HIORING ioRing, _In_ IORING_HANDLE_REF file, UINT_PTR opToCancel, UINT_PTR userData);

// Builds a submission queue entry for IORING_OP_READ
STDAPI
BuildIoRingReadFile(
    _In_ HIORING ioRing,
    IORING_HANDLE_REF fileRef,
    IORING_BUFFER_REF dataRef,
    UINT32 numberOfBytesToRead,
    UINT64 fileOffset,
    UINT_PTR userData,
    IORING_SQE_FLAGS sqeFlags
    );

// Builds a submission queue entry for IORING_OP_REGISTER_FILES
STDAPI
BuildIoRingRegisterFileHandles(
    _In_ HIORING ioRing,
    UINT32 count,
    _In_reads_(count) HANDLE const handles[],
    UINT_PTR userData
    );

// Builds a submission queue entry for IORING_OP_REGISTER_BUFFERS
STDAPI
BuildIoRingRegisterBuffers(
    _In_ HIORING ioRing,
    UINT32 count,
    _In_reads_(count) IORING_BUFFER_INFO const buffers[],
    UINT_PTR userData
    );

#pragma region api-ms-win-core-ioring-l1-1-1
STDAPI
BuildIoRingWriteFile(
    _In_ HIORING ioRing,
    IORING_HANDLE_REF fileRef,
    IORING_BUFFER_REF bufferRef,
    UINT32 numberOfBytesToWrite,
    UINT64 fileOffset,
    FILE_WRITE_FLAGS writeFlags,
    UINT_PTR userData,
    IORING_SQE_FLAGS sqeFlags
    );

STDAPI
BuildIoRingFlushFile(
    _In_ HIORING ioRing,
    IORING_HANDLE_REF fileRef,
    FILE_FLUSH_MODE flushMode,
    UINT_PTR userData,
    IORING_SQE_FLAGS sqeFlags
    );

#pragma endregion // api-ms-win-core-ioring-l1-1-1

#pragma region api-ms-win-core-ioring-l1-1-2
STDAPI
BuildIoRingReadFileScatter(
    _In_ HIORING ioRing,
    IORING_HANDLE_REF fileRef,
    UINT32 segmentCount,
    _In_reads_(segmentCount) FILE_SEGMENT_ELEMENT segmentArray[],
    UINT32 numberOfBytesToRead,
    UINT64 fileOffset,
    UINT_PTR userData,
    IORING_SQE_FLAGS sqeFlags
    );

STDAPI
BuildIoRingWriteFileGather(
    _In_ HIORING ioRing,
    IORING_HANDLE_REF fileRef,
    UINT32 segmentCount,
    _In_reads_(segmentCount) FILE_SEGMENT_ELEMENT segmentArray[],
    UINT32 numberOfBytesToWrite,
    UINT64 fileOffset,
    FILE_WRITE_FLAGS writeFlags,
    UINT_PTR userData,
    IORING_SQE_FLAGS sqeFlags
    );

#pragma endregion // api-ms-win-core-ioring-l1-1-2

#ifdef __cplusplus
} //extern "C"
#endif

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)
#pragma endregion // Application Family or OneCore Family

#endif // _APISET_IORING_

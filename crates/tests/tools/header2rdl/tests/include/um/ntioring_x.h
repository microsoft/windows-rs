/*++

Copyright (c) Microsoft Corporation

Module Name:

    ntioring_x.w

Abstract:

    This is the header file defining the data structures and declaring the API
    interfaces for implementing IoRing to provide high performant asynchronous
    IO interface.  This is a public header.

--*/

#pragma once

#ifdef __cplusplus
extern "C" {
#endif

#if defined(NTDDI_WIN10_CO) && (NTDDI_VERSION >= NTDDI_WIN10_CO)

typedef enum IORING_VERSION {

    IORING_VERSION_INVALID = 0,
    IORING_VERSION_1,

    /// <summary>Minor update</summary>
    /// <remarks>
    /// Fixes a bug where user provided completion event may not be signaled
    /// even if the completion queue transitions from empty to non-empty because
    /// of a race condition. In earlier version please do a timed wait to work
    /// around this issue.
    /// </remarks>
    IORING_VERSION_2,

#if defined(NTDDI_WIN10_NI) && (NTDDI_VERSION >= NTDDI_WIN10_NI)
    /// <summary>Major update; Adds write, FLUSH and DRAIN support</summary>
    IORING_VERSION_3 = 300,
#endif

    /// <summary>Major update; Adds scatter-gather support</summary>
    IORING_VERSION_4 = 400,
} IORING_VERSION;

/// <summary>
/// Flags indicating functionality supported by a given implementation
/// </summary>
typedef enum IORING_FEATURE_FLAGS {

    /// <summary>No specific functionality for the implementation</summary>
    IORING_FEATURE_FLAGS_NONE = 0,

    /// <summary>
    /// IoRing support is emulated in User Mode (not directly supported by KM)
    /// </summary>
    /// <remarks>
    /// When this flag is set there is no underlying kernel support for IoRing.
    /// However, a user mode emulation layer is available to provide application
    /// compatibility, without the benefit of kernel support.  This provides
    /// application compatibility at the expense of performance. Thus, it allows
    /// apps to make a choice at run-time.
    /// </remarks>
    IORING_FEATURE_UM_EMULATION = 0x00000001,

    /// <summary>
    /// If this flag is present the SetIoRingCompletionEvent API is available
    /// and supported
    /// </summary>
    IORING_FEATURE_SET_COMPLETION_EVENT = 0x00000002,
} IORING_FEATURE_FLAGS;
DEFINE_ENUM_FLAG_OPERATORS( IORING_FEATURE_FLAGS )

/// <summary>Values for a submission queue entry opcode</summary>
/// <remarks>
/// To maintain the versioning strategy and compatibility, opcodes are never
/// re-used.  New values may be added, thus they always increment as new opcodes
/// are added.  Some codes may be deprecated and replaced with a new code but
/// the actual op code value is never re-used.
/// </remarks>
typedef enum IORING_OP_CODE {

    /// <summary>Do not perform any I/O</summary>
    /// <remarks>
    /// Useful for testing overhead performance and draining the queue
    /// </remarks>
    IORING_OP_NOP,

    /// <summary>Read from a file to a buffer</summary>
    IORING_OP_READ,

    /// <summary>Registers an array of file HANDLEs with the IoRing</summary>
    /// <remarks>
    /// If any existing registration exists, it is completely replaced by the
    /// registration for this opcode. Any entries in the array with
    /// INVALID_HANDLE_VALUE are sparse entries, and not used. This allows
    /// effectively releasing one or more of the previously registered files.
    /// Unregistration of all current files is accomplished by providing zero
    /// length array.
    ///
    /// The input array must remain valid until the operation completes. The
    /// change impacts all entries in the queue after this completes. (E.g.,
    /// this implicitly has "link" semantics in that any subsequent entry will
    /// not start until after this is completed)
    /// </remarks>
    IORING_OP_REGISTER_FILES,

    /// <summary>
    /// Registers an array of IORING_BUFFER_INFO with the IoRing
    /// </summary>
    /// <remarks>
    /// If any existing registration exists, it is completely replaced by the
    /// registration for this opcode. Any entries in the array with Address=NULL
    /// and Length=0 are sparse entries, and not used. This allows effectively
    /// releasing one or more of the previously registered buffers.
    /// Unregistration of all current buffers is accomplished by providing zero
    /// length array.
    ///
    /// The input array must remain valid until the operation completes. The
    /// change impacts all entries in the queue after this completes. E.g. this
    /// implicitly has "link" semantics in that any subsequent entry will not
    /// start until after this completes.
    /// </remarks>
    IORING_OP_REGISTER_BUFFERS,

    /// <summary>Requests cancellation of a previously submitted operation</summary>
    /// <remarks>
    /// This attempts to cancel a previously submitted operation. The UserData for the
    /// operation to cancel is used to identify the operation.
    /// </remarks>
    IORING_OP_CANCEL,

#if defined(NTDDI_WIN10_NI) && (NTDDI_VERSION >= NTDDI_WIN10_NI)
    /// <summary>Write to a file from a buffer</summary>
    IORING_OP_WRITE,

    /// <summary>Flush buffers for a file</summary>
    IORING_OP_FLUSH,
#endif

    /// <summary>Read from a file to buffer segments</summary>
    IORING_OP_READ_SCATTER,

    /// <summary>Write to a file from buffer segments</summary>
    IORING_OP_WRITE_GATHER,
} IORING_OP_CODE;

/// <summary>Buffer data for registering buffers with an IoRing</summary>
typedef struct IORING_BUFFER_INFO {
    void* Address;
    UINT32 Length;
} IORING_BUFFER_INFO;

/// <summary>Structure for a registered buffer</summary>
/// <remarks>
/// With a preregistered buffer both an index for the buffer and an offset
/// are used to determine the final address of an I/O operation. This allows
/// for registration of a larger buffer and using portions of it for different
/// operations that operate in parallel.
/// </remarks>
typedef struct IORING_REGISTERED_BUFFER {

    // Index of pre-registered buffer
    UINT32 BufferIndex;

    // Offset into the pre-registered buffer
    UINT32 Offset;
} IORING_REGISTERED_BUFFER;

/// <summary>Requests SubmitIoRing to wait for all operations to complete</summary>
/// <remarks>
/// The constant value for waitOperations parameter in SubmitIoRing to wait for
/// all the entries submitted so far, to be completed.
/// </remarks>
#define IORING_SUBMIT_WAIT_ALL                  MAXUINT32

#endif // (NTDDI_VERSION >= NTDDI_WIN10_CO)

#ifdef __cplusplus
}
#endif

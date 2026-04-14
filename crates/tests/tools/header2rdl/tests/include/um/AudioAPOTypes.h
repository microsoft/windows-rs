// Microsoft Windows
// Copyright (C) Microsoft Corporation. All rights reserved.
//
#pragma once
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


typedef LONGLONG    HNSTIME;   // hundred nanosecond unit
typedef LONGLONG    MFTIME;    // hundred nanosecond unit

typedef float       FLOAT32;   // float
typedef double      FLOAT64;   // double

// Validation flags for APO_CONNECTION_PROPERTY. Each APO connection has
// an APO_CONNECTION_PROPERTY structure associated with it. The buffer
// for each connection may be either invalid, valid, or silent.
typedef enum APO_BUFFER_FLAGS
{
    // BUFFER_INVALID means that there is no valid data in  the connection
    // buffer. The buffer pointer will still be valid and capable of holding
    // the amount of valid audio data specified in the APO_CONNECTION_PROPERTY.
    // The processor will mark every connection BUFFER_INVALID before running
    // the IAudioOutputEndpoint::GetOutputDataPointer,
    // IAudioInputEndpointRT::GetInputDataPointer, or
    // IAudioProcessingObjectRT::APOProcess each time its IAudioProcess::Process
    // routine is called.
    BUFFER_INVALID  = 0,

    // Connection buffer has valid data. This is the "normal" operational
    // state of the connection buffer. An APO will set this flag once it
    // writes valid data into a buffer.
    BUFFER_VALID    = 1,

    // The connection buffer should be treated as if it contains silence.
    // APOs will mark their output connection buffers as silent (instead
    // of writing silence into the buffer) if they generate a buffer of
    // silence. This typically only happens when the buffer(s) going in
    // are marked BUFFER_SILENT.
    BUFFER_SILENT   = 2
} APO_BUFFER_FLAGS;

// This structure contains the dynamically changing connection properties.
// The connection between APOs ends up resolving to the APO_CONNECTION_PROPERTY
// structure for the IAudioProcessingObjectRT::APOProcess call.  This structure
// is passed in IAudioInputEndpointRT::GetInputDataPointer and
// IAudioOutputEndpointRT::ReleaseOutputDataPointer.
typedef struct APO_CONNECTION_PROPERTY
{
    // The connection buffer. APOs use this buffer to read and write
    // audio data.
    //
    // Alignment required
    // (128 bit or frame aligned)
    //            |
    //      +-----+
    //      V
    //      +-------------------------------------------------------------+
    //      |                                                             |
    //      |                                                             |
    //      |                     audio buffer                            |
    //      |                                                             |
    //      |                                                             |
    //      +-------------------------------------------------------------+
    //      ^
    //      |
    //   pBuffer
    //
    UINT_PTR pBuffer;

    // Number of valid frames in the connection buffer. This must
    // be less than or equal to APO_CONNECTION_DESCRIPTOR.u32MaxFrameCount.
    // An APO will use the valid frame count to determine how much data to
    // process on an input buffer. An APO will set the valid frame count
    // upon writing data into its output connection(s).
    UINT32 u32ValidFrameCount;

    // Connection flags for this buffer.  Tells APOs if the buffer is valid,
    // in valid, or silent. See APO_BUFFER_FLAGS.
    APO_BUFFER_FLAGS u32BufferFlags;

    // A tag identifying a valid APO_CONNECTION_PROPERTY structure.
    UINT32 u32Signature;
} APO_CONNECTION_PROPERTY;

// This structure defines V2 of the APO_CONNECTION_PROPERTY.
typedef struct APO_CONNECTION_PROPERTY_V2
{
    APO_CONNECTION_PROPERTY   property;
    UINT64                    u64QPCTime;
} APO_CONNECTION_PROPERTY_V2;

#ifndef _AUDIO_CURVE_TYPE_
#define _AUDIO_CURVE_TYPE_
typedef enum {
    AUDIO_CURVE_TYPE_NONE           = 0,
    AUDIO_CURVE_TYPE_WINDOWS_FADE   = 1,
} AUDIO_CURVE_TYPE;
#endif // _AUDIO_CURVE_TYPE_

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


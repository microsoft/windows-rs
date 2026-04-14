/*-------------------------------------------------------------------------------------
 *
 * Copyright (c) Microsoft Corporation
 *
 *-------------------------------------------------------------------------------------*/

// Perception Device Core
// Common definitions that are shared between DDI interfaces and the perception device api.

#ifdef _MSC_VER
#pragma once
#endif //_MSC_VER

#if NTDDI_VERSION >= NTDDI_WIN10_19H1
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES | WINAPI_FAMILY_SYSTEM)

#ifdef __cplusplus

struct PERCEPTION_PAYLOAD_FIELD
{
    // Identifies both the format of the data, as well as the  semantic
    // meaning / purpose of the data.  Fields with different GUIDs may have the
    // same format, but they will have different semantic meaning.
    // A given FieldId GUID should only appear in a descriptor once.
    GUID FieldId;

    // Offset / Size in bytes of the field within the payload.
    // Fields may overlap or be contained by other fields.
    UINT OffsetInBytes;
    UINT SizeInBytes;
};

// Timestamps for state stream payload.
// Required by the platform to fulfill state queries in the kernel filter driver.
struct PERCEPTION_STATE_STREAM_TIMESTAMPS
{
    // The timestamp in QPC ticks of the state report.
    // Timestamps must always be increasing, with no duplicates or reversions.
    LONGLONG InputTimestampInQpcCounts;

    // The timestamp in QPC ticks when this state is stored into the circular
    // buffer by the filter driver.
    // The function driver must set this to zero.
    LONGLONG AvailableTimestampInQpcCounts;
};

#endif

//
// FieldId for the timestamps in a state stream's payload.
//
// The function driver can choose where in the payload to store the timestamps,
// but it must use the PERCEPTION_STATE_STREAM_TIMESTAMPS structure,
// and refer to it with PERCEPTIONFIELD_StateStream_TimeStamps in the descriptor.
//
// Field data type: PERCEPTION_STATE_STREAM_TIMESTAMPS
//
// {AA886119-F32F-49BF-92CA-F9DDF784D297}
DEFINE_GUID( PERCEPTIONFIELD_StateStream_TimeStamps,
    0xaa886119, 0xf32f, 0x49bf, 0x92, 0xca, 0xf9, 0xdd, 0xf7, 0x84, 0xd2, 0x97);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES | WINAPI_FAMILY_SYSTEM) */
#endif /* NTDDI_VERSION >= NTDDI_WIN10_19H1 */

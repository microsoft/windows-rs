
/*++

Copyright 2004 (c) Microsoft Corporation. All rights reserved.

Module Name:

    evntcons.h

Abstract:

    This defines the event consumer API

Revision History:

    Insung Park (insungp) 26-Aug-2004
        Created the file.

--*/

#ifndef _EVNTCONS_H_
#define _EVNTCONS_H_

#pragma once
#include <winapifamily.h>

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#include <wmistr.h>
#include <evntrace.h>
#include <evntprov.h>

#ifndef EVNTCONS_INLINE
#define EVNTCONS_INLINE __inline
#endif

#ifdef __cplusplus
extern "C" {
#endif

#if defined (_MSC_VER)
#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4820) // padding added
#endif
#pragma warning(disable:4201) // nonstandard extension used : nameless struct/union
#pragma warning(disable:4214) // nonstandard extension used : bit field types other then int
#endif

#define EVENT_HEADER_EXT_TYPE_RELATED_ACTIVITYID   0x0001
#define EVENT_HEADER_EXT_TYPE_SID                  0x0002
#define EVENT_HEADER_EXT_TYPE_TS_ID                0x0003
#define EVENT_HEADER_EXT_TYPE_INSTANCE_INFO        0x0004
#define EVENT_HEADER_EXT_TYPE_STACK_TRACE32        0x0005
#define EVENT_HEADER_EXT_TYPE_STACK_TRACE64        0x0006
#define EVENT_HEADER_EXT_TYPE_PEBS_INDEX           0x0007
#define EVENT_HEADER_EXT_TYPE_PMC_COUNTERS         0x0008
#define EVENT_HEADER_EXT_TYPE_PSM_KEY              0x0009
#define EVENT_HEADER_EXT_TYPE_EVENT_KEY            0x000A
#define EVENT_HEADER_EXT_TYPE_EVENT_SCHEMA_TL      0x000B
#define EVENT_HEADER_EXT_TYPE_PROV_TRAITS          0x000C
#define EVENT_HEADER_EXT_TYPE_PROCESS_START_KEY    0x000D
#define EVENT_HEADER_EXT_TYPE_CONTROL_GUID         0x000E
#define EVENT_HEADER_EXT_TYPE_QPC_DELTA            0x000F
#define EVENT_HEADER_EXT_TYPE_CONTAINER_ID         0x0010
#define EVENT_HEADER_EXT_TYPE_STACK_KEY32          0x0011
#define EVENT_HEADER_EXT_TYPE_STACK_KEY64          0x0012
#define EVENT_HEADER_EXT_TYPE_MAX                  0x0013

#ifndef EVENT_HEADER_EXTENDED_DATA_ITEM_DEF
#define EVENT_HEADER_EXTENDED_DATA_ITEM_DEF
typedef struct _EVENT_HEADER_EXTENDED_DATA_ITEM {

    USHORT      Reserved1;                      // Reserved for internal use
    USHORT      ExtType;                        // Extended info type
    struct {
        USHORT  Linkage             :  1;       // Indicates additional extended
                                                // data item
        USHORT  Reserved2           : 15;
    };
    USHORT      DataSize;                       // Size of extended info data
    ULONGLONG   DataPtr;                        // Pointer to extended info data

} EVENT_HEADER_EXTENDED_DATA_ITEM, *PEVENT_HEADER_EXTENDED_DATA_ITEM;
#endif

//
// Structures for extended items.
//

typedef struct _EVENT_EXTENDED_ITEM_INSTANCE {
    ULONG InstanceId;
    ULONG ParentInstanceId;
    GUID  ParentGuid;
} EVENT_EXTENDED_ITEM_INSTANCE, *PEVENT_EXTENDED_ITEM_INSTANCE;

typedef struct _EVENT_EXTENDED_ITEM_RELATED_ACTIVITYID {
    GUID  RelatedActivityId;
} EVENT_EXTENDED_ITEM_RELATED_ACTIVITYID, *PEVENT_EXTENDED_ITEM_RELATED_ACTIVITYID;

typedef struct _EVENT_EXTENDED_ITEM_TS_ID {
    ULONG SessionId;
} EVENT_EXTENDED_ITEM_TS_ID, *PEVENT_EXTENDED_ITEM_TS_ID;

typedef struct _EVENT_EXTENDED_ITEM_STACK_TRACE32 {
    ULONG64 MatchId;
    ULONG   Address[ANYSIZE_ARRAY];
} EVENT_EXTENDED_ITEM_STACK_TRACE32, *PEVENT_EXTENDED_ITEM_STACK_TRACE32;

typedef struct _EVENT_EXTENDED_ITEM_STACK_TRACE64 {
    ULONG64 MatchId;
    ULONG64 Address[ANYSIZE_ARRAY];
} EVENT_EXTENDED_ITEM_STACK_TRACE64, *PEVENT_EXTENDED_ITEM_STACK_TRACE64;

typedef struct _EVENT_EXTENDED_ITEM_STACK_KEY32 {
    ULONG64 MatchId;
    ULONG StackKey;
    ULONG Padding;
} EVENT_EXTENDED_ITEM_STACK_KEY32, *PEVENT_EXTENDED_ITEM_STACK_KEY32;

typedef struct _EVENT_EXTENDED_ITEM_STACK_KEY64 {
    ULONG64 MatchId;
    ULONG64 StackKey;
} EVENT_EXTENDED_ITEM_STACK_KEY64, *PEVENT_EXTENDED_ITEM_STACK_KEY64;

typedef struct _EVENT_EXTENDED_ITEM_PEBS_INDEX {
    ULONG64 PebsIndex;
} EVENT_EXTENDED_ITEM_PEBS_INDEX, *PEVENT_EXTENDED_ITEM_PEBS_INDEX;

typedef struct _EVENT_EXTENDED_ITEM_PMC_COUNTERS {
    ULONG64 Counter[ANYSIZE_ARRAY];
} EVENT_EXTENDED_ITEM_PMC_COUNTERS, *PEVENT_EXTENDED_ITEM_PMC_COUNTERS;

typedef struct _EVENT_EXTENDED_ITEM_PROCESS_START_KEY {
    ULONG64 ProcessStartKey;
} EVENT_EXTENDED_ITEM_PROCESS_START_KEY, *PEVENT_EXTENDED_ITEM_PROCESS_START_KEY;

typedef struct _EVENT_EXTENDED_ITEM_EVENT_KEY {
    ULONG64 Key;
} EVENT_EXTENDED_ITEM_EVENT_KEY, *PEVENT_EXTENDED_ITEM_EVENT_KEY;

#define EVENT_HEADER_PROPERTY_XML               0x0001
#define EVENT_HEADER_PROPERTY_FORWARDED_XML     0x0002
#define EVENT_HEADER_PROPERTY_LEGACY_EVENTLOG   0x0004
#define EVENT_HEADER_PROPERTY_RELOGGABLE        0x0008

#define EVENT_HEADER_FLAG_EXTENDED_INFO         0x0001
#define EVENT_HEADER_FLAG_PRIVATE_SESSION       0x0002
#define EVENT_HEADER_FLAG_STRING_ONLY           0x0004
#define EVENT_HEADER_FLAG_TRACE_MESSAGE         0x0008
#define EVENT_HEADER_FLAG_NO_CPUTIME            0x0010
#define EVENT_HEADER_FLAG_32_BIT_HEADER         0x0020
#define EVENT_HEADER_FLAG_64_BIT_HEADER         0x0040
#define EVENT_HEADER_FLAG_DECODE_GUID           0x0080 // ProviderId is decode GUID.
#define EVENT_HEADER_FLAG_CLASSIC_HEADER        0x0100
#define EVENT_HEADER_FLAG_PROCESSOR_INDEX       0x0200

#ifndef EVENT_HEADER_DEF
#define EVENT_HEADER_DEF
typedef struct _EVENT_HEADER {

    USHORT              Size;                   // Event Size
    USHORT              HeaderType;             // Header Type
    USHORT              Flags;                  // Flags
    USHORT              EventProperty;          // User given event property
    ULONG               ThreadId;               // Thread Id
    ULONG               ProcessId;              // Process Id
    LARGE_INTEGER       TimeStamp;              // Event Timestamp
    GUID                ProviderId;             // Provider Id
    EVENT_DESCRIPTOR    EventDescriptor;        // Event Descriptor
    union {
        struct {
            ULONG       KernelTime;             // Kernel Mode CPU ticks
            ULONG       UserTime;               // User mode CPU ticks
        } DUMMYSTRUCTNAME;
        ULONG64         ProcessorTime;          // Processor Clock
                                                // for private session events
    } DUMMYUNIONNAME;
    GUID                ActivityId;             // Activity Id

} EVENT_HEADER, *PEVENT_HEADER;
#endif

#ifndef EVENT_RECORD_DEF
#define EVENT_RECORD_DEF
typedef struct _EVENT_RECORD {

    EVENT_HEADER        EventHeader;            // Event header
    ETW_BUFFER_CONTEXT  BufferContext;          // Buffer context
    USHORT              ExtendedDataCount;      // Number of extended
                                                // data items
    USHORT              UserDataLength;         // User data length
    PEVENT_HEADER_EXTENDED_DATA_ITEM            // Pointer to an array of
                        ExtendedData;           // extended data items
    PVOID               UserData;               // Pointer to user data
    PVOID               UserContext;            // Context from OpenTrace
} EVENT_RECORD, *PEVENT_RECORD;

typedef const EVENT_RECORD *PCEVENT_RECORD;
#endif

#define EVENT_ENABLE_PROPERTY_SID                       0x00000001
#define EVENT_ENABLE_PROPERTY_TS_ID                     0x00000002
#define EVENT_ENABLE_PROPERTY_STACK_TRACE               0x00000004
#define EVENT_ENABLE_PROPERTY_PSM_KEY                   0x00000008
#define EVENT_ENABLE_PROPERTY_IGNORE_KEYWORD_0          0x00000010
#define EVENT_ENABLE_PROPERTY_PROVIDER_GROUP            0x00000020
#define EVENT_ENABLE_PROPERTY_ENABLE_KEYWORD_0          0x00000040
#define EVENT_ENABLE_PROPERTY_PROCESS_START_KEY         0x00000080
#define EVENT_ENABLE_PROPERTY_EVENT_KEY                 0x00000100
#define EVENT_ENABLE_PROPERTY_EXCLUDE_INPRIVATE         0x00000200
#define EVENT_ENABLE_PROPERTY_ENABLE_SILOS              0x00000400
#define EVENT_ENABLE_PROPERTY_SOURCE_CONTAINER_TRACKING 0x00000800 

//
// Consumer API
//

#define PROCESS_TRACE_MODE_REAL_TIME                0x00000100
#define PROCESS_TRACE_MODE_RAW_TIMESTAMP            0x00001000
#define PROCESS_TRACE_MODE_EVENT_RECORD             0x10000000

EVNTCONS_INLINE
ULONG
GetEventProcessorIndex (
    _In_ PCEVENT_RECORD EventRecord
    )
{
    if ((EventRecord->EventHeader.Flags & EVENT_HEADER_FLAG_PROCESSOR_INDEX) != 0) {
        return EventRecord->BufferContext.ProcessorIndex;

    } else {
        return EventRecord->BufferContext.ProcessorNumber;
    }
}

//
// Provider Trait APIs
//

typedef enum {
    EtwProviderTraitTypeGroup  = 1, // Provider group GUID.
    EtwProviderTraitDecodeGuid = 2, // Decode GUID (when different from control GUID)
    EtwProviderTraitTypeMax
} ETW_PROVIDER_TRAIT_TYPE;

EVNTCONS_INLINE
VOID
EtwGetTraitFromProviderTraits(
    _In_ PVOID ProviderTraits,
    _In_ UCHAR TraitType,
    _Out_ PVOID* Trait,
    _Out_ PUSHORT Size
    )
{
    USHORT const ByteCount = *(USHORT UNALIGNED*)ProviderTraits;
    PUCHAR Ptr = (PUCHAR)ProviderTraits;
    UCHAR const* PtrEnd = Ptr + ByteCount;

    *Trait = NULL;
    *Size = 0;

    //
    // Abort on invalid size.
    //

    if (ByteCount < 3) {
        return;
    }

    //
    // Skip byte counts
    //

    Ptr += 2;

    //
    // Skip the Provider Name, including the Null termination
    //

    Ptr += strnlen((PCSTR)Ptr, ByteCount - 3u);
    Ptr += 1;

    //
    // Loop through the rest of the traits until one of the
    // desired type is located.
    //

    while (Ptr < PtrEnd) {
        USHORT const TraitByteCount = *(USHORT const UNALIGNED*)Ptr;

        //
        // Abort on invalid trait size.
        //

        if (TraitByteCount < 3) {
            return;
        }

        if ((Ptr[2] == TraitType) &&
            (Ptr + TraitByteCount <= PtrEnd)) {

            *Trait = (PVOID)(Ptr + 3);
            *Size = TraitByteCount - 3u;
            return;
        }

        Ptr += TraitByteCount;
    }

    return;
}

//
// Event Security APIs
//

typedef enum {
    EventSecuritySetDACL,
    EventSecuritySetSACL,
    EventSecurityAddDACL,
    EventSecurityAddSACL,
    EventSecurityMax
} EVENTSECURITYOPERATION;

#if (WINVER >= _WIN32_WINNT_LONGHORN)
ULONG
EVNTAPI
EventAccessControl(
    _In_ LPGUID Guid,
    _In_ ULONG Operation,
    _In_ PSID Sid,
    _In_ ULONG Rights,
    _In_ BOOLEAN AllowOrDeny
    );
#endif

#if (WINVER >= _WIN32_WINNT_LONGHORN)
ULONG
EVNTAPI
EventAccessQuery(
    _In_ LPGUID Guid,
    _Out_writes_bytes_opt_(*BufferSize) PSECURITY_DESCRIPTOR Buffer,
    _Inout_ PULONG BufferSize
    );
#endif

#if (WINVER >= _WIN32_WINNT_LONGHORN)
ULONG
EVNTAPI
EventAccessRemove(
    _In_ LPGUID Guid
    );
#endif

#if defined (_MSC_VER)
#if _MSC_VER >= 1200
#pragma warning(pop)          // restore 4201,4214
#else
#pragma warning(default:4201) // nonstandard extension used : nameless struct/union
#pragma warning(default:4214) // nonstandard extension used : bit field types other then int
#endif
#endif

#ifdef __cplusplus
}
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif

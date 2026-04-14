/*++
Copyright (c) Microsoft Corporation.  All rights reserved.

Module Name:

    perflib.h

Abstract:

    Public headers for PERFLIB provider and consumer APIs.
--*/

#ifndef _PERFLIB_H_
#define _PERFLIB_H_

#pragma once
#include <winapifamily.h>

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#ifdef __cplusplus
extern "C" {
#endif

// ****************************************************************************
// PERFLIB V2 provider side published literals, data structures and APIs.
// ****************************************************************************

// This is used in generated PERF_COUNTERSET_INFO structure to declare provider type.
// Kernel provider is reserved for Microsoft internal use.
// Driver provider and user-mode provider literals will be published.
//
#define PERF_PROVIDER_USER_MODE   0
#define PERF_PROVIDER_KERNEL_MODE 1
#define PERF_PROVIDER_DRIVER      2

// These are used for PERF_COUNTERSET_INFO::InstanceType value. That is, whether the CounterSet
// allows multiple instances (for example, Process, PhysicalDisk, etc) or only single default instance
// (for example, Memory, TCP, etc).
//
#define PERF_COUNTERSET_FLAG_MULTIPLE             2
#define PERF_COUNTERSET_FLAG_AGGREGATE            4
#define PERF_COUNTERSET_FLAG_HISTORY              8
#define PERF_COUNTERSET_FLAG_INSTANCE            16

#define PERF_COUNTERSET_SINGLE_INSTANCE          0 /*
    Single instance.
    Only one instance of the counterset is active on the system at any time
    while the system is running. */
#define PERF_COUNTERSET_MULTI_INSTANCES          (PERF_COUNTERSET_FLAG_MULTIPLE) /*
    Multiple instances.
    There can be several instances of the counterset active on the system at
    any time while the system is running. */
#define PERF_COUNTERSET_SINGLE_AGGREGATE         (PERF_COUNTERSET_FLAG_AGGREGATE) /*
    Global aggregate.
    Performs an aggregation operation that is specified in the performance
    counter definition. The aggregation operation is performed on the client
    side for each counter in the counterset across all available and active
    instances of the counterset in the system. */
#define PERF_COUNTERSET_MULTI_AGGREGATE          (PERF_COUNTERSET_FLAG_AGGREGATE | PERF_COUNTERSET_FLAG_MULTIPLE) /*
    Multiple-instance aggregate.
    Performs an aggregation operation that is specified in the performance
    counter definition. The aggregation operation is performed on the client
    side for each performance counter in the counterset across a
    client-specified set of instances of that counterset. For example, a
    client can average the value of counter "A" from counterset instances "1",
    "2", and "5". */
#define PERF_COUNTERSET_SINGLE_AGGREGATE_HISTORY (PERF_COUNTERSET_FLAG_HISTORY | PERF_COUNTERSET_SINGLE_AGGREGATE) /*
    Global aggregate history.
    Performs an aggregation operation that is specified in the counter
    definition. The aggregation operation is performed on the client side for
    each performance counter in the counterset across all available instances
    of the counterset. The result of the aggregation operation can then be
    cached by the consumer and referenced for later use. For example, if a
    counter is deleted by the server between client queries, the client can
    use the value of the counter that was obtained in the last query for the
    aggregation operation. */
#define PERF_COUNTERSET_INSTANCE_AGGREGATE       (PERF_COUNTERSET_MULTI_AGGREGATE | PERF_COUNTERSET_FLAG_INSTANCE) /*
    Instance aggregate. Not implemented. */

/*
The aggregation function to be performed by the client on the counter if the
counterset to which the counter belongs is of type Global Aggregate, Multiple
Instance Aggregate, or Global Aggregate History. The client specifies across
which counter instances the aggregation is performed if the counterset type
is Multiple Instance Aggregate; otherwise, the client MUST aggregate values
across all instances of the counterset. One of the following values MUST be
specified.
*/
#define PERF_AGGREGATE_UNDEFINED  0 /* Undefined. */
#define PERF_AGGREGATE_TOTAL      1 /* Total.
    The sum of the values of the returned counter instances. */
#define PERF_AGGREGATE_AVG        2 /* Average.
    The average of the values of the returned counter instances. */
#define PERF_AGGREGATE_MIN        3 /* Minimum.
    The minimum value of the returned counter instance values. */
#define PERF_AGGREGATE_MAX        4 /* Maximum.
    The maximum value of the returned counter instance values. */

/*
These are possible attributes used in generated PERF_COUNTER_INFO::Attrib value.
Note that only certain combinations of the preceding possible values are allowed:
- PERF_ATTRIB_BY_REFERENCE can be specified with any other value.
- PERF_ATTRIB_NO_DISPLAYABLE MUST NOT be specified with
  PERF_ATTRIB_NO_GROUP_SEPARATOR, PERF_ATTRIB_DISPLAY_AS_REAL or
  PERF_ATTRIB_DISPLAY_AS_HEX.
- PERF_ATTRIB_DISPLAY_AS_HEX MUST not be specified with
  PERF_ATTRIB_NO_GROUP_SEPARATOR or PERF_ATTRIB_DISPLAY_AS_REAL.
*/
#define PERF_ATTRIB_BY_REFERENCE       0x0000000000000001 /* Reference.
    The query on the server MUST dereference the counter to obtain the value. */
#define PERF_ATTRIB_NO_DISPLAYABLE     0x0000000000000002 /* No display.
    Instructs the client consumer querying for performance counter data not to
    display the counter value. */
#define PERF_ATTRIB_NO_GROUP_SEPARATOR 0x0000000000000004 /* No group separator.
    Instructs the client consumer querying performance counter data to display
    the counter values as a single number without commas between digits. */
#define PERF_ATTRIB_DISPLAY_AS_REAL    0x0000000000000008 /* Display as real.
    Instructs the client consumer querying performance counter to display the
    counter value as a real number. */
#define PERF_ATTRIB_DISPLAY_AS_HEX     0x0000000000000010 /* Display as hexadecimal.
    Instructs the client consumer querying performance counter to display the
    counter value as a hexadecimal number. */

// Provider counterset is defined as a leading PERF_COUNTERSET_INFO structure followed by a sequence
// of PERF_COUNTER_INFO structures. Note that the structure block will be automatically generated
// by schema generation/parsing tool.
//
typedef struct _PERF_COUNTERSET_INFO {
    GUID   CounterSetGuid;
    GUID   ProviderGuid;
    ULONG  NumCounters;
    ULONG  InstanceType;
} PERF_COUNTERSET_INFO, * PPERF_COUNTERSET_INFO;

typedef struct _PERF_COUNTER_INFO {
    ULONG      CounterId;     // max of 64K counters per GUID instance
    ULONG      Type;
    ULONGLONG  Attrib;
    ULONG      Size;
    ULONG      DetailLevel;
    LONG       Scale;
    ULONG      Offset;         // overlays to give the actual counter
} PERF_COUNTER_INFO, * PPERF_COUNTER_INFO;

// PERF_COUNTERSET_INSTANCE block is returned from PerfCreateInstance() API call to identify specific
// instance of a counterset. The returned block is formed by PERF_COUNTERSET_INSTANCE structure followed
// by counter data block (layout defined by provider counterset template) and instance name string (if exists).
//
typedef struct _PERF_COUNTERSET_INSTANCE {
    GUID   CounterSetGuid;
    ULONG  dwSize;
    ULONG  InstanceId;
    ULONG  InstanceNameOffset;
    ULONG  InstanceNameSize;
} PERF_COUNTERSET_INSTANCE, * PPERF_COUNTERSET_INSTANCE;

// PERF_COUNTER_IDENTITY structure is used in customized notification callback. Whenever PERFLIB V2
// invokes customized notification callback, it passes wnode datablock (which contains WNODE_HEADER
// structure followed by other binary data) that contains the information providers can use.
//
// For PERF_ADD_COUNTER and PERF_REMOVE_COUNTER request, PERFLIB will pass PERF_COUNTER_IDENTITY block
// so that providers know which counter is added/removed. For other requests, currently only machine name
// is passed (so that providers can determine whether the request is for physical node or virtual node).
//
typedef struct _PERF_COUNTER_IDENTITY {
    GUID   CounterSetGuid;
    ULONG  BufferSize;
    ULONG  CounterId;
    ULONG  InstanceId;
    ULONG  MachineOffset;
    ULONG  NameOffset;
    ULONG  Reserved;
} PERF_COUNTER_IDENTITY, * PPERF_COUNTER_IDENTITY;

#define PERF_WILDCARD_COUNTER   0xFFFFFFFF
#define PERF_WILDCARD_INSTANCE  L"*"
#define PERF_AGGREGATE_INSTANCE L"_Total"
#define PERF_MAX_INSTANCE_NAME  1024

#define PERF_ADD_COUNTER            1
#define PERF_REMOVE_COUNTER         2
#define PERF_ENUM_INSTANCES         3
#define PERF_COLLECT_START          5
#define PERF_COLLECT_END            6
#define PERF_FILTER                 9

// Prototype for service request callback. Data providers register with PERFLIB V2 by passing a service
// request callback function that is called for all PERFLIB requests.
//
typedef ULONG (
#ifndef MIDL_PASS
WINAPI
#endif
* PERFLIBREQUEST)(
    IN ULONG  RequestCode,
    IN PVOID  Buffer,
    IN ULONG  BufferSize
);

// Usually PerfSetCounterSetInfo() calls is automatically generated PerfAutoStartUp() function (generated
// by schema generation/parsing tool) to inform PERFLIB the layout of specific counterset.
//
ULONG WINAPI
PerfStartProvider(
    _In_     LPGUID          ProviderGuid,
    _In_opt_ PERFLIBREQUEST  ControlCallback,
    _Out_    HANDLE        * phProvider
);

// Start PERFLIB V2 provider with customized memory allocation/free routines.
//
typedef LPVOID (CALLBACK * PERF_MEM_ALLOC)(IN SIZE_T AllocSize, IN LPVOID pContext);
typedef void (CALLBACK * PERF_MEM_FREE)(IN LPVOID pBuffer, IN LPVOID pContext);

typedef struct _PROVIDER_CONTEXT {
    DWORD          ContextSize; // should be sizeof(PERF_PROVIDER_CONTEXT)
    DWORD          Reserved;
    PERFLIBREQUEST ControlCallback;
    PERF_MEM_ALLOC MemAllocRoutine;
    PERF_MEM_FREE  MemFreeRoutine;
    LPVOID         pMemContext;
} PERF_PROVIDER_CONTEXT, * PPERF_PROVIDER_CONTEXT;

ULONG WINAPI
PerfStartProviderEx(
    _In_ LPGUID ProviderGuid,
    _In_opt_ PPERF_PROVIDER_CONTEXT ProviderContext,
    _Out_ PHANDLE Provider
    );

ULONG WINAPI
PerfStartProvider(
    _In_ LPGUID ProviderGuid,
    _In_opt_ PERFLIBREQUEST ControlCallback,
    _Out_ PHANDLE Provider
    );

ULONG WINAPI
PerfStopProvider(
    _In_ HANDLE ProviderHandle
    );

ULONG WINAPI
PerfSetCounterSetInfo(
    _In_ HANDLE ProviderHandle,
    _Inout_updates_bytes_(TemplateSize) PPERF_COUNTERSET_INFO Template,
    _In_ ULONG TemplateSize
    );

PPERF_COUNTERSET_INSTANCE WINAPI
PerfCreateInstance(
    _In_ HANDLE ProviderHandle,
    _In_ LPCGUID CounterSetGuid,
    _In_ PCWSTR Name,
    _In_ ULONG Id
    );

ULONG WINAPI
PerfDeleteInstance(
    _In_ HANDLE Provider,
    _In_ PPERF_COUNTERSET_INSTANCE InstanceBlock
    );

PPERF_COUNTERSET_INSTANCE WINAPI
PerfQueryInstance(
    _In_ HANDLE ProviderHandle,
    _In_ LPCGUID CounterSetGuid,
    _In_ PCWSTR Name,
    _In_ ULONG Id
    );

ULONG WINAPI
PerfSetCounterRefValue(
    _In_ HANDLE Provider,
    _Inout_ PPERF_COUNTERSET_INSTANCE Instance,
    _In_ ULONG CounterId,
    _In_ PVOID Address
    );

ULONG WINAPI
PerfSetULongCounterValue(
    _In_ HANDLE Provider,
    _Inout_ PPERF_COUNTERSET_INSTANCE Instance,
    _In_ ULONG CounterId,
    _In_ ULONG Value
    );

ULONG WINAPI
PerfSetULongLongCounterValue(
    _In_ HANDLE Provider,
    _Inout_ PPERF_COUNTERSET_INSTANCE Instance,
    _In_ ULONG CounterId,
    _In_ ULONGLONG Value
    );

ULONG WINAPI
PerfIncrementULongCounterValue(
    _In_ HANDLE Provider,
    _Inout_ PPERF_COUNTERSET_INSTANCE Instance,
    _In_ ULONG CounterId,
    _In_ ULONG Value
    );

ULONG WINAPI
PerfIncrementULongLongCounterValue(
    _In_ HANDLE Provider,
    _Inout_ PPERF_COUNTERSET_INSTANCE Instance,
    _In_ ULONG CounterId,
    _In_ ULONGLONG Value
    );

ULONG WINAPI
PerfDecrementULongCounterValue(
    _In_ HANDLE Provider,
    _Inout_ PPERF_COUNTERSET_INSTANCE Instance,
    _In_ ULONG CounterId,
    _In_ ULONG Value
    );

ULONG WINAPI
PerfDecrementULongLongCounterValue(
    _In_ HANDLE Provider,
    _Inout_ PPERF_COUNTERSET_INSTANCE Instance,
    _In_ ULONG CounterId,
    _In_ ULONGLONG Value
    );

// ****************************************************************************
// PERFLIB V2 consumer side published literals, data structures and APIs.
// ****************************************************************************

/*
The following APIs provide access to PERFLIB V2 performance counter data. The
data is structured according to the [MS-PCQ] (Performance Counter Query)
protocol. For details, refer to the [MS-PCQ] documentation:
http://msdn.microsoft.com/en-us/library/cc238290.aspx
*/

/*
A PERF_INSTANCE_HEADER block consists of a PERF_INSTANCE_HEADER structure
(containing the size of the block and the instance ID) immediately followed by
a nul-terminated utf-16le string (containing the instance name), followed by
padding such that the size of the block is a multiple of 8.

Each active instance of a counterset can be identified by the combination of
its instance name and instance identifier. Two active instances of a
counterset SHOULD NOT have the same combination of instance name and instance
identifier. However, clients should tolerate instances with duplicate name+id.

The PerfEnumerateCounterSetInstances function returns a sequence of
PERF_INSTANCE_HEADER blocks.

The PerfQueryCounterData function returns a PERF_DATA_HEADER block that may
contain PERF_INSTANCE_HEADER blocks (within the PERF_MULTI_INSTANCES block).
*/
typedef struct _PERF_INSTANCE_HEADER {
    ULONG Size;       // = sizeof(PERF_INSTANCE_HEADER) + sizeof(InstanceName) + sizeof(Padding)
    ULONG InstanceId; // Instance ID.
    // Followed by:
    // WCHAR InstanceName[]; // Nul-terminated.
    // WCHAR Padding[];      // Pad to a multiple of 8 bytes
} PERF_INSTANCE_HEADER, *PPERF_INSTANCE_HEADER;

/*
The PerfQueryCounterSetRegistrationInfo function accepts a requestCode value
from the PerfRegInfoType enumeration.
*/
typedef enum _PerfRegInfoType {
    PERF_REG_COUNTERSET_STRUCT = 1,   /* Returns a PERF_COUNTERSET_REG_INFO block.
        The block includes a PERF_COUNTERSET_REG_INFO structure followed by 1 or
        more PERF_COUNTER_REG_INFO structures. */
    PERF_REG_COUNTER_STRUCT,          /* Returns a PERF_COUNTER_REG_INFO structure.
        Uses requestLangId to specify the counter ID. */
    PERF_REG_COUNTERSET_NAME_STRING,  /* Returns a nul-terminated utf-16le string.
        Uses requestLangId to specify the preferred locale of the result. */
    PERF_REG_COUNTERSET_HELP_STRING,  /* Returns a nul-terminated utf-16le string.
        Uses requestLangId to specify the preferred locale of the result. */
    PERF_REG_COUNTER_NAME_STRINGS,    /* Returns a PERF_STRING_BUFFER_HEADER block.
        The block includes a PERF_STRING_BUFFER_HEADER structure followed by 1
        or more PERF_STRING_COUNTER_HEADER structures followed by string data.
        Uses requestLangId to specify the preferred locale of the result. */
    PERF_REG_COUNTER_HELP_STRINGS,    /* Returns a PERF_STRING_BUFFER_HEADER block.
        The block includes a PERF_STRING_BUFFER_HEADER structure followed by 1
        or more PERF_STRING_COUNTER_HEADER structures followed by string data.
        Uses requestLangId to specify the preferred locale of the result. */
    PERF_REG_PROVIDER_NAME,           /* Returns a nul-terminated utf-16le string. */
    PERF_REG_PROVIDER_GUID,           /* Returns a GUID. */
    PERF_REG_COUNTERSET_ENGLISH_NAME, /* Returns a nul-terminated utf-16le string.
        Equivalent to PERF_REG_COUNTERSET_NAME_STRING with requestLangId = 0. */
    PERF_REG_COUNTER_ENGLISH_NAMES    /* Returns a PERF_STRING_BUFFER_HEADER block.
        Equivalent to PERF_REG_COUNTER_NAME_STRINGS with requestLangId = 0. */
} PerfRegInfoType;

/*
A PERF_COUNTERSET_REG_INFO block consists of a PERF_COUNTERSET_REG_INFO
structure immediately followed by NumCounters instances of the
PERF_COUNTER_REG_INFO structure.

The PerfQueryCounterSetRegistrationInfo function with request code
PERF_REG_COUNTERSET_STRUCT returns a PERF_COUNTERSET_REG_INFO block.
*/
typedef struct _PERF_COUNTERSET_REG_INFO {
    GUID   CounterSetGuid; // Unique ID for the counter set
    ULONG  CounterSetType; // Reserved.
    ULONG  DetailLevel;    // PERF_DETAIL_NOVICE or PERF_DETAIL_ADVANCED.
    ULONG  NumCounters;    // Number of PERF_COUNTER_REG_INFO structures in this block.
    ULONG  InstanceType;   // PERF_COUNTERSET_* type (SINGLE_INSTANCE, MULTI_INSTANCES, etc.)
    // Followed by:
    // PERF_COUNTER_REG_INFO CounterInfo[NumCounters];
} PERF_COUNTERSET_REG_INFO, * PPERF_COUNTERSET_REG_INFO;

/*
The PerfQueryCounterSetRegistrationInfo function with request code
PERF_REG_COUNTERSET_STRUCT returns a PERF_COUNTERSET_REG_INFO block that
contains one or more PERF_COUNTER_REG_INFO structures.

The PerfQueryCounterSetRegistrationInfo function with request code
PERF_REG_COUNTER_STRUCT returns a PERF_COUNTER_REG_INFO structure.
*/
typedef struct _PERF_COUNTER_REG_INFO {
    ULONG     CounterId;     // Counter ID is unique within the counter set. Max of 64K counters per counter set.
    ULONG     Type;          // Counter type from winperf.h, e.g. PERF_COUNTER_COUNTER, PERF_SAMPLE_COUNTER, etc.
    ULONGLONG Attrib;        // PERF_ATTRIB value.
    ULONG     DetailLevel;   // PERF_DETAIL_NOVICE or PERF_DETAIL_ADVANCED.
    LONG      DefaultScale;  // -10 to 10. ScaledValue = RawValue * (10^DefaultScale).
    ULONG     BaseCounterId; // The counter ID of the base counter, 0xffffffff for none.
    ULONG     PerfTimeId;    // The counter ID of the perf counter, 0xffffffff for none.
    ULONG     PerfFreqId;    // The counter ID of the freq counter, 0xffffffff for none.
    ULONG     MultiId;       // The counter ID of the multi counter, 0xffffffff for none.
    ULONG     AggregateFunc; // PERF_AGGREGATE value, used if counter set type is aggregate.
    ULONG     Reserved;      // Reserved.
} PERF_COUNTER_REG_INFO, * PPERF_COUNTER_REG_INFO;

/*
The PERF_STRING_BUFFER_HEADER block consists of a PERF_STRING_BUFFER_HEADER
structure followed by dwCounters instances of the PERF_STRING_COUNTER_HEADER
structure followed by a block of string data. The dwSize field contains the
size (in bytes) of the entire PERF_STRING_BUFFER_HEADER block, including the
strings.

The PerfQueryCounterSetRegistrationInfo function with request code
PERF_REG_COUNTER_NAME_STRINGS or PERF_REG_COUNTER_HELP_STRINGS returns a
PERF_STRING_BUFFER_HEADER block.
*/
typedef struct _STRING_BUFFER_HEADER {
    DWORD  dwSize;     // = sizeof(PERF_STRING_BUFFER_HEADER) + sizeof(Counters) + sizeof(StringData)
    DWORD  dwCounters; // Number of PERF_STRING_COUNTER_HEADER structures in this block.
    // Followed by:
    // PERF_STRING_COUNTER_HEADER Counters[dwCounters];
    // WCHAR StringData[];
} PERF_STRING_BUFFER_HEADER, *PPERF_STRING_BUFFER_HEADER;

/*
The PERF_STRING_COUNTER_HEADER structure is part of the
PERF_STRING_BUFFER_HEADER block.

The dwOffset field is a byte offset from the start of the
PERF_STRING_BUFFER_HEADER block to the nul-terminated utf-16le data. The
dwOffset field can be 0xffffffff to indicate that the string is not present
(i.e. that the value of the string is NULL).

The PerfQueryCounterSetRegistrationInfo function with request code
PERF_REG_COUNTER_NAME_STRINGS or PERF_REG_COUNTER_HELP_STRINGS returns a
PERF_STRING_BUFFER_HEADER block that contains one or more
PERF_STRING_COUNTER_HEADER structures.
*/
typedef struct _STRING_COUNTER_HEADER {
    DWORD  dwCounterId; // ID of the counter.
    DWORD  dwOffset;    // Byte offset from PERF_STRING_BUFFER_HEADER to string. Offset can be 0xffffffff to indicate NULL.
} PERF_STRING_COUNTER_HEADER, *PPERF_STRING_COUNTER_HEADER;

/*
A PERF_COUNTER_IDENTIFIER block consists of a PERF_COUNTER_IDENTIFIER
structure optionally followed by a nul-terminated utf-16le string, followed by
padding as needed to a multiple of 8 bytes. The Size field contains the size
of the block (including the string and padding) and should be a multiple of 8.

Note that when specifying an identifier for a single-instance counter set, the
InstanceName MUST NOT be specified, i.e. the size of the block must be exactly
sizeof(PERF_COUNTER_IDENTIFIER). On the other hand, when specifying an
identifier for a multi-instance counter set, the InstanceName MUST be
specified, i.e. the identifier will be considered invalid unless the size of
the block is greater than sizeof(PERF_COUNTER_IDENTIFIER); if you do not want
to filter based on instance name, use PERF_WILDCARD_INSTANCE as the instance
name.

The PerfAddCounters and PerfDeleteCounters functions accept a sequence of
A PERF_COUNTER_IDENTIFIER blocks to define the counter specifications to be
added or removed from a query.

The PerfQueryCounterInfo function returns a sequence of PERF_COUNTER_IDENTIFIER
blocks to indicate the counter specifications in a query and to indicate the
order in which the results will be returned by the query (via the Index field).
*/
typedef struct _PERF_COUNTER_IDENTIFIER {
    GUID   CounterSetGuid; // The GUID of the counterset.
    ULONG  Status;         // Win32 error code indicating success/failure of the add/delete operation.
    ULONG  Size;           // sizeof(PERF_COUNTER_IDENTIFIER) + sizeof(InstanceName) + sizeof(Padding)
    ULONG  CounterId;      // CounterId, or PERF_WILDCARD_COUNTER for all counters.
    ULONG  InstanceId;     // InstanceId, or 0xFFFFFFFF to not filter on instance ID.
    ULONG  Index;          // Set by PerfQueryCounterInfo to the position in which the corresponding counter data is returned.
    ULONG  Reserved;       // Reserved.
    // Followed by:
    // WCHAR InstanceName[];
    // WCHAR Padding[];
} PERF_COUNTER_IDENTIFIER, * PPERF_COUNTER_IDENTIFIER;

/*
The PERF_DATA_HEADER block consists of a PERF_DATA_HEADER structure followed
by dwNumCounters PERF_COUNTER_HEADER blocks. The size of the PERF_DATA_HEADER
block (including the PERF_COUNTER_HEADER blocks) is dwTotalSize. Each
PERF_COUNTER_HEADER block corresponds to one query specification in the query.
The ordering of the PERF_COUNTER_HEADER blocks is based on the Index field of
the PERF_COUNTER_IDENTIFIER blocks returned by PerfQueryCounterInfo. Each
PERF_COUNTER_HEADER block is 8-byte aligned, so dwTotalSize will be a multiple
of 8.

The timestamp information in the PERF_DATA_HEADER structure is required when
computing the display values of certain counters.

The PerfQueryCounterData function returns a PERF_DATA_HEADER block.
*/
typedef struct _PERF_DATA_HEADER {
    ULONG      dwTotalSize;     // = sizeof(PERF_DATA_HEADER) + sizeof(PERF_COUNTER_HEADER blocks...)
    ULONG      dwNumCounters;   // The number of PERF_COUNTER_HEADER blocks.
    LONGLONG   PerfTimeStamp;   // Timestamp from a high-resolution clock.
    LONGLONG   PerfTime100NSec; // The number of 100 nanosecond intervals since January 1, 1601, in Coordinated Universal Time (UTC).
    LONGLONG   PerfFreq;        // The frequency of a high-resolution clock.
    SYSTEMTIME SystemTime;      // The time at which data is collected on the provider side.
    // Followed by:
    // PERF_COUNTER_HEADER blocks...;
} PERF_DATA_HEADER, * PPERF_DATA_HEADER;

/*
The PerfQueryCounterData function returns a PERF_DATA_HEADER block that
contains PERF_COUNTER_HEADER blocks that indicate their content type using
values from the PerfCounterDataType enumeration.
*/
typedef enum _PerfCounterDataType {
    PERF_ERROR_RETURN = 0,       /* An error occurred when the performance counter value was queried. */
    PERF_SINGLE_COUNTER = 1,     /* Query returned a single counter from a single-instance. */
    PERF_MULTIPLE_COUNTERS = 2,  /* Query returned multiple counters from a single instance. */
    PERF_MULTIPLE_INSTANCES = 4, /* Query returned a single counter from each of multiple instances. */
    PERF_COUNTERSET = 6          /* Query returned multiple counters from each of multiple instances. */
} PerfCounterDataType;

/*
A PERF_COUNTER_HEADER block consists of a PERF_COUNTER_HEADER structure
followed by additional data. The type of the additional data is based on the
value of the dwType field as follows:

* PERF_ERROR_RETURN - PERFLIB cannot get valid counter data back from provider.
  No additional data follows the PERF_COUNTER_HEADER. The dwStatus field
  contains the error code.

* PERF_SINGLE_COUNTER - single-counter single-instance query, e.g.
  "\Processor(_Total)\% Processor Time". The additional data consists of:
  PERF_COUNTER_DATA block.

* PERF_MULTIPLE_COUNTERS - multi-counter single-instance query, e.g.
  "\Processor(_Total)\*". The additional data consists of:
  PERF_MULTI_COUNTERS block + PERF_COUNTER_DATA blocks.

* PERF_MULTIPLE_INSTANCES - single-counter multi-instance query, e.g.
  "\Processor(*)\% Processor Time". The additional data consists of:
  PERF_MULTI_INSTANCES block.

* PERF_COUNTERSET - multi-counter multi-instance query, e.g.
  "\Processor(*)\*". The additional data consists of:
  PERF_MULTI_COUNTERS block + PERF_MULTI_INSTANCES block.

The size of the block (including the PERF_COUNTER_HEADER structure and the
additional data) is given by the dwSize field. Each PERF_COUNTER_HEADER block
is 8-byte aligned, so dwSize will be a multiple of 8.

The PerfQueryCounterData function returns a PERF_DATA_HEADER block that
contains a sequence of PERF_COUNTER_HEADER blocks.
*/
typedef struct _PERF_COUNTER_HEADER {
    ULONG      dwStatus;        // Win32 error code indicating success/failure of the query operation.
    PerfCounterDataType dwType; // Result type - error, single/single, multi/single, single/multi, multi/multi.
    ULONG      dwSize;          // = sizeof(PERF_COUNTER_HEADER) + sizeof(Additional data)
    ULONG      Reserved;        // Reserved.
    // Followed by additional data:
    // If dwType == PERF_ERROR_RETURN:       nothing.
    // If dwType == PERF_SINGLE_COUNTER:     PERF_COUNTER_DATA block.
    // If dwType == PERF_MULTIPLE_COUNTERS:  PERF_MULTI_COUNTERS block + PERF_COUNTER_DATA blocks.
    // If dwType == PERF_MULTIPLE_INSTANCES: PERF_MULTI_INSTANCES block.
    // If dwType == PERF_COUNTERSET:         PERF_MULTI_COUNTERS block + PERF_MULTI_INSTANCES block.
} PERF_COUNTER_HEADER, * PPERF_COUNTER_HEADER;

/*
A PERF_MULTI_INSTANCES block consists of a PERF_MULTI_INSTANCES structure
followed by dwInstances of instance data blocks. Each instance data block
consists of a PERF_INSTANCE_HEADER block followed by a number of
PERF_COUNTER_DATA blocks. The number of PERF_COUNTER_DATA blocks depends on
the context - if the PERF_MULTI_INSTANCES block is part of a
PERF_COUNTER_HEADER block with type PERF_MULTIPLE_INSTANCES, there will be one
PERF_COUNTER_DATA block; if the PERF_MULTI_INSTANCES block is part of a
PERF_COUNTER_HEADER block with type PERF_COUNTERSET, the number of
PERF_COUNTER_DATA blocks is indicated by the PERF_MULTI_COUNTERS block.
The total size of the block is dwTotalSize, which will be a multiple of 8.

The PerfQueryCounterData function returns a PERF_DATA_HEADER block that may
contain PERF_MULTI_INSTANCES blocks (within the PERF_COUNTER_HEADER block).
*/
typedef struct _PERF_MULTI_INSTANCES {
    ULONG      dwTotalSize; // = sizeof(PERF_MULTI_INSTANCES) + sizeof(instance data blocks...)
    ULONG      dwInstances; // Number of instance data blocks.
    // Followed by:
    // Instance data blocks...;
} PERF_MULTI_INSTANCES, * PPERF_MULTI_INSTANCES;

/*
A PERF_MULTI_COUNTERS block consists of a PERF_MULTI_COUNTERS structure
followed by a sequence of DWORD counter ID values.

The PerfQueryCounterData function returns a PERF_DATA_HEADER block that may
contain PERF_MULTI_COUNTERS blocks (within the PERF_COUNTER_HEADER block).
*/
typedef struct _PERF_MULTI_COUNTERS {
    ULONG      dwSize;     // sizeof(PERF_MULTI_COUNTERS) + sizeof(CounterIds)
    ULONG      dwCounters; // Number of counter ids.
    // Followed by:
    // DWORD CounterIds[dwCounters];
} PERF_MULTI_COUNTERS, * PPERF_MULTI_COUNTERS;

/*
A PERF_COUNTER_DATA block consists of a PERF_COUNTER_DATA structure followed by
dwDataSize bytes of raw counter data followed by padding to a multiple of 8
bytes. The dwSize field contains the total size of the PERF_COUNTER_DATA block,
including padding.

The PerfQueryCounterData function returns a PERF_DATA_HEADER block that may
directly or indirectly contain PERF_COUNTER_DATA blocks.
*/
typedef struct _PERF_COUNTER_DATA {
    ULONG      dwDataSize; // Size of the counter data, in bytes.
    ULONG      dwSize;     // = sizeof(PERF_COUNTER_DATA) + sizeof(Data) + sizeof(Padding)
    // Followed by:
    // BYTE Data[dwDataSize];
    // BYTE Padding[];
} PERF_COUNTER_DATA, * PPERF_COUNTER_DATA;

/*
PerfEnumerateCounterSet:

Gets the counter set IDs (GUIDs) of the counter sets registered on the
specified system.

szMachine:
The name of the machine to be queried, or NULL for the local system.

pCounterSetIds:
Pointer to a buffer with room to receive cCounterSetIds GUIDs. May be NULL if
cCounterSetIds is 0.

cCounterSetIds:
The size of the pCounterSetIds buffer, measured in GUIDs.

pcCounterSetIdsActual:
Receives the size of the buffer required. The meaning depends on the status
returned by the function.
- ERROR_SUCCESS: On return, *pcCounterSetIdsActual contains the number of
  GUIDs stored into the pCounterSetIds buffer by the function.
- ERROR_NOT_ENOUGH_MEMORY: On return *pcCounterSetIdsActual contains the
  size (in GUIDs) of the buffer required. Resize the buffer to the required
  size and call the function again.
- Other: On return, the value *pcCounterSetIdsActual is undefined and should
  not be used.
*/
_Success_(return == ERROR_SUCCESS)
ULONG
WINAPI
PerfEnumerateCounterSet(
    _In_opt_z_ LPCWSTR szMachine,
    _Out_opt_cap_post_count_(cCounterSetIds, *pcCounterSetIdsActual) LPGUID pCounterSetIds,
    DWORD cCounterSetIds,
    _Out_ LPDWORD pcCounterSetIdsActual
    );

/*
PerfEnumerateCounterSetInstances:

Gets the names and IDs of the active instances of a counter set on the
specified system.

szMachine:
The name of the machine to be queried, or NULL for the local system.

pCounterSetId:
The counter set ID (GUID) of the counter set to be queried.

pInstances:
Pointer to a buffer with room to receive cbInstances BYTEs of data. May be
NULL if cbInstances is 0.

cbInstances:
The size of the pInstances buffer, measured in BYTEs.

pcbInstancesActual:
Receives the size of the buffer required. The meaning depends on the status
returned by the function.
- ERROR_SUCCESS: On return, *pcbInstancesActual contains the number of BYTEs
  stored into the pInstances buffer by the function.
- ERROR_NOT_ENOUGH_MEMORY: On return *pcbInstancesActual contains the size (in
  BYTEs) of the buffer required. Resize the buffer to the required size and
  call the function again.
- Other: On return, the value *pcbInstancesActual is undefined and should not
  be used.

Remarks:

The returned data is a sequence of PERF_INSTANCE_HEADER blocks. The size of
the sequence is *pcbInstancesActual. Each PERF_INSTANCE_HEADER block consists
of a PERF_INSTANCE_HEADER immediately followed by a nul-terminated utf-16le
instance name, followed by padding so that the size of the
PERF_INSTANCE_HEADER block is a multiple of 8.
*/
_Success_(return == ERROR_SUCCESS)
ULONG
WINAPI
PerfEnumerateCounterSetInstances(
    _In_opt_z_ LPCWSTR szMachine,
    _In_ LPCGUID pCounterSetId,
    _Out_opt_bytecap_post_bytecount_(cbInstances, *pcbInstancesActual) PPERF_INSTANCE_HEADER pInstances,
    DWORD cbInstances,
    _Out_ LPDWORD pcbInstancesActual
    );

/*
PerfQueryCounterSetRegistrationInfo:

Gets information about a counter set on the specified system.

szMachine:
The name of the machine to be queried, or NULL for the local system.

pCounterSetId:
The counter set ID (GUID) of the counter set to be queried.

requestCode:
The type of data needed.

requestLangId:
For request codes PERF_REG_COUNTERSET_NAME_STRING,
PERF_REG_COUNTERSET_HELP_STRING, PERF_REG_COUNTER_NAME_STRINGS, and
PERF_REG_COUNTER_HELP_STRINGS, this specifies the preferred locale ID of the
returned strings. For PERF_REG_COUNTER_STRUCT, this specifies the counter ID
for which data should be returned. For all other request codes, set
requestLangId to 0.

pbRegInfo:
Pointer to a buffer with room to receive cbRegInfo BYTEs of data. May be
NULL if cbRegInfo is 0.

cbRegInfo:
The size of the pbRegInfo buffer, measured in BYTEs.

pcbRegInfoActual:
Receives the size of the buffer required. The meaning depends on the status
returned by the function.
- ERROR_SUCCESS: On return, *pcbRegInfoActual contains the number of BYTEs
  stored into the pbRegInfo buffer by the function.
- ERROR_NOT_ENOUGH_MEMORY: On return *pcbRegInfoActual contains the size (in
  BYTEs) of the buffer required. Resize the buffer to the required size and
  call the function again.
- Other: On return, the value *pcbRegInfoActual is undefined and should not
  be used.

Remarks:

See the PerfRegInfoType enum for the types of data that can be requested and
the formats of the data returned for each type of request.
*/
_Success_(return == ERROR_SUCCESS)
ULONG
WINAPI
PerfQueryCounterSetRegistrationInfo(
    _In_opt_z_ LPCWSTR szMachine,
    _In_ LPCGUID pCounterSetId,
    PerfRegInfoType requestCode,
    DWORD requestLangId,
    _Out_opt_bytecap_post_bytecount_(cbRegInfo, *pcbRegInfoActual) LPBYTE pbRegInfo,
    DWORD cbRegInfo,
    _Out_ LPDWORD pcbRegInfoActual
    );

/*
PerfOpenQueryHandle:

Creates a handle that references a query on the specified system. A query is a
list of counter specifications. Use PerfAddCounters and PerfDeleteCounters to
add or remove counter specifications to the list. Use PerfQueryCounterInfo to
get the counter specifications currently in the list and to determine the
indexes at which the data for each counter will be returned by 
PerfQueryCounterData. Use PerfQueryCounterData to retrieve the values of the
counters that match the counter specifications.

szMachine:
The name of the machine to be queried, or NULL for the local system.

phQuery:
Receives the query handle. The handle should be closed with
PerfCloseQueryHandle when it is no longer needed.
*/
_Success_(return == ERROR_SUCCESS)
ULONG
WINAPI
PerfOpenQueryHandle(
    _In_opt_z_ LPCWSTR szMachine,
    _Out_ HANDLE * phQuery
    );

/*
PerfCloseQueryHandle:

Closes a query handle that was opened by PerfOpenQueryHandle.

hQuery:
The query handle to be closed.
*/
_Success_(return == ERROR_SUCCESS)
ULONG
WINAPI
PerfCloseQueryHandle(
    _In_ HANDLE hQuery
    );

/*
PerfQueryCounterInfo:

Gets the counter specifications in the specified query.

hQuery:
The query from which information is to be retrieved.

pCounters:
Pointer to a buffer with room to receive cbCounters BYTEs of data. May be
NULL if cbCounters is 0.

cbCounters:
The size of the pCounters buffer, measured in BYTEs.

pcbCountersActual:
Receives the size of the buffer required. The meaning depends on the status
returned by the function.
- ERROR_SUCCESS: On return, *pcbCountersActual contains the number of BYTEs
  stored into the pCounters buffer by the function.
- ERROR_NOT_ENOUGH_MEMORY: On return *pcbCountersActual contains the size (in
  BYTEs) of the buffer required. Resize the buffer to the required size and
  call the function again.
- Other: On return, the value *pcbCountersActual is undefined and should not
  be used.

Remarks:

The returned data is a sequence of PERF_COUNTER_IDENTIFIER blocks. Each block
consists of a PERF_COUNTER_IDENTIFIER structure optionally followed by a nul-
terminated utf-16le InstanceName, followed by padding to a multiple of 8 bytes.
The size of each block (including the PERF_COUNTER_IDENTIFIER, InstanceName,
and padding) is determined by the PERF_COUNTER_IDENTIFIER Size field, which
will be a multiple of 8 bytes.
*/
_Success_(return == ERROR_SUCCESS)
ULONG
WINAPI
PerfQueryCounterInfo(
    _In_ HANDLE hQuery,
    _Out_opt_bytecap_post_bytecount_(cbCounters, *pcbCountersActual) PPERF_COUNTER_IDENTIFIER pCounters,
    DWORD cbCounters,
    _Out_ LPDWORD pcbCountersActual
    );

/*
PerfQueryCounterData:

Gets the values of the counters that match the counter specifications in the
specified query.

hQuery:
The query from which information is to be retrieved.

pCounterBlock:
Pointer to a buffer with room to receive cbCounterBlock BYTEs of data. May be
NULL if cbCounterBlock is 0.

cbCounterBlock:
The size of the pCounterBlock buffer, measured in BYTEs.

pcbCounterBlockActual:
Receives the size of the buffer required. The meaning depends on the status
returned by the function.
- ERROR_SUCCESS: On return, *pcbCounterBlockActual contains the number of
  BYTEs stored into the pCounterBlock buffer by the function.
- ERROR_NOT_ENOUGH_MEMORY: On return *pcbCounterBlockActual contains the size
  (in BYTEs) of the buffer required. Resize the buffer to the required size
  and call the function again.
- Other: On return, the value *pcbCounterBlockActual is undefined and should
  not be used.

Remarks:

The returned data will be a PERF_DATA_HEADER block, which is a PERF_DATA_HEADER
structure followed by a sequence of PERF_COUNTER_HEADER blocks. See the
description of the PERF_DATA_HEADER and PERF_COUNTER_HEADER structures for
details.
*/
_Success_(return == ERROR_SUCCESS)
ULONG
WINAPI
PerfQueryCounterData(
    _In_ HANDLE hQuery,
    _Out_opt_bytecap_post_bytecount_(cbCounterBlock, *pcbCounterBlockActual) PPERF_DATA_HEADER pCounterBlock,
    DWORD cbCounterBlock,
    _Out_ LPDWORD pcbCounterBlockActual
    );

/*
PerfAddCounters:

Adds counter specifications to the specified query.

hQuery:
The query to be updated.

pCounters:
A pointer to the counter specification(s) to be added.

cbCounters:
The size of the pCounters buffer, measured in BYTEs.

Remarks:

The pCounters data should point to a sequence of PERF_COUNTER_IDENTIFIER
blocks. Each PERF_COUNTER_IDENTIFIER block consists of a
PERF_COUNTER_IDENTIFIER structure optionally followed by a nul-terminated
utf-16le InstanceName string, followed by padding to a multiple of 8 bytes.

For each PERF_COUNTER_IDENTIFIER block:
- Set CounterSetGuid to the ID of the counter set to be queried.
- Set Status to 0.
- Set Size to the size of the PERF_COUNTER_IDENTIFIER block (including the
  PERF_COUNTER_IDENTIFIER structure, the InstanceName, and the padding). The
  Size must be a multiple of 8.
- Set CounterId to the ID of the counter that should be returned by the query.
  To return all counters, set CounterId to PERF_WILDCARD_COUNTER.
- Set InstanceId to the ID of the instance that should be returned by the
  query. If no filtering should be done based on instance ID, set
  InstanceId to PERF_WILDCARD_COUNTER.
- Set Index to 0.
- Set Reserved to 0.
- The InstanceName goes immediately after the PERF_COUNTER_IDENTIFIER
  structure. If the counter set is single-instance, the InstanceName MUST NOT
  be set (i.e. Size must be set to sizeof(PERF_COUNTER_IDENTIFIER)). If the
  counter set is single-instance, the InstanceName MUST be set. If no filtering
  should be done based on instance name, use PERF_WILDCARD_INSTANCE as the
  instance name.

PerfAddCounters will attempt to add one counter specification to the query for
each PERF_COUNTER_IDENTIFIER block, and will update the Status field of each
block with the result of the attempt.
*/
_Success_(return == ERROR_SUCCESS)
ULONG
WINAPI
PerfAddCounters(
    _In_ HANDLE hQuery,
    _Inout_bytecount_(cbCounters) PPERF_COUNTER_IDENTIFIER pCounters,
    DWORD cbCounters
    );

/*
PerfDeleteCounters:

Removes counter specifications from the specified query.

hQuery:
The query to be updated.

pCounters:
A pointer to the counter specification(s) to be removed.

cbCounters:
The size of the pCounters buffer, measured in BYTEs.

Remarks:

The pCounters data should point to a sequence of PERF_COUNTER_IDENTIFIER
blocks. Each PERF_COUNTER_IDENTIFIER block consists of a
PERF_COUNTER_IDENTIFIER structure optionally followed by a nul-terminated
utf-16le InstanceName string, followed by padding to a multiple of 8 bytes.

Configure each PERF_COUNTER_IDENTIFIER block as was done for PerfAddCounters.

PerfDeleteCounters will attempt to remove one counter specification from the
query for each PERF_COUNTER_IDENTIFIER block, and will update the Status field
of each block with the result of the attempt.
*/
_Success_(return == ERROR_SUCCESS)
ULONG
WINAPI
PerfDeleteCounters(
    _In_ HANDLE hQuery,
    _Inout_bytecount_(cbCounters) PPERF_COUNTER_IDENTIFIER pCounters,
    DWORD cbCounters
    );

#ifdef __cplusplus
} // extern "C"
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif /* _PERFLIB_H_ */

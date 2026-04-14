 
/*++
Copyright (C) Microsoft Corporation.  All rights reserved.

Module Name:

    winevt.h

Abstract:

    Windows Events API

--*/

#ifndef __WINEVT_H__
#define __WINEVT_H__
#include <winapifamily.h>

#pragma region Desktop Family or EventLogService Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_EVENTLOGSERVICE)

#ifdef __cplusplus
extern "C"
{
#endif

#if (WINVER >= _WIN32_WINNT_LONGHORN)

typedef HANDLE EVT_HANDLE, *PEVT_HANDLE;

typedef enum _EVT_VARIANT_TYPE
{
    EvtVarTypeNull        = 0,
    EvtVarTypeString      = 1,
    EvtVarTypeAnsiString  = 2,
    EvtVarTypeSByte       = 3,
    EvtVarTypeByte        = 4,
    EvtVarTypeInt16       = 5,
    EvtVarTypeUInt16      = 6,
    EvtVarTypeInt32       = 7,
    EvtVarTypeUInt32      = 8,
    EvtVarTypeInt64       = 9,
    EvtVarTypeUInt64      = 10,
    EvtVarTypeSingle      = 11,
    EvtVarTypeDouble      = 12,
    EvtVarTypeBoolean     = 13,
    EvtVarTypeBinary      = 14,
    EvtVarTypeGuid        = 15,
    EvtVarTypeSizeT       = 16,
    EvtVarTypeFileTime    = 17,
    EvtVarTypeSysTime     = 18,
    EvtVarTypeSid         = 19,
    EvtVarTypeHexInt32    = 20,
    EvtVarTypeHexInt64    = 21,

    // these types used internally
    EvtVarTypeEvtHandle   = 32,
    EvtVarTypeEvtXml      = 35

} EVT_VARIANT_TYPE;

#define EVT_VARIANT_TYPE_MASK 0x7f
#define EVT_VARIANT_TYPE_ARRAY 128

#pragma warning(push)
#pragma warning(disable:4201)  // Nameless struct/union
typedef struct _EVT_VARIANT
{
    union
    {
        BOOL        BooleanVal;
        INT8        SByteVal;
        INT16       Int16Val;
        INT32       Int32Val;
        INT64       Int64Val;
        UINT8       ByteVal;
        UINT16      UInt16Val;
        UINT32      UInt32Val;
        UINT64      UInt64Val;
        float       SingleVal;
        double      DoubleVal;
        ULONGLONG   FileTimeVal;
        SYSTEMTIME* SysTimeVal;
        GUID*       GuidVal;
        LPCWSTR     StringVal;
        LPCSTR      AnsiStringVal;
        PBYTE       BinaryVal;
        PSID        SidVal;
        size_t      SizeTVal;

        // array fields
        BOOL*       BooleanArr;
        INT8*       SByteArr;
        INT16*      Int16Arr;
        INT32*      Int32Arr;
        INT64*      Int64Arr;
        UINT8*      ByteArr;
        UINT16*     UInt16Arr;
        UINT32*     UInt32Arr;
        UINT64*     UInt64Arr;
        float*      SingleArr;
        double*     DoubleArr;
        FILETIME*   FileTimeArr;
        SYSTEMTIME* SysTimeArr;
        GUID*       GuidArr;
        LPWSTR*     StringArr;
        LPSTR*      AnsiStringArr;
        PSID*       SidArr;
        size_t*     SizeTArr;

        // internal fields
        EVT_HANDLE  EvtHandleVal;
        LPCWSTR     XmlVal;
        LPCWSTR*    XmlValArr;
    };

    DWORD Count;   // for arrays: number of elements (not the number of bytes).
    DWORD Type;

} EVT_VARIANT, *PEVT_VARIANT;
#pragma warning(pop)

////////////////////////////////////////////////////////////////////////////////
//
// Sessions
//
////////////////////////////////////////////////////////////////////////////////

typedef enum _EVT_LOGIN_CLASS
{
    EvtRpcLogin = 1

} EVT_LOGIN_CLASS;

typedef enum _EVT_RPC_LOGIN_FLAGS
{
    EvtRpcLoginAuthDefault = 0,
    EvtRpcLoginAuthNegotiate,
    EvtRpcLoginAuthKerberos,
    EvtRpcLoginAuthNTLM

} EVT_RPC_LOGIN_FLAGS;

typedef struct _EVT_RPC_LOGIN
{
    // all str params are optional
    LPWSTR  Server;
    LPWSTR  User;
    LPWSTR  Domain;
    LPWSTR  Password;
    DWORD   Flags;                      // EVT_RPC_LOGIN_FLAGS

} EVT_RPC_LOGIN;


// Timeout and Flags must currently be 0
EVT_HANDLE
WINAPI
EvtOpenSession(
    EVT_LOGIN_CLASS LoginClass,
    _When_((LoginClass == EvtRpcLogin), _In_reads_bytes_(sizeof(EVT_RPC_LOGIN))) PVOID Login,
    _Reserved_ DWORD Timeout,
    _Reserved_ DWORD Flags
    );

////////////////////////////////////////////////////////////////////////////////
//
// General Purpose Functions
//
////////////////////////////////////////////////////////////////////////////////

_Success_(return != 0)
BOOL
WINAPI
EvtClose(
    _In_ _Post_invalid_ EVT_HANDLE Object
    );


BOOL
WINAPI
EvtCancel(
    _In_opt_ EVT_HANDLE Object
    );

_Success_(return == ERROR_SUCCESS)
DWORD
WINAPI
EvtGetExtendedStatus(
    DWORD BufferSize,
    _Out_writes_to_opt_(BufferSize, *BufferUsed) LPWSTR Buffer,
    _Out_ PDWORD BufferUsed
    );

////////////////////////////////////////////////////////////////////////////////
//
// Queries
//
////////////////////////////////////////////////////////////////////////////////

typedef enum _EVT_QUERY_FLAGS
{
    EvtQueryChannelPath                 = 0x1,
    EvtQueryFilePath                    = 0x2,

    EvtQueryForwardDirection            = 0x100,
    EvtQueryReverseDirection            = 0x200,

    EvtQueryTolerateQueryErrors         = 0x1000

} EVT_QUERY_FLAGS;

typedef enum _EVT_SEEK_FLAGS
{
    EvtSeekRelativeToFirst    = 1,
    EvtSeekRelativeToLast     = 2,
    EvtSeekRelativeToCurrent  = 3,
    EvtSeekRelativeToBookmark = 4,
    EvtSeekOriginMask         = 7,

    EvtSeekStrict             = 0x10000,

} EVT_SEEK_FLAGS;


EVT_HANDLE
WINAPI
EvtQuery(
    _In_opt_ EVT_HANDLE Session,
    _In_opt_z_ LPCWSTR Path,
    _In_opt_z_ LPCWSTR Query,
    DWORD Flags
    );

_Success_(return != 0)
BOOL
WINAPI
EvtNext(
    _In_ EVT_HANDLE ResultSet,
    DWORD EventsSize,
    _Out_writes_to_(EventsSize, *Returned) PEVT_HANDLE Events,
    DWORD Timeout,
    DWORD Flags,
    _Out_range_(0, EventsSize) PDWORD Returned
    );

// Timeout must currently be 0
BOOL
WINAPI
EvtSeek(
    _In_ EVT_HANDLE ResultSet,
    LONGLONG Position,
    _In_opt_ EVT_HANDLE Bookmark,
    _Reserved_ DWORD Timeout,
    DWORD Flags
    );

////////////////////////////////////////////////////////////////////////////////
//
// Subscriptions
//
////////////////////////////////////////////////////////////////////////////////

typedef enum _EVT_SUBSCRIBE_FLAGS
{
    EvtSubscribeToFutureEvents      = 1,
    EvtSubscribeStartAtOldestRecord = 2,
    EvtSubscribeStartAfterBookmark  = 3,
    EvtSubscribeOriginMask          = 3,

    EvtSubscribeTolerateQueryErrors = 0x1000,

    EvtSubscribeStrict              = 0x10000,

} EVT_SUBSCRIBE_FLAGS;

typedef enum _EVT_SUBSCRIBE_NOTIFY_ACTION
{
    EvtSubscribeActionError = 0,
    EvtSubscribeActionDeliver

} EVT_SUBSCRIBE_NOTIFY_ACTION;

typedef DWORD (WINAPI *EVT_SUBSCRIBE_CALLBACK)(
    EVT_SUBSCRIBE_NOTIFY_ACTION Action,
    _Maybenull_ PVOID UserContext,
    _In_ EVT_HANDLE Event );

EVT_HANDLE
WINAPI
EvtSubscribe(
    _In_opt_ EVT_HANDLE Session,
    _In_opt_ HANDLE SignalEvent,
    _In_opt_z_ LPCWSTR ChannelPath,
    _In_opt_z_ LPCWSTR Query,
    _In_opt_ EVT_HANDLE Bookmark,
    _Maybenull_ PVOID Context,
    _Maybenull_ EVT_SUBSCRIBE_CALLBACK Callback,
    DWORD Flags
    );

////////////////////////////////////////////////////////////////////////////////
//
// Rendering
//
////////////////////////////////////////////////////////////////////////////////

typedef enum _EVT_SYSTEM_PROPERTY_ID
{
    EvtSystemProviderName = 0,          // EvtVarTypeString
    EvtSystemProviderGuid,              // EvtVarTypeGuid
    EvtSystemEventID,                   // EvtVarTypeUInt16
    EvtSystemQualifiers,                // EvtVarTypeUInt16
    EvtSystemLevel,                     // EvtVarTypeUInt8
    EvtSystemTask,                      // EvtVarTypeUInt16
    EvtSystemOpcode,                    // EvtVarTypeUInt8
    EvtSystemKeywords,                  // EvtVarTypeHexInt64
    EvtSystemTimeCreated,               // EvtVarTypeFileTime
    EvtSystemEventRecordId,             // EvtVarTypeUInt64
    EvtSystemActivityID,                // EvtVarTypeGuid
    EvtSystemRelatedActivityID,         // EvtVarTypeGuid
    EvtSystemProcessID,                 // EvtVarTypeUInt32
    EvtSystemThreadID,                  // EvtVarTypeUInt32
    EvtSystemChannel,                   // EvtVarTypeString
    EvtSystemComputer,                  // EvtVarTypeString
    EvtSystemUserID,                    // EvtVarTypeSid
    EvtSystemVersion,                   // EvtVarTypeUInt8
    EvtSystemPropertyIdEND

} EVT_SYSTEM_PROPERTY_ID;

typedef enum _EVT_RENDER_CONTEXT_FLAGS
{
    EvtRenderContextValues = 0,         // Render specific properties
    EvtRenderContextSystem,             // Render all system properties (System)
    EvtRenderContextUser                // Render all user properties (User/EventData)

 } EVT_RENDER_CONTEXT_FLAGS;

typedef enum _EVT_RENDER_FLAGS
{
    EvtRenderEventValues = 0,           // Variants
    EvtRenderEventXml,                  // XML
    EvtRenderBookmark                   // Bookmark

 } EVT_RENDER_FLAGS;

EVT_HANDLE
WINAPI
EvtCreateRenderContext(
    DWORD ValuePathsCount,
    _In_reads_opt_(ValuePathsCount) LPCWSTR* ValuePaths,
    DWORD Flags
    );

_Success_(return != 0)
BOOL
WINAPI
EvtRender(
    _In_opt_ EVT_HANDLE Context,
    _In_ EVT_HANDLE Fragment,
    DWORD Flags,
    DWORD BufferSize,
    _Out_writes_bytes_to_opt_(BufferSize, *BufferUsed) PVOID Buffer,
    _Out_ PDWORD BufferUsed,
    _Out_ PDWORD PropertyCount
    );

typedef enum _EVT_FORMAT_MESSAGE_FLAGS
{
    EvtFormatMessageEvent = 1,
    EvtFormatMessageLevel,
    EvtFormatMessageTask,
    EvtFormatMessageOpcode,
    EvtFormatMessageKeyword,
    EvtFormatMessageChannel,
    EvtFormatMessageProvider,
    EvtFormatMessageId,
    EvtFormatMessageXml,

 } EVT_FORMAT_MESSAGE_FLAGS;


// PublisherMetadata is NULL for forwarded events
_Success_(return != 0)
BOOL
WINAPI
EvtFormatMessage(
    _In_opt_ EVT_HANDLE PublisherMetadata,
    _In_opt_ EVT_HANDLE Event,
    DWORD MessageId,
    DWORD ValueCount,
    _In_reads_opt_(ValueCount) PEVT_VARIANT Values,
    DWORD Flags,
    DWORD BufferSize,
    _Out_writes_to_opt_(BufferSize, *BufferUsed) LPWSTR Buffer,
    _Out_ PDWORD BufferUsed
    );

////////////////////////////////////////////////////////////////////////////////
//
// Log Maintenace and Information
//
////////////////////////////////////////////////////////////////////////////////

typedef enum _EVT_OPEN_LOG_FLAGS
{
    EvtOpenChannelPath          = 0x1,
    EvtOpenFilePath             = 0x2

} EVT_OPEN_LOG_FLAGS;

typedef enum _EVT_LOG_PROPERTY_ID
{
    EvtLogCreationTime = 0,             // EvtVarTypeFileTime
    EvtLogLastAccessTime,               // EvtVarTypeFileTime
    EvtLogLastWriteTime,                // EvtVarTypeFileTime
    EvtLogFileSize,                     // EvtVarTypeUInt64
    EvtLogAttributes,                   // EvtVarTypeUInt32
    EvtLogNumberOfLogRecords,           // EvtVarTypeUInt64
    EvtLogOldestRecordNumber,           // EvtVarTypeUInt64
    EvtLogFull,                         // EvtVarTypeBoolean

} EVT_LOG_PROPERTY_ID;

EVT_HANDLE
WINAPI
EvtOpenLog(
    _In_opt_ EVT_HANDLE Session,
    _In_z_ LPCWSTR Path,
    DWORD Flags
    );

_Success_(return != 0)
BOOL
WINAPI
EvtGetLogInfo(
    _In_ EVT_HANDLE Log,
    EVT_LOG_PROPERTY_ID PropertyId,
    DWORD PropertyValueBufferSize,
    _Out_writes_bytes_to_opt_(PropertyValueBufferSize, *PropertyValueBufferUsed) PEVT_VARIANT PropertyValueBuffer,
    _Out_ PDWORD PropertyValueBufferUsed
    );

BOOL
WINAPI
EvtClearLog(
    _In_opt_ EVT_HANDLE Session,
    _In_z_ LPCWSTR ChannelPath,
    _In_opt_z_ LPCWSTR TargetFilePath,
    DWORD Flags
    );


typedef enum _EVT_EXPORTLOG_FLAGS
{
    EvtExportLogChannelPath     = 0x1,
    EvtExportLogFilePath        = 0x2,
    EvtExportLogTolerateQueryErrors = 0x1000,
    EvtExportLogOverwrite = 0x2000

} EVT_EXPORTLOG_FLAGS;

BOOL
WINAPI
EvtExportLog(
    _In_opt_ EVT_HANDLE Session,
    _In_opt_z_ LPCWSTR Path,
    _In_opt_z_ LPCWSTR Query,
    _In_z_ LPCWSTR TargetFilePath,
    DWORD Flags
    );


BOOL
WINAPI
EvtArchiveExportedLog(
    _In_opt_ EVT_HANDLE Session,
    _In_z_ LPCWSTR LogFilePath,
    LCID Locale,
    DWORD Flags
    );

////////////////////////////////////////////////////////////////////////////////
//
// Channel Configuration
//
////////////////////////////////////////////////////////////////////////////////

typedef enum _EVT_CHANNEL_CONFIG_PROPERTY_ID
{
    EvtChannelConfigEnabled = 0,            // EvtVarTypeBoolean
    EvtChannelConfigIsolation,              // EvtVarTypeUInt32, EVT_CHANNEL_ISOLATION_TYPE
    EvtChannelConfigType,                   // EvtVarTypeUInt32, EVT_CHANNEL_TYPE
    EvtChannelConfigOwningPublisher,        // EvtVarTypeString
    EvtChannelConfigClassicEventlog,        // EvtVarTypeBoolean
    EvtChannelConfigAccess,                 // EvtVarTypeString
    EvtChannelLoggingConfigRetention,       // EvtVarTypeBoolean
    EvtChannelLoggingConfigAutoBackup,      // EvtVarTypeBoolean
    EvtChannelLoggingConfigMaxSize,         // EvtVarTypeUInt64
    EvtChannelLoggingConfigLogFilePath,     // EvtVarTypeString
    EvtChannelPublishingConfigLevel,        // EvtVarTypeUInt32
    EvtChannelPublishingConfigKeywords,     // EvtVarTypeUInt64
    EvtChannelPublishingConfigControlGuid,  // EvtVarTypeGuid
    EvtChannelPublishingConfigBufferSize,   // EvtVarTypeUInt32
    EvtChannelPublishingConfigMinBuffers,   // EvtVarTypeUInt32
    EvtChannelPublishingConfigMaxBuffers,   // EvtVarTypeUInt32
    EvtChannelPublishingConfigLatency,      // EvtVarTypeUInt32
    EvtChannelPublishingConfigClockType,    // EvtVarTypeUInt32, EVT_CHANNEL_CLOCK_TYPE
    EvtChannelPublishingConfigSidType,      // EvtVarTypeUInt32, EVT_CHANNEL_SID_TYPE
    EvtChannelPublisherList,                // EvtVarTypeString | EVT_VARIANT_TYPE_ARRAY
    EvtChannelPublishingConfigFileMax,      // EvtVarTypeUint32
    EvtChannelConfigPropertyIdEND

} EVT_CHANNEL_CONFIG_PROPERTY_ID;

typedef enum _EVT_CHANNEL_TYPE
{
    EvtChannelTypeAdmin = 0,
    EvtChannelTypeOperational,
    EvtChannelTypeAnalytic,
    EvtChannelTypeDebug

} EVT_CHANNEL_TYPE;

typedef enum _EVT_CHANNEL_ISOLATION_TYPE
{
    EvtChannelIsolationTypeApplication = 0,
    EvtChannelIsolationTypeSystem,
    EvtChannelIsolationTypeCustom

} EVT_CHANNEL_ISOLATION_TYPE;

typedef enum _EVT_CHANNEL_CLOCK_TYPE
{
    EvtChannelClockTypeSystemTime = 0,      // System time
    EvtChannelClockTypeQPC                  // Query performance counter

} EVT_CHANNEL_CLOCK_TYPE;

typedef enum _EVT_CHANNEL_SID_TYPE
{
    EvtChannelSidTypeNone = 0,
    EvtChannelSidTypePublishing

} EVT_CHANNEL_SID_TYPE;


EVT_HANDLE
WINAPI
EvtOpenChannelEnum(
    _In_opt_ EVT_HANDLE Session,
    DWORD Flags
    );

_Success_(return != 0)
BOOL
WINAPI
EvtNextChannelPath(
    _In_ EVT_HANDLE ChannelEnum,
    DWORD ChannelPathBufferSize,
    _Out_writes_to_opt_(ChannelPathBufferSize, *ChannelPathBufferUsed) LPWSTR ChannelPathBuffer,
    _Out_ PDWORD ChannelPathBufferUsed
    );

EVT_HANDLE
WINAPI
EvtOpenChannelConfig(
    _In_opt_ EVT_HANDLE Session,
    _In_z_ LPCWSTR ChannelPath,
    DWORD Flags
    );


BOOL
WINAPI
EvtSaveChannelConfig(
    _In_ EVT_HANDLE ChannelConfig,
    DWORD Flags
    );

BOOL
WINAPI
EvtSetChannelConfigProperty(
    _In_ EVT_HANDLE ChannelConfig,
    EVT_CHANNEL_CONFIG_PROPERTY_ID PropertyId,
    DWORD Flags,
    _In_ PEVT_VARIANT PropertyValue
    );

_Success_(return != 0)
BOOL
WINAPI
EvtGetChannelConfigProperty(
    _In_ EVT_HANDLE ChannelConfig,
    EVT_CHANNEL_CONFIG_PROPERTY_ID PropertyId,
    DWORD Flags,
    DWORD PropertyValueBufferSize,
    _Out_writes_bytes_to_opt_(PropertyValueBufferSize, *PropertyValueBufferUsed) PEVT_VARIANT PropertyValueBuffer,
    _Out_ PDWORD PropertyValueBufferUsed
    );

////////////////////////////////////////////////////////////////////////////////
//
// Publisher Metadata
//
////////////////////////////////////////////////////////////////////////////////

typedef enum _EVT_CHANNEL_REFERENCE_FLAGS
{
    EvtChannelReferenceImported = 0x1,

} EVT_CHANNEL_REFERENCE_FLAGS;

typedef enum _EVT_PUBLISHER_METADATA_PROPERTY_ID
{
    EvtPublisherMetadataPublisherGuid = 0,      // EvtVarTypeGuid
    EvtPublisherMetadataResourceFilePath,       // EvtVarTypeString
    EvtPublisherMetadataParameterFilePath,      // EvtVarTypeString
    EvtPublisherMetadataMessageFilePath,        // EvtVarTypeString
    EvtPublisherMetadataHelpLink,               // EvtVarTypeString
    EvtPublisherMetadataPublisherMessageID,     // EvtVarTypeUInt32

    EvtPublisherMetadataChannelReferences,      // EvtVarTypeEvtHandle, ObjectArray
    EvtPublisherMetadataChannelReferencePath,   // EvtVarTypeString
    EvtPublisherMetadataChannelReferenceIndex,  // EvtVarTypeUInt32
    EvtPublisherMetadataChannelReferenceID,     // EvtVarTypeUInt32
    EvtPublisherMetadataChannelReferenceFlags,  // EvtVarTypeUInt32
    EvtPublisherMetadataChannelReferenceMessageID, // EvtVarTypeUInt32

    EvtPublisherMetadataLevels,                 // EvtVarTypeEvtHandle, ObjectArray
    EvtPublisherMetadataLevelName,              // EvtVarTypeString
    EvtPublisherMetadataLevelValue,             // EvtVarTypeUInt32
    EvtPublisherMetadataLevelMessageID,         // EvtVarTypeUInt32

    EvtPublisherMetadataTasks,                  // EvtVarTypeEvtHandle, ObjectArray
    EvtPublisherMetadataTaskName,               // EvtVarTypeString
    EvtPublisherMetadataTaskEventGuid,          // EvtVarTypeGuid
    EvtPublisherMetadataTaskValue,              // EvtVarTypeUInt32
    EvtPublisherMetadataTaskMessageID,          // EvtVarTypeUInt32

    EvtPublisherMetadataOpcodes,                // EvtVarTypeEvtHandle, ObjectArray
    EvtPublisherMetadataOpcodeName,             // EvtVarTypeString
    EvtPublisherMetadataOpcodeValue,            // EvtVarTypeUInt32
    EvtPublisherMetadataOpcodeMessageID,        // EvtVarTypeUInt32

    EvtPublisherMetadataKeywords,               // EvtVarTypeEvtHandle, ObjectArray
    EvtPublisherMetadataKeywordName,            // EvtVarTypeString
    EvtPublisherMetadataKeywordValue,           // EvtVarTypeUInt64
    EvtPublisherMetadataKeywordMessageID,       // EvtVarTypeUInt32

    EvtPublisherMetadataPropertyIdEND

} EVT_PUBLISHER_METADATA_PROPERTY_ID;


EVT_HANDLE
WINAPI
EvtOpenPublisherEnum(
    _In_opt_ EVT_HANDLE Session,
    DWORD Flags
    );

_Success_(return != 0)
BOOL
WINAPI
EvtNextPublisherId(
    _In_ EVT_HANDLE PublisherEnum,
    DWORD PublisherIdBufferSize,
    _Out_writes_to_opt_(PublisherIdBufferSize, *PublisherIdBufferUsed) LPWSTR PublisherIdBuffer,
    _Out_ PDWORD PublisherIdBufferUsed
    );

EVT_HANDLE
WINAPI
EvtOpenPublisherMetadata(
    _In_opt_ EVT_HANDLE Session,
    _In_z_ LPCWSTR PublisherId,
    _In_opt_z_ LPCWSTR LogFilePath,
    LCID Locale,
    DWORD Flags
    );

_Success_(return != 0)
BOOL
WINAPI
EvtGetPublisherMetadataProperty(
    _In_ EVT_HANDLE PublisherMetadata,
    EVT_PUBLISHER_METADATA_PROPERTY_ID PropertyId,
    DWORD Flags,
    DWORD PublisherMetadataPropertyBufferSize,
    _Out_writes_bytes_to_opt_(PublisherMetadataPropertyBufferSize, *PublisherMetadataPropertyBufferUsed) PEVT_VARIANT PublisherMetadataPropertyBuffer,
    _Out_ PDWORD PublisherMetadataPropertyBufferUsed
    );

////////////////////////////////////////////////////////////////////////////////
//
// Event Metadata Configuratin
//
////////////////////////////////////////////////////////////////////////////////

typedef enum _EVT_EVENT_METADATA_PROPERTY_ID
{
    EventMetadataEventID,       // EvtVarTypeUInt32
    EventMetadataEventVersion,  // EvtVarTypeUInt32
    EventMetadataEventChannel,  // EvtVarTypeUInt32
    EventMetadataEventLevel,    // EvtVarTypeUInt32
    EventMetadataEventOpcode,   // EvtVarTypeUInt32
    EventMetadataEventTask,     // EvtVarTypeUInt32
    EventMetadataEventKeyword,  // EvtVarTypeUInt64
    EventMetadataEventMessageID,// EvtVarTypeUInt32
    EventMetadataEventTemplate, // EvtVarTypeString
    EvtEventMetadataPropertyIdEND

} EVT_EVENT_METADATA_PROPERTY_ID;


EVT_HANDLE
WINAPI
EvtOpenEventMetadataEnum(
    _In_ EVT_HANDLE PublisherMetadata,
    DWORD Flags
    );

EVT_HANDLE
WINAPI
EvtNextEventMetadata(
    _In_ EVT_HANDLE EventMetadataEnum,
    DWORD Flags
    );

_Success_(return != 0)
BOOL
WINAPI
EvtGetEventMetadataProperty(
    _In_ EVT_HANDLE EventMetadata,
    EVT_EVENT_METADATA_PROPERTY_ID PropertyId,
    DWORD Flags,
    DWORD EventMetadataPropertyBufferSize,
    _Out_writes_bytes_to_opt_(EventMetadataPropertyBufferSize, *EventMetadataPropertyBufferUsed) PEVT_VARIANT EventMetadataPropertyBuffer,
    _Out_ PDWORD EventMetadataPropertyBufferUsed
    );

////////////////////////////////////////////////////////////////////////////////
//
// Array Access
//
////////////////////////////////////////////////////////////////////////////////

typedef HANDLE EVT_OBJECT_ARRAY_PROPERTY_HANDLE;


_Success_(return != 0)
BOOL
WINAPI
EvtGetObjectArraySize(
    _In_ EVT_OBJECT_ARRAY_PROPERTY_HANDLE ObjectArray,
    _Out_ PDWORD ObjectArraySize
    );

_Success_(return != 0)
BOOL
WINAPI
EvtGetObjectArrayProperty(
    _In_ EVT_OBJECT_ARRAY_PROPERTY_HANDLE ObjectArray,
    DWORD PropertyId,
    DWORD ArrayIndex,
    DWORD Flags,
    DWORD PropertyValueBufferSize,
    _Out_writes_bytes_to_opt_(PropertyValueBufferSize, *PropertyValueBufferUsed) PEVT_VARIANT PropertyValueBuffer,
    _Out_ PDWORD PropertyValueBufferUsed
    );

/////////////////////////////////////////////////////////////////////////////
//
// Misc Event Consumer Functions
//
////////////////////////////////////////////////////////////////////////////

typedef enum _EVT_QUERY_PROPERTY_ID
{
    //
    // list of channels or logfiles indentified in the query. Variant will be
    // array of EvtVarTypeString.
    //
    EvtQueryNames,

    //
    // Array of EvtVarTypeUInt32, indicating creation status ( Win32 error
    // code ) for the list of names returned by the EvtQueryNames
    // property.
    //
    EvtQueryStatuses,

    EvtQueryPropertyIdEND

} EVT_QUERY_PROPERTY_ID;

typedef enum _EVT_EVENT_PROPERTY_ID
{
    EvtEventQueryIDs = 0,
    EvtEventPath,
    EvtEventPropertyIdEND

} EVT_EVENT_PROPERTY_ID;


_Success_(return != 0)
BOOL
WINAPI
EvtGetQueryInfo(
    _In_ EVT_HANDLE QueryOrSubscription,
    EVT_QUERY_PROPERTY_ID PropertyId,
    DWORD PropertyValueBufferSize,
    _Out_writes_bytes_to_opt_(PropertyValueBufferSize, *PropertyValueBufferUsed) PEVT_VARIANT PropertyValueBuffer,
    _Out_ PDWORD PropertyValueBufferUsed
    );

EVT_HANDLE
WINAPI
EvtCreateBookmark(
    _In_opt_z_ LPCWSTR BookmarkXml
    );

BOOL
WINAPI
EvtUpdateBookmark(
    _In_ EVT_HANDLE Bookmark,
    _In_ EVT_HANDLE Event
    );

_Success_(return != 0)
BOOL
WINAPI
EvtGetEventInfo(
    _In_ EVT_HANDLE Event,
    EVT_EVENT_PROPERTY_ID PropertyId,
    DWORD PropertyValueBufferSize,
    _Out_writes_bytes_to_opt_(PropertyValueBufferSize, *PropertyValueBufferUsed) PEVT_VARIANT PropertyValueBuffer,
    _Out_ PDWORD PropertyValueBufferUsed
    );

////////////////////////////////////////////////////////////////////////////////
//
// Access Control Permissions
//
////////////////////////////////////////////////////////////////////////////////

#define EVT_READ_ACCESS    0x1
#define EVT_WRITE_ACCESS   0x2
#define EVT_CLEAR_ACCESS   0x4
#define EVT_ALL_ACCESS     0x7

#endif // WINVER >= _WIN32_WINNT_LONGHORN

#ifdef __cplusplus
}
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_EVENTLOGSERVICE) */
#pragma endregion

#endif //__WINEVT_H__


/*++

Copyright (c) Microsoft Corporation.  All rights reserved.

Module Name:

    cfapi.h

Abstract:

    This module defines the constants, data structures, and exported functions
    for the Cloud Files API.

--*/

#ifndef CFAPI_H
#define CFAPI_H

#if _MSC_VER > 1000
#pragma once
#endif

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION( WINAPI_PARTITION_DESKTOP )

#if ( NTDDI_VERSION >= NTDDI_WIN10_RS2 )

#pragma region Common Structures

#define CF_EOF ( -1LL )
#define CF_REQUEST_KEY_DEFAULT ( 0 )


#define DECLARE_OPAQUE_KEY( name )                                                                 \
    typedef struct name##__ {                                                                      \
        LONGLONG Internal;                                                                         \
    } name, *P##name

DECLARE_OPAQUE_KEY( CF_CONNECTION_KEY );

typedef LARGE_INTEGER CF_TRANSFER_KEY;
typedef LARGE_INTEGER CF_REQUEST_KEY;

#ifdef __cplusplus

#define DEFINE_USHORT_ENUM(ENUMTYPE)                                           \
  struct ENUMTYPE##_USHORT {                                                   \
    USHORT us;                                                                 \
    ENUMTYPE##_USHORT() { us = 0; }                                            \
    ENUMTYPE##_USHORT(const ENUMTYPE e) { us = (USHORT)e; }                    \
    operator ENUMTYPE() const { return (ENUMTYPE)us; }                         \
    ENUMTYPE##_USHORT &operator|=(const ENUMTYPE##_USHORT &e) {                \
      us |= e.us;                                                              \
      return *this;                                                            \
    }                                                                          \
    ENUMTYPE##_USHORT &operator&=(const ENUMTYPE##_USHORT &e) {                \
      us &= e.us;                                                              \
      return *this;                                                            \
    }                                                                          \
  };

#else

#define DEFINE_USHORT_ENUM( ENUMTYPE ) typedef USHORT ENUMTYPE##_USHORT

#endif

#define CF_PLACEHOLDER_MAX_FILE_IDENTITY_LENGTH 4096

typedef struct CF_FS_METADATA {
    FILE_BASIC_INFO BasicInfo;
    LARGE_INTEGER FileSize;
} CF_FS_METADATA;

typedef enum CF_PLACEHOLDER_CREATE_FLAGS {
    CF_PLACEHOLDER_CREATE_FLAG_NONE                         = 0x00000000,
    CF_PLACEHOLDER_CREATE_FLAG_DISABLE_ON_DEMAND_POPULATION = 0x00000001,
    CF_PLACEHOLDER_CREATE_FLAG_MARK_IN_SYNC                 = 0x00000002,
    CF_PLACEHOLDER_CREATE_FLAG_SUPERSEDE                    = 0x00000004,
    CF_PLACEHOLDER_CREATE_FLAG_ALWAYS_FULL                  = 0x00000008,
} CF_PLACEHOLDER_CREATE_FLAGS;

DEFINE_ENUM_FLAG_OPERATORS( CF_PLACEHOLDER_CREATE_FLAGS );

typedef struct CF_PLACEHOLDER_CREATE_INFO {
    LPCWSTR RelativeFileName;
    CF_FS_METADATA FsMetadata;
    LPCVOID FileIdentity;
    DWORD FileIdentityLength;
    CF_PLACEHOLDER_CREATE_FLAGS Flags;
    HRESULT Result;
    USN CreateUsn;
} CF_PLACEHOLDER_CREATE_INFO;

typedef enum CF_SYNC_PROVIDER_STATUS {
    CF_PROVIDER_STATUS_DISCONNECTED       = 0x00000000,
    CF_PROVIDER_STATUS_IDLE               = 0x00000001,
    CF_PROVIDER_STATUS_POPULATE_NAMESPACE = 0x00000002,
    CF_PROVIDER_STATUS_POPULATE_METADATA  = 0x00000004,
    CF_PROVIDER_STATUS_POPULATE_CONTENT   = 0x00000008,
    CF_PROVIDER_STATUS_SYNC_INCREMENTAL   = 0x00000010,
    CF_PROVIDER_STATUS_SYNC_FULL          = 0x00000020,
    CF_PROVIDER_STATUS_CONNECTIVITY_LOST  = 0x00000040,

    CF_PROVIDER_STATUS_CLEAR_FLAGS        = 0x80000000,
    CF_PROVIDER_STATUS_TERMINATED         = 0xC0000001,
    CF_PROVIDER_STATUS_ERROR              = 0xC0000002,
} CF_SYNC_PROVIDER_STATUS;

DEFINE_ENUM_FLAG_OPERATORS( CF_SYNC_PROVIDER_STATUS );

typedef struct CF_PROCESS_INFO {
    DWORD StructSize;
    DWORD ProcessId;
    PCWSTR ImagePath;
    PCWSTR PackageName;
    PCWSTR ApplicationId;
    PCWSTR CommandLine;
    DWORD SessionId;
} CF_PROCESS_INFO;

#pragma endregion


#pragma region Platform APIs

typedef struct CF_PLATFORM_INFO {
    DWORD BuildNumber;
    DWORD RevisionNumber;
    DWORD IntegrationNumber;
} CF_PLATFORM_INFO;

STDAPI
CfGetPlatformInfo (
    _Out_ CF_PLATFORM_INFO *PlatformVersion
    );

#pragma endregion


//
//  Sync Engine APIs
//

#pragma region Sync Engine APIs

typedef enum CF_REGISTER_FLAGS {
    CF_REGISTER_FLAG_NONE                                   = 0x00000000,
    CF_REGISTER_FLAG_UPDATE                                 = 0x00000001,
    CF_REGISTER_FLAG_DISABLE_ON_DEMAND_POPULATION_ON_ROOT   = 0x00000002,
    CF_REGISTER_FLAG_MARK_IN_SYNC_ON_ROOT                   = 0x00000004,
} CF_REGISTER_FLAGS;

DEFINE_ENUM_FLAG_OPERATORS( CF_REGISTER_FLAGS );

typedef enum CF_HYDRATION_POLICY_PRIMARY {
    CF_HYDRATION_POLICY_PARTIAL     = 0,
    CF_HYDRATION_POLICY_PROGRESSIVE = 1,
    CF_HYDRATION_POLICY_FULL        = 2,
    CF_HYDRATION_POLICY_ALWAYS_FULL = 3,
} CF_HYDRATION_POLICY_PRIMARY;

DEFINE_USHORT_ENUM( CF_HYDRATION_POLICY_PRIMARY );

typedef enum CF_HYDRATION_POLICY_MODIFIER {
    CF_HYDRATION_POLICY_MODIFIER_NONE                              = 0x0000,
    CF_HYDRATION_POLICY_MODIFIER_VALIDATION_REQUIRED               = 0x0001,
    CF_HYDRATION_POLICY_MODIFIER_STREAMING_ALLOWED                 = 0x0002,
    CF_HYDRATION_POLICY_MODIFIER_AUTO_DEHYDRATION_ALLOWED          = 0x0004,
    CF_HYDRATION_POLICY_MODIFIER_ALLOW_FULL_RESTART_HYDRATION      = 0x0008,
} CF_HYDRATION_POLICY_MODIFIER;

DEFINE_USHORT_ENUM( CF_HYDRATION_POLICY_MODIFIER );
DEFINE_ENUM_FLAG_OPERATORS( CF_HYDRATION_POLICY_MODIFIER );

typedef struct CF_HYDRATION_POLICY {
    CF_HYDRATION_POLICY_PRIMARY_USHORT Primary;
    CF_HYDRATION_POLICY_MODIFIER_USHORT Modifier;
} CF_HYDRATION_POLICY;

typedef enum CF_POPULATION_POLICY_PRIMARY {
    CF_POPULATION_POLICY_PARTIAL        = 0,
    CF_POPULATION_POLICY_FULL           = 2,
    CF_POPULATION_POLICY_ALWAYS_FULL    = 3,
} CF_POPULATION_POLICY_PRIMARY;

DEFINE_USHORT_ENUM( CF_POPULATION_POLICY_PRIMARY );

typedef enum CF_POPULATION_POLICY_MODIFIER {
    CF_POPULATION_POLICY_MODIFIER_NONE  = 0x0000,
} CF_POPULATION_POLICY_MODIFIER;

DEFINE_USHORT_ENUM( CF_POPULATION_POLICY_MODIFIER );
DEFINE_ENUM_FLAG_OPERATORS( CF_POPULATION_POLICY_MODIFIER );

typedef struct CF_POPULATION_POLICY {
    CF_POPULATION_POLICY_PRIMARY_USHORT Primary;
    CF_POPULATION_POLICY_MODIFIER_USHORT Modifier;
} CF_POPULATION_POLICY;

typedef enum CF_PLACEHOLDER_MANAGEMENT_POLICY {
    CF_PLACEHOLDER_MANAGEMENT_POLICY_DEFAULT          = 0x00000000,
    CF_PLACEHOLDER_MANAGEMENT_POLICY_CREATE_UNRESTRICTED     = 0x00000001,
    CF_PLACEHOLDER_MANAGEMENT_POLICY_CONVERT_TO_UNRESTRICTED = 0x00000002,
    CF_PLACEHOLDER_MANAGEMENT_POLICY_UPDATE_UNRESTRICTED     = 0x00000004,
} CF_PLACEHOLDER_MANAGEMENT_POLICY;

typedef enum CF_INSYNC_POLICY {
    CF_INSYNC_POLICY_NONE                               = 0x00000000,
    CF_INSYNC_POLICY_TRACK_FILE_CREATION_TIME           = 0x00000001,
    CF_INSYNC_POLICY_TRACK_FILE_READONLY_ATTRIBUTE      = 0x00000002,
    CF_INSYNC_POLICY_TRACK_FILE_HIDDEN_ATTRIBUTE        = 0x00000004,
    CF_INSYNC_POLICY_TRACK_FILE_SYSTEM_ATTRIBUTE        = 0x00000008,
    CF_INSYNC_POLICY_TRACK_DIRECTORY_CREATION_TIME      = 0x00000010,
    CF_INSYNC_POLICY_TRACK_DIRECTORY_READONLY_ATTRIBUTE = 0x00000020,
    CF_INSYNC_POLICY_TRACK_DIRECTORY_HIDDEN_ATTRIBUTE   = 0x00000040,
    CF_INSYNC_POLICY_TRACK_DIRECTORY_SYSTEM_ATTRIBUTE   = 0x00000080,
    CF_INSYNC_POLICY_TRACK_FILE_LAST_WRITE_TIME         = 0x00000100,
    CF_INSYNC_POLICY_TRACK_DIRECTORY_LAST_WRITE_TIME    = 0x00000200,
    CF_INSYNC_POLICY_TRACK_FILE_ALL                     = 0x0055550f,
    CF_INSYNC_POLICY_TRACK_DIRECTORY_ALL                = 0x00aaaaf0,
    CF_INSYNC_POLICY_TRACK_ALL                          = 0x00ffffff,
    CF_INSYNC_POLICY_PRESERVE_INSYNC_FOR_SYNC_ENGINE    = 0x80000000,
} CF_INSYNC_POLICY;

DEFINE_ENUM_FLAG_OPERATORS( CF_INSYNC_POLICY );

typedef enum CF_HARDLINK_POLICY {
    CF_HARDLINK_POLICY_NONE    = 0x00000000,
    CF_HARDLINK_POLICY_ALLOWED = 0x00000001,
} CF_HARDLINK_POLICY;

DEFINE_ENUM_FLAG_OPERATORS( CF_HARDLINK_POLICY );

typedef struct CF_SYNC_POLICIES {
    ULONG StructSize;
    CF_HYDRATION_POLICY Hydration;
    CF_POPULATION_POLICY Population;
    CF_INSYNC_POLICY InSync;
    CF_HARDLINK_POLICY HardLink;
    CF_PLACEHOLDER_MANAGEMENT_POLICY PlaceholderManagement;
} CF_SYNC_POLICIES;

typedef struct CF_SYNC_REGISTRATION {
    ULONG StructSize;
    LPCWSTR ProviderName;
    LPCWSTR ProviderVersion;
    _Field_size_bytes_( SyncRootIdentityLength ) LPCVOID SyncRootIdentity;
    DWORD SyncRootIdentityLength;
    _Field_size_bytes_( FileIdentityLength ) LPCVOID FileIdentity;
    DWORD FileIdentityLength;
    GUID ProviderId;
} CF_SYNC_REGISTRATION;

STDAPI
CfRegisterSyncRoot (
    _In_ LPCWSTR SyncRootPath,
    _In_ CONST CF_SYNC_REGISTRATION *Registration,
    _In_ CONST CF_SYNC_POLICIES *Policies,
    _In_ CF_REGISTER_FLAGS RegisterFlags
    );

STDAPI
CfUnregisterSyncRoot (
    _In_ LPCWSTR SyncRootPath
    );

#define CF_MAX_PRIORITY_HINT   15


typedef struct CF_CALLBACK_INFO {
    DWORD StructSize;
    CF_CONNECTION_KEY ConnectionKey;
    LPVOID CallbackContext;
    PCWSTR VolumeGuidName;
    PCWSTR VolumeDosName;
    DWORD VolumeSerialNumber;
    LARGE_INTEGER SyncRootFileId;
    _Field_size_bytes_( SyncRootIdentityLength ) LPCVOID SyncRootIdentity;
    DWORD SyncRootIdentityLength;
    LARGE_INTEGER FileId;
    LARGE_INTEGER FileSize;
    _Field_size_bytes_( FileIdentityLength ) LPCVOID FileIdentity;
    DWORD FileIdentityLength;
    PCWSTR NormalizedPath;
    CF_TRANSFER_KEY TransferKey;
    UCHAR PriorityHint;
    PCORRELATION_VECTOR CorrelationVector;
    CF_PROCESS_INFO* ProcessInfo;
    CF_REQUEST_KEY RequestKey;
} CF_CALLBACK_INFO;


typedef enum CF_CALLBACK_CANCEL_FLAGS {
    CF_CALLBACK_CANCEL_FLAG_NONE        = 0x00000000,
    CF_CALLBACK_CANCEL_FLAG_IO_TIMEOUT  = 0x00000001,
    CF_CALLBACK_CANCEL_FLAG_IO_ABORTED  = 0x00000002,
} CF_CALLBACK_CANCEL_FLAGS;

DEFINE_ENUM_FLAG_OPERATORS( CF_CALLBACK_CANCEL_FLAGS );

typedef enum CF_CALLBACK_FETCH_DATA_FLAGS {
    CF_CALLBACK_FETCH_DATA_FLAG_NONE                  = 0x00000000,
    CF_CALLBACK_FETCH_DATA_FLAG_RECOVERY              = 0x00000001,
    CF_CALLBACK_FETCH_DATA_FLAG_EXPLICIT_HYDRATION    = 0x00000002,
} CF_CALLBACK_FETCH_DATA_FLAGS;

DEFINE_ENUM_FLAG_OPERATORS( CF_CALLBACK_FETCH_DATA_FLAGS );

typedef enum CF_CALLBACK_VALIDATE_DATA_FLAGS {
    CF_CALLBACK_VALIDATE_DATA_FLAG_NONE                 = 0x00000000,
    CF_CALLBACK_VALIDATE_DATA_FLAG_EXPLICIT_HYDRATION   = 0x00000002,
} CF_CALLBACK_VALIDATE_DATA_FLAGS;

DEFINE_ENUM_FLAG_OPERATORS( CF_CALLBACK_VALIDATE_DATA_FLAGS );

typedef enum CF_CALLBACK_FETCH_PLACEHOLDERS_FLAGS {
    CF_CALLBACK_FETCH_PLACEHOLDERS_FLAG_NONE = 0x00000000,
} CF_CALLBACK_FETCH_PLACEHOLDERS_FLAGS;

DEFINE_ENUM_FLAG_OPERATORS( CF_CALLBACK_FETCH_PLACEHOLDERS_FLAGS );

typedef enum CF_CALLBACK_OPEN_COMPLETION_FLAGS {
    CF_CALLBACK_OPEN_COMPLETION_FLAG_NONE                    = 0x00000000,
    CF_CALLBACK_OPEN_COMPLETION_FLAG_PLACEHOLDER_UNKNOWN     = 0x00000001,
    CF_CALLBACK_OPEN_COMPLETION_FLAG_PLACEHOLDER_UNSUPPORTED = 0x00000002,
} CF_CALLBACK_OPEN_COMPLETION_FLAGS;

DEFINE_ENUM_FLAG_OPERATORS( CF_CALLBACK_OPEN_COMPLETION_FLAGS );

typedef enum CF_CALLBACK_CLOSE_COMPLETION_FLAGS {
    CF_CALLBACK_CLOSE_COMPLETION_FLAG_NONE       = 0x00000000,
    CF_CALLBACK_CLOSE_COMPLETION_FLAG_DELETED    = 0x00000001,
} CF_CALLBACK_CLOSE_COMPLETION_FLAGS;

DEFINE_ENUM_FLAG_OPERATORS( CF_CALLBACK_CLOSE_COMPLETION_FLAGS );

typedef enum CF_CALLBACK_DEHYDRATE_FLAGS {
    CF_CALLBACK_DEHYDRATE_FLAG_NONE            = 0x00000000,
    CF_CALLBACK_DEHYDRATE_FLAG_BACKGROUND      = 0x00000001,
} CF_CALLBACK_DEHYDRATE_FLAGS;

DEFINE_ENUM_FLAG_OPERATORS( CF_CALLBACK_DEHYDRATE_FLAGS );

typedef enum CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS {
    CF_CALLBACK_DEHYDRATE_COMPLETION_FLAG_NONE          = 0x00000000,
    CF_CALLBACK_DEHYDRATE_COMPLETION_FLAG_BACKGROUND    = 0x00000001,
    CF_CALLBACK_DEHYDRATE_COMPLETION_FLAG_DEHYDRATED    = 0x00000002,
} CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS;

DEFINE_ENUM_FLAG_OPERATORS( CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS );

typedef enum CF_CALLBACK_DELETE_FLAGS {
    CF_CALLBACK_DELETE_FLAG_NONE         = 0x00000000,
    CF_CALLBACK_DELETE_FLAG_IS_DIRECTORY = 0x00000001,
    CF_CALLBACK_DELETE_FLAG_IS_UNDELETE  = 0x00000002,
} CF_CALLBACK_DELETE_FLAGS;

DEFINE_ENUM_FLAG_OPERATORS( CF_CALLBACK_DELETE_FLAGS );

typedef enum CF_CALLBACK_DELETE_COMPLETION_FLAGS {
    CF_CALLBACK_DELETE_COMPLETION_FLAG_NONE = 0x00000000,
} CF_CALLBACK_DELETE_COMPLETION_FLAGS;

DEFINE_ENUM_FLAG_OPERATORS( CF_CALLBACK_DELETE_COMPLETION_FLAGS );

typedef enum CF_CALLBACK_RENAME_FLAGS {
    CF_CALLBACK_RENAME_FLAG_NONE            = 0x00000000,
    CF_CALLBACK_RENAME_FLAG_IS_DIRECTORY    = 0x00000001,
    CF_CALLBACK_RENAME_FLAG_SOURCE_IN_SCOPE = 0x00000002,
    CF_CALLBACK_RENAME_FLAG_TARGET_IN_SCOPE = 0x00000004,
} CF_CALLBACK_RENAME_FLAGS;

DEFINE_ENUM_FLAG_OPERATORS( CF_CALLBACK_RENAME_FLAGS );

typedef enum CF_CALLBACK_RENAME_COMPLETION_FLAGS {
    CF_CALLBACK_RENAME_COMPLETION_FLAG_NONE = 0x00000000,
} CF_CALLBACK_RENAME_COMPLETION_FLAGS;

DEFINE_ENUM_FLAG_OPERATORS( CF_CALLBACK_RENAME_COMPLETION_FLAGS );

typedef enum CF_CALLBACK_DEHYDRATION_REASON {

    CF_CALLBACK_DEHYDRATION_REASON_NONE,
    CF_CALLBACK_DEHYDRATION_REASON_USER_MANUAL,
    CF_CALLBACK_DEHYDRATION_REASON_SYSTEM_LOW_SPACE,
    CF_CALLBACK_DEHYDRATION_REASON_SYSTEM_INACTIVITY,
    CF_CALLBACK_DEHYDRATION_REASON_SYSTEM_OS_UPGRADE,

} CF_CALLBACK_DEHYDRATION_REASON;

#pragma warning( push )
#pragma warning( disable : 4201 )

typedef struct CF_CALLBACK_PARAMETERS {

    ULONG ParamSize;

    union {

        struct {

            CF_CALLBACK_CANCEL_FLAGS Flags;

            union {

                struct {
                    LARGE_INTEGER FileOffset;
                    LARGE_INTEGER Length;
                } FetchData;

            } DUMMYUNIONNAME;

        } Cancel;

        struct {
            CF_CALLBACK_FETCH_DATA_FLAGS Flags;
            LARGE_INTEGER RequiredFileOffset;
            LARGE_INTEGER RequiredLength;
            LARGE_INTEGER OptionalFileOffset;
            LARGE_INTEGER OptionalLength;
            LARGE_INTEGER LastDehydrationTime;
            CF_CALLBACK_DEHYDRATION_REASON LastDehydrationReason;
        } FetchData;

        struct {
            CF_CALLBACK_VALIDATE_DATA_FLAGS Flags;
            LARGE_INTEGER RequiredFileOffset;
            LARGE_INTEGER RequiredLength;
        } ValidateData;

        struct {
            CF_CALLBACK_FETCH_PLACEHOLDERS_FLAGS Flags;
            PCWSTR Pattern;
        } FetchPlaceholders;

        struct {
            CF_CALLBACK_OPEN_COMPLETION_FLAGS Flags;
        } OpenCompletion;

        struct {
            CF_CALLBACK_CLOSE_COMPLETION_FLAGS Flags;
        } CloseCompletion;

        struct {
            CF_CALLBACK_DEHYDRATE_FLAGS Flags;
            CF_CALLBACK_DEHYDRATION_REASON Reason;
        } Dehydrate;

        struct {
            CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS Flags;
            CF_CALLBACK_DEHYDRATION_REASON Reason;
        } DehydrateCompletion;

        struct {
            CF_CALLBACK_DELETE_FLAGS Flags;
        } Delete;

        struct {
            CF_CALLBACK_DELETE_COMPLETION_FLAGS Flags;
        } DeleteCompletion;

        struct {
            CF_CALLBACK_RENAME_FLAGS Flags;
            PCWSTR TargetPath;
        } Rename;

        struct {
            CF_CALLBACK_RENAME_COMPLETION_FLAGS Flags;
            PCWSTR SourcePath;
        } RenameCompletion;

    } DUMMYUNIONNAME;

} CF_CALLBACK_PARAMETERS;

#pragma warning( pop )

typedef
VOID
(CALLBACK *CF_CALLBACK) (
    _In_ CONST CF_CALLBACK_INFO *CallbackInfo,
    _In_ CONST CF_CALLBACK_PARAMETERS *CallbackParameters
    );

typedef enum CF_CALLBACK_TYPE {
    CF_CALLBACK_TYPE_FETCH_DATA,
    CF_CALLBACK_TYPE_VALIDATE_DATA,
    CF_CALLBACK_TYPE_CANCEL_FETCH_DATA,
    CF_CALLBACK_TYPE_FETCH_PLACEHOLDERS,
    CF_CALLBACK_TYPE_CANCEL_FETCH_PLACEHOLDERS,
    CF_CALLBACK_TYPE_NOTIFY_FILE_OPEN_COMPLETION,
    CF_CALLBACK_TYPE_NOTIFY_FILE_CLOSE_COMPLETION,
    CF_CALLBACK_TYPE_NOTIFY_DEHYDRATE,
    CF_CALLBACK_TYPE_NOTIFY_DEHYDRATE_COMPLETION,
    CF_CALLBACK_TYPE_NOTIFY_DELETE,
    CF_CALLBACK_TYPE_NOTIFY_DELETE_COMPLETION,
    CF_CALLBACK_TYPE_NOTIFY_RENAME,
    CF_CALLBACK_TYPE_NOTIFY_RENAME_COMPLETION,
    CF_CALLBACK_TYPE_NONE = 0xffffffff
} CF_CALLBACK_TYPE;

typedef struct CF_CALLBACK_REGISTRATION {
    CF_CALLBACK_TYPE Type;
    CF_CALLBACK Callback;
} CF_CALLBACK_REGISTRATION;

#define CF_CALLBACK_REGISTRATION_END {CF_CALLBACK_TYPE_NONE, NULL}

typedef enum CF_CONNECT_FLAGS {
    CF_CONNECT_FLAG_NONE                            = 0x00000000,
    CF_CONNECT_FLAG_REQUIRE_PROCESS_INFO            = 0x00000002,
    CF_CONNECT_FLAG_REQUIRE_FULL_FILE_PATH          = 0x00000004,
    CF_CONNECT_FLAG_BLOCK_SELF_IMPLICIT_HYDRATION   = 0x00000008,
} CF_CONNECT_FLAGS;

DEFINE_ENUM_FLAG_OPERATORS( CF_CONNECT_FLAGS );

STDAPI
CfConnectSyncRoot (
    _In_ LPCWSTR SyncRootPath,
    _In_ CONST CF_CALLBACK_REGISTRATION *CallbackTable,
    _In_opt_ LPCVOID CallbackContext,
    _In_ CF_CONNECT_FLAGS ConnectFlags,
    _Out_ CF_CONNECTION_KEY *ConnectionKey
    );

STDAPI
CfDisconnectSyncRoot (
    _In_ CF_CONNECTION_KEY ConnectionKey
    );

STDAPI
CfGetTransferKey(
    _In_ HANDLE FileHandle,
    _Out_ CF_TRANSFER_KEY* TransferKey
    );

STDAPI_(VOID)
CfReleaseTransferKey(
    _In_ HANDLE FileHandle,
    _Out_ CF_TRANSFER_KEY* TransferKey
    );


typedef enum CF_OPERATION_TYPE {
    CF_OPERATION_TYPE_TRANSFER_DATA,
    CF_OPERATION_TYPE_RETRIEVE_DATA,
    CF_OPERATION_TYPE_ACK_DATA,
    CF_OPERATION_TYPE_RESTART_HYDRATION,
    CF_OPERATION_TYPE_TRANSFER_PLACEHOLDERS,
    CF_OPERATION_TYPE_ACK_DEHYDRATE,
    CF_OPERATION_TYPE_ACK_DELETE,
    CF_OPERATION_TYPE_ACK_RENAME,
} CF_OPERATION_TYPE;


typedef struct CF_SYNC_STATUS {
    ULONG StructSize;
    ULONG Code;
    ULONG DescriptionOffset;
    ULONG DescriptionLength;
    ULONG DeviceIdOffset;
    ULONG DeviceIdLength;
} CF_SYNC_STATUS;


typedef struct CF_OPERATION_INFO {
    ULONG StructSize;
    CF_OPERATION_TYPE Type;
    CF_CONNECTION_KEY ConnectionKey;
    CF_TRANSFER_KEY TransferKey;
    CONST CORRELATION_VECTOR* CorrelationVector;
    CONST CF_SYNC_STATUS* SyncStatus;
    CF_REQUEST_KEY RequestKey;
} CF_OPERATION_INFO;


typedef enum CF_OPERATION_TRANSFER_DATA_FLAGS {
    CF_OPERATION_TRANSFER_DATA_FLAG_NONE     = 0x00000000,
} CF_OPERATION_TRANSFER_DATA_FLAGS;

DEFINE_ENUM_FLAG_OPERATORS( CF_OPERATION_TRANSFER_DATA_FLAGS );

typedef enum CF_OPERATION_RETRIEVE_DATA_FLAGS {
    CF_OPERATION_RETRIEVE_DATA_FLAG_NONE     = 0x00000000,
} CF_OPERATION_RETRIEVE_DATA_FLAGS;

DEFINE_ENUM_FLAG_OPERATORS( CF_OPERATION_RETRIEVE_DATA_FLAGS );

typedef enum CF_OPERATION_ACK_DATA_FLAGS {
    CF_OPERATION_ACK_DATA_FLAG_NONE = 0x00000000,
} CF_OPERATION_ACK_DATA_FLAGS;

DEFINE_ENUM_FLAG_OPERATORS( CF_OPERATION_ACK_DATA_FLAGS );

typedef enum CF_OPERATION_RESTART_HYDRATION_FLAGS {
    CF_OPERATION_RESTART_HYDRATION_FLAG_NONE            = 0x00000000,
    CF_OPERATION_RESTART_HYDRATION_FLAG_MARK_IN_SYNC    = 0x00000001,
} CF_OPERATION_RESTART_HYDRATION_FLAGS;

DEFINE_ENUM_FLAG_OPERATORS( CF_OPERATION_RESTART_HYDRATION_FLAGS );

typedef enum CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS {
    CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAG_NONE               = 0x00000000,
    CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAG_STOP_ON_ERROR      = 0x00000001,
    CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAG_DISABLE_ON_DEMAND_POPULATION = 0x00000002,
} CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS;

DEFINE_ENUM_FLAG_OPERATORS( CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS );

typedef enum CF_OPERATION_ACK_DEHYDRATE_FLAGS {
    CF_OPERATION_ACK_DEHYDRATE_FLAG_NONE                  = 0x00000000,
} CF_OPERATION_ACK_DEHYDRATE_FLAGS;

DEFINE_ENUM_FLAG_OPERATORS( CF_OPERATION_ACK_DEHYDRATE_FLAGS );

typedef enum CF_OPERATION_ACK_RENAME_FLAGS {
    CF_OPERATION_ACK_RENAME_FLAG_NONE        = 0x00000000,
} CF_OPERATION_ACK_RENAME_FLAGS;

DEFINE_ENUM_FLAG_OPERATORS( CF_OPERATION_ACK_RENAME_FLAGS );

typedef enum CF_OPERATION_ACK_DELETE_FLAGS {
    CF_OPERATION_ACK_DELETE_FLAG_NONE        = 0x00000000,
} CF_OPERATION_ACK_DELETE_FLAGS;

DEFINE_ENUM_FLAG_OPERATORS( CF_OPERATION_ACK_DELETE_FLAGS );

#pragma warning( push )
#pragma warning( disable : 4201 )

typedef struct CF_OPERATION_PARAMETERS {

    ULONG ParamSize;

    union {

        struct {
            CF_OPERATION_TRANSFER_DATA_FLAGS Flags;
            NTSTATUS CompletionStatus;
            _Field_size_bytes_( Length.QuadPart ) LPCVOID Buffer;
            LARGE_INTEGER Offset;
            LARGE_INTEGER Length;
        } TransferData;

        struct {
            CF_OPERATION_RETRIEVE_DATA_FLAGS Flags;
            _Field_size_bytes_( Length.QuadPart ) LPVOID Buffer;
            LARGE_INTEGER Offset;
            LARGE_INTEGER Length;
            LARGE_INTEGER ReturnedLength;
        } RetrieveData;

        struct {
            CF_OPERATION_ACK_DATA_FLAGS Flags;
            NTSTATUS CompletionStatus;
            LARGE_INTEGER Offset;
            LARGE_INTEGER Length;
        } AckData;

        struct {
            CF_OPERATION_RESTART_HYDRATION_FLAGS Flags;
            CONST CF_FS_METADATA* FsMetadata;
            _Field_size_bytes_( FileIdentityLength ) LPCVOID FileIdentity;
            DWORD FileIdentityLength;
        } RestartHydration;

        struct {
            CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS Flags;
            NTSTATUS CompletionStatus;
            LARGE_INTEGER PlaceholderTotalCount;
            CF_PLACEHOLDER_CREATE_INFO* PlaceholderArray;
            DWORD PlaceholderCount;
            DWORD EntriesProcessed;
        } TransferPlaceholders;

        struct {
            CF_OPERATION_ACK_DEHYDRATE_FLAGS Flags;
            NTSTATUS CompletionStatus;
            _Field_size_bytes_( FileIdentityLength ) LPCVOID FileIdentity;
            DWORD FileIdentityLength;
        } AckDehydrate;

        struct {
            CF_OPERATION_ACK_RENAME_FLAGS Flags;
            NTSTATUS CompletionStatus;
        } AckRename;

        struct {
            CF_OPERATION_ACK_DELETE_FLAGS Flags;
            NTSTATUS CompletionStatus;
        } AckDelete;

    } DUMMYUNIONNAME;

} CF_OPERATION_PARAMETERS;

#pragma warning( pop )

STDAPI
CfExecute(
    _In_ CONST CF_OPERATION_INFO *OpInfo,
    _Inout_  CF_OPERATION_PARAMETERS *OpParams
    );


STDAPI
CfUpdateSyncProviderStatus(
    _In_ CF_CONNECTION_KEY ConnectionKey,
    _In_ CF_SYNC_PROVIDER_STATUS ProviderStatus
    );

STDAPI
CfQuerySyncProviderStatus(
    _In_ CF_CONNECTION_KEY ConnectionKey,
    _Out_ CF_SYNC_PROVIDER_STATUS *ProviderStatus
    );

#pragma endregion


STDAPI
CfReportSyncStatus(
    _In_ LPCWSTR SyncRootPath,
    _In_opt_ CF_SYNC_STATUS *SyncStatus
    );


//
//  Placeholder Management APIs
//

#pragma region Placeholder Management APIs

typedef enum CF_CREATE_FLAGS {
    CF_CREATE_FLAG_NONE          = 0x00000000,
    CF_CREATE_FLAG_STOP_ON_ERROR = 0x00000001,
} CF_CREATE_FLAGS;

DEFINE_ENUM_FLAG_OPERATORS( CF_CREATE_FLAGS );

STDAPI
CfCreatePlaceholders (
    _In_ LPCWSTR BaseDirectoryPath,
    _Inout_updates_(PlaceholderCount) CF_PLACEHOLDER_CREATE_INFO *PlaceholderArray,
    _In_ DWORD PlaceholderCount,
    _In_ CF_CREATE_FLAGS CreateFlags,
    _Out_opt_ PDWORD EntriesProcessed
    );

typedef enum CF_OPEN_FILE_FLAGS {
    CF_OPEN_FILE_FLAG_NONE                   = 0x00000000,
    CF_OPEN_FILE_FLAG_EXCLUSIVE              = 0x00000001,
    CF_OPEN_FILE_FLAG_WRITE_ACCESS           = 0x00000002,
    CF_OPEN_FILE_FLAG_DELETE_ACCESS          = 0x00000004,
    CF_OPEN_FILE_FLAG_FOREGROUND             = 0x00000008,
} CF_OPEN_FILE_FLAGS;

DEFINE_ENUM_FLAG_OPERATORS( CF_OPEN_FILE_FLAGS );

STDAPI
CfOpenFileWithOplock (
    _In_ LPCWSTR FilePath,
    _In_ CF_OPEN_FILE_FLAGS Flags,
    _Out_ PHANDLE ProtectedHandle
    );

STDAPI_( BOOLEAN )
CfReferenceProtectedHandle (
    _In_ HANDLE ProtectedHandle
    );

STDAPI_( HANDLE )
CfGetWin32HandleFromProtectedHandle (
    _In_ HANDLE ProtectedHandle
    );

STDAPI_( VOID )
CfReleaseProtectedHandle(
    _In_ HANDLE ProtectedHandle
    );

STDAPI_(VOID)
CfCloseHandle(
    _In_ HANDLE FileHandle
    );

typedef struct CF_FILE_RANGE {
    LARGE_INTEGER StartingOffset;
    LARGE_INTEGER Length;
} CF_FILE_RANGE;

typedef enum CF_CONVERT_FLAGS {
    CF_CONVERT_FLAG_NONE                        = 0x00000000,
    CF_CONVERT_FLAG_MARK_IN_SYNC                = 0x00000001,
    CF_CONVERT_FLAG_DEHYDRATE                   = 0x00000002,
    CF_CONVERT_FLAG_ENABLE_ON_DEMAND_POPULATION = 0x00000004,
    CF_CONVERT_FLAG_ALWAYS_FULL                 = 0x00000008,
    CF_CONVERT_FLAG_FORCE_CONVERT_TO_CLOUD_FILE = 0x00000010,
} CF_CONVERT_FLAGS;

DEFINE_ENUM_FLAG_OPERATORS( CF_CONVERT_FLAGS );

STDAPI
CfConvertToPlaceholder (
    _In_ HANDLE FileHandle,
    _In_reads_bytes_opt_(FileIdentityLength) LPCVOID FileIdentity,
    _In_ DWORD FileIdentityLength,
    _In_ CF_CONVERT_FLAGS ConvertFlags,
    _Out_opt_ USN *ConvertUsn,
    _Inout_opt_ LPOVERLAPPED Overlapped
    );

typedef enum CF_UPDATE_FLAGS {
    CF_UPDATE_FLAG_NONE                         = 0x00000000,
    CF_UPDATE_FLAG_VERIFY_IN_SYNC               = 0x00000001,
    CF_UPDATE_FLAG_MARK_IN_SYNC                 = 0x00000002,
    CF_UPDATE_FLAG_DEHYDRATE                    = 0x00000004,
    CF_UPDATE_FLAG_ENABLE_ON_DEMAND_POPULATION  = 0x00000008,
    CF_UPDATE_FLAG_DISABLE_ON_DEMAND_POPULATION = 0x00000010,
    CF_UPDATE_FLAG_REMOVE_FILE_IDENTITY         = 0x00000020,
    CF_UPDATE_FLAG_CLEAR_IN_SYNC                = 0x00000040,
    CF_UPDATE_FLAG_REMOVE_PROPERTY              = 0x00000080,
    CF_UPDATE_FLAG_PASSTHROUGH_FS_METADATA      = 0x00000100,
    CF_UPDATE_FLAG_ALWAYS_FULL                  = 0x00000200,
    CF_UPDATE_FLAG_ALLOW_PARTIAL                = 0x00000400,
} CF_UPDATE_FLAGS;

DEFINE_ENUM_FLAG_OPERATORS( CF_UPDATE_FLAGS );

STDAPI
CfUpdatePlaceholder (
    _In_ HANDLE FileHandle,
    _In_opt_ CONST CF_FS_METADATA *FsMetadata,
    _In_reads_bytes_opt_(FileIdentityLength) LPCVOID FileIdentity,
    _In_ DWORD FileIdentityLength,
    _In_reads_opt_(DehydrateRangeCount) CONST CF_FILE_RANGE *DehydrateRangeArray,
    _In_ DWORD DehydrateRangeCount,
    _In_ CF_UPDATE_FLAGS UpdateFlags,
    _Inout_opt_ USN *UpdateUsn,
    _Inout_opt_ LPOVERLAPPED Overlapped
    );

typedef enum CF_REVERT_FLAGS {
    CF_REVERT_FLAG_NONE              = 0x00000000,
} CF_REVERT_FLAGS;

DEFINE_ENUM_FLAG_OPERATORS( CF_REVERT_FLAGS );

STDAPI
CfRevertPlaceholder (
    _In_ HANDLE FileHandle,
    _In_ CF_REVERT_FLAGS RevertFlags,
    _Inout_opt_ LPOVERLAPPED Overlapped
    );

typedef enum CF_HYDRATE_FLAGS {
    CF_HYDRATE_FLAG_NONE             = 0x00000000,
} CF_HYDRATE_FLAGS;

DEFINE_ENUM_FLAG_OPERATORS( CF_HYDRATE_FLAGS );

STDAPI
CfHydratePlaceholder (
    _In_ HANDLE FileHandle,
    _In_ LARGE_INTEGER StartingOffset,
    _In_ LARGE_INTEGER Length,
    _In_ CF_HYDRATE_FLAGS HydrateFlags,
    _Inout_opt_ LPOVERLAPPED Overlapped
    );

typedef enum CF_DEHYDRATE_FLAGS {
    CF_DEHYDRATE_FLAG_NONE                       = 0x00000000,
    CF_DEHYDRATE_FLAG_BACKGROUND                 = 0x00000001,
} CF_DEHYDRATE_FLAGS;

DEFINE_ENUM_FLAG_OPERATORS( CF_DEHYDRATE_FLAGS );

STDAPI
CfDehydratePlaceholder (
    _In_ HANDLE FileHandle,
    _In_ LARGE_INTEGER StartingOffset,
    _In_ LARGE_INTEGER Length,
    _In_ CF_DEHYDRATE_FLAGS DehydrateFlags,
    _Inout_opt_ LPOVERLAPPED Overlapped
    );

typedef enum CF_PIN_STATE {
    CF_PIN_STATE_UNSPECIFIED = 0,
    CF_PIN_STATE_PINNED = 1,
    CF_PIN_STATE_UNPINNED = 2,
    CF_PIN_STATE_EXCLUDED = 3,
    CF_PIN_STATE_INHERIT = 4  // set only
} CF_PIN_STATE;

typedef enum CF_SET_PIN_FLAGS {
    CF_SET_PIN_FLAG_NONE                  = 0x00000000,
    CF_SET_PIN_FLAG_RECURSE               = 0x00000001,
    CF_SET_PIN_FLAG_RECURSE_ONLY          = 0x00000002,
    CF_SET_PIN_FLAG_RECURSE_STOP_ON_ERROR = 0x00000004
} CF_SET_PIN_FLAGS;

DEFINE_ENUM_FLAG_OPERATORS( CF_SET_PIN_FLAGS );

STDAPI
CfSetPinState (
    _In_ HANDLE FileHandle,
    _In_ CF_PIN_STATE PinState,
    _In_ CF_SET_PIN_FLAGS PinFlags,
    _Inout_opt_ LPOVERLAPPED Overlapped
    );

typedef enum CF_IN_SYNC_STATE {
    CF_IN_SYNC_STATE_NOT_IN_SYNC    = 0,
    CF_IN_SYNC_STATE_IN_SYNC        = 1
} CF_IN_SYNC_STATE;

typedef enum CF_SET_IN_SYNC_FLAGS {
    CF_SET_IN_SYNC_FLAG_NONE = 0x00000000,
} CF_SET_IN_SYNC_FLAGS;

DEFINE_ENUM_FLAG_OPERATORS( CF_SET_IN_SYNC_FLAGS );

STDAPI
CfSetInSyncState (
    _In_ HANDLE FileHandle,
    _In_ CF_IN_SYNC_STATE InSyncState,
    _In_ CF_SET_IN_SYNC_FLAGS InSyncFlags,
    _Inout_opt_ USN *InSyncUsn
    );

STDAPI
CfSetCorrelationVector (
    _In_ HANDLE FileHandle,
    _In_ const PCORRELATION_VECTOR CorrelationVector
    );

STDAPI
CfGetCorrelationVector (
    _In_ HANDLE FileHandle,
    _Out_ PCORRELATION_VECTOR CorrelationVector
    );

typedef enum CF_PLACEHOLDER_STATE {
    CF_PLACEHOLDER_STATE_NO_STATES                      = 0x00000000,
    CF_PLACEHOLDER_STATE_PLACEHOLDER                    = 0x00000001,
    CF_PLACEHOLDER_STATE_SYNC_ROOT                      = 0x00000002,
    CF_PLACEHOLDER_STATE_ESSENTIAL_PROP_PRESENT         = 0x00000004,
    CF_PLACEHOLDER_STATE_IN_SYNC                        = 0x00000008,
    CF_PLACEHOLDER_STATE_PARTIAL                        = 0x00000010,
    CF_PLACEHOLDER_STATE_PARTIALLY_ON_DISK              = 0x00000020,
    CF_PLACEHOLDER_STATE_INVALID                        = 0xffffffff,
} CF_PLACEHOLDER_STATE;

DEFINE_ENUM_FLAG_OPERATORS( CF_PLACEHOLDER_STATE );

STDAPI_( CF_PLACEHOLDER_STATE )
CfGetPlaceholderStateFromAttributeTag (
    _In_ DWORD FileAttributes,
    _In_ DWORD ReparseTag
    );

STDAPI_( CF_PLACEHOLDER_STATE )
CfGetPlaceholderStateFromFileInfo (
    _In_ LPCVOID InfoBuffer,
    _In_ FILE_INFO_BY_HANDLE_CLASS InfoClass
    );

STDAPI_( CF_PLACEHOLDER_STATE )
CfGetPlaceholderStateFromFindData (
    _In_ CONST WIN32_FIND_DATA *FindData
    );

typedef enum CF_PLACEHOLDER_INFO_CLASS {

    CF_PLACEHOLDER_INFO_BASIC = 0,
    CF_PLACEHOLDER_INFO_STANDARD = 1,

} CF_PLACEHOLDER_INFO_CLASS;

//
// Placeholder basic info
//

typedef struct CF_PLACEHOLDER_BASIC_INFO {

    CF_PIN_STATE PinState;
    CF_IN_SYNC_STATE InSyncState;
    LARGE_INTEGER FileId;
    LARGE_INTEGER SyncRootFileId;
    ULONG FileIdentityLength;
    BYTE FileIdentity[1];

} CF_PLACEHOLDER_BASIC_INFO;

//
// Placeholder standard info
//

typedef struct CF_PLACEHOLDER_STANDARD_INFO {

    LARGE_INTEGER OnDiskDataSize;
    LARGE_INTEGER ValidatedDataSize;
    LARGE_INTEGER ModifiedDataSize;
    LARGE_INTEGER PropertiesSize;
    CF_PIN_STATE PinState;
    CF_IN_SYNC_STATE InSyncState;
    LARGE_INTEGER FileId;
    LARGE_INTEGER SyncRootFileId;
    ULONG FileIdentityLength;
    BYTE FileIdentity[1];

} CF_PLACEHOLDER_STANDARD_INFO;

STDAPI
CfGetPlaceholderInfo (
    _In_ HANDLE FileHandle,
    _In_ CF_PLACEHOLDER_INFO_CLASS InfoClass,
    _Out_bytecapcount_(InfoBufferLength) PVOID InfoBuffer,
    _In_ DWORD InfoBufferLength,
    _Out_opt_ PDWORD ReturnedLength
    );

typedef enum CF_SYNC_ROOT_INFO_CLASS {

    CF_SYNC_ROOT_INFO_BASIC = 0,
    CF_SYNC_ROOT_INFO_STANDARD = 1,
    CF_SYNC_ROOT_INFO_PROVIDER = 2,

} CF_SYNC_ROOT_INFO_CLASS;

//
// SyncRoot basic info
//

typedef struct CF_SYNC_ROOT_BASIC_INFO {

    LARGE_INTEGER SyncRootFileId;

} CF_SYNC_ROOT_BASIC_INFO;


#define CF_MAX_PROVIDER_NAME_LENGTH            255
#define CF_MAX_PROVIDER_VERSION_LENGTH         255


//
// SyncRoot provider info
//

typedef struct CF_SYNC_ROOT_PROVIDER_INFO {

    CF_SYNC_PROVIDER_STATUS ProviderStatus;
    WCHAR ProviderName[CF_MAX_PROVIDER_NAME_LENGTH + 1];
    WCHAR ProviderVersion[CF_MAX_PROVIDER_VERSION_LENGTH + 1];

} CF_SYNC_ROOT_PROVIDER_INFO;


//
// SyncRoot standard info
//

typedef struct CF_SYNC_ROOT_STANDARD_INFO {

    LARGE_INTEGER SyncRootFileId;
    CF_HYDRATION_POLICY HydrationPolicy;
    CF_POPULATION_POLICY PopulationPolicy;
    CF_INSYNC_POLICY InSyncPolicy;
    CF_HARDLINK_POLICY HardLinkPolicy;
    CF_SYNC_PROVIDER_STATUS ProviderStatus;
    WCHAR ProviderName[CF_MAX_PROVIDER_NAME_LENGTH + 1];
    WCHAR ProviderVersion[CF_MAX_PROVIDER_VERSION_LENGTH + 1];
    ULONG SyncRootIdentityLength;
    BYTE SyncRootIdentity[1];

} CF_SYNC_ROOT_STANDARD_INFO;


STDAPI
CfGetSyncRootInfoByPath(
    _In_ LPCWSTR FilePath,
    _In_ CF_SYNC_ROOT_INFO_CLASS InfoClass,
    _Out_ PVOID InfoBuffer,
    _In_ DWORD InfoBufferLength,
    _Out_opt_ DWORD *ReturnedLength
    );

STDAPI
CfGetSyncRootInfoByHandle(
    _In_ HANDLE FileHandle,
    _In_ CF_SYNC_ROOT_INFO_CLASS InfoClass,
    _Out_ PVOID InfoBuffer,
    _In_ DWORD InfoBufferLength,
    _Out_opt_ DWORD *ReturnedLength
    );

typedef enum CF_PLACEHOLDER_RANGE_INFO_CLASS {

    CF_PLACEHOLDER_RANGE_INFO_ONDISK = 1,
    CF_PLACEHOLDER_RANGE_INFO_VALIDATED = 2,
    CF_PLACEHOLDER_RANGE_INFO_MODIFIED = 3,

} CF_PLACEHOLDER_RANGE_INFO_CLASS;

STDAPI
CfGetPlaceholderRangeInfo (
    _In_ HANDLE FileHandle,
    _In_ CF_PLACEHOLDER_RANGE_INFO_CLASS InfoClass,
    _In_ LARGE_INTEGER StartingOffset,
    _In_ LARGE_INTEGER Length,
    _Out_bytecapcount_(InfoBufferLength) PVOID InfoBuffer,
    _In_ DWORD InfoBufferLength,
    _Out_opt_ PDWORD ReturnedLength
    );

STDAPI
CfGetPlaceholderRangeInfoForHydration(
    _In_ CF_CONNECTION_KEY ConnectionKey,
    _In_ CF_TRANSFER_KEY TransferKey,    
    _In_ LARGE_INTEGER FileId,
    _In_ CF_PLACEHOLDER_RANGE_INFO_CLASS InfoClass,
    _In_ LARGE_INTEGER StartingOffset,
    _In_ LARGE_INTEGER RangeLength,
    _Out_writes_bytes_to_(InfoBufferSize, *InfoBufferWritten) PVOID InfoBuffer,
    _In_ DWORD InfoBufferSize,
    _Out_opt_ PDWORD InfoBufferWritten
    );


#pragma endregion


//
//  Progress APIs
//

#pragma region Progress APIs

STDAPI
CfReportProviderProgress (
    _In_ CF_CONNECTION_KEY ConnectionKey,
    _In_ CF_TRANSFER_KEY TransferKey,
    _In_ LARGE_INTEGER ProviderProgressTotal,
    _In_ LARGE_INTEGER ProviderProgressCompleted
    );

#if ( NTDDI_VERSION >= NTDDI_WIN10_RS5 )

STDAPI
CfReportProviderProgress2(
    _In_ CF_CONNECTION_KEY ConnectionKey,
    _In_ CF_TRANSFER_KEY TransferKey,
    _In_ CF_REQUEST_KEY RequestKey,
    _In_ LARGE_INTEGER ProviderProgressTotal,
    _In_ LARGE_INTEGER ProviderProgressCompleted,
    _In_ DWORD TargetSessionId
    );

#endif // #if (NTDDI_VERSION >= NTDDI_WIN10_RS2)

#pragma endregion


#pragma region Deprecated APIs

#pragma endregion

#endif // #if (NTDDI_VERSION >= NTDDI_WIN10_RS2)

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#endif // CFAPI_H

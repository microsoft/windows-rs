/*++    

Copyright (c) Microsoft Corporation.  All rights reserved.

Module Name:

    projectedfslib.h

Abstract:

    This module contains the user-mode APIs for the Projected File System (ProjFS) library.

    ProjFS enables an application, the "provider", to use these APIs to present a store of hierarchical
    data as a projection into the file system.  The on-disk location of these files are rooted in a
    directory called a "virtualization root".  The provider chooses this directory and manages all
    files within it.  The virtualization root together with its descendants comprises a "virtualization
    instance".
    
    For example, a hypothetical ProjFS registry provider could present the keys and values in the Windows
    Registry as a tree of directories and files rooted at C:\Registry.  A single provider may manage
    multiple virtualization instances, each with its own virtualization root.  To extend the example,
    a ProjFS registry provider could choose to establish a separate virtualization instance for each
    registry hive, resulting in directory trees rooted at C:\Registry\HKEY_CLASSES_ROOT,
    C:\Registry\HKEY_CURRENT_USER, etc.  Note that a virtualization root cannot be a descendant of
    another virtualization root.


--*/

#ifndef PROJECTEDFSLIB_H
#define PROJECTEDFSLIB_H

#if _MSC_VER > 1000
#pragma once
#endif

#pragma warning(disable:4201) // nameless struct/union

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN10_RS5)

#pragma region Common structures

typedef enum PRJ_NOTIFY_TYPES
{
    PRJ_NOTIFY_NONE                                 = 0x00000000,
    PRJ_NOTIFY_SUPPRESS_NOTIFICATIONS               = 0x00000001,
    PRJ_NOTIFY_FILE_OPENED                          = 0x00000002,
    PRJ_NOTIFY_NEW_FILE_CREATED                     = 0x00000004,
    PRJ_NOTIFY_FILE_OVERWRITTEN                     = 0x00000008,
    PRJ_NOTIFY_PRE_DELETE                           = 0x00000010,
    PRJ_NOTIFY_PRE_RENAME                           = 0x00000020,
    PRJ_NOTIFY_PRE_SET_HARDLINK                     = 0x00000040,
    PRJ_NOTIFY_FILE_RENAMED                         = 0x00000080,
    PRJ_NOTIFY_HARDLINK_CREATED                     = 0x00000100,
    PRJ_NOTIFY_FILE_HANDLE_CLOSED_NO_MODIFICATION   = 0x00000200,
    PRJ_NOTIFY_FILE_HANDLE_CLOSED_FILE_MODIFIED     = 0x00000400,
    PRJ_NOTIFY_FILE_HANDLE_CLOSED_FILE_DELETED      = 0x00000800,
    PRJ_NOTIFY_FILE_PRE_CONVERT_TO_FULL             = 0x00001000,
    PRJ_NOTIFY_USE_EXISTING_MASK                    = 0xFFFFFFFF
} PRJ_NOTIFY_TYPES;

DEFINE_ENUM_FLAG_OPERATORS(PRJ_NOTIFY_TYPES);

// This enum shares the same value space as PRJ_NOTIFY_TYPES, but
// these values are not bit flags.
typedef enum PRJ_NOTIFICATION
{
    PRJ_NOTIFICATION_FILE_OPENED                        = 0x00000002,
    PRJ_NOTIFICATION_NEW_FILE_CREATED                   = 0x00000004,
    PRJ_NOTIFICATION_FILE_OVERWRITTEN                   = 0x00000008,
    PRJ_NOTIFICATION_PRE_DELETE                         = 0x00000010,
    PRJ_NOTIFICATION_PRE_RENAME                         = 0x00000020,
    PRJ_NOTIFICATION_PRE_SET_HARDLINK                   = 0x00000040,
    PRJ_NOTIFICATION_FILE_RENAMED                       = 0x00000080,
    PRJ_NOTIFICATION_HARDLINK_CREATED                   = 0x00000100,
    PRJ_NOTIFICATION_FILE_HANDLE_CLOSED_NO_MODIFICATION = 0x00000200,
    PRJ_NOTIFICATION_FILE_HANDLE_CLOSED_FILE_MODIFIED   = 0x00000400,
    PRJ_NOTIFICATION_FILE_HANDLE_CLOSED_FILE_DELETED    = 0x00000800,
    PRJ_NOTIFICATION_FILE_PRE_CONVERT_TO_FULL           = 0x00001000,
} PRJ_NOTIFICATION;

DECLARE_HANDLE(PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT);

DECLARE_HANDLE(PRJ_DIR_ENTRY_BUFFER_HANDLE);

typedef enum PRJ_EXT_INFO_TYPE {
    PRJ_EXT_INFO_TYPE_SYMLINK = 1
} PRJ_EXT_INFO_TYPE;


typedef struct PRJ_EXTENDED_INFO {

    PRJ_EXT_INFO_TYPE InfoType;

    ULONG NextInfoOffset;

    union {
        struct {
            PCWSTR TargetName;
        } Symlink;
    } DUMMYUNIONNAME;

} PRJ_EXTENDED_INFO;

//
//  Forward declaration.
//

typedef struct PRJ_CALLBACKS PRJ_CALLBACKS;

#pragma endregion

#pragma region Virtualization instance APIs

typedef struct PRJ_NOTIFICATION_MAPPING
{
    PRJ_NOTIFY_TYPES NotificationBitMask;
    PCWSTR NotificationRoot;
} PRJ_NOTIFICATION_MAPPING;

typedef enum PRJ_STARTVIRTUALIZING_FLAGS
{
    PRJ_FLAG_NONE                       = 0x00000000,
    PRJ_FLAG_USE_NEGATIVE_PATH_CACHE    = 0x00000001
} PRJ_STARTVIRTUALIZING_FLAGS;

DEFINE_ENUM_FLAG_OPERATORS(PRJ_STARTVIRTUALIZING_FLAGS);

typedef struct PRJ_STARTVIRTUALIZING_OPTIONS
{
    PRJ_STARTVIRTUALIZING_FLAGS Flags;
    UINT32 PoolThreadCount;
    UINT32 ConcurrentThreadCount;
    PRJ_NOTIFICATION_MAPPING* NotificationMappings;
    UINT32 NotificationMappingsCount;
} PRJ_STARTVIRTUALIZING_OPTIONS;

STDAPI
PrjStartVirtualizing(
    _In_ PCWSTR virtualizationRootPath,
    _In_ const PRJ_CALLBACKS* callbacks,
    _In_opt_ const void* instanceContext,
    _In_opt_ const PRJ_STARTVIRTUALIZING_OPTIONS* options,
    _Outptr_ PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT* namespaceVirtualizationContext
    );

STDAPI_(void)
PrjStopVirtualizing(
    _In_ PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT namespaceVirtualizationContext
    );

STDAPI
PrjClearNegativePathCache(
    _In_ PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT namespaceVirtualizationContext,
    _Out_opt_ UINT32* totalEntryNumber
    );

typedef struct PRJ_VIRTUALIZATION_INSTANCE_INFO
{
    GUID InstanceID;
    UINT32 WriteAlignment;
} PRJ_VIRTUALIZATION_INSTANCE_INFO;

STDAPI
PrjGetVirtualizationInstanceInfo(
    _In_ PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT namespaceVirtualizationContext,
    _Out_ PRJ_VIRTUALIZATION_INSTANCE_INFO* virtualizationInstanceInfo
    );

#pragma endregion

#pragma region Placeholder and File APIs

typedef enum PRJ_PLACEHOLDER_ID
{
    PRJ_PLACEHOLDER_ID_LENGTH = 128
} PRJ_PLACEHOLDER_ID;

typedef struct PRJ_PLACEHOLDER_VERSION_INFO
{
    UINT8 ProviderID[PRJ_PLACEHOLDER_ID_LENGTH];
    UINT8 ContentID[PRJ_PLACEHOLDER_ID_LENGTH];
} PRJ_PLACEHOLDER_VERSION_INFO;

STDAPI
PrjMarkDirectoryAsPlaceholder(
    _In_ PCWSTR rootPathName,
    _In_opt_ PCWSTR targetPathName,
    _In_opt_ const PRJ_PLACEHOLDER_VERSION_INFO* versionInfo,
    _In_ const GUID* virtualizationInstanceID
    );

typedef struct PRJ_FILE_BASIC_INFO {
    BOOLEAN IsDirectory;
    INT64 FileSize;
    LARGE_INTEGER CreationTime;
    LARGE_INTEGER LastAccessTime;
    LARGE_INTEGER LastWriteTime;
    LARGE_INTEGER ChangeTime;
    UINT32 FileAttributes;
} PRJ_FILE_BASIC_INFO;

typedef struct PRJ_PLACEHOLDER_INFO
{
    PRJ_FILE_BASIC_INFO FileBasicInfo;

    struct
    {
        UINT32 EaBufferSize;
        UINT32 OffsetToFirstEa;
    } EaInformation;

    struct
    {
        UINT32 SecurityBufferSize;
        UINT32 OffsetToSecurityDescriptor;
    } SecurityInformation;

    struct
    {
        UINT32 StreamsInfoBufferSize;
        UINT32 OffsetToFirstStreamInfo;
    } StreamsInformation;

    PRJ_PLACEHOLDER_VERSION_INFO VersionInfo;
    UINT8 VariableData[1];
} PRJ_PLACEHOLDER_INFO;

STDAPI
PrjWritePlaceholderInfo(
    _In_ PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT namespaceVirtualizationContext,
    _In_ PCWSTR destinationFileName,
    _In_reads_bytes_(placeholderInfoSize) const PRJ_PLACEHOLDER_INFO* placeholderInfo,
    _In_ UINT32 placeholderInfoSize
    );

STDAPI
PrjWritePlaceholderInfo2(
    _In_ PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT namespaceVirtualizationContext,
    _In_ PCWSTR destinationFileName,
    _In_reads_bytes_(placeholderInfoSize) const PRJ_PLACEHOLDER_INFO* placeholderInfo,
    _In_ UINT32 placeholderInfoSize,
    _In_opt_ const PRJ_EXTENDED_INFO* ExtendedInfo
    );

typedef enum PRJ_UPDATE_TYPES
{
    PRJ_UPDATE_NONE                 = 0x00000000,
    PRJ_UPDATE_ALLOW_DIRTY_METADATA = 0x00000001,
    PRJ_UPDATE_ALLOW_DIRTY_DATA     = 0x00000002,
    PRJ_UPDATE_ALLOW_TOMBSTONE      = 0x00000004,
    PRJ_UPDATE_RESERVED1            = 0x00000008,
    PRJ_UPDATE_RESERVED2            = 0x00000010,
    PRJ_UPDATE_ALLOW_READ_ONLY      = 0x00000020,
    PRJ_UPDATE_MAX_VAL = (PRJ_UPDATE_ALLOW_READ_ONLY << 1)
} PRJ_UPDATE_TYPES;

DEFINE_ENUM_FLAG_OPERATORS(PRJ_UPDATE_TYPES);

typedef enum PRJ_UPDATE_FAILURE_CAUSES
{
    PRJ_UPDATE_FAILURE_CAUSE_NONE           = 0x00000000,
    PRJ_UPDATE_FAILURE_CAUSE_DIRTY_METADATA = 0x00000001,
    PRJ_UPDATE_FAILURE_CAUSE_DIRTY_DATA     = 0x00000002,
    PRJ_UPDATE_FAILURE_CAUSE_TOMBSTONE      = 0x00000004,
    PRJ_UPDATE_FAILURE_CAUSE_READ_ONLY      = 0x00000008,
} PRJ_UPDATE_FAILURE_CAUSES;

DEFINE_ENUM_FLAG_OPERATORS(PRJ_UPDATE_FAILURE_CAUSES);

STDAPI
PrjUpdateFileIfNeeded(
    _In_ PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT namespaceVirtualizationContext,
    _In_ PCWSTR destinationFileName,
    _In_reads_bytes_(placeholderInfoSize) const PRJ_PLACEHOLDER_INFO* placeholderInfo,
    _In_ UINT32 placeholderInfoSize,
    _In_opt_ PRJ_UPDATE_TYPES updateFlags,
    _Out_opt_ PRJ_UPDATE_FAILURE_CAUSES* failureReason
    );

STDAPI
PrjDeleteFile(
    _In_ PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT namespaceVirtualizationContext,
    _In_ PCWSTR destinationFileName,
    _In_opt_ PRJ_UPDATE_TYPES updateFlags,
    _Out_opt_ PRJ_UPDATE_FAILURE_CAUSES* failureReason
   );

STDAPI
PrjWriteFileData(
    _In_ PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT namespaceVirtualizationContext,
    _In_ const GUID* dataStreamId,
    _In_reads_bytes_(length) void* buffer,
    _In_ UINT64 byteOffset,
    _In_ UINT32 length
    );

typedef enum PRJ_FILE_STATE
{
    PRJ_FILE_STATE_PLACEHOLDER          = 0x00000001,
    PRJ_FILE_STATE_HYDRATED_PLACEHOLDER = 0x00000002,
    PRJ_FILE_STATE_DIRTY_PLACEHOLDER    = 0x00000004,
    PRJ_FILE_STATE_FULL                 = 0x00000008,
    PRJ_FILE_STATE_TOMBSTONE            = 0x00000010,
} PRJ_FILE_STATE;

DEFINE_ENUM_FLAG_OPERATORS(PRJ_FILE_STATE);

STDAPI
PrjGetOnDiskFileState(
    _In_ PCWSTR destinationFileName,
    _Out_ PRJ_FILE_STATE* fileState
    );

STDAPI_(void*)
PrjAllocateAlignedBuffer (
    _In_ PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT namespaceVirtualizationContext,
    _In_ size_t size
    );

STDAPI_(void)
PrjFreeAlignedBuffer (
    _In_ void* buffer
    );

#pragma endregion

#pragma region Callback support

typedef enum PRJ_CALLBACK_DATA_FLAGS
{
    PRJ_CB_DATA_FLAG_ENUM_RESTART_SCAN          = 0x00000001,
    PRJ_CB_DATA_FLAG_ENUM_RETURN_SINGLE_ENTRY   = 0x00000002
} PRJ_CALLBACK_DATA_FLAGS;

typedef struct PRJ_CALLBACK_DATA
{
    UINT32 Size;
    PRJ_CALLBACK_DATA_FLAGS Flags;
    PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT NamespaceVirtualizationContext;
    INT32 CommandId;
    GUID FileId;
    GUID DataStreamId;
    PCWSTR FilePathName;
    PRJ_PLACEHOLDER_VERSION_INFO* VersionInfo;
    UINT32 TriggeringProcessId;
    PCWSTR TriggeringProcessImageFileName;
    void* InstanceContext;
} PRJ_CALLBACK_DATA;

typedef
_Function_class_(PRJ_START_DIRECTORY_ENUMERATION_CB)
HRESULT
(CALLBACK PRJ_START_DIRECTORY_ENUMERATION_CB)(
    _In_ const PRJ_CALLBACK_DATA* callbackData,
    _In_ const GUID* enumerationId
    );

typedef
_Function_class_(PRJ_GET_DIRECTORY_ENUMERATION_CB)
HRESULT
(CALLBACK PRJ_GET_DIRECTORY_ENUMERATION_CB)(
    _In_ const PRJ_CALLBACK_DATA* callbackData,
    _In_ const GUID* enumerationId,
    _In_opt_ PCWSTR searchExpression,
    _In_ PRJ_DIR_ENTRY_BUFFER_HANDLE dirEntryBufferHandle
    );

typedef
_Function_class_(PRJ_END_DIRECTORY_ENUMERATION_CB)
HRESULT
(CALLBACK PRJ_END_DIRECTORY_ENUMERATION_CB)(
    _In_ const PRJ_CALLBACK_DATA* callbackData,
    _In_ const GUID* enumerationId
    );

typedef
_Function_class_(PRJ_GET_PLACEHOLDER_INFO_CB)
HRESULT
(CALLBACK PRJ_GET_PLACEHOLDER_INFO_CB)(
    _In_ const PRJ_CALLBACK_DATA* callbackData
    );

typedef
_Function_class_(PRJ_GET_FILE_DATA_CB)
HRESULT
(CALLBACK PRJ_GET_FILE_DATA_CB)(
    _In_ const PRJ_CALLBACK_DATA* callbackData,
    _In_ UINT64 byteOffset,
    _In_ UINT32 length
    );

typedef
_Function_class_(PRJ_QUERY_FILE_NAME_CB)
HRESULT
(CALLBACK PRJ_QUERY_FILE_NAME_CB)(
    _In_ const PRJ_CALLBACK_DATA* callbackData
    );

typedef union PRJ_NOTIFICATION_PARAMETERS
{
    struct {
        PRJ_NOTIFY_TYPES NotificationMask;
    } PostCreate;

    struct {
        PRJ_NOTIFY_TYPES NotificationMask;
    } FileRenamed;

    struct {
        BOOLEAN IsFileModified;
    } FileDeletedOnHandleClose;
} PRJ_NOTIFICATION_PARAMETERS;

typedef
_Function_class_(PRJ_NOTIFICATION_CB)
HRESULT
(CALLBACK PRJ_NOTIFICATION_CB)(
    _In_ const PRJ_CALLBACK_DATA* callbackData,
    _In_ BOOLEAN isDirectory,
    _In_ PRJ_NOTIFICATION notification,
    _In_opt_ PCWSTR destinationFileName,
    _Inout_ PRJ_NOTIFICATION_PARAMETERS* operationParameters
    );

typedef
_Function_class_(PRJ_CANCEL_COMMAND_CB)
VOID
(CALLBACK PRJ_CANCEL_COMMAND_CB)(
    _In_ const PRJ_CALLBACK_DATA* callbackData
    );

typedef struct PRJ_CALLBACKS {
    PRJ_START_DIRECTORY_ENUMERATION_CB* StartDirectoryEnumerationCallback;
    PRJ_END_DIRECTORY_ENUMERATION_CB* EndDirectoryEnumerationCallback;
    PRJ_GET_DIRECTORY_ENUMERATION_CB* GetDirectoryEnumerationCallback;
    PRJ_GET_PLACEHOLDER_INFO_CB* GetPlaceholderInfoCallback;
    PRJ_GET_FILE_DATA_CB* GetFileDataCallback;

    PRJ_QUERY_FILE_NAME_CB* QueryFileNameCallback;
    PRJ_NOTIFICATION_CB* NotificationCallback;
    PRJ_CANCEL_COMMAND_CB* CancelCommandCallback;
} PRJ_CALLBACKS;

typedef enum PRJ_COMPLETE_COMMAND_TYPE
{
    PRJ_COMPLETE_COMMAND_TYPE_NOTIFICATION = 1,
    PRJ_COMPLETE_COMMAND_TYPE_ENUMERATION = 2
} PRJ_COMPLETE_COMMAND_TYPE;

typedef struct PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS {

    PRJ_COMPLETE_COMMAND_TYPE CommandType;

    union {
        struct {
            PRJ_NOTIFY_TYPES NotificationMask;
        } Notification;

        struct {
            PRJ_DIR_ENTRY_BUFFER_HANDLE DirEntryBufferHandle;
        } Enumeration;
    } DUMMYUNIONNAME;

} PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS;

STDAPI
PrjCompleteCommand(
    _In_ PRJ_NAMESPACE_VIRTUALIZATION_CONTEXT namespaceVirtualizationContext,
    _In_ INT32 commandId,
    _In_ HRESULT completionResult,
    _In_opt_ PRJ_COMPLETE_COMMAND_EXTENDED_PARAMETERS* extendedParameters
    );

#pragma endregion

#pragma region Enumeration APIs

STDAPI
PrjFillDirEntryBuffer(
    _In_ PCWSTR fileName,
    _In_opt_ PRJ_FILE_BASIC_INFO* fileBasicInfo,
    _In_ PRJ_DIR_ENTRY_BUFFER_HANDLE dirEntryBufferHandle
    );

STDAPI
PrjFillDirEntryBuffer2(
    _In_ PRJ_DIR_ENTRY_BUFFER_HANDLE dirEntryBufferHandle,
    _In_ PCWSTR fileName,
    _In_opt_ PRJ_FILE_BASIC_INFO* fileBasicInfo,
    _In_opt_ PRJ_EXTENDED_INFO* extendedInfo
    );

STDAPI_(BOOLEAN)
PrjFileNameMatch (
    _In_ PCWSTR fileNameToCheck,
    _In_ PCWSTR pattern
    );

STDAPI_(int)
PrjFileNameCompare (
    _In_ PCWSTR fileName1,
    _In_ PCWSTR fileName2
    );

STDAPI_(BOOLEAN)
PrjDoesNameContainWildCards (
    _In_ LPCWSTR fileName
    );

#pragma endregion

#endif // _WIN32_WINNT >= _WIN32_WINNT_WIN10_RS5
#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#endif // PROJECTEDFSLIB_H

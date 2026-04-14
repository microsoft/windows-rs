

/* this ALWAYS GENERATED file contains the definitions for the interfaces */


 /* File created by MIDL compiler version 8.01.0628 */
/* @@MIDL_FILE_HEADING(  ) */



/* verify that the <rpcndr.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCNDR_H_VERSION__
#define __REQUIRED_RPCNDR_H_VERSION__ 501
#endif

/* verify that the <rpcsal.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCSAL_H_VERSION__
#define __REQUIRED_RPCSAL_H_VERSION__ 100
#endif

#include "rpc.h"
#include "rpcndr.h"

#ifndef __RPCNDR_H_VERSION__
#error this stub requires an updated version of <rpcndr.h>
#endif /* __RPCNDR_H_VERSION__ */


#ifndef __vdssys_h__
#define __vdssys_h__

#if defined(_MSC_VER) && (_MSC_VER >= 1020)
#pragma once
#endif

#ifndef DECLSPEC_XFGVIRT
#if defined(_CONTROL_FLOW_GUARD_XFG)
#define DECLSPEC_XFGVIRT(base, func) __declspec(xfg_virtual(base, func))
#else
#define DECLSPEC_XFGVIRT(base, func)
#endif
#endif

/* Forward Declarations */ 

/* header files for imported files */
#include "wtypes.h"
#include "oaidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_vdssys_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#if (WINVER >= _WIN32_WINNT_WIN7)
#if defined(VDS_MIDL_PASS)
typedef LPWSTR PWSTR;

typedef LPCWSTR PCWSTR;

typedef DWORD ACCESS_MASK;

#pragma warning(push)
#pragma warning(disable:4668) 
#pragma warning(disable:4001) 
#pragma once
#pragma warning(pop)
typedef PVOID PSECURITY_DESCRIPTOR;

typedef PVOID LPOVERLAPPED;

typedef ULONG *PULONG;

typedef HANDLE *PHANDLE;

#pragma once
#pragma warning(push)
#pragma warning(disable:4668) 
#pragma once
#pragma region Input Buffer SAL 1 compatibility macros
#pragma endregion Input Buffer SAL 1 compatibility macros
#pragma once
#pragma once
#pragma warning(pop)
#pragma warning(push)
#pragma warning(disable:4001) 
#pragma once
#pragma warning(push)
#pragma warning(disable:4001) 
#pragma once
#pragma warning(pop)
#pragma warning(pop)
#pragma region Desktop Family or VHD Package
#pragma warning(push)
#pragma warning(disable:4820) 
#pragma warning(disable:4200) 
#pragma warning(disable:4201) 
typedef struct _VIRTUAL_STORAGE_TYPE
    {
    ULONG DeviceId;
    GUID VendorId;
    } 	VIRTUAL_STORAGE_TYPE;

typedef struct _VIRTUAL_STORAGE_TYPE *PVIRTUAL_STORAGE_TYPE;

typedef 
enum _OPEN_VIRTUAL_DISK_VERSION
    {
        OPEN_VIRTUAL_DISK_VERSION_UNSPECIFIED	= 0,
        OPEN_VIRTUAL_DISK_VERSION_1	= 1,
        OPEN_VIRTUAL_DISK_VERSION_2	= 2,
        OPEN_VIRTUAL_DISK_VERSION_3	= 3
    } 	OPEN_VIRTUAL_DISK_VERSION;

typedef struct _OPEN_VIRTUAL_DISK_PARAMETERS
    {
    OPEN_VIRTUAL_DISK_VERSION Version;
    union 
        {
        struct 
            {
            ULONG RWDepth;
            } 	Version1;
        struct 
            {
            BOOL GetInfoOnly;
            BOOL ReadOnly;
            GUID ResiliencyGuid;
            } 	Version2;
        struct 
            {
            BOOL GetInfoOnly;
            BOOL ReadOnly;
            GUID ResiliencyGuid;
            GUID SnapshotId;
            } 	Version3;
        } 	;
    } 	OPEN_VIRTUAL_DISK_PARAMETERS;

typedef struct _OPEN_VIRTUAL_DISK_PARAMETERS *POPEN_VIRTUAL_DISK_PARAMETERS;

typedef /* [v1_enum] */ 
enum _VIRTUAL_DISK_ACCESS_MASK
    {
        VIRTUAL_DISK_ACCESS_NONE	= 0,
        VIRTUAL_DISK_ACCESS_ATTACH_RO	= 0x10000,
        VIRTUAL_DISK_ACCESS_ATTACH_RW	= 0x20000,
        VIRTUAL_DISK_ACCESS_DETACH	= 0x40000,
        VIRTUAL_DISK_ACCESS_GET_INFO	= 0x80000,
        VIRTUAL_DISK_ACCESS_CREATE	= 0x100000,
        VIRTUAL_DISK_ACCESS_METAOPS	= 0x200000,
        VIRTUAL_DISK_ACCESS_READ	= 0xd0000,
        VIRTUAL_DISK_ACCESS_ALL	= 0x3f0000,
        VIRTUAL_DISK_ACCESS_WRITABLE	= 0x320000
    } 	VIRTUAL_DISK_ACCESS_MASK;

typedef 
enum _OPEN_VIRTUAL_DISK_FLAG
    {
        OPEN_VIRTUAL_DISK_FLAG_NONE	= 0,
        OPEN_VIRTUAL_DISK_FLAG_NO_PARENTS	= 0x1,
        OPEN_VIRTUAL_DISK_FLAG_BLANK_FILE	= 0x2,
        OPEN_VIRTUAL_DISK_FLAG_BOOT_DRIVE	= 0x4,
        OPEN_VIRTUAL_DISK_FLAG_CACHED_IO	= 0x8,
        OPEN_VIRTUAL_DISK_FLAG_CUSTOM_DIFF_CHAIN	= 0x10,
        OPEN_VIRTUAL_DISK_FLAG_PARENT_CACHED_IO	= 0x20,
        OPEN_VIRTUAL_DISK_FLAG_VHDSET_FILE_ONLY	= 0x40,
        OPEN_VIRTUAL_DISK_FLAG_IGNORE_RELATIVE_PARENT_LOCATOR	= 0x80,
        OPEN_VIRTUAL_DISK_FLAG_NO_WRITE_HARDENING	= 0x100,
        OPEN_VIRTUAL_DISK_FLAG_SUPPORT_COMPRESSED_VOLUMES	= 0x200,
        OPEN_VIRTUAL_DISK_FLAG_SUPPORT_SPARSE_FILES_ANY_FS	= 0x400,
        OPEN_VIRTUAL_DISK_FLAG_SUPPORT_ENCRYPTED_FILES	= 0x800
    } 	OPEN_VIRTUAL_DISK_FLAG;

DWORD __stdcall OpenVirtualDisk( 
    PVIRTUAL_STORAGE_TYPE VirtualStorageType,
    PCWSTR Path,
    VIRTUAL_DISK_ACCESS_MASK VirtualDiskAccessMask,
    OPEN_VIRTUAL_DISK_FLAG Flags,
    POPEN_VIRTUAL_DISK_PARAMETERS Parameters,
    PHANDLE Handle);

typedef 
enum _CREATE_VIRTUAL_DISK_VERSION
    {
        CREATE_VIRTUAL_DISK_VERSION_UNSPECIFIED	= 0,
        CREATE_VIRTUAL_DISK_VERSION_1	= 1,
        CREATE_VIRTUAL_DISK_VERSION_2	= 2,
        CREATE_VIRTUAL_DISK_VERSION_3	= 3,
        CREATE_VIRTUAL_DISK_VERSION_4	= 4
    } 	CREATE_VIRTUAL_DISK_VERSION;

typedef struct _CREATE_VIRTUAL_DISK_PARAMETERS
    {
    CREATE_VIRTUAL_DISK_VERSION Version;
    union 
        {
        struct 
            {
            GUID UniqueId;
            ULONGLONG MaximumSize;
            ULONG BlockSizeInBytes;
            ULONG SectorSizeInBytes;
            PCWSTR ParentPath;
            PCWSTR SourcePath;
            } 	Version1;
        struct 
            {
            GUID UniqueId;
            ULONGLONG MaximumSize;
            ULONG BlockSizeInBytes;
            ULONG SectorSizeInBytes;
            ULONG PhysicalSectorSizeInBytes;
            PCWSTR ParentPath;
            PCWSTR SourcePath;
            OPEN_VIRTUAL_DISK_FLAG OpenFlags;
            VIRTUAL_STORAGE_TYPE ParentVirtualStorageType;
            VIRTUAL_STORAGE_TYPE SourceVirtualStorageType;
            GUID ResiliencyGuid;
            } 	Version2;
        struct 
            {
            GUID UniqueId;
            ULONGLONG MaximumSize;
            ULONG BlockSizeInBytes;
            ULONG SectorSizeInBytes;
            ULONG PhysicalSectorSizeInBytes;
            PCWSTR ParentPath;
            PCWSTR SourcePath;
            OPEN_VIRTUAL_DISK_FLAG OpenFlags;
            VIRTUAL_STORAGE_TYPE ParentVirtualStorageType;
            VIRTUAL_STORAGE_TYPE SourceVirtualStorageType;
            GUID ResiliencyGuid;
            PCWSTR SourceLimitPath;
            VIRTUAL_STORAGE_TYPE BackingStorageType;
            } 	Version3;
        struct 
            {
            GUID UniqueId;
            ULONGLONG MaximumSize;
            ULONG BlockSizeInBytes;
            ULONG SectorSizeInBytes;
            ULONG PhysicalSectorSizeInBytes;
            PCWSTR ParentPath;
            PCWSTR SourcePath;
            OPEN_VIRTUAL_DISK_FLAG OpenFlags;
            VIRTUAL_STORAGE_TYPE ParentVirtualStorageType;
            VIRTUAL_STORAGE_TYPE SourceVirtualStorageType;
            GUID ResiliencyGuid;
            PCWSTR SourceLimitPath;
            VIRTUAL_STORAGE_TYPE BackingStorageType;
            GUID PmemAddressAbstractionType;
            ULONGLONG DataAlignment;
            } 	Version4;
        } 	;
    } 	CREATE_VIRTUAL_DISK_PARAMETERS;

typedef struct _CREATE_VIRTUAL_DISK_PARAMETERS *PCREATE_VIRTUAL_DISK_PARAMETERS;

typedef 
enum _CREATE_VIRTUAL_DISK_FLAG
    {
        CREATE_VIRTUAL_DISK_FLAG_NONE	= 0,
        CREATE_VIRTUAL_DISK_FLAG_FULL_PHYSICAL_ALLOCATION	= 0x1,
        CREATE_VIRTUAL_DISK_FLAG_PREVENT_WRITES_TO_SOURCE_DISK	= 0x2,
        CREATE_VIRTUAL_DISK_FLAG_DO_NOT_COPY_METADATA_FROM_PARENT	= 0x4,
        CREATE_VIRTUAL_DISK_FLAG_CREATE_BACKING_STORAGE	= 0x8,
        CREATE_VIRTUAL_DISK_FLAG_USE_CHANGE_TRACKING_SOURCE_LIMIT	= 0x10,
        CREATE_VIRTUAL_DISK_FLAG_PRESERVE_PARENT_CHANGE_TRACKING_STATE	= 0x20,
        CREATE_VIRTUAL_DISK_FLAG_VHD_SET_USE_ORIGINAL_BACKING_STORAGE	= 0x40,
        CREATE_VIRTUAL_DISK_FLAG_SPARSE_FILE	= 0x80,
        CREATE_VIRTUAL_DISK_FLAG_PMEM_COMPATIBLE	= 0x100,
        CREATE_VIRTUAL_DISK_FLAG_SUPPORT_COMPRESSED_VOLUMES	= 0x200,
        CREATE_VIRTUAL_DISK_FLAG_SUPPORT_SPARSE_FILES_ANY_FS	= 0x400
    } 	CREATE_VIRTUAL_DISK_FLAG;

DWORD __stdcall CreateVirtualDisk( 
    PVIRTUAL_STORAGE_TYPE VirtualStorageType,
    PCWSTR Path,
    VIRTUAL_DISK_ACCESS_MASK VirtualDiskAccessMask,
    PSECURITY_DESCRIPTOR SecurityDescriptor,
    CREATE_VIRTUAL_DISK_FLAG Flags,
    ULONG ProviderSpecificFlags,
    PCREATE_VIRTUAL_DISK_PARAMETERS Parameters,
    LPOVERLAPPED Overlapped,
    PHANDLE Handle);

typedef 
enum _ATTACH_VIRTUAL_DISK_VERSION
    {
        ATTACH_VIRTUAL_DISK_VERSION_UNSPECIFIED	= 0,
        ATTACH_VIRTUAL_DISK_VERSION_1	= 1,
        ATTACH_VIRTUAL_DISK_VERSION_2	= 2
    } 	ATTACH_VIRTUAL_DISK_VERSION;

typedef struct _ATTACH_VIRTUAL_DISK_PARAMETERS
    {
    ATTACH_VIRTUAL_DISK_VERSION Version;
    union 
        {
        struct 
            {
            ULONG Reserved;
            } 	Version1;
        struct 
            {
            ULONGLONG RestrictedOffset;
            ULONGLONG RestrictedLength;
            } 	Version2;
        } 	;
    } 	ATTACH_VIRTUAL_DISK_PARAMETERS;

typedef struct _ATTACH_VIRTUAL_DISK_PARAMETERS *PATTACH_VIRTUAL_DISK_PARAMETERS;

typedef 
enum _ATTACH_VIRTUAL_DISK_FLAG
    {
        ATTACH_VIRTUAL_DISK_FLAG_NONE	= 0,
        ATTACH_VIRTUAL_DISK_FLAG_READ_ONLY	= 0x1,
        ATTACH_VIRTUAL_DISK_FLAG_NO_DRIVE_LETTER	= 0x2,
        ATTACH_VIRTUAL_DISK_FLAG_PERMANENT_LIFETIME	= 0x4,
        ATTACH_VIRTUAL_DISK_FLAG_NO_LOCAL_HOST	= 0x8,
        ATTACH_VIRTUAL_DISK_FLAG_NO_SECURITY_DESCRIPTOR	= 0x10,
        ATTACH_VIRTUAL_DISK_FLAG_BYPASS_DEFAULT_ENCRYPTION_POLICY	= 0x20,
        ATTACH_VIRTUAL_DISK_FLAG_NON_PNP	= 0x40,
        ATTACH_VIRTUAL_DISK_FLAG_RESTRICTED_RANGE	= 0x80,
        ATTACH_VIRTUAL_DISK_FLAG_SINGLE_PARTITION	= 0x100,
        ATTACH_VIRTUAL_DISK_FLAG_REGISTER_VOLUME	= 0x200,
        ATTACH_VIRTUAL_DISK_FLAG_AT_BOOT	= 0x400
    } 	ATTACH_VIRTUAL_DISK_FLAG;

DWORD __stdcall AttachVirtualDisk( 
    HANDLE VirtualDiskHandle,
    PSECURITY_DESCRIPTOR SecurityDescriptor,
    ATTACH_VIRTUAL_DISK_FLAG Flags,
    ULONG ProviderSpecificFlags,
    PATTACH_VIRTUAL_DISK_PARAMETERS Parameters,
    LPOVERLAPPED Overlapped);

typedef 
enum _DETACH_VIRTUAL_DISK_FLAG
    {
        DETACH_VIRTUAL_DISK_FLAG_NONE	= 0
    } 	DETACH_VIRTUAL_DISK_FLAG;

DWORD __stdcall DetachVirtualDisk( 
    HANDLE VirtualDiskHandle,
    DETACH_VIRTUAL_DISK_FLAG Flags,
    ULONG ProviderSpecificFlags);

DWORD __stdcall GetVirtualDiskPhysicalPath( 
    HANDLE VirtualDiskHandle,
    PULONG DiskPathSizeInBytes,
    PWSTR DiskPath);

DWORD __stdcall GetAllAttachedVirtualDiskPhysicalPaths( 
    PULONG PathsBufferSizeInBytes,
    PWSTR PathsBuffer);

typedef 
enum _DEPENDENT_DISK_FLAG
    {
        DEPENDENT_DISK_FLAG_NONE	= 0,
        DEPENDENT_DISK_FLAG_MULT_BACKING_FILES	= 0x1,
        DEPENDENT_DISK_FLAG_FULLY_ALLOCATED	= 0x2,
        DEPENDENT_DISK_FLAG_READ_ONLY	= 0x4,
        DEPENDENT_DISK_FLAG_REMOTE	= 0x8,
        DEPENDENT_DISK_FLAG_SYSTEM_VOLUME	= 0x10,
        DEPENDENT_DISK_FLAG_SYSTEM_VOLUME_PARENT	= 0x20,
        DEPENDENT_DISK_FLAG_REMOVABLE	= 0x40,
        DEPENDENT_DISK_FLAG_NO_DRIVE_LETTER	= 0x80,
        DEPENDENT_DISK_FLAG_PARENT	= 0x100,
        DEPENDENT_DISK_FLAG_NO_HOST_DISK	= 0x200,
        DEPENDENT_DISK_FLAG_PERMANENT_LIFETIME	= 0x400,
        DEPENDENT_DISK_FLAG_SUPPORT_COMPRESSED_VOLUMES	= 0x800,
        DEPENDENT_DISK_FLAG_ALWAYS_ALLOW_SPARSE	= 0x1000,
        DEPENDENT_DISK_FLAG_SUPPORT_ENCRYPTED_FILES	= 0x2000
    } 	DEPENDENT_DISK_FLAG;

typedef 
enum _STORAGE_DEPENDENCY_INFO_VERSION
    {
        STORAGE_DEPENDENCY_INFO_VERSION_UNSPECIFIED	= 0,
        STORAGE_DEPENDENCY_INFO_VERSION_1	= 1,
        STORAGE_DEPENDENCY_INFO_VERSION_2	= 2
    } 	STORAGE_DEPENDENCY_INFO_VERSION;

typedef struct _STORAGE_DEPENDENCY_INFO_TYPE_1
    {
    DEPENDENT_DISK_FLAG DependencyTypeFlags;
    ULONG ProviderSpecificFlags;
    VIRTUAL_STORAGE_TYPE VirtualStorageType;
    } 	STORAGE_DEPENDENCY_INFO_TYPE_1;

typedef struct _STORAGE_DEPENDENCY_INFO_TYPE_1 *PSTORAGE_DEPENDENCY_INFO_TYPE_1;

typedef struct _STORAGE_DEPENDENCY_INFO_TYPE_2
    {
    DEPENDENT_DISK_FLAG DependencyTypeFlags;
    ULONG ProviderSpecificFlags;
    VIRTUAL_STORAGE_TYPE VirtualStorageType;
    ULONG AncestorLevel;
    PWSTR DependencyDeviceName;
    PWSTR HostVolumeName;
    PWSTR DependentVolumeName;
    PWSTR DependentVolumeRelativePath;
    } 	STORAGE_DEPENDENCY_INFO_TYPE_2;

typedef struct _STORAGE_DEPENDENCY_INFO_TYPE_2 *PSTORAGE_DEPENDENCY_INFO_TYPE_2;

typedef struct _STORAGE_DEPENDENCY_INFO
    {
    STORAGE_DEPENDENCY_INFO_VERSION Version;
    ULONG NumberEntries;
    union 
        {
        STORAGE_DEPENDENCY_INFO_TYPE_1 Version1Entries[ 1 ];
        STORAGE_DEPENDENCY_INFO_TYPE_2 Version2Entries[ 1 ];
        } 	;
    } 	STORAGE_DEPENDENCY_INFO;

typedef struct _STORAGE_DEPENDENCY_INFO *PSTORAGE_DEPENDENCY_INFO;

typedef 
enum _GET_STORAGE_DEPENDENCY_FLAG
    {
        GET_STORAGE_DEPENDENCY_FLAG_NONE	= 0,
        GET_STORAGE_DEPENDENCY_FLAG_HOST_VOLUMES	= 0x1,
        GET_STORAGE_DEPENDENCY_FLAG_DISK_HANDLE	= 0x2
    } 	GET_STORAGE_DEPENDENCY_FLAG;

DWORD __stdcall GetStorageDependencyInformation( 
    HANDLE ObjectHandle,
    GET_STORAGE_DEPENDENCY_FLAG Flags,
    ULONG StorageDependencyInfoSize,
    PSTORAGE_DEPENDENCY_INFO StorageDependencyInfo,
    PULONG SizeUsed);

typedef 
enum _GET_VIRTUAL_DISK_INFO_VERSION
    {
        GET_VIRTUAL_DISK_INFO_UNSPECIFIED	= 0,
        GET_VIRTUAL_DISK_INFO_SIZE	= 1,
        GET_VIRTUAL_DISK_INFO_IDENTIFIER	= 2,
        GET_VIRTUAL_DISK_INFO_PARENT_LOCATION	= 3,
        GET_VIRTUAL_DISK_INFO_PARENT_IDENTIFIER	= 4,
        GET_VIRTUAL_DISK_INFO_PARENT_TIMESTAMP	= 5,
        GET_VIRTUAL_DISK_INFO_VIRTUAL_STORAGE_TYPE	= 6,
        GET_VIRTUAL_DISK_INFO_PROVIDER_SUBTYPE	= 7,
        GET_VIRTUAL_DISK_INFO_IS_4K_ALIGNED	= 8,
        GET_VIRTUAL_DISK_INFO_PHYSICAL_DISK	= 9,
        GET_VIRTUAL_DISK_INFO_VHD_PHYSICAL_SECTOR_SIZE	= 10,
        GET_VIRTUAL_DISK_INFO_SMALLEST_SAFE_VIRTUAL_SIZE	= 11,
        GET_VIRTUAL_DISK_INFO_FRAGMENTATION	= 12,
        GET_VIRTUAL_DISK_INFO_IS_LOADED	= 13,
        GET_VIRTUAL_DISK_INFO_VIRTUAL_DISK_ID	= 14,
        GET_VIRTUAL_DISK_INFO_CHANGE_TRACKING_STATE	= 15
    } 	GET_VIRTUAL_DISK_INFO_VERSION;

typedef struct _GET_VIRTUAL_DISK_INFO
    {
    GET_VIRTUAL_DISK_INFO_VERSION Version;
    union 
        {
        struct 
            {
            ULONGLONG VirtualSize;
            ULONGLONG PhysicalSize;
            ULONG BlockSize;
            ULONG SectorSize;
            } 	Size;
        GUID Identifier;
        struct 
            {
            BOOL ParentResolved;
            WCHAR ParentLocationBuffer[ 1 ];
            } 	ParentLocation;
        GUID ParentIdentifier;
        ULONG ParentTimestamp;
        VIRTUAL_STORAGE_TYPE VirtualStorageType;
        ULONG ProviderSubtype;
        BOOL Is4kAligned;
        BOOL IsLoaded;
        struct 
            {
            ULONG LogicalSectorSize;
            ULONG PhysicalSectorSize;
            BOOL IsRemote;
            } 	PhysicalDisk;
        ULONG VhdPhysicalSectorSize;
        ULONGLONG SmallestSafeVirtualSize;
        ULONG FragmentationPercentage;
        GUID VirtualDiskId;
        struct 
            {
            BOOL Enabled;
            BOOL NewerChanges;
            WCHAR MostRecentId[ 1 ];
            } 	ChangeTrackingState;
        } 	;
    } 	GET_VIRTUAL_DISK_INFO;

typedef struct _GET_VIRTUAL_DISK_INFO *PGET_VIRTUAL_DISK_INFO;

DWORD __stdcall GetVirtualDiskInformation( 
    HANDLE VirtualDiskHandle,
    PULONG VirtualDiskInfoSize,
    PGET_VIRTUAL_DISK_INFO VirtualDiskInfo,
    PULONG SizeUsed);

typedef 
enum _SET_VIRTUAL_DISK_INFO_VERSION
    {
        SET_VIRTUAL_DISK_INFO_UNSPECIFIED	= 0,
        SET_VIRTUAL_DISK_INFO_PARENT_PATH	= 1,
        SET_VIRTUAL_DISK_INFO_IDENTIFIER	= 2,
        SET_VIRTUAL_DISK_INFO_PARENT_PATH_WITH_DEPTH	= 3,
        SET_VIRTUAL_DISK_INFO_PHYSICAL_SECTOR_SIZE	= 4,
        SET_VIRTUAL_DISK_INFO_VIRTUAL_DISK_ID	= 5,
        SET_VIRTUAL_DISK_INFO_CHANGE_TRACKING_STATE	= 6,
        SET_VIRTUAL_DISK_INFO_PARENT_LOCATOR	= 7
    } 	SET_VIRTUAL_DISK_INFO_VERSION;

typedef struct _SET_VIRTUAL_DISK_INFO
    {
    SET_VIRTUAL_DISK_INFO_VERSION Version;
    union 
        {
        PCWSTR ParentFilePath;
        GUID UniqueIdentifier;
        struct 
            {
            ULONG ChildDepth;
            PCWSTR ParentFilePath;
            } 	ParentPathWithDepthInfo;
        ULONG VhdPhysicalSectorSize;
        GUID VirtualDiskId;
        BOOL ChangeTrackingEnabled;
        struct 
            {
            GUID LinkageId;
            PCWSTR ParentFilePath;
            } 	ParentLocator;
        } 	;
    } 	SET_VIRTUAL_DISK_INFO;

typedef struct _SET_VIRTUAL_DISK_INFO *PSET_VIRTUAL_DISK_INFO;

DWORD __stdcall SetVirtualDiskInformation( 
    HANDLE VirtualDiskHandle,
    PSET_VIRTUAL_DISK_INFO VirtualDiskInfo);

DWORD __stdcall EnumerateVirtualDiskMetadata( 
    HANDLE VirtualDiskHandle,
    PULONG NumberOfItems,
    GUID *Items);

DWORD __stdcall GetVirtualDiskMetadata( 
    HANDLE VirtualDiskHandle,
    const GUID *Item,
    PULONG MetaDataSize,
    PVOID MetaData);

DWORD __stdcall SetVirtualDiskMetadata( 
    HANDLE VirtualDiskHandle,
    const GUID *Item,
    ULONG MetaDataSize,
    const void *MetaData);

DWORD __stdcall DeleteVirtualDiskMetadata( 
    HANDLE VirtualDiskHandle,
    const GUID *Item);

typedef struct _VIRTUAL_DISK_PROGRESS
    {
    DWORD OperationStatus;
    ULONGLONG CurrentValue;
    ULONGLONG CompletionValue;
    } 	VIRTUAL_DISK_PROGRESS;

typedef struct _VIRTUAL_DISK_PROGRESS *PVIRTUAL_DISK_PROGRESS;

DWORD __stdcall GetVirtualDiskOperationProgress( 
    HANDLE VirtualDiskHandle,
    LPOVERLAPPED Overlapped,
    PVIRTUAL_DISK_PROGRESS Progress);

typedef 
enum _COMPACT_VIRTUAL_DISK_VERSION
    {
        COMPACT_VIRTUAL_DISK_VERSION_UNSPECIFIED	= 0,
        COMPACT_VIRTUAL_DISK_VERSION_1	= 1
    } 	COMPACT_VIRTUAL_DISK_VERSION;

typedef struct _COMPACT_VIRTUAL_DISK_PARAMETERS
    {
    COMPACT_VIRTUAL_DISK_VERSION Version;
    union 
        {
        struct 
            {
            ULONG Reserved;
            } 	Version1;
        } 	;
    } 	COMPACT_VIRTUAL_DISK_PARAMETERS;

typedef struct _COMPACT_VIRTUAL_DISK_PARAMETERS *PCOMPACT_VIRTUAL_DISK_PARAMETERS;

typedef 
enum _COMPACT_VIRTUAL_DISK_FLAG
    {
        COMPACT_VIRTUAL_DISK_FLAG_NONE	= 0,
        COMPACT_VIRTUAL_DISK_FLAG_NO_ZERO_SCAN	= 0x1,
        COMPACT_VIRTUAL_DISK_FLAG_NO_BLOCK_MOVES	= 0x2
    } 	COMPACT_VIRTUAL_DISK_FLAG;

DWORD __stdcall CompactVirtualDisk( 
    HANDLE VirtualDiskHandle,
    COMPACT_VIRTUAL_DISK_FLAG Flags,
    PCOMPACT_VIRTUAL_DISK_PARAMETERS Parameters,
    LPOVERLAPPED Overlapped);

typedef 
enum _MERGE_VIRTUAL_DISK_VERSION
    {
        MERGE_VIRTUAL_DISK_VERSION_UNSPECIFIED	= 0,
        MERGE_VIRTUAL_DISK_VERSION_1	= 1,
        MERGE_VIRTUAL_DISK_VERSION_2	= 2
    } 	MERGE_VIRTUAL_DISK_VERSION;

typedef struct _MERGE_VIRTUAL_DISK_PARAMETERS
    {
    MERGE_VIRTUAL_DISK_VERSION Version;
    union 
        {
        struct 
            {
            ULONG MergeDepth;
            } 	Version1;
        struct 
            {
            ULONG MergeSourceDepth;
            ULONG MergeTargetDepth;
            } 	Version2;
        } 	;
    } 	MERGE_VIRTUAL_DISK_PARAMETERS;

typedef struct _MERGE_VIRTUAL_DISK_PARAMETERS *PMERGE_VIRTUAL_DISK_PARAMETERS;

typedef 
enum _MERGE_VIRTUAL_DISK_FLAG
    {
        MERGE_VIRTUAL_DISK_FLAG_NONE	= 0
    } 	MERGE_VIRTUAL_DISK_FLAG;

DWORD __stdcall MergeVirtualDisk( 
    HANDLE VirtualDiskHandle,
    MERGE_VIRTUAL_DISK_FLAG Flags,
    PMERGE_VIRTUAL_DISK_PARAMETERS Parameters,
    LPOVERLAPPED Overlapped);

typedef 
enum _EXPAND_VIRTUAL_DISK_VERSION
    {
        EXPAND_VIRTUAL_DISK_VERSION_UNSPECIFIED	= 0,
        EXPAND_VIRTUAL_DISK_VERSION_1	= 1
    } 	EXPAND_VIRTUAL_DISK_VERSION;

typedef struct _EXPAND_VIRTUAL_DISK_PARAMETERS
    {
    EXPAND_VIRTUAL_DISK_VERSION Version;
    union 
        {
        struct 
            {
            ULONGLONG NewSize;
            } 	Version1;
        } 	;
    } 	EXPAND_VIRTUAL_DISK_PARAMETERS;

typedef struct _EXPAND_VIRTUAL_DISK_PARAMETERS *PEXPAND_VIRTUAL_DISK_PARAMETERS;

typedef 
enum _EXPAND_VIRTUAL_DISK_FLAG
    {
        EXPAND_VIRTUAL_DISK_FLAG_NONE	= 0,
        EXPAND_VIRTUAL_DISK_FLAG_NOTIFY_CHANGE	= 0x1
    } 	EXPAND_VIRTUAL_DISK_FLAG;

DWORD __stdcall ExpandVirtualDisk( 
    HANDLE VirtualDiskHandle,
    EXPAND_VIRTUAL_DISK_FLAG Flags,
    PEXPAND_VIRTUAL_DISK_PARAMETERS Parameters,
    LPOVERLAPPED Overlapped);

typedef 
enum _RESIZE_VIRTUAL_DISK_VERSION
    {
        RESIZE_VIRTUAL_DISK_VERSION_UNSPECIFIED	= 0,
        RESIZE_VIRTUAL_DISK_VERSION_1	= 1
    } 	RESIZE_VIRTUAL_DISK_VERSION;

typedef struct _RESIZE_VIRTUAL_DISK_PARAMETERS
    {
    RESIZE_VIRTUAL_DISK_VERSION Version;
    union 
        {
        struct 
            {
            ULONGLONG NewSize;
            } 	Version1;
        } 	;
    } 	RESIZE_VIRTUAL_DISK_PARAMETERS;

typedef struct _RESIZE_VIRTUAL_DISK_PARAMETERS *PRESIZE_VIRTUAL_DISK_PARAMETERS;

typedef 
enum _RESIZE_VIRTUAL_DISK_FLAG
    {
        RESIZE_VIRTUAL_DISK_FLAG_NONE	= 0,
        RESIZE_VIRTUAL_DISK_FLAG_ALLOW_UNSAFE_VIRTUAL_SIZE	= 0x1,
        RESIZE_VIRTUAL_DISK_FLAG_RESIZE_TO_SMALLEST_SAFE_VIRTUAL_SIZE	= 0x2
    } 	RESIZE_VIRTUAL_DISK_FLAG;

DWORD __stdcall ResizeVirtualDisk( 
    HANDLE VirtualDiskHandle,
    RESIZE_VIRTUAL_DISK_FLAG Flags,
    PRESIZE_VIRTUAL_DISK_PARAMETERS Parameters,
    LPOVERLAPPED Overlapped);

typedef 
enum _MIRROR_VIRTUAL_DISK_VERSION
    {
        MIRROR_VIRTUAL_DISK_VERSION_UNSPECIFIED	= 0,
        MIRROR_VIRTUAL_DISK_VERSION_1	= 1
    } 	MIRROR_VIRTUAL_DISK_VERSION;

typedef struct _MIRROR_VIRTUAL_DISK_PARAMETERS
    {
    MIRROR_VIRTUAL_DISK_VERSION Version;
    union 
        {
        struct 
            {
            PCWSTR MirrorVirtualDiskPath;
            } 	Version1;
        } 	;
    } 	MIRROR_VIRTUAL_DISK_PARAMETERS;

typedef struct _MIRROR_VIRTUAL_DISK_PARAMETERS *PMIRROR_VIRTUAL_DISK_PARAMETERS;

typedef 
enum _MIRROR_VIRTUAL_DISK_FLAG
    {
        MIRROR_VIRTUAL_DISK_FLAG_NONE	= 0,
        MIRROR_VIRTUAL_DISK_FLAG_EXISTING_FILE	= 0x1,
        MIRROR_VIRTUAL_DISK_FLAG_SKIP_MIRROR_ACTIVATION	= 0x2,
        MIRROR_VIRTUAL_DISK_FLAG_ENABLE_SMB_COMPRESSION	= 0x4,
        MIRROR_VIRTUAL_DISK_FLAG_IS_LIVE_MIGRATION	= 0x8
    } 	MIRROR_VIRTUAL_DISK_FLAG;

DWORD __stdcall MirrorVirtualDisk( 
    HANDLE VirtualDiskHandle,
    MIRROR_VIRTUAL_DISK_FLAG Flags,
    PMIRROR_VIRTUAL_DISK_PARAMETERS Parameters,
    LPOVERLAPPED Overlapped);

DWORD __stdcall BreakMirrorVirtualDisk( 
    HANDLE VirtualDiskHandle);

DWORD __stdcall AddVirtualDiskParent( 
    HANDLE VirtualDiskHandle,
    PCWSTR ParentPath);

typedef struct _QUERY_CHANGES_VIRTUAL_DISK_RANGE
    {
    ULONG64 ByteOffset;
    ULONG64 ByteLength;
    ULONG64 Reserved;
    } 	QUERY_CHANGES_VIRTUAL_DISK_RANGE;

typedef struct _QUERY_CHANGES_VIRTUAL_DISK_RANGE *PQUERY_CHANGES_VIRTUAL_DISK_RANGE;

typedef 
enum _QUERY_CHANGES_VIRTUAL_DISK_FLAG
    {
        QUERY_CHANGES_VIRTUAL_DISK_FLAG_NONE	= 0
    } 	QUERY_CHANGES_VIRTUAL_DISK_FLAG;

typedef 
enum _TAKE_SNAPSHOT_VHDSET_FLAG
    {
        TAKE_SNAPSHOT_VHDSET_FLAG_NONE	= 0,
        TAKE_SNAPSHOT_VHDSET_FLAG_WRITEABLE	= 0x1
    } 	TAKE_SNAPSHOT_VHDSET_FLAG;

typedef enum _TAKE_SNAPSHOT_VHDSET_FLAG *PTAKE_SNAPSHOT_VHDSET_FLAG;

typedef 
enum _TAKE_SNAPSHOT_VHDSET_VERSION
    {
        TAKE_SNAPSHOT_VHDSET_VERSION_UNSPECIFIED	= 0,
        TAKE_SNAPSHOT_VHDSET_VERSION_1	= 1
    } 	TAKE_SNAPSHOT_VHDSET_VERSION;

typedef struct _TAKE_SNAPSHOT_VHDSET_PARAMETERS
    {
    TAKE_SNAPSHOT_VHDSET_VERSION Version;
    union 
        {
        struct 
            {
            GUID SnapshotId;
            } 	Version1;
        } 	;
    } 	TAKE_SNAPSHOT_VHDSET_PARAMETERS;

typedef struct _TAKE_SNAPSHOT_VHDSET_PARAMETERS *PTAKE_SNAPSHOT_VHDSET_PARAMETERS;

typedef 
enum _DELETE_SNAPSHOT_VHDSET_FLAG
    {
        DELETE_SNAPSHOT_VHDSET_FLAG_NONE	= 0,
        DELETE_SNAPSHOT_VHDSET_FLAG_PERSIST_RCT	= 0x1
    } 	DELETE_SNAPSHOT_VHDSET_FLAG;

typedef enum _DELETE_SNAPSHOT_VHDSET_FLAG *PDELETE_SNAPSHOT_VHDSET_FLAG;

typedef 
enum _DELETE_SNAPSHOT_VHDSET_VERSION
    {
        DELETE_SNAPSHOT_VHDSET_VERSION_UNSPECIFIED	= 0,
        DELETE_SNAPSHOT_VHDSET_VERSION_1	= 1
    } 	DELETE_SNAPSHOT_VHDSET_VERSION;

typedef struct _DELETE_SNAPSHOT_VHDSET_PARAMETERS
    {
    DELETE_SNAPSHOT_VHDSET_VERSION Version;
    union 
        {
        struct 
            {
            GUID SnapshotId;
            } 	Version1;
        } 	;
    } 	DELETE_SNAPSHOT_VHDSET_PARAMETERS;

typedef struct _DELETE_SNAPSHOT_VHDSET_PARAMETERS *PDELETE_SNAPSHOT_VHDSET_PARAMETERS;

typedef 
enum _MODIFY_VHDSET_VERSION
    {
        MODIFY_VHDSET_UNSPECIFIED	= 0,
        MODIFY_VHDSET_SNAPSHOT_PATH	= 1,
        MODIFY_VHDSET_REMOVE_SNAPSHOT	= 2,
        MODIFY_VHDSET_DEFAULT_SNAPSHOT_PATH	= 3
    } 	MODIFY_VHDSET_VERSION;

typedef enum _MODIFY_VHDSET_VERSION *PMODIFY_VHDSET_VERSION;

typedef 
enum _MODIFY_VHDSET_FLAG
    {
        MODIFY_VHDSET_FLAG_NONE	= 0,
        MODIFY_VHDSET_FLAG_WRITEABLE_SNAPSHOT	= 0x1
    } 	MODIFY_VHDSET_FLAG;

typedef enum _MODIFY_VHDSET_FLAG *PMODIFY_VHDSET_FLAG;

typedef struct _MODIFY_VHDSET_PARAMETERS
    {
    MODIFY_VHDSET_VERSION Version;
    union 
        {
        struct 
            {
            GUID SnapshotId;
            PCWSTR SnapshotFilePath;
            } 	SnapshotPath;
        GUID SnapshotId;
        PCWSTR DefaultFilePath;
        } 	;
    } 	MODIFY_VHDSET_PARAMETERS;

typedef struct _MODIFY_VHDSET_PARAMETERS *PMODIFY_VHDSET_PARAMETERS;

typedef 
enum _APPLY_SNAPSHOT_VHDSET_FLAG
    {
        APPLY_SNAPSHOT_VHDSET_FLAG_NONE	= 0,
        APPLY_SNAPSHOT_VHDSET_FLAG_WRITEABLE	= 0x1
    } 	APPLY_SNAPSHOT_VHDSET_FLAG;

typedef enum _APPLY_SNAPSHOT_VHDSET_FLAG *PAPPLY_SNAPSHOT_VHDSET_FLAG;

typedef 
enum _APPLY_SNAPSHOT_VHDSET_VERSION
    {
        APPLY_SNAPSHOT_VHDSET_VERSION_UNSPECIFIED	= 0,
        APPLY_SNAPSHOT_VHDSET_VERSION_1	= 1
    } 	APPLY_SNAPSHOT_VHDSET_VERSION;

typedef struct _APPLY_SNAPSHOT_VHDSET_PARAMETERS
    {
    APPLY_SNAPSHOT_VHDSET_VERSION Version;
    union 
        {
        struct 
            {
            GUID SnapshotId;
            GUID LeafSnapshotId;
            } 	Version1;
        } 	;
    } 	APPLY_SNAPSHOT_VHDSET_PARAMETERS;

typedef struct _APPLY_SNAPSHOT_VHDSET_PARAMETERS *PAPPLY_SNAPSHOT_VHDSET_PARAMETERS;

typedef 
enum _RAW_SCSI_VIRTUAL_DISK_FLAG
    {
        RAW_SCSI_VIRTUAL_DISK_FLAG_NONE	= 0
    } 	RAW_SCSI_VIRTUAL_DISK_FLAG;

typedef enum _RAW_SCSI_VIRTUAL_DISK_FLAG *PRAW_SCSI_VIRTUAL_DISK_FLAG;

typedef 
enum _RAW_SCSI_VIRTUAL_DISK_VERSION
    {
        RAW_SCSI_VIRTUAL_DISK_VERSION_UNSPECIFIED	= 0,
        RAW_SCSI_VIRTUAL_DISK_VERSION_1	= 1
    } 	RAW_SCSI_VIRTUAL_DISK_VERSION;

typedef struct _RAW_SCSI_VIRTUAL_DISK_PARAMETERS
    {
    RAW_SCSI_VIRTUAL_DISK_VERSION Version;
    union 
        {
        struct 
            {
            BOOL RSVDHandle;
            UCHAR DataIn;
            UCHAR CdbLength;
            UCHAR SenseInfoLength;
            ULONG SrbFlags;
            ULONG DataTransferLength;
            PVOID DataBuffer;
            UCHAR *SenseInfo;
            UCHAR *Cdb;
            } 	Version1;
        } 	;
    } 	RAW_SCSI_VIRTUAL_DISK_PARAMETERS;

typedef struct _RAW_SCSI_VIRTUAL_DISK_PARAMETERS *PRAW_SCSI_VIRTUAL_DISK_PARAMETERS;

typedef struct _RAW_SCSI_VIRTUAL_DISK_RESPONSE
    {
    RAW_SCSI_VIRTUAL_DISK_VERSION Version;
    union 
        {
        struct 
            {
            UCHAR ScsiStatus;
            UCHAR SenseInfoLength;
            ULONG DataTransferLength;
            } 	Version1;
        } 	;
    } 	RAW_SCSI_VIRTUAL_DISK_RESPONSE;

typedef struct _RAW_SCSI_VIRTUAL_DISK_RESPONSE *PRAW_SCSI_VIRTUAL_DISK_RESPONSE;

DWORD __stdcall QueryChangesVirtualDisk( 
    HANDLE VirtualDiskHandle,
    PCWSTR ChangeTrackingId,
    ULONG64 ByteOffset,
    ULONG64 ByteLength,
    QUERY_CHANGES_VIRTUAL_DISK_FLAG Flags,
    PQUERY_CHANGES_VIRTUAL_DISK_RANGE Ranges,
    PULONG RangeCount,
    PULONG64 ProcessedLength);

DWORD __stdcall TakeSnapshotVhdSet( 
    HANDLE VirtualDiskHandle,
    const PTAKE_SNAPSHOT_VHDSET_PARAMETERS Parameters,
    TAKE_SNAPSHOT_VHDSET_FLAG Flags);

DWORD __stdcall DeleteSnapshotVhdSet( 
    HANDLE VirtualDiskHandle,
    const PDELETE_SNAPSHOT_VHDSET_PARAMETERS Parameters,
    DELETE_SNAPSHOT_VHDSET_FLAG Flags);

DWORD __stdcall ModifyVhdSet( 
    HANDLE VirtualDiskHandle,
    const PMODIFY_VHDSET_PARAMETERS Parameters,
    MODIFY_VHDSET_FLAG Flags);

DWORD __stdcall ApplySnapshotVhdSet( 
    HANDLE VirtualDiskHandle,
    const PAPPLY_SNAPSHOT_VHDSET_PARAMETERS Parameters,
    APPLY_SNAPSHOT_VHDSET_FLAG Flags);

DWORD __stdcall RawSCSIVirtualDisk( 
    HANDLE VirtualDiskHandle,
    const PRAW_SCSI_VIRTUAL_DISK_PARAMETERS Parameters,
    RAW_SCSI_VIRTUAL_DISK_FLAG Flags,
    PRAW_SCSI_VIRTUAL_DISK_RESPONSE Response);

typedef 
enum _FORK_VIRTUAL_DISK_VERSION
    {
        FORK_VIRTUAL_DISK_VERSION_UNSPECIFIED	= 0,
        FORK_VIRTUAL_DISK_VERSION_1	= 1
    } 	FORK_VIRTUAL_DISK_VERSION;

typedef struct _FORK_VIRTUAL_DISK_PARAMETERS
    {
    FORK_VIRTUAL_DISK_VERSION Version;
    union 
        {
        struct 
            {
            PCWSTR ForkedVirtualDiskPath;
            } 	Version1;
        } 	;
    } 	FORK_VIRTUAL_DISK_PARAMETERS;

typedef struct _FORK_VIRTUAL_DISK_PARAMETERS *PFORK_VIRTUAL_DISK_PARAMETERS;

typedef 
enum _FORK_VIRTUAL_DISK_FLAG
    {
        FORK_VIRTUAL_DISK_FLAG_NONE	= 0,
        FORK_VIRTUAL_DISK_FLAG_EXISTING_FILE	= 0x1
    } 	FORK_VIRTUAL_DISK_FLAG;

DWORD __stdcall ForkVirtualDisk( 
    HANDLE VirtualDiskHandle,
    FORK_VIRTUAL_DISK_FLAG Flags,
    const FORK_VIRTUAL_DISK_PARAMETERS *Parameters,
    LPOVERLAPPED Overlapped);

DWORD __stdcall CompleteForkVirtualDisk( 
    HANDLE VirtualDiskHandle);

typedef 
enum _SURFACE_VIRTUAL_DISK_FLAG
    {
        SURFACE_VIRTUAL_DISK_FLAG_NONE	= 0,
        SURFACE_VIRTUAL_DISK_FLAG_READ_ONLY	= 0x1,
        SURFACE_VIRTUAL_DISK_FLAG_NO_DRIVE_LETTER	= 0x2,
        SURFACE_VIRTUAL_DISK_FLAG_PERMANENT_LIFETIME	= 0x4,
        SURFACE_VIRTUAL_DISK_FLAG_NO_LOCAL_HOST	= 0x8,
        SURFACE_VIRTUAL_DISK_FLAG_NO_SECURITY_DESCRIPTOR	= 0x10
    } 	SURFACE_VIRTUAL_DISK_FLAG;

typedef 
enum _UNSURFACE_VIRTUAL_DISK_FLAG
    {
        UNSURFACE_VIRTUAL_DISK_FLAG_NONE	= 0
    } 	UNSURFACE_VIRTUAL_DISK_FLAG;

#pragma warning(pop)
#pragma endregion
#else //defined(VDS_MIDL_PASS)
#include "virtdisk.h"
#endif //defined(VDS_MIDL_PASS)
#endif // _WIN32_WINNT_WIN7
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_vdssys_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_vdssys_0000_0000_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif



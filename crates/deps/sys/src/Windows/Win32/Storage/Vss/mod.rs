#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Storage_Vss`*"]
    pub fn CreateVssExpressWriterInternal(ppwriter: *mut IVssExpressWriter) -> ::windows_sys::core::HRESULT;
}
pub struct IVssAdmin(i32);
pub struct IVssAdminEx(i32);
pub struct IVssAsync(i32);
pub struct IVssComponent(i32);
pub struct IVssComponentEx(i32);
pub struct IVssComponentEx2(i32);
pub struct IVssCreateExpressWriterMetadata(i32);
pub struct IVssCreateWriterMetadata(i32);
pub struct IVssDifferentialSoftwareSnapshotMgmt(i32);
pub struct IVssDifferentialSoftwareSnapshotMgmt2(i32);
pub struct IVssDifferentialSoftwareSnapshotMgmt3(i32);
pub struct IVssEnumMgmtObject(i32);
pub struct IVssEnumObject(i32);
pub struct IVssExamineWriterMetadata(i32);
pub struct IVssExpressWriter(i32);
pub struct IVssFileShareSnapshotProvider(i32);
pub struct IVssHardwareSnapshotProvider(i32);
pub struct IVssHardwareSnapshotProviderEx(i32);
pub struct IVssProviderCreateSnapshotSet(i32);
pub struct IVssProviderNotifications(i32);
pub struct IVssSnapshotMgmt(i32);
pub struct IVssSnapshotMgmt2(i32);
pub struct IVssSoftwareSnapshotProvider(i32);
pub struct IVssWMDependency(i32);
pub struct IVssWMFiledesc(i32);
pub struct IVssWriterComponents(i32);
pub struct IVssWriterImpl(i32);
pub struct VSSCoordinator(i32);
pub struct VSS_ALTERNATE_WRITER_STATE(i32);
pub struct VSS_APPLICATION_LEVEL(i32);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_ASSOC_NO_MAX_SPACE: i32 = -1i32;
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_ASSOC_REMOVE: u32 = 0u32;
pub struct VSS_BACKUP_SCHEMA(i32);
pub struct VSS_BACKUP_TYPE(i32);
pub struct VSS_COMPONENT_FLAGS(i32);
pub struct VSS_COMPONENT_TYPE(i32);
pub struct VSS_DIFF_AREA_PROP(i32);
pub struct VSS_DIFF_VOLUME_PROP(i32);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_ASRERROR_CRITICAL_DISKS_TOO_SMALL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212280i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_ASRERROR_CRITICAL_DISK_CANNOT_BE_EXCLUDED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212267i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_ASRERROR_DATADISK_RDISK0: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212282i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_ASRERROR_DISK_ASSIGNMENT_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212287i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_ASRERROR_DISK_RECREATION_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212286i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_ASRERROR_DYNAMIC_VHD_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212278i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_ASRERROR_FIXED_PHYSICAL_DISK_AVAILABLE_AFTER_DISK_EXCLUSION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212268i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_ASRERROR_MISSING_DYNDISK: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212284i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_ASRERROR_NO_ARCPATH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212285i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_ASRERROR_NO_PHYSICAL_DISK_AVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212269i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_ASRERROR_RDISK0_TOOSMALL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212281i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_ASRERROR_RDISK_FOR_SYSTEM_DISK_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212270i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_ASRERROR_SHARED_CRIDISK: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212283i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_ASRERROR_SYSTEM_PARTITION_HIDDEN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212266i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_AUTORECOVERY_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212293i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_BAD_STATE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212543i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_BREAK_REVERT_ID_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212298i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_CANNOT_REVERT_DISKID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212290i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_CLUSTER_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212288i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_CLUSTER_TIMEOUT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212498i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_CORRUPT_XML_DOCUMENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212528i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_CRITICAL_VOLUME_ON_INVALID_DISK: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212271i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_DYNAMIC_DISK_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212292i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_FLUSH_WRITES_TIMEOUT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212525i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_FSS_TIMEOUT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212265i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_HOLD_WRITES_TIMEOUT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212524i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_INSUFFICIENT_STORAGE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212513i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_INVALID_XML_DOCUMENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212527i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_LEGACY_PROVIDER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212297i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_MAXIMUM_DIFFAREA_ASSOCIATIONS_REACHED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212514i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_MAXIMUM_NUMBER_OF_REMOTE_MACHINES_REACHED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212510i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_MAXIMUM_NUMBER_OF_SNAPSHOTS_REACHED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212521i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_MAXIMUM_NUMBER_OF_VOLUMES_REACHED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212526i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_MISSING_DISK: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212296i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_MISSING_HIDDEN_VOLUME: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212295i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_MISSING_VOLUME: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212294i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_NESTED_VOLUME_LIMIT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212500i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_NONTRANSPORTABLE_BCD: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212291i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212497i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_NO_SNAPSHOTS_IMPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212512i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_OBJECT_ALREADY_EXISTS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212531i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_OBJECT_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212536i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_PROVIDER_ALREADY_REGISTERED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212541i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_PROVIDER_IN_USE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212537i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_PROVIDER_NOT_REGISTERED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212540i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_PROVIDER_VETO: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212538i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_REBOOT_REQUIRED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212505i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_REMOTE_SERVER_UNAVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212509i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_REMOTE_SERVER_UNSUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212508i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_RESYNC_IN_PROGRESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212289i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_REVERT_IN_PROGRESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212507i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_REVERT_VOLUME_LOST: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212506i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_SNAPSHOT_NOT_IN_SET: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212501i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_SNAPSHOT_SET_IN_PROGRESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212522i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_SOME_SNAPSHOTS_NOT_IMPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212511i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_TRANSACTION_FREEZE_TIMEOUT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212504i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_TRANSACTION_THAW_TIMEOUT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212503i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_UNEXPECTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212542i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_UNEXPECTED_PROVIDER_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212529i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_UNEXPECTED_WRITER_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212523i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_UNSELECTED_VOLUME: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212502i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_UNSUPPORTED_CONTEXT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212517i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_VOLUME_IN_USE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212515i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_VOLUME_NOT_LOCAL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212499i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_VOLUME_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212532i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_VOLUME_NOT_SUPPORTED_BY_PROVIDER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212530i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_WRITERERROR_INCONSISTENTSNAPSHOT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212304i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_WRITERERROR_NONRETRYABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212300i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_WRITERERROR_OUTOFRESOURCES: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212303i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_WRITERERROR_PARTIAL_FAILURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212490i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_WRITERERROR_RECOVERY_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212299i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_WRITERERROR_RETRYABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212301i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_WRITERERROR_TIMEOUT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212302i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_WRITER_ALREADY_SUBSCRIBED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212518i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_WRITER_INFRASTRUCTURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212520i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_WRITER_NOT_RESPONDING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212519i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_E_WRITER_STATUS_NOT_AVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212279i32 as _);
pub struct VSS_FILE_RESTORE_STATUS(i32);
pub struct VSS_FILE_SPEC_BACKUP_TYPE(i32);
pub struct VSS_HARDWARE_OPTIONS(i32);
pub struct VSS_MGMT_OBJECT_PROP(i32);
pub struct VSS_MGMT_OBJECT_TYPE(i32);
pub struct VSS_MGMT_OBJECT_UNION(i32);
pub struct VSS_OBJECT_PROP(i32);
pub struct VSS_OBJECT_TYPE(i32);
pub struct VSS_OBJECT_UNION(i32);
pub struct VSS_PROTECTION_FAULT(i32);
pub struct VSS_PROTECTION_LEVEL(i32);
pub struct VSS_PROVIDER_CAPABILITIES(i32);
pub struct VSS_PROVIDER_PROP(i32);
pub struct VSS_PROVIDER_TYPE(i32);
pub struct VSS_RECOVERY_OPTIONS(i32);
pub struct VSS_RESTOREMETHOD_ENUM(i32);
pub struct VSS_RESTORE_TARGET(i32);
pub struct VSS_RESTORE_TYPE(i32);
pub struct VSS_ROLLFORWARD_TYPE(i32);
pub struct VSS_SNAPSHOT_COMPATIBILITY(i32);
pub struct VSS_SNAPSHOT_CONTEXT(i32);
pub struct VSS_SNAPSHOT_PROP(i32);
pub struct VSS_SNAPSHOT_PROPERTY_ID(i32);
pub struct VSS_SNAPSHOT_STATE(i32);
pub struct VSS_SOURCE_TYPE(i32);
pub struct VSS_SUBSCRIBE_MASK(i32);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_S_ASYNC_CANCELLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(271115i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_S_ASYNC_FINISHED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(271114i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_S_ASYNC_PENDING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(271113i32 as _);
#[doc = "*Required features: `Win32_Storage_Vss`*"]
pub const VSS_S_SOME_SNAPSHOTS_NOT_IMPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(271137i32 as _);
pub struct VSS_USAGE_TYPE(i32);
pub struct VSS_VOLUME_PROP(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct VSS_VOLUME_PROTECTION_INFO(i32);
pub struct VSS_VOLUME_SNAPSHOT_ATTRIBUTES(i32);
pub struct VSS_WRITERRESTORE_ENUM(i32);
pub struct VSS_WRITER_STATE(i32);
pub struct VssSnapshotMgmt(i32);

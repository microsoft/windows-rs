#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    pub fn CreateVssExpressWriterInternal(ppwriter: *mut IVssExpressWriter) -> ::windows_sys::core::HRESULT;
}
#[repr(transparent)]
pub struct IVssAdmin(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVssAdmin {}
impl ::core::clone::Clone for IVssAdmin {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVssAdminEx(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVssAdminEx {}
impl ::core::clone::Clone for IVssAdminEx {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVssAsync(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVssAsync {}
impl ::core::clone::Clone for IVssAsync {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVssComponent(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVssComponent {}
impl ::core::clone::Clone for IVssComponent {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVssComponentEx(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVssComponentEx {}
impl ::core::clone::Clone for IVssComponentEx {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVssComponentEx2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVssComponentEx2 {}
impl ::core::clone::Clone for IVssComponentEx2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVssCreateExpressWriterMetadata(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVssCreateExpressWriterMetadata {}
impl ::core::clone::Clone for IVssCreateExpressWriterMetadata {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVssCreateWriterMetadata(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVssCreateWriterMetadata {}
impl ::core::clone::Clone for IVssCreateWriterMetadata {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVssDifferentialSoftwareSnapshotMgmt(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVssDifferentialSoftwareSnapshotMgmt {}
impl ::core::clone::Clone for IVssDifferentialSoftwareSnapshotMgmt {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVssDifferentialSoftwareSnapshotMgmt2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVssDifferentialSoftwareSnapshotMgmt2 {}
impl ::core::clone::Clone for IVssDifferentialSoftwareSnapshotMgmt2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVssDifferentialSoftwareSnapshotMgmt3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVssDifferentialSoftwareSnapshotMgmt3 {}
impl ::core::clone::Clone for IVssDifferentialSoftwareSnapshotMgmt3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVssEnumMgmtObject(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVssEnumMgmtObject {}
impl ::core::clone::Clone for IVssEnumMgmtObject {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVssEnumObject(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVssEnumObject {}
impl ::core::clone::Clone for IVssEnumObject {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IVssExamineWriterMetadata(pub u8);
#[repr(transparent)]
pub struct IVssExpressWriter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVssExpressWriter {}
impl ::core::clone::Clone for IVssExpressWriter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVssFileShareSnapshotProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVssFileShareSnapshotProvider {}
impl ::core::clone::Clone for IVssFileShareSnapshotProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVssHardwareSnapshotProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVssHardwareSnapshotProvider {}
impl ::core::clone::Clone for IVssHardwareSnapshotProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVssHardwareSnapshotProviderEx(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVssHardwareSnapshotProviderEx {}
impl ::core::clone::Clone for IVssHardwareSnapshotProviderEx {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVssProviderCreateSnapshotSet(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVssProviderCreateSnapshotSet {}
impl ::core::clone::Clone for IVssProviderCreateSnapshotSet {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVssProviderNotifications(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVssProviderNotifications {}
impl ::core::clone::Clone for IVssProviderNotifications {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVssSnapshotMgmt(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVssSnapshotMgmt {}
impl ::core::clone::Clone for IVssSnapshotMgmt {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVssSnapshotMgmt2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVssSnapshotMgmt2 {}
impl ::core::clone::Clone for IVssSnapshotMgmt2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVssSoftwareSnapshotProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVssSoftwareSnapshotProvider {}
impl ::core::clone::Clone for IVssSoftwareSnapshotProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVssWMDependency(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVssWMDependency {}
impl ::core::clone::Clone for IVssWMDependency {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVssWMFiledesc(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVssWMFiledesc {}
impl ::core::clone::Clone for IVssWMFiledesc {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVssWriterComponents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVssWriterComponents {}
impl ::core::clone::Clone for IVssWriterComponents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVssWriterImpl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVssWriterImpl {}
impl ::core::clone::Clone for IVssWriterImpl {
    fn clone(&self) -> Self {
        *self
    }
}
pub const VSSCoordinator: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3849956191, data2: 7364, data3: 17588, data4: [190, 217, 222, 9, 145, 255, 6, 35] };
pub const VSS_AWS_UNDEFINED: i32 = 0i32;
pub const VSS_AWS_NO_ALTERNATE_WRITER: i32 = 1i32;
pub const VSS_AWS_ALTERNATE_WRITER_EXISTS: i32 = 2i32;
pub const VSS_AWS_THIS_IS_ALTERNATE_WRITER: i32 = 3i32;
pub const VSS_APP_UNKNOWN: i32 = 0i32;
pub const VSS_APP_SYSTEM: i32 = 1i32;
pub const VSS_APP_BACK_END: i32 = 2i32;
pub const VSS_APP_FRONT_END: i32 = 3i32;
pub const VSS_APP_SYSTEM_RM: i32 = 4i32;
pub const VSS_APP_AUTO: i32 = -1i32;
pub const VSS_ASSOC_NO_MAX_SPACE: i32 = -1i32;
pub const VSS_ASSOC_REMOVE: u32 = 0u32;
pub const VSS_BS_UNDEFINED: i32 = 0i32;
pub const VSS_BS_DIFFERENTIAL: i32 = 1i32;
pub const VSS_BS_INCREMENTAL: i32 = 2i32;
pub const VSS_BS_EXCLUSIVE_INCREMENTAL_DIFFERENTIAL: i32 = 4i32;
pub const VSS_BS_LOG: i32 = 8i32;
pub const VSS_BS_COPY: i32 = 16i32;
pub const VSS_BS_TIMESTAMPED: i32 = 32i32;
pub const VSS_BS_LAST_MODIFY: i32 = 64i32;
pub const VSS_BS_LSN: i32 = 128i32;
pub const VSS_BS_WRITER_SUPPORTS_NEW_TARGET: i32 = 256i32;
pub const VSS_BS_WRITER_SUPPORTS_RESTORE_WITH_MOVE: i32 = 512i32;
pub const VSS_BS_INDEPENDENT_SYSTEM_STATE: i32 = 1024i32;
pub const VSS_BS_ROLLFORWARD_RESTORE: i32 = 4096i32;
pub const VSS_BS_RESTORE_RENAME: i32 = 8192i32;
pub const VSS_BS_AUTHORITATIVE_RESTORE: i32 = 16384i32;
pub const VSS_BS_WRITER_SUPPORTS_PARALLEL_RESTORES: i32 = 32768i32;
pub const VSS_BT_UNDEFINED: i32 = 0i32;
pub const VSS_BT_FULL: i32 = 1i32;
pub const VSS_BT_INCREMENTAL: i32 = 2i32;
pub const VSS_BT_DIFFERENTIAL: i32 = 3i32;
pub const VSS_BT_LOG: i32 = 4i32;
pub const VSS_BT_COPY: i32 = 5i32;
pub const VSS_BT_OTHER: i32 = 6i32;
pub const VSS_CF_BACKUP_RECOVERY: i32 = 1i32;
pub const VSS_CF_APP_ROLLBACK_RECOVERY: i32 = 2i32;
pub const VSS_CF_NOT_SYSTEM_STATE: i32 = 4i32;
pub const VSS_CT_UNDEFINED: i32 = 0i32;
pub const VSS_CT_DATABASE: i32 = 1i32;
pub const VSS_CT_FILEGROUP: i32 = 2i32;
#[repr(C)]
pub struct VSS_DIFF_AREA_PROP {
    pub m_pwszVolumeName: *mut u16,
    pub m_pwszDiffAreaVolumeName: *mut u16,
    pub m_llMaximumDiffSpace: i64,
    pub m_llAllocatedDiffSpace: i64,
    pub m_llUsedDiffSpace: i64,
}
impl ::core::marker::Copy for VSS_DIFF_AREA_PROP {}
impl ::core::clone::Clone for VSS_DIFF_AREA_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct VSS_DIFF_VOLUME_PROP {
    pub m_pwszVolumeName: *mut u16,
    pub m_pwszVolumeDisplayName: *mut u16,
    pub m_llVolumeFreeSpace: i64,
    pub m_llVolumeTotalSpace: i64,
}
impl ::core::marker::Copy for VSS_DIFF_VOLUME_PROP {}
impl ::core::clone::Clone for VSS_DIFF_VOLUME_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
pub const VSS_E_ASRERROR_CRITICAL_DISKS_TOO_SMALL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212280i32 as _);
pub const VSS_E_ASRERROR_CRITICAL_DISK_CANNOT_BE_EXCLUDED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212267i32 as _);
pub const VSS_E_ASRERROR_DATADISK_RDISK0: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212282i32 as _);
pub const VSS_E_ASRERROR_DISK_ASSIGNMENT_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212287i32 as _);
pub const VSS_E_ASRERROR_DISK_RECREATION_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212286i32 as _);
pub const VSS_E_ASRERROR_DYNAMIC_VHD_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212278i32 as _);
pub const VSS_E_ASRERROR_FIXED_PHYSICAL_DISK_AVAILABLE_AFTER_DISK_EXCLUSION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212268i32 as _);
pub const VSS_E_ASRERROR_MISSING_DYNDISK: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212284i32 as _);
pub const VSS_E_ASRERROR_NO_ARCPATH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212285i32 as _);
pub const VSS_E_ASRERROR_NO_PHYSICAL_DISK_AVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212269i32 as _);
pub const VSS_E_ASRERROR_RDISK0_TOOSMALL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212281i32 as _);
pub const VSS_E_ASRERROR_RDISK_FOR_SYSTEM_DISK_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212270i32 as _);
pub const VSS_E_ASRERROR_SHARED_CRIDISK: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212283i32 as _);
pub const VSS_E_ASRERROR_SYSTEM_PARTITION_HIDDEN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212266i32 as _);
pub const VSS_E_AUTORECOVERY_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212293i32 as _);
pub const VSS_E_BAD_STATE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212543i32 as _);
pub const VSS_E_BREAK_REVERT_ID_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212298i32 as _);
pub const VSS_E_CANNOT_REVERT_DISKID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212290i32 as _);
pub const VSS_E_CLUSTER_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212288i32 as _);
pub const VSS_E_CLUSTER_TIMEOUT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212498i32 as _);
pub const VSS_E_CORRUPT_XML_DOCUMENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212528i32 as _);
pub const VSS_E_CRITICAL_VOLUME_ON_INVALID_DISK: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212271i32 as _);
pub const VSS_E_DYNAMIC_DISK_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212292i32 as _);
pub const VSS_E_FLUSH_WRITES_TIMEOUT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212525i32 as _);
pub const VSS_E_FSS_TIMEOUT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212265i32 as _);
pub const VSS_E_HOLD_WRITES_TIMEOUT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212524i32 as _);
pub const VSS_E_INSUFFICIENT_STORAGE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212513i32 as _);
pub const VSS_E_INVALID_XML_DOCUMENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212527i32 as _);
pub const VSS_E_LEGACY_PROVIDER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212297i32 as _);
pub const VSS_E_MAXIMUM_DIFFAREA_ASSOCIATIONS_REACHED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212514i32 as _);
pub const VSS_E_MAXIMUM_NUMBER_OF_REMOTE_MACHINES_REACHED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212510i32 as _);
pub const VSS_E_MAXIMUM_NUMBER_OF_SNAPSHOTS_REACHED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212521i32 as _);
pub const VSS_E_MAXIMUM_NUMBER_OF_VOLUMES_REACHED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212526i32 as _);
pub const VSS_E_MISSING_DISK: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212296i32 as _);
pub const VSS_E_MISSING_HIDDEN_VOLUME: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212295i32 as _);
pub const VSS_E_MISSING_VOLUME: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212294i32 as _);
pub const VSS_E_NESTED_VOLUME_LIMIT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212500i32 as _);
pub const VSS_E_NONTRANSPORTABLE_BCD: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212291i32 as _);
pub const VSS_E_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212497i32 as _);
pub const VSS_E_NO_SNAPSHOTS_IMPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212512i32 as _);
pub const VSS_E_OBJECT_ALREADY_EXISTS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212531i32 as _);
pub const VSS_E_OBJECT_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212536i32 as _);
pub const VSS_E_PROVIDER_ALREADY_REGISTERED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212541i32 as _);
pub const VSS_E_PROVIDER_IN_USE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212537i32 as _);
pub const VSS_E_PROVIDER_NOT_REGISTERED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212540i32 as _);
pub const VSS_E_PROVIDER_VETO: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212538i32 as _);
pub const VSS_E_REBOOT_REQUIRED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212505i32 as _);
pub const VSS_E_REMOTE_SERVER_UNAVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212509i32 as _);
pub const VSS_E_REMOTE_SERVER_UNSUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212508i32 as _);
pub const VSS_E_RESYNC_IN_PROGRESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212289i32 as _);
pub const VSS_E_REVERT_IN_PROGRESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212507i32 as _);
pub const VSS_E_REVERT_VOLUME_LOST: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212506i32 as _);
pub const VSS_E_SNAPSHOT_NOT_IN_SET: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212501i32 as _);
pub const VSS_E_SNAPSHOT_SET_IN_PROGRESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212522i32 as _);
pub const VSS_E_SOME_SNAPSHOTS_NOT_IMPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212511i32 as _);
pub const VSS_E_TRANSACTION_FREEZE_TIMEOUT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212504i32 as _);
pub const VSS_E_TRANSACTION_THAW_TIMEOUT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212503i32 as _);
pub const VSS_E_UNEXPECTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212542i32 as _);
pub const VSS_E_UNEXPECTED_PROVIDER_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212529i32 as _);
pub const VSS_E_UNEXPECTED_WRITER_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212523i32 as _);
pub const VSS_E_UNSELECTED_VOLUME: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212502i32 as _);
pub const VSS_E_UNSUPPORTED_CONTEXT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212517i32 as _);
pub const VSS_E_VOLUME_IN_USE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212515i32 as _);
pub const VSS_E_VOLUME_NOT_LOCAL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212499i32 as _);
pub const VSS_E_VOLUME_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212532i32 as _);
pub const VSS_E_VOLUME_NOT_SUPPORTED_BY_PROVIDER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212530i32 as _);
pub const VSS_E_WRITERERROR_INCONSISTENTSNAPSHOT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212304i32 as _);
pub const VSS_E_WRITERERROR_NONRETRYABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212300i32 as _);
pub const VSS_E_WRITERERROR_OUTOFRESOURCES: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212303i32 as _);
pub const VSS_E_WRITERERROR_PARTIAL_FAILURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212490i32 as _);
pub const VSS_E_WRITERERROR_RECOVERY_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212299i32 as _);
pub const VSS_E_WRITERERROR_RETRYABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212301i32 as _);
pub const VSS_E_WRITERERROR_TIMEOUT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212302i32 as _);
pub const VSS_E_WRITER_ALREADY_SUBSCRIBED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212518i32 as _);
pub const VSS_E_WRITER_INFRASTRUCTURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212520i32 as _);
pub const VSS_E_WRITER_NOT_RESPONDING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212519i32 as _);
pub const VSS_E_WRITER_STATUS_NOT_AVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212279i32 as _);
pub const VSS_RS_UNDEFINED: i32 = 0i32;
pub const VSS_RS_NONE: i32 = 1i32;
pub const VSS_RS_ALL: i32 = 2i32;
pub const VSS_RS_FAILED: i32 = 3i32;
pub const VSS_FSBT_FULL_BACKUP_REQUIRED: i32 = 1i32;
pub const VSS_FSBT_DIFFERENTIAL_BACKUP_REQUIRED: i32 = 2i32;
pub const VSS_FSBT_INCREMENTAL_BACKUP_REQUIRED: i32 = 4i32;
pub const VSS_FSBT_LOG_BACKUP_REQUIRED: i32 = 8i32;
pub const VSS_FSBT_FULL_SNAPSHOT_REQUIRED: i32 = 256i32;
pub const VSS_FSBT_DIFFERENTIAL_SNAPSHOT_REQUIRED: i32 = 512i32;
pub const VSS_FSBT_INCREMENTAL_SNAPSHOT_REQUIRED: i32 = 1024i32;
pub const VSS_FSBT_LOG_SNAPSHOT_REQUIRED: i32 = 2048i32;
pub const VSS_FSBT_CREATED_DURING_BACKUP: i32 = 65536i32;
pub const VSS_FSBT_ALL_BACKUP_REQUIRED: i32 = 15i32;
pub const VSS_FSBT_ALL_SNAPSHOT_REQUIRED: i32 = 3840i32;
pub const VSS_BREAKEX_FLAG_MASK_LUNS: i32 = 1i32;
pub const VSS_BREAKEX_FLAG_MAKE_READ_WRITE: i32 = 2i32;
pub const VSS_BREAKEX_FLAG_REVERT_IDENTITY_ALL: i32 = 4i32;
pub const VSS_BREAKEX_FLAG_REVERT_IDENTITY_NONE: i32 = 8i32;
pub const VSS_ONLUNSTATECHANGE_NOTIFY_READ_WRITE: i32 = 256i32;
pub const VSS_ONLUNSTATECHANGE_NOTIFY_LUN_PRE_RECOVERY: i32 = 512i32;
pub const VSS_ONLUNSTATECHANGE_NOTIFY_LUN_POST_RECOVERY: i32 = 1024i32;
pub const VSS_ONLUNSTATECHANGE_DO_MASK_LUNS: i32 = 2048i32;
#[repr(C)]
pub struct VSS_MGMT_OBJECT_PROP {
    pub Type: VSS_MGMT_OBJECT_TYPE,
    pub Obj: VSS_MGMT_OBJECT_UNION,
}
impl ::core::marker::Copy for VSS_MGMT_OBJECT_PROP {}
impl ::core::clone::Clone for VSS_MGMT_OBJECT_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
pub const VSS_MGMT_OBJECT_UNKNOWN: i32 = 0i32;
pub const VSS_MGMT_OBJECT_VOLUME: i32 = 1i32;
pub const VSS_MGMT_OBJECT_DIFF_VOLUME: i32 = 2i32;
pub const VSS_MGMT_OBJECT_DIFF_AREA: i32 = 3i32;
#[repr(C)]
pub union VSS_MGMT_OBJECT_UNION {
    pub Vol: VSS_VOLUME_PROP,
    pub DiffVol: VSS_DIFF_VOLUME_PROP,
    pub DiffArea: VSS_DIFF_AREA_PROP,
}
impl ::core::marker::Copy for VSS_MGMT_OBJECT_UNION {}
impl ::core::clone::Clone for VSS_MGMT_OBJECT_UNION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct VSS_OBJECT_PROP {
    pub Type: VSS_OBJECT_TYPE,
    pub Obj: VSS_OBJECT_UNION,
}
impl ::core::marker::Copy for VSS_OBJECT_PROP {}
impl ::core::clone::Clone for VSS_OBJECT_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
pub const VSS_OBJECT_UNKNOWN: i32 = 0i32;
pub const VSS_OBJECT_NONE: i32 = 1i32;
pub const VSS_OBJECT_SNAPSHOT_SET: i32 = 2i32;
pub const VSS_OBJECT_SNAPSHOT: i32 = 3i32;
pub const VSS_OBJECT_PROVIDER: i32 = 4i32;
pub const VSS_OBJECT_TYPE_COUNT: i32 = 5i32;
#[repr(C)]
pub union VSS_OBJECT_UNION {
    pub Snap: VSS_SNAPSHOT_PROP,
    pub Prov: VSS_PROVIDER_PROP,
}
impl ::core::marker::Copy for VSS_OBJECT_UNION {}
impl ::core::clone::Clone for VSS_OBJECT_UNION {
    fn clone(&self) -> Self {
        *self
    }
}
pub const VSS_PROTECTION_FAULT_NONE: i32 = 0i32;
pub const VSS_PROTECTION_FAULT_DIFF_AREA_MISSING: i32 = 1i32;
pub const VSS_PROTECTION_FAULT_IO_FAILURE_DURING_ONLINE: i32 = 2i32;
pub const VSS_PROTECTION_FAULT_META_DATA_CORRUPTION: i32 = 3i32;
pub const VSS_PROTECTION_FAULT_MEMORY_ALLOCATION_FAILURE: i32 = 4i32;
pub const VSS_PROTECTION_FAULT_MAPPED_MEMORY_FAILURE: i32 = 5i32;
pub const VSS_PROTECTION_FAULT_COW_READ_FAILURE: i32 = 6i32;
pub const VSS_PROTECTION_FAULT_COW_WRITE_FAILURE: i32 = 7i32;
pub const VSS_PROTECTION_FAULT_DIFF_AREA_FULL: i32 = 8i32;
pub const VSS_PROTECTION_FAULT_GROW_TOO_SLOW: i32 = 9i32;
pub const VSS_PROTECTION_FAULT_GROW_FAILED: i32 = 10i32;
pub const VSS_PROTECTION_FAULT_DESTROY_ALL_SNAPSHOTS: i32 = 11i32;
pub const VSS_PROTECTION_FAULT_FILE_SYSTEM_FAILURE: i32 = 12i32;
pub const VSS_PROTECTION_FAULT_IO_FAILURE: i32 = 13i32;
pub const VSS_PROTECTION_FAULT_DIFF_AREA_REMOVED: i32 = 14i32;
pub const VSS_PROTECTION_FAULT_EXTERNAL_WRITER_TO_DIFF_AREA: i32 = 15i32;
pub const VSS_PROTECTION_FAULT_MOUNT_DURING_CLUSTER_OFFLINE: i32 = 16i32;
pub const VSS_PROTECTION_LEVEL_ORIGINAL_VOLUME: i32 = 0i32;
pub const VSS_PROTECTION_LEVEL_SNAPSHOT: i32 = 1i32;
pub const VSS_PRV_CAPABILITY_LEGACY: i32 = 1i32;
pub const VSS_PRV_CAPABILITY_COMPLIANT: i32 = 2i32;
pub const VSS_PRV_CAPABILITY_LUN_REPOINT: i32 = 4i32;
pub const VSS_PRV_CAPABILITY_LUN_RESYNC: i32 = 8i32;
pub const VSS_PRV_CAPABILITY_OFFLINE_CREATION: i32 = 16i32;
pub const VSS_PRV_CAPABILITY_MULTIPLE_IMPORT: i32 = 32i32;
pub const VSS_PRV_CAPABILITY_RECYCLING: i32 = 64i32;
pub const VSS_PRV_CAPABILITY_PLEX: i32 = 128i32;
pub const VSS_PRV_CAPABILITY_DIFFERENTIAL: i32 = 256i32;
pub const VSS_PRV_CAPABILITY_CLUSTERED: i32 = 512i32;
#[repr(C)]
pub struct VSS_PROVIDER_PROP {
    pub m_ProviderId: ::windows_sys::core::GUID,
    pub m_pwszProviderName: *mut u16,
    pub m_eProviderType: VSS_PROVIDER_TYPE,
    pub m_pwszProviderVersion: *mut u16,
    pub m_ProviderVersionId: ::windows_sys::core::GUID,
    pub m_ClassId: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for VSS_PROVIDER_PROP {}
impl ::core::clone::Clone for VSS_PROVIDER_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
pub const VSS_PROV_UNKNOWN: i32 = 0i32;
pub const VSS_PROV_SYSTEM: i32 = 1i32;
pub const VSS_PROV_SOFTWARE: i32 = 2i32;
pub const VSS_PROV_HARDWARE: i32 = 3i32;
pub const VSS_PROV_FILESHARE: i32 = 4i32;
pub const VSS_RECOVERY_REVERT_IDENTITY_ALL: i32 = 256i32;
pub const VSS_RECOVERY_NO_VOLUME_CHECK: i32 = 512i32;
pub const VSS_RME_UNDEFINED: i32 = 0i32;
pub const VSS_RME_RESTORE_IF_NOT_THERE: i32 = 1i32;
pub const VSS_RME_RESTORE_IF_CAN_REPLACE: i32 = 2i32;
pub const VSS_RME_STOP_RESTORE_START: i32 = 3i32;
pub const VSS_RME_RESTORE_TO_ALTERNATE_LOCATION: i32 = 4i32;
pub const VSS_RME_RESTORE_AT_REBOOT: i32 = 5i32;
pub const VSS_RME_RESTORE_AT_REBOOT_IF_CANNOT_REPLACE: i32 = 6i32;
pub const VSS_RME_CUSTOM: i32 = 7i32;
pub const VSS_RME_RESTORE_STOP_START: i32 = 8i32;
pub const VSS_RT_UNDEFINED: i32 = 0i32;
pub const VSS_RT_ORIGINAL: i32 = 1i32;
pub const VSS_RT_ALTERNATE: i32 = 2i32;
pub const VSS_RT_DIRECTED: i32 = 3i32;
pub const VSS_RT_ORIGINAL_LOCATION: i32 = 4i32;
pub const VSS_RTYPE_UNDEFINED: i32 = 0i32;
pub const VSS_RTYPE_BY_COPY: i32 = 1i32;
pub const VSS_RTYPE_IMPORT: i32 = 2i32;
pub const VSS_RTYPE_OTHER: i32 = 3i32;
pub const VSS_RF_UNDEFINED: i32 = 0i32;
pub const VSS_RF_NONE: i32 = 1i32;
pub const VSS_RF_ALL: i32 = 2i32;
pub const VSS_RF_PARTIAL: i32 = 3i32;
pub const VSS_SC_DISABLE_DEFRAG: i32 = 1i32;
pub const VSS_SC_DISABLE_CONTENTINDEX: i32 = 2i32;
pub const VSS_CTX_BACKUP: i32 = 0i32;
pub const VSS_CTX_FILE_SHARE_BACKUP: i32 = 16i32;
pub const VSS_CTX_NAS_ROLLBACK: i32 = 25i32;
pub const VSS_CTX_APP_ROLLBACK: i32 = 9i32;
pub const VSS_CTX_CLIENT_ACCESSIBLE: i32 = 29i32;
pub const VSS_CTX_CLIENT_ACCESSIBLE_WRITERS: i32 = 13i32;
pub const VSS_CTX_ALL: i32 = -1i32;
#[repr(C)]
pub struct VSS_SNAPSHOT_PROP {
    pub m_SnapshotId: ::windows_sys::core::GUID,
    pub m_SnapshotSetId: ::windows_sys::core::GUID,
    pub m_lSnapshotsCount: i32,
    pub m_pwszSnapshotDeviceObject: *mut u16,
    pub m_pwszOriginalVolumeName: *mut u16,
    pub m_pwszOriginatingMachine: *mut u16,
    pub m_pwszServiceMachine: *mut u16,
    pub m_pwszExposedName: *mut u16,
    pub m_pwszExposedPath: *mut u16,
    pub m_ProviderId: ::windows_sys::core::GUID,
    pub m_lSnapshotAttributes: i32,
    pub m_tsCreationTimestamp: i64,
    pub m_eStatus: VSS_SNAPSHOT_STATE,
}
impl ::core::marker::Copy for VSS_SNAPSHOT_PROP {}
impl ::core::clone::Clone for VSS_SNAPSHOT_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
pub const VSS_SPROPID_UNKNOWN: i32 = 0i32;
pub const VSS_SPROPID_SNAPSHOT_ID: i32 = 1i32;
pub const VSS_SPROPID_SNAPSHOT_SET_ID: i32 = 2i32;
pub const VSS_SPROPID_SNAPSHOTS_COUNT: i32 = 3i32;
pub const VSS_SPROPID_SNAPSHOT_DEVICE: i32 = 4i32;
pub const VSS_SPROPID_ORIGINAL_VOLUME: i32 = 5i32;
pub const VSS_SPROPID_ORIGINATING_MACHINE: i32 = 6i32;
pub const VSS_SPROPID_SERVICE_MACHINE: i32 = 7i32;
pub const VSS_SPROPID_EXPOSED_NAME: i32 = 8i32;
pub const VSS_SPROPID_EXPOSED_PATH: i32 = 9i32;
pub const VSS_SPROPID_PROVIDER_ID: i32 = 10i32;
pub const VSS_SPROPID_SNAPSHOT_ATTRIBUTES: i32 = 11i32;
pub const VSS_SPROPID_CREATION_TIMESTAMP: i32 = 12i32;
pub const VSS_SPROPID_STATUS: i32 = 13i32;
pub const VSS_SS_UNKNOWN: i32 = 0i32;
pub const VSS_SS_PREPARING: i32 = 1i32;
pub const VSS_SS_PROCESSING_PREPARE: i32 = 2i32;
pub const VSS_SS_PREPARED: i32 = 3i32;
pub const VSS_SS_PROCESSING_PRECOMMIT: i32 = 4i32;
pub const VSS_SS_PRECOMMITTED: i32 = 5i32;
pub const VSS_SS_PROCESSING_COMMIT: i32 = 6i32;
pub const VSS_SS_COMMITTED: i32 = 7i32;
pub const VSS_SS_PROCESSING_POSTCOMMIT: i32 = 8i32;
pub const VSS_SS_PROCESSING_PREFINALCOMMIT: i32 = 9i32;
pub const VSS_SS_PREFINALCOMMITTED: i32 = 10i32;
pub const VSS_SS_PROCESSING_POSTFINALCOMMIT: i32 = 11i32;
pub const VSS_SS_CREATED: i32 = 12i32;
pub const VSS_SS_ABORTED: i32 = 13i32;
pub const VSS_SS_DELETED: i32 = 14i32;
pub const VSS_SS_POSTCOMMITTED: i32 = 15i32;
pub const VSS_SS_COUNT: i32 = 16i32;
pub const VSS_ST_UNDEFINED: i32 = 0i32;
pub const VSS_ST_TRANSACTEDDB: i32 = 1i32;
pub const VSS_ST_NONTRANSACTEDDB: i32 = 2i32;
pub const VSS_ST_OTHER: i32 = 3i32;
pub const VSS_SM_POST_SNAPSHOT_FLAG: i32 = 1i32;
pub const VSS_SM_BACKUP_EVENTS_FLAG: i32 = 2i32;
pub const VSS_SM_RESTORE_EVENTS_FLAG: i32 = 4i32;
pub const VSS_SM_IO_THROTTLING_FLAG: i32 = 8i32;
pub const VSS_SM_ALL_FLAGS: i32 = -1i32;
pub const VSS_S_ASYNC_CANCELLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(271115i32 as _);
pub const VSS_S_ASYNC_FINISHED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(271114i32 as _);
pub const VSS_S_ASYNC_PENDING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(271113i32 as _);
pub const VSS_S_SOME_SNAPSHOTS_NOT_IMPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(271137i32 as _);
pub const VSS_UT_UNDEFINED: i32 = 0i32;
pub const VSS_UT_BOOTABLESYSTEMSTATE: i32 = 1i32;
pub const VSS_UT_SYSTEMSERVICE: i32 = 2i32;
pub const VSS_UT_USERDATA: i32 = 3i32;
pub const VSS_UT_OTHER: i32 = 4i32;
#[repr(C)]
pub struct VSS_VOLUME_PROP {
    pub m_pwszVolumeName: *mut u16,
    pub m_pwszVolumeDisplayName: *mut u16,
}
impl ::core::marker::Copy for VSS_VOLUME_PROP {}
impl ::core::clone::Clone for VSS_VOLUME_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct VSS_VOLUME_PROTECTION_INFO {
    pub m_protectionLevel: VSS_PROTECTION_LEVEL,
    pub m_volumeIsOfflineForProtection: super::super::Foundation::BOOL,
    pub m_protectionFault: VSS_PROTECTION_FAULT,
    pub m_failureStatus: i32,
    pub m_volumeHasUnusedDiffArea: super::super::Foundation::BOOL,
    pub m_reserved: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VSS_VOLUME_PROTECTION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VSS_VOLUME_PROTECTION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const VSS_VOLSNAP_ATTR_PERSISTENT: i32 = 1i32;
pub const VSS_VOLSNAP_ATTR_NO_AUTORECOVERY: i32 = 2i32;
pub const VSS_VOLSNAP_ATTR_CLIENT_ACCESSIBLE: i32 = 4i32;
pub const VSS_VOLSNAP_ATTR_NO_AUTO_RELEASE: i32 = 8i32;
pub const VSS_VOLSNAP_ATTR_NO_WRITERS: i32 = 16i32;
pub const VSS_VOLSNAP_ATTR_TRANSPORTABLE: i32 = 32i32;
pub const VSS_VOLSNAP_ATTR_NOT_SURFACED: i32 = 64i32;
pub const VSS_VOLSNAP_ATTR_NOT_TRANSACTED: i32 = 128i32;
pub const VSS_VOLSNAP_ATTR_HARDWARE_ASSISTED: i32 = 65536i32;
pub const VSS_VOLSNAP_ATTR_DIFFERENTIAL: i32 = 131072i32;
pub const VSS_VOLSNAP_ATTR_PLEX: i32 = 262144i32;
pub const VSS_VOLSNAP_ATTR_IMPORTED: i32 = 524288i32;
pub const VSS_VOLSNAP_ATTR_EXPOSED_LOCALLY: i32 = 1048576i32;
pub const VSS_VOLSNAP_ATTR_EXPOSED_REMOTELY: i32 = 2097152i32;
pub const VSS_VOLSNAP_ATTR_AUTORECOVER: i32 = 4194304i32;
pub const VSS_VOLSNAP_ATTR_ROLLBACK_RECOVERY: i32 = 8388608i32;
pub const VSS_VOLSNAP_ATTR_DELAYED_POSTSNAPSHOT: i32 = 16777216i32;
pub const VSS_VOLSNAP_ATTR_TXF_RECOVERY: i32 = 33554432i32;
pub const VSS_VOLSNAP_ATTR_FILE_SHARE: i32 = 67108864i32;
pub const VSS_WRE_UNDEFINED: i32 = 0i32;
pub const VSS_WRE_NEVER: i32 = 1i32;
pub const VSS_WRE_IF_REPLACE_FAILS: i32 = 2i32;
pub const VSS_WRE_ALWAYS: i32 = 3i32;
pub const VSS_WS_UNKNOWN: i32 = 0i32;
pub const VSS_WS_STABLE: i32 = 1i32;
pub const VSS_WS_WAITING_FOR_FREEZE: i32 = 2i32;
pub const VSS_WS_WAITING_FOR_THAW: i32 = 3i32;
pub const VSS_WS_WAITING_FOR_POST_SNAPSHOT: i32 = 4i32;
pub const VSS_WS_WAITING_FOR_BACKUP_COMPLETE: i32 = 5i32;
pub const VSS_WS_FAILED_AT_IDENTIFY: i32 = 6i32;
pub const VSS_WS_FAILED_AT_PREPARE_BACKUP: i32 = 7i32;
pub const VSS_WS_FAILED_AT_PREPARE_SNAPSHOT: i32 = 8i32;
pub const VSS_WS_FAILED_AT_FREEZE: i32 = 9i32;
pub const VSS_WS_FAILED_AT_THAW: i32 = 10i32;
pub const VSS_WS_FAILED_AT_POST_SNAPSHOT: i32 = 11i32;
pub const VSS_WS_FAILED_AT_BACKUP_COMPLETE: i32 = 12i32;
pub const VSS_WS_FAILED_AT_PRE_RESTORE: i32 = 13i32;
pub const VSS_WS_FAILED_AT_POST_RESTORE: i32 = 14i32;
pub const VSS_WS_FAILED_AT_BACKUPSHUTDOWN: i32 = 15i32;
pub const VSS_WS_COUNT: i32 = 16i32;
pub const VssSnapshotMgmt: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 190458962,
    data2: 16057,
    data3: 18186,
    data4: [150, 226, 108, 109, 69, 112, 228, 15],
};

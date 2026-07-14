windows_core::imp::define_interface!(IVssAsync, IVssAsync_Vtbl, 0x507c37b4_cf5b_4e95_b0af_14eb9767467e);
windows_core::imp::interface_hierarchy!(IVssAsync, windows_core::IUnknown);
impl IVssAsync {
    pub unsafe fn Cancel(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Wait(&self, dwmilliseconds: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Wait)(windows_core::Interface::as_raw(self), dwmilliseconds) }
    }
    pub unsafe fn QueryStatus(&self, phrresult: *mut windows_core::HRESULT, preserved: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryStatus)(windows_core::Interface::as_raw(self), phrresult as _, preserved as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssAsync_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Wait: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub QueryStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT, *mut i32) -> windows_core::HRESULT,
}
pub trait IVssAsync_Impl: windows_core::IUnknownImpl {
    fn Cancel(&self) -> windows_core::Result<()>;
    fn Wait(&self, dwmilliseconds: u32) -> windows_core::Result<()>;
    fn QueryStatus(&self, phrresult: *mut windows_core::HRESULT, preserved: *mut i32) -> windows_core::Result<()>;
}
impl IVssAsync_Vtbl {
    pub const fn new<Identity: IVssAsync_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Cancel<Identity: IVssAsync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssAsync_Impl::Cancel(this).into()
            }
        }
        unsafe extern "system" fn Wait<Identity: IVssAsync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmilliseconds: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssAsync_Impl::Wait(this, core::mem::transmute_copy(&dwmilliseconds)).into()
            }
        }
        unsafe extern "system" fn QueryStatus<Identity: IVssAsync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phrresult: *mut windows_core::HRESULT, preserved: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssAsync_Impl::QueryStatus(this, core::mem::transmute_copy(&phrresult), core::mem::transmute_copy(&preserved)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Cancel: Cancel::<Identity, OFFSET>,
            Wait: Wait::<Identity, OFFSET>,
            QueryStatus: QueryStatus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssAsync as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IVssAsync {}
windows_core::imp::define_interface!(IVssEnumObject, IVssEnumObject_Vtbl, 0xae1c7110_2f60_11d3_8a39_00c04f72d8e3);
windows_core::imp::interface_hierarchy!(IVssEnumObject, windows_core::IUnknown);
impl IVssEnumObject {
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut VSS_OBJECT_PROP, pceltfetched: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, rgelt as _, pceltfetched as _) }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Clone(&self, ppenum: *mut Option<Self>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), core::mem::transmute(ppenum)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssEnumObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut VSS_OBJECT_PROP, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IVssEnumObject_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, rgelt: *mut VSS_OBJECT_PROP, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self, ppenum: windows_core::OutRef<IVssEnumObject>) -> windows_core::Result<()>;
}
impl IVssEnumObject_Vtbl {
    pub const fn new<Identity: IVssEnumObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IVssEnumObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut VSS_OBJECT_PROP, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssEnumObject_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IVssEnumObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssEnumObject_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IVssEnumObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssEnumObject_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IVssEnumObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssEnumObject_Impl::Clone(this, core::mem::transmute_copy(&ppenum)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssEnumObject as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IVssEnumObject {}
pub type PVSS_APPLICATION_LEVEL = *mut VSS_APPLICATION_LEVEL;
pub type PVSS_BACKUP_SCHEMA = *mut VSS_BACKUP_SCHEMA;
pub type PVSS_BACKUP_TYPE = *mut VSS_BACKUP_TYPE;
pub type PVSS_FILE_SPEC_BACKUP_TYPE = *mut VSS_FILE_SPEC_BACKUP_TYPE;
pub type PVSS_HARDWARE_OPTIONS = *mut VSS_HARDWARE_OPTIONS;
pub type PVSS_OBJECT_PROP = *mut VSS_OBJECT_PROP;
pub type PVSS_OBJECT_TYPE = *mut VSS_OBJECT_TYPE;
pub type PVSS_PROVIDER_CAPABILITIES = *mut VSS_PROVIDER_CAPABILITIES;
pub type PVSS_PROVIDER_PROP = *mut VSS_PROVIDER_PROP;
pub type PVSS_PROVIDER_TYPE = *mut VSS_PROVIDER_TYPE;
pub type PVSS_RECOVERY_OPTIONS = *mut VSS_RECOVERY_OPTIONS;
pub type PVSS_RESTORE_TYPE = *mut VSS_RESTORE_TYPE;
pub type PVSS_ROLLFORWARD_TYPE = *mut VSS_ROLLFORWARD_TYPE;
pub type PVSS_SNAPSHOT_CONTEXT = *mut VSS_SNAPSHOT_CONTEXT;
pub type PVSS_SNAPSHOT_PROP = *mut VSS_SNAPSHOT_PROP;
pub type PVSS_SNAPSHOT_PROPERTY_ID = *mut VSS_SNAPSHOT_PROPERTY_ID;
pub type PVSS_SNAPSHOT_STATE = *mut VSS_SNAPSHOT_STATE;
pub type PVSS_VOLUME_SNAPSHOT_ATTRIBUTES = *mut VSS_VOLUME_SNAPSHOT_ATTRIBUTES;
pub type PVSS_WRITER_STATE = *mut VSS_WRITER_STATE;
pub type VSS_APPLICATION_LEVEL = i32;
pub const VSS_APP_AUTO: VSS_APPLICATION_LEVEL = -1;
pub const VSS_APP_BACK_END: VSS_APPLICATION_LEVEL = 2;
pub const VSS_APP_FRONT_END: VSS_APPLICATION_LEVEL = 3;
pub const VSS_APP_SYSTEM: VSS_APPLICATION_LEVEL = 1;
pub const VSS_APP_SYSTEM_RM: VSS_APPLICATION_LEVEL = 4;
pub const VSS_APP_UNKNOWN: VSS_APPLICATION_LEVEL = 0;
pub type VSS_BACKUP_SCHEMA = i32;
pub type VSS_BACKUP_TYPE = i32;
pub const VSS_BREAKEX_FLAG_MAKE_READ_WRITE: VSS_HARDWARE_OPTIONS = 2;
pub const VSS_BREAKEX_FLAG_MASK_LUNS: VSS_HARDWARE_OPTIONS = 1;
pub const VSS_BREAKEX_FLAG_REVERT_IDENTITY_ALL: VSS_HARDWARE_OPTIONS = 4;
pub const VSS_BREAKEX_FLAG_REVERT_IDENTITY_NONE: VSS_HARDWARE_OPTIONS = 8;
pub const VSS_BS_AUTHORITATIVE_RESTORE: VSS_BACKUP_SCHEMA = 16384;
pub const VSS_BS_COPY: VSS_BACKUP_SCHEMA = 16;
pub const VSS_BS_DIFFERENTIAL: VSS_BACKUP_SCHEMA = 1;
pub const VSS_BS_EXCLUSIVE_INCREMENTAL_DIFFERENTIAL: VSS_BACKUP_SCHEMA = 4;
pub const VSS_BS_INCREMENTAL: VSS_BACKUP_SCHEMA = 2;
pub const VSS_BS_INDEPENDENT_SYSTEM_STATE: VSS_BACKUP_SCHEMA = 1024;
pub const VSS_BS_LAST_MODIFY: VSS_BACKUP_SCHEMA = 64;
pub const VSS_BS_LOG: VSS_BACKUP_SCHEMA = 8;
pub const VSS_BS_LSN: VSS_BACKUP_SCHEMA = 128;
pub const VSS_BS_RESTORE_RENAME: VSS_BACKUP_SCHEMA = 8192;
pub const VSS_BS_ROLLFORWARD_RESTORE: VSS_BACKUP_SCHEMA = 4096;
pub const VSS_BS_TIMESTAMPED: VSS_BACKUP_SCHEMA = 32;
pub const VSS_BS_UNDEFINED: VSS_BACKUP_SCHEMA = 0;
pub const VSS_BS_WRITER_SUPPORTS_NEW_TARGET: VSS_BACKUP_SCHEMA = 256;
pub const VSS_BS_WRITER_SUPPORTS_PARALLEL_RESTORES: VSS_BACKUP_SCHEMA = 32768;
pub const VSS_BS_WRITER_SUPPORTS_RESTORE_WITH_MOVE: VSS_BACKUP_SCHEMA = 512;
pub const VSS_BT_COPY: VSS_BACKUP_TYPE = 5;
pub const VSS_BT_DIFFERENTIAL: VSS_BACKUP_TYPE = 3;
pub const VSS_BT_FULL: VSS_BACKUP_TYPE = 1;
pub const VSS_BT_INCREMENTAL: VSS_BACKUP_TYPE = 2;
pub const VSS_BT_LOG: VSS_BACKUP_TYPE = 4;
pub const VSS_BT_OTHER: VSS_BACKUP_TYPE = 6;
pub const VSS_BT_UNDEFINED: VSS_BACKUP_TYPE = 0;
pub const VSS_CTX_ALL: VSS_SNAPSHOT_CONTEXT = -1;
pub const VSS_CTX_APP_ROLLBACK: VSS_SNAPSHOT_CONTEXT = 9;
pub const VSS_CTX_BACKUP: VSS_SNAPSHOT_CONTEXT = 0;
pub const VSS_CTX_CLIENT_ACCESSIBLE: VSS_SNAPSHOT_CONTEXT = 29;
pub const VSS_CTX_CLIENT_ACCESSIBLE_WRITERS: VSS_SNAPSHOT_CONTEXT = 13;
pub const VSS_CTX_FILE_SHARE_BACKUP: VSS_SNAPSHOT_CONTEXT = 16;
pub const VSS_CTX_NAS_ROLLBACK: VSS_SNAPSHOT_CONTEXT = 25;
pub type VSS_FILE_SPEC_BACKUP_TYPE = i32;
pub const VSS_FSBT_ALL_BACKUP_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = 15;
pub const VSS_FSBT_ALL_SNAPSHOT_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = 3840;
pub const VSS_FSBT_CREATED_DURING_BACKUP: VSS_FILE_SPEC_BACKUP_TYPE = 65536;
pub const VSS_FSBT_DIFFERENTIAL_BACKUP_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = 2;
pub const VSS_FSBT_DIFFERENTIAL_SNAPSHOT_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = 512;
pub const VSS_FSBT_FULL_BACKUP_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = 1;
pub const VSS_FSBT_FULL_SNAPSHOT_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = 256;
pub const VSS_FSBT_INCREMENTAL_BACKUP_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = 4;
pub const VSS_FSBT_INCREMENTAL_SNAPSHOT_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = 1024;
pub const VSS_FSBT_LOG_BACKUP_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = 8;
pub const VSS_FSBT_LOG_SNAPSHOT_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = 2048;
pub type VSS_HARDWARE_OPTIONS = i32;
pub type VSS_ID = windows_core::GUID;
pub const VSS_OBJECT_NONE: VSS_OBJECT_TYPE = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VSS_OBJECT_PROP {
    pub Type: VSS_OBJECT_TYPE,
    pub Obj: VSS_OBJECT_UNION,
}
impl Default for VSS_OBJECT_PROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const VSS_OBJECT_PROVIDER: VSS_OBJECT_TYPE = 4;
pub const VSS_OBJECT_SNAPSHOT: VSS_OBJECT_TYPE = 3;
pub const VSS_OBJECT_SNAPSHOT_SET: VSS_OBJECT_TYPE = 2;
pub type VSS_OBJECT_TYPE = i32;
pub const VSS_OBJECT_TYPE_COUNT: VSS_OBJECT_TYPE = 5;
#[repr(C)]
#[derive(Clone, Copy)]
pub union VSS_OBJECT_UNION {
    pub Snap: VSS_SNAPSHOT_PROP,
    pub Prov: VSS_PROVIDER_PROP,
}
impl Default for VSS_OBJECT_UNION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const VSS_OBJECT_UNKNOWN: VSS_OBJECT_TYPE = 0;
pub const VSS_ONLUNSTATECHANGE_DO_MASK_LUNS: VSS_HARDWARE_OPTIONS = 2048;
pub const VSS_ONLUNSTATECHANGE_NOTIFY_LUN_POST_RECOVERY: VSS_HARDWARE_OPTIONS = 1024;
pub const VSS_ONLUNSTATECHANGE_NOTIFY_LUN_PRE_RECOVERY: VSS_HARDWARE_OPTIONS = 512;
pub const VSS_ONLUNSTATECHANGE_NOTIFY_READ_WRITE: VSS_HARDWARE_OPTIONS = 256;
pub type VSS_PROVIDER_CAPABILITIES = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VSS_PROVIDER_PROP {
    pub m_ProviderId: VSS_ID,
    pub m_pwszProviderName: VSS_PWSZ,
    pub m_eProviderType: VSS_PROVIDER_TYPE,
    pub m_pwszProviderVersion: VSS_PWSZ,
    pub m_ProviderVersionId: VSS_ID,
    pub m_ClassId: windows_core::GUID,
}
pub type VSS_PROVIDER_TYPE = i32;
pub const VSS_PROV_FILESHARE: VSS_PROVIDER_TYPE = 4;
pub const VSS_PROV_HARDWARE: VSS_PROVIDER_TYPE = 3;
pub const VSS_PROV_SOFTWARE: VSS_PROVIDER_TYPE = 2;
pub const VSS_PROV_SYSTEM: VSS_PROVIDER_TYPE = 1;
pub const VSS_PROV_UNKNOWN: VSS_PROVIDER_TYPE = 0;
pub const VSS_PRV_CAPABILITY_CLUSTERED: VSS_PROVIDER_CAPABILITIES = 512;
pub const VSS_PRV_CAPABILITY_COMPLIANT: VSS_PROVIDER_CAPABILITIES = 2;
pub const VSS_PRV_CAPABILITY_DIFFERENTIAL: VSS_PROVIDER_CAPABILITIES = 256;
pub const VSS_PRV_CAPABILITY_LEGACY: VSS_PROVIDER_CAPABILITIES = 1;
pub const VSS_PRV_CAPABILITY_LUN_REPOINT: VSS_PROVIDER_CAPABILITIES = 4;
pub const VSS_PRV_CAPABILITY_LUN_RESYNC: VSS_PROVIDER_CAPABILITIES = 8;
pub const VSS_PRV_CAPABILITY_MULTIPLE_IMPORT: VSS_PROVIDER_CAPABILITIES = 32;
pub const VSS_PRV_CAPABILITY_OFFLINE_CREATION: VSS_PROVIDER_CAPABILITIES = 16;
pub const VSS_PRV_CAPABILITY_PLEX: VSS_PROVIDER_CAPABILITIES = 128;
pub const VSS_PRV_CAPABILITY_RECYCLING: VSS_PROVIDER_CAPABILITIES = 64;
pub type VSS_PWSZ = *mut u16;
pub const VSS_RECOVERY_NO_VOLUME_CHECK: VSS_RECOVERY_OPTIONS = 512;
pub type VSS_RECOVERY_OPTIONS = i32;
pub const VSS_RECOVERY_REVERT_IDENTITY_ALL: VSS_RECOVERY_OPTIONS = 256;
pub type VSS_RESTORE_TYPE = i32;
pub const VSS_RF_ALL: VSS_ROLLFORWARD_TYPE = 2;
pub const VSS_RF_NONE: VSS_ROLLFORWARD_TYPE = 1;
pub const VSS_RF_PARTIAL: VSS_ROLLFORWARD_TYPE = 3;
pub const VSS_RF_UNDEFINED: VSS_ROLLFORWARD_TYPE = 0;
pub type VSS_ROLLFORWARD_TYPE = i32;
pub const VSS_RTYPE_BY_COPY: VSS_RESTORE_TYPE = 1;
pub const VSS_RTYPE_IMPORT: VSS_RESTORE_TYPE = 2;
pub const VSS_RTYPE_OTHER: VSS_RESTORE_TYPE = 3;
pub const VSS_RTYPE_UNDEFINED: VSS_RESTORE_TYPE = 0;
pub const VSS_SC_DISABLE_CONTENTINDEX: VSS_SNAPSHOT_COMPATIBILITY = 2;
pub const VSS_SC_DISABLE_DEFRAG: VSS_SNAPSHOT_COMPATIBILITY = 1;
pub type VSS_SNAPSHOT_COMPATIBILITY = i32;
pub type VSS_SNAPSHOT_CONTEXT = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VSS_SNAPSHOT_PROP {
    pub m_SnapshotId: VSS_ID,
    pub m_SnapshotSetId: VSS_ID,
    pub m_lSnapshotsCount: i32,
    pub m_pwszSnapshotDeviceObject: VSS_PWSZ,
    pub m_pwszOriginalVolumeName: VSS_PWSZ,
    pub m_pwszOriginatingMachine: VSS_PWSZ,
    pub m_pwszServiceMachine: VSS_PWSZ,
    pub m_pwszExposedName: VSS_PWSZ,
    pub m_pwszExposedPath: VSS_PWSZ,
    pub m_ProviderId: VSS_ID,
    pub m_lSnapshotAttributes: i32,
    pub m_tsCreationTimestamp: VSS_TIMESTAMP,
    pub m_eStatus: VSS_SNAPSHOT_STATE,
}
pub type VSS_SNAPSHOT_PROPERTY_ID = i32;
pub type VSS_SNAPSHOT_STATE = i32;
pub const VSS_SPROPID_CREATION_TIMESTAMP: VSS_SNAPSHOT_PROPERTY_ID = 12;
pub const VSS_SPROPID_EXPOSED_NAME: VSS_SNAPSHOT_PROPERTY_ID = 8;
pub const VSS_SPROPID_EXPOSED_PATH: VSS_SNAPSHOT_PROPERTY_ID = 9;
pub const VSS_SPROPID_ORIGINAL_VOLUME: VSS_SNAPSHOT_PROPERTY_ID = 5;
pub const VSS_SPROPID_ORIGINATING_MACHINE: VSS_SNAPSHOT_PROPERTY_ID = 6;
pub const VSS_SPROPID_PROVIDER_ID: VSS_SNAPSHOT_PROPERTY_ID = 10;
pub const VSS_SPROPID_SERVICE_MACHINE: VSS_SNAPSHOT_PROPERTY_ID = 7;
pub const VSS_SPROPID_SNAPSHOTS_COUNT: VSS_SNAPSHOT_PROPERTY_ID = 3;
pub const VSS_SPROPID_SNAPSHOT_ATTRIBUTES: VSS_SNAPSHOT_PROPERTY_ID = 11;
pub const VSS_SPROPID_SNAPSHOT_DEVICE: VSS_SNAPSHOT_PROPERTY_ID = 4;
pub const VSS_SPROPID_SNAPSHOT_ID: VSS_SNAPSHOT_PROPERTY_ID = 1;
pub const VSS_SPROPID_SNAPSHOT_SET_ID: VSS_SNAPSHOT_PROPERTY_ID = 2;
pub const VSS_SPROPID_STATUS: VSS_SNAPSHOT_PROPERTY_ID = 13;
pub const VSS_SPROPID_UNKNOWN: VSS_SNAPSHOT_PROPERTY_ID = 0;
pub const VSS_SS_ABORTED: VSS_SNAPSHOT_STATE = 13;
pub const VSS_SS_COMMITTED: VSS_SNAPSHOT_STATE = 7;
pub const VSS_SS_COUNT: VSS_SNAPSHOT_STATE = 16;
pub const VSS_SS_CREATED: VSS_SNAPSHOT_STATE = 12;
pub const VSS_SS_DELETED: VSS_SNAPSHOT_STATE = 14;
pub const VSS_SS_POSTCOMMITTED: VSS_SNAPSHOT_STATE = 15;
pub const VSS_SS_PRECOMMITTED: VSS_SNAPSHOT_STATE = 5;
pub const VSS_SS_PREFINALCOMMITTED: VSS_SNAPSHOT_STATE = 10;
pub const VSS_SS_PREPARED: VSS_SNAPSHOT_STATE = 3;
pub const VSS_SS_PREPARING: VSS_SNAPSHOT_STATE = 1;
pub const VSS_SS_PROCESSING_COMMIT: VSS_SNAPSHOT_STATE = 6;
pub const VSS_SS_PROCESSING_POSTCOMMIT: VSS_SNAPSHOT_STATE = 8;
pub const VSS_SS_PROCESSING_POSTFINALCOMMIT: VSS_SNAPSHOT_STATE = 11;
pub const VSS_SS_PROCESSING_PRECOMMIT: VSS_SNAPSHOT_STATE = 4;
pub const VSS_SS_PROCESSING_PREFINALCOMMIT: VSS_SNAPSHOT_STATE = 9;
pub const VSS_SS_PROCESSING_PREPARE: VSS_SNAPSHOT_STATE = 2;
pub const VSS_SS_UNKNOWN: VSS_SNAPSHOT_STATE = 0;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct VSS_TIMESTAMP(pub i64);
pub const VSS_VOLSNAP_ATTR_AUTORECOVER: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 4194304;
pub const VSS_VOLSNAP_ATTR_CLIENT_ACCESSIBLE: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 4;
pub const VSS_VOLSNAP_ATTR_DELAYED_POSTSNAPSHOT: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 16777216;
pub const VSS_VOLSNAP_ATTR_DIFFERENTIAL: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 131072;
pub const VSS_VOLSNAP_ATTR_EXPOSED_LOCALLY: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 1048576;
pub const VSS_VOLSNAP_ATTR_EXPOSED_REMOTELY: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 2097152;
pub const VSS_VOLSNAP_ATTR_FILE_SHARE: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 67108864;
pub const VSS_VOLSNAP_ATTR_HARDWARE_ASSISTED: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 65536;
pub const VSS_VOLSNAP_ATTR_IMPORTED: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 524288;
pub const VSS_VOLSNAP_ATTR_NOT_SURFACED: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 64;
pub const VSS_VOLSNAP_ATTR_NOT_TRANSACTED: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 128;
pub const VSS_VOLSNAP_ATTR_NO_AUTORECOVERY: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 2;
pub const VSS_VOLSNAP_ATTR_NO_AUTO_RELEASE: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 8;
pub const VSS_VOLSNAP_ATTR_NO_WRITERS: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 16;
pub const VSS_VOLSNAP_ATTR_PERSISTENT: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 1;
pub const VSS_VOLSNAP_ATTR_PLEX: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 262144;
pub const VSS_VOLSNAP_ATTR_ROLLBACK_RECOVERY: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 8388608;
pub const VSS_VOLSNAP_ATTR_TRANSPORTABLE: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 32;
pub const VSS_VOLSNAP_ATTR_TXF_RECOVERY: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = 33554432;
pub type VSS_VOLUME_SNAPSHOT_ATTRIBUTES = i32;
pub type VSS_WRITER_STATE = i32;
pub const VSS_WS_COUNT: VSS_WRITER_STATE = 16;
pub const VSS_WS_FAILED_AT_BACKUPSHUTDOWN: VSS_WRITER_STATE = 15;
pub const VSS_WS_FAILED_AT_BACKUP_COMPLETE: VSS_WRITER_STATE = 12;
pub const VSS_WS_FAILED_AT_FREEZE: VSS_WRITER_STATE = 9;
pub const VSS_WS_FAILED_AT_IDENTIFY: VSS_WRITER_STATE = 6;
pub const VSS_WS_FAILED_AT_POST_RESTORE: VSS_WRITER_STATE = 14;
pub const VSS_WS_FAILED_AT_POST_SNAPSHOT: VSS_WRITER_STATE = 11;
pub const VSS_WS_FAILED_AT_PREPARE_BACKUP: VSS_WRITER_STATE = 7;
pub const VSS_WS_FAILED_AT_PREPARE_SNAPSHOT: VSS_WRITER_STATE = 8;
pub const VSS_WS_FAILED_AT_PRE_RESTORE: VSS_WRITER_STATE = 13;
pub const VSS_WS_FAILED_AT_THAW: VSS_WRITER_STATE = 10;
pub const VSS_WS_STABLE: VSS_WRITER_STATE = 1;
pub const VSS_WS_UNKNOWN: VSS_WRITER_STATE = 0;
pub const VSS_WS_WAITING_FOR_BACKUP_COMPLETE: VSS_WRITER_STATE = 5;
pub const VSS_WS_WAITING_FOR_FREEZE: VSS_WRITER_STATE = 2;
pub const VSS_WS_WAITING_FOR_POST_SNAPSHOT: VSS_WRITER_STATE = 4;
pub const VSS_WS_WAITING_FOR_THAW: VSS_WRITER_STATE = 3;

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
pub const CLSID_VdsLoader: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2620976481,
    data2: 54629,
    data3: 18216,
    data4: [174, 238, 200, 9, 82, 240, 236, 222],
};
pub const CLSID_VdsService: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2098803659,
    data2: 34550,
    data3: 19096,
    data4: [134, 40, 1, 190, 148, 201, 165, 117],
};
pub const GPT_PARTITION_NAME_LENGTH: u32 = 36u32;
#[repr(transparent)]
pub struct IEnumVdsObject(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEnumVdsObject {}
impl ::core::clone::Clone for IEnumVdsObject {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVdsAdmin(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVdsAdmin {}
impl ::core::clone::Clone for IVdsAdmin {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVdsAdviseSink(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVdsAdviseSink {}
impl ::core::clone::Clone for IVdsAdviseSink {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVdsAsync(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVdsAsync {}
impl ::core::clone::Clone for IVdsAsync {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVdsController(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVdsController {}
impl ::core::clone::Clone for IVdsController {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVdsControllerControllerPort(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVdsControllerControllerPort {}
impl ::core::clone::Clone for IVdsControllerControllerPort {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVdsControllerPort(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVdsControllerPort {}
impl ::core::clone::Clone for IVdsControllerPort {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVdsDrive(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVdsDrive {}
impl ::core::clone::Clone for IVdsDrive {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVdsDrive2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVdsDrive2 {}
impl ::core::clone::Clone for IVdsDrive2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVdsHwProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVdsHwProvider {}
impl ::core::clone::Clone for IVdsHwProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVdsHwProviderPrivate(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVdsHwProviderPrivate {}
impl ::core::clone::Clone for IVdsHwProviderPrivate {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVdsHwProviderPrivateMpio(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVdsHwProviderPrivateMpio {}
impl ::core::clone::Clone for IVdsHwProviderPrivateMpio {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVdsHwProviderStoragePools(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVdsHwProviderStoragePools {}
impl ::core::clone::Clone for IVdsHwProviderStoragePools {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVdsHwProviderType(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVdsHwProviderType {}
impl ::core::clone::Clone for IVdsHwProviderType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVdsHwProviderType2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVdsHwProviderType2 {}
impl ::core::clone::Clone for IVdsHwProviderType2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVdsIscsiPortal(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVdsIscsiPortal {}
impl ::core::clone::Clone for IVdsIscsiPortal {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVdsIscsiPortalGroup(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVdsIscsiPortalGroup {}
impl ::core::clone::Clone for IVdsIscsiPortalGroup {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVdsIscsiTarget(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVdsIscsiTarget {}
impl ::core::clone::Clone for IVdsIscsiTarget {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVdsLun(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVdsLun {}
impl ::core::clone::Clone for IVdsLun {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVdsLun2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVdsLun2 {}
impl ::core::clone::Clone for IVdsLun2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVdsLunControllerPorts(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVdsLunControllerPorts {}
impl ::core::clone::Clone for IVdsLunControllerPorts {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVdsLunIscsi(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVdsLunIscsi {}
impl ::core::clone::Clone for IVdsLunIscsi {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVdsLunMpio(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVdsLunMpio {}
impl ::core::clone::Clone for IVdsLunMpio {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVdsLunNaming(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVdsLunNaming {}
impl ::core::clone::Clone for IVdsLunNaming {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVdsLunNumber(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVdsLunNumber {}
impl ::core::clone::Clone for IVdsLunNumber {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVdsLunPlex(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVdsLunPlex {}
impl ::core::clone::Clone for IVdsLunPlex {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVdsMaintenance(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVdsMaintenance {}
impl ::core::clone::Clone for IVdsMaintenance {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVdsProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVdsProvider {}
impl ::core::clone::Clone for IVdsProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVdsProviderPrivate(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVdsProviderPrivate {}
impl ::core::clone::Clone for IVdsProviderPrivate {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVdsProviderSupport(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVdsProviderSupport {}
impl ::core::clone::Clone for IVdsProviderSupport {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVdsStoragePool(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVdsStoragePool {}
impl ::core::clone::Clone for IVdsStoragePool {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVdsSubSystem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVdsSubSystem {}
impl ::core::clone::Clone for IVdsSubSystem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVdsSubSystem2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVdsSubSystem2 {}
impl ::core::clone::Clone for IVdsSubSystem2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVdsSubSystemInterconnect(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVdsSubSystemInterconnect {}
impl ::core::clone::Clone for IVdsSubSystemInterconnect {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVdsSubSystemIscsi(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVdsSubSystemIscsi {}
impl ::core::clone::Clone for IVdsSubSystemIscsi {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVdsSubSystemNaming(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVdsSubSystemNaming {}
impl ::core::clone::Clone for IVdsSubSystemNaming {
    fn clone(&self) -> Self {
        *self
    }
}
pub const MAX_FS_ALLOWED_CLUSTER_SIZES_SIZE: u32 = 32u32;
pub const MAX_FS_FORMAT_SUPPORT_NAME_SIZE: u32 = 32u32;
pub const MAX_FS_NAME_SIZE: u32 = 8u32;
#[repr(C)]
pub struct VDS_ASYNC_OUTPUT {
    pub r#type: VDS_ASYNC_OUTPUT_TYPE,
    pub Anonymous: VDS_ASYNC_OUTPUT_0,
}
impl ::core::marker::Copy for VDS_ASYNC_OUTPUT {}
impl ::core::clone::Clone for VDS_ASYNC_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union VDS_ASYNC_OUTPUT_0 {
    pub cp: VDS_ASYNC_OUTPUT_0_2,
    pub cv: VDS_ASYNC_OUTPUT_0_5,
    pub bvp: VDS_ASYNC_OUTPUT_0_0,
    pub sv: VDS_ASYNC_OUTPUT_0_7,
    pub cl: VDS_ASYNC_OUTPUT_0_1,
    pub ct: VDS_ASYNC_OUTPUT_0_4,
    pub cpg: VDS_ASYNC_OUTPUT_0_3,
    pub cvd: VDS_ASYNC_OUTPUT_0_6,
}
impl ::core::marker::Copy for VDS_ASYNC_OUTPUT_0 {}
impl ::core::clone::Clone for VDS_ASYNC_OUTPUT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct VDS_ASYNC_OUTPUT_0_0 {
    pub pVolumeUnk: ::windows_sys::core::IUnknown,
}
impl ::core::marker::Copy for VDS_ASYNC_OUTPUT_0_0 {}
impl ::core::clone::Clone for VDS_ASYNC_OUTPUT_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct VDS_ASYNC_OUTPUT_0_1 {
    pub pLunUnk: ::windows_sys::core::IUnknown,
}
impl ::core::marker::Copy for VDS_ASYNC_OUTPUT_0_1 {}
impl ::core::clone::Clone for VDS_ASYNC_OUTPUT_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct VDS_ASYNC_OUTPUT_0_2 {
    pub ullOffset: u64,
    pub volumeId: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for VDS_ASYNC_OUTPUT_0_2 {}
impl ::core::clone::Clone for VDS_ASYNC_OUTPUT_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct VDS_ASYNC_OUTPUT_0_3 {
    pub pPortalGroupUnk: ::windows_sys::core::IUnknown,
}
impl ::core::marker::Copy for VDS_ASYNC_OUTPUT_0_3 {}
impl ::core::clone::Clone for VDS_ASYNC_OUTPUT_0_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct VDS_ASYNC_OUTPUT_0_4 {
    pub pTargetUnk: ::windows_sys::core::IUnknown,
}
impl ::core::marker::Copy for VDS_ASYNC_OUTPUT_0_4 {}
impl ::core::clone::Clone for VDS_ASYNC_OUTPUT_0_4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct VDS_ASYNC_OUTPUT_0_5 {
    pub pVolumeUnk: ::windows_sys::core::IUnknown,
}
impl ::core::marker::Copy for VDS_ASYNC_OUTPUT_0_5 {}
impl ::core::clone::Clone for VDS_ASYNC_OUTPUT_0_5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct VDS_ASYNC_OUTPUT_0_6 {
    pub pVDiskUnk: ::windows_sys::core::IUnknown,
}
impl ::core::marker::Copy for VDS_ASYNC_OUTPUT_0_6 {}
impl ::core::clone::Clone for VDS_ASYNC_OUTPUT_0_6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct VDS_ASYNC_OUTPUT_0_7 {
    pub ullReclaimedBytes: u64,
}
impl ::core::marker::Copy for VDS_ASYNC_OUTPUT_0_7 {}
impl ::core::clone::Clone for VDS_ASYNC_OUTPUT_0_7 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const VDS_ASYNCOUT_UNKNOWN: i32 = 0i32;
pub const VDS_ASYNCOUT_CREATEVOLUME: i32 = 1i32;
pub const VDS_ASYNCOUT_EXTENDVOLUME: i32 = 2i32;
pub const VDS_ASYNCOUT_SHRINKVOLUME: i32 = 3i32;
pub const VDS_ASYNCOUT_ADDVOLUMEPLEX: i32 = 4i32;
pub const VDS_ASYNCOUT_BREAKVOLUMEPLEX: i32 = 5i32;
pub const VDS_ASYNCOUT_REMOVEVOLUMEPLEX: i32 = 6i32;
pub const VDS_ASYNCOUT_REPAIRVOLUMEPLEX: i32 = 7i32;
pub const VDS_ASYNCOUT_RECOVERPACK: i32 = 8i32;
pub const VDS_ASYNCOUT_REPLACEDISK: i32 = 9i32;
pub const VDS_ASYNCOUT_CREATEPARTITION: i32 = 10i32;
pub const VDS_ASYNCOUT_CLEAN: i32 = 11i32;
pub const VDS_ASYNCOUT_CREATELUN: i32 = 50i32;
pub const VDS_ASYNCOUT_ADDLUNPLEX: i32 = 52i32;
pub const VDS_ASYNCOUT_REMOVELUNPLEX: i32 = 53i32;
pub const VDS_ASYNCOUT_EXTENDLUN: i32 = 54i32;
pub const VDS_ASYNCOUT_SHRINKLUN: i32 = 55i32;
pub const VDS_ASYNCOUT_RECOVERLUN: i32 = 56i32;
pub const VDS_ASYNCOUT_LOGINTOTARGET: i32 = 60i32;
pub const VDS_ASYNCOUT_LOGOUTFROMTARGET: i32 = 61i32;
pub const VDS_ASYNCOUT_CREATETARGET: i32 = 62i32;
pub const VDS_ASYNCOUT_CREATEPORTALGROUP: i32 = 63i32;
pub const VDS_ASYNCOUT_DELETETARGET: i32 = 64i32;
pub const VDS_ASYNCOUT_ADDPORTAL: i32 = 65i32;
pub const VDS_ASYNCOUT_REMOVEPORTAL: i32 = 66i32;
pub const VDS_ASYNCOUT_DELETEPORTALGROUP: i32 = 67i32;
pub const VDS_ASYNCOUT_FORMAT: i32 = 101i32;
pub const VDS_ASYNCOUT_CREATE_VDISK: i32 = 200i32;
pub const VDS_ASYNCOUT_ATTACH_VDISK: i32 = 201i32;
pub const VDS_ASYNCOUT_COMPACT_VDISK: i32 = 202i32;
pub const VDS_ASYNCOUT_MERGE_VDISK: i32 = 203i32;
pub const VDS_ASYNCOUT_EXPAND_VDISK: i32 = 204i32;
pub const VDS_ATTACH_VIRTUAL_DISK_FLAG_USE_FILE_ACL: u32 = 1u32;
#[repr(C)]
pub struct VDS_CONTROLLER_NOTIFICATION {
    pub ulEvent: VDS_NF_CONTROLLER,
    pub controllerId: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for VDS_CONTROLLER_NOTIFICATION {}
impl ::core::clone::Clone for VDS_CONTROLLER_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_CONTROLLER_PROP {
    pub id: ::windows_sys::core::GUID,
    pub pwszFriendlyName: super::super::Foundation::PWSTR,
    pub pwszIdentification: super::super::Foundation::PWSTR,
    pub status: VDS_CONTROLLER_STATUS,
    pub health: VDS_HEALTH,
    pub sNumberOfPorts: i16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_CONTROLLER_PROP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_CONTROLLER_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
pub const VDS_CS_UNKNOWN: i32 = 0i32;
pub const VDS_CS_ONLINE: i32 = 1i32;
pub const VDS_CS_NOT_READY: i32 = 2i32;
pub const VDS_CS_OFFLINE: i32 = 4i32;
pub const VDS_CS_FAILED: i32 = 5i32;
pub const VDS_CS_REMOVED: i32 = 8i32;
#[repr(C)]
pub struct VDS_DISK_NOTIFICATION {
    pub ulEvent: VDS_NF_DISK,
    pub diskId: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for VDS_DISK_NOTIFICATION {}
impl ::core::clone::Clone for VDS_DISK_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_DRIVE_EXTENT {
    pub id: ::windows_sys::core::GUID,
    pub LunId: ::windows_sys::core::GUID,
    pub ullSize: u64,
    pub bUsed: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_DRIVE_EXTENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_DRIVE_EXTENT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const VDS_DRF_HOTSPARE: i32 = 1i32;
pub const VDS_DRF_ASSIGNED: i32 = 2i32;
pub const VDS_DRF_UNASSIGNED: i32 = 4i32;
pub const VDS_DRF_HOTSPARE_IN_USE: i32 = 8i32;
pub const VDS_DRF_HOTSPARE_STANDBY: i32 = 16i32;
#[repr(C)]
pub struct VDS_DRIVE_LETTER_NOTIFICATION {
    pub ulEvent: u32,
    pub wcLetter: u16,
    pub volumeId: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for VDS_DRIVE_LETTER_NOTIFICATION {}
impl ::core::clone::Clone for VDS_DRIVE_LETTER_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct VDS_DRIVE_NOTIFICATION {
    pub ulEvent: VDS_NF_DRIVE,
    pub driveId: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for VDS_DRIVE_NOTIFICATION {}
impl ::core::clone::Clone for VDS_DRIVE_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_DRIVE_PROP {
    pub id: ::windows_sys::core::GUID,
    pub ullSize: u64,
    pub pwszFriendlyName: super::super::Foundation::PWSTR,
    pub pwszIdentification: super::super::Foundation::PWSTR,
    pub ulFlags: u32,
    pub status: VDS_DRIVE_STATUS,
    pub health: VDS_HEALTH,
    pub sInternalBusNumber: i16,
    pub sSlotNumber: i16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_DRIVE_PROP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_DRIVE_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_DRIVE_PROP2 {
    pub id: ::windows_sys::core::GUID,
    pub ullSize: u64,
    pub pwszFriendlyName: super::super::Foundation::PWSTR,
    pub pwszIdentification: super::super::Foundation::PWSTR,
    pub ulFlags: u32,
    pub status: VDS_DRIVE_STATUS,
    pub health: VDS_HEALTH,
    pub sInternalBusNumber: i16,
    pub sSlotNumber: i16,
    pub ulEnclosureNumber: u32,
    pub busType: VDS_STORAGE_BUS_TYPE,
    pub ulSpindleSpeed: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_DRIVE_PROP2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_DRIVE_PROP2 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const VDS_DRS_UNKNOWN: i32 = 0i32;
pub const VDS_DRS_ONLINE: i32 = 1i32;
pub const VDS_DRS_NOT_READY: i32 = 2i32;
pub const VDS_DRS_OFFLINE: i32 = 4i32;
pub const VDS_DRS_FAILED: i32 = 5i32;
pub const VDS_DRS_REMOVED: i32 = 8i32;
pub const VDS_E_ACCESS_DENIED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212249i32 as _);
pub const VDS_E_ACTIVE_PARTITION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212232i32 as _);
pub const VDS_E_ADDRESSES_INCOMPLETELY_SET: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211517i32 as _);
pub const VDS_E_ALIGN_BEYOND_FIRST_CYLINDER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211949i32 as _);
pub const VDS_E_ALIGN_IS_ZERO: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211888i32 as _);
pub const VDS_E_ALIGN_NOT_A_POWER_OF_TWO: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211889i32 as _);
pub const VDS_E_ALIGN_NOT_SECTOR_SIZE_MULTIPLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211948i32 as _);
pub const VDS_E_ALIGN_NOT_ZERO: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211947i32 as _);
pub const VDS_E_ALREADY_REGISTERED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212285i32 as _);
pub const VDS_E_ANOTHER_CALL_IN_PROGRESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212284i32 as _);
pub const VDS_E_ASSOCIATED_LUNS_EXIST: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211509i32 as _);
pub const VDS_E_ASSOCIATED_PORTALS_EXIST: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211508i32 as _);
pub const VDS_E_ASYNC_OBJECT_FAILURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212210i32 as _);
pub const VDS_E_BAD_BOOT_DISK: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211898i32 as _);
pub const VDS_E_BAD_COOKIE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212271i32 as _);
pub const VDS_E_BAD_LABEL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212247i32 as _);
pub const VDS_E_BAD_PNP_MESSAGE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212017i32 as _);
pub const VDS_E_BAD_PROVIDER_DATA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212223i32 as _);
pub const VDS_E_BAD_REVISION_NUMBER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211880i32 as _);
pub const VDS_E_BLOCK_CLUSTERED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147210749i32 as _);
pub const VDS_E_BOOT_DISK: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211257i32 as _);
pub const VDS_E_BOOT_PAGEFILE_DRIVE_LETTER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147210994i32 as _);
pub const VDS_E_BOOT_PARTITION_NUMBER_CHANGE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212234i32 as _);
pub const VDS_E_CACHE_CORRUPT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211946i32 as _);
pub const VDS_E_CANCEL_TOO_LATE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212276i32 as _);
pub const VDS_E_CANNOT_CLEAR_VOLUME_FLAG: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211945i32 as _);
pub const VDS_E_CANNOT_EXTEND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212274i32 as _);
pub const VDS_E_CANNOT_SHRINK: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212002i32 as _);
pub const VDS_E_CANT_INVALIDATE_FVE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211886i32 as _);
pub const VDS_E_CANT_QUICK_FORMAT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212246i32 as _);
pub const VDS_E_CLEAN_WITH_BOOTBACKING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147210743i32 as _);
pub const VDS_E_CLEAN_WITH_CRITICAL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147210990i32 as _);
pub const VDS_E_CLEAN_WITH_DATA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147210992i32 as _);
pub const VDS_E_CLEAN_WITH_OEM: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147210991i32 as _);
pub const VDS_E_CLUSTER_COUNT_BEYOND_32BITS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212240i32 as _);
pub const VDS_E_CLUSTER_SIZE_TOO_BIG: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212241i32 as _);
pub const VDS_E_CLUSTER_SIZE_TOO_SMALL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212242i32 as _);
pub const VDS_E_COMPRESSION_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147210984i32 as _);
pub const VDS_E_CONFIG_LIMIT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211976i32 as _);
pub const VDS_E_CORRUPT_EXTENT_INFO: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212021i32 as _);
pub const VDS_E_CORRUPT_NOTIFICATION_INFO: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211990i32 as _);
pub const VDS_E_CORRUPT_PARTITION_INFO: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212023i32 as _);
pub const VDS_E_CORRUPT_VOLUME_INFO: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212029i32 as _);
pub const VDS_E_CRASHDUMP_DISK: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211250i32 as _);
pub const VDS_E_CRITICAL_PLEX: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211906i32 as _);
pub const VDS_E_DELETE_WITH_BOOTBACKING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147210745i32 as _);
pub const VDS_E_DELETE_WITH_CRITICAL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147210993i32 as _);
pub const VDS_E_DEVICE_IN_USE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212269i32 as _);
pub const VDS_E_DISK_BEING_CLEANED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211944i32 as _);
pub const VDS_E_DISK_CONFIGURATION_CORRUPTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211975i32 as _);
pub const VDS_E_DISK_CONFIGURATION_NOT_IN_SYNC: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211974i32 as _);
pub const VDS_E_DISK_CONFIGURATION_UPDATE_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211973i32 as _);
pub const VDS_E_DISK_DYNAMIC: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211972i32 as _);
pub const VDS_E_DISK_HAS_BANDS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147210748i32 as _);
pub const VDS_E_DISK_IN_USE_BY_VOLUME: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212212i32 as _);
pub const VDS_E_DISK_IO_FAILING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211968i32 as _);
pub const VDS_E_DISK_IS_OFFLINE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211254i32 as _);
pub const VDS_E_DISK_IS_READ_ONLY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211253i32 as _);
pub const VDS_E_DISK_LAYOUT_PARTITIONS_TOO_SMALL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211969i32 as _);
pub const VDS_E_DISK_NOT_CONVERTIBLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211943i32 as _);
pub const VDS_E_DISK_NOT_CONVERTIBLE_SIZE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147210971i32 as _);
pub const VDS_E_DISK_NOT_EMPTY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212268i32 as _);
pub const VDS_E_DISK_NOT_FOUND_IN_PACK: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211987i32 as _);
pub const VDS_E_DISK_NOT_IMPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212206i32 as _);
pub const VDS_E_DISK_NOT_INITIALIZED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212265i32 as _);
pub const VDS_E_DISK_NOT_LOADED_TO_CACHE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212217i32 as _);
pub const VDS_E_DISK_NOT_MISSING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212031i32 as _);
pub const VDS_E_DISK_NOT_OFFLINE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211883i32 as _);
pub const VDS_E_DISK_NOT_ONLINE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212213i32 as _);
pub const VDS_E_DISK_PNP_REG_CORRUPT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212203i32 as _);
pub const VDS_E_DISK_REMOVEABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211942i32 as _);
pub const VDS_E_DISK_REMOVEABLE_NOT_EMPTY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211941i32 as _);
pub const VDS_E_DISTINCT_VOLUME: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211909i32 as _);
pub const VDS_E_DMADMIN_CORRUPT_NOTIFICATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212252i32 as _);
pub const VDS_E_DMADMIN_METHOD_CALL_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212256i32 as _);
pub const VDS_E_DMADMIN_SERVICE_CONNECTION_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212261i32 as _);
pub const VDS_E_DRIVER_INTERNAL_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212027i32 as _);
pub const VDS_E_DRIVER_INVALID_PARAM: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212004i32 as _);
pub const VDS_E_DRIVER_NO_PACK_NAME: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212019i32 as _);
pub const VDS_E_DRIVER_OBJECT_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211971i32 as _);
pub const VDS_E_DRIVE_LETTER_NOT_FREE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211940i32 as _);
pub const VDS_E_DUPLICATE_DISK: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211986i32 as _);
pub const VDS_E_DUP_EMPTY_PACK_GUID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212020i32 as _);
pub const VDS_E_DYNAMIC_DISKS_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211967i32 as _);
pub const VDS_E_EXTEND_FILE_SYSTEM_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212186i32 as _);
pub const VDS_E_EXTEND_MULTIPLE_DISKS_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211939i32 as _);
pub const VDS_E_EXTEND_TOO_MANY_CLUSTERS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147210968i32 as _);
pub const VDS_E_EXTEND_UNKNOWN_FILESYSTEM: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147210967i32 as _);
pub const VDS_E_EXTENT_EXCEEDS_DISK_FREE_SPACE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212011i32 as _);
pub const VDS_E_EXTENT_SIZE_LESS_THAN_MIN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212237i32 as _);
pub const VDS_E_FAILED_TO_OFFLINE_DISK: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211881i32 as _);
pub const VDS_E_FAILED_TO_ONLINE_DISK: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211882i32 as _);
pub const VDS_E_FAT32_FORMAT_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147210987i32 as _);
pub const VDS_E_FAT_FORMAT_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147210986i32 as _);
pub const VDS_E_FAULT_TOLERANT_DISKS_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211966i32 as _);
pub const VDS_E_FLAG_ALREADY_SET: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211911i32 as _);
pub const VDS_E_FORMAT_CRITICAL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147210989i32 as _);
pub const VDS_E_FORMAT_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147210985i32 as _);
pub const VDS_E_FORMAT_WITH_BOOTBACKING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147210744i32 as _);
pub const VDS_E_FS_NOT_DETERMINED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211885i32 as _);
pub const VDS_E_GET_SAN_POLICY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211259i32 as _);
pub const VDS_E_GPT_ATTRIBUTES_INVALID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211965i32 as _);
pub const VDS_E_HIBERNATION_FILE_DISK: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211251i32 as _);
pub const VDS_E_IA64_BOOT_MIRRORED_TO_MBR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212198i32 as _);
pub const VDS_E_IMPORT_SET_INCOMPLETE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212207i32 as _);
pub const VDS_E_INCOMPATIBLE_FILE_SYSTEM: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212251i32 as _);
pub const VDS_E_INCOMPATIBLE_MEDIA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212250i32 as _);
pub const VDS_E_INCORRECT_BOOT_VOLUME_EXTENT_INFO: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211260i32 as _);
pub const VDS_E_INCORRECT_SYSTEM_VOLUME_EXTENT_INFO: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211248i32 as _);
pub const VDS_E_INITIALIZED_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212287i32 as _);
pub const VDS_E_INITIALIZE_NOT_CALLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212286i32 as _);
pub const VDS_E_INITIATOR_ADAPTER_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211008i32 as _);
pub const VDS_E_INITIATOR_SPECIFIC_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211513i32 as _);
pub const VDS_E_INTERNAL_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212216i32 as _);
pub const VDS_E_INVALID_BLOCK_SIZE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211982i32 as _);
pub const VDS_E_INVALID_DISK: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212007i32 as _);
pub const VDS_E_INVALID_DISK_COUNT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211994i32 as _);
pub const VDS_E_INVALID_DRIVE_LETTER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211938i32 as _);
pub const VDS_E_INVALID_DRIVE_LETTER_COUNT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211937i32 as _);
pub const VDS_E_INVALID_ENUMERATOR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212028i32 as _);
pub const VDS_E_INVALID_EXTENT_COUNT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211993i32 as _);
pub const VDS_E_INVALID_FS_FLAG: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211936i32 as _);
pub const VDS_E_INVALID_FS_TYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211935i32 as _);
pub const VDS_E_INVALID_IP_ADDRESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147210997i32 as _);
pub const VDS_E_INVALID_ISCSI_PATH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147210980i32 as _);
pub const VDS_E_INVALID_ISCSI_TARGET_NAME: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211005i32 as _);
pub const VDS_E_INVALID_MEMBER_COUNT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211998i32 as _);
pub const VDS_E_INVALID_MEMBER_ORDER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211996i32 as _);
pub const VDS_E_INVALID_OBJECT_TYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211934i32 as _);
pub const VDS_E_INVALID_OPERATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212267i32 as _);
pub const VDS_E_INVALID_PACK: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212006i32 as _);
pub const VDS_E_INVALID_PARTITION_LAYOUT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211933i32 as _);
pub const VDS_E_INVALID_PARTITION_STYLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211932i32 as _);
pub const VDS_E_INVALID_PARTITION_TYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211931i32 as _);
pub const VDS_E_INVALID_PATH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147210981i32 as _);
pub const VDS_E_INVALID_PLEX_BLOCK_SIZE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211978i32 as _);
pub const VDS_E_INVALID_PLEX_COUNT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211999i32 as _);
pub const VDS_E_INVALID_PLEX_GUID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211988i32 as _);
pub const VDS_E_INVALID_PLEX_ORDER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211997i32 as _);
pub const VDS_E_INVALID_PLEX_TYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211979i32 as _);
pub const VDS_E_INVALID_PORT_PATH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211006i32 as _);
pub const VDS_E_INVALID_PROVIDER_CLSID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211930i32 as _);
pub const VDS_E_INVALID_PROVIDER_ID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211929i32 as _);
pub const VDS_E_INVALID_PROVIDER_NAME: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211928i32 as _);
pub const VDS_E_INVALID_PROVIDER_TYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211927i32 as _);
pub const VDS_E_INVALID_PROVIDER_VERSION_GUID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211926i32 as _);
pub const VDS_E_INVALID_PROVIDER_VERSION_STRING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211925i32 as _);
pub const VDS_E_INVALID_QUERY_PROVIDER_FLAG: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211924i32 as _);
pub const VDS_E_INVALID_SECTOR_SIZE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211984i32 as _);
pub const VDS_E_INVALID_SERVICE_FLAG: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211923i32 as _);
pub const VDS_E_INVALID_SHRINK_SIZE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211241i32 as _);
pub const VDS_E_INVALID_SPACE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212282i32 as _);
pub const VDS_E_INVALID_STATE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147210747i32 as _);
pub const VDS_E_INVALID_STRIPE_SIZE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211995i32 as _);
pub const VDS_E_INVALID_VOLUME_FLAG: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211922i32 as _);
pub const VDS_E_INVALID_VOLUME_LENGTH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211954i32 as _);
pub const VDS_E_INVALID_VOLUME_TYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211899i32 as _);
pub const VDS_E_IO_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212245i32 as _);
pub const VDS_E_ISCSI_CHAP_SECRET: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147210998i32 as _);
pub const VDS_E_ISCSI_GET_IKE_INFO: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211003i32 as _);
pub const VDS_E_ISCSI_GROUP_PRESHARE_KEY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147210999i32 as _);
pub const VDS_E_ISCSI_INITIATOR_NODE_NAME: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211000i32 as _);
pub const VDS_E_ISCSI_LOGIN_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211512i32 as _);
pub const VDS_E_ISCSI_LOGOUT_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211511i32 as _);
pub const VDS_E_ISCSI_LOGOUT_INCOMPLETE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211504i32 as _);
pub const VDS_E_ISCSI_SESSION_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211510i32 as _);
pub const VDS_E_ISCSI_SET_IKE_INFO: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211002i32 as _);
pub const VDS_E_LAST_VALID_DISK: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211985i32 as _);
pub const VDS_E_LBN_REMAP_ENABLED_FLAG: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212202i32 as _);
pub const VDS_E_LDM_TIMEOUT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212191i32 as _);
pub const VDS_E_LEGACY_VOLUME_FORMAT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212230i32 as _);
pub const VDS_E_LOG_UPDATE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211897i32 as _);
pub const VDS_E_LUN_DISK_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211239i32 as _);
pub const VDS_E_LUN_DISK_MISSING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211240i32 as _);
pub const VDS_E_LUN_DISK_NOT_READY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211238i32 as _);
pub const VDS_E_LUN_DISK_NO_MEDIA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211237i32 as _);
pub const VDS_E_LUN_DISK_READ_ONLY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147210978i32 as _);
pub const VDS_E_LUN_DYNAMIC: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147210976i32 as _);
pub const VDS_E_LUN_DYNAMIC_OFFLINE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147210975i32 as _);
pub const VDS_E_LUN_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211234i32 as _);
pub const VDS_E_LUN_NOT_READY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211236i32 as _);
pub const VDS_E_LUN_OFFLINE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211235i32 as _);
pub const VDS_E_LUN_SHRINK_GPT_HEADER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147210974i32 as _);
pub const VDS_E_LUN_UPDATE_DISK: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147210977i32 as _);
pub const VDS_E_MAX_USABLE_MBR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212184i32 as _);
pub const VDS_E_MEDIA_WRITE_PROTECTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212248i32 as _);
pub const VDS_E_MEMBER_IS_HEALTHY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211964i32 as _);
pub const VDS_E_MEMBER_MISSING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211958i32 as _);
pub const VDS_E_MEMBER_REGENERATING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211963i32 as _);
pub const VDS_E_MEMBER_SIZE_INVALID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212010i32 as _);
pub const VDS_E_MIGRATE_OPEN_VOLUME: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212228i32 as _);
pub const VDS_E_MIRROR_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147210973i32 as _);
pub const VDS_E_MISSING_DISK: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212204i32 as _);
pub const VDS_E_MULTIPLE_DISCOVERY_DOMAINS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211506i32 as _);
pub const VDS_E_MULTIPLE_PACKS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212001i32 as _);
pub const VDS_E_NAME_NOT_UNIQUE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211519i32 as _);
pub const VDS_E_NON_CONTIGUOUS_DATA_PARTITIONS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212229i32 as _);
pub const VDS_E_NOT_AN_UNALLOCATED_DISK: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212264i32 as _);
pub const VDS_E_NOT_ENOUGH_DRIVE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212272i32 as _);
pub const VDS_E_NOT_ENOUGH_SPACE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212273i32 as _);
pub const VDS_E_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212288i32 as _);
pub const VDS_E_NO_DISCOVERY_DOMAIN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211507i32 as _);
pub const VDS_E_NO_DISKS_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212258i32 as _);
pub const VDS_E_NO_DISK_PATHNAME: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211505i32 as _);
pub const VDS_E_NO_DRIVELETTER_FLAG: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212201i32 as _);
pub const VDS_E_NO_EXTENTS_FOR_PLEX: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211980i32 as _);
pub const VDS_E_NO_EXTENTS_FOR_VOLUME: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212218i32 as _);
pub const VDS_E_NO_FREE_SPACE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212233i32 as _);
pub const VDS_E_NO_HEALTHY_DISKS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211977i32 as _);
pub const VDS_E_NO_IMPORT_TARGET: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211501i32 as _);
pub const VDS_E_NO_MAINTENANCE_MODE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147210750i32 as _);
pub const VDS_E_NO_MEDIA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212270i32 as _);
pub const VDS_E_NO_PNP_DISK_ARRIVE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212016i32 as _);
pub const VDS_E_NO_PNP_DISK_REMOVE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212014i32 as _);
pub const VDS_E_NO_PNP_VOLUME_ARRIVE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212015i32 as _);
pub const VDS_E_NO_PNP_VOLUME_REMOVE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212013i32 as _);
pub const VDS_E_NO_POOL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147210752i32 as _);
pub const VDS_E_NO_POOL_CREATED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147210751i32 as _);
pub const VDS_E_NO_SOFTWARE_PROVIDERS_LOADED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212032i32 as _);
pub const VDS_E_NO_VALID_LOG_COPIES: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211894i32 as _);
pub const VDS_E_NO_VOLUME_LAYOUT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212030i32 as _);
pub const VDS_E_NO_VOLUME_PATHNAME: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211503i32 as _);
pub const VDS_E_NTFS_FORMAT_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147210988i32 as _);
pub const VDS_E_OBJECT_DELETED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212277i32 as _);
pub const VDS_E_OBJECT_EXISTS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212259i32 as _);
pub const VDS_E_OBJECT_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212283i32 as _);
pub const VDS_E_OBJECT_OUT_OF_SYNC: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212205i32 as _);
pub const VDS_E_OBJECT_STATUS_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212239i32 as _);
pub const VDS_E_OFFLINE_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147210970i32 as _);
pub const VDS_E_ONE_EXTENT_PER_DISK: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211983i32 as _);
pub const VDS_E_ONLINE_PACK_EXISTS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212188i32 as _);
pub const VDS_E_OPERATION_CANCELED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212275i32 as _);
pub const VDS_E_OPERATION_DENIED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212278i32 as _);
pub const VDS_E_OPERATION_PENDING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212279i32 as _);
pub const VDS_E_PACK_NAME_INVALID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211962i32 as _);
pub const VDS_E_PACK_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212208i32 as _);
pub const VDS_E_PACK_OFFLINE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212220i32 as _);
pub const VDS_E_PACK_ONLINE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212000i32 as _);
pub const VDS_E_PAGEFILE_DISK: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211252i32 as _);
pub const VDS_E_PARTITION_LDM: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211891i32 as _);
pub const VDS_E_PARTITION_LIMIT_REACHED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212281i32 as _);
pub const VDS_E_PARTITION_MSR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211892i32 as _);
pub const VDS_E_PARTITION_NON_DATA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211907i32 as _);
pub const VDS_E_PARTITION_NOT_CYLINDER_ALIGNED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211970i32 as _);
pub const VDS_E_PARTITION_NOT_EMPTY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212280i32 as _);
pub const VDS_E_PARTITION_NOT_OEM: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211921i32 as _);
pub const VDS_E_PARTITION_OF_UNKNOWN_TYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212231i32 as _);
pub const VDS_E_PARTITION_PROTECTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211920i32 as _);
pub const VDS_E_PARTITION_STYLE_MISMATCH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211919i32 as _);
pub const VDS_E_PATH_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212266i32 as _);
pub const VDS_E_PLEX_IS_HEALTHY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211961i32 as _);
pub const VDS_E_PLEX_LAST_ACTIVE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211960i32 as _);
pub const VDS_E_PLEX_MISSING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211959i32 as _);
pub const VDS_E_PLEX_NOT_LOADED_TO_CACHE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211893i32 as _);
pub const VDS_E_PLEX_REGENERATING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211957i32 as _);
pub const VDS_E_PLEX_SIZE_INVALID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211981i32 as _);
pub const VDS_E_PROVIDER_CACHE_CORRUPT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212257i32 as _);
pub const VDS_E_PROVIDER_CACHE_OUTOFSYNC: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211502i32 as _);
pub const VDS_E_PROVIDER_EXITING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212012i32 as _);
pub const VDS_E_PROVIDER_FAILURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212222i32 as _);
pub const VDS_E_PROVIDER_INITIALIZATION_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212260i32 as _);
pub const VDS_E_PROVIDER_INTERNAL_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211918i32 as _);
pub const VDS_E_PROVIDER_TYPE_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212214i32 as _);
pub const VDS_E_PROVIDER_VOL_DEVICE_NAME_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212254i32 as _);
pub const VDS_E_PROVIDER_VOL_OPEN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212253i32 as _);
pub const VDS_E_RAID5_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147210972i32 as _);
pub const VDS_E_READONLY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211900i32 as _);
pub const VDS_E_REBOOT_REQUIRED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147210996i32 as _);
pub const VDS_E_REFS_FORMAT_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147210746i32 as _);
pub const VDS_E_REPAIR_VOLUMESTATE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212192i32 as _);
pub const VDS_E_REQUIRES_CONTIGUOUS_DISK_SPACE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212224i32 as _);
pub const VDS_E_RETRY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212189i32 as _);
pub const VDS_E_REVERT_ON_CLOSE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212200i32 as _);
pub const VDS_E_REVERT_ON_CLOSE_MISMATCH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212190i32 as _);
pub const VDS_E_REVERT_ON_CLOSE_SET: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212199i32 as _);
pub const VDS_E_SECTOR_SIZE_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211229i32 as _);
pub const VDS_E_SECURITY_INCOMPLETELY_SET: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211515i32 as _);
pub const VDS_E_SET_SAN_POLICY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211258i32 as _);
pub const VDS_E_SET_TUNNEL_MODE_OUTER_ADDRESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211004i32 as _);
pub const VDS_E_SHRINK_DIRTY_VOLUME: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211878i32 as _);
pub const VDS_E_SHRINK_EXTEND_UNALIGNED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147210496i32 as _);
pub const VDS_E_SHRINK_IN_PROGRESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211887i32 as _);
pub const VDS_E_SHRINK_LUN_NOT_UNMASKED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147210979i32 as _);
pub const VDS_E_SHRINK_OVER_DATA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211242i32 as _);
pub const VDS_E_SHRINK_SIZE_LESS_THAN_MIN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211917i32 as _);
pub const VDS_E_SHRINK_SIZE_TOO_BIG: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211916i32 as _);
pub const VDS_E_SHRINK_UNKNOWN_FILESYSTEM: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147210966i32 as _);
pub const VDS_E_SHRINK_USER_CANCELLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211879i32 as _);
pub const VDS_E_SOURCE_IS_TARGET_PACK: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211992i32 as _);
pub const VDS_E_SUBSYSTEM_ID_IS_NULL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211001i32 as _);
pub const VDS_E_SYSTEM_DISK: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211247i32 as _);
pub const VDS_E_TARGET_PACK_NOT_EMPTY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212003i32 as _);
pub const VDS_E_TARGET_PORTAL_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211007i32 as _);
pub const VDS_E_TARGET_SPECIFIC_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211514i32 as _);
pub const VDS_E_TIMEOUT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212193i32 as _);
pub const VDS_E_UNABLE_TO_FIND_BOOT_DISK: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211261i32 as _);
pub const VDS_E_UNABLE_TO_FIND_SYSTEM_DISK: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211249i32 as _);
pub const VDS_E_UNEXPECTED_DISK_LAYOUT_CHANGE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211955i32 as _);
pub const VDS_E_UNRECOVERABLE_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212263i32 as _);
pub const VDS_E_UNRECOVERABLE_PROVIDER_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211915i32 as _);
pub const VDS_E_VDISK_INVALID_OP_STATE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147210982i32 as _);
pub const VDS_E_VDISK_NOT_OPEN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147210983i32 as _);
pub const VDS_E_VDISK_PATHNAME_INVALID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147210969i32 as _);
pub const VDS_E_VD_ALREADY_ATTACHED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147210956i32 as _);
pub const VDS_E_VD_ALREADY_COMPACTING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147210958i32 as _);
pub const VDS_E_VD_ALREADY_DETACHED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147210955i32 as _);
pub const VDS_E_VD_ALREADY_MERGING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147210957i32 as _);
pub const VDS_E_VD_DISK_ALREADY_EXPANDING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147210959i32 as _);
pub const VDS_E_VD_DISK_ALREADY_OPEN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147210960i32 as _);
pub const VDS_E_VD_DISK_IS_COMPACTING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147210963i32 as _);
pub const VDS_E_VD_DISK_IS_EXPANDING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147210964i32 as _);
pub const VDS_E_VD_DISK_IS_MERGING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147210962i32 as _);
pub const VDS_E_VD_DISK_NOT_OPEN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147210965i32 as _);
pub const VDS_E_VD_IS_ATTACHED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147210961i32 as _);
pub const VDS_E_VD_IS_BEING_ATTACHED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147210953i32 as _);
pub const VDS_E_VD_IS_BEING_DETACHED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147210952i32 as _);
pub const VDS_E_VD_NOT_ATTACHED_READONLY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147210954i32 as _);
pub const VDS_E_VOLUME_DISK_COUNT_MAX_EXCEEDED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211991i32 as _);
pub const VDS_E_VOLUME_EXTEND_FVE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211230i32 as _);
pub const VDS_E_VOLUME_EXTEND_FVE_CORRUPT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211232i32 as _);
pub const VDS_E_VOLUME_EXTEND_FVE_LOCKED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211233i32 as _);
pub const VDS_E_VOLUME_EXTEND_FVE_RECOVERY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211231i32 as _);
pub const VDS_E_VOLUME_GUID_PATHNAME_NOT_ALLOWED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147210995i32 as _);
pub const VDS_E_VOLUME_HAS_PATH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212194i32 as _);
pub const VDS_E_VOLUME_HIDDEN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211914i32 as _);
pub const VDS_E_VOLUME_INCOMPLETE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212238i32 as _);
pub const VDS_E_VOLUME_INVALID_NAME: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212025i32 as _);
pub const VDS_E_VOLUME_LENGTH_NOT_SECTOR_SIZE_MULTIPLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211953i32 as _);
pub const VDS_E_VOLUME_MIRRORED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211896i32 as _);
pub const VDS_E_VOLUME_NOT_A_MIRROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212219i32 as _);
pub const VDS_E_VOLUME_NOT_FOUND_IN_PACK: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211908i32 as _);
pub const VDS_E_VOLUME_NOT_HEALTHY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212226i32 as _);
pub const VDS_E_VOLUME_NOT_MOUNTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212209i32 as _);
pub const VDS_E_VOLUME_NOT_ONLINE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212227i32 as _);
pub const VDS_E_VOLUME_NOT_RETAINED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211952i32 as _);
pub const VDS_E_VOLUME_ON_DISK: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212005i32 as _);
pub const VDS_E_VOLUME_PERMANENTLY_DISMOUNTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212195i32 as _);
pub const VDS_E_VOLUME_REGENERATING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211904i32 as _);
pub const VDS_E_VOLUME_RETAINED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211951i32 as _);
pub const VDS_E_VOLUME_SHRINK_FVE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211243i32 as _);
pub const VDS_E_VOLUME_SHRINK_FVE_CORRUPT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211245i32 as _);
pub const VDS_E_VOLUME_SHRINK_FVE_LOCKED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211246i32 as _);
pub const VDS_E_VOLUME_SHRINK_FVE_RECOVERY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211244i32 as _);
pub const VDS_E_VOLUME_SIMPLE_SPANNED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211895i32 as _);
pub const VDS_E_VOLUME_SPANS_DISKS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212225i32 as _);
pub const VDS_E_VOLUME_SYNCHRONIZING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147211905i32 as _);
pub const VDS_E_VOLUME_TEMPORARILY_DISMOUNTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212196i32 as _);
pub const VDS_E_VOLUME_TOO_BIG: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212243i32 as _);
pub const VDS_E_VOLUME_TOO_SMALL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212244i32 as _);
#[repr(C)]
pub struct VDS_FILE_SYSTEM_NOTIFICATION {
    pub ulEvent: VDS_NF_FILE_SYSTEM,
    pub volumeId: ::windows_sys::core::GUID,
    pub dwPercentCompleted: u32,
}
impl ::core::marker::Copy for VDS_FILE_SYSTEM_NOTIFICATION {}
impl ::core::clone::Clone for VDS_FILE_SYSTEM_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub const VDS_FST_UNKNOWN: i32 = 0i32;
pub const VDS_FST_RAW: i32 = 1i32;
pub const VDS_FST_FAT: i32 = 2i32;
pub const VDS_FST_FAT32: i32 = 3i32;
pub const VDS_FST_NTFS: i32 = 4i32;
pub const VDS_FST_CDFS: i32 = 5i32;
pub const VDS_FST_UDF: i32 = 6i32;
pub const VDS_FST_EXFAT: i32 = 7i32;
pub const VDS_FST_CSVFS: i32 = 8i32;
pub const VDS_FST_REFS: i32 = 9i32;
#[repr(C)]
pub struct VDS_HBAPORT_PROP {
    pub id: ::windows_sys::core::GUID,
    pub wwnNode: VDS_WWN,
    pub wwnPort: VDS_WWN,
    pub r#type: VDS_HBAPORT_TYPE,
    pub status: VDS_HBAPORT_STATUS,
    pub ulPortSpeed: u32,
    pub ulSupportedPortSpeed: u32,
}
impl ::core::marker::Copy for VDS_HBAPORT_PROP {}
impl ::core::clone::Clone for VDS_HBAPORT_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
pub const VDS_HSF_UNKNOWN: i32 = 0i32;
pub const VDS_HSF_1GBIT: i32 = 1i32;
pub const VDS_HSF_2GBIT: i32 = 2i32;
pub const VDS_HSF_10GBIT: i32 = 4i32;
pub const VDS_HSF_4GBIT: i32 = 8i32;
pub const VDS_HSF_NOT_NEGOTIATED: i32 = 32768i32;
pub const VDS_HPS_UNKNOWN: i32 = 1i32;
pub const VDS_HPS_ONLINE: i32 = 2i32;
pub const VDS_HPS_OFFLINE: i32 = 3i32;
pub const VDS_HPS_BYPASSED: i32 = 4i32;
pub const VDS_HPS_DIAGNOSTICS: i32 = 5i32;
pub const VDS_HPS_LINKDOWN: i32 = 6i32;
pub const VDS_HPS_ERROR: i32 = 7i32;
pub const VDS_HPS_LOOPBACK: i32 = 8i32;
pub const VDS_HPT_UNKNOWN: i32 = 1i32;
pub const VDS_HPT_OTHER: i32 = 2i32;
pub const VDS_HPT_NOTPRESENT: i32 = 3i32;
pub const VDS_HPT_NPORT: i32 = 5i32;
pub const VDS_HPT_NLPORT: i32 = 6i32;
pub const VDS_HPT_FLPORT: i32 = 7i32;
pub const VDS_HPT_FPORT: i32 = 8i32;
pub const VDS_HPT_EPORT: i32 = 9i32;
pub const VDS_HPT_GPORT: i32 = 10i32;
pub const VDS_HPT_LPORT: i32 = 20i32;
pub const VDS_HPT_PTP: i32 = 21i32;
pub const VDS_H_UNKNOWN: i32 = 0i32;
pub const VDS_H_HEALTHY: i32 = 1i32;
pub const VDS_H_REBUILDING: i32 = 2i32;
pub const VDS_H_STALE: i32 = 3i32;
pub const VDS_H_FAILING: i32 = 4i32;
pub const VDS_H_FAILING_REDUNDANCY: i32 = 5i32;
pub const VDS_H_FAILED_REDUNDANCY: i32 = 6i32;
pub const VDS_H_FAILED_REDUNDANCY_FAILING: i32 = 7i32;
pub const VDS_H_FAILED: i32 = 8i32;
pub const VDS_H_REPLACED: i32 = 9i32;
pub const VDS_H_PENDING_FAILURE: i32 = 10i32;
pub const VDS_H_DEGRADED: i32 = 11i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_HINTS {
    pub ullHintMask: u64,
    pub ullExpectedMaximumSize: u64,
    pub ulOptimalReadSize: u32,
    pub ulOptimalReadAlignment: u32,
    pub ulOptimalWriteSize: u32,
    pub ulOptimalWriteAlignment: u32,
    pub ulMaximumDriveCount: u32,
    pub ulStripeSize: u32,
    pub bFastCrashRecoveryRequired: super::super::Foundation::BOOL,
    pub bMostlyReads: super::super::Foundation::BOOL,
    pub bOptimizeForSequentialReads: super::super::Foundation::BOOL,
    pub bOptimizeForSequentialWrites: super::super::Foundation::BOOL,
    pub bRemapEnabled: super::super::Foundation::BOOL,
    pub bReadBackVerifyEnabled: super::super::Foundation::BOOL,
    pub bWriteThroughCachingEnabled: super::super::Foundation::BOOL,
    pub bHardwareChecksumEnabled: super::super::Foundation::BOOL,
    pub bIsYankable: super::super::Foundation::BOOL,
    pub sRebuildPriority: i16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_HINTS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_HINTS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_HINTS2 {
    pub ullHintMask: u64,
    pub ullExpectedMaximumSize: u64,
    pub ulOptimalReadSize: u32,
    pub ulOptimalReadAlignment: u32,
    pub ulOptimalWriteSize: u32,
    pub ulOptimalWriteAlignment: u32,
    pub ulMaximumDriveCount: u32,
    pub ulStripeSize: u32,
    pub ulReserved1: u32,
    pub ulReserved2: u32,
    pub ulReserved3: u32,
    pub bFastCrashRecoveryRequired: super::super::Foundation::BOOL,
    pub bMostlyReads: super::super::Foundation::BOOL,
    pub bOptimizeForSequentialReads: super::super::Foundation::BOOL,
    pub bOptimizeForSequentialWrites: super::super::Foundation::BOOL,
    pub bRemapEnabled: super::super::Foundation::BOOL,
    pub bReadBackVerifyEnabled: super::super::Foundation::BOOL,
    pub bWriteThroughCachingEnabled: super::super::Foundation::BOOL,
    pub bHardwareChecksumEnabled: super::super::Foundation::BOOL,
    pub bIsYankable: super::super::Foundation::BOOL,
    pub bAllocateHotSpare: super::super::Foundation::BOOL,
    pub bUseMirroredCache: super::super::Foundation::BOOL,
    pub bReadCachingEnabled: super::super::Foundation::BOOL,
    pub bWriteCachingEnabled: super::super::Foundation::BOOL,
    pub bMediaScanEnabled: super::super::Foundation::BOOL,
    pub bConsistencyCheckEnabled: super::super::Foundation::BOOL,
    pub BusType: VDS_STORAGE_BUS_TYPE,
    pub bReserved1: super::super::Foundation::BOOL,
    pub bReserved2: super::super::Foundation::BOOL,
    pub bReserved3: super::super::Foundation::BOOL,
    pub sRebuildPriority: i16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_HINTS2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_HINTS2 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const VDS_HINT_ALLOCATEHOTSPARE: i32 = 512i32;
pub const VDS_HINT_BUSTYPE: i32 = 1024i32;
pub const VDS_HINT_CONSISTENCYCHECKENABLED: i32 = 32768i32;
pub const VDS_HINT_FASTCRASHRECOVERYREQUIRED: i32 = 1i32;
pub const VDS_HINT_HARDWARECHECKSUMENABLED: i32 = 128i32;
pub const VDS_HINT_ISYANKABLE: i32 = 256i32;
pub const VDS_HINT_MEDIASCANENABLED: i32 = 16384i32;
pub const VDS_HINT_MOSTLYREADS: i32 = 2i32;
pub const VDS_HINT_OPTIMIZEFORSEQUENTIALREADS: i32 = 4i32;
pub const VDS_HINT_OPTIMIZEFORSEQUENTIALWRITES: i32 = 8i32;
pub const VDS_HINT_READBACKVERIFYENABLED: i32 = 16i32;
pub const VDS_HINT_READCACHINGENABLED: i32 = 4096i32;
pub const VDS_HINT_REMAPENABLED: i32 = 32i32;
pub const VDS_HINT_USEMIRROREDCACHE: i32 = 2048i32;
pub const VDS_HINT_WRITECACHINGENABLED: i32 = 8192i32;
pub const VDS_HINT_WRITETHROUGHCACHINGENABLED: i32 = 64i32;
pub const VDS_HWT_UNKNOWN: i32 = 0i32;
pub const VDS_HWT_PCI_RAID: i32 = 1i32;
pub const VDS_HWT_FIBRE_CHANNEL: i32 = 2i32;
pub const VDS_HWT_ISCSI: i32 = 3i32;
pub const VDS_HWT_SAS: i32 = 4i32;
pub const VDS_HWT_HYBRID: i32 = 5i32;
#[repr(C)]
pub struct VDS_INTERCONNECT {
    pub m_addressType: VDS_INTERCONNECT_ADDRESS_TYPE,
    pub m_cbPort: u32,
    pub m_pbPort: *mut u8,
    pub m_cbAddress: u32,
    pub m_pbAddress: *mut u8,
}
impl ::core::marker::Copy for VDS_INTERCONNECT {}
impl ::core::clone::Clone for VDS_INTERCONNECT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const VDS_IA_UNKNOWN: i32 = 0i32;
pub const VDS_IA_FCFS: i32 = 1i32;
pub const VDS_IA_FCPH: i32 = 2i32;
pub const VDS_IA_FCPH3: i32 = 3i32;
pub const VDS_IA_MAC: i32 = 4i32;
pub const VDS_IA_SCSI: i32 = 5i32;
pub const VDS_ITF_PCI_RAID: i32 = 1i32;
pub const VDS_ITF_FIBRE_CHANNEL: i32 = 2i32;
pub const VDS_ITF_ISCSI: i32 = 4i32;
pub const VDS_ITF_SAS: i32 = 8i32;
#[repr(C)]
pub struct VDS_IPADDRESS {
    pub r#type: VDS_IPADDRESS_TYPE,
    pub ipv4Address: u32,
    pub ipv6Address: [u8; 16],
    pub ulIpv6FlowInfo: u32,
    pub ulIpv6ScopeId: u32,
    pub wszTextAddress: [u16; 257],
    pub ulPort: u32,
}
impl ::core::marker::Copy for VDS_IPADDRESS {}
impl ::core::clone::Clone for VDS_IPADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const VDS_IPT_TEXT: i32 = 0i32;
pub const VDS_IPT_IPV4: i32 = 1i32;
pub const VDS_IPT_IPV6: i32 = 2i32;
pub const VDS_IPT_EMPTY: i32 = 3i32;
pub const VDS_IAT_NONE: i32 = 0i32;
pub const VDS_IAT_CHAP: i32 = 1i32;
pub const VDS_IAT_MUTUAL_CHAP: i32 = 2i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_ISCSI_INITIATOR_ADAPTER_PROP {
    pub id: ::windows_sys::core::GUID,
    pub pwszName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_ISCSI_INITIATOR_ADAPTER_PROP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_ISCSI_INITIATOR_ADAPTER_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct VDS_ISCSI_INITIATOR_PORTAL_PROP {
    pub id: ::windows_sys::core::GUID,
    pub address: VDS_IPADDRESS,
    pub ulPortIndex: u32,
}
impl ::core::marker::Copy for VDS_ISCSI_INITIATOR_PORTAL_PROP {}
impl ::core::clone::Clone for VDS_ISCSI_INITIATOR_PORTAL_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
pub const VDS_IIF_VALID: i32 = 1i32;
pub const VDS_IIF_IKE: i32 = 2i32;
pub const VDS_IIF_MAIN_MODE: i32 = 4i32;
pub const VDS_IIF_AGGRESSIVE_MODE: i32 = 8i32;
pub const VDS_IIF_PFS_ENABLE: i32 = 16i32;
pub const VDS_IIF_TRANSPORT_MODE_PREFERRED: i32 = 32i32;
pub const VDS_IIF_TUNNEL_MODE_PREFERRED: i32 = 64i32;
#[repr(C)]
pub struct VDS_ISCSI_IPSEC_KEY {
    pub pKey: *mut u8,
    pub ulKeySize: u32,
}
impl ::core::marker::Copy for VDS_ISCSI_IPSEC_KEY {}
impl ::core::clone::Clone for VDS_ISCSI_IPSEC_KEY {
    fn clone(&self) -> Self {
        *self
    }
}
pub const VDS_ILF_REQUIRE_IPSEC: i32 = 1i32;
pub const VDS_ILF_MULTIPATH_ENABLED: i32 = 2i32;
pub const VDS_ILT_MANUAL: i32 = 0i32;
pub const VDS_ILT_PERSISTENT: i32 = 1i32;
pub const VDS_ILT_BOOT: i32 = 2i32;
#[repr(C)]
pub struct VDS_ISCSI_PORTALGROUP_PROP {
    pub id: ::windows_sys::core::GUID,
    pub tag: u16,
}
impl ::core::marker::Copy for VDS_ISCSI_PORTALGROUP_PROP {}
impl ::core::clone::Clone for VDS_ISCSI_PORTALGROUP_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct VDS_ISCSI_PORTAL_PROP {
    pub id: ::windows_sys::core::GUID,
    pub address: VDS_IPADDRESS,
    pub status: VDS_ISCSI_PORTAL_STATUS,
}
impl ::core::marker::Copy for VDS_ISCSI_PORTAL_PROP {}
impl ::core::clone::Clone for VDS_ISCSI_PORTAL_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
pub const VDS_IPS_UNKNOWN: i32 = 0i32;
pub const VDS_IPS_ONLINE: i32 = 1i32;
pub const VDS_IPS_NOT_READY: i32 = 2i32;
pub const VDS_IPS_OFFLINE: i32 = 4i32;
pub const VDS_IPS_FAILED: i32 = 5i32;
#[repr(C)]
pub struct VDS_ISCSI_SHARED_SECRET {
    pub pSharedSecret: *mut u8,
    pub ulSharedSecretSize: u32,
}
impl ::core::marker::Copy for VDS_ISCSI_SHARED_SECRET {}
impl ::core::clone::Clone for VDS_ISCSI_SHARED_SECRET {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_ISCSI_TARGET_PROP {
    pub id: ::windows_sys::core::GUID,
    pub pwszIscsiName: super::super::Foundation::PWSTR,
    pub pwszFriendlyName: super::super::Foundation::PWSTR,
    pub bChapEnabled: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_ISCSI_TARGET_PROP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_ISCSI_TARGET_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
pub const VDS_LBP_UNKNOWN: i32 = 0i32;
pub const VDS_LBP_FAILOVER: i32 = 1i32;
pub const VDS_LBP_ROUND_ROBIN: i32 = 2i32;
pub const VDS_LBP_ROUND_ROBIN_WITH_SUBSET: i32 = 3i32;
pub const VDS_LBP_DYN_LEAST_QUEUE_DEPTH: i32 = 4i32;
pub const VDS_LBP_WEIGHTED_PATHS: i32 = 5i32;
pub const VDS_LBP_LEAST_BLOCKS: i32 = 6i32;
pub const VDS_LBP_VENDOR_SPECIFIC: i32 = 7i32;
pub const VDS_LF_LBN_REMAP_ENABLED: i32 = 1i32;
pub const VDS_LF_READ_BACK_VERIFY_ENABLED: i32 = 2i32;
pub const VDS_LF_WRITE_THROUGH_CACHING_ENABLED: i32 = 4i32;
pub const VDS_LF_HARDWARE_CHECKSUM_ENABLED: i32 = 8i32;
pub const VDS_LF_READ_CACHE_ENABLED: i32 = 16i32;
pub const VDS_LF_WRITE_CACHE_ENABLED: i32 = 32i32;
pub const VDS_LF_MEDIA_SCAN_ENABLED: i32 = 64i32;
pub const VDS_LF_CONSISTENCY_CHECK_ENABLED: i32 = 128i32;
pub const VDS_LF_SNAPSHOT: i32 = 256i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_LUN_INFORMATION {
    pub m_version: u32,
    pub m_DeviceType: u8,
    pub m_DeviceTypeModifier: u8,
    pub m_bCommandQueueing: super::super::Foundation::BOOL,
    pub m_BusType: VDS_STORAGE_BUS_TYPE,
    pub m_szVendorId: *mut u8,
    pub m_szProductId: *mut u8,
    pub m_szProductRevision: *mut u8,
    pub m_szSerialNumber: *mut u8,
    pub m_diskSignature: ::windows_sys::core::GUID,
    pub m_deviceIdDescriptor: VDS_STORAGE_DEVICE_ID_DESCRIPTOR,
    pub m_cInterconnects: u32,
    pub m_rgInterconnects: *mut VDS_INTERCONNECT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_LUN_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_LUN_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct VDS_LUN_NOTIFICATION {
    pub ulEvent: VDS_NF_LUN,
    pub LunId: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for VDS_LUN_NOTIFICATION {}
impl ::core::clone::Clone for VDS_LUN_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub const VDS_LPF_LBN_REMAP_ENABLED: i32 = 1i32;
#[repr(C)]
pub struct VDS_LUN_PLEX_PROP {
    pub id: ::windows_sys::core::GUID,
    pub ullSize: u64,
    pub r#type: VDS_LUN_PLEX_TYPE,
    pub status: VDS_LUN_PLEX_STATUS,
    pub health: VDS_HEALTH,
    pub TransitionState: VDS_TRANSITION_STATE,
    pub ulFlags: u32,
    pub ulStripeSize: u32,
    pub sRebuildPriority: i16,
}
impl ::core::marker::Copy for VDS_LUN_PLEX_PROP {}
impl ::core::clone::Clone for VDS_LUN_PLEX_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
pub const VDS_LPS_UNKNOWN: i32 = 0i32;
pub const VDS_LPS_ONLINE: i32 = 1i32;
pub const VDS_LPS_NOT_READY: i32 = 2i32;
pub const VDS_LPS_OFFLINE: i32 = 4i32;
pub const VDS_LPS_FAILED: i32 = 5i32;
pub const VDS_LPT_UNKNOWN: i32 = 0i32;
pub const VDS_LPT_SIMPLE: i32 = 10i32;
pub const VDS_LPT_SPAN: i32 = 11i32;
pub const VDS_LPT_STRIPE: i32 = 12i32;
pub const VDS_LPT_PARITY: i32 = 14i32;
pub const VDS_LPT_RAID2: i32 = 15i32;
pub const VDS_LPT_RAID3: i32 = 16i32;
pub const VDS_LPT_RAID4: i32 = 17i32;
pub const VDS_LPT_RAID5: i32 = 18i32;
pub const VDS_LPT_RAID6: i32 = 19i32;
pub const VDS_LPT_RAID03: i32 = 21i32;
pub const VDS_LPT_RAID05: i32 = 22i32;
pub const VDS_LPT_RAID10: i32 = 23i32;
pub const VDS_LPT_RAID15: i32 = 24i32;
pub const VDS_LPT_RAID30: i32 = 25i32;
pub const VDS_LPT_RAID50: i32 = 26i32;
pub const VDS_LPT_RAID53: i32 = 28i32;
pub const VDS_LPT_RAID60: i32 = 29i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_LUN_PROP {
    pub id: ::windows_sys::core::GUID,
    pub ullSize: u64,
    pub pwszFriendlyName: super::super::Foundation::PWSTR,
    pub pwszIdentification: super::super::Foundation::PWSTR,
    pub pwszUnmaskingList: super::super::Foundation::PWSTR,
    pub ulFlags: u32,
    pub r#type: VDS_LUN_TYPE,
    pub status: VDS_LUN_STATUS,
    pub health: VDS_HEALTH,
    pub TransitionState: VDS_TRANSITION_STATE,
    pub sRebuildPriority: i16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_LUN_PROP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_LUN_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
pub const VDS_LS_UNKNOWN: i32 = 0i32;
pub const VDS_LS_ONLINE: i32 = 1i32;
pub const VDS_LS_NOT_READY: i32 = 2i32;
pub const VDS_LS_OFFLINE: i32 = 4i32;
pub const VDS_LS_FAILED: i32 = 5i32;
pub const VDS_LT_UNKNOWN: i32 = 0i32;
pub const VDS_LT_DEFAULT: i32 = 1i32;
pub const VDS_LT_FAULT_TOLERANT: i32 = 2i32;
pub const VDS_LT_NON_FAULT_TOLERANT: i32 = 3i32;
pub const VDS_LT_SIMPLE: i32 = 10i32;
pub const VDS_LT_SPAN: i32 = 11i32;
pub const VDS_LT_STRIPE: i32 = 12i32;
pub const VDS_LT_MIRROR: i32 = 13i32;
pub const VDS_LT_PARITY: i32 = 14i32;
pub const VDS_LT_RAID2: i32 = 15i32;
pub const VDS_LT_RAID3: i32 = 16i32;
pub const VDS_LT_RAID4: i32 = 17i32;
pub const VDS_LT_RAID5: i32 = 18i32;
pub const VDS_LT_RAID6: i32 = 19i32;
pub const VDS_LT_RAID01: i32 = 20i32;
pub const VDS_LT_RAID03: i32 = 21i32;
pub const VDS_LT_RAID05: i32 = 22i32;
pub const VDS_LT_RAID10: i32 = 23i32;
pub const VDS_LT_RAID15: i32 = 24i32;
pub const VDS_LT_RAID30: i32 = 25i32;
pub const VDS_LT_RAID50: i32 = 26i32;
pub const VDS_LT_RAID51: i32 = 27i32;
pub const VDS_LT_RAID53: i32 = 28i32;
pub const VDS_LT_RAID60: i32 = 29i32;
pub const VDS_LT_RAID61: i32 = 30i32;
pub const BlinkLight: i32 = 1i32;
pub const BeepAlarm: i32 = 2i32;
pub const SpinDown: i32 = 3i32;
pub const SpinUp: i32 = 4i32;
pub const Ping: i32 = 5i32;
#[repr(C)]
pub struct VDS_MOUNT_POINT_NOTIFICATION {
    pub ulEvent: u32,
    pub volumeId: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for VDS_MOUNT_POINT_NOTIFICATION {}
impl ::core::clone::Clone for VDS_MOUNT_POINT_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub const VDS_NF_CONTROLLER_ARRIVE: u32 = 103u32;
pub const VDS_NF_CONTROLLER_DEPART: u32 = 104u32;
pub const VDS_NF_CONTROLLER_MODIFY: u32 = 350u32;
pub const VDS_NF_CONTROLLER_REMOVED: u32 = 351u32;
pub const VDS_NF_DISK_ARRIVE: u32 = 8u32;
pub const VDS_NF_DISK_DEPART: u32 = 9u32;
pub const VDS_NF_DISK_MODIFY: u32 = 10u32;
pub const VDS_NF_DRIVE_ARRIVE: u32 = 105u32;
pub const VDS_NF_DRIVE_DEPART: u32 = 106u32;
pub const VDS_NF_DRIVE_MODIFY: u32 = 107u32;
pub const VDS_NF_DRIVE_REMOVED: u32 = 354u32;
pub const VDS_NF_DRIVE_LETTER_ASSIGN: u32 = 202u32;
pub const VDS_NF_DRIVE_LETTER_FREE: u32 = 201u32;
pub const VDS_NF_FILE_SYSTEM_MODIFY: u32 = 203u32;
pub const VDS_NF_FILE_SYSTEM_FORMAT_PROGRESS: u32 = 204u32;
pub const VDS_NF_FILE_SYSTEM_SHRINKING_PROGRESS: u32 = 206u32;
pub const VDS_NF_LUN_ARRIVE: u32 = 108u32;
pub const VDS_NF_LUN_DEPART: u32 = 109u32;
pub const VDS_NF_LUN_MODIFY: u32 = 110u32;
pub const VDS_NF_MOUNT_POINTS_CHANGE: u32 = 205u32;
pub const VDS_NF_PACK_ARRIVE: u32 = 1u32;
pub const VDS_NF_PACK_DEPART: u32 = 2u32;
pub const VDS_NF_PACK_MODIFY: u32 = 3u32;
pub const VDS_NF_PARTITION_ARRIVE: u32 = 11u32;
pub const VDS_NF_PARTITION_DEPART: u32 = 12u32;
pub const VDS_NF_PARTITION_MODIFY: u32 = 13u32;
pub const VDS_NF_PORT_ARRIVE: u32 = 121u32;
pub const VDS_NF_PORT_DEPART: u32 = 122u32;
pub const VDS_NF_PORT_MODIFY: u32 = 352u32;
pub const VDS_NF_PORT_REMOVED: u32 = 353u32;
pub const VDS_NF_PORTAL_ARRIVE: u32 = 123u32;
pub const VDS_NF_PORTAL_DEPART: u32 = 124u32;
pub const VDS_NF_PORTAL_GROUP_ARRIVE: u32 = 129u32;
pub const VDS_NF_PORTAL_GROUP_DEPART: u32 = 130u32;
pub const VDS_NF_PORTAL_GROUP_MODIFY: u32 = 131u32;
pub const VDS_NF_PORTAL_MODIFY: u32 = 125u32;
pub const VDS_NF_SERVICE_OUT_OF_SYNC: u32 = 301u32;
pub const VDS_NF_SUB_SYSTEM_ARRIVE: u32 = 101u32;
pub const VDS_NF_SUB_SYSTEM_DEPART: u32 = 102u32;
pub const VDS_NF_SUB_SYSTEM_MODIFY: u32 = 151u32;
pub const VDS_NF_TARGET_ARRIVE: u32 = 126u32;
pub const VDS_NF_TARGET_DEPART: u32 = 127u32;
pub const VDS_NF_TARGET_MODIFY: u32 = 128u32;
pub const VDS_NF_VOLUME_ARRIVE: u32 = 4u32;
pub const VDS_NF_VOLUME_DEPART: u32 = 5u32;
pub const VDS_NF_VOLUME_MODIFY: u32 = 6u32;
pub const VDS_NF_VOLUME_REBUILDING_PROGRESS: u32 = 7u32;
#[repr(C)]
pub struct VDS_NOTIFICATION {
    pub objectType: VDS_NOTIFICATION_TARGET_TYPE,
    pub Anonymous: VDS_NOTIFICATION_0,
}
impl ::core::marker::Copy for VDS_NOTIFICATION {}
impl ::core::clone::Clone for VDS_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union VDS_NOTIFICATION_0 {
    pub Pack: VDS_PACK_NOTIFICATION,
    pub Disk: VDS_DISK_NOTIFICATION,
    pub Volume: VDS_VOLUME_NOTIFICATION,
    pub Partition: VDS_PARTITION_NOTIFICATION,
    pub Letter: VDS_DRIVE_LETTER_NOTIFICATION,
    pub FileSystem: VDS_FILE_SYSTEM_NOTIFICATION,
    pub MountPoint: VDS_MOUNT_POINT_NOTIFICATION,
    pub SubSystem: VDS_SUB_SYSTEM_NOTIFICATION,
    pub Controller: VDS_CONTROLLER_NOTIFICATION,
    pub Drive: VDS_DRIVE_NOTIFICATION,
    pub Lun: VDS_LUN_NOTIFICATION,
    pub Port: VDS_PORT_NOTIFICATION,
    pub Portal: VDS_PORTAL_NOTIFICATION,
    pub Target: VDS_TARGET_NOTIFICATION,
    pub PortalGroup: VDS_PORTAL_GROUP_NOTIFICATION,
    pub Service: VDS_SERVICE_NOTIFICATION,
}
impl ::core::marker::Copy for VDS_NOTIFICATION_0 {}
impl ::core::clone::Clone for VDS_NOTIFICATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const VDS_NTT_UNKNOWN: i32 = 0i32;
pub const VDS_NTT_PACK: i32 = 10i32;
pub const VDS_NTT_VOLUME: i32 = 11i32;
pub const VDS_NTT_DISK: i32 = 13i32;
pub const VDS_NTT_PARTITION: i32 = 60i32;
pub const VDS_NTT_DRIVE_LETTER: i32 = 61i32;
pub const VDS_NTT_FILE_SYSTEM: i32 = 62i32;
pub const VDS_NTT_MOUNT_POINT: i32 = 63i32;
pub const VDS_NTT_SUB_SYSTEM: i32 = 30i32;
pub const VDS_NTT_CONTROLLER: i32 = 31i32;
pub const VDS_NTT_DRIVE: i32 = 32i32;
pub const VDS_NTT_LUN: i32 = 33i32;
pub const VDS_NTT_PORT: i32 = 35i32;
pub const VDS_NTT_PORTAL: i32 = 36i32;
pub const VDS_NTT_TARGET: i32 = 37i32;
pub const VDS_NTT_PORTAL_GROUP: i32 = 38i32;
pub const VDS_NTT_SERVICE: i32 = 200i32;
pub const VDS_OT_UNKNOWN: i32 = 0i32;
pub const VDS_OT_PROVIDER: i32 = 1i32;
pub const VDS_OT_PACK: i32 = 10i32;
pub const VDS_OT_VOLUME: i32 = 11i32;
pub const VDS_OT_VOLUME_PLEX: i32 = 12i32;
pub const VDS_OT_DISK: i32 = 13i32;
pub const VDS_OT_SUB_SYSTEM: i32 = 30i32;
pub const VDS_OT_CONTROLLER: i32 = 31i32;
pub const VDS_OT_DRIVE: i32 = 32i32;
pub const VDS_OT_LUN: i32 = 33i32;
pub const VDS_OT_LUN_PLEX: i32 = 34i32;
pub const VDS_OT_PORT: i32 = 35i32;
pub const VDS_OT_PORTAL: i32 = 36i32;
pub const VDS_OT_TARGET: i32 = 37i32;
pub const VDS_OT_PORTAL_GROUP: i32 = 38i32;
pub const VDS_OT_STORAGE_POOL: i32 = 39i32;
pub const VDS_OT_HBAPORT: i32 = 90i32;
pub const VDS_OT_INIT_ADAPTER: i32 = 91i32;
pub const VDS_OT_INIT_PORTAL: i32 = 92i32;
pub const VDS_OT_ASYNC: i32 = 100i32;
pub const VDS_OT_ENUM: i32 = 101i32;
pub const VDS_OT_VDISK: i32 = 200i32;
pub const VDS_OT_OPEN_VDISK: i32 = 201i32;
#[repr(C)]
pub struct VDS_PACK_NOTIFICATION {
    pub ulEvent: VDS_NF_PACK,
    pub packId: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for VDS_PACK_NOTIFICATION {}
impl ::core::clone::Clone for VDS_PACK_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct VDS_PARTITION_NOTIFICATION {
    pub ulEvent: u32,
    pub diskId: ::windows_sys::core::GUID,
    pub ullOffset: u64,
}
impl ::core::marker::Copy for VDS_PARTITION_NOTIFICATION {}
impl ::core::clone::Clone for VDS_PARTITION_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct VDS_PATH_ID {
    pub ullSourceId: u64,
    pub ullPathId: u64,
}
impl ::core::marker::Copy for VDS_PATH_ID {}
impl ::core::clone::Clone for VDS_PATH_ID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct VDS_PATH_INFO {
    pub pathId: VDS_PATH_ID,
    pub r#type: VDS_HWPROVIDER_TYPE,
    pub status: VDS_PATH_STATUS,
    pub Anonymous1: VDS_PATH_INFO_0,
    pub Anonymous2: VDS_PATH_INFO_1,
    pub Anonymous3: VDS_PATH_INFO_2,
}
impl ::core::marker::Copy for VDS_PATH_INFO {}
impl ::core::clone::Clone for VDS_PATH_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union VDS_PATH_INFO_0 {
    pub controllerPortId: ::windows_sys::core::GUID,
    pub targetPortalId: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for VDS_PATH_INFO_0 {}
impl ::core::clone::Clone for VDS_PATH_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union VDS_PATH_INFO_1 {
    pub hbaPortId: ::windows_sys::core::GUID,
    pub initiatorAdapterId: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for VDS_PATH_INFO_1 {}
impl ::core::clone::Clone for VDS_PATH_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union VDS_PATH_INFO_2 {
    pub pHbaPortProp: *mut VDS_HBAPORT_PROP,
    pub pInitiatorPortalIpAddr: *mut VDS_IPADDRESS,
}
impl ::core::marker::Copy for VDS_PATH_INFO_2 {}
impl ::core::clone::Clone for VDS_PATH_INFO_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_PATH_POLICY {
    pub pathId: VDS_PATH_ID,
    pub bPrimaryPath: super::super::Foundation::BOOL,
    pub ulWeight: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_PATH_POLICY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_PATH_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
pub const VDS_MPS_UNKNOWN: i32 = 0i32;
pub const VDS_MPS_ONLINE: i32 = 1i32;
pub const VDS_MPS_FAILED: i32 = 5i32;
pub const VDS_MPS_STANDBY: i32 = 7i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_POOL_ATTRIBUTES {
    pub ullAttributeMask: u64,
    pub raidType: VDS_RAID_TYPE,
    pub busType: VDS_STORAGE_BUS_TYPE,
    pub pwszIntendedUsage: super::super::Foundation::PWSTR,
    pub bSpinDown: super::super::Foundation::BOOL,
    pub bIsThinProvisioned: super::super::Foundation::BOOL,
    pub ullProvisionedSpace: u64,
    pub bNoSinglePointOfFailure: super::super::Foundation::BOOL,
    pub ulDataRedundancyMax: u32,
    pub ulDataRedundancyMin: u32,
    pub ulDataRedundancyDefault: u32,
    pub ulPackageRedundancyMax: u32,
    pub ulPackageRedundancyMin: u32,
    pub ulPackageRedundancyDefault: u32,
    pub ulStripeSize: u32,
    pub ulStripeSizeMax: u32,
    pub ulStripeSizeMin: u32,
    pub ulDefaultStripeSize: u32,
    pub ulNumberOfColumns: u32,
    pub ulNumberOfColumnsMax: u32,
    pub ulNumberOfColumnsMin: u32,
    pub ulDefaultNumberofColumns: u32,
    pub ulDataAvailabilityHint: u32,
    pub ulAccessRandomnessHint: u32,
    pub ulAccessDirectionHint: u32,
    pub ulAccessSizeHint: u32,
    pub ulAccessLatencyHint: u32,
    pub ulAccessBandwidthWeightHint: u32,
    pub ulStorageCostHint: u32,
    pub ulStorageEfficiencyHint: u32,
    pub ulNumOfCustomAttributes: u32,
    pub pPoolCustomAttributes: *mut VDS_POOL_CUSTOM_ATTRIBUTES,
    pub bReserved1: super::super::Foundation::BOOL,
    pub bReserved2: super::super::Foundation::BOOL,
    pub ulReserved1: u32,
    pub ulReserved2: u32,
    pub ullReserved1: u64,
    pub ullReserved2: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_POOL_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_POOL_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
pub const VDS_POOL_ATTRIB_ACCS_BDW_WT_HINT: i32 = 16777216i32;
pub const VDS_POOL_ATTRIB_ACCS_DIR_HINT: i32 = 2097152i32;
pub const VDS_POOL_ATTRIB_ACCS_LTNCY_HINT: i32 = 8388608i32;
pub const VDS_POOL_ATTRIB_ACCS_RNDM_HINT: i32 = 1048576i32;
pub const VDS_POOL_ATTRIB_ACCS_SIZE_HINT: i32 = 4194304i32;
pub const VDS_POOL_ATTRIB_ALLOW_SPINDOWN: i32 = 4i32;
pub const VDS_POOL_ATTRIB_BUSTYPE: i32 = 2i32;
pub const VDS_POOL_ATTRIB_CUSTOM_ATTRIB: i32 = 134217728i32;
pub const VDS_POOL_ATTRIB_DATA_AVL_HINT: i32 = 524288i32;
pub const VDS_POOL_ATTRIB_DATA_RDNCY_DEF: i32 = 128i32;
pub const VDS_POOL_ATTRIB_DATA_RDNCY_MAX: i32 = 32i32;
pub const VDS_POOL_ATTRIB_DATA_RDNCY_MIN: i32 = 64i32;
pub const VDS_POOL_ATTRIB_NO_SINGLE_POF: i32 = 16i32;
pub const VDS_POOL_ATTRIB_NUM_CLMNS: i32 = 32768i32;
pub const VDS_POOL_ATTRIB_NUM_CLMNS_DEF: i32 = 262144i32;
pub const VDS_POOL_ATTRIB_NUM_CLMNS_MAX: i32 = 65536i32;
pub const VDS_POOL_ATTRIB_NUM_CLMNS_MIN: i32 = 131072i32;
pub const VDS_POOL_ATTRIB_PKG_RDNCY_DEF: i32 = 1024i32;
pub const VDS_POOL_ATTRIB_PKG_RDNCY_MAX: i32 = 256i32;
pub const VDS_POOL_ATTRIB_PKG_RDNCY_MIN: i32 = 512i32;
pub const VDS_POOL_ATTRIB_RAIDTYPE: i32 = 1i32;
pub const VDS_POOL_ATTRIB_STOR_COST_HINT: i32 = 33554432i32;
pub const VDS_POOL_ATTRIB_STOR_EFFCY_HINT: i32 = 67108864i32;
pub const VDS_POOL_ATTRIB_STRIPE_SIZE: i32 = 2048i32;
pub const VDS_POOL_ATTRIB_STRIPE_SIZE_DEF: i32 = 16384i32;
pub const VDS_POOL_ATTRIB_STRIPE_SIZE_MAX: i32 = 4096i32;
pub const VDS_POOL_ATTRIB_STRIPE_SIZE_MIN: i32 = 8192i32;
pub const VDS_POOL_ATTRIB_THIN_PROVISION: i32 = 8i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_POOL_CUSTOM_ATTRIBUTES {
    pub pwszName: super::super::Foundation::PWSTR,
    pub pwszValue: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_POOL_CUSTOM_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_POOL_CUSTOM_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct VDS_PORTAL_GROUP_NOTIFICATION {
    pub ulEvent: u32,
    pub portalGroupId: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for VDS_PORTAL_GROUP_NOTIFICATION {}
impl ::core::clone::Clone for VDS_PORTAL_GROUP_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct VDS_PORTAL_NOTIFICATION {
    pub ulEvent: u32,
    pub portalId: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for VDS_PORTAL_NOTIFICATION {}
impl ::core::clone::Clone for VDS_PORTAL_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct VDS_PORT_NOTIFICATION {
    pub ulEvent: VDS_NF_PORT,
    pub portId: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for VDS_PORT_NOTIFICATION {}
impl ::core::clone::Clone for VDS_PORT_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_PORT_PROP {
    pub id: ::windows_sys::core::GUID,
    pub pwszFriendlyName: super::super::Foundation::PWSTR,
    pub pwszIdentification: super::super::Foundation::PWSTR,
    pub status: VDS_PORT_STATUS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_PORT_PROP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_PORT_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
pub const VDS_PRS_UNKNOWN: i32 = 0i32;
pub const VDS_PRS_ONLINE: i32 = 1i32;
pub const VDS_PRS_NOT_READY: i32 = 2i32;
pub const VDS_PRS_OFFLINE: i32 = 4i32;
pub const VDS_PRS_FAILED: i32 = 5i32;
pub const VDS_PRS_REMOVED: i32 = 8i32;
pub const VDS_PF_DYNAMIC: i32 = 1i32;
pub const VDS_PF_INTERNAL_HARDWARE_PROVIDER: i32 = 2i32;
pub const VDS_PF_ONE_DISK_ONLY_PER_PACK: i32 = 4i32;
pub const VDS_PF_ONE_PACK_ONLINE_ONLY: i32 = 8i32;
pub const VDS_PF_VOLUME_SPACE_MUST_BE_CONTIGUOUS: i32 = 16i32;
pub const VDS_PF_SUPPORT_DYNAMIC: i32 = -2147483648i32;
pub const VDS_PF_SUPPORT_FAULT_TOLERANT: i32 = 1073741824i32;
pub const VDS_PF_SUPPORT_DYNAMIC_1394: i32 = 536870912i32;
pub const VDS_PF_SUPPORT_MIRROR: i32 = 32i32;
pub const VDS_PF_SUPPORT_RAID5: i32 = 64i32;
pub const VDS_LBF_FAILOVER: i32 = 1i32;
pub const VDS_LBF_ROUND_ROBIN: i32 = 2i32;
pub const VDS_LBF_ROUND_ROBIN_WITH_SUBSET: i32 = 4i32;
pub const VDS_LBF_DYN_LEAST_QUEUE_DEPTH: i32 = 8i32;
pub const VDS_LBF_WEIGHTED_PATHS: i32 = 16i32;
pub const VDS_LBF_LEAST_BLOCKS: i32 = 32i32;
pub const VDS_LBF_VENDOR_SPECIFIC: i32 = 64i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_PROVIDER_PROP {
    pub id: ::windows_sys::core::GUID,
    pub pwszName: super::super::Foundation::PWSTR,
    pub guidVersionId: ::windows_sys::core::GUID,
    pub pwszVersion: super::super::Foundation::PWSTR,
    pub r#type: VDS_PROVIDER_TYPE,
    pub ulFlags: u32,
    pub ulStripeSizeFlags: u32,
    pub sRebuildPriority: i16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_PROVIDER_PROP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_PROVIDER_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
pub const VDS_PT_UNKNOWN: i32 = 0i32;
pub const VDS_PT_SOFTWARE: i32 = 1i32;
pub const VDS_PT_HARDWARE: i32 = 2i32;
pub const VDS_PT_VIRTUALDISK: i32 = 3i32;
pub const VDS_PT_MAX: i32 = 4i32;
pub const VDS_RT_UNKNOWN: i32 = 0i32;
pub const VDS_RT_RAID0: i32 = 10i32;
pub const VDS_RT_RAID1: i32 = 11i32;
pub const VDS_RT_RAID2: i32 = 12i32;
pub const VDS_RT_RAID3: i32 = 13i32;
pub const VDS_RT_RAID4: i32 = 14i32;
pub const VDS_RT_RAID5: i32 = 15i32;
pub const VDS_RT_RAID6: i32 = 16i32;
pub const VDS_RT_RAID01: i32 = 17i32;
pub const VDS_RT_RAID03: i32 = 18i32;
pub const VDS_RT_RAID05: i32 = 19i32;
pub const VDS_RT_RAID10: i32 = 20i32;
pub const VDS_RT_RAID15: i32 = 21i32;
pub const VDS_RT_RAID30: i32 = 22i32;
pub const VDS_RT_RAID50: i32 = 23i32;
pub const VDS_RT_RAID51: i32 = 24i32;
pub const VDS_RT_RAID53: i32 = 25i32;
pub const VDS_RT_RAID60: i32 = 26i32;
pub const VDS_RT_RAID61: i32 = 27i32;
pub const VDS_REBUILD_PRIORITY_MAX: u32 = 16u32;
pub const VDS_REBUILD_PRIORITY_MIN: u32 = 0u32;
pub const VDS_RA_UNKNOWN: i32 = 0i32;
pub const VDS_RA_REFRESH: i32 = 1i32;
pub const VDS_RA_RESTART: i32 = 2i32;
#[repr(C)]
pub struct VDS_SERVICE_NOTIFICATION {
    pub ulEvent: u32,
    pub action: VDS_RECOVER_ACTION,
}
impl ::core::marker::Copy for VDS_SERVICE_NOTIFICATION {}
impl ::core::clone::Clone for VDS_SERVICE_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub const VDSBusTypeUnknown: i32 = 0i32;
pub const VDSBusTypeScsi: i32 = 1i32;
pub const VDSBusTypeAtapi: i32 = 2i32;
pub const VDSBusTypeAta: i32 = 3i32;
pub const VDSBusType1394: i32 = 4i32;
pub const VDSBusTypeSsa: i32 = 5i32;
pub const VDSBusTypeFibre: i32 = 6i32;
pub const VDSBusTypeUsb: i32 = 7i32;
pub const VDSBusTypeRAID: i32 = 8i32;
pub const VDSBusTypeiScsi: i32 = 9i32;
pub const VDSBusTypeSas: i32 = 10i32;
pub const VDSBusTypeSata: i32 = 11i32;
pub const VDSBusTypeSd: i32 = 12i32;
pub const VDSBusTypeMmc: i32 = 13i32;
pub const VDSBusTypeMax: i32 = 14i32;
pub const VDSBusTypeVirtual: i32 = 14i32;
pub const VDSBusTypeFileBackedVirtual: i32 = 15i32;
pub const VDSBusTypeSpaces: i32 = 16i32;
pub const VDSBusTypeNVMe: i32 = 17i32;
pub const VDSBusTypeScm: i32 = 18i32;
pub const VDSBusTypeUfs: i32 = 19i32;
pub const VDSBusTypeMaxReserved: i32 = 127i32;
#[repr(C)]
pub struct VDS_STORAGE_DEVICE_ID_DESCRIPTOR {
    pub m_version: u32,
    pub m_cIdentifiers: u32,
    pub m_rgIdentifiers: *mut VDS_STORAGE_IDENTIFIER,
}
impl ::core::marker::Copy for VDS_STORAGE_DEVICE_ID_DESCRIPTOR {}
impl ::core::clone::Clone for VDS_STORAGE_DEVICE_ID_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct VDS_STORAGE_IDENTIFIER {
    pub m_CodeSet: VDS_STORAGE_IDENTIFIER_CODE_SET,
    pub m_Type: VDS_STORAGE_IDENTIFIER_TYPE,
    pub m_cbIdentifier: u32,
    pub m_rgbIdentifier: *mut u8,
}
impl ::core::marker::Copy for VDS_STORAGE_IDENTIFIER {}
impl ::core::clone::Clone for VDS_STORAGE_IDENTIFIER {
    fn clone(&self) -> Self {
        *self
    }
}
pub const VDSStorageIdCodeSetReserved: i32 = 0i32;
pub const VDSStorageIdCodeSetBinary: i32 = 1i32;
pub const VDSStorageIdCodeSetAscii: i32 = 2i32;
pub const VDSStorageIdCodeSetUtf8: i32 = 3i32;
pub const VDSStorageIdTypeVendorSpecific: i32 = 0i32;
pub const VDSStorageIdTypeVendorId: i32 = 1i32;
pub const VDSStorageIdTypeEUI64: i32 = 2i32;
pub const VDSStorageIdTypeFCPHName: i32 = 3i32;
pub const VDSStorageIdTypePortRelative: i32 = 4i32;
pub const VDSStorageIdTypeTargetPortGroup: i32 = 5i32;
pub const VDSStorageIdTypeLogicalUnitGroup: i32 = 6i32;
pub const VDSStorageIdTypeMD5LogicalUnitIdentifier: i32 = 7i32;
pub const VDSStorageIdTypeScsiNameString: i32 = 8i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_STORAGE_POOL_DRIVE_EXTENT {
    pub id: ::windows_sys::core::GUID,
    pub ullSize: u64,
    pub bUsed: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_STORAGE_POOL_DRIVE_EXTENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_STORAGE_POOL_DRIVE_EXTENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_STORAGE_POOL_PROP {
    pub id: ::windows_sys::core::GUID,
    pub status: VDS_STORAGE_POOL_STATUS,
    pub health: VDS_HEALTH,
    pub r#type: VDS_STORAGE_POOL_TYPE,
    pub pwszName: super::super::Foundation::PWSTR,
    pub pwszDescription: super::super::Foundation::PWSTR,
    pub ullTotalConsumedSpace: u64,
    pub ullTotalManagedSpace: u64,
    pub ullRemainingFreeSpace: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_STORAGE_POOL_PROP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_STORAGE_POOL_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
pub const VDS_SPS_UNKNOWN: i32 = 0i32;
pub const VDS_SPS_ONLINE: i32 = 1i32;
pub const VDS_SPS_NOT_READY: i32 = 2i32;
pub const VDS_SPS_OFFLINE: i32 = 4i32;
pub const VDS_SPT_UNKNOWN: i32 = 0i32;
pub const VDS_SPT_PRIMORDIAL: i32 = 1i32;
pub const VDS_SPT_CONCRETE: i32 = 2i32;
pub const VDS_SF_LUN_MASKING_CAPABLE: i32 = 1i32;
pub const VDS_SF_LUN_PLEXING_CAPABLE: i32 = 2i32;
pub const VDS_SF_LUN_REMAPPING_CAPABLE: i32 = 4i32;
pub const VDS_SF_DRIVE_EXTENT_CAPABLE: i32 = 8i32;
pub const VDS_SF_HARDWARE_CHECKSUM_CAPABLE: i32 = 16i32;
pub const VDS_SF_RADIUS_CAPABLE: i32 = 32i32;
pub const VDS_SF_READ_BACK_VERIFY_CAPABLE: i32 = 64i32;
pub const VDS_SF_WRITE_THROUGH_CACHING_CAPABLE: i32 = 128i32;
pub const VDS_SF_SUPPORTS_FAULT_TOLERANT_LUNS: i32 = 512i32;
pub const VDS_SF_SUPPORTS_NON_FAULT_TOLERANT_LUNS: i32 = 1024i32;
pub const VDS_SF_SUPPORTS_SIMPLE_LUNS: i32 = 2048i32;
pub const VDS_SF_SUPPORTS_SPAN_LUNS: i32 = 4096i32;
pub const VDS_SF_SUPPORTS_STRIPE_LUNS: i32 = 8192i32;
pub const VDS_SF_SUPPORTS_MIRROR_LUNS: i32 = 16384i32;
pub const VDS_SF_SUPPORTS_PARITY_LUNS: i32 = 32768i32;
pub const VDS_SF_SUPPORTS_AUTH_CHAP: i32 = 65536i32;
pub const VDS_SF_SUPPORTS_AUTH_MUTUAL_CHAP: i32 = 131072i32;
pub const VDS_SF_SUPPORTS_SIMPLE_TARGET_CONFIG: i32 = 262144i32;
pub const VDS_SF_SUPPORTS_LUN_NUMBER: i32 = 524288i32;
pub const VDS_SF_SUPPORTS_MIRRORED_CACHE: i32 = 1048576i32;
pub const VDS_SF_READ_CACHING_CAPABLE: i32 = 2097152i32;
pub const VDS_SF_WRITE_CACHING_CAPABLE: i32 = 4194304i32;
pub const VDS_SF_MEDIA_SCAN_CAPABLE: i32 = 8388608i32;
pub const VDS_SF_CONSISTENCY_CHECK_CAPABLE: i32 = 16777216i32;
#[repr(C)]
pub struct VDS_SUB_SYSTEM_NOTIFICATION {
    pub ulEvent: u32,
    pub subSystemId: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for VDS_SUB_SYSTEM_NOTIFICATION {}
impl ::core::clone::Clone for VDS_SUB_SYSTEM_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_SUB_SYSTEM_PROP {
    pub id: ::windows_sys::core::GUID,
    pub pwszFriendlyName: super::super::Foundation::PWSTR,
    pub pwszIdentification: super::super::Foundation::PWSTR,
    pub ulFlags: u32,
    pub ulStripeSizeFlags: u32,
    pub status: VDS_SUB_SYSTEM_STATUS,
    pub health: VDS_HEALTH,
    pub sNumberOfInternalBuses: i16,
    pub sMaxNumberOfSlotsEachBus: i16,
    pub sMaxNumberOfControllers: i16,
    pub sRebuildPriority: i16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_SUB_SYSTEM_PROP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_SUB_SYSTEM_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_SUB_SYSTEM_PROP2 {
    pub id: ::windows_sys::core::GUID,
    pub pwszFriendlyName: super::super::Foundation::PWSTR,
    pub pwszIdentification: super::super::Foundation::PWSTR,
    pub ulFlags: u32,
    pub ulStripeSizeFlags: u32,
    pub ulSupportedRaidTypeFlags: u32,
    pub status: VDS_SUB_SYSTEM_STATUS,
    pub health: VDS_HEALTH,
    pub sNumberOfInternalBuses: i16,
    pub sMaxNumberOfSlotsEachBus: i16,
    pub sMaxNumberOfControllers: i16,
    pub sRebuildPriority: i16,
    pub ulNumberOfEnclosures: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_SUB_SYSTEM_PROP2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_SUB_SYSTEM_PROP2 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const VDS_SSS_UNKNOWN: i32 = 0i32;
pub const VDS_SSS_ONLINE: i32 = 1i32;
pub const VDS_SSS_NOT_READY: i32 = 2i32;
pub const VDS_SSS_OFFLINE: i32 = 4i32;
pub const VDS_SSS_FAILED: i32 = 5i32;
pub const VDS_SSS_PARTIALLY_MANAGED: i32 = 9i32;
pub const VDS_SF_SUPPORTS_RAID2_LUNS: i32 = 1i32;
pub const VDS_SF_SUPPORTS_RAID3_LUNS: i32 = 2i32;
pub const VDS_SF_SUPPORTS_RAID4_LUNS: i32 = 4i32;
pub const VDS_SF_SUPPORTS_RAID5_LUNS: i32 = 8i32;
pub const VDS_SF_SUPPORTS_RAID6_LUNS: i32 = 16i32;
pub const VDS_SF_SUPPORTS_RAID01_LUNS: i32 = 32i32;
pub const VDS_SF_SUPPORTS_RAID03_LUNS: i32 = 64i32;
pub const VDS_SF_SUPPORTS_RAID05_LUNS: i32 = 128i32;
pub const VDS_SF_SUPPORTS_RAID10_LUNS: i32 = 256i32;
pub const VDS_SF_SUPPORTS_RAID15_LUNS: i32 = 512i32;
pub const VDS_SF_SUPPORTS_RAID30_LUNS: i32 = 1024i32;
pub const VDS_SF_SUPPORTS_RAID50_LUNS: i32 = 2048i32;
pub const VDS_SF_SUPPORTS_RAID51_LUNS: i32 = 4096i32;
pub const VDS_SF_SUPPORTS_RAID53_LUNS: i32 = 8192i32;
pub const VDS_SF_SUPPORTS_RAID60_LUNS: i32 = 16384i32;
pub const VDS_SF_SUPPORTS_RAID61_LUNS: i32 = 32768i32;
pub const VDS_S_ACCESS_PATH_NOT_DELETED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(279108i32 as _);
pub const VDS_S_ALREADY_EXISTS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(272148i32 as _);
pub const VDS_S_BOOT_PARTITION_NUMBER_CHANGE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(271414i32 as _);
pub const VDS_S_DEFAULT_PLEX_MEMBER_IDS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(271640i32 as _);
pub const VDS_S_DISK_DISMOUNT_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(272393i32 as _);
pub const VDS_S_DISK_IS_MISSING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(271624i32 as _);
pub const VDS_S_DISK_MOUNT_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(272392i32 as _);
pub const VDS_S_DISK_PARTIALLY_CLEANED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(271386i32 as _);
pub const VDS_S_DISMOUNT_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(271735i32 as _);
pub const VDS_S_EXTEND_FILE_SYSTEM_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(271461i32 as _);
pub const VDS_S_FS_LOCK: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(271747i32 as _);
pub const VDS_S_GPT_BOOT_MIRRORED_TO_MBR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147212183i32 as _);
pub const VDS_S_IA64_BOOT_MIRRORED_TO_MBR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(271450i32 as _);
pub const VDS_S_IN_PROGRESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(271437i32 as _);
pub const VDS_S_ISCSI_LOGIN_ALREAD_EXISTS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(272386i32 as _);
pub const VDS_S_ISCSI_PERSISTENT_LOGIN_MAY_NOT_BE_REMOVED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(272385i32 as _);
pub const VDS_S_ISCSI_SESSION_NOT_FOUND_PERSISTENT_LOGIN_REMOVED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(272384i32 as _);
pub const VDS_S_MBR_BOOT_MIRRORED_TO_GPT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(271463i32 as _);
pub const VDS_S_NAME_TRUNCATED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(272128i32 as _);
pub const VDS_S_NONCONFORMANT_PARTITION_INFO: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(271626i32 as _);
pub const VDS_S_NO_NOTIFICATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(271639i32 as _);
pub const VDS_S_PLEX_NOT_LOADED_TO_CACHE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(271755i32 as _);
pub const VDS_S_PROPERTIES_INCOMPLETE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(272149i32 as _);
pub const VDS_S_PROVIDER_ERROR_LOADING_CACHE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(271393i32 as _);
pub const VDS_S_REMOUNT_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(271736i32 as _);
pub const VDS_S_RESYNC_NOTIFICATION_TASK_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(271738i32 as _);
pub const VDS_S_STATUSES_INCOMPLETELY_SET: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(272130i32 as _);
pub const VDS_S_SYSTEM_PARTITION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(271630i32 as _);
pub const VDS_S_UNABLE_TO_GET_GPT_ATTRIBUTES: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(271451i32 as _);
pub const VDS_S_UPDATE_BOOTFILE_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(271412i32 as _);
pub const VDS_S_VOLUME_COMPRESS_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(271427i32 as _);
pub const VDS_S_VSS_FLUSH_AND_HOLD_WRITES: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(271745i32 as _);
pub const VDS_S_VSS_RELEASE_WRITES: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(271746i32 as _);
pub const VDS_S_WINPE_BOOTENTRY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(271758i32 as _);
#[repr(C)]
pub struct VDS_TARGET_NOTIFICATION {
    pub ulEvent: u32,
    pub targetId: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for VDS_TARGET_NOTIFICATION {}
impl ::core::clone::Clone for VDS_TARGET_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub const VDS_TS_UNKNOWN: i32 = 0i32;
pub const VDS_TS_STABLE: i32 = 1i32;
pub const VDS_TS_EXTENDING: i32 = 2i32;
pub const VDS_TS_SHRINKING: i32 = 3i32;
pub const VDS_TS_RECONFIGING: i32 = 4i32;
pub const VDS_TS_RESTRIPING: i32 = 5i32;
pub const VDS_VSF_1_0: i32 = 1i32;
pub const VDS_VSF_1_1: i32 = 2i32;
pub const VDS_VSF_2_0: i32 = 4i32;
pub const VDS_VSF_2_1: i32 = 8i32;
pub const VDS_VSF_3_0: i32 = 16i32;
#[repr(C)]
pub struct VDS_VOLUME_NOTIFICATION {
    pub ulEvent: u32,
    pub volumeId: ::windows_sys::core::GUID,
    pub plexId: ::windows_sys::core::GUID,
    pub ulPercentCompleted: u32,
}
impl ::core::marker::Copy for VDS_VOLUME_NOTIFICATION {}
impl ::core::clone::Clone for VDS_VOLUME_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct VDS_WWN {
    pub rguchWwn: [u8; 8],
}
impl ::core::marker::Copy for VDS_WWN {}
impl ::core::clone::Clone for VDS_WWN {
    fn clone(&self) -> Self {
        *self
    }
}
pub const VER_VDS_LUN_INFORMATION: u32 = 1u32;

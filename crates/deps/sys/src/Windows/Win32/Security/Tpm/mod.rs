#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ITpmVirtualSmartCardManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITpmVirtualSmartCardManager {}
impl ::core::clone::Clone for ITpmVirtualSmartCardManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITpmVirtualSmartCardManager2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITpmVirtualSmartCardManager2 {}
impl ::core::clone::Clone for ITpmVirtualSmartCardManager2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITpmVirtualSmartCardManager3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITpmVirtualSmartCardManager3 {}
impl ::core::clone::Clone for ITpmVirtualSmartCardManager3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITpmVirtualSmartCardManagerStatusCallback(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITpmVirtualSmartCardManagerStatusCallback {}
impl ::core::clone::Clone for ITpmVirtualSmartCardManagerStatusCallback {
    fn clone(&self) -> Self {
        *self
    }
}
pub const RemoteTpmVirtualSmartCardManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 355377832, data2: 28892, data3: 19545, data4: [139, 42, 50, 170, 60, 160, 220, 172] };
pub const TPMVSCMGR_ERROR_IMPERSONATION: i32 = 0i32;
pub const TPMVSCMGR_ERROR_PIN_COMPLEXITY: i32 = 1i32;
pub const TPMVSCMGR_ERROR_READER_COUNT_LIMIT: i32 = 2i32;
pub const TPMVSCMGR_ERROR_TERMINAL_SERVICES_SESSION: i32 = 3i32;
pub const TPMVSCMGR_ERROR_VTPMSMARTCARD_INITIALIZE: i32 = 4i32;
pub const TPMVSCMGR_ERROR_VTPMSMARTCARD_CREATE: i32 = 5i32;
pub const TPMVSCMGR_ERROR_VTPMSMARTCARD_DESTROY: i32 = 6i32;
pub const TPMVSCMGR_ERROR_VGIDSSIMULATOR_INITIALIZE: i32 = 7i32;
pub const TPMVSCMGR_ERROR_VGIDSSIMULATOR_CREATE: i32 = 8i32;
pub const TPMVSCMGR_ERROR_VGIDSSIMULATOR_DESTROY: i32 = 9i32;
pub const TPMVSCMGR_ERROR_VGIDSSIMULATOR_WRITE_PROPERTY: i32 = 10i32;
pub const TPMVSCMGR_ERROR_VGIDSSIMULATOR_READ_PROPERTY: i32 = 11i32;
pub const TPMVSCMGR_ERROR_VREADER_INITIALIZE: i32 = 12i32;
pub const TPMVSCMGR_ERROR_VREADER_CREATE: i32 = 13i32;
pub const TPMVSCMGR_ERROR_VREADER_DESTROY: i32 = 14i32;
pub const TPMVSCMGR_ERROR_GENERATE_LOCATE_READER: i32 = 15i32;
pub const TPMVSCMGR_ERROR_GENERATE_FILESYSTEM: i32 = 16i32;
pub const TPMVSCMGR_ERROR_CARD_CREATE: i32 = 17i32;
pub const TPMVSCMGR_ERROR_CARD_DESTROY: i32 = 18i32;
pub const TPMVSCMGR_STATUS_VTPMSMARTCARD_INITIALIZING: i32 = 0i32;
pub const TPMVSCMGR_STATUS_VTPMSMARTCARD_CREATING: i32 = 1i32;
pub const TPMVSCMGR_STATUS_VTPMSMARTCARD_DESTROYING: i32 = 2i32;
pub const TPMVSCMGR_STATUS_VGIDSSIMULATOR_INITIALIZING: i32 = 3i32;
pub const TPMVSCMGR_STATUS_VGIDSSIMULATOR_CREATING: i32 = 4i32;
pub const TPMVSCMGR_STATUS_VGIDSSIMULATOR_DESTROYING: i32 = 5i32;
pub const TPMVSCMGR_STATUS_VREADER_INITIALIZING: i32 = 6i32;
pub const TPMVSCMGR_STATUS_VREADER_CREATING: i32 = 7i32;
pub const TPMVSCMGR_STATUS_VREADER_DESTROYING: i32 = 8i32;
pub const TPMVSCMGR_STATUS_GENERATE_WAITING: i32 = 9i32;
pub const TPMVSCMGR_STATUS_GENERATE_AUTHENTICATING: i32 = 10i32;
pub const TPMVSCMGR_STATUS_GENERATE_RUNNING: i32 = 11i32;
pub const TPMVSCMGR_STATUS_CARD_CREATED: i32 = 12i32;
pub const TPMVSCMGR_STATUS_CARD_DESTROYED: i32 = 13i32;
pub const TPMVSC_ATTESTATION_NONE: i32 = 0i32;
pub const TPMVSC_ATTESTATION_AIK_ONLY: i32 = 1i32;
pub const TPMVSC_ATTESTATION_AIK_AND_CERTIFICATE: i32 = 2i32;
pub const TPMVSC_DEFAULT_ADMIN_ALGORITHM_ID: u32 = 130u32;
pub const TpmVirtualSmartCardManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 379686534,
    data2: 32622,
    data3: 19488,
    data4: [173, 137, 79, 252, 13, 183, 169, 106],
};

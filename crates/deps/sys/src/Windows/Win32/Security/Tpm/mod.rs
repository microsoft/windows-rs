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
#[repr(transparent)]
pub struct TPMVSCMGR_ERROR(pub i32);
pub const TPMVSCMGR_ERROR_IMPERSONATION: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(0i32);
pub const TPMVSCMGR_ERROR_PIN_COMPLEXITY: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(1i32);
pub const TPMVSCMGR_ERROR_READER_COUNT_LIMIT: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(2i32);
pub const TPMVSCMGR_ERROR_TERMINAL_SERVICES_SESSION: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(3i32);
pub const TPMVSCMGR_ERROR_VTPMSMARTCARD_INITIALIZE: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(4i32);
pub const TPMVSCMGR_ERROR_VTPMSMARTCARD_CREATE: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(5i32);
pub const TPMVSCMGR_ERROR_VTPMSMARTCARD_DESTROY: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(6i32);
pub const TPMVSCMGR_ERROR_VGIDSSIMULATOR_INITIALIZE: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(7i32);
pub const TPMVSCMGR_ERROR_VGIDSSIMULATOR_CREATE: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(8i32);
pub const TPMVSCMGR_ERROR_VGIDSSIMULATOR_DESTROY: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(9i32);
pub const TPMVSCMGR_ERROR_VGIDSSIMULATOR_WRITE_PROPERTY: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(10i32);
pub const TPMVSCMGR_ERROR_VGIDSSIMULATOR_READ_PROPERTY: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(11i32);
pub const TPMVSCMGR_ERROR_VREADER_INITIALIZE: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(12i32);
pub const TPMVSCMGR_ERROR_VREADER_CREATE: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(13i32);
pub const TPMVSCMGR_ERROR_VREADER_DESTROY: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(14i32);
pub const TPMVSCMGR_ERROR_GENERATE_LOCATE_READER: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(15i32);
pub const TPMVSCMGR_ERROR_GENERATE_FILESYSTEM: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(16i32);
pub const TPMVSCMGR_ERROR_CARD_CREATE: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(17i32);
pub const TPMVSCMGR_ERROR_CARD_DESTROY: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(18i32);
impl ::core::marker::Copy for TPMVSCMGR_ERROR {}
impl ::core::clone::Clone for TPMVSCMGR_ERROR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TPMVSCMGR_STATUS(pub i32);
pub const TPMVSCMGR_STATUS_VTPMSMARTCARD_INITIALIZING: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(0i32);
pub const TPMVSCMGR_STATUS_VTPMSMARTCARD_CREATING: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(1i32);
pub const TPMVSCMGR_STATUS_VTPMSMARTCARD_DESTROYING: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(2i32);
pub const TPMVSCMGR_STATUS_VGIDSSIMULATOR_INITIALIZING: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(3i32);
pub const TPMVSCMGR_STATUS_VGIDSSIMULATOR_CREATING: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(4i32);
pub const TPMVSCMGR_STATUS_VGIDSSIMULATOR_DESTROYING: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(5i32);
pub const TPMVSCMGR_STATUS_VREADER_INITIALIZING: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(6i32);
pub const TPMVSCMGR_STATUS_VREADER_CREATING: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(7i32);
pub const TPMVSCMGR_STATUS_VREADER_DESTROYING: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(8i32);
pub const TPMVSCMGR_STATUS_GENERATE_WAITING: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(9i32);
pub const TPMVSCMGR_STATUS_GENERATE_AUTHENTICATING: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(10i32);
pub const TPMVSCMGR_STATUS_GENERATE_RUNNING: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(11i32);
pub const TPMVSCMGR_STATUS_CARD_CREATED: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(12i32);
pub const TPMVSCMGR_STATUS_CARD_DESTROYED: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(13i32);
impl ::core::marker::Copy for TPMVSCMGR_STATUS {}
impl ::core::clone::Clone for TPMVSCMGR_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TPMVSC_ATTESTATION_TYPE(pub i32);
pub const TPMVSC_ATTESTATION_NONE: TPMVSC_ATTESTATION_TYPE = TPMVSC_ATTESTATION_TYPE(0i32);
pub const TPMVSC_ATTESTATION_AIK_ONLY: TPMVSC_ATTESTATION_TYPE = TPMVSC_ATTESTATION_TYPE(1i32);
pub const TPMVSC_ATTESTATION_AIK_AND_CERTIFICATE: TPMVSC_ATTESTATION_TYPE = TPMVSC_ATTESTATION_TYPE(2i32);
impl ::core::marker::Copy for TPMVSC_ATTESTATION_TYPE {}
impl ::core::clone::Clone for TPMVSC_ATTESTATION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const TPMVSC_DEFAULT_ADMIN_ALGORITHM_ID: u32 = 130u32;
pub const TpmVirtualSmartCardManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 379686534,
    data2: 32622,
    data3: 19488,
    data4: [173, 137, 79, 252, 13, 183, 169, 106],
};

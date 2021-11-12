#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct ITpmVirtualSmartCardManager(i32);
pub struct ITpmVirtualSmartCardManager2(i32);
pub struct ITpmVirtualSmartCardManager3(i32);
pub struct ITpmVirtualSmartCardManagerStatusCallback(i32);
pub struct RemoteTpmVirtualSmartCardManager(i32);
pub struct TPMVSCMGR_ERROR(i32);
pub struct TPMVSCMGR_STATUS(i32);
pub struct TPMVSC_ATTESTATION_TYPE(i32);
#[doc = "*Required features: `Win32_Security_Tpm`*"]
pub const TPMVSC_DEFAULT_ADMIN_ALGORITHM_ID: u32 = 130u32;
pub struct TpmVirtualSmartCardManager(i32);

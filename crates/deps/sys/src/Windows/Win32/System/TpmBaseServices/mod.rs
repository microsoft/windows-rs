#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDeviceID(pbwindowsaik: *mut u8, cbwindowsaik: u32, pcbresult: *mut u32, pfprotectedbytpm: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDeviceIDString(pszwindowsaik: super::super::Foundation::PWSTR, cchwindowsaik: u32, pcchresult: *mut u32, pfprotectedbytpm: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    pub fn Tbsi_Context_Create(pcontextparams: *const TBS_CONTEXT_PARAMS, phcontext: *mut *mut ::core::ffi::c_void) -> u32;
    pub fn Tbsi_Create_Windows_Key(keyhandle: u32) -> u32;
    pub fn Tbsi_GetDeviceInfo(size: u32, info: *mut ::core::ffi::c_void) -> u32;
    pub fn Tbsi_Get_OwnerAuth(hcontext: *const ::core::ffi::c_void, ownerauthtype: u32, poutputbuf: *mut u8, poutputbuflen: *mut u32) -> u32;
    pub fn Tbsi_Get_TCG_Log(hcontext: *const ::core::ffi::c_void, poutputbuf: *mut u8, poutputbuflen: *mut u32) -> u32;
    pub fn Tbsi_Get_TCG_Log_Ex(logtype: u32, pboutput: *mut u8, pcboutput: *mut u32) -> u32;
    pub fn Tbsi_Physical_Presence_Command(hcontext: *const ::core::ffi::c_void, pabinput: *const u8, cbinput: u32, paboutput: *mut u8, pcboutput: *mut u32) -> u32;
    pub fn Tbsi_Revoke_Attestation() -> u32;
    pub fn Tbsip_Cancel_Commands(hcontext: *const ::core::ffi::c_void) -> u32;
    pub fn Tbsip_Context_Close(hcontext: *const ::core::ffi::c_void) -> u32;
    pub fn Tbsip_Submit_Command(hcontext: *const ::core::ffi::c_void, locality: TBS_COMMAND_LOCALITY, priority: TBS_COMMAND_PRIORITY, pabcommand: *const u8, cbcommand: u32, pabresult: *mut u8, pcbresult: *mut u32) -> u32;
}
#[repr(C)]
pub struct TBS_COMMAND_LOCALITY(i32);
#[repr(C)]
pub struct TBS_COMMAND_PRIORITY(i32);
#[repr(C)]
pub struct TBS_CONTEXT_PARAMS(i32);
#[repr(C)]
pub struct TBS_CONTEXT_PARAMS2(i32);
pub const TBS_CONTEXT_VERSION_ONE: u32 = 1u32;
pub const TBS_CONTEXT_VERSION_TWO: u32 = 2u32;
pub const TBS_OWNERAUTH_TYPE_ADMIN: u32 = 2u32;
pub const TBS_OWNERAUTH_TYPE_ENDORSEMENT: u32 = 4u32;
pub const TBS_OWNERAUTH_TYPE_ENDORSEMENT_20: u32 = 12u32;
pub const TBS_OWNERAUTH_TYPE_FULL: u32 = 1u32;
pub const TBS_OWNERAUTH_TYPE_STORAGE_20: u32 = 13u32;
pub const TBS_OWNERAUTH_TYPE_USER: u32 = 3u32;
pub const TBS_SUCCESS: u32 = 0u32;
pub const TBS_TCGLOG_DRTM_BOOT: u32 = 4u32;
pub const TBS_TCGLOG_DRTM_CURRENT: u32 = 1u32;
pub const TBS_TCGLOG_DRTM_RESUME: u32 = 5u32;
pub const TBS_TCGLOG_SRTM_BOOT: u32 = 2u32;
pub const TBS_TCGLOG_SRTM_CURRENT: u32 = 0u32;
pub const TBS_TCGLOG_SRTM_RESUME: u32 = 3u32;
#[repr(C)]
pub struct TPM_DEVICE_INFO(i32);
pub const TPM_IFTYPE_1: u32 = 1u32;
pub const TPM_IFTYPE_EMULATOR: u32 = 4u32;
pub const TPM_IFTYPE_HW: u32 = 3u32;
pub const TPM_IFTYPE_SPB: u32 = 5u32;
pub const TPM_IFTYPE_TRUSTZONE: u32 = 2u32;
pub const TPM_IFTYPE_UNKNOWN: u32 = 0u32;
pub const TPM_VERSION_12: u32 = 1u32;
pub const TPM_VERSION_20: u32 = 2u32;
pub const TPM_VERSION_UNKNOWN: u32 = 0u32;
pub const TPM_WNF_INFO_CLEAR_SUCCESSFUL: u32 = 1u32;
pub const TPM_WNF_INFO_NO_REBOOT_REQUIRED: u32 = 1u32;
pub const TPM_WNF_INFO_OWNERSHIP_SUCCESSFUL: u32 = 2u32;
#[repr(C)]
pub struct tdTPM_WNF_PROVISIONING(i32);

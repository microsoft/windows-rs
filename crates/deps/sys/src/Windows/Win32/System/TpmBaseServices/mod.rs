#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_TpmBaseServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDeviceID(pbwindowsaik: *mut u8, cbwindowsaik: u32, pcbresult: *mut u32, pfprotectedbytpm: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_TpmBaseServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDeviceIDString(pszwindowsaik: super::super::Foundation::PWSTR, cchwindowsaik: u32, pcchresult: *mut u32, pfprotectedbytpm: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_TpmBaseServices`*"]
    pub fn Tbsi_Context_Create(pcontextparams: *const TBS_CONTEXT_PARAMS, phcontext: *mut *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_System_TpmBaseServices`*"]
    pub fn Tbsi_Create_Windows_Key(keyhandle: u32) -> u32;
    #[doc = "*Required features: `Win32_System_TpmBaseServices`*"]
    pub fn Tbsi_GetDeviceInfo(size: u32, info: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_System_TpmBaseServices`*"]
    pub fn Tbsi_Get_OwnerAuth(hcontext: *const ::core::ffi::c_void, ownerauthtype: u32, poutputbuf: *mut u8, poutputbuflen: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_TpmBaseServices`*"]
    pub fn Tbsi_Get_TCG_Log(hcontext: *const ::core::ffi::c_void, poutputbuf: *mut u8, poutputbuflen: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_TpmBaseServices`*"]
    pub fn Tbsi_Get_TCG_Log_Ex(logtype: u32, pboutput: *mut u8, pcboutput: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_TpmBaseServices`*"]
    pub fn Tbsi_Physical_Presence_Command(hcontext: *const ::core::ffi::c_void, pabinput: *const u8, cbinput: u32, paboutput: *mut u8, pcboutput: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_TpmBaseServices`*"]
    pub fn Tbsi_Revoke_Attestation() -> u32;
    #[doc = "*Required features: `Win32_System_TpmBaseServices`*"]
    pub fn Tbsip_Cancel_Commands(hcontext: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_System_TpmBaseServices`*"]
    pub fn Tbsip_Context_Close(hcontext: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_System_TpmBaseServices`*"]
    pub fn Tbsip_Submit_Command(hcontext: *const ::core::ffi::c_void, locality: TBS_COMMAND_LOCALITY, priority: TBS_COMMAND_PRIORITY, pabcommand: *const u8, cbcommand: u32, pabresult: *mut u8, pcbresult: *mut u32) -> u32;
}

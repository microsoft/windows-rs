#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_TpmBaseServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDeviceID();
    #[doc = "*Required features: `Win32_System_TpmBaseServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDeviceIDString();
    #[doc = "*Required features: `Win32_System_TpmBaseServices`*"]
    pub fn Tbsi_Context_Create();
    #[doc = "*Required features: `Win32_System_TpmBaseServices`*"]
    pub fn Tbsi_Create_Windows_Key();
    #[doc = "*Required features: `Win32_System_TpmBaseServices`*"]
    pub fn Tbsi_GetDeviceInfo();
    #[doc = "*Required features: `Win32_System_TpmBaseServices`*"]
    pub fn Tbsi_Get_OwnerAuth();
    #[doc = "*Required features: `Win32_System_TpmBaseServices`*"]
    pub fn Tbsi_Get_TCG_Log();
    #[doc = "*Required features: `Win32_System_TpmBaseServices`*"]
    pub fn Tbsi_Get_TCG_Log_Ex();
    #[doc = "*Required features: `Win32_System_TpmBaseServices`*"]
    pub fn Tbsi_Physical_Presence_Command();
    #[doc = "*Required features: `Win32_System_TpmBaseServices`*"]
    pub fn Tbsi_Revoke_Attestation();
    #[doc = "*Required features: `Win32_System_TpmBaseServices`*"]
    pub fn Tbsip_Cancel_Commands();
    #[doc = "*Required features: `Win32_System_TpmBaseServices`*"]
    pub fn Tbsip_Context_Close();
    #[doc = "*Required features: `Win32_System_TpmBaseServices`*"]
    pub fn Tbsip_Submit_Command();
}

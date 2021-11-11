#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub fn CloseIMsgSession();
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_AddressBook`*"]
    #[cfg(feature = "Win32_System_AddressBook")]
    pub fn GetAttribIMsgOnIStg();
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub fn MapStorageSCode();
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_AddressBook`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_System_AddressBook", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn OpenIMsgOnIStg();
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn OpenIMsgSession();
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_AddressBook`*"]
    #[cfg(feature = "Win32_System_AddressBook")]
    pub fn SetAttribIMsgOnIStg();
}

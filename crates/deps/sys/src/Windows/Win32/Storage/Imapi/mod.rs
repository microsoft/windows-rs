#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub fn CloseIMsgSession(lpmsgsess: *mut _MSGSESS);
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_AddressBook`*"]
    #[cfg(feature = "Win32_System_AddressBook")]
    pub fn GetAttribIMsgOnIStg(lpobject: *mut ::core::ffi::c_void, lpproptagarray: *mut super::super::System::AddressBook::SPropTagArray, lpppropattrarray: *mut *mut SPropAttrArray) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_Imapi`*"]
    pub fn MapStorageSCode(stgscode: i32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_AddressBook`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_System_AddressBook", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn OpenIMsgOnIStg(lpmsgsess: *mut _MSGSESS, lpallocatebuffer: ::windows::runtime::RawPtr, lpallocatemore: ::windows::runtime::RawPtr, lpfreebuffer: ::windows::runtime::RawPtr, lpmalloc: ::windows::runtime::RawPtr, lpmapisup: *mut ::core::ffi::c_void, lpstg: ::windows::runtime::RawPtr, lpfmsgcallrelease: *mut ::windows::runtime::RawPtr, ulcallerdata: u32, ulflags: u32, lppmsg: *mut ::windows::runtime::RawPtr) -> i32;
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn OpenIMsgSession(lpmalloc: ::windows::runtime::RawPtr, ulflags: u32, lppmsgsess: *mut *mut _MSGSESS) -> i32;
    #[doc = "*Required features: `Win32_Storage_Imapi`, `Win32_System_AddressBook`*"]
    #[cfg(feature = "Win32_System_AddressBook")]
    pub fn SetAttribIMsgOnIStg(lpobject: *mut ::core::ffi::c_void, lpproptags: *mut super::super::System::AddressBook::SPropTagArray, lppropattrs: *mut SPropAttrArray, lpppropproblems: *mut *mut super::super::System::AddressBook::SPropProblemArray) -> ::windows::runtime::HRESULT;
}

#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Foundation`*"]
    pub fn CloseHandle();
    #[doc = "*Required features: `Win32_Foundation`*"]
    pub fn CompareObjectHandles();
    #[doc = "*Required features: `Win32_Foundation`*"]
    pub fn DuplicateHandle();
    #[doc = "*Required features: `Win32_Foundation`*"]
    pub fn GetHandleInformation();
    #[doc = "*Required features: `Win32_Foundation`*"]
    pub fn GetLastError();
    #[doc = "*Required features: `Win32_Foundation`*"]
    pub fn RtlNtStatusToDosError();
    #[doc = "*Required features: `Win32_Foundation`*"]
    pub fn SetHandleInformation();
    #[doc = "*Required features: `Win32_Foundation`*"]
    pub fn SetLastError();
    #[doc = "*Required features: `Win32_Foundation`*"]
    pub fn SetLastErrorEx();
    #[doc = "*Required features: `Win32_Foundation`*"]
    pub fn SysAddRefString();
    #[doc = "*Required features: `Win32_Foundation`*"]
    pub fn SysAllocString();
    #[doc = "*Required features: `Win32_Foundation`*"]
    pub fn SysAllocStringByteLen();
    #[doc = "*Required features: `Win32_Foundation`*"]
    pub fn SysAllocStringLen();
    #[doc = "*Required features: `Win32_Foundation`*"]
    pub fn SysFreeString();
    #[doc = "*Required features: `Win32_Foundation`*"]
    pub fn SysReAllocString();
    #[doc = "*Required features: `Win32_Foundation`*"]
    pub fn SysReAllocStringLen();
    #[doc = "*Required features: `Win32_Foundation`*"]
    pub fn SysReleaseString();
    #[doc = "*Required features: `Win32_Foundation`*"]
    pub fn SysStringByteLen();
    #[doc = "*Required features: `Win32_Foundation`*"]
    pub fn SysStringLen();
}

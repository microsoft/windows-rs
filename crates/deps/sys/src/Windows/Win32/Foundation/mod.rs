#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Foundation`*"]
    pub fn CloseHandle(hobject: HANDLE) -> BOOL;
    #[doc = "*Required features: `Win32_Foundation`*"]
    pub fn CompareObjectHandles(hfirstobjecthandle: HANDLE, hsecondobjecthandle: HANDLE) -> BOOL;
    #[doc = "*Required features: `Win32_Foundation`*"]
    pub fn DuplicateHandle(hsourceprocesshandle: HANDLE, hsourcehandle: HANDLE, htargetprocesshandle: HANDLE, lptargethandle: *mut HANDLE, dwdesiredaccess: u32, binherithandle: BOOL, dwoptions: DUPLICATE_HANDLE_OPTIONS) -> BOOL;
    #[doc = "*Required features: `Win32_Foundation`*"]
    pub fn GetHandleInformation(hobject: HANDLE, lpdwflags: *mut u32) -> BOOL;
    #[doc = "*Required features: `Win32_Foundation`*"]
    pub fn GetLastError() -> WIN32_ERROR;
    #[doc = "*Required features: `Win32_Foundation`*"]
    pub fn RtlNtStatusToDosError(status: NTSTATUS) -> u32;
    #[doc = "*Required features: `Win32_Foundation`*"]
    pub fn SetHandleInformation(hobject: HANDLE, dwmask: u32, dwflags: HANDLE_FLAGS) -> BOOL;
    #[doc = "*Required features: `Win32_Foundation`*"]
    pub fn SetLastError(dwerrcode: WIN32_ERROR);
    #[doc = "*Required features: `Win32_Foundation`*"]
    pub fn SetLastErrorEx(dwerrcode: WIN32_ERROR, dwtype: u32);
    #[doc = "*Required features: `Win32_Foundation`*"]
    pub fn SysAddRefString(bstrstring: ::core::mem::ManuallyDrop<BSTR>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Foundation`*"]
    pub fn SysAllocString(psz: PWSTR) -> BSTR;
    #[doc = "*Required features: `Win32_Foundation`*"]
    pub fn SysAllocStringByteLen(psz: PSTR, len: u32) -> BSTR;
    #[doc = "*Required features: `Win32_Foundation`*"]
    pub fn SysAllocStringLen(strin: PWSTR, ui: u32) -> BSTR;
    #[doc = "*Required features: `Win32_Foundation`*"]
    pub fn SysFreeString(bstrstring: ::core::mem::ManuallyDrop<BSTR>);
    #[doc = "*Required features: `Win32_Foundation`*"]
    pub fn SysReAllocString(pbstr: *mut ::core::mem::ManuallyDrop<BSTR>, psz: PWSTR) -> i32;
    #[doc = "*Required features: `Win32_Foundation`*"]
    pub fn SysReAllocStringLen(pbstr: *mut ::core::mem::ManuallyDrop<BSTR>, psz: PWSTR, len: u32) -> i32;
    #[doc = "*Required features: `Win32_Foundation`*"]
    pub fn SysReleaseString(bstrstring: ::core::mem::ManuallyDrop<BSTR>);
    #[doc = "*Required features: `Win32_Foundation`*"]
    pub fn SysStringByteLen(bstr: ::core::mem::ManuallyDrop<BSTR>) -> u32;
    #[doc = "*Required features: `Win32_Foundation`*"]
    pub fn SysStringLen(pbstr: ::core::mem::ManuallyDrop<BSTR>) -> u32;
}

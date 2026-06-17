windows_link::link!("kernel32.dll" "system" fn FormatMessageW(dwflags : FORMAT_MESSAGE_OPTIONS, lpsource : *const core::ffi::c_void, dwmessageid : u32, dwlanguageid : u32, lpbuffer : PWSTR, nsize : u32, arguments : *const *const i8) -> u32);
windows_link::link!("oleaut32.dll" "system" fn GetErrorInfo(dwreserved : u32, pperrinfo : *mut *mut core::ffi::c_void) -> HRESULT);
windows_link::link!("kernel32.dll" "system" fn GetLastError() -> WIN32_ERROR);
windows_link::link!("kernel32.dll" "system" fn GetProcessHeap() -> HANDLE);
windows_link::link!("kernel32.dll" "system" fn HeapFree(hheap : HANDLE, dwflags : HEAP_FLAGS, lpmem : *const core::ffi::c_void) -> BOOL);
windows_link::link!("kernel32.dll" "system" fn LoadLibraryExA(lplibfilename : PCSTR, hfile : HANDLE, dwflags : LOAD_LIBRARY_FLAGS) -> HMODULE);
windows_link::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoOriginateErrorW(error : HRESULT, cchmax : u32, message : PCWSTR) -> BOOL);
windows_link::link!("oleaut32.dll" "system" fn SetErrorInfo(dwreserved : u32, perrinfo : *mut core::ffi::c_void) -> HRESULT);
windows_link::link!("oleaut32.dll" "system" fn SysFreeString(bstrstring : BSTR));
windows_link::link!("oleaut32.dll" "system" fn SysStringLen(pbstr : BSTR) -> u32);
pub type BOOL = i32;
pub type BSTR = *const u16;
pub const ERROR_INVALID_DATA: WIN32_ERROR = 13;
pub const ERROR_NO_UNICODE_TRANSLATION: WIN32_ERROR = 1113;
pub const E_UNEXPECTED: HRESULT = 0x8000FFFF_u32 as _;
pub const FORMAT_MESSAGE_ALLOCATE_BUFFER: FORMAT_MESSAGE_OPTIONS = 256;
pub const FORMAT_MESSAGE_FROM_HMODULE: FORMAT_MESSAGE_OPTIONS = 2048;
pub const FORMAT_MESSAGE_FROM_SYSTEM: FORMAT_MESSAGE_OPTIONS = 4096;
pub const FORMAT_MESSAGE_IGNORE_INSERTS: FORMAT_MESSAGE_OPTIONS = 512;
pub type FORMAT_MESSAGE_OPTIONS = u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct GUID {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}
pub type HANDLE = *mut core::ffi::c_void;
pub type HEAP_FLAGS = u32;
pub type HINSTANCE = *mut core::ffi::c_void;
pub type HMODULE = *mut core::ffi::c_void;
pub type HRESULT = i32;
pub const IID_IErrorInfo: GUID = GUID {
    data1: 0x1cf2b120,
    data2: 0x547d,
    data3: 0x101b,
    data4: [142, 101, 8, 0, 43, 43, 209, 25],
};
#[repr(C)]
pub struct IErrorInfo_Vtbl {
    pub base__: IUnknown_Vtbl,
    pub GetGUID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GUID) -> HRESULT,
    pub GetSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BSTR) -> HRESULT,
    pub GetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BSTR) -> HRESULT,
    pub GetHelpFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BSTR) -> HRESULT,
    pub GetHelpContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> HRESULT,
}
pub const IID_IRestrictedErrorInfo: GUID = GUID {
    data1: 0x82ba7092,
    data2: 0x4c88,
    data3: 0x427d,
    data4: [167, 188, 22, 221, 147, 254, 182, 126],
};
#[repr(C)]
pub struct IRestrictedErrorInfo_Vtbl {
    pub base__: IUnknown_Vtbl,
    pub GetErrorDetails: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut BSTR,
        *mut HRESULT,
        *mut BSTR,
        *mut BSTR,
    ) -> HRESULT,
    pub GetReference: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BSTR) -> HRESULT,
}
pub const IID_IUnknown: GUID = GUID {
    data1: 0x00000000,
    data2: 0x0000,
    data3: 0x0000,
    data4: [192, 0, 0, 0, 0, 0, 0, 70],
};
#[repr(C)]
pub struct IUnknown_Vtbl {
    pub QueryInterface: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        iid: *const GUID,
        interface: *mut *mut core::ffi::c_void,
    ) -> HRESULT,
    pub AddRef: unsafe extern "system" fn(this: *mut core::ffi::c_void) -> u32,
    pub Release: unsafe extern "system" fn(this: *mut core::ffi::c_void) -> u32,
}
pub type LOAD_LIBRARY_FLAGS = u32;
pub const LOAD_LIBRARY_SEARCH_DEFAULT_DIRS: LOAD_LIBRARY_FLAGS = 4096;
pub type PCSTR = *const u8;
pub type PCWSTR = *const u16;
pub type PWSTR = *mut u16;
pub type WIN32_ERROR = u32;

windows_link::link!("kernel32.dll" "system" fn CloseHandle(hobject : HANDLE) -> BOOL);
windows_link::link!("ktmw32.dll" "system" fn CommitTransaction(transactionhandle : HANDLE) -> BOOL);
windows_link::link!("ktmw32.dll" "system" fn CreateTransaction(lptransactionattributes : *mut SECURITY_ATTRIBUTES, uow : *mut GUID, createoptions : u32, isolationlevel : u32, isolationflags : u32, timeout : u32, description : PCWSTR) -> HANDLE);
windows_link::link!("kernel32.dll" "system" fn GetProcessHeap() -> HANDLE);
windows_link::link!("kernel32.dll" "system" fn HeapAlloc(hheap : HANDLE, dwflags : u32, dwbytes : usize) -> *mut core::ffi::c_void);
windows_link::link!("kernel32.dll" "system" fn HeapFree(hheap : HANDLE, dwflags : u32, lpmem : *mut core::ffi::c_void) -> BOOL);
windows_link::link!("advapi32.dll" "system" fn RegCloseKey(hkey : HKEY) -> LSTATUS);
windows_link::link!("advapi32.dll" "system" fn RegCreateKeyExW(hkey : HKEY, lpsubkey : PCWSTR, reserved : u32, lpclass : PCWSTR, dwoptions : u32, samdesired : REGSAM, lpsecurityattributes : *const SECURITY_ATTRIBUTES, phkresult : *mut HKEY, lpdwdisposition : *mut u32) -> LSTATUS);
windows_link::link!("advapi32.dll" "system" fn RegCreateKeyTransactedW(hkey : HKEY, lpsubkey : PCWSTR, reserved : u32, lpclass : PCWSTR, dwoptions : u32, samdesired : REGSAM, lpsecurityattributes : *const SECURITY_ATTRIBUTES, phkresult : *mut HKEY, lpdwdisposition : *mut u32, htransaction : HANDLE, pextendedparemeter : *const core::ffi::c_void) -> LSTATUS);
windows_link::link!("advapi32.dll" "system" fn RegDeleteTreeW(hkey : HKEY, lpsubkey : PCWSTR) -> LSTATUS);
windows_link::link!("advapi32.dll" "system" fn RegDeleteValueW(hkey : HKEY, lpvaluename : PCWSTR) -> LSTATUS);
windows_link::link!("advapi32.dll" "system" fn RegEnumKeyExW(hkey : HKEY, dwindex : u32, lpname : PWSTR, lpcchname : *mut u32, lpreserved : *const u32, lpclass : PWSTR, lpcchclass : *mut u32, lpftlastwritetime : *mut FILETIME) -> LSTATUS);
windows_link::link!("advapi32.dll" "system" fn RegEnumValueW(hkey : HKEY, dwindex : u32, lpvaluename : PWSTR, lpcchvaluename : *mut u32, lpreserved : *const u32, lptype : *mut u32, lpdata : *mut u8, lpcbdata : *mut u32) -> LSTATUS);
windows_link::link!("advapi32.dll" "system" fn RegOpenKeyExW(hkey : HKEY, lpsubkey : PCWSTR, uloptions : u32, samdesired : REGSAM, phkresult : *mut HKEY) -> LSTATUS);
windows_link::link!("advapi32.dll" "system" fn RegOpenKeyTransactedW(hkey : HKEY, lpsubkey : PCWSTR, uloptions : u32, samdesired : REGSAM, phkresult : *mut HKEY, htransaction : HANDLE, pextendedparemeter : *const core::ffi::c_void) -> LSTATUS);
windows_link::link!("advapi32.dll" "system" fn RegQueryInfoKeyW(hkey : HKEY, lpclass : PWSTR, lpcchclass : *mut u32, lpreserved : *const u32, lpcsubkeys : *mut u32, lpcbmaxsubkeylen : *mut u32, lpcbmaxclasslen : *mut u32, lpcvalues : *mut u32, lpcbmaxvaluenamelen : *mut u32, lpcbmaxvaluelen : *mut u32, lpcbsecuritydescriptor : *mut u32, lpftlastwritetime : *mut FILETIME) -> LSTATUS);
windows_link::link!("advapi32.dll" "system" fn RegQueryValueExW(hkey : HKEY, lpvaluename : PCWSTR, lpreserved : *const u32, lptype : *mut u32, lpdata : *mut u8, lpcbdata : *mut u32) -> LSTATUS);
windows_link::link!("advapi32.dll" "system" fn RegRenameKey(hkey : HKEY, lpsubkeyname : PCWSTR, lpnewkeyname : PCWSTR) -> LSTATUS);
windows_link::link!("advapi32.dll" "system" fn RegSetValueExW(hkey : HKEY, lpvaluename : PCWSTR, reserved : u32, dwtype : u32, lpdata : *const u8, cbdata : u32) -> LSTATUS);
pub type ACCESS_MASK = u32;
pub type BOOL = i32;
pub const ERROR_INVALID_DATA: u32 = 13;
pub const ERROR_NO_MORE_ITEMS: u32 = 259;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct FILETIME {
    pub dwLowDateTime: u32,
    pub dwHighDateTime: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GUID {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}
pub type HANDLE = *mut core::ffi::c_void;
pub type HKEY = *mut core::ffi::c_void;
pub const HKEY_CLASSES_ROOT: HKEY = -2147483648 as _;
pub const HKEY_CURRENT_CONFIG: HKEY = -2147483643 as _;
pub const HKEY_CURRENT_USER: HKEY = -2147483647 as _;
pub const HKEY_LOCAL_MACHINE: HKEY = -2147483646 as _;
pub const HKEY_USERS: HKEY = -2147483645 as _;
pub const INVALID_HANDLE_VALUE: HANDLE = -1 as _;
pub const KEY_READ: u32 = 131097;
pub const KEY_WOW64_32KEY: u32 = 512;
pub const KEY_WOW64_64KEY: u32 = 256;
pub const KEY_WRITE: u32 = 131078;
pub type LSTATUS = i32;
pub type PCWSTR = *const u16;
pub type PWSTR = *mut u16;
pub type REGSAM = ACCESS_MASK;
pub const REG_BINARY: u32 = 3;
pub const REG_DWORD: u32 = 4;
pub const REG_EXPAND_SZ: u32 = 2;
pub const REG_MULTI_SZ: u32 = 7;
pub const REG_OPTION_NON_VOLATILE: u32 = 0;
pub const REG_OPTION_VOLATILE: u32 = 1;
pub const REG_QWORD: u32 = 11;
pub const REG_SZ: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SECURITY_ATTRIBUTES {
    pub nLength: u32,
    pub lpSecurityDescriptor: *mut core::ffi::c_void,
    pub bInheritHandle: BOOL,
}
impl Default for SECURITY_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

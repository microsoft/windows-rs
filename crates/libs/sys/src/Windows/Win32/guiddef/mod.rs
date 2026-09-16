windows_link::link!("ole32.dll" "C" fn IsEqualGUID(rguid1 : *const windows_sys::core::GUID, rguid2 : *const windows_sys::core::GUID) -> i32);
pub type LPCGUID = *const windows_sys::core::GUID;
pub type LPCLSID = *mut windows_sys::core::GUID;
pub type LPFMTID = *mut windows_sys::core::GUID;
pub type LPGUID = *mut windows_sys::core::GUID;
pub type LPIID = *mut windows_sys::core::GUID;

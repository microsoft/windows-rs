#[inline]
pub unsafe fn IsEqualGUID(rguid1: *const windows_core::GUID, rguid2: *const windows_core::GUID) -> i32 {
    windows_core::link!("ole32.dll" "C" fn IsEqualGUID(rguid1 : *const windows_core::GUID, rguid2 : *const windows_core::GUID) -> i32);
    unsafe { IsEqualGUID(rguid1, rguid2) }
}
pub type LPCGUID = *const windows_core::GUID;
pub type LPCLSID = *mut windows_core::GUID;
pub type LPFMTID = *mut windows_core::GUID;
pub type LPGUID = *mut windows_core::GUID;
pub type LPIID = *mut windows_core::GUID;

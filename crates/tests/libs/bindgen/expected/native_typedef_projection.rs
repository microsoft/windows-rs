#[inline]
pub unsafe fn Create<T>() -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_core::link!("test.dll" "system" fn Create(riid : REFIID, object : PPVOID) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe {
        Create(&T::IID, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn GetTime(result: LPSYSTEMTIME) {
    windows_core::link!("test.dll" "system" fn GetTime(result : LPSYSTEMTIME));
    unsafe { GetTime(result as _) }
}
#[inline]
pub unsafe fn GetValue(result: LPDWORD) -> windows_core::HRESULT {
    windows_core::link!("test.dll" "system" fn GetValue(result : LPDWORD) -> windows_core::HRESULT);
    unsafe { GetValue(result as _) }
}
pub type DWORD = u32;
pub type IID = windows_core::GUID;
pub type LPDWORD = *mut DWORD;
pub type LPSYSTEMTIME = *mut SYSTEMTIME;
pub type PPVOID = *mut *mut core::ffi::c_void;
pub type REFIID = *const IID;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SYSTEMTIME {
    pub value: u32,
}

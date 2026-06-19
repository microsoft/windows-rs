#[inline]
pub unsafe fn BoolFunction(value: u32) -> BOOL {
    windows_core::link!("test.dll" "system" fn BoolFunction(value : u32) -> BOOL);
    unsafe { BoolFunction(value) }
}
#[inline]
pub unsafe fn HresultFunction(value: u32) -> windows_core::Result<()> {
    windows_core::link!("test.dll" "system" fn HresultFunction(value : u32) -> windows_core::HRESULT);
    unsafe { HresultFunction(value).ok() }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct BOOL(pub i32);

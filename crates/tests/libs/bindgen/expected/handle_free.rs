#[inline]
pub unsafe fn CloseHandle(handle: Handle) -> i32 {
    windows_core::link!("test.dll" "system" fn CloseHandle(handle : Handle) -> i32);
    unsafe { CloseHandle(handle) }
}
#[inline]
pub unsafe fn OpenHandle(name: u32) -> Handle {
    windows_core::link!("test.dll" "system" fn OpenHandle(name : u32) -> Handle);
    unsafe { OpenHandle(name) }
}
pub type Handle = isize;
pub const INVALID: Handle = -1;

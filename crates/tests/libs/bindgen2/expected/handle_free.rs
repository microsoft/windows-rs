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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Handle(pub isize);
pub const INVALID: Handle = Handle(-1 as _);

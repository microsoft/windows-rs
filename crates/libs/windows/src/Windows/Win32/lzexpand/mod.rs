#[inline]
pub unsafe fn CopyLZFile(hfsource: i32, hfdest: i32) -> i32 {
    windows_core::link!("lz32.dll" "system" fn CopyLZFile(hfsource : i32, hfdest : i32) -> i32);
    unsafe { CopyLZFile(hfsource, hfdest) }
}
#[inline]
pub unsafe fn GetExpandedNameA<P0>(lpszsource: P0, lpszbuffer: windows_core::PSTR) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("lz32.dll" "system" fn GetExpandedNameA(lpszsource : windows_core::PCSTR, lpszbuffer : windows_core::PSTR) -> i32);
    unsafe { GetExpandedNameA(lpszsource.param().abi(), lpszbuffer) }
}
#[inline]
pub unsafe fn GetExpandedNameW<P0>(lpszsource: P0, lpszbuffer: windows_core::PWSTR) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("lz32.dll" "system" fn GetExpandedNameW(lpszsource : windows_core::PCWSTR, lpszbuffer : windows_core::PWSTR) -> i32);
    unsafe { GetExpandedNameW(lpszsource.param().abi(), lpszbuffer) }
}
#[inline]
pub unsafe fn LZClose(hfile: i32) {
    windows_core::link!("lz32.dll" "system" fn LZClose(hfile : i32));
    unsafe { LZClose(hfile) }
}
#[inline]
pub unsafe fn LZCopy(hfsource: i32, hfdest: i32) -> i32 {
    windows_core::link!("lz32.dll" "system" fn LZCopy(hfsource : i32, hfdest : i32) -> i32);
    unsafe { LZCopy(hfsource, hfdest) }
}
#[inline]
pub unsafe fn LZDone() {
    windows_core::link!("lz32.dll" "system" fn LZDone());
    unsafe { LZDone() }
}
#[inline]
pub unsafe fn LZInit(hfsource: i32) -> i32 {
    windows_core::link!("lz32.dll" "system" fn LZInit(hfsource : i32) -> i32);
    unsafe { LZInit(hfsource) }
}
#[cfg(feature = "winbase")]
#[inline]
pub unsafe fn LZOpenFileA<P0>(lpfilename: P0, lpreopenbuf: *mut super::OFSTRUCT, wstyle: u16) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("lz32.dll" "system" fn LZOpenFileA(lpfilename : windows_core::PCSTR, lpreopenbuf : *mut super::OFSTRUCT, wstyle : u16) -> i32);
    unsafe { LZOpenFileA(lpfilename.param().abi(), lpreopenbuf as _, wstyle) }
}
#[cfg(feature = "winbase")]
#[inline]
pub unsafe fn LZOpenFileW<P0>(lpfilename: P0, lpreopenbuf: *mut super::OFSTRUCT, wstyle: u16) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("lz32.dll" "system" fn LZOpenFileW(lpfilename : windows_core::PCWSTR, lpreopenbuf : *mut super::OFSTRUCT, wstyle : u16) -> i32);
    unsafe { LZOpenFileW(lpfilename.param().abi(), lpreopenbuf as _, wstyle) }
}
#[inline]
pub unsafe fn LZRead(hfile: i32, lpbuffer: &mut [u8]) -> i32 {
    windows_core::link!("lz32.dll" "system" fn LZRead(hfile : i32, lpbuffer : *mut i8, cbread : i32) -> i32);
    unsafe { LZRead(hfile, core::mem::transmute(lpbuffer.as_mut_ptr()), lpbuffer.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn LZSeek(hfile: i32, loffset: i32, iorigin: i32) -> i32 {
    windows_core::link!("lz32.dll" "system" fn LZSeek(hfile : i32, loffset : i32, iorigin : i32) -> i32);
    unsafe { LZSeek(hfile, loffset, iorigin) }
}
#[inline]
pub unsafe fn LZStart() -> i32 {
    windows_core::link!("lz32.dll" "system" fn LZStart() -> i32);
    unsafe { LZStart() }
}
pub const LZERROR_BADINHANDLE: i32 = -1;
pub const LZERROR_BADOUTHANDLE: i32 = -2;
pub const LZERROR_BADVALUE: i32 = -7;
pub const LZERROR_GLOBALLOC: i32 = -5;
pub const LZERROR_GLOBLOCK: i32 = -6;
pub const LZERROR_READ: i32 = -3;
pub const LZERROR_UNKNOWNALG: i32 = -8;
pub const LZERROR_WRITE: i32 = -4;

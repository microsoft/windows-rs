#[must_use]
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CopyLZFile(hfsource: super::INT, hfdest: super::INT) -> super::LONG {
    windows_core::link!("lz32.dll" "system" fn CopyLZFile(hfsource : super::INT, hfdest : super::INT) -> super::LONG);
    unsafe { CopyLZFile(hfsource, hfdest) }
}
#[must_use]
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetExpandedNameA<P0>(lpszsource: P0, lpszbuffer: windows_core::PSTR) -> super::INT
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("lz32.dll" "system" fn GetExpandedNameA(lpszsource : windows_core::PCSTR, lpszbuffer : windows_core::PSTR) -> super::INT);
    unsafe { GetExpandedNameA(lpszsource.param().abi(), lpszbuffer) }
}
#[must_use]
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetExpandedNameW<P0>(lpszsource: P0, lpszbuffer: windows_core::PWSTR) -> super::INT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("lz32.dll" "system" fn GetExpandedNameW(lpszsource : windows_core::PCWSTR, lpszbuffer : windows_core::PWSTR) -> super::INT);
    unsafe { GetExpandedNameW(lpszsource.param().abi(), lpszbuffer) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn LZClose(hfile: super::INT) {
    windows_core::link!("lz32.dll" "system" fn LZClose(hfile : super::INT));
    unsafe { LZClose(hfile) }
}
#[must_use]
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn LZCopy(hfsource: super::INT, hfdest: super::INT) -> super::LONG {
    windows_core::link!("lz32.dll" "system" fn LZCopy(hfsource : super::INT, hfdest : super::INT) -> super::LONG);
    unsafe { LZCopy(hfsource, hfdest) }
}
#[inline]
pub unsafe fn LZDone() {
    windows_core::link!("lz32.dll" "system" fn LZDone());
    unsafe { LZDone() }
}
#[must_use]
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn LZInit(hfsource: super::INT) -> super::INT {
    windows_core::link!("lz32.dll" "system" fn LZInit(hfsource : super::INT) -> super::INT);
    unsafe { LZInit(hfsource) }
}
#[must_use]
#[cfg(all(feature = "shared", feature = "winbase"))]
#[inline]
pub unsafe fn LZOpenFileA<P0>(lpfilename: P0, lpreopenbuf: super::LPOFSTRUCT, wstyle: super::WORD) -> super::INT
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("lz32.dll" "system" fn LZOpenFileA(lpfilename : windows_core::PCSTR, lpreopenbuf : super::LPOFSTRUCT, wstyle : super::WORD) -> super::INT);
    unsafe { LZOpenFileA(lpfilename.param().abi(), lpreopenbuf as _, wstyle) }
}
#[must_use]
#[cfg(all(feature = "shared", feature = "winbase"))]
#[inline]
pub unsafe fn LZOpenFileW<P0>(lpfilename: P0, lpreopenbuf: super::LPOFSTRUCT, wstyle: super::WORD) -> super::INT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("lz32.dll" "system" fn LZOpenFileW(lpfilename : windows_core::PCWSTR, lpreopenbuf : super::LPOFSTRUCT, wstyle : super::WORD) -> super::INT);
    unsafe { LZOpenFileW(lpfilename.param().abi(), lpreopenbuf as _, wstyle) }
}
#[must_use]
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn LZRead(hfile: super::INT, lpbuffer: *mut super::CHAR, cbread: super::INT) -> super::INT {
    windows_core::link!("lz32.dll" "system" fn LZRead(hfile : super::INT, lpbuffer : *mut super::CHAR, cbread : super::INT) -> super::INT);
    unsafe { LZRead(hfile, lpbuffer as _, cbread) }
}
#[must_use]
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn LZSeek(hfile: super::INT, loffset: super::LONG, iorigin: super::INT) -> super::LONG {
    windows_core::link!("lz32.dll" "system" fn LZSeek(hfile : super::INT, loffset : super::LONG, iorigin : super::INT) -> super::LONG);
    unsafe { LZSeek(hfile, loffset, iorigin) }
}
#[must_use]
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn LZStart() -> super::INT {
    windows_core::link!("lz32.dll" "system" fn LZStart() -> super::INT);
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

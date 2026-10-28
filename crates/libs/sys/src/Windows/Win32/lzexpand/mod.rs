#[cfg(feature = "shared")]
windows_link::link!("lz32.dll" "system" fn CopyLZFile(hfsource : super::INT, hfdest : super::INT) -> super::LONG);
#[cfg(feature = "shared")]
windows_link::link!("lz32.dll" "system" fn GetExpandedNameA(lpszsource : windows_sys::core::PCSTR, lpszbuffer : windows_sys::core::PSTR) -> super::INT);
#[cfg(feature = "shared")]
windows_link::link!("lz32.dll" "system" fn GetExpandedNameW(lpszsource : windows_sys::core::PCWSTR, lpszbuffer : windows_sys::core::PWSTR) -> super::INT);
#[cfg(feature = "shared")]
windows_link::link!("lz32.dll" "system" fn LZClose(hfile : super::INT));
#[cfg(feature = "shared")]
windows_link::link!("lz32.dll" "system" fn LZCopy(hfsource : super::INT, hfdest : super::INT) -> super::LONG);
windows_link::link!("lz32.dll" "system" fn LZDone());
#[cfg(feature = "shared")]
windows_link::link!("lz32.dll" "system" fn LZInit(hfsource : super::INT) -> super::INT);
#[cfg(all(feature = "shared", feature = "winbase"))]
windows_link::link!("lz32.dll" "system" fn LZOpenFileA(lpfilename : windows_sys::core::PCSTR, lpreopenbuf : super::LPOFSTRUCT, wstyle : super::WORD) -> super::INT);
#[cfg(all(feature = "shared", feature = "winbase"))]
windows_link::link!("lz32.dll" "system" fn LZOpenFileW(lpfilename : windows_sys::core::PCWSTR, lpreopenbuf : super::LPOFSTRUCT, wstyle : super::WORD) -> super::INT);
#[cfg(feature = "shared")]
windows_link::link!("lz32.dll" "system" fn LZRead(hfile : super::INT, lpbuffer : *mut super::CHAR, cbread : super::INT) -> super::INT);
#[cfg(feature = "shared")]
windows_link::link!("lz32.dll" "system" fn LZSeek(hfile : super::INT, loffset : super::LONG, iorigin : super::INT) -> super::LONG);
#[cfg(feature = "shared")]
windows_link::link!("lz32.dll" "system" fn LZStart() -> super::INT);
pub const LZERROR_BADINHANDLE: i32 = -1;
pub const LZERROR_BADOUTHANDLE: i32 = -2;
pub const LZERROR_BADVALUE: i32 = -7;
pub const LZERROR_GLOBALLOC: i32 = -5;
pub const LZERROR_GLOBLOCK: i32 = -6;
pub const LZERROR_READ: i32 = -3;
pub const LZERROR_UNKNOWNALG: i32 = -8;
pub const LZERROR_WRITE: i32 = -4;

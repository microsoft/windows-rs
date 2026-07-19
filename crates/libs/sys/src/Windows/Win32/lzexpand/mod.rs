windows_link::link!("lz32.dll" "system" fn CopyLZFile(hfsource : i32, hfdest : i32) -> i32);
windows_link::link!("lz32.dll" "system" fn GetExpandedNameA(lpszsource : windows_sys::core::PCSTR, lpszbuffer : windows_sys::core::PSTR) -> i32);
windows_link::link!("lz32.dll" "system" fn GetExpandedNameW(lpszsource : windows_sys::core::PCWSTR, lpszbuffer : windows_sys::core::PWSTR) -> i32);
windows_link::link!("lz32.dll" "system" fn LZClose(hfile : i32));
windows_link::link!("lz32.dll" "system" fn LZCopy(hfsource : i32, hfdest : i32) -> i32);
windows_link::link!("lz32.dll" "system" fn LZDone());
windows_link::link!("lz32.dll" "system" fn LZInit(hfsource : i32) -> i32);
#[cfg(feature = "winbase")]
windows_link::link!("lz32.dll" "system" fn LZOpenFileA(lpfilename : windows_sys::core::PCSTR, lpreopenbuf : *mut super::OFSTRUCT, wstyle : u16) -> i32);
#[cfg(feature = "winbase")]
windows_link::link!("lz32.dll" "system" fn LZOpenFileW(lpfilename : windows_sys::core::PCWSTR, lpreopenbuf : *mut super::OFSTRUCT, wstyle : u16) -> i32);
windows_link::link!("lz32.dll" "system" fn LZRead(hfile : i32, lpbuffer : *mut i8, cbread : i32) -> i32);
windows_link::link!("lz32.dll" "system" fn LZSeek(hfile : i32, loffset : i32, iorigin : i32) -> i32);
windows_link::link!("lz32.dll" "system" fn LZStart() -> i32);
pub const LZERROR_BADINHANDLE: i32 = -1;
pub const LZERROR_BADOUTHANDLE: i32 = -2;
pub const LZERROR_BADVALUE: i32 = -7;
pub const LZERROR_GLOBALLOC: i32 = -5;
pub const LZERROR_GLOBLOCK: i32 = -6;
pub const LZERROR_READ: i32 = -3;
pub const LZERROR_UNKNOWNALG: i32 = -8;
pub const LZERROR_WRITE: i32 = -4;

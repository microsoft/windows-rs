windows_link::link!("kernel32.dll" "system" fn SetComputerNameA(lpcomputername : PCSTR) -> BOOL);
pub type BOOL = i32;
pub type PCSTR = *const u8;

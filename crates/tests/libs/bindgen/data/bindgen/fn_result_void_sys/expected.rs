windows_link::link!("kernel32.dll" "system" fn SetComputerNameA(lpcomputername : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);

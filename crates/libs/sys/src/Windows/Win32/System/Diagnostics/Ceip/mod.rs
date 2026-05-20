pub type CeipIsOptedIn = unsafe extern "system" fn() -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn CeipIsOptedIn() -> windows_sys::core::BOOL);

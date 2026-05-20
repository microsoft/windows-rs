pub type GetTickCount = unsafe extern "system" fn() -> u32;
windows_link::link!("kernel32.dll" "system" fn GetTickCount() -> u32);

pub type GetTickCount = unsafe extern "system" fn() -> u32;
windows_targets::link!("kernel32.dll" "system" fn GetTickCount() -> u32);

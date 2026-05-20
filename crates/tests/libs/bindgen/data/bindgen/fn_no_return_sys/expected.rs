pub type FatalExit = unsafe extern "system" fn(exitcode: i32) -> !;
windows_link::link!("kernel32.dll" "system" fn FatalExit(exitcode : i32) -> !);

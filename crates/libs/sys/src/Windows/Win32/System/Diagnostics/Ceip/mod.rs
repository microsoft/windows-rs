#[cfg(feature = "Win32_Foundation")]
::windows_sys::core::windows_link ! ( "kernel32.dll" ,"system" fn CeipIsOptedIn ( ) -> super::super::super::Foundation:: BOOL );

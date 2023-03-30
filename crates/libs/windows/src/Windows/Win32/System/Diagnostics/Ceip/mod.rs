#[doc = "*Required features: `\"Win32_System_Diagnostics_Ceip\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CeipIsOptedIn() -> super::super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "kernel32.dll""system" fn CeipIsOptedIn ( ) -> super::super::super::Foundation:: BOOL );
    CeipIsOptedIn()
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg_attr(feature = "use_raw_dylib", link(name = "kernel32", kind = "raw-dylib"))]
#[cfg_attr(not(feature = "use_raw_dylib"), link(name = "windows"))]
extern "system" {
    #[doc = "*Required features: 'Win32_System_Diagnostics_Ceip', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CeipIsOptedIn() -> super::super::super::Foundation::BOOL;
}

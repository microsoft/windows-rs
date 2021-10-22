#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn CeipIsOptedIn() -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "kernel32")]
        extern "system" {
            fn CeipIsOptedIn() -> super::super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CeipIsOptedIn())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}

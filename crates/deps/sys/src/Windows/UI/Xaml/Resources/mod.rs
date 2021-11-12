#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct CustomXamlResourceLoader(i32);
pub struct ICustomXamlResourceLoader(pub *mut ::core::ffi::c_void);
pub struct ICustomXamlResourceLoaderFactory(pub *mut ::core::ffi::c_void);
pub struct ICustomXamlResourceLoaderOverrides(pub *mut ::core::ffi::c_void);
pub struct ICustomXamlResourceLoaderStatics(pub *mut ::core::ffi::c_void);

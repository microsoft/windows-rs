#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct CustomXamlResourceLoader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICustomXamlResourceLoader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICustomXamlResourceLoaderFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICustomXamlResourceLoaderOverrides(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICustomXamlResourceLoaderStatics(pub *mut ::core::ffi::c_void);

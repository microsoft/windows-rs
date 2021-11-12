#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct CustomXamlResourceLoader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CustomXamlResourceLoader {}
impl ::core::clone::Clone for CustomXamlResourceLoader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICustomXamlResourceLoader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICustomXamlResourceLoader {}
impl ::core::clone::Clone for ICustomXamlResourceLoader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICustomXamlResourceLoaderFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICustomXamlResourceLoaderFactory {}
impl ::core::clone::Clone for ICustomXamlResourceLoaderFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICustomXamlResourceLoaderOverrides(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICustomXamlResourceLoaderOverrides {}
impl ::core::clone::Clone for ICustomXamlResourceLoaderOverrides {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICustomXamlResourceLoaderStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICustomXamlResourceLoaderStatics {}
impl ::core::clone::Clone for ICustomXamlResourceLoaderStatics {
    fn clone(&self) -> Self {
        *self
    }
}

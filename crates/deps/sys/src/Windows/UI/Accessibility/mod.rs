#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IScreenReaderPositionChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IScreenReaderPositionChangedEventArgs {}
impl ::core::clone::Clone for IScreenReaderPositionChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IScreenReaderService(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IScreenReaderService {}
impl ::core::clone::Clone for IScreenReaderService {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ScreenReaderPositionChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ScreenReaderPositionChangedEventArgs {}
impl ::core::clone::Clone for ScreenReaderPositionChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ScreenReaderService(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ScreenReaderService {}
impl ::core::clone::Clone for ScreenReaderService {
    fn clone(&self) -> Self {
        *self
    }
}

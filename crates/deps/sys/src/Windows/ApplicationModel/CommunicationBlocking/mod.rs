#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ICommunicationBlockingAccessManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICommunicationBlockingAccessManagerStatics {}
impl ::core::clone::Clone for ICommunicationBlockingAccessManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICommunicationBlockingAppManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICommunicationBlockingAppManagerStatics {}
impl ::core::clone::Clone for ICommunicationBlockingAppManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICommunicationBlockingAppManagerStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICommunicationBlockingAppManagerStatics2 {}
impl ::core::clone::Clone for ICommunicationBlockingAppManagerStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}

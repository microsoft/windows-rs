#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct CommunicationBlockingContract(i32);
#[repr(transparent)]
pub struct ICommunicationBlockingAccessManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICommunicationBlockingAppManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICommunicationBlockingAppManagerStatics2(pub *mut ::core::ffi::c_void);

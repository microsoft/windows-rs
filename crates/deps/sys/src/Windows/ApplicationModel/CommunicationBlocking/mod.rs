#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct CommunicationBlockingAccessManager(i32);
pub struct CommunicationBlockingAppManager(i32);
pub struct CommunicationBlockingContract(i32);
pub struct ICommunicationBlockingAccessManagerStatics(pub *mut ::core::ffi::c_void);
pub struct ICommunicationBlockingAppManagerStatics(pub *mut ::core::ffi::c_void);
pub struct ICommunicationBlockingAppManagerStatics2(pub *mut ::core::ffi::c_void);

#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub const DISPID_EVENT_ON_CONTEXT_DATA: u32 = 7u32;
pub const DISPID_EVENT_ON_SEND_ERROR: u32 = 8u32;
pub const DISPID_EVENT_ON_STATE_CHANGED: u32 = 5u32;
pub const DISPID_EVENT_ON_TERMINATION: u32 = 6u32;
#[repr(transparent)]
pub struct DRendezvousSessionEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRendezvousApplication(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRendezvousSession(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct RENDEZVOUS_SESSION_FLAGS(i32);
#[repr(C)]
pub struct RENDEZVOUS_SESSION_STATE(i32);
#[repr(C)]
pub struct RendezvousApplication(i32);

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
pub const DISPID_EVENT_ON_CONTEXT_DATA: u32 = 7u32;
pub const DISPID_EVENT_ON_SEND_ERROR: u32 = 8u32;
pub const DISPID_EVENT_ON_STATE_CHANGED: u32 = 5u32;
pub const DISPID_EVENT_ON_TERMINATION: u32 = 6u32;
#[repr(transparent)]
pub struct DRendezvousSessionEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DRendezvousSessionEvents {}
impl ::core::clone::Clone for DRendezvousSessionEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRendezvousApplication(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRendezvousApplication {}
impl ::core::clone::Clone for IRendezvousApplication {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRendezvousSession(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRendezvousSession {}
impl ::core::clone::Clone for IRendezvousSession {
    fn clone(&self) -> Self {
        *self
    }
}
pub const RSF_NONE: i32 = 0i32;
pub const RSF_INVITER: i32 = 1i32;
pub const RSF_INVITEE: i32 = 2i32;
pub const RSF_ORIGINAL_INVITER: i32 = 4i32;
pub const RSF_REMOTE_LEGACYSESSION: i32 = 8i32;
pub const RSF_REMOTE_WIN7SESSION: i32 = 16i32;
pub const RSS_UNKNOWN: i32 = 0i32;
pub const RSS_READY: i32 = 1i32;
pub const RSS_INVITATION: i32 = 2i32;
pub const RSS_ACCEPTED: i32 = 3i32;
pub const RSS_CONNECTED: i32 = 4i32;
pub const RSS_CANCELLED: i32 = 5i32;
pub const RSS_DECLINED: i32 = 6i32;
pub const RSS_TERMINATED: i32 = 7i32;
pub const RendezvousApplication: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 192807322,
    data2: 46558,
    data3: 18426,
    data4: [137, 102, 144, 130, 248, 47, 177, 146],
};

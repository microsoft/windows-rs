#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[doc = "*Required features: `Win32_System_RemoteAssistance`*"]
pub const DISPID_EVENT_ON_CONTEXT_DATA: u32 = 7u32;
#[doc = "*Required features: `Win32_System_RemoteAssistance`*"]
pub const DISPID_EVENT_ON_SEND_ERROR: u32 = 8u32;
#[doc = "*Required features: `Win32_System_RemoteAssistance`*"]
pub const DISPID_EVENT_ON_STATE_CHANGED: u32 = 5u32;
#[doc = "*Required features: `Win32_System_RemoteAssistance`*"]
pub const DISPID_EVENT_ON_TERMINATION: u32 = 6u32;
pub struct DRendezvousSessionEvents(i32);
pub struct IRendezvousApplication(i32);
pub struct IRendezvousSession(i32);
pub struct RENDEZVOUS_SESSION_FLAGS(i32);
pub struct RENDEZVOUS_SESSION_STATE(i32);
pub struct RendezvousApplication(i32);

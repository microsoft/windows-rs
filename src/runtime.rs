#![allow(dead_code)]

use crate::error::*;
use crate::guid::*;

// TODO: Is there not a std marker type for this?
pub enum VOID {}

#[link(name = "onecore")]
extern "C" {
    // TODO: these need stdcall
    pub fn WindowsCreateString(value: *const u16, length: u32, result: *mut *mut VOID) -> ErrorCode;
    pub fn WindowsGetStringRawBuffer(hstring: *const VOID, length: &mut u32) -> *const u16;
    pub fn WindowsGetStringLen(hstring: *const VOID) -> u32;

    pub fn CoIncrementMTAUsage(cookie: *mut *mut VOID) -> ErrorCode;
    pub fn RoGetActivationFactory(hstring: *const VOID, interface: &Guid, result: *mut *mut VOID) -> ErrorCode;
}

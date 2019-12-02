#![allow(dead_code)]

use crate::error::*;
use crate::guid::*;

pub enum VOID {}

#[link(name = "onecore")]
extern "stdcall" {
    pub fn CoIncrementMTAUsage(cookie: *mut *mut VOID) -> ErrorCode;

    pub fn WindowsCreateString(value: *const u16, length: u32, result: *mut *mut VOID) -> ErrorCode;
    pub fn WindowsGetStringRawBuffer(hstring: *const VOID, length: &mut u32) -> *const u16;
    pub fn WindowsGetStringLen(hstring: *const VOID) -> u32;
    pub fn WindowsPreallocateStringBuffer(len: u32, buffer: *mut *mut u16, handle: *mut *mut VOID) -> ErrorCode;
    pub fn WindowsPromoteStringBuffer(handle: *const VOID, hstring: *mut *mut VOID) -> ErrorCode;
    pub fn WindowsDeleteString(hstring: *const VOID) -> ErrorCode;

    pub fn RoGetActivationFactory(hstring: *const VOID, interface: &Guid, result: *mut *mut VOID) -> ErrorCode;
}

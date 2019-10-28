#![allow(dead_code)]

#[link(name = "onecore")]
extern "C" {
    pub fn RoInitialize(apartment: ApartmentType) -> i32;

    pub fn WindowsCreateString(value: *const u16, length: u32, hstring: *mut PVOID) -> i32;
    pub fn WindowsGetStringRawBuffer(hstring: PVOID, length: *mut u32) -> *const u16;
}

pub enum VOID{}
pub type PVOID = *mut VOID;

#[repr(C)]
pub enum ApartmentType {
    SingleThreaded,
    MultiThreaded,
}

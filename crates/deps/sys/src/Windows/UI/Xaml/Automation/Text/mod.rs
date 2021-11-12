#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct TextPatternRangeEndpoint(i32);
#[repr(C)]
pub struct TextUnit(i32);

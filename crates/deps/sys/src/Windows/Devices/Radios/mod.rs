#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IRadio(i32);
pub struct IRadioStatics(i32);
pub struct Radio(i32);
pub struct RadioAccessStatus(i32);
pub struct RadioKind(i32);
pub struct RadioState(i32);

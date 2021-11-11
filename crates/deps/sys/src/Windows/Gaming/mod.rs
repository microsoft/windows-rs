#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Gaming_Input")]
pub mod Input;
#[cfg(feature = "Gaming_Preview")]
pub mod Preview;
#[cfg(feature = "Gaming_UI")]
pub mod UI;
#[cfg(feature = "Gaming_XboxLive")]
pub mod XboxLive;
#[link(name = "windows")]
extern "system" {}

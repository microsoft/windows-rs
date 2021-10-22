#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[cfg(feature = "Phone_ApplicationModel")]
pub mod ApplicationModel;
#[cfg(feature = "Phone_Devices")]
pub mod Devices;
#[cfg(feature = "Phone_Management")]
pub mod Management;
#[cfg(feature = "Phone_Media")]
pub mod Media;
#[cfg(feature = "Phone_Notification")]
pub mod Notification;
#[cfg(feature = "Phone_PersonalInformation")]
pub mod PersonalInformation;
#[cfg(feature = "Phone_Speech")]
pub mod Speech;
#[cfg(feature = "Phone_StartScreen")]
pub mod StartScreen;
#[cfg(feature = "Phone_System")]
pub mod System;
#[cfg(feature = "Phone_UI")]
pub mod UI;
#[repr(C)]
#[derive(
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
)]
pub struct PhoneContract(pub u8);

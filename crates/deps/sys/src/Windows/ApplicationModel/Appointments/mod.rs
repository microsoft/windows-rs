#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
pub mod AppointmentsProvider;
#[cfg(feature = "ApplicationModel_Appointments_DataProvider")]
pub mod DataProvider;
#[link(name = "windows")]
extern "system" {}

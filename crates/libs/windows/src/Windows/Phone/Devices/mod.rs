#[cfg(feature = "Phone_Devices_Notification")]
pub mod Notification;
#[cfg(feature = "Phone_Devices_Power")]
pub mod Power;
#[cfg(feature = "implement")]
::core::include!("impl.rs");

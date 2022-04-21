#[cfg(feature = "Win32_Devices_Enumeration_Pnp")]
pub mod Pnp;
#[cfg(feature = "implement")]
::core::include!("impl.rs");

#[cfg(feature = "Win32_Storage_Packaging_Appx")]
pub mod Appx;
#[cfg(feature = "Win32_Storage_Packaging_Opc")]
pub mod Opc;
#[cfg(feature = "implement")]
::core::include!("impl.rs");

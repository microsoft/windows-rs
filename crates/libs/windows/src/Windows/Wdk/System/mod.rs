#[cfg(feature = "Wdk_System_OfflineRegistry")]
pub mod OfflineRegistry;
#[cfg(feature = "Wdk_System_SystemServices")]
pub mod SystemServices;
#[cfg(feature = "implement")]
::core::include!("impl.rs");

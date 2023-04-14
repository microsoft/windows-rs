#[cfg(feature = "Wdk_Graphics_Direct3D")]
pub mod Direct3D;
#[cfg(feature = "implement")]
::core::include!("impl.rs");

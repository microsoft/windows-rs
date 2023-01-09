#[cfg(feature = "Perception_Automation_Core")]
pub mod Core;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
#[cfg(feature = "default")]
::core::include!("default.rs");

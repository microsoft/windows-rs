#[cfg(feature = "Phone_Speech_Recognition")]
pub mod Recognition;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
#[cfg(feature = "default")]
::core::include!("default.rs");

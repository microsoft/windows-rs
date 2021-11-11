#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Win32_System_Diagnostics_Ceip")]
pub mod Ceip;
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
pub mod Debug;
#[cfg(feature = "Win32_System_Diagnostics_Etw")]
pub mod Etw;
#[cfg(feature = "Win32_System_Diagnostics_ProcessSnapshotting")]
pub mod ProcessSnapshotting;
#[cfg(feature = "Win32_System_Diagnostics_ToolHelp")]
pub mod ToolHelp;

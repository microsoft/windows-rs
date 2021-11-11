#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Security_Authentication_Identity")]
pub mod Identity;
#[cfg(feature = "Security_Authentication_OnlineId")]
pub mod OnlineId;
#[cfg(feature = "Security_Authentication_Web")]
pub mod Web;

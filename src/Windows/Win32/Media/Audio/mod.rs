#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[cfg(feature = "Win32_Media_Audio_CoreAudio")]
pub mod CoreAudio;
#[cfg(feature = "Win32_Media_Audio_DirectMusic")]
pub mod DirectMusic;
#[cfg(feature = "Win32_Media_Audio_XAudio2")]
pub mod XAudio2;

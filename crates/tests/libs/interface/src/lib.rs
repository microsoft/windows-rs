#![cfg(windows)]
#![doc = include_str!("../../../../libs/interface/readme.md")]

#[expect(
    nonstandard_style,
    clippy::missing_safety_doc,
    clippy::too_many_arguments
)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use bindings::*;

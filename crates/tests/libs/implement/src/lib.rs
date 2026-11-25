#![cfg(windows)]
#![doc = include_str!("../../../../libs/implement/readme.md")]

#[expect(nonstandard_style, clippy::missing_safety_doc)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use bindings::*;

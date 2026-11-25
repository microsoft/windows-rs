#![cfg(windows)]

#[expect(
    nonstandard_style,
    clippy::missing_safety_doc,
    clippy::missing_transmute_annotations
)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use bindings::*;

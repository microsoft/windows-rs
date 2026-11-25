#![cfg(windows)]

#[expect(non_snake_case, clippy::missing_safety_doc)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use bindings::*;

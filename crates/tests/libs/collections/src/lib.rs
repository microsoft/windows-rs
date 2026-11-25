#[cfg(windows)]
#[expect(nonstandard_style, clippy::missing_safety_doc)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[cfg(windows)]
pub use bindings::*;

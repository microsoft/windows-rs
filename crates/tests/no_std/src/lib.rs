//! Test for `#![no_std]` crates.
//!
//! Compiling this crate verifies that the Windows crates can be compiled with their "std"
//! feature disabled.

#![no_std]

#[cfg(test)]
mod tests {
    use windows::core::{implement, ComObject};

    #[implement]
    struct App {}

    // Compilation is sufficient to test.
    #[test]
    fn basic() {
        let object = ComObject::new(App {});
        drop(object);
    }
}

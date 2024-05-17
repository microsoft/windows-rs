//! Test for `#![no_std]` crates.

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

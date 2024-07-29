//! Test for `#![no_std]` crates.
//!
//! Compiling this crate verifies that the Windows crates can be compiled with their "std"
//! feature disabled. The tricky part is that `cargo test` depends on `std` so testing
//! `no_std` requires using `cargo check` instead. That's what the `no_std.yml` is for.

#![no_std]

#[windows::core::implement]
struct App;

#[cfg(not(test))]
fn _test() {
    let _ = windows::core::HRESULT(0);
    let _ = windows::core::ComObject::new(App);
}

// #[cfg_attr(not(test), panic_handler)]
// fn _panic(_: &core::panic::PanicInfo<'_>) -> ! {
//     loop {}
// }

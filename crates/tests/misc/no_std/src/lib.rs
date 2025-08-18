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
    // Pull in something from each library crate that supports `no_std` to ensure they don't
    // accidentally depend on `std`.
    let _ = windows_core::ComObject::new(App);
    let _ = windows_registry::CURRENT_USER.create("software\\windows-rs");
    let _ = windows_result::HRESULT(0);
    let _ = windows_strings::BSTR::new();
    let _ = windows_sys::core::GUID::from_u128(0);
    let _ = windows_version::OsVersion::current();
    let _ = windows::core::ComObject::new(App);
    let _ = windows_strings::h!("hello");
    let _ = windows_strings::s!("hello");
    let _ = windows_strings::w!("hello");
    let _: Option<windows_collections::IVector<i32>> = None;
    let _: Option<windows_future::IAsyncOperation<i32>> = None;
    let _ = windows_numerics::Vector2::new(0.0, 0.0);
    let _ = windows_threading::Pool::new();
}

// This panic handler will cause a build error if an indirect `std` dependency exists as `std`
// will include its own panic handler and conflict with this one.
#[cfg_attr(not(test), panic_handler)]
#[expect(clippy::empty_loop)] // just a test
fn _panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

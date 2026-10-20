//! Runs each fixture's `client.rs` against the component through the windows-rs projection. The
//! client is compiled into this test, but the component stays in its own DLL, so calls cross the
//! same activation and ABI boundary as the C++ and C# clients.

#![cfg(windows)]

#[allow(
    dead_code,
    reason = "the C++ and C# consumers use the compiled winmd and harness"
)]
mod support;
use support::*;

#[allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
    include!(concat!(env!("OUT_DIR"), "/win32.rs"));
}

use bindings::*;
use windows_core::*;

include!(concat!(env!("OUT_DIR"), "/client.rs"));
include!(concat!(env!("OUT_DIR"), "/fixtures.rs"));

fn run(name: &str, namespace: &str) {
    let Some(test) = fixture_client(name) else {
        return;
    };

    let fixture = Fixture::new("rust", name);
    let component = fixture
        .component()
        .unwrap_or_else(|| panic!("fixture `{name}` has a `client.rs` but no component"));

    // windows-core loads `<namespace>.dll` for an unregistered class, searching the default
    // directories, which include those added with `AddDllDirectory`.
    std::fs::copy(component, fixture.out.join(format!("{namespace}.dll"))).unwrap();
    let directory = HSTRING::from(fixture.out.as_path());
    let cookie = unsafe { AddDllDirectory(&directory) };
    assert!(!cookie.is_null(), "{}", Error::from_thread());

    test().unwrap();
}

## Generate Rust bindings for Windows

The [windows-bindgen](https://crates.io/crates/windows-bindgen) crate automatically generates Rust bindings from Windows metadata.

* [Getting started](https://kennykerr.ca/rust-getting-started/)
* [Samples](https://github.com/microsoft/windows-rs/tree/0.55.0/crates/samples) <!-- link to samples for upcoming release -->
* [Releases](https://github.com/microsoft/windows-rs/releases)

Start by adding the following to your Cargo.toml file:

```toml
[dependencies.windows-targets]
version = "0.52.4"

[dev-dependencies.windows-bindgen]
version = "0.55.0"
```

Generates Rust bindings in a build script or test as needed:

```rust,no_run
#[test]
fn bindgen() {
    let args = [
        "--out",
        "src/bindings.rs",
        "--config",
        "flatten",
        "--filter",
        "Windows.Win32.System.SystemInformation.GetTickCount",
    ];

    windows_bindgen::bindgen(args).unwrap();
}

mod bindings;

fn main() {
    unsafe {
        println!("{}", bindings::GetTickCount());
    }
}
```

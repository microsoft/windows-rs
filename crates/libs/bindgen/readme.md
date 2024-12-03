## Code generator for Windows metadata

The [windows-bindgen](https://crates.io/crates/windows-bindgen) crate automatically generates Rust bindings from Windows metadata.

* [Getting started](https://kennykerr.ca/rust-getting-started/)
* [Samples](https://github.com/microsoft/windows-rs/tree/0.58.0/crates/samples)
* [Releases](https://github.com/microsoft/windows-rs/releases)

Start by adding the following to your Cargo.toml file:

```toml
[dependencies.windows-targets]
version = "0.52"

[dev-dependencies.windows-bindgen]
version = "0.58"
```

Generate Rust bindings in a build script as follows:

```rust,no_run
fn main() {
    let args = [
        "--out",
        "src/bindings.rs",
        "--flat",
        "--filter",
        "GetTickCount",
    ];

    windows_bindgen::bindgen(args);
}
```

And then use the bindings as follows:

```rust,ignore
mod bindings;

fn main() {
    unsafe {
        println!("{}", bindings::GetTickCount());
    }
}
```

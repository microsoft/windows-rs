# cppwinrt

Run the bundled C++/WinRT compiler from a Rust build. Use it when the build also compiles C++ that
consumes or implements WinRT types; use [windows-bindgen](https://docs.rs/windows-bindgen) for Rust.
The bundled compiler runs on Windows.

- 📦 [crates.io](https://crates.io/crates/cppwinrt)
- 📖 [docs.rs](https://docs.rs/cppwinrt)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/cppwinrt)

## Generate headers

```toml
[build-dependencies]
cppwinrt = "0.100"
```

In `build.rs`, pass your component metadata and a build-owned output directory:

```rust,no_run
use std::{ffi::OsStr, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=component.winmd");
    let output = PathBuf::from(std::env::var_os("OUT_DIR").unwrap()).join("include");
    cppwinrt::cppwinrt([
        OsStr::new("-in"),
        OsStr::new("component.winmd"),
        OsStr::new("-out"),
        output.as_os_str(),
    ]);
}
```

Add reference metadata inputs required by the component, then add the output directory to your
C++ compiler's include paths. Compile and link C++ sources separately, with Cargo rerun directives
for those inputs too.

`cppwinrt(["-help"])` lists the bundled compiler's options. The wrapper returns standard output on
success and panics on failure, stopping the build.

The [robot C++ component][component] and [C++ client][client] show the complete pipeline.

[component]: https://github.com/microsoft/windows-rs/tree/master/crates/samples/robot/component_cpp
[client]: https://github.com/microsoft/windows-rs/tree/master/crates/samples/robot/client_cpp

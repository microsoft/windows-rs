## Windows metadata binding generator

The [windows-bindgen](https://crates.io/crates/windows-bindgen) crate generates Rust bindings from
Windows metadata.

* [Getting started](https://github.com/microsoft/windows-rs/blob/master/docs/readme.md)
* [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples)
* [Releases](https://github.com/microsoft/windows-rs/releases)

Add the generator as a build dependency and the generated code's runtime dependency:

```toml
[dependencies.windows-link]
version = "0.2"

[build-dependencies.windows-bindgen]
version = "0.66"
```

Generate bindings from `build.rs`:

```rust,no_run
let args = [
    "--out",
    "src/bindings.rs",
    "--flat",
    "--sys",
    "--filter",
    "GetTickCount",
];
windows_bindgen::bindgen(args);
```

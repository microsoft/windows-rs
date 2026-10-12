## windows-bindgen

The [windows-bindgen](https://crates.io/crates/windows-bindgen) crate generates Rust bindings from
Windows metadata.

* [Getting
  started](https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-bindgen.md)

Add the generator as a build dependency and the generated code's runtime dependency:

```toml
[dependencies.windows-link]
version = "0.100"

[build-dependencies.windows-bindgen]
version = "0.100"
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

Formatting is best effort. If `rustfmt` is unavailable or fails, generation writes unformatted
bindings without a warning. Install `rustfmt` when generating checked-in source.

And then use the bindings as follows:

```rust,ignore
mod bindings;

unsafe {
    println!("{}", bindings::GetTickCount());
}
```

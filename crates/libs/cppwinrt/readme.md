## cppwinrt

The [cppwinrt](https://crates.io/crates/cppwinrt) crate bundles the C++/WinRT compiler for use in
Rust.

* [Getting
  started](https://github.com/microsoft/windows-rs/blob/master/docs/crates/cppwinrt.md)

Start by adding the following to your Cargo.toml file:

```toml
[dependencies.cppwinrt]
version = "0.100"
```

Use `cppwinrt` function as needed:

```rust,ignore
println!("{}", cppwinrt::cppwinrt(["-help"]));
```

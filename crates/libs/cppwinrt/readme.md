## C++/WinRT

The [cppwinrt](https://crates.io/crates/cppwinrt) crate bundles the C++/WinRT compiler for use in Rust.

* [Getting started](https://kennykerr.ca/rust-getting-started/)
* [Samples](https://github.com/microsoft/windows-rs/tree/0.58.0/crates/samples)
* [Releases](https://github.com/microsoft/windows-rs/releases)

Start by adding the following to your Cargo.toml file:

```toml
[dependencies.cppwinrt]
version = "0.1"
```

Use `cppwinrt` function as needed:

```rust
match cppwinrt::cppwinrt(["-help"]) {
    Ok(output) => println!("{output}"),
    Err(error) => println!("{error}"),
};
```

Source:

* <https://github.com/microsoft/cppwinrt>
* <https://www.nuget.org/packages/microsoft.windows.cppwinrt>
* Version 2.0.240405.15

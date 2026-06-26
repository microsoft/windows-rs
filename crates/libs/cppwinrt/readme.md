## C++/WinRT

The [cppwinrt](https://crates.io/crates/cppwinrt) crate bundles the C++/WinRT compiler for use in Rust.

* [Getting started](https://github.com/microsoft/windows-rs/blob/master/docs/getting-started.md)
* [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples)
* [Releases](https://github.com/microsoft/windows-rs/releases)

Start by adding the following to your Cargo.toml file:

```toml
[dependencies.cppwinrt]
version = "0.3"
```

Use `cppwinrt` function as needed:

```rust,ignore
println!("{}", cppwinrt::cppwinrt(["-help"]));
```

Source:

* <https://github.com/microsoft/cppwinrt>
* <https://www.nuget.org/packages/microsoft.windows.cppwinrt>
* Version 2.0.250303.1

# windows-rdl

Compile Rust Definition Language (RDL) into Windows metadata, or turn `.winmd` files back into
reviewable RDL. Use it for custom API contracts or output from
[windows-clang](https://docs.rs/windows-clang).

- 📦 [crates.io](https://crates.io/crates/windows-rdl)
- 📖 [docs.rs](https://docs.rs/windows-rdl)
- 📚 [Language reference](https://github.com/microsoft/windows-rs/blob/master/crates/libs/rdl/rdl.md)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/rdl)

## Define a contract

Create `example.rdl`:

```text
#[winrt]
mod Example {
    #[guid(auto)]
    interface ICounter {
        fn Value(&self) -> i32;
        fn Add(&self, #[in] amount: i32);
    }
}
```

Use `#[winrt]` for WinRT contracts or `#[win32]` for native API declarations. WinRT interfaces
need a GUID, explicit or automatic, and every explicit argument needs `#[in]` or `#[out]`.
RDL looks like Rust but is a separate language.

## Compile and inspect metadata

```toml
[dependencies]
windows-rdl = "0.100"
```

```rust,no_run
windows_rdl::reader()
    .input("example.rdl")
    .reference_default()
    .output("example.winmd")
    .write()
    .unwrap();

windows_rdl::writer()
    .input("example.winmd")
    .output("roundtrip.rdl")
    .write()
    .unwrap();
```

References resolve names without copying their definitions into the output. Use
`reference_default()` for standard Windows types or `reference("dependency.winmd")` for another
component. Writer references likewise resolve external types and attributes without emitting them.

Pass the `.winmd` to [windows-bindgen](https://docs.rs/windows-bindgen) for Rust bindings or
[cppwinrt](https://docs.rs/cppwinrt) for C++ headers. Keep the RDL as the reviewed source and
regenerate the binary metadata when it changes.

Use a dedicated output directory with writer `split` or `partition`: these modes remove existing
`.rdl` files there. The [robot component][robot] combines RDL and binding generation in one build.

[robot]: https://github.com/microsoft/windows-rs/tree/master/crates/samples/robot/component

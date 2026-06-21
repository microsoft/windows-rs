## RDL parser library and ECMA-335 generator

The [windows-rdl](https://crates.io/crates/windows-rdl) crate compiles **RDL**
(Rust Definition Language) — a Rust-like text format for describing Windows
APIs — into ECMA-335 `.winmd` metadata, and back again. It is the engine behind
the `riddle` metadata compiler.

* [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples)
* [Releases](https://github.com/microsoft/windows-rs/releases)

Start by adding the following to your Cargo.toml file:

```toml
[dependencies.windows-rdl]
version = "0.3"
```

Use the `reader` to compile `.rdl` source into a `.winmd`, and the `writer` to
regenerate canonical `.rdl` from a `.winmd`:

```rust,no_run
// RDL source -> winmd metadata.
windows_rdl::reader()
    .input("example.rdl")
    .output("example.winmd")
    .write()
    .unwrap();

// winmd metadata -> canonical RDL source.
windows_rdl::writer()
    .input("example.winmd")
    .output("example.rdl")
    .write()
    .unwrap();
```

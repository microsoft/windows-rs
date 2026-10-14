## windows-rdl

The [windows-rdl](https://crates.io/crates/windows-rdl) crate compiles **RDL** (Rust Definition
Language) - a Rust-like text format for describing Windows APIs - into ECMA-335 `.winmd` metadata,
and back again.

Shared and named nested records retain owner-scoped declarations and references such as
`Outer::Inner`; anonymous value fields can use inline structs and unions.

Win32 type expressions include qualified pointer chains and inline C/system function pointers,
such as `extern "C" fn(*const u8, u32) -> i32`, without requiring named callback declarations.

* [Getting
  started](https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-rdl.md)

Start by adding the following to your Cargo.toml file:

```toml
[dependencies.windows-rdl]
version = "0.100"
```

Use the `reader` to compile `.rdl` source into a `.winmd`, and the `writer` to regenerate canonical
`.rdl` from a `.winmd`:

```rust,no_run
windows_rdl::reader()
    .input("example.rdl")
    .output("example.winmd")
    .write()
    .unwrap();

windows_rdl::writer()
    .input("example.winmd")
    .output("example.rdl")
    .write()
    .unwrap();
```

## windows-clang

The [windows-clang](https://crates.io/crates/windows-clang) crate scrapes C/C++ headers with
libclang and emits **RDL** (Rust Definition Language) source - the text format understood by
[windows-rdl](https://crates.io/crates/windows-rdl). It is the header-facing front end of the Win32
metadata pipeline: headers to RDL (this crate) to `.winmd` (windows-rdl).

* [Getting
  started](https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-clang.md)

Start by adding the following to your Cargo.toml file:

```toml
[dependencies.windows-clang]
version = "0.100"
```

Point it at one or more headers and write the resulting per-header RDL, then feed that RDL to
`windows_rdl::reader()` to compile a `.winmd`:

```rust,no_run
windows_clang::clang()
    .input("Example.h")
    .output("rdl")
    .namespace("Example")
    .write_by_header()
    .unwrap();
```

## RDL generator for C/C++ headers

The [windows-clang](https://crates.io/crates/windows-clang) crate scrapes C/C++ headers with
libclang and emits **RDL** (Rust Definition Language) source - the text format understood by
[windows-rdl](https://crates.io/crates/windows-rdl). It is the header-facing front end of the Win32
metadata pipeline: headers to RDL (this crate) to `.winmd` (windows-rdl).

* [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples)
* [Releases](https://github.com/microsoft/windows-rs/releases)

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

Use `.reference("dependency.winmd")` when the headers refer to types defined by another metadata
file. Use `.input_text(source)` or `.input_texts(sources)` for C/C++ source already in memory. Use
`.reference_default()` for the standard Windows metadata.

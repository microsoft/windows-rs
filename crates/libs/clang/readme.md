# windows-clang

Capture C and C++ declarations with Clang and generate RDL for Windows bindings.
Use this crate when an API has headers but no suitable metadata; use
[windows-bindgen](https://docs.rs/windows-bindgen) directly when a `.winmd` already exists.

- 📦 [crates.io](https://crates.io/crates/windows-clang)
- 📖 [docs.rs](https://docs.rs/windows-clang)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/clang)

## Set up the compiler

The parser needs a matched `libclang.dll` and `windows_clang.dll` pair. Set `LIBCLANG_PATH` to
their directory. The [raw bindings README][native] explains how to prepare the libraries.
Include paths, SDK headers, and compiler definitions are supplied by the caller.

For RDL output, enable `projection`:

```toml
[dependencies]
windows-clang = { version = "0.101", features = ["projection"] }
```

## Generate RDL

```rust,no_run
# #[cfg(feature = "projection")]
# fn main() {
use windows_clang::{Input, projection};

let input = Input {
    name: "example.h".into(),
    source: "struct Record { int count; void* data; };".into(),
    roots: vec!["Record".into()],
    ..Default::default()
};

let output = projection::generate(&[input], "Example").unwrap();
for (name, partition) in output.partitions {
    std::fs::write(format!("{name}.rdl"), partition.rdl).unwrap();
}
# }
# #[cfg(not(feature = "projection"))]
# fn main() {}
```

`generate` processes x86, x64, and ARM64 and fails on unsupported reachable declarations.
Compile its RDL with [windows-rdl](https://docs.rs/windows-rdl), then generate Rust with
`windows-bindgen`.

For a real header, set `Input.source` to its include directive and supply include paths and compiler
definitions in `Input.arguments`. Use `roots` to select declarations and their dependencies.
Publishing functions also requires DLL/export ownership; see the
[`projection` API](https://docs.rs/windows-clang/latest/windows_clang/projection/index.html).

Keep the native runtime, metadata readers, and binding generators compatible when upgrading.
Check generated signatures and layouts against the native API before using the bindings.

[native]: https://github.com/microsoft/windows-rs/blob/master/crates/libs/clang-sys/readme.md

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
windows-clang = { version = "0.100", features = ["projection"] }
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

`generate` processes x86, x64, and ARM64. Compile its RDL with
[windows-rdl](https://docs.rs/windows-rdl), then generate Rust with `windows-bindgen`.
For native inspection without RDL, use `capture` and `merge` without the projection feature.

## Configure a header

Set `Input.source` to header text or an include directive, with include paths and definitions in
`Input.arguments`. Start with explicit roots so unsupported declarations are easy to diagnose.

| Input | Use |
| --- | --- |
| `roots` | Select qualified declarations and their dependencies; empty enables automatic selection. |
| `root_paths` | Restrict automatic selection from included headers. |
| `macros` | Select object-like macros for evaluation. |
| `macro_headers` | Discover object-like macros owned by selected headers. |

Function publication also needs DLL/export ownership through `projection::Imports`; it cannot be
inferred from a header. Use `projection::References` when existing metadata owns shared definitions.

Strict generation fails on unsupported reachable declarations. `generate_partial` returns
supported output plus omissions. Inspect those omissions before using the result: partial output
does not establish full header coverage, and native conflicts remain errors.

Keep the native runtime, metadata readers, and binding generators compatible when upgrading.
Successful metadata generation alone does not establish that a consumer supports every encoded
contract; check representative generated signatures and layouts.

[native]: https://github.com/microsoft/windows-rs/blob/master/crates/libs/clang-sys/readme.md

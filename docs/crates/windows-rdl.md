# windows-rdl

> A parser for RDL (Rust Definition Language) and an ECMA-335 metadata generator.

- 📦 [crates.io](https://crates.io/crates/windows-rdl)
- 📖 [API reference (docs.rs)](https://docs.rs/windows-rdl)
- 🚀 [Getting started](../../crates/libs/rdl/readme.md)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/rdl)

`windows-rdl` parses RDL — a compact, Rust-like syntax for describing Windows
APIs — and produces ECMA-335 `.winmd` metadata. It lets API definitions be
authored in a readable text format and then compiled into the same metadata that
[`windows-bindgen`](windows-bindgen.md) consumes. RDL sits at the front of the
metadata authoring pipeline.

## How it's built

Uses `syn`/`quote`/`proc-macro2` for the RDL grammar and `clang-sys` for the C++
header-parsing path. Exercised by the `test_rdl` and `test_clang` crates and the
`rdl_roundtrip` tool.

## Testing

Run `cargo test -p windows-rdl`; see also the workspace test crates.

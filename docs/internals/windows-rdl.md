# windows-rdl — internals

> Maintenance notes for `windows-rdl`. For usage, see the [guide](../guide/windows-rdl.md).

- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/rdl)

## How it's built

Uses `syn`/`quote`/`proc-macro2` for the RDL grammar and `clang-sys` for the C++ header-parsing path. Exercised by the `test_rdl` and `test_clang` crates and the `rdl_roundtrip` tool.

## Testing

_See the workspace test crates and `cargo test -p windows-rdl`._

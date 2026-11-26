# windows-metadata

Read and write ECMA-335 metadata for WinRT and Win32 APIs.
Use this crate for inspectors and generators, not for calling Windows APIs.

- 📦 [crates.io](https://crates.io/crates/windows-metadata)
- 📖 [docs.rs](https://docs.rs/windows-metadata)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/metadata)

## Inspect a type

```toml
[dependencies]
windows-metadata = "0.101"
```

The default `writer` feature enables writing and merging. Set `default-features = false` for
reader-only tools.

Given a `Windows.winmd` file:

```rust,no_run
use windows_metadata::*;

let index = reader::Index::read("Windows.winmd").unwrap();
let point = index.expect("Windows.Foundation", "Point");
let fields: Vec<_> = point.fields().collect();

assert_eq!(fields[0].name(), "X");
assert_eq!(fields[0].ty(), Type::F32);
assert_eq!(fields[1].name(), "Y");
```

[windows-default](https://docs.rs/windows-default) supplies the standard metadata as embedded
bytes when a separate file is not available.

Rows borrow the index, so keep it alive while using them. `expect` requires one matching
definition; use `get` when duplicates are possible, including architecture-specific variants.
Include a component's dependencies when resolving its referenced types.

For text authoring or decompilation, start with [windows-rdl](https://docs.rs/windows-rdl).
For generated Rust APIs, use [windows-bindgen](https://docs.rs/windows-bindgen).

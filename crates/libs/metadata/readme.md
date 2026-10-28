# windows-metadata

Read and write the ECMA-335 metadata used by .NET, WinRT, and Windows API bindings.
Use this crate for inspectors and generators, not for calling Windows APIs.

- 📦 [crates.io](https://crates.io/crates/windows-metadata)
- 📖 [docs.rs](https://docs.rs/windows-metadata)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/metadata)

## Inspect a type

```toml
[dependencies]
windows-metadata = "0.100"
```

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

## Load and query

| Task | API |
| --- | --- |
| Read bytes | `reader::File::new` |
| Query several files | `reader::Index::new` |
| Select a target architecture | `reader::Index::new_for_architecture` |
| Combine metadata files | `merge()` |
| Change namespace ownership | `remap()` |
| Construct tables | `writer::File`, then `into_stream` |

Rows borrow the index, so keep it alive while using them. `expect` requires one matching
definition; use `get` when duplicates are possible, including architecture-specific variants.
Include a component's dependencies when resolving its referenced types.

Use `MethodDef::params_by_sequence` to associate parameter rows with signature positions rather
than assuming table order. Sequence zero describes the return value.

For text authoring or decompilation, start with [windows-rdl](https://docs.rs/windows-rdl).
For generated Rust APIs, use [windows-bindgen](https://docs.rs/windows-bindgen).

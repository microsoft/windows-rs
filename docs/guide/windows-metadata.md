# windows-metadata

> A low-level reader and writer for the ECMA-335 metadata format.

- 📦 [crates.io](https://crates.io/crates/windows-metadata)
- 📖 [API reference (docs.rs)](https://docs.rs/windows-metadata)
- 🛠 [Internals](../internals/windows-metadata.md)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/metadata)

## Overview

`windows-metadata` reads and writes the ECMA-335 metadata format used by .NET,
WinRT, and the Win32 metadata. It is the foundation
[`windows-bindgen`](windows-bindgen.md) builds on. The `reader::Index` type
loads one or more `.winmd` files and lets you query namespaces, type
definitions, and their members.

## Example

```rust,no_run
use windows_metadata::*;

let index = reader::Index::read("Windows.winmd").unwrap();

let def = index.expect("Windows.Foundation", "Point");
assert_eq!(def.namespace(), "Windows.Foundation");
assert_eq!(def.name(), "Point");

let extends = def.extends().unwrap();
assert_eq!(extends.namespace(), "System");
assert_eq!(extends.name(), "ValueType");
```
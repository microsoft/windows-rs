## Low-level metadata library for ECMA-335

The [windows-metadata](https://crates.io/crates/windows-metadata) crate provides a reader and writer
for the ECMA-335 metadata format used by .NET, WinRT, and more recently the Win32 metadata. 

* [Getting started](https://kennykerr.ca/rust-getting-started/)
* [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples)
* [Releases](https://github.com/microsoft/windows-rs/releases)

Start by adding the following to your Cargo.toml file:

```toml
[dependencies.windows-metadata]
version = "0.59"
```

Use the Windows metadata support as needed. Here is how you might use the reader to query type information:

```rust,no_run
use windows_metadata::*;

let index = reader::TypeIndex::read("Windows.winmd").unwrap();

let def = index.expect("Windows.Foundation", "Point");
assert_eq!(def.namespace(), "Windows.Foundation");
assert_eq!(def.name(), "Point");

let extends = def.extends().unwrap();
assert_eq!(extends.namespace(), "System");
assert_eq!(extends.name(), "ValueType");

let fields: Vec<_> = def.fields().collect();
assert_eq!(fields.len(), 2);
assert_eq!(fields[0].name(), "X");
assert_eq!(fields[1].name(), "Y");
assert_eq!(fields[0].ty(), Type::F32);
assert_eq!(fields[1].ty(), Type::F32);
```

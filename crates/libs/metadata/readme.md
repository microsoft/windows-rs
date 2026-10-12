## windows-metadata

The [windows-metadata](https://crates.io/crates/windows-metadata) crate reads and writes the
ECMA-335 metadata format used by .NET, WinRT, and Win32 metadata.

Pointer signatures preserve mixed const/mutable runs through recursive `Type` nodes. Uniform
signatures retain their existing bytes; mixed signatures use explicit modifier boundaries.

`Type::Const` and `Type::Volatile` preserve native qualifiers on individual type nodes using
required signature modifiers, independently of parameter direction and pointer-run constness.

`TypeDef::is_opaque()` distinguishes opaque native records from complete empty records. Use
`Index::validate_opaque_types` to reject invalid opaque storage uses.

Raw native annotation identities and original positional bindings survive independently of
parameter names and direction. `Index::validate_native_contracts` and checked
`writer::File::try_into_stream` check opaque, annotation, and bit-field invariants before publishing
metadata.

`Field::bitfields` reads logical member types, qualifiers, and raw evidence independently of backing
storage. Legacy interval-only members retain an unspecified logical type.

* [Getting
  started](https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-metadata.md)

Start by adding the following to your Cargo.toml file:

```toml
[dependencies.windows-metadata]
version = "0.100"
```

Query a type with the metadata reader:

```rust,no_run
use windows_metadata::*;

let index = reader::Index::read("Windows.winmd").unwrap();

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

## windows-rdl

The [windows-rdl](https://crates.io/crates/windows-rdl) crate compiles **RDL** (Rust Definition
Language) - a Rust-like text format for describing Windows APIs - into ECMA-335 `.winmd` metadata,
and back again.

* [Getting
  started](https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-rdl.md)

Start by adding the following to your Cargo.toml file:

```toml
[dependencies.windows-rdl]
version = "0.100"
```

Use the `reader` to compile `.rdl` source into a `.winmd`, and the `writer` to regenerate canonical
`.rdl` from a `.winmd`:

```rust,no_run
windows_rdl::reader()
    .input("example.rdl")
    .output("example.winmd")
    .write()
    .unwrap();

windows_rdl::writer()
    .input("example.winmd")
    .output("example.rdl")
    .write()
    .unwrap();
```

Use `windows_rdl::inventory("example.rdl")` to inspect declarations and members without reference
metadata. The parsed inventory preserves duplicates and source order; it does not validate the
original native API.

Pointer types preserve mixed qualifiers such as `*const *mut i32` through RDL/WinMD round-trips,
including fields, parameters, results, and typedef targets. Parameter direction remains separate.

Native `Const<T>` and `Volatile<T>` qualify an exact type node, such as a pointer value rather
than its pointee. RDL/WinMD preserves their placement; Rust projection does not yet support them.

Native annotation attributes retain source-family ordering and original positional bindings
without interpreting payloads. `#[unspecified]` preserves missing direction independently of
pointer qualifiers and the legacy default.

Bit-field blocks accept `name: LogicalType: width` alongside legacy `name: width`. Logical types,
qualifiers, and raw member annotations survive without adding storage fields; padding is not a
value member.

`#[import("example.dll", name = "NativeValue", kind = data)] static Value: Type;` declares imported
data without a value. Export names or ordinals and import kind survive independently of native
type qualifiers; ordinary constants are unchanged.

Native interfaces opt into `#[vtable(size = N, inherited = M)]` with absolute `#[slot(index)]`
or `#[slot(index, reuse = Owner)]` declarations. Source order and inherited slot identity survive
without fabricated method rows; Rust projection does not yet support the explicit model.

`#[opaque] struct Tag {}` or `#[opaque] union Tag {}` declares a resolved nominal type without
exposed storage. Pointers, aliases, and declaration-only data retain its identity; storage uses
and array elements are rejected.

`type First = Record;` and `type Second = Record;` preserve separate alias names targeting one
nominal record. Aliases retain their targets through RDL/WinMD round-trips without copying fields.

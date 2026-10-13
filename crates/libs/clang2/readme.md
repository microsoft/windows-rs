# windows-clang2

Unpublished native header capture using libclang. The native IR is independent of RDL and WinMD.

```rust,no_run
use windows_clang2::{Input, Target, capture, merge};

let input = Input {
    name: "example.h".into(),
    source: "struct Record { int count; void* data; };".into(),
    roots: vec!["Record".into()],
    ..Default::default()
};
let snapshots = capture(&[input], &[Target::X86, Target::X64, Target::Arm64]).unwrap();
let merged = merge(&snapshots).unwrap();
println!("{} records with target variants", merged.records.len());
```

Supply libclang through `LIBCLANG_PATH` or its loader search paths. `capture` returns one owned
snapshot per target/TU, in target-major order. `resolve` checks global types from the same target
and compiler configuration, retaining original observations and completion decisions.

`Input.arguments` supplies include paths and compiler flags. `Input.roots` selects global type
names and their transitive dependencies within each TU; empty selects all declarations. Missing
roots and unsupported reachable declarations fail. `merge` reconciles each target, then groups native
identities with separate target layouts and type graphs. An unobserved target is not proof of
absence.

Indices are created before parallel capture and reused across TUs. Batches are serialized; other
libclang callers must not create indices during a batch. Native hangs affect the whole process.

Supported: named structs, typedefs, forwards, scalars, qualified pointers, arrays, and observed
layouts. Written alias edges and all captured declaration observations are retained. Compatibility
uses canonical types; typedef names are not nominal types. Attribute evidence is retained, but
unresolved attributes block `resolve` and `merge`. No packing request is guessed from layout.
Unions and functions are not implemented.

The optional `projection` feature provides `projection::generate(inputs, namespace)` for
three-target capture, merge, and per-header RDL output. It projects structs and value typedefs,
checks sequential ABI layout, and rejects unsupported declarations or ambiguous publication names.
`tool-win32-2` supplies the SDK configuration and writes canonical files.

`capture_partial` and `projection::generate_partial` return supported output plus omission reports.
Failed entities and dependent declarations are removed across all targets; compiler errors and
conflicts in retained definitions still fail. A typedef that cannot be published can remain as
native evidence when its canonical type is representable. Macro constants are not captured.

Fixtures, inspection, and scaling live in `test_clang2`. See
[crate notes](../../../docs/crates/windows-clang2.md) for commands.

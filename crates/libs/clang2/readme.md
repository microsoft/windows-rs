# windows-clang2

Unpublished native header capture using libclang, independent of RDL and WinMD.

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
snapshot per target/TU, in target-major order. `resolve` checks global structs from the same target
and compiler configuration, retaining original observations and completion decisions.

`Input.arguments` supplies include paths and compiler flags. `Input.roots` selects global struct
names and their transitive dependencies within each TU; empty selects all declarations. Missing
roots and unsupported reachable declarations fail. `merge` reconciles each target, then groups native
identities with separate target layouts and type graphs. An unobserved target is not proof of
absence.

Indices are created before parallel capture and reused across TUs. Batches are serialized; other
libclang callers must not create indices during a batch. Native hangs affect the whole process.

Supported: named structs, forwards, scalars, qualified pointers, arrays, and observed layouts.
Typedefs, functions, annotations (including opaque compiler attributes), and metadata output are
not implemented. No common cross-target layout or full C++ compatibility is inferred.

Fixtures, inspection, and scaling live in `test_clang2`. See
[crate notes](../../../docs/crates/windows-clang2.md) for commands.

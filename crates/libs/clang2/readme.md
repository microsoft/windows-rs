# windows-clang2

Unpublished native header capture using libclang, independent of RDL and WinMD.

```rust,no_run
use windows_clang2::{Input, Target, capture, resolve};

let input = Input {
    name: "example.h".into(),
    source: "struct Record { int count; void* data; };".into(),
};
let snapshots = capture(&[input], &[Target::X64]).unwrap();
let resolved = resolve(&snapshots).unwrap();
println!("{} record groups", resolved.groups.len());
```

Supply libclang through `LIBCLANG_PATH` or its loader search paths. `capture` returns one owned
snapshot per target/TU, in target-major order. `resolve` checks global structs from the same target
and compiler configuration, retaining original observations and completion decisions.

Indices are created before parallel capture and reused across TUs. Batches are serialized; other
libclang callers must not create indices during a batch. Native hangs affect the whole process.

Supported: named structs, forwards, scalars, qualified pointers, arrays, and observed layouts.
Unsupported declarations fail explicitly. Typedefs, functions, annotations, cross-target merging,
and metadata output are not implemented.

Fixtures, inspection, and scaling live in `test_clang2`. See
[crate notes](../../../docs/crates/windows-clang2.md) for commands.

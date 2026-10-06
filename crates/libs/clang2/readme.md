# windows-clang2

An experimental replacement for `windows-clang`, built directly on
libclang. The existing crate and production generators are unchanged.

The prototype captures selected native declarations and their dependencies into an owned graph,
checks agreement across translation units, and builds a closed RDL projection plan. It retains
separate redeclaration observations, written typedef edges, canonical callable types, native layout,
and annotation origins. Projection cannot bypass native resolution.

```rust,no_run
use windows_clang2::{Input, ProjectionOptions, capture};

let snapshot = capture(
    [Input::new("api.hpp", "struct Packet { int value; };")],
    &["-x", "c++", "--target=x86_64-pc-windows-msvc"],
    &["Packet"],
)?;
let resolved = snapshot.resolve()?;
assert!(resolved.report().incomplete.is_empty());
let plan = resolved.project(&ProjectionOptions::new("Test"))?;
println!("{}", plan.rdl());
# Ok::<(), windows_clang2::Error>(())
```

The caller must make libclang available through `LIBCLANG_PATH` or its normal loader search paths.
Repository tests and the inspection example use the shared pinned dependency helper.

Projection currently covers ordinary C-layout records, fixed-prototype functions, scalar and raw
pointer constants, UUID-bearing local COM interfaces, and explicit external record/interface
bindings. Local interfaces support a single base and pure virtual system-ABI methods. Supported
parameter annotations include direction, optional input pointers, and COM output pointers.
Interface-valued constants have an omission reason in `Plan::omitted`; unsupported projections are
errors. General SAL lowering and production generator integration are not implemented.

The API and diagnostic dump are experimental. See
[`docs/crates/windows-clang2.md`](../../../docs/crates/windows-clang2.md) for the supported subset,
validation contract, and remaining prototype work.

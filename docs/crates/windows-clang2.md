# windows-clang2

An unpublished, libclang-only native capture experiment. See the
[readme](../../crates/libs/clang2/readme.md) for usage.

`runtime` owns compiler sessions, `capture` builds owned IR, `native` wraps compiler queries, and
`resolve` checks same-target observations. `merge` associates checked groups across targets without
collapsing their layouts or type edges. Each target worker reuses a precreated index across TUs.
Snapshots retain declarations separately from entities; resolution borrows them rather than
overwriting evidence. Type and cursor lookup use hash indexes. File paths are interned.

Current scope: global structs, forward completion, native scalars, qualified pointers, arrays, and
observed layouts. Configured inputs select global struct roots and their dependency closure.
Unselected declarations are not encoded, but all compiler errors remain fatal. No typedefs,
functions, annotations, or metadata output yet.
Layout agreement is not proof of identical source packing requests or full C++ compatibility.

Fixtures and callers live in `crates/tests/libs/clang2`, using only the public API. Add headers
under `input` and assertions under `tests`. The runner provides compact inspection and synthetic
scaling:

```text
cargo test -p test_clang2
cargo run -p test_clang2 -- inspect shared_a shared_b
cargo run --release -p test_clang2 -- scale
cargo test -p test_clang2 --test sdk -- --ignored
```

The SDK case uses the `tool-win32` SDK pin and cached clang resource headers (or
`CLANG_RESOURCE_DIR`).
It captures `_LIST_ENTRY` through a root including `windows.h`, across two TUs and three targets.
The attributed `_GUID` remains an explicit rejection case, not silently reduced to its fields.

The scaling report separates capture time, merge time, and comparison counts. Owned-byte
estimates cover snapshot capacities, not native memory, allocator overhead, or resolution tables.

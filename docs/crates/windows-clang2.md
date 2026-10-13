# windows-clang2

An unpublished, libclang-only native capture experiment. See the
[readme](../../crates/libs/clang2/readme.md) for usage.

`runtime` owns compiler sessions, `capture` builds owned IR, `native` wraps compiler queries, and
`resolve` checks same-target observations. Each target worker reuses a precreated index across TUs.
Snapshots retain declarations separately from entities; resolution borrows them rather than
overwriting evidence. Type and cursor lookup use hash indexes. File paths are interned.

Current scope: global structs, forward completion, native scalars, qualified pointers, arrays, and
observed layouts. No typedefs, functions, annotations, cross-target merge, or metadata output yet.
Layout agreement is not proof of identical source packing requests or full C++ compatibility.

Fixtures and callers live in `crates/tests/libs/clang2`, using only the public API. Add headers under
`input` and assertions under `tests`. The runner provides compact inspection and synthetic scaling:

```text
cargo test -p test_clang2
cargo run -p test_clang2 -- inspect shared_a shared_b
cargo run --release -p test_clang2 -- scale
```

The scaling report separates capture time, resolution time, and comparison counts. Owned-byte
estimates cover snapshot capacities, not native memory, allocator overhead, or resolution tables.

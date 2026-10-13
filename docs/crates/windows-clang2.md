# windows-clang2

An unpublished, libclang-only native capture experiment. See the
[readme](../../crates/libs/clang2/readme.md) for usage.

`runtime` owns compiler sessions, `capture` builds owned IR, `native` wraps compiler queries, and
`resolve` checks same-target observations. `merge` associates checked groups across targets without
collapsing their layouts or type edges. Each target worker reuses a precreated index across TUs.
Snapshots retain declarations separately from entities; resolution borrows them rather than
overwriting evidence. Type and cursor lookup use hash indexes. File paths are interned.

Global structs and typedefs retain written and canonical types, attributes, and observed layouts.
Typedef USRs contain filenames, so global typedef correspondence uses names followed by canonical
type checks. Configured roots include their dependency closure. All compiler errors remain fatal.
Unresolved attributes block reconciliation even for a single observation: matching layouts or
matching opaque cursor kinds do not prove identical packing/alignment requests.

Fixtures and callers live in `crates/tests/libs/clang2`, using only the public API. Add headers
under `input` and assertions under `tests`. The runner provides compact inspection and synthetic
scaling:

```text
cargo test -p test_clang2
cargo run -p test_clang2 -- inspect shared_a shared_b
cargo run --release -p test_clang2 -- scale
cargo test -p test_clang2 --test sdk -- --ignored
cargo run --release -p test_clang2 -- sdk-gate
cargo run --release -p test_clang2 -- sdk-scale 32
```

The SDK case uses the `tool-win32` SDK pin and cached clang resource headers (or
`CLANG_RESOURCE_DIR`).
`sdk-gate` checks 26 SDK roots on all three targets and exits unsuccessfully if any are blocked.
`sdk-scale` measures 15 supported roots with 2, 8, or 32 TUs, checking comparison-work bounds.
Opaque `_GUID` and pragma-packing attributes remain recorded evidence, not verified semantics.

The scaling report separates capture time, merge time, and comparison counts. Owned-byte
estimates cover snapshot capacities, not native memory, allocator overhead, or resolution tables.

# Reactor benchmark

This benchmark measures Reactor retained updates, component scheduling, lifecycle work,
virtualization, and memory attribution without WinUI layout or rendering.

```text
cargo run -p test-reactor-bench --release --quiet
cargo run -p test-reactor-bench --release --quiet -- --count 4096 --samples 40 --batch 4
cargo run -p test-reactor-bench --release --quiet -- --architecture-gate
cargo run -p test-reactor-bench --profile profiling --quiet -- --retained-memory --count 512
```

The output includes median and p95 update time, allocated bytes and allocation count per update,
retained bytes per object, isolated component updates, and retained component-scope memory.
Component rows also cover isolated and all-consumer typed-context invalidation, same-type updates,
and retained-root type replacement.

The recursive component rows build balanced trees, target the deepest leaf, and report retained
bytes per scope, update latency, allocations, allocated bytes, and mutation count. The timed
updates disable recording-adapter batch validation because that diagnostic intentionally clones
the complete adapter state.

The `--architecture-gate` mode uses six samples by default and reports:

- Keyed no-change and one-property-change updates across two 50,000-object graphs in one timed
  operation. Reactor enforces a 65,536-object limit per graph.
- Controlled 513-node memory fixtures that separate declaration construction, retained
  graph storage, string properties, events, TreeNode relation shape, and recording-adapter storage.
- Root-type replacement and middle-child removal in a 16,384-component graph.

Use `--gate-samples` or `--component-count` to override the gate defaults.

The `--retained-memory` mode releases benchmark scratch buffers after mounting and reports live
retained-graph allocations separately from declarations and adapter state. It prints release
layout sizes and additive slot, relation, child-vector, property/event, virtual, and retirement
storage for keyed and unkeyed flat trees, a deep chain, relation-heavy controls, TreeNode
structural objects, a virtual source, and an active retirement.

No-op allocation, changed-update allocation, retained memory, and large-tree scaling are separate
gates.

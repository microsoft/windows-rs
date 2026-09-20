# Reactor2 comparison benchmark

This benchmark runs matching retained-update workloads through `windows-reactor` and the
`windows-reactor2` prototype.

```text
cargo run -p test-reactor2-bench --release --quiet
cargo run -p test-reactor2-bench --release --quiet -- --count 4096 --samples 40 --batch 4
```

The output includes median and p95 update time, allocated bytes and allocation count per update,
retained bytes per object, isolated Reactor2 component updates, and retained component-scope
memory. Component rows also cover isolated and all-consumer typed-context invalidation. Results
compare the recording backends and do not include WinUI layout, rendering, or COM allocation
costs.

The benchmark intentionally reports regressions as well as improvements. Reactor2 must not be
accepted based only on lower source complexity or faster reorder cases; no-op allocation, changed
update allocation, retained memory, and large-tree scaling are separate gates.

# Reactor headless benchmarks

This benchmark measures Reactor retained updates, component scheduling, lifecycle work,
virtualization, and memory attribution without WinUI layout or rendering.

Keep correctness coverage in `windows-reactor` and [`test-reactor`](../reactor/readme.md).
The separate executable isolates the counting allocator and release profiling from functional tests.
The live Rust/C# comparison is [`test_reactor_perf`](../reactor_perf/readme.md).

```text
cargo run -p test-reactor-bench --release --quiet
cargo run -p test-reactor-bench --release --quiet -- --count 4096 --samples 40 --batch 4
cargo run -p test-reactor-bench --release --quiet -- --architecture-report
cargo run -p test-reactor-bench --profile profiling --quiet -- --retained-memory --count 512
```

These commands report measurements; they do not enforce performance thresholds or compare against
a baseline. The scheduled/manual Reactor CI job runs `--architecture-report` and writes results to
the job log. Compare release builds on the same idle machine when evaluating a change.

## Workloads

Select at most one mode. With no mode, the runner measures updates, component scheduling,
retirement, virtualization, retained memory, and recursive component scaling.

| Mode | Measurements |
| --- | --- |
| `--architecture-report` | Large keyed updates, memory attribution, component replacement/removal |
| `--retained-memory` | Retained graph storage by shape and allocation category |
| `--virtual-memory` | A 10,000-item source with eight realized rows |
| `--virtual-items` | Source creation/replacement, realization, updates, recycling, memory, mutations |
| `--keyed-scaling` | No-change and one-property updates at 512, 1,024, and 10,000 children |
| `--profile-workload NAME` | One workload repeated for a profiler |

`--samples` defaults to 80, or six for `--architecture-report`. `--batch` defaults to eight
operations per sample for workloads that support batching. `--count` defaults to 512 children for
the default suite and retained-memory fixtures. Numeric options must be positive integers.

Profiling workload names are `reactor-100k-no-change`, `reactor-100k-one-changed`,
`reactor-context-broad`, and `reactor-virtual-source-replace`. Use `--profile-iterations` to override
the iteration count, which defaults to `--samples`. `--component-count` defaults to 16,384 for
the architecture report and broad-context profiling.

## Measurement details

The output includes median and p95 update time, allocated bytes and allocation count per update,
retained bytes per object, isolated component updates, and retained component-scope memory.
Component rows also cover isolated and all-consumer typed-context invalidation, same-type updates,
and retained-root type replacement.

The recursive component rows build balanced trees, target the deepest leaf, and report retained
bytes per scope, update latency, allocations, allocated bytes, and mutation count. The timed
updates disable recording-adapter batch validation because that diagnostic intentionally clones
the complete adapter state.

The `--architecture-report` mode reports:

- Keyed no-change and one-property-change updates across two 50,000-object graphs in one timed
  operation. Reactor enforces a 65,536-object limit per graph.
- Controlled 513-node memory fixtures that separate declaration construction, retained
  graph storage, string properties, events, TreeNode relation shape, and recording-adapter storage.
- Root-type replacement and middle-child removal in a 16,384-component graph.

The `--retained-memory` mode releases benchmark scratch buffers after mounting and reports live
retained-graph allocations separately from declarations and adapter state. It prints release
layout sizes and additive slot, relation, child-vector, property/event, virtual, and retirement
storage for keyed and unkeyed flat trees, a deep chain, relation-heavy controls, TreeNode
structural objects, a virtual source, and an active retirement.

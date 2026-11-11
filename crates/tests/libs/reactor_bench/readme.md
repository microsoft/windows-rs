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
cargo run -p test-reactor-bench --release --quiet -- --component-work --count 1024
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
| `--component-work` | Untimed component-update and virtual-key work for `--count` items |
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

`--component-work` includes message delivery, `view` construction, expansion, and retained updates,
unlike the keyed-update rows that reuse prebuilt declarations. It reports one update after a warmup:
a no-op message, one label changed in a large component, equal input, an unchanged large child
before and after its own update, one label owned by a small child, and changes to used/unused fields
of a context. The context cases pass through an unchanged component that does not read the context.
Component-heavy cases revisit a component per child or tooltip, and update one leaf among sibling
components. The root-attachment case has one component-backed tooltip regardless of `--count`.
A second table counts scope and pending-render visits during window publication.

Virtual-key cases isolate source mount, unchanged revision, new revision with equal keys,
reordering, single-key churn, and resizing. They use index-derived numeric keys without allocating
an application snapshot or realizing rows. The resize case warms with a shrink, then measures
growth back to `--count`. Their view/expansion/window columns are zero because they use the retained
runtime directly, not a component host.

Work columns count calls to component views, declaration expansion, object validation,
retained-state comparison, object reconciliation, and root-reference refresh. They count visits,
not distinct objects, property comparisons, or all host bookkeeping. Synthetic attachment wrappers
are not counted unless they enter the corresponding traversal. Counts are collected on the calling
thread through the `test` feature; normal library builds have no counting hooks. Allocation columns
use the existing process-wide allocator and exclude setup and report formatting. The adapter does
no native work. This mode reports no elapsed time and does not measure WinUI layout or rendering.
Timed benchmark modes still include inactive counting checks, so timing comparisons require
matching instrumentation in both builds.

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

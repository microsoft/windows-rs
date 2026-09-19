# windows-reactor2

`windows-reactor2` is an experimental crate for evaluating a replacement internal model for
`windows-reactor`. It is not a supported application framework or a published crate.

## Pipeline

The prototype separates three concerns:

```text
typed declarations -> retained generational arena -> generic mutation batch -> runtime adapter
```

The public declaration types constrain relation shapes. For example, a `Border` accepts one visual,
a `Grid` accepts keyed visual children, and a `TreeView` accepts keyed `TreeNode` values. A
`TreeNode` can own visual content and nested `TreeNode` values, but it cannot be inserted directly
into a visual-child relation.

`tool-reactor2` reads `crates/tools/reactor2/src/schema.toml` and generates object, property, and
relation identifiers plus their contracts. The schema records:

| Contract field | Meaning |
| --- | --- |
| category | Visual, structural, or data object |
| cardinality | Zero-or-one or ordered-many relation |
| identity | Positional or keyed reconciliation |
| realization | Owned visual, structural native object, or generated container |

Declarations are validated and reconciled directly into one retained generational arena. There is
no transient flattened object graph and no per-update clone of the retained graph. Common empty and
single-value declaration lists are stored without a `Vec`, retained relations own their child
vectors directly, and the runtime reuses bounded mutation storage. Every object passes through the
same property and relation algorithms, which emit generic mutations such as `Attach`, `Insert`,
`Remove`, and `Reorder`.

Desired declarations and retained objects use the same generated schema and payload types, but are
different generations of the model. Declarations use recursive ownership for natural builder
composition. Retained relations use stable generational `ObjectId` values needed by native and
lifecycle state. Reconciliation moves between those forms directly rather than introducing a
third representation.

The recording adapter validates ownership and relation categories. The WinUI adapter maps the same
mutations to `Border.Child`, panel children, `TreeViewNode` collections, and `ListView.Items`.
TreeView's item template and safe collection synchronization remain private to that adapter. The
`test-reactor2-selftest` executable opens a real Reactor2 window and repeatedly reorders realized
TreeView nodes with custom visual content.

## Current thin slice

| Object | Purpose |
| --- | --- |
| `TextBlock` | Scalar property updates |
| `Border` | One owned visual |
| `Grid` | Ordered keyed visual children |
| `StackPanel` | Ordered positional visual children |
| `TreeView` and `TreeNode` | Recursive keyed structural objects with owned visual content |
| `ListView` and `DataItem` | Keyed data with native container generation |

Virtualized realization is intentionally excluded until the eager and container-generated paths
are proven. A count and revision without a realization protocol would not validate virtualization.

## Validation

Run:

```text
cargo run -p tool-reactor2 --quiet
cargo test -p windows-reactor2 -p tool-reactor2 --quiet
cargo clippy -p windows-reactor2 -p tool-reactor2 -p test-reactor2-bench \
    --all-targets -- -D warnings
cargo run -p test-reactor2-bench --release --quiet
cargo run -p test-reactor2-selftest --quiet
cargo run -p test-reactor-bench --bin reactor-live-compare --release --quiet -- \
    --frontend reactor2 --workload text --count 512 --updates 120
```

`test-reactor2-bench` compares Reactor and Reactor2 using the same node counts and update patterns.
It reports median and p95 time, allocated bytes, allocation count, and retained bytes per object.

An optimized 4,096-object recording run after direct declaration-to-arena reconciliation produced:

| Workload | Reactor median | Reactor2 median | Reactor bytes/op | Reactor2 bytes/op |
| --- | ---: | ---: | ---: | ---: |
| Keyed no change | 377.1 us | 24.2 us | 4 | 4 |
| One keyed value changed | 1,043.1 us | 223.6 us | 808,234 | 606,524 |
| Rotate keyed children | 1,602.9 us | 671.9 us | 1,619,284 | 1,114,344 |
| Reverse keyed children | 2,400.8 us | 523.1 us | 1,955,032 | 1,171,636 |
| Rotate TreeView roots | 786.9 us | 594.5 us | 875,532 | 1,114,344 |

Retained-memory rows now separate the arena from `RecordingAdapter`, which intentionally mirrors
the object and ownership graph for validation:

| Shape | Reactor | Reactor2 graph | Reactor2 with recording adapter |
| --- | ---: | ---: | ---: |
| Keyed visual children | 1,750.5 bytes/object | 218.0 bytes/object | 454.0 bytes/object |
| Plain TreeView nodes | 204.2 bytes/object | 282.0 bytes/object | 666.0 bytes/object |
| TreeView nodes with visual content | - | 245.0 bytes/object | 551.0 bytes/object |

The graph-only rows are the internal representation gate. Recording-adapter totals are useful for
test-process sizing but are not retained arena cost.

The recording results are not sufficient for an architecture decision. `reactor-live-compare`
runs each frontend in a separate process with the same native `Grid`, 512 keyed `TextBlock`
children, deterministic mutations, 120 render-paced updates, allocator, and process metrics.
Stabilized native runs with 512 objects and 300 render-paced updates produced:

| Workload | Reactor avg | Reactor2 avg | Reactor bytes/update | Reactor2 bytes/update |
| --- | ---: | ---: | ---: | ---: |
| Change one text value | 627.4 us | 296.9 us | 216,904 | 178,615 |
| Rotate one child | 725.6 us | 472.4 us | 316,449 | 229,589 |
| Reverse all children | 3,835.6 us | 1,500.0 us | 411,641 | 236,705 |
| Remove/restore 64 children | 1,083.1 us | 891.3 us | 352,023 | 167,953 |

The initial native run failed badly because every reorder used
`UIElementCollection::ReplaceAll` and each declaration object allocated empty property and relation
vectors. Replacing dense synchronization with a generic move plan and storing zero-or-one
properties and relations inline removed those costs. Direct declaration reconciliation then
removed the transient object graph and full retained-graph clone. Reactor2 is faster in all four
average-latency workloads while allocating fewer objects and bytes. Churn p95 remains slightly
worse and is still a tail-latency optimization gate.

Phase measurements attribute a text update to about 128 us of declaration construction and 153 us
of runtime work, including about 33 us in the native adapter. The public declaration layer accounts
for about 1,026 allocations per update, while the runtime accounts for about 11. This makes
generated declaration construction the clearest remaining allocation target. Churn is dominated by
the 160-operation native batch; future work should preserve the generic relation contract while
letting each backend select an efficient batch operation.

## Prototype limits

- Component scopes, events, resources, transitions, and asynchronous work are not represented.
- The native stress fixture covers TreeView custom-content updates and reorder, but not input or
  accessibility behavior.
- Validation rejects declaration depth above 128 before reconciliation so recursive planner paths
  cannot overflow the stack.
- Validation and no-change matching reject graphs above 65,536 objects.
- Adapter validation or application failures poison the runtime and clear retained state. Dynamic
  declaration errors are rejected before retained or native mutation.
- Component identity, scheduling, effects, references, virtualization, and full native lifecycle
  behavior remain outside the thin slice and must be proven without adding parallel object models.
- Native child churn still has worse p95 latency than Reactor even though its average time, CPU,
  allocations, and allocated bytes are lower.

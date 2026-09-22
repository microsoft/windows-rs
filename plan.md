# Reactor2 architecture review and project status

Date: 2026-09-21

Branch: `reactor3`

Repository state: dirty and uncommitted. Do not discard or partially regenerate files without first
reading this report and inspecting `git status`.

## Purpose

Reactor2 is an architecture prototype intended to replace Reactor's planner-heavy retained UI model
with a smaller schema-driven design:

```text
typed declarations -> retained generational graph -> generic mutation batch -> runtime adapter
```

The architecture decision gate is complete and accepts Reactor2 as the replacement direction. It is
not yet complete API or behavioral parity. Typed imperative references and declaration-owned
TitleBar attachment are complete; the next host-owned lifecycle contracts are ToolTip and
ContentDialog.

## Current conclusion

Early results favor Reactor2:

- It represents all 79 old Reactor control classes.
- Its runtime and generator contain about 39% fewer physical lines than Reactor.
- Its generated runtime output is about 20% smaller.
- Its handwritten runtime surface is much smaller than Reactor's.
- Retained update benchmarks are substantially faster in the measured workloads.
- Exact no-change updates allocate nothing.
- Isolated changed-property allocation remains constant through 10,000 objects.
- Keyed retained memory is substantially lower.
- The generator now expresses shared properties, events, relations, selection, feedback, layout,
  retirement, and virtualization without per-control planner implementations.

The remaining work is completeness rather than an architecture blocker:

- Property and event parity remains incomplete.
- All capability contracts are represented; two lifecycle/placement contracts remain.
- Native ItemsRepeater uses a count-backed WinRT source and boxes indices only when WinUI requests
  them.
- Tree and component retained-memory costs are measured and explained below.
- Binary size and sampled CPU results favor Reactor2 or identify bounded future optimizations.
- Matched real-WinUI startup reaches the first composition rendering callback in nearly the same
  median time: 183.57 ms for Reactor and 184.64 ms for Reactor2.
- The live selftest passes the new fixtures but cannot complete the real pointer fixture in the
  current desktop session because the test window is not foreground.
- Dense owned-child reorder now preserves retiring slots and performs near Reactor's native apply
  time at 512 and 1,024 children.

## Decision-gate progress

The milestone described below was committed as `8e60986672` on branch `reactor3`.

Architecture review started after that commit. Do not add parity surface until the blockers in this
section are fixed and reviewed.

### Repeatable compile measurements

Six isolated checks were run in alternating package order:

| Package | Approximate median | Fresh target output |
| --- | ---: | ---: |
| `windows-reactor` | 6.47 s | 170.2 MB |
| `windows-reactor2` | 5.13 s | 130.2 MB |

At this checkpoint Reactor2 checks about 21% faster and produces 23.5% less fresh target output.
Alternating order removed the large order bias seen in the earlier single-run measurement.

Provisional isolated release-target totals were:

| Package | Release build | Target output |
| --- | ---: | ---: |
| `windows-reactor` | 11.86 s | 51.6 MB |
| `windows-reactor2` | 8.89 s | 35.1 MB |

Exact release `.rlib` sizes:

| Package | Release `.rlib` |
| --- | ---: |
| `windows-reactor` | 15,462,520 bytes |
| `windows-reactor2` | 10,325,718 bytes |

The Reactor2 release library is 33.2% smaller in this measurement. Cargo did not emit a separate
top-level `.rmeta` artifact for these release library builds.

### Handwritten architecture audit

The retained-graph undo journal appears complete, including root, slot, free-list, generation, and
retirement-map rollback. The audit found blockers outside that journal:

1. Component input, message, and context state can be committed before declaration/native success.
   A recoverable graph error can leave component state describing UI that never committed.
2. Public `ComponentHost::runtime_mut` permits graph changes that bypass scope, component,
   reference, task, effect, virtual-row, and context-consumer ownership.
3. Public event-by-event dispatch can pop and discard pending virtual realization/recycle work.
4. Returned errors after effect cleanup/setup or retirement work must not leave a reusable
   divergent host.
5. Runtime poisoning can discard the graph while externally held nested `ElementRef` cells still
   contain stale object IDs.

Current bounded fix order:

1. Preserve virtual work in every public event API.
2. Remove unrestricted mutable runtime access from `ComponentHost`.
3. Clear every retained reference through one poison path.
4. Make component input/message/context operations transactional or poison on divergence.
5. Keep panic behavior simple: no custom unwind recovery or abort wrappers.
6. Re-run the architecture audit before CPU, startup, and native-memory measurements.

### Architecture gate pass 1

Completed after the initial audit:

- `Runtime::next_native_event` peeks virtual realization/recycle/cancel work instead of consuming
  work it cannot dispatch.
- Public `ComponentHost::runtime_mut` was removed.
- Public mutable adapter access was removed from both `ComponentHost` and `Runtime`.
- Narrow host/runtime operations now cover event-waker configuration, recording diagnostics,
  ordered test input, retirement completion, focus, and read-only adapter inspection.
- Compile-fail API tests reject mutable adapter extraction and arbitrary adapter `apply` or
  `pop_native_event` calls.
- Runtime poisoning now clears nested declaration references and virtual-row references before
  graph discard.

Validation at this checkpoint:

- 129 unit tests.
- 1 integration test.
- 72 doctests.
- All Reactor2 samples, selftest, and benchmark targets compile.
- Strict Clippy with `-D warnings` and `git diff --check` pass.

This pass is complete; component input/message/context returned errors now invalidate the host.

Panic policy: do not add panic wrappers. FFI panics abort naturally when unwinding cannot cross the
boundary. Internal Rust failures panic normally. Transaction and poison handling applies to ordinary
returned errors, not panics.

### Architecture gate pass 2

Completed after pass 1:

- Component input/message/context mutations followed by a returned graph or adapter error now
  invalidate the complete host.
- Invalidation clears scopes, virtual rows, effects, tasks, references, contexts, consumers,
  retained state, pending messages, and wakers.
- `MessageQueue` has an explicit closed state. Stale senders, completions, callbacks, and controlled
  task sends reject delivery after invalidation and release queued payloads.
- Natural Rust and FFI panic behavior is unchanged. No custom panic wrapper or caught-unwind
  recovery remains.

Validation at this checkpoint:

- 135 unit tests.
- 1 integration test.
- 72 doctests.
- All samples, selftest, and benchmark targets compile.
- Strict Clippy with `-D warnings` and `git diff --check` pass.

The post-fix handwritten architecture re-audit found no high-confidence correctness blocker.
Remaining findings are measurement candidates:

- Eager native ItemsRepeater index objects.
- Full retained-reference collection on committed updates.
- Linear `RetainedGraph::owner` lookup.
- Linear native selection-item lookup.
- Visibility of public WinUI simulation helpers.
- Consolidation of centralized handwritten backend exceptions.

### Scale and live measurement progress

At 100,000 keyed objects, measured as two 50,000-object graphs because one graph is capped at
65,536 objects:

| Workload | Reactor median | Reactor2 median | Reactor2 allocation |
| --- | ---: | ---: | ---: |
| No change | 14.30 ms | 1.45 ms | 0 |
| One property changed | 30.24 ms | 3.61 ms | 1,140 bytes, 8 allocations |

The original 513-entry Tree comparison was not a precise graph-to-graph measurement. The
Reactor2 fixture was built before the allocator baseline, so allocations transferred from the
declaration into the retained graph were excluded. The Reactor fixture also represents the 512
logical TreeNode values behind only four retained graph nodes rather than 513 independently
reconciled objects.

| Reactor2 category | Bytes |
| --- | ---: |
| Fixture-held declarations | 102,440 |
| Retained graph | 127,160 |
| Recording adapter | 233,748 |
| Fixture + recorded runtime | 463,348 |

Reactor's fixture still measures 109,718 bytes, but its pump contains four retained graph nodes for
the TreeView while the 512 TreeNode values are carried as collection data. It is not an equivalent
per-object retained representation. Reactor2's corrected graph-only result is 15.9% larger than
that whole Reactor fixture and retains 513 generic objects with individual identities, properties,
and relations.

#### Retained graph memory

Repeatable command:

```text
cargo run -p test-reactor2-bench --profile profiling --quiet -- \
    --retained-memory --count 512
```

The mode constructs declarations after the allocator baseline, mounts through a null adapter,
releases runtime validation and mutation scratch, and checks the remaining allocation against
graph capacities. Empty controlled fixtures have zero unattributed bytes.

The release layout is:

| Structure or field | Bytes |
| --- | ---: |
| `RetainedGraph` | 112 |
| `RetainedSlot` | 152 |
| `RetainedObject` | 144 |
| `ObjectType` | 1 |
| `Option<Key>` | 24 |
| `Option<ElementRef>` | 8 |
| `Option<ExitTransition>` | 16 |
| Retained `SharedList<Property>` | 48 |
| Retained event pointer | 8 |
| Relation `Vec` header | 24 |
| Optional virtual-items pointer | 8 |
| `RetainedRelation` | 32 |
| `RetainedRelationValue` | 24 |
| `ObjectId` | 8 |
| `RetainedVirtualItems` | 184 |
| `RetainedRetirement` | 32 |
| Empty `HashMap` header | 48 |

The slot's eight bytes beyond `RetainedObject` hold generation/retirement state and padding.
Within each object, the largest always-present fields are the 48-byte inline property list, the
24-byte optional key, and the 24-byte relation-vector header. Empty free lists, retirement maps,
and optional virtual state allocate no heap memory.

The validator already counts every declaration object. `Runtime::update` now uses that count to
reserve the empty graph's slot vector exactly before the first mount. This changes the 513-object
slot capacity from 1,024 to 513 and removes 77,672 bytes of unused slot storage:

| Tree graph measurement | Bytes |
| --- | ---: |
| Correct graph-only result before exact reservation | 204,832 |
| Graph-only result after exact reservation | 127,160 |
| Reduction | 77,672 (37.9%) |

The earlier 136,200-byte number is retained in historical benchmark output only; it excluded
declaration-owned allocations while including runtime scratch and cannot be used for additive
attribution.

Representative post-fix shapes:

| Shape | Objects | Bytes | Bytes/object | Slots | Relations | Child IDs | Payload/other |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Flat keyed empty Border children | 513 | 98,488 | 192.0 | 77,976 | 16,416 | 4,096 | 0 |
| Flat unkeyed empty Border children | 513 | 98,488 | 192.0 | 77,976 | 16,416 | 4,096 | 0 |
| Flat one-string-property children | 513 | 110,776 | 215.9 | 77,976 | 16,416 | 4,096 | 12,288 |
| Flat one-event children | 513 | 143,544 | 279.8 | 77,976 | 16,416 | 4,096 | 45,056 |
| Deep one-child chain | 128 | 23,552 | 184.0 | 19,456 | 4,096 | 0 | 0 |
| Empty NavigationView relation sets | 513 | 180,408 | 351.7 | 77,976 | 98,336 | 4,096 | 0 |
| Filled NavigationView one-child slots | 641 | 139,448 | 217.5 | 97,432 | 40,992 | 1,024 | 0 |
| TreeView plus structural TreeNodes | 513 | 127,160 | 247.9 | 77,976 | 32,800 | 4,096 | 12,288 |

The same shapes at 65 and 1,025 objects remain linear. Flat empty storage stays about 192
bytes/object and Tree structural storage stays about 247-248 bytes/object. Numeric keyed and
unkeyed flat fixtures are identical because `Option<Key>` is inline and integer keys have no heap
payload. One short string property adds 24 bytes/object. One event adds 88 bytes/object for the
retained event vector and callback allocation.

The Tree breakdown is exact: 77,976 slot bytes, 32,800 relation-record bytes, 4,096 child-ID bytes,
and 12,288 bytes of short string payload/Rc allocation. The 512 TreeNode leaves each retain two
schema relation records (`Content` and `Children`) even when both are empty. This accounts for
32,768 bytes and is the largest remaining reducible category.

Virtual and retirement state remain pay-for-use:

- A 10,000-item, zero-realized-row virtual source uses one 152-byte slot, one 32-byte relation,
  the 184-byte virtual sidecar, and 160,080 bytes for the existing stable-key vector and its Rc
  owners. Its two empty hash maps allocate no buckets.
- A three-object active-retirement fixture uses 780 bytes. The retirement node vector has capacity
  four; the remaining small residual contains the retirement hash bucket plus the fixture's key
  and string payload.
- The free-list vector remains unallocated until slots are retired.

No sparse-relation change was made. Omitting empty relation states could save 32,768 bytes in this
Tree shape and 98,304 bytes across 512 empty NavigationView controls, but it would require missing
relations to behave as typed defaults throughout matching, mutable transaction access,
virtualization, retirement, selection, and rollback. That is a separate representation change,
not a safe layout-only optimization. The current remaining cost is explained and acceptable for
the generic per-object semantics.

Exact measurement commands:

```text
cargo run -p test-reactor2-bench --profile profiling --quiet -- \
    --retained-memory --count 64
cargo run -p test-reactor2-bench --profile profiling --quiet -- \
    --retained-memory --count 512
cargo run -p test-reactor2-bench --profile profiling --quiet -- \
    --retained-memory --count 1024
```

Post-change regression gates:

| Gate | Result |
| --- | --- |
| Reactor2 100k no-change | 1.471 ms median, 0 bytes/op |
| Reactor2 100k one-change | 3.864 ms median, 1,140 bytes and 8 allocations/op |
| 16,384-consumer broad context | 21.631 ms median, 28.397 ms p95 |
| 10k virtual source replacement | 373.4 us median, 402.9 us p95 |
| Live reverse, 512 children | Reactor2/Reactor 1.059x |
| Live reverse, 1,024 children | Reactor2/Reactor 1.020x |
| Live reverse Reactor2 scaling | 2.199x |

Commands:

```text
cargo run -p test-reactor2-bench --profile profiling --quiet -- \
    --architecture-gate --gate-samples 6 --component-count 16384
cargo run -p test-reactor2-bench --profile profiling --quiet -- \
    --profile-workload reactor2-context-broad --profile-iterations 400 \
    --component-count 16384
cargo run -p test-reactor2-bench --profile profiling --quiet -- \
    --virtual-items --samples 20 --batch 1
cargo run -p test-reactor-bench --bin reactor-live-compare --release --quiet -- \
    --gate-reverse
```

Validation after the retained-memory change:

- 143 `windows-reactor2` unit/integration tests passed.
- 72 doctests passed.
- Reactor2 selftest, benchmark, Counter, Explorer, Solitaire, and Workbench targets compile.
- Strict all-target Clippy passed for `windows-reactor2`, `windows-reactor`, and
  `test-reactor2-bench`.

The matched live WinUI grid comparison found:

- Text updates: Reactor2 181 us average versus Reactor 423 us.
- One-child rotation: Reactor2 363 us versus Reactor 468 us.
- Churn: Reactor2 981 us versus Reactor 1,020 us, with a higher Reactor2 p95.
- Dense reverse originally measured Reactor2 20.2 ms versus Reactor 4.0 ms at 512 children.

The dense reverse regression is fixed. `WinUiAdapter::reorder` snapshots COM identities once,
simulates active-child swaps in Rust, and applies native remove/insert operations in lockstep.
Retiring children remain at their exact physical indices while active children reorder through
active slots. The live fixture verifies complete physical identity order before retirement
completion and active order afterward.

The steady-state release gate resets Reactor2 adapter metrics after initial mount and asserts that
the native apply sample count equals the requested update count. An independent run measured:

| Children | Reactor | Reactor2 | Reactor2/Reactor |
| ---: | ---: | ---: | ---: |
| 512 | 3,586 us | 4,027 us | 1.123x |
| 1,024 | 8,067 us | 8,301 us | 1.029x |

Reactor2 scaling from 512 to 1,024 children was 2.061x. The ratio and scaling gates pass.

### Repeated native lifecycle

The lifecycle harness repeatedly creates a Reactor2 runtime, mounts a 10,000-item virtual source,
realizes eight rows, updates the source, activates a window, waits for a rendering callback, closes
the window, and waits for the real `Closed` event before dropping the runtime and starting the next
cycle. Waiting for `Closed` is required: sampling and remounting immediately after `Window::Close`
measured asynchronous WinUI teardown as retained memory.

The gate alternates empty-window and loaded-window child processes. Five 40-cycle pairs reported:

| Measurement | Empty window | Loaded window | Loaded excess |
| --- | ---: | ---: | ---: |
| Private-byte slope/cycle | 37,295 | 53,032 | 15,737 |
| Working-set slope/cycle | 40,960 | 98,304 | 57,344 |
| Rust live-byte slope/cycle | 0 | 0 | 0 |

The gate allows at most 128 KiB/cycle of loaded excess for private bytes and working set and requires
zero Rust live-byte slope. It passes. A separate loaded 120-cycle run reported 63,453 private bytes
and 79,282 working-set bytes per cycle with zero Rust slope, consistent with bounded WinUI process
caching rather than retained Reactor2 Rust state.

Run:

```text
cargo run -p test-reactor-bench --bin reactor-live-compare --release --quiet -- \
    --gate-native-lifecycle
```

## Exact parity status

Run:

```text
cargo run -p tool-reactor2 --quiet -- --parity-report
```

Current result:

| Contract | Mapped | Total | Missing |
| --- | ---: | ---: | ---: |
| Controls | 79 | 79 | 0 |
| Properties | 191 | 233 | 42 |
| Events | 37 | 68 | 31 |
| Slots | 42 | 42 | 0 |
| Selections | 3 | 3 | 0 |
| Capabilities | 158 | 158 | 0 |
| Lifecycle/placement | 0 | 2 | 2 |

`--check-parity` must continue to fail until every old contract is represented exactly.

## Source size

Physical line counts at this checkpoint:

| Surface | Reactor | Reactor2 | Change |
| --- | ---: | ---: | ---: |
| Runtime crate Rust | 96,139 | 57,438 | -40.3% |
| Generated runtime | 51,689 | 41,212 | -20.3% |
| Generator plus schema | 10,991 | 7,721 | -29.8% |
| Runtime plus generator | 107,130 | 65,159 | -39.2% |

The Reactor2 runtime total includes its large in-crate test module. The generated total includes:

- `generated.rs`
- `generated_declarations.rs`
- `native/generated.rs`
- `native/bindings.rs`

The current dirty diff is approximately:

```text
21 files changed, 14,185 insertions, 11,468 deletions
```

This net diff is smaller than the amount of new behavior because shared property-contract lookup,
declaration macros, dead getter pruning, and minimal binding generation removed substantial earlier
generated output.

## Compile measurements

Single isolated fresh-target measurement:

| Package | Fresh `cargo check` | Target output |
| --- | ---: | ---: |
| `windows-reactor` | 7.81 s | 162.3 MB |
| `windows-reactor2` | 7.53 s | 124.2 MB |

Interpretation:

- Reactor2 produced 23.5% less target output.
- The measured fresh check was only 3.6% faster.
- One sequential run is sensitive to filesystem and compiler cache order.
- Repeat this in alternating order over several runs before claiming a compile-time win.

Warm no-op checks measured about 0.37 s for Reactor and 0.34 s for Reactor2 earlier in the session.
That difference is too small to treat as meaningful.

## Retained update performance

Current release benchmark:

```text
cargo run -p test-reactor2-bench --release --quiet
```

### Shared Reactor/Reactor2 workloads

| Workload, 512 objects | Reactor median | Reactor2 median | Reactor2 bytes/op |
| --- | ---: | ---: | ---: |
| Keyed no change | 48.15 us | 4.73 us | 0 |
| One property changed | 115.96 us | 20.06 us | 1,140 |
| Rotate one keyed child | 165.55 us | 74.21 us | 130,984 |
| Reverse keyed children | 193.64 us | 66.26 us | 138,100 |
| Rotate one tree child | 80.21 us | 71.30 us | 130,984 |

The no-change and isolated-change results are the clearest evidence for the new architecture.

The transaction implementation originally cloned the complete retained graph. At 512 objects that
caused about 95 KB of allocation for a no-change update. It was replaced with an undo journal:

| Objects | No-change median | No-change allocation | One-change allocation |
| ---: | ---: | ---: | ---: |
| 512 | about 4.3 us | 0 | about 1.1 KB |
| 1,024 | about 8.8 us | 0 | about 1.1 KB |
| 10,000 | about 81.8 us | 0 | about 1.1 KB |

Changed-update time still scales with declaration traversal. Retained-state copying and allocation
no longer scale with graph size.

### Component isolation

| Workload, 512 scopes | Median | Bytes/op | Allocations/op |
| --- | ---: | ---: | ---: |
| Isolated component | 1.16 us | 2,145 | 13 |
| Component with effect | 1.33 us | 2,682 | 18 |
| Component replacement | 1.34 us | 2,150 | 13 |
| Isolated context update | 1.40 us | 2,784 | 19 |
| Broad context update | 581 us | 1,307,836 | 7,173 |

Recursive isolated updates remain nearly constant in the current fixture:

| Scopes | Median | Bytes/op | Mutations/op |
| ---: | ---: | ---: | ---: |
| 585 | 1.1 us | 2,146 | 1 |
| 4,681 | 1.1 us | 2,146 | 1 |
| 21,845 | 1.1 us | 2,146 | 1 |

These results should be reproduced in a clean session before publication.

## Retained memory

| Workload | Reactor | Reactor2 |
| --- | ---: | ---: |
| Keyed grid, 513 objects | 900,168 bytes | 363,844 bytes |
| Keyed grid, bytes/object | 1,754.7 | 709.2 |
| Tree, 513 objects | 109,718 bytes | 472,388 bytes |
| Tree, bytes/object | 213.9 | 920.8 |
| Component idle, 512 scopes | n/a | 557,516 bytes |
| Component idle, bytes/scope | n/a | 1,088.9 |
| Component effect, 512 scopes | n/a | 685,004 bytes |
| Component effect, bytes/scope | n/a | 1,337.9 |

The keyed-grid result favors Reactor2. The tree result favors Reactor and must be investigated
before claiming a general memory improvement. Confirm that both tree fixtures retain equivalent
data, callbacks, native-recording state, and relation structures.

## ItemsRepeater status

Reactor2 now has a retained virtual-source protocol with:

- Lazy declaration/component realization.
- Stable keyed identity across insertion, removal, reorder, and same-key updates.
- Ordered realization and recycle occurrences.
- Container leases and stale-generation rejection.
- Recycle-before-realize cancellation.
- Component task/effect/reference cleanup before native recycle publication.
- One `ReplaceAll` notification per source replacement.
- Recording and real WinUI element-factory fixtures.

Current 10,000-item benchmark with eight realized rows:

| Operation | Median | Bytes/op | Allocations |
| --- | ---: | ---: | ---: |
| Create source | 363.1 us | 441,064 | 18 |
| Replace source | 291.5 us | 439,884 | 13 |
| Realize one row | 5.8 us | 3,364 | 24 |
| Update eight rows | 48.4 us | 48,672 | 220 |
| Recycle one row | 5.5 us | 4,428 | 15 |

Recording-adapter retained memory is 171,492 bytes with nine graph objects: the repeater and eight
realized rows.

The original WinUI adapter created and retained one boxed `IReference<i32>` value per logical item.
Live process measurements confirmed that this was a material memory cost.

### Live ItemsRepeater memory before the lazy source

Commands:

```powershell
cargo run -p test-reactor-bench --release --bin reactor-live-compare --quiet `
  -- --items-memory

1..4 | ForEach-Object {
    cargo run -p test-reactor-bench --release --bin reactor-live-compare --quiet `
      -- --items-memory
}

cargo run -p test-reactor2-bench --release --quiet -- --virtual-memory

cargo fmt -p test-reactor-bench
cargo fmt -p test-reactor2-bench
cargo check -p test-reactor-bench --bin reactor-live-compare --quiet
cargo check -p test-reactor2-bench --quiet
cargo clippy -p test-reactor-bench --bin reactor-live-compare -- -D warnings
cargo clippy -p test-reactor2-bench --all-targets -- -D warnings
cargo test -p windows-reactor2 virtual_source_replacement_raises_one_reset_notification --quiet
git diff --check
```

The table reports medians from five fresh live processes. Process deltas use the mounted empty
ItemsRepeater as the baseline. The median startup working set was 31,993,856 bytes and private size
was 6,934,528 bytes. After mounting the empty repeater, those values were 48,898,048 and 10,403,840
bytes.

| Stage | Working-set delta | Private delta | Rust live delta | Graph objects | Rows |
| --- | ---: | ---: | ---: | ---: | ---: |
| 10,000-item source, no rows | 1,290,240 | 1,228,800 | 720,000 | 1 | 0 |
| 10,000-item source, eight rows | 1,810,432 | 1,228,800 | 729,204 | 9 | 8 |
| Same-size source replacement | 2,129,920 | 1,826,816 | 729,508 | 1 | 0 |
| Source cleared | 1,327,104 | 700,416 | 87,972 | 1 | 0 |
| Window closed and runtime dropped | 2,281,472 | 987,136 | 80,016 | 0 | 0 |

The source-only Rust delta was exactly 720,000 bytes in every run, or 72 bytes per logical item.
The median private-process delta was 1,228,800 bytes before any row was realized. Realizing eight
real WinUI rows added 9,204 live Rust bytes; it did not require another private-memory commit in
these samples. The source-only private delta ranged from 1,196,032 to 1,232,896 bytes across the
five processes. The eager index source therefore dominates the retained row cost.

The recording-only fixture retained 171,492 bytes with the repeater and eight realized rows. This
number excludes WinUI and COM. The live source-only private delta is about 7.2 times that complete
recording fixture, while the tracked Rust source delta is about 4.2 times it.

Measurement limitations:

- The live fixture mounts a real WinUI `ItemsRepeater` in a real window but does not activate the
  window. It calls `GetOrCreateElement` for exactly eight rows so layout cannot realize a
  machine-dependent row count. This includes the native collection, element factory, COM values,
  shells, and row elements, but excludes active-window layout and rendering memory.
- Working set and private bytes include WinUI caches and allocator reservation. Memory remaining
  after source clear or teardown is not evidence of a leak by itself.
- Same-size replacement recycles the eight rows before the replacement snapshot, so that stage
  measures source replacement and allocator/COM churn rather than retained row memory.
- There is no direct COM allocation counter. Attribution to index values is based on the isolated
  zero-row source delta, the exact 72-byte-per-item Rust delta, and the removed
  `virtual_item_values` implementation, which created one boxed `IReference<i32>` per item.

The eager boxed index objects were an architecture-gate blocker for memory-scalable virtualization.

### Lazy native index source

`NativeVirtualSource` now implements `IObservableVector<IInspectable>`,
`IVector<IInspectable>`, `IVectorView<IInspectable>`, and `IIterable<IInspectable>`. It stores an
atomic count and event registrations. `GetAt`, `GetMany`, and iteration create an
`IReference<i32>` only when WinUI requests an item. The source has no item-count-sized vector, map,
or cache.

The vector is read-only to external callers. Unsupported mutation methods return
`E_ILLEGAL_METHOD_CALL`. Source replacement validates the i32 index range, publishes the new
revision and count, and raises one `CollectionChange::Reset` notification. Focused tests cover
bounds, maximum count, index lookup, views, iteration, mutation errors, event add/remove behavior,
and repeated same-size resets.

Post-change commands:

```powershell
cargo run -p test-reactor-bench --release --bin reactor-live-compare --quiet `
  -- --items-memory

1..4 | ForEach-Object {
    cargo run -p test-reactor-bench --release --bin reactor-live-compare --quiet `
      -- --items-memory
}

cargo run -p test-reactor2-bench --release --quiet -- --virtual-items
cargo run -p test-reactor2-selftest --quiet -- --headless
cargo test -p windows-reactor2 --quiet
cargo check -p test-reactor2-selftest --quiet
cargo check -p test-reactor2-bench --quiet
cargo check -p test-reactor-bench --bin reactor-live-compare --quiet
cargo clippy -p windows-reactor2 --all-targets -- -D warnings
cargo clippy -p test-reactor2-selftest --all-targets -- -D warnings
cargo clippy -p test-reactor2-bench --all-targets -- -D warnings
cargo clippy -p test-reactor-bench --bin reactor-live-compare -- -D warnings
cargo fmt -p windows-reactor2
cargo fmt -p test-reactor2-bench
cargo fmt -p test-reactor-bench
git diff --check
```

Post-change medians from five fresh processes:

| Stage | Working-set delta | Private delta | Rust live delta | Graph objects | Rows |
| --- | ---: | ---: | ---: | ---: | ---: |
| 10,000-item source, no rows | 561,152 | 446,464 | 160,000 | 1 | 0 |
| 10,000-item source, eight rows | 1,085,440 | 446,464 | 169,588 | 9 | 8 |
| Same-size source replacement | 1,175,552 | 446,464 | 169,508 | 1 | 0 |
| Source cleared | 860,160 | 106,496 | 7,972 | 1 | 0 |
| Window closed and runtime dropped | 1,761,280 | 286,720 | 16 | 0 | 0 |

The zero-row private delta fell from 1,228,800 to 446,464 bytes, a 63.7% reduction. The Rust delta
fell from 720,000 to 160,000 bytes, removing 560,000 bytes of retained boxed indices. The remaining
exact 16 bytes per logical item are the generic retained key vector used for stable identity and
duplicate-key validation. The native COM source itself retains O(1) state with logical item count.

The post-change recording timing run reported:

| Operation | Median | Bytes/op | Allocations |
| --- | ---: | ---: | ---: |
| Create source | 259.0 us | 441,064 | 18 |
| Replace source | 258.5 us | 439,884 | 13 |
| Realize one row | 4.9 us | 3,364 | 24 |
| Update eight rows | 27.8 us | 48,672 | 220 |
| Recycle one row | 5.8 us | 4,428 | 15 |

These timings use the recording adapter, so the native source implementation cannot improve them
directly. They provide a regression check and show no retained-runtime slowdown.

Residual limitations:

- The live fixture still uses an unactivated window to keep realization fixed at exactly eight
  rows. It verifies real WinUI source, factory, shell, realization, replacement, and recycle paths,
  but not active-window layout memory.
- Process-private and working-set residuals include WinUI and allocator caches. The teardown delta
  is not a leak measurement.
- The zero-row private delta remains 446,464 bytes. The allocator reports 160,000 bytes for retained
  generic keys; the rest may include WinUI's own item-count bookkeeping and heap commit. This
  measurement proves removal of retained boxed index objects, but it does not prove that every
  internal WinUI structure is O(1) with item count.
- Stable generic keys remain O(logical item count). They are required by the current duplicate-key,
  reorder, and same-key update contract and are separate from the native index-source blocker.

Architecture-gate conclusion: the eager native index blocker is resolved. WinUI accepted the custom
collection, existing stale-revision and ordered realization/recycle behavior remains intact, and
native source state no longer scales with logical item count.

### Matched native WinUI startup latency

`reactor-live-compare` now has a matched startup child and parent gate:

```text
cargo run -p test-reactor-bench --release --bin reactor-live-compare --quiet -- \
    --gate-startup --startup-runs 12
```

The release binary was built before timing. The parent then launched 12 fresh processes for each
frontend, alternating which frontend ran first. Both frontends used the same `App::run_with`
initialization, a 320x200 window, one text element, equivalent window policy, and the same
`CompositionTarget::Rendering` subscription.

The child timestamps were:

1. Process entry to entry into the `App::run_with` initialization closure.
2. App initialization to return from the real native `Window.Activate()` call.
3. Activation to the first subsequent `CompositionTarget::Rendering` callback.
4. Process entry to that callback.

The rendering callback is a real WinUI composition rendering signal. It is not `UpdateLayout`, a
sleep, or a posted message. It does not prove hardware scan-out completion. The parent also measured
OS process launch to receipt of the child's flushed JSON report. That value is reported separately
and is not mixed with the child's `Instant` timeline.

Summary:

| Frontend | Entry-app median/p95 | App-activate median/p95 |
| --- | ---: | ---: |
| Reactor | 35.815 / 40.258 ms | 36.477 / 43.016 ms |
| Reactor2 | 35.673 / 36.150 ms | 35.439 / 37.572 ms |

| Frontend | Activate-frame median/p95 | Entry-frame median/p95 |
| --- | ---: | ---: |
| Reactor | 111.640 / 128.221 ms | 183.567 / 197.677 ms |
| Reactor2 | 112.866 / 117.676 ms | 184.645 / 191.000 ms |

| Frontend | CPU median/p95 | Parent launch-report median/p95 |
| --- | ---: | ---: |
| Reactor | 242.188 / 312.500 ms | 195.912 / 211.751 ms |
| Reactor2 | 250.000 / 281.250 ms | 196.127 / 202.247 ms |

| Frontend | Working set median/p95 | Private bytes median/p95 |
| --- | ---: | ---: |
| Reactor | 82,184,192 / 82,817,024 | 71,946,240 / 72,806,400 |
| Reactor2 | 80,943,104 / 81,551,360 | 71,813,120 / 72,900,608 |

Raw child wall-clock results, in milliseconds:

| Run | Reactor entry/app | app/activate | activate/frame | entry/frame |
| ---: | ---: | ---: | ---: | ---: |
| 1 | 35.792 | 33.664 | 128.221 | 197.677 |
| 2 | 36.972 | 37.064 | 109.206 | 183.241 |
| 3 | 34.214 | 37.290 | 108.021 | 179.525 |
| 4 | 35.948 | 34.935 | 123.402 | 194.285 |
| 5 | 35.479 | 35.890 | 111.314 | 182.683 |
| 6 | 36.386 | 37.699 | 108.181 | 182.265 |
| 7 | 40.258 | 43.016 | 107.174 | 190.448 |
| 8 | 35.629 | 34.848 | 113.189 | 183.665 |
| 9 | 35.838 | 37.435 | 107.910 | 181.182 |
| 10 | 35.658 | 41.978 | 113.821 | 191.457 |
| 11 | 37.193 | 35.682 | 111.965 | 184.840 |
| 12 | 35.550 | 34.413 | 113.506 | 183.469 |

| Run | Reactor2 entry/app | app/activate | activate/frame | entry/frame |
| ---: | ---: | ---: | ---: | ---: |
| 1 | 35.040 | 34.388 | 116.657 | 186.085 |
| 2 | 35.752 | 37.572 | 117.676 | 191.000 |
| 3 | 35.700 | 37.506 | 106.467 | 179.673 |
| 4 | 35.565 | 35.416 | 111.628 | 182.609 |
| 5 | 36.094 | 35.214 | 116.046 | 187.354 |
| 6 | 35.545 | 37.020 | 113.743 | 186.308 |
| 7 | 36.139 | 35.463 | 111.977 | 183.579 |
| 8 | 34.982 | 34.992 | 114.883 | 184.857 |
| 9 | 36.011 | 35.590 | 112.832 | 184.433 |
| 10 | 36.150 | 34.821 | 111.695 | 182.666 |
| 11 | 35.646 | 34.885 | 112.373 | 182.904 |
| 12 | 35.631 | 36.540 | 112.900 | 185.071 |

Raw Reactor process results:

| Run | CPU ms | Parent ms | Working set bytes | Private bytes |
| ---: | ---: | ---: | ---: | ---: |
| 1 | 312.500 | 211.751 | 80,764,928 | 70,930,432 |
| 2 | 234.375 | 195.751 | 81,612,800 | 71,405,568 |
| 3 | 265.625 | 191.262 | 82,206,720 | 72,241,152 |
| 4 | 250.000 | 205.773 | 82,710,528 | 72,208,384 |
| 5 | 250.000 | 196.073 | 82,223,104 | 71,327,744 |
| 6 | 234.375 | 194.125 | 82,817,024 | 72,806,400 |
| 7 | 234.375 | 201.712 | 82,731,008 | 72,458,240 |
| 8 | 218.750 | 194.938 | 82,665,472 | 71,737,344 |
| 9 | 187.500 | 192.733 | 81,379,328 | 72,273,920 |
| 10 | 281.250 | 202.937 | 81,879,040 | 72,155,136 |
| 11 | 281.250 | 196.430 | 82,161,664 | 71,311,360 |
| 12 | 234.375 | 195.740 | 80,535,552 | 71,028,736 |

Raw Reactor2 process results:

| Run | CPU ms | Parent ms | Working set bytes | Private bytes |
| ---: | ---: | ---: | ---: | ---: |
| 1 | 265.625 | 200.768 | 80,699,392 | 71,716,864 |
| 2 | 250.000 | 202.247 | 80,900,096 | 71,839,744 |
| 3 | 218.750 | 192.069 | 80,408,576 | 72,757,248 |
| 4 | 203.125 | 195.023 | 80,334,848 | 72,749,056 |
| 5 | 234.375 | 201.766 | 81,182,720 | 72,785,920 |
| 6 | 281.250 | 197.694 | 80,986,112 | 71,438,336 |
| 7 | 250.000 | 195.375 | 80,740,352 | 71,786,496 |
| 8 | 234.375 | 196.448 | 81,149,952 | 72,511,488 |
| 9 | 281.250 | 195.805 | 81,334,272 | 71,659,520 |
| 10 | 250.000 | 194.231 | 81,084,416 | 72,900,608 |
| 11 | 250.000 | 193.972 | 81,551,360 | 71,458,816 |
| 12 | 218.750 | 199.711 | 80,363,520 | 70,676,480 |

The median Reactor2 entry-to-first-render time is 1.077 ms, or 0.59%, above Reactor. Reactor2's p95
is 6.677 ms lower. Parent launch-to-report medians differ by 0.215 ms. These distributions do not
identify a startup architecture blocker.

Limitations:

- `CompositionTarget::Rendering` marks the first composition rendering callback after activation,
  not completed monitor scan-out.
- Windows process CPU accounting is quantized in 15.625 ms steps on this machine, so CPU results are
  useful only as coarse corroboration.
- Working set and private bytes are snapshots in the first rendering callback and include shared
  WinUI/app initialization, allocator state, and benchmark instrumentation.
- The parent measurement ends when it receives the report, before child teardown completes.

### Sampled CPU profiles

Release binaries with PDBs were built with:

```text
cargo build --profile profiling -p test-reactor2-bench -p test-reactor-bench --bins --quiet
```

WPR and Samply could not enable sampled profiling without elevation:

- `wpr -start CPU -filemode` failed with `0xc5585011`.
- `samply record` reported that Administrator privileges are required on Windows.

The non-elevated Visual Studio DiagnosticsHub CPU agent worked. It attached to each already-running
profiling binary using `CpuUsageBase.json`, collected sampled stacks at the default 1 kHz rate, and
produced a `.diagsession` plus ETL. `xperf -symbols -a profile -detail` and `xperf -symbols -a stack
-butterfly` resolved the Rust PDB symbols.

All commands, original `.diagsession` archives, expanded ETLs, symbolized reports, extracted
top-symbol TSV files, workload output, binaries, and PDBs are stored outside the repository:

```text
C:\Users\kekerr\.copilot\session-state\b09bb5bf-29b4-46df-a2f4-228b0686bb7a\
    files\cpu-profiles
```

The artifact `README.md` records every workload command and the capture/export command shape.
`summary.json` contains the extracted top inclusive and exclusive Rust symbols.

#### 100,000-object retained updates

| Frontend/workload | Median | p95 | Bytes/op | Allocations/op |
| --- | ---: | ---: | ---: | ---: |
| Reactor no change | 15.194 ms | 27.852 ms | 8 | 0 |
| Reactor2 no change | 1.314 ms | 1.834 ms | 8 | 0 |
| Reactor one changed | 27.160 ms | 33.430 ms | 8,553,662 | 78 |
| Reactor2 one changed | 3.457 ms | 4.370 ms | 1,148 | 8 |

Reactor no-change attribution:

- `Pump::update_view` was 94.5% inclusive.
- `keyed_views_match` was 94.5% inclusive and 12.7% exclusive.
- Native-property comparison was 50.3% inclusive. Its generated visitor closure was 38.6%
  exclusive, generated property equality was 11.3% exclusive, and `Tree::children` was 6.0%.
- Recording-adapter application was 3.8% exclusive.

Reactor2 no-change attribution:

- `Runtime::update` was 98.5% inclusive.
- `RetainedGraph::matches_declaration_inner` was 91.2% inclusive and 61.0% exclusive.
- Generated declaration equality was 26.6% inclusive and 7.8% exclusive.
- The recording adapter's active-index lookup was 6.7% exclusive.
- No allocator or lock hotspot appeared. This matches the zero-allocation timing result.

Reactor one-change attribution:

- `Pump::reconcile_keyed_child_list` was 69.1% inclusive.
- Native-property comparison remained 26.9% inclusive; its generated visitor closure was 20.7%
  exclusive.
- Keyed-list reconciliation, native-root traversal, generated equality, tree-child lookup, and
  recording application were each 3.3-7.2% exclusive.
- The 8.55 MB/op allocation result corresponds to the changed keyed planning path, although sampled
  CPU was spread across reconciliation rather than one allocator entry point.

Reactor2 one-change attribution:

- `Runtime::update` was 98.5% inclusive.
- IR validation was 37.3% inclusive and 10.4% exclusive.
- `Planner::reconcile_keyed_many` was 33.0% inclusive and 12.6% exclusive.
- Hash-map insertion was 19.4% inclusive and 11.0% exclusive.
- `matches_declaration_inner` was 13.8% exclusive and `reconcile_object` was 9.7%.
- The sampled work agrees with the small eight-allocation mutation path. There was no lock or ABI
  crossing hotspot.

The no-change profiles explain the measured gap: Reactor repeatedly traverses generated property
visitors and planner matches, while Reactor2 performs a narrower declaration match. The one-change
profiles show Reactor2's remaining CPU in validation and temporary keyed-map construction, but this
is not an architecture blocker at the measured 3.46 ms for 100,000 objects.

#### Broad context propagation

The matched Reactor equivalent was not included because Reactor's existing context benchmark uses
its different provider/component model. Treating it as a direct frontend comparison would be
misleading.

The initial 16,384-consumer result was:

| Median | p95 | Bytes/op | Allocations/op |
| ---: | ---: | ---: | ---: |
| 615.296 ms | 631.689 ms | 41,844,924 | 229,381 |

`ComponentHost::set_context` was 99.2% inclusive and `apply_render` was 97.9%. The dominant symbol
was the `RetainedGraph::references` collection iterator inside `update_subtree_before_apply`: 92.3%
inclusive and 92.2% exclusive. It scanned the whole retained graph before every consumer subtree
update.

This blocker is fixed. `update_subtree_inner` now derives its poison-time detached-reference set
from the planner's `ReferenceChange::Clear` records. Those records cover the updated object and
every owned descendant actually detached by the transaction, including realized virtual rows.
References that remain in the committed graph are cleared by the existing single
`clear_references` pass when the runtime is poisoned. Thus:

- A returned planning error still rolls the transaction back without touching external refs.
- A returned validation/apply error commits the divergent transaction, clears every pre-update
  binding detached by that transaction, clears every binding still retained by the graph, and then
  discards the graph.
- Unrelated objects are not scanned before each subtree update.
- Generation-checked `ElementRef::clear` prevents an old detached binding from clearing a newer
  reused-slot binding.

The other full-reference snapshots were reviewed. Root-wide update, explicit full host
invalidation, virtual realization/recycle, cancel, child removal, and retirement completion retain
their existing behavior. They are not part of the per-consumer context loop, and changing their
panic/failure boundaries was outside this fix.

After the fix, a fresh 400-operation sampled profile reported:

| Median | p95 | Bytes/op | Allocations/op |
| ---: | ---: | ---: | ---: |
| 22.425 ms | 28.760 ms | 41,844,924 | 229,381 |

The after-profile archive, ETL, reports, binary, and PDB are in
`files\cpu-profiles\reactor2-context-broad-after` under the session folder.

Median time fell by 96.4%, or 27.4x. `RetainedGraph::references` no longer appears among sampled hot
symbols. The new inclusive leaders are `ComponentHost::apply_render` at 77.8%,
`Runtime::update_subtree_before_apply` at 53.0%, and planner reconciliation at 17.3%. No single
exclusive symbol exceeds 6.9%.

Repeated scaling runs gave:

| Consumers | Median range | Bytes/op | Allocations/op |
| ---: | ---: | ---: | ---: |
| 8,192 | 10.110-10.679 ms | 20,922,556 | 114,693 |
| 16,384 | 21.538-22.775 ms | 41,844,924 | 229,381 |

The stable median scaling ratio is about 2.1x for 2x consumers. Bytes and allocations scale exactly
linearly with consumer count.

The allocation counters did not fall. The original profile established that the reference scan
dominated CPU, but the post-fix result proves it was not the source of the 41.8 MB/229k allocations:
the benchmark components do not retain element references, so collecting their empty reference
sets scanned the graph without allocating reference entries. The remaining roughly 14 allocations
and 2.55 KB per consumer are spread across component view construction, graph transactions,
declaration cloning, dependency replacement, recording-adapter application, and hash/vector
growth. Reducing those is a separate component rendering/transaction optimization and was not
folded into this reference-correctness fix.

Adversarial coverage now proves:

- Nested refs removed or replaced by a subtree update clear after both validation and apply failure.
- A realized virtual-row ref detached with its owning subtree clears.
- A ref outside the failing subtree clears during host/runtime invalidation.
- A removed slot reused at a new generation does not let the stale binding clear the new binding.
- An apply failure after one successful context consumer clears every consumer ref and poisons the
  host.
- A 16,384-consumer context update performs no additional full-graph reference scans.

#### 10,000-item virtual source replacement

The retained recording workload kept eight rows realized and replaced the source 350,000 times:

| Time/op | Mutations/op | Bytes/op | Allocations/op |
| ---: | ---: | ---: | ---: |
| 22.063 us | 1 | 48,352 | 219 |

Attribution:

- Native-event dispatch was 60.3% inclusive and virtual realization was 53.2%.
- `Planner::reconcile_object` was 44.9% inclusive and 29.7% exclusive.
- Recording-adapter application was 14.8% inclusive and 3.1% exclusive.
- Hashing, vector growth/collection, snapshot cloning, small sorting, and boxed declaration cloning
  each contributed roughly 0.6-3.3% exclusive.
- `ntdll` accounted for 24.5% of module samples, consistent with the 219 allocations/op.

The source itself remains O(1), and one source mutation is published per replacement. The profile
shows allocation-heavy realized-row reconciliation, not an eager-index or ABI-crossing regression.
At 22 us per replacement it is not an architecture gate blocker, but the retained allocation count
is a future optimization target.

#### Native 1,024-child reverse

Both live traces ran 600 render-paced reversals:

| Frontend | Update avg/p95 | Native apply avg/p95 | Allocations/update |
| --- | ---: | ---: | ---: |
| Reactor | 8.680 / 9.550 ms | 8.095 / 8.962 ms | 2,175 |
| Reactor2 | 8.551 / 9.808 ms | 8.335 / 9.566 ms | 84 |

The process-wide module shares were close:

| Module/category | Reactor | Reactor2 |
| --- | ---: | ---: |
| DirectWrite | 18.3% | 18.2% |
| Microsoft.UI.Xaml | 14.5% | 15.3% |
| DWM composition | 13.9% | 13.5% |
| `ntdll` | 15.3% | 15.8% |
| Kernel | 8.7% | 9.2% |
| COM/DCOM/marshal | 11.2% | 10.2% |
| D2D/D3D/text shaping | 10.2% | 10.6% |
| Benchmark executable | 1.4% | 0.9% |

Reactor's `WinUiRuntime::synchronize_children` was 14.9% inclusive. Reactor2 native apply was 15.5%
inclusive; `GeneratedCollection::remove_at` was 10.1%, `insert_at` 4.4%, and the Rust
`simulate_active_slot_reorder` helper only 0.29%. This confirms that dense reverse cost is the
required native collection mutation and downstream WinUI/COM/layout work, not the Rust reorder
simulation or repeated collection scanning. The sampled result matches the prior steady-state
native apply ratio and scaling gate.

No sampled workload showed lock contention. The headless retained workloads had no meaningful COM
or ABI crossing. The live dense reverse had substantial COM/WinUI/DWrite/DWM cost, but Reactor and
Reactor2 distributions were matched and Reactor2 did not add an excessive crossing hotspot.

Architecture-gate conclusion: dense reorder, 100k retained updates, virtual replacement, and broad
context propagation show no current CPU-scaling blocker. The per-consumer whole-graph
`ElementRef` scan is removed. Broad context allocation volume remains a separate optimization
candidate, but it scales linearly and no longer drives sampled CPU.

## Architecture implemented

### Schema and generation

- Ordinary Reactor2 generation reads only `crates/tools/reactor2/src/schema.toml`.
- `--convert-old-schema` is a one-shot migration command.
- `--parity-report` compares all old schema contracts with Reactor2.
- `--check-parity` fails while anything remains unresolved.
- Generated files are deterministic across consecutive runs.
- Metadata resolves declaring dependency-property owners rather than guessing from the concrete
  control class.
- Unknown imported defaults clear the declaring dependency property.
- Explicit schema defaults write the old value on removal.
- Shared declaration/property machinery avoids repeated per-control contract arrays.
- Unused generated getters, event variants, and binding vtables were removed.
- No `#[allow(dead_code)]` remains in the audited Reactor2, generator, or selftest code.

### Retained graph and transactions

- Generational `ObjectId` values reject stale work.
- Updates use an undo journal, not full graph cloning.
- Graph, references, and mutation batches are transactional.
- Free-list ordering and slot generations roll back exactly.
- Native observations survive a later declaration-planning failure.
- Invalid native occurrences poison the runtime instead of partially advancing state.
- Returned graph errors roll back through the graph journal.
- FFI panics abort naturally; internal Rust failures panic normally.

### Ordered native events

Observations, callbacks, retirement completion, realization, and recycling use one ordered native
occurrence stream.

For each occurrence:

1. Its observation is validated and applied.
2. A stale callback revision suppresses only the callback, not the observation.
3. The callback runs.
4. Callback-triggered component reconciliation completes.
5. The next native occurrence is processed.

This prevents callback A from observing native state from later event B.

### Controlled feedback

- Synchronous exact feedback.
- Synchronous normalized feedback.
- Deferred exact RichEdit feedback.
- Clear-value feedback.
- Multiple matching deferred RichEdit events remain suppressed.
- A differing native RichEdit value ends the expectation and is delivered.
- Application-driven values do not invoke user callbacks.

### Selection

NavigationView, ListBox, and SelectorBar use retained selected-object identity:

- Identity survives reorder.
- Removal clears selection.
- Same-key generations cannot cross-update.
- Application writes suppress native echoes.
- Native changes update retained state and invoke once.
- NavigationView extracts `SelectedItem` from its event arguments.

### Layout and exit retirement

The shared layout family includes:

- Width, height, minimum and maximum dimensions.
- Margin and alignments.
- Opacity and theme transitions.
- Grid row, column, and spans with validation.
- RelativePanel flags.
- Canvas positions.
- Automation name, ID, and heading level.
- Exit fade retirement.

Removed keyed subtrees leave active lookup immediately but retain native ownership until
asynchronous completion. Events, references, components, effects, tasks, and active contexts
retire at logical removal. Completion is idempotent and generation-safe.

### Other completed families

- All 42 old slot relations.
- Typed native collection relations.
- All three old selection contracts.
- Shared `IsEnabled`.
- Element references and generic focus commands.
- Typed Grid, Image, WebView2, and SwapChainPanel imperative references with rebinding-aware
  observations and stale-completion rejection.
- Text styling and validated `FontWeight`.
- RichEditBox controlled `ITextDocument` text with LF normalization.
- Typed Grid row and column definitions.
- Optional number/rating values with distinct native sentinels.
- Checked selection-index conversion.
- Typed event-argument payload extraction for represented event families.
- Real pointer-event extraction without process abort on unavailable capture state.

## Reliability and correctness work completed

The following bugs were found by review and fixed:

- Pointer callback process abort in Solitaire.
- Missing pointer capture collection treated as fatal.
- Controlled feedback counted without runtime suppression.
- RatingControl and NumberBox sharing the wrong empty sentinel.
- Selection index conversion panic.
- Generated event getter failures aborting the process.
- Event payload semantics overcounted by parity.
- NavigationView selection rereading owner state instead of event arguments.
- Ref-only updates skipped by declaration matching.
- Order-dependent `ElementRef` transfer.
- Public property-contract enumeration removed by output compression.
- Missing Grid row/span validation.
- Partial graph mutation on rejected retirement updates.
- Duplicate retirement completion poisoning.
- O(total graph) retained graph cloning.
- Native observations lost during plan rollback.
- Separate observation/callback queues reordering native events.
- Returned-error handling outside graph and adapter transactions.
- Stale callback revisions dropping valid observations.
- Retirement completion overtaking earlier native events.
- Callback panic permanently stalling the event pump.
- RichEdit clear feedback and repeated deferred-match feedback.
- ItemsRepeater destruction using generic ListView removal.
- Updates consuming pending realization work.
- Recycle-before-realize poisoning.
- Virtual component cleanup after native destruction.
- One native collection notification per source item.

## Current validation

Latest local validation:

- Corrected native lifecycle gate: five alternating empty/loaded 40-cycle pairs passed.
- Dense reverse gate: Reactor2/Reactor was 1.059x at 512 and 1.033x at 1,024 children; scaling was
  2.136x.
- 451 `windows-reactor` and 143 `windows-reactor2` unit/integration tests passed after recovery.
- Affected selftest and benchmark all-target checks passed.
- Strict all-target Clippy passed for both Reactor crates and both benchmark crates.
- Broad-context reference fix: 142 unit tests, 1 integration test, and 72 doctests passed.
- `test-reactor2-selftest`: check passed.
- `windows-reactor2` and `test-reactor2-bench`: strict all-target Clippy passed.
- The 16,384-consumer scan-counter test observed no additional full-graph reference scans.
- `windows-reactor --features test`: 451 unit tests, 1 integration test, and 9 doctests passed.
- `test-reactor-bench --all-targets`: check and strict Clippy passed.
- `windows-reactor --features test --all-targets`: strict Clippy passed.
- Startup gate: 12 alternating fresh processes per frontend completed.
- `tool-reactor2`: 8 tests passed.
- `windows-reactor2`: 126 unit tests passed.
- Additional integration test: passed.
- Doctests: 69 passed.
- `test-reactor2-selftest`: compiles.
- Strict clippy with `-D warnings`: passed in the affected Reactor2 targets.
- Consecutive generator hashes: stable.
- `git diff --check`: passed.
- No product `abort()` remains in the audited Reactor2 paths.
- No `#[allow(dead_code)]` remains in the audited Reactor2 paths.
- TitleBar and compatibility checkpoint: 153 `windows-reactor2` library tests passed.
- All 73 doctests and all 8 `tool-reactor2` tests passed.
- Affected Reactor2 libraries, samples, and selftest passed all-target checks and strict Clippy.
- Consecutive generation remained stable after the TitleBar and TextBlock surface changes.

The live selftest passes the newer selection, feedback, layout, retirement, RichEdit, Grid, and
ItemsRepeater fixtures. It also passes opening a window before declaring a TitleBar, changing its
preferred height from tall to standard, and removing it. It later fails at the real
`PointerReleased` injection fixture because the test window is not foreground in the current
desktop session. This is an environment/test-host blocker, not a reason to skip or weaken the test.

## Remaining exact parity

### Capabilities: 0

All 158 capability contracts are represented. The four typed reference capabilities share one
bounded runtime queue, binding generations, observation revocation, asynchronous completion
filtering, and runtime-drop cleanup model.

TitleBar is owned by its declaration and retained graph. The planner maintains a transactional
index, rejects duplicate declarations, defers native attachment until structural mutations
complete, and clears the native attachment before destruction. WinUI synchronizes open windows
after each mutation batch, so mounting, height changes, removal, and opening a window after initial
publication use the same path. One retained root may be content of only one live native window.

Reference-family validation:

- 149 `windows-reactor2` library tests passed, including destroyed-object revocation and imperative
  budget rearming.
- 73 doctests passed, including typed reference mismatch compile failure.
- `tool-reactor2` generated stable output and all 8 generator tests passed.
- Strict all-target Clippy passed for Reactor2, its generator, selftest, and both benchmark crates.
- The live selftest passed the new Grid, Image, WebView2, and SwapChainPanel fixture. The process
  later stopped at the existing foreground-dependent pointer injection test because the test window
  was not foreground.

### Solitaire compatibility checkpoint

The original Reactor and Reactor2 Solitaire samples were compared before continuing with more
surface generation. Avoidable differences were removed:

- TitleBar attachment no longer requires component-reference lookup or a `WindowPolicy` object ID.
- The TitleBar is declared inline and owns its preferred height, matching the original view.
- Viewbox exposes the canonical `child` relation rather than duplicate `content` and `child`
  aliases for the same native slot.
- `TextBlock` again uses `TextBlock::new().text(value)` because empty text is valid.
- Strings convert directly to `Visual`, so content controls accept `.content("New Game")`.

The remaining sample differences are intentional architecture boundaries:

- Component callbacks use queued senders so native events cannot mutate component state inline.
- Heterogeneous child arrays require explicit `Visual` conversions; retaining iterator support and
  adding tuple overloads under one Rust method would create competing collection APIs.
- The native host owns window creation and `WindowPolicy`; component declarations remain usable in
  recording and non-window adapters.
- `Solitaire -> Board -> keyed CardView` preserves targeted subtree ownership and card identity
  across pile moves, so it should not be flattened to match the original component structure.

### Lifecycle and placement: 2

- ContentDialog lifecycle
- ToolTip attachment

These require host-owned placement/lifecycle state. They should not be represented as ordinary child
relations or property aliases.

### Properties: 42

The parity report currently groups these as:

- 35 missing properties.
- 3 theme-style semantics.
- 4 controlled/clear-feedback semantics.

Main property families:

- Resource dictionaries and styles.
- Keyboard accelerators.
- URI and image sources.
- Brushes, geometry, and rich-text blocks.
- Scale and transition values.
- Pointer/focus/drop behavior.
- ListView and TreeView selection/drag contracts.
- Theme-style brush variants.
- Remaining controlled CheckBox, ToggleButton, Expander, and NavigationView behavior.

### Events: 31

Main event families:

- Drag/drop.
- Pointer pressed/moved/entered/exited.
- Routed key, character, and focus events.
- Breadcrumb and AutoSuggest payloads.
- NavigationView display/pane state.
- Controlled state-change events.
- Tab, color, date, and time payloads.
- ContentDialog close.
- List/Grid/Tree item and drag events.
- Border `PointerReleased` routed semantics.

## Known risks

### Native ItemsRepeater memory

Resolved. The count-backed native source boxes indices on demand. Five live 10,000-item runs reduced
the zero-row private delta from 1,228,800 to 446,464 bytes and removed 560,000 bytes of retained
boxed-index storage. The remaining 160,000-byte Rust delta is the generic stable-key vector.

### Tree memory

Reactor2 tree retained memory is higher in the current fixture. Profile relation allocation, event
storage, declaration retention, and recording-adapter state before making a memory claim.

### Generated binding size

Generated bindings remain the largest generated surface. Confirm that each requested type/member is
reachable from schema/native code. Continue getter/member pruning based on actual observed-event and
adapter needs.

### Linear declaration traversal

Isolated changed updates allocate constant memory but still traverse the declaration tree. Current
times are favorable, but CPU profiling at 10,000 and 100,000 objects should confirm where time is
spent before introducing indexing or dirty-path caches.

### Live test host

The pointer fixture needs an unrestricted foreground desktop or a CI host that can reliably focus
the test window and inject input. Do not replace it with `PostMessage` or a simulated product hook.

### Dirty worktree

All changes are uncommitted. Generated, schema, runtime, tests, and docs form one coherent state.
Do not revert individual generated files or only part of ItemsRepeater.

## Architecture review plan

Do this before adding parity surface.

### 1. Reproduce the milestone

```text
git status --short
cargo run -p tool-reactor2 --quiet
cargo run -p tool-reactor2 --quiet -- --parity-report
cargo test -p tool-reactor2 -p windows-reactor2 --quiet
cargo check -p test-reactor2-selftest -p test-reactor2-bench --quiet
cargo clippy -p tool-reactor2 -p windows-reactor2 -p test-reactor2-selftest \
    -p test-reactor2-bench --all-targets -- -D warnings
git diff --check
```

Run the generator twice and compare hashes because `git diff` cannot prove stability in a dirty
worktree.

### 2. Review handwritten architecture

Read these first:

- `crates/libs/reactor2/src/reconcile.rs`
- `crates/libs/reactor2/src/component.rs`
- `crates/libs/reactor2/src/adapter.rs`
- `crates/libs/reactor2/src/native/winui.rs`
- `crates/tools/reactor2/src/main.rs`
- `crates/tools/reactor2/src/parity.rs`
- `crates/tools/reactor2/src/schema.toml`

Review generated files through generator invariants, representative samples, compilation, and
measurement rather than line-by-line inspection.

Specific review questions:

- Does every retained mutation have journal coverage?
- Can any native occurrence be consumed without being applied or explicitly rejected?
- Can stale generations, callback revisions, leases, or retirement completions cross into a
  replacement object?
- Do callbacks reconcile before later native state becomes visible?
- Do component effects, tasks, and references retire before native publication?
- Does every public API correspond to a schema contract or intentional runtime service?
- Can shared generated lookup tables replace more repeated match arms without increasing runtime
  or binary cost?
- Are any internal items `pub` when private or `pub(crate)` is sufficient?
- Is any dead code hidden by feature combinations or generated binding reachability?

### 3. Performance and CPU

Run the existing benchmark several times and save raw output:

```text
cargo run -p test-reactor2-bench --release --quiet
```

Add or run:

- Alternating fresh `cargo check` order over at least five runs.
- Release binary and `.rmeta`/`.rlib` size comparison.
- CPU sampling for 10,000-object no-change, isolated change, reorder, broad context, and virtual
  source replacement.
- Native mutation count and duration for the same workloads.
- 100,000-object declaration traversal to find the current CPU limit.
- Startup from process creation through first activated window and first presented frame.

Do not optimize generated source size without measuring binary size and runtime cost.

### 4. Memory

Measure:

- Rust allocations already covered by `test-reactor2-bench`.
- Process-private bytes and working set after first window.
- Native WinUI object count for 10,000 virtual items with eight realized rows.
- Tree fixture allocation breakdown.
- Component idle/effect memory with recording and WinUI adapters separated.
- Memory after repeated mount/unmount, retirement, recycle, and window close.

### 5. Reliability

Run fault-injection tests for:

- Adapter validation failure.
- Adapter apply failure.
- Ordinary internal panic behavior and natural FFI abort behavior.
- Native getter/setter failure.
- Window close with queued native events, timers, retirements, and virtual leases.
- Rapid same-key removal/reinsertion.
- Recycle before realization and shell reuse.
- Duplicate and stale completions.
- Reentrant native handlers.

Run the complete live selftest on an unrestricted foreground desktop.

### 6. Decision gate

Passed. Reactor2 meets the gate:

- No architectural correctness blocker remains.
- Retained update performance remains materially better.
- Compile, binary, startup, and native-memory results are acceptable.
- The handwritten runtime remains substantially smaller and easier to trace.
- Generated code remains deterministic and output growth follows represented API surface.
- Live WinUI fixtures pass without product test hooks.

If these conditions fail, fix the architecture before adding more controls or adapters.

## Completeness plan after the decision gate

Recommended order:

1. [x] Typed imperative reference service shared by Grid, Image, WebView2, and SwapChainPanel.
2. [x] Window-owned TitleBar attachment.
3. [ ] ToolTip attachment lifecycle.
4. [ ] ContentDialog lifecycle.
5. [ ] Shared resource/style/brush/image/geometry value contracts.
6. [ ] Controlled CheckBox, ToggleButton, Expander, and NavigationView events.
7. [ ] Typed drag/drop, routed input, focus, tab, color, date, and time events.
8. [ ] Remaining rich-text and collection contracts.
9. [ ] Strict `--check-parity`.
10. [ ] Full apples-to-apples benchmarks and live acceptance suite.

Each family must have:

- Public compile-surface coverage.
- Retained/recording mutation tests.
- Real WinUI fixture coverage.
- Failure and stale-work coverage.
- Native mutation-count assertions where relevant.
- Parity credit only after exact old semantics are represented.

## Worktree files

Current modified files:

```text
crates/libs/reactor2/readme.md
crates/libs/reactor2/src/adapter.rs
crates/libs/reactor2/src/component.rs
crates/libs/reactor2/src/declaration.rs
crates/libs/reactor2/src/generated.rs
crates/libs/reactor2/src/generated_declarations.rs
crates/libs/reactor2/src/ir.rs
crates/libs/reactor2/src/native/bindings.rs
crates/libs/reactor2/src/native/generated.rs
crates/libs/reactor2/src/native/winui.rs
crates/libs/reactor2/src/reconcile.rs
crates/libs/reactor2/src/tests.rs
crates/tests/libs/reactor2_bench/src/main.rs
crates/tests/libs/reactor2_selftest/src/main.rs
crates/tests/libs/reactor_bench/src/live_compare.rs
crates/tests/libs/reactor_bench/src/reactor2_live_notepad.rs
crates/tools/reactor2/src/bindings.txt
crates/tools/reactor2/src/main.rs
crates/tools/reactor2/src/parity.rs
crates/tools/reactor2/src/schema.toml
docs/crates/windows-reactor2.md
```

`plan.md` is the durable status and restart document for this milestone.

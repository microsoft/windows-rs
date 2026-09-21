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

The current milestone is sufficient for an architecture decision. It is not yet complete API or
behavioral parity. The next session should review and measure the architecture before adding more
surface area.

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

The decision is not final:

- Property and event parity remains incomplete.
- Five capability contracts and two lifecycle/placement contracts remain.
- Native ItemsRepeater still creates one boxed WinRT index value per logical item.
- Tree and component retained-memory results are mixed and need explanation.
- Startup, binary size, CPU profiles, and native WinUI memory have not been measured carefully.
- The live selftest passes the new fixtures but cannot complete the real pointer fixture in the
  current desktop session because the test window is not foreground.

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
| Capabilities | 153 | 158 | 5 |
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

Important limitation: the WinUI adapter still creates and retains one boxed `IReference<i32>` value
for each logical item. The recording benchmark does not observe those COM allocations. Do not claim
that the native path retains only keys and realized rows until this is replaced with a lazy/indexed
WinRT collection and measured in the live process.

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
- Graph transactions roll back during unwinding.
- Adapter validation/application unwind paths poison the runtime.

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
- Non-unwind-safe graph and adapter transactions.
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

The live selftest passes the newer selection, feedback, layout, retirement, RichEdit, Grid, and
ItemsRepeater fixtures. It later fails at the real `PointerReleased` injection fixture because the
test window is not foreground in the current desktop session. This is an environment/test-host
blocker, not a reason to skip or weaken the test.

## Remaining exact parity

### Capabilities: 5

- `reference`: Grid
- `reference`: Image
- `reference`: WebView2
- `reference`: SwapChainPanel
- `window_title_bar`: TitleBar

The four references need typed asynchronous imperative endpoints with rebinding, observations, and
stale-completion rejection. TitleBar needs explicit window ownership and clear-before-replace
attachment lifecycle.

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

The native source eagerly creates one COM index value per logical item. Replace it with a lazy
index-backed WinRT vector or equivalent source, then measure process-private bytes and COM object
count in a live 10,000-item fixture.

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
- Callback panic caught by the host.
- Native getter/setter failure.
- Window close with queued native events, timers, retirements, and virtual leases.
- Rapid same-key removal/reinsertion.
- Recycle before realization and shell reuse.
- Duplicate and stale completions.
- Reentrant native handlers.

Run the complete live selftest on an unrestricted foreground desktop.

### 6. Decision gate

Proceed to full parity only if:

- No architectural correctness blocker remains.
- Retained update performance remains materially better.
- Compile, binary, startup, and native-memory results are acceptable.
- The handwritten runtime remains substantially smaller and easier to trace.
- Generated code remains deterministic and output growth follows represented API surface.
- Live WinUI fixtures pass without product test hooks.

If these conditions fail, fix the architecture before adding more controls or adapters.

## Completeness plan after the decision gate

Recommended order:

1. Lazy native ItemsRepeater index source and native-memory measurement.
2. Typed imperative reference service shared by Grid, Image, WebView2, and SwapChainPanel.
3. Window-owned TitleBar attachment.
4. ToolTip attachment lifecycle.
5. ContentDialog lifecycle.
6. Shared resource/style/brush/image/geometry value contracts.
7. Controlled CheckBox, ToggleButton, Expander, and NavigationView events.
8. Typed drag/drop, routed input, focus, tab, color, date, and time events.
9. Remaining rich-text and collection contracts.
10. Strict `--check-parity`.
11. Full apples-to-apples benchmarks and live acceptance suite.

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

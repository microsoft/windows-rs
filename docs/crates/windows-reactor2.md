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

`tool-reactor2` reads `crates/tools/reactor2/src/schema.toml` and generates object, property,
event, and relation identifiers plus their contracts. The schema records:

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
`test-reactor2-selftest` executable opens a real Reactor2 window, repeatedly reorders realized
TreeView nodes with custom visual content, and exercises controlled TextBox input.

`reactor2-solitaire` is the first application-shaped sample. Its game state, component messages,
declarations, reconciliation, TextBox events, and game window use Reactor2. It uses the current
Reactor application host only to initialize and keep the WinUI dispatcher alive because Reactor2
does not yet provide a standalone application bootstrap. The sample uses a text board until the
pointer, layout, and styling slices needed by the visual Solitaire sample are projected.

`tool-reactor2` now generates typed declaration builders from `schema.toml`. The generated surface
covers all current prototype objects, and the public `Button` slice uses the generated builder with
an owned visual `Content` relation and unit-valued `Click` event. Button creation, native content,
and event delivery reuse the existing retained relation and queued-event protocols without changes
to the planner or component lifecycle.

The same schema now generates native realization for ordinary controls. The current set is
TextBlock, Button, CheckBox, Border, Grid, StackPanel, Canvas, ScrollViewer, and Slider. It covers
string, `f64`, nullable boxed `bool`, and metadata-derived enum properties; keyed and positional
panel children; content ownership; and unit events. The generated `GeneratedHandle` owns native
construction, object-kind and UIElement conversion, direct properties, panel children, content
attachment, event subscription, and callback lookup. `tool-reactor2` verifies setter ABI shapes and
resolves property, content, event, and enum information through the metadata resolver shared with
`tool-reactor`; it also derives the binding filter needed by those generated paths. Generated
mutable controls report native feedback into the retained graph even when no callback is installed.
TextBox feedback, TreeView structural nodes, ListView container data, and templates remain focused
handwritten adapter cases.

## Public API shape

The declaration API exposes one typed path for each generated contract:

| Contract | Public shape |
| --- | --- |
| Authoritative scalar value | Required constructor input |
| Optional scalar or event | Builder method; omission removes it |
| Optional owned child | Builder method; omission represents no child |
| Positional visual relation | Iterator of `Visual` |
| Keyed visual relation | Iterator of `KeyedVisual` created by `keyed` |
| Structural or data relation | Iterator of its specific keyed declaration type |

This keeps invalid category and identity combinations from compiling. Compile-fail doctests cover
visual, keyed visual, structural, and data relation boundaries. Controlled `TextBox` text remains a
required constructor input because an unset value would create a second authority model. There is
no separate common and advanced control path.

`Property`, `Event`, `Observation`, `EventDispatch`, and their payload enums form the public
adapter protocol rather than the application declaration API. External adapters need to consume
generic mutations and report native observations and events. The runtime validates those messages
against the generated object contracts before changing retained state or exposing a callback.
Events for stale objects or replaced callbacks are discarded.

## Component lifecycle slice

Component scopes are lifecycle records, not another UI representation. A scope owns application
state, a bounded message destination, effect slots, and the `ObjectId` of its retained subtree
root. Its view still produces an ordinary typed `Visual`. `Runtime::update_subtree` reconciles that
declaration directly against the existing retained object, so an isolated component update does
not rebuild or walk the parent declaration. The scope does not retain its last `Visual`; removal
uses the same generic `Remove` and `Destroy` mutations as declarative reconciliation.

An owned component root can change visual type without adding a permanent wrapper object.
`Mutation::Replace` preserves the retained `ObjectId` while the backend replaces the native child
at its current parent position. The runtime rejects replacement through structural or data
relations, and adapters discard observations and events queued by the old native object. Changed
effect cleanup runs after mutation validation but before native replacement; new setup runs after
the replacement is applied.

`ComponentHost` type-erases component state while preserving typed inputs, messages, callbacks, and
factories. Different component types coexist as keyed roots under one retained `Grid`:

- messages are queued and never run inline with native callbacks;
- equal parent inputs do no work, while changed inputs reconcile only the owning subtree;
- a state change reconciles only the owning retained subtree;
- the retained root identity and `ElementRef` remain stable across updates;
- effects compare typed dependencies, clean up before replacement, and clean up on retirement;
- typed contexts maintain a reverse dependency index and rerender only subscribed scopes;
- background work and timers are scope-owned, bounded, cancellable, and wake the host once per
  pending batch;
- completion handles can cross threads, while generation checks discard delivery after retirement;
- removing a component invalidates its reference without adding lifecycle mutations to the
  backend protocol.

At 16,384 component scopes, an isolated update measured about 0.6 us median and p95 with eight
allocations and 313 bytes. The matching current Reactor benchmark measured about 172 us
median and 202 us p95 with eight allocations and 752.5 bytes. A Reactor2 effect-bearing update
measured about 0.8 us median and p95 with 12 allocations and 786 bytes, compared with about 163 us
median and 180 us p95 with 11 allocations and 969.5 bytes.

Changing the retained root type for one component among 16,384 scopes measured about 2.3 us median
and p95, seven to eight allocations, and 392 bytes. The component host passes the known parent
relation to the runtime, so this path does not scan the retained arena.

Retained memory at the same scale was about 799 bytes per Reactor2 idle component and 1,048 bytes
per effect-bearing component, including the retained object and recording adapter. Current Reactor
measured about 3,138 and 3,595 bytes per scope respectively. These numbers establish that lifecycle
state can remain separate from the UI representation without requiring a second retained UI tree.

Changing a context with one subscriber among 16,384 scopes measured about 0.8 us median and 0.9 us
p95. Changing a context consumed by all 16,384 scopes measured about 11.9 ms median and allocated
about 10.4 MB, compared with current Reactor's roughly 37.6 ms and 31.6 MB all-consumer path.

## Architecture verdict

The direction resolves the architectural problem exposed by issue #4972. Custom TreeView content
does not require a TreeView-specific public update model, a parallel flattened tree, or a special
component representation. It is an owned visual relation on a structural object and flows through
the same generated contract, retained arena, mutation planner, and backend policy as other
relations.

| Original goal | Result |
| --- | --- |
| Clean type-safe declarations | Proven for visual, structural, data, keyed, positional, and controlled-input contracts |
| One compact internal object model | Proven; declarations reconcile directly into one generational retained arena |
| Backend-specific optimization | Proven with sparse moves, dense `ReplaceAll`, stable ListView data, and native observations |
| Component lifecycle without another UI graph | Proven for heterogeneous keyed roots, inputs, messages, contexts, effects, references, tasks, timers, and retirement |
| Compile-time success implies valid relation shape | Proven with typed builders and compile-fail tests |
| Avoid generated-code growth | Proven; components add no generated control variants |
| Material end-to-end improvement | Proven on live Grid, ListView, and TextBox workloads |

The retained/control architecture should continue in the prototype, but production migration is
blocked on recursive component composition. `ComponentHost` proves the required ownership and
scheduling model, but its flat root collection is not the final application API. Heterogeneous
component declarations still need to compose recursively inside ordinary control relations
without adding a retained wrapper or cached expanded view. Production integration should reuse the
existing DispatcherQueue timer and Windows thread-pool services rather than the prototype's host
threads.

1. Component scopes may retain lifecycle state and one subtree `ObjectId`, but never a cached or
   mirrored UI declaration tree.
2. Every visual, structural, and data object remains in the same retained arena.
3. Component updates use targeted subtree reconciliation, including in-place owned-root
   replacement; removal uses generic relation mutations.
4. Native observations are applied before queued component messages and are never consumed by an
   unrelated subtree update.
5. New lifecycle features must not add control-specific planner paths or generated component
   variants.

## Migration stages

1. Move the generated contracts, retained arena, generic mutations, and adapter policy into
   `windows-reactor` behind its existing public control builders.
2. Migrate controls by relation category rather than one control at a time: scalar, owned visual,
   positional visual, keyed visual, structural, and container-generated data.
3. Route current component publications to targeted retained subtree updates while keeping the
   current public component traits and dispatcher/thread-pool implementations.
4. Replace the current component store internals with type-erased scopes, dependency-indexed
   contexts, effects, references, and generation-checked delivery.
5. Add recursive component declarations and then implement TreeView custom content as an ordinary
   structural-to-visual relation.
6. Remove the old planner and duplicated representations only after live Grid, ListView, TextBox,
   TreeView, component, memory, and allocation gates pass on the migrated implementation.

## Current thin slice

| Object | Purpose |
| --- | --- |
| `TextBlock` | Scalar property updates |
| `TextBox` | Controlled text, replaceable callbacks, and native feedback suppression |
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
cargo run -p reactor2-solitaire
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
| Keyed visual children | 1,754.7 bytes/object | 255.6 bytes/object | 539.5 bytes/object |
| Plain TreeView nodes | 213.9 bytes/object | 319.4 bytes/object | 751.1 bytes/object |
| TreeView nodes with visual content | - | 271.8 bytes/object | 625.7 bytes/object |

The graph-only rows are the internal representation gate. Recording-adapter totals are useful for
test-process sizing but are not retained arena cost. Retained events use a nullable shared pointer
because most controls have no event handlers; declarations retain the inline single-event form so
building a common one-handler control does not allocate.

The recording results are not sufficient for an architecture decision. `reactor-live-compare`
runs each frontend in a separate process with the same native `Grid`, 512 keyed `TextBlock`
children, deterministic mutations, 120 render-paced updates, allocator, and process metrics.
Stabilized native runs with 512 objects and 300 render-paced updates produced:

| Workload | Reactor avg | Reactor2 avg | Reactor bytes/update | Reactor2 bytes/update |
| --- | ---: | ---: | ---: | ---: |
| Change one text value | 627.4 us | 97.0 us | 216,904 | 59,085 |
| Rotate one child | 725.6 us | 220.7 us | 316,449 | 172,675 |
| Reverse all children | 3,835.6 us | 1,417.8 us | 411,641 | 179,791 |
| Remove/restore 64 children | 1,083.1 us | 864.6 us | 352,023 | 114,623 |

The initial native run failed badly because every reorder used
`UIElementCollection::ReplaceAll` and each declaration object allocated empty property and relation
vectors. Replacing dense synchronization with a generic move plan and storing zero-or-one
properties and relations inline removed those costs. Direct declaration reconciliation then
removed the transient object graph and full retained-graph clone. Reactor2 is faster in all four
average-latency workloads while allocating fewer objects and bytes. Churn p95 remains slightly
worse and is still a tail-latency optimization gate.

Shared immutable strings allow unchanged property values to flow from application state into
declarations without allocation. The keyed reconciler also detects unchanged key order before
allocating lookup and move-planning storage. Together these changes reduce a 512-object text update
to about 10 allocations and 59 KiB.

The same harness has a visible ListView surface for larger end-to-end runs. At 10,000 items,
Reactor2 sustained the render cadence for one changed item at 1.07 ms average and 2.54 ms p95, and
for alternately removing and restoring 100 items at 4.33 ms average and 9.18 ms p95. The matching
Reactor runs measured 5.57 ms and 137.86 ms average respectively, although the two frontends use
different native list item representations.

Reactor2 ListView data items keep stable native identity through an observable map bound by the
item template. Updating text mutates the item directly instead of performing a linear native
`IndexOf` and replacing the collection entry. At 10,000 items, native text-update p95 fell from
about 2.75 ms to about 8 us. Declaration construction uses two allocations per update when the
application model preserves unchanged strings as `Rc<str>`.

A dense 2,000-item reversal initially took 391 ms because the container adapter replayed every
move through native `IndexOf`, remove, and insert calls. The generic `Reorder` mutation also carries
the final order, allowing the ListView backend to select `ReplaceAll` for a complete reorder. That
reduced the update to 8.47 ms, compared with 10.29 ms for Reactor. Reversing all 10,000 items still
takes about 37 ms and remains outside the 60 Hz gate.

TextBox events use the same generated schema and generic mutation stream as properties and
relations. A native TextBox subscribes once and keeps the current callback in a replaceable slot,
so changing or removing the callback does not recreate the control or subscription. The backend
tracks the last observed native text. It updates that value before a programmatic setter and drops
matching native notifications, including repeated or delayed notifications. A different native
value is queued as a generic property observation before invoking the current callback. The runtime
applies queued observations to the retained graph before dispatching revision-checked events on a
later UI turn. Application callbacks therefore cannot reenter reconciliation from the native event
handler, and stale events do not reach replacement callbacks. A controlled rerender of the observed
value produces no mutation and no native setter call. Authoritative replacements preserve and
clamp UTF-16 selection indices.

The native self-test routes a simulated native text change through the same observed-text and
callback path as the WinUI event handler, rerenders the controlled value, and then applies a
different authoritative value. It verifies callback count, native text, selection preservation,
and delayed programmatic feedback suppression across message-loop turns. Raw keyboard injection
remains a benchmark concern because foreground-window activation is not deterministic enough for
the correctness fixture.

The matched `reactor-live-notepad --single-line` and `reactor2-live-notepad` benchmarks inject real
keyboard input into the same native TextBox configuration. With 1,000 measured characters,
Reactor2 used 4 allocations and 1,245 allocated bytes per input versus Reactor's 16 allocations and
3,433 bytes. Reactor2 emitted no mutation and made no native `SetText` call for controlled
feedback. Callback-to-reconcile latency was about 1.0 us median. End-to-end latency remained
dominated by WinUI and was within the run-to-run range of Reactor.

At an initial text size of 100,000 bytes, Reactor2 used about 200,285 allocated bytes per input
versus Reactor's 401,500 bytes. Reactor2 callback-to-reconcile remained about 1.7 us median, while
both end-to-end medians were about 41 ms. The remaining Reactor2 allocation is about twice the text
length because the minimal binding reads a `String` and the declaration retains an `Rc<str>`.

An isolated representation benchmark rejected a general switch to `HSTRING`. At 100,000 bytes,
constructing `Rc<str>` from UTF-8 took about 1.67 us and cloning it took 0.7 ns. Constructing
`HSTRING` took about 48.47 us and cloning it took 9.9 ns. The current
`HSTRING -> String -> Rc<str>` input path took about 139.15 us, while retaining the intermediate
`String` in `Rc<String>` took about 91.01 us. Saving that copy would add a second retained string
representation for less than 0.2% of measured end-to-end input time. Reactor2 therefore keeps
`Rc<str>` across declarations, retained state, and callbacks. An opaque native string type should
be reconsidered only if it can remain native across the complete application round trip.

## Prototype limits

- Component scopes, resources, transitions, and asynchronous work are not represented. Event
  contracts are proven only for TextBox `TextChanged`.
- The native stress fixture covers TreeView custom-content updates and reorder plus controlled
  TextBox input, but not accessibility behavior.
- Validation rejects declaration depth above 128 before reconciliation so recursive planner paths
  cannot overflow the stack.
- Validation and no-change matching reject graphs above 65,536 objects.
- Adapter validation or application failures poison the runtime and clear retained state. Dynamic
  declaration errors are rejected before retained or native mutation.
- `ComponentHost` currently places heterogeneous keyed roots under one retained container.
  Recursive component declarations and nearest-ancestor context providers remain to be designed.
- Component views currently return only `Visual`, so structural or data-rooted component scopes
  are not represented.
- Prototype timers and background work use cancellable host threads. Production integration must
  retain the current DispatcherQueue and Windows thread-pool implementations.
- Virtualization and full native lifecycle behavior remain outside the thin slice.
- Ten-thousand-item churn remains below the frame budget at p95, but its 250-operation native batch
  is the largest remaining backend cost in the current scale fixture.

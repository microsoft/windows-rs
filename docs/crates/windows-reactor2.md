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
TreeView nodes with custom visual content, exercises controlled TextBox input, and injects a real
mouse click through the WinUI pointer event path.

The standalone application host is exposed as `App`, `AppContext`, `AppProxy`, and `AppCallback`.
For unpackaged processes it adds the installed Windows App Runtime framework package to the process
graph, initializes per-monitor DPI awareness and the UI thread's STA, composes the WinUI
`Application` with the controls metadata provider, installs `XamlControlsResources`, and selects
explicit dispatcher shutdown. `App::run_with` retains its startup result until the message loop
ends. UI callback failures are returned from `run_with` after requesting application exit.
`AppContext::component_services` supplies Windows thread-pool execution and one-shot
`DispatcherQueueTimer` scheduling to `ComponentHost::mount_with_services`. The host owns that
service boundary and passes it to every component scope. `ComponentHost::mount` remains available
for recording tests and generic adapters through a default Windows thread-pool implementation
whose timers use cancellable waits rather than WinUI.
`WindowPolicy` keeps window title, root theme, client size, minimum client constraints, and custom
AppWindow title-bar configuration outside the generated control property schema.

`reactor2-counter` is the minimal recursive sample. The parent owns the count and event handler,
while a nested `Count` component receives the value through `Border.Content`. This keeps the sample
small while proving that event delivery, parent state, child input, and targeted retained updates
work together.

`reactor2-solitaire` is the first application-shaped sample. It shares the original Solitaire
rules and presents a scaled green board with positioned cards, suit colors, face-down cards,
foundation and tableau slots, direct card actions, move highlights, and failure feedback. Its
`Solitaire -> Board -> keyed CardView` hierarchy preserves card component identity across pile
moves. Reactor2 supplies the Windows App Runtime bootstrap and WinUI application host without
creating a separate framework window. Its window policy matches the original sample's title, dark
theme, 800x600 client and minimum sizes, and tall AppWindow title bar.

The port added solid-color, thickness, and corner-radius property values to the generic adapter
protocol. Border styling, inherited layout properties, pointer release, Viewbox, and TitleBar then
entered through `schema.toml` and generated realization without planner or control-specific
adapter branches. AppWindow title-bar height and client-size policy remain application-host state.
Reposition transitions use the generated visual-property contract and apply
`ThemeTransition::Reposition` to each keyed `CardView` root. Pointer release uses the generated
typed event path even though Solitaire's click logic does not inspect its payload.

`reactor2-explorer` is the recursive structural sample. Component-rendered TreeView rows support
filtering, root reorder, explicit selection, expansion, asynchronous child loading, cancellation
through scope retirement, and an independently updated details component. Native TreeView
selection is not projected yet, so row buttons currently send selection messages.

`reactor2-workbench` is the composition sample. It combines navigation, controlled form input,
keyed project rows, shared theme context, selection details, and row-owned background work.
Acceptance tests verify keyed identity across reorder, isolated row updates, form-driven insertion,
targeted context invalidation, page retirement, and stale background-message rejection.

## Complexity ladder

Samples advance only when each rung defeats a distinct source of framework complexity:

| Rung | Workload | Required proof |
| --- | --- | --- |
| Counter | Parent state, click event, nested display component | One child subtree changes; no wrapper or parent rebuild |
| Solitaire | Controlled input, commands, game state, three component levels, keyed board lines | Stable child identities and bounded mutations during application-shaped churn |
| TreeView explorer | Structural nodes, arbitrary component content, expansion, selection, async loading, reorder | No duplicated hierarchy ownership; live reorder and cancellation remain safe |
| Complex application | Multiple views, navigation, shared context, forms, lists, background work, secondary state | Features compose without new planner branches, component objects, or parallel UI trees |

Each rung must preserve the architectural invariants rather than only render correctly:

1. New ordinary controls enter through schema and metadata generation.
2. Backend exceptions remain native policy and do not change declaration or planner semantics.
3. Components retain lifecycle state and a root `ObjectId`, never rendered declarations.
4. A child update produces mutations only inside that child's retained subtree.
5. Keyed reorder preserves component state and retained root identity.
6. Removal cancels tasks, cleans effects, invalidates references, and rejects stale messages.
7. Recording tests prove mutation shape; live WinUI tests prove behavior that recording cannot see.
8. Retained bytes, allocations, update latency, mutation count, and handwritten framework code are
   measured at every application rung.

The recursive component benchmark builds balanced trees and updates the deepest leaf. A release run
with 5,000 samples produced:

| Scopes | Depth | Retained bytes/scope | Median | P95 | Allocations/update | Bytes/update | Mutations/update |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 585 | 3 | 1,113.1 | 0.8 us | 0.9 us | 11 | 1,463.8 | 1 |
| 4,681 | 4 | 1,087.6 | 0.8 us | 0.9 us | 11 | 1,463.8 | 1 |
| 21,845 | 7 | 1,002.8 | 0.9 us | 0.9 us | 11 | 1,463.8 | 1 |

The retained measurement includes component state, the retained object graph, and the recording
adapter. Timed updates disable recording-adapter batch validation because that diagnostic clones
the complete adapter state. Constant latency, allocation cost, and one mutation across the three
sizes show that a nested message does not reconcile unrelated component scopes.

Counter, recursive Solitaire, and Explorer required no changes to `component.rs`, `reconcile.rs`,
the declaration schema, or generated control realization after the recursive component baseline.
The Workbench combines those features without changing the framework.
The single-window sample bootstrap added Window closed-event projection and lifecycle plumbing, not
a planner branch or retained component representation. The handwritten WinUI exceptions remain
the same three focused families: controlled TextBox feedback, TreeView structural content, and
ListView container data/templates.

`tool-reactor2` now generates typed declaration builders from `schema.toml`. The generated surface
covers all current prototype objects, and the public `Button` slice uses the generated builder with
an owned visual `Content` relation and unit-valued `Click` event. Button creation, native content,
and event delivery reuse the existing retained relation and queued-event protocols without changes
to the planner or component lifecycle.

The same schema now represents all 79 control classes from the old Reactor schema. Ordinary WinUI
controls use generated native realization; TextBox feedback, TreeView structural nodes, ListView
container data, and templates remain focused handwritten adapter cases. The generated surface
covers string, `bool`, `i32`, `f64`, nullable boxed `bool`, solid color, thickness, corner radius,
and metadata-derived enum properties. It also covers validated numeric properties, keyed and
positional panel children, single-child native relations, Canvas attached positioning, unit
events, typed pointer events, and visual theme-transition collections.

`cargo run -p tool-reactor2 --quiet -- --convert-old-schema` is a one-shot migration command. It
reads the old schema only for that explicit command, merges metadata-backed direct contracts into
the current Reactor2 schema, and writes ordinary Reactor2 TOML to standard output. Normal
generation reads only `schema.toml`. The strict parity report currently accounts for 79/79
controls, 191/233 properties, 37/68 events, 42/42 slots, 3/3 selection contracts, 157/158
capabilities, and 0/2 lifecycle contracts. It includes inspectable strings and string lists,
distinct NumberBox and RatingControl optional numeric values, checked selection indices,
controlled and coercing property feedback backed by native events, and renamed event-builder
fields. Feedback expectations are keyed by retained object and event. Exact native echoes are
suppressed, normalized observations are deferred until the native setter returns, and clear
operations install the corresponding default expectation. Event payload contracts read supported
values from native event arguments or the event's declared source property; only matching value
types count toward parity. Native collection contracts cover inspectable, typed, and observable
vectors plus ItemCollection, while typed child and allowed-object contracts preserve native and
schema item restrictions.

NavigationView, ListBox, and SelectorBar selection uses one generated contract over keyed owned
relations. The retained child `ObjectId` is the stable selection identity, while callbacks receive
the selected item's optional Tag or Text. Application-driven item selection and collection changes
suppress owner feedback. Native changes update controlled child `IsSelected` properties and queue
one callback. Keyed reorder preserves the selected native object, selected-item removal clears it,
and observations queued for an item removed before dispatch are ignored. The selection descriptor
also records where the event's selected item comes from. NavigationView reads
`NavigationViewSelectionChangedEventArgs.SelectedItem`; ListBox and SelectorBar read their owner
selection properties. Parity requires this source and event-args type to match the old contract.
The remaining surface includes dependency-property-only feedback, more routed and typed event
payloads, ToolTip placement, ContentDialog lifecycle, and the remaining shared capability families.

`PointerEventInfo` carries element-local and window-relative coordinates, pointer identity,
left/right/middle button state, current capture state, and an optional capture-attempt result.
`PointerReleased` leaves the capture-attempt result unset because release does not initiate
capture. Attached and visual properties are ordinary retained properties on the child visual. The
adapter applies them through the owning WinUI class and clears the dependency property when
omitted. Theme transitions create a WinUI `TransitionCollection` owned by
`UIElement.Transitions`; each `ThemeTransition::Reposition` entry creates a
`RepositionThemeTransition`. The generated `GeneratedHandle` owns native construction,
object-kind and UIElement conversion, direct properties, panel children, content attachment, event
subscription, and callback lookup. `tool-reactor2` verifies setter ABI shapes and resolves
property, dependency-property owner, content, event, and enum information through the metadata
resolver shared with `tool-reactor`; it also derives the binding filter needed by those generated
paths. Generated mutable controls report native feedback into the retained graph even when no
callback is installed.

Width, height, min/max sizing, margin, horizontal and vertical alignment, opacity, and transition
collections are owner-aware shared visual contracts. Grid, RelativePanel, Canvas, and automation
attached properties use the same metadata-checked declaration and native paths. Missing values
clear the dependency property unless the old contract specifies an explicit clear value, such as
the empty automation name and ID. Local properties cannot collide with a shared property.

The `layout` capability includes generic retained exit retirement. Removing a transitioned visual
from an owned child collection removes it from active keys and order immediately, clears references
and event callbacks, and reserves its generational object slots while the native subtree remains
attached. WinUI applies a `ScalarTransition` to opacity and queues completion in the same
chronological native-occurrence stream as observations and callbacks. Completion removes the native
subtree and frees the retained slots only after earlier occurrences have been processed. Recording
adapters use the same command and completion path. Reusing the same key creates a new object
generation, stale events cannot target retiring nodes, concurrent retirements complete
independently, and removing a parent forces its pending child retirements to finish before parent
destruction. Component scopes, effects, tasks, and contexts retire with logical ownership rather
than waiting for the fade.
Reconciliation journals each touched retained slot and structural operation, so an unsupported
transition rolls back without cloning the full graph or partially changing earlier retirements,
references, and events. Native observations are applied before declaration planning and remain
visible if planning fails. Duplicate or stale completion notifications are ignored. Zero-duration
fades are disabled, single-child attachments reject exit retirement, and dropping the WinUI adapter
stops outstanding timers. `NativeWindow` does not own the runtime, so closing a window does not
cancel retirement work while the runtime remains alive.

`enabled` uses one shared `Control.IsEnabled` contract, including handwritten TextBox realization.
`focus` uses generated capability membership, typed `ElementRef` attachment, and
`Runtime::focus`; recording and WinUI adapters implement the same command. `text_style` is an old
schema marker rather than a generator mixin. Its three members count only after their explicit
properties are represented; TextBlock font weight uses a checked `FontWeight` value.
An `ElementRef` may occur once in a declaration tree. Validation rejects duplicates before native
mutation, while conditional cleanup makes transfers, replacement, detachment, and destruction
independent of reconciliation order.

Grid, Image, WebView2, and SwapChainPanel use generated typed reference attachment. Their
imperative APIs cover composition child visuals, native image sources, CoreWebView2
initialization, swap-chain assignment, rendering requests, and native metric observations.
References publish only after a native transaction commits. Each binding has a generation used by
queued requests, asynchronous completions, and observation callbacks, so work from an earlier
object cannot reach a rebound reference. Observation handles revoke their native subscriptions on
drop. Queue overflow, shutdown, runtime poison, and runtime drop complete pending one-shot
requests as unavailable rather than leaving them unresolved. Native observation failures enter the
adapter error stream and poison the runtime through the same boundary as other adapter failures.

Hand-authored properties with a schema default restore that explicit value when removed. Imported
properties without a known default clear the declaring dependency property instead, allowing the
native control or style default to apply. The generator resolves the declaring class through
metadata rather than assuming the dependency property is declared by the concrete control.

## Public API shape

The declaration API exposes one typed path for each generated contract:

| Contract | Public shape |
| --- | --- |
| Authoritative scalar value | Required constructor input |
| Optional scalar or event | Builder method; omission removes it |
| Attached visual property | Builder method on every visual; omission clears the dependency value |
| Visual collection property | Typed iterator builder on every visual; omission clears the dependency value |
| Optional owned child | Builder method; omission represents no child |
| Positional visual relation | Iterator of `Visual` |
| Keyed visual relation | Iterator of `KeyedVisual` created by `keyed` |
| Structural or data relation | Iterator of its specific keyed declaration type |

This keeps invalid category and identity combinations from compiling. Compile-fail doctests cover
visual, keyed visual, structural, and data relation boundaries. Controlled `TextBox` text remains a
required constructor input because an unset value would create a second authority model. There is
no separate common and advanced control path.

`Property`, `Event`, `Observation`, `NativeEvent`, `EventDispatch`, and their payload enums form the
public adapter protocol rather than the application declaration API. External adapters consume
generic mutations and expose one ordered native-occurrence stream. Each occurrence carries an
observation and callback, a retirement completion, a virtual realization or recycle request, or
either half of an observed callback. The runtime validates pending state changes transactionally,
then applies each occurrence in native order. A stale callback revision suppresses only callback
dispatch; its paired observation still updates retained state. Occurrences for replaced or retired
native objects are discarded by the adapter or ignored by the generational graph.

This protocol intentionally replaces the earlier split observation, callback, and retirement
drains. `windows-reactor2` is version `0.0.0`, so no deprecated compatibility methods preserve the
semantically incorrect batching model. `Adapter::focus` remains required because the adapter error
type has no generic way to construct a clear unsupported-operation error.

`Runtime::next_native_event` returns an RAII dispatch boundary. It remains active while the callback
and its component reconciliation run, and dropping it permits the next occurrence. Unwinding a user
callback drops the boundary automatically. Callback panics do not poison the runtime; after the
caller catches the panic, queued occurrences may continue. Invalid adapter input or adapter
validation/application failure still poisons the runtime explicitly.

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
factories. Components compose recursively inside ordinary visual relations. Their keys are local
to the parent component, while paths provide typed access to nested senders, inputs, and
references:

- messages are queued and never run inline with native callbacks;
- each native callback's component messages reconcile before the next native occurrence is applied;
- equal parent inputs do no work, while changed inputs reconcile only the owning subtree;
- a state change reconciles only the owning retained subtree;
- the retained root identity and `ElementRef` remain stable across updates;
- keyed child reorder preserves both component state and retained root identity;
- transient expansion marks component roots only long enough to recover their `ObjectId`;
- effects compare typed dependencies, clean up before replacement, and clean up on retirement;
- typed contexts maintain a reverse dependency index and rerender only subscribed scopes;
- background work and timers are scope-owned, bounded, cancellable, and wake the host once per
  pending batch;
- executor and timer implementations are host services, so component lifecycle code does not
  depend on WinUI;
- completion handles can cross threads, while generation checks discard delivery after retirement;
- removing a component invalidates its reference without adding lifecycle mutations to the
  backend protocol.

At 16,384 component scopes, an isolated update measured about 0.6 us median and p95 with eight
allocations and 313 bytes. The matching current Reactor benchmark measured about 172 us
median and 202 us p95 with eight allocations and 752.5 bytes. A Reactor2 effect-bearing update
measured about 0.8 us median and p95 with 12 allocations and 786 bytes, compared with about 163 us
median and 180 us p95 with 11 allocations and 969.5 bytes.

Changing the retained root type for one component among 16,384 scopes measured about 2.3 us median
and p95, seven to eight allocations, and 392 bytes. Nested root replacement asks the retained graph
for the component root's actual owning relation, so the same path works in content, panel, and
TreeNode relations.

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
| Component lifecycle without another UI graph | Proven for recursive keyed scopes, inputs, messages, contexts, effects, references, tasks, timers, reorder, and retirement |
| Compile-time success implies valid relation shape | Proven with typed builders and compile-fail tests |
| Avoid generated-code growth | Proven; components add no generated control variants |
| Material end-to-end improvement | Proven on live Grid, ListView, and TextBox workloads |

The retained/control architecture should continue toward production migration. Recursive
components now compose in ordinary owned relations without adding a retained wrapper, cached
expanded view, component mutation, or planner branch. Recording tests cover nested input and
message updates, root replacement, keyed reorder, removal, effects, stale delivery, and
parent-local keys. Explorer acceptance tests also cover application-level selection, filtering,
generation-safe row replacement, and stale row senders. The live fixture repeatedly reorders
realized TreeView nodes whose custom content is produced and updated by nested components.

The recursive memory, latency, allocation, and mutation-radius gates pass through 21,845 scopes.
Nearest-ancestor context providers and structural or data-rooted component scopes remain before
migration. Typed pointer payloads and transition declarations now use generated protocol and
native adapter paths. The standalone host owns AppWindow policy, Windows thread-pool background
execution, and DispatcherQueue timers without coupling generic component lifecycle code to native
bindings.

1. Component scopes may retain lifecycle state and one subtree `ObjectId`, but never a cached or
   mirrored UI declaration tree.
2. Every visual, structural, and data object remains in the same retained arena.
3. Component updates use targeted subtree reconciliation, including in-place owned-root
   replacement; removal uses generic relation mutations.
4. Native observations and callbacks retain native order. One occurrence's observation is applied
   before its callback, and callback-driven component reconciliation completes before the next
   occurrence.
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
| `ItemsRepeater` | Lazy keyed visual realization with recyclable native containers |

`VirtualSource` stores a key revision, logical length, key projection, and lazy view projection.
Key projections define stable identity without creating visual declarations. WinUI element-factory
requests enter the ordered native stream; the component host expands only requested rows and owns
their nested component scopes until recycle. Source updates remap active containers by logical
index while retaining matching keyed subtrees, reject stale source and lease revisions, and remove
rows whose keys leave the realized window. Recording tests and the native self-test use a
10,000-item source while retaining only realized visual objects.

## Validation

Run:

```text
cargo run -p tool-reactor2 --quiet
cargo run -p tool-reactor2 --quiet -- --parity-report
cargo test -p windows-reactor2 -p tool-reactor2 --quiet
cargo clippy -p windows-reactor2 -p tool-reactor2 -p test-reactor2-bench \
    --all-targets -- -D warnings
cargo run -p test-reactor2-bench --release --quiet
cargo run -p test-reactor2-selftest --quiet
cargo run -p reactor2-solitaire
cargo run -p test-reactor-bench --bin reactor-live-compare --release --quiet -- \
    --frontend reactor2 --workload text --count 512 --updates 120
```

`--parity-report` compares the complete Reactor `winui.toml` surface with Reactor2's schema. It
counts controls, properties, events, slots, selections, capabilities, and lifecycle contracts,
then groups every unresolved contract by the missing model or behavior. `--check-parity` emits the
same report and exits unsuccessfully while anything remains unresolved. The schema contains all 79
old Reactor controls plus Reactor2's data and structural objects.

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
value and its callback form one ordered occurrence. The runtime applies that observation, invokes
the revision-checked callback, reconciles resulting component messages, and only then advances to
the next occurrence. If the callback revision changed while the occurrence was queued, the observed
value still updates retained state but the old callback is not invoked. Application callbacks
therefore cannot reenter from the native handler, and an earlier callback cannot observe a later
native value. A controlled rerender of the observed value produces no mutation and no native setter
call. Authoritative replacements preserve and clamp UTF-16 selection indices.

The native self-test routes simulated native text, password, rating, toggle, slider, and selected
index changes through the same observations and typed callbacks as the WinUI event handlers. It
rerenders controlled values and verifies callback counts, native state, selection preservation,
and delayed programmatic feedback suppression across message-loop turns. NavigationView, ListBox,
and SelectorBar fixtures also verify initial controlled selection, one callback per native change,
queued NavigationView changes retaining their event-specific payloads, keyed reorder, and
selected-item removal. The test injects a real mouse press and release into a Reactor2 Border. This
exercises `PointerRoutedEventArgs`, validates the typed payload, and verifies that a missing
pointer-capture collection means "not captured." Raw keyboard injection remains a benchmark
concern because foreground-window activation is not deterministic enough for the correctness
fixture.

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
- Component roots must be visual declarations. A component cannot directly delegate its root to
  another component, and structural or data-rooted component scopes are not represented.
- Contexts remain host-global; nearest-ancestor context providers remain to be designed.
- Component views currently return only `Visual`, so structural or data-rooted component scopes
  are not represented.
- Prototype timers and background work use cancellable host threads. Production integration must
  retain the current DispatcherQueue and Windows thread-pool implementations.
- Virtualization and full native lifecycle behavior remain outside the thin slice.
- Ten-thousand-item churn remains below the frame budget at p95, but its 250-operation native batch
  is the largest remaining backend cost in the current scale fixture.

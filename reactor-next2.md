# Windows Reactor Next - Working Architecture

## Objective

Build a small declarative WinUI core that is easier to reason about than `windows-reactor` and the
abandoned Reactor2 design while retaining competitive compile time, update cost, memory use, and
native behavior.

This document is authoritative for the current prototype. Earlier fine-grained recovery work was
an experiment and is not part of the selected architecture.

## Core decisions

### Trust the platform

Reactor validates everything it can before calling WinUI. Once native mutation begins, WinUI is
expected to satisfy its documented contract.

An unexpected native command failure is fatal:

1. Stop at the first failing command.
2. Report the command index and native error.
3. Do not publish the candidate.
4. Abort the process in the production WinUI host.

Do not interpret partial success, retry properties, repair structure, remount controls, rebuild the
window, or continue dispatching work. The failure indicates a Reactor bug or a platform bug that
must be fixed.

A narrow workaround may be added later for a real, repeatable platform defect. It must not become
a generic recovery engine.

Documented subsystem failures remain adapter concerns. Canvas device loss and WebView process loss
do not change the failure policy for ordinary WinUI controls.

### One ownership graph

The published `Tree` is the only authority for parent, key, type, and child order.

```text
ApplicationHost
  -> WindowHost
       -> component scope storage
            component instances, state, effects, message queues
       -> structural tree
            native nodes, fragments, component boundaries, scope references
       -> WinUI runtime
            COM handles and subscriptions
```

The component store owns non-cloneable component instances. It does not own another structural
graph. Candidate trees may reference reserved component scopes but never clone component state.

### Candidate publication

```text
application/component state
  -> desired View or Element
  -> validated candidate tree
  -> ordered native commands
  -> apply all commands successfully
  -> publish candidate and effects
```

Planning errors occur before native mutation and return ordinary `PumpError` values. Native apply
returns either success or `NativeApplyError { command, error }`.

Candidate construction remains transactional because it prevents Reactor from publishing invalid
logical state. It is not a native rollback or recovery mechanism.

### Identity

`WindowToken` identifies a logical window lifetime. Shutdown advances its epoch and rejects stale
events, dispatcher work, async completions, and component senders.

Native remount identity was removed because automatic remounting was removed. Virtual collection
leases retain their own generation checks.

Child identity has two domains. Positional children use an internal `Position` key assigned from
their index, but only statically shaped `IntoViews` expressions can request that identity.
Explicit children use public integer or string `Key` values. `Key` is opaque, so application code
cannot construct a positional key or collide with that domain. Dynamic collections require
explicit keys, which prevents the React index-as-key state bug where an insertion shifts retained
component state onto different items.

Effect identity is always explicit. `EffectKey` is an opaque numeric or string key with no
positional variant. The dependency remains a separate typed `PartialEq` value. Conditional
omission and reordering therefore do not reassign effect slots, and owned components have no
positional hook-order contract.

## Component model

The owned-component frontend remains the primary direction:

```rust
trait Component: 'static {
    type Props: Clone + PartialEq + 'static;
    type Message: 'static;

    fn create(props: &Self::Props, context: &mut ComponentContext<Self>) -> Self;
    fn changed(&mut self, _props: &Self::Props, _context: &mut ComponentContext<Self>) {}
    fn update(&mut self, message: Self::Message, context: &mut ComponentContext<Self>);
    fn view(&self, props: &Self::Props, context: &mut ViewContext<Self>) -> View;
}
```

Rules:

- Component identity is parent boundary plus key plus component type.
- Same key and type retain the scope across props and movement.
- Same key and different type replace the scope.
- The component store owns current props and borrows them into `view`.
- Sends enqueue typed messages and never call component code reentrantly.
- Props apply parent-first before surviving descendant messages.
- Retirement drops queued work for removed descendants.
- One turn composes each dirty scope at most once.
- Local leaf updates do not clone the full tree.
- Typed context reads resolve to a specific logical provider and publish with the candidate.

Components and `View` are the only public frontend. The hook frontend was retained for early
comparison measurements, then removed before the API freeze so the core has one state and effect
model.

Generated controls convert directly to `View`. `ContentControl::content`,
`ChildrenControl::children`, `ChildrenControl::keyed_children`, and `SlotsControl::slots` consume
the control and return `View`. These are terminal composition methods on the core model, not a
wrapper DSL. The sealed `IntoViews` trait accepts `()`, fixed arrays, and heterogeneous tuples up
to 16 elements. It does not accept `Vec`, slices, arbitrary `IntoIterator` inputs, or iterator
adapters. Positional inputs first become `Vec<View>` and then private positional keyed edges before
planning, so `ViewKind::Children` and `ViewKind::Fragment` remain the only collection planner
paths.

## Logical anchoring

`Fragment` is a logical node and creates no hidden WinUI control. It may represent zero, one, or
many native roots.

- Generated children collections accept many flattened roots.
- Window and content slots accept zero or one flattened root.
- Invalid arity fails planning before native mutation.
- `View::fragment` assigns positional identity to a static shape, and `View::keyed_fragment`
  accepts explicit keys for dynamic collections.
- Exact fragment order uses one coalesced `SynchronizeChildren` command per native parent.
- Ordinary keyed children retain sparse insert and move plans.

## Controlled properties

Controlled native events are observations, not failures.

```text
desired value
known native value
property-specific feedback contract
```

A trusted event payload updates the known native value before the application callback. If the
application keeps the old desired value, normal reconciliation emits a restoring write.

There is no divergent state, retry counter, or repair scheduler. If the restoring WinUI setter
unexpectedly fails, the native failure is fatal.

The generated slice accepts synchronous exact and synchronous normalized feedback. Exact feedback
suppresses only the setter's expected payload. Normalized feedback captures the last matching event
during the setter because WinUI may coerce the value. Reactor records that payload as known native
state without invoking the application callback or scheduling an immediate retry. A later
application update can then restore the desired value when the coercing constraint changes without
looping while the desired value is impossible.

NumberBox declares `Minimum` and `Maximum` as coercers of `ValueChanged`, orders both before
`Value`, and observes only `Value`. Floating property and event comparisons treat two NaN values as
the same empty numeric state. Deferred and unknown feedback contracts fail generation rather than
guessing.

## Turn and lifecycle order

One window turn:

1. Drain and validate native observations.
2. Apply parent props.
3. Drain component messages by enqueue sequence.
4. Compose dirty scopes parent-first.
5. Validate the complete candidate without running more user code.
6. Prepare retiring and changed effect cleanup child-first.
7. Apply native commands.
8. Publish tree, scopes, desired state, and version.
9. Run new and changed effect setup parent-first.
10. Drain imperative commands against the published window and node identities.

An unexpected failure in step 7 aborts the production process. Effect setup and candidate
publication do not run.

Normal shutdown cleans effects before dropping native resources. Cleanup remains idempotent across
explicit shutdown and `Drop`.

Keyed component effects follow the same boundary: changed cleanup runs after candidate validation
and before native apply, while setup runs after publication. Removed and changed cleanup follows
published key order; setup follows new registration order. Duplicate keys fail planning before
either phase, and the retry discards only pending registrations while retaining published slots.
If component props are applied before later planning fails, touched scopes remain planning-dirty
so an identical-props retry recomposes rather than accepting stale structure.

## Scheduling

- Native callbacks enqueue work and return.
- Typed element references enqueue imperative work and wake their owning Pump.
- Each dispatcher turn handles at most 64 events, 64 component messages, and 32 realizations.
- Remaining work rearms the scheduler.
- Component messages are capped at 4,096 per window and expose backpressure.
- Typed event-message callbacks preserve that backpressure result.
- Scheduler rejection is an explicit host fault.
- Work queued during dispatch is rearmed after the current turn.

The budgets count queued items. They do not preempt one component `view` call or split one candidate
plan. Adding a general composition continuation would restore much of the state-machine complexity
that this design removed. Large trees are instead governed by locality and keyed-scale gates.

Dispatcher rearming schedules remaining bounded work after the current callback. A failed enqueue
is surfaced as a host fault; neither behavior retries native mutation.

Generated payload event setters accept unit-returning `Fn(T)` closures through
`IntoPayloadCallback<T>`, and zero-argument setters accept `Fn()` through `IntoUnitCallback`.
Both also accept `Callback<T>`.
`ViewContext::callback` maps payloads to component messages, and `ViewContext::message` clones a
fixed message for each zero-argument invocation. Both forward to `LocalSender`, so delivery still
uses the local component queue.

`Callback::call` returns `true` for ordinary closures and the exact `LocalSender::send` result for
message adapters. A current event that receives `false` stays at the front of the Pump queue while
the host drains component work, then retries on a later turn. Window, node, subscription, and
event-revision validation runs first, so stale work is discarded without invoking the callback.

## Generated and specialized controls

Ordinary control metadata drives:

- Public typed builders.
- Minimal WinUI bindings.
- Handle variants.
- Property setters and clear operations.
- Structural roles.
- Event subscriptions and payload conversion.

Unsupported metadata shapes fail generation. Specialized adapters such as `ItemsRepeater`, Canvas,
and WebView own only behavior that cannot be expressed by the ordinary schema.

## Evidence

### Performance

| Metric | Result |
| --- | --- |
| Thin counter clean compile ratio | 0.40x current reactor |
| Thin counter source-only rebuild ratio | 0.18x current reactor |
| Thin counter release executable ratio | 0.29x current reactor |
| NumberBox source-only rebuild delta | Within measurement noise: 1.228 -> 1.237 seconds |
| NumberBox thin release delta | 828,928 -> 847,360 bytes: +18,432 bytes, +2.22% |
| NumberBox retained core layouts | Unchanged: `Node` 416, `MountedProps` 72, `Element` 80 bytes |
| Slider median source-only rebuild delta | 1.290 -> 1.330 seconds: +3.1% |
| Slider thin release delta | 847,360 -> 858,624 bytes: +11,264 bytes, +1.33% |
| Slider PE section delta | `.text` +8,416 bytes; `.rdata` +3,008 bytes |
| Slider retained core layouts | Unchanged: `Node` 416, `MountedProps` 72, `Element` 80 bytes |
| NavigationView slots source-only rebuild | 0.530 -> 0.493 seconds: within noise |
| NavigationView slots thin release delta | 886,784 -> 905,216 bytes: +18,432 bytes, +2.08% |
| NavigationView slots PE section delta | `.text` +17,136; `.rdata` +880; `.pdata` +792 bytes |
| NavigationView slots retained layouts | Unchanged: `Node` 416, `MountedProps` 72, `Element` 80 |
| ProgressBar source-only rebuild | 0.471 -> 0.503 seconds: +7.0%, 33 ms |
| ProgressBar thin release delta | 905,216 -> 916,992 bytes: +11,776 bytes, +1.30% |
| ProgressBar PE section delta | `.text` +8,768; `.rdata` +2,928; `.pdata` +288 bytes |
| ProgressBar retained layouts | Unchanged: `Node` 416, `MountedProps` 72, `Element` 80 |
| ToggleSwitch source-only rebuild | 0.466 -> 0.469 seconds: +0.5% |
| ToggleSwitch thin release delta | 916,992 -> 929,792 bytes: +12,800 bytes, +1.40% |
| ToggleSwitch PE section delta | `.text` +9,152; `.rdata` +3,112; `.pdata` +468 bytes |
| ToggleSwitch retained layouts | Unchanged: `Node` 416, `MountedProps` 72, `Element` 80 |
| Isolated component leaf at 512 scopes | 0.51 us, 430 bytes, 9 allocations |
| Isolated component leaf at 16,384 scopes | 0.51 us, 430 bytes, 9 allocations |
| Idle component memory | About 2,440 bytes per scope |
| Ordinary keyed reversal, 512 -> 4,096 | 0.19 ms -> 2.15 ms |
| Component same order, 512 -> 4,096 | 0.42 ms -> 4.09 ms |
| Component reversal, 512 -> 4,096 | 0.49 ms -> 5.76 ms |
| Component 10-25% movement at 4,096 | 5.0-5.4 ms |
| Context provider, 512 -> 16,384 unrelated scopes | 3.8 us -> 4.3 us |
| Broad provider with one consumer, 512 -> 16,384 descendants | 3.6 us -> 3.7 us |
| One provider update among 512 -> 16,384 providers | 4.1 us -> 4.7 us |
| Background task, 512 -> 16,384 unrelated scopes | 69 us -> 66 us |

The local component path remains close to the current `windows-reactor` baseline while using fewer
allocations than the recovery prototype.

### Complexity reduction

Removing fine-grained recovery produced these immediate changes:

| Area | Recovery prototype | Fatal-failure core |
| --- | ---: | ---: |
| `pump/publish.rs` | 396 lines | 57 lines |
| `pump/mod.rs` | 556 lines | 412 lines |
| `pump/plan.rs` | 107 lines | 71 lines |
| Pump unit tests | 136 tests | 107 focused tests |

Removed concepts include per-command outcome vectors, skipped outcomes, divergent properties,
retry attempts, structural remount plans, realization remount epochs, pending recovery,
continuation budgets, aggregate recovery receipts, and recovery-specific live fixtures.

### Failure evidence

The repository's concrete reactor reports concern reconciliation, component scheduling, generated
property coverage, nested content, and startup policy. The review found no recurring arbitrary
WinUI mutation failures that justify a generic recovery engine.

## Gates

| Gate | Required result |
| --- | --- |
| Ownership | One structural authority; candidate state never aliases component state |
| Publication | Planning completes before apply; publication follows full native success |
| Native failure | Stop at first error; production host reports and aborts |
| Reentrancy | Native callbacks and component sends are queue-only |
| Props | Parent props precede surviving descendant messages |
| Anchoring | Empty, single-root, multi-root, and pass-through views remain local |
| Lifecycle | Cleanup and setup ordering is deterministic and one-time |
| Scheduling | Bounded work, backpressure, and no lost wakeup |
| Locality | 512 -> 16K unrelated scopes changes leaf time by <= 25% |
| Compile time | Component <= 1.25x equivalent hook application |
| Binary size | Component <= 1.20x equivalent hook application |
| Keyed scale | No quadratic behavior through 4,096 siblings |
| Live evidence | Templates, input, two windows, repeater reuse, and shutdown |

### Future expansion gate

`compare.md` is a useful replacement review and expansion checklist. Its context-scaling and
owner-scoped background-task gaps describe older revisions; exact context consumer invalidation,
chunked provider storage, task ownership, cancellation, queue bounds, and live delivery are now
implemented. The remaining guidance is the gate for growing beyond the initial API:

| Expansion | Required proof |
| --- | --- |
| Coercing controls | Slider or NumberBox fits generated feedback contracts without Pump state |
| Multiple slots | Passed: NavigationView uses generated roles rather than control branches |
| Templates | ItemsRepeater passes recycling, local-value, selection, move, and key tests |
| Third-party controls | Extension contracts add native behavior without a runtime type registry |
| Generated scale | Each difficult slice records compile, binary, and core-layout deltas |
| Multi-window failure | Process-fatal native failure remains an explicit product decision |
| Large composition | A practical render bound is documented or a small continuation is proven |

Keep the current reactor's retained controls, stable keys, generated mappings, explicit cleanup,
templates, collections, and multi-window behavior. Keep Reactor2's generational identity, queued
work, controlled observations, and explicit ownership. Use the C# Reactor as evidence for
control-specific WinUI behavior, not as a runtime design to copy.

The stop condition is concrete: if a feature requires new Pump state, a handwritten reconciliation
path, another authoritative side table, or generic recovery machinery, stop and revise the
contract before adding it. Difficult vertical slices come before dozens of ordinary controls.

NumberBox passes the coercing-control correctness gate through schema and generated backend
contracts. Its live fixture tightens a bound below the retained desired value, confirms the
coerced value and zero application callbacks, relaxes the bound, and confirms that the desired
value is restored. Headless tests prove silent known-native observation and NaN idempotency. This
added no Pump state or control-specific reconciliation path.

The control-cost gate remains open. NumberBox did not change source-only rebuild time or retained
core layouts, but it added 18,432 bytes to the thin release counter. The PE sections grew by 12,256
bytes of executable code and 4,680 bytes of read-only data. Generated backend dispatch is
centralized over runtime control and property IDs, so the linker cannot discard every unused-control
branch. Do not extrapolate broad coverage from the favorable compile result alone. Measure a
representative control batch and decide whether control feature partitioning is needed before
adding dozens of controls.

For each difficult control slice:

1. Compare the parent and candidate with isolated targets and `CARGO_INCREMENTAL=0`.
2. Record source-only rebuild time for the thin counter.
3. Record the release executable and `.text`, `.rdata`, `.pdata`, and `.reloc` deltas.
4. Run the core-layout test; growth in `Node`, `MountedProps`, or `Element` requires an explicit
   design review.
5. Recalculate the projected cost of the curated control set against the equivalent current
   reactor application.

One control does not justify feature partitioning. A representative batch must show whether the
slope remains roughly linear and whether optional control groups would save enough to offset their
generator, documentation, and user-facing complexity.

Slider provides the second controlled cost point. It reused the normalized range contract through
schema alone and passed the same live tighten/coerce/relax/restore sequence as NumberBox. It added
11,264 bytes to the thin release counter, compared with NumberBox's 18,432 bytes, and did not change
retained core layouts. This confirms a recurring per-control binary cost but does not justify
feature partitioning yet. The next difficult slice should proceed while preserving the same
measurement gate; a varied control batch remains necessary before projecting broad coverage.

NavigationView passes the generated multi-slot gate with typed `Content` and `Header` slots. The
schema resolves each object-valued setter from metadata and generates the public slot enum, slot
IDs, descriptors, binding filters, and WinUI setter dispatch. Pump sees only transparent
`NamedSlot` nodes and the generic `SetSlot` command. It has no NavigationView branch, slot ownership
table, or recovery state. Each slot accepts zero or one flattened native root. Recording tests
cover independent updates, replacement, clear, invalid arity, context propagation, and effect
cleanup; the live fixture updates both WinUI slots.

The slice adds 18,432 bytes (+2.08%) to the native thin release counter, mostly executable code,
and no measured compile or retained-layout regression. This is the third recurring generated-cost
point and the first that includes new shared structural machinery. Keep the cost gate open until
the varied batch includes enum-heavy properties and additional event payload shapes.

Normalized clear feedback now has a separate semantic from normalized setter feedback. A
successful `ClearValue` suppresses its synchronous echo but does not turn the known-native local
value from `None` into `Some(default)`. NumberBox and Slider live gates clear `Value`, drain native
events, rerender the same cleared control, and verify that no native command is emitted.

ProgressBar supplies the first unrelated ordinary-control cost point after the range and slot
slices. Seven metadata-derived floating and boolean properties required only schema rows,
generated output, and qualification code. The thin native counter added 11,776 bytes (+1.30%);
core layouts remained fixed. The 33 ms source-rebuild increase is small in absolute terms but
keeps compile cost in the watch set.

The live gate rejected an attempted Viewbox-as-content slice with `E_NOINTERFACE`. Viewbox owns a
typed `Child` property rather than implementing `IContentControl`; recording tests alone could not
expose that false role. The schema resolver now verifies `content` against `IContentControl` and
`children` against `IPanel` metadata. A future Viewbox slice must use a generated typed single-child
slot contract rather than pretending it is generic content.

ToggleSwitch supplies the next controlled shape: `IsOn` observes a typed boolean payload from
`Toggled` under the existing synchronous-exact contract. It required no production schema,
generator, Pump, or WinUI branch. The live fixture proves native true/false read-back and zero
escaped programmatic callbacks. Its 12,800-byte (+1.40%) thin-binary cost matches the recurring
ordinary generated-control slope, while source rebuild and core layouts remain stable.

The measured varied batch now spans normalized floating feedback, exact string and boolean
feedback, a seven-property ordinary control, and generated named slots. Excluding the one-time slot
infrastructure, recent controls add about 11-13 KB each. At the current slope, a curated set of
roughly 50 controls would add about 0.6 MB before specialized adapters. Do not add feature
partitioning now: the projected native thin binary remains well below the current reactor baseline,
and optional control groups would add generator, documentation, testing, and user-facing
complexity. Revisit the decision with actual broad generated coverage rather than another small
sample.

### Structural composition decision

Initial reactor-next composition is synchronous and atomic. It does not add a continuation state
machine for one large component render. Applications should put large collections behind
virtualization and keep ordinary component subtrees within one UI turn. The reconciler is qualified
through 4,096 keyed siblings, but that is a scale test rather than a recommended per-frame render.

Revisit this only with a measured live UI stall that virtualization or component boundaries cannot
solve. Any continuation must preserve one candidate publication boundary and may not expose
partially rendered logical or native state.

### Migration boundary

The initial migration model is direct:

| Current reactor | Reactor-next |
| --- | --- |
| Root render function | Root `Component` |
| `RenderCx` hook state | Component fields and typed messages |
| Hook effect | `ViewContext::use_effect(key, dependency, setup)` |
| Root `Element` | Opaque `View` built from generated controls |
| Callback state mutation | Callback sends a component message |
| `App::new().render(...)` | `App::run_component::<C>(props)` |

This is not yet a general replacement path. Applications using controls outside the generated
slice, mature templates, navigation, focus helpers, animation, third-party controls, or current
reactor resource facilities must wait for the matching difficult vertical slice and live proof.

## Current work

The fatal native failure simplification is implemented in the core:

- `NativeRuntime::apply` stops on the first error.
- `NativeApplyError` retains only command index and error for diagnostics.
- The WinUI host aborts on `NativeApplyFailed`.
- Update publication has one success path; initial mount and realization retain separate,
  documented invariants.
- Controlled observations reconcile without retry state.
- Fine-grained recovery code and tests are removed.
- The isolated component path improved from 476 bytes and 11 allocations to about 430 bytes and
  9 allocations without losing constant scaling through 16,384 scopes.
- Failed direct and component-turn planning keeps touched scopes dirty for an identical-props
  retry.
- Hook and component cleanup precede native mutation, and final `Drop` cleans components before
  native reset.
- Component effects use opaque explicit keys. Conditional omission, call reordering, duplicate
  rejection, dependency replacement, and retry retention use the existing publication engine.
- Component-owned keyed reconciliation uses one key index. Reorders with 256 or more operations
  use synchronization and remain near-linear through adversarial 10%, 20%, and 25% movement at
  4,096 children.
- `ItemsRepeater` stores keyed `View` rows and realizes them through the ordinary View planner.
  Each published row has one logical ownership root and one native attachment root. Component
  scopes stay lazy, key-stable source updates preserve row identity, and recycle or source reset
  uses the normal child-first component lifecycle.
- The token-keyed live host owns the application separately from its Pumps. Two real windows route
  window-specific event payloads independently; closing the secondary discards stale scheduled
  work while the primary continues through structural and component updates. In-flight tokens
  remain visible to close routing so synchronous closure cannot resurrect a Pump or trigger
  premature application exit. This proves ownership and lifecycle isolation, not survival after a
  process-fatal native failure.

The consolidation checkpoint is complete:

- A standalone thin counter compiled clean in 5.1 seconds, compared with 12.7-13.0 seconds for the
  equivalent current-reactor counter. A source-only rebuild took 0.55 seconds, compared with 3.0
  seconds. The release executables were 856 KB and 2.99 MB.
- These current-reactor ratios are favorable but provisional. The current crate carries broad
  generated control coverage while reactor-next still has a narrow generated slice. The same
  measurements must be repeated after broad control generation.
- Isolated component work remains about 0.51 us, 430 bytes, and 9 allocations through 16,384
  unrelated scopes.
- Initial mount, update, and realization retain distinct publication invariants while sharing the
  fatal native-apply boundary.
- The Pump, scheduler, and close lifecycle audit found no recovery-era state to remove. Normal
  scheduler closure after an in-flight dispatch is not reported as a fault.
- Components and `View` are now the sole public frontend. The 593-line hook frontend, its second
  effect engine, live host path, and benchmark path are removed.
- `View` is opaque. Unsupported virtual-item construction is not public, and an empty view is an
  empty fragment rather than a second planner case.
- Component scopes use direct function pointers rather than a boxed view closure or test-only
  dispatch enums. The transient `Retiring` state is removed; removal cancels work and advances the
  scope generation directly.
- Task cancellation and task status share one atomic state. Dropping a task handle does not
  cancel its work.
- Component turns scan pending descendants only at a parent-before-child composition boundary,
  rather than after every dispatched message.
- The exact context dependency index remains alongside the context-ID index. Removing it would
  turn one-provider updates into scans across every provider of the same context and violate the
  measured many-provider locality gate.
- Idle component storage is now about 2,440 bytes per scope. Isolated leaf updates remain about
  430 bytes and 9 allocations with flat time through 16,384 unrelated scopes.

Context propagation is complete:

- `Context<T>` provides a typed default, `View::provide` adds a logical provider, and
  `ViewContext::use_context` records the resolved provider identity.
- Provider changes use a published reverse dependency index and directly recompose the exact
  surviving consumers after child reconciliation. They do not scan the provider subtree or force
  unchanged component ancestor paths through composition.
- Context-key replacement is shadow-aware. Consumers resolved to nearer providers do not
  recompose.
- Dependency and reverse-index changes publish only after native success. Planning and injected
  native-apply failures retain the prior dependency set.
- Sparse provider values use two-level copy-on-write chunks rather than one global map.
- A broad provider with one consumer remains about 3.6-3.7 us from 512 to 16,384 descendants.
  Updating one provider among 16,384 providers remains within the 25% locality gate.

Scope-owned background tasks are implemented:

- `ComponentContext::spawn_background` runs one `Send` closure per owned OS thread and routes its
  typed result through normal component message dispatch.
- Each closure receives cooperative cancellation. Scope retirement, explicit cancellation, Pump
  shutdown, and window close prevent delivery even when work ignores cancellation.
- Each Pump permits 64 live task threads and 4,096 queued completions. Excess work is `Rejected`.
- Local and background queues remain separate and alternate during draining, preserving non-`Send`
  local messages without starvation.
- An explicit wake-pending bit coalesces dispatcher callbacks. Wake rejection rejects every
  completion covered by that callback instead of stranding later work.
- Task status is observable as `Running`, `Queued`, `Delivered`, `Cancelled`, or `Rejected`.
- The live two-window fixture delivers a primary background result through WinUI's dispatcher,
  closes the secondary with a task in flight, discards its late result, and keeps the primary
  operational.
- End-to-end thread creation, completion enqueue, and UI dispatch measure about 67 us and 825
  allocated bytes through 16,384 unrelated scopes.

The current phase is API consolidation and replacement qualification.

### Developer UX gate

Begin developer UX qualification before broad control generation. The goal is not to recreate the
current reactor's helpers or add macros speculatively. It is to settle public contracts that become
costly to change once generated coverage expands:

1. One normal composition path for native controls, components, fragments, providers, and slots.
2. Explicit rules for positional and keyed child identity.
3. Effect identity, which is currently positional even though component state is not hook-based.
4. Typed imperative references for focus and other native operations without a second owner.

Use a realistic form with controlled text and numeric input, validation, focus, and background
submission as the first qualification slice. Preserve the current implementation as a baseline,
then measure source size, explicit keys, forwarding closures, extraction into child components,
diagnostics, compile time, and edit/rebuild time after each API change.

Convenience constructors, component macros, broad styling helpers, resource abstractions, and
migration shims follow application evidence. They must remain syntax and services over the single
`View` and owned-component model, not introduce another reconciliation frontend or state engine.

The initial form baseline is `crates/samples/reactor-next/form`. Its first compile exposed that
generated controls converted to `Element` but not directly to `View`, forcing `View::native` at
every logical child boundary because Rust does not chain `Into` conversions. The generator emits
direct control-to-`View` conversions that call the existing `View::native`; this adds no tree type
or reconciliation path.

Passing store-owned props to `view` lets the read-only summary render without duplicating its props
or synchronizing them in `changed`. The core structural capability methods reduce the formatted
form from 176 to 143 source lines and from seven explicit child keys to zero. Typed event-message
adapters remove all three sender handles and forwarding closures. The form retains one empty
`update` in the read-only child component. The form now owns generated typed references and queues
focus for the first invalid field. `ElementRef<T>` binds only after successful candidate
publication and carries the exact window epoch and generational node identity into queued work.
Removal, replacement, shutdown, and window close unbind it and discard stale requests.

Focus-capable schema rows generate a sealed marker for `Button`, `TextBox`, `NumberBox`, `Slider`,
and `ToggleSwitch`. `request_focus` reports queue acceptance; WinUI `Focus(Programmatic)` returning
`false` completes normally. Raw native handles remain intentionally absent so callbacks cannot
bypass the Pump. Canvas, WebView, and similar subsystems require later specialized adapters with
their own ownership and documented-failure contracts. A reference has one published owner;
duplicate use within a tree or across windows fails candidate validation before native mutation.
Imperative work waits for the native-event and component-message backlog to publish, preserving the
causal ordering between a request and later removal or replacement messages already in the queue.

## Post-UX qualification plan

Complete these gates in order. Do not broaden generated control coverage until the architecture
audit and integrated sample show that the current lifecycle model remains understandable and
scales acceptably.

### 1. Architecture and implementation audit

- [x] Restate the ownership and publication invariants from the implementation.
- [x] Trace initial mount, full update, and the local component fast path.
- [x] Trace keyed movement, replacement, fragments, providers, content, children, and slots.
- [x] Trace virtual source update, lazy realization, recycle, and realization retry.
- [x] Prove virtual source revisions and shell-lifetime tokens reject delayed native callbacks.
- [x] Define optional virtual-row attachment and non-fatal invalid-root diagnostics.
- [x] Trace component reservation, publication, prop staging, retry, and retirement.
- [x] Trace keyed effect registration, preparation, setup, cleanup, and shutdown.
- [x] Trace reference attach, swap, detach, duplicate detection, and imperative queue ordering.
- [x] Check every planning error leaves native state, the published tree, effects, and references
      unchanged, apart from documented component prop staging that is carried into retry.
- [x] Check every native apply error poisons the Pump without publishing candidate state.
- [x] Check every retirement path cleans components child-first and invalidates stale work.
- [x] Check shutdown, window close, Pump restart, and multi-window identity independently.
- [x] Find lifecycle logic duplicated across properties, references, effects, components, and
      virtualization that could drift as new facilities are added.
- [x] Record high-confidence defects, scaling risks, and accepted tradeoffs in this document.
- [x] Close the audit only after focused regressions and the existing live host gate pass.

### 2. Performance checkpoint

- [x] Select equivalent incumbent and reactor-next applications.
- [x] Record clean compile time and representative incremental edit time.
- [x] Record release binary size and generated binding growth.
- [x] Measure mount and unchanged-update work.
- [x] Measure keyed insertion, removal, reorder, and replacement.
- [x] Measure component message throughput and local-update hit rate.
- [x] Measure virtual source update, realization, recycle, and row-component overhead.
- [x] Measure candidate-tree cloning and reference-validation cost at realistic and stress sizes.
- [x] Compare against the current reactor baseline and define acceptable regression bounds.

### 3. Integrated virtual task/editor sample

- [x] Use keyed row components with controlled editing and retained row state.
- [x] Exercise add, remove, front insertion, reorder, and replacement.
- [x] Exercise selection and focus transfer during editing and validation.
- [x] Exercise conditional rows, contexts, keyed effects, and background loading.
- [x] Exercise realization, recycle, key-stable payload updates, and source reset.
- [x] Add a deterministic stress path with hundreds or thousands of rows.
- [x] Use findings to adjust core contracts before adding convenience APIs.

### 4. Navigation and multi-window sample

- [x] Retain page state across navigation.
- [x] Qualify context propagation across page and window boundaries.
- [x] Qualify window creation, title configuration, close, task cancellation, and cleanup.
- [x] Prove queues, references, events, and background completions remain window-isolated.
- [x] Let sample evidence define navigation and window APIs.

### 5. Application performance gate

- [x] Measure a local edit in one realized virtual row.
- [x] Measure a broad parent update whose rows are mostly unchanged.
- [x] Measure a redundant component message, unchanged root-component memo hit, and forced
      value-equal root recomposition separately.
- [x] Measure sustained scrolling and realize/recycle traffic with controlled input, focus, effects,
      selection changes, and background completions active.
- [x] Separate Rust planning time from WinUI layout, rendering, and presentation time.
- [x] Record allocation volume plus median, p95, and p99 frame times rather than only best-case
      microbenchmark time.
- [x] Profile before changing architecture if Rust planning approaches 4 ms or sustained p95 frame
      time exceeds 16.7 ms on the checkpoint machine.
- [x] Preserve one-tree ownership and transactional publication in any optimization.

The shared virtual-editor recording driver covers the Rust half of this gate without duplicating
the sample model. On the August 21, 2026 checkpoint machine, 500 release-mode samples put the mixed
background/selection/32-row recycle-and-realize cycle at 866 us median, 1.02 ms p95, and 1.21 ms
p99. A local controlled edit was 213 us median and 325 us p99. Sharing task payloads and replacing
one single-consumer context provider per task with a direct row prop reduced that edit from 826 KB
and 5,422 allocations to 532 KB and 1,355 allocations. Durable write-through ownership remains in
the parent task model.

The live run then forced a new virtual index on every frame while alternating selection, editing
controlled rows, and delivering background completions. Its 300-frame p95 was 17.79 ms versus
17.16 ms for the idle editor; two active frames exceeded 33.4 ms, while the baseline had none.
Instrumented active host turns were 1.41 ms median and 1.78 ms p95. Native apply batches were
199 us median and 1.13 ms p95; the worst dropped frame coincided with an 18.47 ms native apply.
The raw p95 threshold therefore triggered phase measurement, but the 0.63 ms baseline-relative p95
increase and low steady-state host cost do not justify an architecture change.

Forced scrolling also exposed and fixed a live recycle-order bug. WinUI's element factory clears
and retires a shell synchronously before the queued Pump recycle runs, so a later
`DetachRealized` must accept that the shell is already absent. Attaching content and clearing a
still-live shell remain strict. The application-performance gate is complete; allocation reduction
and rare native realization outliers remain performance watches during control expansion.

### 6. Control-expansion gate

- [ ] Add a control only when a qualification sample requires it.
- [ ] Prioritize layout and application-shell gaps such as Grid and row/column definitions.
- [ ] Qualify each new capability through generation, planning, recording runtime, and live WinUI.
- [ ] Defer templates, dialogs, menus, and broad navigation until their ownership contract is
      explicit.
- [ ] Re-run compile-time, binary-size, and generated-surface measurements after each control
      tranche.

#### Grid tranche

- [x] Add schema-driven Grid generation with row and column spacing.
- [x] Represent row and column definitions as value properties rather than definition nodes.
- [x] Route attached row, column, and span values through the existing property transaction.
- [x] Keep attached placement on concrete native controls; require a native wrapper around
      components and fragments.
- [x] Qualify mount, update, clear, no-op, keyed reorder, failed publication, and live WinUI
      readback.
- [x] Convert the form sample to a two-column Grid.
- [x] Measure the tranche against the exact pre-Grid branch state.

The Grid tranche keeps `Node` and `MountedProps` at 432 and 72 bytes. `Element` grows from 88 to 96
bytes for the optional `Rc<GridPlacement>` carried by layout-capable incoming controls. This is
transient candidate data; published attached values use the existing native property map.

In isolated target directories, five source-only thin-counter rebuilds moved from a 0.791-second
median to 0.813 seconds (+2.8%, 22 ms). The release thin counter moved from 1,041,920 to 1,098,752
bytes (+56,832 bytes, +5.45%). Grid adds a WinUI control, two definition classes and collections,
GridLength conversion, and attached-property statics, so its binary cost is above an ordinary
single-control tranche. The resulting thin executable remains about 37% of the measured
`windows-reactor` counter. Keep the recurring binary slope in the control-expansion watch; this
tranche does not justify feature partitioning or another runtime path.

The iterator-based `rows` and `columns` builders allocate owned definition values on each render.
The post-expansion allocation pass should measure this in Grid-heavy forms and add a shared-value
builder only if those allocations are material; do not add a retained layout cache in Pump.

The follow-up review closed two property-boundary gaps. Pixel and star lengths now reject negative,
NaN, and infinite values in the generated builders, before invalid application input can reach the
fatal native boundary. Virtual collections now retain ordinary `NativeState` and use the shared
initial/update property planners before source-specific work. `ItemsRepeater` therefore implements
`LayoutControl` again and supports direct Grid placement, update, clear, and transactional failure
without a ScrollViewer wrapper or a virtual-only property branch.

#### SplitView tranche

- [x] Add schema-driven SplitView properties and `Pane`/`Content` slots.
- [x] Classify `UIElement`-typed slot setters without adding a SplitView planner path.
- [x] Apply pane lengths and display mode before opening the pane.
- [x] Qualify property dispatch, slot mount and clear, native readback, and generated ordering.
- [x] Convert the navigation sample to a SplitView application shell.
- [x] Measure the tranche against the completed Grid state.

SplitView reuses ordinary properties and named slots. The schema now distinguishes
`IInspectable`-typed and `UIElement`-typed slot setters, while Pump retains one `SetSlot` command
and one named-slot lifecycle. This avoids the incumbent defect where `display_mode()` existed in
the public builder but never reached WinUI.

The tranche keeps `Node`, `MountedProps`, and `Element` at 432, 72, and 96 bytes. Five isolated
source-only thin-counter rebuilds moved from the Grid checkpoint's 0.813-second median to 0.873
seconds (+7.4%, 60 ms). The release thin counter moved from 1,098,752 to 1,111,040 bytes (+12,288
bytes, +1.12%). This matches the recurring ordinary generated-control binary slope and adds no
retained engine state.

#### Post-expansion review follow-up

- [x] Separate the unchanged root-component memo hit from forced value-equal root recomposition.
- [x] Centralize candidate abort and fail-stop policy by publication stage.
- [x] Test native shell recycle immediately before a keyed source reset and under repeated resets.
- [x] Measure virtual source declaration and update costs at 1,000, 10,000, and 100,000 rows.
- [x] A/B shared task payloads and direct selection props before designing a lazy virtual source.
- [x] Run longer active and idle live measurements, emphasizing missed frames and native work.

The unchanged root-component memo hit is 0.2 microseconds and two allocations, but it does not
reconcile the application tree. Forced value-equal `TaskEditor` recomposition is 110 microseconds
and 1,202 allocations at 1,000 tasks. It scales to 1.72 ms at 10,000 and 21.8 ms at 100,000. The
focused value-equal 512-leaf static-tree case remains about 26 times slower than the incumbent. The
current 1,000-item application does not justify another source model. A real 100,000-item
application would justify designing a lazy indexed source that preserves one authoritative tree.

Two repeated 600-frame active runs had 17.91-18.10 ms p95, two frames over 33.4 ms, roughly 1.5 ms
host-dispatch p95, and roughly 1.05 ms native-apply p95. The 600-frame idle run had 17.16 ms p95 and
no frames over 25 ms. One earlier active run under system contention reached 33.33 ms p95 while
host and native-apply p95 rose to 5.08 and 3.31 ms. This confirms that missed-frame counts and
correlated host/native phases are more useful than one headline frame percentile.

### 7. Performance-optimization gate

Run this gate after representative layout and application-shell controls exist, and before deciding
that `windows-reactor-next` can replace `windows-reactor`. Rust's performance and memory advantages
must be visible in realistic applications, not only in isolated component operations.

- [x] Build matched application workloads for `windows-reactor`, `windows-reactor-next`, and the C#
      Reactor where their semantics overlap.
- [x] Measure wall time, median/p95/p99 frame intervals, allocator traffic, retained Rust memory,
      process working set, compile time, and release binary size on the same machine.
- [x] Reduce the 1,000-item controlled-edit allocation volume from 826 KB and 5,422 allocations.
      Shared immutable task payloads and direct row selection props reach 532 KB and 1,355
      allocations without weakening durable edit ownership.
- [x] Investigate repeated key/view construction, unchanged parent reconciliation, candidate-tree
      copy-on-write granularity, and virtual-row command construction with profiles and allocation
      traces.
- [x] Investigate rare native realization outliers separately from Rust planning so native layout
      work does not drive frontend caching or ownership changes.
- [x] Define replacement targets from the matched workloads. At minimum, next must retain its
      compile-time, binary-size, and retained-memory advantages and avoid a material sustained
      runtime regression.
- [x] Reject optimizations that add a second mutable UI tree, make rollback stateful, bypass
      transactional publication, or leave cache invalidation implicit.
- [x] Re-run the full correctness, live, compile-time, binary-size, and application-performance
      gates after optimization.

#### Matched 32-task application checkpoint

The first matched workload uses only controls shared by all three frontends: SplitView, Grid,
StackPanel, TextBox, TextBlock, and ToggleSwitch. It mounts an inline application shell with 32
keyed task rows, then exercises a local title edit, selection change, broad done toggle, and keyed
reversal. The Rust driver also measures a value-equal declaration rebuild. Initial mount is outside
the timed loop.

The two Rust variants run against their in-memory recording backends. Both backends retain recorded
native operations, so the measurement includes declaration, reconciliation, publication, and
recording work but excludes WinUI control mutation, layout, rendering, and presentation. Five
hundred release samples after 16 warmups produced:

| Operation | Inc. median | Next median | Inc. bytes | Next bytes | Inc. allocs | Next allocs |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Local edit | 25.5 us | 53.5 us | 111,064 | 54,805 | 201 | 681 |
| Selection | 19.0 us | 47.0 us | 113,148 | 57,202 | 216 | 723 |
| Broad toggle | 35.8 us | 100.3 us | 163,557 | 106,098 | 445 | 1,580 |
| Reverse keys | 20.2 us | 51.9 us | 123,273 | 63,860 | 198 | 695 |
| Value equal | 15.1 us | 26.6 us | 108,223 | 41,103 | 177 | 414 |

Next is 1.8-2.8x slower in this frontend-only workload, but allocates 35-62% fewer bytes per
operation. Its remaining weakness is allocation count: it makes about 2.3-3.6x as many calls.
Retained allocator deltas after mount are 304,733 bytes for the incumbent and 213,331 bytes for
next, a 30% reduction.

The phase split identified unchanged native-backed child planning as the first target. Before the
change, next publication made 1,129 allocations even for the value-equal case because each
`Children` control rebuilt key vectors, sets, maps, and order vectors. A read-only mounted-view
matcher now skips an exact native-backed subtree before allocating those collections. It does not
skip components or virtual collections, and it is disabled while a native observation is pending.
A nested-repeater regression proves that an equal virtual source still recomposes a dirty realized
component.

In a controlled run with command recording disabled before and after the optimization:

| Operation | Before | After | Allocations before | Allocations after |
| --- | ---: | ---: | ---: | ---: |
| Local edit | 90.4 us | 59.7 us | 1,557 | 854 |
| Selection | 85.9 us | 49.6 us | 1,570 | 801 |
| Broad toggle | 97.3 us | 103.4 us | 1,683 | 1,698 |
| Reverse keys | 97.8 us | 52.7 us | 1,598 | 778 |
| Value equal | 88.4 us | 26.6 us | 1,543 | 486 |

The broad case remains flat because every row changes. The fast path preserves one-tree ownership,
performs no native mutation, and adds no retained cache or invalidation protocol.

The remaining equal rebuild originally spent about 14 us and 414 allocations declaring the view,
then about 13-15 us and 72 allocations publishing it. Those 72 calls aligned with string-valued
properties: the generated property visitor materialized owned `PropertyValue::Str` values while
checking desired values against authoritative known-native values.

The generated visitor now yields `PropertyValueRef` values. Strings and Grid definitions remain
borrowed during inspection; scalar values stay copied. Comparison still reads
`NativeState.properties`, so silent native normalization and coercion are repaired exactly as
before. A changed value becomes owned only when it enters a native command and property commit.
There is no retained cache or invalidation protocol.

Across three 500-sample repetitions, equal publication fell from 72 allocations and about 13-15 us
to zero allocations and 10.5-11.6 us. Total allocation calls fell 20% for local edits, 10% for
selection, 7% for broad toggles, and 11% for keyed reversal. The mixed live workload fell from
941.0 to 828.2 Rust allocations per update and from 66,334 to 65,542 bytes per update. Retained
post-mount bytes stayed at 213,331.

Feature-isolated build and binary measurements remain favorable:

| Frontend | Clean check | Source-only check median | Release executable |
| --- | ---: | ---: | ---: |
| `windows-reactor` | 6.17 s | 0.575 s | 2,196,992 bytes |
| `windows-reactor-next` | 3.70 s | 0.453 s | 872,960 bytes |

The Rust live protocol now runs the two frontends in separate feature-isolated executables against
real WinUI. Both maximize the active window, wait 16 warmup updates, verify the client dimensions,
and apply one operation per composition frame. Returning to the dispatcher between updates prevents
the unbounded queued-layout drift seen in the first C# run. Three paired 500-update release
repetitions used the same `3840x2054` client area throughout:

| Metric | Incumbent range | Next range |
| --- | ---: | ---: |
| Frame median | 17.17-17.36 ms | 17.35-17.60 ms |
| Frame p95 | 27.12-27.80 ms | 27.57-27.87 ms |
| Frames over 25 ms | 43-49 / 500 | 48-50 / 500 |
| Frames over 33.4 ms | 0-5 / 500 | 3-4 / 500 |
| Rust bytes/update | 119,732 | 65,542 |
| Rust allocations/update | 250.6 | 828.2 |
| Ending working set | 115.2-116.3 MB | 115.2-116.1 MB |
| Ending private bytes | 100.7-102.1 MB | 99.8-101.6 MB |

An additional corrected run after removing warmup-buffer extraction from allocator accounting
reported the same allocation rates before the borrowed visitor change. With that change, next uses
45.3% fewer Rust allocation bytes but makes 3.30x more allocation calls. Frame cadence,
missed-frame counts, and ending process memory are close
enough that there is no evidence of a material live runtime or memory regression. The incumbent's
cheap-update tree-build plus reconcile median is about 72-80 us; next's host-dispatch median is
about 143-152 us. Broad native mutation dominates both p95 paths at about 8.7-8.9 ms.

The live reports do not claim a Rust retained-heap delta: next's test instrumentation owns timing
vectors that the incumbent host does not, so such a value would measure the harness. Process
working/private bytes and the recording benchmark's post-mount retained-tree measurement remain the
valid memory gates.

The C# M15 benchmark now uses the same five-operation sequence, 16 warmup updates, one update per
composition frame, eight settle frames, maximized `3840x2054` client area, and three 500-update
repetitions. `CompositionTarget.Rendering` supplies cadence and queues mutation at normal dispatcher
priority, matching next's component scheduler. The executable has the WinUI control resources and
Per-Monitor-V2 manifest required to render the exact TextBox and SplitView topology.

| Metric | C# Direct range | C# ReactorToday range | C# Reactor range |
| --- | ---: | ---: | ---: |
| Frame median | 22.12-22.66 ms | 24.60-25.25 ms | 24.80-25.27 ms |
| Frame p95 | 35.47-35.92 ms | 38.04-38.73 ms | 38.17-38.97 ms |
| Frames over 25 ms | 177-187 / 500 | 243-252 / 500 | 244-254 / 500 |
| Frames over 33.4 ms | 47-50 / 500 | 60-65 / 500 | 58-61 / 500 |
| Managed bytes/update | 2.5-3.5 KB | 112.4-113.7 KB | 112.6 KB |
| Ending working set | 166.4-178.7 MB | 187.2-189.5 MB | 189.9-192.2 MB |
| Ending private bytes | 119.4-131.7 MB | 133.2-136.2 MB | 135.5-139.8 MB |

C# Reactor and ReactorToday are indistinguishable at this scale. Next's frame median is about
7-8 ms lower, its frame p95 is about 10-11 ms lower, it has about one fifth as many frames over
25 ms, and it ends roughly 74-76 MB lower in working set and 34-40 MB lower in private bytes. Next
also allocates about 65.5 KB of Rust heap per update versus C# Reactor's 112.6 KB of managed heap.
Managed and Rust allocation counters describe different runtimes, so that last comparison is
directional rather than a direct retained-memory ratio. The imperative C# path remains an
implementation lower bound, not a competing declarative architecture.

This checkpoint closes workload construction, the first profile-led optimizations, and the matched
replacement-readiness performance gate. Current evidence supports continuing: next retains its
memory, compile-time, and binary-size advantages and has better live cadence than both the incumbent
Rust frontend and C# Reactor in this workload. Its remaining cheap-update CPU and allocation-call
gaps are localized and do not require an architectural rewrite. Performance remains a planned
remeasurement and optimization phase as control coverage and API design grow.

## API polish record

### Generated declaration surface

The first public-surface audit removed generated `*_property()` and `*_callback()` getters. No
application or sample used them; one integration test inspected declaration storage through them.
Property values and callback wiring are already covered at the generated visitor, Pump, recording
runtime, and live WinUI boundaries. Keeping public getters only for tests exposed `Property<T>` and
control storage details without supporting a declarative application task.

`Property<T>` and its inspection helper are now crate-private. Generated controls expose builders,
structural capabilities, references, and typed events, but not their backing representation. This
removed 117 generated public methods without changing declaration layout, planning, or runtime
behavior. Generator tests reject reintroducing property, Grid-definition, and callback getters.

Conditional property reset now uses the generated setter surface without exposing `Property<T>`.
Scalar and enum setters accept either `T` or `Option<T>`. String properties and Grid definitions
retain their inference-friendly ordinary setter and add `*_optional` methods. `Some` follows the
existing set path; `None` produces the same inherited declaration as omission. A published set
therefore transitions through the existing `ClearProperty` command and WinUI `ClearValue`, while a
repeated `None` emits no native work.

Recording tests prove set-to-`None` and repeated-`None` behavior for TextBox, NumberBox, Slider,
and ToggleSwitch. The existing controlled-feedback boundary suppresses native echoes during clear
and commits the authoritative cleared state. Grid optional definitions validate only present
values. This adds no planner command, public wrapper, retained state, or alternate control path.

## Architecture audit record

### Authoritative ownership

| State | Owner | Candidate behavior |
| --- | --- | --- |
| Logical topology and identity | `Tree` | Candidate is published or discarded as a unit |
| Desired and observed properties | Native `Tree` nodes | Commit after native apply |
| Components, tasks, and effects | `ComponentStore` | Tracks reservations and staged updates |
| Context providers and dependencies | `Tree`, `ComponentStore` | Publish with component scopes |
| Virtual leases and rows | Virtual `Tree` nodes | Retain logical and optional native roots |
| WinUI objects and subscriptions | `NativeRuntime` | Change through ordered commands |
| Imperative reference binding | Shared `ElementRef` target | Changes after apply |

There is still one logical tree, one component store, and one native runtime. Virtual rows,
references, effects, and generated controls do not add another frontend or owner.

### Shared publication sequence

Initial mount, full updates, local leaf updates, and realization now converge on
`Pump::publish_candidate`:

1. Validate reference ownership without changing a shared target.
2. Prepare changed and retired effects. Cleanup is child-first.
3. Apply the ordered native command batch.
4. Commit observed properties and desired reference declarations to the candidate.
5. Publish reserved scopes, context dependencies, retirements, and the candidate tree.
6. Apply reference unbind/bind commits against the published generational identity.
7. Set up new and changed effects parent-first.
8. Publish the Pump version and clear native-observation repair state.

Unexpected native apply failure poisons the Pump and does not publish the candidate. Effect cleanup
may already have run; this is part of the fatal-failure policy, not a recoverable transaction.
An effect-preparation error also poisons because earlier cleanup closures may have run. Post-apply
component-store failures poison because native state has already changed.

`CandidateFailureStage` makes cleanup policy explicit at each publication exit. Planning discard
removes reservations. Planning retry also retains every touched scope in `planning_dirty`.
Effect-preparation, native-apply, and publication failures remove remaining reservations and
fail-stop. `Pump::fail_stop` poisons the Pump and clears queued events and realizations. Any
nonfatal realization failure restores the consumed native request.

A live active-scroll soak exposed a separate shell-lifetime leak: 79 retired tokens remained after
120 frames because WinUI can recycle a shell whose queued realization never publishes. Rejected
recycle work now emits `AcknowledgeRecycle`. The WinUI adapter removes only a retired token, rejects
a still-live token, and accepts a token already consumed by a reset detach. The same 120-frame soak
settled with no retired tokens after the fix.

### Findings fixed during the audit

- Initial mount duplicated publication and could drift from update behavior. Both element and
  component mount now use `publish_candidate`.
- Duplicate-reference validation could fail after component reservation without removing reserved
  scopes. Pre-apply failures now remove reservations centrally.
- The same validation path could leave staged component props outside `planning_dirty`. Pre-apply
  failures now retain every touched scope for forced recomposition on retry.
- A realized row that failed nonfatally consumed its native realization request. The request is
  restored until planning and validation can succeed.
- A rejected recycle for an unpublished or stale realization left WinUI's retired shell token
  unconsumed. Rejected recycle work now acknowledges that token without weakening ordinary detach
  validation.
- Reusable WinUI shells shared a container identity across physical lifetimes, and realization
  requests carried no source generation. Each checkout now gets a fresh lifetime token, recycle
  retires its live mapping before pooling the physical control, and attachment resolves only live
  tokens. Key-changing source resets publish a checked `u64` revision that callbacks capture.
  Planning rejects mismatched revisions and realizations superseded by a queued recycle.
- Realization found an indexed row by scanning keyed views for the lease key. It now reads the
  validated index directly and verifies that the indexed key matches the lease before composition.
- Empty or multi-root virtual rows raised `StructureUnsupported`, so a component message could
  terminate the window. A realized row now retains its logical subtree with an optional shell
  attachment. Zero roots leave the shell empty. Multiple roots leave it empty and commit one
  `VirtualRowRootCount` diagnostic. Returning to one root reattaches without recreating row
  components or effects. Diagnostics publish only with the candidate, and the live host warns
  without shutdown. Deterministic invalid shape does not enter a retry queue.
- Imperative work had an unbounded queue and unbounded drain. The queue now accepts at most 4,096
  requests and applies at most 64 per host turn in one native batch.
- Reference uniqueness walked the complete tree on publications with no reference change.
  Validation now skips the steady-state path unless a candidate introduces a binding and uses a
  hash set of stable reference-cell identities when a scan is required.
- A full local-message queue turned ordinary native input backpressure into a fatal host fault.
  Rejected adapted callbacks now retain the event and retry after component work frees capacity.
- Same-key virtual payload reconciliation scanned the source and realized-container ownership once
  per realized row. It now builds temporary key and logical-owner maps once per update, while
  direct realization uses its validated index.

Focused regressions cover reservation cleanup, staged-prop retry, realization retry, source-reset
staleness, recycle supersession, deep-index identity, revision exhaustion, optional row attachment,
message and source shape transitions, detached retirement, transactional diagnostics, bounded
imperative work, cross-window reference ownership, and the shared publication paths.

### Accepted tradeoffs and open audit risks

- Component messages and changed props mutate the authoritative component before candidate proof.
  `planning_dirty` guarantees recomposition after a planning or pre-apply failure, but component
  state is not rolled back. `Component::changed` can also send messages or start background work.
  This follows from the decision not to clone component instances and needs an explicit application
  contract or a narrower `changed` context.
- Full updates clone the `Tree`. This keeps candidate ownership simple. Broad reconciliation is
  slower and allocates more than the incumbent, but the measured absolute cost remains within the
  performance bounds below.
- Effect key lookup is linear within each component. The intended case has few effects; measure
  before adding a map and another allocation.
- Reference validation scans the candidate tree only when a new binding is introduced and uses a
  hash set for duplicate detection. At 512 referenced controls, validation adds 39% mount time and
  5% transient bytes over the same controls without references.
- Typed references increased `Element` from 80 to 88 bytes and `Node` from 416 to 424 bytes.
  Virtual source revisions then increased `Node` to 432 bytes. The retained per-node cost is
  accepted because reference ownership and stale source rejection belong to authoritative tree
  state; the performance gate must measure its aggregate cost.
- Realization must know whether a publication error happened before native apply so it can retain
  the native request. The current retryable pre-apply error is duplicate reference ownership.
  Future pre-apply validators need to join the same classification rather than add another
  one-off restoration path.
- Content and named slots remain strict zero-or-one-native-root positions. A component that places
  multiple roots there receives `StructureUnsupported`; unlike virtual rows, these APIs declare a
  single structural value rather than a dynamic row template. Revisit this only if application
  evidence shows that preserving a detached logical subtree is useful outside virtualization.

### Audit conclusion

The architecture audit is closed. Initial mount, full reconciliation, local component updates, and
virtual realization share the same candidate publication spine. Native failure remains fatal;
planning and validation failure do not publish; effects, references, component scopes, virtual
leases, and diagnostics commit in defined order. Native callbacks are guarded by window, node,
event, source, and shell-lifetime generations. Work queues have explicit capacities or turn
budgets, and ordinary local-message backpressure defers rather than faults.

The remaining risks are measured-design questions rather than known correctness defects: full
Tree cloning, retained node size, effect lookup at unusually high counts, reference-heavy initial
publication, and the cost of detached virtual rows. These move to the performance checkpoint.

## Performance checkpoint

The August 21, 2026 checkpoint is recorded in
`crates/tests/libs/reactor-next-bench/readme.md`. The same machine and release toolchain measured:

| Measure | Incumbent | Reactor next |
| --- | ---: | ---: |
| Clean library check | 5.134 s | 2.712 s |
| Source-only library check | 3.601 s | 1.502 s |
| Thin counter executable | 2,975,744 bytes | 991,232 bytes |
| Local component message | 593 ns, 457 bytes | 700 ns, 430 bytes |
| Retained component scope | 3,628 bytes | 2,552 bytes |
| Change all 512 leaves | 164 us | 410 us |
| Reverse 512 keyed leaves | 105 us | 278 us |
| Rotate 512 keyed leaves | 70 us | 271 us |

The primary path passes: a local message is 1.18x incumbent time, uses fewer bytes, and remains
flat from 512 through 16,384 unrelated scopes. Compile time is 0.42-0.53x, the thin executable is
0.33x, and retained component memory is 0.70x.

Broad reconciliation remains the main watch. It is 2.5-3.9x incumbent time with higher transient
allocation, but stays below 1 ms at 512 rows and below 5 ms at 4,096 rows. A 10,000-item virtual
source update stays below 1.5 ms, and realizing plus recycling 32 rows takes 63 us. The checkpoint
accepts these absolute costs for the next integrated sample rather than weakening candidate
publication or adding a second mutable tree.

Until application evidence replaces them, the gates are:

- local component messages <= 1.5x incumbent time and flat with unrelated scope count;
- compile time, retained component memory, and thin binary size <= the incumbent;
- broad reconciliation < 1 ms at 512 rows and < 8 ms at 4,096 rows;
- 10,000-item virtual source updates < 2 ms and 32-row realize/recycle < 100 us;
- reference-heavy mount overhead < 50% time and < 10% bytes over identical controls.

The largest relative gap is a no-change update of a 512-leaf tree: about 7.5 us in the incumbent
and 197 us in next, or 26.3x. The incumbent skips the shared root in O(1), while next clones and
traverses a candidate tree. This remains below the 1 ms absolute bound and does not justify a
second mutable tree, but it must remain visible in performance reports.

The integrated virtual editor is the next gate. If it exceeds these bounds, profile repeated
key/view collection and copy-on-write chunk mutation before changing ownership or publication.

## Integrated virtual editor

`crates/samples/reactor-next/virtual` now contains a task editor rather than a static 10,000-label
list. It uses keyed row components, controlled text and toggle values, selection context, keyed
effects, typed focus, conditional row content, background loading, and dynamic source operations.
Buttons cover front insertion, removal, move-to-end, reversal, 100-row background loading, and a
deterministic 1,000-row reset.

The sample established the durable-state boundary. A realized row component survives key-stable
payload updates, but native recycle or a key-changing source reset may retire it. Every non-empty
draft therefore enters the parent task model on its controlled input event. A blank draft remains
row-local validation state and falls back to the last valid model value if the row is recycled. The
recording-runtime test enters edit mode, checks the focus effect, edits a row, recycles and
re-realizes it, reverses the source, and re-realizes the same task key. It verifies that the edited
title survives and each retired row effect cleans once.

Reset advances a load generation and clears the loading state. A late background result from an
older generation is ignored, so `Reset to 1,000` cannot later become 1,100 tasks.

This does not require another state store or a special virtual component. Durable application data
belongs to the keyed source model; row-local state is realization-local UI state. The distinction
matches native virtualization lifetime and is now part of the sample documentation.

The existing control set was sufficient, so this gate added no generated controls. The sample is
larger than the form because it spells out parent actions, row messages, and durable model updates.
That is useful evidence for the deferred API-polish work, but convenience APIs should wait until
the navigation sample shows whether the same action-forwarding pattern recurs outside virtual rows.

## Navigation and multi-window qualification

`crates/samples/reactor-next/navigation` starts a primary workspace and opens the secondary through
`ComponentContext::open_window`. Each Pump owns its current page, controlled editor text, typed
editor reference, component messages, and background task. Page components may retire when
navigation replaces them; durable page data stays in the owning workspace model and is restored
through controlled props when the page returns.

The windows share an application coordinator, not a Pump or component store. The coordinator owns
the common theme value and a sender registry populated by one lifecycle effect per window. A shared
change sends an ordinary message to each Pump, and each Pump publishes its own context provider.
Context identity may be shared, but context updates do not cross Pump boundaries by themselves.
This keeps scheduling and publication local while making cross-window fan-out explicit.

The recording qualification drives a real TextBox event in the primary editor, navigates away and
back, and checks that only the primary model changed. It then broadcasts a theme change, starts
background work in the secondary, shuts down that Pump, and verifies cancellation, one-time effect
cleanup, stale-sender rejection, peer notification, and continued primary work.

The first slice exposes a host API boundary:

| Operation | Current contract |
| --- | --- |
| Create multiple windows | Startup roots or committed runtime-open requests |
| Configure content | Each startup root is an independent `View` |
| Configure native window | `ViewContext` declares the title; size and presenter are deferred |
| Create a window after startup | `ComponentContext::open_window` stages an independent root |
| Request close from a component | Token-bound `WindowRef` request commits after publication |
| React to close | Pump retirement cleans effects, tasks, references, queues, and sender registry |

Do not expose the private WinUI `Window` to fill these gaps. The next design step must decide
whether dynamic windows are controlled application data or host resources opened through a queued
capability. It must preserve one Pump per window, route every request through the UI scheduler, and
make close ownership explicit before adding title or size options.

### Window API direction

A top-level application component returning keyed windows is not the next step. Native close is an
external lifetime event: if the key remains in a controlled window list, reconciliation would
reopen the window. A candidate spanning several Pumps would also have no coherent failure or
publication boundary. One candidate publication must remain scoped to one Pump and one native
window.

Window lifetime should instead be an app-owned host resource:

1. A cloneable, token-bound `WindowRef` now exposes `request_close()`. It uses request-shaped naming
   because close cancellation is a separate future contract.
2. Close is accepted only during `create`, `changed`, or `update`. The lifecycle endpoint stages at
   most one close with the candidate, and publication applies it in a separate native batch after
   frontend state and effects commit. The first committed close latches the endpoint, so later
   turns cannot issue another native close while `Closed` is pending. Planning failure discards the
   request. An inactive or stale reference returns false without touching another window.
3. `ComponentContext::open_window` is a committed host request carrying an independent window root.
   The host registers pending-open and in-flight state before dispatcher work, then mounts a new
   Pump after the opener publishes. The opener's component scope does not own the new window.
   Cross-window data continues to use an application-owned sender registry rather than a typed
   message channel on the window handle.
4. `LiveHost` now pre-registers startup Pumps as in-flight and counts pending opens before mount.
   This prevents a `create`-time close from being lost and prevents exit while another startup
   window remains. Runtime open joins the same accounting before it enters the dispatcher.

Configuration and lifetime use different rules. `ViewContext::window_title` is declarative Pump
state with one live component owner. Candidate planning stores declarations by component scope and
validates the completed set, so surviving siblings can transfer ownership in either traversal
order. A changed value is part of candidate planning, omission or owner retirement clears it, and
duplicate ownership rejects the candidate before native mutation. The local native fast path is
used only when the declaration matches published state. This keeps component-derived titles in the
authoritative tree without exposing an imperative native handle.

Size, position, and presenter state can change outside the framework and need create-time semantics
or controlled native feedback. A static startup descriptor is useful for create-time options, but
it cannot by itself express component-derived state. Do not add size or presenter setters until
their observation contract is defined.

Host requests issued by `create`, `changed`, or `update` must commit with the candidate. Effect
setup runs after publication and is not currently a window-request context. A planning failure must
not open or close a window for an unpublished component turn. Native apply failure remains
process-fatal; a planning failure while preparing a new independent root must reject that open
without shutting down existing windows.

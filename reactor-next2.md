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

## Component model

The owned-component frontend remains the primary direction:

```rust
trait Component: 'static {
    type Props: Clone + PartialEq + 'static;
    type Message: 'static;

    fn create(props: &Self::Props, context: &mut ComponentContext<Self>) -> Self;
    fn changed(&mut self, props: &Self::Props, context: &mut ComponentContext<Self>);
    fn update(&mut self, message: Self::Message, context: &mut ComponentContext<Self>);
    fn view(&self, context: &mut ViewContext<Self>) -> View;
}
```

Rules:

- Component identity is parent boundary plus key plus component type.
- Same key and type retain the scope across props and movement.
- Same key and different type replace the scope.
- Sends enqueue typed messages and never call component code reentrantly.
- Props apply parent-first before surviving descendant messages.
- Retirement drops queued work for removed descendants.
- One turn composes each dirty scope at most once.
- Local leaf updates do not clone the full tree.
- Typed context reads resolve to a specific logical provider and publish with the candidate.

Components and `View` are the only public frontend. The hook frontend was retained for early
comparison measurements, then removed before the API freeze so the core has one state and effect
model.

## Logical anchoring

`Fragment` is a logical node and creates no hidden WinUI control. It may represent zero, one, or
many native roots.

- Generated children collections accept many flattened roots.
- Window and content slots accept zero or one flattened root.
- Invalid arity fails planning before native mutation.
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

An unexpected failure in step 7 aborts the production process. Effect setup and candidate
publication do not run.

Normal shutdown cleans effects before dropping native resources. Cleanup remains idempotent across
explicit shutdown and `Drop`.

Hook effects follow the same boundary: changed cleanup runs after candidate validation and before
native apply, while setup runs after publication. If component props are applied before later
planning fails, touched scopes remain planning-dirty so an identical-props retry recomposes rather
than accepting stale structure.

## Scheduling

- Native callbacks enqueue work and return.
- Each dispatcher turn handles at most 64 events, 64 component messages, and 32 realizations.
- Remaining work rearms the scheduler.
- Component messages are capped at 4,096 per window and expose backpressure.
- Scheduler rejection is an explicit host fault.
- Work queued during dispatch is rearmed after the current turn.

The budgets count queued items. They do not preempt one component `view` call or split one candidate
plan. Adding a general composition continuation would restore much of the state-machine complexity
that this design removed. Large trees are instead governed by locality and keyed-scale gates.

Dispatcher rearming schedules remaining bounded work after the current callback. A failed enqueue
is surfaced as a host fault; neither behavior retries native mutation.

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
| Multiple slots | NavigationView or TabView uses generated roles rather than control branches |
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
branch.
Do not extrapolate broad coverage from the favorable compile result alone. Measure a
representative control batch and decide whether control feature partitioning is needed before
adding dozens of controls. The generated multi-slot gate follows that decision.

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
| Hook effect | `ViewContext::use_effect` |
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
- Component-owned keyed reconciliation uses one key index. Reorders with 256 or more operations
  use synchronization and remain near-linear through adversarial 10%, 20%, and 25% movement at
  4,096 children.
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

The next phase is API consolidation and replacement qualification.

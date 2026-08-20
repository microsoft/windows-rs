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

Hooks remain available as a comparison frontend. They use the same planner, runtime, and fatal
native failure policy.

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

The initial generated slice accepts synchronous exact feedback. Unsupported feedback contracts
fail generation rather than guessing.

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
| Component clean compile ratio | 0.99x hook |
| Component source-only rebuild ratio | 1.01x hook |
| Component release executable ratio | 0.91x hook |
| Isolated component leaf at 512 scopes | 0.51 us, 430 bytes, 9 allocations |
| Isolated component leaf at 16,384 scopes | 0.51 us, 430 bytes, 9 allocations |
| Idle component memory | About 2,496 bytes per scope |
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

- Clean compile time is 0.99x the equivalent hook application; source-only rebuild time is 1.01x.
- The component release executable is 0.91x the hook executable.
- Isolated component work remains about 0.51 us, 430 bytes, and 9 allocations through 16,384
  unrelated scopes.
- Initial mount, update, and realization retain distinct publication invariants while sharing the
  fatal native-apply boundary.
- The Pump, scheduler, and close lifecycle audit found no recovery-era state to remove. Normal
  scheduler closure after an in-flight dispatch is not reported as a fault.

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
- End-to-end thread creation, completion enqueue, and UI dispatch measure about 66-69 us and 817
  allocated bytes through 16,384 unrelated scopes.

The next phase is API consolidation and replacement qualification.

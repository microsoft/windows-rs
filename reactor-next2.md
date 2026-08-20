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

## Scheduling

- Native callbacks enqueue work and return.
- Each dispatcher turn handles at most 64 events, 64 component messages, and 32 realizations.
- Remaining work rearms the scheduler.
- Component messages are capped at 4,096 per window and expose backpressure.
- Scheduler rejection is an explicit host fault.
- Work queued during dispatch is rearmed after the current turn.

Scheduler retry means retrying dispatcher enqueue after rejection. It is unrelated to native
mutation recovery.

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
| Component clean/incremental compile ratio | 0.78x-0.91x hook |
| Component release executable ratio | 0.97x hook |
| Isolated component leaf at 512 scopes | 0.5 us, 430 bytes, 9 allocations |
| Isolated component leaf at 16,384 scopes | 0.5 us, 430 bytes, 9 allocations |
| Idle component memory | About 2,455 bytes per scope |
| Dense keyed reversal, 512 -> 4,096 | 0.19 ms -> 2.30 ms |

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
- Candidate publication has one success path.
- Controlled observations reconcile without retry state.
- Fine-grained recovery code and tests are removed.
- The isolated component path improved from 476 bytes and 11 allocations to about 430 bytes and
  9 allocations without losing constant scaling through 16,384 scopes.

Before feature expansion:

1. Re-run compile-time, runtime, allocation, and binary-size measurements.
2. Run the process-isolated live WinUI fixture.
3. Add live two-window ownership and fault-isolation coverage.
4. Review remaining Pump and scheduler complexity without adding new recovery behavior.

Do not add context, background async ownership, or more control coverage until these gates pass.

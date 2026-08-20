# Windows Reactor Next - Core Decision Plan

## Objective

Decide whether a Rust-owned component frontend is a better long-term model than the current hook
frontend over the same corrected WinUI backend.

This is a bounded A/B prototype. It is not a control-expansion phase and not a survey of additional
UI architectures.

Retain:

- Generated control schema and typed public API.
- Generational identity.
- Cloneable candidate structural tree.
- Ordered mutation batches and per-command receipts.
- Queued native events and realization.
- Specialized adapters for controls such as `ItemsRepeater`.
- Current hook frontend as the comparison control.

Do not add:

- New control coverage.
- Direct signal-to-COM bindings.
- A generic Xilem-style type tree.
- Context or async component ownership before basic component identity passes.
- A worker-thread frontend/backend channel.

## Required contracts

### Ownership authority

One published structural relation is authoritative for component parent, key, type, and child
order.

```text
ApplicationHost
  -> WindowHost
       -> component scope storage
            user state, effects, message queues
       -> structural tree
            native nodes, component boundaries, scope references
       -> WinUI runtime
            COM handles, subscriptions, observed native state
```

Rules:

- Non-cloneable component state never enters or aliases a cloned candidate.
- Stable scope storage owns component instances but not a second parent-child graph.
- The published structural tree owns component parent, key, type, and order.
- Candidate scopes are reserved but cannot receive messages before publication.
- Failed candidates retire reserved scopes without exposing them.
- Published retirement runs cleanup before destroying native resources required by cleanup.
- A scope may be temporarily reserved or retiring, but never independently published.

### Identity domains

Window lifetime and native realization are separate invalidation domains:

```text
WindowToken {
    window_epoch,
    window_id,
}

ComponentToken {
    window_epoch,
    scope_id,
}

NativeToken {
    window_epoch,
    realization_epoch,
    node_id,
    revision,
}
```

Rules:

- Closing or recreating a logical window changes `window_epoch` and invalidates all window work.
- Native reset or root remount changes `realization_epoch` without invalidating component scopes.
- Component replacement relies on generational `scope_id`.
- Events, subscriptions, dispatcher work, async completions, repeater requests, and leases carry
  the narrowest token that proves their owner remains current.

### Publication

The following state layers are distinct:

- **Application state:** committed when a message or hook update returns.
- **Desired view:** derived from application state and retained across native failure.
- **Published structure:** committed after valid planning and its defined native apply outcome.
- **Native certainty:** updated from trusted observations, successful writes, and receipts.
- **Effects:** run only after the corresponding published/native state reaches its required point.

Native failure never rolls arbitrary application or component state back. Recovery retries or
remounts the same current desired view.

### Native property certainty

Controlled properties track:

```text
desired: T
native: Known(T) | Divergent
feedback: property-specific state
attempts: u32
```

`Known(T)` requires a trusted native observation, generated readback, or successful write under a
documented native contract. Setter failure produces `Divergent` unless readback establishes the
actual value.

Each controlled property declares one feedback contract:

| Contract | Runtime handling |
| --- | --- |
| Synchronous exact | Suppress the expected value during the setter |
| Synchronous normalized | Read back or compare the event payload |
| Deferred ordered | Track write revision and expected sequence |
| Deferred coalesced | Treat feedback as observation and reconcile desired versus observed |
| Unknown | Do not suppress; dispatch observation and repair rejected changes |

The first slice may support only synchronous exact feedback. Schema validation rejects unsupported
contracts.

### Scheduling and reentrancy

- Sending always enqueues; it never calls component code directly.
- Native callbacks only enqueue observations or messages.
- One scheduler drains each window in defined phases.
- A scope is mutably borrowed for one message or lifecycle call at a time.
- Messages sent during `update`, `changed`, or `view` wait for the current phase to end.
- Nested message loops mark work pending without reborrowing an active scope.
- Parent/child message order is deterministic.
- A per-tick budget defers excess work and preserves a wakeup.
- Enqueue rejection becomes an explicit fault or leaves work armed for retry.
- Panic follows the fatal host policy without leaving an active-borrow marker.

Use separate sender types:

```text
LocalSender<M: 'static>
BackgroundSender<M: Send + 'static>
```

Both become validated local envelopes on the UI thread.

### Component composition

The component specification must define:

- Native nodes and keyed component children in one `View`.
- Type plus key child identity.
- Prop creation, comparison, coalescing, and update ordering.
- Same type/key retention and same key/different type replacement.
- Empty, single-root, multi-root, pass-through, and component-only views.
- A logical component boundary that anchors local recomposition without adding a WinUI control.
- Safe heterogeneous storage and typed message erasure.
- Cleanup, replacement, and native subtree teardown.

Props are desired parent input, not ordinary messages. The ordering of prop application relative to
queued child messages must be deterministic.

`view` is initially infallible. Panic follows the existing fatal policy; no recoverable rendering
error boundary is part of this prototype.

### Effect lifecycle

- Setup runs only after the corresponding view commit.
- Dependency change runs old cleanup before new setup.
- Retirement runs cleanup before native resources required by cleanup are destroyed.
- Failed recovery does not run setup for an unrealized view.
- Cleanup is idempotent across replacement, close, and terminal fault.
- Framework subscriptions cannot retain an untracked cycle to their component scope.

## Work sequence

### 1. Repair the shared backend

- [x] Add separate window-lifetime and native-realization identity domains.
- [x] Reject stale events, callbacks, dispatcher work, and repeater leases after the corresponding
  epoch changes.
- [x] Move startup/window mounting to `OnLaunched`.
- [x] Install `XamlControlsResources`.
- [x] Replace scheduled booleans with an explicit pending/scheduled/dispatching/closing scheduler.
- [x] Preserve queued work across reentrancy and surface enqueue rejection as a terminal fault.
- [x] Run cleanup before revoking subscriptions and releasing required native resources.
- [x] Clear recycled repeater shell content synchronously.
- [x] Add property certainty and controlled-observation tracking.
- [x] Add schema-validated feedback contracts.
- [x] Retain event payload source and conversion in the schema.

**Exit:** verified backend behavior is independent of the frontend model.

### 2. Add the live backend harness

- [x] Add a process-isolated live WinUI fixture runner.
- [x] Verify `OnLaunched`, control resources, and first native view commit.
- [ ] Queue old work, remount native content, and reject only old native work.
- [ ] Close/recreate a window and reject all old window work.
- [ ] Exercise dispatcher reentrancy and enqueue rejection.
- [ ] Exercise mutate-then-fail setters and rejected controlled edits.
- [ ] Exercise delayed/coalesced feedback for supported contracts.
- [ ] Exercise repeater recycle and immediate shell reuse.
- [ ] Verify cleanup observes required native resources and runs once.
- [ ] Verify two windows cannot consume each other's work or faults.

`RecordingRuntime` remains useful for deterministic planning and failure positions. It cannot prove
template behavior, COM reentrancy, partial native mutation, shell visuals, or shutdown ordering.

**Exit:** the repaired backend passes headless and live WinUI tests before frontend comparison.

### 3. Write the component ownership specification

- [ ] Define the one authoritative parent-child relation.
- [ ] Define scope reservation, publication, retirement, and failed-candidate cleanup.
- [ ] Define candidate interaction with non-cloneable component state.
- [ ] Define `Component`, `View`, props, keys, logical anchors, and type replacement.
- [ ] Define safe erased storage and typed local message envelopes.
- [ ] Define queue phases, borrowing, ordering, budgets, and panic policy.
- [ ] Define effect ordering and native-resource access.
- [ ] Define per-window ownership and fault containment.

**Exit:** the four blocking contracts are explicit and do not depend on implementation convention.

### 4. Implement the bounded component prototype

- [ ] Add stable generational scope storage.
- [ ] Add logical component boundaries to the structural tree.
- [ ] Add nested components with props and local typed messages.
- [ ] Keep local recomposition within the component boundary.
- [ ] Retain child state across parent prop changes and keyed movement.
- [ ] Replace same-key/different-type children and retire the old scope.
- [ ] Queue reentrant messages without directly reborrowing a component.
- [ ] Retain current application state and desired view across native failure.
- [ ] Run cleanup exactly once in the required order.

Do not add context, background senders, or general async ownership in this phase.

**Exit:** nested ownership, identity, replacement, failure retry, and cleanup pass headlessly and
against live WinUI.

### 5. Compare hooks and components

Run equivalent applications through both frontends:

- Counter.
- Controlled form with rejected input.
- Nested pass-through and native-owning components.
- Keyed insert, move, removal, and type replacement.
- Empty, single-root, multi-root, and component-only views.
- Reentrant message/event delivery.
- Repeated mount and retirement.
- Virtual collection with immediate shell reuse.
- Two windows.

Compare:

- Correctness and failure convergence.
- Source size and framework boilerplate.
- Compiler diagnostics for props, messages, keys, and thread use.
- Render, plan, native apply, publication, and effects separately.
- Allocation count and retained memory per idle component.
- Compile time, executable size, p95/p99 latency, dispatcher backlog, and resource bounds.

### 6. Conditional context and async proof

Only after the basic component gates pass:

- [ ] Add context dependency tracking and consumer-only invalidation.
- [ ] Add `BackgroundSender<M: Send>`.
- [ ] Add owner-scoped async cancellation.
- [ ] Reject completion after scope or window retirement.
- [ ] Preserve window isolation under close and fault.

### 7. Continue, pivot, or stop

Proceed with owned components only if they show a clear correctness, locality, usability, or
performance advantage over hooks. Do not remove hooks or expand controls before this decision.

If both frontends require duplicate identity graphs, component-specific native recovery, unbounded
retry/scheduler behavior, or proportional unrelated-tree work, stop the replacement effort and
carry the proven generator and backend work into the current reactor.

## Continuation gates

| Gate | Required result |
| --- | --- |
| Scope storage | Candidate state never aliases non-cloneable scope state; reserved scopes are not dispatchable |
| Parent-child authority | One published relation owns component parent, key, type, and order |
| Epoch domains | Window lifetime and native realization invalidation are separate and tested |
| Controlled feedback | Every controlled property has a supported contract |
| Reentrancy | Message send is queue-only; nested loops cannot reborrow an active scope |
| Props | Coalescing, comparison, and ordering relative to messages are deterministic |
| Anchoring | Empty, single-root, multi-root, and component-only views recompose locally |
| Failure | Mutate-then-fail setters, structural failure, and rejected edits converge |
| Lifecycle | Effects and subscriptions clean up once before required native resources disappear |
| Scheduling | No lost wakeup under rejection, reentrancy, cross-window activity, or close |
| Generation | Ordinary controls remain schema-only; event payloads retain typed source conversion |
| Ergonomics | Representative code does not expose erased internals or scheduler plumbing |
| Resources | Repeated component, window, and repeater lifecycles reach bounded steady state |
| Complexity | No second independent identity tree or component-specific native recovery |
| Live evidence | Automated WinUI tests cover templates, two windows, input, repeater reuse, and shutdown |

## Numerical gates

Thresholds are fixed before collecting prototype results. Compare equivalent hook and component
applications on the same machine and toolchain.

| Metric | Gate |
| --- | --- |
| Clean and incremental compile time | Component <= 1.25x hook |
| Release executable size | Component <= 1.20x hook |
| No-change and isolated-leaf median | Component <= 1.25x hook |
| No-change and isolated-leaf p95 | Component <= 1.50x hook |
| Isolated leaf, 512 -> 16K unrelated nodes | Time grows <= 25%; allocation count remains constant |
| Keyed siblings, 512 -> 4K | No quadratic growth |
| Repeated mount and retirement | Scope, callback, COM handle, and memory counts reach steady state |
| Message burst | Queue is bounded or applies documented backpressure |
| Native recovery | Fixed per-tick budget; cannot monopolize the UI thread |
| Idle scope memory | Report retained bytes at 512, 4K, and 16K scopes |
| Compile diagnostics | Invalid props, messages, and thread crossing fail at the public API boundary |

Report update CPU p95 and p99 separately from end-to-end frame latency. Set the absolute CPU budget
from current-reactor measurements on the same live scenarios; a full 16.7 ms frame is not an
acceptable update CPU budget.

The locality gate excludes declared dependencies such as changed parent structure or consumed
context.

## Current work

Current phase: **2 - add the live backend harness**

- [x] Implement separate window and realization identity domains.
- [x] Add stale-work tests for native remount and complete window replacement.
- [x] Replace the live scheduler boolean with an explicit state machine.
- [x] Mount from `OnLaunched` and install WinUI control resources.
- [x] Run effect cleanup before native reset.
- [x] Clear recycled repeater shells synchronously.
- [x] Replace committed-only property comparison with desired/known/divergent native state.
- [x] Restore rejected controlled edits from native observations.
- [x] Require an explicit supported feedback contract for controlled properties.
- [x] Retain typed event payload source and conversion in the schema.
- [x] Add the first live startup/resources fixture.
- [ ] Add live tests for scheduler reentrancy and shutdown ordering.

Control expansion remains frozen.

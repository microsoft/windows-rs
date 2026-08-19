# Windows Reactor Next

## Goal

Build `windows-reactor-next` beside the current crate and prove a simpler core before expanding
control coverage.

The design combines:

- Reactor2's single generational arena and queued native boundary.
- Current reactor's metadata generation and typed public API.
- C# Reactor's property clearing, controlled-value, and event-lifecycle contracts.

The first milestone is an end-to-end representative slice, not API parity.

## Core design

### Ownership

One arena is the only mutable ownership graph:

```text
Arena<Node>
  Application
  Window
  Component
  Slot
  Native
  VirtualCollection
```

Each node has one parent and a generational `NodeId`. Native handles, subscriptions, callback
cells, comparison state, and controlled-value state belong to the corresponding arena node.

There is no separate backend identity tree.

### Update cycle

The UI-thread pump runs these phases:

```text
drain queued work
-> render dirty roots
-> reconcile and validate
-> build ordered native commands
-> apply commands
-> commit successful comparison state
-> update refs and run cleanups/effects
-> dispatch application callbacks
```

Rules:

- Native callbacks enqueue events and do not render directly.
- State, context, size, and DPI changes initially render the application root.
- Explicit memo nodes compare only their own props; callbacks are excluded.
- A work budget defers excess work to the next dispatcher tick without dropping it.
- Async work is cancelled when its owner retires.
- `ItemsRepeater` realization is a narrow synchronous exception guarded by a lease and
  reentrancy rules.

### Generated control model

WinUI metadata supplies the base control, property, event, and dependency-property facts. A small
curation schema adds:

- Public capabilities.
- Controlled-property behavior.
- Clearability.
- Content and structural slots.
- Event coalescing.
- Property ordering.
- Re-creatable control policy.
- Complex-control overrides.

The same schema generates:

- Props and builders.
- Capability implementations.
- Element and mounted variants.
- Native construction and teardown.
- Property comparison, set, and clear operations.
- Event payloads, subscriptions, and callback dispatch.
- Recording-runtime descriptions.
- Compile-fail API tests.

Adding an ordinary control should require one schema row and at most one reviewed override. It must
not require parallel hand-written matches.

### Properties and events

Clearable dependency properties use:

```rust
enum Property<T> {
    Inherited,
    Set(T),
}
```

`Inherited` calls `ClearValue`. Non-clearable properties receive a different generated API.

Native event handlers are subscribed once. They read the latest callback from an arena-owned cell.
Queued events carry node generation and subscription revision.

Controlled properties store expected native feedback beside the native handle. Reactor-originated
writes do not echo into application state.

### Failure handling

WinUI mutation is not transactional, so the design does not attempt rollback.

- A property failure reports to the fault sink, marks the property diverged, and does not commit
  its comparison value.
- A structural failure retires and remounts the nearest re-creatable subtree with fresh
  generations.
- A failure involving non-recreatable native state escalates through the fault sink.
- A failed remount leaves a faulted subtree and does not retry indefinitely.

No recoverable failure is debug-only or silently ignored.

### Collections

One keyed differ produces an abstract operation list.

- Small panels apply the operations as structural commands.
- Virtual models apply them as collection notifications.
- Realized rows use keys and leases to reject callbacks from recycled containers.

Exit animation is deferred because it introduces a second physical child order.

## Initial scope

The representative slice includes:

- `Application` and multiple `Window` nodes.
- `TextBlock` as a leaf.
- `Button` for content and events.
- `StackPanel` for keyed children.
- `TextBox` for controlled values and `ClearValue`.
- A minimal read-only `ItemsRepeater`.
- A non-recreatable native-control test adapter.
- State, callback, effect cleanup, context, and owner-scoped async work.

Deferred:

- Control pooling.
- Exit animation.
- Selective dirty-ancestor scheduling.
- Panic error boundaries.
- Navigation infrastructure.
- Advanced list selection, grouping, and drag/reorder.
- Public backend or reconciler mutation APIs.
- Packed representations added only for benchmark results.

## Implementation phases

### 0. Baseline

- Record toolchain, target, profile, and runner.
- Record current reactor clean check, release build, incremental check, and executable size.
- Record `test_reactor_bench` output.
- Record live stock-grid results at 10% and 100% mutation.
- Define paired counter and representative-tree samples.

**Exit:** measurements are reproducible and paired scenarios have matching behavior.

### 1. Workspace

- Add unpublished `windows-reactor-next`.
- Add the semantic generator, headless tests, benchmark, and counter sample packages.
- Establish private module boundaries.
- Add a public-API snapshot and generated-diff check.
- Build against `RecordingRuntime` without starting WinUI.

**Exit:** all packages build and generated output is deterministic.

### 2. Schema and generator

- Generate `TextBlock`, `Button`, `StackPanel`, and `TextBox`.
- Generate capabilities, props, mounted state, native operations, and event pairing.
- Add compile-fail tests for unsupported operations.
- Measure edit sites for one ordinary property and control.

**Exit:** ordinary coverage grows through the schema without parallel runtime edits.

### 3. Pure core

- Implement the generational arena and child-first retirement.
- Implement keys and one keyed differ.
- Implement `RecordingRuntime`.
- Implement the initial hooks and phased pump.
- Implement command receipts and conditional comparison-state commit.
- Implement queued events with stale-work rejection.
- Add randomized model tests and command failure injection.

**Exit:** counter, nested component, and keyed panel pass headlessly.

### 4. Native slice

- Start and stop WinUI with application and window nodes.
- Mount `StackPanel`, `TextBlock`, and `Button`.
- Apply set, clear, insert, remove, and move operations.
- Queue button events and coalesce state updates.
- Revoke subscriptions before releasing handles.
- Add the fault sink.

**Exit:** the counter runs without callbacks during arena mutation or native apply.

### 5. Controlled input and failures

- Add `TextBox`.
- Implement mount-before-subscribe and expected-feedback suppression.
- Replace callbacks without native resubscription.
- Implement property divergence and structural failure handling.
- Test non-recreatable failure escalation.

**Exit:** controlled input and every injected failure reach a defined state.

### 6. Collections and lifecycle

- Add minimal `ItemsRepeater` and realization leases.
- Use the shared keyed differ for panels and models.
- Test recycled callbacks, pending work during window close, and repeated mount/retire cycles.
- Prove Reactor-owned resource counts return to a bounded steady state.

**Exit:** collection realization and shutdown cannot retain or address retired state.

### 7. Expansion decision

Review all correctness, complexity, compile-time, runtime, and memory gates. Expand control coverage
only if the representative slice remains simpler and competitive with current reactor.

## Gates

### Hard from the start

- The arena is the only mutable ownership graph.
- Native records cannot form an independent tree.
- Ordinary control mappings come from one schema.
- The pure core contains no `unsafe`.
- Control correspondence does not depend on `unreachable!()`.
- Unsupported operations have no silent catch-all.
- Generated output has a zero-diff check.
- Runtime internals remain private.
- User code does not run during reconciliation or native apply.
- Failed writes are not committed as successful state.
- Stale ids, events, state handles, and leases are rejected.

### Semantic runtime

- No-change update: zero native commands.
- One-property update: one property command and no control creation.
- Keyed reorder: correct final order without recreating survivors.
- `Inherited`: uses `ClearValue`.
- Controlled native input: one application callback.
- Reactor-originated controlled write: no echo callback.
- Shutdown: no work touches released COM objects.
- Resource counts: bounded after repeated lifecycle tests.

### Compile time and artifact size

Compare paired applications on the same machine and toolchain:

- Clean `cargo check`.
- Clean debug and release builds.
- Incremental application-only check.
- Release executable size.

Measurements are report-only until representative parity.

After parity:

- Warn above current reactor by 10%.
- Fail after confirmation above current reactor by 20%.
- Record the cost of every new dependency or proc macro.
- Keep semantic generation offline so applications do not compile the generator.

### Headless performance

Compare component mount, mount/unmount, no-change update, one-leaf update, keyed reverse/rotate,
event dispatch, controlled values, and stale-event rejection.

After parity:

- Time: warn above 15%, fail after confirmation above 25%.
- Allocated bytes: warn above 5%, fail above 10%.
- No-change allocation count must not exceed current reactor.
- Track nodes visited, components rendered, commands, controls created, and controls retired.

### Live performance

Compare startup, 95th-percentile update time, FPS, allocations per render, working set, control
churn, and dispatcher backlog.

After representative parity:

- Warn above current reactor by 10%.
- Fail after confirmation above current reactor by 20%.
- Do not accept a better median that introduces long UI-thread stalls.

## Current checklist

### In progress

- [ ] Capture current reactor baseline measurements.
- [ ] Define paired sample behavior.

### Next

- [ ] Add workspace packages.
- [ ] Establish the metadata and curation schema.
- [ ] Generate the first four controls.
- [ ] Implement the arena and recording runtime.

### Decisions needed before implementation

- [ ] Public key representation.
- [ ] Generated command encoding.
- [ ] Runtime receipt granularity.
- [ ] Fault-sink API.
- [ ] Per-window or per-UI-thread arena.
- [ ] Pump work-budget unit.
- [ ] Owner-scoped async contract.
- [ ] Synchronous realization contract.

### Gate state

- Architecture and correctness gates: active.
- Compile and runtime gates: report-only until parity.
- Current phase: 0 - baseline.
- Next action: capture baseline environment and benchmark output.

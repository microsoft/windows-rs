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
- `ItemsRepeater` callbacks synchronously lease stable container shells and enqueue realization.
  User row builders and arena mutation run on the dispatcher commit path.
- Deferred shells have a nonzero estimated extent; zero-size shells can force full-source
  realization before queued content is mounted.

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

WinUI mutation is not transactional, so the pump does not attempt rollback.

- A property failure reports to the fault sink, keeps the old comparison value and render version,
  and retains a retry signal.
- A structural update failure does not publish the failed candidate. The runtime clears the window
  content and remounts the control root with fresh generations while preserving the application
  and window.
- A successful remount reports a recoverable fault. A failed remount or host-creation failure
  poisons the pump.

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

- [x] Record toolchain, target, profile, and runner.
- [x] Record current reactor clean check, release build, incremental check, and executable size.
- [x] Record `test_reactor_bench` output.
- [x] Record live stock-grid results at 10% and 100% mutation.
- [x] Define paired counter and representative-tree samples.

**Exit:** measurements are reproducible and paired scenarios have matching behavior.

### 1. Workspace

- [x] Add unpublished `windows-reactor-next`.
- [x] Add the semantic generator, headless tests, benchmark, and counter sample packages.
- [x] Establish private module boundaries.
- [x] Add a public-API snapshot.
- [x] Add a generated-diff check.
- [x] Build against `RecordingRuntime` without starting WinUI.

**Exit:** all packages build and generated output is deterministic.

### 2. Schema and generator

- [x] Generate `TextBlock`, `Button`, `StackPanel`, and `TextBox`.
- [x] Generate capabilities, props, mounted state, native operation descriptors, and event pairing.
- [x] Add compile-fail tests for unsupported operations.
- [x] Prove an ordinary control needs only schema input.

**Exit:** the semantic API and backend-neutral descriptors grow from one schema.

### 3. Pure core

- [x] Implement the generational arena and child-first retirement.
- [x] Implement keys and one keyed differ.
- [x] Implement `RecordingRuntime`.
- [x] Implement the initial hooks and phased pump.
- [x] Implement command receipts.
- [x] Implement conditional comparison-state commit.
- [x] Implement queued events with stale-work rejection.
- [x] Add randomized model tests and command failure injection.

**Exit:** counter, nested component, and keyed panel pass headlessly.

### 4. Native slice

- [x] Own application and window lifetimes in arena nodes.
- [x] Mount `StackPanel`, `TextBlock`, and `Button`.
- [x] Apply set, clear, insert, remove, and move operations.
- [x] Queue button events and coalesce state updates.
- [x] Revoke subscriptions before releasing handles.
- [x] Generate ordinary native mappings from the schema.
- [x] Add the fault sink.

**Exit:** the counter runs without callbacks during arena mutation or native apply.

### 5. Controlled input and failures

- [x] Add `TextBox`.
- [x] Implement mount-before-subscribe and expected-feedback suppression.
- [x] Replace callbacks without native resubscription.
- [x] Implement property divergence and structural failure handling.
- [x] Test non-recreatable failure escalation.

**Exit:** controlled input and every injected failure reach a defined state.

### 6. Collections and lifecycle

- [x] Add minimal vertical `ItemsRepeater`, stable shells, and realization leases.
- [x] Use the shared keyed differ for panels and models.
- [x] Reject stale keys, containers, collection generations, and duplicate recycle work.
- [ ] Test pending realization during window close and repeated native mount/retire cycles.
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
- Crate exports come from module-level wildcard re-exports; internal modules do not maintain
  parallel symbol lists.
- Test-only backends stay behind one module boundary; ordinary unit tests stay in `tests` modules.
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

- [x] Generate shallow mounted props and split structural payloads without cloning subtrees.
- [x] Implement initial state and callback hooks.
- [x] Add candidate-tree recursive mount, property updates, and keyed move/insert/remove.
- [x] Add state scheduling, queued callback dispatch, effects, cleanup, and a render budget.
- [x] Make failed commits explicit, preserve property retries, and poison structural divergence.
- [x] Add arena-owned virtual models and generation-checked realization leases.
- [x] Include native container identity in leases and reject reused-container callbacks.

### Next

- [x] Generate `TextBox.TextChanged` payload extraction.
- [x] Suppress controlled-property feedback.
- [x] Replace whole-pump poison with fresh-generation root remount where safe.

### Decisions needed before implementation

- [x] Fault-sink API.
- [ ] Per-window or per-UI-thread arena.
- [ ] Owner-scoped async contract.
- [x] Queued realization and factory contract.

### Gate state

- Architecture and correctness gates: active.
- Compile and runtime gates: report-only until parity.
- Current phase: 6 - collections and lifecycle.
- Next action: finish window-close and repeated native lifecycle tests for realized rows.

## Current reactor baseline

Environment: commit `e73382fa`, Rust nightly 1.99.0, `x86_64-pc-windows-msvc`, Windows 11 build
26674, Core i9-12900K, Windows App Runtime 2.4.0.

| Compile measurement | Result |
| --- | ---: |
| Library clean check | 4.903 s |
| Stock sample clean check | 5.148 s |
| Stock sample incremental check | 0.493 s |
| Stock sample clean release build | 12.269 s |
| Stock sample executable | 3,270,656 bytes |

Selected headless results:

| Scenario | ns/op | bytes/op | allocations/op |
| --- | ---: | ---: | ---: |
| Component mount | 700 | 399 | 10 |
| Mount/unmount 512 | 199,818 | 152,536 | 2,076 |
| One changed leaf in 512 | 11,931 | 658 | 7 |
| No change in 512 | 7,386 | 0 | 0 |
| Keyed reverse 512 | 101,004 | 58,244 | 11 |
| Keyed rotate 512 | 67,286 | 82,828 | 33 |

| Live stock grid | 10% mutation | 100% mutation |
| --- | ---: | ---: |
| Average FPS | 55.3 | 9.4 |
| Average reconcile | 3.58 ms | 11.24 ms |
| Allocated bytes/render | 4.13 MB | 6.28 MB |
| Average working set | 182.6 MB | 190.3 MB |
| Average private memory | 229.0 MB | 237.7 MB |

The paired counter covers window, panel, text, button events, state, and effect cleanup. The paired
representative tree adds nested components, one changed leaf, no-change renders, keyed reorder,
controlled text, property clearing, and a virtual collection.

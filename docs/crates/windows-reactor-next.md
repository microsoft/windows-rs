# windows-reactor-next

`windows-reactor-next` is an unpublished experiment for a smaller declarative WinUI architecture.
It is developed beside `windows-reactor` and will not replace it until the architecture,
correctness, compile-time, runtime, memory, and live WinUI gates in
[`reactor-next2.md`](../../reactor-next2.md) pass.

## Architecture

The core has one authoritative structural `Tree`. It owns native nodes, component boundaries,
context providers, fragments, keys, parents, and child order. A separate generational component
store owns non-cloneable component instances, state, effects, context reads, and message queues
without duplicating the structural graph.

Updates follow one path:

```text
state
  -> desired View or Element
  -> validated candidate
  -> ordered native commands
  -> native apply
  -> candidate and effect publication
```

The planner is runtime-independent. `core/pump/planner/` handles topology, ordinary elements, and
component views. `core/pump/publish.rs` owns update publication. Initial mount and virtual
realization use separate paths because neither replaces a published candidate.

## Native failure policy

Reactor validates candidates before native mutation and trusts WinUI's documented contracts.
`NativeRuntime::apply` stops at the first failing command and returns
`NativeApplyError { command, error }`.

An unexpected native failure is fatal in the production WinUI host. It reports the command and
error and aborts the process. Reactor does not inspect partial success, retry properties, repair
structure, remount controls, or reconstruct the window.

This policy keeps platform workarounds narrow. A workaround may be added for a demonstrated,
repeatable platform defect. Canvas device loss and WebView process failure remain specialized
adapter concerns because those APIs document those failure modes.

## Components and hooks

The owned-component frontend stores stable generational scopes outside cloned candidates. Scope
reservation, publication, replacement, and retirement follow the structural tree transaction.
Same-key, same-type children retain scopes across prop updates and keyed movement.

Component sends are queue-only. Each window accepts at most 4,096 queued messages, and each
dispatcher turn drains at most 64 messages. Dirty scopes compose parent-first. Parent props apply
before queued child messages, and retiring a child removes its queued work.

An isolated native leaf uses a property-only candidate and does not clone the full tree. Hooks
remain available as a comparison frontend and use the same planner and update publication path.
Hook and component effect cleanup runs before native mutation, and setup runs after publication.

If component props are applied but later candidate validation fails, Reactor records each touched
scope. Direct updates and component turns seed their next candidate with those scopes, so an
identical-props retry recomposes them instead of accepting the old structural tree.

## Context

`Context<T>` is a typed key with a default value. `View::provide` creates a transparent logical
provider, and `ViewContext::use_context` resolves the nearest matching ancestor.

Each consumer records the provider node it resolved, not only the context type. A provider value
change therefore skips consumers shadowed by a nested provider. Reactor recomposes only matching
consumers and their component ancestor paths. Keyed movement retains provider identity, retirement
removes dependency state with the component scope, and separate Pumps never share values or
subscriptions.

Context reads are candidate data. Planning stages new dependency sets and publishes them only after
native apply succeeds. A failed plan retains the previously published dependencies and forces the
same touched scopes to retry.

The arena uses 256-node copy-on-write chunks, and the scope-to-node index is copy-on-write. This
keeps context and other structural candidates from cloning the full tree while preserving
transactional publication.

## Native events and controlled properties

Native callbacks capture typed payloads and enqueue them with the current window identity and event
revision. Stale work is discarded before dispatch.

Controlled feedback updates the known native value before invoking the application callback. If
the application rejects the edit, ordinary reconciliation writes the desired value again. There
is no divergent-property state or retry scheduler. An unexpected restoring-setter failure follows
the fatal native policy.

The generated slice currently accepts synchronous exact feedback. Unsupported feedback contracts
fail generation.

## Fragments and collections

Logical fragments create no hidden WinUI control. They flatten zero or more native roots into
generated children collections. Window and content slots accept zero or one flattened root and
reject invalid arity before native mutation.

Component-owned keyed children build one key index and one desired order. Small keyed edits use
insert and move commands. Updates with 256 or more ordering operations use child synchronization,
which bounds repeated vector search and movement for dense and adversarial sparse reorders.

`ItemsRepeater` owns virtual collection leases and stable native shells. Recycling clears shell
content before reuse. Realized row subtrees remain ordinary arena nodes and retain independent
generation checks.

## Scheduling and lifecycle

Each dispatcher turn handles at most 64 native events, 64 component messages, and 32 realization
requests. Remaining work rearms the scheduler. Work queued during dispatch is not lost, and
dispatcher rejection is an explicit host fault.

Changed and retired effect cleanup runs child-first before native mutation. New setup runs
parent-first after publication. Normal shutdown cleans effects before native reset, and cleanup is
idempotent across shutdown and `Drop`.

These budgets bound queued work items, not the size of one component composition or candidate.
Large synchronous component trees are controlled by the locality and keyed-scale gates rather than
a continuation state machine.

## Multi-window host

`LiveHost` owns the WinUI `Application` and a `WindowToken`-keyed Pump map. Each Pump owns its tree,
component store, queues, scheduler state, native window, subscriptions, and effects.

Scheduler callbacks route only to the captured token. `Window.Closed` removes that Pump during the
native lifecycle callback, closes its scheduler, and suppresses native reset against the closed
window. The host tracks a Pump that is temporarily in flight, so a synchronous close cannot
reinsert it or make another close appear to be the last window. A stale queued callback then finds
no matching Pump. Closing the last window exits the UI thread; closing a secondary leaves the
other Pumps running.

This is ownership and lifecycle isolation, not native fault isolation. An unexpected native
failure in any Pump follows the process-fatal policy above.

## Generation

`tool_reactor_next` reads `crates/tools/reactor-next/src/winui.toml` and generates the public typed
controls, minimal bindings filter, handle variants, property operations, structural roles, and
event payload conversion. Generated files must not be edited by hand.

```powershell
cargo run -p tool_reactor_next --quiet
cargo check -p windows-reactor-next --quiet
```

## Current evidence

The component frontend measured about 0.5 us, 430 allocated bytes, and 9 allocations for an
isolated leaf at both 512 and 16,384 unrelated scopes. Idle storage was about 2,448 bytes per scope.
Equivalent component applications measured 0.99x hook time for a clean build and 1.01x for a
source-only rebuild. The component release executable was 0.91x the hook executable.

Removing fine-grained recovery reduced `core/pump/publish.rs` from 396 lines to 57 and removed
per-command outcome vectors, divergent properties, retries, remount recovery, recovery
continuations, and their specialized tests.

Component-owned keyed updates scale near-linearly from 512 to 4,096 children. The current release
measurements are about 0.42 ms/4.09 ms for same-order parent recomposition and 0.49 ms/5.76 ms for
reversal. Rotate, insert, and remove remain in the same range at 4,096 children. Reorders moving
10%, 20%, and 25% of 4,096 keys take about 5.0-5.4 ms and use child synchronization.

An isolated context-provider update measured about 3.4 us and 7.7 KB at 512 unrelated scopes and
4.1 us and 9.2 KB at 16,384. Only the resolved consumer recomposed; the time increase was about
21%.

The `test` feature exposes the recording runtime and Pump to the headless test and benchmark
packages. `test_reactor_next_selftest` exercises two real WinUI windows in a process-isolated
fixture. It edits one window at a time, verifies window-specific callback payloads, closes the
secondary with stale queued work, and continues updates and final effect cleanup in the surviving
window. An explicit completion marker prevents an early `App::run_windows` return from passing.

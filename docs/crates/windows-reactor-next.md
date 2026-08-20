# windows-reactor-next

`windows-reactor-next` is an unpublished experiment for a smaller declarative WinUI architecture.
It is developed beside `windows-reactor` and will not replace it until the architecture,
correctness, compile-time, runtime, memory, and live WinUI gates in
[`reactor-next2.md`](../../reactor-next2.md) pass.

## Architecture

The core has one authoritative structural `Tree`. It owns native nodes, component boundaries,
fragments, keys, parents, and child order. A separate generational component store owns
non-cloneable component instances, state, effects, and message queues without duplicating the
structural graph.

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
component views. `core/pump/publish.rs` is the only candidate publication path.

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
remain available as a comparison frontend and use the same planner and publication path.

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

Sparse keyed edits use insert and move commands. Dense keyed reversal uses `ResetChildren`
followed by ordered attachment to avoid quadratic vector movement.

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
isolated leaf at both 512 and 16,384 unrelated scopes. Idle storage was about 2,455 bytes per scope.
Equivalent component applications compiled at 0.78x-0.91x hook time and produced a 0.97x release
executable.

Removing fine-grained recovery reduced `core/pump/publish.rs` from 396 lines to 57 and removed
per-command outcome vectors, divergent properties, retries, remount recovery, recovery
continuations, and their specialized tests.

The `test` feature exposes the recording runtime and Pump to the headless test and benchmark
packages. `test_reactor_next_selftest` exercises the real WinUI host in a process-isolated fixture.

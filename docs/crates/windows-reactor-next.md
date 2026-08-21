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

## Components

The owned-component frontend stores stable generational scopes outside cloned candidates. Scope
reservation, publication, replacement, and retirement follow the structural tree transaction.
Same-key, same-type children retain scopes across prop updates and keyed movement.
The component store owns current props and borrows them into `Component::view`. Components that
render directly from props do not need duplicate fields or a `changed` implementation.

Generated controls convert directly to `View`. Structural capabilities are terminal methods on
the controls: `content`, `children`, `keyed_children`, and `slots` all return the same core `View`
used by components and the planner. This is not a wrapper frontend. Property and event builders
must run before a terminal structural method.

`children` assigns positional identity only through the sealed `IntoViews` trait. `()` supplies an
empty shape, fixed arrays supply homogeneous shapes, and tuples of up to 16 elements supply
heterogeneous shapes whose leaves each implement `Into<View>`. The tuple syntax therefore accepts
controls, components, and terminal content views together without per-leaf `.into()` calls.

Dynamic lists cannot implement `IntoViews`. Inserting at the front of an index-identified list
would reuse each existing component scope for the next item, reproducing React's index-as-key state
bug. Dynamic `Vec`, slice, and iterator inputs must use `keyed_children` with `KeyedView`.
Explicit keys follow items through insertion and reordering. Public numeric and string keys cannot
collide with the private positional-key domain.

Component sends are queue-only. Each window accepts at most 4,096 queued messages, and each
dispatcher turn drains at most 64 messages. Dirty scopes compose parent-first. Parent props apply
before queued child messages, and retiring a child removes its queued work.

`ViewContext::callback` maps an event payload to a component message, while
`ViewContext::message` clones a fixed message for repeated zero-argument events. The methods
forward to the equivalent `LocalSender` methods and produce `Callback<T>` values; they do not add
a queue or state path. Generated payload and zero-argument event setters use generic conversion
traits so both these callbacks and ordinary unit-returning closures retain the same control API.

An isolated native leaf uses a property-only candidate and does not clone the full tree. Component
effect cleanup runs before native mutation, and setup runs after publication.

## Imperative element references

`ElementRef<T>` is typed to a focus-capable generated control and is normally an owned component
field. Those controls bind it with `element_ref`; a reference for one control type cannot bind to
another. `Button`, `TextBox`, `NumberBox`, `Slider`, and `ToggleSwitch` implement the sealed
generated focus capability, which adds `request_focus() -> bool` to their references.

The return value reports queue acceptance, not the final WinUI focus result. A request captures the
current `WindowToken` and generational `NodeId`, enters that Pump's shared imperative queue, and
wakes the host. The host drains pending native events and component messages before imperative work,
so removal or replacement publishes before stale work is checked. `Focus(Programmatic)` returning
`false` completes normally; an HRESULT error follows the native failure policy.

The per-window imperative queue accepts 4,096 requests and applies at most 64 in one host turn.
`request_focus` returns `false` when the reference is unbound or the queue is full. Valid requests
in one turn are sent to the runtime as one command batch.

Reference attach, swap, and detach are candidate commits. Planning only records them. Successful
native apply and logical publication bind the new target before effect setup. Failed planning or
native apply cannot expose a candidate binding. Retirement, shutdown, and window close conditionally
unbind the exact published identity and clear pending imperative work. This also covers locally
composed native leaves and lazily realized `View` rows.

One reference has one published owner. Candidate validation rejects duplicate use within a tree or
across windows with `PumpError::DuplicateElementRef` before effect cleanup or native mutation.

Raw WinUI handles are intentionally not exposed or cloned. Render, update, event, and effect
callbacks remain queue-only. Canvas, WebView, and other specialized subsystems need adapters with
their own ownership and documented-failure contracts.

`ComponentContext::window` returns a token-bound `WindowRef`. `request_close` is accepted only
during `create`, `changed`, or `update`, where the component lifecycle invocation provides the
candidate transaction. One accepted close is staged with that candidate and applied in a separate
native batch after frontend publication and effect setup. Planning failure discards it. Shutdown
closes the endpoint, and references from another Pump or an inactive lifecycle call return false.
The live `Window.Closed` callback then follows the ordinary in-flight close path.

`ViewContext::use_effect(key, dependency, setup)` identifies each effect with an opaque
`EffectKey`. Numeric and string conversions make keys concise without exposing an internal
positional form. Each key must be unique within one component view. Omitted keys clean their
published effects, reordered keys retain their effects, and changed typed dependencies clean then
set up exactly once across the publication boundary. Duplicate keys return
`PumpError::DuplicateEffectKey` during planning, before effect cleanup or native mutation.
`begin_view` clears only pending registrations from an earlier failed plan; published keyed slots
remain available for the retry. Owned components have no positional hook-order contract.

If component props are applied but later candidate validation fails, Reactor records each touched
scope. Direct updates and component turns seed their next candidate with those scopes, so an
identical-props retry recomposes them instead of accepting the old structural tree.

## Context

`Context<T>` is a typed key with a default value. `View::provide` creates a transparent logical
provider, and `ViewContext::use_context` resolves the nearest matching ancestor.

Each consumer records the provider node it resolved, not only the context type. A published reverse
dependency index maps that identity to consuming scopes. A provider change therefore recomposes the
exact surviving consumers without scanning its subtree or recomposing unchanged component paths.
Changing the context key also skips consumers shadowed by nearer providers. Keyed movement retains
provider identity, retirement removes dependency state with the component scope, and separate Pumps
never share values or subscriptions.

Context reads are candidate data. Planning stages new dependency sets and publishes them only after
native apply succeeds. A failed plan retains the previously published dependencies and forces the
same touched scopes to retry.

The arena uses 256-node copy-on-write chunks, the scope-to-node index is copy-on-write, and sparse
provider values use a two-level chunked store. Updating one provider clones only its small provider
chunk and directory group rather than a global provider map. Context dependencies and their reverse
index publish only after native success.

## Background tasks

`ComponentContext::spawn_background` runs a `Send` closure on an owned OS thread. The closure
receives a cooperative `CancellationToken`; its typed result enters the normal component message
path on the UI thread. Local non-`Send` messages and background `Send` completions remain in
separate queues, and draining alternates between them.

A window permits 64 live task threads and 4,096 queued completions. Exceeding either bound produces
a `ComponentTask` with `Rejected` status. Dispatcher wake rejection rejects all completions covered
by that wake, so none can strand the queue. Scope retirement, explicit cancellation, window close,
and Pump shutdown mark owned work `Cancelled`, remove queued completions, and rely on scope
generations to reject late races.

Cancellation is cooperative for the closure but absolute for delivery. A closure that ignores its
token may continue running, but its result cannot reach a retired or replaced component. Task
panics remain confined to their worker thread and produce `Rejected` status.
Dropping a `ComponentTask` handle does not cancel its task.

## Native events and controlled properties

Native callbacks capture typed payloads and enqueue them with the current window identity and event
revision. Stale work is discarded before dispatch.

`Callback::call` returns an acceptance bit. Ordinary user closures return `true`. Callbacks
adapted from `LocalSender` return the exact result of `LocalSender::send`. After window, node,
subscription, and event-revision checks pass, a `false` result leaves the native event at the front
of the Pump queue. The host drains component work and retries the callback on a later turn. This
preserves bounded local-message backpressure without losing the event or faulting the window. Work
rejected by an earlier stale-event check never invokes the callback.

Controlled feedback updates the known native value before invoking the application callback. If
the application rejects the edit, ordinary reconciliation writes the desired value again. There
is no divergent-property state or retry scheduler. An unexpected restoring-setter failure follows
the fatal native policy.

Generated controls distinguish synchronous exact feedback from synchronous normalized feedback.
TextBox and ToggleSwitch suppress only the exact payload expected from their setters. NumberBox
and Slider suppress a programmatic `ValueChanged` during `Minimum`, `Maximum`, or `Value` writes
because WinUI may coerce the numeric payload. The last suppressed normalized payload updates known
native state without invoking the application callback or scheduling an immediate retry. Bounds
are generated before `Value`, and only `Value` observes user feedback. Two NaN values compare as
the same empty numeric state during reconciliation. Deferred and unknown feedback contracts still
fail generation.
Clear operations suppress the same synchronous event but do not retain its concrete default as a
native observation. The known-native state remains `None`, which represents a cleared local value
and keeps clear-then-rerender idempotent.

## Fragments and collections

Logical fragments create no hidden WinUI control. They flatten zero or more native roots into
generated children collections. Window and content slots accept zero or one flattened root and
reject invalid arity before native mutation.

`View::fragment` accepts the same statically shaped `IntoViews` inputs as `children`.
`View::keyed_fragment` accepts dynamic `KeyedView` collections when fragment descendants need
explicit stable identity. Both forms become the existing keyed fragment edges before planning and
use the same `ViewKind::Fragment` path.

Generated named slots use a distinct transparent logical node and a generic
`SetSlot { parent, slot, child }` command. `NavigationView` currently exposes typed `Content` and
`Header` slots. Components, fragments, providers, context, and effects pass through named slots
without another ownership graph. Each slot accepts zero or one flattened native root and validates
arity before native apply. Mount, independent update, replacement, clear, and retirement contain
no `NavigationView` branch in Pump code.

Component-owned keyed children build one key index and one desired order. Small keyed edits use
insert and move commands. Updates with 256 or more ordering operations use child synchronization,
which bounds repeated vector search and movement for dense and adversarial sparse reorders.

`ItemsRepeater::item` accepts a key and any `Into<View>`. `ItemsRepeater::items` accepts
`IntoIterator<Item = KeyedView>`. Rows stay as keyed desired views until native realization asks
for one, so unrealized components are not created and their effects do not run.

Each realization entry stores the row's logical ownership root and an optional native attachment
root. The logical root may be a component, provider, fragment, or native node. Mount and key-stable
updates use the ordinary `View` planner. Exactly one flattened root attaches to the WinUI shell.
Zero roots leave the shell empty. Multiple roots also leave it empty and commit
`PumpDiagnostic::VirtualRowRootCount`; `Pump::drain_diagnostics` exposes committed diagnostics and
the application host writes them as non-fatal warnings. Returning to one root reattaches the same
logical row without recreating its component or effects. References remain bound to published
native descendants while detached. Invalid root count is a committed row shape, not pending work,
so the Pump does not retry it without another source or component update.

Recycling and source replacement retire the logical subtree child-first. `DetachRealized` targets
only a present attachment, while every unattached native descendant is destroyed through ordinary
logical subtree retirement. Realization keeps the existing generation, container, key, and
work-budget checks and does not increment the Pump version. Diagnostics are part of the update
plan and become visible only after native apply and candidate publication succeed.

`RealizedContainer` identifies one native shell lifetime, not a reusable physical control. WinUI
assigns a new monotonically increasing token whenever it checks out a shell, removes that token
before pooling the cleared control, and resolves attachment only through the live token map. Each
virtual source also has a `u64` revision. Key changes increment it in the candidate, publish it with
the reset command, and make callbacks from an older source revision stale. Key-stable payload
updates preserve the revision and realized rows. A queued recycle supersedes an earlier unprocessed
realization for the same lifetime token. Valid realization resolves its row by index and verifies
that the indexed key matches the lease key before composition.

The virtual task editor in `crates/samples/reactor-next/virtual` is the integrated qualification
slice. Durable task data remains in the parent model because WinUI may recycle a realized row and a
key-changing source reset intentionally retires all current row scopes. A key-stable payload update
reuses the row component. The recording-runtime test edits a row, reverses the source, re-realizes
the same key, and verifies data survival plus one-time effect cleanup and setup.

## Scheduling and lifecycle

Each dispatcher turn handles at most 64 native events, 64 component messages, and 32 realization
requests, then drains accepted imperative requests against the published tree. Remaining work
rearms the scheduler. Work queued during dispatch is not lost, and dispatcher rejection is an
explicit host fault.

Changed and retired effect cleanup runs child-first before native mutation. New setup runs
parent-first after publication. Normal shutdown cleans effects before native reset, and cleanup is
idempotent across shutdown and `Drop`. Within one component, changed and removed cleanup follows
the published key order, setup follows the new registration order, and final cleanup reverses the
current published order.

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

Startup Pumps are registered as in-flight before `mount`, and `LiveHost` counts pending opens.
A close requested by `Component::create` is therefore retained until mount returns, and closing an
early startup window cannot exit while another startup Pump still needs to mount.

This is ownership and lifecycle isolation, not native fault isolation. An unexpected native
failure in any Pump follows the process-fatal policy above.

The navigation sample in `crates/samples/reactor-next/navigation` qualifies two startup windows.
Each window owns its page model, controlled editor, typed reference, queue, and background work.
A shared application coordinator broadcasts theme changes by sending an ordinary local message to
each registered window. Closing one window retires its effects and tasks, removes its sender, and
notifies the remaining window without sharing Pump state.

`App::run_windows` currently fixes the complete window set at startup. Runtime creation and
declarative title or size configuration are not public contracts. Component-requested close uses
the transactional `WindowRef` contract above.

## Generation

`tool_reactor_next` reads `crates/tools/reactor-next/src/winui.toml` and generates the public typed
controls, minimal bindings filter, handle variants, property operations, structural roles, and
event payload conversion, reference binding, and focus capability declarations. Generated files
must not be edited by hand.

```powershell
cargo run -p tool_reactor_next --quiet
cargo check -p windows-reactor-next --quiet
```

## Current evidence

The component frontend measured about 0.5 us, 430 allocated bytes, and 9 allocations for an
isolated leaf at both 512 and 16,384 unrelated scopes. Idle storage was about 2,440 bytes per scope.
A standalone thin counter compiled clean in about 5.1 seconds, compared with 12.7-13.0 seconds for
the equivalent current-reactor counter. A source-only rebuild took 0.55 seconds, compared with 3.0
seconds. The release executables were 856 KB and 2.99 MB. These ratios are provisional because the
current crate carries much broader generated control coverage. Repeat the comparison after broad
reactor-next generation.

Adding NumberBox left a source-only thin-counter rebuild within measurement noise
(1.228 -> 1.237 seconds) and did not enlarge `Node`, `MountedProps`, or `Element`. The release
counter grew from 828,928 to 847,360 bytes (+18,432 bytes, +2.22%), mostly in executable code.
Generated backend dispatch remains reachable through runtime IDs even when the application does
not construct NumberBox. Measure a representative control batch before deciding whether generated
controls need feature partitioning.

Slider reused the NumberBox range contract without new generator or runtime behavior. Relative to
the corrected NumberBox baseline, median source-only rebuild time changed from 1.290 to 1.330
seconds (+3.1%). The thin release counter grew from 847,360 to 858,624 bytes (+11,264 bytes,
+1.33%), including 8,416 bytes of executable code and 3,008 bytes of read-only data. Core layouts
did not change.

The NavigationView multi-slot slice added the shared named-slot tree and command protocol plus one
generated control. A seven-sample source-only thin-counter rebuild changed from a 0.530-second
median to 0.493 seconds, which is measurement noise rather than a compile regression. The native
thin release counter grew from 886,784 to 905,216 bytes (+18,432 bytes, +2.08%). PE virtual sizes
grew by 17,136 bytes in `.text`, 880 bytes in `.rdata`, 792 bytes in `.pdata`, and 28 bytes in
`.reloc`. `Node`, `MountedProps`, and `Element` remain 416, 72, and 80 bytes.

ProgressBar added seven ordinary generated properties without new runtime or generator behavior.
The source-only thin-counter rebuild median changed from 0.471 to 0.503 seconds (+7.0%, 33 ms).
The native thin release counter grew from 905,216 to 916,992 bytes (+11,776 bytes, +1.30%).
PE virtual sizes grew by 8,768 bytes in `.text`, 2,928 bytes in `.rdata`, 288 bytes in `.pdata`,
and 84 bytes in `.reloc`. Core layouts did not change.

An attempted Viewbox slice was rejected by the live gate because Viewbox owns a typed `Child`
property and does not implement `IContentControl`. Schema resolution now proves `content` and
`children` roles against their metadata interfaces, so this mismatch fails generation instead of
reaching native apply.

ToggleSwitch adds a controlled boolean property and a boolean event payload through schema alone.
The live gate changes `IsOn` in both directions, confirms the native read-back, and verifies that
programmatic `Toggled` feedback does not escape to the application callback. The source-only
thin-counter rebuild median changed from 0.466 to 0.469 seconds (+0.5%). The native thin release
counter grew from 916,992 to 929,792 bytes (+12,800 bytes, +1.40%). PE virtual sizes grew by 9,152
bytes in `.text`, 3,112 bytes in `.rdata`, 468 bytes in `.pdata`, and 48 bytes in `.reloc`. Core
layouts did not change.

Removing fine-grained recovery reduced `core/pump/publish.rs` from 396 lines to 57 and removed
per-command outcome vectors, divergent properties, retries, remount recovery, recovery
continuations, and their specialized tests.

Component-owned keyed updates scale near-linearly from 512 to 4,096 children. The current release
measurements are about 0.42 ms/4.09 ms for same-order parent recomposition and 0.49 ms/5.76 ms for
reversal. Rotate, insert, and remove remain in the same range at 4,096 children. Reorders moving
10%, 20%, and 25% of 4,096 keys take about 5.0-5.4 ms and use child synchronization.

An isolated context-provider update measured about 3.8 us at 512 unrelated scopes and 4.3 us at
16,384. A provider spanning 16,384 descendants with one consumer remained about 3.7 us. Updating
one of 16,384 independent providers measured about 4.7 us, compared with 4.1 us at 512. A provider
with 16,384 actual consumers remains linear in the work requested.

A complete background task, including OS-thread creation, result enqueue, and UI dispatch, measured
about 67 us and 825 allocated bytes at both 512 and 16,384 unrelated scopes. Live tasks are bounded
separately from queued completions. Idle component storage is about 2,440 bytes per scope.

The `test` feature exposes the recording runtime and Pump to the headless test and benchmark
packages. `test_reactor_next_selftest` exercises two real WinUI windows in a process-isolated
fixture. It edits one window at a time, verifies window-specific callback payloads, closes the
secondary with a background task in flight, verifies that its completion is discarded, and
continues background delivery, updates, and final effect cleanup in the surviving window. An
explicit completion marker prevents an early `App::run_windows` return from passing.

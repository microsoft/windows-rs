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
    window: WindowToken,
    scope_id,
}

NativeToken {
    window: WindowToken,
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

## Component prototype contract

### Published ownership and scope states

The published `Tree` owns component parent, key, type, and sibling order. A component boundary is a
logical `NodeKind::Component` node with one `ScopeId`. The scope arena does not store parent, key,
or sibling links.

```text
ScopeSlot {
    generation,
    entry: None | ScopeEntry,
}

ScopeEntry<C> {
    state: Reserved | Published | Retiring,
    component: C,
    props: C::Props,
    desired: View,
    effects,
}
```

| Scope state | Structural relation | Message handling |
| --- | --- | --- |
| Reserved | Candidate boundary only | Envelopes may queue but cannot execute |
| Published | Exactly one published boundary | Valid envelopes execute |
| Retiring | Old published boundary until apply begins | New envelopes are rejected |
| Vacant slot | None | Tokens fail generation validation |

Planning first validates keys, types, slot cardinality, and scope reservations. It does not run
cleanup. After a valid plan, retiring scopes run cleanup child-first while their native resources
still exist. Native apply and immediate recovery then publish the candidate relation or fault the
host. Failed reservations are retired without setup. There is no rollback to an old component
state or old desired view after component code has accepted props or a message.

The arena retains a vacant slot's generation and increments it before reuse. A stale token
therefore cannot address a later scope in the same slot. Any invalid key, type, cardinality, or
reservation invalidates the entire candidate, drops every reservation in that plan, and faults the
window. Phase 4 does not publish a valid subset of an invalid candidate.

The candidate tree may clone `ScopeId` values but never scope entries. Existing published scopes
remain in the arena while the candidate is planned. New scopes live in a reservation list owned by
that plan. Publication changes their state to `Published`; abandoning the plan drops them and all
queued reserved envelopes.

### Component and message erasure

The bounded public contract is:

```text
trait Component: 'static {
    type Props: Clone + PartialEq + 'static;
    type Message: 'static;

    fn create(props: &Self::Props, context: &mut ComponentContext<Self>) -> Self;
    fn changed(&mut self, props: &Self::Props, context: &mut ComponentContext<Self>);
    fn update(&mut self, message: Self::Message, context: &mut ComponentContext<Self>);
    fn view(&self, context: &mut ViewContext<Self>) -> View;
}
```

`ScopeEntry<C>` is held behind an object-safe internal trait. Props and message envelopes carry
their concrete `TypeId`; internal dispatch functions downcast both the scope and payload. A failed
downcast is a framework invariant fault, not an ignored message. No unsafe cast is needed.

`LocalSender<C::Message>` contains a `ComponentToken` and the window-local queue. `send` only
enqueues. Validation checks the full `WindowToken`, scope generation, published state, and message
type before dispatch. A sender for a reserved scope may enqueue, but its envelopes remain blocked
until publication and are discarded if reservation fails. Background senders are outside Phase 4.

Props are parent input, not messages. Equal props do nothing. New props replace the stored desired
props and call `changed` once before that child's next `view`. If a parent update discovers new
props while child messages are already queued, props are applied first; surviving child messages
then observe the new props. Multiple parent renders before child composition coalesce to the last
props.

### View shape and logical anchoring

`View` has four forms:

```text
Empty
Native(Element)
Component { key, factory, props }
Fragment(Vec<KeyedView>)
```

Component and fragment nodes are logical and never create a WinUI object. A component boundary owns
one logical slot containing its current `View`.

The initial prototype uses explicit native slot adapters rather than changing every generated
builder:

```text
View::content(control, child)
View::children(control, keyed_children)
View::virtual_items(control, keyed_items)
```

The control argument must have the matching generated role and must not already contain structural
children. This keeps ordinary `Element` APIs unchanged during the A/B test. If the component model
passes, the generator may accept `Into<View>` directly.

Fragments splice into a native children slot. Empty and single-root views are valid in any slot.
A content slot and the window root accept at most one realized native root after transparent
component and fragment expansion. A multi-root fragment in either location is a planning error.
Component-only chains and pass-through components therefore add no native control.

Child identity is `(parent component boundary, key, component TypeId)`. An unkeyed child uses its
stable ordinal among unkeyed component children in that logical slot. Same key and type retains the
scope across props and keyed movement. Same key with a different type retires the old scope and
reserves a new generation.

### Drain, composition, and publication phases

One window turn runs these phases:

1. Drain native observations and validate their native tokens.
2. Apply coalesced parent props to published scopes, parent-first.
3. Drain validated component messages by global enqueue sequence.
4. Compose dirty scopes parent-first; retirement discovered by a parent prevents later child work.
   When a component view has no matching published scope, call `create`, add its scope to the
   plan's reservation list, and call its first `view` depth-first in this phase.
5. Validate the already-built logical candidate and every reservation without running user code.
6. Run retiring cleanup and old cleanup for changed effects child-first while old native resources
   still exist, apply native commands, recover if needed, and publish.
7. Run new and changed effect setup parent-first after publication.

Messages sent from `create`, `changed`, `update`, `view`, callbacks, or effects append to the queue.
They never execute in the current borrow. A turn has separate message and composition budgets.
Message-budget exhaustion leaves envelopes queued. Composition-budget exhaustion preserves the
candidate, reservations, retirement set, and a traversal cursor. The next turn resumes that plan
before processing later props or messages; native observations may queue but cannot mutate the
paused plan. This prevents partial-plan publication and guarantees that a tree larger than one
tick's budget can finish. Either exhaustion rearms the dispatcher. Panic or an invariant fault
discards the paused plan, clears the active-borrow marker, faults only that window host, runs
cleanup, and rejects later tokens.

Component state and desired views commit when their lifecycle call returns. Structural publication
still follows native structural success or successful immediate remount. Property failure leaves
the new desired view and published relation in place with per-property divergence. Effects from a
failed, unpublished candidate do not run.

### Window ownership and native access

Each logical window owns one scope arena, component queue, scheduler, structural tree, and native
runtime under the same `WindowToken`. No process-global scope or message queue participates in
dispatch. Native remount advances only `NativeIdentity`; component senders remain valid. Window
close advances `WindowToken`, retires every scope, and rejects all old senders.

Effect setup and cleanup receive validated window/component access, not an unowned raw handle.
Cleanup runs while the retiring published relation and required native handles remain queryable.
After cleanup, subscriptions are revoked and native nodes are destroyed. A window fault cannot
drain, retire, or report success for another window.

## Work sequence

### 1. Repair the shared backend

- [x] Add separate window-lifetime and native-realization identity domains.
- [x] Reject stale events, callbacks, dispatcher work, and repeater leases after the corresponding
  epoch changes.
- [x] Move startup/window mounting to `OnLaunched`.
- [x] Install `XamlControlsResources`.
- [x] Replace scheduled booleans with an explicit pending/scheduled/dispatching/closing scheduler.
- [x] Preserve queued work across reentrancy, priority escalation, and enqueue rejection.
- [x] Bound native event and realization work per dispatcher turn and rearm remaining work.
- [x] Run cleanup before revoking subscriptions and releasing required native resources.
- [x] Clear recycled repeater shell content synchronously.
- [x] Add property certainty, per-property retry accounting, and controlled-observation tracking.
- [x] Add schema-validated feedback contracts.
- [x] Retain and honor event payload source, interface, and conversion in generation.

**Exit:** verified backend behavior is independent of the frontend model.

### 2. Add the live backend harness

- [x] Add a process-isolated live WinUI fixture runner.
- [x] Verify `OnLaunched`, control resources, and first native view commit.
- [x] Queue old work, remount native content, and reject only old native work.
- [ ] Close/recreate a window and reject all old window work.
- [x] Exercise dispatcher reentrancy and enqueue rejection.
- [x] Exercise mutate-then-fail setters and rejected controlled edits.
- [x] Reject delayed/coalesced feedback until those contracts are implemented.
- [ ] Exercise repeater recycle and immediate shell reuse.
- [x] Verify cleanup runs once before native reset.
- [ ] Verify two windows cannot consume each other's work or faults.

`RecordingRuntime` remains useful for deterministic planning and failure positions. It cannot prove
template behavior, COM reentrancy, partial native mutation, shell visuals, or shutdown ordering.

**Exit:** the repaired backend passes headless and live WinUI tests before frontend comparison.

### 3. Write the component ownership specification

- [x] Define the one authoritative parent-child relation.
- [x] Define scope reservation, publication, retirement, and failed-candidate cleanup.
- [x] Define candidate interaction with non-cloneable component state.
- [x] Define `Component`, `View`, props, keys, logical anchors, and type replacement.
- [x] Define safe erased storage and typed local message envelopes.
- [x] Define queue phases, borrowing, ordering, budgets, and panic policy.
- [x] Define effect ordering and native-resource access.
- [x] Define per-window ownership and fault containment.

**Exit:** the four blocking contracts are explicit and do not depend on implementation convention.

### 4. Implement the bounded component prototype

- [x] Add stable generational scope storage.
- [x] Add logical component boundaries to the structural tree.
- [x] Mount component-only chains ending in one native root through the shared command path.
- [x] Add nested components with props and local typed messages.
- [x] Keep leaf recomposition within the component boundary.
- [x] Retain child state across parent prop changes and keyed movement.
- [x] Replace same-key/different-type children and retire the old scope.
- [x] Queue reentrant messages without directly reborrowing a component.
- [x] Retain current application state and desired view across native failure.
- [x] Run cleanup exactly once in the required order.

Do not add context, background senders, or general async ownership in this phase.

**Exit:** nested ownership, identity, replacement, failure retry, and cleanup pass headlessly and
against live WinUI.

### 5. Compare hooks and components

| Scenario | Result |
| --- | --- |
| Counter | Hook and component binaries compile from equivalent generated controls |
| Controlled rejection | Both restore the desired `TextBox.Text` value |
| Nested ownership | Pass-through and native-owning component tests pass |
| Keyed changes | Insert, move, removal, type replacement, and dense reorder pass |
| Anchoring | Empty, single-root, multi-root children, and component-only views pass |
| Reentrancy | Queue-only sends, a 65-message live burst, and fixed-capacity backpressure pass |
| Repeated lifecycle | Scope, callback, effect, repeater, and shutdown tests return to zero |
| Virtual collection | Component-owned repeater shells recycle and immediately realize new rows |
| Two windows | Separate pumps reject cross-window tokens; the live host still owns one window |

The component counter uses 42 nonblank source lines versus 19 for hooks. Most of the difference is
the explicit state type and `create`/`changed`/`update`/`view` methods. The component surface gives
props, messages, effects, and ownership separate compiler-checked boundaries, but it is not the
shorter frontend for a small root.

The release benchmark used 256 latency samples for p50/p95/p99 and disabled command history in the
recording runtime:

| Frontend and operation | N | p50 | p95 | p99 | Bytes/op | Allocs/op |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Hooks, forced no-change | 512 | 25.5 us | 29.3 us | 35.8 us | 49,122 | 515 |
| Components, forced no-change | 512 | 0.2 us | 0.3 us | 0.3 us | 100 | 3 |
| Hooks, isolated leaf | 512 | 30.9 us | 34.5 us | 40.8 us | 62,605 | 530 |
| Components, isolated leaf | 512 | 0.6 us | 0.9 us | 1.1 us | 476 | 11 |
| Hooks, isolated leaf | 4,096 | 288.4 us | 396.3 us | 560.7 us | 495,785 | 4,114 |
| Components, isolated leaf | 4,096 | 0.6 us | 0.7 us | 0.7 us | 476 | 11 |
| Hooks, isolated leaf | 16,384 | 1.67 ms | 3.02 ms | 4.47 ms | 1,989,017 | 16,402 |
| Components, isolated leaf | 16,384 | 0.6 us | 0.7 us | 0.7 us | 476 | 11 |

The component isolated-leaf path retained constant allocation and latency through 16,384 unrelated
leaf scopes. Including the root, idle retained memory was 1,271,876 bytes at 513 scopes, 10,156,612
bytes at 4,097, and 40,618,564 bytes at 16,385 - about 2,479 bytes per scope at every size.

Dense keyed reversal initially exposed quadratic vector movement. Reorders above the dense-change
threshold now issue one `ResetChildren` followed by ordered attachment, while sparse changes keep
minimal moves. The 512-to-4,096 reversal grew from 0.17 ms to 1.86 ms for an 8x input increase and
the live WinUI fixture exercises the reset transaction.

Two isolated compile trials built the same counter controls:

| Metric | Hooks | Components | Component/hook |
| --- | ---: | ---: | ---: |
| Clean release, trial 1 | 5.54 s | 4.91 s | 0.89x |
| Clean release, trial 2 | 5.54 s | 5.03 s | 0.91x |
| Incremental release, trial 1 | 2.71 s | 2.43 s | 0.90x |
| Incremental release, trial 2 | 2.59 s | 2.03 s | 0.78x |
| Release executable | 706,560 bytes | 686,592 bytes | 0.97x |

The current `windows-reactor` headless baseline measured 0.54 us and 457 bytes for one dirty
component. The new component path measured about 0.60 us and 476 bytes, so local update cost remains
close to the established crate rather than paying the root-hook cost. The current crate remains
faster for a 512-item dense keyed reversal (0.10 ms versus 0.17 ms).

**Decision:** continue with the owned-component architecture and retain hooks during the remaining
gate work. Components provide a clear locality advantage without compile-time or executable-size
regressions, and they stay close to the current crate's local component baseline. Do not start
context or async ownership yet. Empty/multi-root anchoring, bounded native recovery, and a live
multi-window host remain blockers rather than being hidden by the favorable leaf benchmark.

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

Phase 5 results:

| Metric | Result |
| --- | --- |
| Clean and incremental compile time | Pass - worst component ratio 0.91x |
| Release executable size | Pass - component ratio 0.97x |
| No-change median and p95 | Pass - component ratios 0.01x |
| Isolated-leaf median and p95 | Pass - component ratios <= 0.02x at 512 |
| Isolated leaf, 512 -> 16K | Pass - constant latency, bytes, and allocations |
| Keyed siblings, 512 -> 4K | Pass - 8x input produced about 11x time after dense reset |
| Repeated mount and retirement | Pass headlessly; more live resource cycles remain |
| Message burst | Pass - 4,096-message queue capacity and observable `false` backpressure |
| Native recovery | Pass - RECOVERY_COMMAND_BUDGET = 64; 11 headless tests cover budget, multi-turn, effects, stale work, poisoning, event deferral |
| Idle scope memory | Reported - about 2,479 bytes per scope |
| Compile diagnostics | Pass - props/messages are typed and `LocalSender` is not `Send` |

Report update CPU p95 and p99 separately from end-to-end frame latency. Set the absolute CPU budget
from current-reactor measurements on the same live scenarios; a full 16.7 ms frame is not an
acceptable update CPU budget.

The locality gate excludes declared dependencies such as changed parent structure or consumed
context.

## Current work

Current phase: **Phase 5 complete - close semantic and host blockers before Phase 6**

The August follow-up review found two accepted event schemas that could generate invalid Rust.
Observation generation now walks controlled properties rather than every payload event, wrapper
fields use minimal-projection Rust names, unsupported enum and multi-field reads are rejected,
accepted primitive/wrapper expressions and the secondary-interface event arm are compiler-checked,
native drains are budgeted, and the live executable has a hard timeout. Feedback events are unique
per controlled property until one event can return multiple observations. A delayed native edit
now proves generated COM `TextChanged` delivery without manually injecting the observation.

- [x] Implement separate window and realization identity domains.
- [x] Add stale-work tests for native remount and complete window replacement.
- [x] Complete scheduler priority escalation and stale callback handling.
- [x] Mount from `OnLaunched` and install WinUI control resources.
- [x] Run effect cleanup before native reset.
- [x] Clear recycled repeater shells synchronously.
- [x] Complete desired/known/divergent state with per-property attempts.
- [x] Restore rejected controlled edits through the live render loop with or without callbacks.
- [x] Require an explicit supported feedback contract for controlled properties.
- [x] Honor typed event payload source interfaces and conversions in generated code.
- [x] Add the first live startup/resources fixture.
- [x] Verify callback-free controlled repair through the live scheduler and WinUI property.
- [x] Add live tests for scheduler rejection/reentrancy, stale remount, and shutdown ordering.

Phase 2 still tracks live window recreation, repeater shell visuals, OS input delivery, and
two-window isolation. These require host or automation surfaces beyond the bounded one-window
component slice; they remain continuation gates rather than being treated as passing evidence.
Phase 4 is complete. The component frontend stores `ScopeId` and component type on logical
component nodes. A separate window-token-bound store owns non-cloneable component state, checked
typed props, and FIFO local message envelopes. The public `Component` and `View` types can reserve a
component-only chain, expand it to one native root in the authoritative candidate tree, run the
normal native command batch, and publish all scopes only after structural success. Same-type prop
updates retain parent and child scopes. Typed local messages are drained before local candidate
reconciliation, and multiple messages for one component coalesce into one view pass.
`View::Content` and `View::Children` mount logical component descendants under a native parent.
Same-key children retain scopes across prop updates and movement. Insert/remove and type
replacement use one scope transaction. Failed structural updates preserve the application and
window, advance realization identity, and remount the desired candidate without recreating
component state. Virtual collections work below components but remain excluded from this recovery
path until leases can be rebound to the new realization identity. Control expansion remains frozen.
`App::run_component` hosts a component root through the same live scheduler as the hook frontend.
Local sends wake normal-priority work only when the queue changes from empty to nonempty. Each turn
drains at most 64 messages and rearms when work remains. Each window accepts at most 4,096 queued
messages, and `LocalSender::send` returns `false` when the queue is full or its owner is stale.
Retired scopes, replaced windows, and shutdown close their queue gates.

Dirty scopes use a derived `ScopeId` index, set-based coalescing, and parent-first composition.
Unchanged child props suppress duplicate child composition. A property-only update to one native
leaf avoids cloning the candidate tree. Phase 5 measured about 0.6 us and 476 bytes per isolated
leaf update through 16,384 unrelated components. Structural changes retain the full candidate and
recovery path. A component host also treats pending property repair as schedulable work and
recomposes mounted scopes when no message is available to trigger the retry.

`ViewContext::use_effect` records dependency-indexed setup and cleanup work. Changed and retired
cleanups run child-first before native mutation. Setups run parent-first after scope and tree
publication, never after failed recovery, and cleanup is idempotent across shutdown and drop.
Headless failure-injection tests and the live WinUI fixture cover setup, dependency changes,
recovery, and shutdown counts.

Phase 5 found a clear reason to continue: owned components preserve local work while root hooks
scale with unrelated tree size, and the component binary did not regress compilation or size.

Empty and multi-root anchoring now uses logical fragment nodes rather than hidden native panels.
Fragments splice zero or more native roots into generated children collections. Window and content
slots validate a zero-or-one-root result before publication. Exact-order publication is coalesced
to one `SynchronizeChildren` command per native parent, while ordinary one-root keyed children keep
sparse insert and move plans. Headless tests cover empty transitions, retained keyed component
scopes, rejected multi-root slots, and structural recovery to the desired fragment order. The live
WinUI fixture exercises empty window content and a retained two-root reorder.
An isolated leaf inside a fragment measured 0.6 us, 476 bytes, and 11 allocations at both 512 and
16,384 scopes, so logical flattening does not reintroduce unrelated-tree work on the local path.

An independent viability review identified a risk of repeating an older design mistake: allowing
optimized component paths to acquire separate publication and lifecycle semantics before the
general model is settled. Treat the following as a blocking semantic gate:

- [x] Parent props discovered by a dirty ancestor apply before queued child messages that survive
  that ancestor's composition.
- [x] A local-path probe that falls back does not call user `view` twice.
- [x] Successful structural recovery retires the dirty work represented by the recovered
  candidate.
- [x] Effect cleanup and setup order match one documented lifecycle contract.
- [x] Local and general component plans use one native publication, receipt interpretation,
  recovery, and lifecycle engine.

The semantic gate passes without another ownership graph. Queued descendants are deferred when a
dirty ancestor precedes them. Ancestor composition applies props first, retirement drops obsolete
messages, and surviving descendants compose once. Full-tree and property-local candidates feed one
publisher for native apply, receipts, property certainty, effects, retry state, and recovery.

The pump review confirmed that the command/receipt protocol is useful but the central coordinator
had accumulated too many engines. `core/pump/plan.rs` owns candidate and plan types, and
`core/pump/publish.rs` owns the shared publisher and recovery policy. This is a transactional
decomposition, not a file-only split.

Component turns are now isolated in `core/pump/turn.rs`, including ancestor-first props and
descendant-message deferral. Effect and scope lifecycle work is in `core/pump/lifecycle.rs`.
Validated events and realization work are in `core/pump/native_work.rs`. The local candidate
continues to meet the 0.6 us, 476-byte, and 11-allocation gate because it carries only desired
props; receipt-derived certainty publishes after native apply.

`core/pump/mod.rs` is now a 539-line phase coordinator. Runtime-independent planning is split into
`planner/topology.rs`, `planner/element.rs`, and `planner/view.rs`; no planning module can apply
native commands, interpret receipts, schedule work, run effects, or mutate published pump state.
Every production module is below the 1,000-line review trigger. The former inline pump tests are
grouped by behavioral contract under `core/pump/tests/`. Pump decomposition therefore passes.
Budgeted structural recovery and live multi-window ownership must pass before context or
background async work begins.

# Windows Reactor Replacement Comparison

## Executive summary

`windows-reactor-next` should continue as the preferred replacement candidate.

Its main advantage is not its current feature count. It has a much simpler answer to the hardest
reliability question: **what state is authoritative?**

- Current `windows-reactor` has several representations that must remain synchronized.
- Reactor2 made ownership more explicit, but replaced hidden complexity with a large handwritten
  runtime and native command protocol.
- Reactor-next has one authoritative logical tree, plans changes before touching WinUI, publishes
  them only after native success, and treats unexpected native failure as terminal rather than
  attempting fine-grained repair.

The current candidate is not ready to replace `windows-reactor`. It still lacks broad control
coverage, async facilities, mature collection/template evidence, devtools, and final compile-time
qualification. Its architecture is nevertheless better suited to becoming the replacement.

## Comparison

| | Current reactor | Reactor2 | Reactor-next |
| --- | --- | --- | --- |
| Main strength | Broad and practical | Explicit identity and ownership | Small, auditable correctness model |
| Authoritative state | Logical tree, mounted tree, and backend tables | More centralized, but still a large runtime protocol | One generational tree |
| Native updates | Mutates live WinUI during reconciliation | Command-oriented, but logical state can advance before native completion | Plans candidate, applies commands, then publishes |
| Events | More callback-stack coupling | Better queued behavior | Native callbacks only enqueue |
| Failure policy | Mixture of warnings, results, and panics | Complex lifecycle plus terminal native failures | First native failure poisons and aborts |
| Identity | Keys plus backend identities | Generational state and references | Generational nodes, scopes, subscriptions, leases, and windows |
| Handwritten size | About 19.6K Rust lines | About 37.3K Rust lines | Smaller fixed core with generated semantics |
| Main risk | Synchronized-state divergence | Runtime and compile-time complexity | Narrow coverage and incomplete qualification |

## Problems in current windows-reactor

Current reactor is the practical implementation and has useful coverage, but its reliability model
depends on several structures agreeing after every operation:

- Logical tree.
- Mounted tree.
- Native backend tables.
- Template and recycling bookkeeping.
- Thread-local open-window registry.

Native mutation occurs during reconciliation. Application callbacks can also be closely coupled to
native callback stacks. When a failure occurs after part of an update, cleanup and backend
bookkeeping must reconstruct a coherent interpretation of what happened.

This works often enough to build applications, but it is difficult to prove correct. Adding more
features tends to add more synchronized backend state and more paths that must preserve every
representation.

Useful ideas worth retaining:

- Retained native controls.
- Stable keys.
- Generated property and event mappings.
- Explicit effect cleanup.
- Practical support for templates, collections, and multiple windows.

## What Reactor2 improved

Reactor2 moved in the right direction in several areas:

- Generational identity.
- Explicit window and element references.
- Queued events.
- Controlled observations.
- Virtualized row ownership.
- Stronger component and state lifetimes.
- More declarative multi-window ownership.

The problem was the cost of expressing all of this through a large handwritten runtime. The
handwritten Rust surface grew to roughly 37.3K lines, compared with about 19.6K in current reactor.

It became easier to identify each piece of state, but harder to understand the complete state
machine. Compile time and implementation complexity grew. Native failure could still occur after a
prefix of commands, while application state and native state did not share one simple publication
boundary.

Reactor2 showed that explicit identity and queued work are necessary. It also showed that a
successor cannot afford to hand-author a parallel protocol for every control and feature.

## Why reactor-next is better

### One authoritative structural tree

Reactor-next puts structural identity, parentage, order, desired properties, subscriptions,
components, fragments, providers, and virtual collections under one generational tree.

Native handles remain private to the backend. There is no second backend ownership graph that must
independently reproduce the frontend structure.

This removes a major class of failures:

```text
frontend thinks A
mounted tree thinks B
backend side table thinks C
```

### Candidate publication

The normal update sequence is:

```text
render components
-> construct and validate candidate
-> prepare old effect cleanup
-> apply native commands
-> publish candidate
-> run new effect setup
```

Planning errors do not mutate native state and do not publish the candidate. Failed native apply
poisons the pump, prevents publication, and terminates the production host.

This is simpler than trying to roll back or repair an arbitrary prefix of WinUI mutations.

### Reentrancy is prevented

Native callbacks enqueue typed work and return. User code does not render while WinUI is still in
the native callback.

Queued work carries:

- Window epoch.
- Generational node identity.
- Subscription revision.
- Component scope generation.
- Realization lease identity.

Stale work from an old node, old subscription, recycled row, closed window, or previous host epoch
is rejected before reaching application code.

### Rust-owned components

The primary model is:

```text
owned component state
+ typed props
+ typed messages
+ explicit view
+ explicit effects
```

Non-cloneable state stays in the component store. Retiring a scope destroys its state and effects.
Generational senders make stale component messages harmless.

This is more naturally suited to Rust than recreating a managed object graph through pervasive
runtime typing and shared interior mutability.

### Generated control semantics

The intended scaling model is:

```text
one schema row
-> control type
-> properties and builders
-> native setters and clearing
-> events and payloads
-> structural capabilities
-> controlled feedback contracts
-> recording-runtime descriptions
-> tests
```

The core planner should not acquire another handwritten match table whenever a control is added.

### Simpler failure handling

An earlier reactor-next prototype accumulated:

- Command receipts.
- Per-property attempts.
- Divergence tracking.
- Retries.
- Structural remount epochs.
- Recovery continuations.

That machinery was removed. The current rule is:

```text
unexpected native failure
-> record failing command
-> poison pump
-> abort production host
```

This is intentionally less ambitious and much easier to audit.

## Comparison with Microsoft.UI.Reactor

The comparison uses `D:\git\microsoft-ui-reactor`, currently a broad C# WinUI Reactor preview.

The C# implementation is far ahead in feature coverage:

- Full WinUI control gallery.
- Templates and virtualization.
- Async resources and mutations.
- Error boundaries.
- Hot reload.
- Devtools and ETW tracing.
- Control pooling.
- Roslyn generators and analyzers.
- Third-party control extension.
- Focus, navigation, accessibility helpers, layout, and interop.

It is the best source of evidence about WinUI control-specific behavior.

### Ideas reactor-next has already borrowed

- `Inherited` properties that call `ClearValue`.
- Controlled property feedback.
- Subscribe-once event handlers.
- LIS-based keyed reconciliation.
- Two-phase effect cleanup and setup.
- Fragments.
- Typed contexts.
- Work budgets.
- Schema-driven control generation.

### Why the C# implementation should not be copied literally

C# Reactor benefits from CLR facilities:

- GC for short-lived element records.
- Runtime type dictionaries and interface dispatch.
- Reflection.
- Exceptions and error boundaries.
- Metadata hot reload.
- Ambient `Task`, `CancellationToken`, and dispatcher support.
- Attached dependency properties for state tied to native COM identity.

Rust does not receive these facilities cheaply. Reactor-next instead uses:

- A generational arena instead of GC-owned element records.
- Static types and monomorphic dispatch instead of runtime registries.
- Explicit messages instead of ambient cross-thread mutation.
- Offline generation instead of build-time reflection.
- Stored node identity instead of attached managed state.
- Fatal native failure instead of arbitrary exception recovery.

Hot reload is the clearest area where the C# implementation has an inherent platform advantage.

## Scaling to more controls

### Straightforward controls

Ordinary properties, events, simple content controls, and layout panels are a good fit for the
generator.

Adding these should increase generated code roughly linearly without making the core planner more
complicated.

### Structural controls

Controls with multiple content areas require generated structural roles:

- Header.
- Content.
- Menu items.
- Footer.
- Pane.
- Children collection.

These should be expressed as capabilities and slots, not new control-specific branches in the
pump.

### Controlled and coercing properties

Text, toggle, and basic selection fit the current desired/known-native model.

Controls such as Slider, NumberBox, CalendarView, and RangeSelector are harder:

- WinUI may coerce the written value.
- One write may raise several events.
- Events may not be synchronous or exact.
- A control may have several controlled properties.

The generator needs explicit feedback contract categories rather than assuming every controlled
property behaves like TextBox.

### Templates and virtualization

This is the highest-risk expansion area.

The C# implementation has already discovered edge cases involving:

- Recycled containers that must remain parented.
- Finding a compatible recycled container instead of blindly popping one.
- Preserving `ReadLocalValue` so style-derived defaults survive recycling.
- Selection animation flicker.
- Incremental collection moves and container animations.
- Reorder-stable row keys.

Reactor-next's generational realization leases are a good safety foundation, but live WinUI testing
must reproduce these scenarios before virtualization is considered mature.

### Async work

The component ownership model is a good basis for async work:

```text
component scope owns task
-> completion sends typed message
-> retirement cancels task
-> stale completion is rejected by scope generation
```

The missing work is an owner-scoped cancellation and scheduling service comparable to the C#
implementation's resources, mutations, and pending scopes.

### Third-party controls

The curated schema is suitable for first-party support but not sufficient for application-specific
or third-party controls.

An extension mechanism must allow a crate to define:

- Native creation.
- Properties and clearing.
- Events.
- Structural roles.
- Controlled feedback.
- Teardown.

It must not require modifying the reactor core, but should also avoid turning the entire runtime
into `Box<dyn Any>` and runtime type lookups.

## Current risks

### Context invalidation

Context publication is correct, but current invalidation is broader than its benchmark suggests.

A provider update scans its complete subtree, consumers walk ancestor chains, and mutating one
provider clones a global provider map. Broad or deeply nested provider trees can approach O(n^2).

A reverse dependency index is the likely fix:

```text
(provider identity, context key) -> consuming component scopes
```

### Unbounded structural composition

Scheduler queues are budgeted, but rendering one very large component subtree remains synchronous
and unbounded.

The project must either:

- Add a continuation model; or
- State that individual component renders must stay within a practical size.

### Multi-window failure policy

Windows have separate pumps, queues, schedulers, trees, components, and effects. The live
asymmetric two-window test passes.

They are not independent native failure domains:

```text
one unexpected WinUI apply failure
-> abort entire process
```

This may be acceptable initially, but it must be an explicit product decision. A later
window-rebuild or supervisor model could improve resilience without restoring per-command recovery.

### Compile time

Early measurements show promising check time, build time, and executable size. They are not a fair
final comparison because reactor-next has only a handful of controls while the C# implementation
has a broad gallery.

Offline generation and avoiding proc macros on the critical path are good choices. Compile-time and
binary-size measurements must be repeated after generated coverage grows substantially.

### Missing product facilities

Reactor-next still lacks or has limited versions of:

- Broad control coverage.
- Async resource ownership.
- Navigation.
- Devtools.
- Hot reload.
- Focus and accessibility helpers.
- Mature templates and collection behavior.
- Third-party control extension.
- Animation.
- Native control pooling.

## Why this candidate is worth pursuing

The three Rust designs move complexity in different directions.

Current reactor becomes more fragile as features add more synchronized backend state.

Reactor2 became larger as features added more handwritten protocol machinery.

Reactor-next aims for:

```text
generated control semantics
+ typed application components
+ one authoritative tree
+ a small fixed reconciliation core
```

That is the right scaling direction.

Recent development also shows that feedback is changing the architecture rather than producing
patches around it:

- Receipt and recovery machinery was removed.
- Pump responsibilities were separated.
- Lifecycle ordering was corrected.
- Failed-planning retry publication was corrected.
- Keyed behavior was measured and improved.
- Multi-window ownership was made explicit and tested live.
- Context state was integrated into candidate publication rather than becoming a second authority.
- Full-tree component effects were corrected and regression-tested.

At the current tip, no known publication or lifecycle correctness blocker remains.

## Recommended expansion plan

Do not add dozens of easy controls first. Test the architecture with difficult vertical slices:

1. Add one coercing controlled control such as Slider or NumberBox.
2. Add one complex multi-slot control such as NavigationView or TabView.
3. Qualify ItemsRepeater templates and recycling using the C# edge-case history.
4. Add owner-scoped async work and cancellation.
5. Fix context invalidation scaling.
6. Add a third-party control extension model.
7. Measure clean and incremental compile time after substantial generated growth.
8. Decide whether process-fatal multi-window behavior is acceptable.

The success criterion should be:

> Hard controls and services fit through generated contracts and small isolated backend
> extensions.

The failure criterion should be:

> Each new control requires new pump state, handwritten reconciliation paths, or another
> authoritative side table.

If the failure pattern appears, stop before recreating Reactor2.

## Recommendation

**Continue investing in reactor-next and treat it as the preferred replacement architecture. Do
not declare it the replacement yet.**

It has learned the important lessons from both Rust predecessors:

- Current reactor showed which practical WinUI features are needed.
- Reactor2 showed the value of explicit identity and the cost of a large handwritten protocol.
- The C# implementation shows the control-specific edge cases and higher-level facilities that a
  mature framework needs.

Reactor-next combines those lessons in a model better suited to Rust: one authoritative tree,
candidate publication, typed components and messages, generational stale-work rejection, generated
semantics, and a deliberately small fatal native-error policy.

The next decision should be based on difficult feature expansion, live WinUI evidence, and
compile-time measurements at scale. Current evidence favors pursuing it.

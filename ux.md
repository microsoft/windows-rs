# Windows Reactor Next - UX and Flexibility Assessment

## Executive summary

`windows-reactor-next` is still primarily a backend and correctness project. Its developer UX is
functional but noticeably more explicit and verbose than the incumbent. The visual application UX
should not materially differ when both expose the same WinUI properties, but the programming model
changes substantially.

The rewrite has selected an owned-component and typed-message model rather than preserving the
incumbent's render-function and hook model. That is a meaningful product decision, not merely an
implementation detail.

The incumbent is substantially more approachable and flexible today. Reactor-next's current
advantage is reliability, auditability, and clearer ownership rather than API polish or feature
breadth.

Developer UX should become the next qualification gate. This does not mean polishing every helper
or recreating the incumbent API now. It means settling the public composition, identity, event,
effect, and imperative-access contracts before broad control generation makes them expensive to
change. Convenience constructors, derives, and broad styling helpers should follow evidence from
real application slices.

## Developer experience

| Area | `windows-reactor` | `windows-reactor-next` |
| --- | --- | --- |
| State model | Render functions and hooks | Owned component structs and typed messages |
| Small component | Very concise | Considerably more boilerplate |
| State transitions | Setter and reducer handles | `Message` enum plus `update` |
| Composition | Helpers, tuples, implicit conversion | Explicit `View` and `KeyedView` |
| Effects | Rich hook API | `ViewContext::use_effect` |
| Async | State, resources, mutations, marshalling | Owned thread returning a message |
| Imperative access | Typed element references and handles | Not present in the public slice |
| Control coverage | Dozens of practical controls and behaviors | Eleven generated controls |
| Styling and interaction | Broad modifier and behavior coverage | Narrow current slice |

The incumbent counter is close to a React-style Rust API:

```rust
fn app(cx: &mut RenderCx) -> Element {
    let (count, set_count) = cx.use_state(0);

    vstack((
        text_block(format!("count = {count}")),
        button("Click").on_click(move || set_count.call(count + 1)),
    ))
    .into()
}
```

The equivalent reactor-next application requires a component type, state field, message type,
three required trait methods, a sender, and explicit keyed views:

```rust
struct Counter {
    count: u32,
}

impl Component for Counter {
    type Props = ();
    type Message = ();

    fn create(_: &(), _: &mut ComponentContext<Self>) -> Self {
        Self { count: 0 }
    }

    fn update(&mut self, _: (), _: &mut ComponentContext<Self>) {
        self.count += 1;
    }

    fn view(&self, _: &Self::Props, cx: &mut ViewContext<Self>) -> View {
        let sender = cx.sender();

        View::children(
            StackPanel::new(),
            [
                KeyedView::new(
                    "value",
                    View::native(TextBlock::new().text(self.count.to_string())),
                ),
                KeyedView::new(
                    "button",
                    View::native(
                        Button::new()
                            .on_click(move || {
                                sender.send(());
                            })
                            .content(TextBlock::new().text("+")),
                    ),
                ),
            ],
        )
    }
}
```

For a toy application, the incumbent is clearly nicer. For a larger stateful component,
reactor-next's explicit state machine may become an advantage: all mutable state lives in the
component, all changes pass through typed messages, and effects and background work have explicit
scope ownership.

## Intentionally different UX

Some of the difference is architectural and should not be hidden behind syntax.

### Owned state instead of hooks

Reactor-next intentionally removed its hook frontend. Arbitrary, non-cloneable Rust state belongs
directly to a stable component instance. This eliminates hook ordering, hook-slot type erasure,
pervasive interior mutability, and a second effect engine.

That is a material public API decision. Reintroducing incumbent-style hooks would give up part of
the simplification the rewrite was created to obtain.

### Typed, queue-only messages

Native callbacks and component senders enqueue messages rather than invoking component code or
rendering reentrantly. `update` is the normal mutation point.

This provides:

- explicit event ordering;
- stale-message rejection by scope generation;
- no late delivery to removed components;
- one path for local and background results;
- component behavior that can be tested as a state machine.

It also requires message enums and forwarding closures that the hook API avoids.

### Explicit identity

`View::children` accepts `KeyedView`, making retained identity visible at the call site. That is
safer than silently assigning positional identity, but it is noisy for static layouts. Requiring a
string key for every child in a two-item `StackPanel` is likely prototype UX rather than a necessary
final requirement.

### Positional effects remain

Owned component fields remove state-hook ordering, but `ViewContext::use_effect` still assigns
effect identity by call position. Conditional insertion or reordering can therefore replace a
different effect slot. This is a smaller constraint than a complete hook runtime, but it is still
a hook-order rule and should be documented or replaced before the API settles.

## Current composition split

Reactor-next currently has two similar-looking composition paths with different capabilities:

| Path | Accepted descendants | Purpose |
| --- | --- | --- |
| Generated structural methods | Native `Element` only | Compact native-only trees |
| `View` structural methods | General `View` | All logical descendants |

For example, `Button::content(TextBlock::new())` works, but putting a component in the same content
position requires changing to `View::content(Button::new(), View::component::<C>(props))`.
`StackPanel::children` and `View::children` have the same distinction. The names do not explain the
semantic boundary, and extracting a native subtree into a component changes its construction form.

This is the highest-priority UX issue because it affects generated APIs and the mental model.
Reactor-next should expose one obvious composition model. A native-only fast path may remain
internally or as an advanced API, but ordinary application code should not need to choose between
two tree languages.

## UX that can improve without changing the architecture

Several current rough edges can be addressed by a thin frontend layer:

- `View::children(control, [KeyedView::new(...)])`.
- No `vstack`, `text_block`, or `button` convenience constructors.
- No tuple or array child conversion comparable to `IntoChildren`.
- Repetitive empty `create` and `update` methods.
- Event closures that manually discard the `bool` from `sender.send`.
- No component derive or macro for common cases.
- No concise child-component syntax.

A frontend could make the same component model more concise:

```rust
vstack((
    keyed("value", text(self.count)),
    keyed(
        "increment",
        button("+").on_click(sender.message(Message::Increment)),
    ),
))
```

This would remain syntax over `View`, `KeyedView`, and typed senders rather than creating another
ownership or state model.

A component derive or macro could reduce common `create` boilerplate. The main constraint is to
avoid turning convenience APIs into a second reconciliation frontend.

The first iteration should prefer ordinary traits, conversions, and methods over procedural
macros. Macros can hide poor underlying types and produce worse diagnostics. Add them only after
the non-macro API is proven by realistic applications.

### Initial form baseline

The first form slice is
`crates/samples/reactor-next/form/src/main.rs`. It includes controlled text and numeric input,
validation, disabled state, progress, background submission, and an extracted summary component.
The initial public-API baseline is:

| Measure | Current result |
| --- | ---: |
| Source lines | 176 |
| Explicit child keys | 7 |
| Sender handles | 3 |
| Event forwarding closures | 3 |
| Empty component lifecycle methods | 1 |

The first attempted version also required six `View::native(...)` calls because Rust does not chain
the generated control-to-`Element` and `Element`-to-`View` conversions. Generated controls now
convert directly to `View`. This removes ceremony without adding a frontend or changing the tree:
the conversion calls the existing `View::native`.

The component store now borrows its authoritative current props into `view`. The read-only summary
therefore remains a unit struct and needs no duplicate props field or `changed` synchronization.
All seven children are static but require keys. Three events repeat sender capture, message
construction, send, and ignored-result handling. The summary still requires `create` and an empty
`update`. Focus cannot be expressed, so the form cannot implement focus-first-invalid or
post-submit focus behavior yet.

## Current flexibility

Structurally, reactor-next already has a useful foundation:

- owned nested components with typed props;
- keyed movement and replacement;
- empty and multi-root fragments;
- transparent typed context providers;
- single-content and multi-child controls;
- controlled inputs;
- effects with cleanup;
- scope-owned background work;
- virtualized `ItemsRepeater` rows;
- multiple independent windows.

Practical application flexibility remains far behind the incumbent. Reactor-next currently
generates only:

- `TextBlock`;
- `Button`;
- `StackPanel`;
- `TextBox`;
- `NumberBox`;
- `Slider`;
- `NavigationView`;
- `ProgressBar`;
- `ToggleSwitch`;
- `ScrollViewer`;
- `ItemsRepeater`.

The incumbent already has broad controls plus templates, navigation, dialogs, flyouts, menus,
images, grids, rich text, trees, tabs, styling resources, animations, pointer handling, drag/drop,
focus through typed element references, accessibility modifiers, Canvas bridges, and richer async
hooks.

An existing nontrivial `windows-reactor` application cannot be ported mechanically today. It would
encounter missing controls and facilities long before backend compatibility became the main issue.

## Application UX

For end users, no visual change is inherent. Both implementations create retained native WinUI
controls. A Button should look, measure, focus, and interact like a WinUI Button.

Reactor-next can provide observable behavioral improvements:

- programmatic controlled-property echoes do not spuriously call application handlers;
- native callbacks cannot trigger reentrant rendering;
- stale callbacks from removed controls or closed windows are rejected;
- keyed movement has clearer retained identity;
- failed candidate planning cannot partially publish logical state.

But reactor-next can currently deliver a worse application experience because important facilities
are missing: focus APIs, templates, richer layout, animation, navigation, accessibility
conveniences, and many control-specific behaviors.

## How much of the work has been backend-focused?

Most of it.

The work so far has concentrated on:

- one authoritative structural tree;
- transactional candidate publication;
- native command ordering;
- fatal unexpected-native-failure policy;
- generational node, scope, event, lease, and window identity;
- queue-only event and component dispatch;
- deterministic effect cleanup and setup;
- controlled-property feedback contracts;
- keyed reconciliation and virtualization;
- context locality;
- scope-owned background work;
- multi-window lifecycle isolation;
- metadata-driven control generation;
- compile-time, allocation, locality, and binary-size measurements.

The current API exists mainly to exercise those invariants. It has not yet received the same
product-level UX iteration as the incumbent.

The authoritative plan identifies API consolidation and replacement qualification as the next
phase, which is appropriate. Broad generation should not freeze the current surface before that
work happens.

## What to decide now and what to defer

UX work should start now, but it should be split into contract work and polish.

| Decide now | Defer until application evidence |
| --- | --- |
| One composition model for all view kinds | Convenience constructor names |
| Positional versus explicit effect identity | Component derive and procedural macros |
| Static positional children versus explicit keyed children | Full styling and modifier vocabulary |
| Sender adapters and observable send rejection | Async resource and mutation conveniences |
| Imperative references and mount lifetime | Broad migration shims |
| Public error and unsupported-control diagnostics | Optional control feature groups |

The "now" work changes foundational types or generated signatures. The deferred work can be added
without changing reconciliation or ownership.

## What should remain and what should change

The following should remain:

- owned component state;
- typed props and messages;
- queue-only sends;
- explicit component identity;
- one effect engine;
- one `View` reconciliation frontend;
- generated property and event semantics.

The following should be treated as open UX work:

- child and content construction syntax;
- key requirements for static versus dynamic children;
- effect identity and conditional-effect rules;
- component boilerplate;
- sender-to-message adapters;
- imperative focus and native-handle access;
- async resource and mutation helpers;
- window and host context;
- layout and styling convenience traits;
- diagnostics when a control or property is unsupported;
- migration aids from the incumbent.

## Recommended UX qualification

Start with one realistic form before broad control generation:

1. A form with controlled text, numeric input, validation, focus, and async submission.

This slice is small enough to finish but exercises the unresolved contracts: nested composition,
controlled values, messages, effects, focus, background work, disabled and error states, and
component extraction. Build it first with the current API and retain that version as the baseline.
Then make the smallest API changes that materially improve it.

Measure:

- application source size and boilerplate;
- number of explicit keys;
- message and forwarding closure count;
- ease of extracting nested components;
- imperative escape-hatch quality;
- async cancellation behavior;
- compile and edit/rebuild time;
- diagnostics for unsupported shapes;
- reliability differences found during the port.

The initial UX gate should settle:

1. How much explicit keying normal layouts require.
2. Which child and content conversions form the one normal composition path.
3. Whether effect identity remains positional.
4. How sender-to-message adapters expose queue rejection.
5. How imperative focus and native handles fit the ownership model.
6. Whether the owned typed-message model remains acceptable in a realistic component.

After this gate, qualify broader facilities with:

1. A navigation application with multiple windows and retained page state.
2. A virtualized collection with row components, selection, templates, and background loading.
3. A Canvas or WebView application requiring imperative handles and subsystem failure handling.
4. An animation and pointer-interaction sample.

Those later slices should drive async helpers, window context, templates, specialized adapters, and
styling APIs. They should not block fixing the foundational composition and identity contracts.

## Final assessment

The rewrite has not preserved the incumbent developer UX unchanged. It has selected a more
explicit Elm/Yew-like component model in exchange for stronger ownership and scheduling
semantics. That choice should be evaluated with users.

The current verbosity is not an unavoidable consequence of the architecture. Much of it can be
removed with an ergonomic layer while retaining owned components and typed messages.

**The architecture can support a better UX than it currently presents, but the incumbent remains
substantially more approachable and flexible today. Reactor-next's present advantage is reliability
and auditability, not user-facing API polish or feature breadth. Begin UX qualification now, before
broad control generation, but focus first on foundational contracts rather than convenience
syntax.**

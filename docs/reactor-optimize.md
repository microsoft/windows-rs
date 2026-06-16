# Reactor Build Optimization Analysis

Profiled on 2026-06-16. Goal: identify remaining opportunities to streamline
`windows-reactor` without splitting into multiple crates.

## Profile Summary

| Area | Lines | % of crate |
|------|------:|:----------:|
| `bindings.rs` | 23,051 | 56% |
| `backend/winui/` | 5,141 | 13% |
| `widgets/` | 3,894 | 10% |
| `reconciler/` | 1,652 | 4% |
| `engine.rs` | 1,358 | 3% |
| Other (`element`, `interaction`, etc.) | ~5,879 | 14% |
| **Total** | **40,975** | |

Clean build: ~10.9s (dependencies dominate; reactor itself compiles fast once
`windows-core` is done). Incremental no-op: 0.18s.

---

## Current Code Generation

`tool_reactor` already generates three files:

| File | Lines | Arms |
|------|------:|-----:|
| `generated.rs` (widget bindings fns) | 909 | — |
| `generated_set_prop.rs` | 525 | 136 |
| `generated_attach_event.rs` | 325 | 26 |

Hand-written dispatch remaining:

| Location | Arms |
|----------|-----:|
| `try_universal_prop` (lines 738–960) | 53 |
| Residual `set_prop` match (lines 1087–1656) | 67 |
| Residual `attach_event` match (lines 2074–2462) | 25 |

---

## Opportunities

### 1. Generate `try_universal_prop` from TOML

The 53-arm `try_universal_prop` function handles FrameworkElement/UIElement/IControl
property setters (`Width`, `Height`, `Margin`, `Opacity`, `HorizontalAlignment`,
attached Grid/Canvas/RelativePanel props, font props, shape props, etc.).

Every arm follows one of these patterns:
- `(Prop::X, PropValue::T(v)) → handle.as_framework_element().put_X(v)`
- `(Prop::X, PropValue::Unset) → handle.as_framework_element().put_X(default)`
- `(Prop::X, PropValue::T(v)) → handle.cast_inner::<IFoo>()?.put_X(v)`
- Multi-try font/foreground/background helpers that cascade through interfaces

These map cleanly to a schema: `(prop, interface, method, value_type, default)`.
Adding a `[universal_props]` section to `winui.toml` would let `tool_reactor`
generate this entire function (~225 lines).

### 2. Generate `classify_container`

Lines 360–512 classify Handle variants into container shapes (Panel,
SingleChild, ContentControl, DirectContent, InspectableVector). The mapping is:

- **Panel**: StackPanel, Grid, Canvas, RelativePanel
- **SingleChild**: Border, Viewbox
- **ContentControl**: ScrollViewer, Expander, TabViewItem, NavigationView, PivotItem
- **DirectContent**: ScrollView, SplitView
- **InspectableVector**: TabView, Pivot

This is entirely schema-expressible. A `container_kind` field in the TOML
widget entries would let the tool generate `classify_container`,
`put_single_child`, and `put_direct_content` (~150 lines).

### 3. Generate the `define_handles!` Input List

The `define_handles!` macro (lines 17–125) already generates the enum, casts,
constructors, and `describe_kind`. But its **input** — the list of 57 variant
names — is hand-maintained. The tool could emit this list from the TOML manifest,
eliminating a place to forget when adding a new widget.

### 4. Generate `style_target_for_handle`

Lines 559–568: a small match returning `(&'static str, IFrameworkElement)` for
handles that support theme styles. Purely schema-driven.

### 5. Move More `set_prop` Arms to Codegen

Of the 67 remaining hand-written `set_prop` arms, most involve compound logic
(Button icon+text layout, NavigationView menu items, ContentDialog modal popup,
etc.) that resists simple codegen. However, ~10–15 arms follow simple patterns:

- `CornerRadius` on Border / Rectangle
- `BorderBrush`, `BorderThickness` on Border
- `IsClosable` on TabViewItem
- `DisplayMode` on SplitView
- `Step` on Slider
- `NavigateUri` on HyperlinkButton

These could be expressed in the TOML with a `method` + optional cast.

### 6. Move ~2,700 Lines of Tests to `test_reactor`

Tests embedded inside `windows-reactor` today:

| File | ~Test Lines |
|------|------------:|
| `element.rs` | 1,128 |
| `interaction.rs` | 656 |
| `widget.rs` | 517 |
| `reconciler/child.rs` | 207 |
| `reconciler.rs` | 186 |
| `backend/mod.rs` | 38 |
| `widgets/image.rs` | 32 |
| **Total** | **~2,764** |

Moving these to a dedicated `test_reactor` crate would:
- Remove ~2,700 lines from `cargo check -p windows-reactor` parsing
- Decouple test infrastructure from the library
- Centralize all reactor unit tests in one place

**Caveat:** `backend/mod.rs:1387` explicitly pokes the private
`RunOnDemandDispatcher::inner` field. Moving it requires either a
`pub(crate)` escape hatch or a test-only helper API.

---

## Non-Opportunities (Already Optimized)

- **`engine.rs` generics**: Heavy generic hook machinery (`ComponentCell<C, P>`)
  is already type-erased behind `dyn ComponentObject`, bounding monomorphization.
- **`reconciler/widget_dispatch.rs`**: Generic mount/update plumbing, not
  per-widget dispatch — nothing to generate here.
- **`reconciler/child.rs`**: Generic keyed/positional reconciliation algorithm,
  not widget-specific.
- **Crate splitting**: Not considered per constraint. The crate is already fast
  enough that splitting would add dependency-graph complexity without meaningful
  wall-clock savings.

---

## Recommendations (Priority Order)

1. ~~**Generate `define_handles!` input list**~~ ✅ Done — `tool_reactor` now
   emits `generated_handles.rs` from the TOML manifest. 10 handle-only controls
   added with `_control = { handle_only = true }`.

2. ~~**Move tests to `test_reactor`**~~ ✅ Done — ~2,700 lines moved from 6
   source files to dedicated test files in `crates/tests/libs/reactor/tests/`.
   Only `backend/mod.rs` dispatcher test kept (accesses private field).

3. **Generate `try_universal_prop`** — biggest remaining win: ~225 lines of
   repetitive set/unset pairs. Requires a schema addition to declare universal
   props and their interface/method/default mappings.

4. **Generate `classify_container`** — ~50 lines, low churn. Would require a
   `container` key in the TOML for ~14 controls. Low ROI relative to schema
   complexity added.

5. **Generate 10–15 more simple `set_prop` arms** — incremental improvement,
   reduces the residual hand-written dispatch.

6. **Generate `style_target_for_handle`** — small but free if the tool already
   knows widget metadata.

---

## Expected Impact

These changes won't dramatically cut wall-clock build time (reactor is already
~10.9s clean, dominated by upstream crates). The value is:

- **Less code to maintain**: ~600 fewer hand-written lines in the backend
- **Fewer bugs when adding widgets**: schema-driven generation means one place
  to declare a new widget instead of 5–6
- **Faster `cargo check`**: moving tests out reduces work for the type checker
- **Better parallelism potential**: smaller compilation units finish sooner,
  unblocking downstream crates earlier in the dependency graph

# Reactor Backend Redesign — Learnings

## Problem Statement

The WinUI backend (`crates/libs/reactor/src/winui/backend/mod.rs`) uses a
triple-indirection dispatch pattern to apply properties from widgets to COM
controls:

1. Each widget's `bindings()` method allocates a `Vec<Binding>` of
   `Binding::Prop(Prop, PropValue)` and `Binding::Event(Event, Option<EventHandler>)`.
2. The reconciler iterates bindings and calls `set_prop(id, prop, value)` for
   each one.
3. `set_prop` is a large match on `(Handle, Prop)` that dispatches to COM setter calls.

## What Was Done — Option A: Build-time Code Generation

After evaluating three approaches (typed handlers, `widget_map!` macros, proc macros),
**Option A** — a standalone codegen tool — was implemented successfully via `tool_reactor`.

See [reactor-codegen.md](reactor-codegen.md) for the tool architecture and
[reactor-metadata.md](reactor-metadata.md) for the metadata-driven TOML design.

### Results

- **52 controls** fully declared in a single 291-line TOML file
- **~2300 lines** of generated dispatch code (bindings, set_prop, attach_event)
- **Widget naming aligned to WinUI** — no more "Widget" suffix, metadata-derived field names
- **Prop/Event/PropValue enums use metadata names** — no synthetic overrides
- **Match arm collapsing** — duplicate set_prop arms merged via wildcards and OR-patterns,
  eliminating `#[allow(clippy::match_same_arms)]`
- **Type inference from metadata** — value types, copy semantics, enum variants all
  resolved from `.winmd` files instead of hardcoded lists

### Failed Approaches (Historical)

Three earlier attempts informed the final design:

1. **Typed Handlers (PR #4527)** — generated `element_set_prop` but never wired into
   the reconciler. ~400 lines of dead code. Lesson: new paths must *replace* old ones.

2. **`widget_map!` Declarative DSL** — hit `macro_rules!` hygiene wall. Captured
   expressions can't see variables introduced by the macro. Only proc macros could
   solve this.

3. **`widget_map!` Explicit Blocks** — compiled and worked, but LOC went UP +176 net.
   Per-control mount+diff blocks roughly double the per-control code.

## COM API Gotchas

Discovered during migration and apply to any future approach:

- `put_IsChecked` on CheckBox takes `Option<bool>` (tri-state nullable boolean)
- TextBox/PasswordBox need get-before-set to avoid cursor position reset
- `string_as_textblock()` creates a TextBlock COM object from `&str`
- ProgressBar uses `IRangeBase` for Value/Min/Max; ProgressRing has direct setters
- ContentDialog needs XamlRoot from a live element (requires backend access)
- Font properties are shared across `IControl`/`ITextBlock`/`IRichTextBlock`

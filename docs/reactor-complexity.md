# Reactor Backend Redesign — Learnings & Future Direction

## Problem Statement

The WinUI backend (`crates/libs/reactor/src/winui/backend/mod.rs`) uses a
triple-indirection dispatch pattern to apply properties from widgets to COM
controls:

1. Each widget's `bindings()` method allocates a `Vec<Binding>` of
   `Binding::Prop(Prop, PropValue)` and `Binding::Event(Event, Option<EventHandler>)`.
2. The reconciler iterates bindings and calls `set_prop(id, prop, value)` for
   each one.
3. `set_prop` is a ~1,750-line match on `(Handle, Prop)` that dispatches to
   COM setter calls.

This architecture is easy to extend (add a new prop → add a variant to `Prop`,
a variant to `PropValue`, a match arm in `set_prop`, and a binding entry) but
carries costs:

- **Allocation**: every render allocates a Vec per widget.
- **Indirection**: every property goes through two enum matches and a clone.
- **Code size**: `set_prop` is ~1,750 lines; `Prop` has ~100 variants; many are
  used by only one or two controls.
- **No diffing at the source**: the reconciler diffs `Vec<Binding>` element-wise.
  If a widget reorders its bindings between renders, the diff silently breaks.

## Failed Approaches

### Attempt 1: Typed Handlers (PR #4527, `reactor2` branch)

**Idea**: Generate a per-control `element_set_prop` that takes typed arguments
directly (e.g., `f64` for FontSize, `&str` for Text) instead of going through
`PropValue`.

**What happened**: The typed handlers were generated but `element_set_prop` was
never actually *called* by the reconciler — the old `set_prop` path remained the
active code path. The PR had ~400 lines of dead code. Event handling was not
addressed. LOC went up ~82%.

**Lesson**: A new dispatch path must *replace* the old one, not exist alongside
it. If the old path still handles everything, nothing is gained.

### Attempt 2: `widget_map!` Macro with Declarative DSL (first `reactor3` attempt)

**Idea**: A `macro_rules!` macro that declares per-control property mappings
in a declarative DSL:

```rust
widget_map! {
    TextBlock => ITextBlock2 {
        text: val Text => put_Text;
        font_size: opt FontSize => put_FontSize;
        is_text_selection_enabled: flag IsTextSelectionEnabled => put_IsTextSelectionEnabled;
    }
}
```

The macro would generate both `mount` (set all) and `diff` (set if changed)
code, eliminating hand-written mount/diff blocks entirely.

**What happened**: Hit a fundamental `macro_rules!` hygiene wall. Captured
`$expr:expr` fragments resolve identifiers at the **call-site** context, not at
the expansion site. Variables introduced by the macro expansion (`let h = ...`,
`let v = ...`) are in a different hygiene context and are invisible to the
captured expressions.

```rust
// The macro expands to:
let h: ITextBlock2 = handle.cast()?;
let v = &widget.font_size;  // introduced by macro
$expr  // ← captured at call site, can't see `h` or `v`
```

This is a fundamental property of Rust `macro_rules!` — switching to
`$($tt:tt)*` does NOT help because tokens still keep their original context.
Only a proc macro could solve this (by rewriting identifier spans), but proc
macros add significant compile-time cost.

**Lesson**: `macro_rules!` cannot provide a truly declarative DSL that
introduces bindings visible to user-supplied expressions. The abstraction
boundary is at the token level, not the semantic level.

### Attempt 3: `widget_map!` with Explicit Mount/Diff Blocks (`reactor3` branch)

**Idea**: Abandon the declarative DSL. Instead, have the user write explicit
mount and diff blocks. The macro only handles dispatch (downcast → call the
right block).

```rust
widget_map! {
    TextBlock(ITextBlock2) mount(w, handle) {
        handle.put_Text(&w.text)?;
        if let Some(v) = w.font_size { handle.put_FontSize(v)?; }
    } diff(old, w, handle) {
        changed!(old, w, text, { handle.put_Text(&w.text)?; });
    }
}
```

**What happened**: This compiled and worked. 20 controls were migrated. All
tests passed. But:

- **LOC went UP by +176 net** (not down). Each control needs ~28 lines of
  hand-written mount+diff in `widget_maps.rs`, replacing ~20 lines of
  `set_prop` arms + ~10 lines from simplified `bindings()`.
- The macro is essentially just a `if let Some(w) = any.downcast_ref::<T>()`
  dispatcher — trivially simple.
- **31 controls still remained** on the legacy path (not 8 as initially thought).
- Converting all 51 controls would be a massive effort with no net LOC reduction.

**Lesson**: Any approach that requires per-control mount AND diff blocks will
roughly *double* the per-control code (one block to set everything, one block to
set only what changed). The savings from removing `Prop`/`PropValue` enums don't
materialize until *every* control is migrated, and even then the improvement is
modest.

## Key Technical Insights

### The Binding System Is Already Doing What Typed Dispatch Does

The `bindings() → set_prop()` architecture has a critical property: the
`bindings()` method acts as a *serializer* that converts typed widget fields
into a uniform `(Prop, PropValue)` representation, and `set_prop()` acts as a
*deserializer* that pattern-matches back into typed COM calls.

Any typed-dispatch approach essentially inlines this serialize/deserialize round
trip. The net effect is moving code from one file to another — it doesn't
disappear.

### The Real Cost Centers

1. **`Prop` enum (100+ variants)**: Many variants are used by only 1-2 controls.
   But consolidating them requires knowing which controls share which props,
   which is hard to express statically.

2. **`PropValue` enum**: Used to type-erase all property values into a single
   enum. This forces `clone()` for every value, even when diffing could avoid
   it.

3. **Per-widget `bindings()` allocation**: A `Vec` allocation per widget per
   render. Could be reduced with `SmallVec` or arena allocation, but the COM
   call overhead likely dwarfs the allocation cost.

4. **Linear scan in `find_prop()`**: The reconciler scans the bindings list
   linearly. For widgets with many props this is O(n²) over all props. In
   practice n is small (≤20) so this doesn't matter.

### COM API Gotchas

These were discovered during migration and will apply to any future approach:

- `put_IsChecked` on CheckBox takes `Option<bool>` (tri-state nullable boolean)
- TextBox/PasswordBox need get-before-set to avoid cursor position reset
- `string_as_textblock()` creates a TextBlock COM object from `&str`
- ProgressBar uses `IRangeBase` for Value/Min/Max; ProgressRing has direct setters
- ContentDialog needs XamlRoot from a live element (requires backend access)
- Font properties are shared across `IControl`/`ITextBlock`/`IRichTextBlock`

## What Would Actually Work

### Option A: Build-time Code Generation (Recommended)

A `build.rs` or standalone tool reads a declarative description file (similar to
the existing `tool_bindings` pattern in this repo) and generates:

- Per-control mount/diff functions with direct COM calls
- Simplified `bindings()` that returns only event bindings (or nothing)
- Updated `Prop`/`PropValue` enums with only the variants still needed

**Pros**:
- Full control over generated code — no hygiene issues
- Declarative source of truth (one place describes each control's properties)
- Can generate both mount and diff from a single description
- Can cross-reference to detect shared properties and generate helpers
- No proc-macro compile-time cost (runs once at build time)

**Cons**:
- Another build tool to maintain
- Generated code is harder to debug (but can be formatted and checked in)
- Requires careful handling of edge cases (cursor preservation, tri-state bools)
  — likely needs escape-hatch annotations

### Option B: Incremental Improvements to Current Architecture

Rather than replacing the architecture, reduce its costs:

1. **Eliminate `PropValue` clones in diffing**: Change the reconciler to diff
   `&PropValue` instead of `PropValue`, avoiding clones for unchanged props.
2. **Use `SmallVec<[Binding; N]>`**: Avoid heap allocation for small widgets.
3. **Order-independent diffing**: Use a prop-keyed approach instead of
   positional diffing to avoid silent bugs from reordered bindings.
4. **Split `set_prop` by control type**: Instead of one giant match, have
   per-Handle-variant functions. This doesn't reduce total code but improves
   readability and compile times.

**Pros**:
- Low risk, incremental, each change is independently valuable
- No new tooling to maintain
- Preserves the existing architecture that works

**Cons**:
- Doesn't address the fundamental code duplication
- `set_prop` remains large
- `Prop`/`PropValue` enums remain large

### Option C: Proc Macro (`#[derive(Widget)]`)

A derive macro on each widget struct that generates `bindings()`, mount, and
diff code from field attributes:

```rust
#[derive(Widget)]
#[widget(handle = "ITextBlock2")]
struct TextBlock {
    #[prop(setter = "put_Text")]
    text: String,
    #[prop(setter = "put_FontSize", optional)]
    font_size: Option<f64>,
}
```

**Pros**:
- Most ergonomic — the struct IS the source of truth
- Can handle hygiene properly (proc macros rewrite spans)
- Familiar pattern to Rust developers

**Cons**:
- Proc macros add significant compile time (~2-5s per invocation)
- Complex to implement (must handle 10+ property kinds, events, edge cases)
- Debugging generated code requires `cargo expand`
- Would be a new crate (`windows-reactor-macros`) in the workspace

## Recommendation

**Start with Option B** (incremental improvements) to get immediate wins with
low risk. **Evaluate Option A** (build-time codegen) when the reactor
stabilizes and the control set is more complete. Option A is the only approach
that can achieve the "north star" of a single declarative description per
control without macro hygiene issues or proc-macro compile costs.

Option C (proc macro) is viable but should be deferred unless compile-time
impact is measured and found acceptable.

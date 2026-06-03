# Reactor / Canvas Naming Collisions

## Problem Statement

`windows-reactor` and `windows-canvas` are companion crates designed to be used
together, yet glob-importing both causes compiler errors:

```rust
use windows_canvas::*;
use windows_reactor::*; // ERROR: `Color` is ambiguous
```

The `canvas_circles` sample works around this by selectively importing from
canvas and glob-importing reactor. The `canvas/minimal` sample glob-imports both
but currently avoids using the colliding names from reactor's side. This is
fragile — adding new types to either crate can silently break downstream code.

---

## Colliding Types

| Name | `windows-reactor` | `windows-canvas` | Same? |
|------|-------------------|-------------------|-------|
| `Color` | `u8` ARGB struct (XAML layout colors) | `f32` RGBA struct (D2D render colors) | **Different** — different representations and use cases |
| `Brush` | `enum Brush { Solid(Color) }` (property value for XAML element backgrounds) | `struct Brush(ID2D1SolidColorBrush)` (GPU brush handle) | **Different** — value type vs GPU resource |
| `Ellipse` | WinRT `Microsoft.UI.Xaml.Shapes.Ellipse` class (XAML shape element) | Geometry value `{ center, radius_x, radius_y }` for D2D | **Different** — UI element vs draw primitive |
| `FontWeight` | WinRT `Windows.UI.Text.FontWeight` struct (`{ Value: u16 }`) | Thin wrapper `FontWeight(pub i32)` with NORMAL/BOLD | **Same concept**, different repr |
| `Error` | Re-export of `windows_core::Error` | Re-export of `windows_core::Error` | **Identical** |
| `Result` | Re-export of `windows_core::Result` | Re-export of `windows_core::Result` | **Identical** |

### Near-collisions (same word, different role)

| Name | `windows-reactor` | `windows-canvas` |
|------|-------------------|-------------------|
| `TextAlignment` | Not directly exported (internal to bindings) | `enum TextAlignment { Leading, Center, Trailing }` |
| `Vector2` | Not exported (uses `windows_numerics`) | Re-export of `windows_numerics::Vector2` |
| `Rect` | Not exported | `struct Rect { left, top, right, bottom }` |

---

## Root Cause Analysis

Both crates define **domain-appropriate** types that happen to share common
English names. This is expected — "Color" means something in both a UI
framework and a 2D graphics library. The problem is that both crates export
these names at the crate root with no namespacing.

Reactor exports via a deep chain: `core::geometry` → `core::element` →
`lib.rs` (via `pub use core::element::*` through the `public_submodules!`
macro → `pub use dsl::*`). Canvas exports directly from `lib.rs`.

---

## Design Precedents

### Win2D (C++/WinRT)

Win2D avoids collisions through **WinRT namespace segmentation**:
- `Microsoft.Graphics.Canvas.CanvasDevice`
- `Microsoft.Graphics.Canvas.Geometry.CanvasEllipse`
- `Microsoft.Graphics.Canvas.Text.CanvasTextFormat`

Color is `Windows.UI.Color` — a platform-wide shared type, not redefined per
library. Win2D accepts `Color` from the platform; it doesn't define its own.

### microsoft-ui-reactor (C#/.NET)

The .NET Reactor avoids collisions by:
1. **Namespace segmentation** — `Microsoft.UI.Reactor` vs `Microsoft.UI.Reactor.Charting.D3`
2. **Domain prefixes** — `D3Color`, `ColorScheme`, `TextBlockElement`
3. **Factory pattern** — `using static Microsoft.UI.Reactor.Factories;` gives
   function-style element construction without polluting the type namespace
4. Spec 040 explicitly audits naming alignment across factory/element/folder

### Rust ecosystem patterns

- `bevy`: submodule prelude approach (`bevy::prelude::*` re-exports selected items)
- `wgpu` + `winit`: each crate has distinct type names (`wgpu::Color` vs no color in winit)
- `iced`: single crate, internal domain separation via modules

---

## Recommendations

### Option A: Remove colliding types from reactor's root (Recommended)

**Principle**: Reactor is a UI framework; it should export UI-framework types at
the root. Drawing primitives belong in canvas. Where both need a concept (like
color), one crate should be the canonical owner and the other should either
re-export it or use a qualified name.

**Concrete changes:**

1. **`Color`** — Reactor's `Color` is a `u8` ARGB struct used to set element
   backgrounds/foregrounds. Canvas's `Color` is an `f32` RGBA struct for D2D.
   They serve different purposes and cannot be unified into one type.

   → **Rename reactor's type to `UiColor`** (or keep it internal and accept
   canvas's `Color` via a `From` impl). Reactor's `Color` is only used in
   property bindings (e.g., `.background(Color::rgb(...))`); renaming to
   `UiColor` or making it accept canvas `Color` via conversion is low-friction.

2. **`Brush`** — Reactor's `Brush` is an enum for XAML property values. Canvas's
   `Brush` is a GPU resource handle. Completely different semantics.

   → **Rename reactor's type to `BrushValue`** or **`UiBrush`**. It's a
   property-value enum, not a rendering resource.

3. **`Ellipse`** — Reactor's `Ellipse` is a WinRT XAML Shapes element (like
   `<Ellipse>` in XAML). Canvas's `Ellipse` is a geometry primitive for D2D.

   → **Do not export reactor's `Ellipse` at the crate root.** It lives in
   `bindings.rs` as a WinRT class and is not currently used in any reactor
   sample or DSL function. If needed, access via `windows_reactor::core::...`.

4. **`FontWeight`** — Same concept, different representations. Canvas defines
   `FontWeight(pub i32)` with `NORMAL`/`BOLD`. Reactor has the WinRT struct.

   → **Unify on canvas's definition** (simpler, sufficient). Reactor can accept
   it via `From` in property setters. Or: reactor stops exporting its
   `FontWeight` at the root (it's only relevant for `RichTextRun`).

5. **`Error` / `Result`** — Both re-export `windows_core::{Error, Result}`.
   Rust handles identical re-exports gracefully (no actual ambiguity error).

   → **No change needed.** Rust unifies identical re-exports.

### Option B: Canvas prefixes its types

Prefix all canvas types with `Canvas` (following Win2D's naming):
`CanvasColor`, `CanvasBrush`, `CanvasEllipse`, `CanvasTextFormat`, etc.

**Pros**: Explicit, no ambiguity even without glob imports.
**Cons**: Verbose, especially for types used heavily in draw callbacks.
`canvas::Color::WHITE` becomes `CanvasColor::WHITE`. Hurts ergonomics in the
draw-loop-heavy code canvas is designed for.

### Option C: Submodule preludes

Each crate provides a `prelude` module that exports only non-colliding names.
Colliding types require explicit `use crate::Type` imports.

```rust
use windows_canvas::prelude::*;   // excludes Color, Brush, Ellipse
use windows_reactor::prelude::*;  // excludes Color, Brush, Ellipse
use windows_canvas::{Color, Brush, Ellipse};  // explicit wins
```

**Pros**: Familiar Rust pattern. Both glob imports work.
**Cons**: Requires users to know which names collide. Confusing for newcomers.

---

## Recommendation: Option A

Option A is the best fit because:

1. **Reactor's colliding types are internal/low-usage.** Reactor's `Color` is
   only used in XAML property bindings (background/foreground), not in hot draw
   loops. Its `Brush` enum is a property value. Its `Ellipse` is a WinRT class
   that isn't surfaced in the DSL at all.

2. **Canvas's types are the user-facing ones.** When someone draws circles, they
   reach for `Color`, `Brush`, `Ellipse` from canvas constantly. These names
   should be short and unqualified.

3. **Follows the microsoft-ui-reactor precedent.** The .NET version uses domain
   prefixes for non-primary types (`D3Color`, `ColorScheme`) and keeps the
   unqualified name for the most commonly used variant.

4. **Minimal user-code churn.** The reactor-only samples never use `Color` or
   `Brush` directly (they use themed brushes via `BrushBinding` and
   `ColorScheme`). The canvas samples use `Color` and `Brush` extensively.

---

## Action Plan

| # | Change | Impact |
|---|--------|--------|
| 1 | Rename `core::geometry::Color` → `UiColor` in reactor | Low — used in widget property setters only |
| 2 | Rename `core::geometry::Brush` → `UiBrush` in reactor | Low — enum for property values |
| 3 | Stop exporting `bindings::Ellipse` at reactor root | None — not used in DSL/samples |
| 4 | Stop exporting `bindings::FontWeight` at reactor root | Low — only used in `RichTextRun` |
| 5 | Add `From<windows_canvas::Color>` for `UiColor` (behind `feature = "canvas"`) | Ergonomic bridge |
| 6 | Update `canvas_circles` to use `windows_canvas::*` + `windows_reactor::*` | Proves the fix |
| 7 | Update `canvas/minimal` — confirm both `*` imports work | Regression test |
| 8 | Update `docs/reactor-canvas.md` code examples | Accuracy |

### Migration for existing reactor users

- `Color::rgb(r, g, b)` → `UiColor::rgb(r, g, b)` (search-and-replace)
- `Brush::Solid(c)` → `UiBrush::Solid(c)` (search-and-replace)
- `Ellipse` (the XAML shape) — access via `windows_reactor::core::...` if needed

---

## Alternative Considered: Shared `windows-graphics-types` crate

A shared primitives crate exporting `Color`, `Rect`, `Ellipse`, etc. that both
reactor and canvas depend on. Rejected because:

- Reactor's `Color` (u8 ARGB for XAML) and canvas's `Color` (f32 RGBA for D2D)
  are genuinely different types with different optimal representations.
- A shared crate would need both variants, making it confusing.
- The platform (WinRT) defines `Windows.UI.Color` as u8 ARGB. Canvas
  intentionally departs from this for ergonomic float arithmetic in shaders/draw code.

---

## Open Questions

1. **Naming**: `UiColor` vs `LayoutColor` vs `XamlColor`? The prefix should
   signal "this is for UI element properties, not for drawing."

2. **Should reactor re-export canvas's `Color`?** If reactor adds
   `pub use windows_canvas::Color` (behind a feature flag), users get one
   `Color` from either crate. But reactor shouldn't depend on canvas
   unconditionally.

3. **`FontWeight` unification**: Should canvas adopt the WinRT representation
   (`u16`) for forward-compat, or keep its simpler `i32`? DWrite uses `i32`
   internally, so canvas's choice is correct for its domain.

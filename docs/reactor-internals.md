# Reactor Internals

This document covers the code generation pipeline, metadata-driven TOML format,
bindings workflow, and threading architecture for contributors working on
`windows-reactor`.

---

## Code Generation (`tool_reactor`)

### Overview

`tool_reactor` reads `winui.toml` and WinUI `.winmd` metadata to
generate type-safe Rust dispatch code for `windows-reactor` via `quote`/`proc-macro2`.

**Run:** `cargo run -p tool_reactor` (~7s, regenerates everything)

### Architecture

```
crates/tools/reactor/src/winui.toml   ← control declarations (metadata-driven)
        │
        ▼
tool_reactor                          ← Reads TOML + loads winmd metadata
        │
        ├── generated_bindings.rs     (per-widget bindings() helpers)
        ├── generated_set_prop.rs     (set_prop dispatch, 6 setter patterns)
        ├── generated_attach_event.rs (event dispatch, 8 invoke patterns)
        ├── generated.txt             (binding filter entries)
        └── bindings.rs               (via windows-bindgen)
```

### Tool Structure

```
crates/tools/reactor/src/
├── main.rs           Pipeline: TOML → resolve → codegen → bindgen
├── schema.rs         Data types + resolve_defaults() inference
├── toml_parser.rs    TOML → Control structs (with metadata validation)
├── metadata.rs       winmd resolver (method → interface + param types)
├── gen_bindings.rs   bindings() helpers (events before props for correct ordering)
├── gen_set_prop.rs   set_prop dispatch (with arm collapsing)
├── gen_attach.rs     attach_event dispatch
├── gen_reactor_txt.rs  generated.txt filter
└── helpers.rs        Shared utilities
```

### Design Choices

- **Metadata-driven** — method, value type, copy semantics, setter pattern, and invoke
  pattern are inferred from `.winmd`. Only non-standard mappings need explicit TOML overrides.
- **Generated + hand-written coexist** — generated dispatch falls through to hand-written
  code in `mod.rs` for complex cases.
- **Deref-aware cast elimination** — generated code skips `.cast::<IFoo>()` when the
  Handle variant's class Derefs to the target interface (the default interface).
- **Filter file split** — `base.txt` contains hand-written filter entries (composition,
  dispatching, native interop). `generated.txt` is auto-derived from the TOML — never
  edit it by hand. Both are passed to `windows-bindgen` to produce `bindings.rs`.
- **One tool, one run** — TOML → code → bindings.rs all in one `cargo run`.

---

## TOML Format (`winui.toml`)

### Design Principle

TOML keys match WinUI metadata names. Given a type and member name, the tool
resolves everything from `.winmd` files — only overrides for non-standard
mappings are needed.

### Format

```toml
["Microsoft.UI.Xaml.Controls.Slider"]
Minimum = {}                                     # type "F64" inferred from metadata
Maximum = {}
Value = {}
Header = {}                                      # Textblock setter, auto Option<T>
IsEnabled = {}
ValueChanged = { property = "NewValue" }          # get_NewValue on args → f64
```

### Inference Rules

| Field | Default | Notes |
|-------|---------|-------|
| `type` | Inferred from metadata param type | `put_IsEnabled(bool)` → `Bool` |
| method | `put_{Name}` | Setter kind auto-detected from metadata param type |
| field | `snake_case(Name)` for props, `on_snake_case(Name)` for events | Derived automatically |
| invoke | Inferred from metadata delegate signature | Matches arg types to invoke pattern |

Only overrides need explicit declaration. Error messages include TOML line numbers.

### Property Overrides

| Override | Description |
|----------|-------------|
| `type = "Type"` | Explicit PropValue variant (`Str`/`F64`/`Bool`/`I32`/etc) |

### Event Overrides

| Override | Description |
|----------|-------------|
| `type = "Type"` | EventHandler payload type (`Unit`/`Bool`/`Str`/`F64`/`I32`/`Color`/`DateTime`/`TimeSpan`) |
| `manual = true` | Skip codegen; hand-written `attach_event` in backend |
| `property = "Name"` | Property on sender/args (e.g. `"IsOn"`); codegen calls `get_Name` |
| `false_event = "Name"` | Complement event for bool-dual (e.g. `"Unchecked"`) |

### Setter Patterns

| Pattern | Description |
|---------|-------------|
| `method` | Direct COM call: `put_Minimum(v)` |
| `method_optional` | Wraps in `Some()`: `put_IsChecked(Some(v))` |
| `method_ireference` | Wraps in `IReference` |
| `method_textblock` | Wraps string in TextBlock for IInspectable params |
| `method_enum_map` | Multi-variant Rust enum → WinUI enum mapping |
| `setter_fn` | Hand-written custom setter in mod.rs |

### Event Invoke Patterns

| Pattern | Description |
|---------|-------------|
| `invoke` | No-arg callback |
| `invoke_bool_getter` | Bool from sender getter |
| `invoke_bool_dual` | Two events (Checked/Unchecked) → true/false |
| `invoke_f64_args` / `invoke_f64_getter` | f64 from args or sender |
| `invoke_string_getter` | String from sender |
| `invoke_i32_args` / `invoke_i32_getter` | i32 from args or sender |

---

## Bindings

The file `crates/libs/reactor/src/bindings.rs` is auto-generated by
`windows-bindgen` from WinUI 3 metadata. **Never edit it by hand.**

### Regenerating

```sh
cargo run -p tool_bindings    # regenerate all bindings (reactor + test)
cargo build -p windows-reactor    # verify reactor compiles
```

### Minimal Filter Syntax

The filter lives in `crates/tools/bindings/src/reactor.txt` (and
`reactor_test.txt` for test-only types). It uses `--minimal` mode which
is opt-in: you list exactly the methods you need, and bindgen computes the
minimal type closure.

| Syntax | Effect |
|--------|--------|
| `Ns.IFace::{m1, m2}` | Specific interface methods (preferred) |
| `Ns.IFace::*` | All methods on an interface (only for `--implement` interfaces) |
| `Ns.Class::{CreateInstance}` | Class instantiation only (no instance methods) |
| `Ns.Type` | Bare type (enum, struct, delegate, constant) |

Methods must use raw metadata names: `get_PropertyName`, `put_PropertyName`,
`add_EventName`, `remove_EventName`.

### Pruning Workflow

1. Comment out or remove the entry from `reactor.txt`
2. `cargo run -p tool_bindings && cargo build -p windows-reactor`
3. Compiler errors show exactly which methods are missing
4. Map Rust names → raw names: `SetX` → `put_X`, `X()` → `get_X`, etc.
5. Add the interface entry with explicit methods: `Ns.IFoo::{put_X, get_Y}`
6. Regenerate and verify

### COM (Win32) Interfaces in Minimal Mode

The `--minimal` filter also works for Win32 COM interfaces (e.g. DXGI, D2D,
DWrite). When a COM method is explicitly listed in the filter, bindgen emits the
full method body and vtable function pointer. Methods NOT in the filter become
`usize` slots (preserving vtable layout without generating code for them).

The type closure is computed automatically: parameter and return types of
requested methods are recursively pulled in. You do NOT need to explicitly list
dependency types — just list the methods you call.

---

## Threading Architecture

### Thread-Local Inventory

| Location | Name | Holds | COM? |
|----------|------|-------|------|
| `app.rs` | `HOST_SLOT` | `ReactorHost` | Yes (STA) |
| `app.rs` | `APP_SLOT` | `Application` | Yes (STA) |
| `host.rs` | `ROOT_FRAMEWORK_ELEMENT` | `IFrameworkElement` | Yes (STA) |
| `host.rs` | `ROOT_WINDOW` | `Window` | Yes (STA) |
| `host.rs` | `PENDING_THEME` | `Cell<Option<ElementTheme>>` | No |
| `host.rs` | `PENDING_TALL` | `Cell<Option<bool>>` | No |
| `dispatcher.rs` | `UI_RERENDER` | `Rc<dyn Fn()>` | No |
| `template_cache.rs` | `SHARED_TEMPLATE` | `DataTemplate` | Yes (STA) |
| `theme.rs` | `CURRENT_COLOR_SCHEME` | `Cell<ColorScheme>` | No |
| `diagnostics.rs` | `EXPECT_PANIC` | `Cell<u32>` | No |
| `modifiers.rs` | `OPS_REGISTRY` | `FxHashMap<TypeId, (CloneFn, EqFn)>` | No |

### Optimization Opportunities

**Pure-Rust thread_locals that could become struct fields:**

- `CURRENT_COLOR_SCHEME` — could be a field on `RenderHost` / passed through `RenderCx`
- `UI_RERENDER` — may be a holdover; audit call sites for reconciler access
- `PENDING_THEME` / `PENDING_TALL` — one-shot latches, could be `RenderHost` fields

**`OPS_REGISTRY` — eliminate via compile-time dispatch:**

Replace the `FxHashMap<TypeId, (CloneFn, EqFn)>` with a custom trait object that
carries clone/eq in its vtable:

```rust
trait AttachedValue: Any {
    fn clone_box(&self) -> Box<dyn AttachedValue>;
    fn eq_box(&self, other: &dyn Any) -> bool;
    fn as_any(&self) -> &dyn Any;
}
impl<T: Clone + PartialEq + 'static> AttachedValue for T { ... }

struct AttachedProps(FxHashMap<TypeId, Box<dyn AttachedValue>>);
```

**What NOT to change:**

- `EXPECT_PANIC` — legitimately thread-local (panic hook context)
- `SHARED_TEMPLATE` — STA-affine COM cache, justified performance win
- `HOST_SLOT` / `APP_SLOT` / `ROOT_*` — STA-affine COM handles; the
  free-function API ergonomics justify the TLS approach

---

## COM Cast Pitfalls

Unnecessary `QueryInterface` casts are a recurring source of overhead in
backend code. These rules apply to all reactor backend code and generated
bindings.

### Deref to Default Interface

Every WinRT class implements `Deref` to its default interface. Casting to the
default interface triggers a runtime `QueryInterface` for no benefit:

```rust
// ❌ unnecessary QI
b.cast::<Xaml::IButton>()?.SetFlyout(&flyout)?;

// ✅ zero-cost Deref
b.SetFlyout(&flyout)?;
```

To check what a class derefs to, search `bindings.rs`:
```
impl core::ops::Deref for <ClassName> {
    type Target = <IDefaultInterface>;
```

### Param Trait Eliminates Parent-Class Casts

Methods accepting `impl Param<ParentClass>` handle conversion automatically.
Don't manually cast subclasses to parent types:

```rust
// ❌ unnecessary QI — put_Background accepts Param<Brush>
let brush: Brush = solid_brush.cast::<Brush>()?;
control.put_Background(&brush)?;

// ✅ zero-cost — SolidColorBrush implements Param<Brush>
control.put_Background(&solid_brush)?;
```

Check for `required_hierarchy!(<ClassName>, <ParentClass>, ...);` in bindings
to verify which parent conversions are free.

### IInspectable Conversions

All WinRT types derive from `IInspectable`. Use `From`, not `.cast()`:

```rust
// ❌
let insp = reference.cast::<IInspectable>()?;

// ✅
let insp: IInspectable = reference.into();
```

Methods with optional parameters infer the type — use plain `None`:

```rust
// ❌
e.SetHeader(None::<&IInspectable>);

// ✅
e.SetHeader(None);
```

### When a Cast IS Required

A cast is needed for methods on a **non-default parent interface**:

```
Button → Deref → IButton        (free)
Button → cast  → IContentControl (QI needed)
Button → cast  → IControl        (QI needed)
```

The generated code uses deref-aware cast elimination — it skips `.cast()` for
default interfaces and only emits casts for non-default interfaces like
`INavigationView2`, `IButtonBase`, `IToggleButton`, `IRangeBase`, `IControl`,
and `IContentControl`.

---

## Naming Collisions Between Reactor and Canvas

> **Status**: Partially resolved. `Color` → `ColorF` rename in canvas is done.
> Other collisions remain.

### Problem

`windows-reactor` and `windows-canvas` are companion crates, but glob-importing
both causes compiler errors because several type names collide:

| Name | `windows-reactor` | `windows-canvas` | Resolution |
|------|-------------------|-------------------|------------|
| `Color` | `u8` ARGB (XAML colors) | `f32` RGBA (D2D colors) | ✅ Canvas renamed to `ColorF` |
| `Brush` | `enum Brush { Solid(Color) }` (property value) | `struct Brush(ID2D1SolidColorBrush)` (GPU handle) | ⚠️ Open |
| `Ellipse` | WinRT XAML shape element | Geometry value for D2D | ⚠️ Open |
| `FontWeight` | WinRT struct `{ Value: u16 }` | Wrapper `FontWeight(pub i32)` | ⚠️ Open |
| `Error`/`Result` | `windows_core::` re-exports | `windows_core::` re-exports | ✅ Identical — Rust unifies |

### Design Decision

Reactor's colliding types are internal/low-usage (property bindings, not hot
draw loops). Canvas's types are user-facing in draw callbacks. When resolving
collisions, canvas keeps the short unqualified name and reactor uses a
domain-prefixed alternative (e.g., `UiBrush`).

### COM API Gotchas (discovered during backend work)

These apply to any future backend or codegen changes:

- `put_IsChecked` on CheckBox takes `Option<bool>` (tri-state nullable boolean)
- TextBox/PasswordBox need get-before-set to avoid cursor position reset
- `string_as_textblock()` creates a TextBlock COM object from `&str`
- ProgressBar uses `IRangeBase` for Value/Min/Max; ProgressRing has direct setters
- ContentDialog needs XamlRoot from a live element (requires backend access)
- Font properties are shared across `IControl`/`ITextBlock`/`IRichTextBlock`

---

## Known Quirks

### `.padding()` silently ignored on `vstack` / `hstack`

In WinUI, `Padding` is a property of `Control`, not `Panel`. Since `StackPanel`
(the backing type for `vstack`/`hstack`) inherits from `Panel` rather than
`Control`, calling `.padding(...)` on a stack compiles and runs but has no
visual effect.

**Workaround:** Use `.margin(...)` on the stack or on individual children.

The `diagnostics` feature emits a debug-mode warning via
`diag::unhandled_modifier` when `.padding()` is applied to an element that
doesn't inherit from `Control`.

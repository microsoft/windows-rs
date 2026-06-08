# Reactor Code Generation

## Overview

`tool_reactor` reads `winui.toml` and WinUI `.winmd` metadata to
generate type-safe Rust dispatch code for `windows-reactor` via `quote`/`proc-macro2`.

**Run:** `cargo run -p tool_reactor` (~7s, regenerates everything)

## Metrics

| Component | Lines |
|-----------|-------|
| Hand-written backend (`mod.rs`) | ~3100 |
| Hand-written backend (`convert.rs`) | ~310 |
| Generated code (3 files) | ~1750 |
| TOML config (non-blank) | ~255 |
| Tool source | ~2800 |

## Architecture

```
winui.toml                    ← 52 controls, metadata-driven declarations
        │
        ▼
tool_reactor                  ← Reads TOML + loads winmd metadata
        │
        ├── generated_bindings.rs     (per-widget bindings() helpers)
        ├── generated_set_prop.rs     (set_prop dispatch, 6 setter patterns)
        ├── generated_attach_event.rs (event dispatch, 8 invoke patterns)
        ├── generated.txt             (binding filter entries)
        └── bindings.rs               (via windows-bindgen)
```

## TOML Format

Keys are WinUI type names. Members are WinUI property/event names resolved against
metadata. The tool looks up `put_{Name}` or `add_{Name}` to classify as property or event.

```toml
["Microsoft.UI.Xaml.Controls.Slider"]
Minimum = {}                                     # type "F64" inferred from metadata
Maximum = {}
Value = {}
Header = {}                                      # Textblock setter, auto Option<T>
IsEnabled = {}
ValueChanged = { property = "NewValue" }          # get_NewValue on args → f64
```

### Inference rules

| Field | Default | Notes |
|-------|---------|-------|
| `type` | Inferred from metadata param type | `put_IsEnabled(bool)` → `Bool` |
| `optional` | Inferred from setter kind | Textblock/IReference → `true`; Method → `false` |
| method | `put_{Name}` | Setter kind auto-detected from metadata param type |
| field | `snake_case(Name)` for props, `on_snake_case(Name)` for events | Derived automatically |
| invoke | Inferred from metadata delegate signature | Matches arg types to invoke pattern |

Only overrides need explicit declaration. Error messages include TOML line numbers.

### Property overrides

| Override | Description |
|----------|-------------|
| `type = "Type"` | Explicit PropValue variant (`Str`/`F64`/`Bool`/`I32`/etc). For Object params, selects IReference wrapping instead of Textblock. |
| `optional = true` | Force `Option<T>` field. Use when `T::default()` ≠ WinUI default (e.g. FontSize). |
| `optional = false` | Force `T` field even for Textblock/IReference setters that normally infer `Option<T>`. |

### Event overrides

| Override | Description |
|----------|-------------|
| `type = "Type"` | EventHandler payload type (`Unit`/`Bool`/`Str`/`F64`/`I32`/`Color`/`DateTime`/`TimeSpan`). |
| `manual = true` | Skip codegen; hand-written `attach_event` in backend. |
| `property = "Name"` | Property on sender/args (e.g. `"IsOn"`); codegen calls `get_Name`. |
| `false_event = "Name"` | Complement event for bool-dual (e.g. `"Unchecked"`). |

### Setter patterns

| Pattern | Description |
|---------|-------------|
| `method` | Direct COM call: `put_Minimum(v)` |
| `method_optional` | Wraps in `Some()`: `put_IsChecked(Some(v))` |
| `method_ireference` | Wraps in `IReference` |
| `method_textblock` | Wraps string in TextBlock for IInspectable params |
| `method_enum_map` | Multi-variant Rust enum → WinUI enum mapping |
| `setter_fn` | Hand-written custom setter in mod.rs |

### Event invoke patterns

| Pattern | Description |
|---------|-------------|
| `invoke` | No-arg callback |
| `invoke_bool_getter` | Bool from sender getter |
| `invoke_bool_dual` | Two events (Checked/Unchecked) → true/false |
| `invoke_f64_args` / `invoke_f64_getter` | f64 from args or sender |
| `invoke_string_getter` | String from sender |
| `invoke_i32_args` / `invoke_i32_getter` | i32 from args or sender |

### Field optionality

- **Default** — Field is `T`, binding always emitted. The diff engine compares old vs
  new bindings and only calls `set_prop` when the value changes.
- **Optional** (`Option<T>`) — Binding emitted when `Some`, unset when `None`. Inferred
  automatically for Textblock and IReference setters (they need `None` to clear the
  WinUI value). Can be forced with `optional = true/false`.

## Match Arm Collapsing

The `gen_set_prop` module groups duplicate match arms that share the same body across
multiple `Handle` variants. For non-custom props (those without `setter_fn`), the largest
duplicate group uses a wildcard `_` arm. For custom props, OR-patterns explicitly list
handles. This eliminated ~60 duplicate arms and removed the need for
`#[allow(clippy::match_same_arms)]`.

## Tool Structure

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

## Design Choices

- **Metadata-driven** — method, value type, copy semantics, setter pattern, and invoke
  pattern are inferred from `.winmd`. Only non-standard mappings need explicit TOML overrides.
- **TOML keys = metadata names** — event/prop TOML keys match WinUI metadata names
  (e.g., `SelectedTimeChanged`, `ColorChanged`). Field names for the public API are
  derived automatically via `on_snake_case(Name)`.
- **Generated + hand-written coexist** — generated dispatch falls through to hand-written
  code in mod.rs for complex cases (~36 setter_fn, ~23 manual events). Widget `bindings()`
  implementations can supplement generated bindings with hand-written logic for compound
  types (BrushBinding, ColorArgb, FlyoutDef, ImageSource, ContentDialogResult).
- **Deref-aware cast elimination** — generated code skips `.cast::<IFoo>()` when the
  Handle variant's class Derefs to the target interface (the default interface). Only
  non-default interfaces (INavigationView2, IButtonBase, IToggleButton, IRangeBase,
  IControl, IContentControl) require explicit casts.
- **Filter file split** — `base.txt` contains hand-written filter entries (composition,
  dispatching, native interop). `generated.txt` is auto-derived from the TOML — never
  edit it by hand. Both are passed to `windows-bindgen` to produce `bindings.rs`.
- **One tool, one run** — TOML → code → bindings.rs all in one `cargo run`.
- **22 codegen tests** validate the pipeline.

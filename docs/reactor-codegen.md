# Reactor Code Generation

## Overview

`tool_reactor_codegen` reads `reactor_widgets.toml` and WinUI `.winmd` metadata to
generate type-safe Rust dispatch code for `windows-reactor` via `quote`/`proc-macro2`.

**Run:** `cargo run -p tool_reactor_codegen` (~7s, regenerates everything)

## Metrics

| Component | Lines |
|-----------|-------|
| Hand-written backend (`mod.rs`) | 3166 |
| Hand-written backend (`convert.rs`) | 338 |
| Generated code (3 files) | 2523 |
| TOML config (non-blank) | 295 |
| Tool source | ~2450 |

## Architecture

```
reactor_widgets.toml          ← 52 controls, metadata-driven declarations
        │
        ▼
tool_reactor_codegen          ← Reads TOML + loads winmd metadata
        │
        ├── generated_bindings.rs     (per-widget bindings() helpers)
        ├── generated_set_prop.rs     (set_prop dispatch, 7 setter patterns)
        ├── generated_attach_event.rs (event dispatch, 8 invoke patterns)
        ├── reactor_generated.txt     (binding filter entries)
        └── bindings.rs               (via windows-bindgen)
```

## TOML Format

Keys are WinUI type names. Members are WinUI property/event names resolved against
metadata. The tool looks up `put_{Name}` or `add_{Name}` to classify as property or event.

```toml
["Microsoft.UI.Xaml.Controls.Slider"]
Minimum = { emit = "always" }                    # value "F64" inferred from metadata
Maximum = { emit = "always" }
Value = { emit = "always" }
Step = { emit = "optional", setter_fn = true, value = "F64" }  # hand-written setter
Header = { emit = "optional" }
IsEnabled = { emit = "when_false" }              # unset auto-inferred to { default = "true" }
ValueChanged = { field = "on_changed", invoke = "invoke_f64_args", getter = "get_NewValue" }
```

### Inference rules

| Field | Default | Example |
|-------|---------|---------|
| `field` | `snake_case(Name)` for props, `on_snake_case(Name)` for events | `PlaceholderText` → `placeholder_text` |
| `method` | `put_{Name}` | `Value` → `put_Value` |
| `value` | Inferred from metadata param type | `put_IsEnabled(bool)` → `Bool` |
| `handler` | Same as event name | `Click` → `Click` |
| `add_method` | `add_{Name}` | `Click` → `add_Click` |
| `unset` | `when_false` → `{default="true"}`, `when_true` → `{default="false"}` | auto-inferred |

Only overrides need explicit declaration. Error messages include TOML line numbers.

### Setter patterns

| Pattern | Description |
|---------|-------------|
| `method` | Direct COM call: `put_Minimum(v)` |
| `method_optional` | Wraps in `Some()`: `put_IsChecked(Some(v))` |
| `method_ireference` | Wraps in `IReference` |
| `method_textblock` | Wraps string in TextBlock for IInspectable params |
| `method_bool_enum` | Bool → WinUI enum mapping |
| `method_enum_map` | Multi-variant Rust enum → WinUI enum mapping |
| `setter_fn` | Hand-written custom setter in mod.rs |

### Event patterns

| Pattern | Description |
|---------|-------------|
| `invoke` | No-arg callback |
| `invoke_bool_getter` | Bool from sender getter |
| `invoke_bool_dual` | Two events (Checked/Unchecked) → true/false |
| `invoke_f64_args` / `invoke_f64_getter` | f64 from args or sender |
| `invoke_string_getter` | String from sender |
| `invoke_i32_args` / `invoke_i32_getter` | i32 from args or sender |

### Emit modes

- `always` — always emitted (field: `T`)
- `optional` — emitted if `Some` (field: `Option<T>`)
- `when_true` / `when_false` — emitted if bool matches
- `non_default` — emitted if `!=` default value

## Tool Structure

```
crates/tools/reactor_codegen/src/
├── main.rs           Pipeline: TOML → resolve → codegen → bindgen
├── schema.rs         Data types + resolve_defaults() inference
├── toml_parser.rs    TOML → Control structs (with metadata validation)
├── metadata.rs       winmd resolver (method → interface + param types)
├── gen_bindings.rs   bindings() helpers
├── gen_set_prop.rs   set_prop dispatch
├── gen_attach.rs     attach_event dispatch
├── gen_reactor_txt.rs  reactor_generated.txt filter
└── helpers.rs        Shared utilities
```

## Design Choices

- **Metadata-driven** — field, method, value, and unset are inferred from `.winmd`.
  Only non-standard mappings need explicit TOML overrides.
- **Generated + hand-written coexist** — generated dispatch falls through to hand-written
  code in mod.rs for complex cases (~36 setter_fn, ~23 attach_fn).
- **One tool, one run** — TOML → code → bindings.rs all in one `cargo run`.

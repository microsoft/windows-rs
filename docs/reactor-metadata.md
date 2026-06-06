# Reactor Metadata-Driven TOML

## Design Principle

The TOML should be driven by WinUI metadata names. Given a type and member name, the
tool resolves everything from `.winmd` files — only overrides for non-standard mappings
are needed.

## Current Format

```toml
["Microsoft.UI.Xaml.Controls.DatePicker"]
Header = { emit = "optional" }
DayVisible = { emit = "optional" }
MonthVisible = { emit = "optional" }
YearVisible = { emit = "optional" }
IsEnabled = { emit = "when_false" }
Changed = { attach_fn = true, handler = "DateTimeChanged" }
```

## What the Tool Infers

Given `"DayVisible"` on `Microsoft.UI.Xaml.Controls.DatePicker`:
1. Finds `put_DayVisible` / `get_DayVisible` in metadata → property
2. `put_DayVisible` takes `bool` → value type `Bool`
3. Rust field = `day_visible`, Prop variant = `DayVisible`
4. Method = `put_DayVisible`, interface = `IDatePicker`
5. `when_false` + `method` → unset auto-inferred as `{ default = "true" }`

## Remaining Overrides

| Category | Count | Why |
|----------|-------|-----|
| `emit` | 190 | Always required — controls mount/diff behavior |
| `value = "..."` | 57 | Non-primitive types, setter_fn props |
| `setter_fn = true` | 36 | Custom setter logic (collections, complex types) |
| `handler = "..."` | 36 | Event handler type differs from event name |
| `unset = { default }` | 20 | Non-bool defaults like `""`, `0.0` |
| `attach_fn = true` | 23 | Custom event attachment logic |
| `field = "..."` | 21 | Ergonomic Rust names (e.g., `placeholder`, `on_changed`) |
| `unset = "custom"` | 5 | Block auto-inference on when_true/when_false |

## Implementation Status

- **295 non-blank lines** (73% reduction from 1105 original)
- V1 format removed — single parser, single format
- Prop/Event enum variants use metadata names
- Widget structs use WinUI type names (no "Widget" suffix)
- Error messages include TOML line numbers
- 19 codegen tests + 3 metadata tests

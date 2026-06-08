# Reactor Metadata-Driven TOML

## Design Principle

The TOML is driven by WinUI metadata names. TOML keys match the exact WinUI method
names (e.g., `SelectedTimeChanged`, `ColorChanged`, `IsEnabled`). Given a type and
member name, the tool resolves everything from `.winmd` files — only overrides for
non-standard mappings are needed.

Event/prop names that are only used internally in generated backend code need no
overrides — they derive naturally from the metadata. The public API field names
(what crate users see) are derived automatically via `snake_case(Name)` for props
and `on_snake_case(Name)` for events. Only genuinely ergonomic overrides use
`field = "..."` (e.g., `placeholder` for `PlaceholderText`).

## Current Format

```toml
["Microsoft.UI.Xaml.Controls.DatePicker"]
Header = {}                                      # emit defaults to "optional"
DayVisible = { emit = "when_false" }
MonthVisible = { emit = "when_false" }
YearVisible = { emit = "when_false" }
IsEnabled = { emit = "when_false" }
SelectedDateChanged = { attach_fn = true, handler = "DateTimeChanged" }
```

## What the Tool Infers

Given `"DayVisible"` on `Microsoft.UI.Xaml.Controls.DatePicker`:
1. Finds `put_DayVisible` / `get_DayVisible` in metadata → property
2. `put_DayVisible` takes `bool` → value type `Bool`
3. Rust field = `day_visible`, Prop variant = `DayVisible`
4. Method = `put_DayVisible`, interface = `IDatePicker`
5. `when_false` + `method` → unset auto-inferred as `{ default = "true" }`

Given `"Header"` with no overrides:
1. Emit defaults to `optional` (field: `Option<String>`)
2. `put_Header` takes IInspectable → `method_textblock` setter pattern
3. Value type `Str` inferred from param classification

Given `"SelectedDateChanged"`:
1. No `put_` method found → classified as event
2. Event variant = `SelectedDateChanged`
3. Rust field = `on_selected_date_changed` (auto-derived)
4. `attach_fn = true` → hand-written backend handler
5. `handler = "DateTimeChanged"` → EventHandler variant (non-default, override needed)

## Remaining Overrides

| Category | Count | Why |
|----------|-------|-----|
| `emit` | ~50 | Only non-default strategies (`always`, `when_true`, `when_false`, `non_default`); `optional` is the default and omitted |
| `setter_fn = true` | 36 | Custom setter logic (collections, complex types) |
| `handler = "..."` | ~34 | Event handler type differs from event name |
| `value = "..."` | ~31 | Non-primitive types, setter_fn props |
| `attach_fn = true` | 23 | Custom event attachment logic |
| `unset = { default }` | 20 | Non-bool defaults like `""`, `0.0` |
| `field = "..."` | ~18 | Ergonomic Rust names (e.g., `placeholder`) |
| `unset = "custom"` | 5 | Block auto-inference on when_true/when_false |

## Implementation Status

- **~290 non-blank TOML lines** (74% reduction from 1105 original)
- V1 format removed — single parser, single format
- Prop/Event enum variants use metadata names (no synthetic names)
- Widget structs use WinUI type names (no "Widget" suffix)
- Error messages include TOML line numbers
- 20 codegen tests + 3 metadata tests

# windows-bindgen Test Coverage Analysis

## test_bindgen2 Coverage Summary

As of June 2026, `test_bindgen2` provides **64.1% line coverage** of actionable
windows-bindgen code (excluding legacy package/index/reference modules) with 51
data-driven tests.

## Removal Candidates

### `--reference` — REMOVED

- Removed in this PR. Was not used by any production tool.
- Test crates and samples that depended on it have been removed.
- The internal `References` infrastructure remains for implicit sibling crate
  references (windows_future, windows_collections, etc.) which are auto-registered
  based on input metadata.

### `--index` — REMOVED

- Removed alongside `--reference`. Was only used by `tool_package` for the
  `windows` crate's `features.json` (a 13MB JSON file consumed by the
  `web/features` search tool).
- `web/features` has been removed.
- `serde` and `serde_json` dependencies dropped from windows-bindgen.

## Production-Only Code (not testable from test_bindgen2)

These modules are used in production but require full Win32/WinRT metadata
(impractical to mock with RDL):

| Module | Coverage | Used By |
|--------|----------|---------|
| `package_writer.rs` (129 lines) | 10.9% | `tool_package` → `windows` and `windows-sys` |

These are implicitly tested by `tool_package` running successfully.

## Hard-to-Test Patterns

The following require metadata patterns difficult to express in RDL:

- **Query return hint** (`cpp_method.rs`) — needs `QueryInterface`-shaped params
  (GUID + void** with specific attributes)
- **Array params** (`cpp_method.rs`) — needs `NativeArrayInfoAttribute` /
  `MemorySizeAttribute` on params
- **Composable classes** (`minimal_type_map.rs`) — needs class hierarchies with
  composable interfaces referencing external types
- **Scoped enums** (`cpp_enum.rs`) — needs `ScopedEnumAttribute` (not supported
  in RDL)
- **String constants** (`cpp_const.rs`) — needs `PCSTR`/`PCWSTR` type + string
  constant values
- **AlsoUsableFor handles** (`cpp_handle.rs`) — needs `AlsoUsableForAttribute`
- **Free functions on handles** (`cpp_handle.rs`) — needs RAIIFree metadata

## Current Coverage by File

Files below 60% (actionable, excluding package/index/reference):

| File | Coverage | Notes |
|------|----------|-------|
| `minimal_type_map.rs` | 35% | Needs complex class hierarchies |
| `io.rs` | 37% | File I/O utilities, package-mode specific |
| `paths.rs` | 37% | Path utilities, package-mode specific |
| `cpp_handle.rs` | 46% | Pointer handles, AlsoUsableFor |
| `filter.rs` | 47% | Complex wildcard/method/enum filters |
| `cpp_enum.rs` | 47% | Scoped enums (not mockable in RDL) |
| `value.rs` | 48% | String constants, GUID struct fields |
| `cpp_const.rs` | 50% | GUID constants, string constants |
| `cpp_interface.rs` | 54% | More interface patterns |
| `filter_parser.rs` | 57% | Complex filter syntax parsing |
| `method_names.rs` | 57% | Overload disambiguation |
| `types/mod.rs` | 58% | Type dispatch |
| `cpp_method.rs` | 59% | Query, array, optional patterns |

## Recommendations

1. **Keep `--package`** — used by tool_package for published crates
2. **Accept ~65% as practical ceiling** for test_bindgen2 — remaining gaps need
   real metadata or RDL extensions
5. **Consider RDL extensions** if we want to push coverage higher:
   - Support for `ScopedEnumAttribute`
   - Support for `NativeArrayInfoAttribute` / `MemorySizeAttribute`
   - Support for `AlsoUsableForAttribute`

## Tests Ported from test_bindgen

The following old test_bindgen fixtures were ported to test_bindgen2:

- `class_hierarchy` — multi-level class inheritance with `required_hierarchy!`
- `auto_events` — event revoker pattern, `#[special]` add/remove, delegate params
- `enum_name_conflict` — variant name collision with enum name (appends `_`)

Not ported (need real metadata):

- `ireference_sugar` — `IReference<T>` generic sugar (needs `windows-reference` crate)
- `composable_class` — composable factory (needs default refs)
- `method_filter_*` (class/mixed/overload/property/setter_only) — need real metadata
- Generic interfaces/delegates — need real WinRT metadata
- Arch-specific structs — need real Win32 metadata

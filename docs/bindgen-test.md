# windows-bindgen Test Coverage Analysis

## test_bindgen2 Coverage Summary

As of June 2026, `test_bindgen2` provides **64.1% line coverage** of actionable
windows-bindgen code (excluding legacy package/index/reference modules) with 51
data-driven tests.

## Removal Candidates

### `--reference` (references.rs, 58 lines)

- **NOT used by any production tool** (tool_bindings, tool_reactor, tool_package)
- Only used in test crates: `tests/winrt/reference*`, `tests/winrt/events`,
  `tests/misc/component*`, `tests/winrt/collection_interop`
- These tests exercise multi-crate scenarios where one crate references types
  from another crate's bindings
- **Candidate for removal** — simplifies bindgen and removes an obscure feature

## Production-Only Code (not testable from test_bindgen2)

These modules are used in production but require full Win32/WinRT metadata
(impractical to mock with RDL):

| Module | Coverage | Used By |
|--------|----------|---------|
| `index.rs` (99 lines) | 0% | `tool_package` → `windows` crate `features.json` |
| `package_writer.rs` (129 lines) | 10.9% | `tool_package` → `windows` and `windows-sys` |
| `guid.rs` (4 lines) | 0% | `index.rs` (GUID Display for JSON) |

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

1. **Remove `--reference`** — unused in production, adds complexity
2. **Keep package/index** — used by tool_package for published crates
3. **Accept ~64% as practical ceiling** for test_bindgen2 — remaining gaps need
   real metadata or RDL extensions
4. **Consider RDL extensions** if we want to push coverage higher:
   - Support for `ScopedEnumAttribute`
   - Support for `NativeArrayInfoAttribute` / `MemorySizeAttribute`
   - Support for `AlsoUsableForAttribute`

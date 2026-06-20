# windows-bindgen Test Coverage Analysis

## test_bindgen2 Coverage Summary

As of June 2026, `test_bindgen2` has **61 data-driven tests** covering all
remaining CLI options and all type categories expressible in RDL.

### CLI Options — All Tested

| Option | Tests |
|--------|-------|
| `--flat` | Most tests (54+) |
| `--sys` | `enum_sys`, `fn_sys`, `fn_sys_flat`, `struct_sys`, `interface_sys`, `method_return_sys`, `enum_flags_sys` |
| `--extern` | `fn_sys_extern` |
| `--minimal` | `class_minimal`, `delegate_minimal`, `enum_minimal`, `event_minimal`, `fn_minimal`, `handle_minimal`, `interface_minimal`, `minimal_deps`, `winrt_enum_minimal`, `winrt_struct_minimal` |
| `--implement` | `class_implement`, `com_implement`, `implement_pattern` |
| `--dead-code` | `dead_code` |
| `--derive` | `derive_enum`, `derive_multiple`, `derive_suppress`, `struct_derive` |
| `--filter` | All tests; method-level in `method_filter_*` |
| Namespace modules | `modules`, `enum_default`, `enum_name_conflict`, `class_hierarchy`, `filter_namespace` |

### Type Categories — All Covered

**Win32 (`#[win32]` in RDL):**
- Functions: `fn`, `fn_sys`, `fn_sys_flat`, `fn_sys_extern`, `fn_result`, `fn_minimal`
- Structs: `struct`, `struct_sys`, `struct_nested`, `struct_derive`
- Enums: `enum_flags`, `enum_flags_sys`
- COM interfaces: `com_interface`, `com_implement`, `interface_required`
- Handles: `handle`, `handle_free`, `handle_minimal`
- Constants: `const`, `const_guid`, `const_types`
- Callbacks: `callback`

**WinRT (`#[winrt]` in RDL):**
- Interfaces: `interface`, `interface_hierarchy`, `interface_minimal`, `interface_string`, `interface_void`, `interface_sys`
- Delegates: `delegate`, `delegate_minimal`
- Classes: `class`, `class_hierarchy`, `class_implement`, `class_minimal`, `class_static`
- Enums: `winrt_enum`, `winrt_enum_minimal`
- Structs: `winrt_struct`, `winrt_struct_minimal`
- Events: `event`, `event_minimal`, `auto_events`

## Removed Options

| Option | Status | Notes |
|--------|--------|-------|
| `--reference` | REMOVED | Internal `References` infra remains for implicit sibling refs |
| `--index` | REMOVED | `features.json` and `web/features` removed; serde deps dropped |
| `--no-toml` | REMOVED | Was dead code (never used) |
| `--link` | REMOVED | Hardcoded to `windows_link` (sys) / `windows_core` (non-sys) |

## Not Testable via RDL (covered by test_bindgen + tool_package)

These require real `.winmd` metadata that cannot be mocked in RDL:

- **Generic delegates/interfaces** — `IAsyncOperation<T>`, `TypedEventHandler<S,A>`, etc.
- **Async class alias** — special metadata attribute triggers `pub type Class = Interface`
- **Composable/exclusive classes** — WinUI-style activation with compose
- **`--package` layout** — per-namespace files + Cargo.toml features (used by `tool_package`)
- **Query return hint** — `QueryInterface`-shaped params (GUID + void**)
- **Array params** — `NativeArrayInfoAttribute` / `MemorySizeAttribute`
- **Scoped enums** — `ScopedEnumAttribute`
- **String constants** — `PCSTR`/`PCWSTR` + string constant values
- **AlsoUsableFor handles** — `AlsoUsableForAttribute`
- **Arch-specific structs** — real Win32 metadata with arch attributes

These are implicitly tested by:
- `test_bindgen` (305 tests using `--in default` with real Windows metadata)
- `tool_package` running successfully (exercises package mode end-to-end)

## No Dead Code Remaining

After removing `--reference`, `--index`, `--no-toml`, and `--link`, all remaining
code in windows-bindgen is reachable through:
1. `test_bindgen2` (RDL-based, 61 tests)
2. `test_bindgen` (real winmd, 305 tests)
3. `tool_package` (package-mode generation)

## Recommendations

1. **Keep `--package`** — used by tool_package for published crates
2. **Accept current coverage as practical ceiling** for test_bindgen2 — remaining
   gaps need real metadata or RDL extensions
3. **Consider RDL extensions** if we want to push test_bindgen2 coverage higher:
   - Support for `ScopedEnumAttribute`
   - Support for `NativeArrayInfoAttribute` / `MemorySizeAttribute`
   - Support for `AlsoUsableForAttribute`
4. **Old test_bindgen still provides value** for generics/async/package paths

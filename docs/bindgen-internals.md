# `windows-bindgen` Internals

Architecture, option map, dependency model, and simplification plan for
`windows-bindgen` contributors.

---

## Design Philosophy

`windows-bindgen` serves two fundamentally different audiences:

1. **Internal tooling** — generates the massive `windows` and `windows-sys`
   umbrella crates using `--package` mode with namespace-based cargo features.
   This is a special case that nobody outside the project needs.

2. **External / library use** — generates focused bindings for specific APIs,
   typically into a single `bindings.rs` file. This is the use case we want to
   **encourage and optimize for**.

The encouraged patterns:

| Pattern | Style | Dependencies | Use Case |
|---------|-------|-------------|----------|
| **flat + sys** | `--sys --flat` | `windows-link` only | Low-level Win32 with zero overhead |
| **flat + minimal** | `--minimal --flat` | Small crates (`windows-core`, `windows-time`, etc.) | COM/WinRT with minimal deps |

The discouraged patterns:

| Pattern | Why |
|---------|-----|
| Generating umbrella crates (`--package`) | Special case, adds complexity |
| Depending on `windows` / `windows-sys` | Monolithic, hurts build times |
| Using `Style::Default` for new code | Generates excessive boilerplate |

**Important distinction:** `windows-core` and the other small dedicated crates
(`windows-time`, `windows-numerics`, `windows-collections`, `windows-future`,
`windows-reference`, `windows-link`, `windows-result`, `windows-strings`) are
**good** dependencies — they're small, focused, and compile fast. The monolithic
crates `windows` and `windows-sys` are what we want to avoid.

---

## Current Architecture

### Source Layout

```
crates/libs/bindgen/src/
├── lib.rs                 Bindgen struct, CLI parsing, write() orchestration
├── config.rs              Config — per-write-call context (reader, filter, refs, etc.)
├── paths.rs               Namespace/type-path resolution + dependency-mode dispatch
├── filter.rs              Type-level include/exclude filter (the `--filter` DSL)
├── minimal_filter.rs      Method-centric filter for `--minimal` mode
├── minimal_type_map.rs    Automatic type closure for `--minimal` mode
├── type_map.rs            Dependency walking + filtered type collection
├── type_tree.rs           Namespace → nested module tree
├── references.rs          `--reference` parsing + type ownership queries
├── implements.rs          `--implement` pattern matcher
├── package_writer.rs      `--package` layout (one file per namespace + Cargo.toml)
├── index.rs               `--index` feature dependency JSON for the `windows` crate
├── derive.rs / derive_writer.rs  Extra derive trait emission
├── libraries.rs           DLL→function map for umbrella import libs
├── io.rs                  File I/O utilities
├── format.rs              rustfmt integration
├── tokens.rs              TokenStream helpers
├── types/                 Per-type-kind code generation (class, interface, struct, etc.)
├── tables/                winmd table accessors (MethodDef, TypeDef, etc.)
└── winmd/                 Raw .winmd reader (PE/ECMA-335 parser)
```

### Pipeline

```
CLI args / Bindgen builder
  → expand_args (flatten --etc files)
  → expand_input (load .winmd files, including bundled "default")
  → Reader (parse .winmd into in-memory tables)
  → Filter or MinimalFilter (parse --filter entries)
  → References (parse --reference entries + auto-add implicit refs)
  → TypeMap::filter / MinimalTypeMap::build (resolve type closure)
  → TypeTree::new (organize into namespace tree)
  → Config::write (dispatch to flat / modules / package writer)
  → Per-type write() calls (class.rs, interface.rs, struct.rs, etc.)
  → paths.rs resolves each type reference to the correct crate path
```

---

## Option Map

### Core Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `--in <path>` | Multi | `"default"` (bundled .winmd) | Input .winmd file(s) or directories |
| `--out <path>` | Required | — | Output file or directory path |
| `--filter <entries>` | Multi, required | — | Types/methods to include; `!` prefix excludes |
| `--etc <file>` | Multi | — | Response file with additional args |

### Layout (mutually exclusive)

| Option | Enum | Description |
|--------|------|-------------|
| *(default)* | `Layout::Modules` | One Rust module per metadata namespace |
| `--flat` | `Layout::Flat` | All items in a single flat file |
| *(inferred)* | `Layout::Package` | One file per namespace + Cargo.toml features (inferred when output dir contains Cargo.toml) |

### Style

| Option | Enum | Description |
|--------|------|-------------|
| *(default)* | `Style::Default` | Full WinRT/COM bindings with ergonomic wrappers |
| `--sys` | `Style::Lean { com: false }` | Raw C-style bindings (link! macros, no COM wrappers) |
| `--minimal` | `Style::Lean { com: true }` | Lean bindings with COM support (demoted unused vtable slots, auto-revoking events) |
| `--extern` | `Lean { extern_fns: true }` | `extern { fn … }` instead of `link!` (requires `--sys`) |

Both `--sys` and `--minimal` are "lean" modes that share most codegen paths.
The `com` flag controls whether COM interfaces get proper vtable types and
method wrappers (`has_com()`) or are reduced to void pointers (`is_sys()`).

Helper methods on `Style`:
- `is_lean()` — true for both sys and minimal
- `has_com()` — true for lean with COM (old minimal)
- `is_sys()` — true for lean without COM (old sys)

### Dependencies

| Option | Enum | Description |
|--------|------|-------------|
| `--deps core` | `DepMode::Core` | Use `windows_core::` for all shared types |
| `--deps specific` | `DepMode::Specific` | Use `windows_result::`, `windows_strings::`, `windows_link::` directly |
| `--deps none` | `DepMode::None` | No `windows-*` crate dependencies; inline everything |

When `--deps` is not specified, the default is inferred:
- `--sys` (lean without COM) → `--deps none` (standalone, only needs `windows-link`)
- All other modes → `--deps core`

### Other

| Option | Description |
|--------|-------------|
| `--reference <spec>` | Delegate types to an external crate (format: `crate,style,path`) |
| `--derive <traits>` | Extra derives on generated types |
| `--implement [patterns]` | Emit `_Impl` scaffolding for WinRT interfaces |
| `--link <crate>` | Override the link macro source (default: `windows_core` or `windows_link`) |
| `--rustfmt <config>` | Rustfmt configuration |

---

## The Dependency Problem (Resolved)

### Problem Statement

When `--sys` mode generated bindings, shared types like `PCWSTR` resolved to
`windows_sys::core::PCWSTR`, forcing a dependency on the monolithic `windows-sys`
crate. The only way to avoid this was `--deps none`, which was non-obvious.

### Resolution (Phase 1)

`--sys` now defaults to `--deps none` when `--deps` is not explicitly specified.
Types like `PCWSTR` become `*const u16` and `HRESULT` becomes `i32`. The
`windows-sys` umbrella crate's `sys.txt` filter explicitly specifies `--deps core`
to opt in to the old behavior.

String constants (`w!`/`s!` macros) emit inline UTF-16/ANSI arrays when
`deps=none` rather than referencing macros from `windows-strings`.

### Who Uses What Today

#### Internal filter files (`tool_bindings`)

| Consumer | Layout | Style | Deps | Key Options |
|----------|--------|-------|------|-------------|
| `windows` crate | `--package` | Default | core (implicit) | `--index`, `--reference windows` |
| `windows-sys` crate | `--package --sys` | Sys | core (implicit) | Broad Win32/Wdk filter |
| `windows-core` bootstrap | `--flat --sys` | Sys | `none` (explicit) | Self-bootstrap |
| `windows-result` bootstrap | `--flat --sys` | Sys | `none` (explicit) | Same bootstrap pattern |
| `windows-strings` bootstrap | `--flat --sys` | Sys | `none` (explicit) | Same bootstrap pattern |
| `windows-reactor` | `--flat --minimal` | Minimal | core (implicit) | Method-level filters |
| `windows-canvas` | `--flat --minimal` | Minimal | core (implicit) | Method-level filters |
| `collections` / `future` / `numerics` / `time` / `metadata` | `--flat` | Default | core (implicit) | Small focused crates |
| `registry` / `services` / `threading` / `version` | `--flat --sys` | Sys | `none` (explicit) | Standalone bootstrap |

#### Samples (build.rs, 7 total)

| Sample | Style | Deps | Notable |
|--------|-------|------|---------|
| `robot/component` | Default | core (implicit) | Custom .winmd, `--implement` |
| `robot/client` | Default | core (implicit) | Custom .winmd |
| `windows/xaml_app` | Default | core (implicit) | `--reference windows,skip-root,...` |
| `windows/webview` | Default | core (implicit) | Custom .winmd, `--reference`, `--implement` |
| `json/json_validator_winrt*` | Default | core (implicit) | Custom .winmd |
| `canvas/standalone` | Minimal | core (implicit) | D2D/DXGI bindings |
| `services/time` | Sys | `none` (explicit) | Fully standalone, extra derives |

#### Tests (build.rs, 27 total)

| Pattern | Count | Deps | Notable |
|---------|-------|------|---------|
| WinRT component tests | ~20 | core (implicit) | Custom .winmd, `--implement`, `--flat` |
| `reference_no_deps` | 1 | `none` (explicit) | Tests `--deps none` path |
| `reference_custom` | 1 | `none` (explicit) | Tests `--deps none` + `--reference` |
| `no_core` | 1 | `specific` (explicit) | Tests `--deps specific` path |
| `just_core` | 1 | core (implicit) | Basic Win32 APIs |
| `reference_windows` | 1 | core (implicit) | `--reference windows` |
| `component` / `component_client` | 2 | core (implicit) | `--reference` |
| `collection_interop` | 1 | core (implicit) | `--reference windows_collections` |

#### Key observations

- **All 34 build.rs files use `--flat`** — nobody uses module layout outside
  the umbrella crates.
- **30 of 34 use `Style::Default`** — only `canvas/standalone` uses `--minimal`
  and `services/time` uses `--sys` among samples/tests.
- **27 of 34 rely on the `--deps core` default** — only 4 specify deps
  explicitly (`none` × 3, `specific` × 1).
- **Most WinRT tests use custom .winmd** — they test the code-generation
  pipeline itself, not typical external usage.
- **8 use `--reference`** — always in conjunction with WinRT tests or samples
  that reference the `windows` umbrella crate or specific sub-crates.
- **`Style::Default` dominance in tests/samples** — these tests exercise the
  full WinRT codegen pipeline. The encouraged patterns (flat+sys, flat+minimal)
  are underrepresented in the test suite.
- External users (#4581) | `--flat [--sys]` | Varies | **core (implicit)** ← the problem

---

## Simplification Plan

### Phase 1: Fix `--sys` Default Dependencies ✅

**Goal:** `--sys` should not reference `windows-sys` by default.

**What changed:**
- `deps` field on `Bindgen` changed from `DepMode` to `Option<DepMode>`
- Added `resolved_deps()` method: sys → None, non-sys → Core, when unspecified
- `sys.txt` (umbrella crate) now explicitly specifies `--deps core`
- String constants emit inline UTF-16/ANSI arrays when `deps=none`
- CppConst::combine() fixed to track PCWSTR/PCSTR in dependency closure

**Impact:** External sys users get standalone bindings by default — no
`windows-sys` dependency, only `windows-link`. Resolves #4581.

### Phase 2: Unify `--sys` and `--minimal` ✅

**Goal:** Replace three `Style` variants with a unified two-level hierarchy.

**What changed:**
- `Style` restructured from `{Default, Sys{extern_fns}, Minimal}` to
  `{Default, Lean{com, extern_fns}}`
- ~44 `is_minimal()` call sites updated to `has_com()`
- ~6 `is_sys()||is_minimal()` sites collapsed to `is_lean()`
- Remaining `is_sys()` sites preserved where lean-without-COM needs different
  behavior (void ptr interfaces, raw delegate params, etc.)

**Breaking changes** (behaviors that were worse in sys mode, now unified):
- Structs/enums always get `Debug`, `Default`, `PartialEq`, `Eq` derives
- Flags enums always get `BitOr`/`BitAnd`/`Not` ops
- `ManuallyDrop` always wraps non-Copy union fields (correctness)
- `Clone`+`Copy` are conditional on actual copyability (correctness)
- `PrimitiveOrEnum` uses the enum type (stronger typing)
- windows-sys `GUID` struct updated with `Debug`+`PartialEq`

**Impact:** 195 windows-sys generated files updated (adding derives). All
semver-compatible (adding trait impls). Reactor/canvas output unchanged.

### Phase 3: Isolate Umbrella Crate Tooling ✅

**Goal:** Remove umbrella-crate-only options from the public interface and infer
them automatically.

**What changed:**
- `paths.rs`: `windows_sys::core::` path now only emitted when
  `is_sys() && is_package()`. For `is_sys() && !is_package()`, deps are
  resolved through the normal `DepMode` match (None→inline, Core→windows_core,
  Specific→per-crate). This isolates the umbrella-crate path from standalone
  use.
- Removed 6 redundant `is_package()` guards from type writers: `method.rs`,
  `cpp_method.rs`, `class.rs`, `cpp_interface.rs`, `interface.rs`, `mod.rs`.
  The `Cfg` struct already returns empty tokens for non-package mode, so these
  guards were dead code.
- **Removed `--package`, `--index`, `--no-toml` from the CLI and builder API.**
  Package mode is now inferred when the output directory contains a `Cargo.toml`.
  Index emission (`features.json`) is inferred when package mode is active and
  the style is `Default` (only the `windows` crate needs this).
- Simplified `Layout` enum: removed `no_toml` field from `Package` variant.
  `--no-toml` was never used by any consumer.

**Preserved:** `--reference` and `ReferenceStyle::SkipRoot` remain in the
public API since they are used by external samples/tests (`xaml_app`,
`webview`, `reference_custom`, etc.).

### Phase 4: Unify Filter Implementations (Lower Priority)

Currently there are two parallel filter + type closure implementations:

1. **`Filter` + `TypeMap::filter`** — traditional type-level include/exclude.
   Pulls in *all* dependencies of matched types. Used for Default and Sys.
2. **`MinimalFilter` + `MinimalTypeMap::build`** — method-centric filter.
   Walks only requested method signatures for a tighter closure. Used for
   Minimal.

```rust
// The fork in write():
let (filter, types) = if let Some(minimal) = &minimal_filter {
    let (types, filter) = MinimalTypeMap::build(&reader, minimal, &references);
    (filter, types)
} else {
    let filter = Filter::new(&reader, &include, &exclude);
    let types = TypeMap::filter(&reader, &filter, &references);
    (filter, types)
};
```

**Convergence path:** Make `MinimalTypeMap`'s method-aware closure the only
implementation. For `Style::Default`, treat all methods on filtered types as
requested (`MethodSet::All`) — producing identical output. For lean modes,
use the method-level filter entries to compute the tight closure.

This would:
- Eliminate the dual code path
- Make method-level filtering available in all modes
- Reduce `filter.rs` + `minimal_filter.rs` to a single filter module

---

## Option Interaction Matrix

| | `--flat` | `--sys` | `--minimal` | `--extern` | `--deps` |
|-|----------|---------|-------------|------------|----------|
| `--flat` | — | ✓ | ✓ | ✓¹ | ✓ |
| `--sys` | ✓ | — | ✗² | ✓ | ✓ |
| `--minimal` | ✓ | ✗² | — | ✗ | ✓ |
| `--deps none` | ✓ | ✓ | ✓ | ✓ | — |

¹ Requires `--sys`.
² Both map to `Style::Lean` with different `com` values — mutually
exclusive at the CLI level because they represent opposite ends of the same
axis (COM support on/off).

Note: Package mode is no longer an explicit option. It is inferred when the
output directory contains a `Cargo.toml`.

---

## Common Invocation Patterns

### Standalone Win32 bindings (most common external use case)

```
--out src/bindings.rs --flat --sys --filter LoadCursorW
```

`--sys` defaults to `--deps none`, so this produces standalone bindings with
no `windows-sys` dependency — only `windows-link`.

### Standalone COM/WinRT bindings

```
--out src/bindings.rs --flat --minimal --filter
    Windows.Win32.Graphics.Direct2D.ID2D1Factory1::{CreateDevice}
```

Dependencies: `windows-core` + any small crates needed by the type closure.

### Internal crate bootstrap

```
--out src/bindings.rs --flat --sys --deps none --filter CoTaskMemAlloc
```

Unchanged — `--deps none` is already explicit.

### Umbrella crate generation (internal only)

```
--out crates/libs/windows --deps core --filter Windows
--out crates/libs/sys --sys --deps core --filter Windows.Win32
```

Package mode and index emission are inferred automatically:
- Package mode activates when `{output}/Cargo.toml` exists
- Index (`features.json`) emits when package mode + `Style::Default`

These must explicitly opt in to `--deps core` for the `windows_core::` /
`windows_sys::core::` paths.

---

## Migration Checklist

### Phase 1 (fix `--sys` default deps) ✅

- [x] Make `--sys` default to `DepMode::None` when `--deps` is not explicitly set
- [x] Add `--deps core` to `crates/tools/bindings/src/sys.txt`
- [x] Verify bootstrap crates already have `--deps none` — no change needed
- [x] Verify non-sys filter files are unaffected
- [x] Update `docs/bindgen.md` to document the new default
- [x] Inline UTF-16/ANSI literals for string constants when `deps=none`
- [x] Run `cargo run -p tool_bindings` — output unchanged
- [x] 325 tests pass

### Phase 2 (unify sys + minimal) ✅

- [x] Audit all `is_sys()` and `is_minimal()` call sites (~70 total)
- [x] Restructure `Style` enum: `{Default, Lean{com, extern_fns}}`
- [x] ~6 shared sites → `is_lean()`
- [x] ~44 minimal sites → `has_com()`
- [x] Remove sys-only derive/flags stripping (breaking: adds derives)
- [x] Fix ManuallyDrop, Clone+Copy correctness (breaking: behavior change)
- [x] Update windows-sys GUID with Debug+PartialEq
- [x] Run `cargo run -p tool_bindings` — 195 sys files updated
- [x] Run `cargo run -p tool_reactor` — output unchanged
- [x] All dependent crates compile (core, windows, sys, reactor, canvas)
- [x] 325 tests pass, clippy clean, fmt clean

### Phase 3 (isolate umbrella tooling) ✅

- [x] Isolate `windows_sys::core::` path to package mode only in `paths.rs`
- [x] Remove 6 redundant `is_package()` guards from type writers
- [x] Remove `--package`, `--index`, `--no-toml` from CLI and builder API
- [x] Infer package mode from `Cargo.toml` existence in output directory
- [x] Infer index emission when package + `Style::Default`
- [x] Simplify `Layout` enum (remove `no_toml` field)
- [x] Preserve `--reference` and `SkipRoot` (used by external samples/tests)
- [x] Zero generated output changes
- [x] 325 tests pass, clippy clean, fmt clean

### Phase 4 (unify filters)

- [ ] Prototype single filter implementation using `MinimalTypeMap` closure
- [ ] Benchmark type closure for `windows` crate filter (must not regress)
- [ ] Verify bit-for-bit output for all modes
- [ ] Remove `filter.rs` / `minimal_filter.rs` duplication

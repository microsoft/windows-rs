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
| `--package` | `Layout::Package` | One file per namespace + Cargo.toml features (umbrella crates only) |
| `--no-toml` | `Package { no_toml: true }` | Skip Cargo.toml rewrite (requires `--package`) |

### Style (mutually exclusive)

| Option | Enum | Description |
|--------|------|-------------|
| *(default)* | `Style::Default` | Full WinRT/COM bindings with ergonomic wrappers |
| `--sys` | `Style::Sys` | Raw C-style bindings (link! macros, no wrappers) |
| `--minimal` | `Style::Minimal` | Reduced bindings: no class wrappers, auto-revoking events, demoted unused vtable slots |
| `--extern` | `Sys { extern_fns: true }` | `extern { fn … }` instead of `link!` (requires `--sys`) |

### Dependencies

| Option | Enum | Description |
|--------|------|-------------|
| `--deps core` | `DepMode::Core` | **Current default.** Use `windows_core::` for all shared types |
| `--deps specific` | `DepMode::Specific` | Use `windows_result::`, `windows_strings::`, `windows_link::` directly |
| `--deps none` | `DepMode::None` | No `windows-*` crate dependencies; inline everything |

### Other

| Option | Description |
|--------|-------------|
| `--reference <spec>` | Delegate types to an external crate (format: `crate,style,path`) |
| `--derive <traits>` | Extra derives on generated types |
| `--implement [patterns]` | Emit `_Impl` scaffolding for WinRT interfaces |
| `--link <crate>` | Override the link macro source (default: `windows_core` or `windows_link`) |
| `--rustfmt <config>` | Rustfmt configuration |
| `--index` | Emit `features.json` alongside output (umbrella crates only) |

---

## The Dependency Problem

### Problem Statement

When `--sys` mode generates bindings, shared types like `PCWSTR` resolve to
`windows_sys::core::PCWSTR`, forcing a dependency on the monolithic `windows-sys`
crate. This happens because `write_specific()` in `paths.rs` hardcodes the
`windows_sys::core::` prefix for all sys-mode bindings when `deps != None`.

The only way to avoid this today is `--deps none`, which is non-obvious and
poorly documented.

### The Issue (#4581)

A user generating `--sys --flat` bindings for `LoadCursorW` got:

```rust
fn LoadCursorW(hinstance: HINSTANCE, lpcursorname: windows_sys::core::PCWSTR) -> HCURSOR;
```

The `windows_sys::core::PCWSTR` reference forces a dependency on `windows-sys`.
The workaround is `--deps none`, which inlines `PCWSTR` as `*const u16`.

### Root Cause in `paths.rs`

```rust
fn write_specific(&self, specific: &str) -> TokenStream {
    if self.bindgen.style.is_sys() {
        if self.bindgen.layout.is_package() || self.bindgen.deps != DepMode::None {
            quote! { windows_sys::core:: }   // ← forces windows-sys dep
        } else if self.bindgen.layout.is_flat() {
            quote! {}                         // ← only with --deps none
        } ...
    }
}
```

The `windows_sys::core::` path exists **solely** to support `--package` mode
(generating the `windows-sys` umbrella crate itself). For standalone `--sys`
bindings, this path should never be used — the user wants either inline types
(`--deps none`) or references to the small crates.

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

### Phase 1: Fix `--sys` Default Dependencies

**Goal:** `--sys` should not reference `windows-sys` by default.

For `--sys` mode, the right default is `--deps none` — sys bindings are raw
C-style FFI that should only need `windows-link`. Types like `PCWSTR` become
`*const u16`, `HRESULT` becomes `i32`, etc. (This is what `write_no_deps()`
in `types/mod.rs` already implements.)

For non-sys modes (`--minimal`, `Style::Default`), the right default is to
depend on the small crates (`windows-core`, `windows-time`, etc.) — the current
`--deps core` behavior with implicit references is correct.

**Concrete change:** When `--sys` is set and `--deps` is not explicitly
specified, default to `DepMode::None` instead of `DepMode::Core`.

**Impact:**
- `windows-sys` umbrella crate (`sys.txt`) must add explicit `--deps core`
- Bootstrap crates (`core.txt`, `result.txt`, `strings.txt`) already use
  `--deps none` — no change
- External sys users get standalone bindings by default ✓
- Non-sys users (reactor, canvas, default mode) are unaffected

### Phase 2: Unify `--sys` and `--minimal` (Medium Risk)

`--sys` and `--minimal` serve very similar purposes — both produce lean
bindings that strip ergonomic wrappers. The key difference:

| Aspect | `--sys` | `--minimal` |
|--------|---------|-------------|
| Free functions | `link!` macro only | `link!` macro only |
| Handles | Bare `pub type` alias | Bare `pub type` alias |
| COM interfaces | Vtable struct + IID only | Full interface type with demoted slots |
| WinRT classes | Not supported | Factory/activation helpers |
| Events | Not supported | Auto-revoking wrappers |
| Dependencies | `windows-link` only | `windows-core` + small crates |
| Type closure | All deps of filtered types | Only deps of requested methods |

**Code overlap analysis** (in `types/*.rs`):

- 6 sites check `is_sys() || is_minimal()` — identical behavior in both modes
  (handle aliases, function wrappers, vtable visibility, enum flattening)
- 28 total `is_sys()` checks — sys-specific paths (skip interface wrappers,
  skip WinRT traits, emit IID constants differently)
- 41 total `is_minimal()` checks — minimal-specific paths (event auto-revoking,
  factory methods, delegate optimization, method demotion)

**Possible unification:** Replace the `Style` enum with a single "lean" mode
that has a COM support level:

```rust
enum Style {
    Default,            // full ergonomic wrappers (umbrella crate use only)
    Lean {
        com: bool,      // false = today's --sys, true = today's --minimal
        extern_fns: bool,
    },
}
```

Or more simply: make `--sys` and `--minimal` composable rather than mutually
exclusive. `--sys` controls the FFI surface (link! vs wrappers), `--minimal`
controls the COM/WinRT surface (demoted slots, auto-revoking events). Used
together, you get the leanest possible bindings.

**Benefits:**
- One concept for users: "use lean bindings, add COM support if needed"
- ~6 `is_sys() || is_minimal()` sites collapse to `is_lean()`
- `MinimalFilter`'s method-level closure becomes available for sys mode too
- `--deps` inference becomes clearer: lean without COM = `none`, lean with COM = `core`

### Phase 3: Separate Umbrella Crate Tooling (Medium Risk)

The `--package`, `--index`, `--reference`, and `ReferenceStyle::SkipRoot`
features exist exclusively for generating the `windows` and `windows-sys`
umbrella crates. They add significant complexity to `windows-bindgen`:

- `package_writer.rs` — 196 lines for namespace-per-file + Cargo.toml features
- `index.rs` — 141 lines for `features.json` emission
- `references.rs` — 112 lines for cross-crate type delegation
- `paths.rs` — the `windows_sys::core::` path exists solely for package mode
- `lib.rs` — implicit reference registration block (~80 lines)

**Options:**

1. **Feature-gate** — put `--package` / `--index` behind a cargo feature so
   they don't affect the default binary size or mental model.

2. **Separate binary** — move umbrella crate generation to a dedicated
   `tool_package` or similar, leaving `windows-bindgen` focused on the
   encouraged use cases (flat + sys, flat + minimal).

3. **Document as internal** — at minimum, mark these options as internal /
   advanced in the docs and CLI help.

**Note on `--reference`:** Unlike `--package` and `--index`, the `--reference`
option is used by 8 tests/samples (e.g., `xaml_app`, `webview`,
`collection_interop`) for cross-crate type delegation. It may need to remain
in the public API even if `--package` is separated. However,
`ReferenceStyle::SkipRoot` is only used by the umbrella crates and could be
removed from the public surface.

**Impact on `paths.rs`:** Once `--package` is separated, `write_specific()`
simplifies dramatically:

```rust
fn write_specific(&self, specific: &str) -> TokenStream {
    match self.bindgen.deps {
        DepMode::None if self.bindgen.layout.is_flat() => quote! {},
        DepMode::None => /* super:: chain for module layout */,
        DepMode::Specific => format!("{specific}::").parse().unwrap(),
        DepMode::Core => quote! { windows_core:: },
    }
}
```

The entire `is_sys()` branch (with its `windows_sys::core::` path) disappears.

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

| | `--flat` | `--package` | `--sys` | `--minimal` | `--extern` | `--deps` |
|-|----------|-------------|---------|-------------|------------|----------|
| `--flat` | — | ✗ | ✓ | ✓ | ✓¹ | ✓ |
| `--package` | ✗ | — | ✓ | ✓ | ✓¹ | ✓ |
| `--sys` | ✓ | ✓ | — | ✗² | ✓ | ✓ |
| `--minimal` | ✓ | ✓ | ✗² | — | ✗ | ✓ |
| `--deps none` | ✓ | ✓ | ✓ | ✓ | ✓ | — |

¹ Requires parent option.
² Currently mutually exclusive — Phase 2 proposes making them composable.

---

## Common Invocation Patterns

### Standalone Win32 bindings (most common external use case)

```
--out src/bindings.rs --flat --sys --filter LoadCursorW
```

**After Phase 1:** `--sys` defaults to `--deps none`, so this Just Works™ —
no `windows-sys` dependency, only `windows-link`.

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
--out crates/libs/windows --package --deps core --index --filter Windows
--out crates/libs/sys --package --sys --deps core --filter Windows.Win32
```

These must explicitly opt in to `--deps core` for the `windows_core::` /
`windows_sys::core::` paths.

---

## Migration Checklist

### Phase 1 (fix `--sys` default deps)

- [ ] Make `--sys` default to `DepMode::None` when `--deps` is not explicitly set
- [ ] Add `--deps core` to `crates/tools/bindings/src/sys.txt`
- [ ] Verify bootstrap crates (`core.txt`, `result.txt`, `strings.txt`, etc.)
      already have `--deps none` — no change needed
- [ ] Verify non-sys filter files (`canvas.txt`, `animation.txt`, `collections.txt`,
      `future.txt`, `numerics.txt`, `metadata.txt`, `time.txt`, `windows.txt`)
      are unaffected (they don't use `--sys`)
- [ ] Update `docs/bindgen.md` to document the new default
- [ ] Update `lib.rs` doc comments for `--deps`
- [ ] Run `cargo run -p tool_bindings` and verify output is unchanged
- [ ] Close #4581

### Phase 2 (unify sys + minimal)

- [ ] Audit the 28 `is_sys()` and 41 `is_minimal()` call sites
- [ ] Identify which checks can collapse to `is_lean()` (~6 sites)
- [ ] Identify which need the COM distinction (separate `has_com()`)
- [ ] Prototype the unified `Style` enum
- [ ] Verify bit-for-bit output equivalence for all existing filter files
- [ ] Make `MinimalFilter` method closure available for sys mode

### Phase 3 (separate umbrella tooling)

- [ ] Feature-gate or extract `--package`, `--index`, `--reference`
- [ ] Remove `windows_sys::core::` path from `paths.rs` (moves to package tool)
- [ ] Simplify `write_specific()` to the 4-branch `(deps)` match
- [ ] Remove `ReferenceStyle::SkipRoot` (only needed by umbrella crates)

### Phase 4 (unify filters)

- [ ] Prototype single filter implementation using `MinimalTypeMap` closure
- [ ] Benchmark type closure for `windows` crate filter (must not regress)
- [ ] Verify bit-for-bit output for all modes
- [ ] Remove `filter.rs` / `minimal_filter.rs` duplication

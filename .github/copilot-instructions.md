# Copilot Instructions for windows-rs

Read this file at the start of every session. It contains the essential commands, conventions, and
architecture knowledge needed to work on this repository.

## Workflow

Do not create git commits automatically. The maintainer reviews all changes first and handles
commits manually. Make changes locally, run fmt/clippy/tests to verify, and report back.

## Repository Layout

Cargo workspace (`resolver = "3"`). Members are globbed from:

- `crates/libs/*` - the published/library crates (`windows`, `windows-sys`, `windows-core`, plus
  `windows-bindgen`, `metadata`, `rdl`, and the newer `reactor`/`canvas`/`webview`/`window` crates).
  See `docs/readme.md` for the full categorized crate index, and `docs/crates/<crate>.md` per crate.
- `crates/tools/*` - code generators and CI helpers, run via `cargo run -p tool_*`.
- `crates/tests/*/*` - test crates; `crates/tests/libs/<crate>` mirrors library crates where a
  separate fixture is useful (e.g. `test_webview`). Crate names are generally `test_<dir>`.
- `crates/samples/*/*` - runnable examples.

The crates fall into rough groups (see `docs/readme.md` for the authoritative list): core & errors
(`windows-core`, `windows-result`, `windows-strings`); values & collections (`numerics`,
`collections`, `reference`, `time`); async & threading (`future`, `threading`); system services
(`registry`, `services`, `version`); COM macros & linking (`implement`, `interface`, `link`); UI &
graphics (`reactor`, `canvas`, `webview`, `window`, `animation`, `reactor-setup`); codegen &
metadata tooling (`bindgen`, `metadata`, `rdl`, `cppwinrt`); and the full API projection (`windows`,
`windows-sys`).

## Before Finalizing Changes

```sh
cargo fmt --all
```

CI enforces rustfmt. Always format before finalizing changes. On Windows, the workspace-wide
command may exceed the process command-line limit; use `cargo fmt -p <crate>` for each affected
crate instead.

## Build & Test Commands

### Core crates

```sh
cargo check -p windows-core --quiet
cargo check -p windows --quiet
cargo clippy -p <crate> --all-targets
cargo test -p <crate>

# Run a single test by name filter
cargo test -p <crate> <test_name_substring>
```

CI sets `RUSTFLAGS: -D warnings`, so any warning fails the build. Workspace-wide lints are
configured in the root `Cargo.toml` (`[workspace.lints]`) - clippy lints like
`uninlined_format_args`, `redundant_clone`, and `semicolon_if_nothing_returned` are promoted to
warnings and therefore enforced.

### Reactor

```sh
# Verify reactor compiles
cargo check -p windows-reactor --quiet

# Unit tests (headless, fast)
cargo test -p windows-reactor --quiet

# Integration tests (launches and closes WinUI windows through UI Automation)
powershell -File crates\tests\libs\reactor_selftest\native.ps1
powershell -File crates\tests\libs\reactor_selftest\native.ps1 -Case smoke

# Enforce coverage floors after producing target\reactor-coverage.json
cargo run -p tool_reactor_coverage --quiet -- target\reactor-coverage.json

# Clippy
cargo clippy -p windows-reactor --all-targets
```

### Canvas

```sh
cargo check -p windows-canvas --quiet
cargo test -p windows-canvas --quiet
cargo clippy -p windows-canvas --all-targets
```

### Full workspace

```sh
cargo run -p tool_clippy_all    # runs clippy across all crates
```

## Code Generation Pipeline

**Never hand-edit generated files.** Generated outputs are committed, and CI fails if regenerating
produces a diff (the `gen` workflow runs each `cargo run -p tool_*` and rejects any change; the
`test` workflow likewise fails if tests modify tracked files). After editing generators or filters,
re-run the tool and commit the result.

The core `windows` / `windows-sys` crates are generated from Windows metadata (`.winmd`) via
`windows-bindgen` (driven by `tool_package`). `windows-metadata` and `windows-rdl` support
reading/authoring that metadata. Reactor is hand-written and has no semantic generator. The other
binding pipelines include:

1. **`tool_bindings`** - reads filter `.txt` files from `crates/tools/bindings/src/` -> runs
   `windows-bindgen` -> generates `bindings.rs` in each crate:
   - `crates/libs/canvas/src/bindings.rs` (from `canvas.txt`)
   - `crates/libs/reactor/src/bindings.rs` (from `reactor.txt` and `reactor.rdl`)
   - `crates/libs/time/src/bindings.rs`, `numerics`, `reference`, etc.

2. **`tool_package`** - generates the published `windows` and `windows-sys` package crates using
   `--package` mode (per-namespace files + Cargo.toml features).

3. After regenerating, always verify: `cargo check -p <affected-crate> --quiet`

## Key Architecture Facts

### Crate relationships

- `windows-core` is the foundation - almost everything depends on it.
- `windows` is the umbrella crate that re-exports from `windows-core`, `windows-numerics`,
  `windows-time`, `windows-collections`, `windows-reference`, etc.
- `windows-reactor` depends on `windows-core` (not `windows`). Its implementation is hand-written;
  `tool_bindings` generates the private WinUI ABI projection from a fixed filter.
- `windows-canvas` similarly uses minimal bindings for D2D/DXGI/DWrite/WIC.
- `windows-animation` wraps Win32 UIAnimation Manager COM APIs.

### Reactor architecture

- The application protocol is in `crates/libs/reactor/src/app/`, and the WinUI runtime is in
  `crates/libs/reactor/src/winui/`.
- Controls, properties, event routing, and native adapters are hand-written. There is no Reactor
  semantic generator or generated control catalog.
- COM casts: classes Deref to their default interface (zero-cost). Only cast to non-default parent
  interfaces. The `Param` trait handles parent-class conversions automatically.

### Canvas architecture

- Canvas wraps D2D/DXGI behind safe Rust types (`GpuDevice`, `SwapChain`, `DrawingSession`,
  `PathBuilder`).
- `animated_canvas()` (reactor feature) renders on UI thread via `CompositionTarget::Rendering`.
- Device-lost recovery is automatic.

## Conventions

- **Panics**: Use `panic!` only for invariant violations. Use `diag::` helpers for missing features
  (warn in debug, no-op in release).
- **`.unwrap()` over `.expect("...")`** - the panic hook provides full context.
- **No `thread_local!` in app code** - use reactor hooks (`use_state`, `use_ref`) instead.
  `thread_local!` is reserved for framework plumbing.
- **Test naming**: Reactor model/unit tests live in `windows-reactor`; native integration tests use
  `test_reactor_selftest`. Canvas tests use WARP software rendering.

## Documentation

The `docs/` folder has one page per crate:

- **`docs/crates/<crate>.md`** - a single page per crate covering both usage and internals (how the
  crate is built and maintained: codegen pipeline, generated files, conventions). It links to the
  crate's own `readme.md` for the user-facing intro and quick example.
- **`crates/libs/<crate>/readme.md`** - the user-facing introduction with a quick example (also the
  crates.io / docs.rs landing).
- **`docs/readme.md`** - the documentation hub and crate index.

`docs/` also holds `contributing.md`, `code_of_conduct.md`, and `security.md`.

When making changes to a crate, update its `docs/crates/<crate>.md` page and its `readme.md` as
needed. For example, `windows-reactor` changes touch `docs/crates/windows-reactor.md` (application
model, native adapters, threading, testing, and maintenance) and
`crates/libs/reactor/readme.md` (getting started and the quick example).

## Writing Style for Docs and Comments

These rules were established while cleaning up the docs and code comments and apply to all Markdown
(`.md`) and Rust comments/doc-comments across the repo. Keep new writing consistent with them.

- **ASCII punctuation only.** Use `-` for dashes (never em/en-dashes), `...` for ellipsis, `->` for
  arrows, `<=`/`>=`/`!=` for comparisons, and straight quotes. Drop the section sign from standard
  references (write `ECMA-335 II.22`, `C11 6.4.4.1`, not `\u00a7...`). The only non-ASCII that stays
  is genuine test data (e.g. a Greek-letter string literal exercising UTF-8 handling).
- **100-column wrap.** Hard-wrap Markdown prose and long comment blocks at 100 columns. Keep the
  wrap consistent within a file - do not mix wrapped and unwrapped paragraphs.
- **No LLM tells.** Avoid the vocabulary and tics that mark agent-written text: `faithful`,
  `corpus`, `ledger`, `crucially`, `notably`, `essentially`, `robust`, `seamless`, `comprehensive`,
  `deliberately`, `simply`, `under the hood`, `that said`, `importantly`, `conceptually`,
  `effectively`, `single source of truth`, `industry-standard`, `first-class`, `leverage`,
  `utilize`, `Note that ...`. Say the concrete thing instead. `ergonomic` and rustdoc `**bold**` are
  fine.
- **No decorative formatting.** No box-drawing banner comments (`// -- Title ------`), no duplicated
  doc paragraphs (a real copy-paste tell), no filler that restates the code.
- **Prefer tables over wordy paragraphs** where the content is a set of parallel cases (rules,
  mappings, options).
- **Describe the code as it is,** not its history. Avoid churn narration (`used to`, `previously`,
  `an earlier version`, `no longer`) unless it describes real runtime behavior, not codebase edits.
- **Do not edit generated files** to satisfy these rules; only hand-written sources.

## Open Investigations / TODO

Enduring record of known issues so they are not lost between sessions. Add findings here; remove or
mark done as they are addressed. Cite code by file + symbol name, not line number - line numbers go
stale fast.

### windows-clang: editorializing beyond the header

Context: issue [#4720](https://github.com/microsoft/windows-rs/issues/4720) and discussion
[#4725](https://github.com/microsoft/windows-rs/issues/4725) (the `SW_NORMAL` should-be-signed
report). The scrape aims to match the SDK headers, not a theoretical C-standard purity,
yet a few rules still rewrite or drop what the header literally declares. The fundamental-type
canonicalization (`DWORD`/`CHAR` -> primitives, `BOOLEAN` -> `bool`,
`LARGE_INTEGER`/`ULARGE_INTEGER` -> `i64`/`u64`, the D2D numerics) is settled and intended; the
items below are the ones still worth revisiting. Sources:
`crates/libs/clang/src/{const,canon,annotation,interface,lib}.rs`,
`crates/tools/win32/src/main.rs`, `docs/crates/windows-clang.md`.

**Active type rewrites - close to the header, but restate the declared type:**

- **Scalar collapse is a curated allowlist** (`canon.rs`: `fundamental_scalar`, `semantic_scalar`,
  `pointer_sized_abi`, `floating_typedef`, `guid_alias`, `void_pointer_alias`, `d2d_compat_alias`).
  `DWORD` -> `u32` - and now `BOOLEAN` -> `bool`, `LARGE_INTEGER` -> `i64` - erase the named alias,
  while `COLORREF`/`ATOM`/`HFILE` (byte-identical underlying types) stay named. The split is
  editorial. Fix: emit every typedef as a named alias in the winmd and move the collapse to the
  Layer-B `windows-bindgen` projection, where it is an ergonomics choice, not a metadata fact.
- **String-alias normalization + SAL const-flip** (`canon.rs`: `normalize_string_alias`,
  `apply_sal_constness`, `promote_null_terminated_string`). `LPCWSTR` -> `PCWSTR`, and `_In_ LPWSTR`
  -> `PCWSTR` flips a mutable pointer to const from the `_In_` bit. Declared type is mutable; the
  emitted type is const. Fix: keep the header's pointer/const-ness; expose SAL direction as a
  separate attribute.
- **`[iid_is]` inferred from parameter name** (`annotation.rs` `infer_iid_is`,
  `IID_SELECTOR_PARAM_NAMES = ["riid","iid","riidltf"]`). An un-annotated `_COM_Outptr_ IUnknown**`
  becomes `void**` + `[iid_is]` because a sibling parameter is named `riid` (5 methods across 4
  functions). The header never expressed the linkage. Fix: a "source-annotations-only" mode that
  honors only an explicit MIDL `[iid_is]` comment.
- **`D2D1_*` -> `D2D_*` compat collapse** (`canon.rs` `d2d_compat_alias`) - curated erasure of the
  `D2D1_`-spelled alias the header declares. Same remedy as the scalar collapse.

**Dropped / reshaped header content - coverage opinions:**

- **Redundant-constant dropping** (`lib.rs` final pass) - drops a top-level constant whose name and
  value match an enumerator elsewhere. Only needed because of the flat namespace below.
- **Single flat `Windows.Win32` namespace** - lossy for genuine name collisions the reference
  disambiguated by sub-namespace (`PID_SECURITY`, the `E_NOTFOUND` HRESULT-vs-`#define` class). Fix:
  on a true USR/value collision, keep both under a disambiguating suffix instead of dropping.
- **`UNICODE`/`_UNICODE` not defined** - the TU is built ANSI-default. Higher-coverage (defining
  `UNICODE` drops 71 bare-ANSI exports) but the generic-text typedefs follow the ANSI branch.
  Fix (large, what the reference does): scrape ANSI and Unicode in two passes and merge.
- **Orphan named-type dropping** - a type an in-scope header declares but no emitted signature
  references is dropped (`PROCESSOR_POWER_INFORMATION`, `FIRMWARE_TABLE_PROVIDER`,
  `PROCESSOR_FEATURE_ID`). Fix: emit all named types defined in a `HEADERS` file, not only the
  reachability closure.
- **`intsafe.h` exclusion, `drop_lib_less`, `vertdll` ordering** (`tool_win32`) - pragmatic drops
  and relinks of content the headers/libs provide. Low priority, defensible.

**Correct as-is (do not "fix"):** overloaded-virtual vtable reversal (`interface.rs`, reproduces the
true MSVC vtable slot order - ABI-critical); the `_HRESULT_TYPEDEF_`/`_NDIS_ERROR_TYPEDEF_`
cast-wrapper map (`const.rs` `cast_wrapper_macro`, honors an explicit author type annotation).

Suggested order: `UNICODE` two-pass (largest fidelity win), then the `[iid_is]` name-gate,
orphan-type retention, the scalar/string/D2D collapses (move to Layer B), then flat-namespace
disambiguation.

### windows-clang: suppress-definition coupling

`lib.rs` (`StructDecl`/`UnionDecl`) and `typedef.rs` consult the `canon.rs` collapse views
(`numerics_alias`, `semantic_scalar`, the scalar/guid/void views) to skip emitting a collapsed
type's definition, mirroring the reference-site collapses in `alias_collapse`. The name lists are
not duplicated (both sides read the shared table), but nothing guarantees a collapse row and its
suppression site stay paired. A suppress-definition column on `Collapse` carrying the cursor kind
(typedef/struct/union) each row suppresses would make the pairing a compiler-checked fact.

Output-neutrality check for any clang change: regenerate the two scrape consumers - `tool_win32`
(flat, `write_by_header`, both the um and km scrapes) and `tool_webview` (namespaced, `write`) - and
confirm `git diff` shows no generated-file changes. `tool_bindings`/`tool_package`/`tool_features`
derive from the winmds, so an unchanged winmd proves them unchanged too.

### Repo-wide dead-code / quality audit (2026-07)

Open items across the hand-written crates (reactor, bindgen, rdl, clang, canvas, metadata, webview,
core) that need a design decision or a larger change. Any bindgen source change must be proven
output-neutral by running the `tool_*` generators and confirming `git diff` shows no generated-file
changes (the `gen` workflow enforces this).

#### Behavioral / correctness (need a design decision)

| Location | Issue |
| --- | --- |
| `webview` `pump.rs` (`WM_QUIT` arm) | `Err(Error::empty())` reports a success `HRESULT(0)` (the empty sentinel maps back to 0). Intentional but easy to misread as success. |

#### Duplication / refactor candidates

| Location | Issue |
| --- | --- |
| `metadata` `merge/mod.rs` `write_type` vs `merge/remap.rs` `write_type` | Two ~60-line structurally identical functions; the remap copy's comment says it mirrors `merge::write_type`. Any new ECMA table must be added to both, with no compiler guard. |
| `canvas` `session.rs` gradient-stop + bitmap-properties builders | Duplicated ABI-stop and bitmap-properties construction across the brush paths. |
| `bindgen` `types/interface.rs` + `cpp_interface.rs` local `fn combine` | Duplicate local helper. |
| `canvas` `color.rs` `DARK_SLATE_BLUE` | `rgb(0.05, 0.05, 0.1)` does not match the CSS color of that name (public API used by samples). |

#### Coverage gaps

| Area | Gap |
| --- | --- |
| `metadata` `Remapper` (`merge/remap.rs`) | No tests anywhere; routing/`split_apis` logic is exercised only in the live build, so a regression yields a malformed namespaced winmd with no failing test. |
| webview | `process-failed`, download, and deferral paths untested. |

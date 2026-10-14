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
- `crates/tools/*` - code generators and CI helpers, run via `cargo run -p tool-*`.
- `crates/tests/*/*` - test crates; `crates/tests/libs/<crate>` mirrors each library crate (e.g.
  `test_reactor`, `test-webview`). Crate names are `test_<dir>`.
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
# Refresh WinUI metadata after changing Windows App SDK or WebView2 pins
cargo run -p tool-reactor-metadata --quiet

# Regenerate Reactor code after editing schema.toml or tool-reactor source
cargo run -p tool-reactor --quiet

# Regenerate bindings (after editing filter .txt files)
cargo run -p tool-bindings --quiet

# Verify reactor compiles
cargo check -p windows-reactor --quiet

# Unit tests (headless, fast)
cargo test -p windows-reactor --all-features --quiet

# Integration tests (launches WinUI window)
cargo run -p test-reactor-selftest
cargo run -p test-reactor-selftest -- --headless    # CI mode
cargo run -p test-reactor-integration --bin application_menu
cargo run -p test-reactor-integration --bin canvas_integration
cargo run -p test-reactor-integration --bin webview_integration
cargo run -p test-reactor-integration --bin window_state

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
cargo run -p tool-clippy-all    # runs clippy across all crates
```

## Code Generation Pipeline

**Never hand-edit generated files.** Generated outputs are committed, and CI fails if regenerating
produces a diff (the `gen` workflow runs each `cargo run -p tool-*` and rejects any change; the
`test` workflow likewise fails if tests modify tracked files). After editing generators or filters,
re-run the tool and commit the result.

The core `windows` / `windows-sys` crates are generated from Windows metadata (`.winmd`) via
`windows-bindgen` (driven by `tool-package`). `windows-metadata` and `windows-rdl` support
reading/authoring that metadata. The reactor / canvas / webview pipelines layer on top:

1. **`tool-reactor-metadata`** - refreshes committed WinUI / Windows App SDK / WebView2 `.winmd`
   metadata and generates `extras.winmd`.

2. **`tool-reactor`** - reads `crates/tools/reactor/src/schema.toml` plus the shared WinUI
   metadata -> generates Reactor declarations, native projection code, the Reactor native
   bindings, and the canvas Reactor bridge bindings.

3. **`tool-bindings`** - reads filter `.txt` files from `crates/tools/bindings/src/` -> runs
   `windows-bindgen` -> generates `bindings.rs` in each crate:
   - `crates/libs/canvas/src/bindings.rs` (from `canvas.txt`)
   - `crates/libs/time/src/bindings.rs`, `numerics`, `reference`, etc.

4. **`tool-package`** - generates the published `windows` and `windows-sys` package crates using
   `--package` mode (per-namespace files + Cargo.toml features).

4. After regenerating, always verify: `cargo check -p <affected-crate> --quiet`

## Key Architecture Facts

### Crate relationships

- `windows-core` is the foundation - almost everything depends on it.
- `windows` is the umbrella crate that re-exports from `windows-core`, `windows-numerics`,
  `windows-time`, `windows-collections`, `windows-reference`, etc.
- `windows-reactor` depends on `windows-core` (not `windows`) and uses minimal bindings generated
  with `--minimal --flat` mode.
- `windows-canvas` similarly uses minimal bindings for D2D/DXGI/DWrite/WIC.
- `windows-animation` wraps Win32 UIAnimation Manager COM APIs.

### Reactor architecture

- WinUI backend ownership is split across `crates/libs/reactor/src/native/winui.rs` and
  `crates/libs/reactor/src/native/winui/`.
- The TOML config (`crates/tools/reactor/src/schema.toml`) declares 79 WinUI controls. Keys are
  WinUI metadata names; the tool infers types, setter patterns, and event handlers from `.winmd`
  files.
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
- **No `thread_local!` in app code** - keep state in components and pass shared values through typed
  contexts. `thread_local!` is reserved for framework plumbing.
- **Test naming**: Reactor unit tests live in `windows-reactor`; live WinUI coverage lives in
  `test-reactor-selftest`. Public application and companion-crate fixtures live in
  `test-reactor-integration` without the Reactor `test` feature. Build that package separately
  from privileged tests to avoid Cargo feature unification. Canvas tests use WARP rendering.

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
needed. For example, `windows-reactor` changes touch `docs/crates/windows-reactor.md` (codegen,
TOML, threading, COM pitfalls, plus the conceptual overview) and `crates/libs/reactor/readme.md`
(getting started and the quick example).

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

### Repo-wide dead-code / quality audit (2026-07)

Open items across the hand-written crates (reactor, bindgen, rdl, clang, canvas, metadata, webview,
core) that need a design decision or a larger change. Any bindgen source change must be proven
output-neutral by running the `tool-*` generators and confirming `git diff` shows no generated-file
changes (the `gen` workflow enforces this).

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

### clang2 rapid iteration

Drive the rewrite toward complete, correct canonical metadata with a short feedback loop:

1. Pick a concrete metadata difference or root omission and add a focused header/RDL fixture
   establishing the intended result.
2. Fix the responsible capture, reconciliation, or projection layer. If a feature requires
   scattered exceptions or conflicting interpretations, correct that boundary before continuing.
3. Regenerate `metadata/win32` and leave the generated changes in the worktree for review. Check
   intended gains, unexpected losses, and representative generated Rust behavior across targets.
   Do not restore generated output merely because the run was partial.
4. Compare root omissions, dependent losses, time, and memory with the preceding iteration.
   Diagnostic counts include dependency cascades; uncaptured macros are not counted.

Partial runs and passing fixtures are progress evidence, not replacement readiness. Large deletion
diffs remain outstanding work. Review architectural pressure points as coverage grows; do not
substitute repeated redesigns, arbitrary naming/routing rules, or weaker rejection for coverage.

### clang2 remaining attribute coverage

`clang2` `capture.rs` `attribute` uses native kind names, requested `MaxFieldAlignment` bits, and
evaluated `AlignedAttr` requests with their semantic form.
`test_clang2` `fidelity.rs` `opaque_packing` distinguishes `pack(1)` and `pack(2)` despite identical
char-only layouts. Reconciliation checks requested packing, and projection preserves it.
Complete-record alignment projects through `#[align(N)]`; request sets are checked even when
layouts match. Field, typedef, and enum alignment, combined packing/alignment, and packed
containment of aligned records remain unsupported.
The native `TypeVisibilityAttr` query decodes default/hidden/protected requests, including the
implicit tokenless attribute on `_GUID`. `clang2` retains and compares default requests; hidden
and protected requests remain unsupported. Supported plain records/enums do not project C++ RTTI
or vtables, so default type visibility needs no emitted attribute. Ordinary `Visibility` and
other undecoded attributes remain rejected. Do not identify unknown attributes from empty tokens
or infer requests from layout.

The runtime pin is `windows-clang-sys::LIBCLANG_VERSION`. LLVM 23.1.3's stock C API also lacks
the packing payload accessor.
Track runtime/resource-header upgrades and binding refreshes separately from declaration work.
The companion builds against a matching LLVM SDK and pinned ClangSharp sources. Its extra packing
query reads `MaxFieldAlignmentAttr`; ClangSharp's `getMaxAlignment` reads explicit alignment.

Independent declaration coverage can proceed while opaque attributes and their dependents remain
explicitly omitted. Resolve this boundary before claiming full coverage, or before a feature needs
those semantics. Do not relax rejection to increase output counts.

`windows-clang-sys` replaces `clang-sys` for both parser crates and helpers. Rebuild native changes
with `cargo run -p tool-clang-sys -- --native`, then regenerate with `cargo run -p tool-clang-sys`.
Helper provisioning builds missing DLLs but does not detect changed native source files.

Callback capture accepts plain C/stdcall prototypes after checking native function properties.
`test_clang2` `opaque_callbacks.h`
distinguishes ordinary C and x86 `regparm(2)` function-pointer types with a compiler `static_assert`.
Both expose `FunctionProto`, C calling convention, identical result/parameter types, and no
attribute cursor children. `noreturn` also survives in a function type without an attribute cursor.
Type spelling and `clang_equalTypes` distinguish extra semantics but do not decode their payloads.
Explicit `__cdecl`/`__stdcall` instead expose `CXType_Attributed`. Do not assume that rejecting
attributed types makes every remaining function prototype plain C.
The sys crate's `windows_clang_getFunctionInfo` recovers these hidden properties, including
explicit `regparm(0)` versus absence, and flags additional prototype information for rejection
or later decoding. `test_clang_sys` exercises these distinctions on all three parsing targets.
Explicit regparm (including zero), noreturn, variadics, exception-qualified functions, and unknown
properties remain rejected. Callback dependency pruning follows result and all parameter edges.
Failed value-alias or callback publication must remove the declaration, not just its root marker:
reachable named values are published as dependencies, so root-only removal would retry the failure.

`windows_clang_getTypeQualifiers` checks qualifiers before `clang2` type interning. CVR is
represented; unaligned and all remaining qualifiers are rejected, including typedef inheritance.
Callback naming uses target-wide candidates after public-family and alias validation. Candidate
searches use the shared native/public type checker and discard assumed pairs after each failure.

### clang2 replacement boundary

`clang2` `projection.rs` `Projector::written_ty` preserves written value-typedef and callback names
at use sites. Native agreement precedes public-name agreement across observations; conflicts are
errors, not representative-selection choices. `Projector::callback` uses positional parameter
names. Native agreement alone does not establish annotation correctness. Before a production
cutover, define intended names and annotations and check the header -> RDL -> WinMD -> generated Rust
path for representative consumers and issue
reproductions. Do not count explicit partial omissions or uncaptured macros as solved coverage.

### clang2 public ownership TODO

`clang2` `projection/plan.rs` `owner` requires a unique header for public declarations. Equivalent
typedefs such as `DWORD` can occur in multiple headers; preserving their names exposes this
ownership ambiguity in dependent declarations. Revisit a shared foundation module or RDL file
after the rewrite is proven. Do not add arbitrary per-name/header preferences or ownership routing
seeded from canonical metadata in the meantime.

`projection/plan.rs` `Plan::new` also selects the alphabetically first by-value alias as each
record's public name. The SDK visibility slice exposes `_GUID` becoming `BATTERY_CHARGER_ID` in
`poclass.rdl`, with `GUID`, `IID`, and `CLSID` referring to it. Native layout equality does not
establish canonical naming or ownership parity. Resolve primary naming from declaration evidence
and an explicit output contract; do not add per-name exceptions.

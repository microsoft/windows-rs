# Copilot Instructions for windows-rs

Read this file at the start of every session. It contains the essential commands, conventions, and
architecture knowledge needed to work on this repository.

## Workflow

Do not create git commits automatically. The maintainer reviews all changes first and handles
commits manually. Make changes locally, run fmt/clippy/tests to verify, and report back.

## Repository Layout

Cargo workspace (`resolver = "3"`). Members are globbed from:

- `crates/libs/*` - the published/library crates (`windows`, `windows-sys`, `windows-core`, plus
  `windows-bindgen`, `metadata`, `rdl`, `riddle`, and the newer
  `reactor`/`canvas`/`webview`/`window` crates). See `docs/readme.md` for the full categorized crate
  index, and `docs/crates/<crate>.md` per crate.
- `crates/tools/*` - code generators and CI helpers, run via `cargo run -p tool_*`.
- `crates/tests/*/*` - test crates; `crates/tests/libs/<crate>` mirrors each library crate (e.g.
  `test_reactor`, `test_webview`). Crate names are `test_<dir>`.
- `crates/samples/*/*` - runnable examples.

The crates fall into rough groups (see `docs/readme.md` for the authoritative list): core & errors
(`windows-core`, `windows-result`, `windows-strings`); values & collections (`numerics`,
`collections`, `reference`, `time`); async & threading (`future`, `threading`); system services
(`registry`, `services`, `version`); COM macros & linking (`implement`, `interface`, `link`); UI &
graphics (`reactor`, `canvas`, `webview`, `window`, `animation`, `reactor-setup`); codegen &
metadata tooling (`bindgen`, `metadata`, `rdl`, `riddle`, `cppwinrt`); and the full API projection
(`windows`, `windows-sys`).

## Before Finalizing Changes

```sh
cargo fmt --all
```

CI enforces rustfmt. Always format before finalizing changes.

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
# Regenerate codegen (after editing winui.toml or tool_reactor source)
cargo run -p tool_reactor --quiet

# Regenerate bindings (after editing filter .txt files)
cargo run -p tool_bindings --quiet

# Verify reactor compiles
cargo check -p windows-reactor --quiet

# Unit tests (headless, fast)
cargo test -p test_reactor --quiet

# Integration tests (launches WinUI window)
cargo run -p test_reactor_selftest
cargo run -p test_reactor_selftest -- --headless    # CI mode
cargo run -p test_reactor_selftest -- --filter Name  # single fixture

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
`windows-bindgen` (driven by `tool_package`). `windows-metadata`, `windows-rdl`, and `riddle`
support reading/authoring that metadata. The reactor / canvas / webview pipelines layer on top:

1. **`tool_reactor`** - reads `crates/tools/reactor/src/winui.toml` + WinUI `.winmd` metadata ->
   generates `generated.rs`, `generated_set_prop.rs`, `generated_attach_event.rs`, and
   `generated.txt` filter entries.

2. **`tool_bindings`** - reads filter `.txt` files from `crates/tools/bindings/src/` -> runs
   `windows-bindgen` -> generates `bindings.rs` in each crate:
   - `crates/libs/canvas/src/bindings.rs` (from `canvas.txt`)
   - `crates/libs/time/src/bindings.rs`, `numerics`, `reference`, etc.

3. **`tool_package`** - generates the published `windows` and `windows-sys` package crates using
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

- WinUI backend is in `crates/libs/reactor/src/backend/winui/`.
- The TOML config (`winui.toml`) declares ~52 WinUI controls. Keys are WinUI metadata names; the
  tool infers types, setter patterns, and event handlers from `.winmd` files.
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
- **Test naming**: Unit tests in `test_reactor`, integration tests in `test_reactor_selftest`.
  Canvas tests use WARP software rendering.

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

Enduring record of known issues to work on so they are not lost between sessions. Add findings here;
remove or mark done as they are addressed.

### windows-clang: subjective decisions that stray from a faithful header interpretation

Context: issue [#4720](https://github.com/microsoft/windows-rs/issues/4720) and discussion
[#4725](https://github.com/microsoft/windows-rs/issues/4725) (the `SW_NORMAL` should-be-signed
report). The scrape aims to be "faithful to the Windows SDK headers, not a theoretical C-standard
purity," but several decisions editorialize beyond what the header literally declares. Two notions
of "faithful" appear in the thread and agree everywhere except item 1: faithful to C semantics (a
token has an unambiguous type) vs faithful to the SDK header spellings. Sources studied:
`crates/libs/clang/src/{const,canon,annotation,interface,lib}.rs`, `crates/tools/win32/src/main.rs`,
`docs/crates/windows-clang.md`.

### Repo-wide dead-code / quality audit (2026-07)

Deep-dive after a month of churn across the hand-written crates (reactor, bindgen, rdl, clang,
canvas, metadata, webview, core). Findings were verified against source before acting. Safe,
output-neutral fixes were applied; behavioral and design questions are recorded below for a
decision. Remove each entry when it is addressed. Any bindgen source change must be proven
output-neutral by running the `tool_*` generators and confirming `git diff` shows no
generated-file changes (the `gen` workflow enforces this).

Completed: dead-code elimination in bindgen (`cpp_enum` dead `else`; `struct.rs` `is_sys()`
tautology; `interface.rs` empty-`if`; no-op `link_fmt`; redundant `Type::Class(_)` arm; unused
`_named_params`/`_reader`/`_style` params and the `for_style` wrapper plus the `config` params it
orphaned), four dead metadata reader accessors (`MethodDef::rva`, `GenericParam::owner`,
`InterfaceImpl::class`, `Constant::parent`), reactor `UiMarshaller::dispatch_low`, and a `sha1.rs`
`_offset` mis-name. All verified with build, tests, clippy, fmt, and a full 10-tool regen.
Follow-ups since: added SHA-1 known-answer tests (`core/src/imp/sha1.rs`), and removed the dead
`Some(1/2/5)` ABI arms from rdl `write_delegate` (only WinRT delegates reach it, so the arms were
unreachable and `#abi` always emitted nothing) plus fixed the stale `read_unmanaged_abi` doc.

#### Behavioral / correctness (need a design decision)

| Location | Issue |
| --- | --- |
| `reactor/src/style.rs:237` + `element.rs:730` | `exit_transition` is set but never read; `.transition(enter, exit)` silently discards the exit arg. Wire it up or drop the parameter. |
| `reactor/src/reconciler.rs:775` + `backend/winui/mod.rs:893` | Verified: a resource-dict change `{k:v}` -> `{}` never reaches the backend (the `&& !new.resources.is_empty()` guard skips it), and the backend handler only inserts map entries - it never removes - so any key removal leaves stale entries. Unlike the pointer/drag-handler paths above, which always emit on change and clear when empty. Needs a replace-vs-merge decision. |
| `reactor/src/reconciler/widget_dispatch.rs:238` | Verified: a `TabItem` key change `Some` -> `None` satisfies `o.key != n.key` but the `&& let Some(key) = &n.key` guard skips the body, so the stale key is retained. `is_closable`/`header` just below always emit on change. Needs a backend clear path for `Prop::ItemKey`. |
| `webview/src/pump.rs:36` | `Err(Error::empty())` on `WM_QUIT` reports a success `HRESULT(0)` (the empty sentinel maps back to 0). Intentional but easy to misread as success. |

#### Duplication / refactor candidates

| Location | Issue |
| --- | --- |
| `metadata/src/merge/mod.rs:218` (`write_type`) vs `merge/remap.rs:170` (`Remapper::write_type`) | Two ~60-line structurally identical functions; the remap copy's comment even says it mirrors `merge::write_type`. Any new ECMA table must be added to both, with no compiler guard. |
| `canvas/src/session.rs` (211-217, 236-242, 394-404) | Duplicated gradient-stop and bitmap-properties builders. |
| `bindgen/src/types/interface.rs:559` + `cpp_interface.rs:215` | Duplicate local `fn combine()`. |
| `canvas/src/color.rs:57` | `DARK_SLATE_BLUE = rgb(0.05, 0.05, 0.1)` does not match the CSS color of that name (public API used by samples). |

#### Coverage gaps

| Area | Gap |
| --- | --- |
| `window` crate | Zero tests and no `crates/tests/libs/window` test crate. |
| `metadata` `Remapper` (`merge/remap.rs`) | No tests anywhere; routing/`split_apis` logic is exercised only in the live build, so a regression yields a malformed namespaced winmd with no failing test. |
| webview | `process-failed`, download, and deferral paths untested. |

**Tier 1 - strays from both the C standard and the header:**

1. **Non-negative `#define` constants default to unsigned.** DONE. `const.rs` now types
   each integer constant by the C11 literal rule (`integer_value`, `c_integer_constant_type`):
   unsuffixed decimal takes the first of `int/long/long long` that fits (`1`->`i32`); hex/octal
   takes the first of `int/uint/long/ulong/...` that fits (`0x80000000`->`u32`); `U`/`L`/`LL`
   suffixes are honored. This makes `SW_NORMAL` and other `SW_*` signed (`i32`) while hex flag
   masks stay unsigned, resolving #4725. The retyping is value-preserving: across the full win32
   scrape, 673 constants changed their spelled type but every one keeps the same bit pattern
   (verified by modular arithmetic), and cases like `BG_LENGTH_TO_EOF` (`(UINT64)(-1)`) are now
   `u64` max instead of a truncated `i32`. Hand-written call sites across the workspace were
   updated with explicit `as u32`/`as i32` casts where Win32 ABI conventions use `DWORD`(u32).

**Tier 2 - faithful-ish to the header, but an active rewrite of the declared type:**

2. **Scalar typedef collapse is a curated allowlist** (`canon.rs` `fundamental_scalar` 289-305,
   `pointer_sized_abi`, `floating_typedef`, `guid_alias`, `void_pointer_alias`, `d2d_compat_alias`).
   `DWORD`->`u32` erases the named alias, but `COLORREF`/`ATOM`/`HFILE` (same underlying types) are
   kept named. The header declares them identically; the split is editorial.
   - Fix: emit every typedef as a named alias in the winmd and move the collapse to the Layer-B
     projection (`windows-bindgen`) where it is an ergonomics choice, not a metadata fact.
3. **String-alias normalization + SAL const-flip** (`canon.rs` rules 1/9/10, `alias_policy`).
   `LPCWSTR`->`PCWSTR`, and `_In_ LPWSTR`->`PCWSTR` flips a non-const pointer to const from the
   `_In_` annotation. Declared type is mutable; emitted type is const.
   - Fix: keep the header's declared pointer/const-ness; expose SAL direction as a separate
     attribute.
4. **`[iid_is]` inferred from parameter name** (`annotation.rs` `infer_iid_is` 367+,
   `IID_SELECTOR_PARAM_NAMES = ["riid","iid","riidltf"]`). An un-annotated
   `_COM_Outptr_ IUnknown **` is rewritten to `void**` + `#[iid_is]` because a sibling parameter is
   named `riid` (5 methods across 4 functions). The header never expressed the linkage.
   - Fix: offer a "source-annotations-only" mode that honors only an explicit MIDL `[iid_is]`
     comment.
5. **`D2D1_*`->`D2D_*` compat collapse** (`canon.rs` `d2d_compat_alias` 363-383) - curated erasure
   of the `D2D1_`-spelled alias the header declares. Same remedy as item 2.

**Tier 3 - coverage / configuration opinions that drop or reshape real header content:**

6. **Redundant-constant dropping** (`lib.rs` final pass) - drops a top-level constant whose name and
   value match an enumerator elsewhere. Only needed because of item 7.
7. **Single flat `Windows.Win32` namespace** - lossy for genuine name collisions the reference
   disambiguated by sub-namespace (`PID_SECURITY`, the `E_NOTFOUND` HRESULT-vs-`#define` class).
   - Fix: on a true USR/value collision, keep both under a disambiguating suffix instead of
     dropping.
8. **`UNICODE`/`_UNICODE` not defined** - the TU is built ANSI-default. This is the direct answer to
   "what is the equivalent of `#include <windows.h>`?"; a normal app build defines `UNICODE`. The
   choice is higher-coverage (defining `UNICODE` drops 71 bare-ANSI exports) but the emitted
   generic-text typedefs follow the ANSI branch.
   - Fix (biggest lift, what the reference does): scrape ANSI and Unicode in two passes and merge.
9. **Orphan named-type dropping** (reachability, no header-scoped retention) - a type an in-scope
   header declares but no emitted signature references is dropped (`PROCESSOR_POWER_INFORMATION`,
   `FIRMWARE_TABLE_PROVIDER`, `PROCESSOR_FEATURE_ID`).
   - Fix: emit all named types *defined in* a `HEADERS` file, not only the reachability closure.
10. **`intsafe.h` exclusion, `drop_lib_less`, `vertdll` ordering** (`tool_win32`) - pragmatic drops/
    relinks of content the headers/libs literally provide. Low priority, defensible.

**Correct as-is (do not "fix"):** overloaded-virtual vtable reversal (`interface.rs`, reproduces the
true MSVC vtable slot order - ABI-critical); the `_HRESULT_TYPEDEF_`/`_NDIS_ERROR_TYPEDEF_`
cast-wrapper map (`const.rs` `cast_wrapper_macro` 983-987, honors an explicit author type
annotation).

Suggested order: item 1 first (the actual bug, moves toward both notions of faithful, contained
change), then 8 (true `windows.h` fidelity, large), then 4 / 9 / 2-3-5 / 7.

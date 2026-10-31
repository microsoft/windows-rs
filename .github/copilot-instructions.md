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
  See `docs/readme.md` for the categorized index and each crate's `readme.md` for getting started.
- `crates/tools/*` - code generators and CI helpers, run via `cargo run -p tool-*`.
- `crates/tests/*/*` - test crates; `crates/tests/libs/<crate>` mirrors each library crate. Check
  its `Cargo.toml` for the package name, such as `test_clang` or `test-reactor`.
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

# Public API tests (x64 interactive desktop; Cargo deploys the runtime)
cargo test -p test-reactor
cargo test -p test-reactor --test ui window::

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
re-run the tool and leave the generated changes with the source changes for review.

For behavior-preserving generator refactors, regenerated output should remain unchanged. Review
intentional output changes against focused fixtures rather than treating every diff as a failure.

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

After regenerating, verify: `cargo check -p <affected-crate> --quiet`.

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

- WinUI backend ownership is split across `crates/libs/reactor/src/native/winui/mod.rs` and
  `crates/libs/reactor/src/native/winui/`.
- The TOML config (`crates/tools/reactor/src/schema.toml`) declares WinUI controls. Keys are WinUI
  metadata names; the tool infers types, setter patterns, and event handlers from `.winmd` files.
- COM casts: classes Deref to their default interface (zero-cost). Only cast to non-default parent
  interfaces. The `Param` trait handles parent-class conversions automatically.

### Canvas architecture

- Canvas wraps D2D/DXGI behind safe Rust types (`GpuDevice`, `SwapChain`, `DrawingSession`,
  `PathBuilder`).
- `animated_canvas()` (reactor feature) renders on UI thread via `CompositionTarget::Rendering`.
- Device-lost recovery is automatic.

## Conventions

- **Module layout**: When a module has a directory, put its entry point in `name/mod.rs`, not
  `name.rs` beside the `name/` directory. The workspace enables Clippy's `self_named_module_files`
  lint to enforce this for modules with external submodules.
- **Fail fast**: Contract violations, internal bugs, and unexpected native failures panic
  (`.unwrap()` or `panic!`); Reactor does not roll back or poison state. Return `Result` only where
  the caller can act: startup through the `App::run_*` entry points and imperative completions such
  as `FocusError`. A dispatcher that rejects work is shutting down; drop the work rather than
  failing.
- **`.unwrap()` over `.expect("...")`** - the panic hook provides full context.
- **No `thread_local!` in app code** - keep state in components and pass shared values through typed
  contexts. `thread_local!` is reserved for framework plumbing.
- **Test naming**: Reactor unit tests live in `windows-reactor`. Public behavioral coverage belongs
  in `test-reactor`. Prefer a small set of high-value contracts over exhaustive UIA matrices or
  preserving every legacy assertion. Do not add separate runners or test scripts. Keep library tests
  headless rather than adding ignored native fixtures. The functional crate participates in
  workspace tests and uses `windows-reactor-setup` in its build script. Live UI cases run on x64;
  setup's filesystem and multiprocess tests run on all Windows architectures.
  Run the crate alone when checking default features rather than the workspace's unified
  features.
  Canvas tests use WARP rendering.
- **Reactor test documentation**: Keep testing instructions in
  `crates/tests/libs/reactor/readme.md`, not the library README. `cargo test -p test-reactor`
  must run the automated suite without ignored-test flags. Generate its native bindings with
  `windows-bindgen` in `build.rs` rather than depending on the `windows` crate.

## Documentation

Each crate has one maintained getting-started page:

- **`crates/libs/<crate>/readme.md`** - purpose, setup, a working example, essential usage
  constraints, and links to API docs and samples. This is also the crates.io landing and, for most
  crates, the rustdoc introduction.
- **`docs/readme.md`** - the documentation hub and crate index.

`docs/` also holds `contributing.md`, `code_of_conduct.md`, and `security.md`.

Keep READMEs concise and user-facing. Preserve the standard emoji link headers. Use links that
work on GitHub, crates.io, and rustdoc. Update a README when setup or essential usage changes,
not for every fix or addition. Avoid implementation walkthroughs and change journals.
Put detailed API behavior in rustdoc and testing instructions beside the tests. Keep substantial
references, such as `crates/libs/rdl/rdl.md`, separate without duplicating getting-started material.

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

## Clang and Metadata

Use the [header annotation reference](../docs/header-annotations.md) for publication contracts and
the [RDL reference](../crates/libs/rdl/rdl.md) for syntax and encoding. Dependency pins, native
builds, and update commands are documented beside the tools that own them.

### Capture and reconciliation

- Compiler queries establish types, layouts, calling conventions, vtable slots, UUIDs, and constant
  values. Successful expression folding alone does not establish a valid static initializer.
  Do not replace native evidence with token parsing, spelling guesses, or SDK-specific exceptions.
- Reconcile native observations before public naming or dependency pruning, including observations
  later omitted by partial publication. Conflicts fail; failed observations are omissions, not
  evidence of agreement. A failed definition cannot fall back to an incomplete declaration.
- Preserve written typedefs, callback identities, annotation owners, qualifiers, and native
  semantics even when publication does not expose them. Parameters with unpublished contracts are
  marked `incomplete`. Do not erase nominal distinctions or contracts to make declarations agree.
- Macro discovery uses active definitions and compiler-evaluated probes in one incremental parse
  per target. Keep failed macro probes separate from native value failures; a probe must not erase
  a valid function with the same name. Do not reintroduce PCH replay, per-macro reparsing, or
  parentheses around initializer fragments.
- SDK include ordering, definition inputs, SAL transport, and `INITGUID` contexts belong in
  `tool-win32`, not parser or per-symbol policy. Do not enable `INITGUID` globally across repeated
  SDK GUID blocks.
- Preserve capture synchronization: index creation touches LLVM's process-global target registry
  and must not overlap live native sessions. Native companion and runtime versions must match;
  explicit `LIBCLANG_PATH` builds remain caller-owned.

### Public ownership and ABI

- Supplied WinMD references own matched public names after compatibility and target checks.
  Ambiguous matches fail. Do not add HRESULT, BOOL, or string canonicalization to clang.
- Primary names follow unique native defining typedefs, not alphabetical order or stripped
  underscores. Keep native records distinct unless checked by-value aliases establish a public
  family. Anonymous nested identity uses the owner and first native field, not generated-looking
  names or source offsets.
- Resolve header ownership across targets before partitioning. Shared declarations go to
  `shared.rdl`; do not seed routing from canonical metadata or add per-name/header preferences.
- Import ownership follows exact target-native compiler symbols and ordered, machine-filtered
  import-library evidence with checked overrides. Preserve named/ordinal evidence and code/data
  distinctions. A missing library is not the same as a present library missing a symbol; neither
  establishes native API absence. Do not guess decorated names or force a blanket x86 stdcall
  default. Callback ABI needs its own evidence.
- Layout and callable ABI are separate checks. Verify parameter and return eligibility
  independently; matching size or trivial copyability is not enough. Opaque records are
  pointer-only, not substitutes for by-value storage.
- Preserve field offsets, packing/alignment requests, bitfield member types, pointer-level
  constness, and architecture variants. Never form aligned references to byte-backed underaligned
  fields. Use `Interface::assume_vtable` for alignment-safe COM vtable-pointer loads.
- COM references are not ordinary value records. Const COM objects do not promise immutability;
  ordinary volatile/restrict evidence does not make generated field access volatile or exclusive.
- Use structural C/stdcall function-pointer signatures only when they preserve the callable
  contract. Contracts needing an attribute owner must retain a compatible named callback.
  Dependency traversal includes results, every parameter, and nested function-pointer signatures.
- Direction, optionality, nullability, success conditions, buffer phases, and incomplete markers
  are separate facts. Missing direction is unspecified. Do not combine conditional facts into a
  stronger unconditional contract or infer safe Rust signature transformations from metadata
  markers alone. C++ defaults and exception specifications are native compatibility evidence,
  not portable invocation or unwind contracts.

### Validation

Prefer focused `.h`/`.rdl` fixtures in `test_clang`, covering all three parsing targets. Verify
native companion queries in `test_clang_sys`. Keep procedural cases until their behavior is
individually verified and covered by dedicated fixtures.

Review regenerated `metadata/win32` and WDK output, including intended gains and unexpected losses.
Leave partial-generation changes visible, including deletions. Committed metadata is a review
baseline, not a correctness oracle; partial output and declaration counts do not prove coverage.
Validate metadata semantics independently of downstream generators, and add generated-code or ABI
coverage when a change affects consumers. Do not omit correct metadata to hide a consumer defect.

## Focused Investigations

Keep only unresolved questions here, with the affected file and symbol. Use focused GitHub issues
for reproducible bugs. Remove resolved entries rather than appending a history of fixes.

### Metadata and tooling

- Metadata reader bounds: `File::new` in `crates/libs/metadata/src/reader/file.rs` and decoding in
  `blob.rs` need focused malformed-input coverage for container/table/heap bounds, compressed
  integers, signature depth, and relationship ordering.
- WDK/SDK x86 callback ownership: `CLFS_BLOCK_ALLOCATION`, `CLFS_BLOCK_DEALLOCATION`, and
  `PIO_IRP_EXT_PROCESS_TRACKED_OFFSET_CALLBACK` are written without a convention in both SDK and
  WDK headers, so x86 user-mode builds read C and driver builds read stdcall. The SDK owns the
  `Windows.Win32` names and `crates/tools/win32/src/wdk.rs` correctly omits the kernel
  observations; no published ntdll/offreg import depends on them. Publishing kernel exports such
  as `ClfsCreateMarshallingArea` or `IoSetFsTrackOffsetState` needs a separate home for kernel
  metadata, not a per-name or per-header preference.
- Shared type traversal: compare `write_type_with_signature` in
  `crates/libs/metadata/src/merge/mod.rs` with `write_type` in `merge/remap.rs`; keep signature and
  nominal-reference handling consistent.
- SAL transport ownership: review the split between the producer in `crates/tools/win32/src/sal.h`
  and capture-time decoder in `crates/libs/clang/src/sal/transport.rs`; changes must keep them
  synchronized.
- Small duplication candidates: local `combine` helpers in
  `crates/libs/bindgen/src/types/interface.rs` and `cpp_interface.rs`, plus gradient-stop
  construction in `crates/libs/canvas/src/session.rs`.
- Public color value: `DARK_SLATE_BLUE` in `crates/libs/canvas/src/color.rs` does not match the CSS
  color of that name. Changing it needs a public-value decision, not a silent refactor.

### Native UI behavior and coverage

Investigate `MenuThemeState::refresh` and `show_pending` in
`crates/libs/reactor/src/native/transient_menu.rs`: a CI run observed a flyout item remaining Light
while its anchor reported Dark. The cause is unconfirmed; headless preference and callback tests
do not establish native theme propagation.

Live coverage still needs flyout-driven `DatePicker::on_date_changed` and
`TimePicker::on_time_changed` cases. Objects without UIA peers (`InfoBadge`, `PersonPicture`,
shapes, icons) have lifecycle coverage without observed property values. WebView process failure,
actual downloads, and deferral paths also need behavioral coverage.

Keep native reproductions separate from Reactor-only observations; do not add timing workarounds:

| Area | Observation | Reproduction |
| --- | --- | --- |
| `ToggleSwitch.IsOn` | Repeated updates become progressively slower | Plain C# WinUI |
| `NavigationView.IsPaneOpen` | Reopening before layout leaves compact width while the property reads true | Plain C# WinUI |
| `TeachingTip.IsOpen` | Changes during open/close animation can be ignored without an event | Reactor only |
| `ListView.ScrollIntoView` (#5146) | Access violation after seven removals, insert, append, and scroll on the next tick | Rust and C# WinUI; C# hangs after the same access violation |

For `NavigationView`, reopening from `PaneClosed` avoided the width defect in the native repro,
but visibly closes and reopens the pane. That behavior is not implemented in Reactor; the `pane`
fixture does not assert width after a rejected toggle.

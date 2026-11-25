# Copilot Instructions for windows-rs

Read this file at the start of every session. It contains the essential commands, conventions, and
architecture knowledge needed to work on this repository.

## Workflow

Do not create git commits automatically. The maintainer reviews all changes first and handles
commits manually. Make changes locally, run fmt/clippy/tests to verify, and report back.

### Reactor and Canvas reliability gates

Treat reliability work as a sequence of small contracts, not a broad cleanup or rewrite. A fast
demo, a green suite, and a review with no findings do not establish an untested contract.

1. **Define the claim before implementation.** State the observable behavior, failure behavior,
   and non-goals. Trace it through callers, queues, native calls, completion, and teardown,
   including unchanged code. Distinguish input delivery, drawing completion, swap-chain submission,
   and physical display. Do not use evidence from one stage to claim another.
2. **Make the claim falsifiable.** For a fix, reproduce the defect before changing production code.
   For a new contract, identify a violation the test must detect. Exercise failures at the layer
   that produces them: returning an error from a draw callback does not test Direct2D `EndDraw`.
   Use focused state/WARP tests for deterministic logic and `test-reactor` fixtures for native
   input, completion, and lifetime contracts. Do not add public test-only APIs or separate runners.
3. **Preserve failing conditions.** Do not make a failure disappear by adding pointer movement,
   changing focus, waiting longer, retrying, or weakening an assertion. A fixture correction needs
   evidence that its setup, rather than the product contract, was wrong. A narrower passing case
   does not resolve the original failure; retain its reproduction in a fixture or focused issue.
   Synchronize on the relevant completion, not elapsed time or successful `SendInput`.
4. **Fix one owning layer before expanding scope.** Do not combine unresolved runtime guarantees
   with sample replacement, API growth, or presentation tuning. Keep independently useful fixes
   separable. Do not revive shelved ownership wrappers, transactional emulation, or crate splits
   without evidence that they remove the demonstrated failure mechanism.
5. **Validate the actual configuration.** Before PR readiness, run the applicable workflow's
   toolchain, flags, feature combinations, and commands on the final code. In particular, follow
   `.github/workflows/reactor.yml`: nightly or feature-unified Clippy is not a substitute for its
   isolated stable checks. Run the default `test-reactor` suite with its COM diagnostics for
   native runtime changes. Report unavailable checks, ignored cases, and toolchain differences.
6. **Review the guarantee, not the test count.** Before recommending a runtime/API change for PR,
   obtain an independent review of the final diff and failure evidence. Give the reviewer the
   contract, assumptions, and unresolved observations, not a desired verdict. Check which defects
   could still pass the tests. If independent review is unavailable, report it as pending.
   Reconcile external findings before recommending merge.

A known defect in the claimed path blocks readiness for that path. A Reactor-only reproduction is
not proof of a WinUI defect; preserve the exact conditions and compare a minimal native reproduction
before assigning ownership. Do not replace a failing contract with a timing workaround.

Report what is verified, what failed, and what remains unverified. Do not call the whole framework
"solid", "hardened", or "ready" from a bounded result. Once a contract has its regression, fix,
validation, and review, stop expanding it and deliver that change. More tests, agents, benchmarks,
or speculative refactoring are not substitutes for closing the specific failure.

## Repository Layout

Cargo workspace (`resolver = "3"`). Members are globbed from:

- `crates/libs/*` - the published/library crates (`windows`, `windows-sys`, `windows-core`, plus
  `windows-bindgen`, `metadata`, `rdl`, and the newer `reactor`/`canvas`/`text`/`webview`/`window`
  crates).
  See `docs/readme.md` for the categorized index and each crate's `readme.md` for getting started.
- `crates/tools/*` - code generators and CI helpers, run via `cargo run -p tool-*`.
- `crates/tests/*/*` - test crates; `crates/tests/libs/<crate>` mirrors each library crate. Check
  its `Cargo.toml` for the package name, such as `test_clang` or `test-reactor`.
- `crates/samples/*/*` - runnable examples.

The crates fall into rough groups (see `docs/readme.md` for the authoritative list): core & errors
(`windows-core`, `windows-result`, `windows-strings`); values & collections (`numerics`,
`collections`, `reference`, `time`); async & threading (`future`, `threading`); system services
(`registry`, `services`, `version`); COM macros & linking (`implement`, `interface`, `link`); UI &
graphics (`reactor`, `canvas`, `text`, `webview`, `window`, `animation`, `reactor-setup`);
codegen & metadata tooling (`bindgen`, `metadata`, `rdl`, `cppwinrt`); and the full API projection
(`windows`, `windows-sys`).

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
cargo test -p windows-reactor --quiet

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

### Text

```sh
cargo check -p windows-text --quiet
cargo test -p test_text --quiet
cargo clippy -p windows-text -p test_text --all-targets
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
   - `crates/libs/text/src/bindings.rs` (from `text.txt`)
   - `crates/libs/time/src/bindings.rs`, `numerics`, `reference`, etc.

4. **`tool-package`** - generates the published `windows` and `windows-sys` package crates using
   `--package` mode (per-namespace files + Cargo.toml features).

After regenerating, verify: `cargo check -p <affected-crate> --quiet`.

## Key Architecture Facts

### Crate relationships

- `windows-core` is the foundation - almost everything depends on it.
- `windows` is the umbrella crate that re-exports from `windows-core`, `windows-numerics`,
  `windows-time`, `windows-collections`, `windows-reference`, etc.
- `windows-reactor` depends on `windows-core` (not `windows`) and uses bindgen's default minimal
  bindings.
- `windows-canvas` similarly uses minimal bindings for D2D/DXGI/WIC.
- `windows-text` wraps DirectWrite fonts and text layout with its own minimal bindings. Canvas
  depends on it, re-exports its text types and `Rect`, and casts with `to_interface` to its own
  `IDWriteTextFormat`/`IDWriteTextLayout` bindings for Direct2D calls. Text positions in the public
  API are UTF-8 byte offsets.
- `windows-animation` wraps Win32 UIAnimation Manager COM APIs.

### Reactor architecture

- WinUI backend ownership is split across `crates/libs/reactor/src/native/winui/mod.rs` and
  `crates/libs/reactor/src/native/winui/`.
- The TOML config (`crates/tools/reactor/src/schema.toml`) declares WinUI controls. Keys are WinUI
  metadata names; the tool infers types, setter patterns, and event handlers from `.winmd` files.
- COM casts: classes Deref to their default interface (zero-cost). Only cast to non-default parent
  interfaces. The `Param` trait handles parent-class conversions automatically.

### Reactor architectural decisions

Read these decisions before proposing another redesign. Shelving code or starting a new session
does not reset the conclusions. Reopening a direction requires naming the changed premise and a
bounded experiment that can disprove the claimed improvement.

- **Construction-once WinUI replacement: shelved, not adopted.** The scoped native-owned editor
  hid weak references, subscription lists, and explicit row refreshes, but added ownership,
  binding, and row-adapter machinery. It did not demonstrate lower total application-plus-library
  complexity. Revisit only with a representative workload that reduces total obligations, not
  just the visible application or wrapper size.
- **Optimistic scalar reconciliation: shelved, not adopted.** Comparing declarations alone misses
  normalization back to the previous model value. The experiment needed explicit writes and
  application guards against native event echoes; passing headless tests did not establish native
  editing behavior. Do not revive it without resolving same-value normalization, callback delivery,
  caret/selection, and IME obligations without application repair logic.
- **List-control replacement is not a general reliability fix.** In the closed #5380
  investigation, top-of-list cases passed while nonzero-scroll resets jumped; granular updates
  traded reset behavior for per-item notification cost. Native reorder also separated item keys
  from source indices and container leases. Preserve these distinctions in any replacement.
  Compare the same workload, including nonzero scroll and churn; do not transfer readiness claims
  from a discarded branch to the current implementation.
- **Subscription-owned event lifetime: preserve when porting.** The experimental
  `NativeSubscription` owns the lifetime token; queued and deferred occurrences hold only weak
  references. Callback replacement preserves the subscription; removal or owner destruction
  expires its occurrences. This makes cancellation follow ownership instead of depending solely
  on every teardown path remembering to clear queues. Preserve both sides of this contract when
  assessing the change independently of the custom control.
- **Canvas process table: specialized control, not a backend replacement.** The experimental
  viewport-sized surface avoids native row containers and has promising manual visual/input
  results. That does not establish failure-path correctness, input-to-pixel latency, or a complete
  control toolkit. Keep its missing accessibility and input capabilities explicit. An owned
  Direct3D backend remains unproven: rendering/text primitives do not settle editing, IME, focus,
  or accessibility.

When an experiment ends, update its decision, evidence, limitations, and reopening condition here
before shelving or replacing it. Keep executable regressions in the owning test crate and unresolved
reproductions in Focused Investigations or focused issues. Preserve the resulting invariant after
removing a resolved investigation; do not rely on chat history or local stashes as the only record.
Keep this section to current decisions, not a chronological account of rewrites.

### Canvas architecture

- Canvas wraps D2D/DXGI behind safe Rust types (`GpuDevice`, `SwapChain`, `DrawingSession`,
  `PathBuilder`).
- `animated_canvas()` (reactor feature) renders on UI thread via `CompositionTarget::Rendering`.
- Device-lost recovery is automatic.
- Owned drawing sessions retain the complete `EndDraw` result until the owner checks it.
  `SwapChain::present` must reject failed drawing before calling DXGI; `RenderTarget::draw` must
  report deferred errors. Native `EndDraw` device loss takes precedence; otherwise callback
  device-loss errors are preserved. Borrowed sessions leave draw completion to their caller.
  Preserve the native-error regressions in `test_canvas` and the Canvas error-path fixture in
  `test-reactor`; a successful draw closure is not completion.
- `Canvas::on_submitted` runs only after successful drawing and swap-chain submission, outside the
  render-state borrow. Publish prepared hit-test geometry there and resolve it in routed pointer
  callbacks before queuing messages. Changing pointer delivery mode replaces the callback, not the
  subscription: queued occurrences retain their original callbacks. Test replacement before removal
  or teardown, which cancel pending occurrences and can mask a replacement bug. Preserve
  the native submitted-input and routing fixtures described in the `test-reactor` README; neither
  submission nor their injected callback device loss proves physical display or native removal.

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
- **Stock collection retirement**: Replacement, removal, clear, and whole-vector replacement
  release retired entries after unlocking, before observable notifications. Destruction may reenter
  and mutate the committed collection; nested insertions during clear or replacement must survive.
- **Collection storage**: Safe direct factories constrain actual retained defaults to `Send + Sync`.
  Unsafe direct factories cover all retained objects and future writes for the collection's entire
  lifetime. Marshaled factories retain agile references through views, iterators, and pairs, resolving
  outside structural locks. Preserve null outputs without converting resolver errors to null.
- **Stock async results**: `ready` and `spawn` transfer successful values once, including their
  progress-interface variants, which do not produce intermediate results. Keep completion status
  and error information independent of value consumption. Results need `Send`, not `Sync`; do not
  clone values or access native error information under the completion-handler mutex.
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

### Mutable COM collections

- Direct stock storage still clones and compares values under structural locks, including map keys.
  `Send + Sync` does not prohibit reentrant `Clone`, `AddRef`, or comparison. Reproduce the owning
  path before changing storage; retirement safety does not establish arbitrary callback reentrancy.
- Native `PropertySet` is not behaviorally identical to the stock observable map. The comparison
  in `crates/tests/libs/collections/tests/stock_observable_map.rs::native_property_set_contract`
  covers nulls, snapshot invalidation, reset notifications, and mutation during notification.
  Native map behavior alone does not establish WinUI binding compatibility or apartment ownership.

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
- Shared type traversal: compare `write_type` in `crates/libs/metadata/src/merge/mod.rs` with
  `Remapper::write_type` in `merge/remap.rs`; keep signature and nominal-reference handling
  consistent.
- SAL transport ownership: review the split between the producer in `crates/tools/win32/src/sal.h`
  and capture-time decoder in `crates/libs/clang/src/sal/transport.rs`; changes must keep them
  synchronized.
- Public color value: `DARK_SLATE_BLUE` in `crates/libs/canvas/src/color.rs` does not match the CSS
  color of that name. Changing it needs a public-value decision, not a silent refactor.

### Native UI behavior and coverage

#### Canvas details recovery

The destination is a reusable in-house details/list control, proved with the Task Manager sample.
Use that workload to harden Reactor and Canvas at the layer owning each demonstrated failure.
This is not another Reactor rewrite or an attempt to match every WinUI ListView feature.

Build on updated master, retaining `reactor-experiments` and closed PR #5382 as salvage references,
not a patch to import wholesale. Paths and symbols for the custom control below refer to that
experimental branch until ported. Keep each iteration independently reviewable and leave the
sample runnable. Subscription-lifetime fixes can be assessed separately.

Current gate: maintainer acceptance of the bounded submitted-frame input change. After acceptance,
integrate that boundary into the existing table, harden it under churn, and adopt it in Task Manager,
keeping those changes separate. Measure performance against the same workload and an agreed
baseline; drawing time is not input-to-pixel latency.

1. **Integrate submitted-frame input in the table.** Use `Canvas::on_submitted` to publish prepared
   geometry and routed pointer callbacks to resolve row targets before queuing messages. Retain
   the small Canvas fixture while adding control-level coverage; do not infer the table's behavior
   from the framework fixture.
   In `crates/tests/libs/reactor/tests/ui/canvas_details.rs`, wheel injection alone is not evidence
   that the scrolled frame was submitted; observe the marker before asserting a new-frame click.
2. **One submitted geometry for input feedback.** In
   `crates/samples/reactor/task-manager/src/details_view/`, review `Frames::present`, pointer
   state, and drawing together. Hover and divider feedback must follow the submitted geometry
   when scrolling or resizing without pointer movement. Cover wheel, keyboard, and scrollbar
   changes, stale row targets, and failed-frame publication with focused cases.
3. **Resolve the cursor observation without changing the reproduction.** In the experimental
   `RoutingFixture` in `crates/tests/libs/reactor/src/pointer.rs`, change a child's cursor while the
   pointer stays over it and its ancestor has an explicit cursor. The experimental test in
   `tests/ui/main.rs` moves away before changing mode; it proves re-entry behavior only. Compare the
   unchanged case with native WinUI before fixing Reactor or documenting a native limitation.

The drawing-completion contract is a prerequisite for claiming successful-frame publication.
Only then assess the complete process-view replacement. Remove resolved items from this recovery
section rather than extending it into a change journal.

#### Other native observations

Investigate `MenuThemeState::refresh` and `show_pending` in
`crates/libs/reactor/src/native/transient_menu.rs`: a CI run observed a flyout item remaining Light
while its anchor reported Dark. The cause is unconfirmed; headless preference and callback tests
do not establish native theme propagation.

Live coverage still needs a flyout-driven `DatePicker::on_date_changed` case. Objects without UIA
peers (`InfoBadge`, `PersonPicture`, shapes, icons) have lifecycle coverage without observed
property values. WebView process failure, actual downloads, and deferral paths also need behavioral
coverage.

Keep native reproductions separate from Reactor-only observations; do not add timing workarounds:

| Area | Observation | Reproduction |
| --- | --- | --- |
| `ToggleSwitch.IsOn` | Repeated updates become progressively slower | Plain C# WinUI |
| `UIElement.ScaleTransition` | Setting a scale axis to zero hides the element immediately, without visible intermediate shrinkage | Plain C# WinUI 2.6.4; `test-reactor` scale fixture: full, transition, zero |
| `NavigationView.IsPaneOpen` | Reopening before layout leaves compact width while the property reads true | Plain C# WinUI |
| `TeachingTip.IsOpen` | Changes during open/close animation can be ignored without an event | Reactor only |
| `ListView.ScrollIntoView` (#5146) | Access violation after seven removals, insert, append, and scroll on the next tick | Rust and C# WinUI; C# hangs after the same access violation |
| `ItemsRepeater` pinned element (microsoft/microsoft-ui-xaml#12659) | A focused row scrolled out of view keeps its old index after a reorder; the next keyed Reset fails with "unique id is not unique" | Plain C# WinUI; the same steps in ListView are correct |
| Pointer input after first show (microsoft/microsoft-ui-xaml#12753) | The first injected move within about 40-60 ms of root `Loaded` raises no pointer events; `Loaded` and `Rendered` do not signal readiness | Plain C# WinUI; WinForms receives the same move |

For `NavigationView`, reopening from `PaneClosed` avoided the width defect in the native repro,
but visibly closes and reopens the pane. That behavior is not implemented in Reactor; the `pane`
fixture does not assert width after a rejected toggle.

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

### COM aggregate-return ABI

`bindgen` `types/cpp_method.rs` `CppMethod::write_abi` emits an explicit result pointer but a void
return for `ReturnHint::ReturnStruct`; `write_upcall` only writes through the pointer. An MSVC x86
native caller in the clang2 ABI experiment read the returned aggregate through EAX, which must hold
that result-buffer address. Rust-to-C++ calls passed, but the reverse call produced incorrect data.
`clang2` `project.rs` `Builder::interface` rejects by-value record results until the downstream ABI
is fixed and covered. See `docs/crates/windows-clang2.md` for the fixture and remaining ABI gates.

### Clang2 pre-scale review

The five review findings have source-fixture coverage: indexed annotation ownership, reusable
projection policy, source-marker SAL capture, compiler-rendered UUID attributes, and pointer-only
integer probes. See `docs/crates/windows-clang2.md` under "Pre-scale critical review".

Preserve captured contracts with RDL `#[annotation(source, payload)]` independently of typed
lowering. SDK/WDK callers must force-include `clang2/src/sal.h` before SDK headers. Keep ordered
source families and original positional parameter bindings through agreement checking.

Open boundaries: long-tail SDK/WDK macro coverage, MIDL comments outside captured declaration
extents, same-name annotated record aliases, imported data, and
large multi-TU peak memory. Continue with `.h`/`.rdl` fixtures and real-header inventories, not
another bindgen or native API validation campaign.

`tool-win32 --clang2-headers shellscalingapi.h,tlhelp32.h --rdl-only` emits all 72 selected names.
The explicit route has no clang1 fallback and does not replace committed metadata. Existing RDL
supports the record aliases; public record/tag naming and production partition ownership remain
cutover decisions. Explicit `ProjectionOptions::pointer_sized` SDK contracts preserve `usize`/`isize`
uses after native signedness and layout checks. Use `metadata/` RDL as the representation baseline,
not as a replacement for native evidence. Adding `pathcch.h` emits all 98 selected names. String
literals preserve compiler-rendered code units in existing RDL `String` constants; counted
character-pointer bindings preserve SAL buffer contracts. The header route explicitly excludes
inline helper overloads without hiding missing non-inline imports. See the bounded migration section
in the clang2 docs.

Discovery classifies declaration-alias and attribute macros with memoized identifier chains;
the header runner reports helpers instead of probing them as constants. Function-like
classification preserves undefined helpers using source tokens and line-splice-aware adjacency.
Public aliases remain a publication decision, not a native export-name guess.

`tool-win32 --clang2-headers all --rdl-only` inventories manifest headers independently.
`manifest.tsv` persists after each attempt; unavailable discovery counts remain unknown, not zero.
`imports.tsv` records the full ordered SDK COFF import candidates. The generic route has no external
bindings, source-name DLL fallback, or legacy `LIBRARY_OVERRIDES`. Emitted roots must pass combined
WinMD compilation with the attribute vocabulary but no default API references in generic profiles.
RDL-only removes the temporary binary; it does not establish cross-header/architecture agreement.
Keep dated measurements and failure tables in `docs/crates/windows-clang2.md`, not this instruction
file.

Experimental SDK prerequisites live in `tool-win32/src/header_profiles.rs`, not parser repairs.
Keep requested-header file identity as the discovery root scope; dependencies only supply native
declarations. Persist actual TU sources and arguments. The legacy input route does not use these
profiles; do not bypass a native header's include contract with synthesized typedefs or guards.

Named callbacks retain compiler TypeRef-backed typedef edges, calling conventions, annotations,
and pointer depth. Incomplete C records preserve nominal pointer identity but reject by-value
calls without layout. Variable redeclarations compare canonical types and retain/check written
typedef contracts. Nested record members are captured through their owner, not independent roots.
Inventory `capture_report` now assigns failed probes by compiler file identity and byte ranges,
then reparses without rejected probes. Source, fatal, and unowned errors remain fatal; an errored
AST never supplies evidence. Strict `capture` is unchanged. `Snapshot::assess` propagates
unavailability over every observation's written/canonical dependencies and checks the closed
available graph; unavailable roots are explicit rejections, including whole overloaded names.
Available native conflicts still fail globally. Strict `resolve` remains unchanged.
Annotation ambiguity fails when selected evidence consumes the context, not during broad indexing.

Capture dominates measured costs; no global projection cache or alternate IR is justified.
Prioritize verified header prerequisites and record layouts using `docs/crates/windows-clang2.md`,
not symbol whitelists, guessed constants, or legacy extraction fallback. The production backend
remains unchanged; full source/RDL inventory is not a completed replacement gate.

### Clang2 Animation cutover

`tool-bindings` generates Animation through clang2 from the pinned SDK and the actual
`animation.txt` filter. Core COM and `IDCompositionAnimation` are explicit subsystem contracts;
local animation declarations do not resolve through default metadata. `class_guids` publishes
opaque source class UUIDs as typed GUID constants without inventing object definitions or data
initializers. `test_clang2` `animation.rs` checks actual wrapper sources, native identities/slots,
keyframe layout, live transitions/storyboards, and curve handoff on x86/x64. ARM64 is projection
only; the generic by-value union gate stays closed.

### Clang2 WebView2 cutover blockers

`test_clang2` `webview.rs` covers six `HWND` inputs and two host-object `VARIANT*` inputs with an
ordinary passing gate and exact public-wrapper signature assertions. Concrete binding filters
include external signature dependencies; a broad namespace filter can omit methods while the
generated file still compiles. `com.rs` exercises handle/string wrappers against native C++.
`clang2` preserves explicit pointer typedef contracts and separate SAL/MIDL evidence: check each
source family across declarations, use SAL direction for local calls, and fall back to MIDL when
SAL has none. Only bounded prefix direction comments are decoded. Remaining WebView2 differences
include flags-enum policy and alias/tag names. DLL routing and broader annotation/production
coverage remain gates; see `docs/crates/windows-clang2.md` for the differential and measurements.

### Clang2 issue-driven correctness gate

Open issue repros exposed lost `clang::flag_enum` evidence and output-slot projection for borrowed
interface objects; both have prototype fixes and regression fixtures. Real WDK member alignment
has native-checked storage support; adjusted-record by-value calls remain explicit rejections.
A native overload experiment showed incorrect
dispatch for `Echo(int)`/`Echo(float)` despite compiling wrappers: `project.rs` `Builder::interface`
rejects same-name methods until MSVC vtable ordering is modeled and covered. Do not replace that
gate with name-based slot-reversal rules. See `docs/crates/windows-clang2.md` for issue scope and
remaining production gates; these changes do not fix or close the production-backend issues.
The independent review also found inherited pure virtual overrides emitted as new slots. Capture
retains compiler override relationships; local projection rejects slot reuse, including implicit
and indirect overrides, until that ABI is modeled. Existing metadata, RDL, generated bindings, and
legacy parity are not correctness oracles: use original API contracts and compiler/native evidence.

`test_clang2` `wdk.rs` compares the real pinned WDK member types against MSVC offsets, size,
alignment, and pointer calls. `crypto.rs` exercises eight real BCrypt exports and a full hashing
lifecycle on x64/x86. `ProjectionOptions::imports` maps native linker symbols to DLL/export
contracts; `windows_rdl::implib` derives these from COFF name types, not spelling heuristics.
`clang2/metadata.rdl` supplies experimental output valid-byte postconditions separately from
capacity; keep both through RDL/WinMD. The production metadata seed is unchanged. SDK-wide DLL
correctness, broader annotation semantics, and full production inputs remain open.

### Clang2 hard-layout continuation

`clang2` `project/record.rs` checks union storage and recursively lowers unnamed records in direct
fields to existing nested RDL. `test_clang2` `layouts.h`/`layouts.rdl` cover source contracts and
three-target metadata; `layouts.cpp` and `abi.rs` compare MSVC size, alignment, offsets, and pointer
mutation in both directions on x86/x64. Union and anonymous-storage by-value calls still reject.
Packing, bitfields, anonymous arrays/pointer identities, and inherited layouts remain separate gates.
Do not infer calling support from matching storage or extend coverage with symbol-specific repairs.

Callable presentation names select the lexicographically smallest usable observed name per original
position, with collision-safe positional fallbacks; filename and input order do not choose names.
Keep source annotation bindings positional. The BCrypt consumer explicitly binds native `LPCWSTR`
to `PCWSTR`; generic header projection does not infer that wrapper. See the clang2 docs restart point.

Inline record-field callbacks retain owner/field-slot identity and compiler-owned parameter contexts
across TUs, rather than merging by function shape or fabricating typedefs. `test_clang2` `ole.rs`
compiles the real OAIDL/WIC `IDispatch`/`ITypeInfo`/`EXCEPINFO` closure on all three targets without
external OLE value bindings. Named nested records keep nominal identity and shared references under
checked scope-prefixed publication; collisions and unsupported by-value calls still reject.
Other anonymous callable positions remain gates. Combined `oaidl.h`/`shellapi.h` capture preserves
Shell return annotations.
`annotation_macro_return.h` covers Clang-attached prefix annotations outside macro-started extents,
original inherited parameter contexts, and mixed prefix/in-range ownership. Keep equal-rank
ambiguities visible; do not suppress annotations or substitute OLE types to improve header counts.

`ProjectionOptions::preserve_typedefs` retains non-interface written typedefs and prefers a unique
typedef name for unselected tags without MIDL-prefix heuristics. Animation uses this policy and the
checked `UINT_PTR` pointer-sized contract. Its differential gate compares every source interface
slot/signature/GUID and public value types against committed metadata, with native checks as the ABI
oracle. No further consumer cutovers precede a classified differential. Packed records, bitfields,
function-macro aliases, and streaming TU disposal remain open porting work.

Multiple selected aliases to a named record retain the source definition and publish all aliases;
do not choose a winner or structurally merge types. Unnamed-owner ambiguity still rejects.
Transparent interface aliases use existing native typedef metadata and are native-checked for COM
identity and clone/release behavior. The focused SSPI/CFAPI/SensorsAPI/WS2TCPIP/XmlLite refresh clears
926 global publication blocks. Annotation contexts use compiler source-location identity rather
than expansion position alone: several macro-generated methods can share an expansion position.
The XAudio2 refresh clears 120 capture blocks without weakening inherited-slot or layout gates.

Double-NUL parameter contracts preserve terminator count and explicit pre/post/unspecified phase
through the experimental `NativeStringTerminationAttribute`; keep raw pointers, not ordinary
string wrappers. Preserve optionality, buffer capacity, and raw SAL/MIDL evidence separately.
Scoped/dereferenced terminators and broader combinator lowering remain unsupported.

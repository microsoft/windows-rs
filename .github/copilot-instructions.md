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

- WinUI backend ownership is split across `crates/libs/reactor/src/native/winui/mod.rs` and
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

- **Module layout**: When a module has a directory, put its entry point in `name/mod.rs`, not
  `name.rs` beside the `name/` directory. The workspace enables Clippy's `self_named_module_files`
  lint to enforce this for modules with external submodules.
- **Panics**: Use `panic!` only for invariant violations. Use `diag::` helpers for missing features
  (warn in debug, no-op in release).
- **`.unwrap()` over `.expect("...")`** - the panic hook provides full context.
- **No `thread_local!` in app code** - keep state in components and pass shared values through typed
  contexts. `thread_local!` is reserved for framework plumbing.
- **Test naming**: Reactor unit tests live in `windows-reactor`. Public behavioral coverage belongs
  in `test-reactor`. Prefer a small set of high-value contracts over exhaustive UIA matrices or
  preserving every legacy assertion. Do not add separate runners or test scripts. Keep library tests
  headless rather than adding ignored native fixtures. The functional crate participates in workspace
  tests and uses `windows-reactor-setup` in its build script. Live UI cases run on x64; setup's
  filesystem and multiprocess tests run on all Windows architectures.
  Run the crate alone when checking default features rather than the workspace's unified
  features.
  Canvas tests use WARP rendering.
- **Reactor test documentation**: Keep testing instructions in `crates/tests/libs/reactor/readme.md`,
  not `docs/crates/windows-reactor.md`. `cargo test -p test-reactor` must run the automated suite
  without ignored-test flags. Generate its native bindings with `windows-bindgen` in `build.rs`
  rather than depending on the `windows` crate.

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

### Native menu theme propagation

Investigate `crates/libs/reactor/src/native/transient_menu.rs` `MenuThemeState::refresh` and
`show_pending`. A CI run observed a flyout item remaining Light while its anchor reported Dark.
The cause is unconfirmed. The headless preference and callback tests do not establish native theme
propagation; investigate the behavior before adding a focused public-API regression case.

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

### clang rapid iteration

SDK-specific definition include configurations are provisional. Revisit these workarounds together
before replacement acceptance, including Olectl's dependency preloading and split
`DEFINE_GUID`/`INITGUID` state. Keep them in the SDK driver, not parser or per-symbol policy.

The `test_clang` breadth fixtures cover free functions, COM-style interfaces, const integers,
booleans, finite floats, and selected object-like macros through capture, target association, RDL,
and WinMD. The SDK case uses `GetCurrentProcessId`, `IUnknown`, `IClassFactory`,
`UIA_InvokePatternId`, and `MAX_PATH`. Keep DLL ownership in publication policy and macro
evaluation distinct from ordinary declaration capture.
`tool-win32 --partial --sdk-imports` reads each target's pinned SDK libraries with the shared
`tool-win32/src/imports.rs` policy. Preserve library order and checked overrides. Explicit public
imports win over exact compiler linker symbols in target-specific maps. Filter short-import members
by machine before ordered selection; ARM64 archives also contain ARM64EC members. Code/data/const
kinds remain distinct. Keep ordinal archive evidence, but publish established exports by name
through checked SDK policy. Unreviewed ordinal records remain explicit omissions; do not add
ordinal consumer support or infer names by undecorating symbols. Consumer fixtures check API
presence, ABI, execution, and named PE imports independently of std's import libraries.
The first target-native import run exposes 182 missing x86 mappings: 94 declarations use default
cdecl while their libraries contain stdcall symbols, and 88 have no matching entry in the selected
x86 libraries. `sdk_import_gaps` preserves both boundaries. A compiler-default stdcall probe matches
the 94 alternate symbols. The approved `--x86-stdcall` experiment uses Clang's
`-Xarch_i686 -mrtd`, not an IR convention rewrite. It remains opt-in; review unannotated callbacks
as well as functions. Do not repair gaps with symbol spelling guesses or treat missing import
evidence as native target absence.
The stdcall trial recovers 94 functions but also changes 84 callbacks, the six ICU
`UReplaceableCallbacks` fields, and `InitializeXamlDiagnosticsEx` on x86. Seventy-five named
declarations matched baseline cdecl before the trial. Native import agreement does not validate
these extra ABI changes; the blanket default is not accepted for canonical publication.
Native ABI conflicts are fatal. Import redeclarations compare parameter labels positionally and
expand only unannotated value typedefs for agreement. Keep the selected observed signature's aliases;
nominal types, qualifiers, callback names and typedef-owned contracts must not be erased.
Repeated GUID blocks also occur in the main TU (`winioctl.h`); enabling INITGUID before its prelude
fails even with satellite definition mode disabled.
`Input.macro_headers` and `--macro-header=header.h` discover object-like macros through compiler
preprocessing records and per-macro expression probes in one incremental native parse. Failed
probes never supply capture evidence. The certbcli slice recovers its 22 baseline constants,
retaining `CSBFT` typedefs, plus two
string macros whose encoding follows UNICODE. Unselected headers' macros remain absent from both
output and omissions; track baseline parity separately from captured-declaration counts.
Winerror and urlmon discovery recover all 7,456 baseline winerror names with unchanged values.
Retain written typedefs; 363 use `DWORD` where the baseline used `u32`. The active definitions of
18 `INET_E_*` macros belong to urlmon. Do not recover names by capturing superseded definitions.
Ntstatus discovery recovers all 3,072 baseline declarations with unchanged types and values.
Its standalone input needs `winternl.h` for `NTSTATUS`; use `WIN32_NO_STATUS` around Windows headers.
Active definitions owned by mferror or wincred remain explicit discovery diagnostics in the full TU.
Winnt macro discovery recovers 3,231 baseline names with unchanged values and 32 written-typedef
differences. A separate lean Windows INITGUID input recovers 259 winnt and 11 winsvc GUIDs through
the existing aggregate evaluator. It does not repeat macro discovery.
Constant observations retain native type, qualifiers, attributes, location, and linkage; only a
declaration has no value. Resolution compares against a definition when available so declarations
cannot hide conflicting values. Missing definitions fail publication; invalid initializers remain
capture failures. All SDK inputs include intsafe before Windows headers, supplying `DWORD_MAX` for
`POWER_LIMIT_VALUE_NO_CONTROL` without displacing winerror's `S_OK`. Mixed support contexts change
native SAL arguments and must fail reconciliation. Winnt has 3,490 baseline names present; its only
missing name, `_ARM64_MULT_INTRINS_SUPPORTED`, is an approved compiler-internal scope exclusion
whose active definition belongs to the unselected support header, not a parser special case.
Hidusage macro discovery recovers all 2,833 baseline constants with exact types and values on all
targets. Only `hidusage.rdl` changes; the header guard is the sole new omission.
Winuser/imm discovery recovers 3,123 baseline names with unchanged values and target availability;
75 retain written typedefs. The active owners of `MOD_ALT`, `MOD_CONTROL`, and `MOD_SHIFT` are imm.
Keep macro-constant exclusions separate from native value exclusions through capture, dependency
pruning, and publication. Failed probes must not erase observed native functions with the same
name; known native conflicts still fail before pruning. The `macro_function_aliases` fixture covers
these boundaries, including SDK-style x86 aliases for functions declared on 64-bit targets.
Wincrypt discovery closes 2,678 baseline names with unchanged values, encodings, and target
availability; 173 retain `LPCSTR` aliases. Its fixture covers string concatenation and embedded NULs.
Four `CERT_STRONG_{SIGN,KEY}_PARA_OS_*` brace initializer macros still require a destination type
and remain unsupported; never infer that type from macro names or tokens.
Commctrl/richedit discovery closes 2,157 and 567 baseline names and five moved commdlg constants.
All prior declarations remain unchanged. Generic opaque pointee markers recover their dependents;
checked compiler static initializers recover the 38 structure-size macros without SDK exceptions.
Mmreg and its ksmedia/wmsdkidl companions add 2,475 baseline constants without changing prior
declarations. Active ownership puts 20 speaker flags in ksmedia and `WAVE_FORMAT_DRM` in wmsdkidl.
Retained values and target availability match; 55 constants preserve written typedefs. The full
selection retains 1,974 of 1,976 mmreg names and all 78 wmsdkidl names. Discovered `__uuidof`
GUID macros, including mmreg's PCM/WAVEFORMATEX GUIDs, publish their native values.
Three ksmedia property keys need definitions; `KSEVENTSETID_VIDCAPTOSTI` is a UUID tag without a
same-named value macro, so publishing its tag name as a constant needs a separate policy decision.
Macro discovery does not use a PCH; a PCH reload lost the builtin `_GUID` definition and rejected
every `__uuidof` macro. `uuid_macros` checks explicit and discovered GUID parity. Do not reintroduce
per-macro reparses or PCH replay; probes share one incremental parse per target.
Macro probe initializers must not add parentheses around the macro. That turns comma-separated
initializer fragments into scalar comma expressions. Preserve source-authored grouped expressions;
let Clang reject field lists without filtering macro names or interpreting their tokens.
Publication recognizes unscoped numbered `__MIDL_` helper-enum tags, preserves authored typedefs,
and otherwise emits loose constants without changing native enum identity. Numeric data-pointer
constants use native static-initialization evidence and require numeric addresses without relocations.
Their values are sign-extended from the target pointer width. Reject relocations, side effects,
undefined behavior, function pointers, and COM interface references.
The `sdk_functions` fixture covers 114 imports across fourteen DLLs, including buffers, callbacks,
COM activation, registry/security, sockets, cryptography, GDI, timing, virtual memory, HTTP,
event logging, service control, process inspection, Direct3D device creation, DirectWrite factory
creation, and Direct2D matrix/device/context operations.
External C++ inline overload observations are retained in Snapshot::inline_functions when the
same selected name has an external C declaration in that TU. Every overload observation must be
inline and a definition must be present. Do not suppress non-inline overloads or C definitions.
Enum constants retain their nominal type. The `sdk_interfaces` fixture checks storage, imaging,
properties, monikers, shell, Direct3D/DXGI, DirectWrite, Direct2D factory4, and WIC raw-imaging
dependencies. Exception contracts retain explicit metadata.
Constant breadth includes compiler-evaluated GUIDs, property keys, ASCII/UTF-16 arrays, and selected
string/GUID macros. `aggregate_constants` and `sdk_constants` check exact RDL and both metadata
round trips, including architecture variants and annotations. Definitions come from compiler cursor
links; aggregate leaves come from `APValue`, not numeric-token parsing. Discovery of
definition inputs for further GUID headers and coverage of further macro headers remain separate work.
Do not count unselected macros as covered or enable `INITGUID` globally across repeated SDK GUID blocks.
Review whether each qualification has useful projected semantics before requiring new metadata.
COM object constness remains in native evidence and reconciliation, but publication maps `I*`
and `const I*` to the same interface reference. DXGI's `SetPrivateDataInterface` retains/releases
its `const IUnknown*` argument; this is not an immutability contract. Preserve const on pointer
storage (`I* const*`), and reject volatile/restrict. Do not apply this normalization to ordinary
records. Interface typedefs publish as transparent aliases; call signatures remain canonical COM
references. Qualification review must not hold up unrelated breadth work.
Keep annotation interpretation bounded: preserve owner and evidence, and expand semantic lowering
only when a concrete API or consumer requires it.

RDL direction is explicit: known In/Out flags are always written, and missing direction has no
attribute. Do not infer direction from types or emit `direction(unspecified)`.
Complementary pointer-null `_When_` branches publish
only common direction/optionality facts and retain both conditions. RegQueryValueExA/W's count
remains conditionally input; shared out/opt flags do not authorize an output-only conversion.

Direct, unscoped, argument-free SAL2 result checking on imports, methods and named callbacks emits
`#[must_use]`. Callback aliases inherit the contract; duplicate annotations emit one marker.
Parameter/global inspection, scoped annotations and success predicates remain separate evidence.
Native SAL identities still participate in reconciliation; canonical result use does not promise
Rust diagnostics or reproduce SAL dataflow analysis.

Bounded success predicates emit `#[success(return == 0)]`, `!= 0`, `== 1`, `>= 0` or `> 0`
through enum-valued SuccessConditionAttribute. Native type checks guard lowering; do not turn
ordered pointer comparisons, complex expressions or competing defaults into interpreted contracts.
Value aliases preserve links to return-default owners, including annotations from other inputs.
Callable conditions override typedef defaults; neither means automatic Rust Result conversion.

Use these slices to assess whether the shared architecture remains simpler and preserves more
native information across equivalent responsibilities. Record/alias counts alone cannot establish
replacement readiness. Do not let bitfield, opaque-record, or annotation completeness become a
prerequisite for a basic slice in another category. Expand difficult cases only when the slice
needs them; keep unsupported contracts explicit.

Drive the rewrite toward complete, correct canonical metadata with a short feedback loop.
Use header-to-RDL fixtures during breadth iteration; defer Rust generation and consumer validation
to final acceptance rather than running them for each batch.

1. Pick a concrete metadata difference or root omission and add a focused header/RDL fixture
   establishing the intended result.
2. Fix the responsible capture, reconciliation, or projection layer. If a feature requires
   scattered exceptions or conflicting interpretations, correct that boundary before continuing.
3. Regenerate tracked `metadata/win32` even for partial runs, leaving all changes visible in
   VS Code's Git diff for review. Do not hide or restore partial changes, including deletions.
   Run `tool-win32 --compare-constants` against the frozen Git baseline after successful generation.
   Check intended gains and unexpected losses against the header/RDL fixtures on all targets.
4. Compare root omissions, dependent losses, time, and memory with the preceding iteration.
   Diagnostic counts include dependency cascades; uncaptured macros are not counted.

Partial runs and passing fixtures are progress evidence, not replacement readiness. Large deletion
diffs remain outstanding work. Review architectural pressure points as coverage grows; do not
substitute repeated redesigns, arbitrary naming/routing rules, or weaker rejection for coverage.
`capture_partial` reconciles successfully captured observations before dependency pruning. Failed
observations remain unvalidated omissions, not evidence of compatibility. The validation-pruning
fixture covers conflicts behind unsupported dependencies and empty enum/alias observations.

`clang` projects genuine incomplete global structs as nominal pointee markers with no class layout.
Completeness comes from the reconciled native group, not the local type's layout. By-value callable
use is rejected; failed definitions and complete empty records cannot fall back to markers.
Incomplete unions, classes, nested records, and layout/UUID-attributed forwards remain unsupported.
`test_clang` `opaque_records.rs` checks this boundary without generating Rust consumers.

### clang remaining attribute coverage

`tool-win32` force-includes its SAL transport. Source hooks and structural wrappers preserve
contract names, source-hook arguments, and further macro expansion in `NativeSalAttribute`.
These strings are evidence, not evaluated expressions or guaranteed original header tokens.
Common direction/argument-optionality flags are projected; result nullability is distinct.
Buffer contracts use named `readable`/`writable` RDL backed by `BufferExtentAttribute`. Keep entry,
success and valid-record phases separate. Native counts support integer parameters, one dereference
and direct sibling fields; `bytes(...)` selects byte units. Full SAL buffers use `on_entry(*count)` for
successful lengths tied to entry values. Null fallbacks remain `or(*written, capacity)`.
Struct fields use `readable(length)` and `writable(capacity)` without a phase argument; their owner
establishes valid-record state. Parameter contracts keep explicit `entry` or `success`.
Returned-buffer SAL2 forms use indirect success extents. Capacity-only forms emit writable extents,
`_to_` adds readable length, and `_all_` emits only readable length using the successful count.
RPC `ecount_full` promises both writable and readable extents. Returned strings use
`null_terminated(success, indirect)`. RPC optional returned strings also require an incoming valid
nullable string: retain its indirect entry-readable extent and entry termination. Optional RPC
counted buffers instead have only `nullable(entry, indirect)` on entry; this permission must not
invent initialized/readable storage. Indirect extents select initialized storage at their phase.
`NullableAttribute` also uses return-row `-> #[nullable] T` and field-owned `#[nullable]`, without
phase properties. Return nullability is independent of success. Keep unsupported scalar/callback
pointer contracts and mixed field contracts as evidence.
Buffer-only annotations must not acquire `out_ptr`'s one-readable-element guarantee.
`if_present(*count)` leaves the extent unknown
when optional count storage is null, without weakening returned-pointer initialization/nullability.
Explicit matching returned-pointer and buffer contracts retain both guarantees, including
null-on-failure. Other distinct combinations remain evidence; do not form permissive unions.
Derive count optionality from the same unconditional facts as parameter flags; retain explicit
fallbacks instead of adding `if_present`. Native counts and ranges share bounded integer expressions:
unsuffixed decimal/hex literals fitting `i32`, grouping, addition, subtraction and multiplication.
Arithmetic uses C promotions; signed constant overflow is rejected. New count arithmetic never emits
`Offset`; legacy offsets retain full metadata spelling. Phases, optional counts and null fallbacks
remain separate. Casts, `sizeof` and unsupported operators remain evidence. Return-based counts still
need their complete success condition before lowering. Public comparison
binds count references positionally while publication retains observed names. Legacy attributes stay
readable.
Typedef reconciliation selects an observed annotation set containing every other observed set;
incomparable sets fail, and original observations remain unchanged. Native definitions and
callable observations still require matching annotations. Annotation owners that cannot be
published are rejected with their dependents, not stripped. Keep this provisional vocabulary
explicit until canonical contract attributes replace it.

Array-bound expressions are accepted through the native `windows_clang_getLayoutExpression`
query, not an expression-kind allowlist. Captured constant array extents and bitfield widths carry
the evaluated values. An incomplete array extent is `None`, distinct from `Some(0)`. Unsized
parameters use native pointer adjustment; incomplete array storage is not projected.
`windows_clang_getFieldInitializer` rejects in-class initializers, including
bitfield initializers hidden by libclang's child visitor.
Callback array parameters follow Clang's canonical pointer adjustment and preserve written element
aliases. Variable bounds and C static/qualified array-parameter contracts remain unsupported.

`clang` `capture/mod.rs` `attribute` uses native kind names, requested `MaxFieldAlignment` bits, and
evaluated `AlignedAttr` requests with their semantic form.
`test_clang` `fidelity.rs` `opaque_packing` distinguishes `pack(1)` and `pack(2)` despite identical
char-only layouts. Reconciliation checks requested packing, and projection preserves it.
Complete-record alignment projects through `#[align(N)]`; request sets are checked even when
layouts match. Member alignment uses field-owned `AlignmentAttribute`; updated bindgen preserves
field types with target-evaluated `MaybeUninit<u8>` padding and containing alignment. By-value
callables containing these members remain rejected pending ABI qualification. Typedef/enum
alignment, aligned bitfields, combined packing/alignment, and packed containment remain unsupported.
The native `TypeVisibilityAttr` query decodes default/hidden/protected requests, including the
implicit tokenless attribute on `_GUID`. `clang` retains and compares default requests; hidden
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
Helper provisioning checks native build-input and DLL fingerprints, rebuilding stale managed pairs.
Explicit `LIBCLANG_PATH` builds remain caller-owned and are never silently replaced.

Callback capture accepts plain C/stdcall prototypes after checking native function properties.
`test_clang` `opaque_callbacks.h`
distinguishes ordinary C and x86 `regparm(2)` function-pointer types with a compiler `static_assert`.
Both expose `FunctionProto`, C calling convention, identical result/parameter types, and no
attribute cursor children. `noreturn` also survives in a function type without an attribute cursor.
Type spelling and `clang_equalTypes` distinguish extra semantics but do not decode their payloads.
Explicit `__cdecl`/`__stdcall` instead expose `CXType_Attributed`. Do not assume that rejecting
attributed types makes every remaining function prototype plain C.
The sys crate's `windows_clang_getFunctionInfo` recovers these hidden properties, including
explicit `regparm(0)` versus absence, and flags additional prototype information for rejection
or later decoding. `test_clang_sys` exercises these distinctions on all three parsing targets.
Explicit regparm (including zero) and unknown properties remain rejected. Variadic imports retain
the compiler's C ABI, including ignored written stdcall requests; variadic callbacks and methods
remain rejected during publication. Native reconciliation compares the variadic flag.
`Function::noreturn` retains the native function-type bit; `Cxx11NoReturn` retains declaration-owned
standard attributes separately. Native/public comparison checks both. The shared callable writer
emits one DoesNotReturn attribute for imports, methods, and named callbacks. Non-returning anonymous
pointers require a compatible named callback, not an ordinary signature with the contract discarded.
The noreturn fixtures cover native ownership, duplicate forms, conflicts, partial dependencies,
metadata round trips, and four SDK exit/exception APIs on all targets.
Unconditional noexcept, nothrow, and empty throw() retain written and canonical exception
specifications as native-only compatibility evidence. Methods, imports and delegates do not emit
NativeExceptionSpecification. Native agreement and callback matching still compare exception
specifications. Supported unnamed non-throwing pointers can use C/stdcall FNPTR; noreturn still
requires an attribute-owning named callback. No unwind ABI or exception translation is implied.
Computed/dependent and nonempty dynamic exception specifications remain unsupported.
Callback dependency pruning follows result and all parameter edges.
Failed value-alias or callback publication must remove the declaration, not just its root marker:
reachable named values are published as dependencies, so root-only removal would retry the failure.

`windows_clang_getTypeQualifiers` checks qualifiers before `clang` type interning. CVR and
supported unaligned requests are represented, including typedef inheritance. An unaligned request
requires canonical unqualified native storage alignment one; native comparison retains the request,
but metadata needs no additional modifier. Wider or incomplete storage and remaining qualifiers
stay rejected. `byte_aligned_qualifiers` and the SDK fixture check this boundary on all targets.
Callback naming uses target-wide candidates after public-family and alias validation. Candidate
searches use the shared native/public type checker and discard assumed pairs after each failure.
When no compatible callback name exists, projection emits structural C/stdcall function-pointer
types. RDL encodes these as ECMA-335 `FNPTR` signatures; do not invent public callback names.
Named callbacks retain their delegate representation. Signature traversal must include results and
every parameter for dependency collection, namespace remapping, and nested-reference rewriting.

### clang native declaration queries

The native companion supplies UUID payloads, language linkage, and static-initializer
validity. `clang_Cursor_Evaluate` can fold `(call(), 3)` to `3`; successful evaluation alone does
not establish a constant initializer. `capture/values.rs` gates scalar evaluation with the native
static-initializer query, including explicit side-effect and undefined-behavior checks. Compiler
field-offset expressions need not be C++ core constant expressions; no macro-name or AST-shape
exceptions are used. Float values retain their IEEE bits, including signed zero;
non-finite values and extended floating-point types remain unsupported.
The integer-element query walks evaluated struct/array values with checked index paths and
zero-filled tails. String arrays remove one trailing terminator; embedded zeros remain intact.
SelectAny is storage evidence, not part of published constant-value equality.
C++ member templates, nonvirtual methods, and method using declarations are retained in
RecordDefinition::non_vtable_members; they are not exported methods or part of ABI comparison.
Parameter defaults retain the exact compiler-owned expression cursor, printed expression, and
location. NativeDefaultArgument preserves that expression without evaluating it or altering arity.
Method CVR is decoded separately from type qualifiers; NativeConstMethod preserves const methods.
Volatile/restrict and ref-qualified methods remain rejected. DirectWrite factory7/font-set3 and
Direct2D factory7/device-context6 closures publish. NativeDeprecated preserves message/replacement
payloads, including enum members and original typedef ownership.
Virtual methods retain absolute slots from MicrosoftVTableContext, not source declaration order.
Overloads retain native metadata names; existing bindgen suffixes must not collide with literal
method names. Verify contiguous slots after the inherited vtable prefix.

COM interfaces publish as opaque references, not packed value records. Check pointer-sized,
fieldless layouts against inherited alignment capped by the local packing request. Keep packing
in native reconciliation. Core vtable-pointer loads and weak-reference forwarding use the shared
alignment-safe `Interface::assume_vtable`; the vtable itself remains aligned.

Value records permit one public nonvirtual base at compiler-reported offset zero, represented by
a NativeBase field. Complete base storage and derived offsets/size/alignment must match Clang.
Inherited values and containing records/arrays remain blocked in by-value callable signatures.
Base types resolve through canonical native
declarations, including typedef-spelled bases, rather than registering typedef cursors as records.

Attributes retain decoded payloads and locations, not raw cursor-range token dumps. Macro-expanded
attribute extents can span unrelated header declarations. Unknown attributes remain explicit errors
with compact diagnostics. Macro evaluation binds the initializer expansion and synthetic declaration
by exact input-file offsets; an earlier expansion of the same name is not probe evidence.

### clang nested records

Stock `clang_Type_visitFields` exposes implicit anonymous aggregate fields and their native offsets.
Use `clang_Cursor_isAnonymousRecordDecl` on the field's type declaration to distinguish a promoted
anonymous aggregate from a named field whose record type is unnamed. The predicates on the field
cursor itself do not make that distinction. These queries work on all three parsing targets.

`clang` `Record::owner` retains semantic ownership. Named children correspond by scoped name;
unnamed children use their first owning native field. `Snapshot::record_identity` supplies the
same owner-relative key to native resolution and cross-target association. Public families check
corresponding children independently, and partial pruning removes children with their owner.
Do not use display spellings or source offsets as nested identity.

RDL preserves shared and named nested records through owner-scoped declarations and references
such as `Outer::Inner`. A native declaration such as `struct { int value; } first, second;` has
one shared type, not two independent inline types. `rdl` `writer/struct.rs` `write_record` keeps
unshared direct fields inline when their nested names match the generated field-position names.
Other nested records remain explicit declarations under their owner, not top-level helper types.

### clang replacement boundary

`clang` `projection/mod.rs` `Projector::written_ty` preserves written value-typedef and callback names
at use sites. Native agreement precedes public-name agreement across observations; conflicts are
errors, not representative-selection choices. Callback parameter evidence belongs to
`AliasObservation`; publication preserves native names, with positional names for unnamed
parameters. ABI-based callback naming excludes annotated and indirect prototypes. Relocated
typedef contracts retain `NativeSalAttribute.TypedefName`. Native agreement alone does not
establish annotation correctness. Before a production
cutover, define intended names and annotations and check the header -> RDL -> WinMD -> generated Rust
path for representative consumers and issue
reproductions. Do not count explicit partial omissions or uncaptured macros as solved coverage.

Scalar constants may use their common native primitive type when conflicting written aliases have
no attributes. Ordinary public agreement preserves unique aliases. The check follows every alias
chain through all resolved observations, including annotation-only inputs without the constant.
Native type/value/annotation conflicts still fail before pruning; enum, pointer, and callable
alias conflicts remain rejected. `scalar_constant_aliases` covers values, macros, definitions,
input order, alias-owned contracts, and constant-owned attributes through RDL and WinMD.

### clang public ownership

After native and public compatibility checks, declarations with several candidate header owners
go to `foundation.rdl`. Candidate owners are combined across targets before partitioning, so a
declaration cannot land in different files on different architectures. A unique alias header
containing its native definition remains preferred; otherwise all declaration headers participate.
Uniquely owned declarations keep their header partitions. `Partition::headers` retains the real
source-header set, including for foundation, rather than inventing a source filename.

The foundation stem is reserved for shared declarations. Missing owners and unrelated headers
colliding in one ordinary partition remain errors. No per-name/header preferences or routing
seeded from canonical metadata are used. `test_clang` `foundation.rs` covers shared aliases,
records, enums, callbacks, cross-target ownership, dependent consumers, ordering, and conflicts.

### clang primary naming

`AliasObservation::defines_tag` retains the native `TagTypeLoc::isDefinition` relationship.
`projection/plan.rs` `Plan::primary_names` runs after compatibility validation. A single native
record or enum uses its unique defining value typedef; zero or multiple defining typedefs leave
the native tag as the definition and preserve other names as aliases. This makes `GUID` the
definition behind `BATTERY_CHARGER_ID`, without a GUID-specific rule, and retains `_ACTRL_ALISTA`
when both `ACTRL_ACCESSA` and `ACTRL_AUDITA` define it.

A checked multi-record family requires a unique defining typedef binding to every member, or a
unique common alias if none defines a tag. An ambiguous family fails even in partial mode.
Do not choose a primary name alphabetically, by source/header order, or by stripping underscores.
Primary naming and output ownership are separate checks.

Named enum constants retain their native enum type. A separate scalar typedef is not an enum
alias merely because its name matches the tag without an underscore. The 239 reviewed baseline
constant-to-enum-member differences, including five signed interpretations of unchanged 32-bit
patterns, are approved. The `native_enum_types` fixture checks this distinction on all targets.

### clang metadata and consumer boundaries

`clang` `Field::bit_width` retains evaluated widths, including unnamed zero-width barriers.
Native and public-family comparisons check widths independently of layout. `projection/bitfields.rs`
groups compatible integer storage; record projection checks offsets, size, and alignment against
Clang. `test_clang` `bitfields.rs` checks RDL/WinMD member attributes and generated Rust layouts on
all three MSVC targets, plus host accessor behavior.

Validate metadata semantics and ABI independently of downstream code generation. Consumer-only
defects do not justify omitting correct metadata; keep separate reproductions and downstream TODOs.
Raw `windows-sys` bindings can check layout while high-level accessor defects remain open.

- `bindgen` `cpp_struct.rs` `BitfieldBacking::write_accessor` emits union field reads without
  `unsafe`; generated union bitfield methods fail to compile. This is a downstream TODO, not a
  clang metadata gate. Union bitfields emit as separate offset-zero storage fields.
- `NativeBitfieldAttribute.MemberType` is an optional `System.Type` property. It preserves mixed
  signedness, bool/enum types, typedef identity, and single-bit integer semantics independently of
  storage. RDL retains typed attributes, and metadata remapping updates their nominal references.
  Typed union accessors use unsafe reads; legacy untyped output retains its existing behavior.
  Updated readers are required. Unnamed padding has no member value, but its native type remains
  in reconciliation.
- MSVC unions containing only bitfields can have alignment 1 despite wider integer storage.
  Keep layout mismatches explicit until the storage alignment has a metadata encoding; do not
  invent a native packing request from the final layout.

Mixed pointer chains use ECMA-335 per-level modifiers: `File::Type` writes `IsConst` after each
`PTR` whose pointee is const. Homogeneous chains keep the existing leading-modifier encoding.
`Blob::read_type_signature` supports both without flattening mixed qualifiers. Older readers
need updating before consuming mixed chains. Metadata's `pointer_constness` and the RDL/clang
`mixed_pointers` fixtures check this boundary. Downstream Rust checks remain focused, not a coverage
gate.

### clang anonymous enum constants

`EnumObservation::anonymous` retains `clang_Cursor_isAnonymous`, not a spelling heuristic.
Projection publishes members as typed constants only when the enum has no public type name.
Compiler-provided typedef names and primary aliases retain named-enum behavior. An unnamed enum
cannot be used as a type merely because its constants can be published.

Constant agreement checks native backing type and value before combining ownership; equal-width
native types such as `int` and `long` are not interchangeable. Type and value names use separate
publication keys. Partial rejection removes the originating enum's constants on every target.
The `anonymous_enum*` fixtures cover constants, typedefs, conflicts, ownership, architecture
variants, rejection, and header -> RDL -> WinMD -> generated Rust behavior.

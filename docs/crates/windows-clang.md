# windows-clang

> Generates RDL from C/C++ headers using libclang.

- 📦 [crates.io](https://crates.io/crates/windows-clang)
- 📖 [docs.rs](https://docs.rs/windows-clang)
- 🚀 [Getting started](../../crates/libs/clang/readme.md)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/clang)

`windows-clang` is the header-facing front end of the Win32 metadata pipeline. It
parses C/C++ headers with **libclang** and emits **RDL** (Rust Definition
Language) — the text format that [`windows-rdl`](windows-rdl.md) compiles into
ECMA-335 `.winmd` metadata. The full pipeline is:

```text
headers ──(windows-clang)──▶ .rdl ──(windows-rdl)──▶ .winmd ──(windows-bindgen)──▶ bindings.rs
```

It exists because some Windows APIs ship only as C/C++ headers with no metadata of
their own. Rather than hand-author RDL, `windows-clang` recovers types, functions,
constants, enums, structs/unions, callbacks, and COM interfaces directly from the
headers — faithfully preserving SAL annotations, calling conventions, and the
symbol → DLL mapping recovered from the SDK import libraries.

## Relationship to windows-rdl

`windows-clang` **depends on** `windows-rdl` — it is a *producer* of RDL text and
reuses the reader-side crate's building blocks:

- `windows_rdl::emit` — the RDL-emission primitives (`write_ident`, `write_type`,
  `write_value`, `uuid_to_u128_literal`) shared with the winmd → RDL writer, so the
  scraper and the writer spell identical RDL.
- `windows_rdl::formatter` — pretty-prints the emitted RDL.
- `windows_rdl::implib` — the COFF import-library reader that supplies the faithful
  function → DLL mapping headers don't carry.
- `windows_rdl::Error`, `expand_input_paths`, `write_to_file` — shared plumbing.

The dependency is one-way: `windows-rdl`'s reader/writer never call into clang, so
crates that only compile or emit RDL (`tool_reactor`, `test_rdl`, `test_metadata`,
`test_bindgen`, `test_win32_metadata`, …) build without pulling in `clang-sys` or
libclang.

## Getting started

Add `windows-clang` as a build dependency (it is run from a codegen tool, not
shipped at runtime):

```toml
[build-dependencies]
windows-clang = "0.0.0"
```

Point `clang()` at one or more headers and write the resulting per-header RDL. Each
header is parsed as its own translation unit — only its own top-level declarations
are emitted, not the things it `#include`s — so list every header you need as a
separate `input`, and pass a reference `.winmd` so cross-header type references
resolve:

```rust,no_run
windows_clang::clang()
    .args(["-x", "c++", "--target=x86_64-pc-windows-msvc"])
    .input("Example.h")
    .input("crates/libs/bindgen/default/Windows.Win32.winmd")
    .output("example.rdl")
    .namespace("Example")
    .library("Example.dll")
    .write()
    .unwrap();
```

`clang_version()` returns the loaded libclang's version string; the tooling pins a
specific libclang release so the scrape is deterministic (see `tool_win32`).

## Consumers

- `tool_win32` — scrapes the Windows SDK headers into
  `crates/libs/bindgen/default/Windows.Win32.winmd` (the in-house Win32 metadata
  that backs `windows-bindgen`'s bundled `"default"` bindings).
- `tool_wdk` — scrapes the WDK kernel-mode headers into
  `crates/libs/bindgen/default/Windows.Wdk.winmd`, an *additive* winmd carrying only the
  surface the WDK adds on top of Win32 (see [tool_wdk](#the-wdk-corpus-tool_wdk)).
- `tool_webview` — scrapes the WebView2 headers into `WebView2.rdl`.
- `test_clang` — golden fixtures that pin the header → RDL behavior.

## Internals

The scraper is organized by declaration kind: `cx` wraps `clang-sys` (the AST
cursor/translation-unit types), `canon` holds the faithful → canonical type
remaps, and per-kind modules (`r#enum`, `r#struct`, `r#const`, `r#fn`, `callback`,
`interface`, `typedef`, `field`, `annotation`) parse a cursor and emit RDL. The
`collector` accumulates items per namespace; `item` dispatches emission. Header
partitioning routes each declaration back to its defining header so the output is a
per-header `<stem>.rdl` file set.

Because `windows-clang` reuses `windows_rdl::emit`, the RDL it produces is spelled
identically to the RDL the winmd → RDL writer produces — the round-trip
`headers → RDL → winmd → RDL` converges, which the golden tests enforce.

## Faithful, source-expressed metadata

The scrape departs deliberately from win32metadata's design. win32metadata aims to
be *higher level* than the headers — it synthesizes friendly constructs (loose
constants promoted to enums, handle lifetimes, struct-size fields) encoded with a
large vocabulary of custom attributes, backed by an enormous body of hand-curated
configuration (`libMappings.rsp`, `enums.json`, `documentationMappings.rsp`, …). We
reproduce **none** of that curation. The goal is metadata *faithful to the original
source* — emitting only what is **directly expressed in the source language**:

- **SAL** — `_In_`/`_Out_`/`_Inout_`/`_opt_` (direction, optionality),
  `_In_reads_`/`_Out_writes_`/`_..._bytes_` (sizing), `_Reserved_`, `_COM_Outptr_`,
  `_Ret_*`, the `_z_` string forms.
- **`__declspec`** — `uuid(...)`, `noreturn`, `align(n)`, `dllimport`, `deprecated`.
- **Language constructs** — `const`, scoped enums, `#pragma pack` / declared
  alignment, unions, bitfield widths, `typedef`, calling conventions.
- **Header signal macros** — `DEFINE_ENUM_FLAG_OPERATORS(E)` (a genuine flags-enum
  signal, not a guess).
- **IDL attributes** (COM) — `[in]`/`[out]`/`[retval]`/`[size_is]`/`[iid_is]`/… .

Anything *not* in the source — handle disposal, struct-size auto-fill, doc URLs,
OS-version availability, `SetLastError` behaviour, loose-constant→enum promotion — is
**out of scope for the metadata**. Where such ergonomics are valuable (RAII handle
wrappers) they belong in the hand-written `windows` projection layer, not smuggled
into the metadata as synthetic attributes.

The **only** hand-authored input is `metadata/win32/metadata.rdl` — the attribute
vocabulary (`NativeTypedefAttribute`, `SupportedArchitectureAttribute`,
`DoesNotReturnAttribute`, …) that exists only in metadata, so no C header can declare
it. Everything else — including the `Foundation` scalars `BOOL`/`HANDLE`/`HRESULT` —
is header-derived. Semantic scalars survive not through a seed list but because
scalar collapse is *opt-in*: `fundamental_scalar` / `pointer_sized_abi` (`canon.rs`)
are curated allowlists of names that collapse to a Rust primitive (`DWORD` → `u32`);
everything else stays a distinct `NativeTypedef`, and `HANDLE` is recognized
structurally.

### UNICODE is deliberately not defined

The translation unit is built *without* `UNICODE`/`_UNICODE` (only `SECURITY_WIN32`
is predefined). This is measured, not accidental: defining `UNICODE` drops 71 real
exported functions and adds none — every family whose bare, unsuffixed name is the
ANSI export guarded by a `#define name nameW` alias (the whole winldap `ldap_*`
family, bare-ANSI wininet functions) disappears because the bare name macro-expands
to the `…W` symbol before clang sees it. The only upside is that generic text-mapped
*typedef aliases* would flip Unicode-first (`LPTSTR` → `PWSTR`) — alias spellings, no
new symbols. The no-`UNICODE` scrape captures both the bare-ANSI and explicit `…W`
declarations as distinct symbols, so it is strictly higher-coverage. (win32metadata
achieves Unicode-first without the loss by scraping ANSI and Unicode in separate
passes and merging — a much larger change than a compile flag.)

## Partitioning: by defining header, not editorial namespace

The Windows SDK is a **flat C namespace**: symbol names are globally unique (a probe
over the reference winmd found only 8 of 34,552 type names — 0.02% — in more than one
namespace, all genuinely distinct COM interfaces). Namespaces are an **editorial
overlay** win32metadata invented, with no source signal to recover, so re-deriving
that taxonomy from source is a category error.

The canonical RDL therefore lives in a **single flat namespace** (`Windows.Win32`),
split **one file per defining header** — the one faithful, mechanical routing signal:

- **clang-native** — every cursor carries its source location, so the defining header
  is intrinsic and total; it selects the output *file* (`winnt.rdl`, …), not a
  namespace.
- **Stable** — a symbol's defining header barely moves across SDK versions (clean git
  diffs); DLLs drift (forwarders, API sets), so per-DLL keying would churn the layout.
- **USR-deduped** — clang's USR gives each entity one identity across every
  redeclaration, so each is emitted exactly once and cross-file references are bare
  names resolving within the flat namespace.

The exporting **DLL is kept as a faithful per-function attribute**
(`#[library("…")]`, recovered from the import libs), so per-DLL truth survives as
*data*. Any friendlier grouping (the legacy `windows` module layout, per-DLL views)
becomes an optional downstream map over the flat namespace.

## The in-house corpus: tool_win32

```text
   Windows SDK ──► clang / libclang ──► one RDL file per header ──► Reader ──► per-arch winmd
   • headers          (windows-clang)      (metadata/win32/*.rdl)                      │
   • SAL              parse shared TU once, walk once, USR-dedup, flat namespace       ▼ merge
   • import libs      (+ hand-authored attribute seed metadata.rdl)           Windows.Win32.winmd
     (symbol → DLL)                                                                    │
                                                                                       ▼ windows-bindgen ──► Rust
```

`cargo run -p tool_win32` builds the in-house winmd. A small manifest
(`crates/tools/win32/src/win32.toml`) lists the SDK headers, satellite rules, and
import libs — deliberately **no type-level curation**. The driver builds one shared
translation unit per target arch (`windows.h` prelude + every manifest header), emits
it via a single `write_by_header` call (parsed once, USR-deduped), reads the per-arch
RDLs to winmd, and coalesces the arches with the multi-arch merge. New APIs are added
by **listing the defining header** in the manifest and regenerating — the reachability
closure is automatic. A full generational SDK bump (e.g. 24H2 → 25H2) is absorbed the
same way, with no scraper changes.

Functions whose exporting DLL cannot be recovered from any import library in the
manifest are **dropped** (matching the reference, which carries none): a function with
an empty `#[library("")]` would otherwise force `windows-bindgen` to emit
`link!("" …)`, a hard `E0454` error. This is the opt-in `drop_lib_less` flag, set only
by `tool_win32`; unit-test fixtures that supply no import libraries leave it off so
they still emit their functions.

## The WDK corpus: tool_wdk

`cargo run -p tool_wdk` builds `Windows.Wdk.winmd` the same way `tool_win32` builds the
Win32 winmd — a whole-header scrape, not a symbol allowlist. It mirrors that driver with
three deliberate simplifications:

- **Same flat `Windows.Win32` namespace.** The WDK surface is emitted into the *global,
  not-WinRT* namespace shared with Win32, so a WDK entity referencing a Win32 type
  (`NTSTATUS`, `LARGE_INTEGER`, `LIST_ENTRY`, …) just names it and bindgen resolves it by
  bare name once both winmds are loaded together — no cross-namespace resolution.
- **Additive (exclusion-referenced).** The Win32 winmd is fed to `write_by_header` as an
  *exclusion reference*: any entity Win32 already defines is skipped rather than re-emitted,
  so `Windows.Wdk.winmd` carries **only the WDK-net-new surface**. This sidesteps the
  reader's duplicate-`(namespace, name)` panic and keeps the winmd small (only kernel types
  like `IO_STATUS_BLOCK` land; the shared closure resolves against Win32).
- **User-mode only.** `scope(["km"])` keeps kernel-mode declarations in scope, and
  `drop_lib_less` + the `ntdll.lib` import library scope the emitted *functions* to the
  user-callable native surface: a routine exported from `ntdll.dll` (`NtReadFile`,
  `RtlGetVersion`, …) is stamped and kept, while a kernel-only routine (exported from
  `ntoskrnl`, absent from any user-mode import library) resolves to an empty library and is
  dropped. Crucially, **no fallback `library`** is set — one would make every function
  lib-ful and drag in the whole kernel export surface.

New WDK APIs are added exactly like Win32 ones: list the defining header in `SOURCE_HEADERS`
and regenerate. The WDK NuGet version is pinned independently of the SDK (its servicing build
lags), but tracks the same marketing line.



The scrape is deliberately **faithful to the C headers** — but a small,
explicitly-tracked set of places *tip the scales* to recover the semantics the SDK
authors clearly intended (but that decades of C typedef/`#define` conventions erased
at the type level). This ledger is the single source of truth for **every** such
bias. The rule: keep this list short, keep each entry justified by original intent
(not by matching win32metadata for its own sake), and add nothing here silently. If a
change makes the metadata diverge from a literal reading of the headers, it belongs in
this table with a rationale.

| # | Deviation | Rule (automatable — no per-symbol list) | Preserve-set (the only curation) | Where | Rationale |
|---|-----------|------------------------------------------|-----------------------------------|-------|-----------|
| 1 | **Non-negative integer constants → `u32`/`u64`** | A `#define` literal's suffix (`L`/`LL`) encodes *width*, not *signedness*; default unsigned, negated → `i32`/`i64`, explicit keyword/named cast wins. | — | `const.rs` | `L` means "at least `long`", never "signed value". Win32 flag/id constants are unsigned domains. |
| 2 | **`_HRESULT_TYPEDEF_(x)` etc. → typed `HRESULT`** | A hardcoded 3-macro map of the canonical SDK error-typedef cast macros routes through `parse_named_cast`. | 3 SDK macros (`_HRESULT_TYPEDEF_`, `_NDIS_ERROR_TYPEDEF_`, …) | `const.rs` | The macro *is* the author's type annotation; `E_FAIL` is an `HRESULT`, not a bare `i32`. |
| 3 | **`LSTATUS` → `WIN32_ERROR` (`type WIN32_ERROR = u32`)** | `error_code_typedef` remaps the one canonical error-code typedef at both its definition and every reference. | 1 typedef (`LSTATUS`) | `canon.rs`, `typedef.rs` | `LSTATUS` is definitionally an unsigned `ERROR_*` domain despite its signed `LONG` spelling. Plain `u32` typedef, **not** win32metadata's synthetic enum. |
| 4 | **Pointer const-ness follows SAL direction (parameters)** | `_In_`/`_In_opt_`/`_Reserved_` (read-only) pointer param → `*const`; `_Out_`/`_Inout_` (writable) → `*mut`. Overrides the C typedef's own mutability. A string wrapper flips its *named* const variant instead (`PWSTR`↔`PCWSTR`, `PSTR`↔`PCSTR`). | — | `canon.rs` (`apply_sal_constness`), from `fn.rs`, `interface.rs` | SAL is the author's read/write contract; `_In_ LPWSTR` is a *read-only* string despite `LPWSTR` being `*mut`. Matches the reference. Parameters only — struct fields keep their C const-ness. |
| 5 | **`LP*`/`P*` pointer aliases collapse to raw pointers (parameters only)** | At a parameter site, a typedef whose one-level underlying type is a pointer is inlined to the raw pointer it spells, keeping the *named* pointee (`LPDWORD`→`*mut DWORD`, `PHKEY`→`*mut HKEY`). The string wrappers are normalised *everywhere*, not just parameters (see #9). | Kept named: string wrappers (per #9); **`BSTR`** (a length-prefixed, `SysAllocString`-owned COM string, never collapsed to `*const OLECHAR`); handles (`HANDLE` = `void*`, `HWND` = `struct HWND__*`); function-pointer aliases (`FARPROC`); non-pointer aliases. | `canon.rs` (`collapse_pointer_alias_param`; name-keyed cases in `alias_policy`) | These aliases are pure portability spelling (`LPDWORD`=`DWORD*`); the pointer *is* the ABI, and SAL const (#4) cannot be expressed while it is hidden inside an opaque `*mut` alias. Collapsing **only at parameter sites** keeps the change surgical — fields, returns and constants keep their named aliases, so no `type LP* = …` is left dangling. The pointee keeps its Win32 name (`*mut DWORD`): ABI-identical. |
| 6 | **Fixed-width portability scalar aliases → primitive** (`DWORD`→`u32`, `WORD`→`u16`, `LONG`→`i32`, `WCHAR`→`u16`, the `INTn`/`UINTn` and C99 `intN_t`/`uintN_t`) | A curated name list (`fundamental_scalar`) collapses only the pure width spellings to their primitive at every use site; the list is *shared* with the const-cast collapse so a typedef and any constant typed by it never disagree. | The collapse-list **is** the curation — every other scalar stays named: `HFILE`/`ATOM`/`COLORREF`/`LANGID`/`LCID`/`BOOL`/`BOOLEAN`/`NTSTATUS`/`HRESULT`, and the pointer-sized aliases (`ULONG_PTR`/`SIZE_T` → `usize`, via `pointer_sized_abi`). | `canon.rs` (`fundamental_scalar`), `typedef.rs`, `const.rs` | `DWORD`=`unsigned long` is pure C portability spelling; `HFILE`(`int`)/`DWORD`(`unsigned long`) are *identical* at the type level — only the **name** separates a domain concept from width noise, so the curation must be name-keyed. Collapsing the width spellings matches the reference; the preserved domain names stay faithful to the header. |
| 7 | **Floating-point typedefs → `f32`/`f64`** (`FLOAT`, `DOUBLE`, `DATE`, `REFTIME`, OLE/GL/SQL float aliases, chained `UI_ANIMATION_SECONDS`→`DOUBLE`) | **Structural, no name list**: a typedef whose *canonical* kind is `float`/`double`/`long double` collapses to the primitive at every use site and its definition is suppressed (`floating_typedef`). | — (the float side has no domain/noise split to preserve) | `canon.rs` (`floating_typedef`), `typedef.rs` | The reference metadata contains **zero** floating `NativeTypedef`s — win32metadata drops *every* one. Unlike the integer side (#6), the float side is uniform, so the rule is **structural** (by canonical kind) rather than a curated list — self-maintaining as the SDK grows. |
| 8 | **`MAKEINTRESOURCE`-family ordinal constants → `PWSTR`/`PSTR` carrying the id** (`IDC_*`, `IDI_*`, `RT_*`, …) | A curated 3-name map (`makeintresource_macro`) recognises `#define X MAKEINTRESOURCE(n)` from the *raw* `#define` body and emits `EnumValue(PWSTR/PSTR, n)` so the constant survives. | 3 macro names (`MAKEINTRESOURCE`/`MAKEINTRESOURCEW`→`PWSTR`, `MAKEINTRESOURCEA`→`PSTR`) | `const.rs` (`makeintresource_macro`) | The macro expands to a *string pointer that holds an integer id*, so the batch evaluator drops it as pointer-valued and the id is lost. The macro name *is* the author's intent ("this is a resource ordinal named as a string"). Bare `MAKEINTRESOURCE` is treated as wide (the scrape runs without `UNICODE`). |
| 9 | **String-pointer aliases → canonical `PWSTR`/`PCWSTR`/`PSTR`/`PCSTR` (universal, flat scrape)** (`LPWSTR`/`LPCWSTR`/`LPSTR`/`LPCSTR`, the OLE `LP*OLESTR`/`P*OLESTR`, bare `P*`/`PC*`) | A string-wrapper alias normalises to its canonical variant at **every** site — field, return, *and* parameter — via `normalize_string_alias` in `to_type` (const-ness follows the spelling; SAL then re-selects the variant for parameters, #4). Redundant alias *definitions* are suppressed. Gated to the flat per-header scrape (`header_root.is_some()`). | Kept: the four canonical wrappers; **`BSTR`** (per #5). | `canon.rs`, `typedef.rs`, `const.rs` | bindgen's core string projection recognises **only** the four canonical spellings, so an `LPCWSTR` *field* would degrade to a raw `*const u16` while an `LPCWSTR` *parameter* became an ergonomic `PCWSTR` — an accidental asymmetry. Normalising everywhere removes it and matches the reference. Flat scrape only: a namespaced scrape resolves against a reference winmd that lacks the const wrappers, so it keeps its verbatim `LP*` locals. |
| 10 | **Raw null-terminated string parameters → canonical wrappers (from the `_z_` SAL bit)** | A *bare* string pointer with no named alias (`_In_z_ WCHAR const*`, …) is promoted to the canonical wrapper via `promote_null_terminated_string`: `*const`/`*mut` of `u16`→`PCWSTR`/`PWSTR`, `i8`/`u8`→`PCSTR`/`PSTR`, const-ness from the post-SAL variant (#4). Captured for the **five pure `_z_` variants only** (`_In_z_`, `_In_opt_z_`, `_Out_z_`, `_Inout_z_`, `_Inout_opt_z_`). Flat scrape only. | Excluded — stays a raw pointer: the **counted** `_z_` variants (`_In_reads_z_`, …), which carry a `NativeArrayInfo`/`MemorySize` and must remain a sized buffer. | `canon.rs` (`promote_null_terminated_string`), `annotation.rs` | Recovers the ergonomic string projection for parameters the SDK spells as a raw `WCHAR const*` rather than a named alias (#9 only fires on the alias). The `_z_` SAL bit *is* the author's null-terminated-string contract, so promoting on it — and never on pointee width alone — is faithful. Gating on `_z_` + no count keeps counted buffers (which need their length metadata) raw. |
| 11 | **Direct2D 1.1 `D2D1_X` compatibility synonyms → shared `D2D_X` base (flat scrape)** (the 16 `typedef D2D_X D2D1_X` re-exports) | A `D2D1_`-prefixed synonym collapses to its shared `D2D_` base at **every** reference via `d2d_compat_alias` (curated name-map). The redundant `type D2D1_X = D2D_X` alias *definitions* are suppressed. Flat scrape only. | Kept: every `D2D1_` type without a shared `D2D_` base (the enums, `D2D1_PIXEL_FORMAT`, …). | `canon.rs` (`d2d_compat_alias`), `typedef.rs` | The Direct2D 1.1 headers re-export the shared `D2D_*` primitives under a `D2D1_`-prefixed spelling with no distinct ABI. Keeping them left struct fields spelled `D2D1_MATRIX_3X2_F`, so bindgen's numerics substitution (keyed on the base `D2D_*`) never fired. Collapsing to the base makes the numerics-mapped members reach the substitution table (`D2D_POINT_2F`→`Vector2`, …) — matching the reference-generated canvas bindings. |
| 13 | **Array *parameters* decay to pointers (C11 §6.7.6.3p7)** | An array *parameter* (`to_type` maps both `T[]` and `T[N]` to `ArrayFixed`) decays to a pointer when it is **unsized** (a flexible array `T[]` → `[T; 0]`, which is a zero-byte by-value drop) **or** carries a SAL count (`_Out_writes_(80) WCHAR szName[80]`): the `NativeArrayInfo` (`#[len_const]`/`#[len_param]`) already encodes the length, so keeping the `[T; N]` *type* too would double-encode it and make bindgen wrap `&mut [[T; N]; N]`. Pointee const-ness follows the array element's C const-ness; SAL (#4) may override. | A *plain* fixed-size array parameter with **no** count (`FLOAT ColorRGBA[4]`) stays `[T; N]`, so bindgen projects a length-checked `&[T; N]`. Struct fields, returns and constants keep their array shape. | `canon.rs` (`decay_array_param`), from `param_metadata_type` | An `ArrayFixed` is faithful for a *struct field* but wrong for a *parameter* — a by-value array is not a real ABI. The reference (`_Out_writes_(80) WCHAR szName[80]` → `*mut u16` + count → `&mut [u16; 80]`; `FLOAT BlendFactor[4]` + count → `Option<&[f32; 4]>`) decays counted buffers to pointer+count and keeps uncounted fixed arrays; this rule reproduces both exactly. |
| 14 | **Mixed-constness pointer chains → uniform (outermost level wins)** | A `PtrMut(PtrConst(T))` / `PtrConst(PtrMut(T))` chain collapses to a uniform chain governed by its *outermost* level (the parameter's real read/write direction, already set by #4): an output `const wchar_t **` retval → `*mut *mut u16`; an input `const wchar_t * const *` → `*const *const u16`. | — (uniform chains are already collapsed to one node by `to_type`, so only a genuinely mixed chain nests here). | `canon.rs` (`normalize_pointer_const_chain`), from `param_metadata_type` | The winmd `Type` model stores a pointer run as a **single** const bit + depth, so it cannot represent `*mut *const T`; serialising one silently corrupts it (the mid-chain `IsConst` modifier is misread and the run degrades to `*const *const T`, a retval the callee cannot `.write()` through). Normalising to the outermost level matches the reference (`ISAXLocator::getPublicId` → `*mut *mut u16`). |

**Guard-rails.** The macro deviations (#2, #8) are matched by *name* from the raw
`#define` body, so a function-like `#define` of the same name is skipped. The
pointer-parameter deviations (#4/#5) touch *every* pointer parameter in the corpus, so
they are validated by re-scraping and compiling the bootstrap consumers against the
in-house winmd — a `*mut`/`*const` mismatch there is the signal that a rule has
strayed. The preserve-sets are the entire curated surface; everything else is derived
from the header text.

### Structural rules vs. one name-keyed policy table

The core discipline that keeps the editorial surface auditable: **structural whenever
the behaviour is uniform, name-keyed only where structure genuinely can't
disambiguate.** The integer collapse (#6) is *selective* — `HFILE` and `DWORD` are
byte-identical and only the name marks one as a domain type worth keeping — so it must
be a name list. The float collapse (#7) is *uniform* — the reference keeps no float
typedef at all — so it is structural and self-maintaining.

The alias handling is split the same way:

- **Structural classifiers** — *interface* (walks the base chain), *handle* (`void*` /
  `struct X__*`), *function-pointer*. Decided from the type's shape alone; no name
  list; self-maintaining.
- **One name-keyed policy table** — `alias_policy(name)` (`canon.rs`) is the *single
  source of truth* for the aliases structure **cannot** disambiguate: `BSTR` and
  `LPCWSTR` are both `wchar_t*`. `AliasPolicy` has exactly two cases —
  `String { canonical, mut_name, const_name }` (the string wrappers; SAL flips the
  const variant) and `KeepNamed` (`BSTR`). Adding a future opinionated alias (e.g.
  `VARIANT_BOOL`) is one new row here, and the audit of "where are we tipping the
  scales by name?" is that one function.

The generic void-pointer aliases (`PVOID`/`LPVOID`/…) are a third name-keyed list,
`void_pointer_alias`, collapsing *everywhere* (like `fundamental_scalar`), so a
`void*` data pointer collapses while a `void*` handle (`HANDLE`) stays named
structurally.

## Type remapping — one canon surface

Every place a faithful type is rewritten is split across **two layers**. Keeping them
straight — and consolidating Layer A into one ordered module — is what keeps the
editorial surface auditable.

**Layer A — `windows-clang` (headers → winmd).** The faithfulness rules live in a
single `canon.rs` with one declared precedence order: universal rules
(`resolve_typedef`, every site) then a parameter-only overlay (`param_metadata_type`).

| # | Rule | Helper (`canon.rs`) | Example | Sites |
|---|------|---------------------|---------|-------|
| 1 | String-wrapper normalise | `normalize_string_alias` | `LPCWSTR` → `PCWSTR` | all |
| 2 | Win32 error domain | `error_code_typedef` | `LSTATUS` → `WIN32_ERROR` | all |
| 3 | Fixed-width scalar collapse | `fundamental_scalar` | `DWORD` → `u32` | all |
| 4 | Floating collapse | `floating_typedef` | `FLOAT`/`DATE` → `f32`/`f64` | all |
| 5 | Pointer-sized collapse | `pointer_sized_abi` | `ULONG_PTR` → `usize` | all |
| 6 | GUID synonym collapse | `guid_alias` | `IID`/`CLSID` → `GUID` | all |
| 7 | Void-pointer collapse | `void_pointer_alias` | `PVOID` → `*mut c_void` | all |
| 8 | D2D compat synonym | `d2d_compat_alias` | `D2D1_POINT_2F` → `D2D_POINT_2F` | all |
| 9 | Pointer-alias collapse | `collapse_pointer_alias_param` | `LPDWORD` → `*mut DWORD` | param |
| 10 | SAL const flip | `apply_sal_constness` | `_In_ LPWSTR` → `PCWSTR` | param |
| 11 | `_z_` string promotion | `promote_null_terminated_string` | `_In_z_ WCHAR const*` → `PCWSTR` | param |
| 12 | Namespace requalify | `requalify_string_alias` | canonical string in namespaced scrapes | param |

Definition suppression (`typedef.rs`) must always stay paired with the reference-site
collapse, or a use-site dangles. Each row maps onto a precedence rank, so a new
special case is one entry at a declared rank — not a splice into an `if`/`else` chain —
with its suppression counterpart called out by the same rank.

**Layer B — `windows-bindgen` (winmd → Rust).** Two kinds of rewrite live here:

- **Genuine gen-time projection — permanent.** `Type::remap` swaps a faithful C type
  for an ergonomic Rust one the winmd deliberately keeps real: the **D2D →
  `windows-numerics`** mapping (`D2D_MATRIX_3X2_F` → `Matrix3x2`, `D2D_POINT_2F` →
  `Vector2`, …), `HSTRING` → `String`, `IInspectable` → `Object`. Correct against
  *both* winmds. This is why ledger deviation #11 exists — without collapsing the
  `D2D1_*` synonyms to their `D2D_*` base, the numerics substitution (keyed on the base
  name) never fires.
- **Faithfulness duplication — gated for deletion.** `to_const_type`/`to_const_ptr`
  re-derive string const-ness (a second copy of Layer A rule 10), needed only because
  the *reference* winmd still lacks the const wrappers. Dead once `tool_package`
  retires the reference winmd; removed in lockstep, never before.

## Namespaced scrapes and the convergence path

The universal string normalisations (#9/#10) are gated to the **flat** per-header
scrape (`header_root.is_some()`). A *namespaced* scrape — `tool_webview`, which scrapes
`WebView2*.h` into a small `WebView2` namespace and resolves external Win32 types
against a winmd via `ref_map` — must not apply them, because it would rewrite a
reference to a canonical while suppressing the local definition, leaving the reference
dangling. `requalify_string_alias` (`canon.rs`, ledger #12) re-qualifies a canonical
string alias present in `ref_map` to the `ref_map` namespace so the local definition
still resolves. `test_clang` exercises **both** modes so the carve-out gating cannot
silently regress.

## Robustness

The scrape's guiding invariant: **every type, constant, and function in the corpus is
derived from the SDK headers**, the single exception being the hand-authored attribute
seed `metadata.rdl`. The `windows-clang` diff is defensively written throughout:
checked-sub token scans, poison-macro isolation in the const evaluator, USR-keyed dedup
with definition-outranks-forward-decl, and deterministic AST-value gating instead of
diagnostic parsing. The bitfield/alignment layout math matches MSVC, and C literal
parsing decodes the full C escape set (a narrow string is kept only if it is valid
UTF-8, so a raw byte array is omitted exactly as the reference omits it).

MIDL compiler-internal names are filtered as a category: the `_User*` wire-marshaling
quartet, the `_Proxy`/`_Stub` stubs, `__MIDL__*` synthetic parameter names, and the
`__MIDL___MIDL_itf_*` file-scope placeholder tags are all recognised and dropped or
renamed, so no generated-stub plumbing leaks onto the public surface.

## Toolchain provisioning

The `provision` module fetches the pinned libclang and NuGet packages the scrapers
depend on, so a fresh checkout regenerates without a manual `pip install` /
`nuget restore`. It is shared by every consumer (`tool_win32`, `tool_wdk`, …) rather
than duplicated per tool:

- `ensure_libclang()` — respects an explicit `LIBCLANG_PATH`; otherwise downloads the
  pinned host-arch `libclang` wheel and points `LIBCLANG_PATH` at it. Call it once at
  the start of `main`, before the first libclang load.
- `assert_libclang_version()` — fails fast if the loaded libclang does not match the
  pinned `LIBCLANG_VERSION` (clang's capture behavior drifts across versions).
- `clang_resource_dir()` — resolves a `-resource-dir` of version-matched builtin
  headers (needed only for the non-x64 arch passes), fetching them on first use.
- `nuget_package(id, version)` — resolves a restored NuGet package (global-cache or
  flat `packages.config` layout), fetching the pinned nupkg from nuget.org on a miss.
  The SDK/WDK version pins stay in each tool since they diverge.

The download cache is keyed by version under `target/windows-clang/`, so all tools
share one libclang wheel and one resource-header extract.

## Outstanding work

None of these block use of the crate; they are tracked design and coverage items.

### Coverage triage

The in-house corpus only covers the headers listed in `win32.toml`, so an API a
developer needs can be silently absent (the reference winmd scraped a broader
surface). Most gaps are a **one-line `win32.toml` addition** — list the defining
header and regenerate; the reachability closure does the rest. Open issues on
[`microsoft/windows-rs`](https://github.com/microsoft/windows-rs/issues) and
[`microsoft/win32metadata`](https://github.com/microsoft/win32metadata/issues) are the
signal for *which* headers matter. Not everything is reachable from the pinned SDK:

- **Feature, not a header add:** `PROPERTYKEY`/`DEVPROPKEY` struct constants (`PKEY_*`,
  `DEVPKEY_*`, ~1300 of them — win32metadata#2090/#2100/#1773) are struct-*valued*
  constants (`DEFINE_PROPERTYKEY` / `DEFINE_DEVPROPKEY`). The scraper only special-cases
  `DEFINE_GUID` today, so closing this is a 3-crate feature: detect the macro expansions
  (`windows-clang`), add an RDL surface for a struct-typed constant field
  (`windows-rdl`), and encode the constant blob (`windows-metadata`).
- **WinRT interop headers** (`windows.ui.interop.h`, … — win32metadata#2186):
  `GetWindowFromWindowId` takes a WinRT *projection* type (`ABI::Windows::UI::WindowId`),
  which cannot be scraped without dragging in the `ABI::Windows::*` surface the manifest
  deliberately excludes. A scope decision, not a bug.
- **Toolchain/out-of-SDK blocks:** X3DAudio (`x3daudio.h`, win32metadata#2045) pulls in
  `DirectXMath.h` → `cpuid.h`, a compiler-intrinsic header not on the scrape include
  path (same class blocks any DirectXMath-dependent header). DISM (`dismapi.h`) and
  WIMGAPI (`wimgapi.h`) ship in the Windows ADK, not the pinned SDK NuGet packages, so
  they are out of scope for an SDK-based scrape.

### Parsing fidelity

- **Remaining SAL bypasses (low priority).** Two surfaces bypass the shared
  `parse_params`: struct fields (`_Field_size_` is not mapped to
  `NativeArrayInfo`/`MemorySize` — parity-only, bindgen never consumes field
  array-sizing) and callback return attributes (no `retval` detection on callbacks,
  moot since a callback's caller owns any return contract).
- **`fastcall` projection is lossy.** The winmd can faithfully record a
  `__fastcall` callback, but `windows-bindgen` silently downgrades it to
  `extern "system"` because Rust's `extern "fastcall"` fn-pointer ABI is nightly-only.
  Moot for the current corpus (zero fastcall callbacks); if one ever appears it needs a
  policy decision.

### Performance

- **Direct-to-winmd in-memory scrape** — build the winmd without the intermediate
  per-header `.rdl` write, keeping RDL as the committed source-of-truth / regeneration
  path.
- **Cache import-lib resolution** — each of the three arch workers re-runs the
  import-lib filesystem scan; resolve once and cache.
- **PCH / preamble caching** for the shared translation unit.

### tool_wdk: whole-header model

**Done** — `tool_wdk` now mirrors `tool_win32` (whole-header, flat `Windows.Win32`
namespace, additive/exclusion-referenced, user-mode-only). See
[The WDK corpus: tool_wdk](#the-wdk-corpus-tool_wdk). Remaining follow-ups: a shared
manifest (`wdk.toml`) so both tools run the same driver from a declarative config, and
per-arch WDK package wiring (the function → DLL map is arch-invariant, so the corpus is
currently x64-derived).

### IDL as the COM source of truth

Parse `.idl` (or `midl` output) directly for COM to recover the pointer-shape
attributes the headers don't express as SAL (`[unique]`/`[length_is]`/`[iid_is]`),
keeping the header path for flat C APIs.

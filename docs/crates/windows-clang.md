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

### Faithful signatures that look less ergonomic are not defects

Because the scrape emits only what the source expresses, some signatures look
lower-level than the hand-curated `windows` crate. These are **correct**: the extra
ergonomics in `windows` come from win32metadata hand-patches with no basis in the
header, not from anything the scraper is missing. Four representative cases, all
exercised by `crates/samples/reactor/direct2d`:

- **`D2D1CreateFactory`.** The header is `_In_ REFIID riid, _Out_ void **ppIFactory`
  with **no SAL linking the two**, so the faithful signature is the raw
  `riid: *const GUID, ppIFactory: *mut *mut void`. The `windows` crate's generic
  `D2D1CreateFactory<T>() -> Result<T>` is a manual win32metadata remap, not
  header-derived — the same `REFIID`+`void**` creator idiom as `CoCreateInstance`.
- **`IDXGISwapChain::Present`.** The header returns `HRESULT` with no signal that it
  can yield multiple *success* codes (e.g. `DXGI_STATUS_OCCLUDED`). Because it has no
  `[retval]`, `windows-bindgen` projects it faithfully as `-> HRESULT`, so a caller can
  observe the success code directly and call `.ok()` when they only want a `Result<()>`.
  This matches the `windows` crate, which returns raw `HRESULT` because win32metadata
  tags it `CanReturnMultipleSuccessValues` — but here it falls out of the header with no
  hand-patch. (See the "prefer `HRESULT` over `Result` for methods that lack `retval`"
  behavior in `docs/crates/windows-bindgen.md`.)
- **`D3D11CreateDevice` `Flags`.** The header parameter is `UINT`, so it projects as
  `u32`; the matching `D3D11_CREATE_DEVICE_FLAG` values are a separate scoped enum
  (hence a `.0 as u32` bridge at the call site). win32metadata retypes the parameter
  to the enum.
- **`D3D11CreateDevice` `Software`.** The header is a bare `HMODULE` with **no
  `_In_opt_`**, so it is required, not `Option` — pass `HMODULE::default()` for the
  null case. Optionality that lives only in prose documentation is out of scope (see
  above).

None of these is a scrape gap. The one genuine bug found in this area — `_COM_Outptr_opt_`
losing its `[Optional]` flag (the `specstrings_strict.h` shim clobber) — was fixed and is
covered by `crates/tests/libs/clang/input/interface_outptr_opt.h`; contrast that with the
four above, which are faithful by design.

### Overloaded virtual methods are emitted in reverse (MSVC vtable order)

A COM/C++ interface method's position in the metadata **is** its vtable slot — bindgen
lays the generated function pointers out in emission order, so the order the scraper
records must match the order MSVC lays the virtual functions into the vtable. For most
interfaces declaration order and vtable order are identical, but there is one exception:
**MSVC emits a run of consecutive same-name (overloaded) virtual functions into the
vtable in *reverse* declaration order.** A header that declares

```cpp
STDMETHOD(SetOffsetX)(IDCompositionAnimation *animation) PURE;  // declared 1st
STDMETHOD(SetOffsetX)(float offsetX) PURE;                      // declared 2nd
```

places the `float` overload in the *earlier* vtable slot and the animation overload in
the *later* one. Non-overloaded methods, and singleton "runs" of one, keep their order;
only the overloaded group is flipped. A three-overload run `A, B, C` becomes `C, B, A`.

`Interface::parse` (`interface.rs`) collects the pure-virtual methods in declaration
order, then walks the list and reverses each maximal run of consecutive methods sharing
a name — reproducing the MSVC layout so slot *N* in the metadata is slot *N* in the real
vtable. Downstream, `windows-bindgen` disambiguates the overloads by emission order, so
after the reversal the animation overload keeps the base name `SetOffsetX` and the scalar
overload becomes `SetOffsetX2` — matching both the true ABI and the pre-in-house
`windows` crate.

This is ABI-critical, not cosmetic: before the fix the `SetOffsetX`/`SetOffsetX2` slots
were swapped, so calling the scalar setter dispatched through the animation slot and
tripped `/GS` stack-cookie fail-fast (`0xC0000409`, `STATUS_STACK_BUFFER_OVERRUN`) — see
the `windows_dcomp` sample. The corpus blast radius is tiny (overloaded pure-virtual COM
methods are rare — DirectComposition, a few Direct2D SVG and DirectWrite interfaces).
Covered by `crates/tests/libs/clang/input/interface_overload.h`.

### Base-interface `_COM_Outptr_` creators are not auto-promoted (rejected heuristic)

A creator like `DWriteCreateFactory(_In_ REFIID iid, _COM_Outptr_ IUnknown **factory)`
(`um/dwrite.h`) is a caller-chosen-type factory whose out-parameter *should* have been
declared `void**` with `[iid_is(iid)]` — the header spells the pointee as the base interface
`IUnknown**` (and elsewhere `IInspectable**`) instead. This is a source bug in the SDK header
that cannot be fixed upstream now, so the faithful scrape keeps `factory: *mut IUnknown` and
bindgen projects `-> Result<IUnknown>`; the caller passes `&IDWriteFactory2::IID` and
`.cast()`s the result. (`IWeakReference::Resolve` looks identical but the *MIDL-generated*
`winrt/WeakReference.h` carries an explicit `[iid_is][out]` comment, so it is already promoted
via the `[iid_is]`-comment path — the difference is the annotation, not the pointee type.)

Auto-promoting the SAL-only form (`com_out_ptr_token` on an `IUnknown**`/`IInspectable**`
pointee with a sibling `REFIID`) was investigated and **rejected**: the signal is not
unambiguous, and an ambiguous promotion would be a lossy transform of the metadata, which is
not acceptable. A corpus scan of `um`/`shared` shows the base-interface `_COM_Outptr_` shape is
dominated by genuine fixed-type returns with **no** `REFIID` sibling (`SHGetThreadRef`,
`GetProcessReference`, `InstantiateComponentFromPackage`, the D3D11 interop
`CreateDirect3D11Device*` accessors, `IDWriteTextLayout::GetDrawingEffect`, …) that must stay
typed; the `REFIID`-sibling subset that *would* be promoted is a mix of last-parameter cases
(`DWriteCreateFactory`) and mid-list cases (`DbgModel` concept accessors,
`IDCompositionSurface::BeginDraw`) where the `REFIID`/out-pointer pair is not even adjacent to
the end of the signature — so no positional rule cleanly separates true creators from
coincidental `REFIID`+`IUnknown**` pairings without risking a wrong (lossy) promotion. Only a
literal `void**` pointee (`is_void_double_ptr`) or an explicit MIDL `[iid_is]` promotes to
`ComOutPtr`; everything else stays faithfully typed and is handled with a `.cast()` at the call
site.

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
| 13 | **Array *parameters* decay to pointers (C11 §6.7.6.3p7)** | An array *parameter* (`to_type` maps both `T[]` and `T[N]` to `ArrayFixed`) decays to a pointer when it is **unsized** (a flexible array `T[]` → `[T; 0]`, which is a zero-byte by-value drop) **or** carries a SAL count (`_Out_writes_(80) WCHAR szName[80]`): the `NativeArrayInfo` (`#[len_const]`/`#[len_param]`) already encodes the length, so keeping the `[T; N]` *type* too would double-encode it and make bindgen wrap `&mut [[T; N]; N]`. Pointee const-ness follows the array element's C const-ness; SAL (#4) may override. (Caveat: for an **unsized** `const T[]` parameter clang strips the element `const` during array-to-pointer adjustment before we see it, so a bare `const T[]` with no SAL direction decays to `*mut T` rather than `*const T` — ABI-identical and vanishingly rare in the corpus, so left as-is.) | A *plain* fixed-size array parameter with **no** count (`FLOAT ColorRGBA[4]`) stays `[T; N]`, so bindgen projects a length-checked `&[T; N]`. Struct fields, returns and constants keep their array shape. | `canon.rs` (`decay_array_param`), from `param_metadata_type` | An `ArrayFixed` is faithful for a *struct field* but wrong for a *parameter* — a by-value array is not a real ABI. The reference (`_Out_writes_(80) WCHAR szName[80]` → `*mut u16` + count → `&mut [u16; 80]`; `FLOAT BlendFactor[4]` + count → `Option<&[f32; 4]>`) decays counted buffers to pointer+count and keeps uncounted fixed arrays; this rule reproduces both exactly. |
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
  flat `<Id>.<Version>` layout), fetching the pinned nupkg from nuget.org on a miss.
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

- **`PROPERTYKEY`/`DEVPROPKEY` struct constants — now scraped (was a deferred feature).**
  `PKEY_*`/`DEVPKEY_*` (~2,672 of them — win32metadata#2090/#2100/#1773) are struct-*valued*
  constants written with `DEFINE_PROPERTYKEY` / `DEFINE_DEVPROPKEY`, both of which expand to
  the identical shape `{ { fmtid-GUID }, pid }`. The scraper matches these macro expansions
  the same way it matches `DEFINE_GUID`, and — rather than reviving win32metadata's brittle
  `ConstantAttribute` C-initializer *string* (which bindgen parsed by hand) — represents each
  one with the two structured primitives that already round-trip faithfully: a `GuidAttribute`
  (the `fmtid`) plus an ordinary ECMA `Constant` of type `u32` (the `pid`). The RDL spelling is
  `#[guid(0x540b947e_…)] const DEVPKEY_Device_BiosDeviceName: DEVPROPKEY = 10;`. bindgen
  reconstructs the struct literal from those two pieces, resolving the field types through any
  native-typedef chain (`DEVPROPKEY.fmtid` is `DEVPROPGUID = GUID`, `DEVPROPKEY.pid` is
  `DEVPROPID = u32`) so the `fmtid` GUID and the newtype-wrapped `pid` land in the right fields
  (`DEVPROPID(10)` in `windows`, bare `10` in `windows-sys`). Only these two struct shapes use
  the path, so no general struct-initializer machinery is needed.
- **WinRT interop headers** (`windows.ui.interop.h`, … — win32metadata#2186):
  `GetWindowFromWindowId` takes a WinRT *projection* type (`ABI::Windows::UI::WindowId`),
  which cannot be scraped without dragging in the `ABI::Windows::*` surface the manifest
  deliberately excludes. A scope decision, not a bug.
- **Toolchain/out-of-SDK blocks:** X3DAudio (`x3daudio.h`, win32metadata#2045) pulls in
  `DirectXMath.h` → `cpuid.h`, a compiler-intrinsic header not on the scrape include
  path (same class blocks any DirectXMath-dependent header). DISM (`dismapi.h`) and
  WIMGAPI (`wimgapi.h`) ship in the Windows ADK, not the pinned SDK NuGet packages, so
  they are out of scope for an SDK-based scrape.
- **Flat-namespace collisions are lossy.** The single flat `Windows.Win32` namespace
  (USR-deduped) cannot hold two genuinely distinct entities that share a name. The
  reference winmd disambiguates such pairs by editorial sub-namespace; here, dedup keeps
  exactly one. Examples: `PID_SECURITY` (`MsiDefs.h` summary property `19` vs.
  `PropIdl.h` reserved PID `0x80000002` — dedup currently keeps the latter) and the
  `E_NOTFOUND` class of duplicated `HRESULT`/`#define` names. Inherent to the flat
  namespace, not a scraper bug; a fix would need a name-disambiguation policy.

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

**To investigate and resolve: `tool_win32` and `tool_wdk` still work quite differently.**
`tool_win32` is the robust, general driver; `tool_wdk` reads like a thin hack bolted on to
parse a few extra headers. They share `windows-clang`/`windows-rdl` but diverge in ways that
are confusing and risk drift — the two should be streamlined onto one common path that
configures headers and other parameters the same way. Concrete discrepancies observed
(all confirmed against the current tree):

- **Config mechanism.** `tool_win32` is driven by a declarative `win32.toml` manifest
  (headers, satellite headers, import libs, arches — see [tool_win32](#the-in-house-corpus-tool_win32));
  `tool_wdk` hardcodes its inputs as `const SOURCE_HEADERS`/`SCOPE` in `main.rs`
  (~200 lines vs. ~490). The planned shared `wdk.toml`/manifest is the obvious unifier.
- **Architecture coverage.** `tool_win32` scrapes **three** arches (x64 + aarch64 + i686)
  and arch-merges them so subset-of-arch symbols get `SupportedArchitecture` tags;
  `tool_wdk` scrapes **x64 only** (single `--target=x86_64-pc-windows-msvc`, no merge).
  Any WDK symbol that differs across arches is therefore silently x64-shaped.
- **Calling-convention emission.** `tool_win32` emits bare `extern fn` throughout (0
  explicit `extern "system"` across the whole `metadata/win32` corpus — `"system"` is the
  inferred default); `tool_wdk` emits **explicit** `extern "system" fn` for hundreds of
  functions (≈452 in `metadata/wdk`), mixed inconsistently with bare `extern fn` even
  within the same corpus. The two should agree on one representation (prefer the `tool_win32`
  "infer and omit the default convention" style) so the RDL is uniform and diffs are clean.

Resolving these means factoring the common scrape/emit/merge pipeline into one shared driver
that both tools parameterize identically, then reducing `tool_wdk` to just its manifest
(headers + scope + import lib) — eliminating the calling-convention and arch-coverage skew.

### IDL as the COM source of truth

Parse `.idl` (or `midl` output) directly for COM to recover the pointer-shape
attributes the headers don't express as SAL (`[unique]`/`[length_is]`/`[iid_is]`),
keeping the header path for flat C APIs.

### Downstream namespace map for the published `windows`/`windows-sys` crates

This realises the "optional downstream map over the flat namespace" promised in
[Partitioning: by defining header, not editorial namespace](#partitioning-by-defining-header-not-editorial-namespace).

**Problem.** The canonical Win32/WDK winmds are flat (`Windows.Win32`), which is correct
and serves every flat/minimal consumer. But `windows-bindgen --package` derives *both* the
file layout **and** the per-namespace Cargo features from namespaces, so a flat winmd yields
one giant module with no feature partitioning. The published `windows`/`windows-sys` crates
are therefore still frozen against the old win32metadata under
`crates/tools/package/reference/*.winmd`.

**Approach (in progress).** Keep the canonical winmd flat; synthesise a header-based
namespace partition **only** at packaging time. One namespace per defining header = per
`.rdl` stem (`wdm.rdl` → `Windows.Wdk.wdm` → feature `wdm`; `bcrypt.rdl` →
`Windows.Win32.bcrypt` → `bcrypt`). Mechanical, source-derived, no curated editorial
grouping to maintain. This is a large, deliberate breaking change to feature names, but the
in-house metadata has not shipped so no released consumer breaks. There is **no** preserved
`Win32_Foundation` special case — the partition is purely metadata-derived.

**Mechanism.**
- `windows-rdl::reader::item_names(file, "Windows.Win32")` already returns, per `.rdl` file, the
  names of every type, function, and constant it defines (a pure syntactic walk). This is the
  `name → stem` routing signal, covering the `Apis`-member split as well as types.
- A remap pass in `windows-metadata` reads the flat winmds and rewrites, driven by the
  `name → namespace` map: each `TypeDef`'s defining namespace, and every reference embedded in
  `extends`, field `Type`s, method `Signature`s, and interface impls. The single flat
  `Windows.Win32.Apis` container is **split** into one `Apis` per target namespace, routing each
  function/constant by name. References not in the map (`System.*`, metadata-attribute types)
  pass through unchanged.
- `tool_package` builds the map from the RDL corpus (one namespace per header `.rdl` file, plus a
  small curated allowlist of header-name prefixes — `FOLD_PREFIXES` — that fold obviously-related
  headers into a family namespace, e.g. `d2d1`/`d2d1_1`/`d2d1effects`/`d2dbasetypes` → `d2d`),
  emits throwaway namespaced winmds under `target/`, and runs `--package` against those plus the
  already-namespaced WinRT `Windows.winmd`.

**Remaining after the winmd remap works:** migrate the in-repo consumers (samples/tests) from
the old editorial feature names (`Win32_System_WinRT`, `Win32_Graphics_Direct2D`, …) to the
header-based names (`winnt`, `d2d`, …), repoint `tool_features` at the remapped
metadata, retire `crates/tools/package/reference/*.winmd` (and the reference-only clang
normalisations gated on it, per [Type remapping](#type-remapping--one-canon-surface)), and
address the metadata-shape differences the switch surfaces (see "Metadata-shape fallout").

**Status.** The remap is wired end to end for the **full corpus** and both published crates
compile. `tool_package` reads the flat `Windows.Win32`/`Windows.Wdk` winmds and routes every
type/function/constant to a `Windows.Win32.<header>` / `Windows.Wdk.<header>` namespace (one per
header, minus the curated fold families): **528** Win32 namespaces / 124813 items
and **6** Wdk namespaces / 6401 items. `cargo check -p windows-sys --all-features` and
`cargo check -p windows --all-features` both succeed. Arch-divergent copies survive the remap
byte-faithfully. `sys.txt`/`windows.txt` now include the whole flat corpus (`--in target/package`,
editorial exclusions dropped — see the summary of feature naming below); WinRT exclusions are
preserved and `Windows.Win32.Metadata` (attribute types) is excluded.

**Header folding.** By default each `.rdl` header becomes its own namespace/feature. On top of
that, `remap.rs::FOLD_PREFIXES` is a small **curated allowlist** of header-name prefixes that fold
obviously-related headers into one family namespace (`bits`, `d2d`, `d3d9`/`d3d10`/`d3d11`/`d3d12`,
`dwrite`, `dxgi`, `functiondiscovery`, `msxml`, `ro`, `rpc`, `winbio`, `windns`, `winusb`, `wlan`,
`ws2`, `wsd`, `xps`). This is deliberately an allowlist, **not** an automatic name heuristic: a
purely name-based rule (fold into the shortest existing header-stem prefix) mis-groups headers that
merely share a prefix — `msinkaut` (Ink) under `msi` (Installer), `playsoundapi` under `pla`,
`icmpapi` (ICMP) under `icm` — and cannot be told apart from legitimate families like `sql`/`sqlext`
without curation. Each listed prefix was verified against the corpus to cover only genuinely related
headers; the longest matching prefix wins. Adding a header that starts with a listed prefix folds it
automatically, so the list should be revisited if an unrelated header ever collides.

**Feature-graph fix.** `package_writer.rs` previously hardcoded the old `Win32_Foundation`
"snowflake" as the base dependency of every top-level Win32/Wdk umbrella feature, and derived a
feature's parent dependency by `feature.rfind('_')`. Neither survives header namespaces:
`Win32_Foundation` no longer exists, and header stems can contain `_` (`bits1_5`, `dxgi1_2`,
`dpa_dsa`, …) so the rfind split produced non-existent parents (`Win32_bits1`). The parent
dependency is now derived from the namespace's **dot** structure (`Windows.Win32.bits1_5` →
`Win32`), the `Win32` and WinRT `Foundation` umbrellas are base features (no dependency), and
`Wdk` depends on the `Win32` umbrella.

**Metadata-shape fallout.** Unfreezing the crates from the old win32metadata means their output
now reflects the in-house flat metadata (PR #4649), which differs in shape from the old
editorial winmd in ways unrelated to namespaces. E.g. some typedef'd Win32 callbacks that the
editorial winmd emitted as plain `type X = Option<extern fn …>` aliases are emitted by the
in-house metadata as newtype structs (`pub struct PROPENUMPROCEX(pub PROPENUMPROCEXA)`) that
`#[derive(PartialEq)]`, tripping rustc's `unpredictable_function_pointer_comparisons` lint in
the full `windows` crate. These are pre-existing properties of the in-house corpus surfaced by
the switch, not remap artefacts, and must be resolved before CI's `-D warnings` passes.

**Bindgen fix surfaced by this work.** Validating a narrow filter (WDK-only, `--flat`) against
the remapped winmd exposed a latent dependency-gathering bug in `windows-bindgen`: when an
arch-divergent **delegate** or **constant** is pulled *purely as a dependency* (not directly
named by the filter), only one arch copy was collected, so the other arch's `cfg`-gated
definition went missing and references to it failed to compile. The multi-copy gather in
`Type::combine` (`crates/libs/bindgen/src/types/mod.rs`) only covered `CppStruct`/`CppFn`; it
now also covers `CppDelegate` and `CppConst` — the full set of arch-bearing (`.arches()`)
types. Directly-filtered types were never affected (both copies always emit), so full-corpus
generation (`tool_package`) shows no drift; only narrow/minimal filters could
hit it. Regression: `crates/tests/libs/bindgen/input/arch_delegate_dependency_sys.rdl`.

**To investigate: inconsistent handle modelling between `--package` and `--minimal`/`--sys`.**
Migrating a `--package` sample (`windows_direct2d`) to header namespaces surfaced a broken
partial-feature build. Two coupled issues were found; the second is now **fixed**, the first is
left open as a modelling question:
1. *(open)* `GLOBALHANDLE`/`LOCALHANDLE` are rightly plain type *aliases* in the RDL, but
   `--package` (default) mode nests them as newtype structs — and here a handle is nested inside
   *another* handle (`pub struct GLOBALHANDLE(pub super::winnt::HANDLE)`). Nesting is reasonable
   when the backing type is a primitive; nesting one named handle within another seems wrong.
   `--minimal`/`--sys` mode emits these as bare `pub type … = …` aliases instead. The default-mode
   modelling should probably be reconsidered and made consistent with minimal/sys mode. This is
   cosmetic/API-shape only — it does **not** block compilation now that (2) is fixed. A second
   manifestation showed up migrating `windows_task_dialog`: a header that pairs a scalar typedef with
   a separate constant enum (`typedef int TASKDIALOG_FLAGS;` + `enum _TASKDIALOG_FLAGS { TDF_… }`)
   yields a newtype `struct TASKDIALOG_FLAGS(pub i32)` for the field type while the `TDF_*` constants
   are bare `_TASKDIALOG_FLAGS` (`= i32`) integers, so combining flags forces the wrap
   `TASKDIALOG_FLAGS(TDF_A | TDF_B)`. Making scalar typedefs bare aliases in `--package` mode (to
   match `--minimal`/`--sys`) would remove the wrap here too; the JET_GRBIT-style constants noted in
   item 3 are the coupled blocker (their values are constructed as `TYPE(value)` tuple structs).

**TODO (deferred — harmonize the C flags/enum idiom in the canonical RDL).** The cleanest fix for
the `TASKDIALOG_FLAGS` wart above is at the *scraper*, not per-style in bindgen: recognize the C
idiom `enum _FOO { … }; typedef int FOO;` (enum tag = `_` + the sibling integer typedef name) and
emit a **single unscoped enum `FOO`** — rename the enum dropping the leading `_`, drop the redundant
typedef. The existing unscoped-enum rule then projects it as `pub type FOO = <int>;` + bare
`pub const MEMBER: FOO = …;` in *every* style (sys/minimal/package), so `dwFlags = TDF_A | TDF_B`
just works and the field type still reads `FOO`. This is what the retired editorial win32metadata
did. Scope measured across `metadata/win32|wdk/*.rdl`: **41 pairs**, all integer (`i32`/`u16`/`u32`),
concentrated in common shell/media headers — `shobjidl_core`/`shobjidl`/`shtypes` (28, incl.
`FILEOPENDIALOGOPTIONS`, `SHCONTF`, `SHGDNF`, `SIIGBF`, `SVGIO`, `SVSIF`, `KF_*`, `TRANSFER_*`),
`commctrl` (`TASKDIALOG_FLAGS`, `TASKDIALOG_COMMON_BUTTON_FLAGS`), `mfplay` (3), `ntifs` (2),
`propsys` (1) — so it is a recurring idiom, not a one-off. Implementation sketch: a pre-pass (sibling
to `build_tag_rename_map` in `lib.rs`) builds an `enum_rename` map `_FOO → FOO` whenever an integer
`typedef FOO` has a matching `enum _FOO`; `Enum::parse` applies the rename *before* the
`flag_enums`/repr check (so `DEFINE_ENUM_FLAG_OPERATORS(FOO)`, keyed on the public name, still
promotes to the unsigned repr and matches the old `u32` typedefs), and `Typedef::parse` skips the
now-redundant typedef. Must NOT touch: typedefs with no matching `_FOO` enum (`HWND`, `WPARAM`,
`DXGI_USAGE`), or `_`-prefixed enums with no matching typedef. Regenerating rescrapes ~41 `*.rdl`
files (`_FOO → FOO`, typedef dropped) and the downstream `windows`/`windows-sys` output; samples then
drop the `TASKDIALOG_FLAGS(…)` / `FILEOPENDIALOGOPTIONS(…)` wraps. Deferred until sample migration is
further along; pick up only if it stays low-complexity.
2. *(fixed)* `write_cpp_handle` (`crates/libs/bindgen/src/types/cpp_handle.rs`) emitted only
   `#arches` and never a per-item `#[cfg(feature = …)]`, so a handle referencing a type from
   another header namespace was not gated on that header's feature — under a partial feature set
   the referenced module was configured out and the handle failed to compile (`--all-features`
   masked it). Same arch/cfg class as PR #4687. Editorial metadata never hit this because all
   handles lived in the always-on `Foundation` namespace. Fix: both callers (`cpp_struct.rs`
   handle path, `cpp_enum.rs` bare-typedef path) now compute the handle's dependency `Cfg` and
   pass it into `write_cpp_handle`, which gates every emitted item (the alias/struct and all
   `impl`s) on both `#arches` and the feature `cfg`, exactly like `write_simple_cfg` does for
   regular structs. This took `windows_direct2d` from 91 `windows`-library errors to 0 — the
   library now compiles under a partial feature set; remaining errors are in the sample's own
   code (incomplete migration + metadata-shape flag/callback ergonomics), not the generated lib.
3. *(fixed)* Callback (delegate) typedefs were newtype-wrapped in `--package` mode and derived
   `PartialEq`, so a struct wrapping a function pointer triggered
   `unpredictable_function_pointer_comparisons` (CI `-D warnings`). Root cause: `is_primitive`
   treats `CppDelegate` as primitive, so a typedef whose target is a callback satisfies
   `CppStruct::is_handle` and reached `write_cpp_handle`'s newtype path. `write_cpp_handle` now
   emits a bare `pub type … = …` alias (matching `--minimal`/`--sys`) when the underlying type
   resolves — directly or through a chain of typedef handles
   (`FONTENUMPROC = FONTENUMPROCA = OLDFONTENUMPROCA = Option<fn…>`) — to a `CppDelegate`
   (`resolves_to_delegate`). Handle-of-handle typedefs (item 1) deliberately stay newtypes: unlike
   callbacks, their constants are constructed as tuple structs elsewhere in the generated code, so
   aliasing them breaks `pub const FOO: JET_GRBIT = JET_GRBIT(…)`. `windows --all-features` is now
   warning-free.

**Win32 extensions removed.** The hand-written `crates/libs/windows/src/extensions/` tree
(`VARIANT_BOOL`, WinSock address helpers, `VARIANT`/`StructuredStorage`, and the WinRT
`DateTime`↔`FILETIME` bridge) was coupled to the editorial namespace layout — cfg-gated on retired
features (`Win32_Foundation`, `Win32_Networking`, `Win32_System`) and using editorial type paths
(`crate::Win32::Foundation::FILETIME`). Under the header-based remap those features/paths no longer
exist, so the modules were permanently dead and emitted `unexpected cfg condition value` warnings.
As the in-house metadata is unreleased, the whole `extensions` module (and its `mod extensions;` in
`lib.rs`) was scrapped rather than migrated; equivalents can be reintroduced against the new layout
if wanted.

**Unscoped enums projected as bare integer aliases (all styles).** An unscoped (C-style) enum
has no metadata marking it a distinct type — logically it is just a set of global integer
constants, and the original C API takes the underlying integer (e.g. `D3D11CreateDevice` takes a
`u32`, not a `D3D11_CREATE_DEVICE_FLAG`). Previously `--package`/default mode wrapped these in a
newtype struct (`pub struct D3D11_CREATE_DEVICE_FLAG(pub i32)` with `pub const … = FLAG(2)`),
which forced call sites into `CONST.0 as u32` gymnastics and diverged from `--minimal`/`--sys`
(which already emit bare aliases). Now **every** style emits an unscoped enum as
`pub type X = <underlying>;` plus bare `pub const V: X = 2;` constants — matching the flat/minimal
projection and the way the API is actually consumed. Implementation: `CppEnum::write`
(`crates/libs/bindgen/src/types/cpp_enum.rs`) routes all `!ScopedEnumAttribute` enums to
`write_cpp_handle`, which now treats an enum `def` as always-bare (an enum is only ever passed
there for the unscoped case); `cpp_const.rs` emits bare integer values for unscoped-enum constants
in all styles. **Scoped enums** (`ScopedEnumAttribute` — WinRT enums and C++ `enum class`) remain
genuine newtype structs with associated constants. Unscoped `FlagsAttribute` enums also become
bare integer aliases and no longer emit `BitOr`/`BitAnd`/`Not`/`contains` impls — the underlying
integer supports `|`/`&`/`!`/`|=`/`&=` natively. Regenerated goldens: `enum_default`, `enum_flags`,
`derive_enum`, `derive_multiple`, `enum_name_conflict`, `modules` in
`crates/tests/libs/bindgen/expected/`.

**Flat Win32/WDK feature names (no `Win32_`/`Wdk_` prefix).** Because every Win32/WDK feature
derives from a globally unique source-header stem, the `Win32_`/`Wdk_` preamble carried no
disambiguating information — there is no `Win32_bcrypt` vs `Wdk_bcrypt` collision to resolve, so the
bare stem is unambiguous. Feature names are therefore just the header stem: `Windows.Win32.bcrypt`
→ feature `bcrypt`, `Windows.Wdk.wdm` → `wdm` (was `Win32_bcrypt` / `Wdk_wdm`). The `Win32`/`Wdk`
umbrella features and the hierarchical WinRT namespaces are unchanged: WinRT keeps its full dotted
path joined with `_` (`Windows.Foundation.Collections` → `Foundation_Collections`), and each header
still depends on its umbrella (`bcrypt = ["Win32"]`, `wdm = ["Wdk"]`, `Wdk = ["Win32"]`). Verified
collision-free: no Win32 stem equals a Wdk stem, and none collides (even case-insensitively) with a
WinRT feature name — Win32/WDK stems are lowercase header names, WinRT features are PascalCase dotted
paths. Implementation: a single `namespace_feature(namespace)` helper
(`crates/libs/bindgen/src/lib.rs`) strips the `Windows.Win32.`/`Windows.Wdk.` prefix and returns the
stem, else falls back to the post-`Windows.` dotted join; `TypeTree::feature`, the per-item
cross-namespace `Cfg::write` gate, and the feature-graph dependency derivation in
`package_writer.rs` all route through it so module gates, item `cfg`s, and Cargo features stay
consistent.

**Sample ergonomics: unannotated `_COM_Outptr_` factory functions.** Faithful metadata surfaces a
minor call-site wart in `windows_direct2d`: `D2D1CreateFactory` (d2d1.h) annotates neither its
`riid`/`ppvObject` out-parameters (`_COM_Outptr_`) nor links them (`#[iid_is]`), so bindgen cannot
generate the usual generic `D2D1CreateFactory::<ID2D1Factory1>(..)` convenience wrapper and the
sample calls the raw 4-arg form (`&ID2D1Factory1::IID`, `&mut factory as *mut _ as *mut *mut c_void`).
This is faithful to the header, not a projection defect; the sample now carries a short comment at the
call site explaining why. No fix is planned unless the annotations are added upstream.

**Fixed: use-after-free on window teardown (`windows-window`).** `wndproc`
(`crates/libs/window/src/window.rs`) took the message/resize handlers out of the boxed `State`
before invoking them (so reentrant dispatch sees empty slots) and restored them afterwards. But a
handler can *synchronously destroy the window* — e.g. letting `DefWindowProc` handle `WM_CLOSE`
calls `DestroyWindow`, which sends a reentrant `WM_NCDESTROY` that frees the `State` box in a nested
`wndproc` frame. The outer frame then wrote the handlers back into freed memory
(`(*state).message = …`), corrupting the heap and later faulting (access violation) while dropping
the closure box. Fix: after invoking the handlers, re-read `GWLP_USERDATA`; if it is now null the
window was destroyed during the handler, so skip the restore (the taken handlers drop locally,
which is correct) and skip the `WM_NCDESTROY` free. Verified with `windows_direct2d`: the debug
build previously terminated with an access violation on exit and now exits cleanly.

### Sample migration status and recipe

Porting an in-repo sample from the old editorial feature names to the header-based namespaces is
mechanical. The mapping mechanism: **a feature/module is a source-header stem**, so the module a
symbol lives in is found by grepping the generated crate
(`crates/libs/windows/src/Windows/Win32/<stem>/mod.rs`) for the symbol — the directory name *is*
the feature and the `use windows::Win32::<stem>::*` path. Enable those stems in `Cargo.toml`
`features`, rewrite the `use` block to match, then let the compiler surface the rest. (The
`crates/samples/*/*` glob has been restored in the workspace `members` now that all samples build;
`crates/samples/windows/spellchecker` stays in `exclude` — see follow-up below.)

Migrated: `windows_direct2d`, `windows_dcomp`, `windows_direct3d12`, `windows_ocr`,
`windows_overlapped`, `windows_core_app`, `windows_uiautomation`, and the `windows_samples`
aggregator (plus the pre-existing `task_dialog`/`file_dialogs`). The faithful in-house metadata
differs in shape from the old
editorial winmd in ways unrelated to namespaces, so each port hits a recurring set of call-site
edits (all correct-by-design, not projection defects):

- **Unscoped C enums are bare integer aliases** (see "Unscoped enums projected as bare integer
  aliases" above). Tuple constructions and `.0` accesses must be dropped: `DXGI_PRESENT(0)` → `0`,
  `DXGI_CREATE_FACTORY_FLAGS(0)` → `0`, `DXGI_ADAPTER_FLAG(x) & …` → `x as i32 & …`,
  `D3D12_COLOR_WRITE_ENABLE_ALL.0 as u8` → `D3D12_COLOR_WRITE_ENABLE_ALL as u8`. Methods whose
  parameter was such an enum now take the bare integer (`IDXGISwapChain::Present(_, flags: u32)`).
- **`NativeTypedef` scalars stay newtypes** (e.g. `DXGI_USAGE(pub u32)`) while their `#define`
  constants are bare integers, so a struct field of that type needs the wrap
  (`BufferUsage: DXGI_USAGE(DXGI_USAGE_RENDER_TARGET_OUTPUT)`). Same for genuinely distinct enum
  newtypes like `D3D12_PRIMITIVE_TOPOLOGY(pub D3D_PRIMITIVE_TOPOLOGY)`.
- **No fabricated `Result` / retval ergonomics.** Win32 functions the editorial winmd hand-patched
  are faithful here: `IDXGIAdapter1::GetDesc1` takes a `*mut DXGI_ADAPTER_DESC1` out-param and
  returns `HRESULT` (call `.ok()?` on a stack `default()`), `AdjustWindowRect` returns `BOOL`
  (`.ok()?`), `CreateEventA` returns a raw `HANDLE` (check `is_invalid()` + `Error::from_thread()`),
  and out-`IBlob` parameters are raw `*mut Option<ID3D10Blob>` (pass `&mut opt` /
  `std::ptr::null_mut()` for the unwanted error blob) rather than `Option<&mut>`.
- **Float `#define` macros are not scraped** (a known coverage gap): `D3D12_MIN_DEPTH`/
  `D3D12_MAX_DEPTH` are absent, so the sample uses the literal `0.0`/`1.0`.
- **Optional-array parameters are faithful slices**, not `Option`: `ClearRenderTargetView`'s
  `prects` is `&[D3D12_RECT]` — pass `&[]` where the old binding took `None`.

Also migrated: `windows_ocr` (WinRT-only — no changes needed, see below), `windows_overlapped`.
Additional drift patterns surfaced by these:

- **WinRT-only samples need no source edits.** WinRT features keep their full dotted path joined
  with `_` (`Media_Ocr`, `Graphics_Imaging`, `Storage_Streams`, `ApplicationModel_Core`,
  `UI_UIAutomation`); only the Win32 editorial features were removed. `windows_ocr` built unchanged
  once added to the workspace `members`.
- **Handle typedefs carry no `RAIIFree`, so `Owned<HANDLE>` does not compile.** The faithful
  in-house metadata scraped by `windows-clang` does not emit the `RAIIFreeAttribute` the editorial
  winmd hand-added, so no `windows_core::Free` impl is generated for `HANDLE` and friends.
  `windows_overlapped` therefore keeps the raw `HANDLE` and calls `CloseHandle(...)` explicitly
  instead of relying on `Owned` RAII. (The now-dead `Free`/`RAIIFree` handling was **removed** from
  `windows-bindgen` in PR #4689 — see "Un-inferable curated attributes removed" in Follow-up — so this
  is a settled scope decision, not a pending metadata enhancement.)
- **Win32 error constants are bare `u32`, not `WIN32_ERROR`.** `ERROR_IO_PENDING` etc. are plain
  integers with no `Into<HRESULT>`; wrap for comparison against `Error::code()`:
  `WIN32_ERROR(ERROR_IO_PENDING).into()`.
- **`ReadFile`/`WriteFile` take an explicit buffer pointer + length**, not a slice: the buffer is
  `Option<*mut c_void>` with a separate `nNumberOfBytesToRead: u32` — pass
  `Some(buffer.as_mut_ptr() as *mut c_void)` and `buffer.len() as u32`, and `.ok()` the returned
  `BOOL`. Access-flag constants (`FILE_GENERIC_READ`, `FILE_SHARE_READ`, `OPEN_EXISTING`,
  `FILE_FLAG_OVERLAPPED`) are bare `u32` (drop any `.0`).

Also migrated: `windows_core_app`, `windows_uiautomation` (both mixed WinRT + Win32). The WinRT
features (`ApplicationModel_Core`, `UI_Core`, `UI_UIAutomation`) are unchanged; only the Win32
editorial features are remapped (`System_Com` → `combaseapi`/`objbase`, `UI_WindowsAndMessaging` →
`winuser`, `UI_Accessibility` → `uiautomationclient`). Further drift:

- **`CoInitializeEx` takes a bare `u32`; `COINIT_*` are bare `i32`.** `COINIT` is an unscoped C enum,
  so `CoInitializeEx(None, COINIT_MULTITHREADED as u32)`.
- **User32/Com functions referencing `windef::HWND` are cfg-gated on the `windef` feature** even
  when `HWND` is not named directly (`MessageBoxW`, `FindWindowA`, …). Symptom is "cannot find
  function X in this scope"; add `windef` to `features` (same transitive-feature pattern as
  direct3d12's `minwinbase`).
- **`FindWindowA` returns a raw `HWND`** (no `Result`): drop `?` and check `is_invalid()`.
- **`UIA_HWND` is a distinct native-typedef newtype**, not `HWND`, and there is no automatic `Param`
  conversion: `automation.ElementFromHandle(UIA_HWND(window.0))`.

Also migrated: the `windows_samples` aggregator (21 `examples/`, one shared feature set). WinRT-only
or `windows-window`-only examples (`simple`, `xml`, `rss`, `device_watcher`, `create_window`) needed
no source edits; the Win32/COM examples hit the patterns above plus these:

- **`CoInitializeSecurity`'s `asAuthSvc` is a faithful slice** `Option<&[SOLE_AUTHENTICATION_SERVICE]>`,
  which encodes the old `cAuthSvc` count. Pass `None` (drop the explicit `-1`); `None` correctly
  means "count `-1`, null array".
- **`VARIANT` has no fabricated ergonomics.** There is no `From<&str>`, `Display`, or `Drop` — the
  editorial winmd hand-added those. To pass a string `VARIANT` you build the union by hand
  (`vt: VARTYPE(VT_BSTR as u16)`, `bstrVal: ManuallyDrop::new(BSTR::from(s))`), read it by matching
  on `vt`, and free it with `VariantClear`. `VT_*` are bare `i32` (cast to `u16` for `VARTYPE`);
  `EOAC_*` are bare `i32` (cast to `u32`).
- **WMI (`IWbemServices`/`IEnumWbemClassObject`) methods are faithful COM.** `Next` takes an explicit
  count + `*mut Option<_>` array pointer (not a slice + retval); the optional `[out]` params of
  `Get`/`GetObject`/`ExecMethod` are raw `*mut` (pass `std::ptr::null_mut()` for the ones you skip,
  and `&mut out` — not `Some(&mut out)` — for the ones you want). `Put` takes `CIMTYPE(CIM_STRING)`.
- **`IServiceProvider::QueryService` is raw** (no generic retval helper).

Also migrated: the remaining umbrella-`windows` samples `lang_perf/component` and `robot/component`
(WinRT activation-factory components → `activation` + `winerror` stems for `IActivationFactory` /
`CLASS_E_CLASSNOTAVAILABLE` / `E_BOUNDS`), and `reactor/swap_chain_panel`
(`d3d11`/`d3dcommon`/`dxgi`/`minwindef`).

**`windows-sys` samples too.** `windows-sys` is regenerated by the same `tool_package` pipeline into
the identical header-stem layout, so its samples migrate the same way. Migrated
`windows_sys_task_dialog` (`commctrl`/`minwindef`/`windef`) and `windows_sys_samples`
(`create_window`/`message_box`/`service`). windows-sys drift is simpler (no COM ergonomics) but hits
the transitive-feature and bare-alias patterns:

- **Functions are cfg-gated on every referenced type's stem.** `CreateFileW`/`WriteFile` reference
  `super::minwinbase::SECURITY_ATTRIBUTES`/`OVERLAPPED` and `super::winnt::HANDLE`, so `fileapi`
  callers must also enable `minwinbase` + `winnt`. `CloseHandle` lives in `handleapi`.
- **State/flag typedefs collapse to `u32`.** `SERVICE_STATUS.dwCurrentState` is now a plain `u32`
  (the `SERVICE_STATUS_CURRENT_STATE` newtype the editorial winmd carried is gone), so helper
  signatures using it become `u32`. `SERVICE_*` state/accept constants are in `winsvc`; the
  `SERVICE_WIN32_OWN_PROCESS` / `FILE_*` access + attribute constants are in `winnt`.
- **`windows-sys` pointer params are untyped `*const c_void`.** `WriteFile`'s buffer is
  `*const core::ffi::c_void`, so a `*const u8` must be cast (`message.as_ptr() as *const _`).

**Bindgen-driven samples share the drift too.** `reactor/direct2d` (and `spellchecker`) generate
their own bindings via a `windows-bindgen` `build.rs` rather than depending on the umbrella crate,
but they read the same in-house metadata, so they hit the bare-alias pattern:
`D3D11_CREATE_DEVICE_BGRA_SUPPORT.0 as u32` → `D3D11_CREATE_DEVICE_BGRA_SUPPORT as u32`.

### Test migration status and recipe

The test crates under `crates/tests/*/*` were removed from the workspace `members` during the
sample migration (they still referenced the old editorial features). Migrating them uses the same
header-stem recipe as the samples, plus a few test-specific gotchas. Because cargo resolves the
whole workspace graph up front, an unported member with a now-nonexistent feature fails resolution
for *every* `-p` build — so test crates are re-added to `members` one batch at a time as they are
ported (keeping the list always resolvable), and the `crates/tests/*/*` glob is restored only once
all are green.

**Status: near-complete.** 41 test crates ported and back in the `crates/tests/*/*` glob; 3 remain in
`exclude` (see BLOCKED below — one out-of-scope, one deferred-feature, one large mechanical port).
Six crates that only exercised deliberately-dropped surface were
**deleted**: the `test_extensions`/`test_variant` extension crates, plus the four curated-attribute
crates `test_agile`/`test_alternate_success_code`/`test_handles`/`test_return_handle` (see
"Un-inferable curated attributes removed" below). `cargo test --workspace
--exclude test_no_std --no-run` and `cargo clippy --workspace --all-targets --exclude test_no_std`
are both green (`test_no_std` is excluded exactly as CI does — `--all-targets` feature-unification
pulls `std` and duplicates its `panic_impl`; `cargo check -p test_no_std` stays clean).

Ported (40): `libs/{core, collections, future, interface, reactor_perf, registry, result, services,
strings, sys}`; `misc/{arch, array, bcrypt, calling_convention, error, interop, marshal, msrv,
no_std, not_dll, query_signature, readme, reserved, return_struct, string_param, structs, unions,
weak_ref, win32, win32_arrays}`; `winrt/{activation, activation_client, composable, composable_client,
constructors, constructors_client, event_core, old, overloads, overloads_client}`.
(`collections`/`future` just dropped an unused `Win32_Foundation`; the rest remapped to header stems
— e.g. `result` → `bcrypt`/`ntstatus`/`winerror` with NTSTATUS wraps, `sys` →
`bcrypt`/`handleapi`/`minwinbase`/`minwindef`/`ntstatus`/`synchapi`/`windef`/`winerror`/`winnt`/
`winuser`; `win32` → the flat DXGI/D3D/COM/UI stems, with `winsock.rs` inlining the removed std↔net
conversions as example helpers).

BLOCKED (2, at the remaining metadata edges — see Follow-up): `libs/implement`; `misc/lib`.
The precise gaps:
- `implement` — needs the WinRT interop interface `IDisplayPathInterop`. **Now scraped** by adding
  `Windows.Devices.Display.Core.Interop.h` to `tool_win32` (a clean `IUnknown`-derived COM interface,
  no `ABI::Windows::*` projection deps, unlike the `windows.ui.interop.h` scope-block above). The crate
  still needs its editorial `Win32_*` features/paths ported to header stems (`Win32::System::Com`,
  `::WinRT`, `::Ole`, `::Foundation`, `::UI::Shell`, …) — a large but mechanical port.
- `lib` — the `clr` test needs `GetFileVersion` from `mscoree.h`, which ships **only in the NETFXSDK**
  (`Windows Kits\NETFXSDK\4.8\…`), not the pinned Windows SDK NuGet — the same hermetic-build block as
  `cor.h`/`RoMetadataApi.h` above, so it is genuinely **out of scope**, not a scrapable gap. The
  `browser` test needs `IECreateFile` from `IEPMapi.h` (a clean header, but deprecated Internet
  Explorer Protected-Mode API — low value).

Newly un-blocked this pass: `misc/arch_feature` (scraped `ntenclv.h` for `VBS_BASIC_ENCLAVE_*`; `CONTEXT`
already present in `winnt`). The `VBS_BASIC_ENCLAVE_BASIC_CALL_CREATE_THREAD` callback now faithfully
takes the `PVBS_BASIC_ENCLAVE_THREAD_DESCRIPTOR` typedef newtype (windows) / raw pointer (windows-sys),
so the test uses that signature rather than a decayed `*const …DESCRIPTOR64`. `misc/structs` is
also un-blocked this pass: `propertykey` by the `PROPERTYKEY`/`DEVPROPKEY` constant scrape above
(`DEVPKEY_Device_BiosDeviceName.pid` is now the faithful `DEVPROPID(10)` newtype, not a bare `u32`),
`bstr` by scraping `DbgProp.h` (`DebugPropertyInfo`), and `winrt` needs no metadata (WinRT
`Storage_Search::SortEntry`).

Header-stem note: a header whose own file name is dotted (`Windows.Devices.Display.Core.Interop.h`)
must collapse to a *single* flat partition (`windowsdevicesdisplaycoreinterop`), not a nested
`Windows::Devices::…` module tree. `header_stem_to_namespace` strips the leftover dots from the stem;
regression: `crates/tests/libs/clang/tests/header_partition.rs::dotted_header_flattens_to_single_partition`.

(`libs/targets` was un-blocked by the general macro-alias pass, which recovers `RtlGenRandom` (and the
`psapi` `K32*`/`EnumProcesses` family) — see Follow-up; `misc/wdk` by scraping `offreg.h` into
`tool_wdk`; `misc/const_params` and `misc/const_ptrs` by adding `pathcch.h`/`propvarutil.h` to
`tool_win32`; `misc/resources` by the negative-`MAKEINTRESOURCE`/char-pointer-sentinel `const.rs`
arms — see Follow-up.)

Test-specific drift beyond the sample patterns:

- **Foundation/error ergonomics come from `windows::core`, not a metadata stem.** `WIN32_ERROR`
  (with `to_hresult()`, `From<WIN32_ERROR> for Error/HRESULT`) is the `windows-result` type
  re-exported through `windows::core` — *not* the bare `winreg::WIN32_ERROR(pub u32)` newtype the
  flat metadata emits. Source `WIN32_ERROR` from `core::*` and do **not** enable/glob `winreg` for
  it. (Two same-named types → glob ambiguity if you import both.)
- **`GetLastError`/`SetLastError` are bare `u32`.** `GetLastError() -> u32` (was `WIN32_ERROR`) and
  `SetLastError(u32)`; wrap with `WIN32_ERROR(GetLastError()).to_hresult()` and drop the wrap on
  `SetLastError(code)`.
- **Error constants need re-wrapping for `Error::from`.** `ERROR_*` are bare `u32`
  (`Error::from(WIN32_ERROR(ERROR_NO_UNICODE_TRANSLATION))`); `*_E_*` HRESULT constants are bare
  `i32` (`Error::from(HRESULT(AUDCLNT_E_UNSUPPORTED_FORMAT))`).
- **Some header stems export a `pub const None`.** A C enum member named `None` (e.g.
  `Win32::ro`'s `RO_ERROR_REPORTING_FLAGS::None` → `pub const None`) shadows the prelude
  `Option::None` under a `use ...::ro::*` glob, breaking any bare `None` (cryptic "constant takes 0
  generic arguments" / `Param` errors). Import such modules' symbols explicitly instead of globbing.
- **Symbol→stem map for common `Foundation` members:** `E_*`/`ERROR_*`/`CRYPT_E_*` → `winerror`;
  `SetLastError`/`GetLastError` → `errhandlingapi`; `IErrorInfo` → `oaidl`;
  `CreateErrorInfo`/`SetErrorInfo` → `oleauto`; `RoSetErrorReportingFlags`/`RO_ERROR_REPORTING_*` →
  `ro`; `AUDCLNT_E_*` → `audioclient`.

### Follow-up work (not done in this branch)

- **The published `windows`/`windows-sys` crates no longer need an umbrella `Win32` module or
  feature — the projection is now completely flat.** In the old editorial layout the module tree was
  nested (`Win32::UI::Shell`, `Win32::System::Com`, …) and enabled via nested cargo features
  (`Win32_UI_Shell`). The in-house metadata is flat: every header stem is its own top-level module
  (`pathcch`, `urlmon`, `winerror`, …) gated by a same-named leaf feature (`pathcch = ["Win32"]`),
  and the intermediate `Win32 = []` feature and the `Win32/mod.rs` re-export shell now carry nothing
  of their own — they exist only as a vestigial grouping. **TODO:** drop the `Win32` umbrella feature
  and the surrounding module entirely (emit the header-stem modules at the crate root, or at least
  stop threading the empty `Win32` feature through every leaf's dependency list), simplifying every
  consumer's feature list from `["Win32", "pathcch", …]` down to just the leaf stems.
- **Missing WinRT `*Interop` bridge interfaces (metadata coverage gap).** The in-house metadata is
  missing some of the WinRT-to-Win32 interop interfaces the editorial winmd carried:
  `IUserConsentVerifierInterop`, `IWindowNative`, `IGraphicsCaptureItemInterop`, and
  `IDisplayPathInterop` are absent, while others (`IInitializeWithWindow`, `IMemoryBufferByteAccess`,
  `IBufferByteAccess`, `ICoreWindowInterop`) are present. As a stopgap the `consent` example was
  reworked to call `UserConsentVerifier::RequestVerificationAsync` (no HWND) instead of the interop
  `RequestVerificationForWindowAsync`, which changes what the sample demonstrates; the
  `test_implement` `com.rs` test (which `#[implement]`s `IDisplayPathInterop`) is BLOCKED and left
  unported. **TODO:** scrape the missing interop interfaces in `windows-clang` and restore the
  original interop-based `consent` sample + `test_implement`.
- **Macro-alias function scraping — done (`test_targets` unblocked).** A general, entirely heuristic
  pass in `windows-clang` recovers functions the SDK declares under a documented name that an
  object-like `#define` textually rewrites to a raw export before clang parses. The prototype is
  written (with its calling-convention token) under the friendly name — e.g.
  `RtlGenRandom(...)`, `EnumProcesses(...)`, `ClfsLsnCreate(...)`, `CM_WaitNoPendingInstallEvents(...)`
  — but `#define RtlGenRandom SystemFunction036` (and the `psapi` `K32*`, `clfs` `Clfs*`, `cfgmgr32`
  `CM_*` families) makes `cursor.name()` the *expanded* export symbol while the source tokens still
  carry the friendly spelling. The pass emits the function under the recovered friendly name and
  records the raw export as the P/Invoke import (link) symbol — a faithful mapping of the header to the
  documented/header-defined name, with the export preserved for the linker.

  **No curated list.** `Parser::alias_map` (`build_alias_map`, from `macro_defs`) inverts every
  object-like `#define <Alias> <Export>` whose body is a single C identifier, excluding the A/W charset
  selectors (`#define X XW`) so the `A`/`W` variants are never collapsed. The rename is applied per
  function only when the *source tokens* confirm this prototype was written with the alias
  (`token_names_function`): the alias token precedes the parameter-list `(` **and** the export name
  does not. This deliberately leaves back-compat aliases alone — where the real prototype is written
  under the export and the `#define` merely adds a legacy spelling (`#define VarBoolFromInt VarBoolFromI4`,
  literal prototype `VarBoolFromI4(...)`) — so `oleauto` et al. are untouched. Total fallout is 4 RDL
  files (`ntsecapi` ×3, `psapi` ×~28, `cfgmgr32` ×1, `clfs` ×10).

  **Also fixes a real calling-convention bug.** Before this, `detect_calling_convention` anchored on
  `cursor.name()` (the expanded export), failed to find it among the source tokens, and fell back to
  `extern "C"` — so every one of these `__stdcall`/`WINAPI`/`NTAPI` exports was recorded with the wrong
  ABI (a stack-corrupting mismatch on x86). Anchoring detection on the recovered source spelling flips
  them to the correct `"system"`. The `test_targets` `symbol.rs` acceptance test (which links
  `RtlGenRandom`) now passes on **both** x64 and i686 — the i686 run is the regression guard for the
  convention fix.

  **Divergence from the editorial winmd is intentional.** The old win32metadata winmd renamed *only*
  the `SystemFunction*` family (curated) and kept raw exports for `K32EnumProcesses`, `LsnCreate`,
  `CMP_WaitNoPendingInstallEvents`; this pass instead uniformly prefers the documented/header-defined
  name. The editorial names are hand-curated and not always reliable, so the mechanical mapping to the
  name the header actually writes the prototype under is treated as the more faithful projection. The
  raw export always survives as the `link!` import symbol, so linkage is unchanged.

  **Plumbing.** `Fn` gained an `import_name: Option<String>`; RDL's `#[library(...)]` attribute gained
  an optional `import = "<export>"` argument (parsed in `rdl/src/reader/fn.rs`, emitted by
  `rdl/src/writer/fn.rs` for the arch-merge round-trip), threaded into the winmd `ImplMap` `ImportName`.
  Downstream `windows-bindgen` already reads `impl_map.import_name()` and emits it as the `link!` entry
  point, so no bindgen change was needed. End-to-end the published crates emit e.g.
  `link!("advapi32.dll" "system" "SystemFunction036" fn RtlGenRandom(...))`.
- **offreg.h scraped into `tool_wdk` — done (`test_wdk` unblocked).** `test_wdk`'s `win.rs`/`sys.rs`
  use `ORCreateHive`, `ORCloseHive`, and the `ORHKEY` handle from `offreg.h`, which `tool_wdk`
  previously did not scrape (only `ntifs.h`+`wdm.h`), so the offline-registry surface was absent.
  `offreg.h` is now appended to `SOURCE_HEADERS` and `offreg.lib` (WDK `Lib/…/km/x64`) is fed to
  `import_library` so `drop_lib_less` keeps the routines (they export from `offreg.dll`). One wrinkle:
  `offreg.h` ships in the WDK `km` folder but is really a *user-mode* API and references the standard
  Win32 `um` typedefs (`DWORD`/`PDWORD`/`BYTE`/`PBYTE`/`PWSTR`/`PCWSTR`/`PFILETIME`/
  `PSECURITY_DESCRIPTOR`/`SECURITY_INFORMATION`) that the kernel-mode TU (`ntifs.h`+`wdm.h`) never
  brings in. A small force-included prelude (`crates/tools/wdk/src/offreg_prelude.h`) supplies just
  those aliases. Two lessons: (1) a `-include` prelude's declarations are *not* emitted (not the main
  input, not in `scope_headers`) **only** if its type names collide with the Win32 winmd exclusion
  reference — the FILETIME alias initially leaked as an empty `_FILETIME` struct because it was
  forward-declared under its *tag* name; giving it the full standard `typedef struct _FILETIME {…}
  FILETIME` definition made it resolve/exclude by the name Win32 uses (`FILETIME`). (2) `ORHKEY` comes
  through as a plain handle newtype `struct ORHKEY(pub *mut c_void)` (no `Default`) and the routines
  return bare `u32` (not `Result`), so the ported test constructs `ORHKEY(core::ptr::null_mut())` and
  asserts the return is `0`.
- **`pathcch.h` + `propvarutil.h` scraped into `tool_win32` — done (`test_const_params`,
  `test_const_ptrs` unblocked).** Both headers were simply absent from `win32.toml`'s `headers`
  list, so their exports (`PathCchFindExtension` / `WINPATHCCHAPI`, and the `PropVariantTo*` /
  `InitPropVariantFrom*` `PSSTDAPI` inline helpers) were never emitted. Adding `pathcch.h` (after
  `shlwapi.h`) and `propvarutil.h` (after `propsys.h`) surfaced them plus a small dependency header
  (`propapi.rdl`); only three *new* `metadata/win32/*.rdl` files appear, no existing rdl changes.
  Two lessons:
  1. **`parse_nested_cast` misread a function-like-macro invocation as a cast.** `pathcch.h` has
     `#define VOLUME_PREFIX_LEN (ARRAYSIZE(VOLUME_PREFIX)-1)`. `parse_nested_cast` (in `const.rs`,
     built for handle sentinels like `((HANDLE)(LONG_PTR)-1)`) ignored paren nesting and read
     `ARRAYSIZE(VOLUME_PREFIX)` as two cast-type identifiers, emitting a bogus
     `const VOLUME_PREFIX_LEN: ARRAYSIZE = -1` that then failed the arch-merge ("type not found").
     Fix: a cast identifier `(TYPE)` is *always* immediately followed by `)`; an identifier followed
     by `(` is a macro call → return `None` so the batch evaluator computes the real value (`10`).
     Regression fixture: `crates/tests/libs/clang/input/macro_invocation_const.h`.
  2. **Header-stem features do NOT auto-enable the stems they reference.** The generated per-header
     Cargo features are flat (`propvarutil = ["Win32"]`, `Win32 = []`); tool_package instead gates
     each *function* with an `all(feature = …)` `cfg` listing every stem its signature touches. So
     `InitPropVariantFromStringVector`/`PropVariantToBSTR` are gated on
     `minwindef, oaidl, objidl, objidlbase, propidlbase, wtypes, wtypesbase` — the consumer must
     enable *all* of them explicitly (a bare `propvarutil` feature silently compiles the module but
     omits the functions, showing up as an "unused import" + "cannot find function"). `test_const_ptrs`
     therefore lists all nine stems. API drift for `test_const_params`: `URI_CREATE_FLAGS` no longer
     exists (pass a bare `0` to `CreateUri`, whose `dwreserved` is now `Option<usize>`); modules moved
     to header stems (`PathCchFindExtension`→`pathcch`, `WindowsGetStringRawBuffer`→`winstring`,
     `CreateUri`/`CreateIUriBuilder`→`urlmon`, `S_OK`→`winerror`).
- **Negative `MAKEINTRESOURCE` ordinals + inline char-pointer sentinels scraped — done
  (`test_resources` unblocked).** Two `const.rs` gaps surfaced by `test_resources`:
  1. **`MAKEINTRESOURCEW(-N)`.** `commctrl.h`'s `TD_ERROR_ICON`/`TD_WARNING_ICON`/… are
     `MAKEINTRESOURCEW(-2)` etc. The existing `makeintresource_macro` arm only matched the positive
     `IDENT ( LITERAL )` shape; the negative form (`IDENT ( - LITERAL )`) fell through and was dropped.
     Added a mirror arm. **Critical subtlety:** `MAKEINTRESOURCEW(i)` is
     `((LPWSTR)((ULONG_PTR)((WORD)(i))))` — the `(WORD)` truncates to 16 bits *before* widening, so a
     negative arg becomes a zero-extended ordinal, **not** a sign-extended pointer. Storing the
     sign-extended value (`-2` = `0xFFFF…FFFE`) gives a non-zero high word, so `LoadIconW`/`FindResource`
     dereference it as a string pointer → **ACCESS VIOLATION** at runtime. The arm therefore emits
     `(raw as u16).wrapping_neg() as i32` (`TD_ERROR_ICON: PWSTR = 65534`); `0xFFFE as i16` is still `-2`,
     so `.0 as i16 == -2` holds while the high word stays zero (treated as an ordinal).
  2. **`((OLECHAR*)(INT_PTR)-1)`.** `objidl.h`'s `COLE_DEFAULT_PRINCIPAL` casts through an *inline*
     pointer-to-char (`OLECHAR*`), which `parse_nested_cast` rejects at the `*` (its cast chain only
     accepts bare typedef names). Added a fixed-token arm + `char_pointer_target` helper mapping
     `OLECHAR`/`WCHAR`→`PWSTR`, `CHAR`→`PSTR`, emitting the sign-extended sentinel
     (`COLE_DEFAULT_PRINCIPAL: PWSTR = -1`, projected `PCWSTR(-1 as _)`, so `.0 as usize == usize::MAX`).
     This is a *genuine* full-width pointer sentinel and must **not** be WORD-truncated like case 1.
  Regression fixtures: `const_makeintresource.h` (extended) and new `const_char_pointer.h`.
- **PR #4689 CI validation fixes (non-MSVC/msrv targets).** Two bugs broke the generated
  `windows-sys` on `gnu`/`gnullvm`/`i686`/`msrv` (they compiled on MSVC stable, so only the cross jobs
  caught them):
  1. **C-variadic calling convention.** A variadic function is always `__cdecl` on Windows — MSVC
     silently ignores a stated `__stdcall`/`WINAPI` for varargs — and rustc *rejects* an
     `extern "system"` C-variadic outright on non-MSVC targets (`E0658`/`E0045`). Several `WINAPI`
     variadics (`AuthzInitializeObjectAccessAuditEvent`, `AuthzReportSecurityEvent`, `setupapi`, Wdk
     `ntifs`) were being emitted as `extern "system"`. Fixed in **two layers**: `clang` (`fn.rs::write`)
     now forces `extern "C"` in the RDL whenever `is_variadic`, and `bindgen` (`cpp_fn.rs::abi`) forces
     `"C"` whenever the method signature carries `MethodCallAttributes::VARARG` — a belt-and-suspenders
     override so any winmd marking a variadic as `system` still emits correctly.
     (`method_def.rs::calling_convention` widened to `-> &'static str`.) Regression fixtures:
     `calling_convention.h` (`VariadicFunc`, clang) and new `variadic_fn_sys.rdl` (bindgen).
  2. **`*const u8: Default`.** `GOPHER_ABSTRACT_ATTRIBUTE_TYPE` (wininet) has an `LPCTSTR` (= `PCSTR`
     = `*const u8`) field yet derived `Default`; raw pointers have no `Default` impl (`E0277`). The
     existing `can_derive_default` caught direct `PtrConst`/`PtrMut`/`PCSTR`/… fields but not a
     **native-typedef alias** that *resolves* to a pointer. Added `CppStruct::resolves_to_pointer`
     (parallel to `resolves_to_fixed_array`); a sys-mode struct with such a field now falls back to the
     manual zeroed `Default` impl instead of deriving. Regression fixture: `struct_typedef_pointer_sys.rdl`.
     RDL churn: `authz.rdl` (3 fns) + `setupapi.rdl` (2 fns) flip `system`→`C`; no other change.
   3. **Pointer-sized `usize` constant overflows 32-bit targets.** A pointer sentinel such as
     `#define ITSAT_DEFAULT_LPARAM ((DWORD_PTR)-1)` is scraped (correctly) as `USize(0xFFFF_FFFF_FFFF_FFFF)`
     — the 64-bit two's-complement of `-1` — but `bindgen` emitted the bare decimal literal
     `pub const ITSAT_DEFAULT_LPARAM: usize = 18446744073709551615;`, which overflows a 32-bit `usize`
     (`E0080`) on `i686`. The reference winmd never carried any `usize`/`isize` integer constant, so the
     published crates had zero of these until the in-house pipeline added them (6 total: `ITSAT_DEFAULT_LPARAM`,
     `SSRVOPT_RESET`, `WMDRMDEVICEAPP_USE_WPD_DEVICE_PTR` across `windows`+`windows-sys`). The metadata value
     is right; only the emission was non-portable. Fixed in `bindgen` (`cpp_const.rs::pointer_sized_const_value`):
     a `usize` value `> u32::MAX` (or an `isize` outside `i32` range) is emitted as `<lit>u64 as usize`
     (resp. `<lit>i64 as isize`), which truncates to the target's pointer width — reproducing the correct
     arch-specific value (`0xFFFF_FFFF` on 32-bit, `0xFFFF_FFFF_FFFF_FFFF` on 64-bit) — while values that
     already fit a 32-bit target keep the bare literal (no churn). Regression fixture:
     `crates/tests/libs/bindgen/input/const_pointer_sized.rdl`.
- **Floating-point constants scraped — done (systematic gap).** The flat Win32 metadata carried
  **zero** floating-point constants (`Select-String "pub const \w+: f(32|64)"` over the whole
  generated `windows` crate = 0 matches); every `windows-clang` const path handled only strings and
  integers, so all float `#define`s and `const double` values were silently dropped. The whole
  downstream pipeline was already float-capable (`metadata::Value::F32/F64`, the winmd R4/R8
  reader/writer, riddle RDL emit/parse, `bindgen`'s `ValueExt::write`) — only the scraper was lossy.
  Two clang sub-gaps, both fixed:
  1. **Float `#define` macros** (`#define D2D1_DEFAULT_FLATTENING_TOLERANCE 0.25f`). `parse_literal`
     (`const.rs`) only tried integer digits; added `parse_float_literal` (falls through when the
     integer parse fails). A literal qualifies as float only if it has a `.` or a decimal exponent
     `e`/`E` (so hex integers like `0xF` stay integers); an `f`/`F` suffix → `F32`, otherwise (or an
     `l`/`L` long-double suffix) → `F64`. Hex floats (`0x1.8p3`) are rejected.
  2. **File-scope `const float`/`const double` variables** (`const double UIA_ScrollPatternNoScroll =
     -1;`). The `CXCursor_VarDecl` handler in `lib.rs` only scraped `IID_*` GUID vars; added
     `Const::parse_var_decl` for const-qualified scalar `float`/`double`/`long double` VarDecls with an
     evaluable initializer, backed by a new `Cursor::evaluate_double` (`cx.rs`) that reads
     `CXEval_Float` *and* coerces a `CXEval_Int` result — libclang evaluates the integer initializer
     `-1` of a `const double` as an int, so it must be coerced to the declared float type. Pointers,
     aggregates, integer vars, and non-const vars are all rejected (keeps churn bounded and additive —
     floats were entirely absent, so nothing collides). Regression fixture:
     `crates/tests/libs/clang/input/const_float.h`.
  Result: **251 float constants** now emit (D3D/D2D `*_DEFAULT_*`/`*_SRGB_*`/`FLOAT32_MAX`/…,
  `UIA_*` doubles, etc.), all additive, all matching the retired editorial metadata's values.
- **`expandedresources.h` + `d3d11shadertracing.h` scraped into `tool_win32` — done.** Both were
  simply absent from `win32.toml`. `HasExpandedResources`/`GetExpandedResourceExclusiveCpuCount`/
  `ReleaseExclusiveCpuSets` now emit (resolving to the `api-ms-win-gaming-expandedresources` apiset
  via the `onecoreuap.lib` umbrella); `D3DDisassemble11Trace` + `ID3D11ShaderTrace`/`ID3D10ShaderTrace`
  emit (resolving to `D3DCOMPILER_47.dll` via `d3dcompiler.lib`). Note `d3d11shadertracing.h`'s
  content is attributed to the **`d3d11`** module (no separate `d3d11shadertracing` feature is
  generated) — so `D3DDisassemble11Trace` lives at `Win32::d3d11`, gated by the `d3d11` feature.
- **`cross.yml`'s `-p test_win32` is unblocked (ported and passing).** The `cross` workflow's
  cross-compile smoke test (`.github/workflows/cross.yml:95`) runs `cargo test --no-run -p test_win32`.
  The crate is now removed from the workspace `exclude` list and its three test files are ported to the
  flat header-stem API; `cargo test -p test_win32` passes all 21 tests. The four genuine metadata gaps
  that had blocked it are filled (above): `D3D12_DEFAULT_BLEND_FACTOR_ALPHA` (f32),
  `UIA_ScrollPatternNoScroll` (f64), `HasExpandedResources`, `D3DDisassemble11Trace`. Test-side
  API-drift adaptations applied:
  - **`win32.rs` / `hresult.rs`.** Enum newtypes are now bare integer aliases
    (`ACCESS_MODE`/`DXGI_ADAPTER_FLAG` → `type = i32`), so `X::default().0`→`X::default()`,
    `both.0 == 3`→`both == 3`; DXGI flag params/returns are bare `u32`
    (`MakeWindowAssociation(hwnd, 0)`, `GetCreationFlags() == 0`); `URI_CREATE_FLAGS::default()`→`0`;
    `STREAM_SEEK_SET` is `i32` (cast to `u32` for `IStream::Seek`); `PLDAPSearch` wraps a raw pointer
    (`PLDAPSearch(123 as _)`). BOOL-returning Win32 fns no longer auto-return `Result` (the flat
    metadata omits the error transform) — use `windows_core::BOOL::ok()` (`SetEvent(e).ok()?`,
    `MiniDumpWriteDump(..).ok().unwrap_err()`); `CreateEventW` now returns a bare `HANDLE` (drop the
    `?`). `hresult.rs`'s `ERROR_*` are bare `u32` (wrap `WIN32_ERROR(ERROR_SUCCESS).into()`).
  - **`winsock.rs` — ported by inlining the conversions as example helpers.** The std ↔ WinSock
    interop conversions it exercised were hand-baked `windows`-crate extensions that are being
    intentionally removed (see "Intentionally-removed `windows`-crate extensions" below), *not* a
    metadata gap. Because the orphan rule forbids `impl From<std::net::*> for <foreign WinSock struct>`
    in a downstream crate, the test now carries small free-function equivalents (`to_in_addr`,
    `from_in_addr`, `to_sockaddr_in`, …) that construct the raw structs directly — doubling as a worked
    example of how a caller does the conversion themselves. Drift folded in: `AF_INET`/`AF_INET6` are
    bare `u32` (was the `ADDRESS_FAMILY` newtype) while `sin_family`/`si_family` stay
    `ADDRESS_FAMILY(pub u16)`, so comparisons need `ADDRESS_FAMILY(AF_INET as u16)`; `SOCKADDR_IN6` is a
    type alias for `SOCKADDR_IN6_LH` (its anonymous union is `SOCKADDR_IN6_LH_0`).
- **Intentionally-removed `windows`-crate extensions (not metadata gaps).** A set of hand-authored
  ergonomic extensions that the old editorial `windows` crate baked on top of the generated bindings
  (see `crates/libs/windows/src/extensions/` on the `master` branch) are being **deliberately dropped**
  in the in-house/flat projection — do **not** try to "fill" them as scraper gaps. Known members: the
  **std ↔ WinSock net conversions** (`IN_ADDR`/`IN6_ADDR`/`SOCKADDR_IN`/`SOCKADDR_IN6`/`SOCKADDR_INET`
  ↔ `std::net::*`), the **`VARIANT`/`PROPVARIANT` extension helpers** (`Win32/System/Variant.rs`,
  `Win32/System/StructuredStorage.rs`), the **`VARIANT_BOOL`** helpers, and the WinRT
  **`Foundation::DateTime`** conversions. Tests/samples that relied on them must construct/convert the
  raw types themselves (as `winsock.rs` now demonstrates) or drop the reliant assertions. The
  `test_extensions` and `test_variant` crates existed solely to test these removed helpers and have
  been **deleted** (`bool32.rs`/`ntstatus.rs` covered `windows-core`'s `BOOL`/`NTSTATUS`, which the
  `test_error`/`test_result` crates already exercise).
- **Un-inferable curated attributes removed from `windows-bindgen` (decision, PR #4689).** The old
  editorial Win32 metadata (Microsoft's `win32metadata` project) carried several *hand-curated*
  attributes that no scraper can derive from C/C++ headers alone. `windows-clang` therefore emits
  **none** of them, so the `windows-bindgen` code that consumed them was dead for the in-house
  pipeline. Rather than carry dead branches (or fake the attributes), the handling was **removed** and
  the four test crates that existed only to assert those transforms were **deleted**
  (`test_agile`/`test_alternate_success_code`/`test_handles`/`test_return_handle`). The four attributes
  and what each removal drops:
  1. **`InvalidHandleValueAttribute`** (`type_def.rs::invalid_values`, `cpp_handle.rs`,
     `cpp_method.rs::handle_last_error`). Supplied per-type invalid sentinels (e.g. `HANDLE` = `{0, -1}`,
     `MSIHANDLE` = `{0, u32::MAX}`, `HANDLE_SDP_TYPE` = `{0, u64::MAX}`) so `is_invalid()` recognised them.
     Without it, **pointer** handles keep a null-check `is_invalid()` (`self.0.is_null()` — independent of
     the attribute) but non-pointer handles get no `is_invalid()`, and `HANDLE(-1)` is no longer reported
     invalid. (`test_handles`.)
  2. **`RAIIFreeAttribute`** (`type_def.rs::free_function`, `cpp_handle.rs`). Named the free routine for a
     handle, driving the `windows_core::Free` impl behind `Owned<T>`. Without it no `Free` impls are
     emitted, so `Owned<HANDLE>`/`Owned<HLOCAL>`/… do not compile; callers use explicit
     `CloseHandle`/`LocalFree`.
  3. **`AgileAttribute`** (one arm of `type_def.rs::is_agile`). Marked a type free-threaded → `Send`/`Sync`.
     **Only the `AgileAttribute` arm was removed** — `is_agile` is retained because
     **`MarshalingBehaviorAttribute`** (value `2` = agile) *does* survive in the WinRT `Windows.winmd`
     (it is preserved through the SDK-contract merge in `tool_windows`), so WinRT types such as `Uri`
     keep their `Send`/`Sync`. The gap is Win32-scraped COM interfaces (e.g. `IRestrictedErrorInfo`),
     which carry neither attribute and so are not `Send`/`Sync`. (`test_agile`.)
  4. **`CanReturnMultipleSuccessValuesAttribute`** (guard in `cpp_method.rs`). Opted a method *out* of the
     `HRESULT → Result` transform so multi-success HRESULTs (e.g. `DoDragDrop` returning
     `DRAGDROP_S_DROP`) stayed raw. With no method carrying it the guard was always taken, so it was
     unwrapped; `DoDragDrop` now returns `Result<u32>`. (`test_alternate_success_code`.)

  **Soundness / proof.** All four attributes occur **zero** times in every input winmd
  (`crates/libs/bindgen/default/{Windows,Windows.Win32,Windows.Wdk}.winmd`; `MarshalingBehaviorAttribute`
  — deliberately kept — is the lone survivor). Regenerating **every** downstream output after the removal
  (`tool_package`, `tool_bindings`, `tool_reactor`, and all 88 `test_bindgen` goldens) produced a
  **byte-identical** diff, confirming the code was truly unreachable for this repo's metadata. **Win:**
  ~90 lines of attribute plumbing and four brittle drift-prone test crates gone. **Tradeoff:**
  `windows-bindgen` is also a published, general-purpose tool — a consumer who feeds it Microsoft's
  *official* `win32metadata` (which *does* set these attributes) will no longer get handle
  invalid-sentinels, `Owned<T>`/`Free`, the `AgileAttribute`-driven marker, or `AlternateSuccessCodes`
  preservation. This is an accepted, explicit narrowing of scope toward the in-house flat metadata as
  the source of truth; the retained `MarshalingBehaviorAttribute` path keeps WinRT agility working for
  both pipelines.
- **`return_handle` / `SupportsLastError` is a separate un-inferable gap.** `test_return_handle`
  (deleted above with the curated-attribute crates) expected `CreateEventA` to return `Result<HANDLE>`
  and yield the last-error code on a duplicate-name collision. That transform is gated not by an invalid
  sentinel but by the **`SupportsLastError`** PInvoke flag (`cpp_method.rs::handle_last_error`, and the
  `BOOL → Result<()>` arm), which `win32metadata` set from documentation/annotations and which
  **`windows-clang` cannot infer from headers** (`SupportsLastError` occurs zero times in
  `Windows.Win32.winmd`). So `CreateEventA` returns a bare `HANDLE` and Win32 `BOOL` functions no longer
  auto-`Result` — the same curated-attribute class as the four above.
- **Follow-up: remove the now-dead `Owned`/`Free` runtime support from `windows-core`.** `RAIIFree`
  was the only attribute with a *runtime* counterpart in `windows-core`: the `Free` trait and the
  `Owned<T>` wrapper in `crates/libs/core/src/resources.rs` (wired via `mod resources; pub use
  resources::*;` in `crates/libs/core/src/windows.rs`). With `RAIIFree` gone, `windows-bindgen`
  emits **no `Free` impls**, so nothing in the generated projection uses `Owned<T>` anymore. The only
  remaining consumer is the self-contained unit test `crates/tests/libs/core/tests/handles.rs`, which
  exercises the machinery with a mock `FreeCounter` (no metadata dependency). **TODO:** delete
  `resources.rs`, drop its `mod`/`pub use` in `windows.rs`, and remove `handles.rs`. This is a
  breaking change to the public `windows-core` surface (`windows::core::Owned` / `windows::core::Free`
  are exported), so it needs explicit sign-off before landing. The other three removed attributes
  (`InvalidHandleValue`, `Agile`, `CanReturnMultipleSuccessValues`) and the `SupportsLastError` flag
  have **no** `windows-core` runtime plumbing — their handling was entirely inside `windows-bindgen`
  (inline `is_invalid`, `unsafe impl Send/Sync`, the HRESULT/last-error transforms) — so there is
  nothing else of this kind to remove.
- **Move and rename the `spellchecker` sample.** `windows_spellchecker` does not use the `windows`
  crate — it generates its own bindings with `windows-bindgen` (`build.rs`) on top of `windows-core`,
  so it belongs with the other bindgen-driven samples, not under `crates/samples/windows/`. **TODO:**
  relocate it to the bindgen sample directory and rename it for consistency. It is intentionally left
  out of the workspace `members` for now. Note it also still needs the one in-house-metadata fix its
  peers needed (`COINIT_MULTITHREADED.0 as u32` → `COINIT_MULTITHREADED as u32`, since `COINIT` is now
  a bare `i32` enum alias) before it will compile against the current default metadata.
- **Restore the `crates/samples/*/*` glob** in the workspace `Cargo.toml` `members` — **done.** The
  glob is restored; only `crates/samples/windows/spellchecker` remains in `exclude` (pending the
  relocation/rename above).

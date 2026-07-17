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

## Scraper layering: one crate, two levels

`windows-clang` is the single crate a scraper depends on. It is *not* a thin libclang
FFI shim — it is a scraping toolkit with a clean two-level API, layered on the metadata
crates below it:

```
windows-metadata          base: winmd read/write
  └─ windows-rdl          RDL text ⇄ winmd; reader(), merge_arch_rdl, ArchInput
       └─ windows-clang   provisioning + parse (clang()) + partitioning + multi-arch scrape()
            └─ tool_win32 / tool_wdk / tool_webview / <third-party scraper>
```

It is *one* builder — `clang()` — with *two* terminals. A consumer configures the builder
the same way for either and picks the terminal that fits its corpus:

- **`.write()` — one shot.** One translation unit, one architecture, one file:
  `clang().args(..).input(..).output(..).write()` then `reader()`. Vendored headers, no
  provisioning, no arch-merge. This is all `tool_webview` uses (~45 lines) to turn
  `WebView2.h` into `WebView2.rdl` → `WebView2.winmd`.
- **`.scrape(ScrapePlan)` — the multi-architecture corpus.** The same configured builder,
  replayed once per architecture (swapping the target triple + per-arch defines): scrape
  each arch into its own per-header RDL partition and winmd, `merge_arch_rdl` so a symbol
  that only exists on (or differs across) a subset of arches gains `SupportedArchitecture`,
  then re-derive one unified winmd from the merged corpus. `tool_win32` and `tool_wdk` are
  both just: resolve the toolchain (NuGet packages → include/lib dirs), configure a
  `clang()` builder (TU sources, include args, scope, import libraries), and call `.scrape`
  with a small [`ScrapePlan`].

The two terminals share the builder because the corpus scrape *is* a single-arch scrape
replayed — so every parse knob (headers, args, scope, import libraries, `drop_lib_less`) is
set once, on the builder that owns it, and `scrape` only layers on the per-arch target,
defines, and (for a multi-arch run) the builtin `-resource-dir`.

### What lives where, and why

Everything generic to *any* header scrape lives in `windows-clang`:

- **Provisioning** — `ensure_libclang` / `assert_libclang_version` (the pinned
  `LIBCLANG_VERSION` wheel, fetched + cached on first use), `clang_resource_dir`, and
  `nuget_package` (restore a pinned NuGet package into the global cache).
- **Parse + emit** — the `clang()` builder (target, args, `input`/`input_str`, `scope`,
  `scope_headers`, `import_library`, `drop_lib_less`), header partitioning
  (`write_by_header`), and the per-kind cursor→RDL modules.
- **Multi-arch orchestration** — the `Clang::scrape` terminal, `Arch` (clang triple +
  `SupportedArchitecture` bits + per-target defines), `ScrapePlan` (the orchestration-only
  state: output paths, arches, reference winmds, seed — *not* a mirror of the builder), and
  `Summary`. This is pure driver: nothing in it is win32- or wdk-specific.

Only what is *genuinely per-scraper* stays in each tool: the NuGet package IDs and pinned
versions, the SDK/WDK include+lib directory layout, the translation-unit source assembly
(the `windows.h` prelude, the `INITGUID` satellite reset, the WDK `offreg` prelude), the
SAL shim, and the `*.toml` manifest of headers/scope/import-libs/arches. A third-party
scraper is a fourth peer of this exact shape: provision its own toolchain, configure a
`clang()` builder, call `.scrape`.

This boundary is deliberate. `windows-clang` already depends on `windows-rdl` and
`windows-metadata` and already reuses `windows_rdl::emit`, so the multi-arch driver —
which only coordinates `clang()`, `merge_arch_rdl`, and `reader()` — belongs here rather
than in a separate crate that would sit on top of clang only to add a thread pool. Making
it a terminal on the same builder gives every consumer one dependency and one mental model:
*use `windows-clang` to scrape*. The earlier `windows-scraper` crate was collapsed into
this module for exactly that reason.

## Internals

The scraper is organized by declaration kind: `cx` wraps `clang-sys` (the AST
cursor/translation-unit types), `canon` holds the faithful → canonical type
remaps, and per-kind modules (`r#enum`, `r#struct`, `r#const`, `r#fn`, `callback`,
`interface`, `typedef`, `field`, `annotation`) parse a cursor and emit RDL. The
`collector` accumulates items per namespace; `item` dispatches emission. Header
partitioning routes each declaration back to its defining header so the output is a
per-header `<stem>.rdl` file set.

`lib.rs` holds the `clang()`/`Clang` builder, the `Parser` cursor walker, and the two
emit terminals; its cross-cutting free helpers are grouped into focused sibling modules:
`guid` (GUID-initializer parsing), `scope` (the reachability sweep, reference/resolution
maps, and header-in-scope tests), `naming` (tag-rename and nested-type synthetic naming),
and `macros` (the object-like-macro evaluation pass). Both emit paths — `parse_and_emit`
(the namespaced `write` path used by `tool_webview`) and `parse_and_emit_by_header` (the
flat per-header `write_by_header` path used by `tool_win32`/`tool_wdk`) — share one
`parse_inputs` helper that loads libclang and parses every header and in-memory source into
translation units once. Its `ParsedInputs` return owns the libclang `Library` guard and
`Index` so the translation units stay valid for its whole lifetime; the paths bind it whole
(never destructure it) so its field declaration order governs drop, disposing the units and
index before the guard unloads libclang.

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

The **only** hand-authored input is `metadata/metadata.rdl` — the attribute
vocabulary (`NativeTypedefAttribute`, `SupportedArchitectureAttribute`,
`DoesNotReturnAttribute`, …) that exists only in metadata, so no C header can declare
it. It lives beside (not inside) the generated `metadata/win32` corpus so that directory
can be freely cleared and rebuilt without disturbing this prerequisite. Everything else —
including the `Foundation` scalars `BOOL`/`HANDLE`/`HRESULT` —
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

### Base-interface `_COM_Outptr_` creators: promoted by name gate (positional rule rejected)

A creator like `DWriteCreateFactory(_In_ REFIID iid, _COM_Outptr_ IUnknown **factory)`
(`um/dwrite.h`) is a caller-chosen-type factory whose out-parameter *should* have been
declared `void**` with `[iid_is(iid)]` — the header spells the pointee as the base interface
`IUnknown**` (and elsewhere `IInspectable**`) instead. This is a source bug in the SDK header
that cannot be fixed upstream now. (`IWeakReference::Resolve` looks identical but the
*MIDL-generated* `winrt/WeakReference.h` carries an explicit `[iid_is][out]` comment, so it is
already promoted via the `[iid_is]`-comment path — the difference is the annotation, not the
pointee type.)

Auto-promoting the base-interface form on a **positional** rule was investigated and
**rejected**: a corpus scan of `um`/`shared` shows the base-interface `_COM_Outptr_` shape is
dominated by genuine fixed-type returns with **no** `REFIID` sibling (`SHGetThreadRef`,
`GetProcessReference`, `InstantiateComponentFromPackage`, the D3D11 interop
`CreateDirect3D11Device*` accessors, `IDWriteTextLayout::GetDrawingEffect`, …) that must stay
typed; and even the `REFIID`-sibling subset mixes last-parameter cases (`DWriteCreateFactory`)
with mid-list cases (`DbgModel` concept accessors) — no positional rule cleanly separates true
creators from coincidental `REFIID`+`IUnknown**` pairings without risking a wrong (lossy)
promotion.

The **name gate** does separate them, so it is what `infer_iid_is` uses (see
`annotation.rs`): the same `HRESULT`-return + a `*const GUID` parameter named
`riid`/`iid`/`riidltf` requirement as the `void**` arm, plus the out-parameter must be the
**bare** base spelling (`is_base_interface_out_ptr`: `*mut IUnknown`/`*mut IInspectable`,
never a concretely typed `IFoo**`). At the ABI a COM interface pointer is itself a pointer, so
`*mut IUnknown` is byte-identical to `*mut *mut void`; the promotion normalises to the latter
and marks `[iid_is]`. A whole-corpus regen promotes exactly **5** methods across 4 functions —
`DWriteCreateFactory`, `IMFCaptureSink::GetService` (×2), `IOpenRowset::OpenRowset`,
`ITableCreationWithConstraints::CreateTableWithConstraints` — every one a genuine caller-chosen
rowset/service/font factory whose `riid` selects the out-pointer's type, and each previously
projected the awkward `iid: *const GUID` + `-> Result<IUnknown>` middle state that forced a
caller `.cast()`. The bare-spelling restriction is what excludes the fixed-type creators whose
`riid` selects a *different* object than the out-pointer (`ActivateAudioInterfaceAsync`'s
`IActivateAudioInterfaceAsyncOperation**`, `RoGetAgileReference`'s `IAgileReference**`), which
keep their concrete type and a call-site `.cast()`. Guarded by the `iid_infer` `test_clang`
fixture (`CreateFactory`/`CreateInspectable` positive, `CreateTyped` → `IFoo**` negative).

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

### Redundant constants that duplicate an enumerator are dropped

A handful of legacy headers expose the *same* value twice: once as a typed
enumerator and once as a loose object-like macro in an unrelated header. The bare
macro copy is a weaker-typed duplicate of the canonical enumerator, and — because the
whole corpus shares one flat namespace — the two would collide once both headers are
in scope. Examples:

- `D3DFORMAT::D3DFMT_X8R8G8B8` (d3d9types.h) vs `#define D3DFMT_X8R8G8B8 22` in
  mfapi.h — the latter a transient `#define`/`#undef` scaffold for
  `DEFINE_MEDIATYPE_GUID` that libclang still reports as a macro definition.
- `OLEMISC::OLEMISC_ACTSLIKELABEL` (oleidl.h) vs the legacy `#define` in olectl.h.
- `MIB_IPROUTE_TYPE::MIB_IPROUTE_TYPE_DIRECT` vs the same-file `#define` in ipmib.h.

After the reachability sweep, a final pass drops any top-level integer constant whose
name **and** value match an enumerator defined anywhere in the corpus, provided
nothing else references the constant (so a genuinely distinct constant that merely
shares a name with an unrelated enumerator is never removed). The typed enumerator is
the canonical projection and survives untouched. This keeps names globally unique
without a curated exclusion list (`enum_member_values` / `const_integer_bits` /
`enum_member_eq` in `lib.rs`).

## The in-house corpus: tool_win32

```text
   Windows SDK ──► clang / libclang ──► one RDL file per header ──► Reader ──► per-arch winmd
   • headers          (windows-clang)      (metadata/win32/*.rdl)                      │
   • SAL              parse shared TU once, walk once, USR-dedup, flat namespace       ▼ merge
   • import libs      (+ hand-authored attribute seed metadata.rdl)           Windows.Win32.winmd
     (symbol → DLL)                                                                    │
                                                                                       ▼ windows-bindgen ──► Rust
```

`cargo run -p tool_win32` builds the in-house winmd. Its configuration is a set of plain
`const` slices at the top of `crates/tools/win32/src/main.rs` (`HEADERS`, `SATELLITE_HEADERS`,
`SCOPE`, `IMPORT_LIBS`, …) listing the SDK headers, satellite rules, and
import libs — deliberately **no type-level curation**. The tool resolves its pinned
toolchain, configures a `windows_clang::clang()` builder, and drives it with
`windows-clang`'s shared [`scrape`](#scraper-layering-one-crate-two-levels) terminal, which
builds one shared translation unit per target arch (`windows.h` prelude + every manifest
header), emits it via a single `write_by_header` call (parsed once, USR-deduped), reads the
per-arch RDLs to winmd, and coalesces the arches with the multi-arch merge. New APIs are added by
**listing the defining header** in the manifest and regenerating — the reachability
closure is automatic. A full generational SDK bump (e.g. 24H2 → 25H2) is absorbed the
same way, with no scraper changes.

Functions whose exporting DLL cannot be recovered from any import library in the
manifest are **dropped** (matching the reference, which carries none): a function with
an empty `#[library("")]` would otherwise force `windows-bindgen` to emit
`link!("" …)`, a hard `E0454` error. This is the opt-in `drop_lib_less` flag, set only
by `tool_win32`; unit-test fixtures that supply no import libraries leave it off so
they still emit their functions.

The import-library list (`IMPORT_LIBS` in `main.rs`) is ordered by resolution priority (first-wins):
per-DLL host libraries (`kernel32.lib`) first, the `api-ms-win-*` apiset umbrella
(`onecore.lib`, `onecoreuap.lib`) next, and `vertdll.lib` — the VBS-enclave runtime —
**dead last**. `vertdll.dll` also exports host-side synchronization/enclave functions
(`WaitOnAddress`, `WakeByAddress*`, `CallEnclave`, `TerminateEnclave`), and importing
those from `vertdll.dll` faults a normal process at load; ordering it after the apiset
umbrella lets them resolve to their loadable `api-ms-win-core-synch`/`-enclave` contract,
leaving `vertdll.lib` to stamp only genuinely enclave-only residue (`EnclaveSealData`, …).

### WinRT interop headers and the `ABI::Windows::*` split

A handful of SDK headers declare Win32 COM entry points whose signatures reach into the
`ABI::Windows::*` C++/WinRT projection namespace — e.g. `roregistrationapi.h`'s
`RoGetActivatableClassRegistration`, which returns an
`ABI::Windows::Foundation::IActivatableClassRegistration**`. That namespace holds two kinds of
type that need opposite handling:

- *Win32 COM interop types* that live in `ABI::Windows::*` but are **not** projected WinRT types
  (absent from `Windows.winmd`) — `IActivatableClassRegistration`, `ActivationType`,
  `RegistrationScope`. These are fully defined COM interfaces/enums and belong in the flat
  `Windows.Win32` namespace like any other Win32 type.
- *True WinRT projection types* that already exist in `Windows.winmd` (e.g.
  `Windows.Foundation.Collections.IMapView`2`). Flattening these would drag the whole WinRT surface
  into Win32, so a referenced one must become a cross-winmd reference (the `Windows.Wdk` →
  `Windows.Win32` pattern, here an `AssemblyRef` on the `Windows` assembly).

`tool_win32` tells the two apart by carrying `Windows.winmd` as a **resolution winmd**
(`RESOLUTION_WINMDS`, threaded through `Clang::resolution_input`): a *resolution-only* input that
is never emitted or excluded, used purely to classify names. `load_winrt_types` reads its
backtick-stripped `Namespace.Name` set; `flatten_decls` then recurses into the `ABI` namespace
(instead of skipping it) and keeps a declaration only when it is *absent* from that set, while
`abi_projection` (`cx.rs`) maps a *present* name to the cross-winmd reference. A generic
instantiation (`IMapView<HSTRING, IInspectable *>`) is rebuilt into its closed WinRT form
(`IMapView<String, Object>`) from the clang template arguments, mapping the C++/WinRT ABI
spellings back to WinRT primitives (`HSTRING` → `String`, `IInspectable *` → `Object`). With no
resolution winmd supplied (e.g. `tool_wdk`) the `ABI` namespace is skipped entirely, as before.

The MIDL-mangled parameterized-interface aliases those headers emit
(`typedef ABI::…::IMapView<HSTRING, IInspectable *> __FIMapView_2_HSTRING_IInspectable_t;`) are
inlined at every use site rather than surfaced: a typedef whose canonical is an `ABI::…<…>` generic
instantiation projects directly to the WinRT generic (`get_Attributes` returns
`IMapView<String, Object>`, not the mangled alias), and the now-unreferenced alias declaration is
dropped by the reachability sweep. Because a WinRT parameterized type is always a COM interface,
`is_interface` also recognizes that canonical form, so the implied-pointer collapse strips the extra
indirection (`IMapView **` → `*mut IMapView`, matching `IActivatableClassRegistration **` →
`*mut IActivatableClassRegistration`) even though the generic's template body is never instantiated
in the scrape.

The same split covers the WinRT *interop* headers — `Windows.Graphics.Capture.Interop.h`,
`windows.ui.composition.interop.h` — whose interfaces (`IGraphicsCaptureItemInterop`,
`ICompositorInterop`, `ICompositionCapabilitiesInteropFactory`, …) hand out projected objects as
COM out-parameters. Two wrinkles beyond the `roregistrationapi.h` case:

- *The projection `#include` does not parse.* Each interop header `#include`s its sibling
  `winrt\` C++/WinRT projection header (e.g. `winrt\windows.ui.composition.h`) purely to name a few
  `ABI::Windows::*` interfaces, but that projection drags in a closure that fails to parse (a
  transitive typedef/enum conflict, e.g. `winrt\Windows.Devices.Sensors.h`'s `MagnetometerAccuracy`).
  Rather than abort the whole translation unit, the by-header scrape is *error-tolerant*: a
  diagnostic whose source file is an emitted (in-scope) header still fails loudly, but a diagnostic
  in a transitive-only include — a `winrt\` projection header that is never itself emitted — is
  tolerated, and the scrape proceeds on clang's best-effort AST (which still carries the in-scope
  interop interfaces). `tool_win32` passes `-ferror-limit=0` so clang does not truncate the later
  declarations after those errors. The `ABI::Windows::*` interfaces the interop headers dereference
  are then classified by the resolution winmd exactly as any other `ABI::` reference.
- *A non-generic `ABI::` interface reference is only forward-declared.* The `typedef interface IC IC;`
  such a header emits leaves the underlying record incomplete, so the structural
  pure-virtual probe in `is_interface` would miss it and `resolve_typedef` would flat-root the bare
  name into a dangling `Windows.Win32` reference. `is_interface` and the `to_type` typedef arm
  therefore recognize a non-generic `ABI::…` *record* canonical directly (symmetric with the
  `ABI::…<…>` generic case above): it routes through `abi_projection` for the cross-winmd reference
  and collapses the implied pointer (`ICompositionTexture **` → `*mut ICompositionTexture`). An
  `ABI::` *enum* typedef (canonical kind `Enum`, e.g. `ActivationType`) is excluded and keeps its
  own name through `resolve_typedef`.

Downstream, the flat `Windows.Win32.winmd` then carries a reference to a WinRT type. The full
`windows` crate loads `Windows.winmd` and projects it normally (`IMapView<String, Object>`
re-exported from `windows-collections`); `windows-sys`, which reads only the Win32 corpus,
degrades an unresolvable WinRT reference type to the opaque COM pointer (`*mut c_void`) — the same
representation it already gives every interface (`windows-bindgen`, `types::from_metadata_type`).

## The WDK corpus: tool_wdk

`cargo run -p tool_wdk` builds `Windows.Wdk.winmd` the same way `tool_win32` builds the
Win32 winmd — a whole-header scrape, not a symbol allowlist. Both tools drive the *same*
[`scrape`](#scraper-layering-one-crate-two-levels) terminal from plain `const` slices in
`main.rs` (`crates/tools/wdk/src/main.rs`); the WDK is just that builder configured for three
things:

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

New WDK APIs are added exactly like Win32 ones: list the defining header in `main.rs`'s
`SOURCE_HEADERS` and regenerate. The WDK NuGet version is pinned independently of the SDK
(its servicing build lags), but tracks the same marketing line.



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

## Differences from the win32metadata reference

The in-house metadata is faithful to the SDK headers, so it differs from Microsoft's
editorial [win32metadata](https://github.com/microsoft/win32metadata) winmd in several
deliberate, consumer-visible ways. These are correct-by-design, not projection defects;
they are the practical consequence of the faithfulness philosophy above.

- **Flat module/feature layout.** Each source-header stem is its own top-level module and
  same-named leaf feature (`windows::winnt`, feature `winnt`) — there is no nested
  `Win32_UI_Shell` editorial feature tree. Each leaf feature's dependency list is *computed*
  from the type graph, and Win32/WDK feature names carry no `Win32_`/`Wdk_` prefix (the
  globally-unique header stem is already unambiguous). WinRT keeps its dotted path joined
  with `_` (`Foundation_Collections`).
- **Unscoped C enums are bare integer aliases** in every style (`pub type X = i32;` plus
  bare `pub const` members); scoped enums (WinRT enums, C++ `enum class`) stay newtypes.
  Flag enums rely on the underlying integer's native `|`/`&`/`!`, so no `BitOr`/`BitAnd`
  impls are emitted. Call sites drop `.0` and tuple construction.
- **Handles carry no `is_invalid()`, no `Owned<T>`/`Free`.** Those came from curated
  win32metadata attributes that cannot be inferred from headers. Check `.0.is_null()` for
  null-sentinel handles or `== INVALID_HANDLE_VALUE` for the `-1` family, and free with an
  explicit `CloseHandle`/`LocalFree`.
- **No fabricated `Result`/retval ergonomics.** Out-params and `BOOL`/`HRESULT` returns are
  faithful to the header — call `.ok()` on a `default()`, pass raw `*mut` out-params. A
  function the reference hand-patched via `SupportsLastError` returns a bare handle/`BOOL`
  here.
- **Un-inferable curated attributes are not emitted:** `InvalidHandleValue`, `RAIIFree`,
  `AgileAttribute`, `CanReturnMultipleSuccessValues`, and the `SupportsLastError` PInvoke
  flag. `MarshalingBehaviorAttribute` (agility) *does* survive the WinRT SDK-contract merge,
  so WinRT types (`Uri`, …) stay `Send`/`Sync`; Win32-scraped COM interfaces do not.
- **Error ergonomics come from `windows::core`, not a metadata stem.** `WIN32_ERROR` with
  `to_hresult()`/`From` is the `windows-result` type re-exported through `windows::core`; the
  bare `winreg::WIN32_ERROR(u32)` newtype the flat metadata emits has none. `GetLastError`/
  `SetLastError` and `ERROR_*`/`*_E_*` constants are bare integers — wrap them for
  `Error::from`.
- **Hand-authored `windows`-crate extensions are dropped:** the std ↔ WinSock net
  conversions, the `VARIANT`/`PROPVARIANT`/`VARIANT_BOOL` helpers, and the WinRT
  `DateTime` ↔ `FILETIME` bridge. Callers construct/convert the raw types directly.
- **`windows-sys`** is generated by the same pipeline into the identical header-stem layout;
  its projection is simpler (raw pointers, no COM ergonomics) but follows the same
  flat-feature and bare-alias rules.

## Known limitations and future work

None of these block use of the crate.

- **Coverage is the `HEADERS` list.** The corpus covers exactly the headers listed in
  `tool_win32`/`tool_wdk`'s `HEADERS`/`SOURCE_HEADERS` consts; a missing API is usually a
  one-line header addition (the reachability closure does the rest). Some surface is still
  structurally out of reach: headers needing compiler-intrinsic or out-of-SDK toolchains
  (DirectXMath → `cpuid.h`; DISM/WIMGAPI in the ADK; `mscoree.h` in the NETFXSDK), and
  headers that `#include` the C++/WinRT (`cppwinrt\`) projection, which does not parse in the
  definition-mode scrape — though a header that only *references* a handful of `ABI::Windows::*`
  interfaces is reached via error-tolerant parsing (the scrape tolerates parse errors in the
  transitive-only `winrt\` projection closure while still failing loudly on errors in emitted
  headers; see the WinRT interop section). And the single flat `Windows.Win32` namespace is **lossy
  for genuine name collisions** — where the reference disambiguated two distinct entities by
  editorial sub-namespace, dedup keeps one (e.g. `PID_SECURITY`, the `E_NOTFOUND`
  HRESULT-vs-`#define` class).
- **Parsing fidelity.** Two SAL surfaces bypass the shared `parse_params`: struct fields
  (`_Field_size_` is not mapped to array-size metadata, which bindgen never consumes) and
  callback return attributes. A `__fastcall` callback is recorded faithfully in the winmd but
  `windows-bindgen` downgrades it to `extern "system"` (Rust's `extern "fastcall"` fn-pointer
  ABI is nightly-only); moot for the current corpus.
- **Performance is near-optimal.** The multi-arch scrape is parallel (one worker per arch)
  and the per-arch clang parse of the SDK translation unit dominates wall time; import-lib
  resolution is parsed once on the builder and cloned per arch. No cheap, low-risk speedup
  remains.
- **IDL as the COM source of truth (future direction).** Parsing `.idl` (or `midl` output)
  directly would recover the pointer-shape attributes headers don't express as SAL
  (`[unique]`/`[length_is]`/`[iid_is]`), keeping the header path for flat C APIs.
- **Drop the empty `windows-sys` interop modules.** An all-interface interop header
  (`windowsgraphicscaptureinterop`, `windowsuicompositioninterop`, and the pre-existing
  `windowsgraphicsimaginginterop`/`windowsmediacoreinterop`/… siblings) projects to an *empty*
  module in `windows-sys` — the flat crate emits no COM interfaces — yet `tool_package` still
  writes the file and a dead Cargo feature. These empty modules and their features should be
  suppressed in the `--sys` package writer (skip a namespace that yields zero `sys`-visible items)
  so the flat crate carries no inert surface.

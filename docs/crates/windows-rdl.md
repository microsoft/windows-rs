# windows-rdl

> A parser for RDL (Rust Definition Language) and an ECMA-335 metadata generator.

- 📦 [crates.io](https://crates.io/crates/windows-rdl)
- 📖 [docs.rs](https://docs.rs/windows-rdl)
- 🚀 [Getting started](../../crates/libs/rdl/readme.md)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/rdl)

`windows-rdl` is the front of the metadata-authoring pipeline. It parses **RDL**
(Rust Definition Language) — a compact, Rust-like syntax for describing Windows
APIs — and compiles it into the same ECMA-335 `.winmd` metadata that
[`windows-bindgen`](windows-bindgen.md) consumes. It also runs the pipeline in
reverse (`.winmd` → canonical RDL). When an API ships only as a header and has no
metadata of its own, the companion [`windows-clang`](windows-clang.md) crate scrapes
the C/C++ headers into RDL that this crate then compiles.

Where `windows-bindgen` answers *"I have metadata, generate Rust"*,
`windows-rdl` answers *"I don't have metadata yet — produce some"*. The two are
designed to be used together: `windows-rdl` manufactures a `.winmd`, then
`windows-bindgen` turns it into `bindings.rs`.

## Getting started

Add `windows-rdl` as a build dependency (it is typically run from a small
codegen tool or `build.rs`, not shipped at runtime):

```toml
[build-dependencies]
windows-rdl = "0.0.0"
```

The crate exposes three builders, one per direction of the pipeline:

- `reader()` — RDL source → `.winmd` metadata.
- `writer()` — `.winmd` metadata → canonical RDL source.
- `clang()` — C/C++ headers → RDL source.

### RDL → winmd, and back

Use the `reader` to compile `.rdl` into a `.winmd`, and the `writer` to
regenerate canonical `.rdl` from a `.winmd`:

```rust,no_run
// RDL source -> winmd metadata.
windows_rdl::reader()
    .input("example.rdl")
    .output("example.winmd")
    .write()
    .unwrap();

// winmd metadata -> canonical RDL source.
windows_rdl::writer()
    .input("example.winmd")
    .output("example.rdl")
    .write()
    .unwrap();
```

RDL can reference types it does not define (for example `HRESULT` or
`Windows::Win32::System::Com::IUnknown`). Add the standard metadata as an extra
`reader` input so those references resolve — the bundled metadata lives in
`crates/libs/bindgen/default`:

```rust,no_run
windows_rdl::reader()
    .input("example.rdl")
    .input("crates/libs/bindgen/default")
    .output("example.winmd")
    .write()
    .unwrap();
```

### C/C++ headers → RDL

When an API ships only a C/C++ header, the [`windows-clang`](windows-clang.md)
crate's `clang()` parses it into RDL, which the `reader` then compiles to
metadata. Each header is parsed as its own translation unit — only its own
top-level declarations are emitted, not the things it `#include`s — so list every
header you need as a separate `input`:

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

## RDL syntax

RDL looks like a stripped-down Rust module. A top-level `mod` is a metadata
namespace, tagged `#[winrt]` or `#[win32]` to choose the type system. Attributes
map to metadata attributes, and the item keywords mirror the metadata kinds.

```text
#[win32]
mod Example {
    #[repr(i32)]
    enum Color {
        Red = 1,
        Green = 2,
        Blue = 3,
    }

    struct Point {
        x: i32,
        y: i32,
    }

    const MAX: u32 = 42;

    #[library("example.dll")]
    extern fn GetPoint() -> Point;

    #[guid(0x00000001_0002_0003_0004_000000000005)]
    interface ICustom : Windows::Win32::System::Com::IUnknown {
        fn Method(&self, value: i32) -> i32;
    }
}
```

Most attributes name a metadata attribute type directly, but a few have short
pseudo-attribute spellings that the reader expands to their full metadata attribute
(`PSEUDO_ATTRS` in `windows-rdl`). For example a struct field's bit-field members are
written `#[bitfield("Name", offset, width)]`, which the reader encodes as the
`Windows.Win32.Metadata.NativeBitfieldAttribute` custom attribute (and the writer
renders back to the short form on round-trip). See
[`windows-clang`](windows-clang.md#bit-field-member-scraping) for how the scrape emits
these and [`windows-bindgen`](windows-bindgen.md#generating-bit-field-accessors) for the
accessors they drive.

WinRT types use the `#[winrt]` namespace flavor and add runtime-class and
property syntax:

```text
#[winrt]
mod Robotics {
    #[Activatable(1)]
    class Robot {
        IRobot,
    }

    #[ExclusiveTo(Robot)]
    interface IRobot {
        fn Speak(&self, message: String);
        Name: String;
    }
}
```

The `crates/tests/libs/rdl/input` directory has a small, focused `.rdl` file for
each construct (`struct_nested.rdl`, `enum_flags.rdl`, `delegate.rdl`,
`interface_generic.rdl`, `union.rdl`, and so on) and doubles as a syntax
reference.

## How it fits with windows-bindgen

`windows-rdl` and `windows-bindgen` are complementary halves of one pipeline:

```text
C/C++ headers ──clang()──►  .rdl  ──reader()──►  .winmd  ──bindgen()──►  bindings.rs
 (windows-rdl)                       (windows-rdl)            (windows-bindgen)
```

When metadata already exists you skip straight to `windows-bindgen`. You reach
for `windows-rdl` when you need to *create* the metadata first — either by
hand-authoring RDL for types that have no metadata, or by lifting them out of a
C/C++ header. Two in-repo tools show both shapes:

- **`tool_webview`** runs the full path. WebView2 ships only a C/C++ header, so
  `clang()` produces `WebView2.rdl`, `reader()` compiles it to `WebView2.winmd`,
  and `windows_bindgen::bindgen` turns that into the bindings for the
  [`windows-webview`](windows-webview.md) crate.
  (`crates/tools/webview/src/main.rs`.)

- **`tool_reactor`** hand-authors the small set of COM interfaces and bootstrap
  functions that the WinUI/WinAppSDK metadata omits in
  `crates/tools/reactor/src/extras.rdl`, compiles it with `reader()` alongside
  the in-house Win32 winmd (`crates/libs/bindgen/default/Windows.Win32.winmd`) into `extras.winmd`, and feeds that
  to `windows_bindgen::bindgen` together with the standard metadata to generate the
  [`windows-reactor`](windows-reactor.md) bindings.
  (`crates/tools/reactor/src/main.rs`.) The reactor filter files
  (`base.txt`/`test.txt`) use the flat `Windows::Win32::<Name>` namespace.

In both cases the `reader` is given the standard metadata as an additional input
so that references from the authored RDL (Win32 handles, `IUnknown`, structs,
and the like) resolve against the canonical definitions.

---

## Internal documentation

The remainder of this page covers how the crate is built and maintained. It is
for contributors and is **not needed to use `windows-rdl`**.

### How it's built

The RDL grammar is parsed with `syn`/`quote`/`proc-macro2`, reusing Rust's own
tokenizer so the syntax stays Rust-shaped. The `reader` lowers that syntax tree
to ECMA-335 and emits a `.winmd` through
[`windows-metadata`](windows-metadata.md); the `writer` walks metadata read back
through the same crate to regenerate canonical RDL. The `clang` path uses
`clang-sys` to parse C/C++ translation units and project their declarations into
the RDL syntax tree, so the header and reader paths converge on the same
lowering code. A `formatter` module pretty-prints generated RDL.

### Testing

Verified by the dedicated test crates `test_rdl` (RDL ↔ winmd round-trips, with
the `input/*.rdl` fixtures) and `test_clang` (header → RDL goldens under
`expected/*.rdl`), plus the SDK-free `tool_roundtrip` validator (re-derives every
committed RDL corpus — WinRT, Win32, WDK — from its committed winmd and is
`git diff`-enforced in the `gen` workflow). Downstream, `test_bindgen`
covers the `.winmd` → Rust step that consumes this crate's output. Run
`cargo test -p test_rdl` and `cargo test -p test_clang`.

## The in-house Windows.Win32.winmd

Beyond compiling authored RDL, `windows-rdl` is the reader stage of an in-repo
pipeline that builds a **faithful, in-house** `Windows.Win32.winmd` directly from
the Windows SDK headers, replacing the reference `Windows.Win32.winmd` from
[win32metadata](https://github.com/microsoft/win32metadata) for the hand-authored
library crates. The [`windows-clang`](windows-clang.md) scraper (`tool_win32`) reads
the SDK headers, SAL annotations, and import libs and commits a browsable RDL
snapshot under `metadata/win32/`; this crate's `reader` compiles that snapshot into
the winmd. The scraper's design — the faithful-metadata principle, header
partitioning, the editorial-deviation ledger, and the canonical type remaps — is
documented in [`windows-clang`](windows-clang.md); this section covers the winmd
artifact those tools produce.

Every maintained crate that needs Win32 metadata resolves against the in-house
winmd — the minimal-binding library crates and `windows-reactor` directly, and the
monolithic `windows` / `windows-sys` crates via `tool_package`'s header-namespace
remap of the same in-house corpus. Nothing reads the win32metadata reference winmds.

### The winmd layout

- `crates/libs/bindgen/default/Windows.winmd` — **WinRT** metadata, rebuilt in-house
  by `tool_winrt`: it merges the per-contract winmds from the Windows SDK Contracts
  NuGet package, decompiles the result to the committed `metadata/winrt/*.rdl`
  snapshot (per namespace), and compiles that snapshot back into the winmd. There is
  no in-house WinRT *scraper* — the external inputs are the SDK's own reference
  winmds, merged rather than compiled from headers — but the RDL step makes the
  layout match Win32/WDK: the committed RDL is the reviewable source of truth.
- `crates/libs/bindgen/default/Windows.Win32.winmd` — **in-house** Win32 metadata,
  written by `tool_win32`. This is what `windows-bindgen`'s bundled `"default"`
  bindings resolve to.
- `crates/libs/bindgen/default/Windows.Wdk.winmd` — **in-house** WDK metadata,
  written by `tool_wdk`.

The committed RDL snapshot (`metadata/win32/*.rdl`) is the reviewable source of
truth — every scrape change is a readable `git diff`; the merged binary winmd is
git-ignored and rebuilt by `tool_win32`. The build is deterministic: the writer
stages tables in `BTreeMap`s and the module MVID is a fixed zero GUID, so
regeneration is byte-for-byte reproducible across platforms.

### Multi-arch merge

`tool_win32` scrapes each target architecture (x64, arm64, x86) into its own RDL
set, then `merge_arch_rdl` coalesces them into one winmd. A type identical across
every arch is emitted once (arch-neutral); a type that diverges is split into
per-arch copies tagged `#[arch(X86|X64|Arm64)]`. The collapse-or-split decision is
made structurally by the merge in [`windows-metadata`](windows-metadata.md) — see
its documentation for the signature that drives it. `merge_arch_rdl` itself is the
orchestration: it reads each arch's RDL, runs the merge, and writes the combined
result, using a per-process scratch directory cleaned up on every return path.

### Published crates: the header-namespace remap

`tool_package` generates the monolithic `windows` / `windows-sys` crates from the
**in-house** winmds (`crates/libs/bindgen/default/Windows.{Win32,Wdk}.winmd`), *not*
the win32metadata reference. The flat in-house metadata lives in a single
`Windows.Win32` namespace, but the published crates expose their API behind hundreds
of Cargo features, so `tool_package`'s `remap` step re-partitions the flat winmd into
header-stem namespaces (written to the git-ignored `target/package/`), using the
committed `metadata/win32` RDL directory as the routing signal, then runs
`windows-bindgen` over that partition. `tool_features` reuses the same remap so the
feature-search page reports the real header stems. The in-house WinRT `Windows.winmd`
is projected alongside the remapped Win32/WDK partition.

### Known limitations

- **Round-trip asymmetries.** A few RDL ↔ winmd forms don't round-trip
  byte-identically (raw identifiers, GUID constants, delegate ABI spelling). The
  winmd is correct either way; this is a cosmetic writer-side gap.

---

## Study: does RDL losslessly round-trip WinRT and Win32 metadata?

This section records a focused investigation into whether RDL — as produced and
consumed by `windows-rdl`, `windows-clang`, and `windows-metadata` — can describe
the full WinRT and Win32 metadata surface without loss, and where the validation
gaps are. It is a findings report intended to seed follow-up work, not a
description of shipped behavior.

### Question

Can RDL express every language/API/metadata feature present in today's WinRT and
Win32 metadata, and do we have proof that a `winmd → RDL → winmd` round-trip is
lossless for both?

### What already guarantees Win32/WDK

Win32 and WDK are lossless **by construction**: the in-house
`Windows.Win32.winmd` and `Windows.Wdk.winmd` are *produced from* RDL. The
committed `metadata/win32/*.rdl` and `metadata/wdk/*.rdl` snapshots are the
reviewable source of truth, and the `gen` CI workflow regenerates the winmds with
`tool_win32` / `tool_wdk` and fails on any `git diff`. So for Win32/WDK, "RDL can
express it" is not a hypothesis — RDL is the authoring format, and CI enforces the
round-trip continuously.

WinRT was different at the time of this study. `Windows.winmd` was built by
`tool_windows`, which **merged the Windows SDK Contracts winmds directly** — RDL was
never in that path — so nothing in the normal build proved that WinRT metadata
survived an RDL round-trip, and there was no committed WinRT RDL snapshot to review
or diff against. (This has since been addressed — see *Implemented design* below —
by inserting the RDL step into the renamed `tool_winrt` and committing
`metadata/winrt`.)

### Method

Three levels of evidence were gathered:

1. **Feature-vocabulary coverage** — the `test_rdl` fixtures (`crates/tests/libs/rdl/input/*.rdl`)
   already exercise the WinRT-relevant constructs: generic interfaces and
   delegates, required interfaces, runtime classes (static/activatable/composable
   factories), events, WinRT enums/flags, `ExclusiveTo`, and custom attribute
   arguments — alongside the Win32 constructs (callbacks, handles, unions,
   packed/aligned structs). `crates/tests/libs/rdl/tests/roundtrip.rs` compiles
   each fixture RDL → winmd → RDL and asserts the regenerated text matches the
   input exactly.

2. **Whole-surface projection** — a `winmd → RDL → winmd` projection was run over the
   real committed `Windows.winmd`. The winmd → RDL writer emitted **343 per-namespace
   `.rdl` files** covering the entire WinRT surface **with no unsupported-feature
   errors**, and the RDL reader compiled all of it back to a winmd, also without
   error. Spot-checking `Windows.Foundation.rdl` confirmed high fidelity:
   multi-parameter generic delegates (`AsyncOperationProgressHandler<TResult, TProgress>`),
   generic interfaces with required interfaces
   (`IAsyncActionWithProgress<TProgress>: IAsyncInfo`), `#[get]`/`#[set]`
   properties, `event` accessors, `ContractVersion` / `ApiContract` /
   `Activatable` / `Static` / `MarshalingBehavior` / `Threading` attributes, and
   GUIDs all survive.

3. **Idempotence proof** — a raw winmd byte comparison is *not* a valid
   losslessness test (winmd encoding legitimately varies in heap dedup and the
   resolved dependency closure, so the round-tripped winmd is a different size
   while being semantically equivalent). The valid check is at the canonical-RDL-text
   level. A double round-trip was run:

   ```text
   Windows.winmd ──writer──▶ RDL(A) ──reader──▶ winmd′ ──writer──▶ RDL(B)
   ```

   **All 343 namespace files of RDL(A) and RDL(B) were byte-identical.** The
   winmd ↔ RDL projection is therefore a stable fixed point across the whole WinRT
   surface.

### Findings

- **RDL is expressive enough for the current WinRT and Win32 surface.** The full
  WinRT winmd projects to RDL and back with zero unsupported-feature errors, and
  the projection is idempotent. Nothing in today's metadata trips an
  RDL-expressiveness limit.

- **The reader fails loud, never silent.** Unsupported forms are hard errors, not
  silent drops — e.g. `"type not supported"` and `"constant type not supported"`
  (`crates/libs/rdl/src/reader/mod.rs`), `"callback abi not supported"` /
  `"variadic parameters are not supported for callbacks"`
  (`reader/callback.rs`), `"function abi not supported"` (`reader/fn.rs`). If a
  future metadata construct exceeded RDL's vocabulary, the round-trip would fail
  rather than quietly lose data. None of these fired on the current corpus.

- **The writer drops nothing structurally.** `write_type_def` handles every
  `TypeCategory` (Struct, Enum, Interface, Class, Delegate/Callback, Attribute)
  with no catch-all drop, and the interface property/event shorthand
  (`get_`/`put_` → property, `add_`/`remove_` → event) is `consumed`-tracked with
  unconsumed methods falling through to full method form. The A == B idempotence
  result confirms every such collapse is reversible.

- **Idempotence is a fixed-point proof, with one honest caveat.** A == B proves
  the RDL round-trip is stable and that the reader preserves everything the writer
  emits. It does *not*, by itself, prove the *first* `winmd → RDL` step preserved
  100% of the original SDK winmd — anything dropped at that first boundary would be
  absent from both A and B and thus invisible to the diff. Closing that last gap
  requires a **semantic** comparison of the original SDK-merged winmd against
  winmd′ (table/row level), which was not built here. **This caveat proved real:**
  the WinRT `Char` primitive (`System.Char`) was written as `u16` — identical to
  `U16` — so the round-trip silently collapsed `Char → U16`. Idempotence never saw
  it (the writer mapped both to `u16` in *both* directions); it surfaced only when
  the RDL-rebuilt `Windows.winmd` broke the cppwinrt C++ header build
  (`CreateChar16Array` became `uint16_t[]`). Fixed by giving `Char` the dedicated
  RDL spelling `Char16` (mapped to `metadata::Type::Char` in the shared
  `emit.rs` writer and the RDL reader), with a `char16`
  round-trip fixture in `test_rdl`.

### Gaps to close

1. ~~**No committed WinRT RDL snapshot.**~~ **Closed.** `metadata/winrt/` now exists
   (per-namespace, analogous to `metadata/win32/`), written by `tool_winrt` and
   re-derivable by `tool_roundtrip`. As predicted by the idempotence result, the
   snapshot is diff-stable.

2. ~~**WinRT round-trip is not CI-enforced.**~~ **Closed.** `tool_winrt` and
   `tool_roundtrip` are in the `gen` workflow matrix (`.github/workflows/gen.yml`),
   so CI regenerates the WinRT RDL + winmd and fails on any `git diff`. The bitrotted
   `tool_rdl_roundtrip` has been removed and superseded by `tool_roundtrip`.

3. **First-boundary losslessness is only spot-checked for WinRT.** Partially
   addressed: the `Char → U16` collapse was found (via the cppwinrt build) and
   fixed with the `Char16` primitive, and a per-namespace comparison of the
   SDK-merged winmd against the RDL-rebuilt `metadata/winrt` now shows only the
   expected accessor param-name normalization. Still open as a *systematic* guard:
   a table/row-level semantic winmd differ would catch any future first-boundary
   loss automatically instead of relying on a downstream build to break.

4. **Documented cosmetic asymmetries** (raw identifiers, GUID constants, delegate
   ABI spelling; see *Known limitations* above) would produce spurious diffs in a
   naive committed-snapshot check unless the writer output is treated as the
   canonical form — which the idempotence result shows it already is for the WinRT
   surface, and which the committed `metadata/winrt` snapshot now formalizes.

### Suggested follow-up

- ~~Add a `metadata/winrt/` RDL snapshot~~ and ~~fix/replace `tool_rdl_roundtrip`~~
  are **done** (see *Implemented design* below).
- Add a semantic winmd-equivalence check (original SDK-merged `Windows.winmd`
  vs. the RDL-round-tripped winmd′, compared at the table/row level via
  `windows-metadata`) to prove first-boundary losslessness, closing the one gap
  idempotence cannot.

---

## Implemented design: unified metadata validation

This section describes the model — now implemented — for validating all three
metadata families (WinRT, Win32, WDK) so that **RDL is the reviewable source of truth
and the `.winmd` is a derived artifact**, with `git diff` enforcing the round-trip.

### The one structural asymmetry to absorb

The winmd is *partition-lossy*, but only for the header-scraped families:

- **WinRT** partitions RDL by **namespace**, which is stored in the winmd. So
  `Windows.winmd → metadata/winrt` is fully reconstructable from the winmd alone —
  a true `winmd → RDL` round-trip.
- **Win32 / WDK** partition RDL by **defining header**, which is *not* in the flat
  `Windows.Win32` winmd. `tool_win32` / `tool_wdk` recover it during the scrape and
  bake it into `metadata/win32` / `metadata/wdk`. That file layout cannot be
  rebuilt from the winmd alone; the RDL corpus owns it.

A consistent design therefore treats the committed RDL as authoritative for
partitioning, and validates *content* round-tripping against it.

### Current vs. target state

| Family | External source | Committed RDL | Committed winmd | RDL in winmd build path? |
|---|---|---|---|---|
| Win32 | SDK headers | `metadata/win32` (per header) | `Windows.Win32.winmd` | Yes |
| WDK | WDK headers | `metadata/wdk` (per header) | `Windows.Wdk.winmd` | Yes |
| WinRT | SDK Contracts winmds | `metadata/winrt` (per namespace) | `Windows.winmd` | Yes — merge → RDL → winmd |

### Part 1 — WinRT matches the other generators (`tool_windows` → `tool_winrt`)

The RDL step is inserted so WinRT's generator is structurally identical to
`tool_win32` / `tool_wdk`:

```text
SDK Contracts winmds ──merge──▶ merged winmd (in target/)
                     ──writer, split by namespace──▶ metadata/winrt/*.rdl   (committed)
                     ──reader──▶ crates/libs/bindgen/default/Windows.winmd  (committed, derived)
```

The `gen` job's `git diff` on *both* `metadata/winrt` and `Windows.winmd`
validates the `RDL → winmd` (reader) direction end-to-end. One-time effects:
`Windows.winmd` re-bases to its RDL-rebuilt form (a reviewable diff), and — because
the RDL accessor shorthand does not record the original accessor parameter name (see
*Known limitations*) — the reader synthesizes canonical names (`value` for property
setters, `handler`/`token` for event add/remove). That normalizes those parameter
names in the generated `windows` crate (a one-time cosmetic regen via `tool_package`;
ABI, signatures, types, and vtable order are unchanged). The crate was renamed to
`tool_winrt` to align it with `tool_win32` / `tool_wdk` (mechanical: `Cargo.toml`,
the `gen` matrix, and doc comments).

### Part 2 — a fast, SDK-free validator (`tool_roundtrip`)

`tool_roundtrip` depends only on `windows-rdl` (no libclang, no NuGet); it
re-derives every committed RDL corpus *from its committed winmd* and relies on
`git diff`:

- **WinRT:** `writer(Windows.winmd).filters(["Windows", "!Windows.Win32", "!Windows.Wdk"]).split(true) → metadata/winrt`.
  Pure `winmd → RDL`; the namespace layout comes from the winmd. Validates the
  *writer* direction independently of the Contracts merge.
- **Win32 / WDK:** the winmd cannot place types in header files, so the tool reads
  the committed corpus for the name → header-stem map (`windows_rdl::item_names`,
  exactly as `merge_arch_rdl` does), restores the hand-authored `metadata/metadata.rdl`
  seed (which lives outside the corpus dir; WDK has no seed — it resolves attributes
  from the Win32 reference winmd), then `writer(winmd).partition(map) → metadata/{win32,wdk}`.
  Validates that the winmd's *content* round-trips to the committed RDL under the committed layout.

Because the round-trip output is byte-identical to what the generator wrote (that
identity *is* the property under test), there is no real two-writer conflict: the
generator is the slow provenance guard; `tool_roundtrip` is the fast consistency
guard. Both the `split` and `partition` writer paths already clear stale `*.rdl`
before writing, so repeated runs are idempotent.

### Part 3 — CI wiring

`winrt` and `roundtrip` are in the `.github/workflows/gen.yml` tool matrix, reusing
the existing `cargo run -p tool_<tool>` + `git diff --exit-code` pattern. (`gen.yml`
is hand-maintained, not generated by `tool_yml`.)

### What each arrow proves

| Arrow | Validated by | Mechanism |
|---|---|---|
| external → RDL → winmd (all three) | generator (`tool_win32` / `tool_wdk` / `tool_winrt`) | `git diff` on RDL + winmd |
| winmd → RDL (WinRT) | `tool_roundtrip` | `git diff` (namespace partition, from winmd) |
| winmd → RDL content (Win32 / WDK) | `tool_roundtrip` | `git diff` (layout from committed corpus + seed) |

Together this makes `RDL ↔ winmd` a `git diff`-enforced bijection for WinRT and a
`git diff`-enforced content round-trip for Win32 / WDK, whose file layout is by
definition owned by the RDL corpus.

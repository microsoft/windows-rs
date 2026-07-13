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
`expected/*.rdl`), plus the `rdl_roundtrip` tool. Downstream, `test_bindgen`
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
  by `tool_windows` by merging the per-contract winmds from the Windows SDK Contracts
  NuGet package (there is no in-house WinRT *scraper* — the inputs are the SDK's own
  reference winmds, merged rather than compiled from headers).
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

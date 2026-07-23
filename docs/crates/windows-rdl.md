# windows-rdl

> A parser for RDL (Rust Definition Language) and an ECMA-335 metadata generator.

- [crates.io](https://crates.io/crates/windows-rdl)
- [docs.rs](https://docs.rs/windows-rdl)
- [Getting started](../../crates/libs/rdl/readme.md)
- [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/rdl)

`windows-rdl` is the front of the metadata authoring pipeline. It parses RDL
(Rust Definition Language), a small Rust-like syntax for Windows APIs. It emits
ECMA-335 `.winmd` metadata for [`windows-bindgen`](windows-bindgen.md). It also
writes canonical RDL from `.winmd` files.

Use `windows-rdl` when an API needs metadata first. You can write RDL by hand.
You can also generate RDL from C or C++ headers with
[`windows-clang`](windows-clang.md). Then pass the `.winmd` output to
`windows-bindgen`.

## Getting started

Add `windows-rdl` as a build dependency. It usually runs from a codegen tool or
`build.rs`. It is not a runtime dependency.

```toml
[build-dependencies]
windows-rdl = "0.0.0"
```

The crate exposes three builders:

- `reader()` compiles RDL source to `.winmd` metadata.
- `writer()` writes canonical RDL source from `.winmd` metadata.
- `clang()` writes RDL source from C or C++ headers.

### RDL to winmd, and back

Use `reader` to compile `.rdl` into `.winmd`. Use `writer` to regenerate
canonical `.rdl` from `.winmd`.

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

RDL can reference types it does not define. Examples include `HRESULT` and
`Windows::Win32::System::Com::IUnknown`. Add the standard metadata as another
`reader` input so those references resolve. The bundled metadata lives in
`crates/libs/bindgen/default`.

```rust,no_run
windows_rdl::reader()
    .input("example.rdl")
    .input("crates/libs/bindgen/default")
    .output("example.winmd")
    .write()
    .unwrap();
```

### C/C++ headers to RDL

Use [`windows-clang`](windows-clang.md) when an API ships only a C or C++ header.
The `clang()` builder parses the header into RDL. Then `reader()` compiles that
RDL to metadata.

Each header is parsed as its own translation unit. The scraper emits only that
header's top-level declarations. It does not emit declarations from `#include`
files. List each header you need as a separate input.

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

RDL looks like a small Rust module. A top-level `mod` is a metadata namespace.
Tag it `#[winrt]` or `#[win32]` to select the type system. Attributes map to
metadata attributes. Item keywords mirror metadata kinds.

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

Most attributes name a metadata attribute type directly. Some attributes use
short pseudo-attribute names. The reader expands those names to full metadata
attributes. See `PSEUDO_ATTRS` in `windows-rdl`.

Struct bit fields use their own syntax. A run of bit fields packed into one
backing integer is written as a C-like block on that field. Each member uses
`Name: width`. Anonymous padding uses `_: width`.

```text
struct D3D11_VIDEO_PROCESSOR_COLOR_SPACE {
    _bitfield: u32 {
        Usage: 1,
        RGB_Range: 1,
        YCbCr_Matrix: 1,
        YCbCr_xvYCC: 1,
        Nominal_Range: 2,
        Reserved: 26,
    },
}
```

Member offsets are implicit. Each offset is the total width of earlier members,
including padding. The reader writes one
`Windows.Win32.Metadata.NativeBitfieldAttribute(name, offset, width)` custom
attribute per named member. The writer renders it back to block form.

See [`windows-clang`](windows-clang.md#bit-field-member-scraping) for how the
scraper emits bit fields. See
[`windows-bindgen`](windows-bindgen.md#generating-bit-field-accessors) for the
accessors they drive.

WinRT types use the `#[winrt]` namespace flavor. They also add runtime-class and
property syntax.

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

The `crates/tests/libs/rdl/input` directory has focused `.rdl` files for syntax
examples. It covers structs, flags, delegates, generic interfaces, unions, and
more.

## How it fits with windows-bindgen

`windows-rdl` and `windows-bindgen` are two stages in one pipeline.

```text
C/C++ headers -- clang() --> .rdl -- reader() --> .winmd -- bindgen() --> bindings.rs
 (windows-clang)             (windows-rdl)        (windows-bindgen)
```

Skip `windows-rdl` when metadata already exists. Use it when you need to create
metadata first. You can write RDL by hand or lift declarations from a header.

Two in-repo tools show both uses:

- `tool_webview` runs the full path. WebView2 ships only a C/C++ header.
  `clang()` produces `WebView2.rdl`. `reader()` compiles it to `WebView2.winmd`.
  Then `windows_bindgen::bindgen` generates bindings for
  [`windows-webview`](windows-webview.md).
- `tool_reactor` hand-writes COM interfaces and bootstrap functions in
  `crates/tools/reactor/src/extras.rdl`. These declarations fill gaps in the
  WinUI and Windows App SDK metadata. The tool compiles them with the standard
  Win32 winmd into `extras.winmd`. Then it feeds that winmd to
  `windows_bindgen::bindgen` for [`windows-reactor`](windows-reactor.md).

In both tools, `reader` also gets the standard metadata as input. That lets RDL
references resolve against the standard definitions.

---

## Internal documentation

The rest of this page covers how the crate is built and maintained. It is for
contributors and is not needed to use `windows-rdl`.

### How it's built

The RDL grammar uses `syn`, `quote`, and `proc-macro2`. It reuses Rust's tokenizer
so the syntax stays Rust-shaped. The `reader` lowers the syntax tree to ECMA-335
and emits `.winmd` through [`windows-metadata`](windows-metadata.md). The `writer`
reads metadata through the same crate and writes canonical RDL.

The `clang` path uses `clang-sys` to parse C or C++ translation units. It projects
the declarations into the RDL syntax tree. The header path and hand-authored RDL
path share the same lowering code. The `formatter` module pretty-prints generated
RDL.

### Testing

Dedicated test crates cover the crate:

- `test_rdl` covers RDL to winmd round trips with `input/*.rdl` fixtures.
- `test_clang` covers header to RDL output with `expected/*.rdl` goldens.
- `tool_roundtrip` re-derives committed RDL files from committed winmd files. The
  `gen` workflow enforces a clean `git diff`.
- `test_bindgen` covers the `.winmd` to Rust step that consumes this crate's
  output.

Run targeted tests with:

```sh
cargo test -p test_rdl
cargo test -p test_clang
```

### Default metadata files

`windows-rdl` builds the default metadata files used by the in-repo generators and
library crates.

| File | Source | Writer |
|------|--------|--------|
| `crates/libs/bindgen/default/Windows.winmd` | SDK Contracts winmds, merged and written through RDL | `tool_winrt` |
| `crates/libs/bindgen/default/Windows.Win32.winmd` | Windows SDK headers scraped to RDL | `tool_win32` |
| `crates/libs/bindgen/default/Windows.Wdk.winmd` | WDK headers scraped to RDL | `tool_wdk` |

The committed RDL files are the reviewable source for these metadata files:

- `metadata/winrt/*.rdl` is partitioned by namespace.
- `metadata/win32/*.rdl` is partitioned by defining header.
- `metadata/wdk/*.rdl` is partitioned by defining header.

The binary winmd files are derived artifacts. Generation is deterministic. The
metadata writer stages tables in `BTreeMap`s and uses a fixed zero GUID for the
module MVID.

Every maintained crate that needs Win32 metadata resolves against the in-repo
`Windows.Win32.winmd`. Minimal-binding crates and `windows-reactor` use it
directly. The `windows` and `windows-sys` crates use it through `tool_package`.

### Multi-arch merge

`tool_win32` scrapes x64, arm64, and x86 into separate RDL sets. Then
`merge_arch_rdl` combines them into one winmd. A type with the same shape on every
architecture is emitted once. A type that differs by architecture is split into
per-architecture copies tagged `#[arch(X86|X64|Arm64)]`.

The merge compares type structure through [`windows-metadata`](windows-metadata.md).
`merge_arch_rdl` handles orchestration. It reads each architecture's RDL, runs the
merge, and writes the combined output.

### Published crates and namespace remap

The in-repo Win32 and WDK metadata lives in flat namespaces. Published `windows`
and `windows-sys` APIs are partitioned behind many Cargo features.

`tool_package` remaps the flat metadata into header-stem namespaces under
`target/package/`. It uses the committed `metadata/win32` RDL directory as the
routing signal. Then it runs `windows-bindgen` over that partition. `tool_features`
uses the same remap so feature search reports the same header stems.

The in-repo WinRT `Windows.winmd` is projected with the remapped Win32 and WDK
metadata.

### Round-trip rules

RDL is the reviewable source for WinRT, Win32, and WDK metadata. The `.winmd` files
are derived artifacts. The `gen` workflow runs the generators and `tool_roundtrip`.
It fails when regeneration changes tracked files.

| Family | External source | RDL layout | Winmd build path |
|--------|-----------------|------------|------------------|
| WinRT | SDK Contracts winmds | `metadata/winrt`, per namespace | merge SDK winmds, write RDL, read RDL to winmd |
| Win32 | SDK headers | `metadata/win32`, per header | scrape headers to RDL, merge architectures, read RDL to winmd |
| WDK | WDK headers | `metadata/wdk`, per header | scrape headers to RDL, merge architectures, read RDL to winmd |

`tool_roundtrip` validates the reverse direction:

- WinRT uses `writer(Windows.winmd).split(true)` to write `metadata/winrt`.
- Win32 and WDK cannot recover header files from flat winmd alone. The tool reads
  the committed RDL layout to map type names back to header stems. Then it writes
  `metadata/win32` or `metadata/wdk` with `writer(winmd).partition(map)`.

### Current normalization rules

Some metadata forms have canonical RDL spellings. These rules are intentional.
They keep generated RDL stable.

| Form | RDL spelling | Reason |
|------|--------------|--------|
| WinRT `System.Char` | `Char16` | It stays distinct from `u16` and maps to `metadata::Type::Char`. |
| Property setter parameter names | `value` | RDL property shorthand does not store the original parameter name. |
| Event add parameter names | `handler` | RDL event shorthand stores the event shape, not the accessor parameter name. |
| Event remove parameter names | `token` | The remove accessor takes an event token. |
| Property and event accessors | Property or event shorthand | The writer tracks consumed `get_`, `put_`, `add_`, and `remove_` methods. |
| Unconsumed interface methods | Full method form | Methods that are not part of shorthand stay explicit. |
| Raw identifiers, GUID constants, and delegate ABI spelling | Canonical writer output | Text can differ while metadata stays equivalent. |

The reader rejects unsupported forms with errors. It does not silently drop them.
Examples include unsupported types, constants, callback ABIs, variadic callback
parameters, and function ABIs.

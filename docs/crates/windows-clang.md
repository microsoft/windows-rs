# windows-clang

> Generates RDL from C/C++ headers using libclang.

- 📦 [crates.io](https://crates.io/crates/windows-clang)
- 📖 [docs.rs](https://docs.rs/windows-clang)
- 🚀 [Getting started](../../crates/libs/clang/readme.md)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/clang)

`windows-clang` is the header front end of the Windows metadata pipeline:

```text
headers --(windows-clang)--> .rdl --(windows-rdl)--> .winmd
        --(windows-bindgen)--> bindings.rs
```

It parses C/C++ declarations, SAL annotations, calling conventions, constants, layouts, COM
interfaces, and import libraries. It emits RDL that
[`windows-rdl`](windows-rdl.md) compiles into ECMA-335 metadata.

## Getting started

Add `windows-clang` as a build dependency:

```toml
[build-dependencies]
windows-clang = "0.100"
```

Configure the parser and write one RDL file per input header:

```rust,no_run
windows_clang::clang()
    .args(["-x", "c++", "--target=x86_64-pc-windows-msvc"])
    .input("Example.h")
    .reference_default()
    .output("rdl")
    .namespace("Example")
    .library("Example.dll")
    .write_by_header()
    .unwrap();
```

Only declarations defined by an input header are emitted. Included declarations are available for
type resolution but are not copied into that header's output. Add each header whose declarations
you want to emit.

Use `input_text` or `input_texts` for source already in memory. Input, reference, resolution,
import-library, and output paths accept strings, `Path`, and `PathBuf`.

### References and resolution

`reference_default()` adds the bundled WinRT and Win32 metadata for external type resolution.
Custom metadata can be supplied with `reference`, `references`, `reference_bytes`, and
`reference_byte_sets`.

Resolution metadata classifies `ABI::Windows::*` declarations without excluding them as existing
definitions. `resolution_default()` adds the bundled WinRT metadata. Custom inputs use
`resolution_input`, `resolution_inputs`, `resolution_bytes`, or `resolution_byte_sets`.

### Output modes

| Terminal | Output |
| --- | --- |
| `write()` | One formatted RDL file for a namespaced scrape. |
| `write_by_header()` | One flat RDL partition per defining header. |
| `scrape(ScrapePlan)` | Per-architecture scrapes followed by architecture-aware RDL and winmd merging. |

`write()` is used for small namespaced metadata such as WebView2. `write_by_header()` and
`scrape()` are used by the Win32 and WDK pipeline.

## Consumers

- `tool_win32` scrapes Windows SDK and WDK headers into the committed Win32 RDL and winmd.
- `tool_webview` scrapes WebView2 headers into namespaced RDL.
- `test_clang` contains golden header-to-RDL fixtures.

---

## Internal documentation

The rest of this page describes the implementation and maintenance workflow.

### Crate layering

```text
windows-metadata
  +- windows-rdl
       +- windows-clang
            +- tool_win32 / tool_webview
```

`windows-clang` reuses `windows-rdl` for RDL emission, formatting, import-library parsing, errors,
and file handling. `windows-rdl` does not depend on libclang.

The `clang()` builder owns parser configuration. The `scrape()` terminal adds target-specific
arguments, runs the configured scrape for each architecture, and merges the resulting metadata.
SDK and WDK package versions, include paths, header lists, and tool-specific preambles remain in
their consuming tools.

### Code organization

- `cx` wraps `clang-sys` cursors and translation units.
- `canon` applies the header-to-metadata type rules.
- `annotation` decodes SAL and IDL annotations.
- `r#enum`, `r#struct`, `r#const`, `r#fn`, `callback`, `interface`, `typedef`, and `field` parse
  declaration kinds.
- `collector` and `item` collect and emit RDL.
- `scope` handles reachability and reference maps.
- `naming` handles tags and nested type names.
- `macros` evaluates object-like macros.
- `provision` locates the pinned libclang and NuGet packages.

Both output paths share one translation-unit parse. The parsed input owns the libclang library,
index, and translation units for the duration of emission.

### Source model

The generated metadata follows declarations expressed by the headers. It does not add curated
handle lifetimes, documentation mappings, struct-size conventions, or synthetic grouping enums.

The scraper preserves:

- SAL and IDL direction, optionality, buffer sizing, retval, and interface-selection annotations;
- `uuid`, `noreturn`, alignment, `dllimport`, and deprecation attributes;
- calling conventions, packing, unions, scoped enums, bit fields, and typedefs;
- explicit constant casts and C integer literal types;
- `DEFINE_ENUM_FLAG_OPERATORS` as a flags-enum signal;
- symbol-to-DLL mappings recovered from import libraries.

Some C portability spellings are canonicalized for metadata consumers. Examples include fixed-width
integer typedefs, pointer-sized integer typedefs, Windows string wrappers, pointer aliases in
parameters, GUID aliases, and Direct2D compatibility aliases. These rules live in `canon.rs`.
Parameter SAL can change pointer constness because it expresses the function's read/write contract.

### Bit-field member scraping

The winmd format cannot represent mixed pointer constness or C bit-field syntax directly. Mixed
pointer chains use the outermost direction, and bit-field runs become integer backing fields with
`NativeBitfieldAttribute` entries for logical members.

### Win32 and WDK generation

`tool_win32` runs these stages:

1. Scrape the Windows SDK `um` and `shared` headers for x64, arm64, and x86.
2. Merge the per-architecture outputs and write `metadata/win32/*.rdl`.
3. Scrape the WDK `km` headers against the user-mode metadata and write `metadata/wdk/*.rdl`.
4. Merge user-mode and WDK metadata, unioning compatible same-named enums.
5. Write `crates/libs/default/Windows.Win32.winmd`.

The header lists and import-library order are defined by `crates/tools/win32`. A function without a
resolved exporting library is omitted when `drop_lib_less` is enabled. The generated metadata is
partitioned by defining header rather than by a curated API namespace.

Per-header generation canonicalizes declarations repeated across translation units before writing
RDL. Identical items keep the last sorted partition owner to match package remapping, a complete
definition suppresses an opaque forward declaration, and a concrete type suppresses a same-named
typedef. Non-equivalent same-named declarations remain visible and fail RDL validation instead of
producing duplicate metadata rows.

`LIBRARY_OVERRIDES` corrects confirmed SDK import-library defects after the normal first-wins
resolution pass. Each entry records the SDK value and its replacement. Generation fails if an SDK
update changes the original value, prompting removal or revalidation of the override.

WinRT projection types referenced from native interop headers are resolved against `Windows.winmd`.
True WinRT types remain cross-metadata references. Native COM types declared under
`ABI::Windows::*` are emitted as Win32 declarations.

### Provisioning

The tooling pins libclang and obtains it from the `libclang.runtime.win-*` NuGet packages unless
`LIBCLANG_PATH` is already set. The matching clang resource headers are cached under
`target/windows-clang`.

`ensure_libclang()` configures the process before libclang is loaded. `libclang_dir()` resolves the
directory without changing the environment, which is useful for CI and tests.
`assert_libclang_version()` rejects a mismatched library.

### Known limitations

- Coverage is limited to the headers listed by each consuming tool.
- Types declared in a listed header but unreachable from emitted declarations are omitted.
- The flat `Windows.Win32` namespace cannot preserve distinct declarations that share a name only
  because curated metadata placed them in different namespaces.
- Attributes that cannot be inferred from headers, including handle cleanup and last-error policy,
  are not emitted.
- Struct-field buffer annotations and callback return annotations do not use the full parameter SAL
  path.
- `__fastcall` callbacks are recorded in metadata but project as `extern "system"` because stable
  Rust does not support the corresponding function-pointer ABI.

### Testing

`test_clang` contains golden fixtures for namespaced and per-header output. It covers annotations,
constants, interfaces, architecture differences, bit fields, layout, header partitioning, and type
canonicalization.

Run:

```powershell
cargo test -p test_clang
```

CI sets `LIBCLANG_PATH` with `cargo run -q -p tool_clang -- path` so the tests use the pinned
libclang.

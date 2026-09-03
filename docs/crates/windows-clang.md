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

Applications and libraries should start with a focused crate or `windows-bindgen` using existing
metadata. Binary applications can use [`windows`](windows.md) or
[`windows-sys`](windows-sys.md) when a broad pre-generated surface is more convenient. Use
`windows-clang` when building a metadata tool for an API available only as C or C++ headers. It
does not generate Rust code directly.

Start with the crate [README](../../crates/libs/clang/readme.md) for setup. A usable installation
also needs libclang. `ensure_libclang` locates or downloads the pinned runtime unless
`LIBCLANG_PATH` is set, and `assert_libclang_version` checks that the loaded version matches.

## First workflow: turn a header into bindings

1. Provision the pinned libclang before the first parse.
2. Add every header that owns declarations you need and pass the same language, include, define,
   extension, and target arguments that the header expects.
3. Emit one namespaced RDL file with `write`.
4. Compile that RDL with [`windows-rdl`](windows-rdl.md).
5. Generate Rust from the resulting winmd with [`windows-bindgen`](windows-bindgen.md).

`tool_webview` is the concrete model. It obtains a pinned WebView2 NuGet package, supplies the
header search path and MSVC target, writes `target/webview/WebView2.rdl`, compiles
`WebView2.winmd`, and runs bindgen from a checked-in command file.

Each header is a translation unit. Included declarations are available while parsing but are not
emitted as declarations owned by the input header. List each owning header explicitly. Use
`input_text` or `input_texts` for source already in memory.

## Input and output model

| Builder input | Purpose |
| --- | --- |
| `input`, `inputs` | Header files or directories containing `.h` files. |
| `arg`, `args`, `target` | libclang language, include, define, extension, and target options. |
| `reference*` | Existing metadata used for type resolution and duplicate suppression. |
| `resolution*` | Metadata used only to classify `ABI::Windows::*` projection declarations. |
| `import_library` | COFF `.lib` symbols used to recover function -> DLL mappings. |
| `library` | One fallback DLL name for imported functions. |

`reference_default` adds the bundled WinRT and Win32 metadata. `resolution_default` adds only the
bundled WinRT metadata for classification. File and byte variants are available for both roles; do
not substitute a resolution input for a reference because their exclusion behavior differs.

| Terminal | Output and use |
| --- | --- |
| `write` | One formatted RDL file in the configured namespace. |
| `write_by_header` | Lowercase `<header-stem>.rdl` files in one flat root namespace. |
| `scrape` | Multi-architecture partitions plus an architecture-aware merged winmd. |

Use `write` for one component-owned namespace. Use `write_by_header` when the defining header is
the partition key. `ScrapePlan` adds architecture triples, bitmasks, scratch outputs, references,
an optional hand-authored seed, and parallel execution; it is intended for SDK-scale generators.

## Common tasks

- Use `filter` and `filters` for normalized header path suffixes.
- Use `symbol` and `symbols` when only named free functions should be roots.
- Use `scope` or `scope_header` to choose roots for a per-header reachability sweep.
- Use `exclude_header` to remove a partition before that sweep.
- Load per-DLL import libraries before umbrella libraries so first-wins symbol resolution keeps the
  real DLL. `libraries` supplies reviewed symbol overrides.
- Enable `drop_lib_less` only when import-library coverage is available; otherwise valid functions
  without mappings are discarded.

## Pitfalls

- The parser sees the preprocessed declaration selected by your arguments. Wrong defines or target
  settings can change layouts, aliases, and exported names without a parser error.
- The winmd format cannot express every C type detail. Mixed pointer constness and bit fields are
  normalized for metadata consumers.
- Header references do not provide DLL ownership. Supply a fallback DLL or import libraries before
  expecting callable free functions in generated bindings.
- A single-architecture scrape cannot describe cross-architecture layout differences. Use
  `ScrapePlan` when one winmd must support X86, X64, and Arm64.
- Header scraping recovers source declarations, not curated lifetime, last-error, or documentation
  policy.

## Samples and consumers

- `tool_webview` is the starting example for a namespaced component scrape.
- `tool_win32` and its WDK stage use per-header, multi-architecture generation.
- `test_clang` contains small header-to-RDL fixtures for annotations, constants, interfaces,
  layouts, bit fields, and canonicalization.

---

## Internal documentation

The remainder of this page covers how the crate is built and maintained. It is for contributors and
is **not needed to use `windows-clang`**.

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

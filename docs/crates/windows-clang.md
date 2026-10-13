# windows-clang

> Generates RDL from C/C++ headers using libclang.

- [crates.io](https://crates.io/crates/windows-clang)
- [docs.rs](https://docs.rs/windows-clang)
- [Getting started](../../crates/libs/clang/readme.md)
- [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/clang)

`windows-clang` is the header-facing stage of the Windows metadata pipeline:

```text
headers -> windows-clang -> RDL -> windows-rdl -> WinMD
```

It extracts declarations and source annotations from C and C++ headers. Compile the resulting
RDL with [`windows-rdl`](windows-rdl.md), then use [`windows-bindgen`](windows-bindgen.md) to
generate Rust bindings.

## Setup

The [README](../../crates/libs/clang/readme.md) shows the Cargo dependency. Install a compatible
libclang and set `LIBCLANG_PATH` to its directory if it is not discoverable on your system.
The crate loads libclang at runtime; it does not install it or the headers you want to process.

Supply compiler arguments for the language, target architecture, include directories, and
preprocessor definitions your headers require. Metadata references do not supply C/C++ headers
or compiler include paths.

## Generate RDL

Configure the builder, then call `write()`:

```rust,no_run
windows_clang::clang()
    .input("Example.h")
    .args(["-x", "c++", "--target=x86_64-pc-windows-msvc"])
    .reference_default()
    .namespace("Example")
    .library("example.dll")
    .output("Example.rdl")
    .write()
    .unwrap();
```

At least one input, a namespace, and an output path are required. Paths are relative to the
generator's working directory.

| Builder method | Purpose |
| --- | --- |
| `input(path)` / `inputs(paths)` | Read header files as separate translation units. |
| `input_text(source)` | Add an in-memory translation unit. |
| `args(arguments)` | Pass compiler arguments to libclang. |
| `filter(suffix)` | Include declarations from included headers matching a path suffix. |
| `symbols(names)` | Restrict emitted free functions to the supplied names. |
| `reference(path)` | Resolve existing declarations from a WinMD file. |
| `reference_default()` | Use the bundled Windows metadata as references. |
| `namespace(name)` | Set the output RDL namespace. |
| `library(name)` | Set the DLL name attached to emitted functions. |
| `output(path)` | Set the output RDL file. |

Declarations from input headers are output roots. Included headers can supply dependencies without
having all their declarations emitted; use `filter` to include their declarations as roots too.
This is a header-path filter, not a function-name filter.

References let generated RDL use existing types without redefining them. Use `reference_default`
when your headers depend on Windows APIs and `reference` for your own metadata.

The [builder reference](https://docs.rs/windows-clang/latest/windows_clang/struct.Clang.html)
documents all methods.

## Inspect extraction results

Use `Input`, `extract`, and `EmitOptions` directly when a generator needs to inspect declarations,
assign different DLLs to functions, or emit separate header partitions.

```rust,no_run
let input = windows_clang::Input::new(
    "Example.h",
    std::fs::read_to_string("Example.h").unwrap(),
);
let snapshot = windows_clang::extract(
    [input],
    &["-x", "c++", "--target=x86_64-pc-windows-msvc"],
)
.unwrap();
```

The resulting `Snapshot` owns translation-unit-local facts and constants. `facts`, `constants`,
`unsupported`, and `dump` expose the extraction result for diagnostics and validation.

`Input::new(name, source)` treats `name` as a root. The following methods control which included
headers contribute output roots:

| Method | Selects |
| --- | --- |
| `with_roots(paths)` | Individual header paths. |
| `with_root_dirs(paths)` | Headers under directory prefixes. |
| `with_root_suffixes(suffixes)` | Headers matching path suffixes. |
| `with_excluded_roots(paths)` | Headers excluded from output ownership. |

## Configure emission

Use `Snapshot::emit(namespace)` for one namespace, `emit_with_library` to attach one DLL to all
functions, or `emit_with_options` for generator policy. `emit_by_header_with_options` returns a
map of defining-header names to RDL partitions.

`EmitOptions` controls:

| Option | Purpose |
| --- | --- |
| `namespace` | RDL namespace. |
| `library` / `libraries` | Default or per-function DLL mappings. |
| `references` | External types and enum members available to the projection. |
| `excluded_types` | Types excluded from root selection. |
| `excluded_functions` | Functions omitted from the local output. |
| `excluded_constants` | Constants omitted from the local output. |
| `functions` | Optional free-function allowlist. |

References are explicit `TypeReference` values classified as `Type`, `Interface`, or `Enum`.
Referenced enum member names let an overlay emit an enum only when it adds members to the base.
`MetadataReferences` builds that map from WinMD files and records existing type, function, and
constant names for exclusion. `apply_reference_exclusions` excludes only types with an
unambiguous external reference, while `apply_exclusions` is available for overlays that must omit
every item from a known base.

Exclusions filter root declarations; they do not establish that a dependency has an external
definition. Supply references for external types; required local dependencies are still emitted.

The caller owns DLL selection, architecture merging, and RDL-to-WinMD compilation. Extraction
does not discover import libraries or combine target architectures automatically.

## Known limits

- RDL cannot represent mixed raw pointer-chain mutability. If any level in a raw pointer run is
  const, every level in that run is emitted as const.
- Coverage is limited to declarations reachable from configured roots.
- Conflicting declarations can fail emission as ambiguous; extraction is not a general C++ binding
  generator.
- SAL projection supports only a subset of annotations and size relationships.
- Non-finite floating-point constants and interface-valued constants cannot be emitted.
- Header extraction cannot infer curated handle cleanup, last-error, or documentation policy.
- Stable Rust has no function-pointer ABI corresponding to every native calling convention.

## Contributing

Run `cargo test -p windows-clang` and `cargo test -p test_clang`. Header fixtures and expected RDL
live in `crates/tests/libs/clang`; tests regenerate the expected output, and CI rejects output
changes that are not committed. After changing extraction or emission, run the affected `tool-*`
generators and review the output diffs.

Repository tooling provisions its pinned libclang with `cargo run -q -p tool-clang -- path`.
Set `LIBCLANG_PATH` to the reported directory before running tests.

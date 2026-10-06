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

It extracts declarations and source annotations into immutable facts, then plans and emits RDL
from the completed fact graph. It does not generate Rust bindings or provision libclang.

## High-level generation

`clang()` returns a `Clang` builder for the common case of extracting one set of headers and
writing RDL directly:

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

The builder is an adapter over the extraction and emission APIs described below. `input` and
`input_text` construct `Input` values, `filter` selects included headers by path suffix,
`reference_default` supplies the Windows metadata references, and `write` calls `extract` and
emits with `EmitOptions`. It does not have a separate parser or projection path.

Use the lower-level API when a generator must inspect facts, compare architectures, merge
snapshots, assign libraries per function, or control output promotion.

## Extraction

Each `Input` contains:

| Field | Purpose |
| --- | --- |
| `name` | Translation-unit name used for diagnostics and default ownership. |
| `source` | C or C++ source passed to libclang. |
| `roots` | Header paths whose declarations may become output roots. |
| `root_dirs` | Directory prefixes whose declarations may become output roots. |
| `root_suffixes` | Header path suffixes whose declarations may become output roots. |
| `excluded_roots` | Header paths excluded from output ownership. |

`Input::new(name, source)` treats `name` as a root. `with_roots`, `with_root_dirs`,
`with_root_suffixes`, and `with_excluded_roots` extend that policy. The caller supplies all
compiler arguments to `extract`, including the language, target, include paths, defines, forced
includes, and extensions.

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

## Emission

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
every item from a known base. Both the high-level builder and repository generators use this
indexing path.

Exclusions filter root declarations; they do not establish that a dependency has an external
definition. Before collecting dependencies, the planner resolves unambiguous external aliases.
Dependencies with a direct reference or a resolved external alias use that definition. Other
dependencies pass through the normal canonicalization and pointer-boundary rules, even when they
underlie an excluded typedef. Required local definitions are collected rather than suppressed.

## Architecture

The implementation has four stages:

1. Parse each translation unit and record immutable facts, source locations, annotations, and
   constants.
2. Select roots and compute the dependency closure from the complete fact graph.
3. Resolve equivalent declarations, external references, and unsupported constructs.
4. Project the plan to RDL without mutating or discovering declarations during emission.

This separation keeps extraction order out of ownership and dependency decisions. Facts retain
translation-unit identity, while equivalent declarations are resolved during planning.

Source declarations supply type identity and pointer qualifiers. Projection applies the canonical
vocabulary and pointer-run normalization described below. SAL supplies direction, optionality, size
relationships, return-value markers, and interface-selection metadata; direction attributes are
interpreted relative to the emitted type.

### Function identity and annotation capture

`FactData` and `TypeRef` describe the projection, not a complete native declaration identity.
Extraction can map different C/C++ types to the same RDL representation. Equality of projected
types is therefore insufficient to establish compatibility between translation units.

`Snapshot` retains each function's canonical declaration location and Clang USR, keyed by its
TU-local origin. Function root selection uses this identity for same-TU redeclarations instead
of collapsing them by projected equality. Matching parent scope and linker names is insufficient:
distinct overloads can share an explicit linker name. Such collisions are rejected, while genuine
redeclarations retain the existing representative and annotation-selection policy.

This identity check applies only within one translation unit. Neither a USR nor a source location
proves that separate TUs observed compatible definitions. Cross-TU function selection and type
reconciliation use their existing rules; no native compatibility pass or definition graph is
constructed.

`extract/annotations.rs` captures parameter annotations once per projection attempt for functions,
callbacks, and interface methods. Each Clang annotation retains its full spelling, including all
arguments. `_In_reads_(n)` and `_Out_writes_(m)` remain separate; `_Out_writes_to_(n,m)` retains
both arguments. Legacy source markers such as `IN`, `OUT`, and `[retval]` remain individual, ordered
entries alongside the attribute strings.

Projection derives an attribute summary from the captured data, applies byte-count and
unresolved-reference policy, then interprets source markers. This order matters for comments whose
meaning depends on an earlier direction annotation. The capture is immutable during projection
and is released afterward; it is not duplicated or retained in `Snapshot`.

`ParamAnnotation` is the RDL projection's supported subset, including only the first size
relationship. Different source contracts can therefore have identical summaries. Neither summary
equality nor raw annotation-string equality establishes cross-TU compatibility. Changing SAL
representation or enabling cross-TU merges requires a separate contract and regression coverage.

### RDL type identity policy

RDL is the authoritative description produced from the headers. It preserves the type named by a
declaration unless that name belongs to a small, explicit canonical vocabulary. Canonicalization is
name-keyed rather than structural: two typedefs with the same ABI representation are not assumed to
have the same meaning.

| Source category | RDL policy |
| --- | --- |
| Scalar vocabulary (`BYTE`, `DWORD`, `FLOAT`, `DOUBLE`) | Use the corresponding RDL primitive. |
| MIDL predefined scalars (`boolean`) | Use the corresponding RDL primitive (`u8`). |
| Pointer-sized vocabulary (`SIZE_T`, `ULONG_PTR`, `LONG_PTR`) | Use `usize` or `isize`. |
| String aliases (`LPCWSTR`, `LPWSTR`) | Use resolvable canonical RDL string vocabulary. |
| GUID aliases (`IID`, `CLSID`, `UUID`) | Use `GUID`. |
| Generic void pointers (`PVOID`, `LPVOID`) | Retain aliases for unrepresentable chains. |
| Interface pointer typedefs | Project to the interface type; WinMD retains pointer semantics. |
| Other typedefs, including pointer typedefs | Preserve the name and emit its definition. |

The planner records each named type and parameter's projection choice within its translation unit.
Dependency collection and emission use that choice; parameter direction follows the emitted type's
RDL defaults. An input-only raw mutable pointer therefore needs `#[in]`, while a named string alias
defaults to input.

Canonical string names are emitted only when their definitions or metadata references are available.
Otherwise, a named alias retains its source name and definition, and an annotated character pointer
remains a raw pointer. Canonicalization does not rename declarations or redirect an alias to a
typedef that depends on it. This keeps concrete pointer definitions and avoids typedef cycles.
Nested pointer chains retain a declared void-pointer typedef when flattening it would produce mixed
`*const` and `*mut` levels that RDL cannot represent. Mutable uses of the same typedef still flatten.

Within a consecutive run of raw pointers, `pointer_run` emits every level as `*mut` only when all
levels are mutable. If any level is const, every level in that run becomes `*const`. This is a lossy
normalization for RDL's uniform pointer-chain representation, not preservation of each C/C++ pointer
qualifier. Retaining a typedef boundary does not change normalization of the raw pointers above it.

For a source declaration `typedef void* PVOID;`:

| C/C++ type | Projected RDL type |
| --- | --- |
| `const char**` | `*const *const i8` |
| `PVOID const*` | `*const PVOID` |
| `PVOID const**` | `*const *const PVOID` |
| `PVOID*` | `*mut *mut void` |

SAL direction is independent of this normalization. For example, `_Out_ PVOID const** value` emits
`#[out] value: *const *const PVOID`. Without a direction annotation, the normalized RDL type
determines the default: this const pointer chain defaults to input, even though the original outer
pointer was mutable. The projection does not synthesize typedefs to preserve mixed raw pointer runs.

The alias fixtures cover local and referenced canonical types, missing definitions, alias chains,
translation-unit isolation, and function, callback, interface, and record uses. Metadata assertions
in `test_clang`'s `projection` tests check typedef targets, pointer depth, and parameter directions
after compiling the RDL to WinMD; successful RDL compilation alone does not establish those properties.

Lowercase `boolean` is part of MIDL's predefined type vocabulary and has an unsigned 8-bit
representation, so it becomes `u8`, not RDL `bool`. Uppercase `BOOLEAN` is a named Windows API
typedef and remains `type BOOLEAN = u8`; references to it retain the `BOOLEAN` name. This preserves
the distinction between canonical language vocabulary and an API-authored typedef without treating
arbitrary byte values as Rust booleans.

For example, the headers declare `PBYTE`, `PDWORD`, and `PORHKEY` in API signatures. RDL retains
those names and separately records their representations:

```rdl
type PBYTE = *mut u8;
type PDWORD = *mut u32;
type ORHKEY = *mut void;
type PORHKEY = *mut ORHKEY;
```

This keeps both the authored API vocabulary and the ABI shape. A parameter declared as
`_Out_opt_ PDWORD` is emitted as `#[out] #[opt] PDWORD`, not `*mut u32`. A parameter written
directly as `ORHKEY *` remains `*mut ORHKEY`; it is not renamed to `PORHKEY`.

Do not flatten typedefs in RDL to accommodate a binding projection. A downstream generator can
resolve or collapse an alias when needed, while recovering a discarded source name is unreliable.
Likewise, do not use SAL direction to change `P*` aliases or mutable pointers into const pointers.
RDL records the projected type and the SAL contract separately; pointer-run normalization applies
regardless of SAL direction.

Incomplete records are valid when used through pointers and rejected when a complete by-value
layout is required. Fixed-underlying forward enums can be represented by their declared integer
type. Unfixed forward enums are rejected rather than assigned a guessed representation.

An incomplete declaration may resolve to a complete declaration from another translation unit when
their public names and C/C++ declaration kinds match. The completed projection may differ from the
placeholder representation: for example, an incomplete `struct` is initially a record but may
resolve to a COM interface once another translation unit supplies its virtual definition.
Incompatible declaration kinds remain ambiguous.

Defined POD C++ classes with public instance fields and no inheritance, methods, constructors,
destructors, conversions, or function templates use the checked record-layout path. Other
non-interface C++ classes remain opaque.

Record layout inference keeps member packing and forced record alignment separate. When more than
one representation matches Clang's size, alignment, and field offsets, it prefers one without
forced alignment and then the least restrictive packing. This distinguishes `#pragma pack(N)` from
an explicitly over-aligned record and permits records that require both.

MIDL-generated headers assign `__MIDL...` tags to anonymous IDL declarations and `_NAME` backing
enum tags to some public scalar typedefs named `NAME`. An unreferenced generated enum is emitted as
loose constants, matching the IDL API identity. When it is followed by a scalar typedef in the
source header, those constants use that public alias. The `_NAME` case additionally requires the
MIDL compiler marker in the same physical header and an exact adjacent `NAME` alias. Generated
records used only to define an opaque pointer typedef are likewise represented by the public
pointer alias. A generated declaration referenced directly by another ABI type keeps its generated
name and layout.

When an active object-like macro shadows an enum member declared by the same header, the member
takes the macro's effective value. This preserves the identifier that C callers see without
emitting two items into the header's shared RDL value namespace. Same-named declarations owned by
different headers remain separate.

RDL can encode a type definition and a member of the namespace's `Apis` class with the same
projected name. The extractor therefore preserves a type and object-like macro with the same public
name, whether they come from one header or are combined across translation units. This does not
impose namespace and type-name uniqueness on WinMD; tagged architecture inputs may contain more
than one matching `TypeDef`.

GUID, property-key, and coclass facts are also planned as values. They honor constant exclusions
while their referenced types still participate in dependency closure.

Native NaN and infinity constants are omitted because RDL and ECMA metadata cannot represent them.
This includes `f64` values that become non-finite when narrowed to their declared `f32` type.
Integer-valued pointer constants remain supported. If a typedef chain resolves to an
interface-pointer alias that is projected as the interface itself, constants declared with that
typedef are omitted because ECMA metadata cannot encode an interface-valued constant. An explicit
pointer to the same interface remains a pointer and is emitted.

### Bit-field member scraping

RDL and WinMD cannot encode C bit-field syntax directly. A consecutive run of bit fields is emitted
as an integer backing field with `NativeBitfieldAttribute` entries that preserve each member's name,
offset, and width. Bindgen uses those attributes to generate accessors over the backing field.

## Generator responsibilities

The consuming tool owns concerns outside header extraction:

- libclang provisioning and version checks;
- SDK, WDK, or component package restoration;
- compiler language mode, target, and include arguments;
- import-library parsing and function-to-DLL policy;
- architecture-specific extraction and merge;
- transactional promotion of generated RDL;
- RDL compilation and downstream binding generation.

The repository's generator tools share these facilities through `crates/tools/helpers`.
`tool-win32` supplies the Win32 and WDK policy, while `tool-webview` supplies the WebView2 policy.

The helpers restore exact NuGet versions beneath `NUGET_PACKAGES`, or
`%USERPROFILE%\.nuget\packages` when it is unset. Completed NuGet global-cache entries are reused
without modification. Archives in global or flat (`id.version`) cache layouts also support offline
restores. Tool downloads are extracted separately and published under `.windows-rs` within the
cache root only after extraction succeeds. Unmarked legacy directories without an archive require
a fresh download; they are not modified.

## Known limits

- RDL cannot represent mixed raw pointer-chain mutability. The projection normalizes each run to
  uniform mutability, losing per-level qualifiers as described above.
- Coverage is limited to declarations reachable from configured roots.
- The flat Win32 namespace cannot preserve distinct declarations that differ only by curated
  namespace placement.
- Header extraction cannot infer curated handle cleanup, last-error, or documentation policy.
- Stable Rust has no function-pointer ABI corresponding to every native calling convention.

## Testing

The crate's integration tests cover fact isolation, constants, layouts, interfaces, annotations,
dependency closure, header ownership, external references, incomplete declarations, macros,
callbacks, arrays, variadics, recursion, and cross-translation-unit resolution. Declarative
input/output cases live in `test_clang`: each `input/<name>.h` fixture generates
`expected/<name>.rdl`, which is parsed with `windows-rdl` before the golden is updated. CI rejects
any uncommitted output change.

Fixtures default to C++, the `Test` namespace, and `test.dll`. Leading `//!` lines may override the
setup:

| Directive | Effect |
| --- | --- |
| `namespace <name>` | Sets the emitted namespace. |
| `library <name>` | Sets the import library. |
| `args <arguments>` | Replaces the libclang arguments. |
| `reference-default` | Resolves extraction types against the default metadata. |
| `reference <name>.rdl` | Compiles a sibling RDL file to metadata for reference and exclusion. |
| `input <name>.h` | Starts a named translation unit in a multi-input fixture. |
| `file <name>.h` | Starts an auxiliary file that may be included by an input. |

Multi-input fixtures use one or more `input` directives followed by the source for each translation
unit. A `file` section writes a sibling header without treating it as a translation unit. The
harness emits inputs in forward and reverse order and requires identical output or errors.
Successful output is parsed with `windows-rdl`. An existing `expected/<name>.error` marks an error
fixture and receives the normalized diagnostic. Other fixtures write `expected/<name>.rdl`.
Custom reference metadata is supplied to both extraction and output compilation. Reference
directives may be repeated to combine metadata files.

```text
cargo test -p windows-clang
cargo test -p test_clang
```

The tests need a loadable compatible libclang. Repository CI obtains the pinned runtime with
`cargo run -q -p tool-clang -- path` and exports `LIBCLANG_PATH` before running the workspace.

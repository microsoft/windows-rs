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

The [replacement contract](#replacement-contract) defines the scope, architecture, and acceptance
matrix for rebuilding the header-to-RDL implementation. The intervening sections describe the
current implementation, not requirements to preserve its behavior.

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
The legacy projection retains a declared void-pointer typedef when flattening it would produce
mixed `*const` and `*mut` levels. Mutable uses of the same typedef still flatten.

Within a consecutive run of raw pointers, `pointer_run` emits every level as `*mut` only when all
levels are mutable. If any level is const, every level in that run becomes `*const`. This is a lossy
normalization in the legacy projection, not preservation of each C/C++ pointer qualifier. RDL
supports mixed pointer runs; this extractor still normalizes them. Retaining a typedef boundary
does not change normalization of the raw pointers above it.

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

## Known limits

- The legacy projection normalizes each raw pointer run to uniform mutability, losing per-level
  qualifiers as described above. RDL can retain mixed runs.
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

## Replacement contract

This is the design for the replacement, not a description of implemented support. Start from
master and port independently justified mechanisms from the clang2 prototype. Do not import its
planner, public API, or support restrictions as the design.

### Objective and scope

Produce a canonical description of Windows header APIs in RDL and WinMD:

```text
pinned headers + compilation profiles + explicit API contracts
    -> owned compiler evidence
    -> checked Windows representation rules
    -> closed RDL declarations
    -> compiled and inspected WinMD
```

Canonical means that a fixed input and policy have one deterministic representation preserving
the API's identities, types, values, layout, calling conventions, and captured contracts. It does
not mean equality with the existing generated files.

The initial production scope is the SDK and WDK inputs configured by `tool-win32`, on x86, x64,
and ARM64, and the header-to-metadata portion of `tool-webview`. Their package and compiler pins
remain in the existing tools. `metadata/winrt` generation from SDK WinMD is outside this scope.
General C++ compilation, arbitrary templates, and arbitrary C++ object models are not goals.
C-compatible data classes and the interface forms required by these Windows inputs are in scope.

Rust bindings, wrapper signatures, activation tests, and Rust calling support are not acceptance
gates. A downstream inability to call a correctly represented aggregate or interface does not
justify rejecting it here. Compiler evidence establishes native layout and callable structure;
native probes are reserved for facts the compiler inspection API cannot establish.

The existing `metadata/win32` and `metadata/wdk` directories are the starting API inventory. They
are not correctness oracles. Neither old extractor output, clang2 output, golden equality, nor a
successful WinMD compilation proves agreement with a header. Required corrections use source,
compiler, or documented API-contract evidence and are recorded as such.

### Stage boundaries

| Stage | Owns | Must not do |
| --- | --- | --- |
| Input profiles | Pins, target, defines, includes, scope | Invent missing declarations |
| Capture | Compiler handles, types, values, provenance | Choose RDL names or erase typedefs |
| Reconciliation | Identity, completions, agreement | Merge by name or projected equality |
| Representation | Names, aliases, contracts, imports, owners | Rediscover facts during rendering |
| Output | Closed RDL, WinMD, reports, promotion | Publish an incomplete required surface |

Capture retains written types alongside canonical types, qualifiers at every level, original
parameter positions, source-owned annotations, and declaration observations. Compiler handles do
not escape capture. Records carry compiler size, alignment, field offsets, and bit-field facts;
callables carry native convention, parameter/result types, and interface inheritance/slot evidence.

One private native model and one closed output model are enough. Reuse RDL emission and metadata
inspection facilities rather than adding another general intermediate representation. Rendering
receives resolved declarations whose dependencies are either local planned items or explicit
references. It performs no capture, alias resolution, ownership selection, or layout inference.

Profiles describe real independent preprocessing configurations. Compatible observations are
checked before coalescing. Mutually exclusive configurations remain distinct until a representation
rule establishes how they contribute to one API. Input order and filename sorting cannot choose a
native contract. A selected profile cannot silently change another API's dependent types.

Where headers conflict internally, an explicit contract must identify the native declaration and
source profile, the exact conflicting evidence, the chosen interpretation, and its supporting
source. Changed evidence invalidates that choice. This is not a general override or fallback path.

### Representation rules

| Family | Required preservation |
| --- | --- |
| Scalars | Native width, signedness, floating kind, and target-specific representation |
| Typedefs | Written API identity and alias target, including pointer and handle boundaries |
| Indirection | Pointer depth, each level's qualifiers, references, and incomplete identity |
| Records | Field order/types, nested identity, offsets, size, alignment, and bit-field meaning |
| Enums | Nominal identity, representation, values, aliases, scopedness, and explicit flags |
| Callables | Convention, result, positional parameters, variadics, and callback identity |
| Interfaces | UUID, base identity, native slot order/reuse, method types, and properties |
| Values | Declared type, compiler-evaluated value, string code units, and aggregate contents |
| Contracts | Source family, owner, scope, positional relationships, and understood semantics |
| Imports | Native linker symbol, DLL, export name or ordinal, and routing evidence |
| Publication | Stable names, defining-header ownership, architecture presence and variants |

Preserve API-authored typedefs by default. Primitive spelling is appropriate for compiler builtin
types, not as an automatic replacement for a named API type. Pointer-sized spelling requires an
explicit Windows typedef contract and target checks; matching pointer width alone is insufficient.
String encoding, termination, direction, and optionality are separate from native pointer
qualifiers. SAL must not turn a mutable source pointer into a const pointer.

A named record definition and its aliases remain distinguishable. Multiple public aliases survive;
structural equality does not make unrelated declarations interchangeable. Same-spelled tag/typedef
pairs may share a definition only after native identity is checked. Type and value namespaces are
separate. Anonymous enum members can become constants when there is no nominal API type to retain;
anonymous types needed by fields or signatures retain owner/slot identity. Generated names must
derive from that identity and reject collisions, not depend on absolute paths or traversal order.

Object-like value macros use their effective state in the chosen source profile. Definition,
expansion, and evaluation provenance remain available. Function-like helpers, declaration fragments,
and declaration aliases are classified separately, not tried as arbitrary constant expressions.
An alias to an API is a publication decision, not evidence of another DLL export. GUID/property-key
definitions and string/aggregate expressions use compiler-owned values. Declaration-only data is
not an invented constant.

Captured SAL and MIDL remain available independently of typed lowering. Parameter relationships
bind to original positions, not presentation names. Unknown captured annotations are preserved as
source contracts, not reported as fully understood semantics. Capture coverage itself must be
tested against the required source families; preserving captured payloads does not prove complete
annotation capture.

An import archive supplies routing, not type or annotation evidence. APIs without archive evidence
need an explicit documented import contract; do not guess a DLL from a function's name. Imported
data must have its own representation rather than become a constant with a fabricated initializer.

### RDL and WinMD representation work

The replacement must address limitations in the output format before encoding a lossy substitute.
Extend RDL and metadata attributes only for required native distinctions, with reader, writer,
and semantic readback tests. Keep the existing syntax when it preserves the required facts.

| Current boundary | Replacement requirement |
| --- | --- |
| Mixed pointer runs rejected | Preserve each level's qualifiers; no constness normalization |
| Type-based direction defaults | Preserve explicit source direction separately |
| Limited typed SAL/MIDL | Retain raw contracts and encode required relationships |
| No imported-data declaration | Define typed imports for required declaration-only data |
| Bit-field backing storage | Retain logical member type, signedness, width, and offset |
| Interface metadata methods | Establish required native slot reuse/order representation |
| Unsupported values/conventions | Report a format limit; no omission or guessed replacement |

ECMA-335 does not directly encode every native distinction. Where a required distinction needs a
metadata attribute, define its meaning and verify that RDL -> WinMD -> RDL retains it. If the
representation cannot preserve a required contract, report a representation blocker. Do not
describe that case as a parser failure or a Rust limitation.

### Fixed implementation matrix

These are work packages, not claims that the listed anchors are already correct. Inspect each
anchor's original header before accepting expected RDL. Existing small fixtures are reusable test
inputs; their goldens must be checked against the new rules.

| ID | Package | Source or fixture anchors |
| --- | --- | --- |
| C0 | API inventory and comparison | `metadata/win32`, `metadata/wdk`, WebView2 headers |
| C1 | Output vocabulary | Mixed pointers, contracts, data imports, interface slots |
| C2 | Sessions, identity, scalars, aliases | `smoke.h`, `alias_chains.h`, `winnt.h` |
| C3 | Records and storage | `data_classes.h`, OAIDL, NVMe, WDK records |
| C4 | Enums and constants | `midl_enum.h`, `enum_flag_macro.h`, GUID/property keys |
| C5 | Functions and callbacks | `callback_pointers.h`, `variadic.h`, `EXCEPINFO` |
| C6 | Windows interfaces | `IUnknown`, `IDispatch`, WebView2, override/overload controls |
| C7 | Annotations and relationships | `indirect_sal_size.h`, `com_outptr.h`, SDK SAL/MIDL |
| C8 | Macro state and evaluation | `calling_redefined.h`, `character_macros.h`, SDK helpers |
| C9 | Imports and declaration-only data | `InitializeXamlDiagnosticsEx`, GUIDs, RPC data |
| C10 | Profiles, ownership, architectures | SDK/WDK enums, `STRING`, main/satellite profiles |
| C11 | Production and approved promotion | `tool-win32`, `tool-webview` metadata generation |

Use this order and these dependencies:

1. C0 identifies the required declarations and their native representation families. C1 defines
   how those families survive RDL and WinMD. Complete the inventory and output design before
   importing a planner or running another broad parser-repair campaign.
2. C2 establishes owned capture and a small complete header -> RDL -> WinMD path. It requires C1.
   Port compiler lifetime and performance mechanisms only with their focused regression cases.
3. C3-C9 extend that path by the fixed families above. They require C2 and the relevant C1 output
   representation. Callables depend on their captured value types; interfaces depend on callable
   capture; annotations attach to the original owners from those packages.
4. C10 requires the covered families and compares their observations across production profiles
   and all three targets. C11 requires C0-C10; it consumes their checked results rather than
   repairing declarations in orchestration.

For each package, list selected native declarations, expected RDL meaning, required compiler facts,
negative controls, format changes, and completion assertions before porting code. Every accepted
case gets a dedicated `.h` input and reviewed `.rdl` expectation, plus semantic WinMD assertions.
Rejected cases get a diagnostic expectation. No generator should overwrite reviewed expectations
as part of an ordinary test run.

### Inventory and completion gates

C0 pins the baseline repository revision and input package/compiler identities. Parse existing RDL
to enumerate namespace, name, kind, members, aliases, signatures, attributes, architecture masks,
and owning partitions. Map declarations to original source profiles; unmapped items remain explicit.
Use the existing RDL parser and metadata readers, not a regex-based API census.

Baseline metadata may already omit APIs. Inventory declarations from the configured header scope
as well, classifying helpers and dependencies separately. Every baseline item and discovered API
root receives a disposition. Discovery establishes coverage, not correctness, and cannot make a
header helper a required exported API merely because it has a name.

Each difference is classified as preserved, source-backed correction, explicit scope exclusion,
or unresolved. Source-backed corrections include changes to existing wrong RDL; unchanged legacy
output is not the default answer. Counts of projected roots are not a substitute for this inventory.

| Gate | Completion assertion |
| --- | --- |
| Scope | Every required item has a source mapping and package; exclusions have reasons |
| Evidence | Accepted declarations use clean compiler evidence and checked dependencies |
| RDL | Reviewed expectations preserve the required identities and contracts |
| WinMD | Readback matches values, signatures, layouts, attributes, and architecture variants |
| Reconciliation | Profile/order controls pass; incompatible observations are not silently merged |
| Determinism | Fixed inputs/policy produce identical output and stable partitions |
| Coverage | No unexplained missing API or unclassified semantic difference |
| Promotion | All required gates pass before any canonical RDL or WinMD is replaced |

Reports distinguish source errors, unavailable evidence, conflicting observations, representation
blockers, and explicit scope exclusions. Each carries stage and source/profile provenance. Partial
output may be staged for diagnosis, but required failures return nonzero and cannot promote it.
An error-bearing compiler AST never supplies accepted evidence. Macro recovery needs a bounded
parse-work regression case, not success through repeated full-input retries.

Performance work follows a measured failure of an agreed resource budget, not an open-ended
optimization phase. Record parsing work, wall time, and peak memory on the pinned full inputs
before approving production use. Compiler capture need not share the old implementation's costs.

### Restart and stop rules

The next implementation task is C0: build the inventory and identify the required representation
families. C1 then resolves those families' output contracts. No new extractor crate, backend switch,
or canonical metadata regeneration is needed to begin those two packages.

The initial scope and C0-C11 package list are fixed. A newly encountered declaration is assigned
to a package or recorded as an explicit scope/design decision, not silently added to the active
task. If a required compiler fact is unavailable through libclang, isolate that fact and decide
whether a small Clang C++ bridge is needed before extending token heuristics. If a required native
contract cannot survive RDL/WinMD, resolve the format boundary before continuing capture work.
Neither condition authorizes guesses, legacy fallback, or another unbounded header campaign.

### Baseline inventory command

Run from the repository root:

```powershell
cargo run -p tool-clang --quiet -- inventory
```

This command uses `windows-rdl`'s parser to inventory every Win32 and WDK RDL partition and the
hand-authored metadata vocabulary seed. It does not load libclang, restore packages, parse headers,
compile metadata, or generate Rust. The default `tool-clang` pin-validation command and its `path`
subcommand are unchanged.

Reports are staged under `target\clang-inventory`:

| Report | Contents |
| --- | --- |
| `baseline.tsv` | Repository revision/status, configured package/compiler pins, scope, counts |
| `inputs.tsv` | RDL/configuration paths, byte sizes, and Git blob hashes without filters |
| `entries.tsv` | Declaration/member tree, spellings, attributes, architecture masks, source lines |
| `summary.tsv` | Counts by parsed entry kind, including members and architecture variants |

All TSV fields escape backslash, tab, CR, and LF as `\\`, `\t`, `\r`, and `\n`. IDs and parent IDs
are local to each `partition`; row order preserves member and parameter positions. The architecture
column uses X86=1, X64=2, and Arm64=4, with combinations added together; empty means unspecified.
Member entries inherit their enclosing mask. Shapes and attributes are normalized parser tokens,
not a semantic comparison or evidence that RDL defaults match a native contract.

`family_package` assigns a starting C1-C6 family from the RDL declaration kind; members inherit the
owner's family. Annotation, macro, import, profile, and architecture work is cross-cutting and still
needs source classification. A partition-derived `header_hint` is only a candidate filename.
`source_status` stays `unresolved` until a header/profile mapping is checked. Vocabulary definitions
are marked `vocabulary`, and namespace/use entries are `not-applicable`.

This completes the existing-RDL census portion of C0, not its original-header discovery, verified
source mappings, WebView2 API inventory, or semantic differential. Baseline pins describe configured
inputs, not packages or compiler binaries observed in a capture. Those remaining C0 obligations
must stay explicit when using the reports to select C1 representation work.

### Header source index

Run compiler-backed discovery without invoking the current extractor:

```powershell
cargo run -p tool-clang --quiet -- sources
cargo run -p tool-clang --quiet -- sources --family sdk --arch x64
cargo run -p tool-clang --quiet -- sources --family sdk --arch all --headers shellscalingapi.h,tlhelp32.h
```

`--family` accepts `all`, `sdk`, `wdk`, or `webview`; `--arch` accepts `all`, `x64`, `x86`, or
`arm64`. Defaults select all configured families and targets. `--headers` is an SDK-only selection
of distinct names from the configured manifest, in supplied order. Selected SDK runs scope only
their explicit headers; full SDK main profiles include the configured directory roots and
exclusions. No declaration projection, constant probing, annotation instrumentation, RDL emission,
WinMD compilation, or Rust generation runs here.

`sources/profiles.rs` reads package pins, SDK manifests/prelude/reset, and WDK source headers from
the existing generator Rust syntax trees. It constructs independent discovery profiles, not a
call through the legacy extractor or an export of its complete configuration. SDK main/satellite
grouping follows the configured manifest. WDK kernel inputs retain their configured order, target
defines, and API-level define. `offreg.h` has its own SDK-prelude profile: real SDK declarations
supply its prerequisites rather than the legacy synthetic typedef prelude. WebView2's two headers
are separate explicit-root profiles on all three targets, with the pinned SDK include closure.
These discovery choices do not establish production profile precedence or compatibility.

The runtime and compiler resource headers use the shared libclang pin. SDK/WDK/WebView2 package
versions are recorded, along with actual included-file hashes and compiler-reported targets.
Libclang can also discover host MSVC support headers; their observed files are fingerprinted, but
the toolset is not pinned by this command. `run.tsv` records that limit and the `INCLUDE` environment.
Complete reproducible toolset selection remains C10 work.

Reports live under `target\clang-sources\<family>-<arch>`, with `-selected` appended for an explicit
header selection:

| Report | Contents |
| --- | --- |
| `run.tsv` | Run status, revision, observed compiler, package pins, selection, open boundaries |
| `*.hpp`, `*.args.tsv` | Actual translation units and ordered compiler arguments |
| `roots.tsv` | Explicit/directory roots and exclusions per profile |
| `profiles.tsv` | Clean/blocked status, observation counts, actual target/pointer width, time |
| `diagnostics.tsv` | Compiler diagnostics and capture failures, including their profiles |
| `inputs.tsv` | Configuration, compiler DLL, baseline RDL, and included-file fingerprints |
| `declarations.tsv` | Compiler names, identities, locations, types, members, macro definitions |
| `mappings.tsv` | Top-level baseline declaration candidates, ambiguity, and missing evidence |
| `groups.tsv`, `groups-summary.tsv` | Unresolved mapping groups, evidence IDs, and work packages |
| `coverage.tsv`, `coverage-summary.tsv` | Baseline coverage across profiles of one family/target |

TSV escaping matches the baseline command. Native IDs are local to a profile; baseline IDs remain
local to their RDL partition. Each mapping candidate points to a native observation row. Source
locations include expansion and spelling provenance. USRs, canonical declaration USRs, semantic
parents, and typedef target USRs are compiler evidence within an observation, not cross-profile
reconciliation rules. Type strings are compiler spellings, not a captured type graph or checked
RDL semantics.

Explicit roots are checked against the compiler inclusion graph with `clang_File_isEqual`, not a
basename match. Exclusions take precedence over explicit and directory roots. Dependency and
translation-unit declarations remain separate from roots. Function bodies are skipped; this
indexes declaration surfaces, not arbitrary C++ implementation code. Dedicated header fixtures
cover same-basename scope collisions, aliases, nested members, overloads, redeclarations,
macro-generated declarations, macro redefinition observations, and error-bearing profiles.

| Mapping status | Meaning |
| --- | --- |
| `single-identity-candidate` | Name/kind/header hint selects one observed native identity |
| `ambiguous-candidate` | More than one native identity remains; all candidates are retained |
| `no-candidate` | Header was included, but no in-scope name/kind candidate was found |
| `header-not-observed` | The partition's header hint was not present in this profile |
| `blocked-profile` | Clean compiler evidence was unavailable; absence cannot be inferred |

Filename/name/kind matching produces candidates, not verified API identity or semantic agreement.
Aliases with different source names, legacy renaming, partition changes, and member mappings still
need review. No candidate is classified as preserved or corrected by this command. Native
`family_package` is a starting declaration-kind classification; COM records still need C6 evidence,
callback typedefs need C5 evidence, and annotations/imports remain cross-cutting.

Macro rows are definition observations, including redefinitions. They do not establish final
effective macro state, value, or whether a helper is a required API. Those decisions belong to C8.
All root observations retain `unclassified-source` until scope and representation dispositions
are checked; dependency/excluded rows are not publication decisions either.

Compiler errors and fatal diagnostics block the entire profile before declaration traversal.
Blocked counts are unknown (empty), not zero. The run returns nonzero after persisting available
reports, and no failed AST supplies declaration rows. `preparing` or `running` in `run.tsv` denotes
an interrupted/incomplete run; only a final `complete` status accepts all listed discovery profiles.
Output directories may contain files from earlier selections; the current `profiles.tsv` and
`run.tsv`, not a directory glob, define the run.

Full discovery at the configured pins completes all 18 profiles without compiler diagnostics.
It records 1,482,950 root observations across three targets, including record members,
parameters, enum members, macros, and repeated declarations. This is not an API count. SDK capture
and indexing took about 11 seconds per target; smaller profiles took about 0.4-1 second, excluding
report serialization and input hashing. No header repair or projection fallback was needed.

The SDK main profile's top-level baseline dispositions are below. Counts deduplicate candidate
rows by baseline partition/ID and respect each target's architecture mask. They are not semantic
agreement counts.

| Target | Single-identity candidate | Ambiguous candidate | No candidate | Header not observed |
| --- | --- | --- | --- | --- |
| x64 | 117,584 | 8,077 | 737 | 2,591 |
| ARM64 | 117,588 | 8,078 | 737 | 2,561 |
| x86 | 117,500 | 8,078 | 737 | 2,423 |

C0 now has the parsed legacy census, full configured source discovery, WebView2 declaration
inventory, and explicit baseline location candidates. Verified source mappings, required-API
dispositions, and classified semantic differences remain open. The next bounded work is reviewing
the unmatched/ambiguous representation families and defining their C1 output contracts, not
expanding header support or starting a Rust-binding validation campaign.

### C0 classification and C1 decisions

The full SDK selection is `HEADERS` followed by `SATELLITE_HEADERS`, matching `scrape_um`'s
configured `all` input. Both manifests are required; a clean main profile alone is not full
configured discovery. There are six profiles per target: SDK main/satellite, WDK kernel/offreg,
and two WebView2 headers.

`groups.tsv` classifies every nonsingle mapping once per baseline partition/ID/profile. It retains
the original mapping status, native evidence IDs, and a work package. Classifications describe
observations, not approved API dispositions. In particular:

| Group | Evidence and boundary |
| --- | --- |
| `tag-and-typedef` | Every typedef target USR equals the one observed tag identity |
| `inline-and-noninline-callables` | Distinct callable identities include inline and noninline forms |
| `distinct-callables` | Multiple callable identities remain without that inline distinction |
| `data-and-macro` | Data and macro definitions share a name; effective state remains C8 work |
| `class-name-value` | A baseline constant has a same-header class name; UUID capture is not inferred |
| `macro-name-callable` | A baseline function has a same-header macro name; target/import not inferred |
| `dependency-name`, `excluded-name` | A same-header compatible name lies outside the root scope |
| `other-header-name` | A name occurs elsewhere; name equality does not verify a moved API |
| `same-header-other-kind` | Same-header spelling exists, but not in the candidate kind family |
| `distinct-native-identities` | Other ambiguities remain, including scoped/nested native names |
| `no-name-observation` | No same-spelled declaration observation; generated names can fall here |
| `unobserved-header-no-name` | Header hint and name are both unavailable in this profile |
| `unavailable-profile` | Compiler evidence is blocked; absence cannot be inferred |

`coverage.tsv` combines only presence statuses for each baseline item and target, across its own
family's selected profiles. It does not merge native identities or contracts. A candidate in any
profile gives `candidate-observed`; an included header without a candidate gives
`observed-no-candidate`; otherwise the result is `unobserved-header`. Any blocked participating
profile gives `unavailable-evidence`. Every contributing profile status remains in the row.
Consequently offreg APIs are not counted as missing merely because the kernel profile omits them,
and SDK satellite-only declarations can satisfy source presence.

| Family/target | Candidate observed | Observed header, no candidate | Unobserved header |
| --- | --- | --- | --- |
| SDK x64 | 126,607 | 738 | 1,644 |
| SDK ARM64 | 126,612 | 738 | 1,614 |
| SDK x86 | 126,524 | 738 | 1,476 |
| WDK x64 | 6,415 | 37 | 0 |
| WDK ARM64 | 6,383 | 39 | 0 |
| WDK x86 | 6,398 | 40 | 0 |

These counts still do not establish required scope, semantic preservation, or missing native APIs.
WebView2 supplies a native declaration inventory but has no baseline partition in these Win32/WDK
RDL directories. Its absence from this legacy coverage table is not missing native discovery.

#### What the unresolved groups establish

Of 8,077 ambiguous x64 SDK main items, 8,047 are compiler-linked tag/typedef observations. The other
30 consist of 19 inline/noninline callable groups, three data/macro groups, and eight remaining
native-identity groups. All 37 x64 WDK kernel ambiguities are compiler-linked tag/typedef groups.
This is reconciliation work, not evidence that libclang cannot parse those declarations.

The x64 SDK main profile's 737 no-candidate items consist of 662 class-name values, 42 macro-name
callables, nine dependency names, 17 absent spellings, and seven same-header other-kind names.
The groups remain candidates until their original source contracts are checked. The following
anchors were checked in the pinned package headers:

| Source anchor | Source fact | Required representation or policy |
| --- | --- | --- |
| `appmodel.h` `PACKAGE_ID` | Tag and typedef name share one definition | Retain alias edge; do not emit unrelated duplicate records |
| `amsi.h` `CAntimalware` | Forward class carries UUID; `CLSID_Antimalware` is separate data | UUID value requires attribute evidence, not an invented object definition or initializer |
| `ntsecapi.h` `RtlGenRandom` | Macro rewrites declaration to `SystemFunction036` | Keep source alias and native callable identity distinct; verify import evidence |
| `clfs.h` `ClfsLsnBlockOffset` | User-mode macro aliases `LsnBlockOffset` | Do not bind it to the same-spelled WDK callable by name |
| `pathcch.h` `PathCchFindExtension` | Imported const-string form plus inline mutable-string overload | Classify helper/export roles before choosing published callable forms |
| `ntddk.h` `PBUS_HANDLER` | Pointer typedef names incomplete `_BUS_HANDLER` | Preserve original nominal tag and unknown layout, not a completed empty record |
| `processthreadsapi.h` pointer aliases | Both aliases name incomplete `_PROC_THREAD_ATTRIBUTE_LIST` | Preserve one incomplete identity and both written aliases |
| `rpcndr.h` `PNDR_CORRELATION_INFO` | Pointer typedef names incomplete `_NDR_CORRELATION_INFO` | No inferred storage or renamed identity from legacy `NDR_CORRELATION_INFO {}` |
| `dhcpsapi.h` `VendorOptions` | Pointer targets an unnamed record owned by `_DHCP_ALL_OPTIONS` | Publish owner/field-slot identity; legacy `_0` is not an original source name |
| `wincodec.h` `CLSID_WICImagingFactory` | GUID declaration precedes version-selected macro alias | Preserve declaration and effective use separately; do not select the first observed value |
| `dxgi.h` `QueryResourceResidency` | `IUnknown *const *`, counted input, separate counted output | Preserve pointer-level constness, source direction, and original count binding |
| `d3dkmthk.h` `pVidPnSourceNtHandles` | Field expands to `void *const *` in the checked target | Mixed qualifiers are also a record-field requirement, not only a parameter concern |

The remaining unobserved headers require C10 manifest/ownership dispositions. Do not add every
legacy partition to the active compiler input or regard each stale name as a parser deficiency.
The eight residual ambiguous SDK main items and anonymous/incomplete spellings stay explicit C2/C3
identity cases; do not remove prefixes, strip `_0`, or structurally merge them.

#### Output design decisions

C1 is a finite format task. It must separate distinctions already expressible from actual output
gaps. No new native extractor is required to implement or test these format contracts.

| ID | Decision | Existing mechanism / required change | Acceptance gate |
| --- | --- | --- | --- |
| C1-A | Preserve mixed pointer chains | Keep RDL `*mut`/`*const`; support nested existing `metadata::Type` nodes and per-level signature decoding | Mixed/uniform chains round-trip in fields, params, results, callbacks, and typedefs; uniform bytes stay stable |
| C1-B | Preserve declared alias identity | Reuse `type` and `NativeTypedefAttribute`; retain the named alias target | Several aliases to one record remain separate names referencing that definition; same-spelled tag/typedef does not duplicate a TypeDef |
| C1-C | Distinguish opaque from empty complete records | Canonical `#[opaque]` records and `NativeOpaqueAttribute`; pending source forwards stay in IR | Marker and nominal identity survive readback; pointer uses work; by-value, field, array, and layout consumers reject unknown storage |
| C1-D | Keep native qualifiers and contracts separate | Extend vocabulary only for qualifiers/contracts absent from signatures; no pointer rewriting from SAL | Per-level const/volatile, raw source-family payloads, explicit/unspecified direction, and positional relationships survive independently |
| C1-E | Preserve logical bit-field meaning | Extend bit-field vocabulary beyond name/offset/width | Signed/enum logical type and qualifiers/attributes survive alongside storage; padding does not become a value member |
| C1-F | Represent declaration-only data | Define typed data-import syntax/routing without `Literal` or a fabricated constant | Type, DLL/export or ordinal, and import kind round-trip; missing initializer is not converted to a value |
| C1-G | Preserve native interface slot relationships | Extend method representation for inherited slot reuse and checked native order | Override/new-slot and overload identities survive readback without appending/reversing slots by name |
| C1-H | Preserve required nonprimitive values | Keep GUID/property-key support; add only required aggregate/string-code-unit distinctions | Declared type, exact code units/fields, and compiler-backed values survive readback; no string-pointer or zero-value substitution |

**C1-A is implemented.** `metadata::Type` pointer variants compress uniform runs and contain a
nested pointer node when the qualifier changes. RDL accepts these chains, and the signature reader
decodes run boundaries recursively. Uniform signatures keep their existing bytes. A const-to-mutable
boundary uses a required `Windows.Win32.Metadata.NativeMutableAttribute` signature modifier;
without it, adjacent `PTR` bytes would merge into one const run.

The reviewed `test_rdl` fixture checks fields, parameters, results, callbacks, interface methods,
typedefs, arrays, and references through semantic readback and exact RDL/WinMD round-trips.
`test_metadata` checks exact uniform/mixed bytes and every qualifier order through depth five.
Merge/remap checks preserve pointer runs, parameter direction, and routed named pointees. The
vocabulary seed compiles with the marker. This does not establish native header capture or
arbitrary volatile/top-level const support; those remain C1-D and later capture contracts.

**C1-C is implemented.** Canonical RDL uses `#[opaque] struct Tag {}` or
`#[opaque] union Tag {}` for a resolved nominal identity with no exposed native storage.
`Windows.Win32.Metadata.NativeOpaqueAttribute` carries that distinction through WinMD. An
unmarked empty body remains a complete record. Pending source forward declarations belong to the
intermediate representation: reconcile them against available definitions before emitting a full
definition or an explicit opaque type. Canonical RDL does not accept `struct Tag;`.

Shared metadata validation follows aliases and supplied references, allows opaque pointers and
references, and rejects value fields/parameters/results, array elements, and layout claims.
Arrays need element storage even behind a pointer/reference. Validation precedes output writes
in RDL compilation/rendering and metadata merge/remap. Opacity participates in architecture
merge keys, so opaque and complete empty variants stay distinct.

The reviewed RDL fixture checks exact text/WinMD round-trips and nominal identity through aliases,
callables, external references, merge/remap, and architecture variants. Raw metadata controls
check malformed marker placement, declaration shapes, and storage uses without passing through
RDL first. These checks do not calculate native layouts or prove source closure; those remain
capture/reconciliation gates.

**C1-B's representation gate is covered.** The existing `type` form writes one value TypeDef
marked `NativeTypedefAttribute`, with one `Value` field carrying the written target. The reviewed
fixture retains several aliases to one complete record, alias chains, pointer aliases, and an
independent same-shaped record. An ordinary one-field record is not inferred to be a typedef.
No alias grammar or type model was added.

Semantic readback checks alias targets in fields, arrays, references, callbacks, imported
functions, and interface methods. Exact RDL/WinMD round-trips preserve those distinctions.
Merge/remap routes aliases and target records independently. External references retain their
assembly scope without copying target definitions; architecture merge keeps neutral aliases
pointing at the selected target variant. Malformed typedefs with zero or multiple payload fields
fail before the RDL writer writes or clears output.

Native same-spelled tag/typedef reconciliation remains a C2 gate. The representation has one
canonical record definition per published identity, not a duplicate self-alias. Prove that
publication choice from compiler-linked observations, not spelling or structural equality.

**C1-D's structural qualifier slice is implemented.** `Const<T>` and `Volatile<T>` qualify
individual native type nodes in RDL. Corresponding `metadata::Type` variants and required
`NativeConstAttribute`/`NativeVolatileAttribute` signature modifiers retain their exact placement,
without changing existing `IsConst` pointer-run/reference semantics or uniform signature bytes.
The reviewed fixture covers fields, aliases, arrays, references, parameters, and results through
exact RDL/WinMD round-trips and merge/remap. Raw signature controls reject optional modifiers
and missing type codes. Qualified values cannot bypass opaque-storage validation.

The legacy direction default examines the type beneath top-level qualifiers; explicit direction
is independent. Neither qualifier changes pointers based on SAL. Rust projection explicitly
rejects the new type nodes rather than erasing qualifiers; Rust output is not this gate.

**C1-D's source-contract slice is implemented.** `NativeAnnotationAttribute` preserves exact
payloads with a source-family/ordinal identity on each owner. `NativeAnnotationBindingAttribute`
links a source identifier in that evidence to an original zero-based parameter position on
the same callable, parameter, or return owner. Attribute table order does not choose source
order, and published parameter names do not resolve the relationship.

`#[unspecified]` preserves neither-`In`-nor-`Out` flags without triggering the legacy default.
Explicit input/output/inout and optionality survive independently of pointer qualifiers and
raw annotation families. A replacement extractor must emit unspecified direction when native
evidence supplies none. Typedef annotations and evidence on void results also survive rendering.

Reviewed text and exact WinMD round-trips cover SAL/MIDL families, source ordering, callbacks,
functions, interfaces, typedefs, fields, return evidence, and renamed presentation parameters.
Merge/remap retains those contracts. Raw metadata controls use out-of-order `Param` rows and
reject malformed identities, owners, and positions before checked emission, rendering/cleanup,
merge, or remap.

C1-D's representation gates are covered. Native annotation ownership, capture, agreement, and
typed interpretation remain later gates: raw evidence does not prove all payload identifiers
are bound, reconcile disagreeing families, or select a pointer/direction policy.

**C1-E's representation gate is implemented.** Typed bit-field members use
`name: LogicalType: width`; interval-only members retain the existing `name: width` syntax without
inventing a declared type from backing storage. `NativeBitfieldTypeAttribute` preserves integral,
enum, and typedef identity. Ordered qualifier companions and member-owned raw annotation companions
retain evidence without adding physical fields. Anonymous padding has no logical value descriptor.

Reviewed text/binary round-trips check signed/enum/alias types, both qualifier orders, raw evidence,
and physical field identity. Merge/remap routes logical references independently of their backing
record. Capacity checks follow known architecture-compatible aliases; malformed raw descriptors
reject before checked emission, rendering/cleanup, merge, or remap. Existing interval blobs and
backing signatures retain their bytes.

Native capture still must supply declared types, allocation units, layout, and zero-width barriers.
Unresolved external types and native-sized backing capacity need closure/target evidence. Storage
agreement does not establish native calling support. Rust projection rejects the new descriptors
until a separate accessor policy exists; it is not an acceptance gate for this header replacement.
Native declaration-only data uses the separate C1-F contract below.

**C1-F's representation gate is implemented.** `#[import(...)] static Name: Type;` declares
imported data without an initializer. One `NativeDataImportAttribute` preserves the exact DLL,
export name or u16 ordinal, and COFF data/constant import kind on a `Public | Static` field in a
neutral native `Apis` container. There is no `Literal` flag, Constant row, or fabricated value.
Native `Const<T>` remains independent of import kind. `reader::Item::Data` keeps these declarations
separate from constants, including legacy constants without an initializer.

Reviewed text/binary round-trips retain mixed pointers, qualifiers, aliases, arrays, raw evidence,
opaque external object identity, and named exports that resemble ordinal notation. Merge/remap
routes the declaration and its type independently without rewriting DLL/export text. Architecture
merge compares data routing and member evidence; differing contracts retain separate variants.
Raw metadata controls reject invalid owners, value/storage flags, routing, kinds, and argument
shapes before checked emission, rendering/cleanup, merge, or remap.

These are import representation checks, not native variable capture, COFF name-type interpretation,
DLL availability, symbol agreement, or storage/layout proof. The replacement extractor must obtain
import kind and exact export/ordinal from native/linker evidence, not from the source name or
constness. The current COFF inventory reader supplies only symbol/DLL pairs; it does not establish
this contract. Rust projection, feature generation, and legacy clang references reject new data
items instead of treating them as constants. Existing metadata is not promoted or repaired.

**C1-G's representation gate is implemented.** Native interfaces opt into
`#[vtable(size = N, inherited = M)]`. Each local declaration has an absolute `#[slot(index)]`,
or `#[slot(index, reuse = Owner)]` naming the ancestor that declares the inherited slot.
New declarations set `NewSlot`; reuse clears it. Source method order stays independent of
native slot order. No inherited MethodDef rows are fabricated.

Shared checks require a complete new-slot suffix, unique local slots, a single nongeneric native
base chain, and an ancestor declaration for known modeled reuse owners. Known base table sizes
must match the inherited prefix. Architecture variants belong to interfaces, not local methods;
merge keys retain slot maps, bases, and parameter/return evidence. Namespace remapping routes
reuse owners independently. Reviewed round-trips and raw negative controls cover overloads,
source-order differences, inherited reuse, external bases, architecture lineages, and rejection
before output replacement or cleanup.

An unavailable or unmodeled base does not supply native order from its method rows. The explicit
inherited count and direct owner identity remain representable; unavailable transitive ancestry
is not guessed. Declared signatures and raw receiver evidence remain separate from slot identity.
Capture/reconciliation must still prove native ordering, override legality, receiver qualifiers,
covariance, and thunks. Rust projection rejects modeled interfaces and their derived consumers
instead of treating row order as dispatch order. Unmarked legacy metadata is unchanged.

**C1-H's approved value model is implemented.** Complete non-union records, fixed arrays, and
exact literal code units use a checked `NativeConstantAttribute` semantic tree without an ECMA
scalar Constant row or invented object-memory bytes. Declared types retain aliases, enums, and
qualifiers. Ordinary narrow and UTF-8 literals remain distinct; UTF-16/32 units preserve embedded
NULs, unpaired surrogates, and their original lengths. No Unicode conversion or implicit terminator
changes the payload. Explicit floating bit constructors retain signed zero and NaN payloads.

GUID/property-key wire formats remain in use. Rendering preserves their declared types,
architecture attributes, and raw evidence; merge retains differing UUID/value contracts.
Reviewed exact round-trips, independent routing, supplied references, architecture-specific
character aliases, and raw malformed controls cover the new representation. Unsupported consumers
reject descriptors instead of treating a missing Constant row as a missing initializer.

All consumed value definitions must be available for type/member checks. Union active-member
values, bit-field initializers, relocations, partial/default initializers, and unevaluated
expressions remain explicit format blockers. The tree does not supply native padding, offsets,
object lifetime, or calling support. C4/C8 must obtain values and literal-kind/charset evidence
from the compiler; neither authored fixtures nor legacy output prove header agreement.

**C2's isolated path is implemented in unpublished `windows-clang2`.** Owned compiler sessions
capture an explicit selected scalar/alias/forward-tag graph without legacy fallback. Written
typedef edges, local qualifiers, compiler identities, and native layouts survive session disposal.
Checked output uses the shared RDL emitter. Reviewed three-target fixtures compile to WinMD and
check the native typedef and opaque representations.

The selected `alias_chains.h` closure and pinned `winnt.h` scalars pass; complete records and
callables from `smoke.h` remain explicit family-boundary failures. Reuse, concurrent sessions,
prior thread-binding restoration, clean diagnostics, exact root files, and deterministic
publication have focused coverage. See [C2's contract and internals](windows-clang2.md).

The next bounded package is C3: complete records and storage from the fixed anchors above.
It requires a selected source/layout matrix and native size/alignment/offset evidence before
porting code. No production generator or canonical metadata is replaced by C2.

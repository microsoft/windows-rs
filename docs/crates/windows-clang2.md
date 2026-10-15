# windows-clang2

An unpublished native capture and metadata projection experiment. See the
[readme](../../crates/libs/clang2/readme.md) for usage.

`runtime` owns compiler sessions, `capture` builds owned IR, `native` wraps compiler queries, and
`resolve` checks same-target native record, enum, function, and constant observations.
`merge` associates groups across
targets without collapsing layouts or type edges. Each worker reuses a precreated index across
TUs. Snapshots retain declarations separately from entities; resolution borrows them rather than
overwriting evidence. Type and cursor lookup use hash indexes. File paths are interned.

The optional `projection` module maps merged native types to public names, header partitions,
architecture variants, and RDL. It checks sequential struct and explicit union layouts against
captured ABI evidence. Native records retain their struct/union kind, including forwards.
Union fields must have offset zero; size is the largest field size rounded to the union alignment.
Nested records retain semantic ownership and stay inside their enclosing metadata type.
Its target-local plan separates native record identity from public alias correspondence:

- Only unqualified by-value alias bindings connect candidate record families. Pointer and array
  aliases do not establish pointee/element equivalence. Nested records follow their checked owner
  families rather than acquiring independent top-level public names.
- Complete definitions must agree in record kind, field names/order, offsets, packing, alignment
  requests, bit widths, layout, and canonical types/qualifiers. Record edges must belong to
  independently established families.
- A single native record uses its unique defining value typedef, otherwise its native tag.
  Multi-record families require a unique common defining typedef, or a unique common alias when
  none defines a tag. Other reachable names remain aliases. A name can be a record on one
  architecture and an alias on another.
- Alias ownership prefers a unique declaration header containing the corresponding native
  definition; otherwise all declaration headers remain candidates. Multiple owners use the shared
  `foundation.rdl` partition, without weakening compatibility checks.

All alias bindings are checked before a representative is used. Native and public comparison share
the same type checker with different record relations. The plan preserves native identities.

`AliasObservation::defines_tag` comes from the native typedef-definition query, not spelling or
header order. Primary naming runs after native compatibility checks. A common alias must bind to
every native record group in its family; a typedef naming only one member cannot name the family.
Several defining typedefs for one named tag leave that tag as the definition and all other names
as aliases. A multi-record family without a unique common choice fails even in partial mode.
The `primary*` fixtures cover these rules, alias chains, `using` aliases, metadata identity,
generated Rust assignability, and input-order independence. The SDK GUID fixture confirms that
`GUID` is a record, not an alias to an unrelated consumer name.

Partial capture records failed entities, follows reverse dependency edges across snapshots, and
compacts the surviving IR before reconciliation. It does not reparse individual declarations.
Unsupported tag and alias kinds are excluded across targets. Diagnostic causes are deterministic;
workers deduplicate diagnostics between TUs, and dependency edges are deduplicated on insertion.

### Nested records

`Record::owner` identifies the enclosing record. Named children correspond by their scoped name;
unnamed children use the first native field whose type contains that declaration. Native USRs
remain evidence, but filename/offset-dependent anonymous USRs do not control correspondence.
The same owner-relative identity is used for same-target resolution, cross-target association,
public-family checks, and omission keys. Pruning an owner also removes its children.

Stock `clang_Type_visitFields` exposes implicit anonymous aggregate fields and their offsets.
Their type declarations carry the anonymous-record predicate; the field cursors do not.
Capture retains an empty native field name for these fields. Projection uses `Anonymous`,
`Anonymous2`, and subsequent names, rejecting collisions rather than renaming real fields.
No offset is inferred from promoted members.

RDL keeps unshared direct fields inline when their generated field-position names match.
Shared, named, and indirectly referenced children use owner-scoped declarations and paths such
as `Outer::Inner`. Shared anonymous types get one scoped definition, not one copy per field.
Out-of-line definitions, pointer/array uses, callback signatures, packing, and ordinary aliases
to nested types retain their ownership. Source-dependent display spellings are not public names.
Nested enums, aliases declared inside records, and general C++ classes remain unsupported.
The interface slice below accepts a narrower base/virtual-method contract.

`nested_records.h` covers native offsets, shared identity, nested forward/out-of-line definitions,
and RDL/WinMD round trips. Its tests check differing source offsets and input order, public owner
families, scoped failures, three-target correspondence, and generated Rust assignability.

### Type evidence

Records and typedef/using aliases retain written and canonical types, attributes, and layouts.
Fixed array bounds use Clang's evaluated extents, including arithmetic, enum constants, `sizeof`,
conditional expressions, and constexpr calls. The companion identifies the corresponding native
type-location expressions rather than accepting expression cursor kinds indiscriminately.
Array parameters decay only when the native canonical parameter type supplies the matching pointer;
element constness and written element aliases remain intact. `TypeKind::Array.length` is `None`
for an incomplete array and `Some(0)` for an explicit zero bound. Unsized parameters and typedefs
used by them follow the same pointer adjustment; inner dimensions retain their extents.
Incomplete array storage remains unsupported. C static/qualified array-parameter
contracts and variable extents remain unsupported. Default field initializers are rejected,
including bitfield initializers that the stock child visitor omits.
Every type is checked through the native qualifier query before normalization or interning.
Only const, volatile, and restrict enter the IR; `__unaligned` and all other native qualifiers
are explicit capture failures, including qualifiers inherited through typedefs.
Typedef USRs can contain filenames or match across different canonical record bindings. Native alias
groups therefore preserve name-corresponding observations without claiming type equality; projection
checks their compatibility using public families. Macro declarations belong to their outer expansion
site, including token-pasted names. Configured roots include their dependency closure.
Compiler errors remain fatal. Unresolved attributes block even a single observation: matching
layouts or matching opaque cursor kinds do not prove identical packing/alignment requests.
Attributes retain their decoded native payload and source location, not token dumps. A
macro-generated attribute's cursor extent can span unrelated declarations between its definition
and expansion. Tokenizing that range is neither local evidence nor bounded diagnostic storage.

### Functions, interfaces, and constants

`ValueDeclaration` retains global function and constant observations. It participates in
the same root selection, canonical type comparison, target association, reverse dependencies,
and compaction as types. Callable observations retain parameter names and written types,
attributes, native mangling, and language linkage. Same-target native conflicts are fatal even
in partial mode. Public parameter names and written aliases must also agree before publication.
Native-compatible value declarations with conflicting public names or aliases fail strict
projection and become explicit all-target omissions in partial mode. No representative declaration
is chosen to hide the disagreement. `function_redeclarations` checks both directions and a conflict
on only one architecture.

When a selected name has an external C declaration, external C++ inline overloads with an observed
definition remain in `Snapshot::inline_functions` as names, USRs, and locations rather than exported
ABI values. Every observation of that overload must be inline. This classification is local to the
translation unit and target; it does not borrow a C declaration from another input or architecture.
Non-inline overloads, declaration-only inline overloads, internal functions, and C definitions
remain unsupported. All C observations still undergo native and public-contract reconciliation.

| Slice | Accepted contract | Publication |
| --- | --- | --- |
| Free function | External C-linkage declaration with a supported C/stdcall prototype | Explicit `Import` supplies DLL and optional export name |
| Interface | UUID-bearing record, one vptr, no data, at most one public non-virtual base, ordered public pure virtual methods | GUID, interface inheritance, and platform COM methods |
| Scalar constant | Const integer, enum, bool, or finite float/double with a constant-expression initializer | Compiler-evaluated value and declared type |
| Aggregate constant | Constant-expression GUID or property key with checked native fields and layout | GUID literal or format GUID plus property ID |
| String constant | Terminated ASCII `char` or valid UTF-16 `wchar_t`/`char16_t` array | `String` with explicit native encoding |
| Object-like macro | Explicitly selected supported expression at the end of the input | Compiler-inferred canonical type and value |

Use `project_with_imports` or `project_partial_with_imports` after capture to supply DLL/export
ownership. Native C mangling is evidence, not an inferred DLL export name. Missing imports are
errors or explicit partial omissions. Callbacks, declared functions, and methods preserve parameter
names, using positional names only for unnamed parameters.

Interfaces remain native records until publication. Reconciliation checks UUIDs, base identities,
base offsets, method slots, and canonical signatures. Native queries establish virtuality and
access; overrides, multiple or virtual inheritance, non-pure virtual methods, and data-bearing
interfaces are rejected.
Methods retain compiler-owned absolute Microsoft vftable slots and are ordered by those slots,
not declaration order. Publication verifies a contiguous extension of the inherited vtable.
Overloads preserve their native names and signatures in metadata; bindgen supplies its existing
numeric suffixes. Literal names that collide with those suffixes are rejected. The
`interface_overloads` fixture checks slot order, metadata round trips, and generated caller,
vtable, and implementation-trait bindings on x86, x64, and ARM64.
Member templates, nonvirtual methods, and method `using` declarations cannot add vtable slots.
Their kinds, names, USRs, and locations remain in `non_vtable_members`, but helper bodies are outside
ABI reconciliation. Constructor inheritance and non-method `using` declarations remain unsupported.
This is not a claim of full C++ ODR equivalence or C++ helper-method publication.

Ordinary value records may have one public nonvirtual value base at compiler-reported offset zero.
The base becomes an explicit `base` field with `NativeBaseAttribute`; its complete layout and all
derived field offsets, size, and alignment must match the compiler. Empty bases, multiple bases,
synthetic field-name collisions, and packed containment of explicitly aligned bases remain errors.
Inherited records and containing records/arrays cannot pass or return by value in callable
signatures without a C++ calling-ABI contract. Pointers and parameter references are supported.
The `value_inheritance` fixture checks both metadata directions and compiled Rust layouts on all
three architectures, including alias-spelled bases, inherited-only records, packing, and nesting.

Parameter defaults retain a compiler-owned expression cursor, location, and compiler-printed
expression. Capture matches that exact cursor when distinguishing defaults from array bounds and
other children; it does not accept arbitrary parameter expressions. Publication uses
`NativeDefaultArgumentAttribute`, not an evaluated constant or an extra ABI parameter. Runtime
default expressions remain evidence, not expressions to execute. Conflicting observed defaults are
errors. The `cpp_members` fixture combines defaults, array bounds, helper overloads, and const methods.

Method CVR qualifiers are separate from function-type qualifiers. Native comparison retains all
three; COM publication accepts const through `NativeConstMethodAttribute` and rejects volatile,
restrict, and ref-qualified methods. Callback signatures cannot carry method qualifiers.

C++ lvalue reference parameters retain a distinct native type and emit metadata byrefs, including
the const `REFIID` used by COM. Reference fields and reference returns remain unsupported.
An interface pointer becomes a metadata interface reference; another native pointer level remains
explicit. On x86, methods must use stdcall; on x64/ARM64, the native unified convention applies.
RDL's non-WinRT interface contract supplies the COM convention without new method syntax.

`Input::macros` selects object-like macros. Compiler-parsed `__typeof__` probes evaluate them after
the input, with native preprocessing expansion/definition evidence identifying their real owner.
The initializer expansion and synthetic variable are matched by exact offsets in the input file.
An earlier expansion elsewhere in the TU cannot supply evidence for an undefined macro at the probe.
No token arithmetic is performed. Ordinary declarations and macro origins remain distinguishable.
Missing/function-like macros, unsupported values, and non-constant expressions fail explicitly.
Unselected macros are not captured and do not appear in the omission report. Constant coverage
therefore needs a baseline comparison, not just publication or omission counts. `certbcli.rdl`
currently lacks 22 baseline constants from object-like macros: the `CSBACKUP_*`, `CSRESTORE_*`,
`CSBFT_*`, and `CSCONTROL_*` families plus `szBACKUPANNOTATION` and `szRESTOREANNOTATION`.
These include numeric expressions, typed casts, and strings; automatic discovery remains pending.

`clang_Cursor_Evaluate` can fold `(call(), 3)` to `3` despite runtime side effects; the companion's
C++11 constant-expression check gates evaluation. Initialized const variables do not become
metadata constants merely because libclang returns a number. `ConstantValue` retains float bits
for exact comparison, including signed zero. Non-finite values and extended floating-point types
remain unsupported. The `scalar_constants` fixtures check rounding, subnormals, finite extremes,
constexpr evaluation, target-dependent values, conflicts, and bit-preserving metadata round trips.
Enum-valued constants use their native backing type for evaluation while retaining the enum or
written typedef as the published type. The `enum_constants` fixture covers signed and unsigned
limits, scoped enum macros, unnamed values, target-dependent values, and cross-input reconciliation.

Aggregate evaluation reads integer leaves from Clang's `APValue`, including implicit zero-filled
array tails. GUID and property-key recognition checks field names, types, offsets, and total size;
publication requires the canonical `GUID` or `PROPERTYKEY` type. Values are not reconstructed from
numeric tokens or host object memory. A declaration can use its compiler-linked definition's
initializer. A declaration without a usable definition remains an explicit omission.

String arrays remove exactly one final terminator and preserve embedded zeros and padding.
UTF-16 decoding rejects unpaired surrogates. Narrow non-ASCII bytes, unterminated arrays, pointer
strings, and arbitrary aggregate shapes remain unsupported. Selected string and GUID expression
macros use the same evaluator. `SelectAny` remains captured evidence but does not participate in
constant-value equality or become a linker-storage attribute: metadata publishes values, not native
object definitions. Other declaration contracts still require agreement.

The `aggregate_constants` header/RDL fixture checks values, architecture variants, copied and
zero-filled aggregates, declaration/definition pairs, selected macros, conflicts, and rejected
initializers. Both metadata round trips retain GUID architecture tags and GUID/property-key
annotations. `sdk_constants` covers known-folder GUIDs, property keys, WinRT class/interface strings,
and numeric SDK macros on all three targets.

The `free_functions`, `interfaces`, and `constants` header/RDL fixtures cover all three targets,
same-target conflicts, input order, pruning, and WinMD round trips. The ignored `sdk_breadth` test
uses the SDK's `GetCurrentProcessId`, `IUnknown`, `IClassFactory`, `UIA_InvokePatternId`, and
`MAX_PATH`. Full SAL interpretation, automatic macro discovery,
and general C++ declarations remain separate coverage work.

The ignored `sdk_functions` fixture publishes 114 explicit imports across kernel32, user32, ole32,
advapi32, ws2_32, bcrypt, gdi32, dxgi, winhttp, wevtapi, d3d11, d3d12, d2d1, and dwrite.
Coverage includes buffers, DirectWrite factory creation and Direct2D matrix operations,
Direct3D device creation and root signatures, callbacks,
COM activation, registry/security, sockets, cryptography, GDI, timing, virtual memory, HTTP,
event logging, service control, and process inspection, alongside `IClassFactory`,
`MAX_PATH`, and three Direct3D floating-point macros. It checks imports, parameter counts, calling
conventions, callback names, enum parameters, target-dependent `SOCKET` storage, and float bits
through three-target WinMD round trips. Strict and partial publication must agree without omissions
for this selected surface. DLL ownership remains explicit input, not automatic import discovery.

The `sdk_interfaces` fixture follows storage, imaging, properties, monikers, shell dialogs, and
file-operation, Direct3D/DXGI, DirectWrite, Direct2D, and WIC raw imaging interfaces from 32 roots.
The complete `IDWriteFactory` and `IDWriteTextAnalyzer` closures cover font collections, font faces,
text analysis, text formatting, and text layout. Every captured interface's UUID, base, method order,
parameter names, and arity must survive both metadata round trips on all three architectures.
Exception contracts, method constness, and parameter defaults must also agree with native evidence.
`IDWriteFactory7` and `IDWriteFontSet3` cover newer font and layout graphs.
`ID2D1Factory7` and `ID2D1DeviceContext6` include effects, printing, geometry, imaging, and SVG.
The fixture includes `d2d1effectauthor.h` and `documenttarget.h` to complete forward declarations
used by those signatures; absent definitions are not guessed.
The function fixture includes `D2D1CreateDevice` and `D2D1CreateDeviceContext`, retaining their
inline C++ overloads as provenance while publishing the C signatures.

Native deprecation retains the compiler-decoded message and replacement through
`NativeDeprecatedAttribute`, including empty payloads and enum members. Relocated typedef
annotations retain `TypedefName`, rather than changing their owner to the underlying declaration.
Native/public reconciliation compares deprecation payloads; disagreements are not representative
selection choices. The attribute preserves evidence and does not yet request Rust deprecation
diagnostics. The `deprecations` fixture covers records, interfaces, methods, imports, callbacks,
typedefs, constants, enum members, fields, and both metadata round trips.

Unconditional `noexcept`, `__declspec(nothrow)`, and empty `throw()` retain distinct written
`ExceptionSpecification` values; Clang's canonical type identifies their non-throwing signatures.
The shared native/public comparator checks exception specifications as well as calling convention
and signature edges. Publication preserves the written contract through
`NativeExceptionSpecificationAttribute("noexcept" | "nothrow" | "throw()")` on methods, imports,
and delegates.
The attribute records evidence; it does not change Rust exception handling or promise generated
unwind adapters. Computed/dependent and nonempty dynamic exception specifications remain rejected.
An unnamed non-throwing function pointer needs a compatible named callback to carry its contract;
an ECMA-335 `FNPTR` alone has no attribute owner. The `exception_specifications` fixtures check
native distinctions, cross-input conflicts, callback naming, partial publication, and round trips.

COM references do not express C++ object constness. Publication maps both `I*` and `const I*`
to the same interface reference, while capture and native reconciliation retain their distinction.
This does not imply object immutability: [DXGI's `SetPrivateDataInterface`][dxgi-private-interface]
retains and releases its `const IUnknown*` argument. Const on outer pointer storage remains
meaningful: `I* const*` becomes `*const I`, while `const I**` becomes `*mut I`. Volatile/restrict
remain unsupported, and ordinary record pointers keep pointee constness.
The `interface_qualifiers` fixture checks those distinctions and native conflicts.

Interface typedefs such as `ID3DBlob` publish through the existing transparent alias representation.
Call signatures use the canonical interface reference, not a by-value C++ interface object.
Alias declarations retain their attributes. The SDK fixture checks the alias through metadata
round trips and high-level Rust emission.

[dxgi-private-interface]:
https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgiobject-setprivatedatainterface

Raw flat bindings generated directly from the fixture namespace have a separate downstream issue:
the generator emits both its built-in `IUnknown_Vtbl` and the fixture's `IUnknown_Vtbl`.
Do not treat the alias-emission check as a compiled raw COM consumer or change native metadata to
hide that duplication.

### Annotation evidence

`tool-win32-2` force-includes its `sal.h` transport before SDK headers. Five SDK source hooks
emit compiler-visible annotations instead of maintaining replacements for individual SAL contracts.
Structural wrappers such as `_When_`, `_At_`, and `_On_failure_` have separate transport because
they bypass those hooks. The header requires `__SPECSTRINGS_STRICT_LEVEL=0`; strict specstrings
mode erases contracts such as `_COM_Outptr_`.

The owned `SalAnnotation` and provisional `Windows.Win32.Metadata.NativeSalAttribute` retain the
SAL version, contract name, source-hook arguments, and arguments after further macro expansion.
Outer macros can substitute arguments before the source hook: these are not a verbatim copy of
the original header invocation. Structural wrappers retain their nested source spelling and expanded
annotation transport without flattening conditional contracts into unconditional ones. Expressions
are preserved, not evaluated.

Common parameter contracts also emit direction and argument-optionality flags. Returned-object
nullability does not imply an optional argument pointer. Buffer capacity and initialized length
remain separate expressions in the retained contract; neither is collapsed into the other.
The `annotations` fixture checks their RDL/WinMD round trips on all three targets, and the SDK
case covers `ReadFile` and `IClassFactory`.

Name-corresponding typedef observations may differ in SAL coverage. Publication selects an observed
annotation set only if it contains every other observed set. Incomparable sets fail, even in partial
mode; publication does not invent their union. Native definitions and callable observations still
require matching annotations. Original observations are retained unchanged.

SAL is emitted on supported functions, methods, parameters, constants, records, fields, aliases,
callbacks, and named enums. Callback prototype parameters belong to `AliasObservation`, not the
interned function type. Their names, written types, and annotations use the same publication path as
ordinary function parameters. Written callback aliases retain their contracts; ABI-only callback
naming excludes annotated and indirect prototypes so unrelated inline pointers cannot inherit them.

When a typedef supplies a record or enum's public name, its annotations carry `TypedefName` in
`NativeSalAttribute`. This distinguishes them from annotations on the underlying tag. A callback
alias chain likewise retains inherited typedef annotations with their original typedef names.
Annotated inline/nested callback parameters, bitfields, enum members, anonymous enums without a
public owner, and expanded typedefs remain unsupported. Partial publication removes their dependent
declarations rather than dropping annotations. Unknown Clang annotations and malformed transport
are explicit errors. Further SAL interpretation is deferred in favor of API breadth.

Fixtures and callers live in `crates/tests/libs/clang2`, using only the public API. Add headers
under `input` and assertions under `tests`. The runner provides compact inspection and synthetic
scaling:

```text
cargo test -p test_clang2
cargo run -p test_clang2 -- inspect shared_a shared_b
cargo run --release -p test_clang2 -- scale
cargo test -p test_clang2 --test sdk -- --ignored
cargo test -p test_clang2 --test breadth -- --include-ignored
cargo test -p test_clang2 --test sdk_functions -- --include-ignored
cargo test -p test_clang2 --test alignment -- --ignored
cargo run --release -p test_clang2 -- sdk-gate
cargo run --release -p test_clang2 -- sdk-scale 32
cargo run --release -p test_clang2 -- sdk-sweep breadth 8
cargo run --release -p test_clang2 -- sdk-sweep tus 32
cargo run -p tool-win32-2 --quiet
cargo run -p tool-win32-2 --quiet -- --partial
cargo run -p tool-win32-2 --quiet -- --partial --sdk-imports
cargo run -p tool-win32-2 --release --quiet -- --partial --first=357
```

`tool-win32-2` is the edit/rerun loop: parse SDK headers on all three targets, merge, project,
compile, then replace `metadata/win32` wholesale. It shares `tool-win32`'s `src/headers.rs`
manifest and defaults to all headers. Main and satellite TUs follow the SDK's include rules.
`--first=N` selects a manifest prefix; positional arguments select headers, not types.
`--import=Function=library.dll` supplies function ownership; `--macro=NAME` selects a constant macro
in each input. Both options can be repeated. Unconfigured function imports remain explicit omissions.
`--sdk-imports` loads the pinned SDK's x64 import libraries using the shared ordered policy in
`crates/tools/win32/src/imports.rs`. Host libraries precede API-set umbrellas, and the enclave runtime
comes last. The same checked DLL corrections apply in both generators; an unexpected SDK mapping
fails generation. This is an x64-derived publication policy, not per-architecture export validation.
Explicit `--import` entries take precedence over discovered entries.
Keep existing known named imports when enabling discovery: a COFF ordinal entry does not establish
whether the DLL also exports that function by name.

The COFF reader distinguishes linker symbols from named or ordinal PE exports. Renamed exports
retain their export name. Ordinal-only entries are excluded and reported in
`target/win32-2/import-omissions.txt`; they are not guessed to be named exports or replaced by a
lower-priority library. Missing libraries and malformed archives are fatal. Header capture and
native reconciliation do not infer DLL ownership.

Strict mode fails on unsupported declarations. `--partial` emits the supported subset and
writes `target/win32-2/omissions.txt`; compiler errors and retained-definition conflicts remain
fatal. Both modes compile before replacing generated RDL; the directory's readme is preserved.
The partial WinMD stays under `target/win32-2`. Git's metadata diff is a review baseline, not a
correctness oracle; fixtures check native layouts and the RDL/WinMD round trip independently.
The omission report covers AST declarations, not uncaptured macro constants.

The full manifest includes `UNICODE_STRING` bindings to both `_LSA_UNICODE_STRING` and
`_UNICODE_STRING`. Native merge preserves both identities; publication checks their definitions
and emits the corresponding aliases. The `record_alias_a/b` and `publication_*` fixtures cover
this model, including recursive records, nominal pointer distinctions, equal-layout conflicts,
input-order independence, and architecture-specific record/alias variants.
The `union_*` fixtures cover union aliases, recursive forwards, packing, arrays, containing structs,
architecture-specific record kinds, and RDL/WinMD layout round trips. Partial exclusions propagate
from unsupported union fields across all targets.

Bitfield capture retains evaluated widths, unnamed members, zero-width allocation barriers, and
native bit offsets. Both native reconciliation and public-family checks compare widths even when
record size and field offsets match. Projection groups contiguous members into integer storage
fields and checks storage offsets, record size, and alignment against Clang. A storage group must
have one size and alignment, and its named members must agree in signedness. Unnamed padding
does not constrain signedness; the backing takes the first named member's canonical integer type,
or the first storage type for an entirely unnamed group. Reconciliation retains all native types.
The existing RDL convention names one backing `_bitfield`, or multiple backings `_bitfield1`, etc.
Collisions with declared names fail rather than choosing another name. Unnamed members advance
member offsets without becoming accessors; trailing padding needs no metadata attribute.
Zero-width barriers split groups and emit no field. If their layout effects cannot be represented
by the remaining storage fields, projection fails rather than inventing padding.

The `bitfields` fixtures cover 8/16/32/64-bit storage, signed accessors, computed widths, padding,
packing, barriers, unions, architecture variants, and same-layout conflicts. Union members use
separate offset-zero storage fields, including members with different types. The ignored test
compiles raw Rust bindings with size, alignment, and storage/member offset assertions for all three
MSVC targets. Separate host execution checks supported struct accessors and neighboring bits.
Enum and bool bitfields remain unsupported. Mixed signed/unsigned members in one allocation unit
need a per-member type contract: `NativeBitfieldAttribute` currently carries only name, offset,
and width, while bindgen derives signedness from the backing field. Bindgen's high-level union
bitfield methods fail to compile because their union reads lack `unsafe`. That consumer defect
does not block metadata; raw bindings provide an independent ABI check. Some MSVC bitfield-only
unions have alignment 1 despite wider integer storage. Those layouts remain rejected until their
storage alignment can be encoded without inventing a native packing request.

Enums use separate typed IDs and observation tables, not integer aliases or synthetic records.
Native reconciliation checks backing types, layouts, scopedness, attributes, and ordered members.
It checks forward declarations as well as definitions. Public enum names use their unique defining
value typedef, otherwise their native tag. Aliases cannot equate distinct native enums. Scoped enums
retain `ScopedEnumAttribute`; no flags are inferred from numeric patterns. The `enum_*` fixtures cover
all eight integer widths/signs, 64-bit limits, expressions, duplicate values, empty definitions,
forwards, nominal conflicts, and architecture-specific backing types. Partial pruning uses shared
tag names so a failed enum also excludes a same-named record on another architecture.

Enums reported anonymous by `clang_Cursor_isAnonymous` publish their members as typed constants
when they have no public type name. Native enum identity and observations remain in the IR.
Compiler-provided typedef names and primary aliases keep named-enum publication. A pointer alias
does not supply an enum type name; references to an unnamed enum type remain explicit failures.
The `anonymous_enum*` fixtures cover these distinctions, integer limits, expressions,
target-specific types and values, RDL/WinMD round trips, and compiled constants on all three MSVC
targets.

Constant names have a separate publication namespace from type names. Same-target constants must
agree in native backing type and value, including native integer distinctions such as `int` versus
`long` that share a metadata representation. Matching constants combine header ownership, using
foundation for shared owners; repeated observations do not duplicate architecture entries.
Attributes remain checked, and partial failures remove the whole rejected enum's constants.
`Output::constants` and the SDK/sweep reports count published constant names, not native enums.
Individual enumerator names are not type roots; empty `Input::roots` includes anonymous enums.

Native `char16_t` and `char32_t` retain distinct scalar kinds during reconciliation and callback
matching, then project to `u16` and `u32`. They are not interchangeable with `wchar_t` or ordinary
unsigned integers merely because their layouts match. `character_scalars` covers aliases, fields,
callbacks, unsigned enum limits, and same-layout native conflicts.

Mixed const/mutable pointer chains preserve pointer-storage qualifiers through RDL and WinMD;
COM object constness follows the normalization described above.
The metadata writer uses per-level modifiers for mixed chains while retaining the leading-modifier
encoding of homogeneous chains. `mixed_pointers` covers fields, arrays, void pointers, written
aliases, and callback signatures, with a focused generated Rust compatibility check.
See [metadata pointer constness](windows-metadata.md#pointer-constness) for the binary contract.

The SDK case uses the `tool-win32` SDK pin and cached clang resource headers (or
`CLANG_RESOURCE_DIR`).
`sdk-gate` checks 26 SDK roots on all three targets and exits unsuccessfully if any are blocked.
`sdk-scale` measures 15 supported roots with 2, 8, or 32 TUs, checking comparison-work bounds.
`sdk-sweep` uses complete SDK header groups without type allowlists: breadth combines 1/2/4/8 groups
in one TU; tus distributes them over 2/8/32 TUs. Artifacts go under `target/clang2-review`, not
canonical metadata. Its merge timing is a separate probe; projection includes its own merge and
pruning passes. Public-family and alias checks are included in projection timing, not native merge
comparison counts. Native counts and owned-byte estimates are measured after capture pruning.
Unresolved attributes such as `Uuid` remain evidence, not verified semantics.

`TypeVisibility` is decoded by the native companion, including Clang's implicit default attribute
on `_GUID`. Only the default request enters the IR as `DefaultTypeVisibility`; hidden/protected
requests are explicit capture failures. Native definitions and public families compare its
presence, and an explicitly attributed record forward requires the same request on its definition.
The attribute controls visibility of a type and its associated C++ linkage objects, not field layout.
Supported plain records and integer enums do not project RTTI or vtables, so default type visibility
needs no RDL attribute. Ordinary symbol `Visibility` remains separate and unsupported; accepting
default type visibility does not authorize discarding other attributes.

The `visibility*` fixtures cover tokenless implicit attributes, records, unions, enums, forwards,
same-layout native/public conflicts, dependency omissions, RDL/WinMD round trips, and generated Rust.
The SDK fixture also captures and projects `_GUID` with its native size, alignment, and fields.

The native query boundary is visible in `opaque_packing.h`: the same named record under `pack(1)`
and `pack(2)` has identical fields and layout, and libclang reports the same tokenless
`UnexposedAttr` with no source location. The companion identifies `MaxFieldAlignment` and returns
8 versus 16 requested bits. Native reconciliation and public-family checks reject that difference
even when the layouts match. Projection emits the requested `#[packed(N)]` and checks every field
offset, final size, and alignment. The `packing` fixtures cover requests from 1 through 16 bytes,
unions, nested records, and preservation through RDL/WinMD round trips.
Clang's [MaxFieldAlignmentAttr declaration][max-field-alignment] has an alignment argument but
no spelling; the [libclang C API][clang-c-api] exposes layout queries, not that argument.
The [windows-clang-sys](windows-clang-sys.md) extension exposes this argument for owned capture.

Explicit alignment uses the companion's `AlignedAttr` query. Capture retains evaluated bits and
the GNU, standard, or declspec form, including zero and multiple requests. Native and public-family
checks compare sorted, deduplicated request sets, not just their maximum or resulting layout.
An aligned forward must have its requests present on the complete definition; an unadorned
forward does not assert absence.

Complete structs and unions emit the maximum nonzero request as `#[align(N)]`, within RDL's
`u16` limit. Layout verification respects over-aligned fields and checks offsets, size, and final
alignment against native evidence. The `alignment_*` fixtures cover all three parsing targets,
RDL/WinMD round trips, same-layout request conflicts, and generated Rust layout assertions on
the host. The ignored alignment test compiles those assertions for all three MSVC targets and
requires their Rust standard libraries to be installed. Field, typedef, and enum alignment remain
unsupported, including primary record aliases and non-value typedefs. Combined packing/alignment
and packed containment of aligned records are rejected; Rust disallows that containment even
through arrays and intermediate records.
Partial projection removes these declarations and their dependents rather than losing alignment.

The runtime is pinned through `windows-clang-sys::LIBCLANG_VERSION`. LLVM 23.1.3's stock C API
also lacks the packing payload accessor, so newer Rust bindings alone cannot expose it.
Runtime and matching resource-header upgrades are separate from declaration coverage changes.

[ClangSharp's native extension][clangsharp-extension] exposes precise attribute kinds through
`clangsharp_Cursor_getAttrKind`. Its `clangsharp_Cursor_getMaxAlignment` calls
`Decl::getMaxAlignment`, which reads explicit `AlignedAttr` requests, not `MaxFieldAlignmentAttr`.
The sys crate adds packing and per-attribute alignment queries to a version-matched native companion
without replacing the owned IR, reconciliation, or projection. Both parser crates use its generated
stock bindings.

Rapid iteration can continue on declarations whose semantics are available through the current
API. Keep opaque attributes and their dependent declarations in the omission report. Revisit this
boundary before adding support that depends on those attributes or claiming complete coverage;
never infer packing or discard unknown attributes to make a batch pass.
An attribute's native kind identifies what still needs decoding; it does not justify discarding
an unknown payload.

With the pinned runtime and attributed-type capture enabled, the `opaque_callbacks.h` x86 probe
gives:

| Declaration | Written function kind | Calling convention | Attribute cursor children |
| --- | --- | --- | --- |
| Plain callback | `FunctionProto` | C | None |
| Explicit `__cdecl` | `Attributed` | C | None |
| Explicit `__stdcall` | `Attributed` | stdcall | None |
| `regparm(2)` callback | `FunctionProto` | C | None |
| `noreturn` callback | `FunctionProto` | C | None |

The plain and `regparm(2)` callbacks have identical queried result types, parameter types,
variadic flags, and exception specifications, but different canonical Clang types and x86 argument
passing. A compiler `static_assert` in the fixture checks the type distinction. Their spellings
also differ; neither spelling nor type equality supplies a decoded attribute payload for the IR.
Rejecting `CXType_Attributed` alone therefore does not make function-prototype capture safe.
`windows_clang_getFunctionInfo` exposes these hidden properties and flags additional prototype
information that callers must handle or reject. The sys fixtures verify explicit `regparm(0)`
versus absence, nonzero `regparm`, `noreturn`, and method-qualifier detection.
Callback capture accepts plain C and stdcall prototypes only after the supplementary query returns
no unsupported properties. Explicit `regparm(0)` is rejected as well as nonzero `regparm`;
`noreturn`, other extra properties, variadics, and exception specifications other than unconditional
`noexcept`, `nothrow`, and empty `throw()` remain unsupported.
Known cdecl/stdcall attributed types retain their written and equivalent types.

Owned function types retain result and parameter edges, calling convention, and exception
specification. Native/public type
comparison follows all signature edges, not just pointer size. Partial pruning likewise follows
every result and parameter dependency, including nested callbacks. These edges can branch, unlike
a pointer or array chain.

Projection preserves written value-typedef and callback names in fields and callback signatures,
including pointers, arrays, results, and nested named callbacks. Reachable value aliases are
published even when they are not explicit roots. Alias declarations retain canonical storage types;
non-value aliases such as `void` expand at use sites. Named callback typedefs become RDL `extern fn`
declarations with native parameter names, using positional names only for unnamed parameters.
Unnamed function-pointer types use the alphabetically first compatible, contract-free explicit
callback prototype across the target.
The plan validates public families and alias bindings before comparing callback candidates with
the shared type checker. Calling convention and arity narrow the search, but every candidate must
match qualifiers, layouts, scalar kinds, nominal enum identities, public record families, and all
signature edges. Failed candidate comparisons do not share assumed type pairs with later attempts.
Renaming or reordering TUs cannot change name availability. Signatures without a compatible named
declaration use structural `extern "C" fn(...)` or `extern fn(...)` types, encoded as ECMA-335 `FNPTR`.
Their written aliases, nested signatures, and native array/function parameter adjustments are
preserved without inventing public names. Bare function aliases are not projected as values.
The output's alias count includes callback declarations.

A failed value-alias or callback projection removes the declaration and its dependents. Removing
only its root marker would not make progress, since dependency publication would select it again.
Non-value aliases can remain as native evidence without being public roots. The callback
fixtures cover this distinction, C/stdcall metadata ABI values, nested results/parameters, arrays,
pointer indirection, same-layout native signature conflicts, and cross-target omission propagation.
`callback_qualifiers.h` proves that plain and unaligned callback types differ on x64/ARM64.
`callback_lookup.h` covers inline callback fields whose named counterparts exist only in another TU,
including equal-layout but distinct scalar and record parameter types.
`inline_callbacks.h` covers structural callbacks, nested results and parameters, C/stdcall target
differences, recursive records, arrays, and pointer qualifiers through RDL, WinMD, and a Rust consumer.

After native compatibility is checked, every same-target record and callback observation must
produce the same public declaration. This additional output check reports name disagreements
instead of selecting a representative's spelling; it does not establish native equivalence.
`public_names.h` and its RDL golden check named scalar and pointer aliases, callback synonyms, and
dependencies through WinMD and compiled Rust bindings. The conflict and ownership fixtures cover
public-name disagreements and publication of shared named dependencies.

Native reconciliation is not a public API compatibility guarantee. Before replacing a production
consumer, compare its header names, RDL, WinMD, and
generated Rust against an explicit output contract. A successful partial SDK run proves only the
accepted subset; omissions and uncaptured macro constants remain outside that claim. New annotations
need both capture and consuming projection support, not just a place to store them.

Public ownership is resolved after compatibility checks. Declarations with one candidate header
stay in its partition; declarations with multiple candidates go to `foundation.rdl`. Ownership
sets are combined across architectures before partitioning. This includes a name defined in one
header on x86 and another on x64/ARM64. `Partition::headers` retains actual candidate source headers,
not a synthetic foundation header. Declarations depending on foundation types keep their own
owners. Missing ownership and ordinary filename collisions remain errors; the foundation stem is
reserved for shared declarations. No per-name header preferences or canonical-metadata routing
are used.

The `foundation` fixtures cover shared scalar aliases, callbacks, records, enums, cross-target
ownership, dependent consumers, input order, and conflicting native definitions. The flat WinMD
namespace is unchanged. Downstream package generation derives a `foundation` namespace from the
RDL filename through its existing remap; there is no new package-generator special case.

The defining typedef for `_GUID` is `GUID`; `BATTERY_CHARGER_ID`, `IID`, and `CLSID` remain aliases.
Declarations such as `_ACTRL_ALISTA`, which defines both `ACTRL_ACCESSA` and `ACTRL_AUDITA`, retain
their native tag as the definition and both public typedefs as aliases. No per-name preferences
are needed for either case. Native layout agreement alone does not establish canonical naming
or ownership parity; inspect the generated diff as coverage expands.

[max-field-alignment]:
  https://github.com/llvm/llvm-project/blob/llvmorg-22.1.8/clang/include/clang/Basic/Attr.td
[clang-c-api]:
  https://github.com/llvm/llvm-project/blob/llvmorg-23.1.3/clang/include/clang-c/Index.h
[clangsharp-extension]:
  https://github.com/dotnet/ClangSharp/blob/0c1c4098a7aab7cb392a2367b75d447bbac05d58/sources/libClangSharp/ClangSharp.cpp

The scaling report separates capture time, merge time, and comparison counts. Owned-byte
estimates cover snapshot capacities, not native memory, allocator overhead, or resolution tables.

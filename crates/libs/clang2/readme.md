# windows-clang2

Unpublished native header capture using libclang. The native IR is independent of RDL and WinMD.

```rust,no_run
use windows_clang2::{Input, Target, capture, merge};

let input = Input {
    name: "example.h".into(),
    source: "struct Record { int count; void* data; };".into(),
    roots: vec!["Record".into()],
    ..Default::default()
};
let snapshots = capture(&[input], &[Target::X86, Target::X64, Target::Arm64]).unwrap();
let merged = merge(&snapshots).unwrap();
println!("{} records with target variants", merged.records.len());
```

Supply the matched `windows-clang-sys` DLL pair through `LIBCLANG_PATH` or its loader search paths.
`capture` returns one owned snapshot per target/TU, in target-major order. `resolve` checks native
record definitions from the same target and compiler configuration, retaining original observations
and completion decisions.

`Input.arguments` supplies include paths and compiler flags. `Input.roots` selects namespace-qualified
declaration names and their transitive dependencies within each TU; empty selects all declarations.
`Input.namespace_headers` optionally limits namespace roots to listed header filenames without
restricting dependencies or global roots. `Input.root_paths` limits automatic roots from
included headers to configured files or directories; `None` permits all headers. The input source,
explicit roots, selected macros and required dependencies remain eligible. Missing roots and
unsupported reachable declarations fail. `merge` reconciles each target, then groups native
identities with separate target layouts and type graphs. An unobserved target is not proof of absence.

Indices are created before parallel capture and reused across TUs. Batches are serialized; other
libclang callers must not create indices during a batch. Native hangs affect the whole process.

Supported: named structs, unions, and integer-backed enums; typedefs, non-template `using` aliases,
forwards, scalars, qualified pointers, arrays, pragma packing, explicit record alignment, and
plain C/stdcall callback prototypes. Written aliases and attributed calling conventions are retained.
Native `clang::flag_enum` attributes emit enum flags without changing the compiler's backing type
or signed member values. Plain enums remain unflagged.
Global class definitions use the same public-field and interface contracts as structs. Native
access controls still apply; constructors, destructors and private fields remain unsupported.
Fixed array extents are compiler-evaluated; fixed and unsized array parameters use native pointer
adjustment, retaining element aliases, constness, and inner dimensions. An absent bound is distinct
from zero length. Incomplete array storage remains unsupported.
Default field initializers remain unsupported, including initializers hidden from cursor traversal.
Native record compatibility uses canonical types. Alias groups retain all name-corresponding
bindings; their public compatibility is checked during projection, not assumed by native merge.
Unresolved attributes block `resolve` and `merge`. No packing request is guessed from layout.
Named and anonymous nested records retain owner-relative identity, including shared types and
out-of-line definitions. RDL preserves their scope through inline fields or nested declarations
and references such as `Outer::Inner`. Nested enums and aliases declared inside records remain
unsupported.
Callback capture checks native function properties before accepting a prototype. Explicit
`regparm`, other calling conventions, and unknown attributes remain
unsupported rather than being normalized to ordinary C. Unconditional `noexcept`, `nothrow`,
and empty `throw()` retain their native distinction and publish an explicit
`NativeExceptionSpecificationAttribute`
on functions, methods, and named callbacks. Other exception specifications remain unsupported.
Variadic imports preserve the compiler's C ABI and emit `...`; reconciliation distinguishes
variadic and fixed prototypes. Variadic callbacks and interface methods remain unsupported.
Native `noreturn` function properties and declaration-owned `[[noreturn]]` remain distinct during
reconciliation and emit `DoesNotReturnAttribute` on imports, methods, and named callbacks.
An unnamed non-returning or non-throwing pointer needs a compatible named callback because a
structural function-pointer signature has no attribute owner.
The native qualifier query retains `__unaligned` only for storage whose canonical unqualified
alignment is already one, including inherited typedef qualifiers. Native comparison preserves the
request; metadata needs no additional alignment modifier. Wider or incomplete storage and other
non-CVR qualifiers remain rejected before interning.
Default type visibility is retained and compared as native evidence. It needs no metadata
attribute for plain records and enums. Interface publication handles its own vtable contract;
C++ RTTI is not projected.
Hidden/protected type visibility and ordinary symbol visibility remain unsupported.

The optional `projection` feature provides `projection::generate(inputs, namespace)` for
three-target capture, merge, and partitioned RDL output. It emits structs, unions, enums, constants,
value typedefs, and named callbacks, checks metadata ABI layout, and rejects unsupported declarations
or ambiguous names. Callback signatures retain calling convention, result and parameter types;
named callbacks preserve declared parameter names. Unnamed signatures have structural function types.
Written value-typedef and callback names are preserved in fields and callback signatures, including
their dependencies. Alias declarations retain canonical storage types. Unnamed function pointers
use compatible callback names resolved across the target, independent of the representative TU.
Without a compatible name they use inline `extern "C" fn(...)` or `extern fn(...)` types, encoded
as ECMA-335 `FNPTR` signatures. No public callback names are synthesized.

Named namespaces remain distinct in native evidence. Local publication uses flat Win32 names and
rejects collisions instead of renaming or merging unrelated declarations. `projection::Options`
can supply authoritative metadata through `References`, separately from import policy. Explicit
namespace wrappers resolve external WinRT ownership; namespace spelling alone does not.
`References::native_interfaces()` supplies the qualified names for `Input.external_interfaces`.
These explicitly external COM definitions retain native identity, attributes, layout and bases,
but their member definitions belong to the supplied metadata. `Record.external` records that
boundary; inconsistent ownership across inputs fails. Publication requires matching UUID and
native interface storage/base evidence. Pointer shape and qualifiers remain checked at use sites.
Incomplete external definitions, by-value use and local inheritance from external definitions
remain unsupported.

Without external capture ownership, references validate non-generic scalar interface/delegate
methods against their native ABI. A configured SDK delegate-name prefix applies only to metadata
delegates. Unsupported external contracts remain explicit omissions in partial mode; identity or
ABI conflicts fail. Neither mode can fall back to a local Win32 declaration.

Scalar constants with conflicting unannotated aliases use their common native primitive type.
Unique written aliases remain preserved. Alias contracts are checked across all observations,
including inputs that do not use the constant. Native type, value, and annotation conflicts stay
errors; this rule does not apply to enum, pointer, record, or callable aliases.

Bare function typedefs also publish named callbacks. Their native types remain functions: an
authored pointer or parameter adjustment becomes the callback value, while additional pointer
levels stay explicit. These typedefs do not supply names for unrelated inline function pointers.

Compiler-provided global aliases retain native observations and compatibility checks, but do not
create public metadata names without an authored declaration. Their underlying types remain usable
through authored aliases such as `va_list`. Native origin comes from the compiler, not file names
or missing source locations.

Incomplete global structs publish empty nominal pointee markers, not native storage layouts.
Pointers preserve their tag identity and qualifiers. A definition in another input supplies the
real record; failed or complete empty definitions cannot fall back to markers. Incomplete records
cannot pass or return by value in callable signatures. Incomplete unions, plain classes, nested
records, and forwards with layout attributes remain unsupported.

Global UUID-bearing class and struct forwards publish GUID constants under their native tag names.
The UUID comes from compiler-decoded attributes, with same-target agreement checked before pruning.
This does not publish a class layout or make the tag usable as a metadata type. Type aliases cannot
rename the constant, and unrelated `CLSID_*` or `IID_*` object declarations still need definitions.

External C-linkage functions retain native signatures, parameter names, and mangling.
`project_with_imports` and `project_partial_with_imports` take `projection::Imports`.
Explicit public-name mappings in `functions` override target-specific native linker mappings in
`symbols`. DLL/export ownership is not guessed from headers. Function `dllimport` and `dllexport`
remain distinct native attributes; neither supplies the DLL identity. The SDK driver reads each
target's pinned import libraries in policy order, applies checked DLL corrections, and preserves
named exports or ordinals. Ordinals survive RDL/WinMD; bindgen support remains a consumer gap.
Native-compatible free-function redeclarations may use equivalent unannotated value typedefs.
Conflicting public types or contracts remain strict errors and all-target partial omissions.
Unresolved import ownership is target-local in partial mode: validated variants remain published,
and omissions identify each target lacking evidence. Strict mode still fails. Missing imports do
not prove target unavailability or excuse native and public contract conflicts.
Single-vptr interfaces support a public non-virtual base and ordered public virtual declarations
with the platform COM convention. GUID-less hierarchies use `#[no_guid]`, including `ID3DInclude`,
without adding COM identity or ownership. GUID-less children retain their native bases without
inheriting a UUID. Bindgen's zero-IID handling for GUID-less COM interfaces remains a separate
consumer issue. Const pointers to GUID-less interfaces remain unsupported. C++ lvalue
reference parameters preserve constness through metadata byrefs. Nonvirtual helpers, member templates,
and method `using` declarations remain provenance, not exported methods or a claim of full C++
compatibility. Const methods and compiler-printed parameter defaults retain explicit metadata
attributes; volatile/restrict and ref-qualified methods remain unsupported.
`__declspec(allocator)` remains checked native profiling evidence, without an inferred ownership
or optimizer contract. The allocator fixtures exercise the required Windows allocation APIs.

Purity remains checked native evidence; virtual method definitions remain unsupported.
Virtual redeclarations can reuse compiler-proven inherited slots. Their parameters, defaults,
and annotations remain in metadata under `NativeOverrideSlotAttribute`; updated RDL and bindgen
reuse the inherited entry instead of adding a vtable field. Covariant and adjusting overrides
remain unsupported. Metadata containing these attributes requires matching readers/generators.

Const integer, enum, boolean, and finite float/double declarations and selected `Input::macros`
use compiler evaluation. Floating-point values retain exact bits, including signed zero.
Enum constants retain their nominal enum or written typedef rather than becoming plain integers.
GUIDs and property/device keys use compiler-evaluated aggregate fields with checked native shapes.
`PROPERTYKEY` and `DEVPROPKEY` retain their distinct metadata types.
Terminated ASCII and UTF-16 arrays publish strings with explicit encodings, preserving embedded
zeros and padding. Typed constant declarations retain their native evidence and reconcile with
definitions in the same or another input. Definitions must agree on type, linkage, contracts, and
value; a missing initializer is distinct from an invalid initializer.
Native and macro constants retain separate identities. Failed macros cannot erase same-named
native constants; conflicts within each identity still fail. Strict publication rejects public
collisions, while partial publication retains the defined native constant and reports the omitted
macro. If the native object has only declarations, partial publication may emit an independently
captured constant macro while reporting the missing native definition. This does not define the
native object; failed native definitions cannot enable macro substitution.
Macros are evaluated after the input and retain native definition ownership and written typedefs,
including selected string and GUID expressions. `Input.macro_headers` discovers object-like macros
in selected header filenames. Isolated compiler probes report invalid expressions without accepting
a failed AST or disrupting unrelated capture. Strict mode fails on rejected candidates; partial mode
reports them as omissions. Diagnostics only identify suspects; each rejection requires an
independent failed parse. Compiler-tokenized empty definitions need no expression parse.
Independent confirmations run on separate, precreated indices. The available CPU count, capped at
24, is divided across targets, with at least one worker per target. Results retain candidate order.
The SDK driver exposes discovery as repeatable `--macro-header=header.h`, or `--sdk-macros`
for the pinned SDK's `um` and `shared` header filenames, including transitive owners, plus the
selected input headers. This includes selected WinRT headers without scanning the whole directory.
The include manifest still determines which headers are parsed. The driver writes its inputs and
resolved imports to `target/win32-2/capture-configuration.txt`; `--dry-run` writes that configuration
without changing RDL/WinMD.
For implicit x86 C imports without ordinary ownership, SDK policy can select a compiler-derived
stdcall alternative after matching its exact linker symbol and provider. Explicit conventions,
parameters, results, and callback types remain unchanged. Original native observations are retained;
there is no global calling-convention override.
Imports, named callbacks, and structural function pointers preserve explicit C/stdcall requests
from Clang's written types on every target. Other prototypes retain their native conventions.
Callback-name selection also checks written conventions in nested signatures. Native evidence and
compiler defaults remain unchanged; real target-dependent contracts remain distinct.
Discovery uses the active definition's owner, not a superseded definition in another header.
Probes do not parenthesize initializers: comma-separated field fragments must not become scalar
constants. Comma expressions already parenthesized in the macro remain supported.
A native static-initializer check requires evaluation without runtime side effects or undefined
behavior. It accepts compiler-supported field-offset expressions without parsing macro tokens.
Successful folding alone is insufficient. This slice does not implement non-finite or extended
floats, arbitrary aggregates, pointer strings, or non-ASCII narrow strings.
Numeric data-pointer constants use compiler-validated static initialization and retain target-width
address bits, pointer typedefs, and pointee constness. Relocations, code addresses, side effects,
undefined behavior, and COM interface constants are rejected.
Publication requires a valid definition for each target. The SDK driver uses separate `INITGUID`
inputs for the selected headers, retaining a lean Windows input for winnt and winsvc definitions.
Main, satellite, and device groups separate repeated, unguarded SDK definition blocks. Ordinary
capture and macro discovery keep their declaration contexts.
All SDK inputs share support headers before Windows headers so constant dependencies are available
and native annotation expansions agree across declaration and definition inputs.
The header/RDL fixtures and SDK breadth test cover all three categories
through x86, x64, and ARM64 WinMD round trips.
The `sdk_functions` fixture covers 114 real SDK imports with explicit DLL ownership, including file
I/O, callbacks, COM activation, registry/security, sockets, cryptography, GDI, timing, virtual memory,
Direct3D/DXGI, HTTP, event logging, service control, and process inspection. It also checks Direct3D
floating-point macros, architecture-dependent socket storage, DirectWrite factory creation, and
Direct2D matrix operations and device/context creation. External C++ inline overloads beside a
selected C declaration retain names, identities, and locations outside the exported ABI.
Non-inline overloads and C function definitions remain unsupported.
The `sdk_interfaces` fixture covers storage, imaging, properties, monikers, shell, Direct3D/DXGI,
DirectWrite, and Direct2D geometry dependencies, including UUIDs, inheritance, method order, and
exception contracts. COM object constness remains native
evidence but does not change the published interface reference; const on pointer storage is retained.
Interface typedefs publish as transparent aliases. The full DirectWrite factory closure includes
font collections, font faces, text analysis, formatting, and layout. Direct2D coverage reaches
`ID2D1Factory7` and `ID2D1DeviceContext6`, including effects, printing, imaging, and SVG, plus WIC
factory2 and raw-image development. DirectWrite coverage reaches `IDWriteFactory7` and
`IDWriteFontSet3`.

Virtual overloads retain their native names and compiler-owned vtable slots; declaration order is
not the MSVC slot order. Ordinary value inheritance publishes one public nonvirtual base as a
`base` field with `NativeBaseAttribute`, after checking the complete derived layout. Inherited
records are supported through pointers and parameter references, not by-value callable signatures.
Multiple, virtual, or empty bases and unrepresentable layouts remain explicit errors.

Deprecation messages and replacement names retain explicit `NativeDeprecatedAttribute` payloads.
Empty payloads and enum-member annotations survive metadata round trips. Relocated typedef
annotations retain their original owner; conflicting deprecation payloads are rejected.

Imported prototypes may differ in parameter names and transparent value typedefs when their resolved
types and annotations agree. Comparison preserves nominal types, qualifiers, callbacks and
typedef-owned annotations, including contracts from other translation units.
Selection prefers a complete observed prototype with more authored names, then lexical declaration
order. The emitted prototype retains that observation's written typedef names.

The SDK driver retains SAL source-hook evidence in native observations. Simple parameter direction
and optionality contracts share semantic normalization across reconciliation and publication,
including the SDK's corresponding SAL1/SAL2 forms. Direct named buffer capacities normalize to
parameter indices with byte/element units. Reserved parameter contracts emit `#[reserved]` for the
zero/null requirement without inventing direction or optionality. These contracts emit canonical
flags and count attributes rather than duplicate `NativeSalAttribute` payloads.
Seven direct RPC SAL aliases share this normalization. RPC input forms emit
`NativeInput(ReadOnly = false)` for valid input on entry; direct SAL1/SAL2 input forms emit
`ReadOnly = true` for their additional read-access contract. Pointer arguments are non-null unless
optional. Neither value changes native qualifiers or implies a count, and false does not require
writable storage. Composed contracts retain raw evidence.
By-value scalar and enum parameters omit `NativeInput`, including through typedefs, while retaining
input direction and native SAL reconciliation. Pointer-backed handles, pointers, references, COM
objects and records keep their input attributes.
RPC dereferenced-pointer forms lower outer direction and optionality without confusing inner-pointer
nullability with optional argument storage. RPC output-pointer aliases, four SAL1 aliases, and ten
SAL2 out-pointer forms replace fully represented evidence with `NativeReturnedPointerAttribute`:
result nullability, one readable pointee on success when non-null, and COM's additional
null-on-failure guarantee. Ordinary null-on-failure forms retain the same failure guarantee without
acquiring COM IID-output markers.
The contract requires native pointer-to-pointer storage and specifies no ownership or allocator.
Explicit COM output-pointer forms also emit `#[iid_is]` for unqualified native `void**`, including
typedef aliases. Ordinary out pointers do not supply this marker. Compiler-bounded leading parameter
comments preserve explicit MIDL `[iid_is]` and `[retval]` result markers without guessing a GUID
association. Other MIDL comment contracts remain unimplemented source evidence.
Input/inout inner-pointer contracts remain evidence. Returned-buffer forms lower outer output
direction and argument optionality without treating the returned allocation as caller capacity.
Direct RPC input strings replace native evidence with typed input and entry termination contracts.
RPC inout strings retain evidence; optional inout strings require termination on entry, without
promising it on success.
RPC buffers also lower direct element capacities and successful initialized lengths independently.
Fully represented RPC output arrays use typed capacity and initialized-length attributes
instead of raw evidence, including the guarantee that capacity remains writable on success.
Full inout arrays retain their initialized length without acquiring that extra writable guarantee.
Direct/full RPC input arrays use typed input and element-count contracts, including optional and
counted-string forms. RPC input contracts do not acquire SAL1/SAL2's additional read-access guarantee.
Partial input readable lengths remain entry evidence; output/inout `xcount` arguments do not become
counts.
Single- and double-NUL termination record distinct entry and successful-output conditions without
inferring an encoding. Raw pre/post qualifiers bind before reconciliation; bare properties have no
inferred phase. Readers must support the double-NUL properties on `NativeNullTerminatedAttribute`
and ordered `NativeSalAttribute.Modifiers` for residual scoped evidence.
Initialized length records a separate
integer parameter or one dereference, with byte/element units; it is not substituted for capacity.
The bounded `p ? *p : n` form records `n` as a fallback when the count pointer is null. The condition
and dereference must reference the same pointer, and the fallback must be a by-value integer.
Other conditional expressions remain explicit evidence.
Full-output/update contracts use the same direct integer count for capacity and initialized length.
Success predicates and count-pointer optionality remain separate contracts. This is not a complete
SAL interpreter; unsupported expressions and nested contracts retain `NativeSalAttribute` evidence.
Published nested evidence uses SAL names and expanded arguments, not internal annotation transport.
Native observations retain the original strings for reconciliation; conditional scopes remain
evidence rather than unconditional contracts.
Direct integer sibling-field counts use `NativeFieldBufferAttribute`, preserving writable capacity,
optional readable extent, byte/element units, and nullable storage. These describe a valid containing
value, not call pre/postconditions. Expressions, bitfield counts, and member paths remain evidence.
Direct named integer buffer capacities emit
`size_param` for bytes and `len_param` for elements, using the declared parameter index.
Other expressions retain SAL evidence without
an inferred count relationship. Typedef publication accepts an observed annotation
set that contains all other observations, but never invents a union of incomparable sets.
Callback parameter names and contracts belong to declaration observations, not shared ABI types.
ABI-based callback naming excludes annotated and indirect prototypes. Defining-typedef annotations
retain their original owner through `NativeSalAttribute.TypedefName` when published on a record or
enum; inherited callback typedef contracts use the same ownership marker.

All same-target record and callback observations must agree on their public names after native
compatibility is checked; conflicting names are errors, including in partial mode.
Requested packing is emitted as `#[packed(N)]`, including requests with no visible layout effect.
Complete-record alignment is emitted as `#[align(N)]`. Native checks retain every requested
alignment and its GNU, standard, or declspec form, even when layout is unchanged. Projection uses
the maximum request; zero adds no alignment attribute. Member alignment uses field-owned
`AlignmentAttribute` and requires an updated generator. Public field types remain unchanged;
Rust storage uses uninitialized padding and the required containing alignment. By-value callable
uses of these records remain rejected, including containing records and arrays. Ordinary pointers
and adjusted array parameters remain supported. Typedef/enum alignment, aligned bitfields,
combined packing/alignment, and packed records containing aligned types remain unsupported.
Unions retain their record kind, publish explicit-layout fields at offset zero, and must match
captured size and alignment. Structs and unions never share a public family, even with identical
fields and layouts.
Integer, bool, and enum bitfields retain evaluated widths and native offsets. Projection coalesces
compatible members into checked storage fields, preserving unnamed padding and zero-width
barriers in native evidence. Union members use separate offset-zero storage fields.
`NativeBitfieldAttribute.MemberType` preserves mixed signedness, bool and enum types, written
typedefs, and single-bit integer semantics independently of storage. Typed metadata requires updated
readers; untyped attributes retain their existing interpretation.
Storage layouts must match native size, alignment, and offsets; no packing request is inferred.
Downstream union-accessor defects are tracked separately rather than blocking correct metadata.
Enums retain nominal identity, scopedness, exact signed/unsigned values, and backing types.
MIDL-generated anonymous-enum tags remain in native evidence, but publication uses an authored
typedef when available or emits loose constants instead of the generated name.
Forward declarations must agree with complete definitions. Projection preserves `#[scoped]` and
the integer representation; it does not infer flags or combine distinct enums by matching values.
Anonymous enums without a public type name publish their members as typed constants. Same-target
constants must agree in native backing type and value; architectures can retain different variants.
Type and constant names occupy separate publication namespaces. Typedef-named enums remain enums;
references to otherwise unnamed enum types are rejected rather than replaced with integers.
Mixed const/mutable pointer chains retain per-level storage qualifiers through RDL and WinMD;
COM object constness follows the publication rule above.
`char16_t` and `char32_t` project to `u16` and `u32`, while native reconciliation keeps their
identities distinct from other equal-sized integer types.
By-value aliases can connect distinct native records only after their definitions are checked.
A single native record or enum uses its unique defining value typedef as its public name; with
zero or multiple defining typedefs it retains its native tag. A multi-record family requires a
unique common defining typedef, or a unique common alias when none defines a tag. Ambiguity is an
error. Other reachable value names remain aliases. Pointer aliases cannot establish equivalence.
Header ownership prefers the unique alias header containing its native definition, otherwise it
retains all declaration headers as candidates. A single owner keeps its header partition; multiple
owners use `foundation.rdl`. Ownership evidence is combined across architectures before routing,
and `Partition::headers` retains the real source-header set. Native identities and source
observations remain distinct; foundation routing never resolves a type conflict.
`tool-win32-2` supplies the SDK configuration and writes canonical files.

`capture_partial` and `projection::generate_partial` return supported output plus omission reports.
`projection::project_partial` accepts an existing partial capture without reparsing.
Successfully captured observations undergo native reconciliation before failed entities and their
dependents are removed across targets. Known conflicts remain fatal even in excluded dependents.
Dependencies unreachable from surviving roots are then removed across inputs and targets. Nested
declarations remain with their surviving owner. This does not suppress conflicting observations
or turn unsupported declarations into compatible ones.
Failed observations are reported as omissions, not validated as compatible. A non-value typedef,
such as `void`, can remain as native evidence and expand at use sites. A named value alias or callback
that cannot be published is removed with its dependents rather than replaced by its canonical type.
Failed macro probes exclude macro values across targets, not observed native functions with
the same spelling. Native declaration failures and known-conflict checks retain their own scope.

Selected object-like macros can also expose function aliases. A compiler-proven direct function
reference preserves its native target, signature, parameters, and annotations. Published aliases
use the native target's DLL/export mapping; publication does not infer targets from macro spelling
or copy baseline signatures. `projection::Options::function_aliases` can restrict publication by
target and name without changing native declarations or reconciliation. The SDK driver selects
required compatibility aliases explicitly rather than publishing every discovered alias.
Pointer expressions, casts, and calls are not aliases.

Fixtures, inspection, and scaling live in `test_clang2`. See
[crate notes](../../../docs/crates/windows-clang2.md) for commands.

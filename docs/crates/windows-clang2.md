# windows-clang2

`windows-clang2` is an unpublished, side-by-side prototype for replacing `windows-clang`. This
slice implements native capture, checked resolution, and a narrow RDL projection. It drives
`tool-webview` and the animation profile in `tool-bindings`; the full Win32 and WDK scrapers still
use `windows-clang`.

See the [crate readme](../../crates/libs/clang2/readme.md) for a small API example.

## Viability decision

Proceed with `windows-clang2` as the replacement architecture. The bounded consumer and header
evidence supports that choice, not full Win32/WDK replacement approval. The owned native graph,
checked cross-TU resolution, and separate RDL projection have supported real SDK declarations
without building on the legacy extractor's output. WebView and Animation use the new path in their
actual binding generators. No architectural blocker has been identified in these workloads.
Remaining canonical profile/name reconciliation, unsupported projection families, and resource
budgets are replacement gates, not exceptions to hide.

Further work should implement missing source-to-RDL cases, not repeat downstream runtime
validation. Use small `.h`/`.rdl` fixtures, compiler-reported evidence, and complete real-header
inventories. Keep unsupported contracts visible; a declaration count alone does not establish
fidelity. Native execution is reserved for an ambiguity that source/compiler evidence cannot settle.

The audio inventory exposes declaration-only data as remaining work. Output-pointer and nullable
result contracts survive as source annotations without guessing typed metadata semantics. General
C++ projection, broader SAL/MIDL coverage, and full Win32/WDK cutover remain outside this subset.

### Pre-scale critical review

The five source-only review findings have targeted fixes and regression fixtures. Annotation
preservation is independent of typed lowering; unknown captured contracts remain in RDL.

| Area | Implementation | Boundary |
| --- | --- | --- |
| SAL capture | `src/sal.h` instruments SDK source markers and combinators. | SDK capture without the adapter rejects. |
| Annotation ownership | Exact source-position index and shared callable contexts. | Inherited evidence keeps its original context. |
| Per-root projection | Immutable root/group/name indices and reusable `Projection`. | Names, dependencies, and collisions remain plan-local. |
| UUIDs | Parse actual UUID attributes from terse compiler-rendered declarations. | Deprecated messages and macro argument order cannot supply guessed UUIDs. |
| Macro constants | Const-reference value probes; integer fallback only for pointers. | Aggregate constructors and dynamic values still reject. |

One optimized x64 synthetic run used independent fixed-prototype functions with one integer
parameter each. The annotated case adds `annotate("_In_")` to every parameter. Rust build time is
excluded; the per-root column assesses every root individually after one capture/resolution:

| Functions | Plain capture | Annotated capture | Annotated resolution | One combined projection | All per-root projections |
| --- | --- | --- | --- | --- | --- |
| 1,024 | 30 ms | 31 ms | 1.2 ms | 1.6 ms | 1.3 ms |
| 2,048 | 37 ms | 50 ms | 2.4 ms | 3.5 ms | 3.0 ms |
| 4,096 | 59 ms | 97 ms | 4.7 ms | 9.7 ms | 6.3 ms |

The review baseline at 4,096 annotated functions was 1,772 ms for capture and 230 ms for all
per-root projections. These measurements isolate indexing costs; they are not an SDK throughput or
memory budget. Peak memory has not been measured.

Keep the owned native graph, checked resolution, and closed projection plan. Written types and
canonical types serve different contracts; separate observations prevent missing evidence from
hiding conflicts. Deterministic output and fail-closed unsupported outcomes are useful, not cleanup
targets. Do not add another general IR, a global projection cache, or a parallel capture framework
to solve the measured indexing problems.

Larger real-header runs should establish peak memory and multi-TU resource
budgets. Parse reuse between discovery, capture, and macro probing is another optimization
candidate. Streaming TUs, broad interning, and container replacement are deferred until those
measurements justify their complexity.

## Current pipeline

```text
Inputs + compiler arguments + named roots
    -> libclang translation units and declaration index
    -> owned native dependency graph, including cross-TU observations
    -> checked native groups, completions, and annotation evidence
    -> closed projection plan or diagnostic
    -> RDL
```

| Module | Responsibility |
| --- | --- |
| `capture.rs` | Owns libclang handles, indexes declarations, and captures selected evidence. |
| `native.rs` | Private native model with explicit typedef edges and incomplete/unavailable states. |
| `validate.rs` | Checks available evidence with an iterative node-pair worklist. |
| `project.rs` | Applies explicit projection policy and renders only planned items. |
| `project/record.rs` | Checks struct/union storage and lowers direct unnamed record fields. |
| `lib.rs` | Experimental input, snapshot, report, and diagnostic inspection API. |

There is no dependency on `windows-clang` in the library. The comparison test crate depends on both.
The native model is private so the prototype can change it without maintaining a second public
projection model.

## Root selection and identity

Callers select qualified native names, such as `API::Packet`. Capture indexes the inputs before
decoding selected native dependencies and their matching observations. Roots can be
functions, records, aliases, callbacks, enums, variables, or selected object-like macros. Unknown
roots are errors. Capture supports more kinds than the current projection.

Canonical cursors identify entities within one TU. Each source declaration remains a separate
observation, with its own written dependencies, parameter names, and annotation context. A redundant
declaration cannot replace earlier evidence. Original USRs are retained and find cross-TU
candidates, but do not establish agreement.

Typedef USRs include source filenames even for namespace-level aliases. Alias candidates therefore
use their qualified native names. Internal-linkage and anonymous-namespace entities stay TU-local.
This is candidate matching, not permission to accept different underlying types.

The current comparison is conservative: named type identity, native scalar kind, qualifiers,
reference kind, and callable shape must agree. Alias declarations and callable signatures also
retain canonical types so spelling differences do not require identical typedef paths. Written
dependencies from every observation are checked independently. This is not a general C++ ODR
checker or ABI-equivalence solver. Unnamed record members match by their checked owner's
candidate identity and direct field slot, not by USR alone: sibling anonymous unions can share a
libclang USR. General unnamed-type identity across different files is not inferred.

### Translation units and compilation profiles

A capture accepts any nonempty collection of uniquely named TUs, not a fixed SDK/WDK pair. Each TU
has independent preprocessing state. All share one compiler argument list and must resolve to the
same target. Source preludes can supply different defines and include sequences; per-input compiler
arguments and merging independently captured snapshots are not implemented.

| Situation | Resolution contract |
| --- | --- |
| A common header appears in many TUs | Retain each observation, check candidate agreement, and emit one closed declaration group. |
| A dependency is selected only through a root in another TU | Capture its matching observations from all indexed TUs, including written typedef edges. |
| One TU has a forward declaration, another a definition | Use a checked completion without discarding the forward observation's UUID or annotations. |
| Include order or macros change a matched type or contract | Reject with source and TU provenance; neither order nor priority suppresses the contradiction. |
| Shared records have unsupported evidence in one TU | Reject the affected closure; a supported observation cannot certify the unsupported one. |
| Declarations have internal linkage or anonymous-namespace scope | Keep TU-local identity rather than merging unrelated entities by name. |
| Multiple observations have equal evidence rank | Choose the first lexicographic spelling-file path, byte offset, and TU name. All observations still participate in agreement. |

Representative selection prefers the strongest evidence before its provenance tie-break. The same
rule chooses completion candidates and comparison anchors. Parameter presentation and merged
annotations have separate policies; choosing a representative is not an override mechanism.
Source partition ownership also remains separate from TU grouping.

Mutually exclusive native configurations must be assessed separately until their intended metadata
representation is specified. Splitting an incompatible include set into TUs can remove a compiler
name collision, but cannot make contradictory declarations agree. Missing prerequisites should be
corrected in an explicit input profile, not hidden by selecting a higher-priority TU.

`translation_units` fixtures include the same physical headers in 1, 4, 16, and 64 TUs, retain exactly
four observations per TU, and check four declaration comparisons per additional TU for that graph.
The RDL and source partitions are identical, and metadata readback checks the dependency and callable
types. Other cases cover many forward-only inputs, reversed input order, renamed TUs, and
include-order or annotation conflicts. These bounded checks are not an SDK-wide memory budget.
Capture retains all live TUs until dependency observations are owned. The header runner's shared
main/satellite construction is not a general profile manifest; full combined SDK/WDK profile
agreement and large multi-TU resource limits remain production gates.

The pinned SDK `winternl.h` exposes a subset of the WDK `wdm.h` `FILE_INFORMATION_CLASS` members.
The SDK/WDK profile fixture compiles each source independently to metadata, checks that every shared
member has the same value, and verifies that strict combined resolution rejects the different
complete enums. This is a real publication boundary, not a parser failure. Canonical metadata needs
an explicit source-profile reconciliation rule for such extensions, with evidence of compatible
representation and common members. No automatic enum union, profile priority, or SDK/WDK snapshot
merge is implemented in clang2.

The bounded profile inventory compares enum tags defined in SDK `winternl.h` and WDK
`wdm.h`/`ntifs.h`, under the same x64 compiler arguments. Each profile passes native resolution
independently and compiles its projected enums to WinMD. The checked-in outcome table records both
the emitted member relation and the result of strict combined native resolution:

| Native enum | SDK members | WDK members | Emitted member relation | Combined native result |
| --- | ---: | ---: | --- | --- |
| `_FILE_INFORMATION_CLASS` | 1 | 84 | SDK is a subset with matching common values. | Rejects different complete definitions. |
| `_KEY_SET_INFORMATION_CLASS` | 7 | 8 | `MaxKeySetInfoClass` changes from 6 to 7. | Rejects the value conflict. |
| `_OBJECT_INFORMATION_CLASS` | 2 | 2 | Identical members and representation. | Agrees. |

The key-set change is present in the original headers: WDK adds `KeySetLayerInformation` before the
terminal member. A larger member count does not make a source authoritative, and a `Max*` spelling
does not permit rewriting its value. An extension rule must keep common-value conflicts visible and
must check native representation, annotations, and dependent contracts before publishing. This
three-enum inventory is not full SDK/WDK reconciliation or a general C++ equivalence check.

## Captured evidence

| Surface | Evidence |
| --- | --- |
| Builtins | Native kind, size, alignment, and qualifiers. |
| Indirection | Typedef edges, pointer levels, and distinct lvalue/rvalue references. |
| Arrays | Constant extent or explicitly incomplete extent. |
| Records | Kind, completeness, layout, fields, bit widths, simple bases, and method signatures. |
| Enums | Completeness, scopedness, explicit Clang/Windows flag markers, underlying type, and member values. |
| Callables | Prototype kind, calling convention, exception specification, result, and parameter types. |
| Annotations | Ordered SAL and bounded MIDL payloads, declaration/member scope, parameter contexts, and locations. |
| COM properties | Declaration-local MIDL `propget`, `propput`, and `propputref` markers, checked across observations. |
| Variables | Written and canonical types, supported initializer values, or explicit unavailable evidence. |
| Provenance | Input name, compiler arguments, resolved target, USR, and spelling location. |

Annotation comparison binds parameter identifiers in argument expressions to their original
positions. Missing annotation evidence does not contradict present evidence, but two nonempty,
different sequences from the same source family conflict. SAL and MIDL evidence are checked separately
across all observations, not just against one anchor. Explicit SAL direction controls local calls;
MIDL supplies direction only when SAL has none. Conflicting MIDL observations still fail when SAL
supplies the final direction. The SDK's `IDispatch::Invoke` has `_In_ DISPPARAMS*` alongside
`/* [annotation][out][in] */`; these describe different contracts, not conflicting SAL declarations.

MIDL recovery accepts contiguous prefix block comments immediately after a parameter delimiter.
Complete bracket groups such as `[in]`, `[out]`, and `[in,out]` supply direction. Trailing comments,
prose, and nested callback parameter comments do not attach to the outer parameter. Parameterized
groups, relationships, optionality, and retval markers are preserved without typed lowering.
Balanced nested brackets and quoted delimiters are retained. Callable-prefix comments inside the
declaration extent are also captured. Comments outside these ownership boundaries are not recovered;
this is not a general MIDL parser.

`#[annotation("sal", "...")]` and `#[annotation("midl", "...")]` preserve captured contracts on
declarations, methods, fields, enum members, and parameters. Each source family has one ordered
payload per scope. Parameter references use `$0`, `$1`, etc.; method positions exclude `self`.
Quoted literals and member names are not rewritten. This lexical binding is not C++ name lookup.
Understood contracts also receive typed metadata attributes; the raw payload remains.

Force-include `crates/libs/clang2/src/sal.h` before SDK headers. It disables strict wrappers that
erase source markers, then instruments the SDK's SAL marker families and compositional operators.
The legacy scraper has a separate adapter. This does not establish exhaustive coverage of every
SDK/WDK annotation macro or arbitrary C++ attribute. New families need source fixtures, not silent
defaults. Annotated same-name record aliases and anonymous callables outside record fields still need
additional representation; unsupported cases reject rather than discard their contracts.

Record field enumeration uses `clang_Type_visitFields`, including implicit anonymous struct/union
members. The graph preserves direct field order, native offsets, member types, and nested layouts.
Anonymous member names are empty rather than libclang's filename-dependent synthesized spelling.
The checked owner and field slot distinguish sibling anonymous records even when their USRs match.

Method evidence includes virtual/pure/static/const flags, reference qualifiers, and compiler-reported
overridden-method USRs. Virtual bases,
multiple or data-bearing inheritance, constructors/destructors,
template specializations, member pointers, and unsupported initializer expressions are reported as
unavailable evidence. Non-ABI member templates are not modeled. Available record fields remain in
the graph when another member is unsupported.

Fully specified struct and array initializer lists retain nested values evaluated by Clang,
including scalar conversions. They are native aggregate observations, not GUID-specific capture
records. Partial lists, dynamic expressions, and copied-record initializers are rejected instead
of filling missing values. Nested value differences participate in ordinary cross-TU agreement.

Selected object-like macros use C++ expression probes. Capture reads the initializer's written type,
not the probe's deduced `auto` type, so a direct or indirect interface-pointer cast keeps its typedef
chain. A separate integer probe evaluates pointer bits when needed. Function-like macros, arbitrary
macro expressions, and poison-expression recovery are outside this slice. `discover` inventories
declarations and macro definitions by exact Clang file identity, without choosing export policy.
It uses expansion locations, so a macro-generated declaration belongs to its invocation header.
Discovery is a separate parse pass; selected names then enter ordinary capture and resolution.

Discovery distinguishes declaration fragments from values using macro definition tokens.
Linkage fragments beginning with `extern`, standalone calling-convention keywords, bare `noexcept`,
and declaration braces are reported helpers. Leading identifier chains propagate a known attribute
or declaration fragment through object-like wrappers, with memoized results and cycle detection.
Only a single-identifier body can alias a native type or function. `noexcept(expression)`, string
literals containing keywords, arithmetic, and unknown prefixes still reach ordinary value probing.
`declaration_macros.h` and its RDL fixture cover the helper boundaries alongside emitted constants
and native functions. The header runner records exclusion reasons rather than listing helper names.

Only generated value-probe cursors receive macro identity and macro source ownership. A native
record or variable with the same name keeps its own identity, location, and written type. When
a selected macro shadows a native declaration, its probe supplies the root; native declarations
remain available as dependencies. The initializer comes from Clang's variable-initializer API, not
the first expression child. Declaration-only values still reject, while initialized values and
dependency records retain their source contracts. `macro_identity.h` covers these cases across TUs,
including conflicting macro values backed by distinct internal-linkage variables.

## Agreement and completion

Every captured declaration is checked for unavailable evidence, including graphs with only one TU.
No unsupported type is accepted merely because another declaration has the same unsupported reason.

All matched complete definitions must agree. Incomplete observations keep their available facts,
including enum representation, and can refer to a checked completion in another TU. An unrelated,
unselected declaration is not admitted merely because it conflicts elsewhere.
UUID evidence is retained on forward record declarations. All supplied UUIDs in a native group
must agree, including when the chosen complete definition has no UUID.

The report separately lists records and enums with no captured completion. A successful report with
an incomplete list is not proof of complete native definitions. Variables without initializers
retain that absence; it does not override or hide contradictory observed constant values.

Comparisons expand each new declaration pair once, check their local facts, and queue their type
dependencies. Recursive assumptions are published to the local completed-pair cache only after the
whole comparison succeeds. The cache is confined to one validation call; failed validation does not
leave state on the snapshot.

`Snapshot::resolve` returns a checked `Resolved` view containing the representative for every
observation, all groups, and merged annotation evidence. `Snapshot::validate` returns just the work
report. Projection consumes the checked view rather than repeating candidate resolution.

Captured data owns no libclang pointers. Validation and diagnostic inspection work after the
translation units and loader guard are dropped. An existing caller-owned thread-local loader
registration is preserved.

## Projection boundaries

`Resolved::project` accepts a destination namespace, an optional import library, explicit export
names, and external type bindings. It returns an owned `Plan`; `Plan::rdl` needs neither the
snapshot nor libclang. The renderer performs no native type classification or dependency discovery.
The default is to preserve source contracts, not to rewrite them for downstream convenience or
match legacy output. Native enum signedness, typedef names, and annotation evidence remain separate
from caller-supplied external bindings.

`Resolved::project_roots` projects a subset of captured root names without reparsing. The entire
captured graph must pass resolution first; this API cannot hide unsupported native evidence or
cross-TU conflicts. Every subset still includes its output dependencies. Successful individual
plans do not prove that a combined plan is valid: alias and output-name conflicts remain errors.
For repeated assessments, create `Resolved::projection(&options)` once and call
`Projection::project_roots`. It reuses binding validation and immutable lookup indices, not
root-dependent projected items.

`Plan::rdl_by_header` partitions the closed plan by source ownership. Complete definitions outrank
forward declarations, and equally complete observations use the first lexicographic source path.
A selected alias that names a record owns that output declaration. Dependency headers receive
their own partitions. Ownership is separate from native spelling locations and identities; the
renderer does not reopen the graph. Declarations without a source file, such as command-line macro
definitions, cannot be header-partitioned.

| Surface | Current policy |
| --- | --- |
| Records | Nonempty structs with natural or increased member/record alignment; verify every offset, final size, and alignment. |
| Enums | Complete integer-backed enums; preserve native width, signedness, names, values, and explicit Clang/Windows flag markers. No value-based flags inference. |
| Local dependencies | Checked complete records and nominal incomplete C records; unknown by-value layouts reject. |
| Functions | Fixed prototypes, supported Windows calling conventions, compiler link names or caller-bound DLL export names. |
| Callbacks | Named fixed prototypes with checked parameter contracts, calling convention, and pointer depth. |
| Parameters | Positional names for functions, native names for methods; supported SAL below. |
| External value types | Explicit native record/enum bindings. Record storage retains native layout; by-value external record calls remain rejected. |
| External scalar typedefs | Explicit value bindings; retain the checked native scalar representation and layout. |
| Pointer-sized integers | Explicit typedef contracts; check native signedness, pointer size, and alignment before emitting `isize` or `usize`. |
| External pointer typedefs | Explicit pointer-sized value bindings; preserve the alias boundary and native pointee constness. |
| External interfaces | Bind a native record; consume exactly one native pointer/reference level. |
| Local interfaces | UUID-bearing, fieldless records with pure virtual system-ABI methods and at most one base. MIDL `propget`/`propput` become `#[special]` on unchanged native method names. |
| Aliases | Preserve annotated scalar, pointer, record, and array aliases at uses. Unannotated aliases can name selected records. Competing selected names reject. |
| Constants | Scalar, raw pointer, and string literals, plus GUID/property-key aggregate storage shapes; omit interface values with a reason. |
| Raw pointers | Collapse mixed mutability to const if any raw-pointer level is const. |

External bindings are trusted metadata contracts, not native ABI proof. All captured native
dependencies must agree before an external binding can suppress local output. Metadata readers
still need the referenced WinMD files when compiling RDL. Scalar typedefs may bind to external
metadata value types through `ProjectionOptions::references`. The native canonical type must be a
supported non-void scalar with matching builtin layout; the caller guarantees the external type has
that same scalar representation. Pointer typedefs may also bind to external metadata value types;
the caller guarantees the pointer representation and semantic identity. The bound value does not
schedule its pointee for local output. Array, reference, and other arbitrary alias bindings are
unsupported.

`ProjectionOptions::pointer_sized` binds native integer typedef names to `PointerSized::Signed` or
`PointerSized::Unsigned`. The policy checks every captured observation, not just the representative.
The same name cannot also have an external metadata binding. The planner follows written alias
chains and uses the existing RDL `isize`/`usize` primitives in fields, parameters, results, arrays,
and constants. Integer SAL length and written-byte checks accept both types. Unbound fixed-width
integers remain fixed-width, even when their size matches a pointer.

An annotated intermediate alias retains its declaration and type-use identity. Its contract must
agree with the requested pointer-sized type; an outer binding cannot erase the annotation to force
a different representation. `pointer_sized.h` covers x86, x64, and ARM64 source-to-RDL output.
`pointer_sized_contract.h` covers invalid bindings, conflicting observations, and annotated chains.

Fixed-size arrays preserve their declared element type and extent, including nested arrays and
arrays of records. Array storage participates in the same native offset, size, and alignment checks
as scalar fields. Externally bound record fields retain their checked native storage layout under
the caller's metadata ABI contract. Neither change relaxes the call gate: arrays and local records
containing adjusted layouts or external records are also rejected in unproven by-value calls.

`ProjectionOptions::imports` maps exact native linker symbols to `FunctionImport { library, target }`.
`ImportTarget::Name` and `ImportTarget::Ordinal` preserve the import library's distinction; RDL
represents the latter as `#[library("library.dll", ordinal = 17)]`.
The native evidence and calling convention are unchanged. Unmapped functions require the explicit
`library` fallback, which retains their compiler name and is suitable for static fixtures, not
proof of DLL export spelling. The BCrypt caller disables that fallback and loads target-specific
SDK import libraries through `windows_rdl::implib`. The reader obeys each COFF name type; it does
not strip decoration based on the spelling alone. Ordinals remain distinct from names, and code
imports remain distinct from data imports. Missing required imports fail projection.

UUID-bearing forward class declarations emit `#[guid(...)] class Name;`, not a fabricated GUID
variable, empty object layout, or WinRT runtime class. The native report still marks the class
incomplete. Object use remains rejected without a captured definition. UUID evidence is combined
across checked declarations independently of which complete definition supplies members.

`ProjectionOptions::class_guids` selects GUID-constant publication for these opaque classes and
supplies the trusted metadata GUID value type. It requires a value binding, retains source
ownership, and uses the checked compiler UUID. It does not create a missing object definition,
infer a class factory, or supply a value for a declaration-only data symbol. Animation uses this
policy for the native activation class identities; generic inventories retain the default class
representation.

Record storage can include gaps required by increased member alignment. Each explicit gap is a
union of a byte array and a zero-length array, not an ordinary initialized byte field: native
copies need not initialize padding. Padding names avoid native field-name collisions. Increased
record alignment uses `#[align(N)]`; packing and reduced alignment still reject. A closed-plan
check rejects by-value calls involving an adjusted record, including nested records and aliases.
Matching storage layout does not prove its calling ABI.

Complete unions use the maximum member size and natural alignment, with every compiler member
offset required to be zero. Increased alignment uses the same checked alignment attribute as
structs. Unnamed records in direct fields lower recursively to the existing nested RDL form, rather
than acquiring guessed global names. Implicit anonymous fields receive `Anonymous{index}`, with
underscores appended to avoid all native and generated field names. Named fields of unnamed types
retain their field names. Source annotations stay attached to the nested type or field.

| Shape | Storage policy | Calling policy |
| --- | --- | --- |
| Ordinary struct | Exact compiler offsets, size, and alignment. | Existing ordinary by-value support. |
| Union | Exact compiler size/alignment; all member offsets zero. | Pointer use only. |
| Direct anonymous struct/union field | Checked recursively; existing nested RDL representation. | Enclosing by-value calls reject. |
| Increased field/record alignment | Explicit padding and alignment attributes. | Adjusted by-value calls reject. |
| Reduced alignment or displaced packed fields | Reject rather than expose misaligned field access. | Unsupported. |
| Bitfields | Reject; mixed signedness needs a representation contract. | Unsupported. |

`layouts.h` and `layouts.rdl` cover nested structs/unions, named anonymous-record members, arrays of
named unions, explicit union alignment, generated-name collisions, and nested source annotations.
Semantic metadata checks run on x86, x64, and ARM64. `layouts.cpp` supplies MSVC size, alignment,
member-offset, and pointer-mutation evidence in both call directions on x86/x64. Native ARM64
execution remains open. `layouts_rejected.h` keeps reduced alignment, packed field placement, and
bitfields rejected, including inside nested records. Function parameters, results, and callbacks
keep the by-value rejection gate.

Selecting an alias for a direct anonymous member cannot remove the enclosing record's ABI gate.
That fact is retained from native field ownership, not inferred from the output type spelling.
`anonymous_by_value.h` covers that selected-alias path.

One explicitly selected record alias owns the output name unless the named tag is also selected.
This preserves `POINT`/`RECT` and permits `decltype` to select real nested WDK member types without
copying their declarations. Native candidate identity
still uses the checked owner and field slot, including fields containing arrays or pointers to
unnamed records. Multiple selected aliases to a named record retain that native definition and emit
transparent aliases; none acquires ownership by traversal order. Multiple aliases to an unnamed
record still reject without a publication identity. Interface aliases use the existing native-typedef
representation, preserving COM identity rather than cloning interface declarations.
Anonymous arrays, pointer targets without a public identity, and general
C++ record layouts remain outside the direct-field slice.

Callable projection retains written typedef paths, not just canonical ABI types. For example,
explicitly binding native `HRESULT` to `Windows.Win32.Foundation.HRESULT` preserves error and COM
query-result semantics in generated Rust; unbound `long` remains `i32`. The planner has no built-in
Windows typedef names. Every callable observation and alias chain must agree on projected contracts.
A bound typedef versus its unbound primitive is a projection conflict even when native resolution
accepts them as ABI-compatible. Successful alias projections are cached within one builder.
Destination namespaces with multiple segments render as nested RDL modules.

Pointer typedef contracts are distinct from raw pointer runs. For example, a bound `LPCWSTR*`
becomes a mutable pointer to the caller's `PCWSTR` value, not a const pointer to a const character
pointer. Pointer-valued contracts cannot serve as integer buffer counts. Counted-buffer annotations
on bound pointer values remain unsupported.

Functions and methods share annotation lowering. `_In_`, `_Out_`, and `_Inout_` retain direction.
`_In_opt_` adds input optionality for pointer parameters. `_Out_opt_` and `_Inout_opt_` require
writable pointers and preserve output/inout direction with `#[opt]`. `_COM_Outptr_` requires a writable
`void**` or interface output pointer. A `void**` receives `#[out] #[iid_is]` (the metadata
`ComOutPtrAttribute`); typed interface outputs retain their interface type and receive `#[out]`.

`_Null_terminated_` and `_NullNull_terminated_` retain typed character-pointer contracts as
`#[termination(Count = 1|2, Phase = 0|1|2)]`. Phase is unspecified, precondition, or postcondition;
a bare marker does not acquire a guessed direction. Distinct contracts survive together: double-NUL
termination also satisfies ordinary NUL termination. The experimental
`NativeStringTerminationAttribute` definition lives in `crates/libs/clang2/metadata.rdl`, not the
production metadata seed.
Double-NUL parameters retain raw pointers rather than ordinary string bindings. The source/RDL
fixtures check exact metadata roundtrips, optionality, and counted buffers on all three targets;
native input/output calls run on x86/x64. Four real SDK environment/volume APIs preserve their
terminator phases across reversed TUs. Dereferenced/scoped terminators and broader SAL combinators
remain outside typed lowering; MIDL evidence stays separate.

An interface object pointer and a pointer to an interface slot are different contracts. After
consuming one native interface pointer level, `_Out_`/`_Inout_` describe mutation of the borrowed
object, not replacement of the caller's pointer. These project as input interface values;
optional forms retain optionality. An outer pointer retains its output/inout slot direction.

Buffer annotations preserve their direction and length units:

| Annotation family | Length metadata |
| --- | --- |
| `_In_reads_`, `_Out_writes_`, `_Inout_updates_` | `#[len_param(index)]` or `#[len_const(count)]` |
| `_In_reads_bytes_`, `_Out_writes_bytes_`, `_Inout_updates_bytes_` | `#[size_param(index)]` |

Each of these six buffer annotations also supports its `_opt_` suffix, such as
`_In_reads_opt_` or `_Out_writes_bytes_opt_`. Optionality applies only to the buffer parameter;
its count parameter is not made optional. Length and direction rules are unchanged.

`_Out_writes_bytes_all_` and `_Out_writes_bytes_to_`, including their optional forms, also retain
a separate successful-return valid-byte extent:

| Annotation | Capacity | Valid bytes when the call succeeds and the buffer is non-null |
| --- | --- | --- |
| `_Out_writes_bytes_all_(capacity)` | By-value integer parameter | The same parameter value |
| `_Out_writes_bytes_to_(capacity, count)` | By-value integer parameter | A by-value integer parameter |
| `_Out_writes_bytes_to_(capacity, *count)` | By-value integer parameter | Dereferenced nonoptional writable integer output parameter |

RDL spells this as `#[written_bytes(BytesParamIndex = N, Dereference = true|false)]`, independently
of `#[size_param(M)]`. The experimental `MemoryWrittenAttribute` definition lives in
`crates/libs/clang2/metadata.rdl`; supply that file as an input or compile it as a reference when
compiling these plans. It is not added to the production metadata seed or bundled default WinMD.
`MethodParam::bytes_written()` reads it separately from capacity. RDL/WinMD round-trips preserve
both relationships. Constant/arithmetic written counts, optional count pointers, and noninteger
counts reject. Declaration-local names resolve to original parameter positions before lowering.

Generated output wrappers still expose unsafe pointers and raw NTSTATUS results. They do not
assume output initialization on failure or turn capacity into a returned slice length. The caller
must check the API's success condition, inspect returned counts, and respect retained-buffer
lifetimes. Postcondition metadata is not a general ownership or success-predicate model.

Resolution binds length parameter names using each annotation's original declaration context.
Projection uses those zero-based positions, not the representative declaration's parameter names;
method positions exclude `self`. A referenced count must be a by-value integer parameter, and its
index must fit `i16`. Constant element counts are decimal literals from zero through `i32::MAX`.
Output buffers must be writable. Element-counted `void*` buffers, invalid understood count types,
and multiple length annotations are errors. Expressions without typed lowering, including arithmetic
capacities and written counts, remain in the source payload without a guessed length attribute.
Unsupported COM output variants likewise retain their source contracts.
Compiling length attributes to WinMD requires the standard metadata attribute definitions, supplied
by `windows_rdl::Reader::reference_default()` in the fixtures.

Scalar string annotations `_In_z_`, `_In_opt_z_`, `_Out_z_`, and `_Inout_z_` preserve raw character
pointers when no `ProjectionOptions::string_references` binding is supplied. Each supplied binding
is a trusted pointer-sized metadata value type. The planner never assumes a Windows metadata
namespace. Interface bindings and malformed names are errors.

| Native single-pointer target | Mutable binding | Const binding |
| --- | --- | --- |
| Signed/unsigned 8-bit character | `StringKind::Ansi` | `StringKind::AnsiConst` |
| Unsigned 16-bit character, including Windows `wchar_t` | `StringKind::Wide` | `StringKind::WideConst` |

For example, bind `StringKind::WideConst` to a `TypeReference` naming the caller's `PCWSTR` with
`ReferenceKind::Value`. Its pointer layout is part of that trusted contract, not inferred from an
external record definition. Native constness and SAL direction remain independent: `_In_z_ char*`
uses the mutable binding with input direction. Output/inout strings require writable pointers.
Other character widths and pointer depths fail projection. Raw unannotated pointers are unchanged;
typedef spelling alone does not establish termination. An explicitly bound pointer typedef retains
its caller-supplied identity. When it also has a string annotation, both bindings must name the same
metadata type. Annotated local aliases retain their own identity. Counted `_z_` forms retain their
source payload without typed string lowering.

Local COM methods must be non-static, non-const, and non-ref-qualified. Their calling convention
must match the RDL system ABI: stdcall for x86, the platform convention for x64 and ARM64.
Inheritance is represented by a base-interface reference, with declared method order preserved.
Same-name methods within an interface are rejected: source order is not sufficient to establish
MSVC's overloaded virtual-method slot order. Rust name disambiguation does not repair that ABI.
Methods that override inherited slots are also rejected, including implicit and indirect-base
overrides. Appending their declarations would create slots that do not exist in the native vtable.
An interface parameter consumes exactly one native pointer/reference level. By-value record results
are rejected for COM methods on all targets until the downstream aggregate-return ABI is covered.
Record pointers and free-function results with ordinary record layouts remain supported.

Packing that changes storage layout, bitfield projection, and anonymous callables outside record
fields remain outside this slice. Named types publish checked scope-prefixed identifiers, retaining
native nominal identity and rejecting output collisions. UUIDs decode from `__declspec(uuid(...))`
only (no `GUID`-typed value decoding yet). General SAL lowering, MIDL relationships, WinRT mapping,
header ownership, and automatic DLL routing are not implemented.

## Evaluation

```powershell
cargo test -p windows-clang2 -p test_clang2
cargo clippy -p windows-clang2 -p test_clang2 --all-targets
cargo fmt -p windows-clang2 -p test_clang2
cargo test -p test_clang2 --test abi --test com --test wdk --test crypto --target i686-pc-windows-msvc
```

Inspect a real file or fixture:

```powershell
cargo run -p windows-clang2 --example inspect -- Native `
    crates\tests\libs\clang2\input\native.h -- -x c++ --target=x86_64-pc-windows-msvc
```

The example provisions the shared pinned libclang and prints the native graph followed by the
agreement report. The library itself does not install or configure dependencies.

Dedicated headers live in `crates/tests/libs/clang2/input`. The native evidence golden specifies
its target environment version so host Visual Studio versions do not change its target line.
Native dump golden inputs are normalized to LF before capture, including anonymous-record USRs
that contain byte positions. Production capture keeps the actual source byte offsets; a separate
LF/CRLF test checks that provenance rather than removing it from the dump. Native record kinds are
typed once at capture and shared by validation and projection.
Update it only when the model change is intentional:

```powershell
$env:UPDATE_EXPECT = "1"
cargo test -p test_clang2 native_evidence_golden
Remove-Item Env:UPDATE_EXPECT
```

Coverage includes native distinctions erased by the legacy projection model, all selected root
categories, alias chains, non-root completions, contradictory definitions, incomplete enums,
constant observations, qualifiers, calling conventions, and unavailable evidence. Shared graphs
at 32, 64, and 128 levels assert exact declaration-pair expansion counts. Target capture runs
for x64, x86, and arm64. Legacy extraction is also exercised in the same test process.

Projection fixtures compare RDL and read generated WinMD to assert external value/interface kinds,
pointer depth, constants, and parameter directions. Coverage includes the external-reference
conflict beneath non-function roots, indirect interface constants, checked completions, redeclaration
permutations, an externally bound `IUnknown`, and a synthetic local interface pair. The
`sdk_interfaces.h` fixture selects `Use` and emits the pinned SDK's `IUnknown` and `IClassFactory`
locally on x86, x64, and ARM64. It checks IIDs, base interfaces, method order, signatures, pointer
depth, optionality, and COM output attributes. Only the `_GUID` value record is externally bound;
neither interface is substituted. Negative controls cover UUID conflicts, incompatible method
calling conventions, and invalid COM output types.

`anonymous.h` covers nested and sibling anonymous aggregates, named members of unnamed records, and
mutations with unchanged size/alignment. The cross-TU matrix changes both input and filename order
on all three targets. An external record binding cannot hide a contradictory anonymous member.
Local direct-field anonymous aggregates emit checked nested RDL records. The conflict checks still
run before storage projection or external substitution.

`enums.h` covers scoped/unscoped enums, same-name typedef roots, alias uses, duplicate values,
negative narrow values, and the maximum unsigned 64-bit value. RDL and semantic metadata checks
run on x86, x64, and ARM64. Booleans and unsupported representations fail explicitly. Signed
constants are sign-extended from the captured representation width before rendering.
`flag_enums.h` distinguishes an explicit Clang flag marker from an equivalent plain enum.
The marker becomes `#[flags]` without changing the native representation. Marker presence must
agree across observations, including forward declarations; mixed annotated/unannotated
declarations are conservatively rejected. `operator_flags.h` covers the Windows
`DEFINE_ENUM_FLAG_OPERATORS` marker. Capture associates its expansion location with the emitted
`operator|` and the compiler's canonical enum type, not a textual argument name. Ordinary operators,
same-named types in another namespace, and an empty macro redefinition do not imply flags. Native
signedness stays unchanged, and conflicting observations reject.

`midl_properties.h` covers declaration-prefix `propget`/`propput` comments. Only marked methods get
`SpecialName`; a matching method-name prefix or a parameter comment is not evidence. Conflicting
observations and marker/name mismatches reject. `propputref` is captured but projection rejects it.

`sal_buffers.h` covers element and byte relationships for functions and methods, forward parameter
references, direction, and constant count bounds. The redeclaration fixture verifies that renaming
a count does not lose its original binding. `sdk_buffers.h` projects `BCryptHashData` and
`BCryptGenRandom` with byte counts referencing their third parameter. `BCryptDeriveKeyPBKDF2`
adds two optional input buffers with independent byte counts and a required output buffer.
The synthetic buffer matrix covers required and optional forms for functions and COM methods.
Conflicting optionality across redeclarations remains an error even when another declaration has
no annotations.

`strings.h` covers all four string binding kinds, typedef chains, functions, COM methods,
direction, optionality, and raw pointer controls. `sdk_strings.h` projects the SDK's
`WinHttpTimeToSystemTime` with an explicit const-wide binding and a local `SYSTEMTIME` record.

The SDK and WDK fixtures force-include the clang2 SAL capture header. It installs SDK wrappers before
replacing source markers, with strict erasure disabled. MIDL prefix comments are captured separately
and do not override explicit SAL direction.

The workspace test job includes both crates on supported hosts. Like the existing clang tests,
frontend tests are excluded from the x86 test process because the pinned runtime has no x86 DLL.
An x86 ABI-only CI step runs capture and generation in the x64 build script, then executes the x86
fixtures without loading libclang in that process. Native x64 and ARM64 jobs run the ABI fixtures as
part of the ordinary package tests.

### Executable ABI fixture

The MSVC build script captures `input/abi.h` for Cargo's target, projects RDL, compiles WinMD, and
generates Rust with `windows-bindgen --flat --sys --extern`. It independently compiles `input/abi.cpp`
with the target C++ compiler. All generated artifacts stay in `OUT_DIR`.

`tests/abi.rs` compares record size, alignment, and every field offset against C++. It exercises
by-value aggregate arguments and returns through a C-linkage function, inherited vtable calls from
Rust into C++, and C++ virtual calls into a stateful Rust object using the generated vtable type.
Integer/floating-point results, `this`, buffer mutations, and call counts are checked. Repeated calls
exercise the x86 stack-cleanup convention. `--extern` links the static fixture, so this does not test
DLL export lookup or decorated import-name policy.

The fixture also includes `enums.h`: record layout containing four enum representations, signed
and unsigned enum values through a free function, and enum arguments/results through virtual calls
in both directions execute on x64 and x86, in debug and release.

An aggregate-returning virtual-method experiment exposed a downstream limit:
`windows-bindgen`'s `CppMethod::write_abi` emits an explicit result pointer with a void return, and
`CppMethod::write_upcall` only writes that result. MSVC x86 callers can read the returned aggregate
through the result-buffer address in EAX; the generated signature does not return that address.
The reverse call failed even though Rust-to-C++ dispatch passed. `com_record_result.h` now checks
explicit rejection for small/large records and aliases on all three targets, with record-pointer
returns as the positive control. Fixing the downstream ABI requires separate bindgen coverage and
regeneration; the prototype does not work around it with a handwritten vtable signature.

### Real WDK storage and BCrypt calls

`input/wdk_layout.h` includes the pinned WDK's `ntifs.h` and `wdm.h`. Two `decltype` typedefs
select `IO_STACK_LOCATION::Parameters.DeviceIoControl` and `QuerySecurity` from those headers.
The types are not copied synthetic definitions. Capture/projection covers x86, x64, and ARM64
with repeated TUs and reversed input order.

`input/wdk_layout.cpp` independently compiles the real types with MSVC. `tests/wdk.rs` compares
every field offset, size, and alignment against that code, then passes generated records through
native pointer calls that read and mutate every field. Native execution covers x64 and x86,
debug and release. ARM64 has capture/metadata coverage here, not locally observed execution.

| Record | x86 size / alignment / offsets | x64 size / alignment / offsets |
| --- | --- | --- |
| `DeviceIoControl` | 16 / 4 / 0, 4, 8, 12 | 32 / 8 / 0, 8, 16, 24 |
| `QuerySecurity` | 8 / 4 / 0, 4 | 16 / 8 / 0, 8 |

The compiler reports larger member gaps on x64. Projection preserves these through padding unions
and record alignment without WDK type-name rules. `adjusted_layouts.h` also covers padding-name
collisions, nested adjusted records, increased record alignment, and by-value rejection.

`tests/crypto.rs` calls the actual Windows `bcrypt.dll` through generated wrappers. Its eight
required exports are `BCryptOpenAlgorithmProvider`, `BCryptGetProperty`, `BCryptCreateHash`,
`BCryptHashData`, `BCryptFinishHash`, `BCryptDestroyHash`, `BCryptDeriveKeyPBKDF2`, and
`BCryptCloseAlgorithmProvider`. The inputs come from the pinned SDK and clang2 SAL adapter; external
`PCWSTR` and handle bindings are explicit. The consumer binds native `LPCWSTR` to
`Windows.Win32.PCWSTR`; generic projection retains the local annotated typedef unless the caller
supplies that contract. Metadata assertions check both provider string inputs and the property
name on all three targets. This is a typedef-level consumer policy, not a per-export string repair.

The hashing lifecycle opens a SHA256 or HMAC provider, queries object and digest lengths, creates
a hash, feeds incremental chunks, finishes it, destroys it, and closes the provider. Both
CNG-allocated and caller-owned object storage are covered. The fixture's hash object borrows the
provider and owns any retained buffer; destruction calls CNG before freeing that storage.
Null-buffer property queries return required lengths; oversized property buffers use only the
reported valid prefix, not their capacity. Known answers cover empty input, `abc`, embedded NULs,
and an HMAC key containing NUL. PBKDF2 retains its ordinary/null/empty/embedded-NUL cases.
Expected bytes were calculated independently with Python `hashlib` and `hmac`.

Guard bytes bound output and retained-object storage. Too-small property/object buffers,
unsupported properties, incorrect digest lengths, and zero PBKDF2 iterations exercise native
statuses without assuming failure outputs are initialized. Exact Rust function-pointer assignments
preserve counted input slices, raw output pointers/capacities, handles, and status results.

Each target uses the matching pinned `microsoft.windows.sdk.cpp.<arch>` package's `bcrypt.lib`.
COFF name-type decoding supplies both the DLL and its export spelling; this covers x86 stdcall
symbols without a hand-maintained mapping. SDK libraries can contain multiple linker aliases for
one export. Every required function is present in metadata, and removing its import mappings
must make projection fail. The eight exports execute on x64/x86 in debug and release; ARM64 has
capture, import-library, and metadata coverage only. This is not SDK-wide library-selection policy.

The contract evidence is the pinned header plus Microsoft's
[SAL parameter reference](https://learn.microsoft.com/en-us/cpp/code-quality/annotating-function-parameters-and-return-values),
[CreateHash](https://learn.microsoft.com/en-us/windows/win32/api/bcrypt/nf-bcrypt-bcryptcreatehash),
[GetProperty](https://learn.microsoft.com/en-us/windows/win32/api/bcrypt/nf-bcrypt-bcryptgetproperty),
and [FinishHash](https://learn.microsoft.com/en-us/windows/win32/api/bcrypt/nf-bcrypt-bcryptfinishhash)
documentation. COFF name types follow the
[PE specification](https://learn.microsoft.com/en-us/windows/win32/debug/pe-format#import-name-type)
and the pinned SDK's `IMPORT_OBJECT_NAME_TYPE`, including `IMPORT_OBJECT_NAME_EXPORTAS`.

The WDK changes fit the existing stages: capture owns unnamed native identity, resolution checks
unchanged evidence, the planner chooses output names and storage, and the renderer prints only
planned items. The by-value gate caches each local record result so shared value dependencies are
not expanded as a tree; it does not infer calling ABI from storage. Import-name contracts likewise
affect projection only. RDL and the metadata reader preserve the explicit valid-byte extent, with
the experimental attribute supplied alongside the plan. Bindgen, production extraction, the
production vocabulary seed, and production metadata remain unchanged.

The paired `wdk` example uses identical pinned headers, selected typedefs, target, and TU counts:

```powershell
cargo run -p test_clang2 --example wdk --release -- new 2
cargo run -p test_clang2 --example wdk --release -- old 2
```

It reports capture and planning/emission time and requires both output records to contain fields.
That presence check is not a layout or parity oracle. The legacy output loses the member gaps:
its x64 `DeviceIoControl` has natural offsets 0, 4, 8, 16 and size 24; `QuerySecurity` has offsets
0, 4 and size 8 despite its alignment attribute. These disagree with the independent compiler
measurements above. Comparing costs is useful for this input, but not an equal-correct-output
speedup claim. Full WDK extraction and resource budgets remain open.

One local x64 release process per cell, with cached dependencies, measured:

| TUs | Rewrite capture | Rewrite resolve / project | Rewrite peak MiB | Legacy capture / emit | Legacy peak MiB |
| --- | --- | --- | --- | --- | --- |
| 1 | 155 ms | 34 / 36 us | 141.5 | 1299 / 61 ms | 93.9 |
| 2 | 333 ms | 46 / 43 us | 169.2 | 2743 / 129 ms | 156.3 |
| 4 | 662 ms | 49 / 43 us | 223.5 | 4982 / 242 ms | 275.9 |

Phase timings exclude compilation and package setup. Peak working set was sampled every 10 ms
from dedicated processes and includes the final RDL/WinMD presence gate, not just capture.
The rewrite retains seven selected groups and 7/14/28 observations; legacy captures
18,890/37,780/75,560 header facts before selection. The rewrite is faster on these inputs, but
its whole-process memory is higher for one and two TUs. This is neither equal-work performance
nor evidence of uniformly lower memory; the two backends also differ in output correctness.

### COM ownership fixture

`input/com.h` includes the pinned SDK's `unknwnbase.h`. The build script captures and locally projects
`IUnknown` and `IClassFactory`, with explicit external `GUID` and scalar `HRESULT` contracts from
`input/com_reference.rdl`. Normal bindgen output supplies the high-level `IClassFactory` wrapper;
bindgen maps the standard `IUnknown` metadata identity to `windows_core::IUnknown`. A separate raw
binding bootstraps a native factory owned by the test. No high-level wrapper or COM vtable is
handwritten in Rust.

`input/com.cpp` implements the SDK interfaces with per-object counters held by the caller.
`tests/com.rs` exercises the generated `CreateInstance<T>` wrapper and core interface operations:

| Case | Assertion |
| --- | --- |
| Clone and drop | Exact AddRef/Release counts; final release destroys each object once. |
| Successful QueryInterface | Correct requested IID, one new reference, and controlling-unknown identity. |
| Unsupported QueryInterface | `E_NOINTERFACE`, no new reference, and no premature release. |
| Successful CreateInstance | One owned output reference, independent of the factory lifetime. |
| Unsupported output IID | Error with null output, no child allocation or owned wrapper. |
| Non-null aggregation argument | `CLASS_E_NOAGGREGATION`; passing the outer object borrows its reference. |

These cases execute on x64 and x86 in debug and release. The fixture uses SDK declarations with an
in-process test implementation, not Windows COM activation, apartments, marshaling, or a registered
server. `scalar_references.h` also covers scalar contracts through alias chains, fields, pointers,
function/method signatures, invalid binding shapes, and order-independent redeclaration conflicts.

The same fixture locally projects `IProperties`, whose C++ implementation stores an opaque `HWND`
and borrows a static UTF-16 string. Generated setters/getters round-trip null, ordinary, and all-one
pointer bits; generated string methods compare equal and unequal inputs. The handle binding uses
the default WinMD's actual `Windows.Win32.HWND` contract. `pointer_references.h` separately covers
alias boundaries, output indirection, constant pointers, fields, and incompatible bindings.
`midl_directions.h` covers function/method directions, SAL precedence, and misleading comments;
cross-observation mutations verify independent source-family conflicts and order invariance.

`IProperties::Inspect` mutates an existing borrowed interface object. The generated implementation
trait must accept `Ref<IUnknown>`, not `OutRef<IUnknown>`. Calling the native implementation checks
the object pointer, QueryInterface result, and reference counts on x64/x86 in debug and release.

### Multi-TU review and scale probe

The focused review covers observation selection, typedef projection agreement, forward UUIDs,
same-name tag roots, and the native-comparison and projection caches. It is not a full production
audit. Counterexamples confirmed two defects: forward UUID evidence was discarded, and selecting
an SDK interface by name also selected its same-name typedef as a separate output item. Capture now
retains forward UUIDs, resolution checks all supplied UUIDs, and tag/typedef roots share one output.

The suspected field-selection gap did not require another projection pass: native validation
rejects different written field-type identities, while alias agreement checks the underlying
canonical evidence. Callable projection separately checks written semantic typedef contracts
because native callable comparison uses canonical types. The regression matrix changes both
input order and filename order; reversing inputs alone cannot challenge a filename-sorted capture.

`tests/webview.rs` parses the pinned `WebView2.h` and `WebView2Interop.h` as separate TUs. The
supported roots are `ICoreWebView2Deferral`, `ICoreWebView2StringCollection`, and
`ICoreWebView2HttpHeadersCollectionIterator`. Golden RDL and semantic metadata checks cover both
filename orders. The interop graph reaches `tagVARIANT`; anonymous members now participate in native
agreement checking. Explicit external `IStream` and `VARIANT` bindings use the repository's default
Win32 metadata, only after the complete selected native graph resolves.

The `test_clang2` example `webview` runs either frontend on these same pinned inputs and prints
phase timings, work counts, interface GUIDs, bases, signatures, and parameter directions. It writes
RDL, WinMD, and generated Rust under `OUT_DIR`. Root selection accepts `1`, `3`, `interop`, or
`consumer`. The last reads all WebView2 roots from the production `webview.txt` binding filter;
method filters select the whole containing interface, not a shortened native vtable.
TU pairs repeat the main and interop inputs:

```powershell
cargo run -p test_clang2 --example webview --release -- new 1 3
cargo run -p test_clang2 --example webview --release -- old 1 3
cargo run -p test_clang2 --example webview --release -- new 4 3
cargo run -p test_clang2 --example webview --release -- new 1 interop
cargo run -p test_clang2 --example webview --release -- new 1 consumer
cargo run -p test_clang2 --example webview --release -- old 1 consumer
```

There is no fallback to legacy extraction. Both backends use the same explicit external type
bindings. The three-root metadata has matching IIDs, bases, method order, HRESULTs, scalar outputs,
and caller-bound string outputs.

One local Windows x64 release run per cell, with cached packages, measured the following for three
selected roots, before pointer-typedef and MIDL-direction support. Peak working set was sampled
from each dedicated process, including metadata and binding generation; it is not a phase-specific
allocation measurement. Timings exclude compilation
of the example, and capture timings exclude package setup:

| TUs | Rewrite capture | Rewrite resolve / project | Rewrite peak MiB | Legacy capture / emit | Legacy peak MiB |
| --- | --- | --- | --- | --- | --- |
| 2 | 647 ms | 78 / 46 us | 185 | 7568 / 274 ms | 379 |
| 4 | 1236 ms | 107 / 57 us | 333 | 15413 / 575 ms | 643 |
| 8 | 2988 ms | 165 / 81 us | 635 | 29819 / 1166 ms | 1189 |

The one-root runs had similar capture cost and memory. Rewrite groups stayed at 17 for the
three-root case; observations grew 48 -> 96 -> 192 and declaration comparisons 31 -> 79 -> 175.
Both backends emitted identical RDL within their own root selection as TU counts increased.
Legacy captures the full header facts before filtering; the rewrite captures only selected native
dependencies after indexing the headers. These numbers demonstrate that selection advantage, not
equal-work production performance or a resource-budget guarantee.

The pressure point in this small-root run is libclang TU/index memory, not projection time. An
eight-TU case uses about 635 MiB even for three small roots. The full-consumer probe below measures
more roots; macro-heavy reparsing, deep alias chains, and parallel target extraction remain
unmeasured.

### Full-consumer viability probe

All 79 WebView2 roots in the production binding filter, plus the native `POINT`/`RECT` typedef
roots, capture, resolve, and project on x86, x64, and ARM64, in both filename orders. They form 530
native groups and 1,431 observations across two
TUs, with no incomplete declarations. The build script compiles the projected RDL and generates
normal Rust wrappers using concrete root filters so external signature dependencies are included.
A broad namespace filter can replace methods with private vtable slots when those dependencies
are absent; compiling such a file does not prove the methods exist. `tests/webview.rs` includes the
wrappers and assigns selected methods to exact Rust function-pointer signatures. Generated
wrappers compile on x64 and x86. Run capture tests in the x64 process: the pinned
libclang runtime is a 64-bit DLL and cannot load in an x86 test executable.

The rewrite emits 209 types and 672 methods; the comparison backend emits 210 types and the same
672 methods. Legacy retains a local `VARIANT` alias; the rewrite references the bound external type.
Both retain the selected `POINT`/`RECT` names. Matching method counts and compiling wrappers are not
semantic parity. The separate loader gate adds all five exports selected by `tool-webview`; this
is not the complete Win32/WDK production gate.

The scaling baseline below predates pointer-typedef bindings, MIDL-direction capture, and concrete
binding filters. One local Windows x64 release run per cell used cached packages and excluded
compilation. Peak working set covers the entire dedicated process, including metadata and Rust
generation, sampled every 25 ms:

| TUs | Rewrite capture | Rewrite resolve / project | Rewrite peak MiB | Legacy capture / emit | Legacy peak MiB |
| --- | --- | --- | --- | --- | --- |
| 2 | 1312 ms | 2264 / 2426 us | 326.6 | 9112 / 394 ms | 379.0 |
| 4 | 2530 ms | 3573 / 3748 us | 456.8 | 16670 / 655 ms | 643.4 |
| 8 | 5253 ms | 9331 / 8762 us | 717.3 | 32976 / 1223 ms | 1188.9 |

RDL hashes remain unchanged within each backend at 2, 4, and 8 TUs. Rewrite declaration-pair
comparisons are 909, 2357, and 5253; type-pair comparisons are 4176, 12530, and 29238. This supports
continued work on the selected graph, but does not establish a production resource budget.

A two-TU, 79-root release run after the handle/string contract gate measured 1027 ms capture,
2349 us resolve, 2459 us project, and 322.7 MiB peak for the rewrite. Legacy measured 6927 ms capture,
333 ms emit, and 378.9 MiB peak. It used the same cached-input, compilation-excluded method and
25 ms sampling. This predates the issue-driven enum/override changes; it is one run, not a
distribution or a refreshed 4/8-TU scaling result.

`webview_compare` compares type kinds, attributes, enum/field values, IIDs, base interfaces,
method flags/order, signatures, import names/libraries, directions, optionality, buffer
relationships, and COM output markers. Pass the two `test.winmd` paths printed by the examples:

```powershell
cargo run -p test_clang2 --example webview_compare -- <old-test.winmd> <new-test.winmd>
```

It reports differences and exits unsuccessfully; it does not normalize known gaps into a green
parity result. With identical explicit bindings, the x64 consumer probe has matching IIDs, bases,
method flags/order, parameter directions, optionality, buffer relationships, and COM output markers.
There are two signature differences, both local-versus-external `VARIANT` aliases. The full
difference counts are 6 removed facts, 4 representations, 47 fields, 2 signatures, and 2 calling
conventions. These are
metadata facts, not counts of distinct defects. Classification explains the differences, not
approves them.

| Difference | Assessment / remaining contract |
| --- | --- |
| Four flags enums retain signed native representation and values | Both outputs carry `FlagsAttribute`. The rewrite preserves the compiler's `i32`, including native `ALL = -1`, instead of the legacy `u32` conversion. |
| Legacy retains a local `VARIANT` alias | The rewrite uses the explicit external `VARIANT` contract directly; no native pointer level is removed. |
| Two loader exports use `C` rather than `system` on x64 | Same platform ABI on this target. Exact imports for all five exports are checked on three targets; real DLL calls execute on x64/x86. |

`consumer_input_contracts_cutover_gate` is an ordinary passing test covering six `HWND` inputs and
two host-object `VARIANT*` inputs. Exact wrapper assertions also require a handle-valued setter,
a handle-valued getter, a `PWSTR` getter, and a host-object input pointer:

```powershell
cargo test -p test_clang2 --test webview consumer_
```

### Actual consumer and loader gate

```powershell
cargo run -p test_clang2 --example webview_consumer -- x86_64-pc-windows-msvc
cargo run -p test_clang2 --example webview_consumer -- i686-pc-windows-msvc
cargo run -p test_clang2 --example webview_consumer -- aarch64-pc-windows-msvc
```

The example copies the actual `windows-webview` sources and manifest into a new
`target/clang2-webview/<target>/run-<pid>` directory. It uses the production binding filter,
including `--flat --minimal --dead-code` and every implementation/method selector, changing only
artifact paths. External bindings use the actual default metadata identities, including
`Windows.Foundation.HResult` and `System.Guid`, not the synthetic COM reference fixture.
The scratch example does not modify the production files. The actual `tool-webview` separately
uses clang2 to regenerate the committed bindings with the same filter.

All four feature combinations (default, `system`, `reactor`, both) compile on x64/x86 and ARM64.
The only approved consumer adjustment is its all-bits request-source initializer: `!0` preserves
the same bits under the existing unsigned binding and the native signed binding. No scratch-source
rewrites hide incompatible generated APIs. This gate exposed missing MIDL property markers that
ordinary wrapper compilation and the earlier differential did not detect; capture and metadata now
preserve those markers.

The example also generates an independent loader executable. Its child-only DLL search path points
to the pinned architecture package, and the executable verifies the loaded module's absolute path.
It calls the generated `CompareBrowserVersions` declaration with equal/older/newer versions,
channel suffixes, guarded output storage, malformed strings, and null parameters. Debug and release
pass on x64/x86 without browser installation or activation. The pinned DLL returns `E_POINTER` for
a null result pointer, despite the published reference saying `E_INVALIDARG` for any null parameter;
the fixture records that observed distinction. Other tested invalid inputs return `E_INVALIDARG`.
No failure-output initialization guarantee is inferred.

All five loader import routes and missing-route rejection are covered on x86/x64/ARM64. Local
ARM64 consumer checks pass, but native linking remains blocked by missing Visual Studio ARM64
runtime libraries. CI runs the gate on each native target; its ARM64 result has not been observed.

### Live WebView cutover

`tool-webview` uses capture -> checked resolution -> projection without a legacy fallback.
It selects the consumer's dependency closure and all five loader exports, rather than claiming
complete SDK coverage. Capture uses x86 to retain source `__stdcall` distinctions for the portable
bindings; native x64 testing remains the working viability gate. See
[binding generation](windows-webview.md#binding-generation) for the configuration.

The generated bindings preserve native signed flags and the `EventRegistrationToken` struct.
No further handwritten wrapper changes are needed beyond the representation-neutral `!0`
initializer.
All 33 existing `test-webview` fixtures passed on x64/x86 in debug/release, covering navigation,
scripts, IPC, DevTools, resource interception, cookies, controller settings, profiles, and host
creation/closure. The self-contained x64 Reactor fixture also passed real WinUI-to-COM
initialization.
The ongoing CI gate is x64 debug with `--headless --require-runtime`; bootstrap failures cannot
be counted as successful skips. These results establish a working consumer, not every ABI or
lifetime property. The harness exits the process after its fixtures; general shutdown guarantees
and untested process-failure/download paths remain outside this gate.

Three local optimized runs with cached packages measured the same 81-root comparison workload:

| Measurement | clang2 | Legacy |
| --- | --- | --- |
| Median process wall time | 1.87 s | 8.39 s |
| Sampled peak working set | 322.8-323.3 MiB | 378.8-379.2 MiB |
| Emitted methods | 672 | 672 |

The legacy extractor captures full header facts before filtering, while clang2 captures selected
dependencies after indexing. This is a practical workload comparison, not equal-work parser
performance or a production resource budget. The actual switched `tool-webview` took 1.71-1.82 s
including process startup and regenerated byte-identical bindings in all three runs.

The integration needs explicit external type identities and compiler-symbol import routes, but no
per-method WebView compatibility rewriting. Functions, callbacks, and methods select the
lexicographically smallest usable observed parameter name at each original position. Empty,
unusable, or duplicate names get collision-safe `p{index}` fallbacks; Rust keywords are escaped.
Filename and input order cannot choose presentation names. Names do not participate in native
agreement, and source annotation contexts keep their original positional bindings.
`parameter_names.h` checks permutations, escaping, fallback collisions, and buffer metadata.
Broader annotations, native ARM64 execution, and SDK-wide production coverage remain open.

### Live Animation cutover

`tool-bindings` generates `windows-animation` through clang2 using the pinned Win32 SDK. Its
existing filter selects eight native roots; no local animation interface, enum, keyframe, or
activation identity is supplied by bundled animation metadata. UI Animation and DirectComposition
have separate translation units. Core COM types and `IDCompositionAnimation` are explicit external
contracts, as appropriate for the crate's COM and graphics bridge.

The DirectComposition interface has a by-value `LARGE_INTEGER` method that generic projection still
rejects. This profile does not remove that ABI gate or pretend to support that call. Its exposed
operation passes an interface pointer to native Animation; a C++ receiver checks that handoff.
Opaque animation activation classes use the typed `class_guids` publication policy. Their constants
come from checked source UUID attributes; default class projection remains unchanged.

| Gate | Result and scope |
| --- | --- |
| Source closure | All eight filter roots project on x86/x64/ARM64, with reversed input order producing the same RDL. |
| Safe consumer | The actual `Manager`, `Variable`, `Storyboard`, `Keyframe`, and transition wrapper sources compile unchanged. |
| Native identity and slots | MSVC checks eight identities, all 13 selected method offsets, and keyframe size/alignment on x86/x64. |
| Live COM behavior | x86/x64 execute exact-value linear and instantaneous transitions, accelerated storyboard sequencing, and native curve handoff. |
| Actual generator | `tool-bindings` writes the committed animation bindings from `target/animation/Animation.winmd`; other generated bindings remain unchanged. |
| Remaining scope | ARM64 execution, generic by-value union ABI, canonical Win32/WDK publication, and broad projection parity are not certified. |

The `preserve_typedefs` policy retains written non-interface typedefs and prefers a unique typedef
name for an unselected record or enum tag. It has no MIDL-prefix test or Animation symbol rules;
multiple aliases do not acquire an arbitrary winner, and explicitly selected tags stay published.
Animation enables it for source names such as `UI_ANIMATION_SECONDS`, the result enums, and
`UI_ANIMATION_KEYFRAME`. The profile also binds the checked native `UINT_PTR` contract to `usize`.
The interface parity gate compares every source slot, signature, GUID, and selected public value
type against committed metadata on all three targets. Native compiler tests remain the ABI oracle.

The cutover gate requires unchanged consumer APIs and no unexplained generated differences.
The bounded differences are:

| Consumer | Difference and boundary |
| --- | --- |
| Animation | Source keyframe record-pointer storage replaces the metadata seed's `void*` target; pointer ABI is native-checked. Written scalar aliases are retained. Source UUID activation constants replace opaque class declarations. |
| WebView2 | External `VARIANT` replaces a local alias. Explicit handle/string typedef and loader import contracts retain native ABI and safe-wrapper signatures; source annotations and properties remain covered by the consumer gates. |

These are not blanket waivers for layout, slots, signatures, annotations, or missing roots. Further
consumer cutovers wait for their own classified differential and native contract gates.
The crate's safe application API does not change. The headless
storyboard sample and the animation crate's existing integration coverage run against the actual
switched bindings.

```text
cargo run -p tool-bindings --quiet
cargo test -p test_clang2 --test animation
cargo test -p test_clang2 --target i686-pc-windows-msvc --test animation native
cargo test -p test_animation
cargo run -p animation-storyboard
```

This is a second bounded production consumer, not a replacement for the full-header inventory.
Continue with shared callable/record representation, source data definitions, and explicit
profile/name reconciliation instead of adding another downstream validation campaign.

### Header-driven Win32 main/satellite slice

```powershell
cargo run -p tool-win32 -- --clang2-audio --rdl-only
```

This conversion-only command writes `target/win32-clang2/audio-rdl`. It compiles the combined RDL
to a temporary WinMD before marking supported roots emitted, then removes that binary. It retains
no metadata, Rust bindings, or consumer. Omitting `--rdl-only` retains the separate downstream
experiment.

This opt-in path reuses `tool-win32`'s pinned SDK, compiler arguments, prelude, and main/satellite
input assembly for `mmdeviceapi.h` and `endpointvolume.h`. It discovers the configured headers'
top-level and nested type declarations and macros instead of maintaining a symbol list. Record
members and enum values travel with their owning types. This describes the active x64 C++ header
configuration, not every conditional branch or architecture.

The only discovery exclusions are function-like macros, empty macros, reserved preprocessing
configuration, and inline helpers without exported entry points. Declaration-only data is not
silently excluded: data without an initialized observation remains visible as a rejection.
The complete selected graph resolves before any root is assessed for projection. Capture and
resolution failures mark selected declarations blocked; they are not treated as benign omissions.

An additional input includes `initguid.h` before `mmdeviceapi.h`, after the ordinary prelude.
This uses the SDK's own definition mode to expose initializers for all 18 property keys and four
device-interface GUIDs. The original main/satellite declaration observations remain in capture.
No macro-name parser, argument-text evaluator, or `IID_` naming guess supplies values. IID, CLSID,
and LIBID extern declarations without initializers remain unsupported.

The pinned headers produce 166 inventory rows and 102 selected names: 66 emit and 36 reject.
Forward declarations and typedefs account for repeated names. One x64 debug RDL-only run resolved
321 groups and 921 observations with 631 declaration comparisons in 2.93 seconds.

Output-pointer and nullable-result annotations are retained as raw contracts. Their presence does
not imply typed nullability or ownership lowering.

| Remaining family | Names | Missing contract |
| --- | --- | --- |
| GUID data declarations | 16 | Initializer or imported-data evidence for IID, CLSID, and LIBID variables. |
| RPC globals | 20 | Imported data is not a metadata constant. |

`MMDeviceEnumerator` emits its source UUID as an opaque native class. The pinned `mmdevapi.lib`
ordinal-17 contract is representable, and `ActivateAudioInterfaceAsync` emits with its dependencies'
nullable-output contracts preserved.

The generator exits 1 while any selected name is rejected, after generating the supported
candidate. A working consumer does not turn incomplete header
coverage into success. No DLL export-name guess replaces the ordinal import.

RDL-only output stays under `target/win32-clang2/audio-rdl`. The separate downstream mode writes
`target/win32-clang2/audio`:

| Artifact | Purpose |
| --- | --- |
| `inventory.tsv` | Every discovered row, its status, and a stage/reason for exclusions or failures. |
| `headers.tsv` | Output filenames and their source header paths; filename collisions fail. |
| `rdl/*.rdl` | Separate `mmdeviceapi`, `endpointvolume`, `guiddef`, `wtypes`, and `devicetopology` partitions. |
| `audio.winmd`, `src/bindings.rs` | Downstream mode only: metadata and bindings. |
| `Cargo.toml`, `src/main.rs` | Downstream mode only: standalone consumer experiment. |

Candidate definitions use `Win32Audio` so bundled Win32 metadata cannot substitute for them.
Standard external COM references remain explicit. The native `_GUID` record and its fixed byte
array stay source-owned; no external-layout call exception is needed for the local property-key
record. The default full scraper and committed Win32 metadata remain unchanged.

The read-only consumer activates the Windows device enumerator, enumerates active render
endpoints, reads device IDs and state, activates endpoint volume, and queries channel counts,
volume state, and the extended interface's channel range. It passed on three local endpoints in
x64 debug. It never changes volume, mute, or device settings, and reports an error rather than a
pass if no active endpoint is available. It does not prove notification callback execution.

The consumer also uses the generated `PKEY_AudioEndpoint_FormFactor` to query the real property
store, checks its variant type and enum range, and clears the variant before checking the result.
The key is converted field-by-field for the externally bound property-store interface; its
source-owned type is not rewritten.

Aggregate projection checks the GUID component widths, unsignedness, offsets, size, and alignment,
and the enclosing property-key layout. RDL uses `#[guid(...)] const ID: NativeGuid;` and adds
`= pid` for a key. Bindgen initializes the native record's original fields in both normal and sys
output. A C++ fixture compares every byte against MSVC-initialized source constants. Separate
fixtures check declaration/definition agreement, nested value conflicts, malformed shapes, and
metadata/RDL roundtrips.

The slice exercises fixed arrays, storage layouts for externally referenced records, and constants
whose native type is an enum or a caller-bound scalar typedef. Source/RDL fixtures cover exact
header identity despite matching basenames, macro expansion ownership, forward completion,
selected-alias ownership, combined-plan collisions, and native conflicts. No audio-specific
projection rules or by-value layout exceptions are added. The full-manifest inventory below
identifies the shared capture and projection blockers; richer annotations and production Win32
coverage remain open.

## Bounded tool-win32 header migration

```powershell
cargo run -p tool-win32 --quiet -- --clang2-headers shellscalingapi.h,tlhelp32.h --rdl-only
cargo run -p tool-win32 --quiet -- --clang2-headers pathcch.h --rdl-only
cargo run -p tool-win32 --quiet -- --clang2-headers all --rdl-only
```

This explicit x64 route shares discovery, complete-graph resolution, per-root assessment, combined
projection, and header ownership reporting with the audio inventory. It does not call clang1 as a
fallback, retain WinMD or Rust, or replace committed metadata. Outputs are under
`target/win32-clang2/<header-stems>-rdl`, including `inventory.tsv`, `headers.tsv`, and `rdl/`.
Stale RDL is cleared before capture, including on a failed rerun. Duplicate or unknown manifest
headers reject. Selected roots cannot be supplied by external bindings.

The header route reads the full ordered `IMPORT_LIBS` manifest from pinned SDK COFF libraries.
The first code import for an exact native linker symbol wins, matching the manifest's archive
precedence. `target/win32-clang2/imports.tsv` records each candidate's source library, native symbol,
DLL, export name or ordinal, and whether it was selected. Unmapped functions reject; no default DLL,
source-name fallback, or per-symbol library override is applied.
The generic route has no external value, enum, record, or interface bindings. Dependencies come
from the captured source closure, including nominal incomplete records. The audio consumer remains
a separate experiment with explicit external contracts.
Every supported combined closure must compile to WinMD before its roots are marked emitted.
Generic header compilation uses the hand-authored attribute vocabulary and clang2's experimental
metadata attributes, not bundled API references. A missing dependency cannot resolve against
committed metadata. Default references are available only to profiles with explicit external
bindings, such as audio. Metadata failures leave supported roots blocked with the compiler's
diagnostic; prior capture/projection rejections remain rejected. RDL-only removes the temporary
WinMD after a successful check.
`exclude_inline_functions` is enabled for this route. It excludes only function groups whose
captured observations are all inline, including header-only overloads of imported functions.
The declaration inventory still reports these inline rows as exclusions. Non-inline functions
without an import remain errors; an inline overload cannot hide an imported function's output.

| Input | Inventory rows | Selected names | Outcome |
| --- | --- | --- | --- |
| `shellscalingapi.h`, `tlhelp32.h` | 88 | 72 | All emit; 100 native groups, 101 observations. |
| `pathcch.h` | 37 | 26 | All emit; 38 native groups, 40 observations. |
| All three headers | 125 | 98 | All emit; 134 native groups, 137 observations. |
| Audio control | 166 | 102 | 66 emit, 36 declaration-only data rejects. |

The scaling API exposed a projection bug: externally bound enums lost their known scalar layout.
Enum bindings now retain compiler-reported width and signedness for fields, parameters, and returns;
they must bind metadata value types. `external_enum.h` covers the source-to-RDL boundary.

Path header coverage includes compiler-opaque scalar types with an exposed canonical builtin,
`char8_t`, and string literals. The string decoder reads compiler-rendered escapes, not raw source
tokens or libclang's NUL-terminated evaluated-string copy. Native evidence retains every code unit
and the literal encoding. Projection uses existing `String` constants with `encoding("ansi")` for
UTF-8 byte storage and `encoding("utf-16")` for wide storage. Unrepresentable bytes, unpaired
surrogates, UTF-32 output, and padded or truncated storage reject rather than changing the value.
`string_constants.h` covers x86, x64, and ARM64, including embedded NULs, Unicode, concatenation,
raw strings, escapes, and declaration annotations.

Counted `PWSTR`/`PCWSTR` bindings retain buffer length, direction, and native mutability checks.
`counted_strings.h` covers element and byte counts, including rejected const output buffers and
opaque handles. `inline_overload.h` covers explicit inline exclusion and missing-import failures.

One local debug run, excluding Rust builds but including process startup and dependency checks:

| Input | Wall time | Sampled peak working set |
| --- | --- | --- |
| Scaling and Toolhelp | 1.18 s | 180 MiB |
| Path APIs | 1.03 s | 179 MiB |
| All three headers | 1.00 s | 180 MiB |
| Audio control | 2.76 s | 362 MiB |

These are per-process OS peak-working-set counters sampled every 10 ms, not a large-input memory
budget. The successful group's source generation was byte-identical on repetition.

**The default production scraper remains on clang1.** Source completeness alone is not sufficient
to replace the committed partitions. Use `metadata/`'s RDL as the representation baseline, while
allowing corrections supported by native evidence. The source comparison shows:

- Toolhelp's named tags and typedefs are separately selected. `HEAPENTRY32` becomes an RDL typedef
  of `tagHEAPENTRY32`. Existing `NativeTypedefAttribute` metadata supports this record alias; its
  `Value` field is an encoding detail, not evidence of a public wrapper. Primitive/pointer typedefs
  have separate handle behavior. No RDL extension is needed for these record aliases.
- Explicit SDK contracts for `SIZE_T`, `DWORD_PTR`, and their signed/unsigned families preserve
  `usize`/`isize` uses without textual replacement or value-width guessing. Toolhelp's record fields,
  heap IDs, read sizes, and the scaling event cookie use the committed pointer-sized spellings.
- `SCALE_CHANGE_FLAGS` retains native signedness plus the explicit flags marker. Do not restore the
  legacy unsigned rewrite merely to eliminate a diff.

After resolving explicit aliases and namespace qualification, all seven record field lists
(including eight pointer-sized fields) and all 26 function type signatures agree with the committed
RDL. This comparison does not equate annotations or calling-convention spellings. Repeating the
header run produces byte-identical RDL and inventory reports.

Public record/tag naming and production partition ownership remain cutover decisions.
Path output matches all 22 committed function type signatures and three constants for x64, retaining
all 12 counted-buffer markers. `PATHCCH_OPTIONS` retains native signedness rather than the committed
unsigned rewrite. The combined run's three RDL partitions are byte-identical to their separate runs.

### Broader header inventory

Run independent headers with `--clang2-headers all --rdl-only` to expose shared failure classes
without one malformed macro preventing assessment of unrelated headers. `manifest.tsv` is updated
after every attempt. Headers failing discovery have unavailable counts, not zero selected names.
A nonzero exit means incomplete coverage. The recorded x64 control baseline uses
the pinned SDK and the explicit imports above. Counts are case-sensitive selected names per header,
not inventory rows, generated dependencies, or a deduplicated cross-header API count.

| Header | Selected | Emitted | Rejected | Blocked before projection |
| --- | ---: | ---: | ---: | ---: |
| `fileapi.h` | 134 | 119 | 15 | 0 |
| `processthreadsapi.h` | 158 | 151 | 7 | 0 |
| `sysinfoapi.h` | 62 | 55 | 7 | 0 |
| `memoryapi.h` | 95 | 77 | 18 | 0 |
| `psapi.h` | 72 | 60 | 12 | 0 |
| `winnls.h` | 732 | 0 | 0 | 732 |
| `winreg.h` | 177 | 163 | 14 | 0 |
| `winsvc.h` | 404 | 384 | 20 | 0 |
| `compressapi.h` | 31 | 31 | 0 | 0 |
| `timeapi.h` | 13 | 6 | 7 | 0 |
| Total | 1878 | 1046 | 100 | 732 |

Discovery reports single-identifier macro chains ending at native types or functions as
`macro_alias` targets. The runner excludes those preprocessing aliases from constant probing and
records the target in `inventory.tsv`. This distinguishes A/W selectors and callback-type aliases
from scalar constants, enumerator aliases, strings, and unknown expressions. Target lookup is
memoized per translation unit and only tokenizes requested alias chains. Cycles remain unclassified.
This rule unblocks capture for file, process/thread, PSAPI, registry, and service headers; it does
not claim that every preprocessing construct is classified.

Declaration attributes beginning with `__declspec`, `__attribute__`, or `[[`, including
single-identifier macro chains ending at those tokens, have a separate `macro_attribute` marker.
The runner reports them as excluded declaration helpers, not failed constants. Function-like
classification uses the definition's token positions and source bytes, including line splices,
rather than libclang's current-state query. A helper undefined after the header remains classified
as function-like. Parenthesized values, whitespace/comment-separated replacement lists, string
literals, and attributes redefined as values stay on the value path.

Named callable typedefs preserve calling convention, buffer annotations, and callback pointer
depth through existing RDL function declarations. Compiler TypeRef cursors recover written
function typedef edges when libclang expands them to raw function types; ambiguous edges reject.
Incomplete C records emit nominal declarations for pointer identity without claiming a layout.
By-value parameters and results still require proven layout. Variable redeclarations compare
canonical native types while retaining written typedef dependencies and checking their projected
contracts. This allows equivalent IID/GUID spellings without weakening declaration agreement.
Dedicated callback, opaque-record, and variable-redeclaration fixtures cover these boundaries.

Inline record-field function pointers capture a separate callable declaration anchored to the native
field cursor. Candidate identity combines the owning record's identity and direct field slot; it does
not merge equal function shapes or invent a native typedef. All retained TU observations participate
in signature and positional annotation agreement. Field annotations stay on field storage, while
callback parameter contracts keep their original source contexts. A field's libclang extent can end
at its name before the function parameters, so those contexts use compiler-attached parameter cursors
rather than the enclosing extent as an ownership test.

Projection names these callbacks from the qualified native owner and field, with a `_Callback`
suffix. Output collisions reject. Existing callback projection retains calling convention, written
typedef contracts, and pointer depth. `anonymous_callbacks.h` and target-specific RDL goldens cover
self-reference, counted inputs, outputs, unnamed parameters, distinct fields, and double pointers.
The conflict matrix changes signatures, annotation direction, parameter names, and TU order.
Variadic, unsupported-convention, no-prototype C, and nested anonymous callback positions reject.
`tests/ole.rs` compiles the real OAIDL/WIC closure for `IDispatch`, `ITypeInfo`, and `EXCEPINFO` on
x86/x64/ARM64 in both TU orders, using attribute vocabulary without default API references or
external value bindings. Metadata readback checks the self-referential callback, `HRESULT` contract,
record fields, and interface methods.

Record-member declarations are excluded as independent roots and captured through their owner's
dependency closure. Nonempty reserved-name macros remain selected; their name is not a reason to
hide a value or a failure.

`sysinfoapi.h` reaches projection after excluding `NOT_BUILD_WINDOWS_DEPRECATE`, retaining
55 emitted roots and explicit layout/import failures. `winnls.h` excludes the temporary
`DEPRECATED(x)` helper and `WINNORMALIZEAPI` attribute chain, but capture still fails on
`MUI_CALLBACK_ALL_FLAGS`: its replacement references `MUI_CALLBACK_FLAG_UPGRADED_INSTALLATION`,
which is unavailable in the configured input. No value is invented for that identifier.
Record layouts and unresolved macro probes remain shared blockers.

Emitted does not mean publication parity. For example, PSAPI emits the native `K32EnumProcesses`
name while committed RDL publishes `EnumProcesses` with `import = "K32EnumProcesses"`. The macro
alias target remains in the report, but public-name selection must be settled before publishing
these partitions. `compressapi.h` has complete source-to-RDL coverage; the other nine controls
remain incomplete. Keep the existing 98-name group as a separate passing control.

### Full manifest and orchestration audit

The independent x64 refresh attempts all 369 manifest headers using the pinned SDK. Emitted
closures pass combined WinMD compilation without bundled API references. These are source and
semantic metadata results, not production or runtime certification. Incomplete coverage produces
a nonzero exit.

| Measure | Current semantic refresh |
| --- | ---: |
| Headers attempted | 369 |
| Complete selected-root coverage | 52 |
| Incomplete coverage | 317 |
| Headers with some emitted output | 335 |
| Headers failing before discovery inventory | 24 |
| Known selected names | 99,515 |
| Emitted selected names | 81,268 |
| Rejected selected names | 18,247 |
| Selected names blocked before projection/output | 0 |

Name totals sum independent per-header inventories and exclude unavailable discovery counts. They
are not deduplicated APIs. Each header's supported roots must also pass combined projection and
source ownership before being marked emitted. The refreshed run also checks combined WinMD
compilation. Independent headers do not test agreement across the full manifest or across
architectures. The current x64 debug run takes 422.6 seconds. A separate baseline resource run
takes 784.0 seconds, peaks at 735.9 MiB working set, and has 636.9 MiB sampled peak private memory
(100 ms sampling). No controlled performance comparison is implied. A serial header run is not a
large multi-TU memory bound: capture retains the participating ASTs until the owned graph is extracted.
Streaming disposal and an agreed resource budget remain open.
More rejections can mean that formerly blocked roots now reach assessment, not that support regressed.

The baseline blocked-root boundaries below are absent from the current full-manifest snapshot:

| Header | Blocked names | Capture or combined-output boundary |
| --- | ---: | --- |
| `sspi.h` | 717 | Multiple selected aliases name the same record. |
| `xaudio2.h` | 120 | Callable annotation contexts share macro expansion positions. |
| `cfapi.h` | 106 | Multiple selected aliases name the same record. |
| `sensorsapi.h` | 43 | Multiple selected aliases name the same record. |
| `ws2tcpip.h` | 40 | Multiple selected aliases name the same record. |
| `xmllite.h` | 20 | Multiple selected aliases name the same record. |

The semantic gate exposed the `_` field failure rather than counting invalid RDL as emitted.
RDL now accepts that native record field name and preserves it through metadata roundtrips,
including nested structs and unions. It does not rename `_` to `unused`, which could collide with
a separate native field. UI Animation emits 64 of 134 selected names, rejects 70, and blocks none.

The alias collision is a combined publication boundary, not permission to choose a public name
from iteration order. Capture conflicts and ambiguous contexts also remain errors until their
source evidence is understood. The 24 unavailable discovery inventories are a separate prerequisite
gate; their unknown counts are not included in this table or the measured sums.

Shared-record publication clears the baseline's 926 blocked names:

| Header | Emitted | Rejected | Blocked |
| --- | ---: | ---: | ---: |
| `sspi.h` | 717 | 3 | 0 |
| `cfapi.h` | 106 | 16 | 0 |
| `sensorsapi.h` | 43 | 22 | 0 |
| `ws2tcpip.h` | 40 | 1 | 0 |
| `xmllite.h` | 20 | 12 | 0 |

Each profile passes combined metadata compilation without default API references. Source fixtures
cover root/TU order, retained named definitions, and unnamed/conflicting rejections. The real
SSPI/XmlLite alias closure passes on all three targets; generated COM aliases preserve native
identity and clone/release counts on x86/x64. The full refresh reproduces these focused counts.
Compiler source-location identity separates annotation tokens that share one macro expansion
position. The focused XAudio2 refresh clears its 120 capture blocks, emits 98 names, and rejects
22. The fixture checks distinct buffer-count parameter bindings, legal parameter renames, reversed
TUs, conflicting annotations, and retained override rejection on all three targets. The real
XAudio2 multi-TU capture passes on all three targets; inherited-slot projection, non-COM virtual
objects, packed records, and aggregate constants still reject. No header-specific exception is used.
The full refresh reports zero blocked selected names; 18,247 roots still reject and 24 discovery
inventories remain unavailable. Zero global blocks is not full metadata parity.

The macro-identity slice removes the global capture blocks in three large headers.
Independent names and their remaining rejected roots are:

| Header | Selected | Emitted | Rejected | Blocked |
| --- | ---: | ---: | ---: | ---: |
| `ks.h` | 458 | 369 | 89 | 0 |
| `wincodec.h` | 433 | 173 | 260 | 0 |
| `wmcodecdsp.h` | 833 | 605 | 228 | 0 |

Their shared failure was name-based probe classification, not missing native initializers or a
source qualifier conflict. Combining them with `devicetopology.h` in the existing main/satellite
profiles emits 1,174 of 1,798 selected names, with 624 rejected and none blocked. The combined
graph checks 2,363 available groups and 3,457 observations. All four emitted RDL sets compile to
WinMD and reach a stable metadata-to-RDL roundtrip. This checks semantic validity of the supported
closure, not publication parity, runtime ABI coverage, or certification of the rejected roots.
The full-manifest refresh above includes this slice.

Declaration-fragment classification removes ICU's global probe block. The independent `icu.h`
inventory selects 1,615 names, emits 1,583, and rejects 32, with no blocked roots. Its 12 declaration
helpers are excluded by source syntax, not by ICU names. Unsupported callable types and variadics
remain explicit rejections. Combining ICU with the four-header main/satellite control selects
3,413 names, emits 2,757, and rejects 656, with none blocked. The combined graph checks 4,164 available
groups and 5,265 observations. Both emitted RDL sets compile to WinMD and reach a stable
metadata-to-RDL roundtrip. These results extend the supported closure, not production publication
parity. The full-manifest refresh above includes the independent ICU result.

The tool audit separates source-derived policy from repairs that would conceal missing evidence:

| Boundary | Current behavior or remaining risk |
| --- | --- |
| Dependency types | Generic header runs use local captured closure, not external metadata bindings. Audio retains separate explicit external contracts. |
| DLL routing | Full ordered SDK import libraries supply exact COFF symbols and export targets; all candidates are reported. Conflicting entries within an archive reject, including shadowed archives. |
| Legacy production repairs | `LibraryMap` applies `LIBRARY_OVERRIDES`, including synthetic `InitializeXamlDiagnosticsEx` routing, and source-name fallback. The clang2 header route adopts neither. |
| Pointer-sized typedefs | Explicit SDK semantic contracts survive only after compiler width, alignment, and signedness checks. Width alone cannot establish `usize`/`isize` intent. |
| Selection | Inline helpers, empty markers, attribute/declaration fragments, declaration aliases, and record-member roots have reported exclusion reasons. Reserved names do not imply exclusion. |
| Input construction | Shared `clang_inputs` supplies source/include profiles; no legacy extraction result or parser fallback is consumed. Individual headers still need verified prerequisites. |
| Macro probing | Compiler file identity and byte ranges assign errors to appended probes. A clean reparse excludes rejected probes; fatal and unowned diagnostics still fail capture. |
| Annotation indexing | Ambiguity is retained until evidence consumes the indexed context. Unrelated CRT declarations do not abort capture; selected ambiguity still rejects. |
| Native availability | Inventory assessment rejects unavailable root closures explicitly. All observations of the available graph still pass strict agreement; rejected roots are not certified. |
| Metadata validity | Emitted roots pass combined WinMD compilation without default API fallback in generic profiles. Wrapper ABI, public-name parity, and DLL usability remain separate gates. |

The remaining failures include unavailable macro identifiers and non-value expressions,
declaration-only GUID/property-key data, packing and bitfields, unsupported union/anonymous shapes,
general C++ inheritance/templates, SAL lowering, and missing header prerequisites. Treat these as
shared source/capture/projection classes. Do not patch individual symbols to improve the totals.

Next gates are verified header profiles and record-layout coverage with native evidence.
Production replacement also
requires combined cross-header and architecture agreement, semantic RDL compilation, public
partition/name policy, DLL-contract checks, and resource budgets. The manifest run exposes the
remaining work; it does not justify switching the production Win32/WDK backend yet.

The focused callable and nested-record refresh runs the mandatory semantic compilation gate without
external OLE value bindings. These profiles remain incomplete:

| Profile | Selected | Emitted | Rejected | Blocked |
| --- | ---: | ---: | ---: | ---: |
| `oaidl.h` | 359 | 199 | 160 | 0 |
| `objidl.h` | 448 | 121 | 327 | 0 |
| `wincodec.h` | 433 | 174 | 259 | 0 |
| `oaidl.h` + `shellapi.h` | 693 | 527 | 166 | 0 |

The shared field-callable representation unblocks 64 OAIDL roots, including `IDispatch`, `ITypeInfo`,
and `ITypeLib`; WIC gains one root, and ObjIDL's selected coverage is unchanged. The combined profile
excludes `SHSTDAPI`/`SHDOCAPI` as declaration helpers, explaining its two-name selection difference
from the bounded union inventory. Both `SHGetNewLinkInfoA/W` retain their `_Success_` contracts.
Named nested records retain shared nominal references with checked scope-prefixed output names;
they are not copied into each field or merged by shape. This adds nine OAIDL roots and two ObjIDL
roots, including wire SAFEARRAY/VARIANT and GDI storage. Three-target reversed-TU metadata fixtures
check shared pointers, aliases, distinct same-shaped owners, collisions, and retained by-value gates.
Transparent interface aliases add 18 OAIDL and eight ObjIDL roots. Optional written-count outputs,
declaration-only data, and unavailable import contracts still reject. These are shared
representation/publication gaps, not reasons to externally bind `VARIANT` in the generic header
runner. This focused refresh does not establish public metadata parity.

Clang can attach a prefix annotation to a function while starting the function's extent at a later
declaration macro. The context index accepts that same-file prefix and chooses the nearest attached
declaration, not an arbitrary nearby token or API name. Inherited attributes keep the original
parameter context; tied conflicting contexts still reject. Prefix ownership also participates in
annotation filtering, so an in-range attribute cannot hide an owned prefix.
`annotation_macro_return.h` and its RDL goldens cover function and method prefixes, mixed
prefix/in-range annotations, and redeclarations with different parameter names before or after the
annotated declaration. Three-target, reversed-input metadata checks read back the original
zero-based bindings. No Shell-specific parsing or annotation exclusions are added.

### Failure-ownership and scaling checkpoint

The checkpoint preserves strict `capture` and `Snapshot::resolve` for production callers.
`capture_report` gives inventory callers explicit per-name macro rejections alongside the remaining
snapshot. Only diagnostics whose expansion file and byte offset belong to an appended probe are
isolated. Diagnostics are not classified by message text. The compiler error limit is disabled
for the probe batch so one clean reparse removes all reported failures. Error-bearing compiler
ASTs are never used as evidence. Fatal diagnostics, source errors, and unowned probe errors still
abort capture.

`Snapshot::assess` propagates unavailability over unique group dependencies, including written and
canonical edges from every observation. It checks the closed available graph with the same native
comparison and annotation agreement rules as strict resolution. Available conflicts remain global
errors; unavailable roots are explicit rejections, not accepted incomplete definitions. A rejected
name cannot retain a successful overload in the projected root set. Rejected graphs are not
certified for native agreement.

Annotation indexing retains ambiguous contexts without throwing for unrelated declarations.
Consuming that context in selected evidence still fails. The pinned CRT fixture checks both an
unrelated record and `_snprintf` itself. Mixed header fixtures check clean probe evidence,
cross-TU rejection, native conflicts, cycles, and semantic WinMD compilation. Batches with
32/64/128 bad macros take three capture parses each, not one parse per macro. Cyclic availability
fixtures check exact unique-edge counts.

| Real control | Selected | Emitted | Rejected | Blocked |
| --- | ---: | ---: | ---: | ---: |
| `dshow.h` | 2 | 2 | 0 | 0 |
| `winnls.h` | 732 | 727 | 5 | 0 |
| `winuser.h` | 4245 | 4137 | 108 | 0 |
| `wingdi.h`, `wincrypt.h`, `winioctl.h` | 9466 | 8689 | 777 | 0 |

NLS's unavailable identifier remains a failed probe; it does not acquire a guessed value.
The large graphics/crypto/IO control retains 14 unavailable native groups as rejections. Its
remaining failures include packing, declaration-only data, annotation lowering, and exact imports.
None of these partial controls is a publication gate.

One local x64 debug measurement used fresh processes, the full import manifest, and OS peak
working-set counters requested every 20 ms. Wall time includes startup and import loading; Rust
builds are excluded. Main and satellite inputs retain the shared source profiles.

| Input | Available groups | Observations | Declaration pairs | Wall time | Sampled peak working set |
| --- | ---: | ---: | ---: | ---: | ---: |
| Scaling, Toolhelp, Path | 134 | 137 | 3 | 1.45 s | 190.7 MiB |
| D3D12, D3DKMT, SetupAPI, NTSTATUS | 8024 | 8217 | 193 | 3.57 s | 267.1 MiB |
| Same main group plus DeviceTopology satellite | 8158 | 11624 | 3467 | 5.38 s | 362.4 MiB |
| GDI, Crypto, IO | 9749 | 9783 | 34 | 6.20 s | 247.6 MiB |

The multi-TU control uses four capture parses, including probes. It visits 11,636 availability
edges and 3,565 comparison type pairs. Discovery takes 1.72 s, capture 2.51 s, resolution 0.06 s,
per-root plus combined projection 0.73 s, and output 0.06 s. It emits 6,111 of 6,971 selected names
and reports 860 rejections. GDI/Crypto/IO uses four capture parses, 8,586 availability edges,
and 28 comparison type pairs; capture takes 4.42 s versus 0.73 s for projection.

These bounded runs expose capture and repeated parsing as the larger measured costs, not native
graph comparison. They do not establish full-SDK memory limits or worst-case per-root shared
closure cost. Keep plan-local projection caches; no global cache, parallel capture framework,
or alternate IR is justified by these results. Broader profiles, architecture agreement, and
semantic compilation of the large emitted partitions remain production gates.

## Rewrite plan and restart point

This page contains the continuation plan; resuming work must not require conversation history or
session-local notes. The objective is a manageable replacement around libclang, not another planner
over the old extractor's lossy output. Proceed through bounded gates, not an unconditional rewrite.

### Current baseline

The old `windows-clang` implementation remains available. `tool-webview` and the animation profile
in `tool-bindings` use clang2; the full Win32/WDK scrapers retain their existing path. Inspect the
worktree before restarting and preserve any local changes.

The green baseline includes the explicit BCrypt string contract and preserved WebView MIDL
annotations. Callable names are deterministic presentation data, independent of input filenames and
order. The first hard-layout slice projects checked union storage and direct anonymous fields through
`project/record.rs`, with semantic metadata and x86/x64 native storage/pointer checks. Continue with
packed storage, bitfield representation, and their calling policies as separate slices. Do not
remove the union/anonymous by-value gate or claim complete hard-layout support from storage equality.

Generated ARM64 union bindings pass isolated Rust size/alignment/offset assertions. The full ARM64
test-package build is blocked by the existing WDK fixture selecting an x64 MSVC compiler with ARM64
defines; do not treat that as native ARM64 execution or expand this storage slice to repair the
unrelated build profile.

The established WebView slice has no ignored cutover cases. Source-focused class fixtures also
cover UUID declarations, completion, ownership, and conflict rejection. These establish the covered
cases, not production parity or completion of the acceptance matrix.

An independent review of the BCrypt postcondition/import changes found no significant issues.
It traced annotation lowering, metadata readback, COFF import decoding, and test-owned lifetimes,
and reran focused regressions plus native crypto on x64/x86 in debug/release. It did not add fuzzing
or establish generalized ownership, opaque-storage initialization, or native ARM64 guarantees.

| Established behavior | Primary evidence |
| --- | --- |
| Owned native evidence, native distinctions, completions, and bounded graph comparisons | `test_clang2/tests/native.rs` |
| Redeclarations retain dependencies and annotation contexts; missing annotations cannot hide conflicts | `test_clang2/tests/observations.rs` |
| External references cannot hide conflicts beneath type, function, callback, or value roots | `test_clang2/tests/projection.rs` |
| Direct and indirect interface constants have the same explicit omission outcome | `test_clang2/tests/projection.rs` |
| Closed plans survive snapshot disposal and compile to semantic WinMD | `test_clang2/tests/projection.rs` |
| Real SDK `IUnknown` capture and external-interface pointer projection | `test_clang2/tests/projection.rs` |
| `__declspec(uuid(...))` decoding and local interface emission (GUID, single inheritance, inherited methods, `_In_`/`_Out_` parameters) | `test_clang2/tests/projection.rs::local_interfaces_project_real_com_metadata` |
| Local SDK `IUnknown` and `IClassFactory` metadata on x86, x64, and ARM64 | `test_clang2/tests/projection.rs::real_interfaces_project_locally` |
| Shared COM output lowering and incompatible type/ABI rejection | `test_clang2/tests/projection.rs` |
| Conflicting interface UUIDs fail native resolution | `test_clang2/tests/observations.rs::interface_uuids_must_agree` |
| Buffer units, direction, constant bounds, and original parameter bindings | `test_clang2/tests/projection.rs` |
| SDK BCrypt input/output byte lengths | `test_clang2/tests/projection.rs::real_sdk_buffer_lengths_project` |
| Optional buffers retain direction, lengths, and per-parameter optionality | `test_clang2/tests/projection.rs::real_sdk_optional_buffers_project` |
| Explicit string contracts preserve constness and direction; missing/invalid contracts fail | `test_clang2/tests/projection.rs` |
| Native record layout, free aggregate calls, and bidirectional inherited vtable dispatch | `test_clang2/tests/abi.rs` |
| Unsupported by-value COM record results fail before emitting bindings | `test_clang2/tests/projection.rs::com_record_results_are_rejected` |
| SDK COM wrappers preserve identity, reference counts, output ownership, and failure HRESULTs | `test_clang2/tests/com.rs` |
| Explicit scalar typedef contracts survive alias chains and reject conflicting projections | `test_clang2/tests/projection.rs` |
| Forward UUIDs agree even without a UUID on the complete definition | `test_clang2/tests/observations.rs::forward_interface_uuid_conflicts_are_not_discarded` |
| SDK same-name tag/typedef roots share output; field/callable contracts survive order changes | `test_clang2/tests/projection.rs` |
| Anonymous aggregate members retain native identity, layout, and conflicting dependencies | `test_clang2/tests/native.rs` |
| Integer enums preserve representation and values through metadata and bidirectional native calls | `test_clang2/tests/projection.rs`, `test_clang2/tests/abi.rs` |
| All 79 WebView2 consumer roots project across targets/orders and generated wrappers compile | `test_clang2/tests/webview.rs`, `test_clang2/build.rs` |
| Actual WebView consumer feature matrix and pinned loader execution | `test_clang2/examples/webview_consumer.rs`, `input/webview_loader.rs`, `.github/workflows/test.yml` |
| Native flags, selected typedef names, and MIDL property markers survive projection | `input/operator_flags.h`, `input/named_alias.h`, `input/midl_properties.h` |
| Bound pointer contracts and MIDL directions preserve eight WebView2 inputs and exact public signatures | `test_clang2/tests/webview.rs::consumer_input_contracts_cutover_gate`, `consumer_wrappers_preserve_public_parameter_shapes` |
| Handle/string wrappers reach native implementations on x64/x86 | `test_clang2/tests/com.rs::generated_handle_setters_and_strings_reach_native_methods` |
| Borrowed interface objects remain distinct from output slots in metadata and generated wrappers | `test_clang2/tests/projection.rs::interface_direction_distinguishes_objects_from_pointer_slots`, `test_clang2/tests/com.rs::generated_object_borrows_reach_native_methods` |
| Explicit enum flags survive; unsupported member layouts and overloaded COM methods fail visibly | `test_clang2/tests/projection.rs`, `input/flag_enums.h`, `input/member_alignment.h`, `input/overloads.h` |
| Double-NUL contracts retain terminator count and phase through metadata and native pointer calls | `test_clang2/tests/string_termination.rs`, `input/double_null.h`, `input/string_termination.h`, `input/double_null_native.cpp` |
| Real WDK member storage matches every native offset, size, and alignment | `test_clang2/tests/wdk.rs`, `input/wdk_layout.cpp` |
| Generated BCrypt wrappers preserve optional counted inputs, output bounds, and native DLL imports | `test_clang2/tests/crypto.rs` |
| Full/partial output byte postconditions survive RDL/WinMD; invalid counts reject | `test_clang2/tests/projection.rs`, `test_metadata/tests/method_params.rs` |

The test crate is at `crates/tests/libs/clang2`. Real-header local COM metadata is covered, with
synthetic positive and negative controls alongside it. The synthetic ABI fixture executes on x64
and x86. Local ARM64 linking is blocked by missing Visual Studio ARM64 compiler/runtime libraries;
native ARM64 execution is configured in CI but has not been observed for this change. Coverage
includes raw ABI calls and high-level COM ownership wrappers against a native test implementation,
plus eight real BCrypt DLL imports, not Windows COM activation or general C++ ABI parity.

Resume by inspecting the worktree and these modules, then rerun the baseline without updating
goldens:

```powershell
git status --short
$env:RUSTFLAGS = "-D warnings"
cargo test -p windows-clang2 -p test_clang2 --quiet
cargo clippy -p windows-clang2 -p test_clang2 --all-targets --quiet
```

### Issue-driven risk gate

Open issue repros challenge the rewrite independently of matching the legacy output. These are
prototype results, not fixes to the production backend or grounds for closing the issues.

| Issue | Reproduction and current outcome |
| --- | --- |
| [#4998](https://github.com/microsoft/windows-rs/issues/4998), in/out interface objects | The exact fixture rejects unsupported `_Outptr_`. Its supported `_COM_Outptr_` counterpart exposed incorrect in/out direction on a borrowed object. Direct object pointers now project as borrowed inputs, while interface output slots retain their direction. Metadata and native COM coverage distinguish both. |
| [#4967](https://github.com/microsoft/windows-rs/issues/4967), member alignment | Real WDK `DeviceIoControl` and `QuerySecurity` member types preserve offsets, size, and alignment. MSVC layout and pointer-call fixtures execute on x64/x86; ARM64 has capture/metadata coverage. Adjusted-record by-value calls still reject. |
| [#5042](https://github.com/microsoft/windows-rs/issues/5042), double-NUL strings | The SAL repro and bare-marker control retain different typed phases. Metadata roundtrips preserve terminator counts; generated native pointer calls preserve every string and both trailing NULs. Ordinary string binding does not replace the sequence contract. |
| [#5047](https://github.com/microsoft/windows-rs/issues/5047), explicit enum flags | The repro exposed a lost `clang::flag_enum` marker. Capture, agreement, and projection now preserve it through `FlagsAttribute`, with unchanged width and values and an unflagged negative control. |
| [#4186](https://github.com/microsoft/windows-rs/issues/4186), duplicate symbols | Relevant C++ cases distinguish rejected free-function output-name collisions from method overloads. A native x64 experiment showed that accepted same-name COM methods could dispatch incorrectly despite distinct generated Rust names. These interfaces now reject before RDL emission. WinRT duplicate properties and architecture overlays are outside this gate. |

The overloaded-method experiment added `Echo(int)` and `Echo(float)` to the native COM fixture.
The generated integer wrapper returned an incorrect value, even though metadata and Rust compiled.
Source declaration order cannot stand in for the compiler's vtable order. `overloads.h` retains the
declarations as a rejection regression on x86, x64, and ARM64; support requires an explicit native
slot-order contract and executable coverage, not a method-name reversal heuristic.

This gate found correctness defects, not just missing features. Correcting the bounded cases and
rejecting unproven layouts strengthens the stage boundaries but does not establish that all
remaining work is completeness. Full production Win32/WDK inputs and resource budgets remain
unproven; the bounded WDK/BCrypt slice above covers two real records and eight real DLL exports.

The independent review found another slot-layout defect: a derived pure virtual override reuses
an inherited native slot, but appending it in RDL adds a new slot. A compiler check for the Windows
MSVC target put `IDerived::F` at slot 0 and its new `G` at slot 1; the accepted projection placed
them at slots 1 and 2. That comparison used native compiler output, not committed bindings.
Capture now retains `clang_getOverriddenCursors` evidence, agreement checks it, and local
projection rejects inherited-slot reuse. `interface_overrides.h` covers direct and indirect
overrides, with and without the `override` keyword, and an external base binding on all three
targets. Slot reuse remains unsupported rather than guessed. The review established no further
high-confidence defect within its bounded pointer-contract and SAL/MIDL scope.

Committed metadata, RDL, generated Rust, and the legacy backend are comparison targets, not
correctness oracles. Annotation meaning requires native API-contract evidence; ABI and layout
require compiler evidence or native execution. Golden equality and metadata round-trips establish
consistency only. Caller-provided external bindings remain assumptions unless independently checked.

### Architecture rules to preserve

Keep entity identity, observation agreement, metadata projection, and output ownership separate.
Canonical types supplement written evidence; they never replace the observations or their
dependencies. Native compatibility runs before reference pruning, exclusions, output naming, and
deduplication for every selected root category.

Keep raw compiler handles in capture. Resolve an immutable owned graph into checked groups and
completions. Projection reads that checked result without mutating the native graph. Every planned
dependency must be a local planned item or an explicit external binding. Rendering must not
rediscover dependencies, resolve aliases, classify interfaces, or infer layout.

Missing, conflicting, and unsupported evidence must remain distinguishable. External/opaque
contracts may permit incomplete evidence, but cannot excuse contradictions or supply an assumed
local by-value layout. Unsupported local dependencies must not silently become `void` or disappear.
Report root outcomes explicitly: emitted, omitted by policy, or unsupported with a reason.

The domain is Windows C ABI declarations plus the COM, MIDL, SAL, and closed WinRT forms required
by the generators. General C++ merging, arbitrary templates, arbitrary inheritance/vtable
reconstruction, and whole-language ODR verification are not goals.

### Next gates: interop evidence, ABI limits, and SAL/MIDL relationships

The aggregate-returning COM method limit needs a downstream bindgen fix and bidirectional native
coverage before relaxing the projection rejection. Confirm the native ARM64 ABI run in CI.

The WebView consumer and loader gates preserve source enum representation, typedef names, and MIDL
properties and execute real browser activation. Keep the x64 debug gate while moving to other
real generator inputs.
Native agreement must still run before external bindings can suppress local output; do not rewrite
source contracts to match legacy generated metadata.

Remaining SAL and MIDL work includes indirect capacities, counted strings, and relationships beyond
the bounded prefix-direction decoder. Preserve explicit string bindings; do not invent unbound
`PSTR`/`PWSTR` names. Retain the local SDK interface, BCrypt, WinHTTP, and WebView2 cases as
regression gates. First record the exact selected roots, expected metadata, supported shapes, and
expected rejections in fixtures. Do not broaden the slice silently as new cases appear.

| Order | Work | Acceptance condition |
| --- | --- | --- |
| 1 | Local COM and UUIDs from `unknwnbase.h` | Metadata case covered: local `IUnknown` and `IClassFactory`, IID, inheritance, method order, system calling conventions, pointer levels, and COM output attributes on three targets. Synthetic executable ABI coverage is in gate 6. |
| 2 | SAL and MIDL relationships | Required/optional buffers, output valid-byte extents, decimal element constants, scalar strings, and MIDL prefix directions with SAL precedence are covered. Counted strings, indirect capacities, and other MIDL relationships remain. |
| 3 | Constants and preprocessing | Fully initialized GUID/property-key shapes have native-byte and roundtrip coverage. Declaration-only data, redefinition/undefinition, final macro state, and poison-expression recovery remain. |
| 4 | Record layout | Union and direct anonymous-field storage have semantic and x86/x64 native layout/pointer coverage. Packing, bitfields, inherited storage, and their calling policies remain; union/anonymous by-value calls reject. |
| 5 | Real multi-TU consumers | `tool-webview` and the header-driven `tool-win32 --clang2-audio` candidate run real consumers. Audio records every discovered declaration and emits per-header RDL, but 38 selected names still reject. Full Win32 cutover and a WDK case with UM references/enum overlays remain. |
| 6 | Target and ABI coverage | Raw-binding layout, free aggregate calls, bidirectional COM-style dispatch, COM ownership, and eight BCrypt DLL imports execute on x64/x86. Native ARM64 execution, Windows COM activation, SDK-wide DLL routing, and aggregate-returning methods remain. |

Use pinned real generator inputs where practical. Preserve main/satellite and WebView multi-TU
configurations rather than replacing them with a convenient single TU. Keep small synthetic
positive/negative controls alongside the real-header fixtures.

Query layout facts from libclang rather than reconstructing them from size guesses. The pinned
runtime was probed for `clang_getOffsetOfBase`, which the current `clang-sys` bindings do not expose.
If needed, use an updated or narrowly extended binding layer, not scattered dynamic symbol lookups.
An available offset API does not make arbitrary inheritance representable in metadata.

Keep annotation/token decoders small and independently covered; do not build another C++ parser.
Keep macro evidence, compiler evaluation, and projection eligibility separate. Failed evaluation,
nonconstant expressions, and unsupported constant representations need distinct outcomes. Preserve
definition/probe provenance and state a policy for context-sensitive macros.

### Acceptance matrix and review

Exercise valid combinations of root kind, local/external binding, alias depth, completeness,
contradictions, and graph shape. Cover functions, records, callbacks, constants, and typed values;
trees, shared graphs, cycles, and shared cycles; input order, filename changes, comparison direction,
and graph entry point. Document combinations that have no valid language meaning.

Pair accepted cases with controlled mutations of nested fields, pointer levels, calling
conventions, GUIDs, annotation relationships, and completions. Assert semantic invariance under
irrelevant ordering while preserving intentional preprocessing differences and source-order
ownership. Filename sorting must not decide semantic output ownership.

Measure unique comparison obligations and edges, including failure/retry paths, rather than relying
on recursion depth or timeouts. Preserve the existing 32/64/128-node work assertions. Review the
stage boundaries and acceptance results before declaring this gate complete.

### Production differential gate

Only after the fixed slice passes should remaining Windows rules be ported one family at a time.
Run both implementations on the same pinned inputs:

| Consumer | Required coverage |
| --- | --- |
| Win32 | x64, x86, and arm64; main and satellite TUs; DLL routing and header ownership. |
| WDK | `ntifs.h`, `wdm.h`, and `offreg.h`; UM references/exclusions and enum overlays. |
| WebView | WebView2 and interop headers; COM/WinRT types, references, and selected exports. |

Compare semantic metadata, exported symbols, import routing, layouts, calling conventions, pointer
categories, annotations, ownership, and unsupported-root outcomes. Text equality alone is
insufficient. Classify every difference as preserved behavior, an intentional correction, or an
explicitly accepted support change. Matching a known legacy bug is not an acceptance requirement.
No unexplained disappearance or new unsupported root passes.

Agree on wall-time and peak-memory budgets before the production pass. Measure both backends on
the same machine over comparable runs, including macro reparsing separately. Capture may dominate
runtime and retaining more evidence may increase memory; a simpler planner does not prove either
cost acceptable.

### Cutover gate

Keep the prototype unpublished and side by side until each caller passes its production gate.
Share fixtures and semantic assertions, not the old planner. Backend selection must be explicit;
there must be no automatic fallback to `windows-clang`.

Decide the public API migration for the builder and inspection consumers such as `Snapshot::facts`.
A compatibility view may preserve useful inspection, but must not feed lossy old types back into
native resolution. Retain procedural tests until each case is verified and moved to dedicated
`.h`/`.rdl` fixtures where appropriate.

Migrate callers only after their gates pass, then remove the old backend and temporary selector.
Do not keep two public pipelines indefinitely. Commits and merges remain maintainer-controlled.

### Stop conditions

Stop or narrow the rewrite if required evidence can only be guessed, compatibility depends on cycle
entry, production needs unrestricted structural matching, or the fixed slice requires global name
propagation, native graph mutation during projection, silent omissions, or fallback.

Also stop if support accumulates exceptions outside the responsible projection rule or cannot meet
the agreed resource budget. A narrow missing C API may justify updated bindings or a version-pinned
native helper. A broad missing AST surface warrants reassessing LibTooling and its C++/LLVM-version
coupling. Replacing the parser does not remove Windows projection policy.

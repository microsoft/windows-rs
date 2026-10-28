# windows-clang

Unpublished native capture and metadata projection. See the
[readme](../../crates/libs/clang/readme.md) for API usage.

## Rebuilding the default WinMD

`cargo run -p tool-win32 -- --partial --sdk-imports --sdk-macros` captures the full SDK manifest,
then captures the WDK against the freshly compiled SDK metadata and publishes the combined WinMD.
Selected-header and `--first` runs do not trigger WDK capture or replace the default WinMD.
`cargo run -p tool-win32 -- --wdk --partial` refreshes only WDK headers against the current SDK RDL,
then compiles and publishes both surfaces.

`cargo run -p tool-win32 -- --compile` compiles the existing `metadata/win32` RDL and metadata
attribute definitions against `Windows.winmd`, then compiles `metadata/wdk` against both references.
It checks each input for duplicate per-target items before merging with same-named enums unioned.
The merged output is checked again, then renamed over `crates/libs/default/Windows.Win32.winmd`.
Compilation, validation and merge failures leave the published WinMD unchanged. Intermediate WinMDs
are written under `target/win32`; replacement requires that directory and the destination to share
a filesystem.

This command does not scrape headers, refresh WDK declarations, or rewrite RDL. It cannot be combined
with scrape options. Publication does not establish complete SDK or WDK coverage. Unsupported
declarations and incompatible shared definitions remain explicit omissions; retained legacy WDK
declarations are not copied into fresh output.

WDK capture uses `ntifs.h` before `wdm.h`, with kernel architecture macros derived from the compiler
target. A separate Offreg input includes real Windows headers rather than synthetic Win32 typedefs.
Only functions selected by target-native `ntdll.lib` and `offreg.lib` symbols become automatic roots.
The pinned WDK has no x86 Offreg import library; this is an import-evidence gap, not proof that the
API cannot exist on x86. Configuration, library evidence and omissions are under `target/win32/wdk`.

Shared type reuse remains a coverage boundary. WDK `STRING` supplies field-buffer facts absent from
the SDK definition; reuse leaves that metadata unspecified rather than adding promises. WDK
`UNICODE_STRING.Buffer` uses a counted `PWCH` buffer, while the SDK uses `PWSTR` with null-termination
typedef evidence. That distinction remains an omission, including dependent `OBJECT_ATTRIBUTES`
APIs. Typedef-owned return contracts through `PNTSTATUS` also remain unproven.

The final merge rejects overlapping non-enum ownership, conflicting shared enum values and duplicate
items on any target before replacing the default WinMD. Generated RDL directories are updated before
final publication and are not transactional; a failed run can leave RDL changes for review without
replacing the default WinMD.

## Architecture

`Headers -> owned native IR -> reconciliation -> publication -> RDL/WinMD -> Rust bindings`

| Module | Responsibility |
| --- | --- |
| `runtime` | Compiler sessions, precreated indices and target/TU workers |
| `native` | Checked companion queries for facts missing from stock libclang |
| `capture` | Owned declarations, types, attributes, values and provenance |
| `resolve` | Same-target native agreement without replacing original observations |
| `merge` | Cross-target association with separate layouts and type graphs |
| `projection::plan` | Public families, aliases, primary names and compatible callback names |
| `projection::publication` | Values, UUID tags, records, enums, aliases and final header partitions |
| `projection::records` / `bitfields` | Storage, bases, fields, nested ownership and bitfield grouping/emission |
| SDK driver | Compilation contexts, root/macro selections, imports and publication policy |

Each worker reuses a precreated index across TUs. Index creation must not overlap capture: libclang
repeats LLVM target registration, which does not permit concurrent registry access. Batches are
serialized; native hangs affect the process. Type/cursor lookup uses hash indexes and file paths are
interned.

Snapshots retain declarations separately from entities. `resolve::records` establishes native groups
and queues definition checks; `resolve::values` checks function/constant observations. Attribute and
enum checks precede the final shared type worklist. Resolution succeeds only after every obligation
succeeds. Native agreement is not public-name compatibility.

Partial capture reconciles successful observations before pruning reverse dependencies and
compacting exclusions. Known conflicts remain fatal even behind unsupported dependencies. Failed
observations are unvalidated omissions, not compatible definitions. Tag/alias failures propagate
across targets; macro-value exclusions remain separate from native-declaration exclusions. Pruning
removes dependencies unreachable from surviving roots and keeps nested declarations with their
owner. No declaration is reparsed to manufacture agreement.

`validation_pruning`, `root_scope`, `publication_*` and the native/public conflict fixtures check
these boundaries, input order and target variation.

## Identity, ownership and names

Named namespaces retain qualified native identities through capture, comparison, selection and
pruning. Local Win32 publication flattens names only at emission; distinct native declarations
cannot share a public type/value name, even with identical bodies or disjoint targets. Reopened
namespaces share identity; anonymous enum constants retain their owner's namespace.

Only unqualified by-value aliases connect candidate record families. Pointer/array aliases do not
equate pointees/elements. Every definition must agree on kind, fields/order, offsets, packing,
alignment requests, bit widths, layout, types and qualifiers. Record edges must belong to checked
families. Native and public comparisons share the type checker with different record relations.

`AliasObservation::defines_tag` comes from the native typedef-definition query. A single native
record/enum uses its unique defining value typedef, otherwise its tag. A multi-record family needs a
unique defining typedef common to every member, or a unique common alias if none defines a tag.
Ambiguity fails even in partial mode. Names are not chosen by header order or stripped underscores.
For example, `GUID` names `_GUID`; multiple defining typedefs leave `_ACTRL_ALISTA` as the
definition. Other reachable aliases remain published.

Unqualified void typedefs publish without a storage layout. Pointer uses retain their written
aliases; function, method and callback returns with canonical void type have no result value.
The `void_aliases` fixture covers alias chains, const pointees, target-specific record/void aliases,
metadata round trips and raw Rust signatures. Qualified void typedefs remain unsupported.

Alias ownership prefers a unique header containing the native definition; otherwise all declaration
headers remain candidates. Ownership is combined across targets before partitioning. One owner keeps
its partition; multiple owners use reserved `shared.rdl`. Repeated inclusion of one source header
does not create shared ownership. `Partition::headers` retains real source headers. Missing owners
and ordinary filename collisions fail. No canonical-metadata or per-name routing preference enters
projection.

Architecture variants are sorted by canonical formatted RDL, matching the metadata writer.
Attribute token order before formatting does not affect declaration order.

Named integer alias families publish as `usize`/`isize` only when every x86, x64 and ARM64
observation is an unqualified integer with pointer-sized storage/alignment and consistent signedness.
Native types and reconciliation remain unchanged. Alias-owned contracts and reference ownership
still apply; pointer/array aliases retain written dependencies on these native-sized aliases.
Fixed-width integers, enum backing types and incompletely observed families are not generalized.
The `native_sized_aliases` fixture checks these boundaries, contracts, native conflicts, both
metadata round trips and rich/raw Rust types and layouts on all three targets.

Win32 partitions merge into the same `Windows.Win32` namespace. The shared filename does not give
their declarations different type identities. Package generation separately derives namespaces and
features from RDL filenames, so it observes partition renames even when flat WinMD identities match.

Nested records use `Record::owner`. Named children correspond by scoped name; unnamed children use
their first owning native field. `Snapshot::record_identity` supplies the same owner-relative key to
resolution, cross-target association and publication. Source offsets/display spellings do not
control identity. Stock `clang_Type_visitFields` supplies implicit anonymous fields and offsets; the
anonymous-record predicate is on the field's type declaration, not the field cursor.

Unshared direct fields can remain inline in RDL. Shared, named or indirectly referenced children use
owner-scoped declarations and `Outer::Inner` paths. A shared anonymous native type gets one
definition, not independent copies. `RecordOwner::field` supplies explicit anonymous provenance:
inline RDL carries it implicitly, and other unnamed children use `#[anonymous(N)]`. Named children
are never inlined merely because their names resemble generated owner/index names.
Promoted fields receive checked `Anonymous`, `Anonymous2`, etc.
names; collisions fail. Public anonymous member enums become record constants. Named member enums
are unsupported and omit their owner and users. Private enums remain native evidence. Aliases
declared inside records remain unsupported.
Semantic ownership also distinguishes member-introduced global tags from true nested records.

Methods, fields and parameters have no linkage, so SDK A/W aliases such as `DrawText -> DrawTextA`
must not rename them. When only object-like macros whose replacement is exactly one identifier
produced the name, including through pass-through macro arguments such as `STDMETHOD_`, capture
retains the written name. Token pasting and names produced by function-like macro bodies keep the
compiler name. Imports, data and enum constants keep the compiler identity, so `DrawTextW` remains
an explicit ABI. The `written_names` fixture covers these cases, including macro chains and `#undef`.

Fixtures: `primary*`, `shared`, `public_names`, `record_alias_*`, `namespace*`, `nested_records`
and `elaborated_records`. They cover native conflicts, owner families, architecture-specific
record/alias variants, metadata round trips and selected Rust assignability.

### Root selection and external metadata

| Input option | Meaning |
| --- | --- |
| `roots` | Explicit qualified names and their dependency closure; empty enables automatic roots |
| `root_paths` | Canonical files/directories eligible for automatic roots; `None` permits all, empty permits only source/explicit selections |
| `namespace_roots` | `All` named-namespace declarations as automatic roots, or `Binary`: C-linkage functions and UUID records only; does not restrict global roots or dependencies |
| `function_symbols` | Exact target-native linker symbols eligible as automatic function roots, including macro aliases |
| `external_interfaces` | Explicit native COM definitions whose members belong to authoritative metadata |
| `macros` / `macro_headers` | Explicit object-like macros / active definitions owned by selected headers |
| `alias_variants` | Additional argument sets; function aliases whose target changes across them are not published |

All observations remain available for native agreement and dependencies. Generated macro-probe
declarations cannot become roots; authored expansions outside exact probe offsets retain theirs.

`projection::Options` separates imports from `References`. `References::new(&index)` uses the
caller-supplied WinMD index as an external authority. Global native names are looked up among
non-WinRT definitions across its metadata namespaces; matched definitions retain their full
metadata identities rather than the local output namespace. Architecture variants are selected before
checking ambiguity. Multiple matches fail when the name is used; unused ambiguous names do not prevent
other references.

References cover value typedefs, enums, records and their nested types, callbacks, and interfaces.
Matched definitions are not emitted locally, except checked enum extensions in the owner's namespace.
Shared enum members must agree; extra members survive the SDK/WDK merge. Distinct local aliases retain
links to referenced types, including through pointers and callback signatures. The referenced
metadata owns the published contracts; native observations remain available for native reconciliation.
There is no HRESULT, BOOL, or string-type normalization policy. Missing matches follow ordinary local
publication, including its unsupported-type diagnostics.

Reference reuse separates canonical ownership from native compatibility:

| Boundary | Rule |
| --- | --- |
| Ownership | A direct record binding is stable, including through referenced typedef chains. Alias-derived ownership requires a unique candidate when no direct binding exists. |
| Secondary aliases | An alias may name a separate metadata record after field, type and layout checks. Alias rejection does not invalidate canonical ownership. |
| ABI | Field names/order, signedness, pointer shape, packing, record/member alignment and layout must agree. Unsupported shared shapes, including bitfields, remain omissions. |
| Contracts | Supplied metadata owns record, value-typedef and callback contracts. Native annotation differences do not rewrite those contracts or prevent reuse. Canonical type comparison expands typedefs without modifying either observation. |
| Opaque pointers | SDK-owned void-pointer aliases may describe typed native data pointers with matching target width, indirection and constness. This does not erase function pointers or COM references, or make native declarations equivalent. |
| Callbacks | Named delegates and structural function pointers compare by target ABI, parameter count and canonical result/parameter types. Direction, optionality and buffer contracts come from the referenced callback. |

`reference_pointer_ownership` covers opaque SDK aliases, fixed-width pointers, referenced fields
and callbacks, rejection boundaries, and conflicting native observations. SDK `usb.h` supplies
opaque `PMDL` and `PIRP` outside kernel builds. WDK publication retains those SDK aliases while
preserving the local `MDL` and `IRP` records. No header-specific alias exceptions are needed.

The SDK `winternl.h` and kernel `ntdef.h` definitions of `UNICODE_STRING` have matching layouts but
different `Buffer` contracts: typedef-owned termination evidence versus field-owned optional
byte-counted storage. The supplied metadata selects the publication authority; merging the native
captures without that boundary still rejects their field-attribute disagreement.
WDK's `ACCESS_STATE` reuses the SDK-owned type without changing its published contracts.

Reference metadata is not enriched or rewritten by native-only evidence. Already-owned
functions/constants still require matching projected declarations. Native forward-enum declarations
may omit SAL present on the definition, but cannot add conflicting evidence.

Actual C++ scopes remain distinct. Explicit namespace wrappers support SDK spellings: empty means
the metadata namespace directly, `ABI` means the SDK ABI wrapper. A configured delegate prefix
maps `IName` only to metadata delegate `Name`. Namespace spelling, an `I` prefix and IInspectable
inheritance alone do not establish ownership without supplied metadata.

`References::native_interfaces()` derives global native COM names and qualified COM/WinRT names for
capture. WinRT namespace identity is required both here and during projection lookup, so unrelated
global declarations with the same leaf name remain local. These definitions retain native identity,
UUID, attributes, layout and bases without traversing member definitions.
Reconciliation rejects inconsistent local/external ownership. Publication requires matching
authority, UUID and pointer-sized interface storage. WinRT references require their
IUnknown/IInspectable base; native COM references require the metadata's explicit base identity.
Pointer shape, qualifiers and local calling conventions remain native use-site contracts.
Capture ownership hints are not target-specific: each hinted interface needs matching metadata
on every target where it is captured. Ordinary value definitions still undergo full native capture;
references do not bypass unsupported capture diagnostics.

Local interfaces can derive from explicitly external native COM interfaces. Metadata supplies the
inherited slot prefix, which local compiler-reported slots must follow. Multiple metadata bases,
cyclic inheritance and external slot overrides are rejected. This does not validate external member
signatures against the header: those definitions remain owned by the configured metadata.

Ordinary capture with references instead validates supported scalar member contracts: slots, names,
convention, HRESULT, scalar widths/signedness and output indirection. This is not general WinRT ABI
lowering. Missing authority, incomplete definitions, by-value use and local inheritance from
external WinRT definitions remain rejected. There is no local stand-in after failure. Referenced
definitions and same-identity aliases are not emitted locally; distinct authored aliases retain
their own attributes.

Unqualified, unannotated native COM typedefs are checked against the referenced interface's nominal
identity, not as by-value storage. UUID, base and layout validation still applies to the interface.
Actual by-value uses remain rejected.

`reference_types` compiles a producer header to WinMD, then consumes its definitions from another
header and output namespace. It covers ordinary names alongside HRESULT, aliases, target variants,
nested records, callbacks, constants, local fallback, ambiguity and native conflicts. Both
`Arbitrary.Types` and `Windows.Win32` producer namespaces are exercised. Raw Rust consumers select
only the consumer record; bindgen includes its referenced nested record's enclosing definition.

`external_references` and `namespaces` cover identity/ABI conflicts and use-site
rejection with authoritative WinRT references. Composition
requires the desktop header's complete `IDesktopWindowTarget`, not its interop forward alone.
`native_reference_ownership` covers global COM references, local inheritance, metadata round trips,
UUID/base mismatches, slot-count disagreement and ambiguous metadata ownership on all targets.
Its `native_reference_aliases` fixture covers explicit interface typedefs, pointer-containing records,
dependent imports, by-value rejection and annotated aliases.
Consumers check ten interop vtables, casts, shared buffers, HRESULT errors, delegates and real
dispatcher-queue creation/shutdown. x86/x64 execute on an x64 host; ARM64 is compile-only there.

## Types and record storage

Written and canonical types, native qualifiers, attributes and layouts remain separate evidence.
Every type passes the native qualifier query before interning. Equal metadata widths do not equate
native `int`/`long`, `char16_t`/`u16`, or `char32_t`/`u32`. Character scalars project to unsigned
storage only after native checks.

| Contract | Supported boundary |
| --- | --- |
| Arrays | Compiler-evaluated extents, including expressions; `None` is incomplete, `Some(0)` is zero. Parameter decay requires matching canonical pointer adjustment and retains aliases/inner dimensions. |
| Qualifiers | CVR remains native evidence. `__unaligned` requires canonical unqualified storage alignment one; wider/incomplete storage and unknown qualifiers fail. No request is inferred from layout. |
| Mixed pointers | Per-level constness survives RDL/WinMD. Homogeneous chains retain the legacy leading-modifier encoding. |
| Fixed 64-bit data pointers | `Pointer64` retains pointee/qualifiers and native 8/8 storage; RDL uses `ptr64<*mut T>` or `ptr64<*const T>`. |
| Plain records | Sequential structs and offset-zero unions must reproduce compiler offsets, size and alignment. Class spelling uses the same public-field checks. |
| Incomplete records | Genuine global structs and classes become nominal pointee markers without layout. A definition in any input wins; failed definitions cannot fall back. |
| C++ data and handles | Public data records retain layout without publishing constructors. Empty classes, non-public storage, nontrivial destruction and opaque bases produce pointer-only markers. |
| Value inheritance | One public nonvirtual base at native offset zero becomes a `NativeBase` field; complete base/derived storage must agree. |
| Multiple interface bases | A native object retains each pointer-sized base as sequential `NativeBase` storage. Compiler base offsets, complete size and alignment must match, including observed local packing. |
| Packing/alignment | Retain requested packing and all evaluated alignment requests, not just resulting layout. Check field and containing ABI against the compiler. |

Multiple-interface-base objects expose borrowed base views, not whole-object COM ownership.
They require public nonvirtual, normally aligned interface bases and no added storage, virtual
methods, nested types or nontrivial destructor. By-value use and further inheritance remain
unsupported. `native_bases` checks metadata round trips, invalid layouts, packed object storage and
rich/minimal/sys consumers on all three targets. Native x86/x64 execution checks primary calls,
secondary `IUnknown` adjustment, reference counting and `QueryInterface`.

Incomplete array storage, C static/qualified array-parameter contracts, variable bounds and in-class
field initializers remain unsupported. The companion catches bitfield initializers hidden by stock
traversal. Constructors, nonvirtual destructors, conversions and friends remain native evidence,
not exported C++ methods. Non-public bases, unsupported inheritance and synthetic base-field name
collisions remain rejected. Incomplete unions, nested records and layout-attributed forwards are
not pointee markers. Opaque types cannot pass/return by value or occupy fields and arrays; their
public nested types and constants remain under their owner.

`RecordSemantics` retains compiler register eligibility and special-member properties. Parameters
and results have separate guards: nontrivial copying/destruction can affect parameters, while
user-provided constructors, constructor templates and nontrivial/deleted assignment also affect
results. Standard layout and trivial copyability do not establish a return ABI. `record_abi` covers
these distinctions, and `member_enums` covers record constants, named member enum omissions,
architecture variants, native conflicts and authoritative references.

The SDK driver selects GDI+'s flat/init/memory declarations and Color constants with
`GDIPVER=0x0110`, using `gdiplus.lib` for target-native import ownership. Out-of-line C++ members
cannot become global roots. `ColorMap` still requires
value storage for opaque `Color`; that API remains omitted rather than inventing a private layout.

Fixed-pointer queries decode native address-space semantics, not numeric address-space IDs. They
remove only the pointer-size qualifier from the pointee. Canonical native-width pointers keep native
equivalence; written typedef chains retain explicit width. Metadata's required width modifier
follows its `PTR`; optional, duplicate and misplaced modifiers fail. Mixed-width chains use
per-level qualifiers. Remapping and nested rewriting follow pointee references.

Bindgen emits a typed address helper with `from_address(u64)` and `address()`, not an integer alias
or native pointer. Exact-name/deep-pointer selection retains helper and pointee dependencies. 32-bit
extension forms, fixed-width function/interface pointers and unqualified fixed-width by-value
callables remain rejected. Ordinary pointers to storage records and adjusted array parameters remain
supported, including ReadFileScatter/WriteFileGather.

`MaxFieldAlignment` supplies pragma-pack requests even when tokenless records have equal layouts.
`AlignedAttr` retains evaluated bits and GNU/standard/declspec form, including zero and multiple
requests. Compare deduplicated request sets; an attributed forward's requests must be on its
definition. Complete records emit the maximum nonzero request within RDL's u16 limit. Typedef/enum
alignment, aligned bitfields and packed aligned value bases remain rejected. Packing caps natural
field alignment, not explicit requirements inherited through fields, arrays and intermediate
records. Projection retains packing and explicit alignment separately and checks every field
offset, final size and alignment against the native layout.

Member alignment belongs to the field without changing its public type. Bindgen inserts
target-evaluated `MaybeUninit<u8>` padding, raises containing alignment and avoids equality over
padding. Affected storage and nested/array owners omit Debug/equality; pointer-only owners retain
them. By-value callables containing inherited or member-aligned storage remain rejected pending
calling-ABI qualification. Mixed packed/aligned records also remain blocked by value. Pointers and
parameter references remain supported. `mixed_layout` covers mixed storage through RDL/WinMD.

Fixtures: `union_*`, `opaque_records`, `class_records`, `value_inheritance`, `fixed_pointers`,
`packing`, `alignment_*`, `issue_contracts`, `mixed_pointers`,
`byte_aligned_qualifiers`, `character_scalars` and `fidelity`. They distinguish equal-layout native
requests, failed definitions, target-width storage and metadata semantics.

### Bitfields

Capture retains evaluated widths, native bit offsets, unnamed padding and zero-width barriers.
Native/public comparison checks widths even if layouts match. Projection groups compatible
contiguous allocation storage, checks offset/size/alignment and splits at barriers. Unrepresentable
barrier effects fail rather than inventing padding or packing. Unions use separate offset-zero
storage fields. Checked backing names use `_bitfield` or `_bitfield1`, etc.; collisions fail.

`NativeBitfieldAttribute.MemberType` preserves signedness, bool/enum/typedef identity and single-bit
integer semantics independently of storage. Compatible signed/unsigned, enum and bool members can
share compiler-proven storage. Unnamed members remain native evidence without public accessors.
Typed attributes survive RDL and nominal-reference remapping. Untyped metadata keeps its existing
interpretation; typed metadata requires updated readers.

Consumer accessors resolve integer wrappers, enums, BOOL, pointer-sized storage and target-varying
aliases. Getters unwrap storage; setters restore wrappers and preserve neighboring bits. Bounds
checks use the consumer target's width, and nested types inherit owner architecture. Typed union
reads use unsafe; legacy untyped union accessors remain a separate consumer issue. Some MSVC
bitfield-only unions have alignment one despite wider storage; those remain rejected until their
storage alignment can be encoded without inventing a packing request.

`bitfields`, `typed_bitfields` and bindgen's `struct_named_bitfield`/`struct_arch_bitfield`
check native storage, round trips, raw/rich layout, sign extension, truncation,
neighboring bits and target-bound rejection. Raw layout success is not typed accessor acceptance.

## Callables and interfaces

`ValueDeclaration` retains global callable observations, parameters, written types, attributes,
native mangling and language linkage. External C imports need explicit DLL/export ownership. Native
mangling does not imply an export name. All same-target native conflicts remain fatal.

Import redeclarations may differ in labels and transparent unannotated value typedefs only after
native agreement. SAL references bind to each observation's own labels and written types before
comparison normalization. Check every alias observation, including unused annotation-only inputs.
Nominal types, qualifiers, callbacks and typedef-owned contracts remain distinct. Emit a whole
observed prototype, preferring more authored labels and then lexical rendered order; never combine
labels or emit the normalized comparison types.

Explicit C/stdcall spelling survives attributed written types on all targets, including where the
native ABIs coincide. Otherwise use the native prototype convention. Nested callback matching checks
written conventions too. Variadic imports use the compiler's C ABI even if a written stdcall request
was ignored; callbacks and methods remain nonvariadic.

Bare function typedefs publish named callbacks: a pointer or parameter adjustment supplies the
callable value, and further pointer levels remain explicit. These typedefs cannot name unrelated
inline pointers. Callback parameter evidence belongs to `AliasObservation`, not interned types.
Written aliases retain contracts. ABI-only naming excludes annotated/indirect prototypes.

Public families and aliases are validated before callback lookup. Convention/arity narrow
candidates, but the shared checker follows result and every parameter edge, native scalars,
qualifiers, nominal identities, layouts and public families. Failed candidates cannot leave assumed
type pairs for another attempt. Choose the alphabetically first compatible contract-free explicit
prototype across the target, independent of TU order; otherwise emit structural C/stdcall FNPTR. No
callback name is invented. A structural signature has no attribute owner, so noreturn or annotated
pointers require a compatible named callback. Supported exception specifications are native-only
evidence and do not prevent structural publication.

Failed alias/callback publication removes the declaration and dependents, not merely its root
marker, since dependency publication could otherwise retry it. Implicit compiler global aliases
participate in agreement but supply neither public names nor header ownership; authored aliases can
use their underlying type. Compiler identity/implicit-declaration queries establish origin, not
empty locations. The implicit original precedes authored redeclarations to avoid self-reference.

| Callable evidence | Publication boundary |
| --- | --- |
| `dllimport` / `dllexport` | Remain distinct native attributes; neither supplies the DLL. Supported exported C declarations can use separately configured imports. |
| `MSAllocator` | Profiling evidence only; no inferred malloc optimization, ownership, nullability or deallocator. |
| `noreturn` / `Cxx11NoReturn` | Function-type and declaration bits remain distinct; emit one DoesNotReturn attribute without changing result type. |
| `noexcept`, `nothrow`, empty `throw()` | Native-only written/canonical evidence checked during agreement and callback matching. No public exception attribute, unwind ABI, translation or consumer optimization is implied. Computed/dependent/nonempty forms fail. |
| Parameter defaults | Compiler-owned expression cursor, location and printed expression remain native-only evidence. No default-expression attribute, optional argument or arity change is published. |
| Method CVR | Const becomes NativeConstMethod; volatile/restrict/ref-qualified methods remain rejected. |
| Native lvalue reference parameters | Distinct metadata byrefs; bindgen uses pointer ABI and mutability without changing WinRT byrefs or caller-owned mutable storage. Fields/reference returns remain unsupported. |

A selected external C declaration may coexist with external C++ inline overload observations in the
same TU/target. Every overload observation must be inline and a definition must exist. They remain
provenance in `Snapshot::inline_functions`, not exports. Non-inline overloads, declaration-only
inline overloads, internal functions and C definitions remain unsupported.

Interfaces require one vptr, no data and at most one public nonvirtual base. Native queries supply
access, UUID, base offsets, virtuality and absolute Microsoft vftable slots. Public virtual
declarations need not be pure, but purity remains checked evidence. Definitions, multiple/virtual
inheritance, unsupported x86 thiscall and explicit undecoded attributes remain rejected. Member
templates, nonvirtual methods and method using declarations remain `non_vtable_members`, not slots
or claims of full C++ ODR support. Constructor inheritance and non-method using remain unsupported.

Order methods by native slots, then verify contiguous extension of the inherited prefix. Overloads
keep native names; bindgen's numeric suffixes must not collide with authored names. An override
requires one compiler-proven overridden method, identical slot and canonical return type;
covariance/adjustments fail. NativeOverrideSlot retains parameters/defaults/annotations. RDL clears
NewSlot; bindgen validates the inherited name/signature and reuses its method/vtable field without
another implementation requirement.

GUID-less interfaces omit the GUID attribute, including children of UUID-bearing bases; they do not
inherit or invent an IID. Clang never requests RDL's `guid(auto)` authoring behavior.
Bare vtable consumers have no automatic AddRef/Release. Bindgen's zero IID for
GUID-less IUnknown-derived interfaces remains an unresolved consumer-identity issue. Const pointers
to GUID-less interfaces remain unsupported.

Ordinary type publication omits volatile/restrict qualification, including through typedefs,
raw-pointer chains, record fields and their arrays. Native capture and reconciliation retain both
qualifiers. Generated field accesses are not automatically volatile; consumers must perform the
accesses required by the native API. The projection does not infer atomics, barriers or exclusive
Rust references. Constness and pointer layout checks remain unchanged.
Storage-qualified constants and C++ reference parameters remain unsupported, as do qualified COM
objects and methods. `pointer_qualifiers` and `pointer_qualifier_storage`
cover these boundaries without adding RDL syntax or metadata modifiers.

COM object constness remains native evidence but both `I*` and `const I*` project as interface
references. This is not immutability: DXGI's SetPrivateDataInterface retains/releases its `const
IUnknown*`. Outer storage constness remains: `I* const*` is `*const I`, `const I**` is `*mut I`.
Volatile/restrict COM object qualification is rejected. Ordinary record pointees keep constness.
Interface typedefs are transparent aliases; callable signatures remain canonical references.
A C++ interface reference `I&` passes the same object pointer as `I*` and projects the same way.
An interface parameter supplies an existing object, not a replaceable pointer, so SAL output and
in/out directions on it publish as input with the SAL evidence retained. The check uses the
canonical type, so interface typedefs such as `LPSTORAGE` behave like `IStorage*`.

An interface's GUID is its `uuid` attribute. Without one, it is the value of the global GUID
constant `IID_<name>`, the C spelling of that interface's IID, as in
`DEFINE_GUID(IID_IDirectSound, ...)`; the constant is still published. MIDL `[propget]` and `[propput]` comments between the start of a
method declaration and its name emit `#[special]`, so bindgen projects them as property accessors.

Packed interface declarations remain opaque references, not packed value records. Check
pointer-sized fieldless layout against inherited alignment capped by local packing; vtable layout
stays aligned. `windows-core::Interface::assume_vtable` performs alignment-safe object-vptr loads,
also used by weak-reference forwarding. Synthetic packed objects exercise the runtime contract; SDK
declarations do not establish that Windows returns unaligned objects.

Fixtures include `import_typedefs`, `import_conventions`, `callback_interop`, `function_typedefs`,
`implicit_aliases`, `inline_callbacks`, `callback_lookup`, `exception_specifications`, `noreturn`,
`cpp_members`, `native_references`, `guidless_interfaces`, `overrides`, `interface_overloads`,
`virtual_methods`, `packed_interfaces` and `interface_qualifiers`. SDK consumers cover AVI,
ID3DInclude/D3DCompile, allocation, context capture and selected native/COM producer calls. Native
query gaps have separate `opaque_callbacks` controls, including explicit regparm(0).

## Constants, enums and macros

Native constants and macro constants have separate reconciliation and cross-target identities,
using the same origin distinction as native functions and function aliases. A failed macro probe
cannot remove a same-named native constant. Each identity still validates its own observations
before pruning; conflicts within native declarations or within macros remain fatal.

When both claim a public name, strict publication rejects the collision. Partial publication
retains a defined native constant and reports the omitted macro, even when their values match.
When the native object has only declarations, partial publication may emit an independently
captured constant macro while reporting the missing native definition. The declaration remains
native evidence, not a definition supplied by the macro. Failed native definitions and projection
failures of defined constants still prevent macro substitution. Strict publication requires the
native definition. The `constant_macros`, `macro_declarations` and SDK GUID fixtures cover failed
probes, conflicting values, separate definition inputs, input order and target-specific preservation
through RDL/WinMD.

Constant observations retain written/canonical types, qualifiers, attributes, location and linkage,
including external language/mangling. `value: None` means a declaration without an initializer, not
failed evaluation. Mutable/thread-local objects remain unsupported. Compare every observation
against a definition when available; declarations cannot hide conflicting values. A missing
definition fails publication and removes the name across targets in partial mode.

| Value kind | Native requirement and output |
| --- | --- |
| Integer, bool, enum | Checked static initializer and native backing type/value; preserve nominal enum/typedef identity |
| Float/double | Preserve IEEE bits, signed zero, finite rounding/subnormals; reject non-finite/extended types |
| GUID / property key | APValue integer leaves, checked fields/layout and canonical GUID/PROPERTYKEY/DEVPROPKEY type |
| String arrays | ASCII char or valid UTF-16 wchar_t/char16_t; remove one final terminator, preserve embedded zeros/padding |
| Numeric data pointer | Native static initialization without relocation, side effects or UB; sign-extend target address bits |
| UUID-bearing forward tag | Compiler UUID becomes a constant under the native tag name, not a type or inferred CLSID/IID alias |

Successful `clang_Cursor_Evaluate` is insufficient: it can fold side-effecting `(call(), 3)`. The
companion verifies static initialization and rejects side effects/undefined behavior.
Compiler-supported field-offset expressions need not be C++ core constant expressions. APValue
traversal handles checked index paths and implicit zero-filled tails, not numeric tokens or host
object memory. String pointers, unterminated/invalid strings, arbitrary aggregate shapes,
function-pointer values and COM interface sentinels remain unsupported.

Global UUID class/struct forwards retain native identity/kind and all UUID observations. A
definition in any input prevents forward-only publication. Differently named type aliases cannot
rename the constant; a plain same-named typedef adds no public type. Value collisions must agree; an
unpublishable native object cannot borrow the tag UUID. Separately declared IID/CLSID values still
need definitions.

Enums have typed IDs/observations and retain backing type, layout, scopedness, attributes and
ordered members, including forwards. Native flag-enum attributes emit flags, as does a free
`E operator|(E, E)` declaration such as `DEFINE_ENUM_FLAG_OPERATORS` provides; value patterns do
not.
Aliases cannot equate distinct enums. Genuine anonymous enums publish loose typed constants only
without a public type name. Reserved unscoped numbered MIDL helper tags likewise publish loose
members unless an authored typedef supplies a name; this does not change native anonymity.
Uses of an enum without a public type name project as its backing integer type, matching its
constants. Unsupported annotations remain errors.

The approved native-enum policy retains 239 baseline names as members of 32 native enums, including
five signed interpretations of unchanged 32-bit patterns. A nearby scalar typedef such as
MediaEventType is not an enum alias. Constant and type publication namespaces remain separate.
Native `int`/`long` equality is not inferred from metadata width.

Unannotated conflicting scalar aliases can use their common native primitive type. Unique written
aliases stay intact. Check each alias chain across every observation, including annotation-only
inputs. Constant attributes remain, and complete published bodies must agree. Enum, pointer, record
and callback conflicts cannot use this fallback.

Macro selection uses active preprocessing definitions and real header ownership; superseded,
undefined, function-like or unselected definitions are not captured. Compiler tokens identify empty
replacement lists. `__typeof__` probes classify expressions; initializer probes preserve the macro
without adding parentheses, which would turn brace fragments into comma expressions. Source-authored
grouped expressions remain valid compiler input. Match the synthetic declaration and initializer
expansion by exact source offsets, not an earlier same-name expansion.

Discovery runs one incremental native parse per target through `windows_clang_probeMacros`: the
original TU is parsed once, then each candidate's type and initializer probes are parsed as
separate inputs against that state. A probe with errors rejects only its own macro, and its
diagnostics are reset before the next candidate, so each rejection is independent. Unbalanced
`()`, `[]` or `{}` in a replacement list rejects the macro without a probe, since it can never be
an expression and would leave the incremental parser inside an open scope. Failed probes
supply no capture evidence. Original-source errors remain fatal, and forced includes are not
replayed in probes. Accepted macros are checked again by the final capture parse.

Direct external C function references, optionally parenthesized, become macro aliases with target
name/USR, signature/contracts and macro-owner location. Calls, casts, pointer values and comma
expressions cannot. Native declarations and aliases retain separate identities: strict public
collisions fail; partial mode omits the alias without replacing native signatures. Target failure
removes aliases, not the reverse. An alias uses the target's native DLL/export ownership.

`Input::alias_variants` lists additional argument sets, such as `-DUNICODE`, for the same headers.
Each alias records the declaration it names under every set; an alias whose target changes, such
as a T-macro, is a configuration choice and is not published. Aliases without variants are all
published. The SDK and WDK drivers supply `-DUNICODE -D_UNICODE`. Native capture/agreement stays
independent. PSAPI publishes EnumProcesses-style aliases and native K32 names without a
spelling-based rename or downlevel-provider switch.

Fixtures include `scalar_constants`, `constant_declarations`, `aggregate_constants`,
`pointer_constants`, `layout_constants`, `enum_*`, `anonymous_enum*`, `native_enum_types`,
`scalar_constant_aliases`, `uuid_tags`, `macro_diagnostics`, `macro_recovery`, `macro_empty`,
`macro_initializers`, `macro_function_aliases` and `function_macros`.

### SDK macro and GUID contexts

| Context | Required invariant |
| --- | --- |
| winerror/urlmon, winuser/imm, ntstatus/mferror/wincred | Active definition ownership wins; never recover superseded definitions |
| certbcli, wincrypt, commctrl/richedit | Preserve written aliases, encoding selection, embedded NULs and native structure-size evaluation |
| mmreg/ksmedia/wmsdkidl | Companion owners retain speaker/WAVE_FORMAT_DRM constants; `__uuidof` GUID macros publish their native values |
| All SDK inputs | intsafe precedes Windows headers, supplying DWORD_MAX without displacing S_OK and keeping SAL expansion consistent |
| Standalone ntstatus | winternl supplies NTSTATUS; WIN32_NO_STATUS surrounds Windows headers |
| GUID definitions | Same native arguments, separate definition TUs, no repeated macro discovery or global INITGUID |

The `_ARM64_MULT_INTRINS_SUPPORTED` compiler-internal exclusion is approved, not a parser special
case. Four CERT_STRONG brace-initializer macros still need a destination type; none is guessed.
`uuid_macros::explicit_and_discovered_uuid` checks that explicit and discovered `__uuidof` macros
produce the same GUIDs.

## Annotation evidence and lowering

The SDK driver force-includes `sal.h`. Five source hooks transport contracts; structural wrappers
need separate handling. `__SPECSTRINGS_STRICT_LEVEL=0` is required because strict mode erases
contracts. `SalAnnotation` retains version, source-hook name/arguments and further expansion, not
guaranteed original header tokens or evaluated expressions.

The shared decoder publishes NativeSal arguments as SAL names/expanded arguments without internal
transport serialization. Quoted text and structural scopes survive; malformed transport fails. It
does not interpret C++ expressions or flatten conditional/targeted/failure contracts. Pre/post
qualifiers bind to the following property/group before sorting/comparison. Residual qualifiers keep
ordered NativeSal.Modifiers; swapped phase ownership cannot compare equal.

Native observations remain unchanged. Simple normalized contracts can compare equivalent; residual
evidence must agree. Typedefs may choose only an observed annotation set containing every other
observed set, never an invented union. Residual relocated typedef annotations keep TypedefName.
Unpublishable annotation owners remove dependents rather than discard contracts.

Direct, argument-free SAL2 `_Check_return_` and `_Must_inspect_result_` on imports, methods and
named callbacks emit one `#[must_use]`. Callback aliases inherit this result contract without
duplicating it. Output parameters, fields, constants and value typedefs retain their own evidence;
scoped, conditional and unfamiliar payloads are not promoted to unconditional callable contracts.
Native observations and agreement retain the source distinctions. The marker does not reproduce
SAL's dataflow analysis or promise Rust diagnostics.

Direct SAL2 `_Success_`, SAL1.1 `__success` and value-typedef `_Return_type_success_` lower to
enum-valued `#[success(...)]` for five bounded return comparisons: `== 0`, `!= 0`, `== 1`,
`>= 0` and `> 0`. Native types gate lowering: integer/Boolean and unscoped-enum results retain
their comparison, nonnegative requires signed storage, and data pointers accept only zero/null
equality or inequality. Float, scoped-enum, ordered-pointer and complex predicates remain evidence.
Scoped, competing and unfamiliar source forms also remain evidence; no C/SAL expression evaluator
is introduced.

Callable conditions remain explicit overrides, independent of typedef defaults. Value aliases
retain written links to annotated types, including annotations observed in another input.
Traversal stops at pointers; competing typedef defaults are not resolved by choosing an ancestor.
Named callbacks coalesce agreeing inherited conditions but retain conflicting evidence and owners.
Relocated nominal-type defaults retain their source ownership rather than becoming a type-wide
condition. `success` governs postconditions, not automatic Rust `Result` conversion.

Missing unconditional direction emits no direction attribute, including on raw pointers,
typedef-spelled pointers, references, scalars and COM references. Known direction always emits
`in`, `out` or both. Neither native mutability nor RDL type spelling establishes direction;
the reader preserves the absence of both metadata flags.

Exactly two unscoped SAL2 `_When_` branches can contribute shared direction and optionality when
their expanded conditions compare the same native pointer parameter to zero with `==` and `!=`.
The branch contracts must be plain `In`, `Out` or `Inout` forms, optionally nullable. Direction
uses the intersection, not the union; optionality requires both branches and does not override a
direct required contract. Both conditions remain evidence. For `RegQueryValueExA/W::lpcbData`,
`out` and `opt` are unconditional facts; the input requirement remains conditional. Those flags
do not replace the retained conditions or establish an output-only consumer conversion.
Nonexhaustive, scoped, compound and other predicates retain evidence without inferred common facts.

| Contract | Metadata semantics and limits |
| --- | --- |
| Direction/optionality | Direct SAL1/SAL2/RPC forms emit in/out/opt; optional argument storage is distinct from returned-pointer nullability. Composed distinct contracts retain evidence. |
| Reserved | Existing ReservedAttribute means zero/null, without implied direction/optionality. |
| Input | SAL1/SAL2/RPC input forms use RDL direction and optionality on every parameter kind. No duplicate NativeInput attribute; input does not imply COM object immutability. |
| NativeReturnedPointer | Successful inner nullability, one readable element for non-null success, and independent NullOnFailure. No inferred allocator/ownership. Requires native pointer-to-pointer storage. |
| COM output | Explicit COM intent on unqualified void** emits iid_is. Ordinary/typed pointers and null-on-failure alone do not infer it. |
| BufferExtent entry | Named `readable(entry, count)` and `writable(entry, count)` distinguish readable input from writable capacity. `bytes(count)` selects byte units. Direct integer counts and one dereference are supported. |
| BufferExtent success | `readable(success, count)` is distinct from capacity. Full SAL buffers preserve entry counts with `on_entry(*count)`. The bounded `p ? *p : n` form becomes `or(*p, n)`. |
| Count expressions | Parameters and integer sibling fields share bounded `+`, `-` and `*` expressions. Unsuffixed decimal/hex literals must fit `i32`; signed constant overflow is rejected. Arithmetic follows C promotions. Phases, optional counts and null fallbacks remain separate. Casts, `sizeof`, other operators and return-dependent lengths remain evidence. |
| Returned buffers | Indirect success extents separate capacity and initialized length. `_all_` promises only readable contents, using the successful count rather than `on_entry`. No one-element or ownership guarantee is added. |
| Optional counts | `if_present(*count)` leaves an extent unknown when count storage is null. It does not weaken returned-pointer initialization/nullability or supply zero/a fallback. |
| Inexpressible pointee extent | A single unscoped `_At_` post-readability property on this parameter's pointee uses `readable(success, unknown(), indirect)`, including byte units. Native payloads remain in reconciliation; other subjects, mixed properties and modifiers remain evidence. |
| RPC output capacity | `writable(success, count)` preserves the successful writable-capacity guarantee; inout forms do not acquire it. |
| NativeNullTerminated | Entry/success phases and separate double-NUL properties; no inferred direction, capacity or encoding. Bare properties have no inferred phase. |
| BufferExtent field | `writable(capacity)` and `readable(length)` describe valid-record fields without a call phase. Counts name direct integer siblings; `bytes(...)` and `nullable` preserve units and optional storage. |
| MIDL comments | Compiler-bounded leading bracket lists supply retval, output-only iid_is, optional and direction. Other relationships stay native evidence. |

RPC input/read-access distinctions survive reconciliation even if projection is identical.
By-value COM references retain output annotation evidence because supplying an object is not replacing
native pointer storage. Bindgen does not yet consume every typed annotation.

SAL1 `__deref_out_opt` permits null results; RPC `__RPC__deref_out_opt` instead aliases required
output storage with a non-null result. SAL1 `__deref_opt_out` permits a null argument and its `_opt`
variant permits both. Input/inout dereferenced-pointer forms retain inner contracts. The 24
returned-buffer forms lower outer direction/optionality and indirect success extents, not caller
capacity. Named counts and one dereference are supported; other expressions retain SAL evidence.
`_Outptr_opt_ _Inout_` remains two distinct contracts, not a permissive union.
Matching ordinary returned-pointer and buffer contracts can share their direction, optionality
and nullability while retaining both guarantees. An explicit pointer contract's null-on-failure
and one-readable-element promises are not supplied by the buffer contract itself.

Direct/full RPC input/output arrays, counted strings and supported inout forms share normalized
counts while preserving their different postconditions. Input readable lengths are entry evidence,
not initialized output lengths. Known capacity can survive alongside an unsupported length.
Output/inout xcount macros that discard count arguments do not acquire capacity. Optional inout
strings do not gain output termination absent from their SDK definition.

Integer capacities, lengths and sibling fields use the same bounded expression parser.
Simple additions do not have a separate offset contract: `count + 1` uses typed arithmetic.
Legacy metadata offsets remain readable through full attributes. Deeper indirection, outer-field
paths and unsupported expressions retain evidence. Optional initialized-length pointers need
their fallback honored before dereference.
Full-buffer forms can use capacity as length only when the source contract says so. Output/update
termination requires representable capacity. Direct pre/post single/double-NUL properties preserve
their own phases independently.

Parameter comments use exact compiler spans and innermost signature ownership; overlapping
macro-expanded spans do not establish ownership. Prose, preceding parameters and nested callbacks
cannot donate comments. A complete, standalone COM returned-pointer contract owns output-slot
direction; it can supersede MIDL's incoming-pointer flag. Other SAL and MIDL direction facts
combine conservatively. Ordinary read/write annotations, conditional annotations, mixed contracts
and opaque evidence cannot erase a MIDL direction. Both sources retain their native identities,
so direction disagreements between observations still fail reconciliation. `retval` and `iid_is`
use the same effective direction as parameter publication. Bare `size_is` markers supply no count,
and `unique` does not imply parameter optionality; an explicit `optional` marker does, including
on by-value parameters. Input/inout iid_is describes supplied interfaces,
not returned COM queries when the effective native direction remains input or in/out.
Explicit output comment markers may retain typed storage; no GUID parameter association is guessed.
Native query selection excludes counted GUID/object buffers rather than mistaking them for IID
parameters or query outputs.

SAL can be retained on supported callables, parameters, constants, records/fields, aliases,
callbacks and named enums. Annotated inline/nested callback parameters, bitfields, enum members,
anonymous enums without owners and expanded typedef owners remain unsupported. Fixture families
include `annotations`, `annotation_normalization`, `annotation_residuals`, `buffer_*`, `rpc_*`,
`input_contracts`, `native_input`, `pointer_parameter_flags`, `returned_pointer_contracts`,
`com_output`, `midl_comments`, `conditional_length`, `scoped_termination`, `field_buffers` and
`reserved_parameters`, with actual SDK macro controls. Round trips establish encoding, not
downstream semantics or issue closure.

## Native query boundary

`windows-clang-sys::LIBCLANG_VERSION` pins the runtime. Rebuild changed native inputs with `cargo
run -p tool-clang-sys -- --native`, then refresh bindings with `cargo run -p tool-clang-sys`.
Managed provisioning checks native build-input and DLL fingerprints under a cache lock, including
same-ABI changes; failed builds invalidate the stamp. External LIBCLANG_PATH remains caller-owned
and is never silently replaced. Fingerprints do not authenticate upstream caches or external
provenance.

| Query/evidence | Why stock layout or cursor information is insufficient |
| --- | --- |
| MaxFieldAlignment / AlignedAttr | Equal layouts can hide different packing or alignment requests |
| FunctionInfo | Plain FunctionProto can hide regparm and noreturn despite no attribute children; even explicit regparm(0) must be rejected |
| ImportConvention | Written/defaulted provenance and compiler-generated alternative symbols cannot be reconstructed from decoration |
| TypeQualifiers | Decode pointer-size/unaligned semantics before interning rather than erase unknown qualifiers |
| Static initialization / integer elements | Folded values alone do not establish side-effect-free initialization; aggregates need APValue leaves |
| Default arguments / field initializers | Exact expression ownership differs from arbitrary child expressions |
| Type visibility / deprecation | Decode tokenless defaults and message/replacement payloads, not unbounded cursor token dumps |
| Written member names | Cursor spelling is the post-expansion identifier; alias proof needs the preprocessor's macro history |

Default type visibility is retained and compared but needs no RTTI/linkage attribute for supported
plain records/enums. Hidden/protected type visibility and ordinary symbol visibility remain
unsupported. NoTrivialAutoVarInit retains compiler local-initialization policy without imposing a
caller storage contract or changing layout. NativeDeprecated retains messages/replacements,
including empty payloads and original typedef ownership, but does not request Rust diagnostics.
Unknown kinds/payloads remain errors, never inferred from empty tokens or matching layouts.

The stock C API also lacks the packing payload accessor; the companion reads
`MaxFieldAlignmentAttr`, not explicit `AlignedAttr`. Runtime/resource-header upgrades remain separate
from declaration work. See [windows-clang-sys](windows-clang-sys.md), `opaque_packing`,
`opaque_callbacks`, `visibility*`, `initialization` and `deprecations` for native boundary checks.

## SDK driver and accounting

`tool-win32` owns the SDK and WDK capture configurations. It captures all three targets, merges,
projects and compiles before replacing tracked `metadata/win32` partitions, preserving the readme.
Partial output, including deletions, stays visible for review; it is not evidence of full coverage.
Compiler errors and native conflicts remain fatal.

| Option/output | Meaning |
| --- | --- |
| Positional headers / `--first=N` | Select headers or a manifest prefix, not type roots |
| `--partial` | Publish supported subset and write omissions.txt; missing evidence is not absence |
| `--sdk-imports` | Read target SDK libraries through shared ordered import policy |
| `--import=Function=library.dll` | Explicit public ownership, ahead of native-symbol mappings |
| `--macro=NAME` / `--macro-header=header.h` | Explicit macro/name-owner selections |
| `--sdk-macros` | Discover filenames under SDK um/shared plus selected headers, not additional includes |
| `--dry-run` | Write attempted capture configuration without parsing or replacing metadata |
| `capture-configuration.txt` | Pins, targets, sources, arguments, selections and resolved imports |
| `omissions.txt` / `import-omissions.txt` | Unsupported declarations/discovery candidates and library exclusions |

Artifacts are under `target\win32`. Root policy admits um/shared/cppwinrt and explicitly selected
winrt headers; resource/UCRT/implicit MSVC headers remain dependencies. Required FILE, time_t,
size_t, va_list and stdint aliases retain real owners. Support WinRT name strings are not selected
by their InterfaceName/RuntimeClass prefix.

`header_contexts.rs` declares bounded namespace/include settings and SAL macro-support exclusions
for specstrings/rpcsal/driverspecs. Explicit header/macro selections remain eligible. Definition
phases stay driver-owned: lean Windows definitions, main/satellite/device groups and separate Olectl
mode avoid repeated unguarded GUID blocks. Storage includes share definition dependencies; shell
definitions explicitly include shlguid. Selected winioctl/winscard supply winioctl/winsmcrd to the
lean input. Olectl preloads ole2/ocidl and establishes DEFINE_GUID definitions, then clears INITGUID
before its guarded root. Do not enable INITGUID globally. Regenerate authoritative Windows.winmd
before Win32 capture when WinRT references change.

Import selection follows shared library order and checked overrides. Filter machine before ordered
selection; ARM64EC is not ARM64. Code/data/const kinds stay distinct; non-code evidence cannot fall
through to another provider. Known absent x86 winhvplatform/vertdll libraries are checked cases;
other missing/malformed libraries fail. Native capture does not guess providers.

Named import records retain their recorded export names. For ordinal-backed records, an exact
target-native compiler symbol identifies the C declaration; publication uses that declaration's
unqualified name and the library's DLL. This is the SDK named-publication policy, not proof from a
DLL export table. Ordinals remain in the evidence report; records without a matching declaration
remain omissions. No decorated-name parsing, frozen-name lookup or ordinal consumer support is
involved. Explicit public mappings still take precedence.

Canonical RDL under `metadata/win32` and `metadata/wdk` is the regeneration comparison point.
Review its diff against native header evidence and the focused SDK/metadata fixtures. Generation
needs neither historical Git objects nor separate export snapshots or approval files.

Eligible implicit C imports on x86 can carry Clang-generated stdcall alternatives. Keep original
observations; capture proves only the outer convention differs. Explicit mappings and ordinary
native mappings win before alternatives. Every observation must allow resolution; explicit
conventions, definitions, variadics, asm labels and unsupported properties cannot be overridden. No
global stdcall switch changes callbacks or compiler defaults.

Partial missing imports are target-local only after native/public contract validation. Conflicts,
invalid mappings, alias ownership changes and unrelated unsupported contracts retain their normal
fatal/all-target behavior. Strict publication fails. `target_imports`,
`sdk_import_availability`, `sdk_ndr_import_availability` and `sdk_import_providers` check
target-specific linkage directly against the pinned headers and import libraries.

## Validation

Fixture tests live in `crates/tests/libs/clang` and use the public API. Each focused test pairs an
input header with its expected RDL and checks the WinMD round trip; SDK-scale behavior is checked
by the committed `metadata/win32` and `metadata/wdk` RDL, the WinMD and the generated `windows`
and `windows-sys` crates, which the `gen` workflow regenerates and diffs. Use `UPDATE_CLANG_RDL=1`
with a selected fixture to refresh its golden RDL, then rerun without it. Never hand-edit generated
outputs.

`tests/issues.rs` keeps one fixture per reported issue in `input/issues`, using the issue's own
repro where possible. Each test names its issue; a golden RDL pins the reviewed result and any
expected omission is asserted explicitly. Annotation and GUID support macros live in `sal.h` and
`guiddef.h` there, outside macro discovery, as SDK support headers do.

```powershell
cargo fmt -p windows-clang -p test_clang -p tool-win32
$env:RUSTFLAGS = '-D warnings'
cargo test -p windows-clang -p test_clang -p tool-win32
cargo clippy -p windows-clang -p test_clang -p tool-win32 --all-targets
cargo check -p windows-clang --no-default-features
```

The SDK/native pair supports x64 and ARM64 hosts, not x86 host processes. Each native host parses
all three targets. Helpers provision the pinned SDK and matched runtime/resource headers;
LIBCLANG_PATH and CLANG_RESOURCE_DIR can select caller-managed inputs. The SDK contract tests in
`tool-win32` need Visual Studio's three-target MSVC/SDK tools:

```powershell
cargo run -p tool-clang -- path
cargo test -p tool-win32 sdk_ -- --include-ignored
```

Regenerate and compare the metadata after capture/publication changes:

```powershell
cargo run -p tool-win32 --release -- --partial --sdk-imports --sdk-macros
git diff -- metadata\win32 metadata\wdk crates\libs\default\Windows.Win32.winmd
```

`WINDOWS_CLANG_PROFILE=1` emits tab-separated phase/operation/count/time records. Native parse,
diagnostic formatting and disposal are timed separately; target workers overlap, so phase times
cannot be summed as elapsed time. Ask for a quiet machine before controlled measurements.

## Compatibility

windows-clang metadata requires
[updated paired readers/generators](windows-bindgen.md#metadata-compatibility).
Qualify inherited slots, typed bitfields, mixed qualifiers, fixed pointers, references, member
alignment and new annotation properties independently. Equal development version strings are not
source/release pins. Round trips alone do not establish consumer semantics.

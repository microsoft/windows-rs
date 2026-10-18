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

Partial capture reconciles successfully captured observations before following reverse dependency
edges and compacting exclusions across snapshots. Known conflicts in excluded dependents remain
fatal. Failed observations remain unvalidated and are reported as omissions; an identity with no
successful observation cannot supply a verified definition. No individual declarations are reparsed.
Unsupported tag and alias kinds are excluded across targets. Diagnostic causes are deterministic;
workers deduplicate diagnostics between TUs, and dependency edges are deduplicated on insertion.
`validation_pruning.h` covers conflicting records and functions behind unsupported dependencies,
input ordering, target-specific conflicts, and failed enum/alias observations.

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
Const, volatile, restrict, and supported `__unaligned` requests enter the IR, including qualifiers
inherited through typedefs. An unaligned request is supported only when the canonical type with
qualifiers removed has native storage alignment one. It cannot reduce that alignment, so publication
needs no additional modifier. Native reconciliation still distinguishes qualified and plain types.
Wider storage, incomplete types, void, function/reference types, and unknown qualifiers remain
explicit capture failures. The `byte_aligned_qualifiers` fixtures check records, packed records,
scalars, arrays, callbacks, native conflicts, and rejected requests on all targets. The SDK fixture
checks the actual `LPITEMIDLIST`, `LPCITEMIDLIST`, and `LPSHITEMID` typedefs.
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
in partial mode. Imported function observations may differ in parameter names only when their
published types, attributes, calling conventions, and return contracts agree. SAL references still
bind against each observation's original names. Publication chooses one complete observed prototype:
prefer more authored parameter names, then the lexically first rendered declaration. It does not
combine names from different observations or choose by input order.

Written aliases remain distinct public type identities. Native-compatible declarations whose
published types or contracts differ fail strict projection and become explicit all-target omissions
in partial mode. ABI equivalence does not establish public type equivalence: aliases can affect
default parameter direction, and downstream built-in types can have distinct behavior.
`function_redeclarations` checks name-only agreement and retained type conflicts on all targets.
`function_names` checks anonymous and partially named prototypes, deterministic selection of a whole
observed prototype, input/target order, metadata round trips, and pointer-alias rejection.
The 370-header workload remains byte-identical under this policy, including canonical export and
omissions. Four `HSTRING_User*` imports and `PerfStartProvider` still differ in written aliases,
not just names. They remain omitted rather than treating matching native ABI as public type proof.

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
| Scalar constant | Const integer, enum, bool, or finite float/double with a checked static initializer | Compiler-evaluated value and declared type |
| Aggregate constant | Constant-expression GUID or property key with checked native fields and layout | GUID literal or format GUID plus property ID |
| String constant | Terminated ASCII `char` or valid UTF-16 `wchar_t`/`char16_t` array | `String` with explicit native encoding |
| Object-like macro | Selected or discovered supported expression at the end of the input | Compiler-inferred written type and value |

Use `project_with_imports` or `project_partial_with_imports` after capture to supply DLL/export
ownership. Native C mangling is evidence, not an inferred DLL export name. Missing imports are
errors or explicit partial omissions. Callbacks, declared functions, and methods preserve parameter
names, using positional names only for unnamed parameters.
Imports, named callbacks, and structural function pointers preserve explicit C/stdcall requests
from Clang's attributed written types, including on x64 and ARM64 where native ABIs coincide.
Without a written request, publication uses the native prototype convention. Native conventions,
mangling, compiler defaults, and import matching remain unchanged. Callback-name selection checks
written conventions throughout nested signatures as well as native type compatibility.
Real target-dependent convention, signature, and import-name differences stay separate.
`import_conventions` covers named callable kinds and 64-bit-only declarations.
`callback_interop` checks named/structural assignments without casts, nested callbacks, both binding
styles, metadata round trips, all-target compilation, and executable calls on x86/x64.
Function `dllimport` and `dllexport` attributes remain distinct native evidence and must agree across
observations. Both can publish a callable import when its DLL/export ownership is supplied separately;
`dllexport` does not identify that DLL. Exported records and constant objects remain unsupported.
The SDK's ICU entry points exercise exported C declarations without overriding their header macros.

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

Genuinely incomplete global structs publish empty nominal pointee markers without class layout.
Pointers retain distinct native tags, aliases, and constness. A definition in any reconciled input
supplies the real record instead; complete empty or rejected definitions cannot become markers.
Callable parameters, results, and callbacks reject incomplete records by value, resolving
completeness across inputs rather than using a local forward's missing layout. Record fields still
require native storage. Incomplete unions, plain classes, nested records, and layout-attributed forwards remain unsupported
as pointee markers. SAL and deprecation annotations remain attached to accepted markers.
Forward-only ownership uses declaration headers; a real definition retains definition ownership.
The `opaque_records` fixture covers these boundaries through RDL and WinMD without Rust generation.

Global UUID-bearing class and struct forwards publish a GUID constant under the native tag name,
not a type alias or an inferred `CLSID_`/`IID_` name. Capture retains native record kinds, identities,
attributes, and separate observations. Native resolution checks every observed UUID before pruning,
including when there is no record definition. Class and struct redeclarations can share a non-union
identity; this does not equate their keywords or supply a layout.
Publication classifies the reconciled forward, not an arbitrary representative cursor. A definition
in any input prevents the forward-only path. Class definitions remain unsupported. GUID tags cannot
be used as metadata types, and differently named type aliases remain explicit omissions. A plain,
same-named typedef adds no public type. Existing interface definitions keep their interface contract.
Value-name collisions require matching published bodies; an unpublishable native value cannot borrow
the tag's UUID. The `uuid_tags` header/RDL fixture checks native conflicts, target variants, input
order, selected roots, matching macro values, type rejection, and both WinMD round trips.

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
The initializer uses the macro without adding parentheses. Otherwise, comma-separated initializer
fragments such as GUID field lists would become scalar comma expressions. Compiler parsing rejects
those fragments while preserving comma expressions parenthesized in the source macro.
`macro_initializers` covers this boundary for selected macros and discovery.
`Input::macro_headers` discovers object-like macros by source-header filename. Active bindings come
from conditional preprocessing references, excluding undefined, function-like, or out-of-scope
redefinitions. Compiler tokens identify empty replacement lists without an expression parse.
A validated TU supplies a temporary PCH for expression probes. Diagnostics from a failed batch
identify suspects, not invalid macros: parser recovery can blame a valid sibling. Each suspect
must fail an independent parse before rejection. If no failure is confirmed, the batch splits.
Confirmations use separate indices created before any capture starts. A CPU budget capped at 24
is divided across targets, with at least one worker per target. Each index is used by only one
thread at a time, and all confirmations finish before their PCH is removed. Results are applied in
candidate order, independent of worker completion order.
Survivors must pass a clean batch; no failed AST supplies capture evidence. Forced
includes are not replayed in probes. The final capture reparses the original input plus accepted
probes cleanly. Original-source and native parse-status errors remain fatal.
`macro_diagnostics`, `macro_recovery`, and `macro_empty` cover recovery, diagnostic limits, empty
active definitions, redefinitions, and valid siblings across the three targets.

Macro-constant exclusions are separate from native value exclusions. A rejected constant probe
removes constants of that name across targets, including failed dependencies and publication,
but does not remove observed native functions. A native declaration failure still excludes the
native value across targets. Known conflicts remain fatal before either kind of pruning.
Synthetic macro storage does not impose linkage on a same-named native declaration. All actual
constant declarations still agree on linkage independently of which observation represents their
group; canonical types, attributes, and values remain checked. `macro_declarations` covers
matching definitions and native linkage, type, and value conflicts in both input orders.
`macro_function_aliases` covers each failure stage and preserves functions only on targets where
they were declared. This matters for the SDK's x86 `*LongPtrA/W` aliases and 64-bit functions.

The native `typeof` operand retains cast typedefs without inheriting the synthetic initializer's
type. Compiler-predefined types such as `sizeof` results are desugared through their native kind,
not a general canonical-type fallback. `macro_discovery` covers rejected probes, forced includes,
and inactive macros. `certbcli_discovery.rdl` checks all 22 baseline constants and two additional
string macros across x86, x64, and ARM64, including ANSI and UNICODE configurations and `CSBFT`
identity. Its RDL-only test is independent of the deferred consumer test.

Unselected headers' macros are not captured and do not appear in the omission report. Constant
coverage therefore still needs a baseline comparison, not publication or omission counts.
The full SDK configuration below combines broad macro selection with manifest-derived definition
inputs; neither selection alone establishes complete constant coverage.

Selecting `winerror.h` and `urlmon.h` recovers all 7,456 frozen winerror constant names through the
same discovery path. All values match; 363 constants retain their written `DWORD` alias rather than
the baseline's `u32`. Eighteen `INET_E_*` definitions are active in urlmon, not winerror. Discovery
does not capture an earlier definition from a selected header when the active owner is unselected.
The override fixture covers that rule and evaluates dependent macros against the active binding.

Selecting `ntstatus.h` recovers its 3,072 baseline declarations with identical types and values,
without changing discovery or evaluation. The standalone fixture includes `winternl.h` for the
SDK-authored `NTSTATUS` typedef and suppresses Windows compatibility status macros with
`WIN32_NO_STATUS`. In the full workload, mferror and wincred own 18 active definitions in the main
TU; those observations remain explicit ownership diagnostics rather than superseded captures.

Winnt discovery and definitions publish 3,490 baseline names with identical per-target values;
32 retain written typedefs instead of primitive spellings. The remaining baseline name,
`_ARM64_MULT_INTRINS_SUPPORTED`, is an approved compiler-internal scope exclusion whose active
definition belongs to the unselected support header.
A separate lean Windows `INITGUID` input supplies definitions for 259 winnt and 11 winsvc GUIDs.
It shares the compiler arguments and target settings
of the ordinary inputs, but does not repeat macro discovery. All native observations are reconciled
before publication. The `winnt` fixture checks exact GUID values,
cross-input reconciliation, and input-order independence.
Every input includes `intsafe.h` before Windows headers. This provides `DWORD_MAX` for
`POWER_LIMIT_VALUE_NO_CONTROL` without displacing winerror's active `S_OK` definition. The common
support context also keeps native SAL arguments consistent: `ULONG_MAX` in
`KCRM_PROTOCOL_BLOB.StaticInfoLength` expands to `0xffffffffUL`. Mixing contexts must fail native
reconciliation even when record layouts agree; the fixture covers this failure in both input orders.

Selecting `hidusage.h` recovers all 2,833 baseline constants with exact `USAGE` and `USAGE_VALUE`
types and values on all three targets. The fixture covers both aliases, zero, and high unsigned
16-bit values. The only omission is the non-value `__HIDUSAGE_H__` guard.

Selecting `winuser.h` and `imm.h` recovers all 2,896 winuser and 221 imm baseline names, plus six
mouse/keyboard constants whose active definitions belong to winuser. `MOD_ALT`, `MOD_CONTROL`,
and `MOD_SHIFT` belong to imm in the full input. Values and target availability match the baseline;
75 constants retain written `LPSTR`, `LPWSTR`, `LONG`, or `DWORD` aliases. The SDK fixture covers
negative handles, high-bit scalar values, resource identifiers, and ANSI/UNICODE pointer types.
The eight 64-bit `*LongPtrA/W` native functions remain independent of their macro aliases.
Rust consumer validation remains deferred.

Selecting `wincrypt.h` recovers all 2,678 baseline constants with unchanged values, encodings, and
target availability; 173 retain written `LPCSTR` aliases instead of `PCSTR`. The 28 additional
strings include 18 algorithm names, nine registry paths, and one multi-string. The fixture checks
ANSI/UNICODE provider selection, pointer sentinels, concatenation, and embedded NULs through WinMD.
The 28 rejected candidates are 17 function aliases, seven guards/type/import macros, and four
`CERT_STRONG_{SIGN,KEY}_PARA_OS_*` brace initializers. Those initializers require a destination type;
discovery does not infer one from tokens or naming. They remain unsupported outside this baseline
closure, rather than being misclassified as missing captured values.

Selecting `commctrl.h` and `richedit.h` recovers all 2,157 and 567 baseline names with unchanged
values, encodings, and target availability; 13 constants retain written typedefs. Five commdlg
constants are owned by these headers in the active input. `EM_GETZOOM` and `EM_SETZOOM` belong to
richedit. The fixture checks ANSI/UNICODE control classes, pointer sentinels, unsigned notification
arithmetic, and target-dependent structure sizes. All 38 size macros use the native static-initializer
contract, not token evaluation.
Five new names are three clipboard strings, a drag-list message string, and a URL limit.

The mmreg selection includes `ksmedia.h` and `wmsdkidl.h`, the active owners of 20 speaker-position
constants and `WAVE_FORMAT_DRM`. It recovers 1,974 of 1,976 mmreg baseline names, plus 454 ksmedia
names and 47 additional wmsdkidl names. All 78 wmsdkidl baseline names are present. Retained values,
attributes, and target availability match; 55 constants across these headers retain written
typedefs.
`mmreg_ownership.h` checks the active-owner boundary with and without the companion selection.

The standalone mmreg discovery fixture has a compiler/PCH mismatch for its three
`KSDATAFORMAT_SUBTYPE_*` GUIDs. Explicit macro capture succeeds, but discovery's PCH probe reports
an incomplete `_GUID`. The SDK-independent `uuid_macros` fixture defines the complete type and
reproduces the disagreement; the matching Clang executable also reproduces it with a PCH.
This is not evidence that the source expression is unsupported. No name-specific exceptions or
guessed GUID values are used. Per-failure source confirmation is not used: an eleven-header SDK
benchmark with 954 failed-probe diagnostics exceeds 515.9 seconds without finishing capture, versus
338.8 seconds for capture/merge/project without confirmation.
The definition remains in the PCH but is not loaded when `__uuidof` uses Clang's builtin GUID tag.
A native diagnostic probe calls `ExternalASTSource::CompleteRedeclChain` before expression parsing;
this restores the real definition and GUID fields on all three targets while still rejecting a
genuinely incomplete GUID type. This libclang bug is deferred; the `uuid_macros` parity test is
ignored with an explicit reason, while the SDK fixture checks the omissions and explicit GUID values.
Only a small companion-side adjustment is in scope. No suitable pre-parse hook is established;
rebuilding libclang or duplicating its parser is not part of this work. Other discovery continues,
without counting affected GUIDs as semantically covered.
Without definition inputs, the companion selection has two missing mmreg names,
`KSDATAFORMAT_SUBTYPE_PCM` and `KSDATAFORMAT_SUBTYPE_WAVEFORMATEX`, rejected by ksmedia's
satellite-input PCH probes. Ksmedia has
460 additional baseline macro GUIDs with the same diagnostic. Three property-key declarations
still need definitions. Its UUID tag `KSEVENTSETID_VIDCAPTOSTI` has no value macro of that name;
the differently named `KSEVENTSETID_VIDCAPNotify` alias is also PCH-blocked. Publishing the tag
itself as a constant needs a separate policy decision. Successful discovery or independent definition capture in another input context does not close
the compiler bug.

MIDL gives anonymous IDL enums reserved, numbered tags. Publication recognizes unscoped
`__MIDL_..._NNNN` helper names, retaining an authored typedef when one is available and otherwise
publishing loose constants. Native tags, members, and compatibility evidence remain unchanged.
This is a publication policy, not a claim that the C compiler marked the enum anonymous.
Conflicting authored names are errors; scoped enums and similar unnumbered names remain named.
References to a helper with no public type and type annotations without an owner remain unsupported.

The installed SDK's `urlmon.idl` declares `MAX_ZONE_PATH` and `MAX_ZONE_DESCRIPTION` in an anonymous
enum, without a typedef. Microsoft's
[MIDL naming explanation](https://devblogs.microsoft.com/oldnewthing/20080327-00/?p=22973)
describes why its generated tag cannot supply an authored type name. The `midl_enums` fixture checks
native identity, retained typedefs, changed generated ordinals, and exact RDL across all targets.
The full SDK publishes 497 baseline constants from 40 such helper enums.

Numeric pointer constants, including `INVALID_P_ROOT_SECURITY_ID`, use a separate native
static-initialization query because integer-to-pointer casts are not C++ constant expressions.
Owned values retain the address bits sign-extended from the target pointer width. Capture rejects
relocations, side effects, undefined behavior, and function pointers; publication also rejects
COM interface references rather than treating them as raw addresses. Written pointer typedefs
and pointee constness survive RDL/WinMD round trips. The `pointer_constants` fixture checks null,
sentinel, high-bit, and target-width differences. All 330 baseline urlmon names are accounted for.

`clang_Cursor_Evaluate` can fold `(call(), 3)` to `3` despite runtime side effects; the companion's
static-initializer query requires compiler acceptance and evaluation without side effects or
undefined behavior. Compiler-supported field-offset and member-end expressions need not be C++
core constant expressions. The `layout_constants` fixture checks target-dependent results and
rejects runtime reads, calls, division by zero, signed overflow, and null reads. No SDK macro names
or expression-shape exceptions enter the parser. `ConstantValue` retains float bits
for exact comparison, including signed zero. Non-finite values and extended floating-point types
remain unsupported. The `scalar_constants` fixtures check rounding, subnormals, finite extremes,
constexpr evaluation, target-dependent values, conflicts, and bit-preserving metadata round trips.
Enum-valued constants use their native backing type for evaluation while retaining the enum or
written typedef as the published type. When the enum publishes only loose members, constant aliases
use the same checked backing type, without inventing a public enum name. This includes genuine
anonymous enums and reserved MIDL helper tags, not unrelated named enums or adjacent typedefs.
Unsupported enum annotations still reject dependent constants. The `enum_constants` fixture covers
signed and unsigned limits, scoped enum macros, unnamed values, target-dependent values, and
cross-input reconciliation.

Aggregate evaluation reads integer leaves from Clang's `APValue`, including implicit zero-filled
array tails. GUID and property-key recognition checks field names, types, offsets, and total size;
publication requires the canonical `GUID`, `PROPERTYKEY`, or `DEVPROPKEY` type. Property and device
keys retain their distinct metadata types while sharing the GUID-plus-unsigned-ID encoding.
Values are not reconstructed from numeric tokens or host object memory.
GUID constants and attributes share the canonical `0x12345678_9abc_def0_0102_030405060708`
spelling, including property keys and UUID-tag constants.

Constant observations retain written/canonical types, qualifiers, attributes, source location, and
internal or external linkage. External linkage includes language and native symbol spelling.
`ValueKind::Constant.value` is `None` only when the native cursor is a declaration without an
initializer, not when evaluation fails. Declarations and definitions remain separate observations
even within one TU. Mutable and thread-local objects are not accepted as constant declarations.

Resolution checks every observation against a definition when one exists, including all other
definitions. A declaration cannot mask differing values. Type, linkage, and attribute conflicts
remain fatal before partial pruning, including conflicts behind an unpublishable constant type.
Publication uses the checked value with each observation's written type and attributes, requiring
their RDL bodies to agree. A target without a definition is an explicit error; partial mode omits
the name on every target rather than inventing target availability. The `constant_declarations`
fixtures cover scalar, pointer, and forward-declared GUID types, missing/invalid definitions,
linkage and qualifier conflicts, target-specific definitions, and pruning boundaries.

The SDK driver supplements declaration inputs with `INITGUID` definition inputs derived from the
selected header manifest. A lean Windows input defines winnt/winsvc GUIDs. Other inputs enable
definitions after the shared Windows prelude, which supplies prerequisites in declaration mode.
Main, satellite, and device groups avoid repeated unguarded GUID/property-key blocks: devpkey,
ntddstor, and poclass have separate direct includes, while ntddcdrm/ntddtape use their ntddstor
definition dependency. Definition inputs do not repeat macro discovery. All inputs use the same
compiler arguments and participate in native reconciliation before pruning. The driver SDK test
checks the full native graph and representative definitions on all three targets.
The shell definition group explicitly includes shlguid, which shlobj_core omits under `INITGUID`.

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
`MAX_PATH`. Full SAL interpretation, broader macro discovery,
and general C++ declarations remain separate coverage work.

The ignored `sdk_functions` fixture publishes 117 explicit imports across kernel32, user32, ole32,
advapi32, ws2_32, bcrypt, gdi32, dxgi, winhttp, wevtapi, d3d11, d3d12, d2d1, dwrite, and icuuc.
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

`Function::noreturn` retains the compiler's function-type property. Standard `[[noreturn]]` is
separate declaration evidence (`Cxx11NoReturn`); it does not change that function-type property.
Native and public compatibility compare both forms before pruning. The shared callable writer emits
one `DoesNotReturnAttribute` (`#[noreturn]`) when either applies, including declarations carrying both.
Imports, methods, and named callbacks preserve the contract without changing native result types.
An unnamed non-returning pointer needs a compatible named callback; an ordinary callback cannot
supply its missing contract. The `noreturn` fixtures check native ownership, input-order conflicts,
callback inference, partial dependency removal, and both metadata round trips on all targets.
The SDK fixture covers `ExitProcess`, `ExitThread`, `FreeLibraryAndExitThread`, and `RpcRaiseException`.

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

Simple parameter direction and argument-optionality contracts normalize to typed values shared
by reconciliation and publication. SAL2 `_In_`, `_Out_`, and `_Inout_`, their optional forms, and
the SDK's six corresponding SAL1 aliases emit canonical `#[in]`, `#[out]`, and `#[opt]` flags
without duplicate `NativeSal` attributes. The native observations keep the original transport.
These direct forms require no arguments and the expected SAL version. Multiple distinct
contracts retain their evidence; they are not replaced with a permissive union. Callback parameter
contracts are checked before publication pruning, including when distinct contracts would emit
the same flags. By-value COM references retain output annotation evidence because their metadata
direction describes supplying an object rather than replacing the native pointer.

SAL2 `_Reserved_` and SAL1 `__reserved` parameter contracts normalize to `#[reserved]`, using
the existing `ReservedAttribute` zero/null requirement. They do not imply direction or optionality.
The expected version and empty arguments are required; malformed payloads and other annotation
owners retain evidence. Reserved and unannotated parameters remain distinct during reconciliation.
The `reserved_parameters` fixture covers scalar and pointer parameters, independent direction
flags, functions, callbacks, methods, source aliases, duplicate annotations, and metadata round trips.

RPC SAL 1.2 uses the same normalization for seven direct SDK aliases: `__RPC__out`,
`__RPC__inout`, `__RPC__inout_opt`, and the `__RPC__inout_ecount` and
`__RPC__inout_ecount_part` forms with their optional variants. The expected version and argument
shape are required. `__RPC__in` and `__RPC__in_opt` emit `NativeInput(ReadOnly = false)` alongside
input direction and argument optionality. The attribute records valid input on entry; pointer
arguments are non-null unless optional. Direct SAL2 `_In_`/`_In_opt_` and SAL1 `__in`/`__in_opt`
emit `ReadOnly = true` for their additional pointee-read-access contract. False does not require
writable storage, and absence of the attribute does not assert either contract. Native qualifiers,
aliases, handles, and references remain unchanged; no element count is inferred. Fully represented
payloads disappear from RDL but remain in native observations. The RPC and SAL read-access
contracts remain distinct during reconciliation, including composed annotations.
`input_contracts` checks every form and actual SDK macros across callable kinds and metadata round
trips. `sdk_accounts_settings` covers the real `HWND` and `REFIID` arguments.
Output-count forms add postconditions, and `__RPC__out_xcount` and `__RPC__inout_xcount` do not
express their argument as a count.
Those contracts retain raw evidence rather than being equated by their names.
`rpc_annotation_normalization` checks functions, callbacks, methods, unchanged native evidence,
input order, SDK definitions, and metadata round trips. `rpc_annotation_residuals` checks the
non-equivalent forms, unexpected arguments, and incorrect versions.

Argument-free RPC dereferenced-pointer forms lower their outer input/output and optionality flags.
Input/inout forms retain their inner-pointer contracts as evidence. `__RPC__deref_in_opt` and
`__RPC__deref_inout_opt` allow a null inner pointer, not a null argument. Their `deref_opt` forms
allow a null argument. In this SDK, `__RPC__deref_out_opt` is an exact alias of
`__RPC__deref_out` and SAL2 `_Outptr_`: required output storage and a non-null returned pointee.
These RPC aliases, four SAL1 aliases, and eight SAL2 out-pointer forms emit
`NativeReturnedPointerAttribute`, replacing fully represented `NativeSal` payloads.
`MayBeNull` describes the returned pointer on
success; `NullOnFailure` records the additional COM failure guarantee. A non-null successful result
points to one readable element. Neither field specifies an allocator or ownership transfer.
Outer storage optionality remains `#[opt]`. Native parameter types must supply pointer-to-pointer
storage, including native COM double pointers. Different result/failure contracts still fail
reconciliation; equivalent aliases share the same normalized contract. Native observations retain
the source annotation. SAL1 `__deref_out_opt` permits a null result, unlike the RPC alias.
SAL1 `__deref_opt_out` instead permits null argument storage, and `__deref_opt_out_opt` permits both.
`pointer_parameter_flags` checks all callable kinds and SDK definitions; `returned_pointer_contracts`
checks alias agreement, malformed contracts, and storage rejection. `sdk_background_copy_error`
checks the real BITS methods through RDL and metadata round trips. `sdk_rometadata` checks
`MetaDataGetDispenser`'s SAL1 output contract without inventing a COM IID-output marker.
The four supported `_COM_Outptr_` forms also retain explicit COM intent in the normalized contract.
On native unqualified `void**` storage, including typedef aliases, they emit the existing
`#[iid_is]` marker. Ordinary output pointers, typed interface pointers, const pointees, and deeper
pointer chains do not acquire that marker. Null-on-failure alone is not used to infer COM identity.
`com_output` checks these boundaries across callable kinds and actual SDK macros.

Leading parameter comments retain their text and location in `Parameter.comments`. Capture uses
compiler-owned parameter spans and the innermost matching signature parentheses; it does not find
parameters by spelling or borrow comments from a preceding parameter or nested callback. Overlapping
macro-expanded parameter spans do not establish ownership. Only bracket-list comments supply MIDL
result markers: `[retval]` and output-only `[iid_is]` emit existing canonical attributes with their
direction. SAL may also supply the direction. Result storage must be a pointer, and an output
`iid_is` requires pointer-to-pointer storage. Explicit MIDL output IID markers remain even on
typed storage; this differs from inferring COM intent from an output-pointer macro. No GUID-parameter
association is guessed. Other comment annotations remain native source evidence, not implemented
MIDL contracts. Input and inout IID relationships are not COM output markers; their native comments
remain available without an inferred output query. MIDL-only callback contracts prevent structural
pointers from acquiring annotated callback names and require a metadata owner.
`midl_comments` checks ownership, prose and macro negatives, input order, conflicts, storage
rejection, and metadata round trips. The accounts-settings and audio-client SDK fixtures
cover comment-only IID and retval markers. `sdk_midl_inputs` covers input IID relationships on bind
status callbacks, text-services sinks, and command streams.

The 24 SAL2 returned-buffer forms also lower outer output direction and argument optionality.
Their buffer extent and result nullability remain evidence: an extent of a returned allocation is
not caller-supplied capacity. `returned_buffer_flags` and its SDK case check the full family;
`sdk_bcrypt` covers the optional returned buffer in `BCryptEnumContexts`.
Selected breadth goldens can be regenerated with `UPDATE_CLANG2_RDL=1`; rerun the same selection
without it to check the saved expectations.

The four direct RPC string contracts lower direction, argument optionality, and termination.
`__RPC__in_string` and `__RPC__in_opt_string` replace raw evidence with
`NativeInput(ReadOnly = false)` and termination on entry. SAL1/SAL2 input strings instead carry
`ReadOnly = true`. The inout forms retain evidence: `__RPC__inout_string` also promises termination
on success, but the SDK's
`__RPC__inout_opt_string` promises it only on entry. Termination phases are independent of direction
and do not imply a capacity or character encoding. Version and empty-argument checks prevent
malformed contracts from supplying flags. `rpc_strings` checks all callable kinds, native conflicts,
invalid buffer types, input order, SDK definitions, and both metadata round trips. These contracts
remain distinct from SAL2 readonly input contracts during reconciliation.

Top-level RPC buffer forms lower representable element capacities and successful initialized
lengths. Fully represented `__RPC__out_ecount_part` contracts replace raw evidence with typed
attributes. Their array attribute includes `PostWritable = true`, preserving the SDK's additional
successful-output writable-capacity guarantee. RDL uses the full `NativeArrayInfo` spelling because
`len_param` alone cannot express this property. Direct/full RPC input buffers and their optional
and counted-string variants also replace fully represented evidence with typed input, count, and
termination contracts. Their `NativeInput(ReadOnly = false)` remains distinct from SAL1/SAL2 input
buffers' read-access guarantee. Equivalent RPC `ecount`/`xcount` spellings share one contract.
Other RPC buffer forms retain their full evidence. Full output/inout buffers use the capacity
parameter as initialized length; partial output buffers use a separate integer count or one dereference.
Partial input readable lengths remain entry-condition evidence, not output lengths. Capacity and
initialized length can be represented independently when the other expression is unsupported.
Counted string forms add termination for the phases declared by the SDK.
Input `xcount` forms use the SDK's element-count contract. Output/inout `xcount` forms discard their
count arguments in the SDK definition, so they lower direction and termination without inventing
a capacity. The RPC buffer fixtures check the actual SDK definitions, all callable kinds, both
metadata round trips, invalid count/buffer types, residual expressions, and pre-pruning conflicts.

Other recognized parameter contracts also emit direction and argument-optionality flags.
Returned-object nullability does not imply an optional argument pointer. A direct named integer
capacity normalizes to a parameter index and byte/element units, emitting `#[size_param(N)]` or
`#[len_param(N)]` without duplicate `NativeSal` evidence. This covers the twelve SAL2
reads/writes/updates forms and ten direct SAL1 aliases established by the SDK definitions.
The legacy optional inout-count forms remain residual evidence rather than assumed equivalents.
Indices are zero-based within the declared parameters, excluding the implicit COM receiver.
The shared parameter path handles functions, callbacks, and methods. Conflicting count parameters or
units, non-pointer buffers, and non-integer count parameters are projection errors, not guessed
relationships.

Reconciliation compares count references by parameter position. Renamed imported-function
parameters can agree when their complete published contracts match.
Capacity and initialized length remain separate. The eight SAL2 `*_to_*` output/update forms
and four direct, non-optional SAL1 aliases encode initialized length in
`NativeInitializedLengthAttribute`. Its zero-based `CountParamIndex` identifies a by-value integer
parameter or, with `Dereference = true`, one pointer to an integer. `IsByteCount` distinguishes bytes
from elements. This is a postcondition subject to the callable's success contract; it does not
authorize dereferencing a null optional count pointer. Count parameter optionality remains separate.
The bounded `p ? *p : n` form adds `NullFallbackParamIndex` for the by-value integer parameter `n`.
The condition must name the same parameter that is dereferenced. A missing field means no fallback,
not parameter zero. Pointer/count types are validated; arbitrary conditions, arithmetic, constants,
and nested ternaries remain evidence rather than guessed relationships. This shared lowering applies
to supported SAL1, SAL2, and RPC initialized-length forms, without changing native observations.
`conditional_length` checks metadata round trips, whitespace, input order, conflicting fallbacks,
invalid types, and unsupported expressions. Its SDK case verifies actual RPC and SAL2 macros;
`sdk_background_copy_enumerator` covers `IEnumBackgroundCopyFiles::Next`.
Consumers using initialized-length guarantees must honor the fallback field before dereferencing;
same-version round trips do not establish compatibility with consumers that ignore it.
The eight SAL2 `*_all_*` output/update forms use the same direct integer parameter for capacity
and initialized length. Unsupported count expressions retain their source evidence and known
direction without an inferred relationship. `buffer_full` checks equivalence with matching
`*_to_*` capacity/length contracts, and its SDK case checks the real macro definitions.
Unrepresented length expressions retain the full SAL payload alongside any known direct capacity.

The six SAL2 input/output/update zero-termination forms and three direct SAL1 aliases emit
`NativeNullTerminatedAttribute`. `OnEntry` and `OnSuccess` distinguish the precondition from the
successful-output postcondition. They derive from each written contract, not the union of parameter
direction flags. Termination specifies no character encoding. Output/update termination lowers only
when its capacity is representable; other forms retain raw evidence.

Constant, indirect, compound, and conditional capacities retain their raw contracts without a
size/count attribute. No arithmetic is evaluated from annotation text. `buffer_sizes.h` and its
exact RDL fixture cover units, typedefs, parameter
ordering, macro expansion, and callback ownership. The SDK annotation case checks ReadFile's
`MemorySizeAttribute.BytesParamIndex = 2` and GetModuleFileNameW's
`NativeArrayInfoAttribute.CountParamIndex = 2` on all three targets, without generating Rust.

Name-corresponding typedef observations may differ in SAL coverage. Publication selects an observed
annotation set only if it contains every other observed set. Incomparable sets fail, even in partial
mode; publication does not invent their union. Callable parameter observations compare normalized
simple contracts and exact residual evidence. Other native annotations still require matching
evidence. Original observations are retained unchanged.

SAL is emitted on supported functions, methods, parameters, constants, records, fields, aliases,
callbacks, and named enums. Callback prototype parameters belong to `AliasObservation`, not the
interned function type. Their names, written types, and annotations use the same publication path as
ordinary function parameters. Written callback aliases retain their contracts; ABI-only callback
naming excludes annotated and indirect prototypes so unrelated inline pointers cannot inherit them.

Bare function typedefs publish named callbacks without changing their native function types.
A pointer to a bare function typedef, or its adjustment in a parameter, supplies the callback
value; each additional pointer level remains explicit. Authored alias names, parameter contracts,
and exception specifications survive alias chains, arrays, and nested pointer uses. Bare typedefs
do not supply ABI-only names for unrelated inline pointers. `function_typedefs` checks these
cases and native function-versus-pointer conflicts across targets and input orders.

Compiler-provided global typedefs referenced by captured types enter the native observation
catalogue even when Clang's translation-unit visitor omits them. Native identity, semantic parent,
and the companion's implicit-declaration query establish their origin; empty source locations do
not. The implicit canonical declaration precedes any authored redeclarations, retaining its
underlying type rather than leaving a self-referencing alias without its original definition.
Implicit observations participate in compatibility checks but supply neither public type names nor
header ownership. Authored aliases retain their names and can use the captured underlying type.
An authored redeclaration has its own publication evidence. `implicit_aliases` covers these cases,
input order, conflicts, rejected nested aliases, and metadata round trips. This query requires
companion ABI 16.

When a typedef supplies a record or enum's public name, its annotations carry `TypedefName` in
`NativeSalAttribute`. This distinguishes them from annotations on the underlying tag. A callback
alias chain likewise retains inherited typedef annotations with their original typedef names.
Annotated inline/nested callback parameters, bitfields, enum members, anonymous enums without a
public owner, and expanded typedefs remain unsupported. Partial publication removes their dependent
declarations rather than dropping annotations. Unknown Clang annotations and malformed transport
are explicit errors. Compound initialized lengths, indirect/compound capacities, pointer-difference
termination forms, and conditional contracts still require semantic lowering. Their retained strings
are not proof of metadata parity.
`annotation_normalization` checks real SDK SAL1/SAL2 equivalence, native evidence retention,
functions/callbacks/methods, input order, conflicts, and both metadata round trips.
`annotation_residuals` checks conditions, termination, returned-object nullability, mixed contracts,
non-parameter ownership, and COM pointer direction.
`buffer_normalization` checks every supported count form, SDK aliases, byte/element units,
parameter-reference conflicts, renamed parameters, invalid count types, and metadata round trips.
`buffer_postconditions` covers entry/success termination, separate capacity and initialized length,
direct and single-dereference counts, real SDK aliases, all callable kinds, and both metadata round
trips. `buffer_postcondition_residuals` covers unsupported expressions, mixed contracts, optional
count pointers, and retained success predicates. These attributes use the existing typed metadata
encoding, without new RDL shorthand or a general annotation-expression language.

Across the 370-header workload, termination and initialized-length lowering replaces 976
`NativeSal` payloads with 410 termination and 566 initialized-length attributes. All 1,665 changed
parameter sets across the three targets have matching
source-derived contracts in both source RDL and canonical WinMD exports. Declaration inventory,
types, values, non-annotation content, omissions, and constant accounting are unchanged.
The direct SAL1 capacity aliases also supply standard contracts:
`D3DReflectLibrary.pSrcData` uses byte-count parameter 1, and
`SetClusterGroupNodeListEx.NodeList` is an input array with element-count parameter 1.
Repeated generation produces byte-identical partitions, omissions, capture configuration,
constant accounting, and canonical WinMD exports. Full runs take about 257-276 seconds with
6.87-7.47 GB observed peak working set on the development machine; these are workload observations,
not quiet-machine benchmarks. These results do not establish parity for the remaining annotation
forms or downstream Rust consumers.

RPC normalization replaces another 3,858 payloads, leaving 27,486 residual payloads. Source and
canonical comparisons account for 24,636 and 18,207 changed parameter sets across the three
targets. Canonical changes include 1,872 output-to-input corrections and 5,280 optional-argument
corrections; input/output aliases also supply missing output and inout directions. Declaration
inventory, non-annotation content, omission identities, and constant accounting are unchanged.
Eight rendered prototypes in existing omission messages reflect the new flags, not new failures.
Repeated output, canonical export, omissions, and configuration are byte-identical. Final full runs
take about 257-273 seconds with 7.32-7.51 GB observed peak working set, without quiet-machine controls.

Reserved-parameter normalization replaces 979 raw payloads with existing `ReservedAttribute`
contracts, leaving 26,507 raw payloads. All 1,500 changed parameter instances across three targets
agree in source and canonical exports; directions, optionality, types, declaration inventory,
and all other attributes are unchanged. Omissions, constant accounting, and capture configuration
are byte-identical. Two full runs take 271.6 and 257.6 seconds with peak working sets of 7.05 and
6.91 GiB. All partitions and canonical output are byte-identical across runs. These are workload
observations, not quiet-machine benchmarks.

Pointer-parameter lowering changes 9,330 source parameter instances across the three targets.
Canonical changes affect 183 instances: 48 output-to-input, 129 output-to-inout, three input-to-output,
and 96 optional-argument corrections, with overlap between direction and optionality changes.
All 26,507 raw payloads remain intact. Declaration inventory, types, other attributes, omissions,
constant accounting, and configuration are unchanged. Two full runs take 261.8 and 263.5 seconds
with observed peak working sets of 6.79 and 7.12 GiB. All 788 partitions and canonical exports are
byte-identical across runs. These are workload observations, not quiet-machine benchmarks.

Direct RPC string lowering changes 1,068 parameter instances across three targets in 29 partitions.
Source and canonical comparisons verify entry termination on every affected parameter, six
output-to-input corrections, and 240 optional-argument corrections. Declaration inventory, types,
raw annotation evidence, other attributes, omission diagnostics, and constant accounting are
unchanged. Two full runs take 248.3 and 263.2 seconds with observed peak working sets of 7.14 and
6.81 GiB. All partitions, canonical exports, capture configuration, omissions, and constant
accounting are byte-identical across runs. These are workload observations, not quiet-machine
benchmarks.

RPC buffer lowering changes 1,437 source and 1,407 canonical parameter instances across three
targets in 60 partitions. Canonical changes include 534 output-to-input, 195 output-to-inout,
42 input-to-output, three input-to-inout, and 183 optionality corrections. All 26,507 raw payloads
remain intact. Inventory, types, unrelated attributes, omission identities, and constant accounting
are unchanged. Four rendered HSTRING prototype lines in existing omissions show the corrected
directions. Two full runs take 227.9 and 236.3 seconds with observed peak working sets of 6.69 and
7.03 GiB. All partitions, canonical exports, configuration, arguments, omissions, and accounting
are byte-identical across runs. These are workload observations, not quiet-machine benchmarks.

The resumed annotation batches pass affected-crate formatting, warnings-denied all-target clippy,
all 54 breadth tests including SDK cases, and a no-default-features library check. The RPC string
regression fails before lowering is added. Review confirms that reconciliation still compares
residual evidence, optional inout strings gain no output-termination promise, and invalid buffers
remain explicit omissions. Remaining annotation forms and downstream consumers are separate gates.

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
cargo run -p tool-win32-2 --quiet -- --partial --sdk-imports --sdk-macros
cargo run -p tool-win32-2 --quiet -- --partial --sdk-imports --sdk-macros --x86-stdcall
cargo run -p tool-win32-2 --quiet -- --partial --sdk-imports --sdk-macros --dry-run
cargo run -p tool-win32-2 --quiet -- --compare-constants
cargo run -p tool-win32-2 --release --quiet -- --partial --first=357
```

`tool-win32-2` is the edit/rerun loop: parse SDK headers on all three targets, merge, project,
compile, then replace the tracked `metadata/win32` RDL partitions in both strict and partial modes.
Leave partial changes, including deletions, visible in VS Code's Git diff for review. This does not
mark the partial output as accepted production metadata. It shares `tool-win32`'s `src/headers.rs`
manifest and defaults to all headers. Main and satellite TUs follow the SDK's include rules.
`--first=N` selects a manifest prefix; positional arguments select headers, not types.
`--import=Function=library.dll` supplies function ownership; `--macro=NAME` selects a constant macro
in each input. `--macro-header=header.h` discovers object-like macros in that header. These options
can be repeated. Unconfigured function imports remain explicit omissions.
`--sdk-macros` selects header filenames recursively from the pinned SDK's `um` and `shared`
directories, including transitive owners such as `winbase.h` and `nserror.h`, and includes every
selected input header's filename. This covers selected `winrt` headers without selecting that
entire directory. It does not add includes: a header must still be reached by the selected input
TUs. CRT and compiler resource directories are not scanned. Selection uses the active definition's
filename, as with `--macro-header`, and does not claim coverage of headers absent from the input.
Adding the 15 selected WinRT owners to the six-TU, 370-header workload recovers three `roerrorapi`
constants on all targets. Existing declarations, types, values, owners, and availability are
unchanged. Candidate/repeat output is byte-identical across 788 partitions, canonical metadata,
omissions, configuration, and accounting. Observed wall times are 249.5/241.9 seconds, with peak
working sets of 7.32/7.31 GB. These observations are not quiet-machine benchmarks.

Each invocation writes `target/win32-2/capture-configuration.txt` with the SDK/compiler pins,
target set, input sources, arguments, macro selections, and resolved imports. `--dry-run` writes
this configuration without parsing headers or changing RDL/WinMD. The file describes the attempted
configuration, not proof that generation completed.

Set `WINDOWS_CLANG_PROFILE=1` to emit tab-separated `clang-profile` records on stderr. Each record
contains the target/input label, phase, operation, count, and elapsed seconds. Native parse,
diagnostic formatting, and TU disposal are timed separately; candidate, empty-definition,
confirmation, and rejection counts identify repeated work. Phase wall times are inclusive, and
target and confirmation workers overlap. Do not add these times together as process elapsed time.

The three-TU, 370-header, 1,800-owner profile performs 29,939 isolated confirmations, versus 36
batch parses.
In the serial-confirmation control, confirmations account for over 90% of capture-worker time.
Native reconciliation and diagnostic formatting are small by comparison. SDK PCH replays show no
material gain from a lean PCH, disabled validation, local-only indexing, reparsing, preamble reuse, or
disabling libclang's helper threads. These experimental settings are not enabled in capture.
Direct `-cc1` replay fails with `CXError_ASTReadError` and supplies no valid speed comparison.
On a 24-logical-CPU, 64 GiB Windows machine, bounded parallel confirmation reduces the recorded
full workload from 692.6 seconds to 220.0 and 223.3 seconds in repeated runs, including projection
and WinMD compilation. Observed peak working set increases from 3.85 to about 6.9 GiB. All 780 RDL
partitions and omissions are byte-identical, including with profiling disabled. The input scope,
independent rejection checks, and confirmation count are unchanged. Precise timings require a
quiet machine.

The six-TU definition control uses the same 370-header manifest, 1,800 macro-owner filenames,
compiler/SDK pins, and three targets with `--partial --sdk-imports --sdk-macros`. Its
candidate contains 93,038 constants in 788 partitions:

| Constant-name accounting | Count |
| --- | ---: |
| Frozen baseline | 82,854 |
| Baseline names present as loose constants | 82,546 (99.628%) |
| Baseline names absent as loose constants | 308 |
| Additional names | 10,492 |

Definition capture, device keys, UUID tags, and anonymous-enum constant aliases add 5,443 constants
to the 87,595-constant control, including 4,009 baseline names. Canonical comparisons find no retained
declarations lost or changed and no new omission identities. Record, enum, alias, function, and
interface counts are unchanged. UUID capture changes 43 diagnostic texts for already-omitted class
definitions and their dependents. The final anonymous-enum alias step removes two omissions and
recovers both baseline declarations with matching types and values.
The 93,036-constant UUID control produces byte-identical RDL partitions and omissions in two runs,
taking 264.5 and 248.2 seconds with peak working sets of 6.73 and 6.93 GiB. The 93,038-constant
candidate takes 278.0 and 255.1 seconds with peak working sets of 6.88 and 7.01 GiB. Its RDL,
omissions, and capture configuration are byte-identical across those runs. These are workload
observations, not another controlled optimization comparison.

Decoded DLL-export attributes add 1,037 ICU functions across all three targets, raising the function
count from 11,363 to 12,400 without changing any retained declarations or the constant accounting.
All added imports are present in the frozen baseline. Ten already-omitted declarations receive
more specific diagnostics: nine depend on the unsupported ARM64 `va_list` alias, directly or through
trace callbacks, and one lacks configured DLL ownership. No new omission identities appear.
The resulting 74,400 omissions remain explicit. Two full runs take 270.8 and 274.7 seconds with
peak working sets of 6.79 and 6.82 GiB. All 787 RDL partitions, omissions, capture configuration,
constant accounting, and canonical WinMD exports are byte-identical.

The DLL-export control's ICU signature comparison finds 432 exact matches on all targets, 580 names
with checked primitive typedef differences, and 25 other names. Twelve of those use inline pointers
for bare function typedefs; 17 have different outer pointer constness, with four in both groups.
The pointer forms preserve native declarations such as `const char **source` as
`*mut *const i8`, not `*const *const i8`.

Bare function typedef publication supplies those twelve ICU callback names and the authored RPC
callback names. Across the full workload it adds 362 callbacks, 20 dependent records, 20 pointer
aliases, and 23 baseline imports, removing 447 omissions without adding any. The candidate has
10,213 records, 13,199 aliases including callbacks, 12,423 functions, and 73,953 omissions.
Constants, enums, interfaces, and partition counts are unchanged. Fifteen retained functions and
three records change only callback type names: expanded source and canonical signatures agree on
every target, including pointer depth and calling convention.

The 23 imports agree with baseline directions, optionality, buffer relationships, DLL ownership,
and ABI types after checked typedef expansion. On ARM64, `RtlInstallFunctionTableCallback` uses
native `ULONG_PTR` (`u64`) instead of baseline `usize` for two parameters. Their ABI widths agree,
but their metadata/Rust type identities differ; this remains a primitive-typedef consumer question.
Additional SAL evidence remains explicit. These comparisons do not grant blanket baseline-parity
approval or resolve the remaining representation and consumer gates.

Implicit global alias capture adds another 16 baseline imports, including `FormatMessageA/W`,
ICU formatting/tracing APIs, and fixed-signature `va_list` formatting functions. It also publishes
the authored `va_list`, `__gnuc_va_list`, `UTraceData`, and `UTraceExit` declarations.
The compiler's implicit `__builtin_va_list` remains native evidence, not a public metadata name.
Its underlying type is compiler-reported `char*` on all three Windows targets.
The additional partition belongs to the authored standard-header `__gnuc_va_list` declaration.
No retained source or canonical declaration changes, and no new omission identities appear.
Twenty omissions disappear; eighteen existing CRT omissions now identify the unresolved
`__crt_locale_pointers` dependency instead of `va_list`.

The candidate contains 13,203 aliases including callbacks, 12,439 functions, and 73,933 omissions.
Other declaration counts are unchanged. Newly published import contracts agree with the baseline
after checked typedef/default-direction expansion, except the source-verified outer-pointer
correction in `utrace_getFunctions`: `const void **context` is `*mut *const void`.
Conditional buffer contracts on `FormatMessageA/W` and other residual annotations remain evidence,
not implemented semantics. ABI-16 runs take 271.6 and 259.4 seconds with peak working sets of
6.88 and 6.78 GiB. All 788 partitions, omissions, capture configuration, constant accounting,
and canonical WinMD exports are byte-identical. These timings are workload observations.

The approved native-enum policy accounts for 239 baseline names as members of 32 named enums rather
than loose constants, including five reviewed signed-interpretation differences with matching
32-bit patterns. The approved UUID-tag policy recovers 713 baseline names without inferring values
for separately declared `CLSID_*` or `IID_*` objects. `MediaEventType` is a separate `DWORD` typedef,
not a native alias of its neighboring MIDL enum. Do not infer that association from adjacency.

Olectl's 27 baseline GUIDs
sit inside `#if !defined(INITGUID)`, so ordinary definition mode excludes their source declarations.
No header guards or declaration macros are overridden to manufacture those definitions.
The driver adds an Olectl definition input when `dshow.h` or `olectl.h` is selected. The manifest
reaches Olectl through DirectShow. This input preloads `ole2.h` and `ocidl.h`, enables the SDK's
`DEFINE_GUID` definition macro through `guiddef.h`, then undefines `INITGUID` before the root
include. Without dependency preloading, nested includes restore the declaration-only macro.
The `sdk_guid_context` fixture checks three GUID values through RDL/WinMD on all targets, ordinary
declaration-only rejection, and declaration/definition reconciliation in either input order.
The driver test checks the real manifest inputs. Macro discovery is not repeated in this input.
This definition configuration is separate from the deferred PCH issue.
SDK-specific definition include configurations are provisional. Review these workarounds together
before replacement acceptance, including dependency preloading and split `DEFINE_GUID`/`INITGUID`
state. They belong in the SDK driver, not parser or per-symbol policy.
The seven-TU candidate and repeat recover all 27 GUIDs with exact baseline types, values,
attributes, owners, and target availability. Only `olectl.rdl` changes; all existing declarations
and omission reasons are unchanged. Exactly 27 omission identities disappear. Accounting reports
82,593 ordinary matches, 239 enum-member matches, and 22 missing baseline names.
All 788 partitions, canonical exports, configuration, arguments, omissions, and accounting repeat
byte-for-byte. Observed wall times are 238.7/255.2 seconds, with peak working sets of 7.55/7.51 GB.
A full metadata round trip preserves 36,248 root/nested type shapes.
Among present loose constants, canonical comparison records 1,900 type-classified names and no
attribute differences. `CLSCTX_VALID_MASK` has the same 32-bit pattern as the baseline's
`2682254367`, but its native enum value is signed `-1612712929`. The `sdk_constants` fixture checks
its negativity and unsigned conversion with compiler `static_assert`s on all targets, then checks
the enum type and signed value through both metadata round trips. This is signed interpretation,
not different mask bits. Name presence, unchanged preceding output, and repeatability do not
resolve all baseline semantic differences or satisfy replacement acceptance.

Byte-aligned qualifier support recovers 17 records, 44 aliases (including two callbacks), 35 imports,
and 66 interfaces across ten partitions. This includes shell item-list APIs and packed WinNT
relocation pointers. All existing source/canonical declarations, owners, availability, and semantics
are unchanged. Constant accounting, input configuration, and arguments are byte-identical.
The batch removes 240 omission identities. Of the changed reasons on remaining omissions, 112
expose existing import-ownership gaps and one exposes the existing `IExpDispSupportXP` dependency.
The full candidate/repeat has 73,380 diagnostics and byte-identical outputs; metadata round trips
preserve 36,377 root/nested type shapes. Observed wall times are 247.4/246.7 seconds, with peak
working sets of 7.32/7.29 GB. These are workload observations, not quiet-machine benchmarks.

Non-returning callable support recovers the four SDK exit/exception imports across three partitions.
The twelve target signatures preserve baseline owners, libraries, parameters, results, and
non-returning contracts, while retaining native `UINT`/`DWORD` aliases, input flags, and the native
64-bit C calling convention. Existing declarations and constant accounting are unchanged.
Four omission identities disappear; remaining changed reasons expose five import-ownership gaps,
five undecoded `Builtin` attributes, and `longjmp`'s computed exception specification.
The candidate/repeat has 73,376 diagnostics and byte-identical output. Observed wall times are
248.4/239.6 seconds, with peak working sets of 7.27/7.52 GB. Metadata round trips preserve all
36,377 root/nested type shapes.

`--sdk-imports` loads each target's pinned SDK import libraries using the shared ordered policy in
`crates/tools/win32/src/imports.rs`. Host libraries precede API-set umbrellas, and the enclave runtime
comes last. The same checked DLL corrections apply in both generators; an unexpected SDK mapping
fails generation. Explicit `--import` entries take precedence over discovered entries.
`projection::Imports` separates public-name overrides in `functions` from exact native linker symbols
in the per-target `symbols` maps. Native lookup uses compiler mangling, with no public-name fallback.
The checked missing SDK entry for `InitializeXamlDiagnosticsEx` is an explicit public mapping.

The COFF reader retains the machine and code/data/const kind alongside linker symbols and named or
ordinal PE exports. Machine filtering precedes first-library-wins selection: ARM64 archives also
contain ARM64EC members, which cannot supply native ARM64 linkage. Non-code records remain excluded
without falling through to another provider. The pinned x86 SDK lacks `winhvplatform.lib` and
`vertdll.lib`; these two checked availability cases are recorded, while other missing libraries and
malformed archives are fatal. `target/win32-2/import-omissions.txt` records these exclusions.
Header capture and native reconciliation do not infer DLL ownership.

SDK publication prefers named exports. `tool-win32-2/src/named_exports.rs` contains checked public
entry points whose SDK archives select ordinals: ten Winsock, two Direct3D 12, six Direct2D, and
`ILClone`/`ILFree`, plus 22 Active Directory helpers from `adshlp.h`. These are established named
bindings, also verified against the installed
x86/x64 DLL export tables. The policy checks exact native symbols, target, provider, ordinal, and
code kind; it does not derive export names from linker decoration. Changed evidence fails.
Unreviewed ordinal imports remain explicit omissions. Original ordinal records remain intact,
and the configuration records each reviewed named-publication decision.

Generic projection can encode `#[library("DLL", import = "#N")]`; this is distinct from SDK
publication policy. `import_targets` checks target mappings, explicit precedence, missing-target
rejection, invalid ordinals, and native conflicts. `sdk_import_targets` preserves native archive
evidence through both metadata round trips. The driver's `sdk_library_policy` checks every reviewed
entry on all targets and compares actual named publication to `sdk_named_imports.rdl` and
`sdk_adshlp.rdl`. The Active Directory fixture covers all 22 helpers with their native dependencies.
`named_import_consumers` consumes that fixture with the unchanged bindgen/windows-link path.
It checks API presence, native signatures, host execution, and three-target named PE imports.
The PE probe excludes Rust std, whose own import libraries may select Winsock ordinals regardless
of the generated bindings. A separate host consumer retains std. This does not establish export
availability on every supported Windows release or package-wide acceptance.

The first per-target import candidate adds 1,462 functions and removes 182, for 13,766 functions
across 789 partitions. Existing retained signatures, types, constants, owners, and availability are
unchanged. Eighteen named-export exceptions become native ordinals on all targets.
`RtlCompareMemory` uses ntdll on x86; `RtlRaiseException` uses ntdll on x86/ARM64, following the
ordered native library records. Exact compiler linker symbols verify all 4,439 added or changed
target declarations. Of 4,382 added target declarations, 4,380 occur in the frozen baseline.

The 182 losses remain unresolved, not approved baseline exclusions. Each has a missing x86 mapping,
so partial publication removes it on every target. For 94 functions, including DHCP and DNS APIs,
the current compiler default produces cdecl symbols while the SDK libraries contain stdcall symbols.
An isolated x86 compiler-default probe matches all 94; that configuration has not been adopted.
The other 88 have no matching x86 entry or named export in the selected libraries, including
NDR64 marshalling and UMS functions. Missing import evidence does not establish runtime absence.
`sdk_import_gaps` preserves both failures without guessed exports, calling conventions, or target
availability. These losses require resolution before accepting the batch.

`--x86-stdcall` opts into an SDK compiler configuration under review. It passes
`-Xarch_i686 -mrtd` to Clang, matching the exact x86 target triple; x64/ARM64 keep their native
defaults. Explicit cdecl/stdcall declarations retain their requested convention. Unannotated free
functions and callbacks change together, as checked by the `calling_defaults` header/RDL fixtures.
The parser and native IR have no SDK-specific convention rewrite. The option is not enabled by
default, and full metadata review must account for callback changes as well as recovered imports.

The stdcall trial recovers all 94 functions on all three targets, with every import matched to the
compiler linker symbol and target library record. It loses no declarations relative to the
target-native import candidate. It also changes 86 existing x86 declarations: 84 named callbacks,
`InitializeXamlDiagnosticsEx`, and the six function-pointer fields in `UReplaceableCallbacks`.
Seventy-five named declarations match the frozen baseline's cdecl ABI before this change; ten
callback aliases have no baseline declaration. The baseline uses opaque pointers for the ICU
fields, so it supplies no calling-convention evidence for them. Import-library agreement for the
94 functions does not validate these callback and explicitly mapped export changes. The blanket
default is not accepted for canonical publication.

All other source/canonical declarations, types, layouts, constants, owners, target availability,
and imports are unchanged. The trial removes 94 import omissions without adding declaration
omissions. Thirteen existing compiler-warning type spellings gain stdcall; an unused-argument
warning records the inactive architecture selector on other targets. The candidate has 13,860
functions and 72,002 omission diagnostics. Metadata round trips preserve 36,462 root/nested type
shapes and stable canonical text. Observed wall time is 270.4 seconds, with a 7.50 GB peak working
set. Repeat generation is deferred while the ABI changes remain unresolved.

Without the stdcall option, the target-native import candidate removes 1,462 omission identities
and adds exactly those 182 import gaps, leaving 72,096 diagnostics. Input sources, arguments, and
constant accounting remain unchanged.
Observed wall time is 242.6 seconds, with a 7.21 GB peak working set. Repeat generation and final
acceptance remain pending; this result is not replacement readiness.

Strict mode fails on unsupported declarations. `--partial` emits the supported subset and
writes `target/win32-2/omissions.txt`; compiler errors and captured-definition conflicts remain
fatal. Both modes compile before replacing generated RDL; the directory's readme is preserved.
The partial WinMD stays under `target/win32-2`. The metadata diff is a review baseline, not a
correctness oracle; fixtures check native layouts and the RDL/WinMD round trip independently.
The omission report covers AST declarations and selected discovery candidates, not unselected
headers' macro constants.

`--compare-constants` reads the working-tree `metadata/win32` RDL and compares it with
commit `6e68e56081340eaa932f4dbc92ec54c56ed51473`. It runs without loading libclang and writes a
deterministic `target/win32-2/constant-coverage.csv`: case-sensitive names, presence status, and
baseline/current owner partitions. Architecture duplicates count once; moved ownership is not
absence. `enum-member` distinguishes a baseline name found in an enum from an ordinary constant
declaration. Enum names come from the RDL parser, including scoped members. Neither status means
equivalent type, scope, value, or architecture. New constants do not offset missing names.
A capture or compilation failure leaves the preceding RDL unchanged; run comparison only after
generation succeeds.

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
`NativeBitfieldAttribute` has an optional `MemberType: System.Type` property. Typed members preserve
signedness, bool and enum identity, written typedefs, and single-bit integer semantics independently
of storage. Compatible signed/unsigned, enum, and bool members may share storage when Clang reports
matching allocation layouts. Unnamed padding remains native evidence without a public member.
RDL retains typed attributes explicitly, and metadata remapping updates their nominal references.
Untyped metadata keeps its existing interpretation and generation. Updated readers are required for
typed metadata.

The `typed_bitfields` fixture checks mixed storage, single-bit signed and unsigned values, bool
fields, enums and aliases, full-width signed values, unions, dependencies, and native conflicts in
both input orders. Raw layout checks and typed accessor compilation run on all three MSVC targets;
host execution checks sign extension, truncation, and neighboring bits. Typed union accessors read
storage inside `unsafe`; legacy untyped union accessors remain a separate consumer defect.
Some MSVC bitfield-only
unions have alignment 1 despite wider integer storage. Those layouts remain rejected until their
storage alignment can be encoded without inventing a native packing request.

The full typed-bitfield batch changes 54 partitions and adds 29 records, 11 aliases (including six
callbacks), eight imports, and seven interfaces. Source and canonical comparisons cover 1,544
changed existing declarations and 8,393 typed members across three targets, including 5,389
single-bit integer members. Existing declarations, storage types, offsets, widths, unrelated
attributes, and constant accounting are unchanged. The batch removes 108 omission identities.
Two existing speech-interface omissions now report their still-omitted base interfaces rather than
the recovered `SPEVENT`/`SPEVENTEX` dependencies. There are no new omission identities.

The candidate has 10,242 records, 4,532 enums, 13,214 aliases, 93,038 constants, 12,447 imports,
3,845 interfaces, and 73,825 omissions. Two full runs take 270.8 and 238.6 seconds with observed
peak working sets of 6.86 and 7.00 GiB. All 788 partitions, canonical exports, configuration,
arguments, omissions, and constant accounting are byte-identical across runs. These are workload
observations, not quiet-machine benchmarks.

Affected crates pass formatting and warnings-denied all-target clippy. Metadata integration
executables, RDL tests, all 153 bindgen golden cases, 14 bindgen error cases, native bitfield
cross-target checks, the 54 breadth tests including SDK cases, and the no-default-features library
check pass. A full SDK metadata round trip preserves 36,138 root/nested type shapes and produces
stable canonical text. Existing bindgen golden outputs and `tool-bindings`/`tool-reactor` generated
outputs are unchanged. A frozen-input `tool-package` control fails its existing assertion against
unrouted types in the flat namespace before code generation, so package-wide neutrality remains
unverified.

SDK header packing affects interface declaration alignment, not the vtable format.
Clang reports pointer-sized layouts with alignment 1 for `IACList`, `IACList2`, and `ICopyHookW`
on all targets. `IColumnsInfo` has alignment 2 on x86 and 8 on the other targets. Requested packing
and actual layouts remain in native reconciliation. Interfaces publish as opaque COM references,
not packed value records. Projection requires pointer-sized, fieldless objects with at most one
interface base at offset zero. Expected alignment comes from the base, or pointer alignment for a
root, capped by the local packing request. Explicit alignment attributes remain unsupported.
`windows-core` loads the object vtable pointer without an object-alignment requirement; the vtable
itself must remain aligned. Weak-reference forwarding uses the same load.
The SDK declarations do not establish that Windows returns unaligned objects. The alignment-safe
core load also supports implementations whose object addresses are actually unaligned.

The `packed_interfaces` fixture checks inherited packing, 1/2/8/16-byte requests, native conflicts
even when layouts match, absolute slots, rejection boundaries, and metadata round trips on all
targets. The core `packed` test exercises an unaligned object through base-vtable access, query,
clone/drop, weak-reference forwarding, and upgrade after the last strong reference is released.
The core case runs on x86 and x64 and compiles for ARM64. All core integration tests, 55 breadth
tests including SDK cases, the expanded SDK interface fixture, and affected warnings-denied clippy
checks pass. The no-default-features clang2 build also passes.

The full interface-packing batch adds 105 interfaces and five aliases across three partitions.
All existing source and canonical declarations, owners, target availability, and semantics remain
unchanged. It removes 190 omission identities with no new omissions or changed diagnostic reasons.
The candidate has 3,950 interfaces, 13,219 aliases, and 73,635 omissions; other inventory counts
remain unchanged. A full metadata round trip preserves 36,248 root/nested type shapes.
Candidate and repeat take 261.9 and 254.8 seconds, with observed peak working sets of 6.93 and
7.08 GiB. All 788 partitions, canonical exports, configuration, arguments, omissions, and constant
accounting are byte-identical between runs. These are workload observations, not quiet-machine
benchmarks.

Enums use separate typed IDs and observation tables, not integer aliases or synthetic records.
Native reconciliation checks backing types, layouts, scopedness, attributes, and ordered members.
It checks forward declarations as well as definitions. Public enum names use their unique defining
value typedef, otherwise their native tag. Aliases cannot equate distinct native enums. Scoped enums
retain `ScopedEnumAttribute`; no flags are inferred from numeric patterns. The `enum_*` fixtures cover
all eight integer widths/signs, 64-bit limits, expressions, duplicate values, empty definitions,
forwards, nominal conflicts, and architecture-specific backing types. Partial pruning uses shared
tag names so a failed enum also excludes a same-named record on another architecture.

The accepted native-enum policy retains 239 baseline constant names as named-enum members.
Their owners and 32-bit value patterns match on all three targets, but the baseline assigns 32
different public typedef names rather than their native enum types. For example, the SDK declares
`enum _BROWSERFRAMEOPTIONS` and a separate `typedef DWORD BROWSERFRAMEOPTIONS`; the latter is not
a typedef of the enum. Five names also change signed interpretation, including `BFO_QUERY_ALL`
and `CDBE_TYPE_ALL` from unsigned maximum to signed `-1`. These type and signed-interpretation
differences are approved corrections, not uncaptured declarations or identical baseline semantics.
Do not infer an enum/typedef relationship by stripping an underscore.
The `native_enum_types` fixture preserves enum members and explicit enum-typed constants separately
from an unrelated unsigned typedef and its explicitly converted constant, through RDL and WinMD
on all three targets. The candidate's generated metadata is unchanged by this decision.
Accounting distinguishes these 239 enum-member matches from ordinary constant matches. A name
match alone does not establish type, value, scope, or architecture parity.

Conflicting unannotated scalar aliases on constants can publish their common native primitive type.
For example, equal `DWORD`/`ULONG` observations use `u32`, and equal `BYTE`/`UCHAR` observations use
`u8`. Ordinary public agreement preserves unique written aliases. Native type and value agreement
still precedes this publication choice; equal-width native `int` and `long` are not interchangeable.
Every alias in the written chain must be unannotated across all resolved observations, including
inputs that declare an annotated alias without using the constant. Constant-owned attributes are
preserved and the complete canonical bodies must agree. Enum, pointer, record, and callable
conflicts do not use this rule.
The `scalar_constant_aliases` header/RDL fixtures cover integer, boolean, and floating values,
selected macros, declarations plus definitions, unique aliases, reversed inputs, and RDL/WinMD
round trips on all targets. Native type, value, signed-zero, and annotation conflicts remain fatal
in partial mode. Local and third-input alias annotations prevent canonical fallback.
The full candidate and repeat recover all 17 serial constants with exact baseline types and
values on x86, x64, and ARM64. Their multiple native header owners route them to `foundation.rdl`
through the existing shared-ownership rule. No existing declaration, owner, availability, or
semantic body changes. Exactly 17 omission identities disappear, with no new or changed failures.
Accounting reports 82,566 ordinary matches, 239 enum-member matches, and 49 missing baseline names.
All 788 partitions, canonical exports, configuration, arguments, omissions, and accounting are
byte-identical between runs. Observed wall times are 231.8/230.6 seconds and peak working sets are
6.83/7.43 GB. A full metadata round trip preserves 36,248 root/nested type shapes.

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
other extra properties, variadics, and exception specifications other than unconditional
`noexcept`, `nothrow`, and empty `throw()` remain unsupported.
Known cdecl/stdcall attributed types retain their written and equivalent types.

Owned function types retain result and parameter edges, calling convention, `noreturn`, and exception
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

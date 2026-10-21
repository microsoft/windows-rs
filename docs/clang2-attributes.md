# Clang2 canonical attribute inventory

The current priority is readable, useful canonical RDL, not zero differences from legacy metadata.
This inventory separates publication contracts, native evidence, and verbose spelling. The
dispositions below are recommendations, not permission to remove contracts or change signatures.
See the [acceptance plan](clang2-acceptance.md) for sequencing and
[review tracking](clang2-evidence.md#current-review-disposition) for deferred work.

## Inventory control

Source: `1a27804432`, with the D27-D28 candidate visible in `metadata/win32`. The comparison is
Git HEAD's tracked legacy RDL, not a previous partial candidate. Counts are attribute occurrences,
not matching lines, APIs, targets, or compiled custom-attribute rows. Repeated declarations and
architecture variants count as written. Comments and quoted strings are excluded.
The family counts below retain this pre-cleanup control; the following section records the
implemented spelling deltas.

| Measure | Tracked RDL | Candidate RDL |
| --- | ---: | ---: |
| Files | 671 | 675 |
| UTF-8 bytes, normalized LF | 16,370,875 | 21,151,170 |
| Explicit `#[...]` occurrences | 79,846 | 186,968 |
| Bytes inside those complete attribute spellings | 1,204,378 | 5,226,232 |
| Fully qualified `Windows::Win32::Metadata::*` occurrences | 0 | 52,747 |

The qualified attributes occupy 3,701,630 bytes. Their repeated namespace prefix alone occupies
1,371,422 bytes. This is text cost, not a claim that their semantics are redundant. A zero in the
tracked qualified-attribute column does not mean the metadata lacks that contract: array counts,
alignment and bitfields already have short or structural RDL forms.

The review's 17,115 `NativeInput` uses are matching lines. There are **28,329 occurrences** on those
lines, occupying 1,619,939 bytes. Every occurrence also has an explicit `#[in]` on that parameter.

## Direction-only cleanup

Shared direction rendering changes 461 of the 675 candidate partitions. All other declaration
text and custom-attribute payloads remain unchanged. Both complete RDL sets compile to
byte-identical WinMD: 14,055,424 bytes, SHA-256
`3e6a7e9ee9c2d9fcb345059dc82aaf62d1a99e32e03347549b9829b8afcda143`.

| Measure | Before | After |
| --- | ---: | ---: |
| UTF-8 bytes, normalized LF | 21,151,170 | 20,794,144 |
| Explicit attribute occurrences | 186,968 | 130,077 |
| `in` | 46,912 | 5,701 |
| `out` | 24,454 | 8,774 |
| `NativeInput` | 28,329 | 28,329 |

This removes 56,891 redundant direction attributes and 357,026 bytes without reducing published
meaning. Native evidence, all declaration counts, omission diagnostics, and constant/function
coverage are unchanged. This spelling-only control precedes the approved input normalization below.

## Compact retained contracts

Compared with the direction-cleaned control, compact contracts change 331 of the 675 partitions
and remove another 673,265 bytes, reducing canonical RDL to 20,120,879 bytes. The complete
before/after WinMD renders to identical canonical
RDL under the updated writer. Architecture-specific comparisons preserve signatures, flags and
custom-attribute values. Binary attribute order can change; byte identity is not claimed.

| Contract | Occurrences | Canonical form |
| --- | ---: | --- |
| Typed bitfield members | 3,384 | `name: Type: width` inside the backing field's block |
| Inherited virtual slots | 1,231 | `#[override_slot(N)]` |
| Field alignment | 7 | `#[align(N)]` |
| Returned pointers | 5,133 | `#[out_ptr]` with independent `nullable` and `null_on_failure` flags |
| Zero termination | 693 | `#[null_terminated(...)]` with separate entry/success and single/double phases |

Typed members retain their `MemberType` property, offsets and widths. Untyped members and anonymous
padding do not acquire types. Unknown properties or non-block-compatible offsets retain full
attributes. Quoted member names preserve strings such as `self`, `crate` and `_` without identifier
substitution; quoted `"_"` is distinct from anonymous padding. Returned-pointer flags preserve
both Boolean properties, including explicit false; outer optionality, COM retval, direction and
buffer counts remain separate.

Termination uses `entry`, `success`, `double_entry` and `double_success`, with at most one flag per
phase. It adds no encoding, count, direction or failure guarantee. Full spelling is retained for
unknown, incomplete or inconsistent payloads, including explicitly false double-termination
properties. This family accounts for 40,980 of the removed bytes. All eight supported phase
combinations retain their exact metadata properties.

Declaration counts, 29,562 omission diagnostics, and required function/constant gaps are unchanged.
This spelling-only comparison retains NativeInput, residual SAL, C++ exception/default and
buffer-length attributes. Input normalization has separate approval under D30.
Focused metadata-only paths reuse the existing bitfield, alignment, nested-record and SDK fixtures
without generating or compiling Rust consumers.

## Qualified attributes: complete inventory

All names below have the `Windows::Win32::Metadata::` prefix. All have zero qualified occurrences
in the tracked RDL. "Consumer" means semantic use in the current reader/bindgen code, not generic
custom-attribute transport or a round-trip assertion.

| Attribute | Count | Meaning and owner | Current consumer and proposed disposition |
| --- | ---: | --- | --- |
| `NativeInput` | 28,329 | Synthesized duplicate of the canonical input contract, with a source SAL/RPC distinction. | Removed from publication under D30. Existing direction/optionality remains; native annotations and reconciliation retain their source distinctions. |
| `NativeSal` | 6,236 | Residual source annotation name, version, argument strings, expansion and optional owner. | Generic transport only. Classify into normalized contracts, native-only evidence, and explicitly unsupported relationships; do not give every raw SAL spelling a new attribute. |
| `NativeExceptionSpecification` | 5,807 | Callable's written `nothrow` or `noexcept` form. | Removed from publication under D31. Native exception specifications remain checked during agreement and callback matching; `noreturn` remains a public contract. |
| `NativeReturnedPointer` | 5,133 | Parameter's returned pointee validity/nullability and null-on-failure guarantee. | No dedicated consumer. Retained losslessly through `out_ptr` syntax, distinct from outer `opt`, `out` and COM `retval`. |
| `NativeBitfield` | 3,384 | Logical member name, bit offset, width and typed member identity on a backing field. | Bindgen consumes it. Typed structural RDL now retains `MemberType`; no property is removed. |
| `NativeOverrideSlot` | 1,231 | Absolute inherited virtual slot on an overriding method. | RDL clears `NewSlot`; bindgen validates and reuses the slot. Retained as `override_slot(N)`. |
| `NativeNullTerminated` | 693 | Parameter entry/success termination, with single versus double zero. | No dedicated consumer. Retained losslessly through `null_terminated` with phase-specific single/double flags, independent of direction, encoding and capacity. |
| `NativeFieldBuffer` | 596 | Record field's capacity/readable sibling-field relationships, units and optional storage. | No dedicated consumer. Distinct field-owned contract, not callable `len_param`. Evaluate compact named-field syntax. |
| `NativeInitializedLength` | 523 | Successfully initialized extent, count indirection, units and optional-count fallback. | No dedicated consumer. Do not fold into capacity. A compact form must preserve dereference and null fallback. |
| `NativeDefaultArgument` | 432 | Parameter's printed C++ default expression, not an evaluated value. | No dedicated consumer. Prefer native-only evidence unless a concrete default-argument publication contract is selected. Do not turn it into `opt` or change callable arity. |
| `NativeConstMethod` | 139 | Native C++ method const qualification. | No dedicated consumer. Not parameter `in`, Rust shared-reference safety, or object immutability. Decide native-only versus an explicit method contract. |
| `NativeArrayInfo` | 136 | Element capacity plus `PostWritable = true`. | Count is consumed; `PostWritable` is not. Extend the existing count spelling only if the extra phase guarantee remains published; do not discard the property to force `len_param`. |
| `NativeDeprecated` | 76 | Declaration/parameter deprecation message, replacement and optional typedef owner. | No dedicated consumer. Useful diagnostic contract; consider normalized deprecation syntax with owner preservation, not raw attribute renaming alone. |
| `NativeBase` | 25 | Marks the field representing a native value-record base. | Storage is represented by the field; no dedicated consumer of the marker. Retain native origin in evidence; decide whether public inheritance identity needs a marker. |
| `Alignment` | 7 | Required member alignment without replacing the field type. | Bindgen consumes it. Field owners now use `align(N)` with the same metadata payload. |

The seed vocabulary also contains attributes rendered through existing syntax, including
`NativeEncoding`, `RetVal`, `SupportedArchitecture`, `MemorySize` and `DoesNotReturn`.
`NativeTypedef` is represented by type declarations; `NativePointer64` and per-level constness are
signature modifiers. They are not absent merely because this textual attribute inventory has no
qualified row for them.

## Existing RDL attributes: complete inventory

These counts include type/declaration attributes and parameter flags. Changes mix scope, native
corrections, projection policy and spelling differences; the delta is not automatically a bug.

| RDL spelling | Tracked | Candidate | Required treatment |
| --- | ---: | ---: | --- |
| `in` | 7,273 | 46,912 | Keep direction; omit only spellings that the RDL reader reconstructs identically. |
| `out` | 9,895 | 24,454 | Same; preserve explicit input/output combinations and mutable input pointers. |
| `opt` | 13,311 | 14,671 | Outer argument optionality, not returned-pointee nullability. |
| `retval` | 5,576 | 5,576 | COM result parameter, not every output pointer. |
| `iid_is` | 370 | 374 | COM output/interface-ID relationship; not a general pointer or buffer marker. |
| `len_param` | 3,442 | 2,809 | Element-capacity relationships need independent review of losses and richer replacements. |
| `size_param` | 2,040 | 1,657 | Byte-capacity relationships; do not interchange with element counts. |
| `len_const` | 101 | 0 | Unresolved contract difference. Inspect constant bounds and residual SAL; do not guess expression values. |
| `reserved` | 546 | 546 | Required zero/null input is distinct from optionality. |
| `encoding` | 3,533 | 5,599 | Character encoding, not termination or capacity. |
| `noreturn` | 4 | 4 | Preserve callable behavior; unrelated to no-throw. |
| `flags` | 318 | 0 | Unresolved public metadata difference. Native flag-enum attributes are supported, but this workload emits none; decide how authored flag semantics are justified. |
| `special` | 4,862 | 0 | Unresolved method/property metadata difference, not cosmetic text. The reader maps it to `SpecialName`. |
| `guid` | 6,444 | 6,779 | Preserve identity independently of count changes. |
| `no_guid` | 40 | 108 | Explicit missing UUID; does not justify a fabricated UUID. |
| `arch` | 1,799 | 2,776 | Preserve actual availability and approved corrections. |
| `packed` | 874 | 2,317 | Separate written packing requests from effective layout. Canonical simplification needs an explicit contract. |
| `align` | 19 | 20 | Keep required storage alignment. |
| `repr` | 4,512 | 4,547 | Keep enum/storage representation. |
| `scoped` | 10 | 10 | Preserve native enum semantics. |
| `library` | 14,206 | 14,387 | Preserve checked provider/export ownership; counts include target variants. |
| `win32` | 671 | 675 | Module-kind marker, not per-declaration annotation noise. |

## Input normalization

D30 approves SAL1/SAL2/RPC input publication through existing RDL direction and optionality.
`_In_` maps to input; `_In_opt_` additionally maps to `opt`. Explicit `in` is omitted when the
written type already implies it. `NativeInput` is not emitted for any parameter category.
No handle-name rules, pointee guesses or COM-specific exceptions are needed.

The regenerated candidate removes all 28,329 uses in 432 partitions and saves 1,648,268 bytes.
It contains 18,472,611 bytes and 98,364 explicit attributes. Every partition matches the preceding
candidate after deleting only those attributes and their separating spaces. Parameter flags,
signatures and every other custom-attribute payload remain identical on all three targets.
Declaration counts, omissions and constant/function coverage are unchanged.

For example, `_In_ LPUNKNOWN pUnk` in `CoIsHandlerConnected` publishes as
`pUnk: LPUNKNOWN`, with inferred input direction. It supplies an interface reference, not an
immutability promise about the COM object.

Native evidence and agreement remain separate. SAL/RPC input distinctions are retained in
`ParameterContract` and compared by `resolve::parameters_equal`. The `native_input` fixture covers
scalars, enums, handles, records, pointers, references and COM objects across imports, callbacks
and methods. SAL and RPC variants now produce identical RDL, while conflicting native observations
still fail reconciliation. Annotated and unannotated const-pointer controls retain their native
evidence without requiring different public input attributes.

The schema retains the legacy attribute definition for existing explicit metadata inputs and
static consumer fixtures. It is not an active clang2 publication contract. Removing that legacy
schema support is separate from changing generated canonical API declarations.

## Native-only exception specifications

D31 keeps unconditional `noexcept`, `nothrow` and empty `throw()` in native evidence rather than
publishing their C++ spellings. These specifications have C++ language/compiler semantics, but the
metadata attribute supplies no cross-language unwind ABI, translation or consumer optimization.
An unannotated API is not thereby declared to throw, and removal does not permit foreign unwinding.

The candidate removes 5,807 occurrences in 43 partitions: 5,788 `nothrow` and 19 `noexcept`.
This saves 470,310 bytes, reducing canonical RDL to 18,002,301 bytes and 92,557 explicit attributes.
Every other RDL token, metadata signature, flag and custom attribute remains unchanged on all three
targets. Coverage and omissions are unchanged; all four SDK `noreturn` contracts remain.

Native comparison still distinguishes exception specifications, including named callback matching.
Supported anonymous non-throwing pointers can use existing C/stdcall FNPTR metadata without a
named callback solely to carry an exception attribute. The focused fixture checks both conventions,
empty `throw()`, strict/partial output and conflicts in either input order. `noreturn` pointers
still require an attribute-owning callback. Computed/dependent and nonempty dynamic specifications
remain unsupported; no native decoding or rejection is weakened.

As with NativeInput, the schema keeps the legacy definition for existing explicit metadata inputs.
New canonical declarations do not use it.

## Remaining publication review

The candidate has 6,742 qualified attribute occurrences across eight families. Residual SAL
accounts for 4,815 of them. Native capture is not, by itself, a reason to publish each source spelling.

| Family | Current count | Review finding and next decision |
| --- | ---: | --- |
| Residual SAL | 4,815 | Still 148 annotation names after D32 result-use normalization. Separate success predicates, conditional relationships and analysis-only evidence. Do not blanket-delete unresolved safety contracts. |
| Field buffers | 596 | Sibling-field capacity/readable relationships, units and optional storage carry distinct information. A concise field-owned form must preserve them. |
| Initialized length | 523 | Thirty-five payload forms include indirect counts and null fallback. These are successful initialized extents, not caller-supplied capacity. |
| Extended array capacity | 136 | Seven payload forms add `PostWritable = true`. Extending `len_param` must retain this phase guarantee. |
| Default arguments | 432 | Forty expression spellings; 292 are `"0"`. Candidate for native-only C++ evidence, not `opt` or a change in callable arity. |
| Const methods | 139 | C++ method qualification is not a Rust shared-reference or COM immutability contract. Candidate for native-only evidence. |
| Deprecation | 76 | Messages, replacements and typedef ownership can support useful diagnostics. Prefer normalized syntax over removal. |
| Native bases | 25 | Base storage already has a field. Decide whether consumers need the inheritance-origin marker separately. |

The publication decisions above remain open; this review does not remove further families.
Missing `flags`, `special` and constant-count attributes also remain unresolved, as do required
API/partition losses. The unchanged 198 unapproved function-target gaps and 15 missing constant
names are not excused by smaller RDL. Downstream qualification remains deferred.

## Residual SAL is not a canonical vocabulary

After D32, 4,815 `NativeSal` occurrences contain 148 source annotation names and 1,493 complete payload
spellings. Fifty-four carry `TypedefName`; none in this candidate carry `Modifiers`. Scoped
termination fixtures still exercise qualifier ownership outside this workload.

| Representative residual evidence | Count | Candidate canonical treatment |
| --- | ---: | --- |
| `_Check_return_`, `_Must_inspect_result_` | 0 direct occurrences | The 1,071 + 350 callable-result occurrences normalize to `#[must_use]` under D32. Nested conditional occurrences remain evidence, not unconditional contracts. |
| `_Success_` | 591 | Success predicate governing postconditions. Do not replace different return-code conventions with an assumed Boolean success. |
| `_When_` | 341 | Conditional contract; must retain condition and owner or remain unsupported evidence. |
| `_Out_writes_opt_`, `_Out_writes_` | 363 + 215 | Some forms already lower to counts; these residual instances require expression/owner analysis, not another direction attribute. |
| `_Outptr_result_buffer_`, `_Outptr_result_bytebuffer_` | 177 + 118 | Returned buffer extent, separate from caller-supplied capacity and outer optionality. |
| `_In_range_` | 211 | Value constraint; distinct from input direction. |
| `__RPC__deref_out_opt_string` | 155 | Inner pointer/string contract; outer `opt` alone loses information. |
| `__RPC_unique_pointer` | 129 | Typedef-level RPC pointer semantics, not blanket optional parameters. |
| `_IRQL_requires_max_` | 128 | Execution-context precondition, not parameter direction or a buffer relationship. |

The remaining names include field-size/range contracts, indirect counts, result nullability,
failure effects, allocator/freeing evidence, format strings and analysis annotations. The same SAL
name can be normalized in one signature and remain evidence in another. Do not classify residual
instances by name alone, parse their strings as compiler-evaluated expressions, or hide unresolved
safety contracts by removing the evidence attribute.

## Canonical result use

D32 maps direct, unscoped, argument-free SAL2 `_Check_return_` and `_Must_inspect_result_` on
callable results to `#[must_use]`, encoded as parameterless `MustUseAttribute`. The caller should
use the result rather than accidentally discard it. This is not `retval`, `noreturn`, purity or
a success predicate; it does not change the signature or promise Rust diagnostics.

The SDK has 1,228 method-owned and 193 callback-owned occurrences. All 1,421 now use the marker,
across 55 partitions. This removes 84,868 bytes, leaving 17,917,433 bytes and 92,557 explicit
attributes. Every other RDL token and all unrelated metadata attributes, signatures and flags
remain unchanged on all three targets. Omissions and constant/function coverage are unchanged.

Microsoft's [SAL function-behavior documentation][sal-function] distinguishes return checking from
inspection of output parameters and globals. Only callable-result owners normalize here.
Parameter, field, constant and value-typedef annotations keep their evidence, as do scoped,
conditional, other-version and nonempty payloads. Callback aliases inherit result use; duplicate
source annotations produce one marker. Native observations retain source identity and conflicts
remain errors. This is a result-use contract, not a reproduction of SAL's dataflow checker.

The spelling follows Rust's [must-use attribute][rust-must-use], without message arguments or a
downstream lint implementation. `_Success_` and `_Return_type_success_` remain unchanged and are
the next review: they describe successful outcomes and postconditions, not whether a result is used.

[sal-function]: https://learn.microsoft.com/en-us/cpp/code-quality/annotating-function-behavior
[rust-must-use]: https://doc.rust-lang.org/reference/attributes/diagnostics.html#the-must_use-attribute

## Representation and next decisions

Short RDL syntax is appropriate for meaningful metadata, but cannot make an unhelpful contract
useful. Decide semantics first, then add reversible syntax for the retained contract.

| Priority | Work | Completion check |
| --- | --- | --- |
| 1 | Canonical direction spelling and NativeInput disposition | Complete under D30. Direction/optionality is unchanged, NativeInput publication is removed, and native annotation conflicts remain checked across parameter kinds. |
| 2 | Buffer, returned-pointer and termination model | Capacity, initialized length, units, inner/outer nullability and entry/success/failure remain distinct. Success predicates are not silently assumed. |
| 3 | Residual SAL and C++ evidence policy | Each family has a normalized contract, a native-only disposition, or an explicit unsupported boundary. No opaque evidence dump masquerades as interpreted metadata. |
| 4 | Compact RDL for retained structural contracts | Typed bitfields, overrides and member alignment round-trip without losing types, slots or layout. Syntax for richer counts preserves every property. |
| 5 | Lost existing attributes and non-attribute differences | Dispositions for flags, SpecialName, constant counts, packing and required missing declarations are based on native evidence and public requirements, not raw parity. |

The reader/writer share `PSEUDO_ATTRS` for one-property short forms. Multi-property contracts need
explicit parsing/rendering, not a lossy entry in that table. Direction rendering uses
`windows_rdl::emit::write_param_direction` in both the RDL writer and clang2. It omits input on
nonmutable written types and output on mutable pointers/references, retaining both flags for
input/output parameters. A named pointer alias has the reader's default input direction, not the
default of its underlying pointer. An absent source direction still uses the reader's defaults.
Equivalent import declarations render comparison directions after normalizing comparison types;
an output pointer and its typedef must not conflict because only one spelling needs `out`.

Direction spelling retains parameter flags and optionality. D30 separately removes redundant
NativeInput publication while retaining native annotations and every other custom attribute.
The `test_rdl` parameter matrix checks all four direction combinations on values, pointer aliases,
mutable/const pointers and references; clang2's `native_input` fixtures cover imports, callbacks
and COM methods on all three targets.

Metadata-level fixtures and exact RDL diffs are the immediate checks. Full package runs, generated
Rust changes, runtime qualification and delivery work wait until this model is accepted.

## Implementation references

| Responsibility | Source |
| --- | --- |
| Attribute schema and property meanings | `metadata/metadata.rdl` |
| Source SAL classification and scoped evidence | `crates/libs/clang2/src/sal/mod.rs`: `shape`, `parameter`, `parameters`, `fields` |
| Canonical parameter/field emission | `crates/libs/clang2/src/projection/annotations.rs`: `parameter`, `buffer`, `field`, `write_with_sal` |
| Default arguments and method contracts | `crates/libs/clang2/src/projection/values.rs`: `parameter_list`, `interface` |
| Typed bitfield storage | `crates/libs/clang2/src/projection/bitfields.rs` |
| Shared short forms and exact property matching | `crates/libs/rdl/src/lib.rs`: `PSEUDO_ATTRS`, `pseudo_for_metadata` |
| Direction defaults | `crates/libs/rdl/src/reader/param.rs`: `parse_param_attributes` |
| Compact attribute rendering | `crates/libs/rdl/src/writer/mod.rs`: `write_params`, `write_custom_attributes_except` |
| Lossless typed bitfield blocks | `crates/libs/rdl/src/reader/field.rs`: `BitfieldMember`; `crates/libs/rdl/src/writer/struct.rs`: `collect_bitfield_members` |
| Existing count consumers | `crates/libs/metadata/src/reader/tables/method_param.rs`: `buffer_relationship` |
| Existing input-only projection | `crates/libs/bindgen/src/tables/method_def.rs`: `signature` |

For a quick occurrence check of qualified attribute families (not a matching-line count):

```powershell
rg --no-filename --only-matching '#\[Windows::Win32::Metadata::[A-Za-z_]+' metadata\win32 |
    Group-Object | Sort-Object Count -Descending | Select-Object Count, Name
```

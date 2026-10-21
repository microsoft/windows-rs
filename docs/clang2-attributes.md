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

## Named buffer extents

D36 uses named `readable` and `writable` attributes rather than extending positional legacy
attributes. `BufferExtentAttribute` keeps access, phase, units and count source separate.
The first native slice covers existing parameter/field contracts and one indirect count:

```rust
#[out] #[opt]
#[writable(entry, bytes(if_present(*lpcbData)))]
#[readable(success, bytes(if_present(*lpcbData)))]
lpData: *mut u8
```

The two occurrences of `*lpcbData` refer to different values: capacity on entry and readable bytes
on successful completion. Full SAL buffers retain entry values through `on_entry(*count)`. RPC
post-writable capacity is a separate `writable(success, count)` contract. Field extents use sibling
names without a phase argument, such as `writable(capacity)` and `readable(length)`; nullable fields
keep that permission. Null count fallbacks become
`or(*written, capacity)`, not an unconditional dereference. Direction and termination remain separate.

| Measure | D35 control | D36 candidate |
| --- | ---: | ---: |
| Readable extents | 0 | 3,262 |
| Writable extents | 0 | 4,018 |
| Positional `len_param` / `size_param` | 4,466 | 0 |
| Qualified initialized-length / field-buffer / extended-array attributes | 1,255 | 0 |
| Residual SAL occurrences | 4,259 | 3,344 |
| Residual SAL names | 147 | 142 |
| UTF-8 bytes, normalized LF | 18,239,397 | 18,169,981 |

All 915 removed SAL occurrences belong to supported buffer forms with indirect capacities; no raw
SAL is added. Legacy schema attributes remain readable for existing authored inputs. Native capture
and reconciliation retain their evidence, including SAL/RPC distinctions. Public comparison binds
count references positionally even when observed parameter labels differ.

RDL also represents literal and return counts and nonzero literal offsets. Native lowering of those
counts, return-owned allocation extents, containing-record extents, field expressions and compound
success conditions still require focused header/RDL cases. No consumer or ownership conversion is
implied.

### Returned buffers and optional counts

D37 uses indirect successful extents for the 24 SAL2 returned-buffer forms. The pinned SAL definitions
make their access guarantees different:

| Family | Successful returned-buffer guarantee |
| --- | --- |
| `_Outptr_result_buffer_(size)` | Writable capacity, without an initialized-length promise. |
| `_Outptr_result_buffer_to_(size, count)` | Writable capacity and readable initialized length. |
| `_Outptr_result_buffer_all_(size)` | Readable length only; no separate writable extent or incoming count. |
| Byte, optional-storage and maybenull variants | Preserve units, outer optionality and inner nullability independently. |

`IMFMediaBuffer::Lock` supplies both capacity and initialized length through optional output pointers:

```rust
#[out]
#[writable(success, bytes(if_present(*pcbMaxLength)), indirect)]
#[readable(success, bytes(if_present(*pcbCurrentLength)), indirect)]
ppbBuffer: *mut *mut BYTE
```

Both count parameters retain `#[out] #[opt]`. A null count pointer means that size was not requested;
it is unknown, not zero and not a fallback. The successful returned pointer is still initialized and
non-null. `nullable` would permit a null returned pointer; outer `opt` would permit missing pointer
storage. Neither follows from count optionality.

The same `if_present` rule applies to ordinary buffers such as `ReadFile` and conditional count
optionality in registry queries. Explicit null fallbacks keep `or(*count, fallback)`. Parameter flags
and count availability share the same unconditional direction/optionality analysis.

Buffer-only forms do not acquire `out_ptr`'s one-readable-element promise, including when the count
is zero. If a separate matching returned-pointer contract is written, both contracts survive:
the ten SDK combinations with explicit null-on-failure retain that guarantee. Mismatched optionality,
nullability and other distinct contracts remain evidence rather than a permissive union.
Constant and compound count expressions remain SAL evidence; no allocator or ownership is inferred.

| Measure | D36 control | D37 candidate |
| --- | ---: | ---: |
| Readable extents | 3,262 | 3,283 |
| Writable extents | 4,018 | 4,335 |
| Counts with unavailable-value semantics | 0 | 489 |
| Residual SAL occurrences | 3,344 | 3,011 |
| Residual SAL names | 142 | 136 |
| UTF-8 bytes, normalized LF | 18,169,981 | 18,156,697 |

The 333 removed SAL occurrences belong to supported returned-buffer families; no SAL is added.
They produce 338 extents because five `_to_` annotations each describe two independent extents.
All-target metadata comparisons preserve existing extents apart from count availability and retain
all non-buffer contracts. Complete SDK RDL/WinMD round trips preserve every attribute. Declaration
totals, omission diagnostics and frozen constant/function coverage are unchanged.

## Remaining publication review

Direction evidence is corrected under D34 and represented without type inference under D35.
Known directions use explicit `in`/`out`; missing direction has no attribute. The two registry-query
count parameters publish only the output/optional facts shared by their complementary branches,
retaining both conditions. Unresolved direction needs neither an invented contract nor an escape
attribute.

The D35 review control has 6,186 qualified attribute occurrences across eight families. Residual SAL
accounts for 4,259 of them. D36's buffer changes are recorded above; the table retains the review
control counts. Native capture is not, by itself, a reason to publish each source spelling.

| Family | Current count | Review finding and next decision |
| --- | ---: | --- |
| Residual SAL | 4,259 | Still 147 annotation names after D32 result use and D33 bounded success predicates. Preserve complex success conditions, conditional relationships and unresolved safety contracts. |
| Field buffers | 596 | D36 uses readable/writable value-phase extents, preserving sibling names, units and nullable storage. |
| Initialized length | 523 | D36 uses readable success-phase extents, preserving dereference and null fallback. |
| Extended array capacity | 136 | D36 preserves `PostWritable = true` as a writable success-phase extent. |
| Default arguments | 432 | Forty expression spellings; 292 are `"0"`. Candidate for native-only C++ evidence, not `opt` or a change in callable arity. |
| Const methods | 139 | C++ method qualification is not a Rust shared-reference or COM immutability contract. Candidate for native-only evidence. |
| Deprecation | 76 | Messages, replacements and typedef ownership can support useful diagnostics. Prefer normalized syntax over removal. |
| Native bases | 25 | Base storage already has a field. Decide whether consumers need the inheritance-origin marker separately. |

The non-buffer publication decisions above remain open; this review does not remove further families.
Missing `flags`, `special` and constant-count attributes also remain unresolved, as do required
API/partition losses. The unchanged 198 unapproved function-target gaps and 15 missing constant
names are not excused by smaller RDL. Downstream qualification remains deferred.

### Direction evidence

Unannotated parameters have no direction attribute and round-trip neither ECMA-335 direction flag.
Known directions are always explicit. Reserved, termination and other independent contracts do
not manufacture direction.

For exactly two unscoped SAL2 `_When_` branches testing the same native pointer parameter against
zero with `==` and `!=`, plain direction contracts contribute only their intersection. Optionality
requires both branches and cannot relax a direct required contract. Typedef-spelled subjects resolve
to their native types; scalar, compound, scoped and nonexhaustive predicates remain evidence.
The existing SAL transport decoder handles nested hook serialization, not a C-expression evaluator.

`RegQueryValueExA/W::lpcbData` is output and optional in both branches. Input is conditional on
`lpData != NULL` and remains in the retained `_When_` evidence. Do not interpret the common flags
as an output-only binding contract or replace them with unconditional input/output.

The D34 metadata correction changes only the intended parameter flags on x86, x64 and ARM64.
D35 changes spelling without changing metadata: existing authored inputs explicitly retain their
established flags, while clang2 generates directions from native evidence. The legacy clang producer
materializes its established projection policy; that compatibility step does not add defaults to
RDL or clang2. Buffer capacities and initialized lengths are the next normalization slice; the review's
analysis-only removal proposals still need per-contract decisions.

## Residual SAL is not a canonical vocabulary

After D33, 4,259 `NativeSal` occurrences contain 147 source annotation names. Fifty-four carry
`TypedefName`; none in this candidate carry `Modifiers`. Scoped
termination fixtures still exercise qualifier ownership outside this workload.

| Representative residual evidence | Count | Candidate canonical treatment |
| --- | ---: | --- |
| `_Check_return_`, `_Must_inspect_result_` | 0 direct occurrences | The 1,071 + 350 callable-result occurrences normalize to `#[must_use]` under D32. Nested conditional occurrences remain evidence, not unconditional contracts. |
| `_Success_` | 56 | Remaining compound, cast, sentinel and constant-true conditions. D33 normalizes 535 SAL2 callable predicates; do not reduce the remaining contracts to Boolean success. |
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
reviewed separately below: they describe postconditions, not whether a result is used.

[sal-function]: https://learn.microsoft.com/en-us/cpp/code-quality/annotating-function-behavior
[rust-must-use]: https://doc.rust-lang.org/reference/attributes/diagnostics.html#the-must_use-attribute

## Success-predicate investigation

This investigation established the D33 boundary implemented below. Its D32 control contains 614
direct success annotations, with 34 complete payload spellings:

| Source annotation | Count | Actual metadata owners |
| --- | ---: | --- |
| SAL2 `_Success_` | 591 | 563 methods/imports and 28 named callbacks |
| SAL1.1 `__success` | 11 | Methods/imports |
| SAL2 `_Return_type_success_` | 12 | Ten scalar typedefs and two pointer typedefs |

The pinned SDK's `sal.h` defines `__success(expr)` through `_Success_(expr)`. These belong to the
same semantic review, rather than leaving a legacy spelling outside the inventory.
All current payloads have equal captured argument and expansion strings; none has a relocated
typedef owner or scope modifier. That does not establish compiler evaluation of the expressions.

The [SAL specification][sal-function] defines success as the condition under which ordinary
postconditions apply. `_On_failure_` uses its negation; `_Always_` applies on both outcomes.
A function's explicit `_Success_` overrides the default supplied by its return typedef's
`_Return_type_success_`. This is separate from `must_use`, output direction and the error model
chosen by a language binding.

| Callable predicate family | Occurrences | Boundary |
| --- | ---: | --- |
| Truth/nonzero spellings | 295 | Integer/Boolean truth and pointer non-nullness need the actual result type. |
| Equal to zero | 205 | Includes null-handle success on GlobalFree/LocalFree; not always an integer status. |
| Equal to one | 30 | Do not weaken to arbitrary nonzero on BOOL or BOOLEAN results. |
| Nonnegative | 12 | Requires signedness and enum/backing-type analysis. |
| Positive | 4 | Distinct from nonzero on signed results. |
| Constant true | 3 | Postconditions apply unconditionally; this is not an infallibility promise. |
| Compound, cast or sentinel | 53 | Preserve parameter relationships, alternatives, types and unresolved names. |

The first six rows cover 549 of 602 callable annotations by textual shape, not 549 proven
lowerings. The remaining 53 comprise 31 buffer-bound predicates, ten alternate-status predicates,
nine sentinel/handle comparisons, two reallocation conditions and one HRESULT cast.

Concrete cases constrain the design:

- `GetCurrentDirectoryA` requires a nonzero result smaller than `nBufferLength`. A nonzero-only
  rule would wrongly treat an insufficient buffer as satisfying the ordinary output guarantees.
- `NetConnectionEnum` accepts zero or 234 (`ERROR_MORE_DATA`) for its postconditions.
  Its [API documentation][connection-enum] requires freeing the returned buffer even with
  `ERROR_MORE_DATA`. This must not become an automatic complete-success/error split.
- `CryptXmlClose` returns HRESULT but explicitly requires zero. Keep that override rather than
  assuming every HRESULT-returning function uses the typedef's nonnegative default.
- `GlobalReAlloc` and `LocalReAlloc` refer to `_Curr_` and an input size. Do not reinterpret the
  SAL subject or simplify away the zero-size case without establishing its binding.
- `PNTSTATUS` and `PNDIS_STATUS` carry `return >= 0` because the SDK places the annotation on
  shared scalar/pointer typedef declarations. This is present in native capture on all three
  targets, not invented by projection. It does not justify a nonnegative pointer test or applying
  the pointee's status convention to a function with such an output parameter.

The shared declarations are in SDK 10.0.28000.2270's `um/ntsecapi.h:895` and
`shared/ndis/types.h:13`. `shared/ntdef.h:1074-1076` also supplies a separate, unannotated pointer
typedef. Keep the observed annotation sets and their ownership; do not repair these cases by name.
The legacy success wrapper is at `shared/sal.h:2848`.

### Implemented bounded predicates

D33 adds expression-form `#[success(...)]` backed by `SuccessConditionAttribute` and its typed
enum. The supported predicates are `return == 0`, `return != 0`, `return == 1`, `return >= 0`
and `return > 0`. Integer/Boolean and unscoped-enum return types retain distinct comparisons;
nonnegative requires signed storage. Data pointers support only zero/null equality or inequality.
The remaining expressions, floats, scoped enums, incompatible pointer predicates and relocated
nominal-type defaults retain evidence. This is not a C/SAL expression evaluator.

The candidate normalizes 556 occurrences: 535 SAL2 callable predicates, eleven SAL1.1 predicates
and ten scalar typedef defaults. The 58 remaining direct annotations comprise 56 callable
conditions and the two pointer-typedef defaults. All five canonical predicates retain their exact
metadata enum discriminants through RDL/WinMD round trips. Unknown values or extra properties keep
full spelling; unsupported short expressions and parameter/field owners are errors.

Explicit callable conditions remain independent of return-type defaults. Agreeing inherited
callback conditions coalesce; conflicting conditions and source ownership stay as evidence.
Value aliases retain their written links when a return-default owner would otherwise be flattened
away, including annotations from another input. The SDK changes three such definitions:
`CONFIGRET = RETURN_TYPE`, `APP_DEPRECATED_HRESULT = HRESULT`, and
`DEPRECATED_HRESULT = HRESULT`. Their underlying storage is unchanged. Do not traverse pointers
to inherit a default or guess precedence between competing typedef defaults.

Across 74 partitions this removes 35,090 bytes, leaving 17,882,343 bytes and 92,557 attributes.
The only metadata changes are the new enum/attribute definitions, the 556 normalized contracts
and those three alias links. All other RDL text, signatures, flags, constants and attributes are
unchanged on all targets; omission diagnostics and coverage are unchanged. Native conflicts remain
errors even when different source spellings would publish the same predicate.

The `success` header/RDL fixture covers defaults, explicit overrides, alias/callback inheritance,
cross-input annotations, native type guards, competing defaults, scoped and expanded evidence,
partial-result/buffer conditions and the shared-declarator pointer boundary. Downstream error
conversion and diagnostic generation remain deferred. Extending this model to the remaining 58
annotations requires parameter binding, sentinel/cast semantics or a separate constant-true form,
not weaker rejection or guessed numeric values.

[connection-enum]: https://learn.microsoft.com/en-us/windows/win32/api/lmshare/nf-lmshare-netconnectionenum

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
`windows_rdl::emit::write_param_direction` in both the RDL writer and clang2. It emits each established
flag without consulting the parameter type. Pointer mutability and alias spelling cannot supply
missing direction or change equivalent import comparisons. Property setters and event accessors
have input parameters by their member contracts, not by type inference.

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

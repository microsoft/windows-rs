# Canonical attribute policy

`metadata/metadata.rdl` defines the 23 Win32 attribute types published by the header pipeline.
This page describes their current meaning, RDL spelling and consumer boundaries. Native capture
and reconciliation retain more information than canonical publication. Capturing a source
annotation does not require a new public attribute.

## Published vocabulary

Attribute names below omit the `Attribute` suffix. Short forms are lossless encodings, not
permission to discard properties. Extended or unsupported payloads retain full attribute spelling.

| Attribute | Owner and meaning | RDL spelling |
| --- | --- | --- |
| `DoesNotReturn` | Callable never returns | `noreturn` |
| `MustUse` | Callable result should be inspected | `must_use` |
| `SuccessCondition` | Callable success predicate, or return-typedef default | `success(return == 0)` and supported comparisons |
| `NativeEncoding` | String constant or field encoding, not termination | `encoding("...")` |
| `NativeTypedef` | Written value-typedef identity | `type Name = Type;` |
| `NativePointer64` | Fixed-width pointer storage; a required signature modifier | `ptr64<*mut T>` or `ptr64<*const T>` |
| `RetVal` | Explicit COM result parameter | `retval` |
| `SupportedArchitecture` | Declaration availability | `arch(...)` |
| `Alignment` | Raised record/member alignment | `align(N)` |
| `IntegerRange` | Integer value bounds at entry, success or valid-record state | `range(...)` |
| `BufferExtent` | Readable or writable extent at a specified phase | `readable(...)`, `writable(...)` |
| `NativeNullTerminated` | Parameter single/double zero termination at entry or success | `null_terminated(...)` |
| `Nullable` | Permission for null, independent of initialized storage | `nullable(...)`; phase-free on fields/results |
| `NativeReturnedPointer` | Initialized returned pointer and its pointee/failure guarantees | `out_ptr(...)` |
| `Reserved` | Argument must be zero/null | `reserved` |
| `ComOutPtr` | COM output/interface-ID relationship | `iid_is` |
| `ScopedEnum` | Native scoped-enum identity | `scoped` |
| `NativeBitfield` | Logical member type, offset and width in backing storage | Typed or untyped bitfield blocks |
| `NativeOverrideSlot` | Absolute inherited vtable slot | `override_slot(N)` |
| `NativeSal` | Uninterpreted SAL evidence, including scope and relocated typedef owner | Fully qualified |
| `NativeDeprecated` | Native deprecation message, replacement and typedef owner | Fully qualified |
| `NativeConstMethod` | Native C++ method qualification | Fully qualified |
| `NativeBase` | Identifies a field containing native base storage | Fully qualified |

`in`, `out`, `opt` and `special` are metadata flags, not extra types in this seed.
`flags` uses `System.FlagsAttribute`. GUIDs, linkage, representation, packing and module kind
also have established RDL forms rather than new annotation families.

### Publication choices

`NativeSal` remains evidence for later refinement. Deprecation can support cross-language
diagnostics; a compact spelling must preserve message, replacement and owner.
Const-method and base markers remain published pending a separate decision. Const methods do not
promise Rust borrowing safety or COM object immutability. Base layout comes from the field,
not the marker. Do not add shortcuts solely to justify retaining a family.

C++ default expressions and exception specifications remain native-only evidence. Defaults are
not portable Rust/C# expressions, are not evaluated for publication, and do not change arity or
mark parameters optional. Their original expressions still participate in native reconciliation.
Exception specifications do not establish a cross-language unwind ABI. `noreturn` remains public.

The seed does not define `NativeArrayInfo`, `MemorySize`, `NativeFieldBuffer`,
`NativeInitializedLength`, `NativeInput`, `NativeExceptionSpecification` or
`NativeDefaultArgument`. Current canonical declarations use none of them. Legacy
`len_param`, `len_const` and `size_param` reader support and self-contained fixtures remain;
removing unused seed definitions does not remove old-format decoding.

## Buffer contracts

One `BufferExtent` model covers parameters, returned buffers and record fields. Capacity and
initialized length are separate accessible extents, not competing attribute families.

| Dimension | Rule |
| --- | --- |
| Access | `writable` describes capacity; `readable` describes initialized/readable elements |
| Phase | Parameters specify `entry` or `success`; fields implicitly describe a valid record |
| Units | Elements by default; `bytes(count)` explicitly selects bytes |
| Subject | Direct by default; `indirect` selects the initialized pointer stored in a parameter |
| Count | Constant, named parameter, one dereference, sibling field, or bounded integer expression |
| Count phase | Same as the extent unless `on_entry(...)` selects the incoming count |
| Missing count | `if_present(*count)` means unknown extent when count storage is null, not zero |
| Inexpressible count | `unknown()` records an extent without a numeric count; it is not zero |
| Fallback | `or(*written, capacity)` supplies an explicit alternative; optionality invents none |
| Pointer nullability | `nullable` permits null indirect/field storage; outer `opt` is separate |

For example:

```rust
#[out] #[opt]
#[writable(entry, bytes(if_present(*size)))]
#[readable(success, bytes(if_present(*size)))]
buffer: *mut u8
```

The two counts refer to different values at entry and success. Full-buffer forms that depend on
the incoming count use `readable(success, on_entry(...))`. Field relationships use sibling names:
`writable(capacity)` and `readable(length)`.

Returned-buffer contracts use `indirect`. SAL capacity-only forms supply writable capacity;
`_to_` adds initialized length; `_all_` supplies readable length without a separate writable
guarantee. RPC full-buffer forms can supply both. Buffer-only contracts do not acquire
`out_ptr`'s one-readable-element guarantee, including when the count is zero.

`out_ptr(nullable, null_on_failure)` specifies successful pointer initialization, permitted null
results and null on failure. It does not imply allocator identity or ownership transfer.
`null_terminated(entry, double_success)` specifies termination without count, encoding or
direction. Its short form requires a parameter owner and compatible storage; typedef/field
termination evidence is not silently moved onto a parameter.

`nullable` on a result row is independent of success. Nullable fields apply to a valid record.
Nullability alone supplies neither initialized nor readable storage.

Distinct returned-buffer/pointer contracts combine only when their facts agree. Other unresolved
combinations remain evidence; conflicting supported counts fail rather than selecting one.
Source expressions outside the supported grammar remain `NativeSal`, with their owner intact.

An unscoped `_At_` selecting this parameter's pointee can lower one post-readability property
with an `_Inexpressible_` count to `readable(success, unknown(), indirect)` (or byte units).
Other subjects, entry properties, mixed properties and scoped evidence remain opaque. Native
annotation observations still compare their full payloads. Unknown counts do not allow offsets,
arithmetic, `on_entry`, optional-count guards or fallbacks.

### Counts and ranges

Counts and `IntegerRange` share typed integer expressions. Native lowering supports bounded
integer literals, grouping, addition, subtraction and multiplication. C integer promotions and
usual arithmetic conversions apply; signed overflow is not mathematical integer arithmetic.
Casts, unsupported operators and unresolved references remain evidence.

The metadata payload is versioned postfix text, not C, Rust or C# source. Tokens are separated
by one space and start with `v1`.

| Token | Meaning |
| --- | --- |
| `i32:N`, `u32:N`, `i64:N`, `u64:N` | Typed decimal literal |
| `p:N` | Zero-based parameter index |
| `d:N` | Dereferenced parameter value |
| `o:N` | Optional dereference; null makes the entire expression unavailable |
| `f:HEX` | Direct sibling-field name encoded as UTF-8 bytes in lowercase hexadecimal |
| `return` | Callable result |
| `add`, `sub`, `mul` | Binary postfix operators |

Reference types come from the owning signature or record. For example, `count + 1` encodes
`v1 p:0 i32:1 add` when `count` is parameter zero. Unknown versions or payloads must not be
interpreted as known counts. Legacy nonzero `Offset` properties remain explicit compatibility
payloads; new arithmetic uses `CountExpression`.

## Consumer boundary

Metadata describes contracts; it does not automatically authorize slices, scalar return values,
COM queries, ownership wrappers or `Result` conversion.

Bindgen distinguishes the presence of buffer semantics from a count it can simplify:

- Any `BufferExtent` excludes the parameter from scalar-return inference. Generic COM queries
  may retain only unknown, successful, indirect readable extents alongside an explicit returned
  pointer contract and COM marker, with matching nullability. All other buffer attributes block
  the query transformation.
- Only supported direct entry relationships can supply slice/count projection. The subject and
  count must also meet direction, type and count-sharing requirements.
- Parameter-owned `NativeSal` prevents signature-changing inference for that parameter, without
  parsing SAL inside bindgen. An unresolved count parameter cannot be hidden as a slice length.
- Known outer optionality remains separate. Ordinary unambiguous scalar outputs and COM queries
  retain their established projections.

The `test_bindgen` `buffer_contracts` fixture checks canonical extents, residual evidence,
explicit retval, arithmetic, returned storage, optional counts, GUID buffers and known scalar/query
controls. Legacy buffer fixtures continue to check older metadata. Clang header/RDL fixtures
cover native lowering, ownership, conflicts and all-target metadata round trips.
Passing those fixtures does not qualify a separate C# consumer.

## Evidence and scope

Win32 RDL contains 8,392 buffer extents and 1,914 residual SAL occurrences.
WDK adds 308 extents and 1,575 SAL occurrences. Counts are written RDL
occurrences, not unique APIs or merged custom-attribute rows.

Residual SAL includes conditional contracts, unsupported counts, ownership, typedef/field
annotations and execution restrictions. IRQL requirements are real restrictions, not automatically
discardable analysis noise. RPC pointer markers do not imply optional arguments, and field
direction does not imply nullability. Preserve source ownership and scope until a focused API
requires a supported interpretation.

Win32 currently emits no `special` or `flags` markers. Clang `flag_enum` is supported; operator
macros, enum values and names are not substitutes for that evidence. Missing `special` changes
projected property-style method names; missing flags changes generated bitwise operators. Those
public API consequences remain review decisions. WinRT still uses both encodings.

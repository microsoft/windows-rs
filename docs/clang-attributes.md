# Metadata attribute support

`windows-clang` publishes native declarations as RDL; `windows-rdl` encodes and decodes metadata;
`windows-bindgen` projects metadata into Rust. Publication, round-trip support, and Rust behavior
are separate: an attribute can be preserved without affecting generated bindings.

Names below omit the `Attribute` suffix. RDL short forms appear without `#[...]`; see the
[RDL reference](../crates/libs/rdl/rdl.md) for syntax and supported owners. "None" means bindgen
does not interpret that attribute, not that clang or RDL discards it. Projection behavior also
depends on the signature and generator style; raw bindings do not acquire every high-level wrapper.

## Win32 header publication

These are the families in the hand-authored [seed](../metadata/metadata.rdl), under
`Windows.Win32.Metadata`. Clang emits them when the native evidence supports the contract.
RDL supports each through the spelling shown or a named attribute with constructor arguments
and properties. Short forms must preserve the payload; unsupported extensions use named spelling.

| Attribute | Meaning | RDL | Bindgen use |
| --- | --- | --- | --- |
| `DoesNotReturn` | Callable never returns | `noreturn` | Projects void-returning imports, methods, and callbacks as `-> !` |
| `MustUse` | Result should be inspected | `must_use` | None; does not emit Rust `#[must_use]` from this marker |
| `SuccessCondition` | Success predicate or return-typedef default | `success(...)` | None; does not drive `Result` conversion |
| `NativeEncoding` | String constant/field encoding, not termination | `encoding("...")` | `"ansi"` selects narrow string-constant output |
| `NativeTypedef` | Written value-typedef identity | `type Name = Type;` | Typedef and handle representation |
| `NativeAnonymous` | Unnamed nested record's first owning native field position | Inline record or `anonymous(N)` | Anonymous nested-type naming |
| `NativePointer64` | Fixed-width data-pointer storage modifier | `ptr64<*mut T>`, `ptr64<*const T>` | 64-bit pointer storage; not by-value callable ABI support |
| `RetVal` | Explicit result parameter | `retval` | Result-parameter projection, subject to eligibility checks |
| `SupportedArchitecture` | Declaration availability | `arch(...)` | Architecture gating and dependency selection |
| `Alignment` | Raised record/member alignment | `align(N)` | Record layout, padding, and underaligned field storage |
| `IntegerRange` | Integer bounds at entry, success, or valid-record state | `range(...)` | None; no runtime range checks |
| `BufferExtent` | Readable/writable extent at a specified phase | `readable(...)`, `writable(...)` | Restricted slice/count projection; guards result/query inference |
| `NativeNullTerminated` | Single/double zero termination at entry or success | `null_terminated(...)` | None; no string conversion from this marker |
| `Nullable` | Permission for null, not initialized storage | `nullable(...)` | None; distinct from returned-pointer nullability below |
| `NativeReturnedPointer` | Returned-pointer initialization, nullability, and failure guarantees | `out_ptr(...)` | Result/query eligibility and nullable COM results |
| `Reserved` | Argument must be zero/null | `reserved` | Optional/reserved argument handling, not value enforcement |
| `ComOutPtr` | COM output/interface-ID relationship | `iid_is` | Generic COM query projection when eligible |
| `ScopedEnum` | Native scoped-enum identity | `scoped` | Enum representation, constants, and name selection |
| `NativeBitfield` | Member type, offset, and width in backing storage | Typed or untyped bitfield blocks | Bitfield accessors |
| `NativeOverrideSlot` | Absolute inherited vtable slot | `override_slot(N)` | Vtable slot validation and overrides |
| `NativeSal` | Uninterpreted SAL, scope, and relocated typedef owner | Named attribute | Blocks slice, scalar-result, and COM-query inference on annotated parameters |
| `NativeDeprecated` | Message, replacement, and typedef owner | Named attribute | None; no Rust deprecation diagnostics from this marker |
| `NativeConstMethod` | Native C++ method qualification | Named attribute | None; not a Rust borrowing or immutability guarantee |
| `NativeBase` | Field containing native base storage | Named attribute | Recognizes native object/interface-base storage; ordinary bases remain fields |

`NativeAnonymous` records the native field position, which can differ from the metadata field
position after bitfield lowering. Neither anonymity nor base layout is inferred from type names.

## Other attributes

The header pipeline also uses existing namespaces rather than adding Win32 seed families:

| Attribute | Clang/RDL | Bindgen use |
| --- | --- | --- |
| `Windows.Foundation.Metadata.Guid` | Native UUID; `guid(...)` | Interface/class identity and GUID/property-key constants |
| `System.Flags` | `flag_enum` or matching native enum bitwise-OR overload; `flags` | Enum bitwise operators |
| `System.Runtime.InteropServices.UnmanagedFunctionPointer` | Clang: C/system callbacks; RDL also accepts `fastcall` | C/system ABI; `fastcall` maps to `system`, not an x86 fastcall ABI |

The following `Windows.Foundation.Metadata` attributes have dedicated WinRT consumers in bindgen.
Clang does not synthesize these WinRT contracts. RDL uses named spelling except for `Default`,
which is supplied by a class's first interface.

| Attribute | Bindgen use |
| --- | --- |
| `Default` | Class default interface |
| `Static` | Static factory interface |
| `Activatable` | Constructors and activation factories |
| `Composable` | Composition factories |
| `ExclusiveTo` | Class-owned interface exposure and factory reachability |
| `MarshalingBehavior` | Agile-type handling |
| `Overload` | Projected method names and overload filtering |
| `NoException` | WinRT fallible/infallible wrapper signatures, not native C++ `noexcept` |
| `ApiContract` | Omits contract marker structs from generated APIs |

Legacy Win32 buffer attributes are not emitted by clang or defined in the canonical seed, but
remain supported with supplied definitions:

| Attribute | RDL | Bindgen use |
| --- | --- | --- |
| `NativeArrayInfo` | `len_param(...)`, `len_const(...)` | Supported element-count relationships and buffer-inference guards |
| `MemorySize` | `size_param(...)` | Supported byte-count relationships and buffer-inference guards |

RDL also accepts declared custom attributes without dedicated syntax. This is not a promise of
arbitrary metadata round trips: the writer omits `System.*` attributes from generic output, with
dedicated encodings handling the supported cases above. Other named attributes may survive RDL
without any bindgen interpretation.

## Flags and modifiers

`in`, `out`, `opt`, and `special` are metadata flags, not custom-attribute families.
Clang preserves explicit direction and optionality; missing direction is unspecified. MIDL
`[propget]` and `[propput]` comments supply `special`, which affects projected accessor names.
Enum flag evidence can come from `DEFINE_ENUM_FLAG_OPERATORS` through its compiler-observed
declaration, not from macro names, enum names, or values.

Pointer constness uses `System.Runtime.CompilerServices.IsConst` signature modifiers, represented
by RDL pointer types. `NativePointer64` is likewise a modifier, not an ordinary attribute row.
Linkage, calling conventions, representation, packing, and module kind have structural RDL forms.

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

Returned-buffer contracts use `indirect`. Buffer-only contracts do not acquire `out_ptr`'s
one-readable-element guarantee, including when the count is zero.

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

Unknown counts do not allow offsets, arithmetic, `on_entry`, optional-count guards, or fallbacks.

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
- Parameter-owned `NativeSal` blocks slice, scalar-result, and COM-query inference for that
  parameter, without parsing SAL inside bindgen. An unresolved count parameter cannot be hidden
  as a slice length. COM inputs can still use their ordinary borrowed conversions.
- Known outer optionality remains separate. Ordinary unambiguous scalar outputs and COM queries
  retain their established projections.

## Native-only and residual evidence

C++ default expressions and exception specifications participate in native reconciliation but
are not published as invocation contracts. Defaults do not change arity or mark parameters
optional; exception specifications do not establish a cross-language unwind ABI.
`NativeDefaultArgument` and `NativeExceptionSpecification` are not canonical seed families.

Residual SAL includes conditional contracts, unsupported counts, ownership, typedef/field
annotations and execution restrictions. IRQL requirements are real restrictions, not automatically
discardable analysis noise. RPC pointer markers do not imply optional arguments, and field
direction does not imply nullability. Preserve source ownership and scope until a focused API
requires a supported interpretation.

## Implementation references

- [Clang annotation publication](../crates/libs/clang/src/projection/annotations.rs)
- [RDL short-name mappings](../crates/libs/rdl/src/lib.rs) and
  [attribute writer](../crates/libs/rdl/src/writer/mod.rs)
- [Bindgen parameter contracts](../crates/libs/bindgen/src/param.rs),
  [native projections](../crates/libs/bindgen/src/types), and
  [shared metadata decoding](../crates/libs/metadata/src/reader)

# Header annotations

Win32 headers describe API contracts with compiler syntax, SAL and MIDL. `windows-clang` reads them,
publishes RDL, `windows-rdl` encodes ECMA-335 metadata, and `windows-bindgen` projects that metadata
into Rust. This page lists every contract the pipeline carries, with one header source, one RDL
spelling and one metadata encoding for each.

Everything below is implemented unless marked **Proposed**. Proposed rows depend on the
win32metadata `_Win32_*` macros under review in microsoft/win32metadata#2365.

| Column | Meaning |
| --- | --- |
| Header | Source syntax; earlier entries are preferred |
| RDL | Short form written inside `#[...]`; "Named" means the full attribute path |
| Metadata | ECMA-335 flags and modifiers, or a `Windows.Win32.Metadata` attribute without its `Attribute` suffix |
| Bindgen | Effect on generated Rust; "-" means preserved but not interpreted |

## Example

```c
_Success_(return != 0) _Check_return_ BOOL WINAPI ReadPayload(
    _In_reads_(count) const int* values,
    _In_ unsigned int count,
    _Out_writes_bytes_to_opt_(*size, *size) void* buffer,
    _Inout_ unsigned int* size,
    _Reserved_ void* reserved);
```

```rust
#[library("example.dll")]
#[must_use]
extern fn ReadPayload(
    #[in] #[readable(entry, count)] values: *const i32,
    #[in] count: u32,
    #[opt] #[out] #[readable(success, bytes(*size))] #[writable(entry, bytes(*size))] buffer: *mut void,
    #[in] #[out] size: *mut u32,
    #[reserved] reserved: *mut void,
) -> BOOL;
```

```rust
pub unsafe fn ReadPayload(
    values: &[i32],
    buffer: Option<*mut core::ffi::c_void>,
    size: *mut u32,
    reserved: Option<*mut core::ffi::c_void>,
) -> BOOL
```

`values` becomes a slice because its count is a direct entry parameter. `buffer` keeps a pointer
because its count is read through `*size`. `_Success_` is not published; it only grounds `retval`
inference. `must_use` adds no attribute here because `BOOL` is already `#[must_use]`.

## Principles

| Rule | Meaning |
| --- | --- |
| Existing syntax first | Compiler syntax, SAL and MIDL own every fact they can express |
| Fix headers, not metadata | Wrong or missing SAL is corrected in the header, not overridden downstream |
| One fact, one spelling | Each fact has one RDL spelling and one metadata encoding; no aliases |
| Structure first | ECMA-335 flags, signatures and modifiers come before custom attributes |
| Checked against evidence | Facts the compiler can confirm must agree with it or publication fails |
| Closed vocabulary | Unknown names, bad arguments, wrong owners and repeated annotations fail |
| Keep native types | Associations are added beside the native type, never by retyping |
| Publish what consumers can act on | Facts without a canonical family are not published; the header remains their source |
| Absent means unspecified | A missing fact is not a promise; `incomplete` marks parameters whose header says more |
| Native prefix | `Native` attributes preserve C/C++ declaration details; unprefixed attributes state language-neutral API facts |
| Separate stages | Metadata describes contracts; bindgen decides independently what Rust to generate |

## Parameters

| Fact | Header | RDL | Metadata | Bindgen |
| --- | --- | --- | --- | --- |
| Direction | `_In_`, `_Out_`, `_Inout_`; MIDL `[in]`, `[out]` | `in`, `out` | Param flags `In`, `Out` | Input and output projection; missing direction stays unspecified |
| Optional argument | SAL `_opt_` forms; MIDL `[optional]` | `opt` | Param flag `Optional` | `Option` for pointer parameters |
| Reserved argument | `_Reserved_` | `reserved` | `Reserved` | Handled like an optional argument; the value is not enforced |
| Result parameter | MIDL `[retval]`; a required non-null `_COM_Outptr_`, or `_Outptr_` to an interface, as the trailing typed `T**` when success is `return >= 0` | `retval` | `RetVal` | Returned as the Rust result when also `out` only, not `opt` or `reserved`, and free of `BufferExtent` and `incomplete` |
| Returned pointer | `_Outptr_` forms, `__deref_out`, `_Deref_out_`, `__RPC__deref_out*`; `_COM_Outptr_` and `_Outptr_result_nullonfailure_` add `null_on_failure` | `out_ptr`, `out_ptr(null_on_failure)` | `ReturnedPointer` | `null_on_failure` makes COM implementations clear the output on error |
| COM query | `_COM_Outptr_` on `void**`; MIDL `[iid_is]` | `iid_is` | `ComOutPtr` | Generic `QueryInterface`-style projection with a trailing `*const GUID` input |
| Nullable pointer | `_Outptr_` and `_COM_Outptr_` `maybenull` forms; `__deref_[opt_]out_opt`, `_Deref_[opt_]out_opt_`; `__RPC__deref_out_opt_string`, `__RPC__deref_out_ecount_*_opt`; `_Ret_maybenull_`; `_Maybenull_` and `_Field_size_*_opt_` on fields | `nullable` | `Nullable` | `Option<T>` for interface results |

`opt` lets the caller pass a null argument. `nullable` records that the API may produce null: the
pointer stored through an `out` parameter, a return value, or a field. Absence does not promise
non-null; only an `out_ptr` or indirect extent without `nullable` does. The `_opt_` in
`_Outptr_opt_` and `_COM_Outptr_opt_` makes the argument optional; the returned pointer is still
non-null on success. Consumers should still handle null where the header makes no promise.
`out_ptr` promises one readable element on success; it does not imply an allocator or ownership
transfer.

## Buffers and strings

| Fact | Header | RDL | Metadata | Bindgen |
| --- | --- | --- | --- | --- |
| Buffer extent | `_In_reads_`, `_Out_writes_`, `_Inout_updates_` and their `_bytes_`, `_to_`, `_opt_` forms; `_Field_size_`; RPC `__RPC__in_ecount_full` and related | `readable(...)`, `writable(...)` | `BufferExtent` | Slices for direct entry counts; blocks `retval` and query projection |
| Termination | Parameter `_z_` forms; `_Pre_` or `_Post_` with `_Null_terminated_` or `_NullNull_terminated_`; RPC `__RPC__in_string` and related | `null_terminated(entry, double_success)` | `NullTerminated` | - |
| String encoding | Character type of a string constant or field | `encoding("ansi")` | `Encoding` | `"ansi"` selects narrow string constants |

### Extent model

One `BufferExtent` covers parameters, returned buffers and record fields. Capacity and
initialized length are separate extents of the same family.

| Dimension | Values |
| --- | --- |
| Access | `writable` is capacity; `readable` is initialized elements |
| Phase | `entry` or `success` on parameters; `success` holds after the API succeeds, a condition that metadata does not publish; fields describe a valid record and omit the phase |
| Units | Elements by default; `bytes(count)` selects bytes |
| Subject | The parameter itself by default; `indirect` selects the pointer stored in it |
| Count | Constant, parameter, `*parameter`, sibling field, `return`, or `+ - *` of these |
| Count phase | Same as the extent unless `on_entry(count)` selects the incoming value |
| Null count | `if_present(*count)` makes the extent unknown when the count pointer is null |
| Fallback | `or(*written, capacity)` names an explicit alternative |
| Unknown | `unknown()` records an extent whose size is not expressible; it is not zero |

| Header | RDL |
| --- | --- |
| `_In_reads_(count)` | `#[in] #[readable(entry, count)]` |
| `_Out_writes_bytes_to_opt_(*size, *size)` | `#[opt] #[out] #[readable(success, bytes(*size))] #[writable(entry, bytes(*size))]` |
| `_Out_writes_all_(count)` | `#[out] #[writable(entry, count)] #[readable(success, count)]` |
| `_Outptr_result_buffer_(*count)` | `#[out] #[writable(success, *count, indirect)]` |
| `_Outptr_result_maybenull_z_` | `#[out] #[out_ptr] #[nullable] #[null_terminated(success, indirect)]` |
| `_Field_size_(capacity)` on a field | `#[writable(capacity)]` |

Counts outside this grammar, such as casts or unresolved names, mark the parameter `incomplete`.
Conflicting supported counts fail rather than selecting one.

### Count encoding

| Count | `BufferExtent` property |
| --- | --- |
| Constant | `CountConst` |
| Parameter, or `*parameter` with `Dereference` | `CountParamIndex` |
| Sibling field | `CountFieldName` |
| `return` | `CountReturn` |
| `unknown()` | `CountUnknown` |
| Arithmetic, `if_present` inside arithmetic | `CountExpression` |
| `if_present(*count)` | `CountOptional` |
| `or(..., fallback)` | `NullFallbackParamIndex` |
| `on_entry(...)` | `CountOnEntry` |

`CountExpression` uses versioned postfix text, separated by single spaces
and starting with `v1`. Unknown versions must not be interpreted.

| Token | Meaning |
| --- | --- |
| `i32:N`, `u32:N`, `i64:N`, `u64:N` | Typed decimal literal |
| `p:N` | Zero-based parameter index |
| `d:N` | Dereferenced parameter |
| `o:N` | Optional dereference; null makes the whole expression unavailable |
| `f:HEX` | Sibling field name as lowercase hexadecimal UTF-8 |
| `return` | Callable result |
| `add`, `sub`, `mul` | Binary operators |

For example, `count + 1` with `count` as parameter zero encodes `v1 p:0 i32:1 add`. C integer
promotions apply; signed overflow is not mathematical arithmetic.

## Callables

| Fact | Header | RDL | Metadata | Bindgen |
| --- | --- | --- | --- | --- |
| Inspect result | `_Check_return_`, `_Must_inspect_result_` | `must_use` | `MustUse` | `#[must_use]` unless the wrapper returns `()`, `Result` or a type that is already `#[must_use]` |
| Never returns | `__declspec(noreturn)`, `[[noreturn]]` | `noreturn` | `DoesNotReturn` | `-> !` |
| Calling convention | Compiler ABI | `extern fn` (system), `extern "C" fn` | Signature and ImplMap | `extern "system"` or `extern "C"` |
| Import library | Import library evidence | `library("dll", import = "name")` | ImplMap `ImportScope`, `ImportName` | `link!` target |
| Last error | **Proposed** `_Win32_SetLastError_` | **Proposed** `library("dll", last_error)` | ImplMap flag `SupportsLastError` | - |
| Raw result | **Proposed** `_Win32_PreserveResult_` | **Proposed** `preserve_sig` | MethodImpl flag `PreserveSig` | - |
| Property accessor | MIDL `[propget]`, `[propput]` | `special` | MethodDef flag `SpecialName` | Accessor names |
| Vtable slot | C++ overrides of inherited slots | `override_slot(N)` | `NativeOverrideSlot` | Slot validation and overrides |
| Deprecation | `__declspec(deprecated)`, `[[deprecated]]` | `deprecated`, `deprecated("message")` | `Deprecated` | - |
| Architecture | Per-target parse results | `arch(X64 \| Arm64)` | `SupportedArchitecture` | Architecture gating |
| Minimum OS | **Proposed** `_Win32_SupportedOS_*_` | **Proposed** `supported_os("...")` | **Proposed** `SupportedOSPlatform` | - |

## Types and layout

| Fact | Header | RDL | Metadata | Bindgen |
| --- | --- | --- | --- | --- |
| Interface identity | `__declspec(uuid)`, `MIDL_INTERFACE` | `guid(0x...)` | `Windows.Foundation.Metadata.Guid` | Interface identity and GUID constants |
| Distinct typedef | Written typedef | `type Name = T;` | `NativeTypedef` | Typedef and handle types |
| Scoped enum | `enum class` | `scoped` | `ScopedEnum` | Enum representation and constant names |
| Flags enum | `[[clang::flag_enum]]`; declared `E operator\|(E, E)`, as from `DEFINE_ENUM_FLAG_OPERATORS` | `flags` | `System.Flags` | Bitwise operators |
| Constness | `const` pointee | `*const T` | `IsConst` modifier | `*const T` |
| Packing | Compiler layout | `packed(N)` | ClassLayout | `#[repr(C, packed(N))]` |
| Raised alignment | `__declspec(align)`, `alignas` | `align(N)` | `Alignment` | Layout, padding and underaligned fields |
| Bitfields | Bitfield members | Bitfield blocks | `NativeBitfield` | Accessors |
| Anonymous records | Unnamed nested members | Inline record or `anonymous(N)` | `NativeAnonymous` | Nested type names |
| Base storage | C++ base classes | `base` | `NativeBase` | Native object and interface bases |
| 64-bit pointer | `__ptr64` | `ptr64<*mut T>` | `NativePointer64` modifier | 64-bit pointer storage |
| Callback ABI | Function pointer typedef | `extern fn` (system), `extern "C" fn` | `System.Runtime.InteropServices.UnmanagedFunctionPointer` | Callback calling convention |

Flags come from compiler-observed declarations, not from macro, enum or constant names.
`NativeAnonymous` stores the native field position, which can differ from the metadata position
after bitfield lowering.

## Incomplete contracts

| Fact | Header | RDL | Metadata | Bindgen |
| --- | --- | --- | --- | --- |
| Incomplete contract | Parameter SAL without a canonical family: `_When_`, `_At_`, unsupported counts | `incomplete` | `Incomplete` | Raw pointer; blocks slice, `retval` and query projection, including slices counted by the parameter |

`_At_(*p, _Post_readable_size_(_Inexpressible_(...)))` and its byte form are the exception: they
publish `readable(success, unknown(), indirect)` and do not mark the parameter `incomplete`.

Other SAL is not published. This covers static-analysis directives such as `_IRQL_*_`,
`_Function_class_` and locking annotations, and SAL on owners without a family, such as `_In_` on a
field or `_Null_terminated_` on a typedef.

These parameter directives state no caller-visible contract and do not mark the parameter
`incomplete`:

| Header | Purpose |
| --- | --- |
| `__in_data_source`, `__out_data_source` | Taint tracking |
| `_Printf_format_string_`, `_Printf_format_string_params_` | Format string checking |
| `_Scanf_format_string_`, `_Scanf_s_format_string_` | Format string checking |
| `_IRQL_uses_cancel_` | Driver cancel-lock checking |
| `_Strict_type_match` | Argument type checking |
| `_In_range_`, `_Deref_in_range_`, `_Deref_out_range_`; `__RPC__in_range` | Value range checking |

`_Success_` and `_Return_type_success_` are not published either; a `return >= 0` condition only
grounds `retval` inference. Return and field ranges, such as `_Ret_range_` and `_Field_range_`, are
dropped with other SAL on owners without a family. Null-state and validity annotations are
contracts; when they cannot be published they still mark the parameter `incomplete`.
`__declspec(nothrow)` and `__declspec(allocator)` must agree across observations but are not
published.

## Bindgen boundary

Metadata never authorizes a projection on its own. Bindgen applies these rules:

| Metadata on a parameter | Bindgen |
| --- | --- |
| `BufferExtent` on `entry` with a constant or by-value input count used by no other buffer | Slice, with the count parameter hidden |
| Signed count on an `opt` buffer, byte count on wider elements, or output-only buffer | Raw pointer |
| Any other `BufferExtent` | Raw pointer, except interfaces; no `retval` projection |
| `readable(success, unknown(), indirect)` with `out_ptr` | Query projection still allowed |
| `incomplete` | Raw pointer, except interfaces; no slice, `retval` or query projection |
| `retval` | Rust result only when the parameter is also eligible; otherwise ignored |
| `opt` or `reserved` | `Option` for pointers, independently of any buffer contract |

## `_Win32_*` macros

**Proposed.** Macros expand to `annotate("Win32Metadata.Name", args...)` on the owning
declaration. Arguments are compiler-evaluated constants or declaration references; strings are
used only for names without a declaration, such as a DLL.

| Macro | Owner | Arguments | Checked against | RDL | Metadata |
| --- | --- | --- | --- | --- | --- |
| `_Win32_SetLastError_` | Function | - | - | `library(..., last_error)` | ImplMap flag `SupportsLastError` |
| `_Win32_PreserveResult_` | Function or method | - | - | `preserve_sig` | MethodImpl flag `PreserveSig` |
| `_Win32_ImportLibrary_` | Function | DLL; optional export name or ordinal | Import library evidence | `library("dll", import = "name")` | ImplMap `ImportScope`, `ImportName` |
| `_Win32_SupportedOS_*_` | Function, record, enum, typedef | - | `NTDDI_VERSION`, `_WIN32_WINNT` guards | `supported_os("...")` | `SupportedOSPlatform` |
| `_Win32_RAIIFree_` | Return or output parameter | Function; integer invalid values | Function declaration | `release(Function, invalid...)` | `Release` |
| `_Win32_Retained_` | Pointer parameter | - | - | `retained` | `Retained` |
| `_Win32_ContainsInteriorPointers_` | Output buffer parameter | - | Buffer extent present | `interior_pointers` | `InteriorPointers` |
| `_Win32_Enum_`, `_Win32_FlagsEnum_` | Typedef | Constants | Compiler-evaluated values | `enum`; `flags` for `_Win32_FlagsEnum_` | Enum TypeDef with Constant rows; `System.Flags` |
| `_Win32_AssociatedEnum_` | Parameter, return or field | Enum type | Integral native type | `associated_enum(Type)` | `AssociatedEnum` |
| `_Win32_AlsoUsableFor_` | Typedef | Typedef | Compatible native type | `also_usable_for(Type)` | `AlsoUsableFor` |

| Rule | Applies to |
| --- | --- |
| Each macro appears at most once per owner | All |
| Must agree with any version guard around the declaration | `_Win32_SupportedOS_*_` |
| Never retypes the owner | `_Win32_Enum_`, `_Win32_AssociatedEnum_` |
| Never scoped; the loose constants remain | `_Win32_Enum_`, `_Win32_FlagsEnum_` |

## Transport

| Source | Path into `windows-clang` |
| --- | --- |
| Compiler syntax | Cursor and type queries |
| SAL | `tool-win32` hooks `_SAL*_Source_` into `windows_clang:sal:` annotations |
| MIDL | Generated `/* [retval][out] */` comments and RPC SAL |
| `_Win32_*` in C/C++ headers | **Proposed** `annotate("Win32Metadata.Name", ...)` |
| `_Win32_*` in IDL | Open; `cpp_quote` comments naming one member |

## WinRT

WinRT contracts come from WinRT metadata, not headers. Where WinRT and Win32 describe the same fact
they share a representation: `Guid`, `System.Flags`, param flags and `SpecialName`. Bindgen also
reads these `Windows.Foundation.Metadata` attributes, which `windows-clang` never emits:

| Attribute | Bindgen |
| --- | --- |
| `Default` | Class default interface; RDL supplies it as the first interface |
| `Static` | Static factory interface |
| `Activatable` | Constructors and activation factories |
| `Composable` | Composition factories |
| `ExclusiveTo` | Class-owned interfaces and factory reachability |
| `MarshalingBehavior` | Agile types |
| `Overload` | Method names and overload filtering |
| `NoException` | Infallible wrapper signatures; unrelated to C++ `noexcept` |
| `ApiContract` | Omits contract marker types |

RDL accepts other declared attributes in named form, but the writer drops `System.*` attributes
without a dedicated spelling, and bindgen ignores names it does not know.

## Open questions

| Question | Options |
| --- | --- |
| `#define` constants reach `annotate` as integer literals without their names | Read the macro expansion location, or require enum constants |
| `AssociatedEnum` may name an enum declared later | Require a visible declaration, or resolve a checked name after merging |
| `Release` needs several invalid values | Array arguments in RDL, or one attribute per value |
| `ComOutPtr` does not name its IID parameter | Add the index, or keep the positional convention |
| `Retained`, `InteriorPointers`, `AlsoUsableFor` have no bindgen consumer | Publish for other consumers, or defer |
| `PreserveSig` projection | Return the native `HRESULT`, or keep it as metadata only |

## Implementation

| Stage | Source |
| --- | --- |
| SAL hooks | [`tool-win32` sal.h](../crates/tools/win32/src/sal.h) |
| Publication | [annotations.rs](../crates/libs/clang/src/projection/annotations.rs) |
| Metadata seed | [metadata.rdl](../metadata/metadata.rdl) |
| RDL short forms | [lib.rs](../crates/libs/rdl/src/lib.rs), [RDL reference](../crates/libs/rdl/rdl.md) |
| Decoding | [metadata reader](../crates/libs/metadata/src/reader) |
| Projection | [param.rs](../crates/libs/bindgen/src/param.rs), [types](../crates/libs/bindgen/src/types) |
| Fixtures | [test_clang input](../crates/tests/libs/clang/input) |

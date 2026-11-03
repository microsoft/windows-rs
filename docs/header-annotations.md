# Header annotations

Win32 headers describe API contracts with C/C++, SAL and MIDL. `windows-clang` reads them and
publishes RDL, `windows-rdl` encodes that RDL as ECMA-335 metadata, and `windows-bindgen` projects
the metadata into Rust. This catalog gives representative header spellings, RDL examples,
[metadata encodings](#metadata-encoding) and consumer behavior.

The parameter, buffer, callable and layout tables describe implemented mappings. `-` means bindgen
does not interpret the fact. Rust examples describe wrappers, not the raw ABI.

The [`_Win32_*` proposals](#proposed-annotations) are design candidates, not supported syntax
or an agreement with microsoft/win32metadata#2365. `windows-clang` currently rejects their
`annotate` payloads. [C# consumption](#c-consumption) is a separate integration boundary.

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
    #[buffer(count)] #[in] values: *const i32,
    #[in] count: u32,
    #[buffer(bytes(*size))] #[opt] #[out] buffer: *mut void,
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

`values` becomes a slice because its count is a by-value input parameter. `buffer` stays a pointer
because its count is read through `*size`. `_Success_` is not published. `must_use` adds nothing
here because `BOOL` is already `#[must_use]`. The DLL comes from import-library evidence, not SAL.

## Principles

| Rule | Meaning |
| --- | --- |
| Existing syntax first | C/C++, SAL and MIDL own every fact they can express; `_Win32_*` macros cover only the rest |
| Fix the header | Wrong or missing SAL is fixed in the header, never overridden downstream |
| One encoding | Equivalent header spellings share one RDL form and metadata encoding; prefer ECMA-335 structure over custom attributes |
| Checked | Facts the compiler can confirm must agree with it, or publication fails |
| Native types kept | Annotations add facts beside the native type and never retype it |
| Actionable | A new fact needs a concrete consumer use, in Rust, C# or another projection |
| Absent means unspecified | A missing fact promises nothing; `incomplete` marks unmodeled parameter contracts, not every omitted SAL annotation |
| Contracts, not projections | Metadata states facts; bindgen decides what Rust to generate |

## Parameters

| Fact | Header | RDL | Rust |
| --- | --- | --- | --- |
| Direction | `_In_ DWORD flags`; `_Out_`, `_Inout_` | `#[in] flags: u32`; `#[out]`, both | Input or output projection |
| Optional | `_In_opt_ const RECT* bounds` | `#[in] #[opt] bounds: *const RECT` | `Option<*const RECT>` |
| Reserved | `_Reserved_ void* reserved` | `#[reserved] reserved: *mut void` | Same as optional; the value is not enforced |
| Result | MIDL `[out, retval] BSTR* name` | `#[out] #[retval] name: *mut BSTR` | Trailing result of an `HRESULT` method becomes `Result<BSTR>` |
| Returned pointer | `_Outptr_ void** data` | `#[out] #[out_ptr] data: *mut *mut void` | Implementations clear the output on error when `null_on_failure` |
| Nullable | `_Outptr_result_maybenull_ IThing** thing` | `#[nullable] #[out] #[out_ptr] thing: *mut IThing` | `Option<IThing>` if otherwise eligible as an interface result |
| Nullable return | `_Ret_maybenull_ void* Find()` | `extern fn Find() -> #[nullable] *mut void;` | Raw pointer |
| Nullable field | `_Maybenull_ int* values;` | `#[nullable] values: *mut i32` | Raw pointer |
| COM query | `_In_ REFIID riid, _COM_Outptr_ void** object` | `#[in] riid: *const GUID, #[iid_is] #[out] #[out_ptr(null_on_failure)] object: *mut *mut void` | An `HRESULT` method can become `Get<T: Interface>() -> Result<T>` |

`opt` lets the caller pass null. `nullable` says the API may produce null: through an `out`
parameter, as a return value, or in a field. Without `nullable`, `out_ptr` promises a non-null
pointer to one readable element on success; it does not imply an allocator or ownership transfer.

`_Outptr_opt_` permits null argument storage, not a null returned pointer. A required non-null
trailing typed `_COM_Outptr_`, or `_Outptr_` to an interface, also gets `retval` when success is
`return >= 0`. A nullable result does not qualify for that inference.

MIDL facts are read from generated bracket comments such as `/* [retval][out] */`. `iid_is` is a
marker, not a named relationship: bindgen selects the last eligible `*const GUID` input. It cannot
recover MIDL's original IID parameter name.

## Buffers and strings

### `buffer`: caller-supplied extent

`#[buffer(count)]` records the buffer extent supplied when the call starts. Direction stays in
`in`/`out`; the extent promises neither ownership nor an initialized output length.

| Header | RDL | Rust |
| --- | --- | --- |
| `_In_reads_(count) const int* values` | `#[buffer(count)] #[in] values: *const i32` | `&[i32]` with an eligible by-value input count |
| `_Inout_updates_(count) int* values` | `#[buffer(count)] #[in] #[out] values: *mut i32` | `&mut [i32]` with an eligible count |
| `_Out_writes_to_(capacity, *written) int* values` | `#[buffer(capacity)] #[out] values: *mut i32` | Raw pointer; the produced-length relationship is not published |
| `_Out_writes_bytes_(*size) void* data` | `#[buffer(bytes(*size))] #[out] data: *mut void` | Raw pointer |

Counts are an unsuffixed nonnegative integer, an integer parameter, or one dereference of a
non-optional integer-pointer parameter. `bytes(count)` selects bytes instead of elements. There are
no phase arguments, arithmetic expressions, field counts, returned-buffer counts or fallbacks.
Unsupported entry counts mark the parameter `incomplete`. A retained entry extent already prevents
scalar-result and COM-query transformations; omitting its produced length needs no extra marker.
When no entry extent can be published, `incomplete` preserves that buffer guard.

Clang still captures and reconciles native capacities, initialized lengths, units and conditional
counts. RDL/WinMD publishes only the entry extent. Produced-length metadata can be added when a
consumer needs it, without rebuilding SAL capture. A Rust initialized-prefix wrapper would also need
success, completion and value-validity rules; ordinary slice arguments do not require that metadata.

### `null_terminated`: trailing zero elements

This attribute describes termination, independently of buffer size and character encoding.
`entry` means the start of the call; `success` means successful return under the API's own success
condition. These arguments belong to termination, not to `buffer`.

| Header | RDL |
| --- | --- |
| `_In_z_ const char* name` | `#[in] #[null_terminated(entry)] name: *const i8` |
| `_Pre_ _NullNull_terminated_ const unsigned short* names` | `#[null_terminated(double_entry)] names: *const u16` |
| `_Outptr_result_maybenull_z_` | `#[null_terminated(success, indirect)] #[nullable] #[out] #[out_ptr]` |

`double_entry` and `double_success` require two trailing zeros at that time. `indirect` selects the
pointer stored through the parameter. Bindgen does not currently use this fact to change the Rust type.

### `encoding`: character encoding

`#[encoding("ansi")]` records narrow character encoding on a string constant or field. For example,
`#define NAME "text"` becomes `#[encoding("ansi")] const NAME: String = "text";`.
Bindgen uses it to select narrow string constants. It specifies neither termination nor capacity.

## Callables

| Fact | Header | RDL | Rust |
| --- | --- | --- | --- |
| Inspect result | `_Check_return_ int Parse(...)`; `_Must_inspect_result_` | `#[must_use]` | `#[must_use]` unless the wrapper returns `()` or a `#[must_use]` type |
| Never returns | `__declspec(noreturn) void Fail()`; `[[noreturn]]` | `#[noreturn]` | `-> !` |
| Calling convention | `void __cdecl Log()` | `extern "C" fn Log()`; `extern fn` is system | `extern "C"` or `extern "system"` |
| Import library | Import library maps `Read` to `ReadEntry` | `#[library("example.dll", import = "ReadEntry")]` | `link!` target |
| Property accessor | MIDL `[propget] HRESULT Name(...)` | `#[special]` | Accessor names |
| Vtable slot | C++ override of an inherited virtual | `#[override_slot(3)]` | Slot validation |
| Deprecation | `[[deprecated("use Next")]]` | `#[deprecated("use Next")]` | - |
| Architecture | Declared only for x64 and Arm64 | `#[arch(X64 \| Arm64)]` | `cfg` gating |

## Types and layout

| Fact | Header | RDL | Rust |
| --- | --- | --- | --- |
| Interface identity | `MIDL_INTERFACE("12345678-...") IThing` | `#[guid(0x12345678_...)] interface IThing` | Interface identity and GUID constants |
| Distinct typedef | `typedef void* HTHING;` | `type HTHING = *mut void;` | Typedef and handle types |
| Scoped enum | `enum class Mode` | `#[scoped] enum Mode` | Enum representation and constant names |
| Flags enum | `[[clang::flag_enum]]` or `DEFINE_ENUM_FLAG_OPERATORS(Mode)` | `#[flags] enum Mode` | Bitwise operators |
| Constness | `const int* values` | `*const i32` | `*const i32` |
| Packing | `#pragma pack(1)` | `#[packed(1)] struct Packed` | `#[repr(C, packed(1))]` |
| Raised alignment | `alignas(32)` | `#[align(32)]` | Layout and padding |
| Bitfields | `unsigned int ready : 1;` | `_bitfield: u32 { ready: bool: 1 }` | Accessors |
| Anonymous records | `union { int a; float b; };` | Inline record or `#[anonymous(1)]` | Nested type names |
| Base storage | `struct Derived : Base` | `#[base]` | Native object and interface bases |
| 64-bit pointer | `int* __ptr64 address` | `ptr64<*mut i32>` | 64-bit pointer storage |
| Callback ABI | `typedef void (CALLBACK* NOTIFY)(void*);` | `extern fn NOTIFY(context: *mut void);` | Callback calling convention |

Flags come from compiler-observed declarations, not from macro, enum or constant names.

## Bindgen boundary

Metadata facts do not by themselves authorize a wrapper transformation:

| Metadata | Current bindgen behavior |
| --- | --- |
| Constant element count or unshared by-value input count | Slice or fixed-size array reference when direction and element type allow it; hide a variable count |
| Constant byte count, signed count on an optional buffer, byte count on wider elements, or output-only buffer | Raw pointer |
| Other buffer counts | Raw pointer, except interface values; no `retval` projection |
| `incomplete` on a buffer, count, query argument or result | Blocks the corresponding slice, query or `retval` transformation |
| `retval` | Trailing output-only, non-optional, non-reserved pointer with no extent or `incomplete`; `HRESULT` becomes `Result<T>`, `void` becomes `T` |
| `opt` or `reserved` | Optional pointer/reference/slice where the parameter projection supports it |

Unspecified direction stays unspecified in metadata. Bindgen can still treat native types that
expose no caller-writable storage as inputs. `HRESULT` calls without `retval` or query projection
already return the raw status; COM implementation methods use `Result`.

## Proposed annotations

All rows below are **Proposed**. The RDL spellings, new metadata families and consumer effects need
implementation unless they reuse an existing mapping. These are additions to SAL/MIDL, not
replacements for them.

| Fact | Header example | Proposed RDL | Intended consumer use |
| --- | --- | --- | --- |
| Import override | `_Win32_ImportLibrary_("example.dll", "ReadEntry")` | `#[library("example.dll", import = "ReadEntry")]` (existing) | Rust link target; C# DLL and entry point |
| Last error | `_Win32_SetLastError_` | `#[library("kernel32.dll", last_error)]` | C# `SetLastError = true`; not an error predicate |
| Exact result | `_Win32_PreserveResult_ HRESULT EnumAdapters1(...)` | `#[preserve_result]` | Keep raw status and output parameters instead of Rust `Result` or C# exception/result conversion |
| Minimum OS | `_Win32_SupportedOS_19041_` | `#[supported_os("windows10.0.19041")]` | C# platform diagnostics |
| Release | `_Out_ _Win32_RAIIFree_(WinHttpCloseHandle, 0) HINTERNET* session` | `#[invalid(0)] #[out] #[release(WinHttpCloseHandle)] session: *mut HINTERNET` | Rust owned handles or C# `SafeHandle`; requires an ownership-aware wrapper |
| Retained | `_In_ _Win32_Retained_ OVERLAPPED* overlapped` | `#[in] #[retained] overlapped: *mut OVERLAPPED` | No call-scoped slice/reference/span wrapper; caller manages lifetime |
| Interior pointers | `_Out_writes_bytes_opt_(cbBuf) _Win32_ContainsInteriorPointers_ BYTE* info` | `#[buffer(bytes(cbBuf))] #[interior_pointers] #[opt] #[out] info: *mut u8` | Keep caller-owned storage alive and in place while interior pointers are used |
| Logical enum | `_Win32_Enum_(FILE_CREATION_DISPOSITION, DWORD, CREATE_NEW, OPEN_EXISTING);` | `#[repr(u32)] enum FILE_CREATION_DISPOSITION { CREATE_NEW = 1, OPEN_EXISTING = 3 }` | Enum type and constants |
| Logical flags | `_Win32_FlagsEnum_(DXGI_MAP_FLAGS, UINT, DXGI_MAP_READ, DXGI_MAP_WRITE);` | `#[flags] #[repr(u32)] enum DXGI_MAP_FLAGS { ... }` | Bitwise operators |
| Enum use | `_Win32_AssociatedEnum_(FILE_CREATION_DISPOSITION) _In_ DWORD disposition` | `#[associated_enum(FILE_CREATION_DISPOSITION)] #[in] disposition: u32` | Typed wrapper parameter in Rust or C#; metadata keeps native storage |
| Compatible typedef | `typedef _Win32_AlsoUsableFor_(HGDIOBJ) HANDLE HBITMAP;` | `#[also_usable_for(HGDIOBJ)] type HBITMAP = HANDLE;` | Directional handle conversion, not ownership transfer |

The proposed contracts require these checks:

| Contract | Rule |
| --- | --- |
| Import override | Checked against the native symbol and target; replaces an explicit entry in [import_policy.rs](../crates/tools/win32/src/import_policy.rs), not native ABI evidence |
| Release | Resolve a declared cleanup function with a compatible parameter; invalid values are integer constants, one `invalid` each. `_Win32_RAIIFree_Handle_(F)` supplies `-1` and `0` |
| Logical enum names | Member names come from the macro arguments and values from the compiler; members replace the loose constants of the same name |
| Enum use | Resolve the name after the whole capture; require exactly one enum with compatible storage |
| Retained | The API uses the pointer after return; this does not describe when that use ends |
| Minimum OS | Use platform version numbers, not marketing names; availability guards need separate evidence before becoming minimum-version claims |

`preserve_result` would keep status codes such as `S_FALSE` or the failure code
`DXGI_ERROR_NOT_FOUND` used to end enumeration. This is projection policy, not native ABI. C#'s
[`DllImport.PreserveSig`](https://learn.microsoft.com/dotnet/api/system.runtime.interopservices.dllimportattribute.preservesig)
defaults to `true` but can be `false`.

### Transport

The proposed transport is `__attribute__((annotate("Win32Metadata.Name", args...)))` on the
declaration it describes. Each macro needs an owner and argument schema; unknown names, bad
arguments, wrong owners and duplicates should fail. Values are compiler-evaluated and declaration
names must resolve. The PR's mixed string and proxy-declaration encodings do not yet meet this design.

`_Win32_SupportedOS_BEGIN_(os)` and `_Win32_SupportedOS_END_()` are region forms in the PR.
Their Clang pragma covers functions, records, enums and typedefs, including included declarations;
nested annotations accumulate rather than override.

In IDL, a macro goes in MIDL's `annotation` attribute on the method, parameter or field, which MIDL
copies into the generated header:

```idl
[annotation("_Win32_PreserveResult_")]
HRESULT EnumAdapters1([in] UINT index, [out, annotation("_COM_Outptr_")] IDXGIAdapter1** adapter);
```

The generated header must see the macro definitions. Passthrough is not payload support.

### Forms not adopted

These forms from microsoft/win32metadata#2365 duplicate existing facts or add a second transport:

| Form | Use instead |
| --- | --- |
| `_Win32_Optional_` | `_opt_` SAL forms, MIDL `[optional]` |
| `_Win32_Reserved_` | `_Reserved_` |
| `_Win32_ComOutPtr_` | `_COM_Outptr_`, MIDL `[iid_is]` |
| `_Win32_Retval_` | MIDL `[retval]` |
| `_Win32_Flags_` | `[[clang::flag_enum]]` |
| `_Win32_AssociatedConstant_` | A constant belongs to one enum or none |
| `/* Win32Metadata: ... */` comments | MIDL `annotation` |

Unknown `annotate` payloads are rejected today. Arbitrary comments are not directives and are
ignored; only the recognized MIDL bracket comments carry contracts.

## C# consumption

DLL/entry-point and calling-convention facts can populate C# P/Invoke declarations.
`SetLastError = true` asks the runtime to capture the native error, not decide whether a call failed.
Custom families such as `Buffer`, `ReturnedPointer`, `Nullable` and `Incomplete` require
explicit generator support. This repository does not establish C# support for this vocabulary;
metadata round trips are not a substitute for consumer and generated-code coverage.

## Not published

| Header | Treatment |
| --- | --- |
| Parameter SAL without a family: `_When_`, `_At_`, unsupported counts | `#[incomplete]`: raw pointer, no slice, `retval` or query projection |
| Produced lengths and post-call capacities | Validated natively, not published; keep a usable entry extent or mark the buffer `incomplete` |
| `_Field_size_*` | Validated natively; extent omitted, nullability retained |
| `_At_(*p, _Post_readable_size_(_Inexpressible_(...)))` | Redundant with an explicit `out_ptr`; otherwise `incomplete` |
| Null-state and validity SAL that cannot be published | `#[incomplete]` |
| `_Success_`, `_Return_type_success_` | Dropped; `return >= 0` only grounds `retval` inference |
| `_In_range_`, `_Deref_in_range_`, `_Deref_out_range_`, `__RPC__in_range` | Dropped; ranges constrain values, not projections |
| `__in_data_source`, `_Printf_format_string_`, `_Scanf_format_string_`, `_IRQL_uses_cancel_`, `_Strict_type_match` | Dropped; static analysis only |
| `_IRQL_*_`, `_Function_class_`, locking SAL, SAL on owners without a family | Dropped |
| `__declspec(nothrow)`, `__declspec(allocator)` | Must agree across observations; not published |

## Metadata encoding

Attributes are in `Windows.Win32.Metadata` unless qualified, written without the `Attribute`
suffix. `Native` attributes preserve C/C++ declaration details; unprefixed attributes state
language-neutral API facts.

| RDL | Metadata |
| --- | --- |
| `in`, `out`, `opt` | Param flags `In`, `Out`, `Optional` |
| `reserved`, `retval`, `nullable`, `incomplete` | `Reserved`, `RetVal`, `Nullable`, `Incomplete` |
| `out_ptr`, `out_ptr(null_on_failure)` | `ReturnedPointer`, with `NullOnFailure` |
| `iid_is` | `ComOutPtr` |
| `buffer(count)`, `buffer(bytes(count))` | `Buffer`, with `CountConst` or `CountParamIndex`, plus `Dereference` and `Bytes` when needed |
| `null_terminated`, `encoding` | `NullTerminated`, `Encoding` |
| `must_use`, `noreturn`, `deprecated` | `MustUse`, `DoesNotReturn`, `Deprecated` |
| `library("dll", import = "name")` | ImplMap `ImportScope` and `ImportName` |
| `extern fn`, `extern "C" fn` | ImplMap calling-convention flags on imports; `System.Runtime.InteropServices.UnmanagedFunctionPointer` on named callbacks |
| `special` | MethodDef flag `SpecialName` |
| `override_slot(N)`, `base` | `NativeOverrideSlot` and cleared `NewSlot`; `NativeBase` |
| `arch(...)` | `SupportedArchitecture` |
| `guid(...)` | `Windows.Foundation.Metadata.Guid` |
| `type Name = T;` | `NativeTypedef` |
| `scoped`, `flags` | `ScopedEnum`, `System.Flags` |
| `*const T`, `ptr64<T>` | `System.Runtime.CompilerServices.IsConst` and `NativePointer64` modifiers |
| `packed(N)`, `align(N)` | ClassLayout, `Alignment` |
| Bitfield blocks, `anonymous(N)` | `NativeBitfield`, `NativeAnonymous` |
| **Proposed** `library(..., last_error)` | ImplMap flag `SupportsLastError` |
| **Proposed** `preserve_result`, `retained`, `interior_pointers` | `PreserveResult`, `Retained`, `InteriorPointers` |
| **Proposed** `release(F)`, `invalid(N)` | `Release`, `InvalidValue` |
| **Proposed** `associated_enum(T)`, `also_usable_for(T)` | `AssociatedEnum`, `AlsoUsableFor` |
| **Proposed** `supported_os("...")` | `System.Runtime.Versioning.SupportedOSPlatform` |

`NativeAnonymous` stores the native field position, which can differ from the metadata position
after bitfield lowering.

The [metadata seed](../metadata/metadata.rdl) defines implemented custom attributes.
[BufferInfo](../crates/libs/metadata/src/buffer_info.rs) defines the buffer count payload.

## Extending the catalog

For each addition, define its header source, owner, arguments, RDL/WinMD encoding and intended
consumer action. Include absence, conflicts and unsupported cases. Keep it **Proposed** until
capture, round-trip and relevant projection coverage exist; record consumers separately.

| Stage | Implementation and coverage |
| --- | --- |
| SAL hooks | [`tool-win32` sal.h](../crates/tools/win32/src/sal.h) turns `_SAL*_Source_` into `windows_clang:sal:` annotations |
| Publication | [annotations.rs](../crates/libs/clang/src/projection/annotations.rs); [header fixtures](../crates/tests/libs/clang/input/fixtures) run on X86, X64 and Arm64 |
| RDL short forms | [lib.rs](../crates/libs/rdl/src/lib.rs), [RDL reference](../crates/libs/rdl/rdl.md) |
| Decoding | [metadata reader](../crates/libs/metadata/src/reader) |
| Rust projection | [param.rs](../crates/libs/bindgen/src/param.rs), [cpp_method.rs](../crates/libs/bindgen/src/types/cpp_method.rs); [bindgen fixtures](../crates/tests/libs/bindgen/input) |

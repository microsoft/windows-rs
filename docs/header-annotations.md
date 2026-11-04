# Header annotations

Win32 headers describe API contracts with C/C++, SAL and MIDL. `windows-clang` reads them and
publishes RDL, `windows-rdl` encodes that RDL as ECMA-335 metadata, and `windows-bindgen` projects
the metadata into Rust. This catalog gives representative header spellings, RDL examples,
[metadata encodings](#metadata-encoding) and consumer behavior.

The parameter, buffer, callable and layout tables describe implemented mappings. `-` means bindgen
does not interpret the fact. Rust examples describe wrappers, not the raw ABI.

`_Win32_SetLastError_`, `_Win32_ImportLibrary_`, `_Win32_Enum_`, `_Win32_FlagsEnum_` and
`_Win32_AssociatedEnum_` are supported. The remaining
[`_Win32_*` proposals](#proposed-annotations) are design candidates, not supported syntax or an
agreement with microsoft/win32metadata#2365.
`windows-clang` rejects unknown `annotate` payloads.
[C# consumption](#c-consumption) is a separate integration boundary.

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
    values: *const i32,
    count: u32,
    buffer: Option<*mut core::ffi::c_void>,
    size: *mut u32,
    reserved: Option<*mut core::ffi::c_void>,
) -> BOOL
```

`values` and `buffer` stay pointers with separate counts. `_Success_` is not published. `must_use`
adds nothing here because `BOOL` is already `#[must_use]`. The DLL comes from import-library evidence,
not SAL.

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
| Pointer result | `_Outptr_ void** data` | `#[out] data: *mut *mut void` | Raw pointer unless eligible as a result |
| COM query | `_In_ REFIID riid, _COM_Outptr_ void** object` | `#[in] riid: *const GUID, #[iid_is] #[out] object: *mut *mut void` | An `HRESULT` method can become `Get<T: Interface>() -> Result<T>` |

`opt` lets the caller pass null, including omitting an output slot. It does not describe the value
written through that slot, an allocator or ownership transfer.

Clang keeps native returned-pointer, nullability and null-on-failure contracts for checking and
reconciliation, but does not publish them as separate attributes.

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
| `_In_reads_(count) const int* values` | `#[buffer(count)] #[in] values: *const i32` | Raw pointer and separate count |
| `_Inout_updates_(count) int* values` | `#[buffer(count)] #[in] #[out] values: *mut i32` | Raw pointer and separate count |
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
success, completion and value-validity rules.

Rust bindgen preserves native buffer pointers and counts rather than combining them into slices.
Callers supply both according to the API's contract, preserving any distinction between null and an
empty buffer. API-specific wrappers can expose slices when the native contract permits it. WinRT
array projection is separate and still uses slices.

### `null_terminated` and `null_null_terminated`: trailing zero elements

These argument-free markers describe the buffer representation, independently of encoding, capacity,
optionality and success conditions.

| Attribute | Meaning |
| --- | --- |
| `#[null_terminated]` | A buffer terminated by one zero element |
| `#[null_null_terminated]` | A buffer terminated by two consecutive zero elements, typically a packed string list |

| Header | RDL |
| --- | --- |
| `_In_z_ const char* name` | `#[in] #[null_terminated] name: *const i8` |
| `_Pre_ _NullNull_terminated_ const unsigned short* names` | `#[null_null_terminated] names: *const u16` |

The double-zero form stands alone. Both markers describe the parameter's direct buffer, not a pointer
stored through it. Neither guarantees initialized output after failure. Mixed single/double
termination across the call and indirect termination remain native facts; their parameters are marked
`incomplete` instead of publishing an ambiguous marker.

Clang retains native phases for validation and reconciliation. Rust bindgen does not currently consume
either marker. CsWin32 uses `NullNullTerminatedAttribute` to select multi-string pointer helpers rather
than ordinary string helpers.

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
| Import override | `_Win32_ImportLibrary_("example.dll", "ReadEntry")` | `#[library("example.dll", import = "ReadEntry")]` | `link!` target for named exports |
| Last error | `_Win32_SetLastError_` on a free function | `#[library("example.dll", last_error)]` | No change |
| Property accessor | MIDL `[propget] HRESULT Name(...)` | `#[special]` | Accessor names |
| Vtable slot | C++ override of an inherited virtual | `#[override_slot(3)]` | Slot validation |
| Deprecation | `[[deprecated("use Next")]]` | `#[deprecated("use Next")]` | - |
| Architecture | Declared only for x64 and Arm64 | `#[arch(X64 \| Arm64)]` | `cfg` gating |

`_Win32_SetLastError_` expands to `__attribute__((annotate("Win32Metadata.SetLastError")))`
with the upstream helper and `WIN32METADATA` enabled. It accepts no arguments and belongs on a
free function, not a method, callback, type or parameter. Duplicate markers fail, and its presence
must agree across native observations. It does not supply DLL ownership.

The marker requests last-error capture by consumers such as C# P/Invoke. It does not specify when
the function fails. Rust signatures and error handling are unchanged.

`_Win32_ImportLibrary_` is authoritative DLL/export evidence on a free function. It overrides
caller-supplied mappings and scanned import libraries, and can supply ownership when neither is
available. Its DLL argument is an ordinary narrow string literal, not a path. An optional second
argument names the export or supplies an integral constant ordinal from 1 through 65535.
Strings are read from the compiler AST; ordinals are compiler-evaluated. Empty strings, control
characters, embedded nulls, invalid owners and duplicate annotations fail.

Omitting the entry point preserves the normally resolved export, or the native declaration name
when no mapping exists. Import overrides do not rename the declaration or establish a calling
convention. Native ABI checks and target-specific convention evidence still apply. Conflicting
annotations fail reconciliation before publication.

Ordinals encode as `#[library("example.dll", import = "#42")]` and survive RDL/WinMD round trips.
Bindgen does not support ordinal linking and excludes these imports from generated Rust bindings.

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

### `_Win32_Enum_` and `_Win32_FlagsEnum_`: constant groups

These macros declare a metadata enum with explicit integer storage. `_Win32_FlagsEnum_` also marks
the group as flags; neither macro guesses membership from names or values.

| Header | RDL | Rust |
| --- | --- | --- |
| `_Win32_Enum_(ADDRESS_FAMILY, USHORT, AF_UNSPEC, AF_INET, AF_INET6);` | `#[repr(u16)] enum ADDRESS_FAMILY { AF_UNSPEC = 0, AF_INET = 2, AF_INET6 = 23 }` | `type ADDRESS_FAMILY = u16;` and typed global constants |
| `_Win32_FlagsEnum_(DUPLICATE_HANDLE_OPTIONS, DWORD, DUPLICATE_CLOSE_SOURCE, DUPLICATE_SAME_ACCESS);` | `#[flags] #[repr(u32)] enum DUPLICATE_HANDLE_OPTIONS { DUPLICATE_CLOSE_SOURCE = 1, DUPLICATE_SAME_ACCESS = 2 }` | Constants can be combined and passed to a `u32` parameter without casts |

Member names are distinct unqualified identifiers from the macro arguments. The compiler supplies
their integer types and values. Every value must fit the declared storage without truncation or
wrapping. For example, `-1` cannot enter a `u32` group; an explicitly converted native constant such
as `#define ALL_OPTIONS ((DWORD)-1)` can. Unrelated constants keep their native types.

A group can annotate an existing compatible typedef, as with `ADDRESS_FAMILY`. Members replace
loose constants of the same name after checking their native definitions. Conflicting group
declarations, overlapping membership and regrouping native enum members fail.
Native constant declarations must be at namespace scope; class-member constants are unsupported.

### `associated_enum`: enum use beside native storage

`_Win32_AssociatedEnum_(ADDRESS_FAMILY) int family` becomes
`#[associated_enum(ADDRESS_FAMILY)] family: i32`. The annotation belongs on an integer parameter,
return value or field, including integer storage reached through pointers or arrays. It names a
logical group or an existing native enum, not a replacement ABI type.

Names resolve in enclosing namespaces or through explicit namespace qualification.
`_Win32_AssociatedEnum_(enum ResourceType)` names an enum tag. Lookup through C++ `using` declarations,
class-scoped lookup, bitfields and already enum-typed uses are unsupported. Use a globally qualified
name in scopes with `using` directives. Missing, ambiguous and non-enum targets fail.

Bindgen retains the associated enum and its constants when selecting the annotated API, but leaves
its native signature unchanged. An `ADDRESS_FAMILY` constant is `u16`; a socket API taking `i32`
still needs a conversion. Signed values keep signed conversion behavior: `-100_i16 as u32` is
`0xFFFFFF9C`, not the `0x0000FF9C` obtained by converting through `u16`.

## Bindgen boundary

Metadata facts do not by themselves authorize a wrapper transformation:

| Metadata | Current bindgen behavior |
| --- | --- |
| `buffer` | Keep native pointers and separate counts; block scalar-result and COM-query transformations on the buffer |
| `incomplete` | Keep raw parameter storage, except borrowed interface values; block query or `retval` transformations on the parameter |
| `retval` | Trailing output-only, non-optional, non-reserved pointer with no extent or `incomplete`; `HRESULT` becomes `Result<T>`, `void` becomes `T` |
| `iid_is` | Output-only `void**` paired with an input-only `GUID*`, neither with an extent or `incomplete`; an `HRESULT` query can project to `Result<T>` without `retval` |
| `opt` or `reserved` | Optional pointer/reference where the parameter projection supports it |

Unspecified direction stays unspecified in metadata. Bindgen can still treat native types that
expose no caller-writable storage as inputs. Without eligible `retval` or query projection, both
callers and native interface implementation methods return `HRESULT` and keep explicit output
parameters. Implementations can return `S_FALSE` or write outputs on failure without result conversion.

An eligible `iid_is` query acts as an implicit result. Callers get `Result<T>`, or `Result<()>` with
explicit storage when that storage is optional. Implementations keep the IID and output pointer
and return `Result<()>`, since the requested interface type is selected at runtime. In/out pointers,
buffers and incomplete contracts do not qualify. WinRT result projection is unchanged.

Interface results lifted from output parameters use `Result<T>`, including COM queries. A successful
call that supplies a null interface becomes `Err(Error::empty())`, whose code is `S_OK`. Direct
interface returns and interface output slots can still use `Option<T>` based on their type, without
nullability metadata. Raw pointer results remain raw pointers.

Generated implementations zero the ABI result storage when the Rust implementation returns `Err`.
This applies to projected Win32 `retval` results and WinRT return values, including both the pointer
and length of returned arrays. It happens after the call so inputs can alias the result storage.
Explicit output parameters, in/out values and raw buffers remain implementation-managed. This is
producer policy, not metadata or a guarantee about external implementations.

## Proposed annotations

All rows below are **Proposed**. The RDL spellings, new metadata families and consumer effects need
implementation unless they reuse an existing mapping. These are additions to SAL/MIDL, not
replacements for them.

| Fact | Header example | Proposed RDL | Intended consumer use |
| --- | --- | --- | --- |
| Exact result | `_Win32_PreserveResult_ HRESULT EnumAdapters1(...)` | `#[preserve_result]` | Keep raw status and output parameters instead of Rust `Result` or C# exception/result conversion |
| Minimum OS | `_Win32_SupportedOS_19041_` | `#[supported_os("windows10.0.19041")]` | C# platform diagnostics |
| Release | `_Out_ _Win32_RAIIFree_(WinHttpCloseHandle, 0) HINTERNET* session` | `#[invalid(0)] #[out] #[release(WinHttpCloseHandle)] session: *mut HINTERNET` | Rust owned handles or C# `SafeHandle`; requires an ownership-aware wrapper |
| Retained | `_In_ _Win32_Retained_ OVERLAPPED* overlapped` | `#[in] #[retained] overlapped: *mut OVERLAPPED` | No call-scoped slice/reference/span wrapper; caller manages lifetime |
| Interior pointers | `_Out_writes_bytes_opt_(cbBuf) _Win32_ContainsInteriorPointers_ BYTE* info` | `#[buffer(bytes(cbBuf))] #[interior_pointers] #[opt] #[out] info: *mut u8` | Keep caller-owned storage alive and in place while interior pointers are used |
| Compatible typedef | `typedef _Win32_AlsoUsableFor_(HGDIOBJ) HANDLE HBITMAP;` | `#[also_usable_for(HGDIOBJ)] type HBITMAP = HANDLE;` | Directional handle conversion, not ownership transfer |

The proposed contracts require these checks:

| Contract | Rule |
| --- | --- |
| Release | Resolve a declared cleanup function with a compatible parameter; invalid values are integer constants, one `invalid` each. `_Win32_RAIIFree_Handle_(F)` supplies `-1` and `0` |
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
names must resolve. Each remaining annotation family needs its own validated argument schema.

`_Win32_SupportedOS_BEGIN_(os)` and `_Win32_SupportedOS_END_()` are region forms in the PR.
Their Clang pragma covers functions, records, enums and typedefs, including included declarations;
nested annotations accumulate rather than override.

MIDL's `annotation("...")` replaces generated SAL rather than adding to it. It is not a general
additive transport for these macros. The upstream helpers use `cpp_quote` to emit annotated
metadata-only declarations that identify a method, parameter or field. Those carriers are not
implemented here.

### Other unimplemented forms

These forms from microsoft/win32metadata#2365 need separate contract or transport decisions.
Related SAL/MIDL forms do not establish that the custom forms are interchangeable or unnecessary.

| Form | Current position |
| --- | --- |
| `_Win32_Optional_` | `_opt_` SAL forms, MIDL `[optional]` |
| `_Win32_Reserved_` | `_Reserved_` |
| `_Win32_ComOutPtr_` | `_COM_Outptr_`, MIDL `[iid_is]` |
| `_Win32_Retval_` | MIDL `[retval]` is supported; hand-written C++ needs a separate spelling |
| `_Win32_Flags_` | `[[clang::flag_enum]]` is supported but also affects compiler diagnostics |
| `_Win32_AssociatedConstant_` | No published association between a global constant and multiple enum groups |
| `_Win32_AnnotateMethod_`, `_Win32_AnnotateParameter_`, `_Win32_AnnotateField_` | Metadata-only declaration carriers from IDL are not implemented |

Unknown `annotate` payloads are rejected today. Arbitrary comments are not directives and are
ignored; only the recognized MIDL bracket comments carry contracts.

## C# consumption

DLL/entry-point and calling-convention facts can populate C# P/Invoke declarations.
`SetLastError = true` asks the runtime to capture the native error, not decide whether a call failed.
Custom families such as `AssociatedEnum`, `Buffer` and `Incomplete` require
explicit generator support. This repository does not establish C# support for this vocabulary;
metadata round trips are not a substitute for consumer and generated-code coverage.

## Not published

| Header | Treatment |
| --- | --- |
| Parameter SAL without a family: `_When_`, `_At_`, unsupported counts | `#[incomplete]`: raw pointer, no slice, `retval` or query projection |
| Produced lengths and post-call capacities | Validated natively, not published; keep a usable entry extent or mark the buffer `incomplete` |
| `_Field_size_*` | Validated natively; extent and nullability omitted |
| `_Ret_maybenull_`, `_Maybenull_`, returned-pointer `_maybenull_` forms | Native nullability retained for checking and reconciliation, not published |
| `_At_(*p, _Post_readable_size_(_Inexpressible_(...)))` | Redundant with an explicit native returned-pointer contract; otherwise `incomplete` |
| Unmodeled null-state and validity SAL | `#[incomplete]` |
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
| `reserved`, `retval`, `incomplete` | `Reserved`, `RetVal`, `Incomplete` |
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
| `associated_enum(T)` | `AssociatedEnum` with a `System.Type` argument; the parameter, return or field type is unchanged |
| `*const T`, `ptr64<T>` | `System.Runtime.CompilerServices.IsConst` and `NativePointer64` modifiers |
| `packed(N)`, `align(N)` | ClassLayout, `Alignment` |
| Bitfield blocks, `anonymous(N)` | `NativeBitfield`, `NativeAnonymous` |
| `library(..., last_error)` | ImplMap flag `SupportsLastError` |
| **Proposed** `preserve_result`, `retained`, `interior_pointers` | `PreserveResult`, `Retained`, `InteriorPointers` |
| **Proposed** `release(F)`, `invalid(N)` | `Release`, `InvalidValue` |
| **Proposed** `also_usable_for(T)` | `AlsoUsableFor` |
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

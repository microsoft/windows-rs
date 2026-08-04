## windows-csharp

Experimental code generator that emits an efficient C# projection of Windows metadata, modeled on
the windows-rs calling convention rather than the managed-wrapper model used by C#/WinRT. It reads a
`.winmd` file and writes C# that projects each WinRT object as a sealed owner over one interface
pointer, calling the COM vtable directly through `delegate* unmanaged` function pointers. Each
owner is one raw pointer with `IDisposable`. Owners must be disposed, calls must not race disposal,
and non-agile objects and activation/static factories must stay in their originating apartment.

Adding `.synchronized()` retains the same generated API while adding atomic call/disposal
coordination, agility and apartment checks, context-aware release, and finalizer recovery.

See [`docs/crates/windows-csharp.md`](https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-csharp.md)
for the design rationale, the benchmark results that motivate it, and the roadmap.

```rust,no_run
windows_csharp::builder()
    .input("component.winmd")
    .input_default()
    .filter("Thin")
    .output("Thin.cs")
    .write()
    .unwrap();
```

Use `.synchronized()` before `.write()` to select the stronger runtime policy.

### C# samples

[`crates/samples/csharp/minesweeper`](../../samples/csharp/minesweeper) is a direct C# port of the
repository's composition Minesweeper. It combines `Windows.UI.Composition`, desktop target
interop, a generated Win32 window procedure and message loop, mouse input, resize handling,
composition shapes and brushes, and key-frame animation without CsWinRT, Windows App SDK
projections, or NuGet packages.

Run it from the repository root with one command:

```powershell
dotnet run --project crates\samples\csharp\minesweeper\minesweeper.csproj `
    -c Release -p:Platform=x64
```

MSBuild invokes the thin Cargo generator before C# compilation. The generated source is committed
and deterministically refreshed by the same command. See the sample readme for bounded smoke
options. The project disables the implicit Windows SDK projection runtime, so its output does not
include `Microsoft.Windows.SDK.NET.dll` or `WinRT.Runtime.dll`.

[`crates/samples/csharp/tictactoe`](../../samples/csharp/tictactoe) is a direct WinUI 3 port of the
reactor Tic-Tac-Toe example. It generates from the pinned reactor metadata, bootstraps the matching
Windows App Runtime, constructs a base `Microsoft.UI.Xaml.Application`, queues window creation on
the WinUI dispatcher, and owns the `Window`, grid, controls, callbacks, and event revokers directly.
MSBuild invokes Cargo to regenerate `Windows.cs` and stage the bootstrap DLL and `resources.pri`.

```powershell
dotnet run --project crates\samples\csharp\tictactoe\tictactoe.csproj `
    -c Release -p:Platform=x64
```

`Builder::select` projects exact classes, interfaces, delegates, enums, structs, and opaque
handles; `Builder::member` narrows classes and interfaces to named members;
`Builder::function`/`Builder::constant` select Win32 exports and constants. All use one transitive
dependency closure instead of projecting an entire namespace. Missing items, wrong item kinds,
unsupported requested signatures or shapes, unavailable architecture copies, and unsupported
required dependencies are deterministic generation errors. Broad namespace filters continue to
emit the supported subset without reporting every incidental unsupported item. See the design
document for the full API. `Builder::architecture` overrides the generator host architecture when
selecting architecture-specific Win32 metadata.

Variadic native exports are omitted by broad filters because `LibraryImport` cannot represent the
unknown tail safely. Selecting one with `Builder::function` returns a deterministic unsupported
diagnostic. Invalid counted-buffer relationships also stay as raw pointer/count parameters instead
of becoming spans.

Method parameter names, flags, optionality, and custom attributes are matched to signature
positions through ECMA-335 `Param.Sequence`. Missing rows retain the signature type and use the
existing `pN`, input, non-optional fallback. Exact selection reports malformed duplicate or
out-of-range sequences instead of pairing rows by table order.

Direction and marker facts come from `MethodParam::direction`, `is_optional`, `is_reserved`, and
`is_retval_attribute`. C# maps an unspecified direction to its existing input fallback, preserves
In+Out for `ref`/`Span<T>` shaping, and keeps buffer validation and COM-return selection local.

This crate is a proof of concept. It projects WinRT runtime classes (with constructors from their
activation and factory metadata, inherited method/property forwarders, and static members from
their static interfaces), interfaces, enums, Boolean values, structs with allocation-free ABI
conversion for Boolean fields and ownership-aware conversion for String fields, arrays, strings,
delegates, events, completion-handler-backed async operations, and the core generic collection
shapes. Async operation and completion-handler IIDs are derived from metadata at generation time;
await checks status once, registers the native `Completed` delegate when needed, and never polls.
Type-safe generic overloads pass derived runtime classes to base-typed parameters through stack-only
QI leases. Reverse WinRT delegates copy borrowed HSTRING inputs, expose object inputs through
callback-confined `Borrowed` views, and transfer owned HSTRING or COM references for managed
string/object returns.
Arrays support scalar, Boolean, enum, blittable-struct, string, and projected-object elements for
input, return, and output parameters.
WinRT structs may contain String fields, including through nested structs. Input conversion owns
the temporary HSTRINGs through the call, while return conversion consumes the native-owned handles.
Vectors support unmanaged values, strings, and projected objects, including safe `Append`,
`RemoveAtEnd`, `GetAt`, `GetMany`, and buffered enumeration. Maps support the same key and value
families. Non-default interfaces are reached with a generic `As<T>()` cast, failing `HRESULT`s
become exceptions, and event registration supports both raw tokens and a disposable revoker.
Apartment-bound owners call
only from their originating COM context, and disposal or finalization returns there through
`IContextCallback`. Cross-apartment access is not implicit. It does not yet cover the full WinRT
type system; see the design document for the current support matrix.

`BorrowAs` amortizes one QI across several calls to a projected base class or interface. The
queried reference remains inside the generated method while its callback receives a non-owning
`Borrowed` view, so copying the view cannot double-release the reference. Use an explicitly typed
`static` lambda or a cached delegate to keep this path allocation-free.

The initial Win32 path emits namespace-local `Apis` classes, native pointers, `BOOL`/`HRESULT`
conversion, and direct IUnknown owners with inherited slots. Exactly one pointer indirection to a
native scalar, enum, or blittable struct projects as `out T`/`ref T` when its metadata direction is
unambiguous. Null-terminated UTF-16 input aliases (`PCWSTR`/`LPCWSTR`) project as `string` or
`string?` and pin the CLR string directly over a pointer-based private import; mutable/output
strings with a scalar capacity project as `Span<char>`; returned/two-call strings and optional
writable pointers stay raw rather than being guessed at. One-to-one element/byte-counted buffers
project as allocation-free `ReadOnlySpan<T>`/`Span<T>` with a checked derived count; an empty span
represents optional null storage, while shared-count buffers remain raw. Direct native COM owners
use the same span surfaces while their generated COM companions keep raw pointer/count ABI
signatures. Native COM record returns use the Microsoft C++ member ABI: an explicit result pointer
immediately follows `this`, and the generated method returns the initialized local. This applies
on X86, X64, and Arm64, including small records and Arm64 homogeneous floating-point aggregates;
scalars, enums, handles, callbacks, and pointers remain direct returns.

An HRESULT function or native COM method promotes an interface double pointer to an owning return
only when metadata identifies one required output-only trailing retval: an explicit
`RetValAttribute`, or one heuristic candidate after input-only parameters. In/out, optional,
reserved, counted, and ambiguous candidates retain their raw `nint*` surface. A promoted failure
releases a non-null
output before throwing, and a required success-null result raises `E_POINTER`. Generated COM
companions always retain the literal pointer ABI. Native callback typedefs project as literal
`delegate* unmanaged[...]` function
pointers with their metadata calling convention, leaving callback lifetime and state with the
caller and introducing no managed delegate allocation. Explicit-layout unions, anonymous nested
records, packing, and architecture-specific record/function copies preserve the selected native
layout. A genuine opaque handle (`HWND`, `HANDLE`, `HDC`, ...) - a native typedef whose sole field
is an opaque `void*` - projects as a distinct, blittable `readonly struct` wrapping one `nint`
field instead of collapsing to a bare `nint`; a scalar identifier alias (`COLORREF`, `ATOM`) or a
pointer-to-named-type alias (`PWSTR`, `LPRECT`) keeps its existing scalar/pointer collapse. The
test slice calls `CreateStreamOnHGlobal` and uses the
returned `IStream` owner for write, seek, read, and deterministic release, and calls the real
`GetDesktopWindow`/`GetWindowRect` pair to pass a returned `HWND` into another native function and
fill an `out` struct. Complete native interfaces also receive `[GeneratedComInterface]` `INameAbi`
companions for managed COM implementations and source-generated interop. These companions preserve
the metadata ABI; the direct owners retain the checked, deterministic consumption surface.

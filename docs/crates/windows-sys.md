# windows-sys

> Raw Windows API declarations, constants, and ABI types.

- 📦 [crates.io](https://crates.io/crates/windows-sys)
- 📖 [API reference](https://microsoft.github.io/windows-docs-rs/)
- 🚀 [Getting started](../../crates/libs/sys/readme.md)
- 🧩 [Samples](../../crates/samples/windows-sys)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/sys)

`windows-sys` projects Windows metadata as raw FFI. Functions keep pointer parameters and native
integer conventions, structs are plain ABI values, and the crate does not provide owning COM or
WinRT wrappers.

## When to use it

Prefer a focused crate when one covers the task. For APIs not covered there, a reusable library
should generate a narrow private binding set with [`windows-bindgen`](windows-bindgen.md).

Choose `windows-sys` when a binary application wants a pre-generated broad API and its Rust layer
should stay close to a C signature, such as:

- sharing layouts and callbacks with existing C or C++ code;
- writing a narrow wrapper where ownership and error translation are handled locally;
- calling a raw API shape that the rich projection cannot represent.

Choose [`windows`](windows.md) when a binary application needs a similarly broad projection with
reference-counted interfaces, parameter conversion traits, and projected results.

Reusable libraries should avoid both umbrella crates. They add dependency weight and version
churn, and several libraries can bring incompatible versions into one dependency tree. Cargo
features limit what compiles, but they do not remove those dependency-management costs.

Do not choose `windows-sys` on the assumption that `windows` adds runtime overhead to every call.
The meaningful difference is API shape and who must maintain the unsafe contract.

## Features and API discovery

The same metadata drives `windows` and `windows-sys`, but raw Win32 feature names usually follow
lowercase header groups. For example, a program using event handles may need `synchapi`,
`handleapi`, and type features such as `winnt` or `minwinbase`.

Use the [API reference](https://microsoft.github.io/windows-docs-rs/) and the generated signature:

1. Search for the native API name.
2. Confirm that the result is available in `windows-sys`; rich-only COM and WinRT projections do
   not have equivalent raw interface wrappers.
3. Enable the feature shown for the function and any feature needed by types in its signature.
4. Inspect compiler errors before adding broad features. Generated feature dependencies already
   include many prerequisites.
5. Verify operating-system availability separately from crate availability.

Keep imports explicit at an FFI boundary. `windows_sys::Win32::*` is convenient in a small sample,
but named imports make it clear which constants, structs, and extern functions a wrapper exposes.

## Raw ABI patterns

The core aliases show the intended level of abstraction:

- `BOOL`, `HRESULT`, `NTSTATUS`, and `RPC_STATUS` are integer aliases.
- `PCSTR` and `PCWSTR` are const pointers.
- `PSTR` and `PWSTR` are mutable pointers.
- `BSTR` and `HSTRING` are raw ABI handles, not owning Rust strings.
- `IUnknown_Vtbl` and `IInspectable_Vtbl` describe base vtable layouts.

Generated functions do not add `Option`, slices, Rust `bool`, automatic out values, or resource
guards. Pass null pointers, lengths, discriminants, and callbacks in the exact form required by the
Windows documentation. Callback functions generally need `extern "system"` and must obey the
documented lifetime and threading rules.

Use `#[repr(C)]` for your own structs that cross the boundary. Do not transmute between
`windows-sys` and `windows` types merely because their current layouts appear equal. Convert
through documented raw representations at one reviewed boundary.

## Safety and ownership

Treat each extern call as a three-part contract:

1. Inputs must remain valid for the duration documented by the API.
2. The return value must be interpreted using that API's success convention.
3. Every acquired resource must be paired with its documented release operation.

Raw handle aliases are often pointers, but pointer shape does not establish ownership. A returned
value may be borrowed, process-owned, closed with `CloseHandle`, released by another function, or
compared with a special sentinel. Build a private RAII guard as soon as an owned resource enters
Rust, and transfer ownership explicitly when the Windows API takes it.

Keep `unsafe` inside a safe wrapper wherever the contract can be expressed and checked in Rust.
Validate buffer sizes and integer conversions before the call. Avoid constructing slices from raw
pointers until nullability, alignment, initialization, and element count have all been established.

## Errors

`windows-sys` returns native status values. It does not translate them into
`windows_core::Result`:

- For `BOOL`, check against zero only when the API documents zero as failure.
- For handles, distinguish null from `INVALID_HANDLE_VALUE`; different functions use different
  sentinels.
- For HRESULT values, use the documented success and failure ranges rather than treating every
  nonzero value as an error.
- For last-error APIs, read `GetLastError` immediately after the failing return and before another
  Windows call.
- Some APIs set last error only for selected failures. Do not report a stale last-error value when
  the API does not promise one.

A wrapper may translate these results into its own error type or use the focused
[`windows-result`](windows-result.md) crate. Keep the native status available when callers need to
make API-specific decisions.

## Strings and buffers

The `s!` and `w!` macros create null-terminated literal pointers. They do not allocate dynamic
storage. For runtime UTF-16 input, encode into a `Vec<u16>`, append a zero, and keep the vector
alive and unchanged until the call returns. Reject or handle interior nulls according to the target
API.

Output strings require the pattern documented by the function. Common forms include a caller-owned
buffer and length, a size query followed by allocation, or a pointer allocated by Windows and
released by a specific function. Do not assume that `PWSTR` implies any one of these forms.

Lengths may count bytes, code units, characters, or include the trailing null. Preserve the native
unit in variable names and check conversions to the API's integer width.

## A practical boundary

A maintainable raw wrapper usually follows this order:

1. Define the smallest public Rust operation needed by the application.
2. Import only the native items needed for that operation.
3. Convert Rust values and establish pointer lifetimes.
4. Make one or a small group of related calls inside a narrow `unsafe` block.
5. Capture status or last error immediately.
6. Put owned resources in guards before any fallible follow-up work.
7. Convert outputs into owned Rust values before returning.

The `windows-sys` service sample is useful for callback and struct layout patterns, but production
code should also check every fallible API result and guard each acquired handle.

---

## Internal documentation

The remainder of this page is for repository contributors.

### How it is built

`tool_package` generates the crate by driving `windows-bindgen` in `--sys --package` mode from
`crates/tools/package/src/sys.txt`. `src/lib.rs` and the small `src/core` ABI module are
hand-maintained; the Windows namespace files and Cargo feature graph are generated.

Sys package generation prunes namespaces that would contain no raw items. In particular, a
namespace containing only projected COM interfaces does not produce an empty module. Generated
extern functions link through `windows-link`.

Do not edit generated source under `crates/libs/sys/src/Windows`. Change metadata, filters, or the
generator and run the appropriate repository tool.

### Testing

Run `cargo test -p windows-sys`; the workspace test crates and `windows-sys` samples cover wider ABI
usage.

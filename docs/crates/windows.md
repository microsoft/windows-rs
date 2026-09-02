# windows

> A broad projected Windows API for Rust binary applications.

- 📦 [crates.io](https://crates.io/crates/windows)
- 📖 [API reference](https://microsoft.github.io/windows-docs-rs/)
- 🚀 [Getting started](../../crates/libs/windows/readme.md)
- 🧩 [Samples](../../crates/samples/windows)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/windows)

The `windows` crate projects Win32, COM, and WinRT metadata into Rust. It keeps the Windows API
shape while adding typed handles, COM reference counting, string and parameter conversions, and
`Result`-returning wrappers where the metadata describes an HRESULT result.

## Choosing a binding crate

| Need | Choose | Why |
| --- | --- | --- |
| Focused crate covers the task | That focused crate | Small, stable dependency and safer API |
| Reusable library needs another API | [`windows-bindgen`](windows-bindgen.md) | Owned binding set |
| Binary application needs broad rich bindings | `windows` | Pre-generated projected API |
| Binary application needs broad raw bindings | [`windows-sys`](windows-sys.md) | Pre-generated FFI |

Start with the focused crates listed in the [crate index](../readme.md). They provide smaller
dependencies and APIs designed around a particular task. If a reusable library needs APIs they do
not expose, generate a private, reviewed binding set with `windows-bindgen`.

Use `windows` when a binary application benefits more from a pre-generated broad projection than
from owning generated bindings. Cargo features limit what compiles, but the umbrella crate still
adds dependency weight and version churn. Different libraries can otherwise bring incompatible
versions of the same large crate into one dependency tree.

It is possible to mix `windows` and `windows-sys`, but their similarly named types are distinct.
Keep conversions at a small boundary rather than passing values from both crates throughout an
application.

## Finding an API and its features

Windows APIs retain their metadata organization:

- WinRT namespaces use Rust modules such as `windows::Data::Xml::Dom`.
- Win32 header groups are exposed below `windows::Win32`, such as `handleapi`, `synchapi`, and
  `winuser`.
- Shared runtime types and traits are under `windows::core`.

Use this workflow when adding an API:

1. Find the Windows API by its native name in the
   [API reference](https://microsoft.github.io/windows-docs-rs/).
2. Read the projected Rust signature rather than translating the C signature by hand.
3. Enable the feature shown for the item. WinRT features follow namespace names with `_` separators,
   such as `Data_Xml_Dom`. Win32 features usually follow lowercase header groups, such as
   `handleapi`.
4. Import narrowly while learning the API. A path such as `windows::Win32::CreateEventW` makes
   missing features and name collisions easier to diagnose than a glob import.
5. Add supporting features reported by the compiler or reference. Cargo feature dependencies pull
   in many prerequisite types automatically, but a function and the types in its signature may be
   gated separately.

Keep the feature list near the code that motivates it. For a workspace, a shared dependency can
centralize the version while each package selects only the features it calls. Avoid enabling every
feature as a discovery shortcut: it slows builds and hides the intended API boundary.

If a documented item does not appear after enabling its feature, check the target first. The
`windows` crate is Windows-only, and availability in Windows metadata does not guarantee that an
API exists on every supported Windows version. Runtime version checks and documented fallback
behavior remain the application's responsibility.

## Reading projected signatures

The Rust signature is the contract. Projection changes common ABI details in useful ways:

- Nullable pointers may become `Option`.
- C `BOOL` inputs may become Rust `bool`.
- String and interface inputs may accept `Param<T>` implementations rather than one exact type.
- COM and WinRT out parameters may become return values.
- HRESULT-returning methods may become `windows::core::Result<T>`.
- Slice-shaped pointer and length pairs may become Rust slices.

These changes do not make every Windows call safe. Generated functions remain `unsafe` when the
caller must uphold pointer, lifetime, threading, initialization, or API-specific invariants. Keep
the `unsafe` block small and document the invariant in the safe wrapper around it.

### Ownership and types

COM and WinRT interface values are owning, reference-counted wrappers. Cloning one calls `AddRef`,
dropping one calls `Release`, and `Interface::cast` performs `QueryInterface`. Prefer those
operations over manipulating raw interface pointers.

Win32 handle types such as `HANDLE` and `HWND` improve type checking but do not imply ownership.
The API documentation determines whether a returned handle is borrowed, owned, or a sentinel and
which function releases it. Pair acquisition and release in one abstraction so early returns do
not leak the resource.

Generated structs follow the Windows ABI. Initialize them with `Default` when supported, then set
required size, version, discriminant, or pointer fields exactly as the native API requires.
Constants sometimes need a cast because the metadata preserves the native constant type while a
function accepts another integer type.

### Errors

`windows::core::Result<T>` and `windows::core::Error` cover HRESULT-based failures. The `?` operator
is the normal path for projected methods that return `Result`.

Win32 APIs use several other failure conventions. A function may return `BOOL`, a null handle,
`INVALID_HANDLE_VALUE`, zero, or another sentinel. Do not apply one check to every function:

- Use `.ok()?` when a projected `BOOL` or `HRESULT` is documented as success/failure.
- Check handle and integer sentinels exactly as documented.
- Call `Error::from_thread()` immediately after an API reports a last-error failure. Another
  Windows call may overwrite the thread's last-error value.
- Do not treat every zero as failure. Some APIs use zero as a valid result or require a separate
  status query.

The `kernel_event` sample demonstrates both patterns: `CreateEventW` requires a null-handle check,
while `SetEvent` and `CloseHandle` return values support `.ok()?`.

### Strings

Choose a string representation from the parameter type and ownership contract:

| Form | Use |
| --- | --- |
| `w!("text")` | Static null-terminated UTF-16 input (`PCWSTR`) |
| `s!("text")` | Static null-terminated narrow input (`PCSTR`) |
| `h!("text")` | Static WinRT `HSTRING` |
| `HSTRING` | Owned WinRT string, including dynamic text |
| `BSTR` | Owned Automation string |
| `PCWSTR` / `PCSTR` | Borrowed const pointer with externally managed lifetime |
| `PWSTR` / `PSTR` | Mutable pointer, commonly for caller-provided output storage |

Prefer the `W` variant of a Win32 API for new code. The `A` variant uses an API-specific Windows
code page, not Rust UTF-8. The literal macros include the required trailing null where applicable.
For dynamic pointer strings, keep the backing buffer alive and immovable for the entire call and
add a trailing null yourself. A pointer string has no ownership; never free it unless the API says
the caller owns the allocation.

## A practical wrapper workflow

When a binary application wraps an API from this crate:

1. Keep generated types and calls in a Windows-specific module.
2. Validate Rust inputs before entering `unsafe`.
3. Convert strings and buffers once, immediately before the call.
4. Check the API's documented success convention before making another Windows call.
5. Move owned handles or interfaces into a type whose `Drop` implementation uses the matching
   release function.
6. Return ordinary Rust values and `Result` from the public wrapper.

This boundary makes API version checks, resource cleanup, and platform-specific tests visible. It
also prevents raw pointers and similarly named `windows` and `windows-sys` types from spreading
through the rest of the program. A reusable library should apply the same boundary to focused
crates and private `windows-bindgen` output instead of depending on this umbrella crate.

---

## Internal documentation

The remainder of this page is for repository contributors.

### How it is built

The published crate is generated by `tool_package`, which drives `windows-bindgen` in `--package`
mode from `crates/tools/package/src/windows.txt`. It emits `src/Windows/mod.rs` and one file per
metadata namespace. `src/lib.rs` is hand-maintained and supplies the docs.rs stub that redirects to
the external API reference.

The generated Cargo features mirror namespaces and Win32 header groups. Feature dependency edges
are generated from type and namespace dependencies. The crate re-exports `windows-core` as
`windows::core` and depends on the focused collections, future, numerics, reference, and time
crates used by the projection.

Do not edit generated source under `crates/libs/windows/src/Windows`. Change metadata, filters, or
the generator and run the appropriate repository tool.

### Testing

Run `cargo test -p windows`; the workspace test crates and Windows samples cover broader API and
projection behavior.

# windows-core

> Core COM and WinRT type support shared by the windows-* crates.

- 📦 [crates.io](https://crates.io/crates/windows-core)
- 📖 [docs.rs](https://docs.rs/windows-core)
- 🚀 [Getting started](../../crates/libs/core/readme.md)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/core)

## When to use this crate

Use `windows-core` directly when you need the common types behind generated Windows bindings,
author a COM or WinRT interface in Rust, or work with COM identity, apartments, factories, or
references. It provides `IUnknown`, `IInspectable`, `Interface`, `GUID`, `RuntimeType`, and the
support used by generated projections.

Most code should start with a focused windows-* crate instead. Those crates already depend on and
re-export the core types they need. Binary applications may use the broad
[`windows`](windows.md) projection. Add a direct `windows-core` dependency when your own public API
names these types or when no higher-level crate
owns the operation.

## Getting started

The crate [README](../../crates/libs/core/readme.md) has the dependency declaration and a minimal
example using strings and results. For a first COM or WinRT workflow:

1. Select or generate bindings for the Windows API you want to call.
2. Initialize the calling thread's COM apartment when the API requires it.
3. Keep projected interface values as owned Rust values and propagate `windows_core::Result`.
4. Use `Interface::cast` when you need another interface implemented by the same object.

For apartment-agnostic command-line code, `init_mta` initializes an uninitialized calling thread as
MTA and keeps the process MTA alive:

```rust
use windows_core::Result;

fn main() -> Result<()> {
    windows_core::init_mta()?;
    // Create and use projected COM or WinRT objects here.
    Ok(())
}
```

If a UI framework or host initializes COM for you, follow its apartment model instead. `init_mta`
does not change a thread that is already initialized in another apartment.

## Core API model

| API | Role |
| --- | --- |
| `IUnknown` | Owning pointer to the base COM interface |
| `IInspectable` | Base interface for WinRT objects |
| `Interface` | Interface identity, vtable access, casts, and raw-pointer interop |
| `GUID` | Interface IDs and other Windows GUID values |
| `Result<T>`, `Error`, `HRESULT` | Re-exported Windows error model |
| `HSTRING`, `PCWSTR`, `PCSTR` | Re-exported Windows string model |
| `AgileReference<T>` | Reference that resolves an apartment-valid proxy |
| `Weak<T>` | Non-owning reference that can be upgraded while the object is alive |
| `EventRevoker` | Event registration that unregisters when dropped |
| `#[interface]`, `#[implement]` | Declare an interface and implement it with a Rust type |

Projected interface values are reference-counted owners. Cloning one performs the corresponding
COM reference-count operation, dropping it releases the reference, and `cast` performs
`QueryInterface`. Prefer these operations over manual `AddRef`, `Release`, or pointer casts.

## Common tasks

### Moving between interfaces

Use `cast` to request another interface from the same COM identity:

```rust
use windows_core::{Interface, IInspectable, IUnknown, Result};

fn as_inspectable(value: &IUnknown) -> Result<IInspectable> {
    value.cast()
}
```

A failed query is returned as an `Error`. A Rust type conversion is not a replacement for
`QueryInterface` unless the generated API provides that conversion.

### Sharing an apartment-bound object

Do not assume that every interface can be sent to another apartment. Create an
`AgileReference<T>`, move or clone that reference, and call `resolve` in the apartment where the
object will be used:

```rust
use windows_core::{AgileReference, Interface, Result};

fn make_agile<T: Interface>(value: &T) -> Result<AgileReference<T>> {
    AgileReference::new(value)
}
```

`Weak<T>` serves a different purpose: it avoids keeping an object alive and returns `None` from
`upgrade` after the object has been destroyed.

### Declaring and implementing an interface

The default `proc-macros` feature re-exports `#[interface]` and `#[implement]`:

```rust
use windows_core::*;

#[interface("7e75ffe0-2f8c-4040-953e-b1f83a48f77b")]
unsafe trait IValue: IUnknown {
    unsafe fn value(&self) -> i32;
}

#[implement(IValue)]
struct Value {
    value: i32,
}

impl IValue_Impl for Value_Impl {
    unsafe fn value(&self) -> i32 {
        self.value
    }
}

fn main() -> Result<()> {
    let object: IValue = Value { value: 42 }.into();
    assert_eq!(unsafe { object.value() }, 42);

    let unknown: IUnknown = object.cast()?;
    let object: IValue = unknown.cast()?;
    assert_eq!(unsafe { object.value() }, 42);

    Ok(())
}
```

`#[interface]` generates the vtable, caller wrappers, and implementation trait. `#[implement]`
generates the wrapper that owns the vtables and reference count. The implementation methods remain
responsible for the interface contract and any safety requirements in the ABI.

## Important choices and pitfalls

- The default features are `std` and `proc-macros`. Disable `proc-macros` when avoiding the
  `syn`, `quote`, and `proc-macro2` build dependencies matters; the narrower `interface_decl!` and
  `implement_decl!` macros remain available.
- Raw interface pointers do not carry an owning lifetime. Use `from_raw`, `from_raw_borrowed`,
  `into_raw`, and related `Interface` operations only when the ownership contract is known.
- COM apartment initialization is per thread. Initializing one thread does not initialize worker
  threads.
- `EventRevoker` unregisters on drop. Keep it alive for as long as the handler should run; use
  `into_token` only when another owner will remove the registration.
- The `imp` module supports generated code and crate internals. Application code should prefer the
  public projected types and traits.

## Samples and next steps

There is no standalone `windows-core` sample group. The
[`windows` samples](../../crates/samples/windows) show `Result`, `init_mta`, strings, and projected
interfaces in complete API calls. The [`robot` sample](../../crates/samples/robot) shows a Rust
component implementing generated COM and WinRT interfaces for multiple clients.

Continue with [`windows-result`](windows-result.md) for error propagation and
[`windows-strings`](windows-strings.md) for ABI string choices.

---

## Internal documentation

The remainder of this page covers how the crate is built and maintained. It is for contributors and
is **not needed to use `windows-core`**.

### How it's built

`src/bindings.rs` is generated by `tool_bindings` from `crates/tools/bindings/src/core.txt`. The
hand-written modules (`agile_reference`, `com_object`, `compose`, `event`) provide the COM runtime
support. The `#[implement]`/`#[interface]` proc macros are re-exported from the
[`windows-implement`](windows-implement.md)/[`windows-interface`](windows-interface.md) crates
behind the `proc-macros` feature, and the `implement_macro`/`interface_macro` modules supply the
`implement_decl!`/`interface_decl!` declarative equivalents used when that feature is off.

### Testing

Run `cargo test -p windows-core`; see also the workspace test crates.

# windows-core

Shared COM and WinRT types, interface ownership, apartment initialization, and authoring macros.
Focused Windows crates often re-export the types they need; depend on `windows-core` directly when
your own code names them.

- 📦 [crates.io](https://crates.io/crates/windows-core)
- 📖 [docs.rs](https://docs.rs/windows-core)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/core)

## Get started

```toml
[dependencies]
windows-core = "0.100"
```

```rust
use windows_core::{h, Result, HRESULT, HSTRING};

let name: &HSTRING = h!("Windows.Foundation.Uri");
assert_eq!(name, &HSTRING::from("Windows.Foundation.Uri"));

fn check(code: HRESULT) -> Result<()> {
    code.ok()
}

assert!(check(HRESULT(0)).is_ok());
assert!(check(HRESULT(-2147467259)).is_err());
```

## Initialize COM

Follow an existing UI framework's apartment model. For standalone code:

| API | Use |
| --- | --- |
| `init_mta()` | Keep a process-wide MTA alive; uninitialized threads use it implicitly. |
| `init_sta()` | Initialize an STA on the calling thread and retain its guard. |

`init_mta` does not change a thread that has already joined another apartment. An STA guard must
outlive its thread's COM objects. `create_instance::<T>(&class_id)` activates an in-process COM
class; choose one compatible with the calling apartment.

## Own and share interfaces

Interface values own reference-counted COM pointers. Clone to retain a reference, let `Drop`
release it, and use `Interface::cast` to request another interface. Prefer a provided base-interface
conversion to an unnecessary `QueryInterface` call.

| Need | API |
| --- | --- |
| Resolve an apartment-valid reference elsewhere | `AgileReference<T>` |
| Refer to an object without keeping it alive | `Weak<T>` |
| Keep an event subscription active | Retain its `EventRevoker` |
| Declare or implement an interface | `#[interface]` and `#[implement]` |

Raw-pointer conversions require a known ownership and lifetime contract. A Win32 handle is not a
COM interface and may need a separate close operation.

The default `proc-macros` feature supplies the authoring attributes. See
[windows-interface](https://docs.rs/windows-interface) and
[windows-implement](https://docs.rs/windows-implement), or the
[robot sample](https://github.com/microsoft/windows-rs/tree/master/crates/samples/robot).

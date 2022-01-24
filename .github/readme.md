[![crates.io](https://img.shields.io/crates/v/windows.svg)](https://crates.io/crates/windows)
[![build](https://github.com/microsoft/windows-rs/workflows/build/badge.svg?event=push)](https://github.com/microsoft/windows-rs/actions)

## Rust for Windows

The `windows` crate lets you call any Windows API past, present, and future using code generated on the fly directly from the metadata describing the API and right into your Rust package where you can call them as if they were just another Rust module. The Rust language projection follows in the tradition established by [C++/WinRT](https://github.com/microsoft/cppwinrt) of building language projections for Windows using standard languages and compilers, providing a natural and idiomatic way for Rust developers to call Windows APIs.

* Crate documentation
    * [windows](https://microsoft.github.io/windows-docs-rs/)
    * [windows-sys](https://docs.rs/windows-sys)
* [Samples](https://github.com/microsoft/windows-samples-rs/)
* [Changelog](https://github.com/microsoft/windows-rs/releases)

Start by adding the following to your Cargo.toml file:

```toml
[dependencies.windows]
version = "0.30.0"
features = [
    "alloc",
    "Data_Xml_Dom",
    "Win32_Foundation",
    "Win32_Security",
    "Win32_System_Threading",
    "Win32_UI_WindowsAndMessaging",
]
```

Make use of any Windows APIs as needed.

```rust
use windows::{
    core::*, Data::Xml::Dom::*, Win32::Foundation::*, Win32::System::Threading::*,
    Win32::UI::WindowsAndMessaging::*,
};

fn main() -> Result<()> {
    let doc = XmlDocument::new()?;
    doc.LoadXml("<html>hello world</html>")?;

    let root = doc.DocumentElement()?;
    assert!(root.NodeName()? == "html");
    assert!(root.InnerText()? == "hello world");

    unsafe {
        let event = CreateEventW(std::ptr::null_mut(), true, false, None);
        SetEvent(event).ok()?;
        WaitForSingleObject(event, 0);
        CloseHandle(event).ok()?;

        MessageBoxA(None, "Text", "Caption", MB_OK);
    }

    Ok(())
}
```

## windows-sys

The `windows-sys` crate is a zero-overhead fallback for the most demanding situations and primarily where the absolute best compile time is essential. It only includes function declarations (externs), structs, and constants. No convenience helpers, traits, or wrappers are provided.

Start by adding the following to your Cargo.toml file:

```toml
[dependencies.windows-sys]
version = "0.30.0"
features = [
    "Win32_Foundation",
    "Win32_Security",
    "Win32_System_Threading",
    "Win32_UI_WindowsAndMessaging",
]
```

Make use of any Windows APIs as needed.

```rust
use windows_sys::{Win32::Foundation::*, Win32::System::Threading::*, Win32::UI::WindowsAndMessaging::*};

fn main() {
    unsafe {
        let event = CreateEventW(std::ptr::null_mut(), 1, 0, std::ptr::null_mut());
        SetEvent(event);
        WaitForSingleObject(event, 0);
        CloseHandle(event);

        MessageBoxA(0, b"Text\0".as_ptr() as _, b"Caption\0".as_ptr() as _, MB_OK);
    }
}
```

## Stability

All crates in `windows-rs` interact with [Windows Metadata (`.winmd`)
files](https://github.com/microsoft/win32metadata) which describe the various
windows APIs. All Windows APIs can be split into two categories:
`win32` APIs and `WinRT/COM` APIs. The way we bind to these APIs is different,
but in the metadata they also have different stability guarantees:

| Metadata category | Stable? |
|-------------------|---------|
| `Win32`           | ❌       |
| `WinRT/COM`       | ✅       |

The `windows-rs` project consists of different crates, which interact with
different parts of the Windows APIs. The stability of a crate is determined by
two factors which both must be stable for the crate to be stable:

1. The stability of the underlying schema we're generating bindings for.
2. The stability of the code we generate for the schema (the "projection").

| Crate name            | `Win32` support? | `WinRT/COM` support? | Projection Stable?  | Crate Stable? |
|-----------------------|------------------|----------------------|---------------------|---------------|
| **`windows`**         | ✅                | ✅                    | ❌                   | ❌             |
| **`windows-sys`**     | ✅                | ❌                    | ❌ (but we're close) | ❌             |
| **`windows-bindgen`** | ✅                | ✅                    | ❌                   | ❌             |

Because of the scope of the project (`230.000+` unique types), marking anything
as "stable" is a difficult task. Despite that, it is our explicit intent to
eventually achieve that. Over time we hope to gradually mark more APIs as
stable, propose concrete plans how to get to `1.0.0` stable releases for all
crates, and ensure that any breakage before then is well-communicated to reduce
friction.

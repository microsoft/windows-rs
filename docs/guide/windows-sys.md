# windows-sys

> Zero-overhead raw bindings (externs, structs, constants) for Windows APIs.

- 📦 [crates.io](https://crates.io/crates/windows-sys)
- 📖 [API reference (docs.rs)](https://docs.rs/windows-sys)
- 🛠 [Internals](../internals/windows-sys.md)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/sys)
- 🔎 [Feature search](https://microsoft.github.io/windows-rs/features)

## Overview

`windows-sys` is the minimal, fastest-compiling projection of the Windows API.
It contains only raw function declarations, structs, and constants — no traits,
helpers, or wrappers. Choose it when compile time matters most and you are
comfortable working with raw pointers and unsafe calls; otherwise prefer the
richer [`windows`](windows.md) crate. As with `windows`, APIs are gated behind
namespace features.

## Example

Requires `Win32_System_Threading` and `Win32_UI_WindowsAndMessaging`:

```rust,ignore
use windows_sys::{
    core::*, Win32::Foundation::*, Win32::System::Threading::*, Win32::UI::WindowsAndMessaging::*,
};

unsafe {
    let event = CreateEventW(std::ptr::null(), 1, 0, std::ptr::null());
    SetEvent(event);
    WaitForSingleObject(event, 0);
    CloseHandle(event);

    MessageBoxW(0 as _, w!("Wide"), w!("Caption"), MB_OK);
}
```
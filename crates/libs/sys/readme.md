## windows-sys

The `windows-sys` crate provides raw Windows API declarations, structs, and constants without
wrappers or helper traits.

* [Getting started](https://github.com/microsoft/windows-rs/blob/master/docs/readme.md)
* [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples)
* [Releases](https://github.com/microsoft/windows-rs/releases)

Start by adding the following to your Cargo.toml file:

```toml
[dependencies.windows-sys]
version = "0.100"
features = [
    "handleapi",
    "synchapi",
    "winuser",
]
```

Make use of any Windows APIs as needed:

```rust,no_run
use windows_sys::{core::*, Win32::*};

unsafe {
    let event = CreateEventW(std::ptr::null(), 1, 0, std::ptr::null());
    SetEvent(event);
    WaitForSingleObject(event, 0);
    CloseHandle(event);

    MessageBoxA(0 as _, s!("Ansi"), s!("Caption"), MB_OK as u32);
    MessageBoxW(0 as _, w!("Wide"), w!("Caption"), MB_OK as u32);
}
```

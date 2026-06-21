# windows-link

> The `link!` macro for linking to Windows functions without import libraries.

- 📦 [crates.io](https://crates.io/crates/windows-link)
- 📖 [API reference (docs.rs)](https://docs.rs/windows-link)
- 🛠 [Internals](../internals/windows-link.md)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/link)

## Overview

`windows-link` provides the `link!` macro for declaring external Windows
functions. It uses `raw-dylib`, so — unlike the equivalent macro in
[`windows-targets`](windows-targets.md) — it does not require import `.lib`
files. The macro declares the function and also emits a `pub type` alias of the
same name describing the function-pointer signature.

## Example

```rust
windows_link::link!("kernel32.dll" "system" fn SetLastError(code: u32));
windows_link::link!("kernel32.dll" "system" fn GetLastError() -> u32);

unsafe {
    SetLastError(1234);
    assert_eq!(GetLastError(), 1234);
}
```
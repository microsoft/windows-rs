# windows-targets

> Import libraries and the `link!` macro for Windows.

- 📦 [crates.io](https://crates.io/crates/windows-targets)
- 📖 [API reference (docs.rs)](https://docs.rs/windows-targets)
- 🛠 [Internals](../internals/windows-targets.md)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/targets)

## Overview

`windows-targets` bundles the Windows import libraries and provides a `link!`
macro for declaring external functions. It supports semantic versioning and,
optionally, `raw-dylib`.

> **Prefer [`windows-link`](windows-link.md)** for new code (Rust 1.71+): it uses
> `raw-dylib` and needs no import `.lib` files. `windows-targets` remains for
> scenarios that require import libraries.

## Example

```rust
windows_targets::link!("kernel32.dll" "system" fn SetLastError(code: u32));
windows_targets::link!("kernel32.dll" "system" fn GetLastError() -> u32);

unsafe {
    SetLastError(1234);
    assert_eq!(GetLastError(), 1234);
}
```
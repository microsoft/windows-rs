# windows-strings

> The string types used across the Windows APIs.

- 📦 [crates.io](https://crates.io/crates/windows-strings)
- 📖 [API reference (docs.rs)](https://docs.rs/windows-strings)
- 🛠 [Internals](../internals/windows-strings.md)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/strings)

## Overview

`windows-strings` provides the owned and borrowed string types that Windows APIs
expect:

- **`HSTRING`** — the reference-counted UTF-16 string used throughout WinRT.
- **`BSTR`** — the length-prefixed UTF-16 string used by COM automation.
- **`PCSTR` / `PCWSTR`** — borrowed, null-terminated ANSI / UTF-16 pointers.

The `s!`, `w!`, and `h!` macros build null-terminated literals at compile time.

## Example

```rust
use windows_strings::{w, BSTR, HSTRING, PCWSTR};

let h = HSTRING::from("hello");
assert_eq!(h, "hello");

let b = BSTR::from("world");
assert_eq!(b, "world");

// A compile-time, null-terminated UTF-16 literal.
const WIDE: PCWSTR = w!("wide");
assert_eq!(unsafe { WIDE.to_string().unwrap() }, "wide");
```
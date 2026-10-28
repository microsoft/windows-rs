# windows-strings

Windows string types and literal macros. Choose the representation required by the API rather
than converting every string to one preferred type.

- 📦 [crates.io](https://crates.io/crates/windows-strings)
- 📖 [docs.rs](https://docs.rs/windows-strings)
- 🧩 [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples/strings)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/strings)

## Get started

```toml
[dependencies]
windows-strings = "0.100"
```

```rust
use windows_strings::*;

let text = HSTRING::from("Hello, Windows!");
let runtime_name: &HSTRING = h!("Windows.Foundation.Uri");
let wide: PCWSTR = w!("Hello");
let narrow: PCSTR = s!("Hello");

assert_eq!(text, "Hello, Windows!");
assert_eq!(unsafe { wide.to_string().unwrap() }, "Hello");
assert_eq!(unsafe { narrow.to_string().unwrap() }, "Hello");
```

## Choose ownership and encoding

| Type | Use |
| --- | --- |
| `HSTRING` | Owned WinRT UTF-16 string |
| `BSTR` | Owned COM Automation string |
| `PCWSTR`, `PCSTR` | Borrowed null-terminated UTF-16 or 8-bit input |
| `PWSTR`, `PSTR` | Mutable pointers to caller- or API-owned storage |

`HSTRING` and `BSTR` carry a length. Pointer wrappers do not record a length, lifetime, or
ownership. Prefer wide Windows APIs: `s!` encodes UTF-8, but many narrow Win32 APIs expect a
Windows code page.

For dynamic pointer strings, encode into a buffer, append a zero, and retain the buffer throughout
the call. Validate interior nulls when the API must receive the whole string. Output parameters
need writable storage with the documented size.

Reading a returned pointer is unsafe. Establish its validity and termination, then follow the
API's allocation and release rules. A null pointer is not an empty string.

`HSTRING::from_wide` accepts existing UTF-16 data. `BSTR` and Windows `OsStr`/`Path` conversions
are Windows-only; standard-library conversions require the default `std` feature.

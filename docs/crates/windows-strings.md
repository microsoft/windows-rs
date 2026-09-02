# windows-strings

> The string types used across the Windows APIs.

- 📦 [crates.io](https://crates.io/crates/windows-strings)
- 📖 [docs.rs](https://docs.rs/windows-strings)
- 🚀 [Getting started](../../crates/libs/strings/readme.md)
- 🧩 [Samples](../../crates/samples/strings)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/strings)

## When to use this crate

Use `windows-strings` when a Windows ABI requires a Windows string type and you do not otherwise
need `windows-core`. Most projected APIs re-export these types, so use the type named by the
function signature instead of converting every string to one preferred representation.

The main choice is ownership: `HSTRING` and `BSTR` own their contents, while `PCSTR`, `PCWSTR`,
`PSTR`, and `PWSTR` are pointer wrappers whose storage belongs elsewhere.

## Getting started

The crate [README](../../crates/libs/strings/readme.md) has the dependency declaration and a
minimal example for each common representation. Before the first API call, inspect its signature:

1. Use `HSTRING` for WinRT strings.
2. Use `BSTR` for COM Automation strings.
3. Use `PCWSTR` or `PCSTR` for borrowed, null-terminated input.
4. Use `PWSTR` or `PSTR` only when the API writes through the pointer.

`HSTRING`, the pointer wrappers, and the literal macros are available across targets. `BSTR` is
Windows-only because allocation and deallocation use the COM Automation ABI. Conversions involving
`OsStr`, `OsString`, or `Path` require Windows and the default `std` feature.

## Core API model

| Type or macro | Encoding and ownership | Common use |
| --- | --- | --- |
| `HSTRING` | Owned, immutable, reference-counted UTF-16 | WinRT parameters and results |
| `BSTR` | Owned, length-prefixed UTF-16 | COM Automation parameters and results |
| `PCWSTR` / `PWSTR` | Borrowed const / mutable UTF-16 pointer | Win32 wide-string ABIs |
| `PCSTR` / `PSTR` | Borrowed const / mutable 8-bit pointer | Win32 8-bit string ABIs |
| `h!` | Static `&HSTRING` literal | Reused WinRT string constant |
| `w!` | Static null-terminated `PCWSTR` literal | Wide input constant |
| `s!` | Static null-terminated UTF-8 `PCSTR` literal | 8-bit input constant |
| `HStringBuilder` | Preallocated mutable UTF-16 buffer | Build one immutable `HSTRING` |

Length-prefixed types can represent data independently of a trailing null. Pointer types depend on
a null terminator and do not record a length or lifetime.

## Common tasks

### Create owned strings

```rust
use windows_strings::{BSTR, HSTRING};

let runtime_name = HSTRING::from("Windows.Foundation.Uri");
assert_eq!(runtime_name, "Windows.Foundation.Uri");

#[cfg(windows)]
{
    let automation = BSTR::from("report");
    assert_eq!(automation, "report");
}
```

`HSTRING::from_wide` accepts UTF-16 code units directly. `to_string_lossy` and `display` are useful
when diagnostics should continue through invalid UTF-16; equality against `str` compares the
encoded contents without first allocating a Rust `String`.

### Pass string literals

```rust
use windows_strings::{h, s, w, HSTRING, PCSTR, PCWSTR};

const UTF8_NAME: PCSTR = s!("name");
const WIDE_NAME: PCWSTR = w!("name");

let runtime_name: &HSTRING = h!("Windows.Foundation.Uri");
```

The literal macros append the required terminator at compile time. `h!` returns a borrowed static
`HSTRING`; clone it or construct `HSTRING::from` only when an owned value is required.

### Read a pointer returned by Windows

Pointer conversion is unsafe because the wrapper cannot prove that the pointer is non-null, valid,
terminated, or still alive:

```rust
use windows_strings::PCWSTR;

unsafe fn copy_wide(value: PCWSTR) -> Result<String, std::string::FromUtf16Error> {
    unsafe { value.to_string() }
}
```

Check the producing API's ownership rules before reading or freeing its pointer. A null pointer is
not an empty string: calling `len`, `as_wide`, `as_bytes`, or `to_string` on it is invalid.

### Fill an `HSTRING` buffer

Use `HStringBuilder::new` when another operation fills a known number of UTF-16 code units. Write
through its mutable slice, call `trim_end` if unused trailing elements remain zero, and convert the
builder into `HSTRING`.

## Important choices and pitfalls

- Prefer wide Windows APIs when both 8-bit and wide variants exist. A `PCSTR` type alone does not
  define which legacy code page a particular Win32 function applies.
- `PCSTR`, `PCWSTR`, `PSTR`, and `PWSTR` do not own their buffers. Keep the backing allocation alive
  for the entire call.
- Do not pass an immutable literal through `PSTR` or `PWSTR`; writable pointer types require
  writable storage sized as the API specifies.
- Use `BSTR` only for APIs that transfer or borrow a COM Automation string according to the stated
  ownership contract.
- The default `std` feature adds standard-library conversions. Disable it only if the remaining
  `alloc`-based API meets the target's needs.

## Samples and next steps

The [string samples](../../crates/samples/strings/samples/examples) cover `HSTRING`, `BSTR`, and
the wide and 8-bit literal macros. Run them from the workspace with:

```text
cargo run -p strings_samples --example hstring
cargo run -p strings_samples --example bstr
cargo run -p strings_samples --example wide_ansi
```

Continue with [`windows-core`](windows-core.md) when the string is part of a projected COM or WinRT
call.

---

## Internal documentation

The remainder of this page covers how the crate is built and maintained. It is for contributors and
is **not needed to use `windows-strings`**.

### How it's built

`src/bindings.rs` is generated by `tool_bindings` from `crates/tools/bindings/src/strings.txt`; the
string types and macros are hand-written.

### Testing

Run `cargo test -p windows-strings`; see also the workspace test crates.

# windows-link

Declare Windows DLL imports without import libraries. Use `link!` for a small, reviewed raw
binding; prefer [windows-bindgen](https://docs.rs/windows-bindgen) when metadata describes the API.

- 📦 [crates.io](https://crates.io/crates/windows-link)
- 📖 [docs.rs](https://docs.rs/windows-link)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/link)

## Get started

```toml
[dependencies]
windows-link = "0.100"
```

```rust,no_run
windows_link::link!("kernel32.dll" "system" fn GetTickCount64() -> u64);

fn main() {
    let function: GetTickCount64 = GetTickCount64;
    println!("{} ms since startup", unsafe { function() });
}
```

The macro declares the imported function and a matching function-pointer type with the same name.
An optional link-name literal selects an export whose name differs from the Rust identifier.
Use `ordinal(12)` instead of that literal to select an export by ordinal. For example,
[`SHCreateMemStream`](https://learn.microsoft.com/windows/win32/api/shlwapi/nf-shlwapi-shcreatememstream)
has the documented ordinal 12:

```rust,no_run
windows_link::link!("shlwapi.dll" "system" ordinal(12)
    fn SHCreateMemStream(data: *const u8, len: u32) -> *mut core::ffi::c_void);
```

Ordinals must be in the range 1 through 65535. Use a documented stable ordinal or the ordinal
recorded in the DLL's import library for the target architecture, not an automatically assigned
number observed in one DLL build. Ordinal imports still require the API's supported OS version and
any required application manifest.

## Match the native contract

Confirm the exporting DLL, symbol name, calling convention, and exact parameter and result types.
A successful link does not prove that the signature is correct. Keep declarations in a private
FFI module and expose a safe wrapper only when it can uphold the native contract.

`link!` creates a static import, not an optional or delayed load. For exports that may be absent on
older Windows versions, resolve them dynamically and retain the module while using its function
pointers. The type alias does not validate an address obtained through dynamic lookup.

Add platform and feature gates in the calling crate. Declaring a Windows function on another
platform does not make the export available there.

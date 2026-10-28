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

## Match the native contract

Confirm the exporting DLL, symbol name, calling convention, and exact parameter and result types.
A successful link does not prove that the signature is correct. Keep declarations in a private
FFI module and expose a safe wrapper only when it can uphold the native contract.

`link!` creates a static import, not an optional or delayed load. For exports that may be absent on
older Windows versions, resolve them dynamically and retain the module while using its function
pointers. The type alias does not validate an address obtained through dynamic lookup.

Add platform and feature gates in the calling crate. Declaring a Windows function on another
platform does not make the export available there.

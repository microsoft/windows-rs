## windows-link

The [windows-link](https://crates.io/crates/windows-link) crate provides the `link` macro that
simplifies linking. The `link` macro uses `raw-dylib` and thus does not require import lib files.

* [Getting
  started](https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-link.md)

Start by adding the following to your Cargo.toml file:

```toml
[dependencies.windows-link]
version = "0.100"
```

Use the `link` macro to import functions and their matching function-pointer types. Here the loader
functions are linked normally while `MessageBoxA` is resolved at runtime:

```rust,no_run
use std::ffi::c_void;
use windows_link::link;

link!("kernel32.dll" "system" fn LoadLibraryA(name: *const u8) -> *mut c_void);
link!("kernel32.dll" "system" fn GetProcAddress(
    library: *mut c_void,
    name: *const u8,
) -> *mut c_void);
link!("user32.dll" "system" fn MessageBoxA(
    hwnd: *mut c_void,
    text: *const u8,
    caption: *const u8,
    kind: u32,
) -> i32);

unsafe {
    let module = LoadLibraryA(b"user32.dll\0".as_ptr());
    let address = GetProcAddress(module, b"MessageBoxA\0".as_ptr());

    if !address.is_null() {
        let message_box: MessageBoxA = std::mem::transmute(address);

        message_box(
            std::ptr::null_mut(),
            b"text\0".as_ptr(),
            b"caption\0".as_ptr(),
            0,
        );
    }
}
```

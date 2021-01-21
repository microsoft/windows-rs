[![crates.io](https://img.shields.io/crates/v/windows.svg)](https://crates.io/crates/windows)
[![docs.rs](https://docs.rs/windows/badge.svg)](https://docs.rs/windows)
[![Build and Test](https://github.com/microsoft/windows-rs/workflows/Build%20and%20Test/badge.svg?event=push)](https://github.com/microsoft/windows-rs/actions)

## Rust for Windows

The `windows` crate lets you call any Windows API past, present, and future using code generated on the fly directly from the metadata describing the API and right into your Rust package where you can call them as if they were just another Rust module.

The Rust language projection follows in the tradition established by [C++/WinRT](https://github.com/microsoft/cppwinrt) of building language projections for Windows using standard languages and compilers, providing a natural and idiomatic way for Rust developers to call Windows APIs. 

## Getting started

Start by adding the following to your Cargo.toml file:

```toml
[dependencies]
windows = "0.2.1"

[build-dependencies]
windows = "0.2.1"
```

This will allow Cargo to download, build, and cache Windows support as a package. Next, specify which types you need inside of a `build.rs` build script and the `windows` crate will generate the necessary bindings:

```rust
fn main() {
    windows::build!(
        windows::data::xml::dom::*
        windows::win32::system_services::{CreateEventW, SetEvent, WaitForSingleObject}
        windows::win32::windows_programming::CloseHandle
    );
}
```

Finally, make use of any Windows APIs as needed.

```rust
mod bindings {
    ::windows::include_bindings!();
}

use bindings::{
    windows::data::xml::dom::*,
    windows::win32::system_services::{CreateEventW, SetEvent, WaitForSingleObject},
    windows::win32::windows_programming::CloseHandle,
};

fn main() -> windows::Result<()> {
    let doc = XmlDocument::new()?;
    doc.load_xml("<html>hello world</html>")?;

    let root = doc.document_element()?;
    assert!(root.node_name()? == "html");
    assert!(root.inner_text()? == "hello world");

    unsafe {
        let event = CreateEventW(
            std::ptr::null_mut(),
            true.into(),
            false.into(),
            std::ptr::null(),
        );

        SetEvent(event).ok()?;
        WaitForSingleObject(event, 0);
        CloseHandle(event).ok()?;
    }

    Ok(())
}
```

To reduce build time, use a `bindings` crate rather simply a module. This will allow Cargo to cache the results and build your project far more quickly.

There is an experimental [documentation generator](https://github.com/microsoft/windows-docs-rs) for the Windows API. The documentation [is published here](https://microsoft.github.io/windows-docs-rs/). This can be useful to figure out how the various Windows APIs map to Rust modules and which `use` paths you need to use from within the `build` macro.

For a more complete example, take a look at Robert Mikhayelyan's [Minesweeper](https://github.com/robmikh/minesweeper-rs). More simple examples [can be found here](https://github.com/kennykerr/samples-rs).

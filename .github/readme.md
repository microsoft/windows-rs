[![crates.io](https://img.shields.io/crates/v/windows.svg)](https://crates.io/crates/windows)
[![docs.rs](https://docs.rs/windows/badge.svg)](https://docs.rs/windows)
[![build](https://github.com/microsoft/windows-rs/workflows/build/badge.svg?event=push)](https://github.com/microsoft/windows-rs/actions)

## Rust for the Windows SDK

The `windows` crate lets you call any Windows API past, present, and future using code generated on the fly directly from the metadata describing the API and right into your Rust package where you can call them as if they were just another Rust module.

The Rust language projection follows in the tradition established by [C++/WinRT](https://github.com/microsoft/cppwinrt) of building language projections for Windows using standard languages and compilers, providing a natural and idiomatic way for Rust developers to call Windows APIs.

Watch the [Getting Started](https://www.youtube.com/watch?v=-oZrsCPKsn4) video! Microsoft Docs also has content on [developing with Rust on Windows](https://docs.microsoft.com/en-us/windows/dev-environment/rust/).

Check out the [FAQ](./docs/FAQ.md) for answers to frequently asked questions.

## Getting started

Start by adding the following to your Cargo.toml file:

```toml
[dependencies]
windows = "0.15.0"

[build-dependencies]
windows = "0.15.0"
```

This will allow Cargo to download, build, and cache Windows support as a package. Next, specify which types you need inside of a `build.rs` build script and the `windows` crate will generate the necessary bindings:

```rust
fn main() {
    windows::build! {
        Windows::Data::Xml::Dom::*,
        Windows::Win32::Foundation::CloseHandle,
        Windows::Win32::System::Threading::{CreateEventW, SetEvent, WaitForSingleObject},
        Windows::Win32::UI::WindowsAndMessaging::MessageBoxA,
    };
}
```

Finally, make use of any Windows APIs as needed.

```rust
mod bindings {
    windows::include_bindings!();
}

use bindings::{
    Windows::Data::Xml::Dom::*,
    Windows::Win32::Foundation::CloseHandle,
    Windows::Win32::System::Threading::{CreateEventW, SetEvent, WaitForSingleObject},
    Windows::Win32::UI::WindowsAndMessaging::{MessageBoxA, MB_OK},
};

fn main() -> windows::Result<()> {
    let doc = XmlDocument::new()?;
    doc.LoadXml("<html>hello world</html>")?;

    let root = doc.DocumentElement()?;
    assert!(root.NodeName()? == "html");
    assert!(root.InnerText()? == "hello world");

    unsafe {
        let event = CreateEventW(std::ptr::null_mut(), true, false, None);
        SetEvent(event).ok()?;
        WaitForSingleObject(event, 0);
        CloseHandle(event).ok()?;

        MessageBoxA(None, "Text", "Caption", MB_OK);
    }

    Ok(())
}
```

To reduce build time, use a `bindings` crate rather than simply a module. This will allow Cargo to cache the results and build your project far more quickly.

There is an experimental [documentation generator](https://github.com/microsoft/windows-docs-rs) for the Windows API. The documentation [is published here](https://microsoft.github.io/windows-docs-rs/). This can be useful to figure out how the various Windows APIs map to Rust modules and which `use` paths you need to use from within the `build` macro.

More examples [can be found here](https://github.com/microsoft/windows-samples-rs). Robert Mikhayelyan's [Minesweeper](https://github.com/robmikh/minesweeper-rs) is also a great example.

A more in-depth getting started guide can also be found [here](docs/getting-started.md).

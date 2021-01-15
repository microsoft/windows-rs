[![crates.io](https://img.shields.io/crates/v/winrt.svg)](https://crates.io/crates/winrt)
[![docs.rs](https://docs.rs/winrt/badge.svg)](https://docs.rs/winrt)
[![Build and Test](https://github.com/microsoft/windows-rs/workflows/Build%20and%20Test/badge.svg?event=push)](https://github.com/microsoft/windows-rs/actions)

## The Rust language projection for the Windows API

Rust/WinRT follows in the tradition established by [C++/WinRT](https://github.com/microsoft/cppwinrt) of building language projections for the Windows Runtime using standard languages and compilers, providing a natural and idiomatic way for Rust developers to call Windows APIs. Rust/WinRT lets you call any Windows API past, present, and future using code generated on the fly directly from the metadata describing the API and right into your Rust package where you can call them as if they were just another Rust module.

The Windows Runtime is based on Component Object Model (COM) APIs under the hood and is designed to be accessed through language projections like C++/WinRT and Rust/WinRT. Those language projections take the metadata describing various APIs and provide natural bindings for the target programming language. As you can imagine, this allows developers to more easily build apps and components for Windows using their desired language. You can then use those Windows APIs to build desktop apps, store apps, or something more unique like a component, NT service, or device driver.

## Getting started

Start by adding the following to your Cargo.toml file:

```toml
[dependencies]
windows = { git = "https://github.com/microsoft/windows-rs" }

[build-dependencies]
windows = { git = "https://github.com/microsoft/windows-rs" }
```

This will allow Cargo to download, build, and cache the Rust/WinRT support as a package. Next, specify which types you need inside of a build.rs build script and Rust/WinRT will generate the necessary bindings:

```rust
fn main() {
    windows::build!(
        windows::data::xml::dom::*
        windows::ui::*
    );
}
```

Finally, make use of any Windows APIs as needed. For example, here is an example of using the `XmlDocument` class to parse an XML document.

```rust
mod bindings {
    ::windows::include_bindings!();
}

use bindings::windows::data::xml::dom::XmlDocument;

fn main() -> windows::Result<()> {
    let doc = XmlDocument::new()?;
    doc.load_xml("<html>hello world</html>")?;

    let root = doc.document_element()?;
    assert!(root.node_name()? == "html");
    assert!(root.inner_text()? == "hello world");

    Ok(())
}
```

To reduce build time, use a `bindings` crate rather simply a module. This will allow Cargo to cache the results and build your project far more quickly.

For a more complete example, take a look at Robert Mikhayelyan's [Minesweeper](https://github.com/robmikh/minesweeper-rs).

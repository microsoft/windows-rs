[![Build and Test](https://github.com/microsoft/winrt-rs/workflows/Build%20and%20Test/badge.svg?event=push)](https://github.com/microsoft/winrt-rs/actions)

## The Rust/WinRT language projection

Rust/WinRT follows in the tradition established by [C++/WinRT](https://github.com/microsoft/cppwinrt) of building language projections for the Windows Runtime using standard languages and compilers, providing a natural and idiomatic way for Rust developers to call Windows APIs. Rust/WinRT lets you call any WinRT API past, present, and future using code generated on the fly directly from the metadata describing the API and right into your Rust package where you can call them as if they were just another Rust module.

The Windows Runtime is based on Component Object Model (COM) APIs under the hood and is designed to be accessed through language projections like C++/WinRT and Rust/WinRT. Those language projections take the metadata describing various APIs and provide natural bindings for the target programming language. As you can imagine, this allows developers to more easily build apps and components for Windows using their desired language. You can then use those Windows APIs to build desktop apps, store apps, or something more unique like a component, NT service, or device driver.

## Getting started

Start by adding the following to your Cargo.toml file:

```toml
[dependencies]
winrt = { git = "https://github.com/microsoft/winrt-rs" }
```

This will allow Cargo to download, build, and cache the Rust/WinRT support as a package directly from GitHub.

```rust
use winrt::*;

// Now use the `import` macro to import the desired winmd files and modules:
import!(
    dependencies
        os
    modules
        "windows.data.xml.dom"
        "windows.foundation"
        "windows.ui"
);

// Finally, make use of any WinRT APIs as needed. For example, here is
// an example of using the `XmlDocument` class to parse an XML document:
fn main() -> Result<()> {
    use windows::data::xml::dom::*;

    let doc = XmlDocument::new()?;
    doc.load_xml("<html>hello world</html>")?;

    let root = doc.document_element()?;
    assert!(root.node_name()? == "html");
    assert!(root.inner_text()? == "hello world");

    Ok(())
}
```

For a more complete example, take a look at Robert Mikhayelyan's [Minesweeper](https://github.com/robmikh/minesweeper-rs).

## Cross-platform support

While support for platforms other than Windows (i.e., *-windows-msvc targets) is possible and on the long-term roadmap, support outside of Windows is not an immediate goal. If you are interested in support outside of Windows, please [let us know](https://github.com/microsoft/winrt-rs/issues/143).

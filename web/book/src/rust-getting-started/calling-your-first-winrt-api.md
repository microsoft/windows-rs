# Calling your first WinRT API

Windows 8 introduced the Windows Runtime, which at its heart, is just COM with a few more conventions thrown in to make language bindings appear more seamless. The [windows](https://crates.io/crates/windows) crate already makes calling COM APIs far more seamless than it is for C++ developers, but WinRT goes further by providing first-class support for modeling things like constructors, events, and class hierarchies. In [calling your first COM API](calling-your-first-com-api.md), we saw that you still had to bootstrap the API with a C-style DLL export before calling COM interface methods. WinRT works the same way but abstracts this away in a generalized manner. 

Let's use a simple example to illustrate. The `XmlDocument` "class" models an XML document that can be loaded from various sources. The Rust [docs for the windows crate](https://microsoft.github.io/windows-docs-rs/doc/windows/Data/Xml/Dom/struct.XmlDocument.html) indicate that this type resides in the `Data::Xml::Dom` module so we can configure our `windows` crate dependency as follows:

```toml
[dependencies.windows]
version = "0.52" 
features = [
    "Data_Xml_Dom",
]
```

And we can employ a `use` declaration to make this API a little more accessible. The `windows` crate's `core` module just provides a few helpers to make it easier to work with Windows APIs, so we'll include that as well:

```rust
use windows::{core::*, Data::Xml::Dom::XmlDocument}; 
```

For this example, I'll just use a simple `main` function with a `Result` type from the `windows::core` module to provide automatic error propagation and simplify the subsequent API calls: 

```rust
fn main() -> Result<()> {

    Ok(())
}
```

Unlike the previous Win32 and COM examples, you'll notice that this `main` function does not need an `unsafe` block since WinRT calls are assumed to be safe thanks to its more constrained type-system.

To begin, we can simply call the `new` method to create a new `XmlDocument` object:

```rust
let doc = XmlDocument::new()?;
```

This looks a lot more like an idiomatic Rust type than your typical COM API, but under the hood a similar mechanism is used to instantiate the `XmlDocument` implementation via a DLL export. We can then call the `LoadXml` method to test it out. There are various other options for loading XML from different sources, which you can [read about in the official documentation](https://learn.microsoft.com/en-us/uwp/api/windows.data.xml.dom.xmldocument?view=winrt-22621) or from the Rust docs for the `XmlDocument` API. The `windows` crate also provides the handy `h!` macro for creating an `HSTRING`, the string type used by WinRT APIs:

```rust
doc.LoadXml(h!("<html>hello world</html>"))?;
```

And just like that, we have a fully-formed Xml document that we can inspect. For this example, let's just grab the document element and then do some basic queries as follows:

```rust
let root = doc.DocumentElement()?;
assert!(root.NodeName()? == "html");
println!("{}", root.InnerText()?);
```

First we assert that the element's name is in fact "html" and then print out the element's inner text. As with the previous COM example, those methods all invoke virtual functions through COM interfaces, but the `windows` crate makes it very simple to make such calls directly from Rust. And that's it. Running the sample should print something like this:

```
hello world
```

Here's the [full sample for reference](https://github.com/microsoft/windows-rs/blob/master/crates/samples/windows/xml/src/main.rs).

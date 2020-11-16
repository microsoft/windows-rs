/*!
Rust/WinRT follows in the tradition established by [C++/WinRT](https://github.com/microsoft/cppwinrt)
of building language projections for the Windows Runtime using standard languages and compilers,
providing a natural and idiomatic way for Rust developers to call Windows APIs. Rust/WinRT lets you
call any WinRT API past, present, and future using code generated on the fly directly from the
metadata describing the API and right into your Rust package where you can call them as if they were
just another Rust module.

The Windows Runtime is based on Component Object Model (COM) APIs under the hood and is designed to
be accessed through language projections like C++/WinRT and Rust/WinRT. Those language projections
take the metadata describing various APIs and provide natural bindings for the target programming
language. As you can imagine, this allows developers to more easily build apps and components for
Windows using their desired language. You can then use those Windows APIs to build desktop apps,
store apps, or something more unique like a component, NT service, or device driver.

Start by adding the following to your Cargo.toml file:

```toml
[dependencies]
winrt = { git = "https://github.com/microsoft/winrt-rs" }

[build-dependencies]
winrt = { git = "https://github.com/microsoft/winrt-rs" }
```

This will allow Cargo to download, build, and cache the Rust/WinRT support as a package.


Next, specify your WinRT dependencies in your Cargo.toml file:

```toml
[package.metadata.winrt.dependencies]
"Microsoft.Windows.SDK.Contracts" = "10.0.19041.1"
```

This automatically downloads the dependency from NuGet and places the WinRT metadata files in a
well-known spot. You will need to run `cargo winrt install` to download the dependencies. You can
read about [cargo winrt here](https://github.com/microsoft/winrt-rs/tree/master/crates/cargo).

Then, generate the code by specifying which types you need inside of a build.rs build script.

```rust
fn main() {
    winrt::build!(
        types
            windows::data::xml::dom::*
            windows::ui::*
    );
}
```

Finally, make use of any WinRT APIs as needed. For example, here is an example of using the
`XmlDocument` class to parse an XML document.

```rust
use winrt::*;
include_bindings!();

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

For a more complete example, take a look at Robert Mikhayelyan's
[Minesweeper](https://github.com/robmikh/minesweeper-rs).
*/

#[macro_use]
mod macros;

mod interfaces;
mod result;
mod runtime;
mod strings;
mod traits;

use interfaces::*;
use runtime::*;
use strings::*;

pub use interfaces::{IActivationFactory, IAgileObject, IUnknown, Object};
pub use result::{Error, ErrorCode, Result};
pub use runtime::{Array, FactoryCache, Guid, Param, RefCount, Waiter};
pub use strings::HString;
pub use traits::{Abi, Interface, RuntimeName, RuntimeType};
pub use winrt_macros::{build, implement};

extern crate self as winrt;

mod bindings {
    include_bindings!();
}

#[doc(hidden)]
pub type RawPtr = *mut std::ffi::c_void;

#[doc(hidden)]
pub use bindings::windows::foundation;

#[doc(hidden)]
pub use const_sha1::ConstBuffer;

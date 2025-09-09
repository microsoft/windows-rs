## Rust for Windows

The [windows](https://crates.io/crates/windows) and [windows-sys](https://crates.io/crates/windows-sys) crates let you call any Windows API past, present, and future using code generated on the fly directly from the [metadata describing the API](https://github.com/microsoft/windows-rs/tree/master/crates/libs/bindgen/default) and right into your Rust package where you can call them as if they were just another Rust module. The Rust language projection follows in the tradition established by [C++/WinRT](https://github.com/microsoft/cppwinrt) of building language projections for Windows using standard languages and compilers, providing a natural and idiomatic way for Rust developers to call Windows APIs.

* [Getting started](https://kennykerr.ca/rust-getting-started/)
* [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples)
* [Releases](https://github.com/microsoft/windows-rs/releases)
* [Feature search](https://microsoft.github.io/windows-rs/features)

Start by adding the following to your Cargo.toml file (see [Dependency Version Ranges](#dependency-version-ranges) for guidance on version specification):

```toml
[dependencies.windows]
version = "0.62"
features = [
    "Data_Xml_Dom",
    "Win32_Security",
    "Win32_System_Threading",
    "Win32_UI_WindowsAndMessaging",
]
```

Make use of any Windows APIs as needed:

```rust,no_run
use windows::{
    core::*, Data::Xml::Dom::*, Win32::Foundation::*, Win32::System::Threading::*,
    Win32::UI::WindowsAndMessaging::*,
};

fn main() -> Result<()> {
    let doc = XmlDocument::new()?;
    doc.LoadXml(h!("<html>hello world</html>"))?;

    let root = doc.DocumentElement()?;
    assert!(root.NodeName()? == "html");
    assert!(root.InnerText()? == "hello world");

    unsafe {
        let event = CreateEventW(None, true, false, None)?;
        SetEvent(event)?;
        WaitForSingleObject(event, 0);
        CloseHandle(event)?;

        MessageBoxA(None, s!("Ansi"), s!("Caption"), MB_OK);
        MessageBoxW(None, w!("Wide"), w!("Caption"), MB_OK);
    }

    Ok(())
}
```

## Dependency Version Ranges

When adding `windows` as a dependency, consider using looser semver ranges to improve ecosystem compatibility and reduce duplicate dependencies in your build graph. Instead of pinning to a specific version:

```toml
# Specific version - may cause duplicate dependencies
[dependencies.windows]
version = "0.62"

# Better: Use a wider range for ecosystem compatibility
[dependencies.windows]
version = ">=0.59, <=0.61"  # Compatible with multiple versions
```

**Benefits of wider version ranges:**

- Reduces likelihood of duplicate `windows` versions in dependency graphs
- Prevents `clippy::multiple-crate-versions` warnings in your consumers
- Improves compatibility with other crates that also depend on `windows`
- Allows users more flexibility in dependency resolution

**Important:** When using wider version ranges, be sure to test your code with the minimum supported version. One approach is by using `cargo-minimal-versions`:

```pwsh
# Install cargo-minimal-versions and cargo-hack
cargo install --locked cargo-minimal-versions 
cargo install --locked cargo-hack

# Test with minimal dependency versions
cargo minimal-versions build
cargo minimal-versions test
```

This ensures your crate actually works with the minimum version you specify, not just the latest.

# windows

> The umbrella crate for calling Windows APIs from Rust.

- 📦 [crates.io](https://crates.io/crates/windows)
- 📖 [API reference (docs.rs)](https://docs.rs/windows)
- 🛠 [Internals](../internals/windows.md)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/windows)

## Overview

The `windows` crate projects the entire Windows API surface — Win32, COM, and
WinRT — into idiomatic Rust, generated from the official Windows metadata. It is
large, so APIs are gated behind Cargo features named after their namespace; you
enable only the modules you use.

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

The [API reference](https://docs.rs/windows) shows which feature enables a given
type or function.

## Example

The snippet below mixes WinRT (`XmlDocument`) and Win32 (`CreateEventW`,
`MessageBoxW`). It requires the features listed above:

```rust,ignore
use windows::{
    core::*, Data::Xml::Dom::*, Win32::Foundation::*, Win32::System::Threading::*,
    Win32::UI::WindowsAndMessaging::*,
};

fn main() -> Result<()> {
    let doc = XmlDocument::new()?;
    doc.LoadXml(h!("<html>hello world</html>"))?;
    let root = doc.DocumentElement()?;
    assert!(root.NodeName()? == "html");

    unsafe {
        let event = CreateEventW(None, true, false, None)?;
        SetEvent(event)?;
        WaitForSingleObject(event, 0);
        CloseHandle(event)?;
        MessageBoxW(None, w!("Wide"), w!("Caption"), MB_OK);
    }

    Ok(())
}
```

> For new, minimal projects, prefer generating a focused binding with
> [`windows-bindgen`](windows-bindgen.md) or composing the smaller `windows-*`
> crates documented here.
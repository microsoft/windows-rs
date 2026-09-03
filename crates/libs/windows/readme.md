## windows

The [windows](https://crates.io/crates/windows) crate provides Rust bindings for Win32, COM, and
WinRT APIs generated from
[Windows metadata](https://github.com/microsoft/windows-rs/tree/master/crates/libs/default).

* [Getting
  started](https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows.md)

Start by adding the following to your Cargo.toml file:

```toml
[dependencies.windows]
version = "0.100"
features = [
    "Data_Xml_Dom",
    "handleapi",
    "synchapi",
    "winuser",
]
```

Make use of any Windows APIs as needed:

```rust,no_run
use windows::{core::*, Data::Xml::Dom::*, Win32::*};

fn main() -> Result<()> {
    let doc = XmlDocument::new()?;
    doc.LoadXml(h!("<html>hello world</html>"))?;

    let root = doc.DocumentElement()?;
    assert!(root.NodeName()? == "html");
    assert!(root.InnerText()? == "hello world");

    unsafe {
        let event = CreateEventW(None, true, false, None);
        SetEvent(event).ok()?;
        WaitForSingleObject(event, 0);
        CloseHandle(event).ok()?;

        MessageBoxA(None, s!("Ansi"), s!("Caption"), MB_OK as u32);
        MessageBoxW(None, w!("Wide"), w!("Caption"), MB_OK as u32);
    }

    Ok(())
}
```

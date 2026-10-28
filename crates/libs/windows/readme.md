# windows

Typed Win32, COM, and WinRT bindings for Rust binary applications, including reference-counted
interfaces and HRESULT-based results.

- 📦 [crates.io](https://crates.io/crates/windows)
- 📖 [API reference](https://microsoft.github.io/windows-docs-rs/)
- 🧩 [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples/windows)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/windows)

Prefer a [focused crate][crates] when one covers the task. Reusable libraries should generate
remaining private bindings with [windows-bindgen](https://docs.rs/windows-bindgen).
Use `windows` when a binary benefits from a broad, pre-generated API.

## Get started

Find an API by its native name in the reference and enable the listed feature:

```toml
[dependencies]
windows = { version = "0.100", features = ["Data_Xml_Dom"] }
```

```rust,no_run
use windows::{core::*, Data::Xml::Dom::XmlDocument};

fn main() -> Result<()> {
    init_mta()?;
    let document = XmlDocument::new()?;
    document.LoadXml(h!("<message>Hello, Windows!</message>"))?;
    println!("{}", document.GetXml()?.display());
    Ok(())
}
```

WinRT features follow namespaces, such as `Data_Xml_Dom`. Win32 features follow header groups,
such as `handleapi` and `synchapi`, with APIs under `windows::Win32`. Supporting types may need
additional features. Feature availability does not guarantee availability on the running OS.

## Read the Rust signature

The projection can turn out parameters into return values, pointer/count pairs into slices, and
HRESULTs into `Result`. Read the generated signature rather than translating a C call by hand.

| Concern | Rule |
| --- | --- |
| Interfaces | Clone to retain a reference; use `Interface::cast` for `QueryInterface`. |
| Handles | Follow ownership rules and use the matching release function. |
| Errors | Use `?` for `Result`; check other values with the API's documented failure sentinel. |
| Last error | Capture `Error::from_thread()` immediately after a last-error failure. |
| Strings | Use `h!` for WinRT and `w!` for UTF-16 literals; check narrow-string encodings. |

Keep unsafe calls inside a small wrapper that upholds pointer, lifetime, threading, and
initialization requirements. Put owned resources in guards before further fallible work.

See [windows-core](https://docs.rs/windows-core) for COM setup and
[windows-strings](https://docs.rs/windows-strings) for strings and buffers.
Choose [windows-sys](https://crates.io/crates/windows-sys) for raw FFI instead.

[crates]: https://github.com/microsoft/windows-rs/blob/master/docs/readme.md

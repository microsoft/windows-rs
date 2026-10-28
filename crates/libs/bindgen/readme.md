# windows-bindgen

Generate a focused set of Rust bindings from Windows metadata (`.winmd`). Bundled metadata covers
Win32 and WinRT; custom metadata can describe additional APIs.

- 📦 [crates.io](https://crates.io/crates/windows-bindgen)
- 📖 [docs.rs](https://docs.rs/windows-bindgen)
- 🧩 [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples/bindgen)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/bindgen)

## Generate bindings

For an application, add the generator as a build dependency and the raw bindings' runtime
dependency:

```toml
[dependencies]
windows-link = "0.100"

[build-dependencies]
windows-bindgen = "0.100"
```

In `build.rs`:

```rust,no_run
use std::path::PathBuf;

fn main() {
    let output = PathBuf::from(std::env::var_os("OUT_DIR").unwrap()).join("bindings.rs");
    windows_bindgen::builder()
        .output(output)
        .filter("GetTickCount64")
        .flat()
        .sys()
        .write();
}
```

In `src/main.rs`:

```rust,ignore
mod bindings {
    include!(concat!(env!("OUT_DIR"), "\\bindings.rs"));
}

fn main() {
    println!("{}", unsafe { bindings::GetTickCount64() });
}
```

For a published library, generate and commit bindings from a separate tool instead of running
the generator in consumer builds. Install `rustfmt` to format generated source.

## Select APIs

Filters select native names, including any `A` or `W` suffix. Signature dependencies are included
automatically. Qualify names when more than one namespace defines them.

| Filter | Selects |
| --- | --- |
| `GetTickCount64` | A function |
| `HWND` | A type |
| `Windows.Win32.GetTickCount64` | A qualified Win32 function |
| `Windows.Foundation` | A WinRT namespace |
| `Windows.Storage.Streams.IDataReader::ReadByte` | One interface method |
| `Windows.Storage.Streams.IDataReader::{ReadByte, ReadUInt32}` | Several methods |
| `Windows.Foundation.Uri::CreateUri` | A runtime-class factory method |

Use `.filters([...])` for several selections or `.filter_file("apis.txt")` for one filter per line.
Filter files ignore blank lines and `//` comments. Select the interface methods you need explicitly.
The bundled Win32 metadata uses `Windows.Win32`, not the umbrella crates' Cargo feature names.

## Choose output and inputs

| Option | Result |
| --- | --- |
| Default | Typed bindings using `windows-core` |
| `.sys()` | Raw bindings using `windows-link` |
| `.minimal()` | Bindings for hand-written wrappers |
| `.flat()` | One flat module instead of namespace modules |
| `.implement(name)` | Implementation traits for an interface |

`sys` and `minimal` are mutually exclusive. Add any focused support crates referenced by the output.

With no input, bindgen uses bundled Windows metadata. `.input("component.winmd")` replaces that
default; add `.input_default()` when the component also references Windows definitions.
Use [windows-rdl](https://docs.rs/windows-rdl) to compile RDL or
[windows-clang](https://docs.rs/windows-clang) to start from headers. Upgrade metadata producers
and binding generators together.

The [VSS sample](https://github.com/microsoft/windows-rs/tree/master/crates/samples/bindgen/vss_backup)
uses typed COM bindings. The
[robot component](https://github.com/microsoft/windows-rs/tree/master/crates/samples/robot/component)
uses custom metadata and implementation traits.

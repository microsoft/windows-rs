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
windows-link = "0.100.1"

[build-dependencies]
windows-bindgen = "0.101"
```

In `build.rs`:

```rust,no_run
use windows_bindgen::*;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    builder().filter("GetTickCount64").sys().write();
}
```

Without `.output(...)`, bindgen writes `bindings.rs` in Cargo's `OUT_DIR`. An explicit output is
required outside Cargo when `OUT_DIR` is unset or empty, and always for `.package()`. Use distinct
explicit paths when generating multiple files.

Bindgen does not emit Cargo dependency-tracking directives. The example above watches only
`build.rs`; declare any other inputs your script reads, including metadata and filter files, with
`cargo:rerun-if-changed`.

In `src/main.rs`:

```rust,ignore
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

fn main() {
    println!("{}", unsafe { bindings::GetTickCount64() });
}
```

For a published library, generate and commit bindings from a separate tool instead of running
the generator in consumer builds. Install `rustfmt` to format generated source.

## Select APIs

Filters select native names, including any `A` or `W` suffix. Signature dependencies are included
automatically. Qualify names when more than one namespace defines them. Output is a flat list of
items; selecting the same name from two namespaces fails to compile unless you exclude one or use
`.modules()`.

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
| Default | Minimal typed bindings using `windows-core` |
| `.full()` | Adds class wrappers, inherited forwarders, handle and function wrappers |
| `.sys()` | Raw bindings using `windows-link` |
| `.modules()` | One module per namespace instead of a single flat list |
| `.implement(name)` | Implementation traits for an interface |

`sys` and `full` are mutually exclusive. Add any focused support crates referenced by the output.
Full mode adds implementation traits for interfaces not marked `ExclusiveTo`. Otherwise select them
with `.implement(name)` or `.implement_all()`; an explicit selection also replaces the full-mode
default. Select the base or required interfaces too when implementing an interface hierarchy.

With no input, bindgen uses bundled Windows metadata. `.input("component.winmd")` replaces that
default; add `.input_default()` when the component also references Windows definitions.
Use [windows-rdl](https://docs.rs/windows-rdl) to compile RDL or
[windows-clang](https://docs.rs/windows-clang) to start from headers. Upgrade metadata producers
and binding generators together.

The [VSS sample](https://github.com/microsoft/windows-rs/tree/master/crates/samples/bindgen/vss_backup)
uses typed COM bindings.

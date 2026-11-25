# windows-bindgen

Generate a focused set of Rust bindings from Windows metadata (`.winmd`). Bundled metadata covers
Win32 and WinRT; custom metadata can describe additional APIs.

- 📦 [crates.io](https://crates.io/crates/windows-bindgen)
- 📖 [docs.rs](https://docs.rs/windows-bindgen)
- 🧩 [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples/bindgen)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/bindgen)

## Generate bindings

For an application, add the generator as a build dependency and `windows-core` for the default
typed bindings:

```toml
[dependencies]
windows-core = "0.101"

[build-dependencies]
windows-bindgen = "0.101"
```

In `build.rs`:

```rust,no_run
use windows_bindgen::*;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    builder().filter("GetTickCount64").write();
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
| `GetTickCount64`, `S_OK`, `HWND` | A function, constant, or type |
| `ISequentialStream` | An interface and its methods |
| `ISequentialStream::{}` | The interface without its callable methods |
| `Uri` | A runtime class, its methods, and its factories |
| `Uri::{}` | The class identity and default interface without selecting its methods |
| `Windows.Win32.GetTickCount64` | A qualified Win32 function |
| `Windows.Foundation` | A namespace and its descendants |
| `IDataReader::ReadByte` | One interface method |
| `IDataReader::{ReadByte, ReadUInt32}` | Several methods |
| `IDataReader::ByteOrder` | A property's getter and setter |
| `IDataReader::get_ByteOrder` | Only the getter |
| `IObservableVector::VectorChanged` | An event's add and remove methods |
| `Uri::CreateUri` | A runtime-class factory method |
| `PropertySet::CreateInstance` | A runtime class's default constructor |
| `AsyncStatus::{Completed, Error}` | Selected enum variants |
| `AsyncStatus::{}` | An enum without variants |
| `Windows::Foundation::{Uri::{CreateUri}, AsyncStatus}` | Nested groups |
| `!Windows.Foundation.Collections` | Exclude a namespace and its descendants |
| `!ISequentialStream` | Exclude a type |
| `!ISequentialStream::Write` | Exclude a member |
| `!AsyncStatus::Error` | Exclude an enum variant |

Dots and `::` both separate path segments. Groups abbreviate separate filters; `*` wildcards are
not supported. Use native member or overload names, not renamed Rust methods.

An interface reached only through a signature can be a name-only declaration. Select its name
to include its methods, or select individual methods. An explicit `Interface::{}` is useful when
you only pass, store, or cast the interface; it does not remove methods requested by other filters.

Rule order does not matter: more specific type/namespace rules win, with exclusions winning ties.
Member and variant selections narrow whole-type selections; excluded members and variants win.
Exclusions do not repair dependent selections; contradictory filters can generate bindings that
fail to compile.
See [`Bindgen::filter`](https://docs.rs/windows-bindgen/latest/windows_bindgen/struct.Bindgen.html#method.filter)
for details.

Use `.filters([...])` for several selections or `.filter_file("apis.txt")` for a text file.
Files separate filters with whitespace and support multiline groups, trailing commas, and `//`
comments:

```text
ISequentialStream::{
    Read,
    Write, // Include both methods
}
```

The bundled Win32 metadata uses `Windows.Win32`, not the umbrella crates' Cargo feature names.

## Choose output and inputs

| Option | Result |
| --- | --- |
| Default | Minimal typed bindings using `windows-core` |
| `.full()` | Adds class wrappers, inherited forwarders, handle and function wrappers |
| `.sys()` | Raw bindings using `windows-link` |
| `.modules()` | One module per namespace instead of a single flat list |
| `.implement("Windows.Win32.ISequentialStream")` | Implementation traits for a selected interface |

For raw bindings, add `.sys()` and use `windows-link = "0.100.1"` instead of `windows-core`.
`sys` and `full` are mutually exclusive. Add any focused support crates referenced by typed output.
For example, generated async signatures use `windows-future`; add it as a runtime dependency
(`windows-future = "0.101"`).

When migrating from `windows`, `.full()` preserves the function wrappers that turn HRESULTs and
output parameters into `Result` values. Minimal bindings expose raw Win32 function signatures;
their WinRT methods use `&str` inputs and `String` outputs instead of `HSTRING`.
The [COM URI sample](https://github.com/microsoft/windows-rs/tree/master/crates/samples/bindgen/com-uri)
uses full bindings, while the
[data-protection sample](https://github.com/microsoft/windows-rs/tree/master/crates/samples/bindgen/data-protection)
uses minimal WinRT bindings and `windows-future`. Both scope native-name lint expectations to the
generated bindings module.

Full mode adds implementation traits for interfaces not marked `ExclusiveTo`. Otherwise select them
with `.implement(name)` or `.implement_all()`; an explicit selection also replaces the full-mode
default. Implementation selectors use dotted qualified names or namespace prefixes, not bare
interface names. Select the base or required interfaces too when implementing an interface hierarchy.
Named implementation selectors reject member filters on matching interfaces; select the whole
interface or `::{}`. `.implement_all()` allows member filters but emits the complete implementation ABI.
In minimal mode, combine a COM interface's `::{}` filter with `.implement(...)` to generate its
implementation trait and vtable without caller wrappers.

With no input, bindgen uses bundled Windows metadata. `.input("component.winmd")` replaces that
default; add `.input_default()` when the component also references Windows definitions.
Use [windows-rdl](https://docs.rs/windows-rdl) to compile RDL or
[windows-clang](https://docs.rs/windows-clang) to start from headers. Upgrade metadata producers
and binding generators together.

The [VSS sample](https://github.com/microsoft/windows-rs/tree/master/crates/samples/bindgen/vss_backup)
uses typed COM bindings.

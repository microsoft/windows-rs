# windows-default

Embedded Windows metadata for build tools, available without locating an SDK or separate files.

- 📦 [crates.io](https://crates.io/crates/windows-default)
- 📖 [docs.rs](https://docs.rs/windows-default)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/default)

## Get started

```toml
[dependencies]
windows-default = "0.101"
```

To write the bundled metadata for a tool that accepts files:

```rust,no_run
use windows_default::{WIN32, WINRT};

std::fs::write("Windows.winmd", WINRT).unwrap();
std::fs::write("Windows.Win32.winmd", WIN32).unwrap();
```

| Static | Contents |
| --- | --- |
| `WINRT` | Windows Runtime contracts |
| `WIN32` | Windows SDK and WDK APIs |

Most generators should use `input_default()` on
[windows-bindgen](https://docs.rs/windows-bindgen) or `reference_default()` on the
[windows-rdl](https://docs.rs/windows-rdl) reader rather than load these bytes themselves.

For a custom inspector, pass each slice's `to_vec()` to
[`windows_metadata::reader::File::new`](https://docs.rs/windows-metadata/latest/windows_metadata/reader/struct.File.html#method.new),
then combine the files in a `reader::Index`.

The bytes describe APIs; they do not supply Windows DLLs or guarantee availability on the running
operating system. Keep metadata dependencies in generators rather than application runtime code.

## Windows metadata reader

The [windows-metadata](https://crates.io/crates/windows-metadata) crate provides a fast reader for Windows metadata files based on the ECMA-335 file format.

* [Getting started](https://kennykerr.ca/rust-getting-started/)
* [Samples](https://github.com/microsoft/windows-rs/tree/0.55.0/crates/samples) <!-- link to samples for upcoming release -->
* [Releases](https://github.com/microsoft/windows-rs/releases)

Start by adding the following to your Cargo.toml file:

```toml
[dependencies.windows-metadata]
version = "0.55.0"
```

Read metadata as needed:

```rust,no_run
use windows_metadata::*;

fn main() {
    let bytes = std::fs::read(r#"C:\Windows\System32\WinMetadata\Windows.Foundation.winmd"#)
        .expect("File not found");

    let file = File::new(bytes).expect("Invalid metadata");

    let reader = Reader::new(vec![file]);

    for def in reader.get_type_def("Windows.Foundation", "IAsyncInfo") {
        println!("{}", def.name());

        for method in def.methods() {
            println!("{}", method.name());
        }
    }
}
```

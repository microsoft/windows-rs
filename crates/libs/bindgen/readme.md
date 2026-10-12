## windows-bindgen

The [windows-bindgen](https://crates.io/crates/windows-bindgen) crate generates Rust bindings from
Windows metadata.

* [Guide](https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-bindgen.md)

Add the generator as a build dependency and the generated code's runtime dependency:

```toml
[dependencies.windows-link]
version = "0.100"

[build-dependencies.windows-bindgen]
version = "0.100"
```

Generate a flat file in `OUT_DIR` from `build.rs`:

```rust,no_run
use std::path::Path;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    windows_bindgen::builder()
        .output(Path::new(&out_dir).join("bindings.rs"))
        .filter("GetTickCount")
        .flat()
        .sys()
        .write();
}
```

Filters name the functions, constants, or types you need. Signature dependencies are included
automatically. See the [guide] for qualified names, member filters, and filter files.

Use the generated bindings from `src/main.rs`:

```rust,ignore
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

fn main() {
    unsafe {
        println!("{}", bindings::GetTickCount());
    }
}
```

For a published library, generate and commit `src/bindings.rs` from a separate tool instead of
running the generator in consumer builds. Formatting is best effort; install `rustfmt` when
generating checked-in source.

[guide]:
  https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-bindgen.md#select-apis

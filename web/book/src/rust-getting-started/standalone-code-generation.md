# Standalone code generation

Even with a [choice between the windows and windows-sys crates](windows-or-windows-sys.md), some developers may prefer to use completely standalone bindings. The [windows-bindgen](https://crates.io/crates/windows-bindgen) crate lets you generate entirely standalone bindings for Windows APIs with a single function call that you can run from a test to automate the generation of bindings. This can help to reduce your dependencies while continuing to provide a sustainable path forward for any future API requirements you might have, or just to refresh your bindings from time to time to pick up any bug fixes automatically from Microsoft.

> **Warning**: Standalone code generation should only be used as a last resort for the most demanding scenarios. It is much simpler to use the [windows-sys](https://crates.io/crates/windows-sys) crate and let Cargo manage this dependency. This `windows-sys` crate provides raw bindings, is heavily tested and widely used, and should not meaningfully impact your build time. 

Start by adding the following to your Cargo.toml file:

```toml
[dependencies.windows-targets]
version = "0.52"

[dev-dependencies.windows-bindgen]
version = "0.52"
```

The `windows-bindgen` crate is only needed for generating bindings and is thus a dev dependency only. The [windows-targets](https://crates.io/crates/windows-targets) crate is a dependency shared by the `windows` and `windows-sys` crates and only contains import libs for supported targets. This will ensure that you can link against any Windows API functions you may need. 

Write a test to generate bindings as follows:

```rust,no_run
#[test]
fn bindgen() {
    let args = [
        "--out",
        "src/bindings.rs",
        "--config",
        "flatten",
        "--filter",
        "Windows.Win32.System.SystemInformation.GetTickCount",
    ];

    windows_bindgen::bindgen(args).unwrap();
}
```

Make use of any Windows APIs as needed.

```rust,no_run,ignore
mod bindings;

fn main() {
    unsafe {
        println!("{}", bindings::GetTickCount());
    }
}
```

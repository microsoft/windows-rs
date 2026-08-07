## Windows metadata binding generator

The [windows-bindgen](https://crates.io/crates/windows-bindgen) crate generates Rust bindings from
Windows metadata.

* [Getting started](https://github.com/microsoft/windows-rs/blob/master/docs/readme.md)
* [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples)
* [Releases](https://github.com/microsoft/windows-rs/releases)

Add the generator as a build dependency and the generated code's runtime dependency:

```toml
[dependencies.windows-link]
version = "0.100"

[build-dependencies.windows-bindgen]
version = "0.100"
```

Generate bindings from `build.rs`:

```rust,no_run
let args = [
    "--out",
    "src/bindings.rs",
    "--flat",
    "--sys",
    "--filter",
    "GetTickCount",
];
windows_bindgen::bindgen(args);
```

Use `windows_bindgen::bindgen(["--etc", "bindings.txt"])` when the commands live in a text file.
For a filter-only file, use `Bindgen::filter_file`/`filter_files` or `--filter-file`.

Variadic native exports are emitted only by `--sys`, where the generated declaration retains the
literal `...` tail. Default and minimal bindings omit them rather than exposing a callable
fixed-prefix wrapper.

Parameter direction uses the shared raw facts from `windows-metadata`, but Rust projection policy
stays local. `Input` and `Unspecified` take the input-only branch, `InputOutput` keeps mutable slice
shapes, and parameters marked only `Output` keep raw pointer/count parameters so callers may
provide uninitialized storage. A trailing retval must be output-only, required, non-reserved,
uncounted, and pointer-shaped. The existing void-pointee and size limits apply only to unmarked
heuristic candidates.

And then use the bindings as follows:

```rust,ignore
mod bindings;

unsafe {
    println!("{}", bindings::GetTickCount());
}
```

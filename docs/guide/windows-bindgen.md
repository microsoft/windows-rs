# windows-bindgen

> The code generator that turns Windows metadata (`.winmd`) into Rust bindings.

- 📦 [crates.io](https://crates.io/crates/windows-bindgen)
- 📖 [API reference (docs.rs)](https://docs.rs/windows-bindgen)
- 🛠 [Internals](../internals/windows-bindgen.md)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/bindgen)

## Overview

`windows-bindgen` generates Rust bindings from Windows metadata. It powers the
`windows` and `windows-sys` crates and is also usable directly — typically from
a `build.rs` — to generate a minimal, project-specific set of bindings. You pass
the same arguments the command-line tool accepts: `--out` for the destination,
a `--filter` selecting the APIs you need, and flags such as `--flat` and
`--sys` to control the shape of the output.

## Example

Generate a flat, `windows-sys`-style binding for a single function:

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

The generated module can then be included and called like any other Rust code.
See the [internals page](../internals/windows-bindgen.md) for how the generator
fits into the codegen pipeline.
# windows-bindgen

> The code generator that turns Windows metadata (`.winmd`) into Rust bindings.

- 📦 [crates.io](https://crates.io/crates/windows-bindgen)
- 📖 [API reference (docs.rs)](https://docs.rs/windows-bindgen)
- 🚀 [Getting started](../../crates/libs/bindgen/readme.md)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/bindgen)

`windows-bindgen` generates Rust bindings from Windows metadata. It powers the
`windows` and `windows-sys` crates and is also usable directly — typically from a
`build.rs` — to generate a minimal, project-specific set of bindings. You pass
the same arguments the command-line tool accepts: `--out` for the destination, a
`--filter` selecting the APIs you need, and flags such as `--flat` and `--sys` to
control the shape of the output.

## How it's built

`windows-bindgen` is hand-written — it is the generator the other crates depend
on, not generated itself. It reads ECMA-335 metadata through
[`windows-metadata`](windows-metadata.md); the default `.winmd` inputs live in
`crates/libs/bindgen/default`. It is driven by `tool_bindings` (the per-crate
`.txt` filters in `crates/tools/bindings/src`) and by `tool_package` (which
produces the published `windows` and `windows-sys` crates).

## Testing

Verified by the dedicated test crates `test_bindgen`, `test_rdl`, and
`test_clang`.

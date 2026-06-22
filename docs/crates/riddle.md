# riddle

> The Windows metadata compiler.

- 📦 [crates.io](https://crates.io/crates/riddle)
- 📖 [docs.rs](https://docs.rs/riddle)
- 🚀 [Getting started](../../crates/libs/riddle/readme.md)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/riddle)

`riddle` is the command-line Windows metadata compiler. It reads
[RDL](windows-rdl.md) source and emits ECMA-335 `.winmd` files, acting as the
authoring front end for the metadata that drives code generation. It is a
developer tool rather than a runtime library.

## How it's built

Builds on `windows-rdl` and `windows-metadata`. Internal tooling, not intended
for direct application use.

## Testing

Run `cargo test -p riddle`; see also the workspace test crates.

# riddle

> The Windows metadata compiler.

- 📦 [crates.io](https://crates.io/crates/riddle)
- 📖 [API reference (docs.rs)](https://docs.rs/riddle)
- 🛠 [Internals](../internals/riddle.md)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/riddle)

## Overview

`riddle` is the command-line Windows metadata compiler. It reads
[RDL](windows-rdl.md) source and emits ECMA-335 `.winmd` files, acting as the
authoring front end for the metadata that drives code generation. It is a
developer tool rather than a runtime library.

See the [internals page](../internals/riddle.md) for how `riddle` is used in the
metadata pipeline.
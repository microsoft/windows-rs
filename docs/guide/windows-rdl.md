# windows-rdl

> A parser for RDL (Rust Definition Language) and an ECMA-335 metadata generator.

- 📦 [crates.io](https://crates.io/crates/windows-rdl)
- 📖 [API reference (docs.rs)](https://docs.rs/windows-rdl)
- 🛠 [Internals](../internals/windows-rdl.md)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/rdl)

## Overview

`windows-rdl` parses RDL — a compact, Rust-like syntax for describing Windows
APIs — and produces ECMA-335 `.winmd` metadata. It lets API definitions be
authored in a readable text format and then compiled into the same metadata that
[`windows-bindgen`](windows-bindgen.md) consumes.

RDL sits at the front of the metadata authoring pipeline; the
[internals page](../internals/windows-rdl.md) describes how it is used to build
and maintain the bindings in this repository.
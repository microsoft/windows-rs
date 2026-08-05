# windows-tracing-macros

> Procedural macros used by `windows-tracing`.

- 📦 [crates.io](https://crates.io/crates/windows-tracing-macros)
- 📖 [docs.rs](https://docs.rs/windows-tracing-macros)
- 🚀 [Getting started](../../crates/libs/tracing-macros/readme.md)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/tracing-macros)

`windows-tracing-macros` provides the `define_provider!` and `write_event!` macros. It is **part of
[`windows-tracing`](windows-tracing.md)** and exists as a separate crate only because Rust requires
procedural macros to ship in their own `proc-macro` crate. `windows-tracing` re-exports both macros,
so applications depend on `windows-tracing` and rarely name this crate directly.

---

## Internal documentation

The remainder of this page covers how the crate is built and maintained. It is for contributors and
is **not needed to use `windows-tracing-macros`**.

### How it's built

A `proc-macro` crate. The macros parse static provider and event declarations, encode TraceLogging
metadata during compilation, and emit typed field bindings and fixed-size descriptor arrays. It
uses `syn`/`quote`/`proc-macro2`.

### Testing

Run `cargo test -p windows-tracing-macros`. `test_tracing` verifies the expanded code with an ETL
and TDH roundtrip.

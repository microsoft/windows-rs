# windows-interface — internals

> Maintenance notes for `windows-interface`. For usage, see the [guide](../guide/windows-interface.md).

- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/interface)

## How it's built

A `proc-macro` crate. The crate-level docs live inline in `src/lib.rs`. Uses `syn`/`quote`/`proc-macro2`.

## Testing

_See the workspace test crates and `cargo test -p windows-interface`._

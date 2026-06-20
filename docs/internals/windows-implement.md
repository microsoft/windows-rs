# windows-implement — internals

> Maintenance notes for `windows-implement`. For usage, see the [guide](../guide/windows-implement.md).

- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/implement)

## How it's built

A `proc-macro` crate. The crate-level docs live inline in `src/lib.rs`. Uses `syn`/`quote`/`proc-macro2` to parse the attribute and emit the vtable plumbing.

## Testing

_See the workspace test crates and `cargo test -p windows-implement`._

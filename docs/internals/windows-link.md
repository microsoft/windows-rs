# windows-link — internals

> Maintenance notes for `windows-link`. For usage, see the [guide](../guide/windows-link.md).

- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/link)

## How it's built

Entirely hand-written — a single `link!` macro with per-architecture expansions. No generated code.

## Testing

_See the workspace test crates and `cargo test -p windows-link`._

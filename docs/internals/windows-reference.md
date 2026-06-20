# windows-reference — internals

> Maintenance notes for `windows-reference`. For usage, see the [guide](../guide/windows-reference.md).

- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/reference)

## How it's built

`src/bindings.rs` is generated; the `IReference<T>` implementation is hand-written.

## Testing

_See the workspace test crates and `cargo test -p windows-reference`._

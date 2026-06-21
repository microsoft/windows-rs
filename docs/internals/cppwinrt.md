# cppwinrt — internals

> Maintenance notes for `cppwinrt`. For usage, see the [guide](../guide/cppwinrt.md).

- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/cppwinrt)

## How it's built

Packages the prebuilt `cppwinrt.exe` and exposes a path to it from build scripts. No generated Rust bindings.

## Testing

_See the workspace test crates and `cargo test -p cppwinrt`._

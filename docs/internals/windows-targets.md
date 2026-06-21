# windows-targets — internals

> Maintenance notes for `windows-targets`. For usage, see the [guide](../guide/windows-targets.md).

- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/targets)

## How it's built

Hand-written shim that selects the matching `windows_<arch>_<abi>` import-lib crate. The precursor to `windows-link`.

## Testing

_See the workspace test crates and `cargo test -p windows-targets`._

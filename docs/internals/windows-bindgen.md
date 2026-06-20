# windows-bindgen — internals

> Maintenance notes for `windows-bindgen`. For usage, see the [guide](../guide/windows-bindgen.md).

- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/bindgen)

## How it's built

`windows-bindgen` is hand-written — it is the generator the other crates depend
on, not generated itself. It reads ECMA-335 metadata through
[`windows-metadata`](../guide/windows-metadata.md); the default `.winmd` inputs
live in `crates/libs/bindgen/default`. It is driven by `tool_bindings` (the
per-crate `.txt` filters in `crates/tools/bindings/src`) and by `tool_package`
(which produces the published `windows` and `windows-sys` crates).

## Testing

Verified by the dedicated test crates `test_bindgen`, `test_rdl`, and
`test_clang`.

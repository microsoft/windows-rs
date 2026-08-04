# windows-default

> The default Windows metadata used by the build-time crates.

- [crates.io](https://crates.io/crates/windows-default)
- [docs.rs](https://docs.rs/windows-default)
- [Getting started](../../crates/libs/default/readme.md)
- [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/default)

`windows-default` embeds the canonical Windows Runtime and Windows API metadata as byte slices.
Build tools use `WINRT` and `WIN32` when they need the standard metadata without locating separate
`.winmd` files.

The crate has no dependencies. `tool_winrt` generates `Windows.winmd`, and `tool_win32` generates
`Windows.Win32.winmd`.

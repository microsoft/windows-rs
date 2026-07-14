These `.winmd` files provide the default metadata for the Windows API, bundled into
`windows-bindgen` and selected with `--in default`. To view the metadata, use a tool
like [ILSpy](https://github.com/icsharpcode/ILSpy).

## `Windows.Win32.winmd`

Generated in-house by `tool_win32` (`cargo run -p tool_win32`) directly from the
Windows SDK C/C++ headers via `windows-clang`, with `windows-metadata` performing the
per-architecture merge. The committed `metadata/win32/*.rdl` corpus is the
human-reviewable snapshot; this winmd is derived from it.

## `Windows.Wdk.winmd`

Generated in-house by `tool_wdk` (`cargo run -p tool_wdk`) from the WDK kernel-mode
headers, in the same flat `Windows.Win32` namespace and *additive* over
`Windows.Win32.winmd` (types the Win32 winmd already defines are dropped). The
committed `metadata/wdk/*.rdl` corpus is the snapshot.

## `Windows.winmd`

The WinRT metadata, generated in-house by `tool_windows` (`cargo run -p tool_windows`)
by merging the per-contract `.winmd` files from the Windows SDK Contracts NuGet package
with `windows-metadata` (the same merger `tool_win32`/`tool_wdk` use), replacing the
external `mdmerge` tool.

For the source package versions and how to update them, see
[`docs/dependencies.md`](../../../../docs/dependencies.md).

---

As with everything else in this repo, the `.winmd` files in this folder are licensed via MIT or Apache-2.0.

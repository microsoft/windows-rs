These `.winmd` files provide the default metadata for the Windows API, bundled into
`windows-bindgen` and selected with `--in default`. To view the metadata, use a tool
like [ILSpy](https://github.com/icsharpcode/ILSpy).

## `Windows.Win32.winmd`

Generated in-house by `tool_win32` (`cargo run -p tool_win32`) directly from the
Windows SDK C/C++ headers via `windows-clang`, with `windows-metadata` performing the
per-architecture merge. The committed `metadata/win32/*.rdl` corpus is the
human-reviewable snapshot; this winmd is derived from it.

- Headers: `Microsoft.Windows.SDK.CPP` / `Microsoft.Windows.SDK.CPP.<arch>`
- Version: `10.0.28000.2270` (pinned in `crates/tools/win32/src/main.rs`)

## `Windows.Wdk.winmd`

Generated in-house by `tool_wdk` (`cargo run -p tool_wdk`) from the WDK kernel-mode
headers, in the same flat `Windows.Win32` namespace and *additive* over
`Windows.Win32.winmd` (types the Win32 winmd already defines are dropped). The
committed `metadata/wdk/*.rdl` corpus is the snapshot.

- Headers: `Microsoft.Windows.WDK.x64`
- Version: `10.0.28000.1839` (pinned in `crates/tools/wdk/src/main.rs`)

## `Windows.winmd`

The WinRT metadata, generated in-house by `tool_windows` (`cargo run -p tool_windows`)
by merging the per-contract `.winmd` files from the Windows SDK Contracts NuGet package
with `windows-metadata` (the same merger `tool_win32`/`tool_wdk` use), replacing the
external `mdmerge` tool.

- Source: <https://www.nuget.org/packages/Microsoft.Windows.SDK.Contracts>
- Version: `10.0.26100.7705` (pinned in `crates/tools/windows/src/main.rs`)

---

As with everything else in this repo, the `.winmd` files in this folder are licensed via MIT or Apache-2.0.

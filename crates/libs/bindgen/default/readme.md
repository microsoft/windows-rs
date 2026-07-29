These `.winmd` files provide the default metadata for the Windows API, bundled into
`windows-bindgen` and selected with `--in default`. To view the metadata, use a tool like
[ILSpy](https://github.com/icsharpcode/ILSpy).

## `Windows.Win32.winmd`

The single flat `Windows.Win32` metadata for the whole native API surface, owned by `tool_wdk`
(`cargo run -p tool_wdk`). `tool_win32` (`cargo run -p tool_win32`) scrapes the Windows SDK C/C++
headers via `windows-clang` into the committed `metadata/win32/*.rdl` snapshot (the human-reviewable
source of truth). `tool_wdk` re-derives the um winmd from that RDL, scrapes the WDK kernel-mode
headers into `metadata/wdk/*.rdl` (additive over Win32, in the same flat namespace), and merges the
two winmds with `windows-metadata` — unioning same-named enums so a value type a um header truncates
(for example `FILE_INFORMATION_CLASS`) carries the km definition's full member set in one enum. This
winmd is derived from the two RDL corpora; `tool_roundtrip` re-validates the round-trip without the
SDK.

- SDK headers: `Microsoft.Windows.SDK.CPP` / `Microsoft.Windows.SDK.CPP.<arch>`, version
  `10.0.28000.2270` (pinned in `crates/tools/win32/src/main.rs`)
- WDK headers: `Microsoft.Windows.WDK.x64`, version `10.0.28000.1839` (pinned in
  `crates/tools/wdk/src/main.rs`)

## `Windows.winmd`

The WinRT metadata, generated in-house by `tool_winrt` (`cargo run -p tool_winrt`) by merging the
per-contract `.winmd` files from the Windows SDK Contracts NuGet package with `windows-metadata`
(the same merger `tool_win32`/`tool_wdk` use, replacing the external `mdmerge` tool), decompiling
the result to the committed `metadata/winrt` RDL snapshot, and compiling that snapshot back into
this winmd. As with `metadata/win32` and `metadata/wdk`, the RDL is the reviewable source of truth:
a WinRT metadata change shows up as a readable RDL `git diff`, and `tool_roundtrip` re-validates the
round-trip without the SDK.

- Source: <https://www.nuget.org/packages/Microsoft.Windows.SDK.Contracts>
- Version: `10.0.28000.2270` (pinned in `crates/tools/winrt/src/main.rs`)

---

As with everything else in this repo, the `.winmd` files in this folder are licensed via MIT or
Apache-2.0.

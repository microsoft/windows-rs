## Default Windows metadata

The [windows-default](https://crates.io/crates/windows-default) crate provides the default metadata
for Windows APIs as embedded byte slices. Build tools can use [`WINRT`] and [`WIN32`] without
locating or distributing separate `.winmd` files.

Most callers should use `.input_default()` on `windows-bindgen` or `windows-csharp`, and
`.reference_default()` on `windows-rdl` or `windows-clang`. Use this crate directly when
implementing another tool that accepts metadata bytes.

Programs that link this crate include both metadata files in their binary.

To view the metadata, use a tool like [ILSpy](https://github.com/icsharpcode/ILSpy).

## `Windows.Win32.winmd`

The single flat `Windows.Win32` metadata for the whole native API surface, owned by `tool_win32`
(`cargo run -p tool_win32`). It runs in three phases: (A) scrape the Windows SDK C/C++ headers via
`windows-clang` into the committed `metadata/win32/*.rdl` snapshot (the human-reviewable source of
truth) and an uncommitted um winmd under `target`; (B) scrape the WDK kernel-mode headers into
`metadata/wdk/*.rdl` (additive over Win32, in the same flat namespace) and an uncommitted km winmd,
resolving against phase A's um winmd; (C) merge the two winmds with `windows-metadata` - unioning
same-named enums so a value type a um header truncates (for example `FILE_INFORMATION_CLASS`)
carries the km definition's full member set in one enum. This winmd is derived from the two RDL
directories; `tool_roundtrip` re-validates the round-trip without the SDK.

- SDK headers: `Microsoft.Windows.SDK.CPP` / `Microsoft.Windows.SDK.CPP.<arch>`, version
  `10.0.28000.2270` (pinned in `crates/tools/win32/src/main.rs`)
- WDK headers: `Microsoft.Windows.WDK.x64`, version `10.0.28000.1839` (pinned in
  `crates/tools/win32/src/km.rs`)

## `Windows.winmd`

The WinRT metadata, generated in-house by `tool_winrt` (`cargo run -p tool_winrt`) by merging the
per-contract `.winmd` files from the Windows SDK Contracts NuGet package with `windows-metadata`
(the same merger `tool_win32` uses, replacing the external `mdmerge` tool), decompiling
the result to the committed `metadata/winrt` RDL snapshot, and compiling that snapshot back into
this winmd. As with `metadata/win32` and `metadata/wdk`, the RDL is the reviewable source of truth:
a WinRT metadata change shows up as a readable RDL `git diff`, and `tool_roundtrip` re-validates the
round-trip without the SDK.

- Source: <https://www.nuget.org/packages/Microsoft.Windows.SDK.Contracts>
- Version: `10.0.28000.2270` (pinned in `crates/tools/winrt/src/main.rs`)

---

As with everything else in this repo, the `.winmd` files in this crate are licensed via MIT or
Apache-2.0.

[`WINRT`]: https://docs.rs/windows-default/latest/windows_default/static.WINRT.html
[`WIN32`]: https://docs.rs/windows-default/latest/windows_default/static.WIN32.html

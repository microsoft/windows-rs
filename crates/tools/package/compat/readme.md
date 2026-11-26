# Package compatibility metadata

`tool-package` uses these pinned `.winmd` files to generate `windows` and `windows-sys` against
the current runtime crates while retaining their compatibility metadata. This is separate from
the metadata in `windows-default` used for focused bindings.

The files match `crates/libs/bindgen/default` at tag `71`, the metadata baseline for `windows`
0.62 and `windows-sys` 0.61.

| File | Source | Version |
| --- | --- | --- |
| `Windows.Win32.winmd` | <https://www.nuget.org/packages/Microsoft.Windows.SDK.Win32Metadata/> | `63.0.31` |
| `Windows.Wdk.winmd` | <https://www.nuget.org/packages/Microsoft.Windows.WDK.Win32Metadata/> | `0.13.25` |
| `Windows.winmd` | <https://www.nuget.org/packages/Microsoft.Windows.SDK.Contracts> | `10.0.26100.4948` |

These files use the Win32 metadata attribute vocabulary in `Windows.Win32.Foundation.Metadata`.
`windows-bindgen` recognizes it in `--package` mode and projects handles, enums, and related
types using the compatibility rules.

From the repository root, regenerate both packages with:

```console
cargo run -p tool-package --quiet
```

The tool reads `crates/tools/package/src/windows.txt` and `sys.txt`, then rewrites generated
sources and Cargo feature lists in `crates/libs/windows` and `crates/libs/sys`. Review the diff;
do not edit generated files or replace these metadata inputs as part of a routine SDK refresh.

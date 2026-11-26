These `.winmd` files are the metadata used to generate the last `windows` 0.62 and `windows-sys`
0.61 releases (release 71). `tool-package` uses them to generate interim releases of those crates
against the current `windows-core` and related crates, to ease migration of existing dependents.

The files are identical to `crates/libs/bindgen/default` at the `71` tag.

| File | Source | Version |
| --- | --- | --- |
| `Windows.Win32.winmd` | <https://www.nuget.org/packages/Microsoft.Windows.SDK.Win32Metadata/> | `63.0.31` |
| `Windows.Wdk.winmd` | <https://www.nuget.org/packages/Microsoft.Windows.WDK.Win32Metadata/> | `0.13.25` |
| `Windows.winmd` | <https://www.nuget.org/packages/Microsoft.Windows.SDK.Contracts> | `10.0.26100.4948` |

These files use the Win32 metadata attribute vocabulary in `Windows.Win32.Foundation.Metadata`.
`windows-bindgen` recognizes it in `--package` mode and projects handles, enums, and related
types as the 0.62 releases did.

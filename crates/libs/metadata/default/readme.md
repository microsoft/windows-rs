These `.winmd` files provide the default metadata for the Windows API. This is used to
generate the `windows` and `windows-sys` crates and are loaded using the `windows-metadata` crate's
`File::with_default` function when reading metadata directly. As with everything else in this repo,
the `.winmd` files in this folder are licensed via MIT or Apache-2.0.

## Windows.Win32.winmd
- Source: https://www.nuget.org/packages/Microsoft.Windows.SDK.Win32Metadata/
- Version: 40.0.14

## Windows.winmd
- Source: https://www.nuget.org/packages/Microsoft.Windows.SDK.Contracts
- Version: 10.0.22621.755

The Windows.winmd was created by merging the .winmd files from the second nuget package as follows

```
mdmerge -o out -i . -n 1
```

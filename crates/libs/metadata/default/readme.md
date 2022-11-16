These `.winmd` files together provide the default metadata for the Windows API. This is used when the
dependent crate or workspace has an empty or non-existent `.windows/winmd` directory or include
`.winmd` files that start with something other than "Windows.".

## Windows.Win32.winmd
- Source: https://www.nuget.org/packages/Microsoft.Windows.SDK.Win32Metadata/
- Version: 38.0.19

## Windows.winmd
- Source: https://www.nuget.org/packages/Microsoft.Windows.SDK.Contracts
- Version: 10.0.22000.196

The Windows.winmd was created by merging the .winmd files from the second nuget package as follows

```
mdmerge -o out -i . -n 1
```

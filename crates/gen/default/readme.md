These `.winmd` files together provide the default metadata for the Windows API. This is used when the
dependent crate or workspace has an empty or non-existent `.windows/winmd` directory or include
`.winmd` files that start with something other than "Windows.".

## Windows.Win32.winmd
- Source: https://www.nuget.org/packages/Microsoft.Windows.SDK.Win32Metadata/
- Version: 10.0.19041.202-preview

## Windows.WinRT.winmd
- Source: https://www.nuget.org/packages/Microsoft.Windows.SDK.Contracts
- Version: 10.0.19041.1

TODO: describe how the merged WinRT winmd is created.

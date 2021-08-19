These `.winmd` files together provide the default metadata for the Windows API. This is used when the
dependent crate or workspace has an empty or non-existent `.windows/winmd` directory or include
`.winmd` files that start with something other than "Windows.".

## Windows.Win32.winmd
- Source: https://www.nuget.org/packages/Microsoft.Windows.SDK.Win32Metadata/
- Version: 10.2.118-preview

## Windows.WinRT.winmd
- Source: https://www.microsoft.com/en-us/software-download/windowsinsiderpreviewSDK
- Version: 10.0.22000.0

The Windows.WinRT.winmd was created from the Windows SDK as follows:

```
C:\temp>for /r "C:\Program Files (x86)\Windows Kits\10\References\10.0.22000.0" %f in (*.winmd) do @copy "%f"

C:\temp>mdmerge -o out -i . -n 1
```

The first step collects all of the winmd files into a single folder. The second step merges them into a single .winmd file. 
This isn't strictly necessary but simplifies distribution and samples.

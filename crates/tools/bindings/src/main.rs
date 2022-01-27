fn main() -> std::io::Result<()> {
    let types = &[
        "Windows.Foundation.DateTime",
        "Windows.Foundation.IPropertyValue",
        "Windows.Foundation.IPropertyValueStatics",
        "Windows.Foundation.IReference",
        "Windows.Foundation.IStringable",
        "Windows.Foundation.Point",
        "Windows.Foundation.PropertyType",
        "Windows.Foundation.PropertyValue",
        "Windows.Foundation.Rect",
        "Windows.Foundation.Size",
        "Windows.Foundation.TimeSpan",
        "Windows.Win32.Foundation.BOOL",
        "Windows.Win32.Foundation.BSTR",
        "Windows.Win32.Foundation.CLASS_E_CLASSNOTAVAILABLE",
        "Windows.Win32.Foundation.CloseHandle",
        "Windows.Win32.Foundation.CO_E_NOTINITIALIZED",
        "Windows.Win32.Foundation.E_NOINTERFACE",
        "Windows.Win32.Foundation.E_OUTOFMEMORY",
        "Windows.Win32.Foundation.FARPROC",
        "Windows.Win32.Foundation.GetLastError",
        "Windows.Win32.Foundation.HANDLE",
        "Windows.Win32.Foundation.HINSTANCE",
        "Windows.Win32.Foundation.PSTR",
        "Windows.Win32.Foundation.PWSTR",
        "Windows.Win32.Foundation.S_OK",
        "Windows.Win32.Foundation.SysAllocStringLen",
        "Windows.Win32.Foundation.SysFreeString",
        "Windows.Win32.Foundation.SysStringLen",
        "Windows.Win32.Foundation.WIN32_ERROR",
        "Windows.Win32.Security.SECURITY_ATTRIBUTES",
        "Windows.Win32.System.Com.CoCreateGuid",
        "Windows.Win32.System.Com.CoTaskMemAlloc",
        "Windows.Win32.System.Com.CoTaskMemFree",
        "Windows.Win32.System.Com.GetErrorInfo",
        "Windows.Win32.System.Com.IAgileObject",
        "Windows.Win32.System.Com.IErrorInfo",
        "Windows.Win32.System.Com.SetErrorInfo",
        "Windows.Win32.System.Diagnostics.Debug.FORMAT_MESSAGE_OPTIONS",
        "Windows.Win32.System.Diagnostics.Debug.FormatMessageW",
        "Windows.Win32.System.LibraryLoader.FreeLibrary",
        "Windows.Win32.System.LibraryLoader.GetProcAddress",
        "Windows.Win32.System.LibraryLoader.LoadLibraryA",
        "Windows.Win32.System.Memory.GetProcessHeap",
        "Windows.Win32.System.Memory.HEAP_FLAGS",
        "Windows.Win32.System.Memory.HeapAlloc",
        "Windows.Win32.System.Memory.HeapFree",
        "Windows.Win32.System.Memory.HeapHandle",
        "Windows.Win32.System.Threading.CreateEventA",
        "Windows.Win32.System.Threading.SetEvent",
        "Windows.Win32.System.Threading.WaitForSingleObject",
        "Windows.Win32.System.WinRT.ILanguageExceptionErrorInfo",
        "Windows.Win32.System.WinRT.ILanguageExceptionErrorInfo2",
        "Windows.Win32.System.WinRT.IRestrictedErrorInfo",
        "Windows.Win32.System.WinRT.IWeakReference",
        "Windows.Win32.System.WinRT.IWeakReferenceSource",
    ];

    let mut tokens = "#![allow(non_snake_case, non_upper_case_globals, dead_code, non_camel_case_types, clippy::upper_case_acronyms, clippy::derivable_impls)]".to_string();
    let gen = bindgen::Gen { namespace: "Windows.", min_enum: true, min_inherit: true, flatten: true, ..Default::default() };

    for name in types {
        tokens += &bindgen::gen_type(name, &gen);
    }

    let mut path: std::path::PathBuf = metadata::workspace_dir().into();
    path.push("crates/libs/windows/src/core/bindings.rs");

    std::fs::write(&path, tokens)?;

    let mut cmd = ::std::process::Command::new("rustfmt");
    cmd.arg(&path);
    cmd.output()?;

    Ok(())
}

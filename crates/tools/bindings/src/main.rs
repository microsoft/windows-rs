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
        "Windows.Win32.Foundation.RPC_E_DISCONNECTED",
        "Windows.Win32.Foundation.JSCRIPT_E_CANTEXECUTE",
        "Windows.Win32.Foundation.FARPROC",
        "Windows.Win32.Foundation.GetLastError",
        "Windows.Win32.Foundation.HANDLE",
        "Windows.Win32.Foundation.HINSTANCE",
        "Windows.Win32.Foundation.S_OK",
        "Windows.Win32.Foundation.SysAllocStringByteLen",
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
        "Windows.Win32.System.Diagnostics.Debug.EncodePointer",
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
        "Windows.Win32.System.WinRT.IAgileReference",
        "Windows.Win32.System.WinRT.AgileReferenceOptions",
        "Windows.Win32.System.WinRT.RoGetAgileReference",
        "Windows.Win32.System.WinRT.ILanguageExceptionErrorInfo",
        "Windows.Win32.System.WinRT.ILanguageExceptionErrorInfo2",
        "Windows.Win32.System.WinRT.IRestrictedErrorInfo",
        "Windows.Win32.System.WinRT.IWeakReference",
        "Windows.Win32.System.WinRT.IWeakReferenceSource",
    ];

    let files = vec![metadata::reader2::File::new("crates/libs/metadata/default/Windows.winmd").unwrap(), metadata::reader2::File::new("crates/libs/metadata/default/Windows.Win32.winmd").unwrap()];
    let reader = &metadata::reader2::Reader::new(&files);

    let gen = &mut bindgen::bindgen2::Gen::new(reader);
    // TODO: this just ensures that the bindings use the windows.lib rather than the function-specific DLL names
    // but this is a bit of a hacky way to get the intended result.
    gen.namespace = "Windows.";
    gen.min_enum = true;
    gen.min_inherit = true;
    gen.flatten = true;

    let mut tokens = String::new();

    for name in types {
        tokens += &bindgen::bindgen2::define(gen, name);
    }

    let path = std::path::Path::new("crates/libs/windows/src/core/bindings.rs");
    std::fs::write(&path, tokens)?;

    let mut cmd = ::std::process::Command::new("rustfmt");
    cmd.arg(&path);
    cmd.output()?;

    Ok(())
}

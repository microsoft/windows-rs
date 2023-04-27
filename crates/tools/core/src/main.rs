fn main() {
    let bindings = [
        "Windows.Win32.Foundation.CloseHandle",
        "Windows.Win32.Foundation.ERROR_NO_UNICODE_TRANSLATION",
        "Windows.Win32.Foundation.GetLastError",
        "Windows.Win32.Foundation.SysAllocStringLen",
        "Windows.Win32.Foundation.SysFreeString",
        "Windows.Win32.Foundation.SysStringLen",
        "Windows.Win32.System.Com.CoTaskMemAlloc",
        "Windows.Win32.System.Com.CoTaskMemFree",
        "Windows.Win32.System.Diagnostics.Debug.EncodePointer",
        "Windows.Win32.System.Diagnostics.Debug.FORMAT_MESSAGE_ALLOCATE_BUFFER",
        "Windows.Win32.System.Diagnostics.Debug.FORMAT_MESSAGE_FROM_SYSTEM",
        "Windows.Win32.System.Diagnostics.Debug.FORMAT_MESSAGE_IGNORE_INSERTS",
        "Windows.Win32.System.Diagnostics.Debug.FormatMessageW",
        "Windows.Win32.System.LibraryLoader.FreeLibrary",
        "Windows.Win32.System.LibraryLoader.GetProcAddress",
        "Windows.Win32.System.LibraryLoader.LOAD_LIBRARY_SEARCH_DEFAULT_DIRS",
        "Windows.Win32.System.LibraryLoader.LoadLibraryExA",
        "Windows.Win32.System.Memory.GetProcessHeap",
        "Windows.Win32.System.Memory.HeapAlloc",
        "Windows.Win32.System.Memory.HeapFree",
        "Windows.Win32.System.Threading.CreateEventW",
        "Windows.Win32.System.Threading.SetEvent",
        "Windows.Win32.System.Threading.WaitForSingleObject",
    ];

    let bindings = windows_bindgen::standalone_sys(&bindings);
    std::fs::write("crates/libs/core/src/imp/bindings.rs", bindings).unwrap();

    let bindings = [
        "Windows.Foundation.IReference",
        "Windows.Foundation.IStringable",
        "Windows.Foundation.PropertyValue",
        "Windows.Win32.Foundation.CLASS_E_CLASSNOTAVAILABLE",
        "Windows.Win32.Foundation.CO_E_NOTINITIALIZED",
        "Windows.Win32.Foundation.E_BOUNDS",
        "Windows.Win32.Foundation.E_NOINTERFACE",
        "Windows.Win32.Foundation.E_OUTOFMEMORY",
        "Windows.Win32.Foundation.JSCRIPT_E_CANTEXECUTE",
        "Windows.Win32.Foundation.RPC_E_DISCONNECTED",
        "Windows.Win32.System.Com.CoCreateGuid",
        "Windows.Win32.System.Com.GetErrorInfo",
        "Windows.Win32.System.Com.IAgileObject",
        "Windows.Win32.System.Com.IErrorInfo",
        "Windows.Win32.System.Com.SetErrorInfo",
        "Windows.Win32.System.WinRT.AGILEREFERENCE_DEFAULT",
        "Windows.Win32.System.WinRT.IAgileReference",
        "Windows.Win32.System.WinRT.ILanguageExceptionErrorInfo2",
        "Windows.Win32.System.WinRT.IRestrictedErrorInfo",
        "Windows.Win32.System.WinRT.IWeakReferenceSource",
        "Windows.Win32.System.WinRT.RoGetAgileReference",
    ];

    let bindings = windows_bindgen::standalone_win(&bindings);
    std::fs::write("crates/libs/core/src/imp/com_bindings.rs", bindings).unwrap();
}

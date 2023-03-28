fn main() {
    let apis = ["Windows.Win32.Foundation.BOOL", "Windows.Win32.Foundation.CloseHandle", "Windows.Win32.Foundation.HANDLE", "Windows.Win32.Foundation.WIN32_ERROR", "Windows.Win32.Security.SECURITY_ATTRIBUTES", "Windows.Win32.System.Com.CLSCTX", "Windows.Win32.System.Com.CoCreateInstance", "Windows.Win32.System.Threading.CreateEventW", "Windows.Win32.System.Threading.SetEvent", "Windows.Win32.System.Threading.WaitForSingleObject"];

    let bindings = windows_bindgen::standalone(&apis);
    std::fs::write("src/bindings.rs", bindings).unwrap();
}

fn main() {
    let apis = [
        "Windows.Win32.Foundation.CloseHandle",
        "Windows.Win32.System.Com.CoCreateInstance",
        "Windows.Win32.System.Com.STGTY_REPEAT",
        "Windows.Win32.System.Threading.CreateEventW",
        "Windows.Win32.System.Threading.SetEvent",
        "Windows.Win32.System.Threading.WaitForSingleObject",
        "Windows.Win32.UI.Animation.UIAnimationManager",
    ];

    let bindings = windows_bindgen::standalone(&apis);
    std::fs::write("src/bindings.rs", bindings).unwrap();
}

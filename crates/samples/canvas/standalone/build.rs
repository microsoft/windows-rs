fn main() {
    if !cfg!(windows) {
        return;
    }

    println!("cargo:rerun-if-changed=build.rs");

    let warnings = windows_bindgen::bindgen([
        "--out",
        "src/bindings.rs",
        "--flat",
        "--minimal",
        "--filter",
        "Windows.Win32.UI.WindowsAndMessaging.RegisterClassA",
        "Windows.Win32.UI.WindowsAndMessaging.CreateWindowExA",
        "Windows.Win32.UI.WindowsAndMessaging.DefWindowProcA",
        "Windows.Win32.UI.WindowsAndMessaging.TranslateMessage",
        "Windows.Win32.UI.WindowsAndMessaging.DispatchMessageA",
        "Windows.Win32.UI.WindowsAndMessaging.PeekMessageA",
        "Windows.Win32.UI.WindowsAndMessaging.PostQuitMessage",
        "Windows.Win32.UI.WindowsAndMessaging.WM_DESTROY",
        "Windows.Win32.UI.WindowsAndMessaging.CW_USEDEFAULT",
        "Windows.Win32.UI.WindowsAndMessaging.WS_OVERLAPPEDWINDOW",
        "Windows.Win32.UI.WindowsAndMessaging.WS_VISIBLE",
        "Windows.Win32.UI.WindowsAndMessaging.PM_REMOVE",
        "Windows.Win32.UI.WindowsAndMessaging.CS_HREDRAW",
        "Windows.Win32.UI.WindowsAndMessaging.CS_VREDRAW",
    ]);

    if !warnings.is_empty() {
        println!("cargo:warning=bindgen warnings: {warnings}");
    }
}

fn main() {
    write(
        "src/b_none.rs",
        &["Windows.Win32.System.SystemInformation.GetTickCount"],
    );

    write(
        "src/b_hresult.rs",
        &["Windows.Win32.System.Com.CoInitialize"],
    );

    write(
        "src/b_hstring.rs",
        &["Windows.Win32.System.WinRT.WindowsGetStringLen"],
    );

    write(
        "src/b_unknown.rs",
        &["Windows.Win32.System.Com.CoIsHandlerConnected"],
    );

    write(
        "src/b_inspectable.rs",
        &["Windows.Win32.System.WinRT.RoActivateInstance"],
    );

    write("src/b_pstr.rs", &["Windows.Win32.System.Ole.VarI1FromDate"]);

    write("src/b_pwstr.rs", &["Windows.Win32.System.Ole.CALPOLESTR"]);

    write("src/b_pcstr.rs", &["Windows.Win32.Globalization.lstrlenA"]);

    write("src/b_pcwstr.rs", &["Windows.Win32.Globalization.lstrlenW"]);

    write(
        "src/b_bstr.rs",
        &["Windows.Win32.Foundation.SysAllocString"],
    );

    write("src/b_guid.rs", &["Windows.Win32.System.Com.CoCreateGuid"]);

    write(
        "src/b_test.rs",
        &[
            "Windows.Win32.Foundation.CloseHandle",
            "Windows.Win32.System.Com.CoCreateInstance",
            "Windows.Win32.System.Com.STGTY_REPEAT",
            "Windows.Win32.System.Threading.CreateEventW",
            "Windows.Win32.System.Threading.SetEvent",
            "Windows.Win32.System.Threading.WaitForSingleObject",
            "Windows.Win32.UI.Animation.UIAnimationManager",
        ],
    );
}

fn write(filename: &str, apis: &[&str]) {
    let bindings = windows_bindgen::standalone(&apis);
    std::fs::write(filename, bindings).unwrap();
}

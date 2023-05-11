fn main() {
    write_sys(
        "src/b_none.rs",
        &["Windows.Win32.System.SystemInformation.GetTickCount"],
    );

    write_sys(
        "src/b_hresult.rs",
        &["Windows.Win32.System.Com.CoInitialize"],
    );

    write_sys(
        "src/b_hstring.rs",
        &["Windows.Win32.System.WinRT.WindowsGetStringLen"],
    );

    write_sys(
        "src/b_unknown.rs",
        &["Windows.Win32.System.Com.CoIsHandlerConnected"],
    );

    write_sys(
        "src/b_inspectable.rs",
        &["Windows.Win32.System.WinRT.RoActivateInstance"],
    );

    write_sys("src/b_pstr.rs", &["Windows.Win32.System.Ole.VarI1FromDate"]);

    write_sys("src/b_pwstr.rs", &["Windows.Win32.System.Ole.CALPOLESTR"]);

    write_sys("src/b_pcstr.rs", &["Windows.Win32.Globalization.lstrlenA"]);

    write_sys("src/b_pcwstr.rs", &["Windows.Win32.Globalization.lstrlenW"]);

    write_sys(
        "src/b_bstr.rs",
        &["Windows.Win32.Foundation.SysAllocString"],
    );

    write_sys("src/b_guid.rs", &["Windows.Win32.System.Com.CoCreateGuid"]);

    write_sys(
        "src/b_arch.rs",
        &[
            "Windows.Win32.Networking.WinSock.WSADATA",
            "Windows.Win32.UI.WindowsAndMessaging.GetWindowLongPtrW",
        ],
    );

    // Ensures that root items that reference architecture independent types
    // of the same name correctly collect all of those types rather than just
    // the first one
    write_sys(
        "src/b_arch_dependencies.rs",
        &["Windows.Win32.System.Diagnostics.Debug.RtlCaptureContext"],
    );

    write_sys(
        "src/b_depends.rs",
        &["Windows.Win32.Networking.WinSock.WSASENDMSG"],
    );

    write_sys(
        "src/b_enumeration.rs",
        &["Windows.Win32.Foundation.WIN32_ERROR"],
    );

    write_sys(
        "src/b_enumerator.rs",
        &[
            "Windows.Win32.Foundation.WAIT_IO_COMPLETION",
            "Windows.Win32.Foundation.WAIT_TIMEOUT",
        ],
    );

    write_win(
        "src/b_win_enumerator.rs",
        &[
            "Windows.Win32.Foundation.WAIT_IO_COMPLETION",
            "Windows.Win32.Foundation.WAIT_TIMEOUT",
        ],
    );

    write_std(
        "src/b_std.rs",
        &[
            "Windows.Win32.Foundation.CloseHandle",
            "Windows.Win32.Foundation.GetLastError",
            "Windows.Win32.Foundation.HMODULE",
            "Windows.Win32.Networking.WinSock.socket",
            "Windows.Win32.Security.Cryptography.BCRYPT_ALG_HANDLE",
            "Windows.Win32.Storage.FileSystem.FindFileHandle",
            "Windows.Win32.Security.Authentication.Identity.RtlGenRandom",
            "Windows.Win32.UI.WindowsAndMessaging.wsprintfA",
        ],
    );

    write_win("src/b_uri.rs", &["Windows.Foundation.Uri"]);
    write_win("src/b_stringable.rs", &["Windows.Foundation.IStringable"]);
    write_win("src/b_calendar.rs", &["Windows.Globalization.Calendar"]);

    write_sys(
        "src/b_test.rs",
        &[
            "Windows.Win32.Foundation.CloseHandle",
            "Windows.Win32.System.Com.CoCreateInstance",
            "Windows.Win32.System.Com.CLSCTX_ALL",
            "Windows.Win32.System.Com.STGTY_REPEAT",
            "Windows.Win32.System.Threading.CreateEventW",
            "Windows.Win32.System.Threading.SetEvent",
            "Windows.Win32.System.Threading.WaitForSingleObject",
            "Windows.Win32.UI.Animation.UIAnimationManager",
            "Windows.Win32.UI.WindowsAndMessaging.wsprintfA",
        ],
    );

    // Ensures nested records write out all the types they depend on
    write_sys(
        "src/b_nested.rs",
        &[
            "Windows.Win32.System.Com.STGMEDIUM",
            "Windows.Win32.Graphics.Gdi.DEVMODEW",
        ],
    );

    write_sys(
        "src/b_overloads.rs",
        &["Windows.Win32.NetworkManagement.NetManagement.AE_RESACCESS"],
    );
}

fn write_sys(filename: &str, apis: &[&str]) {
    let bindings = windows_bindgen::standalone_sys(apis);
    std::fs::write(filename, bindings).unwrap();
}

fn write_win(filename: &str, apis: &[&str]) {
    let bindings = windows_bindgen::standalone_win(apis);
    std::fs::write(filename, bindings).unwrap();
}

fn write_std(filename: &str, apis: &[&str]) {
    let bindings = windows_bindgen::standalone_std(apis);
    std::fs::write(filename, bindings).unwrap();
}

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

    // Ensures that constant types are properly collected
    write_sys(
        "src/b_constant_types.rs",
        &[
            "Windows.Win32.UI.WindowsAndMessaging.IDC_UPARROW",
            "Windows.Win32.Security.Cryptography.CMC_ADD_ATTRIBUTES",
        ],
    );

    // Ensure that no-inner-attribute works, and the resulting
    // file can be `include!` inside a mod{} block.
    write_no_inner_attr(
        "src/b_include_me.rs",
        &["Windows.Win32.System.SystemInformation.GetVersion"],
    );

    // Ensure that contained types behind pointers are resolved as dependencies.
    write_sys(
        "src/b_variant.rs",
        &["Windows.Win32.System.Variant.VARIANT"],
    );

    write_vtbl(
        "src/b_vtbl_0.rs",
        &["Windows.Win32.System.Com.IAgileObject"],
    );
    write_vtbl("src/b_vtbl_1.rs", &["Windows.Win32.System.Com.IDispatch"]);
    write_vtbl(
        "src/b_vtbl_2.rs",
        &["Windows.Win32.System.WinRT.IActivationFactory"],
    );
    write_vtbl("src/b_vtbl_3.rs", &["Windows.Foundation.IStringable"]);
    write_vtbl(
        "src/b_vtbl_4.rs",
        &["Windows.Win32.System.Com.IPersistFile"],
    );
}

fn write_sys(output: &str, filter: &[&str]) {
    riddle(output, filter, &["flatten", "sys", "minimal"]);
}

fn write_win(output: &str, filter: &[&str]) {
    riddle(output, filter, &["flatten", "minimal"]);
}

fn write_std(output: &str, filter: &[&str]) {
    riddle(output, filter, &["flatten", "std", "minimal"]);
}

fn write_no_inner_attr(output: &str, filter: &[&str]) {
    riddle(
        output,
        filter,
        &["flatten", "no-inner-attributes", "minimal"],
    );
}

fn write_vtbl(output: &str, filter: &[&str]) {
    riddle(output, filter, &["flatten", "sys", "minimal", "vtbl"]);
}

fn riddle(output: &str, filter: &[&str], config: &[&str]) {
    // Rust-analyzer may re-run build scripts whenever a source file is deleted
    // which causes an endless loop if the file is deleted from a build script.
    // To workaround this, we truncate the file instead of deleting it.
    // See https://github.com/microsoft/windows-rs/issues/2777
    _ = std::fs::File::options()
        .truncate(true)
        .write(true)
        .open(output);

    let mut command = std::process::Command::new("cargo");

    command.args([
        "run",
        "-p",
        "riddle",
        "--target-dir",
        "../../../target/test_standalone", // TODO: workaround for https://github.com/rust-lang/cargo/issues/6412
        "--",
        "--in",
        "../../libs/bindgen/default",
        "--out",
        output,
        "--filter",
    ]);

    command.args(filter);
    command.arg("--config");
    command.args(config);

    if !command.status().unwrap().success() {
        panic!("Failed to run riddle");
    }
}

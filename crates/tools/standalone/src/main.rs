//! Regenerates bindings sources for `crates/tests/standalone`

use std::path::Path;

// TODO: move these into the new bindgen test/tool

fn main() {
    if !Path::new("crates/tests/misc/standalone/Cargo.toml").exists() {
        println!("This tool must be run from the root of the repo.");
        std::process::exit(1);
    }

    let src = Path::new("crates/tests/misc/standalone/src");

    write_sys(
        &src.join("b_none.rs"),
        &["Windows.Win32.System.SystemInformation.GetTickCount"],
    );

    write_sys(
        &src.join("b_hresult.rs"),
        &["Windows.Win32.System.Com.CoInitialize"],
    );

    write_sys(
        &src.join("b_hstring.rs"),
        &["Windows.Win32.System.WinRT.WindowsGetStringLen"],
    );

    write_sys(
        &src.join("b_unknown.rs"),
        &["Windows.Win32.System.Com.CoIsHandlerConnected"],
    );

    write_sys(
        &src.join("b_inspectable.rs"),
        &["Windows.Win32.System.WinRT.RoActivateInstance"],
    );

    write_sys(
        &src.join("b_pstr.rs"),
        &["Windows.Win32.System.Ole.VarI1FromDate"],
    );

    write_sys(
        &src.join("b_pwstr.rs"),
        &["Windows.Win32.System.Ole.CALPOLESTR"],
    );

    write_sys(
        &src.join("b_pcstr.rs"),
        &["Windows.Win32.Globalization.lstrlenA"],
    );

    write_sys(
        &src.join("b_pcwstr.rs"),
        &["Windows.Win32.Globalization.lstrlenW"],
    );

    write_sys(
        &src.join("b_bstr.rs"),
        &["Windows.Win32.Foundation.SysAllocString"],
    );

    write_sys(
        &src.join("b_guid.rs"),
        &["Windows.Win32.System.Com.CoCreateGuid"],
    );

    write_sys(
        &src.join("b_arch.rs"),
        &[
            "Windows.Win32.Networking.WinSock.WSADATA",
            "Windows.Win32.UI.WindowsAndMessaging.GetWindowLongPtrW",
        ],
    );

    // Ensures that root items that reference architecture independent types
    // of the same name correctly collect all of those types rather than just
    // the first one
    write_sys(
        &src.join("b_arch_dependencies.rs"),
        &["Windows.Win32.System.Diagnostics.Debug.RtlCaptureContext"],
    );

    write_sys(
        &src.join("b_depends.rs"),
        &["Windows.Win32.Networking.WinSock.WSASENDMSG"],
    );

    write_sys(
        &src.join("b_enumeration.rs"),
        &["Windows.Win32.Foundation.WIN32_ERROR"],
    );

    write_sys(
        &src.join("b_enumerator.rs"),
        &[
            "Windows.Win32.Foundation.WAIT_IO_COMPLETION",
            "Windows.Win32.Foundation.WAIT_TIMEOUT",
        ],
    );

    write_win(
        &src.join("b_win_enumerator.rs"),
        &[
            "Windows.Win32.Foundation.WAIT_IO_COMPLETION",
            "Windows.Win32.Foundation.WAIT_TIMEOUT",
        ],
    );

    write_sys(
        &src.join("b_std.rs"),
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

    write_win(&src.join("b_uri.rs"), &["Windows.Foundation.Uri"]);
    write_win(
        &src.join("b_stringable.rs"),
        &["Windows.Foundation.IStringable"],
    );
    write_win(
        &src.join("b_calendar.rs"),
        &["Windows.Globalization.Calendar"],
    );

    write_sys(
        &src.join("b_test.rs"),
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
        &src.join("b_nested.rs"),
        &[
            "Windows.Win32.System.Com.STGMEDIUM",
            "Windows.Win32.Graphics.Gdi.DEVMODEW",
        ],
    );

    write_sys(
        &src.join("b_overloads.rs"),
        &["Windows.Win32.NetworkManagement.NetManagement.AE_RESACCESS"],
    );

    // Ensures that constant types are properly collected
    write_sys(
        &src.join("b_constant_types.rs"),
        &[
            "Windows.Win32.UI.WindowsAndMessaging.IDC_UPARROW",
            "Windows.Win32.Security.Cryptography.CMC_ADD_ATTRIBUTES",
            "Windows.Win32.Foundation.E_ACCESSDENIED",
            "Windows.Win32.Foundation.STATUS_NOT_FOUND",
        ],
    );

    // Ensure that no-inner-attribute works, and the resulting
    // file can be `include!` inside a mod{} block.
    write_no_inner_attr(
        &src.join("b_include_me.rs"),
        &["Windows.Win32.System.SystemInformation.GetVersion"],
    );

    // Ensure that contained types behind pointers are resolved as dependencies.
    write_sys(
        &src.join("b_variant.rs"),
        &["Windows.Win32.System.Variant.VARIANT"],
    );

    write_vtbl(
        &src.join("b_vtbl_0.rs"),
        &["Windows.Win32.System.Com.IAgileObject"],
    );
    write_vtbl(
        &src.join("b_vtbl_1.rs"),
        &["Windows.Win32.System.Com.IDispatch"],
    );
    write_vtbl(
        &src.join("b_vtbl_2.rs"),
        &["Windows.Win32.System.WinRT.IActivationFactory"],
    );
    write_vtbl(
        &src.join("b_vtbl_3.rs"),
        &["Windows.Foundation.IStringable"],
    );
    write_vtbl(
        &src.join("b_vtbl_4.rs"),
        &["Windows.Win32.System.Com.IPersistFile"],
    );

    bindgen(
        &src.join("b_prepend.rs"),
        &["Windows.Foundation.DateTime"],
        &[
            "--flat",
            "--derive",
            "Windows.Foundation.DateTime=PartialOrd,Ord,Eq",
        ],
    );
}

fn write_sys(output: &Path, filter: &[&str]) {
    bindgen(output, filter, &["--flat", "--sys"]);
}

fn write_win(output: &Path, filter: &[&str]) {
    let output: &str = output.as_os_str().to_str().unwrap();
    let mut args = vec!["--no-deps", "--out", output, "--filter"];
    args.extend_from_slice(filter);
    args.extend_from_slice(&["--no-comment"]);
    args.extend_from_slice(&["--flat"]);
    println!("running: bindgen {}", args.join(" "));
    _ = windows_bindgen::bindgen(args);
}

fn write_no_inner_attr(output: &Path, filter: &[&str]) {
    bindgen(output, filter, &["--flat", "--no-allow"]);
}

fn write_vtbl(output: &Path, filter: &[&str]) {
    bindgen(output, filter, &["--flat", "--sys"]);
}

fn bindgen(output: &Path, filter: &[&str], config: &[&str]) {
    let output: &str = output.as_os_str().to_str().unwrap();
    let mut args = vec!["--no-deps", "--out", output, "--filter"];
    args.extend_from_slice(filter);
    args.extend_from_slice(&["--no-comment"]);
    args.extend_from_slice(config);
    println!("running: bindgen {}", args.join(" "));
    _ = windows_bindgen::bindgen(args);
}

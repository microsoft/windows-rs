include!(concat!(env!("OUT_DIR"), "/generated_tests.rs"));

fn run(name: &str) {
    let input_path = format!("input/{name}.rdl");
    let expected_path = format!("expected/{name}.rs");
    let scratch = format!("{}/{name}", env!("OUT_DIR"));
    std::fs::create_dir_all(&scratch).unwrap();

    // Extract args from `//!` comment lines at the top.
    let contents = std::fs::read_to_string(&input_path).unwrap();
    let mut args = Vec::new();

    for line in contents.lines() {
        if let Some(rest) = line.strip_prefix("//!") {
            for arg in rest.split_whitespace() {
                args.push(arg.to_string());
            }
        }
    }

    let winmd = format!("{scratch}/out.winmd");

    windows_rdl::reader()
        .input(&input_path)
        .output(&winmd)
        .write()
        .unwrap();

    let out_rs = format!("{scratch}/out.rs");
    let mut cli: Vec<String> = vec!["--in".into(), winmd];
    cli.extend(args);
    cli.push("--out".into());
    cli.push(out_rs.clone());

    windows_bindgen::bindgen(cli);

    let actual = std::fs::read_to_string(&out_rs).unwrap();
    std::fs::write(&expected_path, &actual).unwrap();
}

#[test]
fn real_variadic_exports_are_raw_sys_only() {
    let rich =
        std::fs::read_to_string("../../../libs/windows/src/Windows/Win32/authz/mod.rs").unwrap();
    assert!(!rich.contains("pub unsafe fn AuthzReportSecurityEvent("));

    let sys = std::fs::read_to_string("../../../libs/sys/src/Windows/Win32/authz/mod.rs").unwrap();
    assert!(sys.contains(
        "windows_link::link!(\"authz.dll\" \"C\" fn AuthzReportSecurityEvent(dwflags : u32, \
         heventprovider : AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE, dwauditid : u32, pusersid : \
         super::PSID, dwcount : u32, ...) -> windows_sys::core::BOOL);"
    ));
}

#[test]
fn real_inout_parameters_remain_output_capable() {
    let wingdi =
        std::fs::read_to_string("../../../libs/windows/src/Windows/Win32/wingdi/mod.rs").unwrap();
    assert!(wingdi.contains(
        "pub unsafe fn DPtoLP(hdc: super::HDC, lppt: &mut [super::POINT]) -> \
         windows_core::BOOL"
    ));

    let dpapi =
        std::fs::read_to_string("../../../libs/windows/src/Windows/Win32/dpapi/mod.rs").unwrap();
    assert!(dpapi.contains(
        "pub unsafe fn CryptProtectMemory(pdatain: *mut core::ffi::c_void, cbdatain: u32, \
         dwflags: u32) -> windows_core::BOOL"
    ));
}

use std::path::Path;
use std::process::Command;

#[test]
fn named_import_consumer() {
    run(None, "sdk_named_imports");
}

#[test]
fn winsock_import_consumer() {
    run(None, "sdk_winsock");
}

#[test]
fn context_import_consumer() {
    run(None, "sdk_context");
}

#[test]
fn default_import_consumer() {
    run(None, "sdk_import_gaps");
}

#[test]
fn guidless_import_consumer() {
    run(None, "sdk_guidless");
}

#[test]
fn allocator_import_consumer() {
    run(None, "sdk_allocator");
}

#[test]
fn overrides_import_consumer() {
    run(None, "sdk_overrides");
}

#[test]
fn psapi_import_consumer() {
    run(None, "sdk_psapi");
}

#[test]
fn target_import_consumer() {
    run(None, "sdk_unresolved_imports");
}

#[test]
fn variadic_import_consumer() {
    run(None, "sdk_variadics");
}

#[test]
#[ignore = "requires three-target Rust/MSVC link toolchains and the pinned LLVM inspection tool"]
fn variadic_import_consumers_cross_targets() {
    for target in [
        "i686-pc-windows-msvc",
        "x86_64-pc-windows-msvc",
        "aarch64-pc-windows-msvc",
    ] {
        run(Some(target), "sdk_variadics");
    }
}

#[test]
#[ignore = "requires three-target Rust/MSVC link toolchains and the pinned LLVM inspection tool"]
fn named_import_consumers_cross_targets() {
    for target in [
        "i686-pc-windows-msvc",
        "x86_64-pc-windows-msvc",
        "aarch64-pc-windows-msvc",
    ] {
        run(Some(target), "sdk_named_imports");
        run(Some(target), "sdk_winsock");
        run(Some(target), "sdk_context");
        run(Some(target), "sdk_import_gaps");
        run(Some(target), "sdk_guidless");
        run(Some(target), "sdk_allocator");
        run(Some(target), "sdk_overrides");
        run(Some(target), "sdk_psapi");
        run(Some(target), "sdk_unresolved_imports");
    }
}

fn checked(command: &mut Command) -> String {
    let output = command.output().unwrap();
    assert!(
        output.status.success(),
        "{command:?}\n{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

fn run(target: Option<&str>, fixture: &str) {
    let consumer = match fixture {
        "sdk_winsock" => "winsock_import_consumer",
        "sdk_named_imports" => "named_import_consumer",
        "sdk_context" => "context_import_consumer",
        "sdk_import_gaps" => "default_import_consumer",
        "sdk_guidless" => "guidless_import_consumer",
        "sdk_allocator" => "allocator_import_consumer",
        "sdk_overrides" => "overrides_import_consumer",
        "sdk_psapi" => "psapi_import_consumer",
        "sdk_unresolved_imports" => "target_import_consumer",
        "sdk_variadics" => "variadic_import_consumer",
        _ => panic!("unknown import fixture {fixture}"),
    };
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let scratch = root
        .join("target\\test-clang-named-import-consumers")
        .join(fixture)
        .join(target.unwrap_or("host"));
    std::fs::create_dir_all(&scratch).unwrap();
    let winmd = scratch.join("input.winmd");
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input(test_clang::input_dir().join(format!("{fixture}.rdl")))
        .output(&winmd)
        .write()
        .unwrap();
    windows_bindgen::builder()
        .input(&winmd)
        .filter("Fixture")
        .sys()
        .flat()
        .output(scratch.join("bindings.rs"))
        .write();
    std::fs::copy(
        test_clang::input_dir().join(format!("{consumer}.rs")),
        scratch.join("main.rs"),
    )
    .unwrap();

    let link = scratch.join("libwindows_link.rlib");
    let mut compiler = Command::new("rustc");
    compiler.args([
        "--edition=2024",
        "--crate-name=windows_link",
        "--crate-type=rlib",
    ]);
    if let Some(target) = target {
        compiler.args(["--target", target]);
    }
    checked(
        compiler
            .arg(root.join("crates\\libs\\link\\src\\lib.rs"))
            .arg("-o")
            .arg(&link),
    );
    let executable = scratch.join("consumer.exe");
    let mut compiler = Command::new("rustc");
    compiler.args(["--edition=2024", "--crate-name=named_import_consumer"]);
    if let Some(target) = target {
        // Isolate our raw imports from std's own ws2_32 import library.
        compiler.args([
            "--target",
            target,
            "--cfg=named_import_probe",
            "-Cpanic=abort",
            "-Copt-level=1",
            "-l",
            "ucrt",
            "-l",
            "vcruntime",
            "-Clink-arg=/ENTRY:mainCRTStartup",
            "-Clink-arg=/SUBSYSTEM:CONSOLE",
        ]);
    }
    checked(
        compiler
            .arg(scratch.join("main.rs"))
            .arg("--extern")
            .arg(format!("windows_link={}", link.display()))
            .arg("-o")
            .arg(&executable),
    );
    if target.is_none()
        || target == Some("i686-pc-windows-msvc")
            && cfg!(any(target_arch = "x86", target_arch = "x86_64"))
        || target == Some("x86_64-pc-windows-msvc") && cfg!(target_arch = "x86_64")
        || target == Some("aarch64-pc-windows-msvc") && cfg!(target_arch = "aarch64")
    {
        checked(&mut Command::new(&executable));
    }
    if target.is_some() {
        let host = if cfg!(target_arch = "aarch64") {
            "aarch64-pc-windows-msvc"
        } else {
            "x86_64-pc-windows-msvc"
        };
        let inspect = root
            .join("target\\tool-clang\\native")
            .join(helpers::LIBCLANG_VERSION)
            .join(host)
            .join("llvm\\bin\\llvm-readobj.exe");
        let imports = checked(Command::new(inspect).arg("--coff-imports").arg(&executable));
        let required: &[(&str, &str)] = match fixture {
            "sdk_winsock" => &[
                ("ws2_32.dll", "htons"),
                ("ws2_32.dll", "htonl"),
                ("ws2_32.dll", "ntohs"),
                ("ws2_32.dll", "ntohl"),
                ("ws2_32.dll", "select"),
                ("ws2_32.dll", "WSAGetLastError"),
                ("ws2_32.dll", "WSASetLastError"),
                ("ws2_32.dll", "WSAStartup"),
                ("ws2_32.dll", "WSACleanup"),
            ],
            "sdk_named_imports" => &[
                ("shell32.dll", "ILClone"),
                ("shell32.dll", "ILFree"),
                ("ws2_32.dll", "WSAStartup"),
            ],
            "sdk_context" => &[("kernel32.dll", "RtlCaptureContext")],
            "sdk_guidless" => &[("d3dcompiler_47.dll", "D3DCompile")],
            "sdk_allocator" => &[
                ("kernel32.dll", "HeapAlloc"),
                ("kernel32.dll", "HeapReAlloc"),
                ("kernel32.dll", "GlobalAlloc"),
                ("kernel32.dll", "GlobalReAlloc"),
                ("kernel32.dll", "LocalAlloc"),
                ("kernel32.dll", "LocalReAlloc"),
            ],
            "sdk_overrides" => &[
                ("avifil32.dll", "AVIFileInit"),
                ("avifil32.dll", "AVIFileExit"),
                ("avifil32.dll", "CreateEditableStream"),
                ("avifil32.dll", "EditStreamClone"),
                ("avifil32.dll", "AVIStreamRelease"),
            ],
            "sdk_psapi" => &[("kernel32.dll", "K32EnumProcesses")],
            "sdk_variadics" => &[
                ("user32.dll", "wsprintfA"),
                ("user32.dll", "wsprintfW"),
                ("shlwapi.dll", "wnsprintfA"),
                ("shlwapi.dll", "wnsprintfW"),
                ("icuin.dll", "u_formatMessage"),
            ],
            "sdk_unresolved_imports" if target == Some("i686-pc-windows-msvc") => &[],
            "sdk_unresolved_imports" => &[
                ("oleaut32.dll", "BSTR_UserSize64"),
                ("dnsapi.dll", "DnsValidateName_W"),
            ],
            "sdk_import_gaps" => &[
                ("dnsapi.dll", "DnsValidateName_W"),
                ("dhcpsapi.dll", "DhcpAddServer"),
            ],
            _ => unreachable!(),
        };
        if fixture == "sdk_unresolved_imports" && target == Some("i686-pc-windows-msvc") {
            assert!(!imports.contains("BSTR_UserSize64"));
            assert!(!imports.contains("DnsValidateName_W"));
        }
        for &(library, name) in required {
            assert!(
                imports.split("Import {").any(|block| {
                    block.lines().any(|line| {
                        line.trim()
                            .strip_prefix("Name: ")
                            .is_some_and(|name| name.eq_ignore_ascii_case(library))
                    }) && block.lines().any(|line| {
                        line.trim()
                            .strip_prefix("Symbol:")
                            .is_some_and(|symbol| symbol.split_whitespace().next() == Some(name))
                    })
                }),
                "missing named import {library}!{name}:\n{imports}"
            );
        }
    }
    std::fs::remove_dir_all(scratch).unwrap();
}

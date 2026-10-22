use windows_clang::*;
use windows_metadata::reader::HasAttributes;

const NUMBERS: [(&str, i32); 13] = [
    ("CSBACKUP_TYPE_FULL", 1),
    ("CSBACKUP_TYPE_LOGS_ONLY", 2),
    ("CSBACKUP_TYPE_MASK", 3),
    ("CSBFT_DATABASE_DIRECTORY", 64),
    ("CSBFT_DIRECTORY", 128),
    ("CSBFT_LOG_DIRECTORY", 32),
    ("CSCONTROL_RESTART", 3),
    ("CSCONTROL_SHUTDOWN", 1),
    ("CSCONTROL_SUSPEND", 2),
    ("CSRESTORE_TYPE_CATCHUP", 4),
    ("CSRESTORE_TYPE_FULL", 1),
    ("CSRESTORE_TYPE_MASK", 5),
    ("CSRESTORE_TYPE_ONLINE", 2),
];

const TYPED: [(&str, u16); 6] = [
    ("CSBFT_CERTSERVER_DATABASE", 68),
    ("CSBFT_CHECKPOINT_DIR", 131),
    ("CSBFT_LOG", 33),
    ("CSBFT_LOG_DIR", 130),
    ("CSBFT_PATCH_FILE", 37),
    ("CSBFT_UNKNOWN", 15),
];

#[test]
#[ignore = "requires the pinned SDK and the x86, x64, and ARM64 MSVC Rust targets"]
fn constants_end_to_end() {
    constants(true);
}

#[test]
#[ignore = "requires the pinned SDK"]
fn constants_rdl() {
    constants(false);
}

fn constants(consumers: bool) {
    helpers::ensure_libclang();
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let directory = root.join("target\\test-clang-certbcli");
    std::fs::create_dir_all(&directory).unwrap();
    let winmd = directory.join("fixture.winmd");
    let roundtrip = directory.join("fixture.rdl");
    let again = directory.join("again.winmd");
    let bindings = directory.join("lib.rs");
    let manifest = directory.join("Cargo.toml");
    for unicode in [false, true] {
        let mut input = test_clang::annotated_sdk_fixture();
        input.name = "certbcli_discovery.h".into();
        input.source = test_clang::fixture("certbcli_discovery").source;
        input.roots = vec!["MacroDiscoveryAnchor".into()];
        input.macro_headers = vec!["certbcli.h".into()];
        if unicode {
            input.arguments.push("-DUNICODE".into());
        }
        let captured =
            capture_partial(&[input], &[Target::X86, Target::X64, Target::Arm64]).unwrap();
        for snapshot in &captured.snapshots {
            for value in &snapshot.values {
                for observation in &value.observations {
                    assert!(snapshot.files[observation.location.file.0].ends_with("certbcli.h"));
                }
            }
        }
        let result = projection::project_partial(captured, "Fixture").unwrap();
        assert_eq!(result.output.constants, 24);
        let expected =
            std::fs::read_to_string(test_clang::input_dir().join("certbcli_discovery.rdl"))
                .unwrap()
                .replace("\r\n", "\n");
        let expected = if unicode {
            expected.replace(
                "#[encoding(\"ansi\")]\n    const wsz",
                "#[encoding(\"utf-16\")]\n    const wsz",
            )
        } else {
            expected
        };
        assert_eq!(result.output.partitions["certbcli"].rdl, expected);
        assert!(
            result
                .omissions
                .iter()
                .any(|message| message.contains("CERTBCLI_API")
                    && message.contains("expression probe failed"))
        );
        if !consumers {
            continue;
        }
        windows_rdl::reader()
            .input(root.join("metadata\\metadata.rdl"))
            .input_texts(result.output.partitions.values().map(|part| &part.rdl))
            .output(&winmd)
            .write()
            .unwrap();
        windows_rdl::writer()
            .input(&winmd)
            .filter("Fixture")
            .output(&roundtrip)
            .write()
            .unwrap();
        windows_rdl::reader()
            .input(root.join("metadata\\metadata.rdl"))
            .input(&roundtrip)
            .output(&again)
            .write()
            .unwrap();
        for path in [&winmd, &again] {
            for bit in [1, 2, 4] {
                let index = windows_metadata::reader::Index::new_for_architecture(
                    vec![windows_metadata::reader::File::read(path).unwrap()],
                    bit,
                );
                let apis = index.expect("Fixture", "Apis");
                let field = |name| apis.fields().find(|field| field.name() == name).unwrap();
                assert_eq!(apis.fields().count(), 24);
                for (name, value) in NUMBERS {
                    assert_eq!(field(name).ty(), windows_metadata::Type::I32);
                    assert_eq!(
                        field(name).constant().unwrap().value(),
                        windows_metadata::Value::I32(value)
                    );
                }
                for (name, value) in TYPED {
                    assert!(
                        matches!(field(name).ty(), windows_metadata::Type::ValueName(name) if name.name == "CSBFT" && name.namespace == "Fixture")
                    );
                    assert_eq!(
                        field(name).constant().unwrap().value(),
                        windows_metadata::Value::U16(value)
                    );
                }
                assert_eq!(
                    field("CSBACKUP_DISABLE_INCREMENTAL").ty(),
                    windows_metadata::Type::U32
                );
                assert_eq!(
                    field("CSBACKUP_DISABLE_INCREMENTAL")
                        .constant()
                        .unwrap()
                        .value(),
                    windows_metadata::Value::U32(u32::MAX)
                );
                for (name, text) in [
                    ("szBACKUPANNOTATION", "Cert Server Backup Interface"),
                    ("szRESTOREANNOTATION", "Cert Server Restore Interface"),
                    ("wszBACKUPANNOTATION", "Cert Server Backup Interface"),
                    ("wszRESTOREANNOTATION", "Cert Server Restore Interface"),
                ] {
                    assert_eq!(
                        field(name).constant().unwrap().value(),
                        windows_metadata::Value::Utf16(text.into())
                    );
                    let encoding = if unicode && name.starts_with("wsz") {
                        "utf-16"
                    } else {
                        "ansi"
                    };
                    assert_eq!(
                        field(name)
                            .find_attribute("NativeEncodingAttribute")
                            .unwrap()
                            .value()[0]
                            .1,
                        windows_metadata::Value::Utf8(encoding.into())
                    );
                }
            }
        }
        windows_bindgen::builder()
            .input(&winmd)
            .filter("Fixture")
            .flat()
            .output(&bindings)
            .write();
        let mut rust = std::fs::read_to_string(&bindings).unwrap();
        rust.push_str("\nconst _: CSBFT = CSBFT_LOG;\nconst _: i32 = CSBACKUP_TYPE_FULL;\nconst _: u32 = CSBACKUP_DISABLE_INCREMENTAL;\nconst _: windows_core::PCSTR = szBACKUPANNOTATION;\n");
        let wide = if unicode { "PCWSTR" } else { "PCSTR" };
        rust.push_str(&format!(
            "const _: windows_core::{wide} = wszBACKUPANNOTATION;\n"
        ));
        rust.push_str("const _: () = {\nassert!(CSBACKUP_DISABLE_INCREMENTAL == u32::MAX);\n");
        for (name, value) in NUMBERS {
            rust.push_str(&format!("assert!({name} == {value});\n"));
        }
        for (name, value) in TYPED {
            rust.push_str(&format!("assert!({name} == {value});\n"));
        }
        rust.push_str("};\n");
        std::fs::write(&bindings, rust).unwrap();
        std::fs::write(&manifest, format!(
            "[package]\nname = \"test-clang-certbcli\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\
             [workspace]\n[lib]\npath = \"lib.rs\"\n[dependencies]\nwindows-core = {{ path = {:?} }}\n",
            root.join("crates\\libs\\core")
        )).unwrap();
        for target in [Target::X86, Target::X64, Target::Arm64] {
            let output = std::process::Command::new("cargo")
                .args([
                    "check",
                    "--offline",
                    "--quiet",
                    "--target",
                    target.triple(),
                    "--manifest-path",
                ])
                .arg(&manifest)
                .arg("--target-dir")
                .arg(root.join("target\\test-clang-binding-build"))
                .output()
                .unwrap();
            assert!(
                output.status.success(),
                "{target:?}: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }
    if !consumers {
        return;
    }
    for path in [
        winmd,
        roundtrip,
        again,
        bindings,
        manifest,
        directory.join("Cargo.lock"),
    ] {
        std::fs::remove_file(path).unwrap();
    }
}

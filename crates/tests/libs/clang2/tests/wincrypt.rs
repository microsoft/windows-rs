use windows_clang2::*;

#[test]
#[ignore = "requires the pinned SDK"]
fn discovered_constant_rdl() {
    helpers::ensure_libclang();
    for unicode in [false, true] {
        let mut input = test_clang2::annotated_sdk_fixture();
        input.name = "wincrypt_discovery.h".into();
        input.source = test_clang2::fixture("wincrypt_discovery").source;
        input.roots = vec!["MacroDiscoveryAnchor".into()];
        input.macro_headers = vec!["wincrypt.h".into()];
        if unicode {
            input.arguments.push("-DUNICODE".into());
        }
        let output = projection::generate_partial(&[input], "Fixture").unwrap();
        let actual = &output.output.partitions["wincrypt"].rdl;
        for name in [
            "CERT_STRONG_SIGN_PARA_OS_1",
            "CERT_STRONG_SIGN_PARA_OS_CURRENT",
            "CERT_STRONG_KEY_PARA_OS_1",
            "CERT_STRONG_KEY_PARA_OS_CURRENT",
        ] {
            assert!(output.omissions.iter().any(|message| {
                message.contains(&format!("macro `{name}`: expression probe failed"))
            }));
            assert!(!actual.contains(&format!("const {name}:")));
        }
        let expected =
            std::fs::read_to_string(test_clang2::input_dir().join("wincrypt_discovery.rdl"))
                .unwrap()
                .replace("\r\n", "\n");
        let expected = if unicode {
            expected.replace(
                "#[encoding(\"ansi\")]\n    const MS_ENHANCED_PROV:",
                "#[encoding(\"utf-16\")]\n    const MS_ENHANCED_PROV:",
            )
        } else {
            expected
        };
        let mut declaration = String::new();
        for line in expected.lines() {
            if line.starts_with("    #[") || line.starts_with("    const ") {
                declaration.push_str(line);
                declaration.push('\n');
            }
            if line.starts_with("    const ") {
                assert!(
                    actual.contains(&declaration),
                    "{declaration}: {:?}",
                    output.omissions.iter().take(5).collect::<Vec<_>>()
                );
                declaration.clear();
            }
        }
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
        let path = root.join("target\\test-clang2-wincrypt.winmd");
        windows_rdl::reader()
            .input(root.join("metadata\\metadata.rdl"))
            .input_texts(output.output.partitions.values().map(|part| &part.rdl))
            .output(&path)
            .write()
            .unwrap();
        let index = windows_metadata::reader::Index::new(vec![
            windows_metadata::reader::File::read(&path).unwrap(),
        ]);
        let field = index
            .expect("Fixture", "Apis")
            .fields()
            .find(|field| field.name() == "CERT_PROT_ROOT_PEER_USAGES_DEFAULT_A")
            .unwrap();
        assert_eq!(
            field.constant().unwrap().value(),
            windows_metadata::Value::Utf16(
                "1.3.6.1.5.5.7.3.2\x001.3.6.1.5.5.7.3.4\x001.3.6.1.4.1.311.10.3.4\0".into()
            )
        );
        std::fs::remove_file(path).unwrap();
    }
}

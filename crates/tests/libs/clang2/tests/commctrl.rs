use windows_clang2::*;

#[test]
#[ignore = "requires the pinned SDK"]
fn discovered_constant_rdl() {
    helpers::ensure_libclang();
    for unicode in [false, true] {
        let mut input = test_clang2::annotated_sdk_fixture();
        input.name = "commctrl_discovery.h".into();
        input.source = test_clang2::fixture("commctrl_discovery").source;
        input.roots = vec!["MacroDiscoveryAnchor".into()];
        input.macro_headers = vec!["commctrl.h".into(), "richedit.h".into()];
        if unicode {
            input.arguments.push("-DUNICODE".into());
        }
        let output = projection::generate_partial(&[input], "Fixture").unwrap();
        for header in ["commctrl", "richedit"] {
            let actual = &output.output.partitions[header].rdl;
            let expected = std::fs::read_to_string(
                test_clang2::input_dir().join(format!("{header}_discovery.rdl")),
            )
            .unwrap()
            .replace("\r\n", "\n");
            let expected = if unicode {
                expected
                .replace("LPSTR_TEXTCALLBACK: LPSTR", "LPSTR_TEXTCALLBACK: LPWSTR")
                .replace(
                    "#[encoding(\"ansi\")]\n    const WC_LISTVIEW:",
                    "#[encoding(\"utf-16\")]\n    const WC_LISTVIEW:",
                )
                .replace(
                    "#[encoding(\"ansi\")]\n    const RICHEDIT_CLASS: String = \"RichEdit20A\";",
                    "#[encoding(\"utf-16\")]\n    const RICHEDIT_CLASS: String = \"RichEdit20W\";",
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
        }
        for name in ["EM_GETZOOM", "EM_SETZOOM"] {
            assert!(
                !output.output.partitions["commctrl"]
                    .rdl
                    .contains(&format!("const {name}:"))
            );
        }
    }
}

use windows_metadata::*;

fn test_dir(name: &str) -> std::path::PathBuf {
    std::env::temp_dir().join(format!("windows_metadata_{name}"))
}

fn input_winmd(dir: &std::path::Path) -> std::path::PathBuf {
    let input = dir.join("input.winmd");
    windows_rdl::reader()
        .input_text(
            "#[win32] mod Flat { \
                struct Routed { value: u32 } \
                struct RoutedMany { value: u32 } \
                struct Other { value: u32 } \
            }",
        )
        .output(&input)
        .write()
        .unwrap();
    input
}

#[test]
fn remapper_routes_types_and_uses_fallback() {
    let dir = test_dir("remap_routes");
    std::fs::create_dir_all(&dir).unwrap();
    let input = input_winmd(&dir);
    let output = dir.join("output.winmd");

    remap()
        .inputs([input])
        .sources(["Flat"])
        .route("Routed", "Flat.Routed")
        .routes([("RoutedMany", "Flat.RoutedMany")])
        .fallback("Flat.Fallback")
        .output(&output)
        .remap()
        .unwrap();

    let index = reader::Index::read(output.to_string_lossy().as_ref()).unwrap();
    assert!(index.get("Flat.Routed", "Routed").next().is_some());
    assert!(index.get("Flat.RoutedMany", "RoutedMany").next().is_some());
    assert!(index.get("Flat.Fallback", "Other").next().is_some());
}

#[test]
fn remapper_routes_attribute_type_values() {
    let dir = test_dir("remap_type_values");
    std::fs::create_dir_all(&dir).unwrap();
    let input = dir.join("input.winmd");
    let output = dir.join("output.winmd");
    windows_rdl::reader()
        .input_text(
            "#[win32] mod Flat {
                attribute MemberAttribute {
                    fn(ty: Type);
                    MemberType: Type,
                }
                #[repr(i32)]
                enum Mode { A = 0 }
                struct Bits {
                    #[Member(Mode, MemberType = Mode)]
                    named: u32,
                    #[Member(i32, MemberType = i32)]
                    primitive: u32,
                }
            }",
        )
        .output(&input)
        .write()
        .unwrap();
    remap()
        .input(&input)
        .sources(["Flat"])
        .route("Mode", "Flat.Enums")
        .route("MemberAttribute", "Flat.Attributes")
        .fallback("Flat.Records")
        .output(&output)
        .remap()
        .unwrap();
    let index = reader::Index::read(output.to_str().unwrap()).unwrap();
    for (field, expected) in index.expect("Flat.Records", "Bits").fields().zip([
        TypeName::named("Flat.Enums", "Mode"),
        TypeName::named("System", "Int32"),
    ]) {
        let attr = field.attributes().next().unwrap();
        assert_eq!(attr.namespace(), "Flat.Attributes");
        assert_eq!(
            attr.value(),
            [
                (String::new(), Value::TypeName(expected.clone())),
                ("MemberType".into(), Value::TypeName(expected)),
            ]
        );
    }
    std::fs::remove_file(input).unwrap();
    std::fs::remove_file(output).unwrap();
}

#[test]
fn remapper_reports_configuration_and_input_errors() {
    let missing_output = remap().remap().unwrap_err().to_string();
    assert!(missing_output.contains("error: output is required"));

    let dir = test_dir("remap_errors");
    std::fs::create_dir_all(&dir).unwrap();
    let missing = dir.join("missing.winmd");
    let output = dir.join("output.winmd");
    let invalid_input = remap()
        .input(&missing)
        .output(output)
        .remap()
        .unwrap_err()
        .to_string();
    assert!(invalid_input.contains(&format!("failed to read `{}`", missing.display())));

    let missing_merge_output = merge().merge().unwrap_err().to_string();
    assert!(missing_merge_output.contains("error: output is required"));

    let invalid_merge_input = merge()
        .input(&missing)
        .output(dir.join("merged.winmd"))
        .merge()
        .unwrap_err()
        .to_string();
    assert!(invalid_merge_input.contains(&format!("failed to read `{}`", missing.display())));
}

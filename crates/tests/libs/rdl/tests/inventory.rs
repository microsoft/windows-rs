use windows_rdl::{InventoryEntry, inventory};

fn fixture() -> Vec<InventoryEntry> {
    inventory(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("inventory.rdl")).unwrap()
}

#[test]
fn inventory_preserves_duplicates_members_and_unresolved_types() {
    let entries = fixture();
    let words = entries
        .iter()
        .filter(|entry| entry.name == "Word")
        .collect::<Vec<_>>();
    assert_eq!(words.len(), 3);
    assert_eq!(words[0].kind, "typedef");
    assert_eq!(words[0].architecture, Some(1));
    assert_eq!(words[1].architecture, Some(6));
    assert_eq!(words[2].kind, "constant");
    assert_eq!(words[2].shape, "u32 = 7");
    let text = entries.iter().find(|entry| entry.name == "Text").unwrap();
    assert_eq!(text.shape, "String = \"#[in]\\t//!\"");
    assert!(entries.iter().any(|entry| entry.shape == "* mut Missing"));
    assert_eq!(entries, fixture());
    for (id, entry) in entries.iter().enumerate() {
        assert_eq!(entry.id, id);
        assert!(entry.parent.is_none_or(|parent| parent < id));
        assert!(entry.line > 0);
    }
}

#[test]
fn inventory_retains_nested_layout_attributes_and_bit_padding() {
    let entries = fixture();
    let packet = entries.iter().find(|entry| entry.name == "Packet").unwrap();
    assert!(packet.attributes.contains("packed (1)"));
    let nested = entries
        .iter()
        .find(|entry| entry.kind == "union" && entry.name.is_empty())
        .unwrap();
    assert_eq!(nested.architecture, Some(3));
    assert!(nested.attributes.contains("align (8)"));
    let bits = entries
        .iter()
        .filter(|entry| entry.kind == "bitfield")
        .collect::<Vec<_>>();
    assert_eq!(bits.len(), 3);
    assert_eq!(bits[0].shape, "offset=0, width=1");
    assert_eq!(bits[1].name, "_");
    assert_eq!(bits[1].shape, "offset=1, width=3");
    assert_eq!(bits[2].shape, "offset=4, width=4");
}

#[test]
fn inventory_preserves_callable_positions_imports_and_contracts() {
    let entries = fixture();
    let call = entries.iter().find(|entry| entry.name == "Call").unwrap();
    assert!(call.shape.contains("\"C\""));
    assert!(call.attributes.contains("import = \"NativeCall\""));
    let children = entries
        .iter()
        .filter(|entry| entry.parent == Some(call.id))
        .collect::<Vec<_>>();
    assert_eq!(
        children.iter().map(|entry| entry.kind).collect::<Vec<_>>(),
        ["parameter", "parameter", "variadic", "return"]
    );
    assert_eq!(children[0].name, "buffer");
    assert!(children[0].attributes.contains("r#in"));
    assert!(children[0].attributes.contains("opt"));
    assert_eq!(children[1].name, "count");
    assert!(children[3].attributes.contains("Custom (\"return\")"));
    assert!(entries.iter().any(|entry| entry.kind == "receiver"));
    let event = entries.iter().find(|entry| entry.kind == "event").unwrap();
    assert!(event.attributes.contains("Custom (\"event\")"));
    assert!(entries.iter().any(|entry| entry.kind == "base"));
    assert!(entries.iter().any(|entry| entry.kind == "implements"));
    assert!(entries.iter().any(|entry| entry.kind == "constructor"));
    assert!(entries.iter().any(|entry| entry.kind == "delegate"));
}

#[test]
fn inventory_reports_parse_errors_without_writing_output() {
    let error =
        inventory(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("inventory_invalid.rdl"))
            .unwrap_err();
    assert!(!error.message.is_empty());
    assert_eq!(error.line, 3);
    assert!(error.file_name.ends_with("inventory_invalid.rdl"));
}

#[test]
fn preprocessing_preserves_literal_values_in_inventory_and_winmd() {
    let input = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("inventory_literals.rdl");
    let entries = inventory(&input).unwrap();
    let text = entries.iter().find(|entry| entry.name == "Text").unwrap();
    assert_eq!(text.shape, "String = \"#[in]\\t//!\"");
    assert_eq!(text.line, 6);
    let raw = entries.iter().find(|entry| entry.name == "Raw").unwrap();
    assert_eq!(raw.shape, "String = r#\"#[in] //!\"#");
    let parameter = entries
        .iter()
        .find(|entry| entry.kind == "parameter")
        .unwrap();
    assert!(parameter.attributes.contains("r#in"));
    assert_eq!(parameter.line, 9);

    let output = std::path::Path::new(env!("OUT_DIR")).join("inventory_literals.winmd");
    windows_rdl::reader()
        .input(input)
        .output(&output)
        .write()
        .unwrap();
    let index = windows_metadata::reader::Index::read(output).unwrap();
    let apis = index.expect("Literal", "Apis");
    for (name, value) in [
        ("Unicode", "λ"),
        ("Text", "#[in]\t//!"),
        ("Raw", "#[in] //!"),
    ] {
        let field = apis.fields().find(|field| field.name() == name).unwrap();
        assert_eq!(
            field.constant().unwrap().value(),
            windows_metadata::Value::Utf16(value.to_string()),
        );
    }
}

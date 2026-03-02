use windows_metadata::Value;

/// Replicate the is_removed_attr logic from bindgen to verify it works correctly
fn is_removed_attr(values: &[(String, Value)]) -> bool {
    if values.len() >= 2 {
        if let (_, Value::AttributeEnum(_, val)) = &values[1] {
            return *val == 1;
        }
        if let (_, Value::I32(val)) = &values[1] {
            return *val == 1;
        }
    }
    false
}

#[test]
fn removed_attr_detects_deprecation_type_remove() {
    // DeprecationType.Remove == 1 using AttributeEnum variant
    let values_remove = vec![
        ("".to_string(), Value::Utf8("This API has been removed.".to_string())),
        ("".to_string(), Value::AttributeEnum("DeprecationType".to_string(), 1)),
    ];
    assert!(is_removed_attr(&values_remove), "DeprecationType.Remove (1) should be detected");
}

#[test]
fn removed_attr_allows_deprecation_type_deprecate() {
    // DeprecationType.Deprecate == 0 using AttributeEnum variant
    let values_deprecate = vec![
        ("".to_string(), Value::Utf8("This API is deprecated.".to_string())),
        ("".to_string(), Value::AttributeEnum("DeprecationType".to_string(), 0)),
    ];
    assert!(!is_removed_attr(&values_deprecate), "DeprecationType.Deprecate (0) should not be removed");
}

#[test]
fn removed_attr_handles_i32_variant() {
    // Test with I32 variant (fallback encoding)
    let values_i32_remove = vec![
        ("".to_string(), Value::Utf8("Removed via I32".to_string())),
        ("".to_string(), Value::I32(1)),
    ];
    assert!(is_removed_attr(&values_i32_remove), "I32 variant with value 1 should be removed");

    let values_i32_deprecate = vec![
        ("".to_string(), Value::Utf8("Deprecated via I32".to_string())),
        ("".to_string(), Value::I32(0)),
    ];
    assert!(!is_removed_attr(&values_i32_deprecate), "I32 variant with value 0 should not be removed");
}

#[test]
fn removed_attr_handles_empty_values() {
    let empty: Vec<(String, Value)> = vec![];
    assert!(!is_removed_attr(&empty), "Empty values should not be removed");

    let single = vec![
        ("".to_string(), Value::Utf8("message".to_string())),
    ];
    assert!(!is_removed_attr(&single), "Single value should not be removed");
}

#[test]
fn deprecated_types_still_have_deprecated_annotation() {
    // Verify that the deprecated_enum module has the PlayToConnectionState type with values
    // This proves that deprecated (not removed) types ARE generated
    #[allow(deprecated)]
    {
        use test_bindgen::deprecated_enum::PlayToConnectionState;
        assert_eq!(PlayToConnectionState::Disconnected.0, 0);
        assert_eq!(PlayToConnectionState::Connected.0, 1);
    }
}

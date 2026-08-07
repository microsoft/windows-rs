fn out_path(name: &str) -> std::path::PathBuf {
    std::path::Path::new(env!("OUT_DIR")).join(format!("test_rdl_validation_{name}.winmd"))
}

fn error(name: &str, source: &str) -> windows_rdl::Error {
    windows_rdl::reader()
        .input_text_named("src/test.rdl", source)
        .output(out_path(name))
        .write()
        .unwrap_err()
}

#[test]
fn duplicate_symbols_are_rejected() {
    let cases = [
        (
            "type",
            "#[win32] mod Test { struct Value {} struct Value {} }",
            "duplicate type `Value`",
        ),
        (
            "overlapping_architecture",
            "#[win32] mod Test {
                #[arch(X86 | X64)]
                struct Value {}
                #[arch(X64)]
                struct Value {}
            }",
            "duplicate type `Value`",
        ),
        (
            "function",
            "#[win32] mod Test {
                #[library(\"test.dll\")]
                extern fn Open();
                #[library(\"test.dll\")]
                extern fn Open();
            }",
            "duplicate function `Open`",
        ),
        (
            "constant",
            "#[win32] mod Test { const Value: i32 = 1; const Value: i32 = 2; }",
            "duplicate constant `Value`",
        ),
        (
            "top_level_kind",
            "#[win32] mod Test { struct Value {} const Value: i32 = 1; }",
            "duplicate symbol `Value`",
        ),
        (
            "field",
            "#[win32] mod Test { struct Value { item: i32, item: i32 } }",
            "duplicate field `item`",
        ),
        (
            "nested_field",
            "#[win32] mod Test { struct Value { Anonymous: struct { item: i32, item: i32 } } }",
            "duplicate field `item`",
        ),
        (
            "bit_field",
            "#[win32] mod Test { struct Value { bits: u32 { item: 1, item: 1 } } }",
            "duplicate bit-field member `item`",
        ),
        (
            "enum_variant",
            "#[win32] mod Test { #[repr(i32)] enum Value { Item = 0, Item = 1 } }",
            "duplicate enum variant `Item`",
        ),
        (
            "method",
            "#[winrt] mod Test { interface IValue { fn Get(&self, value: i32); fn Get(&self, value: i32); } }",
            "duplicate method `Get` on `Test.IValue`",
        ),
        (
            "return_type",
            "#[winrt] mod Test { interface IValue { fn Get(&self) -> i32; fn Get(&self) -> u32; } }",
            "duplicate method `Get` on `Test.IValue`",
        ),
        (
            "property",
            "#[winrt] mod Test { interface IValue { Name: String; Name: String; } }",
            "duplicate property `Name` on `Test.IValue`",
        ),
        (
            "property_type",
            "#[winrt] mod Test { interface IValue { #[get] Value: i32; #[set] Value: u32; } }",
            "duplicate property `Value` on `Test.IValue`",
        ),
        (
            "event",
            "#[winrt] mod Test { delegate fn Handler(); interface IValue { event Changed: Handler; event Changed: Handler; } }",
            "duplicate event `Changed` on `Test.IValue`",
        ),
        (
            "interface_member_kind",
            "#[winrt] mod Test { interface IValue { fn Name(&self); Name: String; } }",
            "duplicate interface member `Name`",
        ),
        (
            "attribute_property",
            "#[win32] mod Test { attribute Value { Item: i32, Item: i32, } }",
            "duplicate attribute property `Item`",
        ),
        (
            "attribute_constructor",
            "#[win32] mod Test { attribute Value { fn(item: i32); fn(other: i32); } }",
            "duplicate attribute constructor `Value`",
        ),
        (
            "class_interface",
            "#[winrt] mod Test { interface IValue {} class Value { IValue, IValue, } }",
            "duplicate interface `Test.IValue` on `Test.Value`",
        ),
        (
            "generic_parameter",
            "#[winrt] mod Test { interface IValue<T, T> {} }",
            "duplicate generic parameter `T`",
        ),
        (
            "parameter",
            "#[winrt] mod Test { interface IValue { fn Get(&self, value: i32, value: i32); } }",
            "duplicate parameter `value`",
        ),
    ];

    for (name, source, message) in cases {
        let error = error(name, source);
        assert_eq!(error.code.as_deref(), Some("RDL0001"), "{name}");
        assert_eq!(error.message, message, "{name}");
        assert_eq!(error.file_name, "src/test.rdl", "{name}");
        assert_eq!(error.labels.len(), 2, "{name}");
        assert_eq!(
            error.labels[0].style,
            windows_rdl::LabelStyle::Primary,
            "{name}"
        );
        assert_eq!(
            error.labels[1].style,
            windows_rdl::LabelStyle::Secondary,
            "{name}"
        );
        assert_eq!(error.labels[1].message, "first declared here", "{name}");
    }
}

#[test]
fn duplicate_labels_preserve_both_source_names() {
    let error = windows_rdl::reader()
        .input_texts_named([
            ("src/first.rdl", "#[win32] mod Test { struct Value {} }"),
            ("src/second.rdl", "#[win32] mod Test { struct Value {} }"),
        ])
        .output(out_path("source_names"))
        .write()
        .unwrap_err();

    assert_eq!(error.file_name, "src/second.rdl");
    assert_eq!(error.labels[0].source, "src/second.rdl");
    assert_eq!(error.labels[1].source, "src/first.rdl");
}

#[test]
fn duplicate_signatures_use_resolved_type_identity() {
    for (name, source, message) in [
        (
            "resolved_method",
            r#"
use Other::Value as Alias;

#[winrt]
mod Test {
    interface IValue {
        fn Get(&self, value: Alias);
        fn Get(&self, value: Other::Value);
    }
}

#[winrt]
mod Other {
    struct Value {}
}
"#,
            "duplicate method `Get` on `Test.IValue`",
        ),
        (
            "resolved_attribute",
            r#"
use Other::Value as Alias;

#[win32]
mod Test {
    attribute Marker {
        fn(value: Alias);
        fn(value: Other::Value);
    }
}

#[win32]
mod Other {
    struct Value {}
}
"#,
            "duplicate attribute constructor `Marker`",
        ),
    ] {
        let error = error(name, source);
        assert_eq!(error.code.as_deref(), Some("RDL0001"));
        assert_eq!(error.message, message);
    }
}

#[test]
fn unresolved_types_are_collected_before_encoding() {
    let report = windows_rdl::reader()
        .input_text_named(
            "src/test.rdl",
            r#"
#[winrt]
mod Test {
    interface IValue {
        fn First(&self, value: MissingFirst);
        fn Second(&self, value: MissingSecond);
    }
}
"#,
        )
        .check_all();

    assert_eq!(report.diagnostics().len(), 2);
    assert!(
        report
            .diagnostics()
            .iter()
            .all(|diagnostic| diagnostic.message == "type not found")
    );
}

#[test]
fn generic_arity_is_validated_on_resolved_types() {
    let report = windows_rdl::reader()
        .input_text_named(
            "src/test.rdl",
            r#"
#[winrt]
mod Test {
    interface IVector<T> {
        fn Append(&self, value: T);
    }
    interface IUses {
        fn Missing(&self, value: IVector);
        fn Extra(&self, value: IVector<i32, u32>);
    }
}
"#,
        )
        .check_all();

    assert_eq!(report.diagnostics().len(), 2);
    assert!(
        report
            .diagnostics()
            .iter()
            .all(|diagnostic| diagnostic.code.as_deref() == Some("RDL0005"))
    );
    assert!(
        report
            .diagnostics()
            .iter()
            .any(|diagnostic| diagnostic.message.contains("but 0 were provided"))
    );
    assert!(
        report
            .diagnostics()
            .iter()
            .any(|diagnostic| diagnostic.message.contains("but 2 were provided"))
    );
}

#[test]
fn unrepresentable_syntax_is_rejected() {
    let cases = [
        (
            "event_attributes",
            "#[winrt] mod Test { interface IValue { #[Marker] event Changed: Object; } }",
            "attributes on event shorthand are not represented",
        ),
        (
            "function_generics",
            "#[win32] mod Test { #[library(\"test.dll\")] extern fn Open<T>(); }",
            "generic parameters are not supported on functions",
        ),
        (
            "callback_generics",
            "#[win32] mod Test { extern fn Callback<T>(); }",
            "generic parameters are not supported on callbacks",
        ),
        (
            "method_generics",
            "#[winrt] mod Test { interface IValue { fn Get<T>(&self); } }",
            "generic parameters are not supported on interface methods",
        ),
        (
            "callback_variadic",
            "#[win32] mod Test { extern \"C\" fn Callback(...); }",
            "variadic parameters are not supported on callbacks",
        ),
        (
            "delegate_variadic",
            "#[winrt] mod Test { delegate fn Handler(...); }",
            "variadic parameters are not supported on delegates",
        ),
        (
            "method_variadic",
            "#[winrt] mod Test { interface IValue { fn Get(&self, ...); } }",
            "variadic parameters are not supported on interface methods",
        ),
        (
            "attribute_constructor_variadic",
            "#[win32] mod Test { attribute Value { fn(...); } }",
            "variadic attribute constructors are not supported",
        ),
        (
            "attribute_constructor_return",
            "#[win32] mod Test { attribute Value { fn() -> i32; } }",
            "attribute constructors cannot return a value",
        ),
        (
            "enum_variant_fields",
            "#[win32] mod Test { #[repr(i32)] enum Value { Item(i32) = 0 } }",
            "enum variants with fields are not supported",
        ),
        (
            "generic_attributes",
            "#[winrt] mod Test { interface IValue<#[Marker] T> {} }",
            "attributes on generic parameters are not represented",
        ),
        (
            "generic_bounds",
            "#[winrt] mod Test { interface IValue<T: Marker> {} }",
            "generic parameter bounds are not represented",
        ),
        (
            "generic_defaults",
            "#[winrt] mod Test { interface IValue<T = i32> {} }",
            "generic parameter defaults are not represented",
        ),
        (
            "const_generics",
            "#[winrt] mod Test { interface IValue<const N: usize> {} }",
            "only type generic parameters are supported on interfaces",
        ),
    ];

    for (name, source, message) in cases {
        let error = error(name, source);
        assert_eq!(error.code.as_deref(), Some("RDL0002"), "{name}");
        assert_eq!(error.message, message, "{name}");
        assert_eq!(error.file_name, "src/test.rdl", "{name}");
        assert_eq!(error.labels.len(), 1, "{name}");
        assert_eq!(
            error.labels[0].message, "not represented in metadata",
            "{name}"
        );
    }
}

#[test]
fn invalid_event_handler_is_rejected() {
    let error = error(
        "invalid_event_handler",
        "#[winrt] mod Test { interface IValue { event Changed: Object; } }",
    );
    assert_eq!(
        error.message,
        "event handler must be a delegate or class type"
    );
    assert_eq!(error.file_name, "src/test.rdl");
    assert_eq!(error.labels.len(), 1);
}

#[test]
fn method_overloads_are_allowed() {
    windows_rdl::reader()
        .input_text(
            "#[winrt] mod Test {
                interface IValue {
                    fn Get(&self, value: i32);
                    fn Get(&self, value: String);
                }
            }",
        )
        .output(out_path("method_overloads"))
        .write()
        .unwrap();
}

#[test]
fn explicit_overloads_emit_projected_and_metadata_names() {
    use windows_metadata::HasAttributes;

    let path = out_path("explicit_overloads");
    windows_rdl::reader()
        .input_text(
            "#[winrt] mod Test {
                interface IValue {
                    #[overload(Get)]
                    #[default_overload]
                    fn Get(&self, value: i32);
                    #[overload(GetWithString)]
                    fn Get(&self, value: String);
                }
            }",
        )
        .output(&path)
        .write()
        .unwrap();

    let index = windows_metadata::reader::Index::read(&path).unwrap();
    let methods: Vec<_> = index.expect("Test", "IValue").methods().collect();
    assert_eq!(methods.len(), 2);
    assert_eq!(methods[0].name(), "Get");
    assert_eq!(
        methods[0]
            .find_attribute("OverloadAttribute")
            .unwrap()
            .value()[0]
            .1,
        windows_metadata::Value::Utf8("Get".to_string())
    );
    assert!(methods[0].has_attribute("DefaultOverloadAttribute"));
    assert_eq!(methods[1].name(), "GetWithString");
    assert_eq!(
        methods[1]
            .find_attribute("OverloadAttribute")
            .unwrap()
            .value()[0]
            .1,
        windows_metadata::Value::Utf8("Get".to_string())
    );

    let rdl = path.with_extension("rdl");
    windows_rdl::writer()
        .input(&path)
        .output(&rdl)
        .write()
        .unwrap();
    let source = std::fs::read_to_string(rdl).unwrap();
    assert!(source.contains("#[overload(Get)]"));
    assert!(source.contains("#[default_overload]"));
    assert!(source.contains("#[overload(GetWithString)]"));
    assert_eq!(source.matches("fn Get(").count(), 2);
}

#[test]
fn overload_validation_reports_source_errors() {
    let cases = [
        (
            "default_without_overload",
            "#[winrt] mod Test {
                interface IValue {
                    #[default_overload]
                    fn Get(&self);
                }
            }",
            "`default_overload` requires an `overload` attribute",
        ),
        (
            "duplicate_overload_signature",
            "#[winrt] mod Test {
                interface IValue {
                    #[overload(GetFirst)]
                    fn Get(&self, value: i32);
                    #[overload(GetSecond)]
                    fn Get(&self, value: i32);
                }
            }",
            "duplicate overload signature `Get` on `Test.IValue`",
        ),
        (
            "duplicate_default_overload",
            "#[winrt] mod Test {
                interface IValue {
                    #[overload(GetFirst)]
                    #[default_overload]
                    fn Get(&self, value: i32);
                    #[overload(GetSecond)]
                    #[default_overload]
                    fn Get(&self, value: String);
                }
            }",
            "duplicate default overload `Get` on `Test.IValue`",
        ),
    ];

    for (name, source, message) in cases {
        assert_eq!(error(name, source).message, message, "{name}");
    }
}

#[test]
fn overload_metadata_names_may_repeat_for_distinct_signatures() {
    windows_rdl::reader()
        .input_text(
            "#[winrt] mod Test {
                interface IValue {
                    #[overload(Get)]
                    fn Get(&self, value: i32);
                    #[overload(Get)]
                    fn Get(&self, value: String);
                }
            }",
        )
        .output(out_path("repeated_overload_metadata_name"))
        .write()
        .unwrap();
}

#[test]
fn unrelated_overload_attribute_is_preserved() {
    let path = out_path("custom_overload_attribute");
    windows_rdl::reader()
        .input_text(
            "#[winrt] mod Test {
                attribute OverloadAttribute { fn(value: String); }
                interface IValue {
                    #[Test::Overload(\"custom\")]
                    fn Get(&self);
                }
            }",
        )
        .output(&path)
        .write()
        .unwrap();

    let rdl = path.with_extension("rdl");
    windows_rdl::writer()
        .input(&path)
        .output(&rdl)
        .write()
        .unwrap();
    let source = std::fs::read_to_string(rdl).unwrap();
    assert!(source.contains("#[Overload(\"custom\")]"));
    assert!(!source.contains("#[overload("));
}

#[test]
fn disjoint_architecture_variants_are_allowed() {
    windows_rdl::reader()
        .input_text(
            "#[win32] mod Test {
                #[arch(X86)]
                struct Value { item: i32 }
                #[arch(X64)]
                struct Value { item: i64 }
            }",
        )
        .output(out_path("architecture_variants"))
        .write()
        .unwrap();
}

#[test]
fn split_property_accessors_are_allowed() {
    windows_rdl::reader()
        .input_text(
            "#[winrt] mod Test {
                interface IValue {
                    #[get]
                    Name: String;
                    #[set]
                    Name: String;
                }
            }",
        )
        .output(out_path("split_property"))
        .write()
        .unwrap();
}

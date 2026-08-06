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
            "duplicate method `Get`",
        ),
        (
            "return_type",
            "#[winrt] mod Test { interface IValue { fn Get(&self) -> i32; fn Get(&self) -> u32; } }",
            "duplicate method `Get`",
        ),
        (
            "property",
            "#[winrt] mod Test { interface IValue { Name: String; Name: String; } }",
            "duplicate property `Name`",
        ),
        (
            "property_type",
            "#[winrt] mod Test { interface IValue { #[get] Value: i32; #[set] Value: u32; } }",
            "duplicate property `Value`",
        ),
        (
            "event",
            "#[winrt] mod Test { interface IValue { event Changed: Object; event Changed: Object; } }",
            "duplicate event `Changed`",
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
            "duplicate class interface `IValue`",
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

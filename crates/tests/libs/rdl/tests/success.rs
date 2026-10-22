use windows_metadata::{HasAttributes, reader};
use windows_rdl::emit::SuccessCondition;

#[test]
fn success_expressions_roundtrip() {
    let source = r#"
        #[win32] mod Test {
            #[success(return >= 0)] type Status = i32;
            #[success(return == 0)] #[library("test.dll")] extern fn Zero() -> Status;
            #[success(return != 0)] #[library("test.dll")] extern fn Nonzero() -> i32;
            #[success(return == 1)] extern fn Callback() -> i32;
            #[no_guid] interface Object {
                #[success(return > 0)] fn Positive(&self) -> i32;
            }
        }
    "#;
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let output = std::path::Path::new(env!("OUT_DIR"));
    let first = output.join("success.winmd");
    let rdl = output.join("success.rdl");
    let second = output.join("success-again.winmd");
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_text(source)
        .output(&first)
        .write()
        .unwrap();
    windows_rdl::writer()
        .input(&first)
        .filter("Test")
        .output(&rdl)
        .write()
        .unwrap();
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input(&rdl)
        .output(&second)
        .write()
        .unwrap();
    let text = std::fs::read_to_string(&rdl).unwrap();
    for expression in [
        "return == 0",
        "return != 0",
        "return == 1",
        "return >= 0",
        "return > 0",
    ] {
        assert!(text.contains(&format!("#[success({expression})]")));
    }
    for path in [&first, &second] {
        let index = reader::Index::read(path).unwrap();
        let check = |attribute: reader::Attribute<'_>, expected: SuccessCondition| {
            assert_eq!(
                attribute.value(),
                vec![(String::new(), expected.to_value())]
            );
        };
        check(
            index
                .expect("Test", "Status")
                .find_attribute("SuccessConditionAttribute")
                .unwrap(),
            SuccessCondition::Nonnegative,
        );
        check(
            index
                .expect("Test", "Callback")
                .find_attribute("SuccessConditionAttribute")
                .unwrap(),
            SuccessCondition::EqualOne,
        );
        for (owner, name, expected) in [
            ("Apis", "Zero", SuccessCondition::EqualZero),
            ("Apis", "Nonzero", SuccessCondition::NotEqualZero),
            ("Object", "Positive", SuccessCondition::Positive),
        ] {
            let method = index
                .expect("Test", owner)
                .methods()
                .find(|method| method.name() == name)
                .unwrap();
            check(
                method.find_attribute("SuccessConditionAttribute").unwrap(),
                expected,
            );
        }
    }
}

#[test]
fn invalid_success_expressions() {
    for expression in [
        "",
        "return",
        "true",
        "return = 0",
        "return == 2",
        "return >= 0u32",
        "return ! = 0",
        "return == 0 || return == 1",
        "value == 0",
        "\"return == 0\"",
    ] {
        let source =
            format!("#[win32] mod Test {{ #[success({expression})] extern fn Call() -> i32; }}");
        let error = windows_rdl::reader()
            .input_text(&source)
            .output(std::path::Path::new(env!("OUT_DIR")).join("invalid-success.winmd"))
            .write()
            .unwrap_err();
        assert!(error.to_string().contains("success"), "{error}");
    }
    for source in [
        "#[win32] mod Test { struct Value { #[success(return == 0)] field: i32, } }",
        "#[win32] mod Test { extern fn Call(#[success(return == 0)] value: i32); }",
    ] {
        let error = windows_rdl::reader()
            .input_text(source)
            .output(std::path::Path::new(env!("OUT_DIR")).join("invalid-owner.winmd"))
            .write()
            .unwrap_err();
        assert!(error.to_string().contains("owner"), "{error}");
    }
}

#[test]
fn unfamiliar_success_payloads_keep_full_spelling() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let schema = std::fs::read_to_string(root.join("metadata\\metadata.rdl"))
        .unwrap()
        .replace("Positive = 4,", "Positive = 4,\nFuture = 99,")
        .replace(
            "fn(#[in] condition: SuccessCondition);",
            "fn(#[in] condition: SuccessCondition);\nExtra: i32,",
        );
    let source = r#"
        #[win32] mod Test {
            #[Windows::Win32::Metadata::SuccessCondition(Future)]
            extern fn Future() -> i32;
            #[Windows::Win32::Metadata::SuccessCondition(EqualZero, Extra = 1)]
            extern fn Extra() -> i32;
        }
    "#;
    let output = std::path::Path::new(env!("OUT_DIR"));
    let first = output.join("success-future.winmd");
    let rdl = first.with_extension("rdl");
    let second = output.join("success-future-again.winmd");
    windows_rdl::reader()
        .input_text(&schema)
        .input_text(source)
        .output(&first)
        .write()
        .unwrap();
    windows_rdl::writer()
        .input(&first)
        .filter("Test")
        .output(&rdl)
        .write()
        .unwrap();
    assert!(
        !std::fs::read_to_string(&rdl)
            .unwrap()
            .contains("#[success(")
    );
    windows_rdl::reader()
        .input_text(&schema)
        .input(&rdl)
        .output(&second)
        .write()
        .unwrap();
    let mut previous = None;
    for path in [first, second] {
        let index = reader::Index::read(path).unwrap();
        let values: Vec<_> = ["Future", "Extra"]
            .map(|name| {
                index
                    .expect("Test", name)
                    .find_attribute("SuccessConditionAttribute")
                    .unwrap()
                    .value()
            })
            .into_iter()
            .collect();
        if let Some(previous) = &previous {
            assert_eq!(&values, previous);
        }
        previous = Some(values);
    }
}

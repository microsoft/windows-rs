use windows_clang::*;
use windows_metadata::{
    BufferAccess, BufferCount, BufferPhase, HasAttributes, IntegerExpression, IntegerLiteral,
    IntegerReference, reader,
};

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

fn input() -> Input {
    let mut input = test_clang::annotated_sdk_fixture();
    input.name = "count_expressions.h".into();
    input.source = test_clang::fixture("count_expressions").source;
    input.roots.clear();
    input
}

fn imports() -> projection::Imports {
    [(
        "ReadCount".into(),
        projection::Import {
            library: "fixture.dll".into(),
            name: None,
        },
    )]
    .into_iter()
    .collect()
}

#[test]
#[ignore = "requires the pinned SDK"]
fn count_expressions() {
    helpers::ensure_libclang();
    let snapshots = capture(&[input()], &TARGETS).unwrap();
    let output =
        projection::project_with_imports(&merge(&snapshots).unwrap(), "Fixture", &imports())
            .unwrap();
    let text: String = output
        .partitions
        .values()
        .map(|part| part.rdl.as_str())
        .collect();
    let expected = test_clang::input_dir().join("count_expressions.rdl");
    if std::env::var_os("UPDATE_CLANG_RDL").is_some() {
        std::fs::write(&expected, &text).unwrap();
    }
    assert_eq!(
        text,
        std::fs::read_to_string(expected)
            .unwrap()
            .replace("\r\n", "\n")
    );
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let first = root.join("target\\test-clang-count-expressions.winmd");
    let rdl = first.with_extension("rdl");
    let second = first.with_extension("again.winmd");
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_text(&text)
        .output(&first)
        .write()
        .unwrap();
    windows_rdl::writer()
        .input(&first)
        .filter("Fixture")
        .output(&rdl)
        .write()
        .unwrap();
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input(&rdl)
        .output(&second)
        .write()
        .unwrap();
    for path in [&first, &second] {
        for architecture in [1, 2, 4] {
            let index = reader::Index::new_for_architecture(
                vec![reader::File::read(path).unwrap()],
                architecture,
            );
            let parameter = |name| {
                index
                    .expect("Fixture", name)
                    .methods()
                    .next()
                    .unwrap()
                    .params()
                    .next()
                    .unwrap()
            };
            for (name, phase, count) in [
                ("ReadFixed", BufferPhase::Entry, 32),
                ("FullFixed", BufferPhase::Success, 32),
                ("ZeroReturned", BufferPhase::Success, 0),
                ("RpcFixed", BufferPhase::Entry, 32),
            ] {
                let param = parameter(name);
                let extent = test_clang::buffer_extent(param, BufferAccess::Read, phase).unwrap();
                assert_eq!(extent.count, BufferCount::Constant(count));
                assert_eq!(extent.offset, 0);
                assert!(!extent.count_on_entry && !extent.count_optional);
                assert_eq!(extent.indirect, name == "ZeroReturned");
                assert!(
                    param
                        .find_attribute("NativeReturnedPointerAttribute")
                        .is_none()
                );
            }
            for (name, source, old, optional, fallback) in [
                ("FullOffset", 1, true, true, None),
                ("PartialOffset", 2, false, true, None),
                ("FallbackOffset", 2, false, false, Some(1)),
                ("ReturnedOffset", 2, false, true, None),
                ("RpcFull", 1, false, false, None),
            ] {
                let extent = test_clang::buffer_extent(
                    parameter(name),
                    BufferAccess::Read,
                    BufferPhase::Success,
                )
                .unwrap();
                assert_eq!(
                    extent.count,
                    BufferCount::Expression(IntegerExpression::Add(
                        Box::new(IntegerExpression::Reference(if optional {
                            IntegerReference::OptionalDereference(source)
                        } else {
                            IntegerReference::Dereference(source)
                        })),
                        Box::new(IntegerExpression::Literal(IntegerLiteral::I32(1))),
                    )),
                    "{name}"
                );
                assert_eq!(extent.offset, 0, "{name}");
                assert_eq!(extent.count_on_entry, old, "{name}");
                assert!(!extent.count_optional, "{name}");
                assert_eq!(extent.null_fallback, fallback, "{name}");
            }
            for name in [
                "Octal",
                "Suffix",
                "Overflow",
                "Huge",
                "Negative",
                "Nested",
                "PointerArithmetic",
                "Cast",
                "ReturnLength",
            ] {
                let param = parameter(name);
                assert!(
                    param.find_attribute("NativeSalAttribute").is_some(),
                    "{name}"
                );
                assert!(
                    test_clang::buffer_extent(param, BufferAccess::Read, BufferPhase::Success)
                        .is_none(),
                    "{name}"
                );
                if name != "ReturnLength" {
                    assert!(
                        param.find_attribute("BufferExtentAttribute").is_none(),
                        "{name}"
                    );
                }
                let hex = test_clang::buffer_extent(
                    parameter("Hexadecimal"),
                    BufferAccess::Write,
                    BufferPhase::Entry,
                )
                .unwrap();
                assert_eq!(hex.count, BufferCount::Constant(32));
            }
            assert_eq!(
                test_clang::buffer_extent(
                    parameter("Product"),
                    BufferAccess::Write,
                    BufferPhase::Entry,
                )
                .unwrap()
                .count,
                BufferCount::Expression(IntegerExpression::Multiply(
                    Box::new(IntegerExpression::Reference(IntegerReference::Parameter(1),)),
                    Box::new(IntegerExpression::Literal(IntegerLiteral::I32(2),)),
                )),
            );
        }
    }
}

#[test]
#[ignore = "requires the pinned SDK"]
fn count_agreement() {
    helpers::ensure_libclang();
    let mut inputs = [input(), input()];
    inputs[1].name = "second.h".into();
    inputs[1].source = inputs[1]
        .source
        .replace("(31 + 1)", "32")
        .replace("count + 1", "((renamed) + (1))")
        .replace("Count count", "Count renamed");
    for input in &mut inputs {
        input.roots = vec!["ReadFixed".into(), "ReadCount".into()];
    }
    for _ in 0..2 {
        let snapshots = capture(&inputs, &TARGETS).unwrap();
        projection::project_with_imports(&merge(&snapshots).unwrap(), "Fixture", &imports())
            .unwrap();
        inputs.reverse();
    }
    let mut inputs = [input(), input()];
    inputs[1].name = "conflicting.h".into();
    inputs[1].source = inputs[1].source.replace("(31 + 1)", "33");
    for input in &mut inputs {
        input.roots = vec!["ReadFixed".into()];
    }
    for _ in 0..2 {
        let error = projection::generate(&inputs, "Fixture").unwrap_err();
        assert!(error.to_string().contains("conflicting"), "{error}");
        let error = projection::generate_partial(&inputs, "Fixture").unwrap_err();
        assert!(error.to_string().contains("conflicting"), "{error}");
        inputs.reverse();
    }
}

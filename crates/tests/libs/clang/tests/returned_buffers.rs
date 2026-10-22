use windows_clang::*;
use windows_metadata::{
    BufferAccess, BufferCount, BufferPhase, HasAttributes, IntegerExpression, IntegerLiteral,
    IntegerReference, reader,
};

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

fn imports() -> projection::Imports {
    ["Fetch", "Lookup"]
        .into_iter()
        .map(|name| {
            (
                name.into(),
                projection::Import {
                    library: "fixture.dll".into(),
                    name: None,
                },
            )
        })
        .collect()
}

fn input(name: &str) -> Input {
    let mut input = test_clang::annotated_sdk_fixture();
    input.name = format!("{name}.h");
    input.source = test_clang::fixture(name).source;
    input.roots.clear();
    input
}

fn roundtrip(output: &projection::Output, name: &str, check: impl Fn(&reader::Index)) {
    let text: String = output
        .partitions
        .values()
        .map(|part| part.rdl.as_str())
        .collect();
    let expected = test_clang::input_dir().join(format!("{name}.rdl"));
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
    let first = root.join(format!("target\\test-clang-{name}.winmd"));
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
            check(&index);
        }
    }
}

#[test]
#[ignore = "requires the pinned SDK"]
fn returned_buffers() {
    helpers::ensure_libclang();
    let snapshots = capture(&[input("returned_buffers")], &TARGETS).unwrap();
    let output =
        projection::project_with_imports(&merge(&snapshots).unwrap(), "Fixture", &imports())
            .unwrap();
    roundtrip(&output, "returned_buffers", |index| {
        for name in ["NullableResult", "NullableResultAlias"] {
            let method = index.expect("Fixture", name).methods().next().unwrap();
            let result = method.params().find(|param| param.sequence() == 0).unwrap();
            assert!(
                result
                    .find_attribute("NullableAttribute")
                    .unwrap()
                    .value()
                    .is_empty()
            );
            assert!(!method.has_attribute("NullableAttribute"));
        }
        for name in ["data", "object"] {
            let field = index
                .expect("Fixture", "NullableFields")
                .fields()
                .find(|field| field.name() == name)
                .unwrap();
            assert!(
                field
                    .find_attribute("NullableAttribute")
                    .unwrap()
                    .value()
                    .is_empty()
            );
        }
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
        for name in ["OptionalCounts", "OptionalStorage"] {
            let parameter = parameter(name);
            assert_eq!(parameter.is_optional(), name == "OptionalStorage");
            for (access, count) in [(BufferAccess::Write, 1), (BufferAccess::Read, 2)] {
                let extent =
                    test_clang::buffer_extent(parameter, access, BufferPhase::Success).unwrap();
                assert_eq!(extent.count, BufferCount::Dereference(count));
                assert!(extent.indirect && extent.count_optional && !extent.count_on_entry);
                assert_eq!(extent.nullable, name == "OptionalStorage");
                assert!(extent.null_fallback.is_none());
            }
        }
        for name in ["Capacity", "AliasStorage", "Repeated"] {
            let parameter = parameter(name);
            assert!(
                test_clang::buffer_extent(parameter, BufferAccess::Write, BufferPhase::Success)
                    .unwrap()
                    .indirect
            );
            assert!(
                test_clang::buffer_extent(parameter, BufferAccess::Read, BufferPhase::Success)
                    .is_none()
            );
        }
        for name in ["PointerElements", "CallbackElements", "InterfaceElements"] {
            let extent = test_clang::buffer_extent(
                parameter(name),
                BufferAccess::Read,
                BufferPhase::Success,
            )
            .unwrap();
            assert_eq!(extent.count, BufferCount::Parameter(1));
            assert!(extent.indirect);
        }
        let full = parameter("Full");
        let extent =
            test_clang::buffer_extent(full, BufferAccess::Read, BufferPhase::Success).unwrap();
        assert!(extent.indirect && !extent.count_on_entry && !extent.count_optional);
        assert!(
            test_clang::buffer_extent(full, BufferAccess::Write, BufferPhase::Success).is_none()
        );
        for name in ["FailurePointer", "FailurePointerReversed"] {
            let parameter = parameter(name);
            assert!(
                parameter
                    .find_attribute("NativeReturnedPointerAttribute")
                    .unwrap()
                    .value()
                    .contains(&("NullOnFailure".into(), windows_metadata::Value::Bool(true)))
            );
            assert!(
                test_clang::buffer_extent(parameter, BufferAccess::Write, BufferPhase::Success)
                    .unwrap()
                    .indirect
            );
            assert!(parameter.find_attribute("NativeSalAttribute").is_none());
        }
        for name in ["Distinct", "MismatchedNullable", "MismatchedOptional"] {
            let parameter = parameter(name);
            assert!(parameter.find_attribute("NativeSalAttribute").is_some());
            assert!(parameter.find_attribute("BufferExtentAttribute").is_none());
        }
        let zero = parameter("Constant");
        let extent =
            test_clang::buffer_extent(zero, BufferAccess::Read, BufferPhase::Success).unwrap();
        assert_eq!(extent.count, BufferCount::Constant(0));
        assert!(extent.indirect);
        assert!(
            zero.find_attribute("NativeReturnedPointerAttribute")
                .is_none()
        );
        let extent = test_clang::buffer_extent(
            parameter("Expression"),
            BufferAccess::Write,
            BufferPhase::Success,
        )
        .unwrap();
        assert_eq!(
            extent.count,
            BufferCount::Expression(IntegerExpression::Add(
                Box::new(IntegerExpression::Reference(IntegerReference::Parameter(1))),
                Box::new(IntegerExpression::Literal(IntegerLiteral::I32(1))),
            ))
        );
        assert_eq!(extent.offset, 0);
        let incoming = parameter("OptionalIncoming");
        for (access, phase) in [
            (BufferAccess::Write, BufferPhase::Entry),
            (BufferAccess::Read, BufferPhase::Success),
        ] {
            let extent = test_clang::buffer_extent(incoming, access, phase).unwrap();
            assert!(extent.count_optional);
            assert_eq!(extent.count_on_entry, phase == BufferPhase::Success);
        }
        let fallback = test_clang::buffer_extent(
            parameter("Fallback"),
            BufferAccess::Read,
            BufferPhase::Success,
        )
        .unwrap();
        assert_eq!(fallback.null_fallback, Some(1));
        assert!(!fallback.count_optional);
    });
}

#[test]
#[ignore = "requires the pinned SDK"]
fn returned_buffer_agreement() {
    helpers::ensure_libclang();
    let mut inputs = [input("returned_buffers"), input("returned_buffers")];
    inputs[1].name = "renamed.h".into();
    inputs[1].source = inputs[1]
        .source
        .replace("capacity", "available")
        .replace("length", "written");
    for input in &mut inputs {
        input.roots = vec!["Fetch".into()];
    }
    for _ in 0..2 {
        let snapshots = capture(&inputs, &TARGETS).unwrap();
        projection::project_with_imports(&merge(&snapshots).unwrap(), "Fixture", &imports())
            .unwrap();
        inputs.reverse();
    }
    let mut inputs = [input("returned_buffers"), input("returned_buffers")];
    inputs[1].name = "reordered.h".into();
    inputs[1].source = inputs[1].source.replace(
        "_Outptr_result_nullonfailure_ _Outptr_result_buffer_(capacity)",
        "_Outptr_result_buffer_(capacity) _Outptr_result_nullonfailure_",
    );
    for input in &mut inputs {
        input.roots = vec!["FailurePointer".into()];
    }
    for _ in 0..2 {
        let snapshots = capture(&inputs, &TARGETS).unwrap();
        projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap();
        inputs.reverse();
    }
    let mut inputs = [input("returned_buffers"), input("returned_buffers")];
    inputs[1].name = "conflict.h".into();
    inputs[1].source = inputs[1].source.replace(
        "_Outptr_result_buffer_(capacity)",
        "_Outptr_result_buffer_all_(capacity)",
    );
    for input in &mut inputs {
        input.roots = vec!["Capacity".into()];
    }
    for _ in 0..2 {
        let error = projection::generate(&inputs, "Fixture").unwrap_err();
        assert!(error.to_string().contains("conflicting"), "{error}");
        let error = projection::generate_partial(&inputs, "Fixture").unwrap_err();
        assert!(error.to_string().contains("conflicting"), "{error}");
        inputs.reverse();
    }
}

#[test]
#[ignore = "requires the pinned SDK"]
fn invalid_returned_buffers() {
    helpers::ensure_libclang();
    for (case, message) in [
        (1, "pointer-to-pointer"),
        (2, "requires an integer"),
        (3, "requires a pointer to an integer"),
        (4, "pointer-to-pointer data"),
        (5, "pointer-to-pointer data"),
        (6, "requires an integer"),
    ] {
        let mut input = input("returned_buffers_invalid");
        input
            .source
            .insert_str(0, &format!("#define CASE {case}\n"));
        let error = projection::generate(std::slice::from_ref(&input), "Fixture").unwrap_err();
        assert!(error.to_string().contains(message), "{case}: {error}");
        let partial = projection::generate_partial(&[input], "Fixture").unwrap();
        assert!(
            partial
                .omissions
                .iter()
                .any(|value| value.contains(message)),
            "{case}: {:?}",
            partial.omissions
        );
        assert!(
            partial
                .output
                .partitions
                .values()
                .all(|part| !part.rdl.contains("fn Invalid("))
        );
    }
}

#[test]
#[ignore = "requires the pinned SDK"]
fn media_buffer_lock() {
    helpers::ensure_libclang();
    let mut input = test_clang::annotated_sdk_fixture();
    input.name = "returned_buffers_sdk.h".into();
    input.source = "#include <mfobjects.h>".into();
    input.roots = vec!["IMFMediaBuffer".into()];
    let snapshots = capture(&[input], &TARGETS).unwrap();
    let output = projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap();
    roundtrip(&output, "returned_buffers_sdk", |index| {
        let lock = index
            .expect("Fixture", "IMFMediaBuffer")
            .methods()
            .find(|method| method.name() == "Lock")
            .unwrap();
        let buffer = lock.params().next().unwrap();
        assert!(
            buffer
                .find_attribute("NativeReturnedPointerAttribute")
                .is_none()
        );
        assert_eq!(buffer.direction(), reader::ParamDirection::Output);
        assert!(!buffer.is_optional());
        for (access, count) in [(BufferAccess::Write, 1), (BufferAccess::Read, 2)] {
            let extent = test_clang::buffer_extent(buffer, access, BufferPhase::Success).unwrap();
            assert_eq!(extent.count, BufferCount::Dereference(count));
            assert_eq!(extent.units, windows_metadata::BufferUnits::Bytes);
            assert!(
                extent.indirect
                    && extent.count_optional
                    && !extent.nullable
                    && !extent.count_on_entry
            );
        }
        assert!(
            lock.params()
                .skip(1)
                .all(|parameter| parameter.is_optional())
        );
    });
}

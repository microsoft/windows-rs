use std::path::Path;
use windows_clang2::{Input, ProjectionOptions, ReferenceKind, TypeReference, capture};

const ARGS: &[&str] = &["-x", "c++", "--target=x86_64-pc-windows-msvc"];

#[test]
fn opaque_uuid_class_preserves_source_identity() {
    let header = Path::new(env!("CARGO_MANIFEST_DIR")).join("input/class.h");
    let source = Input::new(header.to_str().unwrap(), include_str!("../input/class.h"));
    let plain = Input::new("plain.h", "class NativeClass;");
    let options = ProjectionOptions::new("Test");
    let expected = include_str!("../expected/class.rdl").replace("\r\n", "\n");
    for inputs in [vec![source.clone(), plain.clone()], vec![plain, source]] {
        let snapshot = capture(inputs, ARGS, &["NativeClass"]).unwrap();
        let resolved = snapshot.resolve().unwrap();
        assert!(
            resolved
                .report()
                .incomplete
                .iter()
                .any(|name| name == "NativeClass")
        );
        let plan = resolved.project(&options).unwrap();
        assert_eq!(plan.rdl(), expected);
        let partitions = plan.rdl_by_header().unwrap();
        assert_eq!(partitions.len(), 1);
        let (owner, text) = partitions.first_key_value().unwrap();
        assert_eq!(Path::new(owner), header);
        assert_eq!(*text, expected);
    }
}

#[test]
fn uuid_on_a_separate_declaration_survives_completion() {
    let declaration = Input::new("uuid.h", include_str!("../input/uuid_declaration.h"));
    let definition = Input::new(
        "definition.h",
        format!(
            "struct IUnknown {{ virtual long __stdcall Query() = 0; }};\n{}",
            include_str!("../input/uuid_definition.h")
        ),
    );
    let mut options = ProjectionOptions::new("Test");
    options.references.insert(
        "IUnknown".into(),
        TypeReference {
            namespace: "External".into(),
            name: "IUnknown".into(),
            kind: ReferenceKind::Interface,
        },
    );
    for inputs in [
        vec![declaration.clone(), definition.clone()],
        vec![definition, declaration],
    ] {
        let snapshot = capture(inputs, ARGS, &["IValue"]).unwrap();
        let plan = snapshot.resolve().unwrap().project(&options).unwrap();
        assert_eq!(
            plan.rdl(),
            include_str!("../expected/uuid_completion.rdl").replace("\r\n", "\n")
        );
    }
}

#[test]
fn conflicting_class_uuids_are_not_hidden_by_a_plain_declaration() {
    let source = include_str!("../input/class.h");
    let conflicting = source.replace("BCDE0395", "ACDE0395");
    let snapshot = capture(
        [
            Input::new("original.h", source),
            Input::new("conflicting.h", conflicting),
            Input::new("plain.h", "class NativeClass;"),
        ],
        ARGS,
        &["NativeClass"],
    )
    .unwrap();
    let error = snapshot.resolve().err().unwrap();
    assert!(error.to_string().contains("UUIDs differ"), "{error}");
}

#[test]
fn identity_does_not_supply_layout_or_missing_values() {
    let mut options = ProjectionOptions::new("Test");
    options.library = Some("test.dll".into());
    for root in ["Missing", "Empty", "Data", "UseIdentity", "UnknownValue"] {
        let snapshot = capture(
            [Input::new(
                "limits.h",
                include_str!("../input/class_limits.h"),
            )],
            ARGS,
            &[root],
        )
        .unwrap();
        assert!(
            snapshot.resolve().unwrap().project(&options).is_err(),
            "{root}"
        );
    }
}

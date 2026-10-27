use windows_clang::*;
use windows_metadata::{Type, Value, reader::*};

#[test]
fn member_enum_ownership() {
    helpers::ensure_libclang();
    let imports = ["UsePalette", "UseChannel"]
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
        .collect();
    let input = test_clang::fixture("member_enums");
    let snapshots = capture(&[input], &[Target::X86, Target::X64, Target::Arm64]).unwrap();
    for snapshot in &snapshots {
        assert_eq!(snapshot.enums.len(), 7);
        assert!(
            snapshot
                .enums
                .iter()
                .all(|enumeration| enumeration.owner.is_some())
        );
    }
    let result = projection::project_partial_with_imports(
        PartialCapture {
            snapshots,
            omissions: vec![],
        },
        "Fixture",
        &imports,
    )
    .unwrap();
    assert!(result.omissions.is_empty(), "{:#?}", result.omissions);
    let rdl = &result.output.partitions["member_enums"].rdl;
    let expected = test_clang::input_dir().join("member_enums.rdl");
    if std::env::var_os("UPDATE_CLANG_RDL").is_some() {
        std::fs::write(&expected, rdl).unwrap();
    }
    assert_eq!(
        rdl,
        &std::fs::read_to_string(expected)
            .unwrap()
            .replace("\r\n", "\n")
    );
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..\\..\\..\\..\\target\\member-enums.winmd");
    windows_rdl::reader()
        .input_text(rdl)
        .output(&path)
        .write()
        .unwrap();
    let index = Index::new_for_architecture(vec![File::read(&path).unwrap()], 2);
    let palette = index.expect("Fixture", "Palette");
    assert_eq!(index.nested(palette).count(), 2);
    assert_eq!(
        palette.fields().next().unwrap().ty(),
        Type::value_named("Fixture", "Palette/Channel")
    );
    let color = index.expect("Fixture", "Color");
    assert_eq!(color.fields().count(), 2);
    assert_eq!(
        color.fields().next().unwrap().constant().unwrap().value(),
        Value::I32(-65536)
    );
    let rewritten = path.with_extension("rdl");
    windows_rdl::writer()
        .input(&path)
        .filter("Fixture")
        .output(&rewritten)
        .write()
        .unwrap();
    let rebuilt = path.with_file_name("member-enums-rebuilt.winmd");
    windows_rdl::reader()
        .input(&rewritten)
        .output(&rebuilt)
        .write()
        .unwrap();
    for path in [path, rewritten, rebuilt] {
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn referenced_member_types_and_constants() {
    helpers::ensure_libclang();
    let targets = [Target::X86, Target::X64, Target::Arm64];
    let producer = projection::generate(
        &[test_clang::selected_fixture(
            "member_enums",
            &["Palette", "Color"],
        )],
        "Owner",
    )
    .unwrap();
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..\\..\\..\\..\\target\\member-enum-reference.winmd");
    windows_rdl::reader()
        .input_texts(producer.partitions.values().map(|part| &part.rdl))
        .output(&path)
        .write()
        .unwrap();
    let metadata = Index::read(&path).unwrap();
    let references = projection::References::new(&metadata);
    let options = projection::Options {
        references: Some(&references),
        ..Default::default()
    };
    let mut input = test_clang::selected_fixture("member_enums", &["Consumer"]);
    input.source.push_str("\nstruct Consumer { Palette* palette; Color* color; Palette::Channel channel; Palette::Mode mode; Color::Value value; };\n");
    let snapshots = capture(std::slice::from_ref(&input), &targets).unwrap();
    let output =
        projection::project_with_options(&merge(&snapshots).unwrap(), "Consumer", &options)
            .unwrap();
    let text: String = output
        .partitions
        .values()
        .map(|part| part.rdl.as_str())
        .collect();
    assert!(text.contains("Owner::Palette::Channel"), "{text}");
    assert!(text.contains("Owner::Palette::Mode"), "{text}");
    assert!(text.contains("*mut Owner::Color"), "{text}");
    assert!(text.contains("Owner::Color::Value"), "{text}");
    assert_eq!(output.records, 1);
    for (old, new, reason) in [
        ("Red = 1", "Red = 3", "differs from metadata"),
        (
            "Red = 0xFFFF0000u",
            "Red = 0xFFFF0001u",
            "differs from metadata",
        ),
        (
            "unsigned int value;",
            "public: unsigned int value; private:",
            "field count differs",
        ),
    ] {
        let mut changed = test_clang::selected_fixture("member_enums", &["Consumer"]);
        changed.source = input.source.replace(old, new);
        let snapshots = capture(&[changed], &targets).unwrap();
        let error =
            projection::project_with_options(&merge(&snapshots).unwrap(), "Consumer", &options)
                .unwrap_err();
        assert!(error.to_string().contains(reason), "{error}");
    }
    std::fs::remove_file(path).unwrap();
}
#[test]
fn owner_identity_and_private_enum_conflicts() {
    helpers::ensure_libclang();
    let mut inputs = [
        test_clang::fixture("member_enums"),
        test_clang::fixture("member_enums"),
    ];
    inputs[1].name = "second.h".into();
    let targets = [Target::X86, Target::X64, Target::Arm64];
    assert!(
        capture_partial(&inputs, &targets)
            .unwrap()
            .omissions
            .is_empty()
    );
    inputs[1].source.insert_str(0, "#define HIDDEN_VALUE 100\n");
    for _ in 0..2 {
        let error = capture_partial(&inputs, &targets).unwrap_err().to_string();
        assert!(
            error.contains("enum") && error.contains("differ"),
            "{error}"
        );
        inputs.reverse();
    }
}

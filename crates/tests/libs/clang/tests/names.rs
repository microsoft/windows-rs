use test_clang::selected_fixture;
use windows_clang::*;
use windows_metadata::{Type, reader};

#[test]
fn public_names() {
    helpers::ensure_libclang();
    roundtrip();
    conflicts();
    ownership();
}

fn roundtrip() {
    let mut inputs = [
        selected_fixture("public_names", &["PublicNames", "AReader"]),
        selected_fixture("public_names", &["PublicNames", "AReader"]),
    ];
    for input in &mut inputs {
        input.source = "#include \"public_names.h\"".into();
    }
    inputs[1].name = "second.h".into();
    let output = projection::generate(&inputs, "Fixture").unwrap();
    inputs.reverse();
    inputs[0].name = "renamed.h".into();
    assert_eq!(output, projection::generate(&inputs, "Fixture").unwrap());
    let rdl = &output.partitions["public_names"].rdl;
    let expected = test_clang::input_dir().join("public_names.rdl");
    if std::env::var_os("UPDATE_CLANG_RDL").is_some() {
        std::fs::write(&expected, rdl).unwrap();
    }
    assert_eq!(
        *rdl,
        std::fs::read_to_string(expected)
            .unwrap()
            .replace("\r\n", "\n")
    );
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let winmd = root.join("target\\test-clang-names.winmd");
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_text(rdl)
        .output(&winmd)
        .write()
        .unwrap();
    for bit in [1, 2, 4] {
        let index =
            reader::Index::new_for_architecture(vec![reader::File::read(&winmd).unwrap()], bit);
        assert_eq!(
            index
                .expect("Fixture", "PublicNames")
                .fields()
                .map(|field| field.ty())
                .collect::<Vec<_>>(),
            [
                Type::value_named("Fixture", "Seconds"),
                Type::value_named("Fixture", "Count"),
                Type::PtrConst(Box::new(Type::value_named("Fixture", "Count")), 1),
                Type::value_named("Fixture", "CountPointer"),
                Type::ArrayFixed(Box::new(Type::value_named("Fixture", "CountAlias")), 2),
                Type::class_named("Fixture", "ZReader"),
                Type::class_named("Fixture", "ReaderAlias"),
                Type::PtrMut(Box::new(Type::value_named("Fixture", "Nothing")), 1),
            ]
        );
        let signature = index
            .expect("Fixture", "ZReader")
            .methods()
            .find(|method| method.name() == "Invoke")
            .unwrap()
            .signature(&[]);
        assert_eq!(signature.return_type, Type::value_named("Fixture", "Count"));
        assert_eq!(
            signature.types,
            [Type::value_named("Fixture", "CountAlias")]
        );
    }
    std::fs::remove_file(winmd).unwrap();
}

fn conflicts() {
    for root in ["PublicNames", "Reader"] {
        let mut inputs = [
            selected_fixture("public_names_conflict", &[root]),
            selected_fixture("public_names_conflict", &[root]),
        ];
        inputs[0].source = "#include \"public_names_conflict.h\"".into();
        inputs[1].name = "other.h".into();
        inputs[1].source = "#define OTHER_NAME\n#include \"public_names_conflict.h\"".into();
        let snapshots = capture(&inputs, &[Target::X86, Target::X64, Target::Arm64]).unwrap();
        let merged = merge(&snapshots).unwrap();
        let error = projection::project(&merged, "Fixture")
            .unwrap_err()
            .to_string();
        assert!(
            error.contains(&format!(
                "conflicting public names or contracts for `{root}`"
            )),
            "{error}"
        );
        inputs.reverse();
        assert!(
            projection::generate_partial(&inputs, "Fixture")
                .unwrap_err()
                .to_string()
                .contains("conflicting public names")
        );
    }
}

fn ownership() {
    let mut inputs = [
        selected_fixture("public_names_ownership", &[]),
        selected_fixture("public_names_ownership", &[]),
    ];
    for input in &mut inputs {
        input.source = "typedef unsigned Count;\n#include \"public_names_ownership.h\"".into();
    }
    inputs[1].name = "other.h".into();
    let output = projection::generate(&inputs, "Fixture").unwrap();
    let result = projection::generate_partial(&inputs, "Fixture").unwrap();
    assert_eq!(output, result.output);
    assert_eq!((result.output.records, result.output.aliases), (2, 1));
    assert!(result.omissions.is_empty());
    assert!(
        result.output.partitions["shared"]
            .rdl
            .contains("type Count = u32;")
    );
    assert!(
        result.output.partitions["public_names_ownership"]
            .rdl
            .contains("struct NamedSurvivor")
    );
    assert!(
        result.output.partitions["public_names_ownership"]
            .rdl
            .contains("struct NamedDependent")
    );
}

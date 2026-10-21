use test_clang2::{input_dir, selected_fixture};
use windows_clang2::*;

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

fn imports() -> projection::Imports {
    [(
        "Convert".into(),
        projection::Import {
            library: "fixture.dll".into(),
            name: None,
        },
    )]
    .into_iter()
    .collect()
}

fn inputs() -> [Input; 2] {
    std::array::from_fn(|variant| {
        let mut input = selected_fixture("import_typedefs", &["Convert"]);
        input.name = format!("variant_{variant}.h");
        input.source = format!("#define VARIANT {variant}\n#include \"import_typedefs.h\"\n");
        input
    })
}

#[test]
fn equivalent_import_typedefs() {
    helpers::ensure_libclang();
    let mut inputs = inputs();
    let mut targets = TARGETS;
    let mut expected = None;
    for _ in 0..2 {
        let snapshots = capture(&inputs, &targets).unwrap();
        let output =
            projection::project_with_imports(&merge(&snapshots).unwrap(), "Fixture", &imports())
                .unwrap();
        let single = projection::project_with_imports(
            &merge(&capture(&inputs[..1], &targets).unwrap()).unwrap(),
            "Fixture",
            &imports(),
        )
        .unwrap();
        if expected.is_none() {
            // The named, typedef-preserving signature is an actual input declaration.
            assert_eq!(output, single);
        }
        if let Some(expected) = &expected {
            assert_eq!(&output, expected);
        }
        expected = Some(output);
        inputs.reverse();
        targets.reverse();
    }
    let output = expected.unwrap();
    assert_eq!(output.functions, 1);
    let text = &output.partitions["import_typedefs"].rdl;
    let golden = input_dir().join("import_typedefs.rdl");
    if std::env::var_os("UPDATE_CLANG2_RDL").is_some() {
        std::fs::write(&golden, text).unwrap();
    }
    assert_eq!(
        text,
        &std::fs::read_to_string(golden)
            .unwrap()
            .replace("\r\n", "\n")
    );
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let winmd = root.join("target\\test-clang2-import-typedefs.winmd");
    let rdl = winmd.with_extension("rdl");
    let mut source = text.clone();
    for _ in 0..2 {
        windows_rdl::reader()
            .input(root.join("metadata\\metadata.rdl"))
            .input_text(&source)
            .output(&winmd)
            .write()
            .unwrap();
        windows_rdl::writer()
            .input(&winmd)
            .filter("Fixture")
            .output(&rdl)
            .write()
            .unwrap();
        source = std::fs::read_to_string(&rdl).unwrap();
        assert_eq!(&source, text);
    }
    std::fs::remove_file(winmd).unwrap();
    std::fs::remove_file(rdl).unwrap();
}

#[test]
fn equivalent_direction_on_pointer_typedefs() {
    helpers::ensure_libclang();
    for direction in ["_In_", "_Out_", "_Inout_"] {
        let mut inputs = inputs();
        for input in &mut inputs {
            input.source.insert_str(
                0,
                &format!(
                    "#define STORAGE_DIRECTION __attribute__((annotate(\"windows_clang:sal:2:{direction}\\n()\\n()\")))\n"
                ),
            );
        }
        let mut expected = None;
        for _ in 0..2 {
            let snapshots = capture(&inputs, &TARGETS).unwrap();
            let output = projection::project_with_imports(
                &merge(&snapshots).unwrap(),
                "Fixture",
                &imports(),
            )
            .unwrap();
            assert_eq!(output.functions, 1);
            if let Some(expected) = &expected {
                assert_eq!(&output, expected);
            }
            expected = Some(output);
            inputs.reverse();
        }
    }
}

#[test]
fn typedef_annotations_still_disagree() {
    helpers::ensure_libclang();
    for (alias, separate_tu) in [
        ("Buffer", false),
        ("Counts", false),
        ("Total", false),
        ("Count", false),
        ("Count", true),
    ] {
        let mut inputs = Vec::from(inputs());
        if separate_tu {
            // The annotation can arrive from a TU that never uses the import.
            let mut extra = selected_fixture("import_typedefs", &["Count"]);
            extra.source.insert_str(0, "#define VARIANT 1\n");
            inputs.push(extra);
        }
        for _ in 0..2 {
            let mut snapshots = capture(&inputs, &TARGETS).unwrap();
            annotate(&mut snapshots, alias, separate_tu);
            reject_annotations(&snapshots);
            inputs.reverse();
        }
    }
}

fn annotate(snapshots: &mut [Snapshot], name: &str, separate_tu: bool) {
    let mut count = 0;
    for snapshot in snapshots {
        if separate_tu && !snapshot.values.is_empty() {
            continue;
        }
        let Some(alias) = snapshot.aliases.iter_mut().find(|alias| alias.name == name) else {
            continue;
        };
        for observation in &mut alias.observations {
            count += 1;
            observation.attributes.push(Attribute {
                kind: AttributeKind::Sal(Box::new(SalAnnotation {
                    version: "2".into(),
                    name: "_Return_type_success_".into(),
                    arguments: "return >= 0".into(),
                    expanded: "return >= 0".into(),
                })),
                location: None,
            });
        }
    }
    assert!(count > 0);
}

fn reject_annotations(snapshots: &[Snapshot]) {
    let error = projection::project_with_imports(&merge(snapshots).unwrap(), "Fixture", &imports())
        .unwrap_err();
    assert!(
        error
            .to_string()
            .contains("conflicting public names or contracts"),
        "{error}"
    );
}

#[test]
fn callback_names_remain_distinct() {
    helpers::ensure_libclang();
    let mut inputs = inputs();
    inputs[1]
        .source
        .insert_str(0, "#define CALLBACK_TYPE OtherCallback\n");
    for _ in 0..2 {
        reject_annotations(&capture(&inputs, &TARGETS).unwrap());
        inputs.reverse();
    }
}

#[test]
fn native_contracts_still_disagree() {
    helpers::ensure_libclang();
    for (from, to) in [
        ("const unsigned long*", "unsigned long*"),
        ("unsigned long Convert", "unsigned int Convert"),
        ("Record*, Mode", "OtherRecord*, Mode"),
        ("Record*, Mode", "Record*, OtherMode"),
        ("int (*)[2]", "int (*)[3]"),
    ] {
        let mut inputs = inputs();
        inputs[1].source = format!(
            "#define VARIANT 1\nstruct OtherRecord {{ int value; }};\nenum OtherMode {{ Other }};\n{}",
            std::fs::read_to_string(input_dir().join("import_typedefs.h"))
                .unwrap()
                .replace(from, to)
        );
        for _ in 0..2 {
            let snapshots = capture(&inputs, &TARGETS).unwrap();
            let error = merge(&snapshots).err().unwrap();
            assert!(error.to_string().contains("differ"), "{error}");
            assert!(capture_partial(&inputs, &TARGETS).is_err());
            inputs.reverse();
        }
    }
}

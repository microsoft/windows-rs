use test_clang::{input_dir, selected_fixture};
use windows_clang::*;

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

#[test]
fn fixed_pointer_boundaries() {
    helpers::ensure_libclang();
    for case in 0..9 {
        let mut input = selected_fixture("fixed_pointers_invalid", &["Bad", "Dependent", "Safe"]);
        input.arguments.push(format!("-DCASE={case}"));
        let output = projection::generate_partial(&[input], "Fixture").unwrap();
        let rdl = &output.output.partitions["fixed_pointers_invalid"].rdl;
        if case == 3 {
            assert!(rdl.contains("type Bad = ptr64<*mut i32>;"), "{rdl}");
            assert!(rdl.contains("struct Dependent"), "{rdl}");
            assert!(output.omissions.is_empty(), "{:?}", output.omissions);
            continue;
        }
        assert!(!rdl.contains("Bad"), "{case}: {rdl}");
        assert!(!rdl.contains("Dependent"), "{case}: {rdl}");
        assert!(rdl.contains("struct Safe"), "{case}: {rdl}");
        assert!(!output.omissions.is_empty(), "{case}");
    }
}

#[test]
fn fixed_pointer_conflicts() {
    helpers::ensure_libclang();
    for other in ["void*", "unsigned long long", "const void* __ptr64"] {
        let mut inputs = ["void* __ptr64", other].map(|pointer| {
            let mut input = selected_fixture("fixed_pointers_conflict", &["Conflict"]);
            input
                .source
                .insert_str(0, &format!("#define POINTER {pointer}\n"));
            input
        });
        inputs[1].name = "other.h".into();
        for _ in 0..2 {
            let error = projection::generate_partial(&inputs, "Fixture")
                .unwrap_err()
                .to_string();
            let expected = if other == "void*" {
                "record layout or field count differs"
            } else {
                "native field types differ"
            };
            assert!(error.contains(expected), "{other}: {error}");
            inputs.reverse();
        }
    }
}

#[test]
fn fixed_pointers() {
    helpers::ensure_libclang();
    let snapshots = capture(
        &[selected_fixture(
            "fixed_pointers",
            &["Addresses", "Segment", "PackedAddresses"],
        )],
        &TARGETS,
    )
    .unwrap();
    for snapshot in &snapshots {
        let field = snapshot
            .observations
            .iter()
            .filter_map(|observation| observation.definition.as_ref())
            .flat_map(|definition| &definition.fields)
            .find(|field| field.name == "constant_storage")
            .unwrap();
        let TypeKind::Pointer(inner) = snapshot.types[field.canonical.0].kind else {
            panic!()
        };
        assert!(
            snapshot.types[inner.0].qualifiers.constant,
            "{:?}: {:?}",
            snapshot.target, snapshot.types[inner.0]
        );
    }
    let output = projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap();
    let rdl = &output.partitions["fixed_pointers"].rdl;
    let expected = input_dir().join("fixed_pointers.rdl");
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
    let scratch = root.join("target\\test-clang-fixed-pointers");
    std::fs::create_dir_all(&scratch).unwrap();
    let winmd = scratch.join("input.winmd");
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_text(rdl)
        .output(&winmd)
        .write()
        .unwrap();
    let rewritten = scratch.join("rewritten.rdl");
    windows_rdl::writer()
        .input(&winmd)
        .filter("Fixture")
        .output(&rewritten)
        .write()
        .unwrap();
    assert_eq!(std::fs::read_to_string(rewritten).unwrap(), *rdl);
    std::fs::remove_dir_all(scratch).unwrap();
}

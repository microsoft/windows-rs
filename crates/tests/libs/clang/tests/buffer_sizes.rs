use windows_clang::*;

fn input(name: &str) -> Input {
    let mut input = test_clang::annotated_sdk_fixture();
    input.name = format!("{name}.h");
    input.source = test_clang::fixture(name).source;
    input.roots.clear();
    input
}

#[test]
#[ignore = "requires the pinned SDK"]
fn buffer_capacity_rdl() {
    helpers::ensure_libclang();
    let mut inputs = [input("buffer_sizes"), input("buffer_sizes")];
    inputs[1].name = "second.h".into();
    let mut targets = [Target::X86, Target::X64, Target::Arm64];
    let path = test_clang::input_dir().join("buffer_sizes.rdl");
    for iteration in 0..2 {
        let snapshots = capture(&inputs, &targets).unwrap();
        let output = projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap();
        let actual: String = output
            .partitions
            .values()
            .map(|part| part.rdl.as_str())
            .collect();
        if iteration == 0 && std::env::var_os("UPDATE_CLANG_RDL").is_some() {
            std::fs::write(&path, &actual).unwrap();
        }
        let expected = std::fs::read_to_string(&path)
            .unwrap()
            .replace("\r\n", "\n");
        assert_eq!(actual, expected);
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
        let metadata = root.join("target\\test-clang-buffer-sizes.winmd");
        windows_rdl::reader()
            .input(root.join("metadata\\metadata.rdl"))
            .input_text(&actual)
            .output(&metadata)
            .write()
            .unwrap();
        for arch in [1, 2, 4] {
            use windows_metadata::{BufferAccess, BufferCount, BufferPhase, BufferUnits};
            let index = windows_metadata::reader::Index::new_for_architecture(
                vec![windows_metadata::reader::File::read(&metadata).unwrap()],
                arch,
            );
            for (name, old) in [("QueryBytes", false), ("FullBytes", true)] {
                let method = index.expect("Fixture", name).methods().next().unwrap();
                let param = method.params().next().unwrap();
                let entry =
                    test_clang::buffer_extent(param, BufferAccess::Write, BufferPhase::Entry)
                        .unwrap();
                let success =
                    test_clang::buffer_extent(param, BufferAccess::Read, BufferPhase::Success)
                        .unwrap();
                assert_eq!(entry.count, BufferCount::Dereference(1));
                assert_eq!(success.count, entry.count);
                assert_eq!(entry.units, BufferUnits::Bytes);
                assert_eq!(success.units, entry.units);
                assert_eq!(success.count_on_entry, old);
            }
        }
        inputs.reverse();
        targets.reverse();
    }
}

#[test]
#[ignore = "requires the pinned SDK"]
fn invalid_buffer_capacity() {
    helpers::ensure_libclang();
    for (case, message) in [
        (1, "conflicting SAL buffer capacity"),
        (2, "requires an integer"),
        (3, "requires a buffer pointer"),
        (4, "conflicting SAL buffer capacity"),
        (6, "requires a buffer pointer"),
    ] {
        let mut input = input("buffer_sizes_invalid");
        input
            .source
            .insert_str(0, &format!("#define CASE {case}\n"));
        let error = projection::generate(std::slice::from_ref(&input), "Fixture").unwrap_err();
        assert!(error.to_string().contains(message), "{error}");
        let partial = projection::generate_partial(&[input], "Fixture").unwrap();
        assert_eq!(partial.output.aliases, 1);
        assert!(
            partial
                .omissions
                .iter()
                .any(|value| value.contains(message)),
            "{:?}",
            partial.omissions
        );
        let actual: String = partial
            .output
            .partitions
            .values()
            .map(|part| part.rdl.as_str())
            .collect();
        assert!(
            actual.contains("#[writable(entry, bytes(capacity))] buffer:"),
            "{actual}"
        );
        assert!(!actual.contains("fn Invalid("), "{actual}");
    }
}

#[test]
#[ignore = "requires the pinned SDK"]
fn noninteger_count_expression_retains_evidence() {
    helpers::ensure_libclang();
    let mut input = input("buffer_sizes_invalid");
    input.source.insert_str(0, "#define CASE 5\n");
    let captured = capture_partial(&[input], &[Target::X86, Target::X64, Target::Arm64]).unwrap();
    assert!(captured.omissions.is_empty());
    let resolved = merge(&captured.snapshots).unwrap();
    let output = projection::project(&resolved, "Fixture").unwrap();
    let actual: String = output
        .partitions
        .values()
        .map(|part| part.rdl.as_str())
        .collect();
    let path = test_clang::input_dir().join("buffer_sizes_noninteger.rdl");
    if std::env::var_os("UPDATE_CLANG_RDL").is_some() {
        std::fs::write(&path, &actual).unwrap();
    }
    assert_eq!(
        actual,
        std::fs::read_to_string(path).unwrap().replace("\r\n", "\n")
    );
    let partial = projection::project_partial(captured, "Fixture").unwrap();
    assert!(partial.omissions.is_empty());
    assert_eq!(partial.output.aliases, 2);
    assert_eq!(partial.output.partitions, output.partitions);
}

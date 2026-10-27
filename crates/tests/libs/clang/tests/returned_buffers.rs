use windows_clang::*;
use windows_metadata::{BufferAccess, BufferCount, BufferPhase, HasAttributes, reader};

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
fn pointee_extents() {
    helpers::ensure_libclang();
    let input = test_clang::fixture("pointee_extents");
    let output = projection::generate(std::slice::from_ref(&input), "Fixture").unwrap();
    roundtrip(&output, "pointee_extents", |index| {
        for name in ["QueryExtent", "ByteExtent", "DataExtent", "Conditional"] {
            let method = index.expect("Fixture", name).methods().next().unwrap();
            let parameter = method.params().last().unwrap();
            let extent = windows_metadata::BufferExtent::from_values(
                &parameter
                    .find_attribute("BufferExtentAttribute")
                    .unwrap()
                    .value(),
            )
            .unwrap();
            assert_eq!(extent.count, BufferCount::Unknown);
            assert_eq!(extent.phase, BufferPhase::Success);
            assert_eq!(extent.access, BufferAccess::Read);
            assert!(extent.indirect);
            assert_eq!(
                extent.units,
                if name == "ByteExtent" {
                    windows_metadata::BufferUnits::Bytes
                } else {
                    windows_metadata::BufferUnits::Elements
                }
            );
            assert_eq!(
                parameter.has_attribute("NativeSalAttribute"),
                name == "Conditional"
            );
        }
        for name in [
            "OtherSubject",
            "Slot",
            "Entry",
            "Mixed",
            "KnownCount",
            "Scoped",
        ] {
            let method = index.expect("Fixture", name).methods().next().unwrap();
            let parameter = method.params().last().unwrap();
            assert!(parameter.has_attribute("NativeSalAttribute"), "{name}");
            assert!(!parameter.has_attribute("BufferExtentAttribute"), "{name}");
        }
    });
    let mut other = test_clang::fixture("pointee_extents");
    other.name = "other.h".into();
    other.source = other.source.replace(
        "(*QueryExtent)(COM_OUT AT(*value, _Post_readable_size_(_Inexpressible_(varies)))",
        "(*QueryExtent)(COM_OUT AT(*value, _Post_readable_size_(_Inexpressible_(different)))",
    );
    let error = projection::generate_partial(&[input, other], "Fixture").unwrap_err();
    assert!(error.to_string().contains("differ"), "{error}");
}

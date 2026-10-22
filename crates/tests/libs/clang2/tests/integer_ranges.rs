use windows_clang2::*;
use windows_metadata::{BufferExtent, HasAttributes, IntegerRange, reader};

#[test]
#[ignore = "requires the pinned SDK"]
fn integer_ranges() {
    helpers::ensure_libclang();
    let mut input = test_clang2::annotated_sdk_fixture();
    input.name = "integer_ranges.h".into();
    input.source = test_clang2::fixture("integer_ranges").source;
    input.roots.clear();
    let snapshots = capture(&[input], &[Target::X86, Target::X64, Target::Arm64]).unwrap();
    let imports = [(
        "ReturnCount".into(),
        projection::Import {
            library: "fixture.dll".into(),
            name: None,
        },
    )]
    .into_iter()
    .collect();
    let output =
        projection::project_with_imports(&merge(&snapshots).unwrap(), "Fixture", &imports).unwrap();
    let text: String = output
        .partitions
        .values()
        .map(|part| part.rdl.as_str())
        .collect();
    let expected = test_clang2::input_dir().join("integer_ranges.rdl");
    if std::env::var_os("UPDATE_CLANG2_RDL").is_some() {
        std::fs::write(&expected, &text).unwrap();
    }
    assert_eq!(
        text,
        std::fs::read_to_string(expected)
            .unwrap()
            .replace("\r\n", "\n")
    );
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let first = root.join("target\\test-clang2-integer-ranges.winmd");
    let second = first.with_extension("again.winmd");
    let rdl = first.with_extension("rdl");
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
    let mut previous = None;
    for path in [first, second] {
        for architecture in [1, 2, 4] {
            let index = reader::Index::new_for_architecture(
                vec![reader::File::read(&path).unwrap()],
                architecture,
            );
            let mut ranges = Vec::new();
            let mut extents = Vec::new();
            for ty in index.types().filter(|ty| ty.namespace() == "Fixture") {
                for field in ty.fields() {
                    for attr in field
                        .attributes()
                        .filter(|attr| attr.name() == "BufferExtentAttribute")
                    {
                        extents.push((
                            format!("{}.{}", ty.name(), field.name()),
                            0,
                            BufferExtent::from_values(&attr.value()).unwrap(),
                        ));
                    }
                    for attr in field
                        .attributes()
                        .filter(|attr| attr.name() == "IntegerRangeAttribute")
                    {
                        ranges.push((
                            format!("{}.{}", ty.name(), field.name()),
                            0,
                            IntegerRange::from_values(&attr.value()).unwrap(),
                        ));
                    }
                }
                for method in ty.methods() {
                    for parameter in method.params() {
                        for attr in parameter
                            .attributes()
                            .filter(|attr| attr.name() == "BufferExtentAttribute")
                        {
                            extents.push((
                                ty.name().to_string(),
                                parameter.sequence(),
                                BufferExtent::from_values(&attr.value()).unwrap(),
                            ));
                        }
                        for attr in parameter
                            .attributes()
                            .filter(|attr| attr.name() == "IntegerRangeAttribute")
                        {
                            ranges.push((
                                ty.name().to_string(),
                                parameter.sequence(),
                                IntegerRange::from_values(&attr.value()).unwrap(),
                            ));
                        }
                    }
                }
            }
            ranges.sort_by(|left, right| (&left.0, left.1).cmp(&(&right.0, right.1)));
            extents.sort_by(|left, right| {
                (&left.0, left.1, left.2.access as i32, left.2.phase as i32).cmp(&(
                    &right.0,
                    right.1,
                    right.2.access as i32,
                    right.2.phase as i32,
                ))
            });
            assert_eq!(ranges.len(), 18);
            assert_eq!(extents.len(), 13);
            let contracts = (ranges, extents);
            if let Some(previous) = &previous {
                assert_eq!(&contracts, previous);
            }
            previous = Some(contracts);
        }
    }
}

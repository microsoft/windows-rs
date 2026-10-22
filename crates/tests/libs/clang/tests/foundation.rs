use test_clang::{input_dir, selected_fixture};
use windows_clang::*;
use windows_metadata::{Type, reader};

#[test]
fn foundation() {
    helpers::ensure_libclang();
    let mut inputs = [
        selected_fixture("foundation_a", &[]),
        selected_fixture("foundation_b", &[]),
        selected_fixture("foundation_consumer", &[]),
    ];
    for input in &mut inputs {
        input.source = format!("#include \"{}\"", input.name);
    }
    let output = projection::generate(&inputs, "Fixture").unwrap();
    assert_eq!((output.records, output.enums, output.aliases), (2, 1, 3));
    assert_eq!(output.partitions.len(), 2);
    for stem in ["foundation", "foundation_consumer"] {
        assert_eq!(
            output.partitions[stem].rdl,
            std::fs::read_to_string(input_dir().join(format!("{stem}.rdl")))
                .unwrap()
                .replace("\r\n", "\n")
        );
    }
    let owners: Vec<_> = output.partitions["foundation"]
        .headers
        .iter()
        .map(|header| {
            std::path::Path::new(header)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
        })
        .collect();
    assert_eq!(owners, ["foundation_a.h", "foundation_b.h"]);
    assert_eq!(output.partitions["foundation_consumer"].headers.len(), 1);
    assert_eq!(
        projection::generate_partial(&inputs, "Fixture")
            .unwrap()
            .output,
        output
    );
    inputs.reverse();
    for (index, input) in inputs.iter_mut().enumerate() {
        input.name = format!("renamed_{index}.h");
    }
    assert_eq!(projection::generate(&inputs, "Fixture").unwrap(), output);

    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let winmd = root.join("target\\test-clang-foundation.winmd");
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_texts(output.partitions.values().map(|partition| &partition.rdl))
        .output(&winmd)
        .write()
        .unwrap();
    for arch in [1, 2, 4] {
        let index =
            reader::Index::new_for_architecture(vec![reader::File::read(&winmd).unwrap()], arch);
        for alias in ["Count", "PlatformCount"] {
            assert_eq!(
                index.expect("Fixture", alias).underlying_type(),
                Some(Type::U32)
            );
        }
        assert_eq!(
            index
                .expect("Fixture", "SharedRecord")
                .fields()
                .next()
                .unwrap()
                .ty(),
            Type::value_named("Fixture", "Count")
        );
    }
    let bindings = root.join("target\\test-clang-foundation.rs");
    windows_bindgen::builder()
        .input(&winmd)
        .filter("Fixture")
        .sys()
        .flat()
        .output(&bindings)
        .write();
    let mut rust = std::fs::read_to_string(&bindings).unwrap();
    rust.push_str(
        "\nfn aliases(value: Count) -> PlatformCount { value }\n\
         const _: () = {\n\
         assert!(core::mem::size_of::<SharedRecord>() == 4);\n\
         assert!(core::mem::offset_of!(Consumer, state) == 4);\n\
         assert!(core::mem::offset_of!(Consumer, callback) == 8);\n\
         };\n",
    );
    std::fs::write(&bindings, rust).unwrap();
    let compiled = root.join("target\\test-clang-foundation.rmeta");
    let result = std::process::Command::new("rustc")
        .args([
            "--crate-name",
            "test_clang_foundation",
            "--crate-type=lib",
            "--emit=metadata",
            "--edition=2024",
        ])
        .arg(&bindings)
        .arg("-o")
        .arg(&compiled)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    for path in [winmd, bindings, compiled] {
        std::fs::remove_file(path).unwrap();
    }

    let mut conflict = selected_fixture("foundation_b", &[]);
    conflict.source = "#define CONFLICT\n#include \"foundation_b.h\"".into();
    let error = projection::generate_partial(
        &[selected_fixture("foundation_a", &[]), conflict],
        "Fixture",
    )
    .unwrap_err()
    .to_string();
    assert!(error.contains("native field types differ"), "{error}");
    for names in [("a\\same.h", "b\\same.h"), ("foundation.h", "other.h")] {
        let inputs = [
            Input {
                name: names.0.into(),
                source: "struct First { int value; };".into(),
                ..Default::default()
            },
            Input {
                name: names.1.into(),
                source: "struct Second { int value; };".into(),
                ..Default::default()
            },
        ];
        let error = projection::generate(&inputs, "Fixture")
            .unwrap_err()
            .to_string();
        assert!(error.contains("collide"), "{error}");
    }
}

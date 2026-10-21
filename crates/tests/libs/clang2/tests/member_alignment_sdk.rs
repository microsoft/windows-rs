use windows_clang2::*;
use windows_metadata::{HasAttributes, Value, reader};

#[test]
#[ignore = "requires the pinned SDK and all three MSVC Rust targets"]
fn sdk_member_alignment_layouts() {
    member_alignment(true);
}

#[test]
#[ignore = "requires the pinned SDK and clang resource headers"]
fn sdk_member_alignment_metadata() {
    member_alignment(false);
}

fn member_alignment(consumers: bool) {
    helpers::ensure_libclang();
    let names = [
        ("CLS_SCAN_CONTEXT", 7),
        ("WHV_X64_VMX_NESTED_STATE", 6),
        ("WHV_X64_SVM_NESTED_STATE", 6),
        ("WHV_X64_NESTED_STATE", 6),
    ];
    let targets = [Target::X86, Target::X64, Target::Arm64];
    let fixture = test_clang2::fixture("sdk_member_alignment");
    let mut input = test_clang2::annotated_sdk_fixture();
    input.name = fixture.name;
    input.source = fixture.source;
    let mut snapshots = Vec::new();
    for (target, bit) in targets.into_iter().zip([1, 2, 4]) {
        input.roots = names
            .iter()
            .filter(|(_, mask)| mask & bit != 0)
            .map(|(name, _)| (*name).into())
            .collect();
        snapshots.extend(capture(std::slice::from_ref(&input), &[target]).unwrap());
    }
    let output = projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap();
    let rdl: String = output
        .partitions
        .values()
        .map(|part| part.rdl.as_str())
        .collect();
    let expected = test_clang2::input_dir().join("sdk_member_alignment.rdl");
    if std::env::var_os("UPDATE_CLANG2_RDL").is_some() {
        std::fs::write(&expected, &rdl).unwrap();
    }
    assert_eq!(
        rdl,
        std::fs::read_to_string(expected)
            .unwrap()
            .replace("\r\n", "\n")
    );
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let winmd = root.join(format!(
        "target\\test-clang2-sdk-member-alignment-{consumers}.winmd"
    ));
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_text(&rdl)
        .output(&winmd)
        .write()
        .unwrap();
    let bindings = root.join("target\\test-clang2-sdk-member-alignment.rs");
    let mut rust = String::new();
    if consumers {
        windows_bindgen::builder()
            .input(&winmd)
            .filter("Fixture")
            .flat()
            .sys()
            .output(&bindings)
            .write();
        rust = std::fs::read_to_string(&bindings).unwrap();
    }
    for snapshot in &snapshots {
        let (bit, arch) = match snapshot.target {
            Target::X86 => (1, "x86"),
            Target::X64 => (2, "x86_64"),
            Target::Arm64 => (4, "aarch64"),
        };
        let index =
            reader::Index::new_for_architecture(vec![reader::File::read(&winmd).unwrap()], bit);
        rust.push_str(&format!(
            "\n#[cfg(target_arch = \"{arch}\")] const _: () = {{\n"
        ));
        for (name, mask) in names {
            let alias = snapshot.aliases.iter().find(|alias| alias.name == name);
            assert_eq!(alias.is_some(), mask & bit != 0, "{name} {arch}");
            let Some(alias) = alias else { continue };
            let TypeKind::Record(id) = snapshot.types[alias.observations[0].canonical.0].kind
            else {
                panic!()
            };
            let record = &snapshot.records[id.0];
            let definition = record
                .observations
                .iter()
                .find_map(|id| snapshot.observations[id.0].definition.as_ref())
                .unwrap();
            rust.push_str(&format!("assert!(core::mem::size_of::<{name}>() == {});\nassert!(core::mem::align_of::<{name}>() == {});\n",
                definition.layout.size, definition.layout.alignment));
            let metadata = index.expect("Fixture", name);
            for field in &definition.fields {
                rust.push_str(&format!(
                    "assert!(core::mem::offset_of!({name}, {}) == {});\n",
                    field.name,
                    field.offset_bits / 8
                ));
                let alignment = field
                    .attributes
                    .iter()
                    .filter_map(|attribute| {
                        if let AttributeKind::Aligned { bits, .. } = attribute.kind {
                            Some(bits / 8)
                        } else {
                            None
                        }
                    })
                    .filter(|bytes| *bytes != 0)
                    .max();
                let published = metadata
                    .fields()
                    .find(|item| item.name() == field.name)
                    .unwrap()
                    .find_attribute("AlignmentAttribute")
                    .map(|attribute| attribute.value());
                assert_eq!(
                    published,
                    alignment.map(|bytes| vec![(String::new(), Value::I32(bytes as i32))])
                );
            }
        }
        rust.push_str("};\n");
    }
    if !consumers {
        std::fs::remove_file(winmd).unwrap();
        return;
    }
    std::fs::write(&bindings, rust).unwrap();
    let compiled = root.join("target\\test-clang2-sdk-member-alignment.rmeta");
    for target in targets {
        let result = std::process::Command::new("rustc")
            .args([
                "--crate-name=test_sdk_member_alignment",
                "--crate-type=lib",
                "--emit=metadata",
                "--edition=2024",
                "--target",
                target.triple(),
            ])
            .arg(&bindings)
            .arg("-o")
            .arg(&compiled)
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "{target:?}: {}",
            String::from_utf8_lossy(&result.stderr)
        );
    }
    for path in [winmd, bindings, compiled] {
        std::fs::remove_file(path).unwrap();
    }
}

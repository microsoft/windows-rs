use test_clang::{input_dir, selected_fixture};
use windows_clang::*;
use windows_metadata::{HasAttributes, Value, reader};

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

#[test]
fn bitfields() {
    helpers::ensure_libclang();
    roundtrip("bitfields", "bitfields");
    conflicts();
    rejections();
}

#[test]
fn typed_bitfields() {
    helpers::ensure_libclang();
    roundtrip("typed_bitfields", "typed-bitfields");
    for (first, second) in [
        ("unsigned int", "signed int"),
        ("unsigned char", "bool"),
        ("Mode", "unsigned int"),
    ] {
        let mut inputs =
            [first, second].map(|ty| {
                let mut input = selected_fixture("bitfields_conflict", &["PublicBits"]);
                input.source.insert_str(0, &format!(
                "enum Mode : unsigned int {{ Off = 0, On = 1 }};\n#define MEMBER_TYPE {ty}\n"
            ));
                input
            });
        inputs[1].name = "other.h".into();
        for _ in 0..2 {
            let error = projection::generate_partial(&inputs, "Fixture")
                .unwrap_err()
                .to_string();
            assert!(error.contains("native field types differ"), "{error}");
            inputs.reverse();
        }
    }
}
fn roundtrip(fixture: &str, name: &str) {
    let snapshots = capture(&[selected_fixture(fixture, &[])], &TARGETS).unwrap();
    let output = projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap();
    assert_eq!(output.records, if fixture == "bitfields" { 16 } else { 7 });
    let rdl = &output.partitions[fixture].rdl;
    if std::env::var_os("UPDATE_CLANG_RDL").is_some() {
        std::fs::write(input_dir().join(format!("{fixture}.rdl")), rdl).unwrap();
    }
    assert_eq!(
        *rdl,
        std::fs::read_to_string(input_dir().join(format!("{fixture}.rdl")))
            .unwrap()
            .replace("\r\n", "\n")
    );
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let winmd = root.join(format!("target\\test-clang-{name}.winmd"));
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_text(rdl)
        .output(&winmd)
        .write()
        .unwrap();
    for (snapshot, bit) in snapshots.iter().zip([1, 2, 4]) {
        let index =
            reader::Index::new_for_architecture(vec![reader::File::read(&winmd).unwrap()], bit);
        for record in &snapshot.records {
            let definition = snapshot.observations[record.observations[0].0]
                .definition
                .as_ref()
                .unwrap();
            let projected = index.expect("Fixture", &record.name);
            assert_eq!(
                projected
                    .flags()
                    .contains(windows_metadata::TypeAttributes::ExplicitLayout),
                record.kind == RecordKind::Union,
            );
            let members: Vec<_> = projected
                .fields()
                .flat_map(|field| {
                    field
                        .attributes()
                        .filter(|attr| attr.name() == "NativeBitfieldAttribute")
                        .map(move |attr| (field.name(), attr.value()))
                })
                .collect();
            let named: Vec<_> = definition
                .fields
                .iter()
                .filter(|field| field.bit_width.is_some() && !field.name.is_empty())
                .collect();
            assert_eq!(members.len(), named.len(), "{}", record.name);
            for field in named {
                let (_, member) = members
                    .iter()
                    .find(|(_, member)| member[0].1 == Value::Utf8(field.name.clone()))
                    .unwrap();
                assert_eq!(member[2].1, Value::I64(i64::from(field.bit_width.unwrap())));
                assert!(matches!(member[1].1, Value::I64(_)), "expected bit offset");
            }
        }
        if fixture == "bitfields" {
            let unsigned = index.expect("Fixture", "Unsigned");
            let attrs: Vec<_> = unsigned
                .fields()
                .next()
                .unwrap()
                .attributes()
                .filter(|attr| attr.name() == "NativeBitfieldAttribute")
                .collect();
            assert_eq!(attrs[1].value()[1].1, Value::I64(5));
        }
    }
    let rewritten = root.join(format!("target\\test-clang-{name}.rdl"));
    windows_rdl::writer()
        .input(&winmd)
        .filter("Fixture")
        .output(&rewritten)
        .write()
        .unwrap();
    assert_eq!(*rdl, std::fs::read_to_string(&rewritten).unwrap());
    std::fs::remove_file(winmd).unwrap();
    std::fs::remove_file(rewritten).unwrap();
}

fn conflicts() {
    for tag in ["Bits", "Other"] {
        let mut inputs = [
            selected_fixture("bitfields_conflict", &["PublicBits"]),
            selected_fixture("bitfields_conflict", &["PublicBits"]),
        ];
        inputs[0].source = "#include \"bitfields_conflict.h\"".into();
        inputs[1].name = "other.h".into();
        inputs[1].source =
            format!("#define WIDTH 4\n#define TAG {tag}\n#include \"bitfields_conflict.h\"");
        let snapshots = capture(&inputs, &TARGETS).unwrap();
        for pair in snapshots.as_chunks::<2>().0 {
            assert_eq!(
                pair[0].observations[0].definition.as_ref().unwrap().layout,
                pair[1].observations[0].definition.as_ref().unwrap().layout
            );
        }
        let error = projection::generate_partial(&inputs, "Fixture")
            .unwrap_err()
            .to_string();
        assert!(
            error.contains(if tag == "Bits" {
                "field name, order, offset, bit width, or attributes differ"
            } else {
                "incompatible native definitions"
            }),
            "{error}"
        );
    }
}

fn rejections() {
    let output =
        projection::generate_partial(&[selected_fixture("bitfields_rejections", &[])], "Fixture")
            .unwrap();
    assert_eq!(output.output.records, 5);
    for name in ["Collision", "AlignedField", "Underaligned"] {
        assert!(
            output
                .omissions
                .iter()
                .any(|message| message.contains(name)),
            "{name}: {:?}",
            output.omissions
        );
    }
    let mut input = selected_fixture("bitfields_rejections", &[]);
    input.source.insert_str(0, "#define TOO_WIDE\n");
    assert!(
        projection::generate_partial(&[input], "Fixture")
            .unwrap_err()
            .to_string()
            .contains("libclang parse failed")
    );
}

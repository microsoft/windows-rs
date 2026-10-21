use test_clang2::{input_dir, selected_fixture};
use windows_clang2::*;
use windows_metadata::{HasAttributes, Value, reader};

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

#[test]
#[ignore = "requires the pinned SDK, clang resource headers, and three MSVC Rust targets"]
fn sdk_working_set() {
    working_set(true);
}

#[test]
#[ignore = "requires the pinned SDK and clang resource headers"]
fn sdk_working_set_metadata() {
    working_set(false);
}

fn working_set(consumers: bool) {
    helpers::ensure_libclang();
    let mut input = test_clang2::annotated_sdk_fixture();
    let fixture = test_clang2::fixture("sdk_psapi");
    input.name = fixture.name;
    input.source = fixture.source;
    input.roots = vec!["PSAPI_WORKING_SET_BLOCK".into()];
    let snapshots = capture(&[input], &TARGETS).unwrap();
    for snapshot in &snapshots {
        let fields: Vec<_> = snapshot
            .observations
            .iter()
            .filter_map(|observation| observation.definition.as_ref())
            .flat_map(|definition| &definition.fields)
            .filter(|field| field.name == "VirtualPage")
            .collect();
        assert_eq!(fields.len(), 1);
        assert_eq!(fields[0].offset_bits, 12);
        assert_eq!(
            fields[0].bit_width,
            Some(if snapshot.target == Target::X86 {
                20
            } else {
                52
            })
        );
    }
    let output = projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap();
    let actual: String = output
        .partitions
        .values()
        .map(|part| part.rdl.as_str())
        .collect();
    let expected = input_dir().join("sdk_working_set.rdl");
    if std::env::var_os("UPDATE_CLANG2_RDL").is_some() {
        std::fs::write(&expected, &actual).unwrap();
    }
    assert_eq!(
        actual,
        std::fs::read_to_string(expected)
            .unwrap()
            .replace("\r\n", "\n")
    );
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let scratch = root.join(format!("target\\test-clang2-sdk-working-set-{consumers}"));
    std::fs::create_dir_all(&scratch).unwrap();
    let winmd = scratch.join("input.winmd");
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_text(&actual)
        .output(&winmd)
        .write()
        .unwrap();
    if !consumers {
        std::fs::remove_file(winmd).unwrap();
        return;
    }
    let bindings = scratch.join("bindings.rs");
    windows_bindgen::builder()
        .input(&winmd)
        .filter("Fixture.PSAPI_WORKING_SET_BLOCK")
        .flat()
        .output(&bindings)
        .write();
    let mut rust = std::fs::read_to_string(&bindings).unwrap();
    rust.push_str(include_str!("../input/sdk_working_set.rs"));
    std::fs::write(&bindings, rust).unwrap();
    for target in TARGETS {
        compile(&bindings, &scratch.join("output.rmeta"), Some(target));
    }
    for target in [Target::X86, Target::X64] {
        let executable = scratch.join(format!("{target:?}.exe"));
        let result = std::process::Command::new("rustc")
            .args(["--edition=2024", "--target", target.triple()])
            .arg(&bindings)
            .arg("-o")
            .arg(&executable)
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert!(
            std::process::Command::new(executable)
                .status()
                .unwrap()
                .success()
        );
    }
}

#[test]
fn bitfields() {
    helpers::ensure_libclang();
    roundtrip("bitfields", "bitfields", &[None]);
    conflicts();
    rejections();
}

#[test]
#[ignore = "requires the x86, x64, and ARM64 MSVC Rust targets"]
fn bitfields_cross_targets() {
    helpers::ensure_libclang();
    roundtrip("bitfields", "bitfields-cross-targets", &TARGETS.map(Some));
    roundtrip(
        "typed_bitfields",
        "typed-bitfields-cross-targets",
        &TARGETS.map(Some),
    );
}

#[test]
fn typed_bitfields() {
    helpers::ensure_libclang();
    roundtrip("typed_bitfields", "typed-bitfields", &[None]);
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

#[test]
fn bitfield_metadata() {
    helpers::ensure_libclang();
    roundtrip("bitfields", "bitfield-metadata", &[]);
    roundtrip("typed_bitfields", "typed-bitfield-metadata", &[]);
}

fn roundtrip(fixture: &str, name: &str, targets: &[Option<Target>]) {
    let snapshots = capture(&[selected_fixture(fixture, &[])], &TARGETS).unwrap();
    let output = projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap();
    assert_eq!(output.records, if fixture == "bitfields" { 16 } else { 7 });
    let rdl = &output.partitions[fixture].rdl;
    if std::env::var_os("UPDATE_CLANG2_RDL").is_some() {
        std::fs::write(input_dir().join(format!("{fixture}.rdl")), rdl).unwrap();
    }
    assert_eq!(
        *rdl,
        std::fs::read_to_string(input_dir().join(format!("{fixture}.rdl")))
            .unwrap()
            .replace("\r\n", "\n")
    );
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let winmd = root.join(format!("target\\test-clang2-{name}.winmd"));
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_text(rdl)
        .output(&winmd)
        .write()
        .unwrap();
    let mut bitfield_checks = String::new();
    for (snapshot, bit) in snapshots.iter().zip([1, 2, 4]) {
        let target = match snapshot.target {
            Target::X86 => "x86",
            Target::X64 => "x86_64",
            Target::Arm64 => "aarch64",
        };
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
                let (backing, member) = members
                    .iter()
                    .find(|(_, member)| member[0].1 == Value::Utf8(field.name.clone()))
                    .unwrap();
                assert_eq!(member[2].1, Value::I64(i64::from(field.bit_width.unwrap())));
                let Value::I64(offset) = member[1].1 else {
                    panic!("expected bit offset")
                };
                bitfield_checks.push_str(&format!(
                    "#[cfg(target_arch = \"{target}\")] const _: () = assert!(core::mem::offset_of!({}, {backing}) * 8 + {offset} == {});\n",
                    record.name, field.offset_bits));
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
    let rewritten = root.join(format!("target\\test-clang2-{name}.rdl"));
    windows_rdl::writer()
        .input(&winmd)
        .filter("Fixture")
        .output(&rewritten)
        .write()
        .unwrap();
    assert_eq!(*rdl, std::fs::read_to_string(&rewritten).unwrap());
    if targets.is_empty() {
        std::fs::remove_file(winmd).unwrap();
        std::fs::remove_file(rewritten).unwrap();
        return;
    }
    let bindings = root.join(format!("target\\test-clang2-{name}.rs"));
    windows_bindgen::builder()
        .input(&winmd)
        .filter("Fixture")
        .sys()
        .flat()
        .output(&bindings)
        .write();
    let mut rust = std::fs::read_to_string(&bindings).unwrap();
    rust.push_str(&bitfield_checks);
    for snapshot in &snapshots {
        let target = match snapshot.target {
            Target::X86 => "x86",
            Target::X64 => "x86_64",
            Target::Arm64 => "aarch64",
        };
        rust.push_str(&format!(
            "\n#[cfg(target_arch = \"{target}\")] const _: () = {{\n"
        ));
        for record in &snapshot.records {
            let definition = snapshot.observations[record.observations[0].0]
                .definition
                .as_ref()
                .unwrap();
            rust.push_str(&format!(
                "assert!(core::mem::size_of::<{}>() == {});\nassert!(core::mem::align_of::<{}>() == {});\n",
                record.name, definition.layout.size, record.name, definition.layout.alignment));
            for field in definition
                .fields
                .iter()
                .filter(|field| field.bit_width.is_none())
            {
                rust.push_str(&format!(
                    "assert!(core::mem::offset_of!({}, {}) == {});\n",
                    record.name,
                    field.name,
                    field.offset_bits / 8
                ));
            }
        }
        rust.push_str("};\n");
    }
    let accessor_checks = if fixture == "typed_bitfields" {
        include_str!("../input/typed_bitfields.rs")
    } else {
        r#"
fn main() {
    let mut unsigned = Unsigned { _bitfield: 0, tail: 0 };
    unsigned.set_low(7);
    unsigned.set_high(31);
    assert_eq!(unsigned.low(), 7);
    assert_eq!(unsigned.high(), 31);
    assert_eq!(unsigned._bitfield, 7 | (31 << 5));
    unsigned.set_low(0);
    assert_eq!(unsigned.high(), 31);
    let mut signed = Signed { _bitfield: 0 };
    signed.set_low(-3);
    signed.set_high(-7);
    assert_eq!(signed.low(), -3);
    assert_eq!(signed.high(), -7);
    let mut full = Full { _bitfield: 0 };
    full.set_value(u64::MAX);
    assert_eq!(full.value(), u64::MAX);
}
"#
    };
    std::fs::write(&bindings, rust).unwrap();
    let compiled = root.join(format!("target\\test-clang2-{name}.rmeta"));
    for target in targets {
        compile(&bindings, &compiled, *target);
    }
    let mut builder = windows_bindgen::builder();
    builder.input(&winmd).flat().output(&bindings);
    for record in if fixture == "bitfields" {
        &["Unsigned", "Signed", "Full"][..]
    } else {
        &[
            "DependentBits",
            "ReverseBits",
            "FullSignedBits",
            "TypedUnion",
        ][..]
    } {
        builder.filter(&format!("Fixture.{record}"));
    }
    builder.write();
    let mut rust = std::fs::read_to_string(&bindings).unwrap();
    rust.push_str(accessor_checks);
    std::fs::write(&bindings, rust).unwrap();
    for target in targets {
        compile(&bindings, &compiled, *target);
    }
    let executable = root.join(format!("target\\test-clang2-{name}.exe"));
    let result = std::process::Command::new("rustc")
        .args(["--edition=2024", "--crate-name", "test_clang2_bitfields"])
        .arg(&bindings)
        .arg("-o")
        .arg(&executable)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(
        std::process::Command::new(&executable)
            .status()
            .unwrap()
            .success()
    );
    for path in [
        winmd,
        rewritten,
        bindings,
        compiled,
        executable,
        root.join(format!("target\\test-clang2-{name}.pdb")),
    ] {
        std::fs::remove_file(path).unwrap();
    }
}

fn compile(bindings: &std::path::Path, compiled: &std::path::Path, target: Option<Target>) {
    let mut command = std::process::Command::new("rustc");
    command.args([
        "--crate-name",
        "test_clang2_bitfields",
        "--crate-type=lib",
        "--emit=metadata",
        "--edition=2024",
    ]);
    if let Some(target) = target {
        command.args(["--target", target.triple()]);
    }
    let result = command
        .arg(bindings)
        .arg("-o")
        .arg(compiled)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{target:?}: {}",
        String::from_utf8_lossy(&result.stderr)
    );
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

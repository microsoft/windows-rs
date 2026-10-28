use windows_clang::*;
use windows_metadata::reader::HasAttributes;

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

fn golden(name: &str, output: &projection::Output) -> String {
    let text: String = output
        .partitions
        .values()
        .map(|part| part.rdl.as_str())
        .collect();
    let path = test_clang::input_dir().join(format!("{name}.rdl"));
    if std::env::var_os("UPDATE_CLANG_RDL").is_some() {
        std::fs::write(&path, &text).unwrap();
    }
    assert_eq!(
        text,
        std::fs::read_to_string(path).unwrap().replace("\r\n", "\n")
    );
    text
}

#[test]
fn guidless_interfaces() {
    helpers::ensure_libclang();
    let input = test_clang::selected_fixture("guidless_interfaces", &["Holder"]);
    let snapshots = capture(&[input], &TARGETS).unwrap();
    for snapshot in &snapshots {
        for record in &snapshot.records {
            if !["Bare", "Extended", "Inherited"].contains(&record.name.as_str()) {
                continue;
            }
            let observation = &snapshot.observations[record.observations[0].0];
            assert!(
                observation
                    .attributes
                    .iter()
                    .all(|attr| !matches!(attr.kind, AttributeKind::Uuid(_)))
            );
            let definition = observation.definition.as_ref().unwrap();
            let size = if snapshot.target == Target::X86 { 4 } else { 8 };
            assert_eq!(
                definition.layout,
                Layout {
                    size,
                    alignment: size
                }
            );
            let slots: Vec<_> = definition
                .methods
                .iter()
                .map(|method| method.slot)
                .collect();
            let expected = match record.name.as_str() {
                "Bare" => vec![0, 1],
                "Extended" => vec![2],
                _ => vec![],
            };
            assert_eq!(slots, expected);
        }
    }
    let output = projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap();
    assert_eq!(output.interfaces, 3);
    assert_eq!(output.records, 1);
    let text = golden("guidless_interfaces", &output);
    for path in metadata("metadata", &text) {
        for bit in [1, 2, 4] {
            let index = windows_metadata::reader::Index::new_for_architecture(
                vec![windows_metadata::reader::File::read(&path).unwrap()],
                bit,
            );
            for (name, methods) in [("Bare", 2), ("Extended", 1), ("Inherited", 0)] {
                let ty = index.expect("Fixture", name);
                assert!(ty.find_attribute("GuidAttribute").is_none());
                assert_eq!(ty.methods().count(), methods);
            }
        }
    }
}

fn metadata(case: &str, text: &str) -> [std::path::PathBuf; 2] {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let scratch = root.join("target\\test-clang-guidless").join(case);
    std::fs::create_dir_all(&scratch).unwrap();
    let rdl = scratch.join("input.rdl");
    let winmd = scratch.join("input.winmd");
    let rewritten = scratch.join("roundtrip.rdl");
    let again = scratch.join("roundtrip.winmd");
    std::fs::write(&rdl, text).unwrap();
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input(&rdl)
        .output(&winmd)
        .write()
        .unwrap();
    windows_rdl::writer()
        .input(&winmd)
        .filter("Fixture")
        .output(&rewritten)
        .write()
        .unwrap();
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input(&rewritten)
        .output(&again)
        .write()
        .unwrap();
    [winmd, again]
}

#[test]
fn virtual_method_purity() {
    helpers::ensure_libclang();
    let input = |pure: bool| {
        let mut input = test_clang::selected_fixture("virtual_methods", &["Derived"]);
        input.name = format!("pure_{pure}.h");
        input.source = format!(
            "#define PURE {}\n#include \"virtual_methods.h\"\n",
            if pure { "= 0" } else { "" }
        );
        input
    };
    let mut expected = None;
    for pure in [true, false] {
        let snapshots = capture(&[input(pure)], &TARGETS).unwrap();
        for snapshot in &snapshots {
            for record in &snapshot.records {
                let definition = snapshot.observations[record.observations[0].0]
                    .definition
                    .as_ref()
                    .unwrap();
                let methods: Vec<_> = definition
                    .methods
                    .iter()
                    .map(|method| (method.slot, method.overrides, method.pure))
                    .collect();
                assert_eq!(
                    methods,
                    if record.name == "Base" {
                        vec![(0, false, pure), (1, false, pure)]
                    } else {
                        vec![(0, true, pure), (2, false, false)]
                    }
                );
            }
        }
        let output = projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap();
        if let Some(expected) = &expected {
            assert_eq!(&output, expected);
        }
        expected = Some(output);
    }
    let text = golden("virtual_methods", &expected.unwrap());
    for path in metadata("virtual_methods", &text) {
        let index = windows_metadata::reader::Index::read(&path).unwrap();
        assert_eq!(index.expect("Fixture", "Base").methods().count(), 2);
        assert_eq!(index.expect("Fixture", "Derived").methods().count(), 2);
    }
    let mut inputs = [input(true), input(false)];
    for _ in 0..2 {
        let snapshots = capture(&inputs, &TARGETS).unwrap();
        let error = merge(&snapshots).err().unwrap();
        assert!(
            error.to_string().contains("native method purity differs"),
            "{error}"
        );
        assert!(capture_partial(&inputs, &TARGETS).is_err());
        inputs.reverse();
    }
    let input = test_clang::selected_fixture("virtual_methods", &["Defined"]);
    let partial = projection::generate_partial(&[input], "Fixture").unwrap();
    assert_eq!(partial.output.interfaces, 0);
    assert!(
        partial
            .omissions
            .iter()
            .any(|message| { message.contains("virtual method definitions are unsupported") })
    );
}

#[test]
fn unsupported_interface_contracts() {
    helpers::ensure_libclang();
    for (name, message) in [
        ("ConstHolder", "const GUID-less interface"),
        ("WithData", "requires one vptr"),
        ("WrongConvention", "does not use the platform"),
    ] {
        let input = test_clang::selected_fixture("guidless_interfaces", &[name]);
        let snapshots = capture(&[input], &TARGETS).unwrap();
        let error = projection::project(&merge(&snapshots).unwrap(), "Fixture")
            .unwrap_err()
            .to_string();
        assert!(error.contains(message), "{name}: {error}");
    }
}

#[test]
fn inherited_slot_overrides() {
    helpers::ensure_libclang();
    let input = test_clang::selected_fixture("overrides", &["Final", "IdentifiedChild"]);
    let snapshots = capture(&[input], &TARGETS).unwrap();
    for snapshot in &snapshots {
        for record in &snapshot.records {
            let observation = &snapshot.observations[record.observations[0].0];
            let definition = observation.definition.as_ref().unwrap();
            let methods: Vec<_> = definition
                .methods
                .iter()
                .map(|method| (method.slot, method.overrides))
                .collect();
            let expected = match record.name.as_str() {
                "Base" => vec![(0, false), (1, false)],
                "Derived" => vec![(0, true), (1, true), (2, false)],
                "Final" => vec![(0, true), (3, false)],
                "IdentifiedBase" => vec![(0, false)],
                "IdentifiedChild" => vec![(0, true)],
                _ => continue,
            };
            assert_eq!(methods, expected);
        }
    }
    let output = projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap();
    let text = golden("overrides", &output);
    for path in metadata("overrides", &text) {
        for bit in [1, 2, 4] {
            let index = windows_metadata::reader::Index::new_for_architecture(
                vec![windows_metadata::reader::File::read(&path).unwrap()],
                bit,
            );
            for (name, expected) in [
                ("Derived", vec![Some(0), Some(1), None]),
                ("Final", vec![Some(0), None]),
            ] {
                let ty = index.expect("Fixture", name);
                for (method, expected) in ty.methods().zip(expected) {
                    let slot = method
                        .find_attribute("NativeOverrideSlotAttribute")
                        .map(|attr| {
                            let values = attr.value();
                            let [(_, windows_metadata::Value::U32(slot))] = values.as_slice()
                            else {
                                panic!("invalid override slot");
                            };
                            *slot
                        });
                    assert_eq!(slot, expected);
                    assert_eq!(
                        method
                            .flags()
                            .contains(windows_metadata::MethodAttributes::NewSlot),
                        expected.is_none()
                    );
                    if expected.is_some() {
                        assert!(
                            method
                                .find_attribute("NativeExceptionSpecificationAttribute")
                                .is_none()
                        );
                    }
                }
            }
        }
    }
    for (root, message) in [
        ("CovariantChild", "covariant virtual overrides"),
        ("Multiple", "multiple overridden virtual methods"),
    ] {
        let input = test_clang::selected_fixture("overrides", &[root]);
        let error = capture(&[input], &TARGETS).unwrap_err().to_string();
        assert!(error.contains(message), "{error}");
    }
}

#[test]
fn invalid_override_metadata() {
    let text = std::fs::read_to_string(test_clang::input_dir().join("overrides.rdl")).unwrap();
    for (case, source, message) in [
        (
            "override-range",
            text.replacen("override_slot(0)", "override_slot(99)", 1),
            "invalid inherited slot",
        ),
        (
            "override-name",
            text.replacen("override_slot(0)", "override_slot(1)", 1),
            "incompatible inherited slot",
        ),
        (
            "override-pointer",
            text.replacen("renamed: u32", "renamed: *mut u32", 1),
            "incompatible inherited slot",
        ),
    ] {
        assert_ne!(source, text);
        for path in metadata(case, &source) {
            let failure = std::panic::catch_unwind(|| {
                windows_bindgen::builder()
                    .input(&path)
                    .filter("Fixture")
                    .sys()
                    .flat()
                    .output(path.with_extension("rs"))
                    .write();
            })
            .unwrap_err();
            let error = failure.downcast_ref::<String>().unwrap();
            assert!(error.contains(message), "{error}");
        }
    }
}

#[test]
fn conflicting_override_evidence() {
    helpers::ensure_libclang();
    for changed in [0, 1] {
        let first = test_clang::selected_fixture("overrides", &["Final"]);
        let mut second = test_clang::selected_fixture("overrides", &["Final"]);
        second.name = "overrides-again.h".into();
        let mut snapshots = capture(&[first, second], &TARGETS).unwrap();
        let snapshot = snapshots
            .iter_mut()
            .filter(|snapshot| snapshot.target == Target::X86)
            .nth(changed)
            .unwrap();
        let record = snapshot
            .records
            .iter()
            .find(|record| record.name == "Derived")
            .unwrap();
        let observation = record.observations[0];
        snapshot.observations[observation.0]
            .definition
            .as_mut()
            .unwrap()
            .methods[0]
            .overrides = false;
        let error = match merge(&snapshots) {
            Err(error) => error.to_string(),
            Ok(_) => panic!("conflicting override evidence was accepted"),
        };
        assert!(error.contains("method names, slots"), "{error}");
    }
}

#[test]
fn guidless_inheritance() {
    helpers::ensure_libclang();
    let input = test_clang::selected_fixture("guidless_interfaces", &["MissingIdentity"]);
    let snapshots = capture(&[input], &TARGETS).unwrap();
    let output = projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap();
    assert_eq!(output.interfaces, 2);
    let text = golden("guidless_inheritance", &output);
    for path in metadata("inheritance", &text) {
        for bit in [1, 2, 4] {
            let index = windows_metadata::reader::Index::new_for_architecture(
                vec![windows_metadata::reader::File::read(&path).unwrap()],
                bit,
            );
            assert!(
                index
                    .expect("Fixture", "Identified")
                    .find_attribute("GuidAttribute")
                    .is_some()
            );
            let child = index.expect("Fixture", "MissingIdentity");
            assert!(child.find_attribute("GuidAttribute").is_none());
            assert_eq!(child.methods().count(), 1);
        }
    }
}

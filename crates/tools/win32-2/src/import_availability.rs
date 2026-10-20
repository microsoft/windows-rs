use super::*;
use windows_clang2::ValueKind;
use windows_rdl::implib::{ImportKind, ImportName};

#[test]
#[ignore = "requires the pinned SDK headers, import libraries, and native companion"]
fn sdk_import_availability() {
    check_import_availability("sdk_import_availability", 89, 90);
}

#[test]
#[ignore = "requires the pinned SDK headers, import libraries, and native companion"]
fn sdk_ndr_import_availability() {
    check_import_availability("sdk_ndr_import_availability", 4, 3);
}

#[test]
#[ignore = "requires the pinned SDK headers, import libraries, and native companion"]
fn sdk_import_providers() {
    let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\tests\\libs\\clang2\\input");
    let text = std::fs::read_to_string(fixtures.join("sdk_import_providers.txt")).unwrap();
    let expected: Vec<_> = import_rows(&text).collect();
    let input = Input {
        name: "sdk_import_providers.h".into(),
        source: std::fs::read_to_string(fixtures.join("sdk_import_providers.h")).unwrap(),
        arguments: tests::sdk_test_arguments(),
        roots: expected.iter().map(|(_, name, _)| (*name).into()).collect(),
        ..Default::default()
    };
    let snapshots =
        windows_clang2::capture(&[input], &[Target::X86, Target::X64, Target::Arm64]).unwrap();
    windows_clang2::merge(&snapshots).unwrap();
    for snapshot in &snapshots {
        assert_eq!(snapshot.values.len(), 3);
        let (arch, machine) = import_target(snapshot.target);
        let bit = match snapshot.target {
            Target::X86 => 1,
            Target::X64 => 2,
            Target::Arm64 => 4,
        };
        let directory =
            helpers::nuget_package(&format!("microsoft.windows.sdk.cpp.{arch}"), SDK_VERSION)
                .join("c");
        let selected = read_sdk_imports(&directory, snapshot.target, &mut String::new()).unwrap();
        let libraries: BTreeMap<_, _> = ["kernel32", "ntdll"]
            .map(|name| {
                let members = windows_rdl::implib::read(
                    &std::fs::read(directory.join(format!("um\\{arch}\\{name}.lib"))).unwrap(),
                )
                .unwrap();
                (format!("{name}.dll"), members)
            })
            .into();
        for value in &snapshot.values {
            for observation in &value.observations {
                let ValueKind::Function {
                    alias: None,
                    mangling,
                    ..
                } = &observation.kind
                else {
                    panic!("{} is not a native function", value.name);
                };
                let mut provider = None;
                for &(mask, name, library) in &expected {
                    if name != value.name {
                        continue;
                    }
                    let members: Vec<_> = libraries[library]
                        .iter()
                        .filter(|import| {
                            import.machine == machine
                                && import.name == ImportName::Name(value.name.clone())
                        })
                        .collect();
                    if mask & bit != 0 {
                        assert!(!members.is_empty(), "{arch}: {library}!{name}");
                        assert!(
                            members.iter().all(|import| {
                                import.symbol == *mangling
                                    && import.kind == ImportKind::Code
                                    && import.dll.eq_ignore_ascii_case(library)
                            }),
                            "{arch}: {library}!{name}: {members:?}"
                        );
                        provider.get_or_insert(library);
                    } else {
                        assert!(members.is_empty(), "{arch}: {library}!{name}: {members:?}");
                    }
                }
                let import = &selected[mangling];
                assert_eq!(import.name, ImportName::Name(value.name.clone()));
                assert!(import.dll.eq_ignore_ascii_case(provider.unwrap()));
            }
        }
    }
}

fn import_rows(text: &str) -> impl Iterator<Item = (u8, &str, &str)> {
    text.lines()
        .filter(|line| !line.starts_with('#'))
        .map(|line| {
            let fields: Vec<_> = line.split_whitespace().collect();
            let [mask, name, library] = fields.as_slice() else {
                panic!("invalid fixture entry: {line}");
            };
            let mask: u8 = mask.parse().unwrap();
            assert!(matches!(mask, 2 | 6 | 7));
            (mask, *name, *library)
        })
}

fn check_import_availability(fixture: &str, count: usize, missing_count: usize) {
    let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\tests\\libs\\clang2\\input");
    let text = std::fs::read_to_string(fixtures.join(format!("{fixture}.txt"))).unwrap();
    let expected: BTreeMap<_, _> = import_rows(&text)
        .map(|(mask, name, library)| (name.to_string(), (mask, library)))
        .collect();
    assert_eq!(expected.len(), count);
    let input = Input {
        name: format!("{fixture}.h"),
        source: std::fs::read_to_string(fixtures.join(format!("{fixture}.h"))).unwrap(),
        arguments: tests::sdk_test_arguments(),
        roots: expected.keys().cloned().collect(),
        ..Default::default()
    };
    let snapshots =
        windows_clang2::capture(&[input], &[Target::X86, Target::X64, Target::Arm64]).unwrap();
    windows_clang2::merge(&snapshots).unwrap();
    let mut missing = 0;
    for snapshot in &snapshots {
        assert_eq!(snapshot.values.len(), expected.len());
        let (arch, machine) = import_target(snapshot.target);
        let bit = match snapshot.target {
            Target::X86 => 1,
            Target::X64 => 2,
            Target::Arm64 => 4,
        };
        let directory =
            helpers::nuget_package(&format!("microsoft.windows.sdk.cpp.{arch}"), SDK_VERSION)
                .join("c");
        let imports = read_sdk_imports(&directory, snapshot.target, &mut String::new()).unwrap();
        for value in &snapshot.values {
            let (mask, library) = expected[&value.name];
            for observation in &value.observations {
                let ValueKind::Function {
                    alias: None,
                    mangling,
                    stdcall_candidate,
                    ..
                } = &observation.kind
                else {
                    panic!("{} is not a native function", value.name);
                };
                if mask & bit != 0 {
                    let import = imports.get(mangling).unwrap();
                    assert_eq!(import.machine, machine);
                    assert_eq!(import.kind, ImportKind::Code);
                    assert_eq!(import.name, ImportName::Name(value.name.clone()));
                    assert!(
                        import.dll.eq_ignore_ascii_case(library),
                        "{}: {import:?}",
                        value.name
                    );
                } else {
                    assert!(
                        !imports.contains_key(mangling),
                        "{}: {mangling}",
                        value.name
                    );
                    if let Some(candidate) = stdcall_candidate {
                        assert!(!imports.contains_key(&candidate.mangling), "{}", value.name);
                    }
                    assert!(
                        !imports
                            .values()
                            .any(|import| { import.name == ImportName::Name(value.name.clone()) }),
                        "unexpected alternative import for {}",
                        value.name
                    );
                }
            }
            missing += usize::from(mask & bit == 0);
        }
        if snapshot.target == Target::Arm64 && expected.contains_key("RtlCaptureContext2") {
            let members = windows_rdl::implib::read(
                &std::fs::read(directory.join("um\\arm64\\ntdll.lib")).unwrap(),
            )
            .unwrap();
            let context: Vec<_> = members
                .iter()
                .filter(|import| import.name == ImportName::Name("RtlCaptureContext2".into()))
                .collect();
            assert!(!context.is_empty());
            assert!(context.iter().all(|import| import.machine == 0xa641));
        }
    }
    assert_eq!(missing, missing_count);
}

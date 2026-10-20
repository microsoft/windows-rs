use test_clang2::{fixture, selected_fixture};
use windows_clang2::*;

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

#[test]
#[ignore = "requires the pinned SDK and clang resource headers"]
fn sdk_namespace_dependencies() {
    helpers::ensure_libclang();
    let mut input = test_clang2::annotated_sdk_fixture();
    let fixture = fixture("sdk_namespace_dependencies");
    input.name = fixture.name;
    input.source = fixture.source;
    input.roots = [
        "ActivationType",
        "RegistrationScope",
        "RegisteredTrustLevel",
    ]
    .map(|name| format!("ABI::Windows::Foundation::{name}"))
    .into();
    let snapshots = capture(&[input], &TARGETS).unwrap();
    for snapshot in &snapshots {
        for name in ["ActivationType", "RegistrationScope"] {
            let enumeration = snapshot
                .enums
                .iter()
                .find(|enumeration| enumeration.name == format!("ABI::Windows::Foundation::{name}"))
                .unwrap();
            let observation = enumeration
                .observations
                .iter()
                .find(|observation| observation.members.is_some())
                .unwrap();
            assert_eq!(
                snapshot.types[observation.canonical.0].kind,
                TypeKind::Scalar(Scalar::Int)
            );
            assert_eq!(
                observation
                    .members
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|member| member.value)
                    .collect::<Vec<_>>(),
                [
                    EnumValue::Signed(0),
                    EnumValue::Signed(1),
                    EnumValue::Signed(2)
                ]
            );
        }
    }
    let output = projection::project(&merge(&snapshots).unwrap(), "Windows.Win32").unwrap();
    let actual = &output.partitions["activationregistration"].rdl;
    let expected = test_clang2::input_dir().join("sdk_namespace_dependencies.rdl");
    if std::env::var_os("UPDATE_CLANG2_RDL").is_some() {
        std::fs::write(&expected, actual).unwrap();
    }
    assert_eq!(
        *actual,
        std::fs::read_to_string(expected)
            .unwrap()
            .replace("\r\n", "\n")
    );
}

#[test]
fn namespace_header_selection() {
    helpers::ensure_libclang();
    for explicit in [false, true] {
        let mut input = selected_fixture("namespace_selection", &[]);
        input.namespace_headers = Some(if explicit {
            Vec::new()
        } else {
            vec!["namespace_selection.h".into()]
        });
        if explicit {
            input.roots.push("Selected::Root".into());
        }
        let captured = capture_partial(&[input], &TARGETS).unwrap();
        assert!(captured.omissions.is_empty(), "{:?}", captured.omissions);
        for snapshot in captured.snapshots {
            assert_eq!(snapshot.roots.len(), 1);
            let DeclarationId::Record(root) = snapshot.roots[0] else {
                panic!()
            };
            assert_eq!(snapshot.records[root.0].name, "Selected::Root");
            assert_eq!(snapshot.records.len(), 2);
            assert!(
                snapshot
                    .records
                    .iter()
                    .any(|record| record.name == "Right::Item")
            );
        }
    }
}

#[test]
fn namespace_native_identity() {
    helpers::ensure_libclang();
    let snapshots = capture(&[fixture("namespace_identity")], &TARGETS).unwrap();
    let merged = merge(&snapshots).unwrap();
    assert_eq!(
        merged
            .aliases
            .iter()
            .map(|alias| alias.name.as_str())
            .collect::<Vec<_>>(),
        ["Left::Callback", "Left::Number", "Right::Number"]
    );
    assert_eq!(
        merged
            .values
            .iter()
            .map(|value| value.name.as_str())
            .collect::<Vec<_>>(),
        ["Left::Value", "Right::Value"]
    );
    for snapshot in &snapshots {
        let child = snapshot
            .records
            .iter()
            .find(|record| record.name == "Child")
            .unwrap();
        let owner = child.owner.unwrap();
        assert_eq!(snapshot.records[owner.record.0].name, "Left::Container");
        assert_eq!(
            snapshot
                .enums
                .iter()
                .map(|value| value.name.as_str())
                .collect::<Vec<_>>(),
            ["Left::Kind", "Right::Kind", "Left::__MIDL_fixture_0001"]
        );
    }
    let selected = capture(
        &[selected_fixture("namespace_identity", &["Left::Item"])],
        &TARGETS,
    )
    .unwrap();
    for snapshot in selected {
        assert_eq!(snapshot.records.len(), 1);
        assert_eq!(snapshot.records[0].name, "Left::Item");
        assert_eq!(snapshot.aliases[0].name, "Left::Number");
    }
}

#[test]
fn namespace_partial_identity() {
    helpers::ensure_libclang();
    let partial = capture_partial(&[fixture("namespace_partial")], &TARGETS).unwrap();
    assert!(
        partial
            .omissions
            .iter()
            .any(|message| message.contains("Left::Item"))
    );
    assert!(
        partial
            .omissions
            .iter()
            .any(|message| message.contains("Left::Dependent"))
    );
    for snapshot in &partial.snapshots {
        let names: std::collections::BTreeSet<_> = snapshot
            .records
            .iter()
            .map(|record| record.name.as_str())
            .collect();
        assert_eq!(names, ["Right::Dependent", "Right::Item"].into());
    }
    merge(&partial.snapshots).unwrap();
}

#[test]
fn namespace_public_names() {
    helpers::ensure_libclang();
    let input = selected_fixture(
        "namespace_identity",
        &[
            "Left::Container",
            "Left::Value",
            "Left::Kind",
            "Left::Callbacks",
            "Left::__MIDL_fixture_0001",
        ],
    );
    let output = projection::generate(&[input], "Fixture").unwrap();
    let actual = &output.partitions["namespace_identity"].rdl;
    let expected = test_clang2::input_dir().join("namespace_identity.rdl");
    if std::env::var_os("UPDATE_CLANG2_RDL").is_some() {
        std::fs::write(&expected, actual).unwrap();
    }
    assert_eq!(
        *actual,
        std::fs::read_to_string(expected)
            .unwrap()
            .replace("\r\n", "\n")
    );
    let scratch = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..\\..\\..\\..\\target\\test-clang2-namespaces");
    std::fs::create_dir_all(&scratch).unwrap();
    let winmd = scratch.join("input.winmd");
    windows_rdl::reader()
        .input_text(actual)
        .output(&winmd)
        .write()
        .unwrap();
    let rewritten = scratch.join("output.rdl");
    windows_rdl::writer()
        .input(&winmd)
        .filter("Fixture")
        .output(&rewritten)
        .write()
        .unwrap();
    assert_eq!(*actual, std::fs::read_to_string(rewritten).unwrap());
}

#[test]
fn namespace_public_collisions() {
    helpers::ensure_libclang();
    for roots in [
        ["Left::Item", "Right::Item"],
        ["Left::Number", "Right::Number"],
        ["Left::Kind", "Right::Kind"],
        ["Left::Value", "Right::Value"],
    ] {
        let input = selected_fixture("namespace_identity", &roots);
        let snapshots = capture(&[input], &TARGETS).unwrap();
        let merged = merge(&snapshots).unwrap();
        let error = projection::project(&merged, "Fixture")
            .unwrap_err()
            .to_string();
        assert!(
            error.contains("collides between native"),
            "{roots:?}: {error}"
        );
        let error = projection::project_partial(
            PartialCapture {
                snapshots,
                omissions: Vec::new(),
            },
            "Fixture",
        )
        .unwrap_err()
        .to_string();
        assert!(
            error.contains("collides between native"),
            "{roots:?}: {error}"
        );
    }
    for name in ["namespace_anonymous", "namespace_targets"] {
        let error = projection::generate_partial(&[fixture(name)], "Fixture")
            .unwrap_err()
            .to_string();
        assert!(error.contains("collides between native"), "{name}: {error}");
    }
}

#[test]
fn namespace_native_conflicts() {
    helpers::ensure_libclang();
    let mut inputs = ["int", "unsigned int"].map(|field| {
        let mut input = selected_fixture("namespace_conflict", &["Scoped::Item"]);
        input
            .source
            .insert_str(0, &format!("#define FIELD {field}\n"));
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

#[test]
#[ignore = "requires the pinned SDK, clang resource headers, and three MSVC Rust targets"]
fn sdk_namespaces() {
    helpers::ensure_libclang();
    let mut input = test_clang2::annotated_sdk_fixture();
    let fixture = fixture("sdk_namespaces");
    input.name = fixture.name;
    input.source = fixture.source;
    input.roots = vec!["NamespaceConsumer".into()];
    let snapshots = capture(&[input], &TARGETS).unwrap();
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let reference_path = root.join("crates\\libs\\default\\Windows.winmd");
    let metadata = windows_metadata::reader::Index::read(&reference_path).unwrap();
    let references = projection::References {
        metadata: &metadata,
        native_prefixes: &["", "ABI"],
        delegate_prefix: Some("I"),
    };
    let output = projection::project_with_options(
        &merge(&snapshots).unwrap(),
        "Windows.Win32",
        &projection::Options {
            references: Some(&references),
            ..Default::default()
        },
    )
    .unwrap();
    let actual: String = output
        .partitions
        .values()
        .map(|part| part.rdl.as_str())
        .collect();
    let expected = test_clang2::input_dir().join("sdk_namespaces.rdl");
    if std::env::var_os("UPDATE_CLANG2_RDL").is_some() {
        std::fs::write(&expected, &actual).unwrap();
    }
    assert_eq!(
        actual,
        std::fs::read_to_string(expected)
            .unwrap()
            .replace("\r\n", "\n")
    );
    let scratch = root.join("target\\test-clang2-sdk-namespaces");
    std::fs::create_dir_all(&scratch).unwrap();
    let winmd = scratch.join("input.winmd");
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .reference(&reference_path)
        .input_text(&actual)
        .output(&winmd)
        .write()
        .unwrap();
    let generated = windows_metadata::reader::Index::read(&winmd).unwrap();
    assert_eq!(generated.get("Windows.Win32", "IBuffer").count(), 0);
    for name in ["IMemoryBufferByteAccess", "IBufferByteAccess"] {
        assert_eq!(generated.get("Windows.Win32", name).count(), 1);
    }
    let consumer = generated
        .get("Windows.Win32", "NamespaceConsumer")
        .next()
        .unwrap();
    assert_eq!(
        consumer.fields().nth(2).unwrap().ty(),
        windows_metadata::Type::class_named("Windows.Storage.Streams", "IBuffer")
    );
    assert_eq!(
        consumer.fields().last().unwrap().ty(),
        windows_metadata::Type::class_named("Windows.System", "DispatcherQueueHandler")
    );
    assert_eq!(
        generated
            .get("Windows.Win32", "IDispatcherQueueHandler")
            .count(),
        0
    );
    windows_bindgen::builder()
        .input(&winmd)
        .input(&reference_path)
        .filter("Windows.Win32.NamespaceConsumer")
        .filter("Windows.Win32.IMemoryBufferByteAccess")
        .filter("Windows.Win32.IBufferByteAccess")
        .filter("Windows.Storage.Streams.IBuffer")
        .filter("Windows.System.DispatcherQueueHandler")
        .flat()
        .output(scratch.join("bindings.rs"))
        .write();
    let manifest = scratch.join("Cargo.toml");
    std::fs::write(&manifest, format!(
        "[package]\nname = \"clang2-namespace-consumer\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\
         [workspace]\n[dependencies]\nwindows-core = {{ path = {:?} }}\n",
        root.join("crates\\libs\\core").display().to_string()
    )).unwrap();
    std::fs::create_dir_all(scratch.join("src")).unwrap();
    std::fs::write(
        scratch.join("src\\main.rs"),
        include_str!("../input/sdk_namespaces.rs"),
    )
    .unwrap();
    for target in TARGETS {
        let result = std::process::Command::new("cargo")
            .args([
                "build",
                "--quiet",
                "--target",
                target.triple(),
                "--manifest-path",
            ])
            .arg(&manifest)
            .arg("--target-dir")
            .arg(scratch.join("build"))
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        if cfg!(target_arch = "x86_64") && matches!(target, Target::X86 | Target::X64)
            || cfg!(target_arch = "aarch64") && target == Target::Arm64
        {
            let executable = scratch
                .join("build")
                .join(target.triple())
                .join("debug\\clang2-namespace-consumer.exe");
            assert!(
                std::process::Command::new(executable)
                    .status()
                    .unwrap()
                    .success()
            );
        }
    }
}

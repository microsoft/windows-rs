use windows_clang::*;

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

fn input(name: &str) -> Input {
    helpers::ensure_libclang();
    let mut input = test_clang::annotated_sdk_fixture();
    let fixture = test_clang::fixture("sdk_composition_interop");
    input.name = fixture.name;
    input.source = fixture.source;
    input.roots = vec![format!("ABI::Windows::UI::Composition::{name}")];
    input
}

#[test]
#[ignore = "requires the pinned SDK and clang resource headers"]
fn sdk_compositor_dependency_boundary() {
    let error = capture(&[input("ICompositorInterop")], &TARGETS)
        .unwrap_err()
        .to_string();
    assert!(
        error.contains("TemplateRef `ITypedEventHandler`: unsupported declaration child"),
        "{error}"
    );
    for target in TARGETS {
        assert!(error.contains(target.triple()), "{error}");
    }
}

#[test]
#[ignore = "requires the pinned SDK and clang resource headers"]
fn sdk_composition_drawing_surface() {
    let snapshots = capture(&[input("ICompositionDrawingSurfaceInterop")], &TARGETS).unwrap();
    for snapshot in &snapshots {
        let record = snapshot
            .records
            .iter()
            .find(|record| {
                record.name == "ABI::Windows::UI::Composition::ICompositionDrawingSurfaceInterop"
            })
            .unwrap();
        let definition = record
            .observations
            .iter()
            .find_map(|id| snapshot.observations[id.0].definition.as_ref())
            .unwrap();
        assert_eq!(
            definition
                .methods
                .iter()
                .map(|method| (method.name.as_str(), method.slot))
                .collect::<Vec<_>>(),
            [
                ("BeginDraw", 3),
                ("EndDraw", 4),
                ("Resize", 5),
                ("Scroll", 6),
                ("ResumeDraw", 7),
                ("SuspendDraw", 8)
            ]
        );
    }

    let output = projection::project(&merge(&snapshots).unwrap(), "Windows.Win32").unwrap();
    let actual = &output.partitions["windowsuicompositioninterop"].rdl;
    let expected = test_clang::input_dir().join("sdk_composition_interop.rdl");
    if std::env::var_os("UPDATE_CLANG_RDL").is_some() {
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
#[ignore = "requires the pinned SDK and clang resource headers"]
fn sdk_compositor_external() {
    compositor_external(true);
}

#[test]
#[ignore = "requires the pinned SDK and clang resource headers"]
fn sdk_compositor_external_metadata() {
    compositor_external(false);
}

fn compositor_external(consumers: bool) {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let reference_path = root.join("crates\\libs\\default\\Windows.winmd");
    let metadata = windows_metadata::reader::Index::read(&reference_path).unwrap();
    let references = projection::References {
        metadata: &metadata,
        native_prefixes: &["", "ABI"],
        delegate_prefix: Some("I"),
    };
    let mut forward = input("Desktop::ICompositorDesktopInterop");
    forward.source = forward
        .source
        .replace("#include <windows.ui.composition.desktop.h>", "");
    forward.external_interfaces = references.native_interfaces().unwrap();
    let snapshots = capture(&[forward], &TARGETS).unwrap();
    let error = projection::project_with_options(
        &merge(&snapshots).unwrap(),
        "Windows.Win32",
        &projection::Options {
            references: Some(&references),
            ..Default::default()
        },
    )
    .unwrap_err()
    .to_string();
    assert!(
        error.contains("IDesktopWindowTarget` has no native definition"),
        "{error}"
    );
    let mut input = input("ICompositorInterop");
    let interfaces = [
        ("ICompositionDrawingSurfaceInterop", 3, 6),
        ("ICompositionDrawingSurfaceInterop2", 9, 1),
        ("ICompositionGraphicsDeviceInterop", 3, 2),
        ("ICompositionTextureInterop", 3, 1),
        ("ICompositorInterop", 3, 3),
        ("ICompositorInterop2", 3, 2),
        ("Desktop::ICompositionCapabilitiesInteropFactory", 6, 1),
        ("Desktop::ICompositorDesktopInterop", 3, 2),
        ("Desktop::IDesktopWindowTargetInterop", 3, 1),
        ("Interactions::IVisualInteractionSourceInterop", 3, 1),
    ];
    input.roots = interfaces
        .iter()
        .map(|(name, _, _)| format!("ABI::Windows::UI::Composition::{name}"))
        .collect();
    input.roots.extend(
        [
            "ABI::Windows::UI::Composition::ICompositionTexture",
            "CreateDispatcherQueueController",
            "PDISPATCHERQUEUE",
            "PDISPATCHERQUEUECONTROLLER",
            "InterfaceName_Windows_UI_Composition_Desktop_IDesktopWindowTarget",
            "RuntimeClass_Windows_UI_Composition_Desktop_DesktopWindowTarget",
        ]
        .map(String::from),
    );
    input.external_interfaces = references.native_interfaces().unwrap();
    let imports = projection::Imports::from_iter([(
        "CreateDispatcherQueueController".into(),
        projection::Import {
            library: "CoreMessaging.dll".into(),
            name: None,
        },
    )]);
    let snapshots = capture(&[input], &TARGETS).unwrap();
    for snapshot in &snapshots {
        for (name, first_slot, count) in interfaces {
            let record = snapshot
                .records
                .iter()
                .find(|record| record.name == format!("ABI::Windows::UI::Composition::{name}"))
                .unwrap();
            assert!(!record.external);
            let definition = record
                .observations
                .iter()
                .find_map(|id| snapshot.observations[id.0].definition.as_ref())
                .unwrap();
            assert_eq!(
                definition
                    .methods
                    .iter()
                    .map(|method| method.slot)
                    .collect::<Vec<_>>(),
                (first_slot..first_slot + count).collect::<Vec<_>>(),
                "{name}"
            );
        }
        let external = snapshot
            .records
            .iter()
            .find(|record| {
                record.name == "ABI::Windows::UI::Composition::ICompositionGraphicsDevice"
            })
            .unwrap();
        assert!(external.external);
        for id in &external.observations {
            if let Some(definition) = &snapshot.observations[id.0].definition {
                assert!(definition.methods.is_empty());
                assert!(definition.fields.is_empty());
                assert_eq!(definition.bases.len(), 1);
            }
        }
        assert!(
            !snapshot
                .records
                .iter()
                .any(|record| record.name.contains("ITypedEventHandler"))
        );
    }
    let merged = merge(&snapshots).unwrap();
    assert!(
        projection::project(&merged, "Windows.Win32")
            .unwrap_err()
            .to_string()
            .contains("requires authoritative metadata")
    );
    let output = projection::project_with_options(
        &merged,
        "Windows.Win32",
        &projection::Options {
            references: Some(&references),
            imports: Some(&imports),
            ..Default::default()
        },
    )
    .unwrap();
    for (partition, fixture) in [
        ("windowsuicompositioninterop", "sdk_compositor_external"),
        ("dispatcherqueue", "sdk_dispatcher_queue"),
        ("windowsuicompositiondesktop", "sdk_composition_desktop"),
    ] {
        let expected = test_clang::input_dir().join(format!("{fixture}.rdl"));
        let actual = &output.partitions[partition].rdl;
        if std::env::var_os("UPDATE_CLANG_RDL").is_some() {
            std::fs::write(&expected, actual).unwrap();
        }
        assert_eq!(
            *actual,
            std::fs::read_to_string(expected)
                .unwrap()
                .replace("\r\n", "\n")
        );
    }
    assert!(
        !output.partitions["windowsuicompositioninterop"]
            .rdl
            .contains("type ICompositionTexture")
    );
    let scratch = root.join(format!("target\\test-clang-sdk-compositor-{consumers}"));
    std::fs::create_dir_all(&scratch).unwrap();
    let winmd = scratch.join("input.winmd");
    let text: String = output
        .partitions
        .values()
        .map(|partition| partition.rdl.as_str())
        .collect();
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .reference(&reference_path)
        .input_text(&text)
        .output(&winmd)
        .write()
        .unwrap();
    let generated = windows_metadata::reader::Index::read(&winmd).unwrap();
    assert_eq!(
        generated.get("Windows.Win32", "ICompositorInterop").count(),
        1
    );
    assert_eq!(
        generated
            .get("Windows.Win32", "ICompositionGraphicsDevice")
            .count(),
        0
    );
    assert_eq!(
        generated
            .get("Windows.UI.Composition", "ICompositionGraphicsDevice")
            .count(),
        0
    );
    if !consumers {
        std::fs::remove_file(winmd).unwrap();
        return;
    }
    windows_bindgen::builder()
        .input(&winmd)
        .input(&reference_path)
        .filter("Windows.Win32")
        .filter("Windows.UI.Composition.ICompositionSurface")
        .filter("Windows.UI.Composition.ICompositionGraphicsDevice")
        .filter("Windows.UI.Composition.ICompositionCapabilities")
        .filter("Windows.UI.Composition.ICompositionTexture")
        .filter("Windows.UI.Composition.Desktop.IDesktopWindowTarget")
        .minimal()
        .flat()
        .output(scratch.join("bindings.rs"))
        .write();
    let manifest = scratch.join("Cargo.toml");
    std::fs::write(&manifest, format!(
        "[package]\nname = \"clang-composition-consumer\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\
         [workspace]\n[dependencies]\nwindows-core = {{ path = {:?} }}\nwindows-future = {{ path = {:?} }}\n",
        root.join("crates\\libs\\core").display().to_string(),
        root.join("crates\\libs\\future").display().to_string()
    )).unwrap();
    std::fs::create_dir_all(scratch.join("src")).unwrap();
    std::fs::write(
        scratch.join("src\\main.rs"),
        include_str!("../input/sdk_composition_interop.rs"),
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
            assert!(
                std::process::Command::new(
                    scratch
                        .join("build")
                        .join(target.triple())
                        .join("debug\\clang-composition-consumer.exe")
                )
                .status()
                .unwrap()
                .success()
            );
        }
    }
}

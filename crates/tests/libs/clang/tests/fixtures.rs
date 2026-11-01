//! Each `input/fixtures/<name>.h` is captured for every target and projected into namespace
//! `Fixture`, with all functions imported from `fixture.dll`. The projection is written to
//! `<name>.rdl` and must round-trip through metadata.

use std::path::Path;
use windows_clang::*;

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];
const NAMESPACE: &str = "Fixture";

include!(concat!(env!("OUT_DIR"), "/fixture_tests.rs"));

/// SDK headers and the SAL transport used by `tool-win32`, so fixtures use the real SAL macros.
fn arguments(root: &Path) -> Vec<String> {
    let win32 = root.join("crates/tools/win32/src");
    let version = helpers::read_str_const(win32.join("main.rs"), "SDK_VERSION");
    let include = helpers::nuget_package("microsoft.windows.sdk.cpp", &version)
        .join("c/Include")
        .join(helpers::marketing_dir(&version));
    let mut arguments = vec![
        "-fms-extensions".into(),
        "-include".into(),
        std::fs::canonicalize(win32.join("sal.h"))
            .unwrap()
            .display()
            .to_string(),
        "-resource-dir".into(),
        helpers::clang_resource_dir(),
    ];
    for directory in ["ucrt", "um", "shared"] {
        arguments.extend([
            "-isystem".into(),
            include.join(directory).display().to_string(),
        ]);
    }
    arguments
}

fn run(name: &str) {
    helpers::ensure_libclang();
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../..");
    let dir = test_clang::input_dir().join("fixtures");
    let file = format!("{name}.h");
    let input = Input {
        source: std::fs::read_to_string(dir.join(&file)).unwrap(),
        name: file,
        arguments: arguments(&root),
        ..Default::default()
    };
    let snapshots = capture(&[input], &TARGETS).unwrap();
    let imports = snapshots
        .iter()
        .flat_map(|snapshot| &snapshot.values)
        .map(|value| {
            (
                value.name.clone(),
                projection::Import {
                    library: "fixture.dll".into(),
                    name: None,
                },
            )
        })
        .collect();
    let output =
        projection::project_with_imports(&merge(&snapshots).unwrap(), NAMESPACE, &imports).unwrap();
    let text: String = output
        .partitions
        .values()
        .map(|part| part.rdl.as_str())
        .collect();

    // Written unconditionally; CI rejects any resulting `git diff`.
    std::fs::write(dir.join(format!("{name}.rdl")), &text).unwrap();

    let metadata = root.join("metadata/metadata.rdl");
    let scratch = Path::new(env!("OUT_DIR"));
    let winmd = scratch.join(format!("{name}.winmd"));
    let rdl = scratch.join(format!("{name}.rdl"));
    windows_rdl::reader()
        .input(&metadata)
        .input_text(&text)
        .output(&winmd)
        .write()
        .unwrap();
    windows_rdl::writer()
        .input(&winmd)
        .filter(NAMESPACE)
        .output(&rdl)
        .write()
        .unwrap();
    assert_eq!(std::fs::read_to_string(&rdl).unwrap(), text, "{name}");
    windows_rdl::reader()
        .input(&metadata)
        .input(&rdl)
        .output(&winmd)
        .write()
        .unwrap();
}

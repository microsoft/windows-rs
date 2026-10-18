use windows_clang2::*;

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];
const FUNCTIONS: &[&str] = &[
    "HeapAlloc",
    "HeapReAlloc",
    "GetProcessHeap",
    "HeapFree",
    "GlobalAlloc",
    "GlobalReAlloc",
    "GlobalFree",
    "LocalAlloc",
    "LocalReAlloc",
    "LocalFree",
];

fn golden(name: &str, output: &projection::Output) {
    let text: String = output
        .partitions
        .values()
        .map(|part| part.rdl.as_str())
        .collect();
    let path = test_clang2::input_dir().join(format!("{name}.rdl"));
    if std::env::var_os("UPDATE_CLANG2_RDL").is_some() {
        std::fs::write(&path, &text).unwrap();
    }
    assert_eq!(
        text,
        std::fs::read_to_string(path).unwrap().replace("\r\n", "\n")
    );
}

#[test]
fn allocator_evidence() {
    helpers::ensure_libclang();
    let snapshots = capture(&[test_clang2::selected_fixture("allocator", &[])], &TARGETS).unwrap();
    for snapshot in &snapshots {
        for value in &snapshot.values {
            assert!(
                value.observations[0]
                    .attributes
                    .iter()
                    .any(|attribute| attribute.kind == AttributeKind::MSAllocator)
            );
        }
        let definition = snapshot.observations[0].definition.as_ref().unwrap();
        assert!(
            definition.methods[0]
                .observation
                .attributes
                .iter()
                .any(|attribute| attribute.kind == AttributeKind::MSAllocator)
        );
    }
    let imports = ["Allocate", "Reallocate"]
        .map(|name| {
            (
                name.into(),
                projection::Import {
                    library: "fixture.dll".into(),
                    name: None,
                },
            )
        })
        .into_iter()
        .collect();
    let output =
        projection::project_with_imports(&merge(&snapshots).unwrap(), "Fixture", &imports).unwrap();
    assert_eq!(output.functions, 2);
    assert_eq!(output.interfaces, 1);
    golden("allocator", &output);
}

#[test]
fn conflicting_allocator_evidence() {
    helpers::ensure_libclang();
    for root in ["Allocate", "Factory"] {
        let mut inputs = [
            test_clang2::selected_fixture("allocator", &[root]),
            test_clang2::selected_fixture("allocator", &[root]),
        ];
        inputs[0].source = "#include \"allocator.h\"".into();
        inputs[1].name = "without_allocator.h".into();
        inputs[1].source = "#define ALLOCATOR\n#include \"allocator.h\"".into();
        for _ in 0..2 {
            let error = capture_partial(&inputs, &TARGETS).unwrap_err().to_string();
            assert!(error.contains(root), "{error}");
            inputs.reverse();
        }
    }
}

#[test]
#[ignore = "requires the pinned SDK"]
fn sdk_allocator() {
    helpers::ensure_libclang();
    let mut input = test_clang2::annotated_sdk_fixture();
    input.name = "sdk_allocator.h".into();
    input.source = test_clang2::fixture("sdk_allocator").source;
    input.roots = FUNCTIONS.iter().map(|name| (*name).into()).collect();
    let snapshots = capture(&[input], &TARGETS).unwrap();
    for snapshot in &snapshots {
        let marked: Vec<_> = snapshot
            .values
            .iter()
            .filter(|value| {
                value.observations.iter().any(|observation| {
                    observation
                        .attributes
                        .iter()
                        .any(|attribute| attribute.kind == AttributeKind::MSAllocator)
                })
            })
            .map(|value| value.name.as_str())
            .collect();
        assert_eq!(marked.len(), 6, "{marked:?}");
    }
    let imports = FUNCTIONS
        .iter()
        .map(|name| {
            (
                (*name).into(),
                projection::Import {
                    library: "kernel32.dll".into(),
                    name: None,
                },
            )
        })
        .collect();
    let output =
        projection::project_with_imports(&merge(&snapshots).unwrap(), "Fixture", &imports).unwrap();
    assert_eq!(output.functions, FUNCTIONS.len());
    golden("sdk_allocator", &output);
}

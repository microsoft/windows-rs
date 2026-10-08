use std::collections::BTreeSet;
use std::path::Path;
use windows_clang2::{Input, Plan, ProjectionOptions, ReferenceKind, TypeReference};

pub fn generate() {
    helpers::ensure_libclang();
    helpers::assert_libclang_version();
    let version = helpers::read_str_const("crates/tools/win32/src/main.rs", "SDK_VERSION");
    let (marketing, _) = version.rsplit_once('.').unwrap();
    let include = helpers::nuget_package("microsoft.windows.sdk.cpp", &version)
        .join("c")
        .join("Include")
        .join(format!("{marketing}.0"));
    let plan = project(
        "i686-pc-windows-msvc",
        &include,
        Path::new("crates/libs/clang2/src/sal.h"),
        inputs(),
    )
    .unwrap();
    assert!(
        plan.omitted().is_empty(),
        "omitted animation roots: {:?}",
        plan.omitted()
    );
    let output = Path::new("target/animation");
    std::fs::create_dir_all(output).unwrap();
    let rdl = output.join("Animation.rdl");
    std::fs::write(&rdl, plan.rdl()).unwrap();
    let winmd = output.join("Animation.winmd");
    windows_rdl::reader()
        .input(rdl)
        .input_text(include_str!("../../../libs/clang2/metadata.rdl"))
        .reference_default()
        .output(&winmd)
        .write()
        .unwrap();
    windows_bindgen::bindgen(["--etc", "crates/tools/bindings/src/animation.txt"]);
}

pub fn roots() -> Vec<&'static str> {
    include_str!("animation.txt")
        .split_once("--filter")
        .unwrap()
        .1
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|name| {
            name.split(':')
                .next()
                .unwrap()
                .strip_prefix("Animation.")
                .unwrap()
        })
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

pub fn inputs() -> [Input; 2] {
    [
        Input::new(
            "animation.hpp",
            "#include <windows.h>\n#include <UIAnimation.h>",
        ),
        Input::new(
            "composition.hpp",
            "#include <windows.h>\n#include <dcomp.h>",
        ),
    ]
}

pub fn project(
    target: &str,
    include: &Path,
    sal: &Path,
    inputs: impl IntoIterator<Item = Input>,
) -> Result<Plan, windows_clang2::Error> {
    let mut arguments = vec![
        "-x".to_string(),
        "c++".into(),
        format!("--target={target}"),
        "-fms-extensions".into(),
        "-include".into(),
        sal.to_str().unwrap().into(),
    ];
    for directory in ["shared", "um", "ucrt"] {
        arguments.extend([
            "-isystem".into(),
            include.join(directory).to_str().unwrap().into(),
        ]);
    }
    let snapshot = windows_clang2::capture(
        inputs,
        &arguments.iter().map(String::as_str).collect::<Vec<_>>(),
        &roots(),
    )?;
    let mut options = ProjectionOptions::new("Animation");
    options.class_guids = Some(TypeReference {
        namespace: "Windows.Win32".into(),
        name: "GUID".into(),
        kind: ReferenceKind::Value,
    });
    for (native, namespace, name, kind) in [
        ("_GUID", "Windows.Win32", "GUID", ReferenceKind::Value),
        (
            "HRESULT",
            "Windows.Foundation",
            "HResult",
            ReferenceKind::Value,
        ),
        ("BOOL", "Windows.Win32", "BOOL", ReferenceKind::Value),
        (
            "IUnknown",
            "Windows.Win32",
            "IUnknown",
            ReferenceKind::Interface,
        ),
        (
            "IDCompositionAnimation",
            "Windows.Win32",
            "IDCompositionAnimation",
            ReferenceKind::Interface,
        ),
    ] {
        options.references.insert(
            native.into(),
            TypeReference {
                namespace: namespace.into(),
                name: name.into(),
                kind,
            },
        );
    }
    snapshot.resolve()?.project(&options)
}

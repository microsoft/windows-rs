use std::path::PathBuf;
use windows_clang2::{Input, extract};

const EXPECTED: &str = include_str!("../../../../metadata/win32/unknwnbase.rdl");
const WIN32_TOOL: &str = include_str!("../../../tools/win32/src/main.rs");

#[test]
fn unknwnbase_interfaces_match_committed_rdl() {
    windows_clang::ensure_libclang();

    let version = rust_string_constant(WIN32_TOOL, "SDK_VERSION");
    let (marketing, _) = version.rsplit_once('.').unwrap();
    let include = PathBuf::from(std::env::var_os("USERPROFILE").unwrap())
        .join(".nuget")
        .join("packages")
        .join("microsoft.windows.sdk.cpp")
        .join(version)
        .join("c")
        .join("Include")
        .join(format!("{marketing}.0"));
    let header = include.join("um").join("unknwnbase.h");
    let source = std::fs::read_to_string(&header).unwrap();
    let shared = include.join("shared");
    let um = include.join("um");
    let sal = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("tools")
        .join("win32")
        .join("src")
        .join("sal.h");
    let snapshot = extract(
        [Input::new(header.to_string_lossy(), source)],
        &[
            "-x",
            "c++",
            "-ferror-limit=0",
            "-include",
            sal.to_str().unwrap(),
            "-isystem",
            shared.to_str().unwrap(),
            "-isystem",
            um.to_str().unwrap(),
        ],
    )
    .unwrap();
    assert!(
        snapshot.facts().iter().any(|fact| fact.name == "IUnknown"),
        "{:?}",
        snapshot
            .facts()
            .iter()
            .filter(|fact| fact.name.contains("Unknown"))
            .collect::<Vec<_>>()
    );
    let actual = snapshot
        .emit_with_library("Windows.Win32", "RPCRT4.dll")
        .unwrap();

    for name in ["IUnknown", "IClassFactory", "AsyncIUnknown"] {
        assert_eq!(
            interface(&actual, name),
            interface(EXPECTED, name),
            "mismatched real-header interface `{name}`"
        );
    }
}

fn interface(source: &str, name: &str) -> String {
    let lines: Vec<_> = source.lines().map(str::trim).collect();
    let start = lines
        .iter()
        .position(|line| {
            line.strip_prefix("interface ").is_some_and(|declaration| {
                declaration == format!("{name} {{") || declaration.starts_with(&format!("{name}: "))
            })
        })
        .unwrap_or_else(|| {
            panic!(
                "interface `{name}` not found in:\n{}",
                lines
                    .iter()
                    .filter(|line| line.contains("interface "))
                    .copied()
                    .collect::<Vec<_>>()
                    .join("\n")
            )
        });
    let mut result = vec![lines[start - 1]];
    for line in &lines[start..] {
        result.push(line);
        if *line == "}" {
            break;
        }
    }
    result.join("\n")
}

fn rust_string_constant<'a>(source: &'a str, name: &str) -> &'a str {
    let prefix = format!("const {name}: &str = \"");
    let value = source
        .lines()
        .find_map(|line| line.trim().strip_prefix(&prefix))
        .unwrap();
    value.strip_suffix("\";").unwrap()
}

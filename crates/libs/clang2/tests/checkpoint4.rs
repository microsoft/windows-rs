use std::collections::BTreeMap;
use std::path::PathBuf;
use windows_clang2::{Input, extract};

const SDKDDKVER_RDL: &str = include_str!("../../../../metadata/win32/sdkddkver.rdl");
const WINAPI_FAMILY_RDL: &str = include_str!("../../../../metadata/win32/winapifamily.rdl");
const AUDIO_SESSION_TYPES_RDL: &str =
    include_str!("../../../../metadata/win32/audiosessiontypes.rdl");
const WIN32_TOOL: &str = include_str!("../../../tools/win32/src/main.rs");

#[test]
fn sdkddkver_matches_committed_supported_constants() {
    windows_clang::ensure_libclang();

    assert_header("shared", "sdkddkver.h", SDKDDKVER_RDL, &[]);
    assert_header("shared", "winapifamily.h", WINAPI_FAMILY_RDL, &[]);
    assert_header(
        "um",
        "audiosessiontypes.h",
        AUDIO_SESSION_TYPES_RDL,
        &["-DANYSIZE_ARRAY=1"],
    );
}

fn assert_header(folder: &str, file: &str, expected_source: &str, definitions: &[&str]) {
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
    let header = include.join(folder).join(file);
    let source = std::fs::read_to_string(&header).unwrap();
    let shared = include.join("shared");
    let um = include.join("um");
    let mut args = vec![
        "-x",
        "c++",
        "-isystem",
        shared.to_str().unwrap(),
        "-isystem",
        um.to_str().unwrap(),
    ];
    args.extend_from_slice(definitions);
    let actual_source = extract([Input::new(header.to_string_lossy(), source)], &args)
        .unwrap()
        .emit("Windows.Win32")
        .unwrap();

    let expected = constants(expected_source);
    let actual = constants(&actual_source);
    for (name, declaration) in expected {
        assert_eq!(
            actual.get(name),
            Some(&declaration),
            "mismatched real-header constant `{name}`"
        );
    }
    for name in actual.keys() {
        assert!(
            expected_source.contains(&format!("const {name}:")) || name.starts_with('_'),
            "unexpected public constant `{name}`"
        );
    }
    assert_eq!(enums(&actual_source), enums(expected_source));

    let output = std::env::temp_dir().join(format!(
        "windows-clang2-{}-{file}.winmd",
        std::process::id()
    ));
    windows_rdl::reader()
        .input_text(&actual_source)
        .output(&output)
        .write()
        .unwrap();
    std::fs::remove_file(output).unwrap();
}

fn constants(source: &str) -> BTreeMap<&str, String> {
    source
        .lines()
        .filter_map(|line| {
            let declaration = line.trim().strip_prefix("const ")?;
            let (name, _) = declaration.split_once(':')?;
            Some((name, format!("const {declaration}")))
        })
        .collect()
}

fn enums(source: &str) -> BTreeMap<String, Vec<String>> {
    let mut result = BTreeMap::new();
    let mut repr = None;
    let mut current = None;
    for line in source.lines().map(str::trim) {
        if line.starts_with("#[repr(") {
            repr = Some(line.to_string());
        } else if let Some(name) = line
            .strip_prefix("enum ")
            .and_then(|line| line.strip_suffix(" {"))
        {
            current = Some((name.to_string(), vec![repr.take().unwrap()]));
        } else if line == "}" {
            if let Some((name, body)) = current.take() {
                result.insert(name, body);
            }
        } else if let Some((_, body)) = &mut current {
            body.push(line.to_string());
        }
    }
    result
}

fn rust_string_constant<'a>(source: &'a str, name: &str) -> &'a str {
    let prefix = format!("const {name}: &str = \"");
    let value = source
        .lines()
        .find_map(|line| line.trim().strip_prefix(&prefix))
        .unwrap();
    value.strip_suffix("\";").unwrap()
}

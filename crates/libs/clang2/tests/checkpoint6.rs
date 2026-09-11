use std::collections::{BTreeMap, BTreeSet};
use windows_clang2::{EmitOptions, Input, TypeReference, TypeReferenceKind, extract};

const WIN32_TOOL: &str = include_str!("../../../tools/win32/src/main.rs");

#[test]
fn generator_policies_route_references_and_exports() {
    windows_clang::ensure_libclang();

    let scratch = std::env::temp_dir().join(format!(
        "windows-clang2-generator-policies-{}",
        std::process::id()
    ));
    std::fs::create_dir_all(&scratch).unwrap();
    std::fs::write(scratch.join("external.hpp"), "struct IExternal;\n").unwrap();
    let include = format!("-I{}", scratch.display());
    let snapshot = extract(
        [Input::new(
            scratch.join("api.hpp").to_string_lossy(),
            "#include \"external.hpp\"\nvoid Keep(IExternal* value);\nvoid Drop();\n",
        )],
        &["-x", "c++", &include],
    )
    .unwrap();

    let references = BTreeMap::from([(
        "IExternal".to_string(),
        TypeReference::new("External.Api", "IExternal", TypeReferenceKind::Interface),
    )]);
    let functions = BTreeSet::from(["Keep".to_string()]);
    let mut options = EmitOptions::new("Local.Api", &references);
    options.library = Some("api.dll");
    options.functions = Some(&functions);
    let rdl = snapshot.emit_with_options(&options).unwrap();

    assert!(
        rdl.contains("fn Keep(value: External::Api::IExternal)"),
        "{rdl}"
    );
    assert!(!rdl.contains("struct IExternal"));
    assert!(!rdl.contains("fn Drop"));

    let missing = BTreeSet::from(["Missing".to_string()]);
    options.functions = Some(&missing);
    assert!(
        snapshot
            .emit_with_options(&options)
            .unwrap_err()
            .to_string()
            .contains("selected function `Missing` was not found")
    );

    std::fs::remove_file(scratch.join("external.hpp")).unwrap();
    std::fs::remove_dir(scratch).unwrap();
}

#[test]
fn enum_flag_macro_controls_projection() {
    windows_clang::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "flags.hpp",
            "#define DEFINE_ENUM_FLAG_OPERATORS(type)\n\
             typedef enum FLAGS { FLAGS_NONE = 0, FLAGS_ALL = -1 } FLAGS;\n\
             DEFINE_ENUM_FLAG_OPERATORS(FLAGS)\n",
        )],
        &["-x", "c++"],
    )
    .unwrap();
    let rdl = snapshot.emit("Flags").unwrap();

    assert!(rdl.contains("#[repr(u32)]\n    #[flags]\n    enum FLAGS"));
    assert!(rdl.contains("FLAGS_ALL = 4294967295"));
}

#[test]
fn source_macro_preserves_calling_convention() {
    windows_clang::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "calling.hpp",
            "#define WINAPI __stdcall\n\
             void WINAPI SystemCall();\n\
             void PlainCall();\n",
        )],
        &[
            "-x",
            "c++",
            "--target=x86_64-pc-windows-msvc",
            "-fms-extensions",
        ],
    )
    .unwrap();
    let rdl = snapshot.emit_with_library("Calling", "api.dll").unwrap();

    assert!(rdl.contains("extern fn SystemCall()"));
    assert!(rdl.contains("extern \"C\" fn PlainCall()"));
}

#[test]
fn included_headers_can_be_declaration_roots() {
    windows_clang::ensure_libclang();

    let scratch = std::env::temp_dir().join(format!(
        "windows-clang2-included-root-{}",
        std::process::id()
    ));
    std::fs::create_dir_all(&scratch).unwrap();
    let header = scratch.join("api.hpp");
    std::fs::write(
        &header,
        "typedef struct VALUE { int value; } VALUE;\nvoid UseValue(VALUE value);\n",
    )
    .unwrap();
    let include = format!("-I{}", scratch.display());
    let snapshot = extract(
        [Input::new(
            scratch.join("combined.hpp").to_string_lossy(),
            "#include \"api.hpp\"\n",
        )
        .with_roots([header.to_string_lossy()])],
        &["-x", "c++", &include],
    )
    .unwrap();
    let rdl = snapshot.emit_with_library("Included", "api.dll").unwrap();

    assert!(rdl.contains("struct VALUE"));
    assert!(rdl.contains("fn UseValue(value: VALUE)"));

    std::fs::remove_file(header).unwrap();
    std::fs::remove_dir(scratch).unwrap();
}

#[test]
fn real_win32_header_plans_from_combined_translation_unit() {
    windows_clang::ensure_libclang();

    let version = rust_string_constant(WIN32_TOOL, "SDK_VERSION");
    let (marketing, _) = version.rsplit_once('.').unwrap();
    let include = std::path::PathBuf::from(std::env::var_os("USERPROFILE").unwrap())
        .join(".nuget")
        .join("packages")
        .join("microsoft.windows.sdk.cpp")
        .join(version)
        .join("c")
        .join("Include")
        .join(format!("{marketing}.0"));
    let header = include.join("um").join("fileapi.h");
    let shared = include.join("shared");
    let um = include.join("um");
    let sal = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("tools")
        .join("win32")
        .join("src")
        .join("sal.h");
    let source = "#define SECURITY_WIN32\n\
                  #include <winsock2.h>\n\
                  #include <windows.h>\n\
                  #include <fileapi.h>\n";
    let snapshot = extract(
        [Input::new("clang2-win32-slice.hpp", source).with_roots([header.to_string_lossy()])],
        &[
            "-x",
            "c++",
            "--target=x86_64-pc-windows-msvc",
            "-fms-extensions",
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
    let rdl = snapshot
        .emit_with_library("Windows.Win32", "Kernel32.dll")
        .unwrap();

    assert!(rdl.contains("fn CreateFileW("));
    assert!(rdl.contains("struct WIN32_FILE_ATTRIBUTE_DATA"));
}

fn rust_string_constant<'a>(source: &'a str, name: &str) -> &'a str {
    let prefix = format!("const {name}: &str = \"");
    source
        .lines()
        .find_map(|line| line.trim().strip_prefix(&prefix))
        .unwrap()
        .strip_suffix("\";")
        .unwrap()
}

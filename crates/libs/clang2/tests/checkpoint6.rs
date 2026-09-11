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
            "#include \"external.hpp\"\n\
             typedef struct IExternal IExternal;\n\
             extern \"C\" void Keep(IExternal* value);\n\
             extern \"C\" void Drop();\n",
        )],
        &["-x", "c++", &include],
    )
    .unwrap();

    let references = BTreeMap::from([(
        "IExternal".to_string(),
        TypeReference::new("External.Api", "IExternal", TypeReferenceKind::Interface),
    )]);
    let functions = BTreeSet::from(["Keep".to_string()]);
    let libraries = BTreeMap::from([("Keep".to_string(), "routed.dll".to_string())]);
    let mut options = EmitOptions::new("Local.Api", &references);
    options.libraries = Some(&libraries);
    options.functions = Some(&functions);
    let rdl = snapshot.emit_with_options(&options).unwrap();

    assert!(
        rdl.contains("fn Keep(value: External::Api::IExternal)"),
        "{rdl}"
    );
    assert!(rdl.contains("#[library(\"routed.dll\")]"));
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

#[test]
fn concrete_record_inheritance_flattens_verified_layout() {
    windows_clang::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "inheritance.hpp",
            "typedef struct BASE { int first; } BASE;\n\
             typedef struct OTHER { short third; } OTHER;\n\
             typedef struct DERIVED : BASE, OTHER { void* second; } DERIVED;\n",
        )],
        &["-x", "c++", "--target=x86_64-pc-windows-msvc"],
    )
    .unwrap();
    let derived = snapshot
        .facts()
        .iter()
        .find(|fact| fact.name == "DERIVED")
        .unwrap();
    let windows_clang2::FactData::Record { base, fields, .. } = &derived.data else {
        panic!("DERIVED is not a record");
    };
    assert!(base.is_some());
    assert_eq!(fields[0].name, "Base");
    assert_eq!(fields[1].name, "Base2");
    assert_eq!(fields[2].name, "second");

    let rdl = snapshot.emit("Inheritance").unwrap();
    assert!(rdl.contains("Base: BASE"));
    assert!(rdl.contains("Base2: OTHER"));
    assert!(rdl.contains("second: *mut void"));
}

#[test]
fn interface_record_base_preserves_base_subobject() {
    windows_clang::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "interface_base.hpp",
            "struct __declspec(uuid(\"00000000-0000-0000-C000-000000000046\")) IFACE {\n\
                 virtual int Query() = 0;\n\
             };\n\
             typedef struct RESULT : IFACE { void* value; } RESULT;\n",
        )],
        &["-x", "c++", "--target=x86_64-pc-windows-msvc"],
    )
    .unwrap();
    let result = snapshot
        .facts()
        .iter()
        .find(|fact| fact.name == "RESULT")
        .unwrap();
    let windows_clang2::FactData::Record { fields, .. } = &result.data else {
        panic!("RESULT is not a record");
    };
    assert_eq!(fields[0].name, "Base");
    assert_eq!(fields[0].offset, 0);
    assert_eq!(fields[1].name, "value");

    let rdl = snapshot.emit("InterfaceBase").unwrap();
    assert!(rdl.contains("Base: IFACE"));
    assert!(rdl.contains("value: *mut void"));
}

#[test]
fn same_tu_compatible_typedef_redeclarations_collapse() {
    windows_clang::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "redeclared.hpp",
            "typedef void* HANDLE;\n\
             typedef HANDLE* PHANDLE;\n\
             typedef void* HANDLE;\n\
             typedef HANDLE* PHANDLE;\n\
             typedef unsigned char BYTE;\n\
             typedef BYTE BOOLEAN;\n\
             typedef unsigned char boolean;\n\
             typedef boolean BOOLEAN;\n\
             typedef long LONG;\n\
             typedef long NTSTATUS;\n\
             typedef NTSTATUS* PNTSTATUS;\n\
             typedef LONG* PNTSTATUS;\n",
        )],
        &["-x", "c++"],
    )
    .unwrap();
    let rdl = snapshot.emit("Redeclared").unwrap();

    assert_eq!(rdl.matches("type HANDLE =").count(), 1);
    assert_eq!(rdl.matches("type PHANDLE =").count(), 1);
    assert_eq!(rdl.matches("type BOOLEAN =").count(), 1);
    assert_eq!(rdl.matches("type PNTSTATUS =").count(), 1);
}

#[test]
fn same_tu_compatible_function_redeclarations_collapse() {
    windows_clang::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "redeclared-function.hpp",
            "extern \"C\" void Shared(int value);\nextern \"C\" void Shared(int value);\n",
        )],
        &["-x", "c++"],
    )
    .unwrap();
    let rdl = snapshot.emit_with_library("Redeclared", "api.dll").unwrap();

    assert_eq!(rdl.matches("fn Shared(").count(), 1);
}

#[test]
fn indirect_sal_size_parameter_is_preserved() {
    windows_clang::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "indirect-size.hpp",
            "#define WRITES_BYTES(c) __attribute__((annotate(\"_Out_writes_bytes_(\" #c \")\")))\n\
             void Read(unsigned* size, WRITES_BYTES(*size) void* data);\n",
        )],
        &["-x", "c++"],
    )
    .unwrap();
    let rdl = snapshot.emit_with_library("Sal", "api.dll").unwrap();

    assert!(rdl.contains("#[size_param(0)] data: *mut void"));
}

#[test]
fn callback_array_parameters_decay_to_pointers() {
    windows_clang::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "callback-array.hpp",
            "typedef int (__stdcall *CALLBACK)(unsigned count, const unsigned values[], \
             const wchar_t* const names[]);\n",
        )],
        &[
            "-x",
            "c++",
            "--target=x86_64-pc-windows-msvc",
            "-fms-extensions",
        ],
    )
    .unwrap();
    let rdl = snapshot.emit("Callback").unwrap();

    assert!(rdl.contains("param1: *const u32"), "{rdl}");
    assert!(rdl.contains("param2: *const *const u16"), "{rdl}");
}

#[test]
fn uuid_class_projects_to_coclass_guid() {
    windows_clang::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "coclass.hpp",
            "typedef class Widget Widget;\n\
             class __declspec(uuid(\"12345678-1234-5678-90ab-cdef12345678\")) Widget;\n",
        )],
        &[
            "-x",
            "c++",
            "--target=x86_64-pc-windows-msvc",
            "-fms-extensions",
        ],
    )
    .unwrap();
    let rdl = snapshot.emit("Coclass").unwrap();

    assert!(rdl.contains("const Widget: GUID = 0x12345678_1234_5678_90ab_cdef12345678;"));
    assert!(!rdl.contains("type Widget"));
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

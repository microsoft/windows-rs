use std::collections::{BTreeMap, BTreeSet};
use windows_clang::{EmitOptions, FactData, Input, TypeReference, TypeReferenceKind, extract};

const WIN32_TOOL: &str = include_str!("../../../tools/win32/src/main.rs");

#[test]
fn generator_policies_route_references_and_exports() {
    helpers::ensure_libclang();

    let scratch = std::env::temp_dir().join(format!(
        "windows-clang-generator-policies-{}",
        std::process::id()
    ));
    std::fs::create_dir_all(&scratch).unwrap();
    std::fs::write(
        scratch.join("external.hpp"),
        "struct IExternal;\nstruct ExternalRecord;\n",
    )
    .unwrap();
    let include = format!("-I{}", scratch.display());
    let snapshot = extract(
        [Input::new(
            scratch.join("api.hpp").to_string_lossy(),
            "#include \"external.hpp\"\n\
             typedef struct IExternal IExternal;\n\
             typedef IExternal * PExternal;\n\
             typedef struct ExternalRecord ExternalRecord;\n\
             extern \"C\" void Keep(IExternal* value);\n\
             extern \"C\" void KeepAlias(PExternal* value);\n\
             extern \"C\" void KeepRecord(ExternalRecord value);\n\
             extern \"C\" void Drop();\n",
        )],
        &["-x", "c++", &include],
    )
    .unwrap();

    let references = BTreeMap::from([
        (
            "IExternal".to_string(),
            TypeReference::new("External.Api", "IExternal", TypeReferenceKind::Interface),
        ),
        (
            "ExternalRecord".to_string(),
            TypeReference::new("External.Api", "ExternalRecord", TypeReferenceKind::Type),
        ),
    ]);
    let functions = BTreeSet::from([
        "Keep".to_string(),
        "KeepAlias".to_string(),
        "KeepRecord".to_string(),
    ]);
    let libraries = BTreeMap::from([
        ("Keep".to_string(), "routed.dll".to_string()),
        ("KeepAlias".to_string(), "routed.dll".to_string()),
        ("KeepRecord".to_string(), "routed.dll".to_string()),
    ]);
    let mut options = EmitOptions::new("Local.Api", &references);
    options.libraries = Some(&libraries);
    options.functions = Some(&functions);
    let rdl = snapshot.emit_with_options(&options).unwrap();

    assert!(
        rdl.contains("fn Keep(value: External::Api::IExternal)"),
        "{rdl}"
    );
    assert!(
        rdl.contains("fn KeepAlias(value: *mut External::Api::IExternal)"),
        "{rdl}"
    );
    assert!(!rdl.contains("type PExternal"), "{rdl}");
    assert!(
        rdl.contains("fn KeepRecord(value: External::Api::ExternalRecord)"),
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
fn exclusion_references_keep_only_locally_extended_enums() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "additive.hpp",
            "typedef enum _EXISTING_ENUM { EXISTING_VALUE = 1, ADDED_VALUE = 2 } EXISTING_ENUM;\n\
             typedef EXISTING_ENUM PUBLIC_ENUM;\n\
             typedef struct _EXISTING_RECORD { int value; } EXISTING_RECORD;\n\
             #define EXISTING_CONSTANT 2\n\
             extern \"C\" void ExistingFunction();\n\
             extern \"C\" void UseExistingTag(struct _EXISTING_RECORD* value);\n",
        )],
        &["-x", "c++"],
    )
    .unwrap();
    let references = BTreeMap::from([
        (
            "PUBLIC_ENUM".to_string(),
            TypeReference::new("Base", "PUBLIC_ENUM", TypeReferenceKind::Enum)
                .with_enum_members(["EXISTING_VALUE"]),
        ),
        (
            "EXISTING_RECORD".to_string(),
            TypeReference::new("Base", "EXISTING_RECORD", TypeReferenceKind::Type),
        ),
    ]);
    let excluded = BTreeSet::from([
        "EXISTING_CONSTANT".to_string(),
        "EXISTING_RECORD".to_string(),
        "ExistingFunction".to_string(),
        "PUBLIC_ENUM".to_string(),
    ]);
    let mut options = EmitOptions::new("Additive", &references);
    options.excluded = Some(&excluded);
    options.library = Some("test.dll");
    let rdl = snapshot.emit_with_options(&options).unwrap();

    assert!(rdl.contains("enum PUBLIC_ENUM"), "{rdl}");
    assert!(rdl.contains("ADDED_VALUE = 2"));
    assert!(!rdl.contains("struct EXISTING_RECORD"));
    assert!(!rdl.contains("EXISTING_CONSTANT"));
    assert!(!rdl.contains("ExistingFunction"));
    assert!(rdl.contains("fn UseExistingTag(value: *mut Base::EXISTING_RECORD)"));
}

#[test]
fn exclusion_drops_unmodified_referenced_enums() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "unchanged-enum.hpp",
            "typedef enum _EXISTING_ENUM { EXISTING_VALUE = 1 } EXISTING_ENUM;\n\
             extern \"C\" void UseEnum(EXISTING_ENUM value);\n",
        )],
        &["-x", "c++"],
    )
    .unwrap();
    let references = BTreeMap::from([(
        "EXISTING_ENUM".to_string(),
        TypeReference::new("Base", "EXISTING_ENUM", TypeReferenceKind::Enum)
            .with_enum_members(["EXISTING_VALUE"]),
    )]);
    let excluded = BTreeSet::from(["EXISTING_ENUM".to_string()]);
    let mut options = EmitOptions::new("Additive", &references);
    options.excluded_types = Some(&excluded);
    options.library = Some("test.dll");
    let rdl = snapshot.emit_with_options(&options).unwrap();

    assert!(!rdl.contains("enum EXISTING_ENUM"), "{rdl}");
    assert!(rdl.contains("UseEnum(value: Base::EXISTING_ENUM)"), "{rdl}");
}

#[test]
fn exclusion_uses_base_enum_when_only_values_differ() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "changed-enum.hpp",
            "typedef enum _EXISTING_ENUM { EXISTING_VALUE = 2 } EXISTING_ENUM;\n",
        )],
        &["-x", "c++"],
    )
    .unwrap();
    let references = BTreeMap::from([(
        "EXISTING_ENUM".to_string(),
        TypeReference::new("Base", "EXISTING_ENUM", TypeReferenceKind::Enum)
            .with_enum_members(["EXISTING_VALUE"]),
    )]);
    let excluded = BTreeSet::from(["EXISTING_ENUM".to_string()]);
    let mut options = EmitOptions::new("Additive", &references);
    options.excluded_types = Some(&excluded);
    let rdl = snapshot.emit_with_options(&options).unwrap();

    assert!(!rdl.contains("enum EXISTING_ENUM"), "{rdl}");
}

#[test]
fn canonical_aliases_use_external_namespaces() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "canonical-alias.hpp",
            "typedef unsigned short* LPWSTR;\n\
             extern \"C\" void GetString(LPWSTR* value);\n",
        )],
        &["-x", "c++"],
    )
    .unwrap();
    let references = BTreeMap::from([(
        "PWSTR".to_string(),
        TypeReference::new("Windows.Win32", "PWSTR", TypeReferenceKind::Type),
    )]);
    let excluded = BTreeSet::from(["PWSTR".to_string()]);
    let mut options = EmitOptions::new("Namespaced", &references);
    options.library = Some("api.dll");
    options.excluded_types = Some(&excluded);
    let rdl = snapshot.emit_with_options(&options).unwrap();

    assert!(
        rdl.contains("GetString(value: *mut Windows::Win32::PWSTR)"),
        "{rdl}"
    );
}

#[test]
fn guid_macro_arguments_emit_constants() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "guids.hpp",
            "#define DEFINE_GUID(name, ...)\n\
             #define DEFINE_OLEGUID(name, ...)\n\
             DEFINE_GUID(GUID_SAMPLE, 0x12345678, 0x1234, 0x5678, 0x90, 0xab, 0xcd, \
             0xef, 0x12, 0x34, 0x56, 0x78)\n\
             DEFINE_OLEGUID(GUID_OLE_SAMPLE, 0x87654321, 0x4321, 0x8765)\n",
        )],
        &["-x", "c++"],
    )
    .unwrap();
    let rdl = snapshot.emit("Guids").unwrap();

    assert!(rdl.contains("const GUID_SAMPLE: GUID = 0x12345678_1234_5678_90ab_cdef12345678"));
    assert!(rdl.contains("const GUID_OLE_SAMPLE: GUID = 0x87654321_4321_8765_c000_000000000046"));

    let excluded = BTreeSet::from(["GUID_SAMPLE".to_string()]);
    let references = BTreeMap::new();
    let mut options = EmitOptions::new("Guids", &references);
    options.excluded_constants = Some(&excluded);
    let rdl = snapshot.emit_with_options(&options).unwrap();

    assert!(!rdl.contains("const GUID_SAMPLE"));
    assert!(rdl.contains("const GUID_OLE_SAMPLE"));
}

#[test]
fn macro_overrides_do_not_replace_enum_members_from_another_header() {
    helpers::ensure_libclang();

    let scratch = std::env::temp_dir().join(format!(
        "windows-clang-cross-header-enum-overrides-{}",
        std::process::id()
    ));
    std::fs::create_dir_all(&scratch).unwrap();
    let enum_header = scratch.join("enum.hpp");
    let macro_header = scratch.join("macro.hpp");
    std::fs::write(
        &enum_header,
        "typedef enum VALUE { SharedValue = 1 } VALUE;\n",
    )
    .unwrap();
    std::fs::write(&macro_header, "#define SharedValue 2\n").unwrap();
    let include = format!("-I{}", scratch.display());
    let snapshot = extract(
        [Input::new(
            scratch.join("input.hpp").to_string_lossy(),
            "#include \"enum.hpp\"\n#include \"macro.hpp\"\n",
        )
        .with_roots([
            enum_header.to_string_lossy(),
            macro_header.to_string_lossy(),
        ])],
        &["-x", "c++", &include],
    )
    .unwrap();
    let rdl = snapshot.emit("Constants").unwrap();

    assert!(rdl.contains("SharedValue = 1"), "{rdl}");
    assert!(rdl.contains("const SharedValue: i32 = 2"), "{rdl}");

    std::fs::remove_file(enum_header).unwrap();
    std::fs::remove_file(macro_header).unwrap();
    std::fs::remove_dir(scratch).unwrap();
}

#[test]
fn property_key_macros_preserve_guid_and_identifier() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "property_keys.hpp",
            "struct PROPERTYKEY { int value; };\n\
             struct DEVPROPKEY { int value; };\n\
             #define DEFINE_PROPERTYKEY(name, ...)\n\
             #define DEFINE_DEVPROPKEY(name, ...)\n\
             DEFINE_PROPERTYKEY(PKEY_Test, 0x12345678, 0x1234, 0xabcd, 0x98, 0x76, \
                 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 4)\n\
             DEFINE_DEVPROPKEY(DEVPKEY_Test, 0x87654321, 0xabcd, 0x1234, 0x01, 0x23, \
                 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 7)\n",
        )],
        &["-x", "c++"],
    )
    .unwrap();
    let rdl = snapshot.emit("PropertyKeys").unwrap();

    assert!(rdl.contains(
        "#[guid(0x12345678_1234_abcd_9876_0123456789ab)]\n    const PKEY_Test: PROPERTYKEY = 4"
    ));
    assert!(rdl.contains(
        "#[guid(0x87654321_abcd_1234_0123_456789abcdef)]\n    const DEVPKEY_Test: DEVPROPKEY = 7"
    ));

    let excluded = BTreeSet::from(["PKEY_Test".to_string()]);
    let references = BTreeMap::new();
    let mut options = EmitOptions::new("PropertyKeys", &references);
    options.excluded_constants = Some(&excluded);
    let rdl = snapshot.emit_with_options(&options).unwrap();

    assert!(!rdl.contains("const PKEY_Test"));
    assert!(rdl.contains("const DEVPKEY_Test"));
}

#[test]
fn source_function_alias_preserves_export_link_name() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "function_alias.hpp",
            "#define PublicFunction ExportedFunction\n\
             extern \"C\" void PublicFunction();\n",
        )],
        &["-x", "c++"],
    )
    .unwrap();
    let function = snapshot
        .facts()
        .iter()
        .find(|fact| {
            fact.name == "PublicFunction" && matches!(fact.data, FactData::Function { .. })
        })
        .unwrap();
    let FactData::Function { link_name, .. } = &function.data else {
        panic!("PublicFunction is not a function");
    };

    assert_eq!(link_name, "ExportedFunction");
    let rdl = snapshot
        .emit_with_library("FunctionAlias", "test.dll")
        .unwrap();
    assert!(rdl.contains(
        "#[library(\"test.dll\", import = \"ExportedFunction\")]\n    \
         extern \"C\" fn PublicFunction()"
    ));
}

#[test]
fn pointer_valued_macros_preserve_their_named_type() {
    helpers::ensure_libclang();

    let source = "typedef void* HANDLE;\n\
             typedef const char* PCSTR;\n\
             typedef void (*CALLBACK)();\n\
             #define POSITIVE_HANDLE ((HANDLE)17)\n\
             #define NEGATIVE_HANDLE ((HANDLE)-1)\n\
             #define RESOURCE_ID ((PCSTR)23)\n\
             #define NESTED_NEGATIVE ((HANDLE)(__UINTPTR_TYPE__)((int)0x80000000))\n\
             #define INVALID_CALLBACK ((CALLBACK)-1)\n";

    for target in ["x86_64-pc-windows-msvc", "i686-pc-windows-msvc"] {
        let snapshot = extract(
            [Input::new("pointer_macros.hpp", source)],
            &["-x", "c++", "-target", target],
        )
        .unwrap();
        let rdl = snapshot.emit("Constants").unwrap();

        assert!(rdl.contains("const POSITIVE_HANDLE: HANDLE = 17"), "{rdl}");
        assert!(rdl.contains("const NEGATIVE_HANDLE: HANDLE = -1"), "{rdl}");
        assert!(rdl.contains("const RESOURCE_ID: PCSTR = 23"), "{rdl}");
        assert!(
            rdl.contains("const NESTED_NEGATIVE: HANDLE = -2147483648"),
            "{rdl}"
        );
        assert!(!rdl.contains("const INVALID_CALLBACK"), "{rdl}");
    }
}

#[test]
fn external_scalar_alias_does_not_rename_the_scalar() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "external-scalar-alias.hpp",
            "typedef unsigned long long ULONGLONG;\n\
             typedef ULONGLONG TRACEHANDLE;\n\
             typedef unsigned int DWORD;\n\
             typedef DWORD CLS_CONTAINER_STATE;\n\
             typedef unsigned short WCHAR;\n\
             typedef WCHAR SEC_WCHAR;\n\
             typedef long long LONGLONG;\n\
             typedef LONGLONG USN;\n\
             typedef long LONG_PTR;\n\
             typedef LONG_PTR RTL_REFERENCE_COUNT;\n\
             struct LOCAL {\n\
                 ULONGLONG handle;\n\
                 DWORD state;\n\
                 WCHAR character;\n\
                 LONGLONG sequence;\n\
                 LONG_PTR workspace;\n\
             };\n",
        )],
        &["-x", "c++"],
    )
    .unwrap();
    let references = BTreeMap::from([
        (
            "TRACEHANDLE".to_string(),
            TypeReference::new("Windows.Win32", "TRACEHANDLE", TypeReferenceKind::Type),
        ),
        (
            "CLS_CONTAINER_STATE".to_string(),
            TypeReference::new(
                "Windows.Win32",
                "CLS_CONTAINER_STATE",
                TypeReferenceKind::Type,
            ),
        ),
        (
            "SEC_WCHAR".to_string(),
            TypeReference::new("Windows.Win32", "SEC_WCHAR", TypeReferenceKind::Type),
        ),
        (
            "USN".to_string(),
            TypeReference::new("Windows.Win32", "USN", TypeReferenceKind::Type),
        ),
        (
            "RTL_REFERENCE_COUNT".to_string(),
            TypeReference::new(
                "Windows.Win32",
                "RTL_REFERENCE_COUNT",
                TypeReferenceKind::Type,
            ),
        ),
    ]);
    let excluded = references.keys().cloned().collect();
    let mut options = EmitOptions::new("Windows.Win32", &references);
    options.excluded_types = Some(&excluded);
    let rdl = snapshot.emit_with_options(&options).unwrap();

    assert!(rdl.contains("handle: u64"), "{rdl}");
    assert!(rdl.contains("state: u32"), "{rdl}");
    assert!(rdl.contains("character: u16"), "{rdl}");
    assert!(rdl.contains("sequence: i64"), "{rdl}");
    assert!(rdl.contains("workspace: isize"), "{rdl}");
    for alias in references.keys() {
        assert!(!rdl.contains(&format!(": {alias}")), "{rdl}");
    }
}

#[test]
fn explicitly_aligned_fields_preserve_record_layout() {
    helpers::ensure_libclang();

    let rdl = extract(
        [Input::new(
            "aligned-fields.hpp",
            "typedef struct ALIGNED_FIELDS {\n\
                 unsigned int first;\n\
                 void* pointer;\n\
                 __declspec(align(8)) unsigned int count;\n\
                 __declspec(align(8)) unsigned char mode;\n\
                 __declspec(align(8)) void* next;\n\
             } ALIGNED_FIELDS;\n",
        )],
        &[
            "-x",
            "c++",
            "-fms-extensions",
            "--target=i686-pc-windows-msvc",
        ],
    )
    .unwrap()
    .emit("Records")
    .unwrap();

    assert!(rdl.contains("#[align(8)]\n    struct ALIGNED_FIELDS"));
    assert!(rdl.contains("count: u32"));
    assert!(rdl.contains("mode: u8"));
}

#[test]
fn local_alias_does_not_rename_external_type() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "external_alias.hpp",
            "typedef unsigned char BOOLEAN;\n\
             typedef BOOLEAN SECURITY_CONTEXT_TRACKING_MODE;\n\
             typedef unsigned long ULONG;\n\
             typedef ULONG *PULONG;\n\
             typedef PULONG PLCID;\n\
             struct STATE { BOOLEAN enabled; PULONG count; };\n\
             void UseCount(PULONG value);\n",
        )],
        &["-x", "c++"],
    )
    .unwrap();
    let references = BTreeMap::from([
        (
            "BOOLEAN".to_string(),
            TypeReference::new("Windows.Win32", "BOOLEAN", TypeReferenceKind::Type),
        ),
        (
            "PULONG".to_string(),
            TypeReference::new("Windows.Win32", "PULONG", TypeReferenceKind::Type),
        ),
    ]);
    let excluded = BTreeSet::from([
        "PLCID".to_string(),
        "SECURITY_CONTEXT_TRACKING_MODE".to_string(),
    ]);
    let mut options = EmitOptions::new("ExternalAlias", &references);
    options.library = Some("api.dll");
    options.excluded_types = Some(&excluded);
    let rdl = snapshot.emit_with_options(&options).unwrap();

    assert!(rdl.contains("enabled: Windows::Win32::BOOLEAN"), "{rdl}");
    assert!(rdl.contains("count: Windows::Win32::PULONG"), "{rdl}");
    assert!(
        rdl.contains("UseCount(value: Windows::Win32::PULONG)"),
        "{rdl}"
    );
    assert!(
        !rdl.contains("enabled: SECURITY_CONTEXT_TRACKING_MODE"),
        "{rdl}"
    );
    assert!(!rdl.contains("count: PLCID"), "{rdl}");
}

#[test]
fn included_headers_can_be_declaration_roots() {
    helpers::ensure_libclang();

    let scratch = std::env::temp_dir().join(format!(
        "windows-clang-included-root-{}",
        std::process::id()
    ));
    std::fs::create_dir_all(&scratch).unwrap();
    let dependency = scratch.join("dependency.hpp");
    let header = scratch.join("api.hpp");
    std::fs::write(
        &dependency,
        "#define VALUE_COUNT 4\n\
         typedef struct VALUE { int value; } VALUE;\n",
    )
    .unwrap();
    std::fs::write(
        &header,
        "#include \"dependency.hpp\"\nvoid UseValue(VALUE value);\n",
    )
    .unwrap();
    let include = format!("-I{}", scratch.display());
    let snapshot = extract(
        [Input::new(
            scratch.join("combined.hpp").to_string_lossy(),
            "#include \"api.hpp\"\n",
        )
        .with_roots([dependency.to_string_lossy(), header.to_string_lossy()])],
        &["-x", "c++", &include],
    )
    .unwrap();
    let rdl = snapshot.emit_with_library("Included", "api.dll").unwrap();

    assert!(rdl.contains("struct VALUE"));
    assert!(rdl.contains("const VALUE_COUNT: i32 = 4"));
    assert!(rdl.contains("fn UseValue(value: VALUE)"));

    let references = BTreeMap::new();
    let mut options = EmitOptions::new("Included", &references);
    options.library = Some("api.dll");
    let partitions = snapshot.emit_by_header_with_options(&options).unwrap();
    let dependency_rdl = &partitions[&dependency.to_string_lossy().replace('\\', "/")];
    let header_rdl = &partitions[&header.to_string_lossy().replace('\\', "/")];
    assert!(dependency_rdl.contains("struct VALUE"));
    assert!(dependency_rdl.contains("const VALUE_COUNT: i32 = 4"));
    assert!(!dependency_rdl.contains("fn UseValue"));
    assert!(header_rdl.contains("fn UseValue(value: VALUE)"));
    assert!(!header_rdl.contains("struct VALUE"));

    let partitioned_dir = scratch.join("partitioned");
    let combined_dir = scratch.join("combined");
    std::fs::create_dir_all(&partitioned_dir).unwrap();
    std::fs::create_dir_all(&combined_dir).unwrap();
    let output = partitioned_dir.join("Included.winmd");
    windows_rdl::reader()
        .input_texts(partitions.values())
        .output(&output)
        .write()
        .unwrap();
    let combined_output = combined_dir.join("Included.winmd");
    windows_rdl::reader()
        .input_text(&rdl)
        .output(&combined_output)
        .write()
        .unwrap();
    assert_eq!(
        std::fs::read(&output).unwrap(),
        std::fs::read(&combined_output).unwrap()
    );
    std::fs::remove_file(output).unwrap();
    std::fs::remove_file(combined_output).unwrap();
    std::fs::remove_dir(partitioned_dir).unwrap();
    std::fs::remove_dir(combined_dir).unwrap();
    std::fs::remove_file(dependency).unwrap();
    std::fs::remove_file(header).unwrap();
    std::fs::remove_dir(scratch).unwrap();
}

#[test]
fn equivalent_types_keep_source_order_ownership() {
    helpers::ensure_libclang();

    let scratch =
        std::env::temp_dir().join(format!("windows-clang-owner-order-{}", std::process::id()));
    std::fs::create_dir_all(&scratch).unwrap();
    let canonical = scratch.join("zcanonical.hpp");
    let duplicate = scratch.join("aduplicate.hpp");
    let api = scratch.join("api.hpp");
    std::fs::write(&canonical, "typedef unsigned VALUE;\n").unwrap();
    std::fs::write(&duplicate, "typedef unsigned VALUE;\n").unwrap();
    std::fs::write(
        &api,
        "#include \"zcanonical.hpp\"\n\
         #include \"aduplicate.hpp\"\n\
         extern \"C\" void UseValue(VALUE value);\n",
    )
    .unwrap();
    let include = format!("-I{}", scratch.display());
    let snapshot = extract(
        [Input::new(
            scratch.join("combined.hpp").to_string_lossy(),
            "#include \"api.hpp\"\n",
        )
        .with_roots([
            canonical.to_string_lossy(),
            duplicate.to_string_lossy(),
            api.to_string_lossy(),
        ])],
        &["-x", "c++", &include],
    )
    .unwrap();
    let references = BTreeMap::new();
    let mut options = EmitOptions::new("Ownership", &references);
    options.library = Some("api.dll");
    let partitions = snapshot.emit_by_header_with_options(&options).unwrap();

    assert!(
        partitions[&canonical.to_string_lossy().replace('\\', "/")].contains("type VALUE = u32")
    );
    assert!(
        partitions
            .get(&duplicate.to_string_lossy().replace('\\', "/"))
            .is_none_or(|rdl| !rdl.contains("type VALUE"))
    );

    std::fs::remove_dir_all(scratch).unwrap();
}

#[test]
fn real_win32_header_plans_from_combined_translation_unit() {
    helpers::ensure_libclang();

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
        [Input::new("clang-win32-slice.hpp", source).with_roots([header.to_string_lossy()])],
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
    helpers::ensure_libclang();

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
    let FactData::Record { base, fields, .. } = &derived.data else {
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
    helpers::ensure_libclang();

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
    let FactData::Record { fields, .. } = &result.data else {
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
fn flat_projection_ignores_helpers_and_resolves_winrt_abi_types() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "namespaces.hpp",
            "namespace Helpers { struct Point { int helper; }; }\n\
             namespace ABI { namespace Windows { namespace Foundation {\n\
                 struct Point { float X; float Y; };\n\
                 struct Interop { int value; };\n\
             } } }\n\
             namespace Windows { namespace Graphics {\n\
                 struct Win32Interop { int value; };\n\
             } }\n\
             struct Global { int value; };\n",
        )],
        &["-x", "c++"],
    )
    .unwrap();
    let references = BTreeMap::from([(
        "Point".to_string(),
        TypeReference::new("Windows.Foundation", "Point", TypeReferenceKind::Type),
    )]);
    let options = EmitOptions::new("Namespaced", &references);
    let rdl = snapshot.emit_with_options(&options).unwrap();

    assert!(!rdl.contains("struct Point"));
    assert!(rdl.contains("struct Interop"));
    assert!(rdl.contains("struct Win32Interop"));
    assert!(rdl.contains("struct Global"));
}

#[test]
fn x86_external_function_link_names_drop_abi_decoration() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "x86-link-names.hpp",
            "extern \"C\" int __stdcall SystemCall(int value);\n\
             extern \"C\" int __cdecl CCall(int value);\n",
        )],
        &["-x", "c++", "--target=i686-pc-windows-msvc"],
    )
    .unwrap();

    for name in ["SystemCall", "CCall"] {
        let function = snapshot
            .facts()
            .iter()
            .find(|fact| fact.name == name)
            .unwrap();
        let FactData::Function { link_name, .. } = &function.data else {
            panic!("{name} is not a function");
        };
        assert_eq!(link_name, name);
    }
}

#[test]
fn winrt_generic_typedef_preserves_closed_arguments() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "winrt-generic.hpp",
            "struct HSTRING__;\n\
             typedef HSTRING__* HSTRING;\n\
             struct IInspectable;\n\
             template<typename K, typename V> struct IMapView {};\n\
             typedef IMapView<HSTRING, IInspectable*> MAP;\n\
             extern \"C\" void GetMap(MAP* value);\n",
        )],
        &["-x", "c++"],
    )
    .unwrap();
    let rdl = snapshot
        .emit_with_library("WinrtGeneric", "api.dll")
        .unwrap();

    assert!(rdl.contains("type MAP = IMapView<String, Object>"), "{rdl}");
    assert!(rdl.contains("GetMap(value: *mut MAP)"), "{rdl}");
}

#[test]
fn closed_generic_interface_typedef_has_one_abi_pointer() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "winrt-generic-interface.hpp",
            "struct HSTRING__;\n\
             typedef HSTRING__* HSTRING;\n\
             struct IInspectable;\n\
             template<typename K, typename V> struct IMapView {};\n\
             typedef IMapView<HSTRING, IInspectable*> MAP;\n\
             extern \"C\" void GetMap(MAP** value);\n",
        )],
        &["-x", "c++"],
    )
    .unwrap();
    let references = BTreeMap::from([(
        "IMapView".to_string(),
        TypeReference::new(
            "Windows.Foundation.Collections",
            "IMapView",
            TypeReferenceKind::Interface,
        ),
    )]);
    let mut options = EmitOptions::new("WinrtGenericInterface", &references);
    options.library = Some("api.dll");
    let rdl = snapshot.emit_with_options(&options).unwrap();

    assert!(
        rdl.contains("type MAP = Windows::Foundation::Collections::IMapView<String, Object>"),
        "{rdl}"
    );
    assert!(rdl.contains("GetMap(value: *mut MAP)"), "{rdl}");
}

#[test]
fn uuid_class_projects_to_coclass_guid() {
    helpers::ensure_libclang();

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

    let excluded = BTreeSet::from(["Widget".to_string()]);
    let references = BTreeMap::new();
    let mut options = EmitOptions::new("Coclass", &references);
    options.excluded_constants = Some(&excluded);
    let rdl = snapshot.emit_with_options(&options).unwrap();

    assert!(!rdl.contains("Widget"));
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

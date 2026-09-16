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
fn uuid_struct_projects_to_guid() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "uuid-struct.hpp",
            "struct __declspec(uuid(\"12345678-1234-5678-90ab-cdef12345678\")) PROPERTY;\n",
        )],
        &["-x", "c++", "-fms-extensions"],
    )
    .unwrap();
    let rdl = snapshot.emit("Guids").unwrap();

    assert!(rdl.contains("const PROPERTY: GUID = 0x12345678_1234_5678_90ab_cdef12345678"));
    assert!(!rdl.contains("struct PROPERTY"));
}

#[test]
fn matching_uuid_structs_collapse_across_translation_units() {
    helpers::ensure_libclang();

    let source = "struct __declspec(uuid(\"12345678-1234-5678-90ab-cdef12345678\")) PROPERTY;\n";
    let snapshot = extract(
        [
            Input::new("first.hpp", source),
            Input::new("second.hpp", source),
        ],
        &["-x", "c++", "-fms-extensions"],
    )
    .unwrap();
    let rdl = snapshot.emit("Guids").unwrap();

    assert_eq!(rdl.matches("const PROPERTY: GUID").count(), 1);
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
fn string_alias_projection_uses_direction_once() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "string-aliases.hpp",
            "#define IN __attribute__((annotate(\"_In_\")))\n\
             #define OUT __attribute__((annotate(\"_Out_\")))\n\
             typedef const char* PCSTR;\n\
             typedef char* LPSTR;\n\
             typedef char* PSTR;\n\
             typedef const unsigned short* PCWSTR;\n\
             typedef unsigned short* LPWSTR;\n\
             typedef unsigned short* PWSTR;\n\
             extern \"C\" void Strings(IN LPSTR legacy_input, OUT LPSTR legacy_output, \
                 IN PSTR explicit_input, IN LPWSTR wide_input, OUT PWSTR wide_output);\n",
        )],
        &["-x", "c++"],
    )
    .unwrap();
    let rdl = snapshot
        .emit_with_library("StringAliases", "api.dll")
        .unwrap();

    assert!(
        rdl.contains(
            "Strings(legacy_input: PCSTR, #[out] legacy_output: PSTR, \
             explicit_input: PSTR, wide_input: PCWSTR, #[out] wide_output: PWSTR)"
        ),
        "{rdl}"
    );
}

#[test]
fn unnamed_record_type_in_named_field_is_inline() {
    helpers::ensure_libclang();

    let rdl = extract(
        [Input::new(
            "named-field.hpp",
            "typedef struct HOLDER {\n\
                 union { int integer; unsigned int unsigned_integer; } Parameters;\n\
             } HOLDER;\n",
        )],
        &["-x", "c++"],
    )
    .unwrap()
    .emit("Records")
    .unwrap();

    assert!(rdl.contains("Parameters: union"));
    assert!(rdl.contains("integer: i32"));
    assert!(rdl.contains("unsigned_integer: u32"));
}

#[test]
fn enum_flag_macro_controls_projection() {
    helpers::ensure_libclang();

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
}

#[test]
fn matching_constant_alias_types_collapse_across_translation_units() {
    helpers::ensure_libclang();

    let first = Input::new(
        "first.hpp",
        "typedef unsigned int DWORD;\n#define SHARED_VALUE ((DWORD)8)\n",
    );
    let second = Input::new(
        "second.hpp",
        "typedef unsigned int ULONG;\n#define SHARED_VALUE ((ULONG)8)\n",
    );
    let rdl = extract([first, second], &["-x", "c++"])
        .unwrap()
        .emit("Constants")
        .unwrap();

    assert_eq!(rdl.matches("const SHARED_VALUE").count(), 1);
    assert!(rdl.contains("const SHARED_VALUE: u32 = 8"));
}

#[test]
fn macro_overrides_replace_same_named_enum_members() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "enum-overrides.hpp",
            "typedef enum _POOL_TYPE {\n\
                 NonPagedPool = 0,\n\
                 NonPagedPoolCacheAligned = 4,\n\
                 NonPagedPoolNx = 512,\n\
                 NonPagedPoolNxCacheAligned = 516,\n\
             } POOL_TYPE;\n\
             #define NonPagedPool NonPagedPoolNx\n\
             #define NonPagedPoolCacheAligned ((POOL_TYPE)NonPagedPoolNxCacheAligned)\n",
        )],
        &["-x", "c++"],
    )
    .unwrap();
    let rdl = snapshot.emit("Constants").unwrap();

    assert!(rdl.contains("NonPagedPool = 512"), "{rdl}");
    assert!(rdl.contains("NonPagedPoolCacheAligned = 516"), "{rdl}");
    assert!(!rdl.contains("const NonPagedPool:"), "{rdl}");
    assert!(!rdl.contains("const NonPagedPoolCacheAligned:"), "{rdl}");
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
fn string_macros_preserve_encoding_aliases_and_concatenation() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "strings.hpp",
            "#define ANSI_TEXT \"hello\" \"\\\\nworld\"\n\
            #define UTF8_BYTES \"\\xC3\\xA9\"\n\
            #define INVALID_BYTES \"\\xFF\"\n\
            #define WIDE_TEXT L\"wide\"\n\
            #define WIDE_ALIAS WIDE_TEXT\n",
        )],
        &["-x", "c++"],
    )
    .unwrap();
    let rdl = snapshot.emit("Strings").unwrap();

    assert!(
        rdl.contains("#[encoding(\"ansi\")]\n    const ANSI_TEXT: String = \"hello\\\\nworld\"")
    );
    assert!(rdl.contains("#[encoding(\"ansi\")]\n    const UTF8_BYTES: String = \"é\""));
    assert!(!rdl.contains("INVALID_BYTES"));
    assert!(rdl.contains("#[encoding(\"utf-16\")]\n    const WIDE_TEXT: String = \"wide\""));
    assert!(rdl.contains("#[encoding(\"utf-16\")]\n    const WIDE_ALIAS: String = \"wide\""));
}

#[test]
fn object_macros_resolve_cast_type_aliases_without_guessing() {
    helpers::ensure_libclang();

    let rdl = extract(
        [Input::new(
            "cast-aliases.hpp",
            "typedef unsigned int REAL_TYPE;\n\
             #define CAST_TYPE REAL_TYPE\n\
             #define CHAIN_TYPE CAST_TYPE\n\
             #define ONE_HOP ((CAST_TYPE)1)\n\
             #define CHAINED ((CHAIN_TYPE)2)\n\
             #define CYCLE_A CYCLE_B\n\
             #define CYCLE_B CYCLE_A\n\
             #define CYCLIC ((CYCLE_A)3)\n\
             #define COLLISION REAL_TYPE\n\
             #undef COLLISION\n\
             #define COLLISION(value) value\n\
             #define FUNCTION_LIKE_COLLISION ((COLLISION)4)\n",
        )],
        &["-x", "c++"],
    )
    .unwrap()
    .emit("CastAliases")
    .unwrap();

    assert!(rdl.contains("const ONE_HOP: REAL_TYPE = 1"), "{rdl}");
    assert!(rdl.contains("const CHAINED: REAL_TYPE = 2"), "{rdl}");
    assert!(!rdl.contains("const CYCLIC"), "{rdl}");
    assert!(!rdl.contains("const FUNCTION_LIKE_COLLISION"), "{rdl}");
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
}

#[test]
fn file_scope_float_constants_are_emitted() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "floats.hpp",
            "const float FLOAT_VALUE = 1.25f;\nconst double DOUBLE_VALUE = 2.0;",
        )],
        &["-x", "c++"],
    )
    .unwrap();
    let rdl = snapshot.emit("Floats").unwrap();

    assert!(rdl.contains("const FLOAT_VALUE: f32 = 1.25"));
    assert!(rdl.contains("const DOUBLE_VALUE: f64 = 2.0"));
}

#[test]
fn floating_point_macros_are_evaluated() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "float_macros.hpp",
            "#define FLOAT_VALUE 1.25f\n#define DOUBLE_VALUE (2.0)",
        )],
        &["-x", "c++"],
    )
    .unwrap();
    let rdl = snapshot.emit("FloatMacros").unwrap();

    assert!(rdl.contains("const FLOAT_VALUE: f32 = 1.25"));
    assert!(rdl.contains("const DOUBLE_VALUE: f64 = 2.0"));
}

#[test]
fn character_macros_are_evaluated() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "character_macros.hpp",
            "#define QUOTE '\"'\n#define MULTI_CHARACTER 'draH'",
        )],
        &["-x", "c++"],
    )
    .unwrap();
    let rdl = snapshot.emit("CharacterMacros").unwrap();

    assert!(rdl.contains("const QUOTE: i8 = 34"), "{rdl}");
    assert!(
        rdl.contains("const MULTI_CHARACTER: i32 = 1685217608"),
        "{rdl}"
    );
}

#[test]
fn constant_byte_sizes_are_converted_to_element_counts() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "constant_byte_sizes.hpp",
            "#define READS_BYTES(c) __attribute__((annotate(\"_In_reads_bytes_(\" #c \")\")))\n\
             extern \"C\" void ReadBytes(READS_BYTES(128) char* bytes);\n\
             extern \"C\" void ReadWide(READS_BYTES(8) wchar_t* text);\n",
        )],
        &["-x", "c++"],
    )
    .unwrap();
    let rdl = snapshot
        .emit_with_library("ConstantByteSizes", "test.dll")
        .unwrap();

    assert!(
        rdl.contains("#[len_const(128)] #[in] bytes: *mut i8"),
        "{rdl}"
    );
    assert!(
        rdl.contains("#[len_const(4)] #[in] text: *mut u16"),
        "{rdl}"
    );
}

#[test]
fn unresolved_sal_sizes_do_not_drop_functions() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "unresolved_sal.hpp",
            "#define WRITES(c) __attribute__((annotate(\"_Out_writes_(\" #c \")\")))\n\
             extern \"C\" void Write(unsigned targetIdCount, \
                 WRITES(targetCount) int* values);\n",
        )],
        &["-x", "c++"],
    )
    .unwrap();
    let rdl = snapshot
        .emit_with_library("UnresolvedSal", "test.dll")
        .unwrap();

    assert!(rdl.contains("fn Write(targetIdCount: u32, values: *mut i32)"));
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
fn source_method_alias_preserves_declared_name() {
    helpers::ensure_libclang();

    let source = "#define PublicMethod ExpandedMethod\n\
         #define DECLARE_METHOD(name) virtual void name() = 0\n\
         struct __declspec(uuid(\"12345678-1234-abcd-9876-0123456789ab\")) ITest {\n\
             DECLARE_METHOD(PublicMethod);\n\
         };\n";
    let path = std::env::temp_dir().join(format!(
        "windows-clang-method-alias-{}.hpp",
        std::process::id()
    ));
    std::fs::write(&path, source).unwrap();
    let snapshot = extract(
        [Input::new(path.to_string_lossy(), source)],
        &["-x", "c++", "-fms-extensions"],
    )
    .unwrap();
    std::fs::remove_file(path).unwrap();
    let rdl = snapshot.emit("MethodAlias").unwrap();

    assert!(rdl.contains("fn PublicMethod(&self)"), "{rdl}");
    assert!(!rdl.contains("fn ExpandedMethod(&self)"), "{rdl}");
}

#[test]
fn const_typedef_does_not_rename_interface() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "const_interface_alias.hpp",
            "struct __declspec(uuid(\"12345678-1234-abcd-9876-0123456789ab\")) ITest {\n\
                 virtual int Method() = 0;\n\
             };\n\
             typedef const ITest CTest;\n",
        )],
        &["-x", "c++", "-fms-extensions"],
    )
    .unwrap();
    let rdl = snapshot.emit("InterfaceAlias").unwrap();

    assert!(rdl.contains("interface ITest"), "{rdl}");
    assert!(!rdl.contains("interface CTest"), "{rdl}");
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
fn interface_iid_macro_is_not_emitted_twice() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "iid.hpp",
            "#define DEFINE_GUID(name, ...)\n\
             struct __declspec(uuid(\"12345678-1234-abcd-9876-0123456789ab\")) ITest {\n\
                 virtual int Method() = 0;\n\
             };\n\
             DEFINE_GUID(IID_ITest, 0x12345678, 0x1234, 0xabcd, 0x98, 0x76, 0x01, \
                 0x23, 0x45, 0x67, 0x89, 0xab)\n",
        )],
        &["-x", "c++", "-fms-extensions"],
    )
    .unwrap();
    let references = BTreeMap::from([(
        "IUnknown".to_string(),
        TypeReference::new("External", "IUnknown", TypeReferenceKind::Interface),
    )]);
    let rdl = snapshot
        .emit_with_options(&EmitOptions::new("Iids", &references))
        .unwrap();

    assert!(rdl.contains("interface ITest"));
    assert!(!rdl.contains("const IID_ITest"));
}

#[test]
fn separate_interface_iid_macro_attaches_to_interface() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "iid.hpp",
            "#define DEFINE_GUID(name, ...)\n\
             struct ITest {\n\
                 virtual int Method() = 0;\n\
             };\n\
             DEFINE_GUID(IID_ITest, 0x12345678, 0x1234, 0xabcd, 0x98, 0x76, 0x01, \
                 0x23, 0x45, 0x67, 0x89, 0xab)\n",
        )],
        &["-x", "c++", "-fms-extensions"],
    )
    .unwrap();
    let rdl = snapshot.emit("Iids").unwrap();

    assert!(rdl.contains("#[guid(0x12345678_1234_abcd_9876_0123456789ab)]\n    interface ITest"));
    assert!(!rdl.contains("const IID_ITest"));
}

#[test]
fn conflicting_interface_uuid_and_iid_are_both_preserved() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "iid.hpp",
            "#define DEFINE_GUID(name, ...)\n\
             struct __declspec(uuid(\"12345678-1234-abcd-9876-0123456789ab\")) ITest {\n\
                 virtual int Method() = 0;\n\
             };\n\
             DEFINE_GUID(IID_ITest, 0x87654321, 0x4321, 0xdcba, 0x67, 0x89, 0xfe, \
                 0xdc, 0xba, 0x98, 0x76, 0x54)\n",
        )],
        &["-x", "c++", "-fms-extensions"],
    )
    .unwrap();
    let rdl = snapshot.emit("Iids").unwrap();

    assert!(rdl.contains("#[guid(0x12345678_1234_abcd_9876_0123456789ab)]\n    interface ITest"));
    assert!(rdl.contains("const IID_ITest: GUID = 0x87654321_4321_dcba_6789_fedcba987654"));
}

#[test]
fn matching_guid_macros_collapse_across_translation_units() {
    helpers::ensure_libclang();

    let source = "#define DEFINE_GUID(name, ...)\n\
                  DEFINE_GUID(GUID_SHARED, 0x12345678, 0x1234, 0x5678, 0x90, 0xab, 0xcd, \
                  0xef, 0x12, 0x34, 0x56, 0x78)\n";
    let first = Input::new("first.hpp", source);
    let second = Input::new("second.hpp", source);
    let forward = extract([first.clone(), second.clone()], &["-x", "c++"])
        .unwrap()
        .emit("Guids")
        .unwrap();
    let reverse = extract([second, first], &["-x", "c++"])
        .unwrap()
        .emit("Guids")
        .unwrap();

    assert_eq!(forward, reverse);
    assert_eq!(forward.matches("const GUID_SHARED").count(), 1);
}

#[test]
fn matching_record_definitions_collapse_across_translation_units() {
    helpers::ensure_libclang();

    let source = "typedef unsigned char BYTE;\n\
                  typedef BYTE* PBYTE;\n\
                  typedef struct SHARED { PBYTE data; unsigned int size; } SHARED;\n";
    let first = Input::new("first.hpp", source);
    let second = Input::new("second.hpp", source);
    let forward = extract([first.clone(), second.clone()], &["-x", "c++"])
        .unwrap()
        .emit("Records")
        .unwrap();
    let reverse = extract([second, first], &["-x", "c++"])
        .unwrap()
        .emit("Records")
        .unwrap();

    assert_eq!(forward, reverse);
    assert_eq!(forward.matches("struct SHARED").count(), 1);

    let conflict = extract(
        [
            Input::new("first.hpp", "struct SHARED { unsigned int value; };"),
            Input::new("second.hpp", "struct SHARED { unsigned short value; };"),
        ],
        &["-x", "c++"],
    )
    .unwrap()
    .emit("Records")
    .unwrap_err();
    assert!(
        conflict
            .to_string()
            .contains("ambiguous type root `SHARED`")
    );
}

#[test]
fn conflicting_nested_aliases_do_not_collapse_by_name() {
    helpers::ensure_libclang();

    let first = Input::new(
        "first-public.hpp",
        "#line 1 \"first-private.hpp\"\n\
         typedef int INNER;\n\
         #line 1 \"first-public.hpp\"\n\
         typedef INNER OUTER;\n",
    );
    let second = Input::new(
        "second-public.hpp",
        "#line 1 \"second-private.hpp\"\n\
         typedef unsigned int INNER;\n\
         #line 1 \"second-public.hpp\"\n\
         typedef INNER OUTER;\n",
    );
    let error = extract([first, second], &["-x", "c++"])
        .unwrap()
        .emit("Aliases")
        .unwrap_err();

    assert!(
        error.to_string().contains("ambiguous type root `INNER`"),
        "{error}"
    );
}

#[test]
fn conflicting_nested_enum_aliases_do_not_collapse_by_name() {
    helpers::ensure_libclang();

    let first = Input::new(
        "first-enum.hpp",
        "enum INNER : int { First = 1 };\ntypedef INNER OUTER;\n",
    );
    let second = Input::new(
        "second-enum.hpp",
        "enum INNER : unsigned int { Second = 2 };\ntypedef INNER OUTER;\n",
    );
    let error = extract([first, second], &["-x", "c++"])
        .unwrap()
        .emit("EnumAliases")
        .unwrap_err();

    assert!(error.to_string().contains("ambiguous type root"));
}

#[test]
fn incompatible_forward_declaration_kinds_do_not_complete_each_other() {
    helpers::ensure_libclang();

    let first = Input::new(
        "forward-struct.hpp",
        "typedef struct INNER OUTER;\ntypedef OUTER* POUTER;\n",
    );
    let second = Input::new(
        "complete-union.hpp",
        "typedef union INNER { int value; } OUTER;\n",
    );
    let error = extract([first, second], &["-x", "c++"])
        .unwrap()
        .emit("ForwardKinds")
        .unwrap_err();

    assert!(error.to_string().contains("ambiguous type root"));
}

#[test]
fn complete_typedef_target_wins_over_cross_tu_forward_declaration() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [
            Input::new(
                "forward.hpp",
                "typedef struct _SHARED SHARED;\ntypedef SHARED* PSHARED;\n",
            ),
            Input::new(
                "complete.hpp",
                "typedef struct _SHARED { unsigned int value; } SHARED;\n\
                 typedef SHARED* PSHARED;\n",
            ),
        ],
        &["-x", "c++"],
    )
    .unwrap();
    let rdl = snapshot.emit_with_library("Records", "test.dll").unwrap();

    assert_eq!(rdl.matches("struct SHARED").count(), 1);
    assert!(rdl.contains("value: u32"));
    assert!(rdl.contains("type PSHARED = *mut SHARED"));
}

#[test]
fn complete_same_name_typedef_target_wins_over_cross_tu_forward_declaration() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [
            Input::new(
                "forward.hpp",
                "typedef struct SHARED SHARED;\n\
                 typedef SHARED* PSHARED;\n\
                 extern \"C\" void UseShared(SHARED value);\n",
            ),
            Input::new(
                "complete.hpp",
                "typedef struct SHARED { unsigned int value; } SHARED;\n",
            ),
        ],
        &["-x", "c++"],
    )
    .unwrap();
    let rdl = snapshot.emit_with_library("Records", "test.dll").unwrap();

    assert_eq!(rdl.matches("struct SHARED").count(), 1);
    assert!(rdl.contains("value: u32"));
    assert!(rdl.contains("type PSHARED = *mut SHARED"));
    assert!(rdl.contains("fn UseShared(value: SHARED)"));
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
fn recursive_cross_tu_shapes_are_input_order_independent() {
    helpers::ensure_libclang();

    let first = Input::new(
        "recursive-first.hpp",
        "typedef struct _LEFT LEFT;\n\
         typedef struct _RIGHT RIGHT;\n\
         struct _LEFT { RIGHT* right; };\n\
         struct _RIGHT { LEFT* left; };\n",
    );
    let second = Input::new(
        "recursive-second.hpp",
        "typedef struct _RIGHT RIGHT;\n\
         typedef struct _LEFT LEFT;\n\
         struct _RIGHT { LEFT* left; };\n\
         struct _LEFT { RIGHT* right; };\n",
    );

    let forward = extract([first.clone(), second.clone()], &["-x", "c++"])
        .unwrap()
        .emit("Recursive")
        .unwrap();
    let reverse = extract([second, first], &["-x", "c++"])
        .unwrap()
        .emit("Recursive")
        .unwrap();

    assert_eq!(forward, reverse);
}

#[test]
fn incomplete_same_name_typedef_emits_opaque_record() {
    helpers::ensure_libclang();

    let rdl = extract(
        [Input::new(
            "opaque.hpp",
            "typedef struct OPAQUE OPAQUE;\n\
             struct OPAQUE;\n\
             extern \"C\" void Use(const OPAQUE* value);\n",
        )],
        &["-x", "c++"],
    )
    .unwrap()
    .emit_with_library("Records", "test.dll")
    .unwrap();

    assert!(rdl.contains("struct OPAQUE {\n"));
    assert!(rdl.contains("fn Use(value: *const OPAQUE)"));
}

#[test]
fn incomplete_record_used_by_value_is_rejected() {
    helpers::ensure_libclang();

    let error = extract(
        [Input::new(
            "incomplete-by-value.hpp",
            "typedef struct OPAQUE OPAQUE;\n\
             extern \"C\" void Use(OPAQUE value);\n",
        )],
        &["-x", "c++"],
    )
    .unwrap()
    .emit_with_library("Records", "test.dll")
    .unwrap_err();

    assert!(
        error
            .to_string()
            .contains("incomplete record `OPAQUE` is used by value"),
        "{error}"
    );
}

#[test]
fn x86_flexible_only_union_preserves_clang_layout() {
    helpers::ensure_libclang();

    let rdl = extract(
        [Input::new(
            "flexible-union.hpp",
            "typedef union FLEXIBLE_UNION {\n\
                 int first[];\n\
                 short second[];\n\
             } FLEXIBLE_UNION;\n",
        )],
        &["-x", "c++", "--target=i686-pc-windows-msvc"],
    )
    .unwrap()
    .emit("Records")
    .unwrap();

    assert!(rdl.contains("union FLEXIBLE_UNION"));
    assert!(rdl.contains("first: [i32; 0]"));
    assert!(rdl.contains("second: [i16; 0]"));
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
fn cpp_implementation_class_pointers_are_opaque() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "opaque-class.hpp",
            "class Implementation { public: virtual ~Implementation() {} int value; };\n\
             extern \"C\" void UseImplementation(Implementation* value);\n",
        )],
        &["-x", "c++"],
    )
    .unwrap();
    let rdl = snapshot
        .emit_with_library("OpaqueClass", "api.dll")
        .unwrap();

    assert!(rdl.contains("fn UseImplementation(value: *mut void)"));
    assert!(!rdl.contains("struct Implementation"));
    assert!(!rdl.contains("interface Implementation"));
}

#[test]
fn token_pasted_record_typedef_preserves_layout() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "opaque-key.hpp",
            "#define DECLARE_KEY(name) typedef struct name##__ { long long Internal; } name\n\
             DECLARE_KEY(CONNECTION_KEY);\n",
        )],
        &["-x", "c++"],
    )
    .unwrap();
    let rdl = snapshot.emit("OpaqueKey").unwrap();

    assert!(rdl.contains("struct CONNECTION_KEY"));
    assert!(rdl.contains("Internal: i64"));
}

#[test]
fn source_macro_preserves_calling_convention() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "calling.hpp",
            "#define WINAPI __stdcall\n\
             #define CALLBACK __stdcall\n\
             void WINAPI SystemCall();\n\
             void PlainCall();\n\
             typedef void CALLBACK CALLBACK_TYPE(void* context);\n\
             typedef CALLBACK_TYPE *PCALLBACK_TYPE;\n",
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
    assert!(rdl.contains("extern fn CALLBACK_TYPE(context: *mut void)"));
    assert!(
        rdl.contains("extern fn PCALLBACK_TYPE(context: *mut void)"),
        "{rdl}"
    );
}

#[test]
fn source_literal_preserves_calling_convention() {
    helpers::ensure_libclang();

    let rdl = extract(
        [Input::new(
            "literal-calling.hpp",
            "void __stdcall SystemCall();\n\
             void __cdecl CCall();\n",
        )],
        &[
            "-x",
            "c++",
            "--target=x86_64-pc-windows-msvc",
            "-fms-extensions",
        ],
    )
    .unwrap()
    .emit_with_library("Calling", "api.dll")
    .unwrap();

    assert!(rdl.contains("extern fn SystemCall()"), "{rdl}");
    assert!(rdl.contains("extern \"C\" fn CCall()"), "{rdl}");
}

#[test]
fn macro_redefinitions_use_the_definition_active_at_each_declaration() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "calling-redefined.hpp",
            "#define ABI __stdcall\n\
             void ABI First();\n\
             #undef ABI\n\
             #define ABI __cdecl\n\
             void ABI Second();\n",
        )],
        &[
            "-x",
            "c++",
            "--target=i686-pc-windows-msvc",
            "-fms-extensions",
        ],
    )
    .unwrap();
    let rdl = snapshot
        .emit_with_library("CallingRedefined", "api.dll")
        .unwrap();

    assert!(rdl.contains("extern fn First()"), "{rdl}");
    assert!(rdl.contains("extern \"C\" fn Second()"), "{rdl}");
}

#[test]
fn undefined_macro_is_not_reused_as_a_calling_convention() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "calling-undefined.hpp",
            "#define ABI __stdcall\n\
             void ABI First();\n\
             #undef ABI\n\
             typedef int ABI;\n\
             ABI Second();\n",
        )],
        &[
            "-x",
            "c++",
            "--target=i686-pc-windows-msvc",
            "-fms-extensions",
        ],
    )
    .unwrap();
    let rdl = snapshot
        .emit_with_library("CallingUndefined", "api.dll")
        .unwrap();

    assert!(rdl.contains("extern fn First()"), "{rdl}");
    assert!(rdl.contains("extern \"C\" fn Second() -> ABI"), "{rdl}");
}

#[test]
fn included_macro_definitions_are_resolved_at_each_declaration() {
    helpers::ensure_libclang();

    let scratch =
        std::env::temp_dir().join(format!("windows-clang-macro-order-{}", std::process::id()));
    std::fs::create_dir_all(&scratch).unwrap();
    std::fs::write(scratch.join("abi.h"), "#define ABI __stdcall\n").unwrap();
    let source = "#include \"abi.h\"\n\
                  void ABI First();\n\
                  #undef ABI\n\
                  #define ABI __cdecl\n\
                  void ABI Second();\n";
    let input = scratch.join("api.hpp");
    std::fs::write(&input, source).unwrap();
    let include = format!("-I{}", scratch.display());
    let snapshot = extract(
        [Input::new(input.to_string_lossy(), source)],
        &[
            "-x",
            "c++",
            "--target=i686-pc-windows-msvc",
            "-fms-extensions",
            &include,
        ],
    )
    .unwrap();
    std::fs::remove_dir_all(&scratch).unwrap();
    let rdl = snapshot
        .emit_with_library("IncludedCalling", "api.dll")
        .unwrap();

    assert!(rdl.contains("extern fn First()"), "{rdl}");
    assert!(rdl.contains("extern \"C\" fn Second()"), "{rdl}");
}

#[test]
fn nested_source_macro_preserves_callback_calling_convention() {
    helpers::ensure_libclang();

    let source = "#define CALLBACK __stdcall\n\
         #define DECLARE_CALLBACK(name) typedef void (CALLBACK *name)(void* context)\n\
         DECLARE_CALLBACK(CALLBACK_TYPE);\n";
    let path = std::env::temp_dir().join(format!(
        "windows-clang-callback-convention-{}.hpp",
        std::process::id()
    ));
    std::fs::write(&path, source).unwrap();
    let snapshot = extract(
        [Input::new(path.to_string_lossy(), source)],
        &[
            "-x",
            "c++",
            "--target=x86_64-pc-windows-msvc",
            "-fms-extensions",
        ],
    )
    .unwrap();
    std::fs::remove_file(path).unwrap();
    let rdl = snapshot.emit("CallbackConvention").unwrap();

    assert!(
        rdl.contains("extern fn CALLBACK_TYPE(context: *mut void)"),
        "{rdl}"
    );
}

#[test]
fn nested_macro_uses_the_definition_active_at_expansion() {
    helpers::ensure_libclang();

    let source = "#define ABI __stdcall\n\
         #define DECLARE_CALLBACK(name) typedef void (ABI *name)(void)\n\
         DECLARE_CALLBACK(FIRST_CALLBACK);\n\
         #undef ABI\n\
         #define ABI __cdecl\n\
         DECLARE_CALLBACK(SECOND_CALLBACK);\n";
    let snapshot = extract(
        [Input::new("nested-calling.hpp", source)],
        &[
            "-x",
            "c++",
            "--target=x86_64-pc-windows-msvc",
            "-fms-extensions",
        ],
    )
    .unwrap();
    let rdl = snapshot.emit("NestedCalling").unwrap();

    assert!(rdl.contains("extern fn FIRST_CALLBACK()"), "{rdl}");
    assert!(rdl.contains("extern \"C\" fn SECOND_CALLBACK()"), "{rdl}");
}

#[test]
fn pointer_typedef_publishes_private_opaque_tag() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "opaque_tag.hpp",
            "struct _THING;\n\
             typedef struct _THING *PTHING;\n\
             void UseThing(PTHING value);\n",
        )],
        &["-x", "c++"],
    )
    .unwrap();
    let rdl = snapshot.emit_with_library("OpaqueTag", "api.dll").unwrap();

    assert!(rdl.contains("struct THING {"), "{rdl}");
    assert!(rdl.contains("type PTHING = *mut THING"), "{rdl}");
}

#[test]
fn pointer_typedef_does_not_replace_distinct_public_type() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "pointer-name-collision.hpp",
            "struct _THING { int first; };\n\
             typedef _THING* PTHING;\n\
             struct THING { double second; };\n",
        )],
        &["-x", "c++"],
    )
    .unwrap();
    let rdl = snapshot.emit("PointerNameCollision").unwrap();

    assert!(rdl.contains("struct _THING"), "{rdl}");
    assert!(rdl.contains("type PTHING = *mut _THING"), "{rdl}");
    assert!(rdl.contains("struct THING"), "{rdl}");

    let incomplete = extract(
        [Input::new(
            "pointer-incomplete-collision.hpp",
            "struct _ITEM;\n\
             typedef _ITEM* PITEM;\n\
             struct ITEM;\n\
             extern \"C\" void UseItem(ITEM* value);\n",
        )],
        &["-x", "c++"],
    )
    .unwrap()
    .emit_with_library("PointerIncompleteCollision", "api.dll")
    .unwrap();
    assert!(incomplete.contains("struct _ITEM"), "{incomplete}");
    assert!(incomplete.contains("struct ITEM"), "{incomplete}");
    assert!(
        incomplete.contains("type PITEM = *mut _ITEM"),
        "{incomplete}"
    );

    let related = extract(
        [Input::new(
            "pointer-name-alias.hpp",
            "typedef struct _THING { int value; } THING, *PTHING;\n",
        )],
        &["-x", "c++"],
    )
    .unwrap()
    .emit("PointerNameAlias")
    .unwrap();
    assert!(related.contains("struct THING"), "{related}");
    assert!(related.contains("type PTHING = *mut THING"), "{related}");
}

#[test]
fn local_record_wins_over_canonical_bare_name() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "canonical-name-collision.hpp",
            "struct BYTE { int value; };\n\
             extern \"C\" void UseByte(BYTE value);\n",
        )],
        &["-x", "c++"],
    )
    .unwrap();
    let rdl = snapshot
        .emit_with_library("CanonicalNameCollision", "api.dll")
        .unwrap();

    assert!(rdl.contains("struct BYTE"), "{rdl}");
    assert!(rdl.contains("UseByte(value: BYTE)"), "{rdl}");
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
fn input_pointer_aliases_project_constness() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "input_pointer_aliases.hpp",
            "#define IN __attribute__((annotate(\"_In_\")))\n\
             #define RESERVED __attribute__((annotate(\"_Reserved_\")))\n\
             typedef void *PVOID;\n\
             typedef unsigned short OLECHAR;\n\
             struct TP_CALLBACK_ENVIRON_V3 {};\n\
             typedef TP_CALLBACK_ENVIRON_V3 *PTP_CALLBACK_ENVIRON;\n\
             void Submit(IN PVOID context, RESERVED PVOID reserved, \
                 IN PTP_CALLBACK_ENVIRON environment, IN OLECHAR **names);\n",
        )],
        &["-x", "c++"],
    )
    .unwrap();
    let rdl = snapshot
        .emit_with_library("InputPointerAliases", "api.dll")
        .unwrap();

    assert!(
        rdl.contains(
            "fn Submit(#[in] context: *mut void, #[reserved] reserved: *mut void, \
             environment: PTP_CALLBACK_ENVIRON, #[in] names: *mut *mut OLECHAR)"
        ),
        "{rdl}"
    );
}

#[test]
fn parameter_typedefs_are_not_expanded_by_name_prefix() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "parameter-typedefs.hpp",
            "typedef __int64 LONG_PTR;\n\
             typedef LONG_PTR LPARAM;\n\
             typedef long PROPERTYID;\n\
             extern \"C\" void UseValues(LPARAM value, PROPERTYID property);\n",
        )],
        &[
            "-x",
            "c++",
            "--target=x86_64-pc-windows-msvc",
            "-fms-extensions",
        ],
    )
    .unwrap();
    let rdl = snapshot
        .emit_with_library("ParameterTypedefs", "api.dll")
        .unwrap();

    assert!(rdl.contains("type LPARAM = isize"), "{rdl}");
    assert!(rdl.contains("type PROPERTYID = i32"), "{rdl}");
    assert!(
        rdl.contains("UseValues(value: LPARAM, property: PROPERTYID)"),
        "{rdl}"
    );
}

#[test]
fn sized_null_terminated_strings_keep_string_types() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "sized-string.hpp",
            "#define READS_OR_Z(c) __attribute__((annotate(\"_In_reads_or_z_(\" #c \")\")))\n\
             typedef const unsigned short* PCWSTR;\n\
             extern \"C\" void Read(unsigned count, READS_OR_Z(count) PCWSTR value);\n",
        )],
        &["-x", "c++"],
    )
    .unwrap();
    let rdl = snapshot
        .emit_with_library("SizedString", "api.dll")
        .unwrap();

    assert!(
        rdl.contains("Read(count: u32, #[len_param(0)] value: PCWSTR)"),
        "{rdl}"
    );
}

#[test]
fn pointer_sized_abi_typedefs_collapse_to_rust_primitives() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "pointer_sized.hpp",
            "typedef unsigned __int64 ULONG_PTR;\n\
             typedef ULONG_PTR SIZE_T;\n\
             void Allocate(SIZE_T size);\n",
        )],
        &[
            "-x",
            "c++",
            "--target=x86_64-pc-windows-msvc",
            "-fms-extensions",
        ],
    )
    .unwrap();
    let rdl = snapshot
        .emit_with_library("PointerSized", "api.dll")
        .unwrap();

    assert!(rdl.contains("fn Allocate(size: usize)"), "{rdl}");
    assert!(!rdl.contains("type ULONG_PTR"), "{rdl}");
    assert!(!rdl.contains("type SIZE_T"), "{rdl}");
}

#[test]
fn direct_nested_records_reserve_generated_names() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "nested_names.hpp",
            "typedef struct ROOT {\n\
                 union { int value; } choice;\n\
                 struct { int value; } entries[1];\n\
             } ROOT;\n",
        )],
        &["-x", "c++"],
    )
    .unwrap();
    let rdl = snapshot.emit("NestedNames").unwrap();

    assert!(rdl.contains("entries: [ROOT_1; 1]"), "{rdl}");
    assert!(rdl.contains("struct ROOT_1"), "{rdl}");
}

#[test]
fn x86_fastcall_callbacks_use_the_platform_convention() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "fastcall.hpp",
            "typedef void (__fastcall *FAST_CALLBACK)(void* value);",
        )],
        &[
            "-x",
            "c++",
            "--target=i686-pc-windows-msvc",
            "-fms-extensions",
        ],
    )
    .unwrap();
    let rdl = snapshot.emit("Fastcall").unwrap();

    assert!(rdl.contains("extern fn FAST_CALLBACK(value: *mut void)"));
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
fn public_alias_bridges_to_nested_c_tag_definition() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "nested_tag.hpp",
            "typedef struct OWNER {\n\
                 union _SEARCH { int value; } search;\n\
             } OWNER;\n\
             typedef union _SEARCH SEARCH;\n",
        )],
        &["-x", "c++"],
    )
    .unwrap();
    let rdl = snapshot.emit("NestedTag").unwrap();

    assert!(rdl.contains("type SEARCH = _SEARCH"));
    assert!(rdl.contains("union _SEARCH"));
    assert!(rdl.contains("value: i32"));
}

#[test]
fn identical_namespaced_types_collapse_in_flat_output() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "namespaces.hpp",
            "namespace First { typedef enum { None = 0, One = 1 } Flags; }\n\
             namespace Second { typedef enum { None = 0, One = 1 } Flags; }\n",
        )],
        &["-x", "c++"],
    )
    .unwrap();
    let rdl = snapshot.emit("Namespaced").unwrap();

    assert_eq!(rdl.matches("enum Flags").count(), 1);
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
fn unnamed_enum_fields_use_their_underlying_type() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "unnamed_enum.hpp",
            "struct WITH_ENUM { enum { None = 0, One = 1 } value; };",
        )],
        &["-x", "c++"],
    )
    .unwrap();
    let rdl = snapshot.emit("UnnamedEnum").unwrap();

    assert!(rdl.contains("value: i32"));
}

#[test]
fn midl_anonymous_enum_constants_use_the_following_scalar_alias() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "midl_enum.hpp",
            "typedef unsigned long DWORD;\n\
             enum __MIDL_generated_flags { FLAG_NONE = 0, FLAG_HIGH = 0x100 };\n\
             typedef DWORD PUBLIC_FLAGS;\n",
        )],
        &["-x", "c++"],
    )
    .unwrap();
    let rdl = snapshot.emit("MidlEnum").unwrap();

    assert!(!rdl.contains("__MIDL"));
    assert!(rdl.contains("type PUBLIC_FLAGS = u32;"));
    assert!(rdl.contains("const FLAG_NONE: PUBLIC_FLAGS = 0;"));
    assert!(rdl.contains("const FLAG_HIGH: PUBLIC_FLAGS = 256;"));
}

#[test]
fn midl_anonymous_enum_without_an_alias_emits_loose_constants() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "midl_loose_enum.hpp",
            "enum __MIDL_generated_values { VALUE_NONE = 0, VALUE_ONE = 1 };",
        )],
        &["-x", "c++"],
    )
    .unwrap();
    let rdl = snapshot.emit("MidlLooseEnum").unwrap();

    assert!(!rdl.contains("__MIDL"));
    assert!(rdl.contains("const VALUE_NONE: i32 = 0;"));
    assert!(rdl.contains("const VALUE_ONE: i32 = 1;"));
}

#[test]
fn referenced_midl_enum_retains_its_generated_abi_type() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "midl_referenced_enum.hpp",
            "enum __MIDL_generated_values { VALUE_NONE = 0, VALUE_ONE = 1 };\n\
             struct USES_VALUES { __MIDL_generated_values value; };",
        )],
        &["-x", "c++"],
    )
    .unwrap();
    let rdl = snapshot.emit("MidlReferencedEnum").unwrap();

    assert!(rdl.contains("enum __MIDL_generated_values"));
    assert!(rdl.contains("value: __MIDL_generated_values"));
}

#[test]
fn midl_anonymous_enum_constants_support_direct_scalar_aliases() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "midl_scalar_enum.hpp",
            "enum __MIDL_generated_flags { FLAG_NONE = 0, FLAG_HIGH = 0x100 };\n\
             typedef unsigned long PUBLIC_FLAGS;\n",
        )],
        &["-x", "c++"],
    )
    .unwrap();
    let rdl = snapshot.emit("MidlScalarEnum").unwrap();

    assert!(!rdl.contains("__MIDL"));
    assert!(rdl.contains("type PUBLIC_FLAGS = u32;"));
    assert!(rdl.contains("const FLAG_HIGH: PUBLIC_FLAGS = 256;"));
}

#[test]
fn midl_private_enum_constants_use_the_matching_public_alias() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "midl_private_enum.hpp",
            "/* File created by MIDL compiler version 8.01.0628 */\n\
             typedef unsigned long DWORD;\n\
             enum _PUBLIC_FLAGS { FLAG_NONE = 0, FLAG_HIGH = 0x80000000 };\n\
             typedef DWORD PUBLIC_FLAGS;\n",
        )],
        &["-x", "c++"],
    )
    .unwrap();
    let rdl = snapshot.emit("MidlPrivateEnum").unwrap();

    assert!(!rdl.contains("enum _PUBLIC_FLAGS"));
    assert!(rdl.contains("type PUBLIC_FLAGS = u32;"));
    assert!(rdl.contains("const FLAG_NONE: PUBLIC_FLAGS = 0;"));
    assert!(rdl.contains("const FLAG_HIGH: PUBLIC_FLAGS = 2147483648;"));
}

#[test]
fn non_midl_private_enum_retains_its_declared_type() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "private_enum.hpp",
            "typedef unsigned long DWORD;\n\
             enum _PUBLIC_FLAGS { FLAG_NONE = 0 };\n\
             typedef DWORD PUBLIC_FLAGS;\n",
        )],
        &["-x", "c++"],
    )
    .unwrap();
    let rdl = snapshot.emit("PrivateEnum").unwrap();

    assert!(rdl.contains("enum _PUBLIC_FLAGS"));
}

#[test]
fn referenced_midl_private_enum_retains_its_declared_type() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "midl_referenced_private_enum.hpp",
            "/* File created by MIDL compiler version 8.01.0628 */\n\
             typedef unsigned long DWORD;\n\
             enum _PUBLIC_FLAGS { FLAG_NONE = 0 };\n\
             typedef DWORD PUBLIC_FLAGS;\n\
             struct USES_FLAGS { _PUBLIC_FLAGS value; };\n",
        )],
        &["-x", "c++"],
    )
    .unwrap();
    let rdl = snapshot.emit("MidlReferencedPrivateEnum").unwrap();

    assert!(rdl.contains("enum _PUBLIC_FLAGS"));
    assert!(rdl.contains("value: _PUBLIC_FLAGS"));
}

#[test]
fn conventional_midl_enum_typedef_retains_its_declared_type() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "midl_enum_typedef.hpp",
            "/* File created by MIDL compiler version 8.01.0628 */\n\
             typedef enum _PUBLIC_FLAGS { FLAG_NONE = 0 } PUBLIC_FLAGS;\n",
        )],
        &["-x", "c++"],
    )
    .unwrap();
    let rdl = snapshot.emit("MidlEnumTypedef").unwrap();

    assert!(rdl.contains("enum PUBLIC_FLAGS"));
}

#[test]
fn midl_private_enum_requires_a_matching_adjacent_alias() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "midl_mismatched_private_enum.hpp",
            "/* File created by MIDL compiler version 8.01.0628 */\n\
             typedef unsigned long DWORD;\n\
             enum _PRIVATE_FLAGS { FLAG_NONE = 0 };\n\
             typedef DWORD PUBLIC_FLAGS;\n",
        )],
        &["-x", "c++"],
    )
    .unwrap();
    let rdl = snapshot.emit("MidlMismatchedPrivateEnum").unwrap();

    assert!(rdl.contains("enum _PRIVATE_FLAGS"));
}

#[test]
fn midl_anonymous_record_pointer_uses_the_public_alias() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "midl_record.hpp",
            "typedef struct __MIDL_generated_record { int _; } *PUBLIC_HANDLE;",
        )],
        &["-x", "c++"],
    )
    .unwrap();
    let rdl = snapshot.emit("MidlRecord").unwrap();

    assert!(!rdl.contains("__MIDL"));
    assert!(rdl.contains("type PUBLIC_HANDLE = *mut void;"));
}

#[test]
fn async_finish_methods_drop_cross_method_sal_lengths() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "async_interface.hpp",
            "#define WRITES(c) __attribute__((annotate(\"_Out_writes_(\" #c \")\")))\n\
             struct IUnknown { virtual int Query() = 0; };\n\
             struct __declspec(uuid(\"12345678-1234-abcd-9876-0123456789ab\")) \
             AsyncITest : IUnknown {\n\
                 virtual int Begin_Read(unsigned count) = 0;\n\
                 virtual int Finish_Read(WRITES(count) int* values) = 0;\n\
             };\n",
        )],
        &["-x", "c++", "-fms-extensions"],
    )
    .unwrap();
    let rdl = snapshot.emit("AsyncInterface").unwrap();

    assert!(rdl.contains("fn Finish_Read(&self, values: *mut i32)"));
    assert!(!rdl.contains("len_param"));
}

#[test]
fn same_tu_compatible_typedef_redeclarations_collapse() {
    helpers::ensure_libclang();

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
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "redeclared-function.hpp",
            "#define IN __attribute__((annotate(\"_In_\")))\n\
             extern \"C\" void Shared(int first);\n\
             extern \"C\" void Shared(IN int second);\n",
        )],
        &["-x", "c++"],
    )
    .unwrap();
    let rdl = snapshot.emit_with_library("Redeclared", "api.dll").unwrap();

    assert_eq!(rdl.matches("fn Shared(").count(), 1);
}

#[test]
fn indirect_sal_size_parameter_is_preserved() {
    helpers::ensure_libclang();

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
fn function_parameters_decay_to_function_pointers() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "function-parameter.hpp",
            "#define IN __attribute__((annotate(\"_In_\")))\n\
             extern \"C\" void UseCallback(void Callback(int value));\n\
             extern \"C\" void UseInputCallback(IN void InputCallback(int value));\n",
        )],
        &["-x", "c++"],
    )
    .unwrap();
    let rdl = snapshot
        .emit_with_library("FunctionParameter", "api.dll")
        .unwrap();

    assert!(rdl.contains("fn UseCallback(Callback: *mut u8)"));
    assert!(rdl.contains("fn UseInputCallback(#[in] InputCallback: *mut u8)"));
}

#[test]
fn bare_function_typedef_emits_callback() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "bare-callback.hpp",
            "typedef long CALLBACK_TYPE(unsigned char* data, unsigned long size);\n",
        )],
        &["-x", "c++", "--target=x86_64-pc-windows-msvc"],
    )
    .unwrap();
    let rdl = snapshot.emit("Callbacks").unwrap();

    assert!(rdl.contains("extern \"C\" fn CALLBACK_TYPE(data: *mut u8, size: u32) -> i32"));
}

#[test]
fn callback_array_parameters_decay_to_pointers() {
    helpers::ensure_libclang();

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

    assert!(rdl.contains("values: *const u32"), "{rdl}");
    assert!(rdl.contains("names: *const *const u16"), "{rdl}");
}

#[test]
fn callback_parameters_preserve_names_and_annotations() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "callback-annotations.hpp",
            "#define IN_OPT __attribute__((annotate(\"_In_opt_\")))\n\
             #define INOUT __attribute__((annotate(\"_Inout_\")))\n\
             typedef void (*CALLBACK)(IN_OPT void* context, INOUT unsigned* value);\n",
        )],
        &["-x", "c++"],
    )
    .unwrap();
    let rdl = snapshot.emit("Callback").unwrap();

    assert!(rdl.contains("#[in] #[opt] context: *mut void"), "{rdl}");
    assert!(rdl.contains("#[in] #[out] value: *mut u32"), "{rdl}");
}

#[test]
fn pointers_to_function_typedefs_preserve_callback_names() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "callback-pointers.hpp",
            "#define CALL __stdcall\n\
             typedef void CALLBACK(int value);\n\
             typedef void* CALL ALLOCATOR(unsigned size);\n\
             struct TABLE { CALLBACK* callback; };\n\
             void UseCallback(CALLBACK* callback, CALLBACK** previous, ALLOCATOR* allocator);\n",
        )],
        &["-x", "c++", "-fms-extensions"],
    )
    .unwrap();
    let rdl = snapshot.emit_with_library("Callbacks", "test.dll").unwrap();

    assert!(rdl.contains("callback: CALLBACK"), "{rdl}");
    assert!(
        rdl.contains(
            "UseCallback(callback: CALLBACK, previous: *mut CALLBACK, allocator: ALLOCATOR)"
        ),
        "{rdl}"
    );
}

#[test]
fn source_com_outptr_macro_preserves_iid_annotation() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "com-outptr.hpp",
            "#define _COM_Outptr_ __attribute__((annotate(\"_Outptr_\")))\n\
             struct IThing;\n\
             extern \"C\" int Create(const void* iid, _COM_Outptr_ void** object);\n\
             extern \"C\" int CreateThing(_COM_Outptr_ IThing** object);\n",
        )],
        &["-x", "c++"],
    )
    .unwrap();
    let rdl = snapshot.emit_with_library("ComOut", "api.dll").unwrap();

    assert!(rdl.contains("#[iid_is] object: *mut *mut void"), "{rdl}");
    assert!(
        rdl.contains("CreateThing(object: *mut *mut IThing)"),
        "{rdl}"
    );
    assert!(!rdl.contains("CreateThing(#[iid_is]"), "{rdl}");
}

#[test]
fn legacy_source_annotations_are_preserved() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "legacy-annotations.hpp",
            "#define IN\n\
             #define OUT\n\
             #define OPTIONAL\n\
             typedef void (*CALLBACK)(IN OUT void* context OPTIONAL);\n\
             extern \"C\" void Update(IN const int* input, OUT int* output OPTIONAL);\n",
        )],
        &["-x", "c++"],
    )
    .unwrap();
    let rdl = snapshot
        .emit_with_library("LegacyAnnotations", "api.dll")
        .unwrap();

    assert!(
        rdl.contains("CALLBACK(#[in] #[out] #[opt] context: *mut void)"),
        "{rdl}"
    );
    assert!(
        rdl.contains("Update(input: *const i32, #[opt] output: *mut i32)"),
        "{rdl}"
    );
}

#[test]
fn output_pointer_chains_preserve_source_constness() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "output-pointer-chain.hpp",
            "#define OUT __attribute__((annotate(\"_Out_\")))\n\
             extern \"C\" void GetValue(OUT const int** value);\n",
        )],
        &["-x", "c++"],
    )
    .unwrap();
    let rdl = snapshot
        .emit_with_library("OutputPointerChain", "api.dll")
        .unwrap();

    assert!(
        rdl.contains("GetValue(#[out] value: *const *const i32)"),
        "{rdl}"
    );
}

#[test]
fn array_typedef_parameters_decay_to_pointers() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "array-parameter.hpp",
            "typedef unsigned char VERSION[4];\n\
             extern \"C\" void GetVersion(VERSION value);\n",
        )],
        &["-x", "c++"],
    )
    .unwrap();
    let rdl = snapshot
        .emit_with_library("ArrayParameter", "api.dll")
        .unwrap();

    assert!(rdl.contains("type VERSION = [u8; 4]"), "{rdl}");
    assert!(rdl.contains("GetVersion(value: *mut u8)"), "{rdl}");
}

#[test]
fn variadic_functions_are_emitted() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "variadic.hpp",
            "extern \"C\" int Report(unsigned count, ...);\n",
        )],
        &["-x", "c++"],
    )
    .unwrap();
    let rdl = snapshot.emit_with_library("Variadic", "api.dll").unwrap();

    assert!(
        rdl.contains("extern \"C\" fn Report(count: u32, ...) -> i32"),
        "{rdl}"
    );
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

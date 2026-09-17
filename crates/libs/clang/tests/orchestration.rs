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
fn non_finite_floating_point_constants_are_omitted() {
    helpers::ensure_libclang();

    let snapshot = extract(
        [Input::new(
            "non_finite_floats.hpp",
            "const float FLOAT_INFINITY = __builtin_huge_valf();\n\
             const double DOUBLE_INFINITY = __builtin_huge_val();\n\
             const double NOT_A_NUMBER = __builtin_nan(\"\");\n\
             #define MACRO_INFINITY (__builtin_huge_val())\n\
             #define MACRO_NAN (__builtin_nan(\"\"))\n\
             #define NARROWED_INFINITY 1.0e100f\n\
             const float SDK_POSITIVE_INFINITY = ((float)(1e308 * 10));\n\
             const float SDK_NEGATIVE_INFINITY = ((float)(-1e308 * 10));\n\
             const float SDK_NAN = ((float)((1e308 * 10) * 0.));\n\
             const float FINITE_FLOAT = 1.5f;\n\
             #define FINITE_MACRO 2.5\n",
        )],
        &["-x", "c++"],
    )
    .unwrap();
    let rdl = snapshot.emit("NonFiniteFloats").unwrap();

    for name in [
        "FLOAT_INFINITY",
        "DOUBLE_INFINITY",
        "NOT_A_NUMBER",
        "MACRO_INFINITY",
        "MACRO_NAN",
        "NARROWED_INFINITY",
        "SDK_POSITIVE_INFINITY",
        "SDK_NEGATIVE_INFINITY",
        "SDK_NAN",
    ] {
        assert!(!rdl.contains(name), "{rdl}");
    }
    assert!(rdl.contains("const FINITE_FLOAT: f32 = 1.5"), "{rdl}");
    assert!(rdl.contains("const FINITE_MACRO: f64 = 2.5"), "{rdl}");

    let output =
        std::env::temp_dir().join(format!("windows-clang-floats-{}.winmd", std::process::id()));
    windows_rdl::reader()
        .input_text(&rdl)
        .output(&output)
        .write()
        .unwrap();
    std::fs::remove_file(output).unwrap();
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

fn rust_string_constant<'a>(source: &'a str, name: &str) -> &'a str {
    let prefix = format!("const {name}: &str = \"");
    source
        .lines()
        .find_map(|line| line.trim().strip_prefix(&prefix))
        .unwrap()
        .strip_suffix("\";")
        .unwrap()
}

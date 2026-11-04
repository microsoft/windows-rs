use windows_clang::*;

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];
const ANNOTATIONS: &str = r#"
#define GROUP(name, storage, ...) \
    typedef __attribute__((annotate("Win32Metadata.Enum(" #__VA_ARGS__ ")", __VA_ARGS__))) storage name
#define FLAGS(name, storage, ...) \
    typedef __attribute__((annotate("Win32Metadata.FlagsEnum(" #__VA_ARGS__ ")", __VA_ARGS__))) storage name
#define ASSOC(name) __attribute__((annotate("Win32Metadata.AssociatedEnum", "Win32Metadata.EnumName:" #name)))
#define IMPORT __attribute__((annotate("Win32Metadata.ImportLibrary", "fixture.dll")))
"#;

fn input(source: &str) -> Input {
    Input {
        name: "enum_annotations.h".into(),
        source: format!("{ANNOTATIONS}\n{source}"),
        ..Default::default()
    }
}

#[test]
fn namespaced_enum_members() {
    helpers::ensure_libclang();
    for (source, macros) in [
        (
            "#define A 1\nnamespace N { GROUP(G, int, A); }",
            vec!["A".into()],
        ),
        ("namespace N { const int A = 1; GROUP(G, int, A); }", vec![]),
    ] {
        let mut source = input(source);
        source.macros = macros;
        let output = projection::generate(&[source], "Fixture").unwrap();
        assert_eq!(output.enums, 1);
        assert_eq!(output.constants, 0);
        let text: String = output
            .partitions
            .values()
            .map(|part| part.rdl.as_str())
            .collect();
        assert!(text.contains("enum G"), "{text}");
        assert!(text.contains("A = 1"), "{text}");
        windows_rdl::reader()
            .input("..\\..\\..\\..\\metadata\\metadata.rdl")
            .input_text(&text)
            .output(std::path::Path::new(env!("OUT_DIR")).join("namespaced-enum-members.winmd"))
            .write()
            .unwrap();
    }
}

#[test]
fn rejected_enum_contracts() {
    helpers::ensure_libclang();
    for (source, expected) in [
        ("#define A -1\nGROUP(G, unsigned int, A);", "does not fit"),
        ("#define A 256\nGROUP(G, unsigned char, A);", "does not fit"),
        ("#define A 128\nGROUP(G, signed char, A);", "does not fit"),
        ("#define A 1\nGROUP(G, float, A);", "integer storage"),
        ("#define A 1.0\nGROUP(G, int, A);", "integral constant"),
        ("#define A 1\nGROUP(G, int, A, A);", "distinct"),
        ("GROUP(G, int, 1 + 2);", "unqualified"),
        ("enum E { A }; GROUP(G, int, A);", "existing native enum"),
        (
            "#define A 1\nGROUP(G, int, A); GROUP(H, int, A);",
            "multiple enum groups",
        ),
        (
            "namespace N { const int A = 1; GROUP(G, int, A); }
             namespace Other { const int A = 2; }",
            "collides with native declaration",
        ),
        (
            "namespace Other { const int A = 2; }\n#define A 1\nnamespace N { GROUP(G, int, A); }",
            "collides with native declaration",
        ),
        (
            "#define A 1\nGROUP(G, int, A); extern \"C\" IMPORT void Bad(ASSOC(G) G value);",
            "enum-group-typed",
        ),
        (
            "extern \"C\" IMPORT void Bad(ASSOC(Missing) int value);",
            "not found",
        ),
        (
            "typedef int E; extern \"C\" IMPORT void Bad(ASSOC(E) int value);",
            "not an enum",
        ),
        (
            "enum E { A }; extern \"C\" IMPORT void Bad(ASSOC(E) float value);",
            "integer storage",
        ),
        (
            "enum E { A }; extern \"C\" IMPORT void Bad(ASSOC(E) E value);",
            "already enum",
        ),
        (
            "enum E { A }; ASSOC(E) const int value = 0;",
            "requires a parameter",
        ),
        (
            "struct C { static constexpr int A = 1; };\n#define A C::A\nGROUP(G, int, A);",
            "enum member declaration",
        ),
        (
            "enum E { A }; namespace N { struct E {}; extern \"C\" IMPORT void Bad(ASSOC(enum E) int value); }",
            "not an enum",
        ),
        (
            "enum E { A }; struct S { ASSOC(E) unsigned int value : 1; };",
            "unsupported",
        ),
        (
            "enum E { A }; extern \"C\" IMPORT void Bad(ASSOC(E) ASSOC(E) int value);",
            "duplicate",
        ),
    ] {
        let error = projection::generate(&[input(source)], "Fixture").unwrap_err();
        assert!(error.to_string().contains(expected), "{source}\n{error}");
    }
}

#[test]
fn group_redeclarations_conflict_before_pruning() {
    helpers::ensure_libclang();
    for (conflict, expected) in [
        ("#define A 2\nGROUP(G, int, A);", "conflicting enum groups"),
        ("#define A 1\nGROUP(G, unsigned int, A);", "G"),
    ] {
        let inputs: Vec<_> = ["typedef int G;", "#define A 1\nGROUP(G, int, A);", conflict]
            .into_iter()
            .enumerate()
            .map(|(index, source)| {
                let mut input = input(source);
                input.name = format!("group{index}.h");
                input
            })
            .collect();
        let error = capture_partial(&inputs, &TARGETS).err().unwrap();
        assert!(error.to_string().contains(expected), "{error}");
    }
}

#[test]
fn filtered_associations_and_partial_failures() {
    helpers::ensure_libclang();
    let source = r#"
        #define A 1
        GROUP(G, unsigned short, A);
        enum E { Native };
        extern "C" IMPORT void Selected(ASSOC(G) unsigned int value, ASSOC(E) int native);
        extern "C" IMPORT ASSOC(G) int Get();
        typedef void (*Callback)(ASSOC(G) unsigned int value);
        struct Holder { ASSOC(G) unsigned int value; };
        extern "C" IMPORT void Other();
    "#;
    let mut selected = input(source);
    selected.roots = vec!["Selected".into()];
    let captured = capture_partial(&[selected], &TARGETS).unwrap();
    assert!(captured.omissions.is_empty(), "{:?}", captured.omissions);
    let output = projection::project_partial(captured, "Fixture").unwrap();
    assert!(output.omissions.is_empty(), "{:?}", output.omissions);
    let text: String = output
        .output
        .partitions
        .values()
        .map(|part| part.rdl.as_str())
        .collect();
    assert!(text.contains("enum G"), "{text}");
    assert!(text.contains("enum E"), "{text}");
    assert!(!text.contains("Other"), "{text}");

    let failed = input(&source.replace("#define A 1", "#define A -1"));
    let output = projection::generate_partial(&[failed], "Fixture").unwrap();
    let text: String = output
        .output
        .partitions
        .values()
        .map(|part| part.rdl.as_str())
        .collect();
    assert!(!text.contains("Selected"), "{text}");
    assert!(!text.contains("Get"), "{text}");
    assert!(!text.contains("Callback"), "{text}");
    assert!(!text.contains("Holder"), "{text}");
    assert!(text.contains("Other"), "{text}");
    assert!(
        output
            .omissions
            .iter()
            .any(|message| message.contains("does not fit")),
        "{:?}",
        output.omissions
    );
}

#[test]
fn explicit_storage_and_native_conversions() {
    helpers::ensure_libclang();
    let mut source = input(
        r#"
        #define NEGATIVE -100
        #define CONVERTED ((unsigned short)-100)
        #define HIGH 0x80000000L
        #define LOW 1L
        GROUP(Signed, short, NEGATIVE);
        GROUP(Unsigned, unsigned short, CONVERTED);
        GROUP(Flags, unsigned long, HIGH, LOW);
    "#,
    );
    source.macros = ["NEGATIVE", "CONVERTED", "HIGH", "LOW"]
        .map(str::to_string)
        .into();
    let output = projection::generate(&[source], "Fixture").unwrap();
    assert_eq!(output.enums, 3);
    assert_eq!(output.constants, 0);
    let text: String = output
        .partitions
        .values()
        .map(|part| part.rdl.as_str())
        .collect();
    assert!(text.contains("NEGATIVE = -100"), "{text}");
    assert!(text.contains("CONVERTED = 65436"), "{text}");
    assert!(text.contains("HIGH = 2147483648"), "{text}");
}

#[test]
fn enum_name_resolution() {
    helpers::ensure_libclang();
    let source = input(
        r#"
        typedef enum E { A } E;
        typedef E Alias;
        #define B 1
        GROUP(G, unsigned int, B);
        typedef G GroupAlias;
        namespace Nested {
            enum Local { C };
            extern "C" IMPORT void Selected(ASSOC(Local) int local,
                ASSOC(::E) int global, ASSOC(Alias) int alias, ASSOC(GroupAlias) unsigned int group);
        }
    "#,
    );
    let output = projection::generate(&[source], "Fixture").unwrap();
    let text: String = output
        .partitions
        .values()
        .map(|part| part.rdl.as_str())
        .collect();
    assert!(
        text.contains("#[associated_enum(Local)] local: i32"),
        "{text}"
    );
    assert!(text.contains("#[associated_enum(E)] global: i32"), "{text}");
    assert!(text.contains("#[associated_enum(E)] alias: i32"), "{text}");
    assert!(text.contains("#[associated_enum(G)] group: u32"), "{text}");
    let error = projection::generate(
        &[input(
            "enum E { A }; struct S { enum E { B }; ASSOC(E) int value; };",
        )],
        "Fixture",
    )
    .unwrap_err();
    assert!(error.to_string().contains("class-scoped"), "{error}");
    let mut source = input(
        "enum E { A }; namespace N { struct E { int value; }; extern \"C\" IMPORT void Selected(ASSOC(E) int value); }",
    );
    source.roots = vec!["N::Selected".into()];
    let error = projection::generate(&[source], "Fixture").unwrap_err();
    assert!(error.to_string().contains("not an enum"), "{error}");
    let mut source = input(
        "namespace N { enum E { A }; } using namespace N; enum E { B }; extern \"C\" IMPORT void Selected(ASSOC(E) int value);",
    );
    source.roots = vec!["Selected".into()];
    let error = projection::generate(&[source], "Fixture").unwrap_err();
    assert!(error.to_string().contains("globally qualified"), "{error}");
}

#[test]
fn external_enum_group_ownership() {
    helpers::ensure_libclang();
    let source = input(
        "#define A 1\nGROUP(G, unsigned short, A);\nextern \"C\" IMPORT void Selected(ASSOC(G) unsigned int value);",
    );
    let snapshots = capture(&[source], &TARGETS).unwrap();
    let merged = merge(&snapshots).unwrap();
    let path = std::path::Path::new(env!("OUT_DIR")).join("external-enum-group.winmd");
    for value in [1, 2] {
        windows_rdl::reader()
            .input_text(&format!(
                "#[win32] mod Shared {{ #[repr(u16)] enum G {{ A = {value}, }} }}"
            ))
            .output(&path)
            .write()
            .unwrap();
        let index = windows_metadata::reader::Index::read(&path).unwrap();
        let references = projection::References::new(&index);
        let output = projection::project_with_options(
            &merged,
            "Fixture",
            &projection::Options {
                references: Some(&references),
                ..Default::default()
            },
        );
        if value == 1 {
            let output = output.unwrap();
            assert_eq!(output.enums, 0);
            let text: String = output
                .partitions
                .values()
                .map(|part| part.rdl.as_str())
                .collect();
            assert!(text.contains("#[associated_enum(Shared::G)]"), "{text}");
        } else {
            assert!(
                output
                    .unwrap_err()
                    .to_string()
                    .contains("differs from metadata")
            );
        }
    }
}

#[test]
fn grouping_is_independent_of_capture_order() {
    helpers::ensure_libclang();
    for roots in [["G", "A"], ["A", "G"]] {
        let mut source = input("const int A = 1; GROUP(G, int, A);");
        source.roots = roots.map(str::to_string).into();
        let output = projection::generate(&[source], "Fixture").unwrap();
        assert_eq!(output.enums, 1);
        assert_eq!(output.constants, 0);
    }
}

#[test]
fn associated_enum_rejects_group_alias_storage() {
    helpers::ensure_libclang();
    let path = std::path::Path::new(env!("OUT_DIR")).join("external-enum-alias.winmd");
    windows_rdl::reader()
        .input_text("#[win32] mod Shared { #[repr(u16)] enum G { A = 1 } }")
        .output(&path)
        .write()
        .unwrap();
    let index = windows_metadata::reader::Index::read(&path).unwrap();
    let references = projection::References::new(&index);
    for alias in ["typedef G Alias;", "typedef G* Alias;"] {
        let source = input(&format!(
            "#define A 1\nGROUP(G, unsigned short, A);
            {alias}
            extern \"C\" IMPORT void Bad(ASSOC(G) Alias value);
            extern \"C\" IMPORT void Good(ASSOC(G) int value);"
        ));
        let captured = capture_partial(&[source], &TARGETS).unwrap();
        let output = projection::project_partial_with_options(
            captured,
            "Fixture",
            &projection::Options {
                references: Some(&references),
                ..Default::default()
            },
        )
        .unwrap();
        assert!(
            output
                .omissions
                .iter()
                .any(|message| message.contains("enum-group-typed")),
            "{:?}",
            output.omissions
        );
        let text: String = output
            .output
            .partitions
            .values()
            .map(|part| part.rdl.as_str())
            .collect();
        assert!(!text.contains("fn Bad("), "{text}");
        assert!(text.contains("fn Good("), "{text}");
        windows_rdl::reader()
            .input("..\\..\\..\\..\\metadata\\metadata.rdl")
            .reference(&path)
            .input_text(&text)
            .output(path.with_extension("output.winmd"))
            .write()
            .unwrap();
    }
}

#[test]
fn native_constant_contracts_survive_grouping() {
    helpers::ensure_libclang();
    let source = r#"
        [[deprecated("use B")]] const int A = 1;
        GROUP(G, unsigned int, A);
        extern "C" IMPORT void Selected(ASSOC(G) unsigned int value);
    "#;
    let mut selected = input(source);
    selected.roots = vec!["Selected".into()];
    let output = projection::generate(&[selected], "Fixture").unwrap();
    let text: String = output
        .partitions
        .values()
        .map(|part| part.rdl.as_str())
        .collect();
    assert!(
        text.contains("#[deprecated(\"use B\")]\n        A = 1"),
        "{text}"
    );
    assert_eq!(output.constants, 0);

    let source = source.replace(
        "[[deprecated(\"use B\")]]",
        "__attribute__((annotate(\"Unknown\")))",
    );
    let output = projection::generate_partial(&[input(&source)], "Fixture").unwrap();
    assert_eq!(output.output.enums, 0);
    assert_eq!(output.output.functions, 0);
    assert!(!output.omissions.is_empty());
}

#[test]
fn sdk_constant_groups() {
    helpers::ensure_libclang();
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let version = helpers::read_str_const(
        root.join("crates\\tools\\win32\\src\\main.rs"),
        "SDK_VERSION",
    );
    let include = helpers::nuget_package("microsoft.windows.sdk.cpp", &version)
        .join("c\\Include")
        .join(helpers::marketing_dir(&version));
    let mut source = input(
        r#"
        #include <winsock2.h>
        FLAGS(DUPLICATE_HANDLE_OPTIONS, unsigned long, DUPLICATE_CLOSE_SOURCE, DUPLICATE_SAME_ACCESS);
        FLAGS(GENERIC_ACCESS_RIGHTS, unsigned long, GENERIC_READ, GENERIC_WRITE, GENERIC_EXECUTE, GENERIC_ALL);
        GROUP(ADDRESS_FAMILY, USHORT, AF_UNSPEC, AF_INET, AF_INET6);
        extern "C" IMPORT void Selected(
            ASSOC(DUPLICATE_HANDLE_OPTIONS) DWORD options,
            ASSOC(GENERIC_ACCESS_RIGHTS) DWORD access,
            ASSOC(ADDRESS_FAMILY) int family);
    "#,
    );
    source.roots = vec!["Selected".into()];
    source.arguments = vec![
        "-fms-extensions".into(),
        "-resource-dir".into(),
        helpers::clang_resource_dir(),
    ];
    for directory in ["ucrt", "um", "shared"] {
        source.arguments.extend([
            "-isystem".into(),
            include.join(directory).display().to_string(),
        ]);
    }
    let output = projection::generate(&[source], "Fixture").unwrap();
    assert_eq!(output.enums, 3);
    let text: String = output
        .partitions
        .values()
        .map(|part| part.rdl.as_str())
        .collect();
    assert!(text.contains("DUPLICATE_SAME_ACCESS = 2"), "{text}");
    assert!(text.contains("GENERIC_READ = 2147483648"), "{text}");
    assert!(text.contains("AF_INET6 = 23"), "{text}");
    let winmd = std::path::Path::new(env!("OUT_DIR")).join("sdk-enum-groups.winmd");
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_text(&text)
        .output(&winmd)
        .write()
        .unwrap();
    let index = windows_metadata::reader::Index::read(&winmd).unwrap();
    assert_eq!(
        index
            .expect("Fixture", "DUPLICATE_HANDLE_OPTIONS")
            .underlying_type(),
        Some(windows_metadata::Type::U32)
    );
    assert_eq!(
        index.expect("Fixture", "ADDRESS_FAMILY").underlying_type(),
        Some(windows_metadata::Type::U16)
    );
    for style in ["default", "minimal", "sys"] {
        let rust = winmd.with_extension(format!("{style}.rs"));
        let mut builder = windows_bindgen::builder();
        builder
            .input(&winmd)
            .filter("Fixture.Selected")
            .flat()
            .output(&rust);
        match style {
            "minimal" => {
                builder.minimal();
            }
            "sys" => {
                builder.sys();
            }
            _ => {}
        }
        builder.write();
        let text = std::fs::read_to_string(rust).unwrap();
        for expected in [
            "pub const DUPLICATE_SAME_ACCESS:",
            "pub const GENERIC_READ:",
            "pub const AF_INET:",
        ] {
            assert!(text.contains(expected), "{style}: {text}");
        }
        assert!(
            text.split_whitespace()
                .collect::<String>()
                .contains("family:i32"),
            "{style}: {text}"
        );
    }
}

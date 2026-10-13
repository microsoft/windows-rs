use std::path::Path;
use windows_clang2::*;
use windows_metadata::{HasAttributes, Value, reader::*};

fn input(source: &str, target: Target, names: &[&str]) -> Input {
    let mut input = Input::new("macros.cpp", source, target);
    input
        .required
        .extend(names.iter().map(|name| name.to_string()));
    input
}

#[test]
fn reviewed_integer_values_on_three_targets() {
    let compiler = Compiler::new().unwrap();
    for target in [Target::X86, Target::X64, Target::Arm64] {
        let snapshot = compiler
            .capture(&input(
                include_str!("..\\input\\macro_values.h"),
                target,
                &[
                    "BASE",
                    "COMBINED",
                    "FLAG",
                    "MAXIMUM",
                    "MINIMUM",
                    "NEGATIVE",
                    "SHORT",
                    "SIGNED_CHAR",
                    "TRUTH",
                    "WRAPPED",
                ],
            ))
            .unwrap();
        let rdl = snapshot.rdl("Test").unwrap();
        assert_eq!(
            rdl,
            include_str!("..\\input\\macro_values.rdl").replace("\r\n", "\n")
        );
        assert_eq!(snapshot.macros().count(), 10);
        let probe = snapshot.macro_probe.as_ref().unwrap();
        assert!(
            probe
                .source
                .starts_with(include_str!("..\\input\\macro_values.h"))
        );
        assert_eq!(probe.arguments, snapshot.arguments);
        let path = Path::new(env!("OUT_DIR")).join(format!("macro_values_{target:?}.winmd"));
        windows_rdl::reader()
            .input_text(&rdl)
            .output(&path)
            .write()
            .unwrap();
        let index = Index::read(&path).unwrap();
        for (name, value) in [
            ("BASE", Value::U32(16)),
            ("COMBINED", Value::U32(28)),
            ("FLAG", Value::U32(1 << 31)),
            ("MAXIMUM", Value::U64(u64::MAX)),
            ("MINIMUM", Value::I64(i64::MIN)),
            ("NEGATIVE", Value::I32(-3)),
            ("SHORT", Value::U16(u16::MAX)),
            ("SIGNED_CHAR", Value::I8(i8::MIN)),
            ("TRUTH", Value::Bool(true)),
            ("WRAPPED", Value::U32(u32::MAX)),
        ] {
            let Item::Const(field) = index.expect_item("Test", name) else {
                panic!("expected constant");
            };
            assert_eq!(field.ty(), value.ty());
            assert_eq!(field.constant().unwrap().value(), value);
        }
        let sized = compiler
            .capture(&input(
                "#define POINTER_BYTES sizeof(void*)",
                target,
                &["POINTER_BYTES"],
            ))
            .unwrap();
        let MacroKind::Integer { value, .. } = &sized.macros().next().unwrap().kind else {
            panic!("expected integer");
        };
        assert_eq!(
            value,
            &if target == Target::X86 {
                Value::U32(4)
            } else {
                Value::U64(8)
            }
        );
    }
}

#[test]
fn aliases_preserve_native_targets_without_replacing_declarations() {
    let compiler = Compiler::new().unwrap();
    let source = "typedef int Original;
        extern \"C\" void Function(int);
        #define TYPE Original
        #define CALL Function";
    let snapshot = compiler
        .capture(&input(source, Target::X64, &["TYPE", "CALL"]))
        .unwrap();
    for (name, target, kind) in [
        ("TYPE", "Original", "TypedefDecl"),
        ("CALL", "Function", "FunctionDecl"),
    ] {
        let evidence = snapshot.macros().find(|value| value.name == name).unwrap();
        let MacroKind::Alias {
            qualified_name,
            kind: observed,
            usr,
            ..
        } = &evidence.kind
        else {
            panic!("expected alias");
        };
        assert_eq!(qualified_name, target);
        assert_eq!(observed, kind);
        assert!(!usr.is_empty());
    }
    assert_eq!(
        snapshot.rdl("Test").unwrap_err().stage,
        Stage::Representation
    );
}

#[test]
fn active_alias_descriptors_roundtrip_without_erasing_ansi_declarations() {
    let compiler = Compiler::new().unwrap();
    for target in [Target::X86, Target::X64, Target::Arm64] {
        let source = "typedef int Entry;
            typedef unsigned int EntryW;
            extern \"C\" void Function(int);
            extern \"C\" void FunctionW(unsigned int);
            #ifdef UNICODE
            #define Entry EntryW
            #define Function FunctionW
            #endif";
        let mut input = input(
            source,
            target,
            &["Entry", "EntryW", "Function", "FunctionW"],
        );
        input.arguments.push("-DUNICODE".into());
        let snapshot = compiler.capture(&input).unwrap();
        let imports = snapshot
            .functions()
            .map(|(decl, function)| {
                (
                    function.linker_name.clone(),
                    Import {
                        dll: "fixture.dll".into(),
                        export: decl.qualified_name.clone(),
                    },
                )
            })
            .collect();
        let rdl = snapshot.rdl_with_imports("Test", &imports).unwrap();
        let directory = Path::new(env!("OUT_DIR")).join(format!("macro_alias_{target:?}"));
        std::fs::create_dir_all(&directory).unwrap();
        let vocabulary = directory.join("vocabulary.winmd");
        windows_rdl::reader()
            .input_text(include_str!("..\\..\\..\\..\\..\\metadata\\metadata.rdl"))
            .output(&vocabulary)
            .write()
            .unwrap();
        let binary = directory.join("aliases.winmd");
        windows_rdl::reader()
            .reference(&vocabulary)
            .input_text(&rdl)
            .output(&binary)
            .write()
            .unwrap();
        let text = directory.join("roundtrip.rdl");
        windows_rdl::writer()
            .input(&binary)
            .output(&text)
            .write()
            .unwrap();
        let roundtrip = directory.join("roundtrip.winmd");
        windows_rdl::reader()
            .reference(&vocabulary)
            .input(&text)
            .output(&roundtrip)
            .write()
            .unwrap();
        for path in [&binary, &roundtrip] {
            let index = Index::read(path).unwrap();
            index.validate_native_contracts(None).unwrap();
            for (name, original, kind) in [
                ("EntryW", "Entry", "TypedefDecl"),
                ("FunctionW", "Function", "FunctionDecl"),
            ] {
                let attribute = match index.expect_item("Test", name) {
                    Item::Type(ty) => ty
                        .attributes()
                        .find(|attribute| attribute.name() == "NativeMacroAliasAttribute")
                        .unwrap(),
                    Item::Fn(method) => method
                        .attributes()
                        .find(|attribute| attribute.name() == "NativeMacroAliasAttribute")
                        .unwrap(),
                    _ => panic!("expected native alias target"),
                };
                assert_eq!(
                    attribute
                        .value()
                        .into_iter()
                        .map(|(_, value)| value)
                        .collect::<Vec<_>>(),
                    [
                        Value::Utf8(original.into()),
                        Value::Utf8(kind.into()),
                        Value::Utf8(format!(
                            "source-sha256={}\n{}",
                            snapshot.source_sha256,
                            snapshot.arguments.join("\n")
                        )),
                    ]
                );
                let original = index.expect_item("Test", original);
                let attributes = match original {
                    Item::Type(ty) => ty.attributes().collect::<Vec<_>>(),
                    Item::Fn(method) => method.attributes().collect::<Vec<_>>(),
                    _ => panic!("expected original ANSI declaration"),
                };
                assert!(
                    !attributes
                        .iter()
                        .any(|attribute| attribute.name() == "NativeMacroAliasAttribute")
                );
            }
        }
    }
}

#[test]
fn unavailable_or_unsupported_macros_never_emit_partial_values() {
    let compiler = Compiler::new().unwrap();
    for source in [
        "#define VALUE",
        "#define VALUE(x) (x)",
        "#define VALUE 1\n#undef VALUE",
        "#define VALUE 1\n#undef VALUE\n#define VALUE 2",
        "#define VALUE runtime()\nint runtime();",
        "#define VALUE 1.25",
        "#define VALUE \"text\"",
        "typedef unsigned int Count;\n#define VALUE ((Count)3)",
        "enum Enum { Member };\n#define VALUE Member",
        "int object;\n#define VALUE (&object)",
    ] {
        assert!(
            compiler
                .capture(&input(source, Target::X64, &["VALUE"]))
                .is_err(),
            "{source}"
        );
    }
}

#[test]
fn active_definition_must_belong_to_the_selected_root() {
    let directory = Path::new(env!("OUT_DIR"));
    let header = directory.join("macro_scope.h");
    std::fs::write(&header, "#define VALUE 1\n").unwrap();
    let mut source = input(
        "#include \"macro_scope.h\"\n#undef VALUE\n#define VALUE 2\n",
        Target::X64,
        &["VALUE"],
    );
    source.arguments.push(format!("-I{}", directory.display()));
    source.roots.push(header);
    let error = Compiler::new().unwrap().capture(&source).unwrap_err();
    assert_eq!(error.stage, Stage::Scope);
    assert!(error.message.contains("active macro definition"));
}

#[test]
fn macro_snapshots_survive_reuse_and_compiler_destruction() {
    let snapshot = {
        let compiler = Compiler::new().unwrap();
        std::thread::scope(|scope| {
            let tasks = (0..4)
                .map(|_| {
                    scope.spawn(|| {
                        compiler
                            .capture(&input("#define VALUE (2u << 3)", Target::X64, &["VALUE"]))
                            .unwrap()
                            .rdl("Test")
                            .unwrap()
                    })
                })
                .collect::<Vec<_>>();
            for task in tasks {
                assert!(task.join().unwrap().contains("const VALUE: u32 = 16;"));
            }
        });
        compiler
            .capture(&input("#define VALUE 9", Target::X64, &["VALUE"]))
            .unwrap()
    };
    assert!(
        snapshot
            .rdl("Test")
            .unwrap()
            .contains("const VALUE: i32 = 9;")
    );
}

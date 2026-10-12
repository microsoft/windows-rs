use std::path::{Path, PathBuf};
use std::sync::Arc;
use windows_clang2::*;
use windows_metadata::{HasAttributes, Type, reader::Index};

fn input(source: &str, target: Target, required: &[&str]) -> Input {
    let mut input = Input::new("capture.cpp", source, target);
    input
        .required
        .extend(required.iter().map(|name| name.to_string()));
    input
}

const REQUIRED: &[&str] = &[
    "Void",
    "Boolean",
    "Narrow",
    "Byte",
    "Short",
    "Word",
    "Integer",
    "Long",
    "ULong",
    "LongLong",
    "ULongLong",
    "Float",
    "Double",
    "LongDouble",
    "Wide",
    "Char16",
    "Char32",
    "Number",
    "Constant",
    "Volatile",
    "Both",
    "PSTR",
    "LPSTR",
    "ConstantPointer",
    "PCSTR",
    "PointerToConstPointer",
    "ConstPointerToPointer",
    "VolatilePointee",
    "Array",
    "ConstantArray",
    "DirectConstantArray",
    "Reference",
    "Forward",
    "Other",
    "PForward",
    "PStorage",
    "Native::Value",
    "Native::Pointer",
];

#[test]
fn reviewed_aliases_on_three_windows_targets() {
    let compiler = Compiler::new().unwrap();
    for target in [Target::X86, Target::X64, Target::Arm64] {
        let snapshot = compiler
            .capture(&input(include_str!("../input/aliases.h"), target, REQUIRED))
            .unwrap();
        let rdl = snapshot.rdl("Test").unwrap();
        assert_eq!(
            rdl,
            include_str!("../input/aliases.rdl").replace("\r\n", "\n")
        );
        let directory = Path::new(env!("OUT_DIR")).join("capture");
        std::fs::create_dir_all(&directory).unwrap();
        let path = directory.join(format!("aliases_{target:?}.winmd"));
        windows_rdl::reader()
            .input_text(&rdl)
            .output(&path)
            .write()
            .unwrap();
        let index = Index::read(&path).unwrap();
        index.validate_native_contracts(None).unwrap();
        let alias = index.get("Test", "Long").next().unwrap();
        assert_eq!(alias.fields().next().unwrap().ty(), Type::I32);
        let constant = index.get("Test", "Constant").next().unwrap();
        assert_eq!(
            constant.fields().next().unwrap().ty(),
            Type::Const(Box::new(Type::value_named("Test", "Number")))
        );
        for name in ["Forward", "Storage"] {
            let def = index.get("Test", name).next().unwrap();
            assert!(def.fields().next().is_none());
            assert!(
                def.attributes()
                    .any(|attr| attr.name() == "NativeOpaqueAttribute")
            );
        }
        let evidence = snapshot
            .evidence()
            .find(|item| item.qualified_name == "PForward")
            .unwrap();
        assert_eq!(
            evidence.layout.unwrap().size,
            u64::from(target.pointer_bits() / 8)
        );
        assert!(!snapshot.compiler.is_empty());
        assert!(snapshot.library.is_file());
        assert!(snapshot.target_triple.contains("windows-msvc"));
    }
}

#[test]
fn compiler_and_translation_unit_do_not_own_snapshot() {
    let snapshot = {
        let compiler = Compiler::new().unwrap();
        compiler
            .capture(&input("typedef int Value;", Target::X64, &["Value"]))
            .unwrap()
    };
    assert!(snapshot.rdl("Test").unwrap().contains("type Value = i32;"));
}

#[test]
fn independent_sessions_share_library_and_restore_thread_bindings() {
    let previous = Arc::new(clang_sys::load_manually().unwrap());
    let saved = clang_sys::set_library(Some(Arc::clone(&previous)));
    let compiler = Arc::new(Compiler::new().unwrap());
    compiler
        .capture(&input("typedef int Value;", Target::X64, &["Value"]))
        .unwrap();
    assert!(Arc::ptr_eq(&previous, &clang_sys::get_library().unwrap()));
    assert!(
        compiler
            .capture(&input("invalid source", Target::X64, &["Value"]))
            .is_err()
    );
    assert!(Arc::ptr_eq(&previous, &clang_sys::get_library().unwrap()));
    std::thread::scope(|scope| {
        let tasks = (0..4)
            .map(|_| {
                let compiler = Arc::clone(&compiler);
                scope.spawn(move || {
                    assert!(clang_sys::get_library().is_none());
                    let snapshot = compiler
                        .capture(&input("typedef int Value;", Target::X64, &["Value"]))
                        .unwrap();
                    assert!(clang_sys::get_library().is_none());
                    snapshot.rdl("Test").unwrap()
                })
            })
            .collect::<Vec<_>>();
        let outputs = tasks
            .into_iter()
            .map(|task| task.join().unwrap())
            .collect::<Vec<_>>();
        assert!(outputs.windows(2).all(|pair| pair[0] == pair[1]));
    });
    clang_sys::set_library(saved);
}

#[test]
fn source_scope_and_family_boundaries_fail_explicitly() {
    let compiler = Compiler::new().unwrap();
    let cases = [
        (
            "typedef int Value; broken source",
            &["Value"][..],
            Stage::Source,
        ),
        (
            "struct Record { virtual void Method(); }; typedef Record *Pointer;",
            &["Pointer"][..],
            Stage::Capture,
        ),
        (
            "inline void Function() {}",
            &["Function"][..],
            Stage::Capture,
        ),
        (
            "typedef int (*Callback)();",
            &["Callback"][..],
            Stage::Capture,
        ),
        (
            "enum __attribute__((flag_enum)) Enumeration { One };",
            &["Enumeration"][..],
            Stage::Capture,
        ),
        ("typedef int Value;", &["Missing"][..], Stage::Scope),
        ("typedef int Value;", &[][..], Stage::Scope),
        (
            "typedef int &&Reference;",
            &["Reference"][..],
            Stage::Capture,
        ),
        (
            "typedef int Value __attribute__((aligned(16)));",
            &["Value"][..],
            Stage::Capture,
        ),
        (
            "typedef int Value; void Bad() { unknown_symbol; }",
            &["Value"][..],
            Stage::Source,
        ),
        (
            "typedef int Value; typedef int Value __attribute__((annotate(\"contract\")));",
            &["Value"][..],
            Stage::Capture,
        ),
    ];
    for (source, required, stage) in cases {
        let error = compiler
            .capture(&input(source, Target::X64, required))
            .unwrap_err();
        assert_eq!(error.stage, stage, "{source}: {error}");
    }
    let mut wrong = input("typedef int Value;", Target::X64, &["Value"]);
    wrong
        .arguments
        .push("--target=x86_64-unknown-linux-gnu".into());
    assert_eq!(compiler.capture(&wrong).unwrap_err().stage, Stage::Capture);
    let snapshot = compiler
        .capture(&input(
            "typedef int Value; void Later();",
            Target::X64,
            &["Value"],
        ))
        .unwrap();
    assert!(
        snapshot
            .roots
            .iter()
            .any(|root| root.qualified_name == "Later" && !root.required)
    );
    assert!(!snapshot.rdl("Test").unwrap().contains("Later"));
}

#[test]
fn exact_file_identity_and_include_order_are_checked() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("input\\scope");
    let a = directory.join("a\\shared.h");
    let b = directory.join("b\\shared.h");
    let include =
        |path: &Path| format!("#include {:?}\n", path.to_str().unwrap().replace('\\', "/"));
    let compiler = Compiler::new().unwrap();
    let mut profile = input(
        &(include(&a) + &include(&b)),
        Target::X64,
        &["One::Value", "Two::Value"],
    );
    profile.roots = vec![a.clone(), b.clone()];
    let expected = include_str!("../input/scope.rdl").replace("\r\n", "\n");
    let snapshot = compiler.capture(&profile).unwrap();
    assert_eq!(snapshot.rdl("Test").unwrap(), expected);
    let identities = snapshot
        .evidence()
        .map(|item| item.usr.as_str())
        .collect::<Vec<_>>();
    assert_eq!(identities.len(), 2);
    assert_ne!(identities[0], identities[1]);
    profile.source = include(&b) + &include(&a);
    profile.roots.reverse();
    assert_eq!(
        compiler.capture(&profile).unwrap().rdl("Test").unwrap(),
        expected
    );
    profile.required = ["One::Value".into()].into();
    profile.roots = vec![b];
    assert_eq!(compiler.capture(&profile).unwrap_err().stage, Stage::Scope);
}

#[test]
fn output_rejects_unrepresented_qualification_and_names() {
    let compiler = Compiler::new().unwrap();
    for (source, name) in [
        ("typedef int *__restrict Pointer;", "Pointer"),
        ("typedef int self;", "self"),
        ("typedef int Self;", "Self"),
        ("typedef int _;", "_"),
    ] {
        let snapshot = compiler
            .capture(&input(source, Target::X64, &[name]))
            .unwrap();
        let error = snapshot.rdl("Test").unwrap_err();
        assert_eq!(error.stage, Stage::Representation);
        assert!(error.message.contains("capture.cpp"));
    }
    let snapshot = compiler
        .capture(&input("typedef int Value;", Target::X64, &["Value"]))
        .unwrap();
    for namespace in ["", "Test..Native", "self", "Invalid-Name"] {
        assert_eq!(
            snapshot.rdl(namespace).unwrap_err().stage,
            Stage::Representation
        );
    }
    let namespace = std::iter::repeat_n("Deep", 65)
        .collect::<Vec<_>>()
        .join(".");
    assert_eq!(
        snapshot.rdl(&namespace).unwrap_err().stage,
        Stage::Representation
    );
    let mut source = String::from("typedef int Alias0;");
    for index in 1..70 {
        source.push_str(&format!("typedef Alias{} Alias{index};", index - 1));
    }
    assert_eq!(
        compiler
            .capture(&input(&source, Target::X64, &["Alias69"]))
            .unwrap_err()
            .stage,
        Stage::Capture
    );
}

#[test]
fn header_identity_and_order_are_not_publication_policy() {
    let compiler = Compiler::new().unwrap();
    let fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("input")
        .join("aliases.h");
    let mut header = input(
        &format!(
            "#include {:?}\n",
            fixture.to_str().unwrap().replace('\\', "/")
        ),
        Target::X64,
        REQUIRED,
    );
    header.roots.push(fixture);
    let expected = compiler
        .capture(&input(
            include_str!("../input/aliases.h"),
            Target::X64,
            REQUIRED,
        ))
        .unwrap()
        .rdl("Test")
        .unwrap();
    assert_eq!(
        compiler.capture(&header).unwrap().rdl("Test").unwrap(),
        expected
    );
    let a = compiler
        .capture(&input(
            "typedef int A; typedef A B;",
            Target::X64,
            &["B", "A"],
        ))
        .unwrap();
    let b = compiler
        .capture(&input(
            "typedef int A; typedef A B;",
            Target::X64,
            &["A", "B"],
        ))
        .unwrap();
    assert_eq!(a.rdl("Test").unwrap(), b.rdl("Test").unwrap());
    header.roots.push(PathBuf::from("absent.h"));
    assert_eq!(compiler.capture(&header).unwrap_err().stage, Stage::Scope);
}

#[test]
fn existing_anchors_have_explicit_selected_and_deferred_families() {
    let compiler = Compiler::new().unwrap();
    let snapshot = compiler
        .capture(&input(
            include_str!("..\\..\\clang\\input\\alias_chains.h"),
            Target::X64,
            &["TEXT", "WTEXT"],
        ))
        .unwrap();
    assert_eq!(
        snapshot.rdl("Test").unwrap(),
        include_str!("../input/legacy_aliases.rdl").replace("\r\n", "\n")
    );
    for name in ["Strings", "Text"] {
        assert!(
            snapshot
                .roots
                .iter()
                .any(|root| root.qualified_name == name && !root.required)
        );
    }
    let source = include_str!("..\\..\\clang\\input\\smoke.h");
    let snapshot = compiler
        .capture(&input(source, Target::X64, &["Point"]))
        .unwrap();
    assert_eq!(
        snapshot.rdl("Test").unwrap(),
        include_str!("../input/point.rdl").replace("\r\n", "\n")
    );
    let callable = compiler
        .capture(&input(source, Target::X64, &["UsePoint"]))
        .unwrap();
    assert_eq!(callable.functions().count(), 1);
    assert!(
        callable
            .evidence()
            .any(|item| item.qualified_name == "Point")
    );
    assert_eq!(
        callable.rdl("Test").unwrap_err().stage,
        Stage::Representation
    );
}

#[test]
fn pinned_winnt_scalar_profile_on_three_targets() {
    let repository = Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(4)
        .unwrap();
    let sdk = std::env::var_os("WINDOWS_CLANG2_SDK_INCLUDE").map_or_else(||
        PathBuf::from(std::env::var_os("USERPROFILE").unwrap())
            .join(".nuget\\packages\\microsoft.windows.sdk.cpp\\10.0.28000.2270\\c\\Include\\10.0.28000.0"), PathBuf::from);
    let resource = std::env::var_os("CLANG_RESOURCE_DIR").map_or_else(
        || repository.join("target\\tool-clang\\clang-resource\\22.1.8"),
        PathBuf::from,
    );
    assert!(
        sdk.join("um\\winnt.h").is_file(),
        "missing pinned SDK headers: {}",
        sdk.display()
    );
    assert!(
        resource.join("include\\intrin.h").is_file(),
        "missing compiler resource headers: {}",
        resource.display()
    );
    let compiler = Compiler::new().unwrap();
    for target in [Target::X86, Target::X64, Target::Arm64] {
        let mut profile = input(
            include_str!("../input/winnt.h"),
            target,
            &[
                "CHAR",
                "LONG",
                "LONGLONG",
                "PVOID",
                "SHORT",
                "ULONG",
                "ULONGLONG",
                "WCHAR",
            ],
        );
        profile
            .arguments
            .extend(["-resource-dir".into(), resource.to_str().unwrap().into()]);
        for directory in ["ucrt", "um", "shared"] {
            profile
                .arguments
                .push(format!("-I{}", sdk.join(directory).display()));
        }

        profile.roots.push(sdk.join("um\\winnt.h"));
        profile.roots.push(sdk.join("shared\\minwindef.h"));
        let snapshot = compiler.capture(&profile).unwrap();
        let rdl = snapshot.rdl("Test").unwrap();
        assert_eq!(
            rdl,
            include_str!("../input/winnt.rdl").replace("\r\n", "\n")
        );
        let path = Path::new(env!("OUT_DIR")).join(format!("winnt_{target:?}.winmd"));
        windows_rdl::reader()
            .input_text(&rdl)
            .output(&path)
            .write()
            .unwrap();
        let index = Index::read(&path).unwrap();
        index.validate_native_contracts(None).unwrap();
        assert_eq!(
            index
                .get("Test", "PVOID")
                .next()
                .unwrap()
                .fields()
                .next()
                .unwrap()
                .ty(),
            Type::PtrMut(Box::new(Type::Void), 1)
        );
        let evidence = snapshot
            .evidence()
            .find(|item| item.qualified_name == "PVOID")
            .unwrap();
        assert_eq!(
            evidence.layout.unwrap().size,
            u64::from(target.pointer_bits() / 8)
        );
    }
}

#[test]
fn nominal_names_do_not_turn_into_rdl_builtins() {
    let compiler = Compiler::new().unwrap();
    let snapshot = compiler
        .capture(&input(
            include_str!("../input/builtin_names.h"),
            Target::X64,
            &["Text", "Number", "Character"],
        ))
        .unwrap();
    let rdl = snapshot.rdl("Test").unwrap();
    assert_eq!(
        rdl,
        include_str!("../input/builtin_names.rdl").replace("\r\n", "\n")
    );
    let path = Path::new(env!("OUT_DIR")).join("builtin_names.winmd");
    windows_rdl::reader()
        .input_text(&rdl)
        .output(&path)
        .write()
        .unwrap();
    let index = Index::read(&path).unwrap();
    for (name, target) in [
        ("Text", "String"),
        ("Number", "i32"),
        ("Character", "Char16"),
    ] {
        assert_eq!(
            index
                .get("Test", name)
                .next()
                .unwrap()
                .fields()
                .next()
                .unwrap()
                .ty(),
            Type::value_named("Test", target)
        );
    }
    let snapshot = compiler
        .capture(&input("struct Guid;", Target::X64, &["Guid"]))
        .unwrap();
    assert_eq!(
        snapshot.rdl("System").unwrap_err().stage,
        Stage::Representation
    );
}

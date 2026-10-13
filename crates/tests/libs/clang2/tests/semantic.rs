#![cfg(target_arch = "x86_64")]

use std::collections::BTreeMap;
use std::path::Path;
use windows_clang2::*;

mod support;

fn input(source: &str, target: Target) -> Input {
    let mut input = Input::new("semantic.cpp", source, target);
    input
        .required
        .extend(["Payload", "Mode", "Send", "Root", "Count"].map(String::from));
    input
}

#[test]
fn compiled_mutations_reject_against_original_native_contracts() {
    if support::worker("compiled_mutations_reject_against_original_native_contracts") {
        return;
    }
    let compiler = Compiler::with_layout_bridge(
        Path::new(&std::env::var_os("WINDOWS_CLANG2_LAYOUT_NATIVE").unwrap())
            .join("windows_clang_layout.dll"),
    )
    .unwrap();
    for target in [Target::X86, Target::X64, Target::Arm64] {
        let snapshot = compiler
            .capture(&input(include_str!("..\\input\\semantic_gate.h"), target))
            .unwrap();
        let imports = snapshot
            .functions()
            .map(|(_, function)| {
                (
                    function.linker_name.clone(),
                    Import {
                        dll: "fixture.dll".into(),
                        export: "Send".into(),
                    },
                )
            })
            .collect::<BTreeMap<_, _>>();
        let rdl = snapshot.rdl_with_imports("Test", &imports).unwrap();
        let directory = Path::new(env!("OUT_DIR")).join(format!("semantic_{target:?}"));
        std::fs::create_dir_all(&directory).unwrap();
        let vocabulary = directory.join("vocabulary.winmd");
        windows_rdl::reader()
            .input_text(include_str!("..\\..\\..\\..\\..\\metadata\\metadata.rdl"))
            .output(&vocabulary)
            .write()
            .unwrap();
        let wide = if target == Target::X86 {
            "type ULONG_PTR = u32;"
        } else {
            "type ULONG_PTR = u64;"
        };
        for (name, candidate, valid) in [
            ("original", rdl.clone(), true),
            (
                "signedness",
                rdl.replace("marker: u32", "marker: i32"),
                false,
            ),
            (
                "alias-width",
                rdl.replace(wide, "type ULONG_PTR = u16;"),
                false,
            ),
            ("array-count", rdl.replace("[i8; 3]", "[i8; 4]"), false),
            (
                "pointee-const",
                rdl.replace("*const Test::Payload", "*mut Test::Payload"),
                false,
            ),
            (
                "pointer-depth",
                rdl.replace("*mut *const void", "*const void"),
                false,
            ),
            (
                "macro-value",
                rdl.replace("Count: i32 = 7", "Count: i32 = 8"),
                false,
            ),
            ("enum-value", rdl.replace("Small = 1", "Small = 2"), false),
            (
                "guid",
                rdl.replace(
                    "00112233445566778899aabbccddeeff",
                    "10112233445566778899aabbccddeeff",
                ),
                false,
            ),
            (
                "slots",
                rdl.replace("#[slot(0)]", "#[slot(2)]")
                    .replace("#[slot(1)]", "#[slot(0)]")
                    .replace("#[slot(2)]", "#[slot(1)]"),
                false,
            ),
            (
                "extra",
                rdl.replace("mod Test {", "mod Test { const Extra: i32 = 0;"),
                false,
            ),
            (
                "packing-request",
                rdl.replace("struct Payload", "#[packed(8)] struct Payload"),
                false,
            ),
            (
                "alignment-request",
                rdl.replace("struct Payload", "#[align(4)] struct Payload"),
                false,
            ),
        ] {
            if !valid {
                assert_ne!(
                    candidate, rdl,
                    "{name}: mutation did not affect the fixture"
                );
            }
            let binary = directory.join(format!("{name}.winmd"));
            windows_rdl::reader()
                .reference(&vocabulary)
                .input_text(&candidate)
                .output(&binary)
                .write()
                .unwrap();
            let index = windows_metadata::reader::Index::read(&binary).unwrap();
            assert_eq!(
                snapshot.verify_winmd("Test", &index).is_ok(),
                valid,
                "{target:?}: {name}"
            );
        }
    }
}

#[test]
fn architecture_agreement_rejects_unapproved_type_and_presence_changes() {
    if support::worker("architecture_agreement_rejects_unapproved_type_and_presence_changes") {
        return;
    }
    let compiler = Compiler::with_layout_bridge(
        Path::new(&std::env::var_os("WINDOWS_CLANG2_LAYOUT_NATIVE").unwrap())
            .join("windows_clang_layout.dll"),
    )
    .unwrap();
    let source = include_str!("..\\input\\semantic_gate.h");
    let x86 = compiler.capture(&input(source, Target::X86)).unwrap();
    for target in [Target::X64, Target::Arm64] {
        let original = compiler.capture(&input(source, target)).unwrap();
        x86.agree_architecture(&original).unwrap();
        for changed in [
            source.replace("unsigned marker", "int marker"),
            source.replace("char name[3]", "char name[4]"),
            source.replace("Small = 1", "Small = 2"),
            source.replace("unsigned long long ULONG_PTR", "unsigned short ULONG_PTR"),
            source.replace("__stdcall Send", "__stdcall Renamed"),
        ] {
            let mut changed_input = input(&changed, target);
            if changed.contains("Renamed") {
                changed_input.required.remove("Send");
                changed_input.required.insert("Renamed".into());
            }
            let changed = compiler.capture(&changed_input).unwrap();
            assert!(x86.agree_architecture(&changed).is_err());
        }
    }
}

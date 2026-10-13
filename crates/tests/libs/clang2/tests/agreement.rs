#![cfg(target_arch = "x86_64")]

use std::collections::BTreeMap;
use std::path::Path;
use windows_clang2::*;

mod support;

fn compiler() -> Compiler {
    Compiler::with_layout_bridge(
        Path::new(&std::env::var_os("WINDOWS_CLANG2_LAYOUT_NATIVE").unwrap())
            .join("windows_clang_layout.dll"),
    )
    .unwrap()
}

fn input(source: &str, target: Target, combined: bool) -> Input {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("input");
    let mut input = Input::new("agreement.cpp", source, target);
    input.arguments.push(format!("-I{}", directory.display()));
    input
        .required
        .extend(["Packet", "Status", "Invoke", "Limit", "Derived"].map(str::to_string));
    input
        .roots
        .extend(["agreement_a.h", "interfaces.h"].map(|name| directory.join(name)));
    if combined {
        input
            .required
            .extend(["Extra", "Shapes::Owner", "Shapes::Peer"].map(str::to_string));
        input
            .roots
            .extend(["agreement_b.h", "nested_records.h"].map(|name| directory.join(name)));
    }
    input
}

fn rdl(snapshot: &Snapshot) -> String {
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
        .collect::<BTreeMap<_, _>>();
    snapshot.rdl_with_imports("Test", &imports).unwrap()
}

#[test]
fn original_native_graph_agrees_across_input_scope_order_and_lifetime() {
    if support::worker("original_native_graph_agrees_across_input_scope_order_and_lifetime") {
        return;
    }
    let compiler = compiler();
    for target in [Target::X86, Target::X64, Target::Arm64] {
        let separate = compiler
            .capture(&input("#include \"agreement_a.h\"\n", target, false))
            .unwrap();
        let combined = compiler
            .capture(&input(
                "#include \"agreement_a.h\"\n#include \"agreement_b.h\"\n",
                target,
                true,
            ))
            .unwrap();
        let reversed = compiler
            .capture(&input(
                "#include \"agreement_b.h\"\n#include \"agreement_a.h\"\n",
                target,
                true,
            ))
            .unwrap();
        separate.agree(&combined).unwrap();
        combined.agree(&reversed).unwrap();
        reversed.agree(&combined).unwrap();
        assert_eq!(rdl(&combined), rdl(&reversed));
        assert!(combined.evidence().any(|decl| {
            separate
                .evidence()
                .any(|other| decl.qualified_name == other.qualified_name && decl.id != other.id)
        }));
        assert_eq!(
            combined.agree(&separate).unwrap_err().stage,
            Stage::Agreement
        );
    }
    let separate = compiler
        .capture(&input("#include \"agreement_a.h\"\n", Target::X64, false))
        .unwrap();
    let combined = compiler
        .capture(&input("#include \"agreement_b.h\"\n", Target::X64, true))
        .unwrap();
    drop(compiler);
    separate.agree(&combined).unwrap();
}

#[test]
fn incompatible_native_facts_reject_even_when_rdl_is_identical() {
    if support::worker("incompatible_native_facts_reject_even_when_rdl_is_identical") {
        return;
    }
    let compiler = compiler();
    let baseline = compiler
        .capture(&input("#include \"agreement_a.h\"\n", Target::X86, false))
        .unwrap();
    for change in [
        "CHANGE_SCALAR",
        "CHANGE_PACKING",
        "CHANGE_ENUM",
        "CHANGE_CONVENTION",
        "CHANGE_SIGNATURE",
        "CHANGE_CONSTANT",
        "CHANGE_GUID",
        "CHANGE_SLOT",
    ] {
        let mut input = input("#include \"agreement_a.h\"\n", Target::X86, false);
        input.arguments.push(format!("-D{change}"));
        let changed = compiler.capture(&input).unwrap();
        let error = baseline.agree(&changed).unwrap_err();
        assert_eq!(error.stage, Stage::Agreement, "{change}: {error}");
        if change == "CHANGE_SCALAR" {
            assert_eq!(rdl(&baseline), rdl(&changed));
        }
    }
    let other_target = compiler
        .capture(&input("#include \"agreement_a.h\"\n", Target::X64, false))
        .unwrap();
    assert_eq!(
        baseline.agree(&other_target).unwrap_err().stage,
        Stage::Agreement
    );
}

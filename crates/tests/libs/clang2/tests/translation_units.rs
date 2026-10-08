use std::collections::BTreeMap;
use std::path::Path;
use windows_clang2::{Input, ProjectionOptions, capture, capture_report};
use windows_metadata::{Type, reader::*};
#[path = "../sdk.rs"]
#[allow(dead_code)]
mod sdk;

fn inputs(sources: &[String], reverse: bool, rename: bool) -> Vec<Input> {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("input\\translation_units");
    let mut inputs: Vec<_> = sources
        .iter()
        .enumerate()
        .map(|(index, source)| {
            let index = if rename {
                sources.len() - 1 - index
            } else {
                index
            };
            Input::new(
                directory.join(format!("{index:03}.hpp")).to_str().unwrap(),
                source,
            )
        })
        .collect();
    if reverse {
        inputs.reverse();
    }
    inputs
}

fn options() -> ProjectionOptions {
    let mut options = ProjectionOptions::new("Test");
    options.library = Some("test.dll".into());
    options
}

const ARGS: &[&str] = &["-x", "c++", "--target=x86_64-pc-windows-msvc"];
const INCLUDE: &str = "#include \"shared.h\"\n#include \"shared.h\"\n";

fn check_metadata(name: &str, rdl: &str) {
    let output = Path::new(env!("OUT_DIR")).join(format!("{name}.winmd"));
    windows_rdl::reader()
        .input_text(rdl)
        .output(&output)
        .write()
        .unwrap();
    let index = Index::read(output).unwrap();
    let shared = index.expect("Test", "Shared");
    assert_eq!(shared.fields().count(), 1);
    assert_eq!(shared.fields().next().unwrap().ty(), Type::I32);
    let packet = index.expect("Test", "Packet");
    let fields: Vec<_> = packet.fields().map(|field| field.ty()).collect();
    assert_eq!(
        fields,
        [
            Type::PtrMut(Box::new(Type::value_named("Test", "Shared")), 1),
            Type::U32,
        ]
    );
    let Item::Fn(function) = index.expect_item("Test", "Use") else {
        panic!()
    };
    assert_eq!(
        function.signature(&[]).types,
        [
            Type::PtrMut(Box::new(Type::value_named("Test", "Packet")), 1),
            Type::U32,
        ]
    );
    assert!(
        function
            .params()
            .all(|parameter| parameter.direction() == ParamDirection::Input)
    );
}

#[test]
fn repeated_headers_keep_all_observations_and_emit_one_closed_plan() {
    let mut baseline = None;
    for count in [1, 4, 16, 64] {
        for reverse in [false, true] {
            let sources = vec![INCLUDE.to_string(); count];
            let report = capture_report(inputs(&sources, reverse, false), ARGS, &["Use"]).unwrap();
            assert_eq!(report.parses, count);
            assert!(report.rejected.is_empty());
            let snapshot = report.snapshot.unwrap();
            let resolved = snapshot.resolve().unwrap();
            assert_eq!(resolved.group_count(), 4);
            assert_eq!(resolved.report().observations, 4 * count);
            assert_eq!(resolved.report().declaration_pairs, 4 * (count - 1));
            let plan = resolved.project(&options()).unwrap();
            assert!(plan.omitted().is_empty());
            let output = (plan.rdl(), plan.rdl_by_header().unwrap());
            let partitions: Vec<_> = output
                .1
                .keys()
                .map(|header| Path::new(header).file_name().unwrap().to_str().unwrap())
                .collect();
            assert_eq!(partitions, ["dependency.h", "shared.h"]);
            if let Some(baseline) = &baseline {
                assert_eq!(&output, baseline);
            } else {
                let expected =
                    Path::new(env!("CARGO_MANIFEST_DIR")).join("expected\\translation_units.rdl");
                if std::env::var_os("UPDATE_EXPECT").is_some() {
                    std::fs::write(&expected, &output.0).unwrap();
                }
                assert_eq!(output.0, std::fs::read_to_string(expected).unwrap());
                check_metadata("translation_units", &output.0);
                baseline = Some(output);
            }
        }
    }
}

#[test]
fn dependency_completions_survive_many_forward_only_tus() {
    let mut sources = vec![format!("#define FORWARD_ONLY\n{INCLUDE}"); 16];
    sources.push(INCLUDE.to_string());
    let mut baseline = None;
    for reverse in [false, true] {
        for rename in [false, true] {
            let snapshot = capture(inputs(&sources, reverse, rename), ARGS, &["Use"]).unwrap();
            let resolved = snapshot.resolve().unwrap();
            assert_eq!(resolved.group_count(), 4);
            assert_eq!(resolved.report().observations, 4 * sources.len());
            assert!(resolved.report().incomplete.is_empty());
            let plan = resolved.project(&options()).unwrap();
            let output = (plan.rdl(), plan.rdl_by_header().unwrap());
            if let Some(baseline) = &baseline {
                assert_eq!(&output, baseline);
            } else {
                check_metadata("translation_unit_completions", &output.0);
                baseline = Some(output);
            }
        }
    }
}

#[test]
fn include_order_conflicts_in_dependency_only_headers_do_not_merge() {
    let sources = [
        format!("#include \"wide.h\"\n{INCLUDE}"),
        format!("{INCLUDE}\n#include \"wide.h\"\n"),
    ];
    for reverse in [false, true] {
        for rename in [false, true] {
            let inputs = inputs(&sources, reverse, rename);
            let units: Vec<_> = inputs.iter().map(|input| input.name.clone()).collect();
            let snapshot = capture(inputs, ARGS, &["Use"]).unwrap();
            for error in [
                snapshot.resolve().err().unwrap(),
                snapshot.assess().err().unwrap(),
            ] {
                let error = error.to_string();
                assert!(
                    error.contains("conflicting native declarations `Shared`"),
                    "{error}"
                );
                assert!(error.contains("dependency.h:"), "{error}");
                for unit in &units {
                    assert!(error.contains(&format!("TU {unit}")), "{error}");
                }
            }
        }
    }
}

#[test]
fn shared_header_contract_conflicts_do_not_use_tu_priority() {
    let sources = [
        INCLUDE.to_string(),
        format!("#define DIRECTION \"_Out_\"\n{INCLUDE}"),
    ];
    for reverse in [false, true] {
        for rename in [false, true] {
            let snapshot = capture(inputs(&sources, reverse, rename), ARGS, &["Use"]).unwrap();
            assert!(snapshot.resolve().is_err());
            assert!(snapshot.assess().is_err());
        }
    }
}

#[test]
fn sdk_and_wdk_enum_profiles_are_not_silently_unioned() {
    let arguments = sdk::wdk_arguments("--target=x86_64-pc-windows-msvc");
    let arguments: Vec<_> = arguments.iter().map(String::as_str).collect();
    let profiles = [
        Input::new(
            "sdk.hpp",
            include_str!("../input/translation_units/sdk_profile.h"),
        ),
        Input::new(
            "wdk.hpp",
            include_str!("../input/translation_units/wdk_profile.h"),
        ),
    ];
    let mut members = vec![];
    for profile in &profiles {
        let snapshot = capture([profile.clone()], &arguments, &["FILE_INFORMATION_CLASS"]).unwrap();
        let plan = snapshot.resolve().unwrap().project(&options()).unwrap();
        let output = Path::new(env!("OUT_DIR")).join(format!("{}.winmd", profile.name));
        windows_rdl::reader()
            .input_text(&plan.rdl())
            .output(&output)
            .write()
            .unwrap();
        let index = Index::read(output).unwrap();
        members.push(
            index
                .expect("Test", "_FILE_INFORMATION_CLASS")
                .fields()
                .filter_map(|field| {
                    field
                        .constant()
                        .map(|value| (field.name().to_string(), value.value()))
                })
                .collect::<BTreeMap<_, _>>(),
        );
    }
    assert!(members[1].len() > members[0].len());
    for (name, value) in &members[0] {
        assert_eq!(members[1].get(name), Some(value), "{name}");
    }
    for reverse in [false, true] {
        let inputs: Vec<_> = if reverse {
            profiles.iter().cloned().rev().collect()
        } else {
            profiles.to_vec()
        };
        let snapshot = capture(inputs, &arguments, &["FILE_INFORMATION_CLASS"]).unwrap();
        for error in [
            snapshot.resolve().err().unwrap(),
            snapshot.assess().err().unwrap(),
        ] {
            let error = error.to_string();
            assert!(error.contains("conflicting native declarations"), "{error}");
            assert!(error.contains("TU sdk.hpp"), "{error}");
            assert!(error.contains("TU wdk.hpp"), "{error}");
        }
    }
}

#[test]
fn shared_sdk_wdk_enum_profiles_have_explicit_outcomes() {
    let arguments = sdk::wdk_arguments("--target=x86_64-pc-windows-msvc");
    let arguments: Vec<_> = arguments.iter().map(String::as_str).collect();
    let profiles = [
        Input::new(
            "sdk.hpp",
            include_str!("../input/translation_units/sdk_profile.h"),
        ),
        Input::new(
            "wdk.hpp",
            include_str!("../input/translation_units/wdk_profile.h"),
        ),
    ];
    let headers = [
        vec![sdk::include().join("um").join("winternl.h")],
        vec![
            sdk::wdk_include().join("km").join("wdm.h"),
            sdk::wdk_include().join("km").join("ntifs.h"),
        ],
    ];
    let names: Vec<_> = profiles
        .iter()
        .zip(&headers)
        .map(|(profile, headers)| {
            windows_clang2::discover(
                [profile.clone()],
                &arguments,
                &headers
                    .iter()
                    .map(|header| header.to_str().unwrap())
                    .collect::<Vec<_>>(),
            )
            .unwrap()
            .into_iter()
            .filter(|declaration| declaration.kind == "EnumDecl")
            .map(|declaration| declaration.name)
            .collect::<std::collections::BTreeSet<_>>()
        })
        .collect();
    let names: Vec<_> = names[0]
        .intersection(&names[1])
        .map(String::as_str)
        .collect();
    assert!(!names.is_empty());
    let snapshots: Vec<_> = profiles
        .iter()
        .map(|profile| capture([profile.clone()], &arguments, &names).unwrap())
        .collect();
    let indices: Vec<_> = snapshots
        .iter()
        .enumerate()
        .map(|(profile, snapshot)| {
            let plan = snapshot.resolve().unwrap().project(&options()).unwrap();
            let output = Path::new(env!("OUT_DIR")).join(format!("enum_profile_{profile}.winmd"));
            windows_rdl::reader()
                .input_text(&plan.rdl())
                .output(&output)
                .write()
                .unwrap();
            Index::read(output).unwrap()
        })
        .collect();
    let mut report = String::from(
        "enum\tsdk_members\twdk_members\tprojected_relation\tnative_agreement\tconflicts\n",
    );
    for name in names {
        let enums: Vec<_> = indices
            .iter()
            .map(|index| index.expect("Test", name))
            .collect();
        let members: Vec<_> = enums
            .iter()
            .map(|ty| {
                ty.fields()
                    .filter_map(|field| {
                        field
                            .constant()
                            .map(|value| (field.name().to_string(), value.value()))
                    })
                    .collect::<BTreeMap<_, _>>()
            })
            .collect();
        let conflicts: Vec<_> = members[0]
            .iter()
            .filter_map(|(name, value)| {
                members[1]
                    .get(name)
                    .filter(|other| *other != value)
                    .map(|other| format!("{name}: {value:?} -> {other:?}"))
            })
            .collect();
        let relation = if enums[0].underlying_type() != enums[1].underlying_type() {
            "different representations"
        } else if !conflicts.is_empty() {
            "conflicting member values"
        } else if members[0] == members[1] {
            "same emitted members"
        } else if members[0]
            .iter()
            .all(|(name, value)| members[1].get(name) == Some(value))
        {
            "candidate WDK extension"
        } else if members[1]
            .iter()
            .all(|(name, value)| members[0].get(name) == Some(value))
        {
            "candidate SDK extension"
        } else {
            "different member sets"
        };
        let snapshot = capture(profiles.clone(), &arguments, &[name]).unwrap();
        let agreement = match snapshot.resolve() {
            Ok(_) => "agrees",
            Err(error) => {
                assert!(
                    error
                        .to_string()
                        .contains("conflicting native declarations"),
                    "{name}: {error}"
                );
                "rejects"
            }
        };
        let conflicts = if conflicts.is_empty() {
            "-".into()
        } else {
            conflicts.join("; ")
        };
        report.push_str(&format!(
            "{name}\t{}\t{}\t{relation}\t{agreement}\t{conflicts}\n",
            members[0].len(),
            members[1].len(),
        ));
    }
    let expected = Path::new(env!("CARGO_MANIFEST_DIR")).join("expected\\enum_profiles.tsv");
    if std::env::var_os("UPDATE_EXPECT").is_some() {
        std::fs::write(&expected, &report).unwrap();
    }
    assert_eq!(
        report,
        std::fs::read_to_string(expected)
            .unwrap()
            .replace("\r\n", "\n")
    );
}

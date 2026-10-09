#![cfg(target_env = "msvc")]

use std::path::Path;
use windows_clang2::{Input, ProjectionOptions, capture, capture_report};
#[path = "../sdk.rs"]
#[allow(dead_code)]
mod sdk;

#[allow(dead_code, non_upper_case_globals, non_snake_case)]
mod values {
    include!(concat!(env!("OUT_DIR"), "/guid_expressions.rs"));
}
#[allow(dead_code, non_upper_case_globals, non_snake_case)]
mod sys {
    include!(concat!(env!("OUT_DIR"), "/guid_expressions_sys.rs"));
}
#[allow(dead_code, non_upper_case_globals, non_snake_case)]
mod sdk_values {
    include!(concat!(env!("OUT_DIR"), "/sdk_expressions.rs"));
}
#[allow(dead_code, non_upper_case_globals, non_snake_case)]
mod sdk_sys {
    include!(concat!(env!("OUT_DIR"), "/sdk_expressions_sys.rs"));
}

const SOURCE: &str = include_str!("../input/guid_expressions.h");

#[test]
#[ignore = "manual SDK multi-TU resource gate"]
fn repeated_sdk_uuid_headers_keep_all_observations() {
    let arguments = sdk::arguments("--target=x86_64-pc-windows-msvc");
    let args: Vec<_> = arguments.iter().map(String::as_str).collect();
    let mut baseline = None;
    for count in [1, 2, 4] {
        let start = std::time::Instant::now();
        let inputs = (0..count)
            .map(|index| Input::new(format!("repeat-{index}.hpp"), sdk::SDK_EXPRESSION_SOURCE));
        let report = capture_report(inputs, &args, sdk::SDK_EXPRESSION_ROOTS).unwrap();
        assert!(report.rejected.is_empty());
        assert_eq!(report.parses, count * 3);
        let snapshot = report.snapshot.unwrap();
        let resolved = snapshot.resolve().unwrap();
        assert_eq!(resolved.group_count(), 4);
        assert_eq!(resolved.report().observations, count * 5);
        assert_eq!(resolved.report().declaration_pairs, count * 5 - 4);
        let rdl = resolved
            .project(&ProjectionOptions::new("Test"))
            .unwrap()
            .rdl();
        if let Some(baseline) = &baseline {
            assert_eq!(baseline, &rdl);
        } else {
            baseline = Some(rdl);
        }
        println!(
            "{count} SDK TUs: {} parses, {} observations, {:.3}s",
            report.parses,
            resolved.report().observations,
            start.elapsed().as_secs_f64(),
        );
    }
}

unsafe extern "C" {
    fn CheckGuidExpressions(
        ty: *const core::ffi::c_void,
        pointer: *const core::ffi::c_void,
        parens: *const core::ffi::c_void,
        chain: *const core::ffi::c_void,
        literal: *const core::ffi::c_void,
        copied: *const core::ffi::c_void,
        alias: *const core::ffi::c_void,
    ) -> i32;
    fn CheckSdkExpressionValues(
        ks: *const core::ffi::c_void,
        media: *const core::ffi::c_void,
        codec: *const core::ffi::c_void,
    ) -> i32;
}

#[test]
fn native_guid_expressions_match_compiler_storage() {
    for pointers in [
        [
            core::ptr::from_ref(&values::TYPE_ID).cast(),
            core::ptr::from_ref(&values::POINTER_ID).cast(),
            core::ptr::from_ref(&values::PAREN_ID).cast(),
            core::ptr::from_ref(&values::CHAIN_ID).cast(),
            core::ptr::from_ref(&values::LiteralId).cast(),
            core::ptr::from_ref(&values::CopiedId).cast(),
            core::ptr::from_ref(&values::LITERAL_ALIAS).cast(),
        ],
        [
            core::ptr::from_ref(&sys::TYPE_ID).cast(),
            core::ptr::from_ref(&sys::POINTER_ID).cast(),
            core::ptr::from_ref(&sys::PAREN_ID).cast(),
            core::ptr::from_ref(&sys::CHAIN_ID).cast(),
            core::ptr::from_ref(&sys::LiteralId).cast(),
            core::ptr::from_ref(&sys::CopiedId).cast(),
            core::ptr::from_ref(&sys::LITERAL_ALIAS).cast(),
        ],
    ] {
        assert_eq!(size_of_val(&values::TYPE_ID), 16);
        assert_eq!(size_of_val(&sys::TYPE_ID), 16);
        assert_eq!(
            unsafe {
                CheckGuidExpressions(
                    pointers[0],
                    pointers[1],
                    pointers[2],
                    pointers[3],
                    pointers[4],
                    pointers[5],
                    pointers[6],
                )
            },
            1
        );
    }
    for pointers in [
        [
            core::ptr::from_ref(&sdk_values::KSPROPSETID_General).cast(),
            core::ptr::from_ref(&sdk_values::KSMEDIUMSETID_MidiBus).cast(),
            core::ptr::from_ref(&sdk_values::CODECAPI_AVEncCommonFormatConstraint).cast(),
        ],
        [
            core::ptr::from_ref(&sdk_sys::KSPROPSETID_General).cast(),
            core::ptr::from_ref(&sdk_sys::KSMEDIUMSETID_MidiBus).cast(),
            core::ptr::from_ref(&sdk_sys::CODECAPI_AVEncCommonFormatConstraint).cast(),
        ],
    ] {
        assert_eq!(
            unsafe { CheckSdkExpressionValues(pointers[0], pointers[1], pointers[2]) },
            1
        );
    }
}

#[test]
fn aggregate_expression_values_agree_and_roundtrip() {
    for arch in ["i686", "x86_64", "aarch64"] {
        let target = format!("--target={arch}-pc-windows-msvc");
        let arguments = sdk::arguments(&target);
        let args: Vec<_> = arguments.iter().map(String::as_str).collect();
        for (name, source, roots) in [
            ("guid_expressions", SOURCE, sdk::EXPRESSION_ROOTS),
            (
                "sdk_expressions",
                sdk::SDK_EXPRESSION_SOURCE,
                sdk::SDK_EXPRESSION_ROOTS,
            ),
        ] {
            for reversed in [false, true] {
                let mut inputs = [Input::new("a.hpp", source), Input::new("b.hpp", source)];
                if reversed {
                    inputs.reverse();
                }
                let plan = capture(inputs, &args, roots)
                    .unwrap()
                    .resolve()
                    .unwrap()
                    .project(&ProjectionOptions::new("Test"))
                    .unwrap();
                let expected = Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("expected")
                    .join(format!("{name}.rdl"));
                if std::env::var_os("UPDATE_EXPECT").is_some() {
                    std::fs::write(&expected, plan.rdl()).unwrap();
                }
                assert_eq!(plan.rdl(), std::fs::read_to_string(expected).unwrap());
                let out = Path::new(env!("OUT_DIR"));
                let winmd = out.join(format!("{name}-{arch}-{reversed}.winmd"));
                windows_rdl::reader()
                    .input_text(&plan.rdl())
                    .reference_default()
                    .output(&winmd)
                    .write()
                    .unwrap();
                let rdl = winmd.with_extension("rdl");
                windows_rdl::writer()
                    .input(&winmd)
                    .filter("Test")
                    .output(&rdl)
                    .write()
                    .unwrap();
                let folder = out.join(format!("{name}-{arch}-{reversed}-roundtrip"));
                std::fs::create_dir_all(&folder).unwrap();
                let roundtrip = folder.join(winmd.file_name().unwrap());
                windows_rdl::reader()
                    .input(rdl)
                    .reference_default()
                    .output(&roundtrip)
                    .write()
                    .unwrap();
                assert_eq!(
                    std::fs::read(winmd).unwrap(),
                    std::fs::read(roundtrip).unwrap()
                );
            }
        }
        let renamed = SOURCE
            .replace("IdentityOwner", "UnrelatedName")
            .replace("IdentityAlias", "UnrelatedAlias");
        let plan = capture(
            [Input::new("renamed.hpp", renamed)],
            &args,
            sdk::EXPRESSION_ROOTS,
        )
        .unwrap()
        .resolve()
        .unwrap()
        .project(&ProjectionOptions::new("Test"))
        .unwrap();
        assert_eq!(
            plan.rdl(),
            std::fs::read_to_string(
                Path::new(env!("CARGO_MANIFEST_DIR")).join("expected/guid_expressions.rdl")
            )
            .unwrap()
        );
        let changed =
            format!("#define EXPRESSION_UUID \"fedcba98-abcd-8765-80ff-0001020304fd\"\n{SOURCE}");
        for reversed in [false, true] {
            let mut inputs = [Input::new("a.hpp", SOURCE), Input::new("b.hpp", &changed)];
            if reversed {
                inputs.reverse();
            }
            let error = capture(inputs, &args, &["TYPE_ID"])
                .unwrap()
                .resolve()
                .err()
                .unwrap();
            assert!(error.to_string().contains("value differs"), "{error}");
        }
    }
}

#[test]
fn unsafe_aggregate_references_and_calls_are_not_constants() {
    let source = include_str!("../input/aggregate_references_rejected.h");
    for arch in ["i686", "x86_64", "aarch64"] {
        let target = format!("--target={arch}-pc-windows-msvc");
        let args = ["-x", "c++", &target];
        for root in [
            "MUTABLE_ALIAS",
            "VOLATILE_ALIAS",
            "MISSING_ALIAS",
            "CYCLIC_ALIAS",
            "MUTABLE_MEMBER_ALIAS",
            "MUTABLE_MEMBER_COPY_ALIAS",
            "VOLATILE_MEMBER_ALIAS",
            "NESTED_MUTABLE_MEMBER_ALIAS",
            "RUNTIME_ALIAS",
            "CONTEXTUAL_ALIAS",
            "CUSTOM_ALIAS",
            "CUSTOM_COPY_ALIAS",
            "PARTIAL_ALIAS",
            "PARTIAL_LITERAL",
        ] {
            let report =
                capture_report([Input::new("limits.hpp", source)], &args, &[root]).unwrap();
            if report.rejected.contains_key(root) {
                assert!(report.snapshot.is_none(), "{root}");
            } else {
                assert!(report.snapshot.unwrap().resolve().is_err(), "{root}");
            }
        }
    }
}

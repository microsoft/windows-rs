use windows_clang2::{Input, ProjectionOptions, ReferenceKind, StringKind, TypeReference, capture};
use windows_metadata::{Type, Value, reader::*};
#[path = "../sdk.rs"]
#[allow(dead_code)]
mod sdk;

const SOURCE: &str = include_str!("../input/string_termination.h");
const ROOTS: &[&str] = &[
    "Bare", "Input", "Output", "Single", "Mixed", "Counted", "Optional", "Ordinary", "Combined",
];

#[cfg(target_env = "msvc")]
#[allow(non_snake_case, dead_code)]
mod native {
    include!(concat!(env!("OUT_DIR"), "/string_termination_native.rs"));
}

#[test]
#[cfg(target_env = "msvc")]
fn native_double_nul_calls_keep_all_strings_and_output_terminators() {
    let _: unsafe extern "C" fn(*const u16) -> u32 = native::MultiLength;
    let _: unsafe extern "C" fn(*mut u16) = native::MakeMulti;
    unsafe {
        assert_eq!(native::MultiLength([0, 0].as_ptr()), 2);
        assert_eq!(native::MultiLength([97, 0, 98, 99, 0, 0].as_ptr()), 6);
        let mut value = [0xffff; 6];
        native::MakeMulti(value.as_mut_ptr());
        assert_eq!(value, [97, 0, 98, 99, 0, 0]);
        assert_eq!(native::MultiLength(value.as_ptr()), 6);
    }
}

#[test]
fn string_terminators_preserve_phase_counts_and_raw_pointer_shapes() {
    for arch in ["i686", "x86_64", "aarch64"] {
        let target = format!("--target={arch}-pc-windows-msvc");
        for reversed in [false, true] {
            let mut inputs = [
                Input::new("a.hpp", SOURCE),
                Input::new("b.hpp", SOURCE.replace("value", "renamed")),
            ];
            if reversed {
                inputs.reverse();
            }
            let snapshot = capture(inputs, &["-x", "c++", &target], ROOTS).unwrap();
            let mut options = ProjectionOptions::new("Test");
            options.library = Some("test.dll".into());
            let rdl = snapshot.resolve().unwrap().project(&options).unwrap().rdl();
            let name = if arch == "i686" {
                "string_termination_x86"
            } else {
                "string_termination"
            };
            let expected = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("expected")
                .join(format!("{name}.rdl"));
            if std::env::var_os("UPDATE_EXPECT").is_some() {
                std::fs::write(&expected, &rdl).unwrap();
            }
            assert_eq!(rdl, std::fs::read_to_string(expected).unwrap());
            let output = std::path::Path::new(env!("OUT_DIR")).join(format!("{name}.winmd"));
            windows_rdl::reader()
                .input_text(&rdl)
                .input(sdk::projection_metadata())
                .input(
                    sdk::tools()
                        .join("..")
                        .join("..")
                        .join("metadata")
                        .join("metadata.rdl"),
                )
                .output(&output)
                .write()
                .unwrap();
            let index = Index::read(&output).unwrap();
            for (name, contracts) in [
                ("Bare", vec![(2, 0)]),
                ("Input", vec![(2, 1)]),
                ("Output", vec![(2, 2)]),
                ("Single", vec![(1, 1)]),
                ("Mixed", vec![(1, 1), (2, 2)]),
                ("Counted", vec![(2, 1)]),
                ("Optional", vec![(2, 1)]),
                ("Ordinary", vec![]),
                ("Combined", vec![(1, 1), (2, 1)]),
            ] {
                let Item::Fn(function) = index.expect_item("Test", name) else {
                    panic!()
                };
                let parameters = function
                    .params_by_sequence(if name == "Counted" { 2 } else { 1 })
                    .unwrap();
                let parameter = parameters.params()[0].unwrap();
                let attrs: Vec<_> = parameter
                    .attributes()
                    .filter(|attr| attr.name() == "NativeStringTerminationAttribute")
                    .collect();
                assert_eq!(attrs.len(), contracts.len(), "{name}");
                for (attribute, (count, phase)) in attrs.iter().zip(contracts) {
                    assert_eq!(
                        attribute.value(),
                        [
                            ("Count".into(), Value::U8(count)),
                            ("Phase".into(), Value::U8(phase))
                        ],
                        "{name}"
                    );
                }
                assert!(parameter.has_attribute("NativeAnnotationAttribute"));
                assert!(matches!(
                    function.signature(&[]).types[0],
                    Type::PtrConst(..) | Type::PtrMut(..)
                ));
                assert_eq!(parameter.is_optional(), name == "Optional");
                if name == "Counted" {
                    assert_eq!(
                        parameter.buffer_relationship(),
                        Some(BufferRelationship::ElementsParam(1))
                    );
                }
            }
            let roundtrip_path = output.with_extension("rdl");
            windows_rdl::writer()
                .input(&output)
                .filter("Test")
                .output(&roundtrip_path)
                .write()
                .unwrap();
            let roundtrip = std::fs::read_to_string(roundtrip_path).unwrap();
            assert!(
                roundtrip.contains("termination(Count = 2, Phase = 2)"),
                "{roundtrip}"
            );
            let reencoded = output.with_extension("roundtrip.winmd");
            windows_rdl::reader()
                .input_text(&roundtrip)
                .input(sdk::projection_metadata())
                .input(
                    sdk::tools()
                        .join("..")
                        .join("..")
                        .join("metadata")
                        .join("metadata.rdl"),
                )
                .output(&reencoded)
                .write()
                .unwrap();
            let roundtrip_index = Index::read(reencoded).unwrap();
            for name in ROOTS {
                let Item::Fn(before) = index.expect_item("Test", name) else {
                    panic!()
                };
                let Item::Fn(after) = roundtrip_index.expect_item("Test", name) else {
                    panic!()
                };
                let before_signature = before.signature(&[]);
                let after_signature = after.signature(&[]);
                assert_eq!(before_signature.flags, after_signature.flags);
                assert_eq!(before_signature.return_type, after_signature.return_type);
                assert_eq!(before_signature.types, after_signature.types);
                let count = if *name == "Counted" { 2 } else { 1 };
                let before = before.params_by_sequence(count).unwrap();
                let after = after.params_by_sequence(count).unwrap();
                for (before, after) in before.params().iter().zip(after.params()) {
                    let before = before.unwrap();
                    let after = after.unwrap();
                    assert_eq!(before.flags(), after.flags());
                    let attributes = |param: MethodParam<'_>| {
                        param
                            .attributes()
                            .map(|attr| (attr.name().to_string(), attr.value()))
                            .collect::<Vec<_>>()
                    };
                    assert_eq!(attributes(before), attributes(after), "{name}");
                }
            }
        }
    }
}

#[test]
fn double_nul_contracts_cannot_be_rebound_as_ordinary_strings() {
    let snapshot = capture(
        [Input::new("strings.hpp", SOURCE)],
        &["-x", "c++", "--target=x86_64-pc-windows-msvc"],
        &["Mixed"],
    )
    .unwrap();
    let mut options = ProjectionOptions::new("Test");
    options.library = Some("test.dll".into());
    options.string_references.insert(
        StringKind::Wide,
        TypeReference {
            namespace: "External".into(),
            name: "PWSTR".into(),
            kind: ReferenceKind::Value,
        },
    );
    let rdl = snapshot.resolve().unwrap().project(&options).unwrap().rdl();
    assert!(rdl.contains("value: *mut u16"), "{rdl}");
    assert!(!rdl.contains("External::PWSTR"), "{rdl}");
    let source = r#"
        typedef unsigned short WCHAR;
        typedef WCHAR* Text;
        extern "C" void Bound(__attribute__((annotate("_NullNull_terminated_"))) Text value);
    "#;
    let snapshot = capture(
        [Input::new("bound.hpp", source)],
        &["-x", "c++", "--target=x86_64-pc-windows-msvc"],
        &["Bound"],
    )
    .unwrap();
    options.references.insert(
        "Text".into(),
        TypeReference {
            namespace: "External".into(),
            name: "PWSTR".into(),
            kind: ReferenceKind::Value,
        },
    );
    let error = snapshot.resolve().unwrap().project(&options).err().unwrap();
    assert!(
        error
            .to_string()
            .contains("ordinary string pointer binding"),
        "{error}"
    );
}

#[test]
fn invalid_or_scoped_terminator_contracts_reject() {
    for (parameter, reason) in [
        (
            r#"SAL("_NullNull_terminated_") int* value"#,
            "require 8-bit or unsigned 16-bit characters",
        ),
        (
            r#"SAL("_NullNull_terminated_") WCHAR** value"#,
            "require a single character pointer",
        ),
        (
            r#"SAL("_NullNull_terminated_") void* value"#,
            "require 8-bit or unsigned 16-bit characters",
        ),
        (
            r#"SAL("_NullNull_terminated_") WCHAR value"#,
            "require a single character pointer",
        ),
        (
            r#"SAL("_Deref_") SAL("_NullNull_terminated_") WCHAR* value"#,
            "scoped string terminators",
        ),
    ] {
        let source = format!(
            "#define SAL(text) __attribute__((annotate(text)))\ntypedef unsigned short WCHAR;\nextern \"C\" void Invalid({parameter});"
        );
        let snapshot = capture(
            [Input::new("invalid.hpp", source)],
            &["-x", "c++", "--target=x86_64-pc-windows-msvc"],
            &["Invalid"],
        )
        .unwrap();
        let mut options = ProjectionOptions::new("Test");
        options.library = Some("test.dll".into());
        let error = snapshot.resolve().unwrap().project(&options).err().unwrap();
        assert!(error.to_string().contains(reason), "{parameter}: {error}");
    }
}

#[test]
fn sdk_multistring_contracts_survive_metadata_without_external_value_bindings() {
    let roots = [
        "SetEnvironmentStringsA",
        "SetEnvironmentStringsW",
        "GetVolumePathNamesForVolumeNameA",
        "GetVolumePathNamesForVolumeNameW",
    ];
    for arch in ["i686", "x86_64", "aarch64"] {
        let target = format!("--target={arch}-pc-windows-msvc");
        let args = sdk::arguments(&target);
        let args: Vec<_> = args.iter().map(String::as_str).collect();
        for reversed in [false, true] {
            let mut inputs = [
                Input::new("a.hpp", "#include <windows.h>"),
                Input::new("b.hpp", "#include <windows.h>"),
            ];
            if reversed {
                inputs.reverse();
            }
            let snapshot = capture(inputs, &args, &roots).unwrap();
            let mut options = ProjectionOptions::new("Test");
            options.library = Some("kernel32.dll".into());
            let rdl = snapshot.resolve().unwrap().project(&options).unwrap().rdl();
            let output = std::path::Path::new(env!("OUT_DIR"))
                .join(format!("sdk-multistring-{arch}-{reversed}.winmd"));
            windows_rdl::reader()
                .input_text(&rdl)
                .input(sdk::projection_metadata())
                .input(
                    sdk::tools()
                        .join("..")
                        .join("..")
                        .join("metadata")
                        .join("metadata.rdl"),
                )
                .output(&output)
                .write()
                .unwrap();
            let index = Index::read(output).unwrap();
            for name in roots {
                let Item::Fn(function) = index.expect_item("Test", name) else {
                    panic!()
                };
                let output = name.starts_with("GetVolume");
                let params = function
                    .params_by_sequence(if output { 4 } else { 1 })
                    .unwrap();
                let parameter = params.params()[usize::from(output)].unwrap();
                let termination = parameter
                    .attributes()
                    .find(|attr| attr.name() == "NativeStringTerminationAttribute")
                    .unwrap();
                assert_eq!(
                    termination.value(),
                    [
                        ("Count".into(), Value::U8(2)),
                        ("Phase".into(), Value::U8(if output { 2 } else { 1 })),
                    ],
                    "{name}"
                );
                let annotation = parameter
                    .attributes()
                    .find(|attr| attr.name() == "NativeAnnotationAttribute")
                    .unwrap();
                assert!(
                    annotation.value().iter().any(|(_, value)| {
                        matches!(value, Value::Utf8(text) if text.contains("_NullNull_terminated_"))
                    }),
                    "{name}"
                );
            }
        }
    }
}

use windows_clang2::{Input, capture};
use windows_metadata::HasAttributes;
#[path = "../sdk.rs"]
#[allow(dead_code)]
mod sdk;

const SOURCE: &str = include_str!("../input/optional_inputs.h");

#[cfg(target_env = "msvc")]
#[allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    dead_code
)]
mod regular {
    include!(concat!(env!("OUT_DIR"), "/optional_inputs.rs"));
}
#[cfg(target_env = "msvc")]
#[allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    dead_code
)]
mod sys {
    include!(concat!(env!("OUT_DIR"), "/optional_inputs_sys.rs"));
}
#[cfg(target_env = "msvc")]
#[allow(
    non_snake_case,
    non_camel_case_types,
    dead_code,
    clippy::upper_case_acronyms
)]
mod sdk_regular {
    include!(concat!(env!("OUT_DIR"), "/optional_sdk_inputs.rs"));
}
#[cfg(target_env = "msvc")]
#[allow(
    non_snake_case,
    non_camel_case_types,
    dead_code,
    clippy::upper_case_acronyms
)]
mod sdk_sys {
    include!(concat!(env!("OUT_DIR"), "/optional_sdk_inputs_sys.rs"));
}

#[cfg(target_env = "msvc")]
#[allow(dead_code)]
fn required_value_signatures() {
    let _: unsafe fn(i64) -> i64 = regular::OptionalInteger;
    let _: unsafe fn(usize) -> usize = regular::OptionalUnsigned;
    let _: unsafe fn(isize) -> isize = regular::OptionalSignedWord;
    let _: unsafe fn(regular::OptionalKind) -> regular::OptionalKind = regular::OptionalEnum;
    let _: unsafe fn(f32) -> f32 = regular::OptionalFloat;
    let _: unsafe fn(regular::OptionalPair) -> i32 = regular::OptionalRecord;
    let _: unsafe fn(regular::OptionalCallback, i64) -> i64 = regular::OptionalInvoke;
}

#[test]
fn input_nullability_preserves_nonpointer_values_and_source_contracts() {
    for arch in ["i686", "x86_64", "aarch64"] {
        let target = format!("--target={arch}-pc-windows-msvc");
        for reversed in [false, true] {
            let mut inputs = [Input::new("a.hpp", SOURCE), Input::new("b.hpp", SOURCE)];
            if reversed {
                inputs.reverse();
            }
            let rdl = capture(inputs, &["-x", "c++", &target], sdk::OPTIONAL_INPUT_ROOTS)
                .unwrap()
                .resolve()
                .unwrap()
                .project(&sdk::optional_value_options())
                .unwrap()
                .rdl();
            let expected = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("expected")
                .join(format!(
                    "optional_inputs{}.rdl",
                    if arch == "i686" { "_x86" } else { "" }
                ));
            if std::env::var_os("UPDATE_EXPECT").is_some() {
                std::fs::write(&expected, &rdl).unwrap();
            }
            assert_eq!(rdl, std::fs::read_to_string(expected).unwrap());
            let output = std::path::Path::new(env!("OUT_DIR"))
                .join(format!("optional-inputs-{arch}-{reversed}.winmd"));
            windows_rdl::reader()
                .input_text(&rdl)
                .output(&output)
                .write()
                .unwrap();
            let roundtrip = output.with_extension("rdl");
            windows_rdl::writer()
                .input(&output)
                .filter("Test")
                .output(&roundtrip)
                .write()
                .unwrap();
            let directory = output.with_extension("roundtrip");
            std::fs::create_dir_all(&directory).unwrap();
            let reencoded = directory.join(output.file_name().unwrap());
            windows_rdl::reader()
                .input(roundtrip)
                .output(&reencoded)
                .write()
                .unwrap();
            assert_eq!(
                std::fs::read(&output).unwrap(),
                std::fs::read(reencoded).unwrap()
            );
            use windows_metadata::{ParamAttributes, Value, reader::Index};
            let index = Index::read(output).unwrap();
            for method in index.expect("Test", "Apis").methods() {
                for (position, parameter) in method.params().enumerate() {
                    let optional = method.name() == "OptionalPointer"
                        || (method.name() == "OptionalInvoke" && position == 0);
                    assert_eq!(
                        parameter.flags().contains(ParamAttributes::Optional),
                        optional
                    );
                    assert!(parameter.flags().contains(ParamAttributes::In));
                    let annotation = parameter
                        .attributes()
                        .find(|attr| attr.name() == "NativeAnnotationAttribute")
                        .unwrap();
                    assert_eq!(
                        annotation
                            .value()
                            .into_iter()
                            .map(|(_, value)| value)
                            .collect::<Vec<_>>(),
                        [Value::Utf8("sal".into()), Value::Utf8("_In_opt_".into())]
                    );
                }
            }
        }
    }
}

#[test]
#[cfg(target_env = "msvc")]
fn native_optional_input_values_are_not_reinterpreted_or_defaulted() {
    let _: unsafe extern "C" fn(i64) -> i64 = sys::OptionalInteger;
    let _: unsafe extern "C" fn(usize) -> usize = sys::OptionalUnsigned;
    let _: unsafe extern "C" fn(isize) -> isize = sys::OptionalSignedWord;
    let _: unsafe extern "C" fn(sys::OptionalPair) -> i32 = sys::OptionalRecord;
    unsafe {
        for value in [0, 1, -1, i64::MIN, i64::MAX] {
            assert_eq!(sys::OptionalInteger(value), value);
        }
        for value in [0, 1, usize::MAX] {
            assert_eq!(sys::OptionalUnsigned(value), value);
        }
        for value in [0, 1, -1, isize::MIN, isize::MAX] {
            assert_eq!(sys::OptionalSignedWord(value), value);
        }
        for value in [sys::OptionalNone, sys::OptionalReady] {
            assert_eq!(sys::OptionalEnum(value), value);
        }
        for value in [0.0f32, -0.0, 0.5, -17.25] {
            assert_eq!(sys::OptionalFloat(value).to_bits(), value.to_bits());
        }
        for value in [0, -1, i32::MIN, i32::MAX] {
            assert_eq!(sys::OptionalRecord(sys::OptionalPair { value }), value);
        }
        let value = i64::MIN;
        assert_eq!(sys::OptionalPointer(&value), value);
        assert_eq!(sys::OptionalPointer(std::ptr::null()), -93);
        assert_eq!(sys::OptionalInvoke(None, i64::MIN), -97);
    }
    #[cfg(target_arch = "x86")]
    unsafe extern "system" fn callback(value: i64) -> i64 {
        value
    }
    #[cfg(not(target_arch = "x86"))]
    unsafe extern "C" fn callback(value: i64) -> i64 {
        value
    }
    for value in [0, -1, i64::MIN, i64::MAX] {
        assert_eq!(unsafe { sys::OptionalInvoke(Some(callback), value) }, value);
    }
}

#[test]
#[cfg(target_env = "msvc")]
fn real_sdk_optional_context_keeps_required_scalar_signatures_and_values() {
    #[cfg(target_arch = "x86")]
    unsafe extern "system" fn callback<const VALUE: isize>(_: i32, _: i32, value: isize) {
        assert_eq!(value, VALUE);
    }
    #[cfg(not(target_arch = "x86"))]
    unsafe extern "C" fn callback<const VALUE: isize>(_: i32, _: i32, value: isize) {
        assert_eq!(value, VALUE);
    }
    type Callback = sdk_regular::LINEDDAPROC;
    let _: unsafe fn(i32, i32, i32, i32, Callback, isize) -> i32 = sdk_regular::LineDDA;
    #[cfg(target_arch = "x86")]
    let _: unsafe extern "system" fn(i32, i32, i32, i32, Callback, isize) -> i32 = sdk_sys::LineDDA;
    #[cfg(not(target_arch = "x86"))]
    let _: unsafe extern "C" fn(i32, i32, i32, i32, Callback, isize) -> i32 = sdk_sys::LineDDA;
    let cases: [(Callback, isize); 3] = [
        (Some(callback::<0>), 0),
        (Some(callback::<-1>), -1),
        (Some(callback::<{ isize::MAX }>), isize::MAX),
    ];
    for (callback, value) in cases {
        unsafe {
            assert_ne!(sdk_regular::LineDDA(0, 0, 3, 2, callback, value), 0);
            assert_ne!(sdk_sys::LineDDA(0, 0, 3, 2, callback, value), 0);
        }
    }
}

#[test]
fn scalar_input_applicability_does_not_erase_source_disagreement() {
    let changed = SOURCE.replace(
        "_In_opt_ long long value);",
        "__attribute__((annotate(\"_In_\"))) long long value);",
    );
    for arch in ["i686", "x86_64", "aarch64"] {
        let target = format!("--target={arch}-pc-windows-msvc");
        for reversed in [false, true] {
            let mut inputs = [Input::new("a.hpp", SOURCE), Input::new("b.hpp", &changed)];
            if reversed {
                inputs.reverse();
            }
            let error = capture(inputs, &["-x", "c++", &target], &["OptionalInteger"])
                .unwrap()
                .resolve()
                .err()
                .unwrap()
                .to_string();
            assert!(error.contains("annotation"), "{error}");
        }
    }
}

#[test]
fn scalar_input_applicability_does_not_relax_other_parameter_contracts() {
    for arch in ["i686", "x86_64", "aarch64"] {
        let target = format!("--target={arch}-pc-windows-msvc");
        for (declaration, expected) in [
            (
                "extern \"C\" void Invalid(__attribute__((annotate(\"_Out_opt_\"))) int value);",
                "writable pointer",
            ),
            (
                "extern \"C\" void Invalid(__attribute__((annotate(\"_Inout_opt_\"))) int value);",
                "writable pointer",
            ),
            (
                "extern \"C\" void Invalid(__attribute__((annotate(\"_In_reads_opt_(1)\"))) int value);",
                "buffer pointer",
            ),
            (
                "#pragma pack(push, 1)\nstruct Adjusted { char prefix; long value; };\n#pragma pack(pop)\nextern \"C\" void Invalid(_In_opt_ Adjusted value);",
                "adjusted record layouts",
            ),
        ] {
            let snapshot = capture(
                [Input::new("limits.hpp", format!("{SOURCE}\n{declaration}"))],
                &["-x", "c++", &target],
                &["Invalid"],
            )
            .unwrap();
            let error = snapshot
                .resolve()
                .unwrap()
                .project(&sdk::optional_value_options())
                .unwrap_err()
                .to_string();
            assert!(error.contains(expected), "{error}");
        }
    }
}

#[test]
#[cfg(target_env = "msvc")]
fn real_sdk_optional_scalar_contracts_agree_and_keep_metadata_shape() {
    use windows_metadata::{ParamAttributes, Type, reader::Index};
    for arch in ["i686", "x86_64", "aarch64"] {
        let target = format!("--target={arch}-pc-windows-msvc");
        let arguments = sdk::arguments(&target);
        let args: Vec<_> = arguments.iter().map(String::as_str).collect();
        for reversed in [false, true] {
            let mut inputs = [
                Input::new("a.hpp", "#include <windows.h>"),
                Input::new("b.hpp", "#include <windows.h>"),
            ];
            if reversed {
                inputs.reverse();
            }
            let rdl = capture(inputs, &args, &["LineDDA"])
                .unwrap()
                .resolve()
                .unwrap()
                .project(&sdk::optional_input_options(&target))
                .unwrap()
                .rdl();
            let expected = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("expected")
                .join(format!(
                    "optional_sdk_inputs{}.rdl",
                    if arch == "i686" { "_x86" } else { "" }
                ));
            if std::env::var_os("UPDATE_EXPECT").is_some() {
                std::fs::write(&expected, &rdl).unwrap();
            }
            assert_eq!(rdl, std::fs::read_to_string(expected).unwrap());
            let output = std::path::Path::new(env!("OUT_DIR"))
                .join(format!("optional-sdk-{arch}-{reversed}.winmd"));
            windows_rdl::reader()
                .input_text(&rdl)
                .output(&output)
                .write()
                .unwrap();
            let index = Index::read(output).unwrap();
            let method = index
                .expect("SdkOptional", "Apis")
                .methods()
                .next()
                .unwrap();
            let parameter = method.params_by_sequence(6).unwrap().params()[5].unwrap();
            assert_eq!(parameter.flags(), ParamAttributes::In);
            assert_eq!(method.signature(&[]).types[5], Type::ISize);
        }
    }
}

#![cfg(target_env = "msvc")]

use std::path::Path;
use windows_clang2::{Input, ProjectionOptions, capture};
use windows_metadata::reader::{HasAttributes, Index, Item};
#[path = "../sdk.rs"]
#[allow(dead_code)]
mod sdk;

#[allow(dead_code, non_upper_case_globals)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/guid_constants.rs"));
}
#[allow(dead_code, non_upper_case_globals)]
mod sys {
    include!(concat!(env!("OUT_DIR"), "/guid_constants_sys.rs"));
}

#[allow(dead_code, non_upper_case_globals, non_snake_case)]
mod sdk_values {
    include!(concat!(env!("OUT_DIR"), "/sdk_data.rs"));
}
#[allow(dead_code, non_upper_case_globals, non_snake_case)]
mod sdk_sys {
    include!(concat!(env!("OUT_DIR"), "/sdk_data_sys.rs"));
}

const ARGS: &[&str] = &["-x", "c++", "--target=x86_64-pc-windows-msvc"];
const SOURCE: &str = include_str!("../input/guid_constants.h");

unsafe extern "C" {
    fn CheckConstants(
        id: *const core::ffi::c_void,
        id_size: u32,
        key: *const core::ffi::c_void,
        key_size: u32,
    ) -> i32;
    fn CheckSdkValues(
        property: *const core::ffi::c_void,
        device: *const core::ffi::c_void,
        folder: *const core::ffi::c_void,
        media: *const core::ffi::c_void,
        avi: *const core::ffi::c_void,
        file: *const core::ffi::c_void,
        network: *const core::ffi::c_void,
    ) -> i32;
}

#[test]
fn native_sdk_values_match_compiler_initializers() {
    for sizes in [
        [
            size_of_val(&sdk_values::PKEY_Address_Country),
            size_of_val(&sdk_values::DEVPKEY_Device_ClassGuid),
            size_of_val(&sdk_values::FOLDERID_Documents),
            size_of_val(&sdk_values::MFVideoFormat_RGB32),
            size_of_val(&sdk_values::IID_IAVIFile),
            size_of_val(&sdk_values::FILE_TYPE_NOTIFICATION_GUID_PAGE_FILE),
            size_of_val(&sdk_values::NETWORK_MANAGER_FIRST_IP_ADDRESS_ARRIVAL_GUID),
        ],
        [
            size_of_val(&sdk_sys::PKEY_Address_Country),
            size_of_val(&sdk_sys::DEVPKEY_Device_ClassGuid),
            size_of_val(&sdk_sys::FOLDERID_Documents),
            size_of_val(&sdk_sys::MFVideoFormat_RGB32),
            size_of_val(&sdk_sys::IID_IAVIFile),
            size_of_val(&sdk_sys::FILE_TYPE_NOTIFICATION_GUID_PAGE_FILE),
            size_of_val(&sdk_sys::NETWORK_MANAGER_FIRST_IP_ADDRESS_ARRIVAL_GUID),
        ],
    ] {
        assert_eq!(sizes, [20, 20, 16, 16, 16, 16, 16]);
    }
    for values in [
        [
            core::ptr::from_ref(&sdk_values::PKEY_Address_Country).cast(),
            core::ptr::from_ref(&sdk_values::DEVPKEY_Device_ClassGuid).cast(),
            core::ptr::from_ref(&sdk_values::FOLDERID_Documents).cast(),
            core::ptr::from_ref(&sdk_values::MFVideoFormat_RGB32).cast(),
            core::ptr::from_ref(&sdk_values::IID_IAVIFile).cast(),
            core::ptr::from_ref(&sdk_values::FILE_TYPE_NOTIFICATION_GUID_PAGE_FILE).cast(),
            core::ptr::from_ref(&sdk_values::NETWORK_MANAGER_FIRST_IP_ADDRESS_ARRIVAL_GUID).cast(),
        ],
        [
            core::ptr::from_ref(&sdk_sys::PKEY_Address_Country).cast(),
            core::ptr::from_ref(&sdk_sys::DEVPKEY_Device_ClassGuid).cast(),
            core::ptr::from_ref(&sdk_sys::FOLDERID_Documents).cast(),
            core::ptr::from_ref(&sdk_sys::MFVideoFormat_RGB32).cast(),
            core::ptr::from_ref(&sdk_sys::IID_IAVIFile).cast(),
            core::ptr::from_ref(&sdk_sys::FILE_TYPE_NOTIFICATION_GUID_PAGE_FILE).cast(),
            core::ptr::from_ref(&sdk_sys::NETWORK_MANAGER_FIRST_IP_ADDRESS_ARRIVAL_GUID).cast(),
        ],
    ] {
        assert_eq!(
            unsafe {
                CheckSdkValues(
                    values[0], values[1], values[2], values[3], values[4], values[5], values[6],
                )
            },
            1
        );
    }
}

#[test]
fn generated_values_match_native_initializers() {
    fn check<I, K>(id: &I, key: &K) {
        assert_eq!(
            unsafe {
                CheckConstants(
                    core::ptr::from_ref(id).cast(),
                    size_of::<I>().try_into().unwrap(),
                    core::ptr::from_ref(key).cast(),
                    size_of::<K>().try_into().unwrap(),
                )
            },
            1
        );
    }
    check(&bindings::ID, &bindings::KEY);
    check(&sys::ID, &sys::KEY);
    assert_eq!(bindings::ID.first, 0xfedcba98);
    assert_eq!(bindings::ID.bytes[7], 0xfe);
    assert_eq!(bindings::KEY.property, 65543);
}

#[test]
fn declarations_and_definitions_agree_and_roundtrip() {
    let snapshot = capture(
        [
            Input::new("decl.hpp", SOURCE),
            Input::new("def.hpp", format!("#define DEFINE_VALUES\n{SOURCE}")),
        ],
        ARGS,
        &["ID", "KEY"],
    )
    .unwrap();
    let plan = snapshot
        .resolve()
        .unwrap()
        .project(&ProjectionOptions::new("Test"))
        .unwrap();
    let expected = Path::new(env!("CARGO_MANIFEST_DIR")).join("expected/guid_constants.rdl");
    if std::env::var_os("UPDATE_EXPECT").is_some() {
        std::fs::write(&expected, plan.rdl()).unwrap();
    }
    assert_eq!(
        plan.rdl(),
        std::fs::read_to_string(expected)
            .unwrap()
            .replace("\r\n", "\n")
    );
    let out = Path::new(env!("OUT_DIR"));
    let rdl = out.join("guid_constants_roundtrip.rdl");
    windows_rdl::writer()
        .input(out.join("guid_constants.winmd"))
        .filter("Test")
        .output(&rdl)
        .write()
        .unwrap();
    let winmd = out.join("guid_constants_roundtrip.winmd");
    windows_rdl::reader()
        .input(rdl)
        .reference_default()
        .output(&winmd)
        .write()
        .unwrap();
    for sys in [false, true] {
        let original = out.join(if sys {
            "guid_constants_sys.rs"
        } else {
            "guid_constants.rs"
        });
        let projected = out.join(if sys {
            "guid_constants_roundtrip_sys.rs"
        } else {
            "guid_constants_roundtrip.rs"
        });
        let mut args = vec![
            "--in",
            "default",
            winmd.to_str().unwrap(),
            "--out",
            projected.to_str().unwrap(),
            "--flat",
            "--filter",
            "Test",
        ];
        if sys {
            args.push("--sys");
        }
        windows_bindgen::bindgen(args);
        assert_eq!(
            std::fs::read(original).unwrap(),
            std::fs::read(projected).unwrap()
        );
    }
}

#[test]
fn conflicting_nested_values_fail_native_resolution() {
    for root in ["ID", "KEY"] {
        let snapshot = capture(
            [
                Input::new("a.hpp", format!("#define DEFINE_VALUES\n{SOURCE}")),
                Input::new(
                    "b.hpp",
                    format!("#define DEFINE_VALUES\n#define LAST_BYTE 0xfd\n{SOURCE}"),
                ),
            ],
            ARGS,
            &[root],
        )
        .unwrap();
        let Err(error) = snapshot.resolve() else {
            panic!("conflicting constants agreed")
        };
        assert!(error.to_string().contains("value differs"), "{error}");
    }
}

#[test]
fn incomplete_and_nonconstant_initializers_are_not_guessed() {
    let source = include_str!("../input/aggregate_limits.h");
    for root in ["Partial", "Dynamic"] {
        let snapshot = capture([Input::new("limits.hpp", source)], ARGS, &[root]).unwrap();
        assert!(snapshot.resolve().is_err(), "{root}");
    }

    for root in ["WrongShape", "WrongSignedness"] {
        let snapshot = capture([Input::new("limits.hpp", source)], ARGS, &[root]).unwrap();
        assert!(
            snapshot
                .resolve()
                .unwrap()
                .project(&ProjectionOptions::new("Test"))
                .is_err(),
            "{root}"
        );
    }
}

#[test]
fn sdk_definition_configuration_preserves_header_values() {
    let source = include_str!("../input/sdk_data.h");
    for arch in ["i686", "x86_64", "aarch64"] {
        let target = format!("--target={arch}-pc-windows-msvc");
        let arguments = sdk::arguments(&target);
        for reversed in [false, true] {
            let mut inputs = [
                Input::new("decl.hpp", source),
                Input::new("def.hpp", sdk::DATA_DEFINITIONS),
            ];
            if reversed {
                inputs.reverse();
            }
            let snapshot = capture(
                inputs,
                &arguments.iter().map(String::as_str).collect::<Vec<_>>(),
                sdk::DATA_ROOTS,
            )
            .unwrap();
            let plan = snapshot
                .resolve()
                .unwrap()
                .project(&ProjectionOptions::new("Test"))
                .unwrap();
            let expected = Path::new(env!("CARGO_MANIFEST_DIR")).join("expected/sdk_data.rdl");
            if std::env::var_os("UPDATE_EXPECT").is_some() {
                std::fs::write(&expected, plan.rdl()).unwrap();
            }
            assert_eq!(plan.rdl(), std::fs::read_to_string(expected).unwrap());
            let out = Path::new(env!("OUT_DIR"));
            let winmd = out.join(format!("sdk_data_{arch}.winmd"));
            windows_rdl::reader()
                .input_text(&plan.rdl())
                .reference_default()
                .output(&winmd)
                .write()
                .unwrap();
            let rdl = out.join(format!("sdk_data_{arch}_roundtrip.rdl"));
            windows_rdl::writer()
                .input(&winmd)
                .filter("Test")
                .output(&rdl)
                .write()
                .unwrap();
            let roundtrip = out.join(format!("sdk_data_{arch}_roundtrip.winmd"));
            windows_rdl::reader()
                .input(rdl)
                .reference_default()
                .output(&roundtrip)
                .write()
                .unwrap();
            let original = Index::read(winmd).unwrap();
            let roundtrip = Index::read(roundtrip).unwrap();
            for root in sdk::DATA_ROOTS {
                let Item::Const(original) = original.expect_item("Test", root) else {
                    panic!()
                };
                let Item::Const(roundtrip) = roundtrip.expect_item("Test", root) else {
                    panic!()
                };
                assert_eq!(original.ty(), roundtrip.ty());
                assert_eq!(
                    original.find_attribute("GuidAttribute").unwrap().value(),
                    roundtrip.find_attribute("GuidAttribute").unwrap().value()
                );
                assert_eq!(
                    original.constant().map(|value| value.value()),
                    roundtrip.constant().map(|value| value.value())
                );
            }
        }
    }
}

#[test]
fn sdk_extern_data_is_not_invented_in_definition_configuration() {
    for (source, root) in [
        (include_str!("../input/sdk_data.h"), "PKEY_Address_Country"),
        (
            "#include <windows.h>\n#include <initguid.h>\n#include <objidl.h>",
            "IID_ISequentialStream",
        ),
    ] {
        let snapshot = sdk::capture_sdk("--target=x86_64-pc-windows-msvc", source, &[root]);
        let error = snapshot
            .resolve()
            .unwrap()
            .project(&ProjectionOptions::new("Test"))
            .unwrap_err();
        assert!(
            error.to_string().contains("declaration-only data"),
            "{error}"
        );
    }
}

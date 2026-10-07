#![cfg(target_env = "msvc")]

use std::path::Path;
use windows_clang2::{Input, ProjectionOptions, capture};

#[allow(dead_code, non_upper_case_globals)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/guid_constants.rs"));
}
#[allow(dead_code, non_upper_case_globals)]
mod sys {
    include!(concat!(env!("OUT_DIR"), "/guid_constants_sys.rs"));
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

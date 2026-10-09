#![cfg(target_env = "msvc")]

use windows_clang2::{Input, ProjectionOptions, capture};
#[path = "../sdk.rs"]
#[allow(dead_code)]
mod sdk;

const SOURCE: &str = include_str!("../input/inherited_interfaces.h");
const ROOTS: &[&str] = &[
    "IExtended",
    "InheritedCreate",
    "InheritedRead",
    "InheritedExtra",
];

#[allow(
    non_snake_case,
    non_camel_case_types,
    dead_code,
    clippy::upper_case_acronyms,
    clippy::missing_transmute_annotations
)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/com.rs"));
}
#[allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    dead_code,
    clippy::upper_case_acronyms
)]
mod sys {
    include!(concat!(env!("OUT_DIR"), "/com_sys.rs"));
}

fn options() -> ProjectionOptions {
    let mut options = ProjectionOptions::new("Test");
    options.library = Some("clang2_com.dll".into());
    options
}

#[test]
fn native_inherited_slots_and_distinct_iids_preserve_ownership() {
    use bindings::{IBaseValue, IExtended, ILeaf, IMarker};
    use windows_core::{IUnknown, Interface};
    let _: unsafe extern "C" fn(*mut core::ffi::c_void) -> i32 = sys::InheritedRead;
    let mut stats = sys::InheritedStats::default();
    let value = unsafe { IExtended::from_raw(sys::InheritedCreate(&raw mut stats)) };
    assert_ne!(IMarker::IID, IBaseValue::IID);
    assert_ne!(ILeaf::IID, IMarker::IID);
    assert_ne!(IExtended::IID, ILeaf::IID);
    let marker = value.cast::<IMarker>().unwrap();
    let leaf = value.cast::<ILeaf>().unwrap();
    let base = value.cast::<IBaseValue>().unwrap();
    let unknown = value.cast::<IUnknown>().unwrap();
    for pointer in [
        marker.as_raw(),
        leaf.as_raw(),
        base.as_raw(),
        unknown.as_raw(),
    ] {
        assert_eq!(pointer, value.as_raw());
    }
    assert_eq!(stats.references, 5);
    assert_eq!(stats.queries, 4);
    assert_eq!(stats.adds, 4);
    unsafe {
        assert_eq!(value.Read(), 42);
        assert_eq!(value.Extra(), 64);
        assert_eq!(marker.Read(), 42);
        assert_eq!(leaf.Read(), 42);
        assert_eq!(sys::InheritedRead(leaf.as_raw()), 42);
        assert_eq!(sys::InheritedExtra(value.as_raw()), 64);
    }
    drop((marker, leaf, base, unknown));
    assert_eq!(stats.references, 1);
    drop(value);
    assert_eq!(stats.references, 0);
    assert_eq!(stats.releases, 5);
    assert_eq!(stats.destroyed, 1);
}

#[windows_core::implement(bindings::IExtended)]
struct RustInherited;

impl bindings::IBaseValue_Impl for RustInherited_Impl {
    fn Read(&self) -> i32 {
        17
    }
}
impl bindings::IMarker_Impl for RustInherited_Impl {}
impl bindings::ILeaf_Impl for RustInherited_Impl {}
impl bindings::IExtended_Impl for RustInherited_Impl {
    fn Extra(&self) -> i32 {
        29
    }
}

#[test]
fn native_caller_uses_rust_inherited_slots() {
    use windows_core::Interface;
    let value: bindings::IExtended = RustInherited.into();
    let leaf = value.cast::<bindings::ILeaf>().unwrap();
    unsafe {
        assert_eq!(sys::InheritedRead(leaf.as_raw()), 17);
        assert_eq!(sys::InheritedExtra(value.as_raw()), 29);
    }
}

#[test]
fn inherited_interfaces_preserve_identity_and_pointer_shape() {
    for arch in ["i686", "x86_64", "aarch64"] {
        let arguments = sdk::arguments(&format!("--target={arch}-pc-windows-msvc"));
        let args: Vec<_> = arguments.iter().map(String::as_str).collect();
        for reversed in [false, true] {
            let mut inputs = [Input::new("a.hpp", SOURCE), Input::new("b.hpp", SOURCE)];
            if reversed {
                inputs.reverse();
            }
            let rdl = capture(inputs, &args, ROOTS)
                .unwrap()
                .resolve()
                .unwrap()
                .project(&options())
                .unwrap()
                .rdl();
            let expected = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("expected")
                .join(format!(
                    "inherited_interfaces{}.rdl",
                    if arch == "i686" { "_x86" } else { "" }
                ));
            if std::env::var_os("UPDATE_EXPECT").is_some() {
                std::fs::write(&expected, &rdl).unwrap();
            }
            assert_eq!(rdl, std::fs::read_to_string(expected).unwrap());
            let output = std::path::Path::new(env!("OUT_DIR"))
                .join(format!("inherited-{arch}-{reversed}.winmd"));
            windows_rdl::reader()
                .input_text(&rdl)
                .reference_default()
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
                .input(&roundtrip)
                .reference_default()
                .output(&reencoded)
                .write()
                .unwrap();
            assert_eq!(
                std::fs::read(&output).unwrap(),
                std::fs::read(reencoded).unwrap()
            );
            use windows_metadata::{Type, reader::Index};
            let index = Index::read(output).unwrap();
            for (name, base, methods) in [
                ("IMarker", "IBaseValue", vec![]),
                ("ILeaf", "IMarker", vec![]),
                ("IExtended", "ILeaf", vec!["Extra"]),
            ] {
                let ty = index.expect("Test", name);
                assert_eq!(ty.fields().len(), 0);
                assert_eq!(
                    ty.methods().map(|method| method.name()).collect::<Vec<_>>(),
                    methods
                );
                assert_eq!(
                    ty.interface_impls().next().unwrap().interface(&[]),
                    Type::class_named("Test", base)
                );
            }
            let method = index
                .expect("Test", "Apis")
                .methods()
                .find(|method| method.name() == "InheritedRead")
                .unwrap();
            assert_eq!(
                method.signature(&[]).types,
                [Type::class_named("Test", "ILeaf")]
            );
        }
    }
}

#[test]
fn inherited_identity_and_slot_changes_conflict_across_units() {
    for arch in ["i686", "x86_64", "aarch64"] {
        let arguments = sdk::arguments(&format!("--target={arch}-pc-windows-msvc"));
        let args: Vec<_> = arguments.iter().map(String::as_str).collect();
        for changed in [
            SOURCE.replace("1e5b7c93-", "2e5b7c93-"),
            SOURCE.replace("ILeaf : IMarker {}", "ILeaf : IBaseValue {}"),
            SOURCE.replace(
                "IMarker : IBaseValue {}",
                "IMarker : IBaseValue { virtual long __stdcall NewSlot() = 0; }",
            ),
        ] {
            for reversed in [false, true] {
                let mut inputs = [Input::new("a.hpp", SOURCE), Input::new("b.hpp", &changed)];
                if reversed {
                    inputs.reverse();
                }
                let error = capture(inputs, &args, &["IExtended"])
                    .unwrap()
                    .resolve()
                    .err()
                    .unwrap();
                let error = error.to_string();
                assert!(error.contains("conflicting native declarations"), "{error}");
                assert!(
                    error.contains("a.hpp") && error.contains("b.hpp"),
                    "{error}"
                );
            }
        }
    }
}

#[test]
fn inherited_interfaces_do_not_relax_storage_identity_or_by_value_gates() {
    for arch in ["i686", "x86_64", "aarch64"] {
        let arguments = sdk::arguments(&format!("--target={arch}-pc-windows-msvc"));
        let args: Vec<_> = arguments.iter().map(String::as_str).collect();
        for (declaration, expected) in [
            (
                "struct __declspec(uuid(\"1e5b7c95-241a-4175-b520-552b1687b411\")) Invalid {};",
                "empty record",
            ),
            (
                "struct Invalid : ILeaf {};",
                "projection is not implemented",
            ),
            (
                "extern \"C\" void Invalid(ILeaf value);",
                "interface objects require a pointer",
            ),
            (
                "struct __declspec(uuid(\"1e5b7c95-241a-4175-b520-552b1687b411\")) Invalid : ILeaf { virtual long __stdcall Read() override = 0; };",
                "inherited virtual slot",
            ),
        ] {
            let snapshot = capture(
                [Input::new("limits.hpp", format!("{SOURCE}\n{declaration}"))],
                &args,
                &["Invalid"],
            )
            .unwrap();
            let error = snapshot
                .resolve()
                .unwrap()
                .project(&options())
                .unwrap_err()
                .to_string();
            assert!(error.contains(expected), "{error}");
        }
        let source = format!(
            "{SOURCE}\nstruct __declspec(uuid(\"1e5b7c95-241a-4175-b520-552b1687b411\")) Invalid : ILeaf {{ long value; }};"
        );
        let error = capture([Input::new("storage.hpp", source)], &args, &["Invalid"])
            .unwrap()
            .resolve()
            .err()
            .unwrap()
            .to_string();
        assert!(error.contains("data-bearing"), "{error}");
    }
}

#[test]
fn real_sdk_method_free_interfaces_project_across_targets_and_unit_order() {
    let source = "#include <windows.h>\n#include <ocidl.h>";
    let roots = ["IFontDisp", "IFontEventsDisp", "IPictureDisp"];
    for arch in ["i686", "x86_64", "aarch64"] {
        let arguments = sdk::arguments(&format!("--target={arch}-pc-windows-msvc"));
        let args: Vec<_> = arguments.iter().map(String::as_str).collect();
        let mut baseline = None;
        for reversed in [false, true] {
            let mut inputs = [Input::new("a.hpp", source), Input::new("b.hpp", source)];
            if reversed {
                inputs.reverse();
            }
            let plan = capture(inputs, &args, &roots)
                .unwrap()
                .resolve()
                .unwrap()
                .project(&options())
                .unwrap();
            let rdl = plan.rdl();
            if let Some(baseline) = &baseline {
                assert_eq!(baseline, &rdl);
            } else {
                baseline = Some(rdl.clone());
            }
            let output = std::path::Path::new(env!("OUT_DIR"))
                .join(format!("sdk-inherited-{arch}-{reversed}.winmd"));
            windows_rdl::reader()
                .input_text(&rdl)
                .reference_default()
                .output(output)
                .write()
                .unwrap();
        }
    }
}

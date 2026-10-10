use windows_clang2::{Input, ProjectionOptions, capture};
use windows_metadata::{Type, reader::*};
#[path = "../sdk.rs"]
#[allow(dead_code)]
mod sdk;

const SOURCE: &str = include_str!("../input/nested_records.h");

fn options() -> ProjectionOptions {
    let mut options = ProjectionOptions::new("Test");
    options.library = Some("test.dll".into());
    options
}

#[test]
fn nested_types_keep_nominal_identity_and_shared_references() {
    for arch in ["i686", "x86_64", "aarch64"] {
        let target = format!("--target={arch}-pc-windows-msvc");
        let args = &["-x", "c++", &target];
        let mut baseline = None;
        for reversed in [false, true] {
            let mut inputs = [Input::new("a.hpp", SOURCE), Input::new("b.hpp", SOURCE)];
            if reversed {
                inputs.reverse();
            }
            let snapshot = capture(inputs, args, &["Packet", "Consume"]).unwrap();
            let plan = snapshot.resolve().unwrap().project(&options()).unwrap();
            let rdl = plan.rdl();
            if let Some(expected) = &baseline {
                assert_eq!(&rdl, expected);
            } else {
                baseline = Some(rdl.clone());
            }
            let name = if arch == "i686" {
                "nested_records_x86.rdl"
            } else {
                "nested_records.rdl"
            };
            let expected = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("expected")
                .join(name);
            if std::env::var_os("UPDATE_EXPECT").is_some() {
                std::fs::write(&expected, &rdl).unwrap();
            }
            assert_eq!(rdl, std::fs::read_to_string(expected).unwrap());
            let output = std::path::Path::new(env!("OUT_DIR"))
                .join(format!("nested-records-{arch}-{reversed}.winmd"));
            windows_rdl::reader()
                .input_text(&rdl)
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
            for owner in ["First", "Second"] {
                let name = format!("{owner}_Shared");
                let value = Type::value_named("Test", &name);
                let pointer = Type::PtrMut(Box::new(value.clone()), 1);
                let record = index.expect("Test", &name);
                let fields: Vec<_> = record.fields().collect();
                assert_eq!(fields[0].ty(), Type::I32);
                assert_eq!(fields[1].ty(), pointer);
                assert_eq!(
                    fields[2].ty(),
                    Type::class_named("Test", &format!("{name}_visit_Callback"))
                );
                let outer = index.expect("Test", owner);
                let fields: Vec<_> = outer.fields().collect();
                assert_eq!(fields[0].ty(), value);
                assert_eq!(fields[1].ty(), pointer);
                if owner == "First" {
                    assert_eq!(fields[2].ty(), pointer);
                    assert_eq!(fields[3].ty(), Type::value_named("Test", "First_Choice"));
                    assert_eq!(fields[4].ty(), Type::value_named("Test", "First_Kind"));
                }
            }
            let Item::Fn(function) = index.expect_item("Test", "Consume") else {
                panic!()
            };
            assert_eq!(
                function.signature(&[]).types,
                [
                    Type::PtrMut(Box::new(Type::value_named("Test", "First_Shared")), 1),
                    Type::PtrMut(Box::new(Type::value_named("Test", "Second_Shared")), 1),
                    Type::PtrMut(Box::new(Type::value_named("Test", "First_Shared")), 1),
                ]
            );
        }
    }
}

#[test]
fn reserved_field_names_compile_to_metadata_without_collisions() {
    let source = include_str!("../input/reserved_fields.h");
    for arch in ["i686", "x86_64", "aarch64"] {
        let target = format!("--target={arch}-pc-windows-msvc");
        let args = &["-x", "c++", &target];
        let snapshot = capture(
            [Input::new("reserved.h", source)],
            args,
            &["ReservedFields"],
        )
        .unwrap();
        let rdl = snapshot
            .resolve()
            .unwrap()
            .project(&ProjectionOptions::new("Test"))
            .unwrap()
            .rdl();
        assert_eq!(
            rdl,
            include_str!("../expected/reserved_fields.rdl").replace("\r\n", "\n")
        );
        let output =
            std::path::Path::new(env!("OUT_DIR")).join(format!("reserved-fields-{arch}.winmd"));
        windows_rdl::reader()
            .input_text(&rdl)
            .output(&output)
            .write()
            .unwrap();
        let index = Index::read(output).unwrap();
        let names: Vec<_> = index
            .expect("Test", "ReservedFields")
            .fields()
            .map(|field| field.name())
            .collect();
        assert_eq!(names, ["Self_", "self_", "super_", "crate_", "type"]);
        let snapshot = capture(
            [Input::new("reserved.h", source)],
            args,
            &["ReservedCollision"],
        )
        .unwrap();
        let error = snapshot
            .resolve()
            .unwrap()
            .project(&ProjectionOptions::new("Test"))
            .unwrap_err();
        assert!(
            error.to_string().contains("multiple native fields map"),
            "{error}"
        );
    }
}

#[test]
fn scoped_publication_does_not_hide_native_or_name_conflicts() {
    for arch in ["i686", "x86_64", "aarch64"] {
        let target = format!("--target={arch}-pc-windows-msvc");
        let args = &["-x", "c++", &target];
        for reversed in [false, true] {
            let changed = format!("#define MEMBER_TYPE float\n{SOURCE}");
            let mut inputs = [Input::new("a.hpp", SOURCE), Input::new("b.hpp", &changed)];
            if reversed {
                inputs.reverse();
            }
            let snapshot = capture(inputs, args, &["Packet"]).unwrap();
            let error = snapshot.resolve().err().unwrap().to_string();
            assert!(error.contains("conflicting native declarations"), "{error}");
        }
        let snapshot = capture([Input::new("a.hpp", SOURCE)], args, &["Collision"]).unwrap();
        let error = snapshot
            .resolve()
            .unwrap()
            .project(&options())
            .unwrap_err()
            .to_string();
        assert!(error.contains("multiple native entities"), "{error}");
        for root in [
            "First::Shared",
            "First::Alias",
            "First::Kind",
            "ConsumeUnion",
        ] {
            let snapshot = capture([Input::new("a.hpp", SOURCE)], args, &[root]).unwrap();
            let plan = snapshot.resolve().unwrap().project(&options()).unwrap();
            assert!(plan.rdl().contains(&root.replace("::", "_")));
        }
        let snapshot = capture([Input::new("a.hpp", SOURCE)], args, &["mod::type"]).unwrap();
        let plan = snapshot.resolve().unwrap().project(&options()).unwrap();
        let output =
            std::path::Path::new(env!("OUT_DIR")).join(format!("scoped-keywords-{arch}.winmd"));
        windows_rdl::reader()
            .input_text(&plan.rdl())
            .output(&output)
            .write()
            .unwrap();
        let index = Index::read(output).unwrap();
        assert_eq!(
            index
                .expect("Test", "mod_type")
                .fields()
                .next()
                .unwrap()
                .ty(),
            Type::I32
        );
    }
}

#[test]
fn real_ole_nested_union_storage_compiles_across_targets_and_tus() {
    for arch in ["i686", "x86_64", "aarch64"] {
        let target = format!("--target={arch}-pc-windows-msvc");
        let arguments = sdk::arguments(&target);
        let args: Vec<_> = arguments.iter().map(String::as_str).collect();
        let mut baseline = None;
        for reversed in [false, true] {
            let mut inputs = [
                Input::new("ole.hpp", "#include <oaidl.h>"),
                Input::new("object.hpp", "#include <objidl.h>"),
            ];
            if reversed {
                inputs.reverse();
            }
            let snapshot = capture(
                inputs,
                &args,
                &["wireVARIANT", "wireSAFEARRAY", "GDI_OBJECT"],
            )
            .unwrap();
            let options = ProjectionOptions::new("Test");
            assert!(options.references.is_empty());
            let plan = snapshot.resolve().unwrap().project(&options).unwrap();
            let rdl = plan.rdl();
            if let Some(expected) = &baseline {
                assert_eq!(&rdl, expected);
            } else {
                baseline = Some(rdl.clone());
            }
            let output = std::path::Path::new(env!("OUT_DIR"))
                .join(format!("ole-nested-{arch}-{reversed}.winmd"));
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
            for (outer, inner) in [
                (
                    "_wireSAFEARRAY_UNION",
                    "_wireSAFEARRAY_UNION___MIDL_IOleAutomationTypes_0001",
                ),
                ("GDI_OBJECT", "_GDI_OBJECT___MIDL_IAdviseSink_0002"),
            ] {
                let record = index.expect("Test", outer);
                let field = record.fields().find(|field| field.name() == "u").unwrap();
                assert_eq!(field.ty(), Type::value_named("Test", inner));
                assert_eq!(index.get("Test", inner).count(), 1);
                assert!(
                    index
                        .expect("Test", inner)
                        .flags()
                        .contains(windows_metadata::TypeAttributes::ExplicitLayout)
                );
            }
        }
    }
}

use windows_clang2::{Input, ProjectionOptions, capture};
use windows_metadata::{Type, reader::*};
#[path = "../sdk.rs"]
#[allow(dead_code)]
mod sdk;

const SOURCE: &str = include_str!("../input/typedef_publication.h");

#[test]
fn real_shared_handle_and_interface_aliases_keep_owned_definitions() {
    for arch in ["i686", "x86_64", "aarch64"] {
        let target = format!("--target={arch}-pc-windows-msvc");
        let arguments = sdk::arguments(&target);
        let args: Vec<_> = arguments.iter().map(String::as_str).collect();
        let mut baseline = None;
        for reversed in [false, true] {
            let mut inputs = [
                Input::new(
                    "security.hpp",
                    "#define SECURITY_WIN32\n#include <windows.h>\n#include <sspi.h>",
                ),
                Input::new("xml.hpp", "#include <xmllite.h>"),
            ];
            let mut roots = [
                "CredHandle",
                "CtxtHandle",
                "IXmlReaderInput",
                "IXmlWriterOutput",
            ];
            if reversed {
                inputs.reverse();
                roots.reverse();
            }
            let snapshot = capture(inputs, &args, &roots).unwrap();
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
                .join(format!("sdk-shared-aliases-{arch}-{reversed}.winmd"));
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
            let mut handle_type = None;
            for name in ["CredHandle", "CtxtHandle"] {
                let alias = index.expect("Test", name);
                assert!(alias.has_attribute("NativeTypedefAttribute"));
                let value = alias.fields().next().unwrap().ty();
                if let Some(expected) = &handle_type {
                    assert_eq!(&value, expected);
                } else {
                    handle_type = Some(value);
                }
            }
            let Type::ValueName(record) = handle_type.unwrap() else {
                panic!("shared security aliases must retain a native value definition")
            };
            assert_eq!(index.get(&record.namespace, &record.name).count(), 1);
            assert_eq!(
                index
                    .expect(&record.namespace, &record.name)
                    .fields()
                    .count(),
                2
            );
            assert_eq!(index.get("Test", "IUnknown").count(), 1);
            assert_eq!(
                index.expect("Test", "IUnknown").category(),
                TypeCategory::Interface
            );
            for name in ["IXmlReaderInput", "IXmlWriterOutput"] {
                let alias = index.expect("Test", name);
                assert!(alias.has_attribute("NativeTypedefAttribute"));
                assert_eq!(
                    alias.fields().next().unwrap().ty(),
                    Type::class_named("Test", "IUnknown")
                );
            }
        }
    }
}

#[test]
fn selected_named_aliases_keep_one_native_definition_without_root_or_tu_priority() {
    for arch in ["i686", "x86_64", "aarch64"] {
        let target = format!("--target={arch}-pc-windows-msvc");
        let args = &["-x", "c++", &target];
        for preserve in [false, true] {
            let mut baseline = None;
            for reversed in [false, true] {
                let mut inputs = [Input::new("a.hpp", SOURCE), Input::new("b.hpp", SOURCE)];
                let mut roots = ["FIRST", "SECOND"];
                if reversed {
                    inputs.reverse();
                    roots.reverse();
                }
                let snapshot = capture(inputs, args, &roots).unwrap();
                let mut options = ProjectionOptions::new("Test");
                options.preserve_typedefs = preserve;
                let plan = snapshot.resolve().unwrap().project(&options).unwrap();
                if let Some(expected) = &baseline {
                    assert_eq!(&plan.rdl(), expected);
                } else {
                    baseline = Some(plan.rdl());
                }
                let output = std::path::Path::new(env!("OUT_DIR"))
                    .join(format!("shared-aliases-{arch}-{preserve}-{reversed}.winmd"));
                windows_rdl::reader()
                    .input_text(&plan.rdl())
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
                assert_eq!(index.get("Test", "Shared").count(), 1);
                for alias in ["FIRST", "SECOND"] {
                    let alias = index.expect("Test", alias);
                    assert!(alias.has_attribute("NativeTypedefAttribute"));
                    assert_eq!(
                        alias.fields().next().unwrap().ty(),
                        Type::value_named("Test", "Shared")
                    );
                }
            }
        }
        let changed = SOURCE.replace(
            "struct Shared { int value; }",
            "struct Shared { float value; }",
        );
        let snapshot = capture(
            [Input::new("a.hpp", SOURCE), Input::new("b.hpp", changed)],
            args,
            &["FIRST", "SECOND"],
        )
        .unwrap();
        assert!(snapshot.resolve().is_err());
    }
}

#[test]
fn typedef_publication_preserves_written_names_without_guessing_tag_prefixes() {
    for arch in ["i686", "x86_64", "aarch64"] {
        let target = format!("--target={arch}-pc-windows-msvc");
        let args = &["-x", "c++", &target];
        let mut baseline = None;
        for reversed in [false, true] {
            let mut inputs = [Input::new("a.hpp", SOURCE), Input::new("b.hpp", SOURCE)];
            if reversed {
                inputs.reverse();
            }
            let snapshot = capture(inputs, args, &["Values", "Update"]).unwrap();
            let mut options = ProjectionOptions::new("Test");
            options.library = Some("test.dll".into());
            options.preserve_typedefs = true;
            let plan = snapshot.resolve().unwrap().project(&options).unwrap();
            let rdl = plan.rdl();
            if let Some(expected) = &baseline {
                assert_eq!(&rdl, expected);
            } else {
                baseline = Some(rdl.clone());
            }
            let name = if arch == "i686" {
                "typedef_publication_x86"
            } else {
                "typedef_publication"
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
            let record = index.expect("Test", "Values");
            let fields: Vec<_> = record.fields().map(|field| field.ty()).collect();
            assert_eq!(
                fields,
                ["SECONDS", "RESULT", "KEYFRAME", "POINT", "FIRST", "SECOND"]
                    .map(|name| Type::value_named("Test", name))
            );
            assert_eq!(
                index
                    .expect("Test", "SECONDS")
                    .fields()
                    .next()
                    .unwrap()
                    .ty(),
                Type::F64
            );
            assert_eq!(
                index.expect("Test", "RESULT").category(),
                TypeCategory::Enum
            );
            for alias in ["FIRST", "SECOND"] {
                assert_eq!(
                    index.expect("Test", alias).fields().next().unwrap().ty(),
                    Type::value_named("Test", "Shared")
                );
            }
            let Item::Fn(function) = index.expect_item("Test", "Update") else {
                panic!()
            };
            assert_eq!(
                function.signature(&[]).return_type,
                Type::value_named("Test", "RESULT")
            );
            assert_eq!(
                function.signature(&[]).types,
                [
                    Type::value_named("Test", "SECONDS"),
                    Type::value_named("Test", "KEYFRAME"),
                    Type::PtrMut(Box::new(Type::value_named("Test", "POINT")), 1),
                ]
            );
            let snapshot = capture(
                [Input::new("a.hpp", SOURCE)],
                args,
                &["RESULT", "__MIDL_result"],
            )
            .unwrap();
            let plan = snapshot.resolve().unwrap().project(&options).unwrap();
            assert!(plan.rdl().contains("enum __MIDL_result"));
            assert!(plan.rdl().contains("type RESULT = __MIDL_result"));
        }
    }
}

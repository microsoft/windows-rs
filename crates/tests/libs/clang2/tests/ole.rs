use windows_clang2::{Input, ProjectionOptions, capture};
use windows_metadata::{Type, reader::*};
#[path = "../sdk.rs"]
#[allow(dead_code)]
mod sdk;

#[test]
fn ole_callback_closure_compiles_without_external_value_bindings() {
    for arch in ["i686", "x86_64", "aarch64"] {
        let target = format!("--target={arch}-pc-windows-msvc");
        let arguments = sdk::arguments(&target);
        let arguments: Vec<_> = arguments.iter().map(String::as_str).collect();
        let mut baseline = None;
        for reversed in [false, true] {
            let mut inputs = [
                Input::new("ole.hpp", "#include <oaidl.h>"),
                Input::new("wic.hpp", "#include <wincodec.h>"),
            ];
            if reversed {
                inputs.reverse();
            }
            let snapshot =
                capture(inputs, &arguments, &["IDispatch", "ITypeInfo", "EXCEPINFO"]).unwrap();
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
                .join(format!("ole-callback-{arch}-{reversed}.winmd"));
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
            let record = index.expect("Test", "EXCEPINFO");
            assert_eq!(record.fields().count(), 9);
            let field = record
                .fields()
                .find(|field| field.name() == "pfnDeferredFillIn")
                .unwrap();
            let name = "tagEXCEPINFO_pfnDeferredFillIn_Callback";
            assert_eq!(field.ty(), Type::class_named("Test", name));
            let callback = index.expect("Test", name);
            let method = callback
                .methods()
                .find(|method| method.name() == "Invoke")
                .unwrap();
            assert_eq!(
                method.signature(&[]).types,
                [Type::PtrMut(
                    Box::new(Type::value_named("Test", "EXCEPINFO")),
                    1
                )]
            );
            assert_eq!(
                method.signature(&[]).return_type,
                Type::value_named("Test", "HRESULT")
            );
            assert_eq!(
                index
                    .expect("Test", "HRESULT")
                    .fields()
                    .next()
                    .unwrap()
                    .ty(),
                Type::I32
            );
            for (interface, method_name) in [("IDispatch", "Invoke"), ("ITypeInfo", "Invoke")] {
                assert!(
                    index
                        .expect("Test", interface)
                        .methods()
                        .any(|method| method.name() == method_name)
                );
            }
        }
    }
}

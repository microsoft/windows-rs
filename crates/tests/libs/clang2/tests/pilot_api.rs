use std::collections::BTreeMap;
use std::path::Path;
use windows_clang2::*;
use windows_metadata::{
    HasAttributes, MethodCallAttributes, PInvokeAttributes, Type, Value, reader::*,
};

fn input(source: &str, target: Target, names: &[&str]) -> Input {
    let mut input = Input::new("pilot_api.cpp", source, target);
    input
        .required
        .extend(names.iter().map(|name| name.to_string()));
    input
}

fn imports(snapshot: &Snapshot) -> BTreeMap<String, Import> {
    snapshot
        .functions()
        .map(|(decl, function)| {
            (
                function.linker_name.clone(),
                Import {
                    dll: "fixture.dll".into(),
                    export: if decl.qualified_name == "ImportCall" {
                        "OriginalImportCall".into()
                    } else {
                        decl.qualified_name.clone()
                    },
                },
            )
        })
        .collect()
}

fn read_metadata(rdl: &str, name: &str, target: Target) -> Index {
    let directory = Path::new(env!("OUT_DIR")).join(format!("{name}_{target:?}"));
    std::fs::create_dir_all(&directory).unwrap();
    let vocabulary = directory.join("vocabulary.winmd");
    windows_rdl::reader()
        .input_text(include_str!("..\\..\\..\\..\\..\\metadata\\metadata.rdl"))
        .output(&vocabulary)
        .write()
        .unwrap();
    let path = directory.join("api.winmd");
    windows_rdl::reader()
        .reference(&vocabulary)
        .input_text(rdl)
        .output(&path)
        .write()
        .unwrap();
    let index = Index::read(&path).unwrap();
    index.validate_native_contracts(None).unwrap();
    index
}

#[test]
fn reviewed_pilot_signatures_and_values_on_three_targets() {
    let compiler = Compiler::new().unwrap();
    for target in [Target::X86, Target::X64, Target::Arm64] {
        let snapshot = compiler
            .capture(&input(
                include_str!("..\\input\\pilot_api.h"),
                target,
                &[
                    "Choose",
                    "Echo",
                    "ImportCall",
                    "Missing",
                    "Variadic",
                    "WideAlias",
                    "StatusAlias",
                ],
            ))
            .unwrap();
        let routes = imports(&snapshot);
        assert_eq!(
            snapshot.rdl("Test").unwrap_err().stage,
            Stage::Representation
        );
        let rdl = snapshot.rdl_with_imports("Test", &routes).unwrap();
        assert_eq!(
            rdl,
            include_str!("..\\input\\pilot_api.rdl").replace("\r\n", "\n")
        );
        let index = read_metadata(&rdl, "pilot_api", target);
        let status = index.get("Test", "Status").next().unwrap();
        assert_eq!(status.fields().next().unwrap().ty(), Type::I32);
        let values = status
            .fields()
            .filter_map(|field| {
                field
                    .constant()
                    .map(|constant| (field.name(), constant.value()))
            })
            .collect::<Vec<_>>();
        assert_eq!(
            values,
            [
                ("Failed", Value::I32(-3)),
                ("FailedAlias", Value::I32(-3)),
                ("Passed", Value::I32(9))
            ]
        );
        let wide = index.get("Test", "Wide").next().unwrap();
        assert_eq!(wide.fields().next().unwrap().ty(), Type::U64);
        assert_eq!(
            wide.fields().nth(1).unwrap().constant().unwrap().value(),
            Value::U64(u64::MAX)
        );
        assert!(
            index
                .get("Test", "Mode")
                .next()
                .unwrap()
                .attributes()
                .any(|attr| attr.name() == "ScopedEnumAttribute")
        );
        let function = |name| match index.get_item("Test", name).next().unwrap() {
            Item::Fn(method) => method,
            _ => panic!("expected function"),
        };
        let echo = function("Echo").signature(&[]);
        assert_eq!(echo.return_type, Type::value_named("Test", "Status"));
        assert_eq!(
            echo.types,
            [
                Type::value_named("Test", "Status"),
                Type::PtrConst(Box::new(Type::value_named("Test", "Item")), 1)
            ]
        );
        let imported = function("ImportCall").impl_map().unwrap();
        assert_eq!(imported.import_scope().name(), "fixture.dll");
        assert_eq!(imported.import_name(), "OriginalImportCall");
        assert!(imported.flags().contains(PInvokeAttributes::CallConvCdecl));
        assert!(
            function("Variadic")
                .signature(&[])
                .flags
                .contains(MethodCallAttributes::VARARG)
        );
        let evidence = snapshot
            .functions()
            .find(|(decl, _)| decl.qualified_name == "ImportCall")
            .unwrap()
            .1;
        assert!(evidence.dll_import);
        assert_eq!(
            evidence.linker_name,
            if target == Target::X86 {
                "_ImportCall"
            } else {
                "ImportCall"
            }
        );
        assert_eq!(
            snapshot
                .parameters()
                .filter(|(decl, _)| decl.qualified_name == "Choose")
                .count(),
            4
        );
    }
}

#[test]
fn unsupported_callable_contracts_do_not_emit_lossy_output() {
    let compiler = Compiler::new().unwrap();
    for (source, name) in [
        ("int Function(int value = 3);", "Function"),
        ("int Function() noexcept;", "Function"),
        (
            "int Function(int __attribute__((annotate(\"contract\"))) value);",
            "Function",
        ),
        ("typedef int (*Callback)(int);", "Callback"),
        (
            "enum __attribute__((flag_enum)) Flags { Bit = 1 };",
            "Flags",
        ),
        ("enum Forward : int;", "Forward"),
    ] {
        assert!(
            compiler
                .capture(&input(source, Target::X64, &[name]))
                .is_err(),
            "{source}"
        );
    }
    let snapshot = compiler
        .capture(&input(
            "int Overload(int); int Overload(float);",
            Target::X64,
            &["Overload"],
        ))
        .unwrap();
    assert!(
        snapshot
            .rdl_with_imports("Test", &imports(&snapshot))
            .is_err()
    );
}

#[test]
fn native_conventions_and_stable_parameter_positions() {
    let compiler = Compiler::new().unwrap();
    for target in [Target::X86, Target::X64, Target::Arm64] {
        let snapshot = compiler
            .capture(&input(
                "extern \"C\" void __stdcall SystemCall(int value);
                 extern \"C\" void __fastcall RegisterCall(int value);",
                target,
                &["SystemCall", "RegisterCall"],
            ))
            .unwrap();
        let rdl = snapshot
            .rdl_with_imports("Test", &imports(&snapshot))
            .unwrap();
        let index = read_metadata(&rdl, "conventions", target);
        for (name, native, metadata) in [
            (
                "SystemCall",
                CallingConvention::Stdcall,
                PInvokeAttributes::CallConvPlatformapi,
            ),
            (
                "RegisterCall",
                CallingConvention::Fastcall,
                PInvokeAttributes::CallConvFastcall,
            ),
        ] {
            let function = snapshot
                .functions()
                .find(|(decl, _)| decl.qualified_name == name)
                .unwrap()
                .1;
            assert_eq!(
                function.convention,
                if target == Target::X86 {
                    native
                } else {
                    CallingConvention::C
                }
            );
            let Item::Fn(method) = index.get_item("Test", name).next().unwrap() else {
                panic!("expected function");
            };
            let mask = PInvokeAttributes::CallConvCdecl | PInvokeAttributes::CallConvFastcall;
            assert_eq!(
                method.impl_map().unwrap().flags() & mask,
                if target == Target::X86 {
                    metadata
                } else {
                    PInvokeAttributes::CallConvCdecl
                }
            );
        }
        let first = compiler
            .capture(&input(
                "void Name(int zeta, int alpha); void Name(int alpha, int zeta);",
                target,
                &["Name"],
            ))
            .unwrap();
        let second = compiler
            .capture(&input(
                "void Name(int alpha, int zeta); void Name(int zeta, int alpha);",
                target,
                &["Name"],
            ))
            .unwrap();
        assert_eq!(
            first.rdl_with_imports("Test", &imports(&first)).unwrap(),
            second.rdl_with_imports("Test", &imports(&second)).unwrap()
        );
    }
}

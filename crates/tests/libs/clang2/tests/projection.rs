use windows_clang2::{
    Input, Plan, ProjectionOptions, ReferenceKind, StringKind, TypeReference, capture,
};
use windows_metadata::{
    Type, Value,
    reader::{
        BufferRelationship, BytesWritten, HasAttributes, Index, Item, ParamDirection, TypeDef,
    },
};
#[path = "../sdk.rs"]
#[allow(dead_code)]
mod sdk;
use sdk::capture_sdk as sdk_capture;

const ARGS: &[&str] = &["-x", "c++", "--target=x86_64-pc-windows-msvc"];

#[test]
fn output_byte_postconditions_survive_metadata_and_roundtrip() {
    for target in ["i686", "x86_64", "aarch64"] {
        let target = format!("--target={target}-pc-windows-msvc");
        let snapshot = capture(
            [Input::new(
                "output.hpp",
                include_str!("../input/output_buffers.h"),
            )],
            &["-x", "c++", &target],
            &[
                "Capacity",
                "All",
                "AllOptional",
                "Partial",
                "PartialOptional",
                "ValueCount",
                "IOutput",
            ],
        )
        .unwrap();
        let plan = snapshot.resolve().unwrap().project(&options()).unwrap();
        let name = if target.contains("i686") {
            "output_buffers_x86"
        } else {
            "output_buffers"
        };
        let index = compile(name, &plan);
        check_output_postconditions(&index);
        let scratch = std::path::Path::new(env!("OUT_DIR")).join(name);
        let rdl = scratch.join("roundtrip.rdl");
        windows_rdl::writer()
            .input(scratch.join("test.winmd"))
            .filter("Test")
            .output(&rdl)
            .write()
            .unwrap();
        let winmd = scratch.join("roundtrip.winmd");
        windows_rdl::reader()
            .input(rdl)
            .input(sdk::projection_metadata())
            .reference_default()
            .output(&winmd)
            .write()
            .unwrap();
        check_output_postconditions(&Index::read(winmd).unwrap());
    }
}

fn check_output_postconditions(index: &Index) {
    for (name, written, optional) in [
        ("Capacity", None, false),
        (
            "All",
            Some(BytesWritten {
                parameter: 1,
                dereference: false,
            }),
            false,
        ),
        (
            "AllOptional",
            Some(BytesWritten {
                parameter: 1,
                dereference: false,
            }),
            true,
        ),
        (
            "Partial",
            Some(BytesWritten {
                parameter: 2,
                dereference: true,
            }),
            false,
        ),
        (
            "PartialOptional",
            Some(BytesWritten {
                parameter: 2,
                dereference: true,
            }),
            true,
        ),
        (
            "ValueCount",
            Some(BytesWritten {
                parameter: 2,
                dereference: false,
            }),
            false,
        ),
    ] {
        let Item::Fn(function) = index.expect_item("Test", name) else {
            panic!()
        };
        let params = function
            .params_by_sequence(function.signature(&[]).types.len())
            .unwrap();
        let buffer = params.params()[0].unwrap();
        assert_eq!(buffer.bytes_written(), written);
        assert_eq!(
            buffer.buffer_relationship(),
            Some(BufferRelationship::BytesParam(1))
        );
        assert_eq!(buffer.direction(), ParamDirection::Output);
        assert_eq!(buffer.is_optional(), optional);
    }
    let method = index.expect("Test", "IOutput").methods().next().unwrap();
    let params = method.params_by_sequence(3).unwrap();
    assert_eq!(
        params.params()[0].unwrap().bytes_written(),
        Some(BytesWritten {
            parameter: 2,
            dereference: true
        })
    );
}

#[test]
fn invalid_output_byte_postconditions_reject() {
    let source = include_str!("../input/output_buffers.h");
    for changed in [
        source.replace("_Out_ unsigned* written", "_In_ unsigned* written"),
        source.replace("_Out_ unsigned* written", "_Out_opt_ unsigned* written"),
        source.replace(
            "_Out_ unsigned* written",
            "_Out_ _Out_opt_ unsigned* written",
        ),
        source.replace("unsigned* written", "const unsigned* written"),
        source.replace("unsigned* written", "float* written"),
        source.replace("unsigned* written", "unsigned** written"),
        source.replace("capacity, *written)", "capacity, *written + 1)"),
        source.replace("capacity, *written)", "*written, capacity)"),
    ] {
        let snapshot = capture([Input::new("bad.hpp", changed)], ARGS, &["Partial"]).unwrap();
        assert!(snapshot.resolve().unwrap().project(&options()).is_err());
    }
}

#[test]
fn output_byte_relationships_bind_original_parameter_positions() {
    let source = include_str!("../input/output_buffers.h");
    let renamed = source
        .replace("capacity", "space")
        .replace("written", "used");
    for swapped in [false, true] {
        let inputs = [
            Input::new(if swapped { "z.hpp" } else { "a.hpp" }, source),
            Input::new(if swapped { "a.hpp" } else { "z.hpp" }, &renamed),
        ];
        let snapshot = capture(inputs, ARGS, &["Partial"]).unwrap();
        let rdl = snapshot
            .resolve()
            .unwrap()
            .project(&options())
            .unwrap()
            .rdl();
        assert!(rdl.contains("BytesParamIndex = 2, Dereference = true"));
        let changed = source.replace(
            "_Out_writes_bytes_all_(capacity) unsigned char* buffer",
            "_Out_writes_bytes_(capacity) unsigned char* buffer",
        );
        let snapshot = capture(
            [
                Input::new(if swapped { "z.hpp" } else { "a.hpp" }, source),
                Input::new(if swapped { "a.hpp" } else { "z.hpp" }, changed),
            ],
            ARGS,
            &["All"],
        )
        .unwrap();
        assert!(snapshot.resolve().is_err());
    }
}

#[test]
fn real_crypto_roots_imports_and_postconditions_are_complete() {
    for arch in ["i686", "x86_64", "aarch64"] {
        let target = format!("--target={arch}-pc-windows-msvc");
        let snapshot = sdk_capture(
            &target,
            include_str!("../input/sdk_buffers.h"),
            sdk::CRYPTO_ROOTS,
        );
        let resolved = snapshot.resolve().unwrap();
        assert!(resolved.report().incomplete.is_empty());
        let options = sdk::crypto_options(&target);
        let plan = resolved.project(&options).unwrap();
        let scratch = std::path::Path::new(env!("OUT_DIR")).join(format!("crypto-{arch}.winmd"));
        windows_rdl::reader()
            .input_text(&plan.rdl())
            .input(sdk::projection_metadata())
            .reference_default()
            .output(&scratch)
            .write()
            .unwrap();
        let index = Index::read(scratch).unwrap();
        for root in sdk::CRYPTO_ROOTS {
            index.expect_item("Crypto", root);
        }
        for (name, position, written, capacity, optional) in [
            (
                "BCryptCreateHash",
                2,
                BytesWritten {
                    parameter: 3,
                    dereference: false,
                },
                3,
                true,
            ),
            (
                "BCryptFinishHash",
                1,
                BytesWritten {
                    parameter: 2,
                    dereference: false,
                },
                2,
                false,
            ),
            (
                "BCryptGetProperty",
                2,
                BytesWritten {
                    parameter: 4,
                    dereference: true,
                },
                3,
                true,
            ),
        ] {
            let Item::Fn(function) = index.expect_item("Crypto", name) else {
                panic!()
            };
            let params = function
                .params_by_sequence(function.signature(&[]).types.len())
                .unwrap();
            let buffer = params.params()[position].unwrap();
            assert_eq!(buffer.bytes_written(), Some(written));
            assert_eq!(
                buffer.buffer_relationship(),
                Some(BufferRelationship::BytesParam(capacity))
            );
            assert_eq!(buffer.is_optional(), optional);
        }
        for root in sdk::CRYPTO_ROOTS
            .iter()
            .filter(|root| root.starts_with("BCrypt"))
        {
            let mut incomplete = sdk::crypto_options(&target);
            incomplete.imports.retain(|_, import| import.name != *root);
            let error = resolved.project(&incomplete).unwrap_err().to_string();
            assert!(
                error.contains(root) && error.contains("requires an import library"),
                "{error}"
            );
        }
    }
}

#[test]
fn export_contracts_do_not_replace_native_linker_evidence() {
    let snapshot = capture(
        [Input::new(
            "api.hpp",
            "extern \"C\" int __stdcall Compute(int value);",
        )],
        &["-x", "c++", "--target=i686-pc-windows-msvc"],
        &["Compute"],
    )
    .unwrap();
    let resolved = snapshot.resolve().unwrap();
    let mut options = options();
    let original = resolved.project(&options).unwrap().rdl();
    assert!(original.contains("import = \"_Compute@4\""), "{original}");
    options.imports.insert(
        "_Compute@4".into(),
        windows_clang2::FunctionImport {
            library: "other.dll".into(),
            name: "ComputeExport".into(),
        },
    );
    let projected = resolved.project(&options).unwrap().rdl();
    assert!(
        projected.contains("import = \"ComputeExport\""),
        "{projected}"
    );
    assert!(projected.contains("extern \"system\""), "{projected}");
    assert!(projected.contains("\"other.dll\""));
    options.imports.clear();
    assert_eq!(resolved.project(&options).unwrap().rdl(), original);
    options.library = None;
    assert!(resolved.project(&options).is_err());
}

#[test]
fn real_wdk_member_layouts_project() {
    for target in ["i686", "x86_64", "aarch64"] {
        let args = sdk::wdk_arguments(&format!("--target={target}-pc-windows-msvc"));
        for swapped in [false, true] {
            let mut inputs = [
                Input::new("a.hpp", include_str!("../input/wdk_layout.h")),
                Input::new("z.hpp", include_str!("../input/wdk_layout.h")),
            ];
            if swapped {
                inputs.reverse();
            }
            let snapshot = capture(
                inputs,
                &args.iter().map(String::as_str).collect::<Vec<_>>(),
                &["DeviceIoControl", "QuerySecurity"],
            )
            .unwrap();
            let resolved = snapshot.resolve().unwrap();
            assert!(resolved.report().incomplete.is_empty());
            let plan = resolved.project(&options()).unwrap();
            compile(
                if target == "i686" {
                    "wdk_layout_x86"
                } else {
                    "wdk_layout"
                },
                &plan,
            );
        }
    }
}

#[test]
fn shared_value_layouts_do_not_expand_the_call_guard_as_a_tree() {
    let mut source = "struct R0 { unsigned char value; };".to_string();
    for level in 1..=24 {
        let previous = level - 1;
        source.push_str(&format!(
            "struct R{level} {{ R{previous} a; R{previous} b; }};"
        ));
    }
    source.push_str("extern \"C\" void Use(R24 value);");
    let snapshot = capture([Input::new("shared.hpp", source)], ARGS, &["Use"]).unwrap();
    let plan = snapshot.resolve().unwrap().project(&options()).unwrap();
    assert_eq!(plan.rdl().matches("    struct R").count(), 25);
}

#[test]
fn adjusted_layouts_preserve_storage_but_reject_unproven_value_calls() {
    let source = include_str!("../input/adjusted_layouts.h");
    for root in ["ByValue", "ReturnValue", "AliasByValue"] {
        let snapshot = capture([Input::new("layout.hpp", source)], ARGS, &[root]).unwrap();
        let error = snapshot
            .resolve()
            .unwrap()
            .project(&options())
            .unwrap_err()
            .to_string();
        assert!(
            error.contains("by-value calls with adjusted record layouts"),
            "{error}"
        );
    }
    let snapshot = capture(
        [Input::new("layout.hpp", source)],
        ARGS,
        &["ByPointer", "Aligned"],
    )
    .unwrap();
    let plan = snapshot.resolve().unwrap().project(&options()).unwrap();
    compile("adjusted_layouts", &plan);
}

#[test]
fn selected_record_aliases_require_unambiguous_ownership() {
    let source = "struct Owner { struct { int value; } member; }; using A = decltype(Owner::member); using B = A;";
    for reverse in [false, true] {
        let mut inputs = [Input::new("a.hpp", source), Input::new("b.hpp", source)];
        if reverse {
            inputs.reverse();
        }
        let snapshot = capture(inputs, ARGS, &["A", "B"]).unwrap();
        let error = snapshot
            .resolve()
            .unwrap()
            .project(&options())
            .unwrap_err()
            .to_string();
        assert!(error.contains("multiple selected aliases"), "{error}");
    }
    let snapshot = capture([Input::new("a.hpp", source)], ARGS, &["A"]).unwrap();
    assert!(snapshot.resolve().unwrap().project(&options()).is_ok());
    let snapshot = capture(
        [
            Input::new("a.hpp", source),
            Input::new("b.hpp", source.replace("int value", "float value")),
        ],
        ARGS,
        &["A"],
    )
    .unwrap();
    assert!(snapshot.resolve().is_err());
}

#[test]
fn selected_named_record_alias_owns_the_projected_definition() {
    let source = include_str!("../input/named_alias.h");
    for swapped in [false, true] {
        let mut inputs = [Input::new("a.hpp", source), Input::new("b.hpp", source)];
        if swapped {
            inputs.reverse();
        }
        let snapshot = capture(inputs, ARGS, &["Use", "Point"]).unwrap();
        let plan = snapshot.resolve().unwrap().project(&options()).unwrap();
        let index = compile("named_alias", &plan);
        assert_eq!(index.types().filter(|ty| ty.name() == "Point").count(), 1);
    }
    let snapshot = capture([Input::new("a.hpp", source)], ARGS, &["Point", "Other"]).unwrap();
    assert!(snapshot.resolve().unwrap().project(&options()).is_err());
    let snapshot = capture([Input::new("a.hpp", source)], ARGS, &["Point", "tagPoint"]).unwrap();
    let plan = snapshot.resolve().unwrap().project(&options()).unwrap();
    compile("named_alias_explicit_tag", &plan);
}

#[test]
fn windows_flag_operators_preserve_native_signedness_and_identity() {
    let source = include_str!("../input/operator_flags.h");
    for target in ["i686", "x86_64", "aarch64"] {
        let target = format!("--target={target}-pc-windows-msvc");
        for (root, flags) in [("Flags", true), ("Plain", false)] {
            let snapshot = capture(
                [Input::new("flags.hpp", source)],
                &["-x", "c++", &target],
                &[root],
            )
            .unwrap();
            let plan = snapshot.resolve().unwrap().project(&options()).unwrap();
            let index = compile(&format!("operator_{}", root.replace("::", "_")), &plan);
            let Item::Type(ty) = index.expect_item("Test", root) else {
                panic!()
            };
            assert_eq!(ty.has_attribute("FlagsAttribute"), flags);
            assert_eq!(ty.underlying_type(), Some(Type::I32));
        }
        let snapshot = capture(
            [Input::new("flags.hpp", source)],
            &["-x", "c++", &target],
            &["Second::Flags"],
        )
        .unwrap();
        assert!(snapshot.dump().contains("flags: false"));
        let snapshot = capture(
            [
                Input::new("a.hpp", source),
                Input::new(
                    "b.hpp",
                    source.replace("DEFINE_ENUM_FLAG_OPERATORS(Flags)", ""),
                ),
            ],
            &["-x", "c++", &target],
            &["Flags"],
        )
        .unwrap();
        assert!(snapshot.resolve().is_err());
    }
}

#[test]
fn explicit_enum_flags_survive_projection() {
    let snapshot = capture(
        [Input::new(
            "flags.hpp",
            include_str!("../input/flag_enums.h"),
        )],
        ARGS,
        &["NativeFlags", "PlainEnum"],
    )
    .unwrap();
    let plan = snapshot.resolve().unwrap().project(&options()).unwrap();
    assert!(plan.rdl().contains("#[flags]"), "{}", plan.rdl());
    let index = compile("flag_enums", &plan);
    for (name, flags) in [("NativeFlags", true), ("PlainEnum", false)] {
        let Item::Type(ty) = index.expect_item("Test", name) else {
            panic!()
        };
        assert_eq!(ty.find_attribute("FlagsAttribute").is_some(), flags);
        assert_eq!(ty.underlying_type(), Some(Type::U32));
        assert_eq!(
            ty.fields()
                .filter_map(|field| field.constant())
                .map(|value| value.value())
                .collect::<Vec<_>>(),
            [Value::U32(0), Value::U32(1), Value::U32(2), Value::U32(3)]
        );
    }
}

#[test]
fn enum_flag_conflicts_cannot_be_hidden_by_external_bindings() {
    let source = include_str!("../input/flag_enums.h");
    for other in [
        source.replace("[[clang::flag_enum]]", ""),
        "enum NativeFlags : unsigned int;".into(),
    ] {
        for swapped in [false, true] {
            let snapshot = capture(
                [
                    Input::new(if swapped { "z.hpp" } else { "a.hpp" }, source),
                    Input::new(if swapped { "a.hpp" } else { "z.hpp" }, &other),
                ],
                ARGS,
                &["NativeFlags"],
            )
            .unwrap();
            assert!(snapshot.resolve().is_err());
        }
    }
}

#[test]
fn member_alignment_is_preserved() {
    for target in ["i686", "x86_64", "aarch64"] {
        let target_arg = format!("--target={target}-pc-windows-msvc");
        for root in ["DeviceIoControl", "QuerySecurity"] {
            let snapshot = capture(
                [Input::new(
                    "alignment.hpp",
                    include_str!("../input/member_alignment.h"),
                )],
                &["-x", "c++", &target_arg],
                &[root],
            )
            .unwrap();
            let resolved = snapshot.resolve().unwrap();
            let plan = resolved.project(&options()).unwrap();
            let index = compile(
                &if target == "i686" {
                    root.into()
                } else {
                    format!("{root}_64")
                },
                &plan,
            );
            let Item::Type(ty) = index.expect_item("Test", root) else {
                panic!()
            };
            assert_eq!(
                ty.fields()
                    .filter(|field| !field.name().starts_with("__padding"))
                    .count(),
                if root == "DeviceIoControl" { 4 } else { 2 }
            );
        }
    }
}

#[test]
fn double_null_annotations_are_not_silently_dropped() {
    for source in [
        include_str!("../input/double_null.h").to_string(),
        include_str!("../input/double_null.h").replace("SAL(\"_Post_\")", ""),
    ] {
        let snapshot =
            capture([Input::new("strings.hpp", source)], ARGS, &["MultiString"]).unwrap();
        let error = snapshot
            .resolve()
            .unwrap()
            .project(&options())
            .unwrap_err()
            .to_string();
        assert!(error.contains("_NullNull_terminated_"), "{error}");
    }
}

#[test]
fn interface_object_direction_is_not_an_output_slot() {
    let source = include_str!("../input/interface_directions.h");
    let snapshot = capture([Input::new("objects.hpp", source)], ARGS, &["ITranslator"]).unwrap();
    assert!(
        snapshot
            .resolve()
            .unwrap()
            .project(&options())
            .unwrap_err()
            .to_string()
            .contains("_Outptr_")
    );
    let snapshot = capture(
        [Input::new(
            "objects.hpp",
            source.replace("annotate(\"_Outptr_\")", "annotate(\"_COM_Outptr_\")"),
        )],
        ARGS,
        &["ITranslator", "MutateObject"],
    )
    .unwrap();
    let plan = snapshot.resolve().unwrap().project(&options()).unwrap();
    assert!(
        plan.rdl().contains("fn Mutate(&self, #[in] context:"),
        "{}",
        plan.rdl()
    );
    let index = compile("interface_directions", &plan);
    let Item::Type(ty) = index.expect_item("Test", "ITranslator") else {
        panic!()
    };
    let methods: Vec<_> = ty.methods().collect();
    assert_eq!(
        methods[0].params().next().unwrap().direction(),
        ParamDirection::Input
    );
    assert_eq!(
        methods[1].params().next().unwrap().direction(),
        ParamDirection::Output
    );
    assert_eq!(
        methods[0].signature(&[]).types,
        [Type::class_named("Test", "IContext")]
    );
    assert_eq!(
        methods[1].signature(&[]).types,
        [Type::PtrMut(
            Box::new(Type::class_named("Test", "IContext")),
            1
        )]
    );
    let Item::Fn(function) = index.expect_item("Test", "MutateObject") else {
        panic!()
    };
    assert_eq!(
        function.params().next().unwrap().direction(),
        ParamDirection::Input
    );
}

#[test]
fn interface_direction_distinguishes_objects_from_pointer_slots() {
    for (annotation, object, slot) in [
        ("_Out_", "#[in] ", "#[out] "),
        ("_Inout_", "#[in] ", "#[in] #[out] "),
        ("_Out_opt_", "#[in] #[opt] ", "#[out] #[opt] "),
        ("_Inout_opt_", "#[in] #[opt] ", "#[in] #[out] #[opt] "),
    ] {
        for (pointers, attributes, projected) in
            [("*", object, "IContext"), ("**", slot, "*mut IContext")]
        {
            let source = include_str!("../input/interface_directions.h")
                .replace(
                    "annotate(\"_Inout_\")",
                    &format!("annotate(\"{annotation}\")"),
                )
                .replace(
                    "void MutateObject(_Inout_ IContext*",
                    &format!("void MutateObject(_Inout_ IContext{pointers}"),
                );
            let snapshot =
                capture([Input::new("objects.hpp", source)], ARGS, &["MutateObject"]).unwrap();
            let rdl = snapshot
                .resolve()
                .unwrap()
                .project(&options())
                .unwrap()
                .rdl();
            assert!(
                rdl.contains(&format!("fn MutateObject({attributes}p0: {projected})")),
                "{rdl}"
            );
        }
    }
}

#[test]
fn output_name_collisions_and_unproven_overload_abi_are_rejected() {
    let source = include_str!("../input/overloads.h");
    for target in ["i686", "x86_64", "aarch64"] {
        let target = format!("--target={target}-pc-windows-msvc");
        let args = &["-x", "c++", &target];
        let snapshot = capture([Input::new("overloads.hpp", source)], args, &["Use"]).unwrap();
        let error = snapshot
            .resolve()
            .unwrap()
            .project(&options())
            .unwrap_err()
            .to_string();
        assert!(error.contains("multiple native entities"), "{error}");
        let snapshot =
            capture([Input::new("overloads.hpp", source)], args, &["IOverloads"]).unwrap();
        let error = snapshot
            .resolve()
            .unwrap()
            .project(&options())
            .unwrap_err()
            .to_string();
        assert!(
            error.contains("overloaded COM methods require native vtable ordering"),
            "{error}"
        );
    }
}

#[test]
fn inherited_virtual_slots_are_not_projected_as_new_methods() {
    for target in ["i686", "x86_64", "aarch64"] {
        let target = format!("--target={target}-pc-windows-msvc");
        for explicit in [false, true] {
            let source = include_str!("../input/interface_overrides.h");
            let source = if explicit {
                source.to_string()
            } else {
                source.replace(" override", "")
            };
            for root in ["IDirect", "IIndirect"] {
                let snapshot = capture(
                    [Input::new("overrides.hpp", &source)],
                    &["-x", "c++", &target],
                    &[root],
                )
                .unwrap();
                let resolved = snapshot.resolve().unwrap();
                for external_base in [false, true] {
                    let mut options = options();
                    if external_base {
                        options.references.insert(
                            "IBase".into(),
                            TypeReference {
                                namespace: "External".into(),
                                name: "IUnknown".into(),
                                kind: ReferenceKind::Interface,
                            },
                        );
                    }
                    let error = resolved.project(&options).unwrap_err().to_string();
                    assert!(error.contains("inherited virtual slot"), "{error}");
                }
            }
        }
    }
}

#[test]
fn enums_preserve_representation_values_and_uses() {
    for target in [
        "--target=x86_64-pc-windows-msvc",
        "--target=i686-pc-windows-msvc",
        "--target=aarch64-pc-windows-msvc",
    ] {
        for reverse in [false, true] {
            let source = include_str!("../input/enums.h");
            let mut inputs = [Input::new("a.hpp", source), Input::new("b.hpp", source)];
            if reverse {
                inputs.reverse();
            }
            let snapshot = capture(
                inputs,
                &["-x", "c++", target],
                &["ConvertEnum", "IEnums", "State"],
            )
            .unwrap();
            let plan = snapshot.resolve().unwrap().project(&options()).unwrap();
            let index = compile(
                if target == "--target=i686-pc-windows-msvc" {
                    "enums_x86"
                } else {
                    "enums"
                },
                &plan,
            );
            for (name, repr, values) in [
                ("Signed", Type::I8, vec![Value::I8(-128), Value::I8(127)]),
                ("Wide", Type::U64, vec![Value::U64(u64::MAX)]),
                (
                    "Scoped",
                    Type::U16,
                    vec![Value::U16(0), Value::U16(u16::MAX)],
                ),
                (
                    "State",
                    Type::I32,
                    vec![Value::I32(-1), Value::I32(2), Value::I32(2)],
                ),
            ] {
                let Item::Type(ty) = index.expect_item("Test", name) else {
                    panic!()
                };
                assert_eq!(ty.underlying_type(), Some(repr));
                assert_eq!(
                    ty.fields()
                        .filter_map(|field| field.constant())
                        .map(|value| value.value())
                        .collect::<Vec<_>>(),
                    values
                );
            }
            let Item::Type(ty) = index.expect_item("Test", "IEnums") else {
                panic!()
            };
            let signature = ty.methods().next().unwrap().signature(&[]);
            assert_eq!(signature.return_type, Type::value_named("Test", "State"));
            assert_eq!(signature.types[0], Type::value_named("Test", "Scoped"));
        }
    }
}

#[test]
fn enum_conflicts_and_unsupported_representations_are_rejected() {
    for reverse in [false, true] {
        let mut inputs = [
            Input::new("a.hpp", "enum E : int { A = 1 };"),
            Input::new("b.hpp", "enum E : int { A = 2 };"),
        ];
        if reverse {
            inputs.reverse();
        }
        assert!(capture(inputs, ARGS, &["E"]).unwrap().resolve().is_err());
    }
    let snapshot = capture(
        [Input::new(
            "a.hpp",
            "enum E : bool { A = false, B = true };",
        )],
        ARGS,
        &["E"],
    )
    .unwrap();
    assert!(
        snapshot
            .resolve()
            .unwrap()
            .project(&options())
            .unwrap_err()
            .to_string()
            .contains("enum representation requires an integer scalar")
    );
}

#[test]
fn com_record_results_are_rejected() {
    for target in [
        "--target=i686-pc-windows-msvc",
        "--target=x86_64-pc-windows-msvc",
        "--target=aarch64-pc-windows-msvc",
    ] {
        for result in ["Small", "Large", "Alias", "Large*"] {
            let snapshot = capture(
                [Input::new(
                    "api.hpp",
                    format!(
                        "#define RESULT {result}\n{}",
                        include_str!("../input/com_record_result.h")
                    ),
                )],
                &["-x", "c++", target],
                &["IRecordResult"],
            )
            .unwrap();
            let resolved = snapshot.resolve().unwrap();
            let plan = resolved.project(&options());
            if result == "Large*" {
                assert!(plan.is_ok(), "{target}: {plan:?}");
            } else {
                assert!(
                    plan.unwrap_err().to_string().contains(
                        "`IRecordResult::Get`: by-value record results are not supported for COM methods"
                    ),
                    "{target}: {result}"
                );
            }
        }
    }
}

#[test]
fn external_record_fields_retain_native_layout() {
    let source = include_str!("../input/external_field.h");
    let mut options = options();
    options.references.insert(
        "_GUID".into(),
        TypeReference {
            namespace: "External".into(),
            name: "GUID".into(),
            kind: ReferenceKind::Value,
        },
    );
    for conflict in [false, true] {
        let other = source.replace("data4[8]", if conflict { "data4[7]" } else { "data4[8]" });
        let snapshot = capture(
            [Input::new("a.hpp", source), Input::new("b.hpp", other)],
            ARGS,
            &["Notification"],
        )
        .unwrap();
        if conflict {
            assert!(snapshot.resolve().is_err());
        } else {
            let plan = snapshot.resolve().unwrap().project(&options).unwrap();
            compile("external_field", &plan);
        }
    }
    let snapshot = capture(
        [Input::new(
            "call.hpp",
            format!("{source}\nextern \"C\" void Use(Notification value);"),
        )],
        ARGS,
        &["Use"],
    )
    .unwrap();
    let error = snapshot
        .resolve()
        .unwrap()
        .project(&options)
        .unwrap_err()
        .to_string();
    assert!(error.contains("by-value external record"), "{error}");
}

#[test]
fn fixed_arrays_preserve_layout_and_call_guards() {
    let source = include_str!("../input/fixed_arrays.h");
    let snapshot = capture([Input::new("arrays.hpp", source)], ARGS, &["Matrix"]).unwrap();
    let plan = snapshot.resolve().unwrap().project(&options()).unwrap();
    compile("fixed_arrays", &plan);
    let snapshot = capture([Input::new("arrays.hpp", source)], ARGS, &["Use"]).unwrap();
    let error = snapshot
        .resolve()
        .unwrap()
        .project(&options())
        .unwrap_err()
        .to_string();
    assert!(
        error.contains("by-value calls with adjusted record layouts"),
        "{error}"
    );
}

fn options() -> ProjectionOptions {
    let mut options = ProjectionOptions::new("Test");
    options.library = Some("test.dll".into());
    for (name, kind) in [
        ("Data", ReferenceKind::Value),
        ("IUnknown", ReferenceKind::Interface),
    ] {
        options.references.insert(
            name.into(),
            TypeReference {
                namespace: "External".into(),
                name: name.into(),
                kind,
            },
        );
    }
    options
}

fn string_options() -> ProjectionOptions {
    let mut options = options();
    for (kind, name) in [
        (StringKind::Ansi, "PSTR"),
        (StringKind::AnsiConst, "PCSTR"),
        (StringKind::Wide, "PWSTR"),
        (StringKind::WideConst, "PCWSTR"),
    ] {
        options.string_references.insert(
            kind,
            TypeReference {
                namespace: "External".into(),
                name: name.into(),
                kind: ReferenceKind::Value,
            },
        );
    }
    options
}

fn scalar_options() -> ProjectionOptions {
    let mut options = options();
    options.references.insert(
        "Status".into(),
        TypeReference {
            namespace: "External".into(),
            name: "Status".into(),
            kind: ReferenceKind::Value,
        },
    );
    options
}

fn pointer_options() -> ProjectionOptions {
    let mut options = string_options();
    for (native, name) in [("Handle", "Handle"), ("Text", "PCWSTR")] {
        options.references.insert(
            native.into(),
            TypeReference {
                namespace: "External".into(),
                name: name.into(),
                kind: ReferenceKind::Value,
            },
        );
    }
    options
}

#[test]
fn midl_properties_preserve_source_markers_not_method_name_heuristics() {
    let source = include_str!("../input/midl_properties.h");
    for target in ["i686", "x86_64", "aarch64"] {
        let target = format!("--target={target}-pc-windows-msvc");
        let args = ["-x", "c++", &target];
        for swapped in [false, true] {
            let mut inputs = [Input::new("a.hpp", source), Input::new("b.hpp", source)];
            if swapped {
                inputs.reverse();
            }
            let snapshot = capture(inputs, &args, &["Properties"]).unwrap();
            let plan = snapshot.resolve().unwrap().project(&options()).unwrap();
            let index = compile("midl_properties", &plan);
            let Item::Type(ty) = index.expect_item("Test", "Properties") else {
                panic!()
            };
            for (method, special) in ty.methods().zip([true, true, false]) {
                assert_eq!(
                    method
                        .flags()
                        .contains(windows_metadata::MethodAttributes::SpecialName),
                    special,
                );
            }
        }
        for changed in [
            source.replace("[propget]", "[propput]"),
            source.replace("/* [propget] */", ""),
        ] {
            let snapshot = capture(
                [Input::new("a.hpp", source), Input::new("b.hpp", changed)],
                &args,
                &["Properties"],
            )
            .unwrap();
            assert!(snapshot.resolve().is_err());
        }
        let snapshot = capture(
            [Input::new(
                "a.hpp",
                source.replace("get_Value", "put_Other"),
            )],
            &args,
            &["Properties"],
        )
        .unwrap();
        assert!(snapshot.resolve().unwrap().project(&options()).is_err());
    }
}

#[test]
fn midl_directions_preserve_function_and_method_contracts() {
    let snapshot = capture(
        [Input::new(
            "directions.hpp",
            include_str!("../input/midl_directions.h"),
        )],
        ARGS,
        &["Directions", "IDirections", "SalWins"],
    )
    .unwrap();
    let plan = snapshot.resolve().unwrap().project(&options()).unwrap();
    let index = compile("midl_directions", &plan);
    let Item::Fn(function) = index.expect_item("Test", "Directions") else {
        panic!()
    };
    let parameters = function.params_by_sequence(4).unwrap();
    for (parameter, direction) in parameters.params().iter().zip([
        ParamDirection::Input,
        ParamDirection::Output,
        ParamDirection::InputOutput,
        ParamDirection::Input,
    ]) {
        assert_eq!(parameter.unwrap().direction(), direction);
    }
    assert!(parameters.params()[3].unwrap().is_optional());
    let Item::Fn(sal_wins) = index.expect_item("Test", "SalWins") else {
        panic!()
    };
    assert_eq!(
        sal_wins.params().next().unwrap().direction(),
        ParamDirection::Input
    );
    let Item::Type(interface) = index.expect_item("Test", "IDirections") else {
        panic!()
    };
    let method = interface.methods().next().unwrap();
    assert_eq!(
        method.params().map(|p| p.direction()).collect::<Vec<_>>(),
        [
            ParamDirection::Input,
            ParamDirection::Output,
            ParamDirection::InputOutput
        ]
    );
}

#[test]
fn midl_direction_evidence_agrees_across_observations() {
    for other in [
        "/* [in] */",
        "__attribute__((annotate(\"_In_\")))",
        "/* [out] */",
    ] {
        for reverse in [false, true] {
            for swapped in [false, true] {
                let mut inputs = [
                    Input::new(
                        if swapped { "b.hpp" } else { "a.hpp" },
                        "extern \"C\" void Use(/* [in] */ int* first);",
                    ),
                    Input::new(
                        if swapped { "a.hpp" } else { "b.hpp" },
                        format!("extern \"C\" void Use({other} int* second);"),
                    ),
                ];
                if reverse {
                    inputs.reverse();
                }
                let snapshot = capture(inputs, ARGS, &["Use"]).unwrap();
                if other == "/* [out] */" {
                    assert!(
                        snapshot
                            .resolve()
                            .err()
                            .unwrap()
                            .to_string()
                            .contains("conflicting annotations")
                    );
                } else {
                    assert!(snapshot.resolve().unwrap().project(&options()).is_ok());
                }
            }
        }
    }
    let snapshot = capture([Input::new("api.hpp",
        "extern \"C\" void Use(/* [in] */ int* first); extern \"C\" void Use(/* [out] */ int* second);")],
        ARGS, &["Use"]).unwrap();
    assert!(snapshot.resolve().is_err());
}

#[test]
fn sal_precedence_cannot_hide_annotation_family_conflicts() {
    for (midl, sal, accepted) in [
        ("/* [out][in] */", "_In_", true),
        ("", "_In_", true),
        ("/* [out][in] */", "", true),
        ("/* [in] */", "_In_", false),
        ("/* [out][in] */", "_Out_", false),
    ] {
        for reverse in [false, true] {
            let source = |midl: &str, sal: &str| {
                let sal = if sal.is_empty() {
                    String::new()
                } else {
                    format!("__attribute__((annotate(\"{sal}\")))")
                };
                format!("extern \"C\" void Use({midl} {sal} int* value);")
            };
            let mut inputs = [
                Input::new("a.hpp", source("/* [out][in] */", "_In_")),
                Input::new("b.hpp", source(midl, sal)),
            ];
            if reverse {
                inputs.reverse();
            }
            let snapshot = capture(inputs, ARGS, &["Use"]).unwrap();
            if accepted {
                let rdl = snapshot
                    .resolve()
                    .unwrap()
                    .project(&options())
                    .unwrap()
                    .rdl();
                assert!(rdl.contains("#[in] p0: *mut i32"), "{rdl}");
                assert!(!rdl.contains("#[out]"), "{rdl}");
            } else {
                assert!(snapshot.resolve().is_err());
            }
        }
    }
}

#[test]
fn pointer_references_preserve_value_and_indirection_contracts() {
    let snapshot = capture(
        [Input::new(
            "pointers.hpp",
            include_str!("../input/pointer_references.h"),
        )],
        ARGS,
        &["Use", "IHandles", "Handle"],
    )
    .unwrap();
    let plan = snapshot
        .resolve()
        .unwrap()
        .project(&pointer_options())
        .unwrap();
    assert_eq!(plan.omitted()["Handle"], "provided by external metadata");
    let index = compile("pointer_references", &plan);
    let Item::Fn(function) = index.expect_item("Test", "Use") else {
        panic!()
    };
    let signature = function.signature(&[]);
    assert_eq!(
        signature.return_type,
        Type::value_named("External", "Handle")
    );
    assert_eq!(
        signature.types[1],
        Type::PtrMut(Box::new(Type::value_named("External", "Handle")), 1)
    );
    assert_eq!(
        signature.types[2],
        Type::PtrMut(Box::new(Type::value_named("External", "PCWSTR")), 1)
    );
    assert_eq!(
        signature.types[3],
        Type::PtrConst(Box::new(Type::value_named("External", "PCWSTR")), 1)
    );
    let Item::Type(interface) = index.expect_item("Test", "IHandles") else {
        panic!()
    };
    let setter = interface.methods().next().unwrap();
    assert_eq!(
        setter.signature(&[]).types,
        [
            Type::value_named("External", "Handle"),
            Type::value_named("External", "PCWSTR"),
        ]
    );
    for param in setter.params() {
        assert_eq!(param.direction(), ParamDirection::Input);
    }
}

#[test]
fn pointer_contracts_reject_conflicting_observations() {
    for other in ["Handle", "Alias", "void*"] {
        for reverse in [false, true] {
            for swapped in [false, true] {
                let source = |parameter: &str| {
                    format!(
                        "typedef void* Handle; typedef Handle Alias; extern \"C\" void Use({parameter} value);"
                    )
                };
                let mut inputs = [
                    Input::new(if swapped { "z.hpp" } else { "a.hpp" }, source("Handle")),
                    Input::new(if swapped { "a.hpp" } else { "z.hpp" }, source(other)),
                ];
                if reverse {
                    inputs.reverse();
                }
                let snapshot = capture(inputs, ARGS, &["Use"]).unwrap();
                let resolved = snapshot.resolve().unwrap();
                let plan = resolved.project(&pointer_options());
                if other == "void*" {
                    assert!(
                        plan.unwrap_err()
                            .to_string()
                            .contains("conflicting projected typedef contracts")
                    );
                } else {
                    assert!(plan.is_ok(), "{plan:?}");
                }
            }
        }
    }
    let source = |ty: &str| format!("struct Record {{ {ty} value; }}; typedef Record* Handle;");
    let snapshot = capture(
        [
            Input::new("a.hpp", source("int")),
            Input::new("b.hpp", source("float")),
        ],
        ARGS,
        &["Handle"],
    )
    .unwrap();
    assert!(snapshot.resolve().is_err());
}

#[test]
fn pointer_string_annotations_require_matching_contracts() {
    let snapshot = capture(
        [Input::new(
            "strings.hpp",
            "typedef const unsigned short* Text;
             extern \"C\" void Use(__attribute__((annotate(\"_In_z_\"))) Text text);",
        )],
        ARGS,
        &["Use"],
    )
    .unwrap();
    let resolved = snapshot.resolve().unwrap();
    assert!(resolved.project(&pointer_options()).is_ok());
    let mut options = pointer_options();
    options
        .string_references
        .get_mut(&StringKind::WideConst)
        .unwrap()
        .name = "Other".into();
    assert!(
        resolved
            .project(&options)
            .unwrap_err()
            .to_string()
            .contains("conflicting typedef and string bindings")
    );
    options = pointer_options();
    options.references.get_mut("Text").unwrap().kind = ReferenceKind::Interface;
    assert!(resolved.project(&options).is_err());
}

#[test]
fn scalar_references_preserve_typedef_contracts() {
    let snapshot = capture(
        [Input::new(
            "api.hpp",
            include_str!("../input/scalar_references.h"),
        )],
        ARGS,
        &["Convert", "IStatus", "Status"],
    )
    .unwrap();
    let plan = snapshot
        .resolve()
        .unwrap()
        .project(&scalar_options())
        .unwrap();
    assert_eq!(plan.omitted()["Status"], "provided by external metadata");
    let index = compile("scalar_references", &plan);
    let Item::Fn(function) = index.expect_item("Test", "Convert") else {
        panic!()
    };
    assert_eq!(
        function.signature(&[]).return_type,
        Type::value_named("External", "Status")
    );
    assert_eq!(
        function.signature(&[]).types[0],
        Type::value_named("External", "Status")
    );
    let Item::Type(interface) = index.expect_item("Test", "IStatus") else {
        panic!()
    };
    assert_eq!(
        interface
            .methods()
            .next()
            .unwrap()
            .signature(&[])
            .return_type,
        Type::value_named("External", "Status")
    );
}

#[test]
fn scalar_contract_redeclarations_are_order_independent() {
    for methods in [false, true] {
        for other in ["Status", "Alias", "long"] {
            for reverse in [false, true] {
                let declaration = if methods {
                    "struct __declspec(uuid(\"00000001-0000-0000-c000-000000000046\")) API {
                        virtual RESULT __stdcall Call(PARAM value) = 0;
                    };"
                } else {
                    "extern \"C\" RESULT API(PARAM value);"
                };
                for (result, param) in [(other, "Status"), ("Status", other)] {
                    let source = |result: &str, param: &str| {
                        format!(
                            "typedef long Status; typedef Status Alias;\n{}",
                            declaration
                                .replace("RESULT", result)
                                .replace("PARAM", param)
                        )
                    };
                    let mut inputs = [
                        Input::new("a.hpp", source("Status", "Status")),
                        Input::new("b.hpp", source(result, param)),
                    ];
                    if reverse {
                        inputs.reverse();
                    }
                    let snapshot = capture(inputs, ARGS, &["API"]).unwrap();
                    let resolved = snapshot.resolve().unwrap();
                    let plan = resolved.project(&scalar_options());
                    if other != "long" {
                        assert!(plan.is_ok(), "{plan:?}");
                    } else {
                        assert!(
                            plan.unwrap_err()
                                .to_string()
                                .contains("conflicting projected typedef contracts")
                        );
                    }
                    assert!(resolved.project(&options()).is_ok());
                }
            }
        }
    }
}

#[test]
fn scalar_references_reject_invalid_native_contracts() {
    for native in ["void", "int&", "double[2]"] {
        let source = if native == "double[2]" {
            "typedef double Status[2];".into()
        } else {
            format!("typedef {native} Status;")
        };
        let snapshot = capture([Input::new("api.hpp", source)], ARGS, &["Status"]).unwrap();
        assert!(
            snapshot
                .resolve()
                .unwrap()
                .project(&scalar_options())
                .is_err()
        );
    }
    let snapshot = capture(
        [Input::new("api.hpp", "typedef long Status;")],
        ARGS,
        &["Status"],
    )
    .unwrap();
    let mut options = scalar_options();
    options.references.get_mut("Status").unwrap().kind = ReferenceKind::Interface;
    assert!(snapshot.resolve().unwrap().project(&options).is_err());
    let conflict = capture(
        [
            Input::new("a.hpp", "typedef long Status;"),
            Input::new("b.hpp", "typedef unsigned long Status;"),
        ],
        ARGS,
        &["Status"],
    )
    .unwrap();
    assert!(conflict.resolve().is_err());
}

#[test]
fn scalar_contracts_cannot_hide_behind_alias_representatives() {
    for root in ["Alias", "Use", "Container"] {
        for reverse in [false, true] {
            let source = |target: &str| {
                format!(
                    "typedef long Status; typedef {target} Alias;
                 struct Container {{ Alias value; }};
                 extern \"C\" Alias Use(Alias value);"
                )
            };
            let mut inputs = [
                Input::new("a.hpp", source("Status")),
                Input::new("b.hpp", source("long")),
            ];
            if reverse {
                inputs.reverse();
            }
            let snapshot = capture(inputs, ARGS, &[root]).unwrap();
            let resolved = snapshot.resolve().unwrap();
            assert!(resolved.project(&options()).is_ok());
            assert!(
                resolved
                    .project(&scalar_options())
                    .unwrap_err()
                    .to_string()
                    .contains("conflicting projected typedef contracts for `Alias`")
            );
        }
    }
}

#[test]
fn observation_contracts_cover_fields_constants_and_callables() {
    let source = |field: &str, result: &str, param: &str| {
        format!(
            "#define FIELD {field}\n#define RESULT {result}\n#define PARAM {param}\n{}",
            include_str!("../input/observation_contracts.h")
        )
    };
    for root in ["Record", "Constant", "Call"] {
        for other in ["Status", "Alias", "long"] {
            for swapped in [false, true] {
                for reverse in [false, true] {
                    let mut inputs = [
                        Input::new(
                            if swapped { "z.hpp" } else { "a.hpp" },
                            source("Status", "Status", "Status"),
                        ),
                        Input::new(
                            if swapped { "a.hpp" } else { "z.hpp" },
                            source(other, other, other),
                        ),
                    ];
                    if reverse {
                        inputs.reverse();
                    }
                    let snapshot = capture(inputs, ARGS, &[root]).unwrap();
                    match snapshot.resolve() {
                        Ok(resolved) => {
                            let projected = resolved.project(&scalar_options());
                            if root == "Constant" {
                                // No initializer: projection must fail even when types agree.
                                assert!(projected.is_err());
                            } else if other != "long" {
                                assert!(projected.is_ok(), "{root}: {projected:?}");
                            } else {
                                assert!(projected.is_err(), "{root}: silently chose one contract");
                            }
                        }
                        Err(error) => {
                            assert_ne!(other, "Status", "{root}: {error}");
                            assert_ne!(root, "Call", "{error}");
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn same_name_tag_roots_emit_one_definition() {
    for reverse in [false, true] {
        let source = include_str!("../input/tag_alias.h");
        let mut inputs = [Input::new("a.hpp", source), Input::new("b.hpp", source)];
        if reverse {
            inputs.reverse();
        }
        let snapshot = capture(inputs, ARGS, &["Record", "IFoo"]).unwrap();
        let plan = snapshot.resolve().unwrap().project(&options()).unwrap();
        compile("tag_alias", &plan);
        assert!(plan.omitted().is_empty());
    }
}

#[test]
fn anonymous_aggregate_bindings_cannot_hide_member_conflicts() {
    let source = include_str!("../input/anonymous.h");
    for conflict in [false, true] {
        let snapshot = capture(
            [
                Input::new("a.hpp", source),
                Input::new(
                    "b.hpp",
                    source.replace(
                        "#define VALUE int",
                        if conflict {
                            "#define VALUE float"
                        } else {
                            "#define VALUE int"
                        },
                    ),
                ),
            ],
            ARGS,
            &["Use"],
        )
        .unwrap();
        if conflict {
            assert!(snapshot.resolve().is_err());
        } else {
            let resolved = snapshot.resolve().unwrap();
            let error = resolved.project(&options()).unwrap_err().to_string();
            assert!(error.contains("anonymous aggregate projection"), "{error}");
            let mut options = options();
            options.references.insert(
                "Packet".into(),
                TypeReference {
                    namespace: "External".into(),
                    name: "Data".into(),
                    kind: ReferenceKind::Value,
                },
            );
            compile("anonymous_external", &resolved.project(&options).unwrap());
        }
    }
}

fn compile(name: &str, plan: &Plan) -> Index {
    let scratch = std::path::Path::new(env!("OUT_DIR")).join(name);
    std::fs::create_dir_all(&scratch).unwrap();
    let reference = scratch.join("external.winmd");
    windows_rdl::reader()
        .input_text(include_str!("../input/external.rdl"))
        .output(&reference)
        .write()
        .unwrap();
    let rdl = plan.rdl();
    let expected = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("expected")
        .join(format!("{name}.rdl"));
    if std::env::var_os("UPDATE_EXPECT").is_some() {
        std::fs::write(&expected, &rdl).unwrap();
    }
    assert_eq!(rdl, std::fs::read_to_string(expected).unwrap());
    let output = scratch.join("test.winmd");
    windows_rdl::reader()
        .input_text(&rdl)
        .input(sdk::projection_metadata())
        .reference_default()
        .reference(reference)
        .output(&output)
        .write()
        .unwrap();
    Index::read(output).unwrap()
}

#[test]
fn external_references_do_not_hide_type_root_conflicts() {
    let source = include_str!("../input/dependencies.h");
    for root in ["Packet", "Use", "Callback", "ITEM"] {
        let snapshot = capture(
            [
                Input::new("a.hpp", format!("#define VALUE int\n{source}")),
                Input::new("b.hpp", format!("#define VALUE float\n{source}")),
            ],
            ARGS,
            &[root],
        )
        .unwrap();
        assert!(snapshot.resolve().is_err());
    }
    let snapshot = capture(
        [Input::new(
            "api.hpp",
            format!("#define VALUE int\n{source}"),
        )],
        ARGS,
        &["Packet", "Use"],
    )
    .unwrap();
    let resolved = snapshot.resolve().unwrap();
    let plan = resolved.project(&options()).unwrap();
    let index = compile("referenced_record", &plan);
    let Item::Type(packet) = index.expect_item("Test", "Packet") else {
        panic!()
    };
    assert_eq!(
        packet.fields().next().unwrap().ty(),
        Type::PtrMut(Box::new(Type::value_named("External", "Data")), 1)
    );
    let Item::Fn(function) = index.expect_item("Test", "Use") else {
        panic!()
    };
    assert_eq!(
        function.signature(&[]).types,
        [Type::PtrMut(
            Box::new(Type::value_named("Test", "Packet")),
            1
        )]
    );
}

#[test]
fn indirect_interface_constants_are_omitted_after_native_resolution() {
    let source = include_str!("../input/interface_constants.h");
    for indirect in [true, false] {
        let source = if indirect {
            source.to_string()
        } else {
            source.replace("typedef Raw LPUNKNOWN;", "typedef IUnknown* LPUNKNOWN;")
        };
        let snapshot = capture(
            [Input::new("api.hpp", source)],
            ARGS,
            &["UNKNOWN_NULL", "INTEGER_CONSTANT", "Use"],
        )
        .unwrap();
        if indirect {
            assert!(snapshot.dump().contains("] Raw "));
            assert!(snapshot.dump().contains("] LPUNKNOWN "));
        }
        let resolved = snapshot.resolve().unwrap();
        let plan = resolved.project(&options()).unwrap();
        assert_eq!(
            plan.omitted().get("UNKNOWN_NULL").unwrap(),
            "interface values are not metadata constants"
        );
        let index = compile("interface_constants", &plan);
        let Item::Fn(function) = index.expect_item("Test", "Use") else {
            panic!()
        };
        assert_eq!(
            function.signature(&[]).types,
            [
                Type::class_named("External", "IUnknown"),
                Type::PtrMut(Box::new(Type::class_named("External", "IUnknown")), 1),
            ]
        );
        let Item::Const(constant) = index.expect_item("Test", "INTEGER_CONSTANT") else {
            panic!()
        };
        assert_eq!(constant.constant().unwrap().value(), Value::I32(42));
    }
}

#[test]
fn mismatched_record_layout_is_not_emitted() {
    let snapshot = capture(
        [Input::new(
            "pack.hpp",
            "#pragma pack(push, 1)\nstruct Packet { char flag; int value; };\n#pragma pack(pop)",
        )],
        ARGS,
        &["Packet"],
    )
    .unwrap();
    let resolved = snapshot.resolve().unwrap();
    assert!(
        resolved
            .project(&options())
            .unwrap_err()
            .to_string()
            .contains("packing")
    );
}

#[test]
fn incompatible_interface_observations_are_not_hidden_by_binding() {
    let snapshot = capture([
        Input::new("a.hpp", "struct IUnknown { virtual void Method(int value) = 0; }; extern \"C\" void Use(IUnknown* value);"),
        Input::new("b.hpp", "struct IUnknown { virtual void Method(float value) = 0; }; extern \"C\" void Use(IUnknown* value);"),
    ], ARGS, &["Use"]).unwrap();
    assert!(snapshot.resolve().is_err());
}

#[test]
fn real_iunknown_projects_through_a_checked_external_binding() {
    let snapshot = sdk_capture(
        "--target=x86_64-pc-windows-msvc",
        "#include <unknwnbase.h>\nextern \"C\" void Use(IUnknown* input, IUnknown** output);",
        &["Use"],
    );
    let resolved = snapshot.resolve().unwrap();
    assert!(snapshot.dump().contains("name: \"QueryInterface\""));
    let index = compile("real_iunknown", &resolved.project(&options()).unwrap());
    let Item::Fn(function) = index.expect_item("Test", "Use") else {
        panic!()
    };
    assert_eq!(
        function.signature(&[]).types,
        [
            Type::class_named("External", "IUnknown"),
            Type::PtrMut(Box::new(Type::class_named("External", "IUnknown")), 1),
        ]
    );
}

#[test]
fn real_interfaces_project_locally() {
    for (expected, target) in [
        ("sdk_interfaces", "--target=x86_64-pc-windows-msvc"),
        ("sdk_interfaces_x86", "--target=i686-pc-windows-msvc"),
        ("sdk_interfaces", "--target=aarch64-pc-windows-msvc"),
    ] {
        let snapshot = sdk_capture(target, include_str!("../input/sdk_interfaces.h"), &["Use"]);
        let mut options = ProjectionOptions::new("Test");
        options.library = Some("test.dll".into());
        options.references.insert(
            "_GUID".into(),
            TypeReference {
                namespace: "External".into(),
                name: "GUID".into(),
                kind: ReferenceKind::Value,
            },
        );
        let resolved = snapshot.resolve().unwrap();
        let plan = resolved.project(&options).unwrap();
        let index = compile(expected, &plan);
        let Item::Type(unknown) = index.expect_item("Test", "IUnknown") else {
            panic!()
        };
        assert_guid(unknown, 0, 0x46);
        assert_eq!(
            unknown
                .methods()
                .map(|method| method.name())
                .collect::<Vec<_>>(),
            ["QueryInterface", "AddRef", "Release"]
        );
        let query = unknown.methods().next().unwrap();
        assert_eq!(query.signature(&[]).return_type, Type::I32);
        assert_eq!(
            query.signature(&[]).types,
            [
                Type::PtrConst(Box::new(Type::value_named("External", "GUID")), 1),
                Type::PtrMut(Box::new(Type::Void), 2),
            ]
        );
        let parameters = query.params_by_sequence(2).unwrap();
        let output = parameters.params()[1].unwrap();
        assert_eq!(output.direction(), ParamDirection::Output);
        assert!(output.has_attribute("ComOutPtrAttribute"));
        assert!(!output.is_optional());
        for method in unknown.methods().skip(1) {
            assert_eq!(method.signature(&[]).return_type, Type::U32);
            assert!(method.signature(&[]).types.is_empty());
        }

        let Item::Type(factory) = index.expect_item("Test", "IClassFactory") else {
            panic!()
        };
        assert_guid(factory, 1, 0x46);
        assert_eq!(
            factory.interface_impls().next().unwrap().interface(&[]),
            Type::class_named("Test", "IUnknown")
        );
        assert_eq!(
            factory
                .methods()
                .map(|method| method.name())
                .collect::<Vec<_>>(),
            ["CreateInstance", "LockServer"]
        );
        let create = factory.methods().next().unwrap();
        assert_eq!(
            create.signature(&[]).types,
            [
                Type::class_named("Test", "IUnknown"),
                Type::PtrConst(Box::new(Type::value_named("External", "GUID")), 1),
                Type::PtrMut(Box::new(Type::Void), 2),
            ]
        );
        let parameters = create.params_by_sequence(3).unwrap();
        assert_eq!(
            parameters.params()[0].unwrap().direction(),
            ParamDirection::Input
        );
        assert!(parameters.params()[0].unwrap().is_optional());
        assert_eq!(
            parameters.params()[1].unwrap().direction(),
            ParamDirection::Input
        );
        assert_eq!(
            parameters.params()[2].unwrap().direction(),
            ParamDirection::Output
        );
        assert!(
            parameters.params()[2]
                .unwrap()
                .has_attribute("ComOutPtrAttribute")
        );
        assert_eq!(create.signature(&[]).return_type, Type::I32);
        let lock = factory.methods().nth(1).unwrap();
        assert_eq!(lock.signature(&[]).types, [Type::I32]);
        assert_eq!(lock.signature(&[]).return_type, Type::I32);

        let Item::Fn(function) = index.expect_item("Test", "Use") else {
            panic!()
        };
        assert_eq!(
            function.signature(&[]).types,
            [
                Type::class_named("Test", "IUnknown"),
                Type::PtrMut(Box::new(Type::class_named("Test", "IUnknown")), 1),
                Type::class_named("Test", "IClassFactory"),
            ]
        );
    }
}

fn assert_guid(ty: TypeDef<'_>, first: u32, last: u8) {
    let values: Vec<_> = ty
        .find_attribute("GuidAttribute")
        .unwrap()
        .value()
        .into_iter()
        .map(|(_, value)| value)
        .collect();
    assert_eq!(
        values,
        [
            Value::U32(first),
            Value::U16(0),
            Value::U16(0),
            Value::U8(0xc0),
            Value::U8(0),
            Value::U8(0),
            Value::U8(0),
            Value::U8(0),
            Value::U8(0),
            Value::U8(0),
            Value::U8(last),
        ]
    );
}

#[test]
fn projection_rejects_unknown_layout_and_annotations() {
    for (source, root, reason) in [
        (
            "struct Data { int value; }; extern \"C\" void Use(Data value);",
            "Use",
            "by-value external",
        ),
        (
            "struct Data { int value; }; extern \"C\" Data Use();",
            "Use",
            "by-value external",
        ),
        ("struct Packet {};", "Packet", "empty record"),
        (
            "extern \"C\" void Use(int n, int* __attribute__((annotate(\"_Out_writes_to_(n,n)\"))) data);",
            "Use",
            "annotation projection",
        ),
    ] {
        let snapshot = capture([Input::new("api.hpp", source)], ARGS, &[root]).unwrap();
        assert!(
            snapshot
                .resolve()
                .unwrap()
                .project(&options())
                .unwrap_err()
                .to_string()
                .contains(reason)
        );
    }
}

#[test]
fn projection_consumes_merged_annotation_evidence() {
    for reversed in [false, true] {
        let mut declarations = [
            "extern \"C\" void Fill(int* buffer);",
            "extern \"C\" void Fill(int* __attribute__((annotate(\"_Out_\"))) data);",
        ];
        if reversed {
            declarations.reverse();
        }
        let snapshot = capture(
            [Input::new("api.hpp", declarations.join("\n"))],
            ARGS,
            &["Fill"],
        )
        .unwrap();
        let index = compile(
            "annotations",
            &snapshot.resolve().unwrap().project(&options()).unwrap(),
        );
        let Item::Fn(function) = index.expect_item("Test", "Fill") else {
            panic!()
        };
        let parameters = function.params_by_sequence(1).unwrap();
        assert_eq!(
            parameters.params()[0].unwrap().direction(),
            ParamDirection::Output
        );
    }
}

#[test]
fn buffer_lengths_preserve_units_direction_and_parameter_positions() {
    for optional in [false, true] {
        let mut source = include_str!("../input/sal_buffers.h").to_string();
        if optional {
            for name in [
                "_In_reads_",
                "_Out_writes_",
                "_Inout_updates_",
                "_In_reads_bytes_",
                "_Out_writes_bytes_",
                "_Inout_updates_bytes_",
            ] {
                source = source.replace(&format!("{name}("), &format!("{name}opt_("));
            }
        }
        let snapshot = capture(
            [Input::new("api.hpp", source)],
            ARGS,
            &["Buffers", "IBuffers", "Zero", "Max"],
        )
        .unwrap();
        let index = compile(
            if optional {
                "sal_buffers_optional"
            } else {
                "sal_buffers"
            },
            &snapshot.resolve().unwrap().project(&options()).unwrap(),
        );
        let Item::Fn(function) = index.expect_item("Test", "Buffers") else {
            panic!()
        };
        let Item::Type(interface) = index.expect_item("Test", "IBuffers") else {
            panic!()
        };
        for (method, offset, count_index) in [
            (function, 1, 0),
            (interface.methods().next().unwrap(), 0, 7),
        ] {
            let parameters = method.params_by_sequence(8).unwrap();
            for (index, direction) in [
                ParamDirection::Input,
                ParamDirection::Output,
                ParamDirection::InputOutput,
                ParamDirection::Input,
                ParamDirection::Output,
                ParamDirection::InputOutput,
                ParamDirection::Input,
            ]
            .into_iter()
            .enumerate()
            {
                let parameter = parameters.params()[offset + index].unwrap();
                assert_eq!(parameter.direction(), direction);
                assert_eq!(parameter.is_optional(), optional);
                assert_eq!(
                    parameter.buffer_relationship(),
                    Some(match index {
                        0..=2 => BufferRelationship::ElementsParam(count_index),
                        3..=5 => BufferRelationship::BytesParam(count_index),
                        _ => BufferRelationship::ElementsConst(4),
                    })
                );
            }
            assert_eq!(
                parameters.params()[usize::try_from(count_index).unwrap()]
                    .unwrap()
                    .buffer_relationship(),
                None
            );
            assert!(
                !parameters.params()[usize::try_from(count_index).unwrap()]
                    .unwrap()
                    .is_optional()
            );
        }
        for (name, count) in [("Zero", 0), ("Max", i32::MAX)] {
            let Item::Fn(function) = index.expect_item("Test", name) else {
                panic!()
            };
            assert_eq!(
                function.params_by_sequence(1).unwrap().params()[0]
                    .unwrap()
                    .buffer_relationship(),
                Some(BufferRelationship::ElementsConst(count)),
            );
            assert_eq!(
                function.params_by_sequence(1).unwrap().params()[0]
                    .unwrap()
                    .is_optional(),
                optional
            );
        }
    }
}

#[test]
fn buffer_lengths_use_original_redeclaration_bindings() {
    let source = include_str!("../input/annotation_context.h");
    for reversed in [false, true] {
        let mut inputs = [
            Input::new("a.hpp", source),
            Input::new("b.hpp", format!("#define REVERSE\n{source}")),
        ];
        if reversed {
            let [left, right] = &mut inputs;
            std::mem::swap(&mut left.source, &mut right.source);
        }
        let snapshot = capture(inputs, ARGS, &["Fill"]).unwrap();
        let index = compile(
            "buffer_redeclarations",
            &snapshot.resolve().unwrap().project(&options()).unwrap(),
        );
        let Item::Fn(function) = index.expect_item("Test", "Fill") else {
            panic!()
        };
        assert_eq!(
            function.params_by_sequence(2).unwrap().params()[1]
                .unwrap()
                .buffer_relationship(),
            Some(BufferRelationship::ElementsParam(0))
        );
    }
}

#[test]
fn real_sdk_buffer_lengths_project() {
    let snapshot = sdk_capture(
        "--target=x86_64-pc-windows-msvc",
        include_str!("../input/sdk_buffers.h"),
        &["BCryptHashData", "BCryptGenRandom"],
    );
    let mut options = ProjectionOptions::new("Test");
    options.library = Some("bcrypt.dll".into());
    let index = compile(
        "sdk_buffers",
        &snapshot.resolve().unwrap().project(&options).unwrap(),
    );
    for (name, direction) in [
        ("BCryptHashData", ParamDirection::Input),
        ("BCryptGenRandom", ParamDirection::Output),
    ] {
        let Item::Fn(function) = index.expect_item("Test", name) else {
            panic!()
        };
        assert_eq!(
            function.signature(&[]).types,
            [
                Type::PtrMut(Box::new(Type::Void), 1),
                Type::PtrMut(Box::new(Type::U8), 1),
                Type::U32,
                Type::U32,
            ]
        );
        let parameters = function.params_by_sequence(4).unwrap();
        let buffer = parameters.params()[1].unwrap();
        assert_eq!(buffer.direction(), direction);
        assert_eq!(
            buffer.buffer_relationship(),
            Some(BufferRelationship::BytesParam(2))
        );
    }
}

#[test]
fn real_sdk_optional_buffers_project() {
    let snapshot = sdk_capture(
        "--target=x86_64-pc-windows-msvc",
        include_str!("../input/sdk_buffers.h"),
        &["BCryptDeriveKeyPBKDF2"],
    );
    let mut options = ProjectionOptions::new("Test");
    options.library = Some("bcrypt.dll".into());
    let index = compile(
        "sdk_optional_buffers",
        &snapshot.resolve().unwrap().project(&options).unwrap(),
    );
    let Item::Fn(function) = index.expect_item("Test", "BCryptDeriveKeyPBKDF2") else {
        panic!()
    };
    assert_eq!(
        function.signature(&[]).types,
        [
            Type::PtrMut(Box::new(Type::Void), 1),
            Type::PtrMut(Box::new(Type::U8), 1),
            Type::U32,
            Type::PtrMut(Box::new(Type::U8), 1),
            Type::U32,
            Type::U64,
            Type::PtrMut(Box::new(Type::U8), 1),
            Type::U32,
            Type::U32,
        ]
    );
    let parameters = function.params_by_sequence(9).unwrap();
    for (index, parameter) in parameters.params().iter().enumerate() {
        let parameter = parameter.unwrap();
        assert_eq!(parameter.is_optional(), matches!(index, 1 | 3));
        assert_eq!(
            parameter.direction(),
            if index == 6 {
                ParamDirection::Output
            } else {
                ParamDirection::Input
            }
        );
        assert_eq!(
            parameter.buffer_relationship(),
            match index {
                1 => Some(BufferRelationship::BytesParam(2)),
                3 => Some(BufferRelationship::BytesParam(4)),
                6 => Some(BufferRelationship::BytesParam(7)),
                _ => None,
            }
        );
    }
}

#[test]
fn optional_pointers_preserve_direction() {
    let snapshot = capture(
        [Input::new(
            "api.hpp",
            include_str!("../input/optional_pointers.h"),
        )],
        ARGS,
        &["Use", "IOptional"],
    )
    .unwrap();
    let index = compile(
        "optional_pointers",
        &snapshot.resolve().unwrap().project(&options()).unwrap(),
    );
    let Item::Fn(function) = index.expect_item("Test", "Use") else {
        panic!()
    };
    let Item::Type(interface) = index.expect_item("Test", "IOptional") else {
        panic!()
    };
    for method in [function, interface.methods().next().unwrap()] {
        let parameters = method.params_by_sequence(3).unwrap();
        for (parameter, direction) in parameters.params().iter().zip([
            ParamDirection::Input,
            ParamDirection::Output,
            ParamDirection::InputOutput,
        ]) {
            let parameter = parameter.unwrap();
            assert_eq!(parameter.direction(), direction);
            assert!(parameter.is_optional());
            assert_eq!(parameter.buffer_relationship(), None);
        }
    }
}

#[test]
fn unsupported_buffer_lengths_are_errors() {
    for (count, buffer, annotation, reason) in [
        (
            "int",
            "int",
            "_In_reads_(count)",
            "requires a buffer pointer",
        ),
        (
            "int",
            "const int*",
            "_Out_writes_(count)",
            "requires a writable buffer",
        ),
        (
            "int",
            "const int*",
            "_Inout_updates_(count)",
            "requires a writable buffer",
        ),
        ("int", "void*", "_In_reads_(count)", "non-void element"),
        ("float", "int*", "_In_reads_(count)", "integer parameter"),
        ("bool", "int*", "_In_reads_(count)", "integer parameter"),
        ("int*", "int*", "_In_reads_(count)", "integer parameter"),
        ("int", "int*", "_In_reads_(data)", "integer parameter"),
        (
            "int",
            "int*",
            "_In_reads_(missing)",
            "unsupported buffer length",
        ),
        (
            "int",
            "int*",
            "_In_reads_(count + 1)",
            "unsupported buffer length",
        ),
        (
            "int*",
            "int*",
            "_In_reads_(*count)",
            "unsupported buffer length",
        ),
        ("int", "int*", "_In_reads_(-1)", "unsupported buffer length"),
        (
            "int",
            "int*",
            "_In_reads_(2147483648)",
            "unsupported buffer length",
        ),
        (
            "int",
            "int*",
            "_In_reads_(010)",
            "unsupported buffer length",
        ),
        (
            "int",
            "int*",
            "_In_reads_(0x10)",
            "unsupported buffer length",
        ),
        (
            "int",
            "int*",
            "_In_reads_($32768)",
            "unsupported buffer length",
        ),
        (
            "int",
            "int*",
            "_In_reads_($+0)",
            "unsupported buffer length",
        ),
        ("int", "int*", "_In_reads_($2)", "integer parameter"),
        (
            "int",
            "int*",
            "_In_reads_bytes_(4)",
            "unsupported buffer length",
        ),
        (
            "int",
            "int*",
            "_Out_writes_to_(count,count)",
            "annotation projection",
        ),
        (
            "int",
            "int",
            "_In_reads_opt_(count)",
            "requires a buffer pointer",
        ),
        (
            "int",
            "const int*",
            "_Out_writes_opt_(count)",
            "requires a writable buffer",
        ),
        (
            "int",
            "const int*",
            "_Inout_updates_bytes_opt_(count)",
            "requires a writable buffer",
        ),
        ("int", "void*", "_In_reads_opt_(count)", "non-void element"),
        (
            "float",
            "int*",
            "_In_reads_bytes_opt_(count)",
            "integer parameter",
        ),
        (
            "int",
            "int*",
            "_In_reads_opt_(count+1)",
            "unsupported buffer length",
        ),
        ("int", "int", "_Out_opt_", "requires a writable pointer"),
        (
            "int",
            "const int*",
            "_Out_opt_",
            "requires a writable pointer",
        ),
        (
            "int",
            "const int*",
            "_Inout_opt_",
            "requires a writable pointer",
        ),
        ("int", "char*", "_In_opt_z_", "explicit string binding"),
        (
            "int",
            "char*",
            "_Out_writes_opt_z_(count)",
            "annotation projection",
        ),
    ] {
        let snapshot = capture(
            [Input::new("api.hpp", format!(
                "#define COUNT_TYPE {count}\n#define BUFFER_TYPE {buffer}\n#define ANNOTATION {annotation:?}\n{}",
                include_str!("../input/buffer_length_case.h"),
            ))],
            ARGS,
            &["Buffer"],
        ).unwrap();
        let error = snapshot.resolve().unwrap().project(&options()).unwrap_err();
        assert!(
            error.to_string().contains(reason),
            "{count}, {buffer}, {annotation}: {error}"
        );
    }
}

#[test]
fn string_bindings_preserve_constness_direction_and_optionality() {
    let plan = {
        let snapshot = capture(
            [Input::new("api.hpp", include_str!("../input/strings.h"))],
            ARGS,
            &["Strings", "IStrings"],
        )
        .unwrap();
        snapshot
            .resolve()
            .unwrap()
            .project(&string_options())
            .unwrap()
    };
    let index = compile("strings", &plan);
    let Item::Fn(function) = index.expect_item("Test", "Strings") else {
        panic!()
    };
    let Item::Type(interface) = index.expect_item("Test", "IStrings") else {
        panic!()
    };
    for method in [function, interface.methods().next().unwrap()] {
        assert_eq!(
            method.signature(&[]).types,
            [
                Type::value_named("External", "PCSTR"),
                Type::value_named("External", "PSTR"),
                Type::value_named("External", "PSTR"),
                Type::value_named("External", "PWSTR"),
                Type::value_named("External", "PCWSTR"),
                Type::PtrMut(Box::new(Type::I8), 1),
            ]
        );
        let parameters = method.params_by_sequence(6).unwrap();
        for (index, direction) in [
            ParamDirection::Input,
            ParamDirection::Input,
            ParamDirection::Output,
            ParamDirection::InputOutput,
            ParamDirection::Input,
            ParamDirection::Output,
        ]
        .into_iter()
        .enumerate()
        {
            let parameter = parameters.params()[index].unwrap();
            assert_eq!(parameter.direction(), direction);
            assert_eq!(parameter.is_optional(), index == 4);
        }
    }
}

#[test]
fn strings_require_valid_explicit_bindings_and_character_pointers() {
    for kind in [
        StringKind::Ansi,
        StringKind::AnsiConst,
        StringKind::Wide,
        StringKind::WideConst,
    ] {
        let snapshot = capture(
            [Input::new("api.hpp", include_str!("../input/strings.h"))],
            ARGS,
            &["Strings"],
        )
        .unwrap();
        let resolved = snapshot.resolve().unwrap();
        let mut options = string_options();
        options.string_references.remove(&kind);
        assert!(
            resolved
                .project(&options)
                .unwrap_err()
                .to_string()
                .contains("explicit string binding")
        );
        let mut options = string_options();
        options.string_references.get_mut(&kind).unwrap().kind = ReferenceKind::Interface;
        assert!(
            resolved
                .project(&options)
                .unwrap_err()
                .to_string()
                .contains("metadata value types")
        );
        let mut options = string_options();
        options.string_references.get_mut(&kind).unwrap().name = "invalid-name".into();
        assert!(
            resolved
                .project(&options)
                .unwrap_err()
                .to_string()
                .contains("unsupported output identifier")
        );
    }
    for (ty, annotation, reason) in [
        ("int", "_In_z_", "single character pointer"),
        ("char**", "_In_z_", "single character pointer"),
        ("void*", "_In_z_", "characters"),
        ("int*", "_In_z_", "characters"),
        ("short*", "_In_z_", "characters"),
        ("const char*", "_Out_z_", "writable string pointer"),
        (
            "const unsigned short*",
            "_Inout_z_",
            "writable string pointer",
        ),
    ] {
        let snapshot = capture(
            [Input::new("api.hpp", format!(
                "#define COUNT_TYPE int\n#define BUFFER_TYPE {ty}\n#define ANNOTATION {annotation:?}\n{}",
                include_str!("../input/buffer_length_case.h"),
            ))],
            ARGS, &["Buffer"],
        ).unwrap();
        let error = snapshot
            .resolve()
            .unwrap()
            .project(&string_options())
            .unwrap_err();
        assert!(
            error.to_string().contains(reason),
            "{ty} {annotation}: {error}"
        );
    }
}

#[test]
fn string_bindings_do_not_hide_conflicting_native_dependencies() {
    let source = include_str!("../input/strings.h");
    let snapshot = capture(
        [
            Input::new("a.hpp", source),
            Input::new(
                "b.hpp",
                source.replace("typedef const char* Narrow;", "typedef const int* Narrow;"),
            ),
        ],
        ARGS,
        &["Strings"],
    )
    .unwrap();
    assert!(snapshot.resolve().is_err());
}

#[test]
fn real_sdk_string_parameter_projects() {
    let snapshot = sdk_capture(
        "--target=x86_64-pc-windows-msvc",
        include_str!("../input/sdk_strings.h"),
        &["WinHttpTimeToSystemTime"],
    );
    let mut options = string_options();
    options.library = Some("winhttp.dll".into());
    let index = compile(
        "sdk_strings",
        &snapshot.resolve().unwrap().project(&options).unwrap(),
    );
    let Item::Fn(function) = index.expect_item("Test", "WinHttpTimeToSystemTime") else {
        panic!()
    };
    assert_eq!(
        function.signature(&[]).types,
        [
            Type::value_named("External", "PCWSTR"),
            Type::PtrMut(Box::new(Type::value_named("Test", "_SYSTEMTIME")), 1),
        ]
    );
    let parameters = function.params_by_sequence(2).unwrap();
    assert_eq!(
        parameters.params()[0].unwrap().direction(),
        ParamDirection::Input
    );
    assert_eq!(
        parameters.params()[1].unwrap().direction(),
        ParamDirection::Output
    );
}

#[test]
fn multiple_buffer_lengths_are_not_silently_combined() {
    let snapshot = capture(
        [Input::new(
            "api.hpp",
            include_str!("../input/sal_buffers.h").replace(
                "_In_reads_(count) const int* input",
                "_In_reads_(count) _In_reads_bytes_(count) const int* input",
            ),
        )],
        ARGS,
        &["Buffers"],
    )
    .unwrap();
    assert!(
        snapshot
            .resolve()
            .unwrap()
            .project(&options())
            .unwrap_err()
            .to_string()
            .contains("multiple buffer-length annotations")
    );
}

#[test]
fn plan_uses_checked_completion_and_owns_its_rendering_data() {
    let plan = {
        let snapshot = capture(
            [
                Input::new("a.hpp", "struct Data; struct Packet { Data* value; };"),
                Input::new("b.hpp", "struct Data { int value; };"),
            ],
            ARGS,
            &["Packet"],
        )
        .unwrap();
        snapshot
            .resolve()
            .unwrap()
            .project(&ProjectionOptions::new("Test"))
            .unwrap()
    };
    let index = compile("completion", &plan);
    let Item::Type(data) = index.expect_item("Test", "Data") else {
        panic!()
    };
    assert_eq!(data.fields().next().unwrap().ty(), Type::I32);
}

#[test]
fn local_interfaces_project_real_com_metadata() {
    let source = include_str!("../input/interface.h");
    let snapshot = capture([Input::new("api.hpp", source)], ARGS, &["Use"]).unwrap();
    let resolved = snapshot.resolve().unwrap();
    let mut options = ProjectionOptions::new("Test");
    options.library = Some("test.dll".into());
    let plan = resolved.project(&options).unwrap();
    let index = compile("local_interface", &plan);

    let Item::Type(base) = index.expect_item("Test", "IBase") else {
        panic!()
    };
    assert_guid(base, 0, 0x46);
    assert!(base.extends().is_none());
    let base_methods: Vec<_> = base.methods().map(|method| method.name()).collect();
    assert_eq!(base_methods, ["Query", "Count"]);

    let Item::Type(derived) = index.expect_item("Test", "IDerived") else {
        panic!()
    };
    assert_guid(derived, 0, 0x47);
    assert_eq!(
        derived.interface_impls().next().unwrap().interface(&[]),
        Type::class_named("Test", "IBase")
    );
    let derived_methods: Vec<_> = derived.methods().map(|method| method.name()).collect();
    assert_eq!(derived_methods, ["Notify"]);
    let notify = derived.methods().next().unwrap();
    assert_eq!(
        notify.signature(&[]).types,
        [Type::I32, Type::PtrMut(Box::new(Type::I32), 1)]
    );
    let parameters = notify.params_by_sequence(2).unwrap();
    assert_eq!(
        parameters.params()[0].unwrap().direction(),
        ParamDirection::Input
    );
    assert_eq!(
        parameters.params()[1].unwrap().direction(),
        ParamDirection::Output
    );

    let Item::Fn(function) = index.expect_item("Test", "Use") else {
        panic!()
    };
    assert_eq!(
        function.signature(&[]).types,
        [Type::class_named("Test", "IDerived")]
    );
}

#[test]
fn com_output_annotations_preserve_pointer_contracts() {
    let snapshot = capture(
        [Input::new(
            "api.hpp",
            include_str!("../input/com_parameters.h"),
        )],
        ARGS,
        &["Create", "IFactory"],
    )
    .unwrap();
    let index = compile(
        "com_parameters",
        &snapshot.resolve().unwrap().project(&options()).unwrap(),
    );
    let Item::Fn(function) = index.expect_item("Test", "Create") else {
        panic!()
    };
    let Item::Type(interface) = index.expect_item("Test", "IFactory") else {
        panic!()
    };
    for method in [function, interface.methods().next().unwrap()] {
        assert_eq!(
            method.signature(&[]).types,
            [
                Type::PtrMut(Box::new(Type::Void), 2),
                Type::PtrMut(Box::new(Type::class_named("External", "IUnknown")), 1),
                Type::class_named("External", "IUnknown"),
            ]
        );
        let parameters = method.params_by_sequence(3).unwrap();
        for (index, parameter) in parameters.params()[..2].iter().enumerate() {
            let parameter = parameter.unwrap();
            assert_eq!(parameter.direction(), ParamDirection::Output);
            assert_eq!(parameter.has_attribute("ComOutPtrAttribute"), index == 0);
            assert!(!parameter.is_optional());
        }
        let optional = parameters.params()[2].unwrap();
        assert_eq!(optional.direction(), ParamDirection::Input);
        assert!(optional.is_optional());
    }
}

#[test]
fn com_output_annotations_reject_incompatible_types() {
    for ty in [
        "int",
        "int*",
        "int**",
        "void*",
        "void***",
        "const void**",
        "IUnknown*",
    ] {
        let snapshot = capture(
            [Input::new(
                "api.hpp",
                format!(
                    "#define OUTPUT_TYPE {ty}\n{}",
                    include_str!("../input/invalid_com_output.h")
                ),
            )],
            ARGS,
            &["Create"],
        )
        .unwrap();
        assert!(
            snapshot
                .resolve()
                .unwrap()
                .project(&options())
                .unwrap_err()
                .to_string()
                .contains("_COM_Outptr_ requires"),
            "{ty}"
        );
    }
}

#[test]
fn interface_methods_require_the_com_calling_convention() {
    for (target, convention) in [
        ("i686", "__cdecl"),
        ("i686", "__thiscall"),
        ("i686", "__fastcall"),
        ("x86_64", "__vectorcall"),
    ] {
        let target = format!("--target={target}-pc-windows-msvc");
        let snapshot = capture(
            [Input::new(
                "api.hpp",
                include_str!("../input/interface.h").replace("__stdcall", convention),
            )],
            &["-x", "c++", &target],
            &["Use"],
        )
        .unwrap();
        assert!(
            snapshot
                .resolve()
                .unwrap()
                .project(&options())
                .unwrap_err()
                .to_string()
                .contains("calling convention incompatible with COM"),
            "{target}: {convention}"
        );
    }
}

#[test]
fn numeric_and_pointer_macro_values_roundtrip() {
    let snapshot = capture(
        [Input::new(
            "values.hpp",
            include_str!("../input/constants.h"),
        )],
        ARGS,
        &[
            "NEGATIVE",
            "UNSIGNED",
            "FLOAT",
            "DOUBLE",
            "POINTER",
            "BOOLEAN",
            "WHOLE",
            "NEGATIVE_ZERO",
        ],
    )
    .unwrap();
    let index = compile(
        "constants",
        &snapshot.resolve().unwrap().project(&options()).unwrap(),
    );
    use windows_metadata::Value;
    for (name, value) in [
        ("NEGATIVE", Value::I32(-42)),
        ("UNSIGNED", Value::U32(u32::MAX)),
        ("FLOAT", Value::F32(1.25)),
        ("WHOLE", Value::F32(2.0)),
        ("DOUBLE", Value::F64(2.5)),
        ("BOOLEAN", Value::Bool(true)),
    ] {
        let Item::Const(field) = index.expect_item("Test", name) else {
            panic!()
        };
        assert_eq!(field.constant().unwrap().value(), value);
    }
    let Item::Const(field) = index.expect_item("Test", "POINTER") else {
        panic!()
    };
    assert_eq!(field.ty(), Type::PtrMut(Box::new(Type::I32), 1));
    assert_eq!(field.constant().unwrap().value(), Value::I32(0));
    let Item::Const(field) = index.expect_item("Test", "NEGATIVE_ZERO") else {
        panic!()
    };
    let Value::F64(value) = field.constant().unwrap().value() else {
        panic!()
    };
    assert_eq!(value.to_bits(), (-0.0_f64).to_bits());
}

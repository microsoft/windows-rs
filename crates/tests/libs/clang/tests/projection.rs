use windows_metadata::{
    Type,
    reader::{Item, MethodDef, ParamDirection},
};

pub mod common;
use common::compile;

#[test]
fn completion_survives_representative_selection() {
    use ParamDirection::{Input, Output};
    let index = common::compile_permutations("contract_filtered_completion");
    let interface = Type::class_named("Test", "IFoo");
    let alias = Type::value_named("Test", "Alias2");
    let slot = Type::PtrMut(Box::new(alias.clone()), 1);
    let Item::Fn(function) = index.expect_item("Test", "Use") else {
        panic!();
    };
    assert_eq!(function.signature(&[]).return_type, alias);
    assert_eq!(
        function.signature(&[]).types,
        [
            alias.clone(),
            slot.clone(),
            alias.clone(),
            slot.clone(),
            Type::value_named("Test", "Slot"),
            Type::PtrMut(Box::new(Type::value_named("Test", "Packet")), 1),
        ]
    );
    directions(function, &[Input, Output, Input, Output, Input, Output]);
    let Item::Type(packet) = index.expect_item("Test", "Packet") else {
        panic!();
    };
    assert_eq!(
        packet.fields().map(|field| field.ty()).collect::<Vec<_>>(),
        [alias, Type::value_named("Test", "Slot"), interface.clone()]
    );
    let Item::Type(alias) = index.expect_item("Test", "Alias") else {
        panic!();
    };
    assert_eq!(alias.underlying_type(), Some(interface.clone()));
    let Item::Type(alias) = index.expect_item("Test", "Alias2") else {
        panic!();
    };
    assert_eq!(
        alias.underlying_type(),
        Some(Type::value_named("Test", "Alias"))
    );
    let Item::Type(alias) = index.expect_item("Test", "Slot") else {
        panic!();
    };
    assert_eq!(alias.underlying_type(), Some(slot));

    let index = common::compile_permutations("contract_type_completion");
    let Item::Type(packet) = index.expect_item("Test", "Packet") else {
        panic!();
    };
    assert_eq!(
        packet.fields().map(|field| field.ty()).collect::<Vec<_>>(),
        [interface.clone(), Type::PtrMut(Box::new(interface), 1)]
    );

    let index = common::compile_permutations("contract_constant_completion");
    assert!(index.get_item("Test", "POINTER_NULL").next().is_none());
    assert!(matches!(
        index.expect_item("Test", "INTEGER_CONSTANT"),
        Item::Const(_)
    ));
    assert!(matches!(index.expect_item("Test", "IFoo"), Item::Type(_)));
}

#[test]
fn external_interface_identity_preserves_native_alias_depth() {
    let index = common::compile_permutations("contract_reference_pointer");
    assert!(index.get_item("Test", "HANDLE_NULL").next().is_none());
    assert!(index.get_item("Test", "COPY_NULL").next().is_none());
    assert!(matches!(
        index.expect_item("Test", "INTEGER_CONSTANT"),
        Item::Const(_)
    ));
    let interface = Type::class_named("External", "Handle");
    let pointer = Type::PtrMut(Box::new(interface.clone()), 1);
    let Item::Fn(function) = index.expect_item("Test", "Use") else {
        panic!();
    };
    assert_eq!(function.signature(&[]).return_type, interface);
    assert_eq!(
        function.signature(&[]).types,
        [
            interface.clone(),
            interface,
            pointer.clone(),
            Type::value_named("Test", "Slot")
        ]
    );
    directions(
        function,
        &[
            ParamDirection::Input,
            ParamDirection::Input,
            ParamDirection::Output,
            ParamDirection::Input,
        ],
    );
    let Item::Type(slot) = index.expect_item("Test", "Slot") else {
        panic!();
    };
    assert_eq!(slot.underlying_type(), Some(pointer));
}

#[test]
fn shared_recursive_proofs_preserve_graph_edges() {
    let index = common::compile_permutations("contract_shared_cycle");
    for (name, targets) in [
        ("Leaf", ["IFoo", "Root"]),
        ("Node1", ["Leaf", "Leaf"]),
        ("Node2", ["Node1", "Leaf"]),
        ("Node3", ["Node2", "Node1"]),
        ("Node4", ["Node3", "Node2"]),
        ("Node5", ["Node4", "Node3"]),
        ("Root", ["Node5", "Node4"]),
    ] {
        let Item::Type(record) = index.expect_item("Test", name) else {
            panic!();
        };
        assert_eq!(
            record.fields().map(|field| field.ty()).collect::<Vec<_>>(),
            targets.map(|target| {
                if target == "IFoo" {
                    Type::class_named("Test", "IFoo")
                } else {
                    Type::PtrMut(Box::new(Type::value_named("Test", target)), 1)
                }
            }),
            "{name}"
        );
    }
}

fn directions(method: MethodDef<'_>, expected: &[ParamDirection]) {
    let signature = method.signature(&[]);
    assert_eq!(signature.types.len(), expected.len());
    let params = method.params_by_sequence(expected.len()).unwrap();
    let actual: Vec<_> = params
        .params()
        .iter()
        .map(|param| param.unwrap().direction())
        .collect();
    assert_eq!(actual, expected, "{}", method.name());
}

#[test]
fn equivalence_completes_interfaces_inside_recursive_records() {
    let index = compile("equivalence_completion");
    let alias = Type::value_named("Test", "Alias3");
    let slot = Type::PtrMut(Box::new(alias.clone()), 1);
    let Item::Type(inner) = index.expect_item("Test", "Inner") else {
        panic!();
    };
    for field in inner.fields() {
        let expected = match field.name() {
            "tag" => Type::class_named("Test", "IFoo"),
            "value" => alias.clone(),
            "slot" => slot.clone(),
            "array" => Type::ArrayFixed(Box::new(alias.clone()), 2),
            _ => panic!(),
        };
        assert_eq!(field.ty(), expected, "{}", field.name());
    }
    let Item::Type(callback) = index.expect_item("Test", "Callback") else {
        panic!();
    };
    let method = callback.methods().next().unwrap();
    let signature = method.signature(&[]);
    assert_eq!(signature.return_type, alias);
    assert_eq!(signature.types, [alias, slot]);
    directions(method, &[ParamDirection::Input, ParamDirection::Output]);
    let Item::Type(uses) = index.expect_item("Test", "Uses") else {
        panic!();
    };
    for field in uses.fields() {
        let expected = match field.name() {
            "next" => Type::PtrMut(Box::new(Type::value_named("Test", "Uses")), 1),
            "inner" => Type::value_named("Test", "Inner"),
            "tag" => Type::class_named("Test", "IFoo"),
            "callback" => Type::class_named("Test", "Callback"),
            "listener" => Type::class_named("Test", "IUse"),
            _ => panic!(),
        };
        assert_eq!(field.ty(), expected, "{}", field.name());
    }
}

#[test]
fn equivalence_preserves_mutually_recursive_record_edges() {
    let index = compile("equivalence_mutual_completion");
    for (name, target) in [("Left", "Right"), ("Right", "Left")] {
        let Item::Type(record) = index.expect_item("Test", name) else {
            panic!();
        };
        for field in record.fields() {
            let expected = match field.name() {
                "next" => Type::PtrMut(Box::new(Type::value_named("Test", target)), 1),
                "value" if name == "Right" => Type::I32,
                "value" | "tag" => Type::class_named("Test", "IFoo"),
                _ => panic!(),
            };
            assert_eq!(field.ty(), expected, "{name}: {}", field.name());
        }
    }
    let Item::Fn(function) = index.expect_item("Test", "Use") else {
        panic!();
    };
    assert_eq!(
        function.signature(&[]).types,
        [Type::PtrMut(Box::new(Type::value_named("Test", "Left")), 1)]
    );
}

#[test]
fn resolution_preserves_interface_alias_depth_and_direction() {
    use ParamDirection::{Input, Output};
    for (fixture, namespace) in [
        ("resolution_local_interfaces", "Test"),
        ("resolution_external_interfaces", "External"),
    ] {
        let index = compile(fixture);
        let interface = Type::class_named(namespace, "IFoo");
        let alias = Type::value_named("Test", "Alias3");
        let slot = Type::PtrMut(Box::new(alias.clone()), 1);
        for (name, target) in [
            ("Alias1", interface.clone()),
            ("Alias2", Type::value_named("Test", "Alias1")),
            ("Alias3", Type::value_named("Test", "Alias2")),
            ("PPFoo", slot.clone()),
        ] {
            let Item::Type(ty) = index.expect_item("Test", name) else {
                panic!();
            };
            assert_eq!(ty.underlying_type(), Some(target), "{fixture}: {name}");
        }
        for name in ["Forward", "Complete"] {
            let Item::Fn(function) = index.expect_item("Test", name) else {
                panic!();
            };
            let signature = function.signature(&[]);
            assert_eq!(signature.return_type, alias, "{fixture}: {name}");
            assert_eq!(
                signature.types,
                [alias.clone(), alias.clone(), slot.clone()],
                "{fixture}: {name}"
            );
            directions(function, &[Input, Input, Output]);
        }
        let Item::Type(ty) = index.expect_item("Test", "Uses") else {
            panic!();
        };
        for field in ty.fields() {
            let expected = match field.name() {
                "tag" => interface.clone(),
                "slot" => slot.clone(),
                "array" => Type::ArrayFixed(Box::new(alias.clone()), 2),
                "value" | "pointer" => alias.clone(),
                _ => panic!(),
            };
            assert_eq!(field.ty(), expected, "{fixture}: {}", field.name());
        }
        if namespace == "External" {
            let Item::Fn(function) = index.expect_item("Test", "Tags") else {
                panic!();
            };
            assert_eq!(
                function.signature(&[]).types,
                [
                    interface.clone(),
                    interface.clone(),
                    Type::PtrMut(Box::new(interface), 1),
                ]
            );
            directions(function, &[Input, Input, Output]);
        }
    }
}

#[test]
fn resolution_filters_interface_alias_constants() {
    let index = compile("resolution_interface_constants");
    assert!(index.get_item("Test", "UNKNOWN_NULL").next().is_none());
    assert!(index.get_item("Test", "UNKNOWN_ALIAS").next().is_none());
    assert!(matches!(
        index.expect_item("Test", "INTEGER_CONSTANT"),
        Item::Const(_)
    ));
}

#[test]
fn resolution_accepts_externally_emitted_dependencies() {
    let index = compile("resolution_referenced_dependency");
    let Item::Fn(function) = index.expect_item("Test", "Referenced") else {
        panic!();
    };
    assert_eq!(
        function.signature(&[]).types,
        [Type::class_named("External", "IFoo")]
    );
    directions(function, &[ParamDirection::Input]);
}

#[test]
fn resolution_ignores_unselected_aliases_in_other_translation_units() {
    let index = compile("resolution_interface_scope");
    let Item::Fn(function) = index.expect_item("Test", "RecordUse") else {
        panic!();
    };
    let record = Type::value_named("Test", "Shared");
    assert_eq!(
        function.signature(&[]).types,
        [record.clone(), Type::PtrMut(Box::new(record), 1)]
    );
    directions(function, &[ParamDirection::Input, ParamDirection::Output]);
}

#[test]
fn completed_interfaces_have_one_pointer_representation() {
    use ParamDirection::{Input, Output};
    let index = compile("cross_tu_interface");
    let interface = Type::class_named("Test", "IFoo");
    for name in ["FromForward", "FromComplete"] {
        let Item::Fn(function) = index.expect_item("Test", name) else {
            panic!();
        };
        let signature = function.signature(&[]);
        assert_eq!(signature.return_type, interface, "{name}");
        assert_eq!(
            signature.types,
            [
                interface.clone(),
                Type::PtrMut(Box::new(interface.clone()), 1),
            ],
            "{name}"
        );
        directions(function, &[Input, Output]);
    }
    for name in ["FooCallback", "IUseForward"] {
        let Item::Type(ty) = index.expect_item("Test", name) else {
            panic!();
        };
        let method = ty.methods().next().unwrap();
        let signature = method.signature(&[]);
        assert_eq!(signature.return_type, interface, "{name}");
        assert_eq!(signature.types, std::slice::from_ref(&interface), "{name}");
        directions(method, &[Input]);
    }
    for name in ["UsesFoo", "UsesComplete"] {
        let Item::Type(ty) = index.expect_item("Test", name) else {
            panic!();
        };
        for field in ty.fields() {
            let expected = match field.name() {
                "slot" => Type::PtrMut(Box::new(interface.clone()), 1),
                "array" => Type::ArrayFixed(Box::new(interface.clone()), 2),
                _ => interface.clone(),
            };
            assert_eq!(field.ty(), expected, "{name}: {}", field.name());
        }
    }
}

#[test]
fn property_keys_use_resolved_external_types() {
    let index = compile("property_key_reference");
    for (name, ty) in [("PKEY_Test", "PROPERTYKEY"), ("DEVPKEY_Test", "DEVPROPKEY")] {
        let Item::Const(field) = index.expect_item("Test", name) else {
            panic!();
        };
        assert_eq!(field.ty(), Type::value_named("External", ty));
    }
}

#[test]
fn string_parameter_directions_follow_emitted_types() {
    use ParamDirection::{Input, InputOutput, Output};
    for (fixture, expected) in [
        (
            "alias_raw_strings",
            vec![Input, Input, Output, InputOutput, Input, Input, Output],
        ),
        (
            "alias_raw_strings_declared",
            vec![Input, Input, Output, Input, Input],
        ),
    ] {
        let index = compile(fixture);
        let Item::Fn(function) = index.expect_item("Test", "Strings") else {
            panic!();
        };
        directions(function, &expected);
        for name in ["CALLBACK", "IStrings"] {
            let Item::Type(ty) = index.expect_item("Test", name) else {
                panic!();
            };
            directions(ty.methods().next().unwrap(), &[Input, Output]);
        }
    }
    let index = compile("alias_named_directions");
    let Item::Fn(function) = index.expect_item("Test", "Strings") else {
        panic!();
    };
    directions(function, &[Input, Output, Input, Output]);
}

#[test]
fn canonical_alias_chains_terminate_in_pointer_types() {
    for (fixture, expected) in [
        (
            "alias_chains",
            vec![
                ("PSTR", Type::PtrMut(Box::new(Type::I8), 1)),
                ("PCWSTR", Type::PtrConst(Box::new(Type::U16), 1)),
                ("TEXT", Type::value_named("Test", "PSTR")),
                ("WTEXT", Type::value_named("Test", "PCWSTR")),
            ],
        ),
        (
            "alias_reverse_chain",
            vec![
                ("LPSTR", Type::PtrMut(Box::new(Type::I8), 1)),
                ("LPCWSTR", Type::PtrConst(Box::new(Type::U16), 1)),
                ("PSTR", Type::value_named("Test", "LPSTR")),
                ("PCWSTR", Type::value_named("Test", "LPCWSTR")),
                ("TEXT", Type::value_named("Test", "PSTR")),
            ],
        ),
    ] {
        let index = compile(fixture);
        for (name, expected) in expected {
            let Item::Type(ty) = index.expect_item("Test", name) else {
                panic!();
            };
            assert_eq!(ty.underlying_type(), Some(expected), "{fixture}: {name}");
        }
    }
}

#[test]
fn void_pointer_boundaries_preserve_constness_and_depth() {
    let index = compile("void_pointer_boundary");
    let Item::Type(ty) = index.expect_item("Test", "PVOID") else {
        panic!();
    };
    assert_eq!(
        ty.underlying_type(),
        Some(Type::PtrMut(Box::new(Type::Void), 1))
    );
    let Item::Fn(function) = index.expect_item("Test", "Compare") else {
        panic!();
    };
    assert_eq!(
        function.signature(&[]).types,
        [
            Type::PtrConst(Box::new(Type::value_named("Test", "PVOID")), 1),
            Type::PtrMut(Box::new(Type::Void), 2),
        ]
    );
}

#[test]
fn excluded_aliases_keep_resolved_types_and_directions() {
    use ParamDirection::{Input, Output};
    for (fixture, namespace, name) in [
        ("alias_excluded_missing", "Test", "LPSTR"),
        ("alias_excluded_local", "Test", "PSTR"),
        ("alias_excluded_referenced", "Canonical", "PSTR"),
    ] {
        let index = compile(fixture);
        let Item::Type(example) = index.expect_item("Test", "Example") else {
            panic!();
        };
        let field = example
            .fields()
            .find(|field| field.name() == "text")
            .unwrap();
        assert_eq!(
            field.ty(),
            Type::PtrMut(Box::new(Type::value_named(namespace, name)), 1),
            "{fixture}"
        );
        if namespace == "Test" {
            let Item::Type(alias) = index.expect_item(namespace, name) else {
                panic!();
            };
            assert_eq!(
                alias.underlying_type(),
                Some(Type::PtrMut(Box::new(Type::I8), 1)),
                "{fixture}"
            );
        }
        let Item::Fn(function) = index.expect_item("Test", "Text") else {
            panic!();
        };
        directions(function, &[Input, Output]);
        if fixture == "alias_excluded_local" {
            for (name, expected) in [
                ("PCSTR", Type::value_named("Test", "LPCSTR")),
                ("LPCSTR", Type::PtrConst(Box::new(Type::I8), 1)),
            ] {
                let Item::Type(alias) = index.expect_item("Test", name) else {
                    panic!();
                };
                assert_eq!(alias.underlying_type(), Some(expected));
            }
        }
    }
}

#[test]
fn excluded_void_aliases_keep_const_boundaries_and_external_identity() {
    for (fixture, expected) in [
        (
            "alias_excluded_void",
            vec![
                ("direct", Type::PtrMut(Box::new(Type::Void), 1)),
                ("output", Type::PtrMut(Box::new(Type::Void), 2)),
                (
                    "input",
                    Type::PtrConst(Box::new(Type::value_named("Test", "PVOID")), 1),
                ),
                (
                    "nested",
                    Type::PtrConst(Box::new(Type::value_named("Test", "PVOID")), 2),
                ),
                ("scalar", Type::U64),
                ("custom", Type::value_named("External", "KnownCustom")),
                ("ambiguous", Type::value_named("Test", "AMBIGUOUS")),
                ("record", Type::value_named("External", "KnownRecord")),
            ],
        ),
        (
            "alias_excluded_direct",
            vec![
                (
                    "text",
                    Type::PtrMut(Box::new(Type::value_named("Direct", "LPSTR")), 1),
                ),
                ("direct", Type::value_named("Direct", "PVOID")),
                (
                    "output",
                    Type::PtrMut(Box::new(Type::value_named("Direct", "PVOID")), 1),
                ),
                (
                    "input",
                    Type::PtrConst(Box::new(Type::value_named("Direct", "PVOID")), 1),
                ),
            ],
        ),
    ] {
        let index = compile(fixture);
        let Item::Type(example) = index.expect_item("Test", "Example") else {
            panic!();
        };
        for (name, expected) in expected {
            let field = example.fields().find(|field| field.name() == name).unwrap();
            assert_eq!(field.ty(), expected, "{fixture}: {name}");
        }
        if fixture == "alias_excluded_void" {
            for name in ["PVOID", "LPVOID", "AMBIGUOUS"] {
                let Item::Type(alias) = index.expect_item("Test", name) else {
                    panic!();
                };
                assert_eq!(
                    alias.underlying_type(),
                    Some(Type::PtrMut(Box::new(Type::Void), 1)),
                    "{name}"
                );
            }
        }
    }
}

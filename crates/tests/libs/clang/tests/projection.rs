use windows_metadata::{
    Type,
    reader::{Item, MethodDef, ParamDirection},
};

pub mod common;
use common::compile;

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

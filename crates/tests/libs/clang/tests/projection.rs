use windows_metadata::{
    Type,
    reader::{Index, Item, MethodDef, ParamDirection},
};

fn compile(name: &str) -> Index {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let scratch = std::path::Path::new(env!("OUT_DIR")).join(format!("projection-{name}"));
    std::fs::create_dir_all(&scratch).unwrap();
    let rdl = scratch.join("test.rdl");
    let winmd = scratch.join("test.winmd");
    let input = root.join("input").join(format!("{name}.h"));
    let source = std::fs::read_to_string(&input).unwrap();
    let mut clang = windows_clang::clang();
    clang
        .input(&input)
        .args(["-x", "c++"])
        .namespace("Test")
        .library("test.dll")
        .output(&rdl);
    let mut reader = windows_rdl::reader();
    reader.input(&rdl).output(&winmd);
    for reference in source
        .lines()
        .filter_map(|line| line.strip_prefix("//! reference "))
    {
        let output = scratch
            .join(format!("reference-{}", reference.trim()))
            .with_extension("winmd");
        windows_rdl::reader()
            .input(root.join("input").join(reference.trim()))
            .output(&output)
            .write()
            .unwrap();
        clang.reference(&output);
        reader.reference(&output);
    }
    clang.write().unwrap();
    reader.write().unwrap();
    Index::read(&winmd).unwrap()
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

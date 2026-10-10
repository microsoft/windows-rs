use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use windows_clang2::{Input, ProjectionOptions, capture, discover, discover_in_scope};
use windows_metadata::{
    Value,
    reader::{Index, Item},
};

const ARGS: &[&str] = &["-x", "c++", "--target=x86_64-pc-windows-msvc"];

#[test]
fn initializer_classification_distinguishes_fragments_from_typed_values_and_strings() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR")).join("input\\macro_initializers.h");
    let inventory = discover(
        [Input::new(
            "initializers.hpp",
            format!("#include \"{}\"", file.display()),
        )],
        ARGS,
        &[file.to_str().unwrap()],
    )
    .unwrap();
    assert_eq!(
        inventory
            .iter()
            .filter(|item| item.macro_initializer)
            .map(|item| item.name.as_str())
            .collect::<BTreeSet<_>>(),
        BTreeSet::from(["Alias", "EmptyInitializer", "Initializer"])
    );
    assert_eq!(
        inventory
            .iter()
            .filter(|item| item.macro_attribute)
            .map(|item| item.name.as_str())
            .collect::<BTreeSet<_>>(),
        BTreeSet::from(["Attribute", "MacroAttribute"]),
    );
    assert!(
        !inventory
            .iter()
            .find(|item| item.name == "ScalarCall")
            .unwrap()
            .macro_attribute
    );
}

#[test]
fn directory_scope_uses_included_files_and_honors_explicit_exclusions() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("input\\discovery");
    let excluded = directory.join("macro_aliases.h");
    let source = format!(
        "#include \"{}\"\n#include \"{}\"\n#include \"{}\"\n",
        directory.join("record_members.h").display(),
        excluded.display(),
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("input\\translation_units\\shared.h")
            .display(),
    );
    let inventory = discover_in_scope(
        [Input::new("directory-scope.hpp", &source)],
        ARGS,
        &[excluded.to_str().unwrap()],
        &[directory.to_str().unwrap()],
        &[excluded.to_str().unwrap()],
    )
    .unwrap();
    assert_eq!(
        inventory
            .iter()
            .map(|item| item.name.as_str())
            .collect::<BTreeSet<_>>(),
        BTreeSet::from(["Outer", "Outer::Inner", "Outer::Mode"]),
    );
    let error = discover_in_scope(
        [Input::new("missing-header.hpp", "")],
        ARGS,
        &[excluded.to_str().unwrap()],
        &[directory.to_str().unwrap()],
        &[],
    )
    .unwrap_err();
    assert!(
        error
            .to_string()
            .contains("discovery header was not included"),
        "{error}"
    );
}

#[test]
fn record_members_remain_in_the_owning_native_closure() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR")).join("input/discovery/record_members.h");
    let input = Input::new(
        file.to_str().unwrap(),
        include_str!("../input/discovery/record_members.h"),
    );
    let inventory = discover([input.clone()], ARGS, &[&input.name]).unwrap();
    assert_eq!(
        inventory
            .iter()
            .map(|declaration| (declaration.name.as_str(), declaration.record_member))
            .collect::<BTreeMap<_, _>>(),
        BTreeMap::from([
            ("Outer", false),
            ("Outer::Inner", true),
            ("Outer::Mode", true)
        ]),
    );
    let snapshot = capture([input], ARGS, &["Outer"]).unwrap();
    let resolved = snapshot.resolve().unwrap();
    assert_eq!(resolved.report().declarations, 3);
    assert!(resolved.report().incomplete.is_empty());
}

#[test]
fn macro_alias_discovery_distinguishes_declarations_from_values() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR")).join("input/discovery/macro_aliases.h");
    let input = Input::new(
        file.to_str().unwrap(),
        include_str!("../input/discovery/macro_aliases.h"),
    );
    let inventory = discover([input.clone()], ARGS, &[&input.name]).unwrap();
    let aliases: BTreeMap<_, _> = inventory
        .iter()
        .filter_map(|declaration| {
            declaration
                .macro_alias
                .as_ref()
                .map(|target| (declaration.name.as_str(), target.as_str()))
        })
        .collect();
    assert_eq!(
        aliases,
        BTreeMap::from([
            ("Function", "FunctionW"),
            ("FunctionChain", "FunctionW"),
            ("TypeAlias", "Record"),
            ("PointerAlias", "Pointer"),
            ("CallbackAlias", "Callback"),
            ("EnumAlias", "Mode"),
            ("TemporaryObject", "FunctionW"),
        ])
    );
    let find = |name: &str| inventory.iter().find(|item| item.name == name).unwrap();
    for name in [
        "DeclarationAttribute",
        "AttributeAlias",
        "AttributeChain",
        "ImportAttribute",
        "BracketAttribute",
    ] {
        assert!(find(name).macro_attribute, "{name}");
    }
    for name in [
        "AttributeText",
        "ParenthesizedValue",
        "CommentSeparated",
        "SpacedContinuation",
        "ChangedAttribute",
    ] {
        assert!(!find(name).macro_attribute, "{name}");
        assert!(!find(name).function_macro, "{name}");
    }
    for name in ["TemporaryFunction", "TemporarySpliced"] {
        assert!(find(name).function_macro, "{name}");
    }
    let snapshot = capture(
        [input],
        ARGS,
        &[
            "EnumValue",
            "ConstantAlias",
            "LiteralValue",
            "Expression",
            "StringValue",
            "Changed",
            "ValueChain",
            "AttributeText",
            "ParenthesizedValue",
            "CommentSeparated",
            "SpacedContinuation",
            "ChangedAttribute",
        ],
    )
    .unwrap();
    let plan = snapshot
        .resolve()
        .unwrap()
        .project(&ProjectionOptions::new("Test"))
        .unwrap();
    let expected =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("expected/discovery/macro_aliases.rdl");
    if std::env::var_os("UPDATE_EXPECT").is_some() {
        std::fs::write(&expected, plan.rdl()).unwrap();
    }
    assert_eq!(
        plan.rdl(),
        std::fs::read_to_string(expected)
            .unwrap()
            .replace("\r\n", "\n")
    );
}

fn inputs() -> Vec<Input> {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("input/discovery");
    [
        ("main.h", include_str!("../input/discovery/main.h")),
        (
            "satellite.h",
            include_str!("../input/discovery/satellite.h"),
        ),
    ]
    .into_iter()
    .map(|(name, source)| Input::new(directory.join(name).to_str().unwrap(), source))
    .collect()
}

#[test]
fn declaration_fragments_do_not_hide_macro_values_or_calls() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR")).join("input\\discovery\\declaration_macros.h");
    let input = Input::new(
        file.to_str().unwrap(),
        include_str!("../input/discovery/declaration_macros.h"),
    );
    let inventory = discover([input.clone()], ARGS, &[&input.name]).unwrap();
    let helpers: Vec<_> = inventory
        .iter()
        .filter(|declaration| declaration.macro_declaration)
        .map(|declaration| declaration.name.as_str())
        .collect();
    assert_eq!(
        helpers,
        [
            "Linkage",
            "LinkageAlias",
            "PublicApi",
            "PublicApiAlias",
            "BeginDeclarations",
            "EndDeclarations",
            "Convention",
            "ConventionAlias",
            "Noexcept",
            "NoexceptAlias",
            "BeginTry",
            "BeginTryAlias",
            "BeginConditional",
            "BeginConditionalAlias",
            "ReturnStatement",
        ]
    );
    for name in [
        "NoexceptValue",
        "KeywordText",
        "Value",
        "ValueAlias",
        "ValueExpression",
        "CallExpression",
        "CycleA",
        "CycleB",
        "CyclePrefix",
    ] {
        let declaration = inventory
            .iter()
            .find(|declaration| declaration.name == name)
            .unwrap();
        assert!(!declaration.macro_declaration, "{name}");
        assert!(!declaration.macro_attribute, "{name}");
        assert!(declaration.macro_alias.is_none(), "{name}");
    }
    let snapshot = capture(
        [input],
        ARGS,
        &[
            "NoexceptValue",
            "KeywordText",
            "ValueExpression",
            "Native",
            "Imported",
        ],
    )
    .unwrap();
    let mut options = ProjectionOptions::new("Test");
    options.library = Some("test.dll".into());
    let plan = snapshot.resolve().unwrap().project(&options).unwrap();
    let expected =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("expected\\discovery\\declaration_macros.rdl");
    if std::env::var_os("UPDATE_EXPECT").is_some() {
        std::fs::write(&expected, plan.rdl()).unwrap();
    }
    assert_eq!(
        plan.rdl(),
        std::fs::read_to_string(expected)
            .unwrap()
            .replace("\r\n", "\n")
    );
}

#[test]
fn exact_header_identity_and_macro_ownership() {
    let inputs = inputs();
    let header = &inputs[0].name;
    let declarations = discover(inputs.clone(), ARGS, &[header]).unwrap();
    let names: BTreeSet<_> = declarations.iter().map(|item| item.name.as_str()).collect();
    assert_eq!(
        names,
        BTreeSet::from([
            "CURRENT_MODE",
            "CURRENT_SIGNED",
            "Consumer",
            "HEADER_HELPER",
            "HEADER_MARKER",
            "HEADER_VALUE",
            "InlineHelper",
            "Missing",
            "NoInitializer",
            "Owned",
            "PublicShared",
        ])
    );
    assert!(
        declarations
            .iter()
            .all(|item| Path::new(&item.header) == Path::new(header))
    );
    for name in ["Shared", "Outside", "Completed", "Second"] {
        assert!(!names.contains(name));
    }
    let find = |name: &str| declarations.iter().find(|item| item.name == name).unwrap();
    assert!(find("HEADER_HELPER").function_macro);
    assert!(find("HEADER_MARKER").empty_macro);
    assert!(find("InlineHelper").inline);
    assert!(find("Owned").definition);
    assert!(!find("Missing").definition);
    assert!(!find("NoInitializer").initializer);
    assert_eq!(
        declarations,
        discover(inputs.iter().cloned().rev(), ARGS, &[header, header],).unwrap()
    );
    let error = discover(inputs.clone(), ARGS, &["not-included.h"]).unwrap_err();
    assert!(error.to_string().contains("was not included"));
    assert!(discover(inputs, ARGS, &[]).is_err());
}

#[test]
fn cached_macro_and_record_owners_share_the_same_physical_header() {
    let mut input = inputs().remove(0);
    input.source.push_str("\n#define PartitionBad {\n");
    let report = windows_clang2::capture_report(
        [input.clone()],
        ARGS,
        &["Owned", "HEADER_VALUE", "PartitionBad"],
    )
    .unwrap();
    assert_eq!(report.rejected.len(), 1);
    let snapshot = report.snapshot.unwrap();
    let resolved = snapshot.resolve().unwrap();
    let options = ProjectionOptions::new("Test");
    let plan = resolved.project(&options).unwrap();
    let partitions = plan.rdl_by_header().unwrap();
    let header = std::fs::canonicalize(&input.name).unwrap();
    let text = &partitions[header.to_str().unwrap()];
    assert!(text.contains("struct Owned"), "{text}");
    assert!(text.contains("const HEADER_VALUE"), "{text}");
    let fresh = capture([input], ARGS, &["Owned", "HEADER_VALUE"]).unwrap();
    assert_eq!(
        partitions,
        fresh
            .resolve()
            .unwrap()
            .project(&options)
            .unwrap()
            .rdl_by_header()
            .unwrap()
    );
}

#[test]
fn selected_projection_is_closed_and_source_partitioned() {
    let inputs = inputs();
    let headers: Vec<_> = inputs.iter().map(|input| input.name.as_str()).collect();
    let declarations = discover(inputs.clone(), ARGS, &headers).unwrap();
    let roots: Vec<_> = declarations
        .iter()
        .filter(|item| !item.function_macro && !item.empty_macro && !item.inline)
        .map(|item| item.name.as_str())
        .collect();
    let snapshot = capture(inputs.clone(), ARGS, &roots).unwrap();
    let resolved = snapshot.resolve().unwrap();
    let options = ProjectionOptions::new("Test");
    assert!(resolved.project(&options).is_err());
    for name in ["NoInitializer", "not_captured"] {
        assert!(resolved.project_roots(&options, &[name]).is_err());
    }
    assert!(resolved.project_roots(&options, &[]).is_err());
    let selected = [
        "Missing",
        "Owned",
        "Consumer",
        "Second",
        "HEADER_VALUE",
        "CURRENT_MODE",
        "CURRENT_SIGNED",
        "PublicShared",
    ];
    let plan = resolved.project_roots(&options, &selected).unwrap();
    assert!(plan.omitted().is_empty());
    let partitions: BTreeMap<_, _> = plan
        .rdl_by_header()
        .unwrap()
        .into_iter()
        .map(|(header, text)| {
            (
                Path::new(&header)
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
                text,
            )
        })
        .collect();
    assert_eq!(
        partitions.keys().map(String::as_str).collect::<Vec<_>>(),
        ["main", "satellite", "shared"]
    );
    let expected = Path::new(env!("CARGO_MANIFEST_DIR")).join("expected/discovery");
    for (header, text) in &partitions {
        let file = expected.join(format!("{header}.rdl"));
        if std::env::var_os("UPDATE_EXPECT").is_some() {
            std::fs::create_dir_all(&expected).unwrap();
            std::fs::write(&file, text).unwrap();
        }
        assert_eq!(
            *text,
            std::fs::read_to_string(file).unwrap().replace("\r\n", "\n")
        );
    }
    let output = Path::new(env!("OUT_DIR")).join("discovery.winmd");
    windows_rdl::reader()
        .input_texts(partitions.values())
        .reference_default()
        .output(&output)
        .write()
        .unwrap();
    let index = Index::read(output).unwrap();
    for (name, value) in [
        ("HEADER_VALUE", Value::I32(7)),
        ("CURRENT_MODE", Value::U16(3)),
        ("CURRENT_SIGNED", Value::I32(-2)),
    ] {
        let Item::Const(field) = index.expect_item("Test", name) else {
            panic!()
        };
        assert_eq!(field.constant().unwrap().value(), value);
    }
    let snapshot = capture(inputs.into_iter().rev(), ARGS, &roots).unwrap();
    let reordered = snapshot
        .resolve()
        .unwrap()
        .project_roots(&options, &selected.into_iter().rev().collect::<Vec<_>>())
        .unwrap();
    assert_eq!(plan.rdl(), reordered.rdl());
    assert_eq!(
        plan.rdl_by_header().unwrap(),
        reordered.rdl_by_header().unwrap()
    );
}

#[test]
fn discovered_dependencies_still_require_native_agreement() {
    let mut inputs = inputs();
    inputs[1].source.insert_str(0, "#define CONFLICT\n");
    let snapshot = capture(inputs, ARGS, &["Consumer", "Second"]).unwrap();
    let Err(error) = snapshot.resolve() else {
        panic!("conflicting header definitions passed resolution");
    };
    assert!(error.to_string().contains("Shared"), "{error}");
}

#[test]
fn header_partitioning_rejects_unowned_command_line_macros() {
    let snapshot = capture(
        [Input::new("empty.hpp", "")],
        &[
            "-x",
            "c++",
            "--target=x86_64-pc-windows-msvc",
            "-DCOMMAND_VALUE=4",
        ],
        &["COMMAND_VALUE"],
    )
    .unwrap();
    let plan = snapshot
        .resolve()
        .unwrap()
        .project(&ProjectionOptions::new("Test"))
        .unwrap();
    assert!(plan.rdl().contains("const COMMAND_VALUE: i32 = 4;"));
    assert!(
        plan.rdl_by_header()
            .unwrap_err()
            .to_string()
            .contains("ownership is unavailable")
    );
}
#[test]
fn aggregate_macros_do_not_receive_integer_probes() {
    let source = include_str!("../input/macro_aggregate.h");
    let mut outputs = vec![];
    for prefix in ["", "#define AS_MACRO\n"] {
        let snapshot = capture(
            [Input::new("macro.h", format!("{prefix}{source}"))],
            &["-x", "c++", "--target=x86_64-pc-windows-msvc"],
            &["ID", "HANDLE_VALUE", "SCALAR_VALUE"],
        )
        .unwrap();
        outputs.push(
            snapshot
                .resolve()
                .unwrap()
                .project(&ProjectionOptions::new("Test"))
                .unwrap()
                .rdl(),
        );
    }

    assert_eq!(outputs[0], outputs[1]);
    assert!(outputs[0].contains("#[guid(0x00000001000200030405060708090a0b)]"));
}

#[test]
fn reusable_projection_keeps_root_specific_names_and_failures_isolated() {
    let snapshot = capture(
        [Input::new(
            "aliases.h",
            "struct Tag { int value; }; typedef Tag A; typedef Tag B; class Incomplete;",
        )],
        &["-x", "c++", "--target=x86_64-pc-windows-msvc"],
        &["Tag", "A", "B", "Incomplete"],
    )
    .unwrap();
    let resolved = snapshot.resolve().unwrap();
    let options = ProjectionOptions::new("Test");
    let projection = resolved.projection(&options).unwrap();
    for roots in [
        &["A"][..],
        &["Incomplete"],
        &["B"],
        &["A", "B"],
        &["Tag", "A", "B"],
        &["A"],
    ] {
        let shared = projection.project_roots(roots);
        let isolated = resolved.project_roots(&options, roots);
        match (shared, isolated) {
            (Ok(shared), Ok(isolated)) => {
                assert_eq!(shared.rdl(), isolated.rdl());
                assert_eq!(
                    shared.rdl_by_header().unwrap(),
                    isolated.rdl_by_header().unwrap()
                );
            }
            (Err(shared), Err(isolated)) => {
                assert_eq!(shared.to_string(), isolated.to_string());
            }
            _ => panic!("projection policy leaked state between root selections"),
        }
    }
    assert!(projection.project_roots(&["A", "B"]).is_ok());
    assert!(projection.project_roots(&["missing"]).is_err());
    assert!(projection.project_roots(&[]).is_err());
}

use test_clang::selected_fixture;
use windows_clang::*;
use windows_metadata::{HasAttributes, Type, Value, reader};

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

#[test]
fn callbacks() {
    helpers::ensure_libclang();
    roundtrip();
    conflicts();
    rejections();
    qualifiers();
    cross_tu_names();
}

fn qualifiers() {
    let mut first = selected_fixture("callback_qualifiers", &["CallbackDependent"]);
    first.source = "#include \"callback_qualifiers.h\"".into();
    let output = projection::generate(std::slice::from_ref(&first), "Fixture").unwrap();
    assert_eq!((output.records, output.aliases), (2, 2));
    let mut second = selected_fixture("callback_qualifiers", &["CallbackDependent"]);
    second.name = "qualified.h".into();
    second.source = "#define QUALIFIED\n#include \"callback_qualifiers.h\"".into();
    let mut inputs = [first, second];
    let error = projection::generate(&inputs, "Fixture")
        .unwrap_err()
        .to_string();
    assert!(
        error.contains("unsupported type qualifiers: 0x8"),
        "{error}"
    );
    inputs[1].roots.clear();
    let partial = projection::generate_partial(&inputs, "Fixture").unwrap();
    assert_eq!((partial.output.records, partial.output.aliases), (1, 1));
    for name in [
        "Unaligned",
        "UnalignedInt",
        "InheritedUnaligned",
        "UnalignedRecord",
        "Callback",
        "CallbackUser",
        "CallbackDependent",
    ] {
        assert!(
            partial
                .omissions
                .iter()
                .any(|message| message.contains(name)),
            "{name}: {:?}",
            partial.omissions
        );
    }
}

fn cross_tu_names() {
    let mut provider = selected_fixture("callback_lookup", &[]);
    provider.name = "m.h".into();
    provider.source = "#define CALLBACK_NAMES\n#include \"callback_lookup.h\"".into();
    let mut consumer = selected_fixture("callback_lookup", &["CallbackUses"]);
    consumer.source = "#include \"callback_lookup.h\"".into();
    consumer.name = "a.h".into();
    let mut inputs = [consumer, provider];
    let first = projection::generate(&inputs, "Fixture").unwrap();
    assert_eq!((first.records, first.aliases), (3, 8));
    let rdl = &first.partitions["callback_lookup"].rdl;
    for field in [
        "scalar: BInt",
        "record: DRecord",
        "other: COther",
        "nested: ENested",
        "pair: CIntPair",
    ] {
        assert!(rdl.contains(field), "{rdl}");
    }
    assert!(rdl.contains("fn ENested(param0: BInt)"), "{rdl}");
    inputs.reverse();
    assert_eq!(first, projection::generate(&inputs, "Fixture").unwrap());
    inputs[1].name = "z.h".into();
    assert_eq!(first, projection::generate(&inputs, "Fixture").unwrap());
    let partial = projection::generate_partial(&inputs, "Fixture").unwrap();
    assert!(partial.omissions.is_empty(), "{:?}", partial.omissions);
    assert_eq!(first, partial.output);
    let inline = projection::generate(&inputs[1..], "Fixture").unwrap();
    assert_eq!((inline.records, inline.aliases), (3, 0));
    assert_eq!(
        inline.partitions["callback_lookup"].rdl,
        std::fs::read_to_string(test_clang::input_dir().join("callback_lookup_inline.rdl"))
            .unwrap()
            .replace("\r\n", "\n")
    );
}

fn roundtrip() {
    let mut inputs = [
        selected_fixture("callbacks", &[]),
        selected_fixture("callbacks", &[]),
    ];
    inputs[1].name = "second.h".into();
    let snapshots = capture(&inputs, &TARGETS).unwrap();
    let output = projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap();
    assert_eq!((output.records, output.enums, output.aliases), (2, 0, 7));
    inputs.reverse();
    assert_eq!(output, projection::generate(&inputs, "Fixture").unwrap());
    assert_eq!(
        output,
        projection::generate_partial(&inputs, "Fixture")
            .unwrap()
            .output
    );
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let winmd = root.join("target\\test-clang-callbacks.winmd");
    let rdl = &output.partitions["callback_types"].rdl;
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_text(rdl)
        .output(&winmd)
        .write()
        .unwrap();
    for bit in [1, 2, 4] {
        let index =
            reader::Index::new_for_architecture(vec![reader::File::read(&winmd).unwrap()], bit);
        for (name, expected) in [
            ("Handler", 2),
            ("Dispatch", 2),
            ("Cdecl", 1),
            ("Stdcall", 1),
            ("Factory", 0),
        ] {
            let definition = index.expect("Fixture", name);
            let signature = definition
                .methods()
                .find(|method| method.name() == "Invoke")
                .unwrap()
                .signature(&[]);
            assert_eq!(signature.types.len(), expected);
            if name == "Handler" {
                assert_eq!(signature.return_type, Type::I32);
                assert_eq!(
                    signature.types,
                    [
                        Type::value_named("Fixture", "Count"),
                        Type::PtrMut(Box::new(Type::value_named("Fixture", "Item")), 1)
                    ]
                );
            } else if name == "Dispatch" {
                assert_eq!(signature.types[0], Type::class_named("Fixture", "Handler"));
            } else if name == "Factory" {
                assert_eq!(
                    signature.return_type,
                    Type::class_named("Fixture", "Handler")
                );
            }
            let attribute = definition
                .find_attribute("UnmanagedFunctionPointerAttribute")
                .unwrap();
            let value = attribute.value().into_iter().next().unwrap().1;
            let Value::EnumValue(_, value) = value else {
                panic!()
            };
            assert_eq!(*value, Value::I32(if name == "Stdcall" { 1 } else { 2 }));
        }
        assert_eq!(
            index
                .expect("Fixture", "Callbacks")
                .fields()
                .map(|field| field.ty())
                .collect::<Vec<_>>(),
            [
                Type::class_named("Fixture", "Handler"),
                Type::class_named("Fixture", "Dispatch"),
                Type::ArrayFixed(Box::new(Type::class_named("Fixture", "Handler")), 2)
            ]
        );
        assert_eq!(
            index.expect("Fixture", "HandlerAddress").underlying_type(),
            Some(Type::PtrMut(
                Box::new(Type::class_named("Fixture", "Handler")),
                1
            ))
        );
        let snapshot = &snapshots
            .iter()
            .find(|snapshot| match bit {
                1 => snapshot.target == Target::X86,
                2 => snapshot.target == Target::X64,
                _ => snapshot.target == Target::Arm64,
            })
            .unwrap();
        let record = snapshot
            .records
            .iter()
            .find(|record| record.name == "Callbacks")
            .unwrap();
        let definition = snapshot.observations[record.observations[0].0]
            .definition
            .as_ref()
            .unwrap();
        let pointer_size = if bit == 1 { 4 } else { 8 };
        assert_eq!(
            definition.layout,
            Layout {
                size: pointer_size * 4,
                alignment: pointer_size
            }
        );
        assert_eq!(
            definition
                .fields
                .iter()
                .map(|field| field.offset_bits)
                .collect::<Vec<_>>(),
            [0, pointer_size * 8, pointer_size * 16]
        );
    }
    let roundtrip = root.join("target\\test-clang-callbacks.rdl");
    windows_rdl::writer()
        .input(&winmd)
        .filter("Fixture")
        .output(&roundtrip)
        .write()
        .unwrap();
    assert_eq!(*rdl, std::fs::read_to_string(&roundtrip).unwrap());
    std::fs::remove_file(winmd).unwrap();
    std::fs::remove_file(roundtrip).unwrap();

    let output =
        projection::generate(&[selected_fixture("callbacks", &["Callbacks"])], "Fixture").unwrap();
    assert_eq!((output.records, output.aliases), (2, 3));
}

fn conflicts() {
    for (original, replacement) in [
        ("typedef int (*Handler)", "typedef unsigned (*Handler)"),
        ("typedef int (*Handler)", "typedef long (*Handler)"),
        ("typedef int (*Handler)", "typedef int (__stdcall *Handler)"),
        ("Count count", "int count"),
        ("Item* item", "const Item* item"),
    ] {
        let first = selected_fixture("callbacks", &["Handler"]);
        let mut second = selected_fixture("callbacks", &["Handler"]);
        second.name = "different.h".into();
        second.source = std::fs::read_to_string(test_clang::input_dir().join("callback_types.h"))
            .unwrap()
            .replace(original, replacement);
        let error = projection::generate(&[first, second], "Fixture")
            .unwrap_err()
            .to_string();
        assert!(error.contains("incompatible native bindings"), "{error}");
    }
}

fn rejections() {
    let input = selected_fixture("callback_rejections", &[]);
    let result = projection::generate_partial(&[input], "Fixture").unwrap();
    assert_eq!((result.output.records, result.output.aliases), (3, 5));
    for name in [
        "Broken",
        "BadResult",
        "BadParameter",
        "Transitive",
        "ResultUser",
        "ParameterUser",
        "TransitiveUser",
        "Variadic",
        "Annotated",
        "ZeroRegisters",
        "Registers",
        "RegisterUser",
        "Volatile",
        "VolatileUser",
    ] {
        assert!(
            result
                .omissions
                .iter()
                .any(|message| message.contains(name)),
            "{name}"
        );
    }
    for (root, reason) in [
        ("ZeroRegisters", "regparm=Some(0)"),
        ("Registers", "regparm=Some(2)"),
    ] {
        let error = capture(
            &[selected_fixture("callback_rejections", &[root])],
            &TARGETS,
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains(reason), "{error}");
    }
}

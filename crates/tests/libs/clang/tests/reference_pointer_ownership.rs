use test_clang::{input_dir, selected_fixture};
use windows_clang::*;
use windows_metadata::{Type, reader};

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

#[test]
fn reference_pointer_ownership() {
    helpers::ensure_libclang();
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let owner = root.join("target\\reference-pointer-owner.winmd");
    let producer = projection::generate(
        &[selected_fixture("reference_pointer_owner", &[])],
        "External",
    )
    .unwrap();
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_texts(producer.partitions.values().map(|part| part.rdl.as_str()))
        .output(&owner)
        .write()
        .unwrap();
    let metadata = reader::Index::read(&owner).unwrap();
    let references = projection::References::new(&metadata);
    let options = projection::Options {
        references: Some(&references),
        ..Default::default()
    };
    let input = selected_fixture("reference_pointer_consumer", &["LocalPacket"]);
    let snapshots = capture(&[input], &TARGETS).unwrap();
    for snapshot in &snapshots {
        let address = snapshot
            .aliases
            .iter()
            .find(|alias| alias.name == "Address")
            .unwrap();
        let TypeKind::Pointer(pointee) = snapshot.types[address.observations[0].canonical.0].kind
        else {
            panic!()
        };
        assert!(matches!(
            snapshot.types[pointee.0].kind,
            TypeKind::Record(_)
        ));
    }
    let result =
        projection::project_with_options(&merge(&snapshots).unwrap(), "Local", &options).unwrap();
    let text: String = result
        .partitions
        .values()
        .map(|part| part.rdl.as_str())
        .collect();
    let expected = input_dir().join("reference_pointer_consumer.rdl");
    if std::env::var_os("UPDATE_CLANG_RDL").is_some() {
        std::fs::write(&expected, &text).unwrap();
    }
    assert_eq!(
        text,
        std::fs::read_to_string(expected)
            .unwrap()
            .replace("\r\n", "\n")
    );
    let consumer = root.join("target\\reference-pointer-consumer.winmd");
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .reference(&owner)
        .input_text(&text)
        .output(&consumer)
        .write()
        .unwrap();
    let local = reader::Index::read(&consumer).unwrap();
    assert!(local.contains("Local", "Data"));
    assert!(!local.contains("Local", "Address"));
    assert_eq!(
        local
            .expect("Local", "LocalPacket")
            .fields()
            .next()
            .unwrap()
            .ty(),
        Type::value_named("External", "Address")
    );
    assert_eq!(
        metadata.expect("External", "Address").underlying_type(),
        Some(Type::PtrMut(Box::new(Type::Void), 1))
    );
    let partial = projection::project_partial_with_options(
        PartialCapture {
            snapshots,
            omissions: Vec::new(),
        },
        "Local",
        &options,
    )
    .unwrap();
    assert!(partial.omissions.is_empty(), "{:?}", partial.omissions);
    assert_eq!(partial.output, result);
    for prefix in [
        "#define ADDRESS_TYPE const Data*\n",
        "#define ADDRESS_TYPE Data**\n",
        "#define ADDRESS_TYPE unsigned long long\n",
        "#define ADDRESS_TYPE Data\n",
        "#define ADDRESS_TYPE FunctionAddress\n",
        "#define ADDRESS_TYPE Object*\n",
        "#define ADDRESS_TYPE Data* __ptr64\n",
        "#define CALLBACK_ABI __cdecl\n",
        "#define CALLBACK_PARAMETER unsigned int\n",
        "#define TYPED_ADDRESS_TYPE Data*\n",
        "#define TYPED_ADDRESS_TYPE void*\n",
    ] {
        let mut input = selected_fixture("reference_pointer_consumer", &["LocalPacket"]);
        input.source.insert_str(0, prefix);
        let snapshots = capture(&[input], &TARGETS).unwrap();
        assert!(
            projection::project_with_options(&merge(&snapshots).unwrap(), "Local", &options)
                .is_err(),
            "{prefix}"
        );
        let partial = projection::project_partial_with_options(
            PartialCapture {
                snapshots,
                omissions: Vec::new(),
            },
            "Local",
            &options,
        )
        .unwrap();
        assert!(!partial.omissions.is_empty(), "{prefix}");
        assert!(
            !partial
                .output
                .partitions
                .values()
                .any(|part| part.rdl.contains("struct LocalPacket")),
            "{prefix}"
        );
    }
    let mut other = selected_fixture("reference_pointer_consumer", &["LocalPacket"]);
    other.name = "conflicting.h".into();
    other.source.insert_str(0, "#define ADDRESS_TYPE int*\n");
    let inputs = [
        selected_fixture("reference_pointer_consumer", &["LocalPacket"]),
        other,
    ];
    assert!(capture_partial(&inputs, &TARGETS).is_err());
    for path in [owner, consumer] {
        std::fs::remove_file(path).unwrap();
    }
}

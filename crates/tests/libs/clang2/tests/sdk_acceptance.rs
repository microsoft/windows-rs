use windows_clang2::*;
use windows_metadata::{Value, reader};

const INTERFACES: [&str; 4] = [
    "ISpErrorLog",
    "ISpGrammarCompiler",
    "IWTSVirtualChannelManager",
    "IWTSBitmapRenderService",
];

#[test]
#[ignore = "requires the pinned SDK and clang resource headers"]
fn sdk_required_declarations() {
    helpers::ensure_libclang();
    let fixture = test_clang2::fixture("sdk_acceptance");
    let mut input = test_clang2::annotated_sdk_fixture();
    input.name = fixture.name;
    input.source = fixture.source;
    input.roots = INTERFACES
        .into_iter()
        .chain(["IMAGE_DOS_HEADER", "NtQuerySystemInformation"])
        .map(String::from)
        .collect();
    input.macros.push("SOMAXCONN".into());
    let snapshots = capture(&[input], &[Target::X86, Target::X64, Target::Arm64]).unwrap();
    let imports = projection::Imports::from_iter([(
        "NtQuerySystemInformation".into(),
        projection::Import {
            library: "ntdll.dll".into(),
            name: None,
        },
    )]);
    let output =
        projection::project_with_imports(&merge(&snapshots).unwrap(), "Windows.Win32", &imports)
            .unwrap();
    let rdl: String = output
        .partitions
        .iter()
        .filter(|(name, _)| {
            [
                "sapiddk",
                "tsvirtualchannels",
                "winnt",
                "winternl",
                "winsock2",
            ]
            .contains(&name.as_str())
        })
        .map(|(_, part)| part.rdl.as_str())
        .collect();
    let golden = test_clang2::input_dir().join("sdk_acceptance.rdl");
    if std::env::var_os("UPDATE_CLANG2_RDL").is_some() {
        std::fs::write(&golden, &rdl).unwrap();
    }
    assert_eq!(
        rdl,
        std::fs::read_to_string(golden)
            .unwrap()
            .replace("\r\n", "\n")
    );
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let winmd = root.join("target\\test-clang2-sdk-acceptance.winmd");
    let mut builder = windows_rdl::reader();
    builder.input(root.join("metadata\\metadata.rdl"));
    for partition in output.partitions.values() {
        builder.input_text(&partition.rdl);
    }
    builder.output(&winmd).write().unwrap();
    for snapshot in &snapshots {
        let bit = match snapshot.target {
            Target::X86 => 1,
            Target::X64 => 2,
            Target::Arm64 => 4,
        };
        let index =
            reader::Index::new_for_architecture(vec![reader::File::read(&winmd).unwrap()], bit);
        let dos = snapshot
            .records
            .iter()
            .find(|record| record.name == "_IMAGE_DOS_HEADER")
            .unwrap();
        let definition = snapshot.observations[dos.observations[0].0]
            .definition
            .as_ref()
            .unwrap();
        assert_eq!(
            definition.layout,
            Layout {
                size: 64,
                alignment: 2
            }
        );
        assert_eq!(
            definition
                .fields
                .iter()
                .find(|field| field.name == "e_lfanew")
                .unwrap()
                .offset_bits,
            60 * 8
        );
        let dos = index.expect("Windows.Win32", "IMAGE_DOS_HEADER");
        assert_eq!(dos.fields().count(), definition.fields.len());
        for name in INTERFACES {
            let record = snapshot
                .records
                .iter()
                .find(|record| record.name == name)
                .unwrap();
            let native = record
                .observations
                .iter()
                .find_map(|id| snapshot.observations[id.0].definition.as_ref())
                .unwrap();
            let definition = index.expect("Windows.Win32", name);
            assert_eq!(definition.category(), reader::TypeCategory::Interface);
            assert_eq!(
                definition
                    .methods()
                    .map(|method| method.name())
                    .collect::<Vec<_>>(),
                native
                    .methods
                    .iter()
                    .map(|method| method.name.as_str())
                    .collect::<Vec<_>>()
            );
        }
        let apis = index.expect("Windows.Win32", "Apis");
        let query = apis
            .methods()
            .find(|method| method.name() == "NtQuerySystemInformation")
            .unwrap();
        assert_eq!(query.signature(&[]).types.len(), 4);
        let backlog = apis
            .fields()
            .find(|field| field.name() == "SOMAXCONN")
            .unwrap();
        assert_eq!(backlog.constant().unwrap().value(), Value::I32(i32::MAX));
    }
    std::fs::remove_file(winmd).unwrap();
}

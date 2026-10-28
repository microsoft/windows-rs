use test_clang::{input_dir, selected_fixture};
use windows_clang::*;

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

#[test]
fn void_aliases() {
    helpers::ensure_libclang();
    let input = selected_fixture("void_aliases", &[]);
    let mut snapshots = capture(&[input], &TARGETS).unwrap();
    let imports = ["Clear", "Set", "Get"]
        .into_iter()
        .map(|name| {
            (
                name.into(),
                projection::Import {
                    library: "fixture.dll".into(),
                    name: None,
                },
            )
        })
        .collect();
    let output =
        projection::project_with_imports(&merge(&snapshots).unwrap(), "Fixture", &imports).unwrap();
    snapshots.reverse();
    assert_eq!(
        output,
        projection::project_with_imports(&merge(&snapshots).unwrap(), "Fixture", &imports).unwrap()
    );
    let text = &output.partitions["void_aliases"].rdl;
    let golden = input_dir().join("void_aliases.rdl");
    if std::env::var_os("UPDATE_CLANG_RDL").is_some() {
        std::fs::write(&golden, text).unwrap();
    }
    assert_eq!(
        *text,
        std::fs::read_to_string(golden)
            .unwrap()
            .replace("\r\n", "\n")
    );
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let scratch = root.join("target\\test-clang-void-aliases");
    std::fs::create_dir_all(&scratch).unwrap();
    let winmd = scratch.join("input.winmd");
    let rewritten = scratch.join("roundtrip.rdl");
    let mut source = text.clone();
    for _ in 0..2 {
        windows_rdl::reader()
            .input(root.join("metadata\\metadata.rdl"))
            .input_text(&source)
            .output(&winmd)
            .write()
            .unwrap();
        windows_rdl::writer()
            .input(&winmd)
            .filter("Fixture")
            .output(&rewritten)
            .write()
            .unwrap();
        source = std::fs::read_to_string(&rewritten).unwrap();
        assert_eq!(&source, text);
        let index = windows_metadata::reader::Index::read(&winmd).unwrap();
        for (owner, names) in [
            ("Apis", &["Clear", "Set"][..]),
            ("IConsumer", &["Clear", "Set"][..]),
            ("Callback", &["Invoke"][..]),
            ("Function", &["Invoke"][..]),
        ] {
            for name in names {
                let method = index
                    .expect("Fixture", owner)
                    .methods()
                    .find(|method| method.name() == *name)
                    .unwrap();
                assert_eq!(
                    method.signature(&[]).return_type,
                    windows_metadata::Type::Void
                );
            }
        }
        for architecture in [1, 2, 4] {
            let index = windows_metadata::reader::Index::new_for_architecture(
                vec![windows_metadata::reader::File::read(&winmd).unwrap()],
                architecture,
            );
            assert_eq!(
                index.expect("Fixture", "TargetValue").underlying_type(),
                Some(if architecture == 2 {
                    windows_metadata::Type::value_named("Fixture", "Holder")
                } else {
                    windows_metadata::Type::Void
                })
            );
        }
    }
    std::fs::remove_dir_all(scratch).unwrap();
}

#[test]
fn qualified_void_aliases_remain_unsupported() {
    helpers::ensure_libclang();
    let mut input = selected_fixture("void_aliases", &[]);
    input.source = "typedef const void Constant; typedef volatile void Volatile;".into();
    let result = projection::generate_partial(&[input], "Fixture").unwrap();
    for name in ["Constant", "Volatile"] {
        assert!(
            result
                .omissions
                .iter()
                .any(|message| message.contains(name)),
            "{:?}",
            result.omissions
        );
    }
    assert_eq!(result.output.aliases, 0);
}

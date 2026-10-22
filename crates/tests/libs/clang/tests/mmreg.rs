use windows_clang::*;

const GUIDS: [&str; 3] = [
    "KSDATAFORMAT_SUBTYPE_IEEE_FLOAT",
    "KSDATAFORMAT_SUBTYPE_PCM",
    "KSDATAFORMAT_SUBTYPE_WAVEFORMATEX",
];

fn input() -> Input {
    let mut input = test_clang::annotated_sdk_fixture();
    input.name = "mmreg_discovery.h".into();
    input.source = test_clang::fixture("mmreg_discovery").source;
    input.roots = vec!["MacroDiscoveryAnchor".into(), "GUID".into()];
    input
}

#[test]
#[ignore = "requires the pinned SDK"]
fn discovered_constant_rdl() {
    helpers::ensure_libclang();
    let mut input = input();
    input.macro_headers = vec!["mmreg.h".into()];
    let output = projection::generate_partial(&[input], "Fixture").unwrap();
    let actual = &output.output.partitions["mmreg"].rdl;
    let expected = std::fs::read_to_string(test_clang::input_dir().join("mmreg_discovery.rdl"))
        .unwrap()
        .replace("\r\n", "\n");
    for line in expected
        .lines()
        .filter(|line| line.starts_with("    const ") && !line.contains(": GUID ="))
    {
        let prefix = line.split_once(':').unwrap().0;
        assert!(
            actual.contains(line),
            "{line}: actual {:?}; {:?}",
            actual.lines().find(|line| line.starts_with(prefix)),
            output.omissions.first()
        );
    }
    for name in GUIDS {
        assert!(!actual.contains(&format!("const {name}:")));
        assert!(output.omissions.iter().any(|message| {
            message.starts_with(&format!("Macro(\"{name}\")")) && message.contains("incomplete")
        }));
        let fragment = format!("STATIC_{name}");
        assert!(!actual.contains(&format!("const {fragment}:")));
        assert!(output.omissions.iter().any(|message| {
            message.starts_with(&format!("Macro(\"{fragment}\")"))
                && message.contains("expression probe failed")
        }));
    }
}

#[test]
#[ignore = "requires the pinned SDK"]
fn explicit_guid_values() {
    helpers::ensure_libclang();
    let mut input = input();
    input.macros = GUIDS.map(String::from).into();
    let output = projection::generate_partial(&[input], "Fixture").unwrap();
    assert!(
        output
            .omissions
            .iter()
            .all(|message| message.starts_with("Tag(\"KSDATAFORMAT_SUBTYPE_")),
        "{:?}",
        output.omissions
    );
    assert_eq!(output.output.constants, 3);
    let actual = &output.output.partitions["mmreg"].rdl;
    let expected = std::fs::read_to_string(test_clang::input_dir().join("mmreg_discovery.rdl"))
        .unwrap()
        .replace("\r\n", "\n");
    for line in expected
        .lines()
        .filter(|line| line.contains("KSDATAFORMAT_SUBTYPE_"))
    {
        assert!(actual.contains(line), "{line}");
    }
}

#[test]
#[ignore = "requires the pinned SDK"]
fn active_companion_ownership() {
    helpers::ensure_libclang();
    for companions in [false, true] {
        let mut input = input();
        input.name = "mmreg_ownership.h".into();
        input.source = test_clang::fixture("mmreg_ownership").source;
        input.macro_headers = vec!["mmreg.h".into()];
        if companions {
            input
                .macro_headers
                .extend(["ksmedia.h".into(), "wmsdkidl.h".into()]);
        }
        let output = projection::generate_partial(&[input], "Fixture").unwrap();
        for header in ["ksmedia", "wmsdkidl"] {
            let expected = std::fs::read_to_string(
                test_clang::input_dir().join(format!("{header}_discovery.rdl")),
            )
            .unwrap();
            for line in expected
                .lines()
                .filter(|line| line.starts_with("    const "))
            {
                let name = line
                    .trim()
                    .strip_prefix("const ")
                    .unwrap()
                    .split_once(':')
                    .unwrap()
                    .0;
                assert!(
                    !output.output.partitions["mmreg"]
                        .rdl
                        .contains(&format!("const {name}:")),
                    "{name}"
                );
                if companions {
                    assert!(
                        output.output.partitions[header].rdl.contains(line),
                        "{line}"
                    );
                } else if name.starts_with("SPEAKER_") || name == "WAVE_FORMAT_DRM" {
                    assert!(
                        output.omissions.iter().any(|message| {
                            message.contains(&format!("macro `{name}`"))
                                && message.contains(&format!("{header}.h:"))
                                && message.contains("active definition is outside")
                        }),
                        "{name}: {:?}",
                        output.omissions
                    );
                }
            }
        }
    }
}

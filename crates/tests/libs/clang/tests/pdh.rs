use windows_clang::*;

#[test]
#[ignore = "requires the pinned SDK and clang resource headers"]
fn sdk_pdh_status_constants() {
    helpers::ensure_libclang();
    let mut input = test_clang::sdk_fixture();
    let fixture = test_clang::fixture("sdk_pdh");
    input.name = fixture.name;
    input.source = fixture.source;
    input.roots = vec!["GUID".into(), "S_PDH".into()];
    input.macro_headers = vec!["PdhMsg.h".into()];
    let snapshots = capture(&[input], &[Target::X86, Target::X64, Target::Arm64]).unwrap();
    for snapshot in &snapshots {
        assert_eq!(snapshot.values.len(), 89);
        let provider = snapshot
            .values
            .iter()
            .find(|value| value.name == "S_PDH")
            .unwrap();
        assert!(provider.observations.iter().all(|observation| {
            matches!(
                observation.kind,
                ValueKind::Constant {
                    value: Some(ConstantValue::Guid(0x04d66358_c4a1_419b_8023_23b73902de2c)),
                    ..
                }
            )
        }));
        for (name, expected) in [
            ("PDH_CSTATUS_VALID_DATA", EnumValue::Signed(0)),
            ("PDH_CSTATUS_NEW_DATA", EnumValue::Signed(1)),
            ("PDH_ACCESS_DENIED", EnumValue::Unsigned(0xC0000BDB)),
        ] {
            let expected = ConstantValue::Integer(expected);
            let value = snapshot
                .values
                .iter()
                .find(|value| value.name == name)
                .unwrap();
            assert!(value.observations.iter().all(|observation| {
                matches!(
                    &observation.kind,
                    ValueKind::Constant { value: Some(actual), .. } if actual == &expected
                )
            }));
        }
    }
    let output = projection::project(&merge(&snapshots).unwrap(), "Windows.Win32").unwrap();
    let actual = &output.partitions["pdhmsg"].rdl;
    let expected = test_clang::input_dir().join("sdk_pdh.rdl");
    if std::env::var_os("UPDATE_CLANG_RDL").is_some() {
        std::fs::write(&expected, actual).unwrap();
    }
    assert_eq!(
        *actual,
        std::fs::read_to_string(expected)
            .unwrap()
            .replace("\r\n", "\n")
    );
}

use windows::runtime::GUID;

#[test]
fn test_new() {
    let zeroed = GUID::zeroed();
    let unique = GUID::new().unwrap();
    assert_ne!(zeroed, unique);
}

#[test]
fn from_u128() {
    let a: GUID = "1FD63FEF-C0D2-42FE-823A-53A4052B8C8F".into();
    let b = GUID::from_values(0x1fd63fef, 0xc0d2, 0x42fe, [0x82, 0x3a, 0x53, 0xa4, 0x5, 0x2b, 0x8c, 0x8f]);
    let c = GUID::from_u128(0x1fd63fef_c0d2_42fe_823a_53a4052b8c8f);

    assert!(a == b);
    assert!(a == c);
}

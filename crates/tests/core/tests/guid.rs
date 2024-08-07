use windows::{core::*, Win32::Foundation::E_INVALIDARG};

#[test]
fn test_new() {
    let zeroed = GUID::zeroed();
    let unique = GUID::new().unwrap();
    assert_ne!(zeroed, unique);
}

#[test]
fn from_u128() {
    let a: GUID = "1FD63FEF-C0D2-42FE-823A-53A4052B8C8F".try_into().unwrap();
    let b = GUID::from_values(
        0x1fd63fef,
        0xc0d2,
        0x42fe,
        [0x82, 0x3a, 0x53, 0xa4, 0x5, 0x2b, 0x8c, 0x8f],
    );
    let c = GUID::from_u128(0x1fd63fef_c0d2_42fe_823a_53a4052b8c8f);

    assert!(a == b);
    assert!(a == c);
}

#[test]
fn to_u128() {
    let num: u128 = 0x1fd63fef_c0d2_42fe_823a_53a4052b8c8f;
    let guid: GUID = "1FD63FEF-C0D2-42FE-823A-53A4052B8C8F".try_into().unwrap();

    assert_eq!(u128::from(guid), num); // From<GUID>
}

#[test]
fn parsing() {
    assert_eq!(
        GUID::zeroed(),
        "00000000-0000-0000-0000-000000000000".try_into().unwrap()
    );

    // Validate invalid length and expected error information.
    let e = GUID::try_from("wrong length").unwrap_err();
    assert_eq!(e.code(), E_INVALIDARG);
    assert!(e.as_ptr().is_null());

    // Validate delimiter
    GUID::try_from("00000000?0000-0000-0000-000000000000").unwrap_err();
    GUID::try_from("00000000-0000?0000-0000-000000000000").unwrap_err();
    GUID::try_from("00000000-0000-0000?0000-000000000000").unwrap_err();
    GUID::try_from("00000000-0000-0000-0000?000000000000").unwrap_err();

    // Validate invalid digits
    GUID::try_from("z0000000-0000-0000-0000-000000000000").unwrap_err();
    GUID::try_from("00000000-z000-0000-0000-000000000000").unwrap_err();
    GUID::try_from("00000000-0000-z000-0000-000000000000").unwrap_err();
    GUID::try_from("00000000-0000-0000-z000-000000000000").unwrap_err();
    GUID::try_from("00000000-0000-0000-0000-z00000000000").unwrap_err();

    // Validate case insensitivity
    let value = GUID::from_u128(0x1fd63fef_c0d2_42fe_823a_53a4052b8c8f);
    assert_eq!(
        value,
        "1FD63FEF-C0D2-42FE-823A-53A4052B8C8F".try_into().unwrap()
    );
    assert_eq!(
        value,
        "1fd63fef-c0d2-42fe-823a-53a4052b8c8f".try_into().unwrap()
    );
}

#[test]
fn debug() {
    let value = GUID::from_u128(0x1fd63fef_c0d2_42fe_823a_53a4052b8c8f);

    assert_eq!(format!("{value:?}"), "1FD63FEF-C0D2-42FE-823A-53A4052B8C8F");
}

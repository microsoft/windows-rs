winrt::import!(
    dependencies
        os
    types
        windows::foundation::*
        windows::application_model::appointments::AppointmentDaysOfWeek
);

use winrt::AbiTransferable;

#[test]
fn signed_enum() {
    use windows::foundation::AsyncStatus;

    assert!(AsyncStatus::default().get_abi() == 0);
    assert!(AsyncStatus::Canceled.get_abi() == 2);
    assert!(AsyncStatus::Completed.get_abi() == 1);
    assert!(AsyncStatus::Error.get_abi() == 3);
    assert!(AsyncStatus::Started.get_abi() == 0);
}

#[test]
fn unsigned_enum() {
    use windows::application_model::appointments::AppointmentDaysOfWeek;

    assert!(AppointmentDaysOfWeek::default().get_abi() == 0);
    assert!(AppointmentDaysOfWeek::None.get_abi() == 0);
    assert!(AppointmentDaysOfWeek::Sunday.get_abi() == 0x1);
    assert!(AppointmentDaysOfWeek::Monday.get_abi() == 0x2);
    assert!(AppointmentDaysOfWeek::Tuesday.get_abi() == 0x4);
    assert!(AppointmentDaysOfWeek::Wednesday.get_abi() == 0x8);
    assert!(AppointmentDaysOfWeek::Thursday.get_abi() == 0x10);
    assert!(AppointmentDaysOfWeek::Friday.get_abi() == 0x20);
    assert!(AppointmentDaysOfWeek::Saturday.get_abi() == 0x40);

    // Use as bitflags
    let weekend = AppointmentDaysOfWeek::Sunday | AppointmentDaysOfWeek::Saturday;
    assert!(weekend.get_abi() == 0x41);
}

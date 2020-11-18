use tests::windows::application_model::appointments::AppointmentDaysOfWeek;
use winrt::foundation::AsyncStatus;
use winrt::Abi;

#[test]
fn signed_enum() {
    assert!(AsyncStatus::default().abi() == 0);
    assert!(AsyncStatus::Canceled.abi() == 2);
    assert!(AsyncStatus::Completed.abi() == 1);
    assert!(AsyncStatus::Error.abi() == 3);
    assert!(AsyncStatus::Started.abi() == 0);
}

#[test]
fn unsigned_enum() {
    assert!(AppointmentDaysOfWeek::default().abi() == 0);
    assert!(AppointmentDaysOfWeek::None.abi() == 0);
    assert!(AppointmentDaysOfWeek::Sunday.abi() == 0x1);
    assert!(AppointmentDaysOfWeek::Monday.abi() == 0x2);
    assert!(AppointmentDaysOfWeek::Tuesday.abi() == 0x4);
    assert!(AppointmentDaysOfWeek::Wednesday.abi() == 0x8);
    assert!(AppointmentDaysOfWeek::Thursday.abi() == 0x10);
    assert!(AppointmentDaysOfWeek::Friday.abi() == 0x20);
    assert!(AppointmentDaysOfWeek::Saturday.abi() == 0x40);

    // Use as bitflags
    let weekend = AppointmentDaysOfWeek::Sunday | AppointmentDaysOfWeek::Saturday;
    assert!(weekend.abi() == 0x41);
}

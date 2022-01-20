use windows::{ApplicationModel::Appointments::AppointmentDaysOfWeek, Foundation::AsyncStatus};

#[test]
fn signed_enum() {
    assert!(AsyncStatus::default().0 == 0);
    assert!(AsyncStatus::Canceled.0 == 2);
    assert!(AsyncStatus::Completed.0 == 1);
    assert!(AsyncStatus::Error.0 == 3);
    assert!(AsyncStatus::Started.0 == 0);
}

#[test]
fn unsigned_enum() {
    assert!(AppointmentDaysOfWeek::default().0 == 0);
    assert!(AppointmentDaysOfWeek::None.0 == 0);
    assert!(AppointmentDaysOfWeek::Sunday.0 == 0x1);
    assert!(AppointmentDaysOfWeek::Monday.0 == 0x2);
    assert!(AppointmentDaysOfWeek::Tuesday.0 == 0x4);
    assert!(AppointmentDaysOfWeek::Wednesday.0 == 0x8);
    assert!(AppointmentDaysOfWeek::Thursday.0 == 0x10);
    assert!(AppointmentDaysOfWeek::Friday.0 == 0x20);
    assert!(AppointmentDaysOfWeek::Saturday.0 == 0x40);

    // Use as bitflags
    let weekend = AppointmentDaysOfWeek::Sunday | AppointmentDaysOfWeek::Saturday;
    assert!(weekend.0 == 0x41);

    let mut days = AppointmentDaysOfWeek::Monday;
    days |= AppointmentDaysOfWeek::Tuesday;
    days |= AppointmentDaysOfWeek::Wednesday;

    assert!(days == AppointmentDaysOfWeek::Monday | AppointmentDaysOfWeek::Tuesday | AppointmentDaysOfWeek::Wednesday);

    days &= AppointmentDaysOfWeek::Monday | AppointmentDaysOfWeek::Wednesday | AppointmentDaysOfWeek::Friday;

    assert!(days == AppointmentDaysOfWeek::Monday | AppointmentDaysOfWeek::Wednesday);
    days &= AppointmentDaysOfWeek::Wednesday;
    assert!(days == AppointmentDaysOfWeek::Wednesday);
}

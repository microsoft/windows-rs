use windows::ApplicationModel::Appointments::AppointmentDaysOfWeek;
use windows_future::*;

#[test]
fn signed_enum() {
    assert_eq!(AsyncStatus::default().0, 0);
    assert_eq!(AsyncStatus::Canceled.0, 2);
    assert_eq!(AsyncStatus::Completed.0, 1);
    assert_eq!(AsyncStatus::Error.0, 3);
    assert_eq!(AsyncStatus::Started.0, 0);
}

#[test]
fn unsigned_enum() {
    assert_eq!(AppointmentDaysOfWeek::default().0, 0);
    assert_eq!(AppointmentDaysOfWeek::None.0, 0);
    assert_eq!(AppointmentDaysOfWeek::Sunday.0, 0x1);
    assert_eq!(AppointmentDaysOfWeek::Monday.0, 0x2);
    assert_eq!(AppointmentDaysOfWeek::Tuesday.0, 0x4);
    assert_eq!(AppointmentDaysOfWeek::Wednesday.0, 0x8);
    assert_eq!(AppointmentDaysOfWeek::Thursday.0, 0x10);
    assert_eq!(AppointmentDaysOfWeek::Friday.0, 0x20);
    assert_eq!(AppointmentDaysOfWeek::Saturday.0, 0x40);

    // Use as bitflags
    let weekend = AppointmentDaysOfWeek::Sunday | AppointmentDaysOfWeek::Saturday;
    assert!(weekend.0 == 0x41);

    let mut days = AppointmentDaysOfWeek::Monday;
    days |= AppointmentDaysOfWeek::Tuesday;
    days |= AppointmentDaysOfWeek::Wednesday;

    assert!(
        days == AppointmentDaysOfWeek::Monday
            | AppointmentDaysOfWeek::Tuesday
            | AppointmentDaysOfWeek::Wednesday
    );

    days &= AppointmentDaysOfWeek::Monday
        | AppointmentDaysOfWeek::Wednesday
        | AppointmentDaysOfWeek::Friday;

    assert!(days == AppointmentDaysOfWeek::Monday | AppointmentDaysOfWeek::Wednesday);
    days &= AppointmentDaysOfWeek::Wednesday;
    assert!(days == AppointmentDaysOfWeek::Wednesday);
}

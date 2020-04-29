winrt::import!(
    dependencies
        "os"
    modules
        "windows.foundation"
        "windows.application_model.appointments"
);

#[test]
fn signed_enum() {
    use windows::foundation::AsyncStatus;

    assert!(AsyncStatus::default() == AsyncStatus::Canceled);
    assert!(AsyncStatus::Canceled as i32 == 2);
    assert!(AsyncStatus::Completed as i32 == 1);
    assert!(AsyncStatus::Error as i32 == 3);
    assert!(AsyncStatus::Started as i32 == 0);
}

#[test]
fn unsigned_enum() {
    use windows::application_model::appointments::AppointmentDaysOfWeek;

    assert!(AppointmentDaysOfWeek::default() == AppointmentDaysOfWeek::None);
    assert!(AppointmentDaysOfWeek::None as u32 == 0);
    assert!(AppointmentDaysOfWeek::Sunday as u32 == 0x1);
    assert!(AppointmentDaysOfWeek::Monday as u32 == 0x2);
    assert!(AppointmentDaysOfWeek::Tuesday as u32 == 0x4);
    assert!(AppointmentDaysOfWeek::Wednesday as u32 == 0x8);
    assert!(AppointmentDaysOfWeek::Thursday as u32 == 0x10);
    assert!(AppointmentDaysOfWeek::Friday as u32 == 0x20);
    assert!(AppointmentDaysOfWeek::Saturday as u32 == 0x40);

    // TODO: Unsigned WinRT enums are meant to be used as bit flags:
    // let weekend = AppointmentDaysOfWeek::Sunday | AppointmentDaysOfWeek::Saturday;
    // assert!(weekend as u32 == 0x41);
}

use tests::windows::application_model::appointments::AppointmentDaysOfWeek;
use winrt::foundation::AsyncStatus;

#[test]
fn signed_enum() {
    assert!(AsyncStatus::default() == 0.into());
    assert!(AsyncStatus::Canceled == 2.into());
    assert!(AsyncStatus::Completed == 1.into());
    assert!(AsyncStatus::Error == 3.into());
    assert!(AsyncStatus::Started == 0.into());
}

#[test]
fn unsigned_enum() {
    assert!(AppointmentDaysOfWeek::default() == 0.into());
    assert!(AppointmentDaysOfWeek::None == 0.into());
    assert!(AppointmentDaysOfWeek::Sunday == 0x1.into());
    assert!(AppointmentDaysOfWeek::Monday == 0x2.into());
    assert!(AppointmentDaysOfWeek::Tuesday == 0x4.into());
    assert!(AppointmentDaysOfWeek::Wednesday == 0x8.into());
    assert!(AppointmentDaysOfWeek::Thursday == 0x10.into());
    assert!(AppointmentDaysOfWeek::Friday == 0x20.into());
    assert!(AppointmentDaysOfWeek::Saturday == 0x40.into());

    // Use as bitflags
    let weekend = AppointmentDaysOfWeek::Sunday | AppointmentDaysOfWeek::Saturday;
    assert!(weekend == 0x41.into());
}

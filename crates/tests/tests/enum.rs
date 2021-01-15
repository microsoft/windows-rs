use tests::windows::application_model::appointments::AppointmentDaysOfWeek;
use windows::foundation::AsyncStatus;

#[test]
fn signed_enum() {
    assert!(AsyncStatus::default() == 0.into());
    assert!(AsyncStatus::Canceled == 2.into());
    assert!(AsyncStatus::Completed == 1.into());
    assert!(AsyncStatus::Error == 3.into());
    assert!(AsyncStatus::Started == 0.into());

    assert!(AsyncStatus::default().0 == 0);
    assert!(AsyncStatus::Canceled.0 == 2);
    assert!(AsyncStatus::Completed.0 == 1);
    assert!(AsyncStatus::Error.0 == 3);
    assert!(AsyncStatus::Started.0 == 0);
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
    assert!(weekend == 0x41.into());
    assert!(weekend.0 == 0x41);
}

winrt::import!(
    dependencies
        "os"
    modules
        "windows.foundation"
        "windows.application_model.appointments"
);

#[test]
fn signature() -> winrt::Result<()> {
    use winrt::RuntimeType;

    assert_eq!(bool::signature(), "b1");
    assert_eq!(i8::signature(), "i1");
    assert_eq!(u8::signature(), "u1");
    assert_eq!(i16::signature(), "i2");
    assert_eq!(u16::signature(), "u2");
    assert_eq!(i32::signature(), "i4");
    assert_eq!(u32::signature(), "u4");
    assert_eq!(i64::signature(), "i8");
    assert_eq!(u64::signature(), "u8");
    assert_eq!(f32::signature(), "f4");
    assert_eq!(f64::signature(), "f8");

    assert_eq!(winrt::Guid::signature(), "g16");
    assert_eq!(winrt::HString::signature(), "string");
    assert_eq!(winrt::Object::signature(), "cinterface(IInspectable)");

    // Class with non-generic default interface
    assert_eq!(
        windows::foundation::Uri::signature(),
        "rc(Windows.Foundation.Uri;{9e365e57-48b2-4160-956f-c7385120bbfc})"
    );

    // Class with generic default interface
    assert_eq!(
        windows::foundation::WwwFormUrlDecoder::signature(),
        "rc(Windows.Foundation.WwwFormUrlDecoder;{d45a0451-f225-4542-9296-0e1df5d254df})"
    );

    // Non-generic interface
    assert_eq!(
        windows::foundation::IAsyncAction::signature(),
        "{5a648006-843a-4da9-865b-9d26e5dfad7b}"
    );

    // Non-generic delegate
    assert_eq!(
        windows::foundation::AsyncActionCompletedHandler::signature(),
        "delegate({a4ed5c81-76c9-40bd-8be6-b1d90fb20ae7})"
    );

    // Generic interface
    assert_eq!(
        windows::foundation::collections::IVector::<i32>::signature(),
        "pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};i4)"
    );

    // TODO: Generic delegate
    assert_eq!(
        windows::foundation::EventHandler::<windows::foundation::IStringable>::signature(),
        "pinterface({9de1c535-6ae1-11e0-84e1-18a905bcc53f};{96369f54-8eb6-48f0-abce-c1b211e627c3})"
    );

    // Signed enum
    assert_eq!(
        windows::foundation::AsyncStatus::signature(),
        "enum(Windows.Foundation.AsyncStatus;i4)"
    );

    // Unsigned enum
    assert_eq!(
        windows::application_model::appointments::AppointmentDaysOfWeek::signature(),
        "enum(Windows.ApplicationModel.Appointments.AppointmentDaysOfWeek;u4)"
    );

    // Simple struct
    assert_eq!(
        windows::foundation::Rect::signature(),
        "struct(Windows.Foundation.Rect;f4;f4;f4;f4)"
    );

    // TODO: struct with string field

    // TODO: struct with IReference<T> field

    // TODO: struct with nested struct

    Ok(())
}

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
    use winrt::ComInterface;

    // Generic types for exhaustive tests
    type A = windows::foundation::collections::IIterable::<windows::foundation::IStringable>;
    type B = windows::foundation::collections::IKeyValuePair::<winrt::HString, windows::foundation::IAsyncOperationWithProgress<A, f32>>;

    // assert_eq!(A::signature(),
    // "pinterface({faa585ea-6214-4217-afda-7f46de5869b3};{96369f54-8eb6-48f0-abce-c1b211e627c3})");

    // assert_eq!(B::signature(),
    // "pinterface({02b51929-c1c4-4a7e-8940-0312b5c18500};string;pinterface({b5d036d7-e297-498f-ba60-0289e76e23dd};pinterface({faa585ea-6214-4217-afda-7f46de5869b3};{96369f54-8eb6-48f0-abce-c1b211e627c3});f4))");

    // assert_eq!(windows::foundation::EventHandler::<A>::signature(),
    // "pinterface({9de1c535-6ae1-11e0-84e1-18a905bcc53f};pinterface({faa585ea-6214-4217-afda-7f46de5869b3};{96369f54-8eb6-48f0-abce-c1b211e627c3}))");

    // assert_eq!(windows::foundation::EventHandler::<B>::signature(),
    // "pinterface({9de1c535-6ae1-11e0-84e1-18a905bcc53f};pinterface({faa585ea-6214-4217-afda-7f46de5869b3};{96369f54-8eb6-48f0-abce-c1b211e627c3}))");

    // assert_eq!(bool::signature(), "b1");
    // assert_eq!(i8::signature(), "i1");
    // assert_eq!(u8::signature(), "u1");
    // assert_eq!(i16::signature(), "i2");
    // assert_eq!(u16::signature(), "u2");
    // assert_eq!(i32::signature(), "i4");
    // assert_eq!(u32::signature(), "u4");
    // assert_eq!(i64::signature(), "i8");
    // assert_eq!(u64::signature(), "u8");
    // assert_eq!(f32::signature(), "f4");
    // assert_eq!(f64::signature(), "f8");

    // assert_eq!(winrt::Guid::signature(), "g16");
    // assert_eq!(winrt::HString::signature(), "string");
    // assert_eq!(winrt::Object::signature(), "cinterface(IInspectable)");

    // // Class with non-generic default interface
    // assert_eq!(
    //     windows::foundation::Uri::signature(),
    //     "rc(Windows.Foundation.Uri;{9e365e57-48b2-4160-956f-c7385120bbfc})"
    // );

    // // Class with generic default interface
    // assert_eq!(
    //     windows::foundation::WwwFormUrlDecoder::signature(),
    //     "rc(Windows.Foundation.WwwFormUrlDecoder;{d45a0451-f225-4542-9296-0e1df5d254df})"
    // );

    // // Non-generic interface
    // assert_eq!(
    //     windows::foundation::IAsyncAction::signature(),
    //     "{5a648006-843a-4da9-865b-9d26e5dfad7b}"
    // );

    // // Non-generic delegate
    // assert_eq!(
    //     windows::foundation::AsyncActionCompletedHandler::signature(),
    //     "delegate({a4ed5c81-76c9-40bd-8be6-b1d90fb20ae7})"
    // );

    // // Generic interface
    // assert_eq!(
    //     windows::foundation::collections::IVector::<i32>::signature(),
    //     "pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};i4)"
    // );

    // // TODO: Generic delegate
    // assert_eq!(
    //     windows::foundation::EventHandler::<windows::foundation::IStringable>::signature(),
    //     "pinterface({9de1c535-6ae1-11e0-84e1-18a905bcc53f};{96369f54-8eb6-48f0-abce-c1b211e627c3})"
    // );

    // // Signed enum
    // assert_eq!(
    //     windows::foundation::AsyncStatus::signature(),
    //     "enum(Windows.Foundation.AsyncStatus;i4)"
    // );

    // // Unsigned enum
    // assert_eq!(
    //     windows::application_model::appointments::AppointmentDaysOfWeek::signature(),
    //     "enum(Windows.ApplicationModel.Appointments.AppointmentDaysOfWeek;u4)"
    // );

    // // Simple struct
    // assert_eq!(
    //     windows::foundation::Rect::signature(),
    //     "struct(Windows.Foundation.Rect;f4;f4;f4;f4)"
    // );

    // // TODO: struct with string field

    // // TODO: struct with IReference<T> field

    // // TODO: struct with nested struct

    // // // Windows.Foundation

    assert_eq!(
        windows::foundation::IAsyncActionWithProgress::<A>::iid(),
        winrt::Guid::from("DD725452-2DA3-5103-9C7D-22EE9BB14AD3")
    );

    assert_eq!(
        windows::foundation::IAsyncOperationWithProgress::<A, B>::iid(),
        winrt::Guid::from("94645425-B9E5-5B91-B509-8DA4DF6A8916")
    );

    assert_eq!(
        windows::foundation::IAsyncOperation::<A>::iid(),
        winrt::Guid::from("2BD35EE6-72D9-5C5D-9827-05EBB81487AB")
    );

    assert_eq!(
        windows::foundation::IReferenceArray::<A>::iid(),
        winrt::Guid::from("4A33FE03-E8B9-5346-A124-5449913ECA57")
    );

    assert_eq!(
        windows::foundation::IReference::<A>::iid(),
        winrt::Guid::from("F9E4006C-6E8C-56DF-811C-61F9990EBFB0")
    );

    assert_eq!(
        windows::foundation::AsyncActionProgressHandler::<A>::iid(),
        winrt::Guid::from("C261D8D0-71BA-5F38-A239-872342253A18")
    );

    assert_eq!(
        windows::foundation::AsyncActionWithProgressCompletedHandler::<A>::iid(),
        winrt::Guid::from("9A0D211C-0374-5D23-9E15-EAA3570FAE63")
    );

    assert_eq!(
        windows::foundation::AsyncOperationCompletedHandler::<A>::iid(),
        winrt::Guid::from("9D534225-231F-55E7-A6D0-6C938E2D9160")
    );

    assert_eq!(
        windows::foundation::AsyncOperationProgressHandler::<A, B>::iid(),
        winrt::Guid::from("264F1E0C-ABE4-590B-9D37-E1CC118ECC75")
    );

    assert_eq!(
        windows::foundation::AsyncOperationWithProgressCompletedHandler::<A, B>::iid(),
        winrt::Guid::from("C2D078D8-AC47-55AB-83E8-123B2BE5BC5A")
    );

    assert_eq!(
        windows::foundation::EventHandler::<A>::iid(),
        winrt::Guid::from("FA0B7D80-7EFA-52DF-9B69-0574CE57ADA4")
    );

    assert_eq!(
        windows::foundation::TypedEventHandler::<A, B>::iid(),
        winrt::Guid::from("EDB31843-B4CF-56EB-925A-D4D0CE97A08D")
    );

    Ok(())
}

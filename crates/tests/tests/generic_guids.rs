#[test]
fn generic_guids() -> winrt::Result<()> {
    use winrt::Interface;

    type A = winrt::foundation::collections::IIterable<winrt::foundation::IStringable>;
    type B = winrt::foundation::collections::IKeyValuePair<
        winrt::HString,
        winrt::foundation::IAsyncOperationWithProgress<A, f32>,
    >;

    //
    // Generated Windows.Foundation GUIDs
    //
    assert_eq!(
        winrt::foundation::IAsyncActionWithProgress::<A>::IID,
        winrt::Guid::from("DD725452-2DA3-5103-9C7D-22EE9BB14AD3")
    );

    assert_eq!(
        winrt::foundation::IAsyncOperationWithProgress::<A, B>::IID,
        winrt::Guid::from("94645425-B9E5-5B91-B509-8DA4DF6A8916")
    );

    assert_eq!(
        winrt::foundation::IAsyncOperation::<A>::IID,
        winrt::Guid::from("2BD35EE6-72D9-5C5D-9827-05EBB81487AB")
    );

    assert_eq!(
        winrt::foundation::IReferenceArray::<A>::IID,
        winrt::Guid::from("4A33FE03-E8B9-5346-A124-5449913ECA57")
    );

    assert_eq!(
        winrt::foundation::IReference::<A>::IID,
        winrt::Guid::from("F9E4006C-6E8C-56DF-811C-61F9990EBFB0")
    );

    assert_eq!(
        winrt::foundation::AsyncActionProgressHandler::<A>::IID,
        winrt::Guid::from("C261D8D0-71BA-5F38-A239-872342253A18")
    );

    assert_eq!(
        winrt::foundation::AsyncActionWithProgressCompletedHandler::<A>::IID,
        winrt::Guid::from("9A0D211C-0374-5D23-9E15-EAA3570FAE63")
    );

    assert_eq!(
        winrt::foundation::AsyncOperationCompletedHandler::<A>::IID,
        winrt::Guid::from("9D534225-231F-55E7-A6D0-6C938E2D9160")
    );

    assert_eq!(
        winrt::foundation::AsyncOperationProgressHandler::<A, B>::IID,
        winrt::Guid::from("264F1E0C-ABE4-590B-9D37-E1CC118ECC75")
    );

    assert_eq!(
        winrt::foundation::AsyncOperationWithProgressCompletedHandler::<A, B>::IID,
        winrt::Guid::from("C2D078D8-AC47-55AB-83E8-123B2BE5BC5A")
    );

    assert_eq!(
        winrt::foundation::EventHandler::<A>::IID,
        winrt::Guid::from("FA0B7D80-7EFA-52DF-9B69-0574CE57ADA4")
    );

    assert_eq!(
        winrt::foundation::TypedEventHandler::<A, B>::IID,
        winrt::Guid::from("EDB31843-B4CF-56EB-925A-D4D0CE97A08D")
    );

    //
    // Generated Windows.Foundation.Collections GUIDs
    //

    assert_eq!(
        winrt::foundation::collections::IIterable::<A>::IID,
        winrt::Guid::from("96565EB9-A692-59C8-BCB5-647CDE4E6C4D")
    );

    assert_eq!(
        winrt::foundation::collections::IIterator::<A>::IID,
        winrt::Guid::from("3C9B1E27-8357-590B-8828-6E917F172390")
    );

    assert_eq!(
        winrt::foundation::collections::IKeyValuePair::<A, B>::IID,
        winrt::Guid::from("89336CD9-8B66-50A7-9759-EB88CCB2E1FE")
    );

    assert_eq!(
        winrt::foundation::collections::IMapChangedEventArgs::<A>::IID,
        winrt::Guid::from("E1AA5138-12BD-51A1-8558-698DFD070ABE")
    );

    assert_eq!(
        winrt::foundation::collections::IMapView::<A, B>::IID,
        winrt::Guid::from("B78F0653-FA89-59CF-BA95-726938AAE666")
    );

    assert_eq!(
        winrt::foundation::collections::IMap::<A, B>::IID,
        winrt::Guid::from("9962CD50-09D5-5C46-B1E1-3C679C1C8FAE")
    );

    assert_eq!(
        winrt::foundation::collections::IObservableMap::<A, B>::IID,
        winrt::Guid::from("75F99E2A-137E-537E-A5B1-0B5A6245FC02")
    );

    assert_eq!(
        winrt::foundation::collections::IObservableVector::<A>::IID,
        winrt::Guid::from("D24C289F-2341-5128-AAA1-292DD0DC1950")
    );

    assert_eq!(
        winrt::foundation::collections::IVectorView::<A>::IID,
        winrt::Guid::from("5F07498B-8E14-556E-9D2E-2E98D5615DA9")
    );

    assert_eq!(
        winrt::foundation::collections::IVector::<A>::IID,
        winrt::Guid::from("0E3F106F-A266-50A1-8043-C90FCF3844F6")
    );

    assert_eq!(
        winrt::foundation::collections::MapChangedEventHandler::<A, B>::IID,
        winrt::Guid::from("19046F0B-CF81-5DEC-BBB2-7CC250DA8B8B")
    );

    assert_eq!(
        winrt::foundation::collections::VectorChangedEventHandler::<A>::IID,
        winrt::Guid::from("A1E9ACD7-E4DF-5A79-AEFA-DE07934AB0FB")
    );

    //
    // Generated primitive GUIDs
    //

    assert_eq!(
        winrt::foundation::IReference::<bool>::IID,
        winrt::Guid::from("3C00FD60-2950-5939-A21A-2D12C5A01B8A")
    );

    assert_eq!(
        winrt::foundation::IReference::<i8>::IID,
        winrt::Guid::from("95500129-FBF6-5AFC-89DF-70642D741990")
    );

    assert_eq!(
        winrt::foundation::IReference::<i16>::IID,
        winrt::Guid::from("6EC9E41B-6709-5647-9918-A1270110FC4E")
    );

    assert_eq!(
        winrt::foundation::IReference::<i32>::IID,
        winrt::Guid::from("548CEFBD-BC8A-5FA0-8DF2-957440FC8BF4")
    );

    assert_eq!(
        winrt::foundation::IReference::<i64>::IID,
        winrt::Guid::from("4DDA9E24-E69F-5C6A-A0A6-93427365AF2A")
    );

    assert_eq!(
        winrt::foundation::IReference::<u8>::IID,
        winrt::Guid::from("e5198cc8-2873-55f5-b0a1-84ff9e4aad62")
    );

    assert_eq!(
        winrt::foundation::IReference::<u16>::IID,
        winrt::Guid::from("5AB7D2C3-6B62-5E71-A4B6-2D49C4F238FD")
    );

    assert_eq!(
        winrt::foundation::IReference::<u32>::IID,
        winrt::Guid::from("513ef3af-e784-5325-a91e-97c2b8111cf3")
    );

    assert_eq!(
        winrt::foundation::IReference::<u64>::IID,
        winrt::Guid::from("6755e376-53bb-568b-a11d-17239868309e")
    );

    assert_eq!(
        winrt::foundation::IReference::<f32>::IID,
        winrt::Guid::from("719CC2BA-3E76-5DEF-9F1A-38D85A145EA8")
    );

    assert_eq!(
        winrt::foundation::IReference::<f64>::IID,
        winrt::Guid::from("2F2D6C29-5473-5F3E-92E7-96572BB990E2")
    );

    assert_eq!(
        winrt::foundation::IReference::<winrt::Guid>::IID,
        winrt::Guid::from("7D50F649-632C-51F9-849A-EE49428933EA")
    );

    assert_eq!(
        winrt::foundation::IReference::<winrt::HString>::IID,
        winrt::Guid::from("FD416DFB-2A07-52EB-AAE3-DFCE14116C05")
    );

    // TODO: etc.

    Ok(())
}

use test_winrt::{Windows::Foundation::Collections::*, Windows::Foundation::*};

#[test]
fn generic_guids() -> windows::runtime::Result<()> {
    use windows::runtime::Interface;

    type A = IIterable<IStringable>;
    type B = IKeyValuePair<windows::runtime::HSTRING, IAsyncOperationWithProgress<A, f32>>;

    //
    // Generated Windows.Foundation GUIDs
    //
    assert_eq!(IAsyncActionWithProgress::<A>::IID, windows::runtime::GUID::from("DD725452-2DA3-5103-9C7D-22EE9BB14AD3"));

    assert_eq!(IAsyncOperationWithProgress::<A, B>::IID, windows::runtime::GUID::from("94645425-B9E5-5B91-B509-8DA4DF6A8916"));

    assert_eq!(IAsyncOperation::<A>::IID, windows::runtime::GUID::from("2BD35EE6-72D9-5C5D-9827-05EBB81487AB"));

    assert_eq!(IReferenceArray::<A>::IID, windows::runtime::GUID::from("4A33FE03-E8B9-5346-A124-5449913ECA57"));

    assert_eq!(IReference::<A>::IID, windows::runtime::GUID::from("F9E4006C-6E8C-56DF-811C-61F9990EBFB0"));

    assert_eq!(AsyncActionProgressHandler::<A>::IID, windows::runtime::GUID::from("C261D8D0-71BA-5F38-A239-872342253A18"));

    assert_eq!(AsyncActionWithProgressCompletedHandler::<A>::IID, windows::runtime::GUID::from("9A0D211C-0374-5D23-9E15-EAA3570FAE63"));

    assert_eq!(AsyncOperationCompletedHandler::<A>::IID, windows::runtime::GUID::from("9D534225-231F-55E7-A6D0-6C938E2D9160"));

    assert_eq!(AsyncOperationProgressHandler::<A, B>::IID, windows::runtime::GUID::from("264F1E0C-ABE4-590B-9D37-E1CC118ECC75"));

    assert_eq!(AsyncOperationWithProgressCompletedHandler::<A, B>::IID, windows::runtime::GUID::from("C2D078D8-AC47-55AB-83E8-123B2BE5BC5A"));

    assert_eq!(EventHandler::<A>::IID, windows::runtime::GUID::from("FA0B7D80-7EFA-52DF-9B69-0574CE57ADA4"));

    assert_eq!(TypedEventHandler::<A, B>::IID, windows::runtime::GUID::from("EDB31843-B4CF-56EB-925A-D4D0CE97A08D"));

    //
    // Generated Windows.Foundation.Collections GUIDs
    //

    assert_eq!(IIterable::<A>::IID, windows::runtime::GUID::from("96565EB9-A692-59C8-BCB5-647CDE4E6C4D"));

    assert_eq!(IIterator::<A>::IID, windows::runtime::GUID::from("3C9B1E27-8357-590B-8828-6E917F172390"));

    assert_eq!(IKeyValuePair::<A, B>::IID, windows::runtime::GUID::from("89336CD9-8B66-50A7-9759-EB88CCB2E1FE"));

    assert_eq!(IMapChangedEventArgs::<A>::IID, windows::runtime::GUID::from("E1AA5138-12BD-51A1-8558-698DFD070ABE"));

    assert_eq!(IMapView::<A, B>::IID, windows::runtime::GUID::from("B78F0653-FA89-59CF-BA95-726938AAE666"));

    assert_eq!(IMap::<A, B>::IID, windows::runtime::GUID::from("9962CD50-09D5-5C46-B1E1-3C679C1C8FAE"));

    assert_eq!(IObservableMap::<A, B>::IID, windows::runtime::GUID::from("75F99E2A-137E-537E-A5B1-0B5A6245FC02"));

    assert_eq!(IObservableVector::<A>::IID, windows::runtime::GUID::from("D24C289F-2341-5128-AAA1-292DD0DC1950"));

    assert_eq!(IVectorView::<A>::IID, windows::runtime::GUID::from("5F07498B-8E14-556E-9D2E-2E98D5615DA9"));

    assert_eq!(IVector::<A>::IID, windows::runtime::GUID::from("0E3F106F-A266-50A1-8043-C90FCF3844F6"));

    assert_eq!(MapChangedEventHandler::<A, B>::IID, windows::runtime::GUID::from("19046F0B-CF81-5DEC-BBB2-7CC250DA8B8B"));

    assert_eq!(VectorChangedEventHandler::<A>::IID, windows::runtime::GUID::from("A1E9ACD7-E4DF-5A79-AEFA-DE07934AB0FB"));

    //
    // Generated primitive GUIDs
    //

    assert_eq!(IReference::<bool>::IID, windows::runtime::GUID::from("3C00FD60-2950-5939-A21A-2D12C5A01B8A"));

    assert_eq!(IReference::<i8>::IID, windows::runtime::GUID::from("95500129-FBF6-5AFC-89DF-70642D741990"));

    assert_eq!(IReference::<i16>::IID, windows::runtime::GUID::from("6EC9E41B-6709-5647-9918-A1270110FC4E"));

    assert_eq!(IReference::<i32>::IID, windows::runtime::GUID::from("548CEFBD-BC8A-5FA0-8DF2-957440FC8BF4"));

    assert_eq!(IReference::<i64>::IID, windows::runtime::GUID::from("4DDA9E24-E69F-5C6A-A0A6-93427365AF2A"));

    assert_eq!(IReference::<u8>::IID, windows::runtime::GUID::from("e5198cc8-2873-55f5-b0a1-84ff9e4aad62"));

    assert_eq!(IReference::<u16>::IID, windows::runtime::GUID::from("5AB7D2C3-6B62-5E71-A4B6-2D49C4F238FD"));

    assert_eq!(IReference::<u32>::IID, windows::runtime::GUID::from("513ef3af-e784-5325-a91e-97c2b8111cf3"));

    assert_eq!(IReference::<u64>::IID, windows::runtime::GUID::from("6755e376-53bb-568b-a11d-17239868309e"));

    assert_eq!(IReference::<f32>::IID, windows::runtime::GUID::from("719CC2BA-3E76-5DEF-9F1A-38D85A145EA8"));

    assert_eq!(IReference::<f64>::IID, windows::runtime::GUID::from("2F2D6C29-5473-5F3E-92E7-96572BB990E2"));

    assert_eq!(IReference::<windows::runtime::GUID>::IID, windows::runtime::GUID::from("7D50F649-632C-51F9-849A-EE49428933EA"));

    assert_eq!(IReference::<windows::runtime::HSTRING>::IID, windows::runtime::GUID::from("FD416DFB-2A07-52EB-AAE3-DFCE14116C05"));

    // TODO: structs and enums

    Ok(())
}

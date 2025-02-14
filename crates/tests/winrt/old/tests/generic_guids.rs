use windows::{core::*, Devices::Enumeration::*, Foundation::Collections::*, Foundation::*};
use windows_collections::*;
use windows_future::*;

#[test]
fn signatures() {
    assert_eq!(
        Uri::SIGNATURE.as_slice(),
        b"rc(Windows.Foundation.Uri;{9e365e57-48b2-4160-956f-c7385120bbfc})"
    );

    assert_eq!(
        IAsyncInfo::SIGNATURE.as_slice(),
        b"{00000036-0000-0000-c000-000000000046}"
    );
}

#[test]
fn generic_guids() {
    type A = IIterable<IStringable>;
    type B = IKeyValuePair<HSTRING, IAsyncOperationWithProgress<A, f32>>;

    //
    // Generated Windows.Foundation GUIDs
    //
    assert_eq!(
        IAsyncActionWithProgress::<A>::IID,
        GUID::try_from("DD725452-2DA3-5103-9C7D-22EE9BB14AD3").unwrap()
    );

    assert_eq!(
        IAsyncOperationWithProgress::<A, B>::IID,
        GUID::try_from("94645425-B9E5-5B91-B509-8DA4DF6A8916").unwrap()
    );

    assert_eq!(
        IAsyncOperation::<A>::IID,
        GUID::try_from("2BD35EE6-72D9-5C5D-9827-05EBB81487AB").unwrap()
    );

    assert_eq!(
        IReferenceArray::<A>::IID,
        GUID::try_from("4A33FE03-E8B9-5346-A124-5449913ECA57").unwrap()
    );

    assert_eq!(
        IReference::<A>::IID,
        GUID::try_from("F9E4006C-6E8C-56DF-811C-61F9990EBFB0").unwrap()
    );

    assert_eq!(
        AsyncActionProgressHandler::<A>::IID,
        GUID::try_from("C261D8D0-71BA-5F38-A239-872342253A18").unwrap()
    );

    assert_eq!(
        AsyncActionWithProgressCompletedHandler::<A>::IID,
        GUID::try_from("9A0D211C-0374-5D23-9E15-EAA3570FAE63").unwrap()
    );

    assert_eq!(
        AsyncOperationCompletedHandler::<A>::IID,
        GUID::try_from("9D534225-231F-55E7-A6D0-6C938E2D9160").unwrap()
    );

    assert_eq!(
        AsyncOperationProgressHandler::<A, B>::IID,
        GUID::try_from("264F1E0C-ABE4-590B-9D37-E1CC118ECC75").unwrap()
    );

    assert_eq!(
        AsyncOperationWithProgressCompletedHandler::<A, B>::IID,
        GUID::try_from("C2D078D8-AC47-55AB-83E8-123B2BE5BC5A").unwrap()
    );

    assert_eq!(
        EventHandler::<A>::IID,
        GUID::try_from("FA0B7D80-7EFA-52DF-9B69-0574CE57ADA4").unwrap()
    );

    assert_eq!(
        TypedEventHandler::<A, B>::IID,
        GUID::try_from("EDB31843-B4CF-56EB-925A-D4D0CE97A08D").unwrap()
    );

    //
    // Generated Windows.Foundation.Collections GUIDs
    //

    assert_eq!(
        IIterable::<A>::IID,
        GUID::try_from("96565EB9-A692-59C8-BCB5-647CDE4E6C4D").unwrap()
    );

    assert_eq!(
        IIterator::<A>::IID,
        GUID::try_from("3C9B1E27-8357-590B-8828-6E917F172390").unwrap()
    );

    assert_eq!(
        IKeyValuePair::<A, B>::IID,
        GUID::try_from("89336CD9-8B66-50A7-9759-EB88CCB2E1FE").unwrap()
    );

    assert_eq!(
        IMapChangedEventArgs::<A>::IID,
        GUID::try_from("E1AA5138-12BD-51A1-8558-698DFD070ABE").unwrap()
    );

    assert_eq!(
        IMapView::<A, B>::IID,
        GUID::try_from("B78F0653-FA89-59CF-BA95-726938AAE666").unwrap()
    );

    assert_eq!(
        IMap::<A, B>::IID,
        GUID::try_from("9962CD50-09D5-5C46-B1E1-3C679C1C8FAE").unwrap()
    );

    assert_eq!(
        IObservableMap::<A, B>::IID,
        GUID::try_from("75F99E2A-137E-537E-A5B1-0B5A6245FC02").unwrap()
    );

    assert_eq!(
        IObservableVector::<A>::IID,
        GUID::try_from("D24C289F-2341-5128-AAA1-292DD0DC1950").unwrap()
    );

    assert_eq!(
        IVectorView::<A>::IID,
        GUID::try_from("5F07498B-8E14-556E-9D2E-2E98D5615DA9").unwrap()
    );

    assert_eq!(
        IVector::<A>::IID,
        GUID::try_from("0E3F106F-A266-50A1-8043-C90FCF3844F6").unwrap()
    );

    assert_eq!(
        MapChangedEventHandler::<A, B>::IID,
        GUID::try_from("19046F0B-CF81-5DEC-BBB2-7CC250DA8B8B").unwrap()
    );

    assert_eq!(
        VectorChangedEventHandler::<A>::IID,
        GUID::try_from("A1E9ACD7-E4DF-5A79-AEFA-DE07934AB0FB").unwrap()
    );

    //
    // Generated primitive GUIDs
    //

    assert_eq!(
        IReference::<bool>::IID,
        GUID::try_from("3C00FD60-2950-5939-A21A-2D12C5A01B8A").unwrap()
    );

    assert_eq!(
        IReference::<i8>::IID,
        GUID::try_from("95500129-FBF6-5AFC-89DF-70642D741990").unwrap()
    );

    assert_eq!(
        IReference::<i16>::IID,
        GUID::try_from("6EC9E41B-6709-5647-9918-A1270110FC4E").unwrap()
    );

    assert_eq!(
        IReference::<i32>::IID,
        GUID::try_from("548CEFBD-BC8A-5FA0-8DF2-957440FC8BF4").unwrap()
    );

    assert_eq!(
        IReference::<i64>::IID,
        GUID::try_from("4DDA9E24-E69F-5C6A-A0A6-93427365AF2A").unwrap()
    );

    assert_eq!(
        IReference::<u8>::IID,
        GUID::try_from("e5198cc8-2873-55f5-b0a1-84ff9e4aad62").unwrap()
    );

    assert_eq!(
        IReference::<u16>::IID,
        GUID::try_from("5AB7D2C3-6B62-5E71-A4B6-2D49C4F238FD").unwrap()
    );

    assert_eq!(
        IReference::<u32>::IID,
        GUID::try_from("513ef3af-e784-5325-a91e-97c2b8111cf3").unwrap()
    );

    assert_eq!(
        IReference::<u64>::IID,
        GUID::try_from("6755e376-53bb-568b-a11d-17239868309e").unwrap()
    );

    assert_eq!(
        IReference::<f32>::IID,
        GUID::try_from("719CC2BA-3E76-5DEF-9F1A-38D85A145EA8").unwrap()
    );

    assert_eq!(
        IReference::<f64>::IID,
        GUID::try_from("2F2D6C29-5473-5F3E-92E7-96572BB990E2").unwrap()
    );

    assert_eq!(
        IReference::<GUID>::IID,
        GUID::try_from("7D50F649-632C-51F9-849A-EE49428933EA").unwrap()
    );

    assert_eq!(
        IReference::<HSTRING>::IID,
        GUID::try_from("FD416DFB-2A07-52EB-AAE3-DFCE14116C05").unwrap()
    );

    // TODO: structs and enums
}

// Test for https://github.com/microsoft/windows-rs/issues/2922
#[test]
fn generic_class_guid() {
    // First the non-generic case...

    unsafe {
        assert_eq!(
            std::str::from_utf8_unchecked(IUriRuntimeClass::SIGNATURE.as_slice()),
            "{9e365e57-48b2-4160-956f-c7385120bbfc}"
        );

        assert_eq!(
            std::str::from_utf8_unchecked(Uri::SIGNATURE.as_slice()),
            "rc(Windows.Foundation.Uri;{9e365e57-48b2-4160-956f-c7385120bbfc})"
        );
    }

    assert_eq!(
        Uri::IID,
        GUID::try_from("9E365E57-48B2-4160-956F-C7385120BBFC").unwrap()
    );

    // Then the generic case...

    unsafe {
        assert_eq!(std::str::from_utf8_unchecked(IVectorView::<DeviceInformation>::SIGNATURE.as_slice()), "pinterface({bbe1fa4c-b0e3-4583-baef-1f1b2e483e56};rc(Windows.Devices.Enumeration.DeviceInformation;{aba0fb95-4398-489d-8e44-e6130927011f}))");

        assert_eq!(std::str::from_utf8_unchecked(DeviceInformationCollection::SIGNATURE.as_slice()), "rc(Windows.Devices.Enumeration.DeviceInformationCollection;pinterface({bbe1fa4c-b0e3-4583-baef-1f1b2e483e56};rc(Windows.Devices.Enumeration.DeviceInformation;{aba0fb95-4398-489d-8e44-e6130927011f})))");
    }

    assert_eq!(
        DeviceInformationCollection::IID,
        GUID::try_from("E170688F-3495-5BF6-AAB5-9CAC17E0F10F").unwrap()
    );
}

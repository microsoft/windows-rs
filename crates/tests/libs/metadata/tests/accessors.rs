use windows_metadata::*;

// Accessors with parameter or return attributes are written as explicit methods in RDL, so these
// guard that the shipped winmd still associates them with their properties and events.
#[test]
fn accessors() {
    let index = reader::Index::new(vec![
        reader::File::new(windows_default::WINRT.to_vec()).unwrap(),
    ]);

    let property = |namespace, ty, name| {
        let property = index
            .expect(namespace, ty)
            .properties()
            .find(|property| property.name() == name)
            .unwrap();
        (
            property.getter().map(|method| method.name()),
            property.setter().map(|method| method.name()),
        )
    };

    assert_eq!(
        property("Windows.UI.Popups", "IUICommand", "Id"),
        (Some("get_Id"), Some("put_Id"))
    );
    assert_eq!(
        property("Windows.Data.Xml.Dom", "IXmlNode", "NodeValue"),
        (Some("get_NodeValue"), Some("put_NodeValue"))
    );
    assert_eq!(
        property("Windows.Networking.Vpn", "IVpnRoute", "PrefixSize"),
        (Some("get_PrefixSize"), Some("put_PrefixSize"))
    );

    let event = index
        .expect("Windows.Services.Maps.OfflineMaps", "IOfflineMapPackage")
        .events()
        .find(|event| event.name() == "StatusChanged")
        .unwrap();

    assert_eq!(event.adder().unwrap().name(), "add_StatusChanged");
    assert_eq!(event.remover().unwrap().name(), "remove_StatusChanged");
    assert!(matches!(
        event.ty(&[]),
        Type::ClassName(name)
            if name.namespace == "Windows.Foundation" && name.name.starts_with("TypedEventHandler")
    ));
}

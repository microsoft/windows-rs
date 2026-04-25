use windows_metadata::*;

/// Verifies that TypeDefs are sorted by (namespace, name) regardless of insertion order.
#[test]
fn test() {
    // Write types in reverse alphabetical order.
    let mut file = writer::File::new("test");
    let value_type = writer::TypeDefOrRef::TypeRef(file.TypeRef("System", "ValueType"));

    let flags = TypeAttributes::Public
        | TypeAttributes::SequentialLayout
        | TypeAttributes::Sealed
        | TypeAttributes::WindowsRuntime;

    // Insert in reverse order: Zebra, Mango, Apple
    let zebra = file.TypeDef("Test", "Zebra", value_type, flags);
    file.Field("ZebraField", &Type::I32, FieldAttributes::Public);

    let _mango = file.TypeDef("Test", "Mango", value_type, flags);
    file.Field("MangoField", &Type::I16, FieldAttributes::Public);

    let _apple = file.TypeDef("Test", "Apple", value_type, flags);
    let apple_field = file.Field("AppleField", &Type::I8, FieldAttributes::Public);

    // Add a constant on Apple's field to verify field-index remapping.
    file.Constant(writer::HasConstant::Field(apple_field), &Value::I8(42));

    // Add a nested type under Mango, out of alphabetical order.
    let mango_inner_z = file.TypeDef("", "ZNested", value_type, TypeAttributes::NestedPublic);
    file.NestedClass(mango_inner_z, _mango);
    file.Field("ZNestedField", &Type::U32, FieldAttributes::Public);

    let mango_inner_a = file.TypeDef("", "ANested", value_type, TypeAttributes::NestedPublic);
    file.NestedClass(mango_inner_a, _mango);
    file.Field("ANestedField", &Type::U8, FieldAttributes::Public);

    // Add an attribute on Zebra to verify TypeDef-index remapping.
    let attr_ref = writer::MemberRefParent::TypeRef(
        file.TypeRef("Windows.Foundation.Metadata", "GuidAttribute"),
    );
    let guid_sig = Signature {
        types: vec![
            Type::U32,
            Type::U16,
            Type::U16,
            Type::U8,
            Type::U8,
            Type::U8,
            Type::U8,
            Type::U8,
            Type::U8,
            Type::U8,
            Type::U8,
        ],
        ..Default::default()
    };
    let ctor = file.MemberRef(".ctor", &guid_sig, attr_ref);
    let guid_value = vec![
        (String::new(), Value::U32(0x11223344)),
        (String::new(), Value::U16(0x5566)),
        (String::new(), Value::U16(0x7788)),
        (String::new(), Value::U8(0xaa)),
        (String::new(), Value::U8(0xbb)),
        (String::new(), Value::U8(0xcc)),
        (String::new(), Value::U8(0xdd)),
        (String::new(), Value::U8(0xee)),
        (String::new(), Value::U8(0xff)),
        (String::new(), Value::U8(0x11)),
        (String::new(), Value::U8(0x22)),
    ];
    file.Attribute(
        writer::HasAttribute::TypeDef(zebra),
        writer::AttributeType::MemberRef(ctor),
        &guid_value,
    );

    let bytes = file.into_stream();
    std::fs::write("tests/sorted.winmd", bytes).unwrap();

    let index = reader::TypeIndex::read("tests/sorted.winmd").unwrap();

    // Each type should have exactly the fields it was written with, regardless of insertion order.
    let apple_ty = index.expect("Test", "Apple");
    let apple_fields: Vec<_> = apple_ty.fields().collect();
    assert_eq!(apple_fields.len(), 1);
    assert_eq!(apple_fields[0].name(), "AppleField");
    assert_eq!(apple_fields[0].ty(), Type::I8);

    let mango_ty = index.expect("Test", "Mango");
    let mango_fields: Vec<_> = mango_ty.fields().collect();
    assert_eq!(mango_fields.len(), 1);
    assert_eq!(mango_fields[0].name(), "MangoField");
    assert_eq!(mango_fields[0].ty(), Type::I16);

    let zebra_ty = index.expect("Test", "Zebra");
    let zebra_fields: Vec<_> = zebra_ty.fields().collect();
    assert_eq!(zebra_fields.len(), 1);
    assert_eq!(zebra_fields[0].name(), "ZebraField");
    assert_eq!(zebra_fields[0].ty(), Type::I32);

    // Zebra's attribute should survive the sort.
    let zebra_attrs: Vec<_> = zebra_ty.attributes().collect();
    assert_eq!(zebra_attrs.len(), 1);

    // Apple's field constant should survive the sort.
    let apple_field = apple_ty.fields().next().unwrap();
    assert_eq!(apple_field.constant().unwrap().value(), Value::I8(42));

    // Nested types under Mango should exist with correct fields.
    let nested: Vec<_> = index.nested(mango_ty).collect();
    assert_eq!(nested.len(), 2);
    let nested_names: Vec<&str> = nested.iter().map(|t| t.name()).collect();
    assert!(nested_names.contains(&"ANested"));
    assert!(nested_names.contains(&"ZNested"));
}

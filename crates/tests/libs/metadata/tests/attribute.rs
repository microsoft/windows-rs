use windows_metadata::*;

#[test]
fn test() {
    let mut file = writer::File::new("test");

    let def = file.TypeDef(
        "Namespace",
        "Name",
        writer::TypeDefOrRef::default(),
        TypeAttributes::Public
            | TypeAttributes::Interface
            | TypeAttributes::Abstract
            | TypeAttributes::WindowsRuntime,
    );

    let attribute = writer::MemberRefParent::TypeRef(
        file.TypeRef("Windows.Foundation.Metadata", "GuidAttribute"),
    );

    let signature = Signature {
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

    let ctor = file.MemberRef(".ctor", &signature, attribute);

    let value = vec![
        (String::new(), Value::U32(0xd095a8ca)),
        (String::new(), Value::U16(0x1103)),
        (String::new(), Value::U16(0x4ef5)),
        (String::new(), Value::U8(0x99)),
        (String::new(), Value::U8(0x8c)),
        (String::new(), Value::U8(0x62)),
        (String::new(), Value::U8(0x82)),
        (String::new(), Value::U8(0x15)),
        (String::new(), Value::U8(0x10)),
        (String::new(), Value::U8(0xef)),
        (String::new(), Value::U8(0x8f)),
    ];

    file.Attribute(
        writer::HasAttribute::TypeDef(def),
        writer::AttributeType::MemberRef(ctor),
        &value,
    );

    let bytes = file.into_stream();
    std::fs::write("tests/attribute.winmd", bytes).unwrap();

    let index = reader::Index::read("tests/attribute.winmd").unwrap();
    let ty = index.expect("Namespace", "Name");

    let attributes: Vec<_> = ty.attributes().collect();
    assert_eq!(attributes.len(), 1);
}

#[test]
fn attribute_order_is_independent_of_tokens_and_heap_offsets() {
    let attributes = [
        ("Z", "TagAttribute", vec![Value::I32(1)]),
        ("A", "TagAttribute", vec![Value::I32(2), Value::I32(1)]),
        ("A", "TagAttribute", vec![Value::I32(1), Value::I32(2)]),
        ("A", "TagAttribute", vec![Value::I32(2), Value::I32(1)]),
        (
            "A",
            "TagAttribute",
            vec![Value::EnumValue(
                TypeName::named("Z", "Kind"),
                Box::new(Value::I32(1)),
            )],
        ),
        (
            "A",
            "TagAttribute",
            vec![Value::EnumValue(
                TypeName::named("A", "Kind"),
                Box::new(Value::I32(1)),
            )],
        ),
        (
            "A",
            "Outer/TagAttribute",
            vec![Value::Utf8("z".repeat(200))],
        ),
        (
            "A",
            "Other/TagAttribute",
            vec![Value::Utf8("a".repeat(16384))],
        ),
        (
            "A",
            "StaticAttribute",
            vec![
                Value::TypeName(TypeName::named("Test", "IStatics2")),
                Value::U32(196608),
                Value::Utf8("Test.Contract".into()),
            ],
        ),
        (
            "A",
            "StaticAttribute",
            vec![
                Value::TypeName(TypeName::named("Test", "IStatics")),
                Value::U32(65536),
                Value::Utf8("Test.Contract".into()),
            ],
        ),
    ];

    let write = |order: &[usize]| {
        let mut file = writer::File::new("test");
        let owner = file.TypeDef(
            "Test",
            "Owner",
            writer::TypeDefOrRef::default(),
            TypeAttributes::Public | TypeAttributes::Interface | TypeAttributes::Abstract,
        );
        let method = file.MethodDef(
            "Method",
            &Signature::default(),
            MethodAttributes::Public,
            MethodImplAttributes::default(),
        );
        for &i in order {
            let (namespace, name, values) = &attributes[i];
            let parent = writer::MemberRefParent::TypeRef(file.TypeRef(namespace, name));
            let signature = Signature {
                types: values.iter().map(Value::ty).collect(),
                ..Default::default()
            };
            let ctor = file.MemberRef(".ctor", &signature, parent);
            let mut values: Vec<_> = values
                .iter()
                .map(|value| (String::new(), value.clone()))
                .collect();
            values.push(("Z".into(), Value::I32(2)));
            values.push(("A".into(), Value::I32(1)));
            for owner in [
                writer::HasAttribute::TypeDef(owner),
                writer::HasAttribute::MethodDef(method),
            ] {
                file.Attribute(owner, writer::AttributeType::MemberRef(ctor), &values);
            }
        }
        reader::Index::new(vec![reader::File::new(file.into_stream()).unwrap()])
    };
    let read = |attributes: reader::RowIterator<'_, reader::Attribute<'_>>| {
        attributes
            .map(|attribute| {
                let reader::MemberRefParent::TypeRef(parent) = attribute.ctor().parent() else {
                    panic!()
                };
                let name = parent.full_name();
                (
                    name.namespace,
                    name.name,
                    attribute.ctor().signature(&[]),
                    attribute.value(),
                )
            })
            .collect::<Vec<_>>()
    };

    let mut order: Vec<_> = (0..attributes.len()).collect();
    let index = write(&order);
    let expected = read(index.expect("Test", "Owner").attributes());
    assert_eq!(expected.len(), attributes.len());
    for item @ (namespace, name, values) in &attributes {
        assert_eq!(
            expected
                .iter()
                .filter(|(ns, n, _, args)| {
                    ns == namespace
                        && n == name
                        && args[..values.len()]
                            .iter()
                            .map(|(_, value)| value)
                            .eq(values)
                })
                .count(),
            attributes.iter().filter(|other| *other == item).count()
        );
    }
    assert!(expected.iter().all(|(_, _, _, args)| {
        args[args.len() - 2..] == [("Z".into(), Value::I32(2)), ("A".into(), Value::I32(1))]
    }));

    order.reverse();
    for _ in 0..order.len() {
        let index = write(&order);
        let owner = index.expect("Test", "Owner");
        assert_eq!(read(owner.attributes()), expected);
        assert_eq!(read(owner.methods().next().unwrap().attributes()), expected);
        order.rotate_left(1);
    }
}

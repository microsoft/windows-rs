use windows_metadata::*;

fn input(path: &std::path::Path) {
    let mut file = writer::File::new("semantic");
    file.TypeDef(
        "Test",
        "IRuntimeClass",
        writer::TypeDefOrRef::default(),
        TypeAttributes::Public | TypeAttributes::Interface | TypeAttributes::Abstract,
    );
    let get = file.MethodDef(
        "get_Item",
        &Signature {
            flags: MethodCallAttributes::HASTHIS,
            return_type: Type::I32,
            types: vec![Type::U32],
        },
        MethodAttributes::Public,
        MethodImplAttributes::Runtime,
    );
    let set = file.MethodDef(
        "set_Item",
        &Signature {
            flags: MethodCallAttributes::HASTHIS,
            return_type: Type::Void,
            types: vec![Type::U32, Type::I32],
        },
        MethodAttributes::Public,
        MethodImplAttributes::Runtime,
    );
    let add = file.MethodDef(
        "add_Changed",
        &Signature {
            flags: MethodCallAttributes::HASTHIS,
            return_type: Type::I32,
            types: vec![Type::ClassName(TypeName::named("Test", "Handler"))],
        },
        MethodAttributes::Public,
        MethodImplAttributes::Runtime,
    );
    let remove = file.MethodDef(
        "remove_Changed",
        &Signature {
            flags: MethodCallAttributes::HASTHIS,
            return_type: Type::Void,
            types: vec![Type::I32],
        },
        MethodAttributes::Public,
        MethodImplAttributes::Runtime,
    );

    let extends = writer::TypeDefOrRef::TypeRef(file.TypeRef("System", "Object"));
    let ty = file.TypeDef(
        "Test",
        "RuntimeClass",
        extends,
        TypeAttributes::Public | TypeAttributes::WindowsRuntime,
    );
    file.MethodDef(
        "Ordinary",
        &Signature {
            flags: MethodCallAttributes::HASTHIS,
            return_type: Type::Void,
            types: vec![],
        },
        MethodAttributes::Public,
        MethodImplAttributes::Runtime,
    );

    let property = file.PropertyWithSignature(
        "Item",
        &Signature {
            flags: MethodCallAttributes::HASTHIS,
            return_type: Type::I32,
            types: vec![Type::U32],
        },
        0x0200,
    );
    file.PropertyMap(ty, property);
    file.Constant(writer::HasConstant::Property(property), &Value::I32(42));
    file.MethodSemantics(0x0002, get, writer::HasSemantics::Property(property));
    file.MethodSemantics(0x0001, set, writer::HasSemantics::Property(property));

    let event = file.EventWithFlags(
        "Changed",
        &Type::ClassName(TypeName::named("Test", "Handler")),
        0x0400,
    );
    file.EventMap(ty, event);
    file.MethodSemantics(0x0008, add, writer::HasSemantics::Event(event));
    file.MethodSemantics(0x0010, remove, writer::HasSemantics::Event(event));

    let parent = writer::MemberRefParent::TypeRef(file.TypeRef("Test", "MarkerAttribute"));
    let ctor = file.MemberRef(
        ".ctor",
        &Signature {
            flags: MethodCallAttributes::HASTHIS,
            return_type: Type::Void,
            types: vec![],
        },
        parent,
    );
    file.Attribute(
        writer::HasAttribute::Property(property),
        writer::AttributeType::MemberRef(ctor),
        &[],
    );
    file.Attribute(
        writer::HasAttribute::Event(event),
        writer::AttributeType::MemberRef(ctor),
        &[],
    );

    std::fs::write(path, file.into_stream()).unwrap();
}

fn verify(path: &std::path::Path, namespace: &str) {
    let index = reader::Index::read(path.to_string_lossy().as_ref()).unwrap();
    let ty = index.expect(namespace, "RuntimeClass");
    let method_names: Vec<_> = ty.methods().map(|method| method.name()).collect();
    assert!(method_names.contains(&"Ordinary"));

    let property = ty.properties().next().unwrap();
    assert_eq!(property.name(), "Item");
    assert_eq!(property.flags(), 0x0200);
    assert_eq!(property.signature(&[]).return_type, Type::I32);
    assert_eq!(property.signature(&[]).types, [Type::U32]);
    assert_eq!(property.constant().unwrap().value(), Value::I32(42));
    assert!(property.has_attribute("MarkerAttribute"));
    let mut property_semantics: Vec<_> = property
        .semantics()
        .map(|row| (row.semantics(), row.method().name()))
        .collect();
    property_semantics.sort();
    assert_eq!(
        property_semantics,
        [(0x0001, "set_Item"), (0x0002, "get_Item")]
    );

    let event = ty.events().next().unwrap();
    assert_eq!(event.name(), "Changed");
    assert_eq!(event.flags(), 0x0400);
    assert_eq!(
        event.ty(&[]),
        Type::ClassName(TypeName::named(namespace, "Handler"))
    );
    assert!(event.has_attribute("MarkerAttribute"));
    let mut event_semantics: Vec<_> = event
        .semantics()
        .map(|row| (row.semantics(), row.method().name()))
        .collect();
    event_semantics.sort();
    assert_eq!(
        event_semantics,
        [(0x0008, "add_Changed"), (0x0010, "remove_Changed")]
    );
}

#[test]
fn merge_preserves_runtime_class_semantics() {
    let dir = std::env::temp_dir().join("windows_metadata_merge_semantics");
    std::fs::create_dir_all(&dir).unwrap();
    let input_path = dir.join("input.winmd");
    let output = dir.join("output.winmd");
    input(&input_path);

    merge().input(input_path).output(&output).merge().unwrap();

    verify(&output, "Test");
}

#[test]
fn remap_preserves_runtime_class_semantics() {
    let dir = std::env::temp_dir().join("windows_metadata_remap_semantics");
    std::fs::create_dir_all(&dir).unwrap();
    let input_path = dir.join("input.winmd");
    let output = dir.join("output.winmd");
    input(&input_path);

    remap()
        .input(input_path)
        .source("Test")
        .fallback("Remapped")
        .output(&output)
        .remap()
        .unwrap();

    verify(&output, "Remapped");
}

#[test]
fn property_count_widens_has_constant() {
    let mut file = writer::File::new("property_width");
    let ty = file.TypeDef(
        "Test",
        "Properties",
        writer::TypeDefOrRef::default(),
        TypeAttributes::Public,
    );
    let mut first = None;
    let mut last = None;
    for index in 0..16_384 {
        let property = file.Property(&format!("P{index}"), &Type::I32);
        first.get_or_insert(property);
        last = Some(property);
    }
    file.PropertyMap(ty, first.unwrap());
    file.Constant(
        writer::HasConstant::Property(last.unwrap()),
        &Value::I32(42),
    );

    let index = reader::Index::new(vec![reader::File::new(file.into_stream()).unwrap()]);
    let properties: Vec<_> = index.expect("Test", "Properties").properties().collect();
    assert_eq!(properties.len(), 16_384);
    assert_eq!(
        properties.last().unwrap().constant().unwrap().value(),
        Value::I32(42)
    );
}

#[test]
fn event_count_widens_has_attribute() {
    let mut file = writer::File::new("event_width");
    let ty = file.TypeDef(
        "Test",
        "Events",
        writer::TypeDefOrRef::default(),
        TypeAttributes::Public,
    );
    let mut first = None;
    let mut last = None;
    for index in 0..2_048 {
        let event = file.Event(
            &format!("E{index}"),
            &Type::ClassName(TypeName::named("Test", "Handler")),
        );
        first.get_or_insert(event);
        last = Some(event);
    }
    file.EventMap(ty, first.unwrap());

    let parent = writer::MemberRefParent::TypeRef(file.TypeRef("Test", "MarkerAttribute"));
    let ctor = file.MemberRef(".ctor", &Signature::default(), parent);
    file.Attribute(
        writer::HasAttribute::Event(last.unwrap()),
        writer::AttributeType::MemberRef(ctor),
        &[],
    );

    let index = reader::Index::new(vec![reader::File::new(file.into_stream()).unwrap()]);
    let events: Vec<_> = index.expect("Test", "Events").events().collect();
    assert_eq!(events.len(), 2_048);
    assert!(events.last().unwrap().has_attribute("MarkerAttribute"));
}
